#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_48(
        p: &Parameters,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatbot: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn5: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_cerfc: f64,
        var_fbbtbot: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard471: f64,
        var_guard475: f64,
        var_one_over_one_minus_pbot: f64,
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
        var_v5: f64,
        var_vbbt: f64,
        var_vbirbotinv: f64,
        var_wdepnulrinvbot: f64,
        var_wgamma: f64,
        var_wgamma_dn5: f64,
        var_wgamma_dn6: f64,
        var_wgamma_dn7: f64,
        var_wgamma_dn8: f64,
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
        var_guard477_slot: &mut f64,
        var_guard478_slot: &mut f64,
        var_guard479_slot: &mut f64,
        var_guard480_slot: &mut f64,
        var_guard481_slot: &mut f64,
        var_guard482_slot: &mut f64,
        var_guard483_slot: &mut f64,
        var_guard484_slot: &mut f64,
        var_guard485_slot: &mut f64,
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
        let mut var_guard477: f64 = *var_guard477_slot;
        let mut var_guard478: f64 = *var_guard478_slot;
        let mut var_guard479: f64 = *var_guard479_slot;
        let mut var_guard480: f64 = *var_guard480_slot;
        let mut var_guard481: f64 = *var_guard481_slot;
        let mut var_guard482: f64 = *var_guard482_slot;
        let mut var_guard483: f64 = *var_guard483_slot;
        let mut var_guard484: f64 = *var_guard484_slot;
        let mut var_guard485: f64 = *var_guard485_slot;
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

        let (assign25480_e29327, assign25480_e29327_d_n5, assign25480_e29327_d_n6, assign25480_e29327_d_n7, assign25480_e29327_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) {
        let assign25480_e29321: f64 = (var_wsrh * var_wgamma);
        let assign25480_e29324: f64 = (var_wsrh + var_wgamma);
        let assign25480_e29325: f64 = (assign25480_e29321 / assign25480_e29324);
        (assign25480_e29325, ((((var_wsrh * var_wgamma_dn5) * assign25480_e29324) - (assign25480_e29321 * var_wgamma_dn5)) / (assign25480_e29324 * assign25480_e29324)), ((((var_wsrh * var_wgamma_dn6) * assign25480_e29324) - (assign25480_e29321 * var_wgamma_dn6)) / (assign25480_e29324 * assign25480_e29324)), ((((var_wsrh * var_wgamma_dn7) * assign25480_e29324) - (assign25480_e29321 * var_wgamma_dn7)) / (assign25480_e29324 * assign25480_e29324)), ((((var_wsrh * var_wgamma_dn8) * assign25480_e29324) - (assign25480_e29321 * var_wgamma_dn8)) / (assign25480_e29324 * assign25480_e29324)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign25480_e29327;
        var_wtat_dn5 = assign25480_e29327_d_n5;
        var_wtat_dn6 = assign25480_e29327_d_n6;
        var_wtat_dn7 = assign25480_e29327_d_n7;
        var_wtat_dn8 = assign25480_e29327_d_n8;

        let (assign25490_e29344, assign25490_e29344_d_n5, assign25490_e29344_d_n6, assign25490_e29344_d_n7, assign25490_e29344_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) {
        let assign25490_e29340: f64 = (var_btat / var_sqrtumax);
        let assign25490_e29341: f64 = (0.375 * assign25490_e29340);
        let assign25490_e29342: f64 = (assign25490_e29341).sqrt();
        (assign25490_e29342, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign25490_e29342)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign25490_e29342)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign25490_e29342)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign25490_e29342)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign25490_e29344;
        var_ktat_dn5 = assign25490_e29344_d_n5;
        var_ktat_dn6 = assign25490_e29344_d_n6;
        var_ktat_dn7 = assign25490_e29344_d_n7;
        var_ktat_dn8 = assign25490_e29344_d_n8;

        let (assign25500_e29362, assign25500_e29362_d_n5, assign25500_e29362_d_n6, assign25500_e29362_d_n7, assign25500_e29362_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) {
        let assign25500_e29357: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign25500_e29358: f64 = (2.0 * assign25500_e29357);
        let assign25500_e29360: f64 = (assign25500_e29358 - var_umax);
        (assign25500_e29360, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign25500_e29362;
        var_ltat_dn5 = assign25500_e29362_d_n5;
        var_ltat_dn6 = assign25500_e29362_d_n6;
        var_ltat_dn7 = assign25500_e29362_d_n7;
        var_ltat_dn8 = assign25500_e29362_d_n8;

        let (assign25510_e29388, assign25510_e29388_d_n5, assign25510_e29388_d_n6, assign25510_e29388_d_n7, assign25510_e29388_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) {
        let assign25510_e29374: f64 = (var_atatbot * var_twoatatoverthreebtat);
        let assign25510_e29376: f64 = (assign25510_e29374 * var_sqrtumax);
        let assign25510_e29379: f64 = (var_atatbot * var_umax);
        let assign25510_e29380: f64 = (assign25510_e29376 - assign25510_e29379);
        let assign25510_e29384: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign25510_e29385: f64 = (0.5 * assign25510_e29384);
        let assign25510_e29386: f64 = (assign25510_e29380 + assign25510_e29385);
        (assign25510_e29386, (((((var_atatbot * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign25510_e29374 * var_sqrtumax_dn5)) - (var_atatbot * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatbot * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign25510_e29374 * var_sqrtumax_dn6)) - (var_atatbot * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign25510_e29374 * var_sqrtumax_dn7)) - (var_atatbot * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign25510_e29374 * var_sqrtumax_dn8)) - (var_atatbot * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign25510_e29388;
        var_mtat_dn5 = assign25510_e29388_d_n5;
        var_mtat_dn6 = assign25510_e29388_d_n6;
        var_mtat_dn7 = assign25510_e29388_d_n7;
        var_mtat_dn8 = assign25510_e29388_d_n8;

        let (assign25520_e29404, assign25520_e29404_d_n5, assign25520_e29404_d_n6, assign25520_e29404_d_n7, assign25520_e29404_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) {
        let assign25520_e29400: f64 = (var_ltat - 1.0);
        let assign25520_e29402: f64 = (assign25520_e29400 * var_ktat);
        (assign25520_e29402, ((var_ltat_dn5 * var_ktat) + (assign25520_e29400 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign25520_e29400 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign25520_e29400 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign25520_e29400 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign25520_e29404;
        var_xerfc_dn5 = assign25520_e29404_d_n5;
        var_xerfc_dn6 = assign25520_e29404_d_n6;
        var_xerfc_dn7 = assign25520_e29404_d_n7;
        var_xerfc_dn8 = assign25520_e29404_d_n8;

        let (assign25530_e29418, assign25530_e29418_d_n5, assign25530_e29418_d_n6, assign25530_e29418_d_n7, assign25530_e29418_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) {
        let assign25530_e29416: f64 = (var_xerfc * var_xerfc);
        (assign25530_e29416, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign25530_e29418;
        var_ysq_dn5 = assign25530_e29418_d_n5;
        var_ysq_dn6 = assign25530_e29418_d_n6;
        var_ysq_dn7 = assign25530_e29418_d_n7;
        var_ysq_dn8 = assign25530_e29418_d_n8;

        let assign25540_e29421: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard477 = assign25540_e29421;

        let (assign25550_e29441, assign25550_e29441_d_n5, assign25550_e29441_d_n6, assign25550_e29441_d_n7, assign25550_e29441_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) && (var_guard477 != 0.0)) {
        let assign25550_e29437: f64 = (var_perfc * var_xerfc);
        let assign25550_e29438: f64 = (1.0 + assign25550_e29437);
        let assign25550_e29439: f64 = (1.0 / assign25550_e29438);
        (assign25550_e29439, (-((var_perfc * var_xerfc_dn5) / (assign25550_e29438 * assign25550_e29438))), (-((var_perfc * var_xerfc_dn6) / (assign25550_e29438 * assign25550_e29438))), (-((var_perfc * var_xerfc_dn7) / (assign25550_e29438 * assign25550_e29438))), (-((var_perfc * var_xerfc_dn8) / (assign25550_e29438 * assign25550_e29438))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign25550_e29441;
        var_terfc_dn5 = assign25550_e29441_d_n5;
        var_terfc_dn6 = assign25550_e29441_d_n6;
        var_terfc_dn7 = assign25550_e29441_d_n7;
        var_terfc_dn8 = assign25550_e29441_d_n8;

        let (assign25560_e29462, assign25560_e29462_d_n5, assign25560_e29462_d_n6, assign25560_e29462_d_n7, assign25560_e29462_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) && (var_guard477 == 0.0)) {
        let assign25560_e29458: f64 = (var_perfc * var_xerfc);
        let assign25560_e29459: f64 = (1.0 - assign25560_e29458);
        let assign25560_e29460: f64 = (1.0 / assign25560_e29459);
        (assign25560_e29460, (-((-(var_perfc * var_xerfc_dn5)) / (assign25560_e29459 * assign25560_e29459))), (-((-(var_perfc * var_xerfc_dn6)) / (assign25560_e29459 * assign25560_e29459))), (-((-(var_perfc * var_xerfc_dn7)) / (assign25560_e29459 * assign25560_e29459))), (-((-(var_perfc * var_xerfc_dn8)) / (assign25560_e29459 * assign25560_e29459))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign25560_e29462;
        var_terfc_dn5 = assign25560_e29462_d_n5;
        var_terfc_dn6 = assign25560_e29462_d_n6;
        var_terfc_dn7 = assign25560_e29462_d_n7;
        var_terfc_dn8 = assign25560_e29462_d_n8;

        let assign25570_e29464: f64 = (-var_ysq);
        let assign25570_e29466: f64 = (assign25570_e29464 + var_mtat);
        let assign25570_e29468: f64 = (-230.25850929940458);
        let assign25570_e29469: f64 = if assign25570_e29466 > assign25570_e29468 { 1.0 } else { 0.0 };
        var_guard478 = assign25570_e29469;

        let (assign25580_e29487, assign25580_e29487_d_n5, assign25580_e29487_d_n6, assign25580_e29487_d_n7, assign25580_e29487_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) && (var_guard478 != 0.0)) {
        let assign25580_e29482: f64 = (-var_ysq);
        let assign25580_e29484: f64 = (assign25580_e29482 + var_mtat);
        let assign25580_e29485: f64 = (assign25580_e29484).exp();
        (assign25580_e29485, (assign25580_e29485 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign25580_e29485 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign25580_e29485 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign25580_e29485 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25580_e29487;
        var_tmp_dn5 = assign25580_e29487_d_n5;
        var_tmp_dn6 = assign25580_e29487_d_n6;
        var_tmp_dn7 = assign25580_e29487_d_n7;
        var_tmp_dn8 = assign25580_e29487_d_n8;

        let (assign25590_e29536, assign25590_e29536_d_n5, assign25590_e29536_d_n6, assign25590_e29536_d_n7, assign25590_e29536_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) && (var_guard478 == 0.0)) {
        let assign25590_e29503: f64 = (-230.25850929940458);
        let assign25590_e29505: f64 = (-var_ysq);
        let assign25590_e29507: f64 = (assign25590_e29505 + var_mtat);
        let assign25590_e29508: f64 = (assign25590_e29503 - assign25590_e29507);
        let assign25590_e29512: f64 = (-230.25850929940458);
        let assign25590_e29514: f64 = (-var_ysq);
        let assign25590_e29516: f64 = (assign25590_e29514 + var_mtat);
        let assign25590_e29517: f64 = (assign25590_e29512 - assign25590_e29516);
        let assign25590_e29520: f64 = (-230.25850929940458);
        let assign25590_e29522: f64 = (-var_ysq);
        let assign25590_e29524: f64 = (assign25590_e29522 + var_mtat);
        let assign25590_e29525: f64 = (assign25590_e29520 - assign25590_e29524);
        let assign25590_e29527: f64 = (assign25590_e29525 * 0.3333333333333333);
        let assign25590_e29528: f64 = (1.0 + assign25590_e29527);
        let assign25590_e29529: f64 = (assign25590_e29517 * assign25590_e29528);
        let assign25590_e29530: f64 = (0.5 * assign25590_e29529);
        let assign25590_e29531: f64 = (1.0 + assign25590_e29530);
        let assign25590_e29532: f64 = (assign25590_e29508 * assign25590_e29531);
        let assign25590_e29533: f64 = (1.0 + assign25590_e29532);
        let assign25590_e29534: f64 = (1e-100 / assign25590_e29533);
        (assign25590_e29534, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign25590_e29531) + (assign25590_e29508 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign25590_e29528) + (assign25590_e29517 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign25590_e29533 * assign25590_e29533))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign25590_e29531) + (assign25590_e29508 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign25590_e29528) + (assign25590_e29517 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign25590_e29533 * assign25590_e29533))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign25590_e29531) + (assign25590_e29508 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign25590_e29528) + (assign25590_e29517 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign25590_e29533 * assign25590_e29533))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign25590_e29531) + (assign25590_e29508 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign25590_e29528) + (assign25590_e29517 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign25590_e29533 * assign25590_e29533))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25590_e29536;
        var_tmp_dn5 = assign25590_e29536_d_n5;
        var_tmp_dn6 = assign25590_e29536_d_n6;
        var_tmp_dn7 = assign25590_e29536_d_n7;
        var_tmp_dn8 = assign25590_e29536_d_n8;

        let (assign25600_e29566, assign25600_e29566_d_n5, assign25600_e29566_d_n6, assign25600_e29566_d_n7, assign25600_e29566_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) {
        let assign25600_e29548: f64 = (0.29214664 * var_terfc);
        let assign25600_e29552: f64 = (var_terfc * var_terfc);
        let assign25600_e29553: f64 = (var_berfc * assign25600_e29552);
        let assign25600_e29554: f64 = (assign25600_e29548 + assign25600_e29553);
        let assign25600_e29558: f64 = (var_terfc * var_terfc);
        let assign25600_e29560: f64 = (assign25600_e29558 * var_terfc);
        let assign25600_e29561: f64 = (var_cerfc * assign25600_e29560);
        let assign25600_e29562: f64 = (assign25600_e29554 + assign25600_e29561);
        let assign25600_e29564: f64 = (assign25600_e29562 * var_tmp);
        (assign25600_e29564, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign25600_e29558 * var_terfc_dn5)))) * var_tmp) + (assign25600_e29562 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign25600_e29558 * var_terfc_dn6)))) * var_tmp) + (assign25600_e29562 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign25600_e29558 * var_terfc_dn7)))) * var_tmp) + (assign25600_e29562 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign25600_e29558 * var_terfc_dn8)))) * var_tmp) + (assign25600_e29562 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign25600_e29566;
        var_erfcpos_dn5 = assign25600_e29566_d_n5;
        var_erfcpos_dn6 = assign25600_e29566_d_n6;
        var_erfcpos_dn7 = assign25600_e29566_d_n7;
        var_erfcpos_dn8 = assign25600_e29566_d_n8;

        let assign25610_e29569: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard479 = assign25610_e29569;

        let (assign25620_e29583, assign25620_e29583_d_n5, assign25620_e29583_d_n6, assign25620_e29583_d_n7, assign25620_e29583_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) && (var_guard479 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign25620_e29583;
        var_erfctimesexpmtat_dn5 = assign25620_e29583_d_n5;
        var_erfctimesexpmtat_dn6 = assign25620_e29583_d_n6;
        var_erfctimesexpmtat_dn7 = assign25620_e29583_d_n7;
        var_erfctimesexpmtat_dn8 = assign25620_e29583_d_n8;

        let assign25630_e29586: f64 = (-230.25850929940458);
        let assign25630_e29587: f64 = if var_mtat > assign25630_e29586 { 1.0 } else { 0.0 };
        var_guard480 = assign25630_e29587;

        let (assign25640_e29605, assign25640_e29605_d_n5, assign25640_e29605_d_n6, assign25640_e29605_d_n7, assign25640_e29605_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) && (var_guard479 == 0.0)) && (var_guard480 != 0.0)) {
        let assign25640_e29603: f64 = (var_mtat).exp();
        (assign25640_e29603, (assign25640_e29603 * var_mtat_dn5), (assign25640_e29603 * var_mtat_dn6), (assign25640_e29603 * var_mtat_dn7), (assign25640_e29603 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25640_e29605;
        var_tmp_dn5 = assign25640_e29605_d_n5;
        var_tmp_dn6 = assign25640_e29605_d_n6;
        var_tmp_dn7 = assign25640_e29605_d_n7;
        var_tmp_dn8 = assign25640_e29605_d_n8;

        let (assign25650_e29648, assign25650_e29648_d_n5, assign25650_e29648_d_n6, assign25650_e29648_d_n7, assign25650_e29648_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) && (var_guard479 == 0.0)) && (var_guard480 == 0.0)) {
        let assign25650_e29624: f64 = (-230.25850929940458);
        let assign25650_e29626: f64 = (assign25650_e29624 - var_mtat);
        let assign25650_e29630: f64 = (-230.25850929940458);
        let assign25650_e29632: f64 = (assign25650_e29630 - var_mtat);
        let assign25650_e29635: f64 = (-230.25850929940458);
        let assign25650_e29637: f64 = (assign25650_e29635 - var_mtat);
        let assign25650_e29639: f64 = (assign25650_e29637 * 0.3333333333333333);
        let assign25650_e29640: f64 = (1.0 + assign25650_e29639);
        let assign25650_e29641: f64 = (assign25650_e29632 * assign25650_e29640);
        let assign25650_e29642: f64 = (0.5 * assign25650_e29641);
        let assign25650_e29643: f64 = (1.0 + assign25650_e29642);
        let assign25650_e29644: f64 = (assign25650_e29626 * assign25650_e29643);
        let assign25650_e29645: f64 = (1.0 + assign25650_e29644);
        let assign25650_e29646: f64 = (1e-100 / assign25650_e29645);
        (assign25650_e29646, (-((1e-100 * (((-var_mtat_dn5) * assign25650_e29643) + (assign25650_e29626 * (0.5 * (((-var_mtat_dn5) * assign25650_e29640) + (assign25650_e29632 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign25650_e29645 * assign25650_e29645))), (-((1e-100 * (((-var_mtat_dn6) * assign25650_e29643) + (assign25650_e29626 * (0.5 * (((-var_mtat_dn6) * assign25650_e29640) + (assign25650_e29632 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign25650_e29645 * assign25650_e29645))), (-((1e-100 * (((-var_mtat_dn7) * assign25650_e29643) + (assign25650_e29626 * (0.5 * (((-var_mtat_dn7) * assign25650_e29640) + (assign25650_e29632 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign25650_e29645 * assign25650_e29645))), (-((1e-100 * (((-var_mtat_dn8) * assign25650_e29643) + (assign25650_e29626 * (0.5 * (((-var_mtat_dn8) * assign25650_e29640) + (assign25650_e29632 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign25650_e29645 * assign25650_e29645))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25650_e29648;
        var_tmp_dn5 = assign25650_e29648_d_n5;
        var_tmp_dn6 = assign25650_e29648_d_n6;
        var_tmp_dn7 = assign25650_e29648_d_n7;
        var_tmp_dn8 = assign25650_e29648_d_n8;

        let (assign25660_e29667, assign25660_e29667_d_n5, assign25660_e29667_d_n6, assign25660_e29667_d_n7, assign25660_e29667_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) && (var_guard479 == 0.0)) {
        let assign25660_e29663: f64 = (2.0 * var_tmp);
        let assign25660_e29665: f64 = (assign25660_e29663 - var_erfcpos);
        (assign25660_e29665, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign25660_e29667;
        var_erfctimesexpmtat_dn5 = assign25660_e29667_d_n5;
        var_erfctimesexpmtat_dn6 = assign25660_e29667_d_n6;
        var_erfctimesexpmtat_dn7 = assign25660_e29667_d_n7;
        var_erfctimesexpmtat_dn8 = assign25660_e29667_d_n8;

        let (assign25670_e29687, assign25670_e29687_d_n5, assign25670_e29687_d_n6, assign25670_e29687_d_n7, assign25670_e29687_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) {
        let assign25670_e29679: f64 = (1.772453850905516 * 0.5);
        let assign25670_e29682: f64 = (var_atatbot * var_erfctimesexpmtat);
        let assign25670_e29684: f64 = (assign25670_e29682 / var_ktat);
        let assign25670_e29685: f64 = (assign25670_e29679 * assign25670_e29684);
        (assign25670_e29685, (assign25670_e29679 * ((((var_atatbot * var_erfctimesexpmtat_dn5) * var_ktat) - (assign25670_e29682 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign25670_e29679 * ((((var_atatbot * var_erfctimesexpmtat_dn6) * var_ktat) - (assign25670_e29682 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign25670_e29679 * ((((var_atatbot * var_erfctimesexpmtat_dn7) * var_ktat) - (assign25670_e29682 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign25670_e29679 * ((((var_atatbot * var_erfctimesexpmtat_dn8) * var_ktat) - (assign25670_e29682 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign25670_e29687;
        var_gammamax_dn5 = assign25670_e29687_d_n5;
        var_gammamax_dn6 = assign25670_e29687_d_n6;
        var_gammamax_dn7 = assign25670_e29687_d_n7;
        var_gammamax_dn8 = assign25670_e29687_d_n8;

        let (assign25680_e29705, assign25680_e29705_d_n5, assign25680_e29705_d_n6, assign25680_e29705_d_n7, assign25680_e29705_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard475 == 0.0)) {
        let assign25680_e29700: f64 = (var_asrh * var_gammamax);
        let assign25680_e29702: f64 = (assign25680_e29700 * var_wtat);
        let assign25680_e29703: f64 = (p.p838 * assign25680_e29702);
        (assign25680_e29703, (p.p838 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign25680_e29700 * var_wtat_dn5))), (p.p838 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign25680_e29700 * var_wtat_dn6))), (p.p838 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign25680_e29700 * var_wtat_dn7))), (p.p838 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign25680_e29700 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign25680_e29705;
        var_itat_dn5 = assign25680_e29705_d_n5;
        var_itat_dn6 = assign25680_e29705_d_n6;
        var_itat_dn7 = assign25680_e29705_d_n7;
        var_itat_dn8 = assign25680_e29705_d_n8;

        let assign25690_e29708: f64 = if p.p844 == 0.0 { 1.0 } else { 0.0 };
        var_guard481 = assign25690_e29708;

        let (assign25700_e29719, assign25700_e29719_d_n5, assign25700_e29719_d_n6, assign25700_e29719_d_n7, assign25700_e29719_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard481 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign25700_e29719;
        var_ibbt_dn5 = assign25700_e29719_d_n5;
        var_ibbt_dn6 = assign25700_e29719_d_n6;
        var_ibbt_dn7 = assign25700_e29719_d_n7;
        var_ibbt_dn8 = assign25700_e29719_d_n8;

        let assign25710_e29722: f64 = if p.p824 == 0.5 { 1.0 } else { 0.0 };
        var_guard482 = assign25710_e29722;

        let (assign25720_e29741, assign25720_e29741_d_n5, assign25720_e29741_d_n6, assign25720_e29741_d_n7, assign25720_e29741_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard481 == 0.0)) && (var_guard482 != 0.0)) {
        let assign25720_e29736: f64 = (p.p821 - var_vbbt);
        let assign25720_e29738: f64 = (assign25720_e29736 * var_vbirbotinv);
        let assign25720_e29739: f64 = (assign25720_e29738).sqrt();
        (assign25720_e29739, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25720_e29741;
        var_tmp_dn5 = assign25720_e29741_d_n5;
        var_tmp_dn6 = assign25720_e29741_d_n6;
        var_tmp_dn7 = assign25720_e29741_d_n7;
        var_tmp_dn8 = assign25720_e29741_d_n8;

        let (assign25730_e29762, assign25730_e29762_d_n5, assign25730_e29762_d_n6, assign25730_e29762_d_n7, assign25730_e29762_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard481 == 0.0)) && (var_guard482 == 0.0)) {
        let assign25730_e29756: f64 = (p.p821 - var_vbbt);
        let assign25730_e29758: f64 = (assign25730_e29756 * var_vbirbotinv);
        let assign25730_e29760: f64 = (assign25730_e29758).powf(p.p824);
        (assign25730_e29760, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25730_e29762;
        var_tmp_dn5 = assign25730_e29762_d_n5;
        var_tmp_dn6 = assign25730_e29762_d_n6;
        var_tmp_dn7 = assign25730_e29762_d_n7;
        var_tmp_dn8 = assign25730_e29762_d_n8;

        let (assign25740_e29782, assign25740_e29782_d_n5, assign25740_e29782_d_n6, assign25740_e29782_d_n7, assign25740_e29782_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard481 == 0.0)) {
        let assign25740_e29775: f64 = (p.p821 - var_vbbt);
        let assign25740_e29777: f64 = (assign25740_e29775 * var_wdepnulrinvbot);
        let assign25740_e29779: f64 = (assign25740_e29777 / var_tmp);
        let assign25740_e29780: f64 = (var_one_over_one_minus_pbot * assign25740_e29779);
        (assign25740_e29780, (var_one_over_one_minus_pbot * (-((assign25740_e29777 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign25740_e29777 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign25740_e29777 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign25740_e29777 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign25740_e29782;
        var_fmaxr_dn5 = assign25740_e29782_d_n5;
        var_fmaxr_dn6 = assign25740_e29782_d_n6;
        var_fmaxr_dn7 = assign25740_e29782_d_n7;
        var_fmaxr_dn8 = assign25740_e29782_d_n8;

        let assign25750_e29784: f64 = (-var_fbbtbot);
        let assign25750_e29786: f64 = (assign25750_e29784 / var_fmaxr);
        let assign25750_e29787: f64 = (assign25750_e29786).abs();
        let assign25750_e29789: f64 = if assign25750_e29787 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard483 = assign25750_e29789;

        let (assign25760_e29807, assign25760_e29807_d_n5, assign25760_e29807_d_n6, assign25760_e29807_d_n7, assign25760_e29807_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard481 == 0.0)) && (var_guard483 != 0.0)) {
        let assign25760_e29802: f64 = (-var_fbbtbot);
        let assign25760_e29804: f64 = (assign25760_e29802 / var_fmaxr);
        let assign25760_e29805: f64 = (assign25760_e29804).exp();
        (assign25760_e29805, (assign25760_e29805 * (-((assign25760_e29802 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign25760_e29805 * (-((assign25760_e29802 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign25760_e29805 * (-((assign25760_e29802 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign25760_e29805 * (-((assign25760_e29802 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25760_e29807;
        var_tmp_dn5 = assign25760_e29807_d_n5;
        var_tmp_dn6 = assign25760_e29807_d_n6;
        var_tmp_dn7 = assign25760_e29807_d_n7;
        var_tmp_dn8 = assign25760_e29807_d_n8;

        let assign25770_e29809: f64 = (-var_fbbtbot);
        let assign25770_e29811: f64 = (assign25770_e29809 / var_fmaxr);
        let assign25770_e29813: f64 = if assign25770_e29811 < 0.0 { 1.0 } else { 0.0 };
        var_guard484 = assign25770_e29813;

        let (assign25780_e29864, assign25780_e29864_d_n5, assign25780_e29864_d_n6, assign25780_e29864_d_n7, assign25780_e29864_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard481 == 0.0)) && (var_guard483 == 0.0)) && (var_guard484 != 0.0)) {
        let assign25780_e29831: f64 = (-230.25850929940458);
        let assign25780_e29833: f64 = (-var_fbbtbot);
        let assign25780_e29835: f64 = (assign25780_e29833 / var_fmaxr);
        let assign25780_e29836: f64 = (assign25780_e29831 - assign25780_e29835);
        let assign25780_e29840: f64 = (-230.25850929940458);
        let assign25780_e29842: f64 = (-var_fbbtbot);
        let assign25780_e29844: f64 = (assign25780_e29842 / var_fmaxr);
        let assign25780_e29845: f64 = (assign25780_e29840 - assign25780_e29844);
        let assign25780_e29848: f64 = (-230.25850929940458);
        let assign25780_e29850: f64 = (-var_fbbtbot);
        let assign25780_e29852: f64 = (assign25780_e29850 / var_fmaxr);
        let assign25780_e29853: f64 = (assign25780_e29848 - assign25780_e29852);
        let assign25780_e29855: f64 = (assign25780_e29853 * 0.3333333333333333);
        let assign25780_e29856: f64 = (1.0 + assign25780_e29855);
        let assign25780_e29857: f64 = (assign25780_e29845 * assign25780_e29856);
        let assign25780_e29858: f64 = (0.5 * assign25780_e29857);
        let assign25780_e29859: f64 = (1.0 + assign25780_e29858);
        let assign25780_e29860: f64 = (assign25780_e29836 * assign25780_e29859);
        let assign25780_e29861: f64 = (1.0 + assign25780_e29860);
        let assign25780_e29862: f64 = (1e-100 / assign25780_e29861);
        (assign25780_e29862, (-((1e-100 * (((-(-((assign25780_e29833 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign25780_e29859) + (assign25780_e29836 * (0.5 * (((-(-((assign25780_e29842 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign25780_e29856) + (assign25780_e29845 * ((-(-((assign25780_e29850 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign25780_e29861 * assign25780_e29861))), (-((1e-100 * (((-(-((assign25780_e29833 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign25780_e29859) + (assign25780_e29836 * (0.5 * (((-(-((assign25780_e29842 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign25780_e29856) + (assign25780_e29845 * ((-(-((assign25780_e29850 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign25780_e29861 * assign25780_e29861))), (-((1e-100 * (((-(-((assign25780_e29833 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign25780_e29859) + (assign25780_e29836 * (0.5 * (((-(-((assign25780_e29842 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign25780_e29856) + (assign25780_e29845 * ((-(-((assign25780_e29850 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign25780_e29861 * assign25780_e29861))), (-((1e-100 * (((-(-((assign25780_e29833 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign25780_e29859) + (assign25780_e29836 * (0.5 * (((-(-((assign25780_e29842 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign25780_e29856) + (assign25780_e29845 * ((-(-((assign25780_e29850 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign25780_e29861 * assign25780_e29861))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25780_e29864;
        var_tmp_dn5 = assign25780_e29864_d_n5;
        var_tmp_dn6 = assign25780_e29864_d_n6;
        var_tmp_dn7 = assign25780_e29864_d_n7;
        var_tmp_dn8 = assign25780_e29864_d_n8;

        let (assign25790_e29913, assign25790_e29913_d_n5, assign25790_e29913_d_n6, assign25790_e29913_d_n7, assign25790_e29913_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard481 == 0.0)) && (var_guard483 == 0.0)) && (var_guard484 == 0.0)) {
        let assign25790_e29883: f64 = (-var_fbbtbot);
        let assign25790_e29885: f64 = (assign25790_e29883 / var_fmaxr);
        let assign25790_e29887: f64 = (assign25790_e29885 - 230.25850929940458);
        let assign25790_e29891: f64 = (-var_fbbtbot);
        let assign25790_e29893: f64 = (assign25790_e29891 / var_fmaxr);
        let assign25790_e29895: f64 = (assign25790_e29893 - 230.25850929940458);
        let assign25790_e29898: f64 = (-var_fbbtbot);
        let assign25790_e29900: f64 = (assign25790_e29898 / var_fmaxr);
        let assign25790_e29902: f64 = (assign25790_e29900 - 230.25850929940458);
        let assign25790_e29904: f64 = (assign25790_e29902 * 0.3333333333333333);
        let assign25790_e29905: f64 = (1.0 + assign25790_e29904);
        let assign25790_e29906: f64 = (assign25790_e29895 * assign25790_e29905);
        let assign25790_e29907: f64 = (0.5 * assign25790_e29906);
        let assign25790_e29908: f64 = (1.0 + assign25790_e29907);
        let assign25790_e29909: f64 = (assign25790_e29887 * assign25790_e29908);
        let assign25790_e29910: f64 = (1.0 + assign25790_e29909);
        let assign25790_e29911: f64 = (1e100 * assign25790_e29910);
        (assign25790_e29911, (1e100 * (((-((assign25790_e29883 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign25790_e29908) + (assign25790_e29887 * (0.5 * (((-((assign25790_e29891 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign25790_e29905) + (assign25790_e29895 * ((-((assign25790_e29898 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign25790_e29883 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign25790_e29908) + (assign25790_e29887 * (0.5 * (((-((assign25790_e29891 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign25790_e29905) + (assign25790_e29895 * ((-((assign25790_e29898 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign25790_e29883 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign25790_e29908) + (assign25790_e29887 * (0.5 * (((-((assign25790_e29891 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign25790_e29905) + (assign25790_e29895 * ((-((assign25790_e29898 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign25790_e29883 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign25790_e29908) + (assign25790_e29887 * (0.5 * (((-((assign25790_e29891 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign25790_e29905) + (assign25790_e29895 * ((-((assign25790_e29898 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25790_e29913;
        var_tmp_dn5 = assign25790_e29913_d_n5;
        var_tmp_dn6 = assign25790_e29913_d_n6;
        var_tmp_dn7 = assign25790_e29913_d_n7;
        var_tmp_dn8 = assign25790_e29913_d_n8;

        let (assign25800_e29933, assign25800_e29933_d_n5, assign25800_e29933_d_n6, assign25800_e29933_d_n7, assign25800_e29933_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard481 == 0.0)) {
        let assign25800_e29926: f64 = (var_v5 * var_fmaxr);
        let assign25800_e29928: f64 = (assign25800_e29926 * var_fmaxr);
        let assign25800_e29930: f64 = (assign25800_e29928 * var_tmp);
        let assign25800_e29931: f64 = (p.p844 * assign25800_e29930);
        (assign25800_e29931, (p.p844 * (((((var_v5 * var_fmaxr_dn5) * var_fmaxr) + (assign25800_e29926 * var_fmaxr_dn5)) * var_tmp) + (assign25800_e29928 * var_tmp_dn5))), (p.p844 * (((((var_v5 * var_fmaxr_dn6) * var_fmaxr) + (assign25800_e29926 * var_fmaxr_dn6)) * var_tmp) + (assign25800_e29928 * var_tmp_dn6))), (p.p844 * (((((var_v5 * var_fmaxr_dn7) * var_fmaxr) + (assign25800_e29926 * var_fmaxr_dn7)) * var_tmp) + (assign25800_e29928 * var_tmp_dn7))), (p.p844 * (((((var_v5 * var_fmaxr_dn8) * var_fmaxr) + (assign25800_e29926 * var_fmaxr_dn8)) * var_tmp) + (assign25800_e29928 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign25800_e29933;
        var_ibbt_dn5 = assign25800_e29933_d_n5;
        var_ibbt_dn6 = assign25800_e29933_d_n6;
        var_ibbt_dn7 = assign25800_e29933_d_n7;
        var_ibbt_dn8 = assign25800_e29933_d_n8;

        let assign25810_e29936: f64 = if p.p853 > 1000.0 { 1.0 } else { 0.0 };
        var_guard485 = assign25810_e29936;

        let (assign25820_e29947, assign25820_e29947_d_n5, assign25820_e29947_d_n6, assign25820_e29947_d_n7, assign25820_e29947_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard485 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign25820_e29947;
        var_fbreakdown_dn5 = assign25820_e29947_d_n5;
        var_fbreakdown_dn6 = assign25820_e29947_d_n6;
        var_fbreakdown_dn7 = assign25820_e29947_d_n7;
        var_fbreakdown_dn8 = assign25820_e29947_d_n8;

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
        *var_guard477_slot = var_guard477;
        *var_guard478_slot = var_guard478;
        *var_guard479_slot = var_guard479;
        *var_guard480_slot = var_guard480;
        *var_guard481_slot = var_guard481;
        *var_guard482_slot = var_guard482;
        *var_guard483_slot = var_guard483;
        *var_guard484_slot = var_guard484;
        *var_guard485_slot = var_guard485;
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

    pub(super) fn stamp_transient_block_49(
        p: &Parameters,
        var_alphaav: f64,
        var_atatsti: f64,
        var_btatpartsti: f64,
        var_fstopbot: f64,
        var_ftdsti: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard471: f64,
        var_guard485: f64,
        var_ibbt: f64,
        var_ibbt_dn5: f64,
        var_ibbt_dn6: f64,
        var_ibbt_dn7: f64,
        var_ibbt_dn8: f64,
        var_idmult: f64,
        var_idsatsti: f64,
        var_lssource_i: f64,
        var_one_minus_psti: f64,
        var_one_over_one_minus_psti: f64,
        var_slopebot: f64,
        var_two_psistar: f64,
        var_vav: f64,
        var_vbirstiinv: f64,
        var_vbisti: f64,
        var_vbrinvbot: f64,
        var_vjsrh: f64,
        var_wdepnulrsti: f64,
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
        var_guard486_slot: &mut f64,
        var_guard487_slot: &mut f64,
        var_guard488_slot: &mut f64,
        var_guard489_slot: &mut f64,
        var_guard490_slot: &mut f64,
        var_guard491_slot: &mut f64,
        var_guard492_slot: &mut f64,
        var_guard493_slot: &mut f64,
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
        let mut var_guard486: f64 = *var_guard486_slot;
        let mut var_guard487: f64 = *var_guard487_slot;
        let mut var_guard488: f64 = *var_guard488_slot;
        let mut var_guard489: f64 = *var_guard489_slot;
        let mut var_guard490: f64 = *var_guard490_slot;
        let mut var_guard491: f64 = *var_guard491_slot;
        let mut var_guard492: f64 = *var_guard492_slot;
        let mut var_guard493: f64 = *var_guard493_slot;
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

        let assign25830_e29950: f64 = (-var_alphaav);
        let assign25830_e29952: f64 = (assign25830_e29950 * p.p853);
        let assign25830_e29953: f64 = if var_vav > assign25830_e29952 { 1.0 } else { 0.0 };
        var_guard486 = assign25830_e29953;

        let assign25840_e29956: f64 = if p.p856 == 4.0 { 1.0 } else { 0.0 };
        var_guard487 = assign25840_e29956;

        let (assign25850_e29986, assign25850_e29986_d_n5, assign25850_e29986_d_n6, assign25850_e29986_d_n7, assign25850_e29986_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard485 == 0.0)) && (var_guard486 != 0.0)) && (var_guard487 != 0.0)) {
        let assign25850_e29972: f64 = (var_vav * var_vbrinvbot);
        let assign25850_e29975: f64 = (var_vav * var_vbrinvbot);
        let assign25850_e29976: f64 = (assign25850_e29972 * assign25850_e29975);
        let assign25850_e29979: f64 = (var_vav * var_vbrinvbot);
        let assign25850_e29980: f64 = (assign25850_e29976 * assign25850_e29979);
        let assign25850_e29983: f64 = (var_vav * var_vbrinvbot);
        let assign25850_e29984: f64 = (assign25850_e29980 * assign25850_e29983);
        (assign25850_e29984, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25850_e29986;
        var_tmp_dn5 = assign25850_e29986_d_n5;
        var_tmp_dn6 = assign25850_e29986_d_n6;
        var_tmp_dn7 = assign25850_e29986_d_n7;
        var_tmp_dn8 = assign25850_e29986_d_n8;

        let (assign25860_e30008, assign25860_e30008_d_n5, assign25860_e30008_d_n6, assign25860_e30008_d_n7, assign25860_e30008_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard485 == 0.0)) && (var_guard486 != 0.0)) && (var_guard487 == 0.0)) {
        let assign25860_e30003: f64 = (var_vav * var_vbrinvbot);
        let assign25860_e30004: f64 = (assign25860_e30003).abs();
        let assign25860_e30006: f64 = (assign25860_e30004).powf(p.p856);
        (assign25860_e30006, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25860_e30008;
        var_tmp_dn5 = assign25860_e30008_d_n5;
        var_tmp_dn6 = assign25860_e30008_d_n6;
        var_tmp_dn7 = assign25860_e30008_d_n7;
        var_tmp_dn8 = assign25860_e30008_d_n8;

        let (assign25870_e30026, assign25870_e30026_d_n5, assign25870_e30026_d_n6, assign25870_e30026_d_n7, assign25870_e30026_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard485 == 0.0)) && (var_guard486 != 0.0)) {
        let assign25870_e30023: f64 = (1.0 - var_tmp);
        let assign25870_e30024: f64 = (1.0 / assign25870_e30023);
        (assign25870_e30024, (-((-var_tmp_dn5) / (assign25870_e30023 * assign25870_e30023))), (-((-var_tmp_dn6) / (assign25870_e30023 * assign25870_e30023))), (-((-var_tmp_dn7) / (assign25870_e30023 * assign25870_e30023))), (-((-var_tmp_dn8) / (assign25870_e30023 * assign25870_e30023))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign25870_e30026;
        var_fbreakdown_dn5 = assign25870_e30026_d_n5;
        var_fbreakdown_dn6 = assign25870_e30026_d_n6;
        var_fbreakdown_dn7 = assign25870_e30026_d_n7;
        var_fbreakdown_dn8 = assign25870_e30026_d_n8;

        let (assign25880_e30049, assign25880_e30049_d_n5, assign25880_e30049_d_n6, assign25880_e30049_d_n7, assign25880_e30049_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) && (var_guard485 == 0.0)) && (var_guard486 == 0.0)) {
        let assign25880_e30043: f64 = (var_alphaav * p.p853);
        let assign25880_e30044: f64 = (var_vav + assign25880_e30043);
        let assign25880_e30046: f64 = (assign25880_e30044 * var_slopebot);
        let assign25880_e30047: f64 = (var_fstopbot + assign25880_e30046);
        (assign25880_e30047, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign25880_e30049;
        var_fbreakdown_dn5 = assign25880_e30049_d_n5;
        var_fbreakdown_dn6 = assign25880_e30049_d_n6;
        var_fbreakdown_dn7 = assign25880_e30049_d_n7;
        var_fbreakdown_dn8 = assign25880_e30049_d_n8;

        let (assign25890_e30068, assign25890_e30068_d_n5, assign25890_e30068_d_n6, assign25890_e30068_d_n7, assign25890_e30068_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard471 == 0.0)) {
        let assign25890_e30059: f64 = (var_id__blk213 + var_isrh);
        let assign25890_e30061: f64 = (assign25890_e30059 + var_itat);
        let assign25890_e30063: f64 = (assign25890_e30061 + var_ibbt);
        let assign25890_e30064: f64 = (p.p29 * assign25890_e30063);
        let assign25890_e30066: f64 = (assign25890_e30064 * var_fbreakdown);
        (assign25890_e30066, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign25890_e30064 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign25890_e30064 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign25890_e30064 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign25890_e30064 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign25890_e30068;
        var_ijunbot_dn5 = assign25890_e30068_d_n5;
        var_ijunbot_dn6 = assign25890_e30068_d_n6;
        var_ijunbot_dn7 = assign25890_e30068_d_n7;
        var_ijunbot_dn8 = assign25890_e30068_d_n8;

        let assign25900_e30071: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard488 = assign25900_e30071;

        let (assign25910_e30079, assign25910_e30079_d_n5, assign25910_e30079_d_n6, assign25910_e30079_d_n7, assign25910_e30079_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign25910_e30079;
        var_ijunsti_dn5 = assign25910_e30079_d_n5;
        var_ijunsti_dn6 = assign25910_e30079_d_n6;
        var_ijunsti_dn7 = assign25910_e30079_d_n7;
        var_ijunsti_dn8 = assign25910_e30079_d_n8;

        let (assign25920_e30090,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) {
        let assign25920_e30088: f64 = (var_idsatsti * var_idmult);
        (assign25920_e30088,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign25920_e30090;

        let assign25930_e30097: f64 = if ((p.p834 == 0.0) && (p.p839 == 0.0)) { 1.0 } else { 0.0 };
        var_guard489 = assign25930_e30097;

        let (assign25940_e30108, assign25940_e30108_d_n5, assign25940_e30108_d_n6, assign25940_e30108_d_n7, assign25940_e30108_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard489 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign25940_e30108;
        var_isrh_dn5 = assign25940_e30108_d_n5;
        var_isrh_dn6 = assign25940_e30108_d_n6;
        var_isrh_dn7 = assign25940_e30108_d_n7;
        var_isrh_dn8 = assign25940_e30108_d_n8;

        let (assign25950_e30122,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard489 == 0.0)) {
        let assign25950_e30120: f64 = (var_vbisti - var_vjsrh);
        (assign25950_e30120,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign25950_e30122;

        let (assign25960_e30141,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard489 == 0.0)) {
        let assign25960_e30136: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign25960_e30137: f64 = (1.0 - assign25960_e30136);
        let assign25960_e30138: f64 = (assign25960_e30137).sqrt();
        let assign25960_e30139: f64 = (1.0 - assign25960_e30138);
        (assign25960_e30139,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign25960_e30141;

        let assign25970_e30144: f64 = if p.p825 == 0.5 { 1.0 } else { 0.0 };
        var_guard490 = assign25970_e30144;

        let (assign25980_e30158,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard489 == 0.0)) && (var_guard490 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign25980_e30158;

        let (assign25990_e30190,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard489 == 0.0)) && (var_guard490 == 0.0)) {
        let assign25990_e30173: f64 = (var_wsrhstep * var_wsrhstep);
        let assign25990_e30175: f64 = (var_wsrhstep).ln();
        let assign25990_e30176: f64 = (assign25990_e30173 * assign25990_e30175);
        let assign25990_e30179: f64 = (1.0 - var_wsrhstep);
        let assign25990_e30180: f64 = (assign25990_e30176 / assign25990_e30179);
        let assign25990_e30182: f64 = (assign25990_e30180 + var_wsrhstep);
        let assign25990_e30186: f64 = (2.0 * p.p825);
        let assign25990_e30187: f64 = (1.0 - assign25990_e30186);
        let assign25990_e30188: f64 = (assign25990_e30182 * assign25990_e30187);
        (assign25990_e30188,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign25990_e30190;

        let (assign26000_e30204,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard489 == 0.0)) {
        let assign26000_e30202: f64 = (var_wsrhstep + var_dwsrh);
        (assign26000_e30202,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign26000_e30204;

        let assign26010_e30207: f64 = if p.p825 == 0.5 { 1.0 } else { 0.0 };
        var_guard491 = assign26010_e30207;

        let (assign26020_e30224, assign26020_e30224_d_n5, assign26020_e30224_d_n6, assign26020_e30224_d_n7, assign26020_e30224_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard489 == 0.0)) && (var_guard491 != 0.0)) {
        let assign26020_e30221: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign26020_e30222: f64 = (assign26020_e30221).sqrt();
        (assign26020_e30222, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26020_e30224;
        var_tmp_dn5 = assign26020_e30224_d_n5;
        var_tmp_dn6 = assign26020_e30224_d_n6;
        var_tmp_dn7 = assign26020_e30224_d_n7;
        var_tmp_dn8 = assign26020_e30224_d_n8;

        let (assign26030_e30243, assign26030_e30243_d_n5, assign26030_e30243_d_n6, assign26030_e30243_d_n7, assign26030_e30243_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard489 == 0.0)) && (var_guard491 == 0.0)) {
        let assign26030_e30239: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign26030_e30241: f64 = (assign26030_e30239).powf(p.p825);
        (assign26030_e30241, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26030_e30243;
        var_tmp_dn5 = assign26030_e30243_d_n5;
        var_tmp_dn6 = assign26030_e30243_d_n6;
        var_tmp_dn7 = assign26030_e30243_d_n7;
        var_tmp_dn8 = assign26030_e30243_d_n8;

        let (assign26040_e30257, assign26040_e30257_d_n5, assign26040_e30257_d_n6, assign26040_e30257_d_n7, assign26040_e30257_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard489 == 0.0)) {
        let assign26040_e30255: f64 = (var_wdepnulrsti * var_tmp);
        (assign26040_e30255, (var_wdepnulrsti * var_tmp_dn5), (var_wdepnulrsti * var_tmp_dn6), (var_wdepnulrsti * var_tmp_dn7), (var_wdepnulrsti * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign26040_e30257;
        var_wdep_dn5 = assign26040_e30257_d_n5;
        var_wdep_dn6 = assign26040_e30257_d_n6;
        var_wdep_dn7 = assign26040_e30257_d_n7;
        var_wdep_dn8 = assign26040_e30257_d_n8;

        let (assign26050_e30275, assign26050_e30275_d_n5, assign26050_e30275_d_n6, assign26050_e30275_d_n7, assign26050_e30275_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard489 == 0.0)) {
        let assign26050_e30270: f64 = (var_zinv - 1.0);
        let assign26050_e30272: f64 = (assign26050_e30270 * var_wdep);
        let assign26050_e30273: f64 = (var_ftdsti * assign26050_e30272);
        (assign26050_e30273, (var_ftdsti * (assign26050_e30270 * var_wdep_dn5)), (var_ftdsti * (assign26050_e30270 * var_wdep_dn6)), (var_ftdsti * (assign26050_e30270 * var_wdep_dn7)), (var_ftdsti * (assign26050_e30270 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign26050_e30275;
        var_asrh_dn5 = assign26050_e30275_d_n5;
        var_asrh_dn6 = assign26050_e30275_d_n6;
        var_asrh_dn7 = assign26050_e30275_d_n7;
        var_asrh_dn8 = assign26050_e30275_d_n8;

        let (assign26060_e30291, assign26060_e30291_d_n5, assign26060_e30291_d_n6, assign26060_e30291_d_n7, assign26060_e30291_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard489 == 0.0)) {
        let assign26060_e30288: f64 = (var_asrh * var_wsrh);
        let assign26060_e30289: f64 = (p.p834 * assign26060_e30288);
        (assign26060_e30289, (p.p834 * (var_asrh_dn5 * var_wsrh)), (p.p834 * (var_asrh_dn6 * var_wsrh)), (p.p834 * (var_asrh_dn7 * var_wsrh)), (p.p834 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign26060_e30291;
        var_isrh_dn5 = assign26060_e30291_d_n5;
        var_isrh_dn6 = assign26060_e30291_d_n6;
        var_isrh_dn7 = assign26060_e30291_d_n7;
        var_isrh_dn8 = assign26060_e30291_d_n8;

        let assign26070_e30294: f64 = if p.p839 == 0.0 { 1.0 } else { 0.0 };
        var_guard492 = assign26070_e30294;

        let (assign26080_e30305, assign26080_e30305_d_n5, assign26080_e30305_d_n6, assign26080_e30305_d_n7, assign26080_e30305_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign26080_e30305;
        var_itat_dn5 = assign26080_e30305_d_n5;
        var_itat_dn6 = assign26080_e30305_d_n6;
        var_itat_dn7 = assign26080_e30305_d_n7;
        var_itat_dn8 = assign26080_e30305_d_n8;

        let (assign26090_e30323, assign26090_e30323_d_n5, assign26090_e30323_d_n6, assign26090_e30323_d_n7, assign26090_e30323_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) {
        let assign26090_e30318: f64 = (var_wdep * var_one_minus_psti);
        let assign26090_e30320: f64 = (assign26090_e30318 / var_vbi_minus_vjsrh);
        let assign26090_e30321: f64 = (var_btatpartsti * assign26090_e30320);
        (assign26090_e30321, (var_btatpartsti * ((var_wdep_dn5 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn6 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn7 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn8 * var_one_minus_psti) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign26090_e30323;
        var_btat_dn5 = assign26090_e30323_d_n5;
        var_btat_dn6 = assign26090_e30323_d_n6;
        var_btat_dn7 = assign26090_e30323_d_n7;
        var_btat_dn8 = assign26090_e30323_d_n8;

        let (assign26100_e30339, assign26100_e30339_d_n5, assign26100_e30339_d_n6, assign26100_e30339_d_n7, assign26100_e30339_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) {
        let assign26100_e30335: f64 = (0.666666666666667 * var_atatsti);
        let assign26100_e30337: f64 = (assign26100_e30335 / var_btat);
        (assign26100_e30337, (-((assign26100_e30335 * var_btat_dn5) / (var_btat * var_btat))), (-((assign26100_e30335 * var_btat_dn6) / (var_btat * var_btat))), (-((assign26100_e30335 * var_btat_dn7) / (var_btat * var_btat))), (-((assign26100_e30335 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign26100_e30339;
        var_twoatatoverthreebtat_dn5 = assign26100_e30339_d_n5;
        var_twoatatoverthreebtat_dn6 = assign26100_e30339_d_n6;
        var_twoatatoverthreebtat_dn7 = assign26100_e30339_d_n7;
        var_twoatatoverthreebtat_dn8 = assign26100_e30339_d_n8;

        let (assign26110_e30353, assign26110_e30353_d_n5, assign26110_e30353_d_n6, assign26110_e30353_d_n7, assign26110_e30353_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) {
        let assign26110_e30351: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign26110_e30351, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign26110_e30353;
        var_umaxbeforelimiting_dn5 = assign26110_e30353_d_n5;
        var_umaxbeforelimiting_dn6 = assign26110_e30353_d_n6;
        var_umaxbeforelimiting_dn7 = assign26110_e30353_d_n7;
        var_umaxbeforelimiting_dn8 = assign26110_e30353_d_n8;

        let (assign26120_e30374, assign26120_e30374_d_n5, assign26120_e30374_d_n6, assign26120_e30374_d_n7, assign26120_e30374_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) {
        let assign26120_e30365: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign26120_e30368: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign26120_e30370: f64 = (assign26120_e30368 + 1.0);
        let assign26120_e30371: f64 = (assign26120_e30365 / assign26120_e30370);
        let assign26120_e30372: f64 = (assign26120_e30371).sqrt();
        (assign26120_e30372, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign26120_e30370) - (assign26120_e30365 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign26120_e30370 * assign26120_e30370)) / (2.0 * assign26120_e30372)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign26120_e30370) - (assign26120_e30365 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign26120_e30370 * assign26120_e30370)) / (2.0 * assign26120_e30372)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign26120_e30370) - (assign26120_e30365 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign26120_e30370 * assign26120_e30370)) / (2.0 * assign26120_e30372)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign26120_e30370) - (assign26120_e30365 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign26120_e30370 * assign26120_e30370)) / (2.0 * assign26120_e30372)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign26120_e30374;
        var_umax_dn5 = assign26120_e30374_d_n5;
        var_umax_dn6 = assign26120_e30374_d_n6;
        var_umax_dn7 = assign26120_e30374_d_n7;
        var_umax_dn8 = assign26120_e30374_d_n8;

        let (assign26130_e30387, assign26130_e30387_d_n5, assign26130_e30387_d_n6, assign26130_e30387_d_n7, assign26130_e30387_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) {
        let assign26130_e30385: f64 = (var_umax).sqrt();
        (assign26130_e30385, (var_umax_dn5 / (2.0 * assign26130_e30385)), (var_umax_dn6 / (2.0 * assign26130_e30385)), (var_umax_dn7 / (2.0 * assign26130_e30385)), (var_umax_dn8 / (2.0 * assign26130_e30385)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign26130_e30387;
        var_sqrtumax_dn5 = assign26130_e30387_d_n5;
        var_sqrtumax_dn6 = assign26130_e30387_d_n6;
        var_sqrtumax_dn7 = assign26130_e30387_d_n7;
        var_sqrtumax_dn8 = assign26130_e30387_d_n8;

        let (assign26140_e30401, assign26140_e30401_d_n5, assign26140_e30401_d_n6, assign26140_e30401_d_n7, assign26140_e30401_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) {
        let assign26140_e30399: f64 = (var_umax * var_sqrtumax);
        (assign26140_e30399, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign26140_e30401;
        var_umaxpoweronepointfive_dn5 = assign26140_e30401_d_n5;
        var_umaxpoweronepointfive_dn6 = assign26140_e30401_d_n6;
        var_umaxpoweronepointfive_dn7 = assign26140_e30401_d_n7;
        var_umaxpoweronepointfive_dn8 = assign26140_e30401_d_n8;

        let assign26150_e30403: f64 = (-p.p825);
        let assign26150_e30405: f64 = (assign26150_e30403 * var_one_over_one_minus_psti);
        let assign26150_e30407: f64 = (-1.0);
        let assign26150_e30408: f64 = if assign26150_e30405 == assign26150_e30407 { 1.0 } else { 0.0 };
        var_guard493 = assign26150_e30408;

        let (assign26160_e30428, assign26160_e30428_d_n5, assign26160_e30428_d_n6, assign26160_e30428_d_n7, assign26160_e30428_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) && (var_guard493 != 0.0)) {
        let assign26160_e30424: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign26160_e30425: f64 = (1.0 + assign26160_e30424);
        let assign26160_e30426: f64 = (1.0 / assign26160_e30425);
        (assign26160_e30426, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign26160_e30425 * assign26160_e30425))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign26160_e30425 * assign26160_e30425))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign26160_e30425 * assign26160_e30425))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign26160_e30425 * assign26160_e30425))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign26160_e30428;
        var_wgamma_dn5 = assign26160_e30428_d_n5;
        var_wgamma_dn6 = assign26160_e30428_d_n6;
        var_wgamma_dn7 = assign26160_e30428_d_n7;
        var_wgamma_dn8 = assign26160_e30428_d_n8;

        let (assign26170_e30452, assign26170_e30452_d_n5, assign26170_e30452_d_n6, assign26170_e30452_d_n7, assign26170_e30452_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) && (var_guard493 == 0.0)) {
        let assign26170_e30444: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign26170_e30445: f64 = (1.0 + assign26170_e30444);
        let assign26170_e30447: f64 = (-p.p825);
        let assign26170_e30449: f64 = (assign26170_e30447 * var_one_over_one_minus_psti);
        let assign26170_e30450: f64 = (assign26170_e30445).powf(assign26170_e30449);
        (assign26170_e30450, if 0.0 == 0.0 && ((assign26170_e30449) as f64).is_finite() && ((assign26170_e30449) as f64).fract() == 0.0 { if assign26170_e30449 == 0.0 { 0.0 } else { (assign26170_e30449 * ((assign26170_e30445).powf(assign26170_e30449 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign26170_e30450 * (assign26170_e30449 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign26170_e30445))) }, if 0.0 == 0.0 && ((assign26170_e30449) as f64).is_finite() && ((assign26170_e30449) as f64).fract() == 0.0 { if assign26170_e30449 == 0.0 { 0.0 } else { (assign26170_e30449 * ((assign26170_e30445).powf(assign26170_e30449 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign26170_e30450 * (assign26170_e30449 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign26170_e30445))) }, if 0.0 == 0.0 && ((assign26170_e30449) as f64).is_finite() && ((assign26170_e30449) as f64).fract() == 0.0 { if assign26170_e30449 == 0.0 { 0.0 } else { (assign26170_e30449 * ((assign26170_e30445).powf(assign26170_e30449 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign26170_e30450 * (assign26170_e30449 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign26170_e30445))) }, if 0.0 == 0.0 && ((assign26170_e30449) as f64).is_finite() && ((assign26170_e30449) as f64).fract() == 0.0 { if assign26170_e30449 == 0.0 { 0.0 } else { (assign26170_e30449 * ((assign26170_e30445).powf(assign26170_e30449 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign26170_e30450 * (assign26170_e30449 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign26170_e30445))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign26170_e30452;
        var_wgamma_dn5 = assign26170_e30452_d_n5;
        var_wgamma_dn6 = assign26170_e30452_d_n6;
        var_wgamma_dn7 = assign26170_e30452_d_n7;
        var_wgamma_dn8 = assign26170_e30452_d_n8;

        let (assign26180_e30470, assign26180_e30470_d_n5, assign26180_e30470_d_n6, assign26180_e30470_d_n7, assign26180_e30470_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) {
        let assign26180_e30464: f64 = (var_wsrh * var_wgamma);
        let assign26180_e30467: f64 = (var_wsrh + var_wgamma);
        let assign26180_e30468: f64 = (assign26180_e30464 / assign26180_e30467);
        (assign26180_e30468, ((((var_wsrh * var_wgamma_dn5) * assign26180_e30467) - (assign26180_e30464 * var_wgamma_dn5)) / (assign26180_e30467 * assign26180_e30467)), ((((var_wsrh * var_wgamma_dn6) * assign26180_e30467) - (assign26180_e30464 * var_wgamma_dn6)) / (assign26180_e30467 * assign26180_e30467)), ((((var_wsrh * var_wgamma_dn7) * assign26180_e30467) - (assign26180_e30464 * var_wgamma_dn7)) / (assign26180_e30467 * assign26180_e30467)), ((((var_wsrh * var_wgamma_dn8) * assign26180_e30467) - (assign26180_e30464 * var_wgamma_dn8)) / (assign26180_e30467 * assign26180_e30467)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign26180_e30470;
        var_wtat_dn5 = assign26180_e30470_d_n5;
        var_wtat_dn6 = assign26180_e30470_d_n6;
        var_wtat_dn7 = assign26180_e30470_d_n7;
        var_wtat_dn8 = assign26180_e30470_d_n8;

        let (assign26190_e30487, assign26190_e30487_d_n5, assign26190_e30487_d_n6, assign26190_e30487_d_n7, assign26190_e30487_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) {
        let assign26190_e30483: f64 = (var_btat / var_sqrtumax);
        let assign26190_e30484: f64 = (0.375 * assign26190_e30483);
        let assign26190_e30485: f64 = (assign26190_e30484).sqrt();
        (assign26190_e30485, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign26190_e30485)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign26190_e30485)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign26190_e30485)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign26190_e30485)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign26190_e30487;
        var_ktat_dn5 = assign26190_e30487_d_n5;
        var_ktat_dn6 = assign26190_e30487_d_n6;
        var_ktat_dn7 = assign26190_e30487_d_n7;
        var_ktat_dn8 = assign26190_e30487_d_n8;

        let (assign26200_e30505, assign26200_e30505_d_n5, assign26200_e30505_d_n6, assign26200_e30505_d_n7, assign26200_e30505_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) {
        let assign26200_e30500: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign26200_e30501: f64 = (2.0 * assign26200_e30500);
        let assign26200_e30503: f64 = (assign26200_e30501 - var_umax);
        (assign26200_e30503, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign26200_e30505;
        var_ltat_dn5 = assign26200_e30505_d_n5;
        var_ltat_dn6 = assign26200_e30505_d_n6;
        var_ltat_dn7 = assign26200_e30505_d_n7;
        var_ltat_dn8 = assign26200_e30505_d_n8;

        let (assign26210_e30531, assign26210_e30531_d_n5, assign26210_e30531_d_n6, assign26210_e30531_d_n7, assign26210_e30531_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) {
        let assign26210_e30517: f64 = (var_atatsti * var_twoatatoverthreebtat);
        let assign26210_e30519: f64 = (assign26210_e30517 * var_sqrtumax);
        let assign26210_e30522: f64 = (var_atatsti * var_umax);
        let assign26210_e30523: f64 = (assign26210_e30519 - assign26210_e30522);
        let assign26210_e30527: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign26210_e30528: f64 = (0.5 * assign26210_e30527);
        let assign26210_e30529: f64 = (assign26210_e30523 + assign26210_e30528);
        (assign26210_e30529, (((((var_atatsti * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign26210_e30517 * var_sqrtumax_dn5)) - (var_atatsti * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatsti * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign26210_e30517 * var_sqrtumax_dn6)) - (var_atatsti * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign26210_e30517 * var_sqrtumax_dn7)) - (var_atatsti * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign26210_e30517 * var_sqrtumax_dn8)) - (var_atatsti * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign26210_e30531;
        var_mtat_dn5 = assign26210_e30531_d_n5;
        var_mtat_dn6 = assign26210_e30531_d_n6;
        var_mtat_dn7 = assign26210_e30531_d_n7;
        var_mtat_dn8 = assign26210_e30531_d_n8;

        let (assign26220_e30547, assign26220_e30547_d_n5, assign26220_e30547_d_n6, assign26220_e30547_d_n7, assign26220_e30547_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) {
        let assign26220_e30543: f64 = (var_ltat - 1.0);
        let assign26220_e30545: f64 = (assign26220_e30543 * var_ktat);
        (assign26220_e30545, ((var_ltat_dn5 * var_ktat) + (assign26220_e30543 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign26220_e30543 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign26220_e30543 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign26220_e30543 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign26220_e30547;
        var_xerfc_dn5 = assign26220_e30547_d_n5;
        var_xerfc_dn6 = assign26220_e30547_d_n6;
        var_xerfc_dn7 = assign26220_e30547_d_n7;
        var_xerfc_dn8 = assign26220_e30547_d_n8;

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
        *var_guard486_slot = var_guard486;
        *var_guard487_slot = var_guard487;
        *var_guard488_slot = var_guard488;
        *var_guard489_slot = var_guard489;
        *var_guard490_slot = var_guard490;
        *var_guard491_slot = var_guard491;
        *var_guard492_slot = var_guard492;
        *var_guard493_slot = var_guard493;
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
    }

    pub(super) fn stamp_transient_block_50(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatsti: f64,
        var_berfc: f64,
        var_cerfc: f64,
        var_fbbtsti: f64,
        var_fstopsti: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard488: f64,
        var_guard492: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_mtat: f64,
        var_mtat_dn5: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_one_over_one_minus_psti: f64,
        var_perfc: f64,
        var_slopesti: f64,
        var_v5: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirstiinv: f64,
        var_vbrinvsti: f64,
        var_wdepnulrinvsti: f64,
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
        var_guard494_slot: &mut f64,
        var_guard495_slot: &mut f64,
        var_guard496_slot: &mut f64,
        var_guard497_slot: &mut f64,
        var_guard498_slot: &mut f64,
        var_guard499_slot: &mut f64,
        var_guard500_slot: &mut f64,
        var_guard501_slot: &mut f64,
        var_guard502_slot: &mut f64,
        var_guard503_slot: &mut f64,
        var_guard504_slot: &mut f64,
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
        let mut var_guard494: f64 = *var_guard494_slot;
        let mut var_guard495: f64 = *var_guard495_slot;
        let mut var_guard496: f64 = *var_guard496_slot;
        let mut var_guard497: f64 = *var_guard497_slot;
        let mut var_guard498: f64 = *var_guard498_slot;
        let mut var_guard499: f64 = *var_guard499_slot;
        let mut var_guard500: f64 = *var_guard500_slot;
        let mut var_guard501: f64 = *var_guard501_slot;
        let mut var_guard502: f64 = *var_guard502_slot;
        let mut var_guard503: f64 = *var_guard503_slot;
        let mut var_guard504: f64 = *var_guard504_slot;
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
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn5: f64 = *var_ysq_dn5_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;

        let (assign26230_e30561, assign26230_e30561_d_n5, assign26230_e30561_d_n6, assign26230_e30561_d_n7, assign26230_e30561_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) {
        let assign26230_e30559: f64 = (var_xerfc * var_xerfc);
        (assign26230_e30559, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign26230_e30561;
        var_ysq_dn5 = assign26230_e30561_d_n5;
        var_ysq_dn6 = assign26230_e30561_d_n6;
        var_ysq_dn7 = assign26230_e30561_d_n7;
        var_ysq_dn8 = assign26230_e30561_d_n8;

        let assign26240_e30564: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard494 = assign26240_e30564;

        let (assign26250_e30584, assign26250_e30584_d_n5, assign26250_e30584_d_n6, assign26250_e30584_d_n7, assign26250_e30584_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) && (var_guard494 != 0.0)) {
        let assign26250_e30580: f64 = (var_perfc * var_xerfc);
        let assign26250_e30581: f64 = (1.0 + assign26250_e30580);
        let assign26250_e30582: f64 = (1.0 / assign26250_e30581);
        (assign26250_e30582, (-((var_perfc * var_xerfc_dn5) / (assign26250_e30581 * assign26250_e30581))), (-((var_perfc * var_xerfc_dn6) / (assign26250_e30581 * assign26250_e30581))), (-((var_perfc * var_xerfc_dn7) / (assign26250_e30581 * assign26250_e30581))), (-((var_perfc * var_xerfc_dn8) / (assign26250_e30581 * assign26250_e30581))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign26250_e30584;
        var_terfc_dn5 = assign26250_e30584_d_n5;
        var_terfc_dn6 = assign26250_e30584_d_n6;
        var_terfc_dn7 = assign26250_e30584_d_n7;
        var_terfc_dn8 = assign26250_e30584_d_n8;

        let (assign26260_e30605, assign26260_e30605_d_n5, assign26260_e30605_d_n6, assign26260_e30605_d_n7, assign26260_e30605_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) && (var_guard494 == 0.0)) {
        let assign26260_e30601: f64 = (var_perfc * var_xerfc);
        let assign26260_e30602: f64 = (1.0 - assign26260_e30601);
        let assign26260_e30603: f64 = (1.0 / assign26260_e30602);
        (assign26260_e30603, (-((-(var_perfc * var_xerfc_dn5)) / (assign26260_e30602 * assign26260_e30602))), (-((-(var_perfc * var_xerfc_dn6)) / (assign26260_e30602 * assign26260_e30602))), (-((-(var_perfc * var_xerfc_dn7)) / (assign26260_e30602 * assign26260_e30602))), (-((-(var_perfc * var_xerfc_dn8)) / (assign26260_e30602 * assign26260_e30602))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign26260_e30605;
        var_terfc_dn5 = assign26260_e30605_d_n5;
        var_terfc_dn6 = assign26260_e30605_d_n6;
        var_terfc_dn7 = assign26260_e30605_d_n7;
        var_terfc_dn8 = assign26260_e30605_d_n8;

        let assign26270_e30607: f64 = (-var_ysq);
        let assign26270_e30609: f64 = (assign26270_e30607 + var_mtat);
        let assign26270_e30611: f64 = (-230.25850929940458);
        let assign26270_e30612: f64 = if assign26270_e30609 > assign26270_e30611 { 1.0 } else { 0.0 };
        var_guard495 = assign26270_e30612;

        let (assign26280_e30630, assign26280_e30630_d_n5, assign26280_e30630_d_n6, assign26280_e30630_d_n7, assign26280_e30630_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) && (var_guard495 != 0.0)) {
        let assign26280_e30625: f64 = (-var_ysq);
        let assign26280_e30627: f64 = (assign26280_e30625 + var_mtat);
        let assign26280_e30628: f64 = (assign26280_e30627).exp();
        (assign26280_e30628, (assign26280_e30628 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign26280_e30628 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign26280_e30628 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign26280_e30628 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26280_e30630;
        var_tmp_dn5 = assign26280_e30630_d_n5;
        var_tmp_dn6 = assign26280_e30630_d_n6;
        var_tmp_dn7 = assign26280_e30630_d_n7;
        var_tmp_dn8 = assign26280_e30630_d_n8;

        let (assign26290_e30679, assign26290_e30679_d_n5, assign26290_e30679_d_n6, assign26290_e30679_d_n7, assign26290_e30679_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) && (var_guard495 == 0.0)) {
        let assign26290_e30646: f64 = (-230.25850929940458);
        let assign26290_e30648: f64 = (-var_ysq);
        let assign26290_e30650: f64 = (assign26290_e30648 + var_mtat);
        let assign26290_e30651: f64 = (assign26290_e30646 - assign26290_e30650);
        let assign26290_e30655: f64 = (-230.25850929940458);
        let assign26290_e30657: f64 = (-var_ysq);
        let assign26290_e30659: f64 = (assign26290_e30657 + var_mtat);
        let assign26290_e30660: f64 = (assign26290_e30655 - assign26290_e30659);
        let assign26290_e30663: f64 = (-230.25850929940458);
        let assign26290_e30665: f64 = (-var_ysq);
        let assign26290_e30667: f64 = (assign26290_e30665 + var_mtat);
        let assign26290_e30668: f64 = (assign26290_e30663 - assign26290_e30667);
        let assign26290_e30670: f64 = (assign26290_e30668 * 0.3333333333333333);
        let assign26290_e30671: f64 = (1.0 + assign26290_e30670);
        let assign26290_e30672: f64 = (assign26290_e30660 * assign26290_e30671);
        let assign26290_e30673: f64 = (0.5 * assign26290_e30672);
        let assign26290_e30674: f64 = (1.0 + assign26290_e30673);
        let assign26290_e30675: f64 = (assign26290_e30651 * assign26290_e30674);
        let assign26290_e30676: f64 = (1.0 + assign26290_e30675);
        let assign26290_e30677: f64 = (1e-100 / assign26290_e30676);
        (assign26290_e30677, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign26290_e30674) + (assign26290_e30651 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign26290_e30671) + (assign26290_e30660 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign26290_e30676 * assign26290_e30676))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign26290_e30674) + (assign26290_e30651 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign26290_e30671) + (assign26290_e30660 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign26290_e30676 * assign26290_e30676))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign26290_e30674) + (assign26290_e30651 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign26290_e30671) + (assign26290_e30660 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign26290_e30676 * assign26290_e30676))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign26290_e30674) + (assign26290_e30651 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign26290_e30671) + (assign26290_e30660 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign26290_e30676 * assign26290_e30676))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26290_e30679;
        var_tmp_dn5 = assign26290_e30679_d_n5;
        var_tmp_dn6 = assign26290_e30679_d_n6;
        var_tmp_dn7 = assign26290_e30679_d_n7;
        var_tmp_dn8 = assign26290_e30679_d_n8;

        let (assign26300_e30709, assign26300_e30709_d_n5, assign26300_e30709_d_n6, assign26300_e30709_d_n7, assign26300_e30709_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) {
        let assign26300_e30691: f64 = (0.29214664 * var_terfc);
        let assign26300_e30695: f64 = (var_terfc * var_terfc);
        let assign26300_e30696: f64 = (var_berfc * assign26300_e30695);
        let assign26300_e30697: f64 = (assign26300_e30691 + assign26300_e30696);
        let assign26300_e30701: f64 = (var_terfc * var_terfc);
        let assign26300_e30703: f64 = (assign26300_e30701 * var_terfc);
        let assign26300_e30704: f64 = (var_cerfc * assign26300_e30703);
        let assign26300_e30705: f64 = (assign26300_e30697 + assign26300_e30704);
        let assign26300_e30707: f64 = (assign26300_e30705 * var_tmp);
        (assign26300_e30707, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign26300_e30701 * var_terfc_dn5)))) * var_tmp) + (assign26300_e30705 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign26300_e30701 * var_terfc_dn6)))) * var_tmp) + (assign26300_e30705 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign26300_e30701 * var_terfc_dn7)))) * var_tmp) + (assign26300_e30705 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign26300_e30701 * var_terfc_dn8)))) * var_tmp) + (assign26300_e30705 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign26300_e30709;
        var_erfcpos_dn5 = assign26300_e30709_d_n5;
        var_erfcpos_dn6 = assign26300_e30709_d_n6;
        var_erfcpos_dn7 = assign26300_e30709_d_n7;
        var_erfcpos_dn8 = assign26300_e30709_d_n8;

        let assign26310_e30712: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard496 = assign26310_e30712;

        let (assign26320_e30726, assign26320_e30726_d_n5, assign26320_e30726_d_n6, assign26320_e30726_d_n7, assign26320_e30726_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) && (var_guard496 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign26320_e30726;
        var_erfctimesexpmtat_dn5 = assign26320_e30726_d_n5;
        var_erfctimesexpmtat_dn6 = assign26320_e30726_d_n6;
        var_erfctimesexpmtat_dn7 = assign26320_e30726_d_n7;
        var_erfctimesexpmtat_dn8 = assign26320_e30726_d_n8;

        let assign26330_e30729: f64 = (-230.25850929940458);
        let assign26330_e30730: f64 = if var_mtat > assign26330_e30729 { 1.0 } else { 0.0 };
        var_guard497 = assign26330_e30730;

        let (assign26340_e30748, assign26340_e30748_d_n5, assign26340_e30748_d_n6, assign26340_e30748_d_n7, assign26340_e30748_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) && (var_guard496 == 0.0)) && (var_guard497 != 0.0)) {
        let assign26340_e30746: f64 = (var_mtat).exp();
        (assign26340_e30746, (assign26340_e30746 * var_mtat_dn5), (assign26340_e30746 * var_mtat_dn6), (assign26340_e30746 * var_mtat_dn7), (assign26340_e30746 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26340_e30748;
        var_tmp_dn5 = assign26340_e30748_d_n5;
        var_tmp_dn6 = assign26340_e30748_d_n6;
        var_tmp_dn7 = assign26340_e30748_d_n7;
        var_tmp_dn8 = assign26340_e30748_d_n8;

        let (assign26350_e30791, assign26350_e30791_d_n5, assign26350_e30791_d_n6, assign26350_e30791_d_n7, assign26350_e30791_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) && (var_guard496 == 0.0)) && (var_guard497 == 0.0)) {
        let assign26350_e30767: f64 = (-230.25850929940458);
        let assign26350_e30769: f64 = (assign26350_e30767 - var_mtat);
        let assign26350_e30773: f64 = (-230.25850929940458);
        let assign26350_e30775: f64 = (assign26350_e30773 - var_mtat);
        let assign26350_e30778: f64 = (-230.25850929940458);
        let assign26350_e30780: f64 = (assign26350_e30778 - var_mtat);
        let assign26350_e30782: f64 = (assign26350_e30780 * 0.3333333333333333);
        let assign26350_e30783: f64 = (1.0 + assign26350_e30782);
        let assign26350_e30784: f64 = (assign26350_e30775 * assign26350_e30783);
        let assign26350_e30785: f64 = (0.5 * assign26350_e30784);
        let assign26350_e30786: f64 = (1.0 + assign26350_e30785);
        let assign26350_e30787: f64 = (assign26350_e30769 * assign26350_e30786);
        let assign26350_e30788: f64 = (1.0 + assign26350_e30787);
        let assign26350_e30789: f64 = (1e-100 / assign26350_e30788);
        (assign26350_e30789, (-((1e-100 * (((-var_mtat_dn5) * assign26350_e30786) + (assign26350_e30769 * (0.5 * (((-var_mtat_dn5) * assign26350_e30783) + (assign26350_e30775 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign26350_e30788 * assign26350_e30788))), (-((1e-100 * (((-var_mtat_dn6) * assign26350_e30786) + (assign26350_e30769 * (0.5 * (((-var_mtat_dn6) * assign26350_e30783) + (assign26350_e30775 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign26350_e30788 * assign26350_e30788))), (-((1e-100 * (((-var_mtat_dn7) * assign26350_e30786) + (assign26350_e30769 * (0.5 * (((-var_mtat_dn7) * assign26350_e30783) + (assign26350_e30775 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign26350_e30788 * assign26350_e30788))), (-((1e-100 * (((-var_mtat_dn8) * assign26350_e30786) + (assign26350_e30769 * (0.5 * (((-var_mtat_dn8) * assign26350_e30783) + (assign26350_e30775 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign26350_e30788 * assign26350_e30788))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26350_e30791;
        var_tmp_dn5 = assign26350_e30791_d_n5;
        var_tmp_dn6 = assign26350_e30791_d_n6;
        var_tmp_dn7 = assign26350_e30791_d_n7;
        var_tmp_dn8 = assign26350_e30791_d_n8;

        let (assign26360_e30810, assign26360_e30810_d_n5, assign26360_e30810_d_n6, assign26360_e30810_d_n7, assign26360_e30810_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) && (var_guard496 == 0.0)) {
        let assign26360_e30806: f64 = (2.0 * var_tmp);
        let assign26360_e30808: f64 = (assign26360_e30806 - var_erfcpos);
        (assign26360_e30808, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign26360_e30810;
        var_erfctimesexpmtat_dn5 = assign26360_e30810_d_n5;
        var_erfctimesexpmtat_dn6 = assign26360_e30810_d_n6;
        var_erfctimesexpmtat_dn7 = assign26360_e30810_d_n7;
        var_erfctimesexpmtat_dn8 = assign26360_e30810_d_n8;

        let (assign26370_e30830, assign26370_e30830_d_n5, assign26370_e30830_d_n6, assign26370_e30830_d_n7, assign26370_e30830_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) {
        let assign26370_e30822: f64 = (1.772453850905516 * 0.5);
        let assign26370_e30825: f64 = (var_atatsti * var_erfctimesexpmtat);
        let assign26370_e30827: f64 = (assign26370_e30825 / var_ktat);
        let assign26370_e30828: f64 = (assign26370_e30822 * assign26370_e30827);
        (assign26370_e30828, (assign26370_e30822 * ((((var_atatsti * var_erfctimesexpmtat_dn5) * var_ktat) - (assign26370_e30825 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign26370_e30822 * ((((var_atatsti * var_erfctimesexpmtat_dn6) * var_ktat) - (assign26370_e30825 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign26370_e30822 * ((((var_atatsti * var_erfctimesexpmtat_dn7) * var_ktat) - (assign26370_e30825 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign26370_e30822 * ((((var_atatsti * var_erfctimesexpmtat_dn8) * var_ktat) - (assign26370_e30825 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign26370_e30830;
        var_gammamax_dn5 = assign26370_e30830_d_n5;
        var_gammamax_dn6 = assign26370_e30830_d_n6;
        var_gammamax_dn7 = assign26370_e30830_d_n7;
        var_gammamax_dn8 = assign26370_e30830_d_n8;

        let (assign26380_e30848, assign26380_e30848_d_n5, assign26380_e30848_d_n6, assign26380_e30848_d_n7, assign26380_e30848_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard492 == 0.0)) {
        let assign26380_e30843: f64 = (var_asrh * var_gammamax);
        let assign26380_e30845: f64 = (assign26380_e30843 * var_wtat);
        let assign26380_e30846: f64 = (p.p839 * assign26380_e30845);
        (assign26380_e30846, (p.p839 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign26380_e30843 * var_wtat_dn5))), (p.p839 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign26380_e30843 * var_wtat_dn6))), (p.p839 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign26380_e30843 * var_wtat_dn7))), (p.p839 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign26380_e30843 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign26380_e30848;
        var_itat_dn5 = assign26380_e30848_d_n5;
        var_itat_dn6 = assign26380_e30848_d_n6;
        var_itat_dn7 = assign26380_e30848_d_n7;
        var_itat_dn8 = assign26380_e30848_d_n8;

        let assign26390_e30851: f64 = if p.p845 == 0.0 { 1.0 } else { 0.0 };
        var_guard498 = assign26390_e30851;

        let (assign26400_e30862, assign26400_e30862_d_n5, assign26400_e30862_d_n6, assign26400_e30862_d_n7, assign26400_e30862_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard498 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign26400_e30862;
        var_ibbt_dn5 = assign26400_e30862_d_n5;
        var_ibbt_dn6 = assign26400_e30862_d_n6;
        var_ibbt_dn7 = assign26400_e30862_d_n7;
        var_ibbt_dn8 = assign26400_e30862_d_n8;

        let assign26410_e30865: f64 = if p.p825 == 0.5 { 1.0 } else { 0.0 };
        var_guard499 = assign26410_e30865;

        let (assign26420_e30884, assign26420_e30884_d_n5, assign26420_e30884_d_n6, assign26420_e30884_d_n7, assign26420_e30884_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard498 == 0.0)) && (var_guard499 != 0.0)) {
        let assign26420_e30879: f64 = (p.p822 - var_vbbt);
        let assign26420_e30881: f64 = (assign26420_e30879 * var_vbirstiinv);
        let assign26420_e30882: f64 = (assign26420_e30881).sqrt();
        (assign26420_e30882, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26420_e30884;
        var_tmp_dn5 = assign26420_e30884_d_n5;
        var_tmp_dn6 = assign26420_e30884_d_n6;
        var_tmp_dn7 = assign26420_e30884_d_n7;
        var_tmp_dn8 = assign26420_e30884_d_n8;

        let (assign26430_e30905, assign26430_e30905_d_n5, assign26430_e30905_d_n6, assign26430_e30905_d_n7, assign26430_e30905_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard498 == 0.0)) && (var_guard499 == 0.0)) {
        let assign26430_e30899: f64 = (p.p822 - var_vbbt);
        let assign26430_e30901: f64 = (assign26430_e30899 * var_vbirstiinv);
        let assign26430_e30903: f64 = (assign26430_e30901).powf(p.p825);
        (assign26430_e30903, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26430_e30905;
        var_tmp_dn5 = assign26430_e30905_d_n5;
        var_tmp_dn6 = assign26430_e30905_d_n6;
        var_tmp_dn7 = assign26430_e30905_d_n7;
        var_tmp_dn8 = assign26430_e30905_d_n8;

        let (assign26440_e30925, assign26440_e30925_d_n5, assign26440_e30925_d_n6, assign26440_e30925_d_n7, assign26440_e30925_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard498 == 0.0)) {
        let assign26440_e30918: f64 = (p.p822 - var_vbbt);
        let assign26440_e30920: f64 = (assign26440_e30918 * var_wdepnulrinvsti);
        let assign26440_e30922: f64 = (assign26440_e30920 / var_tmp);
        let assign26440_e30923: f64 = (var_one_over_one_minus_psti * assign26440_e30922);
        (assign26440_e30923, (var_one_over_one_minus_psti * (-((assign26440_e30920 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign26440_e30920 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign26440_e30920 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign26440_e30920 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign26440_e30925;
        var_fmaxr_dn5 = assign26440_e30925_d_n5;
        var_fmaxr_dn6 = assign26440_e30925_d_n6;
        var_fmaxr_dn7 = assign26440_e30925_d_n7;
        var_fmaxr_dn8 = assign26440_e30925_d_n8;

        let assign26450_e30927: f64 = (-var_fbbtsti);
        let assign26450_e30929: f64 = (assign26450_e30927 / var_fmaxr);
        let assign26450_e30930: f64 = (assign26450_e30929).abs();
        let assign26450_e30932: f64 = if assign26450_e30930 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard500 = assign26450_e30932;

        let (assign26460_e30950, assign26460_e30950_d_n5, assign26460_e30950_d_n6, assign26460_e30950_d_n7, assign26460_e30950_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard498 == 0.0)) && (var_guard500 != 0.0)) {
        let assign26460_e30945: f64 = (-var_fbbtsti);
        let assign26460_e30947: f64 = (assign26460_e30945 / var_fmaxr);
        let assign26460_e30948: f64 = (assign26460_e30947).exp();
        (assign26460_e30948, (assign26460_e30948 * (-((assign26460_e30945 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign26460_e30948 * (-((assign26460_e30945 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign26460_e30948 * (-((assign26460_e30945 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign26460_e30948 * (-((assign26460_e30945 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26460_e30950;
        var_tmp_dn5 = assign26460_e30950_d_n5;
        var_tmp_dn6 = assign26460_e30950_d_n6;
        var_tmp_dn7 = assign26460_e30950_d_n7;
        var_tmp_dn8 = assign26460_e30950_d_n8;

        let assign26470_e30952: f64 = (-var_fbbtsti);
        let assign26470_e30954: f64 = (assign26470_e30952 / var_fmaxr);
        let assign26470_e30956: f64 = if assign26470_e30954 < 0.0 { 1.0 } else { 0.0 };
        var_guard501 = assign26470_e30956;

        let (assign26480_e31007, assign26480_e31007_d_n5, assign26480_e31007_d_n6, assign26480_e31007_d_n7, assign26480_e31007_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard498 == 0.0)) && (var_guard500 == 0.0)) && (var_guard501 != 0.0)) {
        let assign26480_e30974: f64 = (-230.25850929940458);
        let assign26480_e30976: f64 = (-var_fbbtsti);
        let assign26480_e30978: f64 = (assign26480_e30976 / var_fmaxr);
        let assign26480_e30979: f64 = (assign26480_e30974 - assign26480_e30978);
        let assign26480_e30983: f64 = (-230.25850929940458);
        let assign26480_e30985: f64 = (-var_fbbtsti);
        let assign26480_e30987: f64 = (assign26480_e30985 / var_fmaxr);
        let assign26480_e30988: f64 = (assign26480_e30983 - assign26480_e30987);
        let assign26480_e30991: f64 = (-230.25850929940458);
        let assign26480_e30993: f64 = (-var_fbbtsti);
        let assign26480_e30995: f64 = (assign26480_e30993 / var_fmaxr);
        let assign26480_e30996: f64 = (assign26480_e30991 - assign26480_e30995);
        let assign26480_e30998: f64 = (assign26480_e30996 * 0.3333333333333333);
        let assign26480_e30999: f64 = (1.0 + assign26480_e30998);
        let assign26480_e31000: f64 = (assign26480_e30988 * assign26480_e30999);
        let assign26480_e31001: f64 = (0.5 * assign26480_e31000);
        let assign26480_e31002: f64 = (1.0 + assign26480_e31001);
        let assign26480_e31003: f64 = (assign26480_e30979 * assign26480_e31002);
        let assign26480_e31004: f64 = (1.0 + assign26480_e31003);
        let assign26480_e31005: f64 = (1e-100 / assign26480_e31004);
        (assign26480_e31005, (-((1e-100 * (((-(-((assign26480_e30976 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign26480_e31002) + (assign26480_e30979 * (0.5 * (((-(-((assign26480_e30985 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign26480_e30999) + (assign26480_e30988 * ((-(-((assign26480_e30993 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign26480_e31004 * assign26480_e31004))), (-((1e-100 * (((-(-((assign26480_e30976 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign26480_e31002) + (assign26480_e30979 * (0.5 * (((-(-((assign26480_e30985 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign26480_e30999) + (assign26480_e30988 * ((-(-((assign26480_e30993 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign26480_e31004 * assign26480_e31004))), (-((1e-100 * (((-(-((assign26480_e30976 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign26480_e31002) + (assign26480_e30979 * (0.5 * (((-(-((assign26480_e30985 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign26480_e30999) + (assign26480_e30988 * ((-(-((assign26480_e30993 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign26480_e31004 * assign26480_e31004))), (-((1e-100 * (((-(-((assign26480_e30976 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign26480_e31002) + (assign26480_e30979 * (0.5 * (((-(-((assign26480_e30985 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign26480_e30999) + (assign26480_e30988 * ((-(-((assign26480_e30993 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign26480_e31004 * assign26480_e31004))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26480_e31007;
        var_tmp_dn5 = assign26480_e31007_d_n5;
        var_tmp_dn6 = assign26480_e31007_d_n6;
        var_tmp_dn7 = assign26480_e31007_d_n7;
        var_tmp_dn8 = assign26480_e31007_d_n8;

        let (assign26490_e31056, assign26490_e31056_d_n5, assign26490_e31056_d_n6, assign26490_e31056_d_n7, assign26490_e31056_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard498 == 0.0)) && (var_guard500 == 0.0)) && (var_guard501 == 0.0)) {
        let assign26490_e31026: f64 = (-var_fbbtsti);
        let assign26490_e31028: f64 = (assign26490_e31026 / var_fmaxr);
        let assign26490_e31030: f64 = (assign26490_e31028 - 230.25850929940458);
        let assign26490_e31034: f64 = (-var_fbbtsti);
        let assign26490_e31036: f64 = (assign26490_e31034 / var_fmaxr);
        let assign26490_e31038: f64 = (assign26490_e31036 - 230.25850929940458);
        let assign26490_e31041: f64 = (-var_fbbtsti);
        let assign26490_e31043: f64 = (assign26490_e31041 / var_fmaxr);
        let assign26490_e31045: f64 = (assign26490_e31043 - 230.25850929940458);
        let assign26490_e31047: f64 = (assign26490_e31045 * 0.3333333333333333);
        let assign26490_e31048: f64 = (1.0 + assign26490_e31047);
        let assign26490_e31049: f64 = (assign26490_e31038 * assign26490_e31048);
        let assign26490_e31050: f64 = (0.5 * assign26490_e31049);
        let assign26490_e31051: f64 = (1.0 + assign26490_e31050);
        let assign26490_e31052: f64 = (assign26490_e31030 * assign26490_e31051);
        let assign26490_e31053: f64 = (1.0 + assign26490_e31052);
        let assign26490_e31054: f64 = (1e100 * assign26490_e31053);
        (assign26490_e31054, (1e100 * (((-((assign26490_e31026 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign26490_e31051) + (assign26490_e31030 * (0.5 * (((-((assign26490_e31034 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign26490_e31048) + (assign26490_e31038 * ((-((assign26490_e31041 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign26490_e31026 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign26490_e31051) + (assign26490_e31030 * (0.5 * (((-((assign26490_e31034 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign26490_e31048) + (assign26490_e31038 * ((-((assign26490_e31041 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign26490_e31026 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign26490_e31051) + (assign26490_e31030 * (0.5 * (((-((assign26490_e31034 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign26490_e31048) + (assign26490_e31038 * ((-((assign26490_e31041 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign26490_e31026 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign26490_e31051) + (assign26490_e31030 * (0.5 * (((-((assign26490_e31034 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign26490_e31048) + (assign26490_e31038 * ((-((assign26490_e31041 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26490_e31056;
        var_tmp_dn5 = assign26490_e31056_d_n5;
        var_tmp_dn6 = assign26490_e31056_d_n6;
        var_tmp_dn7 = assign26490_e31056_d_n7;
        var_tmp_dn8 = assign26490_e31056_d_n8;

        let (assign26500_e31076, assign26500_e31076_d_n5, assign26500_e31076_d_n6, assign26500_e31076_d_n7, assign26500_e31076_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard498 == 0.0)) {
        let assign26500_e31069: f64 = (var_v5 * var_fmaxr);
        let assign26500_e31071: f64 = (assign26500_e31069 * var_fmaxr);
        let assign26500_e31073: f64 = (assign26500_e31071 * var_tmp);
        let assign26500_e31074: f64 = (p.p845 * assign26500_e31073);
        (assign26500_e31074, (p.p845 * (((((var_v5 * var_fmaxr_dn5) * var_fmaxr) + (assign26500_e31069 * var_fmaxr_dn5)) * var_tmp) + (assign26500_e31071 * var_tmp_dn5))), (p.p845 * (((((var_v5 * var_fmaxr_dn6) * var_fmaxr) + (assign26500_e31069 * var_fmaxr_dn6)) * var_tmp) + (assign26500_e31071 * var_tmp_dn6))), (p.p845 * (((((var_v5 * var_fmaxr_dn7) * var_fmaxr) + (assign26500_e31069 * var_fmaxr_dn7)) * var_tmp) + (assign26500_e31071 * var_tmp_dn7))), (p.p845 * (((((var_v5 * var_fmaxr_dn8) * var_fmaxr) + (assign26500_e31069 * var_fmaxr_dn8)) * var_tmp) + (assign26500_e31071 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign26500_e31076;
        var_ibbt_dn5 = assign26500_e31076_d_n5;
        var_ibbt_dn6 = assign26500_e31076_d_n6;
        var_ibbt_dn7 = assign26500_e31076_d_n7;
        var_ibbt_dn8 = assign26500_e31076_d_n8;

        let assign26510_e31079: f64 = if p.p854 > 1000.0 { 1.0 } else { 0.0 };
        var_guard502 = assign26510_e31079;

        let (assign26520_e31090, assign26520_e31090_d_n5, assign26520_e31090_d_n6, assign26520_e31090_d_n7, assign26520_e31090_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard502 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign26520_e31090;
        var_fbreakdown_dn5 = assign26520_e31090_d_n5;
        var_fbreakdown_dn6 = assign26520_e31090_d_n6;
        var_fbreakdown_dn7 = assign26520_e31090_d_n7;
        var_fbreakdown_dn8 = assign26520_e31090_d_n8;

        let assign26530_e31093: f64 = (-var_alphaav);
        let assign26530_e31095: f64 = (assign26530_e31093 * p.p854);
        let assign26530_e31096: f64 = if var_vav > assign26530_e31095 { 1.0 } else { 0.0 };
        var_guard503 = assign26530_e31096;

        let assign26540_e31099: f64 = if p.p857 == 4.0 { 1.0 } else { 0.0 };
        var_guard504 = assign26540_e31099;

        let (assign26550_e31129, assign26550_e31129_d_n5, assign26550_e31129_d_n6, assign26550_e31129_d_n7, assign26550_e31129_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard502 == 0.0)) && (var_guard503 != 0.0)) && (var_guard504 != 0.0)) {
        let assign26550_e31115: f64 = (var_vav * var_vbrinvsti);
        let assign26550_e31118: f64 = (var_vav * var_vbrinvsti);
        let assign26550_e31119: f64 = (assign26550_e31115 * assign26550_e31118);
        let assign26550_e31122: f64 = (var_vav * var_vbrinvsti);
        let assign26550_e31123: f64 = (assign26550_e31119 * assign26550_e31122);
        let assign26550_e31126: f64 = (var_vav * var_vbrinvsti);
        let assign26550_e31127: f64 = (assign26550_e31123 * assign26550_e31126);
        (assign26550_e31127, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26550_e31129;
        var_tmp_dn5 = assign26550_e31129_d_n5;
        var_tmp_dn6 = assign26550_e31129_d_n6;
        var_tmp_dn7 = assign26550_e31129_d_n7;
        var_tmp_dn8 = assign26550_e31129_d_n8;

        let (assign26560_e31151, assign26560_e31151_d_n5, assign26560_e31151_d_n6, assign26560_e31151_d_n7, assign26560_e31151_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard502 == 0.0)) && (var_guard503 != 0.0)) && (var_guard504 == 0.0)) {
        let assign26560_e31146: f64 = (var_vav * var_vbrinvsti);
        let assign26560_e31147: f64 = (assign26560_e31146).abs();
        let assign26560_e31149: f64 = (assign26560_e31147).powf(p.p857);
        (assign26560_e31149, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26560_e31151;
        var_tmp_dn5 = assign26560_e31151_d_n5;
        var_tmp_dn6 = assign26560_e31151_d_n6;
        var_tmp_dn7 = assign26560_e31151_d_n7;
        var_tmp_dn8 = assign26560_e31151_d_n8;

        let (assign26570_e31169, assign26570_e31169_d_n5, assign26570_e31169_d_n6, assign26570_e31169_d_n7, assign26570_e31169_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard502 == 0.0)) && (var_guard503 != 0.0)) {
        let assign26570_e31166: f64 = (1.0 - var_tmp);
        let assign26570_e31167: f64 = (1.0 / assign26570_e31166);
        (assign26570_e31167, (-((-var_tmp_dn5) / (assign26570_e31166 * assign26570_e31166))), (-((-var_tmp_dn6) / (assign26570_e31166 * assign26570_e31166))), (-((-var_tmp_dn7) / (assign26570_e31166 * assign26570_e31166))), (-((-var_tmp_dn8) / (assign26570_e31166 * assign26570_e31166))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign26570_e31169;
        var_fbreakdown_dn5 = assign26570_e31169_d_n5;
        var_fbreakdown_dn6 = assign26570_e31169_d_n6;
        var_fbreakdown_dn7 = assign26570_e31169_d_n7;
        var_fbreakdown_dn8 = assign26570_e31169_d_n8;

        let (assign26580_e31192, assign26580_e31192_d_n5, assign26580_e31192_d_n6, assign26580_e31192_d_n7, assign26580_e31192_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) && (var_guard502 == 0.0)) && (var_guard503 == 0.0)) {
        let assign26580_e31186: f64 = (var_alphaav * p.p854);
        let assign26580_e31187: f64 = (var_vav + assign26580_e31186);
        let assign26580_e31189: f64 = (assign26580_e31187 * var_slopesti);
        let assign26580_e31190: f64 = (var_fstopsti + assign26580_e31189);
        (assign26580_e31190, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign26580_e31192;
        var_fbreakdown_dn5 = assign26580_e31192_d_n5;
        var_fbreakdown_dn6 = assign26580_e31192_d_n6;
        var_fbreakdown_dn7 = assign26580_e31192_d_n7;
        var_fbreakdown_dn8 = assign26580_e31192_d_n8;

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
        *var_guard494_slot = var_guard494;
        *var_guard495_slot = var_guard495;
        *var_guard496_slot = var_guard496;
        *var_guard497_slot = var_guard497;
        *var_guard498_slot = var_guard498;
        *var_guard499_slot = var_guard499;
        *var_guard500_slot = var_guard500;
        *var_guard501_slot = var_guard501;
        *var_guard502_slot = var_guard502;
        *var_guard503_slot = var_guard503;
        *var_guard504_slot = var_guard504;
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
        *var_ysq_slot = var_ysq;
        *var_ysq_dn5_slot = var_ysq_dn5;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
    }

    pub(super) fn stamp_transient_block_51(
        p: &Parameters,
        var_atatgat: f64,
        var_btatpartgat: f64,
        var_fbreakdown: f64,
        var_fbreakdown_dn5: f64,
        var_fbreakdown_dn6: f64,
        var_fbreakdown_dn7: f64,
        var_fbreakdown_dn8: f64,
        var_ftdgat: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard488: f64,
        var_ibbt: f64,
        var_ibbt_dn5: f64,
        var_ibbt_dn6: f64,
        var_ibbt_dn7: f64,
        var_ibbt_dn8: f64,
        var_idmult: f64,
        var_idsatgat: f64,
        var_lgsource_i: f64,
        var_one_minus_pgat: f64,
        var_one_over_one_minus_pgat: f64,
        var_perfc: f64,
        var_two_psistar: f64,
        var_vbigat: f64,
        var_vbirgatinv: f64,
        var_vjsrh: f64,
        var_wdepnulrgat: f64,
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
        var_guard505_slot: &mut f64,
        var_guard506_slot: &mut f64,
        var_guard507_slot: &mut f64,
        var_guard508_slot: &mut f64,
        var_guard509_slot: &mut f64,
        var_guard510_slot: &mut f64,
        var_guard511_slot: &mut f64,
        var_guard512_slot: &mut f64,
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
        let mut var_guard505: f64 = *var_guard505_slot;
        let mut var_guard506: f64 = *var_guard506_slot;
        let mut var_guard507: f64 = *var_guard507_slot;
        let mut var_guard508: f64 = *var_guard508_slot;
        let mut var_guard509: f64 = *var_guard509_slot;
        let mut var_guard510: f64 = *var_guard510_slot;
        let mut var_guard511: f64 = *var_guard511_slot;
        let mut var_guard512: f64 = *var_guard512_slot;
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

        let (assign26590_e31211, assign26590_e31211_d_n5, assign26590_e31211_d_n6, assign26590_e31211_d_n7, assign26590_e31211_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard488 == 0.0)) {
        let assign26590_e31202: f64 = (var_id__blk213 + var_isrh);
        let assign26590_e31204: f64 = (assign26590_e31202 + var_itat);
        let assign26590_e31206: f64 = (assign26590_e31204 + var_ibbt);
        let assign26590_e31207: f64 = (p.p29 * assign26590_e31206);
        let assign26590_e31209: f64 = (assign26590_e31207 * var_fbreakdown);
        (assign26590_e31209, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign26590_e31207 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign26590_e31207 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign26590_e31207 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign26590_e31207 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign26590_e31211;
        var_ijunsti_dn5 = assign26590_e31211_d_n5;
        var_ijunsti_dn6 = assign26590_e31211_d_n6;
        var_ijunsti_dn7 = assign26590_e31211_d_n7;
        var_ijunsti_dn8 = assign26590_e31211_d_n8;

        let assign26600_e31214: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard505 = assign26600_e31214;

        let (assign26610_e31222, assign26610_e31222_d_n5, assign26610_e31222_d_n6, assign26610_e31222_d_n7, assign26610_e31222_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign26610_e31222;
        var_ijungat_dn5 = assign26610_e31222_d_n5;
        var_ijungat_dn6 = assign26610_e31222_d_n6;
        var_ijungat_dn7 = assign26610_e31222_d_n7;
        var_ijungat_dn8 = assign26610_e31222_d_n8;

        let (assign26620_e31233,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) {
        let assign26620_e31231: f64 = (var_idsatgat * var_idmult);
        (assign26620_e31231,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign26620_e31233;

        let assign26630_e31240: f64 = if ((p.p835 == 0.0) && (p.p840 == 0.0)) { 1.0 } else { 0.0 };
        var_guard506 = assign26630_e31240;

        let (assign26640_e31251, assign26640_e31251_d_n5, assign26640_e31251_d_n6, assign26640_e31251_d_n7, assign26640_e31251_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard506 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign26640_e31251;
        var_isrh_dn5 = assign26640_e31251_d_n5;
        var_isrh_dn6 = assign26640_e31251_d_n6;
        var_isrh_dn7 = assign26640_e31251_d_n7;
        var_isrh_dn8 = assign26640_e31251_d_n8;

        let (assign26650_e31265,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard506 == 0.0)) {
        let assign26650_e31263: f64 = (var_vbigat - var_vjsrh);
        (assign26650_e31263,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign26650_e31265;

        let (assign26660_e31284,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard506 == 0.0)) {
        let assign26660_e31279: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign26660_e31280: f64 = (1.0 - assign26660_e31279);
        let assign26660_e31281: f64 = (assign26660_e31280).sqrt();
        let assign26660_e31282: f64 = (1.0 - assign26660_e31281);
        (assign26660_e31282,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign26660_e31284;

        let assign26670_e31287: f64 = if p.p826 == 0.5 { 1.0 } else { 0.0 };
        var_guard507 = assign26670_e31287;

        let (assign26680_e31301,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard506 == 0.0)) && (var_guard507 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign26680_e31301;

        let (assign26690_e31333,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard506 == 0.0)) && (var_guard507 == 0.0)) {
        let assign26690_e31316: f64 = (var_wsrhstep * var_wsrhstep);
        let assign26690_e31318: f64 = (var_wsrhstep).ln();
        let assign26690_e31319: f64 = (assign26690_e31316 * assign26690_e31318);
        let assign26690_e31322: f64 = (1.0 - var_wsrhstep);
        let assign26690_e31323: f64 = (assign26690_e31319 / assign26690_e31322);
        let assign26690_e31325: f64 = (assign26690_e31323 + var_wsrhstep);
        let assign26690_e31329: f64 = (2.0 * p.p826);
        let assign26690_e31330: f64 = (1.0 - assign26690_e31329);
        let assign26690_e31331: f64 = (assign26690_e31325 * assign26690_e31330);
        (assign26690_e31331,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign26690_e31333;

        let (assign26700_e31347,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard506 == 0.0)) {
        let assign26700_e31345: f64 = (var_wsrhstep + var_dwsrh);
        (assign26700_e31345,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign26700_e31347;

        let assign26710_e31350: f64 = if p.p826 == 0.5 { 1.0 } else { 0.0 };
        var_guard508 = assign26710_e31350;

        let (assign26720_e31367, assign26720_e31367_d_n5, assign26720_e31367_d_n6, assign26720_e31367_d_n7, assign26720_e31367_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard506 == 0.0)) && (var_guard508 != 0.0)) {
        let assign26720_e31364: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign26720_e31365: f64 = (assign26720_e31364).sqrt();
        (assign26720_e31365, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26720_e31367;
        var_tmp_dn5 = assign26720_e31367_d_n5;
        var_tmp_dn6 = assign26720_e31367_d_n6;
        var_tmp_dn7 = assign26720_e31367_d_n7;
        var_tmp_dn8 = assign26720_e31367_d_n8;

        let (assign26730_e31386, assign26730_e31386_d_n5, assign26730_e31386_d_n6, assign26730_e31386_d_n7, assign26730_e31386_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard506 == 0.0)) && (var_guard508 == 0.0)) {
        let assign26730_e31382: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign26730_e31384: f64 = (assign26730_e31382).powf(p.p826);
        (assign26730_e31384, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26730_e31386;
        var_tmp_dn5 = assign26730_e31386_d_n5;
        var_tmp_dn6 = assign26730_e31386_d_n6;
        var_tmp_dn7 = assign26730_e31386_d_n7;
        var_tmp_dn8 = assign26730_e31386_d_n8;

        let (assign26740_e31400, assign26740_e31400_d_n5, assign26740_e31400_d_n6, assign26740_e31400_d_n7, assign26740_e31400_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard506 == 0.0)) {
        let assign26740_e31398: f64 = (var_wdepnulrgat * var_tmp);
        (assign26740_e31398, (var_wdepnulrgat * var_tmp_dn5), (var_wdepnulrgat * var_tmp_dn6), (var_wdepnulrgat * var_tmp_dn7), (var_wdepnulrgat * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign26740_e31400;
        var_wdep_dn5 = assign26740_e31400_d_n5;
        var_wdep_dn6 = assign26740_e31400_d_n6;
        var_wdep_dn7 = assign26740_e31400_d_n7;
        var_wdep_dn8 = assign26740_e31400_d_n8;

        let (assign26750_e31418, assign26750_e31418_d_n5, assign26750_e31418_d_n6, assign26750_e31418_d_n7, assign26750_e31418_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard506 == 0.0)) {
        let assign26750_e31413: f64 = (var_zinv - 1.0);
        let assign26750_e31415: f64 = (assign26750_e31413 * var_wdep);
        let assign26750_e31416: f64 = (var_ftdgat * assign26750_e31415);
        (assign26750_e31416, (var_ftdgat * (assign26750_e31413 * var_wdep_dn5)), (var_ftdgat * (assign26750_e31413 * var_wdep_dn6)), (var_ftdgat * (assign26750_e31413 * var_wdep_dn7)), (var_ftdgat * (assign26750_e31413 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign26750_e31418;
        var_asrh_dn5 = assign26750_e31418_d_n5;
        var_asrh_dn6 = assign26750_e31418_d_n6;
        var_asrh_dn7 = assign26750_e31418_d_n7;
        var_asrh_dn8 = assign26750_e31418_d_n8;

        let (assign26760_e31434, assign26760_e31434_d_n5, assign26760_e31434_d_n6, assign26760_e31434_d_n7, assign26760_e31434_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard506 == 0.0)) {
        let assign26760_e31431: f64 = (var_asrh * var_wsrh);
        let assign26760_e31432: f64 = (p.p835 * assign26760_e31431);
        (assign26760_e31432, (p.p835 * (var_asrh_dn5 * var_wsrh)), (p.p835 * (var_asrh_dn6 * var_wsrh)), (p.p835 * (var_asrh_dn7 * var_wsrh)), (p.p835 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign26760_e31434;
        var_isrh_dn5 = assign26760_e31434_d_n5;
        var_isrh_dn6 = assign26760_e31434_d_n6;
        var_isrh_dn7 = assign26760_e31434_d_n7;
        var_isrh_dn8 = assign26760_e31434_d_n8;

        let assign26770_e31437: f64 = if p.p840 == 0.0 { 1.0 } else { 0.0 };
        var_guard509 = assign26770_e31437;

        let (assign26780_e31448, assign26780_e31448_d_n5, assign26780_e31448_d_n6, assign26780_e31448_d_n7, assign26780_e31448_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign26780_e31448;
        var_itat_dn5 = assign26780_e31448_d_n5;
        var_itat_dn6 = assign26780_e31448_d_n6;
        var_itat_dn7 = assign26780_e31448_d_n7;
        var_itat_dn8 = assign26780_e31448_d_n8;

        let (assign26790_e31466, assign26790_e31466_d_n5, assign26790_e31466_d_n6, assign26790_e31466_d_n7, assign26790_e31466_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) {
        let assign26790_e31461: f64 = (var_wdep * var_one_minus_pgat);
        let assign26790_e31463: f64 = (assign26790_e31461 / var_vbi_minus_vjsrh);
        let assign26790_e31464: f64 = (var_btatpartgat * assign26790_e31463);
        (assign26790_e31464, (var_btatpartgat * ((var_wdep_dn5 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn6 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn7 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn8 * var_one_minus_pgat) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign26790_e31466;
        var_btat_dn5 = assign26790_e31466_d_n5;
        var_btat_dn6 = assign26790_e31466_d_n6;
        var_btat_dn7 = assign26790_e31466_d_n7;
        var_btat_dn8 = assign26790_e31466_d_n8;

        let (assign26800_e31482, assign26800_e31482_d_n5, assign26800_e31482_d_n6, assign26800_e31482_d_n7, assign26800_e31482_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) {
        let assign26800_e31478: f64 = (0.666666666666667 * var_atatgat);
        let assign26800_e31480: f64 = (assign26800_e31478 / var_btat);
        (assign26800_e31480, (-((assign26800_e31478 * var_btat_dn5) / (var_btat * var_btat))), (-((assign26800_e31478 * var_btat_dn6) / (var_btat * var_btat))), (-((assign26800_e31478 * var_btat_dn7) / (var_btat * var_btat))), (-((assign26800_e31478 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign26800_e31482;
        var_twoatatoverthreebtat_dn5 = assign26800_e31482_d_n5;
        var_twoatatoverthreebtat_dn6 = assign26800_e31482_d_n6;
        var_twoatatoverthreebtat_dn7 = assign26800_e31482_d_n7;
        var_twoatatoverthreebtat_dn8 = assign26800_e31482_d_n8;

        let (assign26810_e31496, assign26810_e31496_d_n5, assign26810_e31496_d_n6, assign26810_e31496_d_n7, assign26810_e31496_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) {
        let assign26810_e31494: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign26810_e31494, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign26810_e31496;
        var_umaxbeforelimiting_dn5 = assign26810_e31496_d_n5;
        var_umaxbeforelimiting_dn6 = assign26810_e31496_d_n6;
        var_umaxbeforelimiting_dn7 = assign26810_e31496_d_n7;
        var_umaxbeforelimiting_dn8 = assign26810_e31496_d_n8;

        let (assign26820_e31517, assign26820_e31517_d_n5, assign26820_e31517_d_n6, assign26820_e31517_d_n7, assign26820_e31517_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) {
        let assign26820_e31508: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign26820_e31511: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign26820_e31513: f64 = (assign26820_e31511 + 1.0);
        let assign26820_e31514: f64 = (assign26820_e31508 / assign26820_e31513);
        let assign26820_e31515: f64 = (assign26820_e31514).sqrt();
        (assign26820_e31515, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign26820_e31513) - (assign26820_e31508 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign26820_e31513 * assign26820_e31513)) / (2.0 * assign26820_e31515)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign26820_e31513) - (assign26820_e31508 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign26820_e31513 * assign26820_e31513)) / (2.0 * assign26820_e31515)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign26820_e31513) - (assign26820_e31508 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign26820_e31513 * assign26820_e31513)) / (2.0 * assign26820_e31515)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign26820_e31513) - (assign26820_e31508 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign26820_e31513 * assign26820_e31513)) / (2.0 * assign26820_e31515)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign26820_e31517;
        var_umax_dn5 = assign26820_e31517_d_n5;
        var_umax_dn6 = assign26820_e31517_d_n6;
        var_umax_dn7 = assign26820_e31517_d_n7;
        var_umax_dn8 = assign26820_e31517_d_n8;

        let (assign26830_e31530, assign26830_e31530_d_n5, assign26830_e31530_d_n6, assign26830_e31530_d_n7, assign26830_e31530_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) {
        let assign26830_e31528: f64 = (var_umax).sqrt();
        (assign26830_e31528, (var_umax_dn5 / (2.0 * assign26830_e31528)), (var_umax_dn6 / (2.0 * assign26830_e31528)), (var_umax_dn7 / (2.0 * assign26830_e31528)), (var_umax_dn8 / (2.0 * assign26830_e31528)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign26830_e31530;
        var_sqrtumax_dn5 = assign26830_e31530_d_n5;
        var_sqrtumax_dn6 = assign26830_e31530_d_n6;
        var_sqrtumax_dn7 = assign26830_e31530_d_n7;
        var_sqrtumax_dn8 = assign26830_e31530_d_n8;

        let (assign26840_e31544, assign26840_e31544_d_n5, assign26840_e31544_d_n6, assign26840_e31544_d_n7, assign26840_e31544_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) {
        let assign26840_e31542: f64 = (var_umax * var_sqrtumax);
        (assign26840_e31542, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign26840_e31544;
        var_umaxpoweronepointfive_dn5 = assign26840_e31544_d_n5;
        var_umaxpoweronepointfive_dn6 = assign26840_e31544_d_n6;
        var_umaxpoweronepointfive_dn7 = assign26840_e31544_d_n7;
        var_umaxpoweronepointfive_dn8 = assign26840_e31544_d_n8;

        let assign26850_e31546: f64 = (-p.p826);
        let assign26850_e31548: f64 = (assign26850_e31546 * var_one_over_one_minus_pgat);
        let assign26850_e31550: f64 = (-1.0);
        let assign26850_e31551: f64 = if assign26850_e31548 == assign26850_e31550 { 1.0 } else { 0.0 };
        var_guard510 = assign26850_e31551;

        let (assign26860_e31571, assign26860_e31571_d_n5, assign26860_e31571_d_n6, assign26860_e31571_d_n7, assign26860_e31571_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) && (var_guard510 != 0.0)) {
        let assign26860_e31567: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign26860_e31568: f64 = (1.0 + assign26860_e31567);
        let assign26860_e31569: f64 = (1.0 / assign26860_e31568);
        (assign26860_e31569, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign26860_e31568 * assign26860_e31568))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign26860_e31568 * assign26860_e31568))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign26860_e31568 * assign26860_e31568))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign26860_e31568 * assign26860_e31568))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign26860_e31571;
        var_wgamma_dn5 = assign26860_e31571_d_n5;
        var_wgamma_dn6 = assign26860_e31571_d_n6;
        var_wgamma_dn7 = assign26860_e31571_d_n7;
        var_wgamma_dn8 = assign26860_e31571_d_n8;

        let (assign26870_e31595, assign26870_e31595_d_n5, assign26870_e31595_d_n6, assign26870_e31595_d_n7, assign26870_e31595_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) && (var_guard510 == 0.0)) {
        let assign26870_e31587: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign26870_e31588: f64 = (1.0 + assign26870_e31587);
        let assign26870_e31590: f64 = (-p.p826);
        let assign26870_e31592: f64 = (assign26870_e31590 * var_one_over_one_minus_pgat);
        let assign26870_e31593: f64 = (assign26870_e31588).powf(assign26870_e31592);
        (assign26870_e31593, if 0.0 == 0.0 && ((assign26870_e31592) as f64).is_finite() && ((assign26870_e31592) as f64).fract() == 0.0 { if assign26870_e31592 == 0.0 { 0.0 } else { (assign26870_e31592 * ((assign26870_e31588).powf(assign26870_e31592 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign26870_e31593 * (assign26870_e31592 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign26870_e31588))) }, if 0.0 == 0.0 && ((assign26870_e31592) as f64).is_finite() && ((assign26870_e31592) as f64).fract() == 0.0 { if assign26870_e31592 == 0.0 { 0.0 } else { (assign26870_e31592 * ((assign26870_e31588).powf(assign26870_e31592 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign26870_e31593 * (assign26870_e31592 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign26870_e31588))) }, if 0.0 == 0.0 && ((assign26870_e31592) as f64).is_finite() && ((assign26870_e31592) as f64).fract() == 0.0 { if assign26870_e31592 == 0.0 { 0.0 } else { (assign26870_e31592 * ((assign26870_e31588).powf(assign26870_e31592 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign26870_e31593 * (assign26870_e31592 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign26870_e31588))) }, if 0.0 == 0.0 && ((assign26870_e31592) as f64).is_finite() && ((assign26870_e31592) as f64).fract() == 0.0 { if assign26870_e31592 == 0.0 { 0.0 } else { (assign26870_e31592 * ((assign26870_e31588).powf(assign26870_e31592 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign26870_e31593 * (assign26870_e31592 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign26870_e31588))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign26870_e31595;
        var_wgamma_dn5 = assign26870_e31595_d_n5;
        var_wgamma_dn6 = assign26870_e31595_d_n6;
        var_wgamma_dn7 = assign26870_e31595_d_n7;
        var_wgamma_dn8 = assign26870_e31595_d_n8;

        let (assign26880_e31613, assign26880_e31613_d_n5, assign26880_e31613_d_n6, assign26880_e31613_d_n7, assign26880_e31613_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) {
        let assign26880_e31607: f64 = (var_wsrh * var_wgamma);
        let assign26880_e31610: f64 = (var_wsrh + var_wgamma);
        let assign26880_e31611: f64 = (assign26880_e31607 / assign26880_e31610);
        (assign26880_e31611, ((((var_wsrh * var_wgamma_dn5) * assign26880_e31610) - (assign26880_e31607 * var_wgamma_dn5)) / (assign26880_e31610 * assign26880_e31610)), ((((var_wsrh * var_wgamma_dn6) * assign26880_e31610) - (assign26880_e31607 * var_wgamma_dn6)) / (assign26880_e31610 * assign26880_e31610)), ((((var_wsrh * var_wgamma_dn7) * assign26880_e31610) - (assign26880_e31607 * var_wgamma_dn7)) / (assign26880_e31610 * assign26880_e31610)), ((((var_wsrh * var_wgamma_dn8) * assign26880_e31610) - (assign26880_e31607 * var_wgamma_dn8)) / (assign26880_e31610 * assign26880_e31610)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign26880_e31613;
        var_wtat_dn5 = assign26880_e31613_d_n5;
        var_wtat_dn6 = assign26880_e31613_d_n6;
        var_wtat_dn7 = assign26880_e31613_d_n7;
        var_wtat_dn8 = assign26880_e31613_d_n8;

        let (assign26890_e31630, assign26890_e31630_d_n5, assign26890_e31630_d_n6, assign26890_e31630_d_n7, assign26890_e31630_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) {
        let assign26890_e31626: f64 = (var_btat / var_sqrtumax);
        let assign26890_e31627: f64 = (0.375 * assign26890_e31626);
        let assign26890_e31628: f64 = (assign26890_e31627).sqrt();
        (assign26890_e31628, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign26890_e31628)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign26890_e31628)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign26890_e31628)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign26890_e31628)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign26890_e31630;
        var_ktat_dn5 = assign26890_e31630_d_n5;
        var_ktat_dn6 = assign26890_e31630_d_n6;
        var_ktat_dn7 = assign26890_e31630_d_n7;
        var_ktat_dn8 = assign26890_e31630_d_n8;

        let (assign26900_e31648, assign26900_e31648_d_n5, assign26900_e31648_d_n6, assign26900_e31648_d_n7, assign26900_e31648_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) {
        let assign26900_e31643: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign26900_e31644: f64 = (2.0 * assign26900_e31643);
        let assign26900_e31646: f64 = (assign26900_e31644 - var_umax);
        (assign26900_e31646, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign26900_e31648;
        var_ltat_dn5 = assign26900_e31648_d_n5;
        var_ltat_dn6 = assign26900_e31648_d_n6;
        var_ltat_dn7 = assign26900_e31648_d_n7;
        var_ltat_dn8 = assign26900_e31648_d_n8;

        let (assign26910_e31674, assign26910_e31674_d_n5, assign26910_e31674_d_n6, assign26910_e31674_d_n7, assign26910_e31674_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) {
        let assign26910_e31660: f64 = (var_atatgat * var_twoatatoverthreebtat);
        let assign26910_e31662: f64 = (assign26910_e31660 * var_sqrtumax);
        let assign26910_e31665: f64 = (var_atatgat * var_umax);
        let assign26910_e31666: f64 = (assign26910_e31662 - assign26910_e31665);
        let assign26910_e31670: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign26910_e31671: f64 = (0.5 * assign26910_e31670);
        let assign26910_e31672: f64 = (assign26910_e31666 + assign26910_e31671);
        (assign26910_e31672, (((((var_atatgat * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign26910_e31660 * var_sqrtumax_dn5)) - (var_atatgat * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign26910_e31660 * var_sqrtumax_dn6)) - (var_atatgat * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign26910_e31660 * var_sqrtumax_dn7)) - (var_atatgat * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign26910_e31660 * var_sqrtumax_dn8)) - (var_atatgat * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign26910_e31674;
        var_mtat_dn5 = assign26910_e31674_d_n5;
        var_mtat_dn6 = assign26910_e31674_d_n6;
        var_mtat_dn7 = assign26910_e31674_d_n7;
        var_mtat_dn8 = assign26910_e31674_d_n8;

        let (assign26920_e31690, assign26920_e31690_d_n5, assign26920_e31690_d_n6, assign26920_e31690_d_n7, assign26920_e31690_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) {
        let assign26920_e31686: f64 = (var_ltat - 1.0);
        let assign26920_e31688: f64 = (assign26920_e31686 * var_ktat);
        (assign26920_e31688, ((var_ltat_dn5 * var_ktat) + (assign26920_e31686 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign26920_e31686 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign26920_e31686 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign26920_e31686 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign26920_e31690;
        var_xerfc_dn5 = assign26920_e31690_d_n5;
        var_xerfc_dn6 = assign26920_e31690_d_n6;
        var_xerfc_dn7 = assign26920_e31690_d_n7;
        var_xerfc_dn8 = assign26920_e31690_d_n8;

        let (assign26930_e31704, assign26930_e31704_d_n5, assign26930_e31704_d_n6, assign26930_e31704_d_n7, assign26930_e31704_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) {
        let assign26930_e31702: f64 = (var_xerfc * var_xerfc);
        (assign26930_e31702, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign26930_e31704;
        var_ysq_dn5 = assign26930_e31704_d_n5;
        var_ysq_dn6 = assign26930_e31704_d_n6;
        var_ysq_dn7 = assign26930_e31704_d_n7;
        var_ysq_dn8 = assign26930_e31704_d_n8;

        let assign26940_e31707: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard511 = assign26940_e31707;

        let (assign26950_e31727, assign26950_e31727_d_n5, assign26950_e31727_d_n6, assign26950_e31727_d_n7, assign26950_e31727_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) && (var_guard511 != 0.0)) {
        let assign26950_e31723: f64 = (var_perfc * var_xerfc);
        let assign26950_e31724: f64 = (1.0 + assign26950_e31723);
        let assign26950_e31725: f64 = (1.0 / assign26950_e31724);
        (assign26950_e31725, (-((var_perfc * var_xerfc_dn5) / (assign26950_e31724 * assign26950_e31724))), (-((var_perfc * var_xerfc_dn6) / (assign26950_e31724 * assign26950_e31724))), (-((var_perfc * var_xerfc_dn7) / (assign26950_e31724 * assign26950_e31724))), (-((var_perfc * var_xerfc_dn8) / (assign26950_e31724 * assign26950_e31724))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign26950_e31727;
        var_terfc_dn5 = assign26950_e31727_d_n5;
        var_terfc_dn6 = assign26950_e31727_d_n6;
        var_terfc_dn7 = assign26950_e31727_d_n7;
        var_terfc_dn8 = assign26950_e31727_d_n8;

        let (assign26960_e31748, assign26960_e31748_d_n5, assign26960_e31748_d_n6, assign26960_e31748_d_n7, assign26960_e31748_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) && (var_guard511 == 0.0)) {
        let assign26960_e31744: f64 = (var_perfc * var_xerfc);
        let assign26960_e31745: f64 = (1.0 - assign26960_e31744);
        let assign26960_e31746: f64 = (1.0 / assign26960_e31745);
        (assign26960_e31746, (-((-(var_perfc * var_xerfc_dn5)) / (assign26960_e31745 * assign26960_e31745))), (-((-(var_perfc * var_xerfc_dn6)) / (assign26960_e31745 * assign26960_e31745))), (-((-(var_perfc * var_xerfc_dn7)) / (assign26960_e31745 * assign26960_e31745))), (-((-(var_perfc * var_xerfc_dn8)) / (assign26960_e31745 * assign26960_e31745))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign26960_e31748;
        var_terfc_dn5 = assign26960_e31748_d_n5;
        var_terfc_dn6 = assign26960_e31748_d_n6;
        var_terfc_dn7 = assign26960_e31748_d_n7;
        var_terfc_dn8 = assign26960_e31748_d_n8;

        let assign26970_e31750: f64 = (-var_ysq);
        let assign26970_e31752: f64 = (assign26970_e31750 + var_mtat);
        let assign26970_e31754: f64 = (-230.25850929940458);
        let assign26970_e31755: f64 = if assign26970_e31752 > assign26970_e31754 { 1.0 } else { 0.0 };
        var_guard512 = assign26970_e31755;

        let (assign26980_e31773, assign26980_e31773_d_n5, assign26980_e31773_d_n6, assign26980_e31773_d_n7, assign26980_e31773_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) && (var_guard512 != 0.0)) {
        let assign26980_e31768: f64 = (-var_ysq);
        let assign26980_e31770: f64 = (assign26980_e31768 + var_mtat);
        let assign26980_e31771: f64 = (assign26980_e31770).exp();
        (assign26980_e31771, (assign26980_e31771 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign26980_e31771 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign26980_e31771 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign26980_e31771 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26980_e31773;
        var_tmp_dn5 = assign26980_e31773_d_n5;
        var_tmp_dn6 = assign26980_e31773_d_n6;
        var_tmp_dn7 = assign26980_e31773_d_n7;
        var_tmp_dn8 = assign26980_e31773_d_n8;

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
        *var_guard505_slot = var_guard505;
        *var_guard506_slot = var_guard506;
        *var_guard507_slot = var_guard507;
        *var_guard508_slot = var_guard508;
        *var_guard509_slot = var_guard509;
        *var_guard510_slot = var_guard510;
        *var_guard511_slot = var_guard511;
        *var_guard512_slot = var_guard512;
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

    pub(super) fn stamp_transient_block_52(
        p: &Parameters,
        var_absource_i: f64,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatgat: f64,
        var_berfc: f64,
        var_cerfc: f64,
        var_fbbtgat: f64,
        var_fbbtgat_dn5: f64,
        var_fbbtgat_dn6: f64,
        var_fbbtgat_dn7: f64,
        var_fbbtgat_dn8: f64,
        var_fstopgat: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard505: f64,
        var_guard509: f64,
        var_guard512: f64,
        var_i4: f64,
        var_i4_dn5: f64,
        var_i4_dn6: f64,
        var_i4_dn7: f64,
        var_i4_dn8: f64,
        var_id__blk213: f64,
        var_idsatbot: f64,
        var_idsatgat: f64,
        var_idsatsti: f64,
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
        var_lgsource_i: f64,
        var_lssource_i: f64,
        var_mfor1_s: f64,
        var_mtat: f64,
        var_mtat_dn5: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_one_over_one_minus_pgat: f64,
        var_phitdinv: f64,
        var_slopegat: f64,
        var_slopegat_dn5: f64,
        var_slopegat_dn6: f64,
        var_slopegat_dn7: f64,
        var_slopegat_dn8: f64,
        var_terfc: f64,
        var_terfc_dn5: f64,
        var_terfc_dn6: f64,
        var_terfc_dn7: f64,
        var_terfc_dn8: f64,
        var_v4: f64,
        var_v5: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirgatinv: f64,
        var_vbrinvgat: f64,
        var_vbrinvgat_dn5: f64,
        var_vbrinvgat_dn6: f64,
        var_vbrinvgat_dn7: f64,
        var_vbrinvgat_dn8: f64,
        var_wdepnulrinvgat: f64,
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
        var_guard513_slot: &mut f64,
        var_guard514_slot: &mut f64,
        var_guard515_slot: &mut f64,
        var_guard516_slot: &mut f64,
        var_guard517_slot: &mut f64,
        var_guard518_slot: &mut f64,
        var_guard519_slot: &mut f64,
        var_guard520_slot: &mut f64,
        var_guard521_slot: &mut f64,
        var_i4_cor_slot: &mut f64,
        var_i4_cor_dn5_slot: &mut f64,
        var_i4_cor_dn6_slot: &mut f64,
        var_i4_cor_dn7_slot: &mut f64,
        var_i4_cor_dn8_slot: &mut f64,
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
        var_isatfor1_s_slot: &mut f64,
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
        let mut var_guard513: f64 = *var_guard513_slot;
        let mut var_guard514: f64 = *var_guard514_slot;
        let mut var_guard515: f64 = *var_guard515_slot;
        let mut var_guard516: f64 = *var_guard516_slot;
        let mut var_guard517: f64 = *var_guard517_slot;
        let mut var_guard518: f64 = *var_guard518_slot;
        let mut var_guard519: f64 = *var_guard519_slot;
        let mut var_guard520: f64 = *var_guard520_slot;
        let mut var_guard521: f64 = *var_guard521_slot;
        let mut var_i4_cor: f64 = *var_i4_cor_slot;
        let mut var_i4_cor_dn5: f64 = *var_i4_cor_dn5_slot;
        let mut var_i4_cor_dn6: f64 = *var_i4_cor_dn6_slot;
        let mut var_i4_cor_dn7: f64 = *var_i4_cor_dn7_slot;
        let mut var_i4_cor_dn8: f64 = *var_i4_cor_dn8_slot;
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
        let mut var_isatfor1_s: f64 = *var_isatfor1_s_slot;
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

        let (assign26990_e31822, assign26990_e31822_d_n5, assign26990_e31822_d_n6, assign26990_e31822_d_n7, assign26990_e31822_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) && (var_guard512 == 0.0)) {
        let assign26990_e31789: f64 = (-230.25850929940458);
        let assign26990_e31791: f64 = (-var_ysq);
        let assign26990_e31793: f64 = (assign26990_e31791 + var_mtat);
        let assign26990_e31794: f64 = (assign26990_e31789 - assign26990_e31793);
        let assign26990_e31798: f64 = (-230.25850929940458);
        let assign26990_e31800: f64 = (-var_ysq);
        let assign26990_e31802: f64 = (assign26990_e31800 + var_mtat);
        let assign26990_e31803: f64 = (assign26990_e31798 - assign26990_e31802);
        let assign26990_e31806: f64 = (-230.25850929940458);
        let assign26990_e31808: f64 = (-var_ysq);
        let assign26990_e31810: f64 = (assign26990_e31808 + var_mtat);
        let assign26990_e31811: f64 = (assign26990_e31806 - assign26990_e31810);
        let assign26990_e31813: f64 = (assign26990_e31811 * 0.3333333333333333);
        let assign26990_e31814: f64 = (1.0 + assign26990_e31813);
        let assign26990_e31815: f64 = (assign26990_e31803 * assign26990_e31814);
        let assign26990_e31816: f64 = (0.5 * assign26990_e31815);
        let assign26990_e31817: f64 = (1.0 + assign26990_e31816);
        let assign26990_e31818: f64 = (assign26990_e31794 * assign26990_e31817);
        let assign26990_e31819: f64 = (1.0 + assign26990_e31818);
        let assign26990_e31820: f64 = (1e-100 / assign26990_e31819);
        (assign26990_e31820, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign26990_e31817) + (assign26990_e31794 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign26990_e31814) + (assign26990_e31803 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign26990_e31819 * assign26990_e31819))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign26990_e31817) + (assign26990_e31794 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign26990_e31814) + (assign26990_e31803 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign26990_e31819 * assign26990_e31819))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign26990_e31817) + (assign26990_e31794 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign26990_e31814) + (assign26990_e31803 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign26990_e31819 * assign26990_e31819))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign26990_e31817) + (assign26990_e31794 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign26990_e31814) + (assign26990_e31803 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign26990_e31819 * assign26990_e31819))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26990_e31822;
        var_tmp_dn5 = assign26990_e31822_d_n5;
        var_tmp_dn6 = assign26990_e31822_d_n6;
        var_tmp_dn7 = assign26990_e31822_d_n7;
        var_tmp_dn8 = assign26990_e31822_d_n8;

        let (assign27000_e31852, assign27000_e31852_d_n5, assign27000_e31852_d_n6, assign27000_e31852_d_n7, assign27000_e31852_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) {
        let assign27000_e31834: f64 = (0.29214664 * var_terfc);
        let assign27000_e31838: f64 = (var_terfc * var_terfc);
        let assign27000_e31839: f64 = (var_berfc * assign27000_e31838);
        let assign27000_e31840: f64 = (assign27000_e31834 + assign27000_e31839);
        let assign27000_e31844: f64 = (var_terfc * var_terfc);
        let assign27000_e31846: f64 = (assign27000_e31844 * var_terfc);
        let assign27000_e31847: f64 = (var_cerfc * assign27000_e31846);
        let assign27000_e31848: f64 = (assign27000_e31840 + assign27000_e31847);
        let assign27000_e31850: f64 = (assign27000_e31848 * var_tmp);
        (assign27000_e31850, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign27000_e31844 * var_terfc_dn5)))) * var_tmp) + (assign27000_e31848 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign27000_e31844 * var_terfc_dn6)))) * var_tmp) + (assign27000_e31848 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign27000_e31844 * var_terfc_dn7)))) * var_tmp) + (assign27000_e31848 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign27000_e31844 * var_terfc_dn8)))) * var_tmp) + (assign27000_e31848 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign27000_e31852;
        var_erfcpos_dn5 = assign27000_e31852_d_n5;
        var_erfcpos_dn6 = assign27000_e31852_d_n6;
        var_erfcpos_dn7 = assign27000_e31852_d_n7;
        var_erfcpos_dn8 = assign27000_e31852_d_n8;

        let assign27010_e31855: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard513 = assign27010_e31855;

        let (assign27020_e31869, assign27020_e31869_d_n5, assign27020_e31869_d_n6, assign27020_e31869_d_n7, assign27020_e31869_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) && (var_guard513 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign27020_e31869;
        var_erfctimesexpmtat_dn5 = assign27020_e31869_d_n5;
        var_erfctimesexpmtat_dn6 = assign27020_e31869_d_n6;
        var_erfctimesexpmtat_dn7 = assign27020_e31869_d_n7;
        var_erfctimesexpmtat_dn8 = assign27020_e31869_d_n8;

        let assign27030_e31872: f64 = (-230.25850929940458);
        let assign27030_e31873: f64 = if var_mtat > assign27030_e31872 { 1.0 } else { 0.0 };
        var_guard514 = assign27030_e31873;

        let (assign27040_e31891, assign27040_e31891_d_n5, assign27040_e31891_d_n6, assign27040_e31891_d_n7, assign27040_e31891_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) && (var_guard513 == 0.0)) && (var_guard514 != 0.0)) {
        let assign27040_e31889: f64 = (var_mtat).exp();
        (assign27040_e31889, (assign27040_e31889 * var_mtat_dn5), (assign27040_e31889 * var_mtat_dn6), (assign27040_e31889 * var_mtat_dn7), (assign27040_e31889 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27040_e31891;
        var_tmp_dn5 = assign27040_e31891_d_n5;
        var_tmp_dn6 = assign27040_e31891_d_n6;
        var_tmp_dn7 = assign27040_e31891_d_n7;
        var_tmp_dn8 = assign27040_e31891_d_n8;

        let (assign27050_e31934, assign27050_e31934_d_n5, assign27050_e31934_d_n6, assign27050_e31934_d_n7, assign27050_e31934_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) && (var_guard513 == 0.0)) && (var_guard514 == 0.0)) {
        let assign27050_e31910: f64 = (-230.25850929940458);
        let assign27050_e31912: f64 = (assign27050_e31910 - var_mtat);
        let assign27050_e31916: f64 = (-230.25850929940458);
        let assign27050_e31918: f64 = (assign27050_e31916 - var_mtat);
        let assign27050_e31921: f64 = (-230.25850929940458);
        let assign27050_e31923: f64 = (assign27050_e31921 - var_mtat);
        let assign27050_e31925: f64 = (assign27050_e31923 * 0.3333333333333333);
        let assign27050_e31926: f64 = (1.0 + assign27050_e31925);
        let assign27050_e31927: f64 = (assign27050_e31918 * assign27050_e31926);
        let assign27050_e31928: f64 = (0.5 * assign27050_e31927);
        let assign27050_e31929: f64 = (1.0 + assign27050_e31928);
        let assign27050_e31930: f64 = (assign27050_e31912 * assign27050_e31929);
        let assign27050_e31931: f64 = (1.0 + assign27050_e31930);
        let assign27050_e31932: f64 = (1e-100 / assign27050_e31931);
        (assign27050_e31932, (-((1e-100 * (((-var_mtat_dn5) * assign27050_e31929) + (assign27050_e31912 * (0.5 * (((-var_mtat_dn5) * assign27050_e31926) + (assign27050_e31918 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign27050_e31931 * assign27050_e31931))), (-((1e-100 * (((-var_mtat_dn6) * assign27050_e31929) + (assign27050_e31912 * (0.5 * (((-var_mtat_dn6) * assign27050_e31926) + (assign27050_e31918 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign27050_e31931 * assign27050_e31931))), (-((1e-100 * (((-var_mtat_dn7) * assign27050_e31929) + (assign27050_e31912 * (0.5 * (((-var_mtat_dn7) * assign27050_e31926) + (assign27050_e31918 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign27050_e31931 * assign27050_e31931))), (-((1e-100 * (((-var_mtat_dn8) * assign27050_e31929) + (assign27050_e31912 * (0.5 * (((-var_mtat_dn8) * assign27050_e31926) + (assign27050_e31918 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign27050_e31931 * assign27050_e31931))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27050_e31934;
        var_tmp_dn5 = assign27050_e31934_d_n5;
        var_tmp_dn6 = assign27050_e31934_d_n6;
        var_tmp_dn7 = assign27050_e31934_d_n7;
        var_tmp_dn8 = assign27050_e31934_d_n8;

        let (assign27060_e31953, assign27060_e31953_d_n5, assign27060_e31953_d_n6, assign27060_e31953_d_n7, assign27060_e31953_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) && (var_guard513 == 0.0)) {
        let assign27060_e31949: f64 = (2.0 * var_tmp);
        let assign27060_e31951: f64 = (assign27060_e31949 - var_erfcpos);
        (assign27060_e31951, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign27060_e31953;
        var_erfctimesexpmtat_dn5 = assign27060_e31953_d_n5;
        var_erfctimesexpmtat_dn6 = assign27060_e31953_d_n6;
        var_erfctimesexpmtat_dn7 = assign27060_e31953_d_n7;
        var_erfctimesexpmtat_dn8 = assign27060_e31953_d_n8;

        let (assign27070_e31973, assign27070_e31973_d_n5, assign27070_e31973_d_n6, assign27070_e31973_d_n7, assign27070_e31973_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) {
        let assign27070_e31965: f64 = (1.772453850905516 * 0.5);
        let assign27070_e31968: f64 = (var_atatgat * var_erfctimesexpmtat);
        let assign27070_e31970: f64 = (assign27070_e31968 / var_ktat);
        let assign27070_e31971: f64 = (assign27070_e31965 * assign27070_e31970);
        (assign27070_e31971, (assign27070_e31965 * ((((var_atatgat * var_erfctimesexpmtat_dn5) * var_ktat) - (assign27070_e31968 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign27070_e31965 * ((((var_atatgat * var_erfctimesexpmtat_dn6) * var_ktat) - (assign27070_e31968 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign27070_e31965 * ((((var_atatgat * var_erfctimesexpmtat_dn7) * var_ktat) - (assign27070_e31968 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign27070_e31965 * ((((var_atatgat * var_erfctimesexpmtat_dn8) * var_ktat) - (assign27070_e31968 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign27070_e31973;
        var_gammamax_dn5 = assign27070_e31973_d_n5;
        var_gammamax_dn6 = assign27070_e31973_d_n6;
        var_gammamax_dn7 = assign27070_e31973_d_n7;
        var_gammamax_dn8 = assign27070_e31973_d_n8;

        let (assign27080_e31991, assign27080_e31991_d_n5, assign27080_e31991_d_n6, assign27080_e31991_d_n7, assign27080_e31991_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard509 == 0.0)) {
        let assign27080_e31986: f64 = (var_asrh * var_gammamax);
        let assign27080_e31988: f64 = (assign27080_e31986 * var_wtat);
        let assign27080_e31989: f64 = (p.p840 * assign27080_e31988);
        (assign27080_e31989, (p.p840 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign27080_e31986 * var_wtat_dn5))), (p.p840 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign27080_e31986 * var_wtat_dn6))), (p.p840 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign27080_e31986 * var_wtat_dn7))), (p.p840 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign27080_e31986 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign27080_e31991;
        var_itat_dn5 = assign27080_e31991_d_n5;
        var_itat_dn6 = assign27080_e31991_d_n6;
        var_itat_dn7 = assign27080_e31991_d_n7;
        var_itat_dn8 = assign27080_e31991_d_n8;

        let assign27090_e31994: f64 = if p.p846 == 0.0 { 1.0 } else { 0.0 };
        var_guard515 = assign27090_e31994;

        let (assign27100_e32005, assign27100_e32005_d_n5, assign27100_e32005_d_n6, assign27100_e32005_d_n7, assign27100_e32005_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard515 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign27100_e32005;
        var_ibbt_dn5 = assign27100_e32005_d_n5;
        var_ibbt_dn6 = assign27100_e32005_d_n6;
        var_ibbt_dn7 = assign27100_e32005_d_n7;
        var_ibbt_dn8 = assign27100_e32005_d_n8;

        let assign27110_e32008: f64 = if p.p826 == 0.5 { 1.0 } else { 0.0 };
        var_guard516 = assign27110_e32008;

        let (assign27120_e32027, assign27120_e32027_d_n5, assign27120_e32027_d_n6, assign27120_e32027_d_n7, assign27120_e32027_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard515 == 0.0)) && (var_guard516 != 0.0)) {
        let assign27120_e32022: f64 = (p.p823 - var_vbbt);
        let assign27120_e32024: f64 = (assign27120_e32022 * var_vbirgatinv);
        let assign27120_e32025: f64 = (assign27120_e32024).sqrt();
        (assign27120_e32025, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27120_e32027;
        var_tmp_dn5 = assign27120_e32027_d_n5;
        var_tmp_dn6 = assign27120_e32027_d_n6;
        var_tmp_dn7 = assign27120_e32027_d_n7;
        var_tmp_dn8 = assign27120_e32027_d_n8;

        let (assign27130_e32048, assign27130_e32048_d_n5, assign27130_e32048_d_n6, assign27130_e32048_d_n7, assign27130_e32048_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard515 == 0.0)) && (var_guard516 == 0.0)) {
        let assign27130_e32042: f64 = (p.p823 - var_vbbt);
        let assign27130_e32044: f64 = (assign27130_e32042 * var_vbirgatinv);
        let assign27130_e32046: f64 = (assign27130_e32044).powf(p.p826);
        (assign27130_e32046, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27130_e32048;
        var_tmp_dn5 = assign27130_e32048_d_n5;
        var_tmp_dn6 = assign27130_e32048_d_n6;
        var_tmp_dn7 = assign27130_e32048_d_n7;
        var_tmp_dn8 = assign27130_e32048_d_n8;

        let (assign27140_e32068, assign27140_e32068_d_n5, assign27140_e32068_d_n6, assign27140_e32068_d_n7, assign27140_e32068_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard515 == 0.0)) {
        let assign27140_e32061: f64 = (p.p823 - var_vbbt);
        let assign27140_e32063: f64 = (assign27140_e32061 * var_wdepnulrinvgat);
        let assign27140_e32065: f64 = (assign27140_e32063 / var_tmp);
        let assign27140_e32066: f64 = (var_one_over_one_minus_pgat * assign27140_e32065);
        (assign27140_e32066, (var_one_over_one_minus_pgat * (-((assign27140_e32063 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign27140_e32063 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign27140_e32063 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign27140_e32063 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign27140_e32068;
        var_fmaxr_dn5 = assign27140_e32068_d_n5;
        var_fmaxr_dn6 = assign27140_e32068_d_n6;
        var_fmaxr_dn7 = assign27140_e32068_d_n7;
        var_fmaxr_dn8 = assign27140_e32068_d_n8;

        let assign27150_e32070: f64 = (-var_fbbtgat);
        let assign27150_e32072: f64 = (assign27150_e32070 / var_fmaxr);
        let assign27150_e32073: f64 = (assign27150_e32072).abs();
        let assign27150_e32075: f64 = if assign27150_e32073 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard517 = assign27150_e32075;

        let (assign27160_e32093, assign27160_e32093_d_n5, assign27160_e32093_d_n6, assign27160_e32093_d_n7, assign27160_e32093_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard515 == 0.0)) && (var_guard517 != 0.0)) {
        let assign27160_e32088: f64 = (-var_fbbtgat);
        let assign27160_e32090: f64 = (assign27160_e32088 / var_fmaxr);
        let assign27160_e32091: f64 = (assign27160_e32090).exp();
        (assign27160_e32091, (assign27160_e32091 * ((((-var_fbbtgat_dn5) * var_fmaxr) - (assign27160_e32088 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign27160_e32091 * ((((-var_fbbtgat_dn6) * var_fmaxr) - (assign27160_e32088 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign27160_e32091 * ((((-var_fbbtgat_dn7) * var_fmaxr) - (assign27160_e32088 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign27160_e32091 * ((((-var_fbbtgat_dn8) * var_fmaxr) - (assign27160_e32088 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27160_e32093;
        var_tmp_dn5 = assign27160_e32093_d_n5;
        var_tmp_dn6 = assign27160_e32093_d_n6;
        var_tmp_dn7 = assign27160_e32093_d_n7;
        var_tmp_dn8 = assign27160_e32093_d_n8;

        let assign27170_e32095: f64 = (-var_fbbtgat);
        let assign27170_e32097: f64 = (assign27170_e32095 / var_fmaxr);
        let assign27170_e32099: f64 = if assign27170_e32097 < 0.0 { 1.0 } else { 0.0 };
        var_guard518 = assign27170_e32099;

        let (assign27180_e32150, assign27180_e32150_d_n5, assign27180_e32150_d_n6, assign27180_e32150_d_n7, assign27180_e32150_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard515 == 0.0)) && (var_guard517 == 0.0)) && (var_guard518 != 0.0)) {
        let assign27180_e32117: f64 = (-230.25850929940458);
        let assign27180_e32119: f64 = (-var_fbbtgat);
        let assign27180_e32121: f64 = (assign27180_e32119 / var_fmaxr);
        let assign27180_e32122: f64 = (assign27180_e32117 - assign27180_e32121);
        let assign27180_e32126: f64 = (-230.25850929940458);
        let assign27180_e32128: f64 = (-var_fbbtgat);
        let assign27180_e32130: f64 = (assign27180_e32128 / var_fmaxr);
        let assign27180_e32131: f64 = (assign27180_e32126 - assign27180_e32130);
        let assign27180_e32134: f64 = (-230.25850929940458);
        let assign27180_e32136: f64 = (-var_fbbtgat);
        let assign27180_e32138: f64 = (assign27180_e32136 / var_fmaxr);
        let assign27180_e32139: f64 = (assign27180_e32134 - assign27180_e32138);
        let assign27180_e32141: f64 = (assign27180_e32139 * 0.3333333333333333);
        let assign27180_e32142: f64 = (1.0 + assign27180_e32141);
        let assign27180_e32143: f64 = (assign27180_e32131 * assign27180_e32142);
        let assign27180_e32144: f64 = (0.5 * assign27180_e32143);
        let assign27180_e32145: f64 = (1.0 + assign27180_e32144);
        let assign27180_e32146: f64 = (assign27180_e32122 * assign27180_e32145);
        let assign27180_e32147: f64 = (1.0 + assign27180_e32146);
        let assign27180_e32148: f64 = (1e-100 / assign27180_e32147);
        (assign27180_e32148, (-((1e-100 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign27180_e32119 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign27180_e32145) + (assign27180_e32122 * (0.5 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign27180_e32128 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign27180_e32142) + (assign27180_e32131 * ((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign27180_e32136 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign27180_e32147 * assign27180_e32147))), (-((1e-100 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign27180_e32119 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign27180_e32145) + (assign27180_e32122 * (0.5 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign27180_e32128 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign27180_e32142) + (assign27180_e32131 * ((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign27180_e32136 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign27180_e32147 * assign27180_e32147))), (-((1e-100 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign27180_e32119 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign27180_e32145) + (assign27180_e32122 * (0.5 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign27180_e32128 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign27180_e32142) + (assign27180_e32131 * ((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign27180_e32136 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign27180_e32147 * assign27180_e32147))), (-((1e-100 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign27180_e32119 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign27180_e32145) + (assign27180_e32122 * (0.5 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign27180_e32128 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign27180_e32142) + (assign27180_e32131 * ((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign27180_e32136 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign27180_e32147 * assign27180_e32147))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27180_e32150;
        var_tmp_dn5 = assign27180_e32150_d_n5;
        var_tmp_dn6 = assign27180_e32150_d_n6;
        var_tmp_dn7 = assign27180_e32150_d_n7;
        var_tmp_dn8 = assign27180_e32150_d_n8;

        let (assign27190_e32199, assign27190_e32199_d_n5, assign27190_e32199_d_n6, assign27190_e32199_d_n7, assign27190_e32199_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard515 == 0.0)) && (var_guard517 == 0.0)) && (var_guard518 == 0.0)) {
        let assign27190_e32169: f64 = (-var_fbbtgat);
        let assign27190_e32171: f64 = (assign27190_e32169 / var_fmaxr);
        let assign27190_e32173: f64 = (assign27190_e32171 - 230.25850929940458);
        let assign27190_e32177: f64 = (-var_fbbtgat);
        let assign27190_e32179: f64 = (assign27190_e32177 / var_fmaxr);
        let assign27190_e32181: f64 = (assign27190_e32179 - 230.25850929940458);
        let assign27190_e32184: f64 = (-var_fbbtgat);
        let assign27190_e32186: f64 = (assign27190_e32184 / var_fmaxr);
        let assign27190_e32188: f64 = (assign27190_e32186 - 230.25850929940458);
        let assign27190_e32190: f64 = (assign27190_e32188 * 0.3333333333333333);
        let assign27190_e32191: f64 = (1.0 + assign27190_e32190);
        let assign27190_e32192: f64 = (assign27190_e32181 * assign27190_e32191);
        let assign27190_e32193: f64 = (0.5 * assign27190_e32192);
        let assign27190_e32194: f64 = (1.0 + assign27190_e32193);
        let assign27190_e32195: f64 = (assign27190_e32173 * assign27190_e32194);
        let assign27190_e32196: f64 = (1.0 + assign27190_e32195);
        let assign27190_e32197: f64 = (1e100 * assign27190_e32196);
        (assign27190_e32197, (1e100 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign27190_e32169 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign27190_e32194) + (assign27190_e32173 * (0.5 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign27190_e32177 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign27190_e32191) + (assign27190_e32181 * (((((-var_fbbtgat_dn5) * var_fmaxr) - (assign27190_e32184 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign27190_e32169 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign27190_e32194) + (assign27190_e32173 * (0.5 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign27190_e32177 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign27190_e32191) + (assign27190_e32181 * (((((-var_fbbtgat_dn6) * var_fmaxr) - (assign27190_e32184 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign27190_e32169 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign27190_e32194) + (assign27190_e32173 * (0.5 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign27190_e32177 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign27190_e32191) + (assign27190_e32181 * (((((-var_fbbtgat_dn7) * var_fmaxr) - (assign27190_e32184 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign27190_e32169 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign27190_e32194) + (assign27190_e32173 * (0.5 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign27190_e32177 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign27190_e32191) + (assign27190_e32181 * (((((-var_fbbtgat_dn8) * var_fmaxr) - (assign27190_e32184 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27190_e32199;
        var_tmp_dn5 = assign27190_e32199_d_n5;
        var_tmp_dn6 = assign27190_e32199_d_n6;
        var_tmp_dn7 = assign27190_e32199_d_n7;
        var_tmp_dn8 = assign27190_e32199_d_n8;

        let (assign27200_e32219, assign27200_e32219_d_n5, assign27200_e32219_d_n6, assign27200_e32219_d_n7, assign27200_e32219_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard515 == 0.0)) {
        let assign27200_e32212: f64 = (var_v5 * var_fmaxr);
        let assign27200_e32214: f64 = (assign27200_e32212 * var_fmaxr);
        let assign27200_e32216: f64 = (assign27200_e32214 * var_tmp);
        let assign27200_e32217: f64 = (p.p846 * assign27200_e32216);
        (assign27200_e32217, (p.p846 * (((((var_v5 * var_fmaxr_dn5) * var_fmaxr) + (assign27200_e32212 * var_fmaxr_dn5)) * var_tmp) + (assign27200_e32214 * var_tmp_dn5))), (p.p846 * (((((var_v5 * var_fmaxr_dn6) * var_fmaxr) + (assign27200_e32212 * var_fmaxr_dn6)) * var_tmp) + (assign27200_e32214 * var_tmp_dn6))), (p.p846 * (((((var_v5 * var_fmaxr_dn7) * var_fmaxr) + (assign27200_e32212 * var_fmaxr_dn7)) * var_tmp) + (assign27200_e32214 * var_tmp_dn7))), (p.p846 * (((((var_v5 * var_fmaxr_dn8) * var_fmaxr) + (assign27200_e32212 * var_fmaxr_dn8)) * var_tmp) + (assign27200_e32214 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign27200_e32219;
        var_ibbt_dn5 = assign27200_e32219_d_n5;
        var_ibbt_dn6 = assign27200_e32219_d_n6;
        var_ibbt_dn7 = assign27200_e32219_d_n7;
        var_ibbt_dn8 = assign27200_e32219_d_n8;

        let assign27210_e32222: f64 = if p.p855 > 1000.0 { 1.0 } else { 0.0 };
        var_guard519 = assign27210_e32222;

        let (assign27220_e32233, assign27220_e32233_d_n5, assign27220_e32233_d_n6, assign27220_e32233_d_n7, assign27220_e32233_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard519 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign27220_e32233;
        var_fbreakdown_dn5 = assign27220_e32233_d_n5;
        var_fbreakdown_dn6 = assign27220_e32233_d_n6;
        var_fbreakdown_dn7 = assign27220_e32233_d_n7;
        var_fbreakdown_dn8 = assign27220_e32233_d_n8;

        let assign27230_e32236: f64 = (-var_alphaav);
        let assign27230_e32238: f64 = (assign27230_e32236 * p.p855);
        let assign27230_e32239: f64 = if var_vav > assign27230_e32238 { 1.0 } else { 0.0 };
        var_guard520 = assign27230_e32239;

        let assign27240_e32242: f64 = if p.p858 == 4.0 { 1.0 } else { 0.0 };
        var_guard521 = assign27240_e32242;

        let (assign27250_e32272, assign27250_e32272_d_n5, assign27250_e32272_d_n6, assign27250_e32272_d_n7, assign27250_e32272_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard519 == 0.0)) && (var_guard520 != 0.0)) && (var_guard521 != 0.0)) {
        let assign27250_e32258: f64 = (var_vav * var_vbrinvgat);
        let assign27250_e32261: f64 = (var_vav * var_vbrinvgat);
        let assign27250_e32262: f64 = (assign27250_e32258 * assign27250_e32261);
        let assign27250_e32265: f64 = (var_vav * var_vbrinvgat);
        let assign27250_e32266: f64 = (assign27250_e32262 * assign27250_e32265);
        let assign27250_e32269: f64 = (var_vav * var_vbrinvgat);
        let assign27250_e32270: f64 = (assign27250_e32266 * assign27250_e32269);
        (assign27250_e32270, (((((((var_vav * var_vbrinvgat_dn5) * assign27250_e32261) + (assign27250_e32258 * (var_vav * var_vbrinvgat_dn5))) * assign27250_e32265) + (assign27250_e32262 * (var_vav * var_vbrinvgat_dn5))) * assign27250_e32269) + (assign27250_e32266 * (var_vav * var_vbrinvgat_dn5))), (((((((var_vav * var_vbrinvgat_dn6) * assign27250_e32261) + (assign27250_e32258 * (var_vav * var_vbrinvgat_dn6))) * assign27250_e32265) + (assign27250_e32262 * (var_vav * var_vbrinvgat_dn6))) * assign27250_e32269) + (assign27250_e32266 * (var_vav * var_vbrinvgat_dn6))), (((((((var_vav * var_vbrinvgat_dn7) * assign27250_e32261) + (assign27250_e32258 * (var_vav * var_vbrinvgat_dn7))) * assign27250_e32265) + (assign27250_e32262 * (var_vav * var_vbrinvgat_dn7))) * assign27250_e32269) + (assign27250_e32266 * (var_vav * var_vbrinvgat_dn7))), (((((((var_vav * var_vbrinvgat_dn8) * assign27250_e32261) + (assign27250_e32258 * (var_vav * var_vbrinvgat_dn8))) * assign27250_e32265) + (assign27250_e32262 * (var_vav * var_vbrinvgat_dn8))) * assign27250_e32269) + (assign27250_e32266 * (var_vav * var_vbrinvgat_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27250_e32272;
        var_tmp_dn5 = assign27250_e32272_d_n5;
        var_tmp_dn6 = assign27250_e32272_d_n6;
        var_tmp_dn7 = assign27250_e32272_d_n7;
        var_tmp_dn8 = assign27250_e32272_d_n8;

        let (assign27260_e32294, assign27260_e32294_d_n5, assign27260_e32294_d_n6, assign27260_e32294_d_n7, assign27260_e32294_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard519 == 0.0)) && (var_guard520 != 0.0)) && (var_guard521 == 0.0)) {
        let assign27260_e32289: f64 = (var_vav * var_vbrinvgat);
        let assign27260_e32290: f64 = (assign27260_e32289).abs();
        let assign27260_e32292: f64 = (assign27260_e32290).powf(p.p858);
        (assign27260_e32292, if 0.0 == 0.0 && ((p.p858) as f64).is_finite() && ((p.p858) as f64).fract() == 0.0 { if p.p858 == 0.0 { 0.0 } else { (p.p858 * ((assign27260_e32290).powf(p.p858 - 1.0) * if assign27260_e32289 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) })) } } else { (assign27260_e32292 * (p.p858 * (if assign27260_e32289 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) } / assign27260_e32290))) }, if 0.0 == 0.0 && ((p.p858) as f64).is_finite() && ((p.p858) as f64).fract() == 0.0 { if p.p858 == 0.0 { 0.0 } else { (p.p858 * ((assign27260_e32290).powf(p.p858 - 1.0) * if assign27260_e32289 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) })) } } else { (assign27260_e32292 * (p.p858 * (if assign27260_e32289 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) } / assign27260_e32290))) }, if 0.0 == 0.0 && ((p.p858) as f64).is_finite() && ((p.p858) as f64).fract() == 0.0 { if p.p858 == 0.0 { 0.0 } else { (p.p858 * ((assign27260_e32290).powf(p.p858 - 1.0) * if assign27260_e32289 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) })) } } else { (assign27260_e32292 * (p.p858 * (if assign27260_e32289 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) } / assign27260_e32290))) }, if 0.0 == 0.0 && ((p.p858) as f64).is_finite() && ((p.p858) as f64).fract() == 0.0 { if p.p858 == 0.0 { 0.0 } else { (p.p858 * ((assign27260_e32290).powf(p.p858 - 1.0) * if assign27260_e32289 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) })) } } else { (assign27260_e32292 * (p.p858 * (if assign27260_e32289 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) } / assign27260_e32290))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27260_e32294;
        var_tmp_dn5 = assign27260_e32294_d_n5;
        var_tmp_dn6 = assign27260_e32294_d_n6;
        var_tmp_dn7 = assign27260_e32294_d_n7;
        var_tmp_dn8 = assign27260_e32294_d_n8;

        let (assign27270_e32312, assign27270_e32312_d_n5, assign27270_e32312_d_n6, assign27270_e32312_d_n7, assign27270_e32312_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard519 == 0.0)) && (var_guard520 != 0.0)) {
        let assign27270_e32309: f64 = (1.0 - var_tmp);
        let assign27270_e32310: f64 = (1.0 / assign27270_e32309);
        (assign27270_e32310, (-((-var_tmp_dn5) / (assign27270_e32309 * assign27270_e32309))), (-((-var_tmp_dn6) / (assign27270_e32309 * assign27270_e32309))), (-((-var_tmp_dn7) / (assign27270_e32309 * assign27270_e32309))), (-((-var_tmp_dn8) / (assign27270_e32309 * assign27270_e32309))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign27270_e32312;
        var_fbreakdown_dn5 = assign27270_e32312_d_n5;
        var_fbreakdown_dn6 = assign27270_e32312_d_n6;
        var_fbreakdown_dn7 = assign27270_e32312_d_n7;
        var_fbreakdown_dn8 = assign27270_e32312_d_n8;

        let (assign27280_e32335, assign27280_e32335_d_n5, assign27280_e32335_d_n6, assign27280_e32335_d_n7, assign27280_e32335_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) && (var_guard519 == 0.0)) && (var_guard520 == 0.0)) {
        let assign27280_e32329: f64 = (var_alphaav * p.p855);
        let assign27280_e32330: f64 = (var_vav + assign27280_e32329);
        let assign27280_e32332: f64 = (assign27280_e32330 * var_slopegat);
        let assign27280_e32333: f64 = (var_fstopgat + assign27280_e32332);
        (assign27280_e32333, (assign27280_e32330 * var_slopegat_dn5), (assign27280_e32330 * var_slopegat_dn6), (assign27280_e32330 * var_slopegat_dn7), (assign27280_e32330 * var_slopegat_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign27280_e32335;
        var_fbreakdown_dn5 = assign27280_e32335_d_n5;
        var_fbreakdown_dn6 = assign27280_e32335_d_n6;
        var_fbreakdown_dn7 = assign27280_e32335_d_n7;
        var_fbreakdown_dn8 = assign27280_e32335_d_n8;

        let (assign27290_e32354, assign27290_e32354_d_n5, assign27290_e32354_d_n6, assign27290_e32354_d_n7, assign27290_e32354_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard505 == 0.0)) {
        let assign27290_e32345: f64 = (var_id__blk213 + var_isrh);
        let assign27290_e32347: f64 = (assign27290_e32345 + var_itat);
        let assign27290_e32349: f64 = (assign27290_e32347 + var_ibbt);
        let assign27290_e32350: f64 = (p.p29 * assign27290_e32349);
        let assign27290_e32352: f64 = (assign27290_e32350 * var_fbreakdown);
        (assign27290_e32352, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign27290_e32350 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign27290_e32350 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign27290_e32350 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign27290_e32350 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign27290_e32354;
        var_ijungat_dn5 = assign27290_e32354_d_n5;
        var_ijungat_dn6 = assign27290_e32354_d_n6;
        var_ijungat_dn7 = assign27290_e32354_d_n7;
        var_ijungat_dn8 = assign27290_e32354_d_n8;

        let (assign27300_e32370, assign27300_e32370_d_n5, assign27300_e32370_d_n6, assign27300_e32370_d_n7, assign27300_e32370_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign27300_e32360: f64 = (var_absource_i * var_ijunbot);
        let assign27300_e32363: f64 = (var_lssource_i * var_ijunsti);
        let assign27300_e32364: f64 = (assign27300_e32360 + assign27300_e32363);
        let assign27300_e32367: f64 = (var_lgsource_i * var_ijungat);
        let assign27300_e32368: f64 = (assign27300_e32364 + assign27300_e32367);
        (assign27300_e32368, (((var_absource_i * var_ijunbot_dn5) + (var_lssource_i * var_ijunsti_dn5)) + (var_lgsource_i * var_ijungat_dn5)), (((var_absource_i * var_ijunbot_dn6) + (var_lssource_i * var_ijunsti_dn6)) + (var_lgsource_i * var_ijungat_dn6)), (((var_absource_i * var_ijunbot_dn7) + (var_lssource_i * var_ijunsti_dn7)) + (var_lgsource_i * var_ijungat_dn7)), (((var_absource_i * var_ijunbot_dn8) + (var_lssource_i * var_ijunsti_dn8)) + (var_lgsource_i * var_ijungat_dn8)),)
    } else {
        (var_i5, var_i5_dn5, var_i5_dn6, var_i5_dn7, var_i5_dn8,)
    }
};
        var_i5 = assign27300_e32370;
        var_i5_dn5 = assign27300_e32370_d_n5;
        var_i5_dn6 = assign27300_e32370_d_n6;
        var_i5_dn7 = assign27300_e32370_d_n7;
        var_i5_dn8 = assign27300_e32370_d_n8;

        let (assign27310_e32386,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign27310_e32376: f64 = (var_absource_i * var_idsatbot);
        let assign27310_e32379: f64 = (var_lssource_i * var_idsatsti);
        let assign27310_e32380: f64 = (assign27310_e32376 + assign27310_e32379);
        let assign27310_e32383: f64 = (var_lgsource_i * var_idsatgat);
        let assign27310_e32384: f64 = (assign27310_e32380 + assign27310_e32383);
        (assign27310_e32384,)
    } else {
        (var_isatfor1_s,)
    }
};
        var_isatfor1_s = assign27310_e32386;

        let (assign27320_e32403, assign27320_e32403_d_n5, assign27320_e32403_d_n6, assign27320_e32403_d_n7, assign27320_e32403_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign27320_e32394: f64 = (var_v4 * var_phitdinv);
        let assign27320_e32396: f64 = (assign27320_e32394 * var_mfor1_s);
        let assign27320_e32397: f64 = (assign27320_e32396).exp();
        let assign27320_e32399: f64 = (assign27320_e32397 - 1.0);
        let assign27320_e32400: f64 = (var_isatfor1_s * assign27320_e32399);
        let assign27320_e32401: f64 = (var_i4 - assign27320_e32400);
        (assign27320_e32401, var_i4_dn5, var_i4_dn6, var_i4_dn7, var_i4_dn8,)
    } else {
        (var_i4_cor, var_i4_cor_dn5, var_i4_cor_dn6, var_i4_cor_dn7, var_i4_cor_dn8,)
    }
};
        var_i4_cor = assign27320_e32403;
        var_i4_cor_dn5 = assign27320_e32403_d_n5;
        var_i4_cor_dn6 = assign27320_e32403_d_n6;
        var_i4_cor_dn7 = assign27320_e32403_d_n7;
        var_i4_cor_dn8 = assign27320_e32403_d_n8;

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
        *var_guard513_slot = var_guard513;
        *var_guard514_slot = var_guard514;
        *var_guard515_slot = var_guard515;
        *var_guard516_slot = var_guard516;
        *var_guard517_slot = var_guard517;
        *var_guard518_slot = var_guard518;
        *var_guard519_slot = var_guard519;
        *var_guard520_slot = var_guard520;
        *var_guard521_slot = var_guard521;
        *var_i4_cor_slot = var_i4_cor;
        *var_i4_cor_dn5_slot = var_i4_cor_dn5;
        *var_i4_cor_dn6_slot = var_i4_cor_dn6;
        *var_i4_cor_dn7_slot = var_i4_cor_dn7;
        *var_i4_cor_dn8_slot = var_i4_cor_dn8;
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
        *var_isatfor1_s_slot = var_isatfor1_s;
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

    pub(super) fn stamp_transient_block_53(
        p: &Parameters,
        var_absource_i: f64,
        var_cjobot: f64,
        var_cjogat: f64,
        var_cjosti: f64,
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
        var_i4_cor: f64,
        var_i4_cor_dn5: f64,
        var_i4_cor_dn6: f64,
        var_i4_cor_dn7: f64,
        var_i4_cor_dn8: f64,
        var_i5: f64,
        var_i5_dn5: f64,
        var_i5_dn6: f64,
        var_i5_dn7: f64,
        var_i5_dn8: f64,
        var_isatfor1_s: f64,
        var_lgsource_i: f64,
        var_lssource_i: f64,
        var_mfor1_s: f64,
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
        var_expxhf1_s_slot: &mut f64,
        var_guard522_slot: &mut f64,
        var_guard523_slot: &mut f64,
        var_guard524_slot: &mut f64,
        var_guard525_slot: &mut f64,
        var_guard526_slot: &mut f64,
        var_guard527_slot: &mut f64,
        var_guard528_slot: &mut f64,
        var_guard529_slot: &mut f64,
        var_guard530_slot: &mut f64,
        var_guard531_slot: &mut f64,
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
        var_i5_cor_slot: &mut f64,
        var_i5_cor_dn5_slot: &mut f64,
        var_i5_cor_dn6_slot: &mut f64,
        var_i5_cor_dn7_slot: &mut f64,
        var_i5_cor_dn8_slot: &mut f64,
        var_isatfor2_s_slot: &mut f64,
        var_isatfor2_s_dn5_slot: &mut f64,
        var_isatfor2_s_dn6_slot: &mut f64,
        var_isatfor2_s_dn7_slot: &mut f64,
        var_isatfor2_s_dn8_slot: &mut f64,
        var_isatrev_s_slot: &mut f64,
        var_isatrev_s_dn5_slot: &mut f64,
        var_isatrev_s_dn6_slot: &mut f64,
        var_isatrev_s_dn7_slot: &mut f64,
        var_isatrev_s_dn8_slot: &mut f64,
        var_m0_rev_slot: &mut f64,
        var_m0_rev_dn5_slot: &mut f64,
        var_m0_rev_dn6_slot: &mut f64,
        var_m0_rev_dn7_slot: &mut f64,
        var_m0_rev_dn8_slot: &mut f64,
        var_m0flag_s_slot: &mut f64,
        var_mcor_rev_slot: &mut f64,
        var_mcor_rev_dn5_slot: &mut f64,
        var_mcor_rev_dn6_slot: &mut f64,
        var_mcor_rev_dn7_slot: &mut f64,
        var_mcor_rev_dn8_slot: &mut f64,
        var_mfor2_s_slot: &mut f64,
        var_mfor2_s_dn5_slot: &mut f64,
        var_mfor2_s_dn6_slot: &mut f64,
        var_mfor2_s_dn7_slot: &mut f64,
        var_mfor2_s_dn8_slot: &mut f64,
        var_mrev_s_slot: &mut f64,
        var_mrev_s_dn5_slot: &mut f64,
        var_mrev_s_dn6_slot: &mut f64,
        var_mrev_s_dn7_slot: &mut f64,
        var_mrev_s_dn8_slot: &mut f64,
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
        var_xhighf1_s_slot: &mut f64,
        var_xhighf2_s_slot: &mut f64,
        var_xhighf2_s_dn5_slot: &mut f64,
        var_xhighf2_s_dn6_slot: &mut f64,
        var_xhighf2_s_dn7_slot: &mut f64,
        var_xhighf2_s_dn8_slot: &mut f64,
        var_xhighr_s_slot: &mut f64,
        var_xhighr_s_dn5_slot: &mut f64,
        var_xhighr_s_dn6_slot: &mut f64,
        var_xhighr_s_dn7_slot: &mut f64,
        var_xhighr_s_dn8_slot: &mut f64,
        var_zflagbot_s_slot: &mut f64,
        var_zflaggat_s_slot: &mut f64,
        var_zflagsti_s_slot: &mut f64,
        var_zfrac_slot: &mut f64,
    ) {
        let mut var_alphaje: f64 = *var_alphaje_slot;
        let mut var_alphaje_dn5: f64 = *var_alphaje_dn5_slot;
        let mut var_alphaje_dn6: f64 = *var_alphaje_dn6_slot;
        let mut var_alphaje_dn7: f64 = *var_alphaje_dn7_slot;
        let mut var_alphaje_dn8: f64 = *var_alphaje_dn8_slot;
        let mut var_expxhf1_s: f64 = *var_expxhf1_s_slot;
        let mut var_guard522: f64 = *var_guard522_slot;
        let mut var_guard523: f64 = *var_guard523_slot;
        let mut var_guard524: f64 = *var_guard524_slot;
        let mut var_guard525: f64 = *var_guard525_slot;
        let mut var_guard526: f64 = *var_guard526_slot;
        let mut var_guard527: f64 = *var_guard527_slot;
        let mut var_guard528: f64 = *var_guard528_slot;
        let mut var_guard529: f64 = *var_guard529_slot;
        let mut var_guard530: f64 = *var_guard530_slot;
        let mut var_guard531: f64 = *var_guard531_slot;
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
        let mut var_i5_cor: f64 = *var_i5_cor_slot;
        let mut var_i5_cor_dn5: f64 = *var_i5_cor_dn5_slot;
        let mut var_i5_cor_dn6: f64 = *var_i5_cor_dn6_slot;
        let mut var_i5_cor_dn7: f64 = *var_i5_cor_dn7_slot;
        let mut var_i5_cor_dn8: f64 = *var_i5_cor_dn8_slot;
        let mut var_isatfor2_s: f64 = *var_isatfor2_s_slot;
        let mut var_isatfor2_s_dn5: f64 = *var_isatfor2_s_dn5_slot;
        let mut var_isatfor2_s_dn6: f64 = *var_isatfor2_s_dn6_slot;
        let mut var_isatfor2_s_dn7: f64 = *var_isatfor2_s_dn7_slot;
        let mut var_isatfor2_s_dn8: f64 = *var_isatfor2_s_dn8_slot;
        let mut var_isatrev_s: f64 = *var_isatrev_s_slot;
        let mut var_isatrev_s_dn5: f64 = *var_isatrev_s_dn5_slot;
        let mut var_isatrev_s_dn6: f64 = *var_isatrev_s_dn6_slot;
        let mut var_isatrev_s_dn7: f64 = *var_isatrev_s_dn7_slot;
        let mut var_isatrev_s_dn8: f64 = *var_isatrev_s_dn8_slot;
        let mut var_m0_rev: f64 = *var_m0_rev_slot;
        let mut var_m0_rev_dn5: f64 = *var_m0_rev_dn5_slot;
        let mut var_m0_rev_dn6: f64 = *var_m0_rev_dn6_slot;
        let mut var_m0_rev_dn7: f64 = *var_m0_rev_dn7_slot;
        let mut var_m0_rev_dn8: f64 = *var_m0_rev_dn8_slot;
        let mut var_m0flag_s: f64 = *var_m0flag_s_slot;
        let mut var_mcor_rev: f64 = *var_mcor_rev_slot;
        let mut var_mcor_rev_dn5: f64 = *var_mcor_rev_dn5_slot;
        let mut var_mcor_rev_dn6: f64 = *var_mcor_rev_dn6_slot;
        let mut var_mcor_rev_dn7: f64 = *var_mcor_rev_dn7_slot;
        let mut var_mcor_rev_dn8: f64 = *var_mcor_rev_dn8_slot;
        let mut var_mfor2_s: f64 = *var_mfor2_s_slot;
        let mut var_mfor2_s_dn5: f64 = *var_mfor2_s_dn5_slot;
        let mut var_mfor2_s_dn6: f64 = *var_mfor2_s_dn6_slot;
        let mut var_mfor2_s_dn7: f64 = *var_mfor2_s_dn7_slot;
        let mut var_mfor2_s_dn8: f64 = *var_mfor2_s_dn8_slot;
        let mut var_mrev_s: f64 = *var_mrev_s_slot;
        let mut var_mrev_s_dn5: f64 = *var_mrev_s_dn5_slot;
        let mut var_mrev_s_dn6: f64 = *var_mrev_s_dn6_slot;
        let mut var_mrev_s_dn7: f64 = *var_mrev_s_dn7_slot;
        let mut var_mrev_s_dn8: f64 = *var_mrev_s_dn8_slot;
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
        let mut var_xhighf1_s: f64 = *var_xhighf1_s_slot;
        let mut var_xhighf2_s: f64 = *var_xhighf2_s_slot;
        let mut var_xhighf2_s_dn5: f64 = *var_xhighf2_s_dn5_slot;
        let mut var_xhighf2_s_dn6: f64 = *var_xhighf2_s_dn6_slot;
        let mut var_xhighf2_s_dn7: f64 = *var_xhighf2_s_dn7_slot;
        let mut var_xhighf2_s_dn8: f64 = *var_xhighf2_s_dn8_slot;
        let mut var_xhighr_s: f64 = *var_xhighr_s_slot;
        let mut var_xhighr_s_dn5: f64 = *var_xhighr_s_dn5_slot;
        let mut var_xhighr_s_dn6: f64 = *var_xhighr_s_dn6_slot;
        let mut var_xhighr_s_dn7: f64 = *var_xhighr_s_dn7_slot;
        let mut var_xhighr_s_dn8: f64 = *var_xhighr_s_dn8_slot;
        let mut var_zflagbot_s: f64 = *var_zflagbot_s_slot;
        let mut var_zflaggat_s: f64 = *var_zflaggat_s_slot;
        let mut var_zflagsti_s: f64 = *var_zflagsti_s_slot;
        let mut var_zfrac: f64 = *var_zfrac_slot;

        let (assign27330_e32420, assign27330_e32420_d_n5, assign27330_e32420_d_n6, assign27330_e32420_d_n7, assign27330_e32420_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign27330_e32411: f64 = (var_v5 * var_phitdinv);
        let assign27330_e32413: f64 = (assign27330_e32411 * var_mfor1_s);
        let assign27330_e32414: f64 = (assign27330_e32413).exp();
        let assign27330_e32416: f64 = (assign27330_e32414 - 1.0);
        let assign27330_e32417: f64 = (var_isatfor1_s * assign27330_e32416);
        let assign27330_e32418: f64 = (var_i5 - assign27330_e32417);
        (assign27330_e32418, var_i5_dn5, var_i5_dn6, var_i5_dn7, var_i5_dn8,)
    } else {
        (var_i5_cor, var_i5_cor_dn5, var_i5_cor_dn6, var_i5_cor_dn7, var_i5_cor_dn8,)
    }
};
        var_i5_cor = assign27330_e32420;
        var_i5_cor_dn5 = assign27330_e32420_d_n5;
        var_i5_cor_dn6 = assign27330_e32420_d_n6;
        var_i5_cor_dn7 = assign27330_e32420_d_n7;
        var_i5_cor_dn8 = assign27330_e32420_d_n8;

        let assign27340_e32432: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard522 = assign27340_e32432;

        let assign27350_e32439: f64 = if ((var_i4 > 0.0) && (var_i5 > 0.0)) { 1.0 } else { 0.0 };
        var_guard523 = assign27350_e32439;

        let assign27360_e32442: f64 = (var_i4_cor / var_i4);
        let assign27360_e32447: f64 = (var_i5_cor / var_i5);
        let assign27360_e32462: f64 = if (((((assign27360_e32442 > 0.001) || (assign27360_e32447 > 0.001)) && (var_i4_cor > 0.0)) && (var_i5_cor > 0.0)) && (var_i5_cor > var_i4_cor)) { 1.0 } else { 0.0 };
        var_guard524 = assign27360_e32462;

        let (assign27370_e32476, assign27370_e32476_d_n5, assign27370_e32476_d_n6, assign27370_e32476_d_n7, assign27370_e32476_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) && (var_guard523 != 0.0)) && (var_guard524 != 0.0)) {
        let assign27370_e32474: f64 = (var_i4_cor / var_i5_cor);
        (assign27370_e32474, (((var_i4_cor_dn5 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn5)) / (var_i5_cor * var_i5_cor)), (((var_i4_cor_dn6 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn6)) / (var_i5_cor * var_i5_cor)), (((var_i4_cor_dn7 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn7)) / (var_i5_cor * var_i5_cor)), (((var_i4_cor_dn8 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn8)) / (var_i5_cor * var_i5_cor)),)
    } else {
        (var_alphaje, var_alphaje_dn5, var_alphaje_dn6, var_alphaje_dn7, var_alphaje_dn8,)
    }
};
        var_alphaje = assign27370_e32476;
        var_alphaje_dn5 = assign27370_e32476_d_n5;
        var_alphaje_dn6 = assign27370_e32476_d_n6;
        var_alphaje_dn7 = assign27370_e32476_d_n7;
        var_alphaje_dn8 = assign27370_e32476_d_n8;

        let (assign27380_e32495, assign27380_e32495_d_n5, assign27380_e32495_d_n6, assign27380_e32495_d_n7, assign27380_e32495_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) && (var_guard523 != 0.0)) && (var_guard524 != 0.0)) {
        let assign27380_e32488: f64 = (var_alphaje).ln();
        let assign27380_e32489: f64 = (var_phitd * assign27380_e32488);
        let assign27380_e32492: f64 = (var_v4 - var_v5);
        let assign27380_e32493: f64 = (assign27380_e32489 / assign27380_e32492);
        (assign27380_e32493, ((var_phitd * (var_alphaje_dn5 / var_alphaje)) / assign27380_e32492), ((var_phitd * (var_alphaje_dn6 / var_alphaje)) / assign27380_e32492), ((var_phitd * (var_alphaje_dn7 / var_alphaje)) / assign27380_e32492), ((var_phitd * (var_alphaje_dn8 / var_alphaje)) / assign27380_e32492),)
    } else {
        (var_mfor2_s, var_mfor2_s_dn5, var_mfor2_s_dn6, var_mfor2_s_dn7, var_mfor2_s_dn8,)
    }
};
        var_mfor2_s = assign27380_e32495;
        var_mfor2_s_dn5 = assign27380_e32495_d_n5;
        var_mfor2_s_dn6 = assign27380_e32495_d_n6;
        var_mfor2_s_dn7 = assign27380_e32495_d_n7;
        var_mfor2_s_dn8 = assign27380_e32495_d_n8;

        let (assign27390_e32516, assign27390_e32516_d_n5, assign27390_e32516_d_n6, assign27390_e32516_d_n7, assign27390_e32516_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) && (var_guard523 != 0.0)) && (var_guard524 != 0.0)) {
        let assign27390_e32508: f64 = (var_v4 * var_phitdinv);
        let assign27390_e32510: f64 = (assign27390_e32508 * var_mfor2_s);
        let assign27390_e32511: f64 = (assign27390_e32510).exp();
        let assign27390_e32513: f64 = (assign27390_e32511 - 1.0);
        let assign27390_e32514: f64 = (var_i4_cor / assign27390_e32513);
        (assign27390_e32514, (((var_i4_cor_dn5 * assign27390_e32513) - (var_i4_cor * (assign27390_e32511 * (assign27390_e32508 * var_mfor2_s_dn5)))) / (assign27390_e32513 * assign27390_e32513)), (((var_i4_cor_dn6 * assign27390_e32513) - (var_i4_cor * (assign27390_e32511 * (assign27390_e32508 * var_mfor2_s_dn6)))) / (assign27390_e32513 * assign27390_e32513)), (((var_i4_cor_dn7 * assign27390_e32513) - (var_i4_cor * (assign27390_e32511 * (assign27390_e32508 * var_mfor2_s_dn7)))) / (assign27390_e32513 * assign27390_e32513)), (((var_i4_cor_dn8 * assign27390_e32513) - (var_i4_cor * (assign27390_e32511 * (assign27390_e32508 * var_mfor2_s_dn8)))) / (assign27390_e32513 * assign27390_e32513)),)
    } else {
        (var_isatfor2_s, var_isatfor2_s_dn5, var_isatfor2_s_dn6, var_isatfor2_s_dn7, var_isatfor2_s_dn8,)
    }
};
        var_isatfor2_s = assign27390_e32516;
        var_isatfor2_s_dn5 = assign27390_e32516_d_n5;
        var_isatfor2_s_dn6 = assign27390_e32516_d_n6;
        var_isatfor2_s_dn7 = assign27390_e32516_d_n7;
        var_isatfor2_s_dn8 = assign27390_e32516_d_n8;

        let (assign27400_e32546, assign27400_e32546_d_n5, assign27400_e32546_d_n6, assign27400_e32546_d_n7, assign27400_e32546_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) {
        let assign27400_e32526: f64 = (var_v1 * var_phitdinv);
        let assign27400_e32528: f64 = (assign27400_e32526 * var_mfor1_s);
        let assign27400_e32529: f64 = (assign27400_e32528).exp();
        let assign27400_e32531: f64 = (assign27400_e32529 - 1.0);
        let assign27400_e32532: f64 = (var_isatfor1_s * assign27400_e32531);
        let assign27400_e32533: f64 = (var_i1 - assign27400_e32532);
        let assign27400_e32537: f64 = (var_v1 * var_phitdinv);
        let assign27400_e32539: f64 = (assign27400_e32537 * var_mfor2_s);
        let assign27400_e32540: f64 = (assign27400_e32539).exp();
        let assign27400_e32542: f64 = (assign27400_e32540 - 1.0);
        let assign27400_e32543: f64 = (var_isatfor2_s * assign27400_e32542);
        let assign27400_e32544: f64 = (assign27400_e32533 - assign27400_e32543);
        (assign27400_e32544, (var_i1_dn5 - ((var_isatfor2_s_dn5 * assign27400_e32542) + (var_isatfor2_s * (assign27400_e32540 * (assign27400_e32537 * var_mfor2_s_dn5))))), (var_i1_dn6 - ((var_isatfor2_s_dn6 * assign27400_e32542) + (var_isatfor2_s * (assign27400_e32540 * (assign27400_e32537 * var_mfor2_s_dn6))))), (var_i1_dn7 - ((var_isatfor2_s_dn7 * assign27400_e32542) + (var_isatfor2_s * (assign27400_e32540 * (assign27400_e32537 * var_mfor2_s_dn7))))), (var_i1_dn8 - ((var_isatfor2_s_dn8 * assign27400_e32542) + (var_isatfor2_s * (assign27400_e32540 * (assign27400_e32537 * var_mfor2_s_dn8))))),)
    } else {
        (var_i1_cor, var_i1_cor_dn5, var_i1_cor_dn6, var_i1_cor_dn7, var_i1_cor_dn8,)
    }
};
        var_i1_cor = assign27400_e32546;
        var_i1_cor_dn5 = assign27400_e32546_d_n5;
        var_i1_cor_dn6 = assign27400_e32546_d_n6;
        var_i1_cor_dn7 = assign27400_e32546_d_n7;
        var_i1_cor_dn8 = assign27400_e32546_d_n8;

        let (assign27410_e32576, assign27410_e32576_d_n5, assign27410_e32576_d_n6, assign27410_e32576_d_n7, assign27410_e32576_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) {
        let assign27410_e32556: f64 = (var_v2 * var_phitdinv);
        let assign27410_e32558: f64 = (assign27410_e32556 * var_mfor1_s);
        let assign27410_e32559: f64 = (assign27410_e32558).exp();
        let assign27410_e32561: f64 = (assign27410_e32559 - 1.0);
        let assign27410_e32562: f64 = (var_isatfor1_s * assign27410_e32561);
        let assign27410_e32563: f64 = (var_i2 - assign27410_e32562);
        let assign27410_e32567: f64 = (var_v2 * var_phitdinv);
        let assign27410_e32569: f64 = (assign27410_e32567 * var_mfor2_s);
        let assign27410_e32570: f64 = (assign27410_e32569).exp();
        let assign27410_e32572: f64 = (assign27410_e32570 - 1.0);
        let assign27410_e32573: f64 = (var_isatfor2_s * assign27410_e32572);
        let assign27410_e32574: f64 = (assign27410_e32563 - assign27410_e32573);
        (assign27410_e32574, (var_i2_dn5 - ((var_isatfor2_s_dn5 * assign27410_e32572) + (var_isatfor2_s * (assign27410_e32570 * (assign27410_e32567 * var_mfor2_s_dn5))))), (var_i2_dn6 - ((var_isatfor2_s_dn6 * assign27410_e32572) + (var_isatfor2_s * (assign27410_e32570 * (assign27410_e32567 * var_mfor2_s_dn6))))), (var_i2_dn7 - ((var_isatfor2_s_dn7 * assign27410_e32572) + (var_isatfor2_s * (assign27410_e32570 * (assign27410_e32567 * var_mfor2_s_dn7))))), (var_i2_dn8 - ((var_isatfor2_s_dn8 * assign27410_e32572) + (var_isatfor2_s * (assign27410_e32570 * (assign27410_e32567 * var_mfor2_s_dn8))))),)
    } else {
        (var_i2_cor, var_i2_cor_dn5, var_i2_cor_dn6, var_i2_cor_dn7, var_i2_cor_dn8,)
    }
};
        var_i2_cor = assign27410_e32576;
        var_i2_cor_dn5 = assign27410_e32576_d_n5;
        var_i2_cor_dn6 = assign27410_e32576_d_n6;
        var_i2_cor_dn7 = assign27410_e32576_d_n7;
        var_i2_cor_dn8 = assign27410_e32576_d_n8;

        let (assign27420_e32606, assign27420_e32606_d_n5, assign27420_e32606_d_n6, assign27420_e32606_d_n7, assign27420_e32606_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) {
        let assign27420_e32586: f64 = (var_v3 * var_phitdinv);
        let assign27420_e32588: f64 = (assign27420_e32586 * var_mfor1_s);
        let assign27420_e32589: f64 = (assign27420_e32588).exp();
        let assign27420_e32591: f64 = (assign27420_e32589 - 1.0);
        let assign27420_e32592: f64 = (var_isatfor1_s * assign27420_e32591);
        let assign27420_e32593: f64 = (var_i3 - assign27420_e32592);
        let assign27420_e32597: f64 = (var_v3 * var_phitdinv);
        let assign27420_e32599: f64 = (assign27420_e32597 * var_mfor2_s);
        let assign27420_e32600: f64 = (assign27420_e32599).exp();
        let assign27420_e32602: f64 = (assign27420_e32600 - 1.0);
        let assign27420_e32603: f64 = (var_isatfor2_s * assign27420_e32602);
        let assign27420_e32604: f64 = (assign27420_e32593 - assign27420_e32603);
        (assign27420_e32604, (var_i3_dn5 - ((var_isatfor2_s_dn5 * assign27420_e32602) + (var_isatfor2_s * (assign27420_e32600 * (assign27420_e32597 * var_mfor2_s_dn5))))), (var_i3_dn6 - ((var_isatfor2_s_dn6 * assign27420_e32602) + (var_isatfor2_s * (assign27420_e32600 * (assign27420_e32597 * var_mfor2_s_dn6))))), (var_i3_dn7 - ((var_isatfor2_s_dn7 * assign27420_e32602) + (var_isatfor2_s * (assign27420_e32600 * (assign27420_e32597 * var_mfor2_s_dn7))))), (var_i3_dn8 - ((var_isatfor2_s_dn8 * assign27420_e32602) + (var_isatfor2_s * (assign27420_e32600 * (assign27420_e32597 * var_mfor2_s_dn8))))),)
    } else {
        (var_i3_cor, var_i3_cor_dn5, var_i3_cor_dn6, var_i3_cor_dn7, var_i3_cor_dn8,)
    }
};
        var_i3_cor = assign27420_e32606;
        var_i3_cor_dn5 = assign27420_e32606_d_n5;
        var_i3_cor_dn6 = assign27420_e32606_d_n6;
        var_i3_cor_dn7 = assign27420_e32606_d_n7;
        var_i3_cor_dn8 = assign27420_e32606_d_n8;

        let assign27430_e32617: f64 = if (((var_i1 < 0.0) && (var_i2 < 0.0)) && (var_i3 < 0.0)) { 1.0 } else { 0.0 };
        var_guard525 = assign27430_e32617;

        let assign27440_e32620: f64 = (var_i1_cor / var_i1);
        let assign27440_e32625: f64 = (var_i2_cor / var_i2);
        let assign27440_e32631: f64 = (var_i3_cor / var_i3);
        let assign27440_e32646: f64 = if ((((((assign27440_e32620 > 0.001) || (assign27440_e32625 > 0.001)) || (assign27440_e32631 > 0.001)) && (var_i1_cor < 0.0)) && (var_i2_cor < 0.0)) && (var_i3_cor < 0.0)) { 1.0 } else { 0.0 };
        var_guard526 = assign27440_e32646;

        let (assign27450_e32660, assign27450_e32660_d_n5, assign27450_e32660_d_n6, assign27450_e32660_d_n7, assign27450_e32660_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 != 0.0)) {
        let assign27450_e32658: f64 = (var_i1_cor / var_i2_cor);
        (assign27450_e32658, (((var_i1_cor_dn5 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn5)) / (var_i2_cor * var_i2_cor)), (((var_i1_cor_dn6 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn6)) / (var_i2_cor * var_i2_cor)), (((var_i1_cor_dn7 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn7)) / (var_i2_cor * var_i2_cor)), (((var_i1_cor_dn8 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn8)) / (var_i2_cor * var_i2_cor)),)
    } else {
        (var_alphaje, var_alphaje_dn5, var_alphaje_dn6, var_alphaje_dn7, var_alphaje_dn8,)
    }
};
        var_alphaje = assign27450_e32660;
        var_alphaje_dn5 = assign27450_e32660_d_n5;
        var_alphaje_dn6 = assign27450_e32660_d_n6;
        var_alphaje_dn7 = assign27450_e32660_d_n7;
        var_alphaje_dn8 = assign27450_e32660_d_n8;

        let (assign27460_e32680, assign27460_e32680_d_n5, assign27460_e32680_d_n6, assign27460_e32680_d_n7, assign27460_e32680_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 != 0.0)) {
        let assign27460_e32671: f64 = (-var_phitd);
        let assign27460_e32673: f64 = (var_alphaje).ln();
        let assign27460_e32674: f64 = (assign27460_e32671 * assign27460_e32673);
        let assign27460_e32677: f64 = (var_v1 - var_v2);
        let assign27460_e32678: f64 = (assign27460_e32674 / assign27460_e32677);
        (assign27460_e32678, ((assign27460_e32671 * (var_alphaje_dn5 / var_alphaje)) / assign27460_e32677), ((assign27460_e32671 * (var_alphaje_dn6 / var_alphaje)) / assign27460_e32677), ((assign27460_e32671 * (var_alphaje_dn7 / var_alphaje)) / assign27460_e32677), ((assign27460_e32671 * (var_alphaje_dn8 / var_alphaje)) / assign27460_e32677),)
    } else {
        (var_m0_rev, var_m0_rev_dn5, var_m0_rev_dn6, var_m0_rev_dn7, var_m0_rev_dn8,)
    }
};
        var_m0_rev = assign27460_e32680;
        var_m0_rev_dn5 = assign27460_e32680_d_n5;
        var_m0_rev_dn6 = assign27460_e32680_d_n6;
        var_m0_rev_dn7 = assign27460_e32680_d_n7;
        var_m0_rev_dn8 = assign27460_e32680_d_n8;

        let (assign27470_e32696,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 != 0.0)) {
        let assign27470_e32693: f64 = (var_v2 - var_v1);
        let assign27470_e32694: f64 = (var_v2 / assign27470_e32693);
        (assign27470_e32694,)
    } else {
        (var_tt0,)
    }
};
        var_tt0 = assign27470_e32696;

        let (assign27480_e32718, assign27480_e32718_d_n5, assign27480_e32718_d_n6, assign27480_e32718_d_n7, assign27480_e32718_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 != 0.0)) {
        let assign27480_e32709: f64 = (var_alphaje - 1.0);
        let assign27480_e32710: f64 = (var_phitd * assign27480_e32709);
        let assign27480_e32713: f64 = (var_alphaje).powf(var_tt0);
        let assign27480_e32715: f64 = (assign27480_e32713 - 1.0);
        let assign27480_e32716: f64 = (assign27480_e32710 * assign27480_e32715);
        (assign27480_e32716, (((var_phitd * var_alphaje_dn5) * assign27480_e32715) + (assign27480_e32710 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn5)) } } else { (assign27480_e32713 * (var_tt0 * (var_alphaje_dn5 / var_alphaje))) })), (((var_phitd * var_alphaje_dn6) * assign27480_e32715) + (assign27480_e32710 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn6)) } } else { (assign27480_e32713 * (var_tt0 * (var_alphaje_dn6 / var_alphaje))) })), (((var_phitd * var_alphaje_dn7) * assign27480_e32715) + (assign27480_e32710 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn7)) } } else { (assign27480_e32713 * (var_tt0 * (var_alphaje_dn7 / var_alphaje))) })), (((var_phitd * var_alphaje_dn8) * assign27480_e32715) + (assign27480_e32710 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn8)) } } else { (assign27480_e32713 * (var_tt0 * (var_alphaje_dn8 / var_alphaje))) })),)
    } else {
        (var_tt1, var_tt1_dn5, var_tt1_dn6, var_tt1_dn7, var_tt1_dn8,)
    }
};
        var_tt1 = assign27480_e32718;
        var_tt1_dn5 = assign27480_e32718_d_n5;
        var_tt1_dn6 = assign27480_e32718_d_n6;
        var_tt1_dn7 = assign27480_e32718_d_n7;
        var_tt1_dn8 = assign27480_e32718_d_n8;

        let (assign27490_e32734,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 != 0.0)) {
        let assign27490_e32731: f64 = (var_v1 - var_v2);
        let assign27490_e32732: f64 = (var_v1 / assign27490_e32731);
        (assign27490_e32732,)
    } else {
        (var_tt0,)
    }
};
        var_tt0 = assign27490_e32734;

        let (assign27500_e32758, assign27500_e32758_d_n5, assign27500_e32758_d_n6, assign27500_e32758_d_n7, assign27500_e32758_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 != 0.0)) {
        let assign27500_e32746: f64 = (var_alphaje).powf(var_tt0);
        let assign27500_e32749: f64 = (var_v2 - var_v1);
        let assign27500_e32750: f64 = (assign27500_e32746 * assign27500_e32749);
        let assign27500_e32753: f64 = (var_alphaje * var_v1);
        let assign27500_e32754: f64 = (assign27500_e32750 + assign27500_e32753);
        let assign27500_e32756: f64 = (assign27500_e32754 - var_v2);
        (assign27500_e32756, ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn5)) } } else { (assign27500_e32746 * (var_tt0 * (var_alphaje_dn5 / var_alphaje))) } * assign27500_e32749) + (var_alphaje_dn5 * var_v1)), ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn6)) } } else { (assign27500_e32746 * (var_tt0 * (var_alphaje_dn6 / var_alphaje))) } * assign27500_e32749) + (var_alphaje_dn6 * var_v1)), ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn7)) } } else { (assign27500_e32746 * (var_tt0 * (var_alphaje_dn7 / var_alphaje))) } * assign27500_e32749) + (var_alphaje_dn7 * var_v1)), ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn8)) } } else { (assign27500_e32746 * (var_tt0 * (var_alphaje_dn8 / var_alphaje))) } * assign27500_e32749) + (var_alphaje_dn8 * var_v1)),)
    } else {
        (var_tt2, var_tt2_dn5, var_tt2_dn6, var_tt2_dn7, var_tt2_dn8,)
    }
};
        var_tt2 = assign27500_e32758;
        var_tt2_dn5 = assign27500_e32758_d_n5;
        var_tt2_dn6 = assign27500_e32758_d_n6;
        var_tt2_dn7 = assign27500_e32758_d_n7;
        var_tt2_dn8 = assign27500_e32758_d_n8;

        let (assign27510_e32772, assign27510_e32772_d_n5, assign27510_e32772_d_n6, assign27510_e32772_d_n7, assign27510_e32772_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 != 0.0)) {
        let assign27510_e32770: f64 = (var_tt1 / var_tt2);
        (assign27510_e32770, (((var_tt1_dn5 * var_tt2) - (var_tt1 * var_tt2_dn5)) / (var_tt2 * var_tt2)), (((var_tt1_dn6 * var_tt2) - (var_tt1 * var_tt2_dn6)) / (var_tt2 * var_tt2)), (((var_tt1_dn7 * var_tt2) - (var_tt1 * var_tt2_dn7)) / (var_tt2 * var_tt2)), (((var_tt1_dn8 * var_tt2) - (var_tt1 * var_tt2_dn8)) / (var_tt2 * var_tt2)),)
    } else {
        (var_mcor_rev, var_mcor_rev_dn5, var_mcor_rev_dn6, var_mcor_rev_dn7, var_mcor_rev_dn8,)
    }
};
        var_mcor_rev = assign27510_e32772;
        var_mcor_rev_dn5 = assign27510_e32772_d_n5;
        var_mcor_rev_dn6 = assign27510_e32772_d_n6;
        var_mcor_rev_dn7 = assign27510_e32772_d_n7;
        var_mcor_rev_dn8 = assign27510_e32772_d_n8;

        let (assign27520_e32786, assign27520_e32786_d_n5, assign27520_e32786_d_n6, assign27520_e32786_d_n7, assign27520_e32786_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 != 0.0)) {
        let assign27520_e32784: f64 = (var_m0_rev + var_mcor_rev);
        (assign27520_e32784, (var_m0_rev_dn5 + var_mcor_rev_dn5), (var_m0_rev_dn6 + var_mcor_rev_dn6), (var_m0_rev_dn7 + var_mcor_rev_dn7), (var_m0_rev_dn8 + var_mcor_rev_dn8),)
    } else {
        (var_mrev_s, var_mrev_s_dn5, var_mrev_s_dn6, var_mrev_s_dn7, var_mrev_s_dn8,)
    }
};
        var_mrev_s = assign27520_e32786;
        var_mrev_s_dn5 = assign27520_e32786_d_n5;
        var_mrev_s_dn6 = assign27520_e32786_d_n6;
        var_mrev_s_dn7 = assign27520_e32786_d_n7;
        var_mrev_s_dn8 = assign27520_e32786_d_n8;

        let assign27530_e32789: f64 = (var_v3 * var_phitdinv);
        let assign27530_e32791: f64 = (assign27530_e32789 * var_mrev_s);
        let assign27530_e32792: f64 = (assign27530_e32791).abs();
        let assign27530_e32794: f64 = if assign27530_e32792 < 1e-6 { 1.0 } else { 0.0 };
        var_guard527 = assign27530_e32794;

        let (assign27540_e32808,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 != 0.0)) && (var_guard527 != 0.0)) {
        (1.0,)
    } else {
        (var_m0flag_s,)
    }
};
        var_m0flag_s = assign27540_e32808;

        let (assign27550_e32832, assign27550_e32832_d_n5, assign27550_e32832_d_n6, assign27550_e32832_d_n7, assign27550_e32832_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 != 0.0)) && (var_guard527 != 0.0)) {
        let assign27550_e32823: f64 = (1.0 / var_v3);
        let assign27550_e32826: f64 = (0.5 * var_phitdinv);
        let assign27550_e32828: f64 = (assign27550_e32826 * var_mrev_s);
        let assign27550_e32829: f64 = (assign27550_e32823 + assign27550_e32828);
        let assign27550_e32830: f64 = (var_i3_cor * assign27550_e32829);
        (assign27550_e32830, ((var_i3_cor_dn5 * assign27550_e32829) + (var_i3_cor * (assign27550_e32826 * var_mrev_s_dn5))), ((var_i3_cor_dn6 * assign27550_e32829) + (var_i3_cor * (assign27550_e32826 * var_mrev_s_dn6))), ((var_i3_cor_dn7 * assign27550_e32829) + (var_i3_cor * (assign27550_e32826 * var_mrev_s_dn7))), ((var_i3_cor_dn8 * assign27550_e32829) + (var_i3_cor * (assign27550_e32826 * var_mrev_s_dn8))),)
    } else {
        (var_isatrev_s, var_isatrev_s_dn5, var_isatrev_s_dn6, var_isatrev_s_dn7, var_isatrev_s_dn8,)
    }
};
        var_isatrev_s = assign27550_e32832;
        var_isatrev_s_dn5 = assign27550_e32832_d_n5;
        var_isatrev_s_dn6 = assign27550_e32832_d_n6;
        var_isatrev_s_dn7 = assign27550_e32832_d_n7;
        var_isatrev_s_dn8 = assign27550_e32832_d_n8;

        let (assign27560_e32855, assign27560_e32855_d_n5, assign27560_e32855_d_n6, assign27560_e32855_d_n7, assign27560_e32855_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 != 0.0)) && (var_guard527 != 0.0)) {
        let assign27560_e32845: f64 = (-0.5);
        let assign27560_e32847: f64 = (assign27560_e32845 * var_i3_cor);
        let assign27560_e32849: f64 = (assign27560_e32847 * var_mrev_s);
        let assign27560_e32851: f64 = (assign27560_e32849 * var_phitdinv);
        let assign27560_e32853: f64 = (assign27560_e32851 / var_v3);
        (assign27560_e32853, (((((assign27560_e32845 * var_i3_cor_dn5) * var_mrev_s) + (assign27560_e32847 * var_mrev_s_dn5)) * var_phitdinv) / var_v3), (((((assign27560_e32845 * var_i3_cor_dn6) * var_mrev_s) + (assign27560_e32847 * var_mrev_s_dn6)) * var_phitdinv) / var_v3), (((((assign27560_e32845 * var_i3_cor_dn7) * var_mrev_s) + (assign27560_e32847 * var_mrev_s_dn7)) * var_phitdinv) / var_v3), (((((assign27560_e32845 * var_i3_cor_dn8) * var_mrev_s) + (assign27560_e32847 * var_mrev_s_dn8)) * var_phitdinv) / var_v3),)
    } else {
        (var_mrev_s, var_mrev_s_dn5, var_mrev_s_dn6, var_mrev_s_dn7, var_mrev_s_dn8,)
    }
};
        var_mrev_s = assign27560_e32855;
        var_mrev_s_dn5 = assign27560_e32855_d_n5;
        var_mrev_s_dn6 = assign27560_e32855_d_n6;
        var_mrev_s_dn7 = assign27560_e32855_d_n7;
        var_mrev_s_dn8 = assign27560_e32855_d_n8;

        let (assign27570_e32870,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 != 0.0)) && (var_guard527 == 0.0)) {
        (0.0,)
    } else {
        (var_m0flag_s,)
    }
};
        var_m0flag_s = assign27570_e32870;

        let (assign27580_e32896, assign27580_e32896_d_n5, assign27580_e32896_d_n6, assign27580_e32896_d_n7, assign27580_e32896_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard522 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 != 0.0)) && (var_guard527 == 0.0)) {
        let assign27580_e32884: f64 = (-var_i3_cor);
        let assign27580_e32886: f64 = (-var_v3);
        let assign27580_e32888: f64 = (assign27580_e32886 * var_phitdinv);
        let assign27580_e32890: f64 = (assign27580_e32888 * var_mrev_s);
        let assign27580_e32891: f64 = (assign27580_e32890).exp();
        let assign27580_e32893: f64 = (assign27580_e32891 - 1.0);
        let assign27580_e32894: f64 = (assign27580_e32884 / assign27580_e32893);
        (assign27580_e32894, ((((-var_i3_cor_dn5) * assign27580_e32893) - (assign27580_e32884 * (assign27580_e32891 * (assign27580_e32888 * var_mrev_s_dn5)))) / (assign27580_e32893 * assign27580_e32893)), ((((-var_i3_cor_dn6) * assign27580_e32893) - (assign27580_e32884 * (assign27580_e32891 * (assign27580_e32888 * var_mrev_s_dn6)))) / (assign27580_e32893 * assign27580_e32893)), ((((-var_i3_cor_dn7) * assign27580_e32893) - (assign27580_e32884 * (assign27580_e32891 * (assign27580_e32888 * var_mrev_s_dn7)))) / (assign27580_e32893 * assign27580_e32893)), ((((-var_i3_cor_dn8) * assign27580_e32893) - (assign27580_e32884 * (assign27580_e32891 * (assign27580_e32888 * var_mrev_s_dn8)))) / (assign27580_e32893 * assign27580_e32893)),)
    } else {
        (var_isatrev_s, var_isatrev_s_dn5, var_isatrev_s_dn6, var_isatrev_s_dn7, var_isatrev_s_dn8,)
    }
};
        var_isatrev_s = assign27580_e32896;
        var_isatrev_s_dn5 = assign27580_e32896_d_n5;
        var_isatrev_s_dn6 = assign27580_e32896_d_n6;
        var_isatrev_s_dn7 = assign27580_e32896_d_n7;
        var_isatrev_s_dn8 = assign27580_e32896_d_n8;

        let (assign27590_e32914,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign27590_e32903: f64 = (var_absource_i * var_cjobot);
        let assign27590_e32906: f64 = (var_lssource_i * var_cjosti);
        let assign27590_e32907: f64 = (assign27590_e32903 + assign27590_e32906);
        let assign27590_e32910: f64 = (var_lgsource_i * var_cjogat);
        let assign27590_e32911: f64 = (assign27590_e32907 + assign27590_e32910);
        let assign27590_e32912: f64 = (p.p922 * assign27590_e32911);
        (assign27590_e32912,)
    } else {
        (var_zfrac,)
    }
};
        var_zfrac = assign27590_e32914;

        let assign27600_e32917: f64 = (var_absource_i * var_cjobot);
        let assign27600_e32919: f64 = if assign27600_e32917 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard528 = assign27600_e32919;

        let (assign27610_e32927,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard528 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagbot_s,)
    }
};
        var_zflagbot_s = assign27610_e32927;

        let assign27620_e32930: f64 = (var_lssource_i * var_cjosti);
        let assign27620_e32932: f64 = if assign27620_e32930 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard529 = assign27620_e32932;

        let (assign27630_e32940,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard529 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagsti_s,)
    }
};
        var_zflagsti_s = assign27630_e32940;

        let assign27640_e32943: f64 = (var_lgsource_i * var_cjogat);
        let assign27640_e32945: f64 = if assign27640_e32943 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard530 = assign27640_e32945;

        let (assign27650_e32953,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard530 != 0.0)) {
        (0.0,)
    } else {
        (var_zflaggat_s,)
    }
};
        var_zflaggat_s = assign27650_e32953;

        let assign27660_e32965: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard531 = assign27660_e32965;

        let (assign27670_e32980,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard531 != 0.0)) {
        let assign27670_e32973: f64 = (0.5 * p.p815);
        let assign27670_e32976: f64 = (var_isatfor1_s + 1e-21);
        let assign27670_e32977: f64 = (assign27670_e32973 / assign27670_e32976);
        let assign27670_e32978: f64 = (assign27670_e32977).ln();
        (assign27670_e32978,)
    } else {
        (var_xhighf1_s,)
    }
};
        var_xhighf1_s = assign27670_e32980;

        let (assign27680_e32995, assign27680_e32995_d_n5, assign27680_e32995_d_n6, assign27680_e32995_d_n7, assign27680_e32995_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard531 != 0.0)) {
        let assign27680_e32988: f64 = (0.5 * p.p815);
        let assign27680_e32991: f64 = (var_isatfor2_s + 1e-21);
        let assign27680_e32992: f64 = (assign27680_e32988 / assign27680_e32991);
        let assign27680_e32993: f64 = (assign27680_e32992).ln();
        (assign27680_e32993, ((-((assign27680_e32988 * var_isatfor2_s_dn5) / (assign27680_e32991 * assign27680_e32991))) / assign27680_e32992), ((-((assign27680_e32988 * var_isatfor2_s_dn6) / (assign27680_e32991 * assign27680_e32991))) / assign27680_e32992), ((-((assign27680_e32988 * var_isatfor2_s_dn7) / (assign27680_e32991 * assign27680_e32991))) / assign27680_e32992), ((-((assign27680_e32988 * var_isatfor2_s_dn8) / (assign27680_e32991 * assign27680_e32991))) / assign27680_e32992),)
    } else {
        (var_xhighf2_s, var_xhighf2_s_dn5, var_xhighf2_s_dn6, var_xhighf2_s_dn7, var_xhighf2_s_dn8,)
    }
};
        var_xhighf2_s = assign27680_e32995;
        var_xhighf2_s_dn5 = assign27680_e32995_d_n5;
        var_xhighf2_s_dn6 = assign27680_e32995_d_n6;
        var_xhighf2_s_dn7 = assign27680_e32995_d_n7;
        var_xhighf2_s_dn8 = assign27680_e32995_d_n8;

        let (assign27690_e33011, assign27690_e33011_d_n5, assign27690_e33011_d_n6, assign27690_e33011_d_n7, assign27690_e33011_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard531 != 0.0)) {
        let assign27690_e33003: f64 = (0.5 * p.p815);
        let assign27690_e33005: f64 = (var_isatrev_s).abs();
        let assign27690_e33007: f64 = (assign27690_e33005 + 1e-21);
        let assign27690_e33008: f64 = (assign27690_e33003 / assign27690_e33007);
        let assign27690_e33009: f64 = (assign27690_e33008).ln();
        (assign27690_e33009, ((-((assign27690_e33003 * if var_isatrev_s >= 0.0 { var_isatrev_s_dn5 } else { (-var_isatrev_s_dn5) }) / (assign27690_e33007 * assign27690_e33007))) / assign27690_e33008), ((-((assign27690_e33003 * if var_isatrev_s >= 0.0 { var_isatrev_s_dn6 } else { (-var_isatrev_s_dn6) }) / (assign27690_e33007 * assign27690_e33007))) / assign27690_e33008), ((-((assign27690_e33003 * if var_isatrev_s >= 0.0 { var_isatrev_s_dn7 } else { (-var_isatrev_s_dn7) }) / (assign27690_e33007 * assign27690_e33007))) / assign27690_e33008), ((-((assign27690_e33003 * if var_isatrev_s >= 0.0 { var_isatrev_s_dn8 } else { (-var_isatrev_s_dn8) }) / (assign27690_e33007 * assign27690_e33007))) / assign27690_e33008),)
    } else {
        (var_xhighr_s, var_xhighr_s_dn5, var_xhighr_s_dn6, var_xhighr_s_dn7, var_xhighr_s_dn8,)
    }
};
        var_xhighr_s = assign27690_e33011;
        var_xhighr_s_dn5 = assign27690_e33011_d_n5;
        var_xhighr_s_dn6 = assign27690_e33011_d_n6;
        var_xhighr_s_dn7 = assign27690_e33011_d_n7;
        var_xhighr_s_dn8 = assign27690_e33011_d_n8;

        let (assign27700_e33019,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign27700_e33017: f64 = (var_xhighf1_s).min(230.25850929940458);
        (assign27700_e33017,)
    } else {
        (var_xhighf1_s,)
    }
};
        var_xhighf1_s = assign27700_e33019;

        let (assign27710_e33026,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign27710_e33024: f64 = (var_xhighf1_s).exp();
        (assign27710_e33024,)
    } else {
        (var_expxhf1_s,)
    }
};
        var_expxhf1_s = assign27710_e33026;

        let (assign27720_e33034, assign27720_e33034_d_n5, assign27720_e33034_d_n6, assign27720_e33034_d_n7, assign27720_e33034_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign27720_e33032: f64 = (var_xhighf2_s).min(230.25850929940458);
        (assign27720_e33032, if var_xhighf2_s <= 230.25850929940458 { var_xhighf2_s_dn5 } else { 0.0 }, if var_xhighf2_s <= 230.25850929940458 { var_xhighf2_s_dn6 } else { 0.0 }, if var_xhighf2_s <= 230.25850929940458 { var_xhighf2_s_dn7 } else { 0.0 }, if var_xhighf2_s <= 230.25850929940458 { var_xhighf2_s_dn8 } else { 0.0 },)
    } else {
        (var_xhighf2_s, var_xhighf2_s_dn5, var_xhighf2_s_dn6, var_xhighf2_s_dn7, var_xhighf2_s_dn8,)
    }
};
        var_xhighf2_s = assign27720_e33034;
        var_xhighf2_s_dn5 = assign27720_e33034_d_n5;
        var_xhighf2_s_dn6 = assign27720_e33034_d_n6;
        var_xhighf2_s_dn7 = assign27720_e33034_d_n7;
        var_xhighf2_s_dn8 = assign27720_e33034_d_n8;

        *var_alphaje_slot = var_alphaje;
        *var_alphaje_dn5_slot = var_alphaje_dn5;
        *var_alphaje_dn6_slot = var_alphaje_dn6;
        *var_alphaje_dn7_slot = var_alphaje_dn7;
        *var_alphaje_dn8_slot = var_alphaje_dn8;
        *var_expxhf1_s_slot = var_expxhf1_s;
        *var_guard522_slot = var_guard522;
        *var_guard523_slot = var_guard523;
        *var_guard524_slot = var_guard524;
        *var_guard525_slot = var_guard525;
        *var_guard526_slot = var_guard526;
        *var_guard527_slot = var_guard527;
        *var_guard528_slot = var_guard528;
        *var_guard529_slot = var_guard529;
        *var_guard530_slot = var_guard530;
        *var_guard531_slot = var_guard531;
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
        *var_i5_cor_slot = var_i5_cor;
        *var_i5_cor_dn5_slot = var_i5_cor_dn5;
        *var_i5_cor_dn6_slot = var_i5_cor_dn6;
        *var_i5_cor_dn7_slot = var_i5_cor_dn7;
        *var_i5_cor_dn8_slot = var_i5_cor_dn8;
        *var_isatfor2_s_slot = var_isatfor2_s;
        *var_isatfor2_s_dn5_slot = var_isatfor2_s_dn5;
        *var_isatfor2_s_dn6_slot = var_isatfor2_s_dn6;
        *var_isatfor2_s_dn7_slot = var_isatfor2_s_dn7;
        *var_isatfor2_s_dn8_slot = var_isatfor2_s_dn8;
        *var_isatrev_s_slot = var_isatrev_s;
        *var_isatrev_s_dn5_slot = var_isatrev_s_dn5;
        *var_isatrev_s_dn6_slot = var_isatrev_s_dn6;
        *var_isatrev_s_dn7_slot = var_isatrev_s_dn7;
        *var_isatrev_s_dn8_slot = var_isatrev_s_dn8;
        *var_m0_rev_slot = var_m0_rev;
        *var_m0_rev_dn5_slot = var_m0_rev_dn5;
        *var_m0_rev_dn6_slot = var_m0_rev_dn6;
        *var_m0_rev_dn7_slot = var_m0_rev_dn7;
        *var_m0_rev_dn8_slot = var_m0_rev_dn8;
        *var_m0flag_s_slot = var_m0flag_s;
        *var_mcor_rev_slot = var_mcor_rev;
        *var_mcor_rev_dn5_slot = var_mcor_rev_dn5;
        *var_mcor_rev_dn6_slot = var_mcor_rev_dn6;
        *var_mcor_rev_dn7_slot = var_mcor_rev_dn7;
        *var_mcor_rev_dn8_slot = var_mcor_rev_dn8;
        *var_mfor2_s_slot = var_mfor2_s;
        *var_mfor2_s_dn5_slot = var_mfor2_s_dn5;
        *var_mfor2_s_dn6_slot = var_mfor2_s_dn6;
        *var_mfor2_s_dn7_slot = var_mfor2_s_dn7;
        *var_mfor2_s_dn8_slot = var_mfor2_s_dn8;
        *var_mrev_s_slot = var_mrev_s;
        *var_mrev_s_dn5_slot = var_mrev_s_dn5;
        *var_mrev_s_dn6_slot = var_mrev_s_dn6;
        *var_mrev_s_dn7_slot = var_mrev_s_dn7;
        *var_mrev_s_dn8_slot = var_mrev_s_dn8;
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
        *var_xhighf1_s_slot = var_xhighf1_s;
        *var_xhighf2_s_slot = var_xhighf2_s;
        *var_xhighf2_s_dn5_slot = var_xhighf2_s_dn5;
        *var_xhighf2_s_dn6_slot = var_xhighf2_s_dn6;
        *var_xhighf2_s_dn7_slot = var_xhighf2_s_dn7;
        *var_xhighf2_s_dn8_slot = var_xhighf2_s_dn8;
        *var_xhighr_s_slot = var_xhighr_s;
        *var_xhighr_s_dn5_slot = var_xhighr_s_dn5;
        *var_xhighr_s_dn6_slot = var_xhighr_s_dn6;
        *var_xhighr_s_dn7_slot = var_xhighr_s_dn7;
        *var_xhighr_s_dn8_slot = var_xhighr_s_dn8;
        *var_zflagbot_s_slot = var_zflagbot_s;
        *var_zflaggat_s_slot = var_zflaggat_s;
        *var_zflagsti_s_slot = var_zflagsti_s;
        *var_zfrac_slot = var_zfrac;
    }

    pub(super) fn stamp_transient_block_54(
        var_abdrain_i: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_exp_vmax_over_phitd_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_idsatbot_d: f64,
        var_lgdrain_i: f64,
        var_lsdrain_i: f64,
        var_pbotd_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_vbbtlim_d: f64,
        var_vbibot_d: f64,
        var_vbimin_d: f64,
        var_vjunrefd_i: f64,
        var_vmax_d: f64,
        var_xhighf2_s: f64,
        var_xhighf2_s_dn5: f64,
        var_xhighf2_s_dn6: f64,
        var_xhighf2_s_dn7: f64,
        var_xhighf2_s_dn8: f64,
        var_dwsrh_slot: &mut f64,
        var_expxhf2_s_slot: &mut f64,
        var_expxhf2_s_dn5_slot: &mut f64,
        var_expxhf2_s_dn6_slot: &mut f64,
        var_expxhf2_s_dn7_slot: &mut f64,
        var_expxhf2_s_dn8_slot: &mut f64,
        var_expxhr_s_slot: &mut f64,
        var_expxhr_s_dn5_slot: &mut f64,
        var_expxhr_s_dn6_slot: &mut f64,
        var_expxhr_s_dn7_slot: &mut f64,
        var_expxhr_s_dn8_slot: &mut f64,
        var_fraci_slot: &mut f64,
        var_fracna_slot: &mut f64,
        var_fracnb_slot: &mut f64,
        var_guard532_slot: &mut f64,
        var_guard533_slot: &mut f64,
        var_guard534_slot: &mut f64,
        var_guard535_slot: &mut f64,
        var_guard536_slot: &mut f64,
        var_guard537_slot: &mut f64,
        var_guard538_slot: &mut f64,
        var_guard539_slot: &mut f64,
        var_guard540_slot: &mut f64,
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
        var_two_psistar_slot: &mut f64,
        var_v1_slot: &mut f64,
        var_v2_slot: &mut f64,
        var_v3_slot: &mut f64,
        var_v4_slot: &mut f64,
        var_v5_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_xhighr_s_slot: &mut f64,
        var_xhighr_s_dn5_slot: &mut f64,
        var_xhighr_s_dn6_slot: &mut f64,
        var_xhighr_s_dn7_slot: &mut f64,
        var_xhighr_s_dn8_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_expxhf2_s: f64 = *var_expxhf2_s_slot;
        let mut var_expxhf2_s_dn5: f64 = *var_expxhf2_s_dn5_slot;
        let mut var_expxhf2_s_dn6: f64 = *var_expxhf2_s_dn6_slot;
        let mut var_expxhf2_s_dn7: f64 = *var_expxhf2_s_dn7_slot;
        let mut var_expxhf2_s_dn8: f64 = *var_expxhf2_s_dn8_slot;
        let mut var_expxhr_s: f64 = *var_expxhr_s_slot;
        let mut var_expxhr_s_dn5: f64 = *var_expxhr_s_dn5_slot;
        let mut var_expxhr_s_dn6: f64 = *var_expxhr_s_dn6_slot;
        let mut var_expxhr_s_dn7: f64 = *var_expxhr_s_dn7_slot;
        let mut var_expxhr_s_dn8: f64 = *var_expxhr_s_dn8_slot;
        let mut var_fraci: f64 = *var_fraci_slot;
        let mut var_fracna: f64 = *var_fracna_slot;
        let mut var_fracnb: f64 = *var_fracnb_slot;
        let mut var_guard532: f64 = *var_guard532_slot;
        let mut var_guard533: f64 = *var_guard533_slot;
        let mut var_guard534: f64 = *var_guard534_slot;
        let mut var_guard535: f64 = *var_guard535_slot;
        let mut var_guard536: f64 = *var_guard536_slot;
        let mut var_guard537: f64 = *var_guard537_slot;
        let mut var_guard538: f64 = *var_guard538_slot;
        let mut var_guard539: f64 = *var_guard539_slot;
        let mut var_guard540: f64 = *var_guard540_slot;
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
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_v1: f64 = *var_v1_slot;
        let mut var_v2: f64 = *var_v2_slot;
        let mut var_v3: f64 = *var_v3_slot;
        let mut var_v4: f64 = *var_v4_slot;
        let mut var_v5: f64 = *var_v5_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_xhighr_s: f64 = *var_xhighr_s_slot;
        let mut var_xhighr_s_dn5: f64 = *var_xhighr_s_dn5_slot;
        let mut var_xhighr_s_dn6: f64 = *var_xhighr_s_dn6_slot;
        let mut var_xhighr_s_dn7: f64 = *var_xhighr_s_dn7_slot;
        let mut var_xhighr_s_dn8: f64 = *var_xhighr_s_dn8_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign27730_e33041, assign27730_e33041_d_n5, assign27730_e33041_d_n6, assign27730_e33041_d_n7, assign27730_e33041_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign27730_e33039: f64 = (var_xhighf2_s).exp();
        (assign27730_e33039, (assign27730_e33039 * var_xhighf2_s_dn5), (assign27730_e33039 * var_xhighf2_s_dn6), (assign27730_e33039 * var_xhighf2_s_dn7), (assign27730_e33039 * var_xhighf2_s_dn8),)
    } else {
        (var_expxhf2_s, var_expxhf2_s_dn5, var_expxhf2_s_dn6, var_expxhf2_s_dn7, var_expxhf2_s_dn8,)
    }
};
        var_expxhf2_s = assign27730_e33041;
        var_expxhf2_s_dn5 = assign27730_e33041_d_n5;
        var_expxhf2_s_dn6 = assign27730_e33041_d_n6;
        var_expxhf2_s_dn7 = assign27730_e33041_d_n7;
        var_expxhf2_s_dn8 = assign27730_e33041_d_n8;

        let (assign27740_e33049, assign27740_e33049_d_n5, assign27740_e33049_d_n6, assign27740_e33049_d_n7, assign27740_e33049_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign27740_e33047: f64 = (var_xhighr_s).min(230.25850929940458);
        (assign27740_e33047, if var_xhighr_s <= 230.25850929940458 { var_xhighr_s_dn5 } else { 0.0 }, if var_xhighr_s <= 230.25850929940458 { var_xhighr_s_dn6 } else { 0.0 }, if var_xhighr_s <= 230.25850929940458 { var_xhighr_s_dn7 } else { 0.0 }, if var_xhighr_s <= 230.25850929940458 { var_xhighr_s_dn8 } else { 0.0 },)
    } else {
        (var_xhighr_s, var_xhighr_s_dn5, var_xhighr_s_dn6, var_xhighr_s_dn7, var_xhighr_s_dn8,)
    }
};
        var_xhighr_s = assign27740_e33049;
        var_xhighr_s_dn5 = assign27740_e33049_d_n5;
        var_xhighr_s_dn6 = assign27740_e33049_d_n6;
        var_xhighr_s_dn7 = assign27740_e33049_d_n7;
        var_xhighr_s_dn8 = assign27740_e33049_d_n8;

        let (assign27750_e33056, assign27750_e33056_d_n5, assign27750_e33056_d_n6, assign27750_e33056_d_n7, assign27750_e33056_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign27750_e33054: f64 = (var_xhighr_s).exp();
        (assign27750_e33054, (assign27750_e33054 * var_xhighr_s_dn5), (assign27750_e33054 * var_xhighr_s_dn6), (assign27750_e33054 * var_xhighr_s_dn7), (assign27750_e33054 * var_xhighr_s_dn8),)
    } else {
        (var_expxhr_s, var_expxhr_s_dn5, var_expxhr_s_dn6, var_expxhr_s_dn7, var_expxhr_s_dn8,)
    }
};
        var_expxhr_s = assign27750_e33056;
        var_expxhr_s_dn5 = assign27750_e33056_d_n5;
        var_expxhr_s_dn6 = assign27750_e33056_d_n6;
        var_expxhr_s_dn7 = assign27750_e33056_d_n7;
        var_expxhr_s_dn8 = assign27750_e33056_d_n8;

        let (assign27760_e33062,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.4,)
    } else {
        (var_fracna,)
    }
};
        var_fracna = assign27760_e33062;

        let (assign27770_e33068,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.65,)
    } else {
        (var_fracnb,)
    }
};
        var_fracnb = assign27770_e33068;

        let (assign27780_e33074,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.8,)
    } else {
        (var_fraci,)
    }
};
        var_fraci = assign27780_e33074;

        let (assign27790_e33083,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign27790_e33079: f64 = (-var_fracna);
        let assign27790_e33081: f64 = (assign27790_e33079 * var_vjunrefd_i);
        (assign27790_e33081,)
    } else {
        (var_v1,)
    }
};
        var_v1 = assign27790_e33083;

        let (assign27800_e33092,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign27800_e33088: f64 = (-var_fracnb);
        let assign27800_e33090: f64 = (assign27800_e33088 * var_vjunrefd_i);
        (assign27800_e33090,)
    } else {
        (var_v2,)
    }
};
        var_v2 = assign27800_e33092;

        let (assign27810_e33101,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign27810_e33097: f64 = (-var_fraci);
        let assign27810_e33099: f64 = (assign27810_e33097 * var_vjunrefd_i);
        (assign27810_e33099,)
    } else {
        (var_v3,)
    }
};
        var_v3 = assign27810_e33101;

        let (assign27820_e33107,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.1,)
    } else {
        (var_v4,)
    }
};
        var_v4 = assign27820_e33107;

        let (assign27830_e33113,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.2,)
    } else {
        (var_v5,)
    }
};
        var_v5 = assign27830_e33113;

        let (assign27840_e33119,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign27840_e33119;

        let (assign27850_e33125,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign27850_e33125;

        let assign27860_e33137: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard532 = assign27860_e33137;

        let assign27940_e33223: f64 = if var_v1 < var_vmax_d { 1.0 } else { 0.0 };
        var_guard533 = assign27940_e33223;

        let assign27950_e33225: f64 = (-0.5);
        let assign27950_e33228: f64 = (var_v1 * var_phitdinv);
        let assign27950_e33229: f64 = (assign27950_e33225 * assign27950_e33228);
        let assign27950_e33230: f64 = (assign27950_e33229).abs();
        let assign27950_e33232: f64 = if assign27950_e33230 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard534 = assign27950_e33232;

        let (assign27960_e33250,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard532 != 0.0)) && (var_guard533 != 0.0)) && (var_guard534 != 0.0)) {
        let assign27960_e33243: f64 = (-0.5);
        let assign27960_e33246: f64 = (var_v1 * var_phitdinv);
        let assign27960_e33247: f64 = (assign27960_e33243 * assign27960_e33246);
        let assign27960_e33248: f64 = (assign27960_e33247).exp();
        (assign27960_e33248,)
    } else {
        (var_z,)
    }
};
        var_z = assign27960_e33250;

        let assign27970_e33252: f64 = (-0.5);
        let assign27970_e33255: f64 = (var_v1 * var_phitdinv);
        let assign27970_e33256: f64 = (assign27970_e33252 * assign27970_e33255);
        let assign27970_e33258: f64 = if assign27970_e33256 < 0.0 { 1.0 } else { 0.0 };
        var_guard535 = assign27970_e33258;

        let (assign27980_e33313,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard532 != 0.0)) && (var_guard533 != 0.0)) && (var_guard534 == 0.0)) && (var_guard535 != 0.0)) {
        let assign27980_e33274: f64 = (-230.25850929940458);
        let assign27980_e33276: f64 = (-0.5);
        let assign27980_e33279: f64 = (var_v1 * var_phitdinv);
        let assign27980_e33280: f64 = (assign27980_e33276 * assign27980_e33279);
        let assign27980_e33281: f64 = (assign27980_e33274 - assign27980_e33280);
        let assign27980_e33285: f64 = (-230.25850929940458);
        let assign27980_e33287: f64 = (-0.5);
        let assign27980_e33290: f64 = (var_v1 * var_phitdinv);
        let assign27980_e33291: f64 = (assign27980_e33287 * assign27980_e33290);
        let assign27980_e33292: f64 = (assign27980_e33285 - assign27980_e33291);
        let assign27980_e33295: f64 = (-230.25850929940458);
        let assign27980_e33297: f64 = (-0.5);
        let assign27980_e33300: f64 = (var_v1 * var_phitdinv);
        let assign27980_e33301: f64 = (assign27980_e33297 * assign27980_e33300);
        let assign27980_e33302: f64 = (assign27980_e33295 - assign27980_e33301);
        let assign27980_e33304: f64 = (assign27980_e33302 * 0.3333333333333333);
        let assign27980_e33305: f64 = (1.0 + assign27980_e33304);
        let assign27980_e33306: f64 = (assign27980_e33292 * assign27980_e33305);
        let assign27980_e33307: f64 = (0.5 * assign27980_e33306);
        let assign27980_e33308: f64 = (1.0 + assign27980_e33307);
        let assign27980_e33309: f64 = (assign27980_e33281 * assign27980_e33308);
        let assign27980_e33310: f64 = (1.0 + assign27980_e33309);
        let assign27980_e33311: f64 = (1e-100 / assign27980_e33310);
        (assign27980_e33311,)
    } else {
        (var_z,)
    }
};
        var_z = assign27980_e33313;

        let (assign27990_e33366,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard532 != 0.0)) && (var_guard533 != 0.0)) && (var_guard534 == 0.0)) && (var_guard535 == 0.0)) {
        let assign27990_e33330: f64 = (-0.5);
        let assign27990_e33333: f64 = (var_v1 * var_phitdinv);
        let assign27990_e33334: f64 = (assign27990_e33330 * assign27990_e33333);
        let assign27990_e33336: f64 = (assign27990_e33334 - 230.25850929940458);
        let assign27990_e33340: f64 = (-0.5);
        let assign27990_e33343: f64 = (var_v1 * var_phitdinv);
        let assign27990_e33344: f64 = (assign27990_e33340 * assign27990_e33343);
        let assign27990_e33346: f64 = (assign27990_e33344 - 230.25850929940458);
        let assign27990_e33349: f64 = (-0.5);
        let assign27990_e33352: f64 = (var_v1 * var_phitdinv);
        let assign27990_e33353: f64 = (assign27990_e33349 * assign27990_e33352);
        let assign27990_e33355: f64 = (assign27990_e33353 - 230.25850929940458);
        let assign27990_e33357: f64 = (assign27990_e33355 * 0.3333333333333333);
        let assign27990_e33358: f64 = (1.0 + assign27990_e33357);
        let assign27990_e33359: f64 = (assign27990_e33346 * assign27990_e33358);
        let assign27990_e33360: f64 = (0.5 * assign27990_e33359);
        let assign27990_e33361: f64 = (1.0 + assign27990_e33360);
        let assign27990_e33362: f64 = (assign27990_e33336 * assign27990_e33361);
        let assign27990_e33363: f64 = (1.0 + assign27990_e33362);
        let assign27990_e33364: f64 = (1e100 * assign27990_e33363);
        (assign27990_e33364,)
    } else {
        (var_z,)
    }
};
        var_z = assign27990_e33366;

        let (assign28000_e33378,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard532 != 0.0)) && (var_guard533 != 0.0)) {
        let assign28000_e33376: f64 = (1.0 / var_z);
        (assign28000_e33376,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign28000_e33378;

        let (assign28010_e33390,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard532 != 0.0)) && (var_guard533 != 0.0)) {
        let assign28010_e33388: f64 = (var_zinv * var_zinv);
        (assign28010_e33388,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign28010_e33390;

        let (assign28020_e33409,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard532 != 0.0)) && (var_guard533 == 0.0)) {
        let assign28020_e33402: f64 = (var_v1 - var_vmax_d);
        let assign28020_e33404: f64 = (assign28020_e33402 * var_phitdinv);
        let assign28020_e33405: f64 = (1.0 + assign28020_e33404);
        let assign28020_e33407: f64 = (assign28020_e33405 * var_exp_vmax_over_phitd_d);
        (assign28020_e33407,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign28020_e33409;

        let (assign28030_e33421,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard532 != 0.0)) && (var_guard533 == 0.0)) {
        let assign28030_e33419: f64 = (var_idmult).sqrt();
        (assign28030_e33419,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign28030_e33421;

        let (assign28040_e33434,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard532 != 0.0)) && (var_guard533 == 0.0)) {
        let assign28040_e33432: f64 = (1.0 / var_zinv);
        (assign28040_e33432,)
    } else {
        (var_z,)
    }
};
        var_z = assign28040_e33434;

        let (assign28050_e33444,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard532 != 0.0)) {
        let assign28050_e33442: f64 = (var_idmult - 1.0);
        (assign28050_e33442,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign28050_e33444;

        let assign28060_e33447: f64 = if var_v1 > 0.0 { 1.0 } else { 0.0 };
        var_guard536 = assign28060_e33447;

        let (assign28070_e33473,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard532 != 0.0)) && (var_guard536 != 0.0)) {
        let assign28070_e33459: f64 = (2.0 + var_z);
        let assign28070_e33462: f64 = (var_z + 1.0);
        let assign28070_e33465: f64 = (var_z + 3.0);
        let assign28070_e33466: f64 = (assign28070_e33462 * assign28070_e33465);
        let assign28070_e33467: f64 = (assign28070_e33466).sqrt();
        let assign28070_e33468: f64 = (assign28070_e33459 + assign28070_e33467);
        let assign28070_e33469: f64 = (assign28070_e33468).ln();
        let assign28070_e33470: f64 = (var_phitd * assign28070_e33469);
        let assign28070_e33471: f64 = (2.0 * assign28070_e33470);
        (assign28070_e33471,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign28070_e33473;

        let (assign28080_e33507,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard532 != 0.0)) && (var_guard536 == 0.0)) {
        let assign28080_e33483: f64 = (-var_v1);
        let assign28080_e33488: f64 = (2.0 * var_zinv);
        let assign28080_e33490: f64 = (assign28080_e33488 + 1.0);
        let assign28080_e33493: f64 = (1.0 + var_zinv);
        let assign28080_e33497: f64 = (3.0 * var_zinv);
        let assign28080_e33498: f64 = (1.0 + assign28080_e33497);
        let assign28080_e33499: f64 = (assign28080_e33493 * assign28080_e33498);
        let assign28080_e33500: f64 = (assign28080_e33499).sqrt();
        let assign28080_e33501: f64 = (assign28080_e33490 + assign28080_e33500);
        let assign28080_e33502: f64 = (assign28080_e33501).ln();
        let assign28080_e33503: f64 = (var_phitd * assign28080_e33502);
        let assign28080_e33504: f64 = (2.0 * assign28080_e33503);
        let assign28080_e33505: f64 = (assign28080_e33483 + assign28080_e33504);
        (assign28080_e33505,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign28080_e33507;

        let (assign28090_e33517,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard532 != 0.0)) {
        let assign28090_e33515: f64 = (var_vbimin_d - var_two_psistar);
        (assign28090_e33515,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign28090_e33517;

        let (assign28100_e33544,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard532 != 0.0)) {
        let assign28100_e33526: f64 = (var_v1 + var_vjlim);
        let assign28100_e33529: f64 = (var_v1 - var_vjlim);
        let assign28100_e33532: f64 = (var_v1 - var_vjlim);
        let assign28100_e33533: f64 = (assign28100_e33529 * assign28100_e33532);
        let assign28100_e33536: f64 = (4.0 * var_phitd);
        let assign28100_e33538: f64 = (assign28100_e33536 * var_phitd);
        let assign28100_e33539: f64 = (assign28100_e33533 + assign28100_e33538);
        let assign28100_e33540: f64 = (assign28100_e33539).sqrt();
        let assign28100_e33541: f64 = (assign28100_e33526 - assign28100_e33540);
        let assign28100_e33542: f64 = (0.5 * assign28100_e33541);
        (assign28100_e33542,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign28100_e33544;

        let (assign28110_e33571,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard532 != 0.0)) {
        let assign28110_e33553: f64 = (var_v1 + var_vbbtlim_d);
        let assign28110_e33556: f64 = (var_v1 - var_vbbtlim_d);
        let assign28110_e33559: f64 = (var_v1 - var_vbbtlim_d);
        let assign28110_e33560: f64 = (assign28110_e33556 * assign28110_e33559);
        let assign28110_e33563: f64 = (4.0 * var_phitr);
        let assign28110_e33565: f64 = (assign28110_e33563 * var_phitr);
        let assign28110_e33566: f64 = (assign28110_e33560 + assign28110_e33565);
        let assign28110_e33567: f64 = (assign28110_e33566).sqrt();
        let assign28110_e33568: f64 = (assign28110_e33553 - assign28110_e33567);
        let assign28110_e33569: f64 = (0.5 * assign28110_e33568);
        (assign28110_e33569,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign28110_e33571;

        let (assign28120_e33598,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard532 != 0.0)) {
        let assign28120_e33580: f64 = var_v1;
        let assign28120_e33583: f64 = var_v1;
        let assign28120_e33586: f64 = var_v1;
        let assign28120_e33587: f64 = (assign28120_e33583 * assign28120_e33586);
        let assign28120_e33590: f64 = (4.0 * 1e-6);
        let assign28120_e33592: f64 = (assign28120_e33590 * 1e-6);
        let assign28120_e33593: f64 = (assign28120_e33587 + assign28120_e33592);
        let assign28120_e33594: f64 = (assign28120_e33593).sqrt();
        let assign28120_e33595: f64 = (assign28120_e33580 - assign28120_e33594);
        let assign28120_e33596: f64 = (0.5 * assign28120_e33595);
        (assign28120_e33596,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign28120_e33598;

        let assign28130_e33601: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard537 = assign28130_e33601;

        let (assign28140_e33609, assign28140_e33609_d_n5, assign28140_e33609_d_n6, assign28140_e33609_d_n7, assign28140_e33609_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign28140_e33609;
        var_ijunbot_dn5 = assign28140_e33609_d_n5;
        var_ijunbot_dn6 = assign28140_e33609_d_n6;
        var_ijunbot_dn7 = assign28140_e33609_d_n7;
        var_ijunbot_dn8 = assign28140_e33609_d_n8;

        let (assign28150_e33620,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) {
        let assign28150_e33618: f64 = (var_idsatbot_d * var_idmult);
        (assign28150_e33618,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign28150_e33620;

        let assign28160_e33627: f64 = if ((var_csrhbotd_i == 0.0) && (var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard538 = assign28160_e33627;

        let (assign28170_e33638, assign28170_e33638_d_n5, assign28170_e33638_d_n6, assign28170_e33638_d_n7, assign28170_e33638_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard538 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign28170_e33638;
        var_isrh_dn5 = assign28170_e33638_d_n5;
        var_isrh_dn6 = assign28170_e33638_d_n6;
        var_isrh_dn7 = assign28170_e33638_d_n7;
        var_isrh_dn8 = assign28170_e33638_d_n8;

        let (assign28180_e33652,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard538 == 0.0)) {
        let assign28180_e33650: f64 = (var_vbibot_d - var_vjsrh);
        (assign28180_e33650,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign28180_e33652;

        let (assign28190_e33671,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard538 == 0.0)) {
        let assign28190_e33666: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign28190_e33667: f64 = (1.0 - assign28190_e33666);
        let assign28190_e33668: f64 = (assign28190_e33667).sqrt();
        let assign28190_e33669: f64 = (1.0 - assign28190_e33668);
        (assign28190_e33669,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign28190_e33671;

        let assign28200_e33674: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard539 = assign28200_e33674;

        let (assign28210_e33688,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard538 == 0.0)) && (var_guard539 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign28210_e33688;

        let (assign28220_e33720,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard538 == 0.0)) && (var_guard539 == 0.0)) {
        let assign28220_e33703: f64 = (var_wsrhstep * var_wsrhstep);
        let assign28220_e33705: f64 = (var_wsrhstep).ln();
        let assign28220_e33706: f64 = (assign28220_e33703 * assign28220_e33705);
        let assign28220_e33709: f64 = (1.0 - var_wsrhstep);
        let assign28220_e33710: f64 = (assign28220_e33706 / assign28220_e33709);
        let assign28220_e33712: f64 = (assign28220_e33710 + var_wsrhstep);
        let assign28220_e33716: f64 = (2.0 * var_pbotd_i);
        let assign28220_e33717: f64 = (1.0 - assign28220_e33716);
        let assign28220_e33718: f64 = (assign28220_e33712 * assign28220_e33717);
        (assign28220_e33718,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign28220_e33720;

        let (assign28230_e33734,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard538 == 0.0)) {
        let assign28230_e33732: f64 = (var_wsrhstep + var_dwsrh);
        (assign28230_e33732,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign28230_e33734;

        let assign28240_e33737: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard540 = assign28240_e33737;

        *var_dwsrh_slot = var_dwsrh;
        *var_expxhf2_s_slot = var_expxhf2_s;
        *var_expxhf2_s_dn5_slot = var_expxhf2_s_dn5;
        *var_expxhf2_s_dn6_slot = var_expxhf2_s_dn6;
        *var_expxhf2_s_dn7_slot = var_expxhf2_s_dn7;
        *var_expxhf2_s_dn8_slot = var_expxhf2_s_dn8;
        *var_expxhr_s_slot = var_expxhr_s;
        *var_expxhr_s_dn5_slot = var_expxhr_s_dn5;
        *var_expxhr_s_dn6_slot = var_expxhr_s_dn6;
        *var_expxhr_s_dn7_slot = var_expxhr_s_dn7;
        *var_expxhr_s_dn8_slot = var_expxhr_s_dn8;
        *var_fraci_slot = var_fraci;
        *var_fracna_slot = var_fracna;
        *var_fracnb_slot = var_fracnb;
        *var_guard532_slot = var_guard532;
        *var_guard533_slot = var_guard533;
        *var_guard534_slot = var_guard534;
        *var_guard535_slot = var_guard535;
        *var_guard536_slot = var_guard536;
        *var_guard537_slot = var_guard537;
        *var_guard538_slot = var_guard538;
        *var_guard539_slot = var_guard539;
        *var_guard540_slot = var_guard540;
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
        *var_two_psistar_slot = var_two_psistar;
        *var_v1_slot = var_v1;
        *var_v2_slot = var_v2;
        *var_v3_slot = var_v3;
        *var_v4_slot = var_v4;
        *var_v5_slot = var_v5;
        *var_vav_slot = var_vav;
        *var_vbbt_slot = var_vbbt;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_vjlim_slot = var_vjlim;
        *var_vjsrh_slot = var_vjsrh;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_xhighr_s_slot = var_xhighr_s;
        *var_xhighr_s_dn5_slot = var_xhighr_s_dn5;
        *var_xhighr_s_dn6_slot = var_xhighr_s_dn6;
        *var_xhighr_s_dn7_slot = var_xhighr_s_dn7;
        *var_xhighr_s_dn8_slot = var_xhighr_s_dn8;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_55(
        var_atatbot_d: f64,
        var_berfc: f64,
        var_btatpartbot_d: f64,
        var_cerfc: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_ftdbot_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard537: f64,
        var_guard538: f64,
        var_guard540: f64,
        var_one_minus_pbot_d: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_perfc: f64,
        var_vbi_minus_vjsrh: f64,
        var_vbirbotinv_d: f64,
        var_wdepnulrbot_d: f64,
        var_wsrh: f64,
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
        var_guard541_slot: &mut f64,
        var_guard542_slot: &mut f64,
        var_guard543_slot: &mut f64,
        var_guard544_slot: &mut f64,
        var_guard545_slot: &mut f64,
        var_guard546_slot: &mut f64,
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
        let mut var_guard541: f64 = *var_guard541_slot;
        let mut var_guard542: f64 = *var_guard542_slot;
        let mut var_guard543: f64 = *var_guard543_slot;
        let mut var_guard544: f64 = *var_guard544_slot;
        let mut var_guard545: f64 = *var_guard545_slot;
        let mut var_guard546: f64 = *var_guard546_slot;
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

        let (assign28250_e33754, assign28250_e33754_d_n5, assign28250_e33754_d_n6, assign28250_e33754_d_n7, assign28250_e33754_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard538 == 0.0)) && (var_guard540 != 0.0)) {
        let assign28250_e33751: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign28250_e33752: f64 = (assign28250_e33751).sqrt();
        (assign28250_e33752, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28250_e33754;
        var_tmp_dn5 = assign28250_e33754_d_n5;
        var_tmp_dn6 = assign28250_e33754_d_n6;
        var_tmp_dn7 = assign28250_e33754_d_n7;
        var_tmp_dn8 = assign28250_e33754_d_n8;

        let (assign28260_e33773, assign28260_e33773_d_n5, assign28260_e33773_d_n6, assign28260_e33773_d_n7, assign28260_e33773_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard538 == 0.0)) && (var_guard540 == 0.0)) {
        let assign28260_e33769: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign28260_e33771: f64 = (assign28260_e33769).powf(var_pbotd_i);
        (assign28260_e33771, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28260_e33773;
        var_tmp_dn5 = assign28260_e33773_d_n5;
        var_tmp_dn6 = assign28260_e33773_d_n6;
        var_tmp_dn7 = assign28260_e33773_d_n7;
        var_tmp_dn8 = assign28260_e33773_d_n8;

        let (assign28270_e33787, assign28270_e33787_d_n5, assign28270_e33787_d_n6, assign28270_e33787_d_n7, assign28270_e33787_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard538 == 0.0)) {
        let assign28270_e33785: f64 = (var_wdepnulrbot_d * var_tmp);
        (assign28270_e33785, (var_wdepnulrbot_d * var_tmp_dn5), (var_wdepnulrbot_d * var_tmp_dn6), (var_wdepnulrbot_d * var_tmp_dn7), (var_wdepnulrbot_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign28270_e33787;
        var_wdep_dn5 = assign28270_e33787_d_n5;
        var_wdep_dn6 = assign28270_e33787_d_n6;
        var_wdep_dn7 = assign28270_e33787_d_n7;
        var_wdep_dn8 = assign28270_e33787_d_n8;

        let (assign28280_e33805, assign28280_e33805_d_n5, assign28280_e33805_d_n6, assign28280_e33805_d_n7, assign28280_e33805_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard538 == 0.0)) {
        let assign28280_e33800: f64 = (var_zinv - 1.0);
        let assign28280_e33802: f64 = (assign28280_e33800 * var_wdep);
        let assign28280_e33803: f64 = (var_ftdbot_d * assign28280_e33802);
        (assign28280_e33803, (var_ftdbot_d * (assign28280_e33800 * var_wdep_dn5)), (var_ftdbot_d * (assign28280_e33800 * var_wdep_dn6)), (var_ftdbot_d * (assign28280_e33800 * var_wdep_dn7)), (var_ftdbot_d * (assign28280_e33800 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign28280_e33805;
        var_asrh_dn5 = assign28280_e33805_d_n5;
        var_asrh_dn6 = assign28280_e33805_d_n6;
        var_asrh_dn7 = assign28280_e33805_d_n7;
        var_asrh_dn8 = assign28280_e33805_d_n8;

        let (assign28290_e33821, assign28290_e33821_d_n5, assign28290_e33821_d_n6, assign28290_e33821_d_n7, assign28290_e33821_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard538 == 0.0)) {
        let assign28290_e33818: f64 = (var_asrh * var_wsrh);
        let assign28290_e33819: f64 = (var_csrhbotd_i * assign28290_e33818);
        (assign28290_e33819, (var_csrhbotd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign28290_e33821;
        var_isrh_dn5 = assign28290_e33821_d_n5;
        var_isrh_dn6 = assign28290_e33821_d_n6;
        var_isrh_dn7 = assign28290_e33821_d_n7;
        var_isrh_dn8 = assign28290_e33821_d_n8;

        let assign28300_e33824: f64 = if var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard541 = assign28300_e33824;

        let (assign28310_e33835, assign28310_e33835_d_n5, assign28310_e33835_d_n6, assign28310_e33835_d_n7, assign28310_e33835_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign28310_e33835;
        var_itat_dn5 = assign28310_e33835_d_n5;
        var_itat_dn6 = assign28310_e33835_d_n6;
        var_itat_dn7 = assign28310_e33835_d_n7;
        var_itat_dn8 = assign28310_e33835_d_n8;

        let (assign28320_e33853, assign28320_e33853_d_n5, assign28320_e33853_d_n6, assign28320_e33853_d_n7, assign28320_e33853_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) {
        let assign28320_e33848: f64 = (var_wdep * var_one_minus_pbot_d);
        let assign28320_e33850: f64 = (assign28320_e33848 / var_vbi_minus_vjsrh);
        let assign28320_e33851: f64 = (var_btatpartbot_d * assign28320_e33850);
        (assign28320_e33851, (var_btatpartbot_d * ((var_wdep_dn5 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn6 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn7 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn8 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign28320_e33853;
        var_btat_dn5 = assign28320_e33853_d_n5;
        var_btat_dn6 = assign28320_e33853_d_n6;
        var_btat_dn7 = assign28320_e33853_d_n7;
        var_btat_dn8 = assign28320_e33853_d_n8;

        let (assign28330_e33869, assign28330_e33869_d_n5, assign28330_e33869_d_n6, assign28330_e33869_d_n7, assign28330_e33869_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) {
        let assign28330_e33865: f64 = (0.666666666666667 * var_atatbot_d);
        let assign28330_e33867: f64 = (assign28330_e33865 / var_btat);
        (assign28330_e33867, (-((assign28330_e33865 * var_btat_dn5) / (var_btat * var_btat))), (-((assign28330_e33865 * var_btat_dn6) / (var_btat * var_btat))), (-((assign28330_e33865 * var_btat_dn7) / (var_btat * var_btat))), (-((assign28330_e33865 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign28330_e33869;
        var_twoatatoverthreebtat_dn5 = assign28330_e33869_d_n5;
        var_twoatatoverthreebtat_dn6 = assign28330_e33869_d_n6;
        var_twoatatoverthreebtat_dn7 = assign28330_e33869_d_n7;
        var_twoatatoverthreebtat_dn8 = assign28330_e33869_d_n8;

        let (assign28340_e33883, assign28340_e33883_d_n5, assign28340_e33883_d_n6, assign28340_e33883_d_n7, assign28340_e33883_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) {
        let assign28340_e33881: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign28340_e33881, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign28340_e33883;
        var_umaxbeforelimiting_dn5 = assign28340_e33883_d_n5;
        var_umaxbeforelimiting_dn6 = assign28340_e33883_d_n6;
        var_umaxbeforelimiting_dn7 = assign28340_e33883_d_n7;
        var_umaxbeforelimiting_dn8 = assign28340_e33883_d_n8;

        let (assign28350_e33904, assign28350_e33904_d_n5, assign28350_e33904_d_n6, assign28350_e33904_d_n7, assign28350_e33904_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) {
        let assign28350_e33895: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign28350_e33898: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign28350_e33900: f64 = (assign28350_e33898 + 1.0);
        let assign28350_e33901: f64 = (assign28350_e33895 / assign28350_e33900);
        let assign28350_e33902: f64 = (assign28350_e33901).sqrt();
        (assign28350_e33902, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign28350_e33900) - (assign28350_e33895 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign28350_e33900 * assign28350_e33900)) / (2.0 * assign28350_e33902)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign28350_e33900) - (assign28350_e33895 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign28350_e33900 * assign28350_e33900)) / (2.0 * assign28350_e33902)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign28350_e33900) - (assign28350_e33895 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign28350_e33900 * assign28350_e33900)) / (2.0 * assign28350_e33902)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign28350_e33900) - (assign28350_e33895 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign28350_e33900 * assign28350_e33900)) / (2.0 * assign28350_e33902)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign28350_e33904;
        var_umax_dn5 = assign28350_e33904_d_n5;
        var_umax_dn6 = assign28350_e33904_d_n6;
        var_umax_dn7 = assign28350_e33904_d_n7;
        var_umax_dn8 = assign28350_e33904_d_n8;

        let (assign28360_e33917, assign28360_e33917_d_n5, assign28360_e33917_d_n6, assign28360_e33917_d_n7, assign28360_e33917_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) {
        let assign28360_e33915: f64 = (var_umax).sqrt();
        (assign28360_e33915, (var_umax_dn5 / (2.0 * assign28360_e33915)), (var_umax_dn6 / (2.0 * assign28360_e33915)), (var_umax_dn7 / (2.0 * assign28360_e33915)), (var_umax_dn8 / (2.0 * assign28360_e33915)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign28360_e33917;
        var_sqrtumax_dn5 = assign28360_e33917_d_n5;
        var_sqrtumax_dn6 = assign28360_e33917_d_n6;
        var_sqrtumax_dn7 = assign28360_e33917_d_n7;
        var_sqrtumax_dn8 = assign28360_e33917_d_n8;

        let (assign28370_e33931, assign28370_e33931_d_n5, assign28370_e33931_d_n6, assign28370_e33931_d_n7, assign28370_e33931_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) {
        let assign28370_e33929: f64 = (var_umax * var_sqrtumax);
        (assign28370_e33929, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign28370_e33931;
        var_umaxpoweronepointfive_dn5 = assign28370_e33931_d_n5;
        var_umaxpoweronepointfive_dn6 = assign28370_e33931_d_n6;
        var_umaxpoweronepointfive_dn7 = assign28370_e33931_d_n7;
        var_umaxpoweronepointfive_dn8 = assign28370_e33931_d_n8;

        let assign28380_e33933: f64 = (-var_pbotd_i);
        let assign28380_e33935: f64 = (assign28380_e33933 * var_one_over_one_minus_pbot_d);
        let assign28380_e33937: f64 = (-1.0);
        let assign28380_e33938: f64 = if assign28380_e33935 == assign28380_e33937 { 1.0 } else { 0.0 };
        var_guard542 = assign28380_e33938;

        let (assign28390_e33958, assign28390_e33958_d_n5, assign28390_e33958_d_n6, assign28390_e33958_d_n7, assign28390_e33958_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) && (var_guard542 != 0.0)) {
        let assign28390_e33954: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign28390_e33955: f64 = (1.0 + assign28390_e33954);
        let assign28390_e33956: f64 = (1.0 / assign28390_e33955);
        (assign28390_e33956, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign28390_e33955 * assign28390_e33955))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign28390_e33955 * assign28390_e33955))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign28390_e33955 * assign28390_e33955))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign28390_e33955 * assign28390_e33955))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign28390_e33958;
        var_wgamma_dn5 = assign28390_e33958_d_n5;
        var_wgamma_dn6 = assign28390_e33958_d_n6;
        var_wgamma_dn7 = assign28390_e33958_d_n7;
        var_wgamma_dn8 = assign28390_e33958_d_n8;

        let (assign28400_e33982, assign28400_e33982_d_n5, assign28400_e33982_d_n6, assign28400_e33982_d_n7, assign28400_e33982_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) && (var_guard542 == 0.0)) {
        let assign28400_e33974: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign28400_e33975: f64 = (1.0 + assign28400_e33974);
        let assign28400_e33977: f64 = (-var_pbotd_i);
        let assign28400_e33979: f64 = (assign28400_e33977 * var_one_over_one_minus_pbot_d);
        let assign28400_e33980: f64 = (assign28400_e33975).powf(assign28400_e33979);
        (assign28400_e33980, if 0.0 == 0.0 && ((assign28400_e33979) as f64).is_finite() && ((assign28400_e33979) as f64).fract() == 0.0 { if assign28400_e33979 == 0.0 { 0.0 } else { (assign28400_e33979 * ((assign28400_e33975).powf(assign28400_e33979 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign28400_e33980 * (assign28400_e33979 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign28400_e33975))) }, if 0.0 == 0.0 && ((assign28400_e33979) as f64).is_finite() && ((assign28400_e33979) as f64).fract() == 0.0 { if assign28400_e33979 == 0.0 { 0.0 } else { (assign28400_e33979 * ((assign28400_e33975).powf(assign28400_e33979 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign28400_e33980 * (assign28400_e33979 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign28400_e33975))) }, if 0.0 == 0.0 && ((assign28400_e33979) as f64).is_finite() && ((assign28400_e33979) as f64).fract() == 0.0 { if assign28400_e33979 == 0.0 { 0.0 } else { (assign28400_e33979 * ((assign28400_e33975).powf(assign28400_e33979 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign28400_e33980 * (assign28400_e33979 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign28400_e33975))) }, if 0.0 == 0.0 && ((assign28400_e33979) as f64).is_finite() && ((assign28400_e33979) as f64).fract() == 0.0 { if assign28400_e33979 == 0.0 { 0.0 } else { (assign28400_e33979 * ((assign28400_e33975).powf(assign28400_e33979 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign28400_e33980 * (assign28400_e33979 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign28400_e33975))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign28400_e33982;
        var_wgamma_dn5 = assign28400_e33982_d_n5;
        var_wgamma_dn6 = assign28400_e33982_d_n6;
        var_wgamma_dn7 = assign28400_e33982_d_n7;
        var_wgamma_dn8 = assign28400_e33982_d_n8;

        let (assign28410_e34000, assign28410_e34000_d_n5, assign28410_e34000_d_n6, assign28410_e34000_d_n7, assign28410_e34000_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) {
        let assign28410_e33994: f64 = (var_wsrh * var_wgamma);
        let assign28410_e33997: f64 = (var_wsrh + var_wgamma);
        let assign28410_e33998: f64 = (assign28410_e33994 / assign28410_e33997);
        (assign28410_e33998, ((((var_wsrh * var_wgamma_dn5) * assign28410_e33997) - (assign28410_e33994 * var_wgamma_dn5)) / (assign28410_e33997 * assign28410_e33997)), ((((var_wsrh * var_wgamma_dn6) * assign28410_e33997) - (assign28410_e33994 * var_wgamma_dn6)) / (assign28410_e33997 * assign28410_e33997)), ((((var_wsrh * var_wgamma_dn7) * assign28410_e33997) - (assign28410_e33994 * var_wgamma_dn7)) / (assign28410_e33997 * assign28410_e33997)), ((((var_wsrh * var_wgamma_dn8) * assign28410_e33997) - (assign28410_e33994 * var_wgamma_dn8)) / (assign28410_e33997 * assign28410_e33997)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign28410_e34000;
        var_wtat_dn5 = assign28410_e34000_d_n5;
        var_wtat_dn6 = assign28410_e34000_d_n6;
        var_wtat_dn7 = assign28410_e34000_d_n7;
        var_wtat_dn8 = assign28410_e34000_d_n8;

        let (assign28420_e34017, assign28420_e34017_d_n5, assign28420_e34017_d_n6, assign28420_e34017_d_n7, assign28420_e34017_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) {
        let assign28420_e34013: f64 = (var_btat / var_sqrtumax);
        let assign28420_e34014: f64 = (0.375 * assign28420_e34013);
        let assign28420_e34015: f64 = (assign28420_e34014).sqrt();
        (assign28420_e34015, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign28420_e34015)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign28420_e34015)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign28420_e34015)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign28420_e34015)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign28420_e34017;
        var_ktat_dn5 = assign28420_e34017_d_n5;
        var_ktat_dn6 = assign28420_e34017_d_n6;
        var_ktat_dn7 = assign28420_e34017_d_n7;
        var_ktat_dn8 = assign28420_e34017_d_n8;

        let (assign28430_e34035, assign28430_e34035_d_n5, assign28430_e34035_d_n6, assign28430_e34035_d_n7, assign28430_e34035_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) {
        let assign28430_e34030: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign28430_e34031: f64 = (2.0 * assign28430_e34030);
        let assign28430_e34033: f64 = (assign28430_e34031 - var_umax);
        (assign28430_e34033, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign28430_e34035;
        var_ltat_dn5 = assign28430_e34035_d_n5;
        var_ltat_dn6 = assign28430_e34035_d_n6;
        var_ltat_dn7 = assign28430_e34035_d_n7;
        var_ltat_dn8 = assign28430_e34035_d_n8;

        let (assign28440_e34061, assign28440_e34061_d_n5, assign28440_e34061_d_n6, assign28440_e34061_d_n7, assign28440_e34061_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) {
        let assign28440_e34047: f64 = (var_atatbot_d * var_twoatatoverthreebtat);
        let assign28440_e34049: f64 = (assign28440_e34047 * var_sqrtumax);
        let assign28440_e34052: f64 = (var_atatbot_d * var_umax);
        let assign28440_e34053: f64 = (assign28440_e34049 - assign28440_e34052);
        let assign28440_e34057: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign28440_e34058: f64 = (0.5 * assign28440_e34057);
        let assign28440_e34059: f64 = (assign28440_e34053 + assign28440_e34058);
        (assign28440_e34059, (((((var_atatbot_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign28440_e34047 * var_sqrtumax_dn5)) - (var_atatbot_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign28440_e34047 * var_sqrtumax_dn6)) - (var_atatbot_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign28440_e34047 * var_sqrtumax_dn7)) - (var_atatbot_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign28440_e34047 * var_sqrtumax_dn8)) - (var_atatbot_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign28440_e34061;
        var_mtat_dn5 = assign28440_e34061_d_n5;
        var_mtat_dn6 = assign28440_e34061_d_n6;
        var_mtat_dn7 = assign28440_e34061_d_n7;
        var_mtat_dn8 = assign28440_e34061_d_n8;

        let (assign28450_e34077, assign28450_e34077_d_n5, assign28450_e34077_d_n6, assign28450_e34077_d_n7, assign28450_e34077_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) {
        let assign28450_e34073: f64 = (var_ltat - 1.0);
        let assign28450_e34075: f64 = (assign28450_e34073 * var_ktat);
        (assign28450_e34075, ((var_ltat_dn5 * var_ktat) + (assign28450_e34073 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign28450_e34073 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign28450_e34073 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign28450_e34073 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign28450_e34077;
        var_xerfc_dn5 = assign28450_e34077_d_n5;
        var_xerfc_dn6 = assign28450_e34077_d_n6;
        var_xerfc_dn7 = assign28450_e34077_d_n7;
        var_xerfc_dn8 = assign28450_e34077_d_n8;

        let (assign28460_e34091, assign28460_e34091_d_n5, assign28460_e34091_d_n6, assign28460_e34091_d_n7, assign28460_e34091_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) {
        let assign28460_e34089: f64 = (var_xerfc * var_xerfc);
        (assign28460_e34089, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign28460_e34091;
        var_ysq_dn5 = assign28460_e34091_d_n5;
        var_ysq_dn6 = assign28460_e34091_d_n6;
        var_ysq_dn7 = assign28460_e34091_d_n7;
        var_ysq_dn8 = assign28460_e34091_d_n8;

        let assign28470_e34094: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard543 = assign28470_e34094;

        let (assign28480_e34114, assign28480_e34114_d_n5, assign28480_e34114_d_n6, assign28480_e34114_d_n7, assign28480_e34114_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) && (var_guard543 != 0.0)) {
        let assign28480_e34110: f64 = (var_perfc * var_xerfc);
        let assign28480_e34111: f64 = (1.0 + assign28480_e34110);
        let assign28480_e34112: f64 = (1.0 / assign28480_e34111);
        (assign28480_e34112, (-((var_perfc * var_xerfc_dn5) / (assign28480_e34111 * assign28480_e34111))), (-((var_perfc * var_xerfc_dn6) / (assign28480_e34111 * assign28480_e34111))), (-((var_perfc * var_xerfc_dn7) / (assign28480_e34111 * assign28480_e34111))), (-((var_perfc * var_xerfc_dn8) / (assign28480_e34111 * assign28480_e34111))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign28480_e34114;
        var_terfc_dn5 = assign28480_e34114_d_n5;
        var_terfc_dn6 = assign28480_e34114_d_n6;
        var_terfc_dn7 = assign28480_e34114_d_n7;
        var_terfc_dn8 = assign28480_e34114_d_n8;

        let (assign28490_e34135, assign28490_e34135_d_n5, assign28490_e34135_d_n6, assign28490_e34135_d_n7, assign28490_e34135_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) && (var_guard543 == 0.0)) {
        let assign28490_e34131: f64 = (var_perfc * var_xerfc);
        let assign28490_e34132: f64 = (1.0 - assign28490_e34131);
        let assign28490_e34133: f64 = (1.0 / assign28490_e34132);
        (assign28490_e34133, (-((-(var_perfc * var_xerfc_dn5)) / (assign28490_e34132 * assign28490_e34132))), (-((-(var_perfc * var_xerfc_dn6)) / (assign28490_e34132 * assign28490_e34132))), (-((-(var_perfc * var_xerfc_dn7)) / (assign28490_e34132 * assign28490_e34132))), (-((-(var_perfc * var_xerfc_dn8)) / (assign28490_e34132 * assign28490_e34132))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign28490_e34135;
        var_terfc_dn5 = assign28490_e34135_d_n5;
        var_terfc_dn6 = assign28490_e34135_d_n6;
        var_terfc_dn7 = assign28490_e34135_d_n7;
        var_terfc_dn8 = assign28490_e34135_d_n8;

        let assign28500_e34137: f64 = (-var_ysq);
        let assign28500_e34139: f64 = (assign28500_e34137 + var_mtat);
        let assign28500_e34141: f64 = (-230.25850929940458);
        let assign28500_e34142: f64 = if assign28500_e34139 > assign28500_e34141 { 1.0 } else { 0.0 };
        var_guard544 = assign28500_e34142;

        let (assign28510_e34160, assign28510_e34160_d_n5, assign28510_e34160_d_n6, assign28510_e34160_d_n7, assign28510_e34160_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) && (var_guard544 != 0.0)) {
        let assign28510_e34155: f64 = (-var_ysq);
        let assign28510_e34157: f64 = (assign28510_e34155 + var_mtat);
        let assign28510_e34158: f64 = (assign28510_e34157).exp();
        (assign28510_e34158, (assign28510_e34158 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign28510_e34158 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign28510_e34158 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign28510_e34158 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28510_e34160;
        var_tmp_dn5 = assign28510_e34160_d_n5;
        var_tmp_dn6 = assign28510_e34160_d_n6;
        var_tmp_dn7 = assign28510_e34160_d_n7;
        var_tmp_dn8 = assign28510_e34160_d_n8;

        let (assign28520_e34209, assign28520_e34209_d_n5, assign28520_e34209_d_n6, assign28520_e34209_d_n7, assign28520_e34209_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) && (var_guard544 == 0.0)) {
        let assign28520_e34176: f64 = (-230.25850929940458);
        let assign28520_e34178: f64 = (-var_ysq);
        let assign28520_e34180: f64 = (assign28520_e34178 + var_mtat);
        let assign28520_e34181: f64 = (assign28520_e34176 - assign28520_e34180);
        let assign28520_e34185: f64 = (-230.25850929940458);
        let assign28520_e34187: f64 = (-var_ysq);
        let assign28520_e34189: f64 = (assign28520_e34187 + var_mtat);
        let assign28520_e34190: f64 = (assign28520_e34185 - assign28520_e34189);
        let assign28520_e34193: f64 = (-230.25850929940458);
        let assign28520_e34195: f64 = (-var_ysq);
        let assign28520_e34197: f64 = (assign28520_e34195 + var_mtat);
        let assign28520_e34198: f64 = (assign28520_e34193 - assign28520_e34197);
        let assign28520_e34200: f64 = (assign28520_e34198 * 0.3333333333333333);
        let assign28520_e34201: f64 = (1.0 + assign28520_e34200);
        let assign28520_e34202: f64 = (assign28520_e34190 * assign28520_e34201);
        let assign28520_e34203: f64 = (0.5 * assign28520_e34202);
        let assign28520_e34204: f64 = (1.0 + assign28520_e34203);
        let assign28520_e34205: f64 = (assign28520_e34181 * assign28520_e34204);
        let assign28520_e34206: f64 = (1.0 + assign28520_e34205);
        let assign28520_e34207: f64 = (1e-100 / assign28520_e34206);
        (assign28520_e34207, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign28520_e34204) + (assign28520_e34181 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign28520_e34201) + (assign28520_e34190 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign28520_e34206 * assign28520_e34206))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign28520_e34204) + (assign28520_e34181 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign28520_e34201) + (assign28520_e34190 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign28520_e34206 * assign28520_e34206))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign28520_e34204) + (assign28520_e34181 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign28520_e34201) + (assign28520_e34190 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign28520_e34206 * assign28520_e34206))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign28520_e34204) + (assign28520_e34181 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign28520_e34201) + (assign28520_e34190 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign28520_e34206 * assign28520_e34206))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28520_e34209;
        var_tmp_dn5 = assign28520_e34209_d_n5;
        var_tmp_dn6 = assign28520_e34209_d_n6;
        var_tmp_dn7 = assign28520_e34209_d_n7;
        var_tmp_dn8 = assign28520_e34209_d_n8;

        let (assign28530_e34239, assign28530_e34239_d_n5, assign28530_e34239_d_n6, assign28530_e34239_d_n7, assign28530_e34239_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) {
        let assign28530_e34221: f64 = (0.29214664 * var_terfc);
        let assign28530_e34225: f64 = (var_terfc * var_terfc);
        let assign28530_e34226: f64 = (var_berfc * assign28530_e34225);
        let assign28530_e34227: f64 = (assign28530_e34221 + assign28530_e34226);
        let assign28530_e34231: f64 = (var_terfc * var_terfc);
        let assign28530_e34233: f64 = (assign28530_e34231 * var_terfc);
        let assign28530_e34234: f64 = (var_cerfc * assign28530_e34233);
        let assign28530_e34235: f64 = (assign28530_e34227 + assign28530_e34234);
        let assign28530_e34237: f64 = (assign28530_e34235 * var_tmp);
        (assign28530_e34237, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign28530_e34231 * var_terfc_dn5)))) * var_tmp) + (assign28530_e34235 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign28530_e34231 * var_terfc_dn6)))) * var_tmp) + (assign28530_e34235 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign28530_e34231 * var_terfc_dn7)))) * var_tmp) + (assign28530_e34235 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign28530_e34231 * var_terfc_dn8)))) * var_tmp) + (assign28530_e34235 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign28530_e34239;
        var_erfcpos_dn5 = assign28530_e34239_d_n5;
        var_erfcpos_dn6 = assign28530_e34239_d_n6;
        var_erfcpos_dn7 = assign28530_e34239_d_n7;
        var_erfcpos_dn8 = assign28530_e34239_d_n8;

        let assign28540_e34242: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard545 = assign28540_e34242;

        let (assign28550_e34256, assign28550_e34256_d_n5, assign28550_e34256_d_n6, assign28550_e34256_d_n7, assign28550_e34256_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) && (var_guard545 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign28550_e34256;
        var_erfctimesexpmtat_dn5 = assign28550_e34256_d_n5;
        var_erfctimesexpmtat_dn6 = assign28550_e34256_d_n6;
        var_erfctimesexpmtat_dn7 = assign28550_e34256_d_n7;
        var_erfctimesexpmtat_dn8 = assign28550_e34256_d_n8;

        let assign28560_e34259: f64 = (-230.25850929940458);
        let assign28560_e34260: f64 = if var_mtat > assign28560_e34259 { 1.0 } else { 0.0 };
        var_guard546 = assign28560_e34260;

        let (assign28570_e34278, assign28570_e34278_d_n5, assign28570_e34278_d_n6, assign28570_e34278_d_n7, assign28570_e34278_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) && (var_guard545 == 0.0)) && (var_guard546 != 0.0)) {
        let assign28570_e34276: f64 = (var_mtat).exp();
        (assign28570_e34276, (assign28570_e34276 * var_mtat_dn5), (assign28570_e34276 * var_mtat_dn6), (assign28570_e34276 * var_mtat_dn7), (assign28570_e34276 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28570_e34278;
        var_tmp_dn5 = assign28570_e34278_d_n5;
        var_tmp_dn6 = assign28570_e34278_d_n6;
        var_tmp_dn7 = assign28570_e34278_d_n7;
        var_tmp_dn8 = assign28570_e34278_d_n8;

        let (assign28580_e34321, assign28580_e34321_d_n5, assign28580_e34321_d_n6, assign28580_e34321_d_n7, assign28580_e34321_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) && (var_guard545 == 0.0)) && (var_guard546 == 0.0)) {
        let assign28580_e34297: f64 = (-230.25850929940458);
        let assign28580_e34299: f64 = (assign28580_e34297 - var_mtat);
        let assign28580_e34303: f64 = (-230.25850929940458);
        let assign28580_e34305: f64 = (assign28580_e34303 - var_mtat);
        let assign28580_e34308: f64 = (-230.25850929940458);
        let assign28580_e34310: f64 = (assign28580_e34308 - var_mtat);
        let assign28580_e34312: f64 = (assign28580_e34310 * 0.3333333333333333);
        let assign28580_e34313: f64 = (1.0 + assign28580_e34312);
        let assign28580_e34314: f64 = (assign28580_e34305 * assign28580_e34313);
        let assign28580_e34315: f64 = (0.5 * assign28580_e34314);
        let assign28580_e34316: f64 = (1.0 + assign28580_e34315);
        let assign28580_e34317: f64 = (assign28580_e34299 * assign28580_e34316);
        let assign28580_e34318: f64 = (1.0 + assign28580_e34317);
        let assign28580_e34319: f64 = (1e-100 / assign28580_e34318);
        (assign28580_e34319, (-((1e-100 * (((-var_mtat_dn5) * assign28580_e34316) + (assign28580_e34299 * (0.5 * (((-var_mtat_dn5) * assign28580_e34313) + (assign28580_e34305 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign28580_e34318 * assign28580_e34318))), (-((1e-100 * (((-var_mtat_dn6) * assign28580_e34316) + (assign28580_e34299 * (0.5 * (((-var_mtat_dn6) * assign28580_e34313) + (assign28580_e34305 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign28580_e34318 * assign28580_e34318))), (-((1e-100 * (((-var_mtat_dn7) * assign28580_e34316) + (assign28580_e34299 * (0.5 * (((-var_mtat_dn7) * assign28580_e34313) + (assign28580_e34305 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign28580_e34318 * assign28580_e34318))), (-((1e-100 * (((-var_mtat_dn8) * assign28580_e34316) + (assign28580_e34299 * (0.5 * (((-var_mtat_dn8) * assign28580_e34313) + (assign28580_e34305 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign28580_e34318 * assign28580_e34318))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28580_e34321;
        var_tmp_dn5 = assign28580_e34321_d_n5;
        var_tmp_dn6 = assign28580_e34321_d_n6;
        var_tmp_dn7 = assign28580_e34321_d_n7;
        var_tmp_dn8 = assign28580_e34321_d_n8;

        let (assign28590_e34340, assign28590_e34340_d_n5, assign28590_e34340_d_n6, assign28590_e34340_d_n7, assign28590_e34340_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) && (var_guard545 == 0.0)) {
        let assign28590_e34336: f64 = (2.0 * var_tmp);
        let assign28590_e34338: f64 = (assign28590_e34336 - var_erfcpos);
        (assign28590_e34338, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign28590_e34340;
        var_erfctimesexpmtat_dn5 = assign28590_e34340_d_n5;
        var_erfctimesexpmtat_dn6 = assign28590_e34340_d_n6;
        var_erfctimesexpmtat_dn7 = assign28590_e34340_d_n7;
        var_erfctimesexpmtat_dn8 = assign28590_e34340_d_n8;

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
        *var_guard541_slot = var_guard541;
        *var_guard542_slot = var_guard542;
        *var_guard543_slot = var_guard543;
        *var_guard544_slot = var_guard544;
        *var_guard545_slot = var_guard545;
        *var_guard546_slot = var_guard546;
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

    pub(super) fn stamp_transient_block_56(
        p: &Parameters,
        var_alphaav: f64,
        var_atatbot_d: f64,
        var_cbbtbotd_i: f64,
        var_csrhstid_i: f64,
        var_ctatbotd_i: f64,
        var_ctatstid_i: f64,
        var_erfctimesexpmtat: f64,
        var_erfctimesexpmtat_dn5: f64,
        var_erfctimesexpmtat_dn6: f64,
        var_erfctimesexpmtat_dn7: f64,
        var_erfctimesexpmtat_dn8: f64,
        var_fbbtbot_d: f64,
        var_fstopbot_d: f64,
        var_ftdsti_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard537: f64,
        var_guard541: f64,
        var_idmult: f64,
        var_idsatsti_d: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_lsdrain_i: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_pbrbotd_i: f64,
        var_pstid_i: f64,
        var_slopebot_d: f64,
        var_two_psistar: f64,
        var_v1: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirbotd_i: f64,
        var_vbirbotinv_d: f64,
        var_vbirstiinv_d: f64,
        var_vbisti_d: f64,
        var_vbrbotd_i: f64,
        var_vbrinvbot_d: f64,
        var_vjsrh: f64,
        var_wdepnulrinvbot_d: f64,
        var_wdepnulrsti_d: f64,
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
        var_guard547_slot: &mut f64,
        var_guard548_slot: &mut f64,
        var_guard549_slot: &mut f64,
        var_guard550_slot: &mut f64,
        var_guard551_slot: &mut f64,
        var_guard552_slot: &mut f64,
        var_guard553_slot: &mut f64,
        var_guard554_slot: &mut f64,
        var_guard555_slot: &mut f64,
        var_guard556_slot: &mut f64,
        var_guard557_slot: &mut f64,
        var_guard558_slot: &mut f64,
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
        let mut var_guard547: f64 = *var_guard547_slot;
        let mut var_guard548: f64 = *var_guard548_slot;
        let mut var_guard549: f64 = *var_guard549_slot;
        let mut var_guard550: f64 = *var_guard550_slot;
        let mut var_guard551: f64 = *var_guard551_slot;
        let mut var_guard552: f64 = *var_guard552_slot;
        let mut var_guard553: f64 = *var_guard553_slot;
        let mut var_guard554: f64 = *var_guard554_slot;
        let mut var_guard555: f64 = *var_guard555_slot;
        let mut var_guard556: f64 = *var_guard556_slot;
        let mut var_guard557: f64 = *var_guard557_slot;
        let mut var_guard558: f64 = *var_guard558_slot;
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

        let (assign28600_e34360, assign28600_e34360_d_n5, assign28600_e34360_d_n6, assign28600_e34360_d_n7, assign28600_e34360_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) {
        let assign28600_e34352: f64 = (1.772453850905516 * 0.5);
        let assign28600_e34355: f64 = (var_atatbot_d * var_erfctimesexpmtat);
        let assign28600_e34357: f64 = (assign28600_e34355 / var_ktat);
        let assign28600_e34358: f64 = (assign28600_e34352 * assign28600_e34357);
        (assign28600_e34358, (assign28600_e34352 * ((((var_atatbot_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign28600_e34355 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign28600_e34352 * ((((var_atatbot_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign28600_e34355 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign28600_e34352 * ((((var_atatbot_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign28600_e34355 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign28600_e34352 * ((((var_atatbot_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign28600_e34355 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign28600_e34360;
        var_gammamax_dn5 = assign28600_e34360_d_n5;
        var_gammamax_dn6 = assign28600_e34360_d_n6;
        var_gammamax_dn7 = assign28600_e34360_d_n7;
        var_gammamax_dn8 = assign28600_e34360_d_n8;

        let (assign28610_e34378, assign28610_e34378_d_n5, assign28610_e34378_d_n6, assign28610_e34378_d_n7, assign28610_e34378_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard541 == 0.0)) {
        let assign28610_e34373: f64 = (var_asrh * var_gammamax);
        let assign28610_e34375: f64 = (assign28610_e34373 * var_wtat);
        let assign28610_e34376: f64 = (var_ctatbotd_i * assign28610_e34375);
        (assign28610_e34376, (var_ctatbotd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign28610_e34373 * var_wtat_dn5))), (var_ctatbotd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign28610_e34373 * var_wtat_dn6))), (var_ctatbotd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign28610_e34373 * var_wtat_dn7))), (var_ctatbotd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign28610_e34373 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign28610_e34378;
        var_itat_dn5 = assign28610_e34378_d_n5;
        var_itat_dn6 = assign28610_e34378_d_n6;
        var_itat_dn7 = assign28610_e34378_d_n7;
        var_itat_dn8 = assign28610_e34378_d_n8;

        let assign28620_e34381: f64 = if var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard547 = assign28620_e34381;

        let (assign28630_e34392, assign28630_e34392_d_n5, assign28630_e34392_d_n6, assign28630_e34392_d_n7, assign28630_e34392_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard547 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign28630_e34392;
        var_ibbt_dn5 = assign28630_e34392_d_n5;
        var_ibbt_dn6 = assign28630_e34392_d_n6;
        var_ibbt_dn7 = assign28630_e34392_d_n7;
        var_ibbt_dn8 = assign28630_e34392_d_n8;

        let assign28640_e34395: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard548 = assign28640_e34395;

        let (assign28650_e34414, assign28650_e34414_d_n5, assign28650_e34414_d_n6, assign28650_e34414_d_n7, assign28650_e34414_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard547 == 0.0)) && (var_guard548 != 0.0)) {
        let assign28650_e34409: f64 = (var_vbirbotd_i - var_vbbt);
        let assign28650_e34411: f64 = (assign28650_e34409 * var_vbirbotinv_d);
        let assign28650_e34412: f64 = (assign28650_e34411).sqrt();
        (assign28650_e34412, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28650_e34414;
        var_tmp_dn5 = assign28650_e34414_d_n5;
        var_tmp_dn6 = assign28650_e34414_d_n6;
        var_tmp_dn7 = assign28650_e34414_d_n7;
        var_tmp_dn8 = assign28650_e34414_d_n8;

        let (assign28660_e34435, assign28660_e34435_d_n5, assign28660_e34435_d_n6, assign28660_e34435_d_n7, assign28660_e34435_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard547 == 0.0)) && (var_guard548 == 0.0)) {
        let assign28660_e34429: f64 = (var_vbirbotd_i - var_vbbt);
        let assign28660_e34431: f64 = (assign28660_e34429 * var_vbirbotinv_d);
        let assign28660_e34433: f64 = (assign28660_e34431).powf(var_pbotd_i);
        (assign28660_e34433, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28660_e34435;
        var_tmp_dn5 = assign28660_e34435_d_n5;
        var_tmp_dn6 = assign28660_e34435_d_n6;
        var_tmp_dn7 = assign28660_e34435_d_n7;
        var_tmp_dn8 = assign28660_e34435_d_n8;

        let (assign28670_e34455, assign28670_e34455_d_n5, assign28670_e34455_d_n6, assign28670_e34455_d_n7, assign28670_e34455_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard547 == 0.0)) {
        let assign28670_e34448: f64 = (var_vbirbotd_i - var_vbbt);
        let assign28670_e34450: f64 = (assign28670_e34448 * var_wdepnulrinvbot_d);
        let assign28670_e34452: f64 = (assign28670_e34450 / var_tmp);
        let assign28670_e34453: f64 = (var_one_over_one_minus_pbot_d * assign28670_e34452);
        (assign28670_e34453, (var_one_over_one_minus_pbot_d * (-((assign28670_e34450 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign28670_e34450 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign28670_e34450 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign28670_e34450 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign28670_e34455;
        var_fmaxr_dn5 = assign28670_e34455_d_n5;
        var_fmaxr_dn6 = assign28670_e34455_d_n6;
        var_fmaxr_dn7 = assign28670_e34455_d_n7;
        var_fmaxr_dn8 = assign28670_e34455_d_n8;

        let assign28680_e34457: f64 = (-var_fbbtbot_d);
        let assign28680_e34459: f64 = (assign28680_e34457 / var_fmaxr);
        let assign28680_e34460: f64 = (assign28680_e34459).abs();
        let assign28680_e34462: f64 = if assign28680_e34460 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard549 = assign28680_e34462;

        let (assign28690_e34480, assign28690_e34480_d_n5, assign28690_e34480_d_n6, assign28690_e34480_d_n7, assign28690_e34480_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard547 == 0.0)) && (var_guard549 != 0.0)) {
        let assign28690_e34475: f64 = (-var_fbbtbot_d);
        let assign28690_e34477: f64 = (assign28690_e34475 / var_fmaxr);
        let assign28690_e34478: f64 = (assign28690_e34477).exp();
        (assign28690_e34478, (assign28690_e34478 * (-((assign28690_e34475 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign28690_e34478 * (-((assign28690_e34475 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign28690_e34478 * (-((assign28690_e34475 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign28690_e34478 * (-((assign28690_e34475 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28690_e34480;
        var_tmp_dn5 = assign28690_e34480_d_n5;
        var_tmp_dn6 = assign28690_e34480_d_n6;
        var_tmp_dn7 = assign28690_e34480_d_n7;
        var_tmp_dn8 = assign28690_e34480_d_n8;

        let assign28700_e34482: f64 = (-var_fbbtbot_d);
        let assign28700_e34484: f64 = (assign28700_e34482 / var_fmaxr);
        let assign28700_e34486: f64 = if assign28700_e34484 < 0.0 { 1.0 } else { 0.0 };
        var_guard550 = assign28700_e34486;

        let (assign28710_e34537, assign28710_e34537_d_n5, assign28710_e34537_d_n6, assign28710_e34537_d_n7, assign28710_e34537_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard547 == 0.0)) && (var_guard549 == 0.0)) && (var_guard550 != 0.0)) {
        let assign28710_e34504: f64 = (-230.25850929940458);
        let assign28710_e34506: f64 = (-var_fbbtbot_d);
        let assign28710_e34508: f64 = (assign28710_e34506 / var_fmaxr);
        let assign28710_e34509: f64 = (assign28710_e34504 - assign28710_e34508);
        let assign28710_e34513: f64 = (-230.25850929940458);
        let assign28710_e34515: f64 = (-var_fbbtbot_d);
        let assign28710_e34517: f64 = (assign28710_e34515 / var_fmaxr);
        let assign28710_e34518: f64 = (assign28710_e34513 - assign28710_e34517);
        let assign28710_e34521: f64 = (-230.25850929940458);
        let assign28710_e34523: f64 = (-var_fbbtbot_d);
        let assign28710_e34525: f64 = (assign28710_e34523 / var_fmaxr);
        let assign28710_e34526: f64 = (assign28710_e34521 - assign28710_e34525);
        let assign28710_e34528: f64 = (assign28710_e34526 * 0.3333333333333333);
        let assign28710_e34529: f64 = (1.0 + assign28710_e34528);
        let assign28710_e34530: f64 = (assign28710_e34518 * assign28710_e34529);
        let assign28710_e34531: f64 = (0.5 * assign28710_e34530);
        let assign28710_e34532: f64 = (1.0 + assign28710_e34531);
        let assign28710_e34533: f64 = (assign28710_e34509 * assign28710_e34532);
        let assign28710_e34534: f64 = (1.0 + assign28710_e34533);
        let assign28710_e34535: f64 = (1e-100 / assign28710_e34534);
        (assign28710_e34535, (-((1e-100 * (((-(-((assign28710_e34506 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign28710_e34532) + (assign28710_e34509 * (0.5 * (((-(-((assign28710_e34515 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign28710_e34529) + (assign28710_e34518 * ((-(-((assign28710_e34523 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign28710_e34534 * assign28710_e34534))), (-((1e-100 * (((-(-((assign28710_e34506 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign28710_e34532) + (assign28710_e34509 * (0.5 * (((-(-((assign28710_e34515 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign28710_e34529) + (assign28710_e34518 * ((-(-((assign28710_e34523 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign28710_e34534 * assign28710_e34534))), (-((1e-100 * (((-(-((assign28710_e34506 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign28710_e34532) + (assign28710_e34509 * (0.5 * (((-(-((assign28710_e34515 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign28710_e34529) + (assign28710_e34518 * ((-(-((assign28710_e34523 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign28710_e34534 * assign28710_e34534))), (-((1e-100 * (((-(-((assign28710_e34506 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign28710_e34532) + (assign28710_e34509 * (0.5 * (((-(-((assign28710_e34515 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign28710_e34529) + (assign28710_e34518 * ((-(-((assign28710_e34523 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign28710_e34534 * assign28710_e34534))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28710_e34537;
        var_tmp_dn5 = assign28710_e34537_d_n5;
        var_tmp_dn6 = assign28710_e34537_d_n6;
        var_tmp_dn7 = assign28710_e34537_d_n7;
        var_tmp_dn8 = assign28710_e34537_d_n8;

        let (assign28720_e34586, assign28720_e34586_d_n5, assign28720_e34586_d_n6, assign28720_e34586_d_n7, assign28720_e34586_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard547 == 0.0)) && (var_guard549 == 0.0)) && (var_guard550 == 0.0)) {
        let assign28720_e34556: f64 = (-var_fbbtbot_d);
        let assign28720_e34558: f64 = (assign28720_e34556 / var_fmaxr);
        let assign28720_e34560: f64 = (assign28720_e34558 - 230.25850929940458);
        let assign28720_e34564: f64 = (-var_fbbtbot_d);
        let assign28720_e34566: f64 = (assign28720_e34564 / var_fmaxr);
        let assign28720_e34568: f64 = (assign28720_e34566 - 230.25850929940458);
        let assign28720_e34571: f64 = (-var_fbbtbot_d);
        let assign28720_e34573: f64 = (assign28720_e34571 / var_fmaxr);
        let assign28720_e34575: f64 = (assign28720_e34573 - 230.25850929940458);
        let assign28720_e34577: f64 = (assign28720_e34575 * 0.3333333333333333);
        let assign28720_e34578: f64 = (1.0 + assign28720_e34577);
        let assign28720_e34579: f64 = (assign28720_e34568 * assign28720_e34578);
        let assign28720_e34580: f64 = (0.5 * assign28720_e34579);
        let assign28720_e34581: f64 = (1.0 + assign28720_e34580);
        let assign28720_e34582: f64 = (assign28720_e34560 * assign28720_e34581);
        let assign28720_e34583: f64 = (1.0 + assign28720_e34582);
        let assign28720_e34584: f64 = (1e100 * assign28720_e34583);
        (assign28720_e34584, (1e100 * (((-((assign28720_e34556 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign28720_e34581) + (assign28720_e34560 * (0.5 * (((-((assign28720_e34564 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign28720_e34578) + (assign28720_e34568 * ((-((assign28720_e34571 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign28720_e34556 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign28720_e34581) + (assign28720_e34560 * (0.5 * (((-((assign28720_e34564 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign28720_e34578) + (assign28720_e34568 * ((-((assign28720_e34571 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign28720_e34556 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign28720_e34581) + (assign28720_e34560 * (0.5 * (((-((assign28720_e34564 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign28720_e34578) + (assign28720_e34568 * ((-((assign28720_e34571 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign28720_e34556 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign28720_e34581) + (assign28720_e34560 * (0.5 * (((-((assign28720_e34564 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign28720_e34578) + (assign28720_e34568 * ((-((assign28720_e34571 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28720_e34586;
        var_tmp_dn5 = assign28720_e34586_d_n5;
        var_tmp_dn6 = assign28720_e34586_d_n6;
        var_tmp_dn7 = assign28720_e34586_d_n7;
        var_tmp_dn8 = assign28720_e34586_d_n8;

        let (assign28730_e34606, assign28730_e34606_d_n5, assign28730_e34606_d_n6, assign28730_e34606_d_n7, assign28730_e34606_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard547 == 0.0)) {
        let assign28730_e34599: f64 = (var_v1 * var_fmaxr);
        let assign28730_e34601: f64 = (assign28730_e34599 * var_fmaxr);
        let assign28730_e34603: f64 = (assign28730_e34601 * var_tmp);
        let assign28730_e34604: f64 = (var_cbbtbotd_i * assign28730_e34603);
        (assign28730_e34604, (var_cbbtbotd_i * (((((var_v1 * var_fmaxr_dn5) * var_fmaxr) + (assign28730_e34599 * var_fmaxr_dn5)) * var_tmp) + (assign28730_e34601 * var_tmp_dn5))), (var_cbbtbotd_i * (((((var_v1 * var_fmaxr_dn6) * var_fmaxr) + (assign28730_e34599 * var_fmaxr_dn6)) * var_tmp) + (assign28730_e34601 * var_tmp_dn6))), (var_cbbtbotd_i * (((((var_v1 * var_fmaxr_dn7) * var_fmaxr) + (assign28730_e34599 * var_fmaxr_dn7)) * var_tmp) + (assign28730_e34601 * var_tmp_dn7))), (var_cbbtbotd_i * (((((var_v1 * var_fmaxr_dn8) * var_fmaxr) + (assign28730_e34599 * var_fmaxr_dn8)) * var_tmp) + (assign28730_e34601 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign28730_e34606;
        var_ibbt_dn5 = assign28730_e34606_d_n5;
        var_ibbt_dn6 = assign28730_e34606_d_n6;
        var_ibbt_dn7 = assign28730_e34606_d_n7;
        var_ibbt_dn8 = assign28730_e34606_d_n8;

        let assign28740_e34609: f64 = if var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard551 = assign28740_e34609;

        let (assign28750_e34620, assign28750_e34620_d_n5, assign28750_e34620_d_n6, assign28750_e34620_d_n7, assign28750_e34620_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard551 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign28750_e34620;
        var_fbreakdown_dn5 = assign28750_e34620_d_n5;
        var_fbreakdown_dn6 = assign28750_e34620_d_n6;
        var_fbreakdown_dn7 = assign28750_e34620_d_n7;
        var_fbreakdown_dn8 = assign28750_e34620_d_n8;

        let assign28760_e34623: f64 = (-var_alphaav);
        let assign28760_e34625: f64 = (assign28760_e34623 * var_vbrbotd_i);
        let assign28760_e34626: f64 = if var_vav > assign28760_e34625 { 1.0 } else { 0.0 };
        var_guard552 = assign28760_e34626;

        let assign28770_e34629: f64 = if var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard553 = assign28770_e34629;

        let (assign28780_e34659, assign28780_e34659_d_n5, assign28780_e34659_d_n6, assign28780_e34659_d_n7, assign28780_e34659_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard551 == 0.0)) && (var_guard552 != 0.0)) && (var_guard553 != 0.0)) {
        let assign28780_e34645: f64 = (var_vav * var_vbrinvbot_d);
        let assign28780_e34648: f64 = (var_vav * var_vbrinvbot_d);
        let assign28780_e34649: f64 = (assign28780_e34645 * assign28780_e34648);
        let assign28780_e34652: f64 = (var_vav * var_vbrinvbot_d);
        let assign28780_e34653: f64 = (assign28780_e34649 * assign28780_e34652);
        let assign28780_e34656: f64 = (var_vav * var_vbrinvbot_d);
        let assign28780_e34657: f64 = (assign28780_e34653 * assign28780_e34656);
        (assign28780_e34657, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28780_e34659;
        var_tmp_dn5 = assign28780_e34659_d_n5;
        var_tmp_dn6 = assign28780_e34659_d_n6;
        var_tmp_dn7 = assign28780_e34659_d_n7;
        var_tmp_dn8 = assign28780_e34659_d_n8;

        let (assign28790_e34681, assign28790_e34681_d_n5, assign28790_e34681_d_n6, assign28790_e34681_d_n7, assign28790_e34681_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard551 == 0.0)) && (var_guard552 != 0.0)) && (var_guard553 == 0.0)) {
        let assign28790_e34676: f64 = (var_vav * var_vbrinvbot_d);
        let assign28790_e34677: f64 = (assign28790_e34676).abs();
        let assign28790_e34679: f64 = (assign28790_e34677).powf(var_pbrbotd_i);
        (assign28790_e34679, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28790_e34681;
        var_tmp_dn5 = assign28790_e34681_d_n5;
        var_tmp_dn6 = assign28790_e34681_d_n6;
        var_tmp_dn7 = assign28790_e34681_d_n7;
        var_tmp_dn8 = assign28790_e34681_d_n8;

        let (assign28800_e34699, assign28800_e34699_d_n5, assign28800_e34699_d_n6, assign28800_e34699_d_n7, assign28800_e34699_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard551 == 0.0)) && (var_guard552 != 0.0)) {
        let assign28800_e34696: f64 = (1.0 - var_tmp);
        let assign28800_e34697: f64 = (1.0 / assign28800_e34696);
        (assign28800_e34697, (-((-var_tmp_dn5) / (assign28800_e34696 * assign28800_e34696))), (-((-var_tmp_dn6) / (assign28800_e34696 * assign28800_e34696))), (-((-var_tmp_dn7) / (assign28800_e34696 * assign28800_e34696))), (-((-var_tmp_dn8) / (assign28800_e34696 * assign28800_e34696))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign28800_e34699;
        var_fbreakdown_dn5 = assign28800_e34699_d_n5;
        var_fbreakdown_dn6 = assign28800_e34699_d_n6;
        var_fbreakdown_dn7 = assign28800_e34699_d_n7;
        var_fbreakdown_dn8 = assign28800_e34699_d_n8;

        let (assign28810_e34722, assign28810_e34722_d_n5, assign28810_e34722_d_n6, assign28810_e34722_d_n7, assign28810_e34722_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) && (var_guard551 == 0.0)) && (var_guard552 == 0.0)) {
        let assign28810_e34716: f64 = (var_alphaav * var_vbrbotd_i);
        let assign28810_e34717: f64 = (var_vav + assign28810_e34716);
        let assign28810_e34719: f64 = (assign28810_e34717 * var_slopebot_d);
        let assign28810_e34720: f64 = (var_fstopbot_d + assign28810_e34719);
        (assign28810_e34720, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign28810_e34722;
        var_fbreakdown_dn5 = assign28810_e34722_d_n5;
        var_fbreakdown_dn6 = assign28810_e34722_d_n6;
        var_fbreakdown_dn7 = assign28810_e34722_d_n7;
        var_fbreakdown_dn8 = assign28810_e34722_d_n8;

        let (assign28820_e34741, assign28820_e34741_d_n5, assign28820_e34741_d_n6, assign28820_e34741_d_n7, assign28820_e34741_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard537 == 0.0)) {
        let assign28820_e34732: f64 = (var_id__blk213 + var_isrh);
        let assign28820_e34734: f64 = (assign28820_e34732 + var_itat);
        let assign28820_e34736: f64 = (assign28820_e34734 + var_ibbt);
        let assign28820_e34737: f64 = (p.p29 * assign28820_e34736);
        let assign28820_e34739: f64 = (assign28820_e34737 * var_fbreakdown);
        (assign28820_e34739, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign28820_e34737 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign28820_e34737 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign28820_e34737 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign28820_e34737 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign28820_e34741;
        var_ijunbot_dn5 = assign28820_e34741_d_n5;
        var_ijunbot_dn6 = assign28820_e34741_d_n6;
        var_ijunbot_dn7 = assign28820_e34741_d_n7;
        var_ijunbot_dn8 = assign28820_e34741_d_n8;

        let assign28830_e34744: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard554 = assign28830_e34744;

        let (assign28840_e34752, assign28840_e34752_d_n5, assign28840_e34752_d_n6, assign28840_e34752_d_n7, assign28840_e34752_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign28840_e34752;
        var_ijunsti_dn5 = assign28840_e34752_d_n5;
        var_ijunsti_dn6 = assign28840_e34752_d_n6;
        var_ijunsti_dn7 = assign28840_e34752_d_n7;
        var_ijunsti_dn8 = assign28840_e34752_d_n8;

        let (assign28850_e34763,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) {
        let assign28850_e34761: f64 = (var_idsatsti_d * var_idmult);
        (assign28850_e34761,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign28850_e34763;

        let assign28860_e34770: f64 = if ((var_csrhstid_i == 0.0) && (var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard555 = assign28860_e34770;

        let (assign28870_e34781, assign28870_e34781_d_n5, assign28870_e34781_d_n6, assign28870_e34781_d_n7, assign28870_e34781_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard555 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign28870_e34781;
        var_isrh_dn5 = assign28870_e34781_d_n5;
        var_isrh_dn6 = assign28870_e34781_d_n6;
        var_isrh_dn7 = assign28870_e34781_d_n7;
        var_isrh_dn8 = assign28870_e34781_d_n8;

        let (assign28880_e34795,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard555 == 0.0)) {
        let assign28880_e34793: f64 = (var_vbisti_d - var_vjsrh);
        (assign28880_e34793,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign28880_e34795;

        let (assign28890_e34814,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard555 == 0.0)) {
        let assign28890_e34809: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign28890_e34810: f64 = (1.0 - assign28890_e34809);
        let assign28890_e34811: f64 = (assign28890_e34810).sqrt();
        let assign28890_e34812: f64 = (1.0 - assign28890_e34811);
        (assign28890_e34812,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign28890_e34814;

        let assign28900_e34817: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard556 = assign28900_e34817;

        let (assign28910_e34831,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard555 == 0.0)) && (var_guard556 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign28910_e34831;

        let (assign28920_e34863,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard555 == 0.0)) && (var_guard556 == 0.0)) {
        let assign28920_e34846: f64 = (var_wsrhstep * var_wsrhstep);
        let assign28920_e34848: f64 = (var_wsrhstep).ln();
        let assign28920_e34849: f64 = (assign28920_e34846 * assign28920_e34848);
        let assign28920_e34852: f64 = (1.0 - var_wsrhstep);
        let assign28920_e34853: f64 = (assign28920_e34849 / assign28920_e34852);
        let assign28920_e34855: f64 = (assign28920_e34853 + var_wsrhstep);
        let assign28920_e34859: f64 = (2.0 * var_pstid_i);
        let assign28920_e34860: f64 = (1.0 - assign28920_e34859);
        let assign28920_e34861: f64 = (assign28920_e34855 * assign28920_e34860);
        (assign28920_e34861,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign28920_e34863;

        let (assign28930_e34877,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard555 == 0.0)) {
        let assign28930_e34875: f64 = (var_wsrhstep + var_dwsrh);
        (assign28930_e34875,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign28930_e34877;

        let assign28940_e34880: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard557 = assign28940_e34880;

        let (assign28950_e34897, assign28950_e34897_d_n5, assign28950_e34897_d_n6, assign28950_e34897_d_n7, assign28950_e34897_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard555 == 0.0)) && (var_guard557 != 0.0)) {
        let assign28950_e34894: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign28950_e34895: f64 = (assign28950_e34894).sqrt();
        (assign28950_e34895, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28950_e34897;
        var_tmp_dn5 = assign28950_e34897_d_n5;
        var_tmp_dn6 = assign28950_e34897_d_n6;
        var_tmp_dn7 = assign28950_e34897_d_n7;
        var_tmp_dn8 = assign28950_e34897_d_n8;

        let (assign28960_e34916, assign28960_e34916_d_n5, assign28960_e34916_d_n6, assign28960_e34916_d_n7, assign28960_e34916_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard555 == 0.0)) && (var_guard557 == 0.0)) {
        let assign28960_e34912: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign28960_e34914: f64 = (assign28960_e34912).powf(var_pstid_i);
        (assign28960_e34914, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28960_e34916;
        var_tmp_dn5 = assign28960_e34916_d_n5;
        var_tmp_dn6 = assign28960_e34916_d_n6;
        var_tmp_dn7 = assign28960_e34916_d_n7;
        var_tmp_dn8 = assign28960_e34916_d_n8;

        let (assign28970_e34930, assign28970_e34930_d_n5, assign28970_e34930_d_n6, assign28970_e34930_d_n7, assign28970_e34930_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard555 == 0.0)) {
        let assign28970_e34928: f64 = (var_wdepnulrsti_d * var_tmp);
        (assign28970_e34928, (var_wdepnulrsti_d * var_tmp_dn5), (var_wdepnulrsti_d * var_tmp_dn6), (var_wdepnulrsti_d * var_tmp_dn7), (var_wdepnulrsti_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign28970_e34930;
        var_wdep_dn5 = assign28970_e34930_d_n5;
        var_wdep_dn6 = assign28970_e34930_d_n6;
        var_wdep_dn7 = assign28970_e34930_d_n7;
        var_wdep_dn8 = assign28970_e34930_d_n8;

        let (assign28980_e34948, assign28980_e34948_d_n5, assign28980_e34948_d_n6, assign28980_e34948_d_n7, assign28980_e34948_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard555 == 0.0)) {
        let assign28980_e34943: f64 = (var_zinv - 1.0);
        let assign28980_e34945: f64 = (assign28980_e34943 * var_wdep);
        let assign28980_e34946: f64 = (var_ftdsti_d * assign28980_e34945);
        (assign28980_e34946, (var_ftdsti_d * (assign28980_e34943 * var_wdep_dn5)), (var_ftdsti_d * (assign28980_e34943 * var_wdep_dn6)), (var_ftdsti_d * (assign28980_e34943 * var_wdep_dn7)), (var_ftdsti_d * (assign28980_e34943 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign28980_e34948;
        var_asrh_dn5 = assign28980_e34948_d_n5;
        var_asrh_dn6 = assign28980_e34948_d_n6;
        var_asrh_dn7 = assign28980_e34948_d_n7;
        var_asrh_dn8 = assign28980_e34948_d_n8;

        let (assign28990_e34964, assign28990_e34964_d_n5, assign28990_e34964_d_n6, assign28990_e34964_d_n7, assign28990_e34964_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard555 == 0.0)) {
        let assign28990_e34961: f64 = (var_asrh * var_wsrh);
        let assign28990_e34962: f64 = (var_csrhstid_i * assign28990_e34961);
        (assign28990_e34962, (var_csrhstid_i * (var_asrh_dn5 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn6 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn7 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign28990_e34964;
        var_isrh_dn5 = assign28990_e34964_d_n5;
        var_isrh_dn6 = assign28990_e34964_d_n6;
        var_isrh_dn7 = assign28990_e34964_d_n7;
        var_isrh_dn8 = assign28990_e34964_d_n8;

        let assign29000_e34967: f64 = if var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard558 = assign29000_e34967;

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
        *var_guard547_slot = var_guard547;
        *var_guard548_slot = var_guard548;
        *var_guard549_slot = var_guard549;
        *var_guard550_slot = var_guard550;
        *var_guard551_slot = var_guard551;
        *var_guard552_slot = var_guard552;
        *var_guard553_slot = var_guard553;
        *var_guard554_slot = var_guard554;
        *var_guard555_slot = var_guard555;
        *var_guard556_slot = var_guard556;
        *var_guard557_slot = var_guard557;
        *var_guard558_slot = var_guard558;
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

    pub(super) fn stamp_transient_block_57(
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatsti_d: f64,
        var_berfc: f64,
        var_btatpartsti_d: f64,
        var_cbbtstid_i: f64,
        var_cerfc: f64,
        var_ctatstid_i: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard554: f64,
        var_guard558: f64,
        var_one_minus_psti_d: f64,
        var_one_over_one_minus_psti_d: f64,
        var_perfc: f64,
        var_pstid_i: f64,
        var_vbbt: f64,
        var_vbi_minus_vjsrh: f64,
        var_vbirstid_i: f64,
        var_vbirstiinv_d: f64,
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
        var_guard559_slot: &mut f64,
        var_guard560_slot: &mut f64,
        var_guard561_slot: &mut f64,
        var_guard562_slot: &mut f64,
        var_guard563_slot: &mut f64,
        var_guard564_slot: &mut f64,
        var_guard565_slot: &mut f64,
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
        let mut var_guard559: f64 = *var_guard559_slot;
        let mut var_guard560: f64 = *var_guard560_slot;
        let mut var_guard561: f64 = *var_guard561_slot;
        let mut var_guard562: f64 = *var_guard562_slot;
        let mut var_guard563: f64 = *var_guard563_slot;
        let mut var_guard564: f64 = *var_guard564_slot;
        let mut var_guard565: f64 = *var_guard565_slot;
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

        let (assign29010_e34978, assign29010_e34978_d_n5, assign29010_e34978_d_n6, assign29010_e34978_d_n7, assign29010_e34978_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign29010_e34978;
        var_itat_dn5 = assign29010_e34978_d_n5;
        var_itat_dn6 = assign29010_e34978_d_n6;
        var_itat_dn7 = assign29010_e34978_d_n7;
        var_itat_dn8 = assign29010_e34978_d_n8;

        let (assign29020_e34996, assign29020_e34996_d_n5, assign29020_e34996_d_n6, assign29020_e34996_d_n7, assign29020_e34996_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) {
        let assign29020_e34991: f64 = (var_wdep * var_one_minus_psti_d);
        let assign29020_e34993: f64 = (assign29020_e34991 / var_vbi_minus_vjsrh);
        let assign29020_e34994: f64 = (var_btatpartsti_d * assign29020_e34993);
        (assign29020_e34994, (var_btatpartsti_d * ((var_wdep_dn5 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn6 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn7 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn8 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign29020_e34996;
        var_btat_dn5 = assign29020_e34996_d_n5;
        var_btat_dn6 = assign29020_e34996_d_n6;
        var_btat_dn7 = assign29020_e34996_d_n7;
        var_btat_dn8 = assign29020_e34996_d_n8;

        let (assign29030_e35012, assign29030_e35012_d_n5, assign29030_e35012_d_n6, assign29030_e35012_d_n7, assign29030_e35012_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) {
        let assign29030_e35008: f64 = (0.666666666666667 * var_atatsti_d);
        let assign29030_e35010: f64 = (assign29030_e35008 / var_btat);
        (assign29030_e35010, (-((assign29030_e35008 * var_btat_dn5) / (var_btat * var_btat))), (-((assign29030_e35008 * var_btat_dn6) / (var_btat * var_btat))), (-((assign29030_e35008 * var_btat_dn7) / (var_btat * var_btat))), (-((assign29030_e35008 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign29030_e35012;
        var_twoatatoverthreebtat_dn5 = assign29030_e35012_d_n5;
        var_twoatatoverthreebtat_dn6 = assign29030_e35012_d_n6;
        var_twoatatoverthreebtat_dn7 = assign29030_e35012_d_n7;
        var_twoatatoverthreebtat_dn8 = assign29030_e35012_d_n8;

        let (assign29040_e35026, assign29040_e35026_d_n5, assign29040_e35026_d_n6, assign29040_e35026_d_n7, assign29040_e35026_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) {
        let assign29040_e35024: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign29040_e35024, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign29040_e35026;
        var_umaxbeforelimiting_dn5 = assign29040_e35026_d_n5;
        var_umaxbeforelimiting_dn6 = assign29040_e35026_d_n6;
        var_umaxbeforelimiting_dn7 = assign29040_e35026_d_n7;
        var_umaxbeforelimiting_dn8 = assign29040_e35026_d_n8;

        let (assign29050_e35047, assign29050_e35047_d_n5, assign29050_e35047_d_n6, assign29050_e35047_d_n7, assign29050_e35047_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) {
        let assign29050_e35038: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign29050_e35041: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign29050_e35043: f64 = (assign29050_e35041 + 1.0);
        let assign29050_e35044: f64 = (assign29050_e35038 / assign29050_e35043);
        let assign29050_e35045: f64 = (assign29050_e35044).sqrt();
        (assign29050_e35045, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign29050_e35043) - (assign29050_e35038 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign29050_e35043 * assign29050_e35043)) / (2.0 * assign29050_e35045)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign29050_e35043) - (assign29050_e35038 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign29050_e35043 * assign29050_e35043)) / (2.0 * assign29050_e35045)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign29050_e35043) - (assign29050_e35038 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign29050_e35043 * assign29050_e35043)) / (2.0 * assign29050_e35045)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign29050_e35043) - (assign29050_e35038 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign29050_e35043 * assign29050_e35043)) / (2.0 * assign29050_e35045)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign29050_e35047;
        var_umax_dn5 = assign29050_e35047_d_n5;
        var_umax_dn6 = assign29050_e35047_d_n6;
        var_umax_dn7 = assign29050_e35047_d_n7;
        var_umax_dn8 = assign29050_e35047_d_n8;

        let (assign29060_e35060, assign29060_e35060_d_n5, assign29060_e35060_d_n6, assign29060_e35060_d_n7, assign29060_e35060_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) {
        let assign29060_e35058: f64 = (var_umax).sqrt();
        (assign29060_e35058, (var_umax_dn5 / (2.0 * assign29060_e35058)), (var_umax_dn6 / (2.0 * assign29060_e35058)), (var_umax_dn7 / (2.0 * assign29060_e35058)), (var_umax_dn8 / (2.0 * assign29060_e35058)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign29060_e35060;
        var_sqrtumax_dn5 = assign29060_e35060_d_n5;
        var_sqrtumax_dn6 = assign29060_e35060_d_n6;
        var_sqrtumax_dn7 = assign29060_e35060_d_n7;
        var_sqrtumax_dn8 = assign29060_e35060_d_n8;

        let (assign29070_e35074, assign29070_e35074_d_n5, assign29070_e35074_d_n6, assign29070_e35074_d_n7, assign29070_e35074_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) {
        let assign29070_e35072: f64 = (var_umax * var_sqrtumax);
        (assign29070_e35072, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign29070_e35074;
        var_umaxpoweronepointfive_dn5 = assign29070_e35074_d_n5;
        var_umaxpoweronepointfive_dn6 = assign29070_e35074_d_n6;
        var_umaxpoweronepointfive_dn7 = assign29070_e35074_d_n7;
        var_umaxpoweronepointfive_dn8 = assign29070_e35074_d_n8;

        let assign29080_e35076: f64 = (-var_pstid_i);
        let assign29080_e35078: f64 = (assign29080_e35076 * var_one_over_one_minus_psti_d);
        let assign29080_e35080: f64 = (-1.0);
        let assign29080_e35081: f64 = if assign29080_e35078 == assign29080_e35080 { 1.0 } else { 0.0 };
        var_guard559 = assign29080_e35081;

        let (assign29090_e35101, assign29090_e35101_d_n5, assign29090_e35101_d_n6, assign29090_e35101_d_n7, assign29090_e35101_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) && (var_guard559 != 0.0)) {
        let assign29090_e35097: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign29090_e35098: f64 = (1.0 + assign29090_e35097);
        let assign29090_e35099: f64 = (1.0 / assign29090_e35098);
        (assign29090_e35099, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign29090_e35098 * assign29090_e35098))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign29090_e35098 * assign29090_e35098))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign29090_e35098 * assign29090_e35098))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign29090_e35098 * assign29090_e35098))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign29090_e35101;
        var_wgamma_dn5 = assign29090_e35101_d_n5;
        var_wgamma_dn6 = assign29090_e35101_d_n6;
        var_wgamma_dn7 = assign29090_e35101_d_n7;
        var_wgamma_dn8 = assign29090_e35101_d_n8;

        let (assign29100_e35125, assign29100_e35125_d_n5, assign29100_e35125_d_n6, assign29100_e35125_d_n7, assign29100_e35125_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) && (var_guard559 == 0.0)) {
        let assign29100_e35117: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign29100_e35118: f64 = (1.0 + assign29100_e35117);
        let assign29100_e35120: f64 = (-var_pstid_i);
        let assign29100_e35122: f64 = (assign29100_e35120 * var_one_over_one_minus_psti_d);
        let assign29100_e35123: f64 = (assign29100_e35118).powf(assign29100_e35122);
        (assign29100_e35123, if 0.0 == 0.0 && ((assign29100_e35122) as f64).is_finite() && ((assign29100_e35122) as f64).fract() == 0.0 { if assign29100_e35122 == 0.0 { 0.0 } else { (assign29100_e35122 * ((assign29100_e35118).powf(assign29100_e35122 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign29100_e35123 * (assign29100_e35122 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign29100_e35118))) }, if 0.0 == 0.0 && ((assign29100_e35122) as f64).is_finite() && ((assign29100_e35122) as f64).fract() == 0.0 { if assign29100_e35122 == 0.0 { 0.0 } else { (assign29100_e35122 * ((assign29100_e35118).powf(assign29100_e35122 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign29100_e35123 * (assign29100_e35122 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign29100_e35118))) }, if 0.0 == 0.0 && ((assign29100_e35122) as f64).is_finite() && ((assign29100_e35122) as f64).fract() == 0.0 { if assign29100_e35122 == 0.0 { 0.0 } else { (assign29100_e35122 * ((assign29100_e35118).powf(assign29100_e35122 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign29100_e35123 * (assign29100_e35122 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign29100_e35118))) }, if 0.0 == 0.0 && ((assign29100_e35122) as f64).is_finite() && ((assign29100_e35122) as f64).fract() == 0.0 { if assign29100_e35122 == 0.0 { 0.0 } else { (assign29100_e35122 * ((assign29100_e35118).powf(assign29100_e35122 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign29100_e35123 * (assign29100_e35122 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign29100_e35118))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign29100_e35125;
        var_wgamma_dn5 = assign29100_e35125_d_n5;
        var_wgamma_dn6 = assign29100_e35125_d_n6;
        var_wgamma_dn7 = assign29100_e35125_d_n7;
        var_wgamma_dn8 = assign29100_e35125_d_n8;

        let (assign29110_e35143, assign29110_e35143_d_n5, assign29110_e35143_d_n6, assign29110_e35143_d_n7, assign29110_e35143_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) {
        let assign29110_e35137: f64 = (var_wsrh * var_wgamma);
        let assign29110_e35140: f64 = (var_wsrh + var_wgamma);
        let assign29110_e35141: f64 = (assign29110_e35137 / assign29110_e35140);
        (assign29110_e35141, ((((var_wsrh * var_wgamma_dn5) * assign29110_e35140) - (assign29110_e35137 * var_wgamma_dn5)) / (assign29110_e35140 * assign29110_e35140)), ((((var_wsrh * var_wgamma_dn6) * assign29110_e35140) - (assign29110_e35137 * var_wgamma_dn6)) / (assign29110_e35140 * assign29110_e35140)), ((((var_wsrh * var_wgamma_dn7) * assign29110_e35140) - (assign29110_e35137 * var_wgamma_dn7)) / (assign29110_e35140 * assign29110_e35140)), ((((var_wsrh * var_wgamma_dn8) * assign29110_e35140) - (assign29110_e35137 * var_wgamma_dn8)) / (assign29110_e35140 * assign29110_e35140)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign29110_e35143;
        var_wtat_dn5 = assign29110_e35143_d_n5;
        var_wtat_dn6 = assign29110_e35143_d_n6;
        var_wtat_dn7 = assign29110_e35143_d_n7;
        var_wtat_dn8 = assign29110_e35143_d_n8;

        let (assign29120_e35160, assign29120_e35160_d_n5, assign29120_e35160_d_n6, assign29120_e35160_d_n7, assign29120_e35160_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) {
        let assign29120_e35156: f64 = (var_btat / var_sqrtumax);
        let assign29120_e35157: f64 = (0.375 * assign29120_e35156);
        let assign29120_e35158: f64 = (assign29120_e35157).sqrt();
        (assign29120_e35158, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign29120_e35158)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign29120_e35158)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign29120_e35158)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign29120_e35158)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign29120_e35160;
        var_ktat_dn5 = assign29120_e35160_d_n5;
        var_ktat_dn6 = assign29120_e35160_d_n6;
        var_ktat_dn7 = assign29120_e35160_d_n7;
        var_ktat_dn8 = assign29120_e35160_d_n8;

        let (assign29130_e35178, assign29130_e35178_d_n5, assign29130_e35178_d_n6, assign29130_e35178_d_n7, assign29130_e35178_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) {
        let assign29130_e35173: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign29130_e35174: f64 = (2.0 * assign29130_e35173);
        let assign29130_e35176: f64 = (assign29130_e35174 - var_umax);
        (assign29130_e35176, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign29130_e35178;
        var_ltat_dn5 = assign29130_e35178_d_n5;
        var_ltat_dn6 = assign29130_e35178_d_n6;
        var_ltat_dn7 = assign29130_e35178_d_n7;
        var_ltat_dn8 = assign29130_e35178_d_n8;

        let (assign29140_e35204, assign29140_e35204_d_n5, assign29140_e35204_d_n6, assign29140_e35204_d_n7, assign29140_e35204_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) {
        let assign29140_e35190: f64 = (var_atatsti_d * var_twoatatoverthreebtat);
        let assign29140_e35192: f64 = (assign29140_e35190 * var_sqrtumax);
        let assign29140_e35195: f64 = (var_atatsti_d * var_umax);
        let assign29140_e35196: f64 = (assign29140_e35192 - assign29140_e35195);
        let assign29140_e35200: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign29140_e35201: f64 = (0.5 * assign29140_e35200);
        let assign29140_e35202: f64 = (assign29140_e35196 + assign29140_e35201);
        (assign29140_e35202, (((((var_atatsti_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign29140_e35190 * var_sqrtumax_dn5)) - (var_atatsti_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign29140_e35190 * var_sqrtumax_dn6)) - (var_atatsti_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign29140_e35190 * var_sqrtumax_dn7)) - (var_atatsti_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign29140_e35190 * var_sqrtumax_dn8)) - (var_atatsti_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign29140_e35204;
        var_mtat_dn5 = assign29140_e35204_d_n5;
        var_mtat_dn6 = assign29140_e35204_d_n6;
        var_mtat_dn7 = assign29140_e35204_d_n7;
        var_mtat_dn8 = assign29140_e35204_d_n8;

        let (assign29150_e35220, assign29150_e35220_d_n5, assign29150_e35220_d_n6, assign29150_e35220_d_n7, assign29150_e35220_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) {
        let assign29150_e35216: f64 = (var_ltat - 1.0);
        let assign29150_e35218: f64 = (assign29150_e35216 * var_ktat);
        (assign29150_e35218, ((var_ltat_dn5 * var_ktat) + (assign29150_e35216 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign29150_e35216 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign29150_e35216 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign29150_e35216 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign29150_e35220;
        var_xerfc_dn5 = assign29150_e35220_d_n5;
        var_xerfc_dn6 = assign29150_e35220_d_n6;
        var_xerfc_dn7 = assign29150_e35220_d_n7;
        var_xerfc_dn8 = assign29150_e35220_d_n8;

        let (assign29160_e35234, assign29160_e35234_d_n5, assign29160_e35234_d_n6, assign29160_e35234_d_n7, assign29160_e35234_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) {
        let assign29160_e35232: f64 = (var_xerfc * var_xerfc);
        (assign29160_e35232, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign29160_e35234;
        var_ysq_dn5 = assign29160_e35234_d_n5;
        var_ysq_dn6 = assign29160_e35234_d_n6;
        var_ysq_dn7 = assign29160_e35234_d_n7;
        var_ysq_dn8 = assign29160_e35234_d_n8;

        let assign29170_e35237: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard560 = assign29170_e35237;

        let (assign29180_e35257, assign29180_e35257_d_n5, assign29180_e35257_d_n6, assign29180_e35257_d_n7, assign29180_e35257_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) && (var_guard560 != 0.0)) {
        let assign29180_e35253: f64 = (var_perfc * var_xerfc);
        let assign29180_e35254: f64 = (1.0 + assign29180_e35253);
        let assign29180_e35255: f64 = (1.0 / assign29180_e35254);
        (assign29180_e35255, (-((var_perfc * var_xerfc_dn5) / (assign29180_e35254 * assign29180_e35254))), (-((var_perfc * var_xerfc_dn6) / (assign29180_e35254 * assign29180_e35254))), (-((var_perfc * var_xerfc_dn7) / (assign29180_e35254 * assign29180_e35254))), (-((var_perfc * var_xerfc_dn8) / (assign29180_e35254 * assign29180_e35254))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign29180_e35257;
        var_terfc_dn5 = assign29180_e35257_d_n5;
        var_terfc_dn6 = assign29180_e35257_d_n6;
        var_terfc_dn7 = assign29180_e35257_d_n7;
        var_terfc_dn8 = assign29180_e35257_d_n8;

        let (assign29190_e35278, assign29190_e35278_d_n5, assign29190_e35278_d_n6, assign29190_e35278_d_n7, assign29190_e35278_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) && (var_guard560 == 0.0)) {
        let assign29190_e35274: f64 = (var_perfc * var_xerfc);
        let assign29190_e35275: f64 = (1.0 - assign29190_e35274);
        let assign29190_e35276: f64 = (1.0 / assign29190_e35275);
        (assign29190_e35276, (-((-(var_perfc * var_xerfc_dn5)) / (assign29190_e35275 * assign29190_e35275))), (-((-(var_perfc * var_xerfc_dn6)) / (assign29190_e35275 * assign29190_e35275))), (-((-(var_perfc * var_xerfc_dn7)) / (assign29190_e35275 * assign29190_e35275))), (-((-(var_perfc * var_xerfc_dn8)) / (assign29190_e35275 * assign29190_e35275))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign29190_e35278;
        var_terfc_dn5 = assign29190_e35278_d_n5;
        var_terfc_dn6 = assign29190_e35278_d_n6;
        var_terfc_dn7 = assign29190_e35278_d_n7;
        var_terfc_dn8 = assign29190_e35278_d_n8;

        let assign29200_e35280: f64 = (-var_ysq);
        let assign29200_e35282: f64 = (assign29200_e35280 + var_mtat);
        let assign29200_e35284: f64 = (-230.25850929940458);
        let assign29200_e35285: f64 = if assign29200_e35282 > assign29200_e35284 { 1.0 } else { 0.0 };
        var_guard561 = assign29200_e35285;

        let (assign29210_e35303, assign29210_e35303_d_n5, assign29210_e35303_d_n6, assign29210_e35303_d_n7, assign29210_e35303_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) && (var_guard561 != 0.0)) {
        let assign29210_e35298: f64 = (-var_ysq);
        let assign29210_e35300: f64 = (assign29210_e35298 + var_mtat);
        let assign29210_e35301: f64 = (assign29210_e35300).exp();
        (assign29210_e35301, (assign29210_e35301 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign29210_e35301 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign29210_e35301 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign29210_e35301 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29210_e35303;
        var_tmp_dn5 = assign29210_e35303_d_n5;
        var_tmp_dn6 = assign29210_e35303_d_n6;
        var_tmp_dn7 = assign29210_e35303_d_n7;
        var_tmp_dn8 = assign29210_e35303_d_n8;

        let (assign29220_e35352, assign29220_e35352_d_n5, assign29220_e35352_d_n6, assign29220_e35352_d_n7, assign29220_e35352_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) && (var_guard561 == 0.0)) {
        let assign29220_e35319: f64 = (-230.25850929940458);
        let assign29220_e35321: f64 = (-var_ysq);
        let assign29220_e35323: f64 = (assign29220_e35321 + var_mtat);
        let assign29220_e35324: f64 = (assign29220_e35319 - assign29220_e35323);
        let assign29220_e35328: f64 = (-230.25850929940458);
        let assign29220_e35330: f64 = (-var_ysq);
        let assign29220_e35332: f64 = (assign29220_e35330 + var_mtat);
        let assign29220_e35333: f64 = (assign29220_e35328 - assign29220_e35332);
        let assign29220_e35336: f64 = (-230.25850929940458);
        let assign29220_e35338: f64 = (-var_ysq);
        let assign29220_e35340: f64 = (assign29220_e35338 + var_mtat);
        let assign29220_e35341: f64 = (assign29220_e35336 - assign29220_e35340);
        let assign29220_e35343: f64 = (assign29220_e35341 * 0.3333333333333333);
        let assign29220_e35344: f64 = (1.0 + assign29220_e35343);
        let assign29220_e35345: f64 = (assign29220_e35333 * assign29220_e35344);
        let assign29220_e35346: f64 = (0.5 * assign29220_e35345);
        let assign29220_e35347: f64 = (1.0 + assign29220_e35346);
        let assign29220_e35348: f64 = (assign29220_e35324 * assign29220_e35347);
        let assign29220_e35349: f64 = (1.0 + assign29220_e35348);
        let assign29220_e35350: f64 = (1e-100 / assign29220_e35349);
        (assign29220_e35350, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign29220_e35347) + (assign29220_e35324 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign29220_e35344) + (assign29220_e35333 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign29220_e35349 * assign29220_e35349))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign29220_e35347) + (assign29220_e35324 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign29220_e35344) + (assign29220_e35333 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign29220_e35349 * assign29220_e35349))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign29220_e35347) + (assign29220_e35324 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign29220_e35344) + (assign29220_e35333 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign29220_e35349 * assign29220_e35349))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign29220_e35347) + (assign29220_e35324 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign29220_e35344) + (assign29220_e35333 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign29220_e35349 * assign29220_e35349))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29220_e35352;
        var_tmp_dn5 = assign29220_e35352_d_n5;
        var_tmp_dn6 = assign29220_e35352_d_n6;
        var_tmp_dn7 = assign29220_e35352_d_n7;
        var_tmp_dn8 = assign29220_e35352_d_n8;

        let (assign29230_e35382, assign29230_e35382_d_n5, assign29230_e35382_d_n6, assign29230_e35382_d_n7, assign29230_e35382_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) {
        let assign29230_e35364: f64 = (0.29214664 * var_terfc);
        let assign29230_e35368: f64 = (var_terfc * var_terfc);
        let assign29230_e35369: f64 = (var_berfc * assign29230_e35368);
        let assign29230_e35370: f64 = (assign29230_e35364 + assign29230_e35369);
        let assign29230_e35374: f64 = (var_terfc * var_terfc);
        let assign29230_e35376: f64 = (assign29230_e35374 * var_terfc);
        let assign29230_e35377: f64 = (var_cerfc * assign29230_e35376);
        let assign29230_e35378: f64 = (assign29230_e35370 + assign29230_e35377);
        let assign29230_e35380: f64 = (assign29230_e35378 * var_tmp);
        (assign29230_e35380, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign29230_e35374 * var_terfc_dn5)))) * var_tmp) + (assign29230_e35378 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign29230_e35374 * var_terfc_dn6)))) * var_tmp) + (assign29230_e35378 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign29230_e35374 * var_terfc_dn7)))) * var_tmp) + (assign29230_e35378 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign29230_e35374 * var_terfc_dn8)))) * var_tmp) + (assign29230_e35378 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign29230_e35382;
        var_erfcpos_dn5 = assign29230_e35382_d_n5;
        var_erfcpos_dn6 = assign29230_e35382_d_n6;
        var_erfcpos_dn7 = assign29230_e35382_d_n7;
        var_erfcpos_dn8 = assign29230_e35382_d_n8;

        let assign29240_e35385: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard562 = assign29240_e35385;

        let (assign29250_e35399, assign29250_e35399_d_n5, assign29250_e35399_d_n6, assign29250_e35399_d_n7, assign29250_e35399_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) && (var_guard562 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign29250_e35399;
        var_erfctimesexpmtat_dn5 = assign29250_e35399_d_n5;
        var_erfctimesexpmtat_dn6 = assign29250_e35399_d_n6;
        var_erfctimesexpmtat_dn7 = assign29250_e35399_d_n7;
        var_erfctimesexpmtat_dn8 = assign29250_e35399_d_n8;

        let assign29260_e35402: f64 = (-230.25850929940458);
        let assign29260_e35403: f64 = if var_mtat > assign29260_e35402 { 1.0 } else { 0.0 };
        var_guard563 = assign29260_e35403;

        let (assign29270_e35421, assign29270_e35421_d_n5, assign29270_e35421_d_n6, assign29270_e35421_d_n7, assign29270_e35421_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) && (var_guard562 == 0.0)) && (var_guard563 != 0.0)) {
        let assign29270_e35419: f64 = (var_mtat).exp();
        (assign29270_e35419, (assign29270_e35419 * var_mtat_dn5), (assign29270_e35419 * var_mtat_dn6), (assign29270_e35419 * var_mtat_dn7), (assign29270_e35419 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29270_e35421;
        var_tmp_dn5 = assign29270_e35421_d_n5;
        var_tmp_dn6 = assign29270_e35421_d_n6;
        var_tmp_dn7 = assign29270_e35421_d_n7;
        var_tmp_dn8 = assign29270_e35421_d_n8;

        let (assign29280_e35464, assign29280_e35464_d_n5, assign29280_e35464_d_n6, assign29280_e35464_d_n7, assign29280_e35464_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) && (var_guard562 == 0.0)) && (var_guard563 == 0.0)) {
        let assign29280_e35440: f64 = (-230.25850929940458);
        let assign29280_e35442: f64 = (assign29280_e35440 - var_mtat);
        let assign29280_e35446: f64 = (-230.25850929940458);
        let assign29280_e35448: f64 = (assign29280_e35446 - var_mtat);
        let assign29280_e35451: f64 = (-230.25850929940458);
        let assign29280_e35453: f64 = (assign29280_e35451 - var_mtat);
        let assign29280_e35455: f64 = (assign29280_e35453 * 0.3333333333333333);
        let assign29280_e35456: f64 = (1.0 + assign29280_e35455);
        let assign29280_e35457: f64 = (assign29280_e35448 * assign29280_e35456);
        let assign29280_e35458: f64 = (0.5 * assign29280_e35457);
        let assign29280_e35459: f64 = (1.0 + assign29280_e35458);
        let assign29280_e35460: f64 = (assign29280_e35442 * assign29280_e35459);
        let assign29280_e35461: f64 = (1.0 + assign29280_e35460);
        let assign29280_e35462: f64 = (1e-100 / assign29280_e35461);
        (assign29280_e35462, (-((1e-100 * (((-var_mtat_dn5) * assign29280_e35459) + (assign29280_e35442 * (0.5 * (((-var_mtat_dn5) * assign29280_e35456) + (assign29280_e35448 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign29280_e35461 * assign29280_e35461))), (-((1e-100 * (((-var_mtat_dn6) * assign29280_e35459) + (assign29280_e35442 * (0.5 * (((-var_mtat_dn6) * assign29280_e35456) + (assign29280_e35448 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign29280_e35461 * assign29280_e35461))), (-((1e-100 * (((-var_mtat_dn7) * assign29280_e35459) + (assign29280_e35442 * (0.5 * (((-var_mtat_dn7) * assign29280_e35456) + (assign29280_e35448 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign29280_e35461 * assign29280_e35461))), (-((1e-100 * (((-var_mtat_dn8) * assign29280_e35459) + (assign29280_e35442 * (0.5 * (((-var_mtat_dn8) * assign29280_e35456) + (assign29280_e35448 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign29280_e35461 * assign29280_e35461))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29280_e35464;
        var_tmp_dn5 = assign29280_e35464_d_n5;
        var_tmp_dn6 = assign29280_e35464_d_n6;
        var_tmp_dn7 = assign29280_e35464_d_n7;
        var_tmp_dn8 = assign29280_e35464_d_n8;

        let (assign29290_e35483, assign29290_e35483_d_n5, assign29290_e35483_d_n6, assign29290_e35483_d_n7, assign29290_e35483_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) && (var_guard562 == 0.0)) {
        let assign29290_e35479: f64 = (2.0 * var_tmp);
        let assign29290_e35481: f64 = (assign29290_e35479 - var_erfcpos);
        (assign29290_e35481, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign29290_e35483;
        var_erfctimesexpmtat_dn5 = assign29290_e35483_d_n5;
        var_erfctimesexpmtat_dn6 = assign29290_e35483_d_n6;
        var_erfctimesexpmtat_dn7 = assign29290_e35483_d_n7;
        var_erfctimesexpmtat_dn8 = assign29290_e35483_d_n8;

        let (assign29300_e35503, assign29300_e35503_d_n5, assign29300_e35503_d_n6, assign29300_e35503_d_n7, assign29300_e35503_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) {
        let assign29300_e35495: f64 = (1.772453850905516 * 0.5);
        let assign29300_e35498: f64 = (var_atatsti_d * var_erfctimesexpmtat);
        let assign29300_e35500: f64 = (assign29300_e35498 / var_ktat);
        let assign29300_e35501: f64 = (assign29300_e35495 * assign29300_e35500);
        (assign29300_e35501, (assign29300_e35495 * ((((var_atatsti_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign29300_e35498 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign29300_e35495 * ((((var_atatsti_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign29300_e35498 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign29300_e35495 * ((((var_atatsti_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign29300_e35498 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign29300_e35495 * ((((var_atatsti_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign29300_e35498 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign29300_e35503;
        var_gammamax_dn5 = assign29300_e35503_d_n5;
        var_gammamax_dn6 = assign29300_e35503_d_n6;
        var_gammamax_dn7 = assign29300_e35503_d_n7;
        var_gammamax_dn8 = assign29300_e35503_d_n8;

        let (assign29310_e35521, assign29310_e35521_d_n5, assign29310_e35521_d_n6, assign29310_e35521_d_n7, assign29310_e35521_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard558 == 0.0)) {
        let assign29310_e35516: f64 = (var_asrh * var_gammamax);
        let assign29310_e35518: f64 = (assign29310_e35516 * var_wtat);
        let assign29310_e35519: f64 = (var_ctatstid_i * assign29310_e35518);
        (assign29310_e35519, (var_ctatstid_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign29310_e35516 * var_wtat_dn5))), (var_ctatstid_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign29310_e35516 * var_wtat_dn6))), (var_ctatstid_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign29310_e35516 * var_wtat_dn7))), (var_ctatstid_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign29310_e35516 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign29310_e35521;
        var_itat_dn5 = assign29310_e35521_d_n5;
        var_itat_dn6 = assign29310_e35521_d_n6;
        var_itat_dn7 = assign29310_e35521_d_n7;
        var_itat_dn8 = assign29310_e35521_d_n8;

        let assign29320_e35524: f64 = if var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard564 = assign29320_e35524;

        let (assign29330_e35535, assign29330_e35535_d_n5, assign29330_e35535_d_n6, assign29330_e35535_d_n7, assign29330_e35535_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard564 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign29330_e35535;
        var_ibbt_dn5 = assign29330_e35535_d_n5;
        var_ibbt_dn6 = assign29330_e35535_d_n6;
        var_ibbt_dn7 = assign29330_e35535_d_n7;
        var_ibbt_dn8 = assign29330_e35535_d_n8;

        let assign29340_e35538: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard565 = assign29340_e35538;

        let (assign29350_e35557, assign29350_e35557_d_n5, assign29350_e35557_d_n6, assign29350_e35557_d_n7, assign29350_e35557_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard564 == 0.0)) && (var_guard565 != 0.0)) {
        let assign29350_e35552: f64 = (var_vbirstid_i - var_vbbt);
        let assign29350_e35554: f64 = (assign29350_e35552 * var_vbirstiinv_d);
        let assign29350_e35555: f64 = (assign29350_e35554).sqrt();
        (assign29350_e35555, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29350_e35557;
        var_tmp_dn5 = assign29350_e35557_d_n5;
        var_tmp_dn6 = assign29350_e35557_d_n6;
        var_tmp_dn7 = assign29350_e35557_d_n7;
        var_tmp_dn8 = assign29350_e35557_d_n8;

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
        *var_guard559_slot = var_guard559;
        *var_guard560_slot = var_guard560;
        *var_guard561_slot = var_guard561;
        *var_guard562_slot = var_guard562;
        *var_guard563_slot = var_guard563;
        *var_guard564_slot = var_guard564;
        *var_guard565_slot = var_guard565;
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

    pub(super) fn stamp_transient_block_58(
        p: &Parameters,
        var_alphaav: f64,
        var_atatgat_d: f64,
        var_btatpartgat_d: f64,
        var_cbbtstid_i: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_fbbtsti_d: f64,
        var_fstopsti_d: f64,
        var_ftdgat_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard554: f64,
        var_guard564: f64,
        var_guard565: f64,
        var_idmult: f64,
        var_idsatgat_d: f64,
        var_lgdrain_i: f64,
        var_one_minus_pgat_d: f64,
        var_one_over_one_minus_psti_d: f64,
        var_pbrstid_i: f64,
        var_pgatd_i: f64,
        var_pstid_i: f64,
        var_slopesti_d: f64,
        var_two_psistar: f64,
        var_v1: f64,
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
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_guard566_slot: &mut f64,
        var_guard567_slot: &mut f64,
        var_guard568_slot: &mut f64,
        var_guard569_slot: &mut f64,
        var_guard570_slot: &mut f64,
        var_guard571_slot: &mut f64,
        var_guard572_slot: &mut f64,
        var_guard573_slot: &mut f64,
        var_guard574_slot: &mut f64,
        var_guard575_slot: &mut f64,
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
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn5: f64 = *var_fmaxr_dn5_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_guard566: f64 = *var_guard566_slot;
        let mut var_guard567: f64 = *var_guard567_slot;
        let mut var_guard568: f64 = *var_guard568_slot;
        let mut var_guard569: f64 = *var_guard569_slot;
        let mut var_guard570: f64 = *var_guard570_slot;
        let mut var_guard571: f64 = *var_guard571_slot;
        let mut var_guard572: f64 = *var_guard572_slot;
        let mut var_guard573: f64 = *var_guard573_slot;
        let mut var_guard574: f64 = *var_guard574_slot;
        let mut var_guard575: f64 = *var_guard575_slot;
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
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn5: f64 = *var_wdep_dn5_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;

        let (assign29360_e35578, assign29360_e35578_d_n5, assign29360_e35578_d_n6, assign29360_e35578_d_n7, assign29360_e35578_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard564 == 0.0)) && (var_guard565 == 0.0)) {
        let assign29360_e35572: f64 = (var_vbirstid_i - var_vbbt);
        let assign29360_e35574: f64 = (assign29360_e35572 * var_vbirstiinv_d);
        let assign29360_e35576: f64 = (assign29360_e35574).powf(var_pstid_i);
        (assign29360_e35576, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29360_e35578;
        var_tmp_dn5 = assign29360_e35578_d_n5;
        var_tmp_dn6 = assign29360_e35578_d_n6;
        var_tmp_dn7 = assign29360_e35578_d_n7;
        var_tmp_dn8 = assign29360_e35578_d_n8;

        let (assign29370_e35598, assign29370_e35598_d_n5, assign29370_e35598_d_n6, assign29370_e35598_d_n7, assign29370_e35598_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard564 == 0.0)) {
        let assign29370_e35591: f64 = (var_vbirstid_i - var_vbbt);
        let assign29370_e35593: f64 = (assign29370_e35591 * var_wdepnulrinvsti_d);
        let assign29370_e35595: f64 = (assign29370_e35593 / var_tmp);
        let assign29370_e35596: f64 = (var_one_over_one_minus_psti_d * assign29370_e35595);
        (assign29370_e35596, (var_one_over_one_minus_psti_d * (-((assign29370_e35593 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign29370_e35593 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign29370_e35593 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign29370_e35593 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign29370_e35598;
        var_fmaxr_dn5 = assign29370_e35598_d_n5;
        var_fmaxr_dn6 = assign29370_e35598_d_n6;
        var_fmaxr_dn7 = assign29370_e35598_d_n7;
        var_fmaxr_dn8 = assign29370_e35598_d_n8;

        let assign29380_e35600: f64 = (-var_fbbtsti_d);
        let assign29380_e35602: f64 = (assign29380_e35600 / var_fmaxr);
        let assign29380_e35603: f64 = (assign29380_e35602).abs();
        let assign29380_e35605: f64 = if assign29380_e35603 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard566 = assign29380_e35605;

        let (assign29390_e35623, assign29390_e35623_d_n5, assign29390_e35623_d_n6, assign29390_e35623_d_n7, assign29390_e35623_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard564 == 0.0)) && (var_guard566 != 0.0)) {
        let assign29390_e35618: f64 = (-var_fbbtsti_d);
        let assign29390_e35620: f64 = (assign29390_e35618 / var_fmaxr);
        let assign29390_e35621: f64 = (assign29390_e35620).exp();
        (assign29390_e35621, (assign29390_e35621 * (-((assign29390_e35618 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign29390_e35621 * (-((assign29390_e35618 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign29390_e35621 * (-((assign29390_e35618 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign29390_e35621 * (-((assign29390_e35618 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29390_e35623;
        var_tmp_dn5 = assign29390_e35623_d_n5;
        var_tmp_dn6 = assign29390_e35623_d_n6;
        var_tmp_dn7 = assign29390_e35623_d_n7;
        var_tmp_dn8 = assign29390_e35623_d_n8;

        let assign29400_e35625: f64 = (-var_fbbtsti_d);
        let assign29400_e35627: f64 = (assign29400_e35625 / var_fmaxr);
        let assign29400_e35629: f64 = if assign29400_e35627 < 0.0 { 1.0 } else { 0.0 };
        var_guard567 = assign29400_e35629;

        let (assign29410_e35680, assign29410_e35680_d_n5, assign29410_e35680_d_n6, assign29410_e35680_d_n7, assign29410_e35680_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard564 == 0.0)) && (var_guard566 == 0.0)) && (var_guard567 != 0.0)) {
        let assign29410_e35647: f64 = (-230.25850929940458);
        let assign29410_e35649: f64 = (-var_fbbtsti_d);
        let assign29410_e35651: f64 = (assign29410_e35649 / var_fmaxr);
        let assign29410_e35652: f64 = (assign29410_e35647 - assign29410_e35651);
        let assign29410_e35656: f64 = (-230.25850929940458);
        let assign29410_e35658: f64 = (-var_fbbtsti_d);
        let assign29410_e35660: f64 = (assign29410_e35658 / var_fmaxr);
        let assign29410_e35661: f64 = (assign29410_e35656 - assign29410_e35660);
        let assign29410_e35664: f64 = (-230.25850929940458);
        let assign29410_e35666: f64 = (-var_fbbtsti_d);
        let assign29410_e35668: f64 = (assign29410_e35666 / var_fmaxr);
        let assign29410_e35669: f64 = (assign29410_e35664 - assign29410_e35668);
        let assign29410_e35671: f64 = (assign29410_e35669 * 0.3333333333333333);
        let assign29410_e35672: f64 = (1.0 + assign29410_e35671);
        let assign29410_e35673: f64 = (assign29410_e35661 * assign29410_e35672);
        let assign29410_e35674: f64 = (0.5 * assign29410_e35673);
        let assign29410_e35675: f64 = (1.0 + assign29410_e35674);
        let assign29410_e35676: f64 = (assign29410_e35652 * assign29410_e35675);
        let assign29410_e35677: f64 = (1.0 + assign29410_e35676);
        let assign29410_e35678: f64 = (1e-100 / assign29410_e35677);
        (assign29410_e35678, (-((1e-100 * (((-(-((assign29410_e35649 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign29410_e35675) + (assign29410_e35652 * (0.5 * (((-(-((assign29410_e35658 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign29410_e35672) + (assign29410_e35661 * ((-(-((assign29410_e35666 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign29410_e35677 * assign29410_e35677))), (-((1e-100 * (((-(-((assign29410_e35649 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign29410_e35675) + (assign29410_e35652 * (0.5 * (((-(-((assign29410_e35658 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign29410_e35672) + (assign29410_e35661 * ((-(-((assign29410_e35666 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign29410_e35677 * assign29410_e35677))), (-((1e-100 * (((-(-((assign29410_e35649 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign29410_e35675) + (assign29410_e35652 * (0.5 * (((-(-((assign29410_e35658 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign29410_e35672) + (assign29410_e35661 * ((-(-((assign29410_e35666 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign29410_e35677 * assign29410_e35677))), (-((1e-100 * (((-(-((assign29410_e35649 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign29410_e35675) + (assign29410_e35652 * (0.5 * (((-(-((assign29410_e35658 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign29410_e35672) + (assign29410_e35661 * ((-(-((assign29410_e35666 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign29410_e35677 * assign29410_e35677))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29410_e35680;
        var_tmp_dn5 = assign29410_e35680_d_n5;
        var_tmp_dn6 = assign29410_e35680_d_n6;
        var_tmp_dn7 = assign29410_e35680_d_n7;
        var_tmp_dn8 = assign29410_e35680_d_n8;

        let (assign29420_e35729, assign29420_e35729_d_n5, assign29420_e35729_d_n6, assign29420_e35729_d_n7, assign29420_e35729_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard564 == 0.0)) && (var_guard566 == 0.0)) && (var_guard567 == 0.0)) {
        let assign29420_e35699: f64 = (-var_fbbtsti_d);
        let assign29420_e35701: f64 = (assign29420_e35699 / var_fmaxr);
        let assign29420_e35703: f64 = (assign29420_e35701 - 230.25850929940458);
        let assign29420_e35707: f64 = (-var_fbbtsti_d);
        let assign29420_e35709: f64 = (assign29420_e35707 / var_fmaxr);
        let assign29420_e35711: f64 = (assign29420_e35709 - 230.25850929940458);
        let assign29420_e35714: f64 = (-var_fbbtsti_d);
        let assign29420_e35716: f64 = (assign29420_e35714 / var_fmaxr);
        let assign29420_e35718: f64 = (assign29420_e35716 - 230.25850929940458);
        let assign29420_e35720: f64 = (assign29420_e35718 * 0.3333333333333333);
        let assign29420_e35721: f64 = (1.0 + assign29420_e35720);
        let assign29420_e35722: f64 = (assign29420_e35711 * assign29420_e35721);
        let assign29420_e35723: f64 = (0.5 * assign29420_e35722);
        let assign29420_e35724: f64 = (1.0 + assign29420_e35723);
        let assign29420_e35725: f64 = (assign29420_e35703 * assign29420_e35724);
        let assign29420_e35726: f64 = (1.0 + assign29420_e35725);
        let assign29420_e35727: f64 = (1e100 * assign29420_e35726);
        (assign29420_e35727, (1e100 * (((-((assign29420_e35699 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign29420_e35724) + (assign29420_e35703 * (0.5 * (((-((assign29420_e35707 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign29420_e35721) + (assign29420_e35711 * ((-((assign29420_e35714 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign29420_e35699 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign29420_e35724) + (assign29420_e35703 * (0.5 * (((-((assign29420_e35707 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign29420_e35721) + (assign29420_e35711 * ((-((assign29420_e35714 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign29420_e35699 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign29420_e35724) + (assign29420_e35703 * (0.5 * (((-((assign29420_e35707 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign29420_e35721) + (assign29420_e35711 * ((-((assign29420_e35714 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign29420_e35699 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign29420_e35724) + (assign29420_e35703 * (0.5 * (((-((assign29420_e35707 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign29420_e35721) + (assign29420_e35711 * ((-((assign29420_e35714 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29420_e35729;
        var_tmp_dn5 = assign29420_e35729_d_n5;
        var_tmp_dn6 = assign29420_e35729_d_n6;
        var_tmp_dn7 = assign29420_e35729_d_n7;
        var_tmp_dn8 = assign29420_e35729_d_n8;

        let (assign29430_e35749, assign29430_e35749_d_n5, assign29430_e35749_d_n6, assign29430_e35749_d_n7, assign29430_e35749_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard564 == 0.0)) {
        let assign29430_e35742: f64 = (var_v1 * var_fmaxr);
        let assign29430_e35744: f64 = (assign29430_e35742 * var_fmaxr);
        let assign29430_e35746: f64 = (assign29430_e35744 * var_tmp);
        let assign29430_e35747: f64 = (var_cbbtstid_i * assign29430_e35746);
        (assign29430_e35747, (var_cbbtstid_i * (((((var_v1 * var_fmaxr_dn5) * var_fmaxr) + (assign29430_e35742 * var_fmaxr_dn5)) * var_tmp) + (assign29430_e35744 * var_tmp_dn5))), (var_cbbtstid_i * (((((var_v1 * var_fmaxr_dn6) * var_fmaxr) + (assign29430_e35742 * var_fmaxr_dn6)) * var_tmp) + (assign29430_e35744 * var_tmp_dn6))), (var_cbbtstid_i * (((((var_v1 * var_fmaxr_dn7) * var_fmaxr) + (assign29430_e35742 * var_fmaxr_dn7)) * var_tmp) + (assign29430_e35744 * var_tmp_dn7))), (var_cbbtstid_i * (((((var_v1 * var_fmaxr_dn8) * var_fmaxr) + (assign29430_e35742 * var_fmaxr_dn8)) * var_tmp) + (assign29430_e35744 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign29430_e35749;
        var_ibbt_dn5 = assign29430_e35749_d_n5;
        var_ibbt_dn6 = assign29430_e35749_d_n6;
        var_ibbt_dn7 = assign29430_e35749_d_n7;
        var_ibbt_dn8 = assign29430_e35749_d_n8;

        let assign29440_e35752: f64 = if var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard568 = assign29440_e35752;

        let (assign29450_e35763, assign29450_e35763_d_n5, assign29450_e35763_d_n6, assign29450_e35763_d_n7, assign29450_e35763_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard568 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign29450_e35763;
        var_fbreakdown_dn5 = assign29450_e35763_d_n5;
        var_fbreakdown_dn6 = assign29450_e35763_d_n6;
        var_fbreakdown_dn7 = assign29450_e35763_d_n7;
        var_fbreakdown_dn8 = assign29450_e35763_d_n8;

        let assign29460_e35766: f64 = (-var_alphaav);
        let assign29460_e35768: f64 = (assign29460_e35766 * var_vbrstid_i);
        let assign29460_e35769: f64 = if var_vav > assign29460_e35768 { 1.0 } else { 0.0 };
        var_guard569 = assign29460_e35769;

        let assign29470_e35772: f64 = if var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        var_guard570 = assign29470_e35772;

        let (assign29480_e35802, assign29480_e35802_d_n5, assign29480_e35802_d_n6, assign29480_e35802_d_n7, assign29480_e35802_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard568 == 0.0)) && (var_guard569 != 0.0)) && (var_guard570 != 0.0)) {
        let assign29480_e35788: f64 = (var_vav * var_vbrinvsti_d);
        let assign29480_e35791: f64 = (var_vav * var_vbrinvsti_d);
        let assign29480_e35792: f64 = (assign29480_e35788 * assign29480_e35791);
        let assign29480_e35795: f64 = (var_vav * var_vbrinvsti_d);
        let assign29480_e35796: f64 = (assign29480_e35792 * assign29480_e35795);
        let assign29480_e35799: f64 = (var_vav * var_vbrinvsti_d);
        let assign29480_e35800: f64 = (assign29480_e35796 * assign29480_e35799);
        (assign29480_e35800, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29480_e35802;
        var_tmp_dn5 = assign29480_e35802_d_n5;
        var_tmp_dn6 = assign29480_e35802_d_n6;
        var_tmp_dn7 = assign29480_e35802_d_n7;
        var_tmp_dn8 = assign29480_e35802_d_n8;

        let (assign29490_e35824, assign29490_e35824_d_n5, assign29490_e35824_d_n6, assign29490_e35824_d_n7, assign29490_e35824_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard568 == 0.0)) && (var_guard569 != 0.0)) && (var_guard570 == 0.0)) {
        let assign29490_e35819: f64 = (var_vav * var_vbrinvsti_d);
        let assign29490_e35820: f64 = (assign29490_e35819).abs();
        let assign29490_e35822: f64 = (assign29490_e35820).powf(var_pbrstid_i);
        (assign29490_e35822, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29490_e35824;
        var_tmp_dn5 = assign29490_e35824_d_n5;
        var_tmp_dn6 = assign29490_e35824_d_n6;
        var_tmp_dn7 = assign29490_e35824_d_n7;
        var_tmp_dn8 = assign29490_e35824_d_n8;

        let (assign29500_e35842, assign29500_e35842_d_n5, assign29500_e35842_d_n6, assign29500_e35842_d_n7, assign29500_e35842_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard568 == 0.0)) && (var_guard569 != 0.0)) {
        let assign29500_e35839: f64 = (1.0 - var_tmp);
        let assign29500_e35840: f64 = (1.0 / assign29500_e35839);
        (assign29500_e35840, (-((-var_tmp_dn5) / (assign29500_e35839 * assign29500_e35839))), (-((-var_tmp_dn6) / (assign29500_e35839 * assign29500_e35839))), (-((-var_tmp_dn7) / (assign29500_e35839 * assign29500_e35839))), (-((-var_tmp_dn8) / (assign29500_e35839 * assign29500_e35839))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign29500_e35842;
        var_fbreakdown_dn5 = assign29500_e35842_d_n5;
        var_fbreakdown_dn6 = assign29500_e35842_d_n6;
        var_fbreakdown_dn7 = assign29500_e35842_d_n7;
        var_fbreakdown_dn8 = assign29500_e35842_d_n8;

        let (assign29510_e35865, assign29510_e35865_d_n5, assign29510_e35865_d_n6, assign29510_e35865_d_n7, assign29510_e35865_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) && (var_guard568 == 0.0)) && (var_guard569 == 0.0)) {
        let assign29510_e35859: f64 = (var_alphaav * var_vbrstid_i);
        let assign29510_e35860: f64 = (var_vav + assign29510_e35859);
        let assign29510_e35862: f64 = (assign29510_e35860 * var_slopesti_d);
        let assign29510_e35863: f64 = (var_fstopsti_d + assign29510_e35862);
        (assign29510_e35863, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign29510_e35865;
        var_fbreakdown_dn5 = assign29510_e35865_d_n5;
        var_fbreakdown_dn6 = assign29510_e35865_d_n6;
        var_fbreakdown_dn7 = assign29510_e35865_d_n7;
        var_fbreakdown_dn8 = assign29510_e35865_d_n8;

        let (assign29520_e35884, assign29520_e35884_d_n5, assign29520_e35884_d_n6, assign29520_e35884_d_n7, assign29520_e35884_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard554 == 0.0)) {
        let assign29520_e35875: f64 = (var_id__blk213 + var_isrh);
        let assign29520_e35877: f64 = (assign29520_e35875 + var_itat);
        let assign29520_e35879: f64 = (assign29520_e35877 + var_ibbt);
        let assign29520_e35880: f64 = (p.p29 * assign29520_e35879);
        let assign29520_e35882: f64 = (assign29520_e35880 * var_fbreakdown);
        (assign29520_e35882, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign29520_e35880 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign29520_e35880 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign29520_e35880 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign29520_e35880 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign29520_e35884;
        var_ijunsti_dn5 = assign29520_e35884_d_n5;
        var_ijunsti_dn6 = assign29520_e35884_d_n6;
        var_ijunsti_dn7 = assign29520_e35884_d_n7;
        var_ijunsti_dn8 = assign29520_e35884_d_n8;

        let assign29530_e35887: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard571 = assign29530_e35887;

        let (assign29540_e35895, assign29540_e35895_d_n5, assign29540_e35895_d_n6, assign29540_e35895_d_n7, assign29540_e35895_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign29540_e35895;
        var_ijungat_dn5 = assign29540_e35895_d_n5;
        var_ijungat_dn6 = assign29540_e35895_d_n6;
        var_ijungat_dn7 = assign29540_e35895_d_n7;
        var_ijungat_dn8 = assign29540_e35895_d_n8;

        let (assign29550_e35906,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) {
        let assign29550_e35904: f64 = (var_idsatgat_d * var_idmult);
        (assign29550_e35904,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign29550_e35906;

        let assign29560_e35913: f64 = if ((var_csrhgatd_i == 0.0) && (var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard572 = assign29560_e35913;

        let (assign29570_e35924, assign29570_e35924_d_n5, assign29570_e35924_d_n6, assign29570_e35924_d_n7, assign29570_e35924_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard572 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign29570_e35924;
        var_isrh_dn5 = assign29570_e35924_d_n5;
        var_isrh_dn6 = assign29570_e35924_d_n6;
        var_isrh_dn7 = assign29570_e35924_d_n7;
        var_isrh_dn8 = assign29570_e35924_d_n8;

        let (assign29580_e35938,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard572 == 0.0)) {
        let assign29580_e35936: f64 = (var_vbigat_d - var_vjsrh);
        (assign29580_e35936,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign29580_e35938;

        let (assign29590_e35957,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard572 == 0.0)) {
        let assign29590_e35952: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign29590_e35953: f64 = (1.0 - assign29590_e35952);
        let assign29590_e35954: f64 = (assign29590_e35953).sqrt();
        let assign29590_e35955: f64 = (1.0 - assign29590_e35954);
        (assign29590_e35955,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign29590_e35957;

        let assign29600_e35960: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard573 = assign29600_e35960;

        let (assign29610_e35974,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard572 == 0.0)) && (var_guard573 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign29610_e35974;

        let (assign29620_e36006,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard572 == 0.0)) && (var_guard573 == 0.0)) {
        let assign29620_e35989: f64 = (var_wsrhstep * var_wsrhstep);
        let assign29620_e35991: f64 = (var_wsrhstep).ln();
        let assign29620_e35992: f64 = (assign29620_e35989 * assign29620_e35991);
        let assign29620_e35995: f64 = (1.0 - var_wsrhstep);
        let assign29620_e35996: f64 = (assign29620_e35992 / assign29620_e35995);
        let assign29620_e35998: f64 = (assign29620_e35996 + var_wsrhstep);
        let assign29620_e36002: f64 = (2.0 * var_pgatd_i);
        let assign29620_e36003: f64 = (1.0 - assign29620_e36002);
        let assign29620_e36004: f64 = (assign29620_e35998 * assign29620_e36003);
        (assign29620_e36004,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign29620_e36006;

        let (assign29630_e36020,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard572 == 0.0)) {
        let assign29630_e36018: f64 = (var_wsrhstep + var_dwsrh);
        (assign29630_e36018,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign29630_e36020;

        let assign29640_e36023: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard574 = assign29640_e36023;

        let (assign29650_e36040, assign29650_e36040_d_n5, assign29650_e36040_d_n6, assign29650_e36040_d_n7, assign29650_e36040_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard572 == 0.0)) && (var_guard574 != 0.0)) {
        let assign29650_e36037: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign29650_e36038: f64 = (assign29650_e36037).sqrt();
        (assign29650_e36038, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29650_e36040;
        var_tmp_dn5 = assign29650_e36040_d_n5;
        var_tmp_dn6 = assign29650_e36040_d_n6;
        var_tmp_dn7 = assign29650_e36040_d_n7;
        var_tmp_dn8 = assign29650_e36040_d_n8;

        let (assign29660_e36059, assign29660_e36059_d_n5, assign29660_e36059_d_n6, assign29660_e36059_d_n7, assign29660_e36059_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard572 == 0.0)) && (var_guard574 == 0.0)) {
        let assign29660_e36055: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign29660_e36057: f64 = (assign29660_e36055).powf(var_pgatd_i);
        (assign29660_e36057, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29660_e36059;
        var_tmp_dn5 = assign29660_e36059_d_n5;
        var_tmp_dn6 = assign29660_e36059_d_n6;
        var_tmp_dn7 = assign29660_e36059_d_n7;
        var_tmp_dn8 = assign29660_e36059_d_n8;

        let (assign29670_e36073, assign29670_e36073_d_n5, assign29670_e36073_d_n6, assign29670_e36073_d_n7, assign29670_e36073_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard572 == 0.0)) {
        let assign29670_e36071: f64 = (var_wdepnulrgat_d * var_tmp);
        (assign29670_e36071, (var_wdepnulrgat_d * var_tmp_dn5), (var_wdepnulrgat_d * var_tmp_dn6), (var_wdepnulrgat_d * var_tmp_dn7), (var_wdepnulrgat_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign29670_e36073;
        var_wdep_dn5 = assign29670_e36073_d_n5;
        var_wdep_dn6 = assign29670_e36073_d_n6;
        var_wdep_dn7 = assign29670_e36073_d_n7;
        var_wdep_dn8 = assign29670_e36073_d_n8;

        let (assign29680_e36091, assign29680_e36091_d_n5, assign29680_e36091_d_n6, assign29680_e36091_d_n7, assign29680_e36091_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard572 == 0.0)) {
        let assign29680_e36086: f64 = (var_zinv - 1.0);
        let assign29680_e36088: f64 = (assign29680_e36086 * var_wdep);
        let assign29680_e36089: f64 = (var_ftdgat_d * assign29680_e36088);
        (assign29680_e36089, (var_ftdgat_d * (assign29680_e36086 * var_wdep_dn5)), (var_ftdgat_d * (assign29680_e36086 * var_wdep_dn6)), (var_ftdgat_d * (assign29680_e36086 * var_wdep_dn7)), (var_ftdgat_d * (assign29680_e36086 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign29680_e36091;
        var_asrh_dn5 = assign29680_e36091_d_n5;
        var_asrh_dn6 = assign29680_e36091_d_n6;
        var_asrh_dn7 = assign29680_e36091_d_n7;
        var_asrh_dn8 = assign29680_e36091_d_n8;

        let (assign29690_e36107, assign29690_e36107_d_n5, assign29690_e36107_d_n6, assign29690_e36107_d_n7, assign29690_e36107_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard572 == 0.0)) {
        let assign29690_e36104: f64 = (var_asrh * var_wsrh);
        let assign29690_e36105: f64 = (var_csrhgatd_i * assign29690_e36104);
        (assign29690_e36105, (var_csrhgatd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign29690_e36107;
        var_isrh_dn5 = assign29690_e36107_d_n5;
        var_isrh_dn6 = assign29690_e36107_d_n6;
        var_isrh_dn7 = assign29690_e36107_d_n7;
        var_isrh_dn8 = assign29690_e36107_d_n8;

        let assign29700_e36110: f64 = if var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard575 = assign29700_e36110;

        let (assign29710_e36121, assign29710_e36121_d_n5, assign29710_e36121_d_n6, assign29710_e36121_d_n7, assign29710_e36121_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign29710_e36121;
        var_itat_dn5 = assign29710_e36121_d_n5;
        var_itat_dn6 = assign29710_e36121_d_n6;
        var_itat_dn7 = assign29710_e36121_d_n7;
        var_itat_dn8 = assign29710_e36121_d_n8;

        let (assign29720_e36139, assign29720_e36139_d_n5, assign29720_e36139_d_n6, assign29720_e36139_d_n7, assign29720_e36139_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) {
        let assign29720_e36134: f64 = (var_wdep * var_one_minus_pgat_d);
        let assign29720_e36136: f64 = (assign29720_e36134 / var_vbi_minus_vjsrh);
        let assign29720_e36137: f64 = (var_btatpartgat_d * assign29720_e36136);
        (assign29720_e36137, (var_btatpartgat_d * ((var_wdep_dn5 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn6 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn7 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn8 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign29720_e36139;
        var_btat_dn5 = assign29720_e36139_d_n5;
        var_btat_dn6 = assign29720_e36139_d_n6;
        var_btat_dn7 = assign29720_e36139_d_n7;
        var_btat_dn8 = assign29720_e36139_d_n8;

        let (assign29730_e36155, assign29730_e36155_d_n5, assign29730_e36155_d_n6, assign29730_e36155_d_n7, assign29730_e36155_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) {
        let assign29730_e36151: f64 = (0.666666666666667 * var_atatgat_d);
        let assign29730_e36153: f64 = (assign29730_e36151 / var_btat);
        (assign29730_e36153, (-((assign29730_e36151 * var_btat_dn5) / (var_btat * var_btat))), (-((assign29730_e36151 * var_btat_dn6) / (var_btat * var_btat))), (-((assign29730_e36151 * var_btat_dn7) / (var_btat * var_btat))), (-((assign29730_e36151 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign29730_e36155;
        var_twoatatoverthreebtat_dn5 = assign29730_e36155_d_n5;
        var_twoatatoverthreebtat_dn6 = assign29730_e36155_d_n6;
        var_twoatatoverthreebtat_dn7 = assign29730_e36155_d_n7;
        var_twoatatoverthreebtat_dn8 = assign29730_e36155_d_n8;

        let (assign29740_e36169, assign29740_e36169_d_n5, assign29740_e36169_d_n6, assign29740_e36169_d_n7, assign29740_e36169_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) {
        let assign29740_e36167: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign29740_e36167, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign29740_e36169;
        var_umaxbeforelimiting_dn5 = assign29740_e36169_d_n5;
        var_umaxbeforelimiting_dn6 = assign29740_e36169_d_n6;
        var_umaxbeforelimiting_dn7 = assign29740_e36169_d_n7;
        var_umaxbeforelimiting_dn8 = assign29740_e36169_d_n8;

        let (assign29750_e36190, assign29750_e36190_d_n5, assign29750_e36190_d_n6, assign29750_e36190_d_n7, assign29750_e36190_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) {
        let assign29750_e36181: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign29750_e36184: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign29750_e36186: f64 = (assign29750_e36184 + 1.0);
        let assign29750_e36187: f64 = (assign29750_e36181 / assign29750_e36186);
        let assign29750_e36188: f64 = (assign29750_e36187).sqrt();
        (assign29750_e36188, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign29750_e36186) - (assign29750_e36181 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign29750_e36186 * assign29750_e36186)) / (2.0 * assign29750_e36188)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign29750_e36186) - (assign29750_e36181 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign29750_e36186 * assign29750_e36186)) / (2.0 * assign29750_e36188)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign29750_e36186) - (assign29750_e36181 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign29750_e36186 * assign29750_e36186)) / (2.0 * assign29750_e36188)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign29750_e36186) - (assign29750_e36181 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign29750_e36186 * assign29750_e36186)) / (2.0 * assign29750_e36188)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign29750_e36190;
        var_umax_dn5 = assign29750_e36190_d_n5;
        var_umax_dn6 = assign29750_e36190_d_n6;
        var_umax_dn7 = assign29750_e36190_d_n7;
        var_umax_dn8 = assign29750_e36190_d_n8;

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
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn5_slot = var_fmaxr_dn5;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_guard566_slot = var_guard566;
        *var_guard567_slot = var_guard567;
        *var_guard568_slot = var_guard568;
        *var_guard569_slot = var_guard569;
        *var_guard570_slot = var_guard570;
        *var_guard571_slot = var_guard571;
        *var_guard572_slot = var_guard572;
        *var_guard573_slot = var_guard573;
        *var_guard574_slot = var_guard574;
        *var_guard575_slot = var_guard575;
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
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn5_slot = var_wdep_dn5;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
    }

    pub(super) fn stamp_transient_block_59(
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatgat_d: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn5: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_cbbtgatd_i: f64,
        var_cerfc: f64,
        var_ctatgatd_i: f64,
        var_fbbtgat_d: f64,
        var_fbbtgat_d_dn5: f64,
        var_fbbtgat_d_dn6: f64,
        var_fbbtgat_d_dn7: f64,
        var_fbbtgat_d_dn8: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard571: f64,
        var_guard575: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_perfc: f64,
        var_pgatd_i: f64,
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
        var_vbbt: f64,
        var_vbirgatd_i: f64,
        var_vbirgatinv_d: f64,
        var_wdepnulrinvgat_d: f64,
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
        var_guard576_slot: &mut f64,
        var_guard577_slot: &mut f64,
        var_guard578_slot: &mut f64,
        var_guard579_slot: &mut f64,
        var_guard580_slot: &mut f64,
        var_guard581_slot: &mut f64,
        var_guard582_slot: &mut f64,
        var_guard583_slot: &mut f64,
        var_guard584_slot: &mut f64,
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
        let mut var_guard576: f64 = *var_guard576_slot;
        let mut var_guard577: f64 = *var_guard577_slot;
        let mut var_guard578: f64 = *var_guard578_slot;
        let mut var_guard579: f64 = *var_guard579_slot;
        let mut var_guard580: f64 = *var_guard580_slot;
        let mut var_guard581: f64 = *var_guard581_slot;
        let mut var_guard582: f64 = *var_guard582_slot;
        let mut var_guard583: f64 = *var_guard583_slot;
        let mut var_guard584: f64 = *var_guard584_slot;
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

        let (assign29760_e36203, assign29760_e36203_d_n5, assign29760_e36203_d_n6, assign29760_e36203_d_n7, assign29760_e36203_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) {
        let assign29760_e36201: f64 = (var_umax).sqrt();
        (assign29760_e36201, (var_umax_dn5 / (2.0 * assign29760_e36201)), (var_umax_dn6 / (2.0 * assign29760_e36201)), (var_umax_dn7 / (2.0 * assign29760_e36201)), (var_umax_dn8 / (2.0 * assign29760_e36201)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign29760_e36203;
        var_sqrtumax_dn5 = assign29760_e36203_d_n5;
        var_sqrtumax_dn6 = assign29760_e36203_d_n6;
        var_sqrtumax_dn7 = assign29760_e36203_d_n7;
        var_sqrtumax_dn8 = assign29760_e36203_d_n8;

        let (assign29770_e36217, assign29770_e36217_d_n5, assign29770_e36217_d_n6, assign29770_e36217_d_n7, assign29770_e36217_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) {
        let assign29770_e36215: f64 = (var_umax * var_sqrtumax);
        (assign29770_e36215, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign29770_e36217;
        var_umaxpoweronepointfive_dn5 = assign29770_e36217_d_n5;
        var_umaxpoweronepointfive_dn6 = assign29770_e36217_d_n6;
        var_umaxpoweronepointfive_dn7 = assign29770_e36217_d_n7;
        var_umaxpoweronepointfive_dn8 = assign29770_e36217_d_n8;

        let assign29780_e36219: f64 = (-var_pgatd_i);
        let assign29780_e36221: f64 = (assign29780_e36219 * var_one_over_one_minus_pgat_d);
        let assign29780_e36223: f64 = (-1.0);
        let assign29780_e36224: f64 = if assign29780_e36221 == assign29780_e36223 { 1.0 } else { 0.0 };
        var_guard576 = assign29780_e36224;

        let (assign29790_e36244, assign29790_e36244_d_n5, assign29790_e36244_d_n6, assign29790_e36244_d_n7, assign29790_e36244_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) && (var_guard576 != 0.0)) {
        let assign29790_e36240: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign29790_e36241: f64 = (1.0 + assign29790_e36240);
        let assign29790_e36242: f64 = (1.0 / assign29790_e36241);
        (assign29790_e36242, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign29790_e36241 * assign29790_e36241))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign29790_e36241 * assign29790_e36241))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign29790_e36241 * assign29790_e36241))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign29790_e36241 * assign29790_e36241))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign29790_e36244;
        var_wgamma_dn5 = assign29790_e36244_d_n5;
        var_wgamma_dn6 = assign29790_e36244_d_n6;
        var_wgamma_dn7 = assign29790_e36244_d_n7;
        var_wgamma_dn8 = assign29790_e36244_d_n8;

        let (assign29800_e36268, assign29800_e36268_d_n5, assign29800_e36268_d_n6, assign29800_e36268_d_n7, assign29800_e36268_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) && (var_guard576 == 0.0)) {
        let assign29800_e36260: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign29800_e36261: f64 = (1.0 + assign29800_e36260);
        let assign29800_e36263: f64 = (-var_pgatd_i);
        let assign29800_e36265: f64 = (assign29800_e36263 * var_one_over_one_minus_pgat_d);
        let assign29800_e36266: f64 = (assign29800_e36261).powf(assign29800_e36265);
        (assign29800_e36266, if 0.0 == 0.0 && ((assign29800_e36265) as f64).is_finite() && ((assign29800_e36265) as f64).fract() == 0.0 { if assign29800_e36265 == 0.0 { 0.0 } else { (assign29800_e36265 * ((assign29800_e36261).powf(assign29800_e36265 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign29800_e36266 * (assign29800_e36265 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign29800_e36261))) }, if 0.0 == 0.0 && ((assign29800_e36265) as f64).is_finite() && ((assign29800_e36265) as f64).fract() == 0.0 { if assign29800_e36265 == 0.0 { 0.0 } else { (assign29800_e36265 * ((assign29800_e36261).powf(assign29800_e36265 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign29800_e36266 * (assign29800_e36265 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign29800_e36261))) }, if 0.0 == 0.0 && ((assign29800_e36265) as f64).is_finite() && ((assign29800_e36265) as f64).fract() == 0.0 { if assign29800_e36265 == 0.0 { 0.0 } else { (assign29800_e36265 * ((assign29800_e36261).powf(assign29800_e36265 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign29800_e36266 * (assign29800_e36265 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign29800_e36261))) }, if 0.0 == 0.0 && ((assign29800_e36265) as f64).is_finite() && ((assign29800_e36265) as f64).fract() == 0.0 { if assign29800_e36265 == 0.0 { 0.0 } else { (assign29800_e36265 * ((assign29800_e36261).powf(assign29800_e36265 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign29800_e36266 * (assign29800_e36265 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign29800_e36261))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign29800_e36268;
        var_wgamma_dn5 = assign29800_e36268_d_n5;
        var_wgamma_dn6 = assign29800_e36268_d_n6;
        var_wgamma_dn7 = assign29800_e36268_d_n7;
        var_wgamma_dn8 = assign29800_e36268_d_n8;

        let (assign29810_e36286, assign29810_e36286_d_n5, assign29810_e36286_d_n6, assign29810_e36286_d_n7, assign29810_e36286_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) {
        let assign29810_e36280: f64 = (var_wsrh * var_wgamma);
        let assign29810_e36283: f64 = (var_wsrh + var_wgamma);
        let assign29810_e36284: f64 = (assign29810_e36280 / assign29810_e36283);
        (assign29810_e36284, ((((var_wsrh * var_wgamma_dn5) * assign29810_e36283) - (assign29810_e36280 * var_wgamma_dn5)) / (assign29810_e36283 * assign29810_e36283)), ((((var_wsrh * var_wgamma_dn6) * assign29810_e36283) - (assign29810_e36280 * var_wgamma_dn6)) / (assign29810_e36283 * assign29810_e36283)), ((((var_wsrh * var_wgamma_dn7) * assign29810_e36283) - (assign29810_e36280 * var_wgamma_dn7)) / (assign29810_e36283 * assign29810_e36283)), ((((var_wsrh * var_wgamma_dn8) * assign29810_e36283) - (assign29810_e36280 * var_wgamma_dn8)) / (assign29810_e36283 * assign29810_e36283)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign29810_e36286;
        var_wtat_dn5 = assign29810_e36286_d_n5;
        var_wtat_dn6 = assign29810_e36286_d_n6;
        var_wtat_dn7 = assign29810_e36286_d_n7;
        var_wtat_dn8 = assign29810_e36286_d_n8;

        let (assign29820_e36303, assign29820_e36303_d_n5, assign29820_e36303_d_n6, assign29820_e36303_d_n7, assign29820_e36303_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) {
        let assign29820_e36299: f64 = (var_btat / var_sqrtumax);
        let assign29820_e36300: f64 = (0.375 * assign29820_e36299);
        let assign29820_e36301: f64 = (assign29820_e36300).sqrt();
        (assign29820_e36301, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign29820_e36301)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign29820_e36301)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign29820_e36301)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign29820_e36301)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign29820_e36303;
        var_ktat_dn5 = assign29820_e36303_d_n5;
        var_ktat_dn6 = assign29820_e36303_d_n6;
        var_ktat_dn7 = assign29820_e36303_d_n7;
        var_ktat_dn8 = assign29820_e36303_d_n8;

        let (assign29830_e36321, assign29830_e36321_d_n5, assign29830_e36321_d_n6, assign29830_e36321_d_n7, assign29830_e36321_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) {
        let assign29830_e36316: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign29830_e36317: f64 = (2.0 * assign29830_e36316);
        let assign29830_e36319: f64 = (assign29830_e36317 - var_umax);
        (assign29830_e36319, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign29830_e36321;
        var_ltat_dn5 = assign29830_e36321_d_n5;
        var_ltat_dn6 = assign29830_e36321_d_n6;
        var_ltat_dn7 = assign29830_e36321_d_n7;
        var_ltat_dn8 = assign29830_e36321_d_n8;

        let (assign29840_e36347, assign29840_e36347_d_n5, assign29840_e36347_d_n6, assign29840_e36347_d_n7, assign29840_e36347_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) {
        let assign29840_e36333: f64 = (var_atatgat_d * var_twoatatoverthreebtat);
        let assign29840_e36335: f64 = (assign29840_e36333 * var_sqrtumax);
        let assign29840_e36338: f64 = (var_atatgat_d * var_umax);
        let assign29840_e36339: f64 = (assign29840_e36335 - assign29840_e36338);
        let assign29840_e36343: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign29840_e36344: f64 = (0.5 * assign29840_e36343);
        let assign29840_e36345: f64 = (assign29840_e36339 + assign29840_e36344);
        (assign29840_e36345, (((((var_atatgat_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign29840_e36333 * var_sqrtumax_dn5)) - (var_atatgat_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign29840_e36333 * var_sqrtumax_dn6)) - (var_atatgat_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign29840_e36333 * var_sqrtumax_dn7)) - (var_atatgat_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign29840_e36333 * var_sqrtumax_dn8)) - (var_atatgat_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign29840_e36347;
        var_mtat_dn5 = assign29840_e36347_d_n5;
        var_mtat_dn6 = assign29840_e36347_d_n6;
        var_mtat_dn7 = assign29840_e36347_d_n7;
        var_mtat_dn8 = assign29840_e36347_d_n8;

        let (assign29850_e36363, assign29850_e36363_d_n5, assign29850_e36363_d_n6, assign29850_e36363_d_n7, assign29850_e36363_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) {
        let assign29850_e36359: f64 = (var_ltat - 1.0);
        let assign29850_e36361: f64 = (assign29850_e36359 * var_ktat);
        (assign29850_e36361, ((var_ltat_dn5 * var_ktat) + (assign29850_e36359 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign29850_e36359 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign29850_e36359 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign29850_e36359 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign29850_e36363;
        var_xerfc_dn5 = assign29850_e36363_d_n5;
        var_xerfc_dn6 = assign29850_e36363_d_n6;
        var_xerfc_dn7 = assign29850_e36363_d_n7;
        var_xerfc_dn8 = assign29850_e36363_d_n8;

        let (assign29860_e36377, assign29860_e36377_d_n5, assign29860_e36377_d_n6, assign29860_e36377_d_n7, assign29860_e36377_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) {
        let assign29860_e36375: f64 = (var_xerfc * var_xerfc);
        (assign29860_e36375, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign29860_e36377;
        var_ysq_dn5 = assign29860_e36377_d_n5;
        var_ysq_dn6 = assign29860_e36377_d_n6;
        var_ysq_dn7 = assign29860_e36377_d_n7;
        var_ysq_dn8 = assign29860_e36377_d_n8;

        let assign29870_e36380: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard577 = assign29870_e36380;

        let (assign29880_e36400, assign29880_e36400_d_n5, assign29880_e36400_d_n6, assign29880_e36400_d_n7, assign29880_e36400_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) && (var_guard577 != 0.0)) {
        let assign29880_e36396: f64 = (var_perfc * var_xerfc);
        let assign29880_e36397: f64 = (1.0 + assign29880_e36396);
        let assign29880_e36398: f64 = (1.0 / assign29880_e36397);
        (assign29880_e36398, (-((var_perfc * var_xerfc_dn5) / (assign29880_e36397 * assign29880_e36397))), (-((var_perfc * var_xerfc_dn6) / (assign29880_e36397 * assign29880_e36397))), (-((var_perfc * var_xerfc_dn7) / (assign29880_e36397 * assign29880_e36397))), (-((var_perfc * var_xerfc_dn8) / (assign29880_e36397 * assign29880_e36397))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign29880_e36400;
        var_terfc_dn5 = assign29880_e36400_d_n5;
        var_terfc_dn6 = assign29880_e36400_d_n6;
        var_terfc_dn7 = assign29880_e36400_d_n7;
        var_terfc_dn8 = assign29880_e36400_d_n8;

        let (assign29890_e36421, assign29890_e36421_d_n5, assign29890_e36421_d_n6, assign29890_e36421_d_n7, assign29890_e36421_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) && (var_guard577 == 0.0)) {
        let assign29890_e36417: f64 = (var_perfc * var_xerfc);
        let assign29890_e36418: f64 = (1.0 - assign29890_e36417);
        let assign29890_e36419: f64 = (1.0 / assign29890_e36418);
        (assign29890_e36419, (-((-(var_perfc * var_xerfc_dn5)) / (assign29890_e36418 * assign29890_e36418))), (-((-(var_perfc * var_xerfc_dn6)) / (assign29890_e36418 * assign29890_e36418))), (-((-(var_perfc * var_xerfc_dn7)) / (assign29890_e36418 * assign29890_e36418))), (-((-(var_perfc * var_xerfc_dn8)) / (assign29890_e36418 * assign29890_e36418))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign29890_e36421;
        var_terfc_dn5 = assign29890_e36421_d_n5;
        var_terfc_dn6 = assign29890_e36421_d_n6;
        var_terfc_dn7 = assign29890_e36421_d_n7;
        var_terfc_dn8 = assign29890_e36421_d_n8;

        let assign29900_e36423: f64 = (-var_ysq);
        let assign29900_e36425: f64 = (assign29900_e36423 + var_mtat);
        let assign29900_e36427: f64 = (-230.25850929940458);
        let assign29900_e36428: f64 = if assign29900_e36425 > assign29900_e36427 { 1.0 } else { 0.0 };
        var_guard578 = assign29900_e36428;

        let (assign29910_e36446, assign29910_e36446_d_n5, assign29910_e36446_d_n6, assign29910_e36446_d_n7, assign29910_e36446_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) && (var_guard578 != 0.0)) {
        let assign29910_e36441: f64 = (-var_ysq);
        let assign29910_e36443: f64 = (assign29910_e36441 + var_mtat);
        let assign29910_e36444: f64 = (assign29910_e36443).exp();
        (assign29910_e36444, (assign29910_e36444 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign29910_e36444 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign29910_e36444 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign29910_e36444 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29910_e36446;
        var_tmp_dn5 = assign29910_e36446_d_n5;
        var_tmp_dn6 = assign29910_e36446_d_n6;
        var_tmp_dn7 = assign29910_e36446_d_n7;
        var_tmp_dn8 = assign29910_e36446_d_n8;

        let (assign29920_e36495, assign29920_e36495_d_n5, assign29920_e36495_d_n6, assign29920_e36495_d_n7, assign29920_e36495_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) && (var_guard578 == 0.0)) {
        let assign29920_e36462: f64 = (-230.25850929940458);
        let assign29920_e36464: f64 = (-var_ysq);
        let assign29920_e36466: f64 = (assign29920_e36464 + var_mtat);
        let assign29920_e36467: f64 = (assign29920_e36462 - assign29920_e36466);
        let assign29920_e36471: f64 = (-230.25850929940458);
        let assign29920_e36473: f64 = (-var_ysq);
        let assign29920_e36475: f64 = (assign29920_e36473 + var_mtat);
        let assign29920_e36476: f64 = (assign29920_e36471 - assign29920_e36475);
        let assign29920_e36479: f64 = (-230.25850929940458);
        let assign29920_e36481: f64 = (-var_ysq);
        let assign29920_e36483: f64 = (assign29920_e36481 + var_mtat);
        let assign29920_e36484: f64 = (assign29920_e36479 - assign29920_e36483);
        let assign29920_e36486: f64 = (assign29920_e36484 * 0.3333333333333333);
        let assign29920_e36487: f64 = (1.0 + assign29920_e36486);
        let assign29920_e36488: f64 = (assign29920_e36476 * assign29920_e36487);
        let assign29920_e36489: f64 = (0.5 * assign29920_e36488);
        let assign29920_e36490: f64 = (1.0 + assign29920_e36489);
        let assign29920_e36491: f64 = (assign29920_e36467 * assign29920_e36490);
        let assign29920_e36492: f64 = (1.0 + assign29920_e36491);
        let assign29920_e36493: f64 = (1e-100 / assign29920_e36492);
        (assign29920_e36493, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign29920_e36490) + (assign29920_e36467 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign29920_e36487) + (assign29920_e36476 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign29920_e36492 * assign29920_e36492))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign29920_e36490) + (assign29920_e36467 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign29920_e36487) + (assign29920_e36476 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign29920_e36492 * assign29920_e36492))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign29920_e36490) + (assign29920_e36467 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign29920_e36487) + (assign29920_e36476 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign29920_e36492 * assign29920_e36492))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign29920_e36490) + (assign29920_e36467 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign29920_e36487) + (assign29920_e36476 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign29920_e36492 * assign29920_e36492))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29920_e36495;
        var_tmp_dn5 = assign29920_e36495_d_n5;
        var_tmp_dn6 = assign29920_e36495_d_n6;
        var_tmp_dn7 = assign29920_e36495_d_n7;
        var_tmp_dn8 = assign29920_e36495_d_n8;

        let (assign29930_e36525, assign29930_e36525_d_n5, assign29930_e36525_d_n6, assign29930_e36525_d_n7, assign29930_e36525_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) {
        let assign29930_e36507: f64 = (0.29214664 * var_terfc);
        let assign29930_e36511: f64 = (var_terfc * var_terfc);
        let assign29930_e36512: f64 = (var_berfc * assign29930_e36511);
        let assign29930_e36513: f64 = (assign29930_e36507 + assign29930_e36512);
        let assign29930_e36517: f64 = (var_terfc * var_terfc);
        let assign29930_e36519: f64 = (assign29930_e36517 * var_terfc);
        let assign29930_e36520: f64 = (var_cerfc * assign29930_e36519);
        let assign29930_e36521: f64 = (assign29930_e36513 + assign29930_e36520);
        let assign29930_e36523: f64 = (assign29930_e36521 * var_tmp);
        (assign29930_e36523, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign29930_e36517 * var_terfc_dn5)))) * var_tmp) + (assign29930_e36521 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign29930_e36517 * var_terfc_dn6)))) * var_tmp) + (assign29930_e36521 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign29930_e36517 * var_terfc_dn7)))) * var_tmp) + (assign29930_e36521 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign29930_e36517 * var_terfc_dn8)))) * var_tmp) + (assign29930_e36521 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign29930_e36525;
        var_erfcpos_dn5 = assign29930_e36525_d_n5;
        var_erfcpos_dn6 = assign29930_e36525_d_n6;
        var_erfcpos_dn7 = assign29930_e36525_d_n7;
        var_erfcpos_dn8 = assign29930_e36525_d_n8;

        let assign29940_e36528: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard579 = assign29940_e36528;

        let (assign29950_e36542, assign29950_e36542_d_n5, assign29950_e36542_d_n6, assign29950_e36542_d_n7, assign29950_e36542_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) && (var_guard579 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign29950_e36542;
        var_erfctimesexpmtat_dn5 = assign29950_e36542_d_n5;
        var_erfctimesexpmtat_dn6 = assign29950_e36542_d_n6;
        var_erfctimesexpmtat_dn7 = assign29950_e36542_d_n7;
        var_erfctimesexpmtat_dn8 = assign29950_e36542_d_n8;

        let assign29960_e36545: f64 = (-230.25850929940458);
        let assign29960_e36546: f64 = if var_mtat > assign29960_e36545 { 1.0 } else { 0.0 };
        var_guard580 = assign29960_e36546;

        let (assign29970_e36564, assign29970_e36564_d_n5, assign29970_e36564_d_n6, assign29970_e36564_d_n7, assign29970_e36564_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) && (var_guard579 == 0.0)) && (var_guard580 != 0.0)) {
        let assign29970_e36562: f64 = (var_mtat).exp();
        (assign29970_e36562, (assign29970_e36562 * var_mtat_dn5), (assign29970_e36562 * var_mtat_dn6), (assign29970_e36562 * var_mtat_dn7), (assign29970_e36562 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29970_e36564;
        var_tmp_dn5 = assign29970_e36564_d_n5;
        var_tmp_dn6 = assign29970_e36564_d_n6;
        var_tmp_dn7 = assign29970_e36564_d_n7;
        var_tmp_dn8 = assign29970_e36564_d_n8;

        let (assign29980_e36607, assign29980_e36607_d_n5, assign29980_e36607_d_n6, assign29980_e36607_d_n7, assign29980_e36607_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) && (var_guard579 == 0.0)) && (var_guard580 == 0.0)) {
        let assign29980_e36583: f64 = (-230.25850929940458);
        let assign29980_e36585: f64 = (assign29980_e36583 - var_mtat);
        let assign29980_e36589: f64 = (-230.25850929940458);
        let assign29980_e36591: f64 = (assign29980_e36589 - var_mtat);
        let assign29980_e36594: f64 = (-230.25850929940458);
        let assign29980_e36596: f64 = (assign29980_e36594 - var_mtat);
        let assign29980_e36598: f64 = (assign29980_e36596 * 0.3333333333333333);
        let assign29980_e36599: f64 = (1.0 + assign29980_e36598);
        let assign29980_e36600: f64 = (assign29980_e36591 * assign29980_e36599);
        let assign29980_e36601: f64 = (0.5 * assign29980_e36600);
        let assign29980_e36602: f64 = (1.0 + assign29980_e36601);
        let assign29980_e36603: f64 = (assign29980_e36585 * assign29980_e36602);
        let assign29980_e36604: f64 = (1.0 + assign29980_e36603);
        let assign29980_e36605: f64 = (1e-100 / assign29980_e36604);
        (assign29980_e36605, (-((1e-100 * (((-var_mtat_dn5) * assign29980_e36602) + (assign29980_e36585 * (0.5 * (((-var_mtat_dn5) * assign29980_e36599) + (assign29980_e36591 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign29980_e36604 * assign29980_e36604))), (-((1e-100 * (((-var_mtat_dn6) * assign29980_e36602) + (assign29980_e36585 * (0.5 * (((-var_mtat_dn6) * assign29980_e36599) + (assign29980_e36591 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign29980_e36604 * assign29980_e36604))), (-((1e-100 * (((-var_mtat_dn7) * assign29980_e36602) + (assign29980_e36585 * (0.5 * (((-var_mtat_dn7) * assign29980_e36599) + (assign29980_e36591 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign29980_e36604 * assign29980_e36604))), (-((1e-100 * (((-var_mtat_dn8) * assign29980_e36602) + (assign29980_e36585 * (0.5 * (((-var_mtat_dn8) * assign29980_e36599) + (assign29980_e36591 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign29980_e36604 * assign29980_e36604))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29980_e36607;
        var_tmp_dn5 = assign29980_e36607_d_n5;
        var_tmp_dn6 = assign29980_e36607_d_n6;
        var_tmp_dn7 = assign29980_e36607_d_n7;
        var_tmp_dn8 = assign29980_e36607_d_n8;

        let (assign29990_e36626, assign29990_e36626_d_n5, assign29990_e36626_d_n6, assign29990_e36626_d_n7, assign29990_e36626_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) && (var_guard579 == 0.0)) {
        let assign29990_e36622: f64 = (2.0 * var_tmp);
        let assign29990_e36624: f64 = (assign29990_e36622 - var_erfcpos);
        (assign29990_e36624, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign29990_e36626;
        var_erfctimesexpmtat_dn5 = assign29990_e36626_d_n5;
        var_erfctimesexpmtat_dn6 = assign29990_e36626_d_n6;
        var_erfctimesexpmtat_dn7 = assign29990_e36626_d_n7;
        var_erfctimesexpmtat_dn8 = assign29990_e36626_d_n8;

        let (assign30000_e36646, assign30000_e36646_d_n5, assign30000_e36646_d_n6, assign30000_e36646_d_n7, assign30000_e36646_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) {
        let assign30000_e36638: f64 = (1.772453850905516 * 0.5);
        let assign30000_e36641: f64 = (var_atatgat_d * var_erfctimesexpmtat);
        let assign30000_e36643: f64 = (assign30000_e36641 / var_ktat);
        let assign30000_e36644: f64 = (assign30000_e36638 * assign30000_e36643);
        (assign30000_e36644, (assign30000_e36638 * ((((var_atatgat_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign30000_e36641 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign30000_e36638 * ((((var_atatgat_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign30000_e36641 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign30000_e36638 * ((((var_atatgat_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign30000_e36641 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign30000_e36638 * ((((var_atatgat_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign30000_e36641 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign30000_e36646;
        var_gammamax_dn5 = assign30000_e36646_d_n5;
        var_gammamax_dn6 = assign30000_e36646_d_n6;
        var_gammamax_dn7 = assign30000_e36646_d_n7;
        var_gammamax_dn8 = assign30000_e36646_d_n8;

        let (assign30010_e36664, assign30010_e36664_d_n5, assign30010_e36664_d_n6, assign30010_e36664_d_n7, assign30010_e36664_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard575 == 0.0)) {
        let assign30010_e36659: f64 = (var_asrh * var_gammamax);
        let assign30010_e36661: f64 = (assign30010_e36659 * var_wtat);
        let assign30010_e36662: f64 = (var_ctatgatd_i * assign30010_e36661);
        (assign30010_e36662, (var_ctatgatd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign30010_e36659 * var_wtat_dn5))), (var_ctatgatd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign30010_e36659 * var_wtat_dn6))), (var_ctatgatd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign30010_e36659 * var_wtat_dn7))), (var_ctatgatd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign30010_e36659 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign30010_e36664;
        var_itat_dn5 = assign30010_e36664_d_n5;
        var_itat_dn6 = assign30010_e36664_d_n6;
        var_itat_dn7 = assign30010_e36664_d_n7;
        var_itat_dn8 = assign30010_e36664_d_n8;

        let assign30020_e36667: f64 = if var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard581 = assign30020_e36667;

        let (assign30030_e36678, assign30030_e36678_d_n5, assign30030_e36678_d_n6, assign30030_e36678_d_n7, assign30030_e36678_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard581 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign30030_e36678;
        var_ibbt_dn5 = assign30030_e36678_d_n5;
        var_ibbt_dn6 = assign30030_e36678_d_n6;
        var_ibbt_dn7 = assign30030_e36678_d_n7;
        var_ibbt_dn8 = assign30030_e36678_d_n8;

        let assign30040_e36681: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard582 = assign30040_e36681;

        let (assign30050_e36700, assign30050_e36700_d_n5, assign30050_e36700_d_n6, assign30050_e36700_d_n7, assign30050_e36700_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard581 == 0.0)) && (var_guard582 != 0.0)) {
        let assign30050_e36695: f64 = (var_vbirgatd_i - var_vbbt);
        let assign30050_e36697: f64 = (assign30050_e36695 * var_vbirgatinv_d);
        let assign30050_e36698: f64 = (assign30050_e36697).sqrt();
        (assign30050_e36698, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30050_e36700;
        var_tmp_dn5 = assign30050_e36700_d_n5;
        var_tmp_dn6 = assign30050_e36700_d_n6;
        var_tmp_dn7 = assign30050_e36700_d_n7;
        var_tmp_dn8 = assign30050_e36700_d_n8;

        let (assign30060_e36721, assign30060_e36721_d_n5, assign30060_e36721_d_n6, assign30060_e36721_d_n7, assign30060_e36721_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard581 == 0.0)) && (var_guard582 == 0.0)) {
        let assign30060_e36715: f64 = (var_vbirgatd_i - var_vbbt);
        let assign30060_e36717: f64 = (assign30060_e36715 * var_vbirgatinv_d);
        let assign30060_e36719: f64 = (assign30060_e36717).powf(var_pgatd_i);
        (assign30060_e36719, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30060_e36721;
        var_tmp_dn5 = assign30060_e36721_d_n5;
        var_tmp_dn6 = assign30060_e36721_d_n6;
        var_tmp_dn7 = assign30060_e36721_d_n7;
        var_tmp_dn8 = assign30060_e36721_d_n8;

        let (assign30070_e36741, assign30070_e36741_d_n5, assign30070_e36741_d_n6, assign30070_e36741_d_n7, assign30070_e36741_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard581 == 0.0)) {
        let assign30070_e36734: f64 = (var_vbirgatd_i - var_vbbt);
        let assign30070_e36736: f64 = (assign30070_e36734 * var_wdepnulrinvgat_d);
        let assign30070_e36738: f64 = (assign30070_e36736 / var_tmp);
        let assign30070_e36739: f64 = (var_one_over_one_minus_pgat_d * assign30070_e36738);
        (assign30070_e36739, (var_one_over_one_minus_pgat_d * (-((assign30070_e36736 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign30070_e36736 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign30070_e36736 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign30070_e36736 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign30070_e36741;
        var_fmaxr_dn5 = assign30070_e36741_d_n5;
        var_fmaxr_dn6 = assign30070_e36741_d_n6;
        var_fmaxr_dn7 = assign30070_e36741_d_n7;
        var_fmaxr_dn8 = assign30070_e36741_d_n8;

        let assign30080_e36743: f64 = (-var_fbbtgat_d);
        let assign30080_e36745: f64 = (assign30080_e36743 / var_fmaxr);
        let assign30080_e36746: f64 = (assign30080_e36745).abs();
        let assign30080_e36748: f64 = if assign30080_e36746 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard583 = assign30080_e36748;

        let (assign30090_e36766, assign30090_e36766_d_n5, assign30090_e36766_d_n6, assign30090_e36766_d_n7, assign30090_e36766_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard581 == 0.0)) && (var_guard583 != 0.0)) {
        let assign30090_e36761: f64 = (-var_fbbtgat_d);
        let assign30090_e36763: f64 = (assign30090_e36761 / var_fmaxr);
        let assign30090_e36764: f64 = (assign30090_e36763).exp();
        (assign30090_e36764, (assign30090_e36764 * ((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign30090_e36761 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign30090_e36764 * ((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign30090_e36761 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign30090_e36764 * ((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign30090_e36761 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign30090_e36764 * ((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign30090_e36761 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30090_e36766;
        var_tmp_dn5 = assign30090_e36766_d_n5;
        var_tmp_dn6 = assign30090_e36766_d_n6;
        var_tmp_dn7 = assign30090_e36766_d_n7;
        var_tmp_dn8 = assign30090_e36766_d_n8;

        let assign30100_e36768: f64 = (-var_fbbtgat_d);
        let assign30100_e36770: f64 = (assign30100_e36768 / var_fmaxr);
        let assign30100_e36772: f64 = if assign30100_e36770 < 0.0 { 1.0 } else { 0.0 };
        var_guard584 = assign30100_e36772;

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
        *var_guard576_slot = var_guard576;
        *var_guard577_slot = var_guard577;
        *var_guard578_slot = var_guard578;
        *var_guard579_slot = var_guard579;
        *var_guard580_slot = var_guard580;
        *var_guard581_slot = var_guard581;
        *var_guard582_slot = var_guard582;
        *var_guard583_slot = var_guard583;
        *var_guard584_slot = var_guard584;
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

    pub(super) fn stamp_transient_block_60(
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
        var_fmaxr: f64,
        var_fmaxr_dn5: f64,
        var_fmaxr_dn6: f64,
        var_fmaxr_dn7: f64,
        var_fmaxr_dn8: f64,
        var_fstopgat_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard571: f64,
        var_guard581: f64,
        var_guard583: f64,
        var_guard584: f64,
        var_id__blk213: f64,
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
        var_pbrgatd_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_slopegat_d: f64,
        var_slopegat_d_dn5: f64,
        var_slopegat_d_dn6: f64,
        var_slopegat_d_dn7: f64,
        var_slopegat_d_dn8: f64,
        var_v1: f64,
        var_v2: f64,
        var_vbbtlim_d: f64,
        var_vbimin_d: f64,
        var_vbrgatd_i: f64,
        var_vbrinvgat_d: f64,
        var_vbrinvgat_d_dn5: f64,
        var_vbrinvgat_d_dn6: f64,
        var_vbrinvgat_d_dn7: f64,
        var_vbrinvgat_d_dn8: f64,
        var_vmax_d: f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_guard585_slot: &mut f64,
        var_guard586_slot: &mut f64,
        var_guard587_slot: &mut f64,
        var_guard588_slot: &mut f64,
        var_guard589_slot: &mut f64,
        var_guard590_slot: &mut f64,
        var_guard591_slot: &mut f64,
        var_guard592_slot: &mut f64,
        var_guard593_slot: &mut f64,
        var_i1_slot: &mut f64,
        var_i1_dn5_slot: &mut f64,
        var_i1_dn6_slot: &mut f64,
        var_i1_dn7_slot: &mut f64,
        var_i1_dn8_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
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
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_guard585: f64 = *var_guard585_slot;
        let mut var_guard586: f64 = *var_guard586_slot;
        let mut var_guard587: f64 = *var_guard587_slot;
        let mut var_guard588: f64 = *var_guard588_slot;
        let mut var_guard589: f64 = *var_guard589_slot;
        let mut var_guard590: f64 = *var_guard590_slot;
        let mut var_guard591: f64 = *var_guard591_slot;
        let mut var_guard592: f64 = *var_guard592_slot;
        let mut var_guard593: f64 = *var_guard593_slot;
        let mut var_i1: f64 = *var_i1_slot;
        let mut var_i1_dn5: f64 = *var_i1_dn5_slot;
        let mut var_i1_dn6: f64 = *var_i1_dn6_slot;
        let mut var_i1_dn7: f64 = *var_i1_dn7_slot;
        let mut var_i1_dn8: f64 = *var_i1_dn8_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
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
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign30110_e36823, assign30110_e36823_d_n5, assign30110_e36823_d_n6, assign30110_e36823_d_n7, assign30110_e36823_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard581 == 0.0)) && (var_guard583 == 0.0)) && (var_guard584 != 0.0)) {
        let assign30110_e36790: f64 = (-230.25850929940458);
        let assign30110_e36792: f64 = (-var_fbbtgat_d);
        let assign30110_e36794: f64 = (assign30110_e36792 / var_fmaxr);
        let assign30110_e36795: f64 = (assign30110_e36790 - assign30110_e36794);
        let assign30110_e36799: f64 = (-230.25850929940458);
        let assign30110_e36801: f64 = (-var_fbbtgat_d);
        let assign30110_e36803: f64 = (assign30110_e36801 / var_fmaxr);
        let assign30110_e36804: f64 = (assign30110_e36799 - assign30110_e36803);
        let assign30110_e36807: f64 = (-230.25850929940458);
        let assign30110_e36809: f64 = (-var_fbbtgat_d);
        let assign30110_e36811: f64 = (assign30110_e36809 / var_fmaxr);
        let assign30110_e36812: f64 = (assign30110_e36807 - assign30110_e36811);
        let assign30110_e36814: f64 = (assign30110_e36812 * 0.3333333333333333);
        let assign30110_e36815: f64 = (1.0 + assign30110_e36814);
        let assign30110_e36816: f64 = (assign30110_e36804 * assign30110_e36815);
        let assign30110_e36817: f64 = (0.5 * assign30110_e36816);
        let assign30110_e36818: f64 = (1.0 + assign30110_e36817);
        let assign30110_e36819: f64 = (assign30110_e36795 * assign30110_e36818);
        let assign30110_e36820: f64 = (1.0 + assign30110_e36819);
        let assign30110_e36821: f64 = (1e-100 / assign30110_e36820);
        (assign30110_e36821, (-((1e-100 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign30110_e36792 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign30110_e36818) + (assign30110_e36795 * (0.5 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign30110_e36801 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign30110_e36815) + (assign30110_e36804 * ((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign30110_e36809 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign30110_e36820 * assign30110_e36820))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign30110_e36792 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign30110_e36818) + (assign30110_e36795 * (0.5 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign30110_e36801 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign30110_e36815) + (assign30110_e36804 * ((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign30110_e36809 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign30110_e36820 * assign30110_e36820))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign30110_e36792 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign30110_e36818) + (assign30110_e36795 * (0.5 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign30110_e36801 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign30110_e36815) + (assign30110_e36804 * ((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign30110_e36809 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign30110_e36820 * assign30110_e36820))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign30110_e36792 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign30110_e36818) + (assign30110_e36795 * (0.5 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign30110_e36801 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign30110_e36815) + (assign30110_e36804 * ((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign30110_e36809 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign30110_e36820 * assign30110_e36820))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30110_e36823;
        var_tmp_dn5 = assign30110_e36823_d_n5;
        var_tmp_dn6 = assign30110_e36823_d_n6;
        var_tmp_dn7 = assign30110_e36823_d_n7;
        var_tmp_dn8 = assign30110_e36823_d_n8;

        let (assign30120_e36872, assign30120_e36872_d_n5, assign30120_e36872_d_n6, assign30120_e36872_d_n7, assign30120_e36872_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard581 == 0.0)) && (var_guard583 == 0.0)) && (var_guard584 == 0.0)) {
        let assign30120_e36842: f64 = (-var_fbbtgat_d);
        let assign30120_e36844: f64 = (assign30120_e36842 / var_fmaxr);
        let assign30120_e36846: f64 = (assign30120_e36844 - 230.25850929940458);
        let assign30120_e36850: f64 = (-var_fbbtgat_d);
        let assign30120_e36852: f64 = (assign30120_e36850 / var_fmaxr);
        let assign30120_e36854: f64 = (assign30120_e36852 - 230.25850929940458);
        let assign30120_e36857: f64 = (-var_fbbtgat_d);
        let assign30120_e36859: f64 = (assign30120_e36857 / var_fmaxr);
        let assign30120_e36861: f64 = (assign30120_e36859 - 230.25850929940458);
        let assign30120_e36863: f64 = (assign30120_e36861 * 0.3333333333333333);
        let assign30120_e36864: f64 = (1.0 + assign30120_e36863);
        let assign30120_e36865: f64 = (assign30120_e36854 * assign30120_e36864);
        let assign30120_e36866: f64 = (0.5 * assign30120_e36865);
        let assign30120_e36867: f64 = (1.0 + assign30120_e36866);
        let assign30120_e36868: f64 = (assign30120_e36846 * assign30120_e36867);
        let assign30120_e36869: f64 = (1.0 + assign30120_e36868);
        let assign30120_e36870: f64 = (1e100 * assign30120_e36869);
        (assign30120_e36870, (1e100 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign30120_e36842 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign30120_e36867) + (assign30120_e36846 * (0.5 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign30120_e36850 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign30120_e36864) + (assign30120_e36854 * (((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign30120_e36857 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign30120_e36842 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign30120_e36867) + (assign30120_e36846 * (0.5 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign30120_e36850 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign30120_e36864) + (assign30120_e36854 * (((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign30120_e36857 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign30120_e36842 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign30120_e36867) + (assign30120_e36846 * (0.5 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign30120_e36850 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign30120_e36864) + (assign30120_e36854 * (((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign30120_e36857 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign30120_e36842 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign30120_e36867) + (assign30120_e36846 * (0.5 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign30120_e36850 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign30120_e36864) + (assign30120_e36854 * (((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign30120_e36857 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30120_e36872;
        var_tmp_dn5 = assign30120_e36872_d_n5;
        var_tmp_dn6 = assign30120_e36872_d_n6;
        var_tmp_dn7 = assign30120_e36872_d_n7;
        var_tmp_dn8 = assign30120_e36872_d_n8;

        let (assign30130_e36892, assign30130_e36892_d_n5, assign30130_e36892_d_n6, assign30130_e36892_d_n7, assign30130_e36892_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard581 == 0.0)) {
        let assign30130_e36885: f64 = (var_v1 * var_fmaxr);
        let assign30130_e36887: f64 = (assign30130_e36885 * var_fmaxr);
        let assign30130_e36889: f64 = (assign30130_e36887 * var_tmp);
        let assign30130_e36890: f64 = (var_cbbtgatd_i * assign30130_e36889);
        (assign30130_e36890, (var_cbbtgatd_i * (((((var_v1 * var_fmaxr_dn5) * var_fmaxr) + (assign30130_e36885 * var_fmaxr_dn5)) * var_tmp) + (assign30130_e36887 * var_tmp_dn5))), (var_cbbtgatd_i * (((((var_v1 * var_fmaxr_dn6) * var_fmaxr) + (assign30130_e36885 * var_fmaxr_dn6)) * var_tmp) + (assign30130_e36887 * var_tmp_dn6))), (var_cbbtgatd_i * (((((var_v1 * var_fmaxr_dn7) * var_fmaxr) + (assign30130_e36885 * var_fmaxr_dn7)) * var_tmp) + (assign30130_e36887 * var_tmp_dn7))), (var_cbbtgatd_i * (((((var_v1 * var_fmaxr_dn8) * var_fmaxr) + (assign30130_e36885 * var_fmaxr_dn8)) * var_tmp) + (assign30130_e36887 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign30130_e36892;
        var_ibbt_dn5 = assign30130_e36892_d_n5;
        var_ibbt_dn6 = assign30130_e36892_d_n6;
        var_ibbt_dn7 = assign30130_e36892_d_n7;
        var_ibbt_dn8 = assign30130_e36892_d_n8;

        let assign30140_e36895: f64 = if var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard585 = assign30140_e36895;

        let (assign30150_e36906, assign30150_e36906_d_n5, assign30150_e36906_d_n6, assign30150_e36906_d_n7, assign30150_e36906_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard585 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign30150_e36906;
        var_fbreakdown_dn5 = assign30150_e36906_d_n5;
        var_fbreakdown_dn6 = assign30150_e36906_d_n6;
        var_fbreakdown_dn7 = assign30150_e36906_d_n7;
        var_fbreakdown_dn8 = assign30150_e36906_d_n8;

        let assign30160_e36909: f64 = (-var_alphaav);
        let assign30160_e36911: f64 = (assign30160_e36909 * var_vbrgatd_i);
        let assign30160_e36912: f64 = if var_vav > assign30160_e36911 { 1.0 } else { 0.0 };
        var_guard586 = assign30160_e36912;

        let assign30170_e36915: f64 = if var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard587 = assign30170_e36915;

        let (assign30180_e36945, assign30180_e36945_d_n5, assign30180_e36945_d_n6, assign30180_e36945_d_n7, assign30180_e36945_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard585 == 0.0)) && (var_guard586 != 0.0)) && (var_guard587 != 0.0)) {
        let assign30180_e36931: f64 = (var_vav * var_vbrinvgat_d);
        let assign30180_e36934: f64 = (var_vav * var_vbrinvgat_d);
        let assign30180_e36935: f64 = (assign30180_e36931 * assign30180_e36934);
        let assign30180_e36938: f64 = (var_vav * var_vbrinvgat_d);
        let assign30180_e36939: f64 = (assign30180_e36935 * assign30180_e36938);
        let assign30180_e36942: f64 = (var_vav * var_vbrinvgat_d);
        let assign30180_e36943: f64 = (assign30180_e36939 * assign30180_e36942);
        (assign30180_e36943, (((((((var_vav * var_vbrinvgat_d_dn5) * assign30180_e36934) + (assign30180_e36931 * (var_vav * var_vbrinvgat_d_dn5))) * assign30180_e36938) + (assign30180_e36935 * (var_vav * var_vbrinvgat_d_dn5))) * assign30180_e36942) + (assign30180_e36939 * (var_vav * var_vbrinvgat_d_dn5))), (((((((var_vav * var_vbrinvgat_d_dn6) * assign30180_e36934) + (assign30180_e36931 * (var_vav * var_vbrinvgat_d_dn6))) * assign30180_e36938) + (assign30180_e36935 * (var_vav * var_vbrinvgat_d_dn6))) * assign30180_e36942) + (assign30180_e36939 * (var_vav * var_vbrinvgat_d_dn6))), (((((((var_vav * var_vbrinvgat_d_dn7) * assign30180_e36934) + (assign30180_e36931 * (var_vav * var_vbrinvgat_d_dn7))) * assign30180_e36938) + (assign30180_e36935 * (var_vav * var_vbrinvgat_d_dn7))) * assign30180_e36942) + (assign30180_e36939 * (var_vav * var_vbrinvgat_d_dn7))), (((((((var_vav * var_vbrinvgat_d_dn8) * assign30180_e36934) + (assign30180_e36931 * (var_vav * var_vbrinvgat_d_dn8))) * assign30180_e36938) + (assign30180_e36935 * (var_vav * var_vbrinvgat_d_dn8))) * assign30180_e36942) + (assign30180_e36939 * (var_vav * var_vbrinvgat_d_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30180_e36945;
        var_tmp_dn5 = assign30180_e36945_d_n5;
        var_tmp_dn6 = assign30180_e36945_d_n6;
        var_tmp_dn7 = assign30180_e36945_d_n7;
        var_tmp_dn8 = assign30180_e36945_d_n8;

        let (assign30190_e36967, assign30190_e36967_d_n5, assign30190_e36967_d_n6, assign30190_e36967_d_n7, assign30190_e36967_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard585 == 0.0)) && (var_guard586 != 0.0)) && (var_guard587 == 0.0)) {
        let assign30190_e36962: f64 = (var_vav * var_vbrinvgat_d);
        let assign30190_e36963: f64 = (assign30190_e36962).abs();
        let assign30190_e36965: f64 = (assign30190_e36963).powf(var_pbrgatd_i);
        (assign30190_e36965, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign30190_e36963).powf(var_pbrgatd_i - 1.0) * if assign30190_e36962 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) })) } } else { (assign30190_e36965 * (var_pbrgatd_i * (if assign30190_e36962 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) } / assign30190_e36963))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign30190_e36963).powf(var_pbrgatd_i - 1.0) * if assign30190_e36962 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) })) } } else { (assign30190_e36965 * (var_pbrgatd_i * (if assign30190_e36962 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) } / assign30190_e36963))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign30190_e36963).powf(var_pbrgatd_i - 1.0) * if assign30190_e36962 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) })) } } else { (assign30190_e36965 * (var_pbrgatd_i * (if assign30190_e36962 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) } / assign30190_e36963))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign30190_e36963).powf(var_pbrgatd_i - 1.0) * if assign30190_e36962 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) })) } } else { (assign30190_e36965 * (var_pbrgatd_i * (if assign30190_e36962 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) } / assign30190_e36963))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30190_e36967;
        var_tmp_dn5 = assign30190_e36967_d_n5;
        var_tmp_dn6 = assign30190_e36967_d_n6;
        var_tmp_dn7 = assign30190_e36967_d_n7;
        var_tmp_dn8 = assign30190_e36967_d_n8;

        let (assign30200_e36985, assign30200_e36985_d_n5, assign30200_e36985_d_n6, assign30200_e36985_d_n7, assign30200_e36985_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard585 == 0.0)) && (var_guard586 != 0.0)) {
        let assign30200_e36982: f64 = (1.0 - var_tmp);
        let assign30200_e36983: f64 = (1.0 / assign30200_e36982);
        (assign30200_e36983, (-((-var_tmp_dn5) / (assign30200_e36982 * assign30200_e36982))), (-((-var_tmp_dn6) / (assign30200_e36982 * assign30200_e36982))), (-((-var_tmp_dn7) / (assign30200_e36982 * assign30200_e36982))), (-((-var_tmp_dn8) / (assign30200_e36982 * assign30200_e36982))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign30200_e36985;
        var_fbreakdown_dn5 = assign30200_e36985_d_n5;
        var_fbreakdown_dn6 = assign30200_e36985_d_n6;
        var_fbreakdown_dn7 = assign30200_e36985_d_n7;
        var_fbreakdown_dn8 = assign30200_e36985_d_n8;

        let (assign30210_e37008, assign30210_e37008_d_n5, assign30210_e37008_d_n6, assign30210_e37008_d_n7, assign30210_e37008_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) && (var_guard585 == 0.0)) && (var_guard586 == 0.0)) {
        let assign30210_e37002: f64 = (var_alphaav * var_vbrgatd_i);
        let assign30210_e37003: f64 = (var_vav + assign30210_e37002);
        let assign30210_e37005: f64 = (assign30210_e37003 * var_slopegat_d);
        let assign30210_e37006: f64 = (var_fstopgat_d + assign30210_e37005);
        (assign30210_e37006, (assign30210_e37003 * var_slopegat_d_dn5), (assign30210_e37003 * var_slopegat_d_dn6), (assign30210_e37003 * var_slopegat_d_dn7), (assign30210_e37003 * var_slopegat_d_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign30210_e37008;
        var_fbreakdown_dn5 = assign30210_e37008_d_n5;
        var_fbreakdown_dn6 = assign30210_e37008_d_n6;
        var_fbreakdown_dn7 = assign30210_e37008_d_n7;
        var_fbreakdown_dn8 = assign30210_e37008_d_n8;

        let (assign30220_e37027, assign30220_e37027_d_n5, assign30220_e37027_d_n6, assign30220_e37027_d_n7, assign30220_e37027_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard571 == 0.0)) {
        let assign30220_e37018: f64 = (var_id__blk213 + var_isrh);
        let assign30220_e37020: f64 = (assign30220_e37018 + var_itat);
        let assign30220_e37022: f64 = (assign30220_e37020 + var_ibbt);
        let assign30220_e37023: f64 = (p.p29 * assign30220_e37022);
        let assign30220_e37025: f64 = (assign30220_e37023 * var_fbreakdown);
        (assign30220_e37025, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign30220_e37023 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign30220_e37023 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign30220_e37023 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign30220_e37023 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign30220_e37027;
        var_ijungat_dn5 = assign30220_e37027_d_n5;
        var_ijungat_dn6 = assign30220_e37027_d_n6;
        var_ijungat_dn7 = assign30220_e37027_d_n7;
        var_ijungat_dn8 = assign30220_e37027_d_n8;

        let (assign30230_e37043, assign30230_e37043_d_n5, assign30230_e37043_d_n6, assign30230_e37043_d_n7, assign30230_e37043_d_n8,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        let assign30230_e37033: f64 = (var_abdrain_i * var_ijunbot);
        let assign30230_e37036: f64 = (var_lsdrain_i * var_ijunsti);
        let assign30230_e37037: f64 = (assign30230_e37033 + assign30230_e37036);
        let assign30230_e37040: f64 = (var_lgdrain_i * var_ijungat);
        let assign30230_e37041: f64 = (assign30230_e37037 + assign30230_e37040);
        (assign30230_e37041, (((var_abdrain_i * var_ijunbot_dn5) + (var_lsdrain_i * var_ijunsti_dn5)) + (var_lgdrain_i * var_ijungat_dn5)), (((var_abdrain_i * var_ijunbot_dn6) + (var_lsdrain_i * var_ijunsti_dn6)) + (var_lgdrain_i * var_ijungat_dn6)), (((var_abdrain_i * var_ijunbot_dn7) + (var_lsdrain_i * var_ijunsti_dn7)) + (var_lgdrain_i * var_ijungat_dn7)), (((var_abdrain_i * var_ijunbot_dn8) + (var_lsdrain_i * var_ijunsti_dn8)) + (var_lgdrain_i * var_ijungat_dn8)),)
    } else {
        (var_i1, var_i1_dn5, var_i1_dn6, var_i1_dn7, var_i1_dn8,)
    }
};
        var_i1 = assign30230_e37043;
        var_i1_dn5 = assign30230_e37043_d_n5;
        var_i1_dn6 = assign30230_e37043_d_n6;
        var_i1_dn7 = assign30230_e37043_d_n7;
        var_i1_dn8 = assign30230_e37043_d_n8;

        let (assign30240_e37049,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign30240_e37049;

        let (assign30250_e37055,) = {
    if ((var_guard176 != 0.0) && (var_guard193 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign30250_e37055;

        let assign30260_e37067: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard588 = assign30260_e37067;

        let assign30340_e37153: f64 = if var_v2 < var_vmax_d { 1.0 } else { 0.0 };
        var_guard589 = assign30340_e37153;

        let assign30350_e37155: f64 = (-0.5);
        let assign30350_e37158: f64 = (var_v2 * var_phitdinv);
        let assign30350_e37159: f64 = (assign30350_e37155 * assign30350_e37158);
        let assign30350_e37160: f64 = (assign30350_e37159).abs();
        let assign30350_e37162: f64 = if assign30350_e37160 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard590 = assign30350_e37162;

        let (assign30360_e37180,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard588 != 0.0)) && (var_guard589 != 0.0)) && (var_guard590 != 0.0)) {
        let assign30360_e37173: f64 = (-0.5);
        let assign30360_e37176: f64 = (var_v2 * var_phitdinv);
        let assign30360_e37177: f64 = (assign30360_e37173 * assign30360_e37176);
        let assign30360_e37178: f64 = (assign30360_e37177).exp();
        (assign30360_e37178,)
    } else {
        (var_z,)
    }
};
        var_z = assign30360_e37180;

        let assign30370_e37182: f64 = (-0.5);
        let assign30370_e37185: f64 = (var_v2 * var_phitdinv);
        let assign30370_e37186: f64 = (assign30370_e37182 * assign30370_e37185);
        let assign30370_e37188: f64 = if assign30370_e37186 < 0.0 { 1.0 } else { 0.0 };
        var_guard591 = assign30370_e37188;

        let (assign30380_e37243,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard588 != 0.0)) && (var_guard589 != 0.0)) && (var_guard590 == 0.0)) && (var_guard591 != 0.0)) {
        let assign30380_e37204: f64 = (-230.25850929940458);
        let assign30380_e37206: f64 = (-0.5);
        let assign30380_e37209: f64 = (var_v2 * var_phitdinv);
        let assign30380_e37210: f64 = (assign30380_e37206 * assign30380_e37209);
        let assign30380_e37211: f64 = (assign30380_e37204 - assign30380_e37210);
        let assign30380_e37215: f64 = (-230.25850929940458);
        let assign30380_e37217: f64 = (-0.5);
        let assign30380_e37220: f64 = (var_v2 * var_phitdinv);
        let assign30380_e37221: f64 = (assign30380_e37217 * assign30380_e37220);
        let assign30380_e37222: f64 = (assign30380_e37215 - assign30380_e37221);
        let assign30380_e37225: f64 = (-230.25850929940458);
        let assign30380_e37227: f64 = (-0.5);
        let assign30380_e37230: f64 = (var_v2 * var_phitdinv);
        let assign30380_e37231: f64 = (assign30380_e37227 * assign30380_e37230);
        let assign30380_e37232: f64 = (assign30380_e37225 - assign30380_e37231);
        let assign30380_e37234: f64 = (assign30380_e37232 * 0.3333333333333333);
        let assign30380_e37235: f64 = (1.0 + assign30380_e37234);
        let assign30380_e37236: f64 = (assign30380_e37222 * assign30380_e37235);
        let assign30380_e37237: f64 = (0.5 * assign30380_e37236);
        let assign30380_e37238: f64 = (1.0 + assign30380_e37237);
        let assign30380_e37239: f64 = (assign30380_e37211 * assign30380_e37238);
        let assign30380_e37240: f64 = (1.0 + assign30380_e37239);
        let assign30380_e37241: f64 = (1e-100 / assign30380_e37240);
        (assign30380_e37241,)
    } else {
        (var_z,)
    }
};
        var_z = assign30380_e37243;

        let (assign30390_e37296,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard588 != 0.0)) && (var_guard589 != 0.0)) && (var_guard590 == 0.0)) && (var_guard591 == 0.0)) {
        let assign30390_e37260: f64 = (-0.5);
        let assign30390_e37263: f64 = (var_v2 * var_phitdinv);
        let assign30390_e37264: f64 = (assign30390_e37260 * assign30390_e37263);
        let assign30390_e37266: f64 = (assign30390_e37264 - 230.25850929940458);
        let assign30390_e37270: f64 = (-0.5);
        let assign30390_e37273: f64 = (var_v2 * var_phitdinv);
        let assign30390_e37274: f64 = (assign30390_e37270 * assign30390_e37273);
        let assign30390_e37276: f64 = (assign30390_e37274 - 230.25850929940458);
        let assign30390_e37279: f64 = (-0.5);
        let assign30390_e37282: f64 = (var_v2 * var_phitdinv);
        let assign30390_e37283: f64 = (assign30390_e37279 * assign30390_e37282);
        let assign30390_e37285: f64 = (assign30390_e37283 - 230.25850929940458);
        let assign30390_e37287: f64 = (assign30390_e37285 * 0.3333333333333333);
        let assign30390_e37288: f64 = (1.0 + assign30390_e37287);
        let assign30390_e37289: f64 = (assign30390_e37276 * assign30390_e37288);
        let assign30390_e37290: f64 = (0.5 * assign30390_e37289);
        let assign30390_e37291: f64 = (1.0 + assign30390_e37290);
        let assign30390_e37292: f64 = (assign30390_e37266 * assign30390_e37291);
        let assign30390_e37293: f64 = (1.0 + assign30390_e37292);
        let assign30390_e37294: f64 = (1e100 * assign30390_e37293);
        (assign30390_e37294,)
    } else {
        (var_z,)
    }
};
        var_z = assign30390_e37296;

        let (assign30400_e37308,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard588 != 0.0)) && (var_guard589 != 0.0)) {
        let assign30400_e37306: f64 = (1.0 / var_z);
        (assign30400_e37306,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign30400_e37308;

        let (assign30410_e37320,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard588 != 0.0)) && (var_guard589 != 0.0)) {
        let assign30410_e37318: f64 = (var_zinv * var_zinv);
        (assign30410_e37318,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign30410_e37320;

        let (assign30420_e37339,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard588 != 0.0)) && (var_guard589 == 0.0)) {
        let assign30420_e37332: f64 = (var_v2 - var_vmax_d);
        let assign30420_e37334: f64 = (assign30420_e37332 * var_phitdinv);
        let assign30420_e37335: f64 = (1.0 + assign30420_e37334);
        let assign30420_e37337: f64 = (assign30420_e37335 * var_exp_vmax_over_phitd_d);
        (assign30420_e37337,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign30420_e37339;

        let (assign30430_e37351,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard588 != 0.0)) && (var_guard589 == 0.0)) {
        let assign30430_e37349: f64 = (var_idmult).sqrt();
        (assign30430_e37349,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign30430_e37351;

        let (assign30440_e37364,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard588 != 0.0)) && (var_guard589 == 0.0)) {
        let assign30440_e37362: f64 = (1.0 / var_zinv);
        (assign30440_e37362,)
    } else {
        (var_z,)
    }
};
        var_z = assign30440_e37364;

        let (assign30450_e37374,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard588 != 0.0)) {
        let assign30450_e37372: f64 = (var_idmult - 1.0);
        (assign30450_e37372,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign30450_e37374;

        let assign30460_e37377: f64 = if var_v2 > 0.0 { 1.0 } else { 0.0 };
        var_guard592 = assign30460_e37377;

        let (assign30470_e37403,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard588 != 0.0)) && (var_guard592 != 0.0)) {
        let assign30470_e37389: f64 = (2.0 + var_z);
        let assign30470_e37392: f64 = (var_z + 1.0);
        let assign30470_e37395: f64 = (var_z + 3.0);
        let assign30470_e37396: f64 = (assign30470_e37392 * assign30470_e37395);
        let assign30470_e37397: f64 = (assign30470_e37396).sqrt();
        let assign30470_e37398: f64 = (assign30470_e37389 + assign30470_e37397);
        let assign30470_e37399: f64 = (assign30470_e37398).ln();
        let assign30470_e37400: f64 = (var_phitd * assign30470_e37399);
        let assign30470_e37401: f64 = (2.0 * assign30470_e37400);
        (assign30470_e37401,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign30470_e37403;

        let (assign30480_e37437,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard588 != 0.0)) && (var_guard592 == 0.0)) {
        let assign30480_e37413: f64 = (-var_v2);
        let assign30480_e37418: f64 = (2.0 * var_zinv);
        let assign30480_e37420: f64 = (assign30480_e37418 + 1.0);
        let assign30480_e37423: f64 = (1.0 + var_zinv);
        let assign30480_e37427: f64 = (3.0 * var_zinv);
        let assign30480_e37428: f64 = (1.0 + assign30480_e37427);
        let assign30480_e37429: f64 = (assign30480_e37423 * assign30480_e37428);
        let assign30480_e37430: f64 = (assign30480_e37429).sqrt();
        let assign30480_e37431: f64 = (assign30480_e37420 + assign30480_e37430);
        let assign30480_e37432: f64 = (assign30480_e37431).ln();
        let assign30480_e37433: f64 = (var_phitd * assign30480_e37432);
        let assign30480_e37434: f64 = (2.0 * assign30480_e37433);
        let assign30480_e37435: f64 = (assign30480_e37413 + assign30480_e37434);
        (assign30480_e37435,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign30480_e37437;

        let (assign30490_e37447,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard588 != 0.0)) {
        let assign30490_e37445: f64 = (var_vbimin_d - var_two_psistar);
        (assign30490_e37445,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign30490_e37447;

        let (assign30500_e37474,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard588 != 0.0)) {
        let assign30500_e37456: f64 = (var_v2 + var_vjlim);
        let assign30500_e37459: f64 = (var_v2 - var_vjlim);
        let assign30500_e37462: f64 = (var_v2 - var_vjlim);
        let assign30500_e37463: f64 = (assign30500_e37459 * assign30500_e37462);
        let assign30500_e37466: f64 = (4.0 * var_phitd);
        let assign30500_e37468: f64 = (assign30500_e37466 * var_phitd);
        let assign30500_e37469: f64 = (assign30500_e37463 + assign30500_e37468);
        let assign30500_e37470: f64 = (assign30500_e37469).sqrt();
        let assign30500_e37471: f64 = (assign30500_e37456 - assign30500_e37470);
        let assign30500_e37472: f64 = (0.5 * assign30500_e37471);
        (assign30500_e37472,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign30500_e37474;

        let (assign30510_e37501,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard588 != 0.0)) {
        let assign30510_e37483: f64 = (var_v2 + var_vbbtlim_d);
        let assign30510_e37486: f64 = (var_v2 - var_vbbtlim_d);
        let assign30510_e37489: f64 = (var_v2 - var_vbbtlim_d);
        let assign30510_e37490: f64 = (assign30510_e37486 * assign30510_e37489);
        let assign30510_e37493: f64 = (4.0 * var_phitr);
        let assign30510_e37495: f64 = (assign30510_e37493 * var_phitr);
        let assign30510_e37496: f64 = (assign30510_e37490 + assign30510_e37495);
        let assign30510_e37497: f64 = (assign30510_e37496).sqrt();
        let assign30510_e37498: f64 = (assign30510_e37483 - assign30510_e37497);
        let assign30510_e37499: f64 = (0.5 * assign30510_e37498);
        (assign30510_e37499,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign30510_e37501;

        let (assign30520_e37528,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard588 != 0.0)) {
        let assign30520_e37510: f64 = var_v2;
        let assign30520_e37513: f64 = var_v2;
        let assign30520_e37516: f64 = var_v2;
        let assign30520_e37517: f64 = (assign30520_e37513 * assign30520_e37516);
        let assign30520_e37520: f64 = (4.0 * 1e-6);
        let assign30520_e37522: f64 = (assign30520_e37520 * 1e-6);
        let assign30520_e37523: f64 = (assign30520_e37517 + assign30520_e37522);
        let assign30520_e37524: f64 = (assign30520_e37523).sqrt();
        let assign30520_e37525: f64 = (assign30520_e37510 - assign30520_e37524);
        let assign30520_e37526: f64 = (0.5 * assign30520_e37525);
        (assign30520_e37526,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign30520_e37528;

        let assign30530_e37531: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard593 = assign30530_e37531;

        let (assign30540_e37539, assign30540_e37539_d_n5, assign30540_e37539_d_n6, assign30540_e37539_d_n7, assign30540_e37539_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign30540_e37539;
        var_ijunbot_dn5 = assign30540_e37539_d_n5;
        var_ijunbot_dn6 = assign30540_e37539_d_n6;
        var_ijunbot_dn7 = assign30540_e37539_d_n7;
        var_ijunbot_dn8 = assign30540_e37539_d_n8;

        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_guard585_slot = var_guard585;
        *var_guard586_slot = var_guard586;
        *var_guard587_slot = var_guard587;
        *var_guard588_slot = var_guard588;
        *var_guard589_slot = var_guard589;
        *var_guard590_slot = var_guard590;
        *var_guard591_slot = var_guard591;
        *var_guard592_slot = var_guard592;
        *var_guard593_slot = var_guard593;
        *var_i1_slot = var_i1;
        *var_i1_dn5_slot = var_i1_dn5;
        *var_i1_dn6_slot = var_i1_dn6;
        *var_i1_dn7_slot = var_i1_dn7;
        *var_i1_dn8_slot = var_i1_dn8;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
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
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_two_psistar_slot = var_two_psistar;
        *var_vav_slot = var_vav;
        *var_vbbt_slot = var_vbbt;
        *var_vjlim_slot = var_vjlim;
        *var_vjsrh_slot = var_vjsrh;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_61(
        var_atatbot_d: f64,
        var_btatpartbot_d: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_ftdbot_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard593: f64,
        var_idmult: f64,
        var_idsatbot_d: f64,
        var_one_minus_pbot_d: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_perfc: f64,
        var_two_psistar: f64,
        var_vbibot_d: f64,
        var_vbirbotinv_d: f64,
        var_vjsrh: f64,
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
        var_guard594_slot: &mut f64,
        var_guard595_slot: &mut f64,
        var_guard596_slot: &mut f64,
        var_guard597_slot: &mut f64,
        var_guard598_slot: &mut f64,
        var_guard599_slot: &mut f64,
        var_guard600_slot: &mut f64,
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
        let mut var_guard594: f64 = *var_guard594_slot;
        let mut var_guard595: f64 = *var_guard595_slot;
        let mut var_guard596: f64 = *var_guard596_slot;
        let mut var_guard597: f64 = *var_guard597_slot;
        let mut var_guard598: f64 = *var_guard598_slot;
        let mut var_guard599: f64 = *var_guard599_slot;
        let mut var_guard600: f64 = *var_guard600_slot;
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

        let (assign30550_e37550,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) {
        let assign30550_e37548: f64 = (var_idsatbot_d * var_idmult);
        (assign30550_e37548,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign30550_e37550;

        let assign30560_e37557: f64 = if ((var_csrhbotd_i == 0.0) && (var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard594 = assign30560_e37557;

        let (assign30570_e37568, assign30570_e37568_d_n5, assign30570_e37568_d_n6, assign30570_e37568_d_n7, assign30570_e37568_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard594 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign30570_e37568;
        var_isrh_dn5 = assign30570_e37568_d_n5;
        var_isrh_dn6 = assign30570_e37568_d_n6;
        var_isrh_dn7 = assign30570_e37568_d_n7;
        var_isrh_dn8 = assign30570_e37568_d_n8;

        let (assign30580_e37582,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard594 == 0.0)) {
        let assign30580_e37580: f64 = (var_vbibot_d - var_vjsrh);
        (assign30580_e37580,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign30580_e37582;

        let (assign30590_e37601,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard594 == 0.0)) {
        let assign30590_e37596: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign30590_e37597: f64 = (1.0 - assign30590_e37596);
        let assign30590_e37598: f64 = (assign30590_e37597).sqrt();
        let assign30590_e37599: f64 = (1.0 - assign30590_e37598);
        (assign30590_e37599,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign30590_e37601;

        let assign30600_e37604: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard595 = assign30600_e37604;

        let (assign30610_e37618,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard594 == 0.0)) && (var_guard595 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign30610_e37618;

        let (assign30620_e37650,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard594 == 0.0)) && (var_guard595 == 0.0)) {
        let assign30620_e37633: f64 = (var_wsrhstep * var_wsrhstep);
        let assign30620_e37635: f64 = (var_wsrhstep).ln();
        let assign30620_e37636: f64 = (assign30620_e37633 * assign30620_e37635);
        let assign30620_e37639: f64 = (1.0 - var_wsrhstep);
        let assign30620_e37640: f64 = (assign30620_e37636 / assign30620_e37639);
        let assign30620_e37642: f64 = (assign30620_e37640 + var_wsrhstep);
        let assign30620_e37646: f64 = (2.0 * var_pbotd_i);
        let assign30620_e37647: f64 = (1.0 - assign30620_e37646);
        let assign30620_e37648: f64 = (assign30620_e37642 * assign30620_e37647);
        (assign30620_e37648,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign30620_e37650;

        let (assign30630_e37664,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard594 == 0.0)) {
        let assign30630_e37662: f64 = (var_wsrhstep + var_dwsrh);
        (assign30630_e37662,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign30630_e37664;

        let assign30640_e37667: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard596 = assign30640_e37667;

        let (assign30650_e37684, assign30650_e37684_d_n5, assign30650_e37684_d_n6, assign30650_e37684_d_n7, assign30650_e37684_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard594 == 0.0)) && (var_guard596 != 0.0)) {
        let assign30650_e37681: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign30650_e37682: f64 = (assign30650_e37681).sqrt();
        (assign30650_e37682, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30650_e37684;
        var_tmp_dn5 = assign30650_e37684_d_n5;
        var_tmp_dn6 = assign30650_e37684_d_n6;
        var_tmp_dn7 = assign30650_e37684_d_n7;
        var_tmp_dn8 = assign30650_e37684_d_n8;

        let (assign30660_e37703, assign30660_e37703_d_n5, assign30660_e37703_d_n6, assign30660_e37703_d_n7, assign30660_e37703_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard594 == 0.0)) && (var_guard596 == 0.0)) {
        let assign30660_e37699: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign30660_e37701: f64 = (assign30660_e37699).powf(var_pbotd_i);
        (assign30660_e37701, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30660_e37703;
        var_tmp_dn5 = assign30660_e37703_d_n5;
        var_tmp_dn6 = assign30660_e37703_d_n6;
        var_tmp_dn7 = assign30660_e37703_d_n7;
        var_tmp_dn8 = assign30660_e37703_d_n8;

        let (assign30670_e37717, assign30670_e37717_d_n5, assign30670_e37717_d_n6, assign30670_e37717_d_n7, assign30670_e37717_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard594 == 0.0)) {
        let assign30670_e37715: f64 = (var_wdepnulrbot_d * var_tmp);
        (assign30670_e37715, (var_wdepnulrbot_d * var_tmp_dn5), (var_wdepnulrbot_d * var_tmp_dn6), (var_wdepnulrbot_d * var_tmp_dn7), (var_wdepnulrbot_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign30670_e37717;
        var_wdep_dn5 = assign30670_e37717_d_n5;
        var_wdep_dn6 = assign30670_e37717_d_n6;
        var_wdep_dn7 = assign30670_e37717_d_n7;
        var_wdep_dn8 = assign30670_e37717_d_n8;

        let (assign30680_e37735, assign30680_e37735_d_n5, assign30680_e37735_d_n6, assign30680_e37735_d_n7, assign30680_e37735_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard594 == 0.0)) {
        let assign30680_e37730: f64 = (var_zinv - 1.0);
        let assign30680_e37732: f64 = (assign30680_e37730 * var_wdep);
        let assign30680_e37733: f64 = (var_ftdbot_d * assign30680_e37732);
        (assign30680_e37733, (var_ftdbot_d * (assign30680_e37730 * var_wdep_dn5)), (var_ftdbot_d * (assign30680_e37730 * var_wdep_dn6)), (var_ftdbot_d * (assign30680_e37730 * var_wdep_dn7)), (var_ftdbot_d * (assign30680_e37730 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign30680_e37735;
        var_asrh_dn5 = assign30680_e37735_d_n5;
        var_asrh_dn6 = assign30680_e37735_d_n6;
        var_asrh_dn7 = assign30680_e37735_d_n7;
        var_asrh_dn8 = assign30680_e37735_d_n8;

        let (assign30690_e37751, assign30690_e37751_d_n5, assign30690_e37751_d_n6, assign30690_e37751_d_n7, assign30690_e37751_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard594 == 0.0)) {
        let assign30690_e37748: f64 = (var_asrh * var_wsrh);
        let assign30690_e37749: f64 = (var_csrhbotd_i * assign30690_e37748);
        (assign30690_e37749, (var_csrhbotd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign30690_e37751;
        var_isrh_dn5 = assign30690_e37751_d_n5;
        var_isrh_dn6 = assign30690_e37751_d_n6;
        var_isrh_dn7 = assign30690_e37751_d_n7;
        var_isrh_dn8 = assign30690_e37751_d_n8;

        let assign30700_e37754: f64 = if var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard597 = assign30700_e37754;

        let (assign30710_e37765, assign30710_e37765_d_n5, assign30710_e37765_d_n6, assign30710_e37765_d_n7, assign30710_e37765_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign30710_e37765;
        var_itat_dn5 = assign30710_e37765_d_n5;
        var_itat_dn6 = assign30710_e37765_d_n6;
        var_itat_dn7 = assign30710_e37765_d_n7;
        var_itat_dn8 = assign30710_e37765_d_n8;

        let (assign30720_e37783, assign30720_e37783_d_n5, assign30720_e37783_d_n6, assign30720_e37783_d_n7, assign30720_e37783_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) {
        let assign30720_e37778: f64 = (var_wdep * var_one_minus_pbot_d);
        let assign30720_e37780: f64 = (assign30720_e37778 / var_vbi_minus_vjsrh);
        let assign30720_e37781: f64 = (var_btatpartbot_d * assign30720_e37780);
        (assign30720_e37781, (var_btatpartbot_d * ((var_wdep_dn5 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn6 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn7 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn8 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign30720_e37783;
        var_btat_dn5 = assign30720_e37783_d_n5;
        var_btat_dn6 = assign30720_e37783_d_n6;
        var_btat_dn7 = assign30720_e37783_d_n7;
        var_btat_dn8 = assign30720_e37783_d_n8;

        let (assign30730_e37799, assign30730_e37799_d_n5, assign30730_e37799_d_n6, assign30730_e37799_d_n7, assign30730_e37799_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) {
        let assign30730_e37795: f64 = (0.666666666666667 * var_atatbot_d);
        let assign30730_e37797: f64 = (assign30730_e37795 / var_btat);
        (assign30730_e37797, (-((assign30730_e37795 * var_btat_dn5) / (var_btat * var_btat))), (-((assign30730_e37795 * var_btat_dn6) / (var_btat * var_btat))), (-((assign30730_e37795 * var_btat_dn7) / (var_btat * var_btat))), (-((assign30730_e37795 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign30730_e37799;
        var_twoatatoverthreebtat_dn5 = assign30730_e37799_d_n5;
        var_twoatatoverthreebtat_dn6 = assign30730_e37799_d_n6;
        var_twoatatoverthreebtat_dn7 = assign30730_e37799_d_n7;
        var_twoatatoverthreebtat_dn8 = assign30730_e37799_d_n8;

        let (assign30740_e37813, assign30740_e37813_d_n5, assign30740_e37813_d_n6, assign30740_e37813_d_n7, assign30740_e37813_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) {
        let assign30740_e37811: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign30740_e37811, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign30740_e37813;
        var_umaxbeforelimiting_dn5 = assign30740_e37813_d_n5;
        var_umaxbeforelimiting_dn6 = assign30740_e37813_d_n6;
        var_umaxbeforelimiting_dn7 = assign30740_e37813_d_n7;
        var_umaxbeforelimiting_dn8 = assign30740_e37813_d_n8;

        let (assign30750_e37834, assign30750_e37834_d_n5, assign30750_e37834_d_n6, assign30750_e37834_d_n7, assign30750_e37834_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) {
        let assign30750_e37825: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign30750_e37828: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign30750_e37830: f64 = (assign30750_e37828 + 1.0);
        let assign30750_e37831: f64 = (assign30750_e37825 / assign30750_e37830);
        let assign30750_e37832: f64 = (assign30750_e37831).sqrt();
        (assign30750_e37832, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign30750_e37830) - (assign30750_e37825 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign30750_e37830 * assign30750_e37830)) / (2.0 * assign30750_e37832)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign30750_e37830) - (assign30750_e37825 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign30750_e37830 * assign30750_e37830)) / (2.0 * assign30750_e37832)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign30750_e37830) - (assign30750_e37825 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign30750_e37830 * assign30750_e37830)) / (2.0 * assign30750_e37832)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign30750_e37830) - (assign30750_e37825 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign30750_e37830 * assign30750_e37830)) / (2.0 * assign30750_e37832)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign30750_e37834;
        var_umax_dn5 = assign30750_e37834_d_n5;
        var_umax_dn6 = assign30750_e37834_d_n6;
        var_umax_dn7 = assign30750_e37834_d_n7;
        var_umax_dn8 = assign30750_e37834_d_n8;

        let (assign30760_e37847, assign30760_e37847_d_n5, assign30760_e37847_d_n6, assign30760_e37847_d_n7, assign30760_e37847_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) {
        let assign30760_e37845: f64 = (var_umax).sqrt();
        (assign30760_e37845, (var_umax_dn5 / (2.0 * assign30760_e37845)), (var_umax_dn6 / (2.0 * assign30760_e37845)), (var_umax_dn7 / (2.0 * assign30760_e37845)), (var_umax_dn8 / (2.0 * assign30760_e37845)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign30760_e37847;
        var_sqrtumax_dn5 = assign30760_e37847_d_n5;
        var_sqrtumax_dn6 = assign30760_e37847_d_n6;
        var_sqrtumax_dn7 = assign30760_e37847_d_n7;
        var_sqrtumax_dn8 = assign30760_e37847_d_n8;

        let (assign30770_e37861, assign30770_e37861_d_n5, assign30770_e37861_d_n6, assign30770_e37861_d_n7, assign30770_e37861_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) {
        let assign30770_e37859: f64 = (var_umax * var_sqrtumax);
        (assign30770_e37859, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign30770_e37861;
        var_umaxpoweronepointfive_dn5 = assign30770_e37861_d_n5;
        var_umaxpoweronepointfive_dn6 = assign30770_e37861_d_n6;
        var_umaxpoweronepointfive_dn7 = assign30770_e37861_d_n7;
        var_umaxpoweronepointfive_dn8 = assign30770_e37861_d_n8;

        let assign30780_e37863: f64 = (-var_pbotd_i);
        let assign30780_e37865: f64 = (assign30780_e37863 * var_one_over_one_minus_pbot_d);
        let assign30780_e37867: f64 = (-1.0);
        let assign30780_e37868: f64 = if assign30780_e37865 == assign30780_e37867 { 1.0 } else { 0.0 };
        var_guard598 = assign30780_e37868;

        let (assign30790_e37888, assign30790_e37888_d_n5, assign30790_e37888_d_n6, assign30790_e37888_d_n7, assign30790_e37888_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) && (var_guard598 != 0.0)) {
        let assign30790_e37884: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign30790_e37885: f64 = (1.0 + assign30790_e37884);
        let assign30790_e37886: f64 = (1.0 / assign30790_e37885);
        (assign30790_e37886, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign30790_e37885 * assign30790_e37885))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign30790_e37885 * assign30790_e37885))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign30790_e37885 * assign30790_e37885))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign30790_e37885 * assign30790_e37885))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign30790_e37888;
        var_wgamma_dn5 = assign30790_e37888_d_n5;
        var_wgamma_dn6 = assign30790_e37888_d_n6;
        var_wgamma_dn7 = assign30790_e37888_d_n7;
        var_wgamma_dn8 = assign30790_e37888_d_n8;

        let (assign30800_e37912, assign30800_e37912_d_n5, assign30800_e37912_d_n6, assign30800_e37912_d_n7, assign30800_e37912_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) && (var_guard598 == 0.0)) {
        let assign30800_e37904: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign30800_e37905: f64 = (1.0 + assign30800_e37904);
        let assign30800_e37907: f64 = (-var_pbotd_i);
        let assign30800_e37909: f64 = (assign30800_e37907 * var_one_over_one_minus_pbot_d);
        let assign30800_e37910: f64 = (assign30800_e37905).powf(assign30800_e37909);
        (assign30800_e37910, if 0.0 == 0.0 && ((assign30800_e37909) as f64).is_finite() && ((assign30800_e37909) as f64).fract() == 0.0 { if assign30800_e37909 == 0.0 { 0.0 } else { (assign30800_e37909 * ((assign30800_e37905).powf(assign30800_e37909 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign30800_e37910 * (assign30800_e37909 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign30800_e37905))) }, if 0.0 == 0.0 && ((assign30800_e37909) as f64).is_finite() && ((assign30800_e37909) as f64).fract() == 0.0 { if assign30800_e37909 == 0.0 { 0.0 } else { (assign30800_e37909 * ((assign30800_e37905).powf(assign30800_e37909 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign30800_e37910 * (assign30800_e37909 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign30800_e37905))) }, if 0.0 == 0.0 && ((assign30800_e37909) as f64).is_finite() && ((assign30800_e37909) as f64).fract() == 0.0 { if assign30800_e37909 == 0.0 { 0.0 } else { (assign30800_e37909 * ((assign30800_e37905).powf(assign30800_e37909 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign30800_e37910 * (assign30800_e37909 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign30800_e37905))) }, if 0.0 == 0.0 && ((assign30800_e37909) as f64).is_finite() && ((assign30800_e37909) as f64).fract() == 0.0 { if assign30800_e37909 == 0.0 { 0.0 } else { (assign30800_e37909 * ((assign30800_e37905).powf(assign30800_e37909 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign30800_e37910 * (assign30800_e37909 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign30800_e37905))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign30800_e37912;
        var_wgamma_dn5 = assign30800_e37912_d_n5;
        var_wgamma_dn6 = assign30800_e37912_d_n6;
        var_wgamma_dn7 = assign30800_e37912_d_n7;
        var_wgamma_dn8 = assign30800_e37912_d_n8;

        let (assign30810_e37930, assign30810_e37930_d_n5, assign30810_e37930_d_n6, assign30810_e37930_d_n7, assign30810_e37930_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) {
        let assign30810_e37924: f64 = (var_wsrh * var_wgamma);
        let assign30810_e37927: f64 = (var_wsrh + var_wgamma);
        let assign30810_e37928: f64 = (assign30810_e37924 / assign30810_e37927);
        (assign30810_e37928, ((((var_wsrh * var_wgamma_dn5) * assign30810_e37927) - (assign30810_e37924 * var_wgamma_dn5)) / (assign30810_e37927 * assign30810_e37927)), ((((var_wsrh * var_wgamma_dn6) * assign30810_e37927) - (assign30810_e37924 * var_wgamma_dn6)) / (assign30810_e37927 * assign30810_e37927)), ((((var_wsrh * var_wgamma_dn7) * assign30810_e37927) - (assign30810_e37924 * var_wgamma_dn7)) / (assign30810_e37927 * assign30810_e37927)), ((((var_wsrh * var_wgamma_dn8) * assign30810_e37927) - (assign30810_e37924 * var_wgamma_dn8)) / (assign30810_e37927 * assign30810_e37927)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign30810_e37930;
        var_wtat_dn5 = assign30810_e37930_d_n5;
        var_wtat_dn6 = assign30810_e37930_d_n6;
        var_wtat_dn7 = assign30810_e37930_d_n7;
        var_wtat_dn8 = assign30810_e37930_d_n8;

        let (assign30820_e37947, assign30820_e37947_d_n5, assign30820_e37947_d_n6, assign30820_e37947_d_n7, assign30820_e37947_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) {
        let assign30820_e37943: f64 = (var_btat / var_sqrtumax);
        let assign30820_e37944: f64 = (0.375 * assign30820_e37943);
        let assign30820_e37945: f64 = (assign30820_e37944).sqrt();
        (assign30820_e37945, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign30820_e37945)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign30820_e37945)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign30820_e37945)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign30820_e37945)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign30820_e37947;
        var_ktat_dn5 = assign30820_e37947_d_n5;
        var_ktat_dn6 = assign30820_e37947_d_n6;
        var_ktat_dn7 = assign30820_e37947_d_n7;
        var_ktat_dn8 = assign30820_e37947_d_n8;

        let (assign30830_e37965, assign30830_e37965_d_n5, assign30830_e37965_d_n6, assign30830_e37965_d_n7, assign30830_e37965_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) {
        let assign30830_e37960: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign30830_e37961: f64 = (2.0 * assign30830_e37960);
        let assign30830_e37963: f64 = (assign30830_e37961 - var_umax);
        (assign30830_e37963, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign30830_e37965;
        var_ltat_dn5 = assign30830_e37965_d_n5;
        var_ltat_dn6 = assign30830_e37965_d_n6;
        var_ltat_dn7 = assign30830_e37965_d_n7;
        var_ltat_dn8 = assign30830_e37965_d_n8;

        let (assign30840_e37991, assign30840_e37991_d_n5, assign30840_e37991_d_n6, assign30840_e37991_d_n7, assign30840_e37991_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) {
        let assign30840_e37977: f64 = (var_atatbot_d * var_twoatatoverthreebtat);
        let assign30840_e37979: f64 = (assign30840_e37977 * var_sqrtumax);
        let assign30840_e37982: f64 = (var_atatbot_d * var_umax);
        let assign30840_e37983: f64 = (assign30840_e37979 - assign30840_e37982);
        let assign30840_e37987: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign30840_e37988: f64 = (0.5 * assign30840_e37987);
        let assign30840_e37989: f64 = (assign30840_e37983 + assign30840_e37988);
        (assign30840_e37989, (((((var_atatbot_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign30840_e37977 * var_sqrtumax_dn5)) - (var_atatbot_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign30840_e37977 * var_sqrtumax_dn6)) - (var_atatbot_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign30840_e37977 * var_sqrtumax_dn7)) - (var_atatbot_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign30840_e37977 * var_sqrtumax_dn8)) - (var_atatbot_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign30840_e37991;
        var_mtat_dn5 = assign30840_e37991_d_n5;
        var_mtat_dn6 = assign30840_e37991_d_n6;
        var_mtat_dn7 = assign30840_e37991_d_n7;
        var_mtat_dn8 = assign30840_e37991_d_n8;

        let (assign30850_e38007, assign30850_e38007_d_n5, assign30850_e38007_d_n6, assign30850_e38007_d_n7, assign30850_e38007_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) {
        let assign30850_e38003: f64 = (var_ltat - 1.0);
        let assign30850_e38005: f64 = (assign30850_e38003 * var_ktat);
        (assign30850_e38005, ((var_ltat_dn5 * var_ktat) + (assign30850_e38003 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign30850_e38003 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign30850_e38003 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign30850_e38003 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign30850_e38007;
        var_xerfc_dn5 = assign30850_e38007_d_n5;
        var_xerfc_dn6 = assign30850_e38007_d_n6;
        var_xerfc_dn7 = assign30850_e38007_d_n7;
        var_xerfc_dn8 = assign30850_e38007_d_n8;

        let (assign30860_e38021, assign30860_e38021_d_n5, assign30860_e38021_d_n6, assign30860_e38021_d_n7, assign30860_e38021_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) {
        let assign30860_e38019: f64 = (var_xerfc * var_xerfc);
        (assign30860_e38019, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign30860_e38021;
        var_ysq_dn5 = assign30860_e38021_d_n5;
        var_ysq_dn6 = assign30860_e38021_d_n6;
        var_ysq_dn7 = assign30860_e38021_d_n7;
        var_ysq_dn8 = assign30860_e38021_d_n8;

        let assign30870_e38024: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard599 = assign30870_e38024;

        let (assign30880_e38044, assign30880_e38044_d_n5, assign30880_e38044_d_n6, assign30880_e38044_d_n7, assign30880_e38044_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) && (var_guard599 != 0.0)) {
        let assign30880_e38040: f64 = (var_perfc * var_xerfc);
        let assign30880_e38041: f64 = (1.0 + assign30880_e38040);
        let assign30880_e38042: f64 = (1.0 / assign30880_e38041);
        (assign30880_e38042, (-((var_perfc * var_xerfc_dn5) / (assign30880_e38041 * assign30880_e38041))), (-((var_perfc * var_xerfc_dn6) / (assign30880_e38041 * assign30880_e38041))), (-((var_perfc * var_xerfc_dn7) / (assign30880_e38041 * assign30880_e38041))), (-((var_perfc * var_xerfc_dn8) / (assign30880_e38041 * assign30880_e38041))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign30880_e38044;
        var_terfc_dn5 = assign30880_e38044_d_n5;
        var_terfc_dn6 = assign30880_e38044_d_n6;
        var_terfc_dn7 = assign30880_e38044_d_n7;
        var_terfc_dn8 = assign30880_e38044_d_n8;

        let (assign30890_e38065, assign30890_e38065_d_n5, assign30890_e38065_d_n6, assign30890_e38065_d_n7, assign30890_e38065_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) && (var_guard599 == 0.0)) {
        let assign30890_e38061: f64 = (var_perfc * var_xerfc);
        let assign30890_e38062: f64 = (1.0 - assign30890_e38061);
        let assign30890_e38063: f64 = (1.0 / assign30890_e38062);
        (assign30890_e38063, (-((-(var_perfc * var_xerfc_dn5)) / (assign30890_e38062 * assign30890_e38062))), (-((-(var_perfc * var_xerfc_dn6)) / (assign30890_e38062 * assign30890_e38062))), (-((-(var_perfc * var_xerfc_dn7)) / (assign30890_e38062 * assign30890_e38062))), (-((-(var_perfc * var_xerfc_dn8)) / (assign30890_e38062 * assign30890_e38062))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign30890_e38065;
        var_terfc_dn5 = assign30890_e38065_d_n5;
        var_terfc_dn6 = assign30890_e38065_d_n6;
        var_terfc_dn7 = assign30890_e38065_d_n7;
        var_terfc_dn8 = assign30890_e38065_d_n8;

        let assign30900_e38067: f64 = (-var_ysq);
        let assign30900_e38069: f64 = (assign30900_e38067 + var_mtat);
        let assign30900_e38071: f64 = (-230.25850929940458);
        let assign30900_e38072: f64 = if assign30900_e38069 > assign30900_e38071 { 1.0 } else { 0.0 };
        var_guard600 = assign30900_e38072;

        let (assign30910_e38090, assign30910_e38090_d_n5, assign30910_e38090_d_n6, assign30910_e38090_d_n7, assign30910_e38090_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) && (var_guard600 != 0.0)) {
        let assign30910_e38085: f64 = (-var_ysq);
        let assign30910_e38087: f64 = (assign30910_e38085 + var_mtat);
        let assign30910_e38088: f64 = (assign30910_e38087).exp();
        (assign30910_e38088, (assign30910_e38088 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign30910_e38088 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign30910_e38088 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign30910_e38088 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30910_e38090;
        var_tmp_dn5 = assign30910_e38090_d_n5;
        var_tmp_dn6 = assign30910_e38090_d_n6;
        var_tmp_dn7 = assign30910_e38090_d_n7;
        var_tmp_dn8 = assign30910_e38090_d_n8;

        let (assign30920_e38139, assign30920_e38139_d_n5, assign30920_e38139_d_n6, assign30920_e38139_d_n7, assign30920_e38139_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) && (var_guard600 == 0.0)) {
        let assign30920_e38106: f64 = (-230.25850929940458);
        let assign30920_e38108: f64 = (-var_ysq);
        let assign30920_e38110: f64 = (assign30920_e38108 + var_mtat);
        let assign30920_e38111: f64 = (assign30920_e38106 - assign30920_e38110);
        let assign30920_e38115: f64 = (-230.25850929940458);
        let assign30920_e38117: f64 = (-var_ysq);
        let assign30920_e38119: f64 = (assign30920_e38117 + var_mtat);
        let assign30920_e38120: f64 = (assign30920_e38115 - assign30920_e38119);
        let assign30920_e38123: f64 = (-230.25850929940458);
        let assign30920_e38125: f64 = (-var_ysq);
        let assign30920_e38127: f64 = (assign30920_e38125 + var_mtat);
        let assign30920_e38128: f64 = (assign30920_e38123 - assign30920_e38127);
        let assign30920_e38130: f64 = (assign30920_e38128 * 0.3333333333333333);
        let assign30920_e38131: f64 = (1.0 + assign30920_e38130);
        let assign30920_e38132: f64 = (assign30920_e38120 * assign30920_e38131);
        let assign30920_e38133: f64 = (0.5 * assign30920_e38132);
        let assign30920_e38134: f64 = (1.0 + assign30920_e38133);
        let assign30920_e38135: f64 = (assign30920_e38111 * assign30920_e38134);
        let assign30920_e38136: f64 = (1.0 + assign30920_e38135);
        let assign30920_e38137: f64 = (1e-100 / assign30920_e38136);
        (assign30920_e38137, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign30920_e38134) + (assign30920_e38111 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign30920_e38131) + (assign30920_e38120 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign30920_e38136 * assign30920_e38136))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign30920_e38134) + (assign30920_e38111 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign30920_e38131) + (assign30920_e38120 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign30920_e38136 * assign30920_e38136))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign30920_e38134) + (assign30920_e38111 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign30920_e38131) + (assign30920_e38120 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign30920_e38136 * assign30920_e38136))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign30920_e38134) + (assign30920_e38111 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign30920_e38131) + (assign30920_e38120 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign30920_e38136 * assign30920_e38136))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30920_e38139;
        var_tmp_dn5 = assign30920_e38139_d_n5;
        var_tmp_dn6 = assign30920_e38139_d_n6;
        var_tmp_dn7 = assign30920_e38139_d_n7;
        var_tmp_dn8 = assign30920_e38139_d_n8;

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
        *var_guard594_slot = var_guard594;
        *var_guard595_slot = var_guard595;
        *var_guard596_slot = var_guard596;
        *var_guard597_slot = var_guard597;
        *var_guard598_slot = var_guard598;
        *var_guard599_slot = var_guard599;
        *var_guard600_slot = var_guard600;
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

    pub(super) fn stamp_transient_block_62(
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
        var_csrhstid_i: f64,
        var_ctatbotd_i: f64,
        var_ctatstid_i: f64,
        var_fbbtbot_d: f64,
        var_fstopbot_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard593: f64,
        var_guard597: f64,
        var_idmult: f64,
        var_idsatsti_d: f64,
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
        var_pstid_i: f64,
        var_slopebot_d: f64,
        var_terfc: f64,
        var_terfc_dn5: f64,
        var_terfc_dn6: f64,
        var_terfc_dn7: f64,
        var_terfc_dn8: f64,
        var_two_psistar: f64,
        var_v2: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirbotd_i: f64,
        var_vbirbotinv_d: f64,
        var_vbisti_d: f64,
        var_vbrbotd_i: f64,
        var_vbrinvbot_d: f64,
        var_vjsrh: f64,
        var_wdepnulrinvbot_d: f64,
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
        var_guard601_slot: &mut f64,
        var_guard602_slot: &mut f64,
        var_guard603_slot: &mut f64,
        var_guard604_slot: &mut f64,
        var_guard605_slot: &mut f64,
        var_guard606_slot: &mut f64,
        var_guard607_slot: &mut f64,
        var_guard608_slot: &mut f64,
        var_guard609_slot: &mut f64,
        var_guard610_slot: &mut f64,
        var_guard611_slot: &mut f64,
        var_guard612_slot: &mut f64,
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
        let mut var_guard601: f64 = *var_guard601_slot;
        let mut var_guard602: f64 = *var_guard602_slot;
        let mut var_guard603: f64 = *var_guard603_slot;
        let mut var_guard604: f64 = *var_guard604_slot;
        let mut var_guard605: f64 = *var_guard605_slot;
        let mut var_guard606: f64 = *var_guard606_slot;
        let mut var_guard607: f64 = *var_guard607_slot;
        let mut var_guard608: f64 = *var_guard608_slot;
        let mut var_guard609: f64 = *var_guard609_slot;
        let mut var_guard610: f64 = *var_guard610_slot;
        let mut var_guard611: f64 = *var_guard611_slot;
        let mut var_guard612: f64 = *var_guard612_slot;
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
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;

        let (assign30930_e38169, assign30930_e38169_d_n5, assign30930_e38169_d_n6, assign30930_e38169_d_n7, assign30930_e38169_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) {
        let assign30930_e38151: f64 = (0.29214664 * var_terfc);
        let assign30930_e38155: f64 = (var_terfc * var_terfc);
        let assign30930_e38156: f64 = (var_berfc * assign30930_e38155);
        let assign30930_e38157: f64 = (assign30930_e38151 + assign30930_e38156);
        let assign30930_e38161: f64 = (var_terfc * var_terfc);
        let assign30930_e38163: f64 = (assign30930_e38161 * var_terfc);
        let assign30930_e38164: f64 = (var_cerfc * assign30930_e38163);
        let assign30930_e38165: f64 = (assign30930_e38157 + assign30930_e38164);
        let assign30930_e38167: f64 = (assign30930_e38165 * var_tmp);
        (assign30930_e38167, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign30930_e38161 * var_terfc_dn5)))) * var_tmp) + (assign30930_e38165 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign30930_e38161 * var_terfc_dn6)))) * var_tmp) + (assign30930_e38165 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign30930_e38161 * var_terfc_dn7)))) * var_tmp) + (assign30930_e38165 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign30930_e38161 * var_terfc_dn8)))) * var_tmp) + (assign30930_e38165 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign30930_e38169;
        var_erfcpos_dn5 = assign30930_e38169_d_n5;
        var_erfcpos_dn6 = assign30930_e38169_d_n6;
        var_erfcpos_dn7 = assign30930_e38169_d_n7;
        var_erfcpos_dn8 = assign30930_e38169_d_n8;

        let assign30940_e38172: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard601 = assign30940_e38172;

        let (assign30950_e38186, assign30950_e38186_d_n5, assign30950_e38186_d_n6, assign30950_e38186_d_n7, assign30950_e38186_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) && (var_guard601 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign30950_e38186;
        var_erfctimesexpmtat_dn5 = assign30950_e38186_d_n5;
        var_erfctimesexpmtat_dn6 = assign30950_e38186_d_n6;
        var_erfctimesexpmtat_dn7 = assign30950_e38186_d_n7;
        var_erfctimesexpmtat_dn8 = assign30950_e38186_d_n8;

        let assign30960_e38189: f64 = (-230.25850929940458);
        let assign30960_e38190: f64 = if var_mtat > assign30960_e38189 { 1.0 } else { 0.0 };
        var_guard602 = assign30960_e38190;

        let (assign30970_e38208, assign30970_e38208_d_n5, assign30970_e38208_d_n6, assign30970_e38208_d_n7, assign30970_e38208_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) && (var_guard601 == 0.0)) && (var_guard602 != 0.0)) {
        let assign30970_e38206: f64 = (var_mtat).exp();
        (assign30970_e38206, (assign30970_e38206 * var_mtat_dn5), (assign30970_e38206 * var_mtat_dn6), (assign30970_e38206 * var_mtat_dn7), (assign30970_e38206 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30970_e38208;
        var_tmp_dn5 = assign30970_e38208_d_n5;
        var_tmp_dn6 = assign30970_e38208_d_n6;
        var_tmp_dn7 = assign30970_e38208_d_n7;
        var_tmp_dn8 = assign30970_e38208_d_n8;

        let (assign30980_e38251, assign30980_e38251_d_n5, assign30980_e38251_d_n6, assign30980_e38251_d_n7, assign30980_e38251_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) && (var_guard601 == 0.0)) && (var_guard602 == 0.0)) {
        let assign30980_e38227: f64 = (-230.25850929940458);
        let assign30980_e38229: f64 = (assign30980_e38227 - var_mtat);
        let assign30980_e38233: f64 = (-230.25850929940458);
        let assign30980_e38235: f64 = (assign30980_e38233 - var_mtat);
        let assign30980_e38238: f64 = (-230.25850929940458);
        let assign30980_e38240: f64 = (assign30980_e38238 - var_mtat);
        let assign30980_e38242: f64 = (assign30980_e38240 * 0.3333333333333333);
        let assign30980_e38243: f64 = (1.0 + assign30980_e38242);
        let assign30980_e38244: f64 = (assign30980_e38235 * assign30980_e38243);
        let assign30980_e38245: f64 = (0.5 * assign30980_e38244);
        let assign30980_e38246: f64 = (1.0 + assign30980_e38245);
        let assign30980_e38247: f64 = (assign30980_e38229 * assign30980_e38246);
        let assign30980_e38248: f64 = (1.0 + assign30980_e38247);
        let assign30980_e38249: f64 = (1e-100 / assign30980_e38248);
        (assign30980_e38249, (-((1e-100 * (((-var_mtat_dn5) * assign30980_e38246) + (assign30980_e38229 * (0.5 * (((-var_mtat_dn5) * assign30980_e38243) + (assign30980_e38235 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign30980_e38248 * assign30980_e38248))), (-((1e-100 * (((-var_mtat_dn6) * assign30980_e38246) + (assign30980_e38229 * (0.5 * (((-var_mtat_dn6) * assign30980_e38243) + (assign30980_e38235 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign30980_e38248 * assign30980_e38248))), (-((1e-100 * (((-var_mtat_dn7) * assign30980_e38246) + (assign30980_e38229 * (0.5 * (((-var_mtat_dn7) * assign30980_e38243) + (assign30980_e38235 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign30980_e38248 * assign30980_e38248))), (-((1e-100 * (((-var_mtat_dn8) * assign30980_e38246) + (assign30980_e38229 * (0.5 * (((-var_mtat_dn8) * assign30980_e38243) + (assign30980_e38235 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign30980_e38248 * assign30980_e38248))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30980_e38251;
        var_tmp_dn5 = assign30980_e38251_d_n5;
        var_tmp_dn6 = assign30980_e38251_d_n6;
        var_tmp_dn7 = assign30980_e38251_d_n7;
        var_tmp_dn8 = assign30980_e38251_d_n8;

        let (assign30990_e38270, assign30990_e38270_d_n5, assign30990_e38270_d_n6, assign30990_e38270_d_n7, assign30990_e38270_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) && (var_guard601 == 0.0)) {
        let assign30990_e38266: f64 = (2.0 * var_tmp);
        let assign30990_e38268: f64 = (assign30990_e38266 - var_erfcpos);
        (assign30990_e38268, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign30990_e38270;
        var_erfctimesexpmtat_dn5 = assign30990_e38270_d_n5;
        var_erfctimesexpmtat_dn6 = assign30990_e38270_d_n6;
        var_erfctimesexpmtat_dn7 = assign30990_e38270_d_n7;
        var_erfctimesexpmtat_dn8 = assign30990_e38270_d_n8;

        let (assign31000_e38290, assign31000_e38290_d_n5, assign31000_e38290_d_n6, assign31000_e38290_d_n7, assign31000_e38290_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) {
        let assign31000_e38282: f64 = (1.772453850905516 * 0.5);
        let assign31000_e38285: f64 = (var_atatbot_d * var_erfctimesexpmtat);
        let assign31000_e38287: f64 = (assign31000_e38285 / var_ktat);
        let assign31000_e38288: f64 = (assign31000_e38282 * assign31000_e38287);
        (assign31000_e38288, (assign31000_e38282 * ((((var_atatbot_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign31000_e38285 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign31000_e38282 * ((((var_atatbot_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign31000_e38285 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign31000_e38282 * ((((var_atatbot_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign31000_e38285 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign31000_e38282 * ((((var_atatbot_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign31000_e38285 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign31000_e38290;
        var_gammamax_dn5 = assign31000_e38290_d_n5;
        var_gammamax_dn6 = assign31000_e38290_d_n6;
        var_gammamax_dn7 = assign31000_e38290_d_n7;
        var_gammamax_dn8 = assign31000_e38290_d_n8;

        let (assign31010_e38308, assign31010_e38308_d_n5, assign31010_e38308_d_n6, assign31010_e38308_d_n7, assign31010_e38308_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard597 == 0.0)) {
        let assign31010_e38303: f64 = (var_asrh * var_gammamax);
        let assign31010_e38305: f64 = (assign31010_e38303 * var_wtat);
        let assign31010_e38306: f64 = (var_ctatbotd_i * assign31010_e38305);
        (assign31010_e38306, (var_ctatbotd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign31010_e38303 * var_wtat_dn5))), (var_ctatbotd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign31010_e38303 * var_wtat_dn6))), (var_ctatbotd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign31010_e38303 * var_wtat_dn7))), (var_ctatbotd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign31010_e38303 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign31010_e38308;
        var_itat_dn5 = assign31010_e38308_d_n5;
        var_itat_dn6 = assign31010_e38308_d_n6;
        var_itat_dn7 = assign31010_e38308_d_n7;
        var_itat_dn8 = assign31010_e38308_d_n8;

        let assign31020_e38311: f64 = if var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard603 = assign31020_e38311;

        let (assign31030_e38322, assign31030_e38322_d_n5, assign31030_e38322_d_n6, assign31030_e38322_d_n7, assign31030_e38322_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard603 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign31030_e38322;
        var_ibbt_dn5 = assign31030_e38322_d_n5;
        var_ibbt_dn6 = assign31030_e38322_d_n6;
        var_ibbt_dn7 = assign31030_e38322_d_n7;
        var_ibbt_dn8 = assign31030_e38322_d_n8;

        let assign31040_e38325: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard604 = assign31040_e38325;

        let (assign31050_e38344, assign31050_e38344_d_n5, assign31050_e38344_d_n6, assign31050_e38344_d_n7, assign31050_e38344_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard603 == 0.0)) && (var_guard604 != 0.0)) {
        let assign31050_e38339: f64 = (var_vbirbotd_i - var_vbbt);
        let assign31050_e38341: f64 = (assign31050_e38339 * var_vbirbotinv_d);
        let assign31050_e38342: f64 = (assign31050_e38341).sqrt();
        (assign31050_e38342, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31050_e38344;
        var_tmp_dn5 = assign31050_e38344_d_n5;
        var_tmp_dn6 = assign31050_e38344_d_n6;
        var_tmp_dn7 = assign31050_e38344_d_n7;
        var_tmp_dn8 = assign31050_e38344_d_n8;

        let (assign31060_e38365, assign31060_e38365_d_n5, assign31060_e38365_d_n6, assign31060_e38365_d_n7, assign31060_e38365_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard603 == 0.0)) && (var_guard604 == 0.0)) {
        let assign31060_e38359: f64 = (var_vbirbotd_i - var_vbbt);
        let assign31060_e38361: f64 = (assign31060_e38359 * var_vbirbotinv_d);
        let assign31060_e38363: f64 = (assign31060_e38361).powf(var_pbotd_i);
        (assign31060_e38363, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31060_e38365;
        var_tmp_dn5 = assign31060_e38365_d_n5;
        var_tmp_dn6 = assign31060_e38365_d_n6;
        var_tmp_dn7 = assign31060_e38365_d_n7;
        var_tmp_dn8 = assign31060_e38365_d_n8;

        let (assign31070_e38385, assign31070_e38385_d_n5, assign31070_e38385_d_n6, assign31070_e38385_d_n7, assign31070_e38385_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard603 == 0.0)) {
        let assign31070_e38378: f64 = (var_vbirbotd_i - var_vbbt);
        let assign31070_e38380: f64 = (assign31070_e38378 * var_wdepnulrinvbot_d);
        let assign31070_e38382: f64 = (assign31070_e38380 / var_tmp);
        let assign31070_e38383: f64 = (var_one_over_one_minus_pbot_d * assign31070_e38382);
        (assign31070_e38383, (var_one_over_one_minus_pbot_d * (-((assign31070_e38380 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign31070_e38380 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign31070_e38380 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign31070_e38380 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign31070_e38385;
        var_fmaxr_dn5 = assign31070_e38385_d_n5;
        var_fmaxr_dn6 = assign31070_e38385_d_n6;
        var_fmaxr_dn7 = assign31070_e38385_d_n7;
        var_fmaxr_dn8 = assign31070_e38385_d_n8;

        let assign31080_e38387: f64 = (-var_fbbtbot_d);
        let assign31080_e38389: f64 = (assign31080_e38387 / var_fmaxr);
        let assign31080_e38390: f64 = (assign31080_e38389).abs();
        let assign31080_e38392: f64 = if assign31080_e38390 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard605 = assign31080_e38392;

        let (assign31090_e38410, assign31090_e38410_d_n5, assign31090_e38410_d_n6, assign31090_e38410_d_n7, assign31090_e38410_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard603 == 0.0)) && (var_guard605 != 0.0)) {
        let assign31090_e38405: f64 = (-var_fbbtbot_d);
        let assign31090_e38407: f64 = (assign31090_e38405 / var_fmaxr);
        let assign31090_e38408: f64 = (assign31090_e38407).exp();
        (assign31090_e38408, (assign31090_e38408 * (-((assign31090_e38405 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign31090_e38408 * (-((assign31090_e38405 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign31090_e38408 * (-((assign31090_e38405 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign31090_e38408 * (-((assign31090_e38405 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31090_e38410;
        var_tmp_dn5 = assign31090_e38410_d_n5;
        var_tmp_dn6 = assign31090_e38410_d_n6;
        var_tmp_dn7 = assign31090_e38410_d_n7;
        var_tmp_dn8 = assign31090_e38410_d_n8;

        let assign31100_e38412: f64 = (-var_fbbtbot_d);
        let assign31100_e38414: f64 = (assign31100_e38412 / var_fmaxr);
        let assign31100_e38416: f64 = if assign31100_e38414 < 0.0 { 1.0 } else { 0.0 };
        var_guard606 = assign31100_e38416;

        let (assign31110_e38467, assign31110_e38467_d_n5, assign31110_e38467_d_n6, assign31110_e38467_d_n7, assign31110_e38467_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard603 == 0.0)) && (var_guard605 == 0.0)) && (var_guard606 != 0.0)) {
        let assign31110_e38434: f64 = (-230.25850929940458);
        let assign31110_e38436: f64 = (-var_fbbtbot_d);
        let assign31110_e38438: f64 = (assign31110_e38436 / var_fmaxr);
        let assign31110_e38439: f64 = (assign31110_e38434 - assign31110_e38438);
        let assign31110_e38443: f64 = (-230.25850929940458);
        let assign31110_e38445: f64 = (-var_fbbtbot_d);
        let assign31110_e38447: f64 = (assign31110_e38445 / var_fmaxr);
        let assign31110_e38448: f64 = (assign31110_e38443 - assign31110_e38447);
        let assign31110_e38451: f64 = (-230.25850929940458);
        let assign31110_e38453: f64 = (-var_fbbtbot_d);
        let assign31110_e38455: f64 = (assign31110_e38453 / var_fmaxr);
        let assign31110_e38456: f64 = (assign31110_e38451 - assign31110_e38455);
        let assign31110_e38458: f64 = (assign31110_e38456 * 0.3333333333333333);
        let assign31110_e38459: f64 = (1.0 + assign31110_e38458);
        let assign31110_e38460: f64 = (assign31110_e38448 * assign31110_e38459);
        let assign31110_e38461: f64 = (0.5 * assign31110_e38460);
        let assign31110_e38462: f64 = (1.0 + assign31110_e38461);
        let assign31110_e38463: f64 = (assign31110_e38439 * assign31110_e38462);
        let assign31110_e38464: f64 = (1.0 + assign31110_e38463);
        let assign31110_e38465: f64 = (1e-100 / assign31110_e38464);
        (assign31110_e38465, (-((1e-100 * (((-(-((assign31110_e38436 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign31110_e38462) + (assign31110_e38439 * (0.5 * (((-(-((assign31110_e38445 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign31110_e38459) + (assign31110_e38448 * ((-(-((assign31110_e38453 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign31110_e38464 * assign31110_e38464))), (-((1e-100 * (((-(-((assign31110_e38436 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign31110_e38462) + (assign31110_e38439 * (0.5 * (((-(-((assign31110_e38445 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign31110_e38459) + (assign31110_e38448 * ((-(-((assign31110_e38453 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign31110_e38464 * assign31110_e38464))), (-((1e-100 * (((-(-((assign31110_e38436 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign31110_e38462) + (assign31110_e38439 * (0.5 * (((-(-((assign31110_e38445 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign31110_e38459) + (assign31110_e38448 * ((-(-((assign31110_e38453 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign31110_e38464 * assign31110_e38464))), (-((1e-100 * (((-(-((assign31110_e38436 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign31110_e38462) + (assign31110_e38439 * (0.5 * (((-(-((assign31110_e38445 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign31110_e38459) + (assign31110_e38448 * ((-(-((assign31110_e38453 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign31110_e38464 * assign31110_e38464))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31110_e38467;
        var_tmp_dn5 = assign31110_e38467_d_n5;
        var_tmp_dn6 = assign31110_e38467_d_n6;
        var_tmp_dn7 = assign31110_e38467_d_n7;
        var_tmp_dn8 = assign31110_e38467_d_n8;

        let (assign31120_e38516, assign31120_e38516_d_n5, assign31120_e38516_d_n6, assign31120_e38516_d_n7, assign31120_e38516_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard603 == 0.0)) && (var_guard605 == 0.0)) && (var_guard606 == 0.0)) {
        let assign31120_e38486: f64 = (-var_fbbtbot_d);
        let assign31120_e38488: f64 = (assign31120_e38486 / var_fmaxr);
        let assign31120_e38490: f64 = (assign31120_e38488 - 230.25850929940458);
        let assign31120_e38494: f64 = (-var_fbbtbot_d);
        let assign31120_e38496: f64 = (assign31120_e38494 / var_fmaxr);
        let assign31120_e38498: f64 = (assign31120_e38496 - 230.25850929940458);
        let assign31120_e38501: f64 = (-var_fbbtbot_d);
        let assign31120_e38503: f64 = (assign31120_e38501 / var_fmaxr);
        let assign31120_e38505: f64 = (assign31120_e38503 - 230.25850929940458);
        let assign31120_e38507: f64 = (assign31120_e38505 * 0.3333333333333333);
        let assign31120_e38508: f64 = (1.0 + assign31120_e38507);
        let assign31120_e38509: f64 = (assign31120_e38498 * assign31120_e38508);
        let assign31120_e38510: f64 = (0.5 * assign31120_e38509);
        let assign31120_e38511: f64 = (1.0 + assign31120_e38510);
        let assign31120_e38512: f64 = (assign31120_e38490 * assign31120_e38511);
        let assign31120_e38513: f64 = (1.0 + assign31120_e38512);
        let assign31120_e38514: f64 = (1e100 * assign31120_e38513);
        (assign31120_e38514, (1e100 * (((-((assign31120_e38486 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign31120_e38511) + (assign31120_e38490 * (0.5 * (((-((assign31120_e38494 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign31120_e38508) + (assign31120_e38498 * ((-((assign31120_e38501 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign31120_e38486 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign31120_e38511) + (assign31120_e38490 * (0.5 * (((-((assign31120_e38494 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign31120_e38508) + (assign31120_e38498 * ((-((assign31120_e38501 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign31120_e38486 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign31120_e38511) + (assign31120_e38490 * (0.5 * (((-((assign31120_e38494 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign31120_e38508) + (assign31120_e38498 * ((-((assign31120_e38501 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign31120_e38486 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign31120_e38511) + (assign31120_e38490 * (0.5 * (((-((assign31120_e38494 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign31120_e38508) + (assign31120_e38498 * ((-((assign31120_e38501 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31120_e38516;
        var_tmp_dn5 = assign31120_e38516_d_n5;
        var_tmp_dn6 = assign31120_e38516_d_n6;
        var_tmp_dn7 = assign31120_e38516_d_n7;
        var_tmp_dn8 = assign31120_e38516_d_n8;

        let (assign31130_e38536, assign31130_e38536_d_n5, assign31130_e38536_d_n6, assign31130_e38536_d_n7, assign31130_e38536_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard603 == 0.0)) {
        let assign31130_e38529: f64 = (var_v2 * var_fmaxr);
        let assign31130_e38531: f64 = (assign31130_e38529 * var_fmaxr);
        let assign31130_e38533: f64 = (assign31130_e38531 * var_tmp);
        let assign31130_e38534: f64 = (var_cbbtbotd_i * assign31130_e38533);
        (assign31130_e38534, (var_cbbtbotd_i * (((((var_v2 * var_fmaxr_dn5) * var_fmaxr) + (assign31130_e38529 * var_fmaxr_dn5)) * var_tmp) + (assign31130_e38531 * var_tmp_dn5))), (var_cbbtbotd_i * (((((var_v2 * var_fmaxr_dn6) * var_fmaxr) + (assign31130_e38529 * var_fmaxr_dn6)) * var_tmp) + (assign31130_e38531 * var_tmp_dn6))), (var_cbbtbotd_i * (((((var_v2 * var_fmaxr_dn7) * var_fmaxr) + (assign31130_e38529 * var_fmaxr_dn7)) * var_tmp) + (assign31130_e38531 * var_tmp_dn7))), (var_cbbtbotd_i * (((((var_v2 * var_fmaxr_dn8) * var_fmaxr) + (assign31130_e38529 * var_fmaxr_dn8)) * var_tmp) + (assign31130_e38531 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign31130_e38536;
        var_ibbt_dn5 = assign31130_e38536_d_n5;
        var_ibbt_dn6 = assign31130_e38536_d_n6;
        var_ibbt_dn7 = assign31130_e38536_d_n7;
        var_ibbt_dn8 = assign31130_e38536_d_n8;

        let assign31140_e38539: f64 = if var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard607 = assign31140_e38539;

        let (assign31150_e38550, assign31150_e38550_d_n5, assign31150_e38550_d_n6, assign31150_e38550_d_n7, assign31150_e38550_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard607 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign31150_e38550;
        var_fbreakdown_dn5 = assign31150_e38550_d_n5;
        var_fbreakdown_dn6 = assign31150_e38550_d_n6;
        var_fbreakdown_dn7 = assign31150_e38550_d_n7;
        var_fbreakdown_dn8 = assign31150_e38550_d_n8;

        let assign31160_e38553: f64 = (-var_alphaav);
        let assign31160_e38555: f64 = (assign31160_e38553 * var_vbrbotd_i);
        let assign31160_e38556: f64 = if var_vav > assign31160_e38555 { 1.0 } else { 0.0 };
        var_guard608 = assign31160_e38556;

        let assign31170_e38559: f64 = if var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard609 = assign31170_e38559;

        let (assign31180_e38589, assign31180_e38589_d_n5, assign31180_e38589_d_n6, assign31180_e38589_d_n7, assign31180_e38589_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard607 == 0.0)) && (var_guard608 != 0.0)) && (var_guard609 != 0.0)) {
        let assign31180_e38575: f64 = (var_vav * var_vbrinvbot_d);
        let assign31180_e38578: f64 = (var_vav * var_vbrinvbot_d);
        let assign31180_e38579: f64 = (assign31180_e38575 * assign31180_e38578);
        let assign31180_e38582: f64 = (var_vav * var_vbrinvbot_d);
        let assign31180_e38583: f64 = (assign31180_e38579 * assign31180_e38582);
        let assign31180_e38586: f64 = (var_vav * var_vbrinvbot_d);
        let assign31180_e38587: f64 = (assign31180_e38583 * assign31180_e38586);
        (assign31180_e38587, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31180_e38589;
        var_tmp_dn5 = assign31180_e38589_d_n5;
        var_tmp_dn6 = assign31180_e38589_d_n6;
        var_tmp_dn7 = assign31180_e38589_d_n7;
        var_tmp_dn8 = assign31180_e38589_d_n8;

        let (assign31190_e38611, assign31190_e38611_d_n5, assign31190_e38611_d_n6, assign31190_e38611_d_n7, assign31190_e38611_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard607 == 0.0)) && (var_guard608 != 0.0)) && (var_guard609 == 0.0)) {
        let assign31190_e38606: f64 = (var_vav * var_vbrinvbot_d);
        let assign31190_e38607: f64 = (assign31190_e38606).abs();
        let assign31190_e38609: f64 = (assign31190_e38607).powf(var_pbrbotd_i);
        (assign31190_e38609, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31190_e38611;
        var_tmp_dn5 = assign31190_e38611_d_n5;
        var_tmp_dn6 = assign31190_e38611_d_n6;
        var_tmp_dn7 = assign31190_e38611_d_n7;
        var_tmp_dn8 = assign31190_e38611_d_n8;

        let (assign31200_e38629, assign31200_e38629_d_n5, assign31200_e38629_d_n6, assign31200_e38629_d_n7, assign31200_e38629_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard607 == 0.0)) && (var_guard608 != 0.0)) {
        let assign31200_e38626: f64 = (1.0 - var_tmp);
        let assign31200_e38627: f64 = (1.0 / assign31200_e38626);
        (assign31200_e38627, (-((-var_tmp_dn5) / (assign31200_e38626 * assign31200_e38626))), (-((-var_tmp_dn6) / (assign31200_e38626 * assign31200_e38626))), (-((-var_tmp_dn7) / (assign31200_e38626 * assign31200_e38626))), (-((-var_tmp_dn8) / (assign31200_e38626 * assign31200_e38626))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign31200_e38629;
        var_fbreakdown_dn5 = assign31200_e38629_d_n5;
        var_fbreakdown_dn6 = assign31200_e38629_d_n6;
        var_fbreakdown_dn7 = assign31200_e38629_d_n7;
        var_fbreakdown_dn8 = assign31200_e38629_d_n8;

        let (assign31210_e38652, assign31210_e38652_d_n5, assign31210_e38652_d_n6, assign31210_e38652_d_n7, assign31210_e38652_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) && (var_guard607 == 0.0)) && (var_guard608 == 0.0)) {
        let assign31210_e38646: f64 = (var_alphaav * var_vbrbotd_i);
        let assign31210_e38647: f64 = (var_vav + assign31210_e38646);
        let assign31210_e38649: f64 = (assign31210_e38647 * var_slopebot_d);
        let assign31210_e38650: f64 = (var_fstopbot_d + assign31210_e38649);
        (assign31210_e38650, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign31210_e38652;
        var_fbreakdown_dn5 = assign31210_e38652_d_n5;
        var_fbreakdown_dn6 = assign31210_e38652_d_n6;
        var_fbreakdown_dn7 = assign31210_e38652_d_n7;
        var_fbreakdown_dn8 = assign31210_e38652_d_n8;

        let (assign31220_e38671, assign31220_e38671_d_n5, assign31220_e38671_d_n6, assign31220_e38671_d_n7, assign31220_e38671_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard593 == 0.0)) {
        let assign31220_e38662: f64 = (var_id__blk213 + var_isrh);
        let assign31220_e38664: f64 = (assign31220_e38662 + var_itat);
        let assign31220_e38666: f64 = (assign31220_e38664 + var_ibbt);
        let assign31220_e38667: f64 = (p.p29 * assign31220_e38666);
        let assign31220_e38669: f64 = (assign31220_e38667 * var_fbreakdown);
        (assign31220_e38669, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign31220_e38667 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign31220_e38667 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign31220_e38667 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign31220_e38667 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign31220_e38671;
        var_ijunbot_dn5 = assign31220_e38671_d_n5;
        var_ijunbot_dn6 = assign31220_e38671_d_n6;
        var_ijunbot_dn7 = assign31220_e38671_d_n7;
        var_ijunbot_dn8 = assign31220_e38671_d_n8;

        let assign31230_e38674: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard610 = assign31230_e38674;

        let (assign31240_e38682, assign31240_e38682_d_n5, assign31240_e38682_d_n6, assign31240_e38682_d_n7, assign31240_e38682_d_n8,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign31240_e38682;
        var_ijunsti_dn5 = assign31240_e38682_d_n5;
        var_ijunsti_dn6 = assign31240_e38682_d_n6;
        var_ijunsti_dn7 = assign31240_e38682_d_n7;
        var_ijunsti_dn8 = assign31240_e38682_d_n8;

        let (assign31250_e38693,) = {
    if (((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) {
        let assign31250_e38691: f64 = (var_idsatsti_d * var_idmult);
        (assign31250_e38691,)
    } else {
        (var_id__blk213,)
    }
};
        var_id__blk213 = assign31250_e38693;

        let assign31260_e38700: f64 = if ((var_csrhstid_i == 0.0) && (var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard611 = assign31260_e38700;

        let (assign31270_e38711, assign31270_e38711_d_n5, assign31270_e38711_d_n6, assign31270_e38711_d_n7, assign31270_e38711_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard611 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign31270_e38711;
        var_isrh_dn5 = assign31270_e38711_d_n5;
        var_isrh_dn6 = assign31270_e38711_d_n6;
        var_isrh_dn7 = assign31270_e38711_d_n7;
        var_isrh_dn8 = assign31270_e38711_d_n8;

        let (assign31280_e38725,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard611 == 0.0)) {
        let assign31280_e38723: f64 = (var_vbisti_d - var_vjsrh);
        (assign31280_e38723,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign31280_e38725;

        let (assign31290_e38744,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard611 == 0.0)) {
        let assign31290_e38739: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign31290_e38740: f64 = (1.0 - assign31290_e38739);
        let assign31290_e38741: f64 = (assign31290_e38740).sqrt();
        let assign31290_e38742: f64 = (1.0 - assign31290_e38741);
        (assign31290_e38742,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign31290_e38744;

        let assign31300_e38747: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard612 = assign31300_e38747;

        let (assign31310_e38761,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard611 == 0.0)) && (var_guard612 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign31310_e38761;

        let (assign31320_e38793,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard611 == 0.0)) && (var_guard612 == 0.0)) {
        let assign31320_e38776: f64 = (var_wsrhstep * var_wsrhstep);
        let assign31320_e38778: f64 = (var_wsrhstep).ln();
        let assign31320_e38779: f64 = (assign31320_e38776 * assign31320_e38778);
        let assign31320_e38782: f64 = (1.0 - var_wsrhstep);
        let assign31320_e38783: f64 = (assign31320_e38779 / assign31320_e38782);
        let assign31320_e38785: f64 = (assign31320_e38783 + var_wsrhstep);
        let assign31320_e38789: f64 = (2.0 * var_pstid_i);
        let assign31320_e38790: f64 = (1.0 - assign31320_e38789);
        let assign31320_e38791: f64 = (assign31320_e38785 * assign31320_e38790);
        (assign31320_e38791,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign31320_e38793;

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
        *var_guard601_slot = var_guard601;
        *var_guard602_slot = var_guard602;
        *var_guard603_slot = var_guard603;
        *var_guard604_slot = var_guard604;
        *var_guard605_slot = var_guard605;
        *var_guard606_slot = var_guard606;
        *var_guard607_slot = var_guard607;
        *var_guard608_slot = var_guard608;
        *var_guard609_slot = var_guard609;
        *var_guard610_slot = var_guard610;
        *var_guard611_slot = var_guard611;
        *var_guard612_slot = var_guard612;
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
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wsrhstep_slot = var_wsrhstep;
    }

    pub(super) fn stamp_transient_block_63(
        var_atatsti_d: f64,
        var_berfc: f64,
        var_btatpartsti_d: f64,
        var_cerfc: f64,
        var_csrhstid_i: f64,
        var_ctatstid_i: f64,
        var_dwsrh: f64,
        var_ftdsti_d: f64,
        var_guard176: f64,
        var_guard193: f64,
        var_guard610: f64,
        var_guard611: f64,
        var_one_minus_psti_d: f64,
        var_one_over_one_minus_psti_d: f64,
        var_perfc: f64,
        var_pstid_i: f64,
        var_vbi_minus_vjsrh: f64,
        var_vbirstiinv_d: f64,
        var_wdepnulrsti_d: f64,
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
        var_guard613_slot: &mut f64,
        var_guard614_slot: &mut f64,
        var_guard615_slot: &mut f64,
        var_guard616_slot: &mut f64,
        var_guard617_slot: &mut f64,
        var_guard618_slot: &mut f64,
        var_guard619_slot: &mut f64,
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
        let mut var_guard613: f64 = *var_guard613_slot;
        let mut var_guard614: f64 = *var_guard614_slot;
        let mut var_guard615: f64 = *var_guard615_slot;
        let mut var_guard616: f64 = *var_guard616_slot;
        let mut var_guard617: f64 = *var_guard617_slot;
        let mut var_guard618: f64 = *var_guard618_slot;
        let mut var_guard619: f64 = *var_guard619_slot;
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

        let (assign31330_e38807,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard611 == 0.0)) {
        let assign31330_e38805: f64 = (var_wsrhstep + var_dwsrh);
        (assign31330_e38805,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign31330_e38807;

        let assign31340_e38810: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard613 = assign31340_e38810;

        let (assign31350_e38827, assign31350_e38827_d_n5, assign31350_e38827_d_n6, assign31350_e38827_d_n7, assign31350_e38827_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard611 == 0.0)) && (var_guard613 != 0.0)) {
        let assign31350_e38824: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign31350_e38825: f64 = (assign31350_e38824).sqrt();
        (assign31350_e38825, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31350_e38827;
        var_tmp_dn5 = assign31350_e38827_d_n5;
        var_tmp_dn6 = assign31350_e38827_d_n6;
        var_tmp_dn7 = assign31350_e38827_d_n7;
        var_tmp_dn8 = assign31350_e38827_d_n8;

        let (assign31360_e38846, assign31360_e38846_d_n5, assign31360_e38846_d_n6, assign31360_e38846_d_n7, assign31360_e38846_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard611 == 0.0)) && (var_guard613 == 0.0)) {
        let assign31360_e38842: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign31360_e38844: f64 = (assign31360_e38842).powf(var_pstid_i);
        (assign31360_e38844, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31360_e38846;
        var_tmp_dn5 = assign31360_e38846_d_n5;
        var_tmp_dn6 = assign31360_e38846_d_n6;
        var_tmp_dn7 = assign31360_e38846_d_n7;
        var_tmp_dn8 = assign31360_e38846_d_n8;

        let (assign31370_e38860, assign31370_e38860_d_n5, assign31370_e38860_d_n6, assign31370_e38860_d_n7, assign31370_e38860_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard611 == 0.0)) {
        let assign31370_e38858: f64 = (var_wdepnulrsti_d * var_tmp);
        (assign31370_e38858, (var_wdepnulrsti_d * var_tmp_dn5), (var_wdepnulrsti_d * var_tmp_dn6), (var_wdepnulrsti_d * var_tmp_dn7), (var_wdepnulrsti_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign31370_e38860;
        var_wdep_dn5 = assign31370_e38860_d_n5;
        var_wdep_dn6 = assign31370_e38860_d_n6;
        var_wdep_dn7 = assign31370_e38860_d_n7;
        var_wdep_dn8 = assign31370_e38860_d_n8;

        let (assign31380_e38878, assign31380_e38878_d_n5, assign31380_e38878_d_n6, assign31380_e38878_d_n7, assign31380_e38878_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard611 == 0.0)) {
        let assign31380_e38873: f64 = (var_zinv - 1.0);
        let assign31380_e38875: f64 = (assign31380_e38873 * var_wdep);
        let assign31380_e38876: f64 = (var_ftdsti_d * assign31380_e38875);
        (assign31380_e38876, (var_ftdsti_d * (assign31380_e38873 * var_wdep_dn5)), (var_ftdsti_d * (assign31380_e38873 * var_wdep_dn6)), (var_ftdsti_d * (assign31380_e38873 * var_wdep_dn7)), (var_ftdsti_d * (assign31380_e38873 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign31380_e38878;
        var_asrh_dn5 = assign31380_e38878_d_n5;
        var_asrh_dn6 = assign31380_e38878_d_n6;
        var_asrh_dn7 = assign31380_e38878_d_n7;
        var_asrh_dn8 = assign31380_e38878_d_n8;

        let (assign31390_e38894, assign31390_e38894_d_n5, assign31390_e38894_d_n6, assign31390_e38894_d_n7, assign31390_e38894_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard611 == 0.0)) {
        let assign31390_e38891: f64 = (var_asrh * var_wsrh);
        let assign31390_e38892: f64 = (var_csrhstid_i * assign31390_e38891);
        (assign31390_e38892, (var_csrhstid_i * (var_asrh_dn5 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn6 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn7 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign31390_e38894;
        var_isrh_dn5 = assign31390_e38894_d_n5;
        var_isrh_dn6 = assign31390_e38894_d_n6;
        var_isrh_dn7 = assign31390_e38894_d_n7;
        var_isrh_dn8 = assign31390_e38894_d_n8;

        let assign31400_e38897: f64 = if var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard614 = assign31400_e38897;

        let (assign31410_e38908, assign31410_e38908_d_n5, assign31410_e38908_d_n6, assign31410_e38908_d_n7, assign31410_e38908_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign31410_e38908;
        var_itat_dn5 = assign31410_e38908_d_n5;
        var_itat_dn6 = assign31410_e38908_d_n6;
        var_itat_dn7 = assign31410_e38908_d_n7;
        var_itat_dn8 = assign31410_e38908_d_n8;

        let (assign31420_e38926, assign31420_e38926_d_n5, assign31420_e38926_d_n6, assign31420_e38926_d_n7, assign31420_e38926_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) {
        let assign31420_e38921: f64 = (var_wdep * var_one_minus_psti_d);
        let assign31420_e38923: f64 = (assign31420_e38921 / var_vbi_minus_vjsrh);
        let assign31420_e38924: f64 = (var_btatpartsti_d * assign31420_e38923);
        (assign31420_e38924, (var_btatpartsti_d * ((var_wdep_dn5 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn6 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn7 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn8 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign31420_e38926;
        var_btat_dn5 = assign31420_e38926_d_n5;
        var_btat_dn6 = assign31420_e38926_d_n6;
        var_btat_dn7 = assign31420_e38926_d_n7;
        var_btat_dn8 = assign31420_e38926_d_n8;

        let (assign31430_e38942, assign31430_e38942_d_n5, assign31430_e38942_d_n6, assign31430_e38942_d_n7, assign31430_e38942_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) {
        let assign31430_e38938: f64 = (0.666666666666667 * var_atatsti_d);
        let assign31430_e38940: f64 = (assign31430_e38938 / var_btat);
        (assign31430_e38940, (-((assign31430_e38938 * var_btat_dn5) / (var_btat * var_btat))), (-((assign31430_e38938 * var_btat_dn6) / (var_btat * var_btat))), (-((assign31430_e38938 * var_btat_dn7) / (var_btat * var_btat))), (-((assign31430_e38938 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign31430_e38942;
        var_twoatatoverthreebtat_dn5 = assign31430_e38942_d_n5;
        var_twoatatoverthreebtat_dn6 = assign31430_e38942_d_n6;
        var_twoatatoverthreebtat_dn7 = assign31430_e38942_d_n7;
        var_twoatatoverthreebtat_dn8 = assign31430_e38942_d_n8;

        let (assign31440_e38956, assign31440_e38956_d_n5, assign31440_e38956_d_n6, assign31440_e38956_d_n7, assign31440_e38956_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) {
        let assign31440_e38954: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign31440_e38954, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign31440_e38956;
        var_umaxbeforelimiting_dn5 = assign31440_e38956_d_n5;
        var_umaxbeforelimiting_dn6 = assign31440_e38956_d_n6;
        var_umaxbeforelimiting_dn7 = assign31440_e38956_d_n7;
        var_umaxbeforelimiting_dn8 = assign31440_e38956_d_n8;

        let (assign31450_e38977, assign31450_e38977_d_n5, assign31450_e38977_d_n6, assign31450_e38977_d_n7, assign31450_e38977_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) {
        let assign31450_e38968: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign31450_e38971: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign31450_e38973: f64 = (assign31450_e38971 + 1.0);
        let assign31450_e38974: f64 = (assign31450_e38968 / assign31450_e38973);
        let assign31450_e38975: f64 = (assign31450_e38974).sqrt();
        (assign31450_e38975, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign31450_e38973) - (assign31450_e38968 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign31450_e38973 * assign31450_e38973)) / (2.0 * assign31450_e38975)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign31450_e38973) - (assign31450_e38968 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign31450_e38973 * assign31450_e38973)) / (2.0 * assign31450_e38975)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign31450_e38973) - (assign31450_e38968 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign31450_e38973 * assign31450_e38973)) / (2.0 * assign31450_e38975)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign31450_e38973) - (assign31450_e38968 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign31450_e38973 * assign31450_e38973)) / (2.0 * assign31450_e38975)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign31450_e38977;
        var_umax_dn5 = assign31450_e38977_d_n5;
        var_umax_dn6 = assign31450_e38977_d_n6;
        var_umax_dn7 = assign31450_e38977_d_n7;
        var_umax_dn8 = assign31450_e38977_d_n8;

        let (assign31460_e38990, assign31460_e38990_d_n5, assign31460_e38990_d_n6, assign31460_e38990_d_n7, assign31460_e38990_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) {
        let assign31460_e38988: f64 = (var_umax).sqrt();
        (assign31460_e38988, (var_umax_dn5 / (2.0 * assign31460_e38988)), (var_umax_dn6 / (2.0 * assign31460_e38988)), (var_umax_dn7 / (2.0 * assign31460_e38988)), (var_umax_dn8 / (2.0 * assign31460_e38988)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign31460_e38990;
        var_sqrtumax_dn5 = assign31460_e38990_d_n5;
        var_sqrtumax_dn6 = assign31460_e38990_d_n6;
        var_sqrtumax_dn7 = assign31460_e38990_d_n7;
        var_sqrtumax_dn8 = assign31460_e38990_d_n8;

        let (assign31470_e39004, assign31470_e39004_d_n5, assign31470_e39004_d_n6, assign31470_e39004_d_n7, assign31470_e39004_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) {
        let assign31470_e39002: f64 = (var_umax * var_sqrtumax);
        (assign31470_e39002, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign31470_e39004;
        var_umaxpoweronepointfive_dn5 = assign31470_e39004_d_n5;
        var_umaxpoweronepointfive_dn6 = assign31470_e39004_d_n6;
        var_umaxpoweronepointfive_dn7 = assign31470_e39004_d_n7;
        var_umaxpoweronepointfive_dn8 = assign31470_e39004_d_n8;

        let assign31480_e39006: f64 = (-var_pstid_i);
        let assign31480_e39008: f64 = (assign31480_e39006 * var_one_over_one_minus_psti_d);
        let assign31480_e39010: f64 = (-1.0);
        let assign31480_e39011: f64 = if assign31480_e39008 == assign31480_e39010 { 1.0 } else { 0.0 };
        var_guard615 = assign31480_e39011;

        let (assign31490_e39031, assign31490_e39031_d_n5, assign31490_e39031_d_n6, assign31490_e39031_d_n7, assign31490_e39031_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) && (var_guard615 != 0.0)) {
        let assign31490_e39027: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign31490_e39028: f64 = (1.0 + assign31490_e39027);
        let assign31490_e39029: f64 = (1.0 / assign31490_e39028);
        (assign31490_e39029, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign31490_e39028 * assign31490_e39028))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign31490_e39028 * assign31490_e39028))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign31490_e39028 * assign31490_e39028))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign31490_e39028 * assign31490_e39028))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign31490_e39031;
        var_wgamma_dn5 = assign31490_e39031_d_n5;
        var_wgamma_dn6 = assign31490_e39031_d_n6;
        var_wgamma_dn7 = assign31490_e39031_d_n7;
        var_wgamma_dn8 = assign31490_e39031_d_n8;

        let (assign31500_e39055, assign31500_e39055_d_n5, assign31500_e39055_d_n6, assign31500_e39055_d_n7, assign31500_e39055_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) && (var_guard615 == 0.0)) {
        let assign31500_e39047: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign31500_e39048: f64 = (1.0 + assign31500_e39047);
        let assign31500_e39050: f64 = (-var_pstid_i);
        let assign31500_e39052: f64 = (assign31500_e39050 * var_one_over_one_minus_psti_d);
        let assign31500_e39053: f64 = (assign31500_e39048).powf(assign31500_e39052);
        (assign31500_e39053, if 0.0 == 0.0 && ((assign31500_e39052) as f64).is_finite() && ((assign31500_e39052) as f64).fract() == 0.0 { if assign31500_e39052 == 0.0 { 0.0 } else { (assign31500_e39052 * ((assign31500_e39048).powf(assign31500_e39052 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign31500_e39053 * (assign31500_e39052 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign31500_e39048))) }, if 0.0 == 0.0 && ((assign31500_e39052) as f64).is_finite() && ((assign31500_e39052) as f64).fract() == 0.0 { if assign31500_e39052 == 0.0 { 0.0 } else { (assign31500_e39052 * ((assign31500_e39048).powf(assign31500_e39052 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign31500_e39053 * (assign31500_e39052 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign31500_e39048))) }, if 0.0 == 0.0 && ((assign31500_e39052) as f64).is_finite() && ((assign31500_e39052) as f64).fract() == 0.0 { if assign31500_e39052 == 0.0 { 0.0 } else { (assign31500_e39052 * ((assign31500_e39048).powf(assign31500_e39052 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign31500_e39053 * (assign31500_e39052 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign31500_e39048))) }, if 0.0 == 0.0 && ((assign31500_e39052) as f64).is_finite() && ((assign31500_e39052) as f64).fract() == 0.0 { if assign31500_e39052 == 0.0 { 0.0 } else { (assign31500_e39052 * ((assign31500_e39048).powf(assign31500_e39052 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign31500_e39053 * (assign31500_e39052 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign31500_e39048))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign31500_e39055;
        var_wgamma_dn5 = assign31500_e39055_d_n5;
        var_wgamma_dn6 = assign31500_e39055_d_n6;
        var_wgamma_dn7 = assign31500_e39055_d_n7;
        var_wgamma_dn8 = assign31500_e39055_d_n8;

        let (assign31510_e39073, assign31510_e39073_d_n5, assign31510_e39073_d_n6, assign31510_e39073_d_n7, assign31510_e39073_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) {
        let assign31510_e39067: f64 = (var_wsrh * var_wgamma);
        let assign31510_e39070: f64 = (var_wsrh + var_wgamma);
        let assign31510_e39071: f64 = (assign31510_e39067 / assign31510_e39070);
        (assign31510_e39071, ((((var_wsrh * var_wgamma_dn5) * assign31510_e39070) - (assign31510_e39067 * var_wgamma_dn5)) / (assign31510_e39070 * assign31510_e39070)), ((((var_wsrh * var_wgamma_dn6) * assign31510_e39070) - (assign31510_e39067 * var_wgamma_dn6)) / (assign31510_e39070 * assign31510_e39070)), ((((var_wsrh * var_wgamma_dn7) * assign31510_e39070) - (assign31510_e39067 * var_wgamma_dn7)) / (assign31510_e39070 * assign31510_e39070)), ((((var_wsrh * var_wgamma_dn8) * assign31510_e39070) - (assign31510_e39067 * var_wgamma_dn8)) / (assign31510_e39070 * assign31510_e39070)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign31510_e39073;
        var_wtat_dn5 = assign31510_e39073_d_n5;
        var_wtat_dn6 = assign31510_e39073_d_n6;
        var_wtat_dn7 = assign31510_e39073_d_n7;
        var_wtat_dn8 = assign31510_e39073_d_n8;

        let (assign31520_e39090, assign31520_e39090_d_n5, assign31520_e39090_d_n6, assign31520_e39090_d_n7, assign31520_e39090_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) {
        let assign31520_e39086: f64 = (var_btat / var_sqrtumax);
        let assign31520_e39087: f64 = (0.375 * assign31520_e39086);
        let assign31520_e39088: f64 = (assign31520_e39087).sqrt();
        (assign31520_e39088, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign31520_e39088)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign31520_e39088)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign31520_e39088)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign31520_e39088)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign31520_e39090;
        var_ktat_dn5 = assign31520_e39090_d_n5;
        var_ktat_dn6 = assign31520_e39090_d_n6;
        var_ktat_dn7 = assign31520_e39090_d_n7;
        var_ktat_dn8 = assign31520_e39090_d_n8;

        let (assign31530_e39108, assign31530_e39108_d_n5, assign31530_e39108_d_n6, assign31530_e39108_d_n7, assign31530_e39108_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) {
        let assign31530_e39103: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign31530_e39104: f64 = (2.0 * assign31530_e39103);
        let assign31530_e39106: f64 = (assign31530_e39104 - var_umax);
        (assign31530_e39106, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign31530_e39108;
        var_ltat_dn5 = assign31530_e39108_d_n5;
        var_ltat_dn6 = assign31530_e39108_d_n6;
        var_ltat_dn7 = assign31530_e39108_d_n7;
        var_ltat_dn8 = assign31530_e39108_d_n8;

        let (assign31540_e39134, assign31540_e39134_d_n5, assign31540_e39134_d_n6, assign31540_e39134_d_n7, assign31540_e39134_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) {
        let assign31540_e39120: f64 = (var_atatsti_d * var_twoatatoverthreebtat);
        let assign31540_e39122: f64 = (assign31540_e39120 * var_sqrtumax);
        let assign31540_e39125: f64 = (var_atatsti_d * var_umax);
        let assign31540_e39126: f64 = (assign31540_e39122 - assign31540_e39125);
        let assign31540_e39130: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign31540_e39131: f64 = (0.5 * assign31540_e39130);
        let assign31540_e39132: f64 = (assign31540_e39126 + assign31540_e39131);
        (assign31540_e39132, (((((var_atatsti_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign31540_e39120 * var_sqrtumax_dn5)) - (var_atatsti_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign31540_e39120 * var_sqrtumax_dn6)) - (var_atatsti_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign31540_e39120 * var_sqrtumax_dn7)) - (var_atatsti_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign31540_e39120 * var_sqrtumax_dn8)) - (var_atatsti_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign31540_e39134;
        var_mtat_dn5 = assign31540_e39134_d_n5;
        var_mtat_dn6 = assign31540_e39134_d_n6;
        var_mtat_dn7 = assign31540_e39134_d_n7;
        var_mtat_dn8 = assign31540_e39134_d_n8;

        let (assign31550_e39150, assign31550_e39150_d_n5, assign31550_e39150_d_n6, assign31550_e39150_d_n7, assign31550_e39150_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) {
        let assign31550_e39146: f64 = (var_ltat - 1.0);
        let assign31550_e39148: f64 = (assign31550_e39146 * var_ktat);
        (assign31550_e39148, ((var_ltat_dn5 * var_ktat) + (assign31550_e39146 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign31550_e39146 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign31550_e39146 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign31550_e39146 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign31550_e39150;
        var_xerfc_dn5 = assign31550_e39150_d_n5;
        var_xerfc_dn6 = assign31550_e39150_d_n6;
        var_xerfc_dn7 = assign31550_e39150_d_n7;
        var_xerfc_dn8 = assign31550_e39150_d_n8;

        let (assign31560_e39164, assign31560_e39164_d_n5, assign31560_e39164_d_n6, assign31560_e39164_d_n7, assign31560_e39164_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) {
        let assign31560_e39162: f64 = (var_xerfc * var_xerfc);
        (assign31560_e39162, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign31560_e39164;
        var_ysq_dn5 = assign31560_e39164_d_n5;
        var_ysq_dn6 = assign31560_e39164_d_n6;
        var_ysq_dn7 = assign31560_e39164_d_n7;
        var_ysq_dn8 = assign31560_e39164_d_n8;

        let assign31570_e39167: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard616 = assign31570_e39167;

        let (assign31580_e39187, assign31580_e39187_d_n5, assign31580_e39187_d_n6, assign31580_e39187_d_n7, assign31580_e39187_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) && (var_guard616 != 0.0)) {
        let assign31580_e39183: f64 = (var_perfc * var_xerfc);
        let assign31580_e39184: f64 = (1.0 + assign31580_e39183);
        let assign31580_e39185: f64 = (1.0 / assign31580_e39184);
        (assign31580_e39185, (-((var_perfc * var_xerfc_dn5) / (assign31580_e39184 * assign31580_e39184))), (-((var_perfc * var_xerfc_dn6) / (assign31580_e39184 * assign31580_e39184))), (-((var_perfc * var_xerfc_dn7) / (assign31580_e39184 * assign31580_e39184))), (-((var_perfc * var_xerfc_dn8) / (assign31580_e39184 * assign31580_e39184))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign31580_e39187;
        var_terfc_dn5 = assign31580_e39187_d_n5;
        var_terfc_dn6 = assign31580_e39187_d_n6;
        var_terfc_dn7 = assign31580_e39187_d_n7;
        var_terfc_dn8 = assign31580_e39187_d_n8;

        let (assign31590_e39208, assign31590_e39208_d_n5, assign31590_e39208_d_n6, assign31590_e39208_d_n7, assign31590_e39208_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) && (var_guard616 == 0.0)) {
        let assign31590_e39204: f64 = (var_perfc * var_xerfc);
        let assign31590_e39205: f64 = (1.0 - assign31590_e39204);
        let assign31590_e39206: f64 = (1.0 / assign31590_e39205);
        (assign31590_e39206, (-((-(var_perfc * var_xerfc_dn5)) / (assign31590_e39205 * assign31590_e39205))), (-((-(var_perfc * var_xerfc_dn6)) / (assign31590_e39205 * assign31590_e39205))), (-((-(var_perfc * var_xerfc_dn7)) / (assign31590_e39205 * assign31590_e39205))), (-((-(var_perfc * var_xerfc_dn8)) / (assign31590_e39205 * assign31590_e39205))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign31590_e39208;
        var_terfc_dn5 = assign31590_e39208_d_n5;
        var_terfc_dn6 = assign31590_e39208_d_n6;
        var_terfc_dn7 = assign31590_e39208_d_n7;
        var_terfc_dn8 = assign31590_e39208_d_n8;

        let assign31600_e39210: f64 = (-var_ysq);
        let assign31600_e39212: f64 = (assign31600_e39210 + var_mtat);
        let assign31600_e39214: f64 = (-230.25850929940458);
        let assign31600_e39215: f64 = if assign31600_e39212 > assign31600_e39214 { 1.0 } else { 0.0 };
        var_guard617 = assign31600_e39215;

        let (assign31610_e39233, assign31610_e39233_d_n5, assign31610_e39233_d_n6, assign31610_e39233_d_n7, assign31610_e39233_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) && (var_guard617 != 0.0)) {
        let assign31610_e39228: f64 = (-var_ysq);
        let assign31610_e39230: f64 = (assign31610_e39228 + var_mtat);
        let assign31610_e39231: f64 = (assign31610_e39230).exp();
        (assign31610_e39231, (assign31610_e39231 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign31610_e39231 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign31610_e39231 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign31610_e39231 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31610_e39233;
        var_tmp_dn5 = assign31610_e39233_d_n5;
        var_tmp_dn6 = assign31610_e39233_d_n6;
        var_tmp_dn7 = assign31610_e39233_d_n7;
        var_tmp_dn8 = assign31610_e39233_d_n8;

        let (assign31620_e39282, assign31620_e39282_d_n5, assign31620_e39282_d_n6, assign31620_e39282_d_n7, assign31620_e39282_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) && (var_guard617 == 0.0)) {
        let assign31620_e39249: f64 = (-230.25850929940458);
        let assign31620_e39251: f64 = (-var_ysq);
        let assign31620_e39253: f64 = (assign31620_e39251 + var_mtat);
        let assign31620_e39254: f64 = (assign31620_e39249 - assign31620_e39253);
        let assign31620_e39258: f64 = (-230.25850929940458);
        let assign31620_e39260: f64 = (-var_ysq);
        let assign31620_e39262: f64 = (assign31620_e39260 + var_mtat);
        let assign31620_e39263: f64 = (assign31620_e39258 - assign31620_e39262);
        let assign31620_e39266: f64 = (-230.25850929940458);
        let assign31620_e39268: f64 = (-var_ysq);
        let assign31620_e39270: f64 = (assign31620_e39268 + var_mtat);
        let assign31620_e39271: f64 = (assign31620_e39266 - assign31620_e39270);
        let assign31620_e39273: f64 = (assign31620_e39271 * 0.3333333333333333);
        let assign31620_e39274: f64 = (1.0 + assign31620_e39273);
        let assign31620_e39275: f64 = (assign31620_e39263 * assign31620_e39274);
        let assign31620_e39276: f64 = (0.5 * assign31620_e39275);
        let assign31620_e39277: f64 = (1.0 + assign31620_e39276);
        let assign31620_e39278: f64 = (assign31620_e39254 * assign31620_e39277);
        let assign31620_e39279: f64 = (1.0 + assign31620_e39278);
        let assign31620_e39280: f64 = (1e-100 / assign31620_e39279);
        (assign31620_e39280, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign31620_e39277) + (assign31620_e39254 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign31620_e39274) + (assign31620_e39263 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign31620_e39279 * assign31620_e39279))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign31620_e39277) + (assign31620_e39254 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign31620_e39274) + (assign31620_e39263 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign31620_e39279 * assign31620_e39279))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign31620_e39277) + (assign31620_e39254 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign31620_e39274) + (assign31620_e39263 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign31620_e39279 * assign31620_e39279))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign31620_e39277) + (assign31620_e39254 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign31620_e39274) + (assign31620_e39263 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign31620_e39279 * assign31620_e39279))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31620_e39282;
        var_tmp_dn5 = assign31620_e39282_d_n5;
        var_tmp_dn6 = assign31620_e39282_d_n6;
        var_tmp_dn7 = assign31620_e39282_d_n7;
        var_tmp_dn8 = assign31620_e39282_d_n8;

        let (assign31630_e39312, assign31630_e39312_d_n5, assign31630_e39312_d_n6, assign31630_e39312_d_n7, assign31630_e39312_d_n8,) = {
    if ((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) {
        let assign31630_e39294: f64 = (0.29214664 * var_terfc);
        let assign31630_e39298: f64 = (var_terfc * var_terfc);
        let assign31630_e39299: f64 = (var_berfc * assign31630_e39298);
        let assign31630_e39300: f64 = (assign31630_e39294 + assign31630_e39299);
        let assign31630_e39304: f64 = (var_terfc * var_terfc);
        let assign31630_e39306: f64 = (assign31630_e39304 * var_terfc);
        let assign31630_e39307: f64 = (var_cerfc * assign31630_e39306);
        let assign31630_e39308: f64 = (assign31630_e39300 + assign31630_e39307);
        let assign31630_e39310: f64 = (assign31630_e39308 * var_tmp);
        (assign31630_e39310, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign31630_e39304 * var_terfc_dn5)))) * var_tmp) + (assign31630_e39308 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign31630_e39304 * var_terfc_dn6)))) * var_tmp) + (assign31630_e39308 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign31630_e39304 * var_terfc_dn7)))) * var_tmp) + (assign31630_e39308 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign31630_e39304 * var_terfc_dn8)))) * var_tmp) + (assign31630_e39308 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign31630_e39312;
        var_erfcpos_dn5 = assign31630_e39312_d_n5;
        var_erfcpos_dn6 = assign31630_e39312_d_n6;
        var_erfcpos_dn7 = assign31630_e39312_d_n7;
        var_erfcpos_dn8 = assign31630_e39312_d_n8;

        let assign31640_e39315: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard618 = assign31640_e39315;

        let (assign31650_e39329, assign31650_e39329_d_n5, assign31650_e39329_d_n6, assign31650_e39329_d_n7, assign31650_e39329_d_n8,) = {
    if (((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) && (var_guard618 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign31650_e39329;
        var_erfctimesexpmtat_dn5 = assign31650_e39329_d_n5;
        var_erfctimesexpmtat_dn6 = assign31650_e39329_d_n6;
        var_erfctimesexpmtat_dn7 = assign31650_e39329_d_n7;
        var_erfctimesexpmtat_dn8 = assign31650_e39329_d_n8;

        let assign31660_e39332: f64 = (-230.25850929940458);
        let assign31660_e39333: f64 = if var_mtat > assign31660_e39332 { 1.0 } else { 0.0 };
        var_guard619 = assign31660_e39333;

        let (assign31670_e39351, assign31670_e39351_d_n5, assign31670_e39351_d_n6, assign31670_e39351_d_n7, assign31670_e39351_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) && (var_guard618 == 0.0)) && (var_guard619 != 0.0)) {
        let assign31670_e39349: f64 = (var_mtat).exp();
        (assign31670_e39349, (assign31670_e39349 * var_mtat_dn5), (assign31670_e39349 * var_mtat_dn6), (assign31670_e39349 * var_mtat_dn7), (assign31670_e39349 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31670_e39351;
        var_tmp_dn5 = assign31670_e39351_d_n5;
        var_tmp_dn6 = assign31670_e39351_d_n6;
        var_tmp_dn7 = assign31670_e39351_d_n7;
        var_tmp_dn8 = assign31670_e39351_d_n8;

        let (assign31680_e39394, assign31680_e39394_d_n5, assign31680_e39394_d_n6, assign31680_e39394_d_n7, assign31680_e39394_d_n8,) = {
    if ((((((var_guard176 != 0.0) && (var_guard193 != 0.0)) && (var_guard610 == 0.0)) && (var_guard614 == 0.0)) && (var_guard618 == 0.0)) && (var_guard619 == 0.0)) {
        let assign31680_e39370: f64 = (-230.25850929940458);
        let assign31680_e39372: f64 = (assign31680_e39370 - var_mtat);
        let assign31680_e39376: f64 = (-230.25850929940458);
        let assign31680_e39378: f64 = (assign31680_e39376 - var_mtat);
        let assign31680_e39381: f64 = (-230.25850929940458);
        let assign31680_e39383: f64 = (assign31680_e39381 - var_mtat);
        let assign31680_e39385: f64 = (assign31680_e39383 * 0.3333333333333333);
        let assign31680_e39386: f64 = (1.0 + assign31680_e39385);
        let assign31680_e39387: f64 = (assign31680_e39378 * assign31680_e39386);
        let assign31680_e39388: f64 = (0.5 * assign31680_e39387);
        let assign31680_e39389: f64 = (1.0 + assign31680_e39388);
        let assign31680_e39390: f64 = (assign31680_e39372 * assign31680_e39389);
        let assign31680_e39391: f64 = (1.0 + assign31680_e39390);
        let assign31680_e39392: f64 = (1e-100 / assign31680_e39391);
        (assign31680_e39392, (-((1e-100 * (((-var_mtat_dn5) * assign31680_e39389) + (assign31680_e39372 * (0.5 * (((-var_mtat_dn5) * assign31680_e39386) + (assign31680_e39378 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign31680_e39391 * assign31680_e39391))), (-((1e-100 * (((-var_mtat_dn6) * assign31680_e39389) + (assign31680_e39372 * (0.5 * (((-var_mtat_dn6) * assign31680_e39386) + (assign31680_e39378 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign31680_e39391 * assign31680_e39391))), (-((1e-100 * (((-var_mtat_dn7) * assign31680_e39389) + (assign31680_e39372 * (0.5 * (((-var_mtat_dn7) * assign31680_e39386) + (assign31680_e39378 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign31680_e39391 * assign31680_e39391))), (-((1e-100 * (((-var_mtat_dn8) * assign31680_e39389) + (assign31680_e39372 * (0.5 * (((-var_mtat_dn8) * assign31680_e39386) + (assign31680_e39378 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign31680_e39391 * assign31680_e39391))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31680_e39394;
        var_tmp_dn5 = assign31680_e39394_d_n5;
        var_tmp_dn6 = assign31680_e39394_d_n6;
        var_tmp_dn7 = assign31680_e39394_d_n7;
        var_tmp_dn8 = assign31680_e39394_d_n8;

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
        *var_guard613_slot = var_guard613;
        *var_guard614_slot = var_guard614;
        *var_guard615_slot = var_guard615;
        *var_guard616_slot = var_guard616;
        *var_guard617_slot = var_guard617;
        *var_guard618_slot = var_guard618;
        *var_guard619_slot = var_guard619;
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
}
