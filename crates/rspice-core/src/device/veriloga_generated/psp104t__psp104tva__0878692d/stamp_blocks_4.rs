#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_64(
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatgat_d: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_btat_dn9: f64,
        var_cbbtgatd_i: f64,
        var_cerfc: f64,
        var_ctatgatd_i: f64,
        var_fbbtgat_d: f64,
        var_fbbtgat_d_dn6: f64,
        var_fbbtgat_d_dn7: f64,
        var_fbbtgat_d_dn8: f64,
        var_fbbtgat_d_dn9: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard626: f64,
        var_guard630: f64,
        var_guard631: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_perfc: f64,
        var_pgatd_i: f64,
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
        var_vbirgatd_i: f64,
        var_vbirgatinv_d: f64,
        var_wdepnulrinvgat_d: f64,
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
        var_guard632_slot: &mut f64,
        var_guard633_slot: &mut f64,
        var_guard634_slot: &mut f64,
        var_guard635_slot: &mut f64,
        var_guard636_slot: &mut f64,
        var_guard637_slot: &mut f64,
        var_guard638_slot: &mut f64,
        var_guard639_slot: &mut f64,
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
        let mut var_guard632: f64 = *var_guard632_slot;
        let mut var_guard633: f64 = *var_guard633_slot;
        let mut var_guard634: f64 = *var_guard634_slot;
        let mut var_guard635: f64 = *var_guard635_slot;
        let mut var_guard636: f64 = *var_guard636_slot;
        let mut var_guard637: f64 = *var_guard637_slot;
        let mut var_guard638: f64 = *var_guard638_slot;
        let mut var_guard639: f64 = *var_guard639_slot;
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

        let (assign31310_e39696, assign31310_e39696_d_n6, assign31310_e39696_d_n7, assign31310_e39696_d_n8, assign31310_e39696_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) && (var_guard631 == 0.0)) {
        let assign31310_e39688: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign31310_e39689: f64 = (1.0 + assign31310_e39688);
        let assign31310_e39691: f64 = (-var_pgatd_i);
        let assign31310_e39693: f64 = (assign31310_e39691 * var_one_over_one_minus_pgat_d);
        let assign31310_e39694: f64 = (assign31310_e39689).powf(assign31310_e39693);
        (assign31310_e39694, if 0.0 == 0.0 && ((assign31310_e39693) as f64).is_finite() && ((assign31310_e39693) as f64).fract() == 0.0 { if assign31310_e39693 == 0.0 { 0.0 } else { (assign31310_e39693 * ((assign31310_e39689).powf(assign31310_e39693 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign31310_e39694 * (assign31310_e39693 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign31310_e39689))) }, if 0.0 == 0.0 && ((assign31310_e39693) as f64).is_finite() && ((assign31310_e39693) as f64).fract() == 0.0 { if assign31310_e39693 == 0.0 { 0.0 } else { (assign31310_e39693 * ((assign31310_e39689).powf(assign31310_e39693 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign31310_e39694 * (assign31310_e39693 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign31310_e39689))) }, if 0.0 == 0.0 && ((assign31310_e39693) as f64).is_finite() && ((assign31310_e39693) as f64).fract() == 0.0 { if assign31310_e39693 == 0.0 { 0.0 } else { (assign31310_e39693 * ((assign31310_e39689).powf(assign31310_e39693 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign31310_e39694 * (assign31310_e39693 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign31310_e39689))) }, if 0.0 == 0.0 && ((assign31310_e39693) as f64).is_finite() && ((assign31310_e39693) as f64).fract() == 0.0 { if assign31310_e39693 == 0.0 { 0.0 } else { (assign31310_e39693 * ((assign31310_e39689).powf(assign31310_e39693 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign31310_e39694 * (assign31310_e39693 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign31310_e39689))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign31310_e39696;
        var_wgamma_dn6 = assign31310_e39696_d_n6;
        var_wgamma_dn7 = assign31310_e39696_d_n7;
        var_wgamma_dn8 = assign31310_e39696_d_n8;
        var_wgamma_dn9 = assign31310_e39696_d_n9;

        let (assign31320_e39714, assign31320_e39714_d_n6, assign31320_e39714_d_n7, assign31320_e39714_d_n8, assign31320_e39714_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) {
        let assign31320_e39708: f64 = (var_wsrh * var_wgamma);
        let assign31320_e39711: f64 = (var_wsrh + var_wgamma);
        let assign31320_e39712: f64 = (assign31320_e39708 / assign31320_e39711);
        (assign31320_e39712, ((((var_wsrh * var_wgamma_dn6) * assign31320_e39711) - (assign31320_e39708 * var_wgamma_dn6)) / (assign31320_e39711 * assign31320_e39711)), ((((var_wsrh * var_wgamma_dn7) * assign31320_e39711) - (assign31320_e39708 * var_wgamma_dn7)) / (assign31320_e39711 * assign31320_e39711)), ((((var_wsrh * var_wgamma_dn8) * assign31320_e39711) - (assign31320_e39708 * var_wgamma_dn8)) / (assign31320_e39711 * assign31320_e39711)), ((((var_wsrh * var_wgamma_dn9) * assign31320_e39711) - (assign31320_e39708 * var_wgamma_dn9)) / (assign31320_e39711 * assign31320_e39711)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign31320_e39714;
        var_wtat_dn6 = assign31320_e39714_d_n6;
        var_wtat_dn7 = assign31320_e39714_d_n7;
        var_wtat_dn8 = assign31320_e39714_d_n8;
        var_wtat_dn9 = assign31320_e39714_d_n9;

        let (assign31330_e39731, assign31330_e39731_d_n6, assign31330_e39731_d_n7, assign31330_e39731_d_n8, assign31330_e39731_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) {
        let assign31330_e39727: f64 = (var_btat / var_sqrtumax);
        let assign31330_e39728: f64 = (0.375 * assign31330_e39727);
        let assign31330_e39729: f64 = (assign31330_e39728).sqrt();
        (assign31330_e39729, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign31330_e39729)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign31330_e39729)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign31330_e39729)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign31330_e39729)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign31330_e39731;
        var_ktat_dn6 = assign31330_e39731_d_n6;
        var_ktat_dn7 = assign31330_e39731_d_n7;
        var_ktat_dn8 = assign31330_e39731_d_n8;
        var_ktat_dn9 = assign31330_e39731_d_n9;

        let (assign31340_e39749, assign31340_e39749_d_n6, assign31340_e39749_d_n7, assign31340_e39749_d_n8, assign31340_e39749_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) {
        let assign31340_e39744: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign31340_e39745: f64 = (2.0 * assign31340_e39744);
        let assign31340_e39747: f64 = (assign31340_e39745 - var_umax);
        (assign31340_e39747, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign31340_e39749;
        var_ltat_dn6 = assign31340_e39749_d_n6;
        var_ltat_dn7 = assign31340_e39749_d_n7;
        var_ltat_dn8 = assign31340_e39749_d_n8;
        var_ltat_dn9 = assign31340_e39749_d_n9;

        let (assign31350_e39775, assign31350_e39775_d_n6, assign31350_e39775_d_n7, assign31350_e39775_d_n8, assign31350_e39775_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) {
        let assign31350_e39761: f64 = (var_atatgat_d * var_twoatatoverthreebtat);
        let assign31350_e39763: f64 = (assign31350_e39761 * var_sqrtumax);
        let assign31350_e39766: f64 = (var_atatgat_d * var_umax);
        let assign31350_e39767: f64 = (assign31350_e39763 - assign31350_e39766);
        let assign31350_e39771: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign31350_e39772: f64 = (0.5 * assign31350_e39771);
        let assign31350_e39773: f64 = (assign31350_e39767 + assign31350_e39772);
        (assign31350_e39773, (((((var_atatgat_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign31350_e39761 * var_sqrtumax_dn6)) - (var_atatgat_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign31350_e39761 * var_sqrtumax_dn7)) - (var_atatgat_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign31350_e39761 * var_sqrtumax_dn8)) - (var_atatgat_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign31350_e39761 * var_sqrtumax_dn9)) - (var_atatgat_d * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign31350_e39775;
        var_mtat_dn6 = assign31350_e39775_d_n6;
        var_mtat_dn7 = assign31350_e39775_d_n7;
        var_mtat_dn8 = assign31350_e39775_d_n8;
        var_mtat_dn9 = assign31350_e39775_d_n9;

        let (assign31360_e39791, assign31360_e39791_d_n6, assign31360_e39791_d_n7, assign31360_e39791_d_n8, assign31360_e39791_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) {
        let assign31360_e39787: f64 = (var_ltat - 1.0);
        let assign31360_e39789: f64 = (assign31360_e39787 * var_ktat);
        (assign31360_e39789, ((var_ltat_dn6 * var_ktat) + (assign31360_e39787 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign31360_e39787 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign31360_e39787 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign31360_e39787 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign31360_e39791;
        var_xerfc_dn6 = assign31360_e39791_d_n6;
        var_xerfc_dn7 = assign31360_e39791_d_n7;
        var_xerfc_dn8 = assign31360_e39791_d_n8;
        var_xerfc_dn9 = assign31360_e39791_d_n9;

        let (assign31370_e39805, assign31370_e39805_d_n6, assign31370_e39805_d_n7, assign31370_e39805_d_n8, assign31370_e39805_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) {
        let assign31370_e39803: f64 = (var_xerfc * var_xerfc);
        (assign31370_e39803, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign31370_e39805;
        var_ysq_dn6 = assign31370_e39805_d_n6;
        var_ysq_dn7 = assign31370_e39805_d_n7;
        var_ysq_dn8 = assign31370_e39805_d_n8;
        var_ysq_dn9 = assign31370_e39805_d_n9;

        let assign31380_e39808: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard632 = assign31380_e39808;

        let (assign31390_e39828, assign31390_e39828_d_n6, assign31390_e39828_d_n7, assign31390_e39828_d_n8, assign31390_e39828_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) && (var_guard632 != 0.0)) {
        let assign31390_e39824: f64 = (var_perfc * var_xerfc);
        let assign31390_e39825: f64 = (1.0 + assign31390_e39824);
        let assign31390_e39826: f64 = (1.0 / assign31390_e39825);
        (assign31390_e39826, (-((var_perfc * var_xerfc_dn6) / (assign31390_e39825 * assign31390_e39825))), (-((var_perfc * var_xerfc_dn7) / (assign31390_e39825 * assign31390_e39825))), (-((var_perfc * var_xerfc_dn8) / (assign31390_e39825 * assign31390_e39825))), (-((var_perfc * var_xerfc_dn9) / (assign31390_e39825 * assign31390_e39825))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign31390_e39828;
        var_terfc_dn6 = assign31390_e39828_d_n6;
        var_terfc_dn7 = assign31390_e39828_d_n7;
        var_terfc_dn8 = assign31390_e39828_d_n8;
        var_terfc_dn9 = assign31390_e39828_d_n9;

        let (assign31400_e39849, assign31400_e39849_d_n6, assign31400_e39849_d_n7, assign31400_e39849_d_n8, assign31400_e39849_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) && (var_guard632 == 0.0)) {
        let assign31400_e39845: f64 = (var_perfc * var_xerfc);
        let assign31400_e39846: f64 = (1.0 - assign31400_e39845);
        let assign31400_e39847: f64 = (1.0 / assign31400_e39846);
        (assign31400_e39847, (-((-(var_perfc * var_xerfc_dn6)) / (assign31400_e39846 * assign31400_e39846))), (-((-(var_perfc * var_xerfc_dn7)) / (assign31400_e39846 * assign31400_e39846))), (-((-(var_perfc * var_xerfc_dn8)) / (assign31400_e39846 * assign31400_e39846))), (-((-(var_perfc * var_xerfc_dn9)) / (assign31400_e39846 * assign31400_e39846))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign31400_e39849;
        var_terfc_dn6 = assign31400_e39849_d_n6;
        var_terfc_dn7 = assign31400_e39849_d_n7;
        var_terfc_dn8 = assign31400_e39849_d_n8;
        var_terfc_dn9 = assign31400_e39849_d_n9;

        let assign31410_e39851: f64 = (-var_ysq);
        let assign31410_e39853: f64 = (assign31410_e39851 + var_mtat);
        let assign31410_e39855: f64 = (-230.25850929940458);
        let assign31410_e39856: f64 = if assign31410_e39853 > assign31410_e39855 { 1.0 } else { 0.0 };
        var_guard633 = assign31410_e39856;

        let (assign31420_e39874, assign31420_e39874_d_n6, assign31420_e39874_d_n7, assign31420_e39874_d_n8, assign31420_e39874_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) && (var_guard633 != 0.0)) {
        let assign31420_e39869: f64 = (-var_ysq);
        let assign31420_e39871: f64 = (assign31420_e39869 + var_mtat);
        let assign31420_e39872: f64 = (assign31420_e39871).exp();
        (assign31420_e39872, (assign31420_e39872 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign31420_e39872 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign31420_e39872 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign31420_e39872 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign31420_e39874;
        var_tmp_dn6 = assign31420_e39874_d_n6;
        var_tmp_dn7 = assign31420_e39874_d_n7;
        var_tmp_dn8 = assign31420_e39874_d_n8;
        var_tmp_dn9 = assign31420_e39874_d_n9;

        let (assign31430_e39923, assign31430_e39923_d_n6, assign31430_e39923_d_n7, assign31430_e39923_d_n8, assign31430_e39923_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) && (var_guard633 == 0.0)) {
        let assign31430_e39890: f64 = (-230.25850929940458);
        let assign31430_e39892: f64 = (-var_ysq);
        let assign31430_e39894: f64 = (assign31430_e39892 + var_mtat);
        let assign31430_e39895: f64 = (assign31430_e39890 - assign31430_e39894);
        let assign31430_e39899: f64 = (-230.25850929940458);
        let assign31430_e39901: f64 = (-var_ysq);
        let assign31430_e39903: f64 = (assign31430_e39901 + var_mtat);
        let assign31430_e39904: f64 = (assign31430_e39899 - assign31430_e39903);
        let assign31430_e39907: f64 = (-230.25850929940458);
        let assign31430_e39909: f64 = (-var_ysq);
        let assign31430_e39911: f64 = (assign31430_e39909 + var_mtat);
        let assign31430_e39912: f64 = (assign31430_e39907 - assign31430_e39911);
        let assign31430_e39914: f64 = (assign31430_e39912 * 0.3333333333333333);
        let assign31430_e39915: f64 = (1.0 + assign31430_e39914);
        let assign31430_e39916: f64 = (assign31430_e39904 * assign31430_e39915);
        let assign31430_e39917: f64 = (0.5 * assign31430_e39916);
        let assign31430_e39918: f64 = (1.0 + assign31430_e39917);
        let assign31430_e39919: f64 = (assign31430_e39895 * assign31430_e39918);
        let assign31430_e39920: f64 = (1.0 + assign31430_e39919);
        let assign31430_e39921: f64 = (1e-100 / assign31430_e39920);
        (assign31430_e39921, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign31430_e39918) + (assign31430_e39895 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign31430_e39915) + (assign31430_e39904 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign31430_e39920 * assign31430_e39920))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign31430_e39918) + (assign31430_e39895 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign31430_e39915) + (assign31430_e39904 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign31430_e39920 * assign31430_e39920))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign31430_e39918) + (assign31430_e39895 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign31430_e39915) + (assign31430_e39904 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign31430_e39920 * assign31430_e39920))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign31430_e39918) + (assign31430_e39895 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign31430_e39915) + (assign31430_e39904 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign31430_e39920 * assign31430_e39920))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign31430_e39923;
        var_tmp_dn6 = assign31430_e39923_d_n6;
        var_tmp_dn7 = assign31430_e39923_d_n7;
        var_tmp_dn8 = assign31430_e39923_d_n8;
        var_tmp_dn9 = assign31430_e39923_d_n9;

        let (assign31440_e39953, assign31440_e39953_d_n6, assign31440_e39953_d_n7, assign31440_e39953_d_n8, assign31440_e39953_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) {
        let assign31440_e39935: f64 = (0.29214664 * var_terfc);
        let assign31440_e39939: f64 = (var_terfc * var_terfc);
        let assign31440_e39940: f64 = (var_berfc * assign31440_e39939);
        let assign31440_e39941: f64 = (assign31440_e39935 + assign31440_e39940);
        let assign31440_e39945: f64 = (var_terfc * var_terfc);
        let assign31440_e39947: f64 = (assign31440_e39945 * var_terfc);
        let assign31440_e39948: f64 = (var_cerfc * assign31440_e39947);
        let assign31440_e39949: f64 = (assign31440_e39941 + assign31440_e39948);
        let assign31440_e39951: f64 = (assign31440_e39949 * var_tmp);
        (assign31440_e39951, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign31440_e39945 * var_terfc_dn6)))) * var_tmp) + (assign31440_e39949 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign31440_e39945 * var_terfc_dn7)))) * var_tmp) + (assign31440_e39949 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign31440_e39945 * var_terfc_dn8)))) * var_tmp) + (assign31440_e39949 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign31440_e39945 * var_terfc_dn9)))) * var_tmp) + (assign31440_e39949 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign31440_e39953;
        var_erfcpos_dn6 = assign31440_e39953_d_n6;
        var_erfcpos_dn7 = assign31440_e39953_d_n7;
        var_erfcpos_dn8 = assign31440_e39953_d_n8;
        var_erfcpos_dn9 = assign31440_e39953_d_n9;

        let assign31450_e39956: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard634 = assign31450_e39956;

        let (assign31460_e39970, assign31460_e39970_d_n6, assign31460_e39970_d_n7, assign31460_e39970_d_n8, assign31460_e39970_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) && (var_guard634 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign31460_e39970;
        var_erfctimesexpmtat_dn6 = assign31460_e39970_d_n6;
        var_erfctimesexpmtat_dn7 = assign31460_e39970_d_n7;
        var_erfctimesexpmtat_dn8 = assign31460_e39970_d_n8;
        var_erfctimesexpmtat_dn9 = assign31460_e39970_d_n9;

        let assign31470_e39973: f64 = (-230.25850929940458);
        let assign31470_e39974: f64 = if var_mtat > assign31470_e39973 { 1.0 } else { 0.0 };
        var_guard635 = assign31470_e39974;

        let (assign31480_e39992, assign31480_e39992_d_n6, assign31480_e39992_d_n7, assign31480_e39992_d_n8, assign31480_e39992_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) && (var_guard634 == 0.0)) && (var_guard635 != 0.0)) {
        let assign31480_e39990: f64 = (var_mtat).exp();
        (assign31480_e39990, (assign31480_e39990 * var_mtat_dn6), (assign31480_e39990 * var_mtat_dn7), (assign31480_e39990 * var_mtat_dn8), (assign31480_e39990 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign31480_e39992;
        var_tmp_dn6 = assign31480_e39992_d_n6;
        var_tmp_dn7 = assign31480_e39992_d_n7;
        var_tmp_dn8 = assign31480_e39992_d_n8;
        var_tmp_dn9 = assign31480_e39992_d_n9;

        let (assign31490_e40035, assign31490_e40035_d_n6, assign31490_e40035_d_n7, assign31490_e40035_d_n8, assign31490_e40035_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) && (var_guard634 == 0.0)) && (var_guard635 == 0.0)) {
        let assign31490_e40011: f64 = (-230.25850929940458);
        let assign31490_e40013: f64 = (assign31490_e40011 - var_mtat);
        let assign31490_e40017: f64 = (-230.25850929940458);
        let assign31490_e40019: f64 = (assign31490_e40017 - var_mtat);
        let assign31490_e40022: f64 = (-230.25850929940458);
        let assign31490_e40024: f64 = (assign31490_e40022 - var_mtat);
        let assign31490_e40026: f64 = (assign31490_e40024 * 0.3333333333333333);
        let assign31490_e40027: f64 = (1.0 + assign31490_e40026);
        let assign31490_e40028: f64 = (assign31490_e40019 * assign31490_e40027);
        let assign31490_e40029: f64 = (0.5 * assign31490_e40028);
        let assign31490_e40030: f64 = (1.0 + assign31490_e40029);
        let assign31490_e40031: f64 = (assign31490_e40013 * assign31490_e40030);
        let assign31490_e40032: f64 = (1.0 + assign31490_e40031);
        let assign31490_e40033: f64 = (1e-100 / assign31490_e40032);
        (assign31490_e40033, (-((1e-100 * (((-var_mtat_dn6) * assign31490_e40030) + (assign31490_e40013 * (0.5 * (((-var_mtat_dn6) * assign31490_e40027) + (assign31490_e40019 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign31490_e40032 * assign31490_e40032))), (-((1e-100 * (((-var_mtat_dn7) * assign31490_e40030) + (assign31490_e40013 * (0.5 * (((-var_mtat_dn7) * assign31490_e40027) + (assign31490_e40019 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign31490_e40032 * assign31490_e40032))), (-((1e-100 * (((-var_mtat_dn8) * assign31490_e40030) + (assign31490_e40013 * (0.5 * (((-var_mtat_dn8) * assign31490_e40027) + (assign31490_e40019 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign31490_e40032 * assign31490_e40032))), (-((1e-100 * (((-var_mtat_dn9) * assign31490_e40030) + (assign31490_e40013 * (0.5 * (((-var_mtat_dn9) * assign31490_e40027) + (assign31490_e40019 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign31490_e40032 * assign31490_e40032))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign31490_e40035;
        var_tmp_dn6 = assign31490_e40035_d_n6;
        var_tmp_dn7 = assign31490_e40035_d_n7;
        var_tmp_dn8 = assign31490_e40035_d_n8;
        var_tmp_dn9 = assign31490_e40035_d_n9;

        let (assign31500_e40054, assign31500_e40054_d_n6, assign31500_e40054_d_n7, assign31500_e40054_d_n8, assign31500_e40054_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) && (var_guard634 == 0.0)) {
        let assign31500_e40050: f64 = (2.0 * var_tmp);
        let assign31500_e40052: f64 = (assign31500_e40050 - var_erfcpos);
        (assign31500_e40052, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign31500_e40054;
        var_erfctimesexpmtat_dn6 = assign31500_e40054_d_n6;
        var_erfctimesexpmtat_dn7 = assign31500_e40054_d_n7;
        var_erfctimesexpmtat_dn8 = assign31500_e40054_d_n8;
        var_erfctimesexpmtat_dn9 = assign31500_e40054_d_n9;

        let (assign31510_e40074, assign31510_e40074_d_n6, assign31510_e40074_d_n7, assign31510_e40074_d_n8, assign31510_e40074_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) {
        let assign31510_e40066: f64 = (1.772453850905516 * 0.5);
        let assign31510_e40069: f64 = (var_atatgat_d * var_erfctimesexpmtat);
        let assign31510_e40071: f64 = (assign31510_e40069 / var_ktat);
        let assign31510_e40072: f64 = (assign31510_e40066 * assign31510_e40071);
        (assign31510_e40072, (assign31510_e40066 * ((((var_atatgat_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign31510_e40069 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign31510_e40066 * ((((var_atatgat_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign31510_e40069 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign31510_e40066 * ((((var_atatgat_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign31510_e40069 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign31510_e40066 * ((((var_atatgat_d * var_erfctimesexpmtat_dn9) * var_ktat) - (assign31510_e40069 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign31510_e40074;
        var_gammamax_dn6 = assign31510_e40074_d_n6;
        var_gammamax_dn7 = assign31510_e40074_d_n7;
        var_gammamax_dn8 = assign31510_e40074_d_n8;
        var_gammamax_dn9 = assign31510_e40074_d_n9;

        let (assign31520_e40092, assign31520_e40092_d_n6, assign31520_e40092_d_n7, assign31520_e40092_d_n8, assign31520_e40092_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) {
        let assign31520_e40087: f64 = (var_asrh * var_gammamax);
        let assign31520_e40089: f64 = (assign31520_e40087 * var_wtat);
        let assign31520_e40090: f64 = (var_ctatgatd_i * assign31520_e40089);
        (assign31520_e40090, (var_ctatgatd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign31520_e40087 * var_wtat_dn6))), (var_ctatgatd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign31520_e40087 * var_wtat_dn7))), (var_ctatgatd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign31520_e40087 * var_wtat_dn8))), (var_ctatgatd_i * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign31520_e40087 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign31520_e40092;
        var_itat_dn6 = assign31520_e40092_d_n6;
        var_itat_dn7 = assign31520_e40092_d_n7;
        var_itat_dn8 = assign31520_e40092_d_n8;
        var_itat_dn9 = assign31520_e40092_d_n9;

        let assign31530_e40095: f64 = if var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard636 = assign31530_e40095;

        let (assign31540_e40106, assign31540_e40106_d_n6, assign31540_e40106_d_n7, assign31540_e40106_d_n8, assign31540_e40106_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard636 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign31540_e40106;
        var_ibbt_dn6 = assign31540_e40106_d_n6;
        var_ibbt_dn7 = assign31540_e40106_d_n7;
        var_ibbt_dn8 = assign31540_e40106_d_n8;
        var_ibbt_dn9 = assign31540_e40106_d_n9;

        let assign31550_e40109: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard637 = assign31550_e40109;

        let (assign31560_e40128, assign31560_e40128_d_n6, assign31560_e40128_d_n7, assign31560_e40128_d_n8, assign31560_e40128_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard636 == 0.0)) && (var_guard637 != 0.0)) {
        let assign31560_e40123: f64 = (var_vbirgatd_i - var_vbbt);
        let assign31560_e40125: f64 = (assign31560_e40123 * var_vbirgatinv_d);
        let assign31560_e40126: f64 = (assign31560_e40125).sqrt();
        (assign31560_e40126, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign31560_e40128;
        var_tmp_dn6 = assign31560_e40128_d_n6;
        var_tmp_dn7 = assign31560_e40128_d_n7;
        var_tmp_dn8 = assign31560_e40128_d_n8;
        var_tmp_dn9 = assign31560_e40128_d_n9;

        let (assign31570_e40149, assign31570_e40149_d_n6, assign31570_e40149_d_n7, assign31570_e40149_d_n8, assign31570_e40149_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard636 == 0.0)) && (var_guard637 == 0.0)) {
        let assign31570_e40143: f64 = (var_vbirgatd_i - var_vbbt);
        let assign31570_e40145: f64 = (assign31570_e40143 * var_vbirgatinv_d);
        let assign31570_e40147: f64 = (assign31570_e40145).powf(var_pgatd_i);
        (assign31570_e40147, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign31570_e40149;
        var_tmp_dn6 = assign31570_e40149_d_n6;
        var_tmp_dn7 = assign31570_e40149_d_n7;
        var_tmp_dn8 = assign31570_e40149_d_n8;
        var_tmp_dn9 = assign31570_e40149_d_n9;

        let (assign31580_e40169, assign31580_e40169_d_n6, assign31580_e40169_d_n7, assign31580_e40169_d_n8, assign31580_e40169_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard636 == 0.0)) {
        let assign31580_e40162: f64 = (var_vbirgatd_i - var_vbbt);
        let assign31580_e40164: f64 = (assign31580_e40162 * var_wdepnulrinvgat_d);
        let assign31580_e40166: f64 = (assign31580_e40164 / var_tmp);
        let assign31580_e40167: f64 = (var_one_over_one_minus_pgat_d * assign31580_e40166);
        (assign31580_e40167, (var_one_over_one_minus_pgat_d * (-((assign31580_e40164 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign31580_e40164 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign31580_e40164 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign31580_e40164 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign31580_e40169;
        var_fmaxr_dn6 = assign31580_e40169_d_n6;
        var_fmaxr_dn7 = assign31580_e40169_d_n7;
        var_fmaxr_dn8 = assign31580_e40169_d_n8;
        var_fmaxr_dn9 = assign31580_e40169_d_n9;

        let assign31590_e40171: f64 = (-var_fbbtgat_d);
        let assign31590_e40173: f64 = (assign31590_e40171 / var_fmaxr);
        let assign31590_e40174: f64 = (assign31590_e40173).abs();
        let assign31590_e40176: f64 = if assign31590_e40174 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard638 = assign31590_e40176;

        let (assign31600_e40194, assign31600_e40194_d_n6, assign31600_e40194_d_n7, assign31600_e40194_d_n8, assign31600_e40194_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard636 == 0.0)) && (var_guard638 != 0.0)) {
        let assign31600_e40189: f64 = (-var_fbbtgat_d);
        let assign31600_e40191: f64 = (assign31600_e40189 / var_fmaxr);
        let assign31600_e40192: f64 = (assign31600_e40191).exp();
        (assign31600_e40192, (assign31600_e40192 * ((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign31600_e40189 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign31600_e40192 * ((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign31600_e40189 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign31600_e40192 * ((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign31600_e40189 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))), (assign31600_e40192 * ((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign31600_e40189 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign31600_e40194;
        var_tmp_dn6 = assign31600_e40194_d_n6;
        var_tmp_dn7 = assign31600_e40194_d_n7;
        var_tmp_dn8 = assign31600_e40194_d_n8;
        var_tmp_dn9 = assign31600_e40194_d_n9;

        let assign31610_e40196: f64 = (-var_fbbtgat_d);
        let assign31610_e40198: f64 = (assign31610_e40196 / var_fmaxr);
        let assign31610_e40200: f64 = if assign31610_e40198 < 0.0 { 1.0 } else { 0.0 };
        var_guard639 = assign31610_e40200;

        let (assign31620_e40251, assign31620_e40251_d_n6, assign31620_e40251_d_n7, assign31620_e40251_d_n8, assign31620_e40251_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard636 == 0.0)) && (var_guard638 == 0.0)) && (var_guard639 != 0.0)) {
        let assign31620_e40218: f64 = (-230.25850929940458);
        let assign31620_e40220: f64 = (-var_fbbtgat_d);
        let assign31620_e40222: f64 = (assign31620_e40220 / var_fmaxr);
        let assign31620_e40223: f64 = (assign31620_e40218 - assign31620_e40222);
        let assign31620_e40227: f64 = (-230.25850929940458);
        let assign31620_e40229: f64 = (-var_fbbtgat_d);
        let assign31620_e40231: f64 = (assign31620_e40229 / var_fmaxr);
        let assign31620_e40232: f64 = (assign31620_e40227 - assign31620_e40231);
        let assign31620_e40235: f64 = (-230.25850929940458);
        let assign31620_e40237: f64 = (-var_fbbtgat_d);
        let assign31620_e40239: f64 = (assign31620_e40237 / var_fmaxr);
        let assign31620_e40240: f64 = (assign31620_e40235 - assign31620_e40239);
        let assign31620_e40242: f64 = (assign31620_e40240 * 0.3333333333333333);
        let assign31620_e40243: f64 = (1.0 + assign31620_e40242);
        let assign31620_e40244: f64 = (assign31620_e40232 * assign31620_e40243);
        let assign31620_e40245: f64 = (0.5 * assign31620_e40244);
        let assign31620_e40246: f64 = (1.0 + assign31620_e40245);
        let assign31620_e40247: f64 = (assign31620_e40223 * assign31620_e40246);
        let assign31620_e40248: f64 = (1.0 + assign31620_e40247);
        let assign31620_e40249: f64 = (1e-100 / assign31620_e40248);
        (assign31620_e40249, (-((1e-100 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign31620_e40220 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign31620_e40246) + (assign31620_e40223 * (0.5 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign31620_e40229 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign31620_e40243) + (assign31620_e40232 * ((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign31620_e40237 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign31620_e40248 * assign31620_e40248))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign31620_e40220 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign31620_e40246) + (assign31620_e40223 * (0.5 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign31620_e40229 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign31620_e40243) + (assign31620_e40232 * ((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign31620_e40237 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign31620_e40248 * assign31620_e40248))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign31620_e40220 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign31620_e40246) + (assign31620_e40223 * (0.5 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign31620_e40229 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign31620_e40243) + (assign31620_e40232 * ((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign31620_e40237 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign31620_e40248 * assign31620_e40248))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign31620_e40220 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign31620_e40246) + (assign31620_e40223 * (0.5 * (((-((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign31620_e40229 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign31620_e40243) + (assign31620_e40232 * ((-((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign31620_e40237 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign31620_e40248 * assign31620_e40248))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign31620_e40251;
        var_tmp_dn6 = assign31620_e40251_d_n6;
        var_tmp_dn7 = assign31620_e40251_d_n7;
        var_tmp_dn8 = assign31620_e40251_d_n8;
        var_tmp_dn9 = assign31620_e40251_d_n9;

        let (assign31630_e40300, assign31630_e40300_d_n6, assign31630_e40300_d_n7, assign31630_e40300_d_n8, assign31630_e40300_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard636 == 0.0)) && (var_guard638 == 0.0)) && (var_guard639 == 0.0)) {
        let assign31630_e40270: f64 = (-var_fbbtgat_d);
        let assign31630_e40272: f64 = (assign31630_e40270 / var_fmaxr);
        let assign31630_e40274: f64 = (assign31630_e40272 - 230.25850929940458);
        let assign31630_e40278: f64 = (-var_fbbtgat_d);
        let assign31630_e40280: f64 = (assign31630_e40278 / var_fmaxr);
        let assign31630_e40282: f64 = (assign31630_e40280 - 230.25850929940458);
        let assign31630_e40285: f64 = (-var_fbbtgat_d);
        let assign31630_e40287: f64 = (assign31630_e40285 / var_fmaxr);
        let assign31630_e40289: f64 = (assign31630_e40287 - 230.25850929940458);
        let assign31630_e40291: f64 = (assign31630_e40289 * 0.3333333333333333);
        let assign31630_e40292: f64 = (1.0 + assign31630_e40291);
        let assign31630_e40293: f64 = (assign31630_e40282 * assign31630_e40292);
        let assign31630_e40294: f64 = (0.5 * assign31630_e40293);
        let assign31630_e40295: f64 = (1.0 + assign31630_e40294);
        let assign31630_e40296: f64 = (assign31630_e40274 * assign31630_e40295);
        let assign31630_e40297: f64 = (1.0 + assign31630_e40296);
        let assign31630_e40298: f64 = (1e100 * assign31630_e40297);
        (assign31630_e40298, (1e100 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign31630_e40270 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign31630_e40295) + (assign31630_e40274 * (0.5 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign31630_e40278 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign31630_e40292) + (assign31630_e40282 * (((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign31630_e40285 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign31630_e40270 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign31630_e40295) + (assign31630_e40274 * (0.5 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign31630_e40278 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign31630_e40292) + (assign31630_e40282 * (((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign31630_e40285 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign31630_e40270 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign31630_e40295) + (assign31630_e40274 * (0.5 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign31630_e40278 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign31630_e40292) + (assign31630_e40282 * (((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign31630_e40285 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign31630_e40270 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign31630_e40295) + (assign31630_e40274 * (0.5 * ((((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign31630_e40278 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign31630_e40292) + (assign31630_e40282 * (((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign31630_e40285 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign31630_e40300;
        var_tmp_dn6 = assign31630_e40300_d_n6;
        var_tmp_dn7 = assign31630_e40300_d_n7;
        var_tmp_dn8 = assign31630_e40300_d_n8;
        var_tmp_dn9 = assign31630_e40300_d_n9;

        let (assign31640_e40320, assign31640_e40320_d_n6, assign31640_e40320_d_n7, assign31640_e40320_d_n8, assign31640_e40320_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard636 == 0.0)) {
        let assign31640_e40313: f64 = (var_v2 * var_fmaxr);
        let assign31640_e40315: f64 = (assign31640_e40313 * var_fmaxr);
        let assign31640_e40317: f64 = (assign31640_e40315 * var_tmp);
        let assign31640_e40318: f64 = (var_cbbtgatd_i * assign31640_e40317);
        (assign31640_e40318, (var_cbbtgatd_i * (((((var_v2 * var_fmaxr_dn6) * var_fmaxr) + (assign31640_e40313 * var_fmaxr_dn6)) * var_tmp) + (assign31640_e40315 * var_tmp_dn6))), (var_cbbtgatd_i * (((((var_v2 * var_fmaxr_dn7) * var_fmaxr) + (assign31640_e40313 * var_fmaxr_dn7)) * var_tmp) + (assign31640_e40315 * var_tmp_dn7))), (var_cbbtgatd_i * (((((var_v2 * var_fmaxr_dn8) * var_fmaxr) + (assign31640_e40313 * var_fmaxr_dn8)) * var_tmp) + (assign31640_e40315 * var_tmp_dn8))), (var_cbbtgatd_i * (((((var_v2 * var_fmaxr_dn9) * var_fmaxr) + (assign31640_e40313 * var_fmaxr_dn9)) * var_tmp) + (assign31640_e40315 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign31640_e40320;
        var_ibbt_dn6 = assign31640_e40320_d_n6;
        var_ibbt_dn7 = assign31640_e40320_d_n7;
        var_ibbt_dn8 = assign31640_e40320_d_n8;
        var_ibbt_dn9 = assign31640_e40320_d_n9;

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
        *var_guard632_slot = var_guard632;
        *var_guard633_slot = var_guard633;
        *var_guard634_slot = var_guard634;
        *var_guard635_slot = var_guard635;
        *var_guard636_slot = var_guard636;
        *var_guard637_slot = var_guard637;
        *var_guard638_slot = var_guard638;
        *var_guard639_slot = var_guard639;
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

    pub(super) fn stamp_transient_block_65(
        p: &Parameters,
        var_abdrain_i: f64,
        var_alphaav: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_exp_vmax_over_phitd_d: f64,
        var_fstopgat_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard626: f64,
        var_ibbt: f64,
        var_ibbt_dn6: f64,
        var_ibbt_dn7: f64,
        var_ibbt_dn8: f64,
        var_ibbt_dn9: f64,
        var_idsatbot_d: f64,
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
        var_lgdrain_i: f64,
        var_lsdrain_i: f64,
        var_pbotd_i: f64,
        var_pbrgatd_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_slopegat_d: f64,
        var_slopegat_d_dn6: f64,
        var_slopegat_d_dn7: f64,
        var_slopegat_d_dn8: f64,
        var_slopegat_d_dn9: f64,
        var_v3: f64,
        var_vbbtlim_d: f64,
        var_vbibot_d: f64,
        var_vbimin_d: f64,
        var_vbrgatd_i: f64,
        var_vbrinvgat_d: f64,
        var_vbrinvgat_d_dn6: f64,
        var_vbrinvgat_d_dn7: f64,
        var_vbrinvgat_d_dn8: f64,
        var_vbrinvgat_d_dn9: f64,
        var_vmax_d: f64,
        var_dwsrh_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_guard640_slot: &mut f64,
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
        let mut var_guard640: f64 = *var_guard640_slot;
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

        let assign31650_e40323: f64 = if var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard640 = assign31650_e40323;

        let (assign31660_e40334, assign31660_e40334_d_n6, assign31660_e40334_d_n7, assign31660_e40334_d_n8, assign31660_e40334_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard640 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign31660_e40334;
        var_fbreakdown_dn6 = assign31660_e40334_d_n6;
        var_fbreakdown_dn7 = assign31660_e40334_d_n7;
        var_fbreakdown_dn8 = assign31660_e40334_d_n8;
        var_fbreakdown_dn9 = assign31660_e40334_d_n9;

        let assign31670_e40337: f64 = (-var_alphaav);
        let assign31670_e40339: f64 = (assign31670_e40337 * var_vbrgatd_i);
        let assign31670_e40340: f64 = if var_vav > assign31670_e40339 { 1.0 } else { 0.0 };
        var_guard641 = assign31670_e40340;

        let assign31680_e40343: f64 = if var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard642 = assign31680_e40343;

        let (assign31690_e40373, assign31690_e40373_d_n6, assign31690_e40373_d_n7, assign31690_e40373_d_n8, assign31690_e40373_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard640 == 0.0)) && (var_guard641 != 0.0)) && (var_guard642 != 0.0)) {
        let assign31690_e40359: f64 = (var_vav * var_vbrinvgat_d);
        let assign31690_e40362: f64 = (var_vav * var_vbrinvgat_d);
        let assign31690_e40363: f64 = (assign31690_e40359 * assign31690_e40362);
        let assign31690_e40366: f64 = (var_vav * var_vbrinvgat_d);
        let assign31690_e40367: f64 = (assign31690_e40363 * assign31690_e40366);
        let assign31690_e40370: f64 = (var_vav * var_vbrinvgat_d);
        let assign31690_e40371: f64 = (assign31690_e40367 * assign31690_e40370);
        (assign31690_e40371, (((((((var_vav * var_vbrinvgat_d_dn6) * assign31690_e40362) + (assign31690_e40359 * (var_vav * var_vbrinvgat_d_dn6))) * assign31690_e40366) + (assign31690_e40363 * (var_vav * var_vbrinvgat_d_dn6))) * assign31690_e40370) + (assign31690_e40367 * (var_vav * var_vbrinvgat_d_dn6))), (((((((var_vav * var_vbrinvgat_d_dn7) * assign31690_e40362) + (assign31690_e40359 * (var_vav * var_vbrinvgat_d_dn7))) * assign31690_e40366) + (assign31690_e40363 * (var_vav * var_vbrinvgat_d_dn7))) * assign31690_e40370) + (assign31690_e40367 * (var_vav * var_vbrinvgat_d_dn7))), (((((((var_vav * var_vbrinvgat_d_dn8) * assign31690_e40362) + (assign31690_e40359 * (var_vav * var_vbrinvgat_d_dn8))) * assign31690_e40366) + (assign31690_e40363 * (var_vav * var_vbrinvgat_d_dn8))) * assign31690_e40370) + (assign31690_e40367 * (var_vav * var_vbrinvgat_d_dn8))), (((((((var_vav * var_vbrinvgat_d_dn9) * assign31690_e40362) + (assign31690_e40359 * (var_vav * var_vbrinvgat_d_dn9))) * assign31690_e40366) + (assign31690_e40363 * (var_vav * var_vbrinvgat_d_dn9))) * assign31690_e40370) + (assign31690_e40367 * (var_vav * var_vbrinvgat_d_dn9))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign31690_e40373;
        var_tmp_dn6 = assign31690_e40373_d_n6;
        var_tmp_dn7 = assign31690_e40373_d_n7;
        var_tmp_dn8 = assign31690_e40373_d_n8;
        var_tmp_dn9 = assign31690_e40373_d_n9;

        let (assign31700_e40395, assign31700_e40395_d_n6, assign31700_e40395_d_n7, assign31700_e40395_d_n8, assign31700_e40395_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard640 == 0.0)) && (var_guard641 != 0.0)) && (var_guard642 == 0.0)) {
        let assign31700_e40390: f64 = (var_vav * var_vbrinvgat_d);
        let assign31700_e40391: f64 = (assign31700_e40390).abs();
        let assign31700_e40393: f64 = (assign31700_e40391).powf(var_pbrgatd_i);
        (assign31700_e40393, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign31700_e40391).powf(var_pbrgatd_i - 1.0) * if assign31700_e40390 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) })) } } else { (assign31700_e40393 * (var_pbrgatd_i * (if assign31700_e40390 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) } / assign31700_e40391))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign31700_e40391).powf(var_pbrgatd_i - 1.0) * if assign31700_e40390 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) })) } } else { (assign31700_e40393 * (var_pbrgatd_i * (if assign31700_e40390 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) } / assign31700_e40391))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign31700_e40391).powf(var_pbrgatd_i - 1.0) * if assign31700_e40390 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) })) } } else { (assign31700_e40393 * (var_pbrgatd_i * (if assign31700_e40390 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) } / assign31700_e40391))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign31700_e40391).powf(var_pbrgatd_i - 1.0) * if assign31700_e40390 >= 0.0 { (var_vav * var_vbrinvgat_d_dn9) } else { (-(var_vav * var_vbrinvgat_d_dn9)) })) } } else { (assign31700_e40393 * (var_pbrgatd_i * (if assign31700_e40390 >= 0.0 { (var_vav * var_vbrinvgat_d_dn9) } else { (-(var_vav * var_vbrinvgat_d_dn9)) } / assign31700_e40391))) },)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign31700_e40395;
        var_tmp_dn6 = assign31700_e40395_d_n6;
        var_tmp_dn7 = assign31700_e40395_d_n7;
        var_tmp_dn8 = assign31700_e40395_d_n8;
        var_tmp_dn9 = assign31700_e40395_d_n9;

        let (assign31710_e40413, assign31710_e40413_d_n6, assign31710_e40413_d_n7, assign31710_e40413_d_n8, assign31710_e40413_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard640 == 0.0)) && (var_guard641 != 0.0)) {
        let assign31710_e40410: f64 = (1.0 - var_tmp);
        let assign31710_e40411: f64 = (1.0 / assign31710_e40410);
        (assign31710_e40411, (-((-var_tmp_dn6) / (assign31710_e40410 * assign31710_e40410))), (-((-var_tmp_dn7) / (assign31710_e40410 * assign31710_e40410))), (-((-var_tmp_dn8) / (assign31710_e40410 * assign31710_e40410))), (-((-var_tmp_dn9) / (assign31710_e40410 * assign31710_e40410))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign31710_e40413;
        var_fbreakdown_dn6 = assign31710_e40413_d_n6;
        var_fbreakdown_dn7 = assign31710_e40413_d_n7;
        var_fbreakdown_dn8 = assign31710_e40413_d_n8;
        var_fbreakdown_dn9 = assign31710_e40413_d_n9;

        let (assign31720_e40436, assign31720_e40436_d_n6, assign31720_e40436_d_n7, assign31720_e40436_d_n8, assign31720_e40436_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard640 == 0.0)) && (var_guard641 == 0.0)) {
        let assign31720_e40430: f64 = (var_alphaav * var_vbrgatd_i);
        let assign31720_e40431: f64 = (var_vav + assign31720_e40430);
        let assign31720_e40433: f64 = (assign31720_e40431 * var_slopegat_d);
        let assign31720_e40434: f64 = (var_fstopgat_d + assign31720_e40433);
        (assign31720_e40434, (assign31720_e40431 * var_slopegat_d_dn6), (assign31720_e40431 * var_slopegat_d_dn7), (assign31720_e40431 * var_slopegat_d_dn8), (assign31720_e40431 * var_slopegat_d_dn9),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign31720_e40436;
        var_fbreakdown_dn6 = assign31720_e40436_d_n6;
        var_fbreakdown_dn7 = assign31720_e40436_d_n7;
        var_fbreakdown_dn8 = assign31720_e40436_d_n8;
        var_fbreakdown_dn9 = assign31720_e40436_d_n9;

        let (assign31730_e40455, assign31730_e40455_d_n6, assign31730_e40455_d_n7, assign31730_e40455_d_n8, assign31730_e40455_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) {
        let assign31730_e40446: f64 = (var_id__blk212 + var_isrh);
        let assign31730_e40448: f64 = (assign31730_e40446 + var_itat);
        let assign31730_e40450: f64 = (assign31730_e40448 + var_ibbt);
        let assign31730_e40451: f64 = (p.p29 * assign31730_e40450);
        let assign31730_e40453: f64 = (assign31730_e40451 * var_fbreakdown);
        (assign31730_e40453, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign31730_e40451 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign31730_e40451 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign31730_e40451 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign31730_e40451 * var_fbreakdown_dn9)),)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign31730_e40455;
        var_ijungat_dn6 = assign31730_e40455_d_n6;
        var_ijungat_dn7 = assign31730_e40455_d_n7;
        var_ijungat_dn8 = assign31730_e40455_d_n8;
        var_ijungat_dn9 = assign31730_e40455_d_n9;

        let (assign31740_e40471, assign31740_e40471_d_n6, assign31740_e40471_d_n7, assign31740_e40471_d_n8, assign31740_e40471_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign31740_e40461: f64 = (var_abdrain_i * var_ijunbot);
        let assign31740_e40464: f64 = (var_lsdrain_i * var_ijunsti);
        let assign31740_e40465: f64 = (assign31740_e40461 + assign31740_e40464);
        let assign31740_e40468: f64 = (var_lgdrain_i * var_ijungat);
        let assign31740_e40469: f64 = (assign31740_e40465 + assign31740_e40468);
        (assign31740_e40469, (((var_abdrain_i * var_ijunbot_dn6) + (var_lsdrain_i * var_ijunsti_dn6)) + (var_lgdrain_i * var_ijungat_dn6)), (((var_abdrain_i * var_ijunbot_dn7) + (var_lsdrain_i * var_ijunsti_dn7)) + (var_lgdrain_i * var_ijungat_dn7)), (((var_abdrain_i * var_ijunbot_dn8) + (var_lsdrain_i * var_ijunsti_dn8)) + (var_lgdrain_i * var_ijungat_dn8)), (((var_abdrain_i * var_ijunbot_dn9) + (var_lsdrain_i * var_ijunsti_dn9)) + (var_lgdrain_i * var_ijungat_dn9)),)
    } else {
        (var_i2, var_i2_dn6, var_i2_dn7, var_i2_dn8, var_i2_dn9,)
    }
};
        var_i2 = assign31740_e40471;
        var_i2_dn6 = assign31740_e40471_d_n6;
        var_i2_dn7 = assign31740_e40471_d_n7;
        var_i2_dn8 = assign31740_e40471_d_n8;
        var_i2_dn9 = assign31740_e40471_d_n9;

        let (assign31750_e40477,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign31750_e40477;

        let (assign31760_e40483,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign31760_e40483;

        let assign31770_e40495: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard643 = assign31770_e40495;

        let assign31850_e40581: f64 = if var_v3 < var_vmax_d { 1.0 } else { 0.0 };
        var_guard644 = assign31850_e40581;

        let assign31860_e40583: f64 = (-0.5);
        let assign31860_e40586: f64 = (var_v3 * var_phitdinv);
        let assign31860_e40587: f64 = (assign31860_e40583 * assign31860_e40586);
        let assign31860_e40588: f64 = (assign31860_e40587).abs();
        let assign31860_e40590: f64 = if assign31860_e40588 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard645 = assign31860_e40590;

        let (assign31870_e40608,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard643 != 0.0)) && (var_guard644 != 0.0)) && (var_guard645 != 0.0)) {
        let assign31870_e40601: f64 = (-0.5);
        let assign31870_e40604: f64 = (var_v3 * var_phitdinv);
        let assign31870_e40605: f64 = (assign31870_e40601 * assign31870_e40604);
        let assign31870_e40606: f64 = (assign31870_e40605).exp();
        (assign31870_e40606,)
    } else {
        (var_z,)
    }
};
        var_z = assign31870_e40608;

        let assign31880_e40610: f64 = (-0.5);
        let assign31880_e40613: f64 = (var_v3 * var_phitdinv);
        let assign31880_e40614: f64 = (assign31880_e40610 * assign31880_e40613);
        let assign31880_e40616: f64 = if assign31880_e40614 < 0.0 { 1.0 } else { 0.0 };
        var_guard646 = assign31880_e40616;

        let (assign31890_e40671,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard643 != 0.0)) && (var_guard644 != 0.0)) && (var_guard645 == 0.0)) && (var_guard646 != 0.0)) {
        let assign31890_e40632: f64 = (-230.25850929940458);
        let assign31890_e40634: f64 = (-0.5);
        let assign31890_e40637: f64 = (var_v3 * var_phitdinv);
        let assign31890_e40638: f64 = (assign31890_e40634 * assign31890_e40637);
        let assign31890_e40639: f64 = (assign31890_e40632 - assign31890_e40638);
        let assign31890_e40643: f64 = (-230.25850929940458);
        let assign31890_e40645: f64 = (-0.5);
        let assign31890_e40648: f64 = (var_v3 * var_phitdinv);
        let assign31890_e40649: f64 = (assign31890_e40645 * assign31890_e40648);
        let assign31890_e40650: f64 = (assign31890_e40643 - assign31890_e40649);
        let assign31890_e40653: f64 = (-230.25850929940458);
        let assign31890_e40655: f64 = (-0.5);
        let assign31890_e40658: f64 = (var_v3 * var_phitdinv);
        let assign31890_e40659: f64 = (assign31890_e40655 * assign31890_e40658);
        let assign31890_e40660: f64 = (assign31890_e40653 - assign31890_e40659);
        let assign31890_e40662: f64 = (assign31890_e40660 * 0.3333333333333333);
        let assign31890_e40663: f64 = (1.0 + assign31890_e40662);
        let assign31890_e40664: f64 = (assign31890_e40650 * assign31890_e40663);
        let assign31890_e40665: f64 = (0.5 * assign31890_e40664);
        let assign31890_e40666: f64 = (1.0 + assign31890_e40665);
        let assign31890_e40667: f64 = (assign31890_e40639 * assign31890_e40666);
        let assign31890_e40668: f64 = (1.0 + assign31890_e40667);
        let assign31890_e40669: f64 = (1e-100 / assign31890_e40668);
        (assign31890_e40669,)
    } else {
        (var_z,)
    }
};
        var_z = assign31890_e40671;

        let (assign31900_e40724,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard643 != 0.0)) && (var_guard644 != 0.0)) && (var_guard645 == 0.0)) && (var_guard646 == 0.0)) {
        let assign31900_e40688: f64 = (-0.5);
        let assign31900_e40691: f64 = (var_v3 * var_phitdinv);
        let assign31900_e40692: f64 = (assign31900_e40688 * assign31900_e40691);
        let assign31900_e40694: f64 = (assign31900_e40692 - 230.25850929940458);
        let assign31900_e40698: f64 = (-0.5);
        let assign31900_e40701: f64 = (var_v3 * var_phitdinv);
        let assign31900_e40702: f64 = (assign31900_e40698 * assign31900_e40701);
        let assign31900_e40704: f64 = (assign31900_e40702 - 230.25850929940458);
        let assign31900_e40707: f64 = (-0.5);
        let assign31900_e40710: f64 = (var_v3 * var_phitdinv);
        let assign31900_e40711: f64 = (assign31900_e40707 * assign31900_e40710);
        let assign31900_e40713: f64 = (assign31900_e40711 - 230.25850929940458);
        let assign31900_e40715: f64 = (assign31900_e40713 * 0.3333333333333333);
        let assign31900_e40716: f64 = (1.0 + assign31900_e40715);
        let assign31900_e40717: f64 = (assign31900_e40704 * assign31900_e40716);
        let assign31900_e40718: f64 = (0.5 * assign31900_e40717);
        let assign31900_e40719: f64 = (1.0 + assign31900_e40718);
        let assign31900_e40720: f64 = (assign31900_e40694 * assign31900_e40719);
        let assign31900_e40721: f64 = (1.0 + assign31900_e40720);
        let assign31900_e40722: f64 = (1e100 * assign31900_e40721);
        (assign31900_e40722,)
    } else {
        (var_z,)
    }
};
        var_z = assign31900_e40724;

        let (assign31910_e40736,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard643 != 0.0)) && (var_guard644 != 0.0)) {
        let assign31910_e40734: f64 = (1.0 / var_z);
        (assign31910_e40734,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign31910_e40736;

        let (assign31920_e40748,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard643 != 0.0)) && (var_guard644 != 0.0)) {
        let assign31920_e40746: f64 = (var_zinv * var_zinv);
        (assign31920_e40746,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign31920_e40748;

        let (assign31930_e40767,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard643 != 0.0)) && (var_guard644 == 0.0)) {
        let assign31930_e40760: f64 = (var_v3 - var_vmax_d);
        let assign31930_e40762: f64 = (assign31930_e40760 * var_phitdinv);
        let assign31930_e40763: f64 = (1.0 + assign31930_e40762);
        let assign31930_e40765: f64 = (assign31930_e40763 * var_exp_vmax_over_phitd_d);
        (assign31930_e40765,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign31930_e40767;

        let (assign31940_e40779,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard643 != 0.0)) && (var_guard644 == 0.0)) {
        let assign31940_e40777: f64 = (var_idmult).sqrt();
        (assign31940_e40777,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign31940_e40779;

        let (assign31950_e40792,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard643 != 0.0)) && (var_guard644 == 0.0)) {
        let assign31950_e40790: f64 = (1.0 / var_zinv);
        (assign31950_e40790,)
    } else {
        (var_z,)
    }
};
        var_z = assign31950_e40792;

        let (assign31960_e40802,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard643 != 0.0)) {
        let assign31960_e40800: f64 = (var_idmult - 1.0);
        (assign31960_e40800,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign31960_e40802;

        let assign31970_e40805: f64 = if var_v3 > 0.0 { 1.0 } else { 0.0 };
        var_guard647 = assign31970_e40805;

        let (assign31980_e40831,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard643 != 0.0)) && (var_guard647 != 0.0)) {
        let assign31980_e40817: f64 = (2.0 + var_z);
        let assign31980_e40820: f64 = (var_z + 1.0);
        let assign31980_e40823: f64 = (var_z + 3.0);
        let assign31980_e40824: f64 = (assign31980_e40820 * assign31980_e40823);
        let assign31980_e40825: f64 = (assign31980_e40824).sqrt();
        let assign31980_e40826: f64 = (assign31980_e40817 + assign31980_e40825);
        let assign31980_e40827: f64 = (assign31980_e40826).ln();
        let assign31980_e40828: f64 = (var_phitd * assign31980_e40827);
        let assign31980_e40829: f64 = (2.0 * assign31980_e40828);
        (assign31980_e40829,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign31980_e40831;

        let (assign31990_e40865,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard643 != 0.0)) && (var_guard647 == 0.0)) {
        let assign31990_e40841: f64 = (-var_v3);
        let assign31990_e40846: f64 = (2.0 * var_zinv);
        let assign31990_e40848: f64 = (assign31990_e40846 + 1.0);
        let assign31990_e40851: f64 = (1.0 + var_zinv);
        let assign31990_e40855: f64 = (3.0 * var_zinv);
        let assign31990_e40856: f64 = (1.0 + assign31990_e40855);
        let assign31990_e40857: f64 = (assign31990_e40851 * assign31990_e40856);
        let assign31990_e40858: f64 = (assign31990_e40857).sqrt();
        let assign31990_e40859: f64 = (assign31990_e40848 + assign31990_e40858);
        let assign31990_e40860: f64 = (assign31990_e40859).ln();
        let assign31990_e40861: f64 = (var_phitd * assign31990_e40860);
        let assign31990_e40862: f64 = (2.0 * assign31990_e40861);
        let assign31990_e40863: f64 = (assign31990_e40841 + assign31990_e40862);
        (assign31990_e40863,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign31990_e40865;

        let (assign32000_e40875,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard643 != 0.0)) {
        let assign32000_e40873: f64 = (var_vbimin_d - var_two_psistar);
        (assign32000_e40873,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign32000_e40875;

        let (assign32010_e40902,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard643 != 0.0)) {
        let assign32010_e40884: f64 = (var_v3 + var_vjlim);
        let assign32010_e40887: f64 = (var_v3 - var_vjlim);
        let assign32010_e40890: f64 = (var_v3 - var_vjlim);
        let assign32010_e40891: f64 = (assign32010_e40887 * assign32010_e40890);
        let assign32010_e40894: f64 = (4.0 * var_phitd);
        let assign32010_e40896: f64 = (assign32010_e40894 * var_phitd);
        let assign32010_e40897: f64 = (assign32010_e40891 + assign32010_e40896);
        let assign32010_e40898: f64 = (assign32010_e40897).sqrt();
        let assign32010_e40899: f64 = (assign32010_e40884 - assign32010_e40898);
        let assign32010_e40900: f64 = (0.5 * assign32010_e40899);
        (assign32010_e40900,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign32010_e40902;

        let (assign32020_e40929,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard643 != 0.0)) {
        let assign32020_e40911: f64 = (var_v3 + var_vbbtlim_d);
        let assign32020_e40914: f64 = (var_v3 - var_vbbtlim_d);
        let assign32020_e40917: f64 = (var_v3 - var_vbbtlim_d);
        let assign32020_e40918: f64 = (assign32020_e40914 * assign32020_e40917);
        let assign32020_e40921: f64 = (4.0 * var_phitr);
        let assign32020_e40923: f64 = (assign32020_e40921 * var_phitr);
        let assign32020_e40924: f64 = (assign32020_e40918 + assign32020_e40923);
        let assign32020_e40925: f64 = (assign32020_e40924).sqrt();
        let assign32020_e40926: f64 = (assign32020_e40911 - assign32020_e40925);
        let assign32020_e40927: f64 = (0.5 * assign32020_e40926);
        (assign32020_e40927,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign32020_e40929;

        let (assign32030_e40956,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard643 != 0.0)) {
        let assign32030_e40938: f64 = var_v3;
        let assign32030_e40941: f64 = var_v3;
        let assign32030_e40944: f64 = var_v3;
        let assign32030_e40945: f64 = (assign32030_e40941 * assign32030_e40944);
        let assign32030_e40948: f64 = (4.0 * 1e-6);
        let assign32030_e40950: f64 = (assign32030_e40948 * 1e-6);
        let assign32030_e40951: f64 = (assign32030_e40945 + assign32030_e40950);
        let assign32030_e40952: f64 = (assign32030_e40951).sqrt();
        let assign32030_e40953: f64 = (assign32030_e40938 - assign32030_e40952);
        let assign32030_e40954: f64 = (0.5 * assign32030_e40953);
        (assign32030_e40954,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign32030_e40956;

        let assign32040_e40959: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard648 = assign32040_e40959;

        let (assign32050_e40967, assign32050_e40967_d_n6, assign32050_e40967_d_n7, assign32050_e40967_d_n8, assign32050_e40967_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign32050_e40967;
        var_ijunbot_dn6 = assign32050_e40967_d_n6;
        var_ijunbot_dn7 = assign32050_e40967_d_n7;
        var_ijunbot_dn8 = assign32050_e40967_d_n8;
        var_ijunbot_dn9 = assign32050_e40967_d_n9;

        let (assign32060_e40978,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) {
        let assign32060_e40976: f64 = (var_idsatbot_d * var_idmult);
        (assign32060_e40976,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign32060_e40978;

        let assign32070_e40985: f64 = if ((var_csrhbotd_i == 0.0) && (var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard649 = assign32070_e40985;

        let (assign32080_e40996, assign32080_e40996_d_n6, assign32080_e40996_d_n7, assign32080_e40996_d_n8, assign32080_e40996_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard649 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign32080_e40996;
        var_isrh_dn6 = assign32080_e40996_d_n6;
        var_isrh_dn7 = assign32080_e40996_d_n7;
        var_isrh_dn8 = assign32080_e40996_d_n8;
        var_isrh_dn9 = assign32080_e40996_d_n9;

        let (assign32090_e41010,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard649 == 0.0)) {
        let assign32090_e41008: f64 = (var_vbibot_d - var_vjsrh);
        (assign32090_e41008,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign32090_e41010;

        let (assign32100_e41029,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard649 == 0.0)) {
        let assign32100_e41024: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign32100_e41025: f64 = (1.0 - assign32100_e41024);
        let assign32100_e41026: f64 = (assign32100_e41025).sqrt();
        let assign32100_e41027: f64 = (1.0 - assign32100_e41026);
        (assign32100_e41027,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign32100_e41029;

        let assign32110_e41032: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard650 = assign32110_e41032;

        let (assign32120_e41046,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard649 == 0.0)) && (var_guard650 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign32120_e41046;

        let (assign32130_e41078,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard649 == 0.0)) && (var_guard650 == 0.0)) {
        let assign32130_e41061: f64 = (var_wsrhstep * var_wsrhstep);
        let assign32130_e41063: f64 = (var_wsrhstep).ln();
        let assign32130_e41064: f64 = (assign32130_e41061 * assign32130_e41063);
        let assign32130_e41067: f64 = (1.0 - var_wsrhstep);
        let assign32130_e41068: f64 = (assign32130_e41064 / assign32130_e41067);
        let assign32130_e41070: f64 = (assign32130_e41068 + var_wsrhstep);
        let assign32130_e41074: f64 = (2.0 * var_pbotd_i);
        let assign32130_e41075: f64 = (1.0 - assign32130_e41074);
        let assign32130_e41076: f64 = (assign32130_e41070 * assign32130_e41075);
        (assign32130_e41076,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign32130_e41078;

        *var_dwsrh_slot = var_dwsrh;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_guard640_slot = var_guard640;
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

    pub(super) fn stamp_transient_block_66(
        var_atatbot_d: f64,
        var_berfc: f64,
        var_btatpartbot_d: f64,
        var_cerfc: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_dwsrh: f64,
        var_ftdbot_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard648: f64,
        var_guard649: f64,
        var_one_minus_pbot_d: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_perfc: f64,
        var_vbi_minus_vjsrh: f64,
        var_vbirbotinv_d: f64,
        var_wdepnulrbot_d: f64,
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
        var_guard651_slot: &mut f64,
        var_guard652_slot: &mut f64,
        var_guard653_slot: &mut f64,
        var_guard654_slot: &mut f64,
        var_guard655_slot: &mut f64,
        var_guard656_slot: &mut f64,
        var_guard657_slot: &mut f64,
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
        let mut var_guard651: f64 = *var_guard651_slot;
        let mut var_guard652: f64 = *var_guard652_slot;
        let mut var_guard653: f64 = *var_guard653_slot;
        let mut var_guard654: f64 = *var_guard654_slot;
        let mut var_guard655: f64 = *var_guard655_slot;
        let mut var_guard656: f64 = *var_guard656_slot;
        let mut var_guard657: f64 = *var_guard657_slot;
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

        let (assign32140_e41092,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard649 == 0.0)) {
        let assign32140_e41090: f64 = (var_wsrhstep + var_dwsrh);
        (assign32140_e41090,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign32140_e41092;

        let assign32150_e41095: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard651 = assign32150_e41095;

        let (assign32160_e41112, assign32160_e41112_d_n6, assign32160_e41112_d_n7, assign32160_e41112_d_n8, assign32160_e41112_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard649 == 0.0)) && (var_guard651 != 0.0)) {
        let assign32160_e41109: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign32160_e41110: f64 = (assign32160_e41109).sqrt();
        (assign32160_e41110, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign32160_e41112;
        var_tmp_dn6 = assign32160_e41112_d_n6;
        var_tmp_dn7 = assign32160_e41112_d_n7;
        var_tmp_dn8 = assign32160_e41112_d_n8;
        var_tmp_dn9 = assign32160_e41112_d_n9;

        let (assign32170_e41131, assign32170_e41131_d_n6, assign32170_e41131_d_n7, assign32170_e41131_d_n8, assign32170_e41131_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard649 == 0.0)) && (var_guard651 == 0.0)) {
        let assign32170_e41127: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign32170_e41129: f64 = (assign32170_e41127).powf(var_pbotd_i);
        (assign32170_e41129, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign32170_e41131;
        var_tmp_dn6 = assign32170_e41131_d_n6;
        var_tmp_dn7 = assign32170_e41131_d_n7;
        var_tmp_dn8 = assign32170_e41131_d_n8;
        var_tmp_dn9 = assign32170_e41131_d_n9;

        let (assign32180_e41145, assign32180_e41145_d_n6, assign32180_e41145_d_n7, assign32180_e41145_d_n8, assign32180_e41145_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard649 == 0.0)) {
        let assign32180_e41143: f64 = (var_wdepnulrbot_d * var_tmp);
        (assign32180_e41143, (var_wdepnulrbot_d * var_tmp_dn6), (var_wdepnulrbot_d * var_tmp_dn7), (var_wdepnulrbot_d * var_tmp_dn8), (var_wdepnulrbot_d * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign32180_e41145;
        var_wdep_dn6 = assign32180_e41145_d_n6;
        var_wdep_dn7 = assign32180_e41145_d_n7;
        var_wdep_dn8 = assign32180_e41145_d_n8;
        var_wdep_dn9 = assign32180_e41145_d_n9;

        let (assign32190_e41163, assign32190_e41163_d_n6, assign32190_e41163_d_n7, assign32190_e41163_d_n8, assign32190_e41163_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard649 == 0.0)) {
        let assign32190_e41158: f64 = (var_zinv - 1.0);
        let assign32190_e41160: f64 = (assign32190_e41158 * var_wdep);
        let assign32190_e41161: f64 = (var_ftdbot_d * assign32190_e41160);
        (assign32190_e41161, (var_ftdbot_d * (assign32190_e41158 * var_wdep_dn6)), (var_ftdbot_d * (assign32190_e41158 * var_wdep_dn7)), (var_ftdbot_d * (assign32190_e41158 * var_wdep_dn8)), (var_ftdbot_d * (assign32190_e41158 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign32190_e41163;
        var_asrh_dn6 = assign32190_e41163_d_n6;
        var_asrh_dn7 = assign32190_e41163_d_n7;
        var_asrh_dn8 = assign32190_e41163_d_n8;
        var_asrh_dn9 = assign32190_e41163_d_n9;

        let (assign32200_e41179, assign32200_e41179_d_n6, assign32200_e41179_d_n7, assign32200_e41179_d_n8, assign32200_e41179_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard649 == 0.0)) {
        let assign32200_e41176: f64 = (var_asrh * var_wsrh);
        let assign32200_e41177: f64 = (var_csrhbotd_i * assign32200_e41176);
        (assign32200_e41177, (var_csrhbotd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn8 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign32200_e41179;
        var_isrh_dn6 = assign32200_e41179_d_n6;
        var_isrh_dn7 = assign32200_e41179_d_n7;
        var_isrh_dn8 = assign32200_e41179_d_n8;
        var_isrh_dn9 = assign32200_e41179_d_n9;

        let assign32210_e41182: f64 = if var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard652 = assign32210_e41182;

        let (assign32220_e41193, assign32220_e41193_d_n6, assign32220_e41193_d_n7, assign32220_e41193_d_n8, assign32220_e41193_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign32220_e41193;
        var_itat_dn6 = assign32220_e41193_d_n6;
        var_itat_dn7 = assign32220_e41193_d_n7;
        var_itat_dn8 = assign32220_e41193_d_n8;
        var_itat_dn9 = assign32220_e41193_d_n9;

        let (assign32230_e41211, assign32230_e41211_d_n6, assign32230_e41211_d_n7, assign32230_e41211_d_n8, assign32230_e41211_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) {
        let assign32230_e41206: f64 = (var_wdep * var_one_minus_pbot_d);
        let assign32230_e41208: f64 = (assign32230_e41206 / var_vbi_minus_vjsrh);
        let assign32230_e41209: f64 = (var_btatpartbot_d * assign32230_e41208);
        (assign32230_e41209, (var_btatpartbot_d * ((var_wdep_dn6 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn7 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn8 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn9 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign32230_e41211;
        var_btat_dn6 = assign32230_e41211_d_n6;
        var_btat_dn7 = assign32230_e41211_d_n7;
        var_btat_dn8 = assign32230_e41211_d_n8;
        var_btat_dn9 = assign32230_e41211_d_n9;

        let (assign32240_e41227, assign32240_e41227_d_n6, assign32240_e41227_d_n7, assign32240_e41227_d_n8, assign32240_e41227_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) {
        let assign32240_e41223: f64 = (0.666666666666667 * var_atatbot_d);
        let assign32240_e41225: f64 = (assign32240_e41223 / var_btat);
        (assign32240_e41225, (-((assign32240_e41223 * var_btat_dn6) / (var_btat * var_btat))), (-((assign32240_e41223 * var_btat_dn7) / (var_btat * var_btat))), (-((assign32240_e41223 * var_btat_dn8) / (var_btat * var_btat))), (-((assign32240_e41223 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign32240_e41227;
        var_twoatatoverthreebtat_dn6 = assign32240_e41227_d_n6;
        var_twoatatoverthreebtat_dn7 = assign32240_e41227_d_n7;
        var_twoatatoverthreebtat_dn8 = assign32240_e41227_d_n8;
        var_twoatatoverthreebtat_dn9 = assign32240_e41227_d_n9;

        let (assign32250_e41241, assign32250_e41241_d_n6, assign32250_e41241_d_n7, assign32250_e41241_d_n8, assign32250_e41241_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) {
        let assign32250_e41239: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign32250_e41239, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign32250_e41241;
        var_umaxbeforelimiting_dn6 = assign32250_e41241_d_n6;
        var_umaxbeforelimiting_dn7 = assign32250_e41241_d_n7;
        var_umaxbeforelimiting_dn8 = assign32250_e41241_d_n8;
        var_umaxbeforelimiting_dn9 = assign32250_e41241_d_n9;

        let (assign32260_e41262, assign32260_e41262_d_n6, assign32260_e41262_d_n7, assign32260_e41262_d_n8, assign32260_e41262_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) {
        let assign32260_e41253: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign32260_e41256: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign32260_e41258: f64 = (assign32260_e41256 + 1.0);
        let assign32260_e41259: f64 = (assign32260_e41253 / assign32260_e41258);
        let assign32260_e41260: f64 = (assign32260_e41259).sqrt();
        (assign32260_e41260, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign32260_e41258) - (assign32260_e41253 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign32260_e41258 * assign32260_e41258)) / (2.0 * assign32260_e41260)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign32260_e41258) - (assign32260_e41253 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign32260_e41258 * assign32260_e41258)) / (2.0 * assign32260_e41260)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign32260_e41258) - (assign32260_e41253 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign32260_e41258 * assign32260_e41258)) / (2.0 * assign32260_e41260)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign32260_e41258) - (assign32260_e41253 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign32260_e41258 * assign32260_e41258)) / (2.0 * assign32260_e41260)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign32260_e41262;
        var_umax_dn6 = assign32260_e41262_d_n6;
        var_umax_dn7 = assign32260_e41262_d_n7;
        var_umax_dn8 = assign32260_e41262_d_n8;
        var_umax_dn9 = assign32260_e41262_d_n9;

        let (assign32270_e41275, assign32270_e41275_d_n6, assign32270_e41275_d_n7, assign32270_e41275_d_n8, assign32270_e41275_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) {
        let assign32270_e41273: f64 = (var_umax).sqrt();
        (assign32270_e41273, (var_umax_dn6 / (2.0 * assign32270_e41273)), (var_umax_dn7 / (2.0 * assign32270_e41273)), (var_umax_dn8 / (2.0 * assign32270_e41273)), (var_umax_dn9 / (2.0 * assign32270_e41273)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign32270_e41275;
        var_sqrtumax_dn6 = assign32270_e41275_d_n6;
        var_sqrtumax_dn7 = assign32270_e41275_d_n7;
        var_sqrtumax_dn8 = assign32270_e41275_d_n8;
        var_sqrtumax_dn9 = assign32270_e41275_d_n9;

        let (assign32280_e41289, assign32280_e41289_d_n6, assign32280_e41289_d_n7, assign32280_e41289_d_n8, assign32280_e41289_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) {
        let assign32280_e41287: f64 = (var_umax * var_sqrtumax);
        (assign32280_e41287, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign32280_e41289;
        var_umaxpoweronepointfive_dn6 = assign32280_e41289_d_n6;
        var_umaxpoweronepointfive_dn7 = assign32280_e41289_d_n7;
        var_umaxpoweronepointfive_dn8 = assign32280_e41289_d_n8;
        var_umaxpoweronepointfive_dn9 = assign32280_e41289_d_n9;

        let assign32290_e41291: f64 = (-var_pbotd_i);
        let assign32290_e41293: f64 = (assign32290_e41291 * var_one_over_one_minus_pbot_d);
        let assign32290_e41295: f64 = (-1.0);
        let assign32290_e41296: f64 = if assign32290_e41293 == assign32290_e41295 { 1.0 } else { 0.0 };
        var_guard653 = assign32290_e41296;

        let (assign32300_e41316, assign32300_e41316_d_n6, assign32300_e41316_d_n7, assign32300_e41316_d_n8, assign32300_e41316_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) && (var_guard653 != 0.0)) {
        let assign32300_e41312: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign32300_e41313: f64 = (1.0 + assign32300_e41312);
        let assign32300_e41314: f64 = (1.0 / assign32300_e41313);
        (assign32300_e41314, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign32300_e41313 * assign32300_e41313))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign32300_e41313 * assign32300_e41313))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign32300_e41313 * assign32300_e41313))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign32300_e41313 * assign32300_e41313))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign32300_e41316;
        var_wgamma_dn6 = assign32300_e41316_d_n6;
        var_wgamma_dn7 = assign32300_e41316_d_n7;
        var_wgamma_dn8 = assign32300_e41316_d_n8;
        var_wgamma_dn9 = assign32300_e41316_d_n9;

        let (assign32310_e41340, assign32310_e41340_d_n6, assign32310_e41340_d_n7, assign32310_e41340_d_n8, assign32310_e41340_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) && (var_guard653 == 0.0)) {
        let assign32310_e41332: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign32310_e41333: f64 = (1.0 + assign32310_e41332);
        let assign32310_e41335: f64 = (-var_pbotd_i);
        let assign32310_e41337: f64 = (assign32310_e41335 * var_one_over_one_minus_pbot_d);
        let assign32310_e41338: f64 = (assign32310_e41333).powf(assign32310_e41337);
        (assign32310_e41338, if 0.0 == 0.0 && ((assign32310_e41337) as f64).is_finite() && ((assign32310_e41337) as f64).fract() == 0.0 { if assign32310_e41337 == 0.0 { 0.0 } else { (assign32310_e41337 * ((assign32310_e41333).powf(assign32310_e41337 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign32310_e41338 * (assign32310_e41337 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign32310_e41333))) }, if 0.0 == 0.0 && ((assign32310_e41337) as f64).is_finite() && ((assign32310_e41337) as f64).fract() == 0.0 { if assign32310_e41337 == 0.0 { 0.0 } else { (assign32310_e41337 * ((assign32310_e41333).powf(assign32310_e41337 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign32310_e41338 * (assign32310_e41337 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign32310_e41333))) }, if 0.0 == 0.0 && ((assign32310_e41337) as f64).is_finite() && ((assign32310_e41337) as f64).fract() == 0.0 { if assign32310_e41337 == 0.0 { 0.0 } else { (assign32310_e41337 * ((assign32310_e41333).powf(assign32310_e41337 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign32310_e41338 * (assign32310_e41337 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign32310_e41333))) }, if 0.0 == 0.0 && ((assign32310_e41337) as f64).is_finite() && ((assign32310_e41337) as f64).fract() == 0.0 { if assign32310_e41337 == 0.0 { 0.0 } else { (assign32310_e41337 * ((assign32310_e41333).powf(assign32310_e41337 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign32310_e41338 * (assign32310_e41337 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign32310_e41333))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign32310_e41340;
        var_wgamma_dn6 = assign32310_e41340_d_n6;
        var_wgamma_dn7 = assign32310_e41340_d_n7;
        var_wgamma_dn8 = assign32310_e41340_d_n8;
        var_wgamma_dn9 = assign32310_e41340_d_n9;

        let (assign32320_e41358, assign32320_e41358_d_n6, assign32320_e41358_d_n7, assign32320_e41358_d_n8, assign32320_e41358_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) {
        let assign32320_e41352: f64 = (var_wsrh * var_wgamma);
        let assign32320_e41355: f64 = (var_wsrh + var_wgamma);
        let assign32320_e41356: f64 = (assign32320_e41352 / assign32320_e41355);
        (assign32320_e41356, ((((var_wsrh * var_wgamma_dn6) * assign32320_e41355) - (assign32320_e41352 * var_wgamma_dn6)) / (assign32320_e41355 * assign32320_e41355)), ((((var_wsrh * var_wgamma_dn7) * assign32320_e41355) - (assign32320_e41352 * var_wgamma_dn7)) / (assign32320_e41355 * assign32320_e41355)), ((((var_wsrh * var_wgamma_dn8) * assign32320_e41355) - (assign32320_e41352 * var_wgamma_dn8)) / (assign32320_e41355 * assign32320_e41355)), ((((var_wsrh * var_wgamma_dn9) * assign32320_e41355) - (assign32320_e41352 * var_wgamma_dn9)) / (assign32320_e41355 * assign32320_e41355)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign32320_e41358;
        var_wtat_dn6 = assign32320_e41358_d_n6;
        var_wtat_dn7 = assign32320_e41358_d_n7;
        var_wtat_dn8 = assign32320_e41358_d_n8;
        var_wtat_dn9 = assign32320_e41358_d_n9;

        let (assign32330_e41375, assign32330_e41375_d_n6, assign32330_e41375_d_n7, assign32330_e41375_d_n8, assign32330_e41375_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) {
        let assign32330_e41371: f64 = (var_btat / var_sqrtumax);
        let assign32330_e41372: f64 = (0.375 * assign32330_e41371);
        let assign32330_e41373: f64 = (assign32330_e41372).sqrt();
        (assign32330_e41373, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign32330_e41373)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign32330_e41373)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign32330_e41373)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign32330_e41373)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign32330_e41375;
        var_ktat_dn6 = assign32330_e41375_d_n6;
        var_ktat_dn7 = assign32330_e41375_d_n7;
        var_ktat_dn8 = assign32330_e41375_d_n8;
        var_ktat_dn9 = assign32330_e41375_d_n9;

        let (assign32340_e41393, assign32340_e41393_d_n6, assign32340_e41393_d_n7, assign32340_e41393_d_n8, assign32340_e41393_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) {
        let assign32340_e41388: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign32340_e41389: f64 = (2.0 * assign32340_e41388);
        let assign32340_e41391: f64 = (assign32340_e41389 - var_umax);
        (assign32340_e41391, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign32340_e41393;
        var_ltat_dn6 = assign32340_e41393_d_n6;
        var_ltat_dn7 = assign32340_e41393_d_n7;
        var_ltat_dn8 = assign32340_e41393_d_n8;
        var_ltat_dn9 = assign32340_e41393_d_n9;

        let (assign32350_e41419, assign32350_e41419_d_n6, assign32350_e41419_d_n7, assign32350_e41419_d_n8, assign32350_e41419_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) {
        let assign32350_e41405: f64 = (var_atatbot_d * var_twoatatoverthreebtat);
        let assign32350_e41407: f64 = (assign32350_e41405 * var_sqrtumax);
        let assign32350_e41410: f64 = (var_atatbot_d * var_umax);
        let assign32350_e41411: f64 = (assign32350_e41407 - assign32350_e41410);
        let assign32350_e41415: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign32350_e41416: f64 = (0.5 * assign32350_e41415);
        let assign32350_e41417: f64 = (assign32350_e41411 + assign32350_e41416);
        (assign32350_e41417, (((((var_atatbot_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign32350_e41405 * var_sqrtumax_dn6)) - (var_atatbot_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign32350_e41405 * var_sqrtumax_dn7)) - (var_atatbot_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign32350_e41405 * var_sqrtumax_dn8)) - (var_atatbot_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign32350_e41405 * var_sqrtumax_dn9)) - (var_atatbot_d * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign32350_e41419;
        var_mtat_dn6 = assign32350_e41419_d_n6;
        var_mtat_dn7 = assign32350_e41419_d_n7;
        var_mtat_dn8 = assign32350_e41419_d_n8;
        var_mtat_dn9 = assign32350_e41419_d_n9;

        let (assign32360_e41435, assign32360_e41435_d_n6, assign32360_e41435_d_n7, assign32360_e41435_d_n8, assign32360_e41435_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) {
        let assign32360_e41431: f64 = (var_ltat - 1.0);
        let assign32360_e41433: f64 = (assign32360_e41431 * var_ktat);
        (assign32360_e41433, ((var_ltat_dn6 * var_ktat) + (assign32360_e41431 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign32360_e41431 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign32360_e41431 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign32360_e41431 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign32360_e41435;
        var_xerfc_dn6 = assign32360_e41435_d_n6;
        var_xerfc_dn7 = assign32360_e41435_d_n7;
        var_xerfc_dn8 = assign32360_e41435_d_n8;
        var_xerfc_dn9 = assign32360_e41435_d_n9;

        let (assign32370_e41449, assign32370_e41449_d_n6, assign32370_e41449_d_n7, assign32370_e41449_d_n8, assign32370_e41449_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) {
        let assign32370_e41447: f64 = (var_xerfc * var_xerfc);
        (assign32370_e41447, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign32370_e41449;
        var_ysq_dn6 = assign32370_e41449_d_n6;
        var_ysq_dn7 = assign32370_e41449_d_n7;
        var_ysq_dn8 = assign32370_e41449_d_n8;
        var_ysq_dn9 = assign32370_e41449_d_n9;

        let assign32380_e41452: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard654 = assign32380_e41452;

        let (assign32390_e41472, assign32390_e41472_d_n6, assign32390_e41472_d_n7, assign32390_e41472_d_n8, assign32390_e41472_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) && (var_guard654 != 0.0)) {
        let assign32390_e41468: f64 = (var_perfc * var_xerfc);
        let assign32390_e41469: f64 = (1.0 + assign32390_e41468);
        let assign32390_e41470: f64 = (1.0 / assign32390_e41469);
        (assign32390_e41470, (-((var_perfc * var_xerfc_dn6) / (assign32390_e41469 * assign32390_e41469))), (-((var_perfc * var_xerfc_dn7) / (assign32390_e41469 * assign32390_e41469))), (-((var_perfc * var_xerfc_dn8) / (assign32390_e41469 * assign32390_e41469))), (-((var_perfc * var_xerfc_dn9) / (assign32390_e41469 * assign32390_e41469))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign32390_e41472;
        var_terfc_dn6 = assign32390_e41472_d_n6;
        var_terfc_dn7 = assign32390_e41472_d_n7;
        var_terfc_dn8 = assign32390_e41472_d_n8;
        var_terfc_dn9 = assign32390_e41472_d_n9;

        let (assign32400_e41493, assign32400_e41493_d_n6, assign32400_e41493_d_n7, assign32400_e41493_d_n8, assign32400_e41493_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) && (var_guard654 == 0.0)) {
        let assign32400_e41489: f64 = (var_perfc * var_xerfc);
        let assign32400_e41490: f64 = (1.0 - assign32400_e41489);
        let assign32400_e41491: f64 = (1.0 / assign32400_e41490);
        (assign32400_e41491, (-((-(var_perfc * var_xerfc_dn6)) / (assign32400_e41490 * assign32400_e41490))), (-((-(var_perfc * var_xerfc_dn7)) / (assign32400_e41490 * assign32400_e41490))), (-((-(var_perfc * var_xerfc_dn8)) / (assign32400_e41490 * assign32400_e41490))), (-((-(var_perfc * var_xerfc_dn9)) / (assign32400_e41490 * assign32400_e41490))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign32400_e41493;
        var_terfc_dn6 = assign32400_e41493_d_n6;
        var_terfc_dn7 = assign32400_e41493_d_n7;
        var_terfc_dn8 = assign32400_e41493_d_n8;
        var_terfc_dn9 = assign32400_e41493_d_n9;

        let assign32410_e41495: f64 = (-var_ysq);
        let assign32410_e41497: f64 = (assign32410_e41495 + var_mtat);
        let assign32410_e41499: f64 = (-230.25850929940458);
        let assign32410_e41500: f64 = if assign32410_e41497 > assign32410_e41499 { 1.0 } else { 0.0 };
        var_guard655 = assign32410_e41500;

        let (assign32420_e41518, assign32420_e41518_d_n6, assign32420_e41518_d_n7, assign32420_e41518_d_n8, assign32420_e41518_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) && (var_guard655 != 0.0)) {
        let assign32420_e41513: f64 = (-var_ysq);
        let assign32420_e41515: f64 = (assign32420_e41513 + var_mtat);
        let assign32420_e41516: f64 = (assign32420_e41515).exp();
        (assign32420_e41516, (assign32420_e41516 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign32420_e41516 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign32420_e41516 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign32420_e41516 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign32420_e41518;
        var_tmp_dn6 = assign32420_e41518_d_n6;
        var_tmp_dn7 = assign32420_e41518_d_n7;
        var_tmp_dn8 = assign32420_e41518_d_n8;
        var_tmp_dn9 = assign32420_e41518_d_n9;

        let (assign32430_e41567, assign32430_e41567_d_n6, assign32430_e41567_d_n7, assign32430_e41567_d_n8, assign32430_e41567_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) && (var_guard655 == 0.0)) {
        let assign32430_e41534: f64 = (-230.25850929940458);
        let assign32430_e41536: f64 = (-var_ysq);
        let assign32430_e41538: f64 = (assign32430_e41536 + var_mtat);
        let assign32430_e41539: f64 = (assign32430_e41534 - assign32430_e41538);
        let assign32430_e41543: f64 = (-230.25850929940458);
        let assign32430_e41545: f64 = (-var_ysq);
        let assign32430_e41547: f64 = (assign32430_e41545 + var_mtat);
        let assign32430_e41548: f64 = (assign32430_e41543 - assign32430_e41547);
        let assign32430_e41551: f64 = (-230.25850929940458);
        let assign32430_e41553: f64 = (-var_ysq);
        let assign32430_e41555: f64 = (assign32430_e41553 + var_mtat);
        let assign32430_e41556: f64 = (assign32430_e41551 - assign32430_e41555);
        let assign32430_e41558: f64 = (assign32430_e41556 * 0.3333333333333333);
        let assign32430_e41559: f64 = (1.0 + assign32430_e41558);
        let assign32430_e41560: f64 = (assign32430_e41548 * assign32430_e41559);
        let assign32430_e41561: f64 = (0.5 * assign32430_e41560);
        let assign32430_e41562: f64 = (1.0 + assign32430_e41561);
        let assign32430_e41563: f64 = (assign32430_e41539 * assign32430_e41562);
        let assign32430_e41564: f64 = (1.0 + assign32430_e41563);
        let assign32430_e41565: f64 = (1e-100 / assign32430_e41564);
        (assign32430_e41565, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign32430_e41562) + (assign32430_e41539 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign32430_e41559) + (assign32430_e41548 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign32430_e41564 * assign32430_e41564))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign32430_e41562) + (assign32430_e41539 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign32430_e41559) + (assign32430_e41548 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign32430_e41564 * assign32430_e41564))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign32430_e41562) + (assign32430_e41539 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign32430_e41559) + (assign32430_e41548 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign32430_e41564 * assign32430_e41564))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign32430_e41562) + (assign32430_e41539 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign32430_e41559) + (assign32430_e41548 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign32430_e41564 * assign32430_e41564))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign32430_e41567;
        var_tmp_dn6 = assign32430_e41567_d_n6;
        var_tmp_dn7 = assign32430_e41567_d_n7;
        var_tmp_dn8 = assign32430_e41567_d_n8;
        var_tmp_dn9 = assign32430_e41567_d_n9;

        let (assign32440_e41597, assign32440_e41597_d_n6, assign32440_e41597_d_n7, assign32440_e41597_d_n8, assign32440_e41597_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) {
        let assign32440_e41579: f64 = (0.29214664 * var_terfc);
        let assign32440_e41583: f64 = (var_terfc * var_terfc);
        let assign32440_e41584: f64 = (var_berfc * assign32440_e41583);
        let assign32440_e41585: f64 = (assign32440_e41579 + assign32440_e41584);
        let assign32440_e41589: f64 = (var_terfc * var_terfc);
        let assign32440_e41591: f64 = (assign32440_e41589 * var_terfc);
        let assign32440_e41592: f64 = (var_cerfc * assign32440_e41591);
        let assign32440_e41593: f64 = (assign32440_e41585 + assign32440_e41592);
        let assign32440_e41595: f64 = (assign32440_e41593 * var_tmp);
        (assign32440_e41595, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign32440_e41589 * var_terfc_dn6)))) * var_tmp) + (assign32440_e41593 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign32440_e41589 * var_terfc_dn7)))) * var_tmp) + (assign32440_e41593 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign32440_e41589 * var_terfc_dn8)))) * var_tmp) + (assign32440_e41593 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign32440_e41589 * var_terfc_dn9)))) * var_tmp) + (assign32440_e41593 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign32440_e41597;
        var_erfcpos_dn6 = assign32440_e41597_d_n6;
        var_erfcpos_dn7 = assign32440_e41597_d_n7;
        var_erfcpos_dn8 = assign32440_e41597_d_n8;
        var_erfcpos_dn9 = assign32440_e41597_d_n9;

        let assign32450_e41600: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard656 = assign32450_e41600;

        let (assign32460_e41614, assign32460_e41614_d_n6, assign32460_e41614_d_n7, assign32460_e41614_d_n8, assign32460_e41614_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) && (var_guard656 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign32460_e41614;
        var_erfctimesexpmtat_dn6 = assign32460_e41614_d_n6;
        var_erfctimesexpmtat_dn7 = assign32460_e41614_d_n7;
        var_erfctimesexpmtat_dn8 = assign32460_e41614_d_n8;
        var_erfctimesexpmtat_dn9 = assign32460_e41614_d_n9;

        let assign32470_e41617: f64 = (-230.25850929940458);
        let assign32470_e41618: f64 = if var_mtat > assign32470_e41617 { 1.0 } else { 0.0 };
        var_guard657 = assign32470_e41618;

        let (assign32480_e41636, assign32480_e41636_d_n6, assign32480_e41636_d_n7, assign32480_e41636_d_n8, assign32480_e41636_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) && (var_guard656 == 0.0)) && (var_guard657 != 0.0)) {
        let assign32480_e41634: f64 = (var_mtat).exp();
        (assign32480_e41634, (assign32480_e41634 * var_mtat_dn6), (assign32480_e41634 * var_mtat_dn7), (assign32480_e41634 * var_mtat_dn8), (assign32480_e41634 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign32480_e41636;
        var_tmp_dn6 = assign32480_e41636_d_n6;
        var_tmp_dn7 = assign32480_e41636_d_n7;
        var_tmp_dn8 = assign32480_e41636_d_n8;
        var_tmp_dn9 = assign32480_e41636_d_n9;

        let (assign32490_e41679, assign32490_e41679_d_n6, assign32490_e41679_d_n7, assign32490_e41679_d_n8, assign32490_e41679_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) && (var_guard656 == 0.0)) && (var_guard657 == 0.0)) {
        let assign32490_e41655: f64 = (-230.25850929940458);
        let assign32490_e41657: f64 = (assign32490_e41655 - var_mtat);
        let assign32490_e41661: f64 = (-230.25850929940458);
        let assign32490_e41663: f64 = (assign32490_e41661 - var_mtat);
        let assign32490_e41666: f64 = (-230.25850929940458);
        let assign32490_e41668: f64 = (assign32490_e41666 - var_mtat);
        let assign32490_e41670: f64 = (assign32490_e41668 * 0.3333333333333333);
        let assign32490_e41671: f64 = (1.0 + assign32490_e41670);
        let assign32490_e41672: f64 = (assign32490_e41663 * assign32490_e41671);
        let assign32490_e41673: f64 = (0.5 * assign32490_e41672);
        let assign32490_e41674: f64 = (1.0 + assign32490_e41673);
        let assign32490_e41675: f64 = (assign32490_e41657 * assign32490_e41674);
        let assign32490_e41676: f64 = (1.0 + assign32490_e41675);
        let assign32490_e41677: f64 = (1e-100 / assign32490_e41676);
        (assign32490_e41677, (-((1e-100 * (((-var_mtat_dn6) * assign32490_e41674) + (assign32490_e41657 * (0.5 * (((-var_mtat_dn6) * assign32490_e41671) + (assign32490_e41663 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign32490_e41676 * assign32490_e41676))), (-((1e-100 * (((-var_mtat_dn7) * assign32490_e41674) + (assign32490_e41657 * (0.5 * (((-var_mtat_dn7) * assign32490_e41671) + (assign32490_e41663 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign32490_e41676 * assign32490_e41676))), (-((1e-100 * (((-var_mtat_dn8) * assign32490_e41674) + (assign32490_e41657 * (0.5 * (((-var_mtat_dn8) * assign32490_e41671) + (assign32490_e41663 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign32490_e41676 * assign32490_e41676))), (-((1e-100 * (((-var_mtat_dn9) * assign32490_e41674) + (assign32490_e41657 * (0.5 * (((-var_mtat_dn9) * assign32490_e41671) + (assign32490_e41663 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign32490_e41676 * assign32490_e41676))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign32490_e41679;
        var_tmp_dn6 = assign32490_e41679_d_n6;
        var_tmp_dn7 = assign32490_e41679_d_n7;
        var_tmp_dn8 = assign32490_e41679_d_n8;
        var_tmp_dn9 = assign32490_e41679_d_n9;

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
        *var_guard651_slot = var_guard651;
        *var_guard652_slot = var_guard652;
        *var_guard653_slot = var_guard653;
        *var_guard654_slot = var_guard654;
        *var_guard655_slot = var_guard655;
        *var_guard656_slot = var_guard656;
        *var_guard657_slot = var_guard657;
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

    pub(super) fn stamp_transient_block_67(
        p: &Parameters,
        var_alphaav: f64,
        var_atatbot_d: f64,
        var_cbbtbotd_i: f64,
        var_csrhstid_i: f64,
        var_ctatbotd_i: f64,
        var_ctatstid_i: f64,
        var_erfcpos: f64,
        var_erfcpos_dn6: f64,
        var_erfcpos_dn7: f64,
        var_erfcpos_dn8: f64,
        var_erfcpos_dn9: f64,
        var_fbbtbot_d: f64,
        var_fstopbot_d: f64,
        var_ftdsti_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard648: f64,
        var_guard652: f64,
        var_guard656: f64,
        var_idmult: f64,
        var_idsatsti_d: f64,
        var_ktat: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_ktat_dn9: f64,
        var_lsdrain_i: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_pbrbotd_i: f64,
        var_pstid_i: f64,
        var_slopebot_d: f64,
        var_two_psistar: f64,
        var_v3: f64,
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
        var_guard658_slot: &mut f64,
        var_guard659_slot: &mut f64,
        var_guard660_slot: &mut f64,
        var_guard661_slot: &mut f64,
        var_guard662_slot: &mut f64,
        var_guard663_slot: &mut f64,
        var_guard664_slot: &mut f64,
        var_guard665_slot: &mut f64,
        var_guard666_slot: &mut f64,
        var_guard667_slot: &mut f64,
        var_guard668_slot: &mut f64,
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
        let mut var_guard658: f64 = *var_guard658_slot;
        let mut var_guard659: f64 = *var_guard659_slot;
        let mut var_guard660: f64 = *var_guard660_slot;
        let mut var_guard661: f64 = *var_guard661_slot;
        let mut var_guard662: f64 = *var_guard662_slot;
        let mut var_guard663: f64 = *var_guard663_slot;
        let mut var_guard664: f64 = *var_guard664_slot;
        let mut var_guard665: f64 = *var_guard665_slot;
        let mut var_guard666: f64 = *var_guard666_slot;
        let mut var_guard667: f64 = *var_guard667_slot;
        let mut var_guard668: f64 = *var_guard668_slot;
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

        let (assign32500_e41698, assign32500_e41698_d_n6, assign32500_e41698_d_n7, assign32500_e41698_d_n8, assign32500_e41698_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) && (var_guard656 == 0.0)) {
        let assign32500_e41694: f64 = (2.0 * var_tmp);
        let assign32500_e41696: f64 = (assign32500_e41694 - var_erfcpos);
        (assign32500_e41696, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign32500_e41698;
        var_erfctimesexpmtat_dn6 = assign32500_e41698_d_n6;
        var_erfctimesexpmtat_dn7 = assign32500_e41698_d_n7;
        var_erfctimesexpmtat_dn8 = assign32500_e41698_d_n8;
        var_erfctimesexpmtat_dn9 = assign32500_e41698_d_n9;

        let (assign32510_e41718, assign32510_e41718_d_n6, assign32510_e41718_d_n7, assign32510_e41718_d_n8, assign32510_e41718_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) {
        let assign32510_e41710: f64 = (1.772453850905516 * 0.5);
        let assign32510_e41713: f64 = (var_atatbot_d * var_erfctimesexpmtat);
        let assign32510_e41715: f64 = (assign32510_e41713 / var_ktat);
        let assign32510_e41716: f64 = (assign32510_e41710 * assign32510_e41715);
        (assign32510_e41716, (assign32510_e41710 * ((((var_atatbot_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign32510_e41713 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign32510_e41710 * ((((var_atatbot_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign32510_e41713 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign32510_e41710 * ((((var_atatbot_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign32510_e41713 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign32510_e41710 * ((((var_atatbot_d * var_erfctimesexpmtat_dn9) * var_ktat) - (assign32510_e41713 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign32510_e41718;
        var_gammamax_dn6 = assign32510_e41718_d_n6;
        var_gammamax_dn7 = assign32510_e41718_d_n7;
        var_gammamax_dn8 = assign32510_e41718_d_n8;
        var_gammamax_dn9 = assign32510_e41718_d_n9;

        let (assign32520_e41736, assign32520_e41736_d_n6, assign32520_e41736_d_n7, assign32520_e41736_d_n8, assign32520_e41736_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard652 == 0.0)) {
        let assign32520_e41731: f64 = (var_asrh * var_gammamax);
        let assign32520_e41733: f64 = (assign32520_e41731 * var_wtat);
        let assign32520_e41734: f64 = (var_ctatbotd_i * assign32520_e41733);
        (assign32520_e41734, (var_ctatbotd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign32520_e41731 * var_wtat_dn6))), (var_ctatbotd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign32520_e41731 * var_wtat_dn7))), (var_ctatbotd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign32520_e41731 * var_wtat_dn8))), (var_ctatbotd_i * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign32520_e41731 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign32520_e41736;
        var_itat_dn6 = assign32520_e41736_d_n6;
        var_itat_dn7 = assign32520_e41736_d_n7;
        var_itat_dn8 = assign32520_e41736_d_n8;
        var_itat_dn9 = assign32520_e41736_d_n9;

        let assign32530_e41739: f64 = if var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard658 = assign32530_e41739;

        let (assign32540_e41750, assign32540_e41750_d_n6, assign32540_e41750_d_n7, assign32540_e41750_d_n8, assign32540_e41750_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard658 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign32540_e41750;
        var_ibbt_dn6 = assign32540_e41750_d_n6;
        var_ibbt_dn7 = assign32540_e41750_d_n7;
        var_ibbt_dn8 = assign32540_e41750_d_n8;
        var_ibbt_dn9 = assign32540_e41750_d_n9;

        let assign32550_e41753: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard659 = assign32550_e41753;

        let (assign32560_e41772, assign32560_e41772_d_n6, assign32560_e41772_d_n7, assign32560_e41772_d_n8, assign32560_e41772_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard658 == 0.0)) && (var_guard659 != 0.0)) {
        let assign32560_e41767: f64 = (var_vbirbotd_i - var_vbbt);
        let assign32560_e41769: f64 = (assign32560_e41767 * var_vbirbotinv_d);
        let assign32560_e41770: f64 = (assign32560_e41769).sqrt();
        (assign32560_e41770, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign32560_e41772;
        var_tmp_dn6 = assign32560_e41772_d_n6;
        var_tmp_dn7 = assign32560_e41772_d_n7;
        var_tmp_dn8 = assign32560_e41772_d_n8;
        var_tmp_dn9 = assign32560_e41772_d_n9;

        let (assign32570_e41793, assign32570_e41793_d_n6, assign32570_e41793_d_n7, assign32570_e41793_d_n8, assign32570_e41793_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard658 == 0.0)) && (var_guard659 == 0.0)) {
        let assign32570_e41787: f64 = (var_vbirbotd_i - var_vbbt);
        let assign32570_e41789: f64 = (assign32570_e41787 * var_vbirbotinv_d);
        let assign32570_e41791: f64 = (assign32570_e41789).powf(var_pbotd_i);
        (assign32570_e41791, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign32570_e41793;
        var_tmp_dn6 = assign32570_e41793_d_n6;
        var_tmp_dn7 = assign32570_e41793_d_n7;
        var_tmp_dn8 = assign32570_e41793_d_n8;
        var_tmp_dn9 = assign32570_e41793_d_n9;

        let (assign32580_e41813, assign32580_e41813_d_n6, assign32580_e41813_d_n7, assign32580_e41813_d_n8, assign32580_e41813_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard658 == 0.0)) {
        let assign32580_e41806: f64 = (var_vbirbotd_i - var_vbbt);
        let assign32580_e41808: f64 = (assign32580_e41806 * var_wdepnulrinvbot_d);
        let assign32580_e41810: f64 = (assign32580_e41808 / var_tmp);
        let assign32580_e41811: f64 = (var_one_over_one_minus_pbot_d * assign32580_e41810);
        (assign32580_e41811, (var_one_over_one_minus_pbot_d * (-((assign32580_e41808 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign32580_e41808 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign32580_e41808 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign32580_e41808 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign32580_e41813;
        var_fmaxr_dn6 = assign32580_e41813_d_n6;
        var_fmaxr_dn7 = assign32580_e41813_d_n7;
        var_fmaxr_dn8 = assign32580_e41813_d_n8;
        var_fmaxr_dn9 = assign32580_e41813_d_n9;

        let assign32590_e41815: f64 = (-var_fbbtbot_d);
        let assign32590_e41817: f64 = (assign32590_e41815 / var_fmaxr);
        let assign32590_e41818: f64 = (assign32590_e41817).abs();
        let assign32590_e41820: f64 = if assign32590_e41818 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard660 = assign32590_e41820;

        let (assign32600_e41838, assign32600_e41838_d_n6, assign32600_e41838_d_n7, assign32600_e41838_d_n8, assign32600_e41838_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard658 == 0.0)) && (var_guard660 != 0.0)) {
        let assign32600_e41833: f64 = (-var_fbbtbot_d);
        let assign32600_e41835: f64 = (assign32600_e41833 / var_fmaxr);
        let assign32600_e41836: f64 = (assign32600_e41835).exp();
        (assign32600_e41836, (assign32600_e41836 * (-((assign32600_e41833 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign32600_e41836 * (-((assign32600_e41833 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign32600_e41836 * (-((assign32600_e41833 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign32600_e41836 * (-((assign32600_e41833 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign32600_e41838;
        var_tmp_dn6 = assign32600_e41838_d_n6;
        var_tmp_dn7 = assign32600_e41838_d_n7;
        var_tmp_dn8 = assign32600_e41838_d_n8;
        var_tmp_dn9 = assign32600_e41838_d_n9;

        let assign32610_e41840: f64 = (-var_fbbtbot_d);
        let assign32610_e41842: f64 = (assign32610_e41840 / var_fmaxr);
        let assign32610_e41844: f64 = if assign32610_e41842 < 0.0 { 1.0 } else { 0.0 };
        var_guard661 = assign32610_e41844;

        let (assign32620_e41895, assign32620_e41895_d_n6, assign32620_e41895_d_n7, assign32620_e41895_d_n8, assign32620_e41895_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard658 == 0.0)) && (var_guard660 == 0.0)) && (var_guard661 != 0.0)) {
        let assign32620_e41862: f64 = (-230.25850929940458);
        let assign32620_e41864: f64 = (-var_fbbtbot_d);
        let assign32620_e41866: f64 = (assign32620_e41864 / var_fmaxr);
        let assign32620_e41867: f64 = (assign32620_e41862 - assign32620_e41866);
        let assign32620_e41871: f64 = (-230.25850929940458);
        let assign32620_e41873: f64 = (-var_fbbtbot_d);
        let assign32620_e41875: f64 = (assign32620_e41873 / var_fmaxr);
        let assign32620_e41876: f64 = (assign32620_e41871 - assign32620_e41875);
        let assign32620_e41879: f64 = (-230.25850929940458);
        let assign32620_e41881: f64 = (-var_fbbtbot_d);
        let assign32620_e41883: f64 = (assign32620_e41881 / var_fmaxr);
        let assign32620_e41884: f64 = (assign32620_e41879 - assign32620_e41883);
        let assign32620_e41886: f64 = (assign32620_e41884 * 0.3333333333333333);
        let assign32620_e41887: f64 = (1.0 + assign32620_e41886);
        let assign32620_e41888: f64 = (assign32620_e41876 * assign32620_e41887);
        let assign32620_e41889: f64 = (0.5 * assign32620_e41888);
        let assign32620_e41890: f64 = (1.0 + assign32620_e41889);
        let assign32620_e41891: f64 = (assign32620_e41867 * assign32620_e41890);
        let assign32620_e41892: f64 = (1.0 + assign32620_e41891);
        let assign32620_e41893: f64 = (1e-100 / assign32620_e41892);
        (assign32620_e41893, (-((1e-100 * (((-(-((assign32620_e41864 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign32620_e41890) + (assign32620_e41867 * (0.5 * (((-(-((assign32620_e41873 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign32620_e41887) + (assign32620_e41876 * ((-(-((assign32620_e41881 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign32620_e41892 * assign32620_e41892))), (-((1e-100 * (((-(-((assign32620_e41864 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign32620_e41890) + (assign32620_e41867 * (0.5 * (((-(-((assign32620_e41873 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign32620_e41887) + (assign32620_e41876 * ((-(-((assign32620_e41881 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign32620_e41892 * assign32620_e41892))), (-((1e-100 * (((-(-((assign32620_e41864 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign32620_e41890) + (assign32620_e41867 * (0.5 * (((-(-((assign32620_e41873 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign32620_e41887) + (assign32620_e41876 * ((-(-((assign32620_e41881 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign32620_e41892 * assign32620_e41892))), (-((1e-100 * (((-(-((assign32620_e41864 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign32620_e41890) + (assign32620_e41867 * (0.5 * (((-(-((assign32620_e41873 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign32620_e41887) + (assign32620_e41876 * ((-(-((assign32620_e41881 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign32620_e41892 * assign32620_e41892))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign32620_e41895;
        var_tmp_dn6 = assign32620_e41895_d_n6;
        var_tmp_dn7 = assign32620_e41895_d_n7;
        var_tmp_dn8 = assign32620_e41895_d_n8;
        var_tmp_dn9 = assign32620_e41895_d_n9;

        let (assign32630_e41944, assign32630_e41944_d_n6, assign32630_e41944_d_n7, assign32630_e41944_d_n8, assign32630_e41944_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard658 == 0.0)) && (var_guard660 == 0.0)) && (var_guard661 == 0.0)) {
        let assign32630_e41914: f64 = (-var_fbbtbot_d);
        let assign32630_e41916: f64 = (assign32630_e41914 / var_fmaxr);
        let assign32630_e41918: f64 = (assign32630_e41916 - 230.25850929940458);
        let assign32630_e41922: f64 = (-var_fbbtbot_d);
        let assign32630_e41924: f64 = (assign32630_e41922 / var_fmaxr);
        let assign32630_e41926: f64 = (assign32630_e41924 - 230.25850929940458);
        let assign32630_e41929: f64 = (-var_fbbtbot_d);
        let assign32630_e41931: f64 = (assign32630_e41929 / var_fmaxr);
        let assign32630_e41933: f64 = (assign32630_e41931 - 230.25850929940458);
        let assign32630_e41935: f64 = (assign32630_e41933 * 0.3333333333333333);
        let assign32630_e41936: f64 = (1.0 + assign32630_e41935);
        let assign32630_e41937: f64 = (assign32630_e41926 * assign32630_e41936);
        let assign32630_e41938: f64 = (0.5 * assign32630_e41937);
        let assign32630_e41939: f64 = (1.0 + assign32630_e41938);
        let assign32630_e41940: f64 = (assign32630_e41918 * assign32630_e41939);
        let assign32630_e41941: f64 = (1.0 + assign32630_e41940);
        let assign32630_e41942: f64 = (1e100 * assign32630_e41941);
        (assign32630_e41942, (1e100 * (((-((assign32630_e41914 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign32630_e41939) + (assign32630_e41918 * (0.5 * (((-((assign32630_e41922 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign32630_e41936) + (assign32630_e41926 * ((-((assign32630_e41929 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign32630_e41914 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign32630_e41939) + (assign32630_e41918 * (0.5 * (((-((assign32630_e41922 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign32630_e41936) + (assign32630_e41926 * ((-((assign32630_e41929 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign32630_e41914 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign32630_e41939) + (assign32630_e41918 * (0.5 * (((-((assign32630_e41922 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign32630_e41936) + (assign32630_e41926 * ((-((assign32630_e41929 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign32630_e41914 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign32630_e41939) + (assign32630_e41918 * (0.5 * (((-((assign32630_e41922 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign32630_e41936) + (assign32630_e41926 * ((-((assign32630_e41929 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign32630_e41944;
        var_tmp_dn6 = assign32630_e41944_d_n6;
        var_tmp_dn7 = assign32630_e41944_d_n7;
        var_tmp_dn8 = assign32630_e41944_d_n8;
        var_tmp_dn9 = assign32630_e41944_d_n9;

        let (assign32640_e41964, assign32640_e41964_d_n6, assign32640_e41964_d_n7, assign32640_e41964_d_n8, assign32640_e41964_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard658 == 0.0)) {
        let assign32640_e41957: f64 = (var_v3 * var_fmaxr);
        let assign32640_e41959: f64 = (assign32640_e41957 * var_fmaxr);
        let assign32640_e41961: f64 = (assign32640_e41959 * var_tmp);
        let assign32640_e41962: f64 = (var_cbbtbotd_i * assign32640_e41961);
        (assign32640_e41962, (var_cbbtbotd_i * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign32640_e41957 * var_fmaxr_dn6)) * var_tmp) + (assign32640_e41959 * var_tmp_dn6))), (var_cbbtbotd_i * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign32640_e41957 * var_fmaxr_dn7)) * var_tmp) + (assign32640_e41959 * var_tmp_dn7))), (var_cbbtbotd_i * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign32640_e41957 * var_fmaxr_dn8)) * var_tmp) + (assign32640_e41959 * var_tmp_dn8))), (var_cbbtbotd_i * (((((var_v3 * var_fmaxr_dn9) * var_fmaxr) + (assign32640_e41957 * var_fmaxr_dn9)) * var_tmp) + (assign32640_e41959 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign32640_e41964;
        var_ibbt_dn6 = assign32640_e41964_d_n6;
        var_ibbt_dn7 = assign32640_e41964_d_n7;
        var_ibbt_dn8 = assign32640_e41964_d_n8;
        var_ibbt_dn9 = assign32640_e41964_d_n9;

        let assign32650_e41967: f64 = if var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard662 = assign32650_e41967;

        let (assign32660_e41978, assign32660_e41978_d_n6, assign32660_e41978_d_n7, assign32660_e41978_d_n8, assign32660_e41978_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard662 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign32660_e41978;
        var_fbreakdown_dn6 = assign32660_e41978_d_n6;
        var_fbreakdown_dn7 = assign32660_e41978_d_n7;
        var_fbreakdown_dn8 = assign32660_e41978_d_n8;
        var_fbreakdown_dn9 = assign32660_e41978_d_n9;

        let assign32670_e41981: f64 = (-var_alphaav);
        let assign32670_e41983: f64 = (assign32670_e41981 * var_vbrbotd_i);
        let assign32670_e41984: f64 = if var_vav > assign32670_e41983 { 1.0 } else { 0.0 };
        var_guard663 = assign32670_e41984;

        let assign32680_e41987: f64 = if var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard664 = assign32680_e41987;

        let (assign32690_e42017, assign32690_e42017_d_n6, assign32690_e42017_d_n7, assign32690_e42017_d_n8, assign32690_e42017_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard662 == 0.0)) && (var_guard663 != 0.0)) && (var_guard664 != 0.0)) {
        let assign32690_e42003: f64 = (var_vav * var_vbrinvbot_d);
        let assign32690_e42006: f64 = (var_vav * var_vbrinvbot_d);
        let assign32690_e42007: f64 = (assign32690_e42003 * assign32690_e42006);
        let assign32690_e42010: f64 = (var_vav * var_vbrinvbot_d);
        let assign32690_e42011: f64 = (assign32690_e42007 * assign32690_e42010);
        let assign32690_e42014: f64 = (var_vav * var_vbrinvbot_d);
        let assign32690_e42015: f64 = (assign32690_e42011 * assign32690_e42014);
        (assign32690_e42015, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign32690_e42017;
        var_tmp_dn6 = assign32690_e42017_d_n6;
        var_tmp_dn7 = assign32690_e42017_d_n7;
        var_tmp_dn8 = assign32690_e42017_d_n8;
        var_tmp_dn9 = assign32690_e42017_d_n9;

        let (assign32700_e42039, assign32700_e42039_d_n6, assign32700_e42039_d_n7, assign32700_e42039_d_n8, assign32700_e42039_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard662 == 0.0)) && (var_guard663 != 0.0)) && (var_guard664 == 0.0)) {
        let assign32700_e42034: f64 = (var_vav * var_vbrinvbot_d);
        let assign32700_e42035: f64 = (assign32700_e42034).abs();
        let assign32700_e42037: f64 = (assign32700_e42035).powf(var_pbrbotd_i);
        (assign32700_e42037, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign32700_e42039;
        var_tmp_dn6 = assign32700_e42039_d_n6;
        var_tmp_dn7 = assign32700_e42039_d_n7;
        var_tmp_dn8 = assign32700_e42039_d_n8;
        var_tmp_dn9 = assign32700_e42039_d_n9;

        let (assign32710_e42057, assign32710_e42057_d_n6, assign32710_e42057_d_n7, assign32710_e42057_d_n8, assign32710_e42057_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard662 == 0.0)) && (var_guard663 != 0.0)) {
        let assign32710_e42054: f64 = (1.0 - var_tmp);
        let assign32710_e42055: f64 = (1.0 / assign32710_e42054);
        (assign32710_e42055, (-((-var_tmp_dn6) / (assign32710_e42054 * assign32710_e42054))), (-((-var_tmp_dn7) / (assign32710_e42054 * assign32710_e42054))), (-((-var_tmp_dn8) / (assign32710_e42054 * assign32710_e42054))), (-((-var_tmp_dn9) / (assign32710_e42054 * assign32710_e42054))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign32710_e42057;
        var_fbreakdown_dn6 = assign32710_e42057_d_n6;
        var_fbreakdown_dn7 = assign32710_e42057_d_n7;
        var_fbreakdown_dn8 = assign32710_e42057_d_n8;
        var_fbreakdown_dn9 = assign32710_e42057_d_n9;

        let (assign32720_e42080, assign32720_e42080_d_n6, assign32720_e42080_d_n7, assign32720_e42080_d_n8, assign32720_e42080_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) && (var_guard662 == 0.0)) && (var_guard663 == 0.0)) {
        let assign32720_e42074: f64 = (var_alphaav * var_vbrbotd_i);
        let assign32720_e42075: f64 = (var_vav + assign32720_e42074);
        let assign32720_e42077: f64 = (assign32720_e42075 * var_slopebot_d);
        let assign32720_e42078: f64 = (var_fstopbot_d + assign32720_e42077);
        (assign32720_e42078, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign32720_e42080;
        var_fbreakdown_dn6 = assign32720_e42080_d_n6;
        var_fbreakdown_dn7 = assign32720_e42080_d_n7;
        var_fbreakdown_dn8 = assign32720_e42080_d_n8;
        var_fbreakdown_dn9 = assign32720_e42080_d_n9;

        let (assign32730_e42099, assign32730_e42099_d_n6, assign32730_e42099_d_n7, assign32730_e42099_d_n8, assign32730_e42099_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard648 == 0.0)) {
        let assign32730_e42090: f64 = (var_id__blk212 + var_isrh);
        let assign32730_e42092: f64 = (assign32730_e42090 + var_itat);
        let assign32730_e42094: f64 = (assign32730_e42092 + var_ibbt);
        let assign32730_e42095: f64 = (p.p29 * assign32730_e42094);
        let assign32730_e42097: f64 = (assign32730_e42095 * var_fbreakdown);
        (assign32730_e42097, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign32730_e42095 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign32730_e42095 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign32730_e42095 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign32730_e42095 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign32730_e42099;
        var_ijunbot_dn6 = assign32730_e42099_d_n6;
        var_ijunbot_dn7 = assign32730_e42099_d_n7;
        var_ijunbot_dn8 = assign32730_e42099_d_n8;
        var_ijunbot_dn9 = assign32730_e42099_d_n9;

        let assign32740_e42102: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard665 = assign32740_e42102;

        let (assign32750_e42110, assign32750_e42110_d_n6, assign32750_e42110_d_n7, assign32750_e42110_d_n8, assign32750_e42110_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign32750_e42110;
        var_ijunsti_dn6 = assign32750_e42110_d_n6;
        var_ijunsti_dn7 = assign32750_e42110_d_n7;
        var_ijunsti_dn8 = assign32750_e42110_d_n8;
        var_ijunsti_dn9 = assign32750_e42110_d_n9;

        let (assign32760_e42121,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) {
        let assign32760_e42119: f64 = (var_idsatsti_d * var_idmult);
        (assign32760_e42119,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign32760_e42121;

        let assign32770_e42128: f64 = if ((var_csrhstid_i == 0.0) && (var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard666 = assign32770_e42128;

        let (assign32780_e42139, assign32780_e42139_d_n6, assign32780_e42139_d_n7, assign32780_e42139_d_n8, assign32780_e42139_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard666 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign32780_e42139;
        var_isrh_dn6 = assign32780_e42139_d_n6;
        var_isrh_dn7 = assign32780_e42139_d_n7;
        var_isrh_dn8 = assign32780_e42139_d_n8;
        var_isrh_dn9 = assign32780_e42139_d_n9;

        let (assign32790_e42153,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard666 == 0.0)) {
        let assign32790_e42151: f64 = (var_vbisti_d - var_vjsrh);
        (assign32790_e42151,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign32790_e42153;

        let (assign32800_e42172,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard666 == 0.0)) {
        let assign32800_e42167: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign32800_e42168: f64 = (1.0 - assign32800_e42167);
        let assign32800_e42169: f64 = (assign32800_e42168).sqrt();
        let assign32800_e42170: f64 = (1.0 - assign32800_e42169);
        (assign32800_e42170,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign32800_e42172;

        let assign32810_e42175: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard667 = assign32810_e42175;

        let (assign32820_e42189,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard666 == 0.0)) && (var_guard667 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign32820_e42189;

        let (assign32830_e42221,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard666 == 0.0)) && (var_guard667 == 0.0)) {
        let assign32830_e42204: f64 = (var_wsrhstep * var_wsrhstep);
        let assign32830_e42206: f64 = (var_wsrhstep).ln();
        let assign32830_e42207: f64 = (assign32830_e42204 * assign32830_e42206);
        let assign32830_e42210: f64 = (1.0 - var_wsrhstep);
        let assign32830_e42211: f64 = (assign32830_e42207 / assign32830_e42210);
        let assign32830_e42213: f64 = (assign32830_e42211 + var_wsrhstep);
        let assign32830_e42217: f64 = (2.0 * var_pstid_i);
        let assign32830_e42218: f64 = (1.0 - assign32830_e42217);
        let assign32830_e42219: f64 = (assign32830_e42213 * assign32830_e42218);
        (assign32830_e42219,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign32830_e42221;

        let (assign32840_e42235,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard666 == 0.0)) {
        let assign32840_e42233: f64 = (var_wsrhstep + var_dwsrh);
        (assign32840_e42233,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign32840_e42235;

        let assign32850_e42238: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard668 = assign32850_e42238;

        let (assign32860_e42255, assign32860_e42255_d_n6, assign32860_e42255_d_n7, assign32860_e42255_d_n8, assign32860_e42255_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard666 == 0.0)) && (var_guard668 != 0.0)) {
        let assign32860_e42252: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign32860_e42253: f64 = (assign32860_e42252).sqrt();
        (assign32860_e42253, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign32860_e42255;
        var_tmp_dn6 = assign32860_e42255_d_n6;
        var_tmp_dn7 = assign32860_e42255_d_n7;
        var_tmp_dn8 = assign32860_e42255_d_n8;
        var_tmp_dn9 = assign32860_e42255_d_n9;

        let (assign32870_e42274, assign32870_e42274_d_n6, assign32870_e42274_d_n7, assign32870_e42274_d_n8, assign32870_e42274_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard666 == 0.0)) && (var_guard668 == 0.0)) {
        let assign32870_e42270: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign32870_e42272: f64 = (assign32870_e42270).powf(var_pstid_i);
        (assign32870_e42272, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign32870_e42274;
        var_tmp_dn6 = assign32870_e42274_d_n6;
        var_tmp_dn7 = assign32870_e42274_d_n7;
        var_tmp_dn8 = assign32870_e42274_d_n8;
        var_tmp_dn9 = assign32870_e42274_d_n9;

        let (assign32880_e42288, assign32880_e42288_d_n6, assign32880_e42288_d_n7, assign32880_e42288_d_n8, assign32880_e42288_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard666 == 0.0)) {
        let assign32880_e42286: f64 = (var_wdepnulrsti_d * var_tmp);
        (assign32880_e42286, (var_wdepnulrsti_d * var_tmp_dn6), (var_wdepnulrsti_d * var_tmp_dn7), (var_wdepnulrsti_d * var_tmp_dn8), (var_wdepnulrsti_d * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign32880_e42288;
        var_wdep_dn6 = assign32880_e42288_d_n6;
        var_wdep_dn7 = assign32880_e42288_d_n7;
        var_wdep_dn8 = assign32880_e42288_d_n8;
        var_wdep_dn9 = assign32880_e42288_d_n9;

        let (assign32890_e42306, assign32890_e42306_d_n6, assign32890_e42306_d_n7, assign32890_e42306_d_n8, assign32890_e42306_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard666 == 0.0)) {
        let assign32890_e42301: f64 = (var_zinv - 1.0);
        let assign32890_e42303: f64 = (assign32890_e42301 * var_wdep);
        let assign32890_e42304: f64 = (var_ftdsti_d * assign32890_e42303);
        (assign32890_e42304, (var_ftdsti_d * (assign32890_e42301 * var_wdep_dn6)), (var_ftdsti_d * (assign32890_e42301 * var_wdep_dn7)), (var_ftdsti_d * (assign32890_e42301 * var_wdep_dn8)), (var_ftdsti_d * (assign32890_e42301 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign32890_e42306;
        var_asrh_dn6 = assign32890_e42306_d_n6;
        var_asrh_dn7 = assign32890_e42306_d_n7;
        var_asrh_dn8 = assign32890_e42306_d_n8;
        var_asrh_dn9 = assign32890_e42306_d_n9;

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
        *var_guard658_slot = var_guard658;
        *var_guard659_slot = var_guard659;
        *var_guard660_slot = var_guard660;
        *var_guard661_slot = var_guard661;
        *var_guard662_slot = var_guard662;
        *var_guard663_slot = var_guard663;
        *var_guard664_slot = var_guard664;
        *var_guard665_slot = var_guard665;
        *var_guard666_slot = var_guard666;
        *var_guard667_slot = var_guard667;
        *var_guard668_slot = var_guard668;
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

    pub(super) fn stamp_transient_block_68(
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatsti_d: f64,
        var_berfc: f64,
        var_btatpartsti_d: f64,
        var_cbbtstid_i: f64,
        var_cerfc: f64,
        var_csrhstid_i: f64,
        var_ctatstid_i: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard665: f64,
        var_guard666: f64,
        var_one_minus_psti_d: f64,
        var_one_over_one_minus_psti_d: f64,
        var_perfc: f64,
        var_pstid_i: f64,
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
        var_guard669_slot: &mut f64,
        var_guard670_slot: &mut f64,
        var_guard671_slot: &mut f64,
        var_guard672_slot: &mut f64,
        var_guard673_slot: &mut f64,
        var_guard674_slot: &mut f64,
        var_guard675_slot: &mut f64,
        var_guard676_slot: &mut f64,
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
        let mut var_guard669: f64 = *var_guard669_slot;
        let mut var_guard670: f64 = *var_guard670_slot;
        let mut var_guard671: f64 = *var_guard671_slot;
        let mut var_guard672: f64 = *var_guard672_slot;
        let mut var_guard673: f64 = *var_guard673_slot;
        let mut var_guard674: f64 = *var_guard674_slot;
        let mut var_guard675: f64 = *var_guard675_slot;
        let mut var_guard676: f64 = *var_guard676_slot;
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

        let (assign32900_e42322, assign32900_e42322_d_n6, assign32900_e42322_d_n7, assign32900_e42322_d_n8, assign32900_e42322_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard666 == 0.0)) {
        let assign32900_e42319: f64 = (var_asrh * var_wsrh);
        let assign32900_e42320: f64 = (var_csrhstid_i * assign32900_e42319);
        (assign32900_e42320, (var_csrhstid_i * (var_asrh_dn6 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn7 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn8 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign32900_e42322;
        var_isrh_dn6 = assign32900_e42322_d_n6;
        var_isrh_dn7 = assign32900_e42322_d_n7;
        var_isrh_dn8 = assign32900_e42322_d_n8;
        var_isrh_dn9 = assign32900_e42322_d_n9;

        let assign32910_e42325: f64 = if var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard669 = assign32910_e42325;

        let (assign32920_e42336, assign32920_e42336_d_n6, assign32920_e42336_d_n7, assign32920_e42336_d_n8, assign32920_e42336_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign32920_e42336;
        var_itat_dn6 = assign32920_e42336_d_n6;
        var_itat_dn7 = assign32920_e42336_d_n7;
        var_itat_dn8 = assign32920_e42336_d_n8;
        var_itat_dn9 = assign32920_e42336_d_n9;

        let (assign32930_e42354, assign32930_e42354_d_n6, assign32930_e42354_d_n7, assign32930_e42354_d_n8, assign32930_e42354_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) {
        let assign32930_e42349: f64 = (var_wdep * var_one_minus_psti_d);
        let assign32930_e42351: f64 = (assign32930_e42349 / var_vbi_minus_vjsrh);
        let assign32930_e42352: f64 = (var_btatpartsti_d * assign32930_e42351);
        (assign32930_e42352, (var_btatpartsti_d * ((var_wdep_dn6 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn7 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn8 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn9 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign32930_e42354;
        var_btat_dn6 = assign32930_e42354_d_n6;
        var_btat_dn7 = assign32930_e42354_d_n7;
        var_btat_dn8 = assign32930_e42354_d_n8;
        var_btat_dn9 = assign32930_e42354_d_n9;

        let (assign32940_e42370, assign32940_e42370_d_n6, assign32940_e42370_d_n7, assign32940_e42370_d_n8, assign32940_e42370_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) {
        let assign32940_e42366: f64 = (0.666666666666667 * var_atatsti_d);
        let assign32940_e42368: f64 = (assign32940_e42366 / var_btat);
        (assign32940_e42368, (-((assign32940_e42366 * var_btat_dn6) / (var_btat * var_btat))), (-((assign32940_e42366 * var_btat_dn7) / (var_btat * var_btat))), (-((assign32940_e42366 * var_btat_dn8) / (var_btat * var_btat))), (-((assign32940_e42366 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign32940_e42370;
        var_twoatatoverthreebtat_dn6 = assign32940_e42370_d_n6;
        var_twoatatoverthreebtat_dn7 = assign32940_e42370_d_n7;
        var_twoatatoverthreebtat_dn8 = assign32940_e42370_d_n8;
        var_twoatatoverthreebtat_dn9 = assign32940_e42370_d_n9;

        let (assign32950_e42384, assign32950_e42384_d_n6, assign32950_e42384_d_n7, assign32950_e42384_d_n8, assign32950_e42384_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) {
        let assign32950_e42382: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign32950_e42382, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign32950_e42384;
        var_umaxbeforelimiting_dn6 = assign32950_e42384_d_n6;
        var_umaxbeforelimiting_dn7 = assign32950_e42384_d_n7;
        var_umaxbeforelimiting_dn8 = assign32950_e42384_d_n8;
        var_umaxbeforelimiting_dn9 = assign32950_e42384_d_n9;

        let (assign32960_e42405, assign32960_e42405_d_n6, assign32960_e42405_d_n7, assign32960_e42405_d_n8, assign32960_e42405_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) {
        let assign32960_e42396: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign32960_e42399: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign32960_e42401: f64 = (assign32960_e42399 + 1.0);
        let assign32960_e42402: f64 = (assign32960_e42396 / assign32960_e42401);
        let assign32960_e42403: f64 = (assign32960_e42402).sqrt();
        (assign32960_e42403, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign32960_e42401) - (assign32960_e42396 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign32960_e42401 * assign32960_e42401)) / (2.0 * assign32960_e42403)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign32960_e42401) - (assign32960_e42396 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign32960_e42401 * assign32960_e42401)) / (2.0 * assign32960_e42403)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign32960_e42401) - (assign32960_e42396 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign32960_e42401 * assign32960_e42401)) / (2.0 * assign32960_e42403)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign32960_e42401) - (assign32960_e42396 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign32960_e42401 * assign32960_e42401)) / (2.0 * assign32960_e42403)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign32960_e42405;
        var_umax_dn6 = assign32960_e42405_d_n6;
        var_umax_dn7 = assign32960_e42405_d_n7;
        var_umax_dn8 = assign32960_e42405_d_n8;
        var_umax_dn9 = assign32960_e42405_d_n9;

        let (assign32970_e42418, assign32970_e42418_d_n6, assign32970_e42418_d_n7, assign32970_e42418_d_n8, assign32970_e42418_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) {
        let assign32970_e42416: f64 = (var_umax).sqrt();
        (assign32970_e42416, (var_umax_dn6 / (2.0 * assign32970_e42416)), (var_umax_dn7 / (2.0 * assign32970_e42416)), (var_umax_dn8 / (2.0 * assign32970_e42416)), (var_umax_dn9 / (2.0 * assign32970_e42416)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign32970_e42418;
        var_sqrtumax_dn6 = assign32970_e42418_d_n6;
        var_sqrtumax_dn7 = assign32970_e42418_d_n7;
        var_sqrtumax_dn8 = assign32970_e42418_d_n8;
        var_sqrtumax_dn9 = assign32970_e42418_d_n9;

        let (assign32980_e42432, assign32980_e42432_d_n6, assign32980_e42432_d_n7, assign32980_e42432_d_n8, assign32980_e42432_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) {
        let assign32980_e42430: f64 = (var_umax * var_sqrtumax);
        (assign32980_e42430, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign32980_e42432;
        var_umaxpoweronepointfive_dn6 = assign32980_e42432_d_n6;
        var_umaxpoweronepointfive_dn7 = assign32980_e42432_d_n7;
        var_umaxpoweronepointfive_dn8 = assign32980_e42432_d_n8;
        var_umaxpoweronepointfive_dn9 = assign32980_e42432_d_n9;

        let assign32990_e42434: f64 = (-var_pstid_i);
        let assign32990_e42436: f64 = (assign32990_e42434 * var_one_over_one_minus_psti_d);
        let assign32990_e42438: f64 = (-1.0);
        let assign32990_e42439: f64 = if assign32990_e42436 == assign32990_e42438 { 1.0 } else { 0.0 };
        var_guard670 = assign32990_e42439;

        let (assign33000_e42459, assign33000_e42459_d_n6, assign33000_e42459_d_n7, assign33000_e42459_d_n8, assign33000_e42459_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) && (var_guard670 != 0.0)) {
        let assign33000_e42455: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign33000_e42456: f64 = (1.0 + assign33000_e42455);
        let assign33000_e42457: f64 = (1.0 / assign33000_e42456);
        (assign33000_e42457, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign33000_e42456 * assign33000_e42456))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign33000_e42456 * assign33000_e42456))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign33000_e42456 * assign33000_e42456))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign33000_e42456 * assign33000_e42456))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign33000_e42459;
        var_wgamma_dn6 = assign33000_e42459_d_n6;
        var_wgamma_dn7 = assign33000_e42459_d_n7;
        var_wgamma_dn8 = assign33000_e42459_d_n8;
        var_wgamma_dn9 = assign33000_e42459_d_n9;

        let (assign33010_e42483, assign33010_e42483_d_n6, assign33010_e42483_d_n7, assign33010_e42483_d_n8, assign33010_e42483_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) && (var_guard670 == 0.0)) {
        let assign33010_e42475: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign33010_e42476: f64 = (1.0 + assign33010_e42475);
        let assign33010_e42478: f64 = (-var_pstid_i);
        let assign33010_e42480: f64 = (assign33010_e42478 * var_one_over_one_minus_psti_d);
        let assign33010_e42481: f64 = (assign33010_e42476).powf(assign33010_e42480);
        (assign33010_e42481, if 0.0 == 0.0 && ((assign33010_e42480) as f64).is_finite() && ((assign33010_e42480) as f64).fract() == 0.0 { if assign33010_e42480 == 0.0 { 0.0 } else { (assign33010_e42480 * ((assign33010_e42476).powf(assign33010_e42480 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign33010_e42481 * (assign33010_e42480 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign33010_e42476))) }, if 0.0 == 0.0 && ((assign33010_e42480) as f64).is_finite() && ((assign33010_e42480) as f64).fract() == 0.0 { if assign33010_e42480 == 0.0 { 0.0 } else { (assign33010_e42480 * ((assign33010_e42476).powf(assign33010_e42480 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign33010_e42481 * (assign33010_e42480 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign33010_e42476))) }, if 0.0 == 0.0 && ((assign33010_e42480) as f64).is_finite() && ((assign33010_e42480) as f64).fract() == 0.0 { if assign33010_e42480 == 0.0 { 0.0 } else { (assign33010_e42480 * ((assign33010_e42476).powf(assign33010_e42480 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign33010_e42481 * (assign33010_e42480 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign33010_e42476))) }, if 0.0 == 0.0 && ((assign33010_e42480) as f64).is_finite() && ((assign33010_e42480) as f64).fract() == 0.0 { if assign33010_e42480 == 0.0 { 0.0 } else { (assign33010_e42480 * ((assign33010_e42476).powf(assign33010_e42480 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign33010_e42481 * (assign33010_e42480 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign33010_e42476))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign33010_e42483;
        var_wgamma_dn6 = assign33010_e42483_d_n6;
        var_wgamma_dn7 = assign33010_e42483_d_n7;
        var_wgamma_dn8 = assign33010_e42483_d_n8;
        var_wgamma_dn9 = assign33010_e42483_d_n9;

        let (assign33020_e42501, assign33020_e42501_d_n6, assign33020_e42501_d_n7, assign33020_e42501_d_n8, assign33020_e42501_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) {
        let assign33020_e42495: f64 = (var_wsrh * var_wgamma);
        let assign33020_e42498: f64 = (var_wsrh + var_wgamma);
        let assign33020_e42499: f64 = (assign33020_e42495 / assign33020_e42498);
        (assign33020_e42499, ((((var_wsrh * var_wgamma_dn6) * assign33020_e42498) - (assign33020_e42495 * var_wgamma_dn6)) / (assign33020_e42498 * assign33020_e42498)), ((((var_wsrh * var_wgamma_dn7) * assign33020_e42498) - (assign33020_e42495 * var_wgamma_dn7)) / (assign33020_e42498 * assign33020_e42498)), ((((var_wsrh * var_wgamma_dn8) * assign33020_e42498) - (assign33020_e42495 * var_wgamma_dn8)) / (assign33020_e42498 * assign33020_e42498)), ((((var_wsrh * var_wgamma_dn9) * assign33020_e42498) - (assign33020_e42495 * var_wgamma_dn9)) / (assign33020_e42498 * assign33020_e42498)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign33020_e42501;
        var_wtat_dn6 = assign33020_e42501_d_n6;
        var_wtat_dn7 = assign33020_e42501_d_n7;
        var_wtat_dn8 = assign33020_e42501_d_n8;
        var_wtat_dn9 = assign33020_e42501_d_n9;

        let (assign33030_e42518, assign33030_e42518_d_n6, assign33030_e42518_d_n7, assign33030_e42518_d_n8, assign33030_e42518_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) {
        let assign33030_e42514: f64 = (var_btat / var_sqrtumax);
        let assign33030_e42515: f64 = (0.375 * assign33030_e42514);
        let assign33030_e42516: f64 = (assign33030_e42515).sqrt();
        (assign33030_e42516, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33030_e42516)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33030_e42516)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33030_e42516)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33030_e42516)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign33030_e42518;
        var_ktat_dn6 = assign33030_e42518_d_n6;
        var_ktat_dn7 = assign33030_e42518_d_n7;
        var_ktat_dn8 = assign33030_e42518_d_n8;
        var_ktat_dn9 = assign33030_e42518_d_n9;

        let (assign33040_e42536, assign33040_e42536_d_n6, assign33040_e42536_d_n7, assign33040_e42536_d_n8, assign33040_e42536_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) {
        let assign33040_e42531: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign33040_e42532: f64 = (2.0 * assign33040_e42531);
        let assign33040_e42534: f64 = (assign33040_e42532 - var_umax);
        (assign33040_e42534, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign33040_e42536;
        var_ltat_dn6 = assign33040_e42536_d_n6;
        var_ltat_dn7 = assign33040_e42536_d_n7;
        var_ltat_dn8 = assign33040_e42536_d_n8;
        var_ltat_dn9 = assign33040_e42536_d_n9;

        let (assign33050_e42562, assign33050_e42562_d_n6, assign33050_e42562_d_n7, assign33050_e42562_d_n8, assign33050_e42562_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) {
        let assign33050_e42548: f64 = (var_atatsti_d * var_twoatatoverthreebtat);
        let assign33050_e42550: f64 = (assign33050_e42548 * var_sqrtumax);
        let assign33050_e42553: f64 = (var_atatsti_d * var_umax);
        let assign33050_e42554: f64 = (assign33050_e42550 - assign33050_e42553);
        let assign33050_e42558: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign33050_e42559: f64 = (0.5 * assign33050_e42558);
        let assign33050_e42560: f64 = (assign33050_e42554 + assign33050_e42559);
        (assign33050_e42560, (((((var_atatsti_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign33050_e42548 * var_sqrtumax_dn6)) - (var_atatsti_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign33050_e42548 * var_sqrtumax_dn7)) - (var_atatsti_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign33050_e42548 * var_sqrtumax_dn8)) - (var_atatsti_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign33050_e42548 * var_sqrtumax_dn9)) - (var_atatsti_d * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign33050_e42562;
        var_mtat_dn6 = assign33050_e42562_d_n6;
        var_mtat_dn7 = assign33050_e42562_d_n7;
        var_mtat_dn8 = assign33050_e42562_d_n8;
        var_mtat_dn9 = assign33050_e42562_d_n9;

        let (assign33060_e42578, assign33060_e42578_d_n6, assign33060_e42578_d_n7, assign33060_e42578_d_n8, assign33060_e42578_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) {
        let assign33060_e42574: f64 = (var_ltat - 1.0);
        let assign33060_e42576: f64 = (assign33060_e42574 * var_ktat);
        (assign33060_e42576, ((var_ltat_dn6 * var_ktat) + (assign33060_e42574 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign33060_e42574 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign33060_e42574 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign33060_e42574 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign33060_e42578;
        var_xerfc_dn6 = assign33060_e42578_d_n6;
        var_xerfc_dn7 = assign33060_e42578_d_n7;
        var_xerfc_dn8 = assign33060_e42578_d_n8;
        var_xerfc_dn9 = assign33060_e42578_d_n9;

        let (assign33070_e42592, assign33070_e42592_d_n6, assign33070_e42592_d_n7, assign33070_e42592_d_n8, assign33070_e42592_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) {
        let assign33070_e42590: f64 = (var_xerfc * var_xerfc);
        (assign33070_e42590, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign33070_e42592;
        var_ysq_dn6 = assign33070_e42592_d_n6;
        var_ysq_dn7 = assign33070_e42592_d_n7;
        var_ysq_dn8 = assign33070_e42592_d_n8;
        var_ysq_dn9 = assign33070_e42592_d_n9;

        let assign33080_e42595: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard671 = assign33080_e42595;

        let (assign33090_e42615, assign33090_e42615_d_n6, assign33090_e42615_d_n7, assign33090_e42615_d_n8, assign33090_e42615_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) && (var_guard671 != 0.0)) {
        let assign33090_e42611: f64 = (var_perfc * var_xerfc);
        let assign33090_e42612: f64 = (1.0 + assign33090_e42611);
        let assign33090_e42613: f64 = (1.0 / assign33090_e42612);
        (assign33090_e42613, (-((var_perfc * var_xerfc_dn6) / (assign33090_e42612 * assign33090_e42612))), (-((var_perfc * var_xerfc_dn7) / (assign33090_e42612 * assign33090_e42612))), (-((var_perfc * var_xerfc_dn8) / (assign33090_e42612 * assign33090_e42612))), (-((var_perfc * var_xerfc_dn9) / (assign33090_e42612 * assign33090_e42612))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign33090_e42615;
        var_terfc_dn6 = assign33090_e42615_d_n6;
        var_terfc_dn7 = assign33090_e42615_d_n7;
        var_terfc_dn8 = assign33090_e42615_d_n8;
        var_terfc_dn9 = assign33090_e42615_d_n9;

        let (assign33100_e42636, assign33100_e42636_d_n6, assign33100_e42636_d_n7, assign33100_e42636_d_n8, assign33100_e42636_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) && (var_guard671 == 0.0)) {
        let assign33100_e42632: f64 = (var_perfc * var_xerfc);
        let assign33100_e42633: f64 = (1.0 - assign33100_e42632);
        let assign33100_e42634: f64 = (1.0 / assign33100_e42633);
        (assign33100_e42634, (-((-(var_perfc * var_xerfc_dn6)) / (assign33100_e42633 * assign33100_e42633))), (-((-(var_perfc * var_xerfc_dn7)) / (assign33100_e42633 * assign33100_e42633))), (-((-(var_perfc * var_xerfc_dn8)) / (assign33100_e42633 * assign33100_e42633))), (-((-(var_perfc * var_xerfc_dn9)) / (assign33100_e42633 * assign33100_e42633))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign33100_e42636;
        var_terfc_dn6 = assign33100_e42636_d_n6;
        var_terfc_dn7 = assign33100_e42636_d_n7;
        var_terfc_dn8 = assign33100_e42636_d_n8;
        var_terfc_dn9 = assign33100_e42636_d_n9;

        let assign33110_e42638: f64 = (-var_ysq);
        let assign33110_e42640: f64 = (assign33110_e42638 + var_mtat);
        let assign33110_e42642: f64 = (-230.25850929940458);
        let assign33110_e42643: f64 = if assign33110_e42640 > assign33110_e42642 { 1.0 } else { 0.0 };
        var_guard672 = assign33110_e42643;

        let (assign33120_e42661, assign33120_e42661_d_n6, assign33120_e42661_d_n7, assign33120_e42661_d_n8, assign33120_e42661_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) && (var_guard672 != 0.0)) {
        let assign33120_e42656: f64 = (-var_ysq);
        let assign33120_e42658: f64 = (assign33120_e42656 + var_mtat);
        let assign33120_e42659: f64 = (assign33120_e42658).exp();
        (assign33120_e42659, (assign33120_e42659 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign33120_e42659 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign33120_e42659 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign33120_e42659 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33120_e42661;
        var_tmp_dn6 = assign33120_e42661_d_n6;
        var_tmp_dn7 = assign33120_e42661_d_n7;
        var_tmp_dn8 = assign33120_e42661_d_n8;
        var_tmp_dn9 = assign33120_e42661_d_n9;

        let (assign33130_e42710, assign33130_e42710_d_n6, assign33130_e42710_d_n7, assign33130_e42710_d_n8, assign33130_e42710_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) && (var_guard672 == 0.0)) {
        let assign33130_e42677: f64 = (-230.25850929940458);
        let assign33130_e42679: f64 = (-var_ysq);
        let assign33130_e42681: f64 = (assign33130_e42679 + var_mtat);
        let assign33130_e42682: f64 = (assign33130_e42677 - assign33130_e42681);
        let assign33130_e42686: f64 = (-230.25850929940458);
        let assign33130_e42688: f64 = (-var_ysq);
        let assign33130_e42690: f64 = (assign33130_e42688 + var_mtat);
        let assign33130_e42691: f64 = (assign33130_e42686 - assign33130_e42690);
        let assign33130_e42694: f64 = (-230.25850929940458);
        let assign33130_e42696: f64 = (-var_ysq);
        let assign33130_e42698: f64 = (assign33130_e42696 + var_mtat);
        let assign33130_e42699: f64 = (assign33130_e42694 - assign33130_e42698);
        let assign33130_e42701: f64 = (assign33130_e42699 * 0.3333333333333333);
        let assign33130_e42702: f64 = (1.0 + assign33130_e42701);
        let assign33130_e42703: f64 = (assign33130_e42691 * assign33130_e42702);
        let assign33130_e42704: f64 = (0.5 * assign33130_e42703);
        let assign33130_e42705: f64 = (1.0 + assign33130_e42704);
        let assign33130_e42706: f64 = (assign33130_e42682 * assign33130_e42705);
        let assign33130_e42707: f64 = (1.0 + assign33130_e42706);
        let assign33130_e42708: f64 = (1e-100 / assign33130_e42707);
        (assign33130_e42708, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign33130_e42705) + (assign33130_e42682 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign33130_e42702) + (assign33130_e42691 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign33130_e42707 * assign33130_e42707))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign33130_e42705) + (assign33130_e42682 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign33130_e42702) + (assign33130_e42691 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign33130_e42707 * assign33130_e42707))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign33130_e42705) + (assign33130_e42682 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign33130_e42702) + (assign33130_e42691 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign33130_e42707 * assign33130_e42707))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign33130_e42705) + (assign33130_e42682 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign33130_e42702) + (assign33130_e42691 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign33130_e42707 * assign33130_e42707))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33130_e42710;
        var_tmp_dn6 = assign33130_e42710_d_n6;
        var_tmp_dn7 = assign33130_e42710_d_n7;
        var_tmp_dn8 = assign33130_e42710_d_n8;
        var_tmp_dn9 = assign33130_e42710_d_n9;

        let (assign33140_e42740, assign33140_e42740_d_n6, assign33140_e42740_d_n7, assign33140_e42740_d_n8, assign33140_e42740_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) {
        let assign33140_e42722: f64 = (0.29214664 * var_terfc);
        let assign33140_e42726: f64 = (var_terfc * var_terfc);
        let assign33140_e42727: f64 = (var_berfc * assign33140_e42726);
        let assign33140_e42728: f64 = (assign33140_e42722 + assign33140_e42727);
        let assign33140_e42732: f64 = (var_terfc * var_terfc);
        let assign33140_e42734: f64 = (assign33140_e42732 * var_terfc);
        let assign33140_e42735: f64 = (var_cerfc * assign33140_e42734);
        let assign33140_e42736: f64 = (assign33140_e42728 + assign33140_e42735);
        let assign33140_e42738: f64 = (assign33140_e42736 * var_tmp);
        (assign33140_e42738, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign33140_e42732 * var_terfc_dn6)))) * var_tmp) + (assign33140_e42736 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign33140_e42732 * var_terfc_dn7)))) * var_tmp) + (assign33140_e42736 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign33140_e42732 * var_terfc_dn8)))) * var_tmp) + (assign33140_e42736 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign33140_e42732 * var_terfc_dn9)))) * var_tmp) + (assign33140_e42736 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign33140_e42740;
        var_erfcpos_dn6 = assign33140_e42740_d_n6;
        var_erfcpos_dn7 = assign33140_e42740_d_n7;
        var_erfcpos_dn8 = assign33140_e42740_d_n8;
        var_erfcpos_dn9 = assign33140_e42740_d_n9;

        let assign33150_e42743: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard673 = assign33150_e42743;

        let (assign33160_e42757, assign33160_e42757_d_n6, assign33160_e42757_d_n7, assign33160_e42757_d_n8, assign33160_e42757_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) && (var_guard673 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign33160_e42757;
        var_erfctimesexpmtat_dn6 = assign33160_e42757_d_n6;
        var_erfctimesexpmtat_dn7 = assign33160_e42757_d_n7;
        var_erfctimesexpmtat_dn8 = assign33160_e42757_d_n8;
        var_erfctimesexpmtat_dn9 = assign33160_e42757_d_n9;

        let assign33170_e42760: f64 = (-230.25850929940458);
        let assign33170_e42761: f64 = if var_mtat > assign33170_e42760 { 1.0 } else { 0.0 };
        var_guard674 = assign33170_e42761;

        let (assign33180_e42779, assign33180_e42779_d_n6, assign33180_e42779_d_n7, assign33180_e42779_d_n8, assign33180_e42779_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) && (var_guard673 == 0.0)) && (var_guard674 != 0.0)) {
        let assign33180_e42777: f64 = (var_mtat).exp();
        (assign33180_e42777, (assign33180_e42777 * var_mtat_dn6), (assign33180_e42777 * var_mtat_dn7), (assign33180_e42777 * var_mtat_dn8), (assign33180_e42777 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33180_e42779;
        var_tmp_dn6 = assign33180_e42779_d_n6;
        var_tmp_dn7 = assign33180_e42779_d_n7;
        var_tmp_dn8 = assign33180_e42779_d_n8;
        var_tmp_dn9 = assign33180_e42779_d_n9;

        let (assign33190_e42822, assign33190_e42822_d_n6, assign33190_e42822_d_n7, assign33190_e42822_d_n8, assign33190_e42822_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) && (var_guard673 == 0.0)) && (var_guard674 == 0.0)) {
        let assign33190_e42798: f64 = (-230.25850929940458);
        let assign33190_e42800: f64 = (assign33190_e42798 - var_mtat);
        let assign33190_e42804: f64 = (-230.25850929940458);
        let assign33190_e42806: f64 = (assign33190_e42804 - var_mtat);
        let assign33190_e42809: f64 = (-230.25850929940458);
        let assign33190_e42811: f64 = (assign33190_e42809 - var_mtat);
        let assign33190_e42813: f64 = (assign33190_e42811 * 0.3333333333333333);
        let assign33190_e42814: f64 = (1.0 + assign33190_e42813);
        let assign33190_e42815: f64 = (assign33190_e42806 * assign33190_e42814);
        let assign33190_e42816: f64 = (0.5 * assign33190_e42815);
        let assign33190_e42817: f64 = (1.0 + assign33190_e42816);
        let assign33190_e42818: f64 = (assign33190_e42800 * assign33190_e42817);
        let assign33190_e42819: f64 = (1.0 + assign33190_e42818);
        let assign33190_e42820: f64 = (1e-100 / assign33190_e42819);
        (assign33190_e42820, (-((1e-100 * (((-var_mtat_dn6) * assign33190_e42817) + (assign33190_e42800 * (0.5 * (((-var_mtat_dn6) * assign33190_e42814) + (assign33190_e42806 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign33190_e42819 * assign33190_e42819))), (-((1e-100 * (((-var_mtat_dn7) * assign33190_e42817) + (assign33190_e42800 * (0.5 * (((-var_mtat_dn7) * assign33190_e42814) + (assign33190_e42806 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign33190_e42819 * assign33190_e42819))), (-((1e-100 * (((-var_mtat_dn8) * assign33190_e42817) + (assign33190_e42800 * (0.5 * (((-var_mtat_dn8) * assign33190_e42814) + (assign33190_e42806 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign33190_e42819 * assign33190_e42819))), (-((1e-100 * (((-var_mtat_dn9) * assign33190_e42817) + (assign33190_e42800 * (0.5 * (((-var_mtat_dn9) * assign33190_e42814) + (assign33190_e42806 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign33190_e42819 * assign33190_e42819))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33190_e42822;
        var_tmp_dn6 = assign33190_e42822_d_n6;
        var_tmp_dn7 = assign33190_e42822_d_n7;
        var_tmp_dn8 = assign33190_e42822_d_n8;
        var_tmp_dn9 = assign33190_e42822_d_n9;

        let (assign33200_e42841, assign33200_e42841_d_n6, assign33200_e42841_d_n7, assign33200_e42841_d_n8, assign33200_e42841_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) && (var_guard673 == 0.0)) {
        let assign33200_e42837: f64 = (2.0 * var_tmp);
        let assign33200_e42839: f64 = (assign33200_e42837 - var_erfcpos);
        (assign33200_e42839, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign33200_e42841;
        var_erfctimesexpmtat_dn6 = assign33200_e42841_d_n6;
        var_erfctimesexpmtat_dn7 = assign33200_e42841_d_n7;
        var_erfctimesexpmtat_dn8 = assign33200_e42841_d_n8;
        var_erfctimesexpmtat_dn9 = assign33200_e42841_d_n9;

        let (assign33210_e42861, assign33210_e42861_d_n6, assign33210_e42861_d_n7, assign33210_e42861_d_n8, assign33210_e42861_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) {
        let assign33210_e42853: f64 = (1.772453850905516 * 0.5);
        let assign33210_e42856: f64 = (var_atatsti_d * var_erfctimesexpmtat);
        let assign33210_e42858: f64 = (assign33210_e42856 / var_ktat);
        let assign33210_e42859: f64 = (assign33210_e42853 * assign33210_e42858);
        (assign33210_e42859, (assign33210_e42853 * ((((var_atatsti_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign33210_e42856 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign33210_e42853 * ((((var_atatsti_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign33210_e42856 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign33210_e42853 * ((((var_atatsti_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign33210_e42856 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign33210_e42853 * ((((var_atatsti_d * var_erfctimesexpmtat_dn9) * var_ktat) - (assign33210_e42856 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign33210_e42861;
        var_gammamax_dn6 = assign33210_e42861_d_n6;
        var_gammamax_dn7 = assign33210_e42861_d_n7;
        var_gammamax_dn8 = assign33210_e42861_d_n8;
        var_gammamax_dn9 = assign33210_e42861_d_n9;

        let (assign33220_e42879, assign33220_e42879_d_n6, assign33220_e42879_d_n7, assign33220_e42879_d_n8, assign33220_e42879_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard669 == 0.0)) {
        let assign33220_e42874: f64 = (var_asrh * var_gammamax);
        let assign33220_e42876: f64 = (assign33220_e42874 * var_wtat);
        let assign33220_e42877: f64 = (var_ctatstid_i * assign33220_e42876);
        (assign33220_e42877, (var_ctatstid_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign33220_e42874 * var_wtat_dn6))), (var_ctatstid_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign33220_e42874 * var_wtat_dn7))), (var_ctatstid_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign33220_e42874 * var_wtat_dn8))), (var_ctatstid_i * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign33220_e42874 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign33220_e42879;
        var_itat_dn6 = assign33220_e42879_d_n6;
        var_itat_dn7 = assign33220_e42879_d_n7;
        var_itat_dn8 = assign33220_e42879_d_n8;
        var_itat_dn9 = assign33220_e42879_d_n9;

        let assign33230_e42882: f64 = if var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard675 = assign33230_e42882;

        let (assign33240_e42893, assign33240_e42893_d_n6, assign33240_e42893_d_n7, assign33240_e42893_d_n8, assign33240_e42893_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard675 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign33240_e42893;
        var_ibbt_dn6 = assign33240_e42893_d_n6;
        var_ibbt_dn7 = assign33240_e42893_d_n7;
        var_ibbt_dn8 = assign33240_e42893_d_n8;
        var_ibbt_dn9 = assign33240_e42893_d_n9;

        let assign33250_e42896: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard676 = assign33250_e42896;

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
        *var_guard669_slot = var_guard669;
        *var_guard670_slot = var_guard670;
        *var_guard671_slot = var_guard671;
        *var_guard672_slot = var_guard672;
        *var_guard673_slot = var_guard673;
        *var_guard674_slot = var_guard674;
        *var_guard675_slot = var_guard675;
        *var_guard676_slot = var_guard676;
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

    pub(super) fn stamp_transient_block_69(
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
        var_guard175: f64,
        var_guard192: f64,
        var_guard665: f64,
        var_guard675: f64,
        var_guard676: f64,
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
        var_v3: f64,
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
        var_guard677_slot: &mut f64,
        var_guard678_slot: &mut f64,
        var_guard679_slot: &mut f64,
        var_guard680_slot: &mut f64,
        var_guard681_slot: &mut f64,
        var_guard682_slot: &mut f64,
        var_guard683_slot: &mut f64,
        var_guard684_slot: &mut f64,
        var_guard685_slot: &mut f64,
        var_guard686_slot: &mut f64,
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
        let mut var_guard677: f64 = *var_guard677_slot;
        let mut var_guard678: f64 = *var_guard678_slot;
        let mut var_guard679: f64 = *var_guard679_slot;
        let mut var_guard680: f64 = *var_guard680_slot;
        let mut var_guard681: f64 = *var_guard681_slot;
        let mut var_guard682: f64 = *var_guard682_slot;
        let mut var_guard683: f64 = *var_guard683_slot;
        let mut var_guard684: f64 = *var_guard684_slot;
        let mut var_guard685: f64 = *var_guard685_slot;
        let mut var_guard686: f64 = *var_guard686_slot;
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

        let (assign33260_e42915, assign33260_e42915_d_n6, assign33260_e42915_d_n7, assign33260_e42915_d_n8, assign33260_e42915_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard675 == 0.0)) && (var_guard676 != 0.0)) {
        let assign33260_e42910: f64 = (var_vbirstid_i - var_vbbt);
        let assign33260_e42912: f64 = (assign33260_e42910 * var_vbirstiinv_d);
        let assign33260_e42913: f64 = (assign33260_e42912).sqrt();
        (assign33260_e42913, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33260_e42915;
        var_tmp_dn6 = assign33260_e42915_d_n6;
        var_tmp_dn7 = assign33260_e42915_d_n7;
        var_tmp_dn8 = assign33260_e42915_d_n8;
        var_tmp_dn9 = assign33260_e42915_d_n9;

        let (assign33270_e42936, assign33270_e42936_d_n6, assign33270_e42936_d_n7, assign33270_e42936_d_n8, assign33270_e42936_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard675 == 0.0)) && (var_guard676 == 0.0)) {
        let assign33270_e42930: f64 = (var_vbirstid_i - var_vbbt);
        let assign33270_e42932: f64 = (assign33270_e42930 * var_vbirstiinv_d);
        let assign33270_e42934: f64 = (assign33270_e42932).powf(var_pstid_i);
        (assign33270_e42934, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33270_e42936;
        var_tmp_dn6 = assign33270_e42936_d_n6;
        var_tmp_dn7 = assign33270_e42936_d_n7;
        var_tmp_dn8 = assign33270_e42936_d_n8;
        var_tmp_dn9 = assign33270_e42936_d_n9;

        let (assign33280_e42956, assign33280_e42956_d_n6, assign33280_e42956_d_n7, assign33280_e42956_d_n8, assign33280_e42956_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard675 == 0.0)) {
        let assign33280_e42949: f64 = (var_vbirstid_i - var_vbbt);
        let assign33280_e42951: f64 = (assign33280_e42949 * var_wdepnulrinvsti_d);
        let assign33280_e42953: f64 = (assign33280_e42951 / var_tmp);
        let assign33280_e42954: f64 = (var_one_over_one_minus_psti_d * assign33280_e42953);
        (assign33280_e42954, (var_one_over_one_minus_psti_d * (-((assign33280_e42951 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign33280_e42951 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign33280_e42951 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign33280_e42951 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign33280_e42956;
        var_fmaxr_dn6 = assign33280_e42956_d_n6;
        var_fmaxr_dn7 = assign33280_e42956_d_n7;
        var_fmaxr_dn8 = assign33280_e42956_d_n8;
        var_fmaxr_dn9 = assign33280_e42956_d_n9;

        let assign33290_e42958: f64 = (-var_fbbtsti_d);
        let assign33290_e42960: f64 = (assign33290_e42958 / var_fmaxr);
        let assign33290_e42961: f64 = (assign33290_e42960).abs();
        let assign33290_e42963: f64 = if assign33290_e42961 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard677 = assign33290_e42963;

        let (assign33300_e42981, assign33300_e42981_d_n6, assign33300_e42981_d_n7, assign33300_e42981_d_n8, assign33300_e42981_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard675 == 0.0)) && (var_guard677 != 0.0)) {
        let assign33300_e42976: f64 = (-var_fbbtsti_d);
        let assign33300_e42978: f64 = (assign33300_e42976 / var_fmaxr);
        let assign33300_e42979: f64 = (assign33300_e42978).exp();
        (assign33300_e42979, (assign33300_e42979 * (-((assign33300_e42976 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign33300_e42979 * (-((assign33300_e42976 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign33300_e42979 * (-((assign33300_e42976 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign33300_e42979 * (-((assign33300_e42976 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33300_e42981;
        var_tmp_dn6 = assign33300_e42981_d_n6;
        var_tmp_dn7 = assign33300_e42981_d_n7;
        var_tmp_dn8 = assign33300_e42981_d_n8;
        var_tmp_dn9 = assign33300_e42981_d_n9;

        let assign33310_e42983: f64 = (-var_fbbtsti_d);
        let assign33310_e42985: f64 = (assign33310_e42983 / var_fmaxr);
        let assign33310_e42987: f64 = if assign33310_e42985 < 0.0 { 1.0 } else { 0.0 };
        var_guard678 = assign33310_e42987;

        let (assign33320_e43038, assign33320_e43038_d_n6, assign33320_e43038_d_n7, assign33320_e43038_d_n8, assign33320_e43038_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard675 == 0.0)) && (var_guard677 == 0.0)) && (var_guard678 != 0.0)) {
        let assign33320_e43005: f64 = (-230.25850929940458);
        let assign33320_e43007: f64 = (-var_fbbtsti_d);
        let assign33320_e43009: f64 = (assign33320_e43007 / var_fmaxr);
        let assign33320_e43010: f64 = (assign33320_e43005 - assign33320_e43009);
        let assign33320_e43014: f64 = (-230.25850929940458);
        let assign33320_e43016: f64 = (-var_fbbtsti_d);
        let assign33320_e43018: f64 = (assign33320_e43016 / var_fmaxr);
        let assign33320_e43019: f64 = (assign33320_e43014 - assign33320_e43018);
        let assign33320_e43022: f64 = (-230.25850929940458);
        let assign33320_e43024: f64 = (-var_fbbtsti_d);
        let assign33320_e43026: f64 = (assign33320_e43024 / var_fmaxr);
        let assign33320_e43027: f64 = (assign33320_e43022 - assign33320_e43026);
        let assign33320_e43029: f64 = (assign33320_e43027 * 0.3333333333333333);
        let assign33320_e43030: f64 = (1.0 + assign33320_e43029);
        let assign33320_e43031: f64 = (assign33320_e43019 * assign33320_e43030);
        let assign33320_e43032: f64 = (0.5 * assign33320_e43031);
        let assign33320_e43033: f64 = (1.0 + assign33320_e43032);
        let assign33320_e43034: f64 = (assign33320_e43010 * assign33320_e43033);
        let assign33320_e43035: f64 = (1.0 + assign33320_e43034);
        let assign33320_e43036: f64 = (1e-100 / assign33320_e43035);
        (assign33320_e43036, (-((1e-100 * (((-(-((assign33320_e43007 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign33320_e43033) + (assign33320_e43010 * (0.5 * (((-(-((assign33320_e43016 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign33320_e43030) + (assign33320_e43019 * ((-(-((assign33320_e43024 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign33320_e43035 * assign33320_e43035))), (-((1e-100 * (((-(-((assign33320_e43007 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign33320_e43033) + (assign33320_e43010 * (0.5 * (((-(-((assign33320_e43016 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign33320_e43030) + (assign33320_e43019 * ((-(-((assign33320_e43024 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign33320_e43035 * assign33320_e43035))), (-((1e-100 * (((-(-((assign33320_e43007 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign33320_e43033) + (assign33320_e43010 * (0.5 * (((-(-((assign33320_e43016 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign33320_e43030) + (assign33320_e43019 * ((-(-((assign33320_e43024 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign33320_e43035 * assign33320_e43035))), (-((1e-100 * (((-(-((assign33320_e43007 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign33320_e43033) + (assign33320_e43010 * (0.5 * (((-(-((assign33320_e43016 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign33320_e43030) + (assign33320_e43019 * ((-(-((assign33320_e43024 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign33320_e43035 * assign33320_e43035))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33320_e43038;
        var_tmp_dn6 = assign33320_e43038_d_n6;
        var_tmp_dn7 = assign33320_e43038_d_n7;
        var_tmp_dn8 = assign33320_e43038_d_n8;
        var_tmp_dn9 = assign33320_e43038_d_n9;

        let (assign33330_e43087, assign33330_e43087_d_n6, assign33330_e43087_d_n7, assign33330_e43087_d_n8, assign33330_e43087_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard675 == 0.0)) && (var_guard677 == 0.0)) && (var_guard678 == 0.0)) {
        let assign33330_e43057: f64 = (-var_fbbtsti_d);
        let assign33330_e43059: f64 = (assign33330_e43057 / var_fmaxr);
        let assign33330_e43061: f64 = (assign33330_e43059 - 230.25850929940458);
        let assign33330_e43065: f64 = (-var_fbbtsti_d);
        let assign33330_e43067: f64 = (assign33330_e43065 / var_fmaxr);
        let assign33330_e43069: f64 = (assign33330_e43067 - 230.25850929940458);
        let assign33330_e43072: f64 = (-var_fbbtsti_d);
        let assign33330_e43074: f64 = (assign33330_e43072 / var_fmaxr);
        let assign33330_e43076: f64 = (assign33330_e43074 - 230.25850929940458);
        let assign33330_e43078: f64 = (assign33330_e43076 * 0.3333333333333333);
        let assign33330_e43079: f64 = (1.0 + assign33330_e43078);
        let assign33330_e43080: f64 = (assign33330_e43069 * assign33330_e43079);
        let assign33330_e43081: f64 = (0.5 * assign33330_e43080);
        let assign33330_e43082: f64 = (1.0 + assign33330_e43081);
        let assign33330_e43083: f64 = (assign33330_e43061 * assign33330_e43082);
        let assign33330_e43084: f64 = (1.0 + assign33330_e43083);
        let assign33330_e43085: f64 = (1e100 * assign33330_e43084);
        (assign33330_e43085, (1e100 * (((-((assign33330_e43057 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign33330_e43082) + (assign33330_e43061 * (0.5 * (((-((assign33330_e43065 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign33330_e43079) + (assign33330_e43069 * ((-((assign33330_e43072 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign33330_e43057 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign33330_e43082) + (assign33330_e43061 * (0.5 * (((-((assign33330_e43065 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign33330_e43079) + (assign33330_e43069 * ((-((assign33330_e43072 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign33330_e43057 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign33330_e43082) + (assign33330_e43061 * (0.5 * (((-((assign33330_e43065 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign33330_e43079) + (assign33330_e43069 * ((-((assign33330_e43072 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign33330_e43057 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign33330_e43082) + (assign33330_e43061 * (0.5 * (((-((assign33330_e43065 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign33330_e43079) + (assign33330_e43069 * ((-((assign33330_e43072 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33330_e43087;
        var_tmp_dn6 = assign33330_e43087_d_n6;
        var_tmp_dn7 = assign33330_e43087_d_n7;
        var_tmp_dn8 = assign33330_e43087_d_n8;
        var_tmp_dn9 = assign33330_e43087_d_n9;

        let (assign33340_e43107, assign33340_e43107_d_n6, assign33340_e43107_d_n7, assign33340_e43107_d_n8, assign33340_e43107_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard675 == 0.0)) {
        let assign33340_e43100: f64 = (var_v3 * var_fmaxr);
        let assign33340_e43102: f64 = (assign33340_e43100 * var_fmaxr);
        let assign33340_e43104: f64 = (assign33340_e43102 * var_tmp);
        let assign33340_e43105: f64 = (var_cbbtstid_i * assign33340_e43104);
        (assign33340_e43105, (var_cbbtstid_i * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign33340_e43100 * var_fmaxr_dn6)) * var_tmp) + (assign33340_e43102 * var_tmp_dn6))), (var_cbbtstid_i * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign33340_e43100 * var_fmaxr_dn7)) * var_tmp) + (assign33340_e43102 * var_tmp_dn7))), (var_cbbtstid_i * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign33340_e43100 * var_fmaxr_dn8)) * var_tmp) + (assign33340_e43102 * var_tmp_dn8))), (var_cbbtstid_i * (((((var_v3 * var_fmaxr_dn9) * var_fmaxr) + (assign33340_e43100 * var_fmaxr_dn9)) * var_tmp) + (assign33340_e43102 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign33340_e43107;
        var_ibbt_dn6 = assign33340_e43107_d_n6;
        var_ibbt_dn7 = assign33340_e43107_d_n7;
        var_ibbt_dn8 = assign33340_e43107_d_n8;
        var_ibbt_dn9 = assign33340_e43107_d_n9;

        let assign33350_e43110: f64 = if var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard679 = assign33350_e43110;

        let (assign33360_e43121, assign33360_e43121_d_n6, assign33360_e43121_d_n7, assign33360_e43121_d_n8, assign33360_e43121_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard679 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign33360_e43121;
        var_fbreakdown_dn6 = assign33360_e43121_d_n6;
        var_fbreakdown_dn7 = assign33360_e43121_d_n7;
        var_fbreakdown_dn8 = assign33360_e43121_d_n8;
        var_fbreakdown_dn9 = assign33360_e43121_d_n9;

        let assign33370_e43124: f64 = (-var_alphaav);
        let assign33370_e43126: f64 = (assign33370_e43124 * var_vbrstid_i);
        let assign33370_e43127: f64 = if var_vav > assign33370_e43126 { 1.0 } else { 0.0 };
        var_guard680 = assign33370_e43127;

        let assign33380_e43130: f64 = if var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        var_guard681 = assign33380_e43130;

        let (assign33390_e43160, assign33390_e43160_d_n6, assign33390_e43160_d_n7, assign33390_e43160_d_n8, assign33390_e43160_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard679 == 0.0)) && (var_guard680 != 0.0)) && (var_guard681 != 0.0)) {
        let assign33390_e43146: f64 = (var_vav * var_vbrinvsti_d);
        let assign33390_e43149: f64 = (var_vav * var_vbrinvsti_d);
        let assign33390_e43150: f64 = (assign33390_e43146 * assign33390_e43149);
        let assign33390_e43153: f64 = (var_vav * var_vbrinvsti_d);
        let assign33390_e43154: f64 = (assign33390_e43150 * assign33390_e43153);
        let assign33390_e43157: f64 = (var_vav * var_vbrinvsti_d);
        let assign33390_e43158: f64 = (assign33390_e43154 * assign33390_e43157);
        (assign33390_e43158, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33390_e43160;
        var_tmp_dn6 = assign33390_e43160_d_n6;
        var_tmp_dn7 = assign33390_e43160_d_n7;
        var_tmp_dn8 = assign33390_e43160_d_n8;
        var_tmp_dn9 = assign33390_e43160_d_n9;

        let (assign33400_e43182, assign33400_e43182_d_n6, assign33400_e43182_d_n7, assign33400_e43182_d_n8, assign33400_e43182_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard679 == 0.0)) && (var_guard680 != 0.0)) && (var_guard681 == 0.0)) {
        let assign33400_e43177: f64 = (var_vav * var_vbrinvsti_d);
        let assign33400_e43178: f64 = (assign33400_e43177).abs();
        let assign33400_e43180: f64 = (assign33400_e43178).powf(var_pbrstid_i);
        (assign33400_e43180, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33400_e43182;
        var_tmp_dn6 = assign33400_e43182_d_n6;
        var_tmp_dn7 = assign33400_e43182_d_n7;
        var_tmp_dn8 = assign33400_e43182_d_n8;
        var_tmp_dn9 = assign33400_e43182_d_n9;

        let (assign33410_e43200, assign33410_e43200_d_n6, assign33410_e43200_d_n7, assign33410_e43200_d_n8, assign33410_e43200_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard679 == 0.0)) && (var_guard680 != 0.0)) {
        let assign33410_e43197: f64 = (1.0 - var_tmp);
        let assign33410_e43198: f64 = (1.0 / assign33410_e43197);
        (assign33410_e43198, (-((-var_tmp_dn6) / (assign33410_e43197 * assign33410_e43197))), (-((-var_tmp_dn7) / (assign33410_e43197 * assign33410_e43197))), (-((-var_tmp_dn8) / (assign33410_e43197 * assign33410_e43197))), (-((-var_tmp_dn9) / (assign33410_e43197 * assign33410_e43197))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign33410_e43200;
        var_fbreakdown_dn6 = assign33410_e43200_d_n6;
        var_fbreakdown_dn7 = assign33410_e43200_d_n7;
        var_fbreakdown_dn8 = assign33410_e43200_d_n8;
        var_fbreakdown_dn9 = assign33410_e43200_d_n9;

        let (assign33420_e43223, assign33420_e43223_d_n6, assign33420_e43223_d_n7, assign33420_e43223_d_n8, assign33420_e43223_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) && (var_guard679 == 0.0)) && (var_guard680 == 0.0)) {
        let assign33420_e43217: f64 = (var_alphaav * var_vbrstid_i);
        let assign33420_e43218: f64 = (var_vav + assign33420_e43217);
        let assign33420_e43220: f64 = (assign33420_e43218 * var_slopesti_d);
        let assign33420_e43221: f64 = (var_fstopsti_d + assign33420_e43220);
        (assign33420_e43221, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign33420_e43223;
        var_fbreakdown_dn6 = assign33420_e43223_d_n6;
        var_fbreakdown_dn7 = assign33420_e43223_d_n7;
        var_fbreakdown_dn8 = assign33420_e43223_d_n8;
        var_fbreakdown_dn9 = assign33420_e43223_d_n9;

        let (assign33430_e43242, assign33430_e43242_d_n6, assign33430_e43242_d_n7, assign33430_e43242_d_n8, assign33430_e43242_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard665 == 0.0)) {
        let assign33430_e43233: f64 = (var_id__blk212 + var_isrh);
        let assign33430_e43235: f64 = (assign33430_e43233 + var_itat);
        let assign33430_e43237: f64 = (assign33430_e43235 + var_ibbt);
        let assign33430_e43238: f64 = (p.p29 * assign33430_e43237);
        let assign33430_e43240: f64 = (assign33430_e43238 * var_fbreakdown);
        (assign33430_e43240, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign33430_e43238 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign33430_e43238 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign33430_e43238 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign33430_e43238 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign33430_e43242;
        var_ijunsti_dn6 = assign33430_e43242_d_n6;
        var_ijunsti_dn7 = assign33430_e43242_d_n7;
        var_ijunsti_dn8 = assign33430_e43242_d_n8;
        var_ijunsti_dn9 = assign33430_e43242_d_n9;

        let assign33440_e43245: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard682 = assign33440_e43245;

        let (assign33450_e43253, assign33450_e43253_d_n6, assign33450_e43253_d_n7, assign33450_e43253_d_n8, assign33450_e43253_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign33450_e43253;
        var_ijungat_dn6 = assign33450_e43253_d_n6;
        var_ijungat_dn7 = assign33450_e43253_d_n7;
        var_ijungat_dn8 = assign33450_e43253_d_n8;
        var_ijungat_dn9 = assign33450_e43253_d_n9;

        let (assign33460_e43264,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) {
        let assign33460_e43262: f64 = (var_idsatgat_d * var_idmult);
        (assign33460_e43262,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign33460_e43264;

        let assign33470_e43271: f64 = if ((var_csrhgatd_i == 0.0) && (var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard683 = assign33470_e43271;

        let (assign33480_e43282, assign33480_e43282_d_n6, assign33480_e43282_d_n7, assign33480_e43282_d_n8, assign33480_e43282_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard683 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign33480_e43282;
        var_isrh_dn6 = assign33480_e43282_d_n6;
        var_isrh_dn7 = assign33480_e43282_d_n7;
        var_isrh_dn8 = assign33480_e43282_d_n8;
        var_isrh_dn9 = assign33480_e43282_d_n9;

        let (assign33490_e43296,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard683 == 0.0)) {
        let assign33490_e43294: f64 = (var_vbigat_d - var_vjsrh);
        (assign33490_e43294,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign33490_e43296;

        let (assign33500_e43315,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard683 == 0.0)) {
        let assign33500_e43310: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign33500_e43311: f64 = (1.0 - assign33500_e43310);
        let assign33500_e43312: f64 = (assign33500_e43311).sqrt();
        let assign33500_e43313: f64 = (1.0 - assign33500_e43312);
        (assign33500_e43313,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign33500_e43315;

        let assign33510_e43318: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard684 = assign33510_e43318;

        let (assign33520_e43332,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard683 == 0.0)) && (var_guard684 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign33520_e43332;

        let (assign33530_e43364,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard683 == 0.0)) && (var_guard684 == 0.0)) {
        let assign33530_e43347: f64 = (var_wsrhstep * var_wsrhstep);
        let assign33530_e43349: f64 = (var_wsrhstep).ln();
        let assign33530_e43350: f64 = (assign33530_e43347 * assign33530_e43349);
        let assign33530_e43353: f64 = (1.0 - var_wsrhstep);
        let assign33530_e43354: f64 = (assign33530_e43350 / assign33530_e43353);
        let assign33530_e43356: f64 = (assign33530_e43354 + var_wsrhstep);
        let assign33530_e43360: f64 = (2.0 * var_pgatd_i);
        let assign33530_e43361: f64 = (1.0 - assign33530_e43360);
        let assign33530_e43362: f64 = (assign33530_e43356 * assign33530_e43361);
        (assign33530_e43362,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign33530_e43364;

        let (assign33540_e43378,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard683 == 0.0)) {
        let assign33540_e43376: f64 = (var_wsrhstep + var_dwsrh);
        (assign33540_e43376,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign33540_e43378;

        let assign33550_e43381: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard685 = assign33550_e43381;

        let (assign33560_e43398, assign33560_e43398_d_n6, assign33560_e43398_d_n7, assign33560_e43398_d_n8, assign33560_e43398_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard683 == 0.0)) && (var_guard685 != 0.0)) {
        let assign33560_e43395: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign33560_e43396: f64 = (assign33560_e43395).sqrt();
        (assign33560_e43396, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33560_e43398;
        var_tmp_dn6 = assign33560_e43398_d_n6;
        var_tmp_dn7 = assign33560_e43398_d_n7;
        var_tmp_dn8 = assign33560_e43398_d_n8;
        var_tmp_dn9 = assign33560_e43398_d_n9;

        let (assign33570_e43417, assign33570_e43417_d_n6, assign33570_e43417_d_n7, assign33570_e43417_d_n8, assign33570_e43417_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard683 == 0.0)) && (var_guard685 == 0.0)) {
        let assign33570_e43413: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign33570_e43415: f64 = (assign33570_e43413).powf(var_pgatd_i);
        (assign33570_e43415, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33570_e43417;
        var_tmp_dn6 = assign33570_e43417_d_n6;
        var_tmp_dn7 = assign33570_e43417_d_n7;
        var_tmp_dn8 = assign33570_e43417_d_n8;
        var_tmp_dn9 = assign33570_e43417_d_n9;

        let (assign33580_e43431, assign33580_e43431_d_n6, assign33580_e43431_d_n7, assign33580_e43431_d_n8, assign33580_e43431_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard683 == 0.0)) {
        let assign33580_e43429: f64 = (var_wdepnulrgat_d * var_tmp);
        (assign33580_e43429, (var_wdepnulrgat_d * var_tmp_dn6), (var_wdepnulrgat_d * var_tmp_dn7), (var_wdepnulrgat_d * var_tmp_dn8), (var_wdepnulrgat_d * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign33580_e43431;
        var_wdep_dn6 = assign33580_e43431_d_n6;
        var_wdep_dn7 = assign33580_e43431_d_n7;
        var_wdep_dn8 = assign33580_e43431_d_n8;
        var_wdep_dn9 = assign33580_e43431_d_n9;

        let (assign33590_e43449, assign33590_e43449_d_n6, assign33590_e43449_d_n7, assign33590_e43449_d_n8, assign33590_e43449_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard683 == 0.0)) {
        let assign33590_e43444: f64 = (var_zinv - 1.0);
        let assign33590_e43446: f64 = (assign33590_e43444 * var_wdep);
        let assign33590_e43447: f64 = (var_ftdgat_d * assign33590_e43446);
        (assign33590_e43447, (var_ftdgat_d * (assign33590_e43444 * var_wdep_dn6)), (var_ftdgat_d * (assign33590_e43444 * var_wdep_dn7)), (var_ftdgat_d * (assign33590_e43444 * var_wdep_dn8)), (var_ftdgat_d * (assign33590_e43444 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign33590_e43449;
        var_asrh_dn6 = assign33590_e43449_d_n6;
        var_asrh_dn7 = assign33590_e43449_d_n7;
        var_asrh_dn8 = assign33590_e43449_d_n8;
        var_asrh_dn9 = assign33590_e43449_d_n9;

        let (assign33600_e43465, assign33600_e43465_d_n6, assign33600_e43465_d_n7, assign33600_e43465_d_n8, assign33600_e43465_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard683 == 0.0)) {
        let assign33600_e43462: f64 = (var_asrh * var_wsrh);
        let assign33600_e43463: f64 = (var_csrhgatd_i * assign33600_e43462);
        (assign33600_e43463, (var_csrhgatd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn8 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign33600_e43465;
        var_isrh_dn6 = assign33600_e43465_d_n6;
        var_isrh_dn7 = assign33600_e43465_d_n7;
        var_isrh_dn8 = assign33600_e43465_d_n8;
        var_isrh_dn9 = assign33600_e43465_d_n9;

        let assign33610_e43468: f64 = if var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard686 = assign33610_e43468;

        let (assign33620_e43479, assign33620_e43479_d_n6, assign33620_e43479_d_n7, assign33620_e43479_d_n8, assign33620_e43479_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign33620_e43479;
        var_itat_dn6 = assign33620_e43479_d_n6;
        var_itat_dn7 = assign33620_e43479_d_n7;
        var_itat_dn8 = assign33620_e43479_d_n8;
        var_itat_dn9 = assign33620_e43479_d_n9;

        let (assign33630_e43497, assign33630_e43497_d_n6, assign33630_e43497_d_n7, assign33630_e43497_d_n8, assign33630_e43497_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) {
        let assign33630_e43492: f64 = (var_wdep * var_one_minus_pgat_d);
        let assign33630_e43494: f64 = (assign33630_e43492 / var_vbi_minus_vjsrh);
        let assign33630_e43495: f64 = (var_btatpartgat_d * assign33630_e43494);
        (assign33630_e43495, (var_btatpartgat_d * ((var_wdep_dn6 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn7 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn8 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn9 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign33630_e43497;
        var_btat_dn6 = assign33630_e43497_d_n6;
        var_btat_dn7 = assign33630_e43497_d_n7;
        var_btat_dn8 = assign33630_e43497_d_n8;
        var_btat_dn9 = assign33630_e43497_d_n9;

        let (assign33640_e43513, assign33640_e43513_d_n6, assign33640_e43513_d_n7, assign33640_e43513_d_n8, assign33640_e43513_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) {
        let assign33640_e43509: f64 = (0.666666666666667 * var_atatgat_d);
        let assign33640_e43511: f64 = (assign33640_e43509 / var_btat);
        (assign33640_e43511, (-((assign33640_e43509 * var_btat_dn6) / (var_btat * var_btat))), (-((assign33640_e43509 * var_btat_dn7) / (var_btat * var_btat))), (-((assign33640_e43509 * var_btat_dn8) / (var_btat * var_btat))), (-((assign33640_e43509 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign33640_e43513;
        var_twoatatoverthreebtat_dn6 = assign33640_e43513_d_n6;
        var_twoatatoverthreebtat_dn7 = assign33640_e43513_d_n7;
        var_twoatatoverthreebtat_dn8 = assign33640_e43513_d_n8;
        var_twoatatoverthreebtat_dn9 = assign33640_e43513_d_n9;

        let (assign33650_e43527, assign33650_e43527_d_n6, assign33650_e43527_d_n7, assign33650_e43527_d_n8, assign33650_e43527_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) {
        let assign33650_e43525: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign33650_e43525, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign33650_e43527;
        var_umaxbeforelimiting_dn6 = assign33650_e43527_d_n6;
        var_umaxbeforelimiting_dn7 = assign33650_e43527_d_n7;
        var_umaxbeforelimiting_dn8 = assign33650_e43527_d_n8;
        var_umaxbeforelimiting_dn9 = assign33650_e43527_d_n9;

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
        *var_guard677_slot = var_guard677;
        *var_guard678_slot = var_guard678;
        *var_guard679_slot = var_guard679;
        *var_guard680_slot = var_guard680;
        *var_guard681_slot = var_guard681;
        *var_guard682_slot = var_guard682;
        *var_guard683_slot = var_guard683;
        *var_guard684_slot = var_guard684;
        *var_guard685_slot = var_guard685;
        *var_guard686_slot = var_guard686;
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

    pub(super) fn stamp_transient_block_70(
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatgat_d: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_btat_dn9: f64,
        var_cbbtgatd_i: f64,
        var_cerfc: f64,
        var_ctatgatd_i: f64,
        var_fbbtgat_d: f64,
        var_fbbtgat_d_dn6: f64,
        var_fbbtgat_d_dn7: f64,
        var_fbbtgat_d_dn8: f64,
        var_fbbtgat_d_dn9: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard682: f64,
        var_guard686: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_perfc: f64,
        var_pgatd_i: f64,
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
        var_vbirgatd_i: f64,
        var_vbirgatinv_d: f64,
        var_wdepnulrinvgat_d: f64,
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
        var_guard687_slot: &mut f64,
        var_guard688_slot: &mut f64,
        var_guard689_slot: &mut f64,
        var_guard690_slot: &mut f64,
        var_guard691_slot: &mut f64,
        var_guard692_slot: &mut f64,
        var_guard693_slot: &mut f64,
        var_guard694_slot: &mut f64,
        var_guard695_slot: &mut f64,
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
        let mut var_guard687: f64 = *var_guard687_slot;
        let mut var_guard688: f64 = *var_guard688_slot;
        let mut var_guard689: f64 = *var_guard689_slot;
        let mut var_guard690: f64 = *var_guard690_slot;
        let mut var_guard691: f64 = *var_guard691_slot;
        let mut var_guard692: f64 = *var_guard692_slot;
        let mut var_guard693: f64 = *var_guard693_slot;
        let mut var_guard694: f64 = *var_guard694_slot;
        let mut var_guard695: f64 = *var_guard695_slot;
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

        let (assign33660_e43548, assign33660_e43548_d_n6, assign33660_e43548_d_n7, assign33660_e43548_d_n8, assign33660_e43548_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) {
        let assign33660_e43539: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign33660_e43542: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign33660_e43544: f64 = (assign33660_e43542 + 1.0);
        let assign33660_e43545: f64 = (assign33660_e43539 / assign33660_e43544);
        let assign33660_e43546: f64 = (assign33660_e43545).sqrt();
        (assign33660_e43546, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign33660_e43544) - (assign33660_e43539 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign33660_e43544 * assign33660_e43544)) / (2.0 * assign33660_e43546)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign33660_e43544) - (assign33660_e43539 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign33660_e43544 * assign33660_e43544)) / (2.0 * assign33660_e43546)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign33660_e43544) - (assign33660_e43539 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign33660_e43544 * assign33660_e43544)) / (2.0 * assign33660_e43546)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign33660_e43544) - (assign33660_e43539 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign33660_e43544 * assign33660_e43544)) / (2.0 * assign33660_e43546)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign33660_e43548;
        var_umax_dn6 = assign33660_e43548_d_n6;
        var_umax_dn7 = assign33660_e43548_d_n7;
        var_umax_dn8 = assign33660_e43548_d_n8;
        var_umax_dn9 = assign33660_e43548_d_n9;

        let (assign33670_e43561, assign33670_e43561_d_n6, assign33670_e43561_d_n7, assign33670_e43561_d_n8, assign33670_e43561_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) {
        let assign33670_e43559: f64 = (var_umax).sqrt();
        (assign33670_e43559, (var_umax_dn6 / (2.0 * assign33670_e43559)), (var_umax_dn7 / (2.0 * assign33670_e43559)), (var_umax_dn8 / (2.0 * assign33670_e43559)), (var_umax_dn9 / (2.0 * assign33670_e43559)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign33670_e43561;
        var_sqrtumax_dn6 = assign33670_e43561_d_n6;
        var_sqrtumax_dn7 = assign33670_e43561_d_n7;
        var_sqrtumax_dn8 = assign33670_e43561_d_n8;
        var_sqrtumax_dn9 = assign33670_e43561_d_n9;

        let (assign33680_e43575, assign33680_e43575_d_n6, assign33680_e43575_d_n7, assign33680_e43575_d_n8, assign33680_e43575_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) {
        let assign33680_e43573: f64 = (var_umax * var_sqrtumax);
        (assign33680_e43573, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign33680_e43575;
        var_umaxpoweronepointfive_dn6 = assign33680_e43575_d_n6;
        var_umaxpoweronepointfive_dn7 = assign33680_e43575_d_n7;
        var_umaxpoweronepointfive_dn8 = assign33680_e43575_d_n8;
        var_umaxpoweronepointfive_dn9 = assign33680_e43575_d_n9;

        let assign33690_e43577: f64 = (-var_pgatd_i);
        let assign33690_e43579: f64 = (assign33690_e43577 * var_one_over_one_minus_pgat_d);
        let assign33690_e43581: f64 = (-1.0);
        let assign33690_e43582: f64 = if assign33690_e43579 == assign33690_e43581 { 1.0 } else { 0.0 };
        var_guard687 = assign33690_e43582;

        let (assign33700_e43602, assign33700_e43602_d_n6, assign33700_e43602_d_n7, assign33700_e43602_d_n8, assign33700_e43602_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) && (var_guard687 != 0.0)) {
        let assign33700_e43598: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign33700_e43599: f64 = (1.0 + assign33700_e43598);
        let assign33700_e43600: f64 = (1.0 / assign33700_e43599);
        (assign33700_e43600, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign33700_e43599 * assign33700_e43599))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign33700_e43599 * assign33700_e43599))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign33700_e43599 * assign33700_e43599))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign33700_e43599 * assign33700_e43599))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign33700_e43602;
        var_wgamma_dn6 = assign33700_e43602_d_n6;
        var_wgamma_dn7 = assign33700_e43602_d_n7;
        var_wgamma_dn8 = assign33700_e43602_d_n8;
        var_wgamma_dn9 = assign33700_e43602_d_n9;

        let (assign33710_e43626, assign33710_e43626_d_n6, assign33710_e43626_d_n7, assign33710_e43626_d_n8, assign33710_e43626_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) && (var_guard687 == 0.0)) {
        let assign33710_e43618: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign33710_e43619: f64 = (1.0 + assign33710_e43618);
        let assign33710_e43621: f64 = (-var_pgatd_i);
        let assign33710_e43623: f64 = (assign33710_e43621 * var_one_over_one_minus_pgat_d);
        let assign33710_e43624: f64 = (assign33710_e43619).powf(assign33710_e43623);
        (assign33710_e43624, if 0.0 == 0.0 && ((assign33710_e43623) as f64).is_finite() && ((assign33710_e43623) as f64).fract() == 0.0 { if assign33710_e43623 == 0.0 { 0.0 } else { (assign33710_e43623 * ((assign33710_e43619).powf(assign33710_e43623 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign33710_e43624 * (assign33710_e43623 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign33710_e43619))) }, if 0.0 == 0.0 && ((assign33710_e43623) as f64).is_finite() && ((assign33710_e43623) as f64).fract() == 0.0 { if assign33710_e43623 == 0.0 { 0.0 } else { (assign33710_e43623 * ((assign33710_e43619).powf(assign33710_e43623 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign33710_e43624 * (assign33710_e43623 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign33710_e43619))) }, if 0.0 == 0.0 && ((assign33710_e43623) as f64).is_finite() && ((assign33710_e43623) as f64).fract() == 0.0 { if assign33710_e43623 == 0.0 { 0.0 } else { (assign33710_e43623 * ((assign33710_e43619).powf(assign33710_e43623 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign33710_e43624 * (assign33710_e43623 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign33710_e43619))) }, if 0.0 == 0.0 && ((assign33710_e43623) as f64).is_finite() && ((assign33710_e43623) as f64).fract() == 0.0 { if assign33710_e43623 == 0.0 { 0.0 } else { (assign33710_e43623 * ((assign33710_e43619).powf(assign33710_e43623 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign33710_e43624 * (assign33710_e43623 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign33710_e43619))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign33710_e43626;
        var_wgamma_dn6 = assign33710_e43626_d_n6;
        var_wgamma_dn7 = assign33710_e43626_d_n7;
        var_wgamma_dn8 = assign33710_e43626_d_n8;
        var_wgamma_dn9 = assign33710_e43626_d_n9;

        let (assign33720_e43644, assign33720_e43644_d_n6, assign33720_e43644_d_n7, assign33720_e43644_d_n8, assign33720_e43644_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) {
        let assign33720_e43638: f64 = (var_wsrh * var_wgamma);
        let assign33720_e43641: f64 = (var_wsrh + var_wgamma);
        let assign33720_e43642: f64 = (assign33720_e43638 / assign33720_e43641);
        (assign33720_e43642, ((((var_wsrh * var_wgamma_dn6) * assign33720_e43641) - (assign33720_e43638 * var_wgamma_dn6)) / (assign33720_e43641 * assign33720_e43641)), ((((var_wsrh * var_wgamma_dn7) * assign33720_e43641) - (assign33720_e43638 * var_wgamma_dn7)) / (assign33720_e43641 * assign33720_e43641)), ((((var_wsrh * var_wgamma_dn8) * assign33720_e43641) - (assign33720_e43638 * var_wgamma_dn8)) / (assign33720_e43641 * assign33720_e43641)), ((((var_wsrh * var_wgamma_dn9) * assign33720_e43641) - (assign33720_e43638 * var_wgamma_dn9)) / (assign33720_e43641 * assign33720_e43641)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign33720_e43644;
        var_wtat_dn6 = assign33720_e43644_d_n6;
        var_wtat_dn7 = assign33720_e43644_d_n7;
        var_wtat_dn8 = assign33720_e43644_d_n8;
        var_wtat_dn9 = assign33720_e43644_d_n9;

        let (assign33730_e43661, assign33730_e43661_d_n6, assign33730_e43661_d_n7, assign33730_e43661_d_n8, assign33730_e43661_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) {
        let assign33730_e43657: f64 = (var_btat / var_sqrtumax);
        let assign33730_e43658: f64 = (0.375 * assign33730_e43657);
        let assign33730_e43659: f64 = (assign33730_e43658).sqrt();
        (assign33730_e43659, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33730_e43659)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33730_e43659)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33730_e43659)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign33730_e43659)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign33730_e43661;
        var_ktat_dn6 = assign33730_e43661_d_n6;
        var_ktat_dn7 = assign33730_e43661_d_n7;
        var_ktat_dn8 = assign33730_e43661_d_n8;
        var_ktat_dn9 = assign33730_e43661_d_n9;

        let (assign33740_e43679, assign33740_e43679_d_n6, assign33740_e43679_d_n7, assign33740_e43679_d_n8, assign33740_e43679_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) {
        let assign33740_e43674: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign33740_e43675: f64 = (2.0 * assign33740_e43674);
        let assign33740_e43677: f64 = (assign33740_e43675 - var_umax);
        (assign33740_e43677, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign33740_e43679;
        var_ltat_dn6 = assign33740_e43679_d_n6;
        var_ltat_dn7 = assign33740_e43679_d_n7;
        var_ltat_dn8 = assign33740_e43679_d_n8;
        var_ltat_dn9 = assign33740_e43679_d_n9;

        let (assign33750_e43705, assign33750_e43705_d_n6, assign33750_e43705_d_n7, assign33750_e43705_d_n8, assign33750_e43705_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) {
        let assign33750_e43691: f64 = (var_atatgat_d * var_twoatatoverthreebtat);
        let assign33750_e43693: f64 = (assign33750_e43691 * var_sqrtumax);
        let assign33750_e43696: f64 = (var_atatgat_d * var_umax);
        let assign33750_e43697: f64 = (assign33750_e43693 - assign33750_e43696);
        let assign33750_e43701: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign33750_e43702: f64 = (0.5 * assign33750_e43701);
        let assign33750_e43703: f64 = (assign33750_e43697 + assign33750_e43702);
        (assign33750_e43703, (((((var_atatgat_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign33750_e43691 * var_sqrtumax_dn6)) - (var_atatgat_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign33750_e43691 * var_sqrtumax_dn7)) - (var_atatgat_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign33750_e43691 * var_sqrtumax_dn8)) - (var_atatgat_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign33750_e43691 * var_sqrtumax_dn9)) - (var_atatgat_d * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign33750_e43705;
        var_mtat_dn6 = assign33750_e43705_d_n6;
        var_mtat_dn7 = assign33750_e43705_d_n7;
        var_mtat_dn8 = assign33750_e43705_d_n8;
        var_mtat_dn9 = assign33750_e43705_d_n9;

        let (assign33760_e43721, assign33760_e43721_d_n6, assign33760_e43721_d_n7, assign33760_e43721_d_n8, assign33760_e43721_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) {
        let assign33760_e43717: f64 = (var_ltat - 1.0);
        let assign33760_e43719: f64 = (assign33760_e43717 * var_ktat);
        (assign33760_e43719, ((var_ltat_dn6 * var_ktat) + (assign33760_e43717 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign33760_e43717 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign33760_e43717 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign33760_e43717 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign33760_e43721;
        var_xerfc_dn6 = assign33760_e43721_d_n6;
        var_xerfc_dn7 = assign33760_e43721_d_n7;
        var_xerfc_dn8 = assign33760_e43721_d_n8;
        var_xerfc_dn9 = assign33760_e43721_d_n9;

        let (assign33770_e43735, assign33770_e43735_d_n6, assign33770_e43735_d_n7, assign33770_e43735_d_n8, assign33770_e43735_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) {
        let assign33770_e43733: f64 = (var_xerfc * var_xerfc);
        (assign33770_e43733, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign33770_e43735;
        var_ysq_dn6 = assign33770_e43735_d_n6;
        var_ysq_dn7 = assign33770_e43735_d_n7;
        var_ysq_dn8 = assign33770_e43735_d_n8;
        var_ysq_dn9 = assign33770_e43735_d_n9;

        let assign33780_e43738: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard688 = assign33780_e43738;

        let (assign33790_e43758, assign33790_e43758_d_n6, assign33790_e43758_d_n7, assign33790_e43758_d_n8, assign33790_e43758_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) && (var_guard688 != 0.0)) {
        let assign33790_e43754: f64 = (var_perfc * var_xerfc);
        let assign33790_e43755: f64 = (1.0 + assign33790_e43754);
        let assign33790_e43756: f64 = (1.0 / assign33790_e43755);
        (assign33790_e43756, (-((var_perfc * var_xerfc_dn6) / (assign33790_e43755 * assign33790_e43755))), (-((var_perfc * var_xerfc_dn7) / (assign33790_e43755 * assign33790_e43755))), (-((var_perfc * var_xerfc_dn8) / (assign33790_e43755 * assign33790_e43755))), (-((var_perfc * var_xerfc_dn9) / (assign33790_e43755 * assign33790_e43755))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign33790_e43758;
        var_terfc_dn6 = assign33790_e43758_d_n6;
        var_terfc_dn7 = assign33790_e43758_d_n7;
        var_terfc_dn8 = assign33790_e43758_d_n8;
        var_terfc_dn9 = assign33790_e43758_d_n9;

        let (assign33800_e43779, assign33800_e43779_d_n6, assign33800_e43779_d_n7, assign33800_e43779_d_n8, assign33800_e43779_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) && (var_guard688 == 0.0)) {
        let assign33800_e43775: f64 = (var_perfc * var_xerfc);
        let assign33800_e43776: f64 = (1.0 - assign33800_e43775);
        let assign33800_e43777: f64 = (1.0 / assign33800_e43776);
        (assign33800_e43777, (-((-(var_perfc * var_xerfc_dn6)) / (assign33800_e43776 * assign33800_e43776))), (-((-(var_perfc * var_xerfc_dn7)) / (assign33800_e43776 * assign33800_e43776))), (-((-(var_perfc * var_xerfc_dn8)) / (assign33800_e43776 * assign33800_e43776))), (-((-(var_perfc * var_xerfc_dn9)) / (assign33800_e43776 * assign33800_e43776))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign33800_e43779;
        var_terfc_dn6 = assign33800_e43779_d_n6;
        var_terfc_dn7 = assign33800_e43779_d_n7;
        var_terfc_dn8 = assign33800_e43779_d_n8;
        var_terfc_dn9 = assign33800_e43779_d_n9;

        let assign33810_e43781: f64 = (-var_ysq);
        let assign33810_e43783: f64 = (assign33810_e43781 + var_mtat);
        let assign33810_e43785: f64 = (-230.25850929940458);
        let assign33810_e43786: f64 = if assign33810_e43783 > assign33810_e43785 { 1.0 } else { 0.0 };
        var_guard689 = assign33810_e43786;

        let (assign33820_e43804, assign33820_e43804_d_n6, assign33820_e43804_d_n7, assign33820_e43804_d_n8, assign33820_e43804_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) && (var_guard689 != 0.0)) {
        let assign33820_e43799: f64 = (-var_ysq);
        let assign33820_e43801: f64 = (assign33820_e43799 + var_mtat);
        let assign33820_e43802: f64 = (assign33820_e43801).exp();
        (assign33820_e43802, (assign33820_e43802 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign33820_e43802 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign33820_e43802 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign33820_e43802 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33820_e43804;
        var_tmp_dn6 = assign33820_e43804_d_n6;
        var_tmp_dn7 = assign33820_e43804_d_n7;
        var_tmp_dn8 = assign33820_e43804_d_n8;
        var_tmp_dn9 = assign33820_e43804_d_n9;

        let (assign33830_e43853, assign33830_e43853_d_n6, assign33830_e43853_d_n7, assign33830_e43853_d_n8, assign33830_e43853_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) && (var_guard689 == 0.0)) {
        let assign33830_e43820: f64 = (-230.25850929940458);
        let assign33830_e43822: f64 = (-var_ysq);
        let assign33830_e43824: f64 = (assign33830_e43822 + var_mtat);
        let assign33830_e43825: f64 = (assign33830_e43820 - assign33830_e43824);
        let assign33830_e43829: f64 = (-230.25850929940458);
        let assign33830_e43831: f64 = (-var_ysq);
        let assign33830_e43833: f64 = (assign33830_e43831 + var_mtat);
        let assign33830_e43834: f64 = (assign33830_e43829 - assign33830_e43833);
        let assign33830_e43837: f64 = (-230.25850929940458);
        let assign33830_e43839: f64 = (-var_ysq);
        let assign33830_e43841: f64 = (assign33830_e43839 + var_mtat);
        let assign33830_e43842: f64 = (assign33830_e43837 - assign33830_e43841);
        let assign33830_e43844: f64 = (assign33830_e43842 * 0.3333333333333333);
        let assign33830_e43845: f64 = (1.0 + assign33830_e43844);
        let assign33830_e43846: f64 = (assign33830_e43834 * assign33830_e43845);
        let assign33830_e43847: f64 = (0.5 * assign33830_e43846);
        let assign33830_e43848: f64 = (1.0 + assign33830_e43847);
        let assign33830_e43849: f64 = (assign33830_e43825 * assign33830_e43848);
        let assign33830_e43850: f64 = (1.0 + assign33830_e43849);
        let assign33830_e43851: f64 = (1e-100 / assign33830_e43850);
        (assign33830_e43851, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign33830_e43848) + (assign33830_e43825 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign33830_e43845) + (assign33830_e43834 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign33830_e43850 * assign33830_e43850))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign33830_e43848) + (assign33830_e43825 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign33830_e43845) + (assign33830_e43834 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign33830_e43850 * assign33830_e43850))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign33830_e43848) + (assign33830_e43825 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign33830_e43845) + (assign33830_e43834 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign33830_e43850 * assign33830_e43850))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign33830_e43848) + (assign33830_e43825 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign33830_e43845) + (assign33830_e43834 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign33830_e43850 * assign33830_e43850))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33830_e43853;
        var_tmp_dn6 = assign33830_e43853_d_n6;
        var_tmp_dn7 = assign33830_e43853_d_n7;
        var_tmp_dn8 = assign33830_e43853_d_n8;
        var_tmp_dn9 = assign33830_e43853_d_n9;

        let (assign33840_e43883, assign33840_e43883_d_n6, assign33840_e43883_d_n7, assign33840_e43883_d_n8, assign33840_e43883_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) {
        let assign33840_e43865: f64 = (0.29214664 * var_terfc);
        let assign33840_e43869: f64 = (var_terfc * var_terfc);
        let assign33840_e43870: f64 = (var_berfc * assign33840_e43869);
        let assign33840_e43871: f64 = (assign33840_e43865 + assign33840_e43870);
        let assign33840_e43875: f64 = (var_terfc * var_terfc);
        let assign33840_e43877: f64 = (assign33840_e43875 * var_terfc);
        let assign33840_e43878: f64 = (var_cerfc * assign33840_e43877);
        let assign33840_e43879: f64 = (assign33840_e43871 + assign33840_e43878);
        let assign33840_e43881: f64 = (assign33840_e43879 * var_tmp);
        (assign33840_e43881, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign33840_e43875 * var_terfc_dn6)))) * var_tmp) + (assign33840_e43879 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign33840_e43875 * var_terfc_dn7)))) * var_tmp) + (assign33840_e43879 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign33840_e43875 * var_terfc_dn8)))) * var_tmp) + (assign33840_e43879 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign33840_e43875 * var_terfc_dn9)))) * var_tmp) + (assign33840_e43879 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign33840_e43883;
        var_erfcpos_dn6 = assign33840_e43883_d_n6;
        var_erfcpos_dn7 = assign33840_e43883_d_n7;
        var_erfcpos_dn8 = assign33840_e43883_d_n8;
        var_erfcpos_dn9 = assign33840_e43883_d_n9;

        let assign33850_e43886: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard690 = assign33850_e43886;

        let (assign33860_e43900, assign33860_e43900_d_n6, assign33860_e43900_d_n7, assign33860_e43900_d_n8, assign33860_e43900_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) && (var_guard690 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign33860_e43900;
        var_erfctimesexpmtat_dn6 = assign33860_e43900_d_n6;
        var_erfctimesexpmtat_dn7 = assign33860_e43900_d_n7;
        var_erfctimesexpmtat_dn8 = assign33860_e43900_d_n8;
        var_erfctimesexpmtat_dn9 = assign33860_e43900_d_n9;

        let assign33870_e43903: f64 = (-230.25850929940458);
        let assign33870_e43904: f64 = if var_mtat > assign33870_e43903 { 1.0 } else { 0.0 };
        var_guard691 = assign33870_e43904;

        let (assign33880_e43922, assign33880_e43922_d_n6, assign33880_e43922_d_n7, assign33880_e43922_d_n8, assign33880_e43922_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) && (var_guard690 == 0.0)) && (var_guard691 != 0.0)) {
        let assign33880_e43920: f64 = (var_mtat).exp();
        (assign33880_e43920, (assign33880_e43920 * var_mtat_dn6), (assign33880_e43920 * var_mtat_dn7), (assign33880_e43920 * var_mtat_dn8), (assign33880_e43920 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33880_e43922;
        var_tmp_dn6 = assign33880_e43922_d_n6;
        var_tmp_dn7 = assign33880_e43922_d_n7;
        var_tmp_dn8 = assign33880_e43922_d_n8;
        var_tmp_dn9 = assign33880_e43922_d_n9;

        let (assign33890_e43965, assign33890_e43965_d_n6, assign33890_e43965_d_n7, assign33890_e43965_d_n8, assign33890_e43965_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) && (var_guard690 == 0.0)) && (var_guard691 == 0.0)) {
        let assign33890_e43941: f64 = (-230.25850929940458);
        let assign33890_e43943: f64 = (assign33890_e43941 - var_mtat);
        let assign33890_e43947: f64 = (-230.25850929940458);
        let assign33890_e43949: f64 = (assign33890_e43947 - var_mtat);
        let assign33890_e43952: f64 = (-230.25850929940458);
        let assign33890_e43954: f64 = (assign33890_e43952 - var_mtat);
        let assign33890_e43956: f64 = (assign33890_e43954 * 0.3333333333333333);
        let assign33890_e43957: f64 = (1.0 + assign33890_e43956);
        let assign33890_e43958: f64 = (assign33890_e43949 * assign33890_e43957);
        let assign33890_e43959: f64 = (0.5 * assign33890_e43958);
        let assign33890_e43960: f64 = (1.0 + assign33890_e43959);
        let assign33890_e43961: f64 = (assign33890_e43943 * assign33890_e43960);
        let assign33890_e43962: f64 = (1.0 + assign33890_e43961);
        let assign33890_e43963: f64 = (1e-100 / assign33890_e43962);
        (assign33890_e43963, (-((1e-100 * (((-var_mtat_dn6) * assign33890_e43960) + (assign33890_e43943 * (0.5 * (((-var_mtat_dn6) * assign33890_e43957) + (assign33890_e43949 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign33890_e43962 * assign33890_e43962))), (-((1e-100 * (((-var_mtat_dn7) * assign33890_e43960) + (assign33890_e43943 * (0.5 * (((-var_mtat_dn7) * assign33890_e43957) + (assign33890_e43949 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign33890_e43962 * assign33890_e43962))), (-((1e-100 * (((-var_mtat_dn8) * assign33890_e43960) + (assign33890_e43943 * (0.5 * (((-var_mtat_dn8) * assign33890_e43957) + (assign33890_e43949 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign33890_e43962 * assign33890_e43962))), (-((1e-100 * (((-var_mtat_dn9) * assign33890_e43960) + (assign33890_e43943 * (0.5 * (((-var_mtat_dn9) * assign33890_e43957) + (assign33890_e43949 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign33890_e43962 * assign33890_e43962))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33890_e43965;
        var_tmp_dn6 = assign33890_e43965_d_n6;
        var_tmp_dn7 = assign33890_e43965_d_n7;
        var_tmp_dn8 = assign33890_e43965_d_n8;
        var_tmp_dn9 = assign33890_e43965_d_n9;

        let (assign33900_e43984, assign33900_e43984_d_n6, assign33900_e43984_d_n7, assign33900_e43984_d_n8, assign33900_e43984_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) && (var_guard690 == 0.0)) {
        let assign33900_e43980: f64 = (2.0 * var_tmp);
        let assign33900_e43982: f64 = (assign33900_e43980 - var_erfcpos);
        (assign33900_e43982, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign33900_e43984;
        var_erfctimesexpmtat_dn6 = assign33900_e43984_d_n6;
        var_erfctimesexpmtat_dn7 = assign33900_e43984_d_n7;
        var_erfctimesexpmtat_dn8 = assign33900_e43984_d_n8;
        var_erfctimesexpmtat_dn9 = assign33900_e43984_d_n9;

        let (assign33910_e44004, assign33910_e44004_d_n6, assign33910_e44004_d_n7, assign33910_e44004_d_n8, assign33910_e44004_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) {
        let assign33910_e43996: f64 = (1.772453850905516 * 0.5);
        let assign33910_e43999: f64 = (var_atatgat_d * var_erfctimesexpmtat);
        let assign33910_e44001: f64 = (assign33910_e43999 / var_ktat);
        let assign33910_e44002: f64 = (assign33910_e43996 * assign33910_e44001);
        (assign33910_e44002, (assign33910_e43996 * ((((var_atatgat_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign33910_e43999 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign33910_e43996 * ((((var_atatgat_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign33910_e43999 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign33910_e43996 * ((((var_atatgat_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign33910_e43999 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign33910_e43996 * ((((var_atatgat_d * var_erfctimesexpmtat_dn9) * var_ktat) - (assign33910_e43999 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign33910_e44004;
        var_gammamax_dn6 = assign33910_e44004_d_n6;
        var_gammamax_dn7 = assign33910_e44004_d_n7;
        var_gammamax_dn8 = assign33910_e44004_d_n8;
        var_gammamax_dn9 = assign33910_e44004_d_n9;

        let (assign33920_e44022, assign33920_e44022_d_n6, assign33920_e44022_d_n7, assign33920_e44022_d_n8, assign33920_e44022_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard686 == 0.0)) {
        let assign33920_e44017: f64 = (var_asrh * var_gammamax);
        let assign33920_e44019: f64 = (assign33920_e44017 * var_wtat);
        let assign33920_e44020: f64 = (var_ctatgatd_i * assign33920_e44019);
        (assign33920_e44020, (var_ctatgatd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign33920_e44017 * var_wtat_dn6))), (var_ctatgatd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign33920_e44017 * var_wtat_dn7))), (var_ctatgatd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign33920_e44017 * var_wtat_dn8))), (var_ctatgatd_i * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign33920_e44017 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign33920_e44022;
        var_itat_dn6 = assign33920_e44022_d_n6;
        var_itat_dn7 = assign33920_e44022_d_n7;
        var_itat_dn8 = assign33920_e44022_d_n8;
        var_itat_dn9 = assign33920_e44022_d_n9;

        let assign33930_e44025: f64 = if var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard692 = assign33930_e44025;

        let (assign33940_e44036, assign33940_e44036_d_n6, assign33940_e44036_d_n7, assign33940_e44036_d_n8, assign33940_e44036_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard692 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign33940_e44036;
        var_ibbt_dn6 = assign33940_e44036_d_n6;
        var_ibbt_dn7 = assign33940_e44036_d_n7;
        var_ibbt_dn8 = assign33940_e44036_d_n8;
        var_ibbt_dn9 = assign33940_e44036_d_n9;

        let assign33950_e44039: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard693 = assign33950_e44039;

        let (assign33960_e44058, assign33960_e44058_d_n6, assign33960_e44058_d_n7, assign33960_e44058_d_n8, assign33960_e44058_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard692 == 0.0)) && (var_guard693 != 0.0)) {
        let assign33960_e44053: f64 = (var_vbirgatd_i - var_vbbt);
        let assign33960_e44055: f64 = (assign33960_e44053 * var_vbirgatinv_d);
        let assign33960_e44056: f64 = (assign33960_e44055).sqrt();
        (assign33960_e44056, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33960_e44058;
        var_tmp_dn6 = assign33960_e44058_d_n6;
        var_tmp_dn7 = assign33960_e44058_d_n7;
        var_tmp_dn8 = assign33960_e44058_d_n8;
        var_tmp_dn9 = assign33960_e44058_d_n9;

        let (assign33970_e44079, assign33970_e44079_d_n6, assign33970_e44079_d_n7, assign33970_e44079_d_n8, assign33970_e44079_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard692 == 0.0)) && (var_guard693 == 0.0)) {
        let assign33970_e44073: f64 = (var_vbirgatd_i - var_vbbt);
        let assign33970_e44075: f64 = (assign33970_e44073 * var_vbirgatinv_d);
        let assign33970_e44077: f64 = (assign33970_e44075).powf(var_pgatd_i);
        (assign33970_e44077, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign33970_e44079;
        var_tmp_dn6 = assign33970_e44079_d_n6;
        var_tmp_dn7 = assign33970_e44079_d_n7;
        var_tmp_dn8 = assign33970_e44079_d_n8;
        var_tmp_dn9 = assign33970_e44079_d_n9;

        let (assign33980_e44099, assign33980_e44099_d_n6, assign33980_e44099_d_n7, assign33980_e44099_d_n8, assign33980_e44099_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard692 == 0.0)) {
        let assign33980_e44092: f64 = (var_vbirgatd_i - var_vbbt);
        let assign33980_e44094: f64 = (assign33980_e44092 * var_wdepnulrinvgat_d);
        let assign33980_e44096: f64 = (assign33980_e44094 / var_tmp);
        let assign33980_e44097: f64 = (var_one_over_one_minus_pgat_d * assign33980_e44096);
        (assign33980_e44097, (var_one_over_one_minus_pgat_d * (-((assign33980_e44094 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign33980_e44094 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign33980_e44094 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign33980_e44094 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign33980_e44099;
        var_fmaxr_dn6 = assign33980_e44099_d_n6;
        var_fmaxr_dn7 = assign33980_e44099_d_n7;
        var_fmaxr_dn8 = assign33980_e44099_d_n8;
        var_fmaxr_dn9 = assign33980_e44099_d_n9;

        let assign33990_e44101: f64 = (-var_fbbtgat_d);
        let assign33990_e44103: f64 = (assign33990_e44101 / var_fmaxr);
        let assign33990_e44104: f64 = (assign33990_e44103).abs();
        let assign33990_e44106: f64 = if assign33990_e44104 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard694 = assign33990_e44106;

        let (assign34000_e44124, assign34000_e44124_d_n6, assign34000_e44124_d_n7, assign34000_e44124_d_n8, assign34000_e44124_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard692 == 0.0)) && (var_guard694 != 0.0)) {
        let assign34000_e44119: f64 = (-var_fbbtgat_d);
        let assign34000_e44121: f64 = (assign34000_e44119 / var_fmaxr);
        let assign34000_e44122: f64 = (assign34000_e44121).exp();
        (assign34000_e44122, (assign34000_e44122 * ((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign34000_e44119 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign34000_e44122 * ((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign34000_e44119 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign34000_e44122 * ((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign34000_e44119 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))), (assign34000_e44122 * ((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign34000_e44119 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign34000_e44124;
        var_tmp_dn6 = assign34000_e44124_d_n6;
        var_tmp_dn7 = assign34000_e44124_d_n7;
        var_tmp_dn8 = assign34000_e44124_d_n8;
        var_tmp_dn9 = assign34000_e44124_d_n9;

        let assign34010_e44126: f64 = (-var_fbbtgat_d);
        let assign34010_e44128: f64 = (assign34010_e44126 / var_fmaxr);
        let assign34010_e44130: f64 = if assign34010_e44128 < 0.0 { 1.0 } else { 0.0 };
        var_guard695 = assign34010_e44130;

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
        *var_guard687_slot = var_guard687;
        *var_guard688_slot = var_guard688;
        *var_guard689_slot = var_guard689;
        *var_guard690_slot = var_guard690;
        *var_guard691_slot = var_guard691;
        *var_guard692_slot = var_guard692;
        *var_guard693_slot = var_guard693;
        *var_guard694_slot = var_guard694;
        *var_guard695_slot = var_guard695;
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

    pub(super) fn stamp_transient_block_71(
        p: &Parameters,
        var_abdrain_i: f64,
        var_alphaav: f64,
        var_cbbtgatd_i: f64,
        var_exp_vmax_over_phitd_d: f64,
        var_fbbtgat_d: f64,
        var_fbbtgat_d_dn6: f64,
        var_fbbtgat_d_dn7: f64,
        var_fbbtgat_d_dn8: f64,
        var_fbbtgat_d_dn9: f64,
        var_fmaxr: f64,
        var_fmaxr_dn6: f64,
        var_fmaxr_dn7: f64,
        var_fmaxr_dn8: f64,
        var_fmaxr_dn9: f64,
        var_fstopgat_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard682: f64,
        var_guard692: f64,
        var_guard694: f64,
        var_guard695: f64,
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
        var_lgdrain_i: f64,
        var_lsdrain_i: f64,
        var_pbrgatd_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_slopegat_d: f64,
        var_slopegat_d_dn6: f64,
        var_slopegat_d_dn7: f64,
        var_slopegat_d_dn8: f64,
        var_slopegat_d_dn9: f64,
        var_v3: f64,
        var_v4: f64,
        var_vbbtlim_d: f64,
        var_vbimin_d: f64,
        var_vbrgatd_i: f64,
        var_vbrinvgat_d: f64,
        var_vbrinvgat_d_dn6: f64,
        var_vbrinvgat_d_dn7: f64,
        var_vbrinvgat_d_dn8: f64,
        var_vbrinvgat_d_dn9: f64,
        var_vmax_d: f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_guard696_slot: &mut f64,
        var_guard697_slot: &mut f64,
        var_guard698_slot: &mut f64,
        var_guard699_slot: &mut f64,
        var_guard700_slot: &mut f64,
        var_guard701_slot: &mut f64,
        var_guard702_slot: &mut f64,
        var_guard703_slot: &mut f64,
        var_guard704_slot: &mut f64,
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
        let mut var_guard696: f64 = *var_guard696_slot;
        let mut var_guard697: f64 = *var_guard697_slot;
        let mut var_guard698: f64 = *var_guard698_slot;
        let mut var_guard699: f64 = *var_guard699_slot;
        let mut var_guard700: f64 = *var_guard700_slot;
        let mut var_guard701: f64 = *var_guard701_slot;
        let mut var_guard702: f64 = *var_guard702_slot;
        let mut var_guard703: f64 = *var_guard703_slot;
        let mut var_guard704: f64 = *var_guard704_slot;
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

        let (assign34020_e44181, assign34020_e44181_d_n6, assign34020_e44181_d_n7, assign34020_e44181_d_n8, assign34020_e44181_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard692 == 0.0)) && (var_guard694 == 0.0)) && (var_guard695 != 0.0)) {
        let assign34020_e44148: f64 = (-230.25850929940458);
        let assign34020_e44150: f64 = (-var_fbbtgat_d);
        let assign34020_e44152: f64 = (assign34020_e44150 / var_fmaxr);
        let assign34020_e44153: f64 = (assign34020_e44148 - assign34020_e44152);
        let assign34020_e44157: f64 = (-230.25850929940458);
        let assign34020_e44159: f64 = (-var_fbbtgat_d);
        let assign34020_e44161: f64 = (assign34020_e44159 / var_fmaxr);
        let assign34020_e44162: f64 = (assign34020_e44157 - assign34020_e44161);
        let assign34020_e44165: f64 = (-230.25850929940458);
        let assign34020_e44167: f64 = (-var_fbbtgat_d);
        let assign34020_e44169: f64 = (assign34020_e44167 / var_fmaxr);
        let assign34020_e44170: f64 = (assign34020_e44165 - assign34020_e44169);
        let assign34020_e44172: f64 = (assign34020_e44170 * 0.3333333333333333);
        let assign34020_e44173: f64 = (1.0 + assign34020_e44172);
        let assign34020_e44174: f64 = (assign34020_e44162 * assign34020_e44173);
        let assign34020_e44175: f64 = (0.5 * assign34020_e44174);
        let assign34020_e44176: f64 = (1.0 + assign34020_e44175);
        let assign34020_e44177: f64 = (assign34020_e44153 * assign34020_e44176);
        let assign34020_e44178: f64 = (1.0 + assign34020_e44177);
        let assign34020_e44179: f64 = (1e-100 / assign34020_e44178);
        (assign34020_e44179, (-((1e-100 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign34020_e44150 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign34020_e44176) + (assign34020_e44153 * (0.5 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign34020_e44159 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign34020_e44173) + (assign34020_e44162 * ((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign34020_e44167 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign34020_e44178 * assign34020_e44178))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign34020_e44150 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign34020_e44176) + (assign34020_e44153 * (0.5 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign34020_e44159 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign34020_e44173) + (assign34020_e44162 * ((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign34020_e44167 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign34020_e44178 * assign34020_e44178))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign34020_e44150 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign34020_e44176) + (assign34020_e44153 * (0.5 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign34020_e44159 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign34020_e44173) + (assign34020_e44162 * ((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign34020_e44167 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign34020_e44178 * assign34020_e44178))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign34020_e44150 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign34020_e44176) + (assign34020_e44153 * (0.5 * (((-((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign34020_e44159 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign34020_e44173) + (assign34020_e44162 * ((-((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign34020_e44167 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign34020_e44178 * assign34020_e44178))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign34020_e44181;
        var_tmp_dn6 = assign34020_e44181_d_n6;
        var_tmp_dn7 = assign34020_e44181_d_n7;
        var_tmp_dn8 = assign34020_e44181_d_n8;
        var_tmp_dn9 = assign34020_e44181_d_n9;

        let (assign34030_e44230, assign34030_e44230_d_n6, assign34030_e44230_d_n7, assign34030_e44230_d_n8, assign34030_e44230_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard692 == 0.0)) && (var_guard694 == 0.0)) && (var_guard695 == 0.0)) {
        let assign34030_e44200: f64 = (-var_fbbtgat_d);
        let assign34030_e44202: f64 = (assign34030_e44200 / var_fmaxr);
        let assign34030_e44204: f64 = (assign34030_e44202 - 230.25850929940458);
        let assign34030_e44208: f64 = (-var_fbbtgat_d);
        let assign34030_e44210: f64 = (assign34030_e44208 / var_fmaxr);
        let assign34030_e44212: f64 = (assign34030_e44210 - 230.25850929940458);
        let assign34030_e44215: f64 = (-var_fbbtgat_d);
        let assign34030_e44217: f64 = (assign34030_e44215 / var_fmaxr);
        let assign34030_e44219: f64 = (assign34030_e44217 - 230.25850929940458);
        let assign34030_e44221: f64 = (assign34030_e44219 * 0.3333333333333333);
        let assign34030_e44222: f64 = (1.0 + assign34030_e44221);
        let assign34030_e44223: f64 = (assign34030_e44212 * assign34030_e44222);
        let assign34030_e44224: f64 = (0.5 * assign34030_e44223);
        let assign34030_e44225: f64 = (1.0 + assign34030_e44224);
        let assign34030_e44226: f64 = (assign34030_e44204 * assign34030_e44225);
        let assign34030_e44227: f64 = (1.0 + assign34030_e44226);
        let assign34030_e44228: f64 = (1e100 * assign34030_e44227);
        (assign34030_e44228, (1e100 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign34030_e44200 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign34030_e44225) + (assign34030_e44204 * (0.5 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign34030_e44208 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign34030_e44222) + (assign34030_e44212 * (((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign34030_e44215 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign34030_e44200 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign34030_e44225) + (assign34030_e44204 * (0.5 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign34030_e44208 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign34030_e44222) + (assign34030_e44212 * (((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign34030_e44215 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign34030_e44200 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign34030_e44225) + (assign34030_e44204 * (0.5 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign34030_e44208 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign34030_e44222) + (assign34030_e44212 * (((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign34030_e44215 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign34030_e44200 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign34030_e44225) + (assign34030_e44204 * (0.5 * ((((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign34030_e44208 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign34030_e44222) + (assign34030_e44212 * (((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign34030_e44215 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign34030_e44230;
        var_tmp_dn6 = assign34030_e44230_d_n6;
        var_tmp_dn7 = assign34030_e44230_d_n7;
        var_tmp_dn8 = assign34030_e44230_d_n8;
        var_tmp_dn9 = assign34030_e44230_d_n9;

        let (assign34040_e44250, assign34040_e44250_d_n6, assign34040_e44250_d_n7, assign34040_e44250_d_n8, assign34040_e44250_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard692 == 0.0)) {
        let assign34040_e44243: f64 = (var_v3 * var_fmaxr);
        let assign34040_e44245: f64 = (assign34040_e44243 * var_fmaxr);
        let assign34040_e44247: f64 = (assign34040_e44245 * var_tmp);
        let assign34040_e44248: f64 = (var_cbbtgatd_i * assign34040_e44247);
        (assign34040_e44248, (var_cbbtgatd_i * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign34040_e44243 * var_fmaxr_dn6)) * var_tmp) + (assign34040_e44245 * var_tmp_dn6))), (var_cbbtgatd_i * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign34040_e44243 * var_fmaxr_dn7)) * var_tmp) + (assign34040_e44245 * var_tmp_dn7))), (var_cbbtgatd_i * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign34040_e44243 * var_fmaxr_dn8)) * var_tmp) + (assign34040_e44245 * var_tmp_dn8))), (var_cbbtgatd_i * (((((var_v3 * var_fmaxr_dn9) * var_fmaxr) + (assign34040_e44243 * var_fmaxr_dn9)) * var_tmp) + (assign34040_e44245 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign34040_e44250;
        var_ibbt_dn6 = assign34040_e44250_d_n6;
        var_ibbt_dn7 = assign34040_e44250_d_n7;
        var_ibbt_dn8 = assign34040_e44250_d_n8;
        var_ibbt_dn9 = assign34040_e44250_d_n9;

        let assign34050_e44253: f64 = if var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard696 = assign34050_e44253;

        let (assign34060_e44264, assign34060_e44264_d_n6, assign34060_e44264_d_n7, assign34060_e44264_d_n8, assign34060_e44264_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard696 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign34060_e44264;
        var_fbreakdown_dn6 = assign34060_e44264_d_n6;
        var_fbreakdown_dn7 = assign34060_e44264_d_n7;
        var_fbreakdown_dn8 = assign34060_e44264_d_n8;
        var_fbreakdown_dn9 = assign34060_e44264_d_n9;

        let assign34070_e44267: f64 = (-var_alphaav);
        let assign34070_e44269: f64 = (assign34070_e44267 * var_vbrgatd_i);
        let assign34070_e44270: f64 = if var_vav > assign34070_e44269 { 1.0 } else { 0.0 };
        var_guard697 = assign34070_e44270;

        let assign34080_e44273: f64 = if var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard698 = assign34080_e44273;

        let (assign34090_e44303, assign34090_e44303_d_n6, assign34090_e44303_d_n7, assign34090_e44303_d_n8, assign34090_e44303_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard696 == 0.0)) && (var_guard697 != 0.0)) && (var_guard698 != 0.0)) {
        let assign34090_e44289: f64 = (var_vav * var_vbrinvgat_d);
        let assign34090_e44292: f64 = (var_vav * var_vbrinvgat_d);
        let assign34090_e44293: f64 = (assign34090_e44289 * assign34090_e44292);
        let assign34090_e44296: f64 = (var_vav * var_vbrinvgat_d);
        let assign34090_e44297: f64 = (assign34090_e44293 * assign34090_e44296);
        let assign34090_e44300: f64 = (var_vav * var_vbrinvgat_d);
        let assign34090_e44301: f64 = (assign34090_e44297 * assign34090_e44300);
        (assign34090_e44301, (((((((var_vav * var_vbrinvgat_d_dn6) * assign34090_e44292) + (assign34090_e44289 * (var_vav * var_vbrinvgat_d_dn6))) * assign34090_e44296) + (assign34090_e44293 * (var_vav * var_vbrinvgat_d_dn6))) * assign34090_e44300) + (assign34090_e44297 * (var_vav * var_vbrinvgat_d_dn6))), (((((((var_vav * var_vbrinvgat_d_dn7) * assign34090_e44292) + (assign34090_e44289 * (var_vav * var_vbrinvgat_d_dn7))) * assign34090_e44296) + (assign34090_e44293 * (var_vav * var_vbrinvgat_d_dn7))) * assign34090_e44300) + (assign34090_e44297 * (var_vav * var_vbrinvgat_d_dn7))), (((((((var_vav * var_vbrinvgat_d_dn8) * assign34090_e44292) + (assign34090_e44289 * (var_vav * var_vbrinvgat_d_dn8))) * assign34090_e44296) + (assign34090_e44293 * (var_vav * var_vbrinvgat_d_dn8))) * assign34090_e44300) + (assign34090_e44297 * (var_vav * var_vbrinvgat_d_dn8))), (((((((var_vav * var_vbrinvgat_d_dn9) * assign34090_e44292) + (assign34090_e44289 * (var_vav * var_vbrinvgat_d_dn9))) * assign34090_e44296) + (assign34090_e44293 * (var_vav * var_vbrinvgat_d_dn9))) * assign34090_e44300) + (assign34090_e44297 * (var_vav * var_vbrinvgat_d_dn9))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign34090_e44303;
        var_tmp_dn6 = assign34090_e44303_d_n6;
        var_tmp_dn7 = assign34090_e44303_d_n7;
        var_tmp_dn8 = assign34090_e44303_d_n8;
        var_tmp_dn9 = assign34090_e44303_d_n9;

        let (assign34100_e44325, assign34100_e44325_d_n6, assign34100_e44325_d_n7, assign34100_e44325_d_n8, assign34100_e44325_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard696 == 0.0)) && (var_guard697 != 0.0)) && (var_guard698 == 0.0)) {
        let assign34100_e44320: f64 = (var_vav * var_vbrinvgat_d);
        let assign34100_e44321: f64 = (assign34100_e44320).abs();
        let assign34100_e44323: f64 = (assign34100_e44321).powf(var_pbrgatd_i);
        (assign34100_e44323, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign34100_e44321).powf(var_pbrgatd_i - 1.0) * if assign34100_e44320 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) })) } } else { (assign34100_e44323 * (var_pbrgatd_i * (if assign34100_e44320 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) } / assign34100_e44321))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign34100_e44321).powf(var_pbrgatd_i - 1.0) * if assign34100_e44320 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) })) } } else { (assign34100_e44323 * (var_pbrgatd_i * (if assign34100_e44320 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) } / assign34100_e44321))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign34100_e44321).powf(var_pbrgatd_i - 1.0) * if assign34100_e44320 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) })) } } else { (assign34100_e44323 * (var_pbrgatd_i * (if assign34100_e44320 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) } / assign34100_e44321))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign34100_e44321).powf(var_pbrgatd_i - 1.0) * if assign34100_e44320 >= 0.0 { (var_vav * var_vbrinvgat_d_dn9) } else { (-(var_vav * var_vbrinvgat_d_dn9)) })) } } else { (assign34100_e44323 * (var_pbrgatd_i * (if assign34100_e44320 >= 0.0 { (var_vav * var_vbrinvgat_d_dn9) } else { (-(var_vav * var_vbrinvgat_d_dn9)) } / assign34100_e44321))) },)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign34100_e44325;
        var_tmp_dn6 = assign34100_e44325_d_n6;
        var_tmp_dn7 = assign34100_e44325_d_n7;
        var_tmp_dn8 = assign34100_e44325_d_n8;
        var_tmp_dn9 = assign34100_e44325_d_n9;

        let (assign34110_e44343, assign34110_e44343_d_n6, assign34110_e44343_d_n7, assign34110_e44343_d_n8, assign34110_e44343_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard696 == 0.0)) && (var_guard697 != 0.0)) {
        let assign34110_e44340: f64 = (1.0 - var_tmp);
        let assign34110_e44341: f64 = (1.0 / assign34110_e44340);
        (assign34110_e44341, (-((-var_tmp_dn6) / (assign34110_e44340 * assign34110_e44340))), (-((-var_tmp_dn7) / (assign34110_e44340 * assign34110_e44340))), (-((-var_tmp_dn8) / (assign34110_e44340 * assign34110_e44340))), (-((-var_tmp_dn9) / (assign34110_e44340 * assign34110_e44340))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign34110_e44343;
        var_fbreakdown_dn6 = assign34110_e44343_d_n6;
        var_fbreakdown_dn7 = assign34110_e44343_d_n7;
        var_fbreakdown_dn8 = assign34110_e44343_d_n8;
        var_fbreakdown_dn9 = assign34110_e44343_d_n9;

        let (assign34120_e44366, assign34120_e44366_d_n6, assign34120_e44366_d_n7, assign34120_e44366_d_n8, assign34120_e44366_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) && (var_guard696 == 0.0)) && (var_guard697 == 0.0)) {
        let assign34120_e44360: f64 = (var_alphaav * var_vbrgatd_i);
        let assign34120_e44361: f64 = (var_vav + assign34120_e44360);
        let assign34120_e44363: f64 = (assign34120_e44361 * var_slopegat_d);
        let assign34120_e44364: f64 = (var_fstopgat_d + assign34120_e44363);
        (assign34120_e44364, (assign34120_e44361 * var_slopegat_d_dn6), (assign34120_e44361 * var_slopegat_d_dn7), (assign34120_e44361 * var_slopegat_d_dn8), (assign34120_e44361 * var_slopegat_d_dn9),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign34120_e44366;
        var_fbreakdown_dn6 = assign34120_e44366_d_n6;
        var_fbreakdown_dn7 = assign34120_e44366_d_n7;
        var_fbreakdown_dn8 = assign34120_e44366_d_n8;
        var_fbreakdown_dn9 = assign34120_e44366_d_n9;

        let (assign34130_e44385, assign34130_e44385_d_n6, assign34130_e44385_d_n7, assign34130_e44385_d_n8, assign34130_e44385_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard682 == 0.0)) {
        let assign34130_e44376: f64 = (var_id__blk212 + var_isrh);
        let assign34130_e44378: f64 = (assign34130_e44376 + var_itat);
        let assign34130_e44380: f64 = (assign34130_e44378 + var_ibbt);
        let assign34130_e44381: f64 = (p.p29 * assign34130_e44380);
        let assign34130_e44383: f64 = (assign34130_e44381 * var_fbreakdown);
        (assign34130_e44383, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign34130_e44381 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign34130_e44381 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign34130_e44381 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign34130_e44381 * var_fbreakdown_dn9)),)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign34130_e44385;
        var_ijungat_dn6 = assign34130_e44385_d_n6;
        var_ijungat_dn7 = assign34130_e44385_d_n7;
        var_ijungat_dn8 = assign34130_e44385_d_n8;
        var_ijungat_dn9 = assign34130_e44385_d_n9;

        let (assign34140_e44401, assign34140_e44401_d_n6, assign34140_e44401_d_n7, assign34140_e44401_d_n8, assign34140_e44401_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign34140_e44391: f64 = (var_abdrain_i * var_ijunbot);
        let assign34140_e44394: f64 = (var_lsdrain_i * var_ijunsti);
        let assign34140_e44395: f64 = (assign34140_e44391 + assign34140_e44394);
        let assign34140_e44398: f64 = (var_lgdrain_i * var_ijungat);
        let assign34140_e44399: f64 = (assign34140_e44395 + assign34140_e44398);
        (assign34140_e44399, (((var_abdrain_i * var_ijunbot_dn6) + (var_lsdrain_i * var_ijunsti_dn6)) + (var_lgdrain_i * var_ijungat_dn6)), (((var_abdrain_i * var_ijunbot_dn7) + (var_lsdrain_i * var_ijunsti_dn7)) + (var_lgdrain_i * var_ijungat_dn7)), (((var_abdrain_i * var_ijunbot_dn8) + (var_lsdrain_i * var_ijunsti_dn8)) + (var_lgdrain_i * var_ijungat_dn8)), (((var_abdrain_i * var_ijunbot_dn9) + (var_lsdrain_i * var_ijunsti_dn9)) + (var_lgdrain_i * var_ijungat_dn9)),)
    } else {
        (var_i3, var_i3_dn6, var_i3_dn7, var_i3_dn8, var_i3_dn9,)
    }
};
        var_i3 = assign34140_e44401;
        var_i3_dn6 = assign34140_e44401_d_n6;
        var_i3_dn7 = assign34140_e44401_d_n7;
        var_i3_dn8 = assign34140_e44401_d_n8;
        var_i3_dn9 = assign34140_e44401_d_n9;

        let (assign34150_e44407,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign34150_e44407;

        let (assign34160_e44413,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign34160_e44413;

        let assign34170_e44425: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard699 = assign34170_e44425;

        let assign34250_e44511: f64 = if var_v4 < var_vmax_d { 1.0 } else { 0.0 };
        var_guard700 = assign34250_e44511;

        let assign34260_e44513: f64 = (-0.5);
        let assign34260_e44516: f64 = (var_v4 * var_phitdinv);
        let assign34260_e44517: f64 = (assign34260_e44513 * assign34260_e44516);
        let assign34260_e44518: f64 = (assign34260_e44517).abs();
        let assign34260_e44520: f64 = if assign34260_e44518 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard701 = assign34260_e44520;

        let (assign34270_e44538,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard699 != 0.0)) && (var_guard700 != 0.0)) && (var_guard701 != 0.0)) {
        let assign34270_e44531: f64 = (-0.5);
        let assign34270_e44534: f64 = (var_v4 * var_phitdinv);
        let assign34270_e44535: f64 = (assign34270_e44531 * assign34270_e44534);
        let assign34270_e44536: f64 = (assign34270_e44535).exp();
        (assign34270_e44536,)
    } else {
        (var_z,)
    }
};
        var_z = assign34270_e44538;

        let assign34280_e44540: f64 = (-0.5);
        let assign34280_e44543: f64 = (var_v4 * var_phitdinv);
        let assign34280_e44544: f64 = (assign34280_e44540 * assign34280_e44543);
        let assign34280_e44546: f64 = if assign34280_e44544 < 0.0 { 1.0 } else { 0.0 };
        var_guard702 = assign34280_e44546;

        let (assign34290_e44601,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard699 != 0.0)) && (var_guard700 != 0.0)) && (var_guard701 == 0.0)) && (var_guard702 != 0.0)) {
        let assign34290_e44562: f64 = (-230.25850929940458);
        let assign34290_e44564: f64 = (-0.5);
        let assign34290_e44567: f64 = (var_v4 * var_phitdinv);
        let assign34290_e44568: f64 = (assign34290_e44564 * assign34290_e44567);
        let assign34290_e44569: f64 = (assign34290_e44562 - assign34290_e44568);
        let assign34290_e44573: f64 = (-230.25850929940458);
        let assign34290_e44575: f64 = (-0.5);
        let assign34290_e44578: f64 = (var_v4 * var_phitdinv);
        let assign34290_e44579: f64 = (assign34290_e44575 * assign34290_e44578);
        let assign34290_e44580: f64 = (assign34290_e44573 - assign34290_e44579);
        let assign34290_e44583: f64 = (-230.25850929940458);
        let assign34290_e44585: f64 = (-0.5);
        let assign34290_e44588: f64 = (var_v4 * var_phitdinv);
        let assign34290_e44589: f64 = (assign34290_e44585 * assign34290_e44588);
        let assign34290_e44590: f64 = (assign34290_e44583 - assign34290_e44589);
        let assign34290_e44592: f64 = (assign34290_e44590 * 0.3333333333333333);
        let assign34290_e44593: f64 = (1.0 + assign34290_e44592);
        let assign34290_e44594: f64 = (assign34290_e44580 * assign34290_e44593);
        let assign34290_e44595: f64 = (0.5 * assign34290_e44594);
        let assign34290_e44596: f64 = (1.0 + assign34290_e44595);
        let assign34290_e44597: f64 = (assign34290_e44569 * assign34290_e44596);
        let assign34290_e44598: f64 = (1.0 + assign34290_e44597);
        let assign34290_e44599: f64 = (1e-100 / assign34290_e44598);
        (assign34290_e44599,)
    } else {
        (var_z,)
    }
};
        var_z = assign34290_e44601;

        let (assign34300_e44654,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard699 != 0.0)) && (var_guard700 != 0.0)) && (var_guard701 == 0.0)) && (var_guard702 == 0.0)) {
        let assign34300_e44618: f64 = (-0.5);
        let assign34300_e44621: f64 = (var_v4 * var_phitdinv);
        let assign34300_e44622: f64 = (assign34300_e44618 * assign34300_e44621);
        let assign34300_e44624: f64 = (assign34300_e44622 - 230.25850929940458);
        let assign34300_e44628: f64 = (-0.5);
        let assign34300_e44631: f64 = (var_v4 * var_phitdinv);
        let assign34300_e44632: f64 = (assign34300_e44628 * assign34300_e44631);
        let assign34300_e44634: f64 = (assign34300_e44632 - 230.25850929940458);
        let assign34300_e44637: f64 = (-0.5);
        let assign34300_e44640: f64 = (var_v4 * var_phitdinv);
        let assign34300_e44641: f64 = (assign34300_e44637 * assign34300_e44640);
        let assign34300_e44643: f64 = (assign34300_e44641 - 230.25850929940458);
        let assign34300_e44645: f64 = (assign34300_e44643 * 0.3333333333333333);
        let assign34300_e44646: f64 = (1.0 + assign34300_e44645);
        let assign34300_e44647: f64 = (assign34300_e44634 * assign34300_e44646);
        let assign34300_e44648: f64 = (0.5 * assign34300_e44647);
        let assign34300_e44649: f64 = (1.0 + assign34300_e44648);
        let assign34300_e44650: f64 = (assign34300_e44624 * assign34300_e44649);
        let assign34300_e44651: f64 = (1.0 + assign34300_e44650);
        let assign34300_e44652: f64 = (1e100 * assign34300_e44651);
        (assign34300_e44652,)
    } else {
        (var_z,)
    }
};
        var_z = assign34300_e44654;

        let (assign34310_e44666,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard699 != 0.0)) && (var_guard700 != 0.0)) {
        let assign34310_e44664: f64 = (1.0 / var_z);
        (assign34310_e44664,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign34310_e44666;

        let (assign34320_e44678,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard699 != 0.0)) && (var_guard700 != 0.0)) {
        let assign34320_e44676: f64 = (var_zinv * var_zinv);
        (assign34320_e44676,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign34320_e44678;

        let (assign34330_e44697,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard699 != 0.0)) && (var_guard700 == 0.0)) {
        let assign34330_e44690: f64 = (var_v4 - var_vmax_d);
        let assign34330_e44692: f64 = (assign34330_e44690 * var_phitdinv);
        let assign34330_e44693: f64 = (1.0 + assign34330_e44692);
        let assign34330_e44695: f64 = (assign34330_e44693 * var_exp_vmax_over_phitd_d);
        (assign34330_e44695,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign34330_e44697;

        let (assign34340_e44709,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard699 != 0.0)) && (var_guard700 == 0.0)) {
        let assign34340_e44707: f64 = (var_idmult).sqrt();
        (assign34340_e44707,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign34340_e44709;

        let (assign34350_e44722,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard699 != 0.0)) && (var_guard700 == 0.0)) {
        let assign34350_e44720: f64 = (1.0 / var_zinv);
        (assign34350_e44720,)
    } else {
        (var_z,)
    }
};
        var_z = assign34350_e44722;

        let (assign34360_e44732,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard699 != 0.0)) {
        let assign34360_e44730: f64 = (var_idmult - 1.0);
        (assign34360_e44730,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign34360_e44732;

        let assign34370_e44735: f64 = if var_v4 > 0.0 { 1.0 } else { 0.0 };
        var_guard703 = assign34370_e44735;

        let (assign34380_e44761,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard699 != 0.0)) && (var_guard703 != 0.0)) {
        let assign34380_e44747: f64 = (2.0 + var_z);
        let assign34380_e44750: f64 = (var_z + 1.0);
        let assign34380_e44753: f64 = (var_z + 3.0);
        let assign34380_e44754: f64 = (assign34380_e44750 * assign34380_e44753);
        let assign34380_e44755: f64 = (assign34380_e44754).sqrt();
        let assign34380_e44756: f64 = (assign34380_e44747 + assign34380_e44755);
        let assign34380_e44757: f64 = (assign34380_e44756).ln();
        let assign34380_e44758: f64 = (var_phitd * assign34380_e44757);
        let assign34380_e44759: f64 = (2.0 * assign34380_e44758);
        (assign34380_e44759,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign34380_e44761;

        let (assign34390_e44795,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard699 != 0.0)) && (var_guard703 == 0.0)) {
        let assign34390_e44771: f64 = (-var_v4);
        let assign34390_e44776: f64 = (2.0 * var_zinv);
        let assign34390_e44778: f64 = (assign34390_e44776 + 1.0);
        let assign34390_e44781: f64 = (1.0 + var_zinv);
        let assign34390_e44785: f64 = (3.0 * var_zinv);
        let assign34390_e44786: f64 = (1.0 + assign34390_e44785);
        let assign34390_e44787: f64 = (assign34390_e44781 * assign34390_e44786);
        let assign34390_e44788: f64 = (assign34390_e44787).sqrt();
        let assign34390_e44789: f64 = (assign34390_e44778 + assign34390_e44788);
        let assign34390_e44790: f64 = (assign34390_e44789).ln();
        let assign34390_e44791: f64 = (var_phitd * assign34390_e44790);
        let assign34390_e44792: f64 = (2.0 * assign34390_e44791);
        let assign34390_e44793: f64 = (assign34390_e44771 + assign34390_e44792);
        (assign34390_e44793,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign34390_e44795;

        let (assign34400_e44805,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard699 != 0.0)) {
        let assign34400_e44803: f64 = (var_vbimin_d - var_two_psistar);
        (assign34400_e44803,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign34400_e44805;

        let (assign34410_e44832,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard699 != 0.0)) {
        let assign34410_e44814: f64 = (var_v4 + var_vjlim);
        let assign34410_e44817: f64 = (var_v4 - var_vjlim);
        let assign34410_e44820: f64 = (var_v4 - var_vjlim);
        let assign34410_e44821: f64 = (assign34410_e44817 * assign34410_e44820);
        let assign34410_e44824: f64 = (4.0 * var_phitd);
        let assign34410_e44826: f64 = (assign34410_e44824 * var_phitd);
        let assign34410_e44827: f64 = (assign34410_e44821 + assign34410_e44826);
        let assign34410_e44828: f64 = (assign34410_e44827).sqrt();
        let assign34410_e44829: f64 = (assign34410_e44814 - assign34410_e44828);
        let assign34410_e44830: f64 = (0.5 * assign34410_e44829);
        (assign34410_e44830,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign34410_e44832;

        let (assign34420_e44859,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard699 != 0.0)) {
        let assign34420_e44841: f64 = (var_v4 + var_vbbtlim_d);
        let assign34420_e44844: f64 = (var_v4 - var_vbbtlim_d);
        let assign34420_e44847: f64 = (var_v4 - var_vbbtlim_d);
        let assign34420_e44848: f64 = (assign34420_e44844 * assign34420_e44847);
        let assign34420_e44851: f64 = (4.0 * var_phitr);
        let assign34420_e44853: f64 = (assign34420_e44851 * var_phitr);
        let assign34420_e44854: f64 = (assign34420_e44848 + assign34420_e44853);
        let assign34420_e44855: f64 = (assign34420_e44854).sqrt();
        let assign34420_e44856: f64 = (assign34420_e44841 - assign34420_e44855);
        let assign34420_e44857: f64 = (0.5 * assign34420_e44856);
        (assign34420_e44857,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign34420_e44859;

        let (assign34430_e44886,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard699 != 0.0)) {
        let assign34430_e44868: f64 = var_v4;
        let assign34430_e44871: f64 = var_v4;
        let assign34430_e44874: f64 = var_v4;
        let assign34430_e44875: f64 = (assign34430_e44871 * assign34430_e44874);
        let assign34430_e44878: f64 = (4.0 * 1e-6);
        let assign34430_e44880: f64 = (assign34430_e44878 * 1e-6);
        let assign34430_e44881: f64 = (assign34430_e44875 + assign34430_e44880);
        let assign34430_e44882: f64 = (assign34430_e44881).sqrt();
        let assign34430_e44883: f64 = (assign34430_e44868 - assign34430_e44882);
        let assign34430_e44884: f64 = (0.5 * assign34430_e44883);
        (assign34430_e44884,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign34430_e44886;

        let assign34440_e44889: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard704 = assign34440_e44889;

        let (assign34450_e44897, assign34450_e44897_d_n6, assign34450_e44897_d_n7, assign34450_e44897_d_n8, assign34450_e44897_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign34450_e44897;
        var_ijunbot_dn6 = assign34450_e44897_d_n6;
        var_ijunbot_dn7 = assign34450_e44897_d_n7;
        var_ijunbot_dn8 = assign34450_e44897_d_n8;
        var_ijunbot_dn9 = assign34450_e44897_d_n9;

        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_guard696_slot = var_guard696;
        *var_guard697_slot = var_guard697;
        *var_guard698_slot = var_guard698;
        *var_guard699_slot = var_guard699;
        *var_guard700_slot = var_guard700;
        *var_guard701_slot = var_guard701;
        *var_guard702_slot = var_guard702;
        *var_guard703_slot = var_guard703;
        *var_guard704_slot = var_guard704;
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

    pub(super) fn stamp_transient_block_72(
        var_atatbot_d: f64,
        var_btatpartbot_d: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_ftdbot_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard704: f64,
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
        var_guard705_slot: &mut f64,
        var_guard706_slot: &mut f64,
        var_guard707_slot: &mut f64,
        var_guard708_slot: &mut f64,
        var_guard709_slot: &mut f64,
        var_guard710_slot: &mut f64,
        var_guard711_slot: &mut f64,
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
        let mut var_guard705: f64 = *var_guard705_slot;
        let mut var_guard706: f64 = *var_guard706_slot;
        let mut var_guard707: f64 = *var_guard707_slot;
        let mut var_guard708: f64 = *var_guard708_slot;
        let mut var_guard709: f64 = *var_guard709_slot;
        let mut var_guard710: f64 = *var_guard710_slot;
        let mut var_guard711: f64 = *var_guard711_slot;
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

        let (assign34460_e44908,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) {
        let assign34460_e44906: f64 = (var_idsatbot_d * var_idmult);
        (assign34460_e44906,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign34460_e44908;

        let assign34470_e44915: f64 = if ((var_csrhbotd_i == 0.0) && (var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard705 = assign34470_e44915;

        let (assign34480_e44926, assign34480_e44926_d_n6, assign34480_e44926_d_n7, assign34480_e44926_d_n8, assign34480_e44926_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard705 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign34480_e44926;
        var_isrh_dn6 = assign34480_e44926_d_n6;
        var_isrh_dn7 = assign34480_e44926_d_n7;
        var_isrh_dn8 = assign34480_e44926_d_n8;
        var_isrh_dn9 = assign34480_e44926_d_n9;

        let (assign34490_e44940,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard705 == 0.0)) {
        let assign34490_e44938: f64 = (var_vbibot_d - var_vjsrh);
        (assign34490_e44938,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign34490_e44940;

        let (assign34500_e44959,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard705 == 0.0)) {
        let assign34500_e44954: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign34500_e44955: f64 = (1.0 - assign34500_e44954);
        let assign34500_e44956: f64 = (assign34500_e44955).sqrt();
        let assign34500_e44957: f64 = (1.0 - assign34500_e44956);
        (assign34500_e44957,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign34500_e44959;

        let assign34510_e44962: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard706 = assign34510_e44962;

        let (assign34520_e44976,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard705 == 0.0)) && (var_guard706 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign34520_e44976;

        let (assign34530_e45008,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard705 == 0.0)) && (var_guard706 == 0.0)) {
        let assign34530_e44991: f64 = (var_wsrhstep * var_wsrhstep);
        let assign34530_e44993: f64 = (var_wsrhstep).ln();
        let assign34530_e44994: f64 = (assign34530_e44991 * assign34530_e44993);
        let assign34530_e44997: f64 = (1.0 - var_wsrhstep);
        let assign34530_e44998: f64 = (assign34530_e44994 / assign34530_e44997);
        let assign34530_e45000: f64 = (assign34530_e44998 + var_wsrhstep);
        let assign34530_e45004: f64 = (2.0 * var_pbotd_i);
        let assign34530_e45005: f64 = (1.0 - assign34530_e45004);
        let assign34530_e45006: f64 = (assign34530_e45000 * assign34530_e45005);
        (assign34530_e45006,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign34530_e45008;

        let (assign34540_e45022,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard705 == 0.0)) {
        let assign34540_e45020: f64 = (var_wsrhstep + var_dwsrh);
        (assign34540_e45020,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign34540_e45022;

        let assign34550_e45025: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard707 = assign34550_e45025;

        let (assign34560_e45042, assign34560_e45042_d_n6, assign34560_e45042_d_n7, assign34560_e45042_d_n8, assign34560_e45042_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard705 == 0.0)) && (var_guard707 != 0.0)) {
        let assign34560_e45039: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign34560_e45040: f64 = (assign34560_e45039).sqrt();
        (assign34560_e45040, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign34560_e45042;
        var_tmp_dn6 = assign34560_e45042_d_n6;
        var_tmp_dn7 = assign34560_e45042_d_n7;
        var_tmp_dn8 = assign34560_e45042_d_n8;
        var_tmp_dn9 = assign34560_e45042_d_n9;

        let (assign34570_e45061, assign34570_e45061_d_n6, assign34570_e45061_d_n7, assign34570_e45061_d_n8, assign34570_e45061_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard705 == 0.0)) && (var_guard707 == 0.0)) {
        let assign34570_e45057: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign34570_e45059: f64 = (assign34570_e45057).powf(var_pbotd_i);
        (assign34570_e45059, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign34570_e45061;
        var_tmp_dn6 = assign34570_e45061_d_n6;
        var_tmp_dn7 = assign34570_e45061_d_n7;
        var_tmp_dn8 = assign34570_e45061_d_n8;
        var_tmp_dn9 = assign34570_e45061_d_n9;

        let (assign34580_e45075, assign34580_e45075_d_n6, assign34580_e45075_d_n7, assign34580_e45075_d_n8, assign34580_e45075_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard705 == 0.0)) {
        let assign34580_e45073: f64 = (var_wdepnulrbot_d * var_tmp);
        (assign34580_e45073, (var_wdepnulrbot_d * var_tmp_dn6), (var_wdepnulrbot_d * var_tmp_dn7), (var_wdepnulrbot_d * var_tmp_dn8), (var_wdepnulrbot_d * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign34580_e45075;
        var_wdep_dn6 = assign34580_e45075_d_n6;
        var_wdep_dn7 = assign34580_e45075_d_n7;
        var_wdep_dn8 = assign34580_e45075_d_n8;
        var_wdep_dn9 = assign34580_e45075_d_n9;

        let (assign34590_e45093, assign34590_e45093_d_n6, assign34590_e45093_d_n7, assign34590_e45093_d_n8, assign34590_e45093_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard705 == 0.0)) {
        let assign34590_e45088: f64 = (var_zinv - 1.0);
        let assign34590_e45090: f64 = (assign34590_e45088 * var_wdep);
        let assign34590_e45091: f64 = (var_ftdbot_d * assign34590_e45090);
        (assign34590_e45091, (var_ftdbot_d * (assign34590_e45088 * var_wdep_dn6)), (var_ftdbot_d * (assign34590_e45088 * var_wdep_dn7)), (var_ftdbot_d * (assign34590_e45088 * var_wdep_dn8)), (var_ftdbot_d * (assign34590_e45088 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign34590_e45093;
        var_asrh_dn6 = assign34590_e45093_d_n6;
        var_asrh_dn7 = assign34590_e45093_d_n7;
        var_asrh_dn8 = assign34590_e45093_d_n8;
        var_asrh_dn9 = assign34590_e45093_d_n9;

        let (assign34600_e45109, assign34600_e45109_d_n6, assign34600_e45109_d_n7, assign34600_e45109_d_n8, assign34600_e45109_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard705 == 0.0)) {
        let assign34600_e45106: f64 = (var_asrh * var_wsrh);
        let assign34600_e45107: f64 = (var_csrhbotd_i * assign34600_e45106);
        (assign34600_e45107, (var_csrhbotd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn8 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign34600_e45109;
        var_isrh_dn6 = assign34600_e45109_d_n6;
        var_isrh_dn7 = assign34600_e45109_d_n7;
        var_isrh_dn8 = assign34600_e45109_d_n8;
        var_isrh_dn9 = assign34600_e45109_d_n9;

        let assign34610_e45112: f64 = if var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard708 = assign34610_e45112;

        let (assign34620_e45123, assign34620_e45123_d_n6, assign34620_e45123_d_n7, assign34620_e45123_d_n8, assign34620_e45123_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign34620_e45123;
        var_itat_dn6 = assign34620_e45123_d_n6;
        var_itat_dn7 = assign34620_e45123_d_n7;
        var_itat_dn8 = assign34620_e45123_d_n8;
        var_itat_dn9 = assign34620_e45123_d_n9;

        let (assign34630_e45141, assign34630_e45141_d_n6, assign34630_e45141_d_n7, assign34630_e45141_d_n8, assign34630_e45141_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) {
        let assign34630_e45136: f64 = (var_wdep * var_one_minus_pbot_d);
        let assign34630_e45138: f64 = (assign34630_e45136 / var_vbi_minus_vjsrh);
        let assign34630_e45139: f64 = (var_btatpartbot_d * assign34630_e45138);
        (assign34630_e45139, (var_btatpartbot_d * ((var_wdep_dn6 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn7 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn8 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn9 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign34630_e45141;
        var_btat_dn6 = assign34630_e45141_d_n6;
        var_btat_dn7 = assign34630_e45141_d_n7;
        var_btat_dn8 = assign34630_e45141_d_n8;
        var_btat_dn9 = assign34630_e45141_d_n9;

        let (assign34640_e45157, assign34640_e45157_d_n6, assign34640_e45157_d_n7, assign34640_e45157_d_n8, assign34640_e45157_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) {
        let assign34640_e45153: f64 = (0.666666666666667 * var_atatbot_d);
        let assign34640_e45155: f64 = (assign34640_e45153 / var_btat);
        (assign34640_e45155, (-((assign34640_e45153 * var_btat_dn6) / (var_btat * var_btat))), (-((assign34640_e45153 * var_btat_dn7) / (var_btat * var_btat))), (-((assign34640_e45153 * var_btat_dn8) / (var_btat * var_btat))), (-((assign34640_e45153 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign34640_e45157;
        var_twoatatoverthreebtat_dn6 = assign34640_e45157_d_n6;
        var_twoatatoverthreebtat_dn7 = assign34640_e45157_d_n7;
        var_twoatatoverthreebtat_dn8 = assign34640_e45157_d_n8;
        var_twoatatoverthreebtat_dn9 = assign34640_e45157_d_n9;

        let (assign34650_e45171, assign34650_e45171_d_n6, assign34650_e45171_d_n7, assign34650_e45171_d_n8, assign34650_e45171_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) {
        let assign34650_e45169: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign34650_e45169, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign34650_e45171;
        var_umaxbeforelimiting_dn6 = assign34650_e45171_d_n6;
        var_umaxbeforelimiting_dn7 = assign34650_e45171_d_n7;
        var_umaxbeforelimiting_dn8 = assign34650_e45171_d_n8;
        var_umaxbeforelimiting_dn9 = assign34650_e45171_d_n9;

        let (assign34660_e45192, assign34660_e45192_d_n6, assign34660_e45192_d_n7, assign34660_e45192_d_n8, assign34660_e45192_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) {
        let assign34660_e45183: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign34660_e45186: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign34660_e45188: f64 = (assign34660_e45186 + 1.0);
        let assign34660_e45189: f64 = (assign34660_e45183 / assign34660_e45188);
        let assign34660_e45190: f64 = (assign34660_e45189).sqrt();
        (assign34660_e45190, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign34660_e45188) - (assign34660_e45183 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign34660_e45188 * assign34660_e45188)) / (2.0 * assign34660_e45190)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign34660_e45188) - (assign34660_e45183 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign34660_e45188 * assign34660_e45188)) / (2.0 * assign34660_e45190)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign34660_e45188) - (assign34660_e45183 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign34660_e45188 * assign34660_e45188)) / (2.0 * assign34660_e45190)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign34660_e45188) - (assign34660_e45183 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign34660_e45188 * assign34660_e45188)) / (2.0 * assign34660_e45190)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign34660_e45192;
        var_umax_dn6 = assign34660_e45192_d_n6;
        var_umax_dn7 = assign34660_e45192_d_n7;
        var_umax_dn8 = assign34660_e45192_d_n8;
        var_umax_dn9 = assign34660_e45192_d_n9;

        let (assign34670_e45205, assign34670_e45205_d_n6, assign34670_e45205_d_n7, assign34670_e45205_d_n8, assign34670_e45205_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) {
        let assign34670_e45203: f64 = (var_umax).sqrt();
        (assign34670_e45203, (var_umax_dn6 / (2.0 * assign34670_e45203)), (var_umax_dn7 / (2.0 * assign34670_e45203)), (var_umax_dn8 / (2.0 * assign34670_e45203)), (var_umax_dn9 / (2.0 * assign34670_e45203)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign34670_e45205;
        var_sqrtumax_dn6 = assign34670_e45205_d_n6;
        var_sqrtumax_dn7 = assign34670_e45205_d_n7;
        var_sqrtumax_dn8 = assign34670_e45205_d_n8;
        var_sqrtumax_dn9 = assign34670_e45205_d_n9;

        let (assign34680_e45219, assign34680_e45219_d_n6, assign34680_e45219_d_n7, assign34680_e45219_d_n8, assign34680_e45219_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) {
        let assign34680_e45217: f64 = (var_umax * var_sqrtumax);
        (assign34680_e45217, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign34680_e45219;
        var_umaxpoweronepointfive_dn6 = assign34680_e45219_d_n6;
        var_umaxpoweronepointfive_dn7 = assign34680_e45219_d_n7;
        var_umaxpoweronepointfive_dn8 = assign34680_e45219_d_n8;
        var_umaxpoweronepointfive_dn9 = assign34680_e45219_d_n9;

        let assign34690_e45221: f64 = (-var_pbotd_i);
        let assign34690_e45223: f64 = (assign34690_e45221 * var_one_over_one_minus_pbot_d);
        let assign34690_e45225: f64 = (-1.0);
        let assign34690_e45226: f64 = if assign34690_e45223 == assign34690_e45225 { 1.0 } else { 0.0 };
        var_guard709 = assign34690_e45226;

        let (assign34700_e45246, assign34700_e45246_d_n6, assign34700_e45246_d_n7, assign34700_e45246_d_n8, assign34700_e45246_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) && (var_guard709 != 0.0)) {
        let assign34700_e45242: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign34700_e45243: f64 = (1.0 + assign34700_e45242);
        let assign34700_e45244: f64 = (1.0 / assign34700_e45243);
        (assign34700_e45244, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign34700_e45243 * assign34700_e45243))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign34700_e45243 * assign34700_e45243))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign34700_e45243 * assign34700_e45243))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign34700_e45243 * assign34700_e45243))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign34700_e45246;
        var_wgamma_dn6 = assign34700_e45246_d_n6;
        var_wgamma_dn7 = assign34700_e45246_d_n7;
        var_wgamma_dn8 = assign34700_e45246_d_n8;
        var_wgamma_dn9 = assign34700_e45246_d_n9;

        let (assign34710_e45270, assign34710_e45270_d_n6, assign34710_e45270_d_n7, assign34710_e45270_d_n8, assign34710_e45270_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) && (var_guard709 == 0.0)) {
        let assign34710_e45262: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign34710_e45263: f64 = (1.0 + assign34710_e45262);
        let assign34710_e45265: f64 = (-var_pbotd_i);
        let assign34710_e45267: f64 = (assign34710_e45265 * var_one_over_one_minus_pbot_d);
        let assign34710_e45268: f64 = (assign34710_e45263).powf(assign34710_e45267);
        (assign34710_e45268, if 0.0 == 0.0 && ((assign34710_e45267) as f64).is_finite() && ((assign34710_e45267) as f64).fract() == 0.0 { if assign34710_e45267 == 0.0 { 0.0 } else { (assign34710_e45267 * ((assign34710_e45263).powf(assign34710_e45267 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign34710_e45268 * (assign34710_e45267 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign34710_e45263))) }, if 0.0 == 0.0 && ((assign34710_e45267) as f64).is_finite() && ((assign34710_e45267) as f64).fract() == 0.0 { if assign34710_e45267 == 0.0 { 0.0 } else { (assign34710_e45267 * ((assign34710_e45263).powf(assign34710_e45267 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign34710_e45268 * (assign34710_e45267 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign34710_e45263))) }, if 0.0 == 0.0 && ((assign34710_e45267) as f64).is_finite() && ((assign34710_e45267) as f64).fract() == 0.0 { if assign34710_e45267 == 0.0 { 0.0 } else { (assign34710_e45267 * ((assign34710_e45263).powf(assign34710_e45267 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign34710_e45268 * (assign34710_e45267 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign34710_e45263))) }, if 0.0 == 0.0 && ((assign34710_e45267) as f64).is_finite() && ((assign34710_e45267) as f64).fract() == 0.0 { if assign34710_e45267 == 0.0 { 0.0 } else { (assign34710_e45267 * ((assign34710_e45263).powf(assign34710_e45267 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign34710_e45268 * (assign34710_e45267 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign34710_e45263))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign34710_e45270;
        var_wgamma_dn6 = assign34710_e45270_d_n6;
        var_wgamma_dn7 = assign34710_e45270_d_n7;
        var_wgamma_dn8 = assign34710_e45270_d_n8;
        var_wgamma_dn9 = assign34710_e45270_d_n9;

        let (assign34720_e45288, assign34720_e45288_d_n6, assign34720_e45288_d_n7, assign34720_e45288_d_n8, assign34720_e45288_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) {
        let assign34720_e45282: f64 = (var_wsrh * var_wgamma);
        let assign34720_e45285: f64 = (var_wsrh + var_wgamma);
        let assign34720_e45286: f64 = (assign34720_e45282 / assign34720_e45285);
        (assign34720_e45286, ((((var_wsrh * var_wgamma_dn6) * assign34720_e45285) - (assign34720_e45282 * var_wgamma_dn6)) / (assign34720_e45285 * assign34720_e45285)), ((((var_wsrh * var_wgamma_dn7) * assign34720_e45285) - (assign34720_e45282 * var_wgamma_dn7)) / (assign34720_e45285 * assign34720_e45285)), ((((var_wsrh * var_wgamma_dn8) * assign34720_e45285) - (assign34720_e45282 * var_wgamma_dn8)) / (assign34720_e45285 * assign34720_e45285)), ((((var_wsrh * var_wgamma_dn9) * assign34720_e45285) - (assign34720_e45282 * var_wgamma_dn9)) / (assign34720_e45285 * assign34720_e45285)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign34720_e45288;
        var_wtat_dn6 = assign34720_e45288_d_n6;
        var_wtat_dn7 = assign34720_e45288_d_n7;
        var_wtat_dn8 = assign34720_e45288_d_n8;
        var_wtat_dn9 = assign34720_e45288_d_n9;

        let (assign34730_e45305, assign34730_e45305_d_n6, assign34730_e45305_d_n7, assign34730_e45305_d_n8, assign34730_e45305_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) {
        let assign34730_e45301: f64 = (var_btat / var_sqrtumax);
        let assign34730_e45302: f64 = (0.375 * assign34730_e45301);
        let assign34730_e45303: f64 = (assign34730_e45302).sqrt();
        (assign34730_e45303, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign34730_e45303)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign34730_e45303)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign34730_e45303)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign34730_e45303)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign34730_e45305;
        var_ktat_dn6 = assign34730_e45305_d_n6;
        var_ktat_dn7 = assign34730_e45305_d_n7;
        var_ktat_dn8 = assign34730_e45305_d_n8;
        var_ktat_dn9 = assign34730_e45305_d_n9;

        let (assign34740_e45323, assign34740_e45323_d_n6, assign34740_e45323_d_n7, assign34740_e45323_d_n8, assign34740_e45323_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) {
        let assign34740_e45318: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign34740_e45319: f64 = (2.0 * assign34740_e45318);
        let assign34740_e45321: f64 = (assign34740_e45319 - var_umax);
        (assign34740_e45321, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign34740_e45323;
        var_ltat_dn6 = assign34740_e45323_d_n6;
        var_ltat_dn7 = assign34740_e45323_d_n7;
        var_ltat_dn8 = assign34740_e45323_d_n8;
        var_ltat_dn9 = assign34740_e45323_d_n9;

        let (assign34750_e45349, assign34750_e45349_d_n6, assign34750_e45349_d_n7, assign34750_e45349_d_n8, assign34750_e45349_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) {
        let assign34750_e45335: f64 = (var_atatbot_d * var_twoatatoverthreebtat);
        let assign34750_e45337: f64 = (assign34750_e45335 * var_sqrtumax);
        let assign34750_e45340: f64 = (var_atatbot_d * var_umax);
        let assign34750_e45341: f64 = (assign34750_e45337 - assign34750_e45340);
        let assign34750_e45345: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign34750_e45346: f64 = (0.5 * assign34750_e45345);
        let assign34750_e45347: f64 = (assign34750_e45341 + assign34750_e45346);
        (assign34750_e45347, (((((var_atatbot_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign34750_e45335 * var_sqrtumax_dn6)) - (var_atatbot_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign34750_e45335 * var_sqrtumax_dn7)) - (var_atatbot_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign34750_e45335 * var_sqrtumax_dn8)) - (var_atatbot_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign34750_e45335 * var_sqrtumax_dn9)) - (var_atatbot_d * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign34750_e45349;
        var_mtat_dn6 = assign34750_e45349_d_n6;
        var_mtat_dn7 = assign34750_e45349_d_n7;
        var_mtat_dn8 = assign34750_e45349_d_n8;
        var_mtat_dn9 = assign34750_e45349_d_n9;

        let (assign34760_e45365, assign34760_e45365_d_n6, assign34760_e45365_d_n7, assign34760_e45365_d_n8, assign34760_e45365_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) {
        let assign34760_e45361: f64 = (var_ltat - 1.0);
        let assign34760_e45363: f64 = (assign34760_e45361 * var_ktat);
        (assign34760_e45363, ((var_ltat_dn6 * var_ktat) + (assign34760_e45361 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign34760_e45361 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign34760_e45361 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign34760_e45361 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign34760_e45365;
        var_xerfc_dn6 = assign34760_e45365_d_n6;
        var_xerfc_dn7 = assign34760_e45365_d_n7;
        var_xerfc_dn8 = assign34760_e45365_d_n8;
        var_xerfc_dn9 = assign34760_e45365_d_n9;

        let (assign34770_e45379, assign34770_e45379_d_n6, assign34770_e45379_d_n7, assign34770_e45379_d_n8, assign34770_e45379_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) {
        let assign34770_e45377: f64 = (var_xerfc * var_xerfc);
        (assign34770_e45377, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign34770_e45379;
        var_ysq_dn6 = assign34770_e45379_d_n6;
        var_ysq_dn7 = assign34770_e45379_d_n7;
        var_ysq_dn8 = assign34770_e45379_d_n8;
        var_ysq_dn9 = assign34770_e45379_d_n9;

        let assign34780_e45382: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard710 = assign34780_e45382;

        let (assign34790_e45402, assign34790_e45402_d_n6, assign34790_e45402_d_n7, assign34790_e45402_d_n8, assign34790_e45402_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) && (var_guard710 != 0.0)) {
        let assign34790_e45398: f64 = (var_perfc * var_xerfc);
        let assign34790_e45399: f64 = (1.0 + assign34790_e45398);
        let assign34790_e45400: f64 = (1.0 / assign34790_e45399);
        (assign34790_e45400, (-((var_perfc * var_xerfc_dn6) / (assign34790_e45399 * assign34790_e45399))), (-((var_perfc * var_xerfc_dn7) / (assign34790_e45399 * assign34790_e45399))), (-((var_perfc * var_xerfc_dn8) / (assign34790_e45399 * assign34790_e45399))), (-((var_perfc * var_xerfc_dn9) / (assign34790_e45399 * assign34790_e45399))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign34790_e45402;
        var_terfc_dn6 = assign34790_e45402_d_n6;
        var_terfc_dn7 = assign34790_e45402_d_n7;
        var_terfc_dn8 = assign34790_e45402_d_n8;
        var_terfc_dn9 = assign34790_e45402_d_n9;

        let (assign34800_e45423, assign34800_e45423_d_n6, assign34800_e45423_d_n7, assign34800_e45423_d_n8, assign34800_e45423_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) && (var_guard710 == 0.0)) {
        let assign34800_e45419: f64 = (var_perfc * var_xerfc);
        let assign34800_e45420: f64 = (1.0 - assign34800_e45419);
        let assign34800_e45421: f64 = (1.0 / assign34800_e45420);
        (assign34800_e45421, (-((-(var_perfc * var_xerfc_dn6)) / (assign34800_e45420 * assign34800_e45420))), (-((-(var_perfc * var_xerfc_dn7)) / (assign34800_e45420 * assign34800_e45420))), (-((-(var_perfc * var_xerfc_dn8)) / (assign34800_e45420 * assign34800_e45420))), (-((-(var_perfc * var_xerfc_dn9)) / (assign34800_e45420 * assign34800_e45420))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign34800_e45423;
        var_terfc_dn6 = assign34800_e45423_d_n6;
        var_terfc_dn7 = assign34800_e45423_d_n7;
        var_terfc_dn8 = assign34800_e45423_d_n8;
        var_terfc_dn9 = assign34800_e45423_d_n9;

        let assign34810_e45425: f64 = (-var_ysq);
        let assign34810_e45427: f64 = (assign34810_e45425 + var_mtat);
        let assign34810_e45429: f64 = (-230.25850929940458);
        let assign34810_e45430: f64 = if assign34810_e45427 > assign34810_e45429 { 1.0 } else { 0.0 };
        var_guard711 = assign34810_e45430;

        let (assign34820_e45448, assign34820_e45448_d_n6, assign34820_e45448_d_n7, assign34820_e45448_d_n8, assign34820_e45448_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) && (var_guard711 != 0.0)) {
        let assign34820_e45443: f64 = (-var_ysq);
        let assign34820_e45445: f64 = (assign34820_e45443 + var_mtat);
        let assign34820_e45446: f64 = (assign34820_e45445).exp();
        (assign34820_e45446, (assign34820_e45446 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign34820_e45446 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign34820_e45446 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign34820_e45446 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign34820_e45448;
        var_tmp_dn6 = assign34820_e45448_d_n6;
        var_tmp_dn7 = assign34820_e45448_d_n7;
        var_tmp_dn8 = assign34820_e45448_d_n8;
        var_tmp_dn9 = assign34820_e45448_d_n9;

        let (assign34830_e45497, assign34830_e45497_d_n6, assign34830_e45497_d_n7, assign34830_e45497_d_n8, assign34830_e45497_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) && (var_guard711 == 0.0)) {
        let assign34830_e45464: f64 = (-230.25850929940458);
        let assign34830_e45466: f64 = (-var_ysq);
        let assign34830_e45468: f64 = (assign34830_e45466 + var_mtat);
        let assign34830_e45469: f64 = (assign34830_e45464 - assign34830_e45468);
        let assign34830_e45473: f64 = (-230.25850929940458);
        let assign34830_e45475: f64 = (-var_ysq);
        let assign34830_e45477: f64 = (assign34830_e45475 + var_mtat);
        let assign34830_e45478: f64 = (assign34830_e45473 - assign34830_e45477);
        let assign34830_e45481: f64 = (-230.25850929940458);
        let assign34830_e45483: f64 = (-var_ysq);
        let assign34830_e45485: f64 = (assign34830_e45483 + var_mtat);
        let assign34830_e45486: f64 = (assign34830_e45481 - assign34830_e45485);
        let assign34830_e45488: f64 = (assign34830_e45486 * 0.3333333333333333);
        let assign34830_e45489: f64 = (1.0 + assign34830_e45488);
        let assign34830_e45490: f64 = (assign34830_e45478 * assign34830_e45489);
        let assign34830_e45491: f64 = (0.5 * assign34830_e45490);
        let assign34830_e45492: f64 = (1.0 + assign34830_e45491);
        let assign34830_e45493: f64 = (assign34830_e45469 * assign34830_e45492);
        let assign34830_e45494: f64 = (1.0 + assign34830_e45493);
        let assign34830_e45495: f64 = (1e-100 / assign34830_e45494);
        (assign34830_e45495, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign34830_e45492) + (assign34830_e45469 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign34830_e45489) + (assign34830_e45478 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign34830_e45494 * assign34830_e45494))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign34830_e45492) + (assign34830_e45469 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign34830_e45489) + (assign34830_e45478 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign34830_e45494 * assign34830_e45494))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign34830_e45492) + (assign34830_e45469 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign34830_e45489) + (assign34830_e45478 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign34830_e45494 * assign34830_e45494))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign34830_e45492) + (assign34830_e45469 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign34830_e45489) + (assign34830_e45478 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign34830_e45494 * assign34830_e45494))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign34830_e45497;
        var_tmp_dn6 = assign34830_e45497_d_n6;
        var_tmp_dn7 = assign34830_e45497_d_n7;
        var_tmp_dn8 = assign34830_e45497_d_n8;
        var_tmp_dn9 = assign34830_e45497_d_n9;

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
        *var_guard705_slot = var_guard705;
        *var_guard706_slot = var_guard706;
        *var_guard707_slot = var_guard707;
        *var_guard708_slot = var_guard708;
        *var_guard709_slot = var_guard709;
        *var_guard710_slot = var_guard710;
        *var_guard711_slot = var_guard711;
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

    pub(super) fn stamp_transient_block_73(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatbot_d: f64,
        var_berfc: f64,
        var_cbbtbotd_i: f64,
        var_cerfc: f64,
        var_csrhstid_i: f64,
        var_ctatbotd_i: f64,
        var_ctatstid_i: f64,
        var_fbbtbot_d: f64,
        var_fstopbot_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard704: f64,
        var_guard708: f64,
        var_idmult: f64,
        var_idsatsti_d: f64,
        var_ktat: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_ktat_dn9: f64,
        var_lsdrain_i: f64,
        var_mtat: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_mtat_dn9: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_pbrbotd_i: f64,
        var_pstid_i: f64,
        var_slopebot_d: f64,
        var_terfc: f64,
        var_terfc_dn6: f64,
        var_terfc_dn7: f64,
        var_terfc_dn8: f64,
        var_terfc_dn9: f64,
        var_two_psistar: f64,
        var_v4: f64,
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
        var_guard722_slot: &mut f64,
        var_guard723_slot: &mut f64,
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
        let mut var_guard722: f64 = *var_guard722_slot;
        let mut var_guard723: f64 = *var_guard723_slot;
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

        let (assign34840_e45527, assign34840_e45527_d_n6, assign34840_e45527_d_n7, assign34840_e45527_d_n8, assign34840_e45527_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) {
        let assign34840_e45509: f64 = (0.29214664 * var_terfc);
        let assign34840_e45513: f64 = (var_terfc * var_terfc);
        let assign34840_e45514: f64 = (var_berfc * assign34840_e45513);
        let assign34840_e45515: f64 = (assign34840_e45509 + assign34840_e45514);
        let assign34840_e45519: f64 = (var_terfc * var_terfc);
        let assign34840_e45521: f64 = (assign34840_e45519 * var_terfc);
        let assign34840_e45522: f64 = (var_cerfc * assign34840_e45521);
        let assign34840_e45523: f64 = (assign34840_e45515 + assign34840_e45522);
        let assign34840_e45525: f64 = (assign34840_e45523 * var_tmp);
        (assign34840_e45525, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign34840_e45519 * var_terfc_dn6)))) * var_tmp) + (assign34840_e45523 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign34840_e45519 * var_terfc_dn7)))) * var_tmp) + (assign34840_e45523 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign34840_e45519 * var_terfc_dn8)))) * var_tmp) + (assign34840_e45523 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign34840_e45519 * var_terfc_dn9)))) * var_tmp) + (assign34840_e45523 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign34840_e45527;
        var_erfcpos_dn6 = assign34840_e45527_d_n6;
        var_erfcpos_dn7 = assign34840_e45527_d_n7;
        var_erfcpos_dn8 = assign34840_e45527_d_n8;
        var_erfcpos_dn9 = assign34840_e45527_d_n9;

        let assign34850_e45530: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard712 = assign34850_e45530;

        let (assign34860_e45544, assign34860_e45544_d_n6, assign34860_e45544_d_n7, assign34860_e45544_d_n8, assign34860_e45544_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) && (var_guard712 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign34860_e45544;
        var_erfctimesexpmtat_dn6 = assign34860_e45544_d_n6;
        var_erfctimesexpmtat_dn7 = assign34860_e45544_d_n7;
        var_erfctimesexpmtat_dn8 = assign34860_e45544_d_n8;
        var_erfctimesexpmtat_dn9 = assign34860_e45544_d_n9;

        let assign34870_e45547: f64 = (-230.25850929940458);
        let assign34870_e45548: f64 = if var_mtat > assign34870_e45547 { 1.0 } else { 0.0 };
        var_guard713 = assign34870_e45548;

        let (assign34880_e45566, assign34880_e45566_d_n6, assign34880_e45566_d_n7, assign34880_e45566_d_n8, assign34880_e45566_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) && (var_guard712 == 0.0)) && (var_guard713 != 0.0)) {
        let assign34880_e45564: f64 = (var_mtat).exp();
        (assign34880_e45564, (assign34880_e45564 * var_mtat_dn6), (assign34880_e45564 * var_mtat_dn7), (assign34880_e45564 * var_mtat_dn8), (assign34880_e45564 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign34880_e45566;
        var_tmp_dn6 = assign34880_e45566_d_n6;
        var_tmp_dn7 = assign34880_e45566_d_n7;
        var_tmp_dn8 = assign34880_e45566_d_n8;
        var_tmp_dn9 = assign34880_e45566_d_n9;

        let (assign34890_e45609, assign34890_e45609_d_n6, assign34890_e45609_d_n7, assign34890_e45609_d_n8, assign34890_e45609_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) && (var_guard712 == 0.0)) && (var_guard713 == 0.0)) {
        let assign34890_e45585: f64 = (-230.25850929940458);
        let assign34890_e45587: f64 = (assign34890_e45585 - var_mtat);
        let assign34890_e45591: f64 = (-230.25850929940458);
        let assign34890_e45593: f64 = (assign34890_e45591 - var_mtat);
        let assign34890_e45596: f64 = (-230.25850929940458);
        let assign34890_e45598: f64 = (assign34890_e45596 - var_mtat);
        let assign34890_e45600: f64 = (assign34890_e45598 * 0.3333333333333333);
        let assign34890_e45601: f64 = (1.0 + assign34890_e45600);
        let assign34890_e45602: f64 = (assign34890_e45593 * assign34890_e45601);
        let assign34890_e45603: f64 = (0.5 * assign34890_e45602);
        let assign34890_e45604: f64 = (1.0 + assign34890_e45603);
        let assign34890_e45605: f64 = (assign34890_e45587 * assign34890_e45604);
        let assign34890_e45606: f64 = (1.0 + assign34890_e45605);
        let assign34890_e45607: f64 = (1e-100 / assign34890_e45606);
        (assign34890_e45607, (-((1e-100 * (((-var_mtat_dn6) * assign34890_e45604) + (assign34890_e45587 * (0.5 * (((-var_mtat_dn6) * assign34890_e45601) + (assign34890_e45593 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign34890_e45606 * assign34890_e45606))), (-((1e-100 * (((-var_mtat_dn7) * assign34890_e45604) + (assign34890_e45587 * (0.5 * (((-var_mtat_dn7) * assign34890_e45601) + (assign34890_e45593 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign34890_e45606 * assign34890_e45606))), (-((1e-100 * (((-var_mtat_dn8) * assign34890_e45604) + (assign34890_e45587 * (0.5 * (((-var_mtat_dn8) * assign34890_e45601) + (assign34890_e45593 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign34890_e45606 * assign34890_e45606))), (-((1e-100 * (((-var_mtat_dn9) * assign34890_e45604) + (assign34890_e45587 * (0.5 * (((-var_mtat_dn9) * assign34890_e45601) + (assign34890_e45593 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign34890_e45606 * assign34890_e45606))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign34890_e45609;
        var_tmp_dn6 = assign34890_e45609_d_n6;
        var_tmp_dn7 = assign34890_e45609_d_n7;
        var_tmp_dn8 = assign34890_e45609_d_n8;
        var_tmp_dn9 = assign34890_e45609_d_n9;

        let (assign34900_e45628, assign34900_e45628_d_n6, assign34900_e45628_d_n7, assign34900_e45628_d_n8, assign34900_e45628_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) && (var_guard712 == 0.0)) {
        let assign34900_e45624: f64 = (2.0 * var_tmp);
        let assign34900_e45626: f64 = (assign34900_e45624 - var_erfcpos);
        (assign34900_e45626, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign34900_e45628;
        var_erfctimesexpmtat_dn6 = assign34900_e45628_d_n6;
        var_erfctimesexpmtat_dn7 = assign34900_e45628_d_n7;
        var_erfctimesexpmtat_dn8 = assign34900_e45628_d_n8;
        var_erfctimesexpmtat_dn9 = assign34900_e45628_d_n9;

        let (assign34910_e45648, assign34910_e45648_d_n6, assign34910_e45648_d_n7, assign34910_e45648_d_n8, assign34910_e45648_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) {
        let assign34910_e45640: f64 = (1.772453850905516 * 0.5);
        let assign34910_e45643: f64 = (var_atatbot_d * var_erfctimesexpmtat);
        let assign34910_e45645: f64 = (assign34910_e45643 / var_ktat);
        let assign34910_e45646: f64 = (assign34910_e45640 * assign34910_e45645);
        (assign34910_e45646, (assign34910_e45640 * ((((var_atatbot_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign34910_e45643 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign34910_e45640 * ((((var_atatbot_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign34910_e45643 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign34910_e45640 * ((((var_atatbot_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign34910_e45643 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign34910_e45640 * ((((var_atatbot_d * var_erfctimesexpmtat_dn9) * var_ktat) - (assign34910_e45643 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign34910_e45648;
        var_gammamax_dn6 = assign34910_e45648_d_n6;
        var_gammamax_dn7 = assign34910_e45648_d_n7;
        var_gammamax_dn8 = assign34910_e45648_d_n8;
        var_gammamax_dn9 = assign34910_e45648_d_n9;

        let (assign34920_e45666, assign34920_e45666_d_n6, assign34920_e45666_d_n7, assign34920_e45666_d_n8, assign34920_e45666_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard708 == 0.0)) {
        let assign34920_e45661: f64 = (var_asrh * var_gammamax);
        let assign34920_e45663: f64 = (assign34920_e45661 * var_wtat);
        let assign34920_e45664: f64 = (var_ctatbotd_i * assign34920_e45663);
        (assign34920_e45664, (var_ctatbotd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign34920_e45661 * var_wtat_dn6))), (var_ctatbotd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign34920_e45661 * var_wtat_dn7))), (var_ctatbotd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign34920_e45661 * var_wtat_dn8))), (var_ctatbotd_i * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign34920_e45661 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign34920_e45666;
        var_itat_dn6 = assign34920_e45666_d_n6;
        var_itat_dn7 = assign34920_e45666_d_n7;
        var_itat_dn8 = assign34920_e45666_d_n8;
        var_itat_dn9 = assign34920_e45666_d_n9;

        let assign34930_e45669: f64 = if var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard714 = assign34930_e45669;

        let (assign34940_e45680, assign34940_e45680_d_n6, assign34940_e45680_d_n7, assign34940_e45680_d_n8, assign34940_e45680_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard714 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign34940_e45680;
        var_ibbt_dn6 = assign34940_e45680_d_n6;
        var_ibbt_dn7 = assign34940_e45680_d_n7;
        var_ibbt_dn8 = assign34940_e45680_d_n8;
        var_ibbt_dn9 = assign34940_e45680_d_n9;

        let assign34950_e45683: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard715 = assign34950_e45683;

        let (assign34960_e45702, assign34960_e45702_d_n6, assign34960_e45702_d_n7, assign34960_e45702_d_n8, assign34960_e45702_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard714 == 0.0)) && (var_guard715 != 0.0)) {
        let assign34960_e45697: f64 = (var_vbirbotd_i - var_vbbt);
        let assign34960_e45699: f64 = (assign34960_e45697 * var_vbirbotinv_d);
        let assign34960_e45700: f64 = (assign34960_e45699).sqrt();
        (assign34960_e45700, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign34960_e45702;
        var_tmp_dn6 = assign34960_e45702_d_n6;
        var_tmp_dn7 = assign34960_e45702_d_n7;
        var_tmp_dn8 = assign34960_e45702_d_n8;
        var_tmp_dn9 = assign34960_e45702_d_n9;

        let (assign34970_e45723, assign34970_e45723_d_n6, assign34970_e45723_d_n7, assign34970_e45723_d_n8, assign34970_e45723_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard714 == 0.0)) && (var_guard715 == 0.0)) {
        let assign34970_e45717: f64 = (var_vbirbotd_i - var_vbbt);
        let assign34970_e45719: f64 = (assign34970_e45717 * var_vbirbotinv_d);
        let assign34970_e45721: f64 = (assign34970_e45719).powf(var_pbotd_i);
        (assign34970_e45721, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign34970_e45723;
        var_tmp_dn6 = assign34970_e45723_d_n6;
        var_tmp_dn7 = assign34970_e45723_d_n7;
        var_tmp_dn8 = assign34970_e45723_d_n8;
        var_tmp_dn9 = assign34970_e45723_d_n9;

        let (assign34980_e45743, assign34980_e45743_d_n6, assign34980_e45743_d_n7, assign34980_e45743_d_n8, assign34980_e45743_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard714 == 0.0)) {
        let assign34980_e45736: f64 = (var_vbirbotd_i - var_vbbt);
        let assign34980_e45738: f64 = (assign34980_e45736 * var_wdepnulrinvbot_d);
        let assign34980_e45740: f64 = (assign34980_e45738 / var_tmp);
        let assign34980_e45741: f64 = (var_one_over_one_minus_pbot_d * assign34980_e45740);
        (assign34980_e45741, (var_one_over_one_minus_pbot_d * (-((assign34980_e45738 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign34980_e45738 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign34980_e45738 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign34980_e45738 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign34980_e45743;
        var_fmaxr_dn6 = assign34980_e45743_d_n6;
        var_fmaxr_dn7 = assign34980_e45743_d_n7;
        var_fmaxr_dn8 = assign34980_e45743_d_n8;
        var_fmaxr_dn9 = assign34980_e45743_d_n9;

        let assign34990_e45745: f64 = (-var_fbbtbot_d);
        let assign34990_e45747: f64 = (assign34990_e45745 / var_fmaxr);
        let assign34990_e45748: f64 = (assign34990_e45747).abs();
        let assign34990_e45750: f64 = if assign34990_e45748 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard716 = assign34990_e45750;

        let (assign35000_e45768, assign35000_e45768_d_n6, assign35000_e45768_d_n7, assign35000_e45768_d_n8, assign35000_e45768_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard714 == 0.0)) && (var_guard716 != 0.0)) {
        let assign35000_e45763: f64 = (-var_fbbtbot_d);
        let assign35000_e45765: f64 = (assign35000_e45763 / var_fmaxr);
        let assign35000_e45766: f64 = (assign35000_e45765).exp();
        (assign35000_e45766, (assign35000_e45766 * (-((assign35000_e45763 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign35000_e45766 * (-((assign35000_e45763 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign35000_e45766 * (-((assign35000_e45763 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign35000_e45766 * (-((assign35000_e45763 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35000_e45768;
        var_tmp_dn6 = assign35000_e45768_d_n6;
        var_tmp_dn7 = assign35000_e45768_d_n7;
        var_tmp_dn8 = assign35000_e45768_d_n8;
        var_tmp_dn9 = assign35000_e45768_d_n9;

        let assign35010_e45770: f64 = (-var_fbbtbot_d);
        let assign35010_e45772: f64 = (assign35010_e45770 / var_fmaxr);
        let assign35010_e45774: f64 = if assign35010_e45772 < 0.0 { 1.0 } else { 0.0 };
        var_guard717 = assign35010_e45774;

        let (assign35020_e45825, assign35020_e45825_d_n6, assign35020_e45825_d_n7, assign35020_e45825_d_n8, assign35020_e45825_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard714 == 0.0)) && (var_guard716 == 0.0)) && (var_guard717 != 0.0)) {
        let assign35020_e45792: f64 = (-230.25850929940458);
        let assign35020_e45794: f64 = (-var_fbbtbot_d);
        let assign35020_e45796: f64 = (assign35020_e45794 / var_fmaxr);
        let assign35020_e45797: f64 = (assign35020_e45792 - assign35020_e45796);
        let assign35020_e45801: f64 = (-230.25850929940458);
        let assign35020_e45803: f64 = (-var_fbbtbot_d);
        let assign35020_e45805: f64 = (assign35020_e45803 / var_fmaxr);
        let assign35020_e45806: f64 = (assign35020_e45801 - assign35020_e45805);
        let assign35020_e45809: f64 = (-230.25850929940458);
        let assign35020_e45811: f64 = (-var_fbbtbot_d);
        let assign35020_e45813: f64 = (assign35020_e45811 / var_fmaxr);
        let assign35020_e45814: f64 = (assign35020_e45809 - assign35020_e45813);
        let assign35020_e45816: f64 = (assign35020_e45814 * 0.3333333333333333);
        let assign35020_e45817: f64 = (1.0 + assign35020_e45816);
        let assign35020_e45818: f64 = (assign35020_e45806 * assign35020_e45817);
        let assign35020_e45819: f64 = (0.5 * assign35020_e45818);
        let assign35020_e45820: f64 = (1.0 + assign35020_e45819);
        let assign35020_e45821: f64 = (assign35020_e45797 * assign35020_e45820);
        let assign35020_e45822: f64 = (1.0 + assign35020_e45821);
        let assign35020_e45823: f64 = (1e-100 / assign35020_e45822);
        (assign35020_e45823, (-((1e-100 * (((-(-((assign35020_e45794 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign35020_e45820) + (assign35020_e45797 * (0.5 * (((-(-((assign35020_e45803 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign35020_e45817) + (assign35020_e45806 * ((-(-((assign35020_e45811 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign35020_e45822 * assign35020_e45822))), (-((1e-100 * (((-(-((assign35020_e45794 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign35020_e45820) + (assign35020_e45797 * (0.5 * (((-(-((assign35020_e45803 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign35020_e45817) + (assign35020_e45806 * ((-(-((assign35020_e45811 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign35020_e45822 * assign35020_e45822))), (-((1e-100 * (((-(-((assign35020_e45794 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign35020_e45820) + (assign35020_e45797 * (0.5 * (((-(-((assign35020_e45803 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign35020_e45817) + (assign35020_e45806 * ((-(-((assign35020_e45811 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign35020_e45822 * assign35020_e45822))), (-((1e-100 * (((-(-((assign35020_e45794 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign35020_e45820) + (assign35020_e45797 * (0.5 * (((-(-((assign35020_e45803 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign35020_e45817) + (assign35020_e45806 * ((-(-((assign35020_e45811 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign35020_e45822 * assign35020_e45822))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35020_e45825;
        var_tmp_dn6 = assign35020_e45825_d_n6;
        var_tmp_dn7 = assign35020_e45825_d_n7;
        var_tmp_dn8 = assign35020_e45825_d_n8;
        var_tmp_dn9 = assign35020_e45825_d_n9;

        let (assign35030_e45874, assign35030_e45874_d_n6, assign35030_e45874_d_n7, assign35030_e45874_d_n8, assign35030_e45874_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard714 == 0.0)) && (var_guard716 == 0.0)) && (var_guard717 == 0.0)) {
        let assign35030_e45844: f64 = (-var_fbbtbot_d);
        let assign35030_e45846: f64 = (assign35030_e45844 / var_fmaxr);
        let assign35030_e45848: f64 = (assign35030_e45846 - 230.25850929940458);
        let assign35030_e45852: f64 = (-var_fbbtbot_d);
        let assign35030_e45854: f64 = (assign35030_e45852 / var_fmaxr);
        let assign35030_e45856: f64 = (assign35030_e45854 - 230.25850929940458);
        let assign35030_e45859: f64 = (-var_fbbtbot_d);
        let assign35030_e45861: f64 = (assign35030_e45859 / var_fmaxr);
        let assign35030_e45863: f64 = (assign35030_e45861 - 230.25850929940458);
        let assign35030_e45865: f64 = (assign35030_e45863 * 0.3333333333333333);
        let assign35030_e45866: f64 = (1.0 + assign35030_e45865);
        let assign35030_e45867: f64 = (assign35030_e45856 * assign35030_e45866);
        let assign35030_e45868: f64 = (0.5 * assign35030_e45867);
        let assign35030_e45869: f64 = (1.0 + assign35030_e45868);
        let assign35030_e45870: f64 = (assign35030_e45848 * assign35030_e45869);
        let assign35030_e45871: f64 = (1.0 + assign35030_e45870);
        let assign35030_e45872: f64 = (1e100 * assign35030_e45871);
        (assign35030_e45872, (1e100 * (((-((assign35030_e45844 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign35030_e45869) + (assign35030_e45848 * (0.5 * (((-((assign35030_e45852 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign35030_e45866) + (assign35030_e45856 * ((-((assign35030_e45859 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign35030_e45844 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign35030_e45869) + (assign35030_e45848 * (0.5 * (((-((assign35030_e45852 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign35030_e45866) + (assign35030_e45856 * ((-((assign35030_e45859 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign35030_e45844 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign35030_e45869) + (assign35030_e45848 * (0.5 * (((-((assign35030_e45852 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign35030_e45866) + (assign35030_e45856 * ((-((assign35030_e45859 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign35030_e45844 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign35030_e45869) + (assign35030_e45848 * (0.5 * (((-((assign35030_e45852 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign35030_e45866) + (assign35030_e45856 * ((-((assign35030_e45859 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35030_e45874;
        var_tmp_dn6 = assign35030_e45874_d_n6;
        var_tmp_dn7 = assign35030_e45874_d_n7;
        var_tmp_dn8 = assign35030_e45874_d_n8;
        var_tmp_dn9 = assign35030_e45874_d_n9;

        let (assign35040_e45894, assign35040_e45894_d_n6, assign35040_e45894_d_n7, assign35040_e45894_d_n8, assign35040_e45894_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard714 == 0.0)) {
        let assign35040_e45887: f64 = (var_v4 * var_fmaxr);
        let assign35040_e45889: f64 = (assign35040_e45887 * var_fmaxr);
        let assign35040_e45891: f64 = (assign35040_e45889 * var_tmp);
        let assign35040_e45892: f64 = (var_cbbtbotd_i * assign35040_e45891);
        (assign35040_e45892, (var_cbbtbotd_i * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign35040_e45887 * var_fmaxr_dn6)) * var_tmp) + (assign35040_e45889 * var_tmp_dn6))), (var_cbbtbotd_i * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign35040_e45887 * var_fmaxr_dn7)) * var_tmp) + (assign35040_e45889 * var_tmp_dn7))), (var_cbbtbotd_i * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign35040_e45887 * var_fmaxr_dn8)) * var_tmp) + (assign35040_e45889 * var_tmp_dn8))), (var_cbbtbotd_i * (((((var_v4 * var_fmaxr_dn9) * var_fmaxr) + (assign35040_e45887 * var_fmaxr_dn9)) * var_tmp) + (assign35040_e45889 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign35040_e45894;
        var_ibbt_dn6 = assign35040_e45894_d_n6;
        var_ibbt_dn7 = assign35040_e45894_d_n7;
        var_ibbt_dn8 = assign35040_e45894_d_n8;
        var_ibbt_dn9 = assign35040_e45894_d_n9;

        let assign35050_e45897: f64 = if var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard718 = assign35050_e45897;

        let (assign35060_e45908, assign35060_e45908_d_n6, assign35060_e45908_d_n7, assign35060_e45908_d_n8, assign35060_e45908_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard718 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign35060_e45908;
        var_fbreakdown_dn6 = assign35060_e45908_d_n6;
        var_fbreakdown_dn7 = assign35060_e45908_d_n7;
        var_fbreakdown_dn8 = assign35060_e45908_d_n8;
        var_fbreakdown_dn9 = assign35060_e45908_d_n9;

        let assign35070_e45911: f64 = (-var_alphaav);
        let assign35070_e45913: f64 = (assign35070_e45911 * var_vbrbotd_i);
        let assign35070_e45914: f64 = if var_vav > assign35070_e45913 { 1.0 } else { 0.0 };
        var_guard719 = assign35070_e45914;

        let assign35080_e45917: f64 = if var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard720 = assign35080_e45917;

        let (assign35090_e45947, assign35090_e45947_d_n6, assign35090_e45947_d_n7, assign35090_e45947_d_n8, assign35090_e45947_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard718 == 0.0)) && (var_guard719 != 0.0)) && (var_guard720 != 0.0)) {
        let assign35090_e45933: f64 = (var_vav * var_vbrinvbot_d);
        let assign35090_e45936: f64 = (var_vav * var_vbrinvbot_d);
        let assign35090_e45937: f64 = (assign35090_e45933 * assign35090_e45936);
        let assign35090_e45940: f64 = (var_vav * var_vbrinvbot_d);
        let assign35090_e45941: f64 = (assign35090_e45937 * assign35090_e45940);
        let assign35090_e45944: f64 = (var_vav * var_vbrinvbot_d);
        let assign35090_e45945: f64 = (assign35090_e45941 * assign35090_e45944);
        (assign35090_e45945, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35090_e45947;
        var_tmp_dn6 = assign35090_e45947_d_n6;
        var_tmp_dn7 = assign35090_e45947_d_n7;
        var_tmp_dn8 = assign35090_e45947_d_n8;
        var_tmp_dn9 = assign35090_e45947_d_n9;

        let (assign35100_e45969, assign35100_e45969_d_n6, assign35100_e45969_d_n7, assign35100_e45969_d_n8, assign35100_e45969_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard718 == 0.0)) && (var_guard719 != 0.0)) && (var_guard720 == 0.0)) {
        let assign35100_e45964: f64 = (var_vav * var_vbrinvbot_d);
        let assign35100_e45965: f64 = (assign35100_e45964).abs();
        let assign35100_e45967: f64 = (assign35100_e45965).powf(var_pbrbotd_i);
        (assign35100_e45967, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35100_e45969;
        var_tmp_dn6 = assign35100_e45969_d_n6;
        var_tmp_dn7 = assign35100_e45969_d_n7;
        var_tmp_dn8 = assign35100_e45969_d_n8;
        var_tmp_dn9 = assign35100_e45969_d_n9;

        let (assign35110_e45987, assign35110_e45987_d_n6, assign35110_e45987_d_n7, assign35110_e45987_d_n8, assign35110_e45987_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard718 == 0.0)) && (var_guard719 != 0.0)) {
        let assign35110_e45984: f64 = (1.0 - var_tmp);
        let assign35110_e45985: f64 = (1.0 / assign35110_e45984);
        (assign35110_e45985, (-((-var_tmp_dn6) / (assign35110_e45984 * assign35110_e45984))), (-((-var_tmp_dn7) / (assign35110_e45984 * assign35110_e45984))), (-((-var_tmp_dn8) / (assign35110_e45984 * assign35110_e45984))), (-((-var_tmp_dn9) / (assign35110_e45984 * assign35110_e45984))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign35110_e45987;
        var_fbreakdown_dn6 = assign35110_e45987_d_n6;
        var_fbreakdown_dn7 = assign35110_e45987_d_n7;
        var_fbreakdown_dn8 = assign35110_e45987_d_n8;
        var_fbreakdown_dn9 = assign35110_e45987_d_n9;

        let (assign35120_e46010, assign35120_e46010_d_n6, assign35120_e46010_d_n7, assign35120_e46010_d_n8, assign35120_e46010_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) && (var_guard718 == 0.0)) && (var_guard719 == 0.0)) {
        let assign35120_e46004: f64 = (var_alphaav * var_vbrbotd_i);
        let assign35120_e46005: f64 = (var_vav + assign35120_e46004);
        let assign35120_e46007: f64 = (assign35120_e46005 * var_slopebot_d);
        let assign35120_e46008: f64 = (var_fstopbot_d + assign35120_e46007);
        (assign35120_e46008, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign35120_e46010;
        var_fbreakdown_dn6 = assign35120_e46010_d_n6;
        var_fbreakdown_dn7 = assign35120_e46010_d_n7;
        var_fbreakdown_dn8 = assign35120_e46010_d_n8;
        var_fbreakdown_dn9 = assign35120_e46010_d_n9;

        let (assign35130_e46029, assign35130_e46029_d_n6, assign35130_e46029_d_n7, assign35130_e46029_d_n8, assign35130_e46029_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard704 == 0.0)) {
        let assign35130_e46020: f64 = (var_id__blk212 + var_isrh);
        let assign35130_e46022: f64 = (assign35130_e46020 + var_itat);
        let assign35130_e46024: f64 = (assign35130_e46022 + var_ibbt);
        let assign35130_e46025: f64 = (p.p29 * assign35130_e46024);
        let assign35130_e46027: f64 = (assign35130_e46025 * var_fbreakdown);
        (assign35130_e46027, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign35130_e46025 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign35130_e46025 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign35130_e46025 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign35130_e46025 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign35130_e46029;
        var_ijunbot_dn6 = assign35130_e46029_d_n6;
        var_ijunbot_dn7 = assign35130_e46029_d_n7;
        var_ijunbot_dn8 = assign35130_e46029_d_n8;
        var_ijunbot_dn9 = assign35130_e46029_d_n9;

        let assign35140_e46032: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard721 = assign35140_e46032;

        let (assign35150_e46040, assign35150_e46040_d_n6, assign35150_e46040_d_n7, assign35150_e46040_d_n8, assign35150_e46040_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign35150_e46040;
        var_ijunsti_dn6 = assign35150_e46040_d_n6;
        var_ijunsti_dn7 = assign35150_e46040_d_n7;
        var_ijunsti_dn8 = assign35150_e46040_d_n8;
        var_ijunsti_dn9 = assign35150_e46040_d_n9;

        let (assign35160_e46051,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) {
        let assign35160_e46049: f64 = (var_idsatsti_d * var_idmult);
        (assign35160_e46049,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign35160_e46051;

        let assign35170_e46058: f64 = if ((var_csrhstid_i == 0.0) && (var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard722 = assign35170_e46058;

        let (assign35180_e46069, assign35180_e46069_d_n6, assign35180_e46069_d_n7, assign35180_e46069_d_n8, assign35180_e46069_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard722 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign35180_e46069;
        var_isrh_dn6 = assign35180_e46069_d_n6;
        var_isrh_dn7 = assign35180_e46069_d_n7;
        var_isrh_dn8 = assign35180_e46069_d_n8;
        var_isrh_dn9 = assign35180_e46069_d_n9;

        let (assign35190_e46083,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard722 == 0.0)) {
        let assign35190_e46081: f64 = (var_vbisti_d - var_vjsrh);
        (assign35190_e46081,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign35190_e46083;

        let (assign35200_e46102,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard722 == 0.0)) {
        let assign35200_e46097: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign35200_e46098: f64 = (1.0 - assign35200_e46097);
        let assign35200_e46099: f64 = (assign35200_e46098).sqrt();
        let assign35200_e46100: f64 = (1.0 - assign35200_e46099);
        (assign35200_e46100,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign35200_e46102;

        let assign35210_e46105: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard723 = assign35210_e46105;

        let (assign35220_e46119,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard722 == 0.0)) && (var_guard723 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign35220_e46119;

        let (assign35230_e46151,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard722 == 0.0)) && (var_guard723 == 0.0)) {
        let assign35230_e46134: f64 = (var_wsrhstep * var_wsrhstep);
        let assign35230_e46136: f64 = (var_wsrhstep).ln();
        let assign35230_e46137: f64 = (assign35230_e46134 * assign35230_e46136);
        let assign35230_e46140: f64 = (1.0 - var_wsrhstep);
        let assign35230_e46141: f64 = (assign35230_e46137 / assign35230_e46140);
        let assign35230_e46143: f64 = (assign35230_e46141 + var_wsrhstep);
        let assign35230_e46147: f64 = (2.0 * var_pstid_i);
        let assign35230_e46148: f64 = (1.0 - assign35230_e46147);
        let assign35230_e46149: f64 = (assign35230_e46143 * assign35230_e46148);
        (assign35230_e46149,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign35230_e46151;

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
        *var_guard722_slot = var_guard722;
        *var_guard723_slot = var_guard723;
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

    pub(super) fn stamp_transient_block_74(
        var_atatsti_d: f64,
        var_berfc: f64,
        var_btatpartsti_d: f64,
        var_cerfc: f64,
        var_csrhstid_i: f64,
        var_ctatstid_i: f64,
        var_dwsrh: f64,
        var_ftdsti_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard721: f64,
        var_guard722: f64,
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
        var_guard724_slot: &mut f64,
        var_guard725_slot: &mut f64,
        var_guard726_slot: &mut f64,
        var_guard727_slot: &mut f64,
        var_guard728_slot: &mut f64,
        var_guard729_slot: &mut f64,
        var_guard730_slot: &mut f64,
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
        let mut var_guard724: f64 = *var_guard724_slot;
        let mut var_guard725: f64 = *var_guard725_slot;
        let mut var_guard726: f64 = *var_guard726_slot;
        let mut var_guard727: f64 = *var_guard727_slot;
        let mut var_guard728: f64 = *var_guard728_slot;
        let mut var_guard729: f64 = *var_guard729_slot;
        let mut var_guard730: f64 = *var_guard730_slot;
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

        let (assign35240_e46165,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard722 == 0.0)) {
        let assign35240_e46163: f64 = (var_wsrhstep + var_dwsrh);
        (assign35240_e46163,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign35240_e46165;

        let assign35250_e46168: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard724 = assign35250_e46168;

        let (assign35260_e46185, assign35260_e46185_d_n6, assign35260_e46185_d_n7, assign35260_e46185_d_n8, assign35260_e46185_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard722 == 0.0)) && (var_guard724 != 0.0)) {
        let assign35260_e46182: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign35260_e46183: f64 = (assign35260_e46182).sqrt();
        (assign35260_e46183, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35260_e46185;
        var_tmp_dn6 = assign35260_e46185_d_n6;
        var_tmp_dn7 = assign35260_e46185_d_n7;
        var_tmp_dn8 = assign35260_e46185_d_n8;
        var_tmp_dn9 = assign35260_e46185_d_n9;

        let (assign35270_e46204, assign35270_e46204_d_n6, assign35270_e46204_d_n7, assign35270_e46204_d_n8, assign35270_e46204_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard722 == 0.0)) && (var_guard724 == 0.0)) {
        let assign35270_e46200: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign35270_e46202: f64 = (assign35270_e46200).powf(var_pstid_i);
        (assign35270_e46202, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35270_e46204;
        var_tmp_dn6 = assign35270_e46204_d_n6;
        var_tmp_dn7 = assign35270_e46204_d_n7;
        var_tmp_dn8 = assign35270_e46204_d_n8;
        var_tmp_dn9 = assign35270_e46204_d_n9;

        let (assign35280_e46218, assign35280_e46218_d_n6, assign35280_e46218_d_n7, assign35280_e46218_d_n8, assign35280_e46218_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard722 == 0.0)) {
        let assign35280_e46216: f64 = (var_wdepnulrsti_d * var_tmp);
        (assign35280_e46216, (var_wdepnulrsti_d * var_tmp_dn6), (var_wdepnulrsti_d * var_tmp_dn7), (var_wdepnulrsti_d * var_tmp_dn8), (var_wdepnulrsti_d * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign35280_e46218;
        var_wdep_dn6 = assign35280_e46218_d_n6;
        var_wdep_dn7 = assign35280_e46218_d_n7;
        var_wdep_dn8 = assign35280_e46218_d_n8;
        var_wdep_dn9 = assign35280_e46218_d_n9;

        let (assign35290_e46236, assign35290_e46236_d_n6, assign35290_e46236_d_n7, assign35290_e46236_d_n8, assign35290_e46236_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard722 == 0.0)) {
        let assign35290_e46231: f64 = (var_zinv - 1.0);
        let assign35290_e46233: f64 = (assign35290_e46231 * var_wdep);
        let assign35290_e46234: f64 = (var_ftdsti_d * assign35290_e46233);
        (assign35290_e46234, (var_ftdsti_d * (assign35290_e46231 * var_wdep_dn6)), (var_ftdsti_d * (assign35290_e46231 * var_wdep_dn7)), (var_ftdsti_d * (assign35290_e46231 * var_wdep_dn8)), (var_ftdsti_d * (assign35290_e46231 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign35290_e46236;
        var_asrh_dn6 = assign35290_e46236_d_n6;
        var_asrh_dn7 = assign35290_e46236_d_n7;
        var_asrh_dn8 = assign35290_e46236_d_n8;
        var_asrh_dn9 = assign35290_e46236_d_n9;

        let (assign35300_e46252, assign35300_e46252_d_n6, assign35300_e46252_d_n7, assign35300_e46252_d_n8, assign35300_e46252_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard722 == 0.0)) {
        let assign35300_e46249: f64 = (var_asrh * var_wsrh);
        let assign35300_e46250: f64 = (var_csrhstid_i * assign35300_e46249);
        (assign35300_e46250, (var_csrhstid_i * (var_asrh_dn6 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn7 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn8 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign35300_e46252;
        var_isrh_dn6 = assign35300_e46252_d_n6;
        var_isrh_dn7 = assign35300_e46252_d_n7;
        var_isrh_dn8 = assign35300_e46252_d_n8;
        var_isrh_dn9 = assign35300_e46252_d_n9;

        let assign35310_e46255: f64 = if var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard725 = assign35310_e46255;

        let (assign35320_e46266, assign35320_e46266_d_n6, assign35320_e46266_d_n7, assign35320_e46266_d_n8, assign35320_e46266_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign35320_e46266;
        var_itat_dn6 = assign35320_e46266_d_n6;
        var_itat_dn7 = assign35320_e46266_d_n7;
        var_itat_dn8 = assign35320_e46266_d_n8;
        var_itat_dn9 = assign35320_e46266_d_n9;

        let (assign35330_e46284, assign35330_e46284_d_n6, assign35330_e46284_d_n7, assign35330_e46284_d_n8, assign35330_e46284_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) {
        let assign35330_e46279: f64 = (var_wdep * var_one_minus_psti_d);
        let assign35330_e46281: f64 = (assign35330_e46279 / var_vbi_minus_vjsrh);
        let assign35330_e46282: f64 = (var_btatpartsti_d * assign35330_e46281);
        (assign35330_e46282, (var_btatpartsti_d * ((var_wdep_dn6 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn7 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn8 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn9 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign35330_e46284;
        var_btat_dn6 = assign35330_e46284_d_n6;
        var_btat_dn7 = assign35330_e46284_d_n7;
        var_btat_dn8 = assign35330_e46284_d_n8;
        var_btat_dn9 = assign35330_e46284_d_n9;

        let (assign35340_e46300, assign35340_e46300_d_n6, assign35340_e46300_d_n7, assign35340_e46300_d_n8, assign35340_e46300_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) {
        let assign35340_e46296: f64 = (0.666666666666667 * var_atatsti_d);
        let assign35340_e46298: f64 = (assign35340_e46296 / var_btat);
        (assign35340_e46298, (-((assign35340_e46296 * var_btat_dn6) / (var_btat * var_btat))), (-((assign35340_e46296 * var_btat_dn7) / (var_btat * var_btat))), (-((assign35340_e46296 * var_btat_dn8) / (var_btat * var_btat))), (-((assign35340_e46296 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign35340_e46300;
        var_twoatatoverthreebtat_dn6 = assign35340_e46300_d_n6;
        var_twoatatoverthreebtat_dn7 = assign35340_e46300_d_n7;
        var_twoatatoverthreebtat_dn8 = assign35340_e46300_d_n8;
        var_twoatatoverthreebtat_dn9 = assign35340_e46300_d_n9;

        let (assign35350_e46314, assign35350_e46314_d_n6, assign35350_e46314_d_n7, assign35350_e46314_d_n8, assign35350_e46314_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) {
        let assign35350_e46312: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign35350_e46312, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign35350_e46314;
        var_umaxbeforelimiting_dn6 = assign35350_e46314_d_n6;
        var_umaxbeforelimiting_dn7 = assign35350_e46314_d_n7;
        var_umaxbeforelimiting_dn8 = assign35350_e46314_d_n8;
        var_umaxbeforelimiting_dn9 = assign35350_e46314_d_n9;

        let (assign35360_e46335, assign35360_e46335_d_n6, assign35360_e46335_d_n7, assign35360_e46335_d_n8, assign35360_e46335_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) {
        let assign35360_e46326: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign35360_e46329: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign35360_e46331: f64 = (assign35360_e46329 + 1.0);
        let assign35360_e46332: f64 = (assign35360_e46326 / assign35360_e46331);
        let assign35360_e46333: f64 = (assign35360_e46332).sqrt();
        (assign35360_e46333, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign35360_e46331) - (assign35360_e46326 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign35360_e46331 * assign35360_e46331)) / (2.0 * assign35360_e46333)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign35360_e46331) - (assign35360_e46326 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign35360_e46331 * assign35360_e46331)) / (2.0 * assign35360_e46333)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign35360_e46331) - (assign35360_e46326 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign35360_e46331 * assign35360_e46331)) / (2.0 * assign35360_e46333)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign35360_e46331) - (assign35360_e46326 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign35360_e46331 * assign35360_e46331)) / (2.0 * assign35360_e46333)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign35360_e46335;
        var_umax_dn6 = assign35360_e46335_d_n6;
        var_umax_dn7 = assign35360_e46335_d_n7;
        var_umax_dn8 = assign35360_e46335_d_n8;
        var_umax_dn9 = assign35360_e46335_d_n9;

        let (assign35370_e46348, assign35370_e46348_d_n6, assign35370_e46348_d_n7, assign35370_e46348_d_n8, assign35370_e46348_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) {
        let assign35370_e46346: f64 = (var_umax).sqrt();
        (assign35370_e46346, (var_umax_dn6 / (2.0 * assign35370_e46346)), (var_umax_dn7 / (2.0 * assign35370_e46346)), (var_umax_dn8 / (2.0 * assign35370_e46346)), (var_umax_dn9 / (2.0 * assign35370_e46346)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign35370_e46348;
        var_sqrtumax_dn6 = assign35370_e46348_d_n6;
        var_sqrtumax_dn7 = assign35370_e46348_d_n7;
        var_sqrtumax_dn8 = assign35370_e46348_d_n8;
        var_sqrtumax_dn9 = assign35370_e46348_d_n9;

        let (assign35380_e46362, assign35380_e46362_d_n6, assign35380_e46362_d_n7, assign35380_e46362_d_n8, assign35380_e46362_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) {
        let assign35380_e46360: f64 = (var_umax * var_sqrtumax);
        (assign35380_e46360, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign35380_e46362;
        var_umaxpoweronepointfive_dn6 = assign35380_e46362_d_n6;
        var_umaxpoweronepointfive_dn7 = assign35380_e46362_d_n7;
        var_umaxpoweronepointfive_dn8 = assign35380_e46362_d_n8;
        var_umaxpoweronepointfive_dn9 = assign35380_e46362_d_n9;

        let assign35390_e46364: f64 = (-var_pstid_i);
        let assign35390_e46366: f64 = (assign35390_e46364 * var_one_over_one_minus_psti_d);
        let assign35390_e46368: f64 = (-1.0);
        let assign35390_e46369: f64 = if assign35390_e46366 == assign35390_e46368 { 1.0 } else { 0.0 };
        var_guard726 = assign35390_e46369;

        let (assign35400_e46389, assign35400_e46389_d_n6, assign35400_e46389_d_n7, assign35400_e46389_d_n8, assign35400_e46389_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) && (var_guard726 != 0.0)) {
        let assign35400_e46385: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign35400_e46386: f64 = (1.0 + assign35400_e46385);
        let assign35400_e46387: f64 = (1.0 / assign35400_e46386);
        (assign35400_e46387, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign35400_e46386 * assign35400_e46386))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign35400_e46386 * assign35400_e46386))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign35400_e46386 * assign35400_e46386))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign35400_e46386 * assign35400_e46386))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign35400_e46389;
        var_wgamma_dn6 = assign35400_e46389_d_n6;
        var_wgamma_dn7 = assign35400_e46389_d_n7;
        var_wgamma_dn8 = assign35400_e46389_d_n8;
        var_wgamma_dn9 = assign35400_e46389_d_n9;

        let (assign35410_e46413, assign35410_e46413_d_n6, assign35410_e46413_d_n7, assign35410_e46413_d_n8, assign35410_e46413_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) && (var_guard726 == 0.0)) {
        let assign35410_e46405: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign35410_e46406: f64 = (1.0 + assign35410_e46405);
        let assign35410_e46408: f64 = (-var_pstid_i);
        let assign35410_e46410: f64 = (assign35410_e46408 * var_one_over_one_minus_psti_d);
        let assign35410_e46411: f64 = (assign35410_e46406).powf(assign35410_e46410);
        (assign35410_e46411, if 0.0 == 0.0 && ((assign35410_e46410) as f64).is_finite() && ((assign35410_e46410) as f64).fract() == 0.0 { if assign35410_e46410 == 0.0 { 0.0 } else { (assign35410_e46410 * ((assign35410_e46406).powf(assign35410_e46410 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign35410_e46411 * (assign35410_e46410 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign35410_e46406))) }, if 0.0 == 0.0 && ((assign35410_e46410) as f64).is_finite() && ((assign35410_e46410) as f64).fract() == 0.0 { if assign35410_e46410 == 0.0 { 0.0 } else { (assign35410_e46410 * ((assign35410_e46406).powf(assign35410_e46410 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign35410_e46411 * (assign35410_e46410 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign35410_e46406))) }, if 0.0 == 0.0 && ((assign35410_e46410) as f64).is_finite() && ((assign35410_e46410) as f64).fract() == 0.0 { if assign35410_e46410 == 0.0 { 0.0 } else { (assign35410_e46410 * ((assign35410_e46406).powf(assign35410_e46410 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign35410_e46411 * (assign35410_e46410 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign35410_e46406))) }, if 0.0 == 0.0 && ((assign35410_e46410) as f64).is_finite() && ((assign35410_e46410) as f64).fract() == 0.0 { if assign35410_e46410 == 0.0 { 0.0 } else { (assign35410_e46410 * ((assign35410_e46406).powf(assign35410_e46410 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign35410_e46411 * (assign35410_e46410 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign35410_e46406))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign35410_e46413;
        var_wgamma_dn6 = assign35410_e46413_d_n6;
        var_wgamma_dn7 = assign35410_e46413_d_n7;
        var_wgamma_dn8 = assign35410_e46413_d_n8;
        var_wgamma_dn9 = assign35410_e46413_d_n9;

        let (assign35420_e46431, assign35420_e46431_d_n6, assign35420_e46431_d_n7, assign35420_e46431_d_n8, assign35420_e46431_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) {
        let assign35420_e46425: f64 = (var_wsrh * var_wgamma);
        let assign35420_e46428: f64 = (var_wsrh + var_wgamma);
        let assign35420_e46429: f64 = (assign35420_e46425 / assign35420_e46428);
        (assign35420_e46429, ((((var_wsrh * var_wgamma_dn6) * assign35420_e46428) - (assign35420_e46425 * var_wgamma_dn6)) / (assign35420_e46428 * assign35420_e46428)), ((((var_wsrh * var_wgamma_dn7) * assign35420_e46428) - (assign35420_e46425 * var_wgamma_dn7)) / (assign35420_e46428 * assign35420_e46428)), ((((var_wsrh * var_wgamma_dn8) * assign35420_e46428) - (assign35420_e46425 * var_wgamma_dn8)) / (assign35420_e46428 * assign35420_e46428)), ((((var_wsrh * var_wgamma_dn9) * assign35420_e46428) - (assign35420_e46425 * var_wgamma_dn9)) / (assign35420_e46428 * assign35420_e46428)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign35420_e46431;
        var_wtat_dn6 = assign35420_e46431_d_n6;
        var_wtat_dn7 = assign35420_e46431_d_n7;
        var_wtat_dn8 = assign35420_e46431_d_n8;
        var_wtat_dn9 = assign35420_e46431_d_n9;

        let (assign35430_e46448, assign35430_e46448_d_n6, assign35430_e46448_d_n7, assign35430_e46448_d_n8, assign35430_e46448_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) {
        let assign35430_e46444: f64 = (var_btat / var_sqrtumax);
        let assign35430_e46445: f64 = (0.375 * assign35430_e46444);
        let assign35430_e46446: f64 = (assign35430_e46445).sqrt();
        (assign35430_e46446, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign35430_e46446)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign35430_e46446)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign35430_e46446)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign35430_e46446)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign35430_e46448;
        var_ktat_dn6 = assign35430_e46448_d_n6;
        var_ktat_dn7 = assign35430_e46448_d_n7;
        var_ktat_dn8 = assign35430_e46448_d_n8;
        var_ktat_dn9 = assign35430_e46448_d_n9;

        let (assign35440_e46466, assign35440_e46466_d_n6, assign35440_e46466_d_n7, assign35440_e46466_d_n8, assign35440_e46466_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) {
        let assign35440_e46461: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign35440_e46462: f64 = (2.0 * assign35440_e46461);
        let assign35440_e46464: f64 = (assign35440_e46462 - var_umax);
        (assign35440_e46464, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign35440_e46466;
        var_ltat_dn6 = assign35440_e46466_d_n6;
        var_ltat_dn7 = assign35440_e46466_d_n7;
        var_ltat_dn8 = assign35440_e46466_d_n8;
        var_ltat_dn9 = assign35440_e46466_d_n9;

        let (assign35450_e46492, assign35450_e46492_d_n6, assign35450_e46492_d_n7, assign35450_e46492_d_n8, assign35450_e46492_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) {
        let assign35450_e46478: f64 = (var_atatsti_d * var_twoatatoverthreebtat);
        let assign35450_e46480: f64 = (assign35450_e46478 * var_sqrtumax);
        let assign35450_e46483: f64 = (var_atatsti_d * var_umax);
        let assign35450_e46484: f64 = (assign35450_e46480 - assign35450_e46483);
        let assign35450_e46488: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign35450_e46489: f64 = (0.5 * assign35450_e46488);
        let assign35450_e46490: f64 = (assign35450_e46484 + assign35450_e46489);
        (assign35450_e46490, (((((var_atatsti_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign35450_e46478 * var_sqrtumax_dn6)) - (var_atatsti_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign35450_e46478 * var_sqrtumax_dn7)) - (var_atatsti_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign35450_e46478 * var_sqrtumax_dn8)) - (var_atatsti_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign35450_e46478 * var_sqrtumax_dn9)) - (var_atatsti_d * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign35450_e46492;
        var_mtat_dn6 = assign35450_e46492_d_n6;
        var_mtat_dn7 = assign35450_e46492_d_n7;
        var_mtat_dn8 = assign35450_e46492_d_n8;
        var_mtat_dn9 = assign35450_e46492_d_n9;

        let (assign35460_e46508, assign35460_e46508_d_n6, assign35460_e46508_d_n7, assign35460_e46508_d_n8, assign35460_e46508_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) {
        let assign35460_e46504: f64 = (var_ltat - 1.0);
        let assign35460_e46506: f64 = (assign35460_e46504 * var_ktat);
        (assign35460_e46506, ((var_ltat_dn6 * var_ktat) + (assign35460_e46504 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign35460_e46504 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign35460_e46504 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign35460_e46504 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign35460_e46508;
        var_xerfc_dn6 = assign35460_e46508_d_n6;
        var_xerfc_dn7 = assign35460_e46508_d_n7;
        var_xerfc_dn8 = assign35460_e46508_d_n8;
        var_xerfc_dn9 = assign35460_e46508_d_n9;

        let (assign35470_e46522, assign35470_e46522_d_n6, assign35470_e46522_d_n7, assign35470_e46522_d_n8, assign35470_e46522_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) {
        let assign35470_e46520: f64 = (var_xerfc * var_xerfc);
        (assign35470_e46520, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign35470_e46522;
        var_ysq_dn6 = assign35470_e46522_d_n6;
        var_ysq_dn7 = assign35470_e46522_d_n7;
        var_ysq_dn8 = assign35470_e46522_d_n8;
        var_ysq_dn9 = assign35470_e46522_d_n9;

        let assign35480_e46525: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard727 = assign35480_e46525;

        let (assign35490_e46545, assign35490_e46545_d_n6, assign35490_e46545_d_n7, assign35490_e46545_d_n8, assign35490_e46545_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) && (var_guard727 != 0.0)) {
        let assign35490_e46541: f64 = (var_perfc * var_xerfc);
        let assign35490_e46542: f64 = (1.0 + assign35490_e46541);
        let assign35490_e46543: f64 = (1.0 / assign35490_e46542);
        (assign35490_e46543, (-((var_perfc * var_xerfc_dn6) / (assign35490_e46542 * assign35490_e46542))), (-((var_perfc * var_xerfc_dn7) / (assign35490_e46542 * assign35490_e46542))), (-((var_perfc * var_xerfc_dn8) / (assign35490_e46542 * assign35490_e46542))), (-((var_perfc * var_xerfc_dn9) / (assign35490_e46542 * assign35490_e46542))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign35490_e46545;
        var_terfc_dn6 = assign35490_e46545_d_n6;
        var_terfc_dn7 = assign35490_e46545_d_n7;
        var_terfc_dn8 = assign35490_e46545_d_n8;
        var_terfc_dn9 = assign35490_e46545_d_n9;

        let (assign35500_e46566, assign35500_e46566_d_n6, assign35500_e46566_d_n7, assign35500_e46566_d_n8, assign35500_e46566_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) && (var_guard727 == 0.0)) {
        let assign35500_e46562: f64 = (var_perfc * var_xerfc);
        let assign35500_e46563: f64 = (1.0 - assign35500_e46562);
        let assign35500_e46564: f64 = (1.0 / assign35500_e46563);
        (assign35500_e46564, (-((-(var_perfc * var_xerfc_dn6)) / (assign35500_e46563 * assign35500_e46563))), (-((-(var_perfc * var_xerfc_dn7)) / (assign35500_e46563 * assign35500_e46563))), (-((-(var_perfc * var_xerfc_dn8)) / (assign35500_e46563 * assign35500_e46563))), (-((-(var_perfc * var_xerfc_dn9)) / (assign35500_e46563 * assign35500_e46563))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign35500_e46566;
        var_terfc_dn6 = assign35500_e46566_d_n6;
        var_terfc_dn7 = assign35500_e46566_d_n7;
        var_terfc_dn8 = assign35500_e46566_d_n8;
        var_terfc_dn9 = assign35500_e46566_d_n9;

        let assign35510_e46568: f64 = (-var_ysq);
        let assign35510_e46570: f64 = (assign35510_e46568 + var_mtat);
        let assign35510_e46572: f64 = (-230.25850929940458);
        let assign35510_e46573: f64 = if assign35510_e46570 > assign35510_e46572 { 1.0 } else { 0.0 };
        var_guard728 = assign35510_e46573;

        let (assign35520_e46591, assign35520_e46591_d_n6, assign35520_e46591_d_n7, assign35520_e46591_d_n8, assign35520_e46591_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) && (var_guard728 != 0.0)) {
        let assign35520_e46586: f64 = (-var_ysq);
        let assign35520_e46588: f64 = (assign35520_e46586 + var_mtat);
        let assign35520_e46589: f64 = (assign35520_e46588).exp();
        (assign35520_e46589, (assign35520_e46589 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign35520_e46589 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign35520_e46589 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign35520_e46589 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35520_e46591;
        var_tmp_dn6 = assign35520_e46591_d_n6;
        var_tmp_dn7 = assign35520_e46591_d_n7;
        var_tmp_dn8 = assign35520_e46591_d_n8;
        var_tmp_dn9 = assign35520_e46591_d_n9;

        let (assign35530_e46640, assign35530_e46640_d_n6, assign35530_e46640_d_n7, assign35530_e46640_d_n8, assign35530_e46640_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) && (var_guard728 == 0.0)) {
        let assign35530_e46607: f64 = (-230.25850929940458);
        let assign35530_e46609: f64 = (-var_ysq);
        let assign35530_e46611: f64 = (assign35530_e46609 + var_mtat);
        let assign35530_e46612: f64 = (assign35530_e46607 - assign35530_e46611);
        let assign35530_e46616: f64 = (-230.25850929940458);
        let assign35530_e46618: f64 = (-var_ysq);
        let assign35530_e46620: f64 = (assign35530_e46618 + var_mtat);
        let assign35530_e46621: f64 = (assign35530_e46616 - assign35530_e46620);
        let assign35530_e46624: f64 = (-230.25850929940458);
        let assign35530_e46626: f64 = (-var_ysq);
        let assign35530_e46628: f64 = (assign35530_e46626 + var_mtat);
        let assign35530_e46629: f64 = (assign35530_e46624 - assign35530_e46628);
        let assign35530_e46631: f64 = (assign35530_e46629 * 0.3333333333333333);
        let assign35530_e46632: f64 = (1.0 + assign35530_e46631);
        let assign35530_e46633: f64 = (assign35530_e46621 * assign35530_e46632);
        let assign35530_e46634: f64 = (0.5 * assign35530_e46633);
        let assign35530_e46635: f64 = (1.0 + assign35530_e46634);
        let assign35530_e46636: f64 = (assign35530_e46612 * assign35530_e46635);
        let assign35530_e46637: f64 = (1.0 + assign35530_e46636);
        let assign35530_e46638: f64 = (1e-100 / assign35530_e46637);
        (assign35530_e46638, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign35530_e46635) + (assign35530_e46612 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign35530_e46632) + (assign35530_e46621 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign35530_e46637 * assign35530_e46637))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign35530_e46635) + (assign35530_e46612 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign35530_e46632) + (assign35530_e46621 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign35530_e46637 * assign35530_e46637))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign35530_e46635) + (assign35530_e46612 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign35530_e46632) + (assign35530_e46621 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign35530_e46637 * assign35530_e46637))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign35530_e46635) + (assign35530_e46612 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign35530_e46632) + (assign35530_e46621 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign35530_e46637 * assign35530_e46637))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35530_e46640;
        var_tmp_dn6 = assign35530_e46640_d_n6;
        var_tmp_dn7 = assign35530_e46640_d_n7;
        var_tmp_dn8 = assign35530_e46640_d_n8;
        var_tmp_dn9 = assign35530_e46640_d_n9;

        let (assign35540_e46670, assign35540_e46670_d_n6, assign35540_e46670_d_n7, assign35540_e46670_d_n8, assign35540_e46670_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) {
        let assign35540_e46652: f64 = (0.29214664 * var_terfc);
        let assign35540_e46656: f64 = (var_terfc * var_terfc);
        let assign35540_e46657: f64 = (var_berfc * assign35540_e46656);
        let assign35540_e46658: f64 = (assign35540_e46652 + assign35540_e46657);
        let assign35540_e46662: f64 = (var_terfc * var_terfc);
        let assign35540_e46664: f64 = (assign35540_e46662 * var_terfc);
        let assign35540_e46665: f64 = (var_cerfc * assign35540_e46664);
        let assign35540_e46666: f64 = (assign35540_e46658 + assign35540_e46665);
        let assign35540_e46668: f64 = (assign35540_e46666 * var_tmp);
        (assign35540_e46668, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign35540_e46662 * var_terfc_dn6)))) * var_tmp) + (assign35540_e46666 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign35540_e46662 * var_terfc_dn7)))) * var_tmp) + (assign35540_e46666 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign35540_e46662 * var_terfc_dn8)))) * var_tmp) + (assign35540_e46666 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign35540_e46662 * var_terfc_dn9)))) * var_tmp) + (assign35540_e46666 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign35540_e46670;
        var_erfcpos_dn6 = assign35540_e46670_d_n6;
        var_erfcpos_dn7 = assign35540_e46670_d_n7;
        var_erfcpos_dn8 = assign35540_e46670_d_n8;
        var_erfcpos_dn9 = assign35540_e46670_d_n9;

        let assign35550_e46673: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard729 = assign35550_e46673;

        let (assign35560_e46687, assign35560_e46687_d_n6, assign35560_e46687_d_n7, assign35560_e46687_d_n8, assign35560_e46687_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) && (var_guard729 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign35560_e46687;
        var_erfctimesexpmtat_dn6 = assign35560_e46687_d_n6;
        var_erfctimesexpmtat_dn7 = assign35560_e46687_d_n7;
        var_erfctimesexpmtat_dn8 = assign35560_e46687_d_n8;
        var_erfctimesexpmtat_dn9 = assign35560_e46687_d_n9;

        let assign35570_e46690: f64 = (-230.25850929940458);
        let assign35570_e46691: f64 = if var_mtat > assign35570_e46690 { 1.0 } else { 0.0 };
        var_guard730 = assign35570_e46691;

        let (assign35580_e46709, assign35580_e46709_d_n6, assign35580_e46709_d_n7, assign35580_e46709_d_n8, assign35580_e46709_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) && (var_guard729 == 0.0)) && (var_guard730 != 0.0)) {
        let assign35580_e46707: f64 = (var_mtat).exp();
        (assign35580_e46707, (assign35580_e46707 * var_mtat_dn6), (assign35580_e46707 * var_mtat_dn7), (assign35580_e46707 * var_mtat_dn8), (assign35580_e46707 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35580_e46709;
        var_tmp_dn6 = assign35580_e46709_d_n6;
        var_tmp_dn7 = assign35580_e46709_d_n7;
        var_tmp_dn8 = assign35580_e46709_d_n8;
        var_tmp_dn9 = assign35580_e46709_d_n9;

        let (assign35590_e46752, assign35590_e46752_d_n6, assign35590_e46752_d_n7, assign35590_e46752_d_n8, assign35590_e46752_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) && (var_guard729 == 0.0)) && (var_guard730 == 0.0)) {
        let assign35590_e46728: f64 = (-230.25850929940458);
        let assign35590_e46730: f64 = (assign35590_e46728 - var_mtat);
        let assign35590_e46734: f64 = (-230.25850929940458);
        let assign35590_e46736: f64 = (assign35590_e46734 - var_mtat);
        let assign35590_e46739: f64 = (-230.25850929940458);
        let assign35590_e46741: f64 = (assign35590_e46739 - var_mtat);
        let assign35590_e46743: f64 = (assign35590_e46741 * 0.3333333333333333);
        let assign35590_e46744: f64 = (1.0 + assign35590_e46743);
        let assign35590_e46745: f64 = (assign35590_e46736 * assign35590_e46744);
        let assign35590_e46746: f64 = (0.5 * assign35590_e46745);
        let assign35590_e46747: f64 = (1.0 + assign35590_e46746);
        let assign35590_e46748: f64 = (assign35590_e46730 * assign35590_e46747);
        let assign35590_e46749: f64 = (1.0 + assign35590_e46748);
        let assign35590_e46750: f64 = (1e-100 / assign35590_e46749);
        (assign35590_e46750, (-((1e-100 * (((-var_mtat_dn6) * assign35590_e46747) + (assign35590_e46730 * (0.5 * (((-var_mtat_dn6) * assign35590_e46744) + (assign35590_e46736 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign35590_e46749 * assign35590_e46749))), (-((1e-100 * (((-var_mtat_dn7) * assign35590_e46747) + (assign35590_e46730 * (0.5 * (((-var_mtat_dn7) * assign35590_e46744) + (assign35590_e46736 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign35590_e46749 * assign35590_e46749))), (-((1e-100 * (((-var_mtat_dn8) * assign35590_e46747) + (assign35590_e46730 * (0.5 * (((-var_mtat_dn8) * assign35590_e46744) + (assign35590_e46736 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign35590_e46749 * assign35590_e46749))), (-((1e-100 * (((-var_mtat_dn9) * assign35590_e46747) + (assign35590_e46730 * (0.5 * (((-var_mtat_dn9) * assign35590_e46744) + (assign35590_e46736 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign35590_e46749 * assign35590_e46749))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35590_e46752;
        var_tmp_dn6 = assign35590_e46752_d_n6;
        var_tmp_dn7 = assign35590_e46752_d_n7;
        var_tmp_dn8 = assign35590_e46752_d_n8;
        var_tmp_dn9 = assign35590_e46752_d_n9;

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
        *var_guard724_slot = var_guard724;
        *var_guard725_slot = var_guard725;
        *var_guard726_slot = var_guard726;
        *var_guard727_slot = var_guard727;
        *var_guard728_slot = var_guard728;
        *var_guard729_slot = var_guard729;
        *var_guard730_slot = var_guard730;
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

    pub(super) fn stamp_transient_block_75(
        p: &Parameters,
        var_alphaav: f64,
        var_atatsti_d: f64,
        var_cbbtstid_i: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_ctatstid_i: f64,
        var_erfcpos: f64,
        var_erfcpos_dn6: f64,
        var_erfcpos_dn7: f64,
        var_erfcpos_dn8: f64,
        var_erfcpos_dn9: f64,
        var_fbbtsti_d: f64,
        var_fstopsti_d: f64,
        var_ftdgat_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard721: f64,
        var_guard725: f64,
        var_guard729: f64,
        var_idmult: f64,
        var_idsatgat_d: f64,
        var_ktat: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_ktat_dn9: f64,
        var_lgdrain_i: f64,
        var_one_over_one_minus_psti_d: f64,
        var_pbrstid_i: f64,
        var_pgatd_i: f64,
        var_pstid_i: f64,
        var_slopesti_d: f64,
        var_two_psistar: f64,
        var_v4: f64,
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
        var_guard741_slot: &mut f64,
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
        let mut var_guard741: f64 = *var_guard741_slot;
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

        let (assign35600_e46771, assign35600_e46771_d_n6, assign35600_e46771_d_n7, assign35600_e46771_d_n8, assign35600_e46771_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) && (var_guard729 == 0.0)) {
        let assign35600_e46767: f64 = (2.0 * var_tmp);
        let assign35600_e46769: f64 = (assign35600_e46767 - var_erfcpos);
        (assign35600_e46769, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign35600_e46771;
        var_erfctimesexpmtat_dn6 = assign35600_e46771_d_n6;
        var_erfctimesexpmtat_dn7 = assign35600_e46771_d_n7;
        var_erfctimesexpmtat_dn8 = assign35600_e46771_d_n8;
        var_erfctimesexpmtat_dn9 = assign35600_e46771_d_n9;

        let (assign35610_e46791, assign35610_e46791_d_n6, assign35610_e46791_d_n7, assign35610_e46791_d_n8, assign35610_e46791_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) {
        let assign35610_e46783: f64 = (1.772453850905516 * 0.5);
        let assign35610_e46786: f64 = (var_atatsti_d * var_erfctimesexpmtat);
        let assign35610_e46788: f64 = (assign35610_e46786 / var_ktat);
        let assign35610_e46789: f64 = (assign35610_e46783 * assign35610_e46788);
        (assign35610_e46789, (assign35610_e46783 * ((((var_atatsti_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign35610_e46786 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign35610_e46783 * ((((var_atatsti_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign35610_e46786 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign35610_e46783 * ((((var_atatsti_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign35610_e46786 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign35610_e46783 * ((((var_atatsti_d * var_erfctimesexpmtat_dn9) * var_ktat) - (assign35610_e46786 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign35610_e46791;
        var_gammamax_dn6 = assign35610_e46791_d_n6;
        var_gammamax_dn7 = assign35610_e46791_d_n7;
        var_gammamax_dn8 = assign35610_e46791_d_n8;
        var_gammamax_dn9 = assign35610_e46791_d_n9;

        let (assign35620_e46809, assign35620_e46809_d_n6, assign35620_e46809_d_n7, assign35620_e46809_d_n8, assign35620_e46809_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard725 == 0.0)) {
        let assign35620_e46804: f64 = (var_asrh * var_gammamax);
        let assign35620_e46806: f64 = (assign35620_e46804 * var_wtat);
        let assign35620_e46807: f64 = (var_ctatstid_i * assign35620_e46806);
        (assign35620_e46807, (var_ctatstid_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign35620_e46804 * var_wtat_dn6))), (var_ctatstid_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign35620_e46804 * var_wtat_dn7))), (var_ctatstid_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign35620_e46804 * var_wtat_dn8))), (var_ctatstid_i * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign35620_e46804 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign35620_e46809;
        var_itat_dn6 = assign35620_e46809_d_n6;
        var_itat_dn7 = assign35620_e46809_d_n7;
        var_itat_dn8 = assign35620_e46809_d_n8;
        var_itat_dn9 = assign35620_e46809_d_n9;

        let assign35630_e46812: f64 = if var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard731 = assign35630_e46812;

        let (assign35640_e46823, assign35640_e46823_d_n6, assign35640_e46823_d_n7, assign35640_e46823_d_n8, assign35640_e46823_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard731 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign35640_e46823;
        var_ibbt_dn6 = assign35640_e46823_d_n6;
        var_ibbt_dn7 = assign35640_e46823_d_n7;
        var_ibbt_dn8 = assign35640_e46823_d_n8;
        var_ibbt_dn9 = assign35640_e46823_d_n9;

        let assign35650_e46826: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard732 = assign35650_e46826;

        let (assign35660_e46845, assign35660_e46845_d_n6, assign35660_e46845_d_n7, assign35660_e46845_d_n8, assign35660_e46845_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard731 == 0.0)) && (var_guard732 != 0.0)) {
        let assign35660_e46840: f64 = (var_vbirstid_i - var_vbbt);
        let assign35660_e46842: f64 = (assign35660_e46840 * var_vbirstiinv_d);
        let assign35660_e46843: f64 = (assign35660_e46842).sqrt();
        (assign35660_e46843, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35660_e46845;
        var_tmp_dn6 = assign35660_e46845_d_n6;
        var_tmp_dn7 = assign35660_e46845_d_n7;
        var_tmp_dn8 = assign35660_e46845_d_n8;
        var_tmp_dn9 = assign35660_e46845_d_n9;

        let (assign35670_e46866, assign35670_e46866_d_n6, assign35670_e46866_d_n7, assign35670_e46866_d_n8, assign35670_e46866_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard731 == 0.0)) && (var_guard732 == 0.0)) {
        let assign35670_e46860: f64 = (var_vbirstid_i - var_vbbt);
        let assign35670_e46862: f64 = (assign35670_e46860 * var_vbirstiinv_d);
        let assign35670_e46864: f64 = (assign35670_e46862).powf(var_pstid_i);
        (assign35670_e46864, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35670_e46866;
        var_tmp_dn6 = assign35670_e46866_d_n6;
        var_tmp_dn7 = assign35670_e46866_d_n7;
        var_tmp_dn8 = assign35670_e46866_d_n8;
        var_tmp_dn9 = assign35670_e46866_d_n9;

        let (assign35680_e46886, assign35680_e46886_d_n6, assign35680_e46886_d_n7, assign35680_e46886_d_n8, assign35680_e46886_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard731 == 0.0)) {
        let assign35680_e46879: f64 = (var_vbirstid_i - var_vbbt);
        let assign35680_e46881: f64 = (assign35680_e46879 * var_wdepnulrinvsti_d);
        let assign35680_e46883: f64 = (assign35680_e46881 / var_tmp);
        let assign35680_e46884: f64 = (var_one_over_one_minus_psti_d * assign35680_e46883);
        (assign35680_e46884, (var_one_over_one_minus_psti_d * (-((assign35680_e46881 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign35680_e46881 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign35680_e46881 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign35680_e46881 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign35680_e46886;
        var_fmaxr_dn6 = assign35680_e46886_d_n6;
        var_fmaxr_dn7 = assign35680_e46886_d_n7;
        var_fmaxr_dn8 = assign35680_e46886_d_n8;
        var_fmaxr_dn9 = assign35680_e46886_d_n9;

        let assign35690_e46888: f64 = (-var_fbbtsti_d);
        let assign35690_e46890: f64 = (assign35690_e46888 / var_fmaxr);
        let assign35690_e46891: f64 = (assign35690_e46890).abs();
        let assign35690_e46893: f64 = if assign35690_e46891 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard733 = assign35690_e46893;

        let (assign35700_e46911, assign35700_e46911_d_n6, assign35700_e46911_d_n7, assign35700_e46911_d_n8, assign35700_e46911_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard731 == 0.0)) && (var_guard733 != 0.0)) {
        let assign35700_e46906: f64 = (-var_fbbtsti_d);
        let assign35700_e46908: f64 = (assign35700_e46906 / var_fmaxr);
        let assign35700_e46909: f64 = (assign35700_e46908).exp();
        (assign35700_e46909, (assign35700_e46909 * (-((assign35700_e46906 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign35700_e46909 * (-((assign35700_e46906 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign35700_e46909 * (-((assign35700_e46906 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign35700_e46909 * (-((assign35700_e46906 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35700_e46911;
        var_tmp_dn6 = assign35700_e46911_d_n6;
        var_tmp_dn7 = assign35700_e46911_d_n7;
        var_tmp_dn8 = assign35700_e46911_d_n8;
        var_tmp_dn9 = assign35700_e46911_d_n9;

        let assign35710_e46913: f64 = (-var_fbbtsti_d);
        let assign35710_e46915: f64 = (assign35710_e46913 / var_fmaxr);
        let assign35710_e46917: f64 = if assign35710_e46915 < 0.0 { 1.0 } else { 0.0 };
        var_guard734 = assign35710_e46917;

        let (assign35720_e46968, assign35720_e46968_d_n6, assign35720_e46968_d_n7, assign35720_e46968_d_n8, assign35720_e46968_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard731 == 0.0)) && (var_guard733 == 0.0)) && (var_guard734 != 0.0)) {
        let assign35720_e46935: f64 = (-230.25850929940458);
        let assign35720_e46937: f64 = (-var_fbbtsti_d);
        let assign35720_e46939: f64 = (assign35720_e46937 / var_fmaxr);
        let assign35720_e46940: f64 = (assign35720_e46935 - assign35720_e46939);
        let assign35720_e46944: f64 = (-230.25850929940458);
        let assign35720_e46946: f64 = (-var_fbbtsti_d);
        let assign35720_e46948: f64 = (assign35720_e46946 / var_fmaxr);
        let assign35720_e46949: f64 = (assign35720_e46944 - assign35720_e46948);
        let assign35720_e46952: f64 = (-230.25850929940458);
        let assign35720_e46954: f64 = (-var_fbbtsti_d);
        let assign35720_e46956: f64 = (assign35720_e46954 / var_fmaxr);
        let assign35720_e46957: f64 = (assign35720_e46952 - assign35720_e46956);
        let assign35720_e46959: f64 = (assign35720_e46957 * 0.3333333333333333);
        let assign35720_e46960: f64 = (1.0 + assign35720_e46959);
        let assign35720_e46961: f64 = (assign35720_e46949 * assign35720_e46960);
        let assign35720_e46962: f64 = (0.5 * assign35720_e46961);
        let assign35720_e46963: f64 = (1.0 + assign35720_e46962);
        let assign35720_e46964: f64 = (assign35720_e46940 * assign35720_e46963);
        let assign35720_e46965: f64 = (1.0 + assign35720_e46964);
        let assign35720_e46966: f64 = (1e-100 / assign35720_e46965);
        (assign35720_e46966, (-((1e-100 * (((-(-((assign35720_e46937 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign35720_e46963) + (assign35720_e46940 * (0.5 * (((-(-((assign35720_e46946 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign35720_e46960) + (assign35720_e46949 * ((-(-((assign35720_e46954 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign35720_e46965 * assign35720_e46965))), (-((1e-100 * (((-(-((assign35720_e46937 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign35720_e46963) + (assign35720_e46940 * (0.5 * (((-(-((assign35720_e46946 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign35720_e46960) + (assign35720_e46949 * ((-(-((assign35720_e46954 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign35720_e46965 * assign35720_e46965))), (-((1e-100 * (((-(-((assign35720_e46937 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign35720_e46963) + (assign35720_e46940 * (0.5 * (((-(-((assign35720_e46946 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign35720_e46960) + (assign35720_e46949 * ((-(-((assign35720_e46954 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign35720_e46965 * assign35720_e46965))), (-((1e-100 * (((-(-((assign35720_e46937 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign35720_e46963) + (assign35720_e46940 * (0.5 * (((-(-((assign35720_e46946 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign35720_e46960) + (assign35720_e46949 * ((-(-((assign35720_e46954 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign35720_e46965 * assign35720_e46965))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35720_e46968;
        var_tmp_dn6 = assign35720_e46968_d_n6;
        var_tmp_dn7 = assign35720_e46968_d_n7;
        var_tmp_dn8 = assign35720_e46968_d_n8;
        var_tmp_dn9 = assign35720_e46968_d_n9;

        let (assign35730_e47017, assign35730_e47017_d_n6, assign35730_e47017_d_n7, assign35730_e47017_d_n8, assign35730_e47017_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard731 == 0.0)) && (var_guard733 == 0.0)) && (var_guard734 == 0.0)) {
        let assign35730_e46987: f64 = (-var_fbbtsti_d);
        let assign35730_e46989: f64 = (assign35730_e46987 / var_fmaxr);
        let assign35730_e46991: f64 = (assign35730_e46989 - 230.25850929940458);
        let assign35730_e46995: f64 = (-var_fbbtsti_d);
        let assign35730_e46997: f64 = (assign35730_e46995 / var_fmaxr);
        let assign35730_e46999: f64 = (assign35730_e46997 - 230.25850929940458);
        let assign35730_e47002: f64 = (-var_fbbtsti_d);
        let assign35730_e47004: f64 = (assign35730_e47002 / var_fmaxr);
        let assign35730_e47006: f64 = (assign35730_e47004 - 230.25850929940458);
        let assign35730_e47008: f64 = (assign35730_e47006 * 0.3333333333333333);
        let assign35730_e47009: f64 = (1.0 + assign35730_e47008);
        let assign35730_e47010: f64 = (assign35730_e46999 * assign35730_e47009);
        let assign35730_e47011: f64 = (0.5 * assign35730_e47010);
        let assign35730_e47012: f64 = (1.0 + assign35730_e47011);
        let assign35730_e47013: f64 = (assign35730_e46991 * assign35730_e47012);
        let assign35730_e47014: f64 = (1.0 + assign35730_e47013);
        let assign35730_e47015: f64 = (1e100 * assign35730_e47014);
        (assign35730_e47015, (1e100 * (((-((assign35730_e46987 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign35730_e47012) + (assign35730_e46991 * (0.5 * (((-((assign35730_e46995 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign35730_e47009) + (assign35730_e46999 * ((-((assign35730_e47002 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign35730_e46987 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign35730_e47012) + (assign35730_e46991 * (0.5 * (((-((assign35730_e46995 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign35730_e47009) + (assign35730_e46999 * ((-((assign35730_e47002 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign35730_e46987 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign35730_e47012) + (assign35730_e46991 * (0.5 * (((-((assign35730_e46995 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign35730_e47009) + (assign35730_e46999 * ((-((assign35730_e47002 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign35730_e46987 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign35730_e47012) + (assign35730_e46991 * (0.5 * (((-((assign35730_e46995 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign35730_e47009) + (assign35730_e46999 * ((-((assign35730_e47002 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35730_e47017;
        var_tmp_dn6 = assign35730_e47017_d_n6;
        var_tmp_dn7 = assign35730_e47017_d_n7;
        var_tmp_dn8 = assign35730_e47017_d_n8;
        var_tmp_dn9 = assign35730_e47017_d_n9;

        let (assign35740_e47037, assign35740_e47037_d_n6, assign35740_e47037_d_n7, assign35740_e47037_d_n8, assign35740_e47037_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard731 == 0.0)) {
        let assign35740_e47030: f64 = (var_v4 * var_fmaxr);
        let assign35740_e47032: f64 = (assign35740_e47030 * var_fmaxr);
        let assign35740_e47034: f64 = (assign35740_e47032 * var_tmp);
        let assign35740_e47035: f64 = (var_cbbtstid_i * assign35740_e47034);
        (assign35740_e47035, (var_cbbtstid_i * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign35740_e47030 * var_fmaxr_dn6)) * var_tmp) + (assign35740_e47032 * var_tmp_dn6))), (var_cbbtstid_i * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign35740_e47030 * var_fmaxr_dn7)) * var_tmp) + (assign35740_e47032 * var_tmp_dn7))), (var_cbbtstid_i * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign35740_e47030 * var_fmaxr_dn8)) * var_tmp) + (assign35740_e47032 * var_tmp_dn8))), (var_cbbtstid_i * (((((var_v4 * var_fmaxr_dn9) * var_fmaxr) + (assign35740_e47030 * var_fmaxr_dn9)) * var_tmp) + (assign35740_e47032 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign35740_e47037;
        var_ibbt_dn6 = assign35740_e47037_d_n6;
        var_ibbt_dn7 = assign35740_e47037_d_n7;
        var_ibbt_dn8 = assign35740_e47037_d_n8;
        var_ibbt_dn9 = assign35740_e47037_d_n9;

        let assign35750_e47040: f64 = if var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard735 = assign35750_e47040;

        let (assign35760_e47051, assign35760_e47051_d_n6, assign35760_e47051_d_n7, assign35760_e47051_d_n8, assign35760_e47051_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard735 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign35760_e47051;
        var_fbreakdown_dn6 = assign35760_e47051_d_n6;
        var_fbreakdown_dn7 = assign35760_e47051_d_n7;
        var_fbreakdown_dn8 = assign35760_e47051_d_n8;
        var_fbreakdown_dn9 = assign35760_e47051_d_n9;

        let assign35770_e47054: f64 = (-var_alphaav);
        let assign35770_e47056: f64 = (assign35770_e47054 * var_vbrstid_i);
        let assign35770_e47057: f64 = if var_vav > assign35770_e47056 { 1.0 } else { 0.0 };
        var_guard736 = assign35770_e47057;

        let assign35780_e47060: f64 = if var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        var_guard737 = assign35780_e47060;

        let (assign35790_e47090, assign35790_e47090_d_n6, assign35790_e47090_d_n7, assign35790_e47090_d_n8, assign35790_e47090_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard735 == 0.0)) && (var_guard736 != 0.0)) && (var_guard737 != 0.0)) {
        let assign35790_e47076: f64 = (var_vav * var_vbrinvsti_d);
        let assign35790_e47079: f64 = (var_vav * var_vbrinvsti_d);
        let assign35790_e47080: f64 = (assign35790_e47076 * assign35790_e47079);
        let assign35790_e47083: f64 = (var_vav * var_vbrinvsti_d);
        let assign35790_e47084: f64 = (assign35790_e47080 * assign35790_e47083);
        let assign35790_e47087: f64 = (var_vav * var_vbrinvsti_d);
        let assign35790_e47088: f64 = (assign35790_e47084 * assign35790_e47087);
        (assign35790_e47088, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35790_e47090;
        var_tmp_dn6 = assign35790_e47090_d_n6;
        var_tmp_dn7 = assign35790_e47090_d_n7;
        var_tmp_dn8 = assign35790_e47090_d_n8;
        var_tmp_dn9 = assign35790_e47090_d_n9;

        let (assign35800_e47112, assign35800_e47112_d_n6, assign35800_e47112_d_n7, assign35800_e47112_d_n8, assign35800_e47112_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard735 == 0.0)) && (var_guard736 != 0.0)) && (var_guard737 == 0.0)) {
        let assign35800_e47107: f64 = (var_vav * var_vbrinvsti_d);
        let assign35800_e47108: f64 = (assign35800_e47107).abs();
        let assign35800_e47110: f64 = (assign35800_e47108).powf(var_pbrstid_i);
        (assign35800_e47110, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35800_e47112;
        var_tmp_dn6 = assign35800_e47112_d_n6;
        var_tmp_dn7 = assign35800_e47112_d_n7;
        var_tmp_dn8 = assign35800_e47112_d_n8;
        var_tmp_dn9 = assign35800_e47112_d_n9;

        let (assign35810_e47130, assign35810_e47130_d_n6, assign35810_e47130_d_n7, assign35810_e47130_d_n8, assign35810_e47130_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard735 == 0.0)) && (var_guard736 != 0.0)) {
        let assign35810_e47127: f64 = (1.0 - var_tmp);
        let assign35810_e47128: f64 = (1.0 / assign35810_e47127);
        (assign35810_e47128, (-((-var_tmp_dn6) / (assign35810_e47127 * assign35810_e47127))), (-((-var_tmp_dn7) / (assign35810_e47127 * assign35810_e47127))), (-((-var_tmp_dn8) / (assign35810_e47127 * assign35810_e47127))), (-((-var_tmp_dn9) / (assign35810_e47127 * assign35810_e47127))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign35810_e47130;
        var_fbreakdown_dn6 = assign35810_e47130_d_n6;
        var_fbreakdown_dn7 = assign35810_e47130_d_n7;
        var_fbreakdown_dn8 = assign35810_e47130_d_n8;
        var_fbreakdown_dn9 = assign35810_e47130_d_n9;

        let (assign35820_e47153, assign35820_e47153_d_n6, assign35820_e47153_d_n7, assign35820_e47153_d_n8, assign35820_e47153_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) && (var_guard735 == 0.0)) && (var_guard736 == 0.0)) {
        let assign35820_e47147: f64 = (var_alphaav * var_vbrstid_i);
        let assign35820_e47148: f64 = (var_vav + assign35820_e47147);
        let assign35820_e47150: f64 = (assign35820_e47148 * var_slopesti_d);
        let assign35820_e47151: f64 = (var_fstopsti_d + assign35820_e47150);
        (assign35820_e47151, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign35820_e47153;
        var_fbreakdown_dn6 = assign35820_e47153_d_n6;
        var_fbreakdown_dn7 = assign35820_e47153_d_n7;
        var_fbreakdown_dn8 = assign35820_e47153_d_n8;
        var_fbreakdown_dn9 = assign35820_e47153_d_n9;

        let (assign35830_e47172, assign35830_e47172_d_n6, assign35830_e47172_d_n7, assign35830_e47172_d_n8, assign35830_e47172_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard721 == 0.0)) {
        let assign35830_e47163: f64 = (var_id__blk212 + var_isrh);
        let assign35830_e47165: f64 = (assign35830_e47163 + var_itat);
        let assign35830_e47167: f64 = (assign35830_e47165 + var_ibbt);
        let assign35830_e47168: f64 = (p.p29 * assign35830_e47167);
        let assign35830_e47170: f64 = (assign35830_e47168 * var_fbreakdown);
        (assign35830_e47170, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign35830_e47168 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign35830_e47168 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign35830_e47168 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign35830_e47168 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign35830_e47172;
        var_ijunsti_dn6 = assign35830_e47172_d_n6;
        var_ijunsti_dn7 = assign35830_e47172_d_n7;
        var_ijunsti_dn8 = assign35830_e47172_d_n8;
        var_ijunsti_dn9 = assign35830_e47172_d_n9;

        let assign35840_e47175: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard738 = assign35840_e47175;

        let (assign35850_e47183, assign35850_e47183_d_n6, assign35850_e47183_d_n7, assign35850_e47183_d_n8, assign35850_e47183_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign35850_e47183;
        var_ijungat_dn6 = assign35850_e47183_d_n6;
        var_ijungat_dn7 = assign35850_e47183_d_n7;
        var_ijungat_dn8 = assign35850_e47183_d_n8;
        var_ijungat_dn9 = assign35850_e47183_d_n9;

        let (assign35860_e47194,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) {
        let assign35860_e47192: f64 = (var_idsatgat_d * var_idmult);
        (assign35860_e47192,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign35860_e47194;

        let assign35870_e47201: f64 = if ((var_csrhgatd_i == 0.0) && (var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard739 = assign35870_e47201;

        let (assign35880_e47212, assign35880_e47212_d_n6, assign35880_e47212_d_n7, assign35880_e47212_d_n8, assign35880_e47212_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard739 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign35880_e47212;
        var_isrh_dn6 = assign35880_e47212_d_n6;
        var_isrh_dn7 = assign35880_e47212_d_n7;
        var_isrh_dn8 = assign35880_e47212_d_n8;
        var_isrh_dn9 = assign35880_e47212_d_n9;

        let (assign35890_e47226,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard739 == 0.0)) {
        let assign35890_e47224: f64 = (var_vbigat_d - var_vjsrh);
        (assign35890_e47224,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign35890_e47226;

        let (assign35900_e47245,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard739 == 0.0)) {
        let assign35900_e47240: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign35900_e47241: f64 = (1.0 - assign35900_e47240);
        let assign35900_e47242: f64 = (assign35900_e47241).sqrt();
        let assign35900_e47243: f64 = (1.0 - assign35900_e47242);
        (assign35900_e47243,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign35900_e47245;

        let assign35910_e47248: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard740 = assign35910_e47248;

        let (assign35920_e47262,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard739 == 0.0)) && (var_guard740 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign35920_e47262;

        let (assign35930_e47294,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard739 == 0.0)) && (var_guard740 == 0.0)) {
        let assign35930_e47277: f64 = (var_wsrhstep * var_wsrhstep);
        let assign35930_e47279: f64 = (var_wsrhstep).ln();
        let assign35930_e47280: f64 = (assign35930_e47277 * assign35930_e47279);
        let assign35930_e47283: f64 = (1.0 - var_wsrhstep);
        let assign35930_e47284: f64 = (assign35930_e47280 / assign35930_e47283);
        let assign35930_e47286: f64 = (assign35930_e47284 + var_wsrhstep);
        let assign35930_e47290: f64 = (2.0 * var_pgatd_i);
        let assign35930_e47291: f64 = (1.0 - assign35930_e47290);
        let assign35930_e47292: f64 = (assign35930_e47286 * assign35930_e47291);
        (assign35930_e47292,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign35930_e47294;

        let (assign35940_e47308,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard739 == 0.0)) {
        let assign35940_e47306: f64 = (var_wsrhstep + var_dwsrh);
        (assign35940_e47306,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign35940_e47308;

        let assign35950_e47311: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard741 = assign35950_e47311;

        let (assign35960_e47328, assign35960_e47328_d_n6, assign35960_e47328_d_n7, assign35960_e47328_d_n8, assign35960_e47328_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard739 == 0.0)) && (var_guard741 != 0.0)) {
        let assign35960_e47325: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign35960_e47326: f64 = (assign35960_e47325).sqrt();
        (assign35960_e47326, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35960_e47328;
        var_tmp_dn6 = assign35960_e47328_d_n6;
        var_tmp_dn7 = assign35960_e47328_d_n7;
        var_tmp_dn8 = assign35960_e47328_d_n8;
        var_tmp_dn9 = assign35960_e47328_d_n9;

        let (assign35970_e47347, assign35970_e47347_d_n6, assign35970_e47347_d_n7, assign35970_e47347_d_n8, assign35970_e47347_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard739 == 0.0)) && (var_guard741 == 0.0)) {
        let assign35970_e47343: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign35970_e47345: f64 = (assign35970_e47343).powf(var_pgatd_i);
        (assign35970_e47345, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign35970_e47347;
        var_tmp_dn6 = assign35970_e47347_d_n6;
        var_tmp_dn7 = assign35970_e47347_d_n7;
        var_tmp_dn8 = assign35970_e47347_d_n8;
        var_tmp_dn9 = assign35970_e47347_d_n9;

        let (assign35980_e47361, assign35980_e47361_d_n6, assign35980_e47361_d_n7, assign35980_e47361_d_n8, assign35980_e47361_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard739 == 0.0)) {
        let assign35980_e47359: f64 = (var_wdepnulrgat_d * var_tmp);
        (assign35980_e47359, (var_wdepnulrgat_d * var_tmp_dn6), (var_wdepnulrgat_d * var_tmp_dn7), (var_wdepnulrgat_d * var_tmp_dn8), (var_wdepnulrgat_d * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign35980_e47361;
        var_wdep_dn6 = assign35980_e47361_d_n6;
        var_wdep_dn7 = assign35980_e47361_d_n7;
        var_wdep_dn8 = assign35980_e47361_d_n8;
        var_wdep_dn9 = assign35980_e47361_d_n9;

        let (assign35990_e47379, assign35990_e47379_d_n6, assign35990_e47379_d_n7, assign35990_e47379_d_n8, assign35990_e47379_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard739 == 0.0)) {
        let assign35990_e47374: f64 = (var_zinv - 1.0);
        let assign35990_e47376: f64 = (assign35990_e47374 * var_wdep);
        let assign35990_e47377: f64 = (var_ftdgat_d * assign35990_e47376);
        (assign35990_e47377, (var_ftdgat_d * (assign35990_e47374 * var_wdep_dn6)), (var_ftdgat_d * (assign35990_e47374 * var_wdep_dn7)), (var_ftdgat_d * (assign35990_e47374 * var_wdep_dn8)), (var_ftdgat_d * (assign35990_e47374 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign35990_e47379;
        var_asrh_dn6 = assign35990_e47379_d_n6;
        var_asrh_dn7 = assign35990_e47379_d_n7;
        var_asrh_dn8 = assign35990_e47379_d_n8;
        var_asrh_dn9 = assign35990_e47379_d_n9;

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
        *var_guard741_slot = var_guard741;
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

    pub(super) fn stamp_transient_block_76(
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatgat_d: f64,
        var_berfc: f64,
        var_btatpartgat_d: f64,
        var_cbbtgatd_i: f64,
        var_cerfc: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard738: f64,
        var_guard739: f64,
        var_one_minus_pgat_d: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_perfc: f64,
        var_pgatd_i: f64,
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
        var_guard742_slot: &mut f64,
        var_guard743_slot: &mut f64,
        var_guard744_slot: &mut f64,
        var_guard745_slot: &mut f64,
        var_guard746_slot: &mut f64,
        var_guard747_slot: &mut f64,
        var_guard748_slot: &mut f64,
        var_guard749_slot: &mut f64,
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
        let mut var_guard742: f64 = *var_guard742_slot;
        let mut var_guard743: f64 = *var_guard743_slot;
        let mut var_guard744: f64 = *var_guard744_slot;
        let mut var_guard745: f64 = *var_guard745_slot;
        let mut var_guard746: f64 = *var_guard746_slot;
        let mut var_guard747: f64 = *var_guard747_slot;
        let mut var_guard748: f64 = *var_guard748_slot;
        let mut var_guard749: f64 = *var_guard749_slot;
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

        let (assign36000_e47395, assign36000_e47395_d_n6, assign36000_e47395_d_n7, assign36000_e47395_d_n8, assign36000_e47395_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard739 == 0.0)) {
        let assign36000_e47392: f64 = (var_asrh * var_wsrh);
        let assign36000_e47393: f64 = (var_csrhgatd_i * assign36000_e47392);
        (assign36000_e47393, (var_csrhgatd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn8 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign36000_e47395;
        var_isrh_dn6 = assign36000_e47395_d_n6;
        var_isrh_dn7 = assign36000_e47395_d_n7;
        var_isrh_dn8 = assign36000_e47395_d_n8;
        var_isrh_dn9 = assign36000_e47395_d_n9;

        let assign36010_e47398: f64 = if var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard742 = assign36010_e47398;

        let (assign36020_e47409, assign36020_e47409_d_n6, assign36020_e47409_d_n7, assign36020_e47409_d_n8, assign36020_e47409_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign36020_e47409;
        var_itat_dn6 = assign36020_e47409_d_n6;
        var_itat_dn7 = assign36020_e47409_d_n7;
        var_itat_dn8 = assign36020_e47409_d_n8;
        var_itat_dn9 = assign36020_e47409_d_n9;

        let (assign36030_e47427, assign36030_e47427_d_n6, assign36030_e47427_d_n7, assign36030_e47427_d_n8, assign36030_e47427_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) {
        let assign36030_e47422: f64 = (var_wdep * var_one_minus_pgat_d);
        let assign36030_e47424: f64 = (assign36030_e47422 / var_vbi_minus_vjsrh);
        let assign36030_e47425: f64 = (var_btatpartgat_d * assign36030_e47424);
        (assign36030_e47425, (var_btatpartgat_d * ((var_wdep_dn6 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn7 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn8 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn9 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign36030_e47427;
        var_btat_dn6 = assign36030_e47427_d_n6;
        var_btat_dn7 = assign36030_e47427_d_n7;
        var_btat_dn8 = assign36030_e47427_d_n8;
        var_btat_dn9 = assign36030_e47427_d_n9;

        let (assign36040_e47443, assign36040_e47443_d_n6, assign36040_e47443_d_n7, assign36040_e47443_d_n8, assign36040_e47443_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) {
        let assign36040_e47439: f64 = (0.666666666666667 * var_atatgat_d);
        let assign36040_e47441: f64 = (assign36040_e47439 / var_btat);
        (assign36040_e47441, (-((assign36040_e47439 * var_btat_dn6) / (var_btat * var_btat))), (-((assign36040_e47439 * var_btat_dn7) / (var_btat * var_btat))), (-((assign36040_e47439 * var_btat_dn8) / (var_btat * var_btat))), (-((assign36040_e47439 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign36040_e47443;
        var_twoatatoverthreebtat_dn6 = assign36040_e47443_d_n6;
        var_twoatatoverthreebtat_dn7 = assign36040_e47443_d_n7;
        var_twoatatoverthreebtat_dn8 = assign36040_e47443_d_n8;
        var_twoatatoverthreebtat_dn9 = assign36040_e47443_d_n9;

        let (assign36050_e47457, assign36050_e47457_d_n6, assign36050_e47457_d_n7, assign36050_e47457_d_n8, assign36050_e47457_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) {
        let assign36050_e47455: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign36050_e47455, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign36050_e47457;
        var_umaxbeforelimiting_dn6 = assign36050_e47457_d_n6;
        var_umaxbeforelimiting_dn7 = assign36050_e47457_d_n7;
        var_umaxbeforelimiting_dn8 = assign36050_e47457_d_n8;
        var_umaxbeforelimiting_dn9 = assign36050_e47457_d_n9;

        let (assign36060_e47478, assign36060_e47478_d_n6, assign36060_e47478_d_n7, assign36060_e47478_d_n8, assign36060_e47478_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) {
        let assign36060_e47469: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign36060_e47472: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign36060_e47474: f64 = (assign36060_e47472 + 1.0);
        let assign36060_e47475: f64 = (assign36060_e47469 / assign36060_e47474);
        let assign36060_e47476: f64 = (assign36060_e47475).sqrt();
        (assign36060_e47476, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign36060_e47474) - (assign36060_e47469 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign36060_e47474 * assign36060_e47474)) / (2.0 * assign36060_e47476)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign36060_e47474) - (assign36060_e47469 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign36060_e47474 * assign36060_e47474)) / (2.0 * assign36060_e47476)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign36060_e47474) - (assign36060_e47469 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign36060_e47474 * assign36060_e47474)) / (2.0 * assign36060_e47476)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign36060_e47474) - (assign36060_e47469 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign36060_e47474 * assign36060_e47474)) / (2.0 * assign36060_e47476)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign36060_e47478;
        var_umax_dn6 = assign36060_e47478_d_n6;
        var_umax_dn7 = assign36060_e47478_d_n7;
        var_umax_dn8 = assign36060_e47478_d_n8;
        var_umax_dn9 = assign36060_e47478_d_n9;

        let (assign36070_e47491, assign36070_e47491_d_n6, assign36070_e47491_d_n7, assign36070_e47491_d_n8, assign36070_e47491_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) {
        let assign36070_e47489: f64 = (var_umax).sqrt();
        (assign36070_e47489, (var_umax_dn6 / (2.0 * assign36070_e47489)), (var_umax_dn7 / (2.0 * assign36070_e47489)), (var_umax_dn8 / (2.0 * assign36070_e47489)), (var_umax_dn9 / (2.0 * assign36070_e47489)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign36070_e47491;
        var_sqrtumax_dn6 = assign36070_e47491_d_n6;
        var_sqrtumax_dn7 = assign36070_e47491_d_n7;
        var_sqrtumax_dn8 = assign36070_e47491_d_n8;
        var_sqrtumax_dn9 = assign36070_e47491_d_n9;

        let (assign36080_e47505, assign36080_e47505_d_n6, assign36080_e47505_d_n7, assign36080_e47505_d_n8, assign36080_e47505_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) {
        let assign36080_e47503: f64 = (var_umax * var_sqrtumax);
        (assign36080_e47503, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign36080_e47505;
        var_umaxpoweronepointfive_dn6 = assign36080_e47505_d_n6;
        var_umaxpoweronepointfive_dn7 = assign36080_e47505_d_n7;
        var_umaxpoweronepointfive_dn8 = assign36080_e47505_d_n8;
        var_umaxpoweronepointfive_dn9 = assign36080_e47505_d_n9;

        let assign36090_e47507: f64 = (-var_pgatd_i);
        let assign36090_e47509: f64 = (assign36090_e47507 * var_one_over_one_minus_pgat_d);
        let assign36090_e47511: f64 = (-1.0);
        let assign36090_e47512: f64 = if assign36090_e47509 == assign36090_e47511 { 1.0 } else { 0.0 };
        var_guard743 = assign36090_e47512;

        let (assign36100_e47532, assign36100_e47532_d_n6, assign36100_e47532_d_n7, assign36100_e47532_d_n8, assign36100_e47532_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) && (var_guard743 != 0.0)) {
        let assign36100_e47528: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign36100_e47529: f64 = (1.0 + assign36100_e47528);
        let assign36100_e47530: f64 = (1.0 / assign36100_e47529);
        (assign36100_e47530, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign36100_e47529 * assign36100_e47529))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign36100_e47529 * assign36100_e47529))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign36100_e47529 * assign36100_e47529))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign36100_e47529 * assign36100_e47529))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign36100_e47532;
        var_wgamma_dn6 = assign36100_e47532_d_n6;
        var_wgamma_dn7 = assign36100_e47532_d_n7;
        var_wgamma_dn8 = assign36100_e47532_d_n8;
        var_wgamma_dn9 = assign36100_e47532_d_n9;

        let (assign36110_e47556, assign36110_e47556_d_n6, assign36110_e47556_d_n7, assign36110_e47556_d_n8, assign36110_e47556_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) && (var_guard743 == 0.0)) {
        let assign36110_e47548: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign36110_e47549: f64 = (1.0 + assign36110_e47548);
        let assign36110_e47551: f64 = (-var_pgatd_i);
        let assign36110_e47553: f64 = (assign36110_e47551 * var_one_over_one_minus_pgat_d);
        let assign36110_e47554: f64 = (assign36110_e47549).powf(assign36110_e47553);
        (assign36110_e47554, if 0.0 == 0.0 && ((assign36110_e47553) as f64).is_finite() && ((assign36110_e47553) as f64).fract() == 0.0 { if assign36110_e47553 == 0.0 { 0.0 } else { (assign36110_e47553 * ((assign36110_e47549).powf(assign36110_e47553 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign36110_e47554 * (assign36110_e47553 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign36110_e47549))) }, if 0.0 == 0.0 && ((assign36110_e47553) as f64).is_finite() && ((assign36110_e47553) as f64).fract() == 0.0 { if assign36110_e47553 == 0.0 { 0.0 } else { (assign36110_e47553 * ((assign36110_e47549).powf(assign36110_e47553 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign36110_e47554 * (assign36110_e47553 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign36110_e47549))) }, if 0.0 == 0.0 && ((assign36110_e47553) as f64).is_finite() && ((assign36110_e47553) as f64).fract() == 0.0 { if assign36110_e47553 == 0.0 { 0.0 } else { (assign36110_e47553 * ((assign36110_e47549).powf(assign36110_e47553 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign36110_e47554 * (assign36110_e47553 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign36110_e47549))) }, if 0.0 == 0.0 && ((assign36110_e47553) as f64).is_finite() && ((assign36110_e47553) as f64).fract() == 0.0 { if assign36110_e47553 == 0.0 { 0.0 } else { (assign36110_e47553 * ((assign36110_e47549).powf(assign36110_e47553 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign36110_e47554 * (assign36110_e47553 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign36110_e47549))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign36110_e47556;
        var_wgamma_dn6 = assign36110_e47556_d_n6;
        var_wgamma_dn7 = assign36110_e47556_d_n7;
        var_wgamma_dn8 = assign36110_e47556_d_n8;
        var_wgamma_dn9 = assign36110_e47556_d_n9;

        let (assign36120_e47574, assign36120_e47574_d_n6, assign36120_e47574_d_n7, assign36120_e47574_d_n8, assign36120_e47574_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) {
        let assign36120_e47568: f64 = (var_wsrh * var_wgamma);
        let assign36120_e47571: f64 = (var_wsrh + var_wgamma);
        let assign36120_e47572: f64 = (assign36120_e47568 / assign36120_e47571);
        (assign36120_e47572, ((((var_wsrh * var_wgamma_dn6) * assign36120_e47571) - (assign36120_e47568 * var_wgamma_dn6)) / (assign36120_e47571 * assign36120_e47571)), ((((var_wsrh * var_wgamma_dn7) * assign36120_e47571) - (assign36120_e47568 * var_wgamma_dn7)) / (assign36120_e47571 * assign36120_e47571)), ((((var_wsrh * var_wgamma_dn8) * assign36120_e47571) - (assign36120_e47568 * var_wgamma_dn8)) / (assign36120_e47571 * assign36120_e47571)), ((((var_wsrh * var_wgamma_dn9) * assign36120_e47571) - (assign36120_e47568 * var_wgamma_dn9)) / (assign36120_e47571 * assign36120_e47571)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign36120_e47574;
        var_wtat_dn6 = assign36120_e47574_d_n6;
        var_wtat_dn7 = assign36120_e47574_d_n7;
        var_wtat_dn8 = assign36120_e47574_d_n8;
        var_wtat_dn9 = assign36120_e47574_d_n9;

        let (assign36130_e47591, assign36130_e47591_d_n6, assign36130_e47591_d_n7, assign36130_e47591_d_n8, assign36130_e47591_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) {
        let assign36130_e47587: f64 = (var_btat / var_sqrtumax);
        let assign36130_e47588: f64 = (0.375 * assign36130_e47587);
        let assign36130_e47589: f64 = (assign36130_e47588).sqrt();
        (assign36130_e47589, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign36130_e47589)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign36130_e47589)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign36130_e47589)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign36130_e47589)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign36130_e47591;
        var_ktat_dn6 = assign36130_e47591_d_n6;
        var_ktat_dn7 = assign36130_e47591_d_n7;
        var_ktat_dn8 = assign36130_e47591_d_n8;
        var_ktat_dn9 = assign36130_e47591_d_n9;

        let (assign36140_e47609, assign36140_e47609_d_n6, assign36140_e47609_d_n7, assign36140_e47609_d_n8, assign36140_e47609_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) {
        let assign36140_e47604: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign36140_e47605: f64 = (2.0 * assign36140_e47604);
        let assign36140_e47607: f64 = (assign36140_e47605 - var_umax);
        (assign36140_e47607, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign36140_e47609;
        var_ltat_dn6 = assign36140_e47609_d_n6;
        var_ltat_dn7 = assign36140_e47609_d_n7;
        var_ltat_dn8 = assign36140_e47609_d_n8;
        var_ltat_dn9 = assign36140_e47609_d_n9;

        let (assign36150_e47635, assign36150_e47635_d_n6, assign36150_e47635_d_n7, assign36150_e47635_d_n8, assign36150_e47635_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) {
        let assign36150_e47621: f64 = (var_atatgat_d * var_twoatatoverthreebtat);
        let assign36150_e47623: f64 = (assign36150_e47621 * var_sqrtumax);
        let assign36150_e47626: f64 = (var_atatgat_d * var_umax);
        let assign36150_e47627: f64 = (assign36150_e47623 - assign36150_e47626);
        let assign36150_e47631: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign36150_e47632: f64 = (0.5 * assign36150_e47631);
        let assign36150_e47633: f64 = (assign36150_e47627 + assign36150_e47632);
        (assign36150_e47633, (((((var_atatgat_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign36150_e47621 * var_sqrtumax_dn6)) - (var_atatgat_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign36150_e47621 * var_sqrtumax_dn7)) - (var_atatgat_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign36150_e47621 * var_sqrtumax_dn8)) - (var_atatgat_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign36150_e47621 * var_sqrtumax_dn9)) - (var_atatgat_d * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign36150_e47635;
        var_mtat_dn6 = assign36150_e47635_d_n6;
        var_mtat_dn7 = assign36150_e47635_d_n7;
        var_mtat_dn8 = assign36150_e47635_d_n8;
        var_mtat_dn9 = assign36150_e47635_d_n9;

        let (assign36160_e47651, assign36160_e47651_d_n6, assign36160_e47651_d_n7, assign36160_e47651_d_n8, assign36160_e47651_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) {
        let assign36160_e47647: f64 = (var_ltat - 1.0);
        let assign36160_e47649: f64 = (assign36160_e47647 * var_ktat);
        (assign36160_e47649, ((var_ltat_dn6 * var_ktat) + (assign36160_e47647 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign36160_e47647 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign36160_e47647 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign36160_e47647 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign36160_e47651;
        var_xerfc_dn6 = assign36160_e47651_d_n6;
        var_xerfc_dn7 = assign36160_e47651_d_n7;
        var_xerfc_dn8 = assign36160_e47651_d_n8;
        var_xerfc_dn9 = assign36160_e47651_d_n9;

        let (assign36170_e47665, assign36170_e47665_d_n6, assign36170_e47665_d_n7, assign36170_e47665_d_n8, assign36170_e47665_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) {
        let assign36170_e47663: f64 = (var_xerfc * var_xerfc);
        (assign36170_e47663, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign36170_e47665;
        var_ysq_dn6 = assign36170_e47665_d_n6;
        var_ysq_dn7 = assign36170_e47665_d_n7;
        var_ysq_dn8 = assign36170_e47665_d_n8;
        var_ysq_dn9 = assign36170_e47665_d_n9;

        let assign36180_e47668: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard744 = assign36180_e47668;

        let (assign36190_e47688, assign36190_e47688_d_n6, assign36190_e47688_d_n7, assign36190_e47688_d_n8, assign36190_e47688_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) && (var_guard744 != 0.0)) {
        let assign36190_e47684: f64 = (var_perfc * var_xerfc);
        let assign36190_e47685: f64 = (1.0 + assign36190_e47684);
        let assign36190_e47686: f64 = (1.0 / assign36190_e47685);
        (assign36190_e47686, (-((var_perfc * var_xerfc_dn6) / (assign36190_e47685 * assign36190_e47685))), (-((var_perfc * var_xerfc_dn7) / (assign36190_e47685 * assign36190_e47685))), (-((var_perfc * var_xerfc_dn8) / (assign36190_e47685 * assign36190_e47685))), (-((var_perfc * var_xerfc_dn9) / (assign36190_e47685 * assign36190_e47685))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign36190_e47688;
        var_terfc_dn6 = assign36190_e47688_d_n6;
        var_terfc_dn7 = assign36190_e47688_d_n7;
        var_terfc_dn8 = assign36190_e47688_d_n8;
        var_terfc_dn9 = assign36190_e47688_d_n9;

        let (assign36200_e47709, assign36200_e47709_d_n6, assign36200_e47709_d_n7, assign36200_e47709_d_n8, assign36200_e47709_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) && (var_guard744 == 0.0)) {
        let assign36200_e47705: f64 = (var_perfc * var_xerfc);
        let assign36200_e47706: f64 = (1.0 - assign36200_e47705);
        let assign36200_e47707: f64 = (1.0 / assign36200_e47706);
        (assign36200_e47707, (-((-(var_perfc * var_xerfc_dn6)) / (assign36200_e47706 * assign36200_e47706))), (-((-(var_perfc * var_xerfc_dn7)) / (assign36200_e47706 * assign36200_e47706))), (-((-(var_perfc * var_xerfc_dn8)) / (assign36200_e47706 * assign36200_e47706))), (-((-(var_perfc * var_xerfc_dn9)) / (assign36200_e47706 * assign36200_e47706))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign36200_e47709;
        var_terfc_dn6 = assign36200_e47709_d_n6;
        var_terfc_dn7 = assign36200_e47709_d_n7;
        var_terfc_dn8 = assign36200_e47709_d_n8;
        var_terfc_dn9 = assign36200_e47709_d_n9;

        let assign36210_e47711: f64 = (-var_ysq);
        let assign36210_e47713: f64 = (assign36210_e47711 + var_mtat);
        let assign36210_e47715: f64 = (-230.25850929940458);
        let assign36210_e47716: f64 = if assign36210_e47713 > assign36210_e47715 { 1.0 } else { 0.0 };
        var_guard745 = assign36210_e47716;

        let (assign36220_e47734, assign36220_e47734_d_n6, assign36220_e47734_d_n7, assign36220_e47734_d_n8, assign36220_e47734_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) && (var_guard745 != 0.0)) {
        let assign36220_e47729: f64 = (-var_ysq);
        let assign36220_e47731: f64 = (assign36220_e47729 + var_mtat);
        let assign36220_e47732: f64 = (assign36220_e47731).exp();
        (assign36220_e47732, (assign36220_e47732 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign36220_e47732 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign36220_e47732 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign36220_e47732 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign36220_e47734;
        var_tmp_dn6 = assign36220_e47734_d_n6;
        var_tmp_dn7 = assign36220_e47734_d_n7;
        var_tmp_dn8 = assign36220_e47734_d_n8;
        var_tmp_dn9 = assign36220_e47734_d_n9;

        let (assign36230_e47783, assign36230_e47783_d_n6, assign36230_e47783_d_n7, assign36230_e47783_d_n8, assign36230_e47783_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) && (var_guard745 == 0.0)) {
        let assign36230_e47750: f64 = (-230.25850929940458);
        let assign36230_e47752: f64 = (-var_ysq);
        let assign36230_e47754: f64 = (assign36230_e47752 + var_mtat);
        let assign36230_e47755: f64 = (assign36230_e47750 - assign36230_e47754);
        let assign36230_e47759: f64 = (-230.25850929940458);
        let assign36230_e47761: f64 = (-var_ysq);
        let assign36230_e47763: f64 = (assign36230_e47761 + var_mtat);
        let assign36230_e47764: f64 = (assign36230_e47759 - assign36230_e47763);
        let assign36230_e47767: f64 = (-230.25850929940458);
        let assign36230_e47769: f64 = (-var_ysq);
        let assign36230_e47771: f64 = (assign36230_e47769 + var_mtat);
        let assign36230_e47772: f64 = (assign36230_e47767 - assign36230_e47771);
        let assign36230_e47774: f64 = (assign36230_e47772 * 0.3333333333333333);
        let assign36230_e47775: f64 = (1.0 + assign36230_e47774);
        let assign36230_e47776: f64 = (assign36230_e47764 * assign36230_e47775);
        let assign36230_e47777: f64 = (0.5 * assign36230_e47776);
        let assign36230_e47778: f64 = (1.0 + assign36230_e47777);
        let assign36230_e47779: f64 = (assign36230_e47755 * assign36230_e47778);
        let assign36230_e47780: f64 = (1.0 + assign36230_e47779);
        let assign36230_e47781: f64 = (1e-100 / assign36230_e47780);
        (assign36230_e47781, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign36230_e47778) + (assign36230_e47755 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign36230_e47775) + (assign36230_e47764 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign36230_e47780 * assign36230_e47780))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign36230_e47778) + (assign36230_e47755 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign36230_e47775) + (assign36230_e47764 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign36230_e47780 * assign36230_e47780))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign36230_e47778) + (assign36230_e47755 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign36230_e47775) + (assign36230_e47764 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign36230_e47780 * assign36230_e47780))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign36230_e47778) + (assign36230_e47755 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign36230_e47775) + (assign36230_e47764 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign36230_e47780 * assign36230_e47780))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign36230_e47783;
        var_tmp_dn6 = assign36230_e47783_d_n6;
        var_tmp_dn7 = assign36230_e47783_d_n7;
        var_tmp_dn8 = assign36230_e47783_d_n8;
        var_tmp_dn9 = assign36230_e47783_d_n9;

        let (assign36240_e47813, assign36240_e47813_d_n6, assign36240_e47813_d_n7, assign36240_e47813_d_n8, assign36240_e47813_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) {
        let assign36240_e47795: f64 = (0.29214664 * var_terfc);
        let assign36240_e47799: f64 = (var_terfc * var_terfc);
        let assign36240_e47800: f64 = (var_berfc * assign36240_e47799);
        let assign36240_e47801: f64 = (assign36240_e47795 + assign36240_e47800);
        let assign36240_e47805: f64 = (var_terfc * var_terfc);
        let assign36240_e47807: f64 = (assign36240_e47805 * var_terfc);
        let assign36240_e47808: f64 = (var_cerfc * assign36240_e47807);
        let assign36240_e47809: f64 = (assign36240_e47801 + assign36240_e47808);
        let assign36240_e47811: f64 = (assign36240_e47809 * var_tmp);
        (assign36240_e47811, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign36240_e47805 * var_terfc_dn6)))) * var_tmp) + (assign36240_e47809 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign36240_e47805 * var_terfc_dn7)))) * var_tmp) + (assign36240_e47809 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign36240_e47805 * var_terfc_dn8)))) * var_tmp) + (assign36240_e47809 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign36240_e47805 * var_terfc_dn9)))) * var_tmp) + (assign36240_e47809 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign36240_e47813;
        var_erfcpos_dn6 = assign36240_e47813_d_n6;
        var_erfcpos_dn7 = assign36240_e47813_d_n7;
        var_erfcpos_dn8 = assign36240_e47813_d_n8;
        var_erfcpos_dn9 = assign36240_e47813_d_n9;

        let assign36250_e47816: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard746 = assign36250_e47816;

        let (assign36260_e47830, assign36260_e47830_d_n6, assign36260_e47830_d_n7, assign36260_e47830_d_n8, assign36260_e47830_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) && (var_guard746 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign36260_e47830;
        var_erfctimesexpmtat_dn6 = assign36260_e47830_d_n6;
        var_erfctimesexpmtat_dn7 = assign36260_e47830_d_n7;
        var_erfctimesexpmtat_dn8 = assign36260_e47830_d_n8;
        var_erfctimesexpmtat_dn9 = assign36260_e47830_d_n9;

        let assign36270_e47833: f64 = (-230.25850929940458);
        let assign36270_e47834: f64 = if var_mtat > assign36270_e47833 { 1.0 } else { 0.0 };
        var_guard747 = assign36270_e47834;

        let (assign36280_e47852, assign36280_e47852_d_n6, assign36280_e47852_d_n7, assign36280_e47852_d_n8, assign36280_e47852_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) && (var_guard746 == 0.0)) && (var_guard747 != 0.0)) {
        let assign36280_e47850: f64 = (var_mtat).exp();
        (assign36280_e47850, (assign36280_e47850 * var_mtat_dn6), (assign36280_e47850 * var_mtat_dn7), (assign36280_e47850 * var_mtat_dn8), (assign36280_e47850 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign36280_e47852;
        var_tmp_dn6 = assign36280_e47852_d_n6;
        var_tmp_dn7 = assign36280_e47852_d_n7;
        var_tmp_dn8 = assign36280_e47852_d_n8;
        var_tmp_dn9 = assign36280_e47852_d_n9;

        let (assign36290_e47895, assign36290_e47895_d_n6, assign36290_e47895_d_n7, assign36290_e47895_d_n8, assign36290_e47895_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) && (var_guard746 == 0.0)) && (var_guard747 == 0.0)) {
        let assign36290_e47871: f64 = (-230.25850929940458);
        let assign36290_e47873: f64 = (assign36290_e47871 - var_mtat);
        let assign36290_e47877: f64 = (-230.25850929940458);
        let assign36290_e47879: f64 = (assign36290_e47877 - var_mtat);
        let assign36290_e47882: f64 = (-230.25850929940458);
        let assign36290_e47884: f64 = (assign36290_e47882 - var_mtat);
        let assign36290_e47886: f64 = (assign36290_e47884 * 0.3333333333333333);
        let assign36290_e47887: f64 = (1.0 + assign36290_e47886);
        let assign36290_e47888: f64 = (assign36290_e47879 * assign36290_e47887);
        let assign36290_e47889: f64 = (0.5 * assign36290_e47888);
        let assign36290_e47890: f64 = (1.0 + assign36290_e47889);
        let assign36290_e47891: f64 = (assign36290_e47873 * assign36290_e47890);
        let assign36290_e47892: f64 = (1.0 + assign36290_e47891);
        let assign36290_e47893: f64 = (1e-100 / assign36290_e47892);
        (assign36290_e47893, (-((1e-100 * (((-var_mtat_dn6) * assign36290_e47890) + (assign36290_e47873 * (0.5 * (((-var_mtat_dn6) * assign36290_e47887) + (assign36290_e47879 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign36290_e47892 * assign36290_e47892))), (-((1e-100 * (((-var_mtat_dn7) * assign36290_e47890) + (assign36290_e47873 * (0.5 * (((-var_mtat_dn7) * assign36290_e47887) + (assign36290_e47879 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign36290_e47892 * assign36290_e47892))), (-((1e-100 * (((-var_mtat_dn8) * assign36290_e47890) + (assign36290_e47873 * (0.5 * (((-var_mtat_dn8) * assign36290_e47887) + (assign36290_e47879 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign36290_e47892 * assign36290_e47892))), (-((1e-100 * (((-var_mtat_dn9) * assign36290_e47890) + (assign36290_e47873 * (0.5 * (((-var_mtat_dn9) * assign36290_e47887) + (assign36290_e47879 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign36290_e47892 * assign36290_e47892))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign36290_e47895;
        var_tmp_dn6 = assign36290_e47895_d_n6;
        var_tmp_dn7 = assign36290_e47895_d_n7;
        var_tmp_dn8 = assign36290_e47895_d_n8;
        var_tmp_dn9 = assign36290_e47895_d_n9;

        let (assign36300_e47914, assign36300_e47914_d_n6, assign36300_e47914_d_n7, assign36300_e47914_d_n8, assign36300_e47914_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) && (var_guard746 == 0.0)) {
        let assign36300_e47910: f64 = (2.0 * var_tmp);
        let assign36300_e47912: f64 = (assign36300_e47910 - var_erfcpos);
        (assign36300_e47912, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign36300_e47914;
        var_erfctimesexpmtat_dn6 = assign36300_e47914_d_n6;
        var_erfctimesexpmtat_dn7 = assign36300_e47914_d_n7;
        var_erfctimesexpmtat_dn8 = assign36300_e47914_d_n8;
        var_erfctimesexpmtat_dn9 = assign36300_e47914_d_n9;

        let (assign36310_e47934, assign36310_e47934_d_n6, assign36310_e47934_d_n7, assign36310_e47934_d_n8, assign36310_e47934_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) {
        let assign36310_e47926: f64 = (1.772453850905516 * 0.5);
        let assign36310_e47929: f64 = (var_atatgat_d * var_erfctimesexpmtat);
        let assign36310_e47931: f64 = (assign36310_e47929 / var_ktat);
        let assign36310_e47932: f64 = (assign36310_e47926 * assign36310_e47931);
        (assign36310_e47932, (assign36310_e47926 * ((((var_atatgat_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign36310_e47929 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign36310_e47926 * ((((var_atatgat_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign36310_e47929 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign36310_e47926 * ((((var_atatgat_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign36310_e47929 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign36310_e47926 * ((((var_atatgat_d * var_erfctimesexpmtat_dn9) * var_ktat) - (assign36310_e47929 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign36310_e47934;
        var_gammamax_dn6 = assign36310_e47934_d_n6;
        var_gammamax_dn7 = assign36310_e47934_d_n7;
        var_gammamax_dn8 = assign36310_e47934_d_n8;
        var_gammamax_dn9 = assign36310_e47934_d_n9;

        let (assign36320_e47952, assign36320_e47952_d_n6, assign36320_e47952_d_n7, assign36320_e47952_d_n8, assign36320_e47952_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard742 == 0.0)) {
        let assign36320_e47947: f64 = (var_asrh * var_gammamax);
        let assign36320_e47949: f64 = (assign36320_e47947 * var_wtat);
        let assign36320_e47950: f64 = (var_ctatgatd_i * assign36320_e47949);
        (assign36320_e47950, (var_ctatgatd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign36320_e47947 * var_wtat_dn6))), (var_ctatgatd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign36320_e47947 * var_wtat_dn7))), (var_ctatgatd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign36320_e47947 * var_wtat_dn8))), (var_ctatgatd_i * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign36320_e47947 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign36320_e47952;
        var_itat_dn6 = assign36320_e47952_d_n6;
        var_itat_dn7 = assign36320_e47952_d_n7;
        var_itat_dn8 = assign36320_e47952_d_n8;
        var_itat_dn9 = assign36320_e47952_d_n9;

        let assign36330_e47955: f64 = if var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard748 = assign36330_e47955;

        let (assign36340_e47966, assign36340_e47966_d_n6, assign36340_e47966_d_n7, assign36340_e47966_d_n8, assign36340_e47966_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard748 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign36340_e47966;
        var_ibbt_dn6 = assign36340_e47966_d_n6;
        var_ibbt_dn7 = assign36340_e47966_d_n7;
        var_ibbt_dn8 = assign36340_e47966_d_n8;
        var_ibbt_dn9 = assign36340_e47966_d_n9;

        let assign36350_e47969: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard749 = assign36350_e47969;

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
        *var_guard742_slot = var_guard742;
        *var_guard743_slot = var_guard743;
        *var_guard744_slot = var_guard744;
        *var_guard745_slot = var_guard745;
        *var_guard746_slot = var_guard746;
        *var_guard747_slot = var_guard747;
        *var_guard748_slot = var_guard748;
        *var_guard749_slot = var_guard749;
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

    pub(super) fn stamp_transient_block_77(
        p: &Parameters,
        var_abdrain_i: f64,
        var_alphaav: f64,
        var_cbbtgatd_i: f64,
        var_exp_vmax_over_phitd_d: f64,
        var_fbbtgat_d: f64,
        var_fbbtgat_d_dn6: f64,
        var_fbbtgat_d_dn7: f64,
        var_fbbtgat_d_dn8: f64,
        var_fbbtgat_d_dn9: f64,
        var_fstopgat_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard738: f64,
        var_guard748: f64,
        var_guard749: f64,
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
        var_lgdrain_i: f64,
        var_lsdrain_i: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_pbrgatd_i: f64,
        var_pgatd_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_slopegat_d: f64,
        var_slopegat_d_dn6: f64,
        var_slopegat_d_dn7: f64,
        var_slopegat_d_dn8: f64,
        var_slopegat_d_dn9: f64,
        var_v4: f64,
        var_v5: f64,
        var_vav: f64,
        var_vbimin_d: f64,
        var_vbirgatd_i: f64,
        var_vbirgatinv_d: f64,
        var_vbrgatd_i: f64,
        var_vbrinvgat_d: f64,
        var_vbrinvgat_d_dn6: f64,
        var_vbrinvgat_d_dn7: f64,
        var_vbrinvgat_d_dn8: f64,
        var_vbrinvgat_d_dn9: f64,
        var_vmax_d: f64,
        var_wdepnulrinvgat_d: f64,
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

        let (assign36360_e47988, assign36360_e47988_d_n6, assign36360_e47988_d_n7, assign36360_e47988_d_n8, assign36360_e47988_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard748 == 0.0)) && (var_guard749 != 0.0)) {
        let assign36360_e47983: f64 = (var_vbirgatd_i - var_vbbt);
        let assign36360_e47985: f64 = (assign36360_e47983 * var_vbirgatinv_d);
        let assign36360_e47986: f64 = (assign36360_e47985).sqrt();
        (assign36360_e47986, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign36360_e47988;
        var_tmp_dn6 = assign36360_e47988_d_n6;
        var_tmp_dn7 = assign36360_e47988_d_n7;
        var_tmp_dn8 = assign36360_e47988_d_n8;
        var_tmp_dn9 = assign36360_e47988_d_n9;

        let (assign36370_e48009, assign36370_e48009_d_n6, assign36370_e48009_d_n7, assign36370_e48009_d_n8, assign36370_e48009_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard748 == 0.0)) && (var_guard749 == 0.0)) {
        let assign36370_e48003: f64 = (var_vbirgatd_i - var_vbbt);
        let assign36370_e48005: f64 = (assign36370_e48003 * var_vbirgatinv_d);
        let assign36370_e48007: f64 = (assign36370_e48005).powf(var_pgatd_i);
        (assign36370_e48007, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign36370_e48009;
        var_tmp_dn6 = assign36370_e48009_d_n6;
        var_tmp_dn7 = assign36370_e48009_d_n7;
        var_tmp_dn8 = assign36370_e48009_d_n8;
        var_tmp_dn9 = assign36370_e48009_d_n9;

        let (assign36380_e48029, assign36380_e48029_d_n6, assign36380_e48029_d_n7, assign36380_e48029_d_n8, assign36380_e48029_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard748 == 0.0)) {
        let assign36380_e48022: f64 = (var_vbirgatd_i - var_vbbt);
        let assign36380_e48024: f64 = (assign36380_e48022 * var_wdepnulrinvgat_d);
        let assign36380_e48026: f64 = (assign36380_e48024 / var_tmp);
        let assign36380_e48027: f64 = (var_one_over_one_minus_pgat_d * assign36380_e48026);
        (assign36380_e48027, (var_one_over_one_minus_pgat_d * (-((assign36380_e48024 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign36380_e48024 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign36380_e48024 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign36380_e48024 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign36380_e48029;
        var_fmaxr_dn6 = assign36380_e48029_d_n6;
        var_fmaxr_dn7 = assign36380_e48029_d_n7;
        var_fmaxr_dn8 = assign36380_e48029_d_n8;
        var_fmaxr_dn9 = assign36380_e48029_d_n9;

        let assign36390_e48031: f64 = (-var_fbbtgat_d);
        let assign36390_e48033: f64 = (assign36390_e48031 / var_fmaxr);
        let assign36390_e48034: f64 = (assign36390_e48033).abs();
        let assign36390_e48036: f64 = if assign36390_e48034 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard750 = assign36390_e48036;

        let (assign36400_e48054, assign36400_e48054_d_n6, assign36400_e48054_d_n7, assign36400_e48054_d_n8, assign36400_e48054_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard748 == 0.0)) && (var_guard750 != 0.0)) {
        let assign36400_e48049: f64 = (-var_fbbtgat_d);
        let assign36400_e48051: f64 = (assign36400_e48049 / var_fmaxr);
        let assign36400_e48052: f64 = (assign36400_e48051).exp();
        (assign36400_e48052, (assign36400_e48052 * ((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign36400_e48049 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign36400_e48052 * ((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign36400_e48049 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign36400_e48052 * ((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign36400_e48049 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))), (assign36400_e48052 * ((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign36400_e48049 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign36400_e48054;
        var_tmp_dn6 = assign36400_e48054_d_n6;
        var_tmp_dn7 = assign36400_e48054_d_n7;
        var_tmp_dn8 = assign36400_e48054_d_n8;
        var_tmp_dn9 = assign36400_e48054_d_n9;

        let assign36410_e48056: f64 = (-var_fbbtgat_d);
        let assign36410_e48058: f64 = (assign36410_e48056 / var_fmaxr);
        let assign36410_e48060: f64 = if assign36410_e48058 < 0.0 { 1.0 } else { 0.0 };
        var_guard751 = assign36410_e48060;

        let (assign36420_e48111, assign36420_e48111_d_n6, assign36420_e48111_d_n7, assign36420_e48111_d_n8, assign36420_e48111_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard748 == 0.0)) && (var_guard750 == 0.0)) && (var_guard751 != 0.0)) {
        let assign36420_e48078: f64 = (-230.25850929940458);
        let assign36420_e48080: f64 = (-var_fbbtgat_d);
        let assign36420_e48082: f64 = (assign36420_e48080 / var_fmaxr);
        let assign36420_e48083: f64 = (assign36420_e48078 - assign36420_e48082);
        let assign36420_e48087: f64 = (-230.25850929940458);
        let assign36420_e48089: f64 = (-var_fbbtgat_d);
        let assign36420_e48091: f64 = (assign36420_e48089 / var_fmaxr);
        let assign36420_e48092: f64 = (assign36420_e48087 - assign36420_e48091);
        let assign36420_e48095: f64 = (-230.25850929940458);
        let assign36420_e48097: f64 = (-var_fbbtgat_d);
        let assign36420_e48099: f64 = (assign36420_e48097 / var_fmaxr);
        let assign36420_e48100: f64 = (assign36420_e48095 - assign36420_e48099);
        let assign36420_e48102: f64 = (assign36420_e48100 * 0.3333333333333333);
        let assign36420_e48103: f64 = (1.0 + assign36420_e48102);
        let assign36420_e48104: f64 = (assign36420_e48092 * assign36420_e48103);
        let assign36420_e48105: f64 = (0.5 * assign36420_e48104);
        let assign36420_e48106: f64 = (1.0 + assign36420_e48105);
        let assign36420_e48107: f64 = (assign36420_e48083 * assign36420_e48106);
        let assign36420_e48108: f64 = (1.0 + assign36420_e48107);
        let assign36420_e48109: f64 = (1e-100 / assign36420_e48108);
        (assign36420_e48109, (-((1e-100 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign36420_e48080 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign36420_e48106) + (assign36420_e48083 * (0.5 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign36420_e48089 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign36420_e48103) + (assign36420_e48092 * ((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign36420_e48097 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign36420_e48108 * assign36420_e48108))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign36420_e48080 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign36420_e48106) + (assign36420_e48083 * (0.5 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign36420_e48089 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign36420_e48103) + (assign36420_e48092 * ((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign36420_e48097 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign36420_e48108 * assign36420_e48108))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign36420_e48080 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign36420_e48106) + (assign36420_e48083 * (0.5 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign36420_e48089 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign36420_e48103) + (assign36420_e48092 * ((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign36420_e48097 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign36420_e48108 * assign36420_e48108))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign36420_e48080 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign36420_e48106) + (assign36420_e48083 * (0.5 * (((-((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign36420_e48089 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign36420_e48103) + (assign36420_e48092 * ((-((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign36420_e48097 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign36420_e48108 * assign36420_e48108))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign36420_e48111;
        var_tmp_dn6 = assign36420_e48111_d_n6;
        var_tmp_dn7 = assign36420_e48111_d_n7;
        var_tmp_dn8 = assign36420_e48111_d_n8;
        var_tmp_dn9 = assign36420_e48111_d_n9;

        let (assign36430_e48160, assign36430_e48160_d_n6, assign36430_e48160_d_n7, assign36430_e48160_d_n8, assign36430_e48160_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard748 == 0.0)) && (var_guard750 == 0.0)) && (var_guard751 == 0.0)) {
        let assign36430_e48130: f64 = (-var_fbbtgat_d);
        let assign36430_e48132: f64 = (assign36430_e48130 / var_fmaxr);
        let assign36430_e48134: f64 = (assign36430_e48132 - 230.25850929940458);
        let assign36430_e48138: f64 = (-var_fbbtgat_d);
        let assign36430_e48140: f64 = (assign36430_e48138 / var_fmaxr);
        let assign36430_e48142: f64 = (assign36430_e48140 - 230.25850929940458);
        let assign36430_e48145: f64 = (-var_fbbtgat_d);
        let assign36430_e48147: f64 = (assign36430_e48145 / var_fmaxr);
        let assign36430_e48149: f64 = (assign36430_e48147 - 230.25850929940458);
        let assign36430_e48151: f64 = (assign36430_e48149 * 0.3333333333333333);
        let assign36430_e48152: f64 = (1.0 + assign36430_e48151);
        let assign36430_e48153: f64 = (assign36430_e48142 * assign36430_e48152);
        let assign36430_e48154: f64 = (0.5 * assign36430_e48153);
        let assign36430_e48155: f64 = (1.0 + assign36430_e48154);
        let assign36430_e48156: f64 = (assign36430_e48134 * assign36430_e48155);
        let assign36430_e48157: f64 = (1.0 + assign36430_e48156);
        let assign36430_e48158: f64 = (1e100 * assign36430_e48157);
        (assign36430_e48158, (1e100 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign36430_e48130 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign36430_e48155) + (assign36430_e48134 * (0.5 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign36430_e48138 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign36430_e48152) + (assign36430_e48142 * (((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign36430_e48145 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign36430_e48130 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign36430_e48155) + (assign36430_e48134 * (0.5 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign36430_e48138 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign36430_e48152) + (assign36430_e48142 * (((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign36430_e48145 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign36430_e48130 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign36430_e48155) + (assign36430_e48134 * (0.5 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign36430_e48138 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign36430_e48152) + (assign36430_e48142 * (((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign36430_e48145 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign36430_e48130 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign36430_e48155) + (assign36430_e48134 * (0.5 * ((((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign36430_e48138 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign36430_e48152) + (assign36430_e48142 * (((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign36430_e48145 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign36430_e48160;
        var_tmp_dn6 = assign36430_e48160_d_n6;
        var_tmp_dn7 = assign36430_e48160_d_n7;
        var_tmp_dn8 = assign36430_e48160_d_n8;
        var_tmp_dn9 = assign36430_e48160_d_n9;

        let (assign36440_e48180, assign36440_e48180_d_n6, assign36440_e48180_d_n7, assign36440_e48180_d_n8, assign36440_e48180_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard748 == 0.0)) {
        let assign36440_e48173: f64 = (var_v4 * var_fmaxr);
        let assign36440_e48175: f64 = (assign36440_e48173 * var_fmaxr);
        let assign36440_e48177: f64 = (assign36440_e48175 * var_tmp);
        let assign36440_e48178: f64 = (var_cbbtgatd_i * assign36440_e48177);
        (assign36440_e48178, (var_cbbtgatd_i * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign36440_e48173 * var_fmaxr_dn6)) * var_tmp) + (assign36440_e48175 * var_tmp_dn6))), (var_cbbtgatd_i * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign36440_e48173 * var_fmaxr_dn7)) * var_tmp) + (assign36440_e48175 * var_tmp_dn7))), (var_cbbtgatd_i * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign36440_e48173 * var_fmaxr_dn8)) * var_tmp) + (assign36440_e48175 * var_tmp_dn8))), (var_cbbtgatd_i * (((((var_v4 * var_fmaxr_dn9) * var_fmaxr) + (assign36440_e48173 * var_fmaxr_dn9)) * var_tmp) + (assign36440_e48175 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign36440_e48180;
        var_ibbt_dn6 = assign36440_e48180_d_n6;
        var_ibbt_dn7 = assign36440_e48180_d_n7;
        var_ibbt_dn8 = assign36440_e48180_d_n8;
        var_ibbt_dn9 = assign36440_e48180_d_n9;

        let assign36450_e48183: f64 = if var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard752 = assign36450_e48183;

        let (assign36460_e48194, assign36460_e48194_d_n6, assign36460_e48194_d_n7, assign36460_e48194_d_n8, assign36460_e48194_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard752 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign36460_e48194;
        var_fbreakdown_dn6 = assign36460_e48194_d_n6;
        var_fbreakdown_dn7 = assign36460_e48194_d_n7;
        var_fbreakdown_dn8 = assign36460_e48194_d_n8;
        var_fbreakdown_dn9 = assign36460_e48194_d_n9;

        let assign36470_e48197: f64 = (-var_alphaav);
        let assign36470_e48199: f64 = (assign36470_e48197 * var_vbrgatd_i);
        let assign36470_e48200: f64 = if var_vav > assign36470_e48199 { 1.0 } else { 0.0 };
        var_guard753 = assign36470_e48200;

        let assign36480_e48203: f64 = if var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard754 = assign36480_e48203;

        let (assign36490_e48233, assign36490_e48233_d_n6, assign36490_e48233_d_n7, assign36490_e48233_d_n8, assign36490_e48233_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard752 == 0.0)) && (var_guard753 != 0.0)) && (var_guard754 != 0.0)) {
        let assign36490_e48219: f64 = (var_vav * var_vbrinvgat_d);
        let assign36490_e48222: f64 = (var_vav * var_vbrinvgat_d);
        let assign36490_e48223: f64 = (assign36490_e48219 * assign36490_e48222);
        let assign36490_e48226: f64 = (var_vav * var_vbrinvgat_d);
        let assign36490_e48227: f64 = (assign36490_e48223 * assign36490_e48226);
        let assign36490_e48230: f64 = (var_vav * var_vbrinvgat_d);
        let assign36490_e48231: f64 = (assign36490_e48227 * assign36490_e48230);
        (assign36490_e48231, (((((((var_vav * var_vbrinvgat_d_dn6) * assign36490_e48222) + (assign36490_e48219 * (var_vav * var_vbrinvgat_d_dn6))) * assign36490_e48226) + (assign36490_e48223 * (var_vav * var_vbrinvgat_d_dn6))) * assign36490_e48230) + (assign36490_e48227 * (var_vav * var_vbrinvgat_d_dn6))), (((((((var_vav * var_vbrinvgat_d_dn7) * assign36490_e48222) + (assign36490_e48219 * (var_vav * var_vbrinvgat_d_dn7))) * assign36490_e48226) + (assign36490_e48223 * (var_vav * var_vbrinvgat_d_dn7))) * assign36490_e48230) + (assign36490_e48227 * (var_vav * var_vbrinvgat_d_dn7))), (((((((var_vav * var_vbrinvgat_d_dn8) * assign36490_e48222) + (assign36490_e48219 * (var_vav * var_vbrinvgat_d_dn8))) * assign36490_e48226) + (assign36490_e48223 * (var_vav * var_vbrinvgat_d_dn8))) * assign36490_e48230) + (assign36490_e48227 * (var_vav * var_vbrinvgat_d_dn8))), (((((((var_vav * var_vbrinvgat_d_dn9) * assign36490_e48222) + (assign36490_e48219 * (var_vav * var_vbrinvgat_d_dn9))) * assign36490_e48226) + (assign36490_e48223 * (var_vav * var_vbrinvgat_d_dn9))) * assign36490_e48230) + (assign36490_e48227 * (var_vav * var_vbrinvgat_d_dn9))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign36490_e48233;
        var_tmp_dn6 = assign36490_e48233_d_n6;
        var_tmp_dn7 = assign36490_e48233_d_n7;
        var_tmp_dn8 = assign36490_e48233_d_n8;
        var_tmp_dn9 = assign36490_e48233_d_n9;

        let (assign36500_e48255, assign36500_e48255_d_n6, assign36500_e48255_d_n7, assign36500_e48255_d_n8, assign36500_e48255_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard752 == 0.0)) && (var_guard753 != 0.0)) && (var_guard754 == 0.0)) {
        let assign36500_e48250: f64 = (var_vav * var_vbrinvgat_d);
        let assign36500_e48251: f64 = (assign36500_e48250).abs();
        let assign36500_e48253: f64 = (assign36500_e48251).powf(var_pbrgatd_i);
        (assign36500_e48253, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign36500_e48251).powf(var_pbrgatd_i - 1.0) * if assign36500_e48250 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) })) } } else { (assign36500_e48253 * (var_pbrgatd_i * (if assign36500_e48250 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) } / assign36500_e48251))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign36500_e48251).powf(var_pbrgatd_i - 1.0) * if assign36500_e48250 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) })) } } else { (assign36500_e48253 * (var_pbrgatd_i * (if assign36500_e48250 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) } / assign36500_e48251))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign36500_e48251).powf(var_pbrgatd_i - 1.0) * if assign36500_e48250 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) })) } } else { (assign36500_e48253 * (var_pbrgatd_i * (if assign36500_e48250 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) } / assign36500_e48251))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign36500_e48251).powf(var_pbrgatd_i - 1.0) * if assign36500_e48250 >= 0.0 { (var_vav * var_vbrinvgat_d_dn9) } else { (-(var_vav * var_vbrinvgat_d_dn9)) })) } } else { (assign36500_e48253 * (var_pbrgatd_i * (if assign36500_e48250 >= 0.0 { (var_vav * var_vbrinvgat_d_dn9) } else { (-(var_vav * var_vbrinvgat_d_dn9)) } / assign36500_e48251))) },)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign36500_e48255;
        var_tmp_dn6 = assign36500_e48255_d_n6;
        var_tmp_dn7 = assign36500_e48255_d_n7;
        var_tmp_dn8 = assign36500_e48255_d_n8;
        var_tmp_dn9 = assign36500_e48255_d_n9;

        let (assign36510_e48273, assign36510_e48273_d_n6, assign36510_e48273_d_n7, assign36510_e48273_d_n8, assign36510_e48273_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard752 == 0.0)) && (var_guard753 != 0.0)) {
        let assign36510_e48270: f64 = (1.0 - var_tmp);
        let assign36510_e48271: f64 = (1.0 / assign36510_e48270);
        (assign36510_e48271, (-((-var_tmp_dn6) / (assign36510_e48270 * assign36510_e48270))), (-((-var_tmp_dn7) / (assign36510_e48270 * assign36510_e48270))), (-((-var_tmp_dn8) / (assign36510_e48270 * assign36510_e48270))), (-((-var_tmp_dn9) / (assign36510_e48270 * assign36510_e48270))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign36510_e48273;
        var_fbreakdown_dn6 = assign36510_e48273_d_n6;
        var_fbreakdown_dn7 = assign36510_e48273_d_n7;
        var_fbreakdown_dn8 = assign36510_e48273_d_n8;
        var_fbreakdown_dn9 = assign36510_e48273_d_n9;

        let (assign36520_e48296, assign36520_e48296_d_n6, assign36520_e48296_d_n7, assign36520_e48296_d_n8, assign36520_e48296_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) && (var_guard752 == 0.0)) && (var_guard753 == 0.0)) {
        let assign36520_e48290: f64 = (var_alphaav * var_vbrgatd_i);
        let assign36520_e48291: f64 = (var_vav + assign36520_e48290);
        let assign36520_e48293: f64 = (assign36520_e48291 * var_slopegat_d);
        let assign36520_e48294: f64 = (var_fstopgat_d + assign36520_e48293);
        (assign36520_e48294, (assign36520_e48291 * var_slopegat_d_dn6), (assign36520_e48291 * var_slopegat_d_dn7), (assign36520_e48291 * var_slopegat_d_dn8), (assign36520_e48291 * var_slopegat_d_dn9),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign36520_e48296;
        var_fbreakdown_dn6 = assign36520_e48296_d_n6;
        var_fbreakdown_dn7 = assign36520_e48296_d_n7;
        var_fbreakdown_dn8 = assign36520_e48296_d_n8;
        var_fbreakdown_dn9 = assign36520_e48296_d_n9;

        let (assign36530_e48315, assign36530_e48315_d_n6, assign36530_e48315_d_n7, assign36530_e48315_d_n8, assign36530_e48315_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard738 == 0.0)) {
        let assign36530_e48306: f64 = (var_id__blk212 + var_isrh);
        let assign36530_e48308: f64 = (assign36530_e48306 + var_itat);
        let assign36530_e48310: f64 = (assign36530_e48308 + var_ibbt);
        let assign36530_e48311: f64 = (p.p29 * assign36530_e48310);
        let assign36530_e48313: f64 = (assign36530_e48311 * var_fbreakdown);
        (assign36530_e48313, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign36530_e48311 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign36530_e48311 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign36530_e48311 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign36530_e48311 * var_fbreakdown_dn9)),)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign36530_e48315;
        var_ijungat_dn6 = assign36530_e48315_d_n6;
        var_ijungat_dn7 = assign36530_e48315_d_n7;
        var_ijungat_dn8 = assign36530_e48315_d_n8;
        var_ijungat_dn9 = assign36530_e48315_d_n9;

        let (assign36540_e48331, assign36540_e48331_d_n6, assign36540_e48331_d_n7, assign36540_e48331_d_n8, assign36540_e48331_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign36540_e48321: f64 = (var_abdrain_i * var_ijunbot);
        let assign36540_e48324: f64 = (var_lsdrain_i * var_ijunsti);
        let assign36540_e48325: f64 = (assign36540_e48321 + assign36540_e48324);
        let assign36540_e48328: f64 = (var_lgdrain_i * var_ijungat);
        let assign36540_e48329: f64 = (assign36540_e48325 + assign36540_e48328);
        (assign36540_e48329, (((var_abdrain_i * var_ijunbot_dn6) + (var_lsdrain_i * var_ijunsti_dn6)) + (var_lgdrain_i * var_ijungat_dn6)), (((var_abdrain_i * var_ijunbot_dn7) + (var_lsdrain_i * var_ijunsti_dn7)) + (var_lgdrain_i * var_ijungat_dn7)), (((var_abdrain_i * var_ijunbot_dn8) + (var_lsdrain_i * var_ijunsti_dn8)) + (var_lgdrain_i * var_ijungat_dn8)), (((var_abdrain_i * var_ijunbot_dn9) + (var_lsdrain_i * var_ijunsti_dn9)) + (var_lgdrain_i * var_ijungat_dn9)),)
    } else {
        (var_i4, var_i4_dn6, var_i4_dn7, var_i4_dn8, var_i4_dn9,)
    }
};
        var_i4 = assign36540_e48331;
        var_i4_dn6 = assign36540_e48331_d_n6;
        var_i4_dn7 = assign36540_e48331_d_n7;
        var_i4_dn8 = assign36540_e48331_d_n8;
        var_i4_dn9 = assign36540_e48331_d_n9;

        let (assign36550_e48337,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign36550_e48337;

        let (assign36560_e48343,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign36560_e48343;

        let assign36570_e48355: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard755 = assign36570_e48355;

        let assign36650_e48441: f64 = if var_v5 < var_vmax_d { 1.0 } else { 0.0 };
        var_guard756 = assign36650_e48441;

        let assign36660_e48443: f64 = (-0.5);
        let assign36660_e48446: f64 = (var_v5 * var_phitdinv);
        let assign36660_e48447: f64 = (assign36660_e48443 * assign36660_e48446);
        let assign36660_e48448: f64 = (assign36660_e48447).abs();
        let assign36660_e48450: f64 = if assign36660_e48448 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard757 = assign36660_e48450;

        let (assign36670_e48468,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard755 != 0.0)) && (var_guard756 != 0.0)) && (var_guard757 != 0.0)) {
        let assign36670_e48461: f64 = (-0.5);
        let assign36670_e48464: f64 = (var_v5 * var_phitdinv);
        let assign36670_e48465: f64 = (assign36670_e48461 * assign36670_e48464);
        let assign36670_e48466: f64 = (assign36670_e48465).exp();
        (assign36670_e48466,)
    } else {
        (var_z,)
    }
};
        var_z = assign36670_e48468;

        let assign36680_e48470: f64 = (-0.5);
        let assign36680_e48473: f64 = (var_v5 * var_phitdinv);
        let assign36680_e48474: f64 = (assign36680_e48470 * assign36680_e48473);
        let assign36680_e48476: f64 = if assign36680_e48474 < 0.0 { 1.0 } else { 0.0 };
        var_guard758 = assign36680_e48476;

        let (assign36690_e48531,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard755 != 0.0)) && (var_guard756 != 0.0)) && (var_guard757 == 0.0)) && (var_guard758 != 0.0)) {
        let assign36690_e48492: f64 = (-230.25850929940458);
        let assign36690_e48494: f64 = (-0.5);
        let assign36690_e48497: f64 = (var_v5 * var_phitdinv);
        let assign36690_e48498: f64 = (assign36690_e48494 * assign36690_e48497);
        let assign36690_e48499: f64 = (assign36690_e48492 - assign36690_e48498);
        let assign36690_e48503: f64 = (-230.25850929940458);
        let assign36690_e48505: f64 = (-0.5);
        let assign36690_e48508: f64 = (var_v5 * var_phitdinv);
        let assign36690_e48509: f64 = (assign36690_e48505 * assign36690_e48508);
        let assign36690_e48510: f64 = (assign36690_e48503 - assign36690_e48509);
        let assign36690_e48513: f64 = (-230.25850929940458);
        let assign36690_e48515: f64 = (-0.5);
        let assign36690_e48518: f64 = (var_v5 * var_phitdinv);
        let assign36690_e48519: f64 = (assign36690_e48515 * assign36690_e48518);
        let assign36690_e48520: f64 = (assign36690_e48513 - assign36690_e48519);
        let assign36690_e48522: f64 = (assign36690_e48520 * 0.3333333333333333);
        let assign36690_e48523: f64 = (1.0 + assign36690_e48522);
        let assign36690_e48524: f64 = (assign36690_e48510 * assign36690_e48523);
        let assign36690_e48525: f64 = (0.5 * assign36690_e48524);
        let assign36690_e48526: f64 = (1.0 + assign36690_e48525);
        let assign36690_e48527: f64 = (assign36690_e48499 * assign36690_e48526);
        let assign36690_e48528: f64 = (1.0 + assign36690_e48527);
        let assign36690_e48529: f64 = (1e-100 / assign36690_e48528);
        (assign36690_e48529,)
    } else {
        (var_z,)
    }
};
        var_z = assign36690_e48531;

        let (assign36700_e48584,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard755 != 0.0)) && (var_guard756 != 0.0)) && (var_guard757 == 0.0)) && (var_guard758 == 0.0)) {
        let assign36700_e48548: f64 = (-0.5);
        let assign36700_e48551: f64 = (var_v5 * var_phitdinv);
        let assign36700_e48552: f64 = (assign36700_e48548 * assign36700_e48551);
        let assign36700_e48554: f64 = (assign36700_e48552 - 230.25850929940458);
        let assign36700_e48558: f64 = (-0.5);
        let assign36700_e48561: f64 = (var_v5 * var_phitdinv);
        let assign36700_e48562: f64 = (assign36700_e48558 * assign36700_e48561);
        let assign36700_e48564: f64 = (assign36700_e48562 - 230.25850929940458);
        let assign36700_e48567: f64 = (-0.5);
        let assign36700_e48570: f64 = (var_v5 * var_phitdinv);
        let assign36700_e48571: f64 = (assign36700_e48567 * assign36700_e48570);
        let assign36700_e48573: f64 = (assign36700_e48571 - 230.25850929940458);
        let assign36700_e48575: f64 = (assign36700_e48573 * 0.3333333333333333);
        let assign36700_e48576: f64 = (1.0 + assign36700_e48575);
        let assign36700_e48577: f64 = (assign36700_e48564 * assign36700_e48576);
        let assign36700_e48578: f64 = (0.5 * assign36700_e48577);
        let assign36700_e48579: f64 = (1.0 + assign36700_e48578);
        let assign36700_e48580: f64 = (assign36700_e48554 * assign36700_e48579);
        let assign36700_e48581: f64 = (1.0 + assign36700_e48580);
        let assign36700_e48582: f64 = (1e100 * assign36700_e48581);
        (assign36700_e48582,)
    } else {
        (var_z,)
    }
};
        var_z = assign36700_e48584;

        let (assign36710_e48596,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard755 != 0.0)) && (var_guard756 != 0.0)) {
        let assign36710_e48594: f64 = (1.0 / var_z);
        (assign36710_e48594,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign36710_e48596;

        let (assign36720_e48608,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard755 != 0.0)) && (var_guard756 != 0.0)) {
        let assign36720_e48606: f64 = (var_zinv * var_zinv);
        (assign36720_e48606,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign36720_e48608;

        let (assign36730_e48627,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard755 != 0.0)) && (var_guard756 == 0.0)) {
        let assign36730_e48620: f64 = (var_v5 - var_vmax_d);
        let assign36730_e48622: f64 = (assign36730_e48620 * var_phitdinv);
        let assign36730_e48623: f64 = (1.0 + assign36730_e48622);
        let assign36730_e48625: f64 = (assign36730_e48623 * var_exp_vmax_over_phitd_d);
        (assign36730_e48625,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign36730_e48627;

        let (assign36740_e48639,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard755 != 0.0)) && (var_guard756 == 0.0)) {
        let assign36740_e48637: f64 = (var_idmult).sqrt();
        (assign36740_e48637,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign36740_e48639;

        let (assign36750_e48652,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard755 != 0.0)) && (var_guard756 == 0.0)) {
        let assign36750_e48650: f64 = (1.0 / var_zinv);
        (assign36750_e48650,)
    } else {
        (var_z,)
    }
};
        var_z = assign36750_e48652;

        let (assign36760_e48662,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard755 != 0.0)) {
        let assign36760_e48660: f64 = (var_idmult - 1.0);
        (assign36760_e48660,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign36760_e48662;

        let assign36770_e48665: f64 = if var_v5 > 0.0 { 1.0 } else { 0.0 };
        var_guard759 = assign36770_e48665;

        let (assign36780_e48691,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard755 != 0.0)) && (var_guard759 != 0.0)) {
        let assign36780_e48677: f64 = (2.0 + var_z);
        let assign36780_e48680: f64 = (var_z + 1.0);
        let assign36780_e48683: f64 = (var_z + 3.0);
        let assign36780_e48684: f64 = (assign36780_e48680 * assign36780_e48683);
        let assign36780_e48685: f64 = (assign36780_e48684).sqrt();
        let assign36780_e48686: f64 = (assign36780_e48677 + assign36780_e48685);
        let assign36780_e48687: f64 = (assign36780_e48686).ln();
        let assign36780_e48688: f64 = (var_phitd * assign36780_e48687);
        let assign36780_e48689: f64 = (2.0 * assign36780_e48688);
        (assign36780_e48689,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign36780_e48691;

        let (assign36790_e48725,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard755 != 0.0)) && (var_guard759 == 0.0)) {
        let assign36790_e48701: f64 = (-var_v5);
        let assign36790_e48706: f64 = (2.0 * var_zinv);
        let assign36790_e48708: f64 = (assign36790_e48706 + 1.0);
        let assign36790_e48711: f64 = (1.0 + var_zinv);
        let assign36790_e48715: f64 = (3.0 * var_zinv);
        let assign36790_e48716: f64 = (1.0 + assign36790_e48715);
        let assign36790_e48717: f64 = (assign36790_e48711 * assign36790_e48716);
        let assign36790_e48718: f64 = (assign36790_e48717).sqrt();
        let assign36790_e48719: f64 = (assign36790_e48708 + assign36790_e48718);
        let assign36790_e48720: f64 = (assign36790_e48719).ln();
        let assign36790_e48721: f64 = (var_phitd * assign36790_e48720);
        let assign36790_e48722: f64 = (2.0 * assign36790_e48721);
        let assign36790_e48723: f64 = (assign36790_e48701 + assign36790_e48722);
        (assign36790_e48723,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign36790_e48725;

        let (assign36800_e48735,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard755 != 0.0)) {
        let assign36800_e48733: f64 = (var_vbimin_d - var_two_psistar);
        (assign36800_e48733,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign36800_e48735;

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

    pub(super) fn stamp_transient_block_78(
        var_abdrain_i: f64,
        var_atatbot_d: f64,
        var_btatpartbot_d: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_ftdbot_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard755: f64,
        var_idmult: f64,
        var_idsatbot_d: f64,
        var_one_minus_pbot_d: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_perfc: f64,
        var_phitd: f64,
        var_phitr: f64,
        var_two_psistar: f64,
        var_v5: f64,
        var_vbbtlim_d: f64,
        var_vbibot_d: f64,
        var_vbirbotinv_d: f64,
        var_vjlim: f64,
        var_wdepnulrbot_d: f64,
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
        var_guard760_slot: &mut f64,
        var_guard761_slot: &mut f64,
        var_guard762_slot: &mut f64,
        var_guard763_slot: &mut f64,
        var_guard764_slot: &mut f64,
        var_guard765_slot: &mut f64,
        var_guard766_slot: &mut f64,
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
        let mut var_guard760: f64 = *var_guard760_slot;
        let mut var_guard761: f64 = *var_guard761_slot;
        let mut var_guard762: f64 = *var_guard762_slot;
        let mut var_guard763: f64 = *var_guard763_slot;
        let mut var_guard764: f64 = *var_guard764_slot;
        let mut var_guard765: f64 = *var_guard765_slot;
        let mut var_guard766: f64 = *var_guard766_slot;
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

        let (assign36810_e48762,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard755 != 0.0)) {
        let assign36810_e48744: f64 = (var_v5 + var_vjlim);
        let assign36810_e48747: f64 = (var_v5 - var_vjlim);
        let assign36810_e48750: f64 = (var_v5 - var_vjlim);
        let assign36810_e48751: f64 = (assign36810_e48747 * assign36810_e48750);
        let assign36810_e48754: f64 = (4.0 * var_phitd);
        let assign36810_e48756: f64 = (assign36810_e48754 * var_phitd);
        let assign36810_e48757: f64 = (assign36810_e48751 + assign36810_e48756);
        let assign36810_e48758: f64 = (assign36810_e48757).sqrt();
        let assign36810_e48759: f64 = (assign36810_e48744 - assign36810_e48758);
        let assign36810_e48760: f64 = (0.5 * assign36810_e48759);
        (assign36810_e48760,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign36810_e48762;

        let (assign36820_e48789,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard755 != 0.0)) {
        let assign36820_e48771: f64 = (var_v5 + var_vbbtlim_d);
        let assign36820_e48774: f64 = (var_v5 - var_vbbtlim_d);
        let assign36820_e48777: f64 = (var_v5 - var_vbbtlim_d);
        let assign36820_e48778: f64 = (assign36820_e48774 * assign36820_e48777);
        let assign36820_e48781: f64 = (4.0 * var_phitr);
        let assign36820_e48783: f64 = (assign36820_e48781 * var_phitr);
        let assign36820_e48784: f64 = (assign36820_e48778 + assign36820_e48783);
        let assign36820_e48785: f64 = (assign36820_e48784).sqrt();
        let assign36820_e48786: f64 = (assign36820_e48771 - assign36820_e48785);
        let assign36820_e48787: f64 = (0.5 * assign36820_e48786);
        (assign36820_e48787,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign36820_e48789;

        let (assign36830_e48816,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard755 != 0.0)) {
        let assign36830_e48798: f64 = var_v5;
        let assign36830_e48801: f64 = var_v5;
        let assign36830_e48804: f64 = var_v5;
        let assign36830_e48805: f64 = (assign36830_e48801 * assign36830_e48804);
        let assign36830_e48808: f64 = (4.0 * 1e-6);
        let assign36830_e48810: f64 = (assign36830_e48808 * 1e-6);
        let assign36830_e48811: f64 = (assign36830_e48805 + assign36830_e48810);
        let assign36830_e48812: f64 = (assign36830_e48811).sqrt();
        let assign36830_e48813: f64 = (assign36830_e48798 - assign36830_e48812);
        let assign36830_e48814: f64 = (0.5 * assign36830_e48813);
        (assign36830_e48814,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign36830_e48816;

        let assign36840_e48819: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard760 = assign36840_e48819;

        let (assign36850_e48827, assign36850_e48827_d_n6, assign36850_e48827_d_n7, assign36850_e48827_d_n8, assign36850_e48827_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign36850_e48827;
        var_ijunbot_dn6 = assign36850_e48827_d_n6;
        var_ijunbot_dn7 = assign36850_e48827_d_n7;
        var_ijunbot_dn8 = assign36850_e48827_d_n8;
        var_ijunbot_dn9 = assign36850_e48827_d_n9;

        let (assign36860_e48838,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) {
        let assign36860_e48836: f64 = (var_idsatbot_d * var_idmult);
        (assign36860_e48836,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign36860_e48838;

        let assign36870_e48845: f64 = if ((var_csrhbotd_i == 0.0) && (var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard761 = assign36870_e48845;

        let (assign36880_e48856, assign36880_e48856_d_n6, assign36880_e48856_d_n7, assign36880_e48856_d_n8, assign36880_e48856_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard761 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign36880_e48856;
        var_isrh_dn6 = assign36880_e48856_d_n6;
        var_isrh_dn7 = assign36880_e48856_d_n7;
        var_isrh_dn8 = assign36880_e48856_d_n8;
        var_isrh_dn9 = assign36880_e48856_d_n9;

        let (assign36890_e48870,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard761 == 0.0)) {
        let assign36890_e48868: f64 = (var_vbibot_d - var_vjsrh);
        (assign36890_e48868,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign36890_e48870;

        let (assign36900_e48889,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard761 == 0.0)) {
        let assign36900_e48884: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign36900_e48885: f64 = (1.0 - assign36900_e48884);
        let assign36900_e48886: f64 = (assign36900_e48885).sqrt();
        let assign36900_e48887: f64 = (1.0 - assign36900_e48886);
        (assign36900_e48887,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign36900_e48889;

        let assign36910_e48892: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard762 = assign36910_e48892;

        let (assign36920_e48906,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard761 == 0.0)) && (var_guard762 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign36920_e48906;

        let (assign36930_e48938,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard761 == 0.0)) && (var_guard762 == 0.0)) {
        let assign36930_e48921: f64 = (var_wsrhstep * var_wsrhstep);
        let assign36930_e48923: f64 = (var_wsrhstep).ln();
        let assign36930_e48924: f64 = (assign36930_e48921 * assign36930_e48923);
        let assign36930_e48927: f64 = (1.0 - var_wsrhstep);
        let assign36930_e48928: f64 = (assign36930_e48924 / assign36930_e48927);
        let assign36930_e48930: f64 = (assign36930_e48928 + var_wsrhstep);
        let assign36930_e48934: f64 = (2.0 * var_pbotd_i);
        let assign36930_e48935: f64 = (1.0 - assign36930_e48934);
        let assign36930_e48936: f64 = (assign36930_e48930 * assign36930_e48935);
        (assign36930_e48936,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign36930_e48938;

        let (assign36940_e48952,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard761 == 0.0)) {
        let assign36940_e48950: f64 = (var_wsrhstep + var_dwsrh);
        (assign36940_e48950,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign36940_e48952;

        let assign36950_e48955: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard763 = assign36950_e48955;

        let (assign36960_e48972, assign36960_e48972_d_n6, assign36960_e48972_d_n7, assign36960_e48972_d_n8, assign36960_e48972_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard761 == 0.0)) && (var_guard763 != 0.0)) {
        let assign36960_e48969: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign36960_e48970: f64 = (assign36960_e48969).sqrt();
        (assign36960_e48970, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign36960_e48972;
        var_tmp_dn6 = assign36960_e48972_d_n6;
        var_tmp_dn7 = assign36960_e48972_d_n7;
        var_tmp_dn8 = assign36960_e48972_d_n8;
        var_tmp_dn9 = assign36960_e48972_d_n9;

        let (assign36970_e48991, assign36970_e48991_d_n6, assign36970_e48991_d_n7, assign36970_e48991_d_n8, assign36970_e48991_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard761 == 0.0)) && (var_guard763 == 0.0)) {
        let assign36970_e48987: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign36970_e48989: f64 = (assign36970_e48987).powf(var_pbotd_i);
        (assign36970_e48989, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign36970_e48991;
        var_tmp_dn6 = assign36970_e48991_d_n6;
        var_tmp_dn7 = assign36970_e48991_d_n7;
        var_tmp_dn8 = assign36970_e48991_d_n8;
        var_tmp_dn9 = assign36970_e48991_d_n9;

        let (assign36980_e49005, assign36980_e49005_d_n6, assign36980_e49005_d_n7, assign36980_e49005_d_n8, assign36980_e49005_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard761 == 0.0)) {
        let assign36980_e49003: f64 = (var_wdepnulrbot_d * var_tmp);
        (assign36980_e49003, (var_wdepnulrbot_d * var_tmp_dn6), (var_wdepnulrbot_d * var_tmp_dn7), (var_wdepnulrbot_d * var_tmp_dn8), (var_wdepnulrbot_d * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign36980_e49005;
        var_wdep_dn6 = assign36980_e49005_d_n6;
        var_wdep_dn7 = assign36980_e49005_d_n7;
        var_wdep_dn8 = assign36980_e49005_d_n8;
        var_wdep_dn9 = assign36980_e49005_d_n9;

        let (assign36990_e49023, assign36990_e49023_d_n6, assign36990_e49023_d_n7, assign36990_e49023_d_n8, assign36990_e49023_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard761 == 0.0)) {
        let assign36990_e49018: f64 = (var_zinv - 1.0);
        let assign36990_e49020: f64 = (assign36990_e49018 * var_wdep);
        let assign36990_e49021: f64 = (var_ftdbot_d * assign36990_e49020);
        (assign36990_e49021, (var_ftdbot_d * (assign36990_e49018 * var_wdep_dn6)), (var_ftdbot_d * (assign36990_e49018 * var_wdep_dn7)), (var_ftdbot_d * (assign36990_e49018 * var_wdep_dn8)), (var_ftdbot_d * (assign36990_e49018 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign36990_e49023;
        var_asrh_dn6 = assign36990_e49023_d_n6;
        var_asrh_dn7 = assign36990_e49023_d_n7;
        var_asrh_dn8 = assign36990_e49023_d_n8;
        var_asrh_dn9 = assign36990_e49023_d_n9;

        let (assign37000_e49039, assign37000_e49039_d_n6, assign37000_e49039_d_n7, assign37000_e49039_d_n8, assign37000_e49039_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard761 == 0.0)) {
        let assign37000_e49036: f64 = (var_asrh * var_wsrh);
        let assign37000_e49037: f64 = (var_csrhbotd_i * assign37000_e49036);
        (assign37000_e49037, (var_csrhbotd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn8 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign37000_e49039;
        var_isrh_dn6 = assign37000_e49039_d_n6;
        var_isrh_dn7 = assign37000_e49039_d_n7;
        var_isrh_dn8 = assign37000_e49039_d_n8;
        var_isrh_dn9 = assign37000_e49039_d_n9;

        let assign37010_e49042: f64 = if var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard764 = assign37010_e49042;

        let (assign37020_e49053, assign37020_e49053_d_n6, assign37020_e49053_d_n7, assign37020_e49053_d_n8, assign37020_e49053_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign37020_e49053;
        var_itat_dn6 = assign37020_e49053_d_n6;
        var_itat_dn7 = assign37020_e49053_d_n7;
        var_itat_dn8 = assign37020_e49053_d_n8;
        var_itat_dn9 = assign37020_e49053_d_n9;

        let (assign37030_e49071, assign37030_e49071_d_n6, assign37030_e49071_d_n7, assign37030_e49071_d_n8, assign37030_e49071_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) {
        let assign37030_e49066: f64 = (var_wdep * var_one_minus_pbot_d);
        let assign37030_e49068: f64 = (assign37030_e49066 / var_vbi_minus_vjsrh);
        let assign37030_e49069: f64 = (var_btatpartbot_d * assign37030_e49068);
        (assign37030_e49069, (var_btatpartbot_d * ((var_wdep_dn6 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn7 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn8 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn9 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign37030_e49071;
        var_btat_dn6 = assign37030_e49071_d_n6;
        var_btat_dn7 = assign37030_e49071_d_n7;
        var_btat_dn8 = assign37030_e49071_d_n8;
        var_btat_dn9 = assign37030_e49071_d_n9;

        let (assign37040_e49087, assign37040_e49087_d_n6, assign37040_e49087_d_n7, assign37040_e49087_d_n8, assign37040_e49087_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) {
        let assign37040_e49083: f64 = (0.666666666666667 * var_atatbot_d);
        let assign37040_e49085: f64 = (assign37040_e49083 / var_btat);
        (assign37040_e49085, (-((assign37040_e49083 * var_btat_dn6) / (var_btat * var_btat))), (-((assign37040_e49083 * var_btat_dn7) / (var_btat * var_btat))), (-((assign37040_e49083 * var_btat_dn8) / (var_btat * var_btat))), (-((assign37040_e49083 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign37040_e49087;
        var_twoatatoverthreebtat_dn6 = assign37040_e49087_d_n6;
        var_twoatatoverthreebtat_dn7 = assign37040_e49087_d_n7;
        var_twoatatoverthreebtat_dn8 = assign37040_e49087_d_n8;
        var_twoatatoverthreebtat_dn9 = assign37040_e49087_d_n9;

        let (assign37050_e49101, assign37050_e49101_d_n6, assign37050_e49101_d_n7, assign37050_e49101_d_n8, assign37050_e49101_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) {
        let assign37050_e49099: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign37050_e49099, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign37050_e49101;
        var_umaxbeforelimiting_dn6 = assign37050_e49101_d_n6;
        var_umaxbeforelimiting_dn7 = assign37050_e49101_d_n7;
        var_umaxbeforelimiting_dn8 = assign37050_e49101_d_n8;
        var_umaxbeforelimiting_dn9 = assign37050_e49101_d_n9;

        let (assign37060_e49122, assign37060_e49122_d_n6, assign37060_e49122_d_n7, assign37060_e49122_d_n8, assign37060_e49122_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) {
        let assign37060_e49113: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign37060_e49116: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign37060_e49118: f64 = (assign37060_e49116 + 1.0);
        let assign37060_e49119: f64 = (assign37060_e49113 / assign37060_e49118);
        let assign37060_e49120: f64 = (assign37060_e49119).sqrt();
        (assign37060_e49120, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign37060_e49118) - (assign37060_e49113 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign37060_e49118 * assign37060_e49118)) / (2.0 * assign37060_e49120)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign37060_e49118) - (assign37060_e49113 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign37060_e49118 * assign37060_e49118)) / (2.0 * assign37060_e49120)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign37060_e49118) - (assign37060_e49113 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign37060_e49118 * assign37060_e49118)) / (2.0 * assign37060_e49120)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign37060_e49118) - (assign37060_e49113 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign37060_e49118 * assign37060_e49118)) / (2.0 * assign37060_e49120)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign37060_e49122;
        var_umax_dn6 = assign37060_e49122_d_n6;
        var_umax_dn7 = assign37060_e49122_d_n7;
        var_umax_dn8 = assign37060_e49122_d_n8;
        var_umax_dn9 = assign37060_e49122_d_n9;

        let (assign37070_e49135, assign37070_e49135_d_n6, assign37070_e49135_d_n7, assign37070_e49135_d_n8, assign37070_e49135_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) {
        let assign37070_e49133: f64 = (var_umax).sqrt();
        (assign37070_e49133, (var_umax_dn6 / (2.0 * assign37070_e49133)), (var_umax_dn7 / (2.0 * assign37070_e49133)), (var_umax_dn8 / (2.0 * assign37070_e49133)), (var_umax_dn9 / (2.0 * assign37070_e49133)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign37070_e49135;
        var_sqrtumax_dn6 = assign37070_e49135_d_n6;
        var_sqrtumax_dn7 = assign37070_e49135_d_n7;
        var_sqrtumax_dn8 = assign37070_e49135_d_n8;
        var_sqrtumax_dn9 = assign37070_e49135_d_n9;

        let (assign37080_e49149, assign37080_e49149_d_n6, assign37080_e49149_d_n7, assign37080_e49149_d_n8, assign37080_e49149_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) {
        let assign37080_e49147: f64 = (var_umax * var_sqrtumax);
        (assign37080_e49147, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign37080_e49149;
        var_umaxpoweronepointfive_dn6 = assign37080_e49149_d_n6;
        var_umaxpoweronepointfive_dn7 = assign37080_e49149_d_n7;
        var_umaxpoweronepointfive_dn8 = assign37080_e49149_d_n8;
        var_umaxpoweronepointfive_dn9 = assign37080_e49149_d_n9;

        let assign37090_e49151: f64 = (-var_pbotd_i);
        let assign37090_e49153: f64 = (assign37090_e49151 * var_one_over_one_minus_pbot_d);
        let assign37090_e49155: f64 = (-1.0);
        let assign37090_e49156: f64 = if assign37090_e49153 == assign37090_e49155 { 1.0 } else { 0.0 };
        var_guard765 = assign37090_e49156;

        let (assign37100_e49176, assign37100_e49176_d_n6, assign37100_e49176_d_n7, assign37100_e49176_d_n8, assign37100_e49176_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) && (var_guard765 != 0.0)) {
        let assign37100_e49172: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign37100_e49173: f64 = (1.0 + assign37100_e49172);
        let assign37100_e49174: f64 = (1.0 / assign37100_e49173);
        (assign37100_e49174, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign37100_e49173 * assign37100_e49173))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign37100_e49173 * assign37100_e49173))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign37100_e49173 * assign37100_e49173))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign37100_e49173 * assign37100_e49173))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign37100_e49176;
        var_wgamma_dn6 = assign37100_e49176_d_n6;
        var_wgamma_dn7 = assign37100_e49176_d_n7;
        var_wgamma_dn8 = assign37100_e49176_d_n8;
        var_wgamma_dn9 = assign37100_e49176_d_n9;

        let (assign37110_e49200, assign37110_e49200_d_n6, assign37110_e49200_d_n7, assign37110_e49200_d_n8, assign37110_e49200_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) && (var_guard765 == 0.0)) {
        let assign37110_e49192: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign37110_e49193: f64 = (1.0 + assign37110_e49192);
        let assign37110_e49195: f64 = (-var_pbotd_i);
        let assign37110_e49197: f64 = (assign37110_e49195 * var_one_over_one_minus_pbot_d);
        let assign37110_e49198: f64 = (assign37110_e49193).powf(assign37110_e49197);
        (assign37110_e49198, if 0.0 == 0.0 && ((assign37110_e49197) as f64).is_finite() && ((assign37110_e49197) as f64).fract() == 0.0 { if assign37110_e49197 == 0.0 { 0.0 } else { (assign37110_e49197 * ((assign37110_e49193).powf(assign37110_e49197 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign37110_e49198 * (assign37110_e49197 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign37110_e49193))) }, if 0.0 == 0.0 && ((assign37110_e49197) as f64).is_finite() && ((assign37110_e49197) as f64).fract() == 0.0 { if assign37110_e49197 == 0.0 { 0.0 } else { (assign37110_e49197 * ((assign37110_e49193).powf(assign37110_e49197 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign37110_e49198 * (assign37110_e49197 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign37110_e49193))) }, if 0.0 == 0.0 && ((assign37110_e49197) as f64).is_finite() && ((assign37110_e49197) as f64).fract() == 0.0 { if assign37110_e49197 == 0.0 { 0.0 } else { (assign37110_e49197 * ((assign37110_e49193).powf(assign37110_e49197 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign37110_e49198 * (assign37110_e49197 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign37110_e49193))) }, if 0.0 == 0.0 && ((assign37110_e49197) as f64).is_finite() && ((assign37110_e49197) as f64).fract() == 0.0 { if assign37110_e49197 == 0.0 { 0.0 } else { (assign37110_e49197 * ((assign37110_e49193).powf(assign37110_e49197 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign37110_e49198 * (assign37110_e49197 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign37110_e49193))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign37110_e49200;
        var_wgamma_dn6 = assign37110_e49200_d_n6;
        var_wgamma_dn7 = assign37110_e49200_d_n7;
        var_wgamma_dn8 = assign37110_e49200_d_n8;
        var_wgamma_dn9 = assign37110_e49200_d_n9;

        let (assign37120_e49218, assign37120_e49218_d_n6, assign37120_e49218_d_n7, assign37120_e49218_d_n8, assign37120_e49218_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) {
        let assign37120_e49212: f64 = (var_wsrh * var_wgamma);
        let assign37120_e49215: f64 = (var_wsrh + var_wgamma);
        let assign37120_e49216: f64 = (assign37120_e49212 / assign37120_e49215);
        (assign37120_e49216, ((((var_wsrh * var_wgamma_dn6) * assign37120_e49215) - (assign37120_e49212 * var_wgamma_dn6)) / (assign37120_e49215 * assign37120_e49215)), ((((var_wsrh * var_wgamma_dn7) * assign37120_e49215) - (assign37120_e49212 * var_wgamma_dn7)) / (assign37120_e49215 * assign37120_e49215)), ((((var_wsrh * var_wgamma_dn8) * assign37120_e49215) - (assign37120_e49212 * var_wgamma_dn8)) / (assign37120_e49215 * assign37120_e49215)), ((((var_wsrh * var_wgamma_dn9) * assign37120_e49215) - (assign37120_e49212 * var_wgamma_dn9)) / (assign37120_e49215 * assign37120_e49215)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign37120_e49218;
        var_wtat_dn6 = assign37120_e49218_d_n6;
        var_wtat_dn7 = assign37120_e49218_d_n7;
        var_wtat_dn8 = assign37120_e49218_d_n8;
        var_wtat_dn9 = assign37120_e49218_d_n9;

        let (assign37130_e49235, assign37130_e49235_d_n6, assign37130_e49235_d_n7, assign37130_e49235_d_n8, assign37130_e49235_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) {
        let assign37130_e49231: f64 = (var_btat / var_sqrtumax);
        let assign37130_e49232: f64 = (0.375 * assign37130_e49231);
        let assign37130_e49233: f64 = (assign37130_e49232).sqrt();
        (assign37130_e49233, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign37130_e49233)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign37130_e49233)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign37130_e49233)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign37130_e49233)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign37130_e49235;
        var_ktat_dn6 = assign37130_e49235_d_n6;
        var_ktat_dn7 = assign37130_e49235_d_n7;
        var_ktat_dn8 = assign37130_e49235_d_n8;
        var_ktat_dn9 = assign37130_e49235_d_n9;

        let (assign37140_e49253, assign37140_e49253_d_n6, assign37140_e49253_d_n7, assign37140_e49253_d_n8, assign37140_e49253_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) {
        let assign37140_e49248: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign37140_e49249: f64 = (2.0 * assign37140_e49248);
        let assign37140_e49251: f64 = (assign37140_e49249 - var_umax);
        (assign37140_e49251, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign37140_e49253;
        var_ltat_dn6 = assign37140_e49253_d_n6;
        var_ltat_dn7 = assign37140_e49253_d_n7;
        var_ltat_dn8 = assign37140_e49253_d_n8;
        var_ltat_dn9 = assign37140_e49253_d_n9;

        let (assign37150_e49279, assign37150_e49279_d_n6, assign37150_e49279_d_n7, assign37150_e49279_d_n8, assign37150_e49279_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) {
        let assign37150_e49265: f64 = (var_atatbot_d * var_twoatatoverthreebtat);
        let assign37150_e49267: f64 = (assign37150_e49265 * var_sqrtumax);
        let assign37150_e49270: f64 = (var_atatbot_d * var_umax);
        let assign37150_e49271: f64 = (assign37150_e49267 - assign37150_e49270);
        let assign37150_e49275: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign37150_e49276: f64 = (0.5 * assign37150_e49275);
        let assign37150_e49277: f64 = (assign37150_e49271 + assign37150_e49276);
        (assign37150_e49277, (((((var_atatbot_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign37150_e49265 * var_sqrtumax_dn6)) - (var_atatbot_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign37150_e49265 * var_sqrtumax_dn7)) - (var_atatbot_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign37150_e49265 * var_sqrtumax_dn8)) - (var_atatbot_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign37150_e49265 * var_sqrtumax_dn9)) - (var_atatbot_d * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign37150_e49279;
        var_mtat_dn6 = assign37150_e49279_d_n6;
        var_mtat_dn7 = assign37150_e49279_d_n7;
        var_mtat_dn8 = assign37150_e49279_d_n8;
        var_mtat_dn9 = assign37150_e49279_d_n9;

        let (assign37160_e49295, assign37160_e49295_d_n6, assign37160_e49295_d_n7, assign37160_e49295_d_n8, assign37160_e49295_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) {
        let assign37160_e49291: f64 = (var_ltat - 1.0);
        let assign37160_e49293: f64 = (assign37160_e49291 * var_ktat);
        (assign37160_e49293, ((var_ltat_dn6 * var_ktat) + (assign37160_e49291 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign37160_e49291 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign37160_e49291 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign37160_e49291 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign37160_e49295;
        var_xerfc_dn6 = assign37160_e49295_d_n6;
        var_xerfc_dn7 = assign37160_e49295_d_n7;
        var_xerfc_dn8 = assign37160_e49295_d_n8;
        var_xerfc_dn9 = assign37160_e49295_d_n9;

        let (assign37170_e49309, assign37170_e49309_d_n6, assign37170_e49309_d_n7, assign37170_e49309_d_n8, assign37170_e49309_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) {
        let assign37170_e49307: f64 = (var_xerfc * var_xerfc);
        (assign37170_e49307, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign37170_e49309;
        var_ysq_dn6 = assign37170_e49309_d_n6;
        var_ysq_dn7 = assign37170_e49309_d_n7;
        var_ysq_dn8 = assign37170_e49309_d_n8;
        var_ysq_dn9 = assign37170_e49309_d_n9;

        let assign37180_e49312: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard766 = assign37180_e49312;

        let (assign37190_e49332, assign37190_e49332_d_n6, assign37190_e49332_d_n7, assign37190_e49332_d_n8, assign37190_e49332_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) && (var_guard766 != 0.0)) {
        let assign37190_e49328: f64 = (var_perfc * var_xerfc);
        let assign37190_e49329: f64 = (1.0 + assign37190_e49328);
        let assign37190_e49330: f64 = (1.0 / assign37190_e49329);
        (assign37190_e49330, (-((var_perfc * var_xerfc_dn6) / (assign37190_e49329 * assign37190_e49329))), (-((var_perfc * var_xerfc_dn7) / (assign37190_e49329 * assign37190_e49329))), (-((var_perfc * var_xerfc_dn8) / (assign37190_e49329 * assign37190_e49329))), (-((var_perfc * var_xerfc_dn9) / (assign37190_e49329 * assign37190_e49329))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign37190_e49332;
        var_terfc_dn6 = assign37190_e49332_d_n6;
        var_terfc_dn7 = assign37190_e49332_d_n7;
        var_terfc_dn8 = assign37190_e49332_d_n8;
        var_terfc_dn9 = assign37190_e49332_d_n9;

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
        *var_guard760_slot = var_guard760;
        *var_guard761_slot = var_guard761;
        *var_guard762_slot = var_guard762;
        *var_guard763_slot = var_guard763;
        *var_guard764_slot = var_guard764;
        *var_guard765_slot = var_guard765;
        *var_guard766_slot = var_guard766;
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

    pub(super) fn stamp_transient_block_79(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatbot_d: f64,
        var_berfc: f64,
        var_cbbtbotd_i: f64,
        var_cerfc: f64,
        var_ctatbotd_i: f64,
        var_fbbtbot_d: f64,
        var_fstopbot_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard760: f64,
        var_guard764: f64,
        var_guard766: f64,
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
        var_lsdrain_i: f64,
        var_mtat: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_mtat_dn9: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_pbrbotd_i: f64,
        var_perfc: f64,
        var_slopebot_d: f64,
        var_v5: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirbotd_i: f64,
        var_vbirbotinv_d: f64,
        var_vbrbotd_i: f64,
        var_vbrinvbot_d: f64,
        var_wdepnulrinvbot_d: f64,
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
        var_guard767_slot: &mut f64,
        var_guard768_slot: &mut f64,
        var_guard769_slot: &mut f64,
        var_guard770_slot: &mut f64,
        var_guard771_slot: &mut f64,
        var_guard772_slot: &mut f64,
        var_guard773_slot: &mut f64,
        var_guard774_slot: &mut f64,
        var_guard775_slot: &mut f64,
        var_guard776_slot: &mut f64,
        var_guard777_slot: &mut f64,
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
        let mut var_guard767: f64 = *var_guard767_slot;
        let mut var_guard768: f64 = *var_guard768_slot;
        let mut var_guard769: f64 = *var_guard769_slot;
        let mut var_guard770: f64 = *var_guard770_slot;
        let mut var_guard771: f64 = *var_guard771_slot;
        let mut var_guard772: f64 = *var_guard772_slot;
        let mut var_guard773: f64 = *var_guard773_slot;
        let mut var_guard774: f64 = *var_guard774_slot;
        let mut var_guard775: f64 = *var_guard775_slot;
        let mut var_guard776: f64 = *var_guard776_slot;
        let mut var_guard777: f64 = *var_guard777_slot;
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

        let (assign37200_e49353, assign37200_e49353_d_n6, assign37200_e49353_d_n7, assign37200_e49353_d_n8, assign37200_e49353_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) && (var_guard766 == 0.0)) {
        let assign37200_e49349: f64 = (var_perfc * var_xerfc);
        let assign37200_e49350: f64 = (1.0 - assign37200_e49349);
        let assign37200_e49351: f64 = (1.0 / assign37200_e49350);
        (assign37200_e49351, (-((-(var_perfc * var_xerfc_dn6)) / (assign37200_e49350 * assign37200_e49350))), (-((-(var_perfc * var_xerfc_dn7)) / (assign37200_e49350 * assign37200_e49350))), (-((-(var_perfc * var_xerfc_dn8)) / (assign37200_e49350 * assign37200_e49350))), (-((-(var_perfc * var_xerfc_dn9)) / (assign37200_e49350 * assign37200_e49350))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign37200_e49353;
        var_terfc_dn6 = assign37200_e49353_d_n6;
        var_terfc_dn7 = assign37200_e49353_d_n7;
        var_terfc_dn8 = assign37200_e49353_d_n8;
        var_terfc_dn9 = assign37200_e49353_d_n9;

        let assign37210_e49355: f64 = (-var_ysq);
        let assign37210_e49357: f64 = (assign37210_e49355 + var_mtat);
        let assign37210_e49359: f64 = (-230.25850929940458);
        let assign37210_e49360: f64 = if assign37210_e49357 > assign37210_e49359 { 1.0 } else { 0.0 };
        var_guard767 = assign37210_e49360;

        let (assign37220_e49378, assign37220_e49378_d_n6, assign37220_e49378_d_n7, assign37220_e49378_d_n8, assign37220_e49378_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) && (var_guard767 != 0.0)) {
        let assign37220_e49373: f64 = (-var_ysq);
        let assign37220_e49375: f64 = (assign37220_e49373 + var_mtat);
        let assign37220_e49376: f64 = (assign37220_e49375).exp();
        (assign37220_e49376, (assign37220_e49376 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign37220_e49376 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign37220_e49376 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign37220_e49376 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign37220_e49378;
        var_tmp_dn6 = assign37220_e49378_d_n6;
        var_tmp_dn7 = assign37220_e49378_d_n7;
        var_tmp_dn8 = assign37220_e49378_d_n8;
        var_tmp_dn9 = assign37220_e49378_d_n9;

        let (assign37230_e49427, assign37230_e49427_d_n6, assign37230_e49427_d_n7, assign37230_e49427_d_n8, assign37230_e49427_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) && (var_guard767 == 0.0)) {
        let assign37230_e49394: f64 = (-230.25850929940458);
        let assign37230_e49396: f64 = (-var_ysq);
        let assign37230_e49398: f64 = (assign37230_e49396 + var_mtat);
        let assign37230_e49399: f64 = (assign37230_e49394 - assign37230_e49398);
        let assign37230_e49403: f64 = (-230.25850929940458);
        let assign37230_e49405: f64 = (-var_ysq);
        let assign37230_e49407: f64 = (assign37230_e49405 + var_mtat);
        let assign37230_e49408: f64 = (assign37230_e49403 - assign37230_e49407);
        let assign37230_e49411: f64 = (-230.25850929940458);
        let assign37230_e49413: f64 = (-var_ysq);
        let assign37230_e49415: f64 = (assign37230_e49413 + var_mtat);
        let assign37230_e49416: f64 = (assign37230_e49411 - assign37230_e49415);
        let assign37230_e49418: f64 = (assign37230_e49416 * 0.3333333333333333);
        let assign37230_e49419: f64 = (1.0 + assign37230_e49418);
        let assign37230_e49420: f64 = (assign37230_e49408 * assign37230_e49419);
        let assign37230_e49421: f64 = (0.5 * assign37230_e49420);
        let assign37230_e49422: f64 = (1.0 + assign37230_e49421);
        let assign37230_e49423: f64 = (assign37230_e49399 * assign37230_e49422);
        let assign37230_e49424: f64 = (1.0 + assign37230_e49423);
        let assign37230_e49425: f64 = (1e-100 / assign37230_e49424);
        (assign37230_e49425, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign37230_e49422) + (assign37230_e49399 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign37230_e49419) + (assign37230_e49408 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign37230_e49424 * assign37230_e49424))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign37230_e49422) + (assign37230_e49399 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign37230_e49419) + (assign37230_e49408 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign37230_e49424 * assign37230_e49424))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign37230_e49422) + (assign37230_e49399 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign37230_e49419) + (assign37230_e49408 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign37230_e49424 * assign37230_e49424))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign37230_e49422) + (assign37230_e49399 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign37230_e49419) + (assign37230_e49408 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign37230_e49424 * assign37230_e49424))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign37230_e49427;
        var_tmp_dn6 = assign37230_e49427_d_n6;
        var_tmp_dn7 = assign37230_e49427_d_n7;
        var_tmp_dn8 = assign37230_e49427_d_n8;
        var_tmp_dn9 = assign37230_e49427_d_n9;

        let (assign37240_e49457, assign37240_e49457_d_n6, assign37240_e49457_d_n7, assign37240_e49457_d_n8, assign37240_e49457_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) {
        let assign37240_e49439: f64 = (0.29214664 * var_terfc);
        let assign37240_e49443: f64 = (var_terfc * var_terfc);
        let assign37240_e49444: f64 = (var_berfc * assign37240_e49443);
        let assign37240_e49445: f64 = (assign37240_e49439 + assign37240_e49444);
        let assign37240_e49449: f64 = (var_terfc * var_terfc);
        let assign37240_e49451: f64 = (assign37240_e49449 * var_terfc);
        let assign37240_e49452: f64 = (var_cerfc * assign37240_e49451);
        let assign37240_e49453: f64 = (assign37240_e49445 + assign37240_e49452);
        let assign37240_e49455: f64 = (assign37240_e49453 * var_tmp);
        (assign37240_e49455, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign37240_e49449 * var_terfc_dn6)))) * var_tmp) + (assign37240_e49453 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign37240_e49449 * var_terfc_dn7)))) * var_tmp) + (assign37240_e49453 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign37240_e49449 * var_terfc_dn8)))) * var_tmp) + (assign37240_e49453 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign37240_e49449 * var_terfc_dn9)))) * var_tmp) + (assign37240_e49453 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign37240_e49457;
        var_erfcpos_dn6 = assign37240_e49457_d_n6;
        var_erfcpos_dn7 = assign37240_e49457_d_n7;
        var_erfcpos_dn8 = assign37240_e49457_d_n8;
        var_erfcpos_dn9 = assign37240_e49457_d_n9;

        let assign37250_e49460: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard768 = assign37250_e49460;

        let (assign37260_e49474, assign37260_e49474_d_n6, assign37260_e49474_d_n7, assign37260_e49474_d_n8, assign37260_e49474_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) && (var_guard768 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign37260_e49474;
        var_erfctimesexpmtat_dn6 = assign37260_e49474_d_n6;
        var_erfctimesexpmtat_dn7 = assign37260_e49474_d_n7;
        var_erfctimesexpmtat_dn8 = assign37260_e49474_d_n8;
        var_erfctimesexpmtat_dn9 = assign37260_e49474_d_n9;

        let assign37270_e49477: f64 = (-230.25850929940458);
        let assign37270_e49478: f64 = if var_mtat > assign37270_e49477 { 1.0 } else { 0.0 };
        var_guard769 = assign37270_e49478;

        let (assign37280_e49496, assign37280_e49496_d_n6, assign37280_e49496_d_n7, assign37280_e49496_d_n8, assign37280_e49496_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) && (var_guard768 == 0.0)) && (var_guard769 != 0.0)) {
        let assign37280_e49494: f64 = (var_mtat).exp();
        (assign37280_e49494, (assign37280_e49494 * var_mtat_dn6), (assign37280_e49494 * var_mtat_dn7), (assign37280_e49494 * var_mtat_dn8), (assign37280_e49494 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign37280_e49496;
        var_tmp_dn6 = assign37280_e49496_d_n6;
        var_tmp_dn7 = assign37280_e49496_d_n7;
        var_tmp_dn8 = assign37280_e49496_d_n8;
        var_tmp_dn9 = assign37280_e49496_d_n9;

        let (assign37290_e49539, assign37290_e49539_d_n6, assign37290_e49539_d_n7, assign37290_e49539_d_n8, assign37290_e49539_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) && (var_guard768 == 0.0)) && (var_guard769 == 0.0)) {
        let assign37290_e49515: f64 = (-230.25850929940458);
        let assign37290_e49517: f64 = (assign37290_e49515 - var_mtat);
        let assign37290_e49521: f64 = (-230.25850929940458);
        let assign37290_e49523: f64 = (assign37290_e49521 - var_mtat);
        let assign37290_e49526: f64 = (-230.25850929940458);
        let assign37290_e49528: f64 = (assign37290_e49526 - var_mtat);
        let assign37290_e49530: f64 = (assign37290_e49528 * 0.3333333333333333);
        let assign37290_e49531: f64 = (1.0 + assign37290_e49530);
        let assign37290_e49532: f64 = (assign37290_e49523 * assign37290_e49531);
        let assign37290_e49533: f64 = (0.5 * assign37290_e49532);
        let assign37290_e49534: f64 = (1.0 + assign37290_e49533);
        let assign37290_e49535: f64 = (assign37290_e49517 * assign37290_e49534);
        let assign37290_e49536: f64 = (1.0 + assign37290_e49535);
        let assign37290_e49537: f64 = (1e-100 / assign37290_e49536);
        (assign37290_e49537, (-((1e-100 * (((-var_mtat_dn6) * assign37290_e49534) + (assign37290_e49517 * (0.5 * (((-var_mtat_dn6) * assign37290_e49531) + (assign37290_e49523 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign37290_e49536 * assign37290_e49536))), (-((1e-100 * (((-var_mtat_dn7) * assign37290_e49534) + (assign37290_e49517 * (0.5 * (((-var_mtat_dn7) * assign37290_e49531) + (assign37290_e49523 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign37290_e49536 * assign37290_e49536))), (-((1e-100 * (((-var_mtat_dn8) * assign37290_e49534) + (assign37290_e49517 * (0.5 * (((-var_mtat_dn8) * assign37290_e49531) + (assign37290_e49523 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign37290_e49536 * assign37290_e49536))), (-((1e-100 * (((-var_mtat_dn9) * assign37290_e49534) + (assign37290_e49517 * (0.5 * (((-var_mtat_dn9) * assign37290_e49531) + (assign37290_e49523 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign37290_e49536 * assign37290_e49536))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign37290_e49539;
        var_tmp_dn6 = assign37290_e49539_d_n6;
        var_tmp_dn7 = assign37290_e49539_d_n7;
        var_tmp_dn8 = assign37290_e49539_d_n8;
        var_tmp_dn9 = assign37290_e49539_d_n9;

        let (assign37300_e49558, assign37300_e49558_d_n6, assign37300_e49558_d_n7, assign37300_e49558_d_n8, assign37300_e49558_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) && (var_guard768 == 0.0)) {
        let assign37300_e49554: f64 = (2.0 * var_tmp);
        let assign37300_e49556: f64 = (assign37300_e49554 - var_erfcpos);
        (assign37300_e49556, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign37300_e49558;
        var_erfctimesexpmtat_dn6 = assign37300_e49558_d_n6;
        var_erfctimesexpmtat_dn7 = assign37300_e49558_d_n7;
        var_erfctimesexpmtat_dn8 = assign37300_e49558_d_n8;
        var_erfctimesexpmtat_dn9 = assign37300_e49558_d_n9;

        let (assign37310_e49578, assign37310_e49578_d_n6, assign37310_e49578_d_n7, assign37310_e49578_d_n8, assign37310_e49578_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) {
        let assign37310_e49570: f64 = (1.772453850905516 * 0.5);
        let assign37310_e49573: f64 = (var_atatbot_d * var_erfctimesexpmtat);
        let assign37310_e49575: f64 = (assign37310_e49573 / var_ktat);
        let assign37310_e49576: f64 = (assign37310_e49570 * assign37310_e49575);
        (assign37310_e49576, (assign37310_e49570 * ((((var_atatbot_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign37310_e49573 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign37310_e49570 * ((((var_atatbot_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign37310_e49573 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign37310_e49570 * ((((var_atatbot_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign37310_e49573 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign37310_e49570 * ((((var_atatbot_d * var_erfctimesexpmtat_dn9) * var_ktat) - (assign37310_e49573 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign37310_e49578;
        var_gammamax_dn6 = assign37310_e49578_d_n6;
        var_gammamax_dn7 = assign37310_e49578_d_n7;
        var_gammamax_dn8 = assign37310_e49578_d_n8;
        var_gammamax_dn9 = assign37310_e49578_d_n9;

        let (assign37320_e49596, assign37320_e49596_d_n6, assign37320_e49596_d_n7, assign37320_e49596_d_n8, assign37320_e49596_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard764 == 0.0)) {
        let assign37320_e49591: f64 = (var_asrh * var_gammamax);
        let assign37320_e49593: f64 = (assign37320_e49591 * var_wtat);
        let assign37320_e49594: f64 = (var_ctatbotd_i * assign37320_e49593);
        (assign37320_e49594, (var_ctatbotd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign37320_e49591 * var_wtat_dn6))), (var_ctatbotd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign37320_e49591 * var_wtat_dn7))), (var_ctatbotd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign37320_e49591 * var_wtat_dn8))), (var_ctatbotd_i * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign37320_e49591 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign37320_e49596;
        var_itat_dn6 = assign37320_e49596_d_n6;
        var_itat_dn7 = assign37320_e49596_d_n7;
        var_itat_dn8 = assign37320_e49596_d_n8;
        var_itat_dn9 = assign37320_e49596_d_n9;

        let assign37330_e49599: f64 = if var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard770 = assign37330_e49599;

        let (assign37340_e49610, assign37340_e49610_d_n6, assign37340_e49610_d_n7, assign37340_e49610_d_n8, assign37340_e49610_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard770 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign37340_e49610;
        var_ibbt_dn6 = assign37340_e49610_d_n6;
        var_ibbt_dn7 = assign37340_e49610_d_n7;
        var_ibbt_dn8 = assign37340_e49610_d_n8;
        var_ibbt_dn9 = assign37340_e49610_d_n9;

        let assign37350_e49613: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard771 = assign37350_e49613;

        let (assign37360_e49632, assign37360_e49632_d_n6, assign37360_e49632_d_n7, assign37360_e49632_d_n8, assign37360_e49632_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard770 == 0.0)) && (var_guard771 != 0.0)) {
        let assign37360_e49627: f64 = (var_vbirbotd_i - var_vbbt);
        let assign37360_e49629: f64 = (assign37360_e49627 * var_vbirbotinv_d);
        let assign37360_e49630: f64 = (assign37360_e49629).sqrt();
        (assign37360_e49630, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign37360_e49632;
        var_tmp_dn6 = assign37360_e49632_d_n6;
        var_tmp_dn7 = assign37360_e49632_d_n7;
        var_tmp_dn8 = assign37360_e49632_d_n8;
        var_tmp_dn9 = assign37360_e49632_d_n9;

        let (assign37370_e49653, assign37370_e49653_d_n6, assign37370_e49653_d_n7, assign37370_e49653_d_n8, assign37370_e49653_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard770 == 0.0)) && (var_guard771 == 0.0)) {
        let assign37370_e49647: f64 = (var_vbirbotd_i - var_vbbt);
        let assign37370_e49649: f64 = (assign37370_e49647 * var_vbirbotinv_d);
        let assign37370_e49651: f64 = (assign37370_e49649).powf(var_pbotd_i);
        (assign37370_e49651, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign37370_e49653;
        var_tmp_dn6 = assign37370_e49653_d_n6;
        var_tmp_dn7 = assign37370_e49653_d_n7;
        var_tmp_dn8 = assign37370_e49653_d_n8;
        var_tmp_dn9 = assign37370_e49653_d_n9;

        let (assign37380_e49673, assign37380_e49673_d_n6, assign37380_e49673_d_n7, assign37380_e49673_d_n8, assign37380_e49673_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard770 == 0.0)) {
        let assign37380_e49666: f64 = (var_vbirbotd_i - var_vbbt);
        let assign37380_e49668: f64 = (assign37380_e49666 * var_wdepnulrinvbot_d);
        let assign37380_e49670: f64 = (assign37380_e49668 / var_tmp);
        let assign37380_e49671: f64 = (var_one_over_one_minus_pbot_d * assign37380_e49670);
        (assign37380_e49671, (var_one_over_one_minus_pbot_d * (-((assign37380_e49668 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign37380_e49668 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign37380_e49668 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign37380_e49668 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign37380_e49673;
        var_fmaxr_dn6 = assign37380_e49673_d_n6;
        var_fmaxr_dn7 = assign37380_e49673_d_n7;
        var_fmaxr_dn8 = assign37380_e49673_d_n8;
        var_fmaxr_dn9 = assign37380_e49673_d_n9;

        let assign37390_e49675: f64 = (-var_fbbtbot_d);
        let assign37390_e49677: f64 = (assign37390_e49675 / var_fmaxr);
        let assign37390_e49678: f64 = (assign37390_e49677).abs();
        let assign37390_e49680: f64 = if assign37390_e49678 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard772 = assign37390_e49680;

        let (assign37400_e49698, assign37400_e49698_d_n6, assign37400_e49698_d_n7, assign37400_e49698_d_n8, assign37400_e49698_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard770 == 0.0)) && (var_guard772 != 0.0)) {
        let assign37400_e49693: f64 = (-var_fbbtbot_d);
        let assign37400_e49695: f64 = (assign37400_e49693 / var_fmaxr);
        let assign37400_e49696: f64 = (assign37400_e49695).exp();
        (assign37400_e49696, (assign37400_e49696 * (-((assign37400_e49693 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign37400_e49696 * (-((assign37400_e49693 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign37400_e49696 * (-((assign37400_e49693 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign37400_e49696 * (-((assign37400_e49693 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign37400_e49698;
        var_tmp_dn6 = assign37400_e49698_d_n6;
        var_tmp_dn7 = assign37400_e49698_d_n7;
        var_tmp_dn8 = assign37400_e49698_d_n8;
        var_tmp_dn9 = assign37400_e49698_d_n9;

        let assign37410_e49700: f64 = (-var_fbbtbot_d);
        let assign37410_e49702: f64 = (assign37410_e49700 / var_fmaxr);
        let assign37410_e49704: f64 = if assign37410_e49702 < 0.0 { 1.0 } else { 0.0 };
        var_guard773 = assign37410_e49704;

        let (assign37420_e49755, assign37420_e49755_d_n6, assign37420_e49755_d_n7, assign37420_e49755_d_n8, assign37420_e49755_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard770 == 0.0)) && (var_guard772 == 0.0)) && (var_guard773 != 0.0)) {
        let assign37420_e49722: f64 = (-230.25850929940458);
        let assign37420_e49724: f64 = (-var_fbbtbot_d);
        let assign37420_e49726: f64 = (assign37420_e49724 / var_fmaxr);
        let assign37420_e49727: f64 = (assign37420_e49722 - assign37420_e49726);
        let assign37420_e49731: f64 = (-230.25850929940458);
        let assign37420_e49733: f64 = (-var_fbbtbot_d);
        let assign37420_e49735: f64 = (assign37420_e49733 / var_fmaxr);
        let assign37420_e49736: f64 = (assign37420_e49731 - assign37420_e49735);
        let assign37420_e49739: f64 = (-230.25850929940458);
        let assign37420_e49741: f64 = (-var_fbbtbot_d);
        let assign37420_e49743: f64 = (assign37420_e49741 / var_fmaxr);
        let assign37420_e49744: f64 = (assign37420_e49739 - assign37420_e49743);
        let assign37420_e49746: f64 = (assign37420_e49744 * 0.3333333333333333);
        let assign37420_e49747: f64 = (1.0 + assign37420_e49746);
        let assign37420_e49748: f64 = (assign37420_e49736 * assign37420_e49747);
        let assign37420_e49749: f64 = (0.5 * assign37420_e49748);
        let assign37420_e49750: f64 = (1.0 + assign37420_e49749);
        let assign37420_e49751: f64 = (assign37420_e49727 * assign37420_e49750);
        let assign37420_e49752: f64 = (1.0 + assign37420_e49751);
        let assign37420_e49753: f64 = (1e-100 / assign37420_e49752);
        (assign37420_e49753, (-((1e-100 * (((-(-((assign37420_e49724 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign37420_e49750) + (assign37420_e49727 * (0.5 * (((-(-((assign37420_e49733 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign37420_e49747) + (assign37420_e49736 * ((-(-((assign37420_e49741 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign37420_e49752 * assign37420_e49752))), (-((1e-100 * (((-(-((assign37420_e49724 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign37420_e49750) + (assign37420_e49727 * (0.5 * (((-(-((assign37420_e49733 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign37420_e49747) + (assign37420_e49736 * ((-(-((assign37420_e49741 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign37420_e49752 * assign37420_e49752))), (-((1e-100 * (((-(-((assign37420_e49724 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign37420_e49750) + (assign37420_e49727 * (0.5 * (((-(-((assign37420_e49733 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign37420_e49747) + (assign37420_e49736 * ((-(-((assign37420_e49741 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign37420_e49752 * assign37420_e49752))), (-((1e-100 * (((-(-((assign37420_e49724 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign37420_e49750) + (assign37420_e49727 * (0.5 * (((-(-((assign37420_e49733 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign37420_e49747) + (assign37420_e49736 * ((-(-((assign37420_e49741 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign37420_e49752 * assign37420_e49752))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign37420_e49755;
        var_tmp_dn6 = assign37420_e49755_d_n6;
        var_tmp_dn7 = assign37420_e49755_d_n7;
        var_tmp_dn8 = assign37420_e49755_d_n8;
        var_tmp_dn9 = assign37420_e49755_d_n9;

        let (assign37430_e49804, assign37430_e49804_d_n6, assign37430_e49804_d_n7, assign37430_e49804_d_n8, assign37430_e49804_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard770 == 0.0)) && (var_guard772 == 0.0)) && (var_guard773 == 0.0)) {
        let assign37430_e49774: f64 = (-var_fbbtbot_d);
        let assign37430_e49776: f64 = (assign37430_e49774 / var_fmaxr);
        let assign37430_e49778: f64 = (assign37430_e49776 - 230.25850929940458);
        let assign37430_e49782: f64 = (-var_fbbtbot_d);
        let assign37430_e49784: f64 = (assign37430_e49782 / var_fmaxr);
        let assign37430_e49786: f64 = (assign37430_e49784 - 230.25850929940458);
        let assign37430_e49789: f64 = (-var_fbbtbot_d);
        let assign37430_e49791: f64 = (assign37430_e49789 / var_fmaxr);
        let assign37430_e49793: f64 = (assign37430_e49791 - 230.25850929940458);
        let assign37430_e49795: f64 = (assign37430_e49793 * 0.3333333333333333);
        let assign37430_e49796: f64 = (1.0 + assign37430_e49795);
        let assign37430_e49797: f64 = (assign37430_e49786 * assign37430_e49796);
        let assign37430_e49798: f64 = (0.5 * assign37430_e49797);
        let assign37430_e49799: f64 = (1.0 + assign37430_e49798);
        let assign37430_e49800: f64 = (assign37430_e49778 * assign37430_e49799);
        let assign37430_e49801: f64 = (1.0 + assign37430_e49800);
        let assign37430_e49802: f64 = (1e100 * assign37430_e49801);
        (assign37430_e49802, (1e100 * (((-((assign37430_e49774 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign37430_e49799) + (assign37430_e49778 * (0.5 * (((-((assign37430_e49782 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign37430_e49796) + (assign37430_e49786 * ((-((assign37430_e49789 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign37430_e49774 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign37430_e49799) + (assign37430_e49778 * (0.5 * (((-((assign37430_e49782 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign37430_e49796) + (assign37430_e49786 * ((-((assign37430_e49789 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign37430_e49774 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign37430_e49799) + (assign37430_e49778 * (0.5 * (((-((assign37430_e49782 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign37430_e49796) + (assign37430_e49786 * ((-((assign37430_e49789 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign37430_e49774 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign37430_e49799) + (assign37430_e49778 * (0.5 * (((-((assign37430_e49782 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign37430_e49796) + (assign37430_e49786 * ((-((assign37430_e49789 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign37430_e49804;
        var_tmp_dn6 = assign37430_e49804_d_n6;
        var_tmp_dn7 = assign37430_e49804_d_n7;
        var_tmp_dn8 = assign37430_e49804_d_n8;
        var_tmp_dn9 = assign37430_e49804_d_n9;

        let (assign37440_e49824, assign37440_e49824_d_n6, assign37440_e49824_d_n7, assign37440_e49824_d_n8, assign37440_e49824_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard770 == 0.0)) {
        let assign37440_e49817: f64 = (var_v5 * var_fmaxr);
        let assign37440_e49819: f64 = (assign37440_e49817 * var_fmaxr);
        let assign37440_e49821: f64 = (assign37440_e49819 * var_tmp);
        let assign37440_e49822: f64 = (var_cbbtbotd_i * assign37440_e49821);
        (assign37440_e49822, (var_cbbtbotd_i * (((((var_v5 * var_fmaxr_dn6) * var_fmaxr) + (assign37440_e49817 * var_fmaxr_dn6)) * var_tmp) + (assign37440_e49819 * var_tmp_dn6))), (var_cbbtbotd_i * (((((var_v5 * var_fmaxr_dn7) * var_fmaxr) + (assign37440_e49817 * var_fmaxr_dn7)) * var_tmp) + (assign37440_e49819 * var_tmp_dn7))), (var_cbbtbotd_i * (((((var_v5 * var_fmaxr_dn8) * var_fmaxr) + (assign37440_e49817 * var_fmaxr_dn8)) * var_tmp) + (assign37440_e49819 * var_tmp_dn8))), (var_cbbtbotd_i * (((((var_v5 * var_fmaxr_dn9) * var_fmaxr) + (assign37440_e49817 * var_fmaxr_dn9)) * var_tmp) + (assign37440_e49819 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign37440_e49824;
        var_ibbt_dn6 = assign37440_e49824_d_n6;
        var_ibbt_dn7 = assign37440_e49824_d_n7;
        var_ibbt_dn8 = assign37440_e49824_d_n8;
        var_ibbt_dn9 = assign37440_e49824_d_n9;

        let assign37450_e49827: f64 = if var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard774 = assign37450_e49827;

        let (assign37460_e49838, assign37460_e49838_d_n6, assign37460_e49838_d_n7, assign37460_e49838_d_n8, assign37460_e49838_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard774 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign37460_e49838;
        var_fbreakdown_dn6 = assign37460_e49838_d_n6;
        var_fbreakdown_dn7 = assign37460_e49838_d_n7;
        var_fbreakdown_dn8 = assign37460_e49838_d_n8;
        var_fbreakdown_dn9 = assign37460_e49838_d_n9;

        let assign37470_e49841: f64 = (-var_alphaav);
        let assign37470_e49843: f64 = (assign37470_e49841 * var_vbrbotd_i);
        let assign37470_e49844: f64 = if var_vav > assign37470_e49843 { 1.0 } else { 0.0 };
        var_guard775 = assign37470_e49844;

        let assign37480_e49847: f64 = if var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard776 = assign37480_e49847;

        let (assign37490_e49877, assign37490_e49877_d_n6, assign37490_e49877_d_n7, assign37490_e49877_d_n8, assign37490_e49877_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard774 == 0.0)) && (var_guard775 != 0.0)) && (var_guard776 != 0.0)) {
        let assign37490_e49863: f64 = (var_vav * var_vbrinvbot_d);
        let assign37490_e49866: f64 = (var_vav * var_vbrinvbot_d);
        let assign37490_e49867: f64 = (assign37490_e49863 * assign37490_e49866);
        let assign37490_e49870: f64 = (var_vav * var_vbrinvbot_d);
        let assign37490_e49871: f64 = (assign37490_e49867 * assign37490_e49870);
        let assign37490_e49874: f64 = (var_vav * var_vbrinvbot_d);
        let assign37490_e49875: f64 = (assign37490_e49871 * assign37490_e49874);
        (assign37490_e49875, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign37490_e49877;
        var_tmp_dn6 = assign37490_e49877_d_n6;
        var_tmp_dn7 = assign37490_e49877_d_n7;
        var_tmp_dn8 = assign37490_e49877_d_n8;
        var_tmp_dn9 = assign37490_e49877_d_n9;

        let (assign37500_e49899, assign37500_e49899_d_n6, assign37500_e49899_d_n7, assign37500_e49899_d_n8, assign37500_e49899_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard774 == 0.0)) && (var_guard775 != 0.0)) && (var_guard776 == 0.0)) {
        let assign37500_e49894: f64 = (var_vav * var_vbrinvbot_d);
        let assign37500_e49895: f64 = (assign37500_e49894).abs();
        let assign37500_e49897: f64 = (assign37500_e49895).powf(var_pbrbotd_i);
        (assign37500_e49897, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign37500_e49899;
        var_tmp_dn6 = assign37500_e49899_d_n6;
        var_tmp_dn7 = assign37500_e49899_d_n7;
        var_tmp_dn8 = assign37500_e49899_d_n8;
        var_tmp_dn9 = assign37500_e49899_d_n9;

        let (assign37510_e49917, assign37510_e49917_d_n6, assign37510_e49917_d_n7, assign37510_e49917_d_n8, assign37510_e49917_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard774 == 0.0)) && (var_guard775 != 0.0)) {
        let assign37510_e49914: f64 = (1.0 - var_tmp);
        let assign37510_e49915: f64 = (1.0 / assign37510_e49914);
        (assign37510_e49915, (-((-var_tmp_dn6) / (assign37510_e49914 * assign37510_e49914))), (-((-var_tmp_dn7) / (assign37510_e49914 * assign37510_e49914))), (-((-var_tmp_dn8) / (assign37510_e49914 * assign37510_e49914))), (-((-var_tmp_dn9) / (assign37510_e49914 * assign37510_e49914))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign37510_e49917;
        var_fbreakdown_dn6 = assign37510_e49917_d_n6;
        var_fbreakdown_dn7 = assign37510_e49917_d_n7;
        var_fbreakdown_dn8 = assign37510_e49917_d_n8;
        var_fbreakdown_dn9 = assign37510_e49917_d_n9;

        let (assign37520_e49940, assign37520_e49940_d_n6, assign37520_e49940_d_n7, assign37520_e49940_d_n8, assign37520_e49940_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) && (var_guard774 == 0.0)) && (var_guard775 == 0.0)) {
        let assign37520_e49934: f64 = (var_alphaav * var_vbrbotd_i);
        let assign37520_e49935: f64 = (var_vav + assign37520_e49934);
        let assign37520_e49937: f64 = (assign37520_e49935 * var_slopebot_d);
        let assign37520_e49938: f64 = (var_fstopbot_d + assign37520_e49937);
        (assign37520_e49938, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign37520_e49940;
        var_fbreakdown_dn6 = assign37520_e49940_d_n6;
        var_fbreakdown_dn7 = assign37520_e49940_d_n7;
        var_fbreakdown_dn8 = assign37520_e49940_d_n8;
        var_fbreakdown_dn9 = assign37520_e49940_d_n9;

        let (assign37530_e49959, assign37530_e49959_d_n6, assign37530_e49959_d_n7, assign37530_e49959_d_n8, assign37530_e49959_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard760 == 0.0)) {
        let assign37530_e49950: f64 = (var_id__blk212 + var_isrh);
        let assign37530_e49952: f64 = (assign37530_e49950 + var_itat);
        let assign37530_e49954: f64 = (assign37530_e49952 + var_ibbt);
        let assign37530_e49955: f64 = (p.p29 * assign37530_e49954);
        let assign37530_e49957: f64 = (assign37530_e49955 * var_fbreakdown);
        (assign37530_e49957, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign37530_e49955 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign37530_e49955 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign37530_e49955 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign37530_e49955 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign37530_e49959;
        var_ijunbot_dn6 = assign37530_e49959_d_n6;
        var_ijunbot_dn7 = assign37530_e49959_d_n7;
        var_ijunbot_dn8 = assign37530_e49959_d_n8;
        var_ijunbot_dn9 = assign37530_e49959_d_n9;

        let assign37540_e49962: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard777 = assign37540_e49962;

        let (assign37550_e49970, assign37550_e49970_d_n6, assign37550_e49970_d_n7, assign37550_e49970_d_n8, assign37550_e49970_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign37550_e49970;
        var_ijunsti_dn6 = assign37550_e49970_d_n6;
        var_ijunsti_dn7 = assign37550_e49970_d_n7;
        var_ijunsti_dn8 = assign37550_e49970_d_n8;
        var_ijunsti_dn9 = assign37550_e49970_d_n9;

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
        *var_guard767_slot = var_guard767;
        *var_guard768_slot = var_guard768;
        *var_guard769_slot = var_guard769;
        *var_guard770_slot = var_guard770;
        *var_guard771_slot = var_guard771;
        *var_guard772_slot = var_guard772;
        *var_guard773_slot = var_guard773;
        *var_guard774_slot = var_guard774;
        *var_guard775_slot = var_guard775;
        *var_guard776_slot = var_guard776;
        *var_guard777_slot = var_guard777;
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
