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
        var_btatpartbot: f64,
        var_cerfc: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard477: f64,
        var_guard478: f64,
        var_one_minus_pbot: f64,
        var_one_over_one_minus_pbot: f64,
        var_perfc: f64,
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
        var_guard481_slot: &mut f64,
        var_guard482_slot: &mut f64,
        var_guard483_slot: &mut f64,
        var_guard484_slot: &mut f64,
        var_guard485_slot: &mut f64,
        var_guard486_slot: &mut f64,
        var_guard487_slot: &mut f64,
        var_guard488_slot: &mut f64,
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
        let mut var_guard481: f64 = *var_guard481_slot;
        let mut var_guard482: f64 = *var_guard482_slot;
        let mut var_guard483: f64 = *var_guard483_slot;
        let mut var_guard484: f64 = *var_guard484_slot;
        let mut var_guard485: f64 = *var_guard485_slot;
        let mut var_guard486: f64 = *var_guard486_slot;
        let mut var_guard487: f64 = *var_guard487_slot;
        let mut var_guard488: f64 = *var_guard488_slot;
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

        let (assign25550_e29407, assign25550_e29407_d_n5, assign25550_e29407_d_n6, assign25550_e29407_d_n7, assign25550_e29407_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard478 == 0.0)) {
        let assign25550_e29404: f64 = (var_asrh * var_wsrh);
        let assign25550_e29405: f64 = (p.p840 * assign25550_e29404);
        (assign25550_e29405, (p.p840 * (var_asrh_dn5 * var_wsrh)), (p.p840 * (var_asrh_dn6 * var_wsrh)), (p.p840 * (var_asrh_dn7 * var_wsrh)), (p.p840 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign25550_e29407;
        var_isrh_dn5 = assign25550_e29407_d_n5;
        var_isrh_dn6 = assign25550_e29407_d_n6;
        var_isrh_dn7 = assign25550_e29407_d_n7;
        var_isrh_dn8 = assign25550_e29407_d_n8;

        let assign25560_e29410: f64 = if p.p845 == 0.0 { 1.0 } else { 0.0 };
        var_guard481 = assign25560_e29410;

        let (assign25570_e29421, assign25570_e29421_d_n5, assign25570_e29421_d_n6, assign25570_e29421_d_n7, assign25570_e29421_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign25570_e29421;
        var_itat_dn5 = assign25570_e29421_d_n5;
        var_itat_dn6 = assign25570_e29421_d_n6;
        var_itat_dn7 = assign25570_e29421_d_n7;
        var_itat_dn8 = assign25570_e29421_d_n8;

        let (assign25580_e29439, assign25580_e29439_d_n5, assign25580_e29439_d_n6, assign25580_e29439_d_n7, assign25580_e29439_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) {
        let assign25580_e29434: f64 = (var_wdep * var_one_minus_pbot);
        let assign25580_e29436: f64 = (assign25580_e29434 / var_vbi_minus_vjsrh);
        let assign25580_e29437: f64 = (var_btatpartbot * assign25580_e29436);
        (assign25580_e29437, (var_btatpartbot * ((var_wdep_dn5 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn6 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn7 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn8 * var_one_minus_pbot) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign25580_e29439;
        var_btat_dn5 = assign25580_e29439_d_n5;
        var_btat_dn6 = assign25580_e29439_d_n6;
        var_btat_dn7 = assign25580_e29439_d_n7;
        var_btat_dn8 = assign25580_e29439_d_n8;

        let (assign25590_e29455, assign25590_e29455_d_n5, assign25590_e29455_d_n6, assign25590_e29455_d_n7, assign25590_e29455_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) {
        let assign25590_e29451: f64 = (0.666666666666667 * var_atatbot);
        let assign25590_e29453: f64 = (assign25590_e29451 / var_btat);
        (assign25590_e29453, (-((assign25590_e29451 * var_btat_dn5) / (var_btat * var_btat))), (-((assign25590_e29451 * var_btat_dn6) / (var_btat * var_btat))), (-((assign25590_e29451 * var_btat_dn7) / (var_btat * var_btat))), (-((assign25590_e29451 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign25590_e29455;
        var_twoatatoverthreebtat_dn5 = assign25590_e29455_d_n5;
        var_twoatatoverthreebtat_dn6 = assign25590_e29455_d_n6;
        var_twoatatoverthreebtat_dn7 = assign25590_e29455_d_n7;
        var_twoatatoverthreebtat_dn8 = assign25590_e29455_d_n8;

        let (assign25600_e29469, assign25600_e29469_d_n5, assign25600_e29469_d_n6, assign25600_e29469_d_n7, assign25600_e29469_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) {
        let assign25600_e29467: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign25600_e29467, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign25600_e29469;
        var_umaxbeforelimiting_dn5 = assign25600_e29469_d_n5;
        var_umaxbeforelimiting_dn6 = assign25600_e29469_d_n6;
        var_umaxbeforelimiting_dn7 = assign25600_e29469_d_n7;
        var_umaxbeforelimiting_dn8 = assign25600_e29469_d_n8;

        let (assign25610_e29490, assign25610_e29490_d_n5, assign25610_e29490_d_n6, assign25610_e29490_d_n7, assign25610_e29490_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) {
        let assign25610_e29481: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign25610_e29484: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign25610_e29486: f64 = (assign25610_e29484 + 1.0);
        let assign25610_e29487: f64 = (assign25610_e29481 / assign25610_e29486);
        let assign25610_e29488: f64 = (assign25610_e29487).sqrt();
        (assign25610_e29488, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign25610_e29486) - (assign25610_e29481 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign25610_e29486 * assign25610_e29486)) / (2.0 * assign25610_e29488)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign25610_e29486) - (assign25610_e29481 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign25610_e29486 * assign25610_e29486)) / (2.0 * assign25610_e29488)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign25610_e29486) - (assign25610_e29481 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign25610_e29486 * assign25610_e29486)) / (2.0 * assign25610_e29488)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign25610_e29486) - (assign25610_e29481 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign25610_e29486 * assign25610_e29486)) / (2.0 * assign25610_e29488)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign25610_e29490;
        var_umax_dn5 = assign25610_e29490_d_n5;
        var_umax_dn6 = assign25610_e29490_d_n6;
        var_umax_dn7 = assign25610_e29490_d_n7;
        var_umax_dn8 = assign25610_e29490_d_n8;

        let (assign25620_e29503, assign25620_e29503_d_n5, assign25620_e29503_d_n6, assign25620_e29503_d_n7, assign25620_e29503_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) {
        let assign25620_e29501: f64 = (var_umax).sqrt();
        (assign25620_e29501, (var_umax_dn5 / (2.0 * assign25620_e29501)), (var_umax_dn6 / (2.0 * assign25620_e29501)), (var_umax_dn7 / (2.0 * assign25620_e29501)), (var_umax_dn8 / (2.0 * assign25620_e29501)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign25620_e29503;
        var_sqrtumax_dn5 = assign25620_e29503_d_n5;
        var_sqrtumax_dn6 = assign25620_e29503_d_n6;
        var_sqrtumax_dn7 = assign25620_e29503_d_n7;
        var_sqrtumax_dn8 = assign25620_e29503_d_n8;

        let (assign25630_e29517, assign25630_e29517_d_n5, assign25630_e29517_d_n6, assign25630_e29517_d_n7, assign25630_e29517_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) {
        let assign25630_e29515: f64 = (var_umax * var_sqrtumax);
        (assign25630_e29515, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign25630_e29517;
        var_umaxpoweronepointfive_dn5 = assign25630_e29517_d_n5;
        var_umaxpoweronepointfive_dn6 = assign25630_e29517_d_n6;
        var_umaxpoweronepointfive_dn7 = assign25630_e29517_d_n7;
        var_umaxpoweronepointfive_dn8 = assign25630_e29517_d_n8;

        let assign25640_e29519: f64 = (-p.p831);
        let assign25640_e29521: f64 = (assign25640_e29519 * var_one_over_one_minus_pbot);
        let assign25640_e29523: f64 = (-1.0);
        let assign25640_e29524: f64 = if assign25640_e29521 == assign25640_e29523 { 1.0 } else { 0.0 };
        var_guard482 = assign25640_e29524;

        let (assign25650_e29544, assign25650_e29544_d_n5, assign25650_e29544_d_n6, assign25650_e29544_d_n7, assign25650_e29544_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) && (var_guard482 != 0.0)) {
        let assign25650_e29540: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign25650_e29541: f64 = (1.0 + assign25650_e29540);
        let assign25650_e29542: f64 = (1.0 / assign25650_e29541);
        (assign25650_e29542, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign25650_e29541 * assign25650_e29541))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign25650_e29541 * assign25650_e29541))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign25650_e29541 * assign25650_e29541))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign25650_e29541 * assign25650_e29541))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign25650_e29544;
        var_wgamma_dn5 = assign25650_e29544_d_n5;
        var_wgamma_dn6 = assign25650_e29544_d_n6;
        var_wgamma_dn7 = assign25650_e29544_d_n7;
        var_wgamma_dn8 = assign25650_e29544_d_n8;

        let (assign25660_e29568, assign25660_e29568_d_n5, assign25660_e29568_d_n6, assign25660_e29568_d_n7, assign25660_e29568_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) && (var_guard482 == 0.0)) {
        let assign25660_e29560: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign25660_e29561: f64 = (1.0 + assign25660_e29560);
        let assign25660_e29563: f64 = (-p.p831);
        let assign25660_e29565: f64 = (assign25660_e29563 * var_one_over_one_minus_pbot);
        let assign25660_e29566: f64 = (assign25660_e29561).powf(assign25660_e29565);
        (assign25660_e29566, if 0.0 == 0.0 && ((assign25660_e29565) as f64).is_finite() && ((assign25660_e29565) as f64).fract() == 0.0 { if assign25660_e29565 == 0.0 { 0.0 } else { (assign25660_e29565 * ((assign25660_e29561).powf(assign25660_e29565 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign25660_e29566 * (assign25660_e29565 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign25660_e29561))) }, if 0.0 == 0.0 && ((assign25660_e29565) as f64).is_finite() && ((assign25660_e29565) as f64).fract() == 0.0 { if assign25660_e29565 == 0.0 { 0.0 } else { (assign25660_e29565 * ((assign25660_e29561).powf(assign25660_e29565 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign25660_e29566 * (assign25660_e29565 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign25660_e29561))) }, if 0.0 == 0.0 && ((assign25660_e29565) as f64).is_finite() && ((assign25660_e29565) as f64).fract() == 0.0 { if assign25660_e29565 == 0.0 { 0.0 } else { (assign25660_e29565 * ((assign25660_e29561).powf(assign25660_e29565 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign25660_e29566 * (assign25660_e29565 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign25660_e29561))) }, if 0.0 == 0.0 && ((assign25660_e29565) as f64).is_finite() && ((assign25660_e29565) as f64).fract() == 0.0 { if assign25660_e29565 == 0.0 { 0.0 } else { (assign25660_e29565 * ((assign25660_e29561).powf(assign25660_e29565 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign25660_e29566 * (assign25660_e29565 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign25660_e29561))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign25660_e29568;
        var_wgamma_dn5 = assign25660_e29568_d_n5;
        var_wgamma_dn6 = assign25660_e29568_d_n6;
        var_wgamma_dn7 = assign25660_e29568_d_n7;
        var_wgamma_dn8 = assign25660_e29568_d_n8;

        let (assign25670_e29586, assign25670_e29586_d_n5, assign25670_e29586_d_n6, assign25670_e29586_d_n7, assign25670_e29586_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) {
        let assign25670_e29580: f64 = (var_wsrh * var_wgamma);
        let assign25670_e29583: f64 = (var_wsrh + var_wgamma);
        let assign25670_e29584: f64 = (assign25670_e29580 / assign25670_e29583);
        (assign25670_e29584, ((((var_wsrh * var_wgamma_dn5) * assign25670_e29583) - (assign25670_e29580 * var_wgamma_dn5)) / (assign25670_e29583 * assign25670_e29583)), ((((var_wsrh * var_wgamma_dn6) * assign25670_e29583) - (assign25670_e29580 * var_wgamma_dn6)) / (assign25670_e29583 * assign25670_e29583)), ((((var_wsrh * var_wgamma_dn7) * assign25670_e29583) - (assign25670_e29580 * var_wgamma_dn7)) / (assign25670_e29583 * assign25670_e29583)), ((((var_wsrh * var_wgamma_dn8) * assign25670_e29583) - (assign25670_e29580 * var_wgamma_dn8)) / (assign25670_e29583 * assign25670_e29583)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign25670_e29586;
        var_wtat_dn5 = assign25670_e29586_d_n5;
        var_wtat_dn6 = assign25670_e29586_d_n6;
        var_wtat_dn7 = assign25670_e29586_d_n7;
        var_wtat_dn8 = assign25670_e29586_d_n8;

        let (assign25680_e29603, assign25680_e29603_d_n5, assign25680_e29603_d_n6, assign25680_e29603_d_n7, assign25680_e29603_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) {
        let assign25680_e29599: f64 = (var_btat / var_sqrtumax);
        let assign25680_e29600: f64 = (0.375 * assign25680_e29599);
        let assign25680_e29601: f64 = (assign25680_e29600).sqrt();
        (assign25680_e29601, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign25680_e29601)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign25680_e29601)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign25680_e29601)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign25680_e29601)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign25680_e29603;
        var_ktat_dn5 = assign25680_e29603_d_n5;
        var_ktat_dn6 = assign25680_e29603_d_n6;
        var_ktat_dn7 = assign25680_e29603_d_n7;
        var_ktat_dn8 = assign25680_e29603_d_n8;

        let (assign25690_e29621, assign25690_e29621_d_n5, assign25690_e29621_d_n6, assign25690_e29621_d_n7, assign25690_e29621_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) {
        let assign25690_e29616: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign25690_e29617: f64 = (2.0 * assign25690_e29616);
        let assign25690_e29619: f64 = (assign25690_e29617 - var_umax);
        (assign25690_e29619, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign25690_e29621;
        var_ltat_dn5 = assign25690_e29621_d_n5;
        var_ltat_dn6 = assign25690_e29621_d_n6;
        var_ltat_dn7 = assign25690_e29621_d_n7;
        var_ltat_dn8 = assign25690_e29621_d_n8;

        let (assign25700_e29647, assign25700_e29647_d_n5, assign25700_e29647_d_n6, assign25700_e29647_d_n7, assign25700_e29647_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) {
        let assign25700_e29633: f64 = (var_atatbot * var_twoatatoverthreebtat);
        let assign25700_e29635: f64 = (assign25700_e29633 * var_sqrtumax);
        let assign25700_e29638: f64 = (var_atatbot * var_umax);
        let assign25700_e29639: f64 = (assign25700_e29635 - assign25700_e29638);
        let assign25700_e29643: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign25700_e29644: f64 = (0.5 * assign25700_e29643);
        let assign25700_e29645: f64 = (assign25700_e29639 + assign25700_e29644);
        (assign25700_e29645, (((((var_atatbot * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign25700_e29633 * var_sqrtumax_dn5)) - (var_atatbot * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatbot * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign25700_e29633 * var_sqrtumax_dn6)) - (var_atatbot * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign25700_e29633 * var_sqrtumax_dn7)) - (var_atatbot * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign25700_e29633 * var_sqrtumax_dn8)) - (var_atatbot * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign25700_e29647;
        var_mtat_dn5 = assign25700_e29647_d_n5;
        var_mtat_dn6 = assign25700_e29647_d_n6;
        var_mtat_dn7 = assign25700_e29647_d_n7;
        var_mtat_dn8 = assign25700_e29647_d_n8;

        let (assign25710_e29663, assign25710_e29663_d_n5, assign25710_e29663_d_n6, assign25710_e29663_d_n7, assign25710_e29663_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) {
        let assign25710_e29659: f64 = (var_ltat - 1.0);
        let assign25710_e29661: f64 = (assign25710_e29659 * var_ktat);
        (assign25710_e29661, ((var_ltat_dn5 * var_ktat) + (assign25710_e29659 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign25710_e29659 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign25710_e29659 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign25710_e29659 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign25710_e29663;
        var_xerfc_dn5 = assign25710_e29663_d_n5;
        var_xerfc_dn6 = assign25710_e29663_d_n6;
        var_xerfc_dn7 = assign25710_e29663_d_n7;
        var_xerfc_dn8 = assign25710_e29663_d_n8;

        let (assign25720_e29677, assign25720_e29677_d_n5, assign25720_e29677_d_n6, assign25720_e29677_d_n7, assign25720_e29677_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) {
        let assign25720_e29675: f64 = (var_xerfc * var_xerfc);
        (assign25720_e29675, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign25720_e29677;
        var_ysq_dn5 = assign25720_e29677_d_n5;
        var_ysq_dn6 = assign25720_e29677_d_n6;
        var_ysq_dn7 = assign25720_e29677_d_n7;
        var_ysq_dn8 = assign25720_e29677_d_n8;

        let assign25730_e29680: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard483 = assign25730_e29680;

        let (assign25740_e29700, assign25740_e29700_d_n5, assign25740_e29700_d_n6, assign25740_e29700_d_n7, assign25740_e29700_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) && (var_guard483 != 0.0)) {
        let assign25740_e29696: f64 = (var_perfc * var_xerfc);
        let assign25740_e29697: f64 = (1.0 + assign25740_e29696);
        let assign25740_e29698: f64 = (1.0 / assign25740_e29697);
        (assign25740_e29698, (-((var_perfc * var_xerfc_dn5) / (assign25740_e29697 * assign25740_e29697))), (-((var_perfc * var_xerfc_dn6) / (assign25740_e29697 * assign25740_e29697))), (-((var_perfc * var_xerfc_dn7) / (assign25740_e29697 * assign25740_e29697))), (-((var_perfc * var_xerfc_dn8) / (assign25740_e29697 * assign25740_e29697))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign25740_e29700;
        var_terfc_dn5 = assign25740_e29700_d_n5;
        var_terfc_dn6 = assign25740_e29700_d_n6;
        var_terfc_dn7 = assign25740_e29700_d_n7;
        var_terfc_dn8 = assign25740_e29700_d_n8;

        let (assign25750_e29721, assign25750_e29721_d_n5, assign25750_e29721_d_n6, assign25750_e29721_d_n7, assign25750_e29721_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) && (var_guard483 == 0.0)) {
        let assign25750_e29717: f64 = (var_perfc * var_xerfc);
        let assign25750_e29718: f64 = (1.0 - assign25750_e29717);
        let assign25750_e29719: f64 = (1.0 / assign25750_e29718);
        (assign25750_e29719, (-((-(var_perfc * var_xerfc_dn5)) / (assign25750_e29718 * assign25750_e29718))), (-((-(var_perfc * var_xerfc_dn6)) / (assign25750_e29718 * assign25750_e29718))), (-((-(var_perfc * var_xerfc_dn7)) / (assign25750_e29718 * assign25750_e29718))), (-((-(var_perfc * var_xerfc_dn8)) / (assign25750_e29718 * assign25750_e29718))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign25750_e29721;
        var_terfc_dn5 = assign25750_e29721_d_n5;
        var_terfc_dn6 = assign25750_e29721_d_n6;
        var_terfc_dn7 = assign25750_e29721_d_n7;
        var_terfc_dn8 = assign25750_e29721_d_n8;

        let assign25760_e29723: f64 = (-var_ysq);
        let assign25760_e29725: f64 = (assign25760_e29723 + var_mtat);
        let assign25760_e29727: f64 = (-230.25850929940458);
        let assign25760_e29728: f64 = if assign25760_e29725 > assign25760_e29727 { 1.0 } else { 0.0 };
        var_guard484 = assign25760_e29728;

        let (assign25770_e29746, assign25770_e29746_d_n5, assign25770_e29746_d_n6, assign25770_e29746_d_n7, assign25770_e29746_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) && (var_guard484 != 0.0)) {
        let assign25770_e29741: f64 = (-var_ysq);
        let assign25770_e29743: f64 = (assign25770_e29741 + var_mtat);
        let assign25770_e29744: f64 = (assign25770_e29743).exp();
        (assign25770_e29744, (assign25770_e29744 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign25770_e29744 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign25770_e29744 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign25770_e29744 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25770_e29746;
        var_tmp_dn5 = assign25770_e29746_d_n5;
        var_tmp_dn6 = assign25770_e29746_d_n6;
        var_tmp_dn7 = assign25770_e29746_d_n7;
        var_tmp_dn8 = assign25770_e29746_d_n8;

        let (assign25780_e29795, assign25780_e29795_d_n5, assign25780_e29795_d_n6, assign25780_e29795_d_n7, assign25780_e29795_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) && (var_guard484 == 0.0)) {
        let assign25780_e29762: f64 = (-230.25850929940458);
        let assign25780_e29764: f64 = (-var_ysq);
        let assign25780_e29766: f64 = (assign25780_e29764 + var_mtat);
        let assign25780_e29767: f64 = (assign25780_e29762 - assign25780_e29766);
        let assign25780_e29771: f64 = (-230.25850929940458);
        let assign25780_e29773: f64 = (-var_ysq);
        let assign25780_e29775: f64 = (assign25780_e29773 + var_mtat);
        let assign25780_e29776: f64 = (assign25780_e29771 - assign25780_e29775);
        let assign25780_e29779: f64 = (-230.25850929940458);
        let assign25780_e29781: f64 = (-var_ysq);
        let assign25780_e29783: f64 = (assign25780_e29781 + var_mtat);
        let assign25780_e29784: f64 = (assign25780_e29779 - assign25780_e29783);
        let assign25780_e29786: f64 = (assign25780_e29784 * 0.3333333333333333);
        let assign25780_e29787: f64 = (1.0 + assign25780_e29786);
        let assign25780_e29788: f64 = (assign25780_e29776 * assign25780_e29787);
        let assign25780_e29789: f64 = (0.5 * assign25780_e29788);
        let assign25780_e29790: f64 = (1.0 + assign25780_e29789);
        let assign25780_e29791: f64 = (assign25780_e29767 * assign25780_e29790);
        let assign25780_e29792: f64 = (1.0 + assign25780_e29791);
        let assign25780_e29793: f64 = (1e-100 / assign25780_e29792);
        (assign25780_e29793, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign25780_e29790) + (assign25780_e29767 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign25780_e29787) + (assign25780_e29776 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign25780_e29792 * assign25780_e29792))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign25780_e29790) + (assign25780_e29767 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign25780_e29787) + (assign25780_e29776 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign25780_e29792 * assign25780_e29792))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign25780_e29790) + (assign25780_e29767 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign25780_e29787) + (assign25780_e29776 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign25780_e29792 * assign25780_e29792))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign25780_e29790) + (assign25780_e29767 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign25780_e29787) + (assign25780_e29776 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign25780_e29792 * assign25780_e29792))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25780_e29795;
        var_tmp_dn5 = assign25780_e29795_d_n5;
        var_tmp_dn6 = assign25780_e29795_d_n6;
        var_tmp_dn7 = assign25780_e29795_d_n7;
        var_tmp_dn8 = assign25780_e29795_d_n8;

        let (assign25790_e29825, assign25790_e29825_d_n5, assign25790_e29825_d_n6, assign25790_e29825_d_n7, assign25790_e29825_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) {
        let assign25790_e29807: f64 = (0.29214664 * var_terfc);
        let assign25790_e29811: f64 = (var_terfc * var_terfc);
        let assign25790_e29812: f64 = (var_berfc * assign25790_e29811);
        let assign25790_e29813: f64 = (assign25790_e29807 + assign25790_e29812);
        let assign25790_e29817: f64 = (var_terfc * var_terfc);
        let assign25790_e29819: f64 = (assign25790_e29817 * var_terfc);
        let assign25790_e29820: f64 = (var_cerfc * assign25790_e29819);
        let assign25790_e29821: f64 = (assign25790_e29813 + assign25790_e29820);
        let assign25790_e29823: f64 = (assign25790_e29821 * var_tmp);
        (assign25790_e29823, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign25790_e29817 * var_terfc_dn5)))) * var_tmp) + (assign25790_e29821 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign25790_e29817 * var_terfc_dn6)))) * var_tmp) + (assign25790_e29821 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign25790_e29817 * var_terfc_dn7)))) * var_tmp) + (assign25790_e29821 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign25790_e29817 * var_terfc_dn8)))) * var_tmp) + (assign25790_e29821 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign25790_e29825;
        var_erfcpos_dn5 = assign25790_e29825_d_n5;
        var_erfcpos_dn6 = assign25790_e29825_d_n6;
        var_erfcpos_dn7 = assign25790_e29825_d_n7;
        var_erfcpos_dn8 = assign25790_e29825_d_n8;

        let assign25800_e29828: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard485 = assign25800_e29828;

        let (assign25810_e29842, assign25810_e29842_d_n5, assign25810_e29842_d_n6, assign25810_e29842_d_n7, assign25810_e29842_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) && (var_guard485 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign25810_e29842;
        var_erfctimesexpmtat_dn5 = assign25810_e29842_d_n5;
        var_erfctimesexpmtat_dn6 = assign25810_e29842_d_n6;
        var_erfctimesexpmtat_dn7 = assign25810_e29842_d_n7;
        var_erfctimesexpmtat_dn8 = assign25810_e29842_d_n8;

        let assign25820_e29845: f64 = (-230.25850929940458);
        let assign25820_e29846: f64 = if var_mtat > assign25820_e29845 { 1.0 } else { 0.0 };
        var_guard486 = assign25820_e29846;

        let (assign25830_e29864, assign25830_e29864_d_n5, assign25830_e29864_d_n6, assign25830_e29864_d_n7, assign25830_e29864_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) && (var_guard485 == 0.0)) && (var_guard486 != 0.0)) {
        let assign25830_e29862: f64 = (var_mtat).exp();
        (assign25830_e29862, (assign25830_e29862 * var_mtat_dn5), (assign25830_e29862 * var_mtat_dn6), (assign25830_e29862 * var_mtat_dn7), (assign25830_e29862 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25830_e29864;
        var_tmp_dn5 = assign25830_e29864_d_n5;
        var_tmp_dn6 = assign25830_e29864_d_n6;
        var_tmp_dn7 = assign25830_e29864_d_n7;
        var_tmp_dn8 = assign25830_e29864_d_n8;

        let (assign25840_e29907, assign25840_e29907_d_n5, assign25840_e29907_d_n6, assign25840_e29907_d_n7, assign25840_e29907_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) && (var_guard485 == 0.0)) && (var_guard486 == 0.0)) {
        let assign25840_e29883: f64 = (-230.25850929940458);
        let assign25840_e29885: f64 = (assign25840_e29883 - var_mtat);
        let assign25840_e29889: f64 = (-230.25850929940458);
        let assign25840_e29891: f64 = (assign25840_e29889 - var_mtat);
        let assign25840_e29894: f64 = (-230.25850929940458);
        let assign25840_e29896: f64 = (assign25840_e29894 - var_mtat);
        let assign25840_e29898: f64 = (assign25840_e29896 * 0.3333333333333333);
        let assign25840_e29899: f64 = (1.0 + assign25840_e29898);
        let assign25840_e29900: f64 = (assign25840_e29891 * assign25840_e29899);
        let assign25840_e29901: f64 = (0.5 * assign25840_e29900);
        let assign25840_e29902: f64 = (1.0 + assign25840_e29901);
        let assign25840_e29903: f64 = (assign25840_e29885 * assign25840_e29902);
        let assign25840_e29904: f64 = (1.0 + assign25840_e29903);
        let assign25840_e29905: f64 = (1e-100 / assign25840_e29904);
        (assign25840_e29905, (-((1e-100 * (((-var_mtat_dn5) * assign25840_e29902) + (assign25840_e29885 * (0.5 * (((-var_mtat_dn5) * assign25840_e29899) + (assign25840_e29891 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign25840_e29904 * assign25840_e29904))), (-((1e-100 * (((-var_mtat_dn6) * assign25840_e29902) + (assign25840_e29885 * (0.5 * (((-var_mtat_dn6) * assign25840_e29899) + (assign25840_e29891 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign25840_e29904 * assign25840_e29904))), (-((1e-100 * (((-var_mtat_dn7) * assign25840_e29902) + (assign25840_e29885 * (0.5 * (((-var_mtat_dn7) * assign25840_e29899) + (assign25840_e29891 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign25840_e29904 * assign25840_e29904))), (-((1e-100 * (((-var_mtat_dn8) * assign25840_e29902) + (assign25840_e29885 * (0.5 * (((-var_mtat_dn8) * assign25840_e29899) + (assign25840_e29891 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign25840_e29904 * assign25840_e29904))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25840_e29907;
        var_tmp_dn5 = assign25840_e29907_d_n5;
        var_tmp_dn6 = assign25840_e29907_d_n6;
        var_tmp_dn7 = assign25840_e29907_d_n7;
        var_tmp_dn8 = assign25840_e29907_d_n8;

        let (assign25850_e29926, assign25850_e29926_d_n5, assign25850_e29926_d_n6, assign25850_e29926_d_n7, assign25850_e29926_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) && (var_guard485 == 0.0)) {
        let assign25850_e29922: f64 = (2.0 * var_tmp);
        let assign25850_e29924: f64 = (assign25850_e29922 - var_erfcpos);
        (assign25850_e29924, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign25850_e29926;
        var_erfctimesexpmtat_dn5 = assign25850_e29926_d_n5;
        var_erfctimesexpmtat_dn6 = assign25850_e29926_d_n6;
        var_erfctimesexpmtat_dn7 = assign25850_e29926_d_n7;
        var_erfctimesexpmtat_dn8 = assign25850_e29926_d_n8;

        let (assign25860_e29946, assign25860_e29946_d_n5, assign25860_e29946_d_n6, assign25860_e29946_d_n7, assign25860_e29946_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) {
        let assign25860_e29938: f64 = (1.772453850905516 * 0.5);
        let assign25860_e29941: f64 = (var_atatbot * var_erfctimesexpmtat);
        let assign25860_e29943: f64 = (assign25860_e29941 / var_ktat);
        let assign25860_e29944: f64 = (assign25860_e29938 * assign25860_e29943);
        (assign25860_e29944, (assign25860_e29938 * ((((var_atatbot * var_erfctimesexpmtat_dn5) * var_ktat) - (assign25860_e29941 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign25860_e29938 * ((((var_atatbot * var_erfctimesexpmtat_dn6) * var_ktat) - (assign25860_e29941 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign25860_e29938 * ((((var_atatbot * var_erfctimesexpmtat_dn7) * var_ktat) - (assign25860_e29941 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign25860_e29938 * ((((var_atatbot * var_erfctimesexpmtat_dn8) * var_ktat) - (assign25860_e29941 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign25860_e29946;
        var_gammamax_dn5 = assign25860_e29946_d_n5;
        var_gammamax_dn6 = assign25860_e29946_d_n6;
        var_gammamax_dn7 = assign25860_e29946_d_n7;
        var_gammamax_dn8 = assign25860_e29946_d_n8;

        let (assign25870_e29964, assign25870_e29964_d_n5, assign25870_e29964_d_n6, assign25870_e29964_d_n7, assign25870_e29964_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard481 == 0.0)) {
        let assign25870_e29959: f64 = (var_asrh * var_gammamax);
        let assign25870_e29961: f64 = (assign25870_e29959 * var_wtat);
        let assign25870_e29962: f64 = (p.p845 * assign25870_e29961);
        (assign25870_e29962, (p.p845 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign25870_e29959 * var_wtat_dn5))), (p.p845 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign25870_e29959 * var_wtat_dn6))), (p.p845 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign25870_e29959 * var_wtat_dn7))), (p.p845 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign25870_e29959 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign25870_e29964;
        var_itat_dn5 = assign25870_e29964_d_n5;
        var_itat_dn6 = assign25870_e29964_d_n6;
        var_itat_dn7 = assign25870_e29964_d_n7;
        var_itat_dn8 = assign25870_e29964_d_n8;

        let assign25880_e29967: f64 = if p.p851 == 0.0 { 1.0 } else { 0.0 };
        var_guard487 = assign25880_e29967;

        let (assign25890_e29978, assign25890_e29978_d_n5, assign25890_e29978_d_n6, assign25890_e29978_d_n7, assign25890_e29978_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard487 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign25890_e29978;
        var_ibbt_dn5 = assign25890_e29978_d_n5;
        var_ibbt_dn6 = assign25890_e29978_d_n6;
        var_ibbt_dn7 = assign25890_e29978_d_n7;
        var_ibbt_dn8 = assign25890_e29978_d_n8;

        let assign25900_e29981: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        var_guard488 = assign25900_e29981;

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
        *var_guard481_slot = var_guard481;
        *var_guard482_slot = var_guard482;
        *var_guard483_slot = var_guard483;
        *var_guard484_slot = var_guard484;
        *var_guard485_slot = var_guard485;
        *var_guard486_slot = var_guard486;
        *var_guard487_slot = var_guard487;
        *var_guard488_slot = var_guard488;
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

    pub(super) fn stamp_transient_block_49(
        p: &Parameters,
        var_alphaav: f64,
        var_atatsti: f64,
        var_btatpartsti: f64,
        var_fbbtbot: f64,
        var_fstopbot: f64,
        var_ftdsti: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard477: f64,
        var_guard487: f64,
        var_guard488: f64,
        var_idmult: f64,
        var_idsatsti: f64,
        var_lssource_i: f64,
        var_one_minus_psti: f64,
        var_one_over_one_minus_pbot: f64,
        var_slopebot: f64,
        var_two_psistar: f64,
        var_v5: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirbotinv: f64,
        var_vbirstiinv: f64,
        var_vbisti: f64,
        var_vbrinvbot: f64,
        var_vjsrh: f64,
        var_wdepnulrinvbot: f64,
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
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn5_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_guard489_slot: &mut f64,
        var_guard490_slot: &mut f64,
        var_guard491_slot: &mut f64,
        var_guard492_slot: &mut f64,
        var_guard493_slot: &mut f64,
        var_guard494_slot: &mut f64,
        var_guard495_slot: &mut f64,
        var_guard496_slot: &mut f64,
        var_guard497_slot: &mut f64,
        var_guard498_slot: &mut f64,
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
        let mut var_guard489: f64 = *var_guard489_slot;
        let mut var_guard490: f64 = *var_guard490_slot;
        let mut var_guard491: f64 = *var_guard491_slot;
        let mut var_guard492: f64 = *var_guard492_slot;
        let mut var_guard493: f64 = *var_guard493_slot;
        let mut var_guard494: f64 = *var_guard494_slot;
        let mut var_guard495: f64 = *var_guard495_slot;
        let mut var_guard496: f64 = *var_guard496_slot;
        let mut var_guard497: f64 = *var_guard497_slot;
        let mut var_guard498: f64 = *var_guard498_slot;
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

        let (assign25910_e30000, assign25910_e30000_d_n5, assign25910_e30000_d_n6, assign25910_e30000_d_n7, assign25910_e30000_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard487 == 0.0)) && (var_guard488 != 0.0)) {
        let assign25910_e29995: f64 = (p.p828 - var_vbbt);
        let assign25910_e29997: f64 = (assign25910_e29995 * var_vbirbotinv);
        let assign25910_e29998: f64 = (assign25910_e29997).sqrt();
        (assign25910_e29998, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25910_e30000;
        var_tmp_dn5 = assign25910_e30000_d_n5;
        var_tmp_dn6 = assign25910_e30000_d_n6;
        var_tmp_dn7 = assign25910_e30000_d_n7;
        var_tmp_dn8 = assign25910_e30000_d_n8;

        let (assign25920_e30021, assign25920_e30021_d_n5, assign25920_e30021_d_n6, assign25920_e30021_d_n7, assign25920_e30021_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard487 == 0.0)) && (var_guard488 == 0.0)) {
        let assign25920_e30015: f64 = (p.p828 - var_vbbt);
        let assign25920_e30017: f64 = (assign25920_e30015 * var_vbirbotinv);
        let assign25920_e30019: f64 = (assign25920_e30017).powf(p.p831);
        (assign25920_e30019, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25920_e30021;
        var_tmp_dn5 = assign25920_e30021_d_n5;
        var_tmp_dn6 = assign25920_e30021_d_n6;
        var_tmp_dn7 = assign25920_e30021_d_n7;
        var_tmp_dn8 = assign25920_e30021_d_n8;

        let (assign25930_e30041, assign25930_e30041_d_n5, assign25930_e30041_d_n6, assign25930_e30041_d_n7, assign25930_e30041_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard487 == 0.0)) {
        let assign25930_e30034: f64 = (p.p828 - var_vbbt);
        let assign25930_e30036: f64 = (assign25930_e30034 * var_wdepnulrinvbot);
        let assign25930_e30038: f64 = (assign25930_e30036 / var_tmp);
        let assign25930_e30039: f64 = (var_one_over_one_minus_pbot * assign25930_e30038);
        (assign25930_e30039, (var_one_over_one_minus_pbot * (-((assign25930_e30036 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign25930_e30036 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign25930_e30036 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign25930_e30036 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign25930_e30041;
        var_fmaxr_dn5 = assign25930_e30041_d_n5;
        var_fmaxr_dn6 = assign25930_e30041_d_n6;
        var_fmaxr_dn7 = assign25930_e30041_d_n7;
        var_fmaxr_dn8 = assign25930_e30041_d_n8;

        let assign25940_e30043: f64 = (-var_fbbtbot);
        let assign25940_e30045: f64 = (assign25940_e30043 / var_fmaxr);
        let assign25940_e30046: f64 = (assign25940_e30045).abs();
        let assign25940_e30048: f64 = if assign25940_e30046 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard489 = assign25940_e30048;

        let (assign25950_e30066, assign25950_e30066_d_n5, assign25950_e30066_d_n6, assign25950_e30066_d_n7, assign25950_e30066_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard487 == 0.0)) && (var_guard489 != 0.0)) {
        let assign25950_e30061: f64 = (-var_fbbtbot);
        let assign25950_e30063: f64 = (assign25950_e30061 / var_fmaxr);
        let assign25950_e30064: f64 = (assign25950_e30063).exp();
        (assign25950_e30064, (assign25950_e30064 * (-((assign25950_e30061 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign25950_e30064 * (-((assign25950_e30061 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign25950_e30064 * (-((assign25950_e30061 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign25950_e30064 * (-((assign25950_e30061 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25950_e30066;
        var_tmp_dn5 = assign25950_e30066_d_n5;
        var_tmp_dn6 = assign25950_e30066_d_n6;
        var_tmp_dn7 = assign25950_e30066_d_n7;
        var_tmp_dn8 = assign25950_e30066_d_n8;

        let assign25960_e30068: f64 = (-var_fbbtbot);
        let assign25960_e30070: f64 = (assign25960_e30068 / var_fmaxr);
        let assign25960_e30072: f64 = if assign25960_e30070 < 0.0 { 1.0 } else { 0.0 };
        var_guard490 = assign25960_e30072;

        let (assign25970_e30123, assign25970_e30123_d_n5, assign25970_e30123_d_n6, assign25970_e30123_d_n7, assign25970_e30123_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard487 == 0.0)) && (var_guard489 == 0.0)) && (var_guard490 != 0.0)) {
        let assign25970_e30090: f64 = (-230.25850929940458);
        let assign25970_e30092: f64 = (-var_fbbtbot);
        let assign25970_e30094: f64 = (assign25970_e30092 / var_fmaxr);
        let assign25970_e30095: f64 = (assign25970_e30090 - assign25970_e30094);
        let assign25970_e30099: f64 = (-230.25850929940458);
        let assign25970_e30101: f64 = (-var_fbbtbot);
        let assign25970_e30103: f64 = (assign25970_e30101 / var_fmaxr);
        let assign25970_e30104: f64 = (assign25970_e30099 - assign25970_e30103);
        let assign25970_e30107: f64 = (-230.25850929940458);
        let assign25970_e30109: f64 = (-var_fbbtbot);
        let assign25970_e30111: f64 = (assign25970_e30109 / var_fmaxr);
        let assign25970_e30112: f64 = (assign25970_e30107 - assign25970_e30111);
        let assign25970_e30114: f64 = (assign25970_e30112 * 0.3333333333333333);
        let assign25970_e30115: f64 = (1.0 + assign25970_e30114);
        let assign25970_e30116: f64 = (assign25970_e30104 * assign25970_e30115);
        let assign25970_e30117: f64 = (0.5 * assign25970_e30116);
        let assign25970_e30118: f64 = (1.0 + assign25970_e30117);
        let assign25970_e30119: f64 = (assign25970_e30095 * assign25970_e30118);
        let assign25970_e30120: f64 = (1.0 + assign25970_e30119);
        let assign25970_e30121: f64 = (1e-100 / assign25970_e30120);
        (assign25970_e30121, (-((1e-100 * (((-(-((assign25970_e30092 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign25970_e30118) + (assign25970_e30095 * (0.5 * (((-(-((assign25970_e30101 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign25970_e30115) + (assign25970_e30104 * ((-(-((assign25970_e30109 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign25970_e30120 * assign25970_e30120))), (-((1e-100 * (((-(-((assign25970_e30092 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign25970_e30118) + (assign25970_e30095 * (0.5 * (((-(-((assign25970_e30101 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign25970_e30115) + (assign25970_e30104 * ((-(-((assign25970_e30109 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign25970_e30120 * assign25970_e30120))), (-((1e-100 * (((-(-((assign25970_e30092 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign25970_e30118) + (assign25970_e30095 * (0.5 * (((-(-((assign25970_e30101 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign25970_e30115) + (assign25970_e30104 * ((-(-((assign25970_e30109 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign25970_e30120 * assign25970_e30120))), (-((1e-100 * (((-(-((assign25970_e30092 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign25970_e30118) + (assign25970_e30095 * (0.5 * (((-(-((assign25970_e30101 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign25970_e30115) + (assign25970_e30104 * ((-(-((assign25970_e30109 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign25970_e30120 * assign25970_e30120))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25970_e30123;
        var_tmp_dn5 = assign25970_e30123_d_n5;
        var_tmp_dn6 = assign25970_e30123_d_n6;
        var_tmp_dn7 = assign25970_e30123_d_n7;
        var_tmp_dn8 = assign25970_e30123_d_n8;

        let (assign25980_e30172, assign25980_e30172_d_n5, assign25980_e30172_d_n6, assign25980_e30172_d_n7, assign25980_e30172_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard487 == 0.0)) && (var_guard489 == 0.0)) && (var_guard490 == 0.0)) {
        let assign25980_e30142: f64 = (-var_fbbtbot);
        let assign25980_e30144: f64 = (assign25980_e30142 / var_fmaxr);
        let assign25980_e30146: f64 = (assign25980_e30144 - 230.25850929940458);
        let assign25980_e30150: f64 = (-var_fbbtbot);
        let assign25980_e30152: f64 = (assign25980_e30150 / var_fmaxr);
        let assign25980_e30154: f64 = (assign25980_e30152 - 230.25850929940458);
        let assign25980_e30157: f64 = (-var_fbbtbot);
        let assign25980_e30159: f64 = (assign25980_e30157 / var_fmaxr);
        let assign25980_e30161: f64 = (assign25980_e30159 - 230.25850929940458);
        let assign25980_e30163: f64 = (assign25980_e30161 * 0.3333333333333333);
        let assign25980_e30164: f64 = (1.0 + assign25980_e30163);
        let assign25980_e30165: f64 = (assign25980_e30154 * assign25980_e30164);
        let assign25980_e30166: f64 = (0.5 * assign25980_e30165);
        let assign25980_e30167: f64 = (1.0 + assign25980_e30166);
        let assign25980_e30168: f64 = (assign25980_e30146 * assign25980_e30167);
        let assign25980_e30169: f64 = (1.0 + assign25980_e30168);
        let assign25980_e30170: f64 = (1e100 * assign25980_e30169);
        (assign25980_e30170, (1e100 * (((-((assign25980_e30142 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign25980_e30167) + (assign25980_e30146 * (0.5 * (((-((assign25980_e30150 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign25980_e30164) + (assign25980_e30154 * ((-((assign25980_e30157 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign25980_e30142 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign25980_e30167) + (assign25980_e30146 * (0.5 * (((-((assign25980_e30150 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign25980_e30164) + (assign25980_e30154 * ((-((assign25980_e30157 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign25980_e30142 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign25980_e30167) + (assign25980_e30146 * (0.5 * (((-((assign25980_e30150 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign25980_e30164) + (assign25980_e30154 * ((-((assign25980_e30157 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign25980_e30142 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign25980_e30167) + (assign25980_e30146 * (0.5 * (((-((assign25980_e30150 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign25980_e30164) + (assign25980_e30154 * ((-((assign25980_e30157 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25980_e30172;
        var_tmp_dn5 = assign25980_e30172_d_n5;
        var_tmp_dn6 = assign25980_e30172_d_n6;
        var_tmp_dn7 = assign25980_e30172_d_n7;
        var_tmp_dn8 = assign25980_e30172_d_n8;

        let (assign25990_e30192, assign25990_e30192_d_n5, assign25990_e30192_d_n6, assign25990_e30192_d_n7, assign25990_e30192_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard487 == 0.0)) {
        let assign25990_e30185: f64 = (var_v5 * var_fmaxr);
        let assign25990_e30187: f64 = (assign25990_e30185 * var_fmaxr);
        let assign25990_e30189: f64 = (assign25990_e30187 * var_tmp);
        let assign25990_e30190: f64 = (p.p851 * assign25990_e30189);
        (assign25990_e30190, (p.p851 * (((((var_v5 * var_fmaxr_dn5) * var_fmaxr) + (assign25990_e30185 * var_fmaxr_dn5)) * var_tmp) + (assign25990_e30187 * var_tmp_dn5))), (p.p851 * (((((var_v5 * var_fmaxr_dn6) * var_fmaxr) + (assign25990_e30185 * var_fmaxr_dn6)) * var_tmp) + (assign25990_e30187 * var_tmp_dn6))), (p.p851 * (((((var_v5 * var_fmaxr_dn7) * var_fmaxr) + (assign25990_e30185 * var_fmaxr_dn7)) * var_tmp) + (assign25990_e30187 * var_tmp_dn7))), (p.p851 * (((((var_v5 * var_fmaxr_dn8) * var_fmaxr) + (assign25990_e30185 * var_fmaxr_dn8)) * var_tmp) + (assign25990_e30187 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign25990_e30192;
        var_ibbt_dn5 = assign25990_e30192_d_n5;
        var_ibbt_dn6 = assign25990_e30192_d_n6;
        var_ibbt_dn7 = assign25990_e30192_d_n7;
        var_ibbt_dn8 = assign25990_e30192_d_n8;

        let assign26000_e30195: f64 = if p.p860 > 1000.0 { 1.0 } else { 0.0 };
        var_guard491 = assign26000_e30195;

        let (assign26010_e30206, assign26010_e30206_d_n5, assign26010_e30206_d_n6, assign26010_e30206_d_n7, assign26010_e30206_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard491 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign26010_e30206;
        var_fbreakdown_dn5 = assign26010_e30206_d_n5;
        var_fbreakdown_dn6 = assign26010_e30206_d_n6;
        var_fbreakdown_dn7 = assign26010_e30206_d_n7;
        var_fbreakdown_dn8 = assign26010_e30206_d_n8;

        let assign26020_e30209: f64 = (-var_alphaav);
        let assign26020_e30211: f64 = (assign26020_e30209 * p.p860);
        let assign26020_e30212: f64 = if var_vav > assign26020_e30211 { 1.0 } else { 0.0 };
        var_guard492 = assign26020_e30212;

        let assign26030_e30215: f64 = if p.p863 == 4.0 { 1.0 } else { 0.0 };
        var_guard493 = assign26030_e30215;

        let (assign26040_e30245, assign26040_e30245_d_n5, assign26040_e30245_d_n6, assign26040_e30245_d_n7, assign26040_e30245_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard491 == 0.0)) && (var_guard492 != 0.0)) && (var_guard493 != 0.0)) {
        let assign26040_e30231: f64 = (var_vav * var_vbrinvbot);
        let assign26040_e30234: f64 = (var_vav * var_vbrinvbot);
        let assign26040_e30235: f64 = (assign26040_e30231 * assign26040_e30234);
        let assign26040_e30238: f64 = (var_vav * var_vbrinvbot);
        let assign26040_e30239: f64 = (assign26040_e30235 * assign26040_e30238);
        let assign26040_e30242: f64 = (var_vav * var_vbrinvbot);
        let assign26040_e30243: f64 = (assign26040_e30239 * assign26040_e30242);
        (assign26040_e30243, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26040_e30245;
        var_tmp_dn5 = assign26040_e30245_d_n5;
        var_tmp_dn6 = assign26040_e30245_d_n6;
        var_tmp_dn7 = assign26040_e30245_d_n7;
        var_tmp_dn8 = assign26040_e30245_d_n8;

        let (assign26050_e30267, assign26050_e30267_d_n5, assign26050_e30267_d_n6, assign26050_e30267_d_n7, assign26050_e30267_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard491 == 0.0)) && (var_guard492 != 0.0)) && (var_guard493 == 0.0)) {
        let assign26050_e30262: f64 = (var_vav * var_vbrinvbot);
        let assign26050_e30263: f64 = (assign26050_e30262).abs();
        let assign26050_e30265: f64 = (assign26050_e30263).powf(p.p863);
        (assign26050_e30265, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26050_e30267;
        var_tmp_dn5 = assign26050_e30267_d_n5;
        var_tmp_dn6 = assign26050_e30267_d_n6;
        var_tmp_dn7 = assign26050_e30267_d_n7;
        var_tmp_dn8 = assign26050_e30267_d_n8;

        let (assign26060_e30285, assign26060_e30285_d_n5, assign26060_e30285_d_n6, assign26060_e30285_d_n7, assign26060_e30285_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard491 == 0.0)) && (var_guard492 != 0.0)) {
        let assign26060_e30282: f64 = (1.0 - var_tmp);
        let assign26060_e30283: f64 = (1.0 / assign26060_e30282);
        (assign26060_e30283, (-((-var_tmp_dn5) / (assign26060_e30282 * assign26060_e30282))), (-((-var_tmp_dn6) / (assign26060_e30282 * assign26060_e30282))), (-((-var_tmp_dn7) / (assign26060_e30282 * assign26060_e30282))), (-((-var_tmp_dn8) / (assign26060_e30282 * assign26060_e30282))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign26060_e30285;
        var_fbreakdown_dn5 = assign26060_e30285_d_n5;
        var_fbreakdown_dn6 = assign26060_e30285_d_n6;
        var_fbreakdown_dn7 = assign26060_e30285_d_n7;
        var_fbreakdown_dn8 = assign26060_e30285_d_n8;

        let (assign26070_e30308, assign26070_e30308_d_n5, assign26070_e30308_d_n6, assign26070_e30308_d_n7, assign26070_e30308_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard491 == 0.0)) && (var_guard492 == 0.0)) {
        let assign26070_e30302: f64 = (var_alphaav * p.p860);
        let assign26070_e30303: f64 = (var_vav + assign26070_e30302);
        let assign26070_e30305: f64 = (assign26070_e30303 * var_slopebot);
        let assign26070_e30306: f64 = (var_fstopbot + assign26070_e30305);
        (assign26070_e30306, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign26070_e30308;
        var_fbreakdown_dn5 = assign26070_e30308_d_n5;
        var_fbreakdown_dn6 = assign26070_e30308_d_n6;
        var_fbreakdown_dn7 = assign26070_e30308_d_n7;
        var_fbreakdown_dn8 = assign26070_e30308_d_n8;

        let (assign26080_e30327, assign26080_e30327_d_n5, assign26080_e30327_d_n6, assign26080_e30327_d_n7, assign26080_e30327_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) {
        let assign26080_e30318: f64 = (var_id__blk219 + var_isrh);
        let assign26080_e30320: f64 = (assign26080_e30318 + var_itat);
        let assign26080_e30322: f64 = (assign26080_e30320 + var_ibbt);
        let assign26080_e30323: f64 = (p.p29 * assign26080_e30322);
        let assign26080_e30325: f64 = (assign26080_e30323 * var_fbreakdown);
        (assign26080_e30325, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign26080_e30323 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign26080_e30323 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign26080_e30323 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign26080_e30323 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign26080_e30327;
        var_ijunbot_dn5 = assign26080_e30327_d_n5;
        var_ijunbot_dn6 = assign26080_e30327_d_n6;
        var_ijunbot_dn7 = assign26080_e30327_d_n7;
        var_ijunbot_dn8 = assign26080_e30327_d_n8;

        let assign26090_e30330: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard494 = assign26090_e30330;

        let (assign26100_e30338, assign26100_e30338_d_n5, assign26100_e30338_d_n6, assign26100_e30338_d_n7, assign26100_e30338_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign26100_e30338;
        var_ijunsti_dn5 = assign26100_e30338_d_n5;
        var_ijunsti_dn6 = assign26100_e30338_d_n6;
        var_ijunsti_dn7 = assign26100_e30338_d_n7;
        var_ijunsti_dn8 = assign26100_e30338_d_n8;

        let (assign26110_e30349,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) {
        let assign26110_e30347: f64 = (var_idsatsti * var_idmult);
        (assign26110_e30347,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign26110_e30349;

        let assign26120_e30356: f64 = if ((p.p841 == 0.0) && (p.p846 == 0.0)) { 1.0 } else { 0.0 };
        var_guard495 = assign26120_e30356;

        let (assign26130_e30367, assign26130_e30367_d_n5, assign26130_e30367_d_n6, assign26130_e30367_d_n7, assign26130_e30367_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard495 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign26130_e30367;
        var_isrh_dn5 = assign26130_e30367_d_n5;
        var_isrh_dn6 = assign26130_e30367_d_n6;
        var_isrh_dn7 = assign26130_e30367_d_n7;
        var_isrh_dn8 = assign26130_e30367_d_n8;

        let (assign26140_e30381,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard495 == 0.0)) {
        let assign26140_e30379: f64 = (var_vbisti - var_vjsrh);
        (assign26140_e30379,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign26140_e30381;

        let (assign26150_e30400,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard495 == 0.0)) {
        let assign26150_e30395: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign26150_e30396: f64 = (1.0 - assign26150_e30395);
        let assign26150_e30397: f64 = (assign26150_e30396).sqrt();
        let assign26150_e30398: f64 = (1.0 - assign26150_e30397);
        (assign26150_e30398,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign26150_e30400;

        let assign26160_e30403: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        var_guard496 = assign26160_e30403;

        let (assign26170_e30417,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard495 == 0.0)) && (var_guard496 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign26170_e30417;

        let (assign26180_e30449,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard495 == 0.0)) && (var_guard496 == 0.0)) {
        let assign26180_e30432: f64 = (var_wsrhstep * var_wsrhstep);
        let assign26180_e30434: f64 = (var_wsrhstep).ln();
        let assign26180_e30435: f64 = (assign26180_e30432 * assign26180_e30434);
        let assign26180_e30438: f64 = (1.0 - var_wsrhstep);
        let assign26180_e30439: f64 = (assign26180_e30435 / assign26180_e30438);
        let assign26180_e30441: f64 = (assign26180_e30439 + var_wsrhstep);
        let assign26180_e30445: f64 = (2.0 * p.p832);
        let assign26180_e30446: f64 = (1.0 - assign26180_e30445);
        let assign26180_e30447: f64 = (assign26180_e30441 * assign26180_e30446);
        (assign26180_e30447,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign26180_e30449;

        let (assign26190_e30463,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard495 == 0.0)) {
        let assign26190_e30461: f64 = (var_wsrhstep + var_dwsrh);
        (assign26190_e30461,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign26190_e30463;

        let assign26200_e30466: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        var_guard497 = assign26200_e30466;

        let (assign26210_e30483, assign26210_e30483_d_n5, assign26210_e30483_d_n6, assign26210_e30483_d_n7, assign26210_e30483_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard495 == 0.0)) && (var_guard497 != 0.0)) {
        let assign26210_e30480: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign26210_e30481: f64 = (assign26210_e30480).sqrt();
        (assign26210_e30481, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26210_e30483;
        var_tmp_dn5 = assign26210_e30483_d_n5;
        var_tmp_dn6 = assign26210_e30483_d_n6;
        var_tmp_dn7 = assign26210_e30483_d_n7;
        var_tmp_dn8 = assign26210_e30483_d_n8;

        let (assign26220_e30502, assign26220_e30502_d_n5, assign26220_e30502_d_n6, assign26220_e30502_d_n7, assign26220_e30502_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard495 == 0.0)) && (var_guard497 == 0.0)) {
        let assign26220_e30498: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign26220_e30500: f64 = (assign26220_e30498).powf(p.p832);
        (assign26220_e30500, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26220_e30502;
        var_tmp_dn5 = assign26220_e30502_d_n5;
        var_tmp_dn6 = assign26220_e30502_d_n6;
        var_tmp_dn7 = assign26220_e30502_d_n7;
        var_tmp_dn8 = assign26220_e30502_d_n8;

        let (assign26230_e30516, assign26230_e30516_d_n5, assign26230_e30516_d_n6, assign26230_e30516_d_n7, assign26230_e30516_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard495 == 0.0)) {
        let assign26230_e30514: f64 = (var_wdepnulrsti * var_tmp);
        (assign26230_e30514, (var_wdepnulrsti * var_tmp_dn5), (var_wdepnulrsti * var_tmp_dn6), (var_wdepnulrsti * var_tmp_dn7), (var_wdepnulrsti * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign26230_e30516;
        var_wdep_dn5 = assign26230_e30516_d_n5;
        var_wdep_dn6 = assign26230_e30516_d_n6;
        var_wdep_dn7 = assign26230_e30516_d_n7;
        var_wdep_dn8 = assign26230_e30516_d_n8;

        let (assign26240_e30534, assign26240_e30534_d_n5, assign26240_e30534_d_n6, assign26240_e30534_d_n7, assign26240_e30534_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard495 == 0.0)) {
        let assign26240_e30529: f64 = (var_zinv - 1.0);
        let assign26240_e30531: f64 = (assign26240_e30529 * var_wdep);
        let assign26240_e30532: f64 = (var_ftdsti * assign26240_e30531);
        (assign26240_e30532, (var_ftdsti * (assign26240_e30529 * var_wdep_dn5)), (var_ftdsti * (assign26240_e30529 * var_wdep_dn6)), (var_ftdsti * (assign26240_e30529 * var_wdep_dn7)), (var_ftdsti * (assign26240_e30529 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign26240_e30534;
        var_asrh_dn5 = assign26240_e30534_d_n5;
        var_asrh_dn6 = assign26240_e30534_d_n6;
        var_asrh_dn7 = assign26240_e30534_d_n7;
        var_asrh_dn8 = assign26240_e30534_d_n8;

        let (assign26250_e30550, assign26250_e30550_d_n5, assign26250_e30550_d_n6, assign26250_e30550_d_n7, assign26250_e30550_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard495 == 0.0)) {
        let assign26250_e30547: f64 = (var_asrh * var_wsrh);
        let assign26250_e30548: f64 = (p.p841 * assign26250_e30547);
        (assign26250_e30548, (p.p841 * (var_asrh_dn5 * var_wsrh)), (p.p841 * (var_asrh_dn6 * var_wsrh)), (p.p841 * (var_asrh_dn7 * var_wsrh)), (p.p841 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign26250_e30550;
        var_isrh_dn5 = assign26250_e30550_d_n5;
        var_isrh_dn6 = assign26250_e30550_d_n6;
        var_isrh_dn7 = assign26250_e30550_d_n7;
        var_isrh_dn8 = assign26250_e30550_d_n8;

        let assign26260_e30553: f64 = if p.p846 == 0.0 { 1.0 } else { 0.0 };
        var_guard498 = assign26260_e30553;

        let (assign26270_e30564, assign26270_e30564_d_n5, assign26270_e30564_d_n6, assign26270_e30564_d_n7, assign26270_e30564_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign26270_e30564;
        var_itat_dn5 = assign26270_e30564_d_n5;
        var_itat_dn6 = assign26270_e30564_d_n6;
        var_itat_dn7 = assign26270_e30564_d_n7;
        var_itat_dn8 = assign26270_e30564_d_n8;

        let (assign26280_e30582, assign26280_e30582_d_n5, assign26280_e30582_d_n6, assign26280_e30582_d_n7, assign26280_e30582_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) {
        let assign26280_e30577: f64 = (var_wdep * var_one_minus_psti);
        let assign26280_e30579: f64 = (assign26280_e30577 / var_vbi_minus_vjsrh);
        let assign26280_e30580: f64 = (var_btatpartsti * assign26280_e30579);
        (assign26280_e30580, (var_btatpartsti * ((var_wdep_dn5 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn6 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn7 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn8 * var_one_minus_psti) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign26280_e30582;
        var_btat_dn5 = assign26280_e30582_d_n5;
        var_btat_dn6 = assign26280_e30582_d_n6;
        var_btat_dn7 = assign26280_e30582_d_n7;
        var_btat_dn8 = assign26280_e30582_d_n8;

        let (assign26290_e30598, assign26290_e30598_d_n5, assign26290_e30598_d_n6, assign26290_e30598_d_n7, assign26290_e30598_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) {
        let assign26290_e30594: f64 = (0.666666666666667 * var_atatsti);
        let assign26290_e30596: f64 = (assign26290_e30594 / var_btat);
        (assign26290_e30596, (-((assign26290_e30594 * var_btat_dn5) / (var_btat * var_btat))), (-((assign26290_e30594 * var_btat_dn6) / (var_btat * var_btat))), (-((assign26290_e30594 * var_btat_dn7) / (var_btat * var_btat))), (-((assign26290_e30594 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign26290_e30598;
        var_twoatatoverthreebtat_dn5 = assign26290_e30598_d_n5;
        var_twoatatoverthreebtat_dn6 = assign26290_e30598_d_n6;
        var_twoatatoverthreebtat_dn7 = assign26290_e30598_d_n7;
        var_twoatatoverthreebtat_dn8 = assign26290_e30598_d_n8;

        let (assign26300_e30612, assign26300_e30612_d_n5, assign26300_e30612_d_n6, assign26300_e30612_d_n7, assign26300_e30612_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) {
        let assign26300_e30610: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign26300_e30610, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign26300_e30612;
        var_umaxbeforelimiting_dn5 = assign26300_e30612_d_n5;
        var_umaxbeforelimiting_dn6 = assign26300_e30612_d_n6;
        var_umaxbeforelimiting_dn7 = assign26300_e30612_d_n7;
        var_umaxbeforelimiting_dn8 = assign26300_e30612_d_n8;

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
        *var_guard489_slot = var_guard489;
        *var_guard490_slot = var_guard490;
        *var_guard491_slot = var_guard491;
        *var_guard492_slot = var_guard492;
        *var_guard493_slot = var_guard493;
        *var_guard494_slot = var_guard494;
        *var_guard495_slot = var_guard495;
        *var_guard496_slot = var_guard496;
        *var_guard497_slot = var_guard497;
        *var_guard498_slot = var_guard498;
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

    pub(super) fn stamp_transient_block_50(
        p: &Parameters,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatsti: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn5: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_cerfc: f64,
        var_fbbtsti: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard494: f64,
        var_guard498: f64,
        var_one_over_one_minus_psti: f64,
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
        var_vbirstiinv: f64,
        var_wdepnulrinvsti: f64,
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
        var_guard499_slot: &mut f64,
        var_guard500_slot: &mut f64,
        var_guard501_slot: &mut f64,
        var_guard502_slot: &mut f64,
        var_guard503_slot: &mut f64,
        var_guard504_slot: &mut f64,
        var_guard505_slot: &mut f64,
        var_guard506_slot: &mut f64,
        var_guard507_slot: &mut f64,
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
        let mut var_guard499: f64 = *var_guard499_slot;
        let mut var_guard500: f64 = *var_guard500_slot;
        let mut var_guard501: f64 = *var_guard501_slot;
        let mut var_guard502: f64 = *var_guard502_slot;
        let mut var_guard503: f64 = *var_guard503_slot;
        let mut var_guard504: f64 = *var_guard504_slot;
        let mut var_guard505: f64 = *var_guard505_slot;
        let mut var_guard506: f64 = *var_guard506_slot;
        let mut var_guard507: f64 = *var_guard507_slot;
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

        let (assign26310_e30633, assign26310_e30633_d_n5, assign26310_e30633_d_n6, assign26310_e30633_d_n7, assign26310_e30633_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) {
        let assign26310_e30624: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign26310_e30627: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign26310_e30629: f64 = (assign26310_e30627 + 1.0);
        let assign26310_e30630: f64 = (assign26310_e30624 / assign26310_e30629);
        let assign26310_e30631: f64 = (assign26310_e30630).sqrt();
        (assign26310_e30631, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign26310_e30629) - (assign26310_e30624 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign26310_e30629 * assign26310_e30629)) / (2.0 * assign26310_e30631)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign26310_e30629) - (assign26310_e30624 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign26310_e30629 * assign26310_e30629)) / (2.0 * assign26310_e30631)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign26310_e30629) - (assign26310_e30624 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign26310_e30629 * assign26310_e30629)) / (2.0 * assign26310_e30631)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign26310_e30629) - (assign26310_e30624 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign26310_e30629 * assign26310_e30629)) / (2.0 * assign26310_e30631)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign26310_e30633;
        var_umax_dn5 = assign26310_e30633_d_n5;
        var_umax_dn6 = assign26310_e30633_d_n6;
        var_umax_dn7 = assign26310_e30633_d_n7;
        var_umax_dn8 = assign26310_e30633_d_n8;

        let (assign26320_e30646, assign26320_e30646_d_n5, assign26320_e30646_d_n6, assign26320_e30646_d_n7, assign26320_e30646_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) {
        let assign26320_e30644: f64 = (var_umax).sqrt();
        (assign26320_e30644, (var_umax_dn5 / (2.0 * assign26320_e30644)), (var_umax_dn6 / (2.0 * assign26320_e30644)), (var_umax_dn7 / (2.0 * assign26320_e30644)), (var_umax_dn8 / (2.0 * assign26320_e30644)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign26320_e30646;
        var_sqrtumax_dn5 = assign26320_e30646_d_n5;
        var_sqrtumax_dn6 = assign26320_e30646_d_n6;
        var_sqrtumax_dn7 = assign26320_e30646_d_n7;
        var_sqrtumax_dn8 = assign26320_e30646_d_n8;

        let (assign26330_e30660, assign26330_e30660_d_n5, assign26330_e30660_d_n6, assign26330_e30660_d_n7, assign26330_e30660_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) {
        let assign26330_e30658: f64 = (var_umax * var_sqrtumax);
        (assign26330_e30658, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign26330_e30660;
        var_umaxpoweronepointfive_dn5 = assign26330_e30660_d_n5;
        var_umaxpoweronepointfive_dn6 = assign26330_e30660_d_n6;
        var_umaxpoweronepointfive_dn7 = assign26330_e30660_d_n7;
        var_umaxpoweronepointfive_dn8 = assign26330_e30660_d_n8;

        let assign26340_e30662: f64 = (-p.p832);
        let assign26340_e30664: f64 = (assign26340_e30662 * var_one_over_one_minus_psti);
        let assign26340_e30666: f64 = (-1.0);
        let assign26340_e30667: f64 = if assign26340_e30664 == assign26340_e30666 { 1.0 } else { 0.0 };
        var_guard499 = assign26340_e30667;

        let (assign26350_e30687, assign26350_e30687_d_n5, assign26350_e30687_d_n6, assign26350_e30687_d_n7, assign26350_e30687_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) && (var_guard499 != 0.0)) {
        let assign26350_e30683: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign26350_e30684: f64 = (1.0 + assign26350_e30683);
        let assign26350_e30685: f64 = (1.0 / assign26350_e30684);
        (assign26350_e30685, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign26350_e30684 * assign26350_e30684))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign26350_e30684 * assign26350_e30684))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign26350_e30684 * assign26350_e30684))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign26350_e30684 * assign26350_e30684))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign26350_e30687;
        var_wgamma_dn5 = assign26350_e30687_d_n5;
        var_wgamma_dn6 = assign26350_e30687_d_n6;
        var_wgamma_dn7 = assign26350_e30687_d_n7;
        var_wgamma_dn8 = assign26350_e30687_d_n8;

        let (assign26360_e30711, assign26360_e30711_d_n5, assign26360_e30711_d_n6, assign26360_e30711_d_n7, assign26360_e30711_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) && (var_guard499 == 0.0)) {
        let assign26360_e30703: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign26360_e30704: f64 = (1.0 + assign26360_e30703);
        let assign26360_e30706: f64 = (-p.p832);
        let assign26360_e30708: f64 = (assign26360_e30706 * var_one_over_one_minus_psti);
        let assign26360_e30709: f64 = (assign26360_e30704).powf(assign26360_e30708);
        (assign26360_e30709, if 0.0 == 0.0 && ((assign26360_e30708) as f64).is_finite() && ((assign26360_e30708) as f64).fract() == 0.0 { if assign26360_e30708 == 0.0 { 0.0 } else { (assign26360_e30708 * ((assign26360_e30704).powf(assign26360_e30708 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign26360_e30709 * (assign26360_e30708 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign26360_e30704))) }, if 0.0 == 0.0 && ((assign26360_e30708) as f64).is_finite() && ((assign26360_e30708) as f64).fract() == 0.0 { if assign26360_e30708 == 0.0 { 0.0 } else { (assign26360_e30708 * ((assign26360_e30704).powf(assign26360_e30708 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign26360_e30709 * (assign26360_e30708 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign26360_e30704))) }, if 0.0 == 0.0 && ((assign26360_e30708) as f64).is_finite() && ((assign26360_e30708) as f64).fract() == 0.0 { if assign26360_e30708 == 0.0 { 0.0 } else { (assign26360_e30708 * ((assign26360_e30704).powf(assign26360_e30708 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign26360_e30709 * (assign26360_e30708 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign26360_e30704))) }, if 0.0 == 0.0 && ((assign26360_e30708) as f64).is_finite() && ((assign26360_e30708) as f64).fract() == 0.0 { if assign26360_e30708 == 0.0 { 0.0 } else { (assign26360_e30708 * ((assign26360_e30704).powf(assign26360_e30708 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign26360_e30709 * (assign26360_e30708 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign26360_e30704))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign26360_e30711;
        var_wgamma_dn5 = assign26360_e30711_d_n5;
        var_wgamma_dn6 = assign26360_e30711_d_n6;
        var_wgamma_dn7 = assign26360_e30711_d_n7;
        var_wgamma_dn8 = assign26360_e30711_d_n8;

        let (assign26370_e30729, assign26370_e30729_d_n5, assign26370_e30729_d_n6, assign26370_e30729_d_n7, assign26370_e30729_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) {
        let assign26370_e30723: f64 = (var_wsrh * var_wgamma);
        let assign26370_e30726: f64 = (var_wsrh + var_wgamma);
        let assign26370_e30727: f64 = (assign26370_e30723 / assign26370_e30726);
        (assign26370_e30727, ((((var_wsrh * var_wgamma_dn5) * assign26370_e30726) - (assign26370_e30723 * var_wgamma_dn5)) / (assign26370_e30726 * assign26370_e30726)), ((((var_wsrh * var_wgamma_dn6) * assign26370_e30726) - (assign26370_e30723 * var_wgamma_dn6)) / (assign26370_e30726 * assign26370_e30726)), ((((var_wsrh * var_wgamma_dn7) * assign26370_e30726) - (assign26370_e30723 * var_wgamma_dn7)) / (assign26370_e30726 * assign26370_e30726)), ((((var_wsrh * var_wgamma_dn8) * assign26370_e30726) - (assign26370_e30723 * var_wgamma_dn8)) / (assign26370_e30726 * assign26370_e30726)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign26370_e30729;
        var_wtat_dn5 = assign26370_e30729_d_n5;
        var_wtat_dn6 = assign26370_e30729_d_n6;
        var_wtat_dn7 = assign26370_e30729_d_n7;
        var_wtat_dn8 = assign26370_e30729_d_n8;

        let (assign26380_e30746, assign26380_e30746_d_n5, assign26380_e30746_d_n6, assign26380_e30746_d_n7, assign26380_e30746_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) {
        let assign26380_e30742: f64 = (var_btat / var_sqrtumax);
        let assign26380_e30743: f64 = (0.375 * assign26380_e30742);
        let assign26380_e30744: f64 = (assign26380_e30743).sqrt();
        (assign26380_e30744, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign26380_e30744)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign26380_e30744)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign26380_e30744)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign26380_e30744)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign26380_e30746;
        var_ktat_dn5 = assign26380_e30746_d_n5;
        var_ktat_dn6 = assign26380_e30746_d_n6;
        var_ktat_dn7 = assign26380_e30746_d_n7;
        var_ktat_dn8 = assign26380_e30746_d_n8;

        let (assign26390_e30764, assign26390_e30764_d_n5, assign26390_e30764_d_n6, assign26390_e30764_d_n7, assign26390_e30764_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) {
        let assign26390_e30759: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign26390_e30760: f64 = (2.0 * assign26390_e30759);
        let assign26390_e30762: f64 = (assign26390_e30760 - var_umax);
        (assign26390_e30762, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign26390_e30764;
        var_ltat_dn5 = assign26390_e30764_d_n5;
        var_ltat_dn6 = assign26390_e30764_d_n6;
        var_ltat_dn7 = assign26390_e30764_d_n7;
        var_ltat_dn8 = assign26390_e30764_d_n8;

        let (assign26400_e30790, assign26400_e30790_d_n5, assign26400_e30790_d_n6, assign26400_e30790_d_n7, assign26400_e30790_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) {
        let assign26400_e30776: f64 = (var_atatsti * var_twoatatoverthreebtat);
        let assign26400_e30778: f64 = (assign26400_e30776 * var_sqrtumax);
        let assign26400_e30781: f64 = (var_atatsti * var_umax);
        let assign26400_e30782: f64 = (assign26400_e30778 - assign26400_e30781);
        let assign26400_e30786: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign26400_e30787: f64 = (0.5 * assign26400_e30786);
        let assign26400_e30788: f64 = (assign26400_e30782 + assign26400_e30787);
        (assign26400_e30788, (((((var_atatsti * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign26400_e30776 * var_sqrtumax_dn5)) - (var_atatsti * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatsti * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign26400_e30776 * var_sqrtumax_dn6)) - (var_atatsti * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign26400_e30776 * var_sqrtumax_dn7)) - (var_atatsti * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign26400_e30776 * var_sqrtumax_dn8)) - (var_atatsti * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign26400_e30790;
        var_mtat_dn5 = assign26400_e30790_d_n5;
        var_mtat_dn6 = assign26400_e30790_d_n6;
        var_mtat_dn7 = assign26400_e30790_d_n7;
        var_mtat_dn8 = assign26400_e30790_d_n8;

        let (assign26410_e30806, assign26410_e30806_d_n5, assign26410_e30806_d_n6, assign26410_e30806_d_n7, assign26410_e30806_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) {
        let assign26410_e30802: f64 = (var_ltat - 1.0);
        let assign26410_e30804: f64 = (assign26410_e30802 * var_ktat);
        (assign26410_e30804, ((var_ltat_dn5 * var_ktat) + (assign26410_e30802 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign26410_e30802 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign26410_e30802 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign26410_e30802 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign26410_e30806;
        var_xerfc_dn5 = assign26410_e30806_d_n5;
        var_xerfc_dn6 = assign26410_e30806_d_n6;
        var_xerfc_dn7 = assign26410_e30806_d_n7;
        var_xerfc_dn8 = assign26410_e30806_d_n8;

        let (assign26420_e30820, assign26420_e30820_d_n5, assign26420_e30820_d_n6, assign26420_e30820_d_n7, assign26420_e30820_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) {
        let assign26420_e30818: f64 = (var_xerfc * var_xerfc);
        (assign26420_e30818, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign26420_e30820;
        var_ysq_dn5 = assign26420_e30820_d_n5;
        var_ysq_dn6 = assign26420_e30820_d_n6;
        var_ysq_dn7 = assign26420_e30820_d_n7;
        var_ysq_dn8 = assign26420_e30820_d_n8;

        let assign26430_e30823: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard500 = assign26430_e30823;

        let (assign26440_e30843, assign26440_e30843_d_n5, assign26440_e30843_d_n6, assign26440_e30843_d_n7, assign26440_e30843_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) && (var_guard500 != 0.0)) {
        let assign26440_e30839: f64 = (var_perfc * var_xerfc);
        let assign26440_e30840: f64 = (1.0 + assign26440_e30839);
        let assign26440_e30841: f64 = (1.0 / assign26440_e30840);
        (assign26440_e30841, (-((var_perfc * var_xerfc_dn5) / (assign26440_e30840 * assign26440_e30840))), (-((var_perfc * var_xerfc_dn6) / (assign26440_e30840 * assign26440_e30840))), (-((var_perfc * var_xerfc_dn7) / (assign26440_e30840 * assign26440_e30840))), (-((var_perfc * var_xerfc_dn8) / (assign26440_e30840 * assign26440_e30840))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign26440_e30843;
        var_terfc_dn5 = assign26440_e30843_d_n5;
        var_terfc_dn6 = assign26440_e30843_d_n6;
        var_terfc_dn7 = assign26440_e30843_d_n7;
        var_terfc_dn8 = assign26440_e30843_d_n8;

        let (assign26450_e30864, assign26450_e30864_d_n5, assign26450_e30864_d_n6, assign26450_e30864_d_n7, assign26450_e30864_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) && (var_guard500 == 0.0)) {
        let assign26450_e30860: f64 = (var_perfc * var_xerfc);
        let assign26450_e30861: f64 = (1.0 - assign26450_e30860);
        let assign26450_e30862: f64 = (1.0 / assign26450_e30861);
        (assign26450_e30862, (-((-(var_perfc * var_xerfc_dn5)) / (assign26450_e30861 * assign26450_e30861))), (-((-(var_perfc * var_xerfc_dn6)) / (assign26450_e30861 * assign26450_e30861))), (-((-(var_perfc * var_xerfc_dn7)) / (assign26450_e30861 * assign26450_e30861))), (-((-(var_perfc * var_xerfc_dn8)) / (assign26450_e30861 * assign26450_e30861))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign26450_e30864;
        var_terfc_dn5 = assign26450_e30864_d_n5;
        var_terfc_dn6 = assign26450_e30864_d_n6;
        var_terfc_dn7 = assign26450_e30864_d_n7;
        var_terfc_dn8 = assign26450_e30864_d_n8;

        let assign26460_e30866: f64 = (-var_ysq);
        let assign26460_e30868: f64 = (assign26460_e30866 + var_mtat);
        let assign26460_e30870: f64 = (-230.25850929940458);
        let assign26460_e30871: f64 = if assign26460_e30868 > assign26460_e30870 { 1.0 } else { 0.0 };
        var_guard501 = assign26460_e30871;

        let (assign26470_e30889, assign26470_e30889_d_n5, assign26470_e30889_d_n6, assign26470_e30889_d_n7, assign26470_e30889_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) && (var_guard501 != 0.0)) {
        let assign26470_e30884: f64 = (-var_ysq);
        let assign26470_e30886: f64 = (assign26470_e30884 + var_mtat);
        let assign26470_e30887: f64 = (assign26470_e30886).exp();
        (assign26470_e30887, (assign26470_e30887 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign26470_e30887 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign26470_e30887 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign26470_e30887 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26470_e30889;
        var_tmp_dn5 = assign26470_e30889_d_n5;
        var_tmp_dn6 = assign26470_e30889_d_n6;
        var_tmp_dn7 = assign26470_e30889_d_n7;
        var_tmp_dn8 = assign26470_e30889_d_n8;

        let (assign26480_e30938, assign26480_e30938_d_n5, assign26480_e30938_d_n6, assign26480_e30938_d_n7, assign26480_e30938_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) && (var_guard501 == 0.0)) {
        let assign26480_e30905: f64 = (-230.25850929940458);
        let assign26480_e30907: f64 = (-var_ysq);
        let assign26480_e30909: f64 = (assign26480_e30907 + var_mtat);
        let assign26480_e30910: f64 = (assign26480_e30905 - assign26480_e30909);
        let assign26480_e30914: f64 = (-230.25850929940458);
        let assign26480_e30916: f64 = (-var_ysq);
        let assign26480_e30918: f64 = (assign26480_e30916 + var_mtat);
        let assign26480_e30919: f64 = (assign26480_e30914 - assign26480_e30918);
        let assign26480_e30922: f64 = (-230.25850929940458);
        let assign26480_e30924: f64 = (-var_ysq);
        let assign26480_e30926: f64 = (assign26480_e30924 + var_mtat);
        let assign26480_e30927: f64 = (assign26480_e30922 - assign26480_e30926);
        let assign26480_e30929: f64 = (assign26480_e30927 * 0.3333333333333333);
        let assign26480_e30930: f64 = (1.0 + assign26480_e30929);
        let assign26480_e30931: f64 = (assign26480_e30919 * assign26480_e30930);
        let assign26480_e30932: f64 = (0.5 * assign26480_e30931);
        let assign26480_e30933: f64 = (1.0 + assign26480_e30932);
        let assign26480_e30934: f64 = (assign26480_e30910 * assign26480_e30933);
        let assign26480_e30935: f64 = (1.0 + assign26480_e30934);
        let assign26480_e30936: f64 = (1e-100 / assign26480_e30935);
        (assign26480_e30936, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign26480_e30933) + (assign26480_e30910 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign26480_e30930) + (assign26480_e30919 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign26480_e30935 * assign26480_e30935))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign26480_e30933) + (assign26480_e30910 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign26480_e30930) + (assign26480_e30919 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign26480_e30935 * assign26480_e30935))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign26480_e30933) + (assign26480_e30910 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign26480_e30930) + (assign26480_e30919 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign26480_e30935 * assign26480_e30935))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign26480_e30933) + (assign26480_e30910 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign26480_e30930) + (assign26480_e30919 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign26480_e30935 * assign26480_e30935))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26480_e30938;
        var_tmp_dn5 = assign26480_e30938_d_n5;
        var_tmp_dn6 = assign26480_e30938_d_n6;
        var_tmp_dn7 = assign26480_e30938_d_n7;
        var_tmp_dn8 = assign26480_e30938_d_n8;

        let (assign26490_e30968, assign26490_e30968_d_n5, assign26490_e30968_d_n6, assign26490_e30968_d_n7, assign26490_e30968_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) {
        let assign26490_e30950: f64 = (0.29214664 * var_terfc);
        let assign26490_e30954: f64 = (var_terfc * var_terfc);
        let assign26490_e30955: f64 = (var_berfc * assign26490_e30954);
        let assign26490_e30956: f64 = (assign26490_e30950 + assign26490_e30955);
        let assign26490_e30960: f64 = (var_terfc * var_terfc);
        let assign26490_e30962: f64 = (assign26490_e30960 * var_terfc);
        let assign26490_e30963: f64 = (var_cerfc * assign26490_e30962);
        let assign26490_e30964: f64 = (assign26490_e30956 + assign26490_e30963);
        let assign26490_e30966: f64 = (assign26490_e30964 * var_tmp);
        (assign26490_e30966, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign26490_e30960 * var_terfc_dn5)))) * var_tmp) + (assign26490_e30964 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign26490_e30960 * var_terfc_dn6)))) * var_tmp) + (assign26490_e30964 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign26490_e30960 * var_terfc_dn7)))) * var_tmp) + (assign26490_e30964 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign26490_e30960 * var_terfc_dn8)))) * var_tmp) + (assign26490_e30964 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign26490_e30968;
        var_erfcpos_dn5 = assign26490_e30968_d_n5;
        var_erfcpos_dn6 = assign26490_e30968_d_n6;
        var_erfcpos_dn7 = assign26490_e30968_d_n7;
        var_erfcpos_dn8 = assign26490_e30968_d_n8;

        let assign26500_e30971: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard502 = assign26500_e30971;

        let (assign26510_e30985, assign26510_e30985_d_n5, assign26510_e30985_d_n6, assign26510_e30985_d_n7, assign26510_e30985_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) && (var_guard502 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign26510_e30985;
        var_erfctimesexpmtat_dn5 = assign26510_e30985_d_n5;
        var_erfctimesexpmtat_dn6 = assign26510_e30985_d_n6;
        var_erfctimesexpmtat_dn7 = assign26510_e30985_d_n7;
        var_erfctimesexpmtat_dn8 = assign26510_e30985_d_n8;

        let assign26520_e30988: f64 = (-230.25850929940458);
        let assign26520_e30989: f64 = if var_mtat > assign26520_e30988 { 1.0 } else { 0.0 };
        var_guard503 = assign26520_e30989;

        let (assign26530_e31007, assign26530_e31007_d_n5, assign26530_e31007_d_n6, assign26530_e31007_d_n7, assign26530_e31007_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) && (var_guard502 == 0.0)) && (var_guard503 != 0.0)) {
        let assign26530_e31005: f64 = (var_mtat).exp();
        (assign26530_e31005, (assign26530_e31005 * var_mtat_dn5), (assign26530_e31005 * var_mtat_dn6), (assign26530_e31005 * var_mtat_dn7), (assign26530_e31005 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26530_e31007;
        var_tmp_dn5 = assign26530_e31007_d_n5;
        var_tmp_dn6 = assign26530_e31007_d_n6;
        var_tmp_dn7 = assign26530_e31007_d_n7;
        var_tmp_dn8 = assign26530_e31007_d_n8;

        let (assign26540_e31050, assign26540_e31050_d_n5, assign26540_e31050_d_n6, assign26540_e31050_d_n7, assign26540_e31050_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) && (var_guard502 == 0.0)) && (var_guard503 == 0.0)) {
        let assign26540_e31026: f64 = (-230.25850929940458);
        let assign26540_e31028: f64 = (assign26540_e31026 - var_mtat);
        let assign26540_e31032: f64 = (-230.25850929940458);
        let assign26540_e31034: f64 = (assign26540_e31032 - var_mtat);
        let assign26540_e31037: f64 = (-230.25850929940458);
        let assign26540_e31039: f64 = (assign26540_e31037 - var_mtat);
        let assign26540_e31041: f64 = (assign26540_e31039 * 0.3333333333333333);
        let assign26540_e31042: f64 = (1.0 + assign26540_e31041);
        let assign26540_e31043: f64 = (assign26540_e31034 * assign26540_e31042);
        let assign26540_e31044: f64 = (0.5 * assign26540_e31043);
        let assign26540_e31045: f64 = (1.0 + assign26540_e31044);
        let assign26540_e31046: f64 = (assign26540_e31028 * assign26540_e31045);
        let assign26540_e31047: f64 = (1.0 + assign26540_e31046);
        let assign26540_e31048: f64 = (1e-100 / assign26540_e31047);
        (assign26540_e31048, (-((1e-100 * (((-var_mtat_dn5) * assign26540_e31045) + (assign26540_e31028 * (0.5 * (((-var_mtat_dn5) * assign26540_e31042) + (assign26540_e31034 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign26540_e31047 * assign26540_e31047))), (-((1e-100 * (((-var_mtat_dn6) * assign26540_e31045) + (assign26540_e31028 * (0.5 * (((-var_mtat_dn6) * assign26540_e31042) + (assign26540_e31034 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign26540_e31047 * assign26540_e31047))), (-((1e-100 * (((-var_mtat_dn7) * assign26540_e31045) + (assign26540_e31028 * (0.5 * (((-var_mtat_dn7) * assign26540_e31042) + (assign26540_e31034 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign26540_e31047 * assign26540_e31047))), (-((1e-100 * (((-var_mtat_dn8) * assign26540_e31045) + (assign26540_e31028 * (0.5 * (((-var_mtat_dn8) * assign26540_e31042) + (assign26540_e31034 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign26540_e31047 * assign26540_e31047))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26540_e31050;
        var_tmp_dn5 = assign26540_e31050_d_n5;
        var_tmp_dn6 = assign26540_e31050_d_n6;
        var_tmp_dn7 = assign26540_e31050_d_n7;
        var_tmp_dn8 = assign26540_e31050_d_n8;

        let (assign26550_e31069, assign26550_e31069_d_n5, assign26550_e31069_d_n6, assign26550_e31069_d_n7, assign26550_e31069_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) && (var_guard502 == 0.0)) {
        let assign26550_e31065: f64 = (2.0 * var_tmp);
        let assign26550_e31067: f64 = (assign26550_e31065 - var_erfcpos);
        (assign26550_e31067, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign26550_e31069;
        var_erfctimesexpmtat_dn5 = assign26550_e31069_d_n5;
        var_erfctimesexpmtat_dn6 = assign26550_e31069_d_n6;
        var_erfctimesexpmtat_dn7 = assign26550_e31069_d_n7;
        var_erfctimesexpmtat_dn8 = assign26550_e31069_d_n8;

        let (assign26560_e31089, assign26560_e31089_d_n5, assign26560_e31089_d_n6, assign26560_e31089_d_n7, assign26560_e31089_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) {
        let assign26560_e31081: f64 = (1.772453850905516 * 0.5);
        let assign26560_e31084: f64 = (var_atatsti * var_erfctimesexpmtat);
        let assign26560_e31086: f64 = (assign26560_e31084 / var_ktat);
        let assign26560_e31087: f64 = (assign26560_e31081 * assign26560_e31086);
        (assign26560_e31087, (assign26560_e31081 * ((((var_atatsti * var_erfctimesexpmtat_dn5) * var_ktat) - (assign26560_e31084 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign26560_e31081 * ((((var_atatsti * var_erfctimesexpmtat_dn6) * var_ktat) - (assign26560_e31084 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign26560_e31081 * ((((var_atatsti * var_erfctimesexpmtat_dn7) * var_ktat) - (assign26560_e31084 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign26560_e31081 * ((((var_atatsti * var_erfctimesexpmtat_dn8) * var_ktat) - (assign26560_e31084 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign26560_e31089;
        var_gammamax_dn5 = assign26560_e31089_d_n5;
        var_gammamax_dn6 = assign26560_e31089_d_n6;
        var_gammamax_dn7 = assign26560_e31089_d_n7;
        var_gammamax_dn8 = assign26560_e31089_d_n8;

        let (assign26570_e31107, assign26570_e31107_d_n5, assign26570_e31107_d_n6, assign26570_e31107_d_n7, assign26570_e31107_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard498 == 0.0)) {
        let assign26570_e31102: f64 = (var_asrh * var_gammamax);
        let assign26570_e31104: f64 = (assign26570_e31102 * var_wtat);
        let assign26570_e31105: f64 = (p.p846 * assign26570_e31104);
        (assign26570_e31105, (p.p846 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign26570_e31102 * var_wtat_dn5))), (p.p846 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign26570_e31102 * var_wtat_dn6))), (p.p846 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign26570_e31102 * var_wtat_dn7))), (p.p846 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign26570_e31102 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign26570_e31107;
        var_itat_dn5 = assign26570_e31107_d_n5;
        var_itat_dn6 = assign26570_e31107_d_n6;
        var_itat_dn7 = assign26570_e31107_d_n7;
        var_itat_dn8 = assign26570_e31107_d_n8;

        let assign26580_e31110: f64 = if p.p852 == 0.0 { 1.0 } else { 0.0 };
        var_guard504 = assign26580_e31110;

        let (assign26590_e31121, assign26590_e31121_d_n5, assign26590_e31121_d_n6, assign26590_e31121_d_n7, assign26590_e31121_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard504 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign26590_e31121;
        var_ibbt_dn5 = assign26590_e31121_d_n5;
        var_ibbt_dn6 = assign26590_e31121_d_n6;
        var_ibbt_dn7 = assign26590_e31121_d_n7;
        var_ibbt_dn8 = assign26590_e31121_d_n8;

        let assign26600_e31124: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        var_guard505 = assign26600_e31124;

        let (assign26610_e31143, assign26610_e31143_d_n5, assign26610_e31143_d_n6, assign26610_e31143_d_n7, assign26610_e31143_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard504 == 0.0)) && (var_guard505 != 0.0)) {
        let assign26610_e31138: f64 = (p.p829 - var_vbbt);
        let assign26610_e31140: f64 = (assign26610_e31138 * var_vbirstiinv);
        let assign26610_e31141: f64 = (assign26610_e31140).sqrt();
        (assign26610_e31141, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26610_e31143;
        var_tmp_dn5 = assign26610_e31143_d_n5;
        var_tmp_dn6 = assign26610_e31143_d_n6;
        var_tmp_dn7 = assign26610_e31143_d_n7;
        var_tmp_dn8 = assign26610_e31143_d_n8;

        let (assign26620_e31164, assign26620_e31164_d_n5, assign26620_e31164_d_n6, assign26620_e31164_d_n7, assign26620_e31164_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard504 == 0.0)) && (var_guard505 == 0.0)) {
        let assign26620_e31158: f64 = (p.p829 - var_vbbt);
        let assign26620_e31160: f64 = (assign26620_e31158 * var_vbirstiinv);
        let assign26620_e31162: f64 = (assign26620_e31160).powf(p.p832);
        (assign26620_e31162, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26620_e31164;
        var_tmp_dn5 = assign26620_e31164_d_n5;
        var_tmp_dn6 = assign26620_e31164_d_n6;
        var_tmp_dn7 = assign26620_e31164_d_n7;
        var_tmp_dn8 = assign26620_e31164_d_n8;

        let (assign26630_e31184, assign26630_e31184_d_n5, assign26630_e31184_d_n6, assign26630_e31184_d_n7, assign26630_e31184_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard504 == 0.0)) {
        let assign26630_e31177: f64 = (p.p829 - var_vbbt);
        let assign26630_e31179: f64 = (assign26630_e31177 * var_wdepnulrinvsti);
        let assign26630_e31181: f64 = (assign26630_e31179 / var_tmp);
        let assign26630_e31182: f64 = (var_one_over_one_minus_psti * assign26630_e31181);
        (assign26630_e31182, (var_one_over_one_minus_psti * (-((assign26630_e31179 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign26630_e31179 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign26630_e31179 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign26630_e31179 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign26630_e31184;
        var_fmaxr_dn5 = assign26630_e31184_d_n5;
        var_fmaxr_dn6 = assign26630_e31184_d_n6;
        var_fmaxr_dn7 = assign26630_e31184_d_n7;
        var_fmaxr_dn8 = assign26630_e31184_d_n8;

        let assign26640_e31186: f64 = (-var_fbbtsti);
        let assign26640_e31188: f64 = (assign26640_e31186 / var_fmaxr);
        let assign26640_e31189: f64 = (assign26640_e31188).abs();
        let assign26640_e31191: f64 = if assign26640_e31189 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard506 = assign26640_e31191;

        let (assign26650_e31209, assign26650_e31209_d_n5, assign26650_e31209_d_n6, assign26650_e31209_d_n7, assign26650_e31209_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard504 == 0.0)) && (var_guard506 != 0.0)) {
        let assign26650_e31204: f64 = (-var_fbbtsti);
        let assign26650_e31206: f64 = (assign26650_e31204 / var_fmaxr);
        let assign26650_e31207: f64 = (assign26650_e31206).exp();
        (assign26650_e31207, (assign26650_e31207 * (-((assign26650_e31204 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign26650_e31207 * (-((assign26650_e31204 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign26650_e31207 * (-((assign26650_e31204 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign26650_e31207 * (-((assign26650_e31204 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26650_e31209;
        var_tmp_dn5 = assign26650_e31209_d_n5;
        var_tmp_dn6 = assign26650_e31209_d_n6;
        var_tmp_dn7 = assign26650_e31209_d_n7;
        var_tmp_dn8 = assign26650_e31209_d_n8;

        let assign26660_e31211: f64 = (-var_fbbtsti);
        let assign26660_e31213: f64 = (assign26660_e31211 / var_fmaxr);
        let assign26660_e31215: f64 = if assign26660_e31213 < 0.0 { 1.0 } else { 0.0 };
        var_guard507 = assign26660_e31215;

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
        *var_guard499_slot = var_guard499;
        *var_guard500_slot = var_guard500;
        *var_guard501_slot = var_guard501;
        *var_guard502_slot = var_guard502;
        *var_guard503_slot = var_guard503;
        *var_guard504_slot = var_guard504;
        *var_guard505_slot = var_guard505;
        *var_guard506_slot = var_guard506;
        *var_guard507_slot = var_guard507;
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

    pub(super) fn stamp_transient_block_51(
        p: &Parameters,
        var_alphaav: f64,
        var_atatgat: f64,
        var_btatpartgat: f64,
        var_fbbtsti: f64,
        var_fmaxr: f64,
        var_fmaxr_dn5: f64,
        var_fmaxr_dn6: f64,
        var_fmaxr_dn7: f64,
        var_fmaxr_dn8: f64,
        var_fstopsti: f64,
        var_ftdgat: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard494: f64,
        var_guard504: f64,
        var_guard506: f64,
        var_guard507: f64,
        var_idmult: f64,
        var_idsatgat: f64,
        var_lgsource_i: f64,
        var_one_minus_pgat: f64,
        var_one_over_one_minus_pgat: f64,
        var_slopesti: f64,
        var_two_psistar: f64,
        var_v5: f64,
        var_vav: f64,
        var_vbigat: f64,
        var_vbirgatinv: f64,
        var_vbrinvsti: f64,
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
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_guard508_slot: &mut f64,
        var_guard509_slot: &mut f64,
        var_guard510_slot: &mut f64,
        var_guard511_slot: &mut f64,
        var_guard512_slot: &mut f64,
        var_guard513_slot: &mut f64,
        var_guard514_slot: &mut f64,
        var_guard515_slot: &mut f64,
        var_guard516_slot: &mut f64,
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
        let mut var_guard508: f64 = *var_guard508_slot;
        let mut var_guard509: f64 = *var_guard509_slot;
        let mut var_guard510: f64 = *var_guard510_slot;
        let mut var_guard511: f64 = *var_guard511_slot;
        let mut var_guard512: f64 = *var_guard512_slot;
        let mut var_guard513: f64 = *var_guard513_slot;
        let mut var_guard514: f64 = *var_guard514_slot;
        let mut var_guard515: f64 = *var_guard515_slot;
        let mut var_guard516: f64 = *var_guard516_slot;
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

        let (assign26670_e31266, assign26670_e31266_d_n5, assign26670_e31266_d_n6, assign26670_e31266_d_n7, assign26670_e31266_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard504 == 0.0)) && (var_guard506 == 0.0)) && (var_guard507 != 0.0)) {
        let assign26670_e31233: f64 = (-230.25850929940458);
        let assign26670_e31235: f64 = (-var_fbbtsti);
        let assign26670_e31237: f64 = (assign26670_e31235 / var_fmaxr);
        let assign26670_e31238: f64 = (assign26670_e31233 - assign26670_e31237);
        let assign26670_e31242: f64 = (-230.25850929940458);
        let assign26670_e31244: f64 = (-var_fbbtsti);
        let assign26670_e31246: f64 = (assign26670_e31244 / var_fmaxr);
        let assign26670_e31247: f64 = (assign26670_e31242 - assign26670_e31246);
        let assign26670_e31250: f64 = (-230.25850929940458);
        let assign26670_e31252: f64 = (-var_fbbtsti);
        let assign26670_e31254: f64 = (assign26670_e31252 / var_fmaxr);
        let assign26670_e31255: f64 = (assign26670_e31250 - assign26670_e31254);
        let assign26670_e31257: f64 = (assign26670_e31255 * 0.3333333333333333);
        let assign26670_e31258: f64 = (1.0 + assign26670_e31257);
        let assign26670_e31259: f64 = (assign26670_e31247 * assign26670_e31258);
        let assign26670_e31260: f64 = (0.5 * assign26670_e31259);
        let assign26670_e31261: f64 = (1.0 + assign26670_e31260);
        let assign26670_e31262: f64 = (assign26670_e31238 * assign26670_e31261);
        let assign26670_e31263: f64 = (1.0 + assign26670_e31262);
        let assign26670_e31264: f64 = (1e-100 / assign26670_e31263);
        (assign26670_e31264, (-((1e-100 * (((-(-((assign26670_e31235 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign26670_e31261) + (assign26670_e31238 * (0.5 * (((-(-((assign26670_e31244 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign26670_e31258) + (assign26670_e31247 * ((-(-((assign26670_e31252 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign26670_e31263 * assign26670_e31263))), (-((1e-100 * (((-(-((assign26670_e31235 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign26670_e31261) + (assign26670_e31238 * (0.5 * (((-(-((assign26670_e31244 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign26670_e31258) + (assign26670_e31247 * ((-(-((assign26670_e31252 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign26670_e31263 * assign26670_e31263))), (-((1e-100 * (((-(-((assign26670_e31235 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign26670_e31261) + (assign26670_e31238 * (0.5 * (((-(-((assign26670_e31244 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign26670_e31258) + (assign26670_e31247 * ((-(-((assign26670_e31252 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign26670_e31263 * assign26670_e31263))), (-((1e-100 * (((-(-((assign26670_e31235 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign26670_e31261) + (assign26670_e31238 * (0.5 * (((-(-((assign26670_e31244 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign26670_e31258) + (assign26670_e31247 * ((-(-((assign26670_e31252 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign26670_e31263 * assign26670_e31263))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26670_e31266;
        var_tmp_dn5 = assign26670_e31266_d_n5;
        var_tmp_dn6 = assign26670_e31266_d_n6;
        var_tmp_dn7 = assign26670_e31266_d_n7;
        var_tmp_dn8 = assign26670_e31266_d_n8;

        let (assign26680_e31315, assign26680_e31315_d_n5, assign26680_e31315_d_n6, assign26680_e31315_d_n7, assign26680_e31315_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard504 == 0.0)) && (var_guard506 == 0.0)) && (var_guard507 == 0.0)) {
        let assign26680_e31285: f64 = (-var_fbbtsti);
        let assign26680_e31287: f64 = (assign26680_e31285 / var_fmaxr);
        let assign26680_e31289: f64 = (assign26680_e31287 - 230.25850929940458);
        let assign26680_e31293: f64 = (-var_fbbtsti);
        let assign26680_e31295: f64 = (assign26680_e31293 / var_fmaxr);
        let assign26680_e31297: f64 = (assign26680_e31295 - 230.25850929940458);
        let assign26680_e31300: f64 = (-var_fbbtsti);
        let assign26680_e31302: f64 = (assign26680_e31300 / var_fmaxr);
        let assign26680_e31304: f64 = (assign26680_e31302 - 230.25850929940458);
        let assign26680_e31306: f64 = (assign26680_e31304 * 0.3333333333333333);
        let assign26680_e31307: f64 = (1.0 + assign26680_e31306);
        let assign26680_e31308: f64 = (assign26680_e31297 * assign26680_e31307);
        let assign26680_e31309: f64 = (0.5 * assign26680_e31308);
        let assign26680_e31310: f64 = (1.0 + assign26680_e31309);
        let assign26680_e31311: f64 = (assign26680_e31289 * assign26680_e31310);
        let assign26680_e31312: f64 = (1.0 + assign26680_e31311);
        let assign26680_e31313: f64 = (1e100 * assign26680_e31312);
        (assign26680_e31313, (1e100 * (((-((assign26680_e31285 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign26680_e31310) + (assign26680_e31289 * (0.5 * (((-((assign26680_e31293 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign26680_e31307) + (assign26680_e31297 * ((-((assign26680_e31300 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign26680_e31285 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign26680_e31310) + (assign26680_e31289 * (0.5 * (((-((assign26680_e31293 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign26680_e31307) + (assign26680_e31297 * ((-((assign26680_e31300 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign26680_e31285 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign26680_e31310) + (assign26680_e31289 * (0.5 * (((-((assign26680_e31293 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign26680_e31307) + (assign26680_e31297 * ((-((assign26680_e31300 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign26680_e31285 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign26680_e31310) + (assign26680_e31289 * (0.5 * (((-((assign26680_e31293 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign26680_e31307) + (assign26680_e31297 * ((-((assign26680_e31300 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26680_e31315;
        var_tmp_dn5 = assign26680_e31315_d_n5;
        var_tmp_dn6 = assign26680_e31315_d_n6;
        var_tmp_dn7 = assign26680_e31315_d_n7;
        var_tmp_dn8 = assign26680_e31315_d_n8;

        let (assign26690_e31335, assign26690_e31335_d_n5, assign26690_e31335_d_n6, assign26690_e31335_d_n7, assign26690_e31335_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard504 == 0.0)) {
        let assign26690_e31328: f64 = (var_v5 * var_fmaxr);
        let assign26690_e31330: f64 = (assign26690_e31328 * var_fmaxr);
        let assign26690_e31332: f64 = (assign26690_e31330 * var_tmp);
        let assign26690_e31333: f64 = (p.p852 * assign26690_e31332);
        (assign26690_e31333, (p.p852 * (((((var_v5 * var_fmaxr_dn5) * var_fmaxr) + (assign26690_e31328 * var_fmaxr_dn5)) * var_tmp) + (assign26690_e31330 * var_tmp_dn5))), (p.p852 * (((((var_v5 * var_fmaxr_dn6) * var_fmaxr) + (assign26690_e31328 * var_fmaxr_dn6)) * var_tmp) + (assign26690_e31330 * var_tmp_dn6))), (p.p852 * (((((var_v5 * var_fmaxr_dn7) * var_fmaxr) + (assign26690_e31328 * var_fmaxr_dn7)) * var_tmp) + (assign26690_e31330 * var_tmp_dn7))), (p.p852 * (((((var_v5 * var_fmaxr_dn8) * var_fmaxr) + (assign26690_e31328 * var_fmaxr_dn8)) * var_tmp) + (assign26690_e31330 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign26690_e31335;
        var_ibbt_dn5 = assign26690_e31335_d_n5;
        var_ibbt_dn6 = assign26690_e31335_d_n6;
        var_ibbt_dn7 = assign26690_e31335_d_n7;
        var_ibbt_dn8 = assign26690_e31335_d_n8;

        let assign26700_e31338: f64 = if p.p861 > 1000.0 { 1.0 } else { 0.0 };
        var_guard508 = assign26700_e31338;

        let (assign26710_e31349, assign26710_e31349_d_n5, assign26710_e31349_d_n6, assign26710_e31349_d_n7, assign26710_e31349_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard508 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign26710_e31349;
        var_fbreakdown_dn5 = assign26710_e31349_d_n5;
        var_fbreakdown_dn6 = assign26710_e31349_d_n6;
        var_fbreakdown_dn7 = assign26710_e31349_d_n7;
        var_fbreakdown_dn8 = assign26710_e31349_d_n8;

        let assign26720_e31352: f64 = (-var_alphaav);
        let assign26720_e31354: f64 = (assign26720_e31352 * p.p861);
        let assign26720_e31355: f64 = if var_vav > assign26720_e31354 { 1.0 } else { 0.0 };
        var_guard509 = assign26720_e31355;

        let assign26730_e31358: f64 = if p.p864 == 4.0 { 1.0 } else { 0.0 };
        var_guard510 = assign26730_e31358;

        let (assign26740_e31388, assign26740_e31388_d_n5, assign26740_e31388_d_n6, assign26740_e31388_d_n7, assign26740_e31388_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard508 == 0.0)) && (var_guard509 != 0.0)) && (var_guard510 != 0.0)) {
        let assign26740_e31374: f64 = (var_vav * var_vbrinvsti);
        let assign26740_e31377: f64 = (var_vav * var_vbrinvsti);
        let assign26740_e31378: f64 = (assign26740_e31374 * assign26740_e31377);
        let assign26740_e31381: f64 = (var_vav * var_vbrinvsti);
        let assign26740_e31382: f64 = (assign26740_e31378 * assign26740_e31381);
        let assign26740_e31385: f64 = (var_vav * var_vbrinvsti);
        let assign26740_e31386: f64 = (assign26740_e31382 * assign26740_e31385);
        (assign26740_e31386, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26740_e31388;
        var_tmp_dn5 = assign26740_e31388_d_n5;
        var_tmp_dn6 = assign26740_e31388_d_n6;
        var_tmp_dn7 = assign26740_e31388_d_n7;
        var_tmp_dn8 = assign26740_e31388_d_n8;

        let (assign26750_e31410, assign26750_e31410_d_n5, assign26750_e31410_d_n6, assign26750_e31410_d_n7, assign26750_e31410_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard508 == 0.0)) && (var_guard509 != 0.0)) && (var_guard510 == 0.0)) {
        let assign26750_e31405: f64 = (var_vav * var_vbrinvsti);
        let assign26750_e31406: f64 = (assign26750_e31405).abs();
        let assign26750_e31408: f64 = (assign26750_e31406).powf(p.p864);
        (assign26750_e31408, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26750_e31410;
        var_tmp_dn5 = assign26750_e31410_d_n5;
        var_tmp_dn6 = assign26750_e31410_d_n6;
        var_tmp_dn7 = assign26750_e31410_d_n7;
        var_tmp_dn8 = assign26750_e31410_d_n8;

        let (assign26760_e31428, assign26760_e31428_d_n5, assign26760_e31428_d_n6, assign26760_e31428_d_n7, assign26760_e31428_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard508 == 0.0)) && (var_guard509 != 0.0)) {
        let assign26760_e31425: f64 = (1.0 - var_tmp);
        let assign26760_e31426: f64 = (1.0 / assign26760_e31425);
        (assign26760_e31426, (-((-var_tmp_dn5) / (assign26760_e31425 * assign26760_e31425))), (-((-var_tmp_dn6) / (assign26760_e31425 * assign26760_e31425))), (-((-var_tmp_dn7) / (assign26760_e31425 * assign26760_e31425))), (-((-var_tmp_dn8) / (assign26760_e31425 * assign26760_e31425))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign26760_e31428;
        var_fbreakdown_dn5 = assign26760_e31428_d_n5;
        var_fbreakdown_dn6 = assign26760_e31428_d_n6;
        var_fbreakdown_dn7 = assign26760_e31428_d_n7;
        var_fbreakdown_dn8 = assign26760_e31428_d_n8;

        let (assign26770_e31451, assign26770_e31451_d_n5, assign26770_e31451_d_n6, assign26770_e31451_d_n7, assign26770_e31451_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) && (var_guard508 == 0.0)) && (var_guard509 == 0.0)) {
        let assign26770_e31445: f64 = (var_alphaav * p.p861);
        let assign26770_e31446: f64 = (var_vav + assign26770_e31445);
        let assign26770_e31448: f64 = (assign26770_e31446 * var_slopesti);
        let assign26770_e31449: f64 = (var_fstopsti + assign26770_e31448);
        (assign26770_e31449, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign26770_e31451;
        var_fbreakdown_dn5 = assign26770_e31451_d_n5;
        var_fbreakdown_dn6 = assign26770_e31451_d_n6;
        var_fbreakdown_dn7 = assign26770_e31451_d_n7;
        var_fbreakdown_dn8 = assign26770_e31451_d_n8;

        let (assign26780_e31470, assign26780_e31470_d_n5, assign26780_e31470_d_n6, assign26780_e31470_d_n7, assign26780_e31470_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard494 == 0.0)) {
        let assign26780_e31461: f64 = (var_id__blk219 + var_isrh);
        let assign26780_e31463: f64 = (assign26780_e31461 + var_itat);
        let assign26780_e31465: f64 = (assign26780_e31463 + var_ibbt);
        let assign26780_e31466: f64 = (p.p29 * assign26780_e31465);
        let assign26780_e31468: f64 = (assign26780_e31466 * var_fbreakdown);
        (assign26780_e31468, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign26780_e31466 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign26780_e31466 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign26780_e31466 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign26780_e31466 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign26780_e31470;
        var_ijunsti_dn5 = assign26780_e31470_d_n5;
        var_ijunsti_dn6 = assign26780_e31470_d_n6;
        var_ijunsti_dn7 = assign26780_e31470_d_n7;
        var_ijunsti_dn8 = assign26780_e31470_d_n8;

        let assign26790_e31473: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard511 = assign26790_e31473;

        let (assign26800_e31481, assign26800_e31481_d_n5, assign26800_e31481_d_n6, assign26800_e31481_d_n7, assign26800_e31481_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign26800_e31481;
        var_ijungat_dn5 = assign26800_e31481_d_n5;
        var_ijungat_dn6 = assign26800_e31481_d_n6;
        var_ijungat_dn7 = assign26800_e31481_d_n7;
        var_ijungat_dn8 = assign26800_e31481_d_n8;

        let (assign26810_e31492,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) {
        let assign26810_e31490: f64 = (var_idsatgat * var_idmult);
        (assign26810_e31490,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign26810_e31492;

        let assign26820_e31499: f64 = if ((p.p842 == 0.0) && (p.p847 == 0.0)) { 1.0 } else { 0.0 };
        var_guard512 = assign26820_e31499;

        let (assign26830_e31510, assign26830_e31510_d_n5, assign26830_e31510_d_n6, assign26830_e31510_d_n7, assign26830_e31510_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard512 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign26830_e31510;
        var_isrh_dn5 = assign26830_e31510_d_n5;
        var_isrh_dn6 = assign26830_e31510_d_n6;
        var_isrh_dn7 = assign26830_e31510_d_n7;
        var_isrh_dn8 = assign26830_e31510_d_n8;

        let (assign26840_e31524,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard512 == 0.0)) {
        let assign26840_e31522: f64 = (var_vbigat - var_vjsrh);
        (assign26840_e31522,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign26840_e31524;

        let (assign26850_e31543,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard512 == 0.0)) {
        let assign26850_e31538: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign26850_e31539: f64 = (1.0 - assign26850_e31538);
        let assign26850_e31540: f64 = (assign26850_e31539).sqrt();
        let assign26850_e31541: f64 = (1.0 - assign26850_e31540);
        (assign26850_e31541,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign26850_e31543;

        let assign26860_e31546: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        var_guard513 = assign26860_e31546;

        let (assign26870_e31560,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard512 == 0.0)) && (var_guard513 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign26870_e31560;

        let (assign26880_e31592,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard512 == 0.0)) && (var_guard513 == 0.0)) {
        let assign26880_e31575: f64 = (var_wsrhstep * var_wsrhstep);
        let assign26880_e31577: f64 = (var_wsrhstep).ln();
        let assign26880_e31578: f64 = (assign26880_e31575 * assign26880_e31577);
        let assign26880_e31581: f64 = (1.0 - var_wsrhstep);
        let assign26880_e31582: f64 = (assign26880_e31578 / assign26880_e31581);
        let assign26880_e31584: f64 = (assign26880_e31582 + var_wsrhstep);
        let assign26880_e31588: f64 = (2.0 * p.p833);
        let assign26880_e31589: f64 = (1.0 - assign26880_e31588);
        let assign26880_e31590: f64 = (assign26880_e31584 * assign26880_e31589);
        (assign26880_e31590,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign26880_e31592;

        let (assign26890_e31606,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard512 == 0.0)) {
        let assign26890_e31604: f64 = (var_wsrhstep + var_dwsrh);
        (assign26890_e31604,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign26890_e31606;

        let assign26900_e31609: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        var_guard514 = assign26900_e31609;

        let (assign26910_e31626, assign26910_e31626_d_n5, assign26910_e31626_d_n6, assign26910_e31626_d_n7, assign26910_e31626_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard512 == 0.0)) && (var_guard514 != 0.0)) {
        let assign26910_e31623: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign26910_e31624: f64 = (assign26910_e31623).sqrt();
        (assign26910_e31624, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26910_e31626;
        var_tmp_dn5 = assign26910_e31626_d_n5;
        var_tmp_dn6 = assign26910_e31626_d_n6;
        var_tmp_dn7 = assign26910_e31626_d_n7;
        var_tmp_dn8 = assign26910_e31626_d_n8;

        let (assign26920_e31645, assign26920_e31645_d_n5, assign26920_e31645_d_n6, assign26920_e31645_d_n7, assign26920_e31645_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard512 == 0.0)) && (var_guard514 == 0.0)) {
        let assign26920_e31641: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign26920_e31643: f64 = (assign26920_e31641).powf(p.p833);
        (assign26920_e31643, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign26920_e31645;
        var_tmp_dn5 = assign26920_e31645_d_n5;
        var_tmp_dn6 = assign26920_e31645_d_n6;
        var_tmp_dn7 = assign26920_e31645_d_n7;
        var_tmp_dn8 = assign26920_e31645_d_n8;

        let (assign26930_e31659, assign26930_e31659_d_n5, assign26930_e31659_d_n6, assign26930_e31659_d_n7, assign26930_e31659_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard512 == 0.0)) {
        let assign26930_e31657: f64 = (var_wdepnulrgat * var_tmp);
        (assign26930_e31657, (var_wdepnulrgat * var_tmp_dn5), (var_wdepnulrgat * var_tmp_dn6), (var_wdepnulrgat * var_tmp_dn7), (var_wdepnulrgat * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign26930_e31659;
        var_wdep_dn5 = assign26930_e31659_d_n5;
        var_wdep_dn6 = assign26930_e31659_d_n6;
        var_wdep_dn7 = assign26930_e31659_d_n7;
        var_wdep_dn8 = assign26930_e31659_d_n8;

        let (assign26940_e31677, assign26940_e31677_d_n5, assign26940_e31677_d_n6, assign26940_e31677_d_n7, assign26940_e31677_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard512 == 0.0)) {
        let assign26940_e31672: f64 = (var_zinv - 1.0);
        let assign26940_e31674: f64 = (assign26940_e31672 * var_wdep);
        let assign26940_e31675: f64 = (var_ftdgat * assign26940_e31674);
        (assign26940_e31675, (var_ftdgat * (assign26940_e31672 * var_wdep_dn5)), (var_ftdgat * (assign26940_e31672 * var_wdep_dn6)), (var_ftdgat * (assign26940_e31672 * var_wdep_dn7)), (var_ftdgat * (assign26940_e31672 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign26940_e31677;
        var_asrh_dn5 = assign26940_e31677_d_n5;
        var_asrh_dn6 = assign26940_e31677_d_n6;
        var_asrh_dn7 = assign26940_e31677_d_n7;
        var_asrh_dn8 = assign26940_e31677_d_n8;

        let (assign26950_e31693, assign26950_e31693_d_n5, assign26950_e31693_d_n6, assign26950_e31693_d_n7, assign26950_e31693_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard512 == 0.0)) {
        let assign26950_e31690: f64 = (var_asrh * var_wsrh);
        let assign26950_e31691: f64 = (p.p842 * assign26950_e31690);
        (assign26950_e31691, (p.p842 * (var_asrh_dn5 * var_wsrh)), (p.p842 * (var_asrh_dn6 * var_wsrh)), (p.p842 * (var_asrh_dn7 * var_wsrh)), (p.p842 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign26950_e31693;
        var_isrh_dn5 = assign26950_e31693_d_n5;
        var_isrh_dn6 = assign26950_e31693_d_n6;
        var_isrh_dn7 = assign26950_e31693_d_n7;
        var_isrh_dn8 = assign26950_e31693_d_n8;

        let assign26960_e31696: f64 = if p.p847 == 0.0 { 1.0 } else { 0.0 };
        var_guard515 = assign26960_e31696;

        let (assign26970_e31707, assign26970_e31707_d_n5, assign26970_e31707_d_n6, assign26970_e31707_d_n7, assign26970_e31707_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign26970_e31707;
        var_itat_dn5 = assign26970_e31707_d_n5;
        var_itat_dn6 = assign26970_e31707_d_n6;
        var_itat_dn7 = assign26970_e31707_d_n7;
        var_itat_dn8 = assign26970_e31707_d_n8;

        let (assign26980_e31725, assign26980_e31725_d_n5, assign26980_e31725_d_n6, assign26980_e31725_d_n7, assign26980_e31725_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) {
        let assign26980_e31720: f64 = (var_wdep * var_one_minus_pgat);
        let assign26980_e31722: f64 = (assign26980_e31720 / var_vbi_minus_vjsrh);
        let assign26980_e31723: f64 = (var_btatpartgat * assign26980_e31722);
        (assign26980_e31723, (var_btatpartgat * ((var_wdep_dn5 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn6 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn7 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn8 * var_one_minus_pgat) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign26980_e31725;
        var_btat_dn5 = assign26980_e31725_d_n5;
        var_btat_dn6 = assign26980_e31725_d_n6;
        var_btat_dn7 = assign26980_e31725_d_n7;
        var_btat_dn8 = assign26980_e31725_d_n8;

        let (assign26990_e31741, assign26990_e31741_d_n5, assign26990_e31741_d_n6, assign26990_e31741_d_n7, assign26990_e31741_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) {
        let assign26990_e31737: f64 = (0.666666666666667 * var_atatgat);
        let assign26990_e31739: f64 = (assign26990_e31737 / var_btat);
        (assign26990_e31739, (-((assign26990_e31737 * var_btat_dn5) / (var_btat * var_btat))), (-((assign26990_e31737 * var_btat_dn6) / (var_btat * var_btat))), (-((assign26990_e31737 * var_btat_dn7) / (var_btat * var_btat))), (-((assign26990_e31737 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign26990_e31741;
        var_twoatatoverthreebtat_dn5 = assign26990_e31741_d_n5;
        var_twoatatoverthreebtat_dn6 = assign26990_e31741_d_n6;
        var_twoatatoverthreebtat_dn7 = assign26990_e31741_d_n7;
        var_twoatatoverthreebtat_dn8 = assign26990_e31741_d_n8;

        let (assign27000_e31755, assign27000_e31755_d_n5, assign27000_e31755_d_n6, assign27000_e31755_d_n7, assign27000_e31755_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) {
        let assign27000_e31753: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign27000_e31753, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign27000_e31755;
        var_umaxbeforelimiting_dn5 = assign27000_e31755_d_n5;
        var_umaxbeforelimiting_dn6 = assign27000_e31755_d_n6;
        var_umaxbeforelimiting_dn7 = assign27000_e31755_d_n7;
        var_umaxbeforelimiting_dn8 = assign27000_e31755_d_n8;

        let (assign27010_e31776, assign27010_e31776_d_n5, assign27010_e31776_d_n6, assign27010_e31776_d_n7, assign27010_e31776_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) {
        let assign27010_e31767: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign27010_e31770: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign27010_e31772: f64 = (assign27010_e31770 + 1.0);
        let assign27010_e31773: f64 = (assign27010_e31767 / assign27010_e31772);
        let assign27010_e31774: f64 = (assign27010_e31773).sqrt();
        (assign27010_e31774, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign27010_e31772) - (assign27010_e31767 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign27010_e31772 * assign27010_e31772)) / (2.0 * assign27010_e31774)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign27010_e31772) - (assign27010_e31767 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign27010_e31772 * assign27010_e31772)) / (2.0 * assign27010_e31774)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign27010_e31772) - (assign27010_e31767 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign27010_e31772 * assign27010_e31772)) / (2.0 * assign27010_e31774)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign27010_e31772) - (assign27010_e31767 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign27010_e31772 * assign27010_e31772)) / (2.0 * assign27010_e31774)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign27010_e31776;
        var_umax_dn5 = assign27010_e31776_d_n5;
        var_umax_dn6 = assign27010_e31776_d_n6;
        var_umax_dn7 = assign27010_e31776_d_n7;
        var_umax_dn8 = assign27010_e31776_d_n8;

        let (assign27020_e31789, assign27020_e31789_d_n5, assign27020_e31789_d_n6, assign27020_e31789_d_n7, assign27020_e31789_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) {
        let assign27020_e31787: f64 = (var_umax).sqrt();
        (assign27020_e31787, (var_umax_dn5 / (2.0 * assign27020_e31787)), (var_umax_dn6 / (2.0 * assign27020_e31787)), (var_umax_dn7 / (2.0 * assign27020_e31787)), (var_umax_dn8 / (2.0 * assign27020_e31787)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign27020_e31789;
        var_sqrtumax_dn5 = assign27020_e31789_d_n5;
        var_sqrtumax_dn6 = assign27020_e31789_d_n6;
        var_sqrtumax_dn7 = assign27020_e31789_d_n7;
        var_sqrtumax_dn8 = assign27020_e31789_d_n8;

        let (assign27030_e31803, assign27030_e31803_d_n5, assign27030_e31803_d_n6, assign27030_e31803_d_n7, assign27030_e31803_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) {
        let assign27030_e31801: f64 = (var_umax * var_sqrtumax);
        (assign27030_e31801, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign27030_e31803;
        var_umaxpoweronepointfive_dn5 = assign27030_e31803_d_n5;
        var_umaxpoweronepointfive_dn6 = assign27030_e31803_d_n6;
        var_umaxpoweronepointfive_dn7 = assign27030_e31803_d_n7;
        var_umaxpoweronepointfive_dn8 = assign27030_e31803_d_n8;

        let assign27040_e31805: f64 = (-p.p833);
        let assign27040_e31807: f64 = (assign27040_e31805 * var_one_over_one_minus_pgat);
        let assign27040_e31809: f64 = (-1.0);
        let assign27040_e31810: f64 = if assign27040_e31807 == assign27040_e31809 { 1.0 } else { 0.0 };
        var_guard516 = assign27040_e31810;

        let (assign27050_e31830, assign27050_e31830_d_n5, assign27050_e31830_d_n6, assign27050_e31830_d_n7, assign27050_e31830_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) && (var_guard516 != 0.0)) {
        let assign27050_e31826: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign27050_e31827: f64 = (1.0 + assign27050_e31826);
        let assign27050_e31828: f64 = (1.0 / assign27050_e31827);
        (assign27050_e31828, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign27050_e31827 * assign27050_e31827))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign27050_e31827 * assign27050_e31827))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign27050_e31827 * assign27050_e31827))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign27050_e31827 * assign27050_e31827))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign27050_e31830;
        var_wgamma_dn5 = assign27050_e31830_d_n5;
        var_wgamma_dn6 = assign27050_e31830_d_n6;
        var_wgamma_dn7 = assign27050_e31830_d_n7;
        var_wgamma_dn8 = assign27050_e31830_d_n8;

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
        *var_guard508_slot = var_guard508;
        *var_guard509_slot = var_guard509;
        *var_guard510_slot = var_guard510;
        *var_guard511_slot = var_guard511;
        *var_guard512_slot = var_guard512;
        *var_guard513_slot = var_guard513;
        *var_guard514_slot = var_guard514;
        *var_guard515_slot = var_guard515;
        *var_guard516_slot = var_guard516;
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

    pub(super) fn stamp_transient_block_52(
        p: &Parameters,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatgat: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn5: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_cerfc: f64,
        var_fbbtgat: f64,
        var_fbbtgat_dn5: f64,
        var_fbbtgat_dn6: f64,
        var_fbbtgat_dn7: f64,
        var_fbbtgat_dn8: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard511: f64,
        var_guard515: f64,
        var_guard516: f64,
        var_one_over_one_minus_pgat: f64,
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
        var_vbirgatinv: f64,
        var_wdepnulrinvgat: f64,
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
        var_guard517_slot: &mut f64,
        var_guard518_slot: &mut f64,
        var_guard519_slot: &mut f64,
        var_guard520_slot: &mut f64,
        var_guard521_slot: &mut f64,
        var_guard522_slot: &mut f64,
        var_guard523_slot: &mut f64,
        var_guard524_slot: &mut f64,
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
        let mut var_guard517: f64 = *var_guard517_slot;
        let mut var_guard518: f64 = *var_guard518_slot;
        let mut var_guard519: f64 = *var_guard519_slot;
        let mut var_guard520: f64 = *var_guard520_slot;
        let mut var_guard521: f64 = *var_guard521_slot;
        let mut var_guard522: f64 = *var_guard522_slot;
        let mut var_guard523: f64 = *var_guard523_slot;
        let mut var_guard524: f64 = *var_guard524_slot;
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

        let (assign27060_e31854, assign27060_e31854_d_n5, assign27060_e31854_d_n6, assign27060_e31854_d_n7, assign27060_e31854_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) && (var_guard516 == 0.0)) {
        let assign27060_e31846: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign27060_e31847: f64 = (1.0 + assign27060_e31846);
        let assign27060_e31849: f64 = (-p.p833);
        let assign27060_e31851: f64 = (assign27060_e31849 * var_one_over_one_minus_pgat);
        let assign27060_e31852: f64 = (assign27060_e31847).powf(assign27060_e31851);
        (assign27060_e31852, if 0.0 == 0.0 && ((assign27060_e31851) as f64).is_finite() && ((assign27060_e31851) as f64).fract() == 0.0 { if assign27060_e31851 == 0.0 { 0.0 } else { (assign27060_e31851 * ((assign27060_e31847).powf(assign27060_e31851 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign27060_e31852 * (assign27060_e31851 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign27060_e31847))) }, if 0.0 == 0.0 && ((assign27060_e31851) as f64).is_finite() && ((assign27060_e31851) as f64).fract() == 0.0 { if assign27060_e31851 == 0.0 { 0.0 } else { (assign27060_e31851 * ((assign27060_e31847).powf(assign27060_e31851 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign27060_e31852 * (assign27060_e31851 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign27060_e31847))) }, if 0.0 == 0.0 && ((assign27060_e31851) as f64).is_finite() && ((assign27060_e31851) as f64).fract() == 0.0 { if assign27060_e31851 == 0.0 { 0.0 } else { (assign27060_e31851 * ((assign27060_e31847).powf(assign27060_e31851 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign27060_e31852 * (assign27060_e31851 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign27060_e31847))) }, if 0.0 == 0.0 && ((assign27060_e31851) as f64).is_finite() && ((assign27060_e31851) as f64).fract() == 0.0 { if assign27060_e31851 == 0.0 { 0.0 } else { (assign27060_e31851 * ((assign27060_e31847).powf(assign27060_e31851 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign27060_e31852 * (assign27060_e31851 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign27060_e31847))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign27060_e31854;
        var_wgamma_dn5 = assign27060_e31854_d_n5;
        var_wgamma_dn6 = assign27060_e31854_d_n6;
        var_wgamma_dn7 = assign27060_e31854_d_n7;
        var_wgamma_dn8 = assign27060_e31854_d_n8;

        let (assign27070_e31872, assign27070_e31872_d_n5, assign27070_e31872_d_n6, assign27070_e31872_d_n7, assign27070_e31872_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) {
        let assign27070_e31866: f64 = (var_wsrh * var_wgamma);
        let assign27070_e31869: f64 = (var_wsrh + var_wgamma);
        let assign27070_e31870: f64 = (assign27070_e31866 / assign27070_e31869);
        (assign27070_e31870, ((((var_wsrh * var_wgamma_dn5) * assign27070_e31869) - (assign27070_e31866 * var_wgamma_dn5)) / (assign27070_e31869 * assign27070_e31869)), ((((var_wsrh * var_wgamma_dn6) * assign27070_e31869) - (assign27070_e31866 * var_wgamma_dn6)) / (assign27070_e31869 * assign27070_e31869)), ((((var_wsrh * var_wgamma_dn7) * assign27070_e31869) - (assign27070_e31866 * var_wgamma_dn7)) / (assign27070_e31869 * assign27070_e31869)), ((((var_wsrh * var_wgamma_dn8) * assign27070_e31869) - (assign27070_e31866 * var_wgamma_dn8)) / (assign27070_e31869 * assign27070_e31869)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign27070_e31872;
        var_wtat_dn5 = assign27070_e31872_d_n5;
        var_wtat_dn6 = assign27070_e31872_d_n6;
        var_wtat_dn7 = assign27070_e31872_d_n7;
        var_wtat_dn8 = assign27070_e31872_d_n8;

        let (assign27080_e31889, assign27080_e31889_d_n5, assign27080_e31889_d_n6, assign27080_e31889_d_n7, assign27080_e31889_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) {
        let assign27080_e31885: f64 = (var_btat / var_sqrtumax);
        let assign27080_e31886: f64 = (0.375 * assign27080_e31885);
        let assign27080_e31887: f64 = (assign27080_e31886).sqrt();
        (assign27080_e31887, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign27080_e31887)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign27080_e31887)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign27080_e31887)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign27080_e31887)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign27080_e31889;
        var_ktat_dn5 = assign27080_e31889_d_n5;
        var_ktat_dn6 = assign27080_e31889_d_n6;
        var_ktat_dn7 = assign27080_e31889_d_n7;
        var_ktat_dn8 = assign27080_e31889_d_n8;

        let (assign27090_e31907, assign27090_e31907_d_n5, assign27090_e31907_d_n6, assign27090_e31907_d_n7, assign27090_e31907_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) {
        let assign27090_e31902: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign27090_e31903: f64 = (2.0 * assign27090_e31902);
        let assign27090_e31905: f64 = (assign27090_e31903 - var_umax);
        (assign27090_e31905, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign27090_e31907;
        var_ltat_dn5 = assign27090_e31907_d_n5;
        var_ltat_dn6 = assign27090_e31907_d_n6;
        var_ltat_dn7 = assign27090_e31907_d_n7;
        var_ltat_dn8 = assign27090_e31907_d_n8;

        let (assign27100_e31933, assign27100_e31933_d_n5, assign27100_e31933_d_n6, assign27100_e31933_d_n7, assign27100_e31933_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) {
        let assign27100_e31919: f64 = (var_atatgat * var_twoatatoverthreebtat);
        let assign27100_e31921: f64 = (assign27100_e31919 * var_sqrtumax);
        let assign27100_e31924: f64 = (var_atatgat * var_umax);
        let assign27100_e31925: f64 = (assign27100_e31921 - assign27100_e31924);
        let assign27100_e31929: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign27100_e31930: f64 = (0.5 * assign27100_e31929);
        let assign27100_e31931: f64 = (assign27100_e31925 + assign27100_e31930);
        (assign27100_e31931, (((((var_atatgat * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign27100_e31919 * var_sqrtumax_dn5)) - (var_atatgat * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign27100_e31919 * var_sqrtumax_dn6)) - (var_atatgat * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign27100_e31919 * var_sqrtumax_dn7)) - (var_atatgat * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign27100_e31919 * var_sqrtumax_dn8)) - (var_atatgat * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign27100_e31933;
        var_mtat_dn5 = assign27100_e31933_d_n5;
        var_mtat_dn6 = assign27100_e31933_d_n6;
        var_mtat_dn7 = assign27100_e31933_d_n7;
        var_mtat_dn8 = assign27100_e31933_d_n8;

        let (assign27110_e31949, assign27110_e31949_d_n5, assign27110_e31949_d_n6, assign27110_e31949_d_n7, assign27110_e31949_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) {
        let assign27110_e31945: f64 = (var_ltat - 1.0);
        let assign27110_e31947: f64 = (assign27110_e31945 * var_ktat);
        (assign27110_e31947, ((var_ltat_dn5 * var_ktat) + (assign27110_e31945 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign27110_e31945 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign27110_e31945 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign27110_e31945 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign27110_e31949;
        var_xerfc_dn5 = assign27110_e31949_d_n5;
        var_xerfc_dn6 = assign27110_e31949_d_n6;
        var_xerfc_dn7 = assign27110_e31949_d_n7;
        var_xerfc_dn8 = assign27110_e31949_d_n8;

        let (assign27120_e31963, assign27120_e31963_d_n5, assign27120_e31963_d_n6, assign27120_e31963_d_n7, assign27120_e31963_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) {
        let assign27120_e31961: f64 = (var_xerfc * var_xerfc);
        (assign27120_e31961, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign27120_e31963;
        var_ysq_dn5 = assign27120_e31963_d_n5;
        var_ysq_dn6 = assign27120_e31963_d_n6;
        var_ysq_dn7 = assign27120_e31963_d_n7;
        var_ysq_dn8 = assign27120_e31963_d_n8;

        let assign27130_e31966: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard517 = assign27130_e31966;

        let (assign27140_e31986, assign27140_e31986_d_n5, assign27140_e31986_d_n6, assign27140_e31986_d_n7, assign27140_e31986_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) && (var_guard517 != 0.0)) {
        let assign27140_e31982: f64 = (var_perfc * var_xerfc);
        let assign27140_e31983: f64 = (1.0 + assign27140_e31982);
        let assign27140_e31984: f64 = (1.0 / assign27140_e31983);
        (assign27140_e31984, (-((var_perfc * var_xerfc_dn5) / (assign27140_e31983 * assign27140_e31983))), (-((var_perfc * var_xerfc_dn6) / (assign27140_e31983 * assign27140_e31983))), (-((var_perfc * var_xerfc_dn7) / (assign27140_e31983 * assign27140_e31983))), (-((var_perfc * var_xerfc_dn8) / (assign27140_e31983 * assign27140_e31983))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign27140_e31986;
        var_terfc_dn5 = assign27140_e31986_d_n5;
        var_terfc_dn6 = assign27140_e31986_d_n6;
        var_terfc_dn7 = assign27140_e31986_d_n7;
        var_terfc_dn8 = assign27140_e31986_d_n8;

        let (assign27150_e32007, assign27150_e32007_d_n5, assign27150_e32007_d_n6, assign27150_e32007_d_n7, assign27150_e32007_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) && (var_guard517 == 0.0)) {
        let assign27150_e32003: f64 = (var_perfc * var_xerfc);
        let assign27150_e32004: f64 = (1.0 - assign27150_e32003);
        let assign27150_e32005: f64 = (1.0 / assign27150_e32004);
        (assign27150_e32005, (-((-(var_perfc * var_xerfc_dn5)) / (assign27150_e32004 * assign27150_e32004))), (-((-(var_perfc * var_xerfc_dn6)) / (assign27150_e32004 * assign27150_e32004))), (-((-(var_perfc * var_xerfc_dn7)) / (assign27150_e32004 * assign27150_e32004))), (-((-(var_perfc * var_xerfc_dn8)) / (assign27150_e32004 * assign27150_e32004))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign27150_e32007;
        var_terfc_dn5 = assign27150_e32007_d_n5;
        var_terfc_dn6 = assign27150_e32007_d_n6;
        var_terfc_dn7 = assign27150_e32007_d_n7;
        var_terfc_dn8 = assign27150_e32007_d_n8;

        let assign27160_e32009: f64 = (-var_ysq);
        let assign27160_e32011: f64 = (assign27160_e32009 + var_mtat);
        let assign27160_e32013: f64 = (-230.25850929940458);
        let assign27160_e32014: f64 = if assign27160_e32011 > assign27160_e32013 { 1.0 } else { 0.0 };
        var_guard518 = assign27160_e32014;

        let (assign27170_e32032, assign27170_e32032_d_n5, assign27170_e32032_d_n6, assign27170_e32032_d_n7, assign27170_e32032_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) && (var_guard518 != 0.0)) {
        let assign27170_e32027: f64 = (-var_ysq);
        let assign27170_e32029: f64 = (assign27170_e32027 + var_mtat);
        let assign27170_e32030: f64 = (assign27170_e32029).exp();
        (assign27170_e32030, (assign27170_e32030 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign27170_e32030 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign27170_e32030 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign27170_e32030 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27170_e32032;
        var_tmp_dn5 = assign27170_e32032_d_n5;
        var_tmp_dn6 = assign27170_e32032_d_n6;
        var_tmp_dn7 = assign27170_e32032_d_n7;
        var_tmp_dn8 = assign27170_e32032_d_n8;

        let (assign27180_e32081, assign27180_e32081_d_n5, assign27180_e32081_d_n6, assign27180_e32081_d_n7, assign27180_e32081_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) && (var_guard518 == 0.0)) {
        let assign27180_e32048: f64 = (-230.25850929940458);
        let assign27180_e32050: f64 = (-var_ysq);
        let assign27180_e32052: f64 = (assign27180_e32050 + var_mtat);
        let assign27180_e32053: f64 = (assign27180_e32048 - assign27180_e32052);
        let assign27180_e32057: f64 = (-230.25850929940458);
        let assign27180_e32059: f64 = (-var_ysq);
        let assign27180_e32061: f64 = (assign27180_e32059 + var_mtat);
        let assign27180_e32062: f64 = (assign27180_e32057 - assign27180_e32061);
        let assign27180_e32065: f64 = (-230.25850929940458);
        let assign27180_e32067: f64 = (-var_ysq);
        let assign27180_e32069: f64 = (assign27180_e32067 + var_mtat);
        let assign27180_e32070: f64 = (assign27180_e32065 - assign27180_e32069);
        let assign27180_e32072: f64 = (assign27180_e32070 * 0.3333333333333333);
        let assign27180_e32073: f64 = (1.0 + assign27180_e32072);
        let assign27180_e32074: f64 = (assign27180_e32062 * assign27180_e32073);
        let assign27180_e32075: f64 = (0.5 * assign27180_e32074);
        let assign27180_e32076: f64 = (1.0 + assign27180_e32075);
        let assign27180_e32077: f64 = (assign27180_e32053 * assign27180_e32076);
        let assign27180_e32078: f64 = (1.0 + assign27180_e32077);
        let assign27180_e32079: f64 = (1e-100 / assign27180_e32078);
        (assign27180_e32079, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign27180_e32076) + (assign27180_e32053 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign27180_e32073) + (assign27180_e32062 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign27180_e32078 * assign27180_e32078))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign27180_e32076) + (assign27180_e32053 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign27180_e32073) + (assign27180_e32062 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign27180_e32078 * assign27180_e32078))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign27180_e32076) + (assign27180_e32053 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign27180_e32073) + (assign27180_e32062 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign27180_e32078 * assign27180_e32078))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign27180_e32076) + (assign27180_e32053 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign27180_e32073) + (assign27180_e32062 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign27180_e32078 * assign27180_e32078))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27180_e32081;
        var_tmp_dn5 = assign27180_e32081_d_n5;
        var_tmp_dn6 = assign27180_e32081_d_n6;
        var_tmp_dn7 = assign27180_e32081_d_n7;
        var_tmp_dn8 = assign27180_e32081_d_n8;

        let (assign27190_e32111, assign27190_e32111_d_n5, assign27190_e32111_d_n6, assign27190_e32111_d_n7, assign27190_e32111_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) {
        let assign27190_e32093: f64 = (0.29214664 * var_terfc);
        let assign27190_e32097: f64 = (var_terfc * var_terfc);
        let assign27190_e32098: f64 = (var_berfc * assign27190_e32097);
        let assign27190_e32099: f64 = (assign27190_e32093 + assign27190_e32098);
        let assign27190_e32103: f64 = (var_terfc * var_terfc);
        let assign27190_e32105: f64 = (assign27190_e32103 * var_terfc);
        let assign27190_e32106: f64 = (var_cerfc * assign27190_e32105);
        let assign27190_e32107: f64 = (assign27190_e32099 + assign27190_e32106);
        let assign27190_e32109: f64 = (assign27190_e32107 * var_tmp);
        (assign27190_e32109, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign27190_e32103 * var_terfc_dn5)))) * var_tmp) + (assign27190_e32107 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign27190_e32103 * var_terfc_dn6)))) * var_tmp) + (assign27190_e32107 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign27190_e32103 * var_terfc_dn7)))) * var_tmp) + (assign27190_e32107 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign27190_e32103 * var_terfc_dn8)))) * var_tmp) + (assign27190_e32107 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign27190_e32111;
        var_erfcpos_dn5 = assign27190_e32111_d_n5;
        var_erfcpos_dn6 = assign27190_e32111_d_n6;
        var_erfcpos_dn7 = assign27190_e32111_d_n7;
        var_erfcpos_dn8 = assign27190_e32111_d_n8;

        let assign27200_e32114: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard519 = assign27200_e32114;

        let (assign27210_e32128, assign27210_e32128_d_n5, assign27210_e32128_d_n6, assign27210_e32128_d_n7, assign27210_e32128_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) && (var_guard519 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign27210_e32128;
        var_erfctimesexpmtat_dn5 = assign27210_e32128_d_n5;
        var_erfctimesexpmtat_dn6 = assign27210_e32128_d_n6;
        var_erfctimesexpmtat_dn7 = assign27210_e32128_d_n7;
        var_erfctimesexpmtat_dn8 = assign27210_e32128_d_n8;

        let assign27220_e32131: f64 = (-230.25850929940458);
        let assign27220_e32132: f64 = if var_mtat > assign27220_e32131 { 1.0 } else { 0.0 };
        var_guard520 = assign27220_e32132;

        let (assign27230_e32150, assign27230_e32150_d_n5, assign27230_e32150_d_n6, assign27230_e32150_d_n7, assign27230_e32150_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) && (var_guard519 == 0.0)) && (var_guard520 != 0.0)) {
        let assign27230_e32148: f64 = (var_mtat).exp();
        (assign27230_e32148, (assign27230_e32148 * var_mtat_dn5), (assign27230_e32148 * var_mtat_dn6), (assign27230_e32148 * var_mtat_dn7), (assign27230_e32148 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27230_e32150;
        var_tmp_dn5 = assign27230_e32150_d_n5;
        var_tmp_dn6 = assign27230_e32150_d_n6;
        var_tmp_dn7 = assign27230_e32150_d_n7;
        var_tmp_dn8 = assign27230_e32150_d_n8;

        let (assign27240_e32193, assign27240_e32193_d_n5, assign27240_e32193_d_n6, assign27240_e32193_d_n7, assign27240_e32193_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) && (var_guard519 == 0.0)) && (var_guard520 == 0.0)) {
        let assign27240_e32169: f64 = (-230.25850929940458);
        let assign27240_e32171: f64 = (assign27240_e32169 - var_mtat);
        let assign27240_e32175: f64 = (-230.25850929940458);
        let assign27240_e32177: f64 = (assign27240_e32175 - var_mtat);
        let assign27240_e32180: f64 = (-230.25850929940458);
        let assign27240_e32182: f64 = (assign27240_e32180 - var_mtat);
        let assign27240_e32184: f64 = (assign27240_e32182 * 0.3333333333333333);
        let assign27240_e32185: f64 = (1.0 + assign27240_e32184);
        let assign27240_e32186: f64 = (assign27240_e32177 * assign27240_e32185);
        let assign27240_e32187: f64 = (0.5 * assign27240_e32186);
        let assign27240_e32188: f64 = (1.0 + assign27240_e32187);
        let assign27240_e32189: f64 = (assign27240_e32171 * assign27240_e32188);
        let assign27240_e32190: f64 = (1.0 + assign27240_e32189);
        let assign27240_e32191: f64 = (1e-100 / assign27240_e32190);
        (assign27240_e32191, (-((1e-100 * (((-var_mtat_dn5) * assign27240_e32188) + (assign27240_e32171 * (0.5 * (((-var_mtat_dn5) * assign27240_e32185) + (assign27240_e32177 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign27240_e32190 * assign27240_e32190))), (-((1e-100 * (((-var_mtat_dn6) * assign27240_e32188) + (assign27240_e32171 * (0.5 * (((-var_mtat_dn6) * assign27240_e32185) + (assign27240_e32177 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign27240_e32190 * assign27240_e32190))), (-((1e-100 * (((-var_mtat_dn7) * assign27240_e32188) + (assign27240_e32171 * (0.5 * (((-var_mtat_dn7) * assign27240_e32185) + (assign27240_e32177 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign27240_e32190 * assign27240_e32190))), (-((1e-100 * (((-var_mtat_dn8) * assign27240_e32188) + (assign27240_e32171 * (0.5 * (((-var_mtat_dn8) * assign27240_e32185) + (assign27240_e32177 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign27240_e32190 * assign27240_e32190))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27240_e32193;
        var_tmp_dn5 = assign27240_e32193_d_n5;
        var_tmp_dn6 = assign27240_e32193_d_n6;
        var_tmp_dn7 = assign27240_e32193_d_n7;
        var_tmp_dn8 = assign27240_e32193_d_n8;

        let (assign27250_e32212, assign27250_e32212_d_n5, assign27250_e32212_d_n6, assign27250_e32212_d_n7, assign27250_e32212_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) && (var_guard519 == 0.0)) {
        let assign27250_e32208: f64 = (2.0 * var_tmp);
        let assign27250_e32210: f64 = (assign27250_e32208 - var_erfcpos);
        (assign27250_e32210, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign27250_e32212;
        var_erfctimesexpmtat_dn5 = assign27250_e32212_d_n5;
        var_erfctimesexpmtat_dn6 = assign27250_e32212_d_n6;
        var_erfctimesexpmtat_dn7 = assign27250_e32212_d_n7;
        var_erfctimesexpmtat_dn8 = assign27250_e32212_d_n8;

        let (assign27260_e32232, assign27260_e32232_d_n5, assign27260_e32232_d_n6, assign27260_e32232_d_n7, assign27260_e32232_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) {
        let assign27260_e32224: f64 = (1.772453850905516 * 0.5);
        let assign27260_e32227: f64 = (var_atatgat * var_erfctimesexpmtat);
        let assign27260_e32229: f64 = (assign27260_e32227 / var_ktat);
        let assign27260_e32230: f64 = (assign27260_e32224 * assign27260_e32229);
        (assign27260_e32230, (assign27260_e32224 * ((((var_atatgat * var_erfctimesexpmtat_dn5) * var_ktat) - (assign27260_e32227 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign27260_e32224 * ((((var_atatgat * var_erfctimesexpmtat_dn6) * var_ktat) - (assign27260_e32227 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign27260_e32224 * ((((var_atatgat * var_erfctimesexpmtat_dn7) * var_ktat) - (assign27260_e32227 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign27260_e32224 * ((((var_atatgat * var_erfctimesexpmtat_dn8) * var_ktat) - (assign27260_e32227 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign27260_e32232;
        var_gammamax_dn5 = assign27260_e32232_d_n5;
        var_gammamax_dn6 = assign27260_e32232_d_n6;
        var_gammamax_dn7 = assign27260_e32232_d_n7;
        var_gammamax_dn8 = assign27260_e32232_d_n8;

        let (assign27270_e32250, assign27270_e32250_d_n5, assign27270_e32250_d_n6, assign27270_e32250_d_n7, assign27270_e32250_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard515 == 0.0)) {
        let assign27270_e32245: f64 = (var_asrh * var_gammamax);
        let assign27270_e32247: f64 = (assign27270_e32245 * var_wtat);
        let assign27270_e32248: f64 = (p.p847 * assign27270_e32247);
        (assign27270_e32248, (p.p847 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign27270_e32245 * var_wtat_dn5))), (p.p847 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign27270_e32245 * var_wtat_dn6))), (p.p847 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign27270_e32245 * var_wtat_dn7))), (p.p847 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign27270_e32245 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign27270_e32250;
        var_itat_dn5 = assign27270_e32250_d_n5;
        var_itat_dn6 = assign27270_e32250_d_n6;
        var_itat_dn7 = assign27270_e32250_d_n7;
        var_itat_dn8 = assign27270_e32250_d_n8;

        let assign27280_e32253: f64 = if p.p853 == 0.0 { 1.0 } else { 0.0 };
        var_guard521 = assign27280_e32253;

        let (assign27290_e32264, assign27290_e32264_d_n5, assign27290_e32264_d_n6, assign27290_e32264_d_n7, assign27290_e32264_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard521 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign27290_e32264;
        var_ibbt_dn5 = assign27290_e32264_d_n5;
        var_ibbt_dn6 = assign27290_e32264_d_n6;
        var_ibbt_dn7 = assign27290_e32264_d_n7;
        var_ibbt_dn8 = assign27290_e32264_d_n8;

        let assign27300_e32267: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        var_guard522 = assign27300_e32267;

        let (assign27310_e32286, assign27310_e32286_d_n5, assign27310_e32286_d_n6, assign27310_e32286_d_n7, assign27310_e32286_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard521 == 0.0)) && (var_guard522 != 0.0)) {
        let assign27310_e32281: f64 = (p.p830 - var_vbbt);
        let assign27310_e32283: f64 = (assign27310_e32281 * var_vbirgatinv);
        let assign27310_e32284: f64 = (assign27310_e32283).sqrt();
        (assign27310_e32284, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27310_e32286;
        var_tmp_dn5 = assign27310_e32286_d_n5;
        var_tmp_dn6 = assign27310_e32286_d_n6;
        var_tmp_dn7 = assign27310_e32286_d_n7;
        var_tmp_dn8 = assign27310_e32286_d_n8;

        let (assign27320_e32307, assign27320_e32307_d_n5, assign27320_e32307_d_n6, assign27320_e32307_d_n7, assign27320_e32307_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard521 == 0.0)) && (var_guard522 == 0.0)) {
        let assign27320_e32301: f64 = (p.p830 - var_vbbt);
        let assign27320_e32303: f64 = (assign27320_e32301 * var_vbirgatinv);
        let assign27320_e32305: f64 = (assign27320_e32303).powf(p.p833);
        (assign27320_e32305, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27320_e32307;
        var_tmp_dn5 = assign27320_e32307_d_n5;
        var_tmp_dn6 = assign27320_e32307_d_n6;
        var_tmp_dn7 = assign27320_e32307_d_n7;
        var_tmp_dn8 = assign27320_e32307_d_n8;

        let (assign27330_e32327, assign27330_e32327_d_n5, assign27330_e32327_d_n6, assign27330_e32327_d_n7, assign27330_e32327_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard521 == 0.0)) {
        let assign27330_e32320: f64 = (p.p830 - var_vbbt);
        let assign27330_e32322: f64 = (assign27330_e32320 * var_wdepnulrinvgat);
        let assign27330_e32324: f64 = (assign27330_e32322 / var_tmp);
        let assign27330_e32325: f64 = (var_one_over_one_minus_pgat * assign27330_e32324);
        (assign27330_e32325, (var_one_over_one_minus_pgat * (-((assign27330_e32322 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign27330_e32322 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign27330_e32322 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign27330_e32322 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign27330_e32327;
        var_fmaxr_dn5 = assign27330_e32327_d_n5;
        var_fmaxr_dn6 = assign27330_e32327_d_n6;
        var_fmaxr_dn7 = assign27330_e32327_d_n7;
        var_fmaxr_dn8 = assign27330_e32327_d_n8;

        let assign27340_e32329: f64 = (-var_fbbtgat);
        let assign27340_e32331: f64 = (assign27340_e32329 / var_fmaxr);
        let assign27340_e32332: f64 = (assign27340_e32331).abs();
        let assign27340_e32334: f64 = if assign27340_e32332 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard523 = assign27340_e32334;

        let (assign27350_e32352, assign27350_e32352_d_n5, assign27350_e32352_d_n6, assign27350_e32352_d_n7, assign27350_e32352_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard521 == 0.0)) && (var_guard523 != 0.0)) {
        let assign27350_e32347: f64 = (-var_fbbtgat);
        let assign27350_e32349: f64 = (assign27350_e32347 / var_fmaxr);
        let assign27350_e32350: f64 = (assign27350_e32349).exp();
        (assign27350_e32350, (assign27350_e32350 * ((((-var_fbbtgat_dn5) * var_fmaxr) - (assign27350_e32347 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign27350_e32350 * ((((-var_fbbtgat_dn6) * var_fmaxr) - (assign27350_e32347 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign27350_e32350 * ((((-var_fbbtgat_dn7) * var_fmaxr) - (assign27350_e32347 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign27350_e32350 * ((((-var_fbbtgat_dn8) * var_fmaxr) - (assign27350_e32347 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27350_e32352;
        var_tmp_dn5 = assign27350_e32352_d_n5;
        var_tmp_dn6 = assign27350_e32352_d_n6;
        var_tmp_dn7 = assign27350_e32352_d_n7;
        var_tmp_dn8 = assign27350_e32352_d_n8;

        let assign27360_e32354: f64 = (-var_fbbtgat);
        let assign27360_e32356: f64 = (assign27360_e32354 / var_fmaxr);
        let assign27360_e32358: f64 = if assign27360_e32356 < 0.0 { 1.0 } else { 0.0 };
        var_guard524 = assign27360_e32358;

        let (assign27370_e32409, assign27370_e32409_d_n5, assign27370_e32409_d_n6, assign27370_e32409_d_n7, assign27370_e32409_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard521 == 0.0)) && (var_guard523 == 0.0)) && (var_guard524 != 0.0)) {
        let assign27370_e32376: f64 = (-230.25850929940458);
        let assign27370_e32378: f64 = (-var_fbbtgat);
        let assign27370_e32380: f64 = (assign27370_e32378 / var_fmaxr);
        let assign27370_e32381: f64 = (assign27370_e32376 - assign27370_e32380);
        let assign27370_e32385: f64 = (-230.25850929940458);
        let assign27370_e32387: f64 = (-var_fbbtgat);
        let assign27370_e32389: f64 = (assign27370_e32387 / var_fmaxr);
        let assign27370_e32390: f64 = (assign27370_e32385 - assign27370_e32389);
        let assign27370_e32393: f64 = (-230.25850929940458);
        let assign27370_e32395: f64 = (-var_fbbtgat);
        let assign27370_e32397: f64 = (assign27370_e32395 / var_fmaxr);
        let assign27370_e32398: f64 = (assign27370_e32393 - assign27370_e32397);
        let assign27370_e32400: f64 = (assign27370_e32398 * 0.3333333333333333);
        let assign27370_e32401: f64 = (1.0 + assign27370_e32400);
        let assign27370_e32402: f64 = (assign27370_e32390 * assign27370_e32401);
        let assign27370_e32403: f64 = (0.5 * assign27370_e32402);
        let assign27370_e32404: f64 = (1.0 + assign27370_e32403);
        let assign27370_e32405: f64 = (assign27370_e32381 * assign27370_e32404);
        let assign27370_e32406: f64 = (1.0 + assign27370_e32405);
        let assign27370_e32407: f64 = (1e-100 / assign27370_e32406);
        (assign27370_e32407, (-((1e-100 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign27370_e32378 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign27370_e32404) + (assign27370_e32381 * (0.5 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign27370_e32387 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign27370_e32401) + (assign27370_e32390 * ((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign27370_e32395 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign27370_e32406 * assign27370_e32406))), (-((1e-100 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign27370_e32378 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign27370_e32404) + (assign27370_e32381 * (0.5 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign27370_e32387 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign27370_e32401) + (assign27370_e32390 * ((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign27370_e32395 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign27370_e32406 * assign27370_e32406))), (-((1e-100 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign27370_e32378 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign27370_e32404) + (assign27370_e32381 * (0.5 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign27370_e32387 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign27370_e32401) + (assign27370_e32390 * ((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign27370_e32395 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign27370_e32406 * assign27370_e32406))), (-((1e-100 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign27370_e32378 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign27370_e32404) + (assign27370_e32381 * (0.5 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign27370_e32387 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign27370_e32401) + (assign27370_e32390 * ((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign27370_e32395 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign27370_e32406 * assign27370_e32406))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27370_e32409;
        var_tmp_dn5 = assign27370_e32409_d_n5;
        var_tmp_dn6 = assign27370_e32409_d_n6;
        var_tmp_dn7 = assign27370_e32409_d_n7;
        var_tmp_dn8 = assign27370_e32409_d_n8;

        let (assign27380_e32458, assign27380_e32458_d_n5, assign27380_e32458_d_n6, assign27380_e32458_d_n7, assign27380_e32458_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard521 == 0.0)) && (var_guard523 == 0.0)) && (var_guard524 == 0.0)) {
        let assign27380_e32428: f64 = (-var_fbbtgat);
        let assign27380_e32430: f64 = (assign27380_e32428 / var_fmaxr);
        let assign27380_e32432: f64 = (assign27380_e32430 - 230.25850929940458);
        let assign27380_e32436: f64 = (-var_fbbtgat);
        let assign27380_e32438: f64 = (assign27380_e32436 / var_fmaxr);
        let assign27380_e32440: f64 = (assign27380_e32438 - 230.25850929940458);
        let assign27380_e32443: f64 = (-var_fbbtgat);
        let assign27380_e32445: f64 = (assign27380_e32443 / var_fmaxr);
        let assign27380_e32447: f64 = (assign27380_e32445 - 230.25850929940458);
        let assign27380_e32449: f64 = (assign27380_e32447 * 0.3333333333333333);
        let assign27380_e32450: f64 = (1.0 + assign27380_e32449);
        let assign27380_e32451: f64 = (assign27380_e32440 * assign27380_e32450);
        let assign27380_e32452: f64 = (0.5 * assign27380_e32451);
        let assign27380_e32453: f64 = (1.0 + assign27380_e32452);
        let assign27380_e32454: f64 = (assign27380_e32432 * assign27380_e32453);
        let assign27380_e32455: f64 = (1.0 + assign27380_e32454);
        let assign27380_e32456: f64 = (1e100 * assign27380_e32455);
        (assign27380_e32456, (1e100 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign27380_e32428 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign27380_e32453) + (assign27380_e32432 * (0.5 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign27380_e32436 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign27380_e32450) + (assign27380_e32440 * (((((-var_fbbtgat_dn5) * var_fmaxr) - (assign27380_e32443 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign27380_e32428 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign27380_e32453) + (assign27380_e32432 * (0.5 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign27380_e32436 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign27380_e32450) + (assign27380_e32440 * (((((-var_fbbtgat_dn6) * var_fmaxr) - (assign27380_e32443 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign27380_e32428 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign27380_e32453) + (assign27380_e32432 * (0.5 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign27380_e32436 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign27380_e32450) + (assign27380_e32440 * (((((-var_fbbtgat_dn7) * var_fmaxr) - (assign27380_e32443 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign27380_e32428 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign27380_e32453) + (assign27380_e32432 * (0.5 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign27380_e32436 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign27380_e32450) + (assign27380_e32440 * (((((-var_fbbtgat_dn8) * var_fmaxr) - (assign27380_e32443 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27380_e32458;
        var_tmp_dn5 = assign27380_e32458_d_n5;
        var_tmp_dn6 = assign27380_e32458_d_n6;
        var_tmp_dn7 = assign27380_e32458_d_n7;
        var_tmp_dn8 = assign27380_e32458_d_n8;

        let (assign27390_e32478, assign27390_e32478_d_n5, assign27390_e32478_d_n6, assign27390_e32478_d_n7, assign27390_e32478_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard521 == 0.0)) {
        let assign27390_e32471: f64 = (var_v5 * var_fmaxr);
        let assign27390_e32473: f64 = (assign27390_e32471 * var_fmaxr);
        let assign27390_e32475: f64 = (assign27390_e32473 * var_tmp);
        let assign27390_e32476: f64 = (p.p853 * assign27390_e32475);
        (assign27390_e32476, (p.p853 * (((((var_v5 * var_fmaxr_dn5) * var_fmaxr) + (assign27390_e32471 * var_fmaxr_dn5)) * var_tmp) + (assign27390_e32473 * var_tmp_dn5))), (p.p853 * (((((var_v5 * var_fmaxr_dn6) * var_fmaxr) + (assign27390_e32471 * var_fmaxr_dn6)) * var_tmp) + (assign27390_e32473 * var_tmp_dn6))), (p.p853 * (((((var_v5 * var_fmaxr_dn7) * var_fmaxr) + (assign27390_e32471 * var_fmaxr_dn7)) * var_tmp) + (assign27390_e32473 * var_tmp_dn7))), (p.p853 * (((((var_v5 * var_fmaxr_dn8) * var_fmaxr) + (assign27390_e32471 * var_fmaxr_dn8)) * var_tmp) + (assign27390_e32473 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign27390_e32478;
        var_ibbt_dn5 = assign27390_e32478_d_n5;
        var_ibbt_dn6 = assign27390_e32478_d_n6;
        var_ibbt_dn7 = assign27390_e32478_d_n7;
        var_ibbt_dn8 = assign27390_e32478_d_n8;

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
        *var_guard517_slot = var_guard517;
        *var_guard518_slot = var_guard518;
        *var_guard519_slot = var_guard519;
        *var_guard520_slot = var_guard520;
        *var_guard521_slot = var_guard521;
        *var_guard522_slot = var_guard522;
        *var_guard523_slot = var_guard523;
        *var_guard524_slot = var_guard524;
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

    pub(super) fn stamp_transient_block_53(
        p: &Parameters,
        var_absource_i: f64,
        var_alphaav: f64,
        var_fstopgat: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard511: f64,
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
        var_ibbt: f64,
        var_ibbt_dn5: f64,
        var_ibbt_dn6: f64,
        var_ibbt_dn7: f64,
        var_ibbt_dn8: f64,
        var_id__blk219: f64,
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
        var_itat: f64,
        var_itat_dn5: f64,
        var_itat_dn6: f64,
        var_itat_dn7: f64,
        var_itat_dn8: f64,
        var_lgsource_i: f64,
        var_lssource_i: f64,
        var_mfor1_s: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_slopegat: f64,
        var_slopegat_dn5: f64,
        var_slopegat_dn6: f64,
        var_slopegat_dn7: f64,
        var_slopegat_dn8: f64,
        var_v1: f64,
        var_v2: f64,
        var_v3: f64,
        var_v4: f64,
        var_v5: f64,
        var_vav: f64,
        var_vbrinvgat: f64,
        var_vbrinvgat_dn5: f64,
        var_vbrinvgat_dn6: f64,
        var_vbrinvgat_dn7: f64,
        var_vbrinvgat_dn8: f64,
        var_alphaje_slot: &mut f64,
        var_alphaje_dn5_slot: &mut f64,
        var_alphaje_dn6_slot: &mut f64,
        var_alphaje_dn7_slot: &mut f64,
        var_alphaje_dn8_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_guard525_slot: &mut f64,
        var_guard526_slot: &mut f64,
        var_guard527_slot: &mut f64,
        var_guard528_slot: &mut f64,
        var_guard529_slot: &mut f64,
        var_guard530_slot: &mut f64,
        var_guard531_slot: &mut f64,
        var_guard532_slot: &mut f64,
        var_guard533_slot: &mut f64,
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
        var_i5_slot: &mut f64,
        var_i5_cor_slot: &mut f64,
        var_i5_cor_dn5_slot: &mut f64,
        var_i5_cor_dn6_slot: &mut f64,
        var_i5_cor_dn7_slot: &mut f64,
        var_i5_cor_dn8_slot: &mut f64,
        var_i5_dn5_slot: &mut f64,
        var_i5_dn6_slot: &mut f64,
        var_i5_dn7_slot: &mut f64,
        var_i5_dn8_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn5_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_isatfor1_s_slot: &mut f64,
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
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
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
    ) {
        let mut var_alphaje: f64 = *var_alphaje_slot;
        let mut var_alphaje_dn5: f64 = *var_alphaje_dn5_slot;
        let mut var_alphaje_dn6: f64 = *var_alphaje_dn6_slot;
        let mut var_alphaje_dn7: f64 = *var_alphaje_dn7_slot;
        let mut var_alphaje_dn8: f64 = *var_alphaje_dn8_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_guard525: f64 = *var_guard525_slot;
        let mut var_guard526: f64 = *var_guard526_slot;
        let mut var_guard527: f64 = *var_guard527_slot;
        let mut var_guard528: f64 = *var_guard528_slot;
        let mut var_guard529: f64 = *var_guard529_slot;
        let mut var_guard530: f64 = *var_guard530_slot;
        let mut var_guard531: f64 = *var_guard531_slot;
        let mut var_guard532: f64 = *var_guard532_slot;
        let mut var_guard533: f64 = *var_guard533_slot;
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
        let mut var_i5: f64 = *var_i5_slot;
        let mut var_i5_cor: f64 = *var_i5_cor_slot;
        let mut var_i5_cor_dn5: f64 = *var_i5_cor_dn5_slot;
        let mut var_i5_cor_dn6: f64 = *var_i5_cor_dn6_slot;
        let mut var_i5_cor_dn7: f64 = *var_i5_cor_dn7_slot;
        let mut var_i5_cor_dn8: f64 = *var_i5_cor_dn8_slot;
        let mut var_i5_dn5: f64 = *var_i5_dn5_slot;
        let mut var_i5_dn6: f64 = *var_i5_dn6_slot;
        let mut var_i5_dn7: f64 = *var_i5_dn7_slot;
        let mut var_i5_dn8: f64 = *var_i5_dn8_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn5: f64 = *var_ijungat_dn5_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_isatfor1_s: f64 = *var_isatfor1_s_slot;
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
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
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

        let assign27400_e32481: f64 = if p.p862 > 1000.0 { 1.0 } else { 0.0 };
        var_guard525 = assign27400_e32481;

        let (assign27410_e32492, assign27410_e32492_d_n5, assign27410_e32492_d_n6, assign27410_e32492_d_n7, assign27410_e32492_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard525 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign27410_e32492;
        var_fbreakdown_dn5 = assign27410_e32492_d_n5;
        var_fbreakdown_dn6 = assign27410_e32492_d_n6;
        var_fbreakdown_dn7 = assign27410_e32492_d_n7;
        var_fbreakdown_dn8 = assign27410_e32492_d_n8;

        let assign27420_e32495: f64 = (-var_alphaav);
        let assign27420_e32497: f64 = (assign27420_e32495 * p.p862);
        let assign27420_e32498: f64 = if var_vav > assign27420_e32497 { 1.0 } else { 0.0 };
        var_guard526 = assign27420_e32498;

        let assign27430_e32501: f64 = if p.p865 == 4.0 { 1.0 } else { 0.0 };
        var_guard527 = assign27430_e32501;

        let (assign27440_e32531, assign27440_e32531_d_n5, assign27440_e32531_d_n6, assign27440_e32531_d_n7, assign27440_e32531_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard525 == 0.0)) && (var_guard526 != 0.0)) && (var_guard527 != 0.0)) {
        let assign27440_e32517: f64 = (var_vav * var_vbrinvgat);
        let assign27440_e32520: f64 = (var_vav * var_vbrinvgat);
        let assign27440_e32521: f64 = (assign27440_e32517 * assign27440_e32520);
        let assign27440_e32524: f64 = (var_vav * var_vbrinvgat);
        let assign27440_e32525: f64 = (assign27440_e32521 * assign27440_e32524);
        let assign27440_e32528: f64 = (var_vav * var_vbrinvgat);
        let assign27440_e32529: f64 = (assign27440_e32525 * assign27440_e32528);
        (assign27440_e32529, (((((((var_vav * var_vbrinvgat_dn5) * assign27440_e32520) + (assign27440_e32517 * (var_vav * var_vbrinvgat_dn5))) * assign27440_e32524) + (assign27440_e32521 * (var_vav * var_vbrinvgat_dn5))) * assign27440_e32528) + (assign27440_e32525 * (var_vav * var_vbrinvgat_dn5))), (((((((var_vav * var_vbrinvgat_dn6) * assign27440_e32520) + (assign27440_e32517 * (var_vav * var_vbrinvgat_dn6))) * assign27440_e32524) + (assign27440_e32521 * (var_vav * var_vbrinvgat_dn6))) * assign27440_e32528) + (assign27440_e32525 * (var_vav * var_vbrinvgat_dn6))), (((((((var_vav * var_vbrinvgat_dn7) * assign27440_e32520) + (assign27440_e32517 * (var_vav * var_vbrinvgat_dn7))) * assign27440_e32524) + (assign27440_e32521 * (var_vav * var_vbrinvgat_dn7))) * assign27440_e32528) + (assign27440_e32525 * (var_vav * var_vbrinvgat_dn7))), (((((((var_vav * var_vbrinvgat_dn8) * assign27440_e32520) + (assign27440_e32517 * (var_vav * var_vbrinvgat_dn8))) * assign27440_e32524) + (assign27440_e32521 * (var_vav * var_vbrinvgat_dn8))) * assign27440_e32528) + (assign27440_e32525 * (var_vav * var_vbrinvgat_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27440_e32531;
        var_tmp_dn5 = assign27440_e32531_d_n5;
        var_tmp_dn6 = assign27440_e32531_d_n6;
        var_tmp_dn7 = assign27440_e32531_d_n7;
        var_tmp_dn8 = assign27440_e32531_d_n8;

        let (assign27450_e32553, assign27450_e32553_d_n5, assign27450_e32553_d_n6, assign27450_e32553_d_n7, assign27450_e32553_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard525 == 0.0)) && (var_guard526 != 0.0)) && (var_guard527 == 0.0)) {
        let assign27450_e32548: f64 = (var_vav * var_vbrinvgat);
        let assign27450_e32549: f64 = (assign27450_e32548).abs();
        let assign27450_e32551: f64 = (assign27450_e32549).powf(p.p865);
        (assign27450_e32551, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign27450_e32549).powf(p.p865 - 1.0) * if assign27450_e32548 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) })) } } else { (assign27450_e32551 * (p.p865 * (if assign27450_e32548 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) } / assign27450_e32549))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign27450_e32549).powf(p.p865 - 1.0) * if assign27450_e32548 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) })) } } else { (assign27450_e32551 * (p.p865 * (if assign27450_e32548 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) } / assign27450_e32549))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign27450_e32549).powf(p.p865 - 1.0) * if assign27450_e32548 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) })) } } else { (assign27450_e32551 * (p.p865 * (if assign27450_e32548 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) } / assign27450_e32549))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign27450_e32549).powf(p.p865 - 1.0) * if assign27450_e32548 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) })) } } else { (assign27450_e32551 * (p.p865 * (if assign27450_e32548 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) } / assign27450_e32549))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign27450_e32553;
        var_tmp_dn5 = assign27450_e32553_d_n5;
        var_tmp_dn6 = assign27450_e32553_d_n6;
        var_tmp_dn7 = assign27450_e32553_d_n7;
        var_tmp_dn8 = assign27450_e32553_d_n8;

        let (assign27460_e32571, assign27460_e32571_d_n5, assign27460_e32571_d_n6, assign27460_e32571_d_n7, assign27460_e32571_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard525 == 0.0)) && (var_guard526 != 0.0)) {
        let assign27460_e32568: f64 = (1.0 - var_tmp);
        let assign27460_e32569: f64 = (1.0 / assign27460_e32568);
        (assign27460_e32569, (-((-var_tmp_dn5) / (assign27460_e32568 * assign27460_e32568))), (-((-var_tmp_dn6) / (assign27460_e32568 * assign27460_e32568))), (-((-var_tmp_dn7) / (assign27460_e32568 * assign27460_e32568))), (-((-var_tmp_dn8) / (assign27460_e32568 * assign27460_e32568))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign27460_e32571;
        var_fbreakdown_dn5 = assign27460_e32571_d_n5;
        var_fbreakdown_dn6 = assign27460_e32571_d_n6;
        var_fbreakdown_dn7 = assign27460_e32571_d_n7;
        var_fbreakdown_dn8 = assign27460_e32571_d_n8;

        let (assign27470_e32594, assign27470_e32594_d_n5, assign27470_e32594_d_n6, assign27470_e32594_d_n7, assign27470_e32594_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) && (var_guard525 == 0.0)) && (var_guard526 == 0.0)) {
        let assign27470_e32588: f64 = (var_alphaav * p.p862);
        let assign27470_e32589: f64 = (var_vav + assign27470_e32588);
        let assign27470_e32591: f64 = (assign27470_e32589 * var_slopegat);
        let assign27470_e32592: f64 = (var_fstopgat + assign27470_e32591);
        (assign27470_e32592, (assign27470_e32589 * var_slopegat_dn5), (assign27470_e32589 * var_slopegat_dn6), (assign27470_e32589 * var_slopegat_dn7), (assign27470_e32589 * var_slopegat_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign27470_e32594;
        var_fbreakdown_dn5 = assign27470_e32594_d_n5;
        var_fbreakdown_dn6 = assign27470_e32594_d_n6;
        var_fbreakdown_dn7 = assign27470_e32594_d_n7;
        var_fbreakdown_dn8 = assign27470_e32594_d_n8;

        let (assign27480_e32613, assign27480_e32613_d_n5, assign27480_e32613_d_n6, assign27480_e32613_d_n7, assign27480_e32613_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard511 == 0.0)) {
        let assign27480_e32604: f64 = (var_id__blk219 + var_isrh);
        let assign27480_e32606: f64 = (assign27480_e32604 + var_itat);
        let assign27480_e32608: f64 = (assign27480_e32606 + var_ibbt);
        let assign27480_e32609: f64 = (p.p29 * assign27480_e32608);
        let assign27480_e32611: f64 = (assign27480_e32609 * var_fbreakdown);
        (assign27480_e32611, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign27480_e32609 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign27480_e32609 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign27480_e32609 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign27480_e32609 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign27480_e32613;
        var_ijungat_dn5 = assign27480_e32613_d_n5;
        var_ijungat_dn6 = assign27480_e32613_d_n6;
        var_ijungat_dn7 = assign27480_e32613_d_n7;
        var_ijungat_dn8 = assign27480_e32613_d_n8;

        let (assign27490_e32629, assign27490_e32629_d_n5, assign27490_e32629_d_n6, assign27490_e32629_d_n7, assign27490_e32629_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign27490_e32619: f64 = (var_absource_i * var_ijunbot);
        let assign27490_e32622: f64 = (var_lssource_i * var_ijunsti);
        let assign27490_e32623: f64 = (assign27490_e32619 + assign27490_e32622);
        let assign27490_e32626: f64 = (var_lgsource_i * var_ijungat);
        let assign27490_e32627: f64 = (assign27490_e32623 + assign27490_e32626);
        (assign27490_e32627, (((var_absource_i * var_ijunbot_dn5) + (var_lssource_i * var_ijunsti_dn5)) + (var_lgsource_i * var_ijungat_dn5)), (((var_absource_i * var_ijunbot_dn6) + (var_lssource_i * var_ijunsti_dn6)) + (var_lgsource_i * var_ijungat_dn6)), (((var_absource_i * var_ijunbot_dn7) + (var_lssource_i * var_ijunsti_dn7)) + (var_lgsource_i * var_ijungat_dn7)), (((var_absource_i * var_ijunbot_dn8) + (var_lssource_i * var_ijunsti_dn8)) + (var_lgsource_i * var_ijungat_dn8)),)
    } else {
        (var_i5, var_i5_dn5, var_i5_dn6, var_i5_dn7, var_i5_dn8,)
    }
};
        var_i5 = assign27490_e32629;
        var_i5_dn5 = assign27490_e32629_d_n5;
        var_i5_dn6 = assign27490_e32629_d_n6;
        var_i5_dn7 = assign27490_e32629_d_n7;
        var_i5_dn8 = assign27490_e32629_d_n8;

        let (assign27500_e32645,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign27500_e32635: f64 = (var_absource_i * var_idsatbot);
        let assign27500_e32638: f64 = (var_lssource_i * var_idsatsti);
        let assign27500_e32639: f64 = (assign27500_e32635 + assign27500_e32638);
        let assign27500_e32642: f64 = (var_lgsource_i * var_idsatgat);
        let assign27500_e32643: f64 = (assign27500_e32639 + assign27500_e32642);
        (assign27500_e32643,)
    } else {
        (var_isatfor1_s,)
    }
};
        var_isatfor1_s = assign27500_e32645;

        let (assign27510_e32662, assign27510_e32662_d_n5, assign27510_e32662_d_n6, assign27510_e32662_d_n7, assign27510_e32662_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign27510_e32653: f64 = (var_v4 * var_phitdinv);
        let assign27510_e32655: f64 = (assign27510_e32653 * var_mfor1_s);
        let assign27510_e32656: f64 = (assign27510_e32655).exp();
        let assign27510_e32658: f64 = (assign27510_e32656 - 1.0);
        let assign27510_e32659: f64 = (var_isatfor1_s * assign27510_e32658);
        let assign27510_e32660: f64 = (var_i4 - assign27510_e32659);
        (assign27510_e32660, var_i4_dn5, var_i4_dn6, var_i4_dn7, var_i4_dn8,)
    } else {
        (var_i4_cor, var_i4_cor_dn5, var_i4_cor_dn6, var_i4_cor_dn7, var_i4_cor_dn8,)
    }
};
        var_i4_cor = assign27510_e32662;
        var_i4_cor_dn5 = assign27510_e32662_d_n5;
        var_i4_cor_dn6 = assign27510_e32662_d_n6;
        var_i4_cor_dn7 = assign27510_e32662_d_n7;
        var_i4_cor_dn8 = assign27510_e32662_d_n8;

        let (assign27520_e32679, assign27520_e32679_d_n5, assign27520_e32679_d_n6, assign27520_e32679_d_n7, assign27520_e32679_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign27520_e32670: f64 = (var_v5 * var_phitdinv);
        let assign27520_e32672: f64 = (assign27520_e32670 * var_mfor1_s);
        let assign27520_e32673: f64 = (assign27520_e32672).exp();
        let assign27520_e32675: f64 = (assign27520_e32673 - 1.0);
        let assign27520_e32676: f64 = (var_isatfor1_s * assign27520_e32675);
        let assign27520_e32677: f64 = (var_i5 - assign27520_e32676);
        (assign27520_e32677, var_i5_dn5, var_i5_dn6, var_i5_dn7, var_i5_dn8,)
    } else {
        (var_i5_cor, var_i5_cor_dn5, var_i5_cor_dn6, var_i5_cor_dn7, var_i5_cor_dn8,)
    }
};
        var_i5_cor = assign27520_e32679;
        var_i5_cor_dn5 = assign27520_e32679_d_n5;
        var_i5_cor_dn6 = assign27520_e32679_d_n6;
        var_i5_cor_dn7 = assign27520_e32679_d_n7;
        var_i5_cor_dn8 = assign27520_e32679_d_n8;

        let assign27530_e32691: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard528 = assign27530_e32691;

        let assign27540_e32698: f64 = if ((var_i4 > 0.0) && (var_i5 > 0.0)) { 1.0 } else { 0.0 };
        var_guard529 = assign27540_e32698;

        let assign27550_e32701: f64 = (var_i4_cor / var_i4);
        let assign27550_e32706: f64 = (var_i5_cor / var_i5);
        let assign27550_e32721: f64 = if (((((assign27550_e32701 > 0.001) || (assign27550_e32706 > 0.001)) && (var_i4_cor > 0.0)) && (var_i5_cor > 0.0)) && (var_i5_cor > var_i4_cor)) { 1.0 } else { 0.0 };
        var_guard530 = assign27550_e32721;

        let (assign27560_e32735, assign27560_e32735_d_n5, assign27560_e32735_d_n6, assign27560_e32735_d_n7, assign27560_e32735_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) && (var_guard529 != 0.0)) && (var_guard530 != 0.0)) {
        let assign27560_e32733: f64 = (var_i4_cor / var_i5_cor);
        (assign27560_e32733, (((var_i4_cor_dn5 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn5)) / (var_i5_cor * var_i5_cor)), (((var_i4_cor_dn6 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn6)) / (var_i5_cor * var_i5_cor)), (((var_i4_cor_dn7 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn7)) / (var_i5_cor * var_i5_cor)), (((var_i4_cor_dn8 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn8)) / (var_i5_cor * var_i5_cor)),)
    } else {
        (var_alphaje, var_alphaje_dn5, var_alphaje_dn6, var_alphaje_dn7, var_alphaje_dn8,)
    }
};
        var_alphaje = assign27560_e32735;
        var_alphaje_dn5 = assign27560_e32735_d_n5;
        var_alphaje_dn6 = assign27560_e32735_d_n6;
        var_alphaje_dn7 = assign27560_e32735_d_n7;
        var_alphaje_dn8 = assign27560_e32735_d_n8;

        let (assign27570_e32754, assign27570_e32754_d_n5, assign27570_e32754_d_n6, assign27570_e32754_d_n7, assign27570_e32754_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) && (var_guard529 != 0.0)) && (var_guard530 != 0.0)) {
        let assign27570_e32747: f64 = (var_alphaje).ln();
        let assign27570_e32748: f64 = (var_phitd * assign27570_e32747);
        let assign27570_e32751: f64 = (var_v4 - var_v5);
        let assign27570_e32752: f64 = (assign27570_e32748 / assign27570_e32751);
        (assign27570_e32752, ((var_phitd * (var_alphaje_dn5 / var_alphaje)) / assign27570_e32751), ((var_phitd * (var_alphaje_dn6 / var_alphaje)) / assign27570_e32751), ((var_phitd * (var_alphaje_dn7 / var_alphaje)) / assign27570_e32751), ((var_phitd * (var_alphaje_dn8 / var_alphaje)) / assign27570_e32751),)
    } else {
        (var_mfor2_s, var_mfor2_s_dn5, var_mfor2_s_dn6, var_mfor2_s_dn7, var_mfor2_s_dn8,)
    }
};
        var_mfor2_s = assign27570_e32754;
        var_mfor2_s_dn5 = assign27570_e32754_d_n5;
        var_mfor2_s_dn6 = assign27570_e32754_d_n6;
        var_mfor2_s_dn7 = assign27570_e32754_d_n7;
        var_mfor2_s_dn8 = assign27570_e32754_d_n8;

        let (assign27580_e32775, assign27580_e32775_d_n5, assign27580_e32775_d_n6, assign27580_e32775_d_n7, assign27580_e32775_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) && (var_guard529 != 0.0)) && (var_guard530 != 0.0)) {
        let assign27580_e32767: f64 = (var_v4 * var_phitdinv);
        let assign27580_e32769: f64 = (assign27580_e32767 * var_mfor2_s);
        let assign27580_e32770: f64 = (assign27580_e32769).exp();
        let assign27580_e32772: f64 = (assign27580_e32770 - 1.0);
        let assign27580_e32773: f64 = (var_i4_cor / assign27580_e32772);
        (assign27580_e32773, (((var_i4_cor_dn5 * assign27580_e32772) - (var_i4_cor * (assign27580_e32770 * (assign27580_e32767 * var_mfor2_s_dn5)))) / (assign27580_e32772 * assign27580_e32772)), (((var_i4_cor_dn6 * assign27580_e32772) - (var_i4_cor * (assign27580_e32770 * (assign27580_e32767 * var_mfor2_s_dn6)))) / (assign27580_e32772 * assign27580_e32772)), (((var_i4_cor_dn7 * assign27580_e32772) - (var_i4_cor * (assign27580_e32770 * (assign27580_e32767 * var_mfor2_s_dn7)))) / (assign27580_e32772 * assign27580_e32772)), (((var_i4_cor_dn8 * assign27580_e32772) - (var_i4_cor * (assign27580_e32770 * (assign27580_e32767 * var_mfor2_s_dn8)))) / (assign27580_e32772 * assign27580_e32772)),)
    } else {
        (var_isatfor2_s, var_isatfor2_s_dn5, var_isatfor2_s_dn6, var_isatfor2_s_dn7, var_isatfor2_s_dn8,)
    }
};
        var_isatfor2_s = assign27580_e32775;
        var_isatfor2_s_dn5 = assign27580_e32775_d_n5;
        var_isatfor2_s_dn6 = assign27580_e32775_d_n6;
        var_isatfor2_s_dn7 = assign27580_e32775_d_n7;
        var_isatfor2_s_dn8 = assign27580_e32775_d_n8;

        let (assign27590_e32805, assign27590_e32805_d_n5, assign27590_e32805_d_n6, assign27590_e32805_d_n7, assign27590_e32805_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) {
        let assign27590_e32785: f64 = (var_v1 * var_phitdinv);
        let assign27590_e32787: f64 = (assign27590_e32785 * var_mfor1_s);
        let assign27590_e32788: f64 = (assign27590_e32787).exp();
        let assign27590_e32790: f64 = (assign27590_e32788 - 1.0);
        let assign27590_e32791: f64 = (var_isatfor1_s * assign27590_e32790);
        let assign27590_e32792: f64 = (var_i1 - assign27590_e32791);
        let assign27590_e32796: f64 = (var_v1 * var_phitdinv);
        let assign27590_e32798: f64 = (assign27590_e32796 * var_mfor2_s);
        let assign27590_e32799: f64 = (assign27590_e32798).exp();
        let assign27590_e32801: f64 = (assign27590_e32799 - 1.0);
        let assign27590_e32802: f64 = (var_isatfor2_s * assign27590_e32801);
        let assign27590_e32803: f64 = (assign27590_e32792 - assign27590_e32802);
        (assign27590_e32803, (var_i1_dn5 - ((var_isatfor2_s_dn5 * assign27590_e32801) + (var_isatfor2_s * (assign27590_e32799 * (assign27590_e32796 * var_mfor2_s_dn5))))), (var_i1_dn6 - ((var_isatfor2_s_dn6 * assign27590_e32801) + (var_isatfor2_s * (assign27590_e32799 * (assign27590_e32796 * var_mfor2_s_dn6))))), (var_i1_dn7 - ((var_isatfor2_s_dn7 * assign27590_e32801) + (var_isatfor2_s * (assign27590_e32799 * (assign27590_e32796 * var_mfor2_s_dn7))))), (var_i1_dn8 - ((var_isatfor2_s_dn8 * assign27590_e32801) + (var_isatfor2_s * (assign27590_e32799 * (assign27590_e32796 * var_mfor2_s_dn8))))),)
    } else {
        (var_i1_cor, var_i1_cor_dn5, var_i1_cor_dn6, var_i1_cor_dn7, var_i1_cor_dn8,)
    }
};
        var_i1_cor = assign27590_e32805;
        var_i1_cor_dn5 = assign27590_e32805_d_n5;
        var_i1_cor_dn6 = assign27590_e32805_d_n6;
        var_i1_cor_dn7 = assign27590_e32805_d_n7;
        var_i1_cor_dn8 = assign27590_e32805_d_n8;

        let (assign27600_e32835, assign27600_e32835_d_n5, assign27600_e32835_d_n6, assign27600_e32835_d_n7, assign27600_e32835_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) {
        let assign27600_e32815: f64 = (var_v2 * var_phitdinv);
        let assign27600_e32817: f64 = (assign27600_e32815 * var_mfor1_s);
        let assign27600_e32818: f64 = (assign27600_e32817).exp();
        let assign27600_e32820: f64 = (assign27600_e32818 - 1.0);
        let assign27600_e32821: f64 = (var_isatfor1_s * assign27600_e32820);
        let assign27600_e32822: f64 = (var_i2 - assign27600_e32821);
        let assign27600_e32826: f64 = (var_v2 * var_phitdinv);
        let assign27600_e32828: f64 = (assign27600_e32826 * var_mfor2_s);
        let assign27600_e32829: f64 = (assign27600_e32828).exp();
        let assign27600_e32831: f64 = (assign27600_e32829 - 1.0);
        let assign27600_e32832: f64 = (var_isatfor2_s * assign27600_e32831);
        let assign27600_e32833: f64 = (assign27600_e32822 - assign27600_e32832);
        (assign27600_e32833, (var_i2_dn5 - ((var_isatfor2_s_dn5 * assign27600_e32831) + (var_isatfor2_s * (assign27600_e32829 * (assign27600_e32826 * var_mfor2_s_dn5))))), (var_i2_dn6 - ((var_isatfor2_s_dn6 * assign27600_e32831) + (var_isatfor2_s * (assign27600_e32829 * (assign27600_e32826 * var_mfor2_s_dn6))))), (var_i2_dn7 - ((var_isatfor2_s_dn7 * assign27600_e32831) + (var_isatfor2_s * (assign27600_e32829 * (assign27600_e32826 * var_mfor2_s_dn7))))), (var_i2_dn8 - ((var_isatfor2_s_dn8 * assign27600_e32831) + (var_isatfor2_s * (assign27600_e32829 * (assign27600_e32826 * var_mfor2_s_dn8))))),)
    } else {
        (var_i2_cor, var_i2_cor_dn5, var_i2_cor_dn6, var_i2_cor_dn7, var_i2_cor_dn8,)
    }
};
        var_i2_cor = assign27600_e32835;
        var_i2_cor_dn5 = assign27600_e32835_d_n5;
        var_i2_cor_dn6 = assign27600_e32835_d_n6;
        var_i2_cor_dn7 = assign27600_e32835_d_n7;
        var_i2_cor_dn8 = assign27600_e32835_d_n8;

        let (assign27610_e32865, assign27610_e32865_d_n5, assign27610_e32865_d_n6, assign27610_e32865_d_n7, assign27610_e32865_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) {
        let assign27610_e32845: f64 = (var_v3 * var_phitdinv);
        let assign27610_e32847: f64 = (assign27610_e32845 * var_mfor1_s);
        let assign27610_e32848: f64 = (assign27610_e32847).exp();
        let assign27610_e32850: f64 = (assign27610_e32848 - 1.0);
        let assign27610_e32851: f64 = (var_isatfor1_s * assign27610_e32850);
        let assign27610_e32852: f64 = (var_i3 - assign27610_e32851);
        let assign27610_e32856: f64 = (var_v3 * var_phitdinv);
        let assign27610_e32858: f64 = (assign27610_e32856 * var_mfor2_s);
        let assign27610_e32859: f64 = (assign27610_e32858).exp();
        let assign27610_e32861: f64 = (assign27610_e32859 - 1.0);
        let assign27610_e32862: f64 = (var_isatfor2_s * assign27610_e32861);
        let assign27610_e32863: f64 = (assign27610_e32852 - assign27610_e32862);
        (assign27610_e32863, (var_i3_dn5 - ((var_isatfor2_s_dn5 * assign27610_e32861) + (var_isatfor2_s * (assign27610_e32859 * (assign27610_e32856 * var_mfor2_s_dn5))))), (var_i3_dn6 - ((var_isatfor2_s_dn6 * assign27610_e32861) + (var_isatfor2_s * (assign27610_e32859 * (assign27610_e32856 * var_mfor2_s_dn6))))), (var_i3_dn7 - ((var_isatfor2_s_dn7 * assign27610_e32861) + (var_isatfor2_s * (assign27610_e32859 * (assign27610_e32856 * var_mfor2_s_dn7))))), (var_i3_dn8 - ((var_isatfor2_s_dn8 * assign27610_e32861) + (var_isatfor2_s * (assign27610_e32859 * (assign27610_e32856 * var_mfor2_s_dn8))))),)
    } else {
        (var_i3_cor, var_i3_cor_dn5, var_i3_cor_dn6, var_i3_cor_dn7, var_i3_cor_dn8,)
    }
};
        var_i3_cor = assign27610_e32865;
        var_i3_cor_dn5 = assign27610_e32865_d_n5;
        var_i3_cor_dn6 = assign27610_e32865_d_n6;
        var_i3_cor_dn7 = assign27610_e32865_d_n7;
        var_i3_cor_dn8 = assign27610_e32865_d_n8;

        let assign27620_e32876: f64 = if (((var_i1 < 0.0) && (var_i2 < 0.0)) && (var_i3 < 0.0)) { 1.0 } else { 0.0 };
        var_guard531 = assign27620_e32876;

        let assign27630_e32879: f64 = (var_i1_cor / var_i1);
        let assign27630_e32884: f64 = (var_i2_cor / var_i2);
        let assign27630_e32890: f64 = (var_i3_cor / var_i3);
        let assign27630_e32905: f64 = if ((((((assign27630_e32879 > 0.001) || (assign27630_e32884 > 0.001)) || (assign27630_e32890 > 0.001)) && (var_i1_cor < 0.0)) && (var_i2_cor < 0.0)) && (var_i3_cor < 0.0)) { 1.0 } else { 0.0 };
        var_guard532 = assign27630_e32905;

        let (assign27640_e32919, assign27640_e32919_d_n5, assign27640_e32919_d_n6, assign27640_e32919_d_n7, assign27640_e32919_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) {
        let assign27640_e32917: f64 = (var_i1_cor / var_i2_cor);
        (assign27640_e32917, (((var_i1_cor_dn5 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn5)) / (var_i2_cor * var_i2_cor)), (((var_i1_cor_dn6 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn6)) / (var_i2_cor * var_i2_cor)), (((var_i1_cor_dn7 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn7)) / (var_i2_cor * var_i2_cor)), (((var_i1_cor_dn8 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn8)) / (var_i2_cor * var_i2_cor)),)
    } else {
        (var_alphaje, var_alphaje_dn5, var_alphaje_dn6, var_alphaje_dn7, var_alphaje_dn8,)
    }
};
        var_alphaje = assign27640_e32919;
        var_alphaje_dn5 = assign27640_e32919_d_n5;
        var_alphaje_dn6 = assign27640_e32919_d_n6;
        var_alphaje_dn7 = assign27640_e32919_d_n7;
        var_alphaje_dn8 = assign27640_e32919_d_n8;

        let (assign27650_e32939, assign27650_e32939_d_n5, assign27650_e32939_d_n6, assign27650_e32939_d_n7, assign27650_e32939_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) {
        let assign27650_e32930: f64 = (-var_phitd);
        let assign27650_e32932: f64 = (var_alphaje).ln();
        let assign27650_e32933: f64 = (assign27650_e32930 * assign27650_e32932);
        let assign27650_e32936: f64 = (var_v1 - var_v2);
        let assign27650_e32937: f64 = (assign27650_e32933 / assign27650_e32936);
        (assign27650_e32937, ((assign27650_e32930 * (var_alphaje_dn5 / var_alphaje)) / assign27650_e32936), ((assign27650_e32930 * (var_alphaje_dn6 / var_alphaje)) / assign27650_e32936), ((assign27650_e32930 * (var_alphaje_dn7 / var_alphaje)) / assign27650_e32936), ((assign27650_e32930 * (var_alphaje_dn8 / var_alphaje)) / assign27650_e32936),)
    } else {
        (var_m0_rev, var_m0_rev_dn5, var_m0_rev_dn6, var_m0_rev_dn7, var_m0_rev_dn8,)
    }
};
        var_m0_rev = assign27650_e32939;
        var_m0_rev_dn5 = assign27650_e32939_d_n5;
        var_m0_rev_dn6 = assign27650_e32939_d_n6;
        var_m0_rev_dn7 = assign27650_e32939_d_n7;
        var_m0_rev_dn8 = assign27650_e32939_d_n8;

        let (assign27660_e32955,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) {
        let assign27660_e32952: f64 = (var_v2 - var_v1);
        let assign27660_e32953: f64 = (var_v2 / assign27660_e32952);
        (assign27660_e32953,)
    } else {
        (var_tt0,)
    }
};
        var_tt0 = assign27660_e32955;

        let (assign27670_e32977, assign27670_e32977_d_n5, assign27670_e32977_d_n6, assign27670_e32977_d_n7, assign27670_e32977_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) {
        let assign27670_e32968: f64 = (var_alphaje - 1.0);
        let assign27670_e32969: f64 = (var_phitd * assign27670_e32968);
        let assign27670_e32972: f64 = (var_alphaje).powf(var_tt0);
        let assign27670_e32974: f64 = (assign27670_e32972 - 1.0);
        let assign27670_e32975: f64 = (assign27670_e32969 * assign27670_e32974);
        (assign27670_e32975, (((var_phitd * var_alphaje_dn5) * assign27670_e32974) + (assign27670_e32969 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn5)) } } else { (assign27670_e32972 * (var_tt0 * (var_alphaje_dn5 / var_alphaje))) })), (((var_phitd * var_alphaje_dn6) * assign27670_e32974) + (assign27670_e32969 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn6)) } } else { (assign27670_e32972 * (var_tt0 * (var_alphaje_dn6 / var_alphaje))) })), (((var_phitd * var_alphaje_dn7) * assign27670_e32974) + (assign27670_e32969 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn7)) } } else { (assign27670_e32972 * (var_tt0 * (var_alphaje_dn7 / var_alphaje))) })), (((var_phitd * var_alphaje_dn8) * assign27670_e32974) + (assign27670_e32969 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn8)) } } else { (assign27670_e32972 * (var_tt0 * (var_alphaje_dn8 / var_alphaje))) })),)
    } else {
        (var_tt1, var_tt1_dn5, var_tt1_dn6, var_tt1_dn7, var_tt1_dn8,)
    }
};
        var_tt1 = assign27670_e32977;
        var_tt1_dn5 = assign27670_e32977_d_n5;
        var_tt1_dn6 = assign27670_e32977_d_n6;
        var_tt1_dn7 = assign27670_e32977_d_n7;
        var_tt1_dn8 = assign27670_e32977_d_n8;

        let (assign27680_e32993,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) {
        let assign27680_e32990: f64 = (var_v1 - var_v2);
        let assign27680_e32991: f64 = (var_v1 / assign27680_e32990);
        (assign27680_e32991,)
    } else {
        (var_tt0,)
    }
};
        var_tt0 = assign27680_e32993;

        let (assign27690_e33017, assign27690_e33017_d_n5, assign27690_e33017_d_n6, assign27690_e33017_d_n7, assign27690_e33017_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) {
        let assign27690_e33005: f64 = (var_alphaje).powf(var_tt0);
        let assign27690_e33008: f64 = (var_v2 - var_v1);
        let assign27690_e33009: f64 = (assign27690_e33005 * assign27690_e33008);
        let assign27690_e33012: f64 = (var_alphaje * var_v1);
        let assign27690_e33013: f64 = (assign27690_e33009 + assign27690_e33012);
        let assign27690_e33015: f64 = (assign27690_e33013 - var_v2);
        (assign27690_e33015, ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn5)) } } else { (assign27690_e33005 * (var_tt0 * (var_alphaje_dn5 / var_alphaje))) } * assign27690_e33008) + (var_alphaje_dn5 * var_v1)), ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn6)) } } else { (assign27690_e33005 * (var_tt0 * (var_alphaje_dn6 / var_alphaje))) } * assign27690_e33008) + (var_alphaje_dn6 * var_v1)), ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn7)) } } else { (assign27690_e33005 * (var_tt0 * (var_alphaje_dn7 / var_alphaje))) } * assign27690_e33008) + (var_alphaje_dn7 * var_v1)), ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn8)) } } else { (assign27690_e33005 * (var_tt0 * (var_alphaje_dn8 / var_alphaje))) } * assign27690_e33008) + (var_alphaje_dn8 * var_v1)),)
    } else {
        (var_tt2, var_tt2_dn5, var_tt2_dn6, var_tt2_dn7, var_tt2_dn8,)
    }
};
        var_tt2 = assign27690_e33017;
        var_tt2_dn5 = assign27690_e33017_d_n5;
        var_tt2_dn6 = assign27690_e33017_d_n6;
        var_tt2_dn7 = assign27690_e33017_d_n7;
        var_tt2_dn8 = assign27690_e33017_d_n8;

        let (assign27700_e33031, assign27700_e33031_d_n5, assign27700_e33031_d_n6, assign27700_e33031_d_n7, assign27700_e33031_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) {
        let assign27700_e33029: f64 = (var_tt1 / var_tt2);
        (assign27700_e33029, (((var_tt1_dn5 * var_tt2) - (var_tt1 * var_tt2_dn5)) / (var_tt2 * var_tt2)), (((var_tt1_dn6 * var_tt2) - (var_tt1 * var_tt2_dn6)) / (var_tt2 * var_tt2)), (((var_tt1_dn7 * var_tt2) - (var_tt1 * var_tt2_dn7)) / (var_tt2 * var_tt2)), (((var_tt1_dn8 * var_tt2) - (var_tt1 * var_tt2_dn8)) / (var_tt2 * var_tt2)),)
    } else {
        (var_mcor_rev, var_mcor_rev_dn5, var_mcor_rev_dn6, var_mcor_rev_dn7, var_mcor_rev_dn8,)
    }
};
        var_mcor_rev = assign27700_e33031;
        var_mcor_rev_dn5 = assign27700_e33031_d_n5;
        var_mcor_rev_dn6 = assign27700_e33031_d_n6;
        var_mcor_rev_dn7 = assign27700_e33031_d_n7;
        var_mcor_rev_dn8 = assign27700_e33031_d_n8;

        let (assign27710_e33045, assign27710_e33045_d_n5, assign27710_e33045_d_n6, assign27710_e33045_d_n7, assign27710_e33045_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) {
        let assign27710_e33043: f64 = (var_m0_rev + var_mcor_rev);
        (assign27710_e33043, (var_m0_rev_dn5 + var_mcor_rev_dn5), (var_m0_rev_dn6 + var_mcor_rev_dn6), (var_m0_rev_dn7 + var_mcor_rev_dn7), (var_m0_rev_dn8 + var_mcor_rev_dn8),)
    } else {
        (var_mrev_s, var_mrev_s_dn5, var_mrev_s_dn6, var_mrev_s_dn7, var_mrev_s_dn8,)
    }
};
        var_mrev_s = assign27710_e33045;
        var_mrev_s_dn5 = assign27710_e33045_d_n5;
        var_mrev_s_dn6 = assign27710_e33045_d_n6;
        var_mrev_s_dn7 = assign27710_e33045_d_n7;
        var_mrev_s_dn8 = assign27710_e33045_d_n8;

        let assign27720_e33048: f64 = (var_v3 * var_phitdinv);
        let assign27720_e33050: f64 = (assign27720_e33048 * var_mrev_s);
        let assign27720_e33051: f64 = (assign27720_e33050).abs();
        let assign27720_e33053: f64 = if assign27720_e33051 < 1e-6 { 1.0 } else { 0.0 };
        var_guard533 = assign27720_e33053;

        let (assign27730_e33067,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) && (var_guard533 != 0.0)) {
        (1.0,)
    } else {
        (var_m0flag_s,)
    }
};
        var_m0flag_s = assign27730_e33067;

        let (assign27740_e33091, assign27740_e33091_d_n5, assign27740_e33091_d_n6, assign27740_e33091_d_n7, assign27740_e33091_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) && (var_guard533 != 0.0)) {
        let assign27740_e33082: f64 = (1.0 / var_v3);
        let assign27740_e33085: f64 = (0.5 * var_phitdinv);
        let assign27740_e33087: f64 = (assign27740_e33085 * var_mrev_s);
        let assign27740_e33088: f64 = (assign27740_e33082 + assign27740_e33087);
        let assign27740_e33089: f64 = (var_i3_cor * assign27740_e33088);
        (assign27740_e33089, ((var_i3_cor_dn5 * assign27740_e33088) + (var_i3_cor * (assign27740_e33085 * var_mrev_s_dn5))), ((var_i3_cor_dn6 * assign27740_e33088) + (var_i3_cor * (assign27740_e33085 * var_mrev_s_dn6))), ((var_i3_cor_dn7 * assign27740_e33088) + (var_i3_cor * (assign27740_e33085 * var_mrev_s_dn7))), ((var_i3_cor_dn8 * assign27740_e33088) + (var_i3_cor * (assign27740_e33085 * var_mrev_s_dn8))),)
    } else {
        (var_isatrev_s, var_isatrev_s_dn5, var_isatrev_s_dn6, var_isatrev_s_dn7, var_isatrev_s_dn8,)
    }
};
        var_isatrev_s = assign27740_e33091;
        var_isatrev_s_dn5 = assign27740_e33091_d_n5;
        var_isatrev_s_dn6 = assign27740_e33091_d_n6;
        var_isatrev_s_dn7 = assign27740_e33091_d_n7;
        var_isatrev_s_dn8 = assign27740_e33091_d_n8;

        let (assign27750_e33114, assign27750_e33114_d_n5, assign27750_e33114_d_n6, assign27750_e33114_d_n7, assign27750_e33114_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) && (var_guard533 != 0.0)) {
        let assign27750_e33104: f64 = (-0.5);
        let assign27750_e33106: f64 = (assign27750_e33104 * var_i3_cor);
        let assign27750_e33108: f64 = (assign27750_e33106 * var_mrev_s);
        let assign27750_e33110: f64 = (assign27750_e33108 * var_phitdinv);
        let assign27750_e33112: f64 = (assign27750_e33110 / var_v3);
        (assign27750_e33112, (((((assign27750_e33104 * var_i3_cor_dn5) * var_mrev_s) + (assign27750_e33106 * var_mrev_s_dn5)) * var_phitdinv) / var_v3), (((((assign27750_e33104 * var_i3_cor_dn6) * var_mrev_s) + (assign27750_e33106 * var_mrev_s_dn6)) * var_phitdinv) / var_v3), (((((assign27750_e33104 * var_i3_cor_dn7) * var_mrev_s) + (assign27750_e33106 * var_mrev_s_dn7)) * var_phitdinv) / var_v3), (((((assign27750_e33104 * var_i3_cor_dn8) * var_mrev_s) + (assign27750_e33106 * var_mrev_s_dn8)) * var_phitdinv) / var_v3),)
    } else {
        (var_mrev_s, var_mrev_s_dn5, var_mrev_s_dn6, var_mrev_s_dn7, var_mrev_s_dn8,)
    }
};
        var_mrev_s = assign27750_e33114;
        var_mrev_s_dn5 = assign27750_e33114_d_n5;
        var_mrev_s_dn6 = assign27750_e33114_d_n6;
        var_mrev_s_dn7 = assign27750_e33114_d_n7;
        var_mrev_s_dn8 = assign27750_e33114_d_n8;

        let (assign27760_e33129,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) && (var_guard533 == 0.0)) {
        (0.0,)
    } else {
        (var_m0flag_s,)
    }
};
        var_m0flag_s = assign27760_e33129;

        *var_alphaje_slot = var_alphaje;
        *var_alphaje_dn5_slot = var_alphaje_dn5;
        *var_alphaje_dn6_slot = var_alphaje_dn6;
        *var_alphaje_dn7_slot = var_alphaje_dn7;
        *var_alphaje_dn8_slot = var_alphaje_dn8;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_guard525_slot = var_guard525;
        *var_guard526_slot = var_guard526;
        *var_guard527_slot = var_guard527;
        *var_guard528_slot = var_guard528;
        *var_guard529_slot = var_guard529;
        *var_guard530_slot = var_guard530;
        *var_guard531_slot = var_guard531;
        *var_guard532_slot = var_guard532;
        *var_guard533_slot = var_guard533;
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
        *var_i5_slot = var_i5;
        *var_i5_cor_slot = var_i5_cor;
        *var_i5_cor_dn5_slot = var_i5_cor_dn5;
        *var_i5_cor_dn6_slot = var_i5_cor_dn6;
        *var_i5_cor_dn7_slot = var_i5_cor_dn7;
        *var_i5_cor_dn8_slot = var_i5_cor_dn8;
        *var_i5_dn5_slot = var_i5_dn5;
        *var_i5_dn6_slot = var_i5_dn6;
        *var_i5_dn7_slot = var_i5_dn7;
        *var_i5_dn8_slot = var_i5_dn8;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn5_slot = var_ijungat_dn5;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_isatfor1_s_slot = var_isatfor1_s;
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
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
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
    }

    pub(super) fn stamp_transient_block_54(
        p: &Parameters,
        var_abdrain_i: f64,
        var_absource_i: f64,
        var_cjobot: f64,
        var_cjogat: f64,
        var_cjosti: f64,
        var_exp_vmax_over_phitd_d: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard528: f64,
        var_guard531: f64,
        var_guard532: f64,
        var_guard533: f64,
        var_i3_cor: f64,
        var_i3_cor_dn5: f64,
        var_i3_cor_dn6: f64,
        var_i3_cor_dn7: f64,
        var_i3_cor_dn8: f64,
        var_isatfor1_s: f64,
        var_isatfor2_s: f64,
        var_isatfor2_s_dn5: f64,
        var_isatfor2_s_dn6: f64,
        var_isatfor2_s_dn7: f64,
        var_isatfor2_s_dn8: f64,
        var_lgdrain_i: f64,
        var_lgsource_i: f64,
        var_lsdrain_i: f64,
        var_lssource_i: f64,
        var_mrev_s: f64,
        var_mrev_s_dn5: f64,
        var_mrev_s_dn6: f64,
        var_mrev_s_dn7: f64,
        var_mrev_s_dn8: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_vbimin_d: f64,
        var_vjunrefd_i: f64,
        var_vmax_d: f64,
        var_expxhf1_s_slot: &mut f64,
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
        var_guard534_slot: &mut f64,
        var_guard535_slot: &mut f64,
        var_guard536_slot: &mut f64,
        var_guard537_slot: &mut f64,
        var_guard538_slot: &mut f64,
        var_guard539_slot: &mut f64,
        var_guard540_slot: &mut f64,
        var_guard541_slot: &mut f64,
        var_guard542_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_isatrev_s_slot: &mut f64,
        var_isatrev_s_dn5_slot: &mut f64,
        var_isatrev_s_dn6_slot: &mut f64,
        var_isatrev_s_dn7_slot: &mut f64,
        var_isatrev_s_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_v1_slot: &mut f64,
        var_v2_slot: &mut f64,
        var_v3_slot: &mut f64,
        var_v4_slot: &mut f64,
        var_v5_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vjlim_slot: &mut f64,
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
        var_z_slot: &mut f64,
        var_zflagbot_s_slot: &mut f64,
        var_zflaggat_s_slot: &mut f64,
        var_zflagsti_s_slot: &mut f64,
        var_zfrac_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_expxhf1_s: f64 = *var_expxhf1_s_slot;
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
        let mut var_guard534: f64 = *var_guard534_slot;
        let mut var_guard535: f64 = *var_guard535_slot;
        let mut var_guard536: f64 = *var_guard536_slot;
        let mut var_guard537: f64 = *var_guard537_slot;
        let mut var_guard538: f64 = *var_guard538_slot;
        let mut var_guard539: f64 = *var_guard539_slot;
        let mut var_guard540: f64 = *var_guard540_slot;
        let mut var_guard541: f64 = *var_guard541_slot;
        let mut var_guard542: f64 = *var_guard542_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_isatrev_s: f64 = *var_isatrev_s_slot;
        let mut var_isatrev_s_dn5: f64 = *var_isatrev_s_dn5_slot;
        let mut var_isatrev_s_dn6: f64 = *var_isatrev_s_dn6_slot;
        let mut var_isatrev_s_dn7: f64 = *var_isatrev_s_dn7_slot;
        let mut var_isatrev_s_dn8: f64 = *var_isatrev_s_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_v1: f64 = *var_v1_slot;
        let mut var_v2: f64 = *var_v2_slot;
        let mut var_v3: f64 = *var_v3_slot;
        let mut var_v4: f64 = *var_v4_slot;
        let mut var_v5: f64 = *var_v5_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
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
        let mut var_z: f64 = *var_z_slot;
        let mut var_zflagbot_s: f64 = *var_zflagbot_s_slot;
        let mut var_zflaggat_s: f64 = *var_zflaggat_s_slot;
        let mut var_zflagsti_s: f64 = *var_zflagsti_s_slot;
        let mut var_zfrac: f64 = *var_zfrac_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign27770_e33155, assign27770_e33155_d_n5, assign27770_e33155_d_n6, assign27770_e33155_d_n7, assign27770_e33155_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard528 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) && (var_guard533 == 0.0)) {
        let assign27770_e33143: f64 = (-var_i3_cor);
        let assign27770_e33145: f64 = (-var_v3);
        let assign27770_e33147: f64 = (assign27770_e33145 * var_phitdinv);
        let assign27770_e33149: f64 = (assign27770_e33147 * var_mrev_s);
        let assign27770_e33150: f64 = (assign27770_e33149).exp();
        let assign27770_e33152: f64 = (assign27770_e33150 - 1.0);
        let assign27770_e33153: f64 = (assign27770_e33143 / assign27770_e33152);
        (assign27770_e33153, ((((-var_i3_cor_dn5) * assign27770_e33152) - (assign27770_e33143 * (assign27770_e33150 * (assign27770_e33147 * var_mrev_s_dn5)))) / (assign27770_e33152 * assign27770_e33152)), ((((-var_i3_cor_dn6) * assign27770_e33152) - (assign27770_e33143 * (assign27770_e33150 * (assign27770_e33147 * var_mrev_s_dn6)))) / (assign27770_e33152 * assign27770_e33152)), ((((-var_i3_cor_dn7) * assign27770_e33152) - (assign27770_e33143 * (assign27770_e33150 * (assign27770_e33147 * var_mrev_s_dn7)))) / (assign27770_e33152 * assign27770_e33152)), ((((-var_i3_cor_dn8) * assign27770_e33152) - (assign27770_e33143 * (assign27770_e33150 * (assign27770_e33147 * var_mrev_s_dn8)))) / (assign27770_e33152 * assign27770_e33152)),)
    } else {
        (var_isatrev_s, var_isatrev_s_dn5, var_isatrev_s_dn6, var_isatrev_s_dn7, var_isatrev_s_dn8,)
    }
};
        var_isatrev_s = assign27770_e33155;
        var_isatrev_s_dn5 = assign27770_e33155_d_n5;
        var_isatrev_s_dn6 = assign27770_e33155_d_n6;
        var_isatrev_s_dn7 = assign27770_e33155_d_n7;
        var_isatrev_s_dn8 = assign27770_e33155_d_n8;

        let (assign27780_e33173,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign27780_e33162: f64 = (var_absource_i * var_cjobot);
        let assign27780_e33165: f64 = (var_lssource_i * var_cjosti);
        let assign27780_e33166: f64 = (assign27780_e33162 + assign27780_e33165);
        let assign27780_e33169: f64 = (var_lgsource_i * var_cjogat);
        let assign27780_e33170: f64 = (assign27780_e33166 + assign27780_e33169);
        let assign27780_e33171: f64 = (p.p929 * assign27780_e33170);
        (assign27780_e33171,)
    } else {
        (var_zfrac,)
    }
};
        var_zfrac = assign27780_e33173;

        let assign27790_e33176: f64 = (var_absource_i * var_cjobot);
        let assign27790_e33178: f64 = if assign27790_e33176 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard534 = assign27790_e33178;

        let (assign27800_e33186,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard534 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagbot_s,)
    }
};
        var_zflagbot_s = assign27800_e33186;

        let assign27810_e33189: f64 = (var_lssource_i * var_cjosti);
        let assign27810_e33191: f64 = if assign27810_e33189 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard535 = assign27810_e33191;

        let (assign27820_e33199,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard535 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagsti_s,)
    }
};
        var_zflagsti_s = assign27820_e33199;

        let assign27830_e33202: f64 = (var_lgsource_i * var_cjogat);
        let assign27830_e33204: f64 = if assign27830_e33202 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard536 = assign27830_e33204;

        let (assign27840_e33212,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard536 != 0.0)) {
        (0.0,)
    } else {
        (var_zflaggat_s,)
    }
};
        var_zflaggat_s = assign27840_e33212;

        let assign27850_e33224: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard537 = assign27850_e33224;

        let (assign27860_e33239,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard537 != 0.0)) {
        let assign27860_e33232: f64 = (0.5 * p.p822);
        let assign27860_e33235: f64 = (var_isatfor1_s + 1e-21);
        let assign27860_e33236: f64 = (assign27860_e33232 / assign27860_e33235);
        let assign27860_e33237: f64 = (assign27860_e33236).ln();
        (assign27860_e33237,)
    } else {
        (var_xhighf1_s,)
    }
};
        var_xhighf1_s = assign27860_e33239;

        let (assign27870_e33254, assign27870_e33254_d_n5, assign27870_e33254_d_n6, assign27870_e33254_d_n7, assign27870_e33254_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard537 != 0.0)) {
        let assign27870_e33247: f64 = (0.5 * p.p822);
        let assign27870_e33250: f64 = (var_isatfor2_s + 1e-21);
        let assign27870_e33251: f64 = (assign27870_e33247 / assign27870_e33250);
        let assign27870_e33252: f64 = (assign27870_e33251).ln();
        (assign27870_e33252, ((-((assign27870_e33247 * var_isatfor2_s_dn5) / (assign27870_e33250 * assign27870_e33250))) / assign27870_e33251), ((-((assign27870_e33247 * var_isatfor2_s_dn6) / (assign27870_e33250 * assign27870_e33250))) / assign27870_e33251), ((-((assign27870_e33247 * var_isatfor2_s_dn7) / (assign27870_e33250 * assign27870_e33250))) / assign27870_e33251), ((-((assign27870_e33247 * var_isatfor2_s_dn8) / (assign27870_e33250 * assign27870_e33250))) / assign27870_e33251),)
    } else {
        (var_xhighf2_s, var_xhighf2_s_dn5, var_xhighf2_s_dn6, var_xhighf2_s_dn7, var_xhighf2_s_dn8,)
    }
};
        var_xhighf2_s = assign27870_e33254;
        var_xhighf2_s_dn5 = assign27870_e33254_d_n5;
        var_xhighf2_s_dn6 = assign27870_e33254_d_n6;
        var_xhighf2_s_dn7 = assign27870_e33254_d_n7;
        var_xhighf2_s_dn8 = assign27870_e33254_d_n8;

        let (assign27880_e33270, assign27880_e33270_d_n5, assign27880_e33270_d_n6, assign27880_e33270_d_n7, assign27880_e33270_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard537 != 0.0)) {
        let assign27880_e33262: f64 = (0.5 * p.p822);
        let assign27880_e33264: f64 = (var_isatrev_s).abs();
        let assign27880_e33266: f64 = (assign27880_e33264 + 1e-21);
        let assign27880_e33267: f64 = (assign27880_e33262 / assign27880_e33266);
        let assign27880_e33268: f64 = (assign27880_e33267).ln();
        (assign27880_e33268, ((-((assign27880_e33262 * if var_isatrev_s >= 0.0 { var_isatrev_s_dn5 } else { (-var_isatrev_s_dn5) }) / (assign27880_e33266 * assign27880_e33266))) / assign27880_e33267), ((-((assign27880_e33262 * if var_isatrev_s >= 0.0 { var_isatrev_s_dn6 } else { (-var_isatrev_s_dn6) }) / (assign27880_e33266 * assign27880_e33266))) / assign27880_e33267), ((-((assign27880_e33262 * if var_isatrev_s >= 0.0 { var_isatrev_s_dn7 } else { (-var_isatrev_s_dn7) }) / (assign27880_e33266 * assign27880_e33266))) / assign27880_e33267), ((-((assign27880_e33262 * if var_isatrev_s >= 0.0 { var_isatrev_s_dn8 } else { (-var_isatrev_s_dn8) }) / (assign27880_e33266 * assign27880_e33266))) / assign27880_e33267),)
    } else {
        (var_xhighr_s, var_xhighr_s_dn5, var_xhighr_s_dn6, var_xhighr_s_dn7, var_xhighr_s_dn8,)
    }
};
        var_xhighr_s = assign27880_e33270;
        var_xhighr_s_dn5 = assign27880_e33270_d_n5;
        var_xhighr_s_dn6 = assign27880_e33270_d_n6;
        var_xhighr_s_dn7 = assign27880_e33270_d_n7;
        var_xhighr_s_dn8 = assign27880_e33270_d_n8;

        let (assign27890_e33278,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign27890_e33276: f64 = (var_xhighf1_s).min(230.25850929940458);
        (assign27890_e33276,)
    } else {
        (var_xhighf1_s,)
    }
};
        var_xhighf1_s = assign27890_e33278;

        let (assign27900_e33285,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign27900_e33283: f64 = (var_xhighf1_s).exp();
        (assign27900_e33283,)
    } else {
        (var_expxhf1_s,)
    }
};
        var_expxhf1_s = assign27900_e33285;

        let (assign27910_e33293, assign27910_e33293_d_n5, assign27910_e33293_d_n6, assign27910_e33293_d_n7, assign27910_e33293_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign27910_e33291: f64 = (var_xhighf2_s).min(230.25850929940458);
        (assign27910_e33291, if var_xhighf2_s <= 230.25850929940458 { var_xhighf2_s_dn5 } else { 0.0 }, if var_xhighf2_s <= 230.25850929940458 { var_xhighf2_s_dn6 } else { 0.0 }, if var_xhighf2_s <= 230.25850929940458 { var_xhighf2_s_dn7 } else { 0.0 }, if var_xhighf2_s <= 230.25850929940458 { var_xhighf2_s_dn8 } else { 0.0 },)
    } else {
        (var_xhighf2_s, var_xhighf2_s_dn5, var_xhighf2_s_dn6, var_xhighf2_s_dn7, var_xhighf2_s_dn8,)
    }
};
        var_xhighf2_s = assign27910_e33293;
        var_xhighf2_s_dn5 = assign27910_e33293_d_n5;
        var_xhighf2_s_dn6 = assign27910_e33293_d_n6;
        var_xhighf2_s_dn7 = assign27910_e33293_d_n7;
        var_xhighf2_s_dn8 = assign27910_e33293_d_n8;

        let (assign27920_e33300, assign27920_e33300_d_n5, assign27920_e33300_d_n6, assign27920_e33300_d_n7, assign27920_e33300_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign27920_e33298: f64 = (var_xhighf2_s).exp();
        (assign27920_e33298, (assign27920_e33298 * var_xhighf2_s_dn5), (assign27920_e33298 * var_xhighf2_s_dn6), (assign27920_e33298 * var_xhighf2_s_dn7), (assign27920_e33298 * var_xhighf2_s_dn8),)
    } else {
        (var_expxhf2_s, var_expxhf2_s_dn5, var_expxhf2_s_dn6, var_expxhf2_s_dn7, var_expxhf2_s_dn8,)
    }
};
        var_expxhf2_s = assign27920_e33300;
        var_expxhf2_s_dn5 = assign27920_e33300_d_n5;
        var_expxhf2_s_dn6 = assign27920_e33300_d_n6;
        var_expxhf2_s_dn7 = assign27920_e33300_d_n7;
        var_expxhf2_s_dn8 = assign27920_e33300_d_n8;

        let (assign27930_e33308, assign27930_e33308_d_n5, assign27930_e33308_d_n6, assign27930_e33308_d_n7, assign27930_e33308_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign27930_e33306: f64 = (var_xhighr_s).min(230.25850929940458);
        (assign27930_e33306, if var_xhighr_s <= 230.25850929940458 { var_xhighr_s_dn5 } else { 0.0 }, if var_xhighr_s <= 230.25850929940458 { var_xhighr_s_dn6 } else { 0.0 }, if var_xhighr_s <= 230.25850929940458 { var_xhighr_s_dn7 } else { 0.0 }, if var_xhighr_s <= 230.25850929940458 { var_xhighr_s_dn8 } else { 0.0 },)
    } else {
        (var_xhighr_s, var_xhighr_s_dn5, var_xhighr_s_dn6, var_xhighr_s_dn7, var_xhighr_s_dn8,)
    }
};
        var_xhighr_s = assign27930_e33308;
        var_xhighr_s_dn5 = assign27930_e33308_d_n5;
        var_xhighr_s_dn6 = assign27930_e33308_d_n6;
        var_xhighr_s_dn7 = assign27930_e33308_d_n7;
        var_xhighr_s_dn8 = assign27930_e33308_d_n8;

        let (assign27940_e33315, assign27940_e33315_d_n5, assign27940_e33315_d_n6, assign27940_e33315_d_n7, assign27940_e33315_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign27940_e33313: f64 = (var_xhighr_s).exp();
        (assign27940_e33313, (assign27940_e33313 * var_xhighr_s_dn5), (assign27940_e33313 * var_xhighr_s_dn6), (assign27940_e33313 * var_xhighr_s_dn7), (assign27940_e33313 * var_xhighr_s_dn8),)
    } else {
        (var_expxhr_s, var_expxhr_s_dn5, var_expxhr_s_dn6, var_expxhr_s_dn7, var_expxhr_s_dn8,)
    }
};
        var_expxhr_s = assign27940_e33315;
        var_expxhr_s_dn5 = assign27940_e33315_d_n5;
        var_expxhr_s_dn6 = assign27940_e33315_d_n6;
        var_expxhr_s_dn7 = assign27940_e33315_d_n7;
        var_expxhr_s_dn8 = assign27940_e33315_d_n8;

        let (assign27950_e33321,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.4,)
    } else {
        (var_fracna,)
    }
};
        var_fracna = assign27950_e33321;

        let (assign27960_e33327,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.65,)
    } else {
        (var_fracnb,)
    }
};
        var_fracnb = assign27960_e33327;

        let (assign27970_e33333,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.8,)
    } else {
        (var_fraci,)
    }
};
        var_fraci = assign27970_e33333;

        let (assign27980_e33342,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign27980_e33338: f64 = (-var_fracna);
        let assign27980_e33340: f64 = (assign27980_e33338 * var_vjunrefd_i);
        (assign27980_e33340,)
    } else {
        (var_v1,)
    }
};
        var_v1 = assign27980_e33342;

        let (assign27990_e33351,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign27990_e33347: f64 = (-var_fracnb);
        let assign27990_e33349: f64 = (assign27990_e33347 * var_vjunrefd_i);
        (assign27990_e33349,)
    } else {
        (var_v2,)
    }
};
        var_v2 = assign27990_e33351;

        let (assign28000_e33360,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign28000_e33356: f64 = (-var_fraci);
        let assign28000_e33358: f64 = (assign28000_e33356 * var_vjunrefd_i);
        (assign28000_e33358,)
    } else {
        (var_v3,)
    }
};
        var_v3 = assign28000_e33360;

        let (assign28010_e33366,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.1,)
    } else {
        (var_v4,)
    }
};
        var_v4 = assign28010_e33366;

        let (assign28020_e33372,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.2,)
    } else {
        (var_v5,)
    }
};
        var_v5 = assign28020_e33372;

        let (assign28030_e33378,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign28030_e33378;

        let (assign28040_e33384,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign28040_e33384;

        let assign28050_e33396: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard538 = assign28050_e33396;

        let assign28130_e33482: f64 = if var_v1 < var_vmax_d { 1.0 } else { 0.0 };
        var_guard539 = assign28130_e33482;

        let assign28140_e33484: f64 = (-0.5);
        let assign28140_e33487: f64 = (var_v1 * var_phitdinv);
        let assign28140_e33488: f64 = (assign28140_e33484 * assign28140_e33487);
        let assign28140_e33489: f64 = (assign28140_e33488).abs();
        let assign28140_e33491: f64 = if assign28140_e33489 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard540 = assign28140_e33491;

        let (assign28150_e33509,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard538 != 0.0)) && (var_guard539 != 0.0)) && (var_guard540 != 0.0)) {
        let assign28150_e33502: f64 = (-0.5);
        let assign28150_e33505: f64 = (var_v1 * var_phitdinv);
        let assign28150_e33506: f64 = (assign28150_e33502 * assign28150_e33505);
        let assign28150_e33507: f64 = (assign28150_e33506).exp();
        (assign28150_e33507,)
    } else {
        (var_z,)
    }
};
        var_z = assign28150_e33509;

        let assign28160_e33511: f64 = (-0.5);
        let assign28160_e33514: f64 = (var_v1 * var_phitdinv);
        let assign28160_e33515: f64 = (assign28160_e33511 * assign28160_e33514);
        let assign28160_e33517: f64 = if assign28160_e33515 < 0.0 { 1.0 } else { 0.0 };
        var_guard541 = assign28160_e33517;

        let (assign28170_e33572,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard538 != 0.0)) && (var_guard539 != 0.0)) && (var_guard540 == 0.0)) && (var_guard541 != 0.0)) {
        let assign28170_e33533: f64 = (-230.25850929940458);
        let assign28170_e33535: f64 = (-0.5);
        let assign28170_e33538: f64 = (var_v1 * var_phitdinv);
        let assign28170_e33539: f64 = (assign28170_e33535 * assign28170_e33538);
        let assign28170_e33540: f64 = (assign28170_e33533 - assign28170_e33539);
        let assign28170_e33544: f64 = (-230.25850929940458);
        let assign28170_e33546: f64 = (-0.5);
        let assign28170_e33549: f64 = (var_v1 * var_phitdinv);
        let assign28170_e33550: f64 = (assign28170_e33546 * assign28170_e33549);
        let assign28170_e33551: f64 = (assign28170_e33544 - assign28170_e33550);
        let assign28170_e33554: f64 = (-230.25850929940458);
        let assign28170_e33556: f64 = (-0.5);
        let assign28170_e33559: f64 = (var_v1 * var_phitdinv);
        let assign28170_e33560: f64 = (assign28170_e33556 * assign28170_e33559);
        let assign28170_e33561: f64 = (assign28170_e33554 - assign28170_e33560);
        let assign28170_e33563: f64 = (assign28170_e33561 * 0.3333333333333333);
        let assign28170_e33564: f64 = (1.0 + assign28170_e33563);
        let assign28170_e33565: f64 = (assign28170_e33551 * assign28170_e33564);
        let assign28170_e33566: f64 = (0.5 * assign28170_e33565);
        let assign28170_e33567: f64 = (1.0 + assign28170_e33566);
        let assign28170_e33568: f64 = (assign28170_e33540 * assign28170_e33567);
        let assign28170_e33569: f64 = (1.0 + assign28170_e33568);
        let assign28170_e33570: f64 = (1e-100 / assign28170_e33569);
        (assign28170_e33570,)
    } else {
        (var_z,)
    }
};
        var_z = assign28170_e33572;

        let (assign28180_e33625,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard538 != 0.0)) && (var_guard539 != 0.0)) && (var_guard540 == 0.0)) && (var_guard541 == 0.0)) {
        let assign28180_e33589: f64 = (-0.5);
        let assign28180_e33592: f64 = (var_v1 * var_phitdinv);
        let assign28180_e33593: f64 = (assign28180_e33589 * assign28180_e33592);
        let assign28180_e33595: f64 = (assign28180_e33593 - 230.25850929940458);
        let assign28180_e33599: f64 = (-0.5);
        let assign28180_e33602: f64 = (var_v1 * var_phitdinv);
        let assign28180_e33603: f64 = (assign28180_e33599 * assign28180_e33602);
        let assign28180_e33605: f64 = (assign28180_e33603 - 230.25850929940458);
        let assign28180_e33608: f64 = (-0.5);
        let assign28180_e33611: f64 = (var_v1 * var_phitdinv);
        let assign28180_e33612: f64 = (assign28180_e33608 * assign28180_e33611);
        let assign28180_e33614: f64 = (assign28180_e33612 - 230.25850929940458);
        let assign28180_e33616: f64 = (assign28180_e33614 * 0.3333333333333333);
        let assign28180_e33617: f64 = (1.0 + assign28180_e33616);
        let assign28180_e33618: f64 = (assign28180_e33605 * assign28180_e33617);
        let assign28180_e33619: f64 = (0.5 * assign28180_e33618);
        let assign28180_e33620: f64 = (1.0 + assign28180_e33619);
        let assign28180_e33621: f64 = (assign28180_e33595 * assign28180_e33620);
        let assign28180_e33622: f64 = (1.0 + assign28180_e33621);
        let assign28180_e33623: f64 = (1e100 * assign28180_e33622);
        (assign28180_e33623,)
    } else {
        (var_z,)
    }
};
        var_z = assign28180_e33625;

        let (assign28190_e33637,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard538 != 0.0)) && (var_guard539 != 0.0)) {
        let assign28190_e33635: f64 = (1.0 / var_z);
        (assign28190_e33635,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign28190_e33637;

        let (assign28200_e33649,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard538 != 0.0)) && (var_guard539 != 0.0)) {
        let assign28200_e33647: f64 = (var_zinv * var_zinv);
        (assign28200_e33647,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign28200_e33649;

        let (assign28210_e33668,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard538 != 0.0)) && (var_guard539 == 0.0)) {
        let assign28210_e33661: f64 = (var_v1 - var_vmax_d);
        let assign28210_e33663: f64 = (assign28210_e33661 * var_phitdinv);
        let assign28210_e33664: f64 = (1.0 + assign28210_e33663);
        let assign28210_e33666: f64 = (assign28210_e33664 * var_exp_vmax_over_phitd_d);
        (assign28210_e33666,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign28210_e33668;

        let (assign28220_e33680,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard538 != 0.0)) && (var_guard539 == 0.0)) {
        let assign28220_e33678: f64 = (var_idmult).sqrt();
        (assign28220_e33678,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign28220_e33680;

        let (assign28230_e33693,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard538 != 0.0)) && (var_guard539 == 0.0)) {
        let assign28230_e33691: f64 = (1.0 / var_zinv);
        (assign28230_e33691,)
    } else {
        (var_z,)
    }
};
        var_z = assign28230_e33693;

        let (assign28240_e33703,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard538 != 0.0)) {
        let assign28240_e33701: f64 = (var_idmult - 1.0);
        (assign28240_e33701,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign28240_e33703;

        let assign28250_e33706: f64 = if var_v1 > 0.0 { 1.0 } else { 0.0 };
        var_guard542 = assign28250_e33706;

        let (assign28260_e33732,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard538 != 0.0)) && (var_guard542 != 0.0)) {
        let assign28260_e33718: f64 = (2.0 + var_z);
        let assign28260_e33721: f64 = (var_z + 1.0);
        let assign28260_e33724: f64 = (var_z + 3.0);
        let assign28260_e33725: f64 = (assign28260_e33721 * assign28260_e33724);
        let assign28260_e33726: f64 = (assign28260_e33725).sqrt();
        let assign28260_e33727: f64 = (assign28260_e33718 + assign28260_e33726);
        let assign28260_e33728: f64 = (assign28260_e33727).ln();
        let assign28260_e33729: f64 = (var_phitd * assign28260_e33728);
        let assign28260_e33730: f64 = (2.0 * assign28260_e33729);
        (assign28260_e33730,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign28260_e33732;

        let (assign28270_e33766,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard538 != 0.0)) && (var_guard542 == 0.0)) {
        let assign28270_e33742: f64 = (-var_v1);
        let assign28270_e33747: f64 = (2.0 * var_zinv);
        let assign28270_e33749: f64 = (assign28270_e33747 + 1.0);
        let assign28270_e33752: f64 = (1.0 + var_zinv);
        let assign28270_e33756: f64 = (3.0 * var_zinv);
        let assign28270_e33757: f64 = (1.0 + assign28270_e33756);
        let assign28270_e33758: f64 = (assign28270_e33752 * assign28270_e33757);
        let assign28270_e33759: f64 = (assign28270_e33758).sqrt();
        let assign28270_e33760: f64 = (assign28270_e33749 + assign28270_e33759);
        let assign28270_e33761: f64 = (assign28270_e33760).ln();
        let assign28270_e33762: f64 = (var_phitd * assign28270_e33761);
        let assign28270_e33763: f64 = (2.0 * assign28270_e33762);
        let assign28270_e33764: f64 = (assign28270_e33742 + assign28270_e33763);
        (assign28270_e33764,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign28270_e33766;

        let (assign28280_e33776,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard538 != 0.0)) {
        let assign28280_e33774: f64 = (var_vbimin_d - var_two_psistar);
        (assign28280_e33774,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign28280_e33776;

        *var_expxhf1_s_slot = var_expxhf1_s;
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
        *var_guard534_slot = var_guard534;
        *var_guard535_slot = var_guard535;
        *var_guard536_slot = var_guard536;
        *var_guard537_slot = var_guard537;
        *var_guard538_slot = var_guard538;
        *var_guard539_slot = var_guard539;
        *var_guard540_slot = var_guard540;
        *var_guard541_slot = var_guard541;
        *var_guard542_slot = var_guard542;
        *var_idmult_slot = var_idmult;
        *var_isatrev_s_slot = var_isatrev_s;
        *var_isatrev_s_dn5_slot = var_isatrev_s_dn5;
        *var_isatrev_s_dn6_slot = var_isatrev_s_dn6;
        *var_isatrev_s_dn7_slot = var_isatrev_s_dn7;
        *var_isatrev_s_dn8_slot = var_isatrev_s_dn8;
        *var_two_psistar_slot = var_two_psistar;
        *var_v1_slot = var_v1;
        *var_v2_slot = var_v2;
        *var_v3_slot = var_v3;
        *var_v4_slot = var_v4;
        *var_v5_slot = var_v5;
        *var_vbbt_slot = var_vbbt;
        *var_vjlim_slot = var_vjlim;
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
        *var_z_slot = var_z;
        *var_zflagbot_s_slot = var_zflagbot_s;
        *var_zflaggat_s_slot = var_zflaggat_s;
        *var_zflagsti_s_slot = var_zflagsti_s;
        *var_zfrac_slot = var_zfrac;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_55(
        var_abdrain_i: f64,
        var_atatbot_d: f64,
        var_btatpartbot_d: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_ftdbot_d: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard538: f64,
        var_idmult: f64,
        var_idsatbot_d: f64,
        var_one_minus_pbot_d: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_perfc: f64,
        var_phitd: f64,
        var_phitr: f64,
        var_two_psistar: f64,
        var_v1: f64,
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
        var_guard543_slot: &mut f64,
        var_guard544_slot: &mut f64,
        var_guard545_slot: &mut f64,
        var_guard546_slot: &mut f64,
        var_guard547_slot: &mut f64,
        var_guard548_slot: &mut f64,
        var_guard549_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
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
        let mut var_guard543: f64 = *var_guard543_slot;
        let mut var_guard544: f64 = *var_guard544_slot;
        let mut var_guard545: f64 = *var_guard545_slot;
        let mut var_guard546: f64 = *var_guard546_slot;
        let mut var_guard547: f64 = *var_guard547_slot;
        let mut var_guard548: f64 = *var_guard548_slot;
        let mut var_guard549: f64 = *var_guard549_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
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

        let (assign28290_e33803,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard538 != 0.0)) {
        let assign28290_e33785: f64 = (var_v1 + var_vjlim);
        let assign28290_e33788: f64 = (var_v1 - var_vjlim);
        let assign28290_e33791: f64 = (var_v1 - var_vjlim);
        let assign28290_e33792: f64 = (assign28290_e33788 * assign28290_e33791);
        let assign28290_e33795: f64 = (4.0 * var_phitd);
        let assign28290_e33797: f64 = (assign28290_e33795 * var_phitd);
        let assign28290_e33798: f64 = (assign28290_e33792 + assign28290_e33797);
        let assign28290_e33799: f64 = (assign28290_e33798).sqrt();
        let assign28290_e33800: f64 = (assign28290_e33785 - assign28290_e33799);
        let assign28290_e33801: f64 = (0.5 * assign28290_e33800);
        (assign28290_e33801,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign28290_e33803;

        let (assign28300_e33830,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard538 != 0.0)) {
        let assign28300_e33812: f64 = (var_v1 + var_vbbtlim_d);
        let assign28300_e33815: f64 = (var_v1 - var_vbbtlim_d);
        let assign28300_e33818: f64 = (var_v1 - var_vbbtlim_d);
        let assign28300_e33819: f64 = (assign28300_e33815 * assign28300_e33818);
        let assign28300_e33822: f64 = (4.0 * var_phitr);
        let assign28300_e33824: f64 = (assign28300_e33822 * var_phitr);
        let assign28300_e33825: f64 = (assign28300_e33819 + assign28300_e33824);
        let assign28300_e33826: f64 = (assign28300_e33825).sqrt();
        let assign28300_e33827: f64 = (assign28300_e33812 - assign28300_e33826);
        let assign28300_e33828: f64 = (0.5 * assign28300_e33827);
        (assign28300_e33828,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign28300_e33830;

        let (assign28310_e33857,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard538 != 0.0)) {
        let assign28310_e33839: f64 = var_v1;
        let assign28310_e33842: f64 = var_v1;
        let assign28310_e33845: f64 = var_v1;
        let assign28310_e33846: f64 = (assign28310_e33842 * assign28310_e33845);
        let assign28310_e33849: f64 = (4.0 * 1e-6);
        let assign28310_e33851: f64 = (assign28310_e33849 * 1e-6);
        let assign28310_e33852: f64 = (assign28310_e33846 + assign28310_e33851);
        let assign28310_e33853: f64 = (assign28310_e33852).sqrt();
        let assign28310_e33854: f64 = (assign28310_e33839 - assign28310_e33853);
        let assign28310_e33855: f64 = (0.5 * assign28310_e33854);
        (assign28310_e33855,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign28310_e33857;

        let assign28320_e33860: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard543 = assign28320_e33860;

        let (assign28330_e33868, assign28330_e33868_d_n5, assign28330_e33868_d_n6, assign28330_e33868_d_n7, assign28330_e33868_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign28330_e33868;
        var_ijunbot_dn5 = assign28330_e33868_d_n5;
        var_ijunbot_dn6 = assign28330_e33868_d_n6;
        var_ijunbot_dn7 = assign28330_e33868_d_n7;
        var_ijunbot_dn8 = assign28330_e33868_d_n8;

        let (assign28340_e33879,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) {
        let assign28340_e33877: f64 = (var_idsatbot_d * var_idmult);
        (assign28340_e33877,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign28340_e33879;

        let assign28350_e33886: f64 = if ((var_csrhbotd_i == 0.0) && (var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard544 = assign28350_e33886;

        let (assign28360_e33897, assign28360_e33897_d_n5, assign28360_e33897_d_n6, assign28360_e33897_d_n7, assign28360_e33897_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard544 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign28360_e33897;
        var_isrh_dn5 = assign28360_e33897_d_n5;
        var_isrh_dn6 = assign28360_e33897_d_n6;
        var_isrh_dn7 = assign28360_e33897_d_n7;
        var_isrh_dn8 = assign28360_e33897_d_n8;

        let (assign28370_e33911,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard544 == 0.0)) {
        let assign28370_e33909: f64 = (var_vbibot_d - var_vjsrh);
        (assign28370_e33909,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign28370_e33911;

        let (assign28380_e33930,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard544 == 0.0)) {
        let assign28380_e33925: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign28380_e33926: f64 = (1.0 - assign28380_e33925);
        let assign28380_e33927: f64 = (assign28380_e33926).sqrt();
        let assign28380_e33928: f64 = (1.0 - assign28380_e33927);
        (assign28380_e33928,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign28380_e33930;

        let assign28390_e33933: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard545 = assign28390_e33933;

        let (assign28400_e33947,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard544 == 0.0)) && (var_guard545 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign28400_e33947;

        let (assign28410_e33979,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard544 == 0.0)) && (var_guard545 == 0.0)) {
        let assign28410_e33962: f64 = (var_wsrhstep * var_wsrhstep);
        let assign28410_e33964: f64 = (var_wsrhstep).ln();
        let assign28410_e33965: f64 = (assign28410_e33962 * assign28410_e33964);
        let assign28410_e33968: f64 = (1.0 - var_wsrhstep);
        let assign28410_e33969: f64 = (assign28410_e33965 / assign28410_e33968);
        let assign28410_e33971: f64 = (assign28410_e33969 + var_wsrhstep);
        let assign28410_e33975: f64 = (2.0 * var_pbotd_i);
        let assign28410_e33976: f64 = (1.0 - assign28410_e33975);
        let assign28410_e33977: f64 = (assign28410_e33971 * assign28410_e33976);
        (assign28410_e33977,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign28410_e33979;

        let (assign28420_e33993,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard544 == 0.0)) {
        let assign28420_e33991: f64 = (var_wsrhstep + var_dwsrh);
        (assign28420_e33991,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign28420_e33993;

        let assign28430_e33996: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard546 = assign28430_e33996;

        let (assign28440_e34013, assign28440_e34013_d_n5, assign28440_e34013_d_n6, assign28440_e34013_d_n7, assign28440_e34013_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard544 == 0.0)) && (var_guard546 != 0.0)) {
        let assign28440_e34010: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign28440_e34011: f64 = (assign28440_e34010).sqrt();
        (assign28440_e34011, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28440_e34013;
        var_tmp_dn5 = assign28440_e34013_d_n5;
        var_tmp_dn6 = assign28440_e34013_d_n6;
        var_tmp_dn7 = assign28440_e34013_d_n7;
        var_tmp_dn8 = assign28440_e34013_d_n8;

        let (assign28450_e34032, assign28450_e34032_d_n5, assign28450_e34032_d_n6, assign28450_e34032_d_n7, assign28450_e34032_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard544 == 0.0)) && (var_guard546 == 0.0)) {
        let assign28450_e34028: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign28450_e34030: f64 = (assign28450_e34028).powf(var_pbotd_i);
        (assign28450_e34030, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28450_e34032;
        var_tmp_dn5 = assign28450_e34032_d_n5;
        var_tmp_dn6 = assign28450_e34032_d_n6;
        var_tmp_dn7 = assign28450_e34032_d_n7;
        var_tmp_dn8 = assign28450_e34032_d_n8;

        let (assign28460_e34046, assign28460_e34046_d_n5, assign28460_e34046_d_n6, assign28460_e34046_d_n7, assign28460_e34046_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard544 == 0.0)) {
        let assign28460_e34044: f64 = (var_wdepnulrbot_d * var_tmp);
        (assign28460_e34044, (var_wdepnulrbot_d * var_tmp_dn5), (var_wdepnulrbot_d * var_tmp_dn6), (var_wdepnulrbot_d * var_tmp_dn7), (var_wdepnulrbot_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign28460_e34046;
        var_wdep_dn5 = assign28460_e34046_d_n5;
        var_wdep_dn6 = assign28460_e34046_d_n6;
        var_wdep_dn7 = assign28460_e34046_d_n7;
        var_wdep_dn8 = assign28460_e34046_d_n8;

        let (assign28470_e34064, assign28470_e34064_d_n5, assign28470_e34064_d_n6, assign28470_e34064_d_n7, assign28470_e34064_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard544 == 0.0)) {
        let assign28470_e34059: f64 = (var_zinv - 1.0);
        let assign28470_e34061: f64 = (assign28470_e34059 * var_wdep);
        let assign28470_e34062: f64 = (var_ftdbot_d * assign28470_e34061);
        (assign28470_e34062, (var_ftdbot_d * (assign28470_e34059 * var_wdep_dn5)), (var_ftdbot_d * (assign28470_e34059 * var_wdep_dn6)), (var_ftdbot_d * (assign28470_e34059 * var_wdep_dn7)), (var_ftdbot_d * (assign28470_e34059 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign28470_e34064;
        var_asrh_dn5 = assign28470_e34064_d_n5;
        var_asrh_dn6 = assign28470_e34064_d_n6;
        var_asrh_dn7 = assign28470_e34064_d_n7;
        var_asrh_dn8 = assign28470_e34064_d_n8;

        let (assign28480_e34080, assign28480_e34080_d_n5, assign28480_e34080_d_n6, assign28480_e34080_d_n7, assign28480_e34080_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard544 == 0.0)) {
        let assign28480_e34077: f64 = (var_asrh * var_wsrh);
        let assign28480_e34078: f64 = (var_csrhbotd_i * assign28480_e34077);
        (assign28480_e34078, (var_csrhbotd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign28480_e34080;
        var_isrh_dn5 = assign28480_e34080_d_n5;
        var_isrh_dn6 = assign28480_e34080_d_n6;
        var_isrh_dn7 = assign28480_e34080_d_n7;
        var_isrh_dn8 = assign28480_e34080_d_n8;

        let assign28490_e34083: f64 = if var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard547 = assign28490_e34083;

        let (assign28500_e34094, assign28500_e34094_d_n5, assign28500_e34094_d_n6, assign28500_e34094_d_n7, assign28500_e34094_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign28500_e34094;
        var_itat_dn5 = assign28500_e34094_d_n5;
        var_itat_dn6 = assign28500_e34094_d_n6;
        var_itat_dn7 = assign28500_e34094_d_n7;
        var_itat_dn8 = assign28500_e34094_d_n8;

        let (assign28510_e34112, assign28510_e34112_d_n5, assign28510_e34112_d_n6, assign28510_e34112_d_n7, assign28510_e34112_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) {
        let assign28510_e34107: f64 = (var_wdep * var_one_minus_pbot_d);
        let assign28510_e34109: f64 = (assign28510_e34107 / var_vbi_minus_vjsrh);
        let assign28510_e34110: f64 = (var_btatpartbot_d * assign28510_e34109);
        (assign28510_e34110, (var_btatpartbot_d * ((var_wdep_dn5 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn6 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn7 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn8 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign28510_e34112;
        var_btat_dn5 = assign28510_e34112_d_n5;
        var_btat_dn6 = assign28510_e34112_d_n6;
        var_btat_dn7 = assign28510_e34112_d_n7;
        var_btat_dn8 = assign28510_e34112_d_n8;

        let (assign28520_e34128, assign28520_e34128_d_n5, assign28520_e34128_d_n6, assign28520_e34128_d_n7, assign28520_e34128_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) {
        let assign28520_e34124: f64 = (0.666666666666667 * var_atatbot_d);
        let assign28520_e34126: f64 = (assign28520_e34124 / var_btat);
        (assign28520_e34126, (-((assign28520_e34124 * var_btat_dn5) / (var_btat * var_btat))), (-((assign28520_e34124 * var_btat_dn6) / (var_btat * var_btat))), (-((assign28520_e34124 * var_btat_dn7) / (var_btat * var_btat))), (-((assign28520_e34124 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign28520_e34128;
        var_twoatatoverthreebtat_dn5 = assign28520_e34128_d_n5;
        var_twoatatoverthreebtat_dn6 = assign28520_e34128_d_n6;
        var_twoatatoverthreebtat_dn7 = assign28520_e34128_d_n7;
        var_twoatatoverthreebtat_dn8 = assign28520_e34128_d_n8;

        let (assign28530_e34142, assign28530_e34142_d_n5, assign28530_e34142_d_n6, assign28530_e34142_d_n7, assign28530_e34142_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) {
        let assign28530_e34140: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign28530_e34140, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign28530_e34142;
        var_umaxbeforelimiting_dn5 = assign28530_e34142_d_n5;
        var_umaxbeforelimiting_dn6 = assign28530_e34142_d_n6;
        var_umaxbeforelimiting_dn7 = assign28530_e34142_d_n7;
        var_umaxbeforelimiting_dn8 = assign28530_e34142_d_n8;

        let (assign28540_e34163, assign28540_e34163_d_n5, assign28540_e34163_d_n6, assign28540_e34163_d_n7, assign28540_e34163_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) {
        let assign28540_e34154: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign28540_e34157: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign28540_e34159: f64 = (assign28540_e34157 + 1.0);
        let assign28540_e34160: f64 = (assign28540_e34154 / assign28540_e34159);
        let assign28540_e34161: f64 = (assign28540_e34160).sqrt();
        (assign28540_e34161, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign28540_e34159) - (assign28540_e34154 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign28540_e34159 * assign28540_e34159)) / (2.0 * assign28540_e34161)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign28540_e34159) - (assign28540_e34154 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign28540_e34159 * assign28540_e34159)) / (2.0 * assign28540_e34161)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign28540_e34159) - (assign28540_e34154 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign28540_e34159 * assign28540_e34159)) / (2.0 * assign28540_e34161)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign28540_e34159) - (assign28540_e34154 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign28540_e34159 * assign28540_e34159)) / (2.0 * assign28540_e34161)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign28540_e34163;
        var_umax_dn5 = assign28540_e34163_d_n5;
        var_umax_dn6 = assign28540_e34163_d_n6;
        var_umax_dn7 = assign28540_e34163_d_n7;
        var_umax_dn8 = assign28540_e34163_d_n8;

        let (assign28550_e34176, assign28550_e34176_d_n5, assign28550_e34176_d_n6, assign28550_e34176_d_n7, assign28550_e34176_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) {
        let assign28550_e34174: f64 = (var_umax).sqrt();
        (assign28550_e34174, (var_umax_dn5 / (2.0 * assign28550_e34174)), (var_umax_dn6 / (2.0 * assign28550_e34174)), (var_umax_dn7 / (2.0 * assign28550_e34174)), (var_umax_dn8 / (2.0 * assign28550_e34174)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign28550_e34176;
        var_sqrtumax_dn5 = assign28550_e34176_d_n5;
        var_sqrtumax_dn6 = assign28550_e34176_d_n6;
        var_sqrtumax_dn7 = assign28550_e34176_d_n7;
        var_sqrtumax_dn8 = assign28550_e34176_d_n8;

        let (assign28560_e34190, assign28560_e34190_d_n5, assign28560_e34190_d_n6, assign28560_e34190_d_n7, assign28560_e34190_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) {
        let assign28560_e34188: f64 = (var_umax * var_sqrtumax);
        (assign28560_e34188, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign28560_e34190;
        var_umaxpoweronepointfive_dn5 = assign28560_e34190_d_n5;
        var_umaxpoweronepointfive_dn6 = assign28560_e34190_d_n6;
        var_umaxpoweronepointfive_dn7 = assign28560_e34190_d_n7;
        var_umaxpoweronepointfive_dn8 = assign28560_e34190_d_n8;

        let assign28570_e34192: f64 = (-var_pbotd_i);
        let assign28570_e34194: f64 = (assign28570_e34192 * var_one_over_one_minus_pbot_d);
        let assign28570_e34196: f64 = (-1.0);
        let assign28570_e34197: f64 = if assign28570_e34194 == assign28570_e34196 { 1.0 } else { 0.0 };
        var_guard548 = assign28570_e34197;

        let (assign28580_e34217, assign28580_e34217_d_n5, assign28580_e34217_d_n6, assign28580_e34217_d_n7, assign28580_e34217_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) && (var_guard548 != 0.0)) {
        let assign28580_e34213: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign28580_e34214: f64 = (1.0 + assign28580_e34213);
        let assign28580_e34215: f64 = (1.0 / assign28580_e34214);
        (assign28580_e34215, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign28580_e34214 * assign28580_e34214))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign28580_e34214 * assign28580_e34214))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign28580_e34214 * assign28580_e34214))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign28580_e34214 * assign28580_e34214))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign28580_e34217;
        var_wgamma_dn5 = assign28580_e34217_d_n5;
        var_wgamma_dn6 = assign28580_e34217_d_n6;
        var_wgamma_dn7 = assign28580_e34217_d_n7;
        var_wgamma_dn8 = assign28580_e34217_d_n8;

        let (assign28590_e34241, assign28590_e34241_d_n5, assign28590_e34241_d_n6, assign28590_e34241_d_n7, assign28590_e34241_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) && (var_guard548 == 0.0)) {
        let assign28590_e34233: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign28590_e34234: f64 = (1.0 + assign28590_e34233);
        let assign28590_e34236: f64 = (-var_pbotd_i);
        let assign28590_e34238: f64 = (assign28590_e34236 * var_one_over_one_minus_pbot_d);
        let assign28590_e34239: f64 = (assign28590_e34234).powf(assign28590_e34238);
        (assign28590_e34239, if 0.0 == 0.0 && ((assign28590_e34238) as f64).is_finite() && ((assign28590_e34238) as f64).fract() == 0.0 { if assign28590_e34238 == 0.0 { 0.0 } else { (assign28590_e34238 * ((assign28590_e34234).powf(assign28590_e34238 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign28590_e34239 * (assign28590_e34238 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign28590_e34234))) }, if 0.0 == 0.0 && ((assign28590_e34238) as f64).is_finite() && ((assign28590_e34238) as f64).fract() == 0.0 { if assign28590_e34238 == 0.0 { 0.0 } else { (assign28590_e34238 * ((assign28590_e34234).powf(assign28590_e34238 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign28590_e34239 * (assign28590_e34238 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign28590_e34234))) }, if 0.0 == 0.0 && ((assign28590_e34238) as f64).is_finite() && ((assign28590_e34238) as f64).fract() == 0.0 { if assign28590_e34238 == 0.0 { 0.0 } else { (assign28590_e34238 * ((assign28590_e34234).powf(assign28590_e34238 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign28590_e34239 * (assign28590_e34238 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign28590_e34234))) }, if 0.0 == 0.0 && ((assign28590_e34238) as f64).is_finite() && ((assign28590_e34238) as f64).fract() == 0.0 { if assign28590_e34238 == 0.0 { 0.0 } else { (assign28590_e34238 * ((assign28590_e34234).powf(assign28590_e34238 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign28590_e34239 * (assign28590_e34238 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign28590_e34234))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign28590_e34241;
        var_wgamma_dn5 = assign28590_e34241_d_n5;
        var_wgamma_dn6 = assign28590_e34241_d_n6;
        var_wgamma_dn7 = assign28590_e34241_d_n7;
        var_wgamma_dn8 = assign28590_e34241_d_n8;

        let (assign28600_e34259, assign28600_e34259_d_n5, assign28600_e34259_d_n6, assign28600_e34259_d_n7, assign28600_e34259_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) {
        let assign28600_e34253: f64 = (var_wsrh * var_wgamma);
        let assign28600_e34256: f64 = (var_wsrh + var_wgamma);
        let assign28600_e34257: f64 = (assign28600_e34253 / assign28600_e34256);
        (assign28600_e34257, ((((var_wsrh * var_wgamma_dn5) * assign28600_e34256) - (assign28600_e34253 * var_wgamma_dn5)) / (assign28600_e34256 * assign28600_e34256)), ((((var_wsrh * var_wgamma_dn6) * assign28600_e34256) - (assign28600_e34253 * var_wgamma_dn6)) / (assign28600_e34256 * assign28600_e34256)), ((((var_wsrh * var_wgamma_dn7) * assign28600_e34256) - (assign28600_e34253 * var_wgamma_dn7)) / (assign28600_e34256 * assign28600_e34256)), ((((var_wsrh * var_wgamma_dn8) * assign28600_e34256) - (assign28600_e34253 * var_wgamma_dn8)) / (assign28600_e34256 * assign28600_e34256)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign28600_e34259;
        var_wtat_dn5 = assign28600_e34259_d_n5;
        var_wtat_dn6 = assign28600_e34259_d_n6;
        var_wtat_dn7 = assign28600_e34259_d_n7;
        var_wtat_dn8 = assign28600_e34259_d_n8;

        let (assign28610_e34276, assign28610_e34276_d_n5, assign28610_e34276_d_n6, assign28610_e34276_d_n7, assign28610_e34276_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) {
        let assign28610_e34272: f64 = (var_btat / var_sqrtumax);
        let assign28610_e34273: f64 = (0.375 * assign28610_e34272);
        let assign28610_e34274: f64 = (assign28610_e34273).sqrt();
        (assign28610_e34274, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign28610_e34274)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign28610_e34274)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign28610_e34274)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign28610_e34274)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign28610_e34276;
        var_ktat_dn5 = assign28610_e34276_d_n5;
        var_ktat_dn6 = assign28610_e34276_d_n6;
        var_ktat_dn7 = assign28610_e34276_d_n7;
        var_ktat_dn8 = assign28610_e34276_d_n8;

        let (assign28620_e34294, assign28620_e34294_d_n5, assign28620_e34294_d_n6, assign28620_e34294_d_n7, assign28620_e34294_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) {
        let assign28620_e34289: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign28620_e34290: f64 = (2.0 * assign28620_e34289);
        let assign28620_e34292: f64 = (assign28620_e34290 - var_umax);
        (assign28620_e34292, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign28620_e34294;
        var_ltat_dn5 = assign28620_e34294_d_n5;
        var_ltat_dn6 = assign28620_e34294_d_n6;
        var_ltat_dn7 = assign28620_e34294_d_n7;
        var_ltat_dn8 = assign28620_e34294_d_n8;

        let (assign28630_e34320, assign28630_e34320_d_n5, assign28630_e34320_d_n6, assign28630_e34320_d_n7, assign28630_e34320_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) {
        let assign28630_e34306: f64 = (var_atatbot_d * var_twoatatoverthreebtat);
        let assign28630_e34308: f64 = (assign28630_e34306 * var_sqrtumax);
        let assign28630_e34311: f64 = (var_atatbot_d * var_umax);
        let assign28630_e34312: f64 = (assign28630_e34308 - assign28630_e34311);
        let assign28630_e34316: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign28630_e34317: f64 = (0.5 * assign28630_e34316);
        let assign28630_e34318: f64 = (assign28630_e34312 + assign28630_e34317);
        (assign28630_e34318, (((((var_atatbot_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign28630_e34306 * var_sqrtumax_dn5)) - (var_atatbot_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign28630_e34306 * var_sqrtumax_dn6)) - (var_atatbot_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign28630_e34306 * var_sqrtumax_dn7)) - (var_atatbot_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign28630_e34306 * var_sqrtumax_dn8)) - (var_atatbot_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign28630_e34320;
        var_mtat_dn5 = assign28630_e34320_d_n5;
        var_mtat_dn6 = assign28630_e34320_d_n6;
        var_mtat_dn7 = assign28630_e34320_d_n7;
        var_mtat_dn8 = assign28630_e34320_d_n8;

        let (assign28640_e34336, assign28640_e34336_d_n5, assign28640_e34336_d_n6, assign28640_e34336_d_n7, assign28640_e34336_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) {
        let assign28640_e34332: f64 = (var_ltat - 1.0);
        let assign28640_e34334: f64 = (assign28640_e34332 * var_ktat);
        (assign28640_e34334, ((var_ltat_dn5 * var_ktat) + (assign28640_e34332 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign28640_e34332 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign28640_e34332 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign28640_e34332 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign28640_e34336;
        var_xerfc_dn5 = assign28640_e34336_d_n5;
        var_xerfc_dn6 = assign28640_e34336_d_n6;
        var_xerfc_dn7 = assign28640_e34336_d_n7;
        var_xerfc_dn8 = assign28640_e34336_d_n8;

        let (assign28650_e34350, assign28650_e34350_d_n5, assign28650_e34350_d_n6, assign28650_e34350_d_n7, assign28650_e34350_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) {
        let assign28650_e34348: f64 = (var_xerfc * var_xerfc);
        (assign28650_e34348, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign28650_e34350;
        var_ysq_dn5 = assign28650_e34350_d_n5;
        var_ysq_dn6 = assign28650_e34350_d_n6;
        var_ysq_dn7 = assign28650_e34350_d_n7;
        var_ysq_dn8 = assign28650_e34350_d_n8;

        let assign28660_e34353: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard549 = assign28660_e34353;

        let (assign28670_e34373, assign28670_e34373_d_n5, assign28670_e34373_d_n6, assign28670_e34373_d_n7, assign28670_e34373_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) && (var_guard549 != 0.0)) {
        let assign28670_e34369: f64 = (var_perfc * var_xerfc);
        let assign28670_e34370: f64 = (1.0 + assign28670_e34369);
        let assign28670_e34371: f64 = (1.0 / assign28670_e34370);
        (assign28670_e34371, (-((var_perfc * var_xerfc_dn5) / (assign28670_e34370 * assign28670_e34370))), (-((var_perfc * var_xerfc_dn6) / (assign28670_e34370 * assign28670_e34370))), (-((var_perfc * var_xerfc_dn7) / (assign28670_e34370 * assign28670_e34370))), (-((var_perfc * var_xerfc_dn8) / (assign28670_e34370 * assign28670_e34370))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign28670_e34373;
        var_terfc_dn5 = assign28670_e34373_d_n5;
        var_terfc_dn6 = assign28670_e34373_d_n6;
        var_terfc_dn7 = assign28670_e34373_d_n7;
        var_terfc_dn8 = assign28670_e34373_d_n8;

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
        *var_guard543_slot = var_guard543;
        *var_guard544_slot = var_guard544;
        *var_guard545_slot = var_guard545;
        *var_guard546_slot = var_guard546;
        *var_guard547_slot = var_guard547;
        *var_guard548_slot = var_guard548;
        *var_guard549_slot = var_guard549;
        *var_id__blk219_slot = var_id__blk219;
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

    pub(super) fn stamp_transient_block_56(
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
        var_guard182: f64,
        var_guard199: f64,
        var_guard543: f64,
        var_guard547: f64,
        var_guard549: f64,
        var_id__blk219: f64,
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
        var_v1: f64,
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
        var_guard550_slot: &mut f64,
        var_guard551_slot: &mut f64,
        var_guard552_slot: &mut f64,
        var_guard553_slot: &mut f64,
        var_guard554_slot: &mut f64,
        var_guard555_slot: &mut f64,
        var_guard556_slot: &mut f64,
        var_guard557_slot: &mut f64,
        var_guard558_slot: &mut f64,
        var_guard559_slot: &mut f64,
        var_guard560_slot: &mut f64,
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
        let mut var_guard550: f64 = *var_guard550_slot;
        let mut var_guard551: f64 = *var_guard551_slot;
        let mut var_guard552: f64 = *var_guard552_slot;
        let mut var_guard553: f64 = *var_guard553_slot;
        let mut var_guard554: f64 = *var_guard554_slot;
        let mut var_guard555: f64 = *var_guard555_slot;
        let mut var_guard556: f64 = *var_guard556_slot;
        let mut var_guard557: f64 = *var_guard557_slot;
        let mut var_guard558: f64 = *var_guard558_slot;
        let mut var_guard559: f64 = *var_guard559_slot;
        let mut var_guard560: f64 = *var_guard560_slot;
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

        let (assign28680_e34394, assign28680_e34394_d_n5, assign28680_e34394_d_n6, assign28680_e34394_d_n7, assign28680_e34394_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) && (var_guard549 == 0.0)) {
        let assign28680_e34390: f64 = (var_perfc * var_xerfc);
        let assign28680_e34391: f64 = (1.0 - assign28680_e34390);
        let assign28680_e34392: f64 = (1.0 / assign28680_e34391);
        (assign28680_e34392, (-((-(var_perfc * var_xerfc_dn5)) / (assign28680_e34391 * assign28680_e34391))), (-((-(var_perfc * var_xerfc_dn6)) / (assign28680_e34391 * assign28680_e34391))), (-((-(var_perfc * var_xerfc_dn7)) / (assign28680_e34391 * assign28680_e34391))), (-((-(var_perfc * var_xerfc_dn8)) / (assign28680_e34391 * assign28680_e34391))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign28680_e34394;
        var_terfc_dn5 = assign28680_e34394_d_n5;
        var_terfc_dn6 = assign28680_e34394_d_n6;
        var_terfc_dn7 = assign28680_e34394_d_n7;
        var_terfc_dn8 = assign28680_e34394_d_n8;

        let assign28690_e34396: f64 = (-var_ysq);
        let assign28690_e34398: f64 = (assign28690_e34396 + var_mtat);
        let assign28690_e34400: f64 = (-230.25850929940458);
        let assign28690_e34401: f64 = if assign28690_e34398 > assign28690_e34400 { 1.0 } else { 0.0 };
        var_guard550 = assign28690_e34401;

        let (assign28700_e34419, assign28700_e34419_d_n5, assign28700_e34419_d_n6, assign28700_e34419_d_n7, assign28700_e34419_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) && (var_guard550 != 0.0)) {
        let assign28700_e34414: f64 = (-var_ysq);
        let assign28700_e34416: f64 = (assign28700_e34414 + var_mtat);
        let assign28700_e34417: f64 = (assign28700_e34416).exp();
        (assign28700_e34417, (assign28700_e34417 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign28700_e34417 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign28700_e34417 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign28700_e34417 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28700_e34419;
        var_tmp_dn5 = assign28700_e34419_d_n5;
        var_tmp_dn6 = assign28700_e34419_d_n6;
        var_tmp_dn7 = assign28700_e34419_d_n7;
        var_tmp_dn8 = assign28700_e34419_d_n8;

        let (assign28710_e34468, assign28710_e34468_d_n5, assign28710_e34468_d_n6, assign28710_e34468_d_n7, assign28710_e34468_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) && (var_guard550 == 0.0)) {
        let assign28710_e34435: f64 = (-230.25850929940458);
        let assign28710_e34437: f64 = (-var_ysq);
        let assign28710_e34439: f64 = (assign28710_e34437 + var_mtat);
        let assign28710_e34440: f64 = (assign28710_e34435 - assign28710_e34439);
        let assign28710_e34444: f64 = (-230.25850929940458);
        let assign28710_e34446: f64 = (-var_ysq);
        let assign28710_e34448: f64 = (assign28710_e34446 + var_mtat);
        let assign28710_e34449: f64 = (assign28710_e34444 - assign28710_e34448);
        let assign28710_e34452: f64 = (-230.25850929940458);
        let assign28710_e34454: f64 = (-var_ysq);
        let assign28710_e34456: f64 = (assign28710_e34454 + var_mtat);
        let assign28710_e34457: f64 = (assign28710_e34452 - assign28710_e34456);
        let assign28710_e34459: f64 = (assign28710_e34457 * 0.3333333333333333);
        let assign28710_e34460: f64 = (1.0 + assign28710_e34459);
        let assign28710_e34461: f64 = (assign28710_e34449 * assign28710_e34460);
        let assign28710_e34462: f64 = (0.5 * assign28710_e34461);
        let assign28710_e34463: f64 = (1.0 + assign28710_e34462);
        let assign28710_e34464: f64 = (assign28710_e34440 * assign28710_e34463);
        let assign28710_e34465: f64 = (1.0 + assign28710_e34464);
        let assign28710_e34466: f64 = (1e-100 / assign28710_e34465);
        (assign28710_e34466, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign28710_e34463) + (assign28710_e34440 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign28710_e34460) + (assign28710_e34449 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign28710_e34465 * assign28710_e34465))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign28710_e34463) + (assign28710_e34440 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign28710_e34460) + (assign28710_e34449 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign28710_e34465 * assign28710_e34465))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign28710_e34463) + (assign28710_e34440 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign28710_e34460) + (assign28710_e34449 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign28710_e34465 * assign28710_e34465))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign28710_e34463) + (assign28710_e34440 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign28710_e34460) + (assign28710_e34449 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign28710_e34465 * assign28710_e34465))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28710_e34468;
        var_tmp_dn5 = assign28710_e34468_d_n5;
        var_tmp_dn6 = assign28710_e34468_d_n6;
        var_tmp_dn7 = assign28710_e34468_d_n7;
        var_tmp_dn8 = assign28710_e34468_d_n8;

        let (assign28720_e34498, assign28720_e34498_d_n5, assign28720_e34498_d_n6, assign28720_e34498_d_n7, assign28720_e34498_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) {
        let assign28720_e34480: f64 = (0.29214664 * var_terfc);
        let assign28720_e34484: f64 = (var_terfc * var_terfc);
        let assign28720_e34485: f64 = (var_berfc * assign28720_e34484);
        let assign28720_e34486: f64 = (assign28720_e34480 + assign28720_e34485);
        let assign28720_e34490: f64 = (var_terfc * var_terfc);
        let assign28720_e34492: f64 = (assign28720_e34490 * var_terfc);
        let assign28720_e34493: f64 = (var_cerfc * assign28720_e34492);
        let assign28720_e34494: f64 = (assign28720_e34486 + assign28720_e34493);
        let assign28720_e34496: f64 = (assign28720_e34494 * var_tmp);
        (assign28720_e34496, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign28720_e34490 * var_terfc_dn5)))) * var_tmp) + (assign28720_e34494 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign28720_e34490 * var_terfc_dn6)))) * var_tmp) + (assign28720_e34494 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign28720_e34490 * var_terfc_dn7)))) * var_tmp) + (assign28720_e34494 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign28720_e34490 * var_terfc_dn8)))) * var_tmp) + (assign28720_e34494 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign28720_e34498;
        var_erfcpos_dn5 = assign28720_e34498_d_n5;
        var_erfcpos_dn6 = assign28720_e34498_d_n6;
        var_erfcpos_dn7 = assign28720_e34498_d_n7;
        var_erfcpos_dn8 = assign28720_e34498_d_n8;

        let assign28730_e34501: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard551 = assign28730_e34501;

        let (assign28740_e34515, assign28740_e34515_d_n5, assign28740_e34515_d_n6, assign28740_e34515_d_n7, assign28740_e34515_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) && (var_guard551 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign28740_e34515;
        var_erfctimesexpmtat_dn5 = assign28740_e34515_d_n5;
        var_erfctimesexpmtat_dn6 = assign28740_e34515_d_n6;
        var_erfctimesexpmtat_dn7 = assign28740_e34515_d_n7;
        var_erfctimesexpmtat_dn8 = assign28740_e34515_d_n8;

        let assign28750_e34518: f64 = (-230.25850929940458);
        let assign28750_e34519: f64 = if var_mtat > assign28750_e34518 { 1.0 } else { 0.0 };
        var_guard552 = assign28750_e34519;

        let (assign28760_e34537, assign28760_e34537_d_n5, assign28760_e34537_d_n6, assign28760_e34537_d_n7, assign28760_e34537_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) && (var_guard551 == 0.0)) && (var_guard552 != 0.0)) {
        let assign28760_e34535: f64 = (var_mtat).exp();
        (assign28760_e34535, (assign28760_e34535 * var_mtat_dn5), (assign28760_e34535 * var_mtat_dn6), (assign28760_e34535 * var_mtat_dn7), (assign28760_e34535 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28760_e34537;
        var_tmp_dn5 = assign28760_e34537_d_n5;
        var_tmp_dn6 = assign28760_e34537_d_n6;
        var_tmp_dn7 = assign28760_e34537_d_n7;
        var_tmp_dn8 = assign28760_e34537_d_n8;

        let (assign28770_e34580, assign28770_e34580_d_n5, assign28770_e34580_d_n6, assign28770_e34580_d_n7, assign28770_e34580_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) && (var_guard551 == 0.0)) && (var_guard552 == 0.0)) {
        let assign28770_e34556: f64 = (-230.25850929940458);
        let assign28770_e34558: f64 = (assign28770_e34556 - var_mtat);
        let assign28770_e34562: f64 = (-230.25850929940458);
        let assign28770_e34564: f64 = (assign28770_e34562 - var_mtat);
        let assign28770_e34567: f64 = (-230.25850929940458);
        let assign28770_e34569: f64 = (assign28770_e34567 - var_mtat);
        let assign28770_e34571: f64 = (assign28770_e34569 * 0.3333333333333333);
        let assign28770_e34572: f64 = (1.0 + assign28770_e34571);
        let assign28770_e34573: f64 = (assign28770_e34564 * assign28770_e34572);
        let assign28770_e34574: f64 = (0.5 * assign28770_e34573);
        let assign28770_e34575: f64 = (1.0 + assign28770_e34574);
        let assign28770_e34576: f64 = (assign28770_e34558 * assign28770_e34575);
        let assign28770_e34577: f64 = (1.0 + assign28770_e34576);
        let assign28770_e34578: f64 = (1e-100 / assign28770_e34577);
        (assign28770_e34578, (-((1e-100 * (((-var_mtat_dn5) * assign28770_e34575) + (assign28770_e34558 * (0.5 * (((-var_mtat_dn5) * assign28770_e34572) + (assign28770_e34564 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign28770_e34577 * assign28770_e34577))), (-((1e-100 * (((-var_mtat_dn6) * assign28770_e34575) + (assign28770_e34558 * (0.5 * (((-var_mtat_dn6) * assign28770_e34572) + (assign28770_e34564 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign28770_e34577 * assign28770_e34577))), (-((1e-100 * (((-var_mtat_dn7) * assign28770_e34575) + (assign28770_e34558 * (0.5 * (((-var_mtat_dn7) * assign28770_e34572) + (assign28770_e34564 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign28770_e34577 * assign28770_e34577))), (-((1e-100 * (((-var_mtat_dn8) * assign28770_e34575) + (assign28770_e34558 * (0.5 * (((-var_mtat_dn8) * assign28770_e34572) + (assign28770_e34564 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign28770_e34577 * assign28770_e34577))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28770_e34580;
        var_tmp_dn5 = assign28770_e34580_d_n5;
        var_tmp_dn6 = assign28770_e34580_d_n6;
        var_tmp_dn7 = assign28770_e34580_d_n7;
        var_tmp_dn8 = assign28770_e34580_d_n8;

        let (assign28780_e34599, assign28780_e34599_d_n5, assign28780_e34599_d_n6, assign28780_e34599_d_n7, assign28780_e34599_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) && (var_guard551 == 0.0)) {
        let assign28780_e34595: f64 = (2.0 * var_tmp);
        let assign28780_e34597: f64 = (assign28780_e34595 - var_erfcpos);
        (assign28780_e34597, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign28780_e34599;
        var_erfctimesexpmtat_dn5 = assign28780_e34599_d_n5;
        var_erfctimesexpmtat_dn6 = assign28780_e34599_d_n6;
        var_erfctimesexpmtat_dn7 = assign28780_e34599_d_n7;
        var_erfctimesexpmtat_dn8 = assign28780_e34599_d_n8;

        let (assign28790_e34619, assign28790_e34619_d_n5, assign28790_e34619_d_n6, assign28790_e34619_d_n7, assign28790_e34619_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) {
        let assign28790_e34611: f64 = (1.772453850905516 * 0.5);
        let assign28790_e34614: f64 = (var_atatbot_d * var_erfctimesexpmtat);
        let assign28790_e34616: f64 = (assign28790_e34614 / var_ktat);
        let assign28790_e34617: f64 = (assign28790_e34611 * assign28790_e34616);
        (assign28790_e34617, (assign28790_e34611 * ((((var_atatbot_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign28790_e34614 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign28790_e34611 * ((((var_atatbot_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign28790_e34614 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign28790_e34611 * ((((var_atatbot_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign28790_e34614 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign28790_e34611 * ((((var_atatbot_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign28790_e34614 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign28790_e34619;
        var_gammamax_dn5 = assign28790_e34619_d_n5;
        var_gammamax_dn6 = assign28790_e34619_d_n6;
        var_gammamax_dn7 = assign28790_e34619_d_n7;
        var_gammamax_dn8 = assign28790_e34619_d_n8;

        let (assign28800_e34637, assign28800_e34637_d_n5, assign28800_e34637_d_n6, assign28800_e34637_d_n7, assign28800_e34637_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard547 == 0.0)) {
        let assign28800_e34632: f64 = (var_asrh * var_gammamax);
        let assign28800_e34634: f64 = (assign28800_e34632 * var_wtat);
        let assign28800_e34635: f64 = (var_ctatbotd_i * assign28800_e34634);
        (assign28800_e34635, (var_ctatbotd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign28800_e34632 * var_wtat_dn5))), (var_ctatbotd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign28800_e34632 * var_wtat_dn6))), (var_ctatbotd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign28800_e34632 * var_wtat_dn7))), (var_ctatbotd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign28800_e34632 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign28800_e34637;
        var_itat_dn5 = assign28800_e34637_d_n5;
        var_itat_dn6 = assign28800_e34637_d_n6;
        var_itat_dn7 = assign28800_e34637_d_n7;
        var_itat_dn8 = assign28800_e34637_d_n8;

        let assign28810_e34640: f64 = if var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard553 = assign28810_e34640;

        let (assign28820_e34651, assign28820_e34651_d_n5, assign28820_e34651_d_n6, assign28820_e34651_d_n7, assign28820_e34651_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard553 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign28820_e34651;
        var_ibbt_dn5 = assign28820_e34651_d_n5;
        var_ibbt_dn6 = assign28820_e34651_d_n6;
        var_ibbt_dn7 = assign28820_e34651_d_n7;
        var_ibbt_dn8 = assign28820_e34651_d_n8;

        let assign28830_e34654: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard554 = assign28830_e34654;

        let (assign28840_e34673, assign28840_e34673_d_n5, assign28840_e34673_d_n6, assign28840_e34673_d_n7, assign28840_e34673_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard553 == 0.0)) && (var_guard554 != 0.0)) {
        let assign28840_e34668: f64 = (var_vbirbotd_i - var_vbbt);
        let assign28840_e34670: f64 = (assign28840_e34668 * var_vbirbotinv_d);
        let assign28840_e34671: f64 = (assign28840_e34670).sqrt();
        (assign28840_e34671, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28840_e34673;
        var_tmp_dn5 = assign28840_e34673_d_n5;
        var_tmp_dn6 = assign28840_e34673_d_n6;
        var_tmp_dn7 = assign28840_e34673_d_n7;
        var_tmp_dn8 = assign28840_e34673_d_n8;

        let (assign28850_e34694, assign28850_e34694_d_n5, assign28850_e34694_d_n6, assign28850_e34694_d_n7, assign28850_e34694_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard553 == 0.0)) && (var_guard554 == 0.0)) {
        let assign28850_e34688: f64 = (var_vbirbotd_i - var_vbbt);
        let assign28850_e34690: f64 = (assign28850_e34688 * var_vbirbotinv_d);
        let assign28850_e34692: f64 = (assign28850_e34690).powf(var_pbotd_i);
        (assign28850_e34692, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28850_e34694;
        var_tmp_dn5 = assign28850_e34694_d_n5;
        var_tmp_dn6 = assign28850_e34694_d_n6;
        var_tmp_dn7 = assign28850_e34694_d_n7;
        var_tmp_dn8 = assign28850_e34694_d_n8;

        let (assign28860_e34714, assign28860_e34714_d_n5, assign28860_e34714_d_n6, assign28860_e34714_d_n7, assign28860_e34714_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard553 == 0.0)) {
        let assign28860_e34707: f64 = (var_vbirbotd_i - var_vbbt);
        let assign28860_e34709: f64 = (assign28860_e34707 * var_wdepnulrinvbot_d);
        let assign28860_e34711: f64 = (assign28860_e34709 / var_tmp);
        let assign28860_e34712: f64 = (var_one_over_one_minus_pbot_d * assign28860_e34711);
        (assign28860_e34712, (var_one_over_one_minus_pbot_d * (-((assign28860_e34709 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign28860_e34709 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign28860_e34709 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign28860_e34709 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign28860_e34714;
        var_fmaxr_dn5 = assign28860_e34714_d_n5;
        var_fmaxr_dn6 = assign28860_e34714_d_n6;
        var_fmaxr_dn7 = assign28860_e34714_d_n7;
        var_fmaxr_dn8 = assign28860_e34714_d_n8;

        let assign28870_e34716: f64 = (-var_fbbtbot_d);
        let assign28870_e34718: f64 = (assign28870_e34716 / var_fmaxr);
        let assign28870_e34719: f64 = (assign28870_e34718).abs();
        let assign28870_e34721: f64 = if assign28870_e34719 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard555 = assign28870_e34721;

        let (assign28880_e34739, assign28880_e34739_d_n5, assign28880_e34739_d_n6, assign28880_e34739_d_n7, assign28880_e34739_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard553 == 0.0)) && (var_guard555 != 0.0)) {
        let assign28880_e34734: f64 = (-var_fbbtbot_d);
        let assign28880_e34736: f64 = (assign28880_e34734 / var_fmaxr);
        let assign28880_e34737: f64 = (assign28880_e34736).exp();
        (assign28880_e34737, (assign28880_e34737 * (-((assign28880_e34734 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign28880_e34737 * (-((assign28880_e34734 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign28880_e34737 * (-((assign28880_e34734 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign28880_e34737 * (-((assign28880_e34734 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28880_e34739;
        var_tmp_dn5 = assign28880_e34739_d_n5;
        var_tmp_dn6 = assign28880_e34739_d_n6;
        var_tmp_dn7 = assign28880_e34739_d_n7;
        var_tmp_dn8 = assign28880_e34739_d_n8;

        let assign28890_e34741: f64 = (-var_fbbtbot_d);
        let assign28890_e34743: f64 = (assign28890_e34741 / var_fmaxr);
        let assign28890_e34745: f64 = if assign28890_e34743 < 0.0 { 1.0 } else { 0.0 };
        var_guard556 = assign28890_e34745;

        let (assign28900_e34796, assign28900_e34796_d_n5, assign28900_e34796_d_n6, assign28900_e34796_d_n7, assign28900_e34796_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard553 == 0.0)) && (var_guard555 == 0.0)) && (var_guard556 != 0.0)) {
        let assign28900_e34763: f64 = (-230.25850929940458);
        let assign28900_e34765: f64 = (-var_fbbtbot_d);
        let assign28900_e34767: f64 = (assign28900_e34765 / var_fmaxr);
        let assign28900_e34768: f64 = (assign28900_e34763 - assign28900_e34767);
        let assign28900_e34772: f64 = (-230.25850929940458);
        let assign28900_e34774: f64 = (-var_fbbtbot_d);
        let assign28900_e34776: f64 = (assign28900_e34774 / var_fmaxr);
        let assign28900_e34777: f64 = (assign28900_e34772 - assign28900_e34776);
        let assign28900_e34780: f64 = (-230.25850929940458);
        let assign28900_e34782: f64 = (-var_fbbtbot_d);
        let assign28900_e34784: f64 = (assign28900_e34782 / var_fmaxr);
        let assign28900_e34785: f64 = (assign28900_e34780 - assign28900_e34784);
        let assign28900_e34787: f64 = (assign28900_e34785 * 0.3333333333333333);
        let assign28900_e34788: f64 = (1.0 + assign28900_e34787);
        let assign28900_e34789: f64 = (assign28900_e34777 * assign28900_e34788);
        let assign28900_e34790: f64 = (0.5 * assign28900_e34789);
        let assign28900_e34791: f64 = (1.0 + assign28900_e34790);
        let assign28900_e34792: f64 = (assign28900_e34768 * assign28900_e34791);
        let assign28900_e34793: f64 = (1.0 + assign28900_e34792);
        let assign28900_e34794: f64 = (1e-100 / assign28900_e34793);
        (assign28900_e34794, (-((1e-100 * (((-(-((assign28900_e34765 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign28900_e34791) + (assign28900_e34768 * (0.5 * (((-(-((assign28900_e34774 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign28900_e34788) + (assign28900_e34777 * ((-(-((assign28900_e34782 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign28900_e34793 * assign28900_e34793))), (-((1e-100 * (((-(-((assign28900_e34765 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign28900_e34791) + (assign28900_e34768 * (0.5 * (((-(-((assign28900_e34774 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign28900_e34788) + (assign28900_e34777 * ((-(-((assign28900_e34782 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign28900_e34793 * assign28900_e34793))), (-((1e-100 * (((-(-((assign28900_e34765 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign28900_e34791) + (assign28900_e34768 * (0.5 * (((-(-((assign28900_e34774 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign28900_e34788) + (assign28900_e34777 * ((-(-((assign28900_e34782 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign28900_e34793 * assign28900_e34793))), (-((1e-100 * (((-(-((assign28900_e34765 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign28900_e34791) + (assign28900_e34768 * (0.5 * (((-(-((assign28900_e34774 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign28900_e34788) + (assign28900_e34777 * ((-(-((assign28900_e34782 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign28900_e34793 * assign28900_e34793))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28900_e34796;
        var_tmp_dn5 = assign28900_e34796_d_n5;
        var_tmp_dn6 = assign28900_e34796_d_n6;
        var_tmp_dn7 = assign28900_e34796_d_n7;
        var_tmp_dn8 = assign28900_e34796_d_n8;

        let (assign28910_e34845, assign28910_e34845_d_n5, assign28910_e34845_d_n6, assign28910_e34845_d_n7, assign28910_e34845_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard553 == 0.0)) && (var_guard555 == 0.0)) && (var_guard556 == 0.0)) {
        let assign28910_e34815: f64 = (-var_fbbtbot_d);
        let assign28910_e34817: f64 = (assign28910_e34815 / var_fmaxr);
        let assign28910_e34819: f64 = (assign28910_e34817 - 230.25850929940458);
        let assign28910_e34823: f64 = (-var_fbbtbot_d);
        let assign28910_e34825: f64 = (assign28910_e34823 / var_fmaxr);
        let assign28910_e34827: f64 = (assign28910_e34825 - 230.25850929940458);
        let assign28910_e34830: f64 = (-var_fbbtbot_d);
        let assign28910_e34832: f64 = (assign28910_e34830 / var_fmaxr);
        let assign28910_e34834: f64 = (assign28910_e34832 - 230.25850929940458);
        let assign28910_e34836: f64 = (assign28910_e34834 * 0.3333333333333333);
        let assign28910_e34837: f64 = (1.0 + assign28910_e34836);
        let assign28910_e34838: f64 = (assign28910_e34827 * assign28910_e34837);
        let assign28910_e34839: f64 = (0.5 * assign28910_e34838);
        let assign28910_e34840: f64 = (1.0 + assign28910_e34839);
        let assign28910_e34841: f64 = (assign28910_e34819 * assign28910_e34840);
        let assign28910_e34842: f64 = (1.0 + assign28910_e34841);
        let assign28910_e34843: f64 = (1e100 * assign28910_e34842);
        (assign28910_e34843, (1e100 * (((-((assign28910_e34815 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign28910_e34840) + (assign28910_e34819 * (0.5 * (((-((assign28910_e34823 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign28910_e34837) + (assign28910_e34827 * ((-((assign28910_e34830 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign28910_e34815 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign28910_e34840) + (assign28910_e34819 * (0.5 * (((-((assign28910_e34823 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign28910_e34837) + (assign28910_e34827 * ((-((assign28910_e34830 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign28910_e34815 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign28910_e34840) + (assign28910_e34819 * (0.5 * (((-((assign28910_e34823 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign28910_e34837) + (assign28910_e34827 * ((-((assign28910_e34830 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign28910_e34815 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign28910_e34840) + (assign28910_e34819 * (0.5 * (((-((assign28910_e34823 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign28910_e34837) + (assign28910_e34827 * ((-((assign28910_e34830 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28910_e34845;
        var_tmp_dn5 = assign28910_e34845_d_n5;
        var_tmp_dn6 = assign28910_e34845_d_n6;
        var_tmp_dn7 = assign28910_e34845_d_n7;
        var_tmp_dn8 = assign28910_e34845_d_n8;

        let (assign28920_e34865, assign28920_e34865_d_n5, assign28920_e34865_d_n6, assign28920_e34865_d_n7, assign28920_e34865_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard553 == 0.0)) {
        let assign28920_e34858: f64 = (var_v1 * var_fmaxr);
        let assign28920_e34860: f64 = (assign28920_e34858 * var_fmaxr);
        let assign28920_e34862: f64 = (assign28920_e34860 * var_tmp);
        let assign28920_e34863: f64 = (var_cbbtbotd_i * assign28920_e34862);
        (assign28920_e34863, (var_cbbtbotd_i * (((((var_v1 * var_fmaxr_dn5) * var_fmaxr) + (assign28920_e34858 * var_fmaxr_dn5)) * var_tmp) + (assign28920_e34860 * var_tmp_dn5))), (var_cbbtbotd_i * (((((var_v1 * var_fmaxr_dn6) * var_fmaxr) + (assign28920_e34858 * var_fmaxr_dn6)) * var_tmp) + (assign28920_e34860 * var_tmp_dn6))), (var_cbbtbotd_i * (((((var_v1 * var_fmaxr_dn7) * var_fmaxr) + (assign28920_e34858 * var_fmaxr_dn7)) * var_tmp) + (assign28920_e34860 * var_tmp_dn7))), (var_cbbtbotd_i * (((((var_v1 * var_fmaxr_dn8) * var_fmaxr) + (assign28920_e34858 * var_fmaxr_dn8)) * var_tmp) + (assign28920_e34860 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign28920_e34865;
        var_ibbt_dn5 = assign28920_e34865_d_n5;
        var_ibbt_dn6 = assign28920_e34865_d_n6;
        var_ibbt_dn7 = assign28920_e34865_d_n7;
        var_ibbt_dn8 = assign28920_e34865_d_n8;

        let assign28930_e34868: f64 = if var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard557 = assign28930_e34868;

        let (assign28940_e34879, assign28940_e34879_d_n5, assign28940_e34879_d_n6, assign28940_e34879_d_n7, assign28940_e34879_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard557 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign28940_e34879;
        var_fbreakdown_dn5 = assign28940_e34879_d_n5;
        var_fbreakdown_dn6 = assign28940_e34879_d_n6;
        var_fbreakdown_dn7 = assign28940_e34879_d_n7;
        var_fbreakdown_dn8 = assign28940_e34879_d_n8;

        let assign28950_e34882: f64 = (-var_alphaav);
        let assign28950_e34884: f64 = (assign28950_e34882 * var_vbrbotd_i);
        let assign28950_e34885: f64 = if var_vav > assign28950_e34884 { 1.0 } else { 0.0 };
        var_guard558 = assign28950_e34885;

        let assign28960_e34888: f64 = if var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard559 = assign28960_e34888;

        let (assign28970_e34918, assign28970_e34918_d_n5, assign28970_e34918_d_n6, assign28970_e34918_d_n7, assign28970_e34918_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard557 == 0.0)) && (var_guard558 != 0.0)) && (var_guard559 != 0.0)) {
        let assign28970_e34904: f64 = (var_vav * var_vbrinvbot_d);
        let assign28970_e34907: f64 = (var_vav * var_vbrinvbot_d);
        let assign28970_e34908: f64 = (assign28970_e34904 * assign28970_e34907);
        let assign28970_e34911: f64 = (var_vav * var_vbrinvbot_d);
        let assign28970_e34912: f64 = (assign28970_e34908 * assign28970_e34911);
        let assign28970_e34915: f64 = (var_vav * var_vbrinvbot_d);
        let assign28970_e34916: f64 = (assign28970_e34912 * assign28970_e34915);
        (assign28970_e34916, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28970_e34918;
        var_tmp_dn5 = assign28970_e34918_d_n5;
        var_tmp_dn6 = assign28970_e34918_d_n6;
        var_tmp_dn7 = assign28970_e34918_d_n7;
        var_tmp_dn8 = assign28970_e34918_d_n8;

        let (assign28980_e34940, assign28980_e34940_d_n5, assign28980_e34940_d_n6, assign28980_e34940_d_n7, assign28980_e34940_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard557 == 0.0)) && (var_guard558 != 0.0)) && (var_guard559 == 0.0)) {
        let assign28980_e34935: f64 = (var_vav * var_vbrinvbot_d);
        let assign28980_e34936: f64 = (assign28980_e34935).abs();
        let assign28980_e34938: f64 = (assign28980_e34936).powf(var_pbrbotd_i);
        (assign28980_e34938, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign28980_e34940;
        var_tmp_dn5 = assign28980_e34940_d_n5;
        var_tmp_dn6 = assign28980_e34940_d_n6;
        var_tmp_dn7 = assign28980_e34940_d_n7;
        var_tmp_dn8 = assign28980_e34940_d_n8;

        let (assign28990_e34958, assign28990_e34958_d_n5, assign28990_e34958_d_n6, assign28990_e34958_d_n7, assign28990_e34958_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard557 == 0.0)) && (var_guard558 != 0.0)) {
        let assign28990_e34955: f64 = (1.0 - var_tmp);
        let assign28990_e34956: f64 = (1.0 / assign28990_e34955);
        (assign28990_e34956, (-((-var_tmp_dn5) / (assign28990_e34955 * assign28990_e34955))), (-((-var_tmp_dn6) / (assign28990_e34955 * assign28990_e34955))), (-((-var_tmp_dn7) / (assign28990_e34955 * assign28990_e34955))), (-((-var_tmp_dn8) / (assign28990_e34955 * assign28990_e34955))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign28990_e34958;
        var_fbreakdown_dn5 = assign28990_e34958_d_n5;
        var_fbreakdown_dn6 = assign28990_e34958_d_n6;
        var_fbreakdown_dn7 = assign28990_e34958_d_n7;
        var_fbreakdown_dn8 = assign28990_e34958_d_n8;

        let (assign29000_e34981, assign29000_e34981_d_n5, assign29000_e34981_d_n6, assign29000_e34981_d_n7, assign29000_e34981_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) && (var_guard557 == 0.0)) && (var_guard558 == 0.0)) {
        let assign29000_e34975: f64 = (var_alphaav * var_vbrbotd_i);
        let assign29000_e34976: f64 = (var_vav + assign29000_e34975);
        let assign29000_e34978: f64 = (assign29000_e34976 * var_slopebot_d);
        let assign29000_e34979: f64 = (var_fstopbot_d + assign29000_e34978);
        (assign29000_e34979, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign29000_e34981;
        var_fbreakdown_dn5 = assign29000_e34981_d_n5;
        var_fbreakdown_dn6 = assign29000_e34981_d_n6;
        var_fbreakdown_dn7 = assign29000_e34981_d_n7;
        var_fbreakdown_dn8 = assign29000_e34981_d_n8;

        let (assign29010_e35000, assign29010_e35000_d_n5, assign29010_e35000_d_n6, assign29010_e35000_d_n7, assign29010_e35000_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard543 == 0.0)) {
        let assign29010_e34991: f64 = (var_id__blk219 + var_isrh);
        let assign29010_e34993: f64 = (assign29010_e34991 + var_itat);
        let assign29010_e34995: f64 = (assign29010_e34993 + var_ibbt);
        let assign29010_e34996: f64 = (p.p29 * assign29010_e34995);
        let assign29010_e34998: f64 = (assign29010_e34996 * var_fbreakdown);
        (assign29010_e34998, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign29010_e34996 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign29010_e34996 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign29010_e34996 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign29010_e34996 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign29010_e35000;
        var_ijunbot_dn5 = assign29010_e35000_d_n5;
        var_ijunbot_dn6 = assign29010_e35000_d_n6;
        var_ijunbot_dn7 = assign29010_e35000_d_n7;
        var_ijunbot_dn8 = assign29010_e35000_d_n8;

        let assign29020_e35003: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard560 = assign29020_e35003;

        let (assign29030_e35011, assign29030_e35011_d_n5, assign29030_e35011_d_n6, assign29030_e35011_d_n7, assign29030_e35011_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign29030_e35011;
        var_ijunsti_dn5 = assign29030_e35011_d_n5;
        var_ijunsti_dn6 = assign29030_e35011_d_n6;
        var_ijunsti_dn7 = assign29030_e35011_d_n7;
        var_ijunsti_dn8 = assign29030_e35011_d_n8;

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
        *var_guard550_slot = var_guard550;
        *var_guard551_slot = var_guard551;
        *var_guard552_slot = var_guard552;
        *var_guard553_slot = var_guard553;
        *var_guard554_slot = var_guard554;
        *var_guard555_slot = var_guard555;
        *var_guard556_slot = var_guard556;
        *var_guard557_slot = var_guard557;
        *var_guard558_slot = var_guard558;
        *var_guard559_slot = var_guard559;
        *var_guard560_slot = var_guard560;
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

    pub(super) fn stamp_transient_block_57(
        var_atatsti_d: f64,
        var_btatpartsti_d: f64,
        var_csrhstid_i: f64,
        var_ctatstid_i: f64,
        var_ftdsti_d: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard560: f64,
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
        var_guard561_slot: &mut f64,
        var_guard562_slot: &mut f64,
        var_guard563_slot: &mut f64,
        var_guard564_slot: &mut f64,
        var_guard565_slot: &mut f64,
        var_guard566_slot: &mut f64,
        var_guard567_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
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
        let mut var_guard561: f64 = *var_guard561_slot;
        let mut var_guard562: f64 = *var_guard562_slot;
        let mut var_guard563: f64 = *var_guard563_slot;
        let mut var_guard564: f64 = *var_guard564_slot;
        let mut var_guard565: f64 = *var_guard565_slot;
        let mut var_guard566: f64 = *var_guard566_slot;
        let mut var_guard567: f64 = *var_guard567_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
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

        let (assign29040_e35022,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) {
        let assign29040_e35020: f64 = (var_idsatsti_d * var_idmult);
        (assign29040_e35020,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign29040_e35022;

        let assign29050_e35029: f64 = if ((var_csrhstid_i == 0.0) && (var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard561 = assign29050_e35029;

        let (assign29060_e35040, assign29060_e35040_d_n5, assign29060_e35040_d_n6, assign29060_e35040_d_n7, assign29060_e35040_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard561 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign29060_e35040;
        var_isrh_dn5 = assign29060_e35040_d_n5;
        var_isrh_dn6 = assign29060_e35040_d_n6;
        var_isrh_dn7 = assign29060_e35040_d_n7;
        var_isrh_dn8 = assign29060_e35040_d_n8;

        let (assign29070_e35054,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard561 == 0.0)) {
        let assign29070_e35052: f64 = (var_vbisti_d - var_vjsrh);
        (assign29070_e35052,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign29070_e35054;

        let (assign29080_e35073,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard561 == 0.0)) {
        let assign29080_e35068: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign29080_e35069: f64 = (1.0 - assign29080_e35068);
        let assign29080_e35070: f64 = (assign29080_e35069).sqrt();
        let assign29080_e35071: f64 = (1.0 - assign29080_e35070);
        (assign29080_e35071,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign29080_e35073;

        let assign29090_e35076: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard562 = assign29090_e35076;

        let (assign29100_e35090,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard561 == 0.0)) && (var_guard562 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign29100_e35090;

        let (assign29110_e35122,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard561 == 0.0)) && (var_guard562 == 0.0)) {
        let assign29110_e35105: f64 = (var_wsrhstep * var_wsrhstep);
        let assign29110_e35107: f64 = (var_wsrhstep).ln();
        let assign29110_e35108: f64 = (assign29110_e35105 * assign29110_e35107);
        let assign29110_e35111: f64 = (1.0 - var_wsrhstep);
        let assign29110_e35112: f64 = (assign29110_e35108 / assign29110_e35111);
        let assign29110_e35114: f64 = (assign29110_e35112 + var_wsrhstep);
        let assign29110_e35118: f64 = (2.0 * var_pstid_i);
        let assign29110_e35119: f64 = (1.0 - assign29110_e35118);
        let assign29110_e35120: f64 = (assign29110_e35114 * assign29110_e35119);
        (assign29110_e35120,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign29110_e35122;

        let (assign29120_e35136,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard561 == 0.0)) {
        let assign29120_e35134: f64 = (var_wsrhstep + var_dwsrh);
        (assign29120_e35134,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign29120_e35136;

        let assign29130_e35139: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard563 = assign29130_e35139;

        let (assign29140_e35156, assign29140_e35156_d_n5, assign29140_e35156_d_n6, assign29140_e35156_d_n7, assign29140_e35156_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard561 == 0.0)) && (var_guard563 != 0.0)) {
        let assign29140_e35153: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign29140_e35154: f64 = (assign29140_e35153).sqrt();
        (assign29140_e35154, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29140_e35156;
        var_tmp_dn5 = assign29140_e35156_d_n5;
        var_tmp_dn6 = assign29140_e35156_d_n6;
        var_tmp_dn7 = assign29140_e35156_d_n7;
        var_tmp_dn8 = assign29140_e35156_d_n8;

        let (assign29150_e35175, assign29150_e35175_d_n5, assign29150_e35175_d_n6, assign29150_e35175_d_n7, assign29150_e35175_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard561 == 0.0)) && (var_guard563 == 0.0)) {
        let assign29150_e35171: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign29150_e35173: f64 = (assign29150_e35171).powf(var_pstid_i);
        (assign29150_e35173, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29150_e35175;
        var_tmp_dn5 = assign29150_e35175_d_n5;
        var_tmp_dn6 = assign29150_e35175_d_n6;
        var_tmp_dn7 = assign29150_e35175_d_n7;
        var_tmp_dn8 = assign29150_e35175_d_n8;

        let (assign29160_e35189, assign29160_e35189_d_n5, assign29160_e35189_d_n6, assign29160_e35189_d_n7, assign29160_e35189_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard561 == 0.0)) {
        let assign29160_e35187: f64 = (var_wdepnulrsti_d * var_tmp);
        (assign29160_e35187, (var_wdepnulrsti_d * var_tmp_dn5), (var_wdepnulrsti_d * var_tmp_dn6), (var_wdepnulrsti_d * var_tmp_dn7), (var_wdepnulrsti_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign29160_e35189;
        var_wdep_dn5 = assign29160_e35189_d_n5;
        var_wdep_dn6 = assign29160_e35189_d_n6;
        var_wdep_dn7 = assign29160_e35189_d_n7;
        var_wdep_dn8 = assign29160_e35189_d_n8;

        let (assign29170_e35207, assign29170_e35207_d_n5, assign29170_e35207_d_n6, assign29170_e35207_d_n7, assign29170_e35207_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard561 == 0.0)) {
        let assign29170_e35202: f64 = (var_zinv - 1.0);
        let assign29170_e35204: f64 = (assign29170_e35202 * var_wdep);
        let assign29170_e35205: f64 = (var_ftdsti_d * assign29170_e35204);
        (assign29170_e35205, (var_ftdsti_d * (assign29170_e35202 * var_wdep_dn5)), (var_ftdsti_d * (assign29170_e35202 * var_wdep_dn6)), (var_ftdsti_d * (assign29170_e35202 * var_wdep_dn7)), (var_ftdsti_d * (assign29170_e35202 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign29170_e35207;
        var_asrh_dn5 = assign29170_e35207_d_n5;
        var_asrh_dn6 = assign29170_e35207_d_n6;
        var_asrh_dn7 = assign29170_e35207_d_n7;
        var_asrh_dn8 = assign29170_e35207_d_n8;

        let (assign29180_e35223, assign29180_e35223_d_n5, assign29180_e35223_d_n6, assign29180_e35223_d_n7, assign29180_e35223_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard561 == 0.0)) {
        let assign29180_e35220: f64 = (var_asrh * var_wsrh);
        let assign29180_e35221: f64 = (var_csrhstid_i * assign29180_e35220);
        (assign29180_e35221, (var_csrhstid_i * (var_asrh_dn5 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn6 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn7 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign29180_e35223;
        var_isrh_dn5 = assign29180_e35223_d_n5;
        var_isrh_dn6 = assign29180_e35223_d_n6;
        var_isrh_dn7 = assign29180_e35223_d_n7;
        var_isrh_dn8 = assign29180_e35223_d_n8;

        let assign29190_e35226: f64 = if var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard564 = assign29190_e35226;

        let (assign29200_e35237, assign29200_e35237_d_n5, assign29200_e35237_d_n6, assign29200_e35237_d_n7, assign29200_e35237_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign29200_e35237;
        var_itat_dn5 = assign29200_e35237_d_n5;
        var_itat_dn6 = assign29200_e35237_d_n6;
        var_itat_dn7 = assign29200_e35237_d_n7;
        var_itat_dn8 = assign29200_e35237_d_n8;

        let (assign29210_e35255, assign29210_e35255_d_n5, assign29210_e35255_d_n6, assign29210_e35255_d_n7, assign29210_e35255_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) {
        let assign29210_e35250: f64 = (var_wdep * var_one_minus_psti_d);
        let assign29210_e35252: f64 = (assign29210_e35250 / var_vbi_minus_vjsrh);
        let assign29210_e35253: f64 = (var_btatpartsti_d * assign29210_e35252);
        (assign29210_e35253, (var_btatpartsti_d * ((var_wdep_dn5 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn6 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn7 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn8 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign29210_e35255;
        var_btat_dn5 = assign29210_e35255_d_n5;
        var_btat_dn6 = assign29210_e35255_d_n6;
        var_btat_dn7 = assign29210_e35255_d_n7;
        var_btat_dn8 = assign29210_e35255_d_n8;

        let (assign29220_e35271, assign29220_e35271_d_n5, assign29220_e35271_d_n6, assign29220_e35271_d_n7, assign29220_e35271_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) {
        let assign29220_e35267: f64 = (0.666666666666667 * var_atatsti_d);
        let assign29220_e35269: f64 = (assign29220_e35267 / var_btat);
        (assign29220_e35269, (-((assign29220_e35267 * var_btat_dn5) / (var_btat * var_btat))), (-((assign29220_e35267 * var_btat_dn6) / (var_btat * var_btat))), (-((assign29220_e35267 * var_btat_dn7) / (var_btat * var_btat))), (-((assign29220_e35267 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign29220_e35271;
        var_twoatatoverthreebtat_dn5 = assign29220_e35271_d_n5;
        var_twoatatoverthreebtat_dn6 = assign29220_e35271_d_n6;
        var_twoatatoverthreebtat_dn7 = assign29220_e35271_d_n7;
        var_twoatatoverthreebtat_dn8 = assign29220_e35271_d_n8;

        let (assign29230_e35285, assign29230_e35285_d_n5, assign29230_e35285_d_n6, assign29230_e35285_d_n7, assign29230_e35285_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) {
        let assign29230_e35283: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign29230_e35283, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign29230_e35285;
        var_umaxbeforelimiting_dn5 = assign29230_e35285_d_n5;
        var_umaxbeforelimiting_dn6 = assign29230_e35285_d_n6;
        var_umaxbeforelimiting_dn7 = assign29230_e35285_d_n7;
        var_umaxbeforelimiting_dn8 = assign29230_e35285_d_n8;

        let (assign29240_e35306, assign29240_e35306_d_n5, assign29240_e35306_d_n6, assign29240_e35306_d_n7, assign29240_e35306_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) {
        let assign29240_e35297: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign29240_e35300: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign29240_e35302: f64 = (assign29240_e35300 + 1.0);
        let assign29240_e35303: f64 = (assign29240_e35297 / assign29240_e35302);
        let assign29240_e35304: f64 = (assign29240_e35303).sqrt();
        (assign29240_e35304, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign29240_e35302) - (assign29240_e35297 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign29240_e35302 * assign29240_e35302)) / (2.0 * assign29240_e35304)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign29240_e35302) - (assign29240_e35297 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign29240_e35302 * assign29240_e35302)) / (2.0 * assign29240_e35304)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign29240_e35302) - (assign29240_e35297 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign29240_e35302 * assign29240_e35302)) / (2.0 * assign29240_e35304)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign29240_e35302) - (assign29240_e35297 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign29240_e35302 * assign29240_e35302)) / (2.0 * assign29240_e35304)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign29240_e35306;
        var_umax_dn5 = assign29240_e35306_d_n5;
        var_umax_dn6 = assign29240_e35306_d_n6;
        var_umax_dn7 = assign29240_e35306_d_n7;
        var_umax_dn8 = assign29240_e35306_d_n8;

        let (assign29250_e35319, assign29250_e35319_d_n5, assign29250_e35319_d_n6, assign29250_e35319_d_n7, assign29250_e35319_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) {
        let assign29250_e35317: f64 = (var_umax).sqrt();
        (assign29250_e35317, (var_umax_dn5 / (2.0 * assign29250_e35317)), (var_umax_dn6 / (2.0 * assign29250_e35317)), (var_umax_dn7 / (2.0 * assign29250_e35317)), (var_umax_dn8 / (2.0 * assign29250_e35317)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign29250_e35319;
        var_sqrtumax_dn5 = assign29250_e35319_d_n5;
        var_sqrtumax_dn6 = assign29250_e35319_d_n6;
        var_sqrtumax_dn7 = assign29250_e35319_d_n7;
        var_sqrtumax_dn8 = assign29250_e35319_d_n8;

        let (assign29260_e35333, assign29260_e35333_d_n5, assign29260_e35333_d_n6, assign29260_e35333_d_n7, assign29260_e35333_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) {
        let assign29260_e35331: f64 = (var_umax * var_sqrtumax);
        (assign29260_e35331, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign29260_e35333;
        var_umaxpoweronepointfive_dn5 = assign29260_e35333_d_n5;
        var_umaxpoweronepointfive_dn6 = assign29260_e35333_d_n6;
        var_umaxpoweronepointfive_dn7 = assign29260_e35333_d_n7;
        var_umaxpoweronepointfive_dn8 = assign29260_e35333_d_n8;

        let assign29270_e35335: f64 = (-var_pstid_i);
        let assign29270_e35337: f64 = (assign29270_e35335 * var_one_over_one_minus_psti_d);
        let assign29270_e35339: f64 = (-1.0);
        let assign29270_e35340: f64 = if assign29270_e35337 == assign29270_e35339 { 1.0 } else { 0.0 };
        var_guard565 = assign29270_e35340;

        let (assign29280_e35360, assign29280_e35360_d_n5, assign29280_e35360_d_n6, assign29280_e35360_d_n7, assign29280_e35360_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) && (var_guard565 != 0.0)) {
        let assign29280_e35356: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign29280_e35357: f64 = (1.0 + assign29280_e35356);
        let assign29280_e35358: f64 = (1.0 / assign29280_e35357);
        (assign29280_e35358, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign29280_e35357 * assign29280_e35357))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign29280_e35357 * assign29280_e35357))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign29280_e35357 * assign29280_e35357))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign29280_e35357 * assign29280_e35357))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign29280_e35360;
        var_wgamma_dn5 = assign29280_e35360_d_n5;
        var_wgamma_dn6 = assign29280_e35360_d_n6;
        var_wgamma_dn7 = assign29280_e35360_d_n7;
        var_wgamma_dn8 = assign29280_e35360_d_n8;

        let (assign29290_e35384, assign29290_e35384_d_n5, assign29290_e35384_d_n6, assign29290_e35384_d_n7, assign29290_e35384_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) && (var_guard565 == 0.0)) {
        let assign29290_e35376: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign29290_e35377: f64 = (1.0 + assign29290_e35376);
        let assign29290_e35379: f64 = (-var_pstid_i);
        let assign29290_e35381: f64 = (assign29290_e35379 * var_one_over_one_minus_psti_d);
        let assign29290_e35382: f64 = (assign29290_e35377).powf(assign29290_e35381);
        (assign29290_e35382, if 0.0 == 0.0 && ((assign29290_e35381) as f64).is_finite() && ((assign29290_e35381) as f64).fract() == 0.0 { if assign29290_e35381 == 0.0 { 0.0 } else { (assign29290_e35381 * ((assign29290_e35377).powf(assign29290_e35381 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign29290_e35382 * (assign29290_e35381 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign29290_e35377))) }, if 0.0 == 0.0 && ((assign29290_e35381) as f64).is_finite() && ((assign29290_e35381) as f64).fract() == 0.0 { if assign29290_e35381 == 0.0 { 0.0 } else { (assign29290_e35381 * ((assign29290_e35377).powf(assign29290_e35381 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign29290_e35382 * (assign29290_e35381 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign29290_e35377))) }, if 0.0 == 0.0 && ((assign29290_e35381) as f64).is_finite() && ((assign29290_e35381) as f64).fract() == 0.0 { if assign29290_e35381 == 0.0 { 0.0 } else { (assign29290_e35381 * ((assign29290_e35377).powf(assign29290_e35381 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign29290_e35382 * (assign29290_e35381 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign29290_e35377))) }, if 0.0 == 0.0 && ((assign29290_e35381) as f64).is_finite() && ((assign29290_e35381) as f64).fract() == 0.0 { if assign29290_e35381 == 0.0 { 0.0 } else { (assign29290_e35381 * ((assign29290_e35377).powf(assign29290_e35381 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign29290_e35382 * (assign29290_e35381 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign29290_e35377))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign29290_e35384;
        var_wgamma_dn5 = assign29290_e35384_d_n5;
        var_wgamma_dn6 = assign29290_e35384_d_n6;
        var_wgamma_dn7 = assign29290_e35384_d_n7;
        var_wgamma_dn8 = assign29290_e35384_d_n8;

        let (assign29300_e35402, assign29300_e35402_d_n5, assign29300_e35402_d_n6, assign29300_e35402_d_n7, assign29300_e35402_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) {
        let assign29300_e35396: f64 = (var_wsrh * var_wgamma);
        let assign29300_e35399: f64 = (var_wsrh + var_wgamma);
        let assign29300_e35400: f64 = (assign29300_e35396 / assign29300_e35399);
        (assign29300_e35400, ((((var_wsrh * var_wgamma_dn5) * assign29300_e35399) - (assign29300_e35396 * var_wgamma_dn5)) / (assign29300_e35399 * assign29300_e35399)), ((((var_wsrh * var_wgamma_dn6) * assign29300_e35399) - (assign29300_e35396 * var_wgamma_dn6)) / (assign29300_e35399 * assign29300_e35399)), ((((var_wsrh * var_wgamma_dn7) * assign29300_e35399) - (assign29300_e35396 * var_wgamma_dn7)) / (assign29300_e35399 * assign29300_e35399)), ((((var_wsrh * var_wgamma_dn8) * assign29300_e35399) - (assign29300_e35396 * var_wgamma_dn8)) / (assign29300_e35399 * assign29300_e35399)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign29300_e35402;
        var_wtat_dn5 = assign29300_e35402_d_n5;
        var_wtat_dn6 = assign29300_e35402_d_n6;
        var_wtat_dn7 = assign29300_e35402_d_n7;
        var_wtat_dn8 = assign29300_e35402_d_n8;

        let (assign29310_e35419, assign29310_e35419_d_n5, assign29310_e35419_d_n6, assign29310_e35419_d_n7, assign29310_e35419_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) {
        let assign29310_e35415: f64 = (var_btat / var_sqrtumax);
        let assign29310_e35416: f64 = (0.375 * assign29310_e35415);
        let assign29310_e35417: f64 = (assign29310_e35416).sqrt();
        (assign29310_e35417, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign29310_e35417)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign29310_e35417)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign29310_e35417)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign29310_e35417)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign29310_e35419;
        var_ktat_dn5 = assign29310_e35419_d_n5;
        var_ktat_dn6 = assign29310_e35419_d_n6;
        var_ktat_dn7 = assign29310_e35419_d_n7;
        var_ktat_dn8 = assign29310_e35419_d_n8;

        let (assign29320_e35437, assign29320_e35437_d_n5, assign29320_e35437_d_n6, assign29320_e35437_d_n7, assign29320_e35437_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) {
        let assign29320_e35432: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign29320_e35433: f64 = (2.0 * assign29320_e35432);
        let assign29320_e35435: f64 = (assign29320_e35433 - var_umax);
        (assign29320_e35435, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign29320_e35437;
        var_ltat_dn5 = assign29320_e35437_d_n5;
        var_ltat_dn6 = assign29320_e35437_d_n6;
        var_ltat_dn7 = assign29320_e35437_d_n7;
        var_ltat_dn8 = assign29320_e35437_d_n8;

        let (assign29330_e35463, assign29330_e35463_d_n5, assign29330_e35463_d_n6, assign29330_e35463_d_n7, assign29330_e35463_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) {
        let assign29330_e35449: f64 = (var_atatsti_d * var_twoatatoverthreebtat);
        let assign29330_e35451: f64 = (assign29330_e35449 * var_sqrtumax);
        let assign29330_e35454: f64 = (var_atatsti_d * var_umax);
        let assign29330_e35455: f64 = (assign29330_e35451 - assign29330_e35454);
        let assign29330_e35459: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign29330_e35460: f64 = (0.5 * assign29330_e35459);
        let assign29330_e35461: f64 = (assign29330_e35455 + assign29330_e35460);
        (assign29330_e35461, (((((var_atatsti_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign29330_e35449 * var_sqrtumax_dn5)) - (var_atatsti_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign29330_e35449 * var_sqrtumax_dn6)) - (var_atatsti_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign29330_e35449 * var_sqrtumax_dn7)) - (var_atatsti_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign29330_e35449 * var_sqrtumax_dn8)) - (var_atatsti_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign29330_e35463;
        var_mtat_dn5 = assign29330_e35463_d_n5;
        var_mtat_dn6 = assign29330_e35463_d_n6;
        var_mtat_dn7 = assign29330_e35463_d_n7;
        var_mtat_dn8 = assign29330_e35463_d_n8;

        let (assign29340_e35479, assign29340_e35479_d_n5, assign29340_e35479_d_n6, assign29340_e35479_d_n7, assign29340_e35479_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) {
        let assign29340_e35475: f64 = (var_ltat - 1.0);
        let assign29340_e35477: f64 = (assign29340_e35475 * var_ktat);
        (assign29340_e35477, ((var_ltat_dn5 * var_ktat) + (assign29340_e35475 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign29340_e35475 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign29340_e35475 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign29340_e35475 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign29340_e35479;
        var_xerfc_dn5 = assign29340_e35479_d_n5;
        var_xerfc_dn6 = assign29340_e35479_d_n6;
        var_xerfc_dn7 = assign29340_e35479_d_n7;
        var_xerfc_dn8 = assign29340_e35479_d_n8;

        let (assign29350_e35493, assign29350_e35493_d_n5, assign29350_e35493_d_n6, assign29350_e35493_d_n7, assign29350_e35493_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) {
        let assign29350_e35491: f64 = (var_xerfc * var_xerfc);
        (assign29350_e35491, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign29350_e35493;
        var_ysq_dn5 = assign29350_e35493_d_n5;
        var_ysq_dn6 = assign29350_e35493_d_n6;
        var_ysq_dn7 = assign29350_e35493_d_n7;
        var_ysq_dn8 = assign29350_e35493_d_n8;

        let assign29360_e35496: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard566 = assign29360_e35496;

        let (assign29370_e35516, assign29370_e35516_d_n5, assign29370_e35516_d_n6, assign29370_e35516_d_n7, assign29370_e35516_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) && (var_guard566 != 0.0)) {
        let assign29370_e35512: f64 = (var_perfc * var_xerfc);
        let assign29370_e35513: f64 = (1.0 + assign29370_e35512);
        let assign29370_e35514: f64 = (1.0 / assign29370_e35513);
        (assign29370_e35514, (-((var_perfc * var_xerfc_dn5) / (assign29370_e35513 * assign29370_e35513))), (-((var_perfc * var_xerfc_dn6) / (assign29370_e35513 * assign29370_e35513))), (-((var_perfc * var_xerfc_dn7) / (assign29370_e35513 * assign29370_e35513))), (-((var_perfc * var_xerfc_dn8) / (assign29370_e35513 * assign29370_e35513))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign29370_e35516;
        var_terfc_dn5 = assign29370_e35516_d_n5;
        var_terfc_dn6 = assign29370_e35516_d_n6;
        var_terfc_dn7 = assign29370_e35516_d_n7;
        var_terfc_dn8 = assign29370_e35516_d_n8;

        let (assign29380_e35537, assign29380_e35537_d_n5, assign29380_e35537_d_n6, assign29380_e35537_d_n7, assign29380_e35537_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) && (var_guard566 == 0.0)) {
        let assign29380_e35533: f64 = (var_perfc * var_xerfc);
        let assign29380_e35534: f64 = (1.0 - assign29380_e35533);
        let assign29380_e35535: f64 = (1.0 / assign29380_e35534);
        (assign29380_e35535, (-((-(var_perfc * var_xerfc_dn5)) / (assign29380_e35534 * assign29380_e35534))), (-((-(var_perfc * var_xerfc_dn6)) / (assign29380_e35534 * assign29380_e35534))), (-((-(var_perfc * var_xerfc_dn7)) / (assign29380_e35534 * assign29380_e35534))), (-((-(var_perfc * var_xerfc_dn8)) / (assign29380_e35534 * assign29380_e35534))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign29380_e35537;
        var_terfc_dn5 = assign29380_e35537_d_n5;
        var_terfc_dn6 = assign29380_e35537_d_n6;
        var_terfc_dn7 = assign29380_e35537_d_n7;
        var_terfc_dn8 = assign29380_e35537_d_n8;

        let assign29390_e35539: f64 = (-var_ysq);
        let assign29390_e35541: f64 = (assign29390_e35539 + var_mtat);
        let assign29390_e35543: f64 = (-230.25850929940458);
        let assign29390_e35544: f64 = if assign29390_e35541 > assign29390_e35543 { 1.0 } else { 0.0 };
        var_guard567 = assign29390_e35544;

        let (assign29400_e35562, assign29400_e35562_d_n5, assign29400_e35562_d_n6, assign29400_e35562_d_n7, assign29400_e35562_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) && (var_guard567 != 0.0)) {
        let assign29400_e35557: f64 = (-var_ysq);
        let assign29400_e35559: f64 = (assign29400_e35557 + var_mtat);
        let assign29400_e35560: f64 = (assign29400_e35559).exp();
        (assign29400_e35560, (assign29400_e35560 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign29400_e35560 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign29400_e35560 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign29400_e35560 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29400_e35562;
        var_tmp_dn5 = assign29400_e35562_d_n5;
        var_tmp_dn6 = assign29400_e35562_d_n6;
        var_tmp_dn7 = assign29400_e35562_d_n7;
        var_tmp_dn8 = assign29400_e35562_d_n8;

        let (assign29410_e35611, assign29410_e35611_d_n5, assign29410_e35611_d_n6, assign29410_e35611_d_n7, assign29410_e35611_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) && (var_guard567 == 0.0)) {
        let assign29410_e35578: f64 = (-230.25850929940458);
        let assign29410_e35580: f64 = (-var_ysq);
        let assign29410_e35582: f64 = (assign29410_e35580 + var_mtat);
        let assign29410_e35583: f64 = (assign29410_e35578 - assign29410_e35582);
        let assign29410_e35587: f64 = (-230.25850929940458);
        let assign29410_e35589: f64 = (-var_ysq);
        let assign29410_e35591: f64 = (assign29410_e35589 + var_mtat);
        let assign29410_e35592: f64 = (assign29410_e35587 - assign29410_e35591);
        let assign29410_e35595: f64 = (-230.25850929940458);
        let assign29410_e35597: f64 = (-var_ysq);
        let assign29410_e35599: f64 = (assign29410_e35597 + var_mtat);
        let assign29410_e35600: f64 = (assign29410_e35595 - assign29410_e35599);
        let assign29410_e35602: f64 = (assign29410_e35600 * 0.3333333333333333);
        let assign29410_e35603: f64 = (1.0 + assign29410_e35602);
        let assign29410_e35604: f64 = (assign29410_e35592 * assign29410_e35603);
        let assign29410_e35605: f64 = (0.5 * assign29410_e35604);
        let assign29410_e35606: f64 = (1.0 + assign29410_e35605);
        let assign29410_e35607: f64 = (assign29410_e35583 * assign29410_e35606);
        let assign29410_e35608: f64 = (1.0 + assign29410_e35607);
        let assign29410_e35609: f64 = (1e-100 / assign29410_e35608);
        (assign29410_e35609, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign29410_e35606) + (assign29410_e35583 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign29410_e35603) + (assign29410_e35592 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign29410_e35608 * assign29410_e35608))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign29410_e35606) + (assign29410_e35583 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign29410_e35603) + (assign29410_e35592 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign29410_e35608 * assign29410_e35608))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign29410_e35606) + (assign29410_e35583 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign29410_e35603) + (assign29410_e35592 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign29410_e35608 * assign29410_e35608))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign29410_e35606) + (assign29410_e35583 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign29410_e35603) + (assign29410_e35592 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign29410_e35608 * assign29410_e35608))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29410_e35611;
        var_tmp_dn5 = assign29410_e35611_d_n5;
        var_tmp_dn6 = assign29410_e35611_d_n6;
        var_tmp_dn7 = assign29410_e35611_d_n7;
        var_tmp_dn8 = assign29410_e35611_d_n8;

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
        *var_guard561_slot = var_guard561;
        *var_guard562_slot = var_guard562;
        *var_guard563_slot = var_guard563;
        *var_guard564_slot = var_guard564;
        *var_guard565_slot = var_guard565;
        *var_guard566_slot = var_guard566;
        *var_guard567_slot = var_guard567;
        *var_id__blk219_slot = var_id__blk219;
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

    pub(super) fn stamp_transient_block_58(
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
        var_guard560: f64,
        var_guard564: f64,
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
        var_v1: f64,
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
        var_guard568_slot: &mut f64,
        var_guard569_slot: &mut f64,
        var_guard570_slot: &mut f64,
        var_guard571_slot: &mut f64,
        var_guard572_slot: &mut f64,
        var_guard573_slot: &mut f64,
        var_guard574_slot: &mut f64,
        var_guard575_slot: &mut f64,
        var_guard576_slot: &mut f64,
        var_guard577_slot: &mut f64,
        var_guard578_slot: &mut f64,
        var_guard579_slot: &mut f64,
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
        let mut var_guard568: f64 = *var_guard568_slot;
        let mut var_guard569: f64 = *var_guard569_slot;
        let mut var_guard570: f64 = *var_guard570_slot;
        let mut var_guard571: f64 = *var_guard571_slot;
        let mut var_guard572: f64 = *var_guard572_slot;
        let mut var_guard573: f64 = *var_guard573_slot;
        let mut var_guard574: f64 = *var_guard574_slot;
        let mut var_guard575: f64 = *var_guard575_slot;
        let mut var_guard576: f64 = *var_guard576_slot;
        let mut var_guard577: f64 = *var_guard577_slot;
        let mut var_guard578: f64 = *var_guard578_slot;
        let mut var_guard579: f64 = *var_guard579_slot;
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

        let (assign29420_e35641, assign29420_e35641_d_n5, assign29420_e35641_d_n6, assign29420_e35641_d_n7, assign29420_e35641_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) {
        let assign29420_e35623: f64 = (0.29214664 * var_terfc);
        let assign29420_e35627: f64 = (var_terfc * var_terfc);
        let assign29420_e35628: f64 = (var_berfc * assign29420_e35627);
        let assign29420_e35629: f64 = (assign29420_e35623 + assign29420_e35628);
        let assign29420_e35633: f64 = (var_terfc * var_terfc);
        let assign29420_e35635: f64 = (assign29420_e35633 * var_terfc);
        let assign29420_e35636: f64 = (var_cerfc * assign29420_e35635);
        let assign29420_e35637: f64 = (assign29420_e35629 + assign29420_e35636);
        let assign29420_e35639: f64 = (assign29420_e35637 * var_tmp);
        (assign29420_e35639, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign29420_e35633 * var_terfc_dn5)))) * var_tmp) + (assign29420_e35637 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign29420_e35633 * var_terfc_dn6)))) * var_tmp) + (assign29420_e35637 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign29420_e35633 * var_terfc_dn7)))) * var_tmp) + (assign29420_e35637 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign29420_e35633 * var_terfc_dn8)))) * var_tmp) + (assign29420_e35637 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign29420_e35641;
        var_erfcpos_dn5 = assign29420_e35641_d_n5;
        var_erfcpos_dn6 = assign29420_e35641_d_n6;
        var_erfcpos_dn7 = assign29420_e35641_d_n7;
        var_erfcpos_dn8 = assign29420_e35641_d_n8;

        let assign29430_e35644: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard568 = assign29430_e35644;

        let (assign29440_e35658, assign29440_e35658_d_n5, assign29440_e35658_d_n6, assign29440_e35658_d_n7, assign29440_e35658_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) && (var_guard568 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign29440_e35658;
        var_erfctimesexpmtat_dn5 = assign29440_e35658_d_n5;
        var_erfctimesexpmtat_dn6 = assign29440_e35658_d_n6;
        var_erfctimesexpmtat_dn7 = assign29440_e35658_d_n7;
        var_erfctimesexpmtat_dn8 = assign29440_e35658_d_n8;

        let assign29450_e35661: f64 = (-230.25850929940458);
        let assign29450_e35662: f64 = if var_mtat > assign29450_e35661 { 1.0 } else { 0.0 };
        var_guard569 = assign29450_e35662;

        let (assign29460_e35680, assign29460_e35680_d_n5, assign29460_e35680_d_n6, assign29460_e35680_d_n7, assign29460_e35680_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) && (var_guard568 == 0.0)) && (var_guard569 != 0.0)) {
        let assign29460_e35678: f64 = (var_mtat).exp();
        (assign29460_e35678, (assign29460_e35678 * var_mtat_dn5), (assign29460_e35678 * var_mtat_dn6), (assign29460_e35678 * var_mtat_dn7), (assign29460_e35678 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29460_e35680;
        var_tmp_dn5 = assign29460_e35680_d_n5;
        var_tmp_dn6 = assign29460_e35680_d_n6;
        var_tmp_dn7 = assign29460_e35680_d_n7;
        var_tmp_dn8 = assign29460_e35680_d_n8;

        let (assign29470_e35723, assign29470_e35723_d_n5, assign29470_e35723_d_n6, assign29470_e35723_d_n7, assign29470_e35723_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) && (var_guard568 == 0.0)) && (var_guard569 == 0.0)) {
        let assign29470_e35699: f64 = (-230.25850929940458);
        let assign29470_e35701: f64 = (assign29470_e35699 - var_mtat);
        let assign29470_e35705: f64 = (-230.25850929940458);
        let assign29470_e35707: f64 = (assign29470_e35705 - var_mtat);
        let assign29470_e35710: f64 = (-230.25850929940458);
        let assign29470_e35712: f64 = (assign29470_e35710 - var_mtat);
        let assign29470_e35714: f64 = (assign29470_e35712 * 0.3333333333333333);
        let assign29470_e35715: f64 = (1.0 + assign29470_e35714);
        let assign29470_e35716: f64 = (assign29470_e35707 * assign29470_e35715);
        let assign29470_e35717: f64 = (0.5 * assign29470_e35716);
        let assign29470_e35718: f64 = (1.0 + assign29470_e35717);
        let assign29470_e35719: f64 = (assign29470_e35701 * assign29470_e35718);
        let assign29470_e35720: f64 = (1.0 + assign29470_e35719);
        let assign29470_e35721: f64 = (1e-100 / assign29470_e35720);
        (assign29470_e35721, (-((1e-100 * (((-var_mtat_dn5) * assign29470_e35718) + (assign29470_e35701 * (0.5 * (((-var_mtat_dn5) * assign29470_e35715) + (assign29470_e35707 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign29470_e35720 * assign29470_e35720))), (-((1e-100 * (((-var_mtat_dn6) * assign29470_e35718) + (assign29470_e35701 * (0.5 * (((-var_mtat_dn6) * assign29470_e35715) + (assign29470_e35707 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign29470_e35720 * assign29470_e35720))), (-((1e-100 * (((-var_mtat_dn7) * assign29470_e35718) + (assign29470_e35701 * (0.5 * (((-var_mtat_dn7) * assign29470_e35715) + (assign29470_e35707 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign29470_e35720 * assign29470_e35720))), (-((1e-100 * (((-var_mtat_dn8) * assign29470_e35718) + (assign29470_e35701 * (0.5 * (((-var_mtat_dn8) * assign29470_e35715) + (assign29470_e35707 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign29470_e35720 * assign29470_e35720))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29470_e35723;
        var_tmp_dn5 = assign29470_e35723_d_n5;
        var_tmp_dn6 = assign29470_e35723_d_n6;
        var_tmp_dn7 = assign29470_e35723_d_n7;
        var_tmp_dn8 = assign29470_e35723_d_n8;

        let (assign29480_e35742, assign29480_e35742_d_n5, assign29480_e35742_d_n6, assign29480_e35742_d_n7, assign29480_e35742_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) && (var_guard568 == 0.0)) {
        let assign29480_e35738: f64 = (2.0 * var_tmp);
        let assign29480_e35740: f64 = (assign29480_e35738 - var_erfcpos);
        (assign29480_e35740, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign29480_e35742;
        var_erfctimesexpmtat_dn5 = assign29480_e35742_d_n5;
        var_erfctimesexpmtat_dn6 = assign29480_e35742_d_n6;
        var_erfctimesexpmtat_dn7 = assign29480_e35742_d_n7;
        var_erfctimesexpmtat_dn8 = assign29480_e35742_d_n8;

        let (assign29490_e35762, assign29490_e35762_d_n5, assign29490_e35762_d_n6, assign29490_e35762_d_n7, assign29490_e35762_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) {
        let assign29490_e35754: f64 = (1.772453850905516 * 0.5);
        let assign29490_e35757: f64 = (var_atatsti_d * var_erfctimesexpmtat);
        let assign29490_e35759: f64 = (assign29490_e35757 / var_ktat);
        let assign29490_e35760: f64 = (assign29490_e35754 * assign29490_e35759);
        (assign29490_e35760, (assign29490_e35754 * ((((var_atatsti_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign29490_e35757 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign29490_e35754 * ((((var_atatsti_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign29490_e35757 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign29490_e35754 * ((((var_atatsti_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign29490_e35757 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign29490_e35754 * ((((var_atatsti_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign29490_e35757 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign29490_e35762;
        var_gammamax_dn5 = assign29490_e35762_d_n5;
        var_gammamax_dn6 = assign29490_e35762_d_n6;
        var_gammamax_dn7 = assign29490_e35762_d_n7;
        var_gammamax_dn8 = assign29490_e35762_d_n8;

        let (assign29500_e35780, assign29500_e35780_d_n5, assign29500_e35780_d_n6, assign29500_e35780_d_n7, assign29500_e35780_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard564 == 0.0)) {
        let assign29500_e35775: f64 = (var_asrh * var_gammamax);
        let assign29500_e35777: f64 = (assign29500_e35775 * var_wtat);
        let assign29500_e35778: f64 = (var_ctatstid_i * assign29500_e35777);
        (assign29500_e35778, (var_ctatstid_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign29500_e35775 * var_wtat_dn5))), (var_ctatstid_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign29500_e35775 * var_wtat_dn6))), (var_ctatstid_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign29500_e35775 * var_wtat_dn7))), (var_ctatstid_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign29500_e35775 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign29500_e35780;
        var_itat_dn5 = assign29500_e35780_d_n5;
        var_itat_dn6 = assign29500_e35780_d_n6;
        var_itat_dn7 = assign29500_e35780_d_n7;
        var_itat_dn8 = assign29500_e35780_d_n8;

        let assign29510_e35783: f64 = if var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard570 = assign29510_e35783;

        let (assign29520_e35794, assign29520_e35794_d_n5, assign29520_e35794_d_n6, assign29520_e35794_d_n7, assign29520_e35794_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard570 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign29520_e35794;
        var_ibbt_dn5 = assign29520_e35794_d_n5;
        var_ibbt_dn6 = assign29520_e35794_d_n6;
        var_ibbt_dn7 = assign29520_e35794_d_n7;
        var_ibbt_dn8 = assign29520_e35794_d_n8;

        let assign29530_e35797: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard571 = assign29530_e35797;

        let (assign29540_e35816, assign29540_e35816_d_n5, assign29540_e35816_d_n6, assign29540_e35816_d_n7, assign29540_e35816_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard570 == 0.0)) && (var_guard571 != 0.0)) {
        let assign29540_e35811: f64 = (var_vbirstid_i - var_vbbt);
        let assign29540_e35813: f64 = (assign29540_e35811 * var_vbirstiinv_d);
        let assign29540_e35814: f64 = (assign29540_e35813).sqrt();
        (assign29540_e35814, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29540_e35816;
        var_tmp_dn5 = assign29540_e35816_d_n5;
        var_tmp_dn6 = assign29540_e35816_d_n6;
        var_tmp_dn7 = assign29540_e35816_d_n7;
        var_tmp_dn8 = assign29540_e35816_d_n8;

        let (assign29550_e35837, assign29550_e35837_d_n5, assign29550_e35837_d_n6, assign29550_e35837_d_n7, assign29550_e35837_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard570 == 0.0)) && (var_guard571 == 0.0)) {
        let assign29550_e35831: f64 = (var_vbirstid_i - var_vbbt);
        let assign29550_e35833: f64 = (assign29550_e35831 * var_vbirstiinv_d);
        let assign29550_e35835: f64 = (assign29550_e35833).powf(var_pstid_i);
        (assign29550_e35835, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29550_e35837;
        var_tmp_dn5 = assign29550_e35837_d_n5;
        var_tmp_dn6 = assign29550_e35837_d_n6;
        var_tmp_dn7 = assign29550_e35837_d_n7;
        var_tmp_dn8 = assign29550_e35837_d_n8;

        let (assign29560_e35857, assign29560_e35857_d_n5, assign29560_e35857_d_n6, assign29560_e35857_d_n7, assign29560_e35857_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard570 == 0.0)) {
        let assign29560_e35850: f64 = (var_vbirstid_i - var_vbbt);
        let assign29560_e35852: f64 = (assign29560_e35850 * var_wdepnulrinvsti_d);
        let assign29560_e35854: f64 = (assign29560_e35852 / var_tmp);
        let assign29560_e35855: f64 = (var_one_over_one_minus_psti_d * assign29560_e35854);
        (assign29560_e35855, (var_one_over_one_minus_psti_d * (-((assign29560_e35852 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign29560_e35852 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign29560_e35852 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign29560_e35852 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign29560_e35857;
        var_fmaxr_dn5 = assign29560_e35857_d_n5;
        var_fmaxr_dn6 = assign29560_e35857_d_n6;
        var_fmaxr_dn7 = assign29560_e35857_d_n7;
        var_fmaxr_dn8 = assign29560_e35857_d_n8;

        let assign29570_e35859: f64 = (-var_fbbtsti_d);
        let assign29570_e35861: f64 = (assign29570_e35859 / var_fmaxr);
        let assign29570_e35862: f64 = (assign29570_e35861).abs();
        let assign29570_e35864: f64 = if assign29570_e35862 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard572 = assign29570_e35864;

        let (assign29580_e35882, assign29580_e35882_d_n5, assign29580_e35882_d_n6, assign29580_e35882_d_n7, assign29580_e35882_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard570 == 0.0)) && (var_guard572 != 0.0)) {
        let assign29580_e35877: f64 = (-var_fbbtsti_d);
        let assign29580_e35879: f64 = (assign29580_e35877 / var_fmaxr);
        let assign29580_e35880: f64 = (assign29580_e35879).exp();
        (assign29580_e35880, (assign29580_e35880 * (-((assign29580_e35877 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign29580_e35880 * (-((assign29580_e35877 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign29580_e35880 * (-((assign29580_e35877 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign29580_e35880 * (-((assign29580_e35877 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29580_e35882;
        var_tmp_dn5 = assign29580_e35882_d_n5;
        var_tmp_dn6 = assign29580_e35882_d_n6;
        var_tmp_dn7 = assign29580_e35882_d_n7;
        var_tmp_dn8 = assign29580_e35882_d_n8;

        let assign29590_e35884: f64 = (-var_fbbtsti_d);
        let assign29590_e35886: f64 = (assign29590_e35884 / var_fmaxr);
        let assign29590_e35888: f64 = if assign29590_e35886 < 0.0 { 1.0 } else { 0.0 };
        var_guard573 = assign29590_e35888;

        let (assign29600_e35939, assign29600_e35939_d_n5, assign29600_e35939_d_n6, assign29600_e35939_d_n7, assign29600_e35939_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard570 == 0.0)) && (var_guard572 == 0.0)) && (var_guard573 != 0.0)) {
        let assign29600_e35906: f64 = (-230.25850929940458);
        let assign29600_e35908: f64 = (-var_fbbtsti_d);
        let assign29600_e35910: f64 = (assign29600_e35908 / var_fmaxr);
        let assign29600_e35911: f64 = (assign29600_e35906 - assign29600_e35910);
        let assign29600_e35915: f64 = (-230.25850929940458);
        let assign29600_e35917: f64 = (-var_fbbtsti_d);
        let assign29600_e35919: f64 = (assign29600_e35917 / var_fmaxr);
        let assign29600_e35920: f64 = (assign29600_e35915 - assign29600_e35919);
        let assign29600_e35923: f64 = (-230.25850929940458);
        let assign29600_e35925: f64 = (-var_fbbtsti_d);
        let assign29600_e35927: f64 = (assign29600_e35925 / var_fmaxr);
        let assign29600_e35928: f64 = (assign29600_e35923 - assign29600_e35927);
        let assign29600_e35930: f64 = (assign29600_e35928 * 0.3333333333333333);
        let assign29600_e35931: f64 = (1.0 + assign29600_e35930);
        let assign29600_e35932: f64 = (assign29600_e35920 * assign29600_e35931);
        let assign29600_e35933: f64 = (0.5 * assign29600_e35932);
        let assign29600_e35934: f64 = (1.0 + assign29600_e35933);
        let assign29600_e35935: f64 = (assign29600_e35911 * assign29600_e35934);
        let assign29600_e35936: f64 = (1.0 + assign29600_e35935);
        let assign29600_e35937: f64 = (1e-100 / assign29600_e35936);
        (assign29600_e35937, (-((1e-100 * (((-(-((assign29600_e35908 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign29600_e35934) + (assign29600_e35911 * (0.5 * (((-(-((assign29600_e35917 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign29600_e35931) + (assign29600_e35920 * ((-(-((assign29600_e35925 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign29600_e35936 * assign29600_e35936))), (-((1e-100 * (((-(-((assign29600_e35908 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign29600_e35934) + (assign29600_e35911 * (0.5 * (((-(-((assign29600_e35917 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign29600_e35931) + (assign29600_e35920 * ((-(-((assign29600_e35925 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign29600_e35936 * assign29600_e35936))), (-((1e-100 * (((-(-((assign29600_e35908 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign29600_e35934) + (assign29600_e35911 * (0.5 * (((-(-((assign29600_e35917 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign29600_e35931) + (assign29600_e35920 * ((-(-((assign29600_e35925 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign29600_e35936 * assign29600_e35936))), (-((1e-100 * (((-(-((assign29600_e35908 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign29600_e35934) + (assign29600_e35911 * (0.5 * (((-(-((assign29600_e35917 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign29600_e35931) + (assign29600_e35920 * ((-(-((assign29600_e35925 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign29600_e35936 * assign29600_e35936))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29600_e35939;
        var_tmp_dn5 = assign29600_e35939_d_n5;
        var_tmp_dn6 = assign29600_e35939_d_n6;
        var_tmp_dn7 = assign29600_e35939_d_n7;
        var_tmp_dn8 = assign29600_e35939_d_n8;

        let (assign29610_e35988, assign29610_e35988_d_n5, assign29610_e35988_d_n6, assign29610_e35988_d_n7, assign29610_e35988_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard570 == 0.0)) && (var_guard572 == 0.0)) && (var_guard573 == 0.0)) {
        let assign29610_e35958: f64 = (-var_fbbtsti_d);
        let assign29610_e35960: f64 = (assign29610_e35958 / var_fmaxr);
        let assign29610_e35962: f64 = (assign29610_e35960 - 230.25850929940458);
        let assign29610_e35966: f64 = (-var_fbbtsti_d);
        let assign29610_e35968: f64 = (assign29610_e35966 / var_fmaxr);
        let assign29610_e35970: f64 = (assign29610_e35968 - 230.25850929940458);
        let assign29610_e35973: f64 = (-var_fbbtsti_d);
        let assign29610_e35975: f64 = (assign29610_e35973 / var_fmaxr);
        let assign29610_e35977: f64 = (assign29610_e35975 - 230.25850929940458);
        let assign29610_e35979: f64 = (assign29610_e35977 * 0.3333333333333333);
        let assign29610_e35980: f64 = (1.0 + assign29610_e35979);
        let assign29610_e35981: f64 = (assign29610_e35970 * assign29610_e35980);
        let assign29610_e35982: f64 = (0.5 * assign29610_e35981);
        let assign29610_e35983: f64 = (1.0 + assign29610_e35982);
        let assign29610_e35984: f64 = (assign29610_e35962 * assign29610_e35983);
        let assign29610_e35985: f64 = (1.0 + assign29610_e35984);
        let assign29610_e35986: f64 = (1e100 * assign29610_e35985);
        (assign29610_e35986, (1e100 * (((-((assign29610_e35958 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign29610_e35983) + (assign29610_e35962 * (0.5 * (((-((assign29610_e35966 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign29610_e35980) + (assign29610_e35970 * ((-((assign29610_e35973 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign29610_e35958 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign29610_e35983) + (assign29610_e35962 * (0.5 * (((-((assign29610_e35966 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign29610_e35980) + (assign29610_e35970 * ((-((assign29610_e35973 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign29610_e35958 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign29610_e35983) + (assign29610_e35962 * (0.5 * (((-((assign29610_e35966 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign29610_e35980) + (assign29610_e35970 * ((-((assign29610_e35973 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign29610_e35958 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign29610_e35983) + (assign29610_e35962 * (0.5 * (((-((assign29610_e35966 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign29610_e35980) + (assign29610_e35970 * ((-((assign29610_e35973 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29610_e35988;
        var_tmp_dn5 = assign29610_e35988_d_n5;
        var_tmp_dn6 = assign29610_e35988_d_n6;
        var_tmp_dn7 = assign29610_e35988_d_n7;
        var_tmp_dn8 = assign29610_e35988_d_n8;

        let (assign29620_e36008, assign29620_e36008_d_n5, assign29620_e36008_d_n6, assign29620_e36008_d_n7, assign29620_e36008_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard570 == 0.0)) {
        let assign29620_e36001: f64 = (var_v1 * var_fmaxr);
        let assign29620_e36003: f64 = (assign29620_e36001 * var_fmaxr);
        let assign29620_e36005: f64 = (assign29620_e36003 * var_tmp);
        let assign29620_e36006: f64 = (var_cbbtstid_i * assign29620_e36005);
        (assign29620_e36006, (var_cbbtstid_i * (((((var_v1 * var_fmaxr_dn5) * var_fmaxr) + (assign29620_e36001 * var_fmaxr_dn5)) * var_tmp) + (assign29620_e36003 * var_tmp_dn5))), (var_cbbtstid_i * (((((var_v1 * var_fmaxr_dn6) * var_fmaxr) + (assign29620_e36001 * var_fmaxr_dn6)) * var_tmp) + (assign29620_e36003 * var_tmp_dn6))), (var_cbbtstid_i * (((((var_v1 * var_fmaxr_dn7) * var_fmaxr) + (assign29620_e36001 * var_fmaxr_dn7)) * var_tmp) + (assign29620_e36003 * var_tmp_dn7))), (var_cbbtstid_i * (((((var_v1 * var_fmaxr_dn8) * var_fmaxr) + (assign29620_e36001 * var_fmaxr_dn8)) * var_tmp) + (assign29620_e36003 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign29620_e36008;
        var_ibbt_dn5 = assign29620_e36008_d_n5;
        var_ibbt_dn6 = assign29620_e36008_d_n6;
        var_ibbt_dn7 = assign29620_e36008_d_n7;
        var_ibbt_dn8 = assign29620_e36008_d_n8;

        let assign29630_e36011: f64 = if var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard574 = assign29630_e36011;

        let (assign29640_e36022, assign29640_e36022_d_n5, assign29640_e36022_d_n6, assign29640_e36022_d_n7, assign29640_e36022_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard574 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign29640_e36022;
        var_fbreakdown_dn5 = assign29640_e36022_d_n5;
        var_fbreakdown_dn6 = assign29640_e36022_d_n6;
        var_fbreakdown_dn7 = assign29640_e36022_d_n7;
        var_fbreakdown_dn8 = assign29640_e36022_d_n8;

        let assign29650_e36025: f64 = (-var_alphaav);
        let assign29650_e36027: f64 = (assign29650_e36025 * var_vbrstid_i);
        let assign29650_e36028: f64 = if var_vav > assign29650_e36027 { 1.0 } else { 0.0 };
        var_guard575 = assign29650_e36028;

        let assign29660_e36031: f64 = if var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        var_guard576 = assign29660_e36031;

        let (assign29670_e36061, assign29670_e36061_d_n5, assign29670_e36061_d_n6, assign29670_e36061_d_n7, assign29670_e36061_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard574 == 0.0)) && (var_guard575 != 0.0)) && (var_guard576 != 0.0)) {
        let assign29670_e36047: f64 = (var_vav * var_vbrinvsti_d);
        let assign29670_e36050: f64 = (var_vav * var_vbrinvsti_d);
        let assign29670_e36051: f64 = (assign29670_e36047 * assign29670_e36050);
        let assign29670_e36054: f64 = (var_vav * var_vbrinvsti_d);
        let assign29670_e36055: f64 = (assign29670_e36051 * assign29670_e36054);
        let assign29670_e36058: f64 = (var_vav * var_vbrinvsti_d);
        let assign29670_e36059: f64 = (assign29670_e36055 * assign29670_e36058);
        (assign29670_e36059, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29670_e36061;
        var_tmp_dn5 = assign29670_e36061_d_n5;
        var_tmp_dn6 = assign29670_e36061_d_n6;
        var_tmp_dn7 = assign29670_e36061_d_n7;
        var_tmp_dn8 = assign29670_e36061_d_n8;

        let (assign29680_e36083, assign29680_e36083_d_n5, assign29680_e36083_d_n6, assign29680_e36083_d_n7, assign29680_e36083_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard574 == 0.0)) && (var_guard575 != 0.0)) && (var_guard576 == 0.0)) {
        let assign29680_e36078: f64 = (var_vav * var_vbrinvsti_d);
        let assign29680_e36079: f64 = (assign29680_e36078).abs();
        let assign29680_e36081: f64 = (assign29680_e36079).powf(var_pbrstid_i);
        (assign29680_e36081, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29680_e36083;
        var_tmp_dn5 = assign29680_e36083_d_n5;
        var_tmp_dn6 = assign29680_e36083_d_n6;
        var_tmp_dn7 = assign29680_e36083_d_n7;
        var_tmp_dn8 = assign29680_e36083_d_n8;

        let (assign29690_e36101, assign29690_e36101_d_n5, assign29690_e36101_d_n6, assign29690_e36101_d_n7, assign29690_e36101_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard574 == 0.0)) && (var_guard575 != 0.0)) {
        let assign29690_e36098: f64 = (1.0 - var_tmp);
        let assign29690_e36099: f64 = (1.0 / assign29690_e36098);
        (assign29690_e36099, (-((-var_tmp_dn5) / (assign29690_e36098 * assign29690_e36098))), (-((-var_tmp_dn6) / (assign29690_e36098 * assign29690_e36098))), (-((-var_tmp_dn7) / (assign29690_e36098 * assign29690_e36098))), (-((-var_tmp_dn8) / (assign29690_e36098 * assign29690_e36098))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign29690_e36101;
        var_fbreakdown_dn5 = assign29690_e36101_d_n5;
        var_fbreakdown_dn6 = assign29690_e36101_d_n6;
        var_fbreakdown_dn7 = assign29690_e36101_d_n7;
        var_fbreakdown_dn8 = assign29690_e36101_d_n8;

        let (assign29700_e36124, assign29700_e36124_d_n5, assign29700_e36124_d_n6, assign29700_e36124_d_n7, assign29700_e36124_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) && (var_guard574 == 0.0)) && (var_guard575 == 0.0)) {
        let assign29700_e36118: f64 = (var_alphaav * var_vbrstid_i);
        let assign29700_e36119: f64 = (var_vav + assign29700_e36118);
        let assign29700_e36121: f64 = (assign29700_e36119 * var_slopesti_d);
        let assign29700_e36122: f64 = (var_fstopsti_d + assign29700_e36121);
        (assign29700_e36122, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign29700_e36124;
        var_fbreakdown_dn5 = assign29700_e36124_d_n5;
        var_fbreakdown_dn6 = assign29700_e36124_d_n6;
        var_fbreakdown_dn7 = assign29700_e36124_d_n7;
        var_fbreakdown_dn8 = assign29700_e36124_d_n8;

        let (assign29710_e36143, assign29710_e36143_d_n5, assign29710_e36143_d_n6, assign29710_e36143_d_n7, assign29710_e36143_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard560 == 0.0)) {
        let assign29710_e36134: f64 = (var_id__blk219 + var_isrh);
        let assign29710_e36136: f64 = (assign29710_e36134 + var_itat);
        let assign29710_e36138: f64 = (assign29710_e36136 + var_ibbt);
        let assign29710_e36139: f64 = (p.p29 * assign29710_e36138);
        let assign29710_e36141: f64 = (assign29710_e36139 * var_fbreakdown);
        (assign29710_e36141, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign29710_e36139 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign29710_e36139 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign29710_e36139 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign29710_e36139 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign29710_e36143;
        var_ijunsti_dn5 = assign29710_e36143_d_n5;
        var_ijunsti_dn6 = assign29710_e36143_d_n6;
        var_ijunsti_dn7 = assign29710_e36143_d_n7;
        var_ijunsti_dn8 = assign29710_e36143_d_n8;

        let assign29720_e36146: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard577 = assign29720_e36146;

        let (assign29730_e36154, assign29730_e36154_d_n5, assign29730_e36154_d_n6, assign29730_e36154_d_n7, assign29730_e36154_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign29730_e36154;
        var_ijungat_dn5 = assign29730_e36154_d_n5;
        var_ijungat_dn6 = assign29730_e36154_d_n6;
        var_ijungat_dn7 = assign29730_e36154_d_n7;
        var_ijungat_dn8 = assign29730_e36154_d_n8;

        let (assign29740_e36165,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) {
        let assign29740_e36163: f64 = (var_idsatgat_d * var_idmult);
        (assign29740_e36163,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign29740_e36165;

        let assign29750_e36172: f64 = if ((var_csrhgatd_i == 0.0) && (var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard578 = assign29750_e36172;

        let (assign29760_e36183, assign29760_e36183_d_n5, assign29760_e36183_d_n6, assign29760_e36183_d_n7, assign29760_e36183_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard578 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign29760_e36183;
        var_isrh_dn5 = assign29760_e36183_d_n5;
        var_isrh_dn6 = assign29760_e36183_d_n6;
        var_isrh_dn7 = assign29760_e36183_d_n7;
        var_isrh_dn8 = assign29760_e36183_d_n8;

        let (assign29770_e36197,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard578 == 0.0)) {
        let assign29770_e36195: f64 = (var_vbigat_d - var_vjsrh);
        (assign29770_e36195,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign29770_e36197;

        let (assign29780_e36216,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard578 == 0.0)) {
        let assign29780_e36211: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign29780_e36212: f64 = (1.0 - assign29780_e36211);
        let assign29780_e36213: f64 = (assign29780_e36212).sqrt();
        let assign29780_e36214: f64 = (1.0 - assign29780_e36213);
        (assign29780_e36214,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign29780_e36216;

        let assign29790_e36219: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard579 = assign29790_e36219;

        let (assign29800_e36233,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard578 == 0.0)) && (var_guard579 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign29800_e36233;

        let (assign29810_e36265,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard578 == 0.0)) && (var_guard579 == 0.0)) {
        let assign29810_e36248: f64 = (var_wsrhstep * var_wsrhstep);
        let assign29810_e36250: f64 = (var_wsrhstep).ln();
        let assign29810_e36251: f64 = (assign29810_e36248 * assign29810_e36250);
        let assign29810_e36254: f64 = (1.0 - var_wsrhstep);
        let assign29810_e36255: f64 = (assign29810_e36251 / assign29810_e36254);
        let assign29810_e36257: f64 = (assign29810_e36255 + var_wsrhstep);
        let assign29810_e36261: f64 = (2.0 * var_pgatd_i);
        let assign29810_e36262: f64 = (1.0 - assign29810_e36261);
        let assign29810_e36263: f64 = (assign29810_e36257 * assign29810_e36262);
        (assign29810_e36263,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign29810_e36265;

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
        *var_guard568_slot = var_guard568;
        *var_guard569_slot = var_guard569;
        *var_guard570_slot = var_guard570;
        *var_guard571_slot = var_guard571;
        *var_guard572_slot = var_guard572;
        *var_guard573_slot = var_guard573;
        *var_guard574_slot = var_guard574;
        *var_guard575_slot = var_guard575;
        *var_guard576_slot = var_guard576;
        *var_guard577_slot = var_guard577;
        *var_guard578_slot = var_guard578;
        *var_guard579_slot = var_guard579;
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

    pub(super) fn stamp_transient_block_59(
        var_atatgat_d: f64,
        var_berfc: f64,
        var_btatpartgat_d: f64,
        var_cerfc: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_dwsrh: f64,
        var_ftdgat_d: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard577: f64,
        var_guard578: f64,
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
        var_guard580_slot: &mut f64,
        var_guard581_slot: &mut f64,
        var_guard582_slot: &mut f64,
        var_guard583_slot: &mut f64,
        var_guard584_slot: &mut f64,
        var_guard585_slot: &mut f64,
        var_guard586_slot: &mut f64,
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
        let mut var_guard580: f64 = *var_guard580_slot;
        let mut var_guard581: f64 = *var_guard581_slot;
        let mut var_guard582: f64 = *var_guard582_slot;
        let mut var_guard583: f64 = *var_guard583_slot;
        let mut var_guard584: f64 = *var_guard584_slot;
        let mut var_guard585: f64 = *var_guard585_slot;
        let mut var_guard586: f64 = *var_guard586_slot;
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

        let (assign29820_e36279,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard578 == 0.0)) {
        let assign29820_e36277: f64 = (var_wsrhstep + var_dwsrh);
        (assign29820_e36277,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign29820_e36279;

        let assign29830_e36282: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard580 = assign29830_e36282;

        let (assign29840_e36299, assign29840_e36299_d_n5, assign29840_e36299_d_n6, assign29840_e36299_d_n7, assign29840_e36299_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard578 == 0.0)) && (var_guard580 != 0.0)) {
        let assign29840_e36296: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign29840_e36297: f64 = (assign29840_e36296).sqrt();
        (assign29840_e36297, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29840_e36299;
        var_tmp_dn5 = assign29840_e36299_d_n5;
        var_tmp_dn6 = assign29840_e36299_d_n6;
        var_tmp_dn7 = assign29840_e36299_d_n7;
        var_tmp_dn8 = assign29840_e36299_d_n8;

        let (assign29850_e36318, assign29850_e36318_d_n5, assign29850_e36318_d_n6, assign29850_e36318_d_n7, assign29850_e36318_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard578 == 0.0)) && (var_guard580 == 0.0)) {
        let assign29850_e36314: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign29850_e36316: f64 = (assign29850_e36314).powf(var_pgatd_i);
        (assign29850_e36316, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign29850_e36318;
        var_tmp_dn5 = assign29850_e36318_d_n5;
        var_tmp_dn6 = assign29850_e36318_d_n6;
        var_tmp_dn7 = assign29850_e36318_d_n7;
        var_tmp_dn8 = assign29850_e36318_d_n8;

        let (assign29860_e36332, assign29860_e36332_d_n5, assign29860_e36332_d_n6, assign29860_e36332_d_n7, assign29860_e36332_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard578 == 0.0)) {
        let assign29860_e36330: f64 = (var_wdepnulrgat_d * var_tmp);
        (assign29860_e36330, (var_wdepnulrgat_d * var_tmp_dn5), (var_wdepnulrgat_d * var_tmp_dn6), (var_wdepnulrgat_d * var_tmp_dn7), (var_wdepnulrgat_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign29860_e36332;
        var_wdep_dn5 = assign29860_e36332_d_n5;
        var_wdep_dn6 = assign29860_e36332_d_n6;
        var_wdep_dn7 = assign29860_e36332_d_n7;
        var_wdep_dn8 = assign29860_e36332_d_n8;

        let (assign29870_e36350, assign29870_e36350_d_n5, assign29870_e36350_d_n6, assign29870_e36350_d_n7, assign29870_e36350_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard578 == 0.0)) {
        let assign29870_e36345: f64 = (var_zinv - 1.0);
        let assign29870_e36347: f64 = (assign29870_e36345 * var_wdep);
        let assign29870_e36348: f64 = (var_ftdgat_d * assign29870_e36347);
        (assign29870_e36348, (var_ftdgat_d * (assign29870_e36345 * var_wdep_dn5)), (var_ftdgat_d * (assign29870_e36345 * var_wdep_dn6)), (var_ftdgat_d * (assign29870_e36345 * var_wdep_dn7)), (var_ftdgat_d * (assign29870_e36345 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign29870_e36350;
        var_asrh_dn5 = assign29870_e36350_d_n5;
        var_asrh_dn6 = assign29870_e36350_d_n6;
        var_asrh_dn7 = assign29870_e36350_d_n7;
        var_asrh_dn8 = assign29870_e36350_d_n8;

        let (assign29880_e36366, assign29880_e36366_d_n5, assign29880_e36366_d_n6, assign29880_e36366_d_n7, assign29880_e36366_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard578 == 0.0)) {
        let assign29880_e36363: f64 = (var_asrh * var_wsrh);
        let assign29880_e36364: f64 = (var_csrhgatd_i * assign29880_e36363);
        (assign29880_e36364, (var_csrhgatd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign29880_e36366;
        var_isrh_dn5 = assign29880_e36366_d_n5;
        var_isrh_dn6 = assign29880_e36366_d_n6;
        var_isrh_dn7 = assign29880_e36366_d_n7;
        var_isrh_dn8 = assign29880_e36366_d_n8;

        let assign29890_e36369: f64 = if var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard581 = assign29890_e36369;

        let (assign29900_e36380, assign29900_e36380_d_n5, assign29900_e36380_d_n6, assign29900_e36380_d_n7, assign29900_e36380_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign29900_e36380;
        var_itat_dn5 = assign29900_e36380_d_n5;
        var_itat_dn6 = assign29900_e36380_d_n6;
        var_itat_dn7 = assign29900_e36380_d_n7;
        var_itat_dn8 = assign29900_e36380_d_n8;

        let (assign29910_e36398, assign29910_e36398_d_n5, assign29910_e36398_d_n6, assign29910_e36398_d_n7, assign29910_e36398_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) {
        let assign29910_e36393: f64 = (var_wdep * var_one_minus_pgat_d);
        let assign29910_e36395: f64 = (assign29910_e36393 / var_vbi_minus_vjsrh);
        let assign29910_e36396: f64 = (var_btatpartgat_d * assign29910_e36395);
        (assign29910_e36396, (var_btatpartgat_d * ((var_wdep_dn5 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn6 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn7 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn8 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign29910_e36398;
        var_btat_dn5 = assign29910_e36398_d_n5;
        var_btat_dn6 = assign29910_e36398_d_n6;
        var_btat_dn7 = assign29910_e36398_d_n7;
        var_btat_dn8 = assign29910_e36398_d_n8;

        let (assign29920_e36414, assign29920_e36414_d_n5, assign29920_e36414_d_n6, assign29920_e36414_d_n7, assign29920_e36414_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) {
        let assign29920_e36410: f64 = (0.666666666666667 * var_atatgat_d);
        let assign29920_e36412: f64 = (assign29920_e36410 / var_btat);
        (assign29920_e36412, (-((assign29920_e36410 * var_btat_dn5) / (var_btat * var_btat))), (-((assign29920_e36410 * var_btat_dn6) / (var_btat * var_btat))), (-((assign29920_e36410 * var_btat_dn7) / (var_btat * var_btat))), (-((assign29920_e36410 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign29920_e36414;
        var_twoatatoverthreebtat_dn5 = assign29920_e36414_d_n5;
        var_twoatatoverthreebtat_dn6 = assign29920_e36414_d_n6;
        var_twoatatoverthreebtat_dn7 = assign29920_e36414_d_n7;
        var_twoatatoverthreebtat_dn8 = assign29920_e36414_d_n8;

        let (assign29930_e36428, assign29930_e36428_d_n5, assign29930_e36428_d_n6, assign29930_e36428_d_n7, assign29930_e36428_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) {
        let assign29930_e36426: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign29930_e36426, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign29930_e36428;
        var_umaxbeforelimiting_dn5 = assign29930_e36428_d_n5;
        var_umaxbeforelimiting_dn6 = assign29930_e36428_d_n6;
        var_umaxbeforelimiting_dn7 = assign29930_e36428_d_n7;
        var_umaxbeforelimiting_dn8 = assign29930_e36428_d_n8;

        let (assign29940_e36449, assign29940_e36449_d_n5, assign29940_e36449_d_n6, assign29940_e36449_d_n7, assign29940_e36449_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) {
        let assign29940_e36440: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign29940_e36443: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign29940_e36445: f64 = (assign29940_e36443 + 1.0);
        let assign29940_e36446: f64 = (assign29940_e36440 / assign29940_e36445);
        let assign29940_e36447: f64 = (assign29940_e36446).sqrt();
        (assign29940_e36447, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign29940_e36445) - (assign29940_e36440 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign29940_e36445 * assign29940_e36445)) / (2.0 * assign29940_e36447)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign29940_e36445) - (assign29940_e36440 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign29940_e36445 * assign29940_e36445)) / (2.0 * assign29940_e36447)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign29940_e36445) - (assign29940_e36440 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign29940_e36445 * assign29940_e36445)) / (2.0 * assign29940_e36447)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign29940_e36445) - (assign29940_e36440 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign29940_e36445 * assign29940_e36445)) / (2.0 * assign29940_e36447)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign29940_e36449;
        var_umax_dn5 = assign29940_e36449_d_n5;
        var_umax_dn6 = assign29940_e36449_d_n6;
        var_umax_dn7 = assign29940_e36449_d_n7;
        var_umax_dn8 = assign29940_e36449_d_n8;

        let (assign29950_e36462, assign29950_e36462_d_n5, assign29950_e36462_d_n6, assign29950_e36462_d_n7, assign29950_e36462_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) {
        let assign29950_e36460: f64 = (var_umax).sqrt();
        (assign29950_e36460, (var_umax_dn5 / (2.0 * assign29950_e36460)), (var_umax_dn6 / (2.0 * assign29950_e36460)), (var_umax_dn7 / (2.0 * assign29950_e36460)), (var_umax_dn8 / (2.0 * assign29950_e36460)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign29950_e36462;
        var_sqrtumax_dn5 = assign29950_e36462_d_n5;
        var_sqrtumax_dn6 = assign29950_e36462_d_n6;
        var_sqrtumax_dn7 = assign29950_e36462_d_n7;
        var_sqrtumax_dn8 = assign29950_e36462_d_n8;

        let (assign29960_e36476, assign29960_e36476_d_n5, assign29960_e36476_d_n6, assign29960_e36476_d_n7, assign29960_e36476_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) {
        let assign29960_e36474: f64 = (var_umax * var_sqrtumax);
        (assign29960_e36474, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign29960_e36476;
        var_umaxpoweronepointfive_dn5 = assign29960_e36476_d_n5;
        var_umaxpoweronepointfive_dn6 = assign29960_e36476_d_n6;
        var_umaxpoweronepointfive_dn7 = assign29960_e36476_d_n7;
        var_umaxpoweronepointfive_dn8 = assign29960_e36476_d_n8;

        let assign29970_e36478: f64 = (-var_pgatd_i);
        let assign29970_e36480: f64 = (assign29970_e36478 * var_one_over_one_minus_pgat_d);
        let assign29970_e36482: f64 = (-1.0);
        let assign29970_e36483: f64 = if assign29970_e36480 == assign29970_e36482 { 1.0 } else { 0.0 };
        var_guard582 = assign29970_e36483;

        let (assign29980_e36503, assign29980_e36503_d_n5, assign29980_e36503_d_n6, assign29980_e36503_d_n7, assign29980_e36503_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) && (var_guard582 != 0.0)) {
        let assign29980_e36499: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign29980_e36500: f64 = (1.0 + assign29980_e36499);
        let assign29980_e36501: f64 = (1.0 / assign29980_e36500);
        (assign29980_e36501, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign29980_e36500 * assign29980_e36500))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign29980_e36500 * assign29980_e36500))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign29980_e36500 * assign29980_e36500))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign29980_e36500 * assign29980_e36500))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign29980_e36503;
        var_wgamma_dn5 = assign29980_e36503_d_n5;
        var_wgamma_dn6 = assign29980_e36503_d_n6;
        var_wgamma_dn7 = assign29980_e36503_d_n7;
        var_wgamma_dn8 = assign29980_e36503_d_n8;

        let (assign29990_e36527, assign29990_e36527_d_n5, assign29990_e36527_d_n6, assign29990_e36527_d_n7, assign29990_e36527_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) && (var_guard582 == 0.0)) {
        let assign29990_e36519: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign29990_e36520: f64 = (1.0 + assign29990_e36519);
        let assign29990_e36522: f64 = (-var_pgatd_i);
        let assign29990_e36524: f64 = (assign29990_e36522 * var_one_over_one_minus_pgat_d);
        let assign29990_e36525: f64 = (assign29990_e36520).powf(assign29990_e36524);
        (assign29990_e36525, if 0.0 == 0.0 && ((assign29990_e36524) as f64).is_finite() && ((assign29990_e36524) as f64).fract() == 0.0 { if assign29990_e36524 == 0.0 { 0.0 } else { (assign29990_e36524 * ((assign29990_e36520).powf(assign29990_e36524 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign29990_e36525 * (assign29990_e36524 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign29990_e36520))) }, if 0.0 == 0.0 && ((assign29990_e36524) as f64).is_finite() && ((assign29990_e36524) as f64).fract() == 0.0 { if assign29990_e36524 == 0.0 { 0.0 } else { (assign29990_e36524 * ((assign29990_e36520).powf(assign29990_e36524 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign29990_e36525 * (assign29990_e36524 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign29990_e36520))) }, if 0.0 == 0.0 && ((assign29990_e36524) as f64).is_finite() && ((assign29990_e36524) as f64).fract() == 0.0 { if assign29990_e36524 == 0.0 { 0.0 } else { (assign29990_e36524 * ((assign29990_e36520).powf(assign29990_e36524 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign29990_e36525 * (assign29990_e36524 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign29990_e36520))) }, if 0.0 == 0.0 && ((assign29990_e36524) as f64).is_finite() && ((assign29990_e36524) as f64).fract() == 0.0 { if assign29990_e36524 == 0.0 { 0.0 } else { (assign29990_e36524 * ((assign29990_e36520).powf(assign29990_e36524 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign29990_e36525 * (assign29990_e36524 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign29990_e36520))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign29990_e36527;
        var_wgamma_dn5 = assign29990_e36527_d_n5;
        var_wgamma_dn6 = assign29990_e36527_d_n6;
        var_wgamma_dn7 = assign29990_e36527_d_n7;
        var_wgamma_dn8 = assign29990_e36527_d_n8;

        let (assign30000_e36545, assign30000_e36545_d_n5, assign30000_e36545_d_n6, assign30000_e36545_d_n7, assign30000_e36545_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) {
        let assign30000_e36539: f64 = (var_wsrh * var_wgamma);
        let assign30000_e36542: f64 = (var_wsrh + var_wgamma);
        let assign30000_e36543: f64 = (assign30000_e36539 / assign30000_e36542);
        (assign30000_e36543, ((((var_wsrh * var_wgamma_dn5) * assign30000_e36542) - (assign30000_e36539 * var_wgamma_dn5)) / (assign30000_e36542 * assign30000_e36542)), ((((var_wsrh * var_wgamma_dn6) * assign30000_e36542) - (assign30000_e36539 * var_wgamma_dn6)) / (assign30000_e36542 * assign30000_e36542)), ((((var_wsrh * var_wgamma_dn7) * assign30000_e36542) - (assign30000_e36539 * var_wgamma_dn7)) / (assign30000_e36542 * assign30000_e36542)), ((((var_wsrh * var_wgamma_dn8) * assign30000_e36542) - (assign30000_e36539 * var_wgamma_dn8)) / (assign30000_e36542 * assign30000_e36542)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign30000_e36545;
        var_wtat_dn5 = assign30000_e36545_d_n5;
        var_wtat_dn6 = assign30000_e36545_d_n6;
        var_wtat_dn7 = assign30000_e36545_d_n7;
        var_wtat_dn8 = assign30000_e36545_d_n8;

        let (assign30010_e36562, assign30010_e36562_d_n5, assign30010_e36562_d_n6, assign30010_e36562_d_n7, assign30010_e36562_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) {
        let assign30010_e36558: f64 = (var_btat / var_sqrtumax);
        let assign30010_e36559: f64 = (0.375 * assign30010_e36558);
        let assign30010_e36560: f64 = (assign30010_e36559).sqrt();
        (assign30010_e36560, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign30010_e36560)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign30010_e36560)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign30010_e36560)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign30010_e36560)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign30010_e36562;
        var_ktat_dn5 = assign30010_e36562_d_n5;
        var_ktat_dn6 = assign30010_e36562_d_n6;
        var_ktat_dn7 = assign30010_e36562_d_n7;
        var_ktat_dn8 = assign30010_e36562_d_n8;

        let (assign30020_e36580, assign30020_e36580_d_n5, assign30020_e36580_d_n6, assign30020_e36580_d_n7, assign30020_e36580_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) {
        let assign30020_e36575: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign30020_e36576: f64 = (2.0 * assign30020_e36575);
        let assign30020_e36578: f64 = (assign30020_e36576 - var_umax);
        (assign30020_e36578, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign30020_e36580;
        var_ltat_dn5 = assign30020_e36580_d_n5;
        var_ltat_dn6 = assign30020_e36580_d_n6;
        var_ltat_dn7 = assign30020_e36580_d_n7;
        var_ltat_dn8 = assign30020_e36580_d_n8;

        let (assign30030_e36606, assign30030_e36606_d_n5, assign30030_e36606_d_n6, assign30030_e36606_d_n7, assign30030_e36606_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) {
        let assign30030_e36592: f64 = (var_atatgat_d * var_twoatatoverthreebtat);
        let assign30030_e36594: f64 = (assign30030_e36592 * var_sqrtumax);
        let assign30030_e36597: f64 = (var_atatgat_d * var_umax);
        let assign30030_e36598: f64 = (assign30030_e36594 - assign30030_e36597);
        let assign30030_e36602: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign30030_e36603: f64 = (0.5 * assign30030_e36602);
        let assign30030_e36604: f64 = (assign30030_e36598 + assign30030_e36603);
        (assign30030_e36604, (((((var_atatgat_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign30030_e36592 * var_sqrtumax_dn5)) - (var_atatgat_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign30030_e36592 * var_sqrtumax_dn6)) - (var_atatgat_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign30030_e36592 * var_sqrtumax_dn7)) - (var_atatgat_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign30030_e36592 * var_sqrtumax_dn8)) - (var_atatgat_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign30030_e36606;
        var_mtat_dn5 = assign30030_e36606_d_n5;
        var_mtat_dn6 = assign30030_e36606_d_n6;
        var_mtat_dn7 = assign30030_e36606_d_n7;
        var_mtat_dn8 = assign30030_e36606_d_n8;

        let (assign30040_e36622, assign30040_e36622_d_n5, assign30040_e36622_d_n6, assign30040_e36622_d_n7, assign30040_e36622_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) {
        let assign30040_e36618: f64 = (var_ltat - 1.0);
        let assign30040_e36620: f64 = (assign30040_e36618 * var_ktat);
        (assign30040_e36620, ((var_ltat_dn5 * var_ktat) + (assign30040_e36618 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign30040_e36618 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign30040_e36618 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign30040_e36618 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign30040_e36622;
        var_xerfc_dn5 = assign30040_e36622_d_n5;
        var_xerfc_dn6 = assign30040_e36622_d_n6;
        var_xerfc_dn7 = assign30040_e36622_d_n7;
        var_xerfc_dn8 = assign30040_e36622_d_n8;

        let (assign30050_e36636, assign30050_e36636_d_n5, assign30050_e36636_d_n6, assign30050_e36636_d_n7, assign30050_e36636_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) {
        let assign30050_e36634: f64 = (var_xerfc * var_xerfc);
        (assign30050_e36634, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign30050_e36636;
        var_ysq_dn5 = assign30050_e36636_d_n5;
        var_ysq_dn6 = assign30050_e36636_d_n6;
        var_ysq_dn7 = assign30050_e36636_d_n7;
        var_ysq_dn8 = assign30050_e36636_d_n8;

        let assign30060_e36639: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard583 = assign30060_e36639;

        let (assign30070_e36659, assign30070_e36659_d_n5, assign30070_e36659_d_n6, assign30070_e36659_d_n7, assign30070_e36659_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) && (var_guard583 != 0.0)) {
        let assign30070_e36655: f64 = (var_perfc * var_xerfc);
        let assign30070_e36656: f64 = (1.0 + assign30070_e36655);
        let assign30070_e36657: f64 = (1.0 / assign30070_e36656);
        (assign30070_e36657, (-((var_perfc * var_xerfc_dn5) / (assign30070_e36656 * assign30070_e36656))), (-((var_perfc * var_xerfc_dn6) / (assign30070_e36656 * assign30070_e36656))), (-((var_perfc * var_xerfc_dn7) / (assign30070_e36656 * assign30070_e36656))), (-((var_perfc * var_xerfc_dn8) / (assign30070_e36656 * assign30070_e36656))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign30070_e36659;
        var_terfc_dn5 = assign30070_e36659_d_n5;
        var_terfc_dn6 = assign30070_e36659_d_n6;
        var_terfc_dn7 = assign30070_e36659_d_n7;
        var_terfc_dn8 = assign30070_e36659_d_n8;

        let (assign30080_e36680, assign30080_e36680_d_n5, assign30080_e36680_d_n6, assign30080_e36680_d_n7, assign30080_e36680_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) && (var_guard583 == 0.0)) {
        let assign30080_e36676: f64 = (var_perfc * var_xerfc);
        let assign30080_e36677: f64 = (1.0 - assign30080_e36676);
        let assign30080_e36678: f64 = (1.0 / assign30080_e36677);
        (assign30080_e36678, (-((-(var_perfc * var_xerfc_dn5)) / (assign30080_e36677 * assign30080_e36677))), (-((-(var_perfc * var_xerfc_dn6)) / (assign30080_e36677 * assign30080_e36677))), (-((-(var_perfc * var_xerfc_dn7)) / (assign30080_e36677 * assign30080_e36677))), (-((-(var_perfc * var_xerfc_dn8)) / (assign30080_e36677 * assign30080_e36677))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign30080_e36680;
        var_terfc_dn5 = assign30080_e36680_d_n5;
        var_terfc_dn6 = assign30080_e36680_d_n6;
        var_terfc_dn7 = assign30080_e36680_d_n7;
        var_terfc_dn8 = assign30080_e36680_d_n8;

        let assign30090_e36682: f64 = (-var_ysq);
        let assign30090_e36684: f64 = (assign30090_e36682 + var_mtat);
        let assign30090_e36686: f64 = (-230.25850929940458);
        let assign30090_e36687: f64 = if assign30090_e36684 > assign30090_e36686 { 1.0 } else { 0.0 };
        var_guard584 = assign30090_e36687;

        let (assign30100_e36705, assign30100_e36705_d_n5, assign30100_e36705_d_n6, assign30100_e36705_d_n7, assign30100_e36705_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) && (var_guard584 != 0.0)) {
        let assign30100_e36700: f64 = (-var_ysq);
        let assign30100_e36702: f64 = (assign30100_e36700 + var_mtat);
        let assign30100_e36703: f64 = (assign30100_e36702).exp();
        (assign30100_e36703, (assign30100_e36703 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign30100_e36703 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign30100_e36703 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign30100_e36703 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30100_e36705;
        var_tmp_dn5 = assign30100_e36705_d_n5;
        var_tmp_dn6 = assign30100_e36705_d_n6;
        var_tmp_dn7 = assign30100_e36705_d_n7;
        var_tmp_dn8 = assign30100_e36705_d_n8;

        let (assign30110_e36754, assign30110_e36754_d_n5, assign30110_e36754_d_n6, assign30110_e36754_d_n7, assign30110_e36754_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) && (var_guard584 == 0.0)) {
        let assign30110_e36721: f64 = (-230.25850929940458);
        let assign30110_e36723: f64 = (-var_ysq);
        let assign30110_e36725: f64 = (assign30110_e36723 + var_mtat);
        let assign30110_e36726: f64 = (assign30110_e36721 - assign30110_e36725);
        let assign30110_e36730: f64 = (-230.25850929940458);
        let assign30110_e36732: f64 = (-var_ysq);
        let assign30110_e36734: f64 = (assign30110_e36732 + var_mtat);
        let assign30110_e36735: f64 = (assign30110_e36730 - assign30110_e36734);
        let assign30110_e36738: f64 = (-230.25850929940458);
        let assign30110_e36740: f64 = (-var_ysq);
        let assign30110_e36742: f64 = (assign30110_e36740 + var_mtat);
        let assign30110_e36743: f64 = (assign30110_e36738 - assign30110_e36742);
        let assign30110_e36745: f64 = (assign30110_e36743 * 0.3333333333333333);
        let assign30110_e36746: f64 = (1.0 + assign30110_e36745);
        let assign30110_e36747: f64 = (assign30110_e36735 * assign30110_e36746);
        let assign30110_e36748: f64 = (0.5 * assign30110_e36747);
        let assign30110_e36749: f64 = (1.0 + assign30110_e36748);
        let assign30110_e36750: f64 = (assign30110_e36726 * assign30110_e36749);
        let assign30110_e36751: f64 = (1.0 + assign30110_e36750);
        let assign30110_e36752: f64 = (1e-100 / assign30110_e36751);
        (assign30110_e36752, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign30110_e36749) + (assign30110_e36726 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign30110_e36746) + (assign30110_e36735 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign30110_e36751 * assign30110_e36751))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign30110_e36749) + (assign30110_e36726 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign30110_e36746) + (assign30110_e36735 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign30110_e36751 * assign30110_e36751))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign30110_e36749) + (assign30110_e36726 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign30110_e36746) + (assign30110_e36735 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign30110_e36751 * assign30110_e36751))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign30110_e36749) + (assign30110_e36726 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign30110_e36746) + (assign30110_e36735 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign30110_e36751 * assign30110_e36751))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30110_e36754;
        var_tmp_dn5 = assign30110_e36754_d_n5;
        var_tmp_dn6 = assign30110_e36754_d_n6;
        var_tmp_dn7 = assign30110_e36754_d_n7;
        var_tmp_dn8 = assign30110_e36754_d_n8;

        let (assign30120_e36784, assign30120_e36784_d_n5, assign30120_e36784_d_n6, assign30120_e36784_d_n7, assign30120_e36784_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) {
        let assign30120_e36766: f64 = (0.29214664 * var_terfc);
        let assign30120_e36770: f64 = (var_terfc * var_terfc);
        let assign30120_e36771: f64 = (var_berfc * assign30120_e36770);
        let assign30120_e36772: f64 = (assign30120_e36766 + assign30120_e36771);
        let assign30120_e36776: f64 = (var_terfc * var_terfc);
        let assign30120_e36778: f64 = (assign30120_e36776 * var_terfc);
        let assign30120_e36779: f64 = (var_cerfc * assign30120_e36778);
        let assign30120_e36780: f64 = (assign30120_e36772 + assign30120_e36779);
        let assign30120_e36782: f64 = (assign30120_e36780 * var_tmp);
        (assign30120_e36782, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign30120_e36776 * var_terfc_dn5)))) * var_tmp) + (assign30120_e36780 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign30120_e36776 * var_terfc_dn6)))) * var_tmp) + (assign30120_e36780 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign30120_e36776 * var_terfc_dn7)))) * var_tmp) + (assign30120_e36780 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign30120_e36776 * var_terfc_dn8)))) * var_tmp) + (assign30120_e36780 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign30120_e36784;
        var_erfcpos_dn5 = assign30120_e36784_d_n5;
        var_erfcpos_dn6 = assign30120_e36784_d_n6;
        var_erfcpos_dn7 = assign30120_e36784_d_n7;
        var_erfcpos_dn8 = assign30120_e36784_d_n8;

        let assign30130_e36787: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard585 = assign30130_e36787;

        let (assign30140_e36801, assign30140_e36801_d_n5, assign30140_e36801_d_n6, assign30140_e36801_d_n7, assign30140_e36801_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) && (var_guard585 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign30140_e36801;
        var_erfctimesexpmtat_dn5 = assign30140_e36801_d_n5;
        var_erfctimesexpmtat_dn6 = assign30140_e36801_d_n6;
        var_erfctimesexpmtat_dn7 = assign30140_e36801_d_n7;
        var_erfctimesexpmtat_dn8 = assign30140_e36801_d_n8;

        let assign30150_e36804: f64 = (-230.25850929940458);
        let assign30150_e36805: f64 = if var_mtat > assign30150_e36804 { 1.0 } else { 0.0 };
        var_guard586 = assign30150_e36805;

        let (assign30160_e36823, assign30160_e36823_d_n5, assign30160_e36823_d_n6, assign30160_e36823_d_n7, assign30160_e36823_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) && (var_guard585 == 0.0)) && (var_guard586 != 0.0)) {
        let assign30160_e36821: f64 = (var_mtat).exp();
        (assign30160_e36821, (assign30160_e36821 * var_mtat_dn5), (assign30160_e36821 * var_mtat_dn6), (assign30160_e36821 * var_mtat_dn7), (assign30160_e36821 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30160_e36823;
        var_tmp_dn5 = assign30160_e36823_d_n5;
        var_tmp_dn6 = assign30160_e36823_d_n6;
        var_tmp_dn7 = assign30160_e36823_d_n7;
        var_tmp_dn8 = assign30160_e36823_d_n8;

        let (assign30170_e36866, assign30170_e36866_d_n5, assign30170_e36866_d_n6, assign30170_e36866_d_n7, assign30170_e36866_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) && (var_guard585 == 0.0)) && (var_guard586 == 0.0)) {
        let assign30170_e36842: f64 = (-230.25850929940458);
        let assign30170_e36844: f64 = (assign30170_e36842 - var_mtat);
        let assign30170_e36848: f64 = (-230.25850929940458);
        let assign30170_e36850: f64 = (assign30170_e36848 - var_mtat);
        let assign30170_e36853: f64 = (-230.25850929940458);
        let assign30170_e36855: f64 = (assign30170_e36853 - var_mtat);
        let assign30170_e36857: f64 = (assign30170_e36855 * 0.3333333333333333);
        let assign30170_e36858: f64 = (1.0 + assign30170_e36857);
        let assign30170_e36859: f64 = (assign30170_e36850 * assign30170_e36858);
        let assign30170_e36860: f64 = (0.5 * assign30170_e36859);
        let assign30170_e36861: f64 = (1.0 + assign30170_e36860);
        let assign30170_e36862: f64 = (assign30170_e36844 * assign30170_e36861);
        let assign30170_e36863: f64 = (1.0 + assign30170_e36862);
        let assign30170_e36864: f64 = (1e-100 / assign30170_e36863);
        (assign30170_e36864, (-((1e-100 * (((-var_mtat_dn5) * assign30170_e36861) + (assign30170_e36844 * (0.5 * (((-var_mtat_dn5) * assign30170_e36858) + (assign30170_e36850 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign30170_e36863 * assign30170_e36863))), (-((1e-100 * (((-var_mtat_dn6) * assign30170_e36861) + (assign30170_e36844 * (0.5 * (((-var_mtat_dn6) * assign30170_e36858) + (assign30170_e36850 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign30170_e36863 * assign30170_e36863))), (-((1e-100 * (((-var_mtat_dn7) * assign30170_e36861) + (assign30170_e36844 * (0.5 * (((-var_mtat_dn7) * assign30170_e36858) + (assign30170_e36850 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign30170_e36863 * assign30170_e36863))), (-((1e-100 * (((-var_mtat_dn8) * assign30170_e36861) + (assign30170_e36844 * (0.5 * (((-var_mtat_dn8) * assign30170_e36858) + (assign30170_e36850 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign30170_e36863 * assign30170_e36863))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30170_e36866;
        var_tmp_dn5 = assign30170_e36866_d_n5;
        var_tmp_dn6 = assign30170_e36866_d_n6;
        var_tmp_dn7 = assign30170_e36866_d_n7;
        var_tmp_dn8 = assign30170_e36866_d_n8;

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
        *var_guard580_slot = var_guard580;
        *var_guard581_slot = var_guard581;
        *var_guard582_slot = var_guard582;
        *var_guard583_slot = var_guard583;
        *var_guard584_slot = var_guard584;
        *var_guard585_slot = var_guard585;
        *var_guard586_slot = var_guard586;
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

    pub(super) fn stamp_transient_block_60(
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
        var_guard182: f64,
        var_guard199: f64,
        var_guard577: f64,
        var_guard581: f64,
        var_guard585: f64,
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
        var_one_over_one_minus_pgat_d: f64,
        var_pbrgatd_i: f64,
        var_pgatd_i: f64,
        var_phitdinv: f64,
        var_slopegat_d: f64,
        var_slopegat_d_dn5: f64,
        var_slopegat_d_dn6: f64,
        var_slopegat_d_dn7: f64,
        var_slopegat_d_dn8: f64,
        var_v1: f64,
        var_v2: f64,
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
        var_guard587_slot: &mut f64,
        var_guard588_slot: &mut f64,
        var_guard589_slot: &mut f64,
        var_guard590_slot: &mut f64,
        var_guard591_slot: &mut f64,
        var_guard592_slot: &mut f64,
        var_guard593_slot: &mut f64,
        var_guard594_slot: &mut f64,
        var_guard595_slot: &mut f64,
        var_guard596_slot: &mut f64,
        var_guard597_slot: &mut f64,
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
        let mut var_guard587: f64 = *var_guard587_slot;
        let mut var_guard588: f64 = *var_guard588_slot;
        let mut var_guard589: f64 = *var_guard589_slot;
        let mut var_guard590: f64 = *var_guard590_slot;
        let mut var_guard591: f64 = *var_guard591_slot;
        let mut var_guard592: f64 = *var_guard592_slot;
        let mut var_guard593: f64 = *var_guard593_slot;
        let mut var_guard594: f64 = *var_guard594_slot;
        let mut var_guard595: f64 = *var_guard595_slot;
        let mut var_guard596: f64 = *var_guard596_slot;
        let mut var_guard597: f64 = *var_guard597_slot;
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

        let (assign30180_e36885, assign30180_e36885_d_n5, assign30180_e36885_d_n6, assign30180_e36885_d_n7, assign30180_e36885_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) && (var_guard585 == 0.0)) {
        let assign30180_e36881: f64 = (2.0 * var_tmp);
        let assign30180_e36883: f64 = (assign30180_e36881 - var_erfcpos);
        (assign30180_e36883, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign30180_e36885;
        var_erfctimesexpmtat_dn5 = assign30180_e36885_d_n5;
        var_erfctimesexpmtat_dn6 = assign30180_e36885_d_n6;
        var_erfctimesexpmtat_dn7 = assign30180_e36885_d_n7;
        var_erfctimesexpmtat_dn8 = assign30180_e36885_d_n8;

        let (assign30190_e36905, assign30190_e36905_d_n5, assign30190_e36905_d_n6, assign30190_e36905_d_n7, assign30190_e36905_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) {
        let assign30190_e36897: f64 = (1.772453850905516 * 0.5);
        let assign30190_e36900: f64 = (var_atatgat_d * var_erfctimesexpmtat);
        let assign30190_e36902: f64 = (assign30190_e36900 / var_ktat);
        let assign30190_e36903: f64 = (assign30190_e36897 * assign30190_e36902);
        (assign30190_e36903, (assign30190_e36897 * ((((var_atatgat_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign30190_e36900 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign30190_e36897 * ((((var_atatgat_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign30190_e36900 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign30190_e36897 * ((((var_atatgat_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign30190_e36900 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign30190_e36897 * ((((var_atatgat_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign30190_e36900 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign30190_e36905;
        var_gammamax_dn5 = assign30190_e36905_d_n5;
        var_gammamax_dn6 = assign30190_e36905_d_n6;
        var_gammamax_dn7 = assign30190_e36905_d_n7;
        var_gammamax_dn8 = assign30190_e36905_d_n8;

        let (assign30200_e36923, assign30200_e36923_d_n5, assign30200_e36923_d_n6, assign30200_e36923_d_n7, assign30200_e36923_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard581 == 0.0)) {
        let assign30200_e36918: f64 = (var_asrh * var_gammamax);
        let assign30200_e36920: f64 = (assign30200_e36918 * var_wtat);
        let assign30200_e36921: f64 = (var_ctatgatd_i * assign30200_e36920);
        (assign30200_e36921, (var_ctatgatd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign30200_e36918 * var_wtat_dn5))), (var_ctatgatd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign30200_e36918 * var_wtat_dn6))), (var_ctatgatd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign30200_e36918 * var_wtat_dn7))), (var_ctatgatd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign30200_e36918 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign30200_e36923;
        var_itat_dn5 = assign30200_e36923_d_n5;
        var_itat_dn6 = assign30200_e36923_d_n6;
        var_itat_dn7 = assign30200_e36923_d_n7;
        var_itat_dn8 = assign30200_e36923_d_n8;

        let assign30210_e36926: f64 = if var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard587 = assign30210_e36926;

        let (assign30220_e36937, assign30220_e36937_d_n5, assign30220_e36937_d_n6, assign30220_e36937_d_n7, assign30220_e36937_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard587 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign30220_e36937;
        var_ibbt_dn5 = assign30220_e36937_d_n5;
        var_ibbt_dn6 = assign30220_e36937_d_n6;
        var_ibbt_dn7 = assign30220_e36937_d_n7;
        var_ibbt_dn8 = assign30220_e36937_d_n8;

        let assign30230_e36940: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard588 = assign30230_e36940;

        let (assign30240_e36959, assign30240_e36959_d_n5, assign30240_e36959_d_n6, assign30240_e36959_d_n7, assign30240_e36959_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard587 == 0.0)) && (var_guard588 != 0.0)) {
        let assign30240_e36954: f64 = (var_vbirgatd_i - var_vbbt);
        let assign30240_e36956: f64 = (assign30240_e36954 * var_vbirgatinv_d);
        let assign30240_e36957: f64 = (assign30240_e36956).sqrt();
        (assign30240_e36957, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30240_e36959;
        var_tmp_dn5 = assign30240_e36959_d_n5;
        var_tmp_dn6 = assign30240_e36959_d_n6;
        var_tmp_dn7 = assign30240_e36959_d_n7;
        var_tmp_dn8 = assign30240_e36959_d_n8;

        let (assign30250_e36980, assign30250_e36980_d_n5, assign30250_e36980_d_n6, assign30250_e36980_d_n7, assign30250_e36980_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard587 == 0.0)) && (var_guard588 == 0.0)) {
        let assign30250_e36974: f64 = (var_vbirgatd_i - var_vbbt);
        let assign30250_e36976: f64 = (assign30250_e36974 * var_vbirgatinv_d);
        let assign30250_e36978: f64 = (assign30250_e36976).powf(var_pgatd_i);
        (assign30250_e36978, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30250_e36980;
        var_tmp_dn5 = assign30250_e36980_d_n5;
        var_tmp_dn6 = assign30250_e36980_d_n6;
        var_tmp_dn7 = assign30250_e36980_d_n7;
        var_tmp_dn8 = assign30250_e36980_d_n8;

        let (assign30260_e37000, assign30260_e37000_d_n5, assign30260_e37000_d_n6, assign30260_e37000_d_n7, assign30260_e37000_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard587 == 0.0)) {
        let assign30260_e36993: f64 = (var_vbirgatd_i - var_vbbt);
        let assign30260_e36995: f64 = (assign30260_e36993 * var_wdepnulrinvgat_d);
        let assign30260_e36997: f64 = (assign30260_e36995 / var_tmp);
        let assign30260_e36998: f64 = (var_one_over_one_minus_pgat_d * assign30260_e36997);
        (assign30260_e36998, (var_one_over_one_minus_pgat_d * (-((assign30260_e36995 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign30260_e36995 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign30260_e36995 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign30260_e36995 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign30260_e37000;
        var_fmaxr_dn5 = assign30260_e37000_d_n5;
        var_fmaxr_dn6 = assign30260_e37000_d_n6;
        var_fmaxr_dn7 = assign30260_e37000_d_n7;
        var_fmaxr_dn8 = assign30260_e37000_d_n8;

        let assign30270_e37002: f64 = (-var_fbbtgat_d);
        let assign30270_e37004: f64 = (assign30270_e37002 / var_fmaxr);
        let assign30270_e37005: f64 = (assign30270_e37004).abs();
        let assign30270_e37007: f64 = if assign30270_e37005 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard589 = assign30270_e37007;

        let (assign30280_e37025, assign30280_e37025_d_n5, assign30280_e37025_d_n6, assign30280_e37025_d_n7, assign30280_e37025_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard587 == 0.0)) && (var_guard589 != 0.0)) {
        let assign30280_e37020: f64 = (-var_fbbtgat_d);
        let assign30280_e37022: f64 = (assign30280_e37020 / var_fmaxr);
        let assign30280_e37023: f64 = (assign30280_e37022).exp();
        (assign30280_e37023, (assign30280_e37023 * ((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign30280_e37020 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign30280_e37023 * ((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign30280_e37020 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign30280_e37023 * ((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign30280_e37020 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign30280_e37023 * ((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign30280_e37020 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30280_e37025;
        var_tmp_dn5 = assign30280_e37025_d_n5;
        var_tmp_dn6 = assign30280_e37025_d_n6;
        var_tmp_dn7 = assign30280_e37025_d_n7;
        var_tmp_dn8 = assign30280_e37025_d_n8;

        let assign30290_e37027: f64 = (-var_fbbtgat_d);
        let assign30290_e37029: f64 = (assign30290_e37027 / var_fmaxr);
        let assign30290_e37031: f64 = if assign30290_e37029 < 0.0 { 1.0 } else { 0.0 };
        var_guard590 = assign30290_e37031;

        let (assign30300_e37082, assign30300_e37082_d_n5, assign30300_e37082_d_n6, assign30300_e37082_d_n7, assign30300_e37082_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard587 == 0.0)) && (var_guard589 == 0.0)) && (var_guard590 != 0.0)) {
        let assign30300_e37049: f64 = (-230.25850929940458);
        let assign30300_e37051: f64 = (-var_fbbtgat_d);
        let assign30300_e37053: f64 = (assign30300_e37051 / var_fmaxr);
        let assign30300_e37054: f64 = (assign30300_e37049 - assign30300_e37053);
        let assign30300_e37058: f64 = (-230.25850929940458);
        let assign30300_e37060: f64 = (-var_fbbtgat_d);
        let assign30300_e37062: f64 = (assign30300_e37060 / var_fmaxr);
        let assign30300_e37063: f64 = (assign30300_e37058 - assign30300_e37062);
        let assign30300_e37066: f64 = (-230.25850929940458);
        let assign30300_e37068: f64 = (-var_fbbtgat_d);
        let assign30300_e37070: f64 = (assign30300_e37068 / var_fmaxr);
        let assign30300_e37071: f64 = (assign30300_e37066 - assign30300_e37070);
        let assign30300_e37073: f64 = (assign30300_e37071 * 0.3333333333333333);
        let assign30300_e37074: f64 = (1.0 + assign30300_e37073);
        let assign30300_e37075: f64 = (assign30300_e37063 * assign30300_e37074);
        let assign30300_e37076: f64 = (0.5 * assign30300_e37075);
        let assign30300_e37077: f64 = (1.0 + assign30300_e37076);
        let assign30300_e37078: f64 = (assign30300_e37054 * assign30300_e37077);
        let assign30300_e37079: f64 = (1.0 + assign30300_e37078);
        let assign30300_e37080: f64 = (1e-100 / assign30300_e37079);
        (assign30300_e37080, (-((1e-100 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign30300_e37051 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign30300_e37077) + (assign30300_e37054 * (0.5 * (((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign30300_e37060 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign30300_e37074) + (assign30300_e37063 * ((-((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign30300_e37068 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign30300_e37079 * assign30300_e37079))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign30300_e37051 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign30300_e37077) + (assign30300_e37054 * (0.5 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign30300_e37060 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign30300_e37074) + (assign30300_e37063 * ((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign30300_e37068 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign30300_e37079 * assign30300_e37079))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign30300_e37051 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign30300_e37077) + (assign30300_e37054 * (0.5 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign30300_e37060 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign30300_e37074) + (assign30300_e37063 * ((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign30300_e37068 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign30300_e37079 * assign30300_e37079))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign30300_e37051 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign30300_e37077) + (assign30300_e37054 * (0.5 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign30300_e37060 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign30300_e37074) + (assign30300_e37063 * ((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign30300_e37068 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign30300_e37079 * assign30300_e37079))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30300_e37082;
        var_tmp_dn5 = assign30300_e37082_d_n5;
        var_tmp_dn6 = assign30300_e37082_d_n6;
        var_tmp_dn7 = assign30300_e37082_d_n7;
        var_tmp_dn8 = assign30300_e37082_d_n8;

        let (assign30310_e37131, assign30310_e37131_d_n5, assign30310_e37131_d_n6, assign30310_e37131_d_n7, assign30310_e37131_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard587 == 0.0)) && (var_guard589 == 0.0)) && (var_guard590 == 0.0)) {
        let assign30310_e37101: f64 = (-var_fbbtgat_d);
        let assign30310_e37103: f64 = (assign30310_e37101 / var_fmaxr);
        let assign30310_e37105: f64 = (assign30310_e37103 - 230.25850929940458);
        let assign30310_e37109: f64 = (-var_fbbtgat_d);
        let assign30310_e37111: f64 = (assign30310_e37109 / var_fmaxr);
        let assign30310_e37113: f64 = (assign30310_e37111 - 230.25850929940458);
        let assign30310_e37116: f64 = (-var_fbbtgat_d);
        let assign30310_e37118: f64 = (assign30310_e37116 / var_fmaxr);
        let assign30310_e37120: f64 = (assign30310_e37118 - 230.25850929940458);
        let assign30310_e37122: f64 = (assign30310_e37120 * 0.3333333333333333);
        let assign30310_e37123: f64 = (1.0 + assign30310_e37122);
        let assign30310_e37124: f64 = (assign30310_e37113 * assign30310_e37123);
        let assign30310_e37125: f64 = (0.5 * assign30310_e37124);
        let assign30310_e37126: f64 = (1.0 + assign30310_e37125);
        let assign30310_e37127: f64 = (assign30310_e37105 * assign30310_e37126);
        let assign30310_e37128: f64 = (1.0 + assign30310_e37127);
        let assign30310_e37129: f64 = (1e100 * assign30310_e37128);
        (assign30310_e37129, (1e100 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign30310_e37101 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign30310_e37126) + (assign30310_e37105 * (0.5 * ((((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign30310_e37109 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign30310_e37123) + (assign30310_e37113 * (((((-var_fbbtgat_d_dn5) * var_fmaxr) - (assign30310_e37116 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign30310_e37101 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign30310_e37126) + (assign30310_e37105 * (0.5 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign30310_e37109 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign30310_e37123) + (assign30310_e37113 * (((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign30310_e37116 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign30310_e37101 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign30310_e37126) + (assign30310_e37105 * (0.5 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign30310_e37109 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign30310_e37123) + (assign30310_e37113 * (((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign30310_e37116 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign30310_e37101 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign30310_e37126) + (assign30310_e37105 * (0.5 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign30310_e37109 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign30310_e37123) + (assign30310_e37113 * (((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign30310_e37116 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30310_e37131;
        var_tmp_dn5 = assign30310_e37131_d_n5;
        var_tmp_dn6 = assign30310_e37131_d_n6;
        var_tmp_dn7 = assign30310_e37131_d_n7;
        var_tmp_dn8 = assign30310_e37131_d_n8;

        let (assign30320_e37151, assign30320_e37151_d_n5, assign30320_e37151_d_n6, assign30320_e37151_d_n7, assign30320_e37151_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard587 == 0.0)) {
        let assign30320_e37144: f64 = (var_v1 * var_fmaxr);
        let assign30320_e37146: f64 = (assign30320_e37144 * var_fmaxr);
        let assign30320_e37148: f64 = (assign30320_e37146 * var_tmp);
        let assign30320_e37149: f64 = (var_cbbtgatd_i * assign30320_e37148);
        (assign30320_e37149, (var_cbbtgatd_i * (((((var_v1 * var_fmaxr_dn5) * var_fmaxr) + (assign30320_e37144 * var_fmaxr_dn5)) * var_tmp) + (assign30320_e37146 * var_tmp_dn5))), (var_cbbtgatd_i * (((((var_v1 * var_fmaxr_dn6) * var_fmaxr) + (assign30320_e37144 * var_fmaxr_dn6)) * var_tmp) + (assign30320_e37146 * var_tmp_dn6))), (var_cbbtgatd_i * (((((var_v1 * var_fmaxr_dn7) * var_fmaxr) + (assign30320_e37144 * var_fmaxr_dn7)) * var_tmp) + (assign30320_e37146 * var_tmp_dn7))), (var_cbbtgatd_i * (((((var_v1 * var_fmaxr_dn8) * var_fmaxr) + (assign30320_e37144 * var_fmaxr_dn8)) * var_tmp) + (assign30320_e37146 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign30320_e37151;
        var_ibbt_dn5 = assign30320_e37151_d_n5;
        var_ibbt_dn6 = assign30320_e37151_d_n6;
        var_ibbt_dn7 = assign30320_e37151_d_n7;
        var_ibbt_dn8 = assign30320_e37151_d_n8;

        let assign30330_e37154: f64 = if var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard591 = assign30330_e37154;

        let (assign30340_e37165, assign30340_e37165_d_n5, assign30340_e37165_d_n6, assign30340_e37165_d_n7, assign30340_e37165_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard591 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign30340_e37165;
        var_fbreakdown_dn5 = assign30340_e37165_d_n5;
        var_fbreakdown_dn6 = assign30340_e37165_d_n6;
        var_fbreakdown_dn7 = assign30340_e37165_d_n7;
        var_fbreakdown_dn8 = assign30340_e37165_d_n8;

        let assign30350_e37168: f64 = (-var_alphaav);
        let assign30350_e37170: f64 = (assign30350_e37168 * var_vbrgatd_i);
        let assign30350_e37171: f64 = if var_vav > assign30350_e37170 { 1.0 } else { 0.0 };
        var_guard592 = assign30350_e37171;

        let assign30360_e37174: f64 = if var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard593 = assign30360_e37174;

        let (assign30370_e37204, assign30370_e37204_d_n5, assign30370_e37204_d_n6, assign30370_e37204_d_n7, assign30370_e37204_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard591 == 0.0)) && (var_guard592 != 0.0)) && (var_guard593 != 0.0)) {
        let assign30370_e37190: f64 = (var_vav * var_vbrinvgat_d);
        let assign30370_e37193: f64 = (var_vav * var_vbrinvgat_d);
        let assign30370_e37194: f64 = (assign30370_e37190 * assign30370_e37193);
        let assign30370_e37197: f64 = (var_vav * var_vbrinvgat_d);
        let assign30370_e37198: f64 = (assign30370_e37194 * assign30370_e37197);
        let assign30370_e37201: f64 = (var_vav * var_vbrinvgat_d);
        let assign30370_e37202: f64 = (assign30370_e37198 * assign30370_e37201);
        (assign30370_e37202, (((((((var_vav * var_vbrinvgat_d_dn5) * assign30370_e37193) + (assign30370_e37190 * (var_vav * var_vbrinvgat_d_dn5))) * assign30370_e37197) + (assign30370_e37194 * (var_vav * var_vbrinvgat_d_dn5))) * assign30370_e37201) + (assign30370_e37198 * (var_vav * var_vbrinvgat_d_dn5))), (((((((var_vav * var_vbrinvgat_d_dn6) * assign30370_e37193) + (assign30370_e37190 * (var_vav * var_vbrinvgat_d_dn6))) * assign30370_e37197) + (assign30370_e37194 * (var_vav * var_vbrinvgat_d_dn6))) * assign30370_e37201) + (assign30370_e37198 * (var_vav * var_vbrinvgat_d_dn6))), (((((((var_vav * var_vbrinvgat_d_dn7) * assign30370_e37193) + (assign30370_e37190 * (var_vav * var_vbrinvgat_d_dn7))) * assign30370_e37197) + (assign30370_e37194 * (var_vav * var_vbrinvgat_d_dn7))) * assign30370_e37201) + (assign30370_e37198 * (var_vav * var_vbrinvgat_d_dn7))), (((((((var_vav * var_vbrinvgat_d_dn8) * assign30370_e37193) + (assign30370_e37190 * (var_vav * var_vbrinvgat_d_dn8))) * assign30370_e37197) + (assign30370_e37194 * (var_vav * var_vbrinvgat_d_dn8))) * assign30370_e37201) + (assign30370_e37198 * (var_vav * var_vbrinvgat_d_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30370_e37204;
        var_tmp_dn5 = assign30370_e37204_d_n5;
        var_tmp_dn6 = assign30370_e37204_d_n6;
        var_tmp_dn7 = assign30370_e37204_d_n7;
        var_tmp_dn8 = assign30370_e37204_d_n8;

        let (assign30380_e37226, assign30380_e37226_d_n5, assign30380_e37226_d_n6, assign30380_e37226_d_n7, assign30380_e37226_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard591 == 0.0)) && (var_guard592 != 0.0)) && (var_guard593 == 0.0)) {
        let assign30380_e37221: f64 = (var_vav * var_vbrinvgat_d);
        let assign30380_e37222: f64 = (assign30380_e37221).abs();
        let assign30380_e37224: f64 = (assign30380_e37222).powf(var_pbrgatd_i);
        (assign30380_e37224, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign30380_e37222).powf(var_pbrgatd_i - 1.0) * if assign30380_e37221 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) })) } } else { (assign30380_e37224 * (var_pbrgatd_i * (if assign30380_e37221 >= 0.0 { (var_vav * var_vbrinvgat_d_dn5) } else { (-(var_vav * var_vbrinvgat_d_dn5)) } / assign30380_e37222))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign30380_e37222).powf(var_pbrgatd_i - 1.0) * if assign30380_e37221 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) })) } } else { (assign30380_e37224 * (var_pbrgatd_i * (if assign30380_e37221 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) } / assign30380_e37222))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign30380_e37222).powf(var_pbrgatd_i - 1.0) * if assign30380_e37221 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) })) } } else { (assign30380_e37224 * (var_pbrgatd_i * (if assign30380_e37221 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) } / assign30380_e37222))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign30380_e37222).powf(var_pbrgatd_i - 1.0) * if assign30380_e37221 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) })) } } else { (assign30380_e37224 * (var_pbrgatd_i * (if assign30380_e37221 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) } / assign30380_e37222))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30380_e37226;
        var_tmp_dn5 = assign30380_e37226_d_n5;
        var_tmp_dn6 = assign30380_e37226_d_n6;
        var_tmp_dn7 = assign30380_e37226_d_n7;
        var_tmp_dn8 = assign30380_e37226_d_n8;

        let (assign30390_e37244, assign30390_e37244_d_n5, assign30390_e37244_d_n6, assign30390_e37244_d_n7, assign30390_e37244_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard591 == 0.0)) && (var_guard592 != 0.0)) {
        let assign30390_e37241: f64 = (1.0 - var_tmp);
        let assign30390_e37242: f64 = (1.0 / assign30390_e37241);
        (assign30390_e37242, (-((-var_tmp_dn5) / (assign30390_e37241 * assign30390_e37241))), (-((-var_tmp_dn6) / (assign30390_e37241 * assign30390_e37241))), (-((-var_tmp_dn7) / (assign30390_e37241 * assign30390_e37241))), (-((-var_tmp_dn8) / (assign30390_e37241 * assign30390_e37241))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign30390_e37244;
        var_fbreakdown_dn5 = assign30390_e37244_d_n5;
        var_fbreakdown_dn6 = assign30390_e37244_d_n6;
        var_fbreakdown_dn7 = assign30390_e37244_d_n7;
        var_fbreakdown_dn8 = assign30390_e37244_d_n8;

        let (assign30400_e37267, assign30400_e37267_d_n5, assign30400_e37267_d_n6, assign30400_e37267_d_n7, assign30400_e37267_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) && (var_guard591 == 0.0)) && (var_guard592 == 0.0)) {
        let assign30400_e37261: f64 = (var_alphaav * var_vbrgatd_i);
        let assign30400_e37262: f64 = (var_vav + assign30400_e37261);
        let assign30400_e37264: f64 = (assign30400_e37262 * var_slopegat_d);
        let assign30400_e37265: f64 = (var_fstopgat_d + assign30400_e37264);
        (assign30400_e37265, (assign30400_e37262 * var_slopegat_d_dn5), (assign30400_e37262 * var_slopegat_d_dn6), (assign30400_e37262 * var_slopegat_d_dn7), (assign30400_e37262 * var_slopegat_d_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign30400_e37267;
        var_fbreakdown_dn5 = assign30400_e37267_d_n5;
        var_fbreakdown_dn6 = assign30400_e37267_d_n6;
        var_fbreakdown_dn7 = assign30400_e37267_d_n7;
        var_fbreakdown_dn8 = assign30400_e37267_d_n8;

        let (assign30410_e37286, assign30410_e37286_d_n5, assign30410_e37286_d_n6, assign30410_e37286_d_n7, assign30410_e37286_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard577 == 0.0)) {
        let assign30410_e37277: f64 = (var_id__blk219 + var_isrh);
        let assign30410_e37279: f64 = (assign30410_e37277 + var_itat);
        let assign30410_e37281: f64 = (assign30410_e37279 + var_ibbt);
        let assign30410_e37282: f64 = (p.p29 * assign30410_e37281);
        let assign30410_e37284: f64 = (assign30410_e37282 * var_fbreakdown);
        (assign30410_e37284, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign30410_e37282 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign30410_e37282 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign30410_e37282 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign30410_e37282 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign30410_e37286;
        var_ijungat_dn5 = assign30410_e37286_d_n5;
        var_ijungat_dn6 = assign30410_e37286_d_n6;
        var_ijungat_dn7 = assign30410_e37286_d_n7;
        var_ijungat_dn8 = assign30410_e37286_d_n8;

        let (assign30420_e37302, assign30420_e37302_d_n5, assign30420_e37302_d_n6, assign30420_e37302_d_n7, assign30420_e37302_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign30420_e37292: f64 = (var_abdrain_i * var_ijunbot);
        let assign30420_e37295: f64 = (var_lsdrain_i * var_ijunsti);
        let assign30420_e37296: f64 = (assign30420_e37292 + assign30420_e37295);
        let assign30420_e37299: f64 = (var_lgdrain_i * var_ijungat);
        let assign30420_e37300: f64 = (assign30420_e37296 + assign30420_e37299);
        (assign30420_e37300, (((var_abdrain_i * var_ijunbot_dn5) + (var_lsdrain_i * var_ijunsti_dn5)) + (var_lgdrain_i * var_ijungat_dn5)), (((var_abdrain_i * var_ijunbot_dn6) + (var_lsdrain_i * var_ijunsti_dn6)) + (var_lgdrain_i * var_ijungat_dn6)), (((var_abdrain_i * var_ijunbot_dn7) + (var_lsdrain_i * var_ijunsti_dn7)) + (var_lgdrain_i * var_ijungat_dn7)), (((var_abdrain_i * var_ijunbot_dn8) + (var_lsdrain_i * var_ijunsti_dn8)) + (var_lgdrain_i * var_ijungat_dn8)),)
    } else {
        (var_i1, var_i1_dn5, var_i1_dn6, var_i1_dn7, var_i1_dn8,)
    }
};
        var_i1 = assign30420_e37302;
        var_i1_dn5 = assign30420_e37302_d_n5;
        var_i1_dn6 = assign30420_e37302_d_n6;
        var_i1_dn7 = assign30420_e37302_d_n7;
        var_i1_dn8 = assign30420_e37302_d_n8;

        let (assign30430_e37308,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign30430_e37308;

        let (assign30440_e37314,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign30440_e37314;

        let assign30450_e37326: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard594 = assign30450_e37326;

        let assign30530_e37412: f64 = if var_v2 < var_vmax_d { 1.0 } else { 0.0 };
        var_guard595 = assign30530_e37412;

        let assign30540_e37414: f64 = (-0.5);
        let assign30540_e37417: f64 = (var_v2 * var_phitdinv);
        let assign30540_e37418: f64 = (assign30540_e37414 * assign30540_e37417);
        let assign30540_e37419: f64 = (assign30540_e37418).abs();
        let assign30540_e37421: f64 = if assign30540_e37419 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard596 = assign30540_e37421;

        let (assign30550_e37439,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard594 != 0.0)) && (var_guard595 != 0.0)) && (var_guard596 != 0.0)) {
        let assign30550_e37432: f64 = (-0.5);
        let assign30550_e37435: f64 = (var_v2 * var_phitdinv);
        let assign30550_e37436: f64 = (assign30550_e37432 * assign30550_e37435);
        let assign30550_e37437: f64 = (assign30550_e37436).exp();
        (assign30550_e37437,)
    } else {
        (var_z,)
    }
};
        var_z = assign30550_e37439;

        let assign30560_e37441: f64 = (-0.5);
        let assign30560_e37444: f64 = (var_v2 * var_phitdinv);
        let assign30560_e37445: f64 = (assign30560_e37441 * assign30560_e37444);
        let assign30560_e37447: f64 = if assign30560_e37445 < 0.0 { 1.0 } else { 0.0 };
        var_guard597 = assign30560_e37447;

        let (assign30570_e37502,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard594 != 0.0)) && (var_guard595 != 0.0)) && (var_guard596 == 0.0)) && (var_guard597 != 0.0)) {
        let assign30570_e37463: f64 = (-230.25850929940458);
        let assign30570_e37465: f64 = (-0.5);
        let assign30570_e37468: f64 = (var_v2 * var_phitdinv);
        let assign30570_e37469: f64 = (assign30570_e37465 * assign30570_e37468);
        let assign30570_e37470: f64 = (assign30570_e37463 - assign30570_e37469);
        let assign30570_e37474: f64 = (-230.25850929940458);
        let assign30570_e37476: f64 = (-0.5);
        let assign30570_e37479: f64 = (var_v2 * var_phitdinv);
        let assign30570_e37480: f64 = (assign30570_e37476 * assign30570_e37479);
        let assign30570_e37481: f64 = (assign30570_e37474 - assign30570_e37480);
        let assign30570_e37484: f64 = (-230.25850929940458);
        let assign30570_e37486: f64 = (-0.5);
        let assign30570_e37489: f64 = (var_v2 * var_phitdinv);
        let assign30570_e37490: f64 = (assign30570_e37486 * assign30570_e37489);
        let assign30570_e37491: f64 = (assign30570_e37484 - assign30570_e37490);
        let assign30570_e37493: f64 = (assign30570_e37491 * 0.3333333333333333);
        let assign30570_e37494: f64 = (1.0 + assign30570_e37493);
        let assign30570_e37495: f64 = (assign30570_e37481 * assign30570_e37494);
        let assign30570_e37496: f64 = (0.5 * assign30570_e37495);
        let assign30570_e37497: f64 = (1.0 + assign30570_e37496);
        let assign30570_e37498: f64 = (assign30570_e37470 * assign30570_e37497);
        let assign30570_e37499: f64 = (1.0 + assign30570_e37498);
        let assign30570_e37500: f64 = (1e-100 / assign30570_e37499);
        (assign30570_e37500,)
    } else {
        (var_z,)
    }
};
        var_z = assign30570_e37502;

        let (assign30580_e37555,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard594 != 0.0)) && (var_guard595 != 0.0)) && (var_guard596 == 0.0)) && (var_guard597 == 0.0)) {
        let assign30580_e37519: f64 = (-0.5);
        let assign30580_e37522: f64 = (var_v2 * var_phitdinv);
        let assign30580_e37523: f64 = (assign30580_e37519 * assign30580_e37522);
        let assign30580_e37525: f64 = (assign30580_e37523 - 230.25850929940458);
        let assign30580_e37529: f64 = (-0.5);
        let assign30580_e37532: f64 = (var_v2 * var_phitdinv);
        let assign30580_e37533: f64 = (assign30580_e37529 * assign30580_e37532);
        let assign30580_e37535: f64 = (assign30580_e37533 - 230.25850929940458);
        let assign30580_e37538: f64 = (-0.5);
        let assign30580_e37541: f64 = (var_v2 * var_phitdinv);
        let assign30580_e37542: f64 = (assign30580_e37538 * assign30580_e37541);
        let assign30580_e37544: f64 = (assign30580_e37542 - 230.25850929940458);
        let assign30580_e37546: f64 = (assign30580_e37544 * 0.3333333333333333);
        let assign30580_e37547: f64 = (1.0 + assign30580_e37546);
        let assign30580_e37548: f64 = (assign30580_e37535 * assign30580_e37547);
        let assign30580_e37549: f64 = (0.5 * assign30580_e37548);
        let assign30580_e37550: f64 = (1.0 + assign30580_e37549);
        let assign30580_e37551: f64 = (assign30580_e37525 * assign30580_e37550);
        let assign30580_e37552: f64 = (1.0 + assign30580_e37551);
        let assign30580_e37553: f64 = (1e100 * assign30580_e37552);
        (assign30580_e37553,)
    } else {
        (var_z,)
    }
};
        var_z = assign30580_e37555;

        let (assign30590_e37567,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard594 != 0.0)) && (var_guard595 != 0.0)) {
        let assign30590_e37565: f64 = (1.0 / var_z);
        (assign30590_e37565,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign30590_e37567;

        let (assign30600_e37579,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard594 != 0.0)) && (var_guard595 != 0.0)) {
        let assign30600_e37577: f64 = (var_zinv * var_zinv);
        (assign30600_e37577,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign30600_e37579;

        let (assign30610_e37598,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard594 != 0.0)) && (var_guard595 == 0.0)) {
        let assign30610_e37591: f64 = (var_v2 - var_vmax_d);
        let assign30610_e37593: f64 = (assign30610_e37591 * var_phitdinv);
        let assign30610_e37594: f64 = (1.0 + assign30610_e37593);
        let assign30610_e37596: f64 = (assign30610_e37594 * var_exp_vmax_over_phitd_d);
        (assign30610_e37596,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign30610_e37598;

        let (assign30620_e37610,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard594 != 0.0)) && (var_guard595 == 0.0)) {
        let assign30620_e37608: f64 = (var_idmult).sqrt();
        (assign30620_e37608,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign30620_e37610;

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
        *var_guard587_slot = var_guard587;
        *var_guard588_slot = var_guard588;
        *var_guard589_slot = var_guard589;
        *var_guard590_slot = var_guard590;
        *var_guard591_slot = var_guard591;
        *var_guard592_slot = var_guard592;
        *var_guard593_slot = var_guard593;
        *var_guard594_slot = var_guard594;
        *var_guard595_slot = var_guard595;
        *var_guard596_slot = var_guard596;
        *var_guard597_slot = var_guard597;
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

    pub(super) fn stamp_transient_block_61(
        var_abdrain_i: f64,
        var_atatbot_d: f64,
        var_btatpartbot_d: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_ftdbot_d: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard594: f64,
        var_guard595: f64,
        var_idsatbot_d: f64,
        var_one_minus_pbot_d: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_phitd: f64,
        var_phitr: f64,
        var_v2: f64,
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
        var_guard598_slot: &mut f64,
        var_guard599_slot: &mut f64,
        var_guard600_slot: &mut f64,
        var_guard601_slot: &mut f64,
        var_guard602_slot: &mut f64,
        var_guard603_slot: &mut f64,
        var_guard604_slot: &mut f64,
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
        let mut var_guard598: f64 = *var_guard598_slot;
        let mut var_guard599: f64 = *var_guard599_slot;
        let mut var_guard600: f64 = *var_guard600_slot;
        let mut var_guard601: f64 = *var_guard601_slot;
        let mut var_guard602: f64 = *var_guard602_slot;
        let mut var_guard603: f64 = *var_guard603_slot;
        let mut var_guard604: f64 = *var_guard604_slot;
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

        let (assign30630_e37623,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard594 != 0.0)) && (var_guard595 == 0.0)) {
        let assign30630_e37621: f64 = (1.0 / var_zinv);
        (assign30630_e37621,)
    } else {
        (var_z,)
    }
};
        var_z = assign30630_e37623;

        let (assign30640_e37633,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard594 != 0.0)) {
        let assign30640_e37631: f64 = (var_idmult - 1.0);
        (assign30640_e37631,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign30640_e37633;

        let assign30650_e37636: f64 = if var_v2 > 0.0 { 1.0 } else { 0.0 };
        var_guard598 = assign30650_e37636;

        let (assign30660_e37662,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard594 != 0.0)) && (var_guard598 != 0.0)) {
        let assign30660_e37648: f64 = (2.0 + var_z);
        let assign30660_e37651: f64 = (var_z + 1.0);
        let assign30660_e37654: f64 = (var_z + 3.0);
        let assign30660_e37655: f64 = (assign30660_e37651 * assign30660_e37654);
        let assign30660_e37656: f64 = (assign30660_e37655).sqrt();
        let assign30660_e37657: f64 = (assign30660_e37648 + assign30660_e37656);
        let assign30660_e37658: f64 = (assign30660_e37657).ln();
        let assign30660_e37659: f64 = (var_phitd * assign30660_e37658);
        let assign30660_e37660: f64 = (2.0 * assign30660_e37659);
        (assign30660_e37660,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign30660_e37662;

        let (assign30670_e37696,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard594 != 0.0)) && (var_guard598 == 0.0)) {
        let assign30670_e37672: f64 = (-var_v2);
        let assign30670_e37677: f64 = (2.0 * var_zinv);
        let assign30670_e37679: f64 = (assign30670_e37677 + 1.0);
        let assign30670_e37682: f64 = (1.0 + var_zinv);
        let assign30670_e37686: f64 = (3.0 * var_zinv);
        let assign30670_e37687: f64 = (1.0 + assign30670_e37686);
        let assign30670_e37688: f64 = (assign30670_e37682 * assign30670_e37687);
        let assign30670_e37689: f64 = (assign30670_e37688).sqrt();
        let assign30670_e37690: f64 = (assign30670_e37679 + assign30670_e37689);
        let assign30670_e37691: f64 = (assign30670_e37690).ln();
        let assign30670_e37692: f64 = (var_phitd * assign30670_e37691);
        let assign30670_e37693: f64 = (2.0 * assign30670_e37692);
        let assign30670_e37694: f64 = (assign30670_e37672 + assign30670_e37693);
        (assign30670_e37694,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign30670_e37696;

        let (assign30680_e37706,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard594 != 0.0)) {
        let assign30680_e37704: f64 = (var_vbimin_d - var_two_psistar);
        (assign30680_e37704,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign30680_e37706;

        let (assign30690_e37733,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard594 != 0.0)) {
        let assign30690_e37715: f64 = (var_v2 + var_vjlim);
        let assign30690_e37718: f64 = (var_v2 - var_vjlim);
        let assign30690_e37721: f64 = (var_v2 - var_vjlim);
        let assign30690_e37722: f64 = (assign30690_e37718 * assign30690_e37721);
        let assign30690_e37725: f64 = (4.0 * var_phitd);
        let assign30690_e37727: f64 = (assign30690_e37725 * var_phitd);
        let assign30690_e37728: f64 = (assign30690_e37722 + assign30690_e37727);
        let assign30690_e37729: f64 = (assign30690_e37728).sqrt();
        let assign30690_e37730: f64 = (assign30690_e37715 - assign30690_e37729);
        let assign30690_e37731: f64 = (0.5 * assign30690_e37730);
        (assign30690_e37731,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign30690_e37733;

        let (assign30700_e37760,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard594 != 0.0)) {
        let assign30700_e37742: f64 = (var_v2 + var_vbbtlim_d);
        let assign30700_e37745: f64 = (var_v2 - var_vbbtlim_d);
        let assign30700_e37748: f64 = (var_v2 - var_vbbtlim_d);
        let assign30700_e37749: f64 = (assign30700_e37745 * assign30700_e37748);
        let assign30700_e37752: f64 = (4.0 * var_phitr);
        let assign30700_e37754: f64 = (assign30700_e37752 * var_phitr);
        let assign30700_e37755: f64 = (assign30700_e37749 + assign30700_e37754);
        let assign30700_e37756: f64 = (assign30700_e37755).sqrt();
        let assign30700_e37757: f64 = (assign30700_e37742 - assign30700_e37756);
        let assign30700_e37758: f64 = (0.5 * assign30700_e37757);
        (assign30700_e37758,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign30700_e37760;

        let (assign30710_e37787,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard594 != 0.0)) {
        let assign30710_e37769: f64 = var_v2;
        let assign30710_e37772: f64 = var_v2;
        let assign30710_e37775: f64 = var_v2;
        let assign30710_e37776: f64 = (assign30710_e37772 * assign30710_e37775);
        let assign30710_e37779: f64 = (4.0 * 1e-6);
        let assign30710_e37781: f64 = (assign30710_e37779 * 1e-6);
        let assign30710_e37782: f64 = (assign30710_e37776 + assign30710_e37781);
        let assign30710_e37783: f64 = (assign30710_e37782).sqrt();
        let assign30710_e37784: f64 = (assign30710_e37769 - assign30710_e37783);
        let assign30710_e37785: f64 = (0.5 * assign30710_e37784);
        (assign30710_e37785,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign30710_e37787;

        let assign30720_e37790: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard599 = assign30720_e37790;

        let (assign30730_e37798, assign30730_e37798_d_n5, assign30730_e37798_d_n6, assign30730_e37798_d_n7, assign30730_e37798_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign30730_e37798;
        var_ijunbot_dn5 = assign30730_e37798_d_n5;
        var_ijunbot_dn6 = assign30730_e37798_d_n6;
        var_ijunbot_dn7 = assign30730_e37798_d_n7;
        var_ijunbot_dn8 = assign30730_e37798_d_n8;

        let (assign30740_e37809,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) {
        let assign30740_e37807: f64 = (var_idsatbot_d * var_idmult);
        (assign30740_e37807,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign30740_e37809;

        let assign30750_e37816: f64 = if ((var_csrhbotd_i == 0.0) && (var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard600 = assign30750_e37816;

        let (assign30760_e37827, assign30760_e37827_d_n5, assign30760_e37827_d_n6, assign30760_e37827_d_n7, assign30760_e37827_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard600 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign30760_e37827;
        var_isrh_dn5 = assign30760_e37827_d_n5;
        var_isrh_dn6 = assign30760_e37827_d_n6;
        var_isrh_dn7 = assign30760_e37827_d_n7;
        var_isrh_dn8 = assign30760_e37827_d_n8;

        let (assign30770_e37841,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard600 == 0.0)) {
        let assign30770_e37839: f64 = (var_vbibot_d - var_vjsrh);
        (assign30770_e37839,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign30770_e37841;

        let (assign30780_e37860,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard600 == 0.0)) {
        let assign30780_e37855: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign30780_e37856: f64 = (1.0 - assign30780_e37855);
        let assign30780_e37857: f64 = (assign30780_e37856).sqrt();
        let assign30780_e37858: f64 = (1.0 - assign30780_e37857);
        (assign30780_e37858,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign30780_e37860;

        let assign30790_e37863: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard601 = assign30790_e37863;

        let (assign30800_e37877,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard600 == 0.0)) && (var_guard601 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign30800_e37877;

        let (assign30810_e37909,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard600 == 0.0)) && (var_guard601 == 0.0)) {
        let assign30810_e37892: f64 = (var_wsrhstep * var_wsrhstep);
        let assign30810_e37894: f64 = (var_wsrhstep).ln();
        let assign30810_e37895: f64 = (assign30810_e37892 * assign30810_e37894);
        let assign30810_e37898: f64 = (1.0 - var_wsrhstep);
        let assign30810_e37899: f64 = (assign30810_e37895 / assign30810_e37898);
        let assign30810_e37901: f64 = (assign30810_e37899 + var_wsrhstep);
        let assign30810_e37905: f64 = (2.0 * var_pbotd_i);
        let assign30810_e37906: f64 = (1.0 - assign30810_e37905);
        let assign30810_e37907: f64 = (assign30810_e37901 * assign30810_e37906);
        (assign30810_e37907,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign30810_e37909;

        let (assign30820_e37923,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard600 == 0.0)) {
        let assign30820_e37921: f64 = (var_wsrhstep + var_dwsrh);
        (assign30820_e37921,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign30820_e37923;

        let assign30830_e37926: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard602 = assign30830_e37926;

        let (assign30840_e37943, assign30840_e37943_d_n5, assign30840_e37943_d_n6, assign30840_e37943_d_n7, assign30840_e37943_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard600 == 0.0)) && (var_guard602 != 0.0)) {
        let assign30840_e37940: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign30840_e37941: f64 = (assign30840_e37940).sqrt();
        (assign30840_e37941, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30840_e37943;
        var_tmp_dn5 = assign30840_e37943_d_n5;
        var_tmp_dn6 = assign30840_e37943_d_n6;
        var_tmp_dn7 = assign30840_e37943_d_n7;
        var_tmp_dn8 = assign30840_e37943_d_n8;

        let (assign30850_e37962, assign30850_e37962_d_n5, assign30850_e37962_d_n6, assign30850_e37962_d_n7, assign30850_e37962_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard600 == 0.0)) && (var_guard602 == 0.0)) {
        let assign30850_e37958: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign30850_e37960: f64 = (assign30850_e37958).powf(var_pbotd_i);
        (assign30850_e37960, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign30850_e37962;
        var_tmp_dn5 = assign30850_e37962_d_n5;
        var_tmp_dn6 = assign30850_e37962_d_n6;
        var_tmp_dn7 = assign30850_e37962_d_n7;
        var_tmp_dn8 = assign30850_e37962_d_n8;

        let (assign30860_e37976, assign30860_e37976_d_n5, assign30860_e37976_d_n6, assign30860_e37976_d_n7, assign30860_e37976_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard600 == 0.0)) {
        let assign30860_e37974: f64 = (var_wdepnulrbot_d * var_tmp);
        (assign30860_e37974, (var_wdepnulrbot_d * var_tmp_dn5), (var_wdepnulrbot_d * var_tmp_dn6), (var_wdepnulrbot_d * var_tmp_dn7), (var_wdepnulrbot_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign30860_e37976;
        var_wdep_dn5 = assign30860_e37976_d_n5;
        var_wdep_dn6 = assign30860_e37976_d_n6;
        var_wdep_dn7 = assign30860_e37976_d_n7;
        var_wdep_dn8 = assign30860_e37976_d_n8;

        let (assign30870_e37994, assign30870_e37994_d_n5, assign30870_e37994_d_n6, assign30870_e37994_d_n7, assign30870_e37994_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard600 == 0.0)) {
        let assign30870_e37989: f64 = (var_zinv - 1.0);
        let assign30870_e37991: f64 = (assign30870_e37989 * var_wdep);
        let assign30870_e37992: f64 = (var_ftdbot_d * assign30870_e37991);
        (assign30870_e37992, (var_ftdbot_d * (assign30870_e37989 * var_wdep_dn5)), (var_ftdbot_d * (assign30870_e37989 * var_wdep_dn6)), (var_ftdbot_d * (assign30870_e37989 * var_wdep_dn7)), (var_ftdbot_d * (assign30870_e37989 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign30870_e37994;
        var_asrh_dn5 = assign30870_e37994_d_n5;
        var_asrh_dn6 = assign30870_e37994_d_n6;
        var_asrh_dn7 = assign30870_e37994_d_n7;
        var_asrh_dn8 = assign30870_e37994_d_n8;

        let (assign30880_e38010, assign30880_e38010_d_n5, assign30880_e38010_d_n6, assign30880_e38010_d_n7, assign30880_e38010_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard600 == 0.0)) {
        let assign30880_e38007: f64 = (var_asrh * var_wsrh);
        let assign30880_e38008: f64 = (var_csrhbotd_i * assign30880_e38007);
        (assign30880_e38008, (var_csrhbotd_i * (var_asrh_dn5 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign30880_e38010;
        var_isrh_dn5 = assign30880_e38010_d_n5;
        var_isrh_dn6 = assign30880_e38010_d_n6;
        var_isrh_dn7 = assign30880_e38010_d_n7;
        var_isrh_dn8 = assign30880_e38010_d_n8;

        let assign30890_e38013: f64 = if var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard603 = assign30890_e38013;

        let (assign30900_e38024, assign30900_e38024_d_n5, assign30900_e38024_d_n6, assign30900_e38024_d_n7, assign30900_e38024_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign30900_e38024;
        var_itat_dn5 = assign30900_e38024_d_n5;
        var_itat_dn6 = assign30900_e38024_d_n6;
        var_itat_dn7 = assign30900_e38024_d_n7;
        var_itat_dn8 = assign30900_e38024_d_n8;

        let (assign30910_e38042, assign30910_e38042_d_n5, assign30910_e38042_d_n6, assign30910_e38042_d_n7, assign30910_e38042_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) {
        let assign30910_e38037: f64 = (var_wdep * var_one_minus_pbot_d);
        let assign30910_e38039: f64 = (assign30910_e38037 / var_vbi_minus_vjsrh);
        let assign30910_e38040: f64 = (var_btatpartbot_d * assign30910_e38039);
        (assign30910_e38040, (var_btatpartbot_d * ((var_wdep_dn5 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn6 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn7 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn8 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign30910_e38042;
        var_btat_dn5 = assign30910_e38042_d_n5;
        var_btat_dn6 = assign30910_e38042_d_n6;
        var_btat_dn7 = assign30910_e38042_d_n7;
        var_btat_dn8 = assign30910_e38042_d_n8;

        let (assign30920_e38058, assign30920_e38058_d_n5, assign30920_e38058_d_n6, assign30920_e38058_d_n7, assign30920_e38058_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) {
        let assign30920_e38054: f64 = (0.666666666666667 * var_atatbot_d);
        let assign30920_e38056: f64 = (assign30920_e38054 / var_btat);
        (assign30920_e38056, (-((assign30920_e38054 * var_btat_dn5) / (var_btat * var_btat))), (-((assign30920_e38054 * var_btat_dn6) / (var_btat * var_btat))), (-((assign30920_e38054 * var_btat_dn7) / (var_btat * var_btat))), (-((assign30920_e38054 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign30920_e38058;
        var_twoatatoverthreebtat_dn5 = assign30920_e38058_d_n5;
        var_twoatatoverthreebtat_dn6 = assign30920_e38058_d_n6;
        var_twoatatoverthreebtat_dn7 = assign30920_e38058_d_n7;
        var_twoatatoverthreebtat_dn8 = assign30920_e38058_d_n8;

        let (assign30930_e38072, assign30930_e38072_d_n5, assign30930_e38072_d_n6, assign30930_e38072_d_n7, assign30930_e38072_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) {
        let assign30930_e38070: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign30930_e38070, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign30930_e38072;
        var_umaxbeforelimiting_dn5 = assign30930_e38072_d_n5;
        var_umaxbeforelimiting_dn6 = assign30930_e38072_d_n6;
        var_umaxbeforelimiting_dn7 = assign30930_e38072_d_n7;
        var_umaxbeforelimiting_dn8 = assign30930_e38072_d_n8;

        let (assign30940_e38093, assign30940_e38093_d_n5, assign30940_e38093_d_n6, assign30940_e38093_d_n7, assign30940_e38093_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) {
        let assign30940_e38084: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign30940_e38087: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign30940_e38089: f64 = (assign30940_e38087 + 1.0);
        let assign30940_e38090: f64 = (assign30940_e38084 / assign30940_e38089);
        let assign30940_e38091: f64 = (assign30940_e38090).sqrt();
        (assign30940_e38091, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign30940_e38089) - (assign30940_e38084 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign30940_e38089 * assign30940_e38089)) / (2.0 * assign30940_e38091)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign30940_e38089) - (assign30940_e38084 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign30940_e38089 * assign30940_e38089)) / (2.0 * assign30940_e38091)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign30940_e38089) - (assign30940_e38084 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign30940_e38089 * assign30940_e38089)) / (2.0 * assign30940_e38091)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign30940_e38089) - (assign30940_e38084 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign30940_e38089 * assign30940_e38089)) / (2.0 * assign30940_e38091)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign30940_e38093;
        var_umax_dn5 = assign30940_e38093_d_n5;
        var_umax_dn6 = assign30940_e38093_d_n6;
        var_umax_dn7 = assign30940_e38093_d_n7;
        var_umax_dn8 = assign30940_e38093_d_n8;

        let (assign30950_e38106, assign30950_e38106_d_n5, assign30950_e38106_d_n6, assign30950_e38106_d_n7, assign30950_e38106_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) {
        let assign30950_e38104: f64 = (var_umax).sqrt();
        (assign30950_e38104, (var_umax_dn5 / (2.0 * assign30950_e38104)), (var_umax_dn6 / (2.0 * assign30950_e38104)), (var_umax_dn7 / (2.0 * assign30950_e38104)), (var_umax_dn8 / (2.0 * assign30950_e38104)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign30950_e38106;
        var_sqrtumax_dn5 = assign30950_e38106_d_n5;
        var_sqrtumax_dn6 = assign30950_e38106_d_n6;
        var_sqrtumax_dn7 = assign30950_e38106_d_n7;
        var_sqrtumax_dn8 = assign30950_e38106_d_n8;

        let (assign30960_e38120, assign30960_e38120_d_n5, assign30960_e38120_d_n6, assign30960_e38120_d_n7, assign30960_e38120_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) {
        let assign30960_e38118: f64 = (var_umax * var_sqrtumax);
        (assign30960_e38118, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign30960_e38120;
        var_umaxpoweronepointfive_dn5 = assign30960_e38120_d_n5;
        var_umaxpoweronepointfive_dn6 = assign30960_e38120_d_n6;
        var_umaxpoweronepointfive_dn7 = assign30960_e38120_d_n7;
        var_umaxpoweronepointfive_dn8 = assign30960_e38120_d_n8;

        let assign30970_e38122: f64 = (-var_pbotd_i);
        let assign30970_e38124: f64 = (assign30970_e38122 * var_one_over_one_minus_pbot_d);
        let assign30970_e38126: f64 = (-1.0);
        let assign30970_e38127: f64 = if assign30970_e38124 == assign30970_e38126 { 1.0 } else { 0.0 };
        var_guard604 = assign30970_e38127;

        let (assign30980_e38147, assign30980_e38147_d_n5, assign30980_e38147_d_n6, assign30980_e38147_d_n7, assign30980_e38147_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) && (var_guard604 != 0.0)) {
        let assign30980_e38143: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign30980_e38144: f64 = (1.0 + assign30980_e38143);
        let assign30980_e38145: f64 = (1.0 / assign30980_e38144);
        (assign30980_e38145, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign30980_e38144 * assign30980_e38144))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign30980_e38144 * assign30980_e38144))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign30980_e38144 * assign30980_e38144))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign30980_e38144 * assign30980_e38144))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign30980_e38147;
        var_wgamma_dn5 = assign30980_e38147_d_n5;
        var_wgamma_dn6 = assign30980_e38147_d_n6;
        var_wgamma_dn7 = assign30980_e38147_d_n7;
        var_wgamma_dn8 = assign30980_e38147_d_n8;

        let (assign30990_e38171, assign30990_e38171_d_n5, assign30990_e38171_d_n6, assign30990_e38171_d_n7, assign30990_e38171_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) && (var_guard604 == 0.0)) {
        let assign30990_e38163: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign30990_e38164: f64 = (1.0 + assign30990_e38163);
        let assign30990_e38166: f64 = (-var_pbotd_i);
        let assign30990_e38168: f64 = (assign30990_e38166 * var_one_over_one_minus_pbot_d);
        let assign30990_e38169: f64 = (assign30990_e38164).powf(assign30990_e38168);
        (assign30990_e38169, if 0.0 == 0.0 && ((assign30990_e38168) as f64).is_finite() && ((assign30990_e38168) as f64).fract() == 0.0 { if assign30990_e38168 == 0.0 { 0.0 } else { (assign30990_e38168 * ((assign30990_e38164).powf(assign30990_e38168 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign30990_e38169 * (assign30990_e38168 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign30990_e38164))) }, if 0.0 == 0.0 && ((assign30990_e38168) as f64).is_finite() && ((assign30990_e38168) as f64).fract() == 0.0 { if assign30990_e38168 == 0.0 { 0.0 } else { (assign30990_e38168 * ((assign30990_e38164).powf(assign30990_e38168 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign30990_e38169 * (assign30990_e38168 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign30990_e38164))) }, if 0.0 == 0.0 && ((assign30990_e38168) as f64).is_finite() && ((assign30990_e38168) as f64).fract() == 0.0 { if assign30990_e38168 == 0.0 { 0.0 } else { (assign30990_e38168 * ((assign30990_e38164).powf(assign30990_e38168 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign30990_e38169 * (assign30990_e38168 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign30990_e38164))) }, if 0.0 == 0.0 && ((assign30990_e38168) as f64).is_finite() && ((assign30990_e38168) as f64).fract() == 0.0 { if assign30990_e38168 == 0.0 { 0.0 } else { (assign30990_e38168 * ((assign30990_e38164).powf(assign30990_e38168 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign30990_e38169 * (assign30990_e38168 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign30990_e38164))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign30990_e38171;
        var_wgamma_dn5 = assign30990_e38171_d_n5;
        var_wgamma_dn6 = assign30990_e38171_d_n6;
        var_wgamma_dn7 = assign30990_e38171_d_n7;
        var_wgamma_dn8 = assign30990_e38171_d_n8;

        let (assign31000_e38189, assign31000_e38189_d_n5, assign31000_e38189_d_n6, assign31000_e38189_d_n7, assign31000_e38189_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) {
        let assign31000_e38183: f64 = (var_wsrh * var_wgamma);
        let assign31000_e38186: f64 = (var_wsrh + var_wgamma);
        let assign31000_e38187: f64 = (assign31000_e38183 / assign31000_e38186);
        (assign31000_e38187, ((((var_wsrh * var_wgamma_dn5) * assign31000_e38186) - (assign31000_e38183 * var_wgamma_dn5)) / (assign31000_e38186 * assign31000_e38186)), ((((var_wsrh * var_wgamma_dn6) * assign31000_e38186) - (assign31000_e38183 * var_wgamma_dn6)) / (assign31000_e38186 * assign31000_e38186)), ((((var_wsrh * var_wgamma_dn7) * assign31000_e38186) - (assign31000_e38183 * var_wgamma_dn7)) / (assign31000_e38186 * assign31000_e38186)), ((((var_wsrh * var_wgamma_dn8) * assign31000_e38186) - (assign31000_e38183 * var_wgamma_dn8)) / (assign31000_e38186 * assign31000_e38186)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign31000_e38189;
        var_wtat_dn5 = assign31000_e38189_d_n5;
        var_wtat_dn6 = assign31000_e38189_d_n6;
        var_wtat_dn7 = assign31000_e38189_d_n7;
        var_wtat_dn8 = assign31000_e38189_d_n8;

        let (assign31010_e38206, assign31010_e38206_d_n5, assign31010_e38206_d_n6, assign31010_e38206_d_n7, assign31010_e38206_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) {
        let assign31010_e38202: f64 = (var_btat / var_sqrtumax);
        let assign31010_e38203: f64 = (0.375 * assign31010_e38202);
        let assign31010_e38204: f64 = (assign31010_e38203).sqrt();
        (assign31010_e38204, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign31010_e38204)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign31010_e38204)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign31010_e38204)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign31010_e38204)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign31010_e38206;
        var_ktat_dn5 = assign31010_e38206_d_n5;
        var_ktat_dn6 = assign31010_e38206_d_n6;
        var_ktat_dn7 = assign31010_e38206_d_n7;
        var_ktat_dn8 = assign31010_e38206_d_n8;

        let (assign31020_e38224, assign31020_e38224_d_n5, assign31020_e38224_d_n6, assign31020_e38224_d_n7, assign31020_e38224_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) {
        let assign31020_e38219: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign31020_e38220: f64 = (2.0 * assign31020_e38219);
        let assign31020_e38222: f64 = (assign31020_e38220 - var_umax);
        (assign31020_e38222, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign31020_e38224;
        var_ltat_dn5 = assign31020_e38224_d_n5;
        var_ltat_dn6 = assign31020_e38224_d_n6;
        var_ltat_dn7 = assign31020_e38224_d_n7;
        var_ltat_dn8 = assign31020_e38224_d_n8;

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
        *var_guard598_slot = var_guard598;
        *var_guard599_slot = var_guard599;
        *var_guard600_slot = var_guard600;
        *var_guard601_slot = var_guard601;
        *var_guard602_slot = var_guard602;
        *var_guard603_slot = var_guard603;
        *var_guard604_slot = var_guard604;
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

    pub(super) fn stamp_transient_block_62(
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
        var_guard182: f64,
        var_guard199: f64,
        var_guard599: f64,
        var_guard603: f64,
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
        var_v2: f64,
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
        var_guard605_slot: &mut f64,
        var_guard606_slot: &mut f64,
        var_guard607_slot: &mut f64,
        var_guard608_slot: &mut f64,
        var_guard609_slot: &mut f64,
        var_guard610_slot: &mut f64,
        var_guard611_slot: &mut f64,
        var_guard612_slot: &mut f64,
        var_guard613_slot: &mut f64,
        var_guard614_slot: &mut f64,
        var_guard615_slot: &mut f64,
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
        let mut var_guard605: f64 = *var_guard605_slot;
        let mut var_guard606: f64 = *var_guard606_slot;
        let mut var_guard607: f64 = *var_guard607_slot;
        let mut var_guard608: f64 = *var_guard608_slot;
        let mut var_guard609: f64 = *var_guard609_slot;
        let mut var_guard610: f64 = *var_guard610_slot;
        let mut var_guard611: f64 = *var_guard611_slot;
        let mut var_guard612: f64 = *var_guard612_slot;
        let mut var_guard613: f64 = *var_guard613_slot;
        let mut var_guard614: f64 = *var_guard614_slot;
        let mut var_guard615: f64 = *var_guard615_slot;
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

        let (assign31030_e38250, assign31030_e38250_d_n5, assign31030_e38250_d_n6, assign31030_e38250_d_n7, assign31030_e38250_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) {
        let assign31030_e38236: f64 = (var_atatbot_d * var_twoatatoverthreebtat);
        let assign31030_e38238: f64 = (assign31030_e38236 * var_sqrtumax);
        let assign31030_e38241: f64 = (var_atatbot_d * var_umax);
        let assign31030_e38242: f64 = (assign31030_e38238 - assign31030_e38241);
        let assign31030_e38246: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign31030_e38247: f64 = (0.5 * assign31030_e38246);
        let assign31030_e38248: f64 = (assign31030_e38242 + assign31030_e38247);
        (assign31030_e38248, (((((var_atatbot_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign31030_e38236 * var_sqrtumax_dn5)) - (var_atatbot_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign31030_e38236 * var_sqrtumax_dn6)) - (var_atatbot_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign31030_e38236 * var_sqrtumax_dn7)) - (var_atatbot_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign31030_e38236 * var_sqrtumax_dn8)) - (var_atatbot_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign31030_e38250;
        var_mtat_dn5 = assign31030_e38250_d_n5;
        var_mtat_dn6 = assign31030_e38250_d_n6;
        var_mtat_dn7 = assign31030_e38250_d_n7;
        var_mtat_dn8 = assign31030_e38250_d_n8;

        let (assign31040_e38266, assign31040_e38266_d_n5, assign31040_e38266_d_n6, assign31040_e38266_d_n7, assign31040_e38266_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) {
        let assign31040_e38262: f64 = (var_ltat - 1.0);
        let assign31040_e38264: f64 = (assign31040_e38262 * var_ktat);
        (assign31040_e38264, ((var_ltat_dn5 * var_ktat) + (assign31040_e38262 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign31040_e38262 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign31040_e38262 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign31040_e38262 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign31040_e38266;
        var_xerfc_dn5 = assign31040_e38266_d_n5;
        var_xerfc_dn6 = assign31040_e38266_d_n6;
        var_xerfc_dn7 = assign31040_e38266_d_n7;
        var_xerfc_dn8 = assign31040_e38266_d_n8;

        let (assign31050_e38280, assign31050_e38280_d_n5, assign31050_e38280_d_n6, assign31050_e38280_d_n7, assign31050_e38280_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) {
        let assign31050_e38278: f64 = (var_xerfc * var_xerfc);
        (assign31050_e38278, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign31050_e38280;
        var_ysq_dn5 = assign31050_e38280_d_n5;
        var_ysq_dn6 = assign31050_e38280_d_n6;
        var_ysq_dn7 = assign31050_e38280_d_n7;
        var_ysq_dn8 = assign31050_e38280_d_n8;

        let assign31060_e38283: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard605 = assign31060_e38283;

        let (assign31070_e38303, assign31070_e38303_d_n5, assign31070_e38303_d_n6, assign31070_e38303_d_n7, assign31070_e38303_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) && (var_guard605 != 0.0)) {
        let assign31070_e38299: f64 = (var_perfc * var_xerfc);
        let assign31070_e38300: f64 = (1.0 + assign31070_e38299);
        let assign31070_e38301: f64 = (1.0 / assign31070_e38300);
        (assign31070_e38301, (-((var_perfc * var_xerfc_dn5) / (assign31070_e38300 * assign31070_e38300))), (-((var_perfc * var_xerfc_dn6) / (assign31070_e38300 * assign31070_e38300))), (-((var_perfc * var_xerfc_dn7) / (assign31070_e38300 * assign31070_e38300))), (-((var_perfc * var_xerfc_dn8) / (assign31070_e38300 * assign31070_e38300))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign31070_e38303;
        var_terfc_dn5 = assign31070_e38303_d_n5;
        var_terfc_dn6 = assign31070_e38303_d_n6;
        var_terfc_dn7 = assign31070_e38303_d_n7;
        var_terfc_dn8 = assign31070_e38303_d_n8;

        let (assign31080_e38324, assign31080_e38324_d_n5, assign31080_e38324_d_n6, assign31080_e38324_d_n7, assign31080_e38324_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) && (var_guard605 == 0.0)) {
        let assign31080_e38320: f64 = (var_perfc * var_xerfc);
        let assign31080_e38321: f64 = (1.0 - assign31080_e38320);
        let assign31080_e38322: f64 = (1.0 / assign31080_e38321);
        (assign31080_e38322, (-((-(var_perfc * var_xerfc_dn5)) / (assign31080_e38321 * assign31080_e38321))), (-((-(var_perfc * var_xerfc_dn6)) / (assign31080_e38321 * assign31080_e38321))), (-((-(var_perfc * var_xerfc_dn7)) / (assign31080_e38321 * assign31080_e38321))), (-((-(var_perfc * var_xerfc_dn8)) / (assign31080_e38321 * assign31080_e38321))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign31080_e38324;
        var_terfc_dn5 = assign31080_e38324_d_n5;
        var_terfc_dn6 = assign31080_e38324_d_n6;
        var_terfc_dn7 = assign31080_e38324_d_n7;
        var_terfc_dn8 = assign31080_e38324_d_n8;

        let assign31090_e38326: f64 = (-var_ysq);
        let assign31090_e38328: f64 = (assign31090_e38326 + var_mtat);
        let assign31090_e38330: f64 = (-230.25850929940458);
        let assign31090_e38331: f64 = if assign31090_e38328 > assign31090_e38330 { 1.0 } else { 0.0 };
        var_guard606 = assign31090_e38331;

        let (assign31100_e38349, assign31100_e38349_d_n5, assign31100_e38349_d_n6, assign31100_e38349_d_n7, assign31100_e38349_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) && (var_guard606 != 0.0)) {
        let assign31100_e38344: f64 = (-var_ysq);
        let assign31100_e38346: f64 = (assign31100_e38344 + var_mtat);
        let assign31100_e38347: f64 = (assign31100_e38346).exp();
        (assign31100_e38347, (assign31100_e38347 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign31100_e38347 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign31100_e38347 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign31100_e38347 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31100_e38349;
        var_tmp_dn5 = assign31100_e38349_d_n5;
        var_tmp_dn6 = assign31100_e38349_d_n6;
        var_tmp_dn7 = assign31100_e38349_d_n7;
        var_tmp_dn8 = assign31100_e38349_d_n8;

        let (assign31110_e38398, assign31110_e38398_d_n5, assign31110_e38398_d_n6, assign31110_e38398_d_n7, assign31110_e38398_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) && (var_guard606 == 0.0)) {
        let assign31110_e38365: f64 = (-230.25850929940458);
        let assign31110_e38367: f64 = (-var_ysq);
        let assign31110_e38369: f64 = (assign31110_e38367 + var_mtat);
        let assign31110_e38370: f64 = (assign31110_e38365 - assign31110_e38369);
        let assign31110_e38374: f64 = (-230.25850929940458);
        let assign31110_e38376: f64 = (-var_ysq);
        let assign31110_e38378: f64 = (assign31110_e38376 + var_mtat);
        let assign31110_e38379: f64 = (assign31110_e38374 - assign31110_e38378);
        let assign31110_e38382: f64 = (-230.25850929940458);
        let assign31110_e38384: f64 = (-var_ysq);
        let assign31110_e38386: f64 = (assign31110_e38384 + var_mtat);
        let assign31110_e38387: f64 = (assign31110_e38382 - assign31110_e38386);
        let assign31110_e38389: f64 = (assign31110_e38387 * 0.3333333333333333);
        let assign31110_e38390: f64 = (1.0 + assign31110_e38389);
        let assign31110_e38391: f64 = (assign31110_e38379 * assign31110_e38390);
        let assign31110_e38392: f64 = (0.5 * assign31110_e38391);
        let assign31110_e38393: f64 = (1.0 + assign31110_e38392);
        let assign31110_e38394: f64 = (assign31110_e38370 * assign31110_e38393);
        let assign31110_e38395: f64 = (1.0 + assign31110_e38394);
        let assign31110_e38396: f64 = (1e-100 / assign31110_e38395);
        (assign31110_e38396, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign31110_e38393) + (assign31110_e38370 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign31110_e38390) + (assign31110_e38379 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign31110_e38395 * assign31110_e38395))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign31110_e38393) + (assign31110_e38370 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign31110_e38390) + (assign31110_e38379 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign31110_e38395 * assign31110_e38395))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign31110_e38393) + (assign31110_e38370 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign31110_e38390) + (assign31110_e38379 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign31110_e38395 * assign31110_e38395))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign31110_e38393) + (assign31110_e38370 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign31110_e38390) + (assign31110_e38379 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign31110_e38395 * assign31110_e38395))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31110_e38398;
        var_tmp_dn5 = assign31110_e38398_d_n5;
        var_tmp_dn6 = assign31110_e38398_d_n6;
        var_tmp_dn7 = assign31110_e38398_d_n7;
        var_tmp_dn8 = assign31110_e38398_d_n8;

        let (assign31120_e38428, assign31120_e38428_d_n5, assign31120_e38428_d_n6, assign31120_e38428_d_n7, assign31120_e38428_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) {
        let assign31120_e38410: f64 = (0.29214664 * var_terfc);
        let assign31120_e38414: f64 = (var_terfc * var_terfc);
        let assign31120_e38415: f64 = (var_berfc * assign31120_e38414);
        let assign31120_e38416: f64 = (assign31120_e38410 + assign31120_e38415);
        let assign31120_e38420: f64 = (var_terfc * var_terfc);
        let assign31120_e38422: f64 = (assign31120_e38420 * var_terfc);
        let assign31120_e38423: f64 = (var_cerfc * assign31120_e38422);
        let assign31120_e38424: f64 = (assign31120_e38416 + assign31120_e38423);
        let assign31120_e38426: f64 = (assign31120_e38424 * var_tmp);
        (assign31120_e38426, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign31120_e38420 * var_terfc_dn5)))) * var_tmp) + (assign31120_e38424 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign31120_e38420 * var_terfc_dn6)))) * var_tmp) + (assign31120_e38424 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign31120_e38420 * var_terfc_dn7)))) * var_tmp) + (assign31120_e38424 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign31120_e38420 * var_terfc_dn8)))) * var_tmp) + (assign31120_e38424 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign31120_e38428;
        var_erfcpos_dn5 = assign31120_e38428_d_n5;
        var_erfcpos_dn6 = assign31120_e38428_d_n6;
        var_erfcpos_dn7 = assign31120_e38428_d_n7;
        var_erfcpos_dn8 = assign31120_e38428_d_n8;

        let assign31130_e38431: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard607 = assign31130_e38431;

        let (assign31140_e38445, assign31140_e38445_d_n5, assign31140_e38445_d_n6, assign31140_e38445_d_n7, assign31140_e38445_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) && (var_guard607 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign31140_e38445;
        var_erfctimesexpmtat_dn5 = assign31140_e38445_d_n5;
        var_erfctimesexpmtat_dn6 = assign31140_e38445_d_n6;
        var_erfctimesexpmtat_dn7 = assign31140_e38445_d_n7;
        var_erfctimesexpmtat_dn8 = assign31140_e38445_d_n8;

        let assign31150_e38448: f64 = (-230.25850929940458);
        let assign31150_e38449: f64 = if var_mtat > assign31150_e38448 { 1.0 } else { 0.0 };
        var_guard608 = assign31150_e38449;

        let (assign31160_e38467, assign31160_e38467_d_n5, assign31160_e38467_d_n6, assign31160_e38467_d_n7, assign31160_e38467_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) && (var_guard607 == 0.0)) && (var_guard608 != 0.0)) {
        let assign31160_e38465: f64 = (var_mtat).exp();
        (assign31160_e38465, (assign31160_e38465 * var_mtat_dn5), (assign31160_e38465 * var_mtat_dn6), (assign31160_e38465 * var_mtat_dn7), (assign31160_e38465 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31160_e38467;
        var_tmp_dn5 = assign31160_e38467_d_n5;
        var_tmp_dn6 = assign31160_e38467_d_n6;
        var_tmp_dn7 = assign31160_e38467_d_n7;
        var_tmp_dn8 = assign31160_e38467_d_n8;

        let (assign31170_e38510, assign31170_e38510_d_n5, assign31170_e38510_d_n6, assign31170_e38510_d_n7, assign31170_e38510_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) && (var_guard607 == 0.0)) && (var_guard608 == 0.0)) {
        let assign31170_e38486: f64 = (-230.25850929940458);
        let assign31170_e38488: f64 = (assign31170_e38486 - var_mtat);
        let assign31170_e38492: f64 = (-230.25850929940458);
        let assign31170_e38494: f64 = (assign31170_e38492 - var_mtat);
        let assign31170_e38497: f64 = (-230.25850929940458);
        let assign31170_e38499: f64 = (assign31170_e38497 - var_mtat);
        let assign31170_e38501: f64 = (assign31170_e38499 * 0.3333333333333333);
        let assign31170_e38502: f64 = (1.0 + assign31170_e38501);
        let assign31170_e38503: f64 = (assign31170_e38494 * assign31170_e38502);
        let assign31170_e38504: f64 = (0.5 * assign31170_e38503);
        let assign31170_e38505: f64 = (1.0 + assign31170_e38504);
        let assign31170_e38506: f64 = (assign31170_e38488 * assign31170_e38505);
        let assign31170_e38507: f64 = (1.0 + assign31170_e38506);
        let assign31170_e38508: f64 = (1e-100 / assign31170_e38507);
        (assign31170_e38508, (-((1e-100 * (((-var_mtat_dn5) * assign31170_e38505) + (assign31170_e38488 * (0.5 * (((-var_mtat_dn5) * assign31170_e38502) + (assign31170_e38494 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign31170_e38507 * assign31170_e38507))), (-((1e-100 * (((-var_mtat_dn6) * assign31170_e38505) + (assign31170_e38488 * (0.5 * (((-var_mtat_dn6) * assign31170_e38502) + (assign31170_e38494 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign31170_e38507 * assign31170_e38507))), (-((1e-100 * (((-var_mtat_dn7) * assign31170_e38505) + (assign31170_e38488 * (0.5 * (((-var_mtat_dn7) * assign31170_e38502) + (assign31170_e38494 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign31170_e38507 * assign31170_e38507))), (-((1e-100 * (((-var_mtat_dn8) * assign31170_e38505) + (assign31170_e38488 * (0.5 * (((-var_mtat_dn8) * assign31170_e38502) + (assign31170_e38494 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign31170_e38507 * assign31170_e38507))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31170_e38510;
        var_tmp_dn5 = assign31170_e38510_d_n5;
        var_tmp_dn6 = assign31170_e38510_d_n6;
        var_tmp_dn7 = assign31170_e38510_d_n7;
        var_tmp_dn8 = assign31170_e38510_d_n8;

        let (assign31180_e38529, assign31180_e38529_d_n5, assign31180_e38529_d_n6, assign31180_e38529_d_n7, assign31180_e38529_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) && (var_guard607 == 0.0)) {
        let assign31180_e38525: f64 = (2.0 * var_tmp);
        let assign31180_e38527: f64 = (assign31180_e38525 - var_erfcpos);
        (assign31180_e38527, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign31180_e38529;
        var_erfctimesexpmtat_dn5 = assign31180_e38529_d_n5;
        var_erfctimesexpmtat_dn6 = assign31180_e38529_d_n6;
        var_erfctimesexpmtat_dn7 = assign31180_e38529_d_n7;
        var_erfctimesexpmtat_dn8 = assign31180_e38529_d_n8;

        let (assign31190_e38549, assign31190_e38549_d_n5, assign31190_e38549_d_n6, assign31190_e38549_d_n7, assign31190_e38549_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) {
        let assign31190_e38541: f64 = (1.772453850905516 * 0.5);
        let assign31190_e38544: f64 = (var_atatbot_d * var_erfctimesexpmtat);
        let assign31190_e38546: f64 = (assign31190_e38544 / var_ktat);
        let assign31190_e38547: f64 = (assign31190_e38541 * assign31190_e38546);
        (assign31190_e38547, (assign31190_e38541 * ((((var_atatbot_d * var_erfctimesexpmtat_dn5) * var_ktat) - (assign31190_e38544 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign31190_e38541 * ((((var_atatbot_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign31190_e38544 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign31190_e38541 * ((((var_atatbot_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign31190_e38544 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign31190_e38541 * ((((var_atatbot_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign31190_e38544 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign31190_e38549;
        var_gammamax_dn5 = assign31190_e38549_d_n5;
        var_gammamax_dn6 = assign31190_e38549_d_n6;
        var_gammamax_dn7 = assign31190_e38549_d_n7;
        var_gammamax_dn8 = assign31190_e38549_d_n8;

        let (assign31200_e38567, assign31200_e38567_d_n5, assign31200_e38567_d_n6, assign31200_e38567_d_n7, assign31200_e38567_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard603 == 0.0)) {
        let assign31200_e38562: f64 = (var_asrh * var_gammamax);
        let assign31200_e38564: f64 = (assign31200_e38562 * var_wtat);
        let assign31200_e38565: f64 = (var_ctatbotd_i * assign31200_e38564);
        (assign31200_e38565, (var_ctatbotd_i * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign31200_e38562 * var_wtat_dn5))), (var_ctatbotd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign31200_e38562 * var_wtat_dn6))), (var_ctatbotd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign31200_e38562 * var_wtat_dn7))), (var_ctatbotd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign31200_e38562 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign31200_e38567;
        var_itat_dn5 = assign31200_e38567_d_n5;
        var_itat_dn6 = assign31200_e38567_d_n6;
        var_itat_dn7 = assign31200_e38567_d_n7;
        var_itat_dn8 = assign31200_e38567_d_n8;

        let assign31210_e38570: f64 = if var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard609 = assign31210_e38570;

        let (assign31220_e38581, assign31220_e38581_d_n5, assign31220_e38581_d_n6, assign31220_e38581_d_n7, assign31220_e38581_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard609 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign31220_e38581;
        var_ibbt_dn5 = assign31220_e38581_d_n5;
        var_ibbt_dn6 = assign31220_e38581_d_n6;
        var_ibbt_dn7 = assign31220_e38581_d_n7;
        var_ibbt_dn8 = assign31220_e38581_d_n8;

        let assign31230_e38584: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard610 = assign31230_e38584;

        let (assign31240_e38603, assign31240_e38603_d_n5, assign31240_e38603_d_n6, assign31240_e38603_d_n7, assign31240_e38603_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard609 == 0.0)) && (var_guard610 != 0.0)) {
        let assign31240_e38598: f64 = (var_vbirbotd_i - var_vbbt);
        let assign31240_e38600: f64 = (assign31240_e38598 * var_vbirbotinv_d);
        let assign31240_e38601: f64 = (assign31240_e38600).sqrt();
        (assign31240_e38601, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31240_e38603;
        var_tmp_dn5 = assign31240_e38603_d_n5;
        var_tmp_dn6 = assign31240_e38603_d_n6;
        var_tmp_dn7 = assign31240_e38603_d_n7;
        var_tmp_dn8 = assign31240_e38603_d_n8;

        let (assign31250_e38624, assign31250_e38624_d_n5, assign31250_e38624_d_n6, assign31250_e38624_d_n7, assign31250_e38624_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard609 == 0.0)) && (var_guard610 == 0.0)) {
        let assign31250_e38618: f64 = (var_vbirbotd_i - var_vbbt);
        let assign31250_e38620: f64 = (assign31250_e38618 * var_vbirbotinv_d);
        let assign31250_e38622: f64 = (assign31250_e38620).powf(var_pbotd_i);
        (assign31250_e38622, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31250_e38624;
        var_tmp_dn5 = assign31250_e38624_d_n5;
        var_tmp_dn6 = assign31250_e38624_d_n6;
        var_tmp_dn7 = assign31250_e38624_d_n7;
        var_tmp_dn8 = assign31250_e38624_d_n8;

        let (assign31260_e38644, assign31260_e38644_d_n5, assign31260_e38644_d_n6, assign31260_e38644_d_n7, assign31260_e38644_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard609 == 0.0)) {
        let assign31260_e38637: f64 = (var_vbirbotd_i - var_vbbt);
        let assign31260_e38639: f64 = (assign31260_e38637 * var_wdepnulrinvbot_d);
        let assign31260_e38641: f64 = (assign31260_e38639 / var_tmp);
        let assign31260_e38642: f64 = (var_one_over_one_minus_pbot_d * assign31260_e38641);
        (assign31260_e38642, (var_one_over_one_minus_pbot_d * (-((assign31260_e38639 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign31260_e38639 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign31260_e38639 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign31260_e38639 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign31260_e38644;
        var_fmaxr_dn5 = assign31260_e38644_d_n5;
        var_fmaxr_dn6 = assign31260_e38644_d_n6;
        var_fmaxr_dn7 = assign31260_e38644_d_n7;
        var_fmaxr_dn8 = assign31260_e38644_d_n8;

        let assign31270_e38646: f64 = (-var_fbbtbot_d);
        let assign31270_e38648: f64 = (assign31270_e38646 / var_fmaxr);
        let assign31270_e38649: f64 = (assign31270_e38648).abs();
        let assign31270_e38651: f64 = if assign31270_e38649 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard611 = assign31270_e38651;

        let (assign31280_e38669, assign31280_e38669_d_n5, assign31280_e38669_d_n6, assign31280_e38669_d_n7, assign31280_e38669_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard609 == 0.0)) && (var_guard611 != 0.0)) {
        let assign31280_e38664: f64 = (-var_fbbtbot_d);
        let assign31280_e38666: f64 = (assign31280_e38664 / var_fmaxr);
        let assign31280_e38667: f64 = (assign31280_e38666).exp();
        (assign31280_e38667, (assign31280_e38667 * (-((assign31280_e38664 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign31280_e38667 * (-((assign31280_e38664 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign31280_e38667 * (-((assign31280_e38664 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign31280_e38667 * (-((assign31280_e38664 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31280_e38669;
        var_tmp_dn5 = assign31280_e38669_d_n5;
        var_tmp_dn6 = assign31280_e38669_d_n6;
        var_tmp_dn7 = assign31280_e38669_d_n7;
        var_tmp_dn8 = assign31280_e38669_d_n8;

        let assign31290_e38671: f64 = (-var_fbbtbot_d);
        let assign31290_e38673: f64 = (assign31290_e38671 / var_fmaxr);
        let assign31290_e38675: f64 = if assign31290_e38673 < 0.0 { 1.0 } else { 0.0 };
        var_guard612 = assign31290_e38675;

        let (assign31300_e38726, assign31300_e38726_d_n5, assign31300_e38726_d_n6, assign31300_e38726_d_n7, assign31300_e38726_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard609 == 0.0)) && (var_guard611 == 0.0)) && (var_guard612 != 0.0)) {
        let assign31300_e38693: f64 = (-230.25850929940458);
        let assign31300_e38695: f64 = (-var_fbbtbot_d);
        let assign31300_e38697: f64 = (assign31300_e38695 / var_fmaxr);
        let assign31300_e38698: f64 = (assign31300_e38693 - assign31300_e38697);
        let assign31300_e38702: f64 = (-230.25850929940458);
        let assign31300_e38704: f64 = (-var_fbbtbot_d);
        let assign31300_e38706: f64 = (assign31300_e38704 / var_fmaxr);
        let assign31300_e38707: f64 = (assign31300_e38702 - assign31300_e38706);
        let assign31300_e38710: f64 = (-230.25850929940458);
        let assign31300_e38712: f64 = (-var_fbbtbot_d);
        let assign31300_e38714: f64 = (assign31300_e38712 / var_fmaxr);
        let assign31300_e38715: f64 = (assign31300_e38710 - assign31300_e38714);
        let assign31300_e38717: f64 = (assign31300_e38715 * 0.3333333333333333);
        let assign31300_e38718: f64 = (1.0 + assign31300_e38717);
        let assign31300_e38719: f64 = (assign31300_e38707 * assign31300_e38718);
        let assign31300_e38720: f64 = (0.5 * assign31300_e38719);
        let assign31300_e38721: f64 = (1.0 + assign31300_e38720);
        let assign31300_e38722: f64 = (assign31300_e38698 * assign31300_e38721);
        let assign31300_e38723: f64 = (1.0 + assign31300_e38722);
        let assign31300_e38724: f64 = (1e-100 / assign31300_e38723);
        (assign31300_e38724, (-((1e-100 * (((-(-((assign31300_e38695 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign31300_e38721) + (assign31300_e38698 * (0.5 * (((-(-((assign31300_e38704 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign31300_e38718) + (assign31300_e38707 * ((-(-((assign31300_e38712 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign31300_e38723 * assign31300_e38723))), (-((1e-100 * (((-(-((assign31300_e38695 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign31300_e38721) + (assign31300_e38698 * (0.5 * (((-(-((assign31300_e38704 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign31300_e38718) + (assign31300_e38707 * ((-(-((assign31300_e38712 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign31300_e38723 * assign31300_e38723))), (-((1e-100 * (((-(-((assign31300_e38695 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign31300_e38721) + (assign31300_e38698 * (0.5 * (((-(-((assign31300_e38704 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign31300_e38718) + (assign31300_e38707 * ((-(-((assign31300_e38712 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign31300_e38723 * assign31300_e38723))), (-((1e-100 * (((-(-((assign31300_e38695 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign31300_e38721) + (assign31300_e38698 * (0.5 * (((-(-((assign31300_e38704 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign31300_e38718) + (assign31300_e38707 * ((-(-((assign31300_e38712 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign31300_e38723 * assign31300_e38723))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31300_e38726;
        var_tmp_dn5 = assign31300_e38726_d_n5;
        var_tmp_dn6 = assign31300_e38726_d_n6;
        var_tmp_dn7 = assign31300_e38726_d_n7;
        var_tmp_dn8 = assign31300_e38726_d_n8;

        let (assign31310_e38775, assign31310_e38775_d_n5, assign31310_e38775_d_n6, assign31310_e38775_d_n7, assign31310_e38775_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard609 == 0.0)) && (var_guard611 == 0.0)) && (var_guard612 == 0.0)) {
        let assign31310_e38745: f64 = (-var_fbbtbot_d);
        let assign31310_e38747: f64 = (assign31310_e38745 / var_fmaxr);
        let assign31310_e38749: f64 = (assign31310_e38747 - 230.25850929940458);
        let assign31310_e38753: f64 = (-var_fbbtbot_d);
        let assign31310_e38755: f64 = (assign31310_e38753 / var_fmaxr);
        let assign31310_e38757: f64 = (assign31310_e38755 - 230.25850929940458);
        let assign31310_e38760: f64 = (-var_fbbtbot_d);
        let assign31310_e38762: f64 = (assign31310_e38760 / var_fmaxr);
        let assign31310_e38764: f64 = (assign31310_e38762 - 230.25850929940458);
        let assign31310_e38766: f64 = (assign31310_e38764 * 0.3333333333333333);
        let assign31310_e38767: f64 = (1.0 + assign31310_e38766);
        let assign31310_e38768: f64 = (assign31310_e38757 * assign31310_e38767);
        let assign31310_e38769: f64 = (0.5 * assign31310_e38768);
        let assign31310_e38770: f64 = (1.0 + assign31310_e38769);
        let assign31310_e38771: f64 = (assign31310_e38749 * assign31310_e38770);
        let assign31310_e38772: f64 = (1.0 + assign31310_e38771);
        let assign31310_e38773: f64 = (1e100 * assign31310_e38772);
        (assign31310_e38773, (1e100 * (((-((assign31310_e38745 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign31310_e38770) + (assign31310_e38749 * (0.5 * (((-((assign31310_e38753 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign31310_e38767) + (assign31310_e38757 * ((-((assign31310_e38760 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign31310_e38745 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign31310_e38770) + (assign31310_e38749 * (0.5 * (((-((assign31310_e38753 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign31310_e38767) + (assign31310_e38757 * ((-((assign31310_e38760 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign31310_e38745 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign31310_e38770) + (assign31310_e38749 * (0.5 * (((-((assign31310_e38753 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign31310_e38767) + (assign31310_e38757 * ((-((assign31310_e38760 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign31310_e38745 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign31310_e38770) + (assign31310_e38749 * (0.5 * (((-((assign31310_e38753 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign31310_e38767) + (assign31310_e38757 * ((-((assign31310_e38760 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31310_e38775;
        var_tmp_dn5 = assign31310_e38775_d_n5;
        var_tmp_dn6 = assign31310_e38775_d_n6;
        var_tmp_dn7 = assign31310_e38775_d_n7;
        var_tmp_dn8 = assign31310_e38775_d_n8;

        let (assign31320_e38795, assign31320_e38795_d_n5, assign31320_e38795_d_n6, assign31320_e38795_d_n7, assign31320_e38795_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard609 == 0.0)) {
        let assign31320_e38788: f64 = (var_v2 * var_fmaxr);
        let assign31320_e38790: f64 = (assign31320_e38788 * var_fmaxr);
        let assign31320_e38792: f64 = (assign31320_e38790 * var_tmp);
        let assign31320_e38793: f64 = (var_cbbtbotd_i * assign31320_e38792);
        (assign31320_e38793, (var_cbbtbotd_i * (((((var_v2 * var_fmaxr_dn5) * var_fmaxr) + (assign31320_e38788 * var_fmaxr_dn5)) * var_tmp) + (assign31320_e38790 * var_tmp_dn5))), (var_cbbtbotd_i * (((((var_v2 * var_fmaxr_dn6) * var_fmaxr) + (assign31320_e38788 * var_fmaxr_dn6)) * var_tmp) + (assign31320_e38790 * var_tmp_dn6))), (var_cbbtbotd_i * (((((var_v2 * var_fmaxr_dn7) * var_fmaxr) + (assign31320_e38788 * var_fmaxr_dn7)) * var_tmp) + (assign31320_e38790 * var_tmp_dn7))), (var_cbbtbotd_i * (((((var_v2 * var_fmaxr_dn8) * var_fmaxr) + (assign31320_e38788 * var_fmaxr_dn8)) * var_tmp) + (assign31320_e38790 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign31320_e38795;
        var_ibbt_dn5 = assign31320_e38795_d_n5;
        var_ibbt_dn6 = assign31320_e38795_d_n6;
        var_ibbt_dn7 = assign31320_e38795_d_n7;
        var_ibbt_dn8 = assign31320_e38795_d_n8;

        let assign31330_e38798: f64 = if var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard613 = assign31330_e38798;

        let (assign31340_e38809, assign31340_e38809_d_n5, assign31340_e38809_d_n6, assign31340_e38809_d_n7, assign31340_e38809_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard613 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign31340_e38809;
        var_fbreakdown_dn5 = assign31340_e38809_d_n5;
        var_fbreakdown_dn6 = assign31340_e38809_d_n6;
        var_fbreakdown_dn7 = assign31340_e38809_d_n7;
        var_fbreakdown_dn8 = assign31340_e38809_d_n8;

        let assign31350_e38812: f64 = (-var_alphaav);
        let assign31350_e38814: f64 = (assign31350_e38812 * var_vbrbotd_i);
        let assign31350_e38815: f64 = if var_vav > assign31350_e38814 { 1.0 } else { 0.0 };
        var_guard614 = assign31350_e38815;

        let assign31360_e38818: f64 = if var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard615 = assign31360_e38818;

        let (assign31370_e38848, assign31370_e38848_d_n5, assign31370_e38848_d_n6, assign31370_e38848_d_n7, assign31370_e38848_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard613 == 0.0)) && (var_guard614 != 0.0)) && (var_guard615 != 0.0)) {
        let assign31370_e38834: f64 = (var_vav * var_vbrinvbot_d);
        let assign31370_e38837: f64 = (var_vav * var_vbrinvbot_d);
        let assign31370_e38838: f64 = (assign31370_e38834 * assign31370_e38837);
        let assign31370_e38841: f64 = (var_vav * var_vbrinvbot_d);
        let assign31370_e38842: f64 = (assign31370_e38838 * assign31370_e38841);
        let assign31370_e38845: f64 = (var_vav * var_vbrinvbot_d);
        let assign31370_e38846: f64 = (assign31370_e38842 * assign31370_e38845);
        (assign31370_e38846, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31370_e38848;
        var_tmp_dn5 = assign31370_e38848_d_n5;
        var_tmp_dn6 = assign31370_e38848_d_n6;
        var_tmp_dn7 = assign31370_e38848_d_n7;
        var_tmp_dn8 = assign31370_e38848_d_n8;

        let (assign31380_e38870, assign31380_e38870_d_n5, assign31380_e38870_d_n6, assign31380_e38870_d_n7, assign31380_e38870_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard613 == 0.0)) && (var_guard614 != 0.0)) && (var_guard615 == 0.0)) {
        let assign31380_e38865: f64 = (var_vav * var_vbrinvbot_d);
        let assign31380_e38866: f64 = (assign31380_e38865).abs();
        let assign31380_e38868: f64 = (assign31380_e38866).powf(var_pbrbotd_i);
        (assign31380_e38868, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31380_e38870;
        var_tmp_dn5 = assign31380_e38870_d_n5;
        var_tmp_dn6 = assign31380_e38870_d_n6;
        var_tmp_dn7 = assign31380_e38870_d_n7;
        var_tmp_dn8 = assign31380_e38870_d_n8;

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
        *var_guard605_slot = var_guard605;
        *var_guard606_slot = var_guard606;
        *var_guard607_slot = var_guard607;
        *var_guard608_slot = var_guard608;
        *var_guard609_slot = var_guard609;
        *var_guard610_slot = var_guard610;
        *var_guard611_slot = var_guard611;
        *var_guard612_slot = var_guard612;
        *var_guard613_slot = var_guard613;
        *var_guard614_slot = var_guard614;
        *var_guard615_slot = var_guard615;
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

    pub(super) fn stamp_transient_block_63(
        p: &Parameters,
        var_alphaav: f64,
        var_atatsti_d: f64,
        var_btatpartsti_d: f64,
        var_csrhstid_i: f64,
        var_ctatstid_i: f64,
        var_fstopbot_d: f64,
        var_ftdsti_d: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard599: f64,
        var_guard613: f64,
        var_guard614: f64,
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
        var_guard616_slot: &mut f64,
        var_guard617_slot: &mut f64,
        var_guard618_slot: &mut f64,
        var_guard619_slot: &mut f64,
        var_guard620_slot: &mut f64,
        var_guard621_slot: &mut f64,
        var_guard622_slot: &mut f64,
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
        let mut var_guard616: f64 = *var_guard616_slot;
        let mut var_guard617: f64 = *var_guard617_slot;
        let mut var_guard618: f64 = *var_guard618_slot;
        let mut var_guard619: f64 = *var_guard619_slot;
        let mut var_guard620: f64 = *var_guard620_slot;
        let mut var_guard621: f64 = *var_guard621_slot;
        let mut var_guard622: f64 = *var_guard622_slot;
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

        let (assign31390_e38888, assign31390_e38888_d_n5, assign31390_e38888_d_n6, assign31390_e38888_d_n7, assign31390_e38888_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard613 == 0.0)) && (var_guard614 != 0.0)) {
        let assign31390_e38885: f64 = (1.0 - var_tmp);
        let assign31390_e38886: f64 = (1.0 / assign31390_e38885);
        (assign31390_e38886, (-((-var_tmp_dn5) / (assign31390_e38885 * assign31390_e38885))), (-((-var_tmp_dn6) / (assign31390_e38885 * assign31390_e38885))), (-((-var_tmp_dn7) / (assign31390_e38885 * assign31390_e38885))), (-((-var_tmp_dn8) / (assign31390_e38885 * assign31390_e38885))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign31390_e38888;
        var_fbreakdown_dn5 = assign31390_e38888_d_n5;
        var_fbreakdown_dn6 = assign31390_e38888_d_n6;
        var_fbreakdown_dn7 = assign31390_e38888_d_n7;
        var_fbreakdown_dn8 = assign31390_e38888_d_n8;

        let (assign31400_e38911, assign31400_e38911_d_n5, assign31400_e38911_d_n6, assign31400_e38911_d_n7, assign31400_e38911_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) && (var_guard613 == 0.0)) && (var_guard614 == 0.0)) {
        let assign31400_e38905: f64 = (var_alphaav * var_vbrbotd_i);
        let assign31400_e38906: f64 = (var_vav + assign31400_e38905);
        let assign31400_e38908: f64 = (assign31400_e38906 * var_slopebot_d);
        let assign31400_e38909: f64 = (var_fstopbot_d + assign31400_e38908);
        (assign31400_e38909, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign31400_e38911;
        var_fbreakdown_dn5 = assign31400_e38911_d_n5;
        var_fbreakdown_dn6 = assign31400_e38911_d_n6;
        var_fbreakdown_dn7 = assign31400_e38911_d_n7;
        var_fbreakdown_dn8 = assign31400_e38911_d_n8;

        let (assign31410_e38930, assign31410_e38930_d_n5, assign31410_e38930_d_n6, assign31410_e38930_d_n7, assign31410_e38930_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard599 == 0.0)) {
        let assign31410_e38921: f64 = (var_id__blk219 + var_isrh);
        let assign31410_e38923: f64 = (assign31410_e38921 + var_itat);
        let assign31410_e38925: f64 = (assign31410_e38923 + var_ibbt);
        let assign31410_e38926: f64 = (p.p29 * assign31410_e38925);
        let assign31410_e38928: f64 = (assign31410_e38926 * var_fbreakdown);
        (assign31410_e38928, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign31410_e38926 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign31410_e38926 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign31410_e38926 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign31410_e38926 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign31410_e38930;
        var_ijunbot_dn5 = assign31410_e38930_d_n5;
        var_ijunbot_dn6 = assign31410_e38930_d_n6;
        var_ijunbot_dn7 = assign31410_e38930_d_n7;
        var_ijunbot_dn8 = assign31410_e38930_d_n8;

        let assign31420_e38933: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard616 = assign31420_e38933;

        let (assign31430_e38941, assign31430_e38941_d_n5, assign31430_e38941_d_n6, assign31430_e38941_d_n7, assign31430_e38941_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign31430_e38941;
        var_ijunsti_dn5 = assign31430_e38941_d_n5;
        var_ijunsti_dn6 = assign31430_e38941_d_n6;
        var_ijunsti_dn7 = assign31430_e38941_d_n7;
        var_ijunsti_dn8 = assign31430_e38941_d_n8;

        let (assign31440_e38952,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) {
        let assign31440_e38950: f64 = (var_idsatsti_d * var_idmult);
        (assign31440_e38950,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign31440_e38952;

        let assign31450_e38959: f64 = if ((var_csrhstid_i == 0.0) && (var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard617 = assign31450_e38959;

        let (assign31460_e38970, assign31460_e38970_d_n5, assign31460_e38970_d_n6, assign31460_e38970_d_n7, assign31460_e38970_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard617 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign31460_e38970;
        var_isrh_dn5 = assign31460_e38970_d_n5;
        var_isrh_dn6 = assign31460_e38970_d_n6;
        var_isrh_dn7 = assign31460_e38970_d_n7;
        var_isrh_dn8 = assign31460_e38970_d_n8;

        let (assign31470_e38984,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard617 == 0.0)) {
        let assign31470_e38982: f64 = (var_vbisti_d - var_vjsrh);
        (assign31470_e38982,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign31470_e38984;

        let (assign31480_e39003,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard617 == 0.0)) {
        let assign31480_e38998: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign31480_e38999: f64 = (1.0 - assign31480_e38998);
        let assign31480_e39000: f64 = (assign31480_e38999).sqrt();
        let assign31480_e39001: f64 = (1.0 - assign31480_e39000);
        (assign31480_e39001,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign31480_e39003;

        let assign31490_e39006: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard618 = assign31490_e39006;

        let (assign31500_e39020,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard617 == 0.0)) && (var_guard618 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign31500_e39020;

        let (assign31510_e39052,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard617 == 0.0)) && (var_guard618 == 0.0)) {
        let assign31510_e39035: f64 = (var_wsrhstep * var_wsrhstep);
        let assign31510_e39037: f64 = (var_wsrhstep).ln();
        let assign31510_e39038: f64 = (assign31510_e39035 * assign31510_e39037);
        let assign31510_e39041: f64 = (1.0 - var_wsrhstep);
        let assign31510_e39042: f64 = (assign31510_e39038 / assign31510_e39041);
        let assign31510_e39044: f64 = (assign31510_e39042 + var_wsrhstep);
        let assign31510_e39048: f64 = (2.0 * var_pstid_i);
        let assign31510_e39049: f64 = (1.0 - assign31510_e39048);
        let assign31510_e39050: f64 = (assign31510_e39044 * assign31510_e39049);
        (assign31510_e39050,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign31510_e39052;

        let (assign31520_e39066,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard617 == 0.0)) {
        let assign31520_e39064: f64 = (var_wsrhstep + var_dwsrh);
        (assign31520_e39064,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign31520_e39066;

        let assign31530_e39069: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard619 = assign31530_e39069;

        let (assign31540_e39086, assign31540_e39086_d_n5, assign31540_e39086_d_n6, assign31540_e39086_d_n7, assign31540_e39086_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard617 == 0.0)) && (var_guard619 != 0.0)) {
        let assign31540_e39083: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign31540_e39084: f64 = (assign31540_e39083).sqrt();
        (assign31540_e39084, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31540_e39086;
        var_tmp_dn5 = assign31540_e39086_d_n5;
        var_tmp_dn6 = assign31540_e39086_d_n6;
        var_tmp_dn7 = assign31540_e39086_d_n7;
        var_tmp_dn8 = assign31540_e39086_d_n8;

        let (assign31550_e39105, assign31550_e39105_d_n5, assign31550_e39105_d_n6, assign31550_e39105_d_n7, assign31550_e39105_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard617 == 0.0)) && (var_guard619 == 0.0)) {
        let assign31550_e39101: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign31550_e39103: f64 = (assign31550_e39101).powf(var_pstid_i);
        (assign31550_e39103, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign31550_e39105;
        var_tmp_dn5 = assign31550_e39105_d_n5;
        var_tmp_dn6 = assign31550_e39105_d_n6;
        var_tmp_dn7 = assign31550_e39105_d_n7;
        var_tmp_dn8 = assign31550_e39105_d_n8;

        let (assign31560_e39119, assign31560_e39119_d_n5, assign31560_e39119_d_n6, assign31560_e39119_d_n7, assign31560_e39119_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard617 == 0.0)) {
        let assign31560_e39117: f64 = (var_wdepnulrsti_d * var_tmp);
        (assign31560_e39117, (var_wdepnulrsti_d * var_tmp_dn5), (var_wdepnulrsti_d * var_tmp_dn6), (var_wdepnulrsti_d * var_tmp_dn7), (var_wdepnulrsti_d * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign31560_e39119;
        var_wdep_dn5 = assign31560_e39119_d_n5;
        var_wdep_dn6 = assign31560_e39119_d_n6;
        var_wdep_dn7 = assign31560_e39119_d_n7;
        var_wdep_dn8 = assign31560_e39119_d_n8;

        let (assign31570_e39137, assign31570_e39137_d_n5, assign31570_e39137_d_n6, assign31570_e39137_d_n7, assign31570_e39137_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard617 == 0.0)) {
        let assign31570_e39132: f64 = (var_zinv - 1.0);
        let assign31570_e39134: f64 = (assign31570_e39132 * var_wdep);
        let assign31570_e39135: f64 = (var_ftdsti_d * assign31570_e39134);
        (assign31570_e39135, (var_ftdsti_d * (assign31570_e39132 * var_wdep_dn5)), (var_ftdsti_d * (assign31570_e39132 * var_wdep_dn6)), (var_ftdsti_d * (assign31570_e39132 * var_wdep_dn7)), (var_ftdsti_d * (assign31570_e39132 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign31570_e39137;
        var_asrh_dn5 = assign31570_e39137_d_n5;
        var_asrh_dn6 = assign31570_e39137_d_n6;
        var_asrh_dn7 = assign31570_e39137_d_n7;
        var_asrh_dn8 = assign31570_e39137_d_n8;

        let (assign31580_e39153, assign31580_e39153_d_n5, assign31580_e39153_d_n6, assign31580_e39153_d_n7, assign31580_e39153_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard617 == 0.0)) {
        let assign31580_e39150: f64 = (var_asrh * var_wsrh);
        let assign31580_e39151: f64 = (var_csrhstid_i * assign31580_e39150);
        (assign31580_e39151, (var_csrhstid_i * (var_asrh_dn5 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn6 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn7 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign31580_e39153;
        var_isrh_dn5 = assign31580_e39153_d_n5;
        var_isrh_dn6 = assign31580_e39153_d_n6;
        var_isrh_dn7 = assign31580_e39153_d_n7;
        var_isrh_dn8 = assign31580_e39153_d_n8;

        let assign31590_e39156: f64 = if var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard620 = assign31590_e39156;

        let (assign31600_e39167, assign31600_e39167_d_n5, assign31600_e39167_d_n6, assign31600_e39167_d_n7, assign31600_e39167_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign31600_e39167;
        var_itat_dn5 = assign31600_e39167_d_n5;
        var_itat_dn6 = assign31600_e39167_d_n6;
        var_itat_dn7 = assign31600_e39167_d_n7;
        var_itat_dn8 = assign31600_e39167_d_n8;

        let (assign31610_e39185, assign31610_e39185_d_n5, assign31610_e39185_d_n6, assign31610_e39185_d_n7, assign31610_e39185_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) {
        let assign31610_e39180: f64 = (var_wdep * var_one_minus_psti_d);
        let assign31610_e39182: f64 = (assign31610_e39180 / var_vbi_minus_vjsrh);
        let assign31610_e39183: f64 = (var_btatpartsti_d * assign31610_e39182);
        (assign31610_e39183, (var_btatpartsti_d * ((var_wdep_dn5 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn6 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn7 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn8 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign31610_e39185;
        var_btat_dn5 = assign31610_e39185_d_n5;
        var_btat_dn6 = assign31610_e39185_d_n6;
        var_btat_dn7 = assign31610_e39185_d_n7;
        var_btat_dn8 = assign31610_e39185_d_n8;

        let (assign31620_e39201, assign31620_e39201_d_n5, assign31620_e39201_d_n6, assign31620_e39201_d_n7, assign31620_e39201_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) {
        let assign31620_e39197: f64 = (0.666666666666667 * var_atatsti_d);
        let assign31620_e39199: f64 = (assign31620_e39197 / var_btat);
        (assign31620_e39199, (-((assign31620_e39197 * var_btat_dn5) / (var_btat * var_btat))), (-((assign31620_e39197 * var_btat_dn6) / (var_btat * var_btat))), (-((assign31620_e39197 * var_btat_dn7) / (var_btat * var_btat))), (-((assign31620_e39197 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign31620_e39201;
        var_twoatatoverthreebtat_dn5 = assign31620_e39201_d_n5;
        var_twoatatoverthreebtat_dn6 = assign31620_e39201_d_n6;
        var_twoatatoverthreebtat_dn7 = assign31620_e39201_d_n7;
        var_twoatatoverthreebtat_dn8 = assign31620_e39201_d_n8;

        let (assign31630_e39215, assign31630_e39215_d_n5, assign31630_e39215_d_n6, assign31630_e39215_d_n7, assign31630_e39215_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) {
        let assign31630_e39213: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign31630_e39213, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign31630_e39215;
        var_umaxbeforelimiting_dn5 = assign31630_e39215_d_n5;
        var_umaxbeforelimiting_dn6 = assign31630_e39215_d_n6;
        var_umaxbeforelimiting_dn7 = assign31630_e39215_d_n7;
        var_umaxbeforelimiting_dn8 = assign31630_e39215_d_n8;

        let (assign31640_e39236, assign31640_e39236_d_n5, assign31640_e39236_d_n6, assign31640_e39236_d_n7, assign31640_e39236_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) {
        let assign31640_e39227: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign31640_e39230: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign31640_e39232: f64 = (assign31640_e39230 + 1.0);
        let assign31640_e39233: f64 = (assign31640_e39227 / assign31640_e39232);
        let assign31640_e39234: f64 = (assign31640_e39233).sqrt();
        (assign31640_e39234, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign31640_e39232) - (assign31640_e39227 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign31640_e39232 * assign31640_e39232)) / (2.0 * assign31640_e39234)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign31640_e39232) - (assign31640_e39227 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign31640_e39232 * assign31640_e39232)) / (2.0 * assign31640_e39234)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign31640_e39232) - (assign31640_e39227 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign31640_e39232 * assign31640_e39232)) / (2.0 * assign31640_e39234)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign31640_e39232) - (assign31640_e39227 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign31640_e39232 * assign31640_e39232)) / (2.0 * assign31640_e39234)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign31640_e39236;
        var_umax_dn5 = assign31640_e39236_d_n5;
        var_umax_dn6 = assign31640_e39236_d_n6;
        var_umax_dn7 = assign31640_e39236_d_n7;
        var_umax_dn8 = assign31640_e39236_d_n8;

        let (assign31650_e39249, assign31650_e39249_d_n5, assign31650_e39249_d_n6, assign31650_e39249_d_n7, assign31650_e39249_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) {
        let assign31650_e39247: f64 = (var_umax).sqrt();
        (assign31650_e39247, (var_umax_dn5 / (2.0 * assign31650_e39247)), (var_umax_dn6 / (2.0 * assign31650_e39247)), (var_umax_dn7 / (2.0 * assign31650_e39247)), (var_umax_dn8 / (2.0 * assign31650_e39247)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign31650_e39249;
        var_sqrtumax_dn5 = assign31650_e39249_d_n5;
        var_sqrtumax_dn6 = assign31650_e39249_d_n6;
        var_sqrtumax_dn7 = assign31650_e39249_d_n7;
        var_sqrtumax_dn8 = assign31650_e39249_d_n8;

        let (assign31660_e39263, assign31660_e39263_d_n5, assign31660_e39263_d_n6, assign31660_e39263_d_n7, assign31660_e39263_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) {
        let assign31660_e39261: f64 = (var_umax * var_sqrtumax);
        (assign31660_e39261, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign31660_e39263;
        var_umaxpoweronepointfive_dn5 = assign31660_e39263_d_n5;
        var_umaxpoweronepointfive_dn6 = assign31660_e39263_d_n6;
        var_umaxpoweronepointfive_dn7 = assign31660_e39263_d_n7;
        var_umaxpoweronepointfive_dn8 = assign31660_e39263_d_n8;

        let assign31670_e39265: f64 = (-var_pstid_i);
        let assign31670_e39267: f64 = (assign31670_e39265 * var_one_over_one_minus_psti_d);
        let assign31670_e39269: f64 = (-1.0);
        let assign31670_e39270: f64 = if assign31670_e39267 == assign31670_e39269 { 1.0 } else { 0.0 };
        var_guard621 = assign31670_e39270;

        let (assign31680_e39290, assign31680_e39290_d_n5, assign31680_e39290_d_n6, assign31680_e39290_d_n7, assign31680_e39290_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) && (var_guard621 != 0.0)) {
        let assign31680_e39286: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign31680_e39287: f64 = (1.0 + assign31680_e39286);
        let assign31680_e39288: f64 = (1.0 / assign31680_e39287);
        (assign31680_e39288, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign31680_e39287 * assign31680_e39287))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign31680_e39287 * assign31680_e39287))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign31680_e39287 * assign31680_e39287))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign31680_e39287 * assign31680_e39287))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign31680_e39290;
        var_wgamma_dn5 = assign31680_e39290_d_n5;
        var_wgamma_dn6 = assign31680_e39290_d_n6;
        var_wgamma_dn7 = assign31680_e39290_d_n7;
        var_wgamma_dn8 = assign31680_e39290_d_n8;

        let (assign31690_e39314, assign31690_e39314_d_n5, assign31690_e39314_d_n6, assign31690_e39314_d_n7, assign31690_e39314_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) && (var_guard621 == 0.0)) {
        let assign31690_e39306: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign31690_e39307: f64 = (1.0 + assign31690_e39306);
        let assign31690_e39309: f64 = (-var_pstid_i);
        let assign31690_e39311: f64 = (assign31690_e39309 * var_one_over_one_minus_psti_d);
        let assign31690_e39312: f64 = (assign31690_e39307).powf(assign31690_e39311);
        (assign31690_e39312, if 0.0 == 0.0 && ((assign31690_e39311) as f64).is_finite() && ((assign31690_e39311) as f64).fract() == 0.0 { if assign31690_e39311 == 0.0 { 0.0 } else { (assign31690_e39311 * ((assign31690_e39307).powf(assign31690_e39311 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign31690_e39312 * (assign31690_e39311 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign31690_e39307))) }, if 0.0 == 0.0 && ((assign31690_e39311) as f64).is_finite() && ((assign31690_e39311) as f64).fract() == 0.0 { if assign31690_e39311 == 0.0 { 0.0 } else { (assign31690_e39311 * ((assign31690_e39307).powf(assign31690_e39311 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign31690_e39312 * (assign31690_e39311 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign31690_e39307))) }, if 0.0 == 0.0 && ((assign31690_e39311) as f64).is_finite() && ((assign31690_e39311) as f64).fract() == 0.0 { if assign31690_e39311 == 0.0 { 0.0 } else { (assign31690_e39311 * ((assign31690_e39307).powf(assign31690_e39311 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign31690_e39312 * (assign31690_e39311 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign31690_e39307))) }, if 0.0 == 0.0 && ((assign31690_e39311) as f64).is_finite() && ((assign31690_e39311) as f64).fract() == 0.0 { if assign31690_e39311 == 0.0 { 0.0 } else { (assign31690_e39311 * ((assign31690_e39307).powf(assign31690_e39311 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign31690_e39312 * (assign31690_e39311 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign31690_e39307))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign31690_e39314;
        var_wgamma_dn5 = assign31690_e39314_d_n5;
        var_wgamma_dn6 = assign31690_e39314_d_n6;
        var_wgamma_dn7 = assign31690_e39314_d_n7;
        var_wgamma_dn8 = assign31690_e39314_d_n8;

        let (assign31700_e39332, assign31700_e39332_d_n5, assign31700_e39332_d_n6, assign31700_e39332_d_n7, assign31700_e39332_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) {
        let assign31700_e39326: f64 = (var_wsrh * var_wgamma);
        let assign31700_e39329: f64 = (var_wsrh + var_wgamma);
        let assign31700_e39330: f64 = (assign31700_e39326 / assign31700_e39329);
        (assign31700_e39330, ((((var_wsrh * var_wgamma_dn5) * assign31700_e39329) - (assign31700_e39326 * var_wgamma_dn5)) / (assign31700_e39329 * assign31700_e39329)), ((((var_wsrh * var_wgamma_dn6) * assign31700_e39329) - (assign31700_e39326 * var_wgamma_dn6)) / (assign31700_e39329 * assign31700_e39329)), ((((var_wsrh * var_wgamma_dn7) * assign31700_e39329) - (assign31700_e39326 * var_wgamma_dn7)) / (assign31700_e39329 * assign31700_e39329)), ((((var_wsrh * var_wgamma_dn8) * assign31700_e39329) - (assign31700_e39326 * var_wgamma_dn8)) / (assign31700_e39329 * assign31700_e39329)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign31700_e39332;
        var_wtat_dn5 = assign31700_e39332_d_n5;
        var_wtat_dn6 = assign31700_e39332_d_n6;
        var_wtat_dn7 = assign31700_e39332_d_n7;
        var_wtat_dn8 = assign31700_e39332_d_n8;

        let (assign31710_e39349, assign31710_e39349_d_n5, assign31710_e39349_d_n6, assign31710_e39349_d_n7, assign31710_e39349_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) {
        let assign31710_e39345: f64 = (var_btat / var_sqrtumax);
        let assign31710_e39346: f64 = (0.375 * assign31710_e39345);
        let assign31710_e39347: f64 = (assign31710_e39346).sqrt();
        (assign31710_e39347, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign31710_e39347)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign31710_e39347)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign31710_e39347)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign31710_e39347)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign31710_e39349;
        var_ktat_dn5 = assign31710_e39349_d_n5;
        var_ktat_dn6 = assign31710_e39349_d_n6;
        var_ktat_dn7 = assign31710_e39349_d_n7;
        var_ktat_dn8 = assign31710_e39349_d_n8;

        let (assign31720_e39367, assign31720_e39367_d_n5, assign31720_e39367_d_n6, assign31720_e39367_d_n7, assign31720_e39367_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) {
        let assign31720_e39362: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign31720_e39363: f64 = (2.0 * assign31720_e39362);
        let assign31720_e39365: f64 = (assign31720_e39363 - var_umax);
        (assign31720_e39365, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign31720_e39367;
        var_ltat_dn5 = assign31720_e39367_d_n5;
        var_ltat_dn6 = assign31720_e39367_d_n6;
        var_ltat_dn7 = assign31720_e39367_d_n7;
        var_ltat_dn8 = assign31720_e39367_d_n8;

        let (assign31730_e39393, assign31730_e39393_d_n5, assign31730_e39393_d_n6, assign31730_e39393_d_n7, assign31730_e39393_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) {
        let assign31730_e39379: f64 = (var_atatsti_d * var_twoatatoverthreebtat);
        let assign31730_e39381: f64 = (assign31730_e39379 * var_sqrtumax);
        let assign31730_e39384: f64 = (var_atatsti_d * var_umax);
        let assign31730_e39385: f64 = (assign31730_e39381 - assign31730_e39384);
        let assign31730_e39389: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign31730_e39390: f64 = (0.5 * assign31730_e39389);
        let assign31730_e39391: f64 = (assign31730_e39385 + assign31730_e39390);
        (assign31730_e39391, (((((var_atatsti_d * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign31730_e39379 * var_sqrtumax_dn5)) - (var_atatsti_d * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign31730_e39379 * var_sqrtumax_dn6)) - (var_atatsti_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign31730_e39379 * var_sqrtumax_dn7)) - (var_atatsti_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign31730_e39379 * var_sqrtumax_dn8)) - (var_atatsti_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign31730_e39393;
        var_mtat_dn5 = assign31730_e39393_d_n5;
        var_mtat_dn6 = assign31730_e39393_d_n6;
        var_mtat_dn7 = assign31730_e39393_d_n7;
        var_mtat_dn8 = assign31730_e39393_d_n8;

        let (assign31740_e39409, assign31740_e39409_d_n5, assign31740_e39409_d_n6, assign31740_e39409_d_n7, assign31740_e39409_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) {
        let assign31740_e39405: f64 = (var_ltat - 1.0);
        let assign31740_e39407: f64 = (assign31740_e39405 * var_ktat);
        (assign31740_e39407, ((var_ltat_dn5 * var_ktat) + (assign31740_e39405 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign31740_e39405 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign31740_e39405 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign31740_e39405 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign31740_e39409;
        var_xerfc_dn5 = assign31740_e39409_d_n5;
        var_xerfc_dn6 = assign31740_e39409_d_n6;
        var_xerfc_dn7 = assign31740_e39409_d_n7;
        var_xerfc_dn8 = assign31740_e39409_d_n8;

        let (assign31750_e39423, assign31750_e39423_d_n5, assign31750_e39423_d_n6, assign31750_e39423_d_n7, assign31750_e39423_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) {
        let assign31750_e39421: f64 = (var_xerfc * var_xerfc);
        (assign31750_e39421, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign31750_e39423;
        var_ysq_dn5 = assign31750_e39423_d_n5;
        var_ysq_dn6 = assign31750_e39423_d_n6;
        var_ysq_dn7 = assign31750_e39423_d_n7;
        var_ysq_dn8 = assign31750_e39423_d_n8;

        let assign31760_e39426: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard622 = assign31760_e39426;

        let (assign31770_e39446, assign31770_e39446_d_n5, assign31770_e39446_d_n6, assign31770_e39446_d_n7, assign31770_e39446_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) && (var_guard622 != 0.0)) {
        let assign31770_e39442: f64 = (var_perfc * var_xerfc);
        let assign31770_e39443: f64 = (1.0 + assign31770_e39442);
        let assign31770_e39444: f64 = (1.0 / assign31770_e39443);
        (assign31770_e39444, (-((var_perfc * var_xerfc_dn5) / (assign31770_e39443 * assign31770_e39443))), (-((var_perfc * var_xerfc_dn6) / (assign31770_e39443 * assign31770_e39443))), (-((var_perfc * var_xerfc_dn7) / (assign31770_e39443 * assign31770_e39443))), (-((var_perfc * var_xerfc_dn8) / (assign31770_e39443 * assign31770_e39443))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign31770_e39446;
        var_terfc_dn5 = assign31770_e39446_d_n5;
        var_terfc_dn6 = assign31770_e39446_d_n6;
        var_terfc_dn7 = assign31770_e39446_d_n7;
        var_terfc_dn8 = assign31770_e39446_d_n8;

        let (assign31780_e39467, assign31780_e39467_d_n5, assign31780_e39467_d_n6, assign31780_e39467_d_n7, assign31780_e39467_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard616 == 0.0)) && (var_guard620 == 0.0)) && (var_guard622 == 0.0)) {
        let assign31780_e39463: f64 = (var_perfc * var_xerfc);
        let assign31780_e39464: f64 = (1.0 - assign31780_e39463);
        let assign31780_e39465: f64 = (1.0 / assign31780_e39464);
        (assign31780_e39465, (-((-(var_perfc * var_xerfc_dn5)) / (assign31780_e39464 * assign31780_e39464))), (-((-(var_perfc * var_xerfc_dn6)) / (assign31780_e39464 * assign31780_e39464))), (-((-(var_perfc * var_xerfc_dn7)) / (assign31780_e39464 * assign31780_e39464))), (-((-(var_perfc * var_xerfc_dn8)) / (assign31780_e39464 * assign31780_e39464))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign31780_e39467;
        var_terfc_dn5 = assign31780_e39467_d_n5;
        var_terfc_dn6 = assign31780_e39467_d_n6;
        var_terfc_dn7 = assign31780_e39467_d_n7;
        var_terfc_dn8 = assign31780_e39467_d_n8;

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
        *var_guard616_slot = var_guard616;
        *var_guard617_slot = var_guard617;
        *var_guard618_slot = var_guard618;
        *var_guard619_slot = var_guard619;
        *var_guard620_slot = var_guard620;
        *var_guard621_slot = var_guard621;
        *var_guard622_slot = var_guard622;
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
}
