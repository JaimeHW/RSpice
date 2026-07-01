#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        p: &Parameters,
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatgat: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_btat_dn9: f64,
        var_cerfc: f64,
        var_fbbtgat: f64,
        var_fbbtgat_dn6: f64,
        var_fbbtgat_dn7: f64,
        var_fbbtgat_dn8: f64,
        var_fbbtgat_dn9: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard336: f64,
        var_guard340: f64,
        var_guard341: f64,
        var_one_over_one_minus_pgat: f64,
        var_perfc: f64,
        var_sqrtumax: f64,
        var_sqrtumax_dn6: f64,
        var_sqrtumax_dn7: f64,
        var_sqrtumax_dn8: f64,
        var_sqrtumax_dn9: f64,
        var_twoatatoverthreebtat: f64,
        var_twoatatoverthreebtat_dn6: f64,
        var_twoatatoverthreebtat_dn7: f64,
        var_twoatatoverthreebtat_dn8: f64,
        var_twoatatoverthreebtat_dn9: f64,
        var_umax: f64,
        var_umax_dn6: f64,
        var_umax_dn7: f64,
        var_umax_dn8: f64,
        var_umax_dn9: f64,
        var_umaxpoweronepointfive: f64,
        var_umaxpoweronepointfive_dn6: f64,
        var_umaxpoweronepointfive_dn7: f64,
        var_umaxpoweronepointfive_dn8: f64,
        var_umaxpoweronepointfive_dn9: f64,
        var_v2: f64,
        var_vbbt: f64,
        var_vbirgatinv: f64,
        var_wdepnulrinvgat: f64,
        var_wsrh: f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfcpos_dn9_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_erfctimesexpmtat_dn9_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_fmaxr_dn9_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_gammamax_dn9_slot: &mut f64,
        var_guard342_slot: &mut f64,
        var_guard343_slot: &mut f64,
        var_guard344_slot: &mut f64,
        var_guard345_slot: &mut f64,
        var_guard346_slot: &mut f64,
        var_guard347_slot: &mut f64,
        var_guard348_slot: &mut f64,
        var_guard349_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ktat_dn9_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_ltat_dn9_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_mtat_dn9_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_terfc_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wgamma_dn9_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_wtat_dn9_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_xerfc_dn9_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
        var_ysq_dn9_slot: &mut f64,
    ) {
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfcpos_dn9: f64 = *var_erfcpos_dn9_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_erfctimesexpmtat_dn9: f64 = *var_erfctimesexpmtat_dn9_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_fmaxr_dn9: f64 = *var_fmaxr_dn9_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_gammamax_dn9: f64 = *var_gammamax_dn9_slot;
        let mut var_guard342: f64 = *var_guard342_slot;
        let mut var_guard343: f64 = *var_guard343_slot;
        let mut var_guard344: f64 = *var_guard344_slot;
        let mut var_guard345: f64 = *var_guard345_slot;
        let mut var_guard346: f64 = *var_guard346_slot;
        let mut var_guard347: f64 = *var_guard347_slot;
        let mut var_guard348: f64 = *var_guard348_slot;
        let mut var_guard349: f64 = *var_guard349_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ktat_dn9: f64 = *var_ktat_dn9_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_ltat_dn9: f64 = *var_ltat_dn9_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_mtat_dn9: f64 = *var_mtat_dn9_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_terfc_dn9: f64 = *var_terfc_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wgamma_dn9: f64 = *var_wgamma_dn9_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_wtat_dn9: f64 = *var_wtat_dn9_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_xerfc_dn9: f64 = *var_xerfc_dn9_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;
        let mut var_ysq_dn9: f64 = *var_ysq_dn9_slot;

        let (assign18780_e19303, assign18780_e19303_d_n6, assign18780_e19303_d_n7, assign18780_e19303_d_n8, assign18780_e19303_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) && (var_guard341 == 0.0)) {
        let assign18780_e19295: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign18780_e19296: f64 = (1.0 + assign18780_e19295);
        let assign18780_e19298: f64 = (-p.p850);
        let assign18780_e19300: f64 = (assign18780_e19298 * var_one_over_one_minus_pgat);
        let assign18780_e19301: f64 = (assign18780_e19296).powf(assign18780_e19300);
        (assign18780_e19301, if 0.0 == 0.0 && ((assign18780_e19300) as f64).is_finite() && ((assign18780_e19300) as f64).fract() == 0.0 { if assign18780_e19300 == 0.0 { 0.0 } else { (assign18780_e19300 * ((assign18780_e19296).powf(assign18780_e19300 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign18780_e19301 * (assign18780_e19300 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign18780_e19296))) }, if 0.0 == 0.0 && ((assign18780_e19300) as f64).is_finite() && ((assign18780_e19300) as f64).fract() == 0.0 { if assign18780_e19300 == 0.0 { 0.0 } else { (assign18780_e19300 * ((assign18780_e19296).powf(assign18780_e19300 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign18780_e19301 * (assign18780_e19300 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign18780_e19296))) }, if 0.0 == 0.0 && ((assign18780_e19300) as f64).is_finite() && ((assign18780_e19300) as f64).fract() == 0.0 { if assign18780_e19300 == 0.0 { 0.0 } else { (assign18780_e19300 * ((assign18780_e19296).powf(assign18780_e19300 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign18780_e19301 * (assign18780_e19300 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign18780_e19296))) }, if 0.0 == 0.0 && ((assign18780_e19300) as f64).is_finite() && ((assign18780_e19300) as f64).fract() == 0.0 { if assign18780_e19300 == 0.0 { 0.0 } else { (assign18780_e19300 * ((assign18780_e19296).powf(assign18780_e19300 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign18780_e19301 * (assign18780_e19300 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign18780_e19296))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign18780_e19303;
        var_wgamma_dn6 = assign18780_e19303_d_n6;
        var_wgamma_dn7 = assign18780_e19303_d_n7;
        var_wgamma_dn8 = assign18780_e19303_d_n8;
        var_wgamma_dn9 = assign18780_e19303_d_n9;

        let (assign18790_e19321, assign18790_e19321_d_n6, assign18790_e19321_d_n7, assign18790_e19321_d_n8, assign18790_e19321_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) {
        let assign18790_e19315: f64 = (var_wsrh * var_wgamma);
        let assign18790_e19318: f64 = (var_wsrh + var_wgamma);
        let assign18790_e19319: f64 = (assign18790_e19315 / assign18790_e19318);
        (assign18790_e19319, ((((var_wsrh * var_wgamma_dn6) * assign18790_e19318) - (assign18790_e19315 * var_wgamma_dn6)) / (assign18790_e19318 * assign18790_e19318)), ((((var_wsrh * var_wgamma_dn7) * assign18790_e19318) - (assign18790_e19315 * var_wgamma_dn7)) / (assign18790_e19318 * assign18790_e19318)), ((((var_wsrh * var_wgamma_dn8) * assign18790_e19318) - (assign18790_e19315 * var_wgamma_dn8)) / (assign18790_e19318 * assign18790_e19318)), ((((var_wsrh * var_wgamma_dn9) * assign18790_e19318) - (assign18790_e19315 * var_wgamma_dn9)) / (assign18790_e19318 * assign18790_e19318)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign18790_e19321;
        var_wtat_dn6 = assign18790_e19321_d_n6;
        var_wtat_dn7 = assign18790_e19321_d_n7;
        var_wtat_dn8 = assign18790_e19321_d_n8;
        var_wtat_dn9 = assign18790_e19321_d_n9;

        let (assign18800_e19338, assign18800_e19338_d_n6, assign18800_e19338_d_n7, assign18800_e19338_d_n8, assign18800_e19338_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) {
        let assign18800_e19334: f64 = (var_btat / var_sqrtumax);
        let assign18800_e19335: f64 = (0.375 * assign18800_e19334);
        let assign18800_e19336: f64 = (assign18800_e19335).sqrt();
        (assign18800_e19336, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign18800_e19336)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign18800_e19336)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign18800_e19336)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign18800_e19336)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign18800_e19338;
        var_ktat_dn6 = assign18800_e19338_d_n6;
        var_ktat_dn7 = assign18800_e19338_d_n7;
        var_ktat_dn8 = assign18800_e19338_d_n8;
        var_ktat_dn9 = assign18800_e19338_d_n9;

        let (assign18810_e19356, assign18810_e19356_d_n6, assign18810_e19356_d_n7, assign18810_e19356_d_n8, assign18810_e19356_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) {
        let assign18810_e19351: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign18810_e19352: f64 = (2.0 * assign18810_e19351);
        let assign18810_e19354: f64 = (assign18810_e19352 - var_umax);
        (assign18810_e19354, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign18810_e19356;
        var_ltat_dn6 = assign18810_e19356_d_n6;
        var_ltat_dn7 = assign18810_e19356_d_n7;
        var_ltat_dn8 = assign18810_e19356_d_n8;
        var_ltat_dn9 = assign18810_e19356_d_n9;

        let (assign18820_e19382, assign18820_e19382_d_n6, assign18820_e19382_d_n7, assign18820_e19382_d_n8, assign18820_e19382_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) {
        let assign18820_e19368: f64 = (var_atatgat * var_twoatatoverthreebtat);
        let assign18820_e19370: f64 = (assign18820_e19368 * var_sqrtumax);
        let assign18820_e19373: f64 = (var_atatgat * var_umax);
        let assign18820_e19374: f64 = (assign18820_e19370 - assign18820_e19373);
        let assign18820_e19378: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign18820_e19379: f64 = (0.5 * assign18820_e19378);
        let assign18820_e19380: f64 = (assign18820_e19374 + assign18820_e19379);
        (assign18820_e19380, (((((var_atatgat * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign18820_e19368 * var_sqrtumax_dn6)) - (var_atatgat * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign18820_e19368 * var_sqrtumax_dn7)) - (var_atatgat * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign18820_e19368 * var_sqrtumax_dn8)) - (var_atatgat * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatgat * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign18820_e19368 * var_sqrtumax_dn9)) - (var_atatgat * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign18820_e19382;
        var_mtat_dn6 = assign18820_e19382_d_n6;
        var_mtat_dn7 = assign18820_e19382_d_n7;
        var_mtat_dn8 = assign18820_e19382_d_n8;
        var_mtat_dn9 = assign18820_e19382_d_n9;

        let (assign18830_e19398, assign18830_e19398_d_n6, assign18830_e19398_d_n7, assign18830_e19398_d_n8, assign18830_e19398_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) {
        let assign18830_e19394: f64 = (var_ltat - 1.0);
        let assign18830_e19396: f64 = (assign18830_e19394 * var_ktat);
        (assign18830_e19396, ((var_ltat_dn6 * var_ktat) + (assign18830_e19394 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign18830_e19394 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign18830_e19394 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign18830_e19394 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign18830_e19398;
        var_xerfc_dn6 = assign18830_e19398_d_n6;
        var_xerfc_dn7 = assign18830_e19398_d_n7;
        var_xerfc_dn8 = assign18830_e19398_d_n8;
        var_xerfc_dn9 = assign18830_e19398_d_n9;

        let (assign18840_e19412, assign18840_e19412_d_n6, assign18840_e19412_d_n7, assign18840_e19412_d_n8, assign18840_e19412_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) {
        let assign18840_e19410: f64 = (var_xerfc * var_xerfc);
        (assign18840_e19410, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign18840_e19412;
        var_ysq_dn6 = assign18840_e19412_d_n6;
        var_ysq_dn7 = assign18840_e19412_d_n7;
        var_ysq_dn8 = assign18840_e19412_d_n8;
        var_ysq_dn9 = assign18840_e19412_d_n9;

        let assign18850_e19415: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard342 = assign18850_e19415;

        let (assign18860_e19435, assign18860_e19435_d_n6, assign18860_e19435_d_n7, assign18860_e19435_d_n8, assign18860_e19435_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) && (var_guard342 != 0.0)) {
        let assign18860_e19431: f64 = (var_perfc * var_xerfc);
        let assign18860_e19432: f64 = (1.0 + assign18860_e19431);
        let assign18860_e19433: f64 = (1.0 / assign18860_e19432);
        (assign18860_e19433, (-((var_perfc * var_xerfc_dn6) / (assign18860_e19432 * assign18860_e19432))), (-((var_perfc * var_xerfc_dn7) / (assign18860_e19432 * assign18860_e19432))), (-((var_perfc * var_xerfc_dn8) / (assign18860_e19432 * assign18860_e19432))), (-((var_perfc * var_xerfc_dn9) / (assign18860_e19432 * assign18860_e19432))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign18860_e19435;
        var_terfc_dn6 = assign18860_e19435_d_n6;
        var_terfc_dn7 = assign18860_e19435_d_n7;
        var_terfc_dn8 = assign18860_e19435_d_n8;
        var_terfc_dn9 = assign18860_e19435_d_n9;

        let (assign18870_e19456, assign18870_e19456_d_n6, assign18870_e19456_d_n7, assign18870_e19456_d_n8, assign18870_e19456_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) && (var_guard342 == 0.0)) {
        let assign18870_e19452: f64 = (var_perfc * var_xerfc);
        let assign18870_e19453: f64 = (1.0 - assign18870_e19452);
        let assign18870_e19454: f64 = (1.0 / assign18870_e19453);
        (assign18870_e19454, (-((-(var_perfc * var_xerfc_dn6)) / (assign18870_e19453 * assign18870_e19453))), (-((-(var_perfc * var_xerfc_dn7)) / (assign18870_e19453 * assign18870_e19453))), (-((-(var_perfc * var_xerfc_dn8)) / (assign18870_e19453 * assign18870_e19453))), (-((-(var_perfc * var_xerfc_dn9)) / (assign18870_e19453 * assign18870_e19453))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign18870_e19456;
        var_terfc_dn6 = assign18870_e19456_d_n6;
        var_terfc_dn7 = assign18870_e19456_d_n7;
        var_terfc_dn8 = assign18870_e19456_d_n8;
        var_terfc_dn9 = assign18870_e19456_d_n9;

        let assign18880_e19458: f64 = (-var_ysq);
        let assign18880_e19460: f64 = (assign18880_e19458 + var_mtat);
        let assign18880_e19462: f64 = (-230.25850929940458);
        let assign18880_e19463: f64 = if assign18880_e19460 > assign18880_e19462 { 1.0 } else { 0.0 };
        var_guard343 = assign18880_e19463;

        let (assign18890_e19481, assign18890_e19481_d_n6, assign18890_e19481_d_n7, assign18890_e19481_d_n8, assign18890_e19481_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) && (var_guard343 != 0.0)) {
        let assign18890_e19476: f64 = (-var_ysq);
        let assign18890_e19478: f64 = (assign18890_e19476 + var_mtat);
        let assign18890_e19479: f64 = (assign18890_e19478).exp();
        (assign18890_e19479, (assign18890_e19479 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign18890_e19479 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign18890_e19479 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign18890_e19479 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign18890_e19481;
        var_tmp_dn6 = assign18890_e19481_d_n6;
        var_tmp_dn7 = assign18890_e19481_d_n7;
        var_tmp_dn8 = assign18890_e19481_d_n8;
        var_tmp_dn9 = assign18890_e19481_d_n9;

        let (assign18900_e19530, assign18900_e19530_d_n6, assign18900_e19530_d_n7, assign18900_e19530_d_n8, assign18900_e19530_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) && (var_guard343 == 0.0)) {
        let assign18900_e19497: f64 = (-230.25850929940458);
        let assign18900_e19499: f64 = (-var_ysq);
        let assign18900_e19501: f64 = (assign18900_e19499 + var_mtat);
        let assign18900_e19502: f64 = (assign18900_e19497 - assign18900_e19501);
        let assign18900_e19506: f64 = (-230.25850929940458);
        let assign18900_e19508: f64 = (-var_ysq);
        let assign18900_e19510: f64 = (assign18900_e19508 + var_mtat);
        let assign18900_e19511: f64 = (assign18900_e19506 - assign18900_e19510);
        let assign18900_e19514: f64 = (-230.25850929940458);
        let assign18900_e19516: f64 = (-var_ysq);
        let assign18900_e19518: f64 = (assign18900_e19516 + var_mtat);
        let assign18900_e19519: f64 = (assign18900_e19514 - assign18900_e19518);
        let assign18900_e19521: f64 = (assign18900_e19519 * 0.3333333333333333);
        let assign18900_e19522: f64 = (1.0 + assign18900_e19521);
        let assign18900_e19523: f64 = (assign18900_e19511 * assign18900_e19522);
        let assign18900_e19524: f64 = (0.5 * assign18900_e19523);
        let assign18900_e19525: f64 = (1.0 + assign18900_e19524);
        let assign18900_e19526: f64 = (assign18900_e19502 * assign18900_e19525);
        let assign18900_e19527: f64 = (1.0 + assign18900_e19526);
        let assign18900_e19528: f64 = (1e-100 / assign18900_e19527);
        (assign18900_e19528, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign18900_e19525) + (assign18900_e19502 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign18900_e19522) + (assign18900_e19511 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign18900_e19527 * assign18900_e19527))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign18900_e19525) + (assign18900_e19502 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign18900_e19522) + (assign18900_e19511 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign18900_e19527 * assign18900_e19527))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign18900_e19525) + (assign18900_e19502 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign18900_e19522) + (assign18900_e19511 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign18900_e19527 * assign18900_e19527))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign18900_e19525) + (assign18900_e19502 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign18900_e19522) + (assign18900_e19511 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign18900_e19527 * assign18900_e19527))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign18900_e19530;
        var_tmp_dn6 = assign18900_e19530_d_n6;
        var_tmp_dn7 = assign18900_e19530_d_n7;
        var_tmp_dn8 = assign18900_e19530_d_n8;
        var_tmp_dn9 = assign18900_e19530_d_n9;

        let (assign18910_e19560, assign18910_e19560_d_n6, assign18910_e19560_d_n7, assign18910_e19560_d_n8, assign18910_e19560_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) {
        let assign18910_e19542: f64 = (0.29214664 * var_terfc);
        let assign18910_e19546: f64 = (var_terfc * var_terfc);
        let assign18910_e19547: f64 = (var_berfc * assign18910_e19546);
        let assign18910_e19548: f64 = (assign18910_e19542 + assign18910_e19547);
        let assign18910_e19552: f64 = (var_terfc * var_terfc);
        let assign18910_e19554: f64 = (assign18910_e19552 * var_terfc);
        let assign18910_e19555: f64 = (var_cerfc * assign18910_e19554);
        let assign18910_e19556: f64 = (assign18910_e19548 + assign18910_e19555);
        let assign18910_e19558: f64 = (assign18910_e19556 * var_tmp);
        (assign18910_e19558, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign18910_e19552 * var_terfc_dn6)))) * var_tmp) + (assign18910_e19556 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign18910_e19552 * var_terfc_dn7)))) * var_tmp) + (assign18910_e19556 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign18910_e19552 * var_terfc_dn8)))) * var_tmp) + (assign18910_e19556 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign18910_e19552 * var_terfc_dn9)))) * var_tmp) + (assign18910_e19556 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign18910_e19560;
        var_erfcpos_dn6 = assign18910_e19560_d_n6;
        var_erfcpos_dn7 = assign18910_e19560_d_n7;
        var_erfcpos_dn8 = assign18910_e19560_d_n8;
        var_erfcpos_dn9 = assign18910_e19560_d_n9;

        let assign18920_e19563: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard344 = assign18920_e19563;

        let (assign18930_e19577, assign18930_e19577_d_n6, assign18930_e19577_d_n7, assign18930_e19577_d_n8, assign18930_e19577_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) && (var_guard344 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign18930_e19577;
        var_erfctimesexpmtat_dn6 = assign18930_e19577_d_n6;
        var_erfctimesexpmtat_dn7 = assign18930_e19577_d_n7;
        var_erfctimesexpmtat_dn8 = assign18930_e19577_d_n8;
        var_erfctimesexpmtat_dn9 = assign18930_e19577_d_n9;

        let assign18940_e19580: f64 = (-230.25850929940458);
        let assign18940_e19581: f64 = if var_mtat > assign18940_e19580 { 1.0 } else { 0.0 };
        var_guard345 = assign18940_e19581;

        let (assign18950_e19599, assign18950_e19599_d_n6, assign18950_e19599_d_n7, assign18950_e19599_d_n8, assign18950_e19599_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) && (var_guard344 == 0.0)) && (var_guard345 != 0.0)) {
        let assign18950_e19597: f64 = (var_mtat).exp();
        (assign18950_e19597, (assign18950_e19597 * var_mtat_dn6), (assign18950_e19597 * var_mtat_dn7), (assign18950_e19597 * var_mtat_dn8), (assign18950_e19597 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign18950_e19599;
        var_tmp_dn6 = assign18950_e19599_d_n6;
        var_tmp_dn7 = assign18950_e19599_d_n7;
        var_tmp_dn8 = assign18950_e19599_d_n8;
        var_tmp_dn9 = assign18950_e19599_d_n9;

        let (assign18960_e19642, assign18960_e19642_d_n6, assign18960_e19642_d_n7, assign18960_e19642_d_n8, assign18960_e19642_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) && (var_guard344 == 0.0)) && (var_guard345 == 0.0)) {
        let assign18960_e19618: f64 = (-230.25850929940458);
        let assign18960_e19620: f64 = (assign18960_e19618 - var_mtat);
        let assign18960_e19624: f64 = (-230.25850929940458);
        let assign18960_e19626: f64 = (assign18960_e19624 - var_mtat);
        let assign18960_e19629: f64 = (-230.25850929940458);
        let assign18960_e19631: f64 = (assign18960_e19629 - var_mtat);
        let assign18960_e19633: f64 = (assign18960_e19631 * 0.3333333333333333);
        let assign18960_e19634: f64 = (1.0 + assign18960_e19633);
        let assign18960_e19635: f64 = (assign18960_e19626 * assign18960_e19634);
        let assign18960_e19636: f64 = (0.5 * assign18960_e19635);
        let assign18960_e19637: f64 = (1.0 + assign18960_e19636);
        let assign18960_e19638: f64 = (assign18960_e19620 * assign18960_e19637);
        let assign18960_e19639: f64 = (1.0 + assign18960_e19638);
        let assign18960_e19640: f64 = (1e-100 / assign18960_e19639);
        (assign18960_e19640, (-((1e-100 * (((-var_mtat_dn6) * assign18960_e19637) + (assign18960_e19620 * (0.5 * (((-var_mtat_dn6) * assign18960_e19634) + (assign18960_e19626 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign18960_e19639 * assign18960_e19639))), (-((1e-100 * (((-var_mtat_dn7) * assign18960_e19637) + (assign18960_e19620 * (0.5 * (((-var_mtat_dn7) * assign18960_e19634) + (assign18960_e19626 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign18960_e19639 * assign18960_e19639))), (-((1e-100 * (((-var_mtat_dn8) * assign18960_e19637) + (assign18960_e19620 * (0.5 * (((-var_mtat_dn8) * assign18960_e19634) + (assign18960_e19626 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign18960_e19639 * assign18960_e19639))), (-((1e-100 * (((-var_mtat_dn9) * assign18960_e19637) + (assign18960_e19620 * (0.5 * (((-var_mtat_dn9) * assign18960_e19634) + (assign18960_e19626 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign18960_e19639 * assign18960_e19639))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign18960_e19642;
        var_tmp_dn6 = assign18960_e19642_d_n6;
        var_tmp_dn7 = assign18960_e19642_d_n7;
        var_tmp_dn8 = assign18960_e19642_d_n8;
        var_tmp_dn9 = assign18960_e19642_d_n9;

        let (assign18970_e19661, assign18970_e19661_d_n6, assign18970_e19661_d_n7, assign18970_e19661_d_n8, assign18970_e19661_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) && (var_guard344 == 0.0)) {
        let assign18970_e19657: f64 = (2.0 * var_tmp);
        let assign18970_e19659: f64 = (assign18970_e19657 - var_erfcpos);
        (assign18970_e19659, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign18970_e19661;
        var_erfctimesexpmtat_dn6 = assign18970_e19661_d_n6;
        var_erfctimesexpmtat_dn7 = assign18970_e19661_d_n7;
        var_erfctimesexpmtat_dn8 = assign18970_e19661_d_n8;
        var_erfctimesexpmtat_dn9 = assign18970_e19661_d_n9;

        let (assign18980_e19681, assign18980_e19681_d_n6, assign18980_e19681_d_n7, assign18980_e19681_d_n8, assign18980_e19681_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) {
        let assign18980_e19673: f64 = (1.772453850905516 * 0.5);
        let assign18980_e19676: f64 = (var_atatgat * var_erfctimesexpmtat);
        let assign18980_e19678: f64 = (assign18980_e19676 / var_ktat);
        let assign18980_e19679: f64 = (assign18980_e19673 * assign18980_e19678);
        (assign18980_e19679, (assign18980_e19673 * ((((var_atatgat * var_erfctimesexpmtat_dn6) * var_ktat) - (assign18980_e19676 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign18980_e19673 * ((((var_atatgat * var_erfctimesexpmtat_dn7) * var_ktat) - (assign18980_e19676 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign18980_e19673 * ((((var_atatgat * var_erfctimesexpmtat_dn8) * var_ktat) - (assign18980_e19676 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign18980_e19673 * ((((var_atatgat * var_erfctimesexpmtat_dn9) * var_ktat) - (assign18980_e19676 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign18980_e19681;
        var_gammamax_dn6 = assign18980_e19681_d_n6;
        var_gammamax_dn7 = assign18980_e19681_d_n7;
        var_gammamax_dn8 = assign18980_e19681_d_n8;
        var_gammamax_dn9 = assign18980_e19681_d_n9;

        let (assign18990_e19699, assign18990_e19699_d_n6, assign18990_e19699_d_n7, assign18990_e19699_d_n8, assign18990_e19699_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) {
        let assign18990_e19694: f64 = (var_asrh * var_gammamax);
        let assign18990_e19696: f64 = (assign18990_e19694 * var_wtat);
        let assign18990_e19697: f64 = (p.p864 * assign18990_e19696);
        (assign18990_e19697, (p.p864 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign18990_e19694 * var_wtat_dn6))), (p.p864 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign18990_e19694 * var_wtat_dn7))), (p.p864 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign18990_e19694 * var_wtat_dn8))), (p.p864 * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign18990_e19694 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign18990_e19699;
        var_itat_dn6 = assign18990_e19699_d_n6;
        var_itat_dn7 = assign18990_e19699_d_n7;
        var_itat_dn8 = assign18990_e19699_d_n8;
        var_itat_dn9 = assign18990_e19699_d_n9;

        let assign19000_e19702: f64 = if p.p870 == 0.0 { 1.0 } else { 0.0 };
        var_guard346 = assign19000_e19702;

        let (assign19010_e19713, assign19010_e19713_d_n6, assign19010_e19713_d_n7, assign19010_e19713_d_n8, assign19010_e19713_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard346 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign19010_e19713;
        var_ibbt_dn6 = assign19010_e19713_d_n6;
        var_ibbt_dn7 = assign19010_e19713_d_n7;
        var_ibbt_dn8 = assign19010_e19713_d_n8;
        var_ibbt_dn9 = assign19010_e19713_d_n9;

        let assign19020_e19716: f64 = if p.p850 == 0.5 { 1.0 } else { 0.0 };
        var_guard347 = assign19020_e19716;

        let (assign19030_e19735, assign19030_e19735_d_n6, assign19030_e19735_d_n7, assign19030_e19735_d_n8, assign19030_e19735_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard346 == 0.0)) && (var_guard347 != 0.0)) {
        let assign19030_e19730: f64 = (p.p847 - var_vbbt);
        let assign19030_e19732: f64 = (assign19030_e19730 * var_vbirgatinv);
        let assign19030_e19733: f64 = (assign19030_e19732).sqrt();
        (assign19030_e19733, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign19030_e19735;
        var_tmp_dn6 = assign19030_e19735_d_n6;
        var_tmp_dn7 = assign19030_e19735_d_n7;
        var_tmp_dn8 = assign19030_e19735_d_n8;
        var_tmp_dn9 = assign19030_e19735_d_n9;

        let (assign19040_e19756, assign19040_e19756_d_n6, assign19040_e19756_d_n7, assign19040_e19756_d_n8, assign19040_e19756_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard346 == 0.0)) && (var_guard347 == 0.0)) {
        let assign19040_e19750: f64 = (p.p847 - var_vbbt);
        let assign19040_e19752: f64 = (assign19040_e19750 * var_vbirgatinv);
        let assign19040_e19754: f64 = (assign19040_e19752).powf(p.p850);
        (assign19040_e19754, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign19040_e19756;
        var_tmp_dn6 = assign19040_e19756_d_n6;
        var_tmp_dn7 = assign19040_e19756_d_n7;
        var_tmp_dn8 = assign19040_e19756_d_n8;
        var_tmp_dn9 = assign19040_e19756_d_n9;

        let (assign19050_e19776, assign19050_e19776_d_n6, assign19050_e19776_d_n7, assign19050_e19776_d_n8, assign19050_e19776_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard346 == 0.0)) {
        let assign19050_e19769: f64 = (p.p847 - var_vbbt);
        let assign19050_e19771: f64 = (assign19050_e19769 * var_wdepnulrinvgat);
        let assign19050_e19773: f64 = (assign19050_e19771 / var_tmp);
        let assign19050_e19774: f64 = (var_one_over_one_minus_pgat * assign19050_e19773);
        (assign19050_e19774, (var_one_over_one_minus_pgat * (-((assign19050_e19771 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign19050_e19771 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign19050_e19771 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign19050_e19771 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign19050_e19776;
        var_fmaxr_dn6 = assign19050_e19776_d_n6;
        var_fmaxr_dn7 = assign19050_e19776_d_n7;
        var_fmaxr_dn8 = assign19050_e19776_d_n8;
        var_fmaxr_dn9 = assign19050_e19776_d_n9;

        let assign19060_e19778: f64 = (-var_fbbtgat);
        let assign19060_e19780: f64 = (assign19060_e19778 / var_fmaxr);
        let assign19060_e19781: f64 = (assign19060_e19780).abs();
        let assign19060_e19783: f64 = if assign19060_e19781 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard348 = assign19060_e19783;

        let (assign19070_e19801, assign19070_e19801_d_n6, assign19070_e19801_d_n7, assign19070_e19801_d_n8, assign19070_e19801_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard346 == 0.0)) && (var_guard348 != 0.0)) {
        let assign19070_e19796: f64 = (-var_fbbtgat);
        let assign19070_e19798: f64 = (assign19070_e19796 / var_fmaxr);
        let assign19070_e19799: f64 = (assign19070_e19798).exp();
        (assign19070_e19799, (assign19070_e19799 * ((((-var_fbbtgat_dn6) * var_fmaxr) - (assign19070_e19796 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign19070_e19799 * ((((-var_fbbtgat_dn7) * var_fmaxr) - (assign19070_e19796 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign19070_e19799 * ((((-var_fbbtgat_dn8) * var_fmaxr) - (assign19070_e19796 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))), (assign19070_e19799 * ((((-var_fbbtgat_dn9) * var_fmaxr) - (assign19070_e19796 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign19070_e19801;
        var_tmp_dn6 = assign19070_e19801_d_n6;
        var_tmp_dn7 = assign19070_e19801_d_n7;
        var_tmp_dn8 = assign19070_e19801_d_n8;
        var_tmp_dn9 = assign19070_e19801_d_n9;

        let assign19080_e19803: f64 = (-var_fbbtgat);
        let assign19080_e19805: f64 = (assign19080_e19803 / var_fmaxr);
        let assign19080_e19807: f64 = if assign19080_e19805 < 0.0 { 1.0 } else { 0.0 };
        var_guard349 = assign19080_e19807;

        let (assign19090_e19858, assign19090_e19858_d_n6, assign19090_e19858_d_n7, assign19090_e19858_d_n8, assign19090_e19858_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard346 == 0.0)) && (var_guard348 == 0.0)) && (var_guard349 != 0.0)) {
        let assign19090_e19825: f64 = (-230.25850929940458);
        let assign19090_e19827: f64 = (-var_fbbtgat);
        let assign19090_e19829: f64 = (assign19090_e19827 / var_fmaxr);
        let assign19090_e19830: f64 = (assign19090_e19825 - assign19090_e19829);
        let assign19090_e19834: f64 = (-230.25850929940458);
        let assign19090_e19836: f64 = (-var_fbbtgat);
        let assign19090_e19838: f64 = (assign19090_e19836 / var_fmaxr);
        let assign19090_e19839: f64 = (assign19090_e19834 - assign19090_e19838);
        let assign19090_e19842: f64 = (-230.25850929940458);
        let assign19090_e19844: f64 = (-var_fbbtgat);
        let assign19090_e19846: f64 = (assign19090_e19844 / var_fmaxr);
        let assign19090_e19847: f64 = (assign19090_e19842 - assign19090_e19846);
        let assign19090_e19849: f64 = (assign19090_e19847 * 0.3333333333333333);
        let assign19090_e19850: f64 = (1.0 + assign19090_e19849);
        let assign19090_e19851: f64 = (assign19090_e19839 * assign19090_e19850);
        let assign19090_e19852: f64 = (0.5 * assign19090_e19851);
        let assign19090_e19853: f64 = (1.0 + assign19090_e19852);
        let assign19090_e19854: f64 = (assign19090_e19830 * assign19090_e19853);
        let assign19090_e19855: f64 = (1.0 + assign19090_e19854);
        let assign19090_e19856: f64 = (1e-100 / assign19090_e19855);
        (assign19090_e19856, (-((1e-100 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign19090_e19827 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign19090_e19853) + (assign19090_e19830 * (0.5 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign19090_e19836 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign19090_e19850) + (assign19090_e19839 * ((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign19090_e19844 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign19090_e19855 * assign19090_e19855))), (-((1e-100 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign19090_e19827 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign19090_e19853) + (assign19090_e19830 * (0.5 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign19090_e19836 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign19090_e19850) + (assign19090_e19839 * ((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign19090_e19844 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign19090_e19855 * assign19090_e19855))), (-((1e-100 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign19090_e19827 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign19090_e19853) + (assign19090_e19830 * (0.5 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign19090_e19836 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign19090_e19850) + (assign19090_e19839 * ((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign19090_e19844 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign19090_e19855 * assign19090_e19855))), (-((1e-100 * (((-((((-var_fbbtgat_dn9) * var_fmaxr) - (assign19090_e19827 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign19090_e19853) + (assign19090_e19830 * (0.5 * (((-((((-var_fbbtgat_dn9) * var_fmaxr) - (assign19090_e19836 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign19090_e19850) + (assign19090_e19839 * ((-((((-var_fbbtgat_dn9) * var_fmaxr) - (assign19090_e19844 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign19090_e19855 * assign19090_e19855))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign19090_e19858;
        var_tmp_dn6 = assign19090_e19858_d_n6;
        var_tmp_dn7 = assign19090_e19858_d_n7;
        var_tmp_dn8 = assign19090_e19858_d_n8;
        var_tmp_dn9 = assign19090_e19858_d_n9;

        let (assign19100_e19907, assign19100_e19907_d_n6, assign19100_e19907_d_n7, assign19100_e19907_d_n8, assign19100_e19907_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard346 == 0.0)) && (var_guard348 == 0.0)) && (var_guard349 == 0.0)) {
        let assign19100_e19877: f64 = (-var_fbbtgat);
        let assign19100_e19879: f64 = (assign19100_e19877 / var_fmaxr);
        let assign19100_e19881: f64 = (assign19100_e19879 - 230.25850929940458);
        let assign19100_e19885: f64 = (-var_fbbtgat);
        let assign19100_e19887: f64 = (assign19100_e19885 / var_fmaxr);
        let assign19100_e19889: f64 = (assign19100_e19887 - 230.25850929940458);
        let assign19100_e19892: f64 = (-var_fbbtgat);
        let assign19100_e19894: f64 = (assign19100_e19892 / var_fmaxr);
        let assign19100_e19896: f64 = (assign19100_e19894 - 230.25850929940458);
        let assign19100_e19898: f64 = (assign19100_e19896 * 0.3333333333333333);
        let assign19100_e19899: f64 = (1.0 + assign19100_e19898);
        let assign19100_e19900: f64 = (assign19100_e19889 * assign19100_e19899);
        let assign19100_e19901: f64 = (0.5 * assign19100_e19900);
        let assign19100_e19902: f64 = (1.0 + assign19100_e19901);
        let assign19100_e19903: f64 = (assign19100_e19881 * assign19100_e19902);
        let assign19100_e19904: f64 = (1.0 + assign19100_e19903);
        let assign19100_e19905: f64 = (1e100 * assign19100_e19904);
        (assign19100_e19905, (1e100 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign19100_e19877 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign19100_e19902) + (assign19100_e19881 * (0.5 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign19100_e19885 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign19100_e19899) + (assign19100_e19889 * (((((-var_fbbtgat_dn6) * var_fmaxr) - (assign19100_e19892 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign19100_e19877 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign19100_e19902) + (assign19100_e19881 * (0.5 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign19100_e19885 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign19100_e19899) + (assign19100_e19889 * (((((-var_fbbtgat_dn7) * var_fmaxr) - (assign19100_e19892 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign19100_e19877 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign19100_e19902) + (assign19100_e19881 * (0.5 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign19100_e19885 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign19100_e19899) + (assign19100_e19889 * (((((-var_fbbtgat_dn8) * var_fmaxr) - (assign19100_e19892 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn9) * var_fmaxr) - (assign19100_e19877 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign19100_e19902) + (assign19100_e19881 * (0.5 * ((((((-var_fbbtgat_dn9) * var_fmaxr) - (assign19100_e19885 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign19100_e19899) + (assign19100_e19889 * (((((-var_fbbtgat_dn9) * var_fmaxr) - (assign19100_e19892 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign19100_e19907;
        var_tmp_dn6 = assign19100_e19907_d_n6;
        var_tmp_dn7 = assign19100_e19907_d_n7;
        var_tmp_dn8 = assign19100_e19907_d_n8;
        var_tmp_dn9 = assign19100_e19907_d_n9;

        let (assign19110_e19927, assign19110_e19927_d_n6, assign19110_e19927_d_n7, assign19110_e19927_d_n8, assign19110_e19927_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard346 == 0.0)) {
        let assign19110_e19920: f64 = (var_v2 * var_fmaxr);
        let assign19110_e19922: f64 = (assign19110_e19920 * var_fmaxr);
        let assign19110_e19924: f64 = (assign19110_e19922 * var_tmp);
        let assign19110_e19925: f64 = (p.p870 * assign19110_e19924);
        (assign19110_e19925, (p.p870 * (((((var_v2 * var_fmaxr_dn6) * var_fmaxr) + (assign19110_e19920 * var_fmaxr_dn6)) * var_tmp) + (assign19110_e19922 * var_tmp_dn6))), (p.p870 * (((((var_v2 * var_fmaxr_dn7) * var_fmaxr) + (assign19110_e19920 * var_fmaxr_dn7)) * var_tmp) + (assign19110_e19922 * var_tmp_dn7))), (p.p870 * (((((var_v2 * var_fmaxr_dn8) * var_fmaxr) + (assign19110_e19920 * var_fmaxr_dn8)) * var_tmp) + (assign19110_e19922 * var_tmp_dn8))), (p.p870 * (((((var_v2 * var_fmaxr_dn9) * var_fmaxr) + (assign19110_e19920 * var_fmaxr_dn9)) * var_tmp) + (assign19110_e19922 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign19110_e19927;
        var_ibbt_dn6 = assign19110_e19927_d_n6;
        var_ibbt_dn7 = assign19110_e19927_d_n7;
        var_ibbt_dn8 = assign19110_e19927_d_n8;
        var_ibbt_dn9 = assign19110_e19927_d_n9;

        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfcpos_dn9_slot = var_erfcpos_dn9;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_erfctimesexpmtat_dn9_slot = var_erfctimesexpmtat_dn9;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_fmaxr_dn9_slot = var_fmaxr_dn9;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_gammamax_dn9_slot = var_gammamax_dn9;
        *var_guard342_slot = var_guard342;
        *var_guard343_slot = var_guard343;
        *var_guard344_slot = var_guard344;
        *var_guard345_slot = var_guard345;
        *var_guard346_slot = var_guard346;
        *var_guard347_slot = var_guard347;
        *var_guard348_slot = var_guard348;
        *var_guard349_slot = var_guard349;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ktat_dn9_slot = var_ktat_dn9;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_ltat_dn9_slot = var_ltat_dn9;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_mtat_dn9_slot = var_mtat_dn9;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_terfc_dn9_slot = var_terfc_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wgamma_dn9_slot = var_wgamma_dn9;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_wtat_dn9_slot = var_wtat_dn9;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_xerfc_dn9_slot = var_xerfc_dn9;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
        *var_ysq_dn9_slot = var_ysq_dn9;
    }

    pub(super) fn stamp_transient_block_33(
        p: &Parameters,
        var_absource_i: f64,
        var_alphaav: f64,
        var_exp_vmax_over_phitd_s: f64,
        var_fstopgat: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard336: f64,
        var_ibbt: f64,
        var_ibbt_dn6: f64,
        var_ibbt_dn7: f64,
        var_ibbt_dn8: f64,
        var_ibbt_dn9: f64,
        var_idsatbot: f64,
        var_ijunsti: f64,
        var_ijunsti_dn6: f64,
        var_ijunsti_dn7: f64,
        var_ijunsti_dn8: f64,
        var_ijunsti_dn9: f64,
        var_itat: f64,
        var_itat_dn6: f64,
        var_itat_dn7: f64,
        var_itat_dn8: f64,
        var_itat_dn9: f64,
        var_lgsource_i: f64,
        var_lssource_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_slopegat: f64,
        var_slopegat_dn6: f64,
        var_slopegat_dn7: f64,
        var_slopegat_dn8: f64,
        var_slopegat_dn9: f64,
        var_v3: f64,
        var_vbbtlim_s: f64,
        var_vbibot: f64,
        var_vbimin_s: f64,
        var_vbrinvgat: f64,
        var_vbrinvgat_dn6: f64,
        var_vbrinvgat_dn7: f64,
        var_vbrinvgat_dn8: f64,
        var_vbrinvgat_dn9: f64,
        var_vmax_s: f64,
        var_dwsrh_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_guard350_slot: &mut f64,
        var_guard351_slot: &mut f64,
        var_guard352_slot: &mut f64,
        var_guard353_slot: &mut f64,
        var_guard354_slot: &mut f64,
        var_guard355_slot: &mut f64,
        var_guard356_slot: &mut f64,
        var_guard357_slot: &mut f64,
        var_guard358_slot: &mut f64,
        var_guard359_slot: &mut f64,
        var_guard360_slot: &mut f64,
        var_i2_slot: &mut f64,
        var_i2_dn6_slot: &mut f64,
        var_i2_dn7_slot: &mut f64,
        var_i2_dn8_slot: &mut f64,
        var_i2_dn9_slot: &mut f64,
        var_id__blk212_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijunbot_dn9_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_ijungat_dn9_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fbreakdown_dn9: f64 = *var_fbreakdown_dn9_slot;
        let mut var_guard350: f64 = *var_guard350_slot;
        let mut var_guard351: f64 = *var_guard351_slot;
        let mut var_guard352: f64 = *var_guard352_slot;
        let mut var_guard353: f64 = *var_guard353_slot;
        let mut var_guard354: f64 = *var_guard354_slot;
        let mut var_guard355: f64 = *var_guard355_slot;
        let mut var_guard356: f64 = *var_guard356_slot;
        let mut var_guard357: f64 = *var_guard357_slot;
        let mut var_guard358: f64 = *var_guard358_slot;
        let mut var_guard359: f64 = *var_guard359_slot;
        let mut var_guard360: f64 = *var_guard360_slot;
        let mut var_i2: f64 = *var_i2_slot;
        let mut var_i2_dn6: f64 = *var_i2_dn6_slot;
        let mut var_i2_dn7: f64 = *var_i2_dn7_slot;
        let mut var_i2_dn8: f64 = *var_i2_dn8_slot;
        let mut var_i2_dn9: f64 = *var_i2_dn9_slot;
        let mut var_id__blk212: f64 = *var_id__blk212_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijunbot_dn9: f64 = *var_ijunbot_dn9_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_ijungat_dn9: f64 = *var_ijungat_dn9_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let assign19120_e19930: f64 = if p.p879 > 1000.0 { 1.0 } else { 0.0 };
        var_guard350 = assign19120_e19930;

        let (assign19130_e19941, assign19130_e19941_d_n6, assign19130_e19941_d_n7, assign19130_e19941_d_n8, assign19130_e19941_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard350 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign19130_e19941;
        var_fbreakdown_dn6 = assign19130_e19941_d_n6;
        var_fbreakdown_dn7 = assign19130_e19941_d_n7;
        var_fbreakdown_dn8 = assign19130_e19941_d_n8;
        var_fbreakdown_dn9 = assign19130_e19941_d_n9;

        let assign19140_e19944: f64 = (-var_alphaav);
        let assign19140_e19946: f64 = (assign19140_e19944 * p.p879);
        let assign19140_e19947: f64 = if var_vav > assign19140_e19946 { 1.0 } else { 0.0 };
        var_guard351 = assign19140_e19947;

        let assign19150_e19950: f64 = if p.p882 == 4.0 { 1.0 } else { 0.0 };
        var_guard352 = assign19150_e19950;

        let (assign19160_e19980, assign19160_e19980_d_n6, assign19160_e19980_d_n7, assign19160_e19980_d_n8, assign19160_e19980_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard350 == 0.0)) && (var_guard351 != 0.0)) && (var_guard352 != 0.0)) {
        let assign19160_e19966: f64 = (var_vav * var_vbrinvgat);
        let assign19160_e19969: f64 = (var_vav * var_vbrinvgat);
        let assign19160_e19970: f64 = (assign19160_e19966 * assign19160_e19969);
        let assign19160_e19973: f64 = (var_vav * var_vbrinvgat);
        let assign19160_e19974: f64 = (assign19160_e19970 * assign19160_e19973);
        let assign19160_e19977: f64 = (var_vav * var_vbrinvgat);
        let assign19160_e19978: f64 = (assign19160_e19974 * assign19160_e19977);
        (assign19160_e19978, (((((((var_vav * var_vbrinvgat_dn6) * assign19160_e19969) + (assign19160_e19966 * (var_vav * var_vbrinvgat_dn6))) * assign19160_e19973) + (assign19160_e19970 * (var_vav * var_vbrinvgat_dn6))) * assign19160_e19977) + (assign19160_e19974 * (var_vav * var_vbrinvgat_dn6))), (((((((var_vav * var_vbrinvgat_dn7) * assign19160_e19969) + (assign19160_e19966 * (var_vav * var_vbrinvgat_dn7))) * assign19160_e19973) + (assign19160_e19970 * (var_vav * var_vbrinvgat_dn7))) * assign19160_e19977) + (assign19160_e19974 * (var_vav * var_vbrinvgat_dn7))), (((((((var_vav * var_vbrinvgat_dn8) * assign19160_e19969) + (assign19160_e19966 * (var_vav * var_vbrinvgat_dn8))) * assign19160_e19973) + (assign19160_e19970 * (var_vav * var_vbrinvgat_dn8))) * assign19160_e19977) + (assign19160_e19974 * (var_vav * var_vbrinvgat_dn8))), (((((((var_vav * var_vbrinvgat_dn9) * assign19160_e19969) + (assign19160_e19966 * (var_vav * var_vbrinvgat_dn9))) * assign19160_e19973) + (assign19160_e19970 * (var_vav * var_vbrinvgat_dn9))) * assign19160_e19977) + (assign19160_e19974 * (var_vav * var_vbrinvgat_dn9))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign19160_e19980;
        var_tmp_dn6 = assign19160_e19980_d_n6;
        var_tmp_dn7 = assign19160_e19980_d_n7;
        var_tmp_dn8 = assign19160_e19980_d_n8;
        var_tmp_dn9 = assign19160_e19980_d_n9;

        let (assign19170_e20002, assign19170_e20002_d_n6, assign19170_e20002_d_n7, assign19170_e20002_d_n8, assign19170_e20002_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard350 == 0.0)) && (var_guard351 != 0.0)) && (var_guard352 == 0.0)) {
        let assign19170_e19997: f64 = (var_vav * var_vbrinvgat);
        let assign19170_e19998: f64 = (assign19170_e19997).abs();
        let assign19170_e20000: f64 = (assign19170_e19998).powf(p.p882);
        (assign19170_e20000, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign19170_e19998).powf(p.p882 - 1.0) * if assign19170_e19997 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) })) } } else { (assign19170_e20000 * (p.p882 * (if assign19170_e19997 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) } / assign19170_e19998))) }, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign19170_e19998).powf(p.p882 - 1.0) * if assign19170_e19997 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) })) } } else { (assign19170_e20000 * (p.p882 * (if assign19170_e19997 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) } / assign19170_e19998))) }, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign19170_e19998).powf(p.p882 - 1.0) * if assign19170_e19997 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) })) } } else { (assign19170_e20000 * (p.p882 * (if assign19170_e19997 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) } / assign19170_e19998))) }, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign19170_e19998).powf(p.p882 - 1.0) * if assign19170_e19997 >= 0.0 { (var_vav * var_vbrinvgat_dn9) } else { (-(var_vav * var_vbrinvgat_dn9)) })) } } else { (assign19170_e20000 * (p.p882 * (if assign19170_e19997 >= 0.0 { (var_vav * var_vbrinvgat_dn9) } else { (-(var_vav * var_vbrinvgat_dn9)) } / assign19170_e19998))) },)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign19170_e20002;
        var_tmp_dn6 = assign19170_e20002_d_n6;
        var_tmp_dn7 = assign19170_e20002_d_n7;
        var_tmp_dn8 = assign19170_e20002_d_n8;
        var_tmp_dn9 = assign19170_e20002_d_n9;

        let (assign19180_e20020, assign19180_e20020_d_n6, assign19180_e20020_d_n7, assign19180_e20020_d_n8, assign19180_e20020_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard350 == 0.0)) && (var_guard351 != 0.0)) {
        let assign19180_e20017: f64 = (1.0 - var_tmp);
        let assign19180_e20018: f64 = (1.0 / assign19180_e20017);
        (assign19180_e20018, (-((-var_tmp_dn6) / (assign19180_e20017 * assign19180_e20017))), (-((-var_tmp_dn7) / (assign19180_e20017 * assign19180_e20017))), (-((-var_tmp_dn8) / (assign19180_e20017 * assign19180_e20017))), (-((-var_tmp_dn9) / (assign19180_e20017 * assign19180_e20017))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign19180_e20020;
        var_fbreakdown_dn6 = assign19180_e20020_d_n6;
        var_fbreakdown_dn7 = assign19180_e20020_d_n7;
        var_fbreakdown_dn8 = assign19180_e20020_d_n8;
        var_fbreakdown_dn9 = assign19180_e20020_d_n9;

        let (assign19190_e20043, assign19190_e20043_d_n6, assign19190_e20043_d_n7, assign19190_e20043_d_n8, assign19190_e20043_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard350 == 0.0)) && (var_guard351 == 0.0)) {
        let assign19190_e20037: f64 = (var_alphaav * p.p879);
        let assign19190_e20038: f64 = (var_vav + assign19190_e20037);
        let assign19190_e20040: f64 = (assign19190_e20038 * var_slopegat);
        let assign19190_e20041: f64 = (var_fstopgat + assign19190_e20040);
        (assign19190_e20041, (assign19190_e20038 * var_slopegat_dn6), (assign19190_e20038 * var_slopegat_dn7), (assign19190_e20038 * var_slopegat_dn8), (assign19190_e20038 * var_slopegat_dn9),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign19190_e20043;
        var_fbreakdown_dn6 = assign19190_e20043_d_n6;
        var_fbreakdown_dn7 = assign19190_e20043_d_n7;
        var_fbreakdown_dn8 = assign19190_e20043_d_n8;
        var_fbreakdown_dn9 = assign19190_e20043_d_n9;

        let (assign19200_e20062, assign19200_e20062_d_n6, assign19200_e20062_d_n7, assign19200_e20062_d_n8, assign19200_e20062_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) {
        let assign19200_e20053: f64 = (var_id__blk212 + var_isrh);
        let assign19200_e20055: f64 = (assign19200_e20053 + var_itat);
        let assign19200_e20057: f64 = (assign19200_e20055 + var_ibbt);
        let assign19200_e20058: f64 = (p.p29 * assign19200_e20057);
        let assign19200_e20060: f64 = (assign19200_e20058 * var_fbreakdown);
        (assign19200_e20060, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign19200_e20058 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign19200_e20058 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign19200_e20058 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign19200_e20058 * var_fbreakdown_dn9)),)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign19200_e20062;
        var_ijungat_dn6 = assign19200_e20062_d_n6;
        var_ijungat_dn7 = assign19200_e20062_d_n7;
        var_ijungat_dn8 = assign19200_e20062_d_n8;
        var_ijungat_dn9 = assign19200_e20062_d_n9;

        let (assign19210_e20078, assign19210_e20078_d_n6, assign19210_e20078_d_n7, assign19210_e20078_d_n8, assign19210_e20078_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign19210_e20068: f64 = (var_absource_i * var_ijunbot);
        let assign19210_e20071: f64 = (var_lssource_i * var_ijunsti);
        let assign19210_e20072: f64 = (assign19210_e20068 + assign19210_e20071);
        let assign19210_e20075: f64 = (var_lgsource_i * var_ijungat);
        let assign19210_e20076: f64 = (assign19210_e20072 + assign19210_e20075);
        (assign19210_e20076, (((var_absource_i * var_ijunbot_dn6) + (var_lssource_i * var_ijunsti_dn6)) + (var_lgsource_i * var_ijungat_dn6)), (((var_absource_i * var_ijunbot_dn7) + (var_lssource_i * var_ijunsti_dn7)) + (var_lgsource_i * var_ijungat_dn7)), (((var_absource_i * var_ijunbot_dn8) + (var_lssource_i * var_ijunsti_dn8)) + (var_lgsource_i * var_ijungat_dn8)), (((var_absource_i * var_ijunbot_dn9) + (var_lssource_i * var_ijunsti_dn9)) + (var_lgsource_i * var_ijungat_dn9)),)
    } else {
        (var_i2, var_i2_dn6, var_i2_dn7, var_i2_dn8, var_i2_dn9,)
    }
};
        var_i2 = assign19210_e20078;
        var_i2_dn6 = assign19210_e20078_d_n6;
        var_i2_dn7 = assign19210_e20078_d_n7;
        var_i2_dn8 = assign19210_e20078_d_n8;
        var_i2_dn9 = assign19210_e20078_d_n9;

        let (assign19220_e20084,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign19220_e20084;

        let (assign19230_e20090,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign19230_e20090;

        let assign19240_e20102: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard353 = assign19240_e20102;

        let assign19320_e20188: f64 = if var_v3 < var_vmax_s { 1.0 } else { 0.0 };
        var_guard354 = assign19320_e20188;

        let assign19330_e20190: f64 = (-0.5);
        let assign19330_e20193: f64 = (var_v3 * var_phitdinv);
        let assign19330_e20194: f64 = (assign19330_e20190 * assign19330_e20193);
        let assign19330_e20195: f64 = (assign19330_e20194).abs();
        let assign19330_e20197: f64 = if assign19330_e20195 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard355 = assign19330_e20197;

        let (assign19340_e20215,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard353 != 0.0)) && (var_guard354 != 0.0)) && (var_guard355 != 0.0)) {
        let assign19340_e20208: f64 = (-0.5);
        let assign19340_e20211: f64 = (var_v3 * var_phitdinv);
        let assign19340_e20212: f64 = (assign19340_e20208 * assign19340_e20211);
        let assign19340_e20213: f64 = (assign19340_e20212).exp();
        (assign19340_e20213,)
    } else {
        (var_z,)
    }
};
        var_z = assign19340_e20215;

        let assign19350_e20217: f64 = (-0.5);
        let assign19350_e20220: f64 = (var_v3 * var_phitdinv);
        let assign19350_e20221: f64 = (assign19350_e20217 * assign19350_e20220);
        let assign19350_e20223: f64 = if assign19350_e20221 < 0.0 { 1.0 } else { 0.0 };
        var_guard356 = assign19350_e20223;

        let (assign19360_e20278,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard353 != 0.0)) && (var_guard354 != 0.0)) && (var_guard355 == 0.0)) && (var_guard356 != 0.0)) {
        let assign19360_e20239: f64 = (-230.25850929940458);
        let assign19360_e20241: f64 = (-0.5);
        let assign19360_e20244: f64 = (var_v3 * var_phitdinv);
        let assign19360_e20245: f64 = (assign19360_e20241 * assign19360_e20244);
        let assign19360_e20246: f64 = (assign19360_e20239 - assign19360_e20245);
        let assign19360_e20250: f64 = (-230.25850929940458);
        let assign19360_e20252: f64 = (-0.5);
        let assign19360_e20255: f64 = (var_v3 * var_phitdinv);
        let assign19360_e20256: f64 = (assign19360_e20252 * assign19360_e20255);
        let assign19360_e20257: f64 = (assign19360_e20250 - assign19360_e20256);
        let assign19360_e20260: f64 = (-230.25850929940458);
        let assign19360_e20262: f64 = (-0.5);
        let assign19360_e20265: f64 = (var_v3 * var_phitdinv);
        let assign19360_e20266: f64 = (assign19360_e20262 * assign19360_e20265);
        let assign19360_e20267: f64 = (assign19360_e20260 - assign19360_e20266);
        let assign19360_e20269: f64 = (assign19360_e20267 * 0.3333333333333333);
        let assign19360_e20270: f64 = (1.0 + assign19360_e20269);
        let assign19360_e20271: f64 = (assign19360_e20257 * assign19360_e20270);
        let assign19360_e20272: f64 = (0.5 * assign19360_e20271);
        let assign19360_e20273: f64 = (1.0 + assign19360_e20272);
        let assign19360_e20274: f64 = (assign19360_e20246 * assign19360_e20273);
        let assign19360_e20275: f64 = (1.0 + assign19360_e20274);
        let assign19360_e20276: f64 = (1e-100 / assign19360_e20275);
        (assign19360_e20276,)
    } else {
        (var_z,)
    }
};
        var_z = assign19360_e20278;

        let (assign19370_e20331,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard353 != 0.0)) && (var_guard354 != 0.0)) && (var_guard355 == 0.0)) && (var_guard356 == 0.0)) {
        let assign19370_e20295: f64 = (-0.5);
        let assign19370_e20298: f64 = (var_v3 * var_phitdinv);
        let assign19370_e20299: f64 = (assign19370_e20295 * assign19370_e20298);
        let assign19370_e20301: f64 = (assign19370_e20299 - 230.25850929940458);
        let assign19370_e20305: f64 = (-0.5);
        let assign19370_e20308: f64 = (var_v3 * var_phitdinv);
        let assign19370_e20309: f64 = (assign19370_e20305 * assign19370_e20308);
        let assign19370_e20311: f64 = (assign19370_e20309 - 230.25850929940458);
        let assign19370_e20314: f64 = (-0.5);
        let assign19370_e20317: f64 = (var_v3 * var_phitdinv);
        let assign19370_e20318: f64 = (assign19370_e20314 * assign19370_e20317);
        let assign19370_e20320: f64 = (assign19370_e20318 - 230.25850929940458);
        let assign19370_e20322: f64 = (assign19370_e20320 * 0.3333333333333333);
        let assign19370_e20323: f64 = (1.0 + assign19370_e20322);
        let assign19370_e20324: f64 = (assign19370_e20311 * assign19370_e20323);
        let assign19370_e20325: f64 = (0.5 * assign19370_e20324);
        let assign19370_e20326: f64 = (1.0 + assign19370_e20325);
        let assign19370_e20327: f64 = (assign19370_e20301 * assign19370_e20326);
        let assign19370_e20328: f64 = (1.0 + assign19370_e20327);
        let assign19370_e20329: f64 = (1e100 * assign19370_e20328);
        (assign19370_e20329,)
    } else {
        (var_z,)
    }
};
        var_z = assign19370_e20331;

        let (assign19380_e20343,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard353 != 0.0)) && (var_guard354 != 0.0)) {
        let assign19380_e20341: f64 = (1.0 / var_z);
        (assign19380_e20341,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign19380_e20343;

        let (assign19390_e20355,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard353 != 0.0)) && (var_guard354 != 0.0)) {
        let assign19390_e20353: f64 = (var_zinv * var_zinv);
        (assign19390_e20353,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign19390_e20355;

        let (assign19400_e20374,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard353 != 0.0)) && (var_guard354 == 0.0)) {
        let assign19400_e20367: f64 = (var_v3 - var_vmax_s);
        let assign19400_e20369: f64 = (assign19400_e20367 * var_phitdinv);
        let assign19400_e20370: f64 = (1.0 + assign19400_e20369);
        let assign19400_e20372: f64 = (assign19400_e20370 * var_exp_vmax_over_phitd_s);
        (assign19400_e20372,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign19400_e20374;

        let (assign19410_e20386,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard353 != 0.0)) && (var_guard354 == 0.0)) {
        let assign19410_e20384: f64 = (var_idmult).sqrt();
        (assign19410_e20384,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign19410_e20386;

        let (assign19420_e20399,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard353 != 0.0)) && (var_guard354 == 0.0)) {
        let assign19420_e20397: f64 = (1.0 / var_zinv);
        (assign19420_e20397,)
    } else {
        (var_z,)
    }
};
        var_z = assign19420_e20399;

        let (assign19430_e20409,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard353 != 0.0)) {
        let assign19430_e20407: f64 = (var_idmult - 1.0);
        (assign19430_e20407,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign19430_e20409;

        let assign19440_e20412: f64 = if var_v3 > 0.0 { 1.0 } else { 0.0 };
        var_guard357 = assign19440_e20412;

        let (assign19450_e20438,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard353 != 0.0)) && (var_guard357 != 0.0)) {
        let assign19450_e20424: f64 = (2.0 + var_z);
        let assign19450_e20427: f64 = (var_z + 1.0);
        let assign19450_e20430: f64 = (var_z + 3.0);
        let assign19450_e20431: f64 = (assign19450_e20427 * assign19450_e20430);
        let assign19450_e20432: f64 = (assign19450_e20431).sqrt();
        let assign19450_e20433: f64 = (assign19450_e20424 + assign19450_e20432);
        let assign19450_e20434: f64 = (assign19450_e20433).ln();
        let assign19450_e20435: f64 = (var_phitd * assign19450_e20434);
        let assign19450_e20436: f64 = (2.0 * assign19450_e20435);
        (assign19450_e20436,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign19450_e20438;

        let (assign19460_e20472,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard353 != 0.0)) && (var_guard357 == 0.0)) {
        let assign19460_e20448: f64 = (-var_v3);
        let assign19460_e20453: f64 = (2.0 * var_zinv);
        let assign19460_e20455: f64 = (assign19460_e20453 + 1.0);
        let assign19460_e20458: f64 = (1.0 + var_zinv);
        let assign19460_e20462: f64 = (3.0 * var_zinv);
        let assign19460_e20463: f64 = (1.0 + assign19460_e20462);
        let assign19460_e20464: f64 = (assign19460_e20458 * assign19460_e20463);
        let assign19460_e20465: f64 = (assign19460_e20464).sqrt();
        let assign19460_e20466: f64 = (assign19460_e20455 + assign19460_e20465);
        let assign19460_e20467: f64 = (assign19460_e20466).ln();
        let assign19460_e20468: f64 = (var_phitd * assign19460_e20467);
        let assign19460_e20469: f64 = (2.0 * assign19460_e20468);
        let assign19460_e20470: f64 = (assign19460_e20448 + assign19460_e20469);
        (assign19460_e20470,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign19460_e20472;

        let (assign19470_e20482,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard353 != 0.0)) {
        let assign19470_e20480: f64 = (var_vbimin_s - var_two_psistar);
        (assign19470_e20480,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign19470_e20482;

        let (assign19480_e20509,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard353 != 0.0)) {
        let assign19480_e20491: f64 = (var_v3 + var_vjlim);
        let assign19480_e20494: f64 = (var_v3 - var_vjlim);
        let assign19480_e20497: f64 = (var_v3 - var_vjlim);
        let assign19480_e20498: f64 = (assign19480_e20494 * assign19480_e20497);
        let assign19480_e20501: f64 = (4.0 * var_phitd);
        let assign19480_e20503: f64 = (assign19480_e20501 * var_phitd);
        let assign19480_e20504: f64 = (assign19480_e20498 + assign19480_e20503);
        let assign19480_e20505: f64 = (assign19480_e20504).sqrt();
        let assign19480_e20506: f64 = (assign19480_e20491 - assign19480_e20505);
        let assign19480_e20507: f64 = (0.5 * assign19480_e20506);
        (assign19480_e20507,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign19480_e20509;

        let (assign19490_e20536,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard353 != 0.0)) {
        let assign19490_e20518: f64 = (var_v3 + var_vbbtlim_s);
        let assign19490_e20521: f64 = (var_v3 - var_vbbtlim_s);
        let assign19490_e20524: f64 = (var_v3 - var_vbbtlim_s);
        let assign19490_e20525: f64 = (assign19490_e20521 * assign19490_e20524);
        let assign19490_e20528: f64 = (4.0 * var_phitr);
        let assign19490_e20530: f64 = (assign19490_e20528 * var_phitr);
        let assign19490_e20531: f64 = (assign19490_e20525 + assign19490_e20530);
        let assign19490_e20532: f64 = (assign19490_e20531).sqrt();
        let assign19490_e20533: f64 = (assign19490_e20518 - assign19490_e20532);
        let assign19490_e20534: f64 = (0.5 * assign19490_e20533);
        (assign19490_e20534,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign19490_e20536;

        let (assign19500_e20563,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard353 != 0.0)) {
        let assign19500_e20545: f64 = var_v3;
        let assign19500_e20548: f64 = var_v3;
        let assign19500_e20551: f64 = var_v3;
        let assign19500_e20552: f64 = (assign19500_e20548 * assign19500_e20551);
        let assign19500_e20555: f64 = (4.0 * 1e-6);
        let assign19500_e20557: f64 = (assign19500_e20555 * 1e-6);
        let assign19500_e20558: f64 = (assign19500_e20552 + assign19500_e20557);
        let assign19500_e20559: f64 = (assign19500_e20558).sqrt();
        let assign19500_e20560: f64 = (assign19500_e20545 - assign19500_e20559);
        let assign19500_e20561: f64 = (0.5 * assign19500_e20560);
        (assign19500_e20561,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign19500_e20563;

        let assign19510_e20566: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard358 = assign19510_e20566;

        let (assign19520_e20574, assign19520_e20574_d_n6, assign19520_e20574_d_n7, assign19520_e20574_d_n8, assign19520_e20574_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign19520_e20574;
        var_ijunbot_dn6 = assign19520_e20574_d_n6;
        var_ijunbot_dn7 = assign19520_e20574_d_n7;
        var_ijunbot_dn8 = assign19520_e20574_d_n8;
        var_ijunbot_dn9 = assign19520_e20574_d_n9;

        let (assign19530_e20585,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) {
        let assign19530_e20583: f64 = (var_idsatbot * var_idmult);
        (assign19530_e20583,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign19530_e20585;

        let assign19540_e20592: f64 = if ((p.p857 == 0.0) && (p.p862 == 0.0)) { 1.0 } else { 0.0 };
        var_guard359 = assign19540_e20592;

        let (assign19550_e20603, assign19550_e20603_d_n6, assign19550_e20603_d_n7, assign19550_e20603_d_n8, assign19550_e20603_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard359 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign19550_e20603;
        var_isrh_dn6 = assign19550_e20603_d_n6;
        var_isrh_dn7 = assign19550_e20603_d_n7;
        var_isrh_dn8 = assign19550_e20603_d_n8;
        var_isrh_dn9 = assign19550_e20603_d_n9;

        let (assign19560_e20617,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard359 == 0.0)) {
        let assign19560_e20615: f64 = (var_vbibot - var_vjsrh);
        (assign19560_e20615,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign19560_e20617;

        let (assign19570_e20636,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard359 == 0.0)) {
        let assign19570_e20631: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign19570_e20632: f64 = (1.0 - assign19570_e20631);
        let assign19570_e20633: f64 = (assign19570_e20632).sqrt();
        let assign19570_e20634: f64 = (1.0 - assign19570_e20633);
        (assign19570_e20634,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign19570_e20636;

        let assign19580_e20639: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };
        var_guard360 = assign19580_e20639;

        let (assign19590_e20653,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard359 == 0.0)) && (var_guard360 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign19590_e20653;

        let (assign19600_e20685,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard359 == 0.0)) && (var_guard360 == 0.0)) {
        let assign19600_e20668: f64 = (var_wsrhstep * var_wsrhstep);
        let assign19600_e20670: f64 = (var_wsrhstep).ln();
        let assign19600_e20671: f64 = (assign19600_e20668 * assign19600_e20670);
        let assign19600_e20674: f64 = (1.0 - var_wsrhstep);
        let assign19600_e20675: f64 = (assign19600_e20671 / assign19600_e20674);
        let assign19600_e20677: f64 = (assign19600_e20675 + var_wsrhstep);
        let assign19600_e20681: f64 = (2.0 * p.p848);
        let assign19600_e20682: f64 = (1.0 - assign19600_e20681);
        let assign19600_e20683: f64 = (assign19600_e20677 * assign19600_e20682);
        (assign19600_e20683,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign19600_e20685;

        *var_dwsrh_slot = var_dwsrh;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_guard350_slot = var_guard350;
        *var_guard351_slot = var_guard351;
        *var_guard352_slot = var_guard352;
        *var_guard353_slot = var_guard353;
        *var_guard354_slot = var_guard354;
        *var_guard355_slot = var_guard355;
        *var_guard356_slot = var_guard356;
        *var_guard357_slot = var_guard357;
        *var_guard358_slot = var_guard358;
        *var_guard359_slot = var_guard359;
        *var_guard360_slot = var_guard360;
        *var_i2_slot = var_i2;
        *var_i2_dn6_slot = var_i2_dn6;
        *var_i2_dn7_slot = var_i2_dn7;
        *var_i2_dn8_slot = var_i2_dn8;
        *var_i2_dn9_slot = var_i2_dn9;
        *var_id__blk212_slot = var_id__blk212;
        *var_idmult_slot = var_idmult;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijunbot_dn9_slot = var_ijunbot_dn9;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_ijungat_dn9_slot = var_ijungat_dn9;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_two_psistar_slot = var_two_psistar;
        *var_vav_slot = var_vav;
        *var_vbbt_slot = var_vbbt;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_vjlim_slot = var_vjlim;
        *var_vjsrh_slot = var_vjsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_34(
        p: &Parameters,
        var_atatbot: f64,
        var_berfc: f64,
        var_btatpartbot: f64,
        var_cerfc: f64,
        var_dwsrh: f64,
        var_ftdbot: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard358: f64,
        var_guard359: f64,
        var_one_minus_pbot: f64,
        var_one_over_one_minus_pbot: f64,
        var_perfc: f64,
        var_vbi_minus_vjsrh: f64,
        var_vbirbotinv: f64,
        var_wdepnulrbot: f64,
        var_wsrhstep: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_asrh_dn9_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_btat_dn9_slot: &mut f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfcpos_dn9_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_erfctimesexpmtat_dn9_slot: &mut f64,
        var_guard361_slot: &mut f64,
        var_guard362_slot: &mut f64,
        var_guard363_slot: &mut f64,
        var_guard364_slot: &mut f64,
        var_guard365_slot: &mut f64,
        var_guard366_slot: &mut f64,
        var_guard367_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ktat_dn9_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_ltat_dn9_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_mtat_dn9_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_sqrtumax_dn9_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_terfc_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_dn9_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umax_dn9_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxbeforelimiting_dn9_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_dn9_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wdep_dn9_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wgamma_dn9_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_wtat_dn9_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_xerfc_dn9_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
        var_ysq_dn9_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_asrh_dn9: f64 = *var_asrh_dn9_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_btat_dn9: f64 = *var_btat_dn9_slot;
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfcpos_dn9: f64 = *var_erfcpos_dn9_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_erfctimesexpmtat_dn9: f64 = *var_erfctimesexpmtat_dn9_slot;
        let mut var_guard361: f64 = *var_guard361_slot;
        let mut var_guard362: f64 = *var_guard362_slot;
        let mut var_guard363: f64 = *var_guard363_slot;
        let mut var_guard364: f64 = *var_guard364_slot;
        let mut var_guard365: f64 = *var_guard365_slot;
        let mut var_guard366: f64 = *var_guard366_slot;
        let mut var_guard367: f64 = *var_guard367_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ktat_dn9: f64 = *var_ktat_dn9_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_ltat_dn9: f64 = *var_ltat_dn9_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_mtat_dn9: f64 = *var_mtat_dn9_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_sqrtumax_dn9: f64 = *var_sqrtumax_dn9_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_terfc_dn9: f64 = *var_terfc_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_twoatatoverthreebtat_dn9: f64 = *var_twoatatoverthreebtat_dn9_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umax_dn9: f64 = *var_umax_dn9_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxbeforelimiting_dn9: f64 = *var_umaxbeforelimiting_dn9_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_umaxpoweronepointfive_dn9: f64 = *var_umaxpoweronepointfive_dn9_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wdep_dn9: f64 = *var_wdep_dn9_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wgamma_dn9: f64 = *var_wgamma_dn9_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_wtat_dn9: f64 = *var_wtat_dn9_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_xerfc_dn9: f64 = *var_xerfc_dn9_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;
        let mut var_ysq_dn9: f64 = *var_ysq_dn9_slot;

        let (assign19610_e20699,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard359 == 0.0)) {
        let assign19610_e20697: f64 = (var_wsrhstep + var_dwsrh);
        (assign19610_e20697,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign19610_e20699;

        let assign19620_e20702: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };
        var_guard361 = assign19620_e20702;

        let (assign19630_e20719, assign19630_e20719_d_n6, assign19630_e20719_d_n7, assign19630_e20719_d_n8, assign19630_e20719_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard359 == 0.0)) && (var_guard361 != 0.0)) {
        let assign19630_e20716: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign19630_e20717: f64 = (assign19630_e20716).sqrt();
        (assign19630_e20717, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign19630_e20719;
        var_tmp_dn6 = assign19630_e20719_d_n6;
        var_tmp_dn7 = assign19630_e20719_d_n7;
        var_tmp_dn8 = assign19630_e20719_d_n8;
        var_tmp_dn9 = assign19630_e20719_d_n9;

        let (assign19640_e20738, assign19640_e20738_d_n6, assign19640_e20738_d_n7, assign19640_e20738_d_n8, assign19640_e20738_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard359 == 0.0)) && (var_guard361 == 0.0)) {
        let assign19640_e20734: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign19640_e20736: f64 = (assign19640_e20734).powf(p.p848);
        (assign19640_e20736, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign19640_e20738;
        var_tmp_dn6 = assign19640_e20738_d_n6;
        var_tmp_dn7 = assign19640_e20738_d_n7;
        var_tmp_dn8 = assign19640_e20738_d_n8;
        var_tmp_dn9 = assign19640_e20738_d_n9;

        let (assign19650_e20752, assign19650_e20752_d_n6, assign19650_e20752_d_n7, assign19650_e20752_d_n8, assign19650_e20752_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard359 == 0.0)) {
        let assign19650_e20750: f64 = (var_wdepnulrbot * var_tmp);
        (assign19650_e20750, (var_wdepnulrbot * var_tmp_dn6), (var_wdepnulrbot * var_tmp_dn7), (var_wdepnulrbot * var_tmp_dn8), (var_wdepnulrbot * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign19650_e20752;
        var_wdep_dn6 = assign19650_e20752_d_n6;
        var_wdep_dn7 = assign19650_e20752_d_n7;
        var_wdep_dn8 = assign19650_e20752_d_n8;
        var_wdep_dn9 = assign19650_e20752_d_n9;

        let (assign19660_e20770, assign19660_e20770_d_n6, assign19660_e20770_d_n7, assign19660_e20770_d_n8, assign19660_e20770_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard359 == 0.0)) {
        let assign19660_e20765: f64 = (var_zinv - 1.0);
        let assign19660_e20767: f64 = (assign19660_e20765 * var_wdep);
        let assign19660_e20768: f64 = (var_ftdbot * assign19660_e20767);
        (assign19660_e20768, (var_ftdbot * (assign19660_e20765 * var_wdep_dn6)), (var_ftdbot * (assign19660_e20765 * var_wdep_dn7)), (var_ftdbot * (assign19660_e20765 * var_wdep_dn8)), (var_ftdbot * (assign19660_e20765 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign19660_e20770;
        var_asrh_dn6 = assign19660_e20770_d_n6;
        var_asrh_dn7 = assign19660_e20770_d_n7;
        var_asrh_dn8 = assign19660_e20770_d_n8;
        var_asrh_dn9 = assign19660_e20770_d_n9;

        let (assign19670_e20786, assign19670_e20786_d_n6, assign19670_e20786_d_n7, assign19670_e20786_d_n8, assign19670_e20786_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard359 == 0.0)) {
        let assign19670_e20783: f64 = (var_asrh * var_wsrh);
        let assign19670_e20784: f64 = (p.p857 * assign19670_e20783);
        (assign19670_e20784, (p.p857 * (var_asrh_dn6 * var_wsrh)), (p.p857 * (var_asrh_dn7 * var_wsrh)), (p.p857 * (var_asrh_dn8 * var_wsrh)), (p.p857 * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign19670_e20786;
        var_isrh_dn6 = assign19670_e20786_d_n6;
        var_isrh_dn7 = assign19670_e20786_d_n7;
        var_isrh_dn8 = assign19670_e20786_d_n8;
        var_isrh_dn9 = assign19670_e20786_d_n9;

        let assign19680_e20789: f64 = if p.p862 == 0.0 { 1.0 } else { 0.0 };
        var_guard362 = assign19680_e20789;

        let (assign19690_e20800, assign19690_e20800_d_n6, assign19690_e20800_d_n7, assign19690_e20800_d_n8, assign19690_e20800_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign19690_e20800;
        var_itat_dn6 = assign19690_e20800_d_n6;
        var_itat_dn7 = assign19690_e20800_d_n7;
        var_itat_dn8 = assign19690_e20800_d_n8;
        var_itat_dn9 = assign19690_e20800_d_n9;

        let (assign19700_e20818, assign19700_e20818_d_n6, assign19700_e20818_d_n7, assign19700_e20818_d_n8, assign19700_e20818_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) {
        let assign19700_e20813: f64 = (var_wdep * var_one_minus_pbot);
        let assign19700_e20815: f64 = (assign19700_e20813 / var_vbi_minus_vjsrh);
        let assign19700_e20816: f64 = (var_btatpartbot * assign19700_e20815);
        (assign19700_e20816, (var_btatpartbot * ((var_wdep_dn6 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn7 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn8 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn9 * var_one_minus_pbot) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign19700_e20818;
        var_btat_dn6 = assign19700_e20818_d_n6;
        var_btat_dn7 = assign19700_e20818_d_n7;
        var_btat_dn8 = assign19700_e20818_d_n8;
        var_btat_dn9 = assign19700_e20818_d_n9;

        let (assign19710_e20834, assign19710_e20834_d_n6, assign19710_e20834_d_n7, assign19710_e20834_d_n8, assign19710_e20834_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) {
        let assign19710_e20830: f64 = (0.666666666666667 * var_atatbot);
        let assign19710_e20832: f64 = (assign19710_e20830 / var_btat);
        (assign19710_e20832, (-((assign19710_e20830 * var_btat_dn6) / (var_btat * var_btat))), (-((assign19710_e20830 * var_btat_dn7) / (var_btat * var_btat))), (-((assign19710_e20830 * var_btat_dn8) / (var_btat * var_btat))), (-((assign19710_e20830 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign19710_e20834;
        var_twoatatoverthreebtat_dn6 = assign19710_e20834_d_n6;
        var_twoatatoverthreebtat_dn7 = assign19710_e20834_d_n7;
        var_twoatatoverthreebtat_dn8 = assign19710_e20834_d_n8;
        var_twoatatoverthreebtat_dn9 = assign19710_e20834_d_n9;

        let (assign19720_e20848, assign19720_e20848_d_n6, assign19720_e20848_d_n7, assign19720_e20848_d_n8, assign19720_e20848_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) {
        let assign19720_e20846: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign19720_e20846, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign19720_e20848;
        var_umaxbeforelimiting_dn6 = assign19720_e20848_d_n6;
        var_umaxbeforelimiting_dn7 = assign19720_e20848_d_n7;
        var_umaxbeforelimiting_dn8 = assign19720_e20848_d_n8;
        var_umaxbeforelimiting_dn9 = assign19720_e20848_d_n9;

        let (assign19730_e20869, assign19730_e20869_d_n6, assign19730_e20869_d_n7, assign19730_e20869_d_n8, assign19730_e20869_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) {
        let assign19730_e20860: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign19730_e20863: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign19730_e20865: f64 = (assign19730_e20863 + 1.0);
        let assign19730_e20866: f64 = (assign19730_e20860 / assign19730_e20865);
        let assign19730_e20867: f64 = (assign19730_e20866).sqrt();
        (assign19730_e20867, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign19730_e20865) - (assign19730_e20860 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign19730_e20865 * assign19730_e20865)) / (2.0 * assign19730_e20867)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign19730_e20865) - (assign19730_e20860 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign19730_e20865 * assign19730_e20865)) / (2.0 * assign19730_e20867)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign19730_e20865) - (assign19730_e20860 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign19730_e20865 * assign19730_e20865)) / (2.0 * assign19730_e20867)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign19730_e20865) - (assign19730_e20860 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign19730_e20865 * assign19730_e20865)) / (2.0 * assign19730_e20867)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign19730_e20869;
        var_umax_dn6 = assign19730_e20869_d_n6;
        var_umax_dn7 = assign19730_e20869_d_n7;
        var_umax_dn8 = assign19730_e20869_d_n8;
        var_umax_dn9 = assign19730_e20869_d_n9;

        let (assign19740_e20882, assign19740_e20882_d_n6, assign19740_e20882_d_n7, assign19740_e20882_d_n8, assign19740_e20882_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) {
        let assign19740_e20880: f64 = (var_umax).sqrt();
        (assign19740_e20880, (var_umax_dn6 / (2.0 * assign19740_e20880)), (var_umax_dn7 / (2.0 * assign19740_e20880)), (var_umax_dn8 / (2.0 * assign19740_e20880)), (var_umax_dn9 / (2.0 * assign19740_e20880)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign19740_e20882;
        var_sqrtumax_dn6 = assign19740_e20882_d_n6;
        var_sqrtumax_dn7 = assign19740_e20882_d_n7;
        var_sqrtumax_dn8 = assign19740_e20882_d_n8;
        var_sqrtumax_dn9 = assign19740_e20882_d_n9;

        let (assign19750_e20896, assign19750_e20896_d_n6, assign19750_e20896_d_n7, assign19750_e20896_d_n8, assign19750_e20896_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) {
        let assign19750_e20894: f64 = (var_umax * var_sqrtumax);
        (assign19750_e20894, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign19750_e20896;
        var_umaxpoweronepointfive_dn6 = assign19750_e20896_d_n6;
        var_umaxpoweronepointfive_dn7 = assign19750_e20896_d_n7;
        var_umaxpoweronepointfive_dn8 = assign19750_e20896_d_n8;
        var_umaxpoweronepointfive_dn9 = assign19750_e20896_d_n9;

        let assign19760_e20898: f64 = (-p.p848);
        let assign19760_e20900: f64 = (assign19760_e20898 * var_one_over_one_minus_pbot);
        let assign19760_e20902: f64 = (-1.0);
        let assign19760_e20903: f64 = if assign19760_e20900 == assign19760_e20902 { 1.0 } else { 0.0 };
        var_guard363 = assign19760_e20903;

        let (assign19770_e20923, assign19770_e20923_d_n6, assign19770_e20923_d_n7, assign19770_e20923_d_n8, assign19770_e20923_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) && (var_guard363 != 0.0)) {
        let assign19770_e20919: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign19770_e20920: f64 = (1.0 + assign19770_e20919);
        let assign19770_e20921: f64 = (1.0 / assign19770_e20920);
        (assign19770_e20921, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign19770_e20920 * assign19770_e20920))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign19770_e20920 * assign19770_e20920))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign19770_e20920 * assign19770_e20920))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign19770_e20920 * assign19770_e20920))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign19770_e20923;
        var_wgamma_dn6 = assign19770_e20923_d_n6;
        var_wgamma_dn7 = assign19770_e20923_d_n7;
        var_wgamma_dn8 = assign19770_e20923_d_n8;
        var_wgamma_dn9 = assign19770_e20923_d_n9;

        let (assign19780_e20947, assign19780_e20947_d_n6, assign19780_e20947_d_n7, assign19780_e20947_d_n8, assign19780_e20947_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) && (var_guard363 == 0.0)) {
        let assign19780_e20939: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign19780_e20940: f64 = (1.0 + assign19780_e20939);
        let assign19780_e20942: f64 = (-p.p848);
        let assign19780_e20944: f64 = (assign19780_e20942 * var_one_over_one_minus_pbot);
        let assign19780_e20945: f64 = (assign19780_e20940).powf(assign19780_e20944);
        (assign19780_e20945, if 0.0 == 0.0 && ((assign19780_e20944) as f64).is_finite() && ((assign19780_e20944) as f64).fract() == 0.0 { if assign19780_e20944 == 0.0 { 0.0 } else { (assign19780_e20944 * ((assign19780_e20940).powf(assign19780_e20944 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign19780_e20945 * (assign19780_e20944 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign19780_e20940))) }, if 0.0 == 0.0 && ((assign19780_e20944) as f64).is_finite() && ((assign19780_e20944) as f64).fract() == 0.0 { if assign19780_e20944 == 0.0 { 0.0 } else { (assign19780_e20944 * ((assign19780_e20940).powf(assign19780_e20944 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign19780_e20945 * (assign19780_e20944 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign19780_e20940))) }, if 0.0 == 0.0 && ((assign19780_e20944) as f64).is_finite() && ((assign19780_e20944) as f64).fract() == 0.0 { if assign19780_e20944 == 0.0 { 0.0 } else { (assign19780_e20944 * ((assign19780_e20940).powf(assign19780_e20944 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign19780_e20945 * (assign19780_e20944 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign19780_e20940))) }, if 0.0 == 0.0 && ((assign19780_e20944) as f64).is_finite() && ((assign19780_e20944) as f64).fract() == 0.0 { if assign19780_e20944 == 0.0 { 0.0 } else { (assign19780_e20944 * ((assign19780_e20940).powf(assign19780_e20944 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign19780_e20945 * (assign19780_e20944 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign19780_e20940))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign19780_e20947;
        var_wgamma_dn6 = assign19780_e20947_d_n6;
        var_wgamma_dn7 = assign19780_e20947_d_n7;
        var_wgamma_dn8 = assign19780_e20947_d_n8;
        var_wgamma_dn9 = assign19780_e20947_d_n9;

        let (assign19790_e20965, assign19790_e20965_d_n6, assign19790_e20965_d_n7, assign19790_e20965_d_n8, assign19790_e20965_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) {
        let assign19790_e20959: f64 = (var_wsrh * var_wgamma);
        let assign19790_e20962: f64 = (var_wsrh + var_wgamma);
        let assign19790_e20963: f64 = (assign19790_e20959 / assign19790_e20962);
        (assign19790_e20963, ((((var_wsrh * var_wgamma_dn6) * assign19790_e20962) - (assign19790_e20959 * var_wgamma_dn6)) / (assign19790_e20962 * assign19790_e20962)), ((((var_wsrh * var_wgamma_dn7) * assign19790_e20962) - (assign19790_e20959 * var_wgamma_dn7)) / (assign19790_e20962 * assign19790_e20962)), ((((var_wsrh * var_wgamma_dn8) * assign19790_e20962) - (assign19790_e20959 * var_wgamma_dn8)) / (assign19790_e20962 * assign19790_e20962)), ((((var_wsrh * var_wgamma_dn9) * assign19790_e20962) - (assign19790_e20959 * var_wgamma_dn9)) / (assign19790_e20962 * assign19790_e20962)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign19790_e20965;
        var_wtat_dn6 = assign19790_e20965_d_n6;
        var_wtat_dn7 = assign19790_e20965_d_n7;
        var_wtat_dn8 = assign19790_e20965_d_n8;
        var_wtat_dn9 = assign19790_e20965_d_n9;

        let (assign19800_e20982, assign19800_e20982_d_n6, assign19800_e20982_d_n7, assign19800_e20982_d_n8, assign19800_e20982_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) {
        let assign19800_e20978: f64 = (var_btat / var_sqrtumax);
        let assign19800_e20979: f64 = (0.375 * assign19800_e20978);
        let assign19800_e20980: f64 = (assign19800_e20979).sqrt();
        (assign19800_e20980, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign19800_e20980)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign19800_e20980)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign19800_e20980)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign19800_e20980)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign19800_e20982;
        var_ktat_dn6 = assign19800_e20982_d_n6;
        var_ktat_dn7 = assign19800_e20982_d_n7;
        var_ktat_dn8 = assign19800_e20982_d_n8;
        var_ktat_dn9 = assign19800_e20982_d_n9;

        let (assign19810_e21000, assign19810_e21000_d_n6, assign19810_e21000_d_n7, assign19810_e21000_d_n8, assign19810_e21000_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) {
        let assign19810_e20995: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign19810_e20996: f64 = (2.0 * assign19810_e20995);
        let assign19810_e20998: f64 = (assign19810_e20996 - var_umax);
        (assign19810_e20998, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign19810_e21000;
        var_ltat_dn6 = assign19810_e21000_d_n6;
        var_ltat_dn7 = assign19810_e21000_d_n7;
        var_ltat_dn8 = assign19810_e21000_d_n8;
        var_ltat_dn9 = assign19810_e21000_d_n9;

        let (assign19820_e21026, assign19820_e21026_d_n6, assign19820_e21026_d_n7, assign19820_e21026_d_n8, assign19820_e21026_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) {
        let assign19820_e21012: f64 = (var_atatbot * var_twoatatoverthreebtat);
        let assign19820_e21014: f64 = (assign19820_e21012 * var_sqrtumax);
        let assign19820_e21017: f64 = (var_atatbot * var_umax);
        let assign19820_e21018: f64 = (assign19820_e21014 - assign19820_e21017);
        let assign19820_e21022: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign19820_e21023: f64 = (0.5 * assign19820_e21022);
        let assign19820_e21024: f64 = (assign19820_e21018 + assign19820_e21023);
        (assign19820_e21024, (((((var_atatbot * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign19820_e21012 * var_sqrtumax_dn6)) - (var_atatbot * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign19820_e21012 * var_sqrtumax_dn7)) - (var_atatbot * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign19820_e21012 * var_sqrtumax_dn8)) - (var_atatbot * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatbot * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign19820_e21012 * var_sqrtumax_dn9)) - (var_atatbot * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign19820_e21026;
        var_mtat_dn6 = assign19820_e21026_d_n6;
        var_mtat_dn7 = assign19820_e21026_d_n7;
        var_mtat_dn8 = assign19820_e21026_d_n8;
        var_mtat_dn9 = assign19820_e21026_d_n9;

        let (assign19830_e21042, assign19830_e21042_d_n6, assign19830_e21042_d_n7, assign19830_e21042_d_n8, assign19830_e21042_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) {
        let assign19830_e21038: f64 = (var_ltat - 1.0);
        let assign19830_e21040: f64 = (assign19830_e21038 * var_ktat);
        (assign19830_e21040, ((var_ltat_dn6 * var_ktat) + (assign19830_e21038 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign19830_e21038 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign19830_e21038 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign19830_e21038 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign19830_e21042;
        var_xerfc_dn6 = assign19830_e21042_d_n6;
        var_xerfc_dn7 = assign19830_e21042_d_n7;
        var_xerfc_dn8 = assign19830_e21042_d_n8;
        var_xerfc_dn9 = assign19830_e21042_d_n9;

        let (assign19840_e21056, assign19840_e21056_d_n6, assign19840_e21056_d_n7, assign19840_e21056_d_n8, assign19840_e21056_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) {
        let assign19840_e21054: f64 = (var_xerfc * var_xerfc);
        (assign19840_e21054, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign19840_e21056;
        var_ysq_dn6 = assign19840_e21056_d_n6;
        var_ysq_dn7 = assign19840_e21056_d_n7;
        var_ysq_dn8 = assign19840_e21056_d_n8;
        var_ysq_dn9 = assign19840_e21056_d_n9;

        let assign19850_e21059: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard364 = assign19850_e21059;

        let (assign19860_e21079, assign19860_e21079_d_n6, assign19860_e21079_d_n7, assign19860_e21079_d_n8, assign19860_e21079_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) && (var_guard364 != 0.0)) {
        let assign19860_e21075: f64 = (var_perfc * var_xerfc);
        let assign19860_e21076: f64 = (1.0 + assign19860_e21075);
        let assign19860_e21077: f64 = (1.0 / assign19860_e21076);
        (assign19860_e21077, (-((var_perfc * var_xerfc_dn6) / (assign19860_e21076 * assign19860_e21076))), (-((var_perfc * var_xerfc_dn7) / (assign19860_e21076 * assign19860_e21076))), (-((var_perfc * var_xerfc_dn8) / (assign19860_e21076 * assign19860_e21076))), (-((var_perfc * var_xerfc_dn9) / (assign19860_e21076 * assign19860_e21076))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign19860_e21079;
        var_terfc_dn6 = assign19860_e21079_d_n6;
        var_terfc_dn7 = assign19860_e21079_d_n7;
        var_terfc_dn8 = assign19860_e21079_d_n8;
        var_terfc_dn9 = assign19860_e21079_d_n9;

        let (assign19870_e21100, assign19870_e21100_d_n6, assign19870_e21100_d_n7, assign19870_e21100_d_n8, assign19870_e21100_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) && (var_guard364 == 0.0)) {
        let assign19870_e21096: f64 = (var_perfc * var_xerfc);
        let assign19870_e21097: f64 = (1.0 - assign19870_e21096);
        let assign19870_e21098: f64 = (1.0 / assign19870_e21097);
        (assign19870_e21098, (-((-(var_perfc * var_xerfc_dn6)) / (assign19870_e21097 * assign19870_e21097))), (-((-(var_perfc * var_xerfc_dn7)) / (assign19870_e21097 * assign19870_e21097))), (-((-(var_perfc * var_xerfc_dn8)) / (assign19870_e21097 * assign19870_e21097))), (-((-(var_perfc * var_xerfc_dn9)) / (assign19870_e21097 * assign19870_e21097))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign19870_e21100;
        var_terfc_dn6 = assign19870_e21100_d_n6;
        var_terfc_dn7 = assign19870_e21100_d_n7;
        var_terfc_dn8 = assign19870_e21100_d_n8;
        var_terfc_dn9 = assign19870_e21100_d_n9;

        let assign19880_e21102: f64 = (-var_ysq);
        let assign19880_e21104: f64 = (assign19880_e21102 + var_mtat);
        let assign19880_e21106: f64 = (-230.25850929940458);
        let assign19880_e21107: f64 = if assign19880_e21104 > assign19880_e21106 { 1.0 } else { 0.0 };
        var_guard365 = assign19880_e21107;

        let (assign19890_e21125, assign19890_e21125_d_n6, assign19890_e21125_d_n7, assign19890_e21125_d_n8, assign19890_e21125_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) && (var_guard365 != 0.0)) {
        let assign19890_e21120: f64 = (-var_ysq);
        let assign19890_e21122: f64 = (assign19890_e21120 + var_mtat);
        let assign19890_e21123: f64 = (assign19890_e21122).exp();
        (assign19890_e21123, (assign19890_e21123 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign19890_e21123 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign19890_e21123 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign19890_e21123 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign19890_e21125;
        var_tmp_dn6 = assign19890_e21125_d_n6;
        var_tmp_dn7 = assign19890_e21125_d_n7;
        var_tmp_dn8 = assign19890_e21125_d_n8;
        var_tmp_dn9 = assign19890_e21125_d_n9;

        let (assign19900_e21174, assign19900_e21174_d_n6, assign19900_e21174_d_n7, assign19900_e21174_d_n8, assign19900_e21174_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) && (var_guard365 == 0.0)) {
        let assign19900_e21141: f64 = (-230.25850929940458);
        let assign19900_e21143: f64 = (-var_ysq);
        let assign19900_e21145: f64 = (assign19900_e21143 + var_mtat);
        let assign19900_e21146: f64 = (assign19900_e21141 - assign19900_e21145);
        let assign19900_e21150: f64 = (-230.25850929940458);
        let assign19900_e21152: f64 = (-var_ysq);
        let assign19900_e21154: f64 = (assign19900_e21152 + var_mtat);
        let assign19900_e21155: f64 = (assign19900_e21150 - assign19900_e21154);
        let assign19900_e21158: f64 = (-230.25850929940458);
        let assign19900_e21160: f64 = (-var_ysq);
        let assign19900_e21162: f64 = (assign19900_e21160 + var_mtat);
        let assign19900_e21163: f64 = (assign19900_e21158 - assign19900_e21162);
        let assign19900_e21165: f64 = (assign19900_e21163 * 0.3333333333333333);
        let assign19900_e21166: f64 = (1.0 + assign19900_e21165);
        let assign19900_e21167: f64 = (assign19900_e21155 * assign19900_e21166);
        let assign19900_e21168: f64 = (0.5 * assign19900_e21167);
        let assign19900_e21169: f64 = (1.0 + assign19900_e21168);
        let assign19900_e21170: f64 = (assign19900_e21146 * assign19900_e21169);
        let assign19900_e21171: f64 = (1.0 + assign19900_e21170);
        let assign19900_e21172: f64 = (1e-100 / assign19900_e21171);
        (assign19900_e21172, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign19900_e21169) + (assign19900_e21146 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign19900_e21166) + (assign19900_e21155 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign19900_e21171 * assign19900_e21171))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign19900_e21169) + (assign19900_e21146 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign19900_e21166) + (assign19900_e21155 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign19900_e21171 * assign19900_e21171))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign19900_e21169) + (assign19900_e21146 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign19900_e21166) + (assign19900_e21155 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign19900_e21171 * assign19900_e21171))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign19900_e21169) + (assign19900_e21146 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign19900_e21166) + (assign19900_e21155 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign19900_e21171 * assign19900_e21171))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign19900_e21174;
        var_tmp_dn6 = assign19900_e21174_d_n6;
        var_tmp_dn7 = assign19900_e21174_d_n7;
        var_tmp_dn8 = assign19900_e21174_d_n8;
        var_tmp_dn9 = assign19900_e21174_d_n9;

        let (assign19910_e21204, assign19910_e21204_d_n6, assign19910_e21204_d_n7, assign19910_e21204_d_n8, assign19910_e21204_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) {
        let assign19910_e21186: f64 = (0.29214664 * var_terfc);
        let assign19910_e21190: f64 = (var_terfc * var_terfc);
        let assign19910_e21191: f64 = (var_berfc * assign19910_e21190);
        let assign19910_e21192: f64 = (assign19910_e21186 + assign19910_e21191);
        let assign19910_e21196: f64 = (var_terfc * var_terfc);
        let assign19910_e21198: f64 = (assign19910_e21196 * var_terfc);
        let assign19910_e21199: f64 = (var_cerfc * assign19910_e21198);
        let assign19910_e21200: f64 = (assign19910_e21192 + assign19910_e21199);
        let assign19910_e21202: f64 = (assign19910_e21200 * var_tmp);
        (assign19910_e21202, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign19910_e21196 * var_terfc_dn6)))) * var_tmp) + (assign19910_e21200 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign19910_e21196 * var_terfc_dn7)))) * var_tmp) + (assign19910_e21200 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign19910_e21196 * var_terfc_dn8)))) * var_tmp) + (assign19910_e21200 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign19910_e21196 * var_terfc_dn9)))) * var_tmp) + (assign19910_e21200 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign19910_e21204;
        var_erfcpos_dn6 = assign19910_e21204_d_n6;
        var_erfcpos_dn7 = assign19910_e21204_d_n7;
        var_erfcpos_dn8 = assign19910_e21204_d_n8;
        var_erfcpos_dn9 = assign19910_e21204_d_n9;

        let assign19920_e21207: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard366 = assign19920_e21207;

        let (assign19930_e21221, assign19930_e21221_d_n6, assign19930_e21221_d_n7, assign19930_e21221_d_n8, assign19930_e21221_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) && (var_guard366 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign19930_e21221;
        var_erfctimesexpmtat_dn6 = assign19930_e21221_d_n6;
        var_erfctimesexpmtat_dn7 = assign19930_e21221_d_n7;
        var_erfctimesexpmtat_dn8 = assign19930_e21221_d_n8;
        var_erfctimesexpmtat_dn9 = assign19930_e21221_d_n9;

        let assign19940_e21224: f64 = (-230.25850929940458);
        let assign19940_e21225: f64 = if var_mtat > assign19940_e21224 { 1.0 } else { 0.0 };
        var_guard367 = assign19940_e21225;

        let (assign19950_e21243, assign19950_e21243_d_n6, assign19950_e21243_d_n7, assign19950_e21243_d_n8, assign19950_e21243_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) && (var_guard366 == 0.0)) && (var_guard367 != 0.0)) {
        let assign19950_e21241: f64 = (var_mtat).exp();
        (assign19950_e21241, (assign19950_e21241 * var_mtat_dn6), (assign19950_e21241 * var_mtat_dn7), (assign19950_e21241 * var_mtat_dn8), (assign19950_e21241 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign19950_e21243;
        var_tmp_dn6 = assign19950_e21243_d_n6;
        var_tmp_dn7 = assign19950_e21243_d_n7;
        var_tmp_dn8 = assign19950_e21243_d_n8;
        var_tmp_dn9 = assign19950_e21243_d_n9;

        let (assign19960_e21286, assign19960_e21286_d_n6, assign19960_e21286_d_n7, assign19960_e21286_d_n8, assign19960_e21286_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) && (var_guard366 == 0.0)) && (var_guard367 == 0.0)) {
        let assign19960_e21262: f64 = (-230.25850929940458);
        let assign19960_e21264: f64 = (assign19960_e21262 - var_mtat);
        let assign19960_e21268: f64 = (-230.25850929940458);
        let assign19960_e21270: f64 = (assign19960_e21268 - var_mtat);
        let assign19960_e21273: f64 = (-230.25850929940458);
        let assign19960_e21275: f64 = (assign19960_e21273 - var_mtat);
        let assign19960_e21277: f64 = (assign19960_e21275 * 0.3333333333333333);
        let assign19960_e21278: f64 = (1.0 + assign19960_e21277);
        let assign19960_e21279: f64 = (assign19960_e21270 * assign19960_e21278);
        let assign19960_e21280: f64 = (0.5 * assign19960_e21279);
        let assign19960_e21281: f64 = (1.0 + assign19960_e21280);
        let assign19960_e21282: f64 = (assign19960_e21264 * assign19960_e21281);
        let assign19960_e21283: f64 = (1.0 + assign19960_e21282);
        let assign19960_e21284: f64 = (1e-100 / assign19960_e21283);
        (assign19960_e21284, (-((1e-100 * (((-var_mtat_dn6) * assign19960_e21281) + (assign19960_e21264 * (0.5 * (((-var_mtat_dn6) * assign19960_e21278) + (assign19960_e21270 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign19960_e21283 * assign19960_e21283))), (-((1e-100 * (((-var_mtat_dn7) * assign19960_e21281) + (assign19960_e21264 * (0.5 * (((-var_mtat_dn7) * assign19960_e21278) + (assign19960_e21270 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign19960_e21283 * assign19960_e21283))), (-((1e-100 * (((-var_mtat_dn8) * assign19960_e21281) + (assign19960_e21264 * (0.5 * (((-var_mtat_dn8) * assign19960_e21278) + (assign19960_e21270 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign19960_e21283 * assign19960_e21283))), (-((1e-100 * (((-var_mtat_dn9) * assign19960_e21281) + (assign19960_e21264 * (0.5 * (((-var_mtat_dn9) * assign19960_e21278) + (assign19960_e21270 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign19960_e21283 * assign19960_e21283))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign19960_e21286;
        var_tmp_dn6 = assign19960_e21286_d_n6;
        var_tmp_dn7 = assign19960_e21286_d_n7;
        var_tmp_dn8 = assign19960_e21286_d_n8;
        var_tmp_dn9 = assign19960_e21286_d_n9;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_asrh_dn9_slot = var_asrh_dn9;
        *var_btat_slot = var_btat;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_btat_dn9_slot = var_btat_dn9;
        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfcpos_dn9_slot = var_erfcpos_dn9;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_erfctimesexpmtat_dn9_slot = var_erfctimesexpmtat_dn9;
        *var_guard361_slot = var_guard361;
        *var_guard362_slot = var_guard362;
        *var_guard363_slot = var_guard363;
        *var_guard364_slot = var_guard364;
        *var_guard365_slot = var_guard365;
        *var_guard366_slot = var_guard366;
        *var_guard367_slot = var_guard367;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ktat_dn9_slot = var_ktat_dn9;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_ltat_dn9_slot = var_ltat_dn9;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_mtat_dn9_slot = var_mtat_dn9;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_sqrtumax_dn9_slot = var_sqrtumax_dn9;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_terfc_dn9_slot = var_terfc_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_twoatatoverthreebtat_dn9_slot = var_twoatatoverthreebtat_dn9;
        *var_umax_slot = var_umax;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umax_dn9_slot = var_umax_dn9;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxbeforelimiting_dn9_slot = var_umaxbeforelimiting_dn9;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_umaxpoweronepointfive_dn9_slot = var_umaxpoweronepointfive_dn9;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wdep_dn9_slot = var_wdep_dn9;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wgamma_dn9_slot = var_wgamma_dn9;
        *var_wsrh_slot = var_wsrh;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_wtat_dn9_slot = var_wtat_dn9;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_xerfc_dn9_slot = var_xerfc_dn9;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
        *var_ysq_dn9_slot = var_ysq_dn9;
    }

    pub(super) fn stamp_transient_block_35(
        p: &Parameters,
        var_alphaav: f64,
        var_atatbot: f64,
        var_erfcpos: f64,
        var_erfcpos_dn6: f64,
        var_erfcpos_dn7: f64,
        var_erfcpos_dn8: f64,
        var_erfcpos_dn9: f64,
        var_fbbtbot: f64,
        var_fstopbot: f64,
        var_ftdsti: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard358: f64,
        var_guard362: f64,
        var_guard366: f64,
        var_idmult: f64,
        var_idsatsti: f64,
        var_ktat: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_ktat_dn9: f64,
        var_lssource_i: f64,
        var_one_over_one_minus_pbot: f64,
        var_slopebot: f64,
        var_two_psistar: f64,
        var_v3: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirbotinv: f64,
        var_vbirstiinv: f64,
        var_vbisti: f64,
        var_vbrinvbot: f64,
        var_vjsrh: f64,
        var_wdepnulrinvbot: f64,
        var_wdepnulrsti: f64,
        var_wtat: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_wtat_dn9: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_asrh_dn9_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_erfctimesexpmtat_dn9_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_fmaxr_dn9_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_gammamax_dn9_slot: &mut f64,
        var_guard368_slot: &mut f64,
        var_guard369_slot: &mut f64,
        var_guard370_slot: &mut f64,
        var_guard371_slot: &mut f64,
        var_guard372_slot: &mut f64,
        var_guard373_slot: &mut f64,
        var_guard374_slot: &mut f64,
        var_guard375_slot: &mut f64,
        var_guard376_slot: &mut f64,
        var_guard377_slot: &mut f64,
        var_guard378_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_id__blk212_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijunbot_dn9_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_ijunsti_dn9_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wdep_dn9_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_asrh_dn9: f64 = *var_asrh_dn9_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_erfctimesexpmtat_dn9: f64 = *var_erfctimesexpmtat_dn9_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fbreakdown_dn9: f64 = *var_fbreakdown_dn9_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_fmaxr_dn9: f64 = *var_fmaxr_dn9_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_gammamax_dn9: f64 = *var_gammamax_dn9_slot;
        let mut var_guard368: f64 = *var_guard368_slot;
        let mut var_guard369: f64 = *var_guard369_slot;
        let mut var_guard370: f64 = *var_guard370_slot;
        let mut var_guard371: f64 = *var_guard371_slot;
        let mut var_guard372: f64 = *var_guard372_slot;
        let mut var_guard373: f64 = *var_guard373_slot;
        let mut var_guard374: f64 = *var_guard374_slot;
        let mut var_guard375: f64 = *var_guard375_slot;
        let mut var_guard376: f64 = *var_guard376_slot;
        let mut var_guard377: f64 = *var_guard377_slot;
        let mut var_guard378: f64 = *var_guard378_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_id__blk212: f64 = *var_id__blk212_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijunbot_dn9: f64 = *var_ijunbot_dn9_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_ijunsti_dn9: f64 = *var_ijunsti_dn9_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wdep_dn9: f64 = *var_wdep_dn9_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;

        let (assign19970_e21305, assign19970_e21305_d_n6, assign19970_e21305_d_n7, assign19970_e21305_d_n8, assign19970_e21305_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) && (var_guard366 == 0.0)) {
        let assign19970_e21301: f64 = (2.0 * var_tmp);
        let assign19970_e21303: f64 = (assign19970_e21301 - var_erfcpos);
        (assign19970_e21303, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign19970_e21305;
        var_erfctimesexpmtat_dn6 = assign19970_e21305_d_n6;
        var_erfctimesexpmtat_dn7 = assign19970_e21305_d_n7;
        var_erfctimesexpmtat_dn8 = assign19970_e21305_d_n8;
        var_erfctimesexpmtat_dn9 = assign19970_e21305_d_n9;

        let (assign19980_e21325, assign19980_e21325_d_n6, assign19980_e21325_d_n7, assign19980_e21325_d_n8, assign19980_e21325_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) {
        let assign19980_e21317: f64 = (1.772453850905516 * 0.5);
        let assign19980_e21320: f64 = (var_atatbot * var_erfctimesexpmtat);
        let assign19980_e21322: f64 = (assign19980_e21320 / var_ktat);
        let assign19980_e21323: f64 = (assign19980_e21317 * assign19980_e21322);
        (assign19980_e21323, (assign19980_e21317 * ((((var_atatbot * var_erfctimesexpmtat_dn6) * var_ktat) - (assign19980_e21320 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign19980_e21317 * ((((var_atatbot * var_erfctimesexpmtat_dn7) * var_ktat) - (assign19980_e21320 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign19980_e21317 * ((((var_atatbot * var_erfctimesexpmtat_dn8) * var_ktat) - (assign19980_e21320 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign19980_e21317 * ((((var_atatbot * var_erfctimesexpmtat_dn9) * var_ktat) - (assign19980_e21320 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign19980_e21325;
        var_gammamax_dn6 = assign19980_e21325_d_n6;
        var_gammamax_dn7 = assign19980_e21325_d_n7;
        var_gammamax_dn8 = assign19980_e21325_d_n8;
        var_gammamax_dn9 = assign19980_e21325_d_n9;

        let (assign19990_e21343, assign19990_e21343_d_n6, assign19990_e21343_d_n7, assign19990_e21343_d_n8, assign19990_e21343_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard362 == 0.0)) {
        let assign19990_e21338: f64 = (var_asrh * var_gammamax);
        let assign19990_e21340: f64 = (assign19990_e21338 * var_wtat);
        let assign19990_e21341: f64 = (p.p862 * assign19990_e21340);
        (assign19990_e21341, (p.p862 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign19990_e21338 * var_wtat_dn6))), (p.p862 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign19990_e21338 * var_wtat_dn7))), (p.p862 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign19990_e21338 * var_wtat_dn8))), (p.p862 * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign19990_e21338 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign19990_e21343;
        var_itat_dn6 = assign19990_e21343_d_n6;
        var_itat_dn7 = assign19990_e21343_d_n7;
        var_itat_dn8 = assign19990_e21343_d_n8;
        var_itat_dn9 = assign19990_e21343_d_n9;

        let assign20000_e21346: f64 = if p.p868 == 0.0 { 1.0 } else { 0.0 };
        var_guard368 = assign20000_e21346;

        let (assign20010_e21357, assign20010_e21357_d_n6, assign20010_e21357_d_n7, assign20010_e21357_d_n8, assign20010_e21357_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard368 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign20010_e21357;
        var_ibbt_dn6 = assign20010_e21357_d_n6;
        var_ibbt_dn7 = assign20010_e21357_d_n7;
        var_ibbt_dn8 = assign20010_e21357_d_n8;
        var_ibbt_dn9 = assign20010_e21357_d_n9;

        let assign20020_e21360: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };
        var_guard369 = assign20020_e21360;

        let (assign20030_e21379, assign20030_e21379_d_n6, assign20030_e21379_d_n7, assign20030_e21379_d_n8, assign20030_e21379_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard368 == 0.0)) && (var_guard369 != 0.0)) {
        let assign20030_e21374: f64 = (p.p845 - var_vbbt);
        let assign20030_e21376: f64 = (assign20030_e21374 * var_vbirbotinv);
        let assign20030_e21377: f64 = (assign20030_e21376).sqrt();
        (assign20030_e21377, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20030_e21379;
        var_tmp_dn6 = assign20030_e21379_d_n6;
        var_tmp_dn7 = assign20030_e21379_d_n7;
        var_tmp_dn8 = assign20030_e21379_d_n8;
        var_tmp_dn9 = assign20030_e21379_d_n9;

        let (assign20040_e21400, assign20040_e21400_d_n6, assign20040_e21400_d_n7, assign20040_e21400_d_n8, assign20040_e21400_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard368 == 0.0)) && (var_guard369 == 0.0)) {
        let assign20040_e21394: f64 = (p.p845 - var_vbbt);
        let assign20040_e21396: f64 = (assign20040_e21394 * var_vbirbotinv);
        let assign20040_e21398: f64 = (assign20040_e21396).powf(p.p848);
        (assign20040_e21398, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20040_e21400;
        var_tmp_dn6 = assign20040_e21400_d_n6;
        var_tmp_dn7 = assign20040_e21400_d_n7;
        var_tmp_dn8 = assign20040_e21400_d_n8;
        var_tmp_dn9 = assign20040_e21400_d_n9;

        let (assign20050_e21420, assign20050_e21420_d_n6, assign20050_e21420_d_n7, assign20050_e21420_d_n8, assign20050_e21420_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard368 == 0.0)) {
        let assign20050_e21413: f64 = (p.p845 - var_vbbt);
        let assign20050_e21415: f64 = (assign20050_e21413 * var_wdepnulrinvbot);
        let assign20050_e21417: f64 = (assign20050_e21415 / var_tmp);
        let assign20050_e21418: f64 = (var_one_over_one_minus_pbot * assign20050_e21417);
        (assign20050_e21418, (var_one_over_one_minus_pbot * (-((assign20050_e21415 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign20050_e21415 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign20050_e21415 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign20050_e21415 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign20050_e21420;
        var_fmaxr_dn6 = assign20050_e21420_d_n6;
        var_fmaxr_dn7 = assign20050_e21420_d_n7;
        var_fmaxr_dn8 = assign20050_e21420_d_n8;
        var_fmaxr_dn9 = assign20050_e21420_d_n9;

        let assign20060_e21422: f64 = (-var_fbbtbot);
        let assign20060_e21424: f64 = (assign20060_e21422 / var_fmaxr);
        let assign20060_e21425: f64 = (assign20060_e21424).abs();
        let assign20060_e21427: f64 = if assign20060_e21425 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard370 = assign20060_e21427;

        let (assign20070_e21445, assign20070_e21445_d_n6, assign20070_e21445_d_n7, assign20070_e21445_d_n8, assign20070_e21445_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard368 == 0.0)) && (var_guard370 != 0.0)) {
        let assign20070_e21440: f64 = (-var_fbbtbot);
        let assign20070_e21442: f64 = (assign20070_e21440 / var_fmaxr);
        let assign20070_e21443: f64 = (assign20070_e21442).exp();
        (assign20070_e21443, (assign20070_e21443 * (-((assign20070_e21440 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign20070_e21443 * (-((assign20070_e21440 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign20070_e21443 * (-((assign20070_e21440 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign20070_e21443 * (-((assign20070_e21440 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20070_e21445;
        var_tmp_dn6 = assign20070_e21445_d_n6;
        var_tmp_dn7 = assign20070_e21445_d_n7;
        var_tmp_dn8 = assign20070_e21445_d_n8;
        var_tmp_dn9 = assign20070_e21445_d_n9;

        let assign20080_e21447: f64 = (-var_fbbtbot);
        let assign20080_e21449: f64 = (assign20080_e21447 / var_fmaxr);
        let assign20080_e21451: f64 = if assign20080_e21449 < 0.0 { 1.0 } else { 0.0 };
        var_guard371 = assign20080_e21451;

        let (assign20090_e21502, assign20090_e21502_d_n6, assign20090_e21502_d_n7, assign20090_e21502_d_n8, assign20090_e21502_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard368 == 0.0)) && (var_guard370 == 0.0)) && (var_guard371 != 0.0)) {
        let assign20090_e21469: f64 = (-230.25850929940458);
        let assign20090_e21471: f64 = (-var_fbbtbot);
        let assign20090_e21473: f64 = (assign20090_e21471 / var_fmaxr);
        let assign20090_e21474: f64 = (assign20090_e21469 - assign20090_e21473);
        let assign20090_e21478: f64 = (-230.25850929940458);
        let assign20090_e21480: f64 = (-var_fbbtbot);
        let assign20090_e21482: f64 = (assign20090_e21480 / var_fmaxr);
        let assign20090_e21483: f64 = (assign20090_e21478 - assign20090_e21482);
        let assign20090_e21486: f64 = (-230.25850929940458);
        let assign20090_e21488: f64 = (-var_fbbtbot);
        let assign20090_e21490: f64 = (assign20090_e21488 / var_fmaxr);
        let assign20090_e21491: f64 = (assign20090_e21486 - assign20090_e21490);
        let assign20090_e21493: f64 = (assign20090_e21491 * 0.3333333333333333);
        let assign20090_e21494: f64 = (1.0 + assign20090_e21493);
        let assign20090_e21495: f64 = (assign20090_e21483 * assign20090_e21494);
        let assign20090_e21496: f64 = (0.5 * assign20090_e21495);
        let assign20090_e21497: f64 = (1.0 + assign20090_e21496);
        let assign20090_e21498: f64 = (assign20090_e21474 * assign20090_e21497);
        let assign20090_e21499: f64 = (1.0 + assign20090_e21498);
        let assign20090_e21500: f64 = (1e-100 / assign20090_e21499);
        (assign20090_e21500, (-((1e-100 * (((-(-((assign20090_e21471 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign20090_e21497) + (assign20090_e21474 * (0.5 * (((-(-((assign20090_e21480 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign20090_e21494) + (assign20090_e21483 * ((-(-((assign20090_e21488 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign20090_e21499 * assign20090_e21499))), (-((1e-100 * (((-(-((assign20090_e21471 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign20090_e21497) + (assign20090_e21474 * (0.5 * (((-(-((assign20090_e21480 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign20090_e21494) + (assign20090_e21483 * ((-(-((assign20090_e21488 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign20090_e21499 * assign20090_e21499))), (-((1e-100 * (((-(-((assign20090_e21471 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign20090_e21497) + (assign20090_e21474 * (0.5 * (((-(-((assign20090_e21480 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign20090_e21494) + (assign20090_e21483 * ((-(-((assign20090_e21488 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign20090_e21499 * assign20090_e21499))), (-((1e-100 * (((-(-((assign20090_e21471 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign20090_e21497) + (assign20090_e21474 * (0.5 * (((-(-((assign20090_e21480 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign20090_e21494) + (assign20090_e21483 * ((-(-((assign20090_e21488 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign20090_e21499 * assign20090_e21499))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20090_e21502;
        var_tmp_dn6 = assign20090_e21502_d_n6;
        var_tmp_dn7 = assign20090_e21502_d_n7;
        var_tmp_dn8 = assign20090_e21502_d_n8;
        var_tmp_dn9 = assign20090_e21502_d_n9;

        let (assign20100_e21551, assign20100_e21551_d_n6, assign20100_e21551_d_n7, assign20100_e21551_d_n8, assign20100_e21551_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard368 == 0.0)) && (var_guard370 == 0.0)) && (var_guard371 == 0.0)) {
        let assign20100_e21521: f64 = (-var_fbbtbot);
        let assign20100_e21523: f64 = (assign20100_e21521 / var_fmaxr);
        let assign20100_e21525: f64 = (assign20100_e21523 - 230.25850929940458);
        let assign20100_e21529: f64 = (-var_fbbtbot);
        let assign20100_e21531: f64 = (assign20100_e21529 / var_fmaxr);
        let assign20100_e21533: f64 = (assign20100_e21531 - 230.25850929940458);
        let assign20100_e21536: f64 = (-var_fbbtbot);
        let assign20100_e21538: f64 = (assign20100_e21536 / var_fmaxr);
        let assign20100_e21540: f64 = (assign20100_e21538 - 230.25850929940458);
        let assign20100_e21542: f64 = (assign20100_e21540 * 0.3333333333333333);
        let assign20100_e21543: f64 = (1.0 + assign20100_e21542);
        let assign20100_e21544: f64 = (assign20100_e21533 * assign20100_e21543);
        let assign20100_e21545: f64 = (0.5 * assign20100_e21544);
        let assign20100_e21546: f64 = (1.0 + assign20100_e21545);
        let assign20100_e21547: f64 = (assign20100_e21525 * assign20100_e21546);
        let assign20100_e21548: f64 = (1.0 + assign20100_e21547);
        let assign20100_e21549: f64 = (1e100 * assign20100_e21548);
        (assign20100_e21549, (1e100 * (((-((assign20100_e21521 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign20100_e21546) + (assign20100_e21525 * (0.5 * (((-((assign20100_e21529 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign20100_e21543) + (assign20100_e21533 * ((-((assign20100_e21536 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign20100_e21521 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign20100_e21546) + (assign20100_e21525 * (0.5 * (((-((assign20100_e21529 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign20100_e21543) + (assign20100_e21533 * ((-((assign20100_e21536 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign20100_e21521 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign20100_e21546) + (assign20100_e21525 * (0.5 * (((-((assign20100_e21529 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign20100_e21543) + (assign20100_e21533 * ((-((assign20100_e21536 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign20100_e21521 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign20100_e21546) + (assign20100_e21525 * (0.5 * (((-((assign20100_e21529 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign20100_e21543) + (assign20100_e21533 * ((-((assign20100_e21536 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20100_e21551;
        var_tmp_dn6 = assign20100_e21551_d_n6;
        var_tmp_dn7 = assign20100_e21551_d_n7;
        var_tmp_dn8 = assign20100_e21551_d_n8;
        var_tmp_dn9 = assign20100_e21551_d_n9;

        let (assign20110_e21571, assign20110_e21571_d_n6, assign20110_e21571_d_n7, assign20110_e21571_d_n8, assign20110_e21571_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard368 == 0.0)) {
        let assign20110_e21564: f64 = (var_v3 * var_fmaxr);
        let assign20110_e21566: f64 = (assign20110_e21564 * var_fmaxr);
        let assign20110_e21568: f64 = (assign20110_e21566 * var_tmp);
        let assign20110_e21569: f64 = (p.p868 * assign20110_e21568);
        (assign20110_e21569, (p.p868 * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign20110_e21564 * var_fmaxr_dn6)) * var_tmp) + (assign20110_e21566 * var_tmp_dn6))), (p.p868 * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign20110_e21564 * var_fmaxr_dn7)) * var_tmp) + (assign20110_e21566 * var_tmp_dn7))), (p.p868 * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign20110_e21564 * var_fmaxr_dn8)) * var_tmp) + (assign20110_e21566 * var_tmp_dn8))), (p.p868 * (((((var_v3 * var_fmaxr_dn9) * var_fmaxr) + (assign20110_e21564 * var_fmaxr_dn9)) * var_tmp) + (assign20110_e21566 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign20110_e21571;
        var_ibbt_dn6 = assign20110_e21571_d_n6;
        var_ibbt_dn7 = assign20110_e21571_d_n7;
        var_ibbt_dn8 = assign20110_e21571_d_n8;
        var_ibbt_dn9 = assign20110_e21571_d_n9;

        let assign20120_e21574: f64 = if p.p877 > 1000.0 { 1.0 } else { 0.0 };
        var_guard372 = assign20120_e21574;

        let (assign20130_e21585, assign20130_e21585_d_n6, assign20130_e21585_d_n7, assign20130_e21585_d_n8, assign20130_e21585_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard372 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign20130_e21585;
        var_fbreakdown_dn6 = assign20130_e21585_d_n6;
        var_fbreakdown_dn7 = assign20130_e21585_d_n7;
        var_fbreakdown_dn8 = assign20130_e21585_d_n8;
        var_fbreakdown_dn9 = assign20130_e21585_d_n9;

        let assign20140_e21588: f64 = (-var_alphaav);
        let assign20140_e21590: f64 = (assign20140_e21588 * p.p877);
        let assign20140_e21591: f64 = if var_vav > assign20140_e21590 { 1.0 } else { 0.0 };
        var_guard373 = assign20140_e21591;

        let assign20150_e21594: f64 = if p.p880 == 4.0 { 1.0 } else { 0.0 };
        var_guard374 = assign20150_e21594;

        let (assign20160_e21624, assign20160_e21624_d_n6, assign20160_e21624_d_n7, assign20160_e21624_d_n8, assign20160_e21624_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard372 == 0.0)) && (var_guard373 != 0.0)) && (var_guard374 != 0.0)) {
        let assign20160_e21610: f64 = (var_vav * var_vbrinvbot);
        let assign20160_e21613: f64 = (var_vav * var_vbrinvbot);
        let assign20160_e21614: f64 = (assign20160_e21610 * assign20160_e21613);
        let assign20160_e21617: f64 = (var_vav * var_vbrinvbot);
        let assign20160_e21618: f64 = (assign20160_e21614 * assign20160_e21617);
        let assign20160_e21621: f64 = (var_vav * var_vbrinvbot);
        let assign20160_e21622: f64 = (assign20160_e21618 * assign20160_e21621);
        (assign20160_e21622, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20160_e21624;
        var_tmp_dn6 = assign20160_e21624_d_n6;
        var_tmp_dn7 = assign20160_e21624_d_n7;
        var_tmp_dn8 = assign20160_e21624_d_n8;
        var_tmp_dn9 = assign20160_e21624_d_n9;

        let (assign20170_e21646, assign20170_e21646_d_n6, assign20170_e21646_d_n7, assign20170_e21646_d_n8, assign20170_e21646_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard372 == 0.0)) && (var_guard373 != 0.0)) && (var_guard374 == 0.0)) {
        let assign20170_e21641: f64 = (var_vav * var_vbrinvbot);
        let assign20170_e21642: f64 = (assign20170_e21641).abs();
        let assign20170_e21644: f64 = (assign20170_e21642).powf(p.p880);
        (assign20170_e21644, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20170_e21646;
        var_tmp_dn6 = assign20170_e21646_d_n6;
        var_tmp_dn7 = assign20170_e21646_d_n7;
        var_tmp_dn8 = assign20170_e21646_d_n8;
        var_tmp_dn9 = assign20170_e21646_d_n9;

        let (assign20180_e21664, assign20180_e21664_d_n6, assign20180_e21664_d_n7, assign20180_e21664_d_n8, assign20180_e21664_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard372 == 0.0)) && (var_guard373 != 0.0)) {
        let assign20180_e21661: f64 = (1.0 - var_tmp);
        let assign20180_e21662: f64 = (1.0 / assign20180_e21661);
        (assign20180_e21662, (-((-var_tmp_dn6) / (assign20180_e21661 * assign20180_e21661))), (-((-var_tmp_dn7) / (assign20180_e21661 * assign20180_e21661))), (-((-var_tmp_dn8) / (assign20180_e21661 * assign20180_e21661))), (-((-var_tmp_dn9) / (assign20180_e21661 * assign20180_e21661))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign20180_e21664;
        var_fbreakdown_dn6 = assign20180_e21664_d_n6;
        var_fbreakdown_dn7 = assign20180_e21664_d_n7;
        var_fbreakdown_dn8 = assign20180_e21664_d_n8;
        var_fbreakdown_dn9 = assign20180_e21664_d_n9;

        let (assign20190_e21687, assign20190_e21687_d_n6, assign20190_e21687_d_n7, assign20190_e21687_d_n8, assign20190_e21687_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) && (var_guard372 == 0.0)) && (var_guard373 == 0.0)) {
        let assign20190_e21681: f64 = (var_alphaav * p.p877);
        let assign20190_e21682: f64 = (var_vav + assign20190_e21681);
        let assign20190_e21684: f64 = (assign20190_e21682 * var_slopebot);
        let assign20190_e21685: f64 = (var_fstopbot + assign20190_e21684);
        (assign20190_e21685, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign20190_e21687;
        var_fbreakdown_dn6 = assign20190_e21687_d_n6;
        var_fbreakdown_dn7 = assign20190_e21687_d_n7;
        var_fbreakdown_dn8 = assign20190_e21687_d_n8;
        var_fbreakdown_dn9 = assign20190_e21687_d_n9;

        let (assign20200_e21706, assign20200_e21706_d_n6, assign20200_e21706_d_n7, assign20200_e21706_d_n8, assign20200_e21706_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard358 == 0.0)) {
        let assign20200_e21697: f64 = (var_id__blk212 + var_isrh);
        let assign20200_e21699: f64 = (assign20200_e21697 + var_itat);
        let assign20200_e21701: f64 = (assign20200_e21699 + var_ibbt);
        let assign20200_e21702: f64 = (p.p29 * assign20200_e21701);
        let assign20200_e21704: f64 = (assign20200_e21702 * var_fbreakdown);
        (assign20200_e21704, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign20200_e21702 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign20200_e21702 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign20200_e21702 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign20200_e21702 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign20200_e21706;
        var_ijunbot_dn6 = assign20200_e21706_d_n6;
        var_ijunbot_dn7 = assign20200_e21706_d_n7;
        var_ijunbot_dn8 = assign20200_e21706_d_n8;
        var_ijunbot_dn9 = assign20200_e21706_d_n9;

        let assign20210_e21709: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard375 = assign20210_e21709;

        let (assign20220_e21717, assign20220_e21717_d_n6, assign20220_e21717_d_n7, assign20220_e21717_d_n8, assign20220_e21717_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign20220_e21717;
        var_ijunsti_dn6 = assign20220_e21717_d_n6;
        var_ijunsti_dn7 = assign20220_e21717_d_n7;
        var_ijunsti_dn8 = assign20220_e21717_d_n8;
        var_ijunsti_dn9 = assign20220_e21717_d_n9;

        let (assign20230_e21728,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) {
        let assign20230_e21726: f64 = (var_idsatsti * var_idmult);
        (assign20230_e21726,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign20230_e21728;

        let assign20240_e21735: f64 = if ((p.p858 == 0.0) && (p.p863 == 0.0)) { 1.0 } else { 0.0 };
        var_guard376 = assign20240_e21735;

        let (assign20250_e21746, assign20250_e21746_d_n6, assign20250_e21746_d_n7, assign20250_e21746_d_n8, assign20250_e21746_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard376 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign20250_e21746;
        var_isrh_dn6 = assign20250_e21746_d_n6;
        var_isrh_dn7 = assign20250_e21746_d_n7;
        var_isrh_dn8 = assign20250_e21746_d_n8;
        var_isrh_dn9 = assign20250_e21746_d_n9;

        let (assign20260_e21760,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard376 == 0.0)) {
        let assign20260_e21758: f64 = (var_vbisti - var_vjsrh);
        (assign20260_e21758,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign20260_e21760;

        let (assign20270_e21779,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard376 == 0.0)) {
        let assign20270_e21774: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign20270_e21775: f64 = (1.0 - assign20270_e21774);
        let assign20270_e21776: f64 = (assign20270_e21775).sqrt();
        let assign20270_e21777: f64 = (1.0 - assign20270_e21776);
        (assign20270_e21777,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign20270_e21779;

        let assign20280_e21782: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };
        var_guard377 = assign20280_e21782;

        let (assign20290_e21796,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard376 == 0.0)) && (var_guard377 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign20290_e21796;

        let (assign20300_e21828,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard376 == 0.0)) && (var_guard377 == 0.0)) {
        let assign20300_e21811: f64 = (var_wsrhstep * var_wsrhstep);
        let assign20300_e21813: f64 = (var_wsrhstep).ln();
        let assign20300_e21814: f64 = (assign20300_e21811 * assign20300_e21813);
        let assign20300_e21817: f64 = (1.0 - var_wsrhstep);
        let assign20300_e21818: f64 = (assign20300_e21814 / assign20300_e21817);
        let assign20300_e21820: f64 = (assign20300_e21818 + var_wsrhstep);
        let assign20300_e21824: f64 = (2.0 * p.p849);
        let assign20300_e21825: f64 = (1.0 - assign20300_e21824);
        let assign20300_e21826: f64 = (assign20300_e21820 * assign20300_e21825);
        (assign20300_e21826,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign20300_e21828;

        let (assign20310_e21842,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard376 == 0.0)) {
        let assign20310_e21840: f64 = (var_wsrhstep + var_dwsrh);
        (assign20310_e21840,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign20310_e21842;

        let assign20320_e21845: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };
        var_guard378 = assign20320_e21845;

        let (assign20330_e21862, assign20330_e21862_d_n6, assign20330_e21862_d_n7, assign20330_e21862_d_n8, assign20330_e21862_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard376 == 0.0)) && (var_guard378 != 0.0)) {
        let assign20330_e21859: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign20330_e21860: f64 = (assign20330_e21859).sqrt();
        (assign20330_e21860, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20330_e21862;
        var_tmp_dn6 = assign20330_e21862_d_n6;
        var_tmp_dn7 = assign20330_e21862_d_n7;
        var_tmp_dn8 = assign20330_e21862_d_n8;
        var_tmp_dn9 = assign20330_e21862_d_n9;

        let (assign20340_e21881, assign20340_e21881_d_n6, assign20340_e21881_d_n7, assign20340_e21881_d_n8, assign20340_e21881_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard376 == 0.0)) && (var_guard378 == 0.0)) {
        let assign20340_e21877: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign20340_e21879: f64 = (assign20340_e21877).powf(p.p849);
        (assign20340_e21879, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20340_e21881;
        var_tmp_dn6 = assign20340_e21881_d_n6;
        var_tmp_dn7 = assign20340_e21881_d_n7;
        var_tmp_dn8 = assign20340_e21881_d_n8;
        var_tmp_dn9 = assign20340_e21881_d_n9;

        let (assign20350_e21895, assign20350_e21895_d_n6, assign20350_e21895_d_n7, assign20350_e21895_d_n8, assign20350_e21895_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard376 == 0.0)) {
        let assign20350_e21893: f64 = (var_wdepnulrsti * var_tmp);
        (assign20350_e21893, (var_wdepnulrsti * var_tmp_dn6), (var_wdepnulrsti * var_tmp_dn7), (var_wdepnulrsti * var_tmp_dn8), (var_wdepnulrsti * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign20350_e21895;
        var_wdep_dn6 = assign20350_e21895_d_n6;
        var_wdep_dn7 = assign20350_e21895_d_n7;
        var_wdep_dn8 = assign20350_e21895_d_n8;
        var_wdep_dn9 = assign20350_e21895_d_n9;

        let (assign20360_e21913, assign20360_e21913_d_n6, assign20360_e21913_d_n7, assign20360_e21913_d_n8, assign20360_e21913_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard376 == 0.0)) {
        let assign20360_e21908: f64 = (var_zinv - 1.0);
        let assign20360_e21910: f64 = (assign20360_e21908 * var_wdep);
        let assign20360_e21911: f64 = (var_ftdsti * assign20360_e21910);
        (assign20360_e21911, (var_ftdsti * (assign20360_e21908 * var_wdep_dn6)), (var_ftdsti * (assign20360_e21908 * var_wdep_dn7)), (var_ftdsti * (assign20360_e21908 * var_wdep_dn8)), (var_ftdsti * (assign20360_e21908 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign20360_e21913;
        var_asrh_dn6 = assign20360_e21913_d_n6;
        var_asrh_dn7 = assign20360_e21913_d_n7;
        var_asrh_dn8 = assign20360_e21913_d_n8;
        var_asrh_dn9 = assign20360_e21913_d_n9;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_asrh_dn9_slot = var_asrh_dn9;
        *var_dwsrh_slot = var_dwsrh;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_erfctimesexpmtat_dn9_slot = var_erfctimesexpmtat_dn9;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_fmaxr_dn9_slot = var_fmaxr_dn9;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_gammamax_dn9_slot = var_gammamax_dn9;
        *var_guard368_slot = var_guard368;
        *var_guard369_slot = var_guard369;
        *var_guard370_slot = var_guard370;
        *var_guard371_slot = var_guard371;
        *var_guard372_slot = var_guard372;
        *var_guard373_slot = var_guard373;
        *var_guard374_slot = var_guard374;
        *var_guard375_slot = var_guard375;
        *var_guard376_slot = var_guard376;
        *var_guard377_slot = var_guard377;
        *var_guard378_slot = var_guard378;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_id__blk212_slot = var_id__blk212;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijunbot_dn9_slot = var_ijunbot_dn9;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_ijunsti_dn9_slot = var_ijunsti_dn9;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wdep_dn9_slot = var_wdep_dn9;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
    }

    pub(super) fn stamp_transient_block_36(
        p: &Parameters,
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatsti: f64,
        var_berfc: f64,
        var_btatpartsti: f64,
        var_cerfc: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard375: f64,
        var_guard376: f64,
        var_one_minus_psti: f64,
        var_one_over_one_minus_psti: f64,
        var_perfc: f64,
        var_vbi_minus_vjsrh: f64,
        var_wdep: f64,
        var_wdep_dn6: f64,
        var_wdep_dn7: f64,
        var_wdep_dn8: f64,
        var_wdep_dn9: f64,
        var_wsrh: f64,
        var_btat_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_btat_dn9_slot: &mut f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfcpos_dn9_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_erfctimesexpmtat_dn9_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_gammamax_dn9_slot: &mut f64,
        var_guard379_slot: &mut f64,
        var_guard380_slot: &mut f64,
        var_guard381_slot: &mut f64,
        var_guard382_slot: &mut f64,
        var_guard383_slot: &mut f64,
        var_guard384_slot: &mut f64,
        var_guard385_slot: &mut f64,
        var_guard386_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ktat_dn9_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_ltat_dn9_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_mtat_dn9_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_sqrtumax_dn9_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_terfc_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_dn9_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umax_dn9_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxbeforelimiting_dn9_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_dn9_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wgamma_dn9_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_wtat_dn9_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_xerfc_dn9_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
        var_ysq_dn9_slot: &mut f64,
    ) {
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_btat_dn9: f64 = *var_btat_dn9_slot;
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfcpos_dn9: f64 = *var_erfcpos_dn9_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_erfctimesexpmtat_dn9: f64 = *var_erfctimesexpmtat_dn9_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_gammamax_dn9: f64 = *var_gammamax_dn9_slot;
        let mut var_guard379: f64 = *var_guard379_slot;
        let mut var_guard380: f64 = *var_guard380_slot;
        let mut var_guard381: f64 = *var_guard381_slot;
        let mut var_guard382: f64 = *var_guard382_slot;
        let mut var_guard383: f64 = *var_guard383_slot;
        let mut var_guard384: f64 = *var_guard384_slot;
        let mut var_guard385: f64 = *var_guard385_slot;
        let mut var_guard386: f64 = *var_guard386_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ktat_dn9: f64 = *var_ktat_dn9_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_ltat_dn9: f64 = *var_ltat_dn9_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_mtat_dn9: f64 = *var_mtat_dn9_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_sqrtumax_dn9: f64 = *var_sqrtumax_dn9_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_terfc_dn9: f64 = *var_terfc_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_twoatatoverthreebtat_dn9: f64 = *var_twoatatoverthreebtat_dn9_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umax_dn9: f64 = *var_umax_dn9_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxbeforelimiting_dn9: f64 = *var_umaxbeforelimiting_dn9_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_umaxpoweronepointfive_dn9: f64 = *var_umaxpoweronepointfive_dn9_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wgamma_dn9: f64 = *var_wgamma_dn9_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_wtat_dn9: f64 = *var_wtat_dn9_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_xerfc_dn9: f64 = *var_xerfc_dn9_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;
        let mut var_ysq_dn9: f64 = *var_ysq_dn9_slot;

        let (assign20370_e21929, assign20370_e21929_d_n6, assign20370_e21929_d_n7, assign20370_e21929_d_n8, assign20370_e21929_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard376 == 0.0)) {
        let assign20370_e21926: f64 = (var_asrh * var_wsrh);
        let assign20370_e21927: f64 = (p.p858 * assign20370_e21926);
        (assign20370_e21927, (p.p858 * (var_asrh_dn6 * var_wsrh)), (p.p858 * (var_asrh_dn7 * var_wsrh)), (p.p858 * (var_asrh_dn8 * var_wsrh)), (p.p858 * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign20370_e21929;
        var_isrh_dn6 = assign20370_e21929_d_n6;
        var_isrh_dn7 = assign20370_e21929_d_n7;
        var_isrh_dn8 = assign20370_e21929_d_n8;
        var_isrh_dn9 = assign20370_e21929_d_n9;

        let assign20380_e21932: f64 = if p.p863 == 0.0 { 1.0 } else { 0.0 };
        var_guard379 = assign20380_e21932;

        let (assign20390_e21943, assign20390_e21943_d_n6, assign20390_e21943_d_n7, assign20390_e21943_d_n8, assign20390_e21943_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign20390_e21943;
        var_itat_dn6 = assign20390_e21943_d_n6;
        var_itat_dn7 = assign20390_e21943_d_n7;
        var_itat_dn8 = assign20390_e21943_d_n8;
        var_itat_dn9 = assign20390_e21943_d_n9;

        let (assign20400_e21961, assign20400_e21961_d_n6, assign20400_e21961_d_n7, assign20400_e21961_d_n8, assign20400_e21961_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) {
        let assign20400_e21956: f64 = (var_wdep * var_one_minus_psti);
        let assign20400_e21958: f64 = (assign20400_e21956 / var_vbi_minus_vjsrh);
        let assign20400_e21959: f64 = (var_btatpartsti * assign20400_e21958);
        (assign20400_e21959, (var_btatpartsti * ((var_wdep_dn6 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn7 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn8 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn9 * var_one_minus_psti) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign20400_e21961;
        var_btat_dn6 = assign20400_e21961_d_n6;
        var_btat_dn7 = assign20400_e21961_d_n7;
        var_btat_dn8 = assign20400_e21961_d_n8;
        var_btat_dn9 = assign20400_e21961_d_n9;

        let (assign20410_e21977, assign20410_e21977_d_n6, assign20410_e21977_d_n7, assign20410_e21977_d_n8, assign20410_e21977_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) {
        let assign20410_e21973: f64 = (0.666666666666667 * var_atatsti);
        let assign20410_e21975: f64 = (assign20410_e21973 / var_btat);
        (assign20410_e21975, (-((assign20410_e21973 * var_btat_dn6) / (var_btat * var_btat))), (-((assign20410_e21973 * var_btat_dn7) / (var_btat * var_btat))), (-((assign20410_e21973 * var_btat_dn8) / (var_btat * var_btat))), (-((assign20410_e21973 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign20410_e21977;
        var_twoatatoverthreebtat_dn6 = assign20410_e21977_d_n6;
        var_twoatatoverthreebtat_dn7 = assign20410_e21977_d_n7;
        var_twoatatoverthreebtat_dn8 = assign20410_e21977_d_n8;
        var_twoatatoverthreebtat_dn9 = assign20410_e21977_d_n9;

        let (assign20420_e21991, assign20420_e21991_d_n6, assign20420_e21991_d_n7, assign20420_e21991_d_n8, assign20420_e21991_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) {
        let assign20420_e21989: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign20420_e21989, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign20420_e21991;
        var_umaxbeforelimiting_dn6 = assign20420_e21991_d_n6;
        var_umaxbeforelimiting_dn7 = assign20420_e21991_d_n7;
        var_umaxbeforelimiting_dn8 = assign20420_e21991_d_n8;
        var_umaxbeforelimiting_dn9 = assign20420_e21991_d_n9;

        let (assign20430_e22012, assign20430_e22012_d_n6, assign20430_e22012_d_n7, assign20430_e22012_d_n8, assign20430_e22012_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) {
        let assign20430_e22003: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign20430_e22006: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign20430_e22008: f64 = (assign20430_e22006 + 1.0);
        let assign20430_e22009: f64 = (assign20430_e22003 / assign20430_e22008);
        let assign20430_e22010: f64 = (assign20430_e22009).sqrt();
        (assign20430_e22010, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign20430_e22008) - (assign20430_e22003 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign20430_e22008 * assign20430_e22008)) / (2.0 * assign20430_e22010)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign20430_e22008) - (assign20430_e22003 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign20430_e22008 * assign20430_e22008)) / (2.0 * assign20430_e22010)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign20430_e22008) - (assign20430_e22003 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign20430_e22008 * assign20430_e22008)) / (2.0 * assign20430_e22010)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign20430_e22008) - (assign20430_e22003 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign20430_e22008 * assign20430_e22008)) / (2.0 * assign20430_e22010)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign20430_e22012;
        var_umax_dn6 = assign20430_e22012_d_n6;
        var_umax_dn7 = assign20430_e22012_d_n7;
        var_umax_dn8 = assign20430_e22012_d_n8;
        var_umax_dn9 = assign20430_e22012_d_n9;

        let (assign20440_e22025, assign20440_e22025_d_n6, assign20440_e22025_d_n7, assign20440_e22025_d_n8, assign20440_e22025_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) {
        let assign20440_e22023: f64 = (var_umax).sqrt();
        (assign20440_e22023, (var_umax_dn6 / (2.0 * assign20440_e22023)), (var_umax_dn7 / (2.0 * assign20440_e22023)), (var_umax_dn8 / (2.0 * assign20440_e22023)), (var_umax_dn9 / (2.0 * assign20440_e22023)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign20440_e22025;
        var_sqrtumax_dn6 = assign20440_e22025_d_n6;
        var_sqrtumax_dn7 = assign20440_e22025_d_n7;
        var_sqrtumax_dn8 = assign20440_e22025_d_n8;
        var_sqrtumax_dn9 = assign20440_e22025_d_n9;

        let (assign20450_e22039, assign20450_e22039_d_n6, assign20450_e22039_d_n7, assign20450_e22039_d_n8, assign20450_e22039_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) {
        let assign20450_e22037: f64 = (var_umax * var_sqrtumax);
        (assign20450_e22037, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign20450_e22039;
        var_umaxpoweronepointfive_dn6 = assign20450_e22039_d_n6;
        var_umaxpoweronepointfive_dn7 = assign20450_e22039_d_n7;
        var_umaxpoweronepointfive_dn8 = assign20450_e22039_d_n8;
        var_umaxpoweronepointfive_dn9 = assign20450_e22039_d_n9;

        let assign20460_e22041: f64 = (-p.p849);
        let assign20460_e22043: f64 = (assign20460_e22041 * var_one_over_one_minus_psti);
        let assign20460_e22045: f64 = (-1.0);
        let assign20460_e22046: f64 = if assign20460_e22043 == assign20460_e22045 { 1.0 } else { 0.0 };
        var_guard380 = assign20460_e22046;

        let (assign20470_e22066, assign20470_e22066_d_n6, assign20470_e22066_d_n7, assign20470_e22066_d_n8, assign20470_e22066_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) && (var_guard380 != 0.0)) {
        let assign20470_e22062: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign20470_e22063: f64 = (1.0 + assign20470_e22062);
        let assign20470_e22064: f64 = (1.0 / assign20470_e22063);
        (assign20470_e22064, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign20470_e22063 * assign20470_e22063))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign20470_e22063 * assign20470_e22063))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign20470_e22063 * assign20470_e22063))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign20470_e22063 * assign20470_e22063))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign20470_e22066;
        var_wgamma_dn6 = assign20470_e22066_d_n6;
        var_wgamma_dn7 = assign20470_e22066_d_n7;
        var_wgamma_dn8 = assign20470_e22066_d_n8;
        var_wgamma_dn9 = assign20470_e22066_d_n9;

        let (assign20480_e22090, assign20480_e22090_d_n6, assign20480_e22090_d_n7, assign20480_e22090_d_n8, assign20480_e22090_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) && (var_guard380 == 0.0)) {
        let assign20480_e22082: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign20480_e22083: f64 = (1.0 + assign20480_e22082);
        let assign20480_e22085: f64 = (-p.p849);
        let assign20480_e22087: f64 = (assign20480_e22085 * var_one_over_one_minus_psti);
        let assign20480_e22088: f64 = (assign20480_e22083).powf(assign20480_e22087);
        (assign20480_e22088, if 0.0 == 0.0 && ((assign20480_e22087) as f64).is_finite() && ((assign20480_e22087) as f64).fract() == 0.0 { if assign20480_e22087 == 0.0 { 0.0 } else { (assign20480_e22087 * ((assign20480_e22083).powf(assign20480_e22087 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign20480_e22088 * (assign20480_e22087 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign20480_e22083))) }, if 0.0 == 0.0 && ((assign20480_e22087) as f64).is_finite() && ((assign20480_e22087) as f64).fract() == 0.0 { if assign20480_e22087 == 0.0 { 0.0 } else { (assign20480_e22087 * ((assign20480_e22083).powf(assign20480_e22087 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign20480_e22088 * (assign20480_e22087 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign20480_e22083))) }, if 0.0 == 0.0 && ((assign20480_e22087) as f64).is_finite() && ((assign20480_e22087) as f64).fract() == 0.0 { if assign20480_e22087 == 0.0 { 0.0 } else { (assign20480_e22087 * ((assign20480_e22083).powf(assign20480_e22087 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign20480_e22088 * (assign20480_e22087 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign20480_e22083))) }, if 0.0 == 0.0 && ((assign20480_e22087) as f64).is_finite() && ((assign20480_e22087) as f64).fract() == 0.0 { if assign20480_e22087 == 0.0 { 0.0 } else { (assign20480_e22087 * ((assign20480_e22083).powf(assign20480_e22087 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign20480_e22088 * (assign20480_e22087 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign20480_e22083))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign20480_e22090;
        var_wgamma_dn6 = assign20480_e22090_d_n6;
        var_wgamma_dn7 = assign20480_e22090_d_n7;
        var_wgamma_dn8 = assign20480_e22090_d_n8;
        var_wgamma_dn9 = assign20480_e22090_d_n9;

        let (assign20490_e22108, assign20490_e22108_d_n6, assign20490_e22108_d_n7, assign20490_e22108_d_n8, assign20490_e22108_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) {
        let assign20490_e22102: f64 = (var_wsrh * var_wgamma);
        let assign20490_e22105: f64 = (var_wsrh + var_wgamma);
        let assign20490_e22106: f64 = (assign20490_e22102 / assign20490_e22105);
        (assign20490_e22106, ((((var_wsrh * var_wgamma_dn6) * assign20490_e22105) - (assign20490_e22102 * var_wgamma_dn6)) / (assign20490_e22105 * assign20490_e22105)), ((((var_wsrh * var_wgamma_dn7) * assign20490_e22105) - (assign20490_e22102 * var_wgamma_dn7)) / (assign20490_e22105 * assign20490_e22105)), ((((var_wsrh * var_wgamma_dn8) * assign20490_e22105) - (assign20490_e22102 * var_wgamma_dn8)) / (assign20490_e22105 * assign20490_e22105)), ((((var_wsrh * var_wgamma_dn9) * assign20490_e22105) - (assign20490_e22102 * var_wgamma_dn9)) / (assign20490_e22105 * assign20490_e22105)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign20490_e22108;
        var_wtat_dn6 = assign20490_e22108_d_n6;
        var_wtat_dn7 = assign20490_e22108_d_n7;
        var_wtat_dn8 = assign20490_e22108_d_n8;
        var_wtat_dn9 = assign20490_e22108_d_n9;

        let (assign20500_e22125, assign20500_e22125_d_n6, assign20500_e22125_d_n7, assign20500_e22125_d_n8, assign20500_e22125_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) {
        let assign20500_e22121: f64 = (var_btat / var_sqrtumax);
        let assign20500_e22122: f64 = (0.375 * assign20500_e22121);
        let assign20500_e22123: f64 = (assign20500_e22122).sqrt();
        (assign20500_e22123, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign20500_e22123)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign20500_e22123)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign20500_e22123)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign20500_e22123)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign20500_e22125;
        var_ktat_dn6 = assign20500_e22125_d_n6;
        var_ktat_dn7 = assign20500_e22125_d_n7;
        var_ktat_dn8 = assign20500_e22125_d_n8;
        var_ktat_dn9 = assign20500_e22125_d_n9;

        let (assign20510_e22143, assign20510_e22143_d_n6, assign20510_e22143_d_n7, assign20510_e22143_d_n8, assign20510_e22143_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) {
        let assign20510_e22138: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign20510_e22139: f64 = (2.0 * assign20510_e22138);
        let assign20510_e22141: f64 = (assign20510_e22139 - var_umax);
        (assign20510_e22141, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign20510_e22143;
        var_ltat_dn6 = assign20510_e22143_d_n6;
        var_ltat_dn7 = assign20510_e22143_d_n7;
        var_ltat_dn8 = assign20510_e22143_d_n8;
        var_ltat_dn9 = assign20510_e22143_d_n9;

        let (assign20520_e22169, assign20520_e22169_d_n6, assign20520_e22169_d_n7, assign20520_e22169_d_n8, assign20520_e22169_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) {
        let assign20520_e22155: f64 = (var_atatsti * var_twoatatoverthreebtat);
        let assign20520_e22157: f64 = (assign20520_e22155 * var_sqrtumax);
        let assign20520_e22160: f64 = (var_atatsti * var_umax);
        let assign20520_e22161: f64 = (assign20520_e22157 - assign20520_e22160);
        let assign20520_e22165: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign20520_e22166: f64 = (0.5 * assign20520_e22165);
        let assign20520_e22167: f64 = (assign20520_e22161 + assign20520_e22166);
        (assign20520_e22167, (((((var_atatsti * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign20520_e22155 * var_sqrtumax_dn6)) - (var_atatsti * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign20520_e22155 * var_sqrtumax_dn7)) - (var_atatsti * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign20520_e22155 * var_sqrtumax_dn8)) - (var_atatsti * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatsti * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign20520_e22155 * var_sqrtumax_dn9)) - (var_atatsti * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign20520_e22169;
        var_mtat_dn6 = assign20520_e22169_d_n6;
        var_mtat_dn7 = assign20520_e22169_d_n7;
        var_mtat_dn8 = assign20520_e22169_d_n8;
        var_mtat_dn9 = assign20520_e22169_d_n9;

        let (assign20530_e22185, assign20530_e22185_d_n6, assign20530_e22185_d_n7, assign20530_e22185_d_n8, assign20530_e22185_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) {
        let assign20530_e22181: f64 = (var_ltat - 1.0);
        let assign20530_e22183: f64 = (assign20530_e22181 * var_ktat);
        (assign20530_e22183, ((var_ltat_dn6 * var_ktat) + (assign20530_e22181 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign20530_e22181 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign20530_e22181 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign20530_e22181 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign20530_e22185;
        var_xerfc_dn6 = assign20530_e22185_d_n6;
        var_xerfc_dn7 = assign20530_e22185_d_n7;
        var_xerfc_dn8 = assign20530_e22185_d_n8;
        var_xerfc_dn9 = assign20530_e22185_d_n9;

        let (assign20540_e22199, assign20540_e22199_d_n6, assign20540_e22199_d_n7, assign20540_e22199_d_n8, assign20540_e22199_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) {
        let assign20540_e22197: f64 = (var_xerfc * var_xerfc);
        (assign20540_e22197, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign20540_e22199;
        var_ysq_dn6 = assign20540_e22199_d_n6;
        var_ysq_dn7 = assign20540_e22199_d_n7;
        var_ysq_dn8 = assign20540_e22199_d_n8;
        var_ysq_dn9 = assign20540_e22199_d_n9;

        let assign20550_e22202: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard381 = assign20550_e22202;

        let (assign20560_e22222, assign20560_e22222_d_n6, assign20560_e22222_d_n7, assign20560_e22222_d_n8, assign20560_e22222_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) && (var_guard381 != 0.0)) {
        let assign20560_e22218: f64 = (var_perfc * var_xerfc);
        let assign20560_e22219: f64 = (1.0 + assign20560_e22218);
        let assign20560_e22220: f64 = (1.0 / assign20560_e22219);
        (assign20560_e22220, (-((var_perfc * var_xerfc_dn6) / (assign20560_e22219 * assign20560_e22219))), (-((var_perfc * var_xerfc_dn7) / (assign20560_e22219 * assign20560_e22219))), (-((var_perfc * var_xerfc_dn8) / (assign20560_e22219 * assign20560_e22219))), (-((var_perfc * var_xerfc_dn9) / (assign20560_e22219 * assign20560_e22219))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign20560_e22222;
        var_terfc_dn6 = assign20560_e22222_d_n6;
        var_terfc_dn7 = assign20560_e22222_d_n7;
        var_terfc_dn8 = assign20560_e22222_d_n8;
        var_terfc_dn9 = assign20560_e22222_d_n9;

        let (assign20570_e22243, assign20570_e22243_d_n6, assign20570_e22243_d_n7, assign20570_e22243_d_n8, assign20570_e22243_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) && (var_guard381 == 0.0)) {
        let assign20570_e22239: f64 = (var_perfc * var_xerfc);
        let assign20570_e22240: f64 = (1.0 - assign20570_e22239);
        let assign20570_e22241: f64 = (1.0 / assign20570_e22240);
        (assign20570_e22241, (-((-(var_perfc * var_xerfc_dn6)) / (assign20570_e22240 * assign20570_e22240))), (-((-(var_perfc * var_xerfc_dn7)) / (assign20570_e22240 * assign20570_e22240))), (-((-(var_perfc * var_xerfc_dn8)) / (assign20570_e22240 * assign20570_e22240))), (-((-(var_perfc * var_xerfc_dn9)) / (assign20570_e22240 * assign20570_e22240))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign20570_e22243;
        var_terfc_dn6 = assign20570_e22243_d_n6;
        var_terfc_dn7 = assign20570_e22243_d_n7;
        var_terfc_dn8 = assign20570_e22243_d_n8;
        var_terfc_dn9 = assign20570_e22243_d_n9;

        let assign20580_e22245: f64 = (-var_ysq);
        let assign20580_e22247: f64 = (assign20580_e22245 + var_mtat);
        let assign20580_e22249: f64 = (-230.25850929940458);
        let assign20580_e22250: f64 = if assign20580_e22247 > assign20580_e22249 { 1.0 } else { 0.0 };
        var_guard382 = assign20580_e22250;

        let (assign20590_e22268, assign20590_e22268_d_n6, assign20590_e22268_d_n7, assign20590_e22268_d_n8, assign20590_e22268_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) && (var_guard382 != 0.0)) {
        let assign20590_e22263: f64 = (-var_ysq);
        let assign20590_e22265: f64 = (assign20590_e22263 + var_mtat);
        let assign20590_e22266: f64 = (assign20590_e22265).exp();
        (assign20590_e22266, (assign20590_e22266 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign20590_e22266 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign20590_e22266 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign20590_e22266 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20590_e22268;
        var_tmp_dn6 = assign20590_e22268_d_n6;
        var_tmp_dn7 = assign20590_e22268_d_n7;
        var_tmp_dn8 = assign20590_e22268_d_n8;
        var_tmp_dn9 = assign20590_e22268_d_n9;

        let (assign20600_e22317, assign20600_e22317_d_n6, assign20600_e22317_d_n7, assign20600_e22317_d_n8, assign20600_e22317_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) && (var_guard382 == 0.0)) {
        let assign20600_e22284: f64 = (-230.25850929940458);
        let assign20600_e22286: f64 = (-var_ysq);
        let assign20600_e22288: f64 = (assign20600_e22286 + var_mtat);
        let assign20600_e22289: f64 = (assign20600_e22284 - assign20600_e22288);
        let assign20600_e22293: f64 = (-230.25850929940458);
        let assign20600_e22295: f64 = (-var_ysq);
        let assign20600_e22297: f64 = (assign20600_e22295 + var_mtat);
        let assign20600_e22298: f64 = (assign20600_e22293 - assign20600_e22297);
        let assign20600_e22301: f64 = (-230.25850929940458);
        let assign20600_e22303: f64 = (-var_ysq);
        let assign20600_e22305: f64 = (assign20600_e22303 + var_mtat);
        let assign20600_e22306: f64 = (assign20600_e22301 - assign20600_e22305);
        let assign20600_e22308: f64 = (assign20600_e22306 * 0.3333333333333333);
        let assign20600_e22309: f64 = (1.0 + assign20600_e22308);
        let assign20600_e22310: f64 = (assign20600_e22298 * assign20600_e22309);
        let assign20600_e22311: f64 = (0.5 * assign20600_e22310);
        let assign20600_e22312: f64 = (1.0 + assign20600_e22311);
        let assign20600_e22313: f64 = (assign20600_e22289 * assign20600_e22312);
        let assign20600_e22314: f64 = (1.0 + assign20600_e22313);
        let assign20600_e22315: f64 = (1e-100 / assign20600_e22314);
        (assign20600_e22315, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign20600_e22312) + (assign20600_e22289 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign20600_e22309) + (assign20600_e22298 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign20600_e22314 * assign20600_e22314))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign20600_e22312) + (assign20600_e22289 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign20600_e22309) + (assign20600_e22298 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign20600_e22314 * assign20600_e22314))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign20600_e22312) + (assign20600_e22289 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign20600_e22309) + (assign20600_e22298 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign20600_e22314 * assign20600_e22314))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign20600_e22312) + (assign20600_e22289 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign20600_e22309) + (assign20600_e22298 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign20600_e22314 * assign20600_e22314))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20600_e22317;
        var_tmp_dn6 = assign20600_e22317_d_n6;
        var_tmp_dn7 = assign20600_e22317_d_n7;
        var_tmp_dn8 = assign20600_e22317_d_n8;
        var_tmp_dn9 = assign20600_e22317_d_n9;

        let (assign20610_e22347, assign20610_e22347_d_n6, assign20610_e22347_d_n7, assign20610_e22347_d_n8, assign20610_e22347_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) {
        let assign20610_e22329: f64 = (0.29214664 * var_terfc);
        let assign20610_e22333: f64 = (var_terfc * var_terfc);
        let assign20610_e22334: f64 = (var_berfc * assign20610_e22333);
        let assign20610_e22335: f64 = (assign20610_e22329 + assign20610_e22334);
        let assign20610_e22339: f64 = (var_terfc * var_terfc);
        let assign20610_e22341: f64 = (assign20610_e22339 * var_terfc);
        let assign20610_e22342: f64 = (var_cerfc * assign20610_e22341);
        let assign20610_e22343: f64 = (assign20610_e22335 + assign20610_e22342);
        let assign20610_e22345: f64 = (assign20610_e22343 * var_tmp);
        (assign20610_e22345, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign20610_e22339 * var_terfc_dn6)))) * var_tmp) + (assign20610_e22343 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign20610_e22339 * var_terfc_dn7)))) * var_tmp) + (assign20610_e22343 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign20610_e22339 * var_terfc_dn8)))) * var_tmp) + (assign20610_e22343 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign20610_e22339 * var_terfc_dn9)))) * var_tmp) + (assign20610_e22343 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign20610_e22347;
        var_erfcpos_dn6 = assign20610_e22347_d_n6;
        var_erfcpos_dn7 = assign20610_e22347_d_n7;
        var_erfcpos_dn8 = assign20610_e22347_d_n8;
        var_erfcpos_dn9 = assign20610_e22347_d_n9;

        let assign20620_e22350: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard383 = assign20620_e22350;

        let (assign20630_e22364, assign20630_e22364_d_n6, assign20630_e22364_d_n7, assign20630_e22364_d_n8, assign20630_e22364_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) && (var_guard383 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign20630_e22364;
        var_erfctimesexpmtat_dn6 = assign20630_e22364_d_n6;
        var_erfctimesexpmtat_dn7 = assign20630_e22364_d_n7;
        var_erfctimesexpmtat_dn8 = assign20630_e22364_d_n8;
        var_erfctimesexpmtat_dn9 = assign20630_e22364_d_n9;

        let assign20640_e22367: f64 = (-230.25850929940458);
        let assign20640_e22368: f64 = if var_mtat > assign20640_e22367 { 1.0 } else { 0.0 };
        var_guard384 = assign20640_e22368;

        let (assign20650_e22386, assign20650_e22386_d_n6, assign20650_e22386_d_n7, assign20650_e22386_d_n8, assign20650_e22386_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) && (var_guard383 == 0.0)) && (var_guard384 != 0.0)) {
        let assign20650_e22384: f64 = (var_mtat).exp();
        (assign20650_e22384, (assign20650_e22384 * var_mtat_dn6), (assign20650_e22384 * var_mtat_dn7), (assign20650_e22384 * var_mtat_dn8), (assign20650_e22384 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20650_e22386;
        var_tmp_dn6 = assign20650_e22386_d_n6;
        var_tmp_dn7 = assign20650_e22386_d_n7;
        var_tmp_dn8 = assign20650_e22386_d_n8;
        var_tmp_dn9 = assign20650_e22386_d_n9;

        let (assign20660_e22429, assign20660_e22429_d_n6, assign20660_e22429_d_n7, assign20660_e22429_d_n8, assign20660_e22429_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) && (var_guard383 == 0.0)) && (var_guard384 == 0.0)) {
        let assign20660_e22405: f64 = (-230.25850929940458);
        let assign20660_e22407: f64 = (assign20660_e22405 - var_mtat);
        let assign20660_e22411: f64 = (-230.25850929940458);
        let assign20660_e22413: f64 = (assign20660_e22411 - var_mtat);
        let assign20660_e22416: f64 = (-230.25850929940458);
        let assign20660_e22418: f64 = (assign20660_e22416 - var_mtat);
        let assign20660_e22420: f64 = (assign20660_e22418 * 0.3333333333333333);
        let assign20660_e22421: f64 = (1.0 + assign20660_e22420);
        let assign20660_e22422: f64 = (assign20660_e22413 * assign20660_e22421);
        let assign20660_e22423: f64 = (0.5 * assign20660_e22422);
        let assign20660_e22424: f64 = (1.0 + assign20660_e22423);
        let assign20660_e22425: f64 = (assign20660_e22407 * assign20660_e22424);
        let assign20660_e22426: f64 = (1.0 + assign20660_e22425);
        let assign20660_e22427: f64 = (1e-100 / assign20660_e22426);
        (assign20660_e22427, (-((1e-100 * (((-var_mtat_dn6) * assign20660_e22424) + (assign20660_e22407 * (0.5 * (((-var_mtat_dn6) * assign20660_e22421) + (assign20660_e22413 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign20660_e22426 * assign20660_e22426))), (-((1e-100 * (((-var_mtat_dn7) * assign20660_e22424) + (assign20660_e22407 * (0.5 * (((-var_mtat_dn7) * assign20660_e22421) + (assign20660_e22413 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign20660_e22426 * assign20660_e22426))), (-((1e-100 * (((-var_mtat_dn8) * assign20660_e22424) + (assign20660_e22407 * (0.5 * (((-var_mtat_dn8) * assign20660_e22421) + (assign20660_e22413 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign20660_e22426 * assign20660_e22426))), (-((1e-100 * (((-var_mtat_dn9) * assign20660_e22424) + (assign20660_e22407 * (0.5 * (((-var_mtat_dn9) * assign20660_e22421) + (assign20660_e22413 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign20660_e22426 * assign20660_e22426))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20660_e22429;
        var_tmp_dn6 = assign20660_e22429_d_n6;
        var_tmp_dn7 = assign20660_e22429_d_n7;
        var_tmp_dn8 = assign20660_e22429_d_n8;
        var_tmp_dn9 = assign20660_e22429_d_n9;

        let (assign20670_e22448, assign20670_e22448_d_n6, assign20670_e22448_d_n7, assign20670_e22448_d_n8, assign20670_e22448_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) && (var_guard383 == 0.0)) {
        let assign20670_e22444: f64 = (2.0 * var_tmp);
        let assign20670_e22446: f64 = (assign20670_e22444 - var_erfcpos);
        (assign20670_e22446, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign20670_e22448;
        var_erfctimesexpmtat_dn6 = assign20670_e22448_d_n6;
        var_erfctimesexpmtat_dn7 = assign20670_e22448_d_n7;
        var_erfctimesexpmtat_dn8 = assign20670_e22448_d_n8;
        var_erfctimesexpmtat_dn9 = assign20670_e22448_d_n9;

        let (assign20680_e22468, assign20680_e22468_d_n6, assign20680_e22468_d_n7, assign20680_e22468_d_n8, assign20680_e22468_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) {
        let assign20680_e22460: f64 = (1.772453850905516 * 0.5);
        let assign20680_e22463: f64 = (var_atatsti * var_erfctimesexpmtat);
        let assign20680_e22465: f64 = (assign20680_e22463 / var_ktat);
        let assign20680_e22466: f64 = (assign20680_e22460 * assign20680_e22465);
        (assign20680_e22466, (assign20680_e22460 * ((((var_atatsti * var_erfctimesexpmtat_dn6) * var_ktat) - (assign20680_e22463 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign20680_e22460 * ((((var_atatsti * var_erfctimesexpmtat_dn7) * var_ktat) - (assign20680_e22463 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign20680_e22460 * ((((var_atatsti * var_erfctimesexpmtat_dn8) * var_ktat) - (assign20680_e22463 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign20680_e22460 * ((((var_atatsti * var_erfctimesexpmtat_dn9) * var_ktat) - (assign20680_e22463 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign20680_e22468;
        var_gammamax_dn6 = assign20680_e22468_d_n6;
        var_gammamax_dn7 = assign20680_e22468_d_n7;
        var_gammamax_dn8 = assign20680_e22468_d_n8;
        var_gammamax_dn9 = assign20680_e22468_d_n9;

        let (assign20690_e22486, assign20690_e22486_d_n6, assign20690_e22486_d_n7, assign20690_e22486_d_n8, assign20690_e22486_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard379 == 0.0)) {
        let assign20690_e22481: f64 = (var_asrh * var_gammamax);
        let assign20690_e22483: f64 = (assign20690_e22481 * var_wtat);
        let assign20690_e22484: f64 = (p.p863 * assign20690_e22483);
        (assign20690_e22484, (p.p863 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign20690_e22481 * var_wtat_dn6))), (p.p863 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign20690_e22481 * var_wtat_dn7))), (p.p863 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign20690_e22481 * var_wtat_dn8))), (p.p863 * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign20690_e22481 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign20690_e22486;
        var_itat_dn6 = assign20690_e22486_d_n6;
        var_itat_dn7 = assign20690_e22486_d_n7;
        var_itat_dn8 = assign20690_e22486_d_n8;
        var_itat_dn9 = assign20690_e22486_d_n9;

        let assign20700_e22489: f64 = if p.p869 == 0.0 { 1.0 } else { 0.0 };
        var_guard385 = assign20700_e22489;

        let (assign20710_e22500, assign20710_e22500_d_n6, assign20710_e22500_d_n7, assign20710_e22500_d_n8, assign20710_e22500_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard385 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign20710_e22500;
        var_ibbt_dn6 = assign20710_e22500_d_n6;
        var_ibbt_dn7 = assign20710_e22500_d_n7;
        var_ibbt_dn8 = assign20710_e22500_d_n8;
        var_ibbt_dn9 = assign20710_e22500_d_n9;

        let assign20720_e22503: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };
        var_guard386 = assign20720_e22503;

        *var_btat_slot = var_btat;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_btat_dn9_slot = var_btat_dn9;
        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfcpos_dn9_slot = var_erfcpos_dn9;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_erfctimesexpmtat_dn9_slot = var_erfctimesexpmtat_dn9;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_gammamax_dn9_slot = var_gammamax_dn9;
        *var_guard379_slot = var_guard379;
        *var_guard380_slot = var_guard380;
        *var_guard381_slot = var_guard381;
        *var_guard382_slot = var_guard382;
        *var_guard383_slot = var_guard383;
        *var_guard384_slot = var_guard384;
        *var_guard385_slot = var_guard385;
        *var_guard386_slot = var_guard386;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ktat_dn9_slot = var_ktat_dn9;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_ltat_dn9_slot = var_ltat_dn9;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_mtat_dn9_slot = var_mtat_dn9;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_sqrtumax_dn9_slot = var_sqrtumax_dn9;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_terfc_dn9_slot = var_terfc_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_twoatatoverthreebtat_dn9_slot = var_twoatatoverthreebtat_dn9;
        *var_umax_slot = var_umax;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umax_dn9_slot = var_umax_dn9;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxbeforelimiting_dn9_slot = var_umaxbeforelimiting_dn9;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_umaxpoweronepointfive_dn9_slot = var_umaxpoweronepointfive_dn9;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wgamma_dn9_slot = var_wgamma_dn9;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_wtat_dn9_slot = var_wtat_dn9;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_xerfc_dn9_slot = var_xerfc_dn9;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
        *var_ysq_dn9_slot = var_ysq_dn9;
    }

    pub(super) fn stamp_transient_block_37(
        p: &Parameters,
        var_alphaav: f64,
        var_atatgat: f64,
        var_btatpartgat: f64,
        var_fbbtsti: f64,
        var_fstopsti: f64,
        var_ftdgat: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard375: f64,
        var_guard385: f64,
        var_guard386: f64,
        var_idmult: f64,
        var_idsatgat: f64,
        var_lgsource_i: f64,
        var_one_minus_pgat: f64,
        var_one_over_one_minus_psti: f64,
        var_slopesti: f64,
        var_two_psistar: f64,
        var_v3: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbigat: f64,
        var_vbirgatinv: f64,
        var_vbirstiinv: f64,
        var_vbrinvsti: f64,
        var_vjsrh: f64,
        var_wdepnulrgat: f64,
        var_wdepnulrinvsti: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_asrh_dn9_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_btat_dn9_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_fmaxr_dn9_slot: &mut f64,
        var_guard387_slot: &mut f64,
        var_guard388_slot: &mut f64,
        var_guard389_slot: &mut f64,
        var_guard390_slot: &mut f64,
        var_guard391_slot: &mut f64,
        var_guard392_slot: &mut f64,
        var_guard393_slot: &mut f64,
        var_guard394_slot: &mut f64,
        var_guard395_slot: &mut f64,
        var_guard396_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_id__blk212_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_ijungat_dn9_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_ijunsti_dn9_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_dn9_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxbeforelimiting_dn9_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wdep_dn9_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_asrh_dn9: f64 = *var_asrh_dn9_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_btat_dn9: f64 = *var_btat_dn9_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fbreakdown_dn9: f64 = *var_fbreakdown_dn9_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_fmaxr_dn9: f64 = *var_fmaxr_dn9_slot;
        let mut var_guard387: f64 = *var_guard387_slot;
        let mut var_guard388: f64 = *var_guard388_slot;
        let mut var_guard389: f64 = *var_guard389_slot;
        let mut var_guard390: f64 = *var_guard390_slot;
        let mut var_guard391: f64 = *var_guard391_slot;
        let mut var_guard392: f64 = *var_guard392_slot;
        let mut var_guard393: f64 = *var_guard393_slot;
        let mut var_guard394: f64 = *var_guard394_slot;
        let mut var_guard395: f64 = *var_guard395_slot;
        let mut var_guard396: f64 = *var_guard396_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_id__blk212: f64 = *var_id__blk212_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_ijungat_dn9: f64 = *var_ijungat_dn9_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_ijunsti_dn9: f64 = *var_ijunsti_dn9_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_twoatatoverthreebtat_dn9: f64 = *var_twoatatoverthreebtat_dn9_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxbeforelimiting_dn9: f64 = *var_umaxbeforelimiting_dn9_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wdep_dn9: f64 = *var_wdep_dn9_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;

        let (assign20730_e22522, assign20730_e22522_d_n6, assign20730_e22522_d_n7, assign20730_e22522_d_n8, assign20730_e22522_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard385 == 0.0)) && (var_guard386 != 0.0)) {
        let assign20730_e22517: f64 = (p.p846 - var_vbbt);
        let assign20730_e22519: f64 = (assign20730_e22517 * var_vbirstiinv);
        let assign20730_e22520: f64 = (assign20730_e22519).sqrt();
        (assign20730_e22520, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20730_e22522;
        var_tmp_dn6 = assign20730_e22522_d_n6;
        var_tmp_dn7 = assign20730_e22522_d_n7;
        var_tmp_dn8 = assign20730_e22522_d_n8;
        var_tmp_dn9 = assign20730_e22522_d_n9;

        let (assign20740_e22543, assign20740_e22543_d_n6, assign20740_e22543_d_n7, assign20740_e22543_d_n8, assign20740_e22543_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard385 == 0.0)) && (var_guard386 == 0.0)) {
        let assign20740_e22537: f64 = (p.p846 - var_vbbt);
        let assign20740_e22539: f64 = (assign20740_e22537 * var_vbirstiinv);
        let assign20740_e22541: f64 = (assign20740_e22539).powf(p.p849);
        (assign20740_e22541, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20740_e22543;
        var_tmp_dn6 = assign20740_e22543_d_n6;
        var_tmp_dn7 = assign20740_e22543_d_n7;
        var_tmp_dn8 = assign20740_e22543_d_n8;
        var_tmp_dn9 = assign20740_e22543_d_n9;

        let (assign20750_e22563, assign20750_e22563_d_n6, assign20750_e22563_d_n7, assign20750_e22563_d_n8, assign20750_e22563_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard385 == 0.0)) {
        let assign20750_e22556: f64 = (p.p846 - var_vbbt);
        let assign20750_e22558: f64 = (assign20750_e22556 * var_wdepnulrinvsti);
        let assign20750_e22560: f64 = (assign20750_e22558 / var_tmp);
        let assign20750_e22561: f64 = (var_one_over_one_minus_psti * assign20750_e22560);
        (assign20750_e22561, (var_one_over_one_minus_psti * (-((assign20750_e22558 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign20750_e22558 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign20750_e22558 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign20750_e22558 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign20750_e22563;
        var_fmaxr_dn6 = assign20750_e22563_d_n6;
        var_fmaxr_dn7 = assign20750_e22563_d_n7;
        var_fmaxr_dn8 = assign20750_e22563_d_n8;
        var_fmaxr_dn9 = assign20750_e22563_d_n9;

        let assign20760_e22565: f64 = (-var_fbbtsti);
        let assign20760_e22567: f64 = (assign20760_e22565 / var_fmaxr);
        let assign20760_e22568: f64 = (assign20760_e22567).abs();
        let assign20760_e22570: f64 = if assign20760_e22568 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard387 = assign20760_e22570;

        let (assign20770_e22588, assign20770_e22588_d_n6, assign20770_e22588_d_n7, assign20770_e22588_d_n8, assign20770_e22588_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard385 == 0.0)) && (var_guard387 != 0.0)) {
        let assign20770_e22583: f64 = (-var_fbbtsti);
        let assign20770_e22585: f64 = (assign20770_e22583 / var_fmaxr);
        let assign20770_e22586: f64 = (assign20770_e22585).exp();
        (assign20770_e22586, (assign20770_e22586 * (-((assign20770_e22583 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign20770_e22586 * (-((assign20770_e22583 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign20770_e22586 * (-((assign20770_e22583 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign20770_e22586 * (-((assign20770_e22583 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20770_e22588;
        var_tmp_dn6 = assign20770_e22588_d_n6;
        var_tmp_dn7 = assign20770_e22588_d_n7;
        var_tmp_dn8 = assign20770_e22588_d_n8;
        var_tmp_dn9 = assign20770_e22588_d_n9;

        let assign20780_e22590: f64 = (-var_fbbtsti);
        let assign20780_e22592: f64 = (assign20780_e22590 / var_fmaxr);
        let assign20780_e22594: f64 = if assign20780_e22592 < 0.0 { 1.0 } else { 0.0 };
        var_guard388 = assign20780_e22594;

        let (assign20790_e22645, assign20790_e22645_d_n6, assign20790_e22645_d_n7, assign20790_e22645_d_n8, assign20790_e22645_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard385 == 0.0)) && (var_guard387 == 0.0)) && (var_guard388 != 0.0)) {
        let assign20790_e22612: f64 = (-230.25850929940458);
        let assign20790_e22614: f64 = (-var_fbbtsti);
        let assign20790_e22616: f64 = (assign20790_e22614 / var_fmaxr);
        let assign20790_e22617: f64 = (assign20790_e22612 - assign20790_e22616);
        let assign20790_e22621: f64 = (-230.25850929940458);
        let assign20790_e22623: f64 = (-var_fbbtsti);
        let assign20790_e22625: f64 = (assign20790_e22623 / var_fmaxr);
        let assign20790_e22626: f64 = (assign20790_e22621 - assign20790_e22625);
        let assign20790_e22629: f64 = (-230.25850929940458);
        let assign20790_e22631: f64 = (-var_fbbtsti);
        let assign20790_e22633: f64 = (assign20790_e22631 / var_fmaxr);
        let assign20790_e22634: f64 = (assign20790_e22629 - assign20790_e22633);
        let assign20790_e22636: f64 = (assign20790_e22634 * 0.3333333333333333);
        let assign20790_e22637: f64 = (1.0 + assign20790_e22636);
        let assign20790_e22638: f64 = (assign20790_e22626 * assign20790_e22637);
        let assign20790_e22639: f64 = (0.5 * assign20790_e22638);
        let assign20790_e22640: f64 = (1.0 + assign20790_e22639);
        let assign20790_e22641: f64 = (assign20790_e22617 * assign20790_e22640);
        let assign20790_e22642: f64 = (1.0 + assign20790_e22641);
        let assign20790_e22643: f64 = (1e-100 / assign20790_e22642);
        (assign20790_e22643, (-((1e-100 * (((-(-((assign20790_e22614 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign20790_e22640) + (assign20790_e22617 * (0.5 * (((-(-((assign20790_e22623 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign20790_e22637) + (assign20790_e22626 * ((-(-((assign20790_e22631 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign20790_e22642 * assign20790_e22642))), (-((1e-100 * (((-(-((assign20790_e22614 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign20790_e22640) + (assign20790_e22617 * (0.5 * (((-(-((assign20790_e22623 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign20790_e22637) + (assign20790_e22626 * ((-(-((assign20790_e22631 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign20790_e22642 * assign20790_e22642))), (-((1e-100 * (((-(-((assign20790_e22614 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign20790_e22640) + (assign20790_e22617 * (0.5 * (((-(-((assign20790_e22623 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign20790_e22637) + (assign20790_e22626 * ((-(-((assign20790_e22631 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign20790_e22642 * assign20790_e22642))), (-((1e-100 * (((-(-((assign20790_e22614 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign20790_e22640) + (assign20790_e22617 * (0.5 * (((-(-((assign20790_e22623 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign20790_e22637) + (assign20790_e22626 * ((-(-((assign20790_e22631 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign20790_e22642 * assign20790_e22642))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20790_e22645;
        var_tmp_dn6 = assign20790_e22645_d_n6;
        var_tmp_dn7 = assign20790_e22645_d_n7;
        var_tmp_dn8 = assign20790_e22645_d_n8;
        var_tmp_dn9 = assign20790_e22645_d_n9;

        let (assign20800_e22694, assign20800_e22694_d_n6, assign20800_e22694_d_n7, assign20800_e22694_d_n8, assign20800_e22694_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard385 == 0.0)) && (var_guard387 == 0.0)) && (var_guard388 == 0.0)) {
        let assign20800_e22664: f64 = (-var_fbbtsti);
        let assign20800_e22666: f64 = (assign20800_e22664 / var_fmaxr);
        let assign20800_e22668: f64 = (assign20800_e22666 - 230.25850929940458);
        let assign20800_e22672: f64 = (-var_fbbtsti);
        let assign20800_e22674: f64 = (assign20800_e22672 / var_fmaxr);
        let assign20800_e22676: f64 = (assign20800_e22674 - 230.25850929940458);
        let assign20800_e22679: f64 = (-var_fbbtsti);
        let assign20800_e22681: f64 = (assign20800_e22679 / var_fmaxr);
        let assign20800_e22683: f64 = (assign20800_e22681 - 230.25850929940458);
        let assign20800_e22685: f64 = (assign20800_e22683 * 0.3333333333333333);
        let assign20800_e22686: f64 = (1.0 + assign20800_e22685);
        let assign20800_e22687: f64 = (assign20800_e22676 * assign20800_e22686);
        let assign20800_e22688: f64 = (0.5 * assign20800_e22687);
        let assign20800_e22689: f64 = (1.0 + assign20800_e22688);
        let assign20800_e22690: f64 = (assign20800_e22668 * assign20800_e22689);
        let assign20800_e22691: f64 = (1.0 + assign20800_e22690);
        let assign20800_e22692: f64 = (1e100 * assign20800_e22691);
        (assign20800_e22692, (1e100 * (((-((assign20800_e22664 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign20800_e22689) + (assign20800_e22668 * (0.5 * (((-((assign20800_e22672 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign20800_e22686) + (assign20800_e22676 * ((-((assign20800_e22679 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign20800_e22664 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign20800_e22689) + (assign20800_e22668 * (0.5 * (((-((assign20800_e22672 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign20800_e22686) + (assign20800_e22676 * ((-((assign20800_e22679 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign20800_e22664 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign20800_e22689) + (assign20800_e22668 * (0.5 * (((-((assign20800_e22672 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign20800_e22686) + (assign20800_e22676 * ((-((assign20800_e22679 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign20800_e22664 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign20800_e22689) + (assign20800_e22668 * (0.5 * (((-((assign20800_e22672 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign20800_e22686) + (assign20800_e22676 * ((-((assign20800_e22679 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20800_e22694;
        var_tmp_dn6 = assign20800_e22694_d_n6;
        var_tmp_dn7 = assign20800_e22694_d_n7;
        var_tmp_dn8 = assign20800_e22694_d_n8;
        var_tmp_dn9 = assign20800_e22694_d_n9;

        let (assign20810_e22714, assign20810_e22714_d_n6, assign20810_e22714_d_n7, assign20810_e22714_d_n8, assign20810_e22714_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard385 == 0.0)) {
        let assign20810_e22707: f64 = (var_v3 * var_fmaxr);
        let assign20810_e22709: f64 = (assign20810_e22707 * var_fmaxr);
        let assign20810_e22711: f64 = (assign20810_e22709 * var_tmp);
        let assign20810_e22712: f64 = (p.p869 * assign20810_e22711);
        (assign20810_e22712, (p.p869 * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign20810_e22707 * var_fmaxr_dn6)) * var_tmp) + (assign20810_e22709 * var_tmp_dn6))), (p.p869 * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign20810_e22707 * var_fmaxr_dn7)) * var_tmp) + (assign20810_e22709 * var_tmp_dn7))), (p.p869 * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign20810_e22707 * var_fmaxr_dn8)) * var_tmp) + (assign20810_e22709 * var_tmp_dn8))), (p.p869 * (((((var_v3 * var_fmaxr_dn9) * var_fmaxr) + (assign20810_e22707 * var_fmaxr_dn9)) * var_tmp) + (assign20810_e22709 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign20810_e22714;
        var_ibbt_dn6 = assign20810_e22714_d_n6;
        var_ibbt_dn7 = assign20810_e22714_d_n7;
        var_ibbt_dn8 = assign20810_e22714_d_n8;
        var_ibbt_dn9 = assign20810_e22714_d_n9;

        let assign20820_e22717: f64 = if p.p878 > 1000.0 { 1.0 } else { 0.0 };
        var_guard389 = assign20820_e22717;

        let (assign20830_e22728, assign20830_e22728_d_n6, assign20830_e22728_d_n7, assign20830_e22728_d_n8, assign20830_e22728_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard389 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign20830_e22728;
        var_fbreakdown_dn6 = assign20830_e22728_d_n6;
        var_fbreakdown_dn7 = assign20830_e22728_d_n7;
        var_fbreakdown_dn8 = assign20830_e22728_d_n8;
        var_fbreakdown_dn9 = assign20830_e22728_d_n9;

        let assign20840_e22731: f64 = (-var_alphaav);
        let assign20840_e22733: f64 = (assign20840_e22731 * p.p878);
        let assign20840_e22734: f64 = if var_vav > assign20840_e22733 { 1.0 } else { 0.0 };
        var_guard390 = assign20840_e22734;

        let assign20850_e22737: f64 = if p.p881 == 4.0 { 1.0 } else { 0.0 };
        var_guard391 = assign20850_e22737;

        let (assign20860_e22767, assign20860_e22767_d_n6, assign20860_e22767_d_n7, assign20860_e22767_d_n8, assign20860_e22767_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard389 == 0.0)) && (var_guard390 != 0.0)) && (var_guard391 != 0.0)) {
        let assign20860_e22753: f64 = (var_vav * var_vbrinvsti);
        let assign20860_e22756: f64 = (var_vav * var_vbrinvsti);
        let assign20860_e22757: f64 = (assign20860_e22753 * assign20860_e22756);
        let assign20860_e22760: f64 = (var_vav * var_vbrinvsti);
        let assign20860_e22761: f64 = (assign20860_e22757 * assign20860_e22760);
        let assign20860_e22764: f64 = (var_vav * var_vbrinvsti);
        let assign20860_e22765: f64 = (assign20860_e22761 * assign20860_e22764);
        (assign20860_e22765, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20860_e22767;
        var_tmp_dn6 = assign20860_e22767_d_n6;
        var_tmp_dn7 = assign20860_e22767_d_n7;
        var_tmp_dn8 = assign20860_e22767_d_n8;
        var_tmp_dn9 = assign20860_e22767_d_n9;

        let (assign20870_e22789, assign20870_e22789_d_n6, assign20870_e22789_d_n7, assign20870_e22789_d_n8, assign20870_e22789_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard389 == 0.0)) && (var_guard390 != 0.0)) && (var_guard391 == 0.0)) {
        let assign20870_e22784: f64 = (var_vav * var_vbrinvsti);
        let assign20870_e22785: f64 = (assign20870_e22784).abs();
        let assign20870_e22787: f64 = (assign20870_e22785).powf(p.p881);
        (assign20870_e22787, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign20870_e22789;
        var_tmp_dn6 = assign20870_e22789_d_n6;
        var_tmp_dn7 = assign20870_e22789_d_n7;
        var_tmp_dn8 = assign20870_e22789_d_n8;
        var_tmp_dn9 = assign20870_e22789_d_n9;

        let (assign20880_e22807, assign20880_e22807_d_n6, assign20880_e22807_d_n7, assign20880_e22807_d_n8, assign20880_e22807_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard389 == 0.0)) && (var_guard390 != 0.0)) {
        let assign20880_e22804: f64 = (1.0 - var_tmp);
        let assign20880_e22805: f64 = (1.0 / assign20880_e22804);
        (assign20880_e22805, (-((-var_tmp_dn6) / (assign20880_e22804 * assign20880_e22804))), (-((-var_tmp_dn7) / (assign20880_e22804 * assign20880_e22804))), (-((-var_tmp_dn8) / (assign20880_e22804 * assign20880_e22804))), (-((-var_tmp_dn9) / (assign20880_e22804 * assign20880_e22804))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign20880_e22807;
        var_fbreakdown_dn6 = assign20880_e22807_d_n6;
        var_fbreakdown_dn7 = assign20880_e22807_d_n7;
        var_fbreakdown_dn8 = assign20880_e22807_d_n8;
        var_fbreakdown_dn9 = assign20880_e22807_d_n9;

        let (assign20890_e22830, assign20890_e22830_d_n6, assign20890_e22830_d_n7, assign20890_e22830_d_n8, assign20890_e22830_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) && (var_guard389 == 0.0)) && (var_guard390 == 0.0)) {
        let assign20890_e22824: f64 = (var_alphaav * p.p878);
        let assign20890_e22825: f64 = (var_vav + assign20890_e22824);
        let assign20890_e22827: f64 = (assign20890_e22825 * var_slopesti);
        let assign20890_e22828: f64 = (var_fstopsti + assign20890_e22827);
        (assign20890_e22828, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign20890_e22830;
        var_fbreakdown_dn6 = assign20890_e22830_d_n6;
        var_fbreakdown_dn7 = assign20890_e22830_d_n7;
        var_fbreakdown_dn8 = assign20890_e22830_d_n8;
        var_fbreakdown_dn9 = assign20890_e22830_d_n9;

        let (assign20900_e22849, assign20900_e22849_d_n6, assign20900_e22849_d_n7, assign20900_e22849_d_n8, assign20900_e22849_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard375 == 0.0)) {
        let assign20900_e22840: f64 = (var_id__blk212 + var_isrh);
        let assign20900_e22842: f64 = (assign20900_e22840 + var_itat);
        let assign20900_e22844: f64 = (assign20900_e22842 + var_ibbt);
        let assign20900_e22845: f64 = (p.p29 * assign20900_e22844);
        let assign20900_e22847: f64 = (assign20900_e22845 * var_fbreakdown);
        (assign20900_e22847, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign20900_e22845 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign20900_e22845 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign20900_e22845 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign20900_e22845 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign20900_e22849;
        var_ijunsti_dn6 = assign20900_e22849_d_n6;
        var_ijunsti_dn7 = assign20900_e22849_d_n7;
        var_ijunsti_dn8 = assign20900_e22849_d_n8;
        var_ijunsti_dn9 = assign20900_e22849_d_n9;

        let assign20910_e22852: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard392 = assign20910_e22852;

        let (assign20920_e22860, assign20920_e22860_d_n6, assign20920_e22860_d_n7, assign20920_e22860_d_n8, assign20920_e22860_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign20920_e22860;
        var_ijungat_dn6 = assign20920_e22860_d_n6;
        var_ijungat_dn7 = assign20920_e22860_d_n7;
        var_ijungat_dn8 = assign20920_e22860_d_n8;
        var_ijungat_dn9 = assign20920_e22860_d_n9;

        let (assign20930_e22871,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) {
        let assign20930_e22869: f64 = (var_idsatgat * var_idmult);
        (assign20930_e22869,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign20930_e22871;

        let assign20940_e22878: f64 = if ((p.p859 == 0.0) && (p.p864 == 0.0)) { 1.0 } else { 0.0 };
        var_guard393 = assign20940_e22878;

        let (assign20950_e22889, assign20950_e22889_d_n6, assign20950_e22889_d_n7, assign20950_e22889_d_n8, assign20950_e22889_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard393 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign20950_e22889;
        var_isrh_dn6 = assign20950_e22889_d_n6;
        var_isrh_dn7 = assign20950_e22889_d_n7;
        var_isrh_dn8 = assign20950_e22889_d_n8;
        var_isrh_dn9 = assign20950_e22889_d_n9;

        let (assign20960_e22903,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard393 == 0.0)) {
        let assign20960_e22901: f64 = (var_vbigat - var_vjsrh);
        (assign20960_e22901,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign20960_e22903;

        let (assign20970_e22922,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard393 == 0.0)) {
        let assign20970_e22917: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign20970_e22918: f64 = (1.0 - assign20970_e22917);
        let assign20970_e22919: f64 = (assign20970_e22918).sqrt();
        let assign20970_e22920: f64 = (1.0 - assign20970_e22919);
        (assign20970_e22920,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign20970_e22922;

        let assign20980_e22925: f64 = if p.p850 == 0.5 { 1.0 } else { 0.0 };
        var_guard394 = assign20980_e22925;

        let (assign20990_e22939,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard393 == 0.0)) && (var_guard394 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign20990_e22939;

        let (assign21000_e22971,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard393 == 0.0)) && (var_guard394 == 0.0)) {
        let assign21000_e22954: f64 = (var_wsrhstep * var_wsrhstep);
        let assign21000_e22956: f64 = (var_wsrhstep).ln();
        let assign21000_e22957: f64 = (assign21000_e22954 * assign21000_e22956);
        let assign21000_e22960: f64 = (1.0 - var_wsrhstep);
        let assign21000_e22961: f64 = (assign21000_e22957 / assign21000_e22960);
        let assign21000_e22963: f64 = (assign21000_e22961 + var_wsrhstep);
        let assign21000_e22967: f64 = (2.0 * p.p850);
        let assign21000_e22968: f64 = (1.0 - assign21000_e22967);
        let assign21000_e22969: f64 = (assign21000_e22963 * assign21000_e22968);
        (assign21000_e22969,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign21000_e22971;

        let (assign21010_e22985,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard393 == 0.0)) {
        let assign21010_e22983: f64 = (var_wsrhstep + var_dwsrh);
        (assign21010_e22983,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign21010_e22985;

        let assign21020_e22988: f64 = if p.p850 == 0.5 { 1.0 } else { 0.0 };
        var_guard395 = assign21020_e22988;

        let (assign21030_e23005, assign21030_e23005_d_n6, assign21030_e23005_d_n7, assign21030_e23005_d_n8, assign21030_e23005_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard393 == 0.0)) && (var_guard395 != 0.0)) {
        let assign21030_e23002: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign21030_e23003: f64 = (assign21030_e23002).sqrt();
        (assign21030_e23003, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign21030_e23005;
        var_tmp_dn6 = assign21030_e23005_d_n6;
        var_tmp_dn7 = assign21030_e23005_d_n7;
        var_tmp_dn8 = assign21030_e23005_d_n8;
        var_tmp_dn9 = assign21030_e23005_d_n9;

        let (assign21040_e23024, assign21040_e23024_d_n6, assign21040_e23024_d_n7, assign21040_e23024_d_n8, assign21040_e23024_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard393 == 0.0)) && (var_guard395 == 0.0)) {
        let assign21040_e23020: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign21040_e23022: f64 = (assign21040_e23020).powf(p.p850);
        (assign21040_e23022, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign21040_e23024;
        var_tmp_dn6 = assign21040_e23024_d_n6;
        var_tmp_dn7 = assign21040_e23024_d_n7;
        var_tmp_dn8 = assign21040_e23024_d_n8;
        var_tmp_dn9 = assign21040_e23024_d_n9;

        let (assign21050_e23038, assign21050_e23038_d_n6, assign21050_e23038_d_n7, assign21050_e23038_d_n8, assign21050_e23038_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard393 == 0.0)) {
        let assign21050_e23036: f64 = (var_wdepnulrgat * var_tmp);
        (assign21050_e23036, (var_wdepnulrgat * var_tmp_dn6), (var_wdepnulrgat * var_tmp_dn7), (var_wdepnulrgat * var_tmp_dn8), (var_wdepnulrgat * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign21050_e23038;
        var_wdep_dn6 = assign21050_e23038_d_n6;
        var_wdep_dn7 = assign21050_e23038_d_n7;
        var_wdep_dn8 = assign21050_e23038_d_n8;
        var_wdep_dn9 = assign21050_e23038_d_n9;

        let (assign21060_e23056, assign21060_e23056_d_n6, assign21060_e23056_d_n7, assign21060_e23056_d_n8, assign21060_e23056_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard393 == 0.0)) {
        let assign21060_e23051: f64 = (var_zinv - 1.0);
        let assign21060_e23053: f64 = (assign21060_e23051 * var_wdep);
        let assign21060_e23054: f64 = (var_ftdgat * assign21060_e23053);
        (assign21060_e23054, (var_ftdgat * (assign21060_e23051 * var_wdep_dn6)), (var_ftdgat * (assign21060_e23051 * var_wdep_dn7)), (var_ftdgat * (assign21060_e23051 * var_wdep_dn8)), (var_ftdgat * (assign21060_e23051 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign21060_e23056;
        var_asrh_dn6 = assign21060_e23056_d_n6;
        var_asrh_dn7 = assign21060_e23056_d_n7;
        var_asrh_dn8 = assign21060_e23056_d_n8;
        var_asrh_dn9 = assign21060_e23056_d_n9;

        let (assign21070_e23072, assign21070_e23072_d_n6, assign21070_e23072_d_n7, assign21070_e23072_d_n8, assign21070_e23072_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard393 == 0.0)) {
        let assign21070_e23069: f64 = (var_asrh * var_wsrh);
        let assign21070_e23070: f64 = (p.p859 * assign21070_e23069);
        (assign21070_e23070, (p.p859 * (var_asrh_dn6 * var_wsrh)), (p.p859 * (var_asrh_dn7 * var_wsrh)), (p.p859 * (var_asrh_dn8 * var_wsrh)), (p.p859 * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign21070_e23072;
        var_isrh_dn6 = assign21070_e23072_d_n6;
        var_isrh_dn7 = assign21070_e23072_d_n7;
        var_isrh_dn8 = assign21070_e23072_d_n8;
        var_isrh_dn9 = assign21070_e23072_d_n9;

        let assign21080_e23075: f64 = if p.p864 == 0.0 { 1.0 } else { 0.0 };
        var_guard396 = assign21080_e23075;

        let (assign21090_e23086, assign21090_e23086_d_n6, assign21090_e23086_d_n7, assign21090_e23086_d_n8, assign21090_e23086_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign21090_e23086;
        var_itat_dn6 = assign21090_e23086_d_n6;
        var_itat_dn7 = assign21090_e23086_d_n7;
        var_itat_dn8 = assign21090_e23086_d_n8;
        var_itat_dn9 = assign21090_e23086_d_n9;

        let (assign21100_e23104, assign21100_e23104_d_n6, assign21100_e23104_d_n7, assign21100_e23104_d_n8, assign21100_e23104_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) {
        let assign21100_e23099: f64 = (var_wdep * var_one_minus_pgat);
        let assign21100_e23101: f64 = (assign21100_e23099 / var_vbi_minus_vjsrh);
        let assign21100_e23102: f64 = (var_btatpartgat * assign21100_e23101);
        (assign21100_e23102, (var_btatpartgat * ((var_wdep_dn6 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn7 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn8 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn9 * var_one_minus_pgat) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign21100_e23104;
        var_btat_dn6 = assign21100_e23104_d_n6;
        var_btat_dn7 = assign21100_e23104_d_n7;
        var_btat_dn8 = assign21100_e23104_d_n8;
        var_btat_dn9 = assign21100_e23104_d_n9;

        let (assign21110_e23120, assign21110_e23120_d_n6, assign21110_e23120_d_n7, assign21110_e23120_d_n8, assign21110_e23120_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) {
        let assign21110_e23116: f64 = (0.666666666666667 * var_atatgat);
        let assign21110_e23118: f64 = (assign21110_e23116 / var_btat);
        (assign21110_e23118, (-((assign21110_e23116 * var_btat_dn6) / (var_btat * var_btat))), (-((assign21110_e23116 * var_btat_dn7) / (var_btat * var_btat))), (-((assign21110_e23116 * var_btat_dn8) / (var_btat * var_btat))), (-((assign21110_e23116 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign21110_e23120;
        var_twoatatoverthreebtat_dn6 = assign21110_e23120_d_n6;
        var_twoatatoverthreebtat_dn7 = assign21110_e23120_d_n7;
        var_twoatatoverthreebtat_dn8 = assign21110_e23120_d_n8;
        var_twoatatoverthreebtat_dn9 = assign21110_e23120_d_n9;

        let (assign21120_e23134, assign21120_e23134_d_n6, assign21120_e23134_d_n7, assign21120_e23134_d_n8, assign21120_e23134_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) {
        let assign21120_e23132: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign21120_e23132, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign21120_e23134;
        var_umaxbeforelimiting_dn6 = assign21120_e23134_d_n6;
        var_umaxbeforelimiting_dn7 = assign21120_e23134_d_n7;
        var_umaxbeforelimiting_dn8 = assign21120_e23134_d_n8;
        var_umaxbeforelimiting_dn9 = assign21120_e23134_d_n9;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_asrh_dn9_slot = var_asrh_dn9;
        *var_btat_slot = var_btat;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_btat_dn9_slot = var_btat_dn9;
        *var_dwsrh_slot = var_dwsrh;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_fmaxr_dn9_slot = var_fmaxr_dn9;
        *var_guard387_slot = var_guard387;
        *var_guard388_slot = var_guard388;
        *var_guard389_slot = var_guard389;
        *var_guard390_slot = var_guard390;
        *var_guard391_slot = var_guard391;
        *var_guard392_slot = var_guard392;
        *var_guard393_slot = var_guard393;
        *var_guard394_slot = var_guard394;
        *var_guard395_slot = var_guard395;
        *var_guard396_slot = var_guard396;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_id__blk212_slot = var_id__blk212;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_ijungat_dn9_slot = var_ijungat_dn9;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_ijunsti_dn9_slot = var_ijunsti_dn9;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_twoatatoverthreebtat_dn9_slot = var_twoatatoverthreebtat_dn9;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxbeforelimiting_dn9_slot = var_umaxbeforelimiting_dn9;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wdep_dn9_slot = var_wdep_dn9;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
    }

    pub(super) fn stamp_transient_block_38(
        p: &Parameters,
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatgat: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_btat_dn9: f64,
        var_cerfc: f64,
        var_fbbtgat: f64,
        var_fbbtgat_dn6: f64,
        var_fbbtgat_dn7: f64,
        var_fbbtgat_dn8: f64,
        var_fbbtgat_dn9: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard392: f64,
        var_guard396: f64,
        var_one_over_one_minus_pgat: f64,
        var_perfc: f64,
        var_twoatatoverthreebtat: f64,
        var_twoatatoverthreebtat_dn6: f64,
        var_twoatatoverthreebtat_dn7: f64,
        var_twoatatoverthreebtat_dn8: f64,
        var_twoatatoverthreebtat_dn9: f64,
        var_umaxbeforelimiting: f64,
        var_umaxbeforelimiting_dn6: f64,
        var_umaxbeforelimiting_dn7: f64,
        var_umaxbeforelimiting_dn8: f64,
        var_umaxbeforelimiting_dn9: f64,
        var_vbbt: f64,
        var_vbirgatinv: f64,
        var_wdepnulrinvgat: f64,
        var_wsrh: f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfcpos_dn9_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_erfctimesexpmtat_dn9_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_fmaxr_dn9_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_gammamax_dn9_slot: &mut f64,
        var_guard397_slot: &mut f64,
        var_guard398_slot: &mut f64,
        var_guard399_slot: &mut f64,
        var_guard400_slot: &mut f64,
        var_guard401_slot: &mut f64,
        var_guard402_slot: &mut f64,
        var_guard403_slot: &mut f64,
        var_guard404_slot: &mut f64,
        var_guard405_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ktat_dn9_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_ltat_dn9_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_mtat_dn9_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_sqrtumax_dn9_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_terfc_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umax_dn9_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_dn9_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wgamma_dn9_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_wtat_dn9_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_xerfc_dn9_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
        var_ysq_dn9_slot: &mut f64,
    ) {
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfcpos_dn9: f64 = *var_erfcpos_dn9_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_erfctimesexpmtat_dn9: f64 = *var_erfctimesexpmtat_dn9_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_fmaxr_dn9: f64 = *var_fmaxr_dn9_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_gammamax_dn9: f64 = *var_gammamax_dn9_slot;
        let mut var_guard397: f64 = *var_guard397_slot;
        let mut var_guard398: f64 = *var_guard398_slot;
        let mut var_guard399: f64 = *var_guard399_slot;
        let mut var_guard400: f64 = *var_guard400_slot;
        let mut var_guard401: f64 = *var_guard401_slot;
        let mut var_guard402: f64 = *var_guard402_slot;
        let mut var_guard403: f64 = *var_guard403_slot;
        let mut var_guard404: f64 = *var_guard404_slot;
        let mut var_guard405: f64 = *var_guard405_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ktat_dn9: f64 = *var_ktat_dn9_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_ltat_dn9: f64 = *var_ltat_dn9_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_mtat_dn9: f64 = *var_mtat_dn9_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_sqrtumax_dn9: f64 = *var_sqrtumax_dn9_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_terfc_dn9: f64 = *var_terfc_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umax_dn9: f64 = *var_umax_dn9_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_umaxpoweronepointfive_dn9: f64 = *var_umaxpoweronepointfive_dn9_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wgamma_dn9: f64 = *var_wgamma_dn9_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_wtat_dn9: f64 = *var_wtat_dn9_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_xerfc_dn9: f64 = *var_xerfc_dn9_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;
        let mut var_ysq_dn9: f64 = *var_ysq_dn9_slot;

        let (assign21130_e23155, assign21130_e23155_d_n6, assign21130_e23155_d_n7, assign21130_e23155_d_n8, assign21130_e23155_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) {
        let assign21130_e23146: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign21130_e23149: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign21130_e23151: f64 = (assign21130_e23149 + 1.0);
        let assign21130_e23152: f64 = (assign21130_e23146 / assign21130_e23151);
        let assign21130_e23153: f64 = (assign21130_e23152).sqrt();
        (assign21130_e23153, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign21130_e23151) - (assign21130_e23146 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign21130_e23151 * assign21130_e23151)) / (2.0 * assign21130_e23153)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign21130_e23151) - (assign21130_e23146 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign21130_e23151 * assign21130_e23151)) / (2.0 * assign21130_e23153)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign21130_e23151) - (assign21130_e23146 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign21130_e23151 * assign21130_e23151)) / (2.0 * assign21130_e23153)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign21130_e23151) - (assign21130_e23146 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign21130_e23151 * assign21130_e23151)) / (2.0 * assign21130_e23153)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign21130_e23155;
        var_umax_dn6 = assign21130_e23155_d_n6;
        var_umax_dn7 = assign21130_e23155_d_n7;
        var_umax_dn8 = assign21130_e23155_d_n8;
        var_umax_dn9 = assign21130_e23155_d_n9;

        let (assign21140_e23168, assign21140_e23168_d_n6, assign21140_e23168_d_n7, assign21140_e23168_d_n8, assign21140_e23168_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) {
        let assign21140_e23166: f64 = (var_umax).sqrt();
        (assign21140_e23166, (var_umax_dn6 / (2.0 * assign21140_e23166)), (var_umax_dn7 / (2.0 * assign21140_e23166)), (var_umax_dn8 / (2.0 * assign21140_e23166)), (var_umax_dn9 / (2.0 * assign21140_e23166)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign21140_e23168;
        var_sqrtumax_dn6 = assign21140_e23168_d_n6;
        var_sqrtumax_dn7 = assign21140_e23168_d_n7;
        var_sqrtumax_dn8 = assign21140_e23168_d_n8;
        var_sqrtumax_dn9 = assign21140_e23168_d_n9;

        let (assign21150_e23182, assign21150_e23182_d_n6, assign21150_e23182_d_n7, assign21150_e23182_d_n8, assign21150_e23182_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) {
        let assign21150_e23180: f64 = (var_umax * var_sqrtumax);
        (assign21150_e23180, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign21150_e23182;
        var_umaxpoweronepointfive_dn6 = assign21150_e23182_d_n6;
        var_umaxpoweronepointfive_dn7 = assign21150_e23182_d_n7;
        var_umaxpoweronepointfive_dn8 = assign21150_e23182_d_n8;
        var_umaxpoweronepointfive_dn9 = assign21150_e23182_d_n9;

        let assign21160_e23184: f64 = (-p.p850);
        let assign21160_e23186: f64 = (assign21160_e23184 * var_one_over_one_minus_pgat);
        let assign21160_e23188: f64 = (-1.0);
        let assign21160_e23189: f64 = if assign21160_e23186 == assign21160_e23188 { 1.0 } else { 0.0 };
        var_guard397 = assign21160_e23189;

        let (assign21170_e23209, assign21170_e23209_d_n6, assign21170_e23209_d_n7, assign21170_e23209_d_n8, assign21170_e23209_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) && (var_guard397 != 0.0)) {
        let assign21170_e23205: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign21170_e23206: f64 = (1.0 + assign21170_e23205);
        let assign21170_e23207: f64 = (1.0 / assign21170_e23206);
        (assign21170_e23207, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign21170_e23206 * assign21170_e23206))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign21170_e23206 * assign21170_e23206))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign21170_e23206 * assign21170_e23206))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign21170_e23206 * assign21170_e23206))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign21170_e23209;
        var_wgamma_dn6 = assign21170_e23209_d_n6;
        var_wgamma_dn7 = assign21170_e23209_d_n7;
        var_wgamma_dn8 = assign21170_e23209_d_n8;
        var_wgamma_dn9 = assign21170_e23209_d_n9;

        let (assign21180_e23233, assign21180_e23233_d_n6, assign21180_e23233_d_n7, assign21180_e23233_d_n8, assign21180_e23233_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) && (var_guard397 == 0.0)) {
        let assign21180_e23225: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign21180_e23226: f64 = (1.0 + assign21180_e23225);
        let assign21180_e23228: f64 = (-p.p850);
        let assign21180_e23230: f64 = (assign21180_e23228 * var_one_over_one_minus_pgat);
        let assign21180_e23231: f64 = (assign21180_e23226).powf(assign21180_e23230);
        (assign21180_e23231, if 0.0 == 0.0 && ((assign21180_e23230) as f64).is_finite() && ((assign21180_e23230) as f64).fract() == 0.0 { if assign21180_e23230 == 0.0 { 0.0 } else { (assign21180_e23230 * ((assign21180_e23226).powf(assign21180_e23230 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign21180_e23231 * (assign21180_e23230 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign21180_e23226))) }, if 0.0 == 0.0 && ((assign21180_e23230) as f64).is_finite() && ((assign21180_e23230) as f64).fract() == 0.0 { if assign21180_e23230 == 0.0 { 0.0 } else { (assign21180_e23230 * ((assign21180_e23226).powf(assign21180_e23230 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign21180_e23231 * (assign21180_e23230 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign21180_e23226))) }, if 0.0 == 0.0 && ((assign21180_e23230) as f64).is_finite() && ((assign21180_e23230) as f64).fract() == 0.0 { if assign21180_e23230 == 0.0 { 0.0 } else { (assign21180_e23230 * ((assign21180_e23226).powf(assign21180_e23230 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign21180_e23231 * (assign21180_e23230 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign21180_e23226))) }, if 0.0 == 0.0 && ((assign21180_e23230) as f64).is_finite() && ((assign21180_e23230) as f64).fract() == 0.0 { if assign21180_e23230 == 0.0 { 0.0 } else { (assign21180_e23230 * ((assign21180_e23226).powf(assign21180_e23230 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign21180_e23231 * (assign21180_e23230 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign21180_e23226))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign21180_e23233;
        var_wgamma_dn6 = assign21180_e23233_d_n6;
        var_wgamma_dn7 = assign21180_e23233_d_n7;
        var_wgamma_dn8 = assign21180_e23233_d_n8;
        var_wgamma_dn9 = assign21180_e23233_d_n9;

        let (assign21190_e23251, assign21190_e23251_d_n6, assign21190_e23251_d_n7, assign21190_e23251_d_n8, assign21190_e23251_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) {
        let assign21190_e23245: f64 = (var_wsrh * var_wgamma);
        let assign21190_e23248: f64 = (var_wsrh + var_wgamma);
        let assign21190_e23249: f64 = (assign21190_e23245 / assign21190_e23248);
        (assign21190_e23249, ((((var_wsrh * var_wgamma_dn6) * assign21190_e23248) - (assign21190_e23245 * var_wgamma_dn6)) / (assign21190_e23248 * assign21190_e23248)), ((((var_wsrh * var_wgamma_dn7) * assign21190_e23248) - (assign21190_e23245 * var_wgamma_dn7)) / (assign21190_e23248 * assign21190_e23248)), ((((var_wsrh * var_wgamma_dn8) * assign21190_e23248) - (assign21190_e23245 * var_wgamma_dn8)) / (assign21190_e23248 * assign21190_e23248)), ((((var_wsrh * var_wgamma_dn9) * assign21190_e23248) - (assign21190_e23245 * var_wgamma_dn9)) / (assign21190_e23248 * assign21190_e23248)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign21190_e23251;
        var_wtat_dn6 = assign21190_e23251_d_n6;
        var_wtat_dn7 = assign21190_e23251_d_n7;
        var_wtat_dn8 = assign21190_e23251_d_n8;
        var_wtat_dn9 = assign21190_e23251_d_n9;

        let (assign21200_e23268, assign21200_e23268_d_n6, assign21200_e23268_d_n7, assign21200_e23268_d_n8, assign21200_e23268_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) {
        let assign21200_e23264: f64 = (var_btat / var_sqrtumax);
        let assign21200_e23265: f64 = (0.375 * assign21200_e23264);
        let assign21200_e23266: f64 = (assign21200_e23265).sqrt();
        (assign21200_e23266, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign21200_e23266)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign21200_e23266)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign21200_e23266)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign21200_e23266)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign21200_e23268;
        var_ktat_dn6 = assign21200_e23268_d_n6;
        var_ktat_dn7 = assign21200_e23268_d_n7;
        var_ktat_dn8 = assign21200_e23268_d_n8;
        var_ktat_dn9 = assign21200_e23268_d_n9;

        let (assign21210_e23286, assign21210_e23286_d_n6, assign21210_e23286_d_n7, assign21210_e23286_d_n8, assign21210_e23286_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) {
        let assign21210_e23281: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign21210_e23282: f64 = (2.0 * assign21210_e23281);
        let assign21210_e23284: f64 = (assign21210_e23282 - var_umax);
        (assign21210_e23284, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign21210_e23286;
        var_ltat_dn6 = assign21210_e23286_d_n6;
        var_ltat_dn7 = assign21210_e23286_d_n7;
        var_ltat_dn8 = assign21210_e23286_d_n8;
        var_ltat_dn9 = assign21210_e23286_d_n9;

        let (assign21220_e23312, assign21220_e23312_d_n6, assign21220_e23312_d_n7, assign21220_e23312_d_n8, assign21220_e23312_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) {
        let assign21220_e23298: f64 = (var_atatgat * var_twoatatoverthreebtat);
        let assign21220_e23300: f64 = (assign21220_e23298 * var_sqrtumax);
        let assign21220_e23303: f64 = (var_atatgat * var_umax);
        let assign21220_e23304: f64 = (assign21220_e23300 - assign21220_e23303);
        let assign21220_e23308: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign21220_e23309: f64 = (0.5 * assign21220_e23308);
        let assign21220_e23310: f64 = (assign21220_e23304 + assign21220_e23309);
        (assign21220_e23310, (((((var_atatgat * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign21220_e23298 * var_sqrtumax_dn6)) - (var_atatgat * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign21220_e23298 * var_sqrtumax_dn7)) - (var_atatgat * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign21220_e23298 * var_sqrtumax_dn8)) - (var_atatgat * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatgat * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign21220_e23298 * var_sqrtumax_dn9)) - (var_atatgat * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign21220_e23312;
        var_mtat_dn6 = assign21220_e23312_d_n6;
        var_mtat_dn7 = assign21220_e23312_d_n7;
        var_mtat_dn8 = assign21220_e23312_d_n8;
        var_mtat_dn9 = assign21220_e23312_d_n9;

        let (assign21230_e23328, assign21230_e23328_d_n6, assign21230_e23328_d_n7, assign21230_e23328_d_n8, assign21230_e23328_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) {
        let assign21230_e23324: f64 = (var_ltat - 1.0);
        let assign21230_e23326: f64 = (assign21230_e23324 * var_ktat);
        (assign21230_e23326, ((var_ltat_dn6 * var_ktat) + (assign21230_e23324 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign21230_e23324 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign21230_e23324 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign21230_e23324 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign21230_e23328;
        var_xerfc_dn6 = assign21230_e23328_d_n6;
        var_xerfc_dn7 = assign21230_e23328_d_n7;
        var_xerfc_dn8 = assign21230_e23328_d_n8;
        var_xerfc_dn9 = assign21230_e23328_d_n9;

        let (assign21240_e23342, assign21240_e23342_d_n6, assign21240_e23342_d_n7, assign21240_e23342_d_n8, assign21240_e23342_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) {
        let assign21240_e23340: f64 = (var_xerfc * var_xerfc);
        (assign21240_e23340, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign21240_e23342;
        var_ysq_dn6 = assign21240_e23342_d_n6;
        var_ysq_dn7 = assign21240_e23342_d_n7;
        var_ysq_dn8 = assign21240_e23342_d_n8;
        var_ysq_dn9 = assign21240_e23342_d_n9;

        let assign21250_e23345: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard398 = assign21250_e23345;

        let (assign21260_e23365, assign21260_e23365_d_n6, assign21260_e23365_d_n7, assign21260_e23365_d_n8, assign21260_e23365_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) && (var_guard398 != 0.0)) {
        let assign21260_e23361: f64 = (var_perfc * var_xerfc);
        let assign21260_e23362: f64 = (1.0 + assign21260_e23361);
        let assign21260_e23363: f64 = (1.0 / assign21260_e23362);
        (assign21260_e23363, (-((var_perfc * var_xerfc_dn6) / (assign21260_e23362 * assign21260_e23362))), (-((var_perfc * var_xerfc_dn7) / (assign21260_e23362 * assign21260_e23362))), (-((var_perfc * var_xerfc_dn8) / (assign21260_e23362 * assign21260_e23362))), (-((var_perfc * var_xerfc_dn9) / (assign21260_e23362 * assign21260_e23362))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign21260_e23365;
        var_terfc_dn6 = assign21260_e23365_d_n6;
        var_terfc_dn7 = assign21260_e23365_d_n7;
        var_terfc_dn8 = assign21260_e23365_d_n8;
        var_terfc_dn9 = assign21260_e23365_d_n9;

        let (assign21270_e23386, assign21270_e23386_d_n6, assign21270_e23386_d_n7, assign21270_e23386_d_n8, assign21270_e23386_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) && (var_guard398 == 0.0)) {
        let assign21270_e23382: f64 = (var_perfc * var_xerfc);
        let assign21270_e23383: f64 = (1.0 - assign21270_e23382);
        let assign21270_e23384: f64 = (1.0 / assign21270_e23383);
        (assign21270_e23384, (-((-(var_perfc * var_xerfc_dn6)) / (assign21270_e23383 * assign21270_e23383))), (-((-(var_perfc * var_xerfc_dn7)) / (assign21270_e23383 * assign21270_e23383))), (-((-(var_perfc * var_xerfc_dn8)) / (assign21270_e23383 * assign21270_e23383))), (-((-(var_perfc * var_xerfc_dn9)) / (assign21270_e23383 * assign21270_e23383))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign21270_e23386;
        var_terfc_dn6 = assign21270_e23386_d_n6;
        var_terfc_dn7 = assign21270_e23386_d_n7;
        var_terfc_dn8 = assign21270_e23386_d_n8;
        var_terfc_dn9 = assign21270_e23386_d_n9;

        let assign21280_e23388: f64 = (-var_ysq);
        let assign21280_e23390: f64 = (assign21280_e23388 + var_mtat);
        let assign21280_e23392: f64 = (-230.25850929940458);
        let assign21280_e23393: f64 = if assign21280_e23390 > assign21280_e23392 { 1.0 } else { 0.0 };
        var_guard399 = assign21280_e23393;

        let (assign21290_e23411, assign21290_e23411_d_n6, assign21290_e23411_d_n7, assign21290_e23411_d_n8, assign21290_e23411_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) && (var_guard399 != 0.0)) {
        let assign21290_e23406: f64 = (-var_ysq);
        let assign21290_e23408: f64 = (assign21290_e23406 + var_mtat);
        let assign21290_e23409: f64 = (assign21290_e23408).exp();
        (assign21290_e23409, (assign21290_e23409 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign21290_e23409 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign21290_e23409 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign21290_e23409 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign21290_e23411;
        var_tmp_dn6 = assign21290_e23411_d_n6;
        var_tmp_dn7 = assign21290_e23411_d_n7;
        var_tmp_dn8 = assign21290_e23411_d_n8;
        var_tmp_dn9 = assign21290_e23411_d_n9;

        let (assign21300_e23460, assign21300_e23460_d_n6, assign21300_e23460_d_n7, assign21300_e23460_d_n8, assign21300_e23460_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) && (var_guard399 == 0.0)) {
        let assign21300_e23427: f64 = (-230.25850929940458);
        let assign21300_e23429: f64 = (-var_ysq);
        let assign21300_e23431: f64 = (assign21300_e23429 + var_mtat);
        let assign21300_e23432: f64 = (assign21300_e23427 - assign21300_e23431);
        let assign21300_e23436: f64 = (-230.25850929940458);
        let assign21300_e23438: f64 = (-var_ysq);
        let assign21300_e23440: f64 = (assign21300_e23438 + var_mtat);
        let assign21300_e23441: f64 = (assign21300_e23436 - assign21300_e23440);
        let assign21300_e23444: f64 = (-230.25850929940458);
        let assign21300_e23446: f64 = (-var_ysq);
        let assign21300_e23448: f64 = (assign21300_e23446 + var_mtat);
        let assign21300_e23449: f64 = (assign21300_e23444 - assign21300_e23448);
        let assign21300_e23451: f64 = (assign21300_e23449 * 0.3333333333333333);
        let assign21300_e23452: f64 = (1.0 + assign21300_e23451);
        let assign21300_e23453: f64 = (assign21300_e23441 * assign21300_e23452);
        let assign21300_e23454: f64 = (0.5 * assign21300_e23453);
        let assign21300_e23455: f64 = (1.0 + assign21300_e23454);
        let assign21300_e23456: f64 = (assign21300_e23432 * assign21300_e23455);
        let assign21300_e23457: f64 = (1.0 + assign21300_e23456);
        let assign21300_e23458: f64 = (1e-100 / assign21300_e23457);
        (assign21300_e23458, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign21300_e23455) + (assign21300_e23432 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign21300_e23452) + (assign21300_e23441 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign21300_e23457 * assign21300_e23457))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign21300_e23455) + (assign21300_e23432 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign21300_e23452) + (assign21300_e23441 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign21300_e23457 * assign21300_e23457))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign21300_e23455) + (assign21300_e23432 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign21300_e23452) + (assign21300_e23441 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign21300_e23457 * assign21300_e23457))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign21300_e23455) + (assign21300_e23432 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign21300_e23452) + (assign21300_e23441 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign21300_e23457 * assign21300_e23457))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign21300_e23460;
        var_tmp_dn6 = assign21300_e23460_d_n6;
        var_tmp_dn7 = assign21300_e23460_d_n7;
        var_tmp_dn8 = assign21300_e23460_d_n8;
        var_tmp_dn9 = assign21300_e23460_d_n9;

        let (assign21310_e23490, assign21310_e23490_d_n6, assign21310_e23490_d_n7, assign21310_e23490_d_n8, assign21310_e23490_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) {
        let assign21310_e23472: f64 = (0.29214664 * var_terfc);
        let assign21310_e23476: f64 = (var_terfc * var_terfc);
        let assign21310_e23477: f64 = (var_berfc * assign21310_e23476);
        let assign21310_e23478: f64 = (assign21310_e23472 + assign21310_e23477);
        let assign21310_e23482: f64 = (var_terfc * var_terfc);
        let assign21310_e23484: f64 = (assign21310_e23482 * var_terfc);
        let assign21310_e23485: f64 = (var_cerfc * assign21310_e23484);
        let assign21310_e23486: f64 = (assign21310_e23478 + assign21310_e23485);
        let assign21310_e23488: f64 = (assign21310_e23486 * var_tmp);
        (assign21310_e23488, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign21310_e23482 * var_terfc_dn6)))) * var_tmp) + (assign21310_e23486 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign21310_e23482 * var_terfc_dn7)))) * var_tmp) + (assign21310_e23486 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign21310_e23482 * var_terfc_dn8)))) * var_tmp) + (assign21310_e23486 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign21310_e23482 * var_terfc_dn9)))) * var_tmp) + (assign21310_e23486 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign21310_e23490;
        var_erfcpos_dn6 = assign21310_e23490_d_n6;
        var_erfcpos_dn7 = assign21310_e23490_d_n7;
        var_erfcpos_dn8 = assign21310_e23490_d_n8;
        var_erfcpos_dn9 = assign21310_e23490_d_n9;

        let assign21320_e23493: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard400 = assign21320_e23493;

        let (assign21330_e23507, assign21330_e23507_d_n6, assign21330_e23507_d_n7, assign21330_e23507_d_n8, assign21330_e23507_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) && (var_guard400 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign21330_e23507;
        var_erfctimesexpmtat_dn6 = assign21330_e23507_d_n6;
        var_erfctimesexpmtat_dn7 = assign21330_e23507_d_n7;
        var_erfctimesexpmtat_dn8 = assign21330_e23507_d_n8;
        var_erfctimesexpmtat_dn9 = assign21330_e23507_d_n9;

        let assign21340_e23510: f64 = (-230.25850929940458);
        let assign21340_e23511: f64 = if var_mtat > assign21340_e23510 { 1.0 } else { 0.0 };
        var_guard401 = assign21340_e23511;

        let (assign21350_e23529, assign21350_e23529_d_n6, assign21350_e23529_d_n7, assign21350_e23529_d_n8, assign21350_e23529_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) && (var_guard400 == 0.0)) && (var_guard401 != 0.0)) {
        let assign21350_e23527: f64 = (var_mtat).exp();
        (assign21350_e23527, (assign21350_e23527 * var_mtat_dn6), (assign21350_e23527 * var_mtat_dn7), (assign21350_e23527 * var_mtat_dn8), (assign21350_e23527 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign21350_e23529;
        var_tmp_dn6 = assign21350_e23529_d_n6;
        var_tmp_dn7 = assign21350_e23529_d_n7;
        var_tmp_dn8 = assign21350_e23529_d_n8;
        var_tmp_dn9 = assign21350_e23529_d_n9;

        let (assign21360_e23572, assign21360_e23572_d_n6, assign21360_e23572_d_n7, assign21360_e23572_d_n8, assign21360_e23572_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) && (var_guard400 == 0.0)) && (var_guard401 == 0.0)) {
        let assign21360_e23548: f64 = (-230.25850929940458);
        let assign21360_e23550: f64 = (assign21360_e23548 - var_mtat);
        let assign21360_e23554: f64 = (-230.25850929940458);
        let assign21360_e23556: f64 = (assign21360_e23554 - var_mtat);
        let assign21360_e23559: f64 = (-230.25850929940458);
        let assign21360_e23561: f64 = (assign21360_e23559 - var_mtat);
        let assign21360_e23563: f64 = (assign21360_e23561 * 0.3333333333333333);
        let assign21360_e23564: f64 = (1.0 + assign21360_e23563);
        let assign21360_e23565: f64 = (assign21360_e23556 * assign21360_e23564);
        let assign21360_e23566: f64 = (0.5 * assign21360_e23565);
        let assign21360_e23567: f64 = (1.0 + assign21360_e23566);
        let assign21360_e23568: f64 = (assign21360_e23550 * assign21360_e23567);
        let assign21360_e23569: f64 = (1.0 + assign21360_e23568);
        let assign21360_e23570: f64 = (1e-100 / assign21360_e23569);
        (assign21360_e23570, (-((1e-100 * (((-var_mtat_dn6) * assign21360_e23567) + (assign21360_e23550 * (0.5 * (((-var_mtat_dn6) * assign21360_e23564) + (assign21360_e23556 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign21360_e23569 * assign21360_e23569))), (-((1e-100 * (((-var_mtat_dn7) * assign21360_e23567) + (assign21360_e23550 * (0.5 * (((-var_mtat_dn7) * assign21360_e23564) + (assign21360_e23556 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign21360_e23569 * assign21360_e23569))), (-((1e-100 * (((-var_mtat_dn8) * assign21360_e23567) + (assign21360_e23550 * (0.5 * (((-var_mtat_dn8) * assign21360_e23564) + (assign21360_e23556 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign21360_e23569 * assign21360_e23569))), (-((1e-100 * (((-var_mtat_dn9) * assign21360_e23567) + (assign21360_e23550 * (0.5 * (((-var_mtat_dn9) * assign21360_e23564) + (assign21360_e23556 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign21360_e23569 * assign21360_e23569))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign21360_e23572;
        var_tmp_dn6 = assign21360_e23572_d_n6;
        var_tmp_dn7 = assign21360_e23572_d_n7;
        var_tmp_dn8 = assign21360_e23572_d_n8;
        var_tmp_dn9 = assign21360_e23572_d_n9;

        let (assign21370_e23591, assign21370_e23591_d_n6, assign21370_e23591_d_n7, assign21370_e23591_d_n8, assign21370_e23591_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) && (var_guard400 == 0.0)) {
        let assign21370_e23587: f64 = (2.0 * var_tmp);
        let assign21370_e23589: f64 = (assign21370_e23587 - var_erfcpos);
        (assign21370_e23589, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign21370_e23591;
        var_erfctimesexpmtat_dn6 = assign21370_e23591_d_n6;
        var_erfctimesexpmtat_dn7 = assign21370_e23591_d_n7;
        var_erfctimesexpmtat_dn8 = assign21370_e23591_d_n8;
        var_erfctimesexpmtat_dn9 = assign21370_e23591_d_n9;

        let (assign21380_e23611, assign21380_e23611_d_n6, assign21380_e23611_d_n7, assign21380_e23611_d_n8, assign21380_e23611_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) {
        let assign21380_e23603: f64 = (1.772453850905516 * 0.5);
        let assign21380_e23606: f64 = (var_atatgat * var_erfctimesexpmtat);
        let assign21380_e23608: f64 = (assign21380_e23606 / var_ktat);
        let assign21380_e23609: f64 = (assign21380_e23603 * assign21380_e23608);
        (assign21380_e23609, (assign21380_e23603 * ((((var_atatgat * var_erfctimesexpmtat_dn6) * var_ktat) - (assign21380_e23606 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign21380_e23603 * ((((var_atatgat * var_erfctimesexpmtat_dn7) * var_ktat) - (assign21380_e23606 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign21380_e23603 * ((((var_atatgat * var_erfctimesexpmtat_dn8) * var_ktat) - (assign21380_e23606 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign21380_e23603 * ((((var_atatgat * var_erfctimesexpmtat_dn9) * var_ktat) - (assign21380_e23606 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign21380_e23611;
        var_gammamax_dn6 = assign21380_e23611_d_n6;
        var_gammamax_dn7 = assign21380_e23611_d_n7;
        var_gammamax_dn8 = assign21380_e23611_d_n8;
        var_gammamax_dn9 = assign21380_e23611_d_n9;

        let (assign21390_e23629, assign21390_e23629_d_n6, assign21390_e23629_d_n7, assign21390_e23629_d_n8, assign21390_e23629_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard396 == 0.0)) {
        let assign21390_e23624: f64 = (var_asrh * var_gammamax);
        let assign21390_e23626: f64 = (assign21390_e23624 * var_wtat);
        let assign21390_e23627: f64 = (p.p864 * assign21390_e23626);
        (assign21390_e23627, (p.p864 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign21390_e23624 * var_wtat_dn6))), (p.p864 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign21390_e23624 * var_wtat_dn7))), (p.p864 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign21390_e23624 * var_wtat_dn8))), (p.p864 * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign21390_e23624 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign21390_e23629;
        var_itat_dn6 = assign21390_e23629_d_n6;
        var_itat_dn7 = assign21390_e23629_d_n7;
        var_itat_dn8 = assign21390_e23629_d_n8;
        var_itat_dn9 = assign21390_e23629_d_n9;

        let assign21400_e23632: f64 = if p.p870 == 0.0 { 1.0 } else { 0.0 };
        var_guard402 = assign21400_e23632;

        let (assign21410_e23643, assign21410_e23643_d_n6, assign21410_e23643_d_n7, assign21410_e23643_d_n8, assign21410_e23643_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard402 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign21410_e23643;
        var_ibbt_dn6 = assign21410_e23643_d_n6;
        var_ibbt_dn7 = assign21410_e23643_d_n7;
        var_ibbt_dn8 = assign21410_e23643_d_n8;
        var_ibbt_dn9 = assign21410_e23643_d_n9;

        let assign21420_e23646: f64 = if p.p850 == 0.5 { 1.0 } else { 0.0 };
        var_guard403 = assign21420_e23646;

        let (assign21430_e23665, assign21430_e23665_d_n6, assign21430_e23665_d_n7, assign21430_e23665_d_n8, assign21430_e23665_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard402 == 0.0)) && (var_guard403 != 0.0)) {
        let assign21430_e23660: f64 = (p.p847 - var_vbbt);
        let assign21430_e23662: f64 = (assign21430_e23660 * var_vbirgatinv);
        let assign21430_e23663: f64 = (assign21430_e23662).sqrt();
        (assign21430_e23663, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign21430_e23665;
        var_tmp_dn6 = assign21430_e23665_d_n6;
        var_tmp_dn7 = assign21430_e23665_d_n7;
        var_tmp_dn8 = assign21430_e23665_d_n8;
        var_tmp_dn9 = assign21430_e23665_d_n9;

        let (assign21440_e23686, assign21440_e23686_d_n6, assign21440_e23686_d_n7, assign21440_e23686_d_n8, assign21440_e23686_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard402 == 0.0)) && (var_guard403 == 0.0)) {
        let assign21440_e23680: f64 = (p.p847 - var_vbbt);
        let assign21440_e23682: f64 = (assign21440_e23680 * var_vbirgatinv);
        let assign21440_e23684: f64 = (assign21440_e23682).powf(p.p850);
        (assign21440_e23684, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign21440_e23686;
        var_tmp_dn6 = assign21440_e23686_d_n6;
        var_tmp_dn7 = assign21440_e23686_d_n7;
        var_tmp_dn8 = assign21440_e23686_d_n8;
        var_tmp_dn9 = assign21440_e23686_d_n9;

        let (assign21450_e23706, assign21450_e23706_d_n6, assign21450_e23706_d_n7, assign21450_e23706_d_n8, assign21450_e23706_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard402 == 0.0)) {
        let assign21450_e23699: f64 = (p.p847 - var_vbbt);
        let assign21450_e23701: f64 = (assign21450_e23699 * var_wdepnulrinvgat);
        let assign21450_e23703: f64 = (assign21450_e23701 / var_tmp);
        let assign21450_e23704: f64 = (var_one_over_one_minus_pgat * assign21450_e23703);
        (assign21450_e23704, (var_one_over_one_minus_pgat * (-((assign21450_e23701 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign21450_e23701 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign21450_e23701 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign21450_e23701 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign21450_e23706;
        var_fmaxr_dn6 = assign21450_e23706_d_n6;
        var_fmaxr_dn7 = assign21450_e23706_d_n7;
        var_fmaxr_dn8 = assign21450_e23706_d_n8;
        var_fmaxr_dn9 = assign21450_e23706_d_n9;

        let assign21460_e23708: f64 = (-var_fbbtgat);
        let assign21460_e23710: f64 = (assign21460_e23708 / var_fmaxr);
        let assign21460_e23711: f64 = (assign21460_e23710).abs();
        let assign21460_e23713: f64 = if assign21460_e23711 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard404 = assign21460_e23713;

        let (assign21470_e23731, assign21470_e23731_d_n6, assign21470_e23731_d_n7, assign21470_e23731_d_n8, assign21470_e23731_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard402 == 0.0)) && (var_guard404 != 0.0)) {
        let assign21470_e23726: f64 = (-var_fbbtgat);
        let assign21470_e23728: f64 = (assign21470_e23726 / var_fmaxr);
        let assign21470_e23729: f64 = (assign21470_e23728).exp();
        (assign21470_e23729, (assign21470_e23729 * ((((-var_fbbtgat_dn6) * var_fmaxr) - (assign21470_e23726 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign21470_e23729 * ((((-var_fbbtgat_dn7) * var_fmaxr) - (assign21470_e23726 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign21470_e23729 * ((((-var_fbbtgat_dn8) * var_fmaxr) - (assign21470_e23726 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))), (assign21470_e23729 * ((((-var_fbbtgat_dn9) * var_fmaxr) - (assign21470_e23726 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign21470_e23731;
        var_tmp_dn6 = assign21470_e23731_d_n6;
        var_tmp_dn7 = assign21470_e23731_d_n7;
        var_tmp_dn8 = assign21470_e23731_d_n8;
        var_tmp_dn9 = assign21470_e23731_d_n9;

        let assign21480_e23733: f64 = (-var_fbbtgat);
        let assign21480_e23735: f64 = (assign21480_e23733 / var_fmaxr);
        let assign21480_e23737: f64 = if assign21480_e23735 < 0.0 { 1.0 } else { 0.0 };
        var_guard405 = assign21480_e23737;

        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfcpos_dn9_slot = var_erfcpos_dn9;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_erfctimesexpmtat_dn9_slot = var_erfctimesexpmtat_dn9;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_fmaxr_dn9_slot = var_fmaxr_dn9;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_gammamax_dn9_slot = var_gammamax_dn9;
        *var_guard397_slot = var_guard397;
        *var_guard398_slot = var_guard398;
        *var_guard399_slot = var_guard399;
        *var_guard400_slot = var_guard400;
        *var_guard401_slot = var_guard401;
        *var_guard402_slot = var_guard402;
        *var_guard403_slot = var_guard403;
        *var_guard404_slot = var_guard404;
        *var_guard405_slot = var_guard405;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ktat_dn9_slot = var_ktat_dn9;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_ltat_dn9_slot = var_ltat_dn9;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_mtat_dn9_slot = var_mtat_dn9;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_sqrtumax_dn9_slot = var_sqrtumax_dn9;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_terfc_dn9_slot = var_terfc_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_umax_slot = var_umax;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umax_dn9_slot = var_umax_dn9;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_umaxpoweronepointfive_dn9_slot = var_umaxpoweronepointfive_dn9;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wgamma_dn9_slot = var_wgamma_dn9;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_wtat_dn9_slot = var_wtat_dn9;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_xerfc_dn9_slot = var_xerfc_dn9;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
        *var_ysq_dn9_slot = var_ysq_dn9;
    }

    pub(super) fn stamp_transient_block_39(
        p: &Parameters,
        var_absource_i: f64,
        var_alphaav: f64,
        var_exp_vmax_over_phitd_s: f64,
        var_fbbtgat: f64,
        var_fbbtgat_dn6: f64,
        var_fbbtgat_dn7: f64,
        var_fbbtgat_dn8: f64,
        var_fbbtgat_dn9: f64,
        var_fmaxr: f64,
        var_fmaxr_dn6: f64,
        var_fmaxr_dn7: f64,
        var_fmaxr_dn8: f64,
        var_fmaxr_dn9: f64,
        var_fstopgat: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard392: f64,
        var_guard402: f64,
        var_guard404: f64,
        var_guard405: f64,
        var_id__blk212: f64,
        var_ijunsti: f64,
        var_ijunsti_dn6: f64,
        var_ijunsti_dn7: f64,
        var_ijunsti_dn8: f64,
        var_ijunsti_dn9: f64,
        var_isrh: f64,
        var_isrh_dn6: f64,
        var_isrh_dn7: f64,
        var_isrh_dn8: f64,
        var_isrh_dn9: f64,
        var_itat: f64,
        var_itat_dn6: f64,
        var_itat_dn7: f64,
        var_itat_dn8: f64,
        var_itat_dn9: f64,
        var_lgsource_i: f64,
        var_lssource_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_slopegat: f64,
        var_slopegat_dn6: f64,
        var_slopegat_dn7: f64,
        var_slopegat_dn8: f64,
        var_slopegat_dn9: f64,
        var_v3: f64,
        var_v4: f64,
        var_vbbtlim_s: f64,
        var_vbimin_s: f64,
        var_vbrinvgat: f64,
        var_vbrinvgat_dn6: f64,
        var_vbrinvgat_dn7: f64,
        var_vbrinvgat_dn8: f64,
        var_vbrinvgat_dn9: f64,
        var_vmax_s: f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_guard406_slot: &mut f64,
        var_guard407_slot: &mut f64,
        var_guard408_slot: &mut f64,
        var_guard409_slot: &mut f64,
        var_guard410_slot: &mut f64,
        var_guard411_slot: &mut f64,
        var_guard412_slot: &mut f64,
        var_guard413_slot: &mut f64,
        var_guard414_slot: &mut f64,
        var_i3_slot: &mut f64,
        var_i3_dn6_slot: &mut f64,
        var_i3_dn7_slot: &mut f64,
        var_i3_dn8_slot: &mut f64,
        var_i3_dn9_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijunbot_dn9_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_ijungat_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fbreakdown_dn9: f64 = *var_fbreakdown_dn9_slot;
        let mut var_guard406: f64 = *var_guard406_slot;
        let mut var_guard407: f64 = *var_guard407_slot;
        let mut var_guard408: f64 = *var_guard408_slot;
        let mut var_guard409: f64 = *var_guard409_slot;
        let mut var_guard410: f64 = *var_guard410_slot;
        let mut var_guard411: f64 = *var_guard411_slot;
        let mut var_guard412: f64 = *var_guard412_slot;
        let mut var_guard413: f64 = *var_guard413_slot;
        let mut var_guard414: f64 = *var_guard414_slot;
        let mut var_i3: f64 = *var_i3_slot;
        let mut var_i3_dn6: f64 = *var_i3_dn6_slot;
        let mut var_i3_dn7: f64 = *var_i3_dn7_slot;
        let mut var_i3_dn8: f64 = *var_i3_dn8_slot;
        let mut var_i3_dn9: f64 = *var_i3_dn9_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijunbot_dn9: f64 = *var_ijunbot_dn9_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_ijungat_dn9: f64 = *var_ijungat_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign21490_e23788, assign21490_e23788_d_n6, assign21490_e23788_d_n7, assign21490_e23788_d_n8, assign21490_e23788_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard402 == 0.0)) && (var_guard404 == 0.0)) && (var_guard405 != 0.0)) {
        let assign21490_e23755: f64 = (-230.25850929940458);
        let assign21490_e23757: f64 = (-var_fbbtgat);
        let assign21490_e23759: f64 = (assign21490_e23757 / var_fmaxr);
        let assign21490_e23760: f64 = (assign21490_e23755 - assign21490_e23759);
        let assign21490_e23764: f64 = (-230.25850929940458);
        let assign21490_e23766: f64 = (-var_fbbtgat);
        let assign21490_e23768: f64 = (assign21490_e23766 / var_fmaxr);
        let assign21490_e23769: f64 = (assign21490_e23764 - assign21490_e23768);
        let assign21490_e23772: f64 = (-230.25850929940458);
        let assign21490_e23774: f64 = (-var_fbbtgat);
        let assign21490_e23776: f64 = (assign21490_e23774 / var_fmaxr);
        let assign21490_e23777: f64 = (assign21490_e23772 - assign21490_e23776);
        let assign21490_e23779: f64 = (assign21490_e23777 * 0.3333333333333333);
        let assign21490_e23780: f64 = (1.0 + assign21490_e23779);
        let assign21490_e23781: f64 = (assign21490_e23769 * assign21490_e23780);
        let assign21490_e23782: f64 = (0.5 * assign21490_e23781);
        let assign21490_e23783: f64 = (1.0 + assign21490_e23782);
        let assign21490_e23784: f64 = (assign21490_e23760 * assign21490_e23783);
        let assign21490_e23785: f64 = (1.0 + assign21490_e23784);
        let assign21490_e23786: f64 = (1e-100 / assign21490_e23785);
        (assign21490_e23786, (-((1e-100 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign21490_e23757 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign21490_e23783) + (assign21490_e23760 * (0.5 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign21490_e23766 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign21490_e23780) + (assign21490_e23769 * ((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign21490_e23774 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign21490_e23785 * assign21490_e23785))), (-((1e-100 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign21490_e23757 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign21490_e23783) + (assign21490_e23760 * (0.5 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign21490_e23766 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign21490_e23780) + (assign21490_e23769 * ((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign21490_e23774 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign21490_e23785 * assign21490_e23785))), (-((1e-100 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign21490_e23757 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign21490_e23783) + (assign21490_e23760 * (0.5 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign21490_e23766 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign21490_e23780) + (assign21490_e23769 * ((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign21490_e23774 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign21490_e23785 * assign21490_e23785))), (-((1e-100 * (((-((((-var_fbbtgat_dn9) * var_fmaxr) - (assign21490_e23757 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign21490_e23783) + (assign21490_e23760 * (0.5 * (((-((((-var_fbbtgat_dn9) * var_fmaxr) - (assign21490_e23766 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign21490_e23780) + (assign21490_e23769 * ((-((((-var_fbbtgat_dn9) * var_fmaxr) - (assign21490_e23774 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign21490_e23785 * assign21490_e23785))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign21490_e23788;
        var_tmp_dn6 = assign21490_e23788_d_n6;
        var_tmp_dn7 = assign21490_e23788_d_n7;
        var_tmp_dn8 = assign21490_e23788_d_n8;
        var_tmp_dn9 = assign21490_e23788_d_n9;

        let (assign21500_e23837, assign21500_e23837_d_n6, assign21500_e23837_d_n7, assign21500_e23837_d_n8, assign21500_e23837_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard402 == 0.0)) && (var_guard404 == 0.0)) && (var_guard405 == 0.0)) {
        let assign21500_e23807: f64 = (-var_fbbtgat);
        let assign21500_e23809: f64 = (assign21500_e23807 / var_fmaxr);
        let assign21500_e23811: f64 = (assign21500_e23809 - 230.25850929940458);
        let assign21500_e23815: f64 = (-var_fbbtgat);
        let assign21500_e23817: f64 = (assign21500_e23815 / var_fmaxr);
        let assign21500_e23819: f64 = (assign21500_e23817 - 230.25850929940458);
        let assign21500_e23822: f64 = (-var_fbbtgat);
        let assign21500_e23824: f64 = (assign21500_e23822 / var_fmaxr);
        let assign21500_e23826: f64 = (assign21500_e23824 - 230.25850929940458);
        let assign21500_e23828: f64 = (assign21500_e23826 * 0.3333333333333333);
        let assign21500_e23829: f64 = (1.0 + assign21500_e23828);
        let assign21500_e23830: f64 = (assign21500_e23819 * assign21500_e23829);
        let assign21500_e23831: f64 = (0.5 * assign21500_e23830);
        let assign21500_e23832: f64 = (1.0 + assign21500_e23831);
        let assign21500_e23833: f64 = (assign21500_e23811 * assign21500_e23832);
        let assign21500_e23834: f64 = (1.0 + assign21500_e23833);
        let assign21500_e23835: f64 = (1e100 * assign21500_e23834);
        (assign21500_e23835, (1e100 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign21500_e23807 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign21500_e23832) + (assign21500_e23811 * (0.5 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign21500_e23815 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign21500_e23829) + (assign21500_e23819 * (((((-var_fbbtgat_dn6) * var_fmaxr) - (assign21500_e23822 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign21500_e23807 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign21500_e23832) + (assign21500_e23811 * (0.5 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign21500_e23815 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign21500_e23829) + (assign21500_e23819 * (((((-var_fbbtgat_dn7) * var_fmaxr) - (assign21500_e23822 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign21500_e23807 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign21500_e23832) + (assign21500_e23811 * (0.5 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign21500_e23815 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign21500_e23829) + (assign21500_e23819 * (((((-var_fbbtgat_dn8) * var_fmaxr) - (assign21500_e23822 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn9) * var_fmaxr) - (assign21500_e23807 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign21500_e23832) + (assign21500_e23811 * (0.5 * ((((((-var_fbbtgat_dn9) * var_fmaxr) - (assign21500_e23815 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign21500_e23829) + (assign21500_e23819 * (((((-var_fbbtgat_dn9) * var_fmaxr) - (assign21500_e23822 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign21500_e23837;
        var_tmp_dn6 = assign21500_e23837_d_n6;
        var_tmp_dn7 = assign21500_e23837_d_n7;
        var_tmp_dn8 = assign21500_e23837_d_n8;
        var_tmp_dn9 = assign21500_e23837_d_n9;

        let (assign21510_e23857, assign21510_e23857_d_n6, assign21510_e23857_d_n7, assign21510_e23857_d_n8, assign21510_e23857_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard402 == 0.0)) {
        let assign21510_e23850: f64 = (var_v3 * var_fmaxr);
        let assign21510_e23852: f64 = (assign21510_e23850 * var_fmaxr);
        let assign21510_e23854: f64 = (assign21510_e23852 * var_tmp);
        let assign21510_e23855: f64 = (p.p870 * assign21510_e23854);
        (assign21510_e23855, (p.p870 * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign21510_e23850 * var_fmaxr_dn6)) * var_tmp) + (assign21510_e23852 * var_tmp_dn6))), (p.p870 * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign21510_e23850 * var_fmaxr_dn7)) * var_tmp) + (assign21510_e23852 * var_tmp_dn7))), (p.p870 * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign21510_e23850 * var_fmaxr_dn8)) * var_tmp) + (assign21510_e23852 * var_tmp_dn8))), (p.p870 * (((((var_v3 * var_fmaxr_dn9) * var_fmaxr) + (assign21510_e23850 * var_fmaxr_dn9)) * var_tmp) + (assign21510_e23852 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign21510_e23857;
        var_ibbt_dn6 = assign21510_e23857_d_n6;
        var_ibbt_dn7 = assign21510_e23857_d_n7;
        var_ibbt_dn8 = assign21510_e23857_d_n8;
        var_ibbt_dn9 = assign21510_e23857_d_n9;

        let assign21520_e23860: f64 = if p.p879 > 1000.0 { 1.0 } else { 0.0 };
        var_guard406 = assign21520_e23860;

        let (assign21530_e23871, assign21530_e23871_d_n6, assign21530_e23871_d_n7, assign21530_e23871_d_n8, assign21530_e23871_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard406 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign21530_e23871;
        var_fbreakdown_dn6 = assign21530_e23871_d_n6;
        var_fbreakdown_dn7 = assign21530_e23871_d_n7;
        var_fbreakdown_dn8 = assign21530_e23871_d_n8;
        var_fbreakdown_dn9 = assign21530_e23871_d_n9;

        let assign21540_e23874: f64 = (-var_alphaav);
        let assign21540_e23876: f64 = (assign21540_e23874 * p.p879);
        let assign21540_e23877: f64 = if var_vav > assign21540_e23876 { 1.0 } else { 0.0 };
        var_guard407 = assign21540_e23877;

        let assign21550_e23880: f64 = if p.p882 == 4.0 { 1.0 } else { 0.0 };
        var_guard408 = assign21550_e23880;

        let (assign21560_e23910, assign21560_e23910_d_n6, assign21560_e23910_d_n7, assign21560_e23910_d_n8, assign21560_e23910_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard406 == 0.0)) && (var_guard407 != 0.0)) && (var_guard408 != 0.0)) {
        let assign21560_e23896: f64 = (var_vav * var_vbrinvgat);
        let assign21560_e23899: f64 = (var_vav * var_vbrinvgat);
        let assign21560_e23900: f64 = (assign21560_e23896 * assign21560_e23899);
        let assign21560_e23903: f64 = (var_vav * var_vbrinvgat);
        let assign21560_e23904: f64 = (assign21560_e23900 * assign21560_e23903);
        let assign21560_e23907: f64 = (var_vav * var_vbrinvgat);
        let assign21560_e23908: f64 = (assign21560_e23904 * assign21560_e23907);
        (assign21560_e23908, (((((((var_vav * var_vbrinvgat_dn6) * assign21560_e23899) + (assign21560_e23896 * (var_vav * var_vbrinvgat_dn6))) * assign21560_e23903) + (assign21560_e23900 * (var_vav * var_vbrinvgat_dn6))) * assign21560_e23907) + (assign21560_e23904 * (var_vav * var_vbrinvgat_dn6))), (((((((var_vav * var_vbrinvgat_dn7) * assign21560_e23899) + (assign21560_e23896 * (var_vav * var_vbrinvgat_dn7))) * assign21560_e23903) + (assign21560_e23900 * (var_vav * var_vbrinvgat_dn7))) * assign21560_e23907) + (assign21560_e23904 * (var_vav * var_vbrinvgat_dn7))), (((((((var_vav * var_vbrinvgat_dn8) * assign21560_e23899) + (assign21560_e23896 * (var_vav * var_vbrinvgat_dn8))) * assign21560_e23903) + (assign21560_e23900 * (var_vav * var_vbrinvgat_dn8))) * assign21560_e23907) + (assign21560_e23904 * (var_vav * var_vbrinvgat_dn8))), (((((((var_vav * var_vbrinvgat_dn9) * assign21560_e23899) + (assign21560_e23896 * (var_vav * var_vbrinvgat_dn9))) * assign21560_e23903) + (assign21560_e23900 * (var_vav * var_vbrinvgat_dn9))) * assign21560_e23907) + (assign21560_e23904 * (var_vav * var_vbrinvgat_dn9))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign21560_e23910;
        var_tmp_dn6 = assign21560_e23910_d_n6;
        var_tmp_dn7 = assign21560_e23910_d_n7;
        var_tmp_dn8 = assign21560_e23910_d_n8;
        var_tmp_dn9 = assign21560_e23910_d_n9;

        let (assign21570_e23932, assign21570_e23932_d_n6, assign21570_e23932_d_n7, assign21570_e23932_d_n8, assign21570_e23932_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard406 == 0.0)) && (var_guard407 != 0.0)) && (var_guard408 == 0.0)) {
        let assign21570_e23927: f64 = (var_vav * var_vbrinvgat);
        let assign21570_e23928: f64 = (assign21570_e23927).abs();
        let assign21570_e23930: f64 = (assign21570_e23928).powf(p.p882);
        (assign21570_e23930, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign21570_e23928).powf(p.p882 - 1.0) * if assign21570_e23927 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) })) } } else { (assign21570_e23930 * (p.p882 * (if assign21570_e23927 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) } / assign21570_e23928))) }, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign21570_e23928).powf(p.p882 - 1.0) * if assign21570_e23927 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) })) } } else { (assign21570_e23930 * (p.p882 * (if assign21570_e23927 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) } / assign21570_e23928))) }, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign21570_e23928).powf(p.p882 - 1.0) * if assign21570_e23927 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) })) } } else { (assign21570_e23930 * (p.p882 * (if assign21570_e23927 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) } / assign21570_e23928))) }, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign21570_e23928).powf(p.p882 - 1.0) * if assign21570_e23927 >= 0.0 { (var_vav * var_vbrinvgat_dn9) } else { (-(var_vav * var_vbrinvgat_dn9)) })) } } else { (assign21570_e23930 * (p.p882 * (if assign21570_e23927 >= 0.0 { (var_vav * var_vbrinvgat_dn9) } else { (-(var_vav * var_vbrinvgat_dn9)) } / assign21570_e23928))) },)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign21570_e23932;
        var_tmp_dn6 = assign21570_e23932_d_n6;
        var_tmp_dn7 = assign21570_e23932_d_n7;
        var_tmp_dn8 = assign21570_e23932_d_n8;
        var_tmp_dn9 = assign21570_e23932_d_n9;

        let (assign21580_e23950, assign21580_e23950_d_n6, assign21580_e23950_d_n7, assign21580_e23950_d_n8, assign21580_e23950_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard406 == 0.0)) && (var_guard407 != 0.0)) {
        let assign21580_e23947: f64 = (1.0 - var_tmp);
        let assign21580_e23948: f64 = (1.0 / assign21580_e23947);
        (assign21580_e23948, (-((-var_tmp_dn6) / (assign21580_e23947 * assign21580_e23947))), (-((-var_tmp_dn7) / (assign21580_e23947 * assign21580_e23947))), (-((-var_tmp_dn8) / (assign21580_e23947 * assign21580_e23947))), (-((-var_tmp_dn9) / (assign21580_e23947 * assign21580_e23947))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign21580_e23950;
        var_fbreakdown_dn6 = assign21580_e23950_d_n6;
        var_fbreakdown_dn7 = assign21580_e23950_d_n7;
        var_fbreakdown_dn8 = assign21580_e23950_d_n8;
        var_fbreakdown_dn9 = assign21580_e23950_d_n9;

        let (assign21590_e23973, assign21590_e23973_d_n6, assign21590_e23973_d_n7, assign21590_e23973_d_n8, assign21590_e23973_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) && (var_guard406 == 0.0)) && (var_guard407 == 0.0)) {
        let assign21590_e23967: f64 = (var_alphaav * p.p879);
        let assign21590_e23968: f64 = (var_vav + assign21590_e23967);
        let assign21590_e23970: f64 = (assign21590_e23968 * var_slopegat);
        let assign21590_e23971: f64 = (var_fstopgat + assign21590_e23970);
        (assign21590_e23971, (assign21590_e23968 * var_slopegat_dn6), (assign21590_e23968 * var_slopegat_dn7), (assign21590_e23968 * var_slopegat_dn8), (assign21590_e23968 * var_slopegat_dn9),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign21590_e23973;
        var_fbreakdown_dn6 = assign21590_e23973_d_n6;
        var_fbreakdown_dn7 = assign21590_e23973_d_n7;
        var_fbreakdown_dn8 = assign21590_e23973_d_n8;
        var_fbreakdown_dn9 = assign21590_e23973_d_n9;

        let (assign21600_e23992, assign21600_e23992_d_n6, assign21600_e23992_d_n7, assign21600_e23992_d_n8, assign21600_e23992_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard392 == 0.0)) {
        let assign21600_e23983: f64 = (var_id__blk212 + var_isrh);
        let assign21600_e23985: f64 = (assign21600_e23983 + var_itat);
        let assign21600_e23987: f64 = (assign21600_e23985 + var_ibbt);
        let assign21600_e23988: f64 = (p.p29 * assign21600_e23987);
        let assign21600_e23990: f64 = (assign21600_e23988 * var_fbreakdown);
        (assign21600_e23990, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign21600_e23988 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign21600_e23988 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign21600_e23988 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign21600_e23988 * var_fbreakdown_dn9)),)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign21600_e23992;
        var_ijungat_dn6 = assign21600_e23992_d_n6;
        var_ijungat_dn7 = assign21600_e23992_d_n7;
        var_ijungat_dn8 = assign21600_e23992_d_n8;
        var_ijungat_dn9 = assign21600_e23992_d_n9;

        let (assign21610_e24008, assign21610_e24008_d_n6, assign21610_e24008_d_n7, assign21610_e24008_d_n8, assign21610_e24008_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign21610_e23998: f64 = (var_absource_i * var_ijunbot);
        let assign21610_e24001: f64 = (var_lssource_i * var_ijunsti);
        let assign21610_e24002: f64 = (assign21610_e23998 + assign21610_e24001);
        let assign21610_e24005: f64 = (var_lgsource_i * var_ijungat);
        let assign21610_e24006: f64 = (assign21610_e24002 + assign21610_e24005);
        (assign21610_e24006, (((var_absource_i * var_ijunbot_dn6) + (var_lssource_i * var_ijunsti_dn6)) + (var_lgsource_i * var_ijungat_dn6)), (((var_absource_i * var_ijunbot_dn7) + (var_lssource_i * var_ijunsti_dn7)) + (var_lgsource_i * var_ijungat_dn7)), (((var_absource_i * var_ijunbot_dn8) + (var_lssource_i * var_ijunsti_dn8)) + (var_lgsource_i * var_ijungat_dn8)), (((var_absource_i * var_ijunbot_dn9) + (var_lssource_i * var_ijunsti_dn9)) + (var_lgsource_i * var_ijungat_dn9)),)
    } else {
        (var_i3, var_i3_dn6, var_i3_dn7, var_i3_dn8, var_i3_dn9,)
    }
};
        var_i3 = assign21610_e24008;
        var_i3_dn6 = assign21610_e24008_d_n6;
        var_i3_dn7 = assign21610_e24008_d_n7;
        var_i3_dn8 = assign21610_e24008_d_n8;
        var_i3_dn9 = assign21610_e24008_d_n9;

        let (assign21620_e24014,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign21620_e24014;

        let (assign21630_e24020,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign21630_e24020;

        let assign21640_e24032: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard409 = assign21640_e24032;

        let assign21720_e24118: f64 = if var_v4 < var_vmax_s { 1.0 } else { 0.0 };
        var_guard410 = assign21720_e24118;

        let assign21730_e24120: f64 = (-0.5);
        let assign21730_e24123: f64 = (var_v4 * var_phitdinv);
        let assign21730_e24124: f64 = (assign21730_e24120 * assign21730_e24123);
        let assign21730_e24125: f64 = (assign21730_e24124).abs();
        let assign21730_e24127: f64 = if assign21730_e24125 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard411 = assign21730_e24127;

        let (assign21740_e24145,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard409 != 0.0)) && (var_guard410 != 0.0)) && (var_guard411 != 0.0)) {
        let assign21740_e24138: f64 = (-0.5);
        let assign21740_e24141: f64 = (var_v4 * var_phitdinv);
        let assign21740_e24142: f64 = (assign21740_e24138 * assign21740_e24141);
        let assign21740_e24143: f64 = (assign21740_e24142).exp();
        (assign21740_e24143,)
    } else {
        (var_z,)
    }
};
        var_z = assign21740_e24145;

        let assign21750_e24147: f64 = (-0.5);
        let assign21750_e24150: f64 = (var_v4 * var_phitdinv);
        let assign21750_e24151: f64 = (assign21750_e24147 * assign21750_e24150);
        let assign21750_e24153: f64 = if assign21750_e24151 < 0.0 { 1.0 } else { 0.0 };
        var_guard412 = assign21750_e24153;

        let (assign21760_e24208,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard409 != 0.0)) && (var_guard410 != 0.0)) && (var_guard411 == 0.0)) && (var_guard412 != 0.0)) {
        let assign21760_e24169: f64 = (-230.25850929940458);
        let assign21760_e24171: f64 = (-0.5);
        let assign21760_e24174: f64 = (var_v4 * var_phitdinv);
        let assign21760_e24175: f64 = (assign21760_e24171 * assign21760_e24174);
        let assign21760_e24176: f64 = (assign21760_e24169 - assign21760_e24175);
        let assign21760_e24180: f64 = (-230.25850929940458);
        let assign21760_e24182: f64 = (-0.5);
        let assign21760_e24185: f64 = (var_v4 * var_phitdinv);
        let assign21760_e24186: f64 = (assign21760_e24182 * assign21760_e24185);
        let assign21760_e24187: f64 = (assign21760_e24180 - assign21760_e24186);
        let assign21760_e24190: f64 = (-230.25850929940458);
        let assign21760_e24192: f64 = (-0.5);
        let assign21760_e24195: f64 = (var_v4 * var_phitdinv);
        let assign21760_e24196: f64 = (assign21760_e24192 * assign21760_e24195);
        let assign21760_e24197: f64 = (assign21760_e24190 - assign21760_e24196);
        let assign21760_e24199: f64 = (assign21760_e24197 * 0.3333333333333333);
        let assign21760_e24200: f64 = (1.0 + assign21760_e24199);
        let assign21760_e24201: f64 = (assign21760_e24187 * assign21760_e24200);
        let assign21760_e24202: f64 = (0.5 * assign21760_e24201);
        let assign21760_e24203: f64 = (1.0 + assign21760_e24202);
        let assign21760_e24204: f64 = (assign21760_e24176 * assign21760_e24203);
        let assign21760_e24205: f64 = (1.0 + assign21760_e24204);
        let assign21760_e24206: f64 = (1e-100 / assign21760_e24205);
        (assign21760_e24206,)
    } else {
        (var_z,)
    }
};
        var_z = assign21760_e24208;

        let (assign21770_e24261,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard409 != 0.0)) && (var_guard410 != 0.0)) && (var_guard411 == 0.0)) && (var_guard412 == 0.0)) {
        let assign21770_e24225: f64 = (-0.5);
        let assign21770_e24228: f64 = (var_v4 * var_phitdinv);
        let assign21770_e24229: f64 = (assign21770_e24225 * assign21770_e24228);
        let assign21770_e24231: f64 = (assign21770_e24229 - 230.25850929940458);
        let assign21770_e24235: f64 = (-0.5);
        let assign21770_e24238: f64 = (var_v4 * var_phitdinv);
        let assign21770_e24239: f64 = (assign21770_e24235 * assign21770_e24238);
        let assign21770_e24241: f64 = (assign21770_e24239 - 230.25850929940458);
        let assign21770_e24244: f64 = (-0.5);
        let assign21770_e24247: f64 = (var_v4 * var_phitdinv);
        let assign21770_e24248: f64 = (assign21770_e24244 * assign21770_e24247);
        let assign21770_e24250: f64 = (assign21770_e24248 - 230.25850929940458);
        let assign21770_e24252: f64 = (assign21770_e24250 * 0.3333333333333333);
        let assign21770_e24253: f64 = (1.0 + assign21770_e24252);
        let assign21770_e24254: f64 = (assign21770_e24241 * assign21770_e24253);
        let assign21770_e24255: f64 = (0.5 * assign21770_e24254);
        let assign21770_e24256: f64 = (1.0 + assign21770_e24255);
        let assign21770_e24257: f64 = (assign21770_e24231 * assign21770_e24256);
        let assign21770_e24258: f64 = (1.0 + assign21770_e24257);
        let assign21770_e24259: f64 = (1e100 * assign21770_e24258);
        (assign21770_e24259,)
    } else {
        (var_z,)
    }
};
        var_z = assign21770_e24261;

        let (assign21780_e24273,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard409 != 0.0)) && (var_guard410 != 0.0)) {
        let assign21780_e24271: f64 = (1.0 / var_z);
        (assign21780_e24271,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign21780_e24273;

        let (assign21790_e24285,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard409 != 0.0)) && (var_guard410 != 0.0)) {
        let assign21790_e24283: f64 = (var_zinv * var_zinv);
        (assign21790_e24283,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign21790_e24285;

        let (assign21800_e24304,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard409 != 0.0)) && (var_guard410 == 0.0)) {
        let assign21800_e24297: f64 = (var_v4 - var_vmax_s);
        let assign21800_e24299: f64 = (assign21800_e24297 * var_phitdinv);
        let assign21800_e24300: f64 = (1.0 + assign21800_e24299);
        let assign21800_e24302: f64 = (assign21800_e24300 * var_exp_vmax_over_phitd_s);
        (assign21800_e24302,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign21800_e24304;

        let (assign21810_e24316,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard409 != 0.0)) && (var_guard410 == 0.0)) {
        let assign21810_e24314: f64 = (var_idmult).sqrt();
        (assign21810_e24314,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign21810_e24316;

        let (assign21820_e24329,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard409 != 0.0)) && (var_guard410 == 0.0)) {
        let assign21820_e24327: f64 = (1.0 / var_zinv);
        (assign21820_e24327,)
    } else {
        (var_z,)
    }
};
        var_z = assign21820_e24329;

        let (assign21830_e24339,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard409 != 0.0)) {
        let assign21830_e24337: f64 = (var_idmult - 1.0);
        (assign21830_e24337,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign21830_e24339;

        let assign21840_e24342: f64 = if var_v4 > 0.0 { 1.0 } else { 0.0 };
        var_guard413 = assign21840_e24342;

        let (assign21850_e24368,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard409 != 0.0)) && (var_guard413 != 0.0)) {
        let assign21850_e24354: f64 = (2.0 + var_z);
        let assign21850_e24357: f64 = (var_z + 1.0);
        let assign21850_e24360: f64 = (var_z + 3.0);
        let assign21850_e24361: f64 = (assign21850_e24357 * assign21850_e24360);
        let assign21850_e24362: f64 = (assign21850_e24361).sqrt();
        let assign21850_e24363: f64 = (assign21850_e24354 + assign21850_e24362);
        let assign21850_e24364: f64 = (assign21850_e24363).ln();
        let assign21850_e24365: f64 = (var_phitd * assign21850_e24364);
        let assign21850_e24366: f64 = (2.0 * assign21850_e24365);
        (assign21850_e24366,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign21850_e24368;

        let (assign21860_e24402,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard409 != 0.0)) && (var_guard413 == 0.0)) {
        let assign21860_e24378: f64 = (-var_v4);
        let assign21860_e24383: f64 = (2.0 * var_zinv);
        let assign21860_e24385: f64 = (assign21860_e24383 + 1.0);
        let assign21860_e24388: f64 = (1.0 + var_zinv);
        let assign21860_e24392: f64 = (3.0 * var_zinv);
        let assign21860_e24393: f64 = (1.0 + assign21860_e24392);
        let assign21860_e24394: f64 = (assign21860_e24388 * assign21860_e24393);
        let assign21860_e24395: f64 = (assign21860_e24394).sqrt();
        let assign21860_e24396: f64 = (assign21860_e24385 + assign21860_e24395);
        let assign21860_e24397: f64 = (assign21860_e24396).ln();
        let assign21860_e24398: f64 = (var_phitd * assign21860_e24397);
        let assign21860_e24399: f64 = (2.0 * assign21860_e24398);
        let assign21860_e24400: f64 = (assign21860_e24378 + assign21860_e24399);
        (assign21860_e24400,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign21860_e24402;

        let (assign21870_e24412,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard409 != 0.0)) {
        let assign21870_e24410: f64 = (var_vbimin_s - var_two_psistar);
        (assign21870_e24410,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign21870_e24412;

        let (assign21880_e24439,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard409 != 0.0)) {
        let assign21880_e24421: f64 = (var_v4 + var_vjlim);
        let assign21880_e24424: f64 = (var_v4 - var_vjlim);
        let assign21880_e24427: f64 = (var_v4 - var_vjlim);
        let assign21880_e24428: f64 = (assign21880_e24424 * assign21880_e24427);
        let assign21880_e24431: f64 = (4.0 * var_phitd);
        let assign21880_e24433: f64 = (assign21880_e24431 * var_phitd);
        let assign21880_e24434: f64 = (assign21880_e24428 + assign21880_e24433);
        let assign21880_e24435: f64 = (assign21880_e24434).sqrt();
        let assign21880_e24436: f64 = (assign21880_e24421 - assign21880_e24435);
        let assign21880_e24437: f64 = (0.5 * assign21880_e24436);
        (assign21880_e24437,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign21880_e24439;

        let (assign21890_e24466,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard409 != 0.0)) {
        let assign21890_e24448: f64 = (var_v4 + var_vbbtlim_s);
        let assign21890_e24451: f64 = (var_v4 - var_vbbtlim_s);
        let assign21890_e24454: f64 = (var_v4 - var_vbbtlim_s);
        let assign21890_e24455: f64 = (assign21890_e24451 * assign21890_e24454);
        let assign21890_e24458: f64 = (4.0 * var_phitr);
        let assign21890_e24460: f64 = (assign21890_e24458 * var_phitr);
        let assign21890_e24461: f64 = (assign21890_e24455 + assign21890_e24460);
        let assign21890_e24462: f64 = (assign21890_e24461).sqrt();
        let assign21890_e24463: f64 = (assign21890_e24448 - assign21890_e24462);
        let assign21890_e24464: f64 = (0.5 * assign21890_e24463);
        (assign21890_e24464,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign21890_e24466;

        let (assign21900_e24493,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard409 != 0.0)) {
        let assign21900_e24475: f64 = var_v4;
        let assign21900_e24478: f64 = var_v4;
        let assign21900_e24481: f64 = var_v4;
        let assign21900_e24482: f64 = (assign21900_e24478 * assign21900_e24481);
        let assign21900_e24485: f64 = (4.0 * 1e-6);
        let assign21900_e24487: f64 = (assign21900_e24485 * 1e-6);
        let assign21900_e24488: f64 = (assign21900_e24482 + assign21900_e24487);
        let assign21900_e24489: f64 = (assign21900_e24488).sqrt();
        let assign21900_e24490: f64 = (assign21900_e24475 - assign21900_e24489);
        let assign21900_e24491: f64 = (0.5 * assign21900_e24490);
        (assign21900_e24491,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign21900_e24493;

        let assign21910_e24496: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard414 = assign21910_e24496;

        let (assign21920_e24504, assign21920_e24504_d_n6, assign21920_e24504_d_n7, assign21920_e24504_d_n8, assign21920_e24504_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign21920_e24504;
        var_ijunbot_dn6 = assign21920_e24504_d_n6;
        var_ijunbot_dn7 = assign21920_e24504_d_n7;
        var_ijunbot_dn8 = assign21920_e24504_d_n8;
        var_ijunbot_dn9 = assign21920_e24504_d_n9;

        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_guard406_slot = var_guard406;
        *var_guard407_slot = var_guard407;
        *var_guard408_slot = var_guard408;
        *var_guard409_slot = var_guard409;
        *var_guard410_slot = var_guard410;
        *var_guard411_slot = var_guard411;
        *var_guard412_slot = var_guard412;
        *var_guard413_slot = var_guard413;
        *var_guard414_slot = var_guard414;
        *var_i3_slot = var_i3;
        *var_i3_dn6_slot = var_i3_dn6;
        *var_i3_dn7_slot = var_i3_dn7;
        *var_i3_dn8_slot = var_i3_dn8;
        *var_i3_dn9_slot = var_i3_dn9;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_idmult_slot = var_idmult;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijunbot_dn9_slot = var_ijunbot_dn9;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_ijungat_dn9_slot = var_ijungat_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_two_psistar_slot = var_two_psistar;
        *var_vav_slot = var_vav;
        *var_vbbt_slot = var_vbbt;
        *var_vjlim_slot = var_vjlim;
        *var_vjsrh_slot = var_vjsrh;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_40(
        p: &Parameters,
        var_atatbot: f64,
        var_btatpartbot: f64,
        var_ftdbot: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard414: f64,
        var_idmult: f64,
        var_idsatbot: f64,
        var_one_minus_pbot: f64,
        var_one_over_one_minus_pbot: f64,
        var_perfc: f64,
        var_two_psistar: f64,
        var_vbibot: f64,
        var_vbirbotinv: f64,
        var_vjsrh: f64,
        var_wdepnulrbot: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_asrh_dn9_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_btat_dn9_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_guard415_slot: &mut f64,
        var_guard416_slot: &mut f64,
        var_guard417_slot: &mut f64,
        var_guard418_slot: &mut f64,
        var_guard419_slot: &mut f64,
        var_guard420_slot: &mut f64,
        var_guard421_slot: &mut f64,
        var_id__blk212_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ktat_dn9_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_ltat_dn9_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_mtat_dn9_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_sqrtumax_dn9_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_terfc_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_dn9_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umax_dn9_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxbeforelimiting_dn9_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_dn9_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wdep_dn9_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wgamma_dn9_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_wtat_dn9_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_xerfc_dn9_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
        var_ysq_dn9_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_asrh_dn9: f64 = *var_asrh_dn9_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_btat_dn9: f64 = *var_btat_dn9_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_guard415: f64 = *var_guard415_slot;
        let mut var_guard416: f64 = *var_guard416_slot;
        let mut var_guard417: f64 = *var_guard417_slot;
        let mut var_guard418: f64 = *var_guard418_slot;
        let mut var_guard419: f64 = *var_guard419_slot;
        let mut var_guard420: f64 = *var_guard420_slot;
        let mut var_guard421: f64 = *var_guard421_slot;
        let mut var_id__blk212: f64 = *var_id__blk212_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ktat_dn9: f64 = *var_ktat_dn9_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_ltat_dn9: f64 = *var_ltat_dn9_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_mtat_dn9: f64 = *var_mtat_dn9_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_sqrtumax_dn9: f64 = *var_sqrtumax_dn9_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_terfc_dn9: f64 = *var_terfc_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_twoatatoverthreebtat_dn9: f64 = *var_twoatatoverthreebtat_dn9_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umax_dn9: f64 = *var_umax_dn9_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxbeforelimiting_dn9: f64 = *var_umaxbeforelimiting_dn9_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_umaxpoweronepointfive_dn9: f64 = *var_umaxpoweronepointfive_dn9_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wdep_dn9: f64 = *var_wdep_dn9_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wgamma_dn9: f64 = *var_wgamma_dn9_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_wtat_dn9: f64 = *var_wtat_dn9_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_xerfc_dn9: f64 = *var_xerfc_dn9_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;
        let mut var_ysq_dn9: f64 = *var_ysq_dn9_slot;

        let (assign21930_e24515,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) {
        let assign21930_e24513: f64 = (var_idsatbot * var_idmult);
        (assign21930_e24513,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign21930_e24515;

        let assign21940_e24522: f64 = if ((p.p857 == 0.0) && (p.p862 == 0.0)) { 1.0 } else { 0.0 };
        var_guard415 = assign21940_e24522;

        let (assign21950_e24533, assign21950_e24533_d_n6, assign21950_e24533_d_n7, assign21950_e24533_d_n8, assign21950_e24533_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard415 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign21950_e24533;
        var_isrh_dn6 = assign21950_e24533_d_n6;
        var_isrh_dn7 = assign21950_e24533_d_n7;
        var_isrh_dn8 = assign21950_e24533_d_n8;
        var_isrh_dn9 = assign21950_e24533_d_n9;

        let (assign21960_e24547,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard415 == 0.0)) {
        let assign21960_e24545: f64 = (var_vbibot - var_vjsrh);
        (assign21960_e24545,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign21960_e24547;

        let (assign21970_e24566,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard415 == 0.0)) {
        let assign21970_e24561: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign21970_e24562: f64 = (1.0 - assign21970_e24561);
        let assign21970_e24563: f64 = (assign21970_e24562).sqrt();
        let assign21970_e24564: f64 = (1.0 - assign21970_e24563);
        (assign21970_e24564,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign21970_e24566;

        let assign21980_e24569: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };
        var_guard416 = assign21980_e24569;

        let (assign21990_e24583,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard415 == 0.0)) && (var_guard416 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign21990_e24583;

        let (assign22000_e24615,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard415 == 0.0)) && (var_guard416 == 0.0)) {
        let assign22000_e24598: f64 = (var_wsrhstep * var_wsrhstep);
        let assign22000_e24600: f64 = (var_wsrhstep).ln();
        let assign22000_e24601: f64 = (assign22000_e24598 * assign22000_e24600);
        let assign22000_e24604: f64 = (1.0 - var_wsrhstep);
        let assign22000_e24605: f64 = (assign22000_e24601 / assign22000_e24604);
        let assign22000_e24607: f64 = (assign22000_e24605 + var_wsrhstep);
        let assign22000_e24611: f64 = (2.0 * p.p848);
        let assign22000_e24612: f64 = (1.0 - assign22000_e24611);
        let assign22000_e24613: f64 = (assign22000_e24607 * assign22000_e24612);
        (assign22000_e24613,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign22000_e24615;

        let (assign22010_e24629,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard415 == 0.0)) {
        let assign22010_e24627: f64 = (var_wsrhstep + var_dwsrh);
        (assign22010_e24627,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign22010_e24629;

        let assign22020_e24632: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };
        var_guard417 = assign22020_e24632;

        let (assign22030_e24649, assign22030_e24649_d_n6, assign22030_e24649_d_n7, assign22030_e24649_d_n8, assign22030_e24649_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard415 == 0.0)) && (var_guard417 != 0.0)) {
        let assign22030_e24646: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign22030_e24647: f64 = (assign22030_e24646).sqrt();
        (assign22030_e24647, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign22030_e24649;
        var_tmp_dn6 = assign22030_e24649_d_n6;
        var_tmp_dn7 = assign22030_e24649_d_n7;
        var_tmp_dn8 = assign22030_e24649_d_n8;
        var_tmp_dn9 = assign22030_e24649_d_n9;

        let (assign22040_e24668, assign22040_e24668_d_n6, assign22040_e24668_d_n7, assign22040_e24668_d_n8, assign22040_e24668_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard415 == 0.0)) && (var_guard417 == 0.0)) {
        let assign22040_e24664: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign22040_e24666: f64 = (assign22040_e24664).powf(p.p848);
        (assign22040_e24666, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign22040_e24668;
        var_tmp_dn6 = assign22040_e24668_d_n6;
        var_tmp_dn7 = assign22040_e24668_d_n7;
        var_tmp_dn8 = assign22040_e24668_d_n8;
        var_tmp_dn9 = assign22040_e24668_d_n9;

        let (assign22050_e24682, assign22050_e24682_d_n6, assign22050_e24682_d_n7, assign22050_e24682_d_n8, assign22050_e24682_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard415 == 0.0)) {
        let assign22050_e24680: f64 = (var_wdepnulrbot * var_tmp);
        (assign22050_e24680, (var_wdepnulrbot * var_tmp_dn6), (var_wdepnulrbot * var_tmp_dn7), (var_wdepnulrbot * var_tmp_dn8), (var_wdepnulrbot * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign22050_e24682;
        var_wdep_dn6 = assign22050_e24682_d_n6;
        var_wdep_dn7 = assign22050_e24682_d_n7;
        var_wdep_dn8 = assign22050_e24682_d_n8;
        var_wdep_dn9 = assign22050_e24682_d_n9;

        let (assign22060_e24700, assign22060_e24700_d_n6, assign22060_e24700_d_n7, assign22060_e24700_d_n8, assign22060_e24700_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard415 == 0.0)) {
        let assign22060_e24695: f64 = (var_zinv - 1.0);
        let assign22060_e24697: f64 = (assign22060_e24695 * var_wdep);
        let assign22060_e24698: f64 = (var_ftdbot * assign22060_e24697);
        (assign22060_e24698, (var_ftdbot * (assign22060_e24695 * var_wdep_dn6)), (var_ftdbot * (assign22060_e24695 * var_wdep_dn7)), (var_ftdbot * (assign22060_e24695 * var_wdep_dn8)), (var_ftdbot * (assign22060_e24695 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign22060_e24700;
        var_asrh_dn6 = assign22060_e24700_d_n6;
        var_asrh_dn7 = assign22060_e24700_d_n7;
        var_asrh_dn8 = assign22060_e24700_d_n8;
        var_asrh_dn9 = assign22060_e24700_d_n9;

        let (assign22070_e24716, assign22070_e24716_d_n6, assign22070_e24716_d_n7, assign22070_e24716_d_n8, assign22070_e24716_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard415 == 0.0)) {
        let assign22070_e24713: f64 = (var_asrh * var_wsrh);
        let assign22070_e24714: f64 = (p.p857 * assign22070_e24713);
        (assign22070_e24714, (p.p857 * (var_asrh_dn6 * var_wsrh)), (p.p857 * (var_asrh_dn7 * var_wsrh)), (p.p857 * (var_asrh_dn8 * var_wsrh)), (p.p857 * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign22070_e24716;
        var_isrh_dn6 = assign22070_e24716_d_n6;
        var_isrh_dn7 = assign22070_e24716_d_n7;
        var_isrh_dn8 = assign22070_e24716_d_n8;
        var_isrh_dn9 = assign22070_e24716_d_n9;

        let assign22080_e24719: f64 = if p.p862 == 0.0 { 1.0 } else { 0.0 };
        var_guard418 = assign22080_e24719;

        let (assign22090_e24730, assign22090_e24730_d_n6, assign22090_e24730_d_n7, assign22090_e24730_d_n8, assign22090_e24730_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign22090_e24730;
        var_itat_dn6 = assign22090_e24730_d_n6;
        var_itat_dn7 = assign22090_e24730_d_n7;
        var_itat_dn8 = assign22090_e24730_d_n8;
        var_itat_dn9 = assign22090_e24730_d_n9;

        let (assign22100_e24748, assign22100_e24748_d_n6, assign22100_e24748_d_n7, assign22100_e24748_d_n8, assign22100_e24748_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) {
        let assign22100_e24743: f64 = (var_wdep * var_one_minus_pbot);
        let assign22100_e24745: f64 = (assign22100_e24743 / var_vbi_minus_vjsrh);
        let assign22100_e24746: f64 = (var_btatpartbot * assign22100_e24745);
        (assign22100_e24746, (var_btatpartbot * ((var_wdep_dn6 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn7 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn8 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn9 * var_one_minus_pbot) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign22100_e24748;
        var_btat_dn6 = assign22100_e24748_d_n6;
        var_btat_dn7 = assign22100_e24748_d_n7;
        var_btat_dn8 = assign22100_e24748_d_n8;
        var_btat_dn9 = assign22100_e24748_d_n9;

        let (assign22110_e24764, assign22110_e24764_d_n6, assign22110_e24764_d_n7, assign22110_e24764_d_n8, assign22110_e24764_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) {
        let assign22110_e24760: f64 = (0.666666666666667 * var_atatbot);
        let assign22110_e24762: f64 = (assign22110_e24760 / var_btat);
        (assign22110_e24762, (-((assign22110_e24760 * var_btat_dn6) / (var_btat * var_btat))), (-((assign22110_e24760 * var_btat_dn7) / (var_btat * var_btat))), (-((assign22110_e24760 * var_btat_dn8) / (var_btat * var_btat))), (-((assign22110_e24760 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign22110_e24764;
        var_twoatatoverthreebtat_dn6 = assign22110_e24764_d_n6;
        var_twoatatoverthreebtat_dn7 = assign22110_e24764_d_n7;
        var_twoatatoverthreebtat_dn8 = assign22110_e24764_d_n8;
        var_twoatatoverthreebtat_dn9 = assign22110_e24764_d_n9;

        let (assign22120_e24778, assign22120_e24778_d_n6, assign22120_e24778_d_n7, assign22120_e24778_d_n8, assign22120_e24778_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) {
        let assign22120_e24776: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign22120_e24776, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign22120_e24778;
        var_umaxbeforelimiting_dn6 = assign22120_e24778_d_n6;
        var_umaxbeforelimiting_dn7 = assign22120_e24778_d_n7;
        var_umaxbeforelimiting_dn8 = assign22120_e24778_d_n8;
        var_umaxbeforelimiting_dn9 = assign22120_e24778_d_n9;

        let (assign22130_e24799, assign22130_e24799_d_n6, assign22130_e24799_d_n7, assign22130_e24799_d_n8, assign22130_e24799_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) {
        let assign22130_e24790: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign22130_e24793: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign22130_e24795: f64 = (assign22130_e24793 + 1.0);
        let assign22130_e24796: f64 = (assign22130_e24790 / assign22130_e24795);
        let assign22130_e24797: f64 = (assign22130_e24796).sqrt();
        (assign22130_e24797, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign22130_e24795) - (assign22130_e24790 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign22130_e24795 * assign22130_e24795)) / (2.0 * assign22130_e24797)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign22130_e24795) - (assign22130_e24790 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign22130_e24795 * assign22130_e24795)) / (2.0 * assign22130_e24797)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign22130_e24795) - (assign22130_e24790 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign22130_e24795 * assign22130_e24795)) / (2.0 * assign22130_e24797)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign22130_e24795) - (assign22130_e24790 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign22130_e24795 * assign22130_e24795)) / (2.0 * assign22130_e24797)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign22130_e24799;
        var_umax_dn6 = assign22130_e24799_d_n6;
        var_umax_dn7 = assign22130_e24799_d_n7;
        var_umax_dn8 = assign22130_e24799_d_n8;
        var_umax_dn9 = assign22130_e24799_d_n9;

        let (assign22140_e24812, assign22140_e24812_d_n6, assign22140_e24812_d_n7, assign22140_e24812_d_n8, assign22140_e24812_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) {
        let assign22140_e24810: f64 = (var_umax).sqrt();
        (assign22140_e24810, (var_umax_dn6 / (2.0 * assign22140_e24810)), (var_umax_dn7 / (2.0 * assign22140_e24810)), (var_umax_dn8 / (2.0 * assign22140_e24810)), (var_umax_dn9 / (2.0 * assign22140_e24810)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign22140_e24812;
        var_sqrtumax_dn6 = assign22140_e24812_d_n6;
        var_sqrtumax_dn7 = assign22140_e24812_d_n7;
        var_sqrtumax_dn8 = assign22140_e24812_d_n8;
        var_sqrtumax_dn9 = assign22140_e24812_d_n9;

        let (assign22150_e24826, assign22150_e24826_d_n6, assign22150_e24826_d_n7, assign22150_e24826_d_n8, assign22150_e24826_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) {
        let assign22150_e24824: f64 = (var_umax * var_sqrtumax);
        (assign22150_e24824, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign22150_e24826;
        var_umaxpoweronepointfive_dn6 = assign22150_e24826_d_n6;
        var_umaxpoweronepointfive_dn7 = assign22150_e24826_d_n7;
        var_umaxpoweronepointfive_dn8 = assign22150_e24826_d_n8;
        var_umaxpoweronepointfive_dn9 = assign22150_e24826_d_n9;

        let assign22160_e24828: f64 = (-p.p848);
        let assign22160_e24830: f64 = (assign22160_e24828 * var_one_over_one_minus_pbot);
        let assign22160_e24832: f64 = (-1.0);
        let assign22160_e24833: f64 = if assign22160_e24830 == assign22160_e24832 { 1.0 } else { 0.0 };
        var_guard419 = assign22160_e24833;

        let (assign22170_e24853, assign22170_e24853_d_n6, assign22170_e24853_d_n7, assign22170_e24853_d_n8, assign22170_e24853_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) && (var_guard419 != 0.0)) {
        let assign22170_e24849: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign22170_e24850: f64 = (1.0 + assign22170_e24849);
        let assign22170_e24851: f64 = (1.0 / assign22170_e24850);
        (assign22170_e24851, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign22170_e24850 * assign22170_e24850))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign22170_e24850 * assign22170_e24850))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign22170_e24850 * assign22170_e24850))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign22170_e24850 * assign22170_e24850))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign22170_e24853;
        var_wgamma_dn6 = assign22170_e24853_d_n6;
        var_wgamma_dn7 = assign22170_e24853_d_n7;
        var_wgamma_dn8 = assign22170_e24853_d_n8;
        var_wgamma_dn9 = assign22170_e24853_d_n9;

        let (assign22180_e24877, assign22180_e24877_d_n6, assign22180_e24877_d_n7, assign22180_e24877_d_n8, assign22180_e24877_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) && (var_guard419 == 0.0)) {
        let assign22180_e24869: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign22180_e24870: f64 = (1.0 + assign22180_e24869);
        let assign22180_e24872: f64 = (-p.p848);
        let assign22180_e24874: f64 = (assign22180_e24872 * var_one_over_one_minus_pbot);
        let assign22180_e24875: f64 = (assign22180_e24870).powf(assign22180_e24874);
        (assign22180_e24875, if 0.0 == 0.0 && ((assign22180_e24874) as f64).is_finite() && ((assign22180_e24874) as f64).fract() == 0.0 { if assign22180_e24874 == 0.0 { 0.0 } else { (assign22180_e24874 * ((assign22180_e24870).powf(assign22180_e24874 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign22180_e24875 * (assign22180_e24874 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign22180_e24870))) }, if 0.0 == 0.0 && ((assign22180_e24874) as f64).is_finite() && ((assign22180_e24874) as f64).fract() == 0.0 { if assign22180_e24874 == 0.0 { 0.0 } else { (assign22180_e24874 * ((assign22180_e24870).powf(assign22180_e24874 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign22180_e24875 * (assign22180_e24874 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign22180_e24870))) }, if 0.0 == 0.0 && ((assign22180_e24874) as f64).is_finite() && ((assign22180_e24874) as f64).fract() == 0.0 { if assign22180_e24874 == 0.0 { 0.0 } else { (assign22180_e24874 * ((assign22180_e24870).powf(assign22180_e24874 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign22180_e24875 * (assign22180_e24874 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign22180_e24870))) }, if 0.0 == 0.0 && ((assign22180_e24874) as f64).is_finite() && ((assign22180_e24874) as f64).fract() == 0.0 { if assign22180_e24874 == 0.0 { 0.0 } else { (assign22180_e24874 * ((assign22180_e24870).powf(assign22180_e24874 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign22180_e24875 * (assign22180_e24874 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign22180_e24870))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign22180_e24877;
        var_wgamma_dn6 = assign22180_e24877_d_n6;
        var_wgamma_dn7 = assign22180_e24877_d_n7;
        var_wgamma_dn8 = assign22180_e24877_d_n8;
        var_wgamma_dn9 = assign22180_e24877_d_n9;

        let (assign22190_e24895, assign22190_e24895_d_n6, assign22190_e24895_d_n7, assign22190_e24895_d_n8, assign22190_e24895_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) {
        let assign22190_e24889: f64 = (var_wsrh * var_wgamma);
        let assign22190_e24892: f64 = (var_wsrh + var_wgamma);
        let assign22190_e24893: f64 = (assign22190_e24889 / assign22190_e24892);
        (assign22190_e24893, ((((var_wsrh * var_wgamma_dn6) * assign22190_e24892) - (assign22190_e24889 * var_wgamma_dn6)) / (assign22190_e24892 * assign22190_e24892)), ((((var_wsrh * var_wgamma_dn7) * assign22190_e24892) - (assign22190_e24889 * var_wgamma_dn7)) / (assign22190_e24892 * assign22190_e24892)), ((((var_wsrh * var_wgamma_dn8) * assign22190_e24892) - (assign22190_e24889 * var_wgamma_dn8)) / (assign22190_e24892 * assign22190_e24892)), ((((var_wsrh * var_wgamma_dn9) * assign22190_e24892) - (assign22190_e24889 * var_wgamma_dn9)) / (assign22190_e24892 * assign22190_e24892)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign22190_e24895;
        var_wtat_dn6 = assign22190_e24895_d_n6;
        var_wtat_dn7 = assign22190_e24895_d_n7;
        var_wtat_dn8 = assign22190_e24895_d_n8;
        var_wtat_dn9 = assign22190_e24895_d_n9;

        let (assign22200_e24912, assign22200_e24912_d_n6, assign22200_e24912_d_n7, assign22200_e24912_d_n8, assign22200_e24912_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) {
        let assign22200_e24908: f64 = (var_btat / var_sqrtumax);
        let assign22200_e24909: f64 = (0.375 * assign22200_e24908);
        let assign22200_e24910: f64 = (assign22200_e24909).sqrt();
        (assign22200_e24910, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign22200_e24910)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign22200_e24910)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign22200_e24910)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign22200_e24910)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign22200_e24912;
        var_ktat_dn6 = assign22200_e24912_d_n6;
        var_ktat_dn7 = assign22200_e24912_d_n7;
        var_ktat_dn8 = assign22200_e24912_d_n8;
        var_ktat_dn9 = assign22200_e24912_d_n9;

        let (assign22210_e24930, assign22210_e24930_d_n6, assign22210_e24930_d_n7, assign22210_e24930_d_n8, assign22210_e24930_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) {
        let assign22210_e24925: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign22210_e24926: f64 = (2.0 * assign22210_e24925);
        let assign22210_e24928: f64 = (assign22210_e24926 - var_umax);
        (assign22210_e24928, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign22210_e24930;
        var_ltat_dn6 = assign22210_e24930_d_n6;
        var_ltat_dn7 = assign22210_e24930_d_n7;
        var_ltat_dn8 = assign22210_e24930_d_n8;
        var_ltat_dn9 = assign22210_e24930_d_n9;

        let (assign22220_e24956, assign22220_e24956_d_n6, assign22220_e24956_d_n7, assign22220_e24956_d_n8, assign22220_e24956_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) {
        let assign22220_e24942: f64 = (var_atatbot * var_twoatatoverthreebtat);
        let assign22220_e24944: f64 = (assign22220_e24942 * var_sqrtumax);
        let assign22220_e24947: f64 = (var_atatbot * var_umax);
        let assign22220_e24948: f64 = (assign22220_e24944 - assign22220_e24947);
        let assign22220_e24952: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign22220_e24953: f64 = (0.5 * assign22220_e24952);
        let assign22220_e24954: f64 = (assign22220_e24948 + assign22220_e24953);
        (assign22220_e24954, (((((var_atatbot * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign22220_e24942 * var_sqrtumax_dn6)) - (var_atatbot * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign22220_e24942 * var_sqrtumax_dn7)) - (var_atatbot * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign22220_e24942 * var_sqrtumax_dn8)) - (var_atatbot * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatbot * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign22220_e24942 * var_sqrtumax_dn9)) - (var_atatbot * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign22220_e24956;
        var_mtat_dn6 = assign22220_e24956_d_n6;
        var_mtat_dn7 = assign22220_e24956_d_n7;
        var_mtat_dn8 = assign22220_e24956_d_n8;
        var_mtat_dn9 = assign22220_e24956_d_n9;

        let (assign22230_e24972, assign22230_e24972_d_n6, assign22230_e24972_d_n7, assign22230_e24972_d_n8, assign22230_e24972_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) {
        let assign22230_e24968: f64 = (var_ltat - 1.0);
        let assign22230_e24970: f64 = (assign22230_e24968 * var_ktat);
        (assign22230_e24970, ((var_ltat_dn6 * var_ktat) + (assign22230_e24968 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign22230_e24968 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign22230_e24968 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign22230_e24968 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign22230_e24972;
        var_xerfc_dn6 = assign22230_e24972_d_n6;
        var_xerfc_dn7 = assign22230_e24972_d_n7;
        var_xerfc_dn8 = assign22230_e24972_d_n8;
        var_xerfc_dn9 = assign22230_e24972_d_n9;

        let (assign22240_e24986, assign22240_e24986_d_n6, assign22240_e24986_d_n7, assign22240_e24986_d_n8, assign22240_e24986_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) {
        let assign22240_e24984: f64 = (var_xerfc * var_xerfc);
        (assign22240_e24984, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign22240_e24986;
        var_ysq_dn6 = assign22240_e24986_d_n6;
        var_ysq_dn7 = assign22240_e24986_d_n7;
        var_ysq_dn8 = assign22240_e24986_d_n8;
        var_ysq_dn9 = assign22240_e24986_d_n9;

        let assign22250_e24989: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard420 = assign22250_e24989;

        let (assign22260_e25009, assign22260_e25009_d_n6, assign22260_e25009_d_n7, assign22260_e25009_d_n8, assign22260_e25009_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) && (var_guard420 != 0.0)) {
        let assign22260_e25005: f64 = (var_perfc * var_xerfc);
        let assign22260_e25006: f64 = (1.0 + assign22260_e25005);
        let assign22260_e25007: f64 = (1.0 / assign22260_e25006);
        (assign22260_e25007, (-((var_perfc * var_xerfc_dn6) / (assign22260_e25006 * assign22260_e25006))), (-((var_perfc * var_xerfc_dn7) / (assign22260_e25006 * assign22260_e25006))), (-((var_perfc * var_xerfc_dn8) / (assign22260_e25006 * assign22260_e25006))), (-((var_perfc * var_xerfc_dn9) / (assign22260_e25006 * assign22260_e25006))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign22260_e25009;
        var_terfc_dn6 = assign22260_e25009_d_n6;
        var_terfc_dn7 = assign22260_e25009_d_n7;
        var_terfc_dn8 = assign22260_e25009_d_n8;
        var_terfc_dn9 = assign22260_e25009_d_n9;

        let (assign22270_e25030, assign22270_e25030_d_n6, assign22270_e25030_d_n7, assign22270_e25030_d_n8, assign22270_e25030_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) && (var_guard420 == 0.0)) {
        let assign22270_e25026: f64 = (var_perfc * var_xerfc);
        let assign22270_e25027: f64 = (1.0 - assign22270_e25026);
        let assign22270_e25028: f64 = (1.0 / assign22270_e25027);
        (assign22270_e25028, (-((-(var_perfc * var_xerfc_dn6)) / (assign22270_e25027 * assign22270_e25027))), (-((-(var_perfc * var_xerfc_dn7)) / (assign22270_e25027 * assign22270_e25027))), (-((-(var_perfc * var_xerfc_dn8)) / (assign22270_e25027 * assign22270_e25027))), (-((-(var_perfc * var_xerfc_dn9)) / (assign22270_e25027 * assign22270_e25027))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign22270_e25030;
        var_terfc_dn6 = assign22270_e25030_d_n6;
        var_terfc_dn7 = assign22270_e25030_d_n7;
        var_terfc_dn8 = assign22270_e25030_d_n8;
        var_terfc_dn9 = assign22270_e25030_d_n9;

        let assign22280_e25032: f64 = (-var_ysq);
        let assign22280_e25034: f64 = (assign22280_e25032 + var_mtat);
        let assign22280_e25036: f64 = (-230.25850929940458);
        let assign22280_e25037: f64 = if assign22280_e25034 > assign22280_e25036 { 1.0 } else { 0.0 };
        var_guard421 = assign22280_e25037;

        let (assign22290_e25055, assign22290_e25055_d_n6, assign22290_e25055_d_n7, assign22290_e25055_d_n8, assign22290_e25055_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) && (var_guard421 != 0.0)) {
        let assign22290_e25050: f64 = (-var_ysq);
        let assign22290_e25052: f64 = (assign22290_e25050 + var_mtat);
        let assign22290_e25053: f64 = (assign22290_e25052).exp();
        (assign22290_e25053, (assign22290_e25053 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign22290_e25053 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign22290_e25053 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign22290_e25053 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign22290_e25055;
        var_tmp_dn6 = assign22290_e25055_d_n6;
        var_tmp_dn7 = assign22290_e25055_d_n7;
        var_tmp_dn8 = assign22290_e25055_d_n8;
        var_tmp_dn9 = assign22290_e25055_d_n9;

        let (assign22300_e25104, assign22300_e25104_d_n6, assign22300_e25104_d_n7, assign22300_e25104_d_n8, assign22300_e25104_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) && (var_guard421 == 0.0)) {
        let assign22300_e25071: f64 = (-230.25850929940458);
        let assign22300_e25073: f64 = (-var_ysq);
        let assign22300_e25075: f64 = (assign22300_e25073 + var_mtat);
        let assign22300_e25076: f64 = (assign22300_e25071 - assign22300_e25075);
        let assign22300_e25080: f64 = (-230.25850929940458);
        let assign22300_e25082: f64 = (-var_ysq);
        let assign22300_e25084: f64 = (assign22300_e25082 + var_mtat);
        let assign22300_e25085: f64 = (assign22300_e25080 - assign22300_e25084);
        let assign22300_e25088: f64 = (-230.25850929940458);
        let assign22300_e25090: f64 = (-var_ysq);
        let assign22300_e25092: f64 = (assign22300_e25090 + var_mtat);
        let assign22300_e25093: f64 = (assign22300_e25088 - assign22300_e25092);
        let assign22300_e25095: f64 = (assign22300_e25093 * 0.3333333333333333);
        let assign22300_e25096: f64 = (1.0 + assign22300_e25095);
        let assign22300_e25097: f64 = (assign22300_e25085 * assign22300_e25096);
        let assign22300_e25098: f64 = (0.5 * assign22300_e25097);
        let assign22300_e25099: f64 = (1.0 + assign22300_e25098);
        let assign22300_e25100: f64 = (assign22300_e25076 * assign22300_e25099);
        let assign22300_e25101: f64 = (1.0 + assign22300_e25100);
        let assign22300_e25102: f64 = (1e-100 / assign22300_e25101);
        (assign22300_e25102, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign22300_e25099) + (assign22300_e25076 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign22300_e25096) + (assign22300_e25085 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign22300_e25101 * assign22300_e25101))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign22300_e25099) + (assign22300_e25076 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign22300_e25096) + (assign22300_e25085 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign22300_e25101 * assign22300_e25101))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign22300_e25099) + (assign22300_e25076 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign22300_e25096) + (assign22300_e25085 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign22300_e25101 * assign22300_e25101))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign22300_e25099) + (assign22300_e25076 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign22300_e25096) + (assign22300_e25085 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign22300_e25101 * assign22300_e25101))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign22300_e25104;
        var_tmp_dn6 = assign22300_e25104_d_n6;
        var_tmp_dn7 = assign22300_e25104_d_n7;
        var_tmp_dn8 = assign22300_e25104_d_n8;
        var_tmp_dn9 = assign22300_e25104_d_n9;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_asrh_dn9_slot = var_asrh_dn9;
        *var_btat_slot = var_btat;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_btat_dn9_slot = var_btat_dn9;
        *var_dwsrh_slot = var_dwsrh;
        *var_guard415_slot = var_guard415;
        *var_guard416_slot = var_guard416;
        *var_guard417_slot = var_guard417;
        *var_guard418_slot = var_guard418;
        *var_guard419_slot = var_guard419;
        *var_guard420_slot = var_guard420;
        *var_guard421_slot = var_guard421;
        *var_id__blk212_slot = var_id__blk212;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ktat_dn9_slot = var_ktat_dn9;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_ltat_dn9_slot = var_ltat_dn9;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_mtat_dn9_slot = var_mtat_dn9;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_sqrtumax_dn9_slot = var_sqrtumax_dn9;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_terfc_dn9_slot = var_terfc_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_twoatatoverthreebtat_dn9_slot = var_twoatatoverthreebtat_dn9;
        *var_umax_slot = var_umax;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umax_dn9_slot = var_umax_dn9;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxbeforelimiting_dn9_slot = var_umaxbeforelimiting_dn9;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_umaxpoweronepointfive_dn9_slot = var_umaxpoweronepointfive_dn9;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wdep_dn9_slot = var_wdep_dn9;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wgamma_dn9_slot = var_wgamma_dn9;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_wtat_dn9_slot = var_wtat_dn9;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_xerfc_dn9_slot = var_xerfc_dn9;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
        *var_ysq_dn9_slot = var_ysq_dn9;
    }

    pub(super) fn stamp_transient_block_41(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatbot: f64,
        var_berfc: f64,
        var_cerfc: f64,
        var_fbbtbot: f64,
        var_fstopbot: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard414: f64,
        var_guard418: f64,
        var_idmult: f64,
        var_idsatsti: f64,
        var_ktat: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_ktat_dn9: f64,
        var_lssource_i: f64,
        var_mtat: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_mtat_dn9: f64,
        var_one_over_one_minus_pbot: f64,
        var_slopebot: f64,
        var_terfc: f64,
        var_terfc_dn6: f64,
        var_terfc_dn7: f64,
        var_terfc_dn8: f64,
        var_terfc_dn9: f64,
        var_two_psistar: f64,
        var_v4: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirbotinv: f64,
        var_vbisti: f64,
        var_vbrinvbot: f64,
        var_vjsrh: f64,
        var_wdepnulrinvbot: f64,
        var_wtat: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_wtat_dn9: f64,
        var_xerfc: f64,
        var_dwsrh_slot: &mut f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfcpos_dn9_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_erfctimesexpmtat_dn9_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_fmaxr_dn9_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_gammamax_dn9_slot: &mut f64,
        var_guard422_slot: &mut f64,
        var_guard423_slot: &mut f64,
        var_guard424_slot: &mut f64,
        var_guard425_slot: &mut f64,
        var_guard426_slot: &mut f64,
        var_guard427_slot: &mut f64,
        var_guard428_slot: &mut f64,
        var_guard429_slot: &mut f64,
        var_guard430_slot: &mut f64,
        var_guard431_slot: &mut f64,
        var_guard432_slot: &mut f64,
        var_guard433_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_id__blk212_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijunbot_dn9_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_ijunsti_dn9_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
    ) {
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfcpos_dn9: f64 = *var_erfcpos_dn9_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_erfctimesexpmtat_dn9: f64 = *var_erfctimesexpmtat_dn9_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fbreakdown_dn9: f64 = *var_fbreakdown_dn9_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_fmaxr_dn9: f64 = *var_fmaxr_dn9_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_gammamax_dn9: f64 = *var_gammamax_dn9_slot;
        let mut var_guard422: f64 = *var_guard422_slot;
        let mut var_guard423: f64 = *var_guard423_slot;
        let mut var_guard424: f64 = *var_guard424_slot;
        let mut var_guard425: f64 = *var_guard425_slot;
        let mut var_guard426: f64 = *var_guard426_slot;
        let mut var_guard427: f64 = *var_guard427_slot;
        let mut var_guard428: f64 = *var_guard428_slot;
        let mut var_guard429: f64 = *var_guard429_slot;
        let mut var_guard430: f64 = *var_guard430_slot;
        let mut var_guard431: f64 = *var_guard431_slot;
        let mut var_guard432: f64 = *var_guard432_slot;
        let mut var_guard433: f64 = *var_guard433_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_id__blk212: f64 = *var_id__blk212_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijunbot_dn9: f64 = *var_ijunbot_dn9_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_ijunsti_dn9: f64 = *var_ijunsti_dn9_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;

        let (assign22310_e25134, assign22310_e25134_d_n6, assign22310_e25134_d_n7, assign22310_e25134_d_n8, assign22310_e25134_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) {
        let assign22310_e25116: f64 = (0.29214664 * var_terfc);
        let assign22310_e25120: f64 = (var_terfc * var_terfc);
        let assign22310_e25121: f64 = (var_berfc * assign22310_e25120);
        let assign22310_e25122: f64 = (assign22310_e25116 + assign22310_e25121);
        let assign22310_e25126: f64 = (var_terfc * var_terfc);
        let assign22310_e25128: f64 = (assign22310_e25126 * var_terfc);
        let assign22310_e25129: f64 = (var_cerfc * assign22310_e25128);
        let assign22310_e25130: f64 = (assign22310_e25122 + assign22310_e25129);
        let assign22310_e25132: f64 = (assign22310_e25130 * var_tmp);
        (assign22310_e25132, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign22310_e25126 * var_terfc_dn6)))) * var_tmp) + (assign22310_e25130 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign22310_e25126 * var_terfc_dn7)))) * var_tmp) + (assign22310_e25130 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign22310_e25126 * var_terfc_dn8)))) * var_tmp) + (assign22310_e25130 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign22310_e25126 * var_terfc_dn9)))) * var_tmp) + (assign22310_e25130 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign22310_e25134;
        var_erfcpos_dn6 = assign22310_e25134_d_n6;
        var_erfcpos_dn7 = assign22310_e25134_d_n7;
        var_erfcpos_dn8 = assign22310_e25134_d_n8;
        var_erfcpos_dn9 = assign22310_e25134_d_n9;

        let assign22320_e25137: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard422 = assign22320_e25137;

        let (assign22330_e25151, assign22330_e25151_d_n6, assign22330_e25151_d_n7, assign22330_e25151_d_n8, assign22330_e25151_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) && (var_guard422 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign22330_e25151;
        var_erfctimesexpmtat_dn6 = assign22330_e25151_d_n6;
        var_erfctimesexpmtat_dn7 = assign22330_e25151_d_n7;
        var_erfctimesexpmtat_dn8 = assign22330_e25151_d_n8;
        var_erfctimesexpmtat_dn9 = assign22330_e25151_d_n9;

        let assign22340_e25154: f64 = (-230.25850929940458);
        let assign22340_e25155: f64 = if var_mtat > assign22340_e25154 { 1.0 } else { 0.0 };
        var_guard423 = assign22340_e25155;

        let (assign22350_e25173, assign22350_e25173_d_n6, assign22350_e25173_d_n7, assign22350_e25173_d_n8, assign22350_e25173_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) && (var_guard422 == 0.0)) && (var_guard423 != 0.0)) {
        let assign22350_e25171: f64 = (var_mtat).exp();
        (assign22350_e25171, (assign22350_e25171 * var_mtat_dn6), (assign22350_e25171 * var_mtat_dn7), (assign22350_e25171 * var_mtat_dn8), (assign22350_e25171 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign22350_e25173;
        var_tmp_dn6 = assign22350_e25173_d_n6;
        var_tmp_dn7 = assign22350_e25173_d_n7;
        var_tmp_dn8 = assign22350_e25173_d_n8;
        var_tmp_dn9 = assign22350_e25173_d_n9;

        let (assign22360_e25216, assign22360_e25216_d_n6, assign22360_e25216_d_n7, assign22360_e25216_d_n8, assign22360_e25216_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) && (var_guard422 == 0.0)) && (var_guard423 == 0.0)) {
        let assign22360_e25192: f64 = (-230.25850929940458);
        let assign22360_e25194: f64 = (assign22360_e25192 - var_mtat);
        let assign22360_e25198: f64 = (-230.25850929940458);
        let assign22360_e25200: f64 = (assign22360_e25198 - var_mtat);
        let assign22360_e25203: f64 = (-230.25850929940458);
        let assign22360_e25205: f64 = (assign22360_e25203 - var_mtat);
        let assign22360_e25207: f64 = (assign22360_e25205 * 0.3333333333333333);
        let assign22360_e25208: f64 = (1.0 + assign22360_e25207);
        let assign22360_e25209: f64 = (assign22360_e25200 * assign22360_e25208);
        let assign22360_e25210: f64 = (0.5 * assign22360_e25209);
        let assign22360_e25211: f64 = (1.0 + assign22360_e25210);
        let assign22360_e25212: f64 = (assign22360_e25194 * assign22360_e25211);
        let assign22360_e25213: f64 = (1.0 + assign22360_e25212);
        let assign22360_e25214: f64 = (1e-100 / assign22360_e25213);
        (assign22360_e25214, (-((1e-100 * (((-var_mtat_dn6) * assign22360_e25211) + (assign22360_e25194 * (0.5 * (((-var_mtat_dn6) * assign22360_e25208) + (assign22360_e25200 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign22360_e25213 * assign22360_e25213))), (-((1e-100 * (((-var_mtat_dn7) * assign22360_e25211) + (assign22360_e25194 * (0.5 * (((-var_mtat_dn7) * assign22360_e25208) + (assign22360_e25200 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign22360_e25213 * assign22360_e25213))), (-((1e-100 * (((-var_mtat_dn8) * assign22360_e25211) + (assign22360_e25194 * (0.5 * (((-var_mtat_dn8) * assign22360_e25208) + (assign22360_e25200 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign22360_e25213 * assign22360_e25213))), (-((1e-100 * (((-var_mtat_dn9) * assign22360_e25211) + (assign22360_e25194 * (0.5 * (((-var_mtat_dn9) * assign22360_e25208) + (assign22360_e25200 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign22360_e25213 * assign22360_e25213))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign22360_e25216;
        var_tmp_dn6 = assign22360_e25216_d_n6;
        var_tmp_dn7 = assign22360_e25216_d_n7;
        var_tmp_dn8 = assign22360_e25216_d_n8;
        var_tmp_dn9 = assign22360_e25216_d_n9;

        let (assign22370_e25235, assign22370_e25235_d_n6, assign22370_e25235_d_n7, assign22370_e25235_d_n8, assign22370_e25235_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) && (var_guard422 == 0.0)) {
        let assign22370_e25231: f64 = (2.0 * var_tmp);
        let assign22370_e25233: f64 = (assign22370_e25231 - var_erfcpos);
        (assign22370_e25233, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign22370_e25235;
        var_erfctimesexpmtat_dn6 = assign22370_e25235_d_n6;
        var_erfctimesexpmtat_dn7 = assign22370_e25235_d_n7;
        var_erfctimesexpmtat_dn8 = assign22370_e25235_d_n8;
        var_erfctimesexpmtat_dn9 = assign22370_e25235_d_n9;

        let (assign22380_e25255, assign22380_e25255_d_n6, assign22380_e25255_d_n7, assign22380_e25255_d_n8, assign22380_e25255_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) {
        let assign22380_e25247: f64 = (1.772453850905516 * 0.5);
        let assign22380_e25250: f64 = (var_atatbot * var_erfctimesexpmtat);
        let assign22380_e25252: f64 = (assign22380_e25250 / var_ktat);
        let assign22380_e25253: f64 = (assign22380_e25247 * assign22380_e25252);
        (assign22380_e25253, (assign22380_e25247 * ((((var_atatbot * var_erfctimesexpmtat_dn6) * var_ktat) - (assign22380_e25250 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign22380_e25247 * ((((var_atatbot * var_erfctimesexpmtat_dn7) * var_ktat) - (assign22380_e25250 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign22380_e25247 * ((((var_atatbot * var_erfctimesexpmtat_dn8) * var_ktat) - (assign22380_e25250 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign22380_e25247 * ((((var_atatbot * var_erfctimesexpmtat_dn9) * var_ktat) - (assign22380_e25250 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign22380_e25255;
        var_gammamax_dn6 = assign22380_e25255_d_n6;
        var_gammamax_dn7 = assign22380_e25255_d_n7;
        var_gammamax_dn8 = assign22380_e25255_d_n8;
        var_gammamax_dn9 = assign22380_e25255_d_n9;

        let (assign22390_e25273, assign22390_e25273_d_n6, assign22390_e25273_d_n7, assign22390_e25273_d_n8, assign22390_e25273_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard418 == 0.0)) {
        let assign22390_e25268: f64 = (var_asrh * var_gammamax);
        let assign22390_e25270: f64 = (assign22390_e25268 * var_wtat);
        let assign22390_e25271: f64 = (p.p862 * assign22390_e25270);
        (assign22390_e25271, (p.p862 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign22390_e25268 * var_wtat_dn6))), (p.p862 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign22390_e25268 * var_wtat_dn7))), (p.p862 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign22390_e25268 * var_wtat_dn8))), (p.p862 * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign22390_e25268 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign22390_e25273;
        var_itat_dn6 = assign22390_e25273_d_n6;
        var_itat_dn7 = assign22390_e25273_d_n7;
        var_itat_dn8 = assign22390_e25273_d_n8;
        var_itat_dn9 = assign22390_e25273_d_n9;

        let assign22400_e25276: f64 = if p.p868 == 0.0 { 1.0 } else { 0.0 };
        var_guard424 = assign22400_e25276;

        let (assign22410_e25287, assign22410_e25287_d_n6, assign22410_e25287_d_n7, assign22410_e25287_d_n8, assign22410_e25287_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard424 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign22410_e25287;
        var_ibbt_dn6 = assign22410_e25287_d_n6;
        var_ibbt_dn7 = assign22410_e25287_d_n7;
        var_ibbt_dn8 = assign22410_e25287_d_n8;
        var_ibbt_dn9 = assign22410_e25287_d_n9;

        let assign22420_e25290: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };
        var_guard425 = assign22420_e25290;

        let (assign22430_e25309, assign22430_e25309_d_n6, assign22430_e25309_d_n7, assign22430_e25309_d_n8, assign22430_e25309_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard424 == 0.0)) && (var_guard425 != 0.0)) {
        let assign22430_e25304: f64 = (p.p845 - var_vbbt);
        let assign22430_e25306: f64 = (assign22430_e25304 * var_vbirbotinv);
        let assign22430_e25307: f64 = (assign22430_e25306).sqrt();
        (assign22430_e25307, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign22430_e25309;
        var_tmp_dn6 = assign22430_e25309_d_n6;
        var_tmp_dn7 = assign22430_e25309_d_n7;
        var_tmp_dn8 = assign22430_e25309_d_n8;
        var_tmp_dn9 = assign22430_e25309_d_n9;

        let (assign22440_e25330, assign22440_e25330_d_n6, assign22440_e25330_d_n7, assign22440_e25330_d_n8, assign22440_e25330_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard424 == 0.0)) && (var_guard425 == 0.0)) {
        let assign22440_e25324: f64 = (p.p845 - var_vbbt);
        let assign22440_e25326: f64 = (assign22440_e25324 * var_vbirbotinv);
        let assign22440_e25328: f64 = (assign22440_e25326).powf(p.p848);
        (assign22440_e25328, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign22440_e25330;
        var_tmp_dn6 = assign22440_e25330_d_n6;
        var_tmp_dn7 = assign22440_e25330_d_n7;
        var_tmp_dn8 = assign22440_e25330_d_n8;
        var_tmp_dn9 = assign22440_e25330_d_n9;

        let (assign22450_e25350, assign22450_e25350_d_n6, assign22450_e25350_d_n7, assign22450_e25350_d_n8, assign22450_e25350_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard424 == 0.0)) {
        let assign22450_e25343: f64 = (p.p845 - var_vbbt);
        let assign22450_e25345: f64 = (assign22450_e25343 * var_wdepnulrinvbot);
        let assign22450_e25347: f64 = (assign22450_e25345 / var_tmp);
        let assign22450_e25348: f64 = (var_one_over_one_minus_pbot * assign22450_e25347);
        (assign22450_e25348, (var_one_over_one_minus_pbot * (-((assign22450_e25345 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign22450_e25345 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign22450_e25345 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign22450_e25345 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign22450_e25350;
        var_fmaxr_dn6 = assign22450_e25350_d_n6;
        var_fmaxr_dn7 = assign22450_e25350_d_n7;
        var_fmaxr_dn8 = assign22450_e25350_d_n8;
        var_fmaxr_dn9 = assign22450_e25350_d_n9;

        let assign22460_e25352: f64 = (-var_fbbtbot);
        let assign22460_e25354: f64 = (assign22460_e25352 / var_fmaxr);
        let assign22460_e25355: f64 = (assign22460_e25354).abs();
        let assign22460_e25357: f64 = if assign22460_e25355 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard426 = assign22460_e25357;

        let (assign22470_e25375, assign22470_e25375_d_n6, assign22470_e25375_d_n7, assign22470_e25375_d_n8, assign22470_e25375_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard424 == 0.0)) && (var_guard426 != 0.0)) {
        let assign22470_e25370: f64 = (-var_fbbtbot);
        let assign22470_e25372: f64 = (assign22470_e25370 / var_fmaxr);
        let assign22470_e25373: f64 = (assign22470_e25372).exp();
        (assign22470_e25373, (assign22470_e25373 * (-((assign22470_e25370 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign22470_e25373 * (-((assign22470_e25370 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign22470_e25373 * (-((assign22470_e25370 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign22470_e25373 * (-((assign22470_e25370 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign22470_e25375;
        var_tmp_dn6 = assign22470_e25375_d_n6;
        var_tmp_dn7 = assign22470_e25375_d_n7;
        var_tmp_dn8 = assign22470_e25375_d_n8;
        var_tmp_dn9 = assign22470_e25375_d_n9;

        let assign22480_e25377: f64 = (-var_fbbtbot);
        let assign22480_e25379: f64 = (assign22480_e25377 / var_fmaxr);
        let assign22480_e25381: f64 = if assign22480_e25379 < 0.0 { 1.0 } else { 0.0 };
        var_guard427 = assign22480_e25381;

        let (assign22490_e25432, assign22490_e25432_d_n6, assign22490_e25432_d_n7, assign22490_e25432_d_n8, assign22490_e25432_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard424 == 0.0)) && (var_guard426 == 0.0)) && (var_guard427 != 0.0)) {
        let assign22490_e25399: f64 = (-230.25850929940458);
        let assign22490_e25401: f64 = (-var_fbbtbot);
        let assign22490_e25403: f64 = (assign22490_e25401 / var_fmaxr);
        let assign22490_e25404: f64 = (assign22490_e25399 - assign22490_e25403);
        let assign22490_e25408: f64 = (-230.25850929940458);
        let assign22490_e25410: f64 = (-var_fbbtbot);
        let assign22490_e25412: f64 = (assign22490_e25410 / var_fmaxr);
        let assign22490_e25413: f64 = (assign22490_e25408 - assign22490_e25412);
        let assign22490_e25416: f64 = (-230.25850929940458);
        let assign22490_e25418: f64 = (-var_fbbtbot);
        let assign22490_e25420: f64 = (assign22490_e25418 / var_fmaxr);
        let assign22490_e25421: f64 = (assign22490_e25416 - assign22490_e25420);
        let assign22490_e25423: f64 = (assign22490_e25421 * 0.3333333333333333);
        let assign22490_e25424: f64 = (1.0 + assign22490_e25423);
        let assign22490_e25425: f64 = (assign22490_e25413 * assign22490_e25424);
        let assign22490_e25426: f64 = (0.5 * assign22490_e25425);
        let assign22490_e25427: f64 = (1.0 + assign22490_e25426);
        let assign22490_e25428: f64 = (assign22490_e25404 * assign22490_e25427);
        let assign22490_e25429: f64 = (1.0 + assign22490_e25428);
        let assign22490_e25430: f64 = (1e-100 / assign22490_e25429);
        (assign22490_e25430, (-((1e-100 * (((-(-((assign22490_e25401 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign22490_e25427) + (assign22490_e25404 * (0.5 * (((-(-((assign22490_e25410 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign22490_e25424) + (assign22490_e25413 * ((-(-((assign22490_e25418 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign22490_e25429 * assign22490_e25429))), (-((1e-100 * (((-(-((assign22490_e25401 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign22490_e25427) + (assign22490_e25404 * (0.5 * (((-(-((assign22490_e25410 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign22490_e25424) + (assign22490_e25413 * ((-(-((assign22490_e25418 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign22490_e25429 * assign22490_e25429))), (-((1e-100 * (((-(-((assign22490_e25401 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign22490_e25427) + (assign22490_e25404 * (0.5 * (((-(-((assign22490_e25410 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign22490_e25424) + (assign22490_e25413 * ((-(-((assign22490_e25418 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign22490_e25429 * assign22490_e25429))), (-((1e-100 * (((-(-((assign22490_e25401 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign22490_e25427) + (assign22490_e25404 * (0.5 * (((-(-((assign22490_e25410 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign22490_e25424) + (assign22490_e25413 * ((-(-((assign22490_e25418 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign22490_e25429 * assign22490_e25429))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign22490_e25432;
        var_tmp_dn6 = assign22490_e25432_d_n6;
        var_tmp_dn7 = assign22490_e25432_d_n7;
        var_tmp_dn8 = assign22490_e25432_d_n8;
        var_tmp_dn9 = assign22490_e25432_d_n9;

        let (assign22500_e25481, assign22500_e25481_d_n6, assign22500_e25481_d_n7, assign22500_e25481_d_n8, assign22500_e25481_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard424 == 0.0)) && (var_guard426 == 0.0)) && (var_guard427 == 0.0)) {
        let assign22500_e25451: f64 = (-var_fbbtbot);
        let assign22500_e25453: f64 = (assign22500_e25451 / var_fmaxr);
        let assign22500_e25455: f64 = (assign22500_e25453 - 230.25850929940458);
        let assign22500_e25459: f64 = (-var_fbbtbot);
        let assign22500_e25461: f64 = (assign22500_e25459 / var_fmaxr);
        let assign22500_e25463: f64 = (assign22500_e25461 - 230.25850929940458);
        let assign22500_e25466: f64 = (-var_fbbtbot);
        let assign22500_e25468: f64 = (assign22500_e25466 / var_fmaxr);
        let assign22500_e25470: f64 = (assign22500_e25468 - 230.25850929940458);
        let assign22500_e25472: f64 = (assign22500_e25470 * 0.3333333333333333);
        let assign22500_e25473: f64 = (1.0 + assign22500_e25472);
        let assign22500_e25474: f64 = (assign22500_e25463 * assign22500_e25473);
        let assign22500_e25475: f64 = (0.5 * assign22500_e25474);
        let assign22500_e25476: f64 = (1.0 + assign22500_e25475);
        let assign22500_e25477: f64 = (assign22500_e25455 * assign22500_e25476);
        let assign22500_e25478: f64 = (1.0 + assign22500_e25477);
        let assign22500_e25479: f64 = (1e100 * assign22500_e25478);
        (assign22500_e25479, (1e100 * (((-((assign22500_e25451 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign22500_e25476) + (assign22500_e25455 * (0.5 * (((-((assign22500_e25459 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign22500_e25473) + (assign22500_e25463 * ((-((assign22500_e25466 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign22500_e25451 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign22500_e25476) + (assign22500_e25455 * (0.5 * (((-((assign22500_e25459 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign22500_e25473) + (assign22500_e25463 * ((-((assign22500_e25466 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign22500_e25451 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign22500_e25476) + (assign22500_e25455 * (0.5 * (((-((assign22500_e25459 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign22500_e25473) + (assign22500_e25463 * ((-((assign22500_e25466 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign22500_e25451 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign22500_e25476) + (assign22500_e25455 * (0.5 * (((-((assign22500_e25459 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign22500_e25473) + (assign22500_e25463 * ((-((assign22500_e25466 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign22500_e25481;
        var_tmp_dn6 = assign22500_e25481_d_n6;
        var_tmp_dn7 = assign22500_e25481_d_n7;
        var_tmp_dn8 = assign22500_e25481_d_n8;
        var_tmp_dn9 = assign22500_e25481_d_n9;

        let (assign22510_e25501, assign22510_e25501_d_n6, assign22510_e25501_d_n7, assign22510_e25501_d_n8, assign22510_e25501_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard424 == 0.0)) {
        let assign22510_e25494: f64 = (var_v4 * var_fmaxr);
        let assign22510_e25496: f64 = (assign22510_e25494 * var_fmaxr);
        let assign22510_e25498: f64 = (assign22510_e25496 * var_tmp);
        let assign22510_e25499: f64 = (p.p868 * assign22510_e25498);
        (assign22510_e25499, (p.p868 * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign22510_e25494 * var_fmaxr_dn6)) * var_tmp) + (assign22510_e25496 * var_tmp_dn6))), (p.p868 * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign22510_e25494 * var_fmaxr_dn7)) * var_tmp) + (assign22510_e25496 * var_tmp_dn7))), (p.p868 * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign22510_e25494 * var_fmaxr_dn8)) * var_tmp) + (assign22510_e25496 * var_tmp_dn8))), (p.p868 * (((((var_v4 * var_fmaxr_dn9) * var_fmaxr) + (assign22510_e25494 * var_fmaxr_dn9)) * var_tmp) + (assign22510_e25496 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign22510_e25501;
        var_ibbt_dn6 = assign22510_e25501_d_n6;
        var_ibbt_dn7 = assign22510_e25501_d_n7;
        var_ibbt_dn8 = assign22510_e25501_d_n8;
        var_ibbt_dn9 = assign22510_e25501_d_n9;

        let assign22520_e25504: f64 = if p.p877 > 1000.0 { 1.0 } else { 0.0 };
        var_guard428 = assign22520_e25504;

        let (assign22530_e25515, assign22530_e25515_d_n6, assign22530_e25515_d_n7, assign22530_e25515_d_n8, assign22530_e25515_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard428 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign22530_e25515;
        var_fbreakdown_dn6 = assign22530_e25515_d_n6;
        var_fbreakdown_dn7 = assign22530_e25515_d_n7;
        var_fbreakdown_dn8 = assign22530_e25515_d_n8;
        var_fbreakdown_dn9 = assign22530_e25515_d_n9;

        let assign22540_e25518: f64 = (-var_alphaav);
        let assign22540_e25520: f64 = (assign22540_e25518 * p.p877);
        let assign22540_e25521: f64 = if var_vav > assign22540_e25520 { 1.0 } else { 0.0 };
        var_guard429 = assign22540_e25521;

        let assign22550_e25524: f64 = if p.p880 == 4.0 { 1.0 } else { 0.0 };
        var_guard430 = assign22550_e25524;

        let (assign22560_e25554, assign22560_e25554_d_n6, assign22560_e25554_d_n7, assign22560_e25554_d_n8, assign22560_e25554_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard428 == 0.0)) && (var_guard429 != 0.0)) && (var_guard430 != 0.0)) {
        let assign22560_e25540: f64 = (var_vav * var_vbrinvbot);
        let assign22560_e25543: f64 = (var_vav * var_vbrinvbot);
        let assign22560_e25544: f64 = (assign22560_e25540 * assign22560_e25543);
        let assign22560_e25547: f64 = (var_vav * var_vbrinvbot);
        let assign22560_e25548: f64 = (assign22560_e25544 * assign22560_e25547);
        let assign22560_e25551: f64 = (var_vav * var_vbrinvbot);
        let assign22560_e25552: f64 = (assign22560_e25548 * assign22560_e25551);
        (assign22560_e25552, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign22560_e25554;
        var_tmp_dn6 = assign22560_e25554_d_n6;
        var_tmp_dn7 = assign22560_e25554_d_n7;
        var_tmp_dn8 = assign22560_e25554_d_n8;
        var_tmp_dn9 = assign22560_e25554_d_n9;

        let (assign22570_e25576, assign22570_e25576_d_n6, assign22570_e25576_d_n7, assign22570_e25576_d_n8, assign22570_e25576_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard428 == 0.0)) && (var_guard429 != 0.0)) && (var_guard430 == 0.0)) {
        let assign22570_e25571: f64 = (var_vav * var_vbrinvbot);
        let assign22570_e25572: f64 = (assign22570_e25571).abs();
        let assign22570_e25574: f64 = (assign22570_e25572).powf(p.p880);
        (assign22570_e25574, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign22570_e25576;
        var_tmp_dn6 = assign22570_e25576_d_n6;
        var_tmp_dn7 = assign22570_e25576_d_n7;
        var_tmp_dn8 = assign22570_e25576_d_n8;
        var_tmp_dn9 = assign22570_e25576_d_n9;

        let (assign22580_e25594, assign22580_e25594_d_n6, assign22580_e25594_d_n7, assign22580_e25594_d_n8, assign22580_e25594_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard428 == 0.0)) && (var_guard429 != 0.0)) {
        let assign22580_e25591: f64 = (1.0 - var_tmp);
        let assign22580_e25592: f64 = (1.0 / assign22580_e25591);
        (assign22580_e25592, (-((-var_tmp_dn6) / (assign22580_e25591 * assign22580_e25591))), (-((-var_tmp_dn7) / (assign22580_e25591 * assign22580_e25591))), (-((-var_tmp_dn8) / (assign22580_e25591 * assign22580_e25591))), (-((-var_tmp_dn9) / (assign22580_e25591 * assign22580_e25591))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign22580_e25594;
        var_fbreakdown_dn6 = assign22580_e25594_d_n6;
        var_fbreakdown_dn7 = assign22580_e25594_d_n7;
        var_fbreakdown_dn8 = assign22580_e25594_d_n8;
        var_fbreakdown_dn9 = assign22580_e25594_d_n9;

        let (assign22590_e25617, assign22590_e25617_d_n6, assign22590_e25617_d_n7, assign22590_e25617_d_n8, assign22590_e25617_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) && (var_guard428 == 0.0)) && (var_guard429 == 0.0)) {
        let assign22590_e25611: f64 = (var_alphaav * p.p877);
        let assign22590_e25612: f64 = (var_vav + assign22590_e25611);
        let assign22590_e25614: f64 = (assign22590_e25612 * var_slopebot);
        let assign22590_e25615: f64 = (var_fstopbot + assign22590_e25614);
        (assign22590_e25615, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign22590_e25617;
        var_fbreakdown_dn6 = assign22590_e25617_d_n6;
        var_fbreakdown_dn7 = assign22590_e25617_d_n7;
        var_fbreakdown_dn8 = assign22590_e25617_d_n8;
        var_fbreakdown_dn9 = assign22590_e25617_d_n9;

        let (assign22600_e25636, assign22600_e25636_d_n6, assign22600_e25636_d_n7, assign22600_e25636_d_n8, assign22600_e25636_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard414 == 0.0)) {
        let assign22600_e25627: f64 = (var_id__blk212 + var_isrh);
        let assign22600_e25629: f64 = (assign22600_e25627 + var_itat);
        let assign22600_e25631: f64 = (assign22600_e25629 + var_ibbt);
        let assign22600_e25632: f64 = (p.p29 * assign22600_e25631);
        let assign22600_e25634: f64 = (assign22600_e25632 * var_fbreakdown);
        (assign22600_e25634, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign22600_e25632 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign22600_e25632 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign22600_e25632 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign22600_e25632 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign22600_e25636;
        var_ijunbot_dn6 = assign22600_e25636_d_n6;
        var_ijunbot_dn7 = assign22600_e25636_d_n7;
        var_ijunbot_dn8 = assign22600_e25636_d_n8;
        var_ijunbot_dn9 = assign22600_e25636_d_n9;

        let assign22610_e25639: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard431 = assign22610_e25639;

        let (assign22620_e25647, assign22620_e25647_d_n6, assign22620_e25647_d_n7, assign22620_e25647_d_n8, assign22620_e25647_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign22620_e25647;
        var_ijunsti_dn6 = assign22620_e25647_d_n6;
        var_ijunsti_dn7 = assign22620_e25647_d_n7;
        var_ijunsti_dn8 = assign22620_e25647_d_n8;
        var_ijunsti_dn9 = assign22620_e25647_d_n9;

        let (assign22630_e25658,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) {
        let assign22630_e25656: f64 = (var_idsatsti * var_idmult);
        (assign22630_e25656,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign22630_e25658;

        let assign22640_e25665: f64 = if ((p.p858 == 0.0) && (p.p863 == 0.0)) { 1.0 } else { 0.0 };
        var_guard432 = assign22640_e25665;

        let (assign22650_e25676, assign22650_e25676_d_n6, assign22650_e25676_d_n7, assign22650_e25676_d_n8, assign22650_e25676_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard432 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign22650_e25676;
        var_isrh_dn6 = assign22650_e25676_d_n6;
        var_isrh_dn7 = assign22650_e25676_d_n7;
        var_isrh_dn8 = assign22650_e25676_d_n8;
        var_isrh_dn9 = assign22650_e25676_d_n9;

        let (assign22660_e25690,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard432 == 0.0)) {
        let assign22660_e25688: f64 = (var_vbisti - var_vjsrh);
        (assign22660_e25688,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign22660_e25690;

        let (assign22670_e25709,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard432 == 0.0)) {
        let assign22670_e25704: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign22670_e25705: f64 = (1.0 - assign22670_e25704);
        let assign22670_e25706: f64 = (assign22670_e25705).sqrt();
        let assign22670_e25707: f64 = (1.0 - assign22670_e25706);
        (assign22670_e25707,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign22670_e25709;

        let assign22680_e25712: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };
        var_guard433 = assign22680_e25712;

        let (assign22690_e25726,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard432 == 0.0)) && (var_guard433 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign22690_e25726;

        let (assign22700_e25758,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard432 == 0.0)) && (var_guard433 == 0.0)) {
        let assign22700_e25741: f64 = (var_wsrhstep * var_wsrhstep);
        let assign22700_e25743: f64 = (var_wsrhstep).ln();
        let assign22700_e25744: f64 = (assign22700_e25741 * assign22700_e25743);
        let assign22700_e25747: f64 = (1.0 - var_wsrhstep);
        let assign22700_e25748: f64 = (assign22700_e25744 / assign22700_e25747);
        let assign22700_e25750: f64 = (assign22700_e25748 + var_wsrhstep);
        let assign22700_e25754: f64 = (2.0 * p.p849);
        let assign22700_e25755: f64 = (1.0 - assign22700_e25754);
        let assign22700_e25756: f64 = (assign22700_e25750 * assign22700_e25755);
        (assign22700_e25756,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign22700_e25758;

        *var_dwsrh_slot = var_dwsrh;
        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfcpos_dn9_slot = var_erfcpos_dn9;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_erfctimesexpmtat_dn9_slot = var_erfctimesexpmtat_dn9;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_fmaxr_dn9_slot = var_fmaxr_dn9;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_gammamax_dn9_slot = var_gammamax_dn9;
        *var_guard422_slot = var_guard422;
        *var_guard423_slot = var_guard423;
        *var_guard424_slot = var_guard424;
        *var_guard425_slot = var_guard425;
        *var_guard426_slot = var_guard426;
        *var_guard427_slot = var_guard427;
        *var_guard428_slot = var_guard428;
        *var_guard429_slot = var_guard429;
        *var_guard430_slot = var_guard430;
        *var_guard431_slot = var_guard431;
        *var_guard432_slot = var_guard432;
        *var_guard433_slot = var_guard433;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_id__blk212_slot = var_id__blk212;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijunbot_dn9_slot = var_ijunbot_dn9;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_ijunsti_dn9_slot = var_ijunsti_dn9;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wsrhstep_slot = var_wsrhstep;
    }

    pub(super) fn stamp_transient_block_42(
        p: &Parameters,
        var_atatsti: f64,
        var_berfc: f64,
        var_btatpartsti: f64,
        var_cerfc: f64,
        var_dwsrh: f64,
        var_ftdsti: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard431: f64,
        var_guard432: f64,
        var_one_minus_psti: f64,
        var_one_over_one_minus_psti: f64,
        var_perfc: f64,
        var_vbi_minus_vjsrh: f64,
        var_vbirstiinv: f64,
        var_wdepnulrsti: f64,
        var_wsrhstep: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_asrh_dn9_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_btat_dn9_slot: &mut f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfcpos_dn9_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_erfctimesexpmtat_dn9_slot: &mut f64,
        var_guard434_slot: &mut f64,
        var_guard435_slot: &mut f64,
        var_guard436_slot: &mut f64,
        var_guard437_slot: &mut f64,
        var_guard438_slot: &mut f64,
        var_guard439_slot: &mut f64,
        var_guard440_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ktat_dn9_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_ltat_dn9_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_mtat_dn9_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_sqrtumax_dn9_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_terfc_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_dn9_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umax_dn9_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxbeforelimiting_dn9_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_dn9_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wdep_dn9_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wgamma_dn9_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_wtat_dn9_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_xerfc_dn9_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
        var_ysq_dn9_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_asrh_dn9: f64 = *var_asrh_dn9_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_btat_dn9: f64 = *var_btat_dn9_slot;
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfcpos_dn9: f64 = *var_erfcpos_dn9_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_erfctimesexpmtat_dn9: f64 = *var_erfctimesexpmtat_dn9_slot;
        let mut var_guard434: f64 = *var_guard434_slot;
        let mut var_guard435: f64 = *var_guard435_slot;
        let mut var_guard436: f64 = *var_guard436_slot;
        let mut var_guard437: f64 = *var_guard437_slot;
        let mut var_guard438: f64 = *var_guard438_slot;
        let mut var_guard439: f64 = *var_guard439_slot;
        let mut var_guard440: f64 = *var_guard440_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ktat_dn9: f64 = *var_ktat_dn9_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_ltat_dn9: f64 = *var_ltat_dn9_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_mtat_dn9: f64 = *var_mtat_dn9_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_sqrtumax_dn9: f64 = *var_sqrtumax_dn9_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_terfc_dn9: f64 = *var_terfc_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_twoatatoverthreebtat_dn9: f64 = *var_twoatatoverthreebtat_dn9_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umax_dn9: f64 = *var_umax_dn9_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxbeforelimiting_dn9: f64 = *var_umaxbeforelimiting_dn9_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_umaxpoweronepointfive_dn9: f64 = *var_umaxpoweronepointfive_dn9_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wdep_dn9: f64 = *var_wdep_dn9_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wgamma_dn9: f64 = *var_wgamma_dn9_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_wtat_dn9: f64 = *var_wtat_dn9_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_xerfc_dn9: f64 = *var_xerfc_dn9_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;
        let mut var_ysq_dn9: f64 = *var_ysq_dn9_slot;

        let (assign22710_e25772,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard432 == 0.0)) {
        let assign22710_e25770: f64 = (var_wsrhstep + var_dwsrh);
        (assign22710_e25770,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign22710_e25772;

        let assign22720_e25775: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };
        var_guard434 = assign22720_e25775;

        let (assign22730_e25792, assign22730_e25792_d_n6, assign22730_e25792_d_n7, assign22730_e25792_d_n8, assign22730_e25792_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard432 == 0.0)) && (var_guard434 != 0.0)) {
        let assign22730_e25789: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign22730_e25790: f64 = (assign22730_e25789).sqrt();
        (assign22730_e25790, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign22730_e25792;
        var_tmp_dn6 = assign22730_e25792_d_n6;
        var_tmp_dn7 = assign22730_e25792_d_n7;
        var_tmp_dn8 = assign22730_e25792_d_n8;
        var_tmp_dn9 = assign22730_e25792_d_n9;

        let (assign22740_e25811, assign22740_e25811_d_n6, assign22740_e25811_d_n7, assign22740_e25811_d_n8, assign22740_e25811_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard432 == 0.0)) && (var_guard434 == 0.0)) {
        let assign22740_e25807: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign22740_e25809: f64 = (assign22740_e25807).powf(p.p849);
        (assign22740_e25809, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign22740_e25811;
        var_tmp_dn6 = assign22740_e25811_d_n6;
        var_tmp_dn7 = assign22740_e25811_d_n7;
        var_tmp_dn8 = assign22740_e25811_d_n8;
        var_tmp_dn9 = assign22740_e25811_d_n9;

        let (assign22750_e25825, assign22750_e25825_d_n6, assign22750_e25825_d_n7, assign22750_e25825_d_n8, assign22750_e25825_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard432 == 0.0)) {
        let assign22750_e25823: f64 = (var_wdepnulrsti * var_tmp);
        (assign22750_e25823, (var_wdepnulrsti * var_tmp_dn6), (var_wdepnulrsti * var_tmp_dn7), (var_wdepnulrsti * var_tmp_dn8), (var_wdepnulrsti * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign22750_e25825;
        var_wdep_dn6 = assign22750_e25825_d_n6;
        var_wdep_dn7 = assign22750_e25825_d_n7;
        var_wdep_dn8 = assign22750_e25825_d_n8;
        var_wdep_dn9 = assign22750_e25825_d_n9;

        let (assign22760_e25843, assign22760_e25843_d_n6, assign22760_e25843_d_n7, assign22760_e25843_d_n8, assign22760_e25843_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard432 == 0.0)) {
        let assign22760_e25838: f64 = (var_zinv - 1.0);
        let assign22760_e25840: f64 = (assign22760_e25838 * var_wdep);
        let assign22760_e25841: f64 = (var_ftdsti * assign22760_e25840);
        (assign22760_e25841, (var_ftdsti * (assign22760_e25838 * var_wdep_dn6)), (var_ftdsti * (assign22760_e25838 * var_wdep_dn7)), (var_ftdsti * (assign22760_e25838 * var_wdep_dn8)), (var_ftdsti * (assign22760_e25838 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign22760_e25843;
        var_asrh_dn6 = assign22760_e25843_d_n6;
        var_asrh_dn7 = assign22760_e25843_d_n7;
        var_asrh_dn8 = assign22760_e25843_d_n8;
        var_asrh_dn9 = assign22760_e25843_d_n9;

        let (assign22770_e25859, assign22770_e25859_d_n6, assign22770_e25859_d_n7, assign22770_e25859_d_n8, assign22770_e25859_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard432 == 0.0)) {
        let assign22770_e25856: f64 = (var_asrh * var_wsrh);
        let assign22770_e25857: f64 = (p.p858 * assign22770_e25856);
        (assign22770_e25857, (p.p858 * (var_asrh_dn6 * var_wsrh)), (p.p858 * (var_asrh_dn7 * var_wsrh)), (p.p858 * (var_asrh_dn8 * var_wsrh)), (p.p858 * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign22770_e25859;
        var_isrh_dn6 = assign22770_e25859_d_n6;
        var_isrh_dn7 = assign22770_e25859_d_n7;
        var_isrh_dn8 = assign22770_e25859_d_n8;
        var_isrh_dn9 = assign22770_e25859_d_n9;

        let assign22780_e25862: f64 = if p.p863 == 0.0 { 1.0 } else { 0.0 };
        var_guard435 = assign22780_e25862;

        let (assign22790_e25873, assign22790_e25873_d_n6, assign22790_e25873_d_n7, assign22790_e25873_d_n8, assign22790_e25873_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign22790_e25873;
        var_itat_dn6 = assign22790_e25873_d_n6;
        var_itat_dn7 = assign22790_e25873_d_n7;
        var_itat_dn8 = assign22790_e25873_d_n8;
        var_itat_dn9 = assign22790_e25873_d_n9;

        let (assign22800_e25891, assign22800_e25891_d_n6, assign22800_e25891_d_n7, assign22800_e25891_d_n8, assign22800_e25891_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) {
        let assign22800_e25886: f64 = (var_wdep * var_one_minus_psti);
        let assign22800_e25888: f64 = (assign22800_e25886 / var_vbi_minus_vjsrh);
        let assign22800_e25889: f64 = (var_btatpartsti * assign22800_e25888);
        (assign22800_e25889, (var_btatpartsti * ((var_wdep_dn6 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn7 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn8 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn9 * var_one_minus_psti) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign22800_e25891;
        var_btat_dn6 = assign22800_e25891_d_n6;
        var_btat_dn7 = assign22800_e25891_d_n7;
        var_btat_dn8 = assign22800_e25891_d_n8;
        var_btat_dn9 = assign22800_e25891_d_n9;

        let (assign22810_e25907, assign22810_e25907_d_n6, assign22810_e25907_d_n7, assign22810_e25907_d_n8, assign22810_e25907_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) {
        let assign22810_e25903: f64 = (0.666666666666667 * var_atatsti);
        let assign22810_e25905: f64 = (assign22810_e25903 / var_btat);
        (assign22810_e25905, (-((assign22810_e25903 * var_btat_dn6) / (var_btat * var_btat))), (-((assign22810_e25903 * var_btat_dn7) / (var_btat * var_btat))), (-((assign22810_e25903 * var_btat_dn8) / (var_btat * var_btat))), (-((assign22810_e25903 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign22810_e25907;
        var_twoatatoverthreebtat_dn6 = assign22810_e25907_d_n6;
        var_twoatatoverthreebtat_dn7 = assign22810_e25907_d_n7;
        var_twoatatoverthreebtat_dn8 = assign22810_e25907_d_n8;
        var_twoatatoverthreebtat_dn9 = assign22810_e25907_d_n9;

        let (assign22820_e25921, assign22820_e25921_d_n6, assign22820_e25921_d_n7, assign22820_e25921_d_n8, assign22820_e25921_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) {
        let assign22820_e25919: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign22820_e25919, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign22820_e25921;
        var_umaxbeforelimiting_dn6 = assign22820_e25921_d_n6;
        var_umaxbeforelimiting_dn7 = assign22820_e25921_d_n7;
        var_umaxbeforelimiting_dn8 = assign22820_e25921_d_n8;
        var_umaxbeforelimiting_dn9 = assign22820_e25921_d_n9;

        let (assign22830_e25942, assign22830_e25942_d_n6, assign22830_e25942_d_n7, assign22830_e25942_d_n8, assign22830_e25942_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) {
        let assign22830_e25933: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign22830_e25936: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign22830_e25938: f64 = (assign22830_e25936 + 1.0);
        let assign22830_e25939: f64 = (assign22830_e25933 / assign22830_e25938);
        let assign22830_e25940: f64 = (assign22830_e25939).sqrt();
        (assign22830_e25940, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign22830_e25938) - (assign22830_e25933 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign22830_e25938 * assign22830_e25938)) / (2.0 * assign22830_e25940)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign22830_e25938) - (assign22830_e25933 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign22830_e25938 * assign22830_e25938)) / (2.0 * assign22830_e25940)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign22830_e25938) - (assign22830_e25933 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign22830_e25938 * assign22830_e25938)) / (2.0 * assign22830_e25940)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign22830_e25938) - (assign22830_e25933 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign22830_e25938 * assign22830_e25938)) / (2.0 * assign22830_e25940)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign22830_e25942;
        var_umax_dn6 = assign22830_e25942_d_n6;
        var_umax_dn7 = assign22830_e25942_d_n7;
        var_umax_dn8 = assign22830_e25942_d_n8;
        var_umax_dn9 = assign22830_e25942_d_n9;

        let (assign22840_e25955, assign22840_e25955_d_n6, assign22840_e25955_d_n7, assign22840_e25955_d_n8, assign22840_e25955_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) {
        let assign22840_e25953: f64 = (var_umax).sqrt();
        (assign22840_e25953, (var_umax_dn6 / (2.0 * assign22840_e25953)), (var_umax_dn7 / (2.0 * assign22840_e25953)), (var_umax_dn8 / (2.0 * assign22840_e25953)), (var_umax_dn9 / (2.0 * assign22840_e25953)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign22840_e25955;
        var_sqrtumax_dn6 = assign22840_e25955_d_n6;
        var_sqrtumax_dn7 = assign22840_e25955_d_n7;
        var_sqrtumax_dn8 = assign22840_e25955_d_n8;
        var_sqrtumax_dn9 = assign22840_e25955_d_n9;

        let (assign22850_e25969, assign22850_e25969_d_n6, assign22850_e25969_d_n7, assign22850_e25969_d_n8, assign22850_e25969_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) {
        let assign22850_e25967: f64 = (var_umax * var_sqrtumax);
        (assign22850_e25967, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign22850_e25969;
        var_umaxpoweronepointfive_dn6 = assign22850_e25969_d_n6;
        var_umaxpoweronepointfive_dn7 = assign22850_e25969_d_n7;
        var_umaxpoweronepointfive_dn8 = assign22850_e25969_d_n8;
        var_umaxpoweronepointfive_dn9 = assign22850_e25969_d_n9;

        let assign22860_e25971: f64 = (-p.p849);
        let assign22860_e25973: f64 = (assign22860_e25971 * var_one_over_one_minus_psti);
        let assign22860_e25975: f64 = (-1.0);
        let assign22860_e25976: f64 = if assign22860_e25973 == assign22860_e25975 { 1.0 } else { 0.0 };
        var_guard436 = assign22860_e25976;

        let (assign22870_e25996, assign22870_e25996_d_n6, assign22870_e25996_d_n7, assign22870_e25996_d_n8, assign22870_e25996_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) && (var_guard436 != 0.0)) {
        let assign22870_e25992: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign22870_e25993: f64 = (1.0 + assign22870_e25992);
        let assign22870_e25994: f64 = (1.0 / assign22870_e25993);
        (assign22870_e25994, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign22870_e25993 * assign22870_e25993))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign22870_e25993 * assign22870_e25993))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign22870_e25993 * assign22870_e25993))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign22870_e25993 * assign22870_e25993))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign22870_e25996;
        var_wgamma_dn6 = assign22870_e25996_d_n6;
        var_wgamma_dn7 = assign22870_e25996_d_n7;
        var_wgamma_dn8 = assign22870_e25996_d_n8;
        var_wgamma_dn9 = assign22870_e25996_d_n9;

        let (assign22880_e26020, assign22880_e26020_d_n6, assign22880_e26020_d_n7, assign22880_e26020_d_n8, assign22880_e26020_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) && (var_guard436 == 0.0)) {
        let assign22880_e26012: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign22880_e26013: f64 = (1.0 + assign22880_e26012);
        let assign22880_e26015: f64 = (-p.p849);
        let assign22880_e26017: f64 = (assign22880_e26015 * var_one_over_one_minus_psti);
        let assign22880_e26018: f64 = (assign22880_e26013).powf(assign22880_e26017);
        (assign22880_e26018, if 0.0 == 0.0 && ((assign22880_e26017) as f64).is_finite() && ((assign22880_e26017) as f64).fract() == 0.0 { if assign22880_e26017 == 0.0 { 0.0 } else { (assign22880_e26017 * ((assign22880_e26013).powf(assign22880_e26017 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign22880_e26018 * (assign22880_e26017 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign22880_e26013))) }, if 0.0 == 0.0 && ((assign22880_e26017) as f64).is_finite() && ((assign22880_e26017) as f64).fract() == 0.0 { if assign22880_e26017 == 0.0 { 0.0 } else { (assign22880_e26017 * ((assign22880_e26013).powf(assign22880_e26017 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign22880_e26018 * (assign22880_e26017 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign22880_e26013))) }, if 0.0 == 0.0 && ((assign22880_e26017) as f64).is_finite() && ((assign22880_e26017) as f64).fract() == 0.0 { if assign22880_e26017 == 0.0 { 0.0 } else { (assign22880_e26017 * ((assign22880_e26013).powf(assign22880_e26017 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign22880_e26018 * (assign22880_e26017 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign22880_e26013))) }, if 0.0 == 0.0 && ((assign22880_e26017) as f64).is_finite() && ((assign22880_e26017) as f64).fract() == 0.0 { if assign22880_e26017 == 0.0 { 0.0 } else { (assign22880_e26017 * ((assign22880_e26013).powf(assign22880_e26017 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign22880_e26018 * (assign22880_e26017 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign22880_e26013))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign22880_e26020;
        var_wgamma_dn6 = assign22880_e26020_d_n6;
        var_wgamma_dn7 = assign22880_e26020_d_n7;
        var_wgamma_dn8 = assign22880_e26020_d_n8;
        var_wgamma_dn9 = assign22880_e26020_d_n9;

        let (assign22890_e26038, assign22890_e26038_d_n6, assign22890_e26038_d_n7, assign22890_e26038_d_n8, assign22890_e26038_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) {
        let assign22890_e26032: f64 = (var_wsrh * var_wgamma);
        let assign22890_e26035: f64 = (var_wsrh + var_wgamma);
        let assign22890_e26036: f64 = (assign22890_e26032 / assign22890_e26035);
        (assign22890_e26036, ((((var_wsrh * var_wgamma_dn6) * assign22890_e26035) - (assign22890_e26032 * var_wgamma_dn6)) / (assign22890_e26035 * assign22890_e26035)), ((((var_wsrh * var_wgamma_dn7) * assign22890_e26035) - (assign22890_e26032 * var_wgamma_dn7)) / (assign22890_e26035 * assign22890_e26035)), ((((var_wsrh * var_wgamma_dn8) * assign22890_e26035) - (assign22890_e26032 * var_wgamma_dn8)) / (assign22890_e26035 * assign22890_e26035)), ((((var_wsrh * var_wgamma_dn9) * assign22890_e26035) - (assign22890_e26032 * var_wgamma_dn9)) / (assign22890_e26035 * assign22890_e26035)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign22890_e26038;
        var_wtat_dn6 = assign22890_e26038_d_n6;
        var_wtat_dn7 = assign22890_e26038_d_n7;
        var_wtat_dn8 = assign22890_e26038_d_n8;
        var_wtat_dn9 = assign22890_e26038_d_n9;

        let (assign22900_e26055, assign22900_e26055_d_n6, assign22900_e26055_d_n7, assign22900_e26055_d_n8, assign22900_e26055_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) {
        let assign22900_e26051: f64 = (var_btat / var_sqrtumax);
        let assign22900_e26052: f64 = (0.375 * assign22900_e26051);
        let assign22900_e26053: f64 = (assign22900_e26052).sqrt();
        (assign22900_e26053, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign22900_e26053)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign22900_e26053)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign22900_e26053)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign22900_e26053)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign22900_e26055;
        var_ktat_dn6 = assign22900_e26055_d_n6;
        var_ktat_dn7 = assign22900_e26055_d_n7;
        var_ktat_dn8 = assign22900_e26055_d_n8;
        var_ktat_dn9 = assign22900_e26055_d_n9;

        let (assign22910_e26073, assign22910_e26073_d_n6, assign22910_e26073_d_n7, assign22910_e26073_d_n8, assign22910_e26073_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) {
        let assign22910_e26068: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign22910_e26069: f64 = (2.0 * assign22910_e26068);
        let assign22910_e26071: f64 = (assign22910_e26069 - var_umax);
        (assign22910_e26071, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign22910_e26073;
        var_ltat_dn6 = assign22910_e26073_d_n6;
        var_ltat_dn7 = assign22910_e26073_d_n7;
        var_ltat_dn8 = assign22910_e26073_d_n8;
        var_ltat_dn9 = assign22910_e26073_d_n9;

        let (assign22920_e26099, assign22920_e26099_d_n6, assign22920_e26099_d_n7, assign22920_e26099_d_n8, assign22920_e26099_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) {
        let assign22920_e26085: f64 = (var_atatsti * var_twoatatoverthreebtat);
        let assign22920_e26087: f64 = (assign22920_e26085 * var_sqrtumax);
        let assign22920_e26090: f64 = (var_atatsti * var_umax);
        let assign22920_e26091: f64 = (assign22920_e26087 - assign22920_e26090);
        let assign22920_e26095: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign22920_e26096: f64 = (0.5 * assign22920_e26095);
        let assign22920_e26097: f64 = (assign22920_e26091 + assign22920_e26096);
        (assign22920_e26097, (((((var_atatsti * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign22920_e26085 * var_sqrtumax_dn6)) - (var_atatsti * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign22920_e26085 * var_sqrtumax_dn7)) - (var_atatsti * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign22920_e26085 * var_sqrtumax_dn8)) - (var_atatsti * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatsti * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign22920_e26085 * var_sqrtumax_dn9)) - (var_atatsti * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign22920_e26099;
        var_mtat_dn6 = assign22920_e26099_d_n6;
        var_mtat_dn7 = assign22920_e26099_d_n7;
        var_mtat_dn8 = assign22920_e26099_d_n8;
        var_mtat_dn9 = assign22920_e26099_d_n9;

        let (assign22930_e26115, assign22930_e26115_d_n6, assign22930_e26115_d_n7, assign22930_e26115_d_n8, assign22930_e26115_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) {
        let assign22930_e26111: f64 = (var_ltat - 1.0);
        let assign22930_e26113: f64 = (assign22930_e26111 * var_ktat);
        (assign22930_e26113, ((var_ltat_dn6 * var_ktat) + (assign22930_e26111 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign22930_e26111 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign22930_e26111 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign22930_e26111 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign22930_e26115;
        var_xerfc_dn6 = assign22930_e26115_d_n6;
        var_xerfc_dn7 = assign22930_e26115_d_n7;
        var_xerfc_dn8 = assign22930_e26115_d_n8;
        var_xerfc_dn9 = assign22930_e26115_d_n9;

        let (assign22940_e26129, assign22940_e26129_d_n6, assign22940_e26129_d_n7, assign22940_e26129_d_n8, assign22940_e26129_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) {
        let assign22940_e26127: f64 = (var_xerfc * var_xerfc);
        (assign22940_e26127, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign22940_e26129;
        var_ysq_dn6 = assign22940_e26129_d_n6;
        var_ysq_dn7 = assign22940_e26129_d_n7;
        var_ysq_dn8 = assign22940_e26129_d_n8;
        var_ysq_dn9 = assign22940_e26129_d_n9;

        let assign22950_e26132: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard437 = assign22950_e26132;

        let (assign22960_e26152, assign22960_e26152_d_n6, assign22960_e26152_d_n7, assign22960_e26152_d_n8, assign22960_e26152_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) && (var_guard437 != 0.0)) {
        let assign22960_e26148: f64 = (var_perfc * var_xerfc);
        let assign22960_e26149: f64 = (1.0 + assign22960_e26148);
        let assign22960_e26150: f64 = (1.0 / assign22960_e26149);
        (assign22960_e26150, (-((var_perfc * var_xerfc_dn6) / (assign22960_e26149 * assign22960_e26149))), (-((var_perfc * var_xerfc_dn7) / (assign22960_e26149 * assign22960_e26149))), (-((var_perfc * var_xerfc_dn8) / (assign22960_e26149 * assign22960_e26149))), (-((var_perfc * var_xerfc_dn9) / (assign22960_e26149 * assign22960_e26149))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign22960_e26152;
        var_terfc_dn6 = assign22960_e26152_d_n6;
        var_terfc_dn7 = assign22960_e26152_d_n7;
        var_terfc_dn8 = assign22960_e26152_d_n8;
        var_terfc_dn9 = assign22960_e26152_d_n9;

        let (assign22970_e26173, assign22970_e26173_d_n6, assign22970_e26173_d_n7, assign22970_e26173_d_n8, assign22970_e26173_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) && (var_guard437 == 0.0)) {
        let assign22970_e26169: f64 = (var_perfc * var_xerfc);
        let assign22970_e26170: f64 = (1.0 - assign22970_e26169);
        let assign22970_e26171: f64 = (1.0 / assign22970_e26170);
        (assign22970_e26171, (-((-(var_perfc * var_xerfc_dn6)) / (assign22970_e26170 * assign22970_e26170))), (-((-(var_perfc * var_xerfc_dn7)) / (assign22970_e26170 * assign22970_e26170))), (-((-(var_perfc * var_xerfc_dn8)) / (assign22970_e26170 * assign22970_e26170))), (-((-(var_perfc * var_xerfc_dn9)) / (assign22970_e26170 * assign22970_e26170))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign22970_e26173;
        var_terfc_dn6 = assign22970_e26173_d_n6;
        var_terfc_dn7 = assign22970_e26173_d_n7;
        var_terfc_dn8 = assign22970_e26173_d_n8;
        var_terfc_dn9 = assign22970_e26173_d_n9;

        let assign22980_e26175: f64 = (-var_ysq);
        let assign22980_e26177: f64 = (assign22980_e26175 + var_mtat);
        let assign22980_e26179: f64 = (-230.25850929940458);
        let assign22980_e26180: f64 = if assign22980_e26177 > assign22980_e26179 { 1.0 } else { 0.0 };
        var_guard438 = assign22980_e26180;

        let (assign22990_e26198, assign22990_e26198_d_n6, assign22990_e26198_d_n7, assign22990_e26198_d_n8, assign22990_e26198_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) && (var_guard438 != 0.0)) {
        let assign22990_e26193: f64 = (-var_ysq);
        let assign22990_e26195: f64 = (assign22990_e26193 + var_mtat);
        let assign22990_e26196: f64 = (assign22990_e26195).exp();
        (assign22990_e26196, (assign22990_e26196 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign22990_e26196 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign22990_e26196 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign22990_e26196 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign22990_e26198;
        var_tmp_dn6 = assign22990_e26198_d_n6;
        var_tmp_dn7 = assign22990_e26198_d_n7;
        var_tmp_dn8 = assign22990_e26198_d_n8;
        var_tmp_dn9 = assign22990_e26198_d_n9;

        let (assign23000_e26247, assign23000_e26247_d_n6, assign23000_e26247_d_n7, assign23000_e26247_d_n8, assign23000_e26247_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) && (var_guard438 == 0.0)) {
        let assign23000_e26214: f64 = (-230.25850929940458);
        let assign23000_e26216: f64 = (-var_ysq);
        let assign23000_e26218: f64 = (assign23000_e26216 + var_mtat);
        let assign23000_e26219: f64 = (assign23000_e26214 - assign23000_e26218);
        let assign23000_e26223: f64 = (-230.25850929940458);
        let assign23000_e26225: f64 = (-var_ysq);
        let assign23000_e26227: f64 = (assign23000_e26225 + var_mtat);
        let assign23000_e26228: f64 = (assign23000_e26223 - assign23000_e26227);
        let assign23000_e26231: f64 = (-230.25850929940458);
        let assign23000_e26233: f64 = (-var_ysq);
        let assign23000_e26235: f64 = (assign23000_e26233 + var_mtat);
        let assign23000_e26236: f64 = (assign23000_e26231 - assign23000_e26235);
        let assign23000_e26238: f64 = (assign23000_e26236 * 0.3333333333333333);
        let assign23000_e26239: f64 = (1.0 + assign23000_e26238);
        let assign23000_e26240: f64 = (assign23000_e26228 * assign23000_e26239);
        let assign23000_e26241: f64 = (0.5 * assign23000_e26240);
        let assign23000_e26242: f64 = (1.0 + assign23000_e26241);
        let assign23000_e26243: f64 = (assign23000_e26219 * assign23000_e26242);
        let assign23000_e26244: f64 = (1.0 + assign23000_e26243);
        let assign23000_e26245: f64 = (1e-100 / assign23000_e26244);
        (assign23000_e26245, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign23000_e26242) + (assign23000_e26219 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign23000_e26239) + (assign23000_e26228 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign23000_e26244 * assign23000_e26244))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign23000_e26242) + (assign23000_e26219 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign23000_e26239) + (assign23000_e26228 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign23000_e26244 * assign23000_e26244))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign23000_e26242) + (assign23000_e26219 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign23000_e26239) + (assign23000_e26228 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign23000_e26244 * assign23000_e26244))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign23000_e26242) + (assign23000_e26219 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign23000_e26239) + (assign23000_e26228 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign23000_e26244 * assign23000_e26244))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23000_e26247;
        var_tmp_dn6 = assign23000_e26247_d_n6;
        var_tmp_dn7 = assign23000_e26247_d_n7;
        var_tmp_dn8 = assign23000_e26247_d_n8;
        var_tmp_dn9 = assign23000_e26247_d_n9;

        let (assign23010_e26277, assign23010_e26277_d_n6, assign23010_e26277_d_n7, assign23010_e26277_d_n8, assign23010_e26277_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) {
        let assign23010_e26259: f64 = (0.29214664 * var_terfc);
        let assign23010_e26263: f64 = (var_terfc * var_terfc);
        let assign23010_e26264: f64 = (var_berfc * assign23010_e26263);
        let assign23010_e26265: f64 = (assign23010_e26259 + assign23010_e26264);
        let assign23010_e26269: f64 = (var_terfc * var_terfc);
        let assign23010_e26271: f64 = (assign23010_e26269 * var_terfc);
        let assign23010_e26272: f64 = (var_cerfc * assign23010_e26271);
        let assign23010_e26273: f64 = (assign23010_e26265 + assign23010_e26272);
        let assign23010_e26275: f64 = (assign23010_e26273 * var_tmp);
        (assign23010_e26275, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign23010_e26269 * var_terfc_dn6)))) * var_tmp) + (assign23010_e26273 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign23010_e26269 * var_terfc_dn7)))) * var_tmp) + (assign23010_e26273 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign23010_e26269 * var_terfc_dn8)))) * var_tmp) + (assign23010_e26273 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign23010_e26269 * var_terfc_dn9)))) * var_tmp) + (assign23010_e26273 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign23010_e26277;
        var_erfcpos_dn6 = assign23010_e26277_d_n6;
        var_erfcpos_dn7 = assign23010_e26277_d_n7;
        var_erfcpos_dn8 = assign23010_e26277_d_n8;
        var_erfcpos_dn9 = assign23010_e26277_d_n9;

        let assign23020_e26280: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard439 = assign23020_e26280;

        let (assign23030_e26294, assign23030_e26294_d_n6, assign23030_e26294_d_n7, assign23030_e26294_d_n8, assign23030_e26294_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) && (var_guard439 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign23030_e26294;
        var_erfctimesexpmtat_dn6 = assign23030_e26294_d_n6;
        var_erfctimesexpmtat_dn7 = assign23030_e26294_d_n7;
        var_erfctimesexpmtat_dn8 = assign23030_e26294_d_n8;
        var_erfctimesexpmtat_dn9 = assign23030_e26294_d_n9;

        let assign23040_e26297: f64 = (-230.25850929940458);
        let assign23040_e26298: f64 = if var_mtat > assign23040_e26297 { 1.0 } else { 0.0 };
        var_guard440 = assign23040_e26298;

        let (assign23050_e26316, assign23050_e26316_d_n6, assign23050_e26316_d_n7, assign23050_e26316_d_n8, assign23050_e26316_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) && (var_guard439 == 0.0)) && (var_guard440 != 0.0)) {
        let assign23050_e26314: f64 = (var_mtat).exp();
        (assign23050_e26314, (assign23050_e26314 * var_mtat_dn6), (assign23050_e26314 * var_mtat_dn7), (assign23050_e26314 * var_mtat_dn8), (assign23050_e26314 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23050_e26316;
        var_tmp_dn6 = assign23050_e26316_d_n6;
        var_tmp_dn7 = assign23050_e26316_d_n7;
        var_tmp_dn8 = assign23050_e26316_d_n8;
        var_tmp_dn9 = assign23050_e26316_d_n9;

        let (assign23060_e26359, assign23060_e26359_d_n6, assign23060_e26359_d_n7, assign23060_e26359_d_n8, assign23060_e26359_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) && (var_guard439 == 0.0)) && (var_guard440 == 0.0)) {
        let assign23060_e26335: f64 = (-230.25850929940458);
        let assign23060_e26337: f64 = (assign23060_e26335 - var_mtat);
        let assign23060_e26341: f64 = (-230.25850929940458);
        let assign23060_e26343: f64 = (assign23060_e26341 - var_mtat);
        let assign23060_e26346: f64 = (-230.25850929940458);
        let assign23060_e26348: f64 = (assign23060_e26346 - var_mtat);
        let assign23060_e26350: f64 = (assign23060_e26348 * 0.3333333333333333);
        let assign23060_e26351: f64 = (1.0 + assign23060_e26350);
        let assign23060_e26352: f64 = (assign23060_e26343 * assign23060_e26351);
        let assign23060_e26353: f64 = (0.5 * assign23060_e26352);
        let assign23060_e26354: f64 = (1.0 + assign23060_e26353);
        let assign23060_e26355: f64 = (assign23060_e26337 * assign23060_e26354);
        let assign23060_e26356: f64 = (1.0 + assign23060_e26355);
        let assign23060_e26357: f64 = (1e-100 / assign23060_e26356);
        (assign23060_e26357, (-((1e-100 * (((-var_mtat_dn6) * assign23060_e26354) + (assign23060_e26337 * (0.5 * (((-var_mtat_dn6) * assign23060_e26351) + (assign23060_e26343 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign23060_e26356 * assign23060_e26356))), (-((1e-100 * (((-var_mtat_dn7) * assign23060_e26354) + (assign23060_e26337 * (0.5 * (((-var_mtat_dn7) * assign23060_e26351) + (assign23060_e26343 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign23060_e26356 * assign23060_e26356))), (-((1e-100 * (((-var_mtat_dn8) * assign23060_e26354) + (assign23060_e26337 * (0.5 * (((-var_mtat_dn8) * assign23060_e26351) + (assign23060_e26343 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign23060_e26356 * assign23060_e26356))), (-((1e-100 * (((-var_mtat_dn9) * assign23060_e26354) + (assign23060_e26337 * (0.5 * (((-var_mtat_dn9) * assign23060_e26351) + (assign23060_e26343 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign23060_e26356 * assign23060_e26356))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23060_e26359;
        var_tmp_dn6 = assign23060_e26359_d_n6;
        var_tmp_dn7 = assign23060_e26359_d_n7;
        var_tmp_dn8 = assign23060_e26359_d_n8;
        var_tmp_dn9 = assign23060_e26359_d_n9;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_asrh_dn9_slot = var_asrh_dn9;
        *var_btat_slot = var_btat;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_btat_dn9_slot = var_btat_dn9;
        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfcpos_dn9_slot = var_erfcpos_dn9;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_erfctimesexpmtat_dn9_slot = var_erfctimesexpmtat_dn9;
        *var_guard434_slot = var_guard434;
        *var_guard435_slot = var_guard435;
        *var_guard436_slot = var_guard436;
        *var_guard437_slot = var_guard437;
        *var_guard438_slot = var_guard438;
        *var_guard439_slot = var_guard439;
        *var_guard440_slot = var_guard440;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ktat_dn9_slot = var_ktat_dn9;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_ltat_dn9_slot = var_ltat_dn9;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_mtat_dn9_slot = var_mtat_dn9;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_sqrtumax_dn9_slot = var_sqrtumax_dn9;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_terfc_dn9_slot = var_terfc_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_twoatatoverthreebtat_dn9_slot = var_twoatatoverthreebtat_dn9;
        *var_umax_slot = var_umax;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umax_dn9_slot = var_umax_dn9;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxbeforelimiting_dn9_slot = var_umaxbeforelimiting_dn9;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_umaxpoweronepointfive_dn9_slot = var_umaxpoweronepointfive_dn9;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wdep_dn9_slot = var_wdep_dn9;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wgamma_dn9_slot = var_wgamma_dn9;
        *var_wsrh_slot = var_wsrh;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_wtat_dn9_slot = var_wtat_dn9;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_xerfc_dn9_slot = var_xerfc_dn9;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
        *var_ysq_dn9_slot = var_ysq_dn9;
    }

    pub(super) fn stamp_transient_block_43(
        p: &Parameters,
        var_alphaav: f64,
        var_atatsti: f64,
        var_erfcpos: f64,
        var_erfcpos_dn6: f64,
        var_erfcpos_dn7: f64,
        var_erfcpos_dn8: f64,
        var_erfcpos_dn9: f64,
        var_fbbtsti: f64,
        var_fstopsti: f64,
        var_ftdgat: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard431: f64,
        var_guard435: f64,
        var_guard439: f64,
        var_idmult: f64,
        var_idsatgat: f64,
        var_ktat: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_ktat_dn9: f64,
        var_lgsource_i: f64,
        var_one_over_one_minus_psti: f64,
        var_slopesti: f64,
        var_two_psistar: f64,
        var_v4: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbigat: f64,
        var_vbirgatinv: f64,
        var_vbirstiinv: f64,
        var_vbrinvsti: f64,
        var_vjsrh: f64,
        var_wdepnulrgat: f64,
        var_wdepnulrinvsti: f64,
        var_wtat: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_wtat_dn9: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_asrh_dn9_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_erfctimesexpmtat_dn9_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_fmaxr_dn9_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_gammamax_dn9_slot: &mut f64,
        var_guard441_slot: &mut f64,
        var_guard442_slot: &mut f64,
        var_guard443_slot: &mut f64,
        var_guard444_slot: &mut f64,
        var_guard445_slot: &mut f64,
        var_guard446_slot: &mut f64,
        var_guard447_slot: &mut f64,
        var_guard448_slot: &mut f64,
        var_guard449_slot: &mut f64,
        var_guard450_slot: &mut f64,
        var_guard451_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_id__blk212_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_ijungat_dn9_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_ijunsti_dn9_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wdep_dn9_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_asrh_dn9: f64 = *var_asrh_dn9_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_erfctimesexpmtat_dn9: f64 = *var_erfctimesexpmtat_dn9_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fbreakdown_dn9: f64 = *var_fbreakdown_dn9_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_fmaxr_dn9: f64 = *var_fmaxr_dn9_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_gammamax_dn9: f64 = *var_gammamax_dn9_slot;
        let mut var_guard441: f64 = *var_guard441_slot;
        let mut var_guard442: f64 = *var_guard442_slot;
        let mut var_guard443: f64 = *var_guard443_slot;
        let mut var_guard444: f64 = *var_guard444_slot;
        let mut var_guard445: f64 = *var_guard445_slot;
        let mut var_guard446: f64 = *var_guard446_slot;
        let mut var_guard447: f64 = *var_guard447_slot;
        let mut var_guard448: f64 = *var_guard448_slot;
        let mut var_guard449: f64 = *var_guard449_slot;
        let mut var_guard450: f64 = *var_guard450_slot;
        let mut var_guard451: f64 = *var_guard451_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_id__blk212: f64 = *var_id__blk212_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_ijungat_dn9: f64 = *var_ijungat_dn9_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_ijunsti_dn9: f64 = *var_ijunsti_dn9_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wdep_dn9: f64 = *var_wdep_dn9_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;

        let (assign23070_e26378, assign23070_e26378_d_n6, assign23070_e26378_d_n7, assign23070_e26378_d_n8, assign23070_e26378_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) && (var_guard439 == 0.0)) {
        let assign23070_e26374: f64 = (2.0 * var_tmp);
        let assign23070_e26376: f64 = (assign23070_e26374 - var_erfcpos);
        (assign23070_e26376, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign23070_e26378;
        var_erfctimesexpmtat_dn6 = assign23070_e26378_d_n6;
        var_erfctimesexpmtat_dn7 = assign23070_e26378_d_n7;
        var_erfctimesexpmtat_dn8 = assign23070_e26378_d_n8;
        var_erfctimesexpmtat_dn9 = assign23070_e26378_d_n9;

        let (assign23080_e26398, assign23080_e26398_d_n6, assign23080_e26398_d_n7, assign23080_e26398_d_n8, assign23080_e26398_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) {
        let assign23080_e26390: f64 = (1.772453850905516 * 0.5);
        let assign23080_e26393: f64 = (var_atatsti * var_erfctimesexpmtat);
        let assign23080_e26395: f64 = (assign23080_e26393 / var_ktat);
        let assign23080_e26396: f64 = (assign23080_e26390 * assign23080_e26395);
        (assign23080_e26396, (assign23080_e26390 * ((((var_atatsti * var_erfctimesexpmtat_dn6) * var_ktat) - (assign23080_e26393 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign23080_e26390 * ((((var_atatsti * var_erfctimesexpmtat_dn7) * var_ktat) - (assign23080_e26393 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign23080_e26390 * ((((var_atatsti * var_erfctimesexpmtat_dn8) * var_ktat) - (assign23080_e26393 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign23080_e26390 * ((((var_atatsti * var_erfctimesexpmtat_dn9) * var_ktat) - (assign23080_e26393 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign23080_e26398;
        var_gammamax_dn6 = assign23080_e26398_d_n6;
        var_gammamax_dn7 = assign23080_e26398_d_n7;
        var_gammamax_dn8 = assign23080_e26398_d_n8;
        var_gammamax_dn9 = assign23080_e26398_d_n9;

        let (assign23090_e26416, assign23090_e26416_d_n6, assign23090_e26416_d_n7, assign23090_e26416_d_n8, assign23090_e26416_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard435 == 0.0)) {
        let assign23090_e26411: f64 = (var_asrh * var_gammamax);
        let assign23090_e26413: f64 = (assign23090_e26411 * var_wtat);
        let assign23090_e26414: f64 = (p.p863 * assign23090_e26413);
        (assign23090_e26414, (p.p863 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign23090_e26411 * var_wtat_dn6))), (p.p863 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign23090_e26411 * var_wtat_dn7))), (p.p863 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign23090_e26411 * var_wtat_dn8))), (p.p863 * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign23090_e26411 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign23090_e26416;
        var_itat_dn6 = assign23090_e26416_d_n6;
        var_itat_dn7 = assign23090_e26416_d_n7;
        var_itat_dn8 = assign23090_e26416_d_n8;
        var_itat_dn9 = assign23090_e26416_d_n9;

        let assign23100_e26419: f64 = if p.p869 == 0.0 { 1.0 } else { 0.0 };
        var_guard441 = assign23100_e26419;

        let (assign23110_e26430, assign23110_e26430_d_n6, assign23110_e26430_d_n7, assign23110_e26430_d_n8, assign23110_e26430_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard441 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign23110_e26430;
        var_ibbt_dn6 = assign23110_e26430_d_n6;
        var_ibbt_dn7 = assign23110_e26430_d_n7;
        var_ibbt_dn8 = assign23110_e26430_d_n8;
        var_ibbt_dn9 = assign23110_e26430_d_n9;

        let assign23120_e26433: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };
        var_guard442 = assign23120_e26433;

        let (assign23130_e26452, assign23130_e26452_d_n6, assign23130_e26452_d_n7, assign23130_e26452_d_n8, assign23130_e26452_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard441 == 0.0)) && (var_guard442 != 0.0)) {
        let assign23130_e26447: f64 = (p.p846 - var_vbbt);
        let assign23130_e26449: f64 = (assign23130_e26447 * var_vbirstiinv);
        let assign23130_e26450: f64 = (assign23130_e26449).sqrt();
        (assign23130_e26450, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23130_e26452;
        var_tmp_dn6 = assign23130_e26452_d_n6;
        var_tmp_dn7 = assign23130_e26452_d_n7;
        var_tmp_dn8 = assign23130_e26452_d_n8;
        var_tmp_dn9 = assign23130_e26452_d_n9;

        let (assign23140_e26473, assign23140_e26473_d_n6, assign23140_e26473_d_n7, assign23140_e26473_d_n8, assign23140_e26473_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard441 == 0.0)) && (var_guard442 == 0.0)) {
        let assign23140_e26467: f64 = (p.p846 - var_vbbt);
        let assign23140_e26469: f64 = (assign23140_e26467 * var_vbirstiinv);
        let assign23140_e26471: f64 = (assign23140_e26469).powf(p.p849);
        (assign23140_e26471, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23140_e26473;
        var_tmp_dn6 = assign23140_e26473_d_n6;
        var_tmp_dn7 = assign23140_e26473_d_n7;
        var_tmp_dn8 = assign23140_e26473_d_n8;
        var_tmp_dn9 = assign23140_e26473_d_n9;

        let (assign23150_e26493, assign23150_e26493_d_n6, assign23150_e26493_d_n7, assign23150_e26493_d_n8, assign23150_e26493_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard441 == 0.0)) {
        let assign23150_e26486: f64 = (p.p846 - var_vbbt);
        let assign23150_e26488: f64 = (assign23150_e26486 * var_wdepnulrinvsti);
        let assign23150_e26490: f64 = (assign23150_e26488 / var_tmp);
        let assign23150_e26491: f64 = (var_one_over_one_minus_psti * assign23150_e26490);
        (assign23150_e26491, (var_one_over_one_minus_psti * (-((assign23150_e26488 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign23150_e26488 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign23150_e26488 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign23150_e26488 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign23150_e26493;
        var_fmaxr_dn6 = assign23150_e26493_d_n6;
        var_fmaxr_dn7 = assign23150_e26493_d_n7;
        var_fmaxr_dn8 = assign23150_e26493_d_n8;
        var_fmaxr_dn9 = assign23150_e26493_d_n9;

        let assign23160_e26495: f64 = (-var_fbbtsti);
        let assign23160_e26497: f64 = (assign23160_e26495 / var_fmaxr);
        let assign23160_e26498: f64 = (assign23160_e26497).abs();
        let assign23160_e26500: f64 = if assign23160_e26498 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard443 = assign23160_e26500;

        let (assign23170_e26518, assign23170_e26518_d_n6, assign23170_e26518_d_n7, assign23170_e26518_d_n8, assign23170_e26518_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard441 == 0.0)) && (var_guard443 != 0.0)) {
        let assign23170_e26513: f64 = (-var_fbbtsti);
        let assign23170_e26515: f64 = (assign23170_e26513 / var_fmaxr);
        let assign23170_e26516: f64 = (assign23170_e26515).exp();
        (assign23170_e26516, (assign23170_e26516 * (-((assign23170_e26513 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign23170_e26516 * (-((assign23170_e26513 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign23170_e26516 * (-((assign23170_e26513 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign23170_e26516 * (-((assign23170_e26513 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23170_e26518;
        var_tmp_dn6 = assign23170_e26518_d_n6;
        var_tmp_dn7 = assign23170_e26518_d_n7;
        var_tmp_dn8 = assign23170_e26518_d_n8;
        var_tmp_dn9 = assign23170_e26518_d_n9;

        let assign23180_e26520: f64 = (-var_fbbtsti);
        let assign23180_e26522: f64 = (assign23180_e26520 / var_fmaxr);
        let assign23180_e26524: f64 = if assign23180_e26522 < 0.0 { 1.0 } else { 0.0 };
        var_guard444 = assign23180_e26524;

        let (assign23190_e26575, assign23190_e26575_d_n6, assign23190_e26575_d_n7, assign23190_e26575_d_n8, assign23190_e26575_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard441 == 0.0)) && (var_guard443 == 0.0)) && (var_guard444 != 0.0)) {
        let assign23190_e26542: f64 = (-230.25850929940458);
        let assign23190_e26544: f64 = (-var_fbbtsti);
        let assign23190_e26546: f64 = (assign23190_e26544 / var_fmaxr);
        let assign23190_e26547: f64 = (assign23190_e26542 - assign23190_e26546);
        let assign23190_e26551: f64 = (-230.25850929940458);
        let assign23190_e26553: f64 = (-var_fbbtsti);
        let assign23190_e26555: f64 = (assign23190_e26553 / var_fmaxr);
        let assign23190_e26556: f64 = (assign23190_e26551 - assign23190_e26555);
        let assign23190_e26559: f64 = (-230.25850929940458);
        let assign23190_e26561: f64 = (-var_fbbtsti);
        let assign23190_e26563: f64 = (assign23190_e26561 / var_fmaxr);
        let assign23190_e26564: f64 = (assign23190_e26559 - assign23190_e26563);
        let assign23190_e26566: f64 = (assign23190_e26564 * 0.3333333333333333);
        let assign23190_e26567: f64 = (1.0 + assign23190_e26566);
        let assign23190_e26568: f64 = (assign23190_e26556 * assign23190_e26567);
        let assign23190_e26569: f64 = (0.5 * assign23190_e26568);
        let assign23190_e26570: f64 = (1.0 + assign23190_e26569);
        let assign23190_e26571: f64 = (assign23190_e26547 * assign23190_e26570);
        let assign23190_e26572: f64 = (1.0 + assign23190_e26571);
        let assign23190_e26573: f64 = (1e-100 / assign23190_e26572);
        (assign23190_e26573, (-((1e-100 * (((-(-((assign23190_e26544 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign23190_e26570) + (assign23190_e26547 * (0.5 * (((-(-((assign23190_e26553 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign23190_e26567) + (assign23190_e26556 * ((-(-((assign23190_e26561 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign23190_e26572 * assign23190_e26572))), (-((1e-100 * (((-(-((assign23190_e26544 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign23190_e26570) + (assign23190_e26547 * (0.5 * (((-(-((assign23190_e26553 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign23190_e26567) + (assign23190_e26556 * ((-(-((assign23190_e26561 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign23190_e26572 * assign23190_e26572))), (-((1e-100 * (((-(-((assign23190_e26544 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign23190_e26570) + (assign23190_e26547 * (0.5 * (((-(-((assign23190_e26553 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign23190_e26567) + (assign23190_e26556 * ((-(-((assign23190_e26561 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign23190_e26572 * assign23190_e26572))), (-((1e-100 * (((-(-((assign23190_e26544 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign23190_e26570) + (assign23190_e26547 * (0.5 * (((-(-((assign23190_e26553 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign23190_e26567) + (assign23190_e26556 * ((-(-((assign23190_e26561 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign23190_e26572 * assign23190_e26572))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23190_e26575;
        var_tmp_dn6 = assign23190_e26575_d_n6;
        var_tmp_dn7 = assign23190_e26575_d_n7;
        var_tmp_dn8 = assign23190_e26575_d_n8;
        var_tmp_dn9 = assign23190_e26575_d_n9;

        let (assign23200_e26624, assign23200_e26624_d_n6, assign23200_e26624_d_n7, assign23200_e26624_d_n8, assign23200_e26624_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard441 == 0.0)) && (var_guard443 == 0.0)) && (var_guard444 == 0.0)) {
        let assign23200_e26594: f64 = (-var_fbbtsti);
        let assign23200_e26596: f64 = (assign23200_e26594 / var_fmaxr);
        let assign23200_e26598: f64 = (assign23200_e26596 - 230.25850929940458);
        let assign23200_e26602: f64 = (-var_fbbtsti);
        let assign23200_e26604: f64 = (assign23200_e26602 / var_fmaxr);
        let assign23200_e26606: f64 = (assign23200_e26604 - 230.25850929940458);
        let assign23200_e26609: f64 = (-var_fbbtsti);
        let assign23200_e26611: f64 = (assign23200_e26609 / var_fmaxr);
        let assign23200_e26613: f64 = (assign23200_e26611 - 230.25850929940458);
        let assign23200_e26615: f64 = (assign23200_e26613 * 0.3333333333333333);
        let assign23200_e26616: f64 = (1.0 + assign23200_e26615);
        let assign23200_e26617: f64 = (assign23200_e26606 * assign23200_e26616);
        let assign23200_e26618: f64 = (0.5 * assign23200_e26617);
        let assign23200_e26619: f64 = (1.0 + assign23200_e26618);
        let assign23200_e26620: f64 = (assign23200_e26598 * assign23200_e26619);
        let assign23200_e26621: f64 = (1.0 + assign23200_e26620);
        let assign23200_e26622: f64 = (1e100 * assign23200_e26621);
        (assign23200_e26622, (1e100 * (((-((assign23200_e26594 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign23200_e26619) + (assign23200_e26598 * (0.5 * (((-((assign23200_e26602 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign23200_e26616) + (assign23200_e26606 * ((-((assign23200_e26609 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign23200_e26594 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign23200_e26619) + (assign23200_e26598 * (0.5 * (((-((assign23200_e26602 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign23200_e26616) + (assign23200_e26606 * ((-((assign23200_e26609 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign23200_e26594 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign23200_e26619) + (assign23200_e26598 * (0.5 * (((-((assign23200_e26602 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign23200_e26616) + (assign23200_e26606 * ((-((assign23200_e26609 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign23200_e26594 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign23200_e26619) + (assign23200_e26598 * (0.5 * (((-((assign23200_e26602 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign23200_e26616) + (assign23200_e26606 * ((-((assign23200_e26609 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23200_e26624;
        var_tmp_dn6 = assign23200_e26624_d_n6;
        var_tmp_dn7 = assign23200_e26624_d_n7;
        var_tmp_dn8 = assign23200_e26624_d_n8;
        var_tmp_dn9 = assign23200_e26624_d_n9;

        let (assign23210_e26644, assign23210_e26644_d_n6, assign23210_e26644_d_n7, assign23210_e26644_d_n8, assign23210_e26644_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard441 == 0.0)) {
        let assign23210_e26637: f64 = (var_v4 * var_fmaxr);
        let assign23210_e26639: f64 = (assign23210_e26637 * var_fmaxr);
        let assign23210_e26641: f64 = (assign23210_e26639 * var_tmp);
        let assign23210_e26642: f64 = (p.p869 * assign23210_e26641);
        (assign23210_e26642, (p.p869 * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign23210_e26637 * var_fmaxr_dn6)) * var_tmp) + (assign23210_e26639 * var_tmp_dn6))), (p.p869 * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign23210_e26637 * var_fmaxr_dn7)) * var_tmp) + (assign23210_e26639 * var_tmp_dn7))), (p.p869 * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign23210_e26637 * var_fmaxr_dn8)) * var_tmp) + (assign23210_e26639 * var_tmp_dn8))), (p.p869 * (((((var_v4 * var_fmaxr_dn9) * var_fmaxr) + (assign23210_e26637 * var_fmaxr_dn9)) * var_tmp) + (assign23210_e26639 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign23210_e26644;
        var_ibbt_dn6 = assign23210_e26644_d_n6;
        var_ibbt_dn7 = assign23210_e26644_d_n7;
        var_ibbt_dn8 = assign23210_e26644_d_n8;
        var_ibbt_dn9 = assign23210_e26644_d_n9;

        let assign23220_e26647: f64 = if p.p878 > 1000.0 { 1.0 } else { 0.0 };
        var_guard445 = assign23220_e26647;

        let (assign23230_e26658, assign23230_e26658_d_n6, assign23230_e26658_d_n7, assign23230_e26658_d_n8, assign23230_e26658_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard445 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign23230_e26658;
        var_fbreakdown_dn6 = assign23230_e26658_d_n6;
        var_fbreakdown_dn7 = assign23230_e26658_d_n7;
        var_fbreakdown_dn8 = assign23230_e26658_d_n8;
        var_fbreakdown_dn9 = assign23230_e26658_d_n9;

        let assign23240_e26661: f64 = (-var_alphaav);
        let assign23240_e26663: f64 = (assign23240_e26661 * p.p878);
        let assign23240_e26664: f64 = if var_vav > assign23240_e26663 { 1.0 } else { 0.0 };
        var_guard446 = assign23240_e26664;

        let assign23250_e26667: f64 = if p.p881 == 4.0 { 1.0 } else { 0.0 };
        var_guard447 = assign23250_e26667;

        let (assign23260_e26697, assign23260_e26697_d_n6, assign23260_e26697_d_n7, assign23260_e26697_d_n8, assign23260_e26697_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard445 == 0.0)) && (var_guard446 != 0.0)) && (var_guard447 != 0.0)) {
        let assign23260_e26683: f64 = (var_vav * var_vbrinvsti);
        let assign23260_e26686: f64 = (var_vav * var_vbrinvsti);
        let assign23260_e26687: f64 = (assign23260_e26683 * assign23260_e26686);
        let assign23260_e26690: f64 = (var_vav * var_vbrinvsti);
        let assign23260_e26691: f64 = (assign23260_e26687 * assign23260_e26690);
        let assign23260_e26694: f64 = (var_vav * var_vbrinvsti);
        let assign23260_e26695: f64 = (assign23260_e26691 * assign23260_e26694);
        (assign23260_e26695, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23260_e26697;
        var_tmp_dn6 = assign23260_e26697_d_n6;
        var_tmp_dn7 = assign23260_e26697_d_n7;
        var_tmp_dn8 = assign23260_e26697_d_n8;
        var_tmp_dn9 = assign23260_e26697_d_n9;

        let (assign23270_e26719, assign23270_e26719_d_n6, assign23270_e26719_d_n7, assign23270_e26719_d_n8, assign23270_e26719_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard445 == 0.0)) && (var_guard446 != 0.0)) && (var_guard447 == 0.0)) {
        let assign23270_e26714: f64 = (var_vav * var_vbrinvsti);
        let assign23270_e26715: f64 = (assign23270_e26714).abs();
        let assign23270_e26717: f64 = (assign23270_e26715).powf(p.p881);
        (assign23270_e26717, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23270_e26719;
        var_tmp_dn6 = assign23270_e26719_d_n6;
        var_tmp_dn7 = assign23270_e26719_d_n7;
        var_tmp_dn8 = assign23270_e26719_d_n8;
        var_tmp_dn9 = assign23270_e26719_d_n9;

        let (assign23280_e26737, assign23280_e26737_d_n6, assign23280_e26737_d_n7, assign23280_e26737_d_n8, assign23280_e26737_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard445 == 0.0)) && (var_guard446 != 0.0)) {
        let assign23280_e26734: f64 = (1.0 - var_tmp);
        let assign23280_e26735: f64 = (1.0 / assign23280_e26734);
        (assign23280_e26735, (-((-var_tmp_dn6) / (assign23280_e26734 * assign23280_e26734))), (-((-var_tmp_dn7) / (assign23280_e26734 * assign23280_e26734))), (-((-var_tmp_dn8) / (assign23280_e26734 * assign23280_e26734))), (-((-var_tmp_dn9) / (assign23280_e26734 * assign23280_e26734))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign23280_e26737;
        var_fbreakdown_dn6 = assign23280_e26737_d_n6;
        var_fbreakdown_dn7 = assign23280_e26737_d_n7;
        var_fbreakdown_dn8 = assign23280_e26737_d_n8;
        var_fbreakdown_dn9 = assign23280_e26737_d_n9;

        let (assign23290_e26760, assign23290_e26760_d_n6, assign23290_e26760_d_n7, assign23290_e26760_d_n8, assign23290_e26760_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) && (var_guard445 == 0.0)) && (var_guard446 == 0.0)) {
        let assign23290_e26754: f64 = (var_alphaav * p.p878);
        let assign23290_e26755: f64 = (var_vav + assign23290_e26754);
        let assign23290_e26757: f64 = (assign23290_e26755 * var_slopesti);
        let assign23290_e26758: f64 = (var_fstopsti + assign23290_e26757);
        (assign23290_e26758, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign23290_e26760;
        var_fbreakdown_dn6 = assign23290_e26760_d_n6;
        var_fbreakdown_dn7 = assign23290_e26760_d_n7;
        var_fbreakdown_dn8 = assign23290_e26760_d_n8;
        var_fbreakdown_dn9 = assign23290_e26760_d_n9;

        let (assign23300_e26779, assign23300_e26779_d_n6, assign23300_e26779_d_n7, assign23300_e26779_d_n8, assign23300_e26779_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard431 == 0.0)) {
        let assign23300_e26770: f64 = (var_id__blk212 + var_isrh);
        let assign23300_e26772: f64 = (assign23300_e26770 + var_itat);
        let assign23300_e26774: f64 = (assign23300_e26772 + var_ibbt);
        let assign23300_e26775: f64 = (p.p29 * assign23300_e26774);
        let assign23300_e26777: f64 = (assign23300_e26775 * var_fbreakdown);
        (assign23300_e26777, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign23300_e26775 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign23300_e26775 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign23300_e26775 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign23300_e26775 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign23300_e26779;
        var_ijunsti_dn6 = assign23300_e26779_d_n6;
        var_ijunsti_dn7 = assign23300_e26779_d_n7;
        var_ijunsti_dn8 = assign23300_e26779_d_n8;
        var_ijunsti_dn9 = assign23300_e26779_d_n9;

        let assign23310_e26782: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard448 = assign23310_e26782;

        let (assign23320_e26790, assign23320_e26790_d_n6, assign23320_e26790_d_n7, assign23320_e26790_d_n8, assign23320_e26790_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign23320_e26790;
        var_ijungat_dn6 = assign23320_e26790_d_n6;
        var_ijungat_dn7 = assign23320_e26790_d_n7;
        var_ijungat_dn8 = assign23320_e26790_d_n8;
        var_ijungat_dn9 = assign23320_e26790_d_n9;

        let (assign23330_e26801,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) {
        let assign23330_e26799: f64 = (var_idsatgat * var_idmult);
        (assign23330_e26799,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign23330_e26801;

        let assign23340_e26808: f64 = if ((p.p859 == 0.0) && (p.p864 == 0.0)) { 1.0 } else { 0.0 };
        var_guard449 = assign23340_e26808;

        let (assign23350_e26819, assign23350_e26819_d_n6, assign23350_e26819_d_n7, assign23350_e26819_d_n8, assign23350_e26819_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard449 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign23350_e26819;
        var_isrh_dn6 = assign23350_e26819_d_n6;
        var_isrh_dn7 = assign23350_e26819_d_n7;
        var_isrh_dn8 = assign23350_e26819_d_n8;
        var_isrh_dn9 = assign23350_e26819_d_n9;

        let (assign23360_e26833,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard449 == 0.0)) {
        let assign23360_e26831: f64 = (var_vbigat - var_vjsrh);
        (assign23360_e26831,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign23360_e26833;

        let (assign23370_e26852,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard449 == 0.0)) {
        let assign23370_e26847: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign23370_e26848: f64 = (1.0 - assign23370_e26847);
        let assign23370_e26849: f64 = (assign23370_e26848).sqrt();
        let assign23370_e26850: f64 = (1.0 - assign23370_e26849);
        (assign23370_e26850,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign23370_e26852;

        let assign23380_e26855: f64 = if p.p850 == 0.5 { 1.0 } else { 0.0 };
        var_guard450 = assign23380_e26855;

        let (assign23390_e26869,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard449 == 0.0)) && (var_guard450 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign23390_e26869;

        let (assign23400_e26901,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard449 == 0.0)) && (var_guard450 == 0.0)) {
        let assign23400_e26884: f64 = (var_wsrhstep * var_wsrhstep);
        let assign23400_e26886: f64 = (var_wsrhstep).ln();
        let assign23400_e26887: f64 = (assign23400_e26884 * assign23400_e26886);
        let assign23400_e26890: f64 = (1.0 - var_wsrhstep);
        let assign23400_e26891: f64 = (assign23400_e26887 / assign23400_e26890);
        let assign23400_e26893: f64 = (assign23400_e26891 + var_wsrhstep);
        let assign23400_e26897: f64 = (2.0 * p.p850);
        let assign23400_e26898: f64 = (1.0 - assign23400_e26897);
        let assign23400_e26899: f64 = (assign23400_e26893 * assign23400_e26898);
        (assign23400_e26899,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign23400_e26901;

        let (assign23410_e26915,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard449 == 0.0)) {
        let assign23410_e26913: f64 = (var_wsrhstep + var_dwsrh);
        (assign23410_e26913,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign23410_e26915;

        let assign23420_e26918: f64 = if p.p850 == 0.5 { 1.0 } else { 0.0 };
        var_guard451 = assign23420_e26918;

        let (assign23430_e26935, assign23430_e26935_d_n6, assign23430_e26935_d_n7, assign23430_e26935_d_n8, assign23430_e26935_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard449 == 0.0)) && (var_guard451 != 0.0)) {
        let assign23430_e26932: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign23430_e26933: f64 = (assign23430_e26932).sqrt();
        (assign23430_e26933, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23430_e26935;
        var_tmp_dn6 = assign23430_e26935_d_n6;
        var_tmp_dn7 = assign23430_e26935_d_n7;
        var_tmp_dn8 = assign23430_e26935_d_n8;
        var_tmp_dn9 = assign23430_e26935_d_n9;

        let (assign23440_e26954, assign23440_e26954_d_n6, assign23440_e26954_d_n7, assign23440_e26954_d_n8, assign23440_e26954_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard449 == 0.0)) && (var_guard451 == 0.0)) {
        let assign23440_e26950: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign23440_e26952: f64 = (assign23440_e26950).powf(p.p850);
        (assign23440_e26952, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23440_e26954;
        var_tmp_dn6 = assign23440_e26954_d_n6;
        var_tmp_dn7 = assign23440_e26954_d_n7;
        var_tmp_dn8 = assign23440_e26954_d_n8;
        var_tmp_dn9 = assign23440_e26954_d_n9;

        let (assign23450_e26968, assign23450_e26968_d_n6, assign23450_e26968_d_n7, assign23450_e26968_d_n8, assign23450_e26968_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard449 == 0.0)) {
        let assign23450_e26966: f64 = (var_wdepnulrgat * var_tmp);
        (assign23450_e26966, (var_wdepnulrgat * var_tmp_dn6), (var_wdepnulrgat * var_tmp_dn7), (var_wdepnulrgat * var_tmp_dn8), (var_wdepnulrgat * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign23450_e26968;
        var_wdep_dn6 = assign23450_e26968_d_n6;
        var_wdep_dn7 = assign23450_e26968_d_n7;
        var_wdep_dn8 = assign23450_e26968_d_n8;
        var_wdep_dn9 = assign23450_e26968_d_n9;

        let (assign23460_e26986, assign23460_e26986_d_n6, assign23460_e26986_d_n7, assign23460_e26986_d_n8, assign23460_e26986_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard449 == 0.0)) {
        let assign23460_e26981: f64 = (var_zinv - 1.0);
        let assign23460_e26983: f64 = (assign23460_e26981 * var_wdep);
        let assign23460_e26984: f64 = (var_ftdgat * assign23460_e26983);
        (assign23460_e26984, (var_ftdgat * (assign23460_e26981 * var_wdep_dn6)), (var_ftdgat * (assign23460_e26981 * var_wdep_dn7)), (var_ftdgat * (assign23460_e26981 * var_wdep_dn8)), (var_ftdgat * (assign23460_e26981 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign23460_e26986;
        var_asrh_dn6 = assign23460_e26986_d_n6;
        var_asrh_dn7 = assign23460_e26986_d_n7;
        var_asrh_dn8 = assign23460_e26986_d_n8;
        var_asrh_dn9 = assign23460_e26986_d_n9;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_asrh_dn9_slot = var_asrh_dn9;
        *var_dwsrh_slot = var_dwsrh;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_erfctimesexpmtat_dn9_slot = var_erfctimesexpmtat_dn9;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_fmaxr_dn9_slot = var_fmaxr_dn9;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_gammamax_dn9_slot = var_gammamax_dn9;
        *var_guard441_slot = var_guard441;
        *var_guard442_slot = var_guard442;
        *var_guard443_slot = var_guard443;
        *var_guard444_slot = var_guard444;
        *var_guard445_slot = var_guard445;
        *var_guard446_slot = var_guard446;
        *var_guard447_slot = var_guard447;
        *var_guard448_slot = var_guard448;
        *var_guard449_slot = var_guard449;
        *var_guard450_slot = var_guard450;
        *var_guard451_slot = var_guard451;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_id__blk212_slot = var_id__blk212;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_ijungat_dn9_slot = var_ijungat_dn9;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_ijunsti_dn9_slot = var_ijunsti_dn9;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wdep_dn9_slot = var_wdep_dn9;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
    }

    pub(super) fn stamp_transient_block_44(
        p: &Parameters,
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatgat: f64,
        var_berfc: f64,
        var_btatpartgat: f64,
        var_cerfc: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard448: f64,
        var_guard449: f64,
        var_one_minus_pgat: f64,
        var_one_over_one_minus_pgat: f64,
        var_perfc: f64,
        var_vbi_minus_vjsrh: f64,
        var_wdep: f64,
        var_wdep_dn6: f64,
        var_wdep_dn7: f64,
        var_wdep_dn8: f64,
        var_wdep_dn9: f64,
        var_wsrh: f64,
        var_btat_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_btat_dn9_slot: &mut f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfcpos_dn9_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_erfctimesexpmtat_dn9_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_gammamax_dn9_slot: &mut f64,
        var_guard452_slot: &mut f64,
        var_guard453_slot: &mut f64,
        var_guard454_slot: &mut f64,
        var_guard455_slot: &mut f64,
        var_guard456_slot: &mut f64,
        var_guard457_slot: &mut f64,
        var_guard458_slot: &mut f64,
        var_guard459_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ktat_dn9_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_ltat_dn9_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_mtat_dn9_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_sqrtumax_dn9_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_terfc_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_dn9_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umax_dn9_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxbeforelimiting_dn9_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_dn9_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wgamma_dn9_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_wtat_dn9_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_xerfc_dn9_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
        var_ysq_dn9_slot: &mut f64,
    ) {
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_btat_dn9: f64 = *var_btat_dn9_slot;
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfcpos_dn9: f64 = *var_erfcpos_dn9_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_erfctimesexpmtat_dn9: f64 = *var_erfctimesexpmtat_dn9_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_gammamax_dn9: f64 = *var_gammamax_dn9_slot;
        let mut var_guard452: f64 = *var_guard452_slot;
        let mut var_guard453: f64 = *var_guard453_slot;
        let mut var_guard454: f64 = *var_guard454_slot;
        let mut var_guard455: f64 = *var_guard455_slot;
        let mut var_guard456: f64 = *var_guard456_slot;
        let mut var_guard457: f64 = *var_guard457_slot;
        let mut var_guard458: f64 = *var_guard458_slot;
        let mut var_guard459: f64 = *var_guard459_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ktat_dn9: f64 = *var_ktat_dn9_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_ltat_dn9: f64 = *var_ltat_dn9_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_mtat_dn9: f64 = *var_mtat_dn9_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_sqrtumax_dn9: f64 = *var_sqrtumax_dn9_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_terfc_dn9: f64 = *var_terfc_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_twoatatoverthreebtat_dn9: f64 = *var_twoatatoverthreebtat_dn9_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umax_dn9: f64 = *var_umax_dn9_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxbeforelimiting_dn9: f64 = *var_umaxbeforelimiting_dn9_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_umaxpoweronepointfive_dn9: f64 = *var_umaxpoweronepointfive_dn9_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wgamma_dn9: f64 = *var_wgamma_dn9_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_wtat_dn9: f64 = *var_wtat_dn9_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_xerfc_dn9: f64 = *var_xerfc_dn9_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;
        let mut var_ysq_dn9: f64 = *var_ysq_dn9_slot;

        let (assign23470_e27002, assign23470_e27002_d_n6, assign23470_e27002_d_n7, assign23470_e27002_d_n8, assign23470_e27002_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard449 == 0.0)) {
        let assign23470_e26999: f64 = (var_asrh * var_wsrh);
        let assign23470_e27000: f64 = (p.p859 * assign23470_e26999);
        (assign23470_e27000, (p.p859 * (var_asrh_dn6 * var_wsrh)), (p.p859 * (var_asrh_dn7 * var_wsrh)), (p.p859 * (var_asrh_dn8 * var_wsrh)), (p.p859 * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign23470_e27002;
        var_isrh_dn6 = assign23470_e27002_d_n6;
        var_isrh_dn7 = assign23470_e27002_d_n7;
        var_isrh_dn8 = assign23470_e27002_d_n8;
        var_isrh_dn9 = assign23470_e27002_d_n9;

        let assign23480_e27005: f64 = if p.p864 == 0.0 { 1.0 } else { 0.0 };
        var_guard452 = assign23480_e27005;

        let (assign23490_e27016, assign23490_e27016_d_n6, assign23490_e27016_d_n7, assign23490_e27016_d_n8, assign23490_e27016_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign23490_e27016;
        var_itat_dn6 = assign23490_e27016_d_n6;
        var_itat_dn7 = assign23490_e27016_d_n7;
        var_itat_dn8 = assign23490_e27016_d_n8;
        var_itat_dn9 = assign23490_e27016_d_n9;

        let (assign23500_e27034, assign23500_e27034_d_n6, assign23500_e27034_d_n7, assign23500_e27034_d_n8, assign23500_e27034_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) {
        let assign23500_e27029: f64 = (var_wdep * var_one_minus_pgat);
        let assign23500_e27031: f64 = (assign23500_e27029 / var_vbi_minus_vjsrh);
        let assign23500_e27032: f64 = (var_btatpartgat * assign23500_e27031);
        (assign23500_e27032, (var_btatpartgat * ((var_wdep_dn6 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn7 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn8 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn9 * var_one_minus_pgat) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign23500_e27034;
        var_btat_dn6 = assign23500_e27034_d_n6;
        var_btat_dn7 = assign23500_e27034_d_n7;
        var_btat_dn8 = assign23500_e27034_d_n8;
        var_btat_dn9 = assign23500_e27034_d_n9;

        let (assign23510_e27050, assign23510_e27050_d_n6, assign23510_e27050_d_n7, assign23510_e27050_d_n8, assign23510_e27050_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) {
        let assign23510_e27046: f64 = (0.666666666666667 * var_atatgat);
        let assign23510_e27048: f64 = (assign23510_e27046 / var_btat);
        (assign23510_e27048, (-((assign23510_e27046 * var_btat_dn6) / (var_btat * var_btat))), (-((assign23510_e27046 * var_btat_dn7) / (var_btat * var_btat))), (-((assign23510_e27046 * var_btat_dn8) / (var_btat * var_btat))), (-((assign23510_e27046 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign23510_e27050;
        var_twoatatoverthreebtat_dn6 = assign23510_e27050_d_n6;
        var_twoatatoverthreebtat_dn7 = assign23510_e27050_d_n7;
        var_twoatatoverthreebtat_dn8 = assign23510_e27050_d_n8;
        var_twoatatoverthreebtat_dn9 = assign23510_e27050_d_n9;

        let (assign23520_e27064, assign23520_e27064_d_n6, assign23520_e27064_d_n7, assign23520_e27064_d_n8, assign23520_e27064_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) {
        let assign23520_e27062: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign23520_e27062, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign23520_e27064;
        var_umaxbeforelimiting_dn6 = assign23520_e27064_d_n6;
        var_umaxbeforelimiting_dn7 = assign23520_e27064_d_n7;
        var_umaxbeforelimiting_dn8 = assign23520_e27064_d_n8;
        var_umaxbeforelimiting_dn9 = assign23520_e27064_d_n9;

        let (assign23530_e27085, assign23530_e27085_d_n6, assign23530_e27085_d_n7, assign23530_e27085_d_n8, assign23530_e27085_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) {
        let assign23530_e27076: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign23530_e27079: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign23530_e27081: f64 = (assign23530_e27079 + 1.0);
        let assign23530_e27082: f64 = (assign23530_e27076 / assign23530_e27081);
        let assign23530_e27083: f64 = (assign23530_e27082).sqrt();
        (assign23530_e27083, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign23530_e27081) - (assign23530_e27076 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign23530_e27081 * assign23530_e27081)) / (2.0 * assign23530_e27083)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign23530_e27081) - (assign23530_e27076 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign23530_e27081 * assign23530_e27081)) / (2.0 * assign23530_e27083)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign23530_e27081) - (assign23530_e27076 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign23530_e27081 * assign23530_e27081)) / (2.0 * assign23530_e27083)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign23530_e27081) - (assign23530_e27076 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign23530_e27081 * assign23530_e27081)) / (2.0 * assign23530_e27083)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign23530_e27085;
        var_umax_dn6 = assign23530_e27085_d_n6;
        var_umax_dn7 = assign23530_e27085_d_n7;
        var_umax_dn8 = assign23530_e27085_d_n8;
        var_umax_dn9 = assign23530_e27085_d_n9;

        let (assign23540_e27098, assign23540_e27098_d_n6, assign23540_e27098_d_n7, assign23540_e27098_d_n8, assign23540_e27098_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) {
        let assign23540_e27096: f64 = (var_umax).sqrt();
        (assign23540_e27096, (var_umax_dn6 / (2.0 * assign23540_e27096)), (var_umax_dn7 / (2.0 * assign23540_e27096)), (var_umax_dn8 / (2.0 * assign23540_e27096)), (var_umax_dn9 / (2.0 * assign23540_e27096)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign23540_e27098;
        var_sqrtumax_dn6 = assign23540_e27098_d_n6;
        var_sqrtumax_dn7 = assign23540_e27098_d_n7;
        var_sqrtumax_dn8 = assign23540_e27098_d_n8;
        var_sqrtumax_dn9 = assign23540_e27098_d_n9;

        let (assign23550_e27112, assign23550_e27112_d_n6, assign23550_e27112_d_n7, assign23550_e27112_d_n8, assign23550_e27112_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) {
        let assign23550_e27110: f64 = (var_umax * var_sqrtumax);
        (assign23550_e27110, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign23550_e27112;
        var_umaxpoweronepointfive_dn6 = assign23550_e27112_d_n6;
        var_umaxpoweronepointfive_dn7 = assign23550_e27112_d_n7;
        var_umaxpoweronepointfive_dn8 = assign23550_e27112_d_n8;
        var_umaxpoweronepointfive_dn9 = assign23550_e27112_d_n9;

        let assign23560_e27114: f64 = (-p.p850);
        let assign23560_e27116: f64 = (assign23560_e27114 * var_one_over_one_minus_pgat);
        let assign23560_e27118: f64 = (-1.0);
        let assign23560_e27119: f64 = if assign23560_e27116 == assign23560_e27118 { 1.0 } else { 0.0 };
        var_guard453 = assign23560_e27119;

        let (assign23570_e27139, assign23570_e27139_d_n6, assign23570_e27139_d_n7, assign23570_e27139_d_n8, assign23570_e27139_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) && (var_guard453 != 0.0)) {
        let assign23570_e27135: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign23570_e27136: f64 = (1.0 + assign23570_e27135);
        let assign23570_e27137: f64 = (1.0 / assign23570_e27136);
        (assign23570_e27137, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign23570_e27136 * assign23570_e27136))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign23570_e27136 * assign23570_e27136))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign23570_e27136 * assign23570_e27136))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign23570_e27136 * assign23570_e27136))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign23570_e27139;
        var_wgamma_dn6 = assign23570_e27139_d_n6;
        var_wgamma_dn7 = assign23570_e27139_d_n7;
        var_wgamma_dn8 = assign23570_e27139_d_n8;
        var_wgamma_dn9 = assign23570_e27139_d_n9;

        let (assign23580_e27163, assign23580_e27163_d_n6, assign23580_e27163_d_n7, assign23580_e27163_d_n8, assign23580_e27163_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) && (var_guard453 == 0.0)) {
        let assign23580_e27155: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign23580_e27156: f64 = (1.0 + assign23580_e27155);
        let assign23580_e27158: f64 = (-p.p850);
        let assign23580_e27160: f64 = (assign23580_e27158 * var_one_over_one_minus_pgat);
        let assign23580_e27161: f64 = (assign23580_e27156).powf(assign23580_e27160);
        (assign23580_e27161, if 0.0 == 0.0 && ((assign23580_e27160) as f64).is_finite() && ((assign23580_e27160) as f64).fract() == 0.0 { if assign23580_e27160 == 0.0 { 0.0 } else { (assign23580_e27160 * ((assign23580_e27156).powf(assign23580_e27160 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign23580_e27161 * (assign23580_e27160 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign23580_e27156))) }, if 0.0 == 0.0 && ((assign23580_e27160) as f64).is_finite() && ((assign23580_e27160) as f64).fract() == 0.0 { if assign23580_e27160 == 0.0 { 0.0 } else { (assign23580_e27160 * ((assign23580_e27156).powf(assign23580_e27160 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign23580_e27161 * (assign23580_e27160 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign23580_e27156))) }, if 0.0 == 0.0 && ((assign23580_e27160) as f64).is_finite() && ((assign23580_e27160) as f64).fract() == 0.0 { if assign23580_e27160 == 0.0 { 0.0 } else { (assign23580_e27160 * ((assign23580_e27156).powf(assign23580_e27160 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign23580_e27161 * (assign23580_e27160 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign23580_e27156))) }, if 0.0 == 0.0 && ((assign23580_e27160) as f64).is_finite() && ((assign23580_e27160) as f64).fract() == 0.0 { if assign23580_e27160 == 0.0 { 0.0 } else { (assign23580_e27160 * ((assign23580_e27156).powf(assign23580_e27160 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign23580_e27161 * (assign23580_e27160 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign23580_e27156))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign23580_e27163;
        var_wgamma_dn6 = assign23580_e27163_d_n6;
        var_wgamma_dn7 = assign23580_e27163_d_n7;
        var_wgamma_dn8 = assign23580_e27163_d_n8;
        var_wgamma_dn9 = assign23580_e27163_d_n9;

        let (assign23590_e27181, assign23590_e27181_d_n6, assign23590_e27181_d_n7, assign23590_e27181_d_n8, assign23590_e27181_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) {
        let assign23590_e27175: f64 = (var_wsrh * var_wgamma);
        let assign23590_e27178: f64 = (var_wsrh + var_wgamma);
        let assign23590_e27179: f64 = (assign23590_e27175 / assign23590_e27178);
        (assign23590_e27179, ((((var_wsrh * var_wgamma_dn6) * assign23590_e27178) - (assign23590_e27175 * var_wgamma_dn6)) / (assign23590_e27178 * assign23590_e27178)), ((((var_wsrh * var_wgamma_dn7) * assign23590_e27178) - (assign23590_e27175 * var_wgamma_dn7)) / (assign23590_e27178 * assign23590_e27178)), ((((var_wsrh * var_wgamma_dn8) * assign23590_e27178) - (assign23590_e27175 * var_wgamma_dn8)) / (assign23590_e27178 * assign23590_e27178)), ((((var_wsrh * var_wgamma_dn9) * assign23590_e27178) - (assign23590_e27175 * var_wgamma_dn9)) / (assign23590_e27178 * assign23590_e27178)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign23590_e27181;
        var_wtat_dn6 = assign23590_e27181_d_n6;
        var_wtat_dn7 = assign23590_e27181_d_n7;
        var_wtat_dn8 = assign23590_e27181_d_n8;
        var_wtat_dn9 = assign23590_e27181_d_n9;

        let (assign23600_e27198, assign23600_e27198_d_n6, assign23600_e27198_d_n7, assign23600_e27198_d_n8, assign23600_e27198_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) {
        let assign23600_e27194: f64 = (var_btat / var_sqrtumax);
        let assign23600_e27195: f64 = (0.375 * assign23600_e27194);
        let assign23600_e27196: f64 = (assign23600_e27195).sqrt();
        (assign23600_e27196, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23600_e27196)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23600_e27196)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23600_e27196)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23600_e27196)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign23600_e27198;
        var_ktat_dn6 = assign23600_e27198_d_n6;
        var_ktat_dn7 = assign23600_e27198_d_n7;
        var_ktat_dn8 = assign23600_e27198_d_n8;
        var_ktat_dn9 = assign23600_e27198_d_n9;

        let (assign23610_e27216, assign23610_e27216_d_n6, assign23610_e27216_d_n7, assign23610_e27216_d_n8, assign23610_e27216_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) {
        let assign23610_e27211: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign23610_e27212: f64 = (2.0 * assign23610_e27211);
        let assign23610_e27214: f64 = (assign23610_e27212 - var_umax);
        (assign23610_e27214, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign23610_e27216;
        var_ltat_dn6 = assign23610_e27216_d_n6;
        var_ltat_dn7 = assign23610_e27216_d_n7;
        var_ltat_dn8 = assign23610_e27216_d_n8;
        var_ltat_dn9 = assign23610_e27216_d_n9;

        let (assign23620_e27242, assign23620_e27242_d_n6, assign23620_e27242_d_n7, assign23620_e27242_d_n8, assign23620_e27242_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) {
        let assign23620_e27228: f64 = (var_atatgat * var_twoatatoverthreebtat);
        let assign23620_e27230: f64 = (assign23620_e27228 * var_sqrtumax);
        let assign23620_e27233: f64 = (var_atatgat * var_umax);
        let assign23620_e27234: f64 = (assign23620_e27230 - assign23620_e27233);
        let assign23620_e27238: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign23620_e27239: f64 = (0.5 * assign23620_e27238);
        let assign23620_e27240: f64 = (assign23620_e27234 + assign23620_e27239);
        (assign23620_e27240, (((((var_atatgat * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign23620_e27228 * var_sqrtumax_dn6)) - (var_atatgat * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign23620_e27228 * var_sqrtumax_dn7)) - (var_atatgat * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign23620_e27228 * var_sqrtumax_dn8)) - (var_atatgat * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatgat * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign23620_e27228 * var_sqrtumax_dn9)) - (var_atatgat * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign23620_e27242;
        var_mtat_dn6 = assign23620_e27242_d_n6;
        var_mtat_dn7 = assign23620_e27242_d_n7;
        var_mtat_dn8 = assign23620_e27242_d_n8;
        var_mtat_dn9 = assign23620_e27242_d_n9;

        let (assign23630_e27258, assign23630_e27258_d_n6, assign23630_e27258_d_n7, assign23630_e27258_d_n8, assign23630_e27258_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) {
        let assign23630_e27254: f64 = (var_ltat - 1.0);
        let assign23630_e27256: f64 = (assign23630_e27254 * var_ktat);
        (assign23630_e27256, ((var_ltat_dn6 * var_ktat) + (assign23630_e27254 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign23630_e27254 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign23630_e27254 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign23630_e27254 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign23630_e27258;
        var_xerfc_dn6 = assign23630_e27258_d_n6;
        var_xerfc_dn7 = assign23630_e27258_d_n7;
        var_xerfc_dn8 = assign23630_e27258_d_n8;
        var_xerfc_dn9 = assign23630_e27258_d_n9;

        let (assign23640_e27272, assign23640_e27272_d_n6, assign23640_e27272_d_n7, assign23640_e27272_d_n8, assign23640_e27272_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) {
        let assign23640_e27270: f64 = (var_xerfc * var_xerfc);
        (assign23640_e27270, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign23640_e27272;
        var_ysq_dn6 = assign23640_e27272_d_n6;
        var_ysq_dn7 = assign23640_e27272_d_n7;
        var_ysq_dn8 = assign23640_e27272_d_n8;
        var_ysq_dn9 = assign23640_e27272_d_n9;

        let assign23650_e27275: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard454 = assign23650_e27275;

        let (assign23660_e27295, assign23660_e27295_d_n6, assign23660_e27295_d_n7, assign23660_e27295_d_n8, assign23660_e27295_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) && (var_guard454 != 0.0)) {
        let assign23660_e27291: f64 = (var_perfc * var_xerfc);
        let assign23660_e27292: f64 = (1.0 + assign23660_e27291);
        let assign23660_e27293: f64 = (1.0 / assign23660_e27292);
        (assign23660_e27293, (-((var_perfc * var_xerfc_dn6) / (assign23660_e27292 * assign23660_e27292))), (-((var_perfc * var_xerfc_dn7) / (assign23660_e27292 * assign23660_e27292))), (-((var_perfc * var_xerfc_dn8) / (assign23660_e27292 * assign23660_e27292))), (-((var_perfc * var_xerfc_dn9) / (assign23660_e27292 * assign23660_e27292))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign23660_e27295;
        var_terfc_dn6 = assign23660_e27295_d_n6;
        var_terfc_dn7 = assign23660_e27295_d_n7;
        var_terfc_dn8 = assign23660_e27295_d_n8;
        var_terfc_dn9 = assign23660_e27295_d_n9;

        let (assign23670_e27316, assign23670_e27316_d_n6, assign23670_e27316_d_n7, assign23670_e27316_d_n8, assign23670_e27316_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) && (var_guard454 == 0.0)) {
        let assign23670_e27312: f64 = (var_perfc * var_xerfc);
        let assign23670_e27313: f64 = (1.0 - assign23670_e27312);
        let assign23670_e27314: f64 = (1.0 / assign23670_e27313);
        (assign23670_e27314, (-((-(var_perfc * var_xerfc_dn6)) / (assign23670_e27313 * assign23670_e27313))), (-((-(var_perfc * var_xerfc_dn7)) / (assign23670_e27313 * assign23670_e27313))), (-((-(var_perfc * var_xerfc_dn8)) / (assign23670_e27313 * assign23670_e27313))), (-((-(var_perfc * var_xerfc_dn9)) / (assign23670_e27313 * assign23670_e27313))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign23670_e27316;
        var_terfc_dn6 = assign23670_e27316_d_n6;
        var_terfc_dn7 = assign23670_e27316_d_n7;
        var_terfc_dn8 = assign23670_e27316_d_n8;
        var_terfc_dn9 = assign23670_e27316_d_n9;

        let assign23680_e27318: f64 = (-var_ysq);
        let assign23680_e27320: f64 = (assign23680_e27318 + var_mtat);
        let assign23680_e27322: f64 = (-230.25850929940458);
        let assign23680_e27323: f64 = if assign23680_e27320 > assign23680_e27322 { 1.0 } else { 0.0 };
        var_guard455 = assign23680_e27323;

        let (assign23690_e27341, assign23690_e27341_d_n6, assign23690_e27341_d_n7, assign23690_e27341_d_n8, assign23690_e27341_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) && (var_guard455 != 0.0)) {
        let assign23690_e27336: f64 = (-var_ysq);
        let assign23690_e27338: f64 = (assign23690_e27336 + var_mtat);
        let assign23690_e27339: f64 = (assign23690_e27338).exp();
        (assign23690_e27339, (assign23690_e27339 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign23690_e27339 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign23690_e27339 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign23690_e27339 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23690_e27341;
        var_tmp_dn6 = assign23690_e27341_d_n6;
        var_tmp_dn7 = assign23690_e27341_d_n7;
        var_tmp_dn8 = assign23690_e27341_d_n8;
        var_tmp_dn9 = assign23690_e27341_d_n9;

        let (assign23700_e27390, assign23700_e27390_d_n6, assign23700_e27390_d_n7, assign23700_e27390_d_n8, assign23700_e27390_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) && (var_guard455 == 0.0)) {
        let assign23700_e27357: f64 = (-230.25850929940458);
        let assign23700_e27359: f64 = (-var_ysq);
        let assign23700_e27361: f64 = (assign23700_e27359 + var_mtat);
        let assign23700_e27362: f64 = (assign23700_e27357 - assign23700_e27361);
        let assign23700_e27366: f64 = (-230.25850929940458);
        let assign23700_e27368: f64 = (-var_ysq);
        let assign23700_e27370: f64 = (assign23700_e27368 + var_mtat);
        let assign23700_e27371: f64 = (assign23700_e27366 - assign23700_e27370);
        let assign23700_e27374: f64 = (-230.25850929940458);
        let assign23700_e27376: f64 = (-var_ysq);
        let assign23700_e27378: f64 = (assign23700_e27376 + var_mtat);
        let assign23700_e27379: f64 = (assign23700_e27374 - assign23700_e27378);
        let assign23700_e27381: f64 = (assign23700_e27379 * 0.3333333333333333);
        let assign23700_e27382: f64 = (1.0 + assign23700_e27381);
        let assign23700_e27383: f64 = (assign23700_e27371 * assign23700_e27382);
        let assign23700_e27384: f64 = (0.5 * assign23700_e27383);
        let assign23700_e27385: f64 = (1.0 + assign23700_e27384);
        let assign23700_e27386: f64 = (assign23700_e27362 * assign23700_e27385);
        let assign23700_e27387: f64 = (1.0 + assign23700_e27386);
        let assign23700_e27388: f64 = (1e-100 / assign23700_e27387);
        (assign23700_e27388, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign23700_e27385) + (assign23700_e27362 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign23700_e27382) + (assign23700_e27371 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign23700_e27387 * assign23700_e27387))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign23700_e27385) + (assign23700_e27362 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign23700_e27382) + (assign23700_e27371 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign23700_e27387 * assign23700_e27387))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign23700_e27385) + (assign23700_e27362 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign23700_e27382) + (assign23700_e27371 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign23700_e27387 * assign23700_e27387))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign23700_e27385) + (assign23700_e27362 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign23700_e27382) + (assign23700_e27371 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign23700_e27387 * assign23700_e27387))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23700_e27390;
        var_tmp_dn6 = assign23700_e27390_d_n6;
        var_tmp_dn7 = assign23700_e27390_d_n7;
        var_tmp_dn8 = assign23700_e27390_d_n8;
        var_tmp_dn9 = assign23700_e27390_d_n9;

        let (assign23710_e27420, assign23710_e27420_d_n6, assign23710_e27420_d_n7, assign23710_e27420_d_n8, assign23710_e27420_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) {
        let assign23710_e27402: f64 = (0.29214664 * var_terfc);
        let assign23710_e27406: f64 = (var_terfc * var_terfc);
        let assign23710_e27407: f64 = (var_berfc * assign23710_e27406);
        let assign23710_e27408: f64 = (assign23710_e27402 + assign23710_e27407);
        let assign23710_e27412: f64 = (var_terfc * var_terfc);
        let assign23710_e27414: f64 = (assign23710_e27412 * var_terfc);
        let assign23710_e27415: f64 = (var_cerfc * assign23710_e27414);
        let assign23710_e27416: f64 = (assign23710_e27408 + assign23710_e27415);
        let assign23710_e27418: f64 = (assign23710_e27416 * var_tmp);
        (assign23710_e27418, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign23710_e27412 * var_terfc_dn6)))) * var_tmp) + (assign23710_e27416 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign23710_e27412 * var_terfc_dn7)))) * var_tmp) + (assign23710_e27416 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign23710_e27412 * var_terfc_dn8)))) * var_tmp) + (assign23710_e27416 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign23710_e27412 * var_terfc_dn9)))) * var_tmp) + (assign23710_e27416 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign23710_e27420;
        var_erfcpos_dn6 = assign23710_e27420_d_n6;
        var_erfcpos_dn7 = assign23710_e27420_d_n7;
        var_erfcpos_dn8 = assign23710_e27420_d_n8;
        var_erfcpos_dn9 = assign23710_e27420_d_n9;

        let assign23720_e27423: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard456 = assign23720_e27423;

        let (assign23730_e27437, assign23730_e27437_d_n6, assign23730_e27437_d_n7, assign23730_e27437_d_n8, assign23730_e27437_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) && (var_guard456 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign23730_e27437;
        var_erfctimesexpmtat_dn6 = assign23730_e27437_d_n6;
        var_erfctimesexpmtat_dn7 = assign23730_e27437_d_n7;
        var_erfctimesexpmtat_dn8 = assign23730_e27437_d_n8;
        var_erfctimesexpmtat_dn9 = assign23730_e27437_d_n9;

        let assign23740_e27440: f64 = (-230.25850929940458);
        let assign23740_e27441: f64 = if var_mtat > assign23740_e27440 { 1.0 } else { 0.0 };
        var_guard457 = assign23740_e27441;

        let (assign23750_e27459, assign23750_e27459_d_n6, assign23750_e27459_d_n7, assign23750_e27459_d_n8, assign23750_e27459_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) && (var_guard456 == 0.0)) && (var_guard457 != 0.0)) {
        let assign23750_e27457: f64 = (var_mtat).exp();
        (assign23750_e27457, (assign23750_e27457 * var_mtat_dn6), (assign23750_e27457 * var_mtat_dn7), (assign23750_e27457 * var_mtat_dn8), (assign23750_e27457 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23750_e27459;
        var_tmp_dn6 = assign23750_e27459_d_n6;
        var_tmp_dn7 = assign23750_e27459_d_n7;
        var_tmp_dn8 = assign23750_e27459_d_n8;
        var_tmp_dn9 = assign23750_e27459_d_n9;

        let (assign23760_e27502, assign23760_e27502_d_n6, assign23760_e27502_d_n7, assign23760_e27502_d_n8, assign23760_e27502_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) && (var_guard456 == 0.0)) && (var_guard457 == 0.0)) {
        let assign23760_e27478: f64 = (-230.25850929940458);
        let assign23760_e27480: f64 = (assign23760_e27478 - var_mtat);
        let assign23760_e27484: f64 = (-230.25850929940458);
        let assign23760_e27486: f64 = (assign23760_e27484 - var_mtat);
        let assign23760_e27489: f64 = (-230.25850929940458);
        let assign23760_e27491: f64 = (assign23760_e27489 - var_mtat);
        let assign23760_e27493: f64 = (assign23760_e27491 * 0.3333333333333333);
        let assign23760_e27494: f64 = (1.0 + assign23760_e27493);
        let assign23760_e27495: f64 = (assign23760_e27486 * assign23760_e27494);
        let assign23760_e27496: f64 = (0.5 * assign23760_e27495);
        let assign23760_e27497: f64 = (1.0 + assign23760_e27496);
        let assign23760_e27498: f64 = (assign23760_e27480 * assign23760_e27497);
        let assign23760_e27499: f64 = (1.0 + assign23760_e27498);
        let assign23760_e27500: f64 = (1e-100 / assign23760_e27499);
        (assign23760_e27500, (-((1e-100 * (((-var_mtat_dn6) * assign23760_e27497) + (assign23760_e27480 * (0.5 * (((-var_mtat_dn6) * assign23760_e27494) + (assign23760_e27486 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign23760_e27499 * assign23760_e27499))), (-((1e-100 * (((-var_mtat_dn7) * assign23760_e27497) + (assign23760_e27480 * (0.5 * (((-var_mtat_dn7) * assign23760_e27494) + (assign23760_e27486 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign23760_e27499 * assign23760_e27499))), (-((1e-100 * (((-var_mtat_dn8) * assign23760_e27497) + (assign23760_e27480 * (0.5 * (((-var_mtat_dn8) * assign23760_e27494) + (assign23760_e27486 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign23760_e27499 * assign23760_e27499))), (-((1e-100 * (((-var_mtat_dn9) * assign23760_e27497) + (assign23760_e27480 * (0.5 * (((-var_mtat_dn9) * assign23760_e27494) + (assign23760_e27486 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign23760_e27499 * assign23760_e27499))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23760_e27502;
        var_tmp_dn6 = assign23760_e27502_d_n6;
        var_tmp_dn7 = assign23760_e27502_d_n7;
        var_tmp_dn8 = assign23760_e27502_d_n8;
        var_tmp_dn9 = assign23760_e27502_d_n9;

        let (assign23770_e27521, assign23770_e27521_d_n6, assign23770_e27521_d_n7, assign23770_e27521_d_n8, assign23770_e27521_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) && (var_guard456 == 0.0)) {
        let assign23770_e27517: f64 = (2.0 * var_tmp);
        let assign23770_e27519: f64 = (assign23770_e27517 - var_erfcpos);
        (assign23770_e27519, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign23770_e27521;
        var_erfctimesexpmtat_dn6 = assign23770_e27521_d_n6;
        var_erfctimesexpmtat_dn7 = assign23770_e27521_d_n7;
        var_erfctimesexpmtat_dn8 = assign23770_e27521_d_n8;
        var_erfctimesexpmtat_dn9 = assign23770_e27521_d_n9;

        let (assign23780_e27541, assign23780_e27541_d_n6, assign23780_e27541_d_n7, assign23780_e27541_d_n8, assign23780_e27541_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) {
        let assign23780_e27533: f64 = (1.772453850905516 * 0.5);
        let assign23780_e27536: f64 = (var_atatgat * var_erfctimesexpmtat);
        let assign23780_e27538: f64 = (assign23780_e27536 / var_ktat);
        let assign23780_e27539: f64 = (assign23780_e27533 * assign23780_e27538);
        (assign23780_e27539, (assign23780_e27533 * ((((var_atatgat * var_erfctimesexpmtat_dn6) * var_ktat) - (assign23780_e27536 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign23780_e27533 * ((((var_atatgat * var_erfctimesexpmtat_dn7) * var_ktat) - (assign23780_e27536 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign23780_e27533 * ((((var_atatgat * var_erfctimesexpmtat_dn8) * var_ktat) - (assign23780_e27536 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign23780_e27533 * ((((var_atatgat * var_erfctimesexpmtat_dn9) * var_ktat) - (assign23780_e27536 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign23780_e27541;
        var_gammamax_dn6 = assign23780_e27541_d_n6;
        var_gammamax_dn7 = assign23780_e27541_d_n7;
        var_gammamax_dn8 = assign23780_e27541_d_n8;
        var_gammamax_dn9 = assign23780_e27541_d_n9;

        let (assign23790_e27559, assign23790_e27559_d_n6, assign23790_e27559_d_n7, assign23790_e27559_d_n8, assign23790_e27559_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard452 == 0.0)) {
        let assign23790_e27554: f64 = (var_asrh * var_gammamax);
        let assign23790_e27556: f64 = (assign23790_e27554 * var_wtat);
        let assign23790_e27557: f64 = (p.p864 * assign23790_e27556);
        (assign23790_e27557, (p.p864 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign23790_e27554 * var_wtat_dn6))), (p.p864 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign23790_e27554 * var_wtat_dn7))), (p.p864 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign23790_e27554 * var_wtat_dn8))), (p.p864 * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign23790_e27554 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign23790_e27559;
        var_itat_dn6 = assign23790_e27559_d_n6;
        var_itat_dn7 = assign23790_e27559_d_n7;
        var_itat_dn8 = assign23790_e27559_d_n8;
        var_itat_dn9 = assign23790_e27559_d_n9;

        let assign23800_e27562: f64 = if p.p870 == 0.0 { 1.0 } else { 0.0 };
        var_guard458 = assign23800_e27562;

        let (assign23810_e27573, assign23810_e27573_d_n6, assign23810_e27573_d_n7, assign23810_e27573_d_n8, assign23810_e27573_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard458 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign23810_e27573;
        var_ibbt_dn6 = assign23810_e27573_d_n6;
        var_ibbt_dn7 = assign23810_e27573_d_n7;
        var_ibbt_dn8 = assign23810_e27573_d_n8;
        var_ibbt_dn9 = assign23810_e27573_d_n9;

        let assign23820_e27576: f64 = if p.p850 == 0.5 { 1.0 } else { 0.0 };
        var_guard459 = assign23820_e27576;

        *var_btat_slot = var_btat;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_btat_dn9_slot = var_btat_dn9;
        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfcpos_dn9_slot = var_erfcpos_dn9;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_erfctimesexpmtat_dn9_slot = var_erfctimesexpmtat_dn9;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_gammamax_dn9_slot = var_gammamax_dn9;
        *var_guard452_slot = var_guard452;
        *var_guard453_slot = var_guard453;
        *var_guard454_slot = var_guard454;
        *var_guard455_slot = var_guard455;
        *var_guard456_slot = var_guard456;
        *var_guard457_slot = var_guard457;
        *var_guard458_slot = var_guard458;
        *var_guard459_slot = var_guard459;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ktat_dn9_slot = var_ktat_dn9;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_ltat_dn9_slot = var_ltat_dn9;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_mtat_dn9_slot = var_mtat_dn9;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_sqrtumax_dn9_slot = var_sqrtumax_dn9;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_terfc_dn9_slot = var_terfc_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_twoatatoverthreebtat_dn9_slot = var_twoatatoverthreebtat_dn9;
        *var_umax_slot = var_umax;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umax_dn9_slot = var_umax_dn9;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxbeforelimiting_dn9_slot = var_umaxbeforelimiting_dn9;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_umaxpoweronepointfive_dn9_slot = var_umaxpoweronepointfive_dn9;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wgamma_dn9_slot = var_wgamma_dn9;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_wtat_dn9_slot = var_wtat_dn9;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_xerfc_dn9_slot = var_xerfc_dn9;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
        *var_ysq_dn9_slot = var_ysq_dn9;
    }

    pub(super) fn stamp_transient_block_45(
        p: &Parameters,
        var_absource_i: f64,
        var_alphaav: f64,
        var_exp_vmax_over_phitd_s: f64,
        var_fbbtgat: f64,
        var_fbbtgat_dn6: f64,
        var_fbbtgat_dn7: f64,
        var_fbbtgat_dn8: f64,
        var_fbbtgat_dn9: f64,
        var_fstopgat: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard448: f64,
        var_guard458: f64,
        var_guard459: f64,
        var_id__blk212: f64,
        var_ijunbot: f64,
        var_ijunbot_dn6: f64,
        var_ijunbot_dn7: f64,
        var_ijunbot_dn8: f64,
        var_ijunbot_dn9: f64,
        var_ijunsti: f64,
        var_ijunsti_dn6: f64,
        var_ijunsti_dn7: f64,
        var_ijunsti_dn8: f64,
        var_ijunsti_dn9: f64,
        var_isrh: f64,
        var_isrh_dn6: f64,
        var_isrh_dn7: f64,
        var_isrh_dn8: f64,
        var_isrh_dn9: f64,
        var_itat: f64,
        var_itat_dn6: f64,
        var_itat_dn7: f64,
        var_itat_dn8: f64,
        var_itat_dn9: f64,
        var_lgsource_i: f64,
        var_lssource_i: f64,
        var_one_over_one_minus_pgat: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_slopegat: f64,
        var_slopegat_dn6: f64,
        var_slopegat_dn7: f64,
        var_slopegat_dn8: f64,
        var_slopegat_dn9: f64,
        var_v4: f64,
        var_v5: f64,
        var_vav: f64,
        var_vbimin_s: f64,
        var_vbirgatinv: f64,
        var_vbrinvgat: f64,
        var_vbrinvgat_dn6: f64,
        var_vbrinvgat_dn7: f64,
        var_vbrinvgat_dn8: f64,
        var_vbrinvgat_dn9: f64,
        var_vmax_s: f64,
        var_wdepnulrinvgat: f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_fmaxr_dn9_slot: &mut f64,
        var_guard460_slot: &mut f64,
        var_guard461_slot: &mut f64,
        var_guard462_slot: &mut f64,
        var_guard463_slot: &mut f64,
        var_guard464_slot: &mut f64,
        var_guard465_slot: &mut f64,
        var_guard466_slot: &mut f64,
        var_guard467_slot: &mut f64,
        var_guard468_slot: &mut f64,
        var_guard469_slot: &mut f64,
        var_i4_slot: &mut f64,
        var_i4_dn6_slot: &mut f64,
        var_i4_dn7_slot: &mut f64,
        var_i4_dn8_slot: &mut f64,
        var_i4_dn9_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_ijungat_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fbreakdown_dn9: f64 = *var_fbreakdown_dn9_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_fmaxr_dn9: f64 = *var_fmaxr_dn9_slot;
        let mut var_guard460: f64 = *var_guard460_slot;
        let mut var_guard461: f64 = *var_guard461_slot;
        let mut var_guard462: f64 = *var_guard462_slot;
        let mut var_guard463: f64 = *var_guard463_slot;
        let mut var_guard464: f64 = *var_guard464_slot;
        let mut var_guard465: f64 = *var_guard465_slot;
        let mut var_guard466: f64 = *var_guard466_slot;
        let mut var_guard467: f64 = *var_guard467_slot;
        let mut var_guard468: f64 = *var_guard468_slot;
        let mut var_guard469: f64 = *var_guard469_slot;
        let mut var_i4: f64 = *var_i4_slot;
        let mut var_i4_dn6: f64 = *var_i4_dn6_slot;
        let mut var_i4_dn7: f64 = *var_i4_dn7_slot;
        let mut var_i4_dn8: f64 = *var_i4_dn8_slot;
        let mut var_i4_dn9: f64 = *var_i4_dn9_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_ijungat_dn9: f64 = *var_ijungat_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign23830_e27595, assign23830_e27595_d_n6, assign23830_e27595_d_n7, assign23830_e27595_d_n8, assign23830_e27595_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard458 == 0.0)) && (var_guard459 != 0.0)) {
        let assign23830_e27590: f64 = (p.p847 - var_vbbt);
        let assign23830_e27592: f64 = (assign23830_e27590 * var_vbirgatinv);
        let assign23830_e27593: f64 = (assign23830_e27592).sqrt();
        (assign23830_e27593, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23830_e27595;
        var_tmp_dn6 = assign23830_e27595_d_n6;
        var_tmp_dn7 = assign23830_e27595_d_n7;
        var_tmp_dn8 = assign23830_e27595_d_n8;
        var_tmp_dn9 = assign23830_e27595_d_n9;

        let (assign23840_e27616, assign23840_e27616_d_n6, assign23840_e27616_d_n7, assign23840_e27616_d_n8, assign23840_e27616_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard458 == 0.0)) && (var_guard459 == 0.0)) {
        let assign23840_e27610: f64 = (p.p847 - var_vbbt);
        let assign23840_e27612: f64 = (assign23840_e27610 * var_vbirgatinv);
        let assign23840_e27614: f64 = (assign23840_e27612).powf(p.p850);
        (assign23840_e27614, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23840_e27616;
        var_tmp_dn6 = assign23840_e27616_d_n6;
        var_tmp_dn7 = assign23840_e27616_d_n7;
        var_tmp_dn8 = assign23840_e27616_d_n8;
        var_tmp_dn9 = assign23840_e27616_d_n9;

        let (assign23850_e27636, assign23850_e27636_d_n6, assign23850_e27636_d_n7, assign23850_e27636_d_n8, assign23850_e27636_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard458 == 0.0)) {
        let assign23850_e27629: f64 = (p.p847 - var_vbbt);
        let assign23850_e27631: f64 = (assign23850_e27629 * var_wdepnulrinvgat);
        let assign23850_e27633: f64 = (assign23850_e27631 / var_tmp);
        let assign23850_e27634: f64 = (var_one_over_one_minus_pgat * assign23850_e27633);
        (assign23850_e27634, (var_one_over_one_minus_pgat * (-((assign23850_e27631 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign23850_e27631 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign23850_e27631 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign23850_e27631 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign23850_e27636;
        var_fmaxr_dn6 = assign23850_e27636_d_n6;
        var_fmaxr_dn7 = assign23850_e27636_d_n7;
        var_fmaxr_dn8 = assign23850_e27636_d_n8;
        var_fmaxr_dn9 = assign23850_e27636_d_n9;

        let assign23860_e27638: f64 = (-var_fbbtgat);
        let assign23860_e27640: f64 = (assign23860_e27638 / var_fmaxr);
        let assign23860_e27641: f64 = (assign23860_e27640).abs();
        let assign23860_e27643: f64 = if assign23860_e27641 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard460 = assign23860_e27643;

        let (assign23870_e27661, assign23870_e27661_d_n6, assign23870_e27661_d_n7, assign23870_e27661_d_n8, assign23870_e27661_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard458 == 0.0)) && (var_guard460 != 0.0)) {
        let assign23870_e27656: f64 = (-var_fbbtgat);
        let assign23870_e27658: f64 = (assign23870_e27656 / var_fmaxr);
        let assign23870_e27659: f64 = (assign23870_e27658).exp();
        (assign23870_e27659, (assign23870_e27659 * ((((-var_fbbtgat_dn6) * var_fmaxr) - (assign23870_e27656 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign23870_e27659 * ((((-var_fbbtgat_dn7) * var_fmaxr) - (assign23870_e27656 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign23870_e27659 * ((((-var_fbbtgat_dn8) * var_fmaxr) - (assign23870_e27656 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))), (assign23870_e27659 * ((((-var_fbbtgat_dn9) * var_fmaxr) - (assign23870_e27656 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23870_e27661;
        var_tmp_dn6 = assign23870_e27661_d_n6;
        var_tmp_dn7 = assign23870_e27661_d_n7;
        var_tmp_dn8 = assign23870_e27661_d_n8;
        var_tmp_dn9 = assign23870_e27661_d_n9;

        let assign23880_e27663: f64 = (-var_fbbtgat);
        let assign23880_e27665: f64 = (assign23880_e27663 / var_fmaxr);
        let assign23880_e27667: f64 = if assign23880_e27665 < 0.0 { 1.0 } else { 0.0 };
        var_guard461 = assign23880_e27667;

        let (assign23890_e27718, assign23890_e27718_d_n6, assign23890_e27718_d_n7, assign23890_e27718_d_n8, assign23890_e27718_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard458 == 0.0)) && (var_guard460 == 0.0)) && (var_guard461 != 0.0)) {
        let assign23890_e27685: f64 = (-230.25850929940458);
        let assign23890_e27687: f64 = (-var_fbbtgat);
        let assign23890_e27689: f64 = (assign23890_e27687 / var_fmaxr);
        let assign23890_e27690: f64 = (assign23890_e27685 - assign23890_e27689);
        let assign23890_e27694: f64 = (-230.25850929940458);
        let assign23890_e27696: f64 = (-var_fbbtgat);
        let assign23890_e27698: f64 = (assign23890_e27696 / var_fmaxr);
        let assign23890_e27699: f64 = (assign23890_e27694 - assign23890_e27698);
        let assign23890_e27702: f64 = (-230.25850929940458);
        let assign23890_e27704: f64 = (-var_fbbtgat);
        let assign23890_e27706: f64 = (assign23890_e27704 / var_fmaxr);
        let assign23890_e27707: f64 = (assign23890_e27702 - assign23890_e27706);
        let assign23890_e27709: f64 = (assign23890_e27707 * 0.3333333333333333);
        let assign23890_e27710: f64 = (1.0 + assign23890_e27709);
        let assign23890_e27711: f64 = (assign23890_e27699 * assign23890_e27710);
        let assign23890_e27712: f64 = (0.5 * assign23890_e27711);
        let assign23890_e27713: f64 = (1.0 + assign23890_e27712);
        let assign23890_e27714: f64 = (assign23890_e27690 * assign23890_e27713);
        let assign23890_e27715: f64 = (1.0 + assign23890_e27714);
        let assign23890_e27716: f64 = (1e-100 / assign23890_e27715);
        (assign23890_e27716, (-((1e-100 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign23890_e27687 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign23890_e27713) + (assign23890_e27690 * (0.5 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign23890_e27696 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign23890_e27710) + (assign23890_e27699 * ((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign23890_e27704 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign23890_e27715 * assign23890_e27715))), (-((1e-100 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign23890_e27687 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign23890_e27713) + (assign23890_e27690 * (0.5 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign23890_e27696 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign23890_e27710) + (assign23890_e27699 * ((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign23890_e27704 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign23890_e27715 * assign23890_e27715))), (-((1e-100 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign23890_e27687 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign23890_e27713) + (assign23890_e27690 * (0.5 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign23890_e27696 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign23890_e27710) + (assign23890_e27699 * ((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign23890_e27704 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign23890_e27715 * assign23890_e27715))), (-((1e-100 * (((-((((-var_fbbtgat_dn9) * var_fmaxr) - (assign23890_e27687 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign23890_e27713) + (assign23890_e27690 * (0.5 * (((-((((-var_fbbtgat_dn9) * var_fmaxr) - (assign23890_e27696 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign23890_e27710) + (assign23890_e27699 * ((-((((-var_fbbtgat_dn9) * var_fmaxr) - (assign23890_e27704 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign23890_e27715 * assign23890_e27715))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23890_e27718;
        var_tmp_dn6 = assign23890_e27718_d_n6;
        var_tmp_dn7 = assign23890_e27718_d_n7;
        var_tmp_dn8 = assign23890_e27718_d_n8;
        var_tmp_dn9 = assign23890_e27718_d_n9;

        let (assign23900_e27767, assign23900_e27767_d_n6, assign23900_e27767_d_n7, assign23900_e27767_d_n8, assign23900_e27767_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard458 == 0.0)) && (var_guard460 == 0.0)) && (var_guard461 == 0.0)) {
        let assign23900_e27737: f64 = (-var_fbbtgat);
        let assign23900_e27739: f64 = (assign23900_e27737 / var_fmaxr);
        let assign23900_e27741: f64 = (assign23900_e27739 - 230.25850929940458);
        let assign23900_e27745: f64 = (-var_fbbtgat);
        let assign23900_e27747: f64 = (assign23900_e27745 / var_fmaxr);
        let assign23900_e27749: f64 = (assign23900_e27747 - 230.25850929940458);
        let assign23900_e27752: f64 = (-var_fbbtgat);
        let assign23900_e27754: f64 = (assign23900_e27752 / var_fmaxr);
        let assign23900_e27756: f64 = (assign23900_e27754 - 230.25850929940458);
        let assign23900_e27758: f64 = (assign23900_e27756 * 0.3333333333333333);
        let assign23900_e27759: f64 = (1.0 + assign23900_e27758);
        let assign23900_e27760: f64 = (assign23900_e27749 * assign23900_e27759);
        let assign23900_e27761: f64 = (0.5 * assign23900_e27760);
        let assign23900_e27762: f64 = (1.0 + assign23900_e27761);
        let assign23900_e27763: f64 = (assign23900_e27741 * assign23900_e27762);
        let assign23900_e27764: f64 = (1.0 + assign23900_e27763);
        let assign23900_e27765: f64 = (1e100 * assign23900_e27764);
        (assign23900_e27765, (1e100 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign23900_e27737 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign23900_e27762) + (assign23900_e27741 * (0.5 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign23900_e27745 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign23900_e27759) + (assign23900_e27749 * (((((-var_fbbtgat_dn6) * var_fmaxr) - (assign23900_e27752 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign23900_e27737 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign23900_e27762) + (assign23900_e27741 * (0.5 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign23900_e27745 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign23900_e27759) + (assign23900_e27749 * (((((-var_fbbtgat_dn7) * var_fmaxr) - (assign23900_e27752 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign23900_e27737 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign23900_e27762) + (assign23900_e27741 * (0.5 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign23900_e27745 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign23900_e27759) + (assign23900_e27749 * (((((-var_fbbtgat_dn8) * var_fmaxr) - (assign23900_e27752 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn9) * var_fmaxr) - (assign23900_e27737 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign23900_e27762) + (assign23900_e27741 * (0.5 * ((((((-var_fbbtgat_dn9) * var_fmaxr) - (assign23900_e27745 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign23900_e27759) + (assign23900_e27749 * (((((-var_fbbtgat_dn9) * var_fmaxr) - (assign23900_e27752 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23900_e27767;
        var_tmp_dn6 = assign23900_e27767_d_n6;
        var_tmp_dn7 = assign23900_e27767_d_n7;
        var_tmp_dn8 = assign23900_e27767_d_n8;
        var_tmp_dn9 = assign23900_e27767_d_n9;

        let (assign23910_e27787, assign23910_e27787_d_n6, assign23910_e27787_d_n7, assign23910_e27787_d_n8, assign23910_e27787_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard458 == 0.0)) {
        let assign23910_e27780: f64 = (var_v4 * var_fmaxr);
        let assign23910_e27782: f64 = (assign23910_e27780 * var_fmaxr);
        let assign23910_e27784: f64 = (assign23910_e27782 * var_tmp);
        let assign23910_e27785: f64 = (p.p870 * assign23910_e27784);
        (assign23910_e27785, (p.p870 * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign23910_e27780 * var_fmaxr_dn6)) * var_tmp) + (assign23910_e27782 * var_tmp_dn6))), (p.p870 * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign23910_e27780 * var_fmaxr_dn7)) * var_tmp) + (assign23910_e27782 * var_tmp_dn7))), (p.p870 * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign23910_e27780 * var_fmaxr_dn8)) * var_tmp) + (assign23910_e27782 * var_tmp_dn8))), (p.p870 * (((((var_v4 * var_fmaxr_dn9) * var_fmaxr) + (assign23910_e27780 * var_fmaxr_dn9)) * var_tmp) + (assign23910_e27782 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign23910_e27787;
        var_ibbt_dn6 = assign23910_e27787_d_n6;
        var_ibbt_dn7 = assign23910_e27787_d_n7;
        var_ibbt_dn8 = assign23910_e27787_d_n8;
        var_ibbt_dn9 = assign23910_e27787_d_n9;

        let assign23920_e27790: f64 = if p.p879 > 1000.0 { 1.0 } else { 0.0 };
        var_guard462 = assign23920_e27790;

        let (assign23930_e27801, assign23930_e27801_d_n6, assign23930_e27801_d_n7, assign23930_e27801_d_n8, assign23930_e27801_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard462 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign23930_e27801;
        var_fbreakdown_dn6 = assign23930_e27801_d_n6;
        var_fbreakdown_dn7 = assign23930_e27801_d_n7;
        var_fbreakdown_dn8 = assign23930_e27801_d_n8;
        var_fbreakdown_dn9 = assign23930_e27801_d_n9;

        let assign23940_e27804: f64 = (-var_alphaav);
        let assign23940_e27806: f64 = (assign23940_e27804 * p.p879);
        let assign23940_e27807: f64 = if var_vav > assign23940_e27806 { 1.0 } else { 0.0 };
        var_guard463 = assign23940_e27807;

        let assign23950_e27810: f64 = if p.p882 == 4.0 { 1.0 } else { 0.0 };
        var_guard464 = assign23950_e27810;

        let (assign23960_e27840, assign23960_e27840_d_n6, assign23960_e27840_d_n7, assign23960_e27840_d_n8, assign23960_e27840_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard462 == 0.0)) && (var_guard463 != 0.0)) && (var_guard464 != 0.0)) {
        let assign23960_e27826: f64 = (var_vav * var_vbrinvgat);
        let assign23960_e27829: f64 = (var_vav * var_vbrinvgat);
        let assign23960_e27830: f64 = (assign23960_e27826 * assign23960_e27829);
        let assign23960_e27833: f64 = (var_vav * var_vbrinvgat);
        let assign23960_e27834: f64 = (assign23960_e27830 * assign23960_e27833);
        let assign23960_e27837: f64 = (var_vav * var_vbrinvgat);
        let assign23960_e27838: f64 = (assign23960_e27834 * assign23960_e27837);
        (assign23960_e27838, (((((((var_vav * var_vbrinvgat_dn6) * assign23960_e27829) + (assign23960_e27826 * (var_vav * var_vbrinvgat_dn6))) * assign23960_e27833) + (assign23960_e27830 * (var_vav * var_vbrinvgat_dn6))) * assign23960_e27837) + (assign23960_e27834 * (var_vav * var_vbrinvgat_dn6))), (((((((var_vav * var_vbrinvgat_dn7) * assign23960_e27829) + (assign23960_e27826 * (var_vav * var_vbrinvgat_dn7))) * assign23960_e27833) + (assign23960_e27830 * (var_vav * var_vbrinvgat_dn7))) * assign23960_e27837) + (assign23960_e27834 * (var_vav * var_vbrinvgat_dn7))), (((((((var_vav * var_vbrinvgat_dn8) * assign23960_e27829) + (assign23960_e27826 * (var_vav * var_vbrinvgat_dn8))) * assign23960_e27833) + (assign23960_e27830 * (var_vav * var_vbrinvgat_dn8))) * assign23960_e27837) + (assign23960_e27834 * (var_vav * var_vbrinvgat_dn8))), (((((((var_vav * var_vbrinvgat_dn9) * assign23960_e27829) + (assign23960_e27826 * (var_vav * var_vbrinvgat_dn9))) * assign23960_e27833) + (assign23960_e27830 * (var_vav * var_vbrinvgat_dn9))) * assign23960_e27837) + (assign23960_e27834 * (var_vav * var_vbrinvgat_dn9))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23960_e27840;
        var_tmp_dn6 = assign23960_e27840_d_n6;
        var_tmp_dn7 = assign23960_e27840_d_n7;
        var_tmp_dn8 = assign23960_e27840_d_n8;
        var_tmp_dn9 = assign23960_e27840_d_n9;

        let (assign23970_e27862, assign23970_e27862_d_n6, assign23970_e27862_d_n7, assign23970_e27862_d_n8, assign23970_e27862_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard462 == 0.0)) && (var_guard463 != 0.0)) && (var_guard464 == 0.0)) {
        let assign23970_e27857: f64 = (var_vav * var_vbrinvgat);
        let assign23970_e27858: f64 = (assign23970_e27857).abs();
        let assign23970_e27860: f64 = (assign23970_e27858).powf(p.p882);
        (assign23970_e27860, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign23970_e27858).powf(p.p882 - 1.0) * if assign23970_e27857 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) })) } } else { (assign23970_e27860 * (p.p882 * (if assign23970_e27857 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) } / assign23970_e27858))) }, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign23970_e27858).powf(p.p882 - 1.0) * if assign23970_e27857 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) })) } } else { (assign23970_e27860 * (p.p882 * (if assign23970_e27857 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) } / assign23970_e27858))) }, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign23970_e27858).powf(p.p882 - 1.0) * if assign23970_e27857 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) })) } } else { (assign23970_e27860 * (p.p882 * (if assign23970_e27857 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) } / assign23970_e27858))) }, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign23970_e27858).powf(p.p882 - 1.0) * if assign23970_e27857 >= 0.0 { (var_vav * var_vbrinvgat_dn9) } else { (-(var_vav * var_vbrinvgat_dn9)) })) } } else { (assign23970_e27860 * (p.p882 * (if assign23970_e27857 >= 0.0 { (var_vav * var_vbrinvgat_dn9) } else { (-(var_vav * var_vbrinvgat_dn9)) } / assign23970_e27858))) },)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign23970_e27862;
        var_tmp_dn6 = assign23970_e27862_d_n6;
        var_tmp_dn7 = assign23970_e27862_d_n7;
        var_tmp_dn8 = assign23970_e27862_d_n8;
        var_tmp_dn9 = assign23970_e27862_d_n9;

        let (assign23980_e27880, assign23980_e27880_d_n6, assign23980_e27880_d_n7, assign23980_e27880_d_n8, assign23980_e27880_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard462 == 0.0)) && (var_guard463 != 0.0)) {
        let assign23980_e27877: f64 = (1.0 - var_tmp);
        let assign23980_e27878: f64 = (1.0 / assign23980_e27877);
        (assign23980_e27878, (-((-var_tmp_dn6) / (assign23980_e27877 * assign23980_e27877))), (-((-var_tmp_dn7) / (assign23980_e27877 * assign23980_e27877))), (-((-var_tmp_dn8) / (assign23980_e27877 * assign23980_e27877))), (-((-var_tmp_dn9) / (assign23980_e27877 * assign23980_e27877))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign23980_e27880;
        var_fbreakdown_dn6 = assign23980_e27880_d_n6;
        var_fbreakdown_dn7 = assign23980_e27880_d_n7;
        var_fbreakdown_dn8 = assign23980_e27880_d_n8;
        var_fbreakdown_dn9 = assign23980_e27880_d_n9;

        let (assign23990_e27903, assign23990_e27903_d_n6, assign23990_e27903_d_n7, assign23990_e27903_d_n8, assign23990_e27903_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) && (var_guard462 == 0.0)) && (var_guard463 == 0.0)) {
        let assign23990_e27897: f64 = (var_alphaav * p.p879);
        let assign23990_e27898: f64 = (var_vav + assign23990_e27897);
        let assign23990_e27900: f64 = (assign23990_e27898 * var_slopegat);
        let assign23990_e27901: f64 = (var_fstopgat + assign23990_e27900);
        (assign23990_e27901, (assign23990_e27898 * var_slopegat_dn6), (assign23990_e27898 * var_slopegat_dn7), (assign23990_e27898 * var_slopegat_dn8), (assign23990_e27898 * var_slopegat_dn9),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign23990_e27903;
        var_fbreakdown_dn6 = assign23990_e27903_d_n6;
        var_fbreakdown_dn7 = assign23990_e27903_d_n7;
        var_fbreakdown_dn8 = assign23990_e27903_d_n8;
        var_fbreakdown_dn9 = assign23990_e27903_d_n9;

        let (assign24000_e27922, assign24000_e27922_d_n6, assign24000_e27922_d_n7, assign24000_e27922_d_n8, assign24000_e27922_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard448 == 0.0)) {
        let assign24000_e27913: f64 = (var_id__blk212 + var_isrh);
        let assign24000_e27915: f64 = (assign24000_e27913 + var_itat);
        let assign24000_e27917: f64 = (assign24000_e27915 + var_ibbt);
        let assign24000_e27918: f64 = (p.p29 * assign24000_e27917);
        let assign24000_e27920: f64 = (assign24000_e27918 * var_fbreakdown);
        (assign24000_e27920, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign24000_e27918 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign24000_e27918 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign24000_e27918 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign24000_e27918 * var_fbreakdown_dn9)),)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign24000_e27922;
        var_ijungat_dn6 = assign24000_e27922_d_n6;
        var_ijungat_dn7 = assign24000_e27922_d_n7;
        var_ijungat_dn8 = assign24000_e27922_d_n8;
        var_ijungat_dn9 = assign24000_e27922_d_n9;

        let (assign24010_e27938, assign24010_e27938_d_n6, assign24010_e27938_d_n7, assign24010_e27938_d_n8, assign24010_e27938_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign24010_e27928: f64 = (var_absource_i * var_ijunbot);
        let assign24010_e27931: f64 = (var_lssource_i * var_ijunsti);
        let assign24010_e27932: f64 = (assign24010_e27928 + assign24010_e27931);
        let assign24010_e27935: f64 = (var_lgsource_i * var_ijungat);
        let assign24010_e27936: f64 = (assign24010_e27932 + assign24010_e27935);
        (assign24010_e27936, (((var_absource_i * var_ijunbot_dn6) + (var_lssource_i * var_ijunsti_dn6)) + (var_lgsource_i * var_ijungat_dn6)), (((var_absource_i * var_ijunbot_dn7) + (var_lssource_i * var_ijunsti_dn7)) + (var_lgsource_i * var_ijungat_dn7)), (((var_absource_i * var_ijunbot_dn8) + (var_lssource_i * var_ijunsti_dn8)) + (var_lgsource_i * var_ijungat_dn8)), (((var_absource_i * var_ijunbot_dn9) + (var_lssource_i * var_ijunsti_dn9)) + (var_lgsource_i * var_ijungat_dn9)),)
    } else {
        (var_i4, var_i4_dn6, var_i4_dn7, var_i4_dn8, var_i4_dn9,)
    }
};
        var_i4 = assign24010_e27938;
        var_i4_dn6 = assign24010_e27938_d_n6;
        var_i4_dn7 = assign24010_e27938_d_n7;
        var_i4_dn8 = assign24010_e27938_d_n8;
        var_i4_dn9 = assign24010_e27938_d_n9;

        let (assign24020_e27944,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign24020_e27944;

        let (assign24030_e27950,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign24030_e27950;

        let assign24040_e27962: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard465 = assign24040_e27962;

        let assign24120_e28048: f64 = if var_v5 < var_vmax_s { 1.0 } else { 0.0 };
        var_guard466 = assign24120_e28048;

        let assign24130_e28050: f64 = (-0.5);
        let assign24130_e28053: f64 = (var_v5 * var_phitdinv);
        let assign24130_e28054: f64 = (assign24130_e28050 * assign24130_e28053);
        let assign24130_e28055: f64 = (assign24130_e28054).abs();
        let assign24130_e28057: f64 = if assign24130_e28055 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard467 = assign24130_e28057;

        let (assign24140_e28075,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard465 != 0.0)) && (var_guard466 != 0.0)) && (var_guard467 != 0.0)) {
        let assign24140_e28068: f64 = (-0.5);
        let assign24140_e28071: f64 = (var_v5 * var_phitdinv);
        let assign24140_e28072: f64 = (assign24140_e28068 * assign24140_e28071);
        let assign24140_e28073: f64 = (assign24140_e28072).exp();
        (assign24140_e28073,)
    } else {
        (var_z,)
    }
};
        var_z = assign24140_e28075;

        let assign24150_e28077: f64 = (-0.5);
        let assign24150_e28080: f64 = (var_v5 * var_phitdinv);
        let assign24150_e28081: f64 = (assign24150_e28077 * assign24150_e28080);
        let assign24150_e28083: f64 = if assign24150_e28081 < 0.0 { 1.0 } else { 0.0 };
        var_guard468 = assign24150_e28083;

        let (assign24160_e28138,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard465 != 0.0)) && (var_guard466 != 0.0)) && (var_guard467 == 0.0)) && (var_guard468 != 0.0)) {
        let assign24160_e28099: f64 = (-230.25850929940458);
        let assign24160_e28101: f64 = (-0.5);
        let assign24160_e28104: f64 = (var_v5 * var_phitdinv);
        let assign24160_e28105: f64 = (assign24160_e28101 * assign24160_e28104);
        let assign24160_e28106: f64 = (assign24160_e28099 - assign24160_e28105);
        let assign24160_e28110: f64 = (-230.25850929940458);
        let assign24160_e28112: f64 = (-0.5);
        let assign24160_e28115: f64 = (var_v5 * var_phitdinv);
        let assign24160_e28116: f64 = (assign24160_e28112 * assign24160_e28115);
        let assign24160_e28117: f64 = (assign24160_e28110 - assign24160_e28116);
        let assign24160_e28120: f64 = (-230.25850929940458);
        let assign24160_e28122: f64 = (-0.5);
        let assign24160_e28125: f64 = (var_v5 * var_phitdinv);
        let assign24160_e28126: f64 = (assign24160_e28122 * assign24160_e28125);
        let assign24160_e28127: f64 = (assign24160_e28120 - assign24160_e28126);
        let assign24160_e28129: f64 = (assign24160_e28127 * 0.3333333333333333);
        let assign24160_e28130: f64 = (1.0 + assign24160_e28129);
        let assign24160_e28131: f64 = (assign24160_e28117 * assign24160_e28130);
        let assign24160_e28132: f64 = (0.5 * assign24160_e28131);
        let assign24160_e28133: f64 = (1.0 + assign24160_e28132);
        let assign24160_e28134: f64 = (assign24160_e28106 * assign24160_e28133);
        let assign24160_e28135: f64 = (1.0 + assign24160_e28134);
        let assign24160_e28136: f64 = (1e-100 / assign24160_e28135);
        (assign24160_e28136,)
    } else {
        (var_z,)
    }
};
        var_z = assign24160_e28138;

        let (assign24170_e28191,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard465 != 0.0)) && (var_guard466 != 0.0)) && (var_guard467 == 0.0)) && (var_guard468 == 0.0)) {
        let assign24170_e28155: f64 = (-0.5);
        let assign24170_e28158: f64 = (var_v5 * var_phitdinv);
        let assign24170_e28159: f64 = (assign24170_e28155 * assign24170_e28158);
        let assign24170_e28161: f64 = (assign24170_e28159 - 230.25850929940458);
        let assign24170_e28165: f64 = (-0.5);
        let assign24170_e28168: f64 = (var_v5 * var_phitdinv);
        let assign24170_e28169: f64 = (assign24170_e28165 * assign24170_e28168);
        let assign24170_e28171: f64 = (assign24170_e28169 - 230.25850929940458);
        let assign24170_e28174: f64 = (-0.5);
        let assign24170_e28177: f64 = (var_v5 * var_phitdinv);
        let assign24170_e28178: f64 = (assign24170_e28174 * assign24170_e28177);
        let assign24170_e28180: f64 = (assign24170_e28178 - 230.25850929940458);
        let assign24170_e28182: f64 = (assign24170_e28180 * 0.3333333333333333);
        let assign24170_e28183: f64 = (1.0 + assign24170_e28182);
        let assign24170_e28184: f64 = (assign24170_e28171 * assign24170_e28183);
        let assign24170_e28185: f64 = (0.5 * assign24170_e28184);
        let assign24170_e28186: f64 = (1.0 + assign24170_e28185);
        let assign24170_e28187: f64 = (assign24170_e28161 * assign24170_e28186);
        let assign24170_e28188: f64 = (1.0 + assign24170_e28187);
        let assign24170_e28189: f64 = (1e100 * assign24170_e28188);
        (assign24170_e28189,)
    } else {
        (var_z,)
    }
};
        var_z = assign24170_e28191;

        let (assign24180_e28203,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard465 != 0.0)) && (var_guard466 != 0.0)) {
        let assign24180_e28201: f64 = (1.0 / var_z);
        (assign24180_e28201,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign24180_e28203;

        let (assign24190_e28215,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard465 != 0.0)) && (var_guard466 != 0.0)) {
        let assign24190_e28213: f64 = (var_zinv * var_zinv);
        (assign24190_e28213,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign24190_e28215;

        let (assign24200_e28234,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard465 != 0.0)) && (var_guard466 == 0.0)) {
        let assign24200_e28227: f64 = (var_v5 - var_vmax_s);
        let assign24200_e28229: f64 = (assign24200_e28227 * var_phitdinv);
        let assign24200_e28230: f64 = (1.0 + assign24200_e28229);
        let assign24200_e28232: f64 = (assign24200_e28230 * var_exp_vmax_over_phitd_s);
        (assign24200_e28232,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign24200_e28234;

        let (assign24210_e28246,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard465 != 0.0)) && (var_guard466 == 0.0)) {
        let assign24210_e28244: f64 = (var_idmult).sqrt();
        (assign24210_e28244,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign24210_e28246;

        let (assign24220_e28259,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard465 != 0.0)) && (var_guard466 == 0.0)) {
        let assign24220_e28257: f64 = (1.0 / var_zinv);
        (assign24220_e28257,)
    } else {
        (var_z,)
    }
};
        var_z = assign24220_e28259;

        let (assign24230_e28269,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard465 != 0.0)) {
        let assign24230_e28267: f64 = (var_idmult - 1.0);
        (assign24230_e28267,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign24230_e28269;

        let assign24240_e28272: f64 = if var_v5 > 0.0 { 1.0 } else { 0.0 };
        var_guard469 = assign24240_e28272;

        let (assign24250_e28298,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard465 != 0.0)) && (var_guard469 != 0.0)) {
        let assign24250_e28284: f64 = (2.0 + var_z);
        let assign24250_e28287: f64 = (var_z + 1.0);
        let assign24250_e28290: f64 = (var_z + 3.0);
        let assign24250_e28291: f64 = (assign24250_e28287 * assign24250_e28290);
        let assign24250_e28292: f64 = (assign24250_e28291).sqrt();
        let assign24250_e28293: f64 = (assign24250_e28284 + assign24250_e28292);
        let assign24250_e28294: f64 = (assign24250_e28293).ln();
        let assign24250_e28295: f64 = (var_phitd * assign24250_e28294);
        let assign24250_e28296: f64 = (2.0 * assign24250_e28295);
        (assign24250_e28296,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign24250_e28298;

        let (assign24260_e28332,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard465 != 0.0)) && (var_guard469 == 0.0)) {
        let assign24260_e28308: f64 = (-var_v5);
        let assign24260_e28313: f64 = (2.0 * var_zinv);
        let assign24260_e28315: f64 = (assign24260_e28313 + 1.0);
        let assign24260_e28318: f64 = (1.0 + var_zinv);
        let assign24260_e28322: f64 = (3.0 * var_zinv);
        let assign24260_e28323: f64 = (1.0 + assign24260_e28322);
        let assign24260_e28324: f64 = (assign24260_e28318 * assign24260_e28323);
        let assign24260_e28325: f64 = (assign24260_e28324).sqrt();
        let assign24260_e28326: f64 = (assign24260_e28315 + assign24260_e28325);
        let assign24260_e28327: f64 = (assign24260_e28326).ln();
        let assign24260_e28328: f64 = (var_phitd * assign24260_e28327);
        let assign24260_e28329: f64 = (2.0 * assign24260_e28328);
        let assign24260_e28330: f64 = (assign24260_e28308 + assign24260_e28329);
        (assign24260_e28330,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign24260_e28332;

        let (assign24270_e28342,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard465 != 0.0)) {
        let assign24270_e28340: f64 = (var_vbimin_s - var_two_psistar);
        (assign24270_e28340,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign24270_e28342;

        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_fmaxr_dn9_slot = var_fmaxr_dn9;
        *var_guard460_slot = var_guard460;
        *var_guard461_slot = var_guard461;
        *var_guard462_slot = var_guard462;
        *var_guard463_slot = var_guard463;
        *var_guard464_slot = var_guard464;
        *var_guard465_slot = var_guard465;
        *var_guard466_slot = var_guard466;
        *var_guard467_slot = var_guard467;
        *var_guard468_slot = var_guard468;
        *var_guard469_slot = var_guard469;
        *var_i4_slot = var_i4;
        *var_i4_dn6_slot = var_i4_dn6;
        *var_i4_dn7_slot = var_i4_dn7;
        *var_i4_dn8_slot = var_i4_dn8;
        *var_i4_dn9_slot = var_i4_dn9;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_idmult_slot = var_idmult;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_ijungat_dn9_slot = var_ijungat_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_two_psistar_slot = var_two_psistar;
        *var_vbbt_slot = var_vbbt;
        *var_vjlim_slot = var_vjlim;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_46(
        p: &Parameters,
        var_absource_i: f64,
        var_atatbot: f64,
        var_btatpartbot: f64,
        var_ftdbot: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard465: f64,
        var_idmult: f64,
        var_idsatbot: f64,
        var_one_minus_pbot: f64,
        var_one_over_one_minus_pbot: f64,
        var_perfc: f64,
        var_phitd: f64,
        var_phitr: f64,
        var_two_psistar: f64,
        var_v5: f64,
        var_vbbtlim_s: f64,
        var_vbibot: f64,
        var_vbirbotinv: f64,
        var_vjlim: f64,
        var_wdepnulrbot: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_asrh_dn9_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_btat_dn9_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_guard470_slot: &mut f64,
        var_guard471_slot: &mut f64,
        var_guard472_slot: &mut f64,
        var_guard473_slot: &mut f64,
        var_guard474_slot: &mut f64,
        var_guard475_slot: &mut f64,
        var_guard476_slot: &mut f64,
        var_id__blk212_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijunbot_dn9_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ktat_dn9_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_ltat_dn9_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_mtat_dn9_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_sqrtumax_dn9_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_terfc_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_dn9_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umax_dn9_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxbeforelimiting_dn9_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_dn9_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wdep_dn9_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wgamma_dn9_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_wtat_dn9_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_xerfc_dn9_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
        var_ysq_dn9_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_asrh_dn9: f64 = *var_asrh_dn9_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_btat_dn9: f64 = *var_btat_dn9_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_guard470: f64 = *var_guard470_slot;
        let mut var_guard471: f64 = *var_guard471_slot;
        let mut var_guard472: f64 = *var_guard472_slot;
        let mut var_guard473: f64 = *var_guard473_slot;
        let mut var_guard474: f64 = *var_guard474_slot;
        let mut var_guard475: f64 = *var_guard475_slot;
        let mut var_guard476: f64 = *var_guard476_slot;
        let mut var_id__blk212: f64 = *var_id__blk212_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijunbot_dn9: f64 = *var_ijunbot_dn9_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ktat_dn9: f64 = *var_ktat_dn9_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_ltat_dn9: f64 = *var_ltat_dn9_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_mtat_dn9: f64 = *var_mtat_dn9_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_sqrtumax_dn9: f64 = *var_sqrtumax_dn9_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_terfc_dn9: f64 = *var_terfc_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_twoatatoverthreebtat_dn9: f64 = *var_twoatatoverthreebtat_dn9_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umax_dn9: f64 = *var_umax_dn9_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxbeforelimiting_dn9: f64 = *var_umaxbeforelimiting_dn9_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_umaxpoweronepointfive_dn9: f64 = *var_umaxpoweronepointfive_dn9_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wdep_dn9: f64 = *var_wdep_dn9_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wgamma_dn9: f64 = *var_wgamma_dn9_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_wtat_dn9: f64 = *var_wtat_dn9_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_xerfc_dn9: f64 = *var_xerfc_dn9_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;
        let mut var_ysq_dn9: f64 = *var_ysq_dn9_slot;

        let (assign24280_e28369,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard465 != 0.0)) {
        let assign24280_e28351: f64 = (var_v5 + var_vjlim);
        let assign24280_e28354: f64 = (var_v5 - var_vjlim);
        let assign24280_e28357: f64 = (var_v5 - var_vjlim);
        let assign24280_e28358: f64 = (assign24280_e28354 * assign24280_e28357);
        let assign24280_e28361: f64 = (4.0 * var_phitd);
        let assign24280_e28363: f64 = (assign24280_e28361 * var_phitd);
        let assign24280_e28364: f64 = (assign24280_e28358 + assign24280_e28363);
        let assign24280_e28365: f64 = (assign24280_e28364).sqrt();
        let assign24280_e28366: f64 = (assign24280_e28351 - assign24280_e28365);
        let assign24280_e28367: f64 = (0.5 * assign24280_e28366);
        (assign24280_e28367,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign24280_e28369;

        let (assign24290_e28396,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard465 != 0.0)) {
        let assign24290_e28378: f64 = (var_v5 + var_vbbtlim_s);
        let assign24290_e28381: f64 = (var_v5 - var_vbbtlim_s);
        let assign24290_e28384: f64 = (var_v5 - var_vbbtlim_s);
        let assign24290_e28385: f64 = (assign24290_e28381 * assign24290_e28384);
        let assign24290_e28388: f64 = (4.0 * var_phitr);
        let assign24290_e28390: f64 = (assign24290_e28388 * var_phitr);
        let assign24290_e28391: f64 = (assign24290_e28385 + assign24290_e28390);
        let assign24290_e28392: f64 = (assign24290_e28391).sqrt();
        let assign24290_e28393: f64 = (assign24290_e28378 - assign24290_e28392);
        let assign24290_e28394: f64 = (0.5 * assign24290_e28393);
        (assign24290_e28394,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign24290_e28396;

        let (assign24300_e28423,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard465 != 0.0)) {
        let assign24300_e28405: f64 = var_v5;
        let assign24300_e28408: f64 = var_v5;
        let assign24300_e28411: f64 = var_v5;
        let assign24300_e28412: f64 = (assign24300_e28408 * assign24300_e28411);
        let assign24300_e28415: f64 = (4.0 * 1e-6);
        let assign24300_e28417: f64 = (assign24300_e28415 * 1e-6);
        let assign24300_e28418: f64 = (assign24300_e28412 + assign24300_e28417);
        let assign24300_e28419: f64 = (assign24300_e28418).sqrt();
        let assign24300_e28420: f64 = (assign24300_e28405 - assign24300_e28419);
        let assign24300_e28421: f64 = (0.5 * assign24300_e28420);
        (assign24300_e28421,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign24300_e28423;

        let assign24310_e28426: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard470 = assign24310_e28426;

        let (assign24320_e28434, assign24320_e28434_d_n6, assign24320_e28434_d_n7, assign24320_e28434_d_n8, assign24320_e28434_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign24320_e28434;
        var_ijunbot_dn6 = assign24320_e28434_d_n6;
        var_ijunbot_dn7 = assign24320_e28434_d_n7;
        var_ijunbot_dn8 = assign24320_e28434_d_n8;
        var_ijunbot_dn9 = assign24320_e28434_d_n9;

        let (assign24330_e28445,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) {
        let assign24330_e28443: f64 = (var_idsatbot * var_idmult);
        (assign24330_e28443,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign24330_e28445;

        let assign24340_e28452: f64 = if ((p.p857 == 0.0) && (p.p862 == 0.0)) { 1.0 } else { 0.0 };
        var_guard471 = assign24340_e28452;

        let (assign24350_e28463, assign24350_e28463_d_n6, assign24350_e28463_d_n7, assign24350_e28463_d_n8, assign24350_e28463_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard471 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign24350_e28463;
        var_isrh_dn6 = assign24350_e28463_d_n6;
        var_isrh_dn7 = assign24350_e28463_d_n7;
        var_isrh_dn8 = assign24350_e28463_d_n8;
        var_isrh_dn9 = assign24350_e28463_d_n9;

        let (assign24360_e28477,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard471 == 0.0)) {
        let assign24360_e28475: f64 = (var_vbibot - var_vjsrh);
        (assign24360_e28475,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign24360_e28477;

        let (assign24370_e28496,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard471 == 0.0)) {
        let assign24370_e28491: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign24370_e28492: f64 = (1.0 - assign24370_e28491);
        let assign24370_e28493: f64 = (assign24370_e28492).sqrt();
        let assign24370_e28494: f64 = (1.0 - assign24370_e28493);
        (assign24370_e28494,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign24370_e28496;

        let assign24380_e28499: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };
        var_guard472 = assign24380_e28499;

        let (assign24390_e28513,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard471 == 0.0)) && (var_guard472 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign24390_e28513;

        let (assign24400_e28545,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard471 == 0.0)) && (var_guard472 == 0.0)) {
        let assign24400_e28528: f64 = (var_wsrhstep * var_wsrhstep);
        let assign24400_e28530: f64 = (var_wsrhstep).ln();
        let assign24400_e28531: f64 = (assign24400_e28528 * assign24400_e28530);
        let assign24400_e28534: f64 = (1.0 - var_wsrhstep);
        let assign24400_e28535: f64 = (assign24400_e28531 / assign24400_e28534);
        let assign24400_e28537: f64 = (assign24400_e28535 + var_wsrhstep);
        let assign24400_e28541: f64 = (2.0 * p.p848);
        let assign24400_e28542: f64 = (1.0 - assign24400_e28541);
        let assign24400_e28543: f64 = (assign24400_e28537 * assign24400_e28542);
        (assign24400_e28543,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign24400_e28545;

        let (assign24410_e28559,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard471 == 0.0)) {
        let assign24410_e28557: f64 = (var_wsrhstep + var_dwsrh);
        (assign24410_e28557,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign24410_e28559;

        let assign24420_e28562: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };
        var_guard473 = assign24420_e28562;

        let (assign24430_e28579, assign24430_e28579_d_n6, assign24430_e28579_d_n7, assign24430_e28579_d_n8, assign24430_e28579_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard471 == 0.0)) && (var_guard473 != 0.0)) {
        let assign24430_e28576: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign24430_e28577: f64 = (assign24430_e28576).sqrt();
        (assign24430_e28577, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign24430_e28579;
        var_tmp_dn6 = assign24430_e28579_d_n6;
        var_tmp_dn7 = assign24430_e28579_d_n7;
        var_tmp_dn8 = assign24430_e28579_d_n8;
        var_tmp_dn9 = assign24430_e28579_d_n9;

        let (assign24440_e28598, assign24440_e28598_d_n6, assign24440_e28598_d_n7, assign24440_e28598_d_n8, assign24440_e28598_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard471 == 0.0)) && (var_guard473 == 0.0)) {
        let assign24440_e28594: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign24440_e28596: f64 = (assign24440_e28594).powf(p.p848);
        (assign24440_e28596, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign24440_e28598;
        var_tmp_dn6 = assign24440_e28598_d_n6;
        var_tmp_dn7 = assign24440_e28598_d_n7;
        var_tmp_dn8 = assign24440_e28598_d_n8;
        var_tmp_dn9 = assign24440_e28598_d_n9;

        let (assign24450_e28612, assign24450_e28612_d_n6, assign24450_e28612_d_n7, assign24450_e28612_d_n8, assign24450_e28612_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard471 == 0.0)) {
        let assign24450_e28610: f64 = (var_wdepnulrbot * var_tmp);
        (assign24450_e28610, (var_wdepnulrbot * var_tmp_dn6), (var_wdepnulrbot * var_tmp_dn7), (var_wdepnulrbot * var_tmp_dn8), (var_wdepnulrbot * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign24450_e28612;
        var_wdep_dn6 = assign24450_e28612_d_n6;
        var_wdep_dn7 = assign24450_e28612_d_n7;
        var_wdep_dn8 = assign24450_e28612_d_n8;
        var_wdep_dn9 = assign24450_e28612_d_n9;

        let (assign24460_e28630, assign24460_e28630_d_n6, assign24460_e28630_d_n7, assign24460_e28630_d_n8, assign24460_e28630_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard471 == 0.0)) {
        let assign24460_e28625: f64 = (var_zinv - 1.0);
        let assign24460_e28627: f64 = (assign24460_e28625 * var_wdep);
        let assign24460_e28628: f64 = (var_ftdbot * assign24460_e28627);
        (assign24460_e28628, (var_ftdbot * (assign24460_e28625 * var_wdep_dn6)), (var_ftdbot * (assign24460_e28625 * var_wdep_dn7)), (var_ftdbot * (assign24460_e28625 * var_wdep_dn8)), (var_ftdbot * (assign24460_e28625 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign24460_e28630;
        var_asrh_dn6 = assign24460_e28630_d_n6;
        var_asrh_dn7 = assign24460_e28630_d_n7;
        var_asrh_dn8 = assign24460_e28630_d_n8;
        var_asrh_dn9 = assign24460_e28630_d_n9;

        let (assign24470_e28646, assign24470_e28646_d_n6, assign24470_e28646_d_n7, assign24470_e28646_d_n8, assign24470_e28646_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard471 == 0.0)) {
        let assign24470_e28643: f64 = (var_asrh * var_wsrh);
        let assign24470_e28644: f64 = (p.p857 * assign24470_e28643);
        (assign24470_e28644, (p.p857 * (var_asrh_dn6 * var_wsrh)), (p.p857 * (var_asrh_dn7 * var_wsrh)), (p.p857 * (var_asrh_dn8 * var_wsrh)), (p.p857 * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign24470_e28646;
        var_isrh_dn6 = assign24470_e28646_d_n6;
        var_isrh_dn7 = assign24470_e28646_d_n7;
        var_isrh_dn8 = assign24470_e28646_d_n8;
        var_isrh_dn9 = assign24470_e28646_d_n9;

        let assign24480_e28649: f64 = if p.p862 == 0.0 { 1.0 } else { 0.0 };
        var_guard474 = assign24480_e28649;

        let (assign24490_e28660, assign24490_e28660_d_n6, assign24490_e28660_d_n7, assign24490_e28660_d_n8, assign24490_e28660_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign24490_e28660;
        var_itat_dn6 = assign24490_e28660_d_n6;
        var_itat_dn7 = assign24490_e28660_d_n7;
        var_itat_dn8 = assign24490_e28660_d_n8;
        var_itat_dn9 = assign24490_e28660_d_n9;

        let (assign24500_e28678, assign24500_e28678_d_n6, assign24500_e28678_d_n7, assign24500_e28678_d_n8, assign24500_e28678_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) {
        let assign24500_e28673: f64 = (var_wdep * var_one_minus_pbot);
        let assign24500_e28675: f64 = (assign24500_e28673 / var_vbi_minus_vjsrh);
        let assign24500_e28676: f64 = (var_btatpartbot * assign24500_e28675);
        (assign24500_e28676, (var_btatpartbot * ((var_wdep_dn6 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn7 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn8 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn9 * var_one_minus_pbot) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign24500_e28678;
        var_btat_dn6 = assign24500_e28678_d_n6;
        var_btat_dn7 = assign24500_e28678_d_n7;
        var_btat_dn8 = assign24500_e28678_d_n8;
        var_btat_dn9 = assign24500_e28678_d_n9;

        let (assign24510_e28694, assign24510_e28694_d_n6, assign24510_e28694_d_n7, assign24510_e28694_d_n8, assign24510_e28694_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) {
        let assign24510_e28690: f64 = (0.666666666666667 * var_atatbot);
        let assign24510_e28692: f64 = (assign24510_e28690 / var_btat);
        (assign24510_e28692, (-((assign24510_e28690 * var_btat_dn6) / (var_btat * var_btat))), (-((assign24510_e28690 * var_btat_dn7) / (var_btat * var_btat))), (-((assign24510_e28690 * var_btat_dn8) / (var_btat * var_btat))), (-((assign24510_e28690 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign24510_e28694;
        var_twoatatoverthreebtat_dn6 = assign24510_e28694_d_n6;
        var_twoatatoverthreebtat_dn7 = assign24510_e28694_d_n7;
        var_twoatatoverthreebtat_dn8 = assign24510_e28694_d_n8;
        var_twoatatoverthreebtat_dn9 = assign24510_e28694_d_n9;

        let (assign24520_e28708, assign24520_e28708_d_n6, assign24520_e28708_d_n7, assign24520_e28708_d_n8, assign24520_e28708_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) {
        let assign24520_e28706: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign24520_e28706, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign24520_e28708;
        var_umaxbeforelimiting_dn6 = assign24520_e28708_d_n6;
        var_umaxbeforelimiting_dn7 = assign24520_e28708_d_n7;
        var_umaxbeforelimiting_dn8 = assign24520_e28708_d_n8;
        var_umaxbeforelimiting_dn9 = assign24520_e28708_d_n9;

        let (assign24530_e28729, assign24530_e28729_d_n6, assign24530_e28729_d_n7, assign24530_e28729_d_n8, assign24530_e28729_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) {
        let assign24530_e28720: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign24530_e28723: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign24530_e28725: f64 = (assign24530_e28723 + 1.0);
        let assign24530_e28726: f64 = (assign24530_e28720 / assign24530_e28725);
        let assign24530_e28727: f64 = (assign24530_e28726).sqrt();
        (assign24530_e28727, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign24530_e28725) - (assign24530_e28720 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign24530_e28725 * assign24530_e28725)) / (2.0 * assign24530_e28727)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign24530_e28725) - (assign24530_e28720 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign24530_e28725 * assign24530_e28725)) / (2.0 * assign24530_e28727)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign24530_e28725) - (assign24530_e28720 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign24530_e28725 * assign24530_e28725)) / (2.0 * assign24530_e28727)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign24530_e28725) - (assign24530_e28720 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign24530_e28725 * assign24530_e28725)) / (2.0 * assign24530_e28727)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign24530_e28729;
        var_umax_dn6 = assign24530_e28729_d_n6;
        var_umax_dn7 = assign24530_e28729_d_n7;
        var_umax_dn8 = assign24530_e28729_d_n8;
        var_umax_dn9 = assign24530_e28729_d_n9;

        let (assign24540_e28742, assign24540_e28742_d_n6, assign24540_e28742_d_n7, assign24540_e28742_d_n8, assign24540_e28742_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) {
        let assign24540_e28740: f64 = (var_umax).sqrt();
        (assign24540_e28740, (var_umax_dn6 / (2.0 * assign24540_e28740)), (var_umax_dn7 / (2.0 * assign24540_e28740)), (var_umax_dn8 / (2.0 * assign24540_e28740)), (var_umax_dn9 / (2.0 * assign24540_e28740)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign24540_e28742;
        var_sqrtumax_dn6 = assign24540_e28742_d_n6;
        var_sqrtumax_dn7 = assign24540_e28742_d_n7;
        var_sqrtumax_dn8 = assign24540_e28742_d_n8;
        var_sqrtumax_dn9 = assign24540_e28742_d_n9;

        let (assign24550_e28756, assign24550_e28756_d_n6, assign24550_e28756_d_n7, assign24550_e28756_d_n8, assign24550_e28756_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) {
        let assign24550_e28754: f64 = (var_umax * var_sqrtumax);
        (assign24550_e28754, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign24550_e28756;
        var_umaxpoweronepointfive_dn6 = assign24550_e28756_d_n6;
        var_umaxpoweronepointfive_dn7 = assign24550_e28756_d_n7;
        var_umaxpoweronepointfive_dn8 = assign24550_e28756_d_n8;
        var_umaxpoweronepointfive_dn9 = assign24550_e28756_d_n9;

        let assign24560_e28758: f64 = (-p.p848);
        let assign24560_e28760: f64 = (assign24560_e28758 * var_one_over_one_minus_pbot);
        let assign24560_e28762: f64 = (-1.0);
        let assign24560_e28763: f64 = if assign24560_e28760 == assign24560_e28762 { 1.0 } else { 0.0 };
        var_guard475 = assign24560_e28763;

        let (assign24570_e28783, assign24570_e28783_d_n6, assign24570_e28783_d_n7, assign24570_e28783_d_n8, assign24570_e28783_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) && (var_guard475 != 0.0)) {
        let assign24570_e28779: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign24570_e28780: f64 = (1.0 + assign24570_e28779);
        let assign24570_e28781: f64 = (1.0 / assign24570_e28780);
        (assign24570_e28781, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign24570_e28780 * assign24570_e28780))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign24570_e28780 * assign24570_e28780))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign24570_e28780 * assign24570_e28780))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign24570_e28780 * assign24570_e28780))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign24570_e28783;
        var_wgamma_dn6 = assign24570_e28783_d_n6;
        var_wgamma_dn7 = assign24570_e28783_d_n7;
        var_wgamma_dn8 = assign24570_e28783_d_n8;
        var_wgamma_dn9 = assign24570_e28783_d_n9;

        let (assign24580_e28807, assign24580_e28807_d_n6, assign24580_e28807_d_n7, assign24580_e28807_d_n8, assign24580_e28807_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) && (var_guard475 == 0.0)) {
        let assign24580_e28799: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign24580_e28800: f64 = (1.0 + assign24580_e28799);
        let assign24580_e28802: f64 = (-p.p848);
        let assign24580_e28804: f64 = (assign24580_e28802 * var_one_over_one_minus_pbot);
        let assign24580_e28805: f64 = (assign24580_e28800).powf(assign24580_e28804);
        (assign24580_e28805, if 0.0 == 0.0 && ((assign24580_e28804) as f64).is_finite() && ((assign24580_e28804) as f64).fract() == 0.0 { if assign24580_e28804 == 0.0 { 0.0 } else { (assign24580_e28804 * ((assign24580_e28800).powf(assign24580_e28804 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign24580_e28805 * (assign24580_e28804 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign24580_e28800))) }, if 0.0 == 0.0 && ((assign24580_e28804) as f64).is_finite() && ((assign24580_e28804) as f64).fract() == 0.0 { if assign24580_e28804 == 0.0 { 0.0 } else { (assign24580_e28804 * ((assign24580_e28800).powf(assign24580_e28804 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign24580_e28805 * (assign24580_e28804 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign24580_e28800))) }, if 0.0 == 0.0 && ((assign24580_e28804) as f64).is_finite() && ((assign24580_e28804) as f64).fract() == 0.0 { if assign24580_e28804 == 0.0 { 0.0 } else { (assign24580_e28804 * ((assign24580_e28800).powf(assign24580_e28804 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign24580_e28805 * (assign24580_e28804 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign24580_e28800))) }, if 0.0 == 0.0 && ((assign24580_e28804) as f64).is_finite() && ((assign24580_e28804) as f64).fract() == 0.0 { if assign24580_e28804 == 0.0 { 0.0 } else { (assign24580_e28804 * ((assign24580_e28800).powf(assign24580_e28804 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign24580_e28805 * (assign24580_e28804 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign24580_e28800))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign24580_e28807;
        var_wgamma_dn6 = assign24580_e28807_d_n6;
        var_wgamma_dn7 = assign24580_e28807_d_n7;
        var_wgamma_dn8 = assign24580_e28807_d_n8;
        var_wgamma_dn9 = assign24580_e28807_d_n9;

        let (assign24590_e28825, assign24590_e28825_d_n6, assign24590_e28825_d_n7, assign24590_e28825_d_n8, assign24590_e28825_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) {
        let assign24590_e28819: f64 = (var_wsrh * var_wgamma);
        let assign24590_e28822: f64 = (var_wsrh + var_wgamma);
        let assign24590_e28823: f64 = (assign24590_e28819 / assign24590_e28822);
        (assign24590_e28823, ((((var_wsrh * var_wgamma_dn6) * assign24590_e28822) - (assign24590_e28819 * var_wgamma_dn6)) / (assign24590_e28822 * assign24590_e28822)), ((((var_wsrh * var_wgamma_dn7) * assign24590_e28822) - (assign24590_e28819 * var_wgamma_dn7)) / (assign24590_e28822 * assign24590_e28822)), ((((var_wsrh * var_wgamma_dn8) * assign24590_e28822) - (assign24590_e28819 * var_wgamma_dn8)) / (assign24590_e28822 * assign24590_e28822)), ((((var_wsrh * var_wgamma_dn9) * assign24590_e28822) - (assign24590_e28819 * var_wgamma_dn9)) / (assign24590_e28822 * assign24590_e28822)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign24590_e28825;
        var_wtat_dn6 = assign24590_e28825_d_n6;
        var_wtat_dn7 = assign24590_e28825_d_n7;
        var_wtat_dn8 = assign24590_e28825_d_n8;
        var_wtat_dn9 = assign24590_e28825_d_n9;

        let (assign24600_e28842, assign24600_e28842_d_n6, assign24600_e28842_d_n7, assign24600_e28842_d_n8, assign24600_e28842_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) {
        let assign24600_e28838: f64 = (var_btat / var_sqrtumax);
        let assign24600_e28839: f64 = (0.375 * assign24600_e28838);
        let assign24600_e28840: f64 = (assign24600_e28839).sqrt();
        (assign24600_e28840, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign24600_e28840)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign24600_e28840)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign24600_e28840)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign24600_e28840)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign24600_e28842;
        var_ktat_dn6 = assign24600_e28842_d_n6;
        var_ktat_dn7 = assign24600_e28842_d_n7;
        var_ktat_dn8 = assign24600_e28842_d_n8;
        var_ktat_dn9 = assign24600_e28842_d_n9;

        let (assign24610_e28860, assign24610_e28860_d_n6, assign24610_e28860_d_n7, assign24610_e28860_d_n8, assign24610_e28860_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) {
        let assign24610_e28855: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign24610_e28856: f64 = (2.0 * assign24610_e28855);
        let assign24610_e28858: f64 = (assign24610_e28856 - var_umax);
        (assign24610_e28858, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign24610_e28860;
        var_ltat_dn6 = assign24610_e28860_d_n6;
        var_ltat_dn7 = assign24610_e28860_d_n7;
        var_ltat_dn8 = assign24610_e28860_d_n8;
        var_ltat_dn9 = assign24610_e28860_d_n9;

        let (assign24620_e28886, assign24620_e28886_d_n6, assign24620_e28886_d_n7, assign24620_e28886_d_n8, assign24620_e28886_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) {
        let assign24620_e28872: f64 = (var_atatbot * var_twoatatoverthreebtat);
        let assign24620_e28874: f64 = (assign24620_e28872 * var_sqrtumax);
        let assign24620_e28877: f64 = (var_atatbot * var_umax);
        let assign24620_e28878: f64 = (assign24620_e28874 - assign24620_e28877);
        let assign24620_e28882: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign24620_e28883: f64 = (0.5 * assign24620_e28882);
        let assign24620_e28884: f64 = (assign24620_e28878 + assign24620_e28883);
        (assign24620_e28884, (((((var_atatbot * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign24620_e28872 * var_sqrtumax_dn6)) - (var_atatbot * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign24620_e28872 * var_sqrtumax_dn7)) - (var_atatbot * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign24620_e28872 * var_sqrtumax_dn8)) - (var_atatbot * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatbot * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign24620_e28872 * var_sqrtumax_dn9)) - (var_atatbot * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign24620_e28886;
        var_mtat_dn6 = assign24620_e28886_d_n6;
        var_mtat_dn7 = assign24620_e28886_d_n7;
        var_mtat_dn8 = assign24620_e28886_d_n8;
        var_mtat_dn9 = assign24620_e28886_d_n9;

        let (assign24630_e28902, assign24630_e28902_d_n6, assign24630_e28902_d_n7, assign24630_e28902_d_n8, assign24630_e28902_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) {
        let assign24630_e28898: f64 = (var_ltat - 1.0);
        let assign24630_e28900: f64 = (assign24630_e28898 * var_ktat);
        (assign24630_e28900, ((var_ltat_dn6 * var_ktat) + (assign24630_e28898 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign24630_e28898 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign24630_e28898 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign24630_e28898 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign24630_e28902;
        var_xerfc_dn6 = assign24630_e28902_d_n6;
        var_xerfc_dn7 = assign24630_e28902_d_n7;
        var_xerfc_dn8 = assign24630_e28902_d_n8;
        var_xerfc_dn9 = assign24630_e28902_d_n9;

        let (assign24640_e28916, assign24640_e28916_d_n6, assign24640_e28916_d_n7, assign24640_e28916_d_n8, assign24640_e28916_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) {
        let assign24640_e28914: f64 = (var_xerfc * var_xerfc);
        (assign24640_e28914, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign24640_e28916;
        var_ysq_dn6 = assign24640_e28916_d_n6;
        var_ysq_dn7 = assign24640_e28916_d_n7;
        var_ysq_dn8 = assign24640_e28916_d_n8;
        var_ysq_dn9 = assign24640_e28916_d_n9;

        let assign24650_e28919: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard476 = assign24650_e28919;

        let (assign24660_e28939, assign24660_e28939_d_n6, assign24660_e28939_d_n7, assign24660_e28939_d_n8, assign24660_e28939_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) && (var_guard476 != 0.0)) {
        let assign24660_e28935: f64 = (var_perfc * var_xerfc);
        let assign24660_e28936: f64 = (1.0 + assign24660_e28935);
        let assign24660_e28937: f64 = (1.0 / assign24660_e28936);
        (assign24660_e28937, (-((var_perfc * var_xerfc_dn6) / (assign24660_e28936 * assign24660_e28936))), (-((var_perfc * var_xerfc_dn7) / (assign24660_e28936 * assign24660_e28936))), (-((var_perfc * var_xerfc_dn8) / (assign24660_e28936 * assign24660_e28936))), (-((var_perfc * var_xerfc_dn9) / (assign24660_e28936 * assign24660_e28936))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign24660_e28939;
        var_terfc_dn6 = assign24660_e28939_d_n6;
        var_terfc_dn7 = assign24660_e28939_d_n7;
        var_terfc_dn8 = assign24660_e28939_d_n8;
        var_terfc_dn9 = assign24660_e28939_d_n9;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_asrh_dn9_slot = var_asrh_dn9;
        *var_btat_slot = var_btat;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_btat_dn9_slot = var_btat_dn9;
        *var_dwsrh_slot = var_dwsrh;
        *var_guard470_slot = var_guard470;
        *var_guard471_slot = var_guard471;
        *var_guard472_slot = var_guard472;
        *var_guard473_slot = var_guard473;
        *var_guard474_slot = var_guard474;
        *var_guard475_slot = var_guard475;
        *var_guard476_slot = var_guard476;
        *var_id__blk212_slot = var_id__blk212;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijunbot_dn9_slot = var_ijunbot_dn9;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ktat_dn9_slot = var_ktat_dn9;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_ltat_dn9_slot = var_ltat_dn9;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_mtat_dn9_slot = var_mtat_dn9;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_sqrtumax_dn9_slot = var_sqrtumax_dn9;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_terfc_dn9_slot = var_terfc_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_twoatatoverthreebtat_dn9_slot = var_twoatatoverthreebtat_dn9;
        *var_umax_slot = var_umax;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umax_dn9_slot = var_umax_dn9;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxbeforelimiting_dn9_slot = var_umaxbeforelimiting_dn9;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_umaxpoweronepointfive_dn9_slot = var_umaxpoweronepointfive_dn9;
        *var_vav_slot = var_vav;
        *var_vbbt_slot = var_vbbt;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_vjsrh_slot = var_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wdep_dn9_slot = var_wdep_dn9;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wgamma_dn9_slot = var_wgamma_dn9;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_wtat_dn9_slot = var_wtat_dn9;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_xerfc_dn9_slot = var_xerfc_dn9;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
        *var_ysq_dn9_slot = var_ysq_dn9;
    }

    pub(super) fn stamp_transient_block_47(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatbot: f64,
        var_berfc: f64,
        var_cerfc: f64,
        var_fbbtbot: f64,
        var_fstopbot: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard470: f64,
        var_guard474: f64,
        var_guard476: f64,
        var_id__blk212: f64,
        var_isrh: f64,
        var_isrh_dn6: f64,
        var_isrh_dn7: f64,
        var_isrh_dn8: f64,
        var_isrh_dn9: f64,
        var_ktat: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_ktat_dn9: f64,
        var_lssource_i: f64,
        var_mtat: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_mtat_dn9: f64,
        var_one_over_one_minus_pbot: f64,
        var_perfc: f64,
        var_slopebot: f64,
        var_v5: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirbotinv: f64,
        var_vbrinvbot: f64,
        var_wdepnulrinvbot: f64,
        var_wtat: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_wtat_dn9: f64,
        var_xerfc: f64,
        var_xerfc_dn6: f64,
        var_xerfc_dn7: f64,
        var_xerfc_dn8: f64,
        var_xerfc_dn9: f64,
        var_ysq: f64,
        var_ysq_dn6: f64,
        var_ysq_dn7: f64,
        var_ysq_dn8: f64,
        var_ysq_dn9: f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfcpos_dn9_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_erfctimesexpmtat_dn9_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_fmaxr_dn9_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_gammamax_dn9_slot: &mut f64,
        var_guard477_slot: &mut f64,
        var_guard478_slot: &mut f64,
        var_guard479_slot: &mut f64,
        var_guard480_slot: &mut f64,
        var_guard481_slot: &mut f64,
        var_guard482_slot: &mut f64,
        var_guard483_slot: &mut f64,
        var_guard484_slot: &mut f64,
        var_guard485_slot: &mut f64,
        var_guard486_slot: &mut f64,
        var_guard487_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijunbot_dn9_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_ijunsti_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_terfc_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
    ) {
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfcpos_dn9: f64 = *var_erfcpos_dn9_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_erfctimesexpmtat_dn9: f64 = *var_erfctimesexpmtat_dn9_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fbreakdown_dn9: f64 = *var_fbreakdown_dn9_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_fmaxr_dn9: f64 = *var_fmaxr_dn9_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_gammamax_dn9: f64 = *var_gammamax_dn9_slot;
        let mut var_guard477: f64 = *var_guard477_slot;
        let mut var_guard478: f64 = *var_guard478_slot;
        let mut var_guard479: f64 = *var_guard479_slot;
        let mut var_guard480: f64 = *var_guard480_slot;
        let mut var_guard481: f64 = *var_guard481_slot;
        let mut var_guard482: f64 = *var_guard482_slot;
        let mut var_guard483: f64 = *var_guard483_slot;
        let mut var_guard484: f64 = *var_guard484_slot;
        let mut var_guard485: f64 = *var_guard485_slot;
        let mut var_guard486: f64 = *var_guard486_slot;
        let mut var_guard487: f64 = *var_guard487_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijunbot_dn9: f64 = *var_ijunbot_dn9_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_ijunsti_dn9: f64 = *var_ijunsti_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_terfc_dn9: f64 = *var_terfc_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;

        let (assign24670_e28960, assign24670_e28960_d_n6, assign24670_e28960_d_n7, assign24670_e28960_d_n8, assign24670_e28960_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) && (var_guard476 == 0.0)) {
        let assign24670_e28956: f64 = (var_perfc * var_xerfc);
        let assign24670_e28957: f64 = (1.0 - assign24670_e28956);
        let assign24670_e28958: f64 = (1.0 / assign24670_e28957);
        (assign24670_e28958, (-((-(var_perfc * var_xerfc_dn6)) / (assign24670_e28957 * assign24670_e28957))), (-((-(var_perfc * var_xerfc_dn7)) / (assign24670_e28957 * assign24670_e28957))), (-((-(var_perfc * var_xerfc_dn8)) / (assign24670_e28957 * assign24670_e28957))), (-((-(var_perfc * var_xerfc_dn9)) / (assign24670_e28957 * assign24670_e28957))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign24670_e28960;
        var_terfc_dn6 = assign24670_e28960_d_n6;
        var_terfc_dn7 = assign24670_e28960_d_n7;
        var_terfc_dn8 = assign24670_e28960_d_n8;
        var_terfc_dn9 = assign24670_e28960_d_n9;

        let assign24680_e28962: f64 = (-var_ysq);
        let assign24680_e28964: f64 = (assign24680_e28962 + var_mtat);
        let assign24680_e28966: f64 = (-230.25850929940458);
        let assign24680_e28967: f64 = if assign24680_e28964 > assign24680_e28966 { 1.0 } else { 0.0 };
        var_guard477 = assign24680_e28967;

        let (assign24690_e28985, assign24690_e28985_d_n6, assign24690_e28985_d_n7, assign24690_e28985_d_n8, assign24690_e28985_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) && (var_guard477 != 0.0)) {
        let assign24690_e28980: f64 = (-var_ysq);
        let assign24690_e28982: f64 = (assign24690_e28980 + var_mtat);
        let assign24690_e28983: f64 = (assign24690_e28982).exp();
        (assign24690_e28983, (assign24690_e28983 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign24690_e28983 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign24690_e28983 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign24690_e28983 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign24690_e28985;
        var_tmp_dn6 = assign24690_e28985_d_n6;
        var_tmp_dn7 = assign24690_e28985_d_n7;
        var_tmp_dn8 = assign24690_e28985_d_n8;
        var_tmp_dn9 = assign24690_e28985_d_n9;

        let (assign24700_e29034, assign24700_e29034_d_n6, assign24700_e29034_d_n7, assign24700_e29034_d_n8, assign24700_e29034_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) && (var_guard477 == 0.0)) {
        let assign24700_e29001: f64 = (-230.25850929940458);
        let assign24700_e29003: f64 = (-var_ysq);
        let assign24700_e29005: f64 = (assign24700_e29003 + var_mtat);
        let assign24700_e29006: f64 = (assign24700_e29001 - assign24700_e29005);
        let assign24700_e29010: f64 = (-230.25850929940458);
        let assign24700_e29012: f64 = (-var_ysq);
        let assign24700_e29014: f64 = (assign24700_e29012 + var_mtat);
        let assign24700_e29015: f64 = (assign24700_e29010 - assign24700_e29014);
        let assign24700_e29018: f64 = (-230.25850929940458);
        let assign24700_e29020: f64 = (-var_ysq);
        let assign24700_e29022: f64 = (assign24700_e29020 + var_mtat);
        let assign24700_e29023: f64 = (assign24700_e29018 - assign24700_e29022);
        let assign24700_e29025: f64 = (assign24700_e29023 * 0.3333333333333333);
        let assign24700_e29026: f64 = (1.0 + assign24700_e29025);
        let assign24700_e29027: f64 = (assign24700_e29015 * assign24700_e29026);
        let assign24700_e29028: f64 = (0.5 * assign24700_e29027);
        let assign24700_e29029: f64 = (1.0 + assign24700_e29028);
        let assign24700_e29030: f64 = (assign24700_e29006 * assign24700_e29029);
        let assign24700_e29031: f64 = (1.0 + assign24700_e29030);
        let assign24700_e29032: f64 = (1e-100 / assign24700_e29031);
        (assign24700_e29032, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign24700_e29029) + (assign24700_e29006 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign24700_e29026) + (assign24700_e29015 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign24700_e29031 * assign24700_e29031))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign24700_e29029) + (assign24700_e29006 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign24700_e29026) + (assign24700_e29015 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign24700_e29031 * assign24700_e29031))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign24700_e29029) + (assign24700_e29006 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign24700_e29026) + (assign24700_e29015 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign24700_e29031 * assign24700_e29031))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign24700_e29029) + (assign24700_e29006 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign24700_e29026) + (assign24700_e29015 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign24700_e29031 * assign24700_e29031))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign24700_e29034;
        var_tmp_dn6 = assign24700_e29034_d_n6;
        var_tmp_dn7 = assign24700_e29034_d_n7;
        var_tmp_dn8 = assign24700_e29034_d_n8;
        var_tmp_dn9 = assign24700_e29034_d_n9;

        let (assign24710_e29064, assign24710_e29064_d_n6, assign24710_e29064_d_n7, assign24710_e29064_d_n8, assign24710_e29064_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) {
        let assign24710_e29046: f64 = (0.29214664 * var_terfc);
        let assign24710_e29050: f64 = (var_terfc * var_terfc);
        let assign24710_e29051: f64 = (var_berfc * assign24710_e29050);
        let assign24710_e29052: f64 = (assign24710_e29046 + assign24710_e29051);
        let assign24710_e29056: f64 = (var_terfc * var_terfc);
        let assign24710_e29058: f64 = (assign24710_e29056 * var_terfc);
        let assign24710_e29059: f64 = (var_cerfc * assign24710_e29058);
        let assign24710_e29060: f64 = (assign24710_e29052 + assign24710_e29059);
        let assign24710_e29062: f64 = (assign24710_e29060 * var_tmp);
        (assign24710_e29062, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign24710_e29056 * var_terfc_dn6)))) * var_tmp) + (assign24710_e29060 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign24710_e29056 * var_terfc_dn7)))) * var_tmp) + (assign24710_e29060 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign24710_e29056 * var_terfc_dn8)))) * var_tmp) + (assign24710_e29060 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign24710_e29056 * var_terfc_dn9)))) * var_tmp) + (assign24710_e29060 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign24710_e29064;
        var_erfcpos_dn6 = assign24710_e29064_d_n6;
        var_erfcpos_dn7 = assign24710_e29064_d_n7;
        var_erfcpos_dn8 = assign24710_e29064_d_n8;
        var_erfcpos_dn9 = assign24710_e29064_d_n9;

        let assign24720_e29067: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard478 = assign24720_e29067;

        let (assign24730_e29081, assign24730_e29081_d_n6, assign24730_e29081_d_n7, assign24730_e29081_d_n8, assign24730_e29081_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) && (var_guard478 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign24730_e29081;
        var_erfctimesexpmtat_dn6 = assign24730_e29081_d_n6;
        var_erfctimesexpmtat_dn7 = assign24730_e29081_d_n7;
        var_erfctimesexpmtat_dn8 = assign24730_e29081_d_n8;
        var_erfctimesexpmtat_dn9 = assign24730_e29081_d_n9;

        let assign24740_e29084: f64 = (-230.25850929940458);
        let assign24740_e29085: f64 = if var_mtat > assign24740_e29084 { 1.0 } else { 0.0 };
        var_guard479 = assign24740_e29085;

        let (assign24750_e29103, assign24750_e29103_d_n6, assign24750_e29103_d_n7, assign24750_e29103_d_n8, assign24750_e29103_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) && (var_guard478 == 0.0)) && (var_guard479 != 0.0)) {
        let assign24750_e29101: f64 = (var_mtat).exp();
        (assign24750_e29101, (assign24750_e29101 * var_mtat_dn6), (assign24750_e29101 * var_mtat_dn7), (assign24750_e29101 * var_mtat_dn8), (assign24750_e29101 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign24750_e29103;
        var_tmp_dn6 = assign24750_e29103_d_n6;
        var_tmp_dn7 = assign24750_e29103_d_n7;
        var_tmp_dn8 = assign24750_e29103_d_n8;
        var_tmp_dn9 = assign24750_e29103_d_n9;

        let (assign24760_e29146, assign24760_e29146_d_n6, assign24760_e29146_d_n7, assign24760_e29146_d_n8, assign24760_e29146_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) && (var_guard478 == 0.0)) && (var_guard479 == 0.0)) {
        let assign24760_e29122: f64 = (-230.25850929940458);
        let assign24760_e29124: f64 = (assign24760_e29122 - var_mtat);
        let assign24760_e29128: f64 = (-230.25850929940458);
        let assign24760_e29130: f64 = (assign24760_e29128 - var_mtat);
        let assign24760_e29133: f64 = (-230.25850929940458);
        let assign24760_e29135: f64 = (assign24760_e29133 - var_mtat);
        let assign24760_e29137: f64 = (assign24760_e29135 * 0.3333333333333333);
        let assign24760_e29138: f64 = (1.0 + assign24760_e29137);
        let assign24760_e29139: f64 = (assign24760_e29130 * assign24760_e29138);
        let assign24760_e29140: f64 = (0.5 * assign24760_e29139);
        let assign24760_e29141: f64 = (1.0 + assign24760_e29140);
        let assign24760_e29142: f64 = (assign24760_e29124 * assign24760_e29141);
        let assign24760_e29143: f64 = (1.0 + assign24760_e29142);
        let assign24760_e29144: f64 = (1e-100 / assign24760_e29143);
        (assign24760_e29144, (-((1e-100 * (((-var_mtat_dn6) * assign24760_e29141) + (assign24760_e29124 * (0.5 * (((-var_mtat_dn6) * assign24760_e29138) + (assign24760_e29130 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign24760_e29143 * assign24760_e29143))), (-((1e-100 * (((-var_mtat_dn7) * assign24760_e29141) + (assign24760_e29124 * (0.5 * (((-var_mtat_dn7) * assign24760_e29138) + (assign24760_e29130 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign24760_e29143 * assign24760_e29143))), (-((1e-100 * (((-var_mtat_dn8) * assign24760_e29141) + (assign24760_e29124 * (0.5 * (((-var_mtat_dn8) * assign24760_e29138) + (assign24760_e29130 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign24760_e29143 * assign24760_e29143))), (-((1e-100 * (((-var_mtat_dn9) * assign24760_e29141) + (assign24760_e29124 * (0.5 * (((-var_mtat_dn9) * assign24760_e29138) + (assign24760_e29130 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign24760_e29143 * assign24760_e29143))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign24760_e29146;
        var_tmp_dn6 = assign24760_e29146_d_n6;
        var_tmp_dn7 = assign24760_e29146_d_n7;
        var_tmp_dn8 = assign24760_e29146_d_n8;
        var_tmp_dn9 = assign24760_e29146_d_n9;

        let (assign24770_e29165, assign24770_e29165_d_n6, assign24770_e29165_d_n7, assign24770_e29165_d_n8, assign24770_e29165_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) && (var_guard478 == 0.0)) {
        let assign24770_e29161: f64 = (2.0 * var_tmp);
        let assign24770_e29163: f64 = (assign24770_e29161 - var_erfcpos);
        (assign24770_e29163, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign24770_e29165;
        var_erfctimesexpmtat_dn6 = assign24770_e29165_d_n6;
        var_erfctimesexpmtat_dn7 = assign24770_e29165_d_n7;
        var_erfctimesexpmtat_dn8 = assign24770_e29165_d_n8;
        var_erfctimesexpmtat_dn9 = assign24770_e29165_d_n9;

        let (assign24780_e29185, assign24780_e29185_d_n6, assign24780_e29185_d_n7, assign24780_e29185_d_n8, assign24780_e29185_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) {
        let assign24780_e29177: f64 = (1.772453850905516 * 0.5);
        let assign24780_e29180: f64 = (var_atatbot * var_erfctimesexpmtat);
        let assign24780_e29182: f64 = (assign24780_e29180 / var_ktat);
        let assign24780_e29183: f64 = (assign24780_e29177 * assign24780_e29182);
        (assign24780_e29183, (assign24780_e29177 * ((((var_atatbot * var_erfctimesexpmtat_dn6) * var_ktat) - (assign24780_e29180 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign24780_e29177 * ((((var_atatbot * var_erfctimesexpmtat_dn7) * var_ktat) - (assign24780_e29180 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign24780_e29177 * ((((var_atatbot * var_erfctimesexpmtat_dn8) * var_ktat) - (assign24780_e29180 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign24780_e29177 * ((((var_atatbot * var_erfctimesexpmtat_dn9) * var_ktat) - (assign24780_e29180 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign24780_e29185;
        var_gammamax_dn6 = assign24780_e29185_d_n6;
        var_gammamax_dn7 = assign24780_e29185_d_n7;
        var_gammamax_dn8 = assign24780_e29185_d_n8;
        var_gammamax_dn9 = assign24780_e29185_d_n9;

        let (assign24790_e29203, assign24790_e29203_d_n6, assign24790_e29203_d_n7, assign24790_e29203_d_n8, assign24790_e29203_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard474 == 0.0)) {
        let assign24790_e29198: f64 = (var_asrh * var_gammamax);
        let assign24790_e29200: f64 = (assign24790_e29198 * var_wtat);
        let assign24790_e29201: f64 = (p.p862 * assign24790_e29200);
        (assign24790_e29201, (p.p862 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign24790_e29198 * var_wtat_dn6))), (p.p862 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign24790_e29198 * var_wtat_dn7))), (p.p862 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign24790_e29198 * var_wtat_dn8))), (p.p862 * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign24790_e29198 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign24790_e29203;
        var_itat_dn6 = assign24790_e29203_d_n6;
        var_itat_dn7 = assign24790_e29203_d_n7;
        var_itat_dn8 = assign24790_e29203_d_n8;
        var_itat_dn9 = assign24790_e29203_d_n9;

        let assign24800_e29206: f64 = if p.p868 == 0.0 { 1.0 } else { 0.0 };
        var_guard480 = assign24800_e29206;

        let (assign24810_e29217, assign24810_e29217_d_n6, assign24810_e29217_d_n7, assign24810_e29217_d_n8, assign24810_e29217_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard480 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign24810_e29217;
        var_ibbt_dn6 = assign24810_e29217_d_n6;
        var_ibbt_dn7 = assign24810_e29217_d_n7;
        var_ibbt_dn8 = assign24810_e29217_d_n8;
        var_ibbt_dn9 = assign24810_e29217_d_n9;

        let assign24820_e29220: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };
        var_guard481 = assign24820_e29220;

        let (assign24830_e29239, assign24830_e29239_d_n6, assign24830_e29239_d_n7, assign24830_e29239_d_n8, assign24830_e29239_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard480 == 0.0)) && (var_guard481 != 0.0)) {
        let assign24830_e29234: f64 = (p.p845 - var_vbbt);
        let assign24830_e29236: f64 = (assign24830_e29234 * var_vbirbotinv);
        let assign24830_e29237: f64 = (assign24830_e29236).sqrt();
        (assign24830_e29237, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign24830_e29239;
        var_tmp_dn6 = assign24830_e29239_d_n6;
        var_tmp_dn7 = assign24830_e29239_d_n7;
        var_tmp_dn8 = assign24830_e29239_d_n8;
        var_tmp_dn9 = assign24830_e29239_d_n9;

        let (assign24840_e29260, assign24840_e29260_d_n6, assign24840_e29260_d_n7, assign24840_e29260_d_n8, assign24840_e29260_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard480 == 0.0)) && (var_guard481 == 0.0)) {
        let assign24840_e29254: f64 = (p.p845 - var_vbbt);
        let assign24840_e29256: f64 = (assign24840_e29254 * var_vbirbotinv);
        let assign24840_e29258: f64 = (assign24840_e29256).powf(p.p848);
        (assign24840_e29258, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign24840_e29260;
        var_tmp_dn6 = assign24840_e29260_d_n6;
        var_tmp_dn7 = assign24840_e29260_d_n7;
        var_tmp_dn8 = assign24840_e29260_d_n8;
        var_tmp_dn9 = assign24840_e29260_d_n9;

        let (assign24850_e29280, assign24850_e29280_d_n6, assign24850_e29280_d_n7, assign24850_e29280_d_n8, assign24850_e29280_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard480 == 0.0)) {
        let assign24850_e29273: f64 = (p.p845 - var_vbbt);
        let assign24850_e29275: f64 = (assign24850_e29273 * var_wdepnulrinvbot);
        let assign24850_e29277: f64 = (assign24850_e29275 / var_tmp);
        let assign24850_e29278: f64 = (var_one_over_one_minus_pbot * assign24850_e29277);
        (assign24850_e29278, (var_one_over_one_minus_pbot * (-((assign24850_e29275 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign24850_e29275 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign24850_e29275 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign24850_e29275 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign24850_e29280;
        var_fmaxr_dn6 = assign24850_e29280_d_n6;
        var_fmaxr_dn7 = assign24850_e29280_d_n7;
        var_fmaxr_dn8 = assign24850_e29280_d_n8;
        var_fmaxr_dn9 = assign24850_e29280_d_n9;

        let assign24860_e29282: f64 = (-var_fbbtbot);
        let assign24860_e29284: f64 = (assign24860_e29282 / var_fmaxr);
        let assign24860_e29285: f64 = (assign24860_e29284).abs();
        let assign24860_e29287: f64 = if assign24860_e29285 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard482 = assign24860_e29287;

        let (assign24870_e29305, assign24870_e29305_d_n6, assign24870_e29305_d_n7, assign24870_e29305_d_n8, assign24870_e29305_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard480 == 0.0)) && (var_guard482 != 0.0)) {
        let assign24870_e29300: f64 = (-var_fbbtbot);
        let assign24870_e29302: f64 = (assign24870_e29300 / var_fmaxr);
        let assign24870_e29303: f64 = (assign24870_e29302).exp();
        (assign24870_e29303, (assign24870_e29303 * (-((assign24870_e29300 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign24870_e29303 * (-((assign24870_e29300 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign24870_e29303 * (-((assign24870_e29300 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign24870_e29303 * (-((assign24870_e29300 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign24870_e29305;
        var_tmp_dn6 = assign24870_e29305_d_n6;
        var_tmp_dn7 = assign24870_e29305_d_n7;
        var_tmp_dn8 = assign24870_e29305_d_n8;
        var_tmp_dn9 = assign24870_e29305_d_n9;

        let assign24880_e29307: f64 = (-var_fbbtbot);
        let assign24880_e29309: f64 = (assign24880_e29307 / var_fmaxr);
        let assign24880_e29311: f64 = if assign24880_e29309 < 0.0 { 1.0 } else { 0.0 };
        var_guard483 = assign24880_e29311;

        let (assign24890_e29362, assign24890_e29362_d_n6, assign24890_e29362_d_n7, assign24890_e29362_d_n8, assign24890_e29362_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard480 == 0.0)) && (var_guard482 == 0.0)) && (var_guard483 != 0.0)) {
        let assign24890_e29329: f64 = (-230.25850929940458);
        let assign24890_e29331: f64 = (-var_fbbtbot);
        let assign24890_e29333: f64 = (assign24890_e29331 / var_fmaxr);
        let assign24890_e29334: f64 = (assign24890_e29329 - assign24890_e29333);
        let assign24890_e29338: f64 = (-230.25850929940458);
        let assign24890_e29340: f64 = (-var_fbbtbot);
        let assign24890_e29342: f64 = (assign24890_e29340 / var_fmaxr);
        let assign24890_e29343: f64 = (assign24890_e29338 - assign24890_e29342);
        let assign24890_e29346: f64 = (-230.25850929940458);
        let assign24890_e29348: f64 = (-var_fbbtbot);
        let assign24890_e29350: f64 = (assign24890_e29348 / var_fmaxr);
        let assign24890_e29351: f64 = (assign24890_e29346 - assign24890_e29350);
        let assign24890_e29353: f64 = (assign24890_e29351 * 0.3333333333333333);
        let assign24890_e29354: f64 = (1.0 + assign24890_e29353);
        let assign24890_e29355: f64 = (assign24890_e29343 * assign24890_e29354);
        let assign24890_e29356: f64 = (0.5 * assign24890_e29355);
        let assign24890_e29357: f64 = (1.0 + assign24890_e29356);
        let assign24890_e29358: f64 = (assign24890_e29334 * assign24890_e29357);
        let assign24890_e29359: f64 = (1.0 + assign24890_e29358);
        let assign24890_e29360: f64 = (1e-100 / assign24890_e29359);
        (assign24890_e29360, (-((1e-100 * (((-(-((assign24890_e29331 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign24890_e29357) + (assign24890_e29334 * (0.5 * (((-(-((assign24890_e29340 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign24890_e29354) + (assign24890_e29343 * ((-(-((assign24890_e29348 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign24890_e29359 * assign24890_e29359))), (-((1e-100 * (((-(-((assign24890_e29331 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign24890_e29357) + (assign24890_e29334 * (0.5 * (((-(-((assign24890_e29340 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign24890_e29354) + (assign24890_e29343 * ((-(-((assign24890_e29348 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign24890_e29359 * assign24890_e29359))), (-((1e-100 * (((-(-((assign24890_e29331 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign24890_e29357) + (assign24890_e29334 * (0.5 * (((-(-((assign24890_e29340 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign24890_e29354) + (assign24890_e29343 * ((-(-((assign24890_e29348 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign24890_e29359 * assign24890_e29359))), (-((1e-100 * (((-(-((assign24890_e29331 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign24890_e29357) + (assign24890_e29334 * (0.5 * (((-(-((assign24890_e29340 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign24890_e29354) + (assign24890_e29343 * ((-(-((assign24890_e29348 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign24890_e29359 * assign24890_e29359))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign24890_e29362;
        var_tmp_dn6 = assign24890_e29362_d_n6;
        var_tmp_dn7 = assign24890_e29362_d_n7;
        var_tmp_dn8 = assign24890_e29362_d_n8;
        var_tmp_dn9 = assign24890_e29362_d_n9;

        let (assign24900_e29411, assign24900_e29411_d_n6, assign24900_e29411_d_n7, assign24900_e29411_d_n8, assign24900_e29411_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard480 == 0.0)) && (var_guard482 == 0.0)) && (var_guard483 == 0.0)) {
        let assign24900_e29381: f64 = (-var_fbbtbot);
        let assign24900_e29383: f64 = (assign24900_e29381 / var_fmaxr);
        let assign24900_e29385: f64 = (assign24900_e29383 - 230.25850929940458);
        let assign24900_e29389: f64 = (-var_fbbtbot);
        let assign24900_e29391: f64 = (assign24900_e29389 / var_fmaxr);
        let assign24900_e29393: f64 = (assign24900_e29391 - 230.25850929940458);
        let assign24900_e29396: f64 = (-var_fbbtbot);
        let assign24900_e29398: f64 = (assign24900_e29396 / var_fmaxr);
        let assign24900_e29400: f64 = (assign24900_e29398 - 230.25850929940458);
        let assign24900_e29402: f64 = (assign24900_e29400 * 0.3333333333333333);
        let assign24900_e29403: f64 = (1.0 + assign24900_e29402);
        let assign24900_e29404: f64 = (assign24900_e29393 * assign24900_e29403);
        let assign24900_e29405: f64 = (0.5 * assign24900_e29404);
        let assign24900_e29406: f64 = (1.0 + assign24900_e29405);
        let assign24900_e29407: f64 = (assign24900_e29385 * assign24900_e29406);
        let assign24900_e29408: f64 = (1.0 + assign24900_e29407);
        let assign24900_e29409: f64 = (1e100 * assign24900_e29408);
        (assign24900_e29409, (1e100 * (((-((assign24900_e29381 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign24900_e29406) + (assign24900_e29385 * (0.5 * (((-((assign24900_e29389 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign24900_e29403) + (assign24900_e29393 * ((-((assign24900_e29396 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign24900_e29381 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign24900_e29406) + (assign24900_e29385 * (0.5 * (((-((assign24900_e29389 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign24900_e29403) + (assign24900_e29393 * ((-((assign24900_e29396 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign24900_e29381 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign24900_e29406) + (assign24900_e29385 * (0.5 * (((-((assign24900_e29389 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign24900_e29403) + (assign24900_e29393 * ((-((assign24900_e29396 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign24900_e29381 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign24900_e29406) + (assign24900_e29385 * (0.5 * (((-((assign24900_e29389 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign24900_e29403) + (assign24900_e29393 * ((-((assign24900_e29396 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign24900_e29411;
        var_tmp_dn6 = assign24900_e29411_d_n6;
        var_tmp_dn7 = assign24900_e29411_d_n7;
        var_tmp_dn8 = assign24900_e29411_d_n8;
        var_tmp_dn9 = assign24900_e29411_d_n9;

        let (assign24910_e29431, assign24910_e29431_d_n6, assign24910_e29431_d_n7, assign24910_e29431_d_n8, assign24910_e29431_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard480 == 0.0)) {
        let assign24910_e29424: f64 = (var_v5 * var_fmaxr);
        let assign24910_e29426: f64 = (assign24910_e29424 * var_fmaxr);
        let assign24910_e29428: f64 = (assign24910_e29426 * var_tmp);
        let assign24910_e29429: f64 = (p.p868 * assign24910_e29428);
        (assign24910_e29429, (p.p868 * (((((var_v5 * var_fmaxr_dn6) * var_fmaxr) + (assign24910_e29424 * var_fmaxr_dn6)) * var_tmp) + (assign24910_e29426 * var_tmp_dn6))), (p.p868 * (((((var_v5 * var_fmaxr_dn7) * var_fmaxr) + (assign24910_e29424 * var_fmaxr_dn7)) * var_tmp) + (assign24910_e29426 * var_tmp_dn7))), (p.p868 * (((((var_v5 * var_fmaxr_dn8) * var_fmaxr) + (assign24910_e29424 * var_fmaxr_dn8)) * var_tmp) + (assign24910_e29426 * var_tmp_dn8))), (p.p868 * (((((var_v5 * var_fmaxr_dn9) * var_fmaxr) + (assign24910_e29424 * var_fmaxr_dn9)) * var_tmp) + (assign24910_e29426 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign24910_e29431;
        var_ibbt_dn6 = assign24910_e29431_d_n6;
        var_ibbt_dn7 = assign24910_e29431_d_n7;
        var_ibbt_dn8 = assign24910_e29431_d_n8;
        var_ibbt_dn9 = assign24910_e29431_d_n9;

        let assign24920_e29434: f64 = if p.p877 > 1000.0 { 1.0 } else { 0.0 };
        var_guard484 = assign24920_e29434;

        let (assign24930_e29445, assign24930_e29445_d_n6, assign24930_e29445_d_n7, assign24930_e29445_d_n8, assign24930_e29445_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard484 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign24930_e29445;
        var_fbreakdown_dn6 = assign24930_e29445_d_n6;
        var_fbreakdown_dn7 = assign24930_e29445_d_n7;
        var_fbreakdown_dn8 = assign24930_e29445_d_n8;
        var_fbreakdown_dn9 = assign24930_e29445_d_n9;

        let assign24940_e29448: f64 = (-var_alphaav);
        let assign24940_e29450: f64 = (assign24940_e29448 * p.p877);
        let assign24940_e29451: f64 = if var_vav > assign24940_e29450 { 1.0 } else { 0.0 };
        var_guard485 = assign24940_e29451;

        let assign24950_e29454: f64 = if p.p880 == 4.0 { 1.0 } else { 0.0 };
        var_guard486 = assign24950_e29454;

        let (assign24960_e29484, assign24960_e29484_d_n6, assign24960_e29484_d_n7, assign24960_e29484_d_n8, assign24960_e29484_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard484 == 0.0)) && (var_guard485 != 0.0)) && (var_guard486 != 0.0)) {
        let assign24960_e29470: f64 = (var_vav * var_vbrinvbot);
        let assign24960_e29473: f64 = (var_vav * var_vbrinvbot);
        let assign24960_e29474: f64 = (assign24960_e29470 * assign24960_e29473);
        let assign24960_e29477: f64 = (var_vav * var_vbrinvbot);
        let assign24960_e29478: f64 = (assign24960_e29474 * assign24960_e29477);
        let assign24960_e29481: f64 = (var_vav * var_vbrinvbot);
        let assign24960_e29482: f64 = (assign24960_e29478 * assign24960_e29481);
        (assign24960_e29482, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign24960_e29484;
        var_tmp_dn6 = assign24960_e29484_d_n6;
        var_tmp_dn7 = assign24960_e29484_d_n7;
        var_tmp_dn8 = assign24960_e29484_d_n8;
        var_tmp_dn9 = assign24960_e29484_d_n9;

        let (assign24970_e29506, assign24970_e29506_d_n6, assign24970_e29506_d_n7, assign24970_e29506_d_n8, assign24970_e29506_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard484 == 0.0)) && (var_guard485 != 0.0)) && (var_guard486 == 0.0)) {
        let assign24970_e29501: f64 = (var_vav * var_vbrinvbot);
        let assign24970_e29502: f64 = (assign24970_e29501).abs();
        let assign24970_e29504: f64 = (assign24970_e29502).powf(p.p880);
        (assign24970_e29504, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign24970_e29506;
        var_tmp_dn6 = assign24970_e29506_d_n6;
        var_tmp_dn7 = assign24970_e29506_d_n7;
        var_tmp_dn8 = assign24970_e29506_d_n8;
        var_tmp_dn9 = assign24970_e29506_d_n9;

        let (assign24980_e29524, assign24980_e29524_d_n6, assign24980_e29524_d_n7, assign24980_e29524_d_n8, assign24980_e29524_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard484 == 0.0)) && (var_guard485 != 0.0)) {
        let assign24980_e29521: f64 = (1.0 - var_tmp);
        let assign24980_e29522: f64 = (1.0 / assign24980_e29521);
        (assign24980_e29522, (-((-var_tmp_dn6) / (assign24980_e29521 * assign24980_e29521))), (-((-var_tmp_dn7) / (assign24980_e29521 * assign24980_e29521))), (-((-var_tmp_dn8) / (assign24980_e29521 * assign24980_e29521))), (-((-var_tmp_dn9) / (assign24980_e29521 * assign24980_e29521))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign24980_e29524;
        var_fbreakdown_dn6 = assign24980_e29524_d_n6;
        var_fbreakdown_dn7 = assign24980_e29524_d_n7;
        var_fbreakdown_dn8 = assign24980_e29524_d_n8;
        var_fbreakdown_dn9 = assign24980_e29524_d_n9;

        let (assign24990_e29547, assign24990_e29547_d_n6, assign24990_e29547_d_n7, assign24990_e29547_d_n8, assign24990_e29547_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) && (var_guard484 == 0.0)) && (var_guard485 == 0.0)) {
        let assign24990_e29541: f64 = (var_alphaav * p.p877);
        let assign24990_e29542: f64 = (var_vav + assign24990_e29541);
        let assign24990_e29544: f64 = (assign24990_e29542 * var_slopebot);
        let assign24990_e29545: f64 = (var_fstopbot + assign24990_e29544);
        (assign24990_e29545, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign24990_e29547;
        var_fbreakdown_dn6 = assign24990_e29547_d_n6;
        var_fbreakdown_dn7 = assign24990_e29547_d_n7;
        var_fbreakdown_dn8 = assign24990_e29547_d_n8;
        var_fbreakdown_dn9 = assign24990_e29547_d_n9;

        let (assign25000_e29566, assign25000_e29566_d_n6, assign25000_e29566_d_n7, assign25000_e29566_d_n8, assign25000_e29566_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard470 == 0.0)) {
        let assign25000_e29557: f64 = (var_id__blk212 + var_isrh);
        let assign25000_e29559: f64 = (assign25000_e29557 + var_itat);
        let assign25000_e29561: f64 = (assign25000_e29559 + var_ibbt);
        let assign25000_e29562: f64 = (p.p29 * assign25000_e29561);
        let assign25000_e29564: f64 = (assign25000_e29562 * var_fbreakdown);
        (assign25000_e29564, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign25000_e29562 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign25000_e29562 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign25000_e29562 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign25000_e29562 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign25000_e29566;
        var_ijunbot_dn6 = assign25000_e29566_d_n6;
        var_ijunbot_dn7 = assign25000_e29566_d_n7;
        var_ijunbot_dn8 = assign25000_e29566_d_n8;
        var_ijunbot_dn9 = assign25000_e29566_d_n9;

        let assign25010_e29569: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard487 = assign25010_e29569;

        let (assign25020_e29577, assign25020_e29577_d_n6, assign25020_e29577_d_n7, assign25020_e29577_d_n8, assign25020_e29577_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign25020_e29577;
        var_ijunsti_dn6 = assign25020_e29577_d_n6;
        var_ijunsti_dn7 = assign25020_e29577_d_n7;
        var_ijunsti_dn8 = assign25020_e29577_d_n8;
        var_ijunsti_dn9 = assign25020_e29577_d_n9;

        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfcpos_dn9_slot = var_erfcpos_dn9;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_erfctimesexpmtat_dn9_slot = var_erfctimesexpmtat_dn9;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_fmaxr_dn9_slot = var_fmaxr_dn9;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_gammamax_dn9_slot = var_gammamax_dn9;
        *var_guard477_slot = var_guard477;
        *var_guard478_slot = var_guard478;
        *var_guard479_slot = var_guard479;
        *var_guard480_slot = var_guard480;
        *var_guard481_slot = var_guard481;
        *var_guard482_slot = var_guard482;
        *var_guard483_slot = var_guard483;
        *var_guard484_slot = var_guard484;
        *var_guard485_slot = var_guard485;
        *var_guard486_slot = var_guard486;
        *var_guard487_slot = var_guard487;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijunbot_dn9_slot = var_ijunbot_dn9;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_ijunsti_dn9_slot = var_ijunsti_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_terfc_dn9_slot = var_terfc_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
    }
}
