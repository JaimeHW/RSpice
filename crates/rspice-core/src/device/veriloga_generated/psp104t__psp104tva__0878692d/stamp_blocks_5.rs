#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_80(
        var_atatsti_d: f64,
        var_btatpartsti_d: f64,
        var_csrhstid_i: f64,
        var_ctatstid_i: f64,
        var_ftdsti_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard777: f64,
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
        var_guard778_slot: &mut f64,
        var_guard779_slot: &mut f64,
        var_guard780_slot: &mut f64,
        var_guard781_slot: &mut f64,
        var_guard782_slot: &mut f64,
        var_guard783_slot: &mut f64,
        var_guard784_slot: &mut f64,
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
        let mut var_guard778: f64 = *var_guard778_slot;
        let mut var_guard779: f64 = *var_guard779_slot;
        let mut var_guard780: f64 = *var_guard780_slot;
        let mut var_guard781: f64 = *var_guard781_slot;
        let mut var_guard782: f64 = *var_guard782_slot;
        let mut var_guard783: f64 = *var_guard783_slot;
        let mut var_guard784: f64 = *var_guard784_slot;
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

        let (assign37560_e49981,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) {
        let assign37560_e49979: f64 = (var_idsatsti_d * var_idmult);
        (assign37560_e49979,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign37560_e49981;

        let assign37570_e49988: f64 = if ((var_csrhstid_i == 0.0) && (var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard778 = assign37570_e49988;

        let (assign37580_e49999, assign37580_e49999_d_n6, assign37580_e49999_d_n7, assign37580_e49999_d_n8, assign37580_e49999_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard778 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign37580_e49999;
        var_isrh_dn6 = assign37580_e49999_d_n6;
        var_isrh_dn7 = assign37580_e49999_d_n7;
        var_isrh_dn8 = assign37580_e49999_d_n8;
        var_isrh_dn9 = assign37580_e49999_d_n9;

        let (assign37590_e50013,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard778 == 0.0)) {
        let assign37590_e50011: f64 = (var_vbisti_d - var_vjsrh);
        (assign37590_e50011,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign37590_e50013;

        let (assign37600_e50032,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard778 == 0.0)) {
        let assign37600_e50027: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign37600_e50028: f64 = (1.0 - assign37600_e50027);
        let assign37600_e50029: f64 = (assign37600_e50028).sqrt();
        let assign37600_e50030: f64 = (1.0 - assign37600_e50029);
        (assign37600_e50030,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign37600_e50032;

        let assign37610_e50035: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard779 = assign37610_e50035;

        let (assign37620_e50049,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard778 == 0.0)) && (var_guard779 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign37620_e50049;

        let (assign37630_e50081,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard778 == 0.0)) && (var_guard779 == 0.0)) {
        let assign37630_e50064: f64 = (var_wsrhstep * var_wsrhstep);
        let assign37630_e50066: f64 = (var_wsrhstep).ln();
        let assign37630_e50067: f64 = (assign37630_e50064 * assign37630_e50066);
        let assign37630_e50070: f64 = (1.0 - var_wsrhstep);
        let assign37630_e50071: f64 = (assign37630_e50067 / assign37630_e50070);
        let assign37630_e50073: f64 = (assign37630_e50071 + var_wsrhstep);
        let assign37630_e50077: f64 = (2.0 * var_pstid_i);
        let assign37630_e50078: f64 = (1.0 - assign37630_e50077);
        let assign37630_e50079: f64 = (assign37630_e50073 * assign37630_e50078);
        (assign37630_e50079,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign37630_e50081;

        let (assign37640_e50095,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard778 == 0.0)) {
        let assign37640_e50093: f64 = (var_wsrhstep + var_dwsrh);
        (assign37640_e50093,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign37640_e50095;

        let assign37650_e50098: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard780 = assign37650_e50098;

        let (assign37660_e50115, assign37660_e50115_d_n6, assign37660_e50115_d_n7, assign37660_e50115_d_n8, assign37660_e50115_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard778 == 0.0)) && (var_guard780 != 0.0)) {
        let assign37660_e50112: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign37660_e50113: f64 = (assign37660_e50112).sqrt();
        (assign37660_e50113, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign37660_e50115;
        var_tmp_dn6 = assign37660_e50115_d_n6;
        var_tmp_dn7 = assign37660_e50115_d_n7;
        var_tmp_dn8 = assign37660_e50115_d_n8;
        var_tmp_dn9 = assign37660_e50115_d_n9;

        let (assign37670_e50134, assign37670_e50134_d_n6, assign37670_e50134_d_n7, assign37670_e50134_d_n8, assign37670_e50134_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard778 == 0.0)) && (var_guard780 == 0.0)) {
        let assign37670_e50130: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign37670_e50132: f64 = (assign37670_e50130).powf(var_pstid_i);
        (assign37670_e50132, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign37670_e50134;
        var_tmp_dn6 = assign37670_e50134_d_n6;
        var_tmp_dn7 = assign37670_e50134_d_n7;
        var_tmp_dn8 = assign37670_e50134_d_n8;
        var_tmp_dn9 = assign37670_e50134_d_n9;

        let (assign37680_e50148, assign37680_e50148_d_n6, assign37680_e50148_d_n7, assign37680_e50148_d_n8, assign37680_e50148_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard778 == 0.0)) {
        let assign37680_e50146: f64 = (var_wdepnulrsti_d * var_tmp);
        (assign37680_e50146, (var_wdepnulrsti_d * var_tmp_dn6), (var_wdepnulrsti_d * var_tmp_dn7), (var_wdepnulrsti_d * var_tmp_dn8), (var_wdepnulrsti_d * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign37680_e50148;
        var_wdep_dn6 = assign37680_e50148_d_n6;
        var_wdep_dn7 = assign37680_e50148_d_n7;
        var_wdep_dn8 = assign37680_e50148_d_n8;
        var_wdep_dn9 = assign37680_e50148_d_n9;

        let (assign37690_e50166, assign37690_e50166_d_n6, assign37690_e50166_d_n7, assign37690_e50166_d_n8, assign37690_e50166_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard778 == 0.0)) {
        let assign37690_e50161: f64 = (var_zinv - 1.0);
        let assign37690_e50163: f64 = (assign37690_e50161 * var_wdep);
        let assign37690_e50164: f64 = (var_ftdsti_d * assign37690_e50163);
        (assign37690_e50164, (var_ftdsti_d * (assign37690_e50161 * var_wdep_dn6)), (var_ftdsti_d * (assign37690_e50161 * var_wdep_dn7)), (var_ftdsti_d * (assign37690_e50161 * var_wdep_dn8)), (var_ftdsti_d * (assign37690_e50161 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign37690_e50166;
        var_asrh_dn6 = assign37690_e50166_d_n6;
        var_asrh_dn7 = assign37690_e50166_d_n7;
        var_asrh_dn8 = assign37690_e50166_d_n8;
        var_asrh_dn9 = assign37690_e50166_d_n9;

        let (assign37700_e50182, assign37700_e50182_d_n6, assign37700_e50182_d_n7, assign37700_e50182_d_n8, assign37700_e50182_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard778 == 0.0)) {
        let assign37700_e50179: f64 = (var_asrh * var_wsrh);
        let assign37700_e50180: f64 = (var_csrhstid_i * assign37700_e50179);
        (assign37700_e50180, (var_csrhstid_i * (var_asrh_dn6 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn7 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn8 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign37700_e50182;
        var_isrh_dn6 = assign37700_e50182_d_n6;
        var_isrh_dn7 = assign37700_e50182_d_n7;
        var_isrh_dn8 = assign37700_e50182_d_n8;
        var_isrh_dn9 = assign37700_e50182_d_n9;

        let assign37710_e50185: f64 = if var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard781 = assign37710_e50185;

        let (assign37720_e50196, assign37720_e50196_d_n6, assign37720_e50196_d_n7, assign37720_e50196_d_n8, assign37720_e50196_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign37720_e50196;
        var_itat_dn6 = assign37720_e50196_d_n6;
        var_itat_dn7 = assign37720_e50196_d_n7;
        var_itat_dn8 = assign37720_e50196_d_n8;
        var_itat_dn9 = assign37720_e50196_d_n9;

        let (assign37730_e50214, assign37730_e50214_d_n6, assign37730_e50214_d_n7, assign37730_e50214_d_n8, assign37730_e50214_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) {
        let assign37730_e50209: f64 = (var_wdep * var_one_minus_psti_d);
        let assign37730_e50211: f64 = (assign37730_e50209 / var_vbi_minus_vjsrh);
        let assign37730_e50212: f64 = (var_btatpartsti_d * assign37730_e50211);
        (assign37730_e50212, (var_btatpartsti_d * ((var_wdep_dn6 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn7 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn8 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn9 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign37730_e50214;
        var_btat_dn6 = assign37730_e50214_d_n6;
        var_btat_dn7 = assign37730_e50214_d_n7;
        var_btat_dn8 = assign37730_e50214_d_n8;
        var_btat_dn9 = assign37730_e50214_d_n9;

        let (assign37740_e50230, assign37740_e50230_d_n6, assign37740_e50230_d_n7, assign37740_e50230_d_n8, assign37740_e50230_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) {
        let assign37740_e50226: f64 = (0.666666666666667 * var_atatsti_d);
        let assign37740_e50228: f64 = (assign37740_e50226 / var_btat);
        (assign37740_e50228, (-((assign37740_e50226 * var_btat_dn6) / (var_btat * var_btat))), (-((assign37740_e50226 * var_btat_dn7) / (var_btat * var_btat))), (-((assign37740_e50226 * var_btat_dn8) / (var_btat * var_btat))), (-((assign37740_e50226 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign37740_e50230;
        var_twoatatoverthreebtat_dn6 = assign37740_e50230_d_n6;
        var_twoatatoverthreebtat_dn7 = assign37740_e50230_d_n7;
        var_twoatatoverthreebtat_dn8 = assign37740_e50230_d_n8;
        var_twoatatoverthreebtat_dn9 = assign37740_e50230_d_n9;

        let (assign37750_e50244, assign37750_e50244_d_n6, assign37750_e50244_d_n7, assign37750_e50244_d_n8, assign37750_e50244_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) {
        let assign37750_e50242: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign37750_e50242, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign37750_e50244;
        var_umaxbeforelimiting_dn6 = assign37750_e50244_d_n6;
        var_umaxbeforelimiting_dn7 = assign37750_e50244_d_n7;
        var_umaxbeforelimiting_dn8 = assign37750_e50244_d_n8;
        var_umaxbeforelimiting_dn9 = assign37750_e50244_d_n9;

        let (assign37760_e50265, assign37760_e50265_d_n6, assign37760_e50265_d_n7, assign37760_e50265_d_n8, assign37760_e50265_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) {
        let assign37760_e50256: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign37760_e50259: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign37760_e50261: f64 = (assign37760_e50259 + 1.0);
        let assign37760_e50262: f64 = (assign37760_e50256 / assign37760_e50261);
        let assign37760_e50263: f64 = (assign37760_e50262).sqrt();
        (assign37760_e50263, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign37760_e50261) - (assign37760_e50256 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign37760_e50261 * assign37760_e50261)) / (2.0 * assign37760_e50263)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign37760_e50261) - (assign37760_e50256 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign37760_e50261 * assign37760_e50261)) / (2.0 * assign37760_e50263)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign37760_e50261) - (assign37760_e50256 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign37760_e50261 * assign37760_e50261)) / (2.0 * assign37760_e50263)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign37760_e50261) - (assign37760_e50256 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign37760_e50261 * assign37760_e50261)) / (2.0 * assign37760_e50263)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign37760_e50265;
        var_umax_dn6 = assign37760_e50265_d_n6;
        var_umax_dn7 = assign37760_e50265_d_n7;
        var_umax_dn8 = assign37760_e50265_d_n8;
        var_umax_dn9 = assign37760_e50265_d_n9;

        let (assign37770_e50278, assign37770_e50278_d_n6, assign37770_e50278_d_n7, assign37770_e50278_d_n8, assign37770_e50278_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) {
        let assign37770_e50276: f64 = (var_umax).sqrt();
        (assign37770_e50276, (var_umax_dn6 / (2.0 * assign37770_e50276)), (var_umax_dn7 / (2.0 * assign37770_e50276)), (var_umax_dn8 / (2.0 * assign37770_e50276)), (var_umax_dn9 / (2.0 * assign37770_e50276)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign37770_e50278;
        var_sqrtumax_dn6 = assign37770_e50278_d_n6;
        var_sqrtumax_dn7 = assign37770_e50278_d_n7;
        var_sqrtumax_dn8 = assign37770_e50278_d_n8;
        var_sqrtumax_dn9 = assign37770_e50278_d_n9;

        let (assign37780_e50292, assign37780_e50292_d_n6, assign37780_e50292_d_n7, assign37780_e50292_d_n8, assign37780_e50292_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) {
        let assign37780_e50290: f64 = (var_umax * var_sqrtumax);
        (assign37780_e50290, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign37780_e50292;
        var_umaxpoweronepointfive_dn6 = assign37780_e50292_d_n6;
        var_umaxpoweronepointfive_dn7 = assign37780_e50292_d_n7;
        var_umaxpoweronepointfive_dn8 = assign37780_e50292_d_n8;
        var_umaxpoweronepointfive_dn9 = assign37780_e50292_d_n9;

        let assign37790_e50294: f64 = (-var_pstid_i);
        let assign37790_e50296: f64 = (assign37790_e50294 * var_one_over_one_minus_psti_d);
        let assign37790_e50298: f64 = (-1.0);
        let assign37790_e50299: f64 = if assign37790_e50296 == assign37790_e50298 { 1.0 } else { 0.0 };
        var_guard782 = assign37790_e50299;

        let (assign37800_e50319, assign37800_e50319_d_n6, assign37800_e50319_d_n7, assign37800_e50319_d_n8, assign37800_e50319_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) && (var_guard782 != 0.0)) {
        let assign37800_e50315: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign37800_e50316: f64 = (1.0 + assign37800_e50315);
        let assign37800_e50317: f64 = (1.0 / assign37800_e50316);
        (assign37800_e50317, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign37800_e50316 * assign37800_e50316))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign37800_e50316 * assign37800_e50316))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign37800_e50316 * assign37800_e50316))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign37800_e50316 * assign37800_e50316))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign37800_e50319;
        var_wgamma_dn6 = assign37800_e50319_d_n6;
        var_wgamma_dn7 = assign37800_e50319_d_n7;
        var_wgamma_dn8 = assign37800_e50319_d_n8;
        var_wgamma_dn9 = assign37800_e50319_d_n9;

        let (assign37810_e50343, assign37810_e50343_d_n6, assign37810_e50343_d_n7, assign37810_e50343_d_n8, assign37810_e50343_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) && (var_guard782 == 0.0)) {
        let assign37810_e50335: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign37810_e50336: f64 = (1.0 + assign37810_e50335);
        let assign37810_e50338: f64 = (-var_pstid_i);
        let assign37810_e50340: f64 = (assign37810_e50338 * var_one_over_one_minus_psti_d);
        let assign37810_e50341: f64 = (assign37810_e50336).powf(assign37810_e50340);
        (assign37810_e50341, if 0.0 == 0.0 && ((assign37810_e50340) as f64).is_finite() && ((assign37810_e50340) as f64).fract() == 0.0 { if assign37810_e50340 == 0.0 { 0.0 } else { (assign37810_e50340 * ((assign37810_e50336).powf(assign37810_e50340 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign37810_e50341 * (assign37810_e50340 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign37810_e50336))) }, if 0.0 == 0.0 && ((assign37810_e50340) as f64).is_finite() && ((assign37810_e50340) as f64).fract() == 0.0 { if assign37810_e50340 == 0.0 { 0.0 } else { (assign37810_e50340 * ((assign37810_e50336).powf(assign37810_e50340 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign37810_e50341 * (assign37810_e50340 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign37810_e50336))) }, if 0.0 == 0.0 && ((assign37810_e50340) as f64).is_finite() && ((assign37810_e50340) as f64).fract() == 0.0 { if assign37810_e50340 == 0.0 { 0.0 } else { (assign37810_e50340 * ((assign37810_e50336).powf(assign37810_e50340 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign37810_e50341 * (assign37810_e50340 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign37810_e50336))) }, if 0.0 == 0.0 && ((assign37810_e50340) as f64).is_finite() && ((assign37810_e50340) as f64).fract() == 0.0 { if assign37810_e50340 == 0.0 { 0.0 } else { (assign37810_e50340 * ((assign37810_e50336).powf(assign37810_e50340 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign37810_e50341 * (assign37810_e50340 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign37810_e50336))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign37810_e50343;
        var_wgamma_dn6 = assign37810_e50343_d_n6;
        var_wgamma_dn7 = assign37810_e50343_d_n7;
        var_wgamma_dn8 = assign37810_e50343_d_n8;
        var_wgamma_dn9 = assign37810_e50343_d_n9;

        let (assign37820_e50361, assign37820_e50361_d_n6, assign37820_e50361_d_n7, assign37820_e50361_d_n8, assign37820_e50361_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) {
        let assign37820_e50355: f64 = (var_wsrh * var_wgamma);
        let assign37820_e50358: f64 = (var_wsrh + var_wgamma);
        let assign37820_e50359: f64 = (assign37820_e50355 / assign37820_e50358);
        (assign37820_e50359, ((((var_wsrh * var_wgamma_dn6) * assign37820_e50358) - (assign37820_e50355 * var_wgamma_dn6)) / (assign37820_e50358 * assign37820_e50358)), ((((var_wsrh * var_wgamma_dn7) * assign37820_e50358) - (assign37820_e50355 * var_wgamma_dn7)) / (assign37820_e50358 * assign37820_e50358)), ((((var_wsrh * var_wgamma_dn8) * assign37820_e50358) - (assign37820_e50355 * var_wgamma_dn8)) / (assign37820_e50358 * assign37820_e50358)), ((((var_wsrh * var_wgamma_dn9) * assign37820_e50358) - (assign37820_e50355 * var_wgamma_dn9)) / (assign37820_e50358 * assign37820_e50358)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign37820_e50361;
        var_wtat_dn6 = assign37820_e50361_d_n6;
        var_wtat_dn7 = assign37820_e50361_d_n7;
        var_wtat_dn8 = assign37820_e50361_d_n8;
        var_wtat_dn9 = assign37820_e50361_d_n9;

        let (assign37830_e50378, assign37830_e50378_d_n6, assign37830_e50378_d_n7, assign37830_e50378_d_n8, assign37830_e50378_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) {
        let assign37830_e50374: f64 = (var_btat / var_sqrtumax);
        let assign37830_e50375: f64 = (0.375 * assign37830_e50374);
        let assign37830_e50376: f64 = (assign37830_e50375).sqrt();
        (assign37830_e50376, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign37830_e50376)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign37830_e50376)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign37830_e50376)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign37830_e50376)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign37830_e50378;
        var_ktat_dn6 = assign37830_e50378_d_n6;
        var_ktat_dn7 = assign37830_e50378_d_n7;
        var_ktat_dn8 = assign37830_e50378_d_n8;
        var_ktat_dn9 = assign37830_e50378_d_n9;

        let (assign37840_e50396, assign37840_e50396_d_n6, assign37840_e50396_d_n7, assign37840_e50396_d_n8, assign37840_e50396_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) {
        let assign37840_e50391: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign37840_e50392: f64 = (2.0 * assign37840_e50391);
        let assign37840_e50394: f64 = (assign37840_e50392 - var_umax);
        (assign37840_e50394, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign37840_e50396;
        var_ltat_dn6 = assign37840_e50396_d_n6;
        var_ltat_dn7 = assign37840_e50396_d_n7;
        var_ltat_dn8 = assign37840_e50396_d_n8;
        var_ltat_dn9 = assign37840_e50396_d_n9;

        let (assign37850_e50422, assign37850_e50422_d_n6, assign37850_e50422_d_n7, assign37850_e50422_d_n8, assign37850_e50422_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) {
        let assign37850_e50408: f64 = (var_atatsti_d * var_twoatatoverthreebtat);
        let assign37850_e50410: f64 = (assign37850_e50408 * var_sqrtumax);
        let assign37850_e50413: f64 = (var_atatsti_d * var_umax);
        let assign37850_e50414: f64 = (assign37850_e50410 - assign37850_e50413);
        let assign37850_e50418: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign37850_e50419: f64 = (0.5 * assign37850_e50418);
        let assign37850_e50420: f64 = (assign37850_e50414 + assign37850_e50419);
        (assign37850_e50420, (((((var_atatsti_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign37850_e50408 * var_sqrtumax_dn6)) - (var_atatsti_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign37850_e50408 * var_sqrtumax_dn7)) - (var_atatsti_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign37850_e50408 * var_sqrtumax_dn8)) - (var_atatsti_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign37850_e50408 * var_sqrtumax_dn9)) - (var_atatsti_d * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign37850_e50422;
        var_mtat_dn6 = assign37850_e50422_d_n6;
        var_mtat_dn7 = assign37850_e50422_d_n7;
        var_mtat_dn8 = assign37850_e50422_d_n8;
        var_mtat_dn9 = assign37850_e50422_d_n9;

        let (assign37860_e50438, assign37860_e50438_d_n6, assign37860_e50438_d_n7, assign37860_e50438_d_n8, assign37860_e50438_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) {
        let assign37860_e50434: f64 = (var_ltat - 1.0);
        let assign37860_e50436: f64 = (assign37860_e50434 * var_ktat);
        (assign37860_e50436, ((var_ltat_dn6 * var_ktat) + (assign37860_e50434 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign37860_e50434 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign37860_e50434 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign37860_e50434 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign37860_e50438;
        var_xerfc_dn6 = assign37860_e50438_d_n6;
        var_xerfc_dn7 = assign37860_e50438_d_n7;
        var_xerfc_dn8 = assign37860_e50438_d_n8;
        var_xerfc_dn9 = assign37860_e50438_d_n9;

        let (assign37870_e50452, assign37870_e50452_d_n6, assign37870_e50452_d_n7, assign37870_e50452_d_n8, assign37870_e50452_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) {
        let assign37870_e50450: f64 = (var_xerfc * var_xerfc);
        (assign37870_e50450, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign37870_e50452;
        var_ysq_dn6 = assign37870_e50452_d_n6;
        var_ysq_dn7 = assign37870_e50452_d_n7;
        var_ysq_dn8 = assign37870_e50452_d_n8;
        var_ysq_dn9 = assign37870_e50452_d_n9;

        let assign37880_e50455: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard783 = assign37880_e50455;

        let (assign37890_e50475, assign37890_e50475_d_n6, assign37890_e50475_d_n7, assign37890_e50475_d_n8, assign37890_e50475_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) && (var_guard783 != 0.0)) {
        let assign37890_e50471: f64 = (var_perfc * var_xerfc);
        let assign37890_e50472: f64 = (1.0 + assign37890_e50471);
        let assign37890_e50473: f64 = (1.0 / assign37890_e50472);
        (assign37890_e50473, (-((var_perfc * var_xerfc_dn6) / (assign37890_e50472 * assign37890_e50472))), (-((var_perfc * var_xerfc_dn7) / (assign37890_e50472 * assign37890_e50472))), (-((var_perfc * var_xerfc_dn8) / (assign37890_e50472 * assign37890_e50472))), (-((var_perfc * var_xerfc_dn9) / (assign37890_e50472 * assign37890_e50472))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign37890_e50475;
        var_terfc_dn6 = assign37890_e50475_d_n6;
        var_terfc_dn7 = assign37890_e50475_d_n7;
        var_terfc_dn8 = assign37890_e50475_d_n8;
        var_terfc_dn9 = assign37890_e50475_d_n9;

        let (assign37900_e50496, assign37900_e50496_d_n6, assign37900_e50496_d_n7, assign37900_e50496_d_n8, assign37900_e50496_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) && (var_guard783 == 0.0)) {
        let assign37900_e50492: f64 = (var_perfc * var_xerfc);
        let assign37900_e50493: f64 = (1.0 - assign37900_e50492);
        let assign37900_e50494: f64 = (1.0 / assign37900_e50493);
        (assign37900_e50494, (-((-(var_perfc * var_xerfc_dn6)) / (assign37900_e50493 * assign37900_e50493))), (-((-(var_perfc * var_xerfc_dn7)) / (assign37900_e50493 * assign37900_e50493))), (-((-(var_perfc * var_xerfc_dn8)) / (assign37900_e50493 * assign37900_e50493))), (-((-(var_perfc * var_xerfc_dn9)) / (assign37900_e50493 * assign37900_e50493))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign37900_e50496;
        var_terfc_dn6 = assign37900_e50496_d_n6;
        var_terfc_dn7 = assign37900_e50496_d_n7;
        var_terfc_dn8 = assign37900_e50496_d_n8;
        var_terfc_dn9 = assign37900_e50496_d_n9;

        let assign37910_e50498: f64 = (-var_ysq);
        let assign37910_e50500: f64 = (assign37910_e50498 + var_mtat);
        let assign37910_e50502: f64 = (-230.25850929940458);
        let assign37910_e50503: f64 = if assign37910_e50500 > assign37910_e50502 { 1.0 } else { 0.0 };
        var_guard784 = assign37910_e50503;

        let (assign37920_e50521, assign37920_e50521_d_n6, assign37920_e50521_d_n7, assign37920_e50521_d_n8, assign37920_e50521_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) && (var_guard784 != 0.0)) {
        let assign37920_e50516: f64 = (-var_ysq);
        let assign37920_e50518: f64 = (assign37920_e50516 + var_mtat);
        let assign37920_e50519: f64 = (assign37920_e50518).exp();
        (assign37920_e50519, (assign37920_e50519 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign37920_e50519 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign37920_e50519 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign37920_e50519 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign37920_e50521;
        var_tmp_dn6 = assign37920_e50521_d_n6;
        var_tmp_dn7 = assign37920_e50521_d_n7;
        var_tmp_dn8 = assign37920_e50521_d_n8;
        var_tmp_dn9 = assign37920_e50521_d_n9;

        let (assign37930_e50570, assign37930_e50570_d_n6, assign37930_e50570_d_n7, assign37930_e50570_d_n8, assign37930_e50570_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) && (var_guard784 == 0.0)) {
        let assign37930_e50537: f64 = (-230.25850929940458);
        let assign37930_e50539: f64 = (-var_ysq);
        let assign37930_e50541: f64 = (assign37930_e50539 + var_mtat);
        let assign37930_e50542: f64 = (assign37930_e50537 - assign37930_e50541);
        let assign37930_e50546: f64 = (-230.25850929940458);
        let assign37930_e50548: f64 = (-var_ysq);
        let assign37930_e50550: f64 = (assign37930_e50548 + var_mtat);
        let assign37930_e50551: f64 = (assign37930_e50546 - assign37930_e50550);
        let assign37930_e50554: f64 = (-230.25850929940458);
        let assign37930_e50556: f64 = (-var_ysq);
        let assign37930_e50558: f64 = (assign37930_e50556 + var_mtat);
        let assign37930_e50559: f64 = (assign37930_e50554 - assign37930_e50558);
        let assign37930_e50561: f64 = (assign37930_e50559 * 0.3333333333333333);
        let assign37930_e50562: f64 = (1.0 + assign37930_e50561);
        let assign37930_e50563: f64 = (assign37930_e50551 * assign37930_e50562);
        let assign37930_e50564: f64 = (0.5 * assign37930_e50563);
        let assign37930_e50565: f64 = (1.0 + assign37930_e50564);
        let assign37930_e50566: f64 = (assign37930_e50542 * assign37930_e50565);
        let assign37930_e50567: f64 = (1.0 + assign37930_e50566);
        let assign37930_e50568: f64 = (1e-100 / assign37930_e50567);
        (assign37930_e50568, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign37930_e50565) + (assign37930_e50542 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign37930_e50562) + (assign37930_e50551 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign37930_e50567 * assign37930_e50567))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign37930_e50565) + (assign37930_e50542 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign37930_e50562) + (assign37930_e50551 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign37930_e50567 * assign37930_e50567))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign37930_e50565) + (assign37930_e50542 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign37930_e50562) + (assign37930_e50551 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign37930_e50567 * assign37930_e50567))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign37930_e50565) + (assign37930_e50542 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign37930_e50562) + (assign37930_e50551 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign37930_e50567 * assign37930_e50567))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign37930_e50570;
        var_tmp_dn6 = assign37930_e50570_d_n6;
        var_tmp_dn7 = assign37930_e50570_d_n7;
        var_tmp_dn8 = assign37930_e50570_d_n8;
        var_tmp_dn9 = assign37930_e50570_d_n9;

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
        *var_guard778_slot = var_guard778;
        *var_guard779_slot = var_guard779;
        *var_guard780_slot = var_guard780;
        *var_guard781_slot = var_guard781;
        *var_guard782_slot = var_guard782;
        *var_guard783_slot = var_guard783;
        *var_guard784_slot = var_guard784;
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

    pub(super) fn stamp_transient_block_81(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatsti_d: f64,
        var_berfc: f64,
        var_cbbtstid_i: f64,
        var_cerfc: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_ctatstid_i: f64,
        var_fbbtsti_d: f64,
        var_fstopsti_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard777: f64,
        var_guard781: f64,
        var_idmult: f64,
        var_idsatgat_d: f64,
        var_ktat: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_ktat_dn9: f64,
        var_lgdrain_i: f64,
        var_mtat: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_mtat_dn9: f64,
        var_one_over_one_minus_psti_d: f64,
        var_pbrstid_i: f64,
        var_pgatd_i: f64,
        var_pstid_i: f64,
        var_slopesti_d: f64,
        var_terfc: f64,
        var_terfc_dn6: f64,
        var_terfc_dn7: f64,
        var_terfc_dn8: f64,
        var_terfc_dn9: f64,
        var_two_psistar: f64,
        var_v5: f64,
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
        var_guard795_slot: &mut f64,
        var_guard796_slot: &mut f64,
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
        let mut var_guard795: f64 = *var_guard795_slot;
        let mut var_guard796: f64 = *var_guard796_slot;
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
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;

        let (assign37940_e50600, assign37940_e50600_d_n6, assign37940_e50600_d_n7, assign37940_e50600_d_n8, assign37940_e50600_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) {
        let assign37940_e50582: f64 = (0.29214664 * var_terfc);
        let assign37940_e50586: f64 = (var_terfc * var_terfc);
        let assign37940_e50587: f64 = (var_berfc * assign37940_e50586);
        let assign37940_e50588: f64 = (assign37940_e50582 + assign37940_e50587);
        let assign37940_e50592: f64 = (var_terfc * var_terfc);
        let assign37940_e50594: f64 = (assign37940_e50592 * var_terfc);
        let assign37940_e50595: f64 = (var_cerfc * assign37940_e50594);
        let assign37940_e50596: f64 = (assign37940_e50588 + assign37940_e50595);
        let assign37940_e50598: f64 = (assign37940_e50596 * var_tmp);
        (assign37940_e50598, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign37940_e50592 * var_terfc_dn6)))) * var_tmp) + (assign37940_e50596 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign37940_e50592 * var_terfc_dn7)))) * var_tmp) + (assign37940_e50596 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign37940_e50592 * var_terfc_dn8)))) * var_tmp) + (assign37940_e50596 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign37940_e50592 * var_terfc_dn9)))) * var_tmp) + (assign37940_e50596 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign37940_e50600;
        var_erfcpos_dn6 = assign37940_e50600_d_n6;
        var_erfcpos_dn7 = assign37940_e50600_d_n7;
        var_erfcpos_dn8 = assign37940_e50600_d_n8;
        var_erfcpos_dn9 = assign37940_e50600_d_n9;

        let assign37950_e50603: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard785 = assign37950_e50603;

        let (assign37960_e50617, assign37960_e50617_d_n6, assign37960_e50617_d_n7, assign37960_e50617_d_n8, assign37960_e50617_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) && (var_guard785 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign37960_e50617;
        var_erfctimesexpmtat_dn6 = assign37960_e50617_d_n6;
        var_erfctimesexpmtat_dn7 = assign37960_e50617_d_n7;
        var_erfctimesexpmtat_dn8 = assign37960_e50617_d_n8;
        var_erfctimesexpmtat_dn9 = assign37960_e50617_d_n9;

        let assign37970_e50620: f64 = (-230.25850929940458);
        let assign37970_e50621: f64 = if var_mtat > assign37970_e50620 { 1.0 } else { 0.0 };
        var_guard786 = assign37970_e50621;

        let (assign37980_e50639, assign37980_e50639_d_n6, assign37980_e50639_d_n7, assign37980_e50639_d_n8, assign37980_e50639_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) && (var_guard785 == 0.0)) && (var_guard786 != 0.0)) {
        let assign37980_e50637: f64 = (var_mtat).exp();
        (assign37980_e50637, (assign37980_e50637 * var_mtat_dn6), (assign37980_e50637 * var_mtat_dn7), (assign37980_e50637 * var_mtat_dn8), (assign37980_e50637 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign37980_e50639;
        var_tmp_dn6 = assign37980_e50639_d_n6;
        var_tmp_dn7 = assign37980_e50639_d_n7;
        var_tmp_dn8 = assign37980_e50639_d_n8;
        var_tmp_dn9 = assign37980_e50639_d_n9;

        let (assign37990_e50682, assign37990_e50682_d_n6, assign37990_e50682_d_n7, assign37990_e50682_d_n8, assign37990_e50682_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) && (var_guard785 == 0.0)) && (var_guard786 == 0.0)) {
        let assign37990_e50658: f64 = (-230.25850929940458);
        let assign37990_e50660: f64 = (assign37990_e50658 - var_mtat);
        let assign37990_e50664: f64 = (-230.25850929940458);
        let assign37990_e50666: f64 = (assign37990_e50664 - var_mtat);
        let assign37990_e50669: f64 = (-230.25850929940458);
        let assign37990_e50671: f64 = (assign37990_e50669 - var_mtat);
        let assign37990_e50673: f64 = (assign37990_e50671 * 0.3333333333333333);
        let assign37990_e50674: f64 = (1.0 + assign37990_e50673);
        let assign37990_e50675: f64 = (assign37990_e50666 * assign37990_e50674);
        let assign37990_e50676: f64 = (0.5 * assign37990_e50675);
        let assign37990_e50677: f64 = (1.0 + assign37990_e50676);
        let assign37990_e50678: f64 = (assign37990_e50660 * assign37990_e50677);
        let assign37990_e50679: f64 = (1.0 + assign37990_e50678);
        let assign37990_e50680: f64 = (1e-100 / assign37990_e50679);
        (assign37990_e50680, (-((1e-100 * (((-var_mtat_dn6) * assign37990_e50677) + (assign37990_e50660 * (0.5 * (((-var_mtat_dn6) * assign37990_e50674) + (assign37990_e50666 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign37990_e50679 * assign37990_e50679))), (-((1e-100 * (((-var_mtat_dn7) * assign37990_e50677) + (assign37990_e50660 * (0.5 * (((-var_mtat_dn7) * assign37990_e50674) + (assign37990_e50666 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign37990_e50679 * assign37990_e50679))), (-((1e-100 * (((-var_mtat_dn8) * assign37990_e50677) + (assign37990_e50660 * (0.5 * (((-var_mtat_dn8) * assign37990_e50674) + (assign37990_e50666 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign37990_e50679 * assign37990_e50679))), (-((1e-100 * (((-var_mtat_dn9) * assign37990_e50677) + (assign37990_e50660 * (0.5 * (((-var_mtat_dn9) * assign37990_e50674) + (assign37990_e50666 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign37990_e50679 * assign37990_e50679))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign37990_e50682;
        var_tmp_dn6 = assign37990_e50682_d_n6;
        var_tmp_dn7 = assign37990_e50682_d_n7;
        var_tmp_dn8 = assign37990_e50682_d_n8;
        var_tmp_dn9 = assign37990_e50682_d_n9;

        let (assign38000_e50701, assign38000_e50701_d_n6, assign38000_e50701_d_n7, assign38000_e50701_d_n8, assign38000_e50701_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) && (var_guard785 == 0.0)) {
        let assign38000_e50697: f64 = (2.0 * var_tmp);
        let assign38000_e50699: f64 = (assign38000_e50697 - var_erfcpos);
        (assign38000_e50699, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign38000_e50701;
        var_erfctimesexpmtat_dn6 = assign38000_e50701_d_n6;
        var_erfctimesexpmtat_dn7 = assign38000_e50701_d_n7;
        var_erfctimesexpmtat_dn8 = assign38000_e50701_d_n8;
        var_erfctimesexpmtat_dn9 = assign38000_e50701_d_n9;

        let (assign38010_e50721, assign38010_e50721_d_n6, assign38010_e50721_d_n7, assign38010_e50721_d_n8, assign38010_e50721_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) {
        let assign38010_e50713: f64 = (1.772453850905516 * 0.5);
        let assign38010_e50716: f64 = (var_atatsti_d * var_erfctimesexpmtat);
        let assign38010_e50718: f64 = (assign38010_e50716 / var_ktat);
        let assign38010_e50719: f64 = (assign38010_e50713 * assign38010_e50718);
        (assign38010_e50719, (assign38010_e50713 * ((((var_atatsti_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign38010_e50716 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign38010_e50713 * ((((var_atatsti_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign38010_e50716 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign38010_e50713 * ((((var_atatsti_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign38010_e50716 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign38010_e50713 * ((((var_atatsti_d * var_erfctimesexpmtat_dn9) * var_ktat) - (assign38010_e50716 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign38010_e50721;
        var_gammamax_dn6 = assign38010_e50721_d_n6;
        var_gammamax_dn7 = assign38010_e50721_d_n7;
        var_gammamax_dn8 = assign38010_e50721_d_n8;
        var_gammamax_dn9 = assign38010_e50721_d_n9;

        let (assign38020_e50739, assign38020_e50739_d_n6, assign38020_e50739_d_n7, assign38020_e50739_d_n8, assign38020_e50739_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard781 == 0.0)) {
        let assign38020_e50734: f64 = (var_asrh * var_gammamax);
        let assign38020_e50736: f64 = (assign38020_e50734 * var_wtat);
        let assign38020_e50737: f64 = (var_ctatstid_i * assign38020_e50736);
        (assign38020_e50737, (var_ctatstid_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign38020_e50734 * var_wtat_dn6))), (var_ctatstid_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign38020_e50734 * var_wtat_dn7))), (var_ctatstid_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign38020_e50734 * var_wtat_dn8))), (var_ctatstid_i * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign38020_e50734 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign38020_e50739;
        var_itat_dn6 = assign38020_e50739_d_n6;
        var_itat_dn7 = assign38020_e50739_d_n7;
        var_itat_dn8 = assign38020_e50739_d_n8;
        var_itat_dn9 = assign38020_e50739_d_n9;

        let assign38030_e50742: f64 = if var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard787 = assign38030_e50742;

        let (assign38040_e50753, assign38040_e50753_d_n6, assign38040_e50753_d_n7, assign38040_e50753_d_n8, assign38040_e50753_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard787 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign38040_e50753;
        var_ibbt_dn6 = assign38040_e50753_d_n6;
        var_ibbt_dn7 = assign38040_e50753_d_n7;
        var_ibbt_dn8 = assign38040_e50753_d_n8;
        var_ibbt_dn9 = assign38040_e50753_d_n9;

        let assign38050_e50756: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard788 = assign38050_e50756;

        let (assign38060_e50775, assign38060_e50775_d_n6, assign38060_e50775_d_n7, assign38060_e50775_d_n8, assign38060_e50775_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard787 == 0.0)) && (var_guard788 != 0.0)) {
        let assign38060_e50770: f64 = (var_vbirstid_i - var_vbbt);
        let assign38060_e50772: f64 = (assign38060_e50770 * var_vbirstiinv_d);
        let assign38060_e50773: f64 = (assign38060_e50772).sqrt();
        (assign38060_e50773, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38060_e50775;
        var_tmp_dn6 = assign38060_e50775_d_n6;
        var_tmp_dn7 = assign38060_e50775_d_n7;
        var_tmp_dn8 = assign38060_e50775_d_n8;
        var_tmp_dn9 = assign38060_e50775_d_n9;

        let (assign38070_e50796, assign38070_e50796_d_n6, assign38070_e50796_d_n7, assign38070_e50796_d_n8, assign38070_e50796_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard787 == 0.0)) && (var_guard788 == 0.0)) {
        let assign38070_e50790: f64 = (var_vbirstid_i - var_vbbt);
        let assign38070_e50792: f64 = (assign38070_e50790 * var_vbirstiinv_d);
        let assign38070_e50794: f64 = (assign38070_e50792).powf(var_pstid_i);
        (assign38070_e50794, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38070_e50796;
        var_tmp_dn6 = assign38070_e50796_d_n6;
        var_tmp_dn7 = assign38070_e50796_d_n7;
        var_tmp_dn8 = assign38070_e50796_d_n8;
        var_tmp_dn9 = assign38070_e50796_d_n9;

        let (assign38080_e50816, assign38080_e50816_d_n6, assign38080_e50816_d_n7, assign38080_e50816_d_n8, assign38080_e50816_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard787 == 0.0)) {
        let assign38080_e50809: f64 = (var_vbirstid_i - var_vbbt);
        let assign38080_e50811: f64 = (assign38080_e50809 * var_wdepnulrinvsti_d);
        let assign38080_e50813: f64 = (assign38080_e50811 / var_tmp);
        let assign38080_e50814: f64 = (var_one_over_one_minus_psti_d * assign38080_e50813);
        (assign38080_e50814, (var_one_over_one_minus_psti_d * (-((assign38080_e50811 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign38080_e50811 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign38080_e50811 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign38080_e50811 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign38080_e50816;
        var_fmaxr_dn6 = assign38080_e50816_d_n6;
        var_fmaxr_dn7 = assign38080_e50816_d_n7;
        var_fmaxr_dn8 = assign38080_e50816_d_n8;
        var_fmaxr_dn9 = assign38080_e50816_d_n9;

        let assign38090_e50818: f64 = (-var_fbbtsti_d);
        let assign38090_e50820: f64 = (assign38090_e50818 / var_fmaxr);
        let assign38090_e50821: f64 = (assign38090_e50820).abs();
        let assign38090_e50823: f64 = if assign38090_e50821 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard789 = assign38090_e50823;

        let (assign38100_e50841, assign38100_e50841_d_n6, assign38100_e50841_d_n7, assign38100_e50841_d_n8, assign38100_e50841_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard787 == 0.0)) && (var_guard789 != 0.0)) {
        let assign38100_e50836: f64 = (-var_fbbtsti_d);
        let assign38100_e50838: f64 = (assign38100_e50836 / var_fmaxr);
        let assign38100_e50839: f64 = (assign38100_e50838).exp();
        (assign38100_e50839, (assign38100_e50839 * (-((assign38100_e50836 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign38100_e50839 * (-((assign38100_e50836 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign38100_e50839 * (-((assign38100_e50836 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign38100_e50839 * (-((assign38100_e50836 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38100_e50841;
        var_tmp_dn6 = assign38100_e50841_d_n6;
        var_tmp_dn7 = assign38100_e50841_d_n7;
        var_tmp_dn8 = assign38100_e50841_d_n8;
        var_tmp_dn9 = assign38100_e50841_d_n9;

        let assign38110_e50843: f64 = (-var_fbbtsti_d);
        let assign38110_e50845: f64 = (assign38110_e50843 / var_fmaxr);
        let assign38110_e50847: f64 = if assign38110_e50845 < 0.0 { 1.0 } else { 0.0 };
        var_guard790 = assign38110_e50847;

        let (assign38120_e50898, assign38120_e50898_d_n6, assign38120_e50898_d_n7, assign38120_e50898_d_n8, assign38120_e50898_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard787 == 0.0)) && (var_guard789 == 0.0)) && (var_guard790 != 0.0)) {
        let assign38120_e50865: f64 = (-230.25850929940458);
        let assign38120_e50867: f64 = (-var_fbbtsti_d);
        let assign38120_e50869: f64 = (assign38120_e50867 / var_fmaxr);
        let assign38120_e50870: f64 = (assign38120_e50865 - assign38120_e50869);
        let assign38120_e50874: f64 = (-230.25850929940458);
        let assign38120_e50876: f64 = (-var_fbbtsti_d);
        let assign38120_e50878: f64 = (assign38120_e50876 / var_fmaxr);
        let assign38120_e50879: f64 = (assign38120_e50874 - assign38120_e50878);
        let assign38120_e50882: f64 = (-230.25850929940458);
        let assign38120_e50884: f64 = (-var_fbbtsti_d);
        let assign38120_e50886: f64 = (assign38120_e50884 / var_fmaxr);
        let assign38120_e50887: f64 = (assign38120_e50882 - assign38120_e50886);
        let assign38120_e50889: f64 = (assign38120_e50887 * 0.3333333333333333);
        let assign38120_e50890: f64 = (1.0 + assign38120_e50889);
        let assign38120_e50891: f64 = (assign38120_e50879 * assign38120_e50890);
        let assign38120_e50892: f64 = (0.5 * assign38120_e50891);
        let assign38120_e50893: f64 = (1.0 + assign38120_e50892);
        let assign38120_e50894: f64 = (assign38120_e50870 * assign38120_e50893);
        let assign38120_e50895: f64 = (1.0 + assign38120_e50894);
        let assign38120_e50896: f64 = (1e-100 / assign38120_e50895);
        (assign38120_e50896, (-((1e-100 * (((-(-((assign38120_e50867 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign38120_e50893) + (assign38120_e50870 * (0.5 * (((-(-((assign38120_e50876 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign38120_e50890) + (assign38120_e50879 * ((-(-((assign38120_e50884 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign38120_e50895 * assign38120_e50895))), (-((1e-100 * (((-(-((assign38120_e50867 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign38120_e50893) + (assign38120_e50870 * (0.5 * (((-(-((assign38120_e50876 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign38120_e50890) + (assign38120_e50879 * ((-(-((assign38120_e50884 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign38120_e50895 * assign38120_e50895))), (-((1e-100 * (((-(-((assign38120_e50867 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign38120_e50893) + (assign38120_e50870 * (0.5 * (((-(-((assign38120_e50876 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign38120_e50890) + (assign38120_e50879 * ((-(-((assign38120_e50884 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign38120_e50895 * assign38120_e50895))), (-((1e-100 * (((-(-((assign38120_e50867 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign38120_e50893) + (assign38120_e50870 * (0.5 * (((-(-((assign38120_e50876 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign38120_e50890) + (assign38120_e50879 * ((-(-((assign38120_e50884 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign38120_e50895 * assign38120_e50895))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38120_e50898;
        var_tmp_dn6 = assign38120_e50898_d_n6;
        var_tmp_dn7 = assign38120_e50898_d_n7;
        var_tmp_dn8 = assign38120_e50898_d_n8;
        var_tmp_dn9 = assign38120_e50898_d_n9;

        let (assign38130_e50947, assign38130_e50947_d_n6, assign38130_e50947_d_n7, assign38130_e50947_d_n8, assign38130_e50947_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard787 == 0.0)) && (var_guard789 == 0.0)) && (var_guard790 == 0.0)) {
        let assign38130_e50917: f64 = (-var_fbbtsti_d);
        let assign38130_e50919: f64 = (assign38130_e50917 / var_fmaxr);
        let assign38130_e50921: f64 = (assign38130_e50919 - 230.25850929940458);
        let assign38130_e50925: f64 = (-var_fbbtsti_d);
        let assign38130_e50927: f64 = (assign38130_e50925 / var_fmaxr);
        let assign38130_e50929: f64 = (assign38130_e50927 - 230.25850929940458);
        let assign38130_e50932: f64 = (-var_fbbtsti_d);
        let assign38130_e50934: f64 = (assign38130_e50932 / var_fmaxr);
        let assign38130_e50936: f64 = (assign38130_e50934 - 230.25850929940458);
        let assign38130_e50938: f64 = (assign38130_e50936 * 0.3333333333333333);
        let assign38130_e50939: f64 = (1.0 + assign38130_e50938);
        let assign38130_e50940: f64 = (assign38130_e50929 * assign38130_e50939);
        let assign38130_e50941: f64 = (0.5 * assign38130_e50940);
        let assign38130_e50942: f64 = (1.0 + assign38130_e50941);
        let assign38130_e50943: f64 = (assign38130_e50921 * assign38130_e50942);
        let assign38130_e50944: f64 = (1.0 + assign38130_e50943);
        let assign38130_e50945: f64 = (1e100 * assign38130_e50944);
        (assign38130_e50945, (1e100 * (((-((assign38130_e50917 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign38130_e50942) + (assign38130_e50921 * (0.5 * (((-((assign38130_e50925 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign38130_e50939) + (assign38130_e50929 * ((-((assign38130_e50932 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign38130_e50917 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign38130_e50942) + (assign38130_e50921 * (0.5 * (((-((assign38130_e50925 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign38130_e50939) + (assign38130_e50929 * ((-((assign38130_e50932 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign38130_e50917 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign38130_e50942) + (assign38130_e50921 * (0.5 * (((-((assign38130_e50925 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign38130_e50939) + (assign38130_e50929 * ((-((assign38130_e50932 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign38130_e50917 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign38130_e50942) + (assign38130_e50921 * (0.5 * (((-((assign38130_e50925 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign38130_e50939) + (assign38130_e50929 * ((-((assign38130_e50932 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38130_e50947;
        var_tmp_dn6 = assign38130_e50947_d_n6;
        var_tmp_dn7 = assign38130_e50947_d_n7;
        var_tmp_dn8 = assign38130_e50947_d_n8;
        var_tmp_dn9 = assign38130_e50947_d_n9;

        let (assign38140_e50967, assign38140_e50967_d_n6, assign38140_e50967_d_n7, assign38140_e50967_d_n8, assign38140_e50967_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard787 == 0.0)) {
        let assign38140_e50960: f64 = (var_v5 * var_fmaxr);
        let assign38140_e50962: f64 = (assign38140_e50960 * var_fmaxr);
        let assign38140_e50964: f64 = (assign38140_e50962 * var_tmp);
        let assign38140_e50965: f64 = (var_cbbtstid_i * assign38140_e50964);
        (assign38140_e50965, (var_cbbtstid_i * (((((var_v5 * var_fmaxr_dn6) * var_fmaxr) + (assign38140_e50960 * var_fmaxr_dn6)) * var_tmp) + (assign38140_e50962 * var_tmp_dn6))), (var_cbbtstid_i * (((((var_v5 * var_fmaxr_dn7) * var_fmaxr) + (assign38140_e50960 * var_fmaxr_dn7)) * var_tmp) + (assign38140_e50962 * var_tmp_dn7))), (var_cbbtstid_i * (((((var_v5 * var_fmaxr_dn8) * var_fmaxr) + (assign38140_e50960 * var_fmaxr_dn8)) * var_tmp) + (assign38140_e50962 * var_tmp_dn8))), (var_cbbtstid_i * (((((var_v5 * var_fmaxr_dn9) * var_fmaxr) + (assign38140_e50960 * var_fmaxr_dn9)) * var_tmp) + (assign38140_e50962 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign38140_e50967;
        var_ibbt_dn6 = assign38140_e50967_d_n6;
        var_ibbt_dn7 = assign38140_e50967_d_n7;
        var_ibbt_dn8 = assign38140_e50967_d_n8;
        var_ibbt_dn9 = assign38140_e50967_d_n9;

        let assign38150_e50970: f64 = if var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard791 = assign38150_e50970;

        let (assign38160_e50981, assign38160_e50981_d_n6, assign38160_e50981_d_n7, assign38160_e50981_d_n8, assign38160_e50981_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard791 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign38160_e50981;
        var_fbreakdown_dn6 = assign38160_e50981_d_n6;
        var_fbreakdown_dn7 = assign38160_e50981_d_n7;
        var_fbreakdown_dn8 = assign38160_e50981_d_n8;
        var_fbreakdown_dn9 = assign38160_e50981_d_n9;

        let assign38170_e50984: f64 = (-var_alphaav);
        let assign38170_e50986: f64 = (assign38170_e50984 * var_vbrstid_i);
        let assign38170_e50987: f64 = if var_vav > assign38170_e50986 { 1.0 } else { 0.0 };
        var_guard792 = assign38170_e50987;

        let assign38180_e50990: f64 = if var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        var_guard793 = assign38180_e50990;

        let (assign38190_e51020, assign38190_e51020_d_n6, assign38190_e51020_d_n7, assign38190_e51020_d_n8, assign38190_e51020_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard791 == 0.0)) && (var_guard792 != 0.0)) && (var_guard793 != 0.0)) {
        let assign38190_e51006: f64 = (var_vav * var_vbrinvsti_d);
        let assign38190_e51009: f64 = (var_vav * var_vbrinvsti_d);
        let assign38190_e51010: f64 = (assign38190_e51006 * assign38190_e51009);
        let assign38190_e51013: f64 = (var_vav * var_vbrinvsti_d);
        let assign38190_e51014: f64 = (assign38190_e51010 * assign38190_e51013);
        let assign38190_e51017: f64 = (var_vav * var_vbrinvsti_d);
        let assign38190_e51018: f64 = (assign38190_e51014 * assign38190_e51017);
        (assign38190_e51018, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38190_e51020;
        var_tmp_dn6 = assign38190_e51020_d_n6;
        var_tmp_dn7 = assign38190_e51020_d_n7;
        var_tmp_dn8 = assign38190_e51020_d_n8;
        var_tmp_dn9 = assign38190_e51020_d_n9;

        let (assign38200_e51042, assign38200_e51042_d_n6, assign38200_e51042_d_n7, assign38200_e51042_d_n8, assign38200_e51042_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard791 == 0.0)) && (var_guard792 != 0.0)) && (var_guard793 == 0.0)) {
        let assign38200_e51037: f64 = (var_vav * var_vbrinvsti_d);
        let assign38200_e51038: f64 = (assign38200_e51037).abs();
        let assign38200_e51040: f64 = (assign38200_e51038).powf(var_pbrstid_i);
        (assign38200_e51040, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38200_e51042;
        var_tmp_dn6 = assign38200_e51042_d_n6;
        var_tmp_dn7 = assign38200_e51042_d_n7;
        var_tmp_dn8 = assign38200_e51042_d_n8;
        var_tmp_dn9 = assign38200_e51042_d_n9;

        let (assign38210_e51060, assign38210_e51060_d_n6, assign38210_e51060_d_n7, assign38210_e51060_d_n8, assign38210_e51060_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard791 == 0.0)) && (var_guard792 != 0.0)) {
        let assign38210_e51057: f64 = (1.0 - var_tmp);
        let assign38210_e51058: f64 = (1.0 / assign38210_e51057);
        (assign38210_e51058, (-((-var_tmp_dn6) / (assign38210_e51057 * assign38210_e51057))), (-((-var_tmp_dn7) / (assign38210_e51057 * assign38210_e51057))), (-((-var_tmp_dn8) / (assign38210_e51057 * assign38210_e51057))), (-((-var_tmp_dn9) / (assign38210_e51057 * assign38210_e51057))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign38210_e51060;
        var_fbreakdown_dn6 = assign38210_e51060_d_n6;
        var_fbreakdown_dn7 = assign38210_e51060_d_n7;
        var_fbreakdown_dn8 = assign38210_e51060_d_n8;
        var_fbreakdown_dn9 = assign38210_e51060_d_n9;

        let (assign38220_e51083, assign38220_e51083_d_n6, assign38220_e51083_d_n7, assign38220_e51083_d_n8, assign38220_e51083_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) && (var_guard791 == 0.0)) && (var_guard792 == 0.0)) {
        let assign38220_e51077: f64 = (var_alphaav * var_vbrstid_i);
        let assign38220_e51078: f64 = (var_vav + assign38220_e51077);
        let assign38220_e51080: f64 = (assign38220_e51078 * var_slopesti_d);
        let assign38220_e51081: f64 = (var_fstopsti_d + assign38220_e51080);
        (assign38220_e51081, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign38220_e51083;
        var_fbreakdown_dn6 = assign38220_e51083_d_n6;
        var_fbreakdown_dn7 = assign38220_e51083_d_n7;
        var_fbreakdown_dn8 = assign38220_e51083_d_n8;
        var_fbreakdown_dn9 = assign38220_e51083_d_n9;

        let (assign38230_e51102, assign38230_e51102_d_n6, assign38230_e51102_d_n7, assign38230_e51102_d_n8, assign38230_e51102_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard777 == 0.0)) {
        let assign38230_e51093: f64 = (var_id__blk212 + var_isrh);
        let assign38230_e51095: f64 = (assign38230_e51093 + var_itat);
        let assign38230_e51097: f64 = (assign38230_e51095 + var_ibbt);
        let assign38230_e51098: f64 = (p.p29 * assign38230_e51097);
        let assign38230_e51100: f64 = (assign38230_e51098 * var_fbreakdown);
        (assign38230_e51100, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign38230_e51098 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign38230_e51098 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign38230_e51098 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign38230_e51098 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign38230_e51102;
        var_ijunsti_dn6 = assign38230_e51102_d_n6;
        var_ijunsti_dn7 = assign38230_e51102_d_n7;
        var_ijunsti_dn8 = assign38230_e51102_d_n8;
        var_ijunsti_dn9 = assign38230_e51102_d_n9;

        let assign38240_e51105: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard794 = assign38240_e51105;

        let (assign38250_e51113, assign38250_e51113_d_n6, assign38250_e51113_d_n7, assign38250_e51113_d_n8, assign38250_e51113_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign38250_e51113;
        var_ijungat_dn6 = assign38250_e51113_d_n6;
        var_ijungat_dn7 = assign38250_e51113_d_n7;
        var_ijungat_dn8 = assign38250_e51113_d_n8;
        var_ijungat_dn9 = assign38250_e51113_d_n9;

        let (assign38260_e51124,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) {
        let assign38260_e51122: f64 = (var_idsatgat_d * var_idmult);
        (assign38260_e51122,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign38260_e51124;

        let assign38270_e51131: f64 = if ((var_csrhgatd_i == 0.0) && (var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard795 = assign38270_e51131;

        let (assign38280_e51142, assign38280_e51142_d_n6, assign38280_e51142_d_n7, assign38280_e51142_d_n8, assign38280_e51142_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard795 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign38280_e51142;
        var_isrh_dn6 = assign38280_e51142_d_n6;
        var_isrh_dn7 = assign38280_e51142_d_n7;
        var_isrh_dn8 = assign38280_e51142_d_n8;
        var_isrh_dn9 = assign38280_e51142_d_n9;

        let (assign38290_e51156,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard795 == 0.0)) {
        let assign38290_e51154: f64 = (var_vbigat_d - var_vjsrh);
        (assign38290_e51154,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign38290_e51156;

        let (assign38300_e51175,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard795 == 0.0)) {
        let assign38300_e51170: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign38300_e51171: f64 = (1.0 - assign38300_e51170);
        let assign38300_e51172: f64 = (assign38300_e51171).sqrt();
        let assign38300_e51173: f64 = (1.0 - assign38300_e51172);
        (assign38300_e51173,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign38300_e51175;

        let assign38310_e51178: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard796 = assign38310_e51178;

        let (assign38320_e51192,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard795 == 0.0)) && (var_guard796 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign38320_e51192;

        let (assign38330_e51224,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard795 == 0.0)) && (var_guard796 == 0.0)) {
        let assign38330_e51207: f64 = (var_wsrhstep * var_wsrhstep);
        let assign38330_e51209: f64 = (var_wsrhstep).ln();
        let assign38330_e51210: f64 = (assign38330_e51207 * assign38330_e51209);
        let assign38330_e51213: f64 = (1.0 - var_wsrhstep);
        let assign38330_e51214: f64 = (assign38330_e51210 / assign38330_e51213);
        let assign38330_e51216: f64 = (assign38330_e51214 + var_wsrhstep);
        let assign38330_e51220: f64 = (2.0 * var_pgatd_i);
        let assign38330_e51221: f64 = (1.0 - assign38330_e51220);
        let assign38330_e51222: f64 = (assign38330_e51216 * assign38330_e51221);
        (assign38330_e51222,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign38330_e51224;

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
        *var_guard795_slot = var_guard795;
        *var_guard796_slot = var_guard796;
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
        *var_wsrhstep_slot = var_wsrhstep;
    }

    pub(super) fn stamp_transient_block_82(
        var_atatgat_d: f64,
        var_berfc: f64,
        var_btatpartgat_d: f64,
        var_cerfc: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_dwsrh: f64,
        var_ftdgat_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard794: f64,
        var_guard795: f64,
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
        var_guard797_slot: &mut f64,
        var_guard798_slot: &mut f64,
        var_guard799_slot: &mut f64,
        var_guard800_slot: &mut f64,
        var_guard801_slot: &mut f64,
        var_guard802_slot: &mut f64,
        var_guard803_slot: &mut f64,
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
        let mut var_guard797: f64 = *var_guard797_slot;
        let mut var_guard798: f64 = *var_guard798_slot;
        let mut var_guard799: f64 = *var_guard799_slot;
        let mut var_guard800: f64 = *var_guard800_slot;
        let mut var_guard801: f64 = *var_guard801_slot;
        let mut var_guard802: f64 = *var_guard802_slot;
        let mut var_guard803: f64 = *var_guard803_slot;
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

        let (assign38340_e51238,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard795 == 0.0)) {
        let assign38340_e51236: f64 = (var_wsrhstep + var_dwsrh);
        (assign38340_e51236,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign38340_e51238;

        let assign38350_e51241: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard797 = assign38350_e51241;

        let (assign38360_e51258, assign38360_e51258_d_n6, assign38360_e51258_d_n7, assign38360_e51258_d_n8, assign38360_e51258_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard795 == 0.0)) && (var_guard797 != 0.0)) {
        let assign38360_e51255: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign38360_e51256: f64 = (assign38360_e51255).sqrt();
        (assign38360_e51256, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38360_e51258;
        var_tmp_dn6 = assign38360_e51258_d_n6;
        var_tmp_dn7 = assign38360_e51258_d_n7;
        var_tmp_dn8 = assign38360_e51258_d_n8;
        var_tmp_dn9 = assign38360_e51258_d_n9;

        let (assign38370_e51277, assign38370_e51277_d_n6, assign38370_e51277_d_n7, assign38370_e51277_d_n8, assign38370_e51277_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard795 == 0.0)) && (var_guard797 == 0.0)) {
        let assign38370_e51273: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign38370_e51275: f64 = (assign38370_e51273).powf(var_pgatd_i);
        (assign38370_e51275, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38370_e51277;
        var_tmp_dn6 = assign38370_e51277_d_n6;
        var_tmp_dn7 = assign38370_e51277_d_n7;
        var_tmp_dn8 = assign38370_e51277_d_n8;
        var_tmp_dn9 = assign38370_e51277_d_n9;

        let (assign38380_e51291, assign38380_e51291_d_n6, assign38380_e51291_d_n7, assign38380_e51291_d_n8, assign38380_e51291_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard795 == 0.0)) {
        let assign38380_e51289: f64 = (var_wdepnulrgat_d * var_tmp);
        (assign38380_e51289, (var_wdepnulrgat_d * var_tmp_dn6), (var_wdepnulrgat_d * var_tmp_dn7), (var_wdepnulrgat_d * var_tmp_dn8), (var_wdepnulrgat_d * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign38380_e51291;
        var_wdep_dn6 = assign38380_e51291_d_n6;
        var_wdep_dn7 = assign38380_e51291_d_n7;
        var_wdep_dn8 = assign38380_e51291_d_n8;
        var_wdep_dn9 = assign38380_e51291_d_n9;

        let (assign38390_e51309, assign38390_e51309_d_n6, assign38390_e51309_d_n7, assign38390_e51309_d_n8, assign38390_e51309_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard795 == 0.0)) {
        let assign38390_e51304: f64 = (var_zinv - 1.0);
        let assign38390_e51306: f64 = (assign38390_e51304 * var_wdep);
        let assign38390_e51307: f64 = (var_ftdgat_d * assign38390_e51306);
        (assign38390_e51307, (var_ftdgat_d * (assign38390_e51304 * var_wdep_dn6)), (var_ftdgat_d * (assign38390_e51304 * var_wdep_dn7)), (var_ftdgat_d * (assign38390_e51304 * var_wdep_dn8)), (var_ftdgat_d * (assign38390_e51304 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign38390_e51309;
        var_asrh_dn6 = assign38390_e51309_d_n6;
        var_asrh_dn7 = assign38390_e51309_d_n7;
        var_asrh_dn8 = assign38390_e51309_d_n8;
        var_asrh_dn9 = assign38390_e51309_d_n9;

        let (assign38400_e51325, assign38400_e51325_d_n6, assign38400_e51325_d_n7, assign38400_e51325_d_n8, assign38400_e51325_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard795 == 0.0)) {
        let assign38400_e51322: f64 = (var_asrh * var_wsrh);
        let assign38400_e51323: f64 = (var_csrhgatd_i * assign38400_e51322);
        (assign38400_e51323, (var_csrhgatd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn8 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign38400_e51325;
        var_isrh_dn6 = assign38400_e51325_d_n6;
        var_isrh_dn7 = assign38400_e51325_d_n7;
        var_isrh_dn8 = assign38400_e51325_d_n8;
        var_isrh_dn9 = assign38400_e51325_d_n9;

        let assign38410_e51328: f64 = if var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard798 = assign38410_e51328;

        let (assign38420_e51339, assign38420_e51339_d_n6, assign38420_e51339_d_n7, assign38420_e51339_d_n8, assign38420_e51339_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign38420_e51339;
        var_itat_dn6 = assign38420_e51339_d_n6;
        var_itat_dn7 = assign38420_e51339_d_n7;
        var_itat_dn8 = assign38420_e51339_d_n8;
        var_itat_dn9 = assign38420_e51339_d_n9;

        let (assign38430_e51357, assign38430_e51357_d_n6, assign38430_e51357_d_n7, assign38430_e51357_d_n8, assign38430_e51357_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) {
        let assign38430_e51352: f64 = (var_wdep * var_one_minus_pgat_d);
        let assign38430_e51354: f64 = (assign38430_e51352 / var_vbi_minus_vjsrh);
        let assign38430_e51355: f64 = (var_btatpartgat_d * assign38430_e51354);
        (assign38430_e51355, (var_btatpartgat_d * ((var_wdep_dn6 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn7 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn8 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn9 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign38430_e51357;
        var_btat_dn6 = assign38430_e51357_d_n6;
        var_btat_dn7 = assign38430_e51357_d_n7;
        var_btat_dn8 = assign38430_e51357_d_n8;
        var_btat_dn9 = assign38430_e51357_d_n9;

        let (assign38440_e51373, assign38440_e51373_d_n6, assign38440_e51373_d_n7, assign38440_e51373_d_n8, assign38440_e51373_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) {
        let assign38440_e51369: f64 = (0.666666666666667 * var_atatgat_d);
        let assign38440_e51371: f64 = (assign38440_e51369 / var_btat);
        (assign38440_e51371, (-((assign38440_e51369 * var_btat_dn6) / (var_btat * var_btat))), (-((assign38440_e51369 * var_btat_dn7) / (var_btat * var_btat))), (-((assign38440_e51369 * var_btat_dn8) / (var_btat * var_btat))), (-((assign38440_e51369 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign38440_e51373;
        var_twoatatoverthreebtat_dn6 = assign38440_e51373_d_n6;
        var_twoatatoverthreebtat_dn7 = assign38440_e51373_d_n7;
        var_twoatatoverthreebtat_dn8 = assign38440_e51373_d_n8;
        var_twoatatoverthreebtat_dn9 = assign38440_e51373_d_n9;

        let (assign38450_e51387, assign38450_e51387_d_n6, assign38450_e51387_d_n7, assign38450_e51387_d_n8, assign38450_e51387_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) {
        let assign38450_e51385: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign38450_e51385, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign38450_e51387;
        var_umaxbeforelimiting_dn6 = assign38450_e51387_d_n6;
        var_umaxbeforelimiting_dn7 = assign38450_e51387_d_n7;
        var_umaxbeforelimiting_dn8 = assign38450_e51387_d_n8;
        var_umaxbeforelimiting_dn9 = assign38450_e51387_d_n9;

        let (assign38460_e51408, assign38460_e51408_d_n6, assign38460_e51408_d_n7, assign38460_e51408_d_n8, assign38460_e51408_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) {
        let assign38460_e51399: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign38460_e51402: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign38460_e51404: f64 = (assign38460_e51402 + 1.0);
        let assign38460_e51405: f64 = (assign38460_e51399 / assign38460_e51404);
        let assign38460_e51406: f64 = (assign38460_e51405).sqrt();
        (assign38460_e51406, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign38460_e51404) - (assign38460_e51399 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign38460_e51404 * assign38460_e51404)) / (2.0 * assign38460_e51406)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign38460_e51404) - (assign38460_e51399 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign38460_e51404 * assign38460_e51404)) / (2.0 * assign38460_e51406)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign38460_e51404) - (assign38460_e51399 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign38460_e51404 * assign38460_e51404)) / (2.0 * assign38460_e51406)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign38460_e51404) - (assign38460_e51399 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign38460_e51404 * assign38460_e51404)) / (2.0 * assign38460_e51406)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign38460_e51408;
        var_umax_dn6 = assign38460_e51408_d_n6;
        var_umax_dn7 = assign38460_e51408_d_n7;
        var_umax_dn8 = assign38460_e51408_d_n8;
        var_umax_dn9 = assign38460_e51408_d_n9;

        let (assign38470_e51421, assign38470_e51421_d_n6, assign38470_e51421_d_n7, assign38470_e51421_d_n8, assign38470_e51421_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) {
        let assign38470_e51419: f64 = (var_umax).sqrt();
        (assign38470_e51419, (var_umax_dn6 / (2.0 * assign38470_e51419)), (var_umax_dn7 / (2.0 * assign38470_e51419)), (var_umax_dn8 / (2.0 * assign38470_e51419)), (var_umax_dn9 / (2.0 * assign38470_e51419)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign38470_e51421;
        var_sqrtumax_dn6 = assign38470_e51421_d_n6;
        var_sqrtumax_dn7 = assign38470_e51421_d_n7;
        var_sqrtumax_dn8 = assign38470_e51421_d_n8;
        var_sqrtumax_dn9 = assign38470_e51421_d_n9;

        let (assign38480_e51435, assign38480_e51435_d_n6, assign38480_e51435_d_n7, assign38480_e51435_d_n8, assign38480_e51435_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) {
        let assign38480_e51433: f64 = (var_umax * var_sqrtumax);
        (assign38480_e51433, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign38480_e51435;
        var_umaxpoweronepointfive_dn6 = assign38480_e51435_d_n6;
        var_umaxpoweronepointfive_dn7 = assign38480_e51435_d_n7;
        var_umaxpoweronepointfive_dn8 = assign38480_e51435_d_n8;
        var_umaxpoweronepointfive_dn9 = assign38480_e51435_d_n9;

        let assign38490_e51437: f64 = (-var_pgatd_i);
        let assign38490_e51439: f64 = (assign38490_e51437 * var_one_over_one_minus_pgat_d);
        let assign38490_e51441: f64 = (-1.0);
        let assign38490_e51442: f64 = if assign38490_e51439 == assign38490_e51441 { 1.0 } else { 0.0 };
        var_guard799 = assign38490_e51442;

        let (assign38500_e51462, assign38500_e51462_d_n6, assign38500_e51462_d_n7, assign38500_e51462_d_n8, assign38500_e51462_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) && (var_guard799 != 0.0)) {
        let assign38500_e51458: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign38500_e51459: f64 = (1.0 + assign38500_e51458);
        let assign38500_e51460: f64 = (1.0 / assign38500_e51459);
        (assign38500_e51460, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign38500_e51459 * assign38500_e51459))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign38500_e51459 * assign38500_e51459))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign38500_e51459 * assign38500_e51459))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign38500_e51459 * assign38500_e51459))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign38500_e51462;
        var_wgamma_dn6 = assign38500_e51462_d_n6;
        var_wgamma_dn7 = assign38500_e51462_d_n7;
        var_wgamma_dn8 = assign38500_e51462_d_n8;
        var_wgamma_dn9 = assign38500_e51462_d_n9;

        let (assign38510_e51486, assign38510_e51486_d_n6, assign38510_e51486_d_n7, assign38510_e51486_d_n8, assign38510_e51486_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) && (var_guard799 == 0.0)) {
        let assign38510_e51478: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign38510_e51479: f64 = (1.0 + assign38510_e51478);
        let assign38510_e51481: f64 = (-var_pgatd_i);
        let assign38510_e51483: f64 = (assign38510_e51481 * var_one_over_one_minus_pgat_d);
        let assign38510_e51484: f64 = (assign38510_e51479).powf(assign38510_e51483);
        (assign38510_e51484, if 0.0 == 0.0 && ((assign38510_e51483) as f64).is_finite() && ((assign38510_e51483) as f64).fract() == 0.0 { if assign38510_e51483 == 0.0 { 0.0 } else { (assign38510_e51483 * ((assign38510_e51479).powf(assign38510_e51483 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign38510_e51484 * (assign38510_e51483 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign38510_e51479))) }, if 0.0 == 0.0 && ((assign38510_e51483) as f64).is_finite() && ((assign38510_e51483) as f64).fract() == 0.0 { if assign38510_e51483 == 0.0 { 0.0 } else { (assign38510_e51483 * ((assign38510_e51479).powf(assign38510_e51483 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign38510_e51484 * (assign38510_e51483 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign38510_e51479))) }, if 0.0 == 0.0 && ((assign38510_e51483) as f64).is_finite() && ((assign38510_e51483) as f64).fract() == 0.0 { if assign38510_e51483 == 0.0 { 0.0 } else { (assign38510_e51483 * ((assign38510_e51479).powf(assign38510_e51483 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign38510_e51484 * (assign38510_e51483 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign38510_e51479))) }, if 0.0 == 0.0 && ((assign38510_e51483) as f64).is_finite() && ((assign38510_e51483) as f64).fract() == 0.0 { if assign38510_e51483 == 0.0 { 0.0 } else { (assign38510_e51483 * ((assign38510_e51479).powf(assign38510_e51483 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign38510_e51484 * (assign38510_e51483 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign38510_e51479))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign38510_e51486;
        var_wgamma_dn6 = assign38510_e51486_d_n6;
        var_wgamma_dn7 = assign38510_e51486_d_n7;
        var_wgamma_dn8 = assign38510_e51486_d_n8;
        var_wgamma_dn9 = assign38510_e51486_d_n9;

        let (assign38520_e51504, assign38520_e51504_d_n6, assign38520_e51504_d_n7, assign38520_e51504_d_n8, assign38520_e51504_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) {
        let assign38520_e51498: f64 = (var_wsrh * var_wgamma);
        let assign38520_e51501: f64 = (var_wsrh + var_wgamma);
        let assign38520_e51502: f64 = (assign38520_e51498 / assign38520_e51501);
        (assign38520_e51502, ((((var_wsrh * var_wgamma_dn6) * assign38520_e51501) - (assign38520_e51498 * var_wgamma_dn6)) / (assign38520_e51501 * assign38520_e51501)), ((((var_wsrh * var_wgamma_dn7) * assign38520_e51501) - (assign38520_e51498 * var_wgamma_dn7)) / (assign38520_e51501 * assign38520_e51501)), ((((var_wsrh * var_wgamma_dn8) * assign38520_e51501) - (assign38520_e51498 * var_wgamma_dn8)) / (assign38520_e51501 * assign38520_e51501)), ((((var_wsrh * var_wgamma_dn9) * assign38520_e51501) - (assign38520_e51498 * var_wgamma_dn9)) / (assign38520_e51501 * assign38520_e51501)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign38520_e51504;
        var_wtat_dn6 = assign38520_e51504_d_n6;
        var_wtat_dn7 = assign38520_e51504_d_n7;
        var_wtat_dn8 = assign38520_e51504_d_n8;
        var_wtat_dn9 = assign38520_e51504_d_n9;

        let (assign38530_e51521, assign38530_e51521_d_n6, assign38530_e51521_d_n7, assign38530_e51521_d_n8, assign38530_e51521_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) {
        let assign38530_e51517: f64 = (var_btat / var_sqrtumax);
        let assign38530_e51518: f64 = (0.375 * assign38530_e51517);
        let assign38530_e51519: f64 = (assign38530_e51518).sqrt();
        (assign38530_e51519, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign38530_e51519)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign38530_e51519)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign38530_e51519)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign38530_e51519)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign38530_e51521;
        var_ktat_dn6 = assign38530_e51521_d_n6;
        var_ktat_dn7 = assign38530_e51521_d_n7;
        var_ktat_dn8 = assign38530_e51521_d_n8;
        var_ktat_dn9 = assign38530_e51521_d_n9;

        let (assign38540_e51539, assign38540_e51539_d_n6, assign38540_e51539_d_n7, assign38540_e51539_d_n8, assign38540_e51539_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) {
        let assign38540_e51534: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign38540_e51535: f64 = (2.0 * assign38540_e51534);
        let assign38540_e51537: f64 = (assign38540_e51535 - var_umax);
        (assign38540_e51537, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign38540_e51539;
        var_ltat_dn6 = assign38540_e51539_d_n6;
        var_ltat_dn7 = assign38540_e51539_d_n7;
        var_ltat_dn8 = assign38540_e51539_d_n8;
        var_ltat_dn9 = assign38540_e51539_d_n9;

        let (assign38550_e51565, assign38550_e51565_d_n6, assign38550_e51565_d_n7, assign38550_e51565_d_n8, assign38550_e51565_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) {
        let assign38550_e51551: f64 = (var_atatgat_d * var_twoatatoverthreebtat);
        let assign38550_e51553: f64 = (assign38550_e51551 * var_sqrtumax);
        let assign38550_e51556: f64 = (var_atatgat_d * var_umax);
        let assign38550_e51557: f64 = (assign38550_e51553 - assign38550_e51556);
        let assign38550_e51561: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign38550_e51562: f64 = (0.5 * assign38550_e51561);
        let assign38550_e51563: f64 = (assign38550_e51557 + assign38550_e51562);
        (assign38550_e51563, (((((var_atatgat_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign38550_e51551 * var_sqrtumax_dn6)) - (var_atatgat_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign38550_e51551 * var_sqrtumax_dn7)) - (var_atatgat_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign38550_e51551 * var_sqrtumax_dn8)) - (var_atatgat_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign38550_e51551 * var_sqrtumax_dn9)) - (var_atatgat_d * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign38550_e51565;
        var_mtat_dn6 = assign38550_e51565_d_n6;
        var_mtat_dn7 = assign38550_e51565_d_n7;
        var_mtat_dn8 = assign38550_e51565_d_n8;
        var_mtat_dn9 = assign38550_e51565_d_n9;

        let (assign38560_e51581, assign38560_e51581_d_n6, assign38560_e51581_d_n7, assign38560_e51581_d_n8, assign38560_e51581_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) {
        let assign38560_e51577: f64 = (var_ltat - 1.0);
        let assign38560_e51579: f64 = (assign38560_e51577 * var_ktat);
        (assign38560_e51579, ((var_ltat_dn6 * var_ktat) + (assign38560_e51577 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign38560_e51577 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign38560_e51577 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign38560_e51577 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign38560_e51581;
        var_xerfc_dn6 = assign38560_e51581_d_n6;
        var_xerfc_dn7 = assign38560_e51581_d_n7;
        var_xerfc_dn8 = assign38560_e51581_d_n8;
        var_xerfc_dn9 = assign38560_e51581_d_n9;

        let (assign38570_e51595, assign38570_e51595_d_n6, assign38570_e51595_d_n7, assign38570_e51595_d_n8, assign38570_e51595_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) {
        let assign38570_e51593: f64 = (var_xerfc * var_xerfc);
        (assign38570_e51593, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign38570_e51595;
        var_ysq_dn6 = assign38570_e51595_d_n6;
        var_ysq_dn7 = assign38570_e51595_d_n7;
        var_ysq_dn8 = assign38570_e51595_d_n8;
        var_ysq_dn9 = assign38570_e51595_d_n9;

        let assign38580_e51598: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard800 = assign38580_e51598;

        let (assign38590_e51618, assign38590_e51618_d_n6, assign38590_e51618_d_n7, assign38590_e51618_d_n8, assign38590_e51618_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) && (var_guard800 != 0.0)) {
        let assign38590_e51614: f64 = (var_perfc * var_xerfc);
        let assign38590_e51615: f64 = (1.0 + assign38590_e51614);
        let assign38590_e51616: f64 = (1.0 / assign38590_e51615);
        (assign38590_e51616, (-((var_perfc * var_xerfc_dn6) / (assign38590_e51615 * assign38590_e51615))), (-((var_perfc * var_xerfc_dn7) / (assign38590_e51615 * assign38590_e51615))), (-((var_perfc * var_xerfc_dn8) / (assign38590_e51615 * assign38590_e51615))), (-((var_perfc * var_xerfc_dn9) / (assign38590_e51615 * assign38590_e51615))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign38590_e51618;
        var_terfc_dn6 = assign38590_e51618_d_n6;
        var_terfc_dn7 = assign38590_e51618_d_n7;
        var_terfc_dn8 = assign38590_e51618_d_n8;
        var_terfc_dn9 = assign38590_e51618_d_n9;

        let (assign38600_e51639, assign38600_e51639_d_n6, assign38600_e51639_d_n7, assign38600_e51639_d_n8, assign38600_e51639_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) && (var_guard800 == 0.0)) {
        let assign38600_e51635: f64 = (var_perfc * var_xerfc);
        let assign38600_e51636: f64 = (1.0 - assign38600_e51635);
        let assign38600_e51637: f64 = (1.0 / assign38600_e51636);
        (assign38600_e51637, (-((-(var_perfc * var_xerfc_dn6)) / (assign38600_e51636 * assign38600_e51636))), (-((-(var_perfc * var_xerfc_dn7)) / (assign38600_e51636 * assign38600_e51636))), (-((-(var_perfc * var_xerfc_dn8)) / (assign38600_e51636 * assign38600_e51636))), (-((-(var_perfc * var_xerfc_dn9)) / (assign38600_e51636 * assign38600_e51636))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign38600_e51639;
        var_terfc_dn6 = assign38600_e51639_d_n6;
        var_terfc_dn7 = assign38600_e51639_d_n7;
        var_terfc_dn8 = assign38600_e51639_d_n8;
        var_terfc_dn9 = assign38600_e51639_d_n9;

        let assign38610_e51641: f64 = (-var_ysq);
        let assign38610_e51643: f64 = (assign38610_e51641 + var_mtat);
        let assign38610_e51645: f64 = (-230.25850929940458);
        let assign38610_e51646: f64 = if assign38610_e51643 > assign38610_e51645 { 1.0 } else { 0.0 };
        var_guard801 = assign38610_e51646;

        let (assign38620_e51664, assign38620_e51664_d_n6, assign38620_e51664_d_n7, assign38620_e51664_d_n8, assign38620_e51664_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) && (var_guard801 != 0.0)) {
        let assign38620_e51659: f64 = (-var_ysq);
        let assign38620_e51661: f64 = (assign38620_e51659 + var_mtat);
        let assign38620_e51662: f64 = (assign38620_e51661).exp();
        (assign38620_e51662, (assign38620_e51662 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign38620_e51662 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign38620_e51662 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign38620_e51662 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38620_e51664;
        var_tmp_dn6 = assign38620_e51664_d_n6;
        var_tmp_dn7 = assign38620_e51664_d_n7;
        var_tmp_dn8 = assign38620_e51664_d_n8;
        var_tmp_dn9 = assign38620_e51664_d_n9;

        let (assign38630_e51713, assign38630_e51713_d_n6, assign38630_e51713_d_n7, assign38630_e51713_d_n8, assign38630_e51713_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) && (var_guard801 == 0.0)) {
        let assign38630_e51680: f64 = (-230.25850929940458);
        let assign38630_e51682: f64 = (-var_ysq);
        let assign38630_e51684: f64 = (assign38630_e51682 + var_mtat);
        let assign38630_e51685: f64 = (assign38630_e51680 - assign38630_e51684);
        let assign38630_e51689: f64 = (-230.25850929940458);
        let assign38630_e51691: f64 = (-var_ysq);
        let assign38630_e51693: f64 = (assign38630_e51691 + var_mtat);
        let assign38630_e51694: f64 = (assign38630_e51689 - assign38630_e51693);
        let assign38630_e51697: f64 = (-230.25850929940458);
        let assign38630_e51699: f64 = (-var_ysq);
        let assign38630_e51701: f64 = (assign38630_e51699 + var_mtat);
        let assign38630_e51702: f64 = (assign38630_e51697 - assign38630_e51701);
        let assign38630_e51704: f64 = (assign38630_e51702 * 0.3333333333333333);
        let assign38630_e51705: f64 = (1.0 + assign38630_e51704);
        let assign38630_e51706: f64 = (assign38630_e51694 * assign38630_e51705);
        let assign38630_e51707: f64 = (0.5 * assign38630_e51706);
        let assign38630_e51708: f64 = (1.0 + assign38630_e51707);
        let assign38630_e51709: f64 = (assign38630_e51685 * assign38630_e51708);
        let assign38630_e51710: f64 = (1.0 + assign38630_e51709);
        let assign38630_e51711: f64 = (1e-100 / assign38630_e51710);
        (assign38630_e51711, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign38630_e51708) + (assign38630_e51685 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign38630_e51705) + (assign38630_e51694 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign38630_e51710 * assign38630_e51710))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign38630_e51708) + (assign38630_e51685 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign38630_e51705) + (assign38630_e51694 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign38630_e51710 * assign38630_e51710))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign38630_e51708) + (assign38630_e51685 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign38630_e51705) + (assign38630_e51694 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign38630_e51710 * assign38630_e51710))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign38630_e51708) + (assign38630_e51685 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign38630_e51705) + (assign38630_e51694 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign38630_e51710 * assign38630_e51710))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38630_e51713;
        var_tmp_dn6 = assign38630_e51713_d_n6;
        var_tmp_dn7 = assign38630_e51713_d_n7;
        var_tmp_dn8 = assign38630_e51713_d_n8;
        var_tmp_dn9 = assign38630_e51713_d_n9;

        let (assign38640_e51743, assign38640_e51743_d_n6, assign38640_e51743_d_n7, assign38640_e51743_d_n8, assign38640_e51743_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) {
        let assign38640_e51725: f64 = (0.29214664 * var_terfc);
        let assign38640_e51729: f64 = (var_terfc * var_terfc);
        let assign38640_e51730: f64 = (var_berfc * assign38640_e51729);
        let assign38640_e51731: f64 = (assign38640_e51725 + assign38640_e51730);
        let assign38640_e51735: f64 = (var_terfc * var_terfc);
        let assign38640_e51737: f64 = (assign38640_e51735 * var_terfc);
        let assign38640_e51738: f64 = (var_cerfc * assign38640_e51737);
        let assign38640_e51739: f64 = (assign38640_e51731 + assign38640_e51738);
        let assign38640_e51741: f64 = (assign38640_e51739 * var_tmp);
        (assign38640_e51741, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign38640_e51735 * var_terfc_dn6)))) * var_tmp) + (assign38640_e51739 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign38640_e51735 * var_terfc_dn7)))) * var_tmp) + (assign38640_e51739 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign38640_e51735 * var_terfc_dn8)))) * var_tmp) + (assign38640_e51739 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign38640_e51735 * var_terfc_dn9)))) * var_tmp) + (assign38640_e51739 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign38640_e51743;
        var_erfcpos_dn6 = assign38640_e51743_d_n6;
        var_erfcpos_dn7 = assign38640_e51743_d_n7;
        var_erfcpos_dn8 = assign38640_e51743_d_n8;
        var_erfcpos_dn9 = assign38640_e51743_d_n9;

        let assign38650_e51746: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard802 = assign38650_e51746;

        let (assign38660_e51760, assign38660_e51760_d_n6, assign38660_e51760_d_n7, assign38660_e51760_d_n8, assign38660_e51760_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) && (var_guard802 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign38660_e51760;
        var_erfctimesexpmtat_dn6 = assign38660_e51760_d_n6;
        var_erfctimesexpmtat_dn7 = assign38660_e51760_d_n7;
        var_erfctimesexpmtat_dn8 = assign38660_e51760_d_n8;
        var_erfctimesexpmtat_dn9 = assign38660_e51760_d_n9;

        let assign38670_e51763: f64 = (-230.25850929940458);
        let assign38670_e51764: f64 = if var_mtat > assign38670_e51763 { 1.0 } else { 0.0 };
        var_guard803 = assign38670_e51764;

        let (assign38680_e51782, assign38680_e51782_d_n6, assign38680_e51782_d_n7, assign38680_e51782_d_n8, assign38680_e51782_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) && (var_guard802 == 0.0)) && (var_guard803 != 0.0)) {
        let assign38680_e51780: f64 = (var_mtat).exp();
        (assign38680_e51780, (assign38680_e51780 * var_mtat_dn6), (assign38680_e51780 * var_mtat_dn7), (assign38680_e51780 * var_mtat_dn8), (assign38680_e51780 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38680_e51782;
        var_tmp_dn6 = assign38680_e51782_d_n6;
        var_tmp_dn7 = assign38680_e51782_d_n7;
        var_tmp_dn8 = assign38680_e51782_d_n8;
        var_tmp_dn9 = assign38680_e51782_d_n9;

        let (assign38690_e51825, assign38690_e51825_d_n6, assign38690_e51825_d_n7, assign38690_e51825_d_n8, assign38690_e51825_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) && (var_guard802 == 0.0)) && (var_guard803 == 0.0)) {
        let assign38690_e51801: f64 = (-230.25850929940458);
        let assign38690_e51803: f64 = (assign38690_e51801 - var_mtat);
        let assign38690_e51807: f64 = (-230.25850929940458);
        let assign38690_e51809: f64 = (assign38690_e51807 - var_mtat);
        let assign38690_e51812: f64 = (-230.25850929940458);
        let assign38690_e51814: f64 = (assign38690_e51812 - var_mtat);
        let assign38690_e51816: f64 = (assign38690_e51814 * 0.3333333333333333);
        let assign38690_e51817: f64 = (1.0 + assign38690_e51816);
        let assign38690_e51818: f64 = (assign38690_e51809 * assign38690_e51817);
        let assign38690_e51819: f64 = (0.5 * assign38690_e51818);
        let assign38690_e51820: f64 = (1.0 + assign38690_e51819);
        let assign38690_e51821: f64 = (assign38690_e51803 * assign38690_e51820);
        let assign38690_e51822: f64 = (1.0 + assign38690_e51821);
        let assign38690_e51823: f64 = (1e-100 / assign38690_e51822);
        (assign38690_e51823, (-((1e-100 * (((-var_mtat_dn6) * assign38690_e51820) + (assign38690_e51803 * (0.5 * (((-var_mtat_dn6) * assign38690_e51817) + (assign38690_e51809 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign38690_e51822 * assign38690_e51822))), (-((1e-100 * (((-var_mtat_dn7) * assign38690_e51820) + (assign38690_e51803 * (0.5 * (((-var_mtat_dn7) * assign38690_e51817) + (assign38690_e51809 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign38690_e51822 * assign38690_e51822))), (-((1e-100 * (((-var_mtat_dn8) * assign38690_e51820) + (assign38690_e51803 * (0.5 * (((-var_mtat_dn8) * assign38690_e51817) + (assign38690_e51809 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign38690_e51822 * assign38690_e51822))), (-((1e-100 * (((-var_mtat_dn9) * assign38690_e51820) + (assign38690_e51803 * (0.5 * (((-var_mtat_dn9) * assign38690_e51817) + (assign38690_e51809 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign38690_e51822 * assign38690_e51822))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38690_e51825;
        var_tmp_dn6 = assign38690_e51825_d_n6;
        var_tmp_dn7 = assign38690_e51825_d_n7;
        var_tmp_dn8 = assign38690_e51825_d_n8;
        var_tmp_dn9 = assign38690_e51825_d_n9;

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
        *var_guard797_slot = var_guard797;
        *var_guard798_slot = var_guard798;
        *var_guard799_slot = var_guard799;
        *var_guard800_slot = var_guard800;
        *var_guard801_slot = var_guard801;
        *var_guard802_slot = var_guard802;
        *var_guard803_slot = var_guard803;
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

    pub(super) fn stamp_transient_block_83(
        p: &Parameters,
        var_abdrain_i: f64,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatgat_d: f64,
        var_cbbtgatd_i: f64,
        var_ctatgatd_i: f64,
        var_erfcpos: f64,
        var_erfcpos_dn6: f64,
        var_erfcpos_dn7: f64,
        var_erfcpos_dn8: f64,
        var_erfcpos_dn9: f64,
        var_fbbtgat_d: f64,
        var_fbbtgat_d_dn6: f64,
        var_fbbtgat_d_dn7: f64,
        var_fbbtgat_d_dn8: f64,
        var_fbbtgat_d_dn9: f64,
        var_fstopgat_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard794: f64,
        var_guard798: f64,
        var_guard802: f64,
        var_i1: f64,
        var_i1_dn6: f64,
        var_i1_dn7: f64,
        var_i1_dn8: f64,
        var_i1_dn9: f64,
        var_i4: f64,
        var_i4_dn6: f64,
        var_i4_dn7: f64,
        var_i4_dn8: f64,
        var_i4_dn9: f64,
        var_id__blk212: f64,
        var_idsatbot_d: f64,
        var_idsatgat_d: f64,
        var_idsatsti_d: f64,
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
        var_ktat: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_ktat_dn9: f64,
        var_lgdrain_i: f64,
        var_lsdrain_i: f64,
        var_mfor1_d: f64,
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
        var_v1: f64,
        var_v4: f64,
        var_v5: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirgatd_i: f64,
        var_vbirgatinv_d: f64,
        var_vbrgatd_i: f64,
        var_vbrinvgat_d: f64,
        var_vbrinvgat_d_dn6: f64,
        var_vbrinvgat_d_dn7: f64,
        var_vbrinvgat_d_dn8: f64,
        var_vbrinvgat_d_dn9: f64,
        var_wdepnulrinvgat_d: f64,
        var_wtat: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_wtat_dn9: f64,
        var_alphaje_slot: &mut f64,
        var_alphaje_dn6_slot: &mut f64,
        var_alphaje_dn7_slot: &mut f64,
        var_alphaje_dn8_slot: &mut f64,
        var_alphaje_dn9_slot: &mut f64,
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
        var_guard804_slot: &mut f64,
        var_guard805_slot: &mut f64,
        var_guard806_slot: &mut f64,
        var_guard807_slot: &mut f64,
        var_guard808_slot: &mut f64,
        var_guard809_slot: &mut f64,
        var_guard810_slot: &mut f64,
        var_guard811_slot: &mut f64,
        var_guard812_slot: &mut f64,
        var_guard813_slot: &mut f64,
        var_i1_cor_slot: &mut f64,
        var_i1_cor_dn6_slot: &mut f64,
        var_i1_cor_dn7_slot: &mut f64,
        var_i1_cor_dn8_slot: &mut f64,
        var_i1_cor_dn9_slot: &mut f64,
        var_i4_cor_slot: &mut f64,
        var_i4_cor_dn6_slot: &mut f64,
        var_i4_cor_dn7_slot: &mut f64,
        var_i4_cor_dn8_slot: &mut f64,
        var_i4_cor_dn9_slot: &mut f64,
        var_i5_slot: &mut f64,
        var_i5_cor_slot: &mut f64,
        var_i5_cor_dn6_slot: &mut f64,
        var_i5_cor_dn7_slot: &mut f64,
        var_i5_cor_dn8_slot: &mut f64,
        var_i5_cor_dn9_slot: &mut f64,
        var_i5_dn6_slot: &mut f64,
        var_i5_dn7_slot: &mut f64,
        var_i5_dn8_slot: &mut f64,
        var_i5_dn9_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_ijungat_dn9_slot: &mut f64,
        var_isatfor1_d_slot: &mut f64,
        var_isatfor2_d_slot: &mut f64,
        var_isatfor2_d_dn6_slot: &mut f64,
        var_isatfor2_d_dn7_slot: &mut f64,
        var_isatfor2_d_dn8_slot: &mut f64,
        var_isatfor2_d_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_mfor2_d_slot: &mut f64,
        var_mfor2_d_dn6_slot: &mut f64,
        var_mfor2_d_dn7_slot: &mut f64,
        var_mfor2_d_dn8_slot: &mut f64,
        var_mfor2_d_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
    ) {
        let mut var_alphaje: f64 = *var_alphaje_slot;
        let mut var_alphaje_dn6: f64 = *var_alphaje_dn6_slot;
        let mut var_alphaje_dn7: f64 = *var_alphaje_dn7_slot;
        let mut var_alphaje_dn8: f64 = *var_alphaje_dn8_slot;
        let mut var_alphaje_dn9: f64 = *var_alphaje_dn9_slot;
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
        let mut var_guard804: f64 = *var_guard804_slot;
        let mut var_guard805: f64 = *var_guard805_slot;
        let mut var_guard806: f64 = *var_guard806_slot;
        let mut var_guard807: f64 = *var_guard807_slot;
        let mut var_guard808: f64 = *var_guard808_slot;
        let mut var_guard809: f64 = *var_guard809_slot;
        let mut var_guard810: f64 = *var_guard810_slot;
        let mut var_guard811: f64 = *var_guard811_slot;
        let mut var_guard812: f64 = *var_guard812_slot;
        let mut var_guard813: f64 = *var_guard813_slot;
        let mut var_i1_cor: f64 = *var_i1_cor_slot;
        let mut var_i1_cor_dn6: f64 = *var_i1_cor_dn6_slot;
        let mut var_i1_cor_dn7: f64 = *var_i1_cor_dn7_slot;
        let mut var_i1_cor_dn8: f64 = *var_i1_cor_dn8_slot;
        let mut var_i1_cor_dn9: f64 = *var_i1_cor_dn9_slot;
        let mut var_i4_cor: f64 = *var_i4_cor_slot;
        let mut var_i4_cor_dn6: f64 = *var_i4_cor_dn6_slot;
        let mut var_i4_cor_dn7: f64 = *var_i4_cor_dn7_slot;
        let mut var_i4_cor_dn8: f64 = *var_i4_cor_dn8_slot;
        let mut var_i4_cor_dn9: f64 = *var_i4_cor_dn9_slot;
        let mut var_i5: f64 = *var_i5_slot;
        let mut var_i5_cor: f64 = *var_i5_cor_slot;
        let mut var_i5_cor_dn6: f64 = *var_i5_cor_dn6_slot;
        let mut var_i5_cor_dn7: f64 = *var_i5_cor_dn7_slot;
        let mut var_i5_cor_dn8: f64 = *var_i5_cor_dn8_slot;
        let mut var_i5_cor_dn9: f64 = *var_i5_cor_dn9_slot;
        let mut var_i5_dn6: f64 = *var_i5_dn6_slot;
        let mut var_i5_dn7: f64 = *var_i5_dn7_slot;
        let mut var_i5_dn8: f64 = *var_i5_dn8_slot;
        let mut var_i5_dn9: f64 = *var_i5_dn9_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_ijungat_dn9: f64 = *var_ijungat_dn9_slot;
        let mut var_isatfor1_d: f64 = *var_isatfor1_d_slot;
        let mut var_isatfor2_d: f64 = *var_isatfor2_d_slot;
        let mut var_isatfor2_d_dn6: f64 = *var_isatfor2_d_dn6_slot;
        let mut var_isatfor2_d_dn7: f64 = *var_isatfor2_d_dn7_slot;
        let mut var_isatfor2_d_dn8: f64 = *var_isatfor2_d_dn8_slot;
        let mut var_isatfor2_d_dn9: f64 = *var_isatfor2_d_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_mfor2_d: f64 = *var_mfor2_d_slot;
        let mut var_mfor2_d_dn6: f64 = *var_mfor2_d_dn6_slot;
        let mut var_mfor2_d_dn7: f64 = *var_mfor2_d_dn7_slot;
        let mut var_mfor2_d_dn8: f64 = *var_mfor2_d_dn8_slot;
        let mut var_mfor2_d_dn9: f64 = *var_mfor2_d_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;

        let (assign38700_e51844, assign38700_e51844_d_n6, assign38700_e51844_d_n7, assign38700_e51844_d_n8, assign38700_e51844_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) && (var_guard802 == 0.0)) {
        let assign38700_e51840: f64 = (2.0 * var_tmp);
        let assign38700_e51842: f64 = (assign38700_e51840 - var_erfcpos);
        (assign38700_e51842, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign38700_e51844;
        var_erfctimesexpmtat_dn6 = assign38700_e51844_d_n6;
        var_erfctimesexpmtat_dn7 = assign38700_e51844_d_n7;
        var_erfctimesexpmtat_dn8 = assign38700_e51844_d_n8;
        var_erfctimesexpmtat_dn9 = assign38700_e51844_d_n9;

        let (assign38710_e51864, assign38710_e51864_d_n6, assign38710_e51864_d_n7, assign38710_e51864_d_n8, assign38710_e51864_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) {
        let assign38710_e51856: f64 = (1.772453850905516 * 0.5);
        let assign38710_e51859: f64 = (var_atatgat_d * var_erfctimesexpmtat);
        let assign38710_e51861: f64 = (assign38710_e51859 / var_ktat);
        let assign38710_e51862: f64 = (assign38710_e51856 * assign38710_e51861);
        (assign38710_e51862, (assign38710_e51856 * ((((var_atatgat_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign38710_e51859 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign38710_e51856 * ((((var_atatgat_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign38710_e51859 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign38710_e51856 * ((((var_atatgat_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign38710_e51859 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign38710_e51856 * ((((var_atatgat_d * var_erfctimesexpmtat_dn9) * var_ktat) - (assign38710_e51859 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign38710_e51864;
        var_gammamax_dn6 = assign38710_e51864_d_n6;
        var_gammamax_dn7 = assign38710_e51864_d_n7;
        var_gammamax_dn8 = assign38710_e51864_d_n8;
        var_gammamax_dn9 = assign38710_e51864_d_n9;

        let (assign38720_e51882, assign38720_e51882_d_n6, assign38720_e51882_d_n7, assign38720_e51882_d_n8, assign38720_e51882_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard798 == 0.0)) {
        let assign38720_e51877: f64 = (var_asrh * var_gammamax);
        let assign38720_e51879: f64 = (assign38720_e51877 * var_wtat);
        let assign38720_e51880: f64 = (var_ctatgatd_i * assign38720_e51879);
        (assign38720_e51880, (var_ctatgatd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign38720_e51877 * var_wtat_dn6))), (var_ctatgatd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign38720_e51877 * var_wtat_dn7))), (var_ctatgatd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign38720_e51877 * var_wtat_dn8))), (var_ctatgatd_i * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign38720_e51877 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign38720_e51882;
        var_itat_dn6 = assign38720_e51882_d_n6;
        var_itat_dn7 = assign38720_e51882_d_n7;
        var_itat_dn8 = assign38720_e51882_d_n8;
        var_itat_dn9 = assign38720_e51882_d_n9;

        let assign38730_e51885: f64 = if var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard804 = assign38730_e51885;

        let (assign38740_e51896, assign38740_e51896_d_n6, assign38740_e51896_d_n7, assign38740_e51896_d_n8, assign38740_e51896_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard804 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign38740_e51896;
        var_ibbt_dn6 = assign38740_e51896_d_n6;
        var_ibbt_dn7 = assign38740_e51896_d_n7;
        var_ibbt_dn8 = assign38740_e51896_d_n8;
        var_ibbt_dn9 = assign38740_e51896_d_n9;

        let assign38750_e51899: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard805 = assign38750_e51899;

        let (assign38760_e51918, assign38760_e51918_d_n6, assign38760_e51918_d_n7, assign38760_e51918_d_n8, assign38760_e51918_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard804 == 0.0)) && (var_guard805 != 0.0)) {
        let assign38760_e51913: f64 = (var_vbirgatd_i - var_vbbt);
        let assign38760_e51915: f64 = (assign38760_e51913 * var_vbirgatinv_d);
        let assign38760_e51916: f64 = (assign38760_e51915).sqrt();
        (assign38760_e51916, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38760_e51918;
        var_tmp_dn6 = assign38760_e51918_d_n6;
        var_tmp_dn7 = assign38760_e51918_d_n7;
        var_tmp_dn8 = assign38760_e51918_d_n8;
        var_tmp_dn9 = assign38760_e51918_d_n9;

        let (assign38770_e51939, assign38770_e51939_d_n6, assign38770_e51939_d_n7, assign38770_e51939_d_n8, assign38770_e51939_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard804 == 0.0)) && (var_guard805 == 0.0)) {
        let assign38770_e51933: f64 = (var_vbirgatd_i - var_vbbt);
        let assign38770_e51935: f64 = (assign38770_e51933 * var_vbirgatinv_d);
        let assign38770_e51937: f64 = (assign38770_e51935).powf(var_pgatd_i);
        (assign38770_e51937, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38770_e51939;
        var_tmp_dn6 = assign38770_e51939_d_n6;
        var_tmp_dn7 = assign38770_e51939_d_n7;
        var_tmp_dn8 = assign38770_e51939_d_n8;
        var_tmp_dn9 = assign38770_e51939_d_n9;

        let (assign38780_e51959, assign38780_e51959_d_n6, assign38780_e51959_d_n7, assign38780_e51959_d_n8, assign38780_e51959_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard804 == 0.0)) {
        let assign38780_e51952: f64 = (var_vbirgatd_i - var_vbbt);
        let assign38780_e51954: f64 = (assign38780_e51952 * var_wdepnulrinvgat_d);
        let assign38780_e51956: f64 = (assign38780_e51954 / var_tmp);
        let assign38780_e51957: f64 = (var_one_over_one_minus_pgat_d * assign38780_e51956);
        (assign38780_e51957, (var_one_over_one_minus_pgat_d * (-((assign38780_e51954 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign38780_e51954 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign38780_e51954 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign38780_e51954 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign38780_e51959;
        var_fmaxr_dn6 = assign38780_e51959_d_n6;
        var_fmaxr_dn7 = assign38780_e51959_d_n7;
        var_fmaxr_dn8 = assign38780_e51959_d_n8;
        var_fmaxr_dn9 = assign38780_e51959_d_n9;

        let assign38790_e51961: f64 = (-var_fbbtgat_d);
        let assign38790_e51963: f64 = (assign38790_e51961 / var_fmaxr);
        let assign38790_e51964: f64 = (assign38790_e51963).abs();
        let assign38790_e51966: f64 = if assign38790_e51964 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard806 = assign38790_e51966;

        let (assign38800_e51984, assign38800_e51984_d_n6, assign38800_e51984_d_n7, assign38800_e51984_d_n8, assign38800_e51984_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard804 == 0.0)) && (var_guard806 != 0.0)) {
        let assign38800_e51979: f64 = (-var_fbbtgat_d);
        let assign38800_e51981: f64 = (assign38800_e51979 / var_fmaxr);
        let assign38800_e51982: f64 = (assign38800_e51981).exp();
        (assign38800_e51982, (assign38800_e51982 * ((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign38800_e51979 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign38800_e51982 * ((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign38800_e51979 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign38800_e51982 * ((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign38800_e51979 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))), (assign38800_e51982 * ((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign38800_e51979 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38800_e51984;
        var_tmp_dn6 = assign38800_e51984_d_n6;
        var_tmp_dn7 = assign38800_e51984_d_n7;
        var_tmp_dn8 = assign38800_e51984_d_n8;
        var_tmp_dn9 = assign38800_e51984_d_n9;

        let assign38810_e51986: f64 = (-var_fbbtgat_d);
        let assign38810_e51988: f64 = (assign38810_e51986 / var_fmaxr);
        let assign38810_e51990: f64 = if assign38810_e51988 < 0.0 { 1.0 } else { 0.0 };
        var_guard807 = assign38810_e51990;

        let (assign38820_e52041, assign38820_e52041_d_n6, assign38820_e52041_d_n7, assign38820_e52041_d_n8, assign38820_e52041_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard804 == 0.0)) && (var_guard806 == 0.0)) && (var_guard807 != 0.0)) {
        let assign38820_e52008: f64 = (-230.25850929940458);
        let assign38820_e52010: f64 = (-var_fbbtgat_d);
        let assign38820_e52012: f64 = (assign38820_e52010 / var_fmaxr);
        let assign38820_e52013: f64 = (assign38820_e52008 - assign38820_e52012);
        let assign38820_e52017: f64 = (-230.25850929940458);
        let assign38820_e52019: f64 = (-var_fbbtgat_d);
        let assign38820_e52021: f64 = (assign38820_e52019 / var_fmaxr);
        let assign38820_e52022: f64 = (assign38820_e52017 - assign38820_e52021);
        let assign38820_e52025: f64 = (-230.25850929940458);
        let assign38820_e52027: f64 = (-var_fbbtgat_d);
        let assign38820_e52029: f64 = (assign38820_e52027 / var_fmaxr);
        let assign38820_e52030: f64 = (assign38820_e52025 - assign38820_e52029);
        let assign38820_e52032: f64 = (assign38820_e52030 * 0.3333333333333333);
        let assign38820_e52033: f64 = (1.0 + assign38820_e52032);
        let assign38820_e52034: f64 = (assign38820_e52022 * assign38820_e52033);
        let assign38820_e52035: f64 = (0.5 * assign38820_e52034);
        let assign38820_e52036: f64 = (1.0 + assign38820_e52035);
        let assign38820_e52037: f64 = (assign38820_e52013 * assign38820_e52036);
        let assign38820_e52038: f64 = (1.0 + assign38820_e52037);
        let assign38820_e52039: f64 = (1e-100 / assign38820_e52038);
        (assign38820_e52039, (-((1e-100 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign38820_e52010 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign38820_e52036) + (assign38820_e52013 * (0.5 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign38820_e52019 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign38820_e52033) + (assign38820_e52022 * ((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign38820_e52027 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign38820_e52038 * assign38820_e52038))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign38820_e52010 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign38820_e52036) + (assign38820_e52013 * (0.5 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign38820_e52019 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign38820_e52033) + (assign38820_e52022 * ((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign38820_e52027 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign38820_e52038 * assign38820_e52038))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign38820_e52010 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign38820_e52036) + (assign38820_e52013 * (0.5 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign38820_e52019 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign38820_e52033) + (assign38820_e52022 * ((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign38820_e52027 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign38820_e52038 * assign38820_e52038))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign38820_e52010 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign38820_e52036) + (assign38820_e52013 * (0.5 * (((-((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign38820_e52019 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign38820_e52033) + (assign38820_e52022 * ((-((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign38820_e52027 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign38820_e52038 * assign38820_e52038))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38820_e52041;
        var_tmp_dn6 = assign38820_e52041_d_n6;
        var_tmp_dn7 = assign38820_e52041_d_n7;
        var_tmp_dn8 = assign38820_e52041_d_n8;
        var_tmp_dn9 = assign38820_e52041_d_n9;

        let (assign38830_e52090, assign38830_e52090_d_n6, assign38830_e52090_d_n7, assign38830_e52090_d_n8, assign38830_e52090_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard804 == 0.0)) && (var_guard806 == 0.0)) && (var_guard807 == 0.0)) {
        let assign38830_e52060: f64 = (-var_fbbtgat_d);
        let assign38830_e52062: f64 = (assign38830_e52060 / var_fmaxr);
        let assign38830_e52064: f64 = (assign38830_e52062 - 230.25850929940458);
        let assign38830_e52068: f64 = (-var_fbbtgat_d);
        let assign38830_e52070: f64 = (assign38830_e52068 / var_fmaxr);
        let assign38830_e52072: f64 = (assign38830_e52070 - 230.25850929940458);
        let assign38830_e52075: f64 = (-var_fbbtgat_d);
        let assign38830_e52077: f64 = (assign38830_e52075 / var_fmaxr);
        let assign38830_e52079: f64 = (assign38830_e52077 - 230.25850929940458);
        let assign38830_e52081: f64 = (assign38830_e52079 * 0.3333333333333333);
        let assign38830_e52082: f64 = (1.0 + assign38830_e52081);
        let assign38830_e52083: f64 = (assign38830_e52072 * assign38830_e52082);
        let assign38830_e52084: f64 = (0.5 * assign38830_e52083);
        let assign38830_e52085: f64 = (1.0 + assign38830_e52084);
        let assign38830_e52086: f64 = (assign38830_e52064 * assign38830_e52085);
        let assign38830_e52087: f64 = (1.0 + assign38830_e52086);
        let assign38830_e52088: f64 = (1e100 * assign38830_e52087);
        (assign38830_e52088, (1e100 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign38830_e52060 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign38830_e52085) + (assign38830_e52064 * (0.5 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign38830_e52068 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign38830_e52082) + (assign38830_e52072 * (((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign38830_e52075 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign38830_e52060 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign38830_e52085) + (assign38830_e52064 * (0.5 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign38830_e52068 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign38830_e52082) + (assign38830_e52072 * (((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign38830_e52075 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign38830_e52060 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign38830_e52085) + (assign38830_e52064 * (0.5 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign38830_e52068 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign38830_e52082) + (assign38830_e52072 * (((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign38830_e52075 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign38830_e52060 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign38830_e52085) + (assign38830_e52064 * (0.5 * ((((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign38830_e52068 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign38830_e52082) + (assign38830_e52072 * (((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign38830_e52075 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38830_e52090;
        var_tmp_dn6 = assign38830_e52090_d_n6;
        var_tmp_dn7 = assign38830_e52090_d_n7;
        var_tmp_dn8 = assign38830_e52090_d_n8;
        var_tmp_dn9 = assign38830_e52090_d_n9;

        let (assign38840_e52110, assign38840_e52110_d_n6, assign38840_e52110_d_n7, assign38840_e52110_d_n8, assign38840_e52110_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard804 == 0.0)) {
        let assign38840_e52103: f64 = (var_v5 * var_fmaxr);
        let assign38840_e52105: f64 = (assign38840_e52103 * var_fmaxr);
        let assign38840_e52107: f64 = (assign38840_e52105 * var_tmp);
        let assign38840_e52108: f64 = (var_cbbtgatd_i * assign38840_e52107);
        (assign38840_e52108, (var_cbbtgatd_i * (((((var_v5 * var_fmaxr_dn6) * var_fmaxr) + (assign38840_e52103 * var_fmaxr_dn6)) * var_tmp) + (assign38840_e52105 * var_tmp_dn6))), (var_cbbtgatd_i * (((((var_v5 * var_fmaxr_dn7) * var_fmaxr) + (assign38840_e52103 * var_fmaxr_dn7)) * var_tmp) + (assign38840_e52105 * var_tmp_dn7))), (var_cbbtgatd_i * (((((var_v5 * var_fmaxr_dn8) * var_fmaxr) + (assign38840_e52103 * var_fmaxr_dn8)) * var_tmp) + (assign38840_e52105 * var_tmp_dn8))), (var_cbbtgatd_i * (((((var_v5 * var_fmaxr_dn9) * var_fmaxr) + (assign38840_e52103 * var_fmaxr_dn9)) * var_tmp) + (assign38840_e52105 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign38840_e52110;
        var_ibbt_dn6 = assign38840_e52110_d_n6;
        var_ibbt_dn7 = assign38840_e52110_d_n7;
        var_ibbt_dn8 = assign38840_e52110_d_n8;
        var_ibbt_dn9 = assign38840_e52110_d_n9;

        let assign38850_e52113: f64 = if var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard808 = assign38850_e52113;

        let (assign38860_e52124, assign38860_e52124_d_n6, assign38860_e52124_d_n7, assign38860_e52124_d_n8, assign38860_e52124_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard808 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign38860_e52124;
        var_fbreakdown_dn6 = assign38860_e52124_d_n6;
        var_fbreakdown_dn7 = assign38860_e52124_d_n7;
        var_fbreakdown_dn8 = assign38860_e52124_d_n8;
        var_fbreakdown_dn9 = assign38860_e52124_d_n9;

        let assign38870_e52127: f64 = (-var_alphaav);
        let assign38870_e52129: f64 = (assign38870_e52127 * var_vbrgatd_i);
        let assign38870_e52130: f64 = if var_vav > assign38870_e52129 { 1.0 } else { 0.0 };
        var_guard809 = assign38870_e52130;

        let assign38880_e52133: f64 = if var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard810 = assign38880_e52133;

        let (assign38890_e52163, assign38890_e52163_d_n6, assign38890_e52163_d_n7, assign38890_e52163_d_n8, assign38890_e52163_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard808 == 0.0)) && (var_guard809 != 0.0)) && (var_guard810 != 0.0)) {
        let assign38890_e52149: f64 = (var_vav * var_vbrinvgat_d);
        let assign38890_e52152: f64 = (var_vav * var_vbrinvgat_d);
        let assign38890_e52153: f64 = (assign38890_e52149 * assign38890_e52152);
        let assign38890_e52156: f64 = (var_vav * var_vbrinvgat_d);
        let assign38890_e52157: f64 = (assign38890_e52153 * assign38890_e52156);
        let assign38890_e52160: f64 = (var_vav * var_vbrinvgat_d);
        let assign38890_e52161: f64 = (assign38890_e52157 * assign38890_e52160);
        (assign38890_e52161, (((((((var_vav * var_vbrinvgat_d_dn6) * assign38890_e52152) + (assign38890_e52149 * (var_vav * var_vbrinvgat_d_dn6))) * assign38890_e52156) + (assign38890_e52153 * (var_vav * var_vbrinvgat_d_dn6))) * assign38890_e52160) + (assign38890_e52157 * (var_vav * var_vbrinvgat_d_dn6))), (((((((var_vav * var_vbrinvgat_d_dn7) * assign38890_e52152) + (assign38890_e52149 * (var_vav * var_vbrinvgat_d_dn7))) * assign38890_e52156) + (assign38890_e52153 * (var_vav * var_vbrinvgat_d_dn7))) * assign38890_e52160) + (assign38890_e52157 * (var_vav * var_vbrinvgat_d_dn7))), (((((((var_vav * var_vbrinvgat_d_dn8) * assign38890_e52152) + (assign38890_e52149 * (var_vav * var_vbrinvgat_d_dn8))) * assign38890_e52156) + (assign38890_e52153 * (var_vav * var_vbrinvgat_d_dn8))) * assign38890_e52160) + (assign38890_e52157 * (var_vav * var_vbrinvgat_d_dn8))), (((((((var_vav * var_vbrinvgat_d_dn9) * assign38890_e52152) + (assign38890_e52149 * (var_vav * var_vbrinvgat_d_dn9))) * assign38890_e52156) + (assign38890_e52153 * (var_vav * var_vbrinvgat_d_dn9))) * assign38890_e52160) + (assign38890_e52157 * (var_vav * var_vbrinvgat_d_dn9))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38890_e52163;
        var_tmp_dn6 = assign38890_e52163_d_n6;
        var_tmp_dn7 = assign38890_e52163_d_n7;
        var_tmp_dn8 = assign38890_e52163_d_n8;
        var_tmp_dn9 = assign38890_e52163_d_n9;

        let (assign38900_e52185, assign38900_e52185_d_n6, assign38900_e52185_d_n7, assign38900_e52185_d_n8, assign38900_e52185_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard808 == 0.0)) && (var_guard809 != 0.0)) && (var_guard810 == 0.0)) {
        let assign38900_e52180: f64 = (var_vav * var_vbrinvgat_d);
        let assign38900_e52181: f64 = (assign38900_e52180).abs();
        let assign38900_e52183: f64 = (assign38900_e52181).powf(var_pbrgatd_i);
        (assign38900_e52183, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign38900_e52181).powf(var_pbrgatd_i - 1.0) * if assign38900_e52180 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) })) } } else { (assign38900_e52183 * (var_pbrgatd_i * (if assign38900_e52180 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) } / assign38900_e52181))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign38900_e52181).powf(var_pbrgatd_i - 1.0) * if assign38900_e52180 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) })) } } else { (assign38900_e52183 * (var_pbrgatd_i * (if assign38900_e52180 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) } / assign38900_e52181))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign38900_e52181).powf(var_pbrgatd_i - 1.0) * if assign38900_e52180 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) })) } } else { (assign38900_e52183 * (var_pbrgatd_i * (if assign38900_e52180 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) } / assign38900_e52181))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign38900_e52181).powf(var_pbrgatd_i - 1.0) * if assign38900_e52180 >= 0.0 { (var_vav * var_vbrinvgat_d_dn9) } else { (-(var_vav * var_vbrinvgat_d_dn9)) })) } } else { (assign38900_e52183 * (var_pbrgatd_i * (if assign38900_e52180 >= 0.0 { (var_vav * var_vbrinvgat_d_dn9) } else { (-(var_vav * var_vbrinvgat_d_dn9)) } / assign38900_e52181))) },)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign38900_e52185;
        var_tmp_dn6 = assign38900_e52185_d_n6;
        var_tmp_dn7 = assign38900_e52185_d_n7;
        var_tmp_dn8 = assign38900_e52185_d_n8;
        var_tmp_dn9 = assign38900_e52185_d_n9;

        let (assign38910_e52203, assign38910_e52203_d_n6, assign38910_e52203_d_n7, assign38910_e52203_d_n8, assign38910_e52203_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard808 == 0.0)) && (var_guard809 != 0.0)) {
        let assign38910_e52200: f64 = (1.0 - var_tmp);
        let assign38910_e52201: f64 = (1.0 / assign38910_e52200);
        (assign38910_e52201, (-((-var_tmp_dn6) / (assign38910_e52200 * assign38910_e52200))), (-((-var_tmp_dn7) / (assign38910_e52200 * assign38910_e52200))), (-((-var_tmp_dn8) / (assign38910_e52200 * assign38910_e52200))), (-((-var_tmp_dn9) / (assign38910_e52200 * assign38910_e52200))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign38910_e52203;
        var_fbreakdown_dn6 = assign38910_e52203_d_n6;
        var_fbreakdown_dn7 = assign38910_e52203_d_n7;
        var_fbreakdown_dn8 = assign38910_e52203_d_n8;
        var_fbreakdown_dn9 = assign38910_e52203_d_n9;

        let (assign38920_e52226, assign38920_e52226_d_n6, assign38920_e52226_d_n7, assign38920_e52226_d_n8, assign38920_e52226_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) && (var_guard808 == 0.0)) && (var_guard809 == 0.0)) {
        let assign38920_e52220: f64 = (var_alphaav * var_vbrgatd_i);
        let assign38920_e52221: f64 = (var_vav + assign38920_e52220);
        let assign38920_e52223: f64 = (assign38920_e52221 * var_slopegat_d);
        let assign38920_e52224: f64 = (var_fstopgat_d + assign38920_e52223);
        (assign38920_e52224, (assign38920_e52221 * var_slopegat_d_dn6), (assign38920_e52221 * var_slopegat_d_dn7), (assign38920_e52221 * var_slopegat_d_dn8), (assign38920_e52221 * var_slopegat_d_dn9),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign38920_e52226;
        var_fbreakdown_dn6 = assign38920_e52226_d_n6;
        var_fbreakdown_dn7 = assign38920_e52226_d_n7;
        var_fbreakdown_dn8 = assign38920_e52226_d_n8;
        var_fbreakdown_dn9 = assign38920_e52226_d_n9;

        let (assign38930_e52245, assign38930_e52245_d_n6, assign38930_e52245_d_n7, assign38930_e52245_d_n8, assign38930_e52245_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard794 == 0.0)) {
        let assign38930_e52236: f64 = (var_id__blk212 + var_isrh);
        let assign38930_e52238: f64 = (assign38930_e52236 + var_itat);
        let assign38930_e52240: f64 = (assign38930_e52238 + var_ibbt);
        let assign38930_e52241: f64 = (p.p29 * assign38930_e52240);
        let assign38930_e52243: f64 = (assign38930_e52241 * var_fbreakdown);
        (assign38930_e52243, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign38930_e52241 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign38930_e52241 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign38930_e52241 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign38930_e52241 * var_fbreakdown_dn9)),)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign38930_e52245;
        var_ijungat_dn6 = assign38930_e52245_d_n6;
        var_ijungat_dn7 = assign38930_e52245_d_n7;
        var_ijungat_dn8 = assign38930_e52245_d_n8;
        var_ijungat_dn9 = assign38930_e52245_d_n9;

        let (assign38940_e52261, assign38940_e52261_d_n6, assign38940_e52261_d_n7, assign38940_e52261_d_n8, assign38940_e52261_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign38940_e52251: f64 = (var_abdrain_i * var_ijunbot);
        let assign38940_e52254: f64 = (var_lsdrain_i * var_ijunsti);
        let assign38940_e52255: f64 = (assign38940_e52251 + assign38940_e52254);
        let assign38940_e52258: f64 = (var_lgdrain_i * var_ijungat);
        let assign38940_e52259: f64 = (assign38940_e52255 + assign38940_e52258);
        (assign38940_e52259, (((var_abdrain_i * var_ijunbot_dn6) + (var_lsdrain_i * var_ijunsti_dn6)) + (var_lgdrain_i * var_ijungat_dn6)), (((var_abdrain_i * var_ijunbot_dn7) + (var_lsdrain_i * var_ijunsti_dn7)) + (var_lgdrain_i * var_ijungat_dn7)), (((var_abdrain_i * var_ijunbot_dn8) + (var_lsdrain_i * var_ijunsti_dn8)) + (var_lgdrain_i * var_ijungat_dn8)), (((var_abdrain_i * var_ijunbot_dn9) + (var_lsdrain_i * var_ijunsti_dn9)) + (var_lgdrain_i * var_ijungat_dn9)),)
    } else {
        (var_i5, var_i5_dn6, var_i5_dn7, var_i5_dn8, var_i5_dn9,)
    }
};
        var_i5 = assign38940_e52261;
        var_i5_dn6 = assign38940_e52261_d_n6;
        var_i5_dn7 = assign38940_e52261_d_n7;
        var_i5_dn8 = assign38940_e52261_d_n8;
        var_i5_dn9 = assign38940_e52261_d_n9;

        let (assign38950_e52277,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign38950_e52267: f64 = (var_abdrain_i * var_idsatbot_d);
        let assign38950_e52270: f64 = (var_lsdrain_i * var_idsatsti_d);
        let assign38950_e52271: f64 = (assign38950_e52267 + assign38950_e52270);
        let assign38950_e52274: f64 = (var_lgdrain_i * var_idsatgat_d);
        let assign38950_e52275: f64 = (assign38950_e52271 + assign38950_e52274);
        (assign38950_e52275,)
    } else {
        (var_isatfor1_d,)
    }
};
        var_isatfor1_d = assign38950_e52277;

        let (assign38960_e52294, assign38960_e52294_d_n6, assign38960_e52294_d_n7, assign38960_e52294_d_n8, assign38960_e52294_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign38960_e52285: f64 = (var_v4 * var_phitdinv);
        let assign38960_e52287: f64 = (assign38960_e52285 * var_mfor1_d);
        let assign38960_e52288: f64 = (assign38960_e52287).exp();
        let assign38960_e52290: f64 = (assign38960_e52288 - 1.0);
        let assign38960_e52291: f64 = (var_isatfor1_d * assign38960_e52290);
        let assign38960_e52292: f64 = (var_i4 - assign38960_e52291);
        (assign38960_e52292, var_i4_dn6, var_i4_dn7, var_i4_dn8, var_i4_dn9,)
    } else {
        (var_i4_cor, var_i4_cor_dn6, var_i4_cor_dn7, var_i4_cor_dn8, var_i4_cor_dn9,)
    }
};
        var_i4_cor = assign38960_e52294;
        var_i4_cor_dn6 = assign38960_e52294_d_n6;
        var_i4_cor_dn7 = assign38960_e52294_d_n7;
        var_i4_cor_dn8 = assign38960_e52294_d_n8;
        var_i4_cor_dn9 = assign38960_e52294_d_n9;

        let (assign38970_e52311, assign38970_e52311_d_n6, assign38970_e52311_d_n7, assign38970_e52311_d_n8, assign38970_e52311_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign38970_e52302: f64 = (var_v5 * var_phitdinv);
        let assign38970_e52304: f64 = (assign38970_e52302 * var_mfor1_d);
        let assign38970_e52305: f64 = (assign38970_e52304).exp();
        let assign38970_e52307: f64 = (assign38970_e52305 - 1.0);
        let assign38970_e52308: f64 = (var_isatfor1_d * assign38970_e52307);
        let assign38970_e52309: f64 = (var_i5 - assign38970_e52308);
        (assign38970_e52309, var_i5_dn6, var_i5_dn7, var_i5_dn8, var_i5_dn9,)
    } else {
        (var_i5_cor, var_i5_cor_dn6, var_i5_cor_dn7, var_i5_cor_dn8, var_i5_cor_dn9,)
    }
};
        var_i5_cor = assign38970_e52311;
        var_i5_cor_dn6 = assign38970_e52311_d_n6;
        var_i5_cor_dn7 = assign38970_e52311_d_n7;
        var_i5_cor_dn8 = assign38970_e52311_d_n8;
        var_i5_cor_dn9 = assign38970_e52311_d_n9;

        let assign38980_e52323: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard811 = assign38980_e52323;

        let assign38990_e52330: f64 = if ((var_i4 > 0.0) && (var_i5 > 0.0)) { 1.0 } else { 0.0 };
        var_guard812 = assign38990_e52330;

        let assign39000_e52333: f64 = (var_i4_cor / var_i4);
        let assign39000_e52338: f64 = (var_i5_cor / var_i5);
        let assign39000_e52353: f64 = if (((((assign39000_e52333 > 0.001) || (assign39000_e52338 > 0.001)) && (var_i4_cor > 0.0)) && (var_i5_cor > 0.0)) && (var_i5_cor > var_i4_cor)) { 1.0 } else { 0.0 };
        var_guard813 = assign39000_e52353;

        let (assign39010_e52367, assign39010_e52367_d_n6, assign39010_e52367_d_n7, assign39010_e52367_d_n8, assign39010_e52367_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) && (var_guard812 != 0.0)) && (var_guard813 != 0.0)) {
        let assign39010_e52365: f64 = (var_i4_cor / var_i5_cor);
        (assign39010_e52365, (((var_i4_cor_dn6 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn6)) / (var_i5_cor * var_i5_cor)), (((var_i4_cor_dn7 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn7)) / (var_i5_cor * var_i5_cor)), (((var_i4_cor_dn8 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn8)) / (var_i5_cor * var_i5_cor)), (((var_i4_cor_dn9 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn9)) / (var_i5_cor * var_i5_cor)),)
    } else {
        (var_alphaje, var_alphaje_dn6, var_alphaje_dn7, var_alphaje_dn8, var_alphaje_dn9,)
    }
};
        var_alphaje = assign39010_e52367;
        var_alphaje_dn6 = assign39010_e52367_d_n6;
        var_alphaje_dn7 = assign39010_e52367_d_n7;
        var_alphaje_dn8 = assign39010_e52367_d_n8;
        var_alphaje_dn9 = assign39010_e52367_d_n9;

        let (assign39020_e52386, assign39020_e52386_d_n6, assign39020_e52386_d_n7, assign39020_e52386_d_n8, assign39020_e52386_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) && (var_guard812 != 0.0)) && (var_guard813 != 0.0)) {
        let assign39020_e52379: f64 = (var_alphaje).ln();
        let assign39020_e52380: f64 = (var_phitd * assign39020_e52379);
        let assign39020_e52383: f64 = (var_v4 - var_v5);
        let assign39020_e52384: f64 = (assign39020_e52380 / assign39020_e52383);
        (assign39020_e52384, ((var_phitd * (var_alphaje_dn6 / var_alphaje)) / assign39020_e52383), ((var_phitd * (var_alphaje_dn7 / var_alphaje)) / assign39020_e52383), ((var_phitd * (var_alphaje_dn8 / var_alphaje)) / assign39020_e52383), ((var_phitd * (var_alphaje_dn9 / var_alphaje)) / assign39020_e52383),)
    } else {
        (var_mfor2_d, var_mfor2_d_dn6, var_mfor2_d_dn7, var_mfor2_d_dn8, var_mfor2_d_dn9,)
    }
};
        var_mfor2_d = assign39020_e52386;
        var_mfor2_d_dn6 = assign39020_e52386_d_n6;
        var_mfor2_d_dn7 = assign39020_e52386_d_n7;
        var_mfor2_d_dn8 = assign39020_e52386_d_n8;
        var_mfor2_d_dn9 = assign39020_e52386_d_n9;

        let (assign39030_e52407, assign39030_e52407_d_n6, assign39030_e52407_d_n7, assign39030_e52407_d_n8, assign39030_e52407_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) && (var_guard812 != 0.0)) && (var_guard813 != 0.0)) {
        let assign39030_e52399: f64 = (var_v4 * var_phitdinv);
        let assign39030_e52401: f64 = (assign39030_e52399 * var_mfor2_d);
        let assign39030_e52402: f64 = (assign39030_e52401).exp();
        let assign39030_e52404: f64 = (assign39030_e52402 - 1.0);
        let assign39030_e52405: f64 = (var_i4_cor / assign39030_e52404);
        (assign39030_e52405, (((var_i4_cor_dn6 * assign39030_e52404) - (var_i4_cor * (assign39030_e52402 * (assign39030_e52399 * var_mfor2_d_dn6)))) / (assign39030_e52404 * assign39030_e52404)), (((var_i4_cor_dn7 * assign39030_e52404) - (var_i4_cor * (assign39030_e52402 * (assign39030_e52399 * var_mfor2_d_dn7)))) / (assign39030_e52404 * assign39030_e52404)), (((var_i4_cor_dn8 * assign39030_e52404) - (var_i4_cor * (assign39030_e52402 * (assign39030_e52399 * var_mfor2_d_dn8)))) / (assign39030_e52404 * assign39030_e52404)), (((var_i4_cor_dn9 * assign39030_e52404) - (var_i4_cor * (assign39030_e52402 * (assign39030_e52399 * var_mfor2_d_dn9)))) / (assign39030_e52404 * assign39030_e52404)),)
    } else {
        (var_isatfor2_d, var_isatfor2_d_dn6, var_isatfor2_d_dn7, var_isatfor2_d_dn8, var_isatfor2_d_dn9,)
    }
};
        var_isatfor2_d = assign39030_e52407;
        var_isatfor2_d_dn6 = assign39030_e52407_d_n6;
        var_isatfor2_d_dn7 = assign39030_e52407_d_n7;
        var_isatfor2_d_dn8 = assign39030_e52407_d_n8;
        var_isatfor2_d_dn9 = assign39030_e52407_d_n9;

        let (assign39040_e52437, assign39040_e52437_d_n6, assign39040_e52437_d_n7, assign39040_e52437_d_n8, assign39040_e52437_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) {
        let assign39040_e52417: f64 = (var_v1 * var_phitdinv);
        let assign39040_e52419: f64 = (assign39040_e52417 * var_mfor1_d);
        let assign39040_e52420: f64 = (assign39040_e52419).exp();
        let assign39040_e52422: f64 = (assign39040_e52420 - 1.0);
        let assign39040_e52423: f64 = (var_isatfor1_d * assign39040_e52422);
        let assign39040_e52424: f64 = (var_i1 - assign39040_e52423);
        let assign39040_e52428: f64 = (var_v1 * var_phitdinv);
        let assign39040_e52430: f64 = (assign39040_e52428 * var_mfor2_d);
        let assign39040_e52431: f64 = (assign39040_e52430).exp();
        let assign39040_e52433: f64 = (assign39040_e52431 - 1.0);
        let assign39040_e52434: f64 = (var_isatfor2_d * assign39040_e52433);
        let assign39040_e52435: f64 = (assign39040_e52424 - assign39040_e52434);
        (assign39040_e52435, (var_i1_dn6 - ((var_isatfor2_d_dn6 * assign39040_e52433) + (var_isatfor2_d * (assign39040_e52431 * (assign39040_e52428 * var_mfor2_d_dn6))))), (var_i1_dn7 - ((var_isatfor2_d_dn7 * assign39040_e52433) + (var_isatfor2_d * (assign39040_e52431 * (assign39040_e52428 * var_mfor2_d_dn7))))), (var_i1_dn8 - ((var_isatfor2_d_dn8 * assign39040_e52433) + (var_isatfor2_d * (assign39040_e52431 * (assign39040_e52428 * var_mfor2_d_dn8))))), (var_i1_dn9 - ((var_isatfor2_d_dn9 * assign39040_e52433) + (var_isatfor2_d * (assign39040_e52431 * (assign39040_e52428 * var_mfor2_d_dn9))))),)
    } else {
        (var_i1_cor, var_i1_cor_dn6, var_i1_cor_dn7, var_i1_cor_dn8, var_i1_cor_dn9,)
    }
};
        var_i1_cor = assign39040_e52437;
        var_i1_cor_dn6 = assign39040_e52437_d_n6;
        var_i1_cor_dn7 = assign39040_e52437_d_n7;
        var_i1_cor_dn8 = assign39040_e52437_d_n8;
        var_i1_cor_dn9 = assign39040_e52437_d_n9;

        *var_alphaje_slot = var_alphaje;
        *var_alphaje_dn6_slot = var_alphaje_dn6;
        *var_alphaje_dn7_slot = var_alphaje_dn7;
        *var_alphaje_dn8_slot = var_alphaje_dn8;
        *var_alphaje_dn9_slot = var_alphaje_dn9;
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
        *var_guard804_slot = var_guard804;
        *var_guard805_slot = var_guard805;
        *var_guard806_slot = var_guard806;
        *var_guard807_slot = var_guard807;
        *var_guard808_slot = var_guard808;
        *var_guard809_slot = var_guard809;
        *var_guard810_slot = var_guard810;
        *var_guard811_slot = var_guard811;
        *var_guard812_slot = var_guard812;
        *var_guard813_slot = var_guard813;
        *var_i1_cor_slot = var_i1_cor;
        *var_i1_cor_dn6_slot = var_i1_cor_dn6;
        *var_i1_cor_dn7_slot = var_i1_cor_dn7;
        *var_i1_cor_dn8_slot = var_i1_cor_dn8;
        *var_i1_cor_dn9_slot = var_i1_cor_dn9;
        *var_i4_cor_slot = var_i4_cor;
        *var_i4_cor_dn6_slot = var_i4_cor_dn6;
        *var_i4_cor_dn7_slot = var_i4_cor_dn7;
        *var_i4_cor_dn8_slot = var_i4_cor_dn8;
        *var_i4_cor_dn9_slot = var_i4_cor_dn9;
        *var_i5_slot = var_i5;
        *var_i5_cor_slot = var_i5_cor;
        *var_i5_cor_dn6_slot = var_i5_cor_dn6;
        *var_i5_cor_dn7_slot = var_i5_cor_dn7;
        *var_i5_cor_dn8_slot = var_i5_cor_dn8;
        *var_i5_cor_dn9_slot = var_i5_cor_dn9;
        *var_i5_dn6_slot = var_i5_dn6;
        *var_i5_dn7_slot = var_i5_dn7;
        *var_i5_dn8_slot = var_i5_dn8;
        *var_i5_dn9_slot = var_i5_dn9;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_ijungat_dn9_slot = var_ijungat_dn9;
        *var_isatfor1_d_slot = var_isatfor1_d;
        *var_isatfor2_d_slot = var_isatfor2_d;
        *var_isatfor2_d_dn6_slot = var_isatfor2_d_dn6;
        *var_isatfor2_d_dn7_slot = var_isatfor2_d_dn7;
        *var_isatfor2_d_dn8_slot = var_isatfor2_d_dn8;
        *var_isatfor2_d_dn9_slot = var_isatfor2_d_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_mfor2_d_slot = var_mfor2_d;
        *var_mfor2_d_dn6_slot = var_mfor2_d_dn6;
        *var_mfor2_d_dn7_slot = var_mfor2_d_dn7;
        *var_mfor2_d_dn8_slot = var_mfor2_d_dn8;
        *var_mfor2_d_dn9_slot = var_mfor2_d_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
    }

    pub(super) fn stamp_transient_block_84(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_abdrain_i: f64,
        var_cjobot_d: f64,
        var_cjogat_d: f64,
        var_cjosti_d: f64,
        var_fjunqd_i: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard811: f64,
        var_i1: f64,
        var_i1_cor: f64,
        var_i1_cor_dn6: f64,
        var_i1_cor_dn7: f64,
        var_i1_cor_dn8: f64,
        var_i1_cor_dn9: f64,
        var_i2: f64,
        var_i2_dn6: f64,
        var_i2_dn7: f64,
        var_i2_dn8: f64,
        var_i2_dn9: f64,
        var_i3: f64,
        var_i3_dn6: f64,
        var_i3_dn7: f64,
        var_i3_dn8: f64,
        var_i3_dn9: f64,
        var_isatfor1_d: f64,
        var_isatfor2_d: f64,
        var_isatfor2_d_dn6: f64,
        var_isatfor2_d_dn7: f64,
        var_isatfor2_d_dn8: f64,
        var_isatfor2_d_dn9: f64,
        var_lgdrain_i: f64,
        var_lsdrain_i: f64,
        var_mfor1_d: f64,
        var_mfor2_d: f64,
        var_mfor2_d_dn6: f64,
        var_mfor2_d_dn7: f64,
        var_mfor2_d_dn8: f64,
        var_mfor2_d_dn9: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_tka: f64,
        var_tkr: f64,
        var_v1: f64,
        var_v2: f64,
        var_v3: f64,
        var_alphaje_slot: &mut f64,
        var_alphaje_dn6_slot: &mut f64,
        var_alphaje_dn7_slot: &mut f64,
        var_alphaje_dn8_slot: &mut f64,
        var_alphaje_dn9_slot: &mut f64,
        var_delt_slot: &mut f64,
        var_delt_dn4_slot: &mut f64,
        var_eg_slot: &mut f64,
        var_eg_dn4_slot: &mut f64,
        var_expxhf1_d_slot: &mut f64,
        var_expxhf2_d_slot: &mut f64,
        var_expxhf2_d_dn6_slot: &mut f64,
        var_expxhf2_d_dn7_slot: &mut f64,
        var_expxhf2_d_dn8_slot: &mut f64,
        var_expxhf2_d_dn9_slot: &mut f64,
        var_expxhr_d_slot: &mut f64,
        var_expxhr_d_dn6_slot: &mut f64,
        var_expxhr_d_dn7_slot: &mut f64,
        var_expxhr_d_dn8_slot: &mut f64,
        var_expxhr_d_dn9_slot: &mut f64,
        var_guard814_slot: &mut f64,
        var_guard815_slot: &mut f64,
        var_guard816_slot: &mut f64,
        var_guard817_slot: &mut f64,
        var_guard818_slot: &mut f64,
        var_guard819_slot: &mut f64,
        var_guard820_slot: &mut f64,
        var_i2_cor_slot: &mut f64,
        var_i2_cor_dn6_slot: &mut f64,
        var_i2_cor_dn7_slot: &mut f64,
        var_i2_cor_dn8_slot: &mut f64,
        var_i2_cor_dn9_slot: &mut f64,
        var_i3_cor_slot: &mut f64,
        var_i3_cor_dn6_slot: &mut f64,
        var_i3_cor_dn7_slot: &mut f64,
        var_i3_cor_dn8_slot: &mut f64,
        var_i3_cor_dn9_slot: &mut f64,
        var_inv_phit_slot: &mut f64,
        var_inv_phit_dn4_slot: &mut f64,
        var_isatrev_d_slot: &mut f64,
        var_isatrev_d_dn6_slot: &mut f64,
        var_isatrev_d_dn7_slot: &mut f64,
        var_isatrev_d_dn8_slot: &mut f64,
        var_isatrev_d_dn9_slot: &mut f64,
        var_ln_rtn_slot: &mut f64,
        var_ln_rtn_dn4_slot: &mut f64,
        var_m0_rev_slot: &mut f64,
        var_m0_rev_dn6_slot: &mut f64,
        var_m0_rev_dn7_slot: &mut f64,
        var_m0_rev_dn8_slot: &mut f64,
        var_m0_rev_dn9_slot: &mut f64,
        var_m0flag_d_slot: &mut f64,
        var_mcor_rev_slot: &mut f64,
        var_mcor_rev_dn6_slot: &mut f64,
        var_mcor_rev_dn7_slot: &mut f64,
        var_mcor_rev_dn8_slot: &mut f64,
        var_mcor_rev_dn9_slot: &mut f64,
        var_mrev_d_slot: &mut f64,
        var_mrev_d_dn6_slot: &mut f64,
        var_mrev_d_dn7_slot: &mut f64,
        var_mrev_d_dn8_slot: &mut f64,
        var_mrev_d_dn9_slot: &mut f64,
        var_phibfac_slot: &mut f64,
        var_phibfac_dn4_slot: &mut f64,
        var_phit_slot: &mut f64,
        var_phit_dn4_slot: &mut f64,
        var_rtn_slot: &mut f64,
        var_rtn_dn4_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp__blk949_slot: &mut f64,
        var_temp__blk949_dn4_slot: &mut f64,
        var_temp__blk949_dn6_slot: &mut f64,
        var_temp__blk949_dn7_slot: &mut f64,
        var_temp__blk949_dn8_slot: &mut f64,
        var_temp__blk949_dn9_slot: &mut f64,
        var_tkd_slot: &mut f64,
        var_tkd_dn4_slot: &mut f64,
        var_tkd_sq_slot: &mut f64,
        var_tkd_sq_dn4_slot: &mut f64,
        var_tt0_slot: &mut f64,
        var_tt1_slot: &mut f64,
        var_tt1_dn6_slot: &mut f64,
        var_tt1_dn7_slot: &mut f64,
        var_tt1_dn8_slot: &mut f64,
        var_tt1_dn9_slot: &mut f64,
        var_tt2_slot: &mut f64,
        var_tt2_dn6_slot: &mut f64,
        var_tt2_dn7_slot: &mut f64,
        var_tt2_dn8_slot: &mut f64,
        var_tt2_dn9_slot: &mut f64,
        var_xhighf1_d_slot: &mut f64,
        var_xhighf2_d_slot: &mut f64,
        var_xhighf2_d_dn6_slot: &mut f64,
        var_xhighf2_d_dn7_slot: &mut f64,
        var_xhighf2_d_dn8_slot: &mut f64,
        var_xhighf2_d_dn9_slot: &mut f64,
        var_xhighr_d_slot: &mut f64,
        var_xhighr_d_dn6_slot: &mut f64,
        var_xhighr_d_dn7_slot: &mut f64,
        var_xhighr_d_dn8_slot: &mut f64,
        var_xhighr_d_dn9_slot: &mut f64,
        var_zflagbot_d_slot: &mut f64,
        var_zflaggat_d_slot: &mut f64,
        var_zflagsti_d_slot: &mut f64,
        var_zfrac_slot: &mut f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let mut var_alphaje: f64 = *var_alphaje_slot;
        let mut var_alphaje_dn6: f64 = *var_alphaje_dn6_slot;
        let mut var_alphaje_dn7: f64 = *var_alphaje_dn7_slot;
        let mut var_alphaje_dn8: f64 = *var_alphaje_dn8_slot;
        let mut var_alphaje_dn9: f64 = *var_alphaje_dn9_slot;
        let mut var_delt: f64 = *var_delt_slot;
        let mut var_delt_dn4: f64 = *var_delt_dn4_slot;
        let mut var_eg: f64 = *var_eg_slot;
        let mut var_eg_dn4: f64 = *var_eg_dn4_slot;
        let mut var_expxhf1_d: f64 = *var_expxhf1_d_slot;
        let mut var_expxhf2_d: f64 = *var_expxhf2_d_slot;
        let mut var_expxhf2_d_dn6: f64 = *var_expxhf2_d_dn6_slot;
        let mut var_expxhf2_d_dn7: f64 = *var_expxhf2_d_dn7_slot;
        let mut var_expxhf2_d_dn8: f64 = *var_expxhf2_d_dn8_slot;
        let mut var_expxhf2_d_dn9: f64 = *var_expxhf2_d_dn9_slot;
        let mut var_expxhr_d: f64 = *var_expxhr_d_slot;
        let mut var_expxhr_d_dn6: f64 = *var_expxhr_d_dn6_slot;
        let mut var_expxhr_d_dn7: f64 = *var_expxhr_d_dn7_slot;
        let mut var_expxhr_d_dn8: f64 = *var_expxhr_d_dn8_slot;
        let mut var_expxhr_d_dn9: f64 = *var_expxhr_d_dn9_slot;
        let mut var_guard814: f64 = *var_guard814_slot;
        let mut var_guard815: f64 = *var_guard815_slot;
        let mut var_guard816: f64 = *var_guard816_slot;
        let mut var_guard817: f64 = *var_guard817_slot;
        let mut var_guard818: f64 = *var_guard818_slot;
        let mut var_guard819: f64 = *var_guard819_slot;
        let mut var_guard820: f64 = *var_guard820_slot;
        let mut var_i2_cor: f64 = *var_i2_cor_slot;
        let mut var_i2_cor_dn6: f64 = *var_i2_cor_dn6_slot;
        let mut var_i2_cor_dn7: f64 = *var_i2_cor_dn7_slot;
        let mut var_i2_cor_dn8: f64 = *var_i2_cor_dn8_slot;
        let mut var_i2_cor_dn9: f64 = *var_i2_cor_dn9_slot;
        let mut var_i3_cor: f64 = *var_i3_cor_slot;
        let mut var_i3_cor_dn6: f64 = *var_i3_cor_dn6_slot;
        let mut var_i3_cor_dn7: f64 = *var_i3_cor_dn7_slot;
        let mut var_i3_cor_dn8: f64 = *var_i3_cor_dn8_slot;
        let mut var_i3_cor_dn9: f64 = *var_i3_cor_dn9_slot;
        let mut var_inv_phit: f64 = *var_inv_phit_slot;
        let mut var_inv_phit_dn4: f64 = *var_inv_phit_dn4_slot;
        let mut var_isatrev_d: f64 = *var_isatrev_d_slot;
        let mut var_isatrev_d_dn6: f64 = *var_isatrev_d_dn6_slot;
        let mut var_isatrev_d_dn7: f64 = *var_isatrev_d_dn7_slot;
        let mut var_isatrev_d_dn8: f64 = *var_isatrev_d_dn8_slot;
        let mut var_isatrev_d_dn9: f64 = *var_isatrev_d_dn9_slot;
        let mut var_ln_rtn: f64 = *var_ln_rtn_slot;
        let mut var_ln_rtn_dn4: f64 = *var_ln_rtn_dn4_slot;
        let mut var_m0_rev: f64 = *var_m0_rev_slot;
        let mut var_m0_rev_dn6: f64 = *var_m0_rev_dn6_slot;
        let mut var_m0_rev_dn7: f64 = *var_m0_rev_dn7_slot;
        let mut var_m0_rev_dn8: f64 = *var_m0_rev_dn8_slot;
        let mut var_m0_rev_dn9: f64 = *var_m0_rev_dn9_slot;
        let mut var_m0flag_d: f64 = *var_m0flag_d_slot;
        let mut var_mcor_rev: f64 = *var_mcor_rev_slot;
        let mut var_mcor_rev_dn6: f64 = *var_mcor_rev_dn6_slot;
        let mut var_mcor_rev_dn7: f64 = *var_mcor_rev_dn7_slot;
        let mut var_mcor_rev_dn8: f64 = *var_mcor_rev_dn8_slot;
        let mut var_mcor_rev_dn9: f64 = *var_mcor_rev_dn9_slot;
        let mut var_mrev_d: f64 = *var_mrev_d_slot;
        let mut var_mrev_d_dn6: f64 = *var_mrev_d_dn6_slot;
        let mut var_mrev_d_dn7: f64 = *var_mrev_d_dn7_slot;
        let mut var_mrev_d_dn8: f64 = *var_mrev_d_dn8_slot;
        let mut var_mrev_d_dn9: f64 = *var_mrev_d_dn9_slot;
        let mut var_phibfac: f64 = *var_phibfac_slot;
        let mut var_phibfac_dn4: f64 = *var_phibfac_dn4_slot;
        let mut var_phit: f64 = *var_phit_slot;
        let mut var_phit_dn4: f64 = *var_phit_dn4_slot;
        let mut var_rtn: f64 = *var_rtn_slot;
        let mut var_rtn_dn4: f64 = *var_rtn_dn4_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp__blk949: f64 = *var_temp__blk949_slot;
        let mut var_temp__blk949_dn4: f64 = *var_temp__blk949_dn4_slot;
        let mut var_temp__blk949_dn6: f64 = *var_temp__blk949_dn6_slot;
        let mut var_temp__blk949_dn7: f64 = *var_temp__blk949_dn7_slot;
        let mut var_temp__blk949_dn8: f64 = *var_temp__blk949_dn8_slot;
        let mut var_temp__blk949_dn9: f64 = *var_temp__blk949_dn9_slot;
        let mut var_tkd: f64 = *var_tkd_slot;
        let mut var_tkd_dn4: f64 = *var_tkd_dn4_slot;
        let mut var_tkd_sq: f64 = *var_tkd_sq_slot;
        let mut var_tkd_sq_dn4: f64 = *var_tkd_sq_dn4_slot;
        let mut var_tt0: f64 = *var_tt0_slot;
        let mut var_tt1: f64 = *var_tt1_slot;
        let mut var_tt1_dn6: f64 = *var_tt1_dn6_slot;
        let mut var_tt1_dn7: f64 = *var_tt1_dn7_slot;
        let mut var_tt1_dn8: f64 = *var_tt1_dn8_slot;
        let mut var_tt1_dn9: f64 = *var_tt1_dn9_slot;
        let mut var_tt2: f64 = *var_tt2_slot;
        let mut var_tt2_dn6: f64 = *var_tt2_dn6_slot;
        let mut var_tt2_dn7: f64 = *var_tt2_dn7_slot;
        let mut var_tt2_dn8: f64 = *var_tt2_dn8_slot;
        let mut var_tt2_dn9: f64 = *var_tt2_dn9_slot;
        let mut var_xhighf1_d: f64 = *var_xhighf1_d_slot;
        let mut var_xhighf2_d: f64 = *var_xhighf2_d_slot;
        let mut var_xhighf2_d_dn6: f64 = *var_xhighf2_d_dn6_slot;
        let mut var_xhighf2_d_dn7: f64 = *var_xhighf2_d_dn7_slot;
        let mut var_xhighf2_d_dn8: f64 = *var_xhighf2_d_dn8_slot;
        let mut var_xhighf2_d_dn9: f64 = *var_xhighf2_d_dn9_slot;
        let mut var_xhighr_d: f64 = *var_xhighr_d_slot;
        let mut var_xhighr_d_dn6: f64 = *var_xhighr_d_dn6_slot;
        let mut var_xhighr_d_dn7: f64 = *var_xhighr_d_dn7_slot;
        let mut var_xhighr_d_dn8: f64 = *var_xhighr_d_dn8_slot;
        let mut var_xhighr_d_dn9: f64 = *var_xhighr_d_dn9_slot;
        let mut var_zflagbot_d: f64 = *var_zflagbot_d_slot;
        let mut var_zflaggat_d: f64 = *var_zflaggat_d_slot;
        let mut var_zflagsti_d: f64 = *var_zflagsti_d_slot;
        let mut var_zfrac: f64 = *var_zfrac_slot;

        let (assign39050_e52467, assign39050_e52467_d_n6, assign39050_e52467_d_n7, assign39050_e52467_d_n8, assign39050_e52467_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) {
        let assign39050_e52447: f64 = (var_v2 * var_phitdinv);
        let assign39050_e52449: f64 = (assign39050_e52447 * var_mfor1_d);
        let assign39050_e52450: f64 = (assign39050_e52449).exp();
        let assign39050_e52452: f64 = (assign39050_e52450 - 1.0);
        let assign39050_e52453: f64 = (var_isatfor1_d * assign39050_e52452);
        let assign39050_e52454: f64 = (var_i2 - assign39050_e52453);
        let assign39050_e52458: f64 = (var_v2 * var_phitdinv);
        let assign39050_e52460: f64 = (assign39050_e52458 * var_mfor2_d);
        let assign39050_e52461: f64 = (assign39050_e52460).exp();
        let assign39050_e52463: f64 = (assign39050_e52461 - 1.0);
        let assign39050_e52464: f64 = (var_isatfor2_d * assign39050_e52463);
        let assign39050_e52465: f64 = (assign39050_e52454 - assign39050_e52464);
        (assign39050_e52465, (var_i2_dn6 - ((var_isatfor2_d_dn6 * assign39050_e52463) + (var_isatfor2_d * (assign39050_e52461 * (assign39050_e52458 * var_mfor2_d_dn6))))), (var_i2_dn7 - ((var_isatfor2_d_dn7 * assign39050_e52463) + (var_isatfor2_d * (assign39050_e52461 * (assign39050_e52458 * var_mfor2_d_dn7))))), (var_i2_dn8 - ((var_isatfor2_d_dn8 * assign39050_e52463) + (var_isatfor2_d * (assign39050_e52461 * (assign39050_e52458 * var_mfor2_d_dn8))))), (var_i2_dn9 - ((var_isatfor2_d_dn9 * assign39050_e52463) + (var_isatfor2_d * (assign39050_e52461 * (assign39050_e52458 * var_mfor2_d_dn9))))),)
    } else {
        (var_i2_cor, var_i2_cor_dn6, var_i2_cor_dn7, var_i2_cor_dn8, var_i2_cor_dn9,)
    }
};
        var_i2_cor = assign39050_e52467;
        var_i2_cor_dn6 = assign39050_e52467_d_n6;
        var_i2_cor_dn7 = assign39050_e52467_d_n7;
        var_i2_cor_dn8 = assign39050_e52467_d_n8;
        var_i2_cor_dn9 = assign39050_e52467_d_n9;

        let (assign39060_e52497, assign39060_e52497_d_n6, assign39060_e52497_d_n7, assign39060_e52497_d_n8, assign39060_e52497_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) {
        let assign39060_e52477: f64 = (var_v3 * var_phitdinv);
        let assign39060_e52479: f64 = (assign39060_e52477 * var_mfor1_d);
        let assign39060_e52480: f64 = (assign39060_e52479).exp();
        let assign39060_e52482: f64 = (assign39060_e52480 - 1.0);
        let assign39060_e52483: f64 = (var_isatfor1_d * assign39060_e52482);
        let assign39060_e52484: f64 = (var_i3 - assign39060_e52483);
        let assign39060_e52488: f64 = (var_v3 * var_phitdinv);
        let assign39060_e52490: f64 = (assign39060_e52488 * var_mfor2_d);
        let assign39060_e52491: f64 = (assign39060_e52490).exp();
        let assign39060_e52493: f64 = (assign39060_e52491 - 1.0);
        let assign39060_e52494: f64 = (var_isatfor2_d * assign39060_e52493);
        let assign39060_e52495: f64 = (assign39060_e52484 - assign39060_e52494);
        (assign39060_e52495, (var_i3_dn6 - ((var_isatfor2_d_dn6 * assign39060_e52493) + (var_isatfor2_d * (assign39060_e52491 * (assign39060_e52488 * var_mfor2_d_dn6))))), (var_i3_dn7 - ((var_isatfor2_d_dn7 * assign39060_e52493) + (var_isatfor2_d * (assign39060_e52491 * (assign39060_e52488 * var_mfor2_d_dn7))))), (var_i3_dn8 - ((var_isatfor2_d_dn8 * assign39060_e52493) + (var_isatfor2_d * (assign39060_e52491 * (assign39060_e52488 * var_mfor2_d_dn8))))), (var_i3_dn9 - ((var_isatfor2_d_dn9 * assign39060_e52493) + (var_isatfor2_d * (assign39060_e52491 * (assign39060_e52488 * var_mfor2_d_dn9))))),)
    } else {
        (var_i3_cor, var_i3_cor_dn6, var_i3_cor_dn7, var_i3_cor_dn8, var_i3_cor_dn9,)
    }
};
        var_i3_cor = assign39060_e52497;
        var_i3_cor_dn6 = assign39060_e52497_d_n6;
        var_i3_cor_dn7 = assign39060_e52497_d_n7;
        var_i3_cor_dn8 = assign39060_e52497_d_n8;
        var_i3_cor_dn9 = assign39060_e52497_d_n9;

        let assign39070_e52508: f64 = if (((var_i1 < 0.0) && (var_i2 < 0.0)) && (var_i3 < 0.0)) { 1.0 } else { 0.0 };
        var_guard814 = assign39070_e52508;

        let assign39080_e52511: f64 = (var_i1_cor / var_i1);
        let assign39080_e52516: f64 = (var_i2_cor / var_i2);
        let assign39080_e52522: f64 = (var_i3_cor / var_i3);
        let assign39080_e52537: f64 = if ((((((assign39080_e52511 > 0.001) || (assign39080_e52516 > 0.001)) || (assign39080_e52522 > 0.001)) && (var_i1_cor < 0.0)) && (var_i2_cor < 0.0)) && (var_i3_cor < 0.0)) { 1.0 } else { 0.0 };
        var_guard815 = assign39080_e52537;

        let (assign39090_e52551, assign39090_e52551_d_n6, assign39090_e52551_d_n7, assign39090_e52551_d_n8, assign39090_e52551_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) && (var_guard814 != 0.0)) && (var_guard815 != 0.0)) {
        let assign39090_e52549: f64 = (var_i1_cor / var_i2_cor);
        (assign39090_e52549, (((var_i1_cor_dn6 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn6)) / (var_i2_cor * var_i2_cor)), (((var_i1_cor_dn7 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn7)) / (var_i2_cor * var_i2_cor)), (((var_i1_cor_dn8 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn8)) / (var_i2_cor * var_i2_cor)), (((var_i1_cor_dn9 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn9)) / (var_i2_cor * var_i2_cor)),)
    } else {
        (var_alphaje, var_alphaje_dn6, var_alphaje_dn7, var_alphaje_dn8, var_alphaje_dn9,)
    }
};
        var_alphaje = assign39090_e52551;
        var_alphaje_dn6 = assign39090_e52551_d_n6;
        var_alphaje_dn7 = assign39090_e52551_d_n7;
        var_alphaje_dn8 = assign39090_e52551_d_n8;
        var_alphaje_dn9 = assign39090_e52551_d_n9;

        let (assign39100_e52571, assign39100_e52571_d_n6, assign39100_e52571_d_n7, assign39100_e52571_d_n8, assign39100_e52571_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) && (var_guard814 != 0.0)) && (var_guard815 != 0.0)) {
        let assign39100_e52562: f64 = (-var_phitd);
        let assign39100_e52564: f64 = (var_alphaje).ln();
        let assign39100_e52565: f64 = (assign39100_e52562 * assign39100_e52564);
        let assign39100_e52568: f64 = (var_v1 - var_v2);
        let assign39100_e52569: f64 = (assign39100_e52565 / assign39100_e52568);
        (assign39100_e52569, ((assign39100_e52562 * (var_alphaje_dn6 / var_alphaje)) / assign39100_e52568), ((assign39100_e52562 * (var_alphaje_dn7 / var_alphaje)) / assign39100_e52568), ((assign39100_e52562 * (var_alphaje_dn8 / var_alphaje)) / assign39100_e52568), ((assign39100_e52562 * (var_alphaje_dn9 / var_alphaje)) / assign39100_e52568),)
    } else {
        (var_m0_rev, var_m0_rev_dn6, var_m0_rev_dn7, var_m0_rev_dn8, var_m0_rev_dn9,)
    }
};
        var_m0_rev = assign39100_e52571;
        var_m0_rev_dn6 = assign39100_e52571_d_n6;
        var_m0_rev_dn7 = assign39100_e52571_d_n7;
        var_m0_rev_dn8 = assign39100_e52571_d_n8;
        var_m0_rev_dn9 = assign39100_e52571_d_n9;

        let (assign39110_e52587,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) && (var_guard814 != 0.0)) && (var_guard815 != 0.0)) {
        let assign39110_e52584: f64 = (var_v2 - var_v1);
        let assign39110_e52585: f64 = (var_v2 / assign39110_e52584);
        (assign39110_e52585,)
    } else {
        (var_tt0,)
    }
};
        var_tt0 = assign39110_e52587;

        let (assign39120_e52609, assign39120_e52609_d_n6, assign39120_e52609_d_n7, assign39120_e52609_d_n8, assign39120_e52609_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) && (var_guard814 != 0.0)) && (var_guard815 != 0.0)) {
        let assign39120_e52600: f64 = (var_alphaje - 1.0);
        let assign39120_e52601: f64 = (var_phitd * assign39120_e52600);
        let assign39120_e52604: f64 = (var_alphaje).powf(var_tt0);
        let assign39120_e52606: f64 = (assign39120_e52604 - 1.0);
        let assign39120_e52607: f64 = (assign39120_e52601 * assign39120_e52606);
        (assign39120_e52607, (((var_phitd * var_alphaje_dn6) * assign39120_e52606) + (assign39120_e52601 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn6)) } } else { (assign39120_e52604 * (var_tt0 * (var_alphaje_dn6 / var_alphaje))) })), (((var_phitd * var_alphaje_dn7) * assign39120_e52606) + (assign39120_e52601 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn7)) } } else { (assign39120_e52604 * (var_tt0 * (var_alphaje_dn7 / var_alphaje))) })), (((var_phitd * var_alphaje_dn8) * assign39120_e52606) + (assign39120_e52601 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn8)) } } else { (assign39120_e52604 * (var_tt0 * (var_alphaje_dn8 / var_alphaje))) })), (((var_phitd * var_alphaje_dn9) * assign39120_e52606) + (assign39120_e52601 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn9)) } } else { (assign39120_e52604 * (var_tt0 * (var_alphaje_dn9 / var_alphaje))) })),)
    } else {
        (var_tt1, var_tt1_dn6, var_tt1_dn7, var_tt1_dn8, var_tt1_dn9,)
    }
};
        var_tt1 = assign39120_e52609;
        var_tt1_dn6 = assign39120_e52609_d_n6;
        var_tt1_dn7 = assign39120_e52609_d_n7;
        var_tt1_dn8 = assign39120_e52609_d_n8;
        var_tt1_dn9 = assign39120_e52609_d_n9;

        let (assign39130_e52625,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) && (var_guard814 != 0.0)) && (var_guard815 != 0.0)) {
        let assign39130_e52622: f64 = (var_v1 - var_v2);
        let assign39130_e52623: f64 = (var_v1 / assign39130_e52622);
        (assign39130_e52623,)
    } else {
        (var_tt0,)
    }
};
        var_tt0 = assign39130_e52625;

        let (assign39140_e52649, assign39140_e52649_d_n6, assign39140_e52649_d_n7, assign39140_e52649_d_n8, assign39140_e52649_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) && (var_guard814 != 0.0)) && (var_guard815 != 0.0)) {
        let assign39140_e52637: f64 = (var_alphaje).powf(var_tt0);
        let assign39140_e52640: f64 = (var_v2 - var_v1);
        let assign39140_e52641: f64 = (assign39140_e52637 * assign39140_e52640);
        let assign39140_e52644: f64 = (var_alphaje * var_v1);
        let assign39140_e52645: f64 = (assign39140_e52641 + assign39140_e52644);
        let assign39140_e52647: f64 = (assign39140_e52645 - var_v2);
        (assign39140_e52647, ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn6)) } } else { (assign39140_e52637 * (var_tt0 * (var_alphaje_dn6 / var_alphaje))) } * assign39140_e52640) + (var_alphaje_dn6 * var_v1)), ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn7)) } } else { (assign39140_e52637 * (var_tt0 * (var_alphaje_dn7 / var_alphaje))) } * assign39140_e52640) + (var_alphaje_dn7 * var_v1)), ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn8)) } } else { (assign39140_e52637 * (var_tt0 * (var_alphaje_dn8 / var_alphaje))) } * assign39140_e52640) + (var_alphaje_dn8 * var_v1)), ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn9)) } } else { (assign39140_e52637 * (var_tt0 * (var_alphaje_dn9 / var_alphaje))) } * assign39140_e52640) + (var_alphaje_dn9 * var_v1)),)
    } else {
        (var_tt2, var_tt2_dn6, var_tt2_dn7, var_tt2_dn8, var_tt2_dn9,)
    }
};
        var_tt2 = assign39140_e52649;
        var_tt2_dn6 = assign39140_e52649_d_n6;
        var_tt2_dn7 = assign39140_e52649_d_n7;
        var_tt2_dn8 = assign39140_e52649_d_n8;
        var_tt2_dn9 = assign39140_e52649_d_n9;

        let (assign39150_e52663, assign39150_e52663_d_n6, assign39150_e52663_d_n7, assign39150_e52663_d_n8, assign39150_e52663_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) && (var_guard814 != 0.0)) && (var_guard815 != 0.0)) {
        let assign39150_e52661: f64 = (var_tt1 / var_tt2);
        (assign39150_e52661, (((var_tt1_dn6 * var_tt2) - (var_tt1 * var_tt2_dn6)) / (var_tt2 * var_tt2)), (((var_tt1_dn7 * var_tt2) - (var_tt1 * var_tt2_dn7)) / (var_tt2 * var_tt2)), (((var_tt1_dn8 * var_tt2) - (var_tt1 * var_tt2_dn8)) / (var_tt2 * var_tt2)), (((var_tt1_dn9 * var_tt2) - (var_tt1 * var_tt2_dn9)) / (var_tt2 * var_tt2)),)
    } else {
        (var_mcor_rev, var_mcor_rev_dn6, var_mcor_rev_dn7, var_mcor_rev_dn8, var_mcor_rev_dn9,)
    }
};
        var_mcor_rev = assign39150_e52663;
        var_mcor_rev_dn6 = assign39150_e52663_d_n6;
        var_mcor_rev_dn7 = assign39150_e52663_d_n7;
        var_mcor_rev_dn8 = assign39150_e52663_d_n8;
        var_mcor_rev_dn9 = assign39150_e52663_d_n9;

        let (assign39160_e52677, assign39160_e52677_d_n6, assign39160_e52677_d_n7, assign39160_e52677_d_n8, assign39160_e52677_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) && (var_guard814 != 0.0)) && (var_guard815 != 0.0)) {
        let assign39160_e52675: f64 = (var_m0_rev + var_mcor_rev);
        (assign39160_e52675, (var_m0_rev_dn6 + var_mcor_rev_dn6), (var_m0_rev_dn7 + var_mcor_rev_dn7), (var_m0_rev_dn8 + var_mcor_rev_dn8), (var_m0_rev_dn9 + var_mcor_rev_dn9),)
    } else {
        (var_mrev_d, var_mrev_d_dn6, var_mrev_d_dn7, var_mrev_d_dn8, var_mrev_d_dn9,)
    }
};
        var_mrev_d = assign39160_e52677;
        var_mrev_d_dn6 = assign39160_e52677_d_n6;
        var_mrev_d_dn7 = assign39160_e52677_d_n7;
        var_mrev_d_dn8 = assign39160_e52677_d_n8;
        var_mrev_d_dn9 = assign39160_e52677_d_n9;

        let assign39170_e52680: f64 = (var_v3 * var_phitdinv);
        let assign39170_e52682: f64 = (assign39170_e52680 * var_mrev_d);
        let assign39170_e52683: f64 = (assign39170_e52682).abs();
        let assign39170_e52685: f64 = if assign39170_e52683 < 1e-6 { 1.0 } else { 0.0 };
        var_guard816 = assign39170_e52685;

        let (assign39180_e52699,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) && (var_guard814 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 != 0.0)) {
        (1.0,)
    } else {
        (var_m0flag_d,)
    }
};
        var_m0flag_d = assign39180_e52699;

        let (assign39190_e52723, assign39190_e52723_d_n6, assign39190_e52723_d_n7, assign39190_e52723_d_n8, assign39190_e52723_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) && (var_guard814 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 != 0.0)) {
        let assign39190_e52714: f64 = (1.0 / var_v3);
        let assign39190_e52717: f64 = (0.5 * var_phitdinv);
        let assign39190_e52719: f64 = (assign39190_e52717 * var_mrev_d);
        let assign39190_e52720: f64 = (assign39190_e52714 + assign39190_e52719);
        let assign39190_e52721: f64 = (var_i3_cor * assign39190_e52720);
        (assign39190_e52721, ((var_i3_cor_dn6 * assign39190_e52720) + (var_i3_cor * (assign39190_e52717 * var_mrev_d_dn6))), ((var_i3_cor_dn7 * assign39190_e52720) + (var_i3_cor * (assign39190_e52717 * var_mrev_d_dn7))), ((var_i3_cor_dn8 * assign39190_e52720) + (var_i3_cor * (assign39190_e52717 * var_mrev_d_dn8))), ((var_i3_cor_dn9 * assign39190_e52720) + (var_i3_cor * (assign39190_e52717 * var_mrev_d_dn9))),)
    } else {
        (var_isatrev_d, var_isatrev_d_dn6, var_isatrev_d_dn7, var_isatrev_d_dn8, var_isatrev_d_dn9,)
    }
};
        var_isatrev_d = assign39190_e52723;
        var_isatrev_d_dn6 = assign39190_e52723_d_n6;
        var_isatrev_d_dn7 = assign39190_e52723_d_n7;
        var_isatrev_d_dn8 = assign39190_e52723_d_n8;
        var_isatrev_d_dn9 = assign39190_e52723_d_n9;

        let (assign39200_e52746, assign39200_e52746_d_n6, assign39200_e52746_d_n7, assign39200_e52746_d_n8, assign39200_e52746_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) && (var_guard814 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 != 0.0)) {
        let assign39200_e52736: f64 = (-0.5);
        let assign39200_e52738: f64 = (assign39200_e52736 * var_i3_cor);
        let assign39200_e52740: f64 = (assign39200_e52738 * var_mrev_d);
        let assign39200_e52742: f64 = (assign39200_e52740 * var_phitdinv);
        let assign39200_e52744: f64 = (assign39200_e52742 / var_v3);
        (assign39200_e52744, (((((assign39200_e52736 * var_i3_cor_dn6) * var_mrev_d) + (assign39200_e52738 * var_mrev_d_dn6)) * var_phitdinv) / var_v3), (((((assign39200_e52736 * var_i3_cor_dn7) * var_mrev_d) + (assign39200_e52738 * var_mrev_d_dn7)) * var_phitdinv) / var_v3), (((((assign39200_e52736 * var_i3_cor_dn8) * var_mrev_d) + (assign39200_e52738 * var_mrev_d_dn8)) * var_phitdinv) / var_v3), (((((assign39200_e52736 * var_i3_cor_dn9) * var_mrev_d) + (assign39200_e52738 * var_mrev_d_dn9)) * var_phitdinv) / var_v3),)
    } else {
        (var_mrev_d, var_mrev_d_dn6, var_mrev_d_dn7, var_mrev_d_dn8, var_mrev_d_dn9,)
    }
};
        var_mrev_d = assign39200_e52746;
        var_mrev_d_dn6 = assign39200_e52746_d_n6;
        var_mrev_d_dn7 = assign39200_e52746_d_n7;
        var_mrev_d_dn8 = assign39200_e52746_d_n8;
        var_mrev_d_dn9 = assign39200_e52746_d_n9;

        let (assign39210_e52761,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) && (var_guard814 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 == 0.0)) {
        (0.0,)
    } else {
        (var_m0flag_d,)
    }
};
        var_m0flag_d = assign39210_e52761;

        let (assign39220_e52787, assign39220_e52787_d_n6, assign39220_e52787_d_n7, assign39220_e52787_d_n8, assign39220_e52787_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard811 != 0.0)) && (var_guard814 != 0.0)) && (var_guard815 != 0.0)) && (var_guard816 == 0.0)) {
        let assign39220_e52775: f64 = (-var_i3_cor);
        let assign39220_e52777: f64 = (-var_v3);
        let assign39220_e52779: f64 = (assign39220_e52777 * var_phitdinv);
        let assign39220_e52781: f64 = (assign39220_e52779 * var_mrev_d);
        let assign39220_e52782: f64 = (assign39220_e52781).exp();
        let assign39220_e52784: f64 = (assign39220_e52782 - 1.0);
        let assign39220_e52785: f64 = (assign39220_e52775 / assign39220_e52784);
        (assign39220_e52785, ((((-var_i3_cor_dn6) * assign39220_e52784) - (assign39220_e52775 * (assign39220_e52782 * (assign39220_e52779 * var_mrev_d_dn6)))) / (assign39220_e52784 * assign39220_e52784)), ((((-var_i3_cor_dn7) * assign39220_e52784) - (assign39220_e52775 * (assign39220_e52782 * (assign39220_e52779 * var_mrev_d_dn7)))) / (assign39220_e52784 * assign39220_e52784)), ((((-var_i3_cor_dn8) * assign39220_e52784) - (assign39220_e52775 * (assign39220_e52782 * (assign39220_e52779 * var_mrev_d_dn8)))) / (assign39220_e52784 * assign39220_e52784)), ((((-var_i3_cor_dn9) * assign39220_e52784) - (assign39220_e52775 * (assign39220_e52782 * (assign39220_e52779 * var_mrev_d_dn9)))) / (assign39220_e52784 * assign39220_e52784)),)
    } else {
        (var_isatrev_d, var_isatrev_d_dn6, var_isatrev_d_dn7, var_isatrev_d_dn8, var_isatrev_d_dn9,)
    }
};
        var_isatrev_d = assign39220_e52787;
        var_isatrev_d_dn6 = assign39220_e52787_d_n6;
        var_isatrev_d_dn7 = assign39220_e52787_d_n7;
        var_isatrev_d_dn8 = assign39220_e52787_d_n8;
        var_isatrev_d_dn9 = assign39220_e52787_d_n9;

        let (assign39230_e52805,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign39230_e52794: f64 = (var_abdrain_i * var_cjobot_d);
        let assign39230_e52797: f64 = (var_lsdrain_i * var_cjosti_d);
        let assign39230_e52798: f64 = (assign39230_e52794 + assign39230_e52797);
        let assign39230_e52801: f64 = (var_lgdrain_i * var_cjogat_d);
        let assign39230_e52802: f64 = (assign39230_e52798 + assign39230_e52801);
        let assign39230_e52803: f64 = (var_fjunqd_i * assign39230_e52802);
        (assign39230_e52803,)
    } else {
        (var_zfrac,)
    }
};
        var_zfrac = assign39230_e52805;

        let assign39240_e52808: f64 = (var_abdrain_i * var_cjobot_d);
        let assign39240_e52810: f64 = if assign39240_e52808 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard817 = assign39240_e52810;

        let (assign39250_e52818,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard817 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagbot_d,)
    }
};
        var_zflagbot_d = assign39250_e52818;

        let assign39260_e52821: f64 = (var_lsdrain_i * var_cjosti_d);
        let assign39260_e52823: f64 = if assign39260_e52821 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard818 = assign39260_e52823;

        let (assign39270_e52831,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard818 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagsti_d,)
    }
};
        var_zflagsti_d = assign39270_e52831;

        let assign39280_e52834: f64 = (var_lgdrain_i * var_cjogat_d);
        let assign39280_e52836: f64 = if assign39280_e52834 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard819 = assign39280_e52836;

        let (assign39290_e52844,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard819 != 0.0)) {
        (0.0,)
    } else {
        (var_zflaggat_d,)
    }
};
        var_zflaggat_d = assign39290_e52844;

        let assign39300_e52856: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard820 = assign39300_e52856;

        let (assign39310_e52871,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard820 != 0.0)) {
        let assign39310_e52864: f64 = (0.5 * p.p839);
        let assign39310_e52867: f64 = (var_isatfor1_d + 1e-21);
        let assign39310_e52868: f64 = (assign39310_e52864 / assign39310_e52867);
        let assign39310_e52869: f64 = (assign39310_e52868).ln();
        (assign39310_e52869,)
    } else {
        (var_xhighf1_d,)
    }
};
        var_xhighf1_d = assign39310_e52871;

        let (assign39320_e52886, assign39320_e52886_d_n6, assign39320_e52886_d_n7, assign39320_e52886_d_n8, assign39320_e52886_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard820 != 0.0)) {
        let assign39320_e52879: f64 = (0.5 * p.p839);
        let assign39320_e52882: f64 = (var_isatfor2_d + 1e-21);
        let assign39320_e52883: f64 = (assign39320_e52879 / assign39320_e52882);
        let assign39320_e52884: f64 = (assign39320_e52883).ln();
        (assign39320_e52884, ((-((assign39320_e52879 * var_isatfor2_d_dn6) / (assign39320_e52882 * assign39320_e52882))) / assign39320_e52883), ((-((assign39320_e52879 * var_isatfor2_d_dn7) / (assign39320_e52882 * assign39320_e52882))) / assign39320_e52883), ((-((assign39320_e52879 * var_isatfor2_d_dn8) / (assign39320_e52882 * assign39320_e52882))) / assign39320_e52883), ((-((assign39320_e52879 * var_isatfor2_d_dn9) / (assign39320_e52882 * assign39320_e52882))) / assign39320_e52883),)
    } else {
        (var_xhighf2_d, var_xhighf2_d_dn6, var_xhighf2_d_dn7, var_xhighf2_d_dn8, var_xhighf2_d_dn9,)
    }
};
        var_xhighf2_d = assign39320_e52886;
        var_xhighf2_d_dn6 = assign39320_e52886_d_n6;
        var_xhighf2_d_dn7 = assign39320_e52886_d_n7;
        var_xhighf2_d_dn8 = assign39320_e52886_d_n8;
        var_xhighf2_d_dn9 = assign39320_e52886_d_n9;

        let (assign39330_e52902, assign39330_e52902_d_n6, assign39330_e52902_d_n7, assign39330_e52902_d_n8, assign39330_e52902_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard820 != 0.0)) {
        let assign39330_e52894: f64 = (0.5 * p.p839);
        let assign39330_e52896: f64 = (var_isatrev_d).abs();
        let assign39330_e52898: f64 = (assign39330_e52896 + 1e-21);
        let assign39330_e52899: f64 = (assign39330_e52894 / assign39330_e52898);
        let assign39330_e52900: f64 = (assign39330_e52899).ln();
        (assign39330_e52900, ((-((assign39330_e52894 * if var_isatrev_d >= 0.0 { var_isatrev_d_dn6 } else { (-var_isatrev_d_dn6) }) / (assign39330_e52898 * assign39330_e52898))) / assign39330_e52899), ((-((assign39330_e52894 * if var_isatrev_d >= 0.0 { var_isatrev_d_dn7 } else { (-var_isatrev_d_dn7) }) / (assign39330_e52898 * assign39330_e52898))) / assign39330_e52899), ((-((assign39330_e52894 * if var_isatrev_d >= 0.0 { var_isatrev_d_dn8 } else { (-var_isatrev_d_dn8) }) / (assign39330_e52898 * assign39330_e52898))) / assign39330_e52899), ((-((assign39330_e52894 * if var_isatrev_d >= 0.0 { var_isatrev_d_dn9 } else { (-var_isatrev_d_dn9) }) / (assign39330_e52898 * assign39330_e52898))) / assign39330_e52899),)
    } else {
        (var_xhighr_d, var_xhighr_d_dn6, var_xhighr_d_dn7, var_xhighr_d_dn8, var_xhighr_d_dn9,)
    }
};
        var_xhighr_d = assign39330_e52902;
        var_xhighr_d_dn6 = assign39330_e52902_d_n6;
        var_xhighr_d_dn7 = assign39330_e52902_d_n7;
        var_xhighr_d_dn8 = assign39330_e52902_d_n8;
        var_xhighr_d_dn9 = assign39330_e52902_d_n9;

        let (assign39340_e52910,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign39340_e52908: f64 = (var_xhighf1_d).min(230.25850929940458);
        (assign39340_e52908,)
    } else {
        (var_xhighf1_d,)
    }
};
        var_xhighf1_d = assign39340_e52910;

        let (assign39350_e52917,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign39350_e52915: f64 = (var_xhighf1_d).exp();
        (assign39350_e52915,)
    } else {
        (var_expxhf1_d,)
    }
};
        var_expxhf1_d = assign39350_e52917;

        let (assign39360_e52925, assign39360_e52925_d_n6, assign39360_e52925_d_n7, assign39360_e52925_d_n8, assign39360_e52925_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign39360_e52923: f64 = (var_xhighf2_d).min(230.25850929940458);
        (assign39360_e52923, if var_xhighf2_d <= 230.25850929940458 { var_xhighf2_d_dn6 } else { 0.0 }, if var_xhighf2_d <= 230.25850929940458 { var_xhighf2_d_dn7 } else { 0.0 }, if var_xhighf2_d <= 230.25850929940458 { var_xhighf2_d_dn8 } else { 0.0 }, if var_xhighf2_d <= 230.25850929940458 { var_xhighf2_d_dn9 } else { 0.0 },)
    } else {
        (var_xhighf2_d, var_xhighf2_d_dn6, var_xhighf2_d_dn7, var_xhighf2_d_dn8, var_xhighf2_d_dn9,)
    }
};
        var_xhighf2_d = assign39360_e52925;
        var_xhighf2_d_dn6 = assign39360_e52925_d_n6;
        var_xhighf2_d_dn7 = assign39360_e52925_d_n7;
        var_xhighf2_d_dn8 = assign39360_e52925_d_n8;
        var_xhighf2_d_dn9 = assign39360_e52925_d_n9;

        let (assign39370_e52932, assign39370_e52932_d_n6, assign39370_e52932_d_n7, assign39370_e52932_d_n8, assign39370_e52932_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign39370_e52930: f64 = (var_xhighf2_d).exp();
        (assign39370_e52930, (assign39370_e52930 * var_xhighf2_d_dn6), (assign39370_e52930 * var_xhighf2_d_dn7), (assign39370_e52930 * var_xhighf2_d_dn8), (assign39370_e52930 * var_xhighf2_d_dn9),)
    } else {
        (var_expxhf2_d, var_expxhf2_d_dn6, var_expxhf2_d_dn7, var_expxhf2_d_dn8, var_expxhf2_d_dn9,)
    }
};
        var_expxhf2_d = assign39370_e52932;
        var_expxhf2_d_dn6 = assign39370_e52932_d_n6;
        var_expxhf2_d_dn7 = assign39370_e52932_d_n7;
        var_expxhf2_d_dn8 = assign39370_e52932_d_n8;
        var_expxhf2_d_dn9 = assign39370_e52932_d_n9;

        let (assign39380_e52940, assign39380_e52940_d_n6, assign39380_e52940_d_n7, assign39380_e52940_d_n8, assign39380_e52940_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign39380_e52938: f64 = (var_xhighr_d).min(230.25850929940458);
        (assign39380_e52938, if var_xhighr_d <= 230.25850929940458 { var_xhighr_d_dn6 } else { 0.0 }, if var_xhighr_d <= 230.25850929940458 { var_xhighr_d_dn7 } else { 0.0 }, if var_xhighr_d <= 230.25850929940458 { var_xhighr_d_dn8 } else { 0.0 }, if var_xhighr_d <= 230.25850929940458 { var_xhighr_d_dn9 } else { 0.0 },)
    } else {
        (var_xhighr_d, var_xhighr_d_dn6, var_xhighr_d_dn7, var_xhighr_d_dn8, var_xhighr_d_dn9,)
    }
};
        var_xhighr_d = assign39380_e52940;
        var_xhighr_d_dn6 = assign39380_e52940_d_n6;
        var_xhighr_d_dn7 = assign39380_e52940_d_n7;
        var_xhighr_d_dn8 = assign39380_e52940_d_n8;
        var_xhighr_d_dn9 = assign39380_e52940_d_n9;

        let (assign39390_e52947, assign39390_e52947_d_n6, assign39390_e52947_d_n7, assign39390_e52947_d_n8, assign39390_e52947_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign39390_e52945: f64 = (var_xhighr_d).exp();
        (assign39390_e52945, (assign39390_e52945 * var_xhighr_d_dn6), (assign39390_e52945 * var_xhighr_d_dn7), (assign39390_e52945 * var_xhighr_d_dn8), (assign39390_e52945 * var_xhighr_d_dn9),)
    } else {
        (var_expxhr_d, var_expxhr_d_dn6, var_expxhr_d_dn7, var_expxhr_d_dn8, var_expxhr_d_dn9,)
    }
};
        var_expxhr_d = assign39390_e52947;
        var_expxhr_d_dn6 = assign39390_e52947_d_n6;
        var_expxhr_d_dn7 = assign39390_e52947_d_n7;
        var_expxhr_d_dn8 = assign39390_e52947_d_n8;
        var_expxhr_d_dn9 = assign39390_e52947_d_n9;

        var_temp__blk949 = 0.0;
        var_temp__blk949_dn4 = 0.0;
        var_temp__blk949_dn6 = 0.0;
        var_temp__blk949_dn7 = 0.0;
        var_temp__blk949_dn8 = 0.0;
        var_temp__blk949_dn9 = 0.0;

        var_temp1 = 0.0;
        var_temp1_dn4 = 0.0;
        var_temp1_dn6 = 0.0;
        var_temp1_dn7 = 0.0;
        var_temp1_dn8 = 0.0;
        var_temp1_dn9 = 0.0;

        var_temp2 = 0.0;
        var_temp2_dn4 = 0.0;
        var_temp2_dn6 = 0.0;
        var_temp2_dn7 = 0.0;
        var_temp2_dn8 = 0.0;
        var_temp2_dn9 = 0.0;

        let assign39430_e52953: f64 = (var_tka + (nv4 - 0.0));
        var_tkd = assign39430_e52953;
        var_tkd_dn4 = 1.0;

        let assign39440_e52956: f64 = (var_tkd * var_tkd);
        var_tkd_sq = assign39440_e52956;
        var_tkd_sq_dn4 = ((var_tkd_dn4 * var_tkd) + (var_tkd * var_tkd_dn4));

        let assign39450_e52959: f64 = (var_tkd - var_tkr);
        var_delt = assign39450_e52959;
        var_delt_dn4 = var_tkd_dn4;

        let assign39460_e52962: f64 = (var_tkr / var_tkd);
        var_rtn = assign39460_e52962;
        var_rtn_dn4 = (-((var_tkr * var_tkd_dn4) / (var_tkd * var_tkd)));

        let assign39470_e52964: f64 = (var_rtn).ln();
        var_ln_rtn = assign39470_e52964;
        var_ln_rtn_dn4 = (var_rtn_dn4 / var_rtn);

        let assign39480_e52967: f64 = (var_tkd * 1.3806505e-23);
        let assign39480_e52969: f64 = (assign39480_e52967 / 1.6021918e-19);
        var_phit = assign39480_e52969;
        var_phit_dn4 = ((var_tkd_dn4 * 1.3806505e-23) / 1.6021918e-19);

        let assign39490_e52972: f64 = (1.0 / var_phit);
        var_inv_phit = assign39490_e52972;
        var_inv_phit_dn4 = (-(var_phit_dn4 / (var_phit * var_phit)));

        let assign39500_e52976: f64 = (9.025e-5 * var_tkd);
        let assign39500_e52977: f64 = (1.179 - assign39500_e52976);
        let assign39500_e52980: f64 = (3.05e-7 * var_tkd_sq);
        let assign39500_e52981: f64 = (assign39500_e52977 - assign39500_e52980);
        var_eg = assign39500_e52981;
        var_eg_dn4 = ((-(9.025e-5 * var_tkd_dn4)) - (3.05e-7 * var_tkd_sq_dn4));

        let assign39510_e52985: f64 = (0.00045 * var_tkd);
        let assign39510_e52986: f64 = (1.045 + assign39510_e52985);
        let assign39510_e52990: f64 = (0.0014 * var_tkd);
        let assign39510_e52991: f64 = (0.523 + assign39510_e52990);
        let assign39510_e52994: f64 = (1.48e-6 * var_tkd_sq);
        let assign39510_e52995: f64 = (assign39510_e52991 - assign39510_e52994);
        let assign39510_e52996: f64 = (assign39510_e52986 * assign39510_e52995);
        let assign39510_e52998: f64 = (assign39510_e52996 * var_tkd_sq);
        let assign39510_e53000: f64 = (assign39510_e52998 / 90000.0);
        var_phibfac = assign39510_e53000;
        var_phibfac_dn4 = ((((((0.00045 * var_tkd_dn4) * assign39510_e52995) + (assign39510_e52986 * ((0.0014 * var_tkd_dn4) - (1.48e-6 * var_tkd_sq_dn4)))) * var_tkd_sq) + (assign39510_e52996 * var_tkd_sq_dn4)) / 90000.0);

        *var_alphaje_slot = var_alphaje;
        *var_alphaje_dn6_slot = var_alphaje_dn6;
        *var_alphaje_dn7_slot = var_alphaje_dn7;
        *var_alphaje_dn8_slot = var_alphaje_dn8;
        *var_alphaje_dn9_slot = var_alphaje_dn9;
        *var_delt_slot = var_delt;
        *var_delt_dn4_slot = var_delt_dn4;
        *var_eg_slot = var_eg;
        *var_eg_dn4_slot = var_eg_dn4;
        *var_expxhf1_d_slot = var_expxhf1_d;
        *var_expxhf2_d_slot = var_expxhf2_d;
        *var_expxhf2_d_dn6_slot = var_expxhf2_d_dn6;
        *var_expxhf2_d_dn7_slot = var_expxhf2_d_dn7;
        *var_expxhf2_d_dn8_slot = var_expxhf2_d_dn8;
        *var_expxhf2_d_dn9_slot = var_expxhf2_d_dn9;
        *var_expxhr_d_slot = var_expxhr_d;
        *var_expxhr_d_dn6_slot = var_expxhr_d_dn6;
        *var_expxhr_d_dn7_slot = var_expxhr_d_dn7;
        *var_expxhr_d_dn8_slot = var_expxhr_d_dn8;
        *var_expxhr_d_dn9_slot = var_expxhr_d_dn9;
        *var_guard814_slot = var_guard814;
        *var_guard815_slot = var_guard815;
        *var_guard816_slot = var_guard816;
        *var_guard817_slot = var_guard817;
        *var_guard818_slot = var_guard818;
        *var_guard819_slot = var_guard819;
        *var_guard820_slot = var_guard820;
        *var_i2_cor_slot = var_i2_cor;
        *var_i2_cor_dn6_slot = var_i2_cor_dn6;
        *var_i2_cor_dn7_slot = var_i2_cor_dn7;
        *var_i2_cor_dn8_slot = var_i2_cor_dn8;
        *var_i2_cor_dn9_slot = var_i2_cor_dn9;
        *var_i3_cor_slot = var_i3_cor;
        *var_i3_cor_dn6_slot = var_i3_cor_dn6;
        *var_i3_cor_dn7_slot = var_i3_cor_dn7;
        *var_i3_cor_dn8_slot = var_i3_cor_dn8;
        *var_i3_cor_dn9_slot = var_i3_cor_dn9;
        *var_inv_phit_slot = var_inv_phit;
        *var_inv_phit_dn4_slot = var_inv_phit_dn4;
        *var_isatrev_d_slot = var_isatrev_d;
        *var_isatrev_d_dn6_slot = var_isatrev_d_dn6;
        *var_isatrev_d_dn7_slot = var_isatrev_d_dn7;
        *var_isatrev_d_dn8_slot = var_isatrev_d_dn8;
        *var_isatrev_d_dn9_slot = var_isatrev_d_dn9;
        *var_ln_rtn_slot = var_ln_rtn;
        *var_ln_rtn_dn4_slot = var_ln_rtn_dn4;
        *var_m0_rev_slot = var_m0_rev;
        *var_m0_rev_dn6_slot = var_m0_rev_dn6;
        *var_m0_rev_dn7_slot = var_m0_rev_dn7;
        *var_m0_rev_dn8_slot = var_m0_rev_dn8;
        *var_m0_rev_dn9_slot = var_m0_rev_dn9;
        *var_m0flag_d_slot = var_m0flag_d;
        *var_mcor_rev_slot = var_mcor_rev;
        *var_mcor_rev_dn6_slot = var_mcor_rev_dn6;
        *var_mcor_rev_dn7_slot = var_mcor_rev_dn7;
        *var_mcor_rev_dn8_slot = var_mcor_rev_dn8;
        *var_mcor_rev_dn9_slot = var_mcor_rev_dn9;
        *var_mrev_d_slot = var_mrev_d;
        *var_mrev_d_dn6_slot = var_mrev_d_dn6;
        *var_mrev_d_dn7_slot = var_mrev_d_dn7;
        *var_mrev_d_dn8_slot = var_mrev_d_dn8;
        *var_mrev_d_dn9_slot = var_mrev_d_dn9;
        *var_phibfac_slot = var_phibfac;
        *var_phibfac_dn4_slot = var_phibfac_dn4;
        *var_phit_slot = var_phit;
        *var_phit_dn4_slot = var_phit_dn4;
        *var_rtn_slot = var_rtn;
        *var_rtn_dn4_slot = var_rtn_dn4;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp__blk949_slot = var_temp__blk949;
        *var_temp__blk949_dn4_slot = var_temp__blk949_dn4;
        *var_temp__blk949_dn6_slot = var_temp__blk949_dn6;
        *var_temp__blk949_dn7_slot = var_temp__blk949_dn7;
        *var_temp__blk949_dn8_slot = var_temp__blk949_dn8;
        *var_temp__blk949_dn9_slot = var_temp__blk949_dn9;
        *var_tkd_slot = var_tkd;
        *var_tkd_dn4_slot = var_tkd_dn4;
        *var_tkd_sq_slot = var_tkd_sq;
        *var_tkd_sq_dn4_slot = var_tkd_sq_dn4;
        *var_tt0_slot = var_tt0;
        *var_tt1_slot = var_tt1;
        *var_tt1_dn6_slot = var_tt1_dn6;
        *var_tt1_dn7_slot = var_tt1_dn7;
        *var_tt1_dn8_slot = var_tt1_dn8;
        *var_tt1_dn9_slot = var_tt1_dn9;
        *var_tt2_slot = var_tt2;
        *var_tt2_dn6_slot = var_tt2_dn6;
        *var_tt2_dn7_slot = var_tt2_dn7;
        *var_tt2_dn8_slot = var_tt2_dn8;
        *var_tt2_dn9_slot = var_tt2_dn9;
        *var_xhighf1_d_slot = var_xhighf1_d;
        *var_xhighf2_d_slot = var_xhighf2_d;
        *var_xhighf2_d_dn6_slot = var_xhighf2_d_dn6;
        *var_xhighf2_d_dn7_slot = var_xhighf2_d_dn7;
        *var_xhighf2_d_dn8_slot = var_xhighf2_d_dn8;
        *var_xhighf2_d_dn9_slot = var_xhighf2_d_dn9;
        *var_xhighr_d_slot = var_xhighr_d;
        *var_xhighr_d_dn6_slot = var_xhighr_d_dn6;
        *var_xhighr_d_dn7_slot = var_xhighr_d_dn7;
        *var_xhighr_d_dn8_slot = var_xhighr_d_dn8;
        *var_xhighr_d_dn9_slot = var_xhighr_d_dn9;
        *var_zflagbot_d_slot = var_zflagbot_d;
        *var_zflaggat_d_slot = var_zflaggat_d;
        *var_zflagsti_d_slot = var_zflagsti_d;
        *var_zfrac_slot = var_zfrac;
    }

    pub(super) fn stamp_transient_block_85(
        p: &Parameters,
        var_a2_i: f64,
        var_betn_i: f64,
        var_betnedge_i: f64,
        var_coxprime: f64,
        var_cs_i: f64,
        var_ct_i: f64,
        var_ctg_i: f64,
        var_delt: f64,
        var_delt_dn4: f64,
        var_delvtac_i: f64,
        var_delvto_i: f64,
        var_delvtoedge_i: f64,
        var_dphib_i: f64,
        var_dvsbnud_i: f64,
        var_eg: f64,
        var_eg_dn4: f64,
        var_epssi: f64,
        var_factuo_i: f64,
        var_factuoedge_i: f64,
        var_fnt_i: f64,
        var_inv_phit: f64,
        var_inv_phit_dn4: f64,
        var_ln_rtn: f64,
        var_ln_rtn_dn4: f64,
        var_mue_i: f64,
        var_neff_i: f64,
        var_neffac_i: f64,
        var_np_i: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_qq: f64,
        var_rs_i: f64,
        var_rtn: f64,
        var_rtn_dn4: f64,
        var_st2vfb_i: f64,
        var_sta2_i: f64,
        var_stbet_i: f64,
        var_stbetedge_i: f64,
        var_stcs_i: f64,
        var_stct_i: f64,
        var_stmue_i: f64,
        var_strs_i: f64,
        var_stthecs_i: f64,
        var_stthemu_i: f64,
        var_stthesat_i: f64,
        var_stvfb_i: f64,
        var_stvfbedge_i: f64,
        var_stxcor_i: f64,
        var_thecs_i: f64,
        var_themu_i: f64,
        var_thesat_i: f64,
        var_thesatac_i: f64,
        var_tkd: f64,
        var_tkd_dn4: f64,
        var_tox_sq: f64,
        var_vfb_i: f64,
        var_vfbedge_i: f64,
        var_vsbnud_i: f64,
        var_xcor_i: f64,
        var_a2_t_slot: &mut f64,
        var_a2_t_dn4_slot: &mut f64,
        var_alpha_b_slot: &mut f64,
        var_alpha_b_dn4_slot: &mut f64,
        var_aphi_ac_slot: &mut f64,
        var_aphi_ac_dn4_slot: &mut f64,
        var_aphi_dc_slot: &mut f64,
        var_aphi_dc_dn4_slot: &mut f64,
        var_arg2max_slot: &mut f64,
        var_bet_i_slot: &mut f64,
        var_bet_i_dn4_slot: &mut f64,
        var_betedge_i_slot: &mut f64,
        var_betedge_i_dn4_slot: &mut f64,
        var_betn_t_slot: &mut f64,
        var_betn_t_dn4_slot: &mut f64,
        var_betnedge_t_slot: &mut f64,
        var_betnedge_t_dn4_slot: &mut f64,
        var_bphi_ac_slot: &mut f64,
        var_bphi_ac_dn4_slot: &mut f64,
        var_bphi_dc_slot: &mut f64,
        var_bphi_dc_dn4_slot: &mut f64,
        var_cs_t_slot: &mut f64,
        var_cs_t_dn4_slot: &mut f64,
        var_ct_t_slot: &mut f64,
        var_ct_t_dn4_slot: &mut f64,
        var_ctg_t_slot: &mut f64,
        var_ctg_t_dn4_slot: &mut f64,
        var_dphibq_slot: &mut f64,
        var_dphibq_dn4_slot: &mut f64,
        var_g_0_ac_slot: &mut f64,
        var_g_0_ac_dn4_slot: &mut f64,
        var_g_0_dc_slot: &mut f64,
        var_g_0_dc_dn4_slot: &mut f64,
        var_guard1024_slot: &mut f64,
        var_guard1025_slot: &mut f64,
        var_guard1026_slot: &mut f64,
        var_guard1027_slot: &mut f64,
        var_kp_slot: &mut f64,
        var_kp_dn4_slot: &mut f64,
        var_mue_t_slot: &mut f64,
        var_mue_t_dn4_slot: &mut f64,
        var_np_slot: &mut f64,
        var_nt_slot: &mut f64,
        var_nt0_slot: &mut f64,
        var_nt0_dn4_slot: &mut f64,
        var_nt_dn4_slot: &mut f64,
        var_phib_ac_slot: &mut f64,
        var_phib_ac_dn4_slot: &mut f64,
        var_phib_dc_slot: &mut f64,
        var_phib_dc_dn4_slot: &mut f64,
        var_phibfac_slot: &mut f64,
        var_phibfac_dn4_slot: &mut f64,
        var_phix1_ac_slot: &mut f64,
        var_phix1_ac_dn4_slot: &mut f64,
        var_phix1_dc_slot: &mut f64,
        var_phix1_dc_dn4_slot: &mut f64,
        var_phix2_slot: &mut f64,
        var_phix2_dn4_slot: &mut f64,
        var_phix_ac_slot: &mut f64,
        var_phix_ac_dn4_slot: &mut f64,
        var_phix_dc_slot: &mut f64,
        var_phix_dc_dn4_slot: &mut f64,
        var_qb0_slot: &mut f64,
        var_qb0_dn4_slot: &mut f64,
        var_qlim2_slot: &mut f64,
        var_qlim2_dn4_slot: &mut f64,
        var_rs_t_slot: &mut f64,
        var_rs_t_dn4_slot: &mut f64,
        var_sqrt_phib_dc_slot: &mut f64,
        var_sqrt_phib_dc_dn4_slot: &mut f64,
        var_tf_bet_slot: &mut f64,
        var_tf_bet_dn4_slot: &mut f64,
        var_tf_betedge_slot: &mut f64,
        var_tf_betedge_dn4_slot: &mut f64,
        var_tf_cs_slot: &mut f64,
        var_tf_cs_dn4_slot: &mut f64,
        var_tf_ct_slot: &mut f64,
        var_tf_ct_dn4_slot: &mut f64,
        var_tf_mue_slot: &mut f64,
        var_tf_mue_dn4_slot: &mut f64,
        var_tf_ther_slot: &mut f64,
        var_tf_ther_dn4_slot: &mut f64,
        var_tf_thesat_slot: &mut f64,
        var_tf_thesat_dn4_slot: &mut f64,
        var_tf_xcor_slot: &mut f64,
        var_tf_xcor_dn4_slot: &mut f64,
        var_thecs_t_slot: &mut f64,
        var_thecs_t_dn4_slot: &mut f64,
        var_themu_t_slot: &mut f64,
        var_themu_t_dn4_slot: &mut f64,
        var_ther_i_slot: &mut f64,
        var_ther_i_dn4_slot: &mut f64,
        var_thesat_t_slot: &mut f64,
        var_thesat_t_dn4_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_us1_slot: &mut f64,
        var_us1_dn4_slot: &mut f64,
        var_us21_slot: &mut f64,
        var_us21_dn4_slot: &mut f64,
        var_vfb_t_slot: &mut f64,
        var_vfb_t_dn4_slot: &mut f64,
        var_vfbedge_t_slot: &mut f64,
        var_vfbedge_t_dn4_slot: &mut f64,
        var_xcor_t_slot: &mut f64,
        var_xcor_t_dn4_slot: &mut f64,
    ) {
        let mut var_a2_t: f64 = *var_a2_t_slot;
        let mut var_a2_t_dn4: f64 = *var_a2_t_dn4_slot;
        let mut var_alpha_b: f64 = *var_alpha_b_slot;
        let mut var_alpha_b_dn4: f64 = *var_alpha_b_dn4_slot;
        let mut var_aphi_ac: f64 = *var_aphi_ac_slot;
        let mut var_aphi_ac_dn4: f64 = *var_aphi_ac_dn4_slot;
        let mut var_aphi_dc: f64 = *var_aphi_dc_slot;
        let mut var_aphi_dc_dn4: f64 = *var_aphi_dc_dn4_slot;
        let mut var_arg2max: f64 = *var_arg2max_slot;
        let mut var_bet_i: f64 = *var_bet_i_slot;
        let mut var_bet_i_dn4: f64 = *var_bet_i_dn4_slot;
        let mut var_betedge_i: f64 = *var_betedge_i_slot;
        let mut var_betedge_i_dn4: f64 = *var_betedge_i_dn4_slot;
        let mut var_betn_t: f64 = *var_betn_t_slot;
        let mut var_betn_t_dn4: f64 = *var_betn_t_dn4_slot;
        let mut var_betnedge_t: f64 = *var_betnedge_t_slot;
        let mut var_betnedge_t_dn4: f64 = *var_betnedge_t_dn4_slot;
        let mut var_bphi_ac: f64 = *var_bphi_ac_slot;
        let mut var_bphi_ac_dn4: f64 = *var_bphi_ac_dn4_slot;
        let mut var_bphi_dc: f64 = *var_bphi_dc_slot;
        let mut var_bphi_dc_dn4: f64 = *var_bphi_dc_dn4_slot;
        let mut var_cs_t: f64 = *var_cs_t_slot;
        let mut var_cs_t_dn4: f64 = *var_cs_t_dn4_slot;
        let mut var_ct_t: f64 = *var_ct_t_slot;
        let mut var_ct_t_dn4: f64 = *var_ct_t_dn4_slot;
        let mut var_ctg_t: f64 = *var_ctg_t_slot;
        let mut var_ctg_t_dn4: f64 = *var_ctg_t_dn4_slot;
        let mut var_dphibq: f64 = *var_dphibq_slot;
        let mut var_dphibq_dn4: f64 = *var_dphibq_dn4_slot;
        let mut var_g_0_ac: f64 = *var_g_0_ac_slot;
        let mut var_g_0_ac_dn4: f64 = *var_g_0_ac_dn4_slot;
        let mut var_g_0_dc: f64 = *var_g_0_dc_slot;
        let mut var_g_0_dc_dn4: f64 = *var_g_0_dc_dn4_slot;
        let mut var_guard1024: f64 = *var_guard1024_slot;
        let mut var_guard1025: f64 = *var_guard1025_slot;
        let mut var_guard1026: f64 = *var_guard1026_slot;
        let mut var_guard1027: f64 = *var_guard1027_slot;
        let mut var_kp: f64 = *var_kp_slot;
        let mut var_kp_dn4: f64 = *var_kp_dn4_slot;
        let mut var_mue_t: f64 = *var_mue_t_slot;
        let mut var_mue_t_dn4: f64 = *var_mue_t_dn4_slot;
        let mut var_np: f64 = *var_np_slot;
        let mut var_nt: f64 = *var_nt_slot;
        let mut var_nt0: f64 = *var_nt0_slot;
        let mut var_nt0_dn4: f64 = *var_nt0_dn4_slot;
        let mut var_nt_dn4: f64 = *var_nt_dn4_slot;
        let mut var_phib_ac: f64 = *var_phib_ac_slot;
        let mut var_phib_ac_dn4: f64 = *var_phib_ac_dn4_slot;
        let mut var_phib_dc: f64 = *var_phib_dc_slot;
        let mut var_phib_dc_dn4: f64 = *var_phib_dc_dn4_slot;
        let mut var_phibfac: f64 = *var_phibfac_slot;
        let mut var_phibfac_dn4: f64 = *var_phibfac_dn4_slot;
        let mut var_phix1_ac: f64 = *var_phix1_ac_slot;
        let mut var_phix1_ac_dn4: f64 = *var_phix1_ac_dn4_slot;
        let mut var_phix1_dc: f64 = *var_phix1_dc_slot;
        let mut var_phix1_dc_dn4: f64 = *var_phix1_dc_dn4_slot;
        let mut var_phix2: f64 = *var_phix2_slot;
        let mut var_phix2_dn4: f64 = *var_phix2_dn4_slot;
        let mut var_phix_ac: f64 = *var_phix_ac_slot;
        let mut var_phix_ac_dn4: f64 = *var_phix_ac_dn4_slot;
        let mut var_phix_dc: f64 = *var_phix_dc_slot;
        let mut var_phix_dc_dn4: f64 = *var_phix_dc_dn4_slot;
        let mut var_qb0: f64 = *var_qb0_slot;
        let mut var_qb0_dn4: f64 = *var_qb0_dn4_slot;
        let mut var_qlim2: f64 = *var_qlim2_slot;
        let mut var_qlim2_dn4: f64 = *var_qlim2_dn4_slot;
        let mut var_rs_t: f64 = *var_rs_t_slot;
        let mut var_rs_t_dn4: f64 = *var_rs_t_dn4_slot;
        let mut var_sqrt_phib_dc: f64 = *var_sqrt_phib_dc_slot;
        let mut var_sqrt_phib_dc_dn4: f64 = *var_sqrt_phib_dc_dn4_slot;
        let mut var_tf_bet: f64 = *var_tf_bet_slot;
        let mut var_tf_bet_dn4: f64 = *var_tf_bet_dn4_slot;
        let mut var_tf_betedge: f64 = *var_tf_betedge_slot;
        let mut var_tf_betedge_dn4: f64 = *var_tf_betedge_dn4_slot;
        let mut var_tf_cs: f64 = *var_tf_cs_slot;
        let mut var_tf_cs_dn4: f64 = *var_tf_cs_dn4_slot;
        let mut var_tf_ct: f64 = *var_tf_ct_slot;
        let mut var_tf_ct_dn4: f64 = *var_tf_ct_dn4_slot;
        let mut var_tf_mue: f64 = *var_tf_mue_slot;
        let mut var_tf_mue_dn4: f64 = *var_tf_mue_dn4_slot;
        let mut var_tf_ther: f64 = *var_tf_ther_slot;
        let mut var_tf_ther_dn4: f64 = *var_tf_ther_dn4_slot;
        let mut var_tf_thesat: f64 = *var_tf_thesat_slot;
        let mut var_tf_thesat_dn4: f64 = *var_tf_thesat_dn4_slot;
        let mut var_tf_xcor: f64 = *var_tf_xcor_slot;
        let mut var_tf_xcor_dn4: f64 = *var_tf_xcor_dn4_slot;
        let mut var_thecs_t: f64 = *var_thecs_t_slot;
        let mut var_thecs_t_dn4: f64 = *var_thecs_t_dn4_slot;
        let mut var_themu_t: f64 = *var_themu_t_slot;
        let mut var_themu_t_dn4: f64 = *var_themu_t_dn4_slot;
        let mut var_ther_i: f64 = *var_ther_i_slot;
        let mut var_ther_i_dn4: f64 = *var_ther_i_dn4_slot;
        let mut var_thesat_t: f64 = *var_thesat_t_slot;
        let mut var_thesat_t_dn4: f64 = *var_thesat_t_dn4_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_us1: f64 = *var_us1_slot;
        let mut var_us1_dn4: f64 = *var_us1_dn4_slot;
        let mut var_us21: f64 = *var_us21_slot;
        let mut var_us21_dn4: f64 = *var_us21_dn4_slot;
        let mut var_vfb_t: f64 = *var_vfb_t_slot;
        let mut var_vfb_t_dn4: f64 = *var_vfb_t_dn4_slot;
        let mut var_vfbedge_t: f64 = *var_vfbedge_t_slot;
        let mut var_vfbedge_t_dn4: f64 = *var_vfbedge_t_dn4_slot;
        let mut var_xcor_t: f64 = *var_xcor_t_slot;
        let mut var_xcor_t_dn4: f64 = *var_xcor_t_dn4_slot;

        let (assign39520_e53006, assign39520_e53006_d_n4,) = {
    if (var_phibfac > 0.001) {
        (var_phibfac, var_phibfac_dn4,)
    } else {
        (0.001, 0.0,)
    }
};
        var_phibfac = assign39520_e53006;
        var_phibfac_dn4 = assign39520_e53006_d_n4;

        let assign39530_e53009: f64 = (4.0 * 1.3806505e-23);
        let assign39530_e53011: f64 = (assign39530_e53009 * var_tkd);
        var_nt0 = assign39530_e53011;
        var_nt0_dn4 = (assign39530_e53009 * var_tkd_dn4);

        let assign39540_e53014: f64 = (var_eg + var_dphib_i);
        let assign39540_e53017: f64 = (2.0 * var_phit);
        let assign39540_e53021: f64 = (-0.75);
        let assign39540_e53022: f64 = (var_phibfac).powf(assign39540_e53021);
        let assign39540_e53023: f64 = (var_neff_i * assign39540_e53022);
        let assign39540_e53025: f64 = (assign39540_e53023 * 4e-26);
        let assign39540_e53026: f64 = (assign39540_e53025).ln();
        let assign39540_e53027: f64 = (assign39540_e53017 * assign39540_e53026);
        let assign39540_e53028: f64 = (assign39540_e53014 + assign39540_e53027);
        var_phib_dc = assign39540_e53028;
        var_phib_dc_dn4 = (var_eg_dn4 + (((2.0 * var_phit_dn4) * assign39540_e53026) + (assign39540_e53017 * (((var_neff_i * if 0.0 == 0.0 && ((assign39540_e53021) as f64).is_finite() && ((assign39540_e53021) as f64).fract() == 0.0 { if assign39540_e53021 == 0.0 { 0.0 } else { (assign39540_e53021 * ((var_phibfac).powf(assign39540_e53021 - 1.0) * var_phibfac_dn4)) } } else { (assign39540_e53022 * (assign39540_e53021 * (var_phibfac_dn4 / var_phibfac))) }) * 4e-26) / assign39540_e53025))));

        let (assign39550_e53034, assign39550_e53034_d_n4,) = {
    if (var_phib_dc > 0.05) {
        (var_phib_dc, var_phib_dc_dn4,)
    } else {
        (0.05, 0.0,)
    }
};
        var_phib_dc = assign39550_e53034;
        var_phib_dc_dn4 = assign39550_e53034_d_n4;

        let assign39560_e53037: f64 = (2.0 * 1.6021918e-19);
        let assign39560_e53039: f64 = (assign39560_e53037 * var_neff_i);
        let assign39560_e53041: f64 = (assign39560_e53039 * var_epssi);
        let assign39560_e53043: f64 = (assign39560_e53041 * var_inv_phit);
        let assign39560_e53044: f64 = (assign39560_e53043).sqrt();
        let assign39560_e53046: f64 = (assign39560_e53044 / var_coxprime);
        var_g_0_dc = assign39560_e53046;
        var_g_0_dc_dn4 = (((assign39560_e53041 * var_inv_phit_dn4) / (2.0 * assign39560_e53044)) / var_coxprime);

        var_kp = 0.0;
        var_kp_dn4 = 0.0;

        var_np = 0.0;

        let assign39590_e53051: f64 = if var_np_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1024 = assign39590_e53051;

        let (assign39600_e53057,) = {
    if (var_guard1024 != 0.0) {
        let assign39600_e53055: f64 = (80000000.0 / var_tox_sq);
        (assign39600_e53055,)
    } else {
        (var_arg2max,)
    }
};
        var_arg2max = assign39600_e53057;

        let (assign39610_e53066,) = {
    if (var_guard1024 != 0.0) {
        let (assign39610_e53064,) = {
            if (var_np_i > var_arg2max) {
                (var_np_i,)
            } else {
                (var_arg2max,)
            }
        };
        (assign39610_e53064,)
    } else {
        (var_np,)
    }
};
        var_np = assign39610_e53066;

        let (assign39620_e53075,) = {
    if (var_guard1024 != 0.0) {
        let (assign39620_e53073,) = {
            if (5e24 > var_np) {
                (5e24,)
            } else {
                (var_np,)
            }
        };
        (assign39620_e53073,)
    } else {
        (var_np,)
    }
};
        var_np = assign39620_e53075;

        let (assign39630_e53091, assign39630_e53091_d_n4,) = {
    if (var_guard1024 != 0.0) {
        let assign39630_e53079: f64 = (2.0 * var_coxprime);
        let assign39630_e53081: f64 = (assign39630_e53079 * var_coxprime);
        let assign39630_e53083: f64 = (assign39630_e53081 * var_phit);
        let assign39630_e53086: f64 = (1.6021918e-19 * var_np);
        let assign39630_e53088: f64 = (assign39630_e53086 * var_epssi);
        let assign39630_e53089: f64 = (assign39630_e53083 / assign39630_e53088);
        (assign39630_e53089, ((assign39630_e53081 * var_phit_dn4) / assign39630_e53088),)
    } else {
        (var_kp, var_kp_dn4,)
    }
};
        var_kp = assign39630_e53091;
        var_kp_dn4 = assign39630_e53091_d_n4;

        let assign39640_e53094: f64 = (100.0 * var_phit);
        let assign39640_e53096: f64 = (assign39640_e53094 * var_phit);
        var_qlim2 = assign39640_e53096;
        var_qlim2_dn4 = (((100.0 * var_phit_dn4) * var_phit) + (assign39640_e53094 * var_phit_dn4));

        let assign39650_e53099: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        var_guard1025 = assign39650_e53099;

        let (assign39660_e53110, assign39660_e53110_d_n4,) = {
    if (var_guard1025 != 0.0) {
        let assign39660_e53103: f64 = (var_phit * var_g_0_dc);
        let assign39660_e53105: f64 = (assign39660_e53103 * var_g_0_dc);
        let assign39660_e53107: f64 = (assign39660_e53105 * var_phib_dc);
        let assign39660_e53108: f64 = (assign39660_e53107).sqrt();
        (assign39660_e53108, (((((((var_phit_dn4 * var_g_0_dc) + (var_phit * var_g_0_dc_dn4)) * var_g_0_dc) + (assign39660_e53103 * var_g_0_dc_dn4)) * var_phib_dc) + (assign39660_e53105 * var_phib_dc_dn4)) / (2.0 * assign39660_e53108)),)
    } else {
        (var_qb0, var_qb0_dn4,)
    }
};
        var_qb0 = assign39660_e53110;
        var_qb0_dn4 = assign39660_e53110_d_n4;

        let (assign39670_e53120, assign39670_e53120_d_n4,) = {
    if (var_guard1025 != 0.0) {
        let assign39670_e53114: f64 = (0.75 * var_qq);
        let assign39670_e53117: f64 = (var_qb0).powf(0.6666666666666666);
        let assign39670_e53118: f64 = (assign39670_e53114 * assign39670_e53117);
        (assign39670_e53118, (assign39670_e53114 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_qb0).powf(0.6666666666666666 - 1.0) * var_qb0_dn4)) } } else { (assign39670_e53117 * (0.6666666666666666 * (var_qb0_dn4 / var_qb0))) }),)
    } else {
        (var_dphibq, var_dphibq_dn4,)
    }
};
        var_dphibq = assign39670_e53120;
        var_dphibq_dn4 = assign39670_e53120_d_n4;

        let (assign39680_e53126, assign39680_e53126_d_n4,) = {
    if (var_guard1025 != 0.0) {
        let assign39680_e53124: f64 = (var_phib_dc + var_dphibq);
        (assign39680_e53124, (var_phib_dc_dn4 + var_dphibq_dn4),)
    } else {
        (var_phib_dc, var_phib_dc_dn4,)
    }
};
        var_phib_dc = assign39680_e53126;
        var_phib_dc_dn4 = assign39680_e53126_d_n4;

        let (assign39690_e53140, assign39690_e53140_d_n4,) = {
    if (var_guard1025 != 0.0) {
        let assign39690_e53132: f64 = (2.0 * 0.6666666666666666);
        let assign39690_e53134: f64 = (assign39690_e53132 * var_dphibq);
        let assign39690_e53136: f64 = (assign39690_e53134 / var_qb0);
        let assign39690_e53137: f64 = (1.0 + assign39690_e53136);
        let assign39690_e53138: f64 = (var_g_0_dc * assign39690_e53137);
        (assign39690_e53138, ((var_g_0_dc_dn4 * assign39690_e53137) + (var_g_0_dc * ((((assign39690_e53132 * var_dphibq_dn4) * var_qb0) - (assign39690_e53134 * var_qb0_dn4)) / (var_qb0 * var_qb0)))),)
    } else {
        (var_g_0_dc, var_g_0_dc_dn4,)
    }
};
        var_g_0_dc = assign39690_e53140;
        var_g_0_dc_dn4 = assign39690_e53140_d_n4;

        let assign39700_e53142: f64 = (var_phib_dc).sqrt();
        var_sqrt_phib_dc = assign39700_e53142;
        var_sqrt_phib_dc_dn4 = (var_phib_dc_dn4 / (2.0 * assign39700_e53142));

        let assign39710_e53145: f64 = (0.95 * var_phib_dc);
        var_phix_dc = assign39710_e53145;
        var_phix_dc_dn4 = (0.95 * var_phib_dc_dn4);

        let assign39720_e53148: f64 = (0.0025 * var_phib_dc);
        let assign39720_e53150: f64 = (assign39720_e53148 * var_phib_dc);
        var_aphi_dc = assign39720_e53150;
        var_aphi_dc_dn4 = (((0.0025 * var_phib_dc_dn4) * var_phib_dc) + (assign39720_e53148 * var_phib_dc_dn4));

        var_bphi_dc = var_aphi_dc;
        var_bphi_dc_dn4 = var_aphi_dc_dn4;

        let assign39740_e53154: f64 = (var_bphi_dc).sqrt();
        let assign39740_e53155: f64 = (0.5 * assign39740_e53154);
        var_phix2 = assign39740_e53155;
        var_phix2_dn4 = (0.5 * (var_bphi_dc_dn4 / (2.0 * assign39740_e53154)));

        let assign39750_e53159: f64 = (var_phix_dc - var_phix2);
        let assign39750_e53161: f64 = assign39750_e53159;
        let assign39750_e53164: f64 = (var_phix_dc - var_phix2);
        let assign39750_e53166: f64 = assign39750_e53164;
        let assign39750_e53169: f64 = (var_phix_dc - var_phix2);
        let assign39750_e53171: f64 = assign39750_e53169;
        let assign39750_e53172: f64 = (assign39750_e53166 * assign39750_e53171);
        let assign39750_e53174: f64 = (assign39750_e53172 + var_aphi_dc);
        let assign39750_e53175: f64 = (assign39750_e53174).sqrt();
        let assign39750_e53176: f64 = (assign39750_e53161 - assign39750_e53175);
        let assign39750_e53177: f64 = (0.5 * assign39750_e53176);
        var_phix1_dc = assign39750_e53177;
        var_phix1_dc_dn4 = (0.5 * ((var_phix_dc_dn4 - var_phix2_dn4) - (((((var_phix_dc_dn4 - var_phix2_dn4) * assign39750_e53171) + (assign39750_e53166 * (var_phix_dc_dn4 - var_phix2_dn4))) + var_aphi_dc_dn4) / (2.0 * assign39750_e53175))));

        let assign39760_e53181: f64 = (var_phib_dc + var_eg);
        let assign39760_e53182: f64 = (0.5 * assign39760_e53181);
        var_alpha_b = assign39760_e53182;
        var_alpha_b_dn4 = (0.5 * (var_phib_dc_dn4 + var_eg_dn4));

        let assign39770_e53185: f64 = (var_vsbnud_i + var_phib_dc);
        let assign39770_e53186: f64 = (assign39770_e53185).sqrt();
        let assign39770_e53188: f64 = (assign39770_e53186 - var_sqrt_phib_dc);
        var_us1 = assign39770_e53188;
        var_us1_dn4 = ((var_phib_dc_dn4 / (2.0 * assign39770_e53186)) - var_sqrt_phib_dc_dn4);

        let assign39780_e53191: f64 = (var_vsbnud_i + var_dvsbnud_i);
        let assign39780_e53193: f64 = (assign39780_e53191 + var_phib_dc);
        let assign39780_e53194: f64 = (assign39780_e53193).sqrt();
        let assign39780_e53196: f64 = (assign39780_e53194 - var_sqrt_phib_dc);
        let assign39780_e53198: f64 = (assign39780_e53196 - var_us1);
        var_us21 = assign39780_e53198;
        var_us21_dn4 = (((var_phib_dc_dn4 / (2.0 * assign39780_e53194)) - var_sqrt_phib_dc_dn4) - var_us1_dn4);

        let assign39790_e53201: f64 = (var_eg + var_dphib_i);
        let assign39790_e53203: f64 = (assign39790_e53201 + var_delvtac_i);
        let assign39790_e53206: f64 = (2.0 * var_phit);
        let assign39790_e53210: f64 = (-0.75);
        let assign39790_e53211: f64 = (var_phibfac).powf(assign39790_e53210);
        let assign39790_e53212: f64 = (var_neffac_i * assign39790_e53211);
        let assign39790_e53214: f64 = (assign39790_e53212 * 4e-26);
        let assign39790_e53215: f64 = (assign39790_e53214).ln();
        let assign39790_e53216: f64 = (assign39790_e53206 * assign39790_e53215);
        let assign39790_e53217: f64 = (assign39790_e53203 + assign39790_e53216);
        var_phib_ac = assign39790_e53217;
        var_phib_ac_dn4 = (var_eg_dn4 + (((2.0 * var_phit_dn4) * assign39790_e53215) + (assign39790_e53206 * (((var_neffac_i * if 0.0 == 0.0 && ((assign39790_e53210) as f64).is_finite() && ((assign39790_e53210) as f64).fract() == 0.0 { if assign39790_e53210 == 0.0 { 0.0 } else { (assign39790_e53210 * ((var_phibfac).powf(assign39790_e53210 - 1.0) * var_phibfac_dn4)) } } else { (assign39790_e53211 * (assign39790_e53210 * (var_phibfac_dn4 / var_phibfac))) }) * 4e-26) / assign39790_e53214))));

        let (assign39800_e53223, assign39800_e53223_d_n4,) = {
    if (var_phib_ac > 0.05) {
        (var_phib_ac, var_phib_ac_dn4,)
    } else {
        (0.05, 0.0,)
    }
};
        var_phib_ac = assign39800_e53223;
        var_phib_ac_dn4 = assign39800_e53223_d_n4;

        let assign39810_e53226: f64 = (2.0 * 1.6021918e-19);
        let assign39810_e53228: f64 = (assign39810_e53226 * var_neffac_i);
        let assign39810_e53230: f64 = (assign39810_e53228 * var_epssi);
        let assign39810_e53232: f64 = (assign39810_e53230 * var_inv_phit);
        let assign39810_e53233: f64 = (assign39810_e53232).sqrt();
        let assign39810_e53235: f64 = (assign39810_e53233 / var_coxprime);
        var_g_0_ac = assign39810_e53235;
        var_g_0_ac_dn4 = (((assign39810_e53230 * var_inv_phit_dn4) / (2.0 * assign39810_e53233)) / var_coxprime);

        let assign39820_e53238: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        var_guard1026 = assign39820_e53238;

        let (assign39830_e53249, assign39830_e53249_d_n4,) = {
    if (var_guard1026 != 0.0) {
        let assign39830_e53242: f64 = (var_phit * var_g_0_ac);
        let assign39830_e53244: f64 = (assign39830_e53242 * var_g_0_ac);
        let assign39830_e53246: f64 = (assign39830_e53244 * var_phib_ac);
        let assign39830_e53247: f64 = (assign39830_e53246).sqrt();
        (assign39830_e53247, (((((((var_phit_dn4 * var_g_0_ac) + (var_phit * var_g_0_ac_dn4)) * var_g_0_ac) + (assign39830_e53242 * var_g_0_ac_dn4)) * var_phib_ac) + (assign39830_e53244 * var_phib_ac_dn4)) / (2.0 * assign39830_e53247)),)
    } else {
        (var_qb0, var_qb0_dn4,)
    }
};
        var_qb0 = assign39830_e53249;
        var_qb0_dn4 = assign39830_e53249_d_n4;

        let (assign39840_e53259, assign39840_e53259_d_n4,) = {
    if (var_guard1026 != 0.0) {
        let assign39840_e53253: f64 = (0.75 * var_qq);
        let assign39840_e53256: f64 = (var_qb0).powf(0.6666666666666666);
        let assign39840_e53257: f64 = (assign39840_e53253 * assign39840_e53256);
        (assign39840_e53257, (assign39840_e53253 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_qb0).powf(0.6666666666666666 - 1.0) * var_qb0_dn4)) } } else { (assign39840_e53256 * (0.6666666666666666 * (var_qb0_dn4 / var_qb0))) }),)
    } else {
        (var_dphibq, var_dphibq_dn4,)
    }
};
        var_dphibq = assign39840_e53259;
        var_dphibq_dn4 = assign39840_e53259_d_n4;

        let (assign39850_e53265, assign39850_e53265_d_n4,) = {
    if (var_guard1026 != 0.0) {
        let assign39850_e53263: f64 = (var_phib_ac + var_dphibq);
        (assign39850_e53263, (var_phib_ac_dn4 + var_dphibq_dn4),)
    } else {
        (var_phib_ac, var_phib_ac_dn4,)
    }
};
        var_phib_ac = assign39850_e53265;
        var_phib_ac_dn4 = assign39850_e53265_d_n4;

        let (assign39860_e53279, assign39860_e53279_d_n4,) = {
    if (var_guard1026 != 0.0) {
        let assign39860_e53271: f64 = (2.0 * 0.6666666666666666);
        let assign39860_e53273: f64 = (assign39860_e53271 * var_dphibq);
        let assign39860_e53275: f64 = (assign39860_e53273 / var_qb0);
        let assign39860_e53276: f64 = (1.0 + assign39860_e53275);
        let assign39860_e53277: f64 = (var_g_0_ac * assign39860_e53276);
        (assign39860_e53277, ((var_g_0_ac_dn4 * assign39860_e53276) + (var_g_0_ac * ((((assign39860_e53271 * var_dphibq_dn4) * var_qb0) - (assign39860_e53273 * var_qb0_dn4)) / (var_qb0 * var_qb0)))),)
    } else {
        (var_g_0_ac, var_g_0_ac_dn4,)
    }
};
        var_g_0_ac = assign39860_e53279;
        var_g_0_ac_dn4 = assign39860_e53279_d_n4;

        let assign39870_e53282: f64 = (0.95 * var_phib_ac);
        var_phix_ac = assign39870_e53282;
        var_phix_ac_dn4 = (0.95 * var_phib_ac_dn4);

        let assign39880_e53285: f64 = (0.0025 * var_phib_ac);
        let assign39880_e53287: f64 = (assign39880_e53285 * var_phib_ac);
        var_aphi_ac = assign39880_e53287;
        var_aphi_ac_dn4 = (((0.0025 * var_phib_ac_dn4) * var_phib_ac) + (assign39880_e53285 * var_phib_ac_dn4));

        var_bphi_ac = var_aphi_ac;
        var_bphi_ac_dn4 = var_aphi_ac_dn4;

        let assign39900_e53291: f64 = (var_bphi_ac).sqrt();
        let assign39900_e53292: f64 = (0.5 * assign39900_e53291);
        var_phix2 = assign39900_e53292;
        var_phix2_dn4 = (0.5 * (var_bphi_ac_dn4 / (2.0 * assign39900_e53291)));

        let assign39910_e53296: f64 = (var_phix_ac - var_phix2);
        let assign39910_e53298: f64 = assign39910_e53296;
        let assign39910_e53301: f64 = (var_phix_ac - var_phix2);
        let assign39910_e53303: f64 = assign39910_e53301;
        let assign39910_e53306: f64 = (var_phix_ac - var_phix2);
        let assign39910_e53308: f64 = assign39910_e53306;
        let assign39910_e53309: f64 = (assign39910_e53303 * assign39910_e53308);
        let assign39910_e53311: f64 = (assign39910_e53309 + var_aphi_ac);
        let assign39910_e53312: f64 = (assign39910_e53311).sqrt();
        let assign39910_e53313: f64 = (assign39910_e53298 - assign39910_e53312);
        let assign39910_e53314: f64 = (0.5 * assign39910_e53313);
        var_phix1_ac = assign39910_e53314;
        var_phix1_ac_dn4 = (0.5 * ((var_phix_ac_dn4 - var_phix2_dn4) - (((((var_phix_ac_dn4 - var_phix2_dn4) * assign39910_e53308) + (assign39910_e53303 * (var_phix_ac_dn4 - var_phix2_dn4))) + var_aphi_ac_dn4) / (2.0 * assign39910_e53312))));

        let assign39920_e53318: f64 = (var_stvfb_i * var_delt);
        let assign39920_e53322: f64 = (var_st2vfb_i * var_delt);
        let assign39920_e53323: f64 = (1.0 + assign39920_e53322);
        let assign39920_e53324: f64 = (assign39920_e53318 * assign39920_e53323);
        let assign39920_e53325: f64 = (var_vfb_i + assign39920_e53324);
        let assign39920_e53327: f64 = (assign39920_e53325 + var_delvto_i);
        var_vfb_t = assign39920_e53327;
        var_vfb_t_dn4 = (((var_stvfb_i * var_delt_dn4) * assign39920_e53323) + (assign39920_e53318 * (var_st2vfb_i * var_delt_dn4)));

        let assign39930_e53330: f64 = (var_stct_i * var_ln_rtn);
        let assign39930_e53331: f64 = (assign39930_e53330).exp();
        var_tf_ct = assign39930_e53331;
        var_tf_ct_dn4 = (assign39930_e53331 * (var_stct_i * var_ln_rtn_dn4));

        let assign39940_e53334: f64 = (var_ct_i * var_tf_ct);
        var_ct_t = assign39940_e53334;
        var_ct_t_dn4 = (var_ct_i * var_tf_ct_dn4);

        let assign39950_e53337: f64 = (var_ctg_i / var_rtn);
        var_ctg_t = assign39950_e53337;
        var_ctg_t_dn4 = (-((var_ctg_i * var_rtn_dn4) / (var_rtn * var_rtn)));

        let assign39960_e53340: f64 = (var_stbet_i * var_ln_rtn);
        let assign39960_e53341: f64 = (assign39960_e53340).exp();
        var_tf_bet = assign39960_e53341;
        var_tf_bet_dn4 = (assign39960_e53341 * (var_stbet_i * var_ln_rtn_dn4));

        let assign39970_e53344: f64 = (var_betn_i * var_tf_bet);
        var_betn_t = assign39970_e53344;
        var_betn_t_dn4 = (var_betn_i * var_tf_bet_dn4);

        let assign39980_e53347: f64 = (var_factuo_i * var_betn_t);
        let assign39980_e53349: f64 = (assign39980_e53347 * var_coxprime);
        var_bet_i = assign39980_e53349;
        var_bet_i_dn4 = ((var_factuo_i * var_betn_t_dn4) * var_coxprime);

        let assign39990_e53353: f64 = (var_stthemu_i * var_ln_rtn);
        let assign39990_e53354: f64 = (assign39990_e53353).exp();
        let assign39990_e53355: f64 = (var_themu_i * assign39990_e53354);
        var_themu_t = assign39990_e53355;
        var_themu_t_dn4 = (var_themu_i * (assign39990_e53354 * (var_stthemu_i * var_ln_rtn_dn4)));

        let assign40000_e53358: f64 = (var_stmue_i * var_ln_rtn);
        let assign40000_e53359: f64 = (assign40000_e53358).exp();
        var_tf_mue = assign40000_e53359;
        var_tf_mue_dn4 = (assign40000_e53359 * (var_stmue_i * var_ln_rtn_dn4));

        let assign40010_e53362: f64 = (var_mue_i * var_tf_mue);
        var_mue_t = assign40010_e53362;
        var_mue_t_dn4 = (var_mue_i * var_tf_mue_dn4);

        let assign40020_e53366: f64 = (var_stthecs_i * var_ln_rtn);
        let assign40020_e53367: f64 = (assign40020_e53366).exp();
        let assign40020_e53368: f64 = (var_thecs_i * assign40020_e53367);
        var_thecs_t = assign40020_e53368;
        var_thecs_t_dn4 = (var_thecs_i * (assign40020_e53367 * (var_stthecs_i * var_ln_rtn_dn4)));

        let assign40030_e53371: f64 = (var_stcs_i * var_ln_rtn);
        let assign40030_e53372: f64 = (assign40030_e53371).exp();
        var_tf_cs = assign40030_e53372;
        var_tf_cs_dn4 = (assign40030_e53372 * (var_stcs_i * var_ln_rtn_dn4));

        let assign40040_e53375: f64 = (var_cs_i * var_tf_cs);
        var_cs_t = assign40040_e53375;
        var_cs_t_dn4 = (var_cs_i * var_tf_cs_dn4);

        let assign40050_e53378: f64 = (var_stxcor_i * var_ln_rtn);
        let assign40050_e53379: f64 = (assign40050_e53378).exp();
        var_tf_xcor = assign40050_e53379;
        var_tf_xcor_dn4 = (assign40050_e53379 * (var_stxcor_i * var_ln_rtn_dn4));

        let assign40060_e53382: f64 = (var_xcor_i * var_tf_xcor);
        var_xcor_t = assign40060_e53382;
        var_xcor_t_dn4 = (var_xcor_i * var_tf_xcor_dn4);

        let assign40070_e53385: f64 = (var_strs_i * var_ln_rtn);
        let assign40070_e53386: f64 = (assign40070_e53385).exp();
        var_tf_ther = assign40070_e53386;
        var_tf_ther_dn4 = (assign40070_e53386 * (var_strs_i * var_ln_rtn_dn4));

        let assign40080_e53389: f64 = (var_rs_i * var_tf_ther);
        var_rs_t = assign40080_e53389;
        var_rs_t_dn4 = (var_rs_i * var_tf_ther_dn4);

        let assign40090_e53392: f64 = (2.0 * var_bet_i);
        let assign40090_e53394: f64 = (assign40090_e53392 * var_rs_t);
        var_ther_i = assign40090_e53394;
        var_ther_i_dn4 = (((2.0 * var_bet_i_dn4) * var_rs_t) + (assign40090_e53392 * var_rs_t_dn4));

        let assign40100_e53397: f64 = (var_stthesat_i * var_ln_rtn);
        let assign40100_e53398: f64 = (assign40100_e53397).exp();
        var_tf_thesat = assign40100_e53398;
        var_tf_thesat_dn4 = (assign40100_e53398 * (var_stthesat_i * var_ln_rtn_dn4));

        let assign40110_e53401: f64 = (var_thesat_i * var_tf_thesat);
        var_thesat_t = assign40110_e53401;
        var_thesat_t_dn4 = (var_thesat_i * var_tf_thesat_dn4);

        let assign40120_e53404: f64 = (var_thesatac_i * var_tf_thesat);
        var_thesatac_t = assign40120_e53404;
        var_thesatac_t_dn4 = (var_thesatac_i * var_tf_thesat_dn4);

        let assign40130_e53407: f64 = (-var_sta2_i);
        let assign40130_e53409: f64 = (assign40130_e53407 * var_ln_rtn);
        let assign40130_e53410: f64 = (assign40130_e53409).exp();
        let assign40130_e53411: f64 = (var_a2_i * assign40130_e53410);
        var_a2_t = assign40130_e53411;
        var_a2_t_dn4 = (var_a2_i * (assign40130_e53410 * (assign40130_e53407 * var_ln_rtn_dn4)));

        let assign40140_e53414: f64 = (var_fnt_i * 4.0);
        let assign40140_e53416: f64 = (assign40140_e53414 * 1.3806505e-23);
        let assign40140_e53418: f64 = (assign40140_e53416 * var_tkd);
        var_nt = assign40140_e53418;
        var_nt_dn4 = (assign40140_e53416 * var_tkd_dn4);

        let assign40160_e53432: f64 = if ((p.p46 != 0.0) && (var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard1027 = assign40160_e53432;

        let (assign40170_e53442, assign40170_e53442_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40170_e53437: f64 = (var_stvfbedge_i * var_delt);
        let assign40170_e53438: f64 = (var_vfbedge_i + assign40170_e53437);
        let assign40170_e53440: f64 = (assign40170_e53438 + var_delvtoedge_i);
        (assign40170_e53440, (var_stvfbedge_i * var_delt_dn4),)
    } else {
        (var_vfbedge_t, var_vfbedge_t_dn4,)
    }
};
        var_vfbedge_t = assign40170_e53442;
        var_vfbedge_t_dn4 = assign40170_e53442_d_n4;

        let (assign40180_e53449, assign40180_e53449_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40180_e53446: f64 = (var_stbetedge_i * var_ln_rtn);
        let assign40180_e53447: f64 = (assign40180_e53446).exp();
        (assign40180_e53447, (assign40180_e53447 * (var_stbetedge_i * var_ln_rtn_dn4)),)
    } else {
        (var_tf_betedge, var_tf_betedge_dn4,)
    }
};
        var_tf_betedge = assign40180_e53449;
        var_tf_betedge_dn4 = assign40180_e53449_d_n4;

        let (assign40190_e53455, assign40190_e53455_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40190_e53453: f64 = (var_betnedge_i * var_tf_betedge);
        (assign40190_e53453, (var_betnedge_i * var_tf_betedge_dn4),)
    } else {
        (var_betnedge_t, var_betnedge_t_dn4,)
    }
};
        var_betnedge_t = assign40190_e53455;
        var_betnedge_t_dn4 = assign40190_e53455_d_n4;

        let (assign40200_e53463, assign40200_e53463_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40200_e53459: f64 = (var_factuoedge_i * var_betnedge_t);
        let assign40200_e53461: f64 = (assign40200_e53459 * var_coxprime);
        (assign40200_e53461, ((var_factuoedge_i * var_betnedge_t_dn4) * var_coxprime),)
    } else {
        (var_betedge_i, var_betedge_i_dn4,)
    }
};
        var_betedge_i = assign40200_e53463;
        var_betedge_i_dn4 = assign40200_e53463_d_n4;

        *var_a2_t_slot = var_a2_t;
        *var_a2_t_dn4_slot = var_a2_t_dn4;
        *var_alpha_b_slot = var_alpha_b;
        *var_alpha_b_dn4_slot = var_alpha_b_dn4;
        *var_aphi_ac_slot = var_aphi_ac;
        *var_aphi_ac_dn4_slot = var_aphi_ac_dn4;
        *var_aphi_dc_slot = var_aphi_dc;
        *var_aphi_dc_dn4_slot = var_aphi_dc_dn4;
        *var_arg2max_slot = var_arg2max;
        *var_bet_i_slot = var_bet_i;
        *var_bet_i_dn4_slot = var_bet_i_dn4;
        *var_betedge_i_slot = var_betedge_i;
        *var_betedge_i_dn4_slot = var_betedge_i_dn4;
        *var_betn_t_slot = var_betn_t;
        *var_betn_t_dn4_slot = var_betn_t_dn4;
        *var_betnedge_t_slot = var_betnedge_t;
        *var_betnedge_t_dn4_slot = var_betnedge_t_dn4;
        *var_bphi_ac_slot = var_bphi_ac;
        *var_bphi_ac_dn4_slot = var_bphi_ac_dn4;
        *var_bphi_dc_slot = var_bphi_dc;
        *var_bphi_dc_dn4_slot = var_bphi_dc_dn4;
        *var_cs_t_slot = var_cs_t;
        *var_cs_t_dn4_slot = var_cs_t_dn4;
        *var_ct_t_slot = var_ct_t;
        *var_ct_t_dn4_slot = var_ct_t_dn4;
        *var_ctg_t_slot = var_ctg_t;
        *var_ctg_t_dn4_slot = var_ctg_t_dn4;
        *var_dphibq_slot = var_dphibq;
        *var_dphibq_dn4_slot = var_dphibq_dn4;
        *var_g_0_ac_slot = var_g_0_ac;
        *var_g_0_ac_dn4_slot = var_g_0_ac_dn4;
        *var_g_0_dc_slot = var_g_0_dc;
        *var_g_0_dc_dn4_slot = var_g_0_dc_dn4;
        *var_guard1024_slot = var_guard1024;
        *var_guard1025_slot = var_guard1025;
        *var_guard1026_slot = var_guard1026;
        *var_guard1027_slot = var_guard1027;
        *var_kp_slot = var_kp;
        *var_kp_dn4_slot = var_kp_dn4;
        *var_mue_t_slot = var_mue_t;
        *var_mue_t_dn4_slot = var_mue_t_dn4;
        *var_np_slot = var_np;
        *var_nt_slot = var_nt;
        *var_nt0_slot = var_nt0;
        *var_nt0_dn4_slot = var_nt0_dn4;
        *var_nt_dn4_slot = var_nt_dn4;
        *var_phib_ac_slot = var_phib_ac;
        *var_phib_ac_dn4_slot = var_phib_ac_dn4;
        *var_phib_dc_slot = var_phib_dc;
        *var_phib_dc_dn4_slot = var_phib_dc_dn4;
        *var_phibfac_slot = var_phibfac;
        *var_phibfac_dn4_slot = var_phibfac_dn4;
        *var_phix1_ac_slot = var_phix1_ac;
        *var_phix1_ac_dn4_slot = var_phix1_ac_dn4;
        *var_phix1_dc_slot = var_phix1_dc;
        *var_phix1_dc_dn4_slot = var_phix1_dc_dn4;
        *var_phix2_slot = var_phix2;
        *var_phix2_dn4_slot = var_phix2_dn4;
        *var_phix_ac_slot = var_phix_ac;
        *var_phix_ac_dn4_slot = var_phix_ac_dn4;
        *var_phix_dc_slot = var_phix_dc;
        *var_phix_dc_dn4_slot = var_phix_dc_dn4;
        *var_qb0_slot = var_qb0;
        *var_qb0_dn4_slot = var_qb0_dn4;
        *var_qlim2_slot = var_qlim2;
        *var_qlim2_dn4_slot = var_qlim2_dn4;
        *var_rs_t_slot = var_rs_t;
        *var_rs_t_dn4_slot = var_rs_t_dn4;
        *var_sqrt_phib_dc_slot = var_sqrt_phib_dc;
        *var_sqrt_phib_dc_dn4_slot = var_sqrt_phib_dc_dn4;
        *var_tf_bet_slot = var_tf_bet;
        *var_tf_bet_dn4_slot = var_tf_bet_dn4;
        *var_tf_betedge_slot = var_tf_betedge;
        *var_tf_betedge_dn4_slot = var_tf_betedge_dn4;
        *var_tf_cs_slot = var_tf_cs;
        *var_tf_cs_dn4_slot = var_tf_cs_dn4;
        *var_tf_ct_slot = var_tf_ct;
        *var_tf_ct_dn4_slot = var_tf_ct_dn4;
        *var_tf_mue_slot = var_tf_mue;
        *var_tf_mue_dn4_slot = var_tf_mue_dn4;
        *var_tf_ther_slot = var_tf_ther;
        *var_tf_ther_dn4_slot = var_tf_ther_dn4;
        *var_tf_thesat_slot = var_tf_thesat;
        *var_tf_thesat_dn4_slot = var_tf_thesat_dn4;
        *var_tf_xcor_slot = var_tf_xcor;
        *var_tf_xcor_dn4_slot = var_tf_xcor_dn4;
        *var_thecs_t_slot = var_thecs_t;
        *var_thecs_t_dn4_slot = var_thecs_t_dn4;
        *var_themu_t_slot = var_themu_t;
        *var_themu_t_dn4_slot = var_themu_t_dn4;
        *var_ther_i_slot = var_ther_i;
        *var_ther_i_dn4_slot = var_ther_i_dn4;
        *var_thesat_t_slot = var_thesat_t;
        *var_thesat_t_dn4_slot = var_thesat_t_dn4;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_us1_slot = var_us1;
        *var_us1_dn4_slot = var_us1_dn4;
        *var_us21_slot = var_us21;
        *var_us21_dn4_slot = var_us21_dn4;
        *var_vfb_t_slot = var_vfb_t;
        *var_vfb_t_dn4_slot = var_vfb_t_dn4;
        *var_vfbedge_t_slot = var_vfbedge_t;
        *var_vfbedge_t_dn4_slot = var_vfbedge_t_dn4;
        *var_xcor_t_slot = var_xcor_t;
        *var_xcor_t_dn4_slot = var_xcor_t_dn4;
    }

    pub(super) fn stamp_transient_block_86(
        ctx: &GeneratedEvalContext<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        var_chnl_type: f64,
        var_coxprime: f64,
        var_ctedge_i: f64,
        var_dphibedge_i: f64,
        var_eg: f64,
        var_eg_dn4: f64,
        var_epssi: f64,
        var_guard1027: f64,
        var_inv_phit: f64,
        var_inv_phit_dn4: f64,
        var_inv_phita: f64,
        var_neffedge_i: f64,
        var_phibfac: f64,
        var_phibfac_dn4: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_rtn: f64,
        var_rtn_dn4: f64,
        var_vfb_t: f64,
        var_vfb_t_dn4: f64,
        var_aphiedge_slot: &mut f64,
        var_aphiedge_dn4_slot: &mut f64,
        var_betedge_i_slot: &mut f64,
        var_betedge_i_dn4_slot: &mut f64,
        var_betnedge_t_slot: &mut f64,
        var_betnedge_t_dn4_slot: &mut f64,
        var_bphiedge_slot: &mut f64,
        var_bphiedge_dn4_slot: &mut f64,
        var_gfedge_slot: &mut f64,
        var_gfedge2_slot: &mut f64,
        var_gfedge2_dn4_slot: &mut f64,
        var_gfedge_dn4_slot: &mut f64,
        var_guard1028_slot: &mut f64,
        var_guard1029_slot: &mut f64,
        var_lngfedge2_slot: &mut f64,
        var_lngfedge2_dn4_slot: &mut f64,
        var_phibedge_slot: &mut f64,
        var_phibedge_dn4_slot: &mut f64,
        var_phit0edge_slot: &mut f64,
        var_phit0edge_dn4_slot: &mut f64,
        var_phix1edge_slot: &mut f64,
        var_phix1edge_dn4_slot: &mut f64,
        var_phix2edge_slot: &mut f64,
        var_phix2edge_dn4_slot: &mut f64,
        var_phixedge_slot: &mut f64,
        var_phixedge_dn4_slot: &mut f64,
        var_sigvds_slot: &mut f64,
        var_tf_betedge_slot: &mut f64,
        var_tf_betedge_dn4_slot: &mut f64,
        var_v_ds_slot: &mut f64,
        var_v_ds_dn7_slot: &mut f64,
        var_v_ds_dn8_slot: &mut f64,
        var_v_gs_slot: &mut f64,
        var_v_gs_dn6_slot: &mut f64,
        var_v_gs_dn7_slot: &mut f64,
        var_v_gs_dn8_slot: &mut f64,
        var_v_sb_slot: &mut f64,
        var_v_sb_dn7_slot: &mut f64,
        var_v_sb_dn8_slot: &mut f64,
        var_v_sb_dn9_slot: &mut f64,
        var_vdbprime_slot: &mut f64,
        var_vdbprime_dn7_slot: &mut f64,
        var_vdbprime_dn8_slot: &mut f64,
        var_vdbprime_dn9_slot: &mut f64,
        var_vfbedge_t_slot: &mut f64,
        var_vfbedge_t_dn4_slot: &mut f64,
        var_vgb_slot: &mut f64,
        var_vgb_dn6_slot: &mut f64,
        var_vgb_dn7_slot: &mut f64,
        var_vgb_dn8_slot: &mut f64,
        var_vgb_dn9_slot: &mut f64,
        var_vgdprime_slot: &mut f64,
        var_vgdprime_dn6_slot: &mut f64,
        var_vgdprime_dn7_slot: &mut f64,
        var_vgdprime_dn8_slot: &mut f64,
        var_vgsprime_slot: &mut f64,
        var_vgsprime_dn6_slot: &mut f64,
        var_vgsprime_dn7_slot: &mut f64,
        var_vgsprime_dn8_slot: &mut f64,
        var_vjun_d_slot: &mut f64,
        var_vjun_d_dn12_slot: &mut f64,
        var_vjun_d_dn8_slot: &mut f64,
        var_vjun_s_slot: &mut f64,
        var_vjun_s_dn11_slot: &mut f64,
        var_vjun_s_dn7_slot: &mut f64,
        var_vsbprime_slot: &mut f64,
        var_vsbprime_dn7_slot: &mut f64,
        var_vsbprime_dn8_slot: &mut f64,
        var_vsbprime_dn9_slot: &mut f64,
        var_xgb_ov_slot: &mut f64,
        var_xgb_ov_dn4_slot: &mut f64,
        var_xgb_ov_dn6_slot: &mut f64,
        var_xgb_ov_dn7_slot: &mut f64,
        var_xgb_ov_dn8_slot: &mut f64,
        var_xgb_ov_dn9_slot: &mut f64,
        var_xgd_ov_slot: &mut f64,
        var_xgd_ov_dn6_slot: &mut f64,
        var_xgd_ov_dn7_slot: &mut f64,
        var_xgd_ov_dn8_slot: &mut f64,
        var_xgs_ov_slot: &mut f64,
        var_xgs_ov_dn6_slot: &mut f64,
        var_xgs_ov_dn7_slot: &mut f64,
        var_xgs_ov_dn8_slot: &mut f64,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let mut var_aphiedge: f64 = *var_aphiedge_slot;
        let mut var_aphiedge_dn4: f64 = *var_aphiedge_dn4_slot;
        let mut var_betedge_i: f64 = *var_betedge_i_slot;
        let mut var_betedge_i_dn4: f64 = *var_betedge_i_dn4_slot;
        let mut var_betnedge_t: f64 = *var_betnedge_t_slot;
        let mut var_betnedge_t_dn4: f64 = *var_betnedge_t_dn4_slot;
        let mut var_bphiedge: f64 = *var_bphiedge_slot;
        let mut var_bphiedge_dn4: f64 = *var_bphiedge_dn4_slot;
        let mut var_gfedge: f64 = *var_gfedge_slot;
        let mut var_gfedge2: f64 = *var_gfedge2_slot;
        let mut var_gfedge2_dn4: f64 = *var_gfedge2_dn4_slot;
        let mut var_gfedge_dn4: f64 = *var_gfedge_dn4_slot;
        let mut var_guard1028: f64 = *var_guard1028_slot;
        let mut var_guard1029: f64 = *var_guard1029_slot;
        let mut var_lngfedge2: f64 = *var_lngfedge2_slot;
        let mut var_lngfedge2_dn4: f64 = *var_lngfedge2_dn4_slot;
        let mut var_phibedge: f64 = *var_phibedge_slot;
        let mut var_phibedge_dn4: f64 = *var_phibedge_dn4_slot;
        let mut var_phit0edge: f64 = *var_phit0edge_slot;
        let mut var_phit0edge_dn4: f64 = *var_phit0edge_dn4_slot;
        let mut var_phix1edge: f64 = *var_phix1edge_slot;
        let mut var_phix1edge_dn4: f64 = *var_phix1edge_dn4_slot;
        let mut var_phix2edge: f64 = *var_phix2edge_slot;
        let mut var_phix2edge_dn4: f64 = *var_phix2edge_dn4_slot;
        let mut var_phixedge: f64 = *var_phixedge_slot;
        let mut var_phixedge_dn4: f64 = *var_phixedge_dn4_slot;
        let mut var_sigvds: f64 = *var_sigvds_slot;
        let mut var_tf_betedge: f64 = *var_tf_betedge_slot;
        let mut var_tf_betedge_dn4: f64 = *var_tf_betedge_dn4_slot;
        let mut var_v_ds: f64 = *var_v_ds_slot;
        let mut var_v_ds_dn7: f64 = *var_v_ds_dn7_slot;
        let mut var_v_ds_dn8: f64 = *var_v_ds_dn8_slot;
        let mut var_v_gs: f64 = *var_v_gs_slot;
        let mut var_v_gs_dn6: f64 = *var_v_gs_dn6_slot;
        let mut var_v_gs_dn7: f64 = *var_v_gs_dn7_slot;
        let mut var_v_gs_dn8: f64 = *var_v_gs_dn8_slot;
        let mut var_v_sb: f64 = *var_v_sb_slot;
        let mut var_v_sb_dn7: f64 = *var_v_sb_dn7_slot;
        let mut var_v_sb_dn8: f64 = *var_v_sb_dn8_slot;
        let mut var_v_sb_dn9: f64 = *var_v_sb_dn9_slot;
        let mut var_vdbprime: f64 = *var_vdbprime_slot;
        let mut var_vdbprime_dn7: f64 = *var_vdbprime_dn7_slot;
        let mut var_vdbprime_dn8: f64 = *var_vdbprime_dn8_slot;
        let mut var_vdbprime_dn9: f64 = *var_vdbprime_dn9_slot;
        let mut var_vfbedge_t: f64 = *var_vfbedge_t_slot;
        let mut var_vfbedge_t_dn4: f64 = *var_vfbedge_t_dn4_slot;
        let mut var_vgb: f64 = *var_vgb_slot;
        let mut var_vgb_dn6: f64 = *var_vgb_dn6_slot;
        let mut var_vgb_dn7: f64 = *var_vgb_dn7_slot;
        let mut var_vgb_dn8: f64 = *var_vgb_dn8_slot;
        let mut var_vgb_dn9: f64 = *var_vgb_dn9_slot;
        let mut var_vgdprime: f64 = *var_vgdprime_slot;
        let mut var_vgdprime_dn6: f64 = *var_vgdprime_dn6_slot;
        let mut var_vgdprime_dn7: f64 = *var_vgdprime_dn7_slot;
        let mut var_vgdprime_dn8: f64 = *var_vgdprime_dn8_slot;
        let mut var_vgsprime: f64 = *var_vgsprime_slot;
        let mut var_vgsprime_dn6: f64 = *var_vgsprime_dn6_slot;
        let mut var_vgsprime_dn7: f64 = *var_vgsprime_dn7_slot;
        let mut var_vgsprime_dn8: f64 = *var_vgsprime_dn8_slot;
        let mut var_vjun_d: f64 = *var_vjun_d_slot;
        let mut var_vjun_d_dn12: f64 = *var_vjun_d_dn12_slot;
        let mut var_vjun_d_dn8: f64 = *var_vjun_d_dn8_slot;
        let mut var_vjun_s: f64 = *var_vjun_s_slot;
        let mut var_vjun_s_dn11: f64 = *var_vjun_s_dn11_slot;
        let mut var_vjun_s_dn7: f64 = *var_vjun_s_dn7_slot;
        let mut var_vsbprime: f64 = *var_vsbprime_slot;
        let mut var_vsbprime_dn7: f64 = *var_vsbprime_dn7_slot;
        let mut var_vsbprime_dn8: f64 = *var_vsbprime_dn8_slot;
        let mut var_vsbprime_dn9: f64 = *var_vsbprime_dn9_slot;
        let mut var_xgb_ov: f64 = *var_xgb_ov_slot;
        let mut var_xgb_ov_dn4: f64 = *var_xgb_ov_dn4_slot;
        let mut var_xgb_ov_dn6: f64 = *var_xgb_ov_dn6_slot;
        let mut var_xgb_ov_dn7: f64 = *var_xgb_ov_dn7_slot;
        let mut var_xgb_ov_dn8: f64 = *var_xgb_ov_dn8_slot;
        let mut var_xgb_ov_dn9: f64 = *var_xgb_ov_dn9_slot;
        let mut var_xgd_ov: f64 = *var_xgd_ov_slot;
        let mut var_xgd_ov_dn6: f64 = *var_xgd_ov_dn6_slot;
        let mut var_xgd_ov_dn7: f64 = *var_xgd_ov_dn7_slot;
        let mut var_xgd_ov_dn8: f64 = *var_xgd_ov_dn8_slot;
        let mut var_xgs_ov: f64 = *var_xgs_ov_slot;
        let mut var_xgs_ov_dn6: f64 = *var_xgs_ov_dn6_slot;
        let mut var_xgs_ov_dn7: f64 = *var_xgs_ov_dn7_slot;
        let mut var_xgs_ov_dn8: f64 = *var_xgs_ov_dn8_slot;

        let (assign40210_e53473, assign40210_e53473_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40210_e53469: f64 = (var_ctedge_i * var_rtn);
        let assign40210_e53470: f64 = (1.0 + assign40210_e53469);
        let assign40210_e53471: f64 = (var_phit * assign40210_e53470);
        (assign40210_e53471, ((var_phit_dn4 * assign40210_e53470) + (var_phit * (var_ctedge_i * var_rtn_dn4))),)
    } else {
        (var_phit0edge, var_phit0edge_dn4,)
    }
};
        var_phit0edge = assign40210_e53473;
        var_phit0edge_dn4 = assign40210_e53473_d_n4;

        let (assign40220_e53493, assign40220_e53493_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40220_e53477: f64 = (var_eg + var_dphibedge_i);
        let assign40220_e53480: f64 = (2.0 * var_phit0edge);
        let assign40220_e53484: f64 = (-0.75);
        let assign40220_e53485: f64 = (var_phibfac).powf(assign40220_e53484);
        let assign40220_e53486: f64 = (var_neffedge_i * assign40220_e53485);
        let assign40220_e53488: f64 = (assign40220_e53486 * 4e-26);
        let assign40220_e53489: f64 = (assign40220_e53488).ln();
        let assign40220_e53490: f64 = (assign40220_e53480 * assign40220_e53489);
        let assign40220_e53491: f64 = (assign40220_e53477 + assign40220_e53490);
        (assign40220_e53491, (var_eg_dn4 + (((2.0 * var_phit0edge_dn4) * assign40220_e53489) + (assign40220_e53480 * (((var_neffedge_i * if 0.0 == 0.0 && ((assign40220_e53484) as f64).is_finite() && ((assign40220_e53484) as f64).fract() == 0.0 { if assign40220_e53484 == 0.0 { 0.0 } else { (assign40220_e53484 * ((var_phibfac).powf(assign40220_e53484 - 1.0) * var_phibfac_dn4)) } } else { (assign40220_e53485 * (assign40220_e53484 * (var_phibfac_dn4 / var_phibfac))) }) * 4e-26) / assign40220_e53488)))),)
    } else {
        (var_phibedge, var_phibedge_dn4,)
    }
};
        var_phibedge = assign40220_e53493;
        var_phibedge_dn4 = assign40220_e53493_d_n4;

        let (assign40230_e53502, assign40230_e53502_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let (assign40230_e53500, assign40230_e53500_d_n4,) = {
            if (var_phibedge > 0.05) {
                (var_phibedge, var_phibedge_dn4,)
            } else {
                (0.05, 0.0,)
            }
        };
        (assign40230_e53500, assign40230_e53500_d_n4,)
    } else {
        (var_phibedge, var_phibedge_dn4,)
    }
};
        var_phibedge = assign40230_e53502;
        var_phibedge_dn4 = assign40230_e53502_d_n4;

        let (assign40240_e53517, assign40240_e53517_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40240_e53506: f64 = (2.0 * 1.6021918e-19);
        let assign40240_e53508: f64 = (assign40240_e53506 * var_neffedge_i);
        let assign40240_e53510: f64 = (assign40240_e53508 * var_epssi);
        let assign40240_e53512: f64 = (assign40240_e53510 * var_inv_phit);
        let assign40240_e53513: f64 = (assign40240_e53512).sqrt();
        let assign40240_e53515: f64 = (assign40240_e53513 / var_coxprime);
        (assign40240_e53515, (((assign40240_e53510 * var_inv_phit_dn4) / (2.0 * assign40240_e53513)) / var_coxprime),)
    } else {
        (var_gfedge, var_gfedge_dn4,)
    }
};
        var_gfedge = assign40240_e53517;
        var_gfedge_dn4 = assign40240_e53517_d_n4;

        let (assign40250_e53523, assign40250_e53523_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40250_e53521: f64 = (var_gfedge * var_gfedge);
        (assign40250_e53521, ((var_gfedge_dn4 * var_gfedge) + (var_gfedge * var_gfedge_dn4)),)
    } else {
        (var_gfedge2, var_gfedge2_dn4,)
    }
};
        var_gfedge2 = assign40250_e53523;
        var_gfedge2_dn4 = assign40250_e53523_d_n4;

        let (assign40260_e53528, assign40260_e53528_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40260_e53526: f64 = (var_gfedge2).ln();
        (assign40260_e53526, (var_gfedge2_dn4 / var_gfedge2),)
    } else {
        (var_lngfedge2, var_lngfedge2_dn4,)
    }
};
        var_lngfedge2 = assign40260_e53528;
        var_lngfedge2_dn4 = assign40260_e53528_d_n4;

        let (assign40270_e53534, assign40270_e53534_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40270_e53532: f64 = (0.95 * var_phibedge);
        (assign40270_e53532, (0.95 * var_phibedge_dn4),)
    } else {
        (var_phixedge, var_phixedge_dn4,)
    }
};
        var_phixedge = assign40270_e53534;
        var_phixedge_dn4 = assign40270_e53534_d_n4;

        let (assign40280_e53542, assign40280_e53542_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40280_e53538: f64 = (0.0025 * var_phibedge);
        let assign40280_e53540: f64 = (assign40280_e53538 * var_phibedge);
        (assign40280_e53540, (((0.0025 * var_phibedge_dn4) * var_phibedge) + (assign40280_e53538 * var_phibedge_dn4)),)
    } else {
        (var_aphiedge, var_aphiedge_dn4,)
    }
};
        var_aphiedge = assign40280_e53542;
        var_aphiedge_dn4 = assign40280_e53542_d_n4;

        let (assign40290_e53546, assign40290_e53546_d_n4,) = {
    if (var_guard1027 != 0.0) {
        (var_aphiedge, var_aphiedge_dn4,)
    } else {
        (var_bphiedge, var_bphiedge_dn4,)
    }
};
        var_bphiedge = assign40290_e53546;
        var_bphiedge_dn4 = assign40290_e53546_d_n4;

        let (assign40300_e53553, assign40300_e53553_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40300_e53550: f64 = (var_bphiedge).sqrt();
        let assign40300_e53551: f64 = (0.5 * assign40300_e53550);
        (assign40300_e53551, (0.5 * (var_bphiedge_dn4 / (2.0 * assign40300_e53550))),)
    } else {
        (var_phix2edge, var_phix2edge_dn4,)
    }
};
        var_phix2edge = assign40300_e53553;
        var_phix2edge_dn4 = assign40300_e53553_d_n4;

        let (assign40310_e53578, assign40310_e53578_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40310_e53558: f64 = (var_phixedge - var_phix2edge);
        let assign40310_e53560: f64 = assign40310_e53558;
        let assign40310_e53563: f64 = (var_phixedge - var_phix2edge);
        let assign40310_e53565: f64 = assign40310_e53563;
        let assign40310_e53568: f64 = (var_phixedge - var_phix2edge);
        let assign40310_e53570: f64 = assign40310_e53568;
        let assign40310_e53571: f64 = (assign40310_e53565 * assign40310_e53570);
        let assign40310_e53573: f64 = (assign40310_e53571 + var_aphiedge);
        let assign40310_e53574: f64 = (assign40310_e53573).sqrt();
        let assign40310_e53575: f64 = (assign40310_e53560 - assign40310_e53574);
        let assign40310_e53576: f64 = (0.5 * assign40310_e53575);
        (assign40310_e53576, (0.5 * ((var_phixedge_dn4 - var_phix2edge_dn4) - (((((var_phixedge_dn4 - var_phix2edge_dn4) * assign40310_e53570) + (assign40310_e53565 * (var_phixedge_dn4 - var_phix2edge_dn4))) + var_aphiedge_dn4) / (2.0 * assign40310_e53574)))),)
    } else {
        (var_phix1edge, var_phix1edge_dn4,)
    }
};
        var_phix1edge = assign40310_e53578;
        var_phix1edge_dn4 = assign40310_e53578_d_n4;

        let (assign40340_e53603, assign40340_e53603_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_vfbedge_t, var_vfbedge_t_dn4,)
    }
};
        var_vfbedge_t = assign40340_e53603;
        var_vfbedge_t_dn4 = assign40340_e53603_d_n4;

        let (assign40350_e53608, assign40350_e53608_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (1.0, 0.0,)
    } else {
        (var_tf_betedge, var_tf_betedge_dn4,)
    }
};
        var_tf_betedge = assign40350_e53608;
        var_tf_betedge_dn4 = assign40350_e53608_d_n4;

        let (assign40360_e53613, assign40360_e53613_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_betnedge_t, var_betnedge_t_dn4,)
    }
};
        var_betnedge_t = assign40360_e53613;
        var_betnedge_t_dn4 = assign40360_e53613_d_n4;

        let (assign40370_e53618, assign40370_e53618_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_betedge_i, var_betedge_i_dn4,)
    }
};
        var_betedge_i = assign40370_e53618;
        var_betedge_i_dn4 = assign40370_e53618_d_n4;

        let (assign40380_e53623, assign40380_e53623_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (var_phit, var_phit_dn4,)
    } else {
        (var_phit0edge, var_phit0edge_dn4,)
    }
};
        var_phit0edge = assign40380_e53623;
        var_phit0edge_dn4 = assign40380_e53623_d_n4;

        let (assign40390_e53628, assign40390_e53628_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_phibedge, var_phibedge_dn4,)
    }
};
        var_phibedge = assign40390_e53628;
        var_phibedge_dn4 = assign40390_e53628_d_n4;

        let (assign40400_e53633, assign40400_e53633_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (1.0, 0.0,)
    } else {
        (var_gfedge, var_gfedge_dn4,)
    }
};
        var_gfedge = assign40400_e53633;
        var_gfedge_dn4 = assign40400_e53633_d_n4;

        let (assign40410_e53638, assign40410_e53638_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (1.0, 0.0,)
    } else {
        (var_gfedge2, var_gfedge2_dn4,)
    }
};
        var_gfedge2 = assign40410_e53638;
        var_gfedge2_dn4 = assign40410_e53638_d_n4;

        let (assign40420_e53643, assign40420_e53643_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_lngfedge2, var_lngfedge2_dn4,)
    }
};
        var_lngfedge2 = assign40420_e53643;
        var_lngfedge2_dn4 = assign40420_e53643_d_n4;

        let (assign40430_e53648, assign40430_e53648_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_phixedge, var_phixedge_dn4,)
    }
};
        var_phixedge = assign40430_e53648;
        var_phixedge_dn4 = assign40430_e53648_d_n4;

        let (assign40440_e53653, assign40440_e53653_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_aphiedge, var_aphiedge_dn4,)
    }
};
        var_aphiedge = assign40440_e53653;
        var_aphiedge_dn4 = assign40440_e53653_d_n4;

        let (assign40450_e53658, assign40450_e53658_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_bphiedge, var_bphiedge_dn4,)
    }
};
        var_bphiedge = assign40450_e53658;
        var_bphiedge_dn4 = assign40450_e53658_d_n4;

        let (assign40460_e53663, assign40460_e53663_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_phix2edge, var_phix2edge_dn4,)
    }
};
        var_phix2edge = assign40460_e53663;
        var_phix2edge_dn4 = assign40460_e53663_d_n4;

        let (assign40470_e53668, assign40470_e53668_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_phix1edge, var_phix1edge_dn4,)
    }
};
        var_phix1edge = assign40470_e53668;
        var_phix1edge_dn4 = assign40470_e53668_d_n4;

        let assign40500_e53681: f64 = 1.0;
        let assign40500_e53682: f64 = if var_chnl_type == assign40500_e53681 { 1.0 } else { 0.0 };
        var_guard1028 = assign40500_e53682;

        let (assign40510_e53686, assign40510_e53686_d_n6, assign40510_e53686_d_n7, assign40510_e53686_d_n8,) = {
    if (var_guard1028 != 0.0) {
        ((nv6 - nv7), 1.0, -1.0, 0.0,)
    } else {
        (var_v_gs, var_v_gs_dn6, var_v_gs_dn7, var_v_gs_dn8,)
    }
};
        var_v_gs = assign40510_e53686;
        var_v_gs_dn6 = assign40510_e53686_d_n6;
        var_v_gs_dn7 = assign40510_e53686_d_n7;
        var_v_gs_dn8 = assign40510_e53686_d_n8;

        let (assign40520_e53690, assign40520_e53690_d_n7, assign40520_e53690_d_n8,) = {
    if (var_guard1028 != 0.0) {
        ((nv8 - nv7), -1.0, 1.0,)
    } else {
        (var_v_ds, var_v_ds_dn7, var_v_ds_dn8,)
    }
};
        var_v_ds = assign40520_e53690;
        var_v_ds_dn7 = assign40520_e53690_d_n7;
        var_v_ds_dn8 = assign40520_e53690_d_n8;

        let (assign40530_e53694, assign40530_e53694_d_n7, assign40530_e53694_d_n8, assign40530_e53694_d_n9,) = {
    if (var_guard1028 != 0.0) {
        ((nv7 - nv9), 1.0, 0.0, -1.0,)
    } else {
        (var_v_sb, var_v_sb_dn7, var_v_sb_dn8, var_v_sb_dn9,)
    }
};
        var_v_sb = assign40530_e53694;
        var_v_sb_dn7 = assign40530_e53694_d_n7;
        var_v_sb_dn8 = assign40530_e53694_d_n8;
        var_v_sb_dn9 = assign40530_e53694_d_n9;

        let (assign40540_e53699, assign40540_e53699_d_n7, assign40540_e53699_d_n11,) = {
    if (var_guard1028 != 0.0) {
        let assign40540_e53697: f64 = (-(nv7 - nv11));
        (assign40540_e53697, (-1.0), 1.0,)
    } else {
        (var_vjun_s, var_vjun_s_dn7, var_vjun_s_dn11,)
    }
};
        var_vjun_s = assign40540_e53699;
        var_vjun_s_dn7 = assign40540_e53699_d_n7;
        var_vjun_s_dn11 = assign40540_e53699_d_n11;

        let (assign40550_e53704, assign40550_e53704_d_n8, assign40550_e53704_d_n12,) = {
    if (var_guard1028 != 0.0) {
        let assign40550_e53702: f64 = (-(nv8 - nv12));
        (assign40550_e53702, (-1.0), 1.0,)
    } else {
        (var_vjun_d, var_vjun_d_dn8, var_vjun_d_dn12,)
    }
};
        var_vjun_d = assign40550_e53704;
        var_vjun_d_dn8 = assign40550_e53704_d_n8;
        var_vjun_d_dn12 = assign40550_e53704_d_n12;

        let (assign40560_e53710, assign40560_e53710_d_n6, assign40560_e53710_d_n7, assign40560_e53710_d_n8,) = {
    if (var_guard1028 == 0.0) {
        let assign40560_e53708: f64 = (-(nv6 - nv7));
        (assign40560_e53708, (-1.0), 1.0, 0.0,)
    } else {
        (var_v_gs, var_v_gs_dn6, var_v_gs_dn7, var_v_gs_dn8,)
    }
};
        var_v_gs = assign40560_e53710;
        var_v_gs_dn6 = assign40560_e53710_d_n6;
        var_v_gs_dn7 = assign40560_e53710_d_n7;
        var_v_gs_dn8 = assign40560_e53710_d_n8;

        let (assign40570_e53716, assign40570_e53716_d_n7, assign40570_e53716_d_n8,) = {
    if (var_guard1028 == 0.0) {
        let assign40570_e53714: f64 = (-(nv8 - nv7));
        (assign40570_e53714, 1.0, (-1.0),)
    } else {
        (var_v_ds, var_v_ds_dn7, var_v_ds_dn8,)
    }
};
        var_v_ds = assign40570_e53716;
        var_v_ds_dn7 = assign40570_e53716_d_n7;
        var_v_ds_dn8 = assign40570_e53716_d_n8;

        let (assign40580_e53722, assign40580_e53722_d_n7, assign40580_e53722_d_n8, assign40580_e53722_d_n9,) = {
    if (var_guard1028 == 0.0) {
        let assign40580_e53720: f64 = (-(nv7 - nv9));
        (assign40580_e53720, (-1.0), 0.0, 1.0,)
    } else {
        (var_v_sb, var_v_sb_dn7, var_v_sb_dn8, var_v_sb_dn9,)
    }
};
        var_v_sb = assign40580_e53722;
        var_v_sb_dn7 = assign40580_e53722_d_n7;
        var_v_sb_dn8 = assign40580_e53722_d_n8;
        var_v_sb_dn9 = assign40580_e53722_d_n9;

        let (assign40590_e53727, assign40590_e53727_d_n7, assign40590_e53727_d_n11,) = {
    if (var_guard1028 == 0.0) {
        ((nv7 - nv11), 1.0, -1.0,)
    } else {
        (var_vjun_s, var_vjun_s_dn7, var_vjun_s_dn11,)
    }
};
        var_vjun_s = assign40590_e53727;
        var_vjun_s_dn7 = assign40590_e53727_d_n7;
        var_vjun_s_dn11 = assign40590_e53727_d_n11;

        let (assign40600_e53732, assign40600_e53732_d_n8, assign40600_e53732_d_n12,) = {
    if (var_guard1028 == 0.0) {
        ((nv8 - nv12), 1.0, -1.0,)
    } else {
        (var_vjun_d, var_vjun_d_dn8, var_vjun_d_dn12,)
    }
};
        var_vjun_d = assign40600_e53732;
        var_vjun_d_dn8 = assign40600_e53732_d_n8;
        var_vjun_d_dn12 = assign40600_e53732_d_n12;

        let assign40610_e53735: f64 = (var_v_gs + var_v_sb);
        var_vgb = assign40610_e53735;
        var_vgb_dn6 = var_v_gs_dn6;
        var_vgb_dn7 = (var_v_gs_dn7 + var_v_sb_dn7);
        var_vgb_dn8 = (var_v_gs_dn8 + var_v_sb_dn8);
        var_vgb_dn9 = var_v_sb_dn9;

        var_vgsprime = var_v_gs;
        var_vgsprime_dn6 = var_v_gs_dn6;
        var_vgsprime_dn7 = var_v_gs_dn7;
        var_vgsprime_dn8 = var_v_gs_dn8;

        var_vsbprime = var_v_sb;
        var_vsbprime_dn7 = var_v_sb_dn7;
        var_vsbprime_dn8 = var_v_sb_dn8;
        var_vsbprime_dn9 = var_v_sb_dn9;

        let assign40640_e53740: f64 = (var_v_ds + var_v_sb);
        var_vdbprime = assign40640_e53740;
        var_vdbprime_dn7 = (var_v_ds_dn7 + var_v_sb_dn7);
        var_vdbprime_dn8 = (var_v_ds_dn8 + var_v_sb_dn8);
        var_vdbprime_dn9 = var_v_sb_dn9;

        let assign40650_e53743: f64 = (var_v_gs - var_v_ds);
        var_vgdprime = assign40650_e53743;
        var_vgdprime_dn6 = var_v_gs_dn6;
        var_vgdprime_dn7 = (var_v_gs_dn7 - var_v_ds_dn7);
        var_vgdprime_dn8 = (var_v_gs_dn8 - var_v_ds_dn8);

        let assign40660_e53745: f64 = (-var_vgsprime);
        let assign40660_e53747: f64 = (assign40660_e53745 * var_inv_phita);
        var_xgs_ov = assign40660_e53747;
        var_xgs_ov_dn6 = ((-var_vgsprime_dn6) * var_inv_phita);
        var_xgs_ov_dn7 = ((-var_vgsprime_dn7) * var_inv_phita);
        var_xgs_ov_dn8 = ((-var_vgsprime_dn8) * var_inv_phita);

        let assign40670_e53749: f64 = (-var_vgdprime);
        let assign40670_e53751: f64 = (assign40670_e53749 * var_inv_phita);
        var_xgd_ov = assign40670_e53751;
        var_xgd_ov_dn6 = ((-var_vgdprime_dn6) * var_inv_phita);
        var_xgd_ov_dn7 = ((-var_vgdprime_dn7) * var_inv_phita);
        var_xgd_ov_dn8 = ((-var_vgdprime_dn8) * var_inv_phita);

        let assign40680_e53754: f64 = (var_vgb - var_vfb_t);
        let assign40680_e53755: f64 = (-assign40680_e53754);
        let assign40680_e53757: f64 = (assign40680_e53755 * var_inv_phita);
        var_xgb_ov = assign40680_e53757;
        var_xgb_ov_dn4 = ((-(-var_vfb_t_dn4)) * var_inv_phita);
        var_xgb_ov_dn6 = ((-var_vgb_dn6) * var_inv_phita);
        var_xgb_ov_dn7 = ((-var_vgb_dn7) * var_inv_phita);
        var_xgb_ov_dn8 = ((-var_vgb_dn8) * var_inv_phita);
        var_xgb_ov_dn9 = ((-var_vgb_dn9) * var_inv_phita);

        var_sigvds = 1.0;

        let assign40700_e53761: f64 = if var_v_ds < 0.0 { 1.0 } else { 0.0 };
        var_guard1029 = assign40700_e53761;

        let (assign40710_e53766,) = {
    if (var_guard1029 != 0.0) {
        let assign40710_e53764: f64 = (-1.0);
        (assign40710_e53764,)
    } else {
        (var_sigvds,)
    }
};
        var_sigvds = assign40710_e53766;

        let (assign40720_e53772, assign40720_e53772_d_n6, assign40720_e53772_d_n7, assign40720_e53772_d_n8,) = {
    if (var_guard1029 != 0.0) {
        let assign40720_e53770: f64 = (var_v_gs - var_v_ds);
        (assign40720_e53770, var_v_gs_dn6, (var_v_gs_dn7 - var_v_ds_dn7), (var_v_gs_dn8 - var_v_ds_dn8),)
    } else {
        (var_v_gs, var_v_gs_dn6, var_v_gs_dn7, var_v_gs_dn8,)
    }
};
        var_v_gs = assign40720_e53772;
        var_v_gs_dn6 = assign40720_e53772_d_n6;
        var_v_gs_dn7 = assign40720_e53772_d_n7;
        var_v_gs_dn8 = assign40720_e53772_d_n8;

        let (assign40730_e53778, assign40730_e53778_d_n7, assign40730_e53778_d_n8, assign40730_e53778_d_n9,) = {
    if (var_guard1029 != 0.0) {
        let assign40730_e53776: f64 = (var_v_sb + var_v_ds);
        (assign40730_e53776, (var_v_sb_dn7 + var_v_ds_dn7), (var_v_sb_dn8 + var_v_ds_dn8), var_v_sb_dn9,)
    } else {
        (var_v_sb, var_v_sb_dn7, var_v_sb_dn8, var_v_sb_dn9,)
    }
};
        var_v_sb = assign40730_e53778;
        var_v_sb_dn7 = assign40730_e53778_d_n7;
        var_v_sb_dn8 = assign40730_e53778_d_n8;
        var_v_sb_dn9 = assign40730_e53778_d_n9;

        *var_aphiedge_slot = var_aphiedge;
        *var_aphiedge_dn4_slot = var_aphiedge_dn4;
        *var_betedge_i_slot = var_betedge_i;
        *var_betedge_i_dn4_slot = var_betedge_i_dn4;
        *var_betnedge_t_slot = var_betnedge_t;
        *var_betnedge_t_dn4_slot = var_betnedge_t_dn4;
        *var_bphiedge_slot = var_bphiedge;
        *var_bphiedge_dn4_slot = var_bphiedge_dn4;
        *var_gfedge_slot = var_gfedge;
        *var_gfedge2_slot = var_gfedge2;
        *var_gfedge2_dn4_slot = var_gfedge2_dn4;
        *var_gfedge_dn4_slot = var_gfedge_dn4;
        *var_guard1028_slot = var_guard1028;
        *var_guard1029_slot = var_guard1029;
        *var_lngfedge2_slot = var_lngfedge2;
        *var_lngfedge2_dn4_slot = var_lngfedge2_dn4;
        *var_phibedge_slot = var_phibedge;
        *var_phibedge_dn4_slot = var_phibedge_dn4;
        *var_phit0edge_slot = var_phit0edge;
        *var_phit0edge_dn4_slot = var_phit0edge_dn4;
        *var_phix1edge_slot = var_phix1edge;
        *var_phix1edge_dn4_slot = var_phix1edge_dn4;
        *var_phix2edge_slot = var_phix2edge;
        *var_phix2edge_dn4_slot = var_phix2edge_dn4;
        *var_phixedge_slot = var_phixedge;
        *var_phixedge_dn4_slot = var_phixedge_dn4;
        *var_sigvds_slot = var_sigvds;
        *var_tf_betedge_slot = var_tf_betedge;
        *var_tf_betedge_dn4_slot = var_tf_betedge_dn4;
        *var_v_ds_slot = var_v_ds;
        *var_v_ds_dn7_slot = var_v_ds_dn7;
        *var_v_ds_dn8_slot = var_v_ds_dn8;
        *var_v_gs_slot = var_v_gs;
        *var_v_gs_dn6_slot = var_v_gs_dn6;
        *var_v_gs_dn7_slot = var_v_gs_dn7;
        *var_v_gs_dn8_slot = var_v_gs_dn8;
        *var_v_sb_slot = var_v_sb;
        *var_v_sb_dn7_slot = var_v_sb_dn7;
        *var_v_sb_dn8_slot = var_v_sb_dn8;
        *var_v_sb_dn9_slot = var_v_sb_dn9;
        *var_vdbprime_slot = var_vdbprime;
        *var_vdbprime_dn7_slot = var_vdbprime_dn7;
        *var_vdbprime_dn8_slot = var_vdbprime_dn8;
        *var_vdbprime_dn9_slot = var_vdbprime_dn9;
        *var_vfbedge_t_slot = var_vfbedge_t;
        *var_vfbedge_t_dn4_slot = var_vfbedge_t_dn4;
        *var_vgb_slot = var_vgb;
        *var_vgb_dn6_slot = var_vgb_dn6;
        *var_vgb_dn7_slot = var_vgb_dn7;
        *var_vgb_dn8_slot = var_vgb_dn8;
        *var_vgb_dn9_slot = var_vgb_dn9;
        *var_vgdprime_slot = var_vgdprime;
        *var_vgdprime_dn6_slot = var_vgdprime_dn6;
        *var_vgdprime_dn7_slot = var_vgdprime_dn7;
        *var_vgdprime_dn8_slot = var_vgdprime_dn8;
        *var_vgsprime_slot = var_vgsprime;
        *var_vgsprime_dn6_slot = var_vgsprime_dn6;
        *var_vgsprime_dn7_slot = var_vgsprime_dn7;
        *var_vgsprime_dn8_slot = var_vgsprime_dn8;
        *var_vjun_d_slot = var_vjun_d;
        *var_vjun_d_dn12_slot = var_vjun_d_dn12;
        *var_vjun_d_dn8_slot = var_vjun_d_dn8;
        *var_vjun_s_slot = var_vjun_s;
        *var_vjun_s_dn11_slot = var_vjun_s_dn11;
        *var_vjun_s_dn7_slot = var_vjun_s_dn7;
        *var_vsbprime_slot = var_vsbprime;
        *var_vsbprime_dn7_slot = var_vsbprime_dn7;
        *var_vsbprime_dn8_slot = var_vsbprime_dn8;
        *var_vsbprime_dn9_slot = var_vsbprime_dn9;
        *var_xgb_ov_slot = var_xgb_ov;
        *var_xgb_ov_dn4_slot = var_xgb_ov_dn4;
        *var_xgb_ov_dn6_slot = var_xgb_ov_dn6;
        *var_xgb_ov_dn7_slot = var_xgb_ov_dn7;
        *var_xgb_ov_dn8_slot = var_xgb_ov_dn8;
        *var_xgb_ov_dn9_slot = var_xgb_ov_dn9;
        *var_xgd_ov_slot = var_xgd_ov;
        *var_xgd_ov_dn6_slot = var_xgd_ov_dn6;
        *var_xgd_ov_dn7_slot = var_xgd_ov_dn7;
        *var_xgd_ov_dn8_slot = var_xgd_ov_dn8;
        *var_xgs_ov_slot = var_xgs_ov;
        *var_xgs_ov_dn6_slot = var_xgs_ov_dn6;
        *var_xgs_ov_dn7_slot = var_xgs_ov_dn7;
        *var_xgs_ov_dn8_slot = var_xgs_ov_dn8;
    }

    pub(super) fn stamp_transient_block_87(
        p: &Parameters,
        var_aphi_dc: f64,
        var_aphi_dc_dn4: f64,
        var_ar: f64,
        var_bphi_dc: f64,
        var_bphi_dc_dn4: f64,
        var_ctb_i: f64,
        var_ctg_i: f64,
        var_g_0_dc: f64,
        var_g_0_dc_dn4: f64,
        var_gfacnud_i: f64,
        var_guard1029: f64,
        var_inv_phit: f64,
        var_inv_phit_dn4: f64,
        var_phib_dc: f64,
        var_phib_dc_dn4: f64,
        var_phix1_dc: f64,
        var_phix1_dc_dn4: f64,
        var_phix_dc: f64,
        var_phix_dc_dn4: f64,
        var_sqrt_phib_dc: f64,
        var_sqrt_phib_dc_dn4: f64,
        var_thesat_t: f64,
        var_thesat_t_dn4: f64,
        var_us1: f64,
        var_us1_dn4: f64,
        var_us21: f64,
        var_us21_dn4: f64,
        var_v_sb: f64,
        var_v_sb_dn7: f64,
        var_v_sb_dn8: f64,
        var_v_sb_dn9: f64,
        var_vfb_t: f64,
        var_vfb_t_dn4: f64,
        var_vgb: f64,
        var_vgb_dn6: f64,
        var_vgb_dn7: f64,
        var_vgb_dn8: f64,
        var_vgb_dn9: f64,
        var_aphi_slot: &mut f64,
        var_aphi_dn4_slot: &mut f64,
        var_arloc_slot: &mut f64,
        var_dctg_slot: &mut f64,
        var_dctg_dn4_slot: &mut f64,
        var_dctg_dn6_slot: &mut f64,
        var_dctg_dn7_slot: &mut f64,
        var_dctg_dn8_slot: &mut f64,
        var_dctg_dn9_slot: &mut f64,
        var_dvbstar_slot: &mut f64,
        var_dvbstar_dc_slot: &mut f64,
        var_dvbstar_dc_dn4_slot: &mut f64,
        var_dvbstar_dc_dn6_slot: &mut f64,
        var_dvbstar_dc_dn7_slot: &mut f64,
        var_dvbstar_dc_dn8_slot: &mut f64,
        var_dvbstar_dc_dn9_slot: &mut f64,
        var_dvbstar_dn4_slot: &mut f64,
        var_dvbstar_dn6_slot: &mut f64,
        var_dvbstar_dn7_slot: &mut f64,
        var_dvbstar_dn8_slot: &mut f64,
        var_dvbstar_dn9_slot: &mut f64,
        var_g_0_slot: &mut f64,
        var_g_0_dn4_slot: &mut f64,
        var_guard1189_slot: &mut f64,
        var_guard1190_slot: &mut f64,
        var_phib_slot: &mut f64,
        var_phib_dn4_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp__blk949_slot: &mut f64,
        var_temp__blk949_dn4_slot: &mut f64,
        var_temp__blk949_dn6_slot: &mut f64,
        var_temp__blk949_dn7_slot: &mut f64,
        var_temp__blk949_dn8_slot: &mut f64,
        var_temp__blk949_dn9_slot: &mut f64,
        var_thesatloc_slot: &mut f64,
        var_thesatloc_dn4_slot: &mut f64,
        var_us_slot: &mut f64,
        var_us_dn4_slot: &mut f64,
        var_us_dn6_slot: &mut f64,
        var_us_dn7_slot: &mut f64,
        var_us_dn8_slot: &mut f64,
        var_us_dn9_slot: &mut f64,
        var_usnew_slot: &mut f64,
        var_usnew_dn4_slot: &mut f64,
        var_usnew_dn6_slot: &mut f64,
        var_usnew_dn7_slot: &mut f64,
        var_usnew_dn8_slot: &mut f64,
        var_usnew_dn9_slot: &mut f64,
        var_v_db_slot: &mut f64,
        var_v_db_dn7_slot: &mut f64,
        var_v_db_dn8_slot: &mut f64,
        var_v_db_dn9_slot: &mut f64,
        var_v_ds_slot: &mut f64,
        var_v_ds_dn7_slot: &mut f64,
        var_v_ds_dn8_slot: &mut f64,
        var_v_xb_slot: &mut f64,
        var_v_xb_dc_tmp_slot: &mut f64,
        var_v_xb_dc_tmp_dn4_slot: &mut f64,
        var_v_xb_dc_tmp_dn7_slot: &mut f64,
        var_v_xb_dc_tmp_dn8_slot: &mut f64,
        var_v_xb_dc_tmp_dn9_slot: &mut f64,
        var_v_xb_dn4_slot: &mut f64,
        var_v_xb_dn7_slot: &mut f64,
        var_v_xb_dn8_slot: &mut f64,
        var_v_xb_dn9_slot: &mut f64,
        var_vdsx_slot: &mut f64,
        var_vdsx_dn7_slot: &mut f64,
        var_vdsx_dn8_slot: &mut f64,
        var_vgb1_slot: &mut f64,
        var_vgb1_dn4_slot: &mut f64,
        var_vgb1_dn6_slot: &mut f64,
        var_vgb1_dn7_slot: &mut f64,
        var_vgb1_dn8_slot: &mut f64,
        var_vgb1_dn9_slot: &mut f64,
        var_vmb_slot: &mut f64,
        var_vmb_dn4_slot: &mut f64,
        var_vmb_dn6_slot: &mut f64,
        var_vmb_dn7_slot: &mut f64,
        var_vmb_dn8_slot: &mut f64,
        var_vmb_dn9_slot: &mut f64,
        var_vmbnew_slot: &mut f64,
        var_vmbnew_dn4_slot: &mut f64,
        var_vmbnew_dn6_slot: &mut f64,
        var_vmbnew_dn7_slot: &mut f64,
        var_vmbnew_dn8_slot: &mut f64,
        var_vmbnew_dn9_slot: &mut f64,
        var_vsbstar_slot: &mut f64,
        var_vsbstar_dc_slot: &mut f64,
        var_vsbstar_dc_dn4_slot: &mut f64,
        var_vsbstar_dc_dn6_slot: &mut f64,
        var_vsbstar_dc_dn7_slot: &mut f64,
        var_vsbstar_dc_dn8_slot: &mut f64,
        var_vsbstar_dc_dn9_slot: &mut f64,
        var_vsbstar_dc_tmp_slot: &mut f64,
        var_vsbstar_dc_tmp_dn4_slot: &mut f64,
        var_vsbstar_dc_tmp_dn6_slot: &mut f64,
        var_vsbstar_dc_tmp_dn7_slot: &mut f64,
        var_vsbstar_dc_tmp_dn8_slot: &mut f64,
        var_vsbstar_dc_tmp_dn9_slot: &mut f64,
        var_vsbstar_dn4_slot: &mut f64,
        var_vsbstar_dn6_slot: &mut f64,
        var_vsbstar_dn7_slot: &mut f64,
        var_vsbstar_dn8_slot: &mut f64,
        var_vsbstar_dn9_slot: &mut f64,
        var_vsbx_slot: &mut f64,
        var_vsbx_dn4_slot: &mut f64,
        var_vsbx_dn6_slot: &mut f64,
        var_vsbx_dn7_slot: &mut f64,
        var_vsbx_dn8_slot: &mut f64,
        var_vsbx_dn9_slot: &mut f64,
        var_xbct_slot: &mut f64,
        var_xbct_dn4_slot: &mut f64,
        var_xctmax_slot: &mut f64,
        var_xctmax_dn4_slot: &mut f64,
        var_xgct_slot: &mut f64,
        var_xgct_dn4_slot: &mut f64,
        var_xgct_dn6_slot: &mut f64,
        var_xgct_dn7_slot: &mut f64,
        var_xgct_dn8_slot: &mut f64,
        var_xgct_dn9_slot: &mut f64,
        var_xmict_slot: &mut f64,
        var_xmict_dn4_slot: &mut f64,
        var_xmict_dn6_slot: &mut f64,
        var_xmict_dn7_slot: &mut f64,
        var_xmict_dn8_slot: &mut f64,
        var_xmict_dn9_slot: &mut f64,
        var_xnct_slot: &mut f64,
        var_xnct_dn4_slot: &mut f64,
        var_xnct_dn6_slot: &mut f64,
        var_xnct_dn7_slot: &mut f64,
        var_xnct_dn8_slot: &mut f64,
        var_xnct_dn9_slot: &mut f64,
        var_xsbstar_slot: &mut f64,
        var_xsbstar_dn4_slot: &mut f64,
        var_xsbstar_dn6_slot: &mut f64,
        var_xsbstar_dn7_slot: &mut f64,
        var_xsbstar_dn8_slot: &mut f64,
        var_xsbstar_dn9_slot: &mut f64,
        var_xsubct_slot: &mut f64,
        var_xsubct_dn4_slot: &mut f64,
        var_xsubct_dn6_slot: &mut f64,
        var_xsubct_dn7_slot: &mut f64,
        var_xsubct_dn8_slot: &mut f64,
        var_xsubct_dn9_slot: &mut f64,
        var_xwict_slot: &mut f64,
        var_xwict_dn4_slot: &mut f64,
        var_xwict_dn6_slot: &mut f64,
        var_xwict_dn7_slot: &mut f64,
        var_xwict_dn8_slot: &mut f64,
        var_xwict_dn9_slot: &mut f64,
    ) {
        let mut var_aphi: f64 = *var_aphi_slot;
        let mut var_aphi_dn4: f64 = *var_aphi_dn4_slot;
        let mut var_arloc: f64 = *var_arloc_slot;
        let mut var_dctg: f64 = *var_dctg_slot;
        let mut var_dctg_dn4: f64 = *var_dctg_dn4_slot;
        let mut var_dctg_dn6: f64 = *var_dctg_dn6_slot;
        let mut var_dctg_dn7: f64 = *var_dctg_dn7_slot;
        let mut var_dctg_dn8: f64 = *var_dctg_dn8_slot;
        let mut var_dctg_dn9: f64 = *var_dctg_dn9_slot;
        let mut var_dvbstar: f64 = *var_dvbstar_slot;
        let mut var_dvbstar_dc: f64 = *var_dvbstar_dc_slot;
        let mut var_dvbstar_dc_dn4: f64 = *var_dvbstar_dc_dn4_slot;
        let mut var_dvbstar_dc_dn6: f64 = *var_dvbstar_dc_dn6_slot;
        let mut var_dvbstar_dc_dn7: f64 = *var_dvbstar_dc_dn7_slot;
        let mut var_dvbstar_dc_dn8: f64 = *var_dvbstar_dc_dn8_slot;
        let mut var_dvbstar_dc_dn9: f64 = *var_dvbstar_dc_dn9_slot;
        let mut var_dvbstar_dn4: f64 = *var_dvbstar_dn4_slot;
        let mut var_dvbstar_dn6: f64 = *var_dvbstar_dn6_slot;
        let mut var_dvbstar_dn7: f64 = *var_dvbstar_dn7_slot;
        let mut var_dvbstar_dn8: f64 = *var_dvbstar_dn8_slot;
        let mut var_dvbstar_dn9: f64 = *var_dvbstar_dn9_slot;
        let mut var_g_0: f64 = *var_g_0_slot;
        let mut var_g_0_dn4: f64 = *var_g_0_dn4_slot;
        let mut var_guard1189: f64 = *var_guard1189_slot;
        let mut var_guard1190: f64 = *var_guard1190_slot;
        let mut var_phib: f64 = *var_phib_slot;
        let mut var_phib_dn4: f64 = *var_phib_dn4_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp__blk949: f64 = *var_temp__blk949_slot;
        let mut var_temp__blk949_dn4: f64 = *var_temp__blk949_dn4_slot;
        let mut var_temp__blk949_dn6: f64 = *var_temp__blk949_dn6_slot;
        let mut var_temp__blk949_dn7: f64 = *var_temp__blk949_dn7_slot;
        let mut var_temp__blk949_dn8: f64 = *var_temp__blk949_dn8_slot;
        let mut var_temp__blk949_dn9: f64 = *var_temp__blk949_dn9_slot;
        let mut var_thesatloc: f64 = *var_thesatloc_slot;
        let mut var_thesatloc_dn4: f64 = *var_thesatloc_dn4_slot;
        let mut var_us: f64 = *var_us_slot;
        let mut var_us_dn4: f64 = *var_us_dn4_slot;
        let mut var_us_dn6: f64 = *var_us_dn6_slot;
        let mut var_us_dn7: f64 = *var_us_dn7_slot;
        let mut var_us_dn8: f64 = *var_us_dn8_slot;
        let mut var_us_dn9: f64 = *var_us_dn9_slot;
        let mut var_usnew: f64 = *var_usnew_slot;
        let mut var_usnew_dn4: f64 = *var_usnew_dn4_slot;
        let mut var_usnew_dn6: f64 = *var_usnew_dn6_slot;
        let mut var_usnew_dn7: f64 = *var_usnew_dn7_slot;
        let mut var_usnew_dn8: f64 = *var_usnew_dn8_slot;
        let mut var_usnew_dn9: f64 = *var_usnew_dn9_slot;
        let mut var_v_db: f64 = *var_v_db_slot;
        let mut var_v_db_dn7: f64 = *var_v_db_dn7_slot;
        let mut var_v_db_dn8: f64 = *var_v_db_dn8_slot;
        let mut var_v_db_dn9: f64 = *var_v_db_dn9_slot;
        let mut var_v_ds: f64 = *var_v_ds_slot;
        let mut var_v_ds_dn7: f64 = *var_v_ds_dn7_slot;
        let mut var_v_ds_dn8: f64 = *var_v_ds_dn8_slot;
        let mut var_v_xb: f64 = *var_v_xb_slot;
        let mut var_v_xb_dc_tmp: f64 = *var_v_xb_dc_tmp_slot;
        let mut var_v_xb_dc_tmp_dn4: f64 = *var_v_xb_dc_tmp_dn4_slot;
        let mut var_v_xb_dc_tmp_dn7: f64 = *var_v_xb_dc_tmp_dn7_slot;
        let mut var_v_xb_dc_tmp_dn8: f64 = *var_v_xb_dc_tmp_dn8_slot;
        let mut var_v_xb_dc_tmp_dn9: f64 = *var_v_xb_dc_tmp_dn9_slot;
        let mut var_v_xb_dn4: f64 = *var_v_xb_dn4_slot;
        let mut var_v_xb_dn7: f64 = *var_v_xb_dn7_slot;
        let mut var_v_xb_dn8: f64 = *var_v_xb_dn8_slot;
        let mut var_v_xb_dn9: f64 = *var_v_xb_dn9_slot;
        let mut var_vdsx: f64 = *var_vdsx_slot;
        let mut var_vdsx_dn7: f64 = *var_vdsx_dn7_slot;
        let mut var_vdsx_dn8: f64 = *var_vdsx_dn8_slot;
        let mut var_vgb1: f64 = *var_vgb1_slot;
        let mut var_vgb1_dn4: f64 = *var_vgb1_dn4_slot;
        let mut var_vgb1_dn6: f64 = *var_vgb1_dn6_slot;
        let mut var_vgb1_dn7: f64 = *var_vgb1_dn7_slot;
        let mut var_vgb1_dn8: f64 = *var_vgb1_dn8_slot;
        let mut var_vgb1_dn9: f64 = *var_vgb1_dn9_slot;
        let mut var_vmb: f64 = *var_vmb_slot;
        let mut var_vmb_dn4: f64 = *var_vmb_dn4_slot;
        let mut var_vmb_dn6: f64 = *var_vmb_dn6_slot;
        let mut var_vmb_dn7: f64 = *var_vmb_dn7_slot;
        let mut var_vmb_dn8: f64 = *var_vmb_dn8_slot;
        let mut var_vmb_dn9: f64 = *var_vmb_dn9_slot;
        let mut var_vmbnew: f64 = *var_vmbnew_slot;
        let mut var_vmbnew_dn4: f64 = *var_vmbnew_dn4_slot;
        let mut var_vmbnew_dn6: f64 = *var_vmbnew_dn6_slot;
        let mut var_vmbnew_dn7: f64 = *var_vmbnew_dn7_slot;
        let mut var_vmbnew_dn8: f64 = *var_vmbnew_dn8_slot;
        let mut var_vmbnew_dn9: f64 = *var_vmbnew_dn9_slot;
        let mut var_vsbstar: f64 = *var_vsbstar_slot;
        let mut var_vsbstar_dc: f64 = *var_vsbstar_dc_slot;
        let mut var_vsbstar_dc_dn4: f64 = *var_vsbstar_dc_dn4_slot;
        let mut var_vsbstar_dc_dn6: f64 = *var_vsbstar_dc_dn6_slot;
        let mut var_vsbstar_dc_dn7: f64 = *var_vsbstar_dc_dn7_slot;
        let mut var_vsbstar_dc_dn8: f64 = *var_vsbstar_dc_dn8_slot;
        let mut var_vsbstar_dc_dn9: f64 = *var_vsbstar_dc_dn9_slot;
        let mut var_vsbstar_dc_tmp: f64 = *var_vsbstar_dc_tmp_slot;
        let mut var_vsbstar_dc_tmp_dn4: f64 = *var_vsbstar_dc_tmp_dn4_slot;
        let mut var_vsbstar_dc_tmp_dn6: f64 = *var_vsbstar_dc_tmp_dn6_slot;
        let mut var_vsbstar_dc_tmp_dn7: f64 = *var_vsbstar_dc_tmp_dn7_slot;
        let mut var_vsbstar_dc_tmp_dn8: f64 = *var_vsbstar_dc_tmp_dn8_slot;
        let mut var_vsbstar_dc_tmp_dn9: f64 = *var_vsbstar_dc_tmp_dn9_slot;
        let mut var_vsbstar_dn4: f64 = *var_vsbstar_dn4_slot;
        let mut var_vsbstar_dn6: f64 = *var_vsbstar_dn6_slot;
        let mut var_vsbstar_dn7: f64 = *var_vsbstar_dn7_slot;
        let mut var_vsbstar_dn8: f64 = *var_vsbstar_dn8_slot;
        let mut var_vsbstar_dn9: f64 = *var_vsbstar_dn9_slot;
        let mut var_vsbx: f64 = *var_vsbx_slot;
        let mut var_vsbx_dn4: f64 = *var_vsbx_dn4_slot;
        let mut var_vsbx_dn6: f64 = *var_vsbx_dn6_slot;
        let mut var_vsbx_dn7: f64 = *var_vsbx_dn7_slot;
        let mut var_vsbx_dn8: f64 = *var_vsbx_dn8_slot;
        let mut var_vsbx_dn9: f64 = *var_vsbx_dn9_slot;
        let mut var_xbct: f64 = *var_xbct_slot;
        let mut var_xbct_dn4: f64 = *var_xbct_dn4_slot;
        let mut var_xctmax: f64 = *var_xctmax_slot;
        let mut var_xctmax_dn4: f64 = *var_xctmax_dn4_slot;
        let mut var_xgct: f64 = *var_xgct_slot;
        let mut var_xgct_dn4: f64 = *var_xgct_dn4_slot;
        let mut var_xgct_dn6: f64 = *var_xgct_dn6_slot;
        let mut var_xgct_dn7: f64 = *var_xgct_dn7_slot;
        let mut var_xgct_dn8: f64 = *var_xgct_dn8_slot;
        let mut var_xgct_dn9: f64 = *var_xgct_dn9_slot;
        let mut var_xmict: f64 = *var_xmict_slot;
        let mut var_xmict_dn4: f64 = *var_xmict_dn4_slot;
        let mut var_xmict_dn6: f64 = *var_xmict_dn6_slot;
        let mut var_xmict_dn7: f64 = *var_xmict_dn7_slot;
        let mut var_xmict_dn8: f64 = *var_xmict_dn8_slot;
        let mut var_xmict_dn9: f64 = *var_xmict_dn9_slot;
        let mut var_xnct: f64 = *var_xnct_slot;
        let mut var_xnct_dn4: f64 = *var_xnct_dn4_slot;
        let mut var_xnct_dn6: f64 = *var_xnct_dn6_slot;
        let mut var_xnct_dn7: f64 = *var_xnct_dn7_slot;
        let mut var_xnct_dn8: f64 = *var_xnct_dn8_slot;
        let mut var_xnct_dn9: f64 = *var_xnct_dn9_slot;
        let mut var_xsbstar: f64 = *var_xsbstar_slot;
        let mut var_xsbstar_dn4: f64 = *var_xsbstar_dn4_slot;
        let mut var_xsbstar_dn6: f64 = *var_xsbstar_dn6_slot;
        let mut var_xsbstar_dn7: f64 = *var_xsbstar_dn7_slot;
        let mut var_xsbstar_dn8: f64 = *var_xsbstar_dn8_slot;
        let mut var_xsbstar_dn9: f64 = *var_xsbstar_dn9_slot;
        let mut var_xsubct: f64 = *var_xsubct_slot;
        let mut var_xsubct_dn4: f64 = *var_xsubct_dn4_slot;
        let mut var_xsubct_dn6: f64 = *var_xsubct_dn6_slot;
        let mut var_xsubct_dn7: f64 = *var_xsubct_dn7_slot;
        let mut var_xsubct_dn8: f64 = *var_xsubct_dn8_slot;
        let mut var_xsubct_dn9: f64 = *var_xsubct_dn9_slot;
        let mut var_xwict: f64 = *var_xwict_slot;
        let mut var_xwict_dn4: f64 = *var_xwict_dn4_slot;
        let mut var_xwict_dn6: f64 = *var_xwict_dn6_slot;
        let mut var_xwict_dn7: f64 = *var_xwict_dn7_slot;
        let mut var_xwict_dn8: f64 = *var_xwict_dn8_slot;
        let mut var_xwict_dn9: f64 = *var_xwict_dn9_slot;

        let (assign40740_e53783, assign40740_e53783_d_n7, assign40740_e53783_d_n8,) = {
    if (var_guard1029 != 0.0) {
        let assign40740_e53781: f64 = (-var_v_ds);
        (assign40740_e53781, (-var_v_ds_dn7), (-var_v_ds_dn8),)
    } else {
        (var_v_ds, var_v_ds_dn7, var_v_ds_dn8,)
    }
};
        var_v_ds = assign40740_e53783;
        var_v_ds_dn7 = assign40740_e53783_d_n7;
        var_v_ds_dn8 = assign40740_e53783_d_n8;

        let assign40750_e53786: f64 = (var_v_ds + var_v_sb);
        var_v_db = assign40750_e53786;
        var_v_db_dn7 = (var_v_ds_dn7 + var_v_sb_dn7);
        var_v_db_dn8 = (var_v_ds_dn8 + var_v_sb_dn8);
        var_v_db_dn9 = var_v_sb_dn9;

        let assign40760_e53789: f64 = (var_v_ds * var_v_ds);
        let assign40760_e53792: f64 = (var_v_ds * var_v_ds);
        let assign40760_e53794: f64 = (assign40760_e53792 + 0.01);
        let assign40760_e53795: f64 = (assign40760_e53794).sqrt();
        let assign40760_e53797: f64 = (assign40760_e53795 + 0.1);
        let assign40760_e53798: f64 = (assign40760_e53789 / assign40760_e53797);
        var_vdsx = assign40760_e53798;
        var_vdsx_dn7 = (((((var_v_ds_dn7 * var_v_ds) + (var_v_ds * var_v_ds_dn7)) * assign40760_e53797) - (assign40760_e53789 * (((var_v_ds_dn7 * var_v_ds) + (var_v_ds * var_v_ds_dn7)) / (2.0 * assign40760_e53795)))) / (assign40760_e53797 * assign40760_e53797));
        var_vdsx_dn8 = (((((var_v_ds_dn8 * var_v_ds) + (var_v_ds * var_v_ds_dn8)) * assign40760_e53797) - (assign40760_e53789 * (((var_v_ds_dn8 * var_v_ds) + (var_v_ds * var_v_ds_dn8)) / (2.0 * assign40760_e53795)))) / (assign40760_e53797 * assign40760_e53797));

        let assign40770_e53802: f64 = (var_v_db + var_v_sb);
        let assign40770_e53805: f64 = (var_v_db - var_v_sb);
        let assign40770_e53808: f64 = (var_v_db - var_v_sb);
        let assign40770_e53809: f64 = (assign40770_e53805 * assign40770_e53808);
        let assign40770_e53811: f64 = (assign40770_e53809 + var_bphi_dc);
        let assign40770_e53812: f64 = (assign40770_e53811).sqrt();
        let assign40770_e53813: f64 = (assign40770_e53802 - assign40770_e53812);
        let assign40770_e53814: f64 = (0.5 * assign40770_e53813);
        let assign40770_e53816: f64 = (assign40770_e53814 + var_phix_dc);
        var_v_xb = assign40770_e53816;
        var_v_xb_dn4 = ((0.5 * (-(var_bphi_dc_dn4 / (2.0 * assign40770_e53812)))) + var_phix_dc_dn4);
        var_v_xb_dn7 = (0.5 * ((var_v_db_dn7 + var_v_sb_dn7) - ((((var_v_db_dn7 - var_v_sb_dn7) * assign40770_e53808) + (assign40770_e53805 * (var_v_db_dn7 - var_v_sb_dn7))) / (2.0 * assign40770_e53812))));
        var_v_xb_dn8 = (0.5 * ((var_v_db_dn8 + var_v_sb_dn8) - ((((var_v_db_dn8 - var_v_sb_dn8) * assign40770_e53808) + (assign40770_e53805 * (var_v_db_dn8 - var_v_sb_dn8))) / (2.0 * assign40770_e53812))));
        var_v_xb_dn9 = (0.5 * ((var_v_db_dn9 + var_v_sb_dn9) - ((((var_v_db_dn9 - var_v_sb_dn9) * assign40770_e53808) + (assign40770_e53805 * (var_v_db_dn9 - var_v_sb_dn9))) / (2.0 * assign40770_e53812))));

        var_v_xb_dc_tmp = var_v_xb;
        var_v_xb_dc_tmp_dn4 = var_v_xb_dn4;
        var_v_xb_dc_tmp_dn7 = var_v_xb_dn7;
        var_v_xb_dc_tmp_dn8 = var_v_xb_dn8;
        var_v_xb_dc_tmp_dn9 = var_v_xb_dn9;

        let assign40790_e53822: f64 = var_v_xb;
        let assign40790_e53825: f64 = var_v_xb;
        let assign40790_e53828: f64 = var_v_xb;
        let assign40790_e53829: f64 = (assign40790_e53825 * assign40790_e53828);
        let assign40790_e53831: f64 = (assign40790_e53829 + var_aphi_dc);
        let assign40790_e53832: f64 = (assign40790_e53831).sqrt();
        let assign40790_e53833: f64 = (assign40790_e53822 - assign40790_e53832);
        let assign40790_e53834: f64 = (0.5 * assign40790_e53833);
        let assign40790_e53835: f64 = (var_v_sb - assign40790_e53834);
        let assign40790_e53837: f64 = (assign40790_e53835 + var_phix1_dc);
        var_vsbstar_dc = assign40790_e53837;
        var_vsbstar_dc_dn4 = ((-(0.5 * (var_v_xb_dn4 - ((((var_v_xb_dn4 * assign40790_e53828) + (assign40790_e53825 * var_v_xb_dn4)) + var_aphi_dc_dn4) / (2.0 * assign40790_e53832))))) + var_phix1_dc_dn4);
        var_vsbstar_dc_dn6 = 0.0;
        var_vsbstar_dc_dn7 = (var_v_sb_dn7 - (0.5 * (var_v_xb_dn7 - (((var_v_xb_dn7 * assign40790_e53828) + (assign40790_e53825 * var_v_xb_dn7)) / (2.0 * assign40790_e53832)))));
        var_vsbstar_dc_dn8 = (var_v_sb_dn8 - (0.5 * (var_v_xb_dn8 - (((var_v_xb_dn8 * assign40790_e53828) + (assign40790_e53825 * var_v_xb_dn8)) / (2.0 * assign40790_e53832)))));
        var_vsbstar_dc_dn9 = (var_v_sb_dn9 - (0.5 * (var_v_xb_dn9 - (((var_v_xb_dn9 * assign40790_e53828) + (assign40790_e53825 * var_v_xb_dn9)) / (2.0 * assign40790_e53832)))));

        var_vsbstar_dc_tmp = var_vsbstar_dc;
        var_vsbstar_dc_tmp_dn4 = var_vsbstar_dc_dn4;
        var_vsbstar_dc_tmp_dn6 = var_vsbstar_dc_dn6;
        var_vsbstar_dc_tmp_dn7 = var_vsbstar_dc_dn7;
        var_vsbstar_dc_tmp_dn8 = var_vsbstar_dc_dn8;
        var_vsbstar_dc_tmp_dn9 = var_vsbstar_dc_dn9;

        var_dvbstar_dc = 0.0;
        var_dvbstar_dc_dn4 = 0.0;
        var_dvbstar_dc_dn6 = 0.0;
        var_dvbstar_dc_dn7 = 0.0;
        var_dvbstar_dc_dn8 = 0.0;
        var_dvbstar_dc_dn9 = 0.0;

        let assign40820_e53846: f64 = if ((p.p45 != 0.0) && (var_gfacnud_i != 1.0)) { 1.0 } else { 0.0 };
        var_guard1189 = assign40820_e53846;

        let (assign40830_e53856, assign40830_e53856_d_n4, assign40830_e53856_d_n6, assign40830_e53856_d_n7, assign40830_e53856_d_n8, assign40830_e53856_d_n9,) = {
    if (var_guard1189 != 0.0) {
        let assign40830_e53852: f64 = (var_v_ds - var_vdsx);
        let assign40830_e53853: f64 = (0.5 * assign40830_e53852);
        let assign40830_e53854: f64 = (var_vsbstar_dc + assign40830_e53853);
        (assign40830_e53854, var_vsbstar_dc_dn4, var_vsbstar_dc_dn6, (var_vsbstar_dc_dn7 + (0.5 * (var_v_ds_dn7 - var_vdsx_dn7))), (var_vsbstar_dc_dn8 + (0.5 * (var_v_ds_dn8 - var_vdsx_dn8))), var_vsbstar_dc_dn9,)
    } else {
        (var_vmb, var_vmb_dn4, var_vmb_dn6, var_vmb_dn7, var_vmb_dn8, var_vmb_dn9,)
    }
};
        var_vmb = assign40830_e53856;
        var_vmb_dn4 = assign40830_e53856_d_n4;
        var_vmb_dn6 = assign40830_e53856_d_n6;
        var_vmb_dn7 = assign40830_e53856_d_n7;
        var_vmb_dn8 = assign40830_e53856_d_n8;
        var_vmb_dn9 = assign40830_e53856_d_n9;

        let (assign40840_e53865, assign40840_e53865_d_n4, assign40840_e53865_d_n6, assign40840_e53865_d_n7, assign40840_e53865_d_n8, assign40840_e53865_d_n9,) = {
    if (var_guard1189 != 0.0) {
        let assign40840_e53860: f64 = (var_vmb + var_phib_dc);
        let assign40840_e53861: f64 = (assign40840_e53860).sqrt();
        let assign40840_e53863: f64 = (assign40840_e53861 - var_sqrt_phib_dc);
        (assign40840_e53863, (((var_vmb_dn4 + var_phib_dc_dn4) / (2.0 * assign40840_e53861)) - var_sqrt_phib_dc_dn4), (var_vmb_dn6 / (2.0 * assign40840_e53861)), (var_vmb_dn7 / (2.0 * assign40840_e53861)), (var_vmb_dn8 / (2.0 * assign40840_e53861)), (var_vmb_dn9 / (2.0 * assign40840_e53861)),)
    } else {
        (var_us, var_us_dn4, var_us_dn6, var_us_dn7, var_us_dn8, var_us_dn9,)
    }
};
        var_us = assign40840_e53865;
        var_us_dn4 = assign40840_e53865_d_n4;
        var_us_dn6 = assign40840_e53865_d_n6;
        var_us_dn7 = assign40840_e53865_d_n7;
        var_us_dn8 = assign40840_e53865_d_n8;
        var_us_dn9 = assign40840_e53865_d_n9;

        let (assign40850_e53877, assign40850_e53877_d_n4, assign40850_e53877_d_n6, assign40850_e53877_d_n7, assign40850_e53877_d_n8, assign40850_e53877_d_n9,) = {
    if (var_guard1189 != 0.0) {
        let assign40850_e53870: f64 = (var_us - var_us1);
        let assign40850_e53871: f64 = (2.0 * assign40850_e53870);
        let assign40850_e53873: f64 = (assign40850_e53871 / var_us21);
        let assign40850_e53875: f64 = (assign40850_e53873 - 1.0);
        (assign40850_e53875, ((((2.0 * (var_us_dn4 - var_us1_dn4)) * var_us21) - (assign40850_e53871 * var_us21_dn4)) / (var_us21 * var_us21)), ((2.0 * var_us_dn6) / var_us21), ((2.0 * var_us_dn7) / var_us21), ((2.0 * var_us_dn8) / var_us21), ((2.0 * var_us_dn9) / var_us21),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign40850_e53877;
        var_temp__blk949_dn4 = assign40850_e53877_d_n4;
        var_temp__blk949_dn6 = assign40850_e53877_d_n6;
        var_temp__blk949_dn7 = assign40850_e53877_d_n7;
        var_temp__blk949_dn8 = assign40850_e53877_d_n8;
        var_temp__blk949_dn9 = assign40850_e53877_d_n9;

        let (assign40860_e53898, assign40860_e53898_d_n4, assign40860_e53898_d_n6, assign40860_e53898_d_n7, assign40860_e53898_d_n8, assign40860_e53898_d_n9,) = {
    if (var_guard1189 != 0.0) {
        let assign40860_e53883: f64 = (1.0 - var_gfacnud_i);
        let assign40860_e53884: f64 = (0.25 * assign40860_e53883);
        let assign40860_e53886: f64 = (assign40860_e53884 * var_us21);
        let assign40860_e53890: f64 = (var_temp__blk949 * var_temp__blk949);
        let assign40860_e53892: f64 = (assign40860_e53890 + 0.4804530139182);
        let assign40860_e53893: f64 = (assign40860_e53892).sqrt();
        let assign40860_e53894: f64 = (var_temp__blk949 + assign40860_e53893);
        let assign40860_e53895: f64 = (assign40860_e53886 * assign40860_e53894);
        let assign40860_e53896: f64 = (var_us - assign40860_e53895);
        (assign40860_e53896, (var_us_dn4 - (((assign40860_e53884 * var_us21_dn4) * assign40860_e53894) + (assign40860_e53886 * (var_temp__blk949_dn4 + (((var_temp__blk949_dn4 * var_temp__blk949) + (var_temp__blk949 * var_temp__blk949_dn4)) / (2.0 * assign40860_e53893)))))), (var_us_dn6 - (assign40860_e53886 * (var_temp__blk949_dn6 + (((var_temp__blk949_dn6 * var_temp__blk949) + (var_temp__blk949 * var_temp__blk949_dn6)) / (2.0 * assign40860_e53893))))), (var_us_dn7 - (assign40860_e53886 * (var_temp__blk949_dn7 + (((var_temp__blk949_dn7 * var_temp__blk949) + (var_temp__blk949 * var_temp__blk949_dn7)) / (2.0 * assign40860_e53893))))), (var_us_dn8 - (assign40860_e53886 * (var_temp__blk949_dn8 + (((var_temp__blk949_dn8 * var_temp__blk949) + (var_temp__blk949 * var_temp__blk949_dn8)) / (2.0 * assign40860_e53893))))), (var_us_dn9 - (assign40860_e53886 * (var_temp__blk949_dn9 + (((var_temp__blk949_dn9 * var_temp__blk949) + (var_temp__blk949 * var_temp__blk949_dn9)) / (2.0 * assign40860_e53893))))),)
    } else {
        (var_usnew, var_usnew_dn4, var_usnew_dn6, var_usnew_dn7, var_usnew_dn8, var_usnew_dn9,)
    }
};
        var_usnew = assign40860_e53898;
        var_usnew_dn4 = assign40860_e53898_d_n4;
        var_usnew_dn6 = assign40860_e53898_d_n6;
        var_usnew_dn7 = assign40860_e53898_d_n7;
        var_usnew_dn8 = assign40860_e53898_d_n8;
        var_usnew_dn9 = assign40860_e53898_d_n9;

        let (assign40870_e53910, assign40870_e53910_d_n4, assign40870_e53910_d_n6, assign40870_e53910_d_n7, assign40870_e53910_d_n8, assign40870_e53910_d_n9,) = {
    if (var_guard1189 != 0.0) {
        let assign40870_e53902: f64 = (var_usnew * var_usnew);
        let assign40870_e53905: f64 = (2.0 * var_sqrt_phib_dc);
        let assign40870_e53907: f64 = (assign40870_e53905 * var_usnew);
        let assign40870_e53908: f64 = (assign40870_e53902 + assign40870_e53907);
        (assign40870_e53908, (((var_usnew_dn4 * var_usnew) + (var_usnew * var_usnew_dn4)) + (((2.0 * var_sqrt_phib_dc_dn4) * var_usnew) + (assign40870_e53905 * var_usnew_dn4))), (((var_usnew_dn6 * var_usnew) + (var_usnew * var_usnew_dn6)) + (assign40870_e53905 * var_usnew_dn6)), (((var_usnew_dn7 * var_usnew) + (var_usnew * var_usnew_dn7)) + (assign40870_e53905 * var_usnew_dn7)), (((var_usnew_dn8 * var_usnew) + (var_usnew * var_usnew_dn8)) + (assign40870_e53905 * var_usnew_dn8)), (((var_usnew_dn9 * var_usnew) + (var_usnew * var_usnew_dn9)) + (assign40870_e53905 * var_usnew_dn9)),)
    } else {
        (var_vmbnew, var_vmbnew_dn4, var_vmbnew_dn6, var_vmbnew_dn7, var_vmbnew_dn8, var_vmbnew_dn9,)
    }
};
        var_vmbnew = assign40870_e53910;
        var_vmbnew_dn4 = assign40870_e53910_d_n4;
        var_vmbnew_dn6 = assign40870_e53910_d_n6;
        var_vmbnew_dn7 = assign40870_e53910_d_n7;
        var_vmbnew_dn8 = assign40870_e53910_d_n8;
        var_vmbnew_dn9 = assign40870_e53910_d_n9;

        let (assign40880_e53920, assign40880_e53920_d_n4, assign40880_e53920_d_n6, assign40880_e53920_d_n7, assign40880_e53920_d_n8, assign40880_e53920_d_n9,) = {
    if (var_guard1189 != 0.0) {
        let assign40880_e53916: f64 = (var_v_ds - var_vdsx);
        let assign40880_e53917: f64 = (0.5 * assign40880_e53916);
        let assign40880_e53918: f64 = (var_vmbnew - assign40880_e53917);
        (assign40880_e53918, var_vmbnew_dn4, var_vmbnew_dn6, (var_vmbnew_dn7 - (0.5 * (var_v_ds_dn7 - var_vdsx_dn7))), (var_vmbnew_dn8 - (0.5 * (var_v_ds_dn8 - var_vdsx_dn8))), var_vmbnew_dn9,)
    } else {
        (var_vsbstar_dc, var_vsbstar_dc_dn4, var_vsbstar_dc_dn6, var_vsbstar_dc_dn7, var_vsbstar_dc_dn8, var_vsbstar_dc_dn9,)
    }
};
        var_vsbstar_dc = assign40880_e53920;
        var_vsbstar_dc_dn4 = assign40880_e53920_d_n4;
        var_vsbstar_dc_dn6 = assign40880_e53920_d_n6;
        var_vsbstar_dc_dn7 = assign40880_e53920_d_n7;
        var_vsbstar_dc_dn8 = assign40880_e53920_d_n8;
        var_vsbstar_dc_dn9 = assign40880_e53920_d_n9;

        let (assign40890_e53926, assign40890_e53926_d_n4, assign40890_e53926_d_n6, assign40890_e53926_d_n7, assign40890_e53926_d_n8, assign40890_e53926_d_n9,) = {
    if (var_guard1189 != 0.0) {
        let assign40890_e53924: f64 = (var_vsbstar_dc_tmp - var_vsbstar_dc);
        (assign40890_e53924, (var_vsbstar_dc_tmp_dn4 - var_vsbstar_dc_dn4), (var_vsbstar_dc_tmp_dn6 - var_vsbstar_dc_dn6), (var_vsbstar_dc_tmp_dn7 - var_vsbstar_dc_dn7), (var_vsbstar_dc_tmp_dn8 - var_vsbstar_dc_dn8), (var_vsbstar_dc_tmp_dn9 - var_vsbstar_dc_dn9),)
    } else {
        (var_dvbstar_dc, var_dvbstar_dc_dn4, var_dvbstar_dc_dn6, var_dvbstar_dc_dn7, var_dvbstar_dc_dn8, var_dvbstar_dc_dn9,)
    }
};
        var_dvbstar_dc = assign40890_e53926;
        var_dvbstar_dc_dn4 = assign40890_e53926_d_n4;
        var_dvbstar_dc_dn6 = assign40890_e53926_d_n6;
        var_dvbstar_dc_dn7 = assign40890_e53926_d_n7;
        var_dvbstar_dc_dn8 = assign40890_e53926_d_n8;
        var_dvbstar_dc_dn9 = assign40890_e53926_d_n9;

        var_phib = var_phib_dc;
        var_phib_dn4 = var_phib_dc_dn4;

        var_aphi = var_aphi_dc;
        var_aphi_dn4 = var_aphi_dc_dn4;

        var_g_0 = var_g_0_dc;
        var_g_0_dn4 = var_g_0_dc_dn4;

        var_vsbstar = var_vsbstar_dc;
        var_vsbstar_dn4 = var_vsbstar_dc_dn4;
        var_vsbstar_dn6 = var_vsbstar_dc_dn6;
        var_vsbstar_dn7 = var_vsbstar_dc_dn7;
        var_vsbstar_dn8 = var_vsbstar_dc_dn8;
        var_vsbstar_dn9 = var_vsbstar_dc_dn9;

        var_dvbstar = var_dvbstar_dc;
        var_dvbstar_dn4 = var_dvbstar_dc_dn4;
        var_dvbstar_dn6 = var_dvbstar_dc_dn6;
        var_dvbstar_dn7 = var_dvbstar_dc_dn7;
        var_dvbstar_dn8 = var_dvbstar_dc_dn8;
        var_dvbstar_dn9 = var_dvbstar_dc_dn9;

        var_thesatloc = var_thesat_t;
        var_thesatloc_dn4 = var_thesat_t_dn4;

        var_arloc = var_ar;

        let assign40970_e53936: f64 = (var_vgb - var_dvbstar);
        let assign40970_e53938: f64 = (assign40970_e53936 - var_vfb_t);
        var_vgb1 = assign40970_e53938;
        var_vgb1_dn4 = ((-var_dvbstar_dn4) - var_vfb_t_dn4);
        var_vgb1_dn6 = (var_vgb_dn6 - var_dvbstar_dn6);
        var_vgb1_dn7 = (var_vgb_dn7 - var_dvbstar_dn7);
        var_vgb1_dn8 = (var_vgb_dn8 - var_dvbstar_dn8);
        var_vgb1_dn9 = (var_vgb_dn9 - var_dvbstar_dn9);

        let assign40980_e53943: f64 = (var_v_ds - var_vdsx);
        let assign40980_e53944: f64 = (0.5 * assign40980_e53943);
        let assign40980_e53945: f64 = (var_vsbstar + assign40980_e53944);
        var_vsbx = assign40980_e53945;
        var_vsbx_dn4 = var_vsbstar_dn4;
        var_vsbx_dn6 = var_vsbstar_dn6;
        var_vsbx_dn7 = (var_vsbstar_dn7 + (0.5 * (var_v_ds_dn7 - var_vdsx_dn7)));
        var_vsbx_dn8 = (var_vsbstar_dn8 + (0.5 * (var_v_ds_dn8 - var_vdsx_dn8)));
        var_vsbx_dn9 = var_vsbstar_dn9;

        var_dctg = 1.0;
        var_dctg_dn4 = 0.0;
        var_dctg_dn6 = 0.0;
        var_dctg_dn7 = 0.0;
        var_dctg_dn8 = 0.0;
        var_dctg_dn9 = 0.0;

        let assign41000_e53949: f64 = if var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1190 = assign41000_e53949;

        let (assign41010_e53955, assign41010_e53955_d_n4,) = {
    if (var_guard1190 != 0.0) {
        let assign41010_e53953: f64 = (var_phib * var_inv_phit);
        (assign41010_e53953, ((var_phib_dn4 * var_inv_phit) + (var_phib * var_inv_phit_dn4)),)
    } else {
        (var_xbct, var_xbct_dn4,)
    }
};
        var_xbct = assign41010_e53955;
        var_xbct_dn4 = assign41010_e53955_d_n4;

        let (assign41020_e53961, assign41020_e53961_d_n4, assign41020_e53961_d_n6, assign41020_e53961_d_n7, assign41020_e53961_d_n8, assign41020_e53961_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41020_e53959: f64 = (var_vsbx * var_inv_phit);
        (assign41020_e53959, ((var_vsbx_dn4 * var_inv_phit) + (var_vsbx * var_inv_phit_dn4)), (var_vsbx_dn6 * var_inv_phit), (var_vsbx_dn7 * var_inv_phit), (var_vsbx_dn8 * var_inv_phit), (var_vsbx_dn9 * var_inv_phit),)
    } else {
        (var_xsbstar, var_xsbstar_dn4, var_xsbstar_dn6, var_xsbstar_dn7, var_xsbstar_dn8, var_xsbstar_dn9,)
    }
};
        var_xsbstar = assign41020_e53961;
        var_xsbstar_dn4 = assign41020_e53961_d_n4;
        var_xsbstar_dn6 = assign41020_e53961_d_n6;
        var_xsbstar_dn7 = assign41020_e53961_d_n7;
        var_xsbstar_dn8 = assign41020_e53961_d_n8;
        var_xsbstar_dn9 = assign41020_e53961_d_n9;

        let (assign41030_e53967, assign41030_e53967_d_n4, assign41030_e53967_d_n6, assign41030_e53967_d_n7, assign41030_e53967_d_n8, assign41030_e53967_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41030_e53965: f64 = (var_vgb1 * var_inv_phit);
        (assign41030_e53965, ((var_vgb1_dn4 * var_inv_phit) + (var_vgb1 * var_inv_phit_dn4)), (var_vgb1_dn6 * var_inv_phit), (var_vgb1_dn7 * var_inv_phit), (var_vgb1_dn8 * var_inv_phit), (var_vgb1_dn9 * var_inv_phit),)
    } else {
        (var_xgct, var_xgct_dn4, var_xgct_dn6, var_xgct_dn7, var_xgct_dn8, var_xgct_dn9,)
    }
};
        var_xgct = assign41030_e53967;
        var_xgct_dn4 = assign41030_e53967_d_n4;
        var_xgct_dn6 = assign41030_e53967_d_n6;
        var_xgct_dn7 = assign41030_e53967_d_n7;
        var_xgct_dn8 = assign41030_e53967_d_n8;
        var_xgct_dn9 = assign41030_e53967_d_n9;

        let (assign41040_e53978, assign41040_e53978_d_n4, assign41040_e53978_d_n6, assign41040_e53978_d_n7, assign41040_e53978_d_n8, assign41040_e53978_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41040_e53972: f64 = (0.5 * var_g_0);
        let assign41040_e53974: f64 = (var_xbct).sqrt();
        let assign41040_e53975: f64 = (assign41040_e53972 / assign41040_e53974);
        let assign41040_e53976: f64 = (1.0 + assign41040_e53975);
        (assign41040_e53976, ((((0.5 * var_g_0_dn4) * assign41040_e53974) - (assign41040_e53972 * (var_xbct_dn4 / (2.0 * assign41040_e53974)))) / (assign41040_e53974 * assign41040_e53974)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign41040_e53978;
        var_temp1_dn4 = assign41040_e53978_d_n4;
        var_temp1_dn6 = assign41040_e53978_d_n6;
        var_temp1_dn7 = assign41040_e53978_d_n7;
        var_temp1_dn8 = assign41040_e53978_d_n8;
        var_temp1_dn9 = assign41040_e53978_d_n9;

        let (assign41050_e53987, assign41050_e53987_d_n4, assign41050_e53987_d_n6, assign41050_e53987_d_n7, assign41050_e53987_d_n8, assign41050_e53987_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41050_e53983: f64 = (var_xbct).sqrt();
        let assign41050_e53984: f64 = (var_g_0 * assign41050_e53983);
        let assign41050_e53985: f64 = (var_xbct + assign41050_e53984);
        (assign41050_e53985, (var_xbct_dn4 + ((var_g_0_dn4 * assign41050_e53983) + (var_g_0 * (var_xbct_dn4 / (2.0 * assign41050_e53983))))), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign41050_e53987;
        var_temp2_dn4 = assign41050_e53987_d_n4;
        var_temp2_dn6 = assign41050_e53987_d_n6;
        var_temp2_dn7 = assign41050_e53987_d_n7;
        var_temp2_dn8 = assign41050_e53987_d_n8;
        var_temp2_dn9 = assign41050_e53987_d_n9;

        let (assign41060_e54005, assign41060_e54005_d_n4, assign41060_e54005_d_n6, assign41060_e54005_d_n7, assign41060_e54005_d_n8, assign41060_e54005_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41060_e53991: f64 = (var_xgct - var_temp2);
        let assign41060_e53993: f64 = (assign41060_e53991 / var_temp1);
        let assign41060_e53996: f64 = (0.5 * var_xbct);
        let assign41060_e53997: f64 = (assign41060_e53993 + assign41060_e53996);
        let assign41060_e54000: f64 = (1.0 + var_ctb_i);
        let assign41060_e54002: f64 = (assign41060_e54000 * var_xsbstar);
        let assign41060_e54003: f64 = (assign41060_e53997 - assign41060_e54002);
        (assign41060_e54003, ((((((var_xgct_dn4 - var_temp2_dn4) * var_temp1) - (assign41060_e53991 * var_temp1_dn4)) / (var_temp1 * var_temp1)) + (0.5 * var_xbct_dn4)) - (assign41060_e54000 * var_xsbstar_dn4)), (((((var_xgct_dn6 - var_temp2_dn6) * var_temp1) - (assign41060_e53991 * var_temp1_dn6)) / (var_temp1 * var_temp1)) - (assign41060_e54000 * var_xsbstar_dn6)), (((((var_xgct_dn7 - var_temp2_dn7) * var_temp1) - (assign41060_e53991 * var_temp1_dn7)) / (var_temp1 * var_temp1)) - (assign41060_e54000 * var_xsbstar_dn7)), (((((var_xgct_dn8 - var_temp2_dn8) * var_temp1) - (assign41060_e53991 * var_temp1_dn8)) / (var_temp1 * var_temp1)) - (assign41060_e54000 * var_xsbstar_dn8)), (((((var_xgct_dn9 - var_temp2_dn9) * var_temp1) - (assign41060_e53991 * var_temp1_dn9)) / (var_temp1 * var_temp1)) - (assign41060_e54000 * var_xsbstar_dn9)),)
    } else {
        (var_xwict, var_xwict_dn4, var_xwict_dn6, var_xwict_dn7, var_xwict_dn8, var_xwict_dn9,)
    }
};
        var_xwict = assign41060_e54005;
        var_xwict_dn4 = assign41060_e54005_d_n4;
        var_xwict_dn6 = assign41060_e54005_d_n6;
        var_xwict_dn7 = assign41060_e54005_d_n7;
        var_xwict_dn8 = assign41060_e54005_d_n8;
        var_xwict_dn9 = assign41060_e54005_d_n9;

        let (assign41070_e54013, assign41070_e54013_d_n4,) = {
    if (var_guard1190 != 0.0) {
        let assign41070_e54009: f64 = (0.5 * var_xbct);
        let assign41070_e54011: f64 = (assign41070_e54009 + 2.0);
        (assign41070_e54011, (0.5 * var_xbct_dn4),)
    } else {
        (var_xctmax, var_xctmax_dn4,)
    }
};
        var_xctmax = assign41070_e54013;
        var_xctmax_dn4 = assign41070_e54013_d_n4;

        let (assign41080_e54019, assign41080_e54019_d_n4, assign41080_e54019_d_n6, assign41080_e54019_d_n7, assign41080_e54019_d_n8, assign41080_e54019_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41080_e54017: f64 = (var_xbct + var_xsbstar);
        (assign41080_e54017, (var_xbct_dn4 + var_xsbstar_dn4), var_xsbstar_dn6, var_xsbstar_dn7, var_xsbstar_dn8, var_xsbstar_dn9,)
    } else {
        (var_xnct, var_xnct_dn4, var_xnct_dn6, var_xnct_dn7, var_xnct_dn8, var_xnct_dn9,)
    }
};
        var_xnct = assign41080_e54019;
        var_xnct_dn4 = assign41080_e54019_d_n4;
        var_xnct_dn6 = assign41080_e54019_d_n6;
        var_xnct_dn7 = assign41080_e54019_d_n7;
        var_xnct_dn8 = assign41080_e54019_d_n8;
        var_xnct_dn9 = assign41080_e54019_d_n9;

        let (assign41090_e54040, assign41090_e54040_d_n4, assign41090_e54040_d_n6, assign41090_e54040_d_n7, assign41090_e54040_d_n8, assign41090_e54040_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41090_e54023: f64 = (var_xgct - var_xnct);
        let assign41090_e54026: f64 = (var_xnct).sqrt();
        let assign41090_e54027: f64 = (var_g_0 * assign41090_e54026);
        let assign41090_e54028: f64 = (assign41090_e54023 - assign41090_e54027);
        let assign41090_e54032: f64 = (var_xbct / var_g_0);
        let assign41090_e54034: f64 = (var_xbct).sqrt();
        let assign41090_e54035: f64 = (assign41090_e54032 + assign41090_e54034);
        let assign41090_e54036: f64 = (assign41090_e54035).ln();
        let assign41090_e54037: f64 = (2.0 * assign41090_e54036);
        let assign41090_e54038: f64 = (assign41090_e54028 - assign41090_e54037);
        (assign41090_e54038, (((var_xgct_dn4 - var_xnct_dn4) - ((var_g_0_dn4 * assign41090_e54026) + (var_g_0 * (var_xnct_dn4 / (2.0 * assign41090_e54026))))) - (2.0 * (((((var_xbct_dn4 * var_g_0) - (var_xbct * var_g_0_dn4)) / (var_g_0 * var_g_0)) + (var_xbct_dn4 / (2.0 * assign41090_e54034))) / assign41090_e54035))), ((var_xgct_dn6 - var_xnct_dn6) - (var_g_0 * (var_xnct_dn6 / (2.0 * assign41090_e54026)))), ((var_xgct_dn7 - var_xnct_dn7) - (var_g_0 * (var_xnct_dn7 / (2.0 * assign41090_e54026)))), ((var_xgct_dn8 - var_xnct_dn8) - (var_g_0 * (var_xnct_dn8 / (2.0 * assign41090_e54026)))), ((var_xgct_dn9 - var_xnct_dn9) - (var_g_0 * (var_xnct_dn9 / (2.0 * assign41090_e54026)))),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign41090_e54040;
        var_temp1_dn4 = assign41090_e54040_d_n4;
        var_temp1_dn6 = assign41090_e54040_d_n6;
        var_temp1_dn7 = assign41090_e54040_d_n7;
        var_temp1_dn8 = assign41090_e54040_d_n8;
        var_temp1_dn9 = assign41090_e54040_d_n9;

        let (assign41100_e54048, assign41100_e54048_d_n4, assign41100_e54048_d_n6, assign41100_e54048_d_n7, assign41100_e54048_d_n8, assign41100_e54048_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41100_e54044: f64 = (2.0 * var_temp1);
        let assign41100_e54046: f64 = (assign41100_e54044 + var_xctmax);
        (assign41100_e54046, ((2.0 * var_temp1_dn4) + var_xctmax_dn4), (2.0 * var_temp1_dn6), (2.0 * var_temp1_dn7), (2.0 * var_temp1_dn8), (2.0 * var_temp1_dn9),)
    } else {
        (var_xmict, var_xmict_dn4, var_xmict_dn6, var_xmict_dn7, var_xmict_dn8, var_xmict_dn9,)
    }
};
        var_xmict = assign41100_e54048;
        var_xmict_dn4 = assign41100_e54048_d_n4;
        var_xmict_dn6 = assign41100_e54048_d_n6;
        var_xmict_dn7 = assign41100_e54048_d_n7;
        var_xmict_dn8 = assign41100_e54048_d_n8;
        var_xmict_dn9 = assign41100_e54048_d_n9;

        let (assign41110_e54067, assign41110_e54067_d_n4, assign41110_e54067_d_n6, assign41110_e54067_d_n7, assign41110_e54067_d_n8, assign41110_e54067_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41110_e54053: f64 = (var_xwict + var_xmict);
        let assign41110_e54056: f64 = (var_xwict - var_xmict);
        let assign41110_e54059: f64 = (var_xwict - var_xmict);
        let assign41110_e54060: f64 = (assign41110_e54056 * assign41110_e54059);
        let assign41110_e54062: f64 = (assign41110_e54060 + 20.0);
        let assign41110_e54063: f64 = (assign41110_e54062).sqrt();
        let assign41110_e54064: f64 = (assign41110_e54053 + assign41110_e54063);
        let assign41110_e54065: f64 = (0.5 * assign41110_e54064);
        (assign41110_e54065, (0.5 * ((var_xwict_dn4 + var_xmict_dn4) + ((((var_xwict_dn4 - var_xmict_dn4) * assign41110_e54059) + (assign41110_e54056 * (var_xwict_dn4 - var_xmict_dn4))) / (2.0 * assign41110_e54063)))), (0.5 * ((var_xwict_dn6 + var_xmict_dn6) + ((((var_xwict_dn6 - var_xmict_dn6) * assign41110_e54059) + (assign41110_e54056 * (var_xwict_dn6 - var_xmict_dn6))) / (2.0 * assign41110_e54063)))), (0.5 * ((var_xwict_dn7 + var_xmict_dn7) + ((((var_xwict_dn7 - var_xmict_dn7) * assign41110_e54059) + (assign41110_e54056 * (var_xwict_dn7 - var_xmict_dn7))) / (2.0 * assign41110_e54063)))), (0.5 * ((var_xwict_dn8 + var_xmict_dn8) + ((((var_xwict_dn8 - var_xmict_dn8) * assign41110_e54059) + (assign41110_e54056 * (var_xwict_dn8 - var_xmict_dn8))) / (2.0 * assign41110_e54063)))), (0.5 * ((var_xwict_dn9 + var_xmict_dn9) + ((((var_xwict_dn9 - var_xmict_dn9) * assign41110_e54059) + (assign41110_e54056 * (var_xwict_dn9 - var_xmict_dn9))) / (2.0 * assign41110_e54063)))),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign41110_e54067;
        var_temp1_dn4 = assign41110_e54067_d_n4;
        var_temp1_dn6 = assign41110_e54067_d_n6;
        var_temp1_dn7 = assign41110_e54067_d_n7;
        var_temp1_dn8 = assign41110_e54067_d_n8;
        var_temp1_dn9 = assign41110_e54067_d_n9;

        let (assign41120_e54077, assign41120_e54077_d_n4, assign41120_e54077_d_n6, assign41120_e54077_d_n7, assign41120_e54077_d_n8, assign41120_e54077_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41120_e54072: f64 = (var_xgct - var_xsbstar);
        let assign41120_e54073: f64 = (2.0 * assign41120_e54072);
        let assign41120_e54075: f64 = (assign41120_e54073 - var_xctmax);
        (assign41120_e54075, ((2.0 * (var_xgct_dn4 - var_xsbstar_dn4)) - var_xctmax_dn4), (2.0 * (var_xgct_dn6 - var_xsbstar_dn6)), (2.0 * (var_xgct_dn7 - var_xsbstar_dn7)), (2.0 * (var_xgct_dn8 - var_xsbstar_dn8)), (2.0 * (var_xgct_dn9 - var_xsbstar_dn9)),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign41120_e54077;
        var_temp2_dn4 = assign41120_e54077_d_n4;
        var_temp2_dn6 = assign41120_e54077_d_n6;
        var_temp2_dn7 = assign41120_e54077_d_n7;
        var_temp2_dn8 = assign41120_e54077_d_n8;
        var_temp2_dn9 = assign41120_e54077_d_n9;

        let (assign41130_e54096, assign41130_e54096_d_n4, assign41130_e54096_d_n6, assign41130_e54096_d_n7, assign41130_e54096_d_n8, assign41130_e54096_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41130_e54082: f64 = (var_temp1 + var_temp2);
        let assign41130_e54085: f64 = (var_temp1 - var_temp2);
        let assign41130_e54088: f64 = (var_temp1 - var_temp2);
        let assign41130_e54089: f64 = (assign41130_e54085 * assign41130_e54088);
        let assign41130_e54091: f64 = (assign41130_e54089 + 20.0);
        let assign41130_e54092: f64 = (assign41130_e54091).sqrt();
        let assign41130_e54093: f64 = (assign41130_e54082 - assign41130_e54092);
        let assign41130_e54094: f64 = (0.5 * assign41130_e54093);
        (assign41130_e54094, (0.5 * ((var_temp1_dn4 + var_temp2_dn4) - ((((var_temp1_dn4 - var_temp2_dn4) * assign41130_e54088) + (assign41130_e54085 * (var_temp1_dn4 - var_temp2_dn4))) / (2.0 * assign41130_e54092)))), (0.5 * ((var_temp1_dn6 + var_temp2_dn6) - ((((var_temp1_dn6 - var_temp2_dn6) * assign41130_e54088) + (assign41130_e54085 * (var_temp1_dn6 - var_temp2_dn6))) / (2.0 * assign41130_e54092)))), (0.5 * ((var_temp1_dn7 + var_temp2_dn7) - ((((var_temp1_dn7 - var_temp2_dn7) * assign41130_e54088) + (assign41130_e54085 * (var_temp1_dn7 - var_temp2_dn7))) / (2.0 * assign41130_e54092)))), (0.5 * ((var_temp1_dn8 + var_temp2_dn8) - ((((var_temp1_dn8 - var_temp2_dn8) * assign41130_e54088) + (assign41130_e54085 * (var_temp1_dn8 - var_temp2_dn8))) / (2.0 * assign41130_e54092)))), (0.5 * ((var_temp1_dn9 + var_temp2_dn9) - ((((var_temp1_dn9 - var_temp2_dn9) * assign41130_e54088) + (assign41130_e54085 * (var_temp1_dn9 - var_temp2_dn9))) / (2.0 * assign41130_e54092)))),)
    } else {
        (var_xsubct, var_xsubct_dn4, var_xsubct_dn6, var_xsubct_dn7, var_xsubct_dn8, var_xsubct_dn9,)
    }
};
        var_xsubct = assign41130_e54096;
        var_xsubct_dn4 = assign41130_e54096_d_n4;
        var_xsubct_dn6 = assign41130_e54096_d_n6;
        var_xsubct_dn7 = assign41130_e54096_d_n7;
        var_xsubct_dn8 = assign41130_e54096_d_n8;
        var_xsubct_dn9 = assign41130_e54096_d_n9;

        let (assign41140_e54115, assign41140_e54115_d_n4, assign41140_e54115_d_n6, assign41140_e54115_d_n7, assign41140_e54115_d_n8, assign41140_e54115_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41140_e54101: f64 = (var_xsubct + var_xctmax);
        let assign41140_e54104: f64 = (var_xsubct - var_xctmax);
        let assign41140_e54107: f64 = (var_xsubct - var_xctmax);
        let assign41140_e54108: f64 = (assign41140_e54104 * assign41140_e54107);
        let assign41140_e54110: f64 = (assign41140_e54108 + 5.0);
        let assign41140_e54111: f64 = (assign41140_e54110).sqrt();
        let assign41140_e54112: f64 = (assign41140_e54101 - assign41140_e54111);
        let assign41140_e54113: f64 = (0.5 * assign41140_e54112);
        (assign41140_e54113, (0.5 * ((var_xsubct_dn4 + var_xctmax_dn4) - ((((var_xsubct_dn4 - var_xctmax_dn4) * assign41140_e54107) + (assign41140_e54104 * (var_xsubct_dn4 - var_xctmax_dn4))) / (2.0 * assign41140_e54111)))), (0.5 * (var_xsubct_dn6 - (((var_xsubct_dn6 * assign41140_e54107) + (assign41140_e54104 * var_xsubct_dn6)) / (2.0 * assign41140_e54111)))), (0.5 * (var_xsubct_dn7 - (((var_xsubct_dn7 * assign41140_e54107) + (assign41140_e54104 * var_xsubct_dn7)) / (2.0 * assign41140_e54111)))), (0.5 * (var_xsubct_dn8 - (((var_xsubct_dn8 * assign41140_e54107) + (assign41140_e54104 * var_xsubct_dn8)) / (2.0 * assign41140_e54111)))), (0.5 * (var_xsubct_dn9 - (((var_xsubct_dn9 * assign41140_e54107) + (assign41140_e54104 * var_xsubct_dn9)) / (2.0 * assign41140_e54111)))),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign41140_e54115;
        var_temp1_dn4 = assign41140_e54115_d_n4;
        var_temp1_dn6 = assign41140_e54115_d_n6;
        var_temp1_dn7 = assign41140_e54115_d_n7;
        var_temp1_dn8 = assign41140_e54115_d_n8;
        var_temp1_dn9 = assign41140_e54115_d_n9;

        *var_aphi_slot = var_aphi;
        *var_aphi_dn4_slot = var_aphi_dn4;
        *var_arloc_slot = var_arloc;
        *var_dctg_slot = var_dctg;
        *var_dctg_dn4_slot = var_dctg_dn4;
        *var_dctg_dn6_slot = var_dctg_dn6;
        *var_dctg_dn7_slot = var_dctg_dn7;
        *var_dctg_dn8_slot = var_dctg_dn8;
        *var_dctg_dn9_slot = var_dctg_dn9;
        *var_dvbstar_slot = var_dvbstar;
        *var_dvbstar_dc_slot = var_dvbstar_dc;
        *var_dvbstar_dc_dn4_slot = var_dvbstar_dc_dn4;
        *var_dvbstar_dc_dn6_slot = var_dvbstar_dc_dn6;
        *var_dvbstar_dc_dn7_slot = var_dvbstar_dc_dn7;
        *var_dvbstar_dc_dn8_slot = var_dvbstar_dc_dn8;
        *var_dvbstar_dc_dn9_slot = var_dvbstar_dc_dn9;
        *var_dvbstar_dn4_slot = var_dvbstar_dn4;
        *var_dvbstar_dn6_slot = var_dvbstar_dn6;
        *var_dvbstar_dn7_slot = var_dvbstar_dn7;
        *var_dvbstar_dn8_slot = var_dvbstar_dn8;
        *var_dvbstar_dn9_slot = var_dvbstar_dn9;
        *var_g_0_slot = var_g_0;
        *var_g_0_dn4_slot = var_g_0_dn4;
        *var_guard1189_slot = var_guard1189;
        *var_guard1190_slot = var_guard1190;
        *var_phib_slot = var_phib;
        *var_phib_dn4_slot = var_phib_dn4;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp__blk949_slot = var_temp__blk949;
        *var_temp__blk949_dn4_slot = var_temp__blk949_dn4;
        *var_temp__blk949_dn6_slot = var_temp__blk949_dn6;
        *var_temp__blk949_dn7_slot = var_temp__blk949_dn7;
        *var_temp__blk949_dn8_slot = var_temp__blk949_dn8;
        *var_temp__blk949_dn9_slot = var_temp__blk949_dn9;
        *var_thesatloc_slot = var_thesatloc;
        *var_thesatloc_dn4_slot = var_thesatloc_dn4;
        *var_us_slot = var_us;
        *var_us_dn4_slot = var_us_dn4;
        *var_us_dn6_slot = var_us_dn6;
        *var_us_dn7_slot = var_us_dn7;
        *var_us_dn8_slot = var_us_dn8;
        *var_us_dn9_slot = var_us_dn9;
        *var_usnew_slot = var_usnew;
        *var_usnew_dn4_slot = var_usnew_dn4;
        *var_usnew_dn6_slot = var_usnew_dn6;
        *var_usnew_dn7_slot = var_usnew_dn7;
        *var_usnew_dn8_slot = var_usnew_dn8;
        *var_usnew_dn9_slot = var_usnew_dn9;
        *var_v_db_slot = var_v_db;
        *var_v_db_dn7_slot = var_v_db_dn7;
        *var_v_db_dn8_slot = var_v_db_dn8;
        *var_v_db_dn9_slot = var_v_db_dn9;
        *var_v_ds_slot = var_v_ds;
        *var_v_ds_dn7_slot = var_v_ds_dn7;
        *var_v_ds_dn8_slot = var_v_ds_dn8;
        *var_v_xb_slot = var_v_xb;
        *var_v_xb_dc_tmp_slot = var_v_xb_dc_tmp;
        *var_v_xb_dc_tmp_dn4_slot = var_v_xb_dc_tmp_dn4;
        *var_v_xb_dc_tmp_dn7_slot = var_v_xb_dc_tmp_dn7;
        *var_v_xb_dc_tmp_dn8_slot = var_v_xb_dc_tmp_dn8;
        *var_v_xb_dc_tmp_dn9_slot = var_v_xb_dc_tmp_dn9;
        *var_v_xb_dn4_slot = var_v_xb_dn4;
        *var_v_xb_dn7_slot = var_v_xb_dn7;
        *var_v_xb_dn8_slot = var_v_xb_dn8;
        *var_v_xb_dn9_slot = var_v_xb_dn9;
        *var_vdsx_slot = var_vdsx;
        *var_vdsx_dn7_slot = var_vdsx_dn7;
        *var_vdsx_dn8_slot = var_vdsx_dn8;
        *var_vgb1_slot = var_vgb1;
        *var_vgb1_dn4_slot = var_vgb1_dn4;
        *var_vgb1_dn6_slot = var_vgb1_dn6;
        *var_vgb1_dn7_slot = var_vgb1_dn7;
        *var_vgb1_dn8_slot = var_vgb1_dn8;
        *var_vgb1_dn9_slot = var_vgb1_dn9;
        *var_vmb_slot = var_vmb;
        *var_vmb_dn4_slot = var_vmb_dn4;
        *var_vmb_dn6_slot = var_vmb_dn6;
        *var_vmb_dn7_slot = var_vmb_dn7;
        *var_vmb_dn8_slot = var_vmb_dn8;
        *var_vmb_dn9_slot = var_vmb_dn9;
        *var_vmbnew_slot = var_vmbnew;
        *var_vmbnew_dn4_slot = var_vmbnew_dn4;
        *var_vmbnew_dn6_slot = var_vmbnew_dn6;
        *var_vmbnew_dn7_slot = var_vmbnew_dn7;
        *var_vmbnew_dn8_slot = var_vmbnew_dn8;
        *var_vmbnew_dn9_slot = var_vmbnew_dn9;
        *var_vsbstar_slot = var_vsbstar;
        *var_vsbstar_dc_slot = var_vsbstar_dc;
        *var_vsbstar_dc_dn4_slot = var_vsbstar_dc_dn4;
        *var_vsbstar_dc_dn6_slot = var_vsbstar_dc_dn6;
        *var_vsbstar_dc_dn7_slot = var_vsbstar_dc_dn7;
        *var_vsbstar_dc_dn8_slot = var_vsbstar_dc_dn8;
        *var_vsbstar_dc_dn9_slot = var_vsbstar_dc_dn9;
        *var_vsbstar_dc_tmp_slot = var_vsbstar_dc_tmp;
        *var_vsbstar_dc_tmp_dn4_slot = var_vsbstar_dc_tmp_dn4;
        *var_vsbstar_dc_tmp_dn6_slot = var_vsbstar_dc_tmp_dn6;
        *var_vsbstar_dc_tmp_dn7_slot = var_vsbstar_dc_tmp_dn7;
        *var_vsbstar_dc_tmp_dn8_slot = var_vsbstar_dc_tmp_dn8;
        *var_vsbstar_dc_tmp_dn9_slot = var_vsbstar_dc_tmp_dn9;
        *var_vsbstar_dn4_slot = var_vsbstar_dn4;
        *var_vsbstar_dn6_slot = var_vsbstar_dn6;
        *var_vsbstar_dn7_slot = var_vsbstar_dn7;
        *var_vsbstar_dn8_slot = var_vsbstar_dn8;
        *var_vsbstar_dn9_slot = var_vsbstar_dn9;
        *var_vsbx_slot = var_vsbx;
        *var_vsbx_dn4_slot = var_vsbx_dn4;
        *var_vsbx_dn6_slot = var_vsbx_dn6;
        *var_vsbx_dn7_slot = var_vsbx_dn7;
        *var_vsbx_dn8_slot = var_vsbx_dn8;
        *var_vsbx_dn9_slot = var_vsbx_dn9;
        *var_xbct_slot = var_xbct;
        *var_xbct_dn4_slot = var_xbct_dn4;
        *var_xctmax_slot = var_xctmax;
        *var_xctmax_dn4_slot = var_xctmax_dn4;
        *var_xgct_slot = var_xgct;
        *var_xgct_dn4_slot = var_xgct_dn4;
        *var_xgct_dn6_slot = var_xgct_dn6;
        *var_xgct_dn7_slot = var_xgct_dn7;
        *var_xgct_dn8_slot = var_xgct_dn8;
        *var_xgct_dn9_slot = var_xgct_dn9;
        *var_xmict_slot = var_xmict;
        *var_xmict_dn4_slot = var_xmict_dn4;
        *var_xmict_dn6_slot = var_xmict_dn6;
        *var_xmict_dn7_slot = var_xmict_dn7;
        *var_xmict_dn8_slot = var_xmict_dn8;
        *var_xmict_dn9_slot = var_xmict_dn9;
        *var_xnct_slot = var_xnct;
        *var_xnct_dn4_slot = var_xnct_dn4;
        *var_xnct_dn6_slot = var_xnct_dn6;
        *var_xnct_dn7_slot = var_xnct_dn7;
        *var_xnct_dn8_slot = var_xnct_dn8;
        *var_xnct_dn9_slot = var_xnct_dn9;
        *var_xsbstar_slot = var_xsbstar;
        *var_xsbstar_dn4_slot = var_xsbstar_dn4;
        *var_xsbstar_dn6_slot = var_xsbstar_dn6;
        *var_xsbstar_dn7_slot = var_xsbstar_dn7;
        *var_xsbstar_dn8_slot = var_xsbstar_dn8;
        *var_xsbstar_dn9_slot = var_xsbstar_dn9;
        *var_xsubct_slot = var_xsubct;
        *var_xsubct_dn4_slot = var_xsubct_dn4;
        *var_xsubct_dn6_slot = var_xsubct_dn6;
        *var_xsubct_dn7_slot = var_xsubct_dn7;
        *var_xsubct_dn8_slot = var_xsubct_dn8;
        *var_xsubct_dn9_slot = var_xsubct_dn9;
        *var_xwict_slot = var_xwict;
        *var_xwict_dn4_slot = var_xwict_dn4;
        *var_xwict_dn6_slot = var_xwict_dn6;
        *var_xwict_dn7_slot = var_xwict_dn7;
        *var_xwict_dn8_slot = var_xwict_dn8;
        *var_xwict_dn9_slot = var_xwict_dn9;
    }

    pub(super) fn stamp_transient_block_88(
        p: &Parameters,
        var_aphi: f64,
        var_aphi_dn4: f64,
        var_cf_i: f64,
        var_cfb_i: f64,
        var_cfd_i: f64,
        var_ct_t: f64,
        var_ct_t_dn4: f64,
        var_ctg_t: f64,
        var_ctg_t_dn4: f64,
        var_g_0: f64,
        var_g_0_dn4: f64,
        var_guard1190: f64,
        var_phib: f64,
        var_phib_dn4: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_psce_i: f64,
        var_psceb_i: f64,
        var_psced_i: f64,
        var_v_xb: f64,
        var_v_xb_dn4: f64,
        var_v_xb_dn7: f64,
        var_v_xb_dn8: f64,
        var_v_xb_dn9: f64,
        var_vdsx: f64,
        var_vdsx_dn7: f64,
        var_vdsx_dn8: f64,
        var_vgb1: f64,
        var_vgb1_dn4: f64,
        var_vgb1_dn6: f64,
        var_vgb1_dn7: f64,
        var_vgb1_dn8: f64,
        var_vgb1_dn9: f64,
        var_vsbstar: f64,
        var_vsbstar_dn4: f64,
        var_vsbstar_dn6: f64,
        var_vsbstar_dn7: f64,
        var_vsbstar_dn8: f64,
        var_vsbstar_dn9: f64,
        var_vsbx: f64,
        var_vsbx_dn4: f64,
        var_vsbx_dn6: f64,
        var_vsbx_dn7: f64,
        var_vsbx_dn8: f64,
        var_vsbx_dn9: f64,
        var_xctmax: f64,
        var_xctmax_dn4: f64,
        var_ct_fact_slot: &mut f64,
        var_ct_fact_dn4_slot: &mut f64,
        var_ct_fact_dn6_slot: &mut f64,
        var_ct_fact_dn7_slot: &mut f64,
        var_ct_fact_dn8_slot: &mut f64,
        var_ct_fact_dn9_slot: &mut f64,
        var_dctg_slot: &mut f64,
        var_dctg_dn4_slot: &mut f64,
        var_dctg_dn6_slot: &mut f64,
        var_dctg_dn7_slot: &mut f64,
        var_dctg_dn8_slot: &mut f64,
        var_dctg_dn9_slot: &mut f64,
        var_delphib_slot: &mut f64,
        var_delphib_dn4_slot: &mut f64,
        var_delphib_dn6_slot: &mut f64,
        var_delphib_dn7_slot: &mut f64,
        var_delphib_dn8_slot: &mut f64,
        var_delphib_dn9_slot: &mut f64,
        var_delta_ns_slot: &mut f64,
        var_delta_ns_dn4_slot: &mut f64,
        var_delta_ns_dn6_slot: &mut f64,
        var_delta_ns_dn7_slot: &mut f64,
        var_delta_ns_dn8_slot: &mut f64,
        var_delta_ns_dn9_slot: &mut f64,
        var_delxb_slot: &mut f64,
        var_delxb_dn4_slot: &mut f64,
        var_delxb_dn6_slot: &mut f64,
        var_delxb_dn7_slot: &mut f64,
        var_delxb_dn8_slot: &mut f64,
        var_delxb_dn9_slot: &mut f64,
        var_dphit1_slot: &mut f64,
        var_dphit1_dn4_slot: &mut f64,
        var_dphit1_dn6_slot: &mut f64,
        var_dphit1_dn7_slot: &mut f64,
        var_dphit1_dn8_slot: &mut f64,
        var_dphit1_dn9_slot: &mut f64,
        var_fscr_slot: &mut f64,
        var_fscr_dn4_slot: &mut f64,
        var_fscr_dn6_slot: &mut f64,
        var_fscr_dn7_slot: &mut f64,
        var_fscr_dn8_slot: &mut f64,
        var_fscr_dn9_slot: &mut f64,
        var_gf_slot: &mut f64,
        var_gf2_slot: &mut f64,
        var_gf2_dn4_slot: &mut f64,
        var_gf2_dn6_slot: &mut f64,
        var_gf2_dn7_slot: &mut f64,
        var_gf2_dn8_slot: &mut f64,
        var_gf2_dn9_slot: &mut f64,
        var_gf_dn4_slot: &mut f64,
        var_gf_dn6_slot: &mut f64,
        var_gf_dn7_slot: &mut f64,
        var_gf_dn8_slot: &mut f64,
        var_gf_dn9_slot: &mut f64,
        var_guard1191_slot: &mut f64,
        var_guard1192_slot: &mut f64,
        var_guard1193_slot: &mut f64,
        var_guard1194_slot: &mut f64,
        var_guard1195_slot: &mut f64,
        var_inv_gf2_slot: &mut f64,
        var_inv_gf2_dn4_slot: &mut f64,
        var_inv_gf2_dn6_slot: &mut f64,
        var_inv_gf2_dn7_slot: &mut f64,
        var_inv_gf2_dn8_slot: &mut f64,
        var_inv_gf2_dn9_slot: &mut f64,
        var_inv_phit1_slot: &mut f64,
        var_inv_phit1_dn4_slot: &mut f64,
        var_inv_phit1_dn6_slot: &mut f64,
        var_inv_phit1_dn7_slot: &mut f64,
        var_inv_phit1_dn8_slot: &mut f64,
        var_inv_phit1_dn9_slot: &mut f64,
        var_nscr_slot: &mut f64,
        var_nscr_dn4_slot: &mut f64,
        var_nscr_dn6_slot: &mut f64,
        var_nscr_dn7_slot: &mut f64,
        var_nscr_dn8_slot: &mut f64,
        var_nscr_dn9_slot: &mut f64,
        var_phit1_slot: &mut f64,
        var_phit1_dn4_slot: &mut f64,
        var_phit1_dn6_slot: &mut f64,
        var_phit1_dn7_slot: &mut f64,
        var_phit1_dn8_slot: &mut f64,
        var_phit1_dn9_slot: &mut f64,
        var_phitct_slot: &mut f64,
        var_phitct_dn4_slot: &mut f64,
        var_phitct_dn6_slot: &mut f64,
        var_phitct_dn7_slot: &mut f64,
        var_phitct_dn8_slot: &mut f64,
        var_phitct_dn9_slot: &mut f64,
        var_qbscr_slot: &mut f64,
        var_qbscr_dn4_slot: &mut f64,
        var_qbscr_dn6_slot: &mut f64,
        var_qbscr_dn7_slot: &mut f64,
        var_qbscr_dn8_slot: &mut f64,
        var_qbscr_dn9_slot: &mut f64,
        var_qiscr_slot: &mut f64,
        var_qiscr0si_slot: &mut f64,
        var_qiscr0si_dn4_slot: &mut f64,
        var_qiscr0si_dn6_slot: &mut f64,
        var_qiscr0si_dn7_slot: &mut f64,
        var_qiscr0si_dn8_slot: &mut f64,
        var_qiscr0si_dn9_slot: &mut f64,
        var_qiscr_dn4_slot: &mut f64,
        var_qiscr_dn6_slot: &mut f64,
        var_qiscr_dn7_slot: &mut f64,
        var_qiscr_dn8_slot: &mut f64,
        var_qiscr_dn9_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp__blk949_slot: &mut f64,
        var_temp__blk949_dn4_slot: &mut f64,
        var_temp__blk949_dn6_slot: &mut f64,
        var_temp__blk949_dn7_slot: &mut f64,
        var_temp__blk949_dn8_slot: &mut f64,
        var_temp__blk949_dn9_slot: &mut f64,
        var_ux_slot: &mut f64,
        var_ux_dn4_slot: &mut f64,
        var_ux_dn6_slot: &mut f64,
        var_ux_dn7_slot: &mut f64,
        var_ux_dn8_slot: &mut f64,
        var_ux_dn9_slot: &mut f64,
        var_vdsp_slot: &mut f64,
        var_vdsp_dn7_slot: &mut f64,
        var_vdsp_dn8_slot: &mut f64,
        var_xb_slot: &mut f64,
        var_xb_dn4_slot: &mut f64,
        var_xb_dn6_slot: &mut f64,
        var_xb_dn7_slot: &mut f64,
        var_xb_dn8_slot: &mut f64,
        var_xb_dn9_slot: &mut f64,
        var_xct_slot: &mut f64,
        var_xct_dn4_slot: &mut f64,
        var_xct_dn6_slot: &mut f64,
        var_xct_dn7_slot: &mut f64,
        var_xct_dn8_slot: &mut f64,
        var_xct_dn9_slot: &mut f64,
        var_xg_slot: &mut f64,
        var_xg_dn4_slot: &mut f64,
        var_xg_dn6_slot: &mut f64,
        var_xg_dn7_slot: &mut f64,
        var_xg_dn8_slot: &mut f64,
        var_xg_dn9_slot: &mut f64,
        var_xgtscr_slot: &mut f64,
        var_xgtscr0_slot: &mut f64,
        var_xgtscr0_dn4_slot: &mut f64,
        var_xgtscr0_dn6_slot: &mut f64,
        var_xgtscr0_dn7_slot: &mut f64,
        var_xgtscr0_dn8_slot: &mut f64,
        var_xgtscr0_dn9_slot: &mut f64,
        var_xgtscr_dn4_slot: &mut f64,
        var_xgtscr_dn6_slot: &mut f64,
        var_xgtscr_dn7_slot: &mut f64,
        var_xgtscr_dn8_slot: &mut f64,
        var_xgtscr_dn9_slot: &mut f64,
        var_xn_s_slot: &mut f64,
        var_xn_s_dn4_slot: &mut f64,
        var_xn_s_dn6_slot: &mut f64,
        var_xn_s_dn7_slot: &mut f64,
        var_xn_s_dn8_slot: &mut f64,
        var_xn_s_dn9_slot: &mut f64,
        var_xno_s_slot: &mut f64,
        var_xno_s_dn4_slot: &mut f64,
        var_xno_s_dn6_slot: &mut f64,
        var_xno_s_dn7_slot: &mut f64,
        var_xno_s_dn8_slot: &mut f64,
        var_xno_s_dn9_slot: &mut f64,
        var_xthscr_slot: &mut f64,
        var_xthscr_dn4_slot: &mut f64,
        var_xthscr_dn6_slot: &mut f64,
        var_xthscr_dn7_slot: &mut f64,
        var_xthscr_dn8_slot: &mut f64,
        var_xthscr_dn9_slot: &mut f64,
    ) {
        let mut var_ct_fact: f64 = *var_ct_fact_slot;
        let mut var_ct_fact_dn4: f64 = *var_ct_fact_dn4_slot;
        let mut var_ct_fact_dn6: f64 = *var_ct_fact_dn6_slot;
        let mut var_ct_fact_dn7: f64 = *var_ct_fact_dn7_slot;
        let mut var_ct_fact_dn8: f64 = *var_ct_fact_dn8_slot;
        let mut var_ct_fact_dn9: f64 = *var_ct_fact_dn9_slot;
        let mut var_dctg: f64 = *var_dctg_slot;
        let mut var_dctg_dn4: f64 = *var_dctg_dn4_slot;
        let mut var_dctg_dn6: f64 = *var_dctg_dn6_slot;
        let mut var_dctg_dn7: f64 = *var_dctg_dn7_slot;
        let mut var_dctg_dn8: f64 = *var_dctg_dn8_slot;
        let mut var_dctg_dn9: f64 = *var_dctg_dn9_slot;
        let mut var_delphib: f64 = *var_delphib_slot;
        let mut var_delphib_dn4: f64 = *var_delphib_dn4_slot;
        let mut var_delphib_dn6: f64 = *var_delphib_dn6_slot;
        let mut var_delphib_dn7: f64 = *var_delphib_dn7_slot;
        let mut var_delphib_dn8: f64 = *var_delphib_dn8_slot;
        let mut var_delphib_dn9: f64 = *var_delphib_dn9_slot;
        let mut var_delta_ns: f64 = *var_delta_ns_slot;
        let mut var_delta_ns_dn4: f64 = *var_delta_ns_dn4_slot;
        let mut var_delta_ns_dn6: f64 = *var_delta_ns_dn6_slot;
        let mut var_delta_ns_dn7: f64 = *var_delta_ns_dn7_slot;
        let mut var_delta_ns_dn8: f64 = *var_delta_ns_dn8_slot;
        let mut var_delta_ns_dn9: f64 = *var_delta_ns_dn9_slot;
        let mut var_delxb: f64 = *var_delxb_slot;
        let mut var_delxb_dn4: f64 = *var_delxb_dn4_slot;
        let mut var_delxb_dn6: f64 = *var_delxb_dn6_slot;
        let mut var_delxb_dn7: f64 = *var_delxb_dn7_slot;
        let mut var_delxb_dn8: f64 = *var_delxb_dn8_slot;
        let mut var_delxb_dn9: f64 = *var_delxb_dn9_slot;
        let mut var_dphit1: f64 = *var_dphit1_slot;
        let mut var_dphit1_dn4: f64 = *var_dphit1_dn4_slot;
        let mut var_dphit1_dn6: f64 = *var_dphit1_dn6_slot;
        let mut var_dphit1_dn7: f64 = *var_dphit1_dn7_slot;
        let mut var_dphit1_dn8: f64 = *var_dphit1_dn8_slot;
        let mut var_dphit1_dn9: f64 = *var_dphit1_dn9_slot;
        let mut var_fscr: f64 = *var_fscr_slot;
        let mut var_fscr_dn4: f64 = *var_fscr_dn4_slot;
        let mut var_fscr_dn6: f64 = *var_fscr_dn6_slot;
        let mut var_fscr_dn7: f64 = *var_fscr_dn7_slot;
        let mut var_fscr_dn8: f64 = *var_fscr_dn8_slot;
        let mut var_fscr_dn9: f64 = *var_fscr_dn9_slot;
        let mut var_gf: f64 = *var_gf_slot;
        let mut var_gf2: f64 = *var_gf2_slot;
        let mut var_gf2_dn4: f64 = *var_gf2_dn4_slot;
        let mut var_gf2_dn6: f64 = *var_gf2_dn6_slot;
        let mut var_gf2_dn7: f64 = *var_gf2_dn7_slot;
        let mut var_gf2_dn8: f64 = *var_gf2_dn8_slot;
        let mut var_gf2_dn9: f64 = *var_gf2_dn9_slot;
        let mut var_gf_dn4: f64 = *var_gf_dn4_slot;
        let mut var_gf_dn6: f64 = *var_gf_dn6_slot;
        let mut var_gf_dn7: f64 = *var_gf_dn7_slot;
        let mut var_gf_dn8: f64 = *var_gf_dn8_slot;
        let mut var_gf_dn9: f64 = *var_gf_dn9_slot;
        let mut var_guard1191: f64 = *var_guard1191_slot;
        let mut var_guard1192: f64 = *var_guard1192_slot;
        let mut var_guard1193: f64 = *var_guard1193_slot;
        let mut var_guard1194: f64 = *var_guard1194_slot;
        let mut var_guard1195: f64 = *var_guard1195_slot;
        let mut var_inv_gf2: f64 = *var_inv_gf2_slot;
        let mut var_inv_gf2_dn4: f64 = *var_inv_gf2_dn4_slot;
        let mut var_inv_gf2_dn6: f64 = *var_inv_gf2_dn6_slot;
        let mut var_inv_gf2_dn7: f64 = *var_inv_gf2_dn7_slot;
        let mut var_inv_gf2_dn8: f64 = *var_inv_gf2_dn8_slot;
        let mut var_inv_gf2_dn9: f64 = *var_inv_gf2_dn9_slot;
        let mut var_inv_phit1: f64 = *var_inv_phit1_slot;
        let mut var_inv_phit1_dn4: f64 = *var_inv_phit1_dn4_slot;
        let mut var_inv_phit1_dn6: f64 = *var_inv_phit1_dn6_slot;
        let mut var_inv_phit1_dn7: f64 = *var_inv_phit1_dn7_slot;
        let mut var_inv_phit1_dn8: f64 = *var_inv_phit1_dn8_slot;
        let mut var_inv_phit1_dn9: f64 = *var_inv_phit1_dn9_slot;
        let mut var_nscr: f64 = *var_nscr_slot;
        let mut var_nscr_dn4: f64 = *var_nscr_dn4_slot;
        let mut var_nscr_dn6: f64 = *var_nscr_dn6_slot;
        let mut var_nscr_dn7: f64 = *var_nscr_dn7_slot;
        let mut var_nscr_dn8: f64 = *var_nscr_dn8_slot;
        let mut var_nscr_dn9: f64 = *var_nscr_dn9_slot;
        let mut var_phit1: f64 = *var_phit1_slot;
        let mut var_phit1_dn4: f64 = *var_phit1_dn4_slot;
        let mut var_phit1_dn6: f64 = *var_phit1_dn6_slot;
        let mut var_phit1_dn7: f64 = *var_phit1_dn7_slot;
        let mut var_phit1_dn8: f64 = *var_phit1_dn8_slot;
        let mut var_phit1_dn9: f64 = *var_phit1_dn9_slot;
        let mut var_phitct: f64 = *var_phitct_slot;
        let mut var_phitct_dn4: f64 = *var_phitct_dn4_slot;
        let mut var_phitct_dn6: f64 = *var_phitct_dn6_slot;
        let mut var_phitct_dn7: f64 = *var_phitct_dn7_slot;
        let mut var_phitct_dn8: f64 = *var_phitct_dn8_slot;
        let mut var_phitct_dn9: f64 = *var_phitct_dn9_slot;
        let mut var_qbscr: f64 = *var_qbscr_slot;
        let mut var_qbscr_dn4: f64 = *var_qbscr_dn4_slot;
        let mut var_qbscr_dn6: f64 = *var_qbscr_dn6_slot;
        let mut var_qbscr_dn7: f64 = *var_qbscr_dn7_slot;
        let mut var_qbscr_dn8: f64 = *var_qbscr_dn8_slot;
        let mut var_qbscr_dn9: f64 = *var_qbscr_dn9_slot;
        let mut var_qiscr: f64 = *var_qiscr_slot;
        let mut var_qiscr0si: f64 = *var_qiscr0si_slot;
        let mut var_qiscr0si_dn4: f64 = *var_qiscr0si_dn4_slot;
        let mut var_qiscr0si_dn6: f64 = *var_qiscr0si_dn6_slot;
        let mut var_qiscr0si_dn7: f64 = *var_qiscr0si_dn7_slot;
        let mut var_qiscr0si_dn8: f64 = *var_qiscr0si_dn8_slot;
        let mut var_qiscr0si_dn9: f64 = *var_qiscr0si_dn9_slot;
        let mut var_qiscr_dn4: f64 = *var_qiscr_dn4_slot;
        let mut var_qiscr_dn6: f64 = *var_qiscr_dn6_slot;
        let mut var_qiscr_dn7: f64 = *var_qiscr_dn7_slot;
        let mut var_qiscr_dn8: f64 = *var_qiscr_dn8_slot;
        let mut var_qiscr_dn9: f64 = *var_qiscr_dn9_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp__blk949: f64 = *var_temp__blk949_slot;
        let mut var_temp__blk949_dn4: f64 = *var_temp__blk949_dn4_slot;
        let mut var_temp__blk949_dn6: f64 = *var_temp__blk949_dn6_slot;
        let mut var_temp__blk949_dn7: f64 = *var_temp__blk949_dn7_slot;
        let mut var_temp__blk949_dn8: f64 = *var_temp__blk949_dn8_slot;
        let mut var_temp__blk949_dn9: f64 = *var_temp__blk949_dn9_slot;
        let mut var_ux: f64 = *var_ux_slot;
        let mut var_ux_dn4: f64 = *var_ux_dn4_slot;
        let mut var_ux_dn6: f64 = *var_ux_dn6_slot;
        let mut var_ux_dn7: f64 = *var_ux_dn7_slot;
        let mut var_ux_dn8: f64 = *var_ux_dn8_slot;
        let mut var_ux_dn9: f64 = *var_ux_dn9_slot;
        let mut var_vdsp: f64 = *var_vdsp_slot;
        let mut var_vdsp_dn7: f64 = *var_vdsp_dn7_slot;
        let mut var_vdsp_dn8: f64 = *var_vdsp_dn8_slot;
        let mut var_xb: f64 = *var_xb_slot;
        let mut var_xb_dn4: f64 = *var_xb_dn4_slot;
        let mut var_xb_dn6: f64 = *var_xb_dn6_slot;
        let mut var_xb_dn7: f64 = *var_xb_dn7_slot;
        let mut var_xb_dn8: f64 = *var_xb_dn8_slot;
        let mut var_xb_dn9: f64 = *var_xb_dn9_slot;
        let mut var_xct: f64 = *var_xct_slot;
        let mut var_xct_dn4: f64 = *var_xct_dn4_slot;
        let mut var_xct_dn6: f64 = *var_xct_dn6_slot;
        let mut var_xct_dn7: f64 = *var_xct_dn7_slot;
        let mut var_xct_dn8: f64 = *var_xct_dn8_slot;
        let mut var_xct_dn9: f64 = *var_xct_dn9_slot;
        let mut var_xg: f64 = *var_xg_slot;
        let mut var_xg_dn4: f64 = *var_xg_dn4_slot;
        let mut var_xg_dn6: f64 = *var_xg_dn6_slot;
        let mut var_xg_dn7: f64 = *var_xg_dn7_slot;
        let mut var_xg_dn8: f64 = *var_xg_dn8_slot;
        let mut var_xg_dn9: f64 = *var_xg_dn9_slot;
        let mut var_xgtscr: f64 = *var_xgtscr_slot;
        let mut var_xgtscr0: f64 = *var_xgtscr0_slot;
        let mut var_xgtscr0_dn4: f64 = *var_xgtscr0_dn4_slot;
        let mut var_xgtscr0_dn6: f64 = *var_xgtscr0_dn6_slot;
        let mut var_xgtscr0_dn7: f64 = *var_xgtscr0_dn7_slot;
        let mut var_xgtscr0_dn8: f64 = *var_xgtscr0_dn8_slot;
        let mut var_xgtscr0_dn9: f64 = *var_xgtscr0_dn9_slot;
        let mut var_xgtscr_dn4: f64 = *var_xgtscr_dn4_slot;
        let mut var_xgtscr_dn6: f64 = *var_xgtscr_dn6_slot;
        let mut var_xgtscr_dn7: f64 = *var_xgtscr_dn7_slot;
        let mut var_xgtscr_dn8: f64 = *var_xgtscr_dn8_slot;
        let mut var_xgtscr_dn9: f64 = *var_xgtscr_dn9_slot;
        let mut var_xn_s: f64 = *var_xn_s_slot;
        let mut var_xn_s_dn4: f64 = *var_xn_s_dn4_slot;
        let mut var_xn_s_dn6: f64 = *var_xn_s_dn6_slot;
        let mut var_xn_s_dn7: f64 = *var_xn_s_dn7_slot;
        let mut var_xn_s_dn8: f64 = *var_xn_s_dn8_slot;
        let mut var_xn_s_dn9: f64 = *var_xn_s_dn9_slot;
        let mut var_xno_s: f64 = *var_xno_s_slot;
        let mut var_xno_s_dn4: f64 = *var_xno_s_dn4_slot;
        let mut var_xno_s_dn6: f64 = *var_xno_s_dn6_slot;
        let mut var_xno_s_dn7: f64 = *var_xno_s_dn7_slot;
        let mut var_xno_s_dn8: f64 = *var_xno_s_dn8_slot;
        let mut var_xno_s_dn9: f64 = *var_xno_s_dn9_slot;
        let mut var_xthscr: f64 = *var_xthscr_slot;
        let mut var_xthscr_dn4: f64 = *var_xthscr_dn4_slot;
        let mut var_xthscr_dn6: f64 = *var_xthscr_dn6_slot;
        let mut var_xthscr_dn7: f64 = *var_xthscr_dn7_slot;
        let mut var_xthscr_dn8: f64 = *var_xthscr_dn8_slot;
        let mut var_xthscr_dn9: f64 = *var_xthscr_dn9_slot;

        let (assign41150_e54137, assign41150_e54137_d_n4, assign41150_e54137_d_n6, assign41150_e54137_d_n7, assign41150_e54137_d_n8, assign41150_e54137_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41150_e54120: f64 = (-var_xctmax);
        let assign41150_e54121: f64 = (var_temp1 + assign41150_e54120);
        let assign41150_e54124: f64 = (-var_xctmax);
        let assign41150_e54125: f64 = (var_temp1 - assign41150_e54124);
        let assign41150_e54128: f64 = (-var_xctmax);
        let assign41150_e54129: f64 = (var_temp1 - assign41150_e54128);
        let assign41150_e54130: f64 = (assign41150_e54125 * assign41150_e54129);
        let assign41150_e54132: f64 = (assign41150_e54130 + 20.0);
        let assign41150_e54133: f64 = (assign41150_e54132).sqrt();
        let assign41150_e54134: f64 = (assign41150_e54121 + assign41150_e54133);
        let assign41150_e54135: f64 = (0.5 * assign41150_e54134);
        (assign41150_e54135, (0.5 * ((var_temp1_dn4 + (-var_xctmax_dn4)) + ((((var_temp1_dn4 - (-var_xctmax_dn4)) * assign41150_e54129) + (assign41150_e54125 * (var_temp1_dn4 - (-var_xctmax_dn4)))) / (2.0 * assign41150_e54133)))), (0.5 * (var_temp1_dn6 + (((var_temp1_dn6 * assign41150_e54129) + (assign41150_e54125 * var_temp1_dn6)) / (2.0 * assign41150_e54133)))), (0.5 * (var_temp1_dn7 + (((var_temp1_dn7 * assign41150_e54129) + (assign41150_e54125 * var_temp1_dn7)) / (2.0 * assign41150_e54133)))), (0.5 * (var_temp1_dn8 + (((var_temp1_dn8 * assign41150_e54129) + (assign41150_e54125 * var_temp1_dn8)) / (2.0 * assign41150_e54133)))), (0.5 * (var_temp1_dn9 + (((var_temp1_dn9 * assign41150_e54129) + (assign41150_e54125 * var_temp1_dn9)) / (2.0 * assign41150_e54133)))),)
    } else {
        (var_xct, var_xct_dn4, var_xct_dn6, var_xct_dn7, var_xct_dn8, var_xct_dn9,)
    }
};
        var_xct = assign41150_e54137;
        var_xct_dn4 = assign41150_e54137_d_n4;
        var_xct_dn6 = assign41150_e54137_d_n6;
        var_xct_dn7 = assign41150_e54137_d_n7;
        var_xct_dn8 = assign41150_e54137_d_n8;
        var_xct_dn9 = assign41150_e54137_d_n9;

        let (assign41160_e54147, assign41160_e54147_d_n4, assign41160_e54147_d_n6, assign41160_e54147_d_n7, assign41160_e54147_d_n8, assign41160_e54147_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41160_e54142: f64 = (var_xct / var_xctmax);
        let assign41160_e54144: f64 = (assign41160_e54142 + 1.0);
        let assign41160_e54145: f64 = (var_ctg_t * assign41160_e54144);
        (assign41160_e54145, ((var_ctg_t_dn4 * assign41160_e54144) + (var_ctg_t * (((var_xct_dn4 * var_xctmax) - (var_xct * var_xctmax_dn4)) / (var_xctmax * var_xctmax)))), (var_ctg_t * (var_xct_dn6 / var_xctmax)), (var_ctg_t * (var_xct_dn7 / var_xctmax)), (var_ctg_t * (var_xct_dn8 / var_xctmax)), (var_ctg_t * (var_xct_dn9 / var_xctmax)),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign41160_e54147;
        var_temp2_dn4 = assign41160_e54147_d_n4;
        var_temp2_dn6 = assign41160_e54147_d_n6;
        var_temp2_dn7 = assign41160_e54147_d_n7;
        var_temp2_dn8 = assign41160_e54147_d_n8;
        var_temp2_dn9 = assign41160_e54147_d_n9;

        let assign41170_e54150: f64 = (-230.25850929940458);
        let assign41170_e54151: f64 = if var_temp2 > assign41170_e54150 { 1.0 } else { 0.0 };
        var_guard1191 = assign41170_e54151;

        let (assign41180_e54158, assign41180_e54158_d_n4, assign41180_e54158_d_n6, assign41180_e54158_d_n7, assign41180_e54158_d_n8, assign41180_e54158_d_n9,) = {
    if ((var_guard1190 != 0.0) && (var_guard1191 != 0.0)) {
        let assign41180_e54156: f64 = (var_temp2).exp();
        (assign41180_e54156, (assign41180_e54156 * var_temp2_dn4), (assign41180_e54156 * var_temp2_dn6), (assign41180_e54156 * var_temp2_dn7), (assign41180_e54156 * var_temp2_dn8), (assign41180_e54156 * var_temp2_dn9),)
    } else {
        (var_dctg, var_dctg_dn4, var_dctg_dn6, var_dctg_dn7, var_dctg_dn8, var_dctg_dn9,)
    }
};
        var_dctg = assign41180_e54158;
        var_dctg_dn4 = assign41180_e54158_d_n4;
        var_dctg_dn6 = assign41180_e54158_d_n6;
        var_dctg_dn7 = assign41180_e54158_d_n7;
        var_dctg_dn8 = assign41180_e54158_d_n8;
        var_dctg_dn9 = assign41180_e54158_d_n9;

        let (assign41190_e54190, assign41190_e54190_d_n4, assign41190_e54190_d_n6, assign41190_e54190_d_n7, assign41190_e54190_d_n8, assign41190_e54190_d_n9,) = {
    if ((var_guard1190 != 0.0) && (var_guard1191 == 0.0)) {
        let assign41190_e54166: f64 = (-230.25850929940458);
        let assign41190_e54168: f64 = (assign41190_e54166 - var_temp2);
        let assign41190_e54172: f64 = (-230.25850929940458);
        let assign41190_e54174: f64 = (assign41190_e54172 - var_temp2);
        let assign41190_e54177: f64 = (-230.25850929940458);
        let assign41190_e54179: f64 = (assign41190_e54177 - var_temp2);
        let assign41190_e54181: f64 = (assign41190_e54179 * 0.3333333333333333);
        let assign41190_e54182: f64 = (1.0 + assign41190_e54181);
        let assign41190_e54183: f64 = (assign41190_e54174 * assign41190_e54182);
        let assign41190_e54184: f64 = (0.5 * assign41190_e54183);
        let assign41190_e54185: f64 = (1.0 + assign41190_e54184);
        let assign41190_e54186: f64 = (assign41190_e54168 * assign41190_e54185);
        let assign41190_e54187: f64 = (1.0 + assign41190_e54186);
        let assign41190_e54188: f64 = (1e-100 / assign41190_e54187);
        (assign41190_e54188, (-((1e-100 * (((-var_temp2_dn4) * assign41190_e54185) + (assign41190_e54168 * (0.5 * (((-var_temp2_dn4) * assign41190_e54182) + (assign41190_e54174 * ((-var_temp2_dn4) * 0.3333333333333333))))))) / (assign41190_e54187 * assign41190_e54187))), (-((1e-100 * (((-var_temp2_dn6) * assign41190_e54185) + (assign41190_e54168 * (0.5 * (((-var_temp2_dn6) * assign41190_e54182) + (assign41190_e54174 * ((-var_temp2_dn6) * 0.3333333333333333))))))) / (assign41190_e54187 * assign41190_e54187))), (-((1e-100 * (((-var_temp2_dn7) * assign41190_e54185) + (assign41190_e54168 * (0.5 * (((-var_temp2_dn7) * assign41190_e54182) + (assign41190_e54174 * ((-var_temp2_dn7) * 0.3333333333333333))))))) / (assign41190_e54187 * assign41190_e54187))), (-((1e-100 * (((-var_temp2_dn8) * assign41190_e54185) + (assign41190_e54168 * (0.5 * (((-var_temp2_dn8) * assign41190_e54182) + (assign41190_e54174 * ((-var_temp2_dn8) * 0.3333333333333333))))))) / (assign41190_e54187 * assign41190_e54187))), (-((1e-100 * (((-var_temp2_dn9) * assign41190_e54185) + (assign41190_e54168 * (0.5 * (((-var_temp2_dn9) * assign41190_e54182) + (assign41190_e54174 * ((-var_temp2_dn9) * 0.3333333333333333))))))) / (assign41190_e54187 * assign41190_e54187))),)
    } else {
        (var_dctg, var_dctg_dn4, var_dctg_dn6, var_dctg_dn7, var_dctg_dn8, var_dctg_dn9,)
    }
};
        var_dctg = assign41190_e54190;
        var_dctg_dn4 = assign41190_e54190_d_n4;
        var_dctg_dn6 = assign41190_e54190_d_n6;
        var_dctg_dn7 = assign41190_e54190_d_n7;
        var_dctg_dn8 = assign41190_e54190_d_n8;
        var_dctg_dn9 = assign41190_e54190_d_n9;

        let assign41200_e54194: f64 = (var_ct_t * var_dctg);
        let assign41200_e54195: f64 = (1.0 + assign41200_e54194);
        var_ct_fact = assign41200_e54195;
        var_ct_fact_dn4 = ((var_ct_t_dn4 * var_dctg) + (var_ct_t * var_dctg_dn4));
        var_ct_fact_dn6 = (var_ct_t * var_dctg_dn6);
        var_ct_fact_dn7 = (var_ct_t * var_dctg_dn7);
        var_ct_fact_dn8 = (var_ct_t * var_dctg_dn8);
        var_ct_fact_dn9 = (var_ct_t * var_dctg_dn9);

        let assign41210_e54198: f64 = (var_phit * var_ct_fact);
        var_phitct = assign41210_e54198;
        var_phitct_dn4 = ((var_phit_dn4 * var_ct_fact) + (var_phit * var_ct_fact_dn4));
        var_phitct_dn6 = (var_phit * var_ct_fact_dn6);
        var_phitct_dn7 = (var_phit * var_ct_fact_dn7);
        var_phitct_dn8 = (var_phit * var_ct_fact_dn8);
        var_phitct_dn9 = (var_phit * var_ct_fact_dn9);

        let assign41220_e54203: f64 = (var_psced_i * var_vdsx);
        let assign41220_e54204: f64 = (1.0 + assign41220_e54203);
        let assign41220_e54205: f64 = (var_psce_i * assign41220_e54204);
        let assign41220_e54209: f64 = (var_psceb_i * var_vsbx);
        let assign41220_e54210: f64 = (1.0 + assign41220_e54209);
        let assign41220_e54211: f64 = (assign41220_e54205 * assign41220_e54210);
        var_dphit1 = assign41220_e54211;
        var_dphit1_dn4 = (assign41220_e54205 * (var_psceb_i * var_vsbx_dn4));
        var_dphit1_dn6 = (assign41220_e54205 * (var_psceb_i * var_vsbx_dn6));
        var_dphit1_dn7 = (((var_psce_i * (var_psced_i * var_vdsx_dn7)) * assign41220_e54210) + (assign41220_e54205 * (var_psceb_i * var_vsbx_dn7)));
        var_dphit1_dn8 = (((var_psce_i * (var_psced_i * var_vdsx_dn8)) * assign41220_e54210) + (assign41220_e54205 * (var_psceb_i * var_vsbx_dn8)));
        var_dphit1_dn9 = (assign41220_e54205 * (var_psceb_i * var_vsbx_dn9));

        let assign41230_e54215: f64 = (1.0 + var_dphit1);
        let assign41230_e54216: f64 = (var_phitct * assign41230_e54215);
        var_phit1 = assign41230_e54216;
        var_phit1_dn4 = ((var_phitct_dn4 * assign41230_e54215) + (var_phitct * var_dphit1_dn4));
        var_phit1_dn6 = ((var_phitct_dn6 * assign41230_e54215) + (var_phitct * var_dphit1_dn6));
        var_phit1_dn7 = ((var_phitct_dn7 * assign41230_e54215) + (var_phitct * var_dphit1_dn7));
        var_phit1_dn8 = ((var_phitct_dn8 * assign41230_e54215) + (var_phitct * var_dphit1_dn8));
        var_phit1_dn9 = ((var_phitct_dn9 * assign41230_e54215) + (var_phitct * var_dphit1_dn9));

        let assign41240_e54219: f64 = (1.0 / var_phit1);
        var_inv_phit1 = assign41240_e54219;
        var_inv_phit1_dn4 = (-(var_phit1_dn4 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn6 = (-(var_phit1_dn6 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn7 = (-(var_phit1_dn7 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn8 = (-(var_phit1_dn8 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn9 = (-(var_phit1_dn9 / (var_phit1 * var_phit1)));

        let assign41250_e54223: f64 = (var_phit * var_inv_phit1);
        let assign41250_e54224: f64 = (assign41250_e54223).sqrt();
        let assign41250_e54225: f64 = (var_g_0 * assign41250_e54224);
        var_gf = assign41250_e54225;
        var_gf_dn4 = ((var_g_0_dn4 * assign41250_e54224) + (var_g_0 * (((var_phit_dn4 * var_inv_phit1) + (var_phit * var_inv_phit1_dn4)) / (2.0 * assign41250_e54224))));
        var_gf_dn6 = (var_g_0 * ((var_phit * var_inv_phit1_dn6) / (2.0 * assign41250_e54224)));
        var_gf_dn7 = (var_g_0 * ((var_phit * var_inv_phit1_dn7) / (2.0 * assign41250_e54224)));
        var_gf_dn8 = (var_g_0 * ((var_phit * var_inv_phit1_dn8) / (2.0 * assign41250_e54224)));
        var_gf_dn9 = (var_g_0 * ((var_phit * var_inv_phit1_dn9) / (2.0 * assign41250_e54224)));

        let assign41260_e54228: f64 = (var_gf * var_gf);
        var_gf2 = assign41260_e54228;
        var_gf2_dn4 = ((var_gf_dn4 * var_gf) + (var_gf * var_gf_dn4));
        var_gf2_dn6 = ((var_gf_dn6 * var_gf) + (var_gf * var_gf_dn6));
        var_gf2_dn7 = ((var_gf_dn7 * var_gf) + (var_gf * var_gf_dn7));
        var_gf2_dn8 = ((var_gf_dn8 * var_gf) + (var_gf * var_gf_dn8));
        var_gf2_dn9 = ((var_gf_dn9 * var_gf) + (var_gf * var_gf_dn9));

        let assign41270_e54231: f64 = (1.0 / var_gf2);
        var_inv_gf2 = assign41270_e54231;
        var_inv_gf2_dn4 = (-(var_gf2_dn4 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn6 = (-(var_gf2_dn6 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn7 = (-(var_gf2_dn7 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn8 = (-(var_gf2_dn8 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn9 = (-(var_gf2_dn9 / (var_gf2 * var_gf2)));

        let assign41280_e54234: f64 = (var_vsbstar * var_inv_phit1);
        var_ux = assign41280_e54234;
        var_ux_dn4 = ((var_vsbstar_dn4 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn4));
        var_ux_dn6 = ((var_vsbstar_dn6 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn6));
        var_ux_dn7 = ((var_vsbstar_dn7 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn7));
        var_ux_dn8 = ((var_vsbstar_dn8 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn8));
        var_ux_dn9 = ((var_vsbstar_dn9 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn9));

        let assign41290_e54237: f64 = (var_vgb1 * var_inv_phit1);
        var_xg = assign41290_e54237;
        var_xg_dn4 = ((var_vgb1_dn4 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn4));
        var_xg_dn6 = ((var_vgb1_dn6 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn6));
        var_xg_dn7 = ((var_vgb1_dn7 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn7));
        var_xg_dn8 = ((var_vgb1_dn8 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn8));
        var_xg_dn9 = ((var_vgb1_dn9 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn9));

        let assign41300_e54240: f64 = (2.0 * var_vdsx);
        let assign41300_e54245: f64 = (var_cfd_i * var_vdsx);
        let assign41300_e54246: f64 = (1.0 + assign41300_e54245);
        let assign41300_e54247: f64 = (assign41300_e54246).sqrt();
        let assign41300_e54248: f64 = (1.0 + assign41300_e54247);
        let assign41300_e54249: f64 = (assign41300_e54240 / assign41300_e54248);
        var_vdsp = assign41300_e54249;
        var_vdsp_dn7 = ((((2.0 * var_vdsx_dn7) * assign41300_e54248) - (assign41300_e54240 * ((var_cfd_i * var_vdsx_dn7) / (2.0 * assign41300_e54247)))) / (assign41300_e54248 * assign41300_e54248));
        var_vdsp_dn8 = ((((2.0 * var_vdsx_dn8) * assign41300_e54248) - (assign41300_e54240 * ((var_cfd_i * var_vdsx_dn8) / (2.0 * assign41300_e54247)))) / (assign41300_e54248 * assign41300_e54248));

        let assign41310_e54252: f64 = (var_cf_i * var_vdsp);
        let assign41310_e54256: f64 = (var_cfb_i * var_vsbx);
        let assign41310_e54257: f64 = (1.0 + assign41310_e54256);
        let assign41310_e54258: f64 = (assign41310_e54252 * assign41310_e54257);
        var_delphib = assign41310_e54258;
        var_delphib_dn4 = (assign41310_e54252 * (var_cfb_i * var_vsbx_dn4));
        var_delphib_dn6 = (assign41310_e54252 * (var_cfb_i * var_vsbx_dn6));
        var_delphib_dn7 = (((var_cf_i * var_vdsp_dn7) * assign41310_e54257) + (assign41310_e54252 * (var_cfb_i * var_vsbx_dn7)));
        var_delphib_dn8 = (((var_cf_i * var_vdsp_dn8) * assign41310_e54257) + (assign41310_e54252 * (var_cfb_i * var_vsbx_dn8)));
        var_delphib_dn9 = (assign41310_e54252 * (var_cfb_i * var_vsbx_dn9));

        let assign41320_e54261: f64 = (var_phib * var_inv_phit1);
        var_xb = assign41320_e54261;
        var_xb_dn4 = ((var_phib_dn4 * var_inv_phit1) + (var_phib * var_inv_phit1_dn4));
        var_xb_dn6 = (var_phib * var_inv_phit1_dn6);
        var_xb_dn7 = (var_phib * var_inv_phit1_dn7);
        var_xb_dn8 = (var_phib * var_inv_phit1_dn8);
        var_xb_dn9 = (var_phib * var_inv_phit1_dn9);

        let assign41330_e54264: f64 = (var_v_xb * var_v_xb);
        let assign41330_e54266: f64 = (assign41330_e54264 + var_aphi);
        let assign41330_e54267: f64 = (assign41330_e54266).sqrt();
        var_temp1 = assign41330_e54267;
        var_temp1_dn4 = ((((var_v_xb_dn4 * var_v_xb) + (var_v_xb * var_v_xb_dn4)) + var_aphi_dn4) / (2.0 * assign41330_e54267));
        var_temp1_dn6 = 0.0;
        var_temp1_dn7 = (((var_v_xb_dn7 * var_v_xb) + (var_v_xb * var_v_xb_dn7)) / (2.0 * assign41330_e54267));
        var_temp1_dn8 = (((var_v_xb_dn8 * var_v_xb) + (var_v_xb * var_v_xb_dn8)) / (2.0 * assign41330_e54267));
        var_temp1_dn9 = (((var_v_xb_dn9 * var_v_xb) + (var_v_xb * var_v_xb_dn9)) / (2.0 * assign41330_e54267));

        let assign41340_e54270: f64 = (var_v_xb - var_delphib);
        let assign41340_e54273: f64 = (var_v_xb - var_delphib);
        let assign41340_e54274: f64 = (assign41340_e54270 * assign41340_e54273);
        let assign41340_e54276: f64 = (assign41340_e54274 + var_aphi);
        let assign41340_e54277: f64 = (assign41340_e54276).sqrt();
        var_temp2 = assign41340_e54277;
        var_temp2_dn4 = (((((var_v_xb_dn4 - var_delphib_dn4) * assign41340_e54273) + (assign41340_e54270 * (var_v_xb_dn4 - var_delphib_dn4))) + var_aphi_dn4) / (2.0 * assign41340_e54277));
        var_temp2_dn6 = ((((-var_delphib_dn6) * assign41340_e54273) + (assign41340_e54270 * (-var_delphib_dn6))) / (2.0 * assign41340_e54277));
        var_temp2_dn7 = ((((var_v_xb_dn7 - var_delphib_dn7) * assign41340_e54273) + (assign41340_e54270 * (var_v_xb_dn7 - var_delphib_dn7))) / (2.0 * assign41340_e54277));
        var_temp2_dn8 = ((((var_v_xb_dn8 - var_delphib_dn8) * assign41340_e54273) + (assign41340_e54270 * (var_v_xb_dn8 - var_delphib_dn8))) / (2.0 * assign41340_e54277));
        var_temp2_dn9 = ((((var_v_xb_dn9 - var_delphib_dn9) * assign41340_e54273) + (assign41340_e54270 * (var_v_xb_dn9 - var_delphib_dn9))) / (2.0 * assign41340_e54277));

        let assign41350_e54280: f64 = (0.5 * var_inv_phit1);
        let assign41350_e54283: f64 = (var_delphib + var_temp1);
        let assign41350_e54285: f64 = (assign41350_e54283 - var_temp2);
        let assign41350_e54286: f64 = (assign41350_e54280 * assign41350_e54285);
        var_delxb = assign41350_e54286;
        var_delxb_dn4 = (((0.5 * var_inv_phit1_dn4) * assign41350_e54285) + (assign41350_e54280 * ((var_delphib_dn4 + var_temp1_dn4) - var_temp2_dn4)));
        var_delxb_dn6 = (((0.5 * var_inv_phit1_dn6) * assign41350_e54285) + (assign41350_e54280 * ((var_delphib_dn6 + var_temp1_dn6) - var_temp2_dn6)));
        var_delxb_dn7 = (((0.5 * var_inv_phit1_dn7) * assign41350_e54285) + (assign41350_e54280 * ((var_delphib_dn7 + var_temp1_dn7) - var_temp2_dn7)));
        var_delxb_dn8 = (((0.5 * var_inv_phit1_dn8) * assign41350_e54285) + (assign41350_e54280 * ((var_delphib_dn8 + var_temp1_dn8) - var_temp2_dn8)));
        var_delxb_dn9 = (((0.5 * var_inv_phit1_dn9) * assign41350_e54285) + (assign41350_e54280 * ((var_delphib_dn9 + var_temp1_dn9) - var_temp2_dn9)));

        let assign41360_e54289: f64 = (var_xb + var_ux);
        var_xno_s = assign41360_e54289;
        var_xno_s_dn4 = (var_xb_dn4 + var_ux_dn4);
        var_xno_s_dn6 = (var_xb_dn6 + var_ux_dn6);
        var_xno_s_dn7 = (var_xb_dn7 + var_ux_dn7);
        var_xno_s_dn8 = (var_xb_dn8 + var_ux_dn8);
        var_xno_s_dn9 = (var_xb_dn9 + var_ux_dn9);

        let assign41370_e54292: f64 = (var_xno_s - var_delxb);
        var_xn_s = assign41370_e54292;
        var_xn_s_dn4 = (var_xno_s_dn4 - var_delxb_dn4);
        var_xn_s_dn6 = (var_xno_s_dn6 - var_delxb_dn6);
        var_xn_s_dn7 = (var_xno_s_dn7 - var_delxb_dn7);
        var_xn_s_dn8 = (var_xno_s_dn8 - var_delxb_dn8);
        var_xn_s_dn9 = (var_xno_s_dn9 - var_delxb_dn9);

        let assign41380_e54295: f64 = if p.p45 > 0.0 { 1.0 } else { 0.0 };
        var_guard1192 = assign41380_e54295;

        let assign41390_e54297: f64 = (var_xn_s).abs();
        let assign41390_e54299: f64 = if assign41390_e54297 < 1e-5 { 1.0 } else { 0.0 };
        var_guard1193 = assign41390_e54299;

        let (assign41400_e54319, assign41400_e54319_d_n4, assign41400_e54319_d_n6, assign41400_e54319_d_n7, assign41400_e54319_d_n8, assign41400_e54319_d_n9,) = {
    if ((var_guard1192 != 0.0) && (var_guard1193 != 0.0)) {
        let assign41400_e54308: f64 = (0.5 * var_xn_s);
        let assign41400_e54312: f64 = (0.3125 * var_xn_s);
        let assign41400_e54313: f64 = (1.0 - assign41400_e54312);
        let assign41400_e54314: f64 = (assign41400_e54308 * assign41400_e54313);
        let assign41400_e54315: f64 = (1.0 - assign41400_e54314);
        let assign41400_e54316: f64 = (var_gf * assign41400_e54315);
        let assign41400_e54317: f64 = (1.0 + assign41400_e54316);
        (assign41400_e54317, ((var_gf_dn4 * assign41400_e54315) + (var_gf * (-(((0.5 * var_xn_s_dn4) * assign41400_e54313) + (assign41400_e54308 * (-(0.3125 * var_xn_s_dn4))))))), ((var_gf_dn6 * assign41400_e54315) + (var_gf * (-(((0.5 * var_xn_s_dn6) * assign41400_e54313) + (assign41400_e54308 * (-(0.3125 * var_xn_s_dn6))))))), ((var_gf_dn7 * assign41400_e54315) + (var_gf * (-(((0.5 * var_xn_s_dn7) * assign41400_e54313) + (assign41400_e54308 * (-(0.3125 * var_xn_s_dn7))))))), ((var_gf_dn8 * assign41400_e54315) + (var_gf * (-(((0.5 * var_xn_s_dn8) * assign41400_e54313) + (assign41400_e54308 * (-(0.3125 * var_xn_s_dn8))))))), ((var_gf_dn9 * assign41400_e54315) + (var_gf * (-(((0.5 * var_xn_s_dn9) * assign41400_e54313) + (assign41400_e54308 * (-(0.3125 * var_xn_s_dn9))))))),)
    } else {
        (var_nscr, var_nscr_dn4, var_nscr_dn6, var_nscr_dn7, var_nscr_dn8, var_nscr_dn9,)
    }
};
        var_nscr = assign41400_e54319;
        var_nscr_dn4 = assign41400_e54319_d_n4;
        var_nscr_dn6 = assign41400_e54319_d_n6;
        var_nscr_dn7 = assign41400_e54319_d_n7;
        var_nscr_dn8 = assign41400_e54319_d_n8;
        var_nscr_dn9 = assign41400_e54319_d_n9;

        let assign41410_e54322: f64 = if var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        var_guard1194 = assign41410_e54322;

        let (assign41420_e54333, assign41420_e54333_d_n4, assign41420_e54333_d_n6, assign41420_e54333_d_n7, assign41420_e54333_d_n8, assign41420_e54333_d_n9,) = {
    if (((var_guard1192 != 0.0) && (var_guard1193 == 0.0)) && (var_guard1194 != 0.0)) {
        let assign41420_e54330: f64 = (-var_xn_s);
        let assign41420_e54331: f64 = (assign41420_e54330).exp();
        (assign41420_e54331, (assign41420_e54331 * (-var_xn_s_dn4)), (assign41420_e54331 * (-var_xn_s_dn6)), (assign41420_e54331 * (-var_xn_s_dn7)), (assign41420_e54331 * (-var_xn_s_dn8)), (assign41420_e54331 * (-var_xn_s_dn9)),)
    } else {
        (var_delta_ns, var_delta_ns_dn4, var_delta_ns_dn6, var_delta_ns_dn7, var_delta_ns_dn8, var_delta_ns_dn9,)
    }
};
        var_delta_ns = assign41420_e54333;
        var_delta_ns_dn4 = assign41420_e54333_d_n4;
        var_delta_ns_dn6 = assign41420_e54333_d_n6;
        var_delta_ns_dn7 = assign41420_e54333_d_n7;
        var_delta_ns_dn8 = assign41420_e54333_d_n8;
        var_delta_ns_dn9 = assign41420_e54333_d_n9;

        let (assign41430_e54365, assign41430_e54365_d_n4, assign41430_e54365_d_n6, assign41430_e54365_d_n7, assign41430_e54365_d_n8, assign41430_e54365_d_n9,) = {
    if (((var_guard1192 != 0.0) && (var_guard1193 == 0.0)) && (var_guard1194 == 0.0)) {
        let assign41430_e54345: f64 = (var_xn_s - 460.51701859880916);
        let assign41430_e54350: f64 = (var_xn_s - 460.51701859880916);
        let assign41430_e54354: f64 = (var_xn_s - 460.51701859880916);
        let assign41430_e54356: f64 = (assign41430_e54354 * 0.3333333333333333);
        let assign41430_e54357: f64 = (1.0 + assign41430_e54356);
        let assign41430_e54358: f64 = (assign41430_e54350 * assign41430_e54357);
        let assign41430_e54359: f64 = (0.5 * assign41430_e54358);
        let assign41430_e54360: f64 = (1.0 + assign41430_e54359);
        let assign41430_e54361: f64 = (assign41430_e54345 * assign41430_e54360);
        let assign41430_e54362: f64 = (1.0 + assign41430_e54361);
        let assign41430_e54363: f64 = (1e-200 / assign41430_e54362);
        (assign41430_e54363, (-((1e-200 * ((var_xn_s_dn4 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((var_xn_s_dn4 * assign41430_e54357) + (assign41430_e54350 * (var_xn_s_dn4 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))), (-((1e-200 * ((var_xn_s_dn6 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((var_xn_s_dn6 * assign41430_e54357) + (assign41430_e54350 * (var_xn_s_dn6 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))), (-((1e-200 * ((var_xn_s_dn7 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((var_xn_s_dn7 * assign41430_e54357) + (assign41430_e54350 * (var_xn_s_dn7 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))), (-((1e-200 * ((var_xn_s_dn8 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((var_xn_s_dn8 * assign41430_e54357) + (assign41430_e54350 * (var_xn_s_dn8 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))), (-((1e-200 * ((var_xn_s_dn9 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((var_xn_s_dn9 * assign41430_e54357) + (assign41430_e54350 * (var_xn_s_dn9 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))),)
    } else {
        (var_delta_ns, var_delta_ns_dn4, var_delta_ns_dn6, var_delta_ns_dn7, var_delta_ns_dn8, var_delta_ns_dn9,)
    }
};
        var_delta_ns = assign41430_e54365;
        var_delta_ns_dn4 = assign41430_e54365_d_n4;
        var_delta_ns_dn6 = assign41430_e54365_d_n6;
        var_delta_ns_dn7 = assign41430_e54365_d_n7;
        var_delta_ns_dn8 = assign41430_e54365_d_n8;
        var_delta_ns_dn9 = assign41430_e54365_d_n9;

        let (assign41440_e54378, assign41440_e54378_d_n4, assign41440_e54378_d_n6, assign41440_e54378_d_n7, assign41440_e54378_d_n8, assign41440_e54378_d_n9,) = {
    if ((var_guard1192 != 0.0) && (var_guard1193 == 0.0)) {
        let (assign41440_e54376,) = {
            if (var_xn_s > 0.0) {
                (1.0,)
            } else {
                let assign41440_e54375: f64 = (-1.0);
                (assign41440_e54375,)
            }
        };
        (assign41440_e54376, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign41440_e54378;
        var_temp__blk949_dn4 = assign41440_e54378_d_n4;
        var_temp__blk949_dn6 = assign41440_e54378_d_n6;
        var_temp__blk949_dn7 = assign41440_e54378_d_n7;
        var_temp__blk949_dn8 = assign41440_e54378_d_n8;
        var_temp__blk949_dn9 = assign41440_e54378_d_n9;

        let (assign41450_e54406, assign41450_e54406_d_n4, assign41450_e54406_d_n6, assign41450_e54406_d_n7, assign41450_e54406_d_n8, assign41450_e54406_d_n9,) = {
    if ((var_guard1192 != 0.0) && (var_guard1193 == 0.0)) {
        let assign41450_e54386: f64 = (var_temp__blk949 * var_gf);
        let assign41450_e54391: f64 = (1.0 - var_xn_s);
        let assign41450_e54392: f64 = (var_delta_ns * assign41450_e54391);
        let assign41450_e54393: f64 = (1.0 - assign41450_e54392);
        let assign41450_e54394: f64 = (assign41450_e54386 * assign41450_e54393);
        let assign41450_e54399: f64 = (1.0 - var_delta_ns);
        let assign41450_e54400: f64 = (var_xn_s * assign41450_e54399);
        let assign41450_e54401: f64 = (assign41450_e54400).sqrt();
        let assign41450_e54402: f64 = (2.0 * assign41450_e54401);
        let assign41450_e54403: f64 = (assign41450_e54394 / assign41450_e54402);
        let assign41450_e54404: f64 = (1.0 + assign41450_e54403);
        (assign41450_e54404, (((((((var_temp__blk949_dn4 * var_gf) + (var_temp__blk949 * var_gf_dn4)) * assign41450_e54393) + (assign41450_e54386 * (-((var_delta_ns_dn4 * assign41450_e54391) + (var_delta_ns * (-var_xn_s_dn4)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((var_xn_s_dn4 * assign41450_e54399) + (var_xn_s * (-var_delta_ns_dn4))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)), (((((((var_temp__blk949_dn6 * var_gf) + (var_temp__blk949 * var_gf_dn6)) * assign41450_e54393) + (assign41450_e54386 * (-((var_delta_ns_dn6 * assign41450_e54391) + (var_delta_ns * (-var_xn_s_dn6)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((var_xn_s_dn6 * assign41450_e54399) + (var_xn_s * (-var_delta_ns_dn6))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)), (((((((var_temp__blk949_dn7 * var_gf) + (var_temp__blk949 * var_gf_dn7)) * assign41450_e54393) + (assign41450_e54386 * (-((var_delta_ns_dn7 * assign41450_e54391) + (var_delta_ns * (-var_xn_s_dn7)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((var_xn_s_dn7 * assign41450_e54399) + (var_xn_s * (-var_delta_ns_dn7))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)), (((((((var_temp__blk949_dn8 * var_gf) + (var_temp__blk949 * var_gf_dn8)) * assign41450_e54393) + (assign41450_e54386 * (-((var_delta_ns_dn8 * assign41450_e54391) + (var_delta_ns * (-var_xn_s_dn8)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((var_xn_s_dn8 * assign41450_e54399) + (var_xn_s * (-var_delta_ns_dn8))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)), (((((((var_temp__blk949_dn9 * var_gf) + (var_temp__blk949 * var_gf_dn9)) * assign41450_e54393) + (assign41450_e54386 * (-((var_delta_ns_dn9 * assign41450_e54391) + (var_delta_ns * (-var_xn_s_dn9)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((var_xn_s_dn9 * assign41450_e54399) + (var_xn_s * (-var_delta_ns_dn9))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)),)
    } else {
        (var_nscr, var_nscr_dn4, var_nscr_dn6, var_nscr_dn7, var_nscr_dn8, var_nscr_dn9,)
    }
};
        var_nscr = assign41450_e54406;
        var_nscr_dn4 = assign41450_e54406_d_n4;
        var_nscr_dn6 = assign41450_e54406_d_n6;
        var_nscr_dn7 = assign41450_e54406_d_n7;
        var_nscr_dn8 = assign41450_e54406_d_n8;
        var_nscr_dn9 = assign41450_e54406_d_n9;

        let (assign41460_e54418, assign41460_e54418_d_n4, assign41460_e54418_d_n6, assign41460_e54418_d_n7, assign41460_e54418_d_n8, assign41460_e54418_d_n9,) = {
    if (var_guard1192 == 0.0) {
        let assign41460_e54412: f64 = (0.5 * var_gf);
        let assign41460_e54414: f64 = (var_xn_s).sqrt();
        let assign41460_e54415: f64 = (assign41460_e54412 / assign41460_e54414);
        let assign41460_e54416: f64 = (1.0 + assign41460_e54415);
        (assign41460_e54416, ((((0.5 * var_gf_dn4) * assign41460_e54414) - (assign41460_e54412 * (var_xn_s_dn4 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)), ((((0.5 * var_gf_dn6) * assign41460_e54414) - (assign41460_e54412 * (var_xn_s_dn6 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)), ((((0.5 * var_gf_dn7) * assign41460_e54414) - (assign41460_e54412 * (var_xn_s_dn7 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)), ((((0.5 * var_gf_dn8) * assign41460_e54414) - (assign41460_e54412 * (var_xn_s_dn8 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)), ((((0.5 * var_gf_dn9) * assign41460_e54414) - (assign41460_e54412 * (var_xn_s_dn9 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)),)
    } else {
        (var_nscr, var_nscr_dn4, var_nscr_dn6, var_nscr_dn7, var_nscr_dn8, var_nscr_dn9,)
    }
};
        var_nscr = assign41460_e54418;
        var_nscr_dn4 = assign41460_e54418_d_n4;
        var_nscr_dn6 = assign41460_e54418_d_n6;
        var_nscr_dn7 = assign41460_e54418_d_n7;
        var_nscr_dn8 = assign41460_e54418_d_n8;
        var_nscr_dn9 = assign41460_e54418_d_n9;

        let assign41470_e54422: f64 = (var_xn_s).sqrt();
        let assign41470_e54423: f64 = (var_gf * assign41470_e54422);
        let assign41470_e54424: f64 = (var_xn_s + assign41470_e54423);
        let assign41470_e54428: f64 = (var_nscr - 1.0);
        let assign41470_e54429: f64 = (assign41470_e54428).ln();
        let assign41470_e54430: f64 = (var_nscr * assign41470_e54429);
        let assign41470_e54431: f64 = (assign41470_e54424 - assign41470_e54430);
        var_xthscr = assign41470_e54431;
        var_xthscr_dn4 = ((var_xn_s_dn4 + ((var_gf_dn4 * assign41470_e54422) + (var_gf * (var_xn_s_dn4 / (2.0 * assign41470_e54422))))) - ((var_nscr_dn4 * assign41470_e54429) + (var_nscr * (var_nscr_dn4 / assign41470_e54428))));
        var_xthscr_dn6 = ((var_xn_s_dn6 + ((var_gf_dn6 * assign41470_e54422) + (var_gf * (var_xn_s_dn6 / (2.0 * assign41470_e54422))))) - ((var_nscr_dn6 * assign41470_e54429) + (var_nscr * (var_nscr_dn6 / assign41470_e54428))));
        var_xthscr_dn7 = ((var_xn_s_dn7 + ((var_gf_dn7 * assign41470_e54422) + (var_gf * (var_xn_s_dn7 / (2.0 * assign41470_e54422))))) - ((var_nscr_dn7 * assign41470_e54429) + (var_nscr * (var_nscr_dn7 / assign41470_e54428))));
        var_xthscr_dn8 = ((var_xn_s_dn8 + ((var_gf_dn8 * assign41470_e54422) + (var_gf * (var_xn_s_dn8 / (2.0 * assign41470_e54422))))) - ((var_nscr_dn8 * assign41470_e54429) + (var_nscr * (var_nscr_dn8 / assign41470_e54428))));
        var_xthscr_dn9 = ((var_xn_s_dn9 + ((var_gf_dn9 * assign41470_e54422) + (var_gf * (var_xn_s_dn9 / (2.0 * assign41470_e54422))))) - ((var_nscr_dn9 * assign41470_e54429) + (var_nscr * (var_nscr_dn9 / assign41470_e54428))));

        let assign41480_e54434: f64 = (var_xg - var_xthscr);
        let assign41480_e54436: f64 = (assign41480_e54434 / var_nscr);
        var_xgtscr = assign41480_e54436;
        var_xgtscr_dn4 = ((((var_xg_dn4 - var_xthscr_dn4) * var_nscr) - (assign41480_e54434 * var_nscr_dn4)) / (var_nscr * var_nscr));
        var_xgtscr_dn6 = ((((var_xg_dn6 - var_xthscr_dn6) * var_nscr) - (assign41480_e54434 * var_nscr_dn6)) / (var_nscr * var_nscr));
        var_xgtscr_dn7 = ((((var_xg_dn7 - var_xthscr_dn7) * var_nscr) - (assign41480_e54434 * var_nscr_dn7)) / (var_nscr * var_nscr));
        var_xgtscr_dn8 = ((((var_xg_dn8 - var_xthscr_dn8) * var_nscr) - (assign41480_e54434 * var_nscr_dn8)) / (var_nscr * var_nscr));
        var_xgtscr_dn9 = ((((var_xg_dn9 - var_xthscr_dn9) * var_nscr) - (assign41480_e54434 * var_nscr_dn9)) / (var_nscr * var_nscr));

        let assign41490_e54439: f64 = (0.5 * var_gf2);
        let assign41490_e54443: f64 = (8.0 / var_gf2);
        let assign41490_e54444: f64 = (1.0 + assign41490_e54443);
        let assign41490_e54445: f64 = (assign41490_e54444).sqrt();
        let assign41490_e54447: f64 = (assign41490_e54445 - 1.0);
        let assign41490_e54448: f64 = (assign41490_e54439 * assign41490_e54447);
        var_qbscr = assign41490_e54448;
        var_qbscr_dn4 = (((0.5 * var_gf2_dn4) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * var_gf2_dn4) / (var_gf2 * var_gf2))) / (2.0 * assign41490_e54445))));
        var_qbscr_dn6 = (((0.5 * var_gf2_dn6) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * var_gf2_dn6) / (var_gf2 * var_gf2))) / (2.0 * assign41490_e54445))));
        var_qbscr_dn7 = (((0.5 * var_gf2_dn7) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * var_gf2_dn7) / (var_gf2 * var_gf2))) / (2.0 * assign41490_e54445))));
        var_qbscr_dn8 = (((0.5 * var_gf2_dn8) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * var_gf2_dn8) / (var_gf2 * var_gf2))) / (2.0 * assign41490_e54445))));
        var_qbscr_dn9 = (((0.5 * var_gf2_dn9) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * var_gf2_dn9) / (var_gf2 * var_gf2))) / (2.0 * assign41490_e54445))));

        var_qiscr = 0.0;
        var_qiscr_dn4 = 0.0;
        var_qiscr_dn6 = 0.0;
        var_qiscr_dn7 = 0.0;
        var_qiscr_dn8 = 0.0;
        var_qiscr_dn9 = 0.0;

        var_fscr = 1.0;
        var_fscr_dn4 = 0.0;
        var_fscr_dn6 = 0.0;
        var_fscr_dn7 = 0.0;
        var_fscr_dn8 = 0.0;
        var_fscr_dn9 = 0.0;

        let assign41520_e54453: f64 = (-30.0);
        let assign41520_e54454: f64 = if var_xgtscr > assign41520_e54453 { 1.0 } else { 0.0 };
        var_guard1195 = assign41520_e54454;

        let (assign41530_e54462, assign41530_e54462_d_n4, assign41530_e54462_d_n6, assign41530_e54462_d_n7, assign41530_e54462_d_n8, assign41530_e54462_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41530_e54458: f64 = (var_nscr * var_xgtscr);
        let assign41530_e54460: f64 = (assign41530_e54458 - 1.0);
        (assign41530_e54460, ((var_nscr_dn4 * var_xgtscr) + (var_nscr * var_xgtscr_dn4)), ((var_nscr_dn6 * var_xgtscr) + (var_nscr * var_xgtscr_dn6)), ((var_nscr_dn7 * var_xgtscr) + (var_nscr * var_xgtscr_dn7)), ((var_nscr_dn8 * var_xgtscr) + (var_nscr * var_xgtscr_dn8)), ((var_nscr_dn9 * var_xgtscr) + (var_nscr * var_xgtscr_dn9)),)
    } else {
        (var_xgtscr0, var_xgtscr0_dn4, var_xgtscr0_dn6, var_xgtscr0_dn7, var_xgtscr0_dn8, var_xgtscr0_dn9,)
    }
};
        var_xgtscr0 = assign41530_e54462;
        var_xgtscr0_dn4 = assign41530_e54462_d_n4;
        var_xgtscr0_dn6 = assign41530_e54462_d_n6;
        var_xgtscr0_dn7 = assign41530_e54462_d_n7;
        var_xgtscr0_dn8 = assign41530_e54462_d_n8;
        var_xgtscr0_dn9 = assign41530_e54462_d_n9;

        let (assign41540_e54475, assign41540_e54475_d_n4, assign41540_e54475_d_n6, assign41540_e54475_d_n7, assign41540_e54475_d_n8, assign41540_e54475_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41540_e54468: f64 = (var_xgtscr0 * var_xgtscr0);
        let assign41540_e54470: f64 = (assign41540_e54468 + 10.0);
        let assign41540_e54471: f64 = (assign41540_e54470).sqrt();
        let assign41540_e54472: f64 = (var_xgtscr0 + assign41540_e54471);
        let assign41540_e54473: f64 = (0.5 * assign41540_e54472);
        (assign41540_e54473, (0.5 * (var_xgtscr0_dn4 + (((var_xgtscr0_dn4 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn4)) / (2.0 * assign41540_e54471)))), (0.5 * (var_xgtscr0_dn6 + (((var_xgtscr0_dn6 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn6)) / (2.0 * assign41540_e54471)))), (0.5 * (var_xgtscr0_dn7 + (((var_xgtscr0_dn7 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn7)) / (2.0 * assign41540_e54471)))), (0.5 * (var_xgtscr0_dn8 + (((var_xgtscr0_dn8 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn8)) / (2.0 * assign41540_e54471)))), (0.5 * (var_xgtscr0_dn9 + (((var_xgtscr0_dn9 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn9)) / (2.0 * assign41540_e54471)))),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign41540_e54475;
        var_temp__blk949_dn4 = assign41540_e54475_d_n4;
        var_temp__blk949_dn6 = assign41540_e54475_d_n6;
        var_temp__blk949_dn7 = assign41540_e54475_d_n7;
        var_temp__blk949_dn8 = assign41540_e54475_d_n8;
        var_temp__blk949_dn9 = assign41540_e54475_d_n9;

        let (assign41550_e54482, assign41550_e54482_d_n4, assign41550_e54482_d_n6, assign41550_e54482_d_n7, assign41550_e54482_d_n8, assign41550_e54482_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41550_e54479: f64 = (var_temp__blk949).ln();
        let assign41550_e54480: f64 = (var_xgtscr - assign41550_e54479);
        (assign41550_e54480, (var_xgtscr_dn4 - (var_temp__blk949_dn4 / var_temp__blk949)), (var_xgtscr_dn6 - (var_temp__blk949_dn6 / var_temp__blk949)), (var_xgtscr_dn7 - (var_temp__blk949_dn7 / var_temp__blk949)), (var_xgtscr_dn8 - (var_temp__blk949_dn8 / var_temp__blk949)), (var_xgtscr_dn9 - (var_temp__blk949_dn9 / var_temp__blk949)),)
    } else {
        (var_qiscr0si, var_qiscr0si_dn4, var_qiscr0si_dn6, var_qiscr0si_dn7, var_qiscr0si_dn8, var_qiscr0si_dn9,)
    }
};
        var_qiscr0si = assign41550_e54482;
        var_qiscr0si_dn4 = assign41550_e54482_d_n4;
        var_qiscr0si_dn6 = assign41550_e54482_d_n6;
        var_qiscr0si_dn7 = assign41550_e54482_d_n7;
        var_qiscr0si_dn8 = assign41550_e54482_d_n8;
        var_qiscr0si_dn9 = assign41550_e54482_d_n9;

        *var_ct_fact_slot = var_ct_fact;
        *var_ct_fact_dn4_slot = var_ct_fact_dn4;
        *var_ct_fact_dn6_slot = var_ct_fact_dn6;
        *var_ct_fact_dn7_slot = var_ct_fact_dn7;
        *var_ct_fact_dn8_slot = var_ct_fact_dn8;
        *var_ct_fact_dn9_slot = var_ct_fact_dn9;
        *var_dctg_slot = var_dctg;
        *var_dctg_dn4_slot = var_dctg_dn4;
        *var_dctg_dn6_slot = var_dctg_dn6;
        *var_dctg_dn7_slot = var_dctg_dn7;
        *var_dctg_dn8_slot = var_dctg_dn8;
        *var_dctg_dn9_slot = var_dctg_dn9;
        *var_delphib_slot = var_delphib;
        *var_delphib_dn4_slot = var_delphib_dn4;
        *var_delphib_dn6_slot = var_delphib_dn6;
        *var_delphib_dn7_slot = var_delphib_dn7;
        *var_delphib_dn8_slot = var_delphib_dn8;
        *var_delphib_dn9_slot = var_delphib_dn9;
        *var_delta_ns_slot = var_delta_ns;
        *var_delta_ns_dn4_slot = var_delta_ns_dn4;
        *var_delta_ns_dn6_slot = var_delta_ns_dn6;
        *var_delta_ns_dn7_slot = var_delta_ns_dn7;
        *var_delta_ns_dn8_slot = var_delta_ns_dn8;
        *var_delta_ns_dn9_slot = var_delta_ns_dn9;
        *var_delxb_slot = var_delxb;
        *var_delxb_dn4_slot = var_delxb_dn4;
        *var_delxb_dn6_slot = var_delxb_dn6;
        *var_delxb_dn7_slot = var_delxb_dn7;
        *var_delxb_dn8_slot = var_delxb_dn8;
        *var_delxb_dn9_slot = var_delxb_dn9;
        *var_dphit1_slot = var_dphit1;
        *var_dphit1_dn4_slot = var_dphit1_dn4;
        *var_dphit1_dn6_slot = var_dphit1_dn6;
        *var_dphit1_dn7_slot = var_dphit1_dn7;
        *var_dphit1_dn8_slot = var_dphit1_dn8;
        *var_dphit1_dn9_slot = var_dphit1_dn9;
        *var_fscr_slot = var_fscr;
        *var_fscr_dn4_slot = var_fscr_dn4;
        *var_fscr_dn6_slot = var_fscr_dn6;
        *var_fscr_dn7_slot = var_fscr_dn7;
        *var_fscr_dn8_slot = var_fscr_dn8;
        *var_fscr_dn9_slot = var_fscr_dn9;
        *var_gf_slot = var_gf;
        *var_gf2_slot = var_gf2;
        *var_gf2_dn4_slot = var_gf2_dn4;
        *var_gf2_dn6_slot = var_gf2_dn6;
        *var_gf2_dn7_slot = var_gf2_dn7;
        *var_gf2_dn8_slot = var_gf2_dn8;
        *var_gf2_dn9_slot = var_gf2_dn9;
        *var_gf_dn4_slot = var_gf_dn4;
        *var_gf_dn6_slot = var_gf_dn6;
        *var_gf_dn7_slot = var_gf_dn7;
        *var_gf_dn8_slot = var_gf_dn8;
        *var_gf_dn9_slot = var_gf_dn9;
        *var_guard1191_slot = var_guard1191;
        *var_guard1192_slot = var_guard1192;
        *var_guard1193_slot = var_guard1193;
        *var_guard1194_slot = var_guard1194;
        *var_guard1195_slot = var_guard1195;
        *var_inv_gf2_slot = var_inv_gf2;
        *var_inv_gf2_dn4_slot = var_inv_gf2_dn4;
        *var_inv_gf2_dn6_slot = var_inv_gf2_dn6;
        *var_inv_gf2_dn7_slot = var_inv_gf2_dn7;
        *var_inv_gf2_dn8_slot = var_inv_gf2_dn8;
        *var_inv_gf2_dn9_slot = var_inv_gf2_dn9;
        *var_inv_phit1_slot = var_inv_phit1;
        *var_inv_phit1_dn4_slot = var_inv_phit1_dn4;
        *var_inv_phit1_dn6_slot = var_inv_phit1_dn6;
        *var_inv_phit1_dn7_slot = var_inv_phit1_dn7;
        *var_inv_phit1_dn8_slot = var_inv_phit1_dn8;
        *var_inv_phit1_dn9_slot = var_inv_phit1_dn9;
        *var_nscr_slot = var_nscr;
        *var_nscr_dn4_slot = var_nscr_dn4;
        *var_nscr_dn6_slot = var_nscr_dn6;
        *var_nscr_dn7_slot = var_nscr_dn7;
        *var_nscr_dn8_slot = var_nscr_dn8;
        *var_nscr_dn9_slot = var_nscr_dn9;
        *var_phit1_slot = var_phit1;
        *var_phit1_dn4_slot = var_phit1_dn4;
        *var_phit1_dn6_slot = var_phit1_dn6;
        *var_phit1_dn7_slot = var_phit1_dn7;
        *var_phit1_dn8_slot = var_phit1_dn8;
        *var_phit1_dn9_slot = var_phit1_dn9;
        *var_phitct_slot = var_phitct;
        *var_phitct_dn4_slot = var_phitct_dn4;
        *var_phitct_dn6_slot = var_phitct_dn6;
        *var_phitct_dn7_slot = var_phitct_dn7;
        *var_phitct_dn8_slot = var_phitct_dn8;
        *var_phitct_dn9_slot = var_phitct_dn9;
        *var_qbscr_slot = var_qbscr;
        *var_qbscr_dn4_slot = var_qbscr_dn4;
        *var_qbscr_dn6_slot = var_qbscr_dn6;
        *var_qbscr_dn7_slot = var_qbscr_dn7;
        *var_qbscr_dn8_slot = var_qbscr_dn8;
        *var_qbscr_dn9_slot = var_qbscr_dn9;
        *var_qiscr_slot = var_qiscr;
        *var_qiscr0si_slot = var_qiscr0si;
        *var_qiscr0si_dn4_slot = var_qiscr0si_dn4;
        *var_qiscr0si_dn6_slot = var_qiscr0si_dn6;
        *var_qiscr0si_dn7_slot = var_qiscr0si_dn7;
        *var_qiscr0si_dn8_slot = var_qiscr0si_dn8;
        *var_qiscr0si_dn9_slot = var_qiscr0si_dn9;
        *var_qiscr_dn4_slot = var_qiscr_dn4;
        *var_qiscr_dn6_slot = var_qiscr_dn6;
        *var_qiscr_dn7_slot = var_qiscr_dn7;
        *var_qiscr_dn8_slot = var_qiscr_dn8;
        *var_qiscr_dn9_slot = var_qiscr_dn9;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp__blk949_slot = var_temp__blk949;
        *var_temp__blk949_dn4_slot = var_temp__blk949_dn4;
        *var_temp__blk949_dn6_slot = var_temp__blk949_dn6;
        *var_temp__blk949_dn7_slot = var_temp__blk949_dn7;
        *var_temp__blk949_dn8_slot = var_temp__blk949_dn8;
        *var_temp__blk949_dn9_slot = var_temp__blk949_dn9;
        *var_ux_slot = var_ux;
        *var_ux_dn4_slot = var_ux_dn4;
        *var_ux_dn6_slot = var_ux_dn6;
        *var_ux_dn7_slot = var_ux_dn7;
        *var_ux_dn8_slot = var_ux_dn8;
        *var_ux_dn9_slot = var_ux_dn9;
        *var_vdsp_slot = var_vdsp;
        *var_vdsp_dn7_slot = var_vdsp_dn7;
        *var_vdsp_dn8_slot = var_vdsp_dn8;
        *var_xb_slot = var_xb;
        *var_xb_dn4_slot = var_xb_dn4;
        *var_xb_dn6_slot = var_xb_dn6;
        *var_xb_dn7_slot = var_xb_dn7;
        *var_xb_dn8_slot = var_xb_dn8;
        *var_xb_dn9_slot = var_xb_dn9;
        *var_xct_slot = var_xct;
        *var_xct_dn4_slot = var_xct_dn4;
        *var_xct_dn6_slot = var_xct_dn6;
        *var_xct_dn7_slot = var_xct_dn7;
        *var_xct_dn8_slot = var_xct_dn8;
        *var_xct_dn9_slot = var_xct_dn9;
        *var_xg_slot = var_xg;
        *var_xg_dn4_slot = var_xg_dn4;
        *var_xg_dn6_slot = var_xg_dn6;
        *var_xg_dn7_slot = var_xg_dn7;
        *var_xg_dn8_slot = var_xg_dn8;
        *var_xg_dn9_slot = var_xg_dn9;
        *var_xgtscr_slot = var_xgtscr;
        *var_xgtscr0_slot = var_xgtscr0;
        *var_xgtscr0_dn4_slot = var_xgtscr0_dn4;
        *var_xgtscr0_dn6_slot = var_xgtscr0_dn6;
        *var_xgtscr0_dn7_slot = var_xgtscr0_dn7;
        *var_xgtscr0_dn8_slot = var_xgtscr0_dn8;
        *var_xgtscr0_dn9_slot = var_xgtscr0_dn9;
        *var_xgtscr_dn4_slot = var_xgtscr_dn4;
        *var_xgtscr_dn6_slot = var_xgtscr_dn6;
        *var_xgtscr_dn7_slot = var_xgtscr_dn7;
        *var_xgtscr_dn8_slot = var_xgtscr_dn8;
        *var_xgtscr_dn9_slot = var_xgtscr_dn9;
        *var_xn_s_slot = var_xn_s;
        *var_xn_s_dn4_slot = var_xn_s_dn4;
        *var_xn_s_dn6_slot = var_xn_s_dn6;
        *var_xn_s_dn7_slot = var_xn_s_dn7;
        *var_xn_s_dn8_slot = var_xn_s_dn8;
        *var_xn_s_dn9_slot = var_xn_s_dn9;
        *var_xno_s_slot = var_xno_s;
        *var_xno_s_dn4_slot = var_xno_s_dn4;
        *var_xno_s_dn6_slot = var_xno_s_dn6;
        *var_xno_s_dn7_slot = var_xno_s_dn7;
        *var_xno_s_dn8_slot = var_xno_s_dn8;
        *var_xno_s_dn9_slot = var_xno_s_dn9;
        *var_xthscr_slot = var_xthscr;
        *var_xthscr_dn4_slot = var_xthscr_dn4;
        *var_xthscr_dn6_slot = var_xthscr_dn6;
        *var_xthscr_dn7_slot = var_xthscr_dn7;
        *var_xthscr_dn8_slot = var_xthscr_dn8;
        *var_xthscr_dn9_slot = var_xthscr_dn9;
    }

    pub(super) fn stamp_transient_block_89(
        var_delxb: f64,
        var_delxb_dn4: f64,
        var_delxb_dn6: f64,
        var_delxb_dn7: f64,
        var_delxb_dn8: f64,
        var_delxb_dn9: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn4: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf2_dn9: f64,
        var_gf_dn4: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_gf_dn9: f64,
        var_guard1195: f64,
        var_inv_gf2: f64,
        var_inv_gf2_dn4: f64,
        var_inv_gf2_dn6: f64,
        var_inv_gf2_dn7: f64,
        var_inv_gf2_dn8: f64,
        var_inv_gf2_dn9: f64,
        var_nscr: f64,
        var_nscr_dn4: f64,
        var_nscr_dn6: f64,
        var_nscr_dn7: f64,
        var_nscr_dn8: f64,
        var_nscr_dn9: f64,
        var_qiscr0si: f64,
        var_qiscr0si_dn4: f64,
        var_qiscr0si_dn6: f64,
        var_qiscr0si_dn7: f64,
        var_qiscr0si_dn8: f64,
        var_qiscr0si_dn9: f64,
        var_xg: f64,
        var_xg_dn4: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xg_dn9: f64,
        var_xgtscr: f64,
        var_xgtscr_dn4: f64,
        var_xgtscr_dn6: f64,
        var_xgtscr_dn7: f64,
        var_xgtscr_dn8: f64,
        var_xgtscr_dn9: f64,
        var_xno_s: f64,
        var_xno_s_dn4: f64,
        var_xno_s_dn6: f64,
        var_xno_s_dn7: f64,
        var_xno_s_dn8: f64,
        var_xno_s_dn9: f64,
        var_delta_ns_slot: &mut f64,
        var_delta_ns_dn4_slot: &mut f64,
        var_delta_ns_dn6_slot: &mut f64,
        var_delta_ns_dn7_slot: &mut f64,
        var_delta_ns_dn8_slot: &mut f64,
        var_delta_ns_dn9_slot: &mut f64,
        var_dscr0_slot: &mut f64,
        var_dscr0_dn4_slot: &mut f64,
        var_dscr0_dn6_slot: &mut f64,
        var_dscr0_dn7_slot: &mut f64,
        var_dscr0_dn8_slot: &mut f64,
        var_dscr0_dn9_slot: &mut f64,
        var_fscr_slot: &mut f64,
        var_fscr_dn4_slot: &mut f64,
        var_fscr_dn6_slot: &mut f64,
        var_fscr_dn7_slot: &mut f64,
        var_fscr_dn8_slot: &mut f64,
        var_fscr_dn9_slot: &mut f64,
        var_guard1196_slot: &mut f64,
        var_guard1197_slot: &mut f64,
        var_guard1198_slot: &mut f64,
        var_guard1199_slot: &mut f64,
        var_guard1200_slot: &mut f64,
        var_inv_xi_slot: &mut f64,
        var_inv_xi_dn4_slot: &mut f64,
        var_inv_xi_dn6_slot: &mut f64,
        var_inv_xi_dn7_slot: &mut f64,
        var_inv_xi_dn8_slot: &mut f64,
        var_inv_xi_dn9_slot: &mut f64,
        var_margin_slot: &mut f64,
        var_mutau_slot: &mut f64,
        var_mutau_dn4_slot: &mut f64,
        var_mutau_dn6_slot: &mut f64,
        var_mutau_dn7_slot: &mut f64,
        var_mutau_dn8_slot: &mut f64,
        var_mutau_dn9_slot: &mut f64,
        var_nu_slot: &mut f64,
        var_nu_dn4_slot: &mut f64,
        var_nu_dn6_slot: &mut f64,
        var_nu_dn7_slot: &mut f64,
        var_nu_dn8_slot: &mut f64,
        var_nu_dn9_slot: &mut f64,
        var_qbscr_slot: &mut f64,
        var_qbscr_dn4_slot: &mut f64,
        var_qbscr_dn6_slot: &mut f64,
        var_qbscr_dn7_slot: &mut f64,
        var_qbscr_dn8_slot: &mut f64,
        var_qbscr_dn9_slot: &mut f64,
        var_qiscr_slot: &mut f64,
        var_qiscr0_slot: &mut f64,
        var_qiscr0_dn4_slot: &mut f64,
        var_qiscr0_dn6_slot: &mut f64,
        var_qiscr0_dn7_slot: &mut f64,
        var_qiscr0_dn8_slot: &mut f64,
        var_qiscr0_dn9_slot: &mut f64,
        var_qiscr_dn4_slot: &mut f64,
        var_qiscr_dn6_slot: &mut f64,
        var_qiscr_dn7_slot: &mut f64,
        var_qiscr_dn8_slot: &mut f64,
        var_qiscr_dn9_slot: &mut f64,
        var_sp_s_a_slot: &mut f64,
        var_sp_s_a_dn4_slot: &mut f64,
        var_sp_s_a_dn6_slot: &mut f64,
        var_sp_s_a_dn7_slot: &mut f64,
        var_sp_s_a_dn8_slot: &mut f64,
        var_sp_s_a_dn9_slot: &mut f64,
        var_sp_s_c_slot: &mut f64,
        var_sp_s_c_dn4_slot: &mut f64,
        var_sp_s_c_dn6_slot: &mut f64,
        var_sp_s_c_dn7_slot: &mut f64,
        var_sp_s_c_dn8_slot: &mut f64,
        var_sp_s_c_dn9_slot: &mut f64,
        var_sp_s_eta_slot: &mut f64,
        var_sp_s_eta_dn4_slot: &mut f64,
        var_sp_s_eta_dn6_slot: &mut f64,
        var_sp_s_eta_dn7_slot: &mut f64,
        var_sp_s_eta_dn8_slot: &mut f64,
        var_sp_s_eta_dn9_slot: &mut f64,
        var_sp_s_tau_slot: &mut f64,
        var_sp_s_tau_dn4_slot: &mut f64,
        var_sp_s_tau_dn6_slot: &mut f64,
        var_sp_s_tau_dn7_slot: &mut f64,
        var_sp_s_tau_dn8_slot: &mut f64,
        var_sp_s_tau_dn9_slot: &mut f64,
        var_sp_s_temp_slot: &mut f64,
        var_sp_s_temp1_slot: &mut f64,
        var_sp_s_temp1_dn4_slot: &mut f64,
        var_sp_s_temp1_dn6_slot: &mut f64,
        var_sp_s_temp1_dn7_slot: &mut f64,
        var_sp_s_temp1_dn8_slot: &mut f64,
        var_sp_s_temp1_dn9_slot: &mut f64,
        var_sp_s_temp_dn4_slot: &mut f64,
        var_sp_s_temp_dn6_slot: &mut f64,
        var_sp_s_temp_dn7_slot: &mut f64,
        var_sp_s_temp_dn8_slot: &mut f64,
        var_sp_s_temp_dn9_slot: &mut f64,
        var_sp_s_x1_slot: &mut f64,
        var_sp_s_x1_dn4_slot: &mut f64,
        var_sp_s_x1_dn6_slot: &mut f64,
        var_sp_s_x1_dn7_slot: &mut f64,
        var_sp_s_x1_dn8_slot: &mut f64,
        var_sp_s_x1_dn9_slot: &mut f64,
        var_sp_s_yg_slot: &mut f64,
        var_sp_s_yg_dn4_slot: &mut f64,
        var_sp_s_yg_dn6_slot: &mut f64,
        var_sp_s_yg_dn7_slot: &mut f64,
        var_sp_s_yg_dn8_slot: &mut f64,
        var_sp_s_yg_dn9_slot: &mut f64,
        var_sp_s_ysub_slot: &mut f64,
        var_sp_s_ysub_dn4_slot: &mut f64,
        var_sp_s_ysub_dn6_slot: &mut f64,
        var_sp_s_ysub_dn7_slot: &mut f64,
        var_sp_s_ysub_dn8_slot: &mut f64,
        var_sp_s_ysub_dn9_slot: &mut f64,
        var_temp__blk949_slot: &mut f64,
        var_temp__blk949_dn4_slot: &mut f64,
        var_temp__blk949_dn6_slot: &mut f64,
        var_temp__blk949_dn7_slot: &mut f64,
        var_temp__blk949_dn8_slot: &mut f64,
        var_temp__blk949_dn9_slot: &mut f64,
        var_x_s_slot: &mut f64,
        var_x_s_dn4_slot: &mut f64,
        var_x_s_dn6_slot: &mut f64,
        var_x_s_dn7_slot: &mut f64,
        var_x_s_dn8_slot: &mut f64,
        var_x_s_dn9_slot: &mut f64,
        var_xi_slot: &mut f64,
        var_xi_dn4_slot: &mut f64,
        var_xi_dn6_slot: &mut f64,
        var_xi_dn7_slot: &mut f64,
        var_xi_dn8_slot: &mut f64,
        var_xi_dn9_slot: &mut f64,
        var_xn_s_slot: &mut f64,
        var_xn_s_dn4_slot: &mut f64,
        var_xn_s_dn6_slot: &mut f64,
        var_xn_s_dn7_slot: &mut f64,
        var_xn_s_dn8_slot: &mut f64,
        var_xn_s_dn9_slot: &mut f64,
    ) {
        let mut var_delta_ns: f64 = *var_delta_ns_slot;
        let mut var_delta_ns_dn4: f64 = *var_delta_ns_dn4_slot;
        let mut var_delta_ns_dn6: f64 = *var_delta_ns_dn6_slot;
        let mut var_delta_ns_dn7: f64 = *var_delta_ns_dn7_slot;
        let mut var_delta_ns_dn8: f64 = *var_delta_ns_dn8_slot;
        let mut var_delta_ns_dn9: f64 = *var_delta_ns_dn9_slot;
        let mut var_dscr0: f64 = *var_dscr0_slot;
        let mut var_dscr0_dn4: f64 = *var_dscr0_dn4_slot;
        let mut var_dscr0_dn6: f64 = *var_dscr0_dn6_slot;
        let mut var_dscr0_dn7: f64 = *var_dscr0_dn7_slot;
        let mut var_dscr0_dn8: f64 = *var_dscr0_dn8_slot;
        let mut var_dscr0_dn9: f64 = *var_dscr0_dn9_slot;
        let mut var_fscr: f64 = *var_fscr_slot;
        let mut var_fscr_dn4: f64 = *var_fscr_dn4_slot;
        let mut var_fscr_dn6: f64 = *var_fscr_dn6_slot;
        let mut var_fscr_dn7: f64 = *var_fscr_dn7_slot;
        let mut var_fscr_dn8: f64 = *var_fscr_dn8_slot;
        let mut var_fscr_dn9: f64 = *var_fscr_dn9_slot;
        let mut var_guard1196: f64 = *var_guard1196_slot;
        let mut var_guard1197: f64 = *var_guard1197_slot;
        let mut var_guard1198: f64 = *var_guard1198_slot;
        let mut var_guard1199: f64 = *var_guard1199_slot;
        let mut var_guard1200: f64 = *var_guard1200_slot;
        let mut var_inv_xi: f64 = *var_inv_xi_slot;
        let mut var_inv_xi_dn4: f64 = *var_inv_xi_dn4_slot;
        let mut var_inv_xi_dn6: f64 = *var_inv_xi_dn6_slot;
        let mut var_inv_xi_dn7: f64 = *var_inv_xi_dn7_slot;
        let mut var_inv_xi_dn8: f64 = *var_inv_xi_dn8_slot;
        let mut var_inv_xi_dn9: f64 = *var_inv_xi_dn9_slot;
        let mut var_margin: f64 = *var_margin_slot;
        let mut var_mutau: f64 = *var_mutau_slot;
        let mut var_mutau_dn4: f64 = *var_mutau_dn4_slot;
        let mut var_mutau_dn6: f64 = *var_mutau_dn6_slot;
        let mut var_mutau_dn7: f64 = *var_mutau_dn7_slot;
        let mut var_mutau_dn8: f64 = *var_mutau_dn8_slot;
        let mut var_mutau_dn9: f64 = *var_mutau_dn9_slot;
        let mut var_nu: f64 = *var_nu_slot;
        let mut var_nu_dn4: f64 = *var_nu_dn4_slot;
        let mut var_nu_dn6: f64 = *var_nu_dn6_slot;
        let mut var_nu_dn7: f64 = *var_nu_dn7_slot;
        let mut var_nu_dn8: f64 = *var_nu_dn8_slot;
        let mut var_nu_dn9: f64 = *var_nu_dn9_slot;
        let mut var_qbscr: f64 = *var_qbscr_slot;
        let mut var_qbscr_dn4: f64 = *var_qbscr_dn4_slot;
        let mut var_qbscr_dn6: f64 = *var_qbscr_dn6_slot;
        let mut var_qbscr_dn7: f64 = *var_qbscr_dn7_slot;
        let mut var_qbscr_dn8: f64 = *var_qbscr_dn8_slot;
        let mut var_qbscr_dn9: f64 = *var_qbscr_dn9_slot;
        let mut var_qiscr: f64 = *var_qiscr_slot;
        let mut var_qiscr0: f64 = *var_qiscr0_slot;
        let mut var_qiscr0_dn4: f64 = *var_qiscr0_dn4_slot;
        let mut var_qiscr0_dn6: f64 = *var_qiscr0_dn6_slot;
        let mut var_qiscr0_dn7: f64 = *var_qiscr0_dn7_slot;
        let mut var_qiscr0_dn8: f64 = *var_qiscr0_dn8_slot;
        let mut var_qiscr0_dn9: f64 = *var_qiscr0_dn9_slot;
        let mut var_qiscr_dn4: f64 = *var_qiscr_dn4_slot;
        let mut var_qiscr_dn6: f64 = *var_qiscr_dn6_slot;
        let mut var_qiscr_dn7: f64 = *var_qiscr_dn7_slot;
        let mut var_qiscr_dn8: f64 = *var_qiscr_dn8_slot;
        let mut var_qiscr_dn9: f64 = *var_qiscr_dn9_slot;
        let mut var_sp_s_a: f64 = *var_sp_s_a_slot;
        let mut var_sp_s_a_dn4: f64 = *var_sp_s_a_dn4_slot;
        let mut var_sp_s_a_dn6: f64 = *var_sp_s_a_dn6_slot;
        let mut var_sp_s_a_dn7: f64 = *var_sp_s_a_dn7_slot;
        let mut var_sp_s_a_dn8: f64 = *var_sp_s_a_dn8_slot;
        let mut var_sp_s_a_dn9: f64 = *var_sp_s_a_dn9_slot;
        let mut var_sp_s_c: f64 = *var_sp_s_c_slot;
        let mut var_sp_s_c_dn4: f64 = *var_sp_s_c_dn4_slot;
        let mut var_sp_s_c_dn6: f64 = *var_sp_s_c_dn6_slot;
        let mut var_sp_s_c_dn7: f64 = *var_sp_s_c_dn7_slot;
        let mut var_sp_s_c_dn8: f64 = *var_sp_s_c_dn8_slot;
        let mut var_sp_s_c_dn9: f64 = *var_sp_s_c_dn9_slot;
        let mut var_sp_s_eta: f64 = *var_sp_s_eta_slot;
        let mut var_sp_s_eta_dn4: f64 = *var_sp_s_eta_dn4_slot;
        let mut var_sp_s_eta_dn6: f64 = *var_sp_s_eta_dn6_slot;
        let mut var_sp_s_eta_dn7: f64 = *var_sp_s_eta_dn7_slot;
        let mut var_sp_s_eta_dn8: f64 = *var_sp_s_eta_dn8_slot;
        let mut var_sp_s_eta_dn9: f64 = *var_sp_s_eta_dn9_slot;
        let mut var_sp_s_tau: f64 = *var_sp_s_tau_slot;
        let mut var_sp_s_tau_dn4: f64 = *var_sp_s_tau_dn4_slot;
        let mut var_sp_s_tau_dn6: f64 = *var_sp_s_tau_dn6_slot;
        let mut var_sp_s_tau_dn7: f64 = *var_sp_s_tau_dn7_slot;
        let mut var_sp_s_tau_dn8: f64 = *var_sp_s_tau_dn8_slot;
        let mut var_sp_s_tau_dn9: f64 = *var_sp_s_tau_dn9_slot;
        let mut var_sp_s_temp: f64 = *var_sp_s_temp_slot;
        let mut var_sp_s_temp1: f64 = *var_sp_s_temp1_slot;
        let mut var_sp_s_temp1_dn4: f64 = *var_sp_s_temp1_dn4_slot;
        let mut var_sp_s_temp1_dn6: f64 = *var_sp_s_temp1_dn6_slot;
        let mut var_sp_s_temp1_dn7: f64 = *var_sp_s_temp1_dn7_slot;
        let mut var_sp_s_temp1_dn8: f64 = *var_sp_s_temp1_dn8_slot;
        let mut var_sp_s_temp1_dn9: f64 = *var_sp_s_temp1_dn9_slot;
        let mut var_sp_s_temp_dn4: f64 = *var_sp_s_temp_dn4_slot;
        let mut var_sp_s_temp_dn6: f64 = *var_sp_s_temp_dn6_slot;
        let mut var_sp_s_temp_dn7: f64 = *var_sp_s_temp_dn7_slot;
        let mut var_sp_s_temp_dn8: f64 = *var_sp_s_temp_dn8_slot;
        let mut var_sp_s_temp_dn9: f64 = *var_sp_s_temp_dn9_slot;
        let mut var_sp_s_x1: f64 = *var_sp_s_x1_slot;
        let mut var_sp_s_x1_dn4: f64 = *var_sp_s_x1_dn4_slot;
        let mut var_sp_s_x1_dn6: f64 = *var_sp_s_x1_dn6_slot;
        let mut var_sp_s_x1_dn7: f64 = *var_sp_s_x1_dn7_slot;
        let mut var_sp_s_x1_dn8: f64 = *var_sp_s_x1_dn8_slot;
        let mut var_sp_s_x1_dn9: f64 = *var_sp_s_x1_dn9_slot;
        let mut var_sp_s_yg: f64 = *var_sp_s_yg_slot;
        let mut var_sp_s_yg_dn4: f64 = *var_sp_s_yg_dn4_slot;
        let mut var_sp_s_yg_dn6: f64 = *var_sp_s_yg_dn6_slot;
        let mut var_sp_s_yg_dn7: f64 = *var_sp_s_yg_dn7_slot;
        let mut var_sp_s_yg_dn8: f64 = *var_sp_s_yg_dn8_slot;
        let mut var_sp_s_yg_dn9: f64 = *var_sp_s_yg_dn9_slot;
        let mut var_sp_s_ysub: f64 = *var_sp_s_ysub_slot;
        let mut var_sp_s_ysub_dn4: f64 = *var_sp_s_ysub_dn4_slot;
        let mut var_sp_s_ysub_dn6: f64 = *var_sp_s_ysub_dn6_slot;
        let mut var_sp_s_ysub_dn7: f64 = *var_sp_s_ysub_dn7_slot;
        let mut var_sp_s_ysub_dn8: f64 = *var_sp_s_ysub_dn8_slot;
        let mut var_sp_s_ysub_dn9: f64 = *var_sp_s_ysub_dn9_slot;
        let mut var_temp__blk949: f64 = *var_temp__blk949_slot;
        let mut var_temp__blk949_dn4: f64 = *var_temp__blk949_dn4_slot;
        let mut var_temp__blk949_dn6: f64 = *var_temp__blk949_dn6_slot;
        let mut var_temp__blk949_dn7: f64 = *var_temp__blk949_dn7_slot;
        let mut var_temp__blk949_dn8: f64 = *var_temp__blk949_dn8_slot;
        let mut var_temp__blk949_dn9: f64 = *var_temp__blk949_dn9_slot;
        let mut var_x_s: f64 = *var_x_s_slot;
        let mut var_x_s_dn4: f64 = *var_x_s_dn4_slot;
        let mut var_x_s_dn6: f64 = *var_x_s_dn6_slot;
        let mut var_x_s_dn7: f64 = *var_x_s_dn7_slot;
        let mut var_x_s_dn8: f64 = *var_x_s_dn8_slot;
        let mut var_x_s_dn9: f64 = *var_x_s_dn9_slot;
        let mut var_xi: f64 = *var_xi_slot;
        let mut var_xi_dn4: f64 = *var_xi_dn4_slot;
        let mut var_xi_dn6: f64 = *var_xi_dn6_slot;
        let mut var_xi_dn7: f64 = *var_xi_dn7_slot;
        let mut var_xi_dn8: f64 = *var_xi_dn8_slot;
        let mut var_xi_dn9: f64 = *var_xi_dn9_slot;
        let mut var_xn_s: f64 = *var_xn_s_slot;
        let mut var_xn_s_dn4: f64 = *var_xn_s_dn4_slot;
        let mut var_xn_s_dn6: f64 = *var_xn_s_dn6_slot;
        let mut var_xn_s_dn7: f64 = *var_xn_s_dn7_slot;
        let mut var_xn_s_dn8: f64 = *var_xn_s_dn8_slot;
        let mut var_xn_s_dn9: f64 = *var_xn_s_dn9_slot;

        let (assign41560_e54495, assign41560_e54495_d_n4, assign41560_e54495_d_n6, assign41560_e54495_d_n7, assign41560_e54495_d_n8, assign41560_e54495_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41560_e54488: f64 = (var_qiscr0si * var_qiscr0si);
        let assign41560_e54490: f64 = (assign41560_e54488 + 2.0);
        let assign41560_e54491: f64 = (assign41560_e54490).sqrt();
        let assign41560_e54492: f64 = (var_qiscr0si + assign41560_e54491);
        let assign41560_e54493: f64 = (0.5 * assign41560_e54492);
        (assign41560_e54493, (0.5 * (var_qiscr0si_dn4 + (((var_qiscr0si_dn4 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn4)) / (2.0 * assign41560_e54491)))), (0.5 * (var_qiscr0si_dn6 + (((var_qiscr0si_dn6 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn6)) / (2.0 * assign41560_e54491)))), (0.5 * (var_qiscr0si_dn7 + (((var_qiscr0si_dn7 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn7)) / (2.0 * assign41560_e54491)))), (0.5 * (var_qiscr0si_dn8 + (((var_qiscr0si_dn8 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn8)) / (2.0 * assign41560_e54491)))), (0.5 * (var_qiscr0si_dn9 + (((var_qiscr0si_dn9 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn9)) / (2.0 * assign41560_e54491)))),)
    } else {
        (var_qiscr0, var_qiscr0_dn4, var_qiscr0_dn6, var_qiscr0_dn7, var_qiscr0_dn8, var_qiscr0_dn9,)
    }
};
        var_qiscr0 = assign41560_e54495;
        var_qiscr0_dn4 = assign41560_e54495_d_n4;
        var_qiscr0_dn6 = assign41560_e54495_d_n6;
        var_qiscr0_dn7 = assign41560_e54495_d_n7;
        var_qiscr0_dn8 = assign41560_e54495_d_n8;
        var_qiscr0_dn9 = assign41560_e54495_d_n9;

        let assign41570_e54498: f64 = (var_xgtscr - var_qiscr0);
        let assign41570_e54500: f64 = if assign41570_e54498 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1196 = assign41570_e54500;

        let (assign41580_e54509, assign41580_e54509_d_n4, assign41580_e54509_d_n6, assign41580_e54509_d_n7, assign41580_e54509_d_n8, assign41580_e54509_d_n9,) = {
    if ((var_guard1195 != 0.0) && (var_guard1196 != 0.0)) {
        let assign41580_e54506: f64 = (var_xgtscr - var_qiscr0);
        let assign41580_e54507: f64 = (assign41580_e54506).exp();
        (assign41580_e54507, (assign41580_e54507 * (var_xgtscr_dn4 - var_qiscr0_dn4)), (assign41580_e54507 * (var_xgtscr_dn6 - var_qiscr0_dn6)), (assign41580_e54507 * (var_xgtscr_dn7 - var_qiscr0_dn7)), (assign41580_e54507 * (var_xgtscr_dn8 - var_qiscr0_dn8)), (assign41580_e54507 * (var_xgtscr_dn9 - var_qiscr0_dn9)),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign41580_e54509;
        var_temp__blk949_dn4 = assign41580_e54509_d_n4;
        var_temp__blk949_dn6 = assign41580_e54509_d_n6;
        var_temp__blk949_dn7 = assign41580_e54509_d_n7;
        var_temp__blk949_dn8 = assign41580_e54509_d_n8;
        var_temp__blk949_dn9 = assign41580_e54509_d_n9;

        let (assign41590_e54544, assign41590_e54544_d_n4, assign41590_e54544_d_n6, assign41590_e54544_d_n7, assign41590_e54544_d_n8, assign41590_e54544_d_n9,) = {
    if ((var_guard1195 != 0.0) && (var_guard1196 == 0.0)) {
        let assign41590_e54518: f64 = (var_xgtscr - var_qiscr0);
        let assign41590_e54520: f64 = (assign41590_e54518 - 230.25850929940458);
        let assign41590_e54525: f64 = (var_xgtscr - var_qiscr0);
        let assign41590_e54527: f64 = (assign41590_e54525 - 230.25850929940458);
        let assign41590_e54531: f64 = (var_xgtscr - var_qiscr0);
        let assign41590_e54533: f64 = (assign41590_e54531 - 230.25850929940458);
        let assign41590_e54535: f64 = (assign41590_e54533 * 0.3333333333333333);
        let assign41590_e54536: f64 = (1.0 + assign41590_e54535);
        let assign41590_e54537: f64 = (assign41590_e54527 * assign41590_e54536);
        let assign41590_e54538: f64 = (0.5 * assign41590_e54537);
        let assign41590_e54539: f64 = (1.0 + assign41590_e54538);
        let assign41590_e54540: f64 = (assign41590_e54520 * assign41590_e54539);
        let assign41590_e54541: f64 = (1.0 + assign41590_e54540);
        let assign41590_e54542: f64 = (1e100 * assign41590_e54541);
        (assign41590_e54542, (1e100 * (((var_xgtscr_dn4 - var_qiscr0_dn4) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((var_xgtscr_dn4 - var_qiscr0_dn4) * assign41590_e54536) + (assign41590_e54527 * ((var_xgtscr_dn4 - var_qiscr0_dn4) * 0.3333333333333333))))))), (1e100 * (((var_xgtscr_dn6 - var_qiscr0_dn6) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((var_xgtscr_dn6 - var_qiscr0_dn6) * assign41590_e54536) + (assign41590_e54527 * ((var_xgtscr_dn6 - var_qiscr0_dn6) * 0.3333333333333333))))))), (1e100 * (((var_xgtscr_dn7 - var_qiscr0_dn7) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((var_xgtscr_dn7 - var_qiscr0_dn7) * assign41590_e54536) + (assign41590_e54527 * ((var_xgtscr_dn7 - var_qiscr0_dn7) * 0.3333333333333333))))))), (1e100 * (((var_xgtscr_dn8 - var_qiscr0_dn8) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((var_xgtscr_dn8 - var_qiscr0_dn8) * assign41590_e54536) + (assign41590_e54527 * ((var_xgtscr_dn8 - var_qiscr0_dn8) * 0.3333333333333333))))))), (1e100 * (((var_xgtscr_dn9 - var_qiscr0_dn9) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((var_xgtscr_dn9 - var_qiscr0_dn9) * assign41590_e54536) + (assign41590_e54527 * ((var_xgtscr_dn9 - var_qiscr0_dn9) * 0.3333333333333333))))))),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign41590_e54544;
        var_temp__blk949_dn4 = assign41590_e54544_d_n4;
        var_temp__blk949_dn6 = assign41590_e54544_d_n6;
        var_temp__blk949_dn7 = assign41590_e54544_d_n7;
        var_temp__blk949_dn8 = assign41590_e54544_d_n8;
        var_temp__blk949_dn9 = assign41590_e54544_d_n9;

        let (assign41600_e54550, assign41600_e54550_d_n4, assign41600_e54550_d_n6, assign41600_e54550_d_n7, assign41600_e54550_d_n8, assign41600_e54550_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41600_e54548: f64 = (var_temp__blk949 / var_nscr);
        (assign41600_e54548, (((var_temp__blk949_dn4 * var_nscr) - (var_temp__blk949 * var_nscr_dn4)) / (var_nscr * var_nscr)), (((var_temp__blk949_dn6 * var_nscr) - (var_temp__blk949 * var_nscr_dn6)) / (var_nscr * var_nscr)), (((var_temp__blk949_dn7 * var_nscr) - (var_temp__blk949 * var_nscr_dn7)) / (var_nscr * var_nscr)), (((var_temp__blk949_dn8 * var_nscr) - (var_temp__blk949 * var_nscr_dn8)) / (var_nscr * var_nscr)), (((var_temp__blk949_dn9 * var_nscr) - (var_temp__blk949 * var_nscr_dn9)) / (var_nscr * var_nscr)),)
    } else {
        (var_dscr0, var_dscr0_dn4, var_dscr0_dn6, var_dscr0_dn7, var_dscr0_dn8, var_dscr0_dn9,)
    }
};
        var_dscr0 = assign41600_e54550;
        var_dscr0_dn4 = assign41600_e54550_d_n4;
        var_dscr0_dn6 = assign41600_e54550_d_n6;
        var_dscr0_dn7 = assign41600_e54550_d_n7;
        var_dscr0_dn8 = assign41600_e54550_d_n8;
        var_dscr0_dn9 = assign41600_e54550_d_n9;

        let (assign41610_e54560, assign41610_e54560_d_n4, assign41610_e54560_d_n6, assign41610_e54560_d_n7, assign41610_e54560_d_n8, assign41610_e54560_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41610_e54555: f64 = (var_qiscr0 + 1.0);
        let assign41610_e54556: f64 = (2.0 * assign41610_e54555);
        let assign41610_e54558: f64 = (assign41610_e54556 - var_dscr0);
        (assign41610_e54558, ((2.0 * var_qiscr0_dn4) - var_dscr0_dn4), ((2.0 * var_qiscr0_dn6) - var_dscr0_dn6), ((2.0 * var_qiscr0_dn7) - var_dscr0_dn7), ((2.0 * var_qiscr0_dn8) - var_dscr0_dn8), ((2.0 * var_qiscr0_dn9) - var_dscr0_dn9),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign41610_e54560;
        var_temp__blk949_dn4 = assign41610_e54560_d_n4;
        var_temp__blk949_dn6 = assign41610_e54560_d_n6;
        var_temp__blk949_dn7 = assign41610_e54560_d_n7;
        var_temp__blk949_dn8 = assign41610_e54560_d_n8;
        var_temp__blk949_dn9 = assign41610_e54560_d_n9;

        let assign41620_e54563: f64 = if var_dscr0 > 1e-6 { 1.0 } else { 0.0 };
        var_guard1197 = assign41620_e54563;

        let (assign41630_e54584, assign41630_e54584_d_n4, assign41630_e54584_d_n6, assign41630_e54584_d_n7, assign41630_e54584_d_n8, assign41630_e54584_d_n9,) = {
    if ((var_guard1195 != 0.0) && (var_guard1197 != 0.0)) {
        let assign41630_e54572: f64 = (var_dscr0 * var_temp__blk949);
        let assign41630_e54573: f64 = (1.0 + assign41630_e54572);
        let assign41630_e54574: f64 = (assign41630_e54573).sqrt();
        let assign41630_e54576: f64 = (assign41630_e54574 - 1.0);
        let assign41630_e54578: f64 = (assign41630_e54576 / var_dscr0);
        let assign41630_e54579: f64 = (var_qiscr0 - assign41630_e54578);
        let assign41630_e54581: f64 = (assign41630_e54579 + 1.0);
        let assign41630_e54582: f64 = (var_nscr * assign41630_e54581);
        (assign41630_e54582, ((var_nscr_dn4 * assign41630_e54581) + (var_nscr * (var_qiscr0_dn4 - ((((((var_dscr0_dn4 * var_temp__blk949) + (var_dscr0 * var_temp__blk949_dn4)) / (2.0 * assign41630_e54574)) * var_dscr0) - (assign41630_e54576 * var_dscr0_dn4)) / (var_dscr0 * var_dscr0))))), ((var_nscr_dn6 * assign41630_e54581) + (var_nscr * (var_qiscr0_dn6 - ((((((var_dscr0_dn6 * var_temp__blk949) + (var_dscr0 * var_temp__blk949_dn6)) / (2.0 * assign41630_e54574)) * var_dscr0) - (assign41630_e54576 * var_dscr0_dn6)) / (var_dscr0 * var_dscr0))))), ((var_nscr_dn7 * assign41630_e54581) + (var_nscr * (var_qiscr0_dn7 - ((((((var_dscr0_dn7 * var_temp__blk949) + (var_dscr0 * var_temp__blk949_dn7)) / (2.0 * assign41630_e54574)) * var_dscr0) - (assign41630_e54576 * var_dscr0_dn7)) / (var_dscr0 * var_dscr0))))), ((var_nscr_dn8 * assign41630_e54581) + (var_nscr * (var_qiscr0_dn8 - ((((((var_dscr0_dn8 * var_temp__blk949) + (var_dscr0 * var_temp__blk949_dn8)) / (2.0 * assign41630_e54574)) * var_dscr0) - (assign41630_e54576 * var_dscr0_dn8)) / (var_dscr0 * var_dscr0))))), ((var_nscr_dn9 * assign41630_e54581) + (var_nscr * (var_qiscr0_dn9 - ((((((var_dscr0_dn9 * var_temp__blk949) + (var_dscr0 * var_temp__blk949_dn9)) / (2.0 * assign41630_e54574)) * var_dscr0) - (assign41630_e54576 * var_dscr0_dn9)) / (var_dscr0 * var_dscr0))))),)
    } else {
        (var_qiscr, var_qiscr_dn4, var_qiscr_dn6, var_qiscr_dn7, var_qiscr_dn8, var_qiscr_dn9,)
    }
};
        var_qiscr = assign41630_e54584;
        var_qiscr_dn4 = assign41630_e54584_d_n4;
        var_qiscr_dn6 = assign41630_e54584_d_n6;
        var_qiscr_dn7 = assign41630_e54584_d_n7;
        var_qiscr_dn8 = assign41630_e54584_d_n8;
        var_qiscr_dn9 = assign41630_e54584_d_n9;

        let (assign41640_e54603, assign41640_e54603_d_n4, assign41640_e54603_d_n6, assign41640_e54603_d_n7, assign41640_e54603_d_n8, assign41640_e54603_d_n9,) = {
    if ((var_guard1195 != 0.0) && (var_guard1197 == 0.0)) {
        let assign41640_e54591: f64 = (var_nscr * 0.5);
        let assign41640_e54593: f64 = (assign41640_e54591 * var_dscr0);
        let assign41640_e54597: f64 = (0.25 * var_temp__blk949);
        let assign41640_e54599: f64 = (assign41640_e54597 * var_temp__blk949);
        let assign41640_e54600: f64 = (1.0 + assign41640_e54599);
        let assign41640_e54601: f64 = (assign41640_e54593 * assign41640_e54600);
        (assign41640_e54601, (((((var_nscr_dn4 * 0.5) * var_dscr0) + (assign41640_e54591 * var_dscr0_dn4)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * var_temp__blk949_dn4) * var_temp__blk949) + (assign41640_e54597 * var_temp__blk949_dn4)))), (((((var_nscr_dn6 * 0.5) * var_dscr0) + (assign41640_e54591 * var_dscr0_dn6)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * var_temp__blk949_dn6) * var_temp__blk949) + (assign41640_e54597 * var_temp__blk949_dn6)))), (((((var_nscr_dn7 * 0.5) * var_dscr0) + (assign41640_e54591 * var_dscr0_dn7)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * var_temp__blk949_dn7) * var_temp__blk949) + (assign41640_e54597 * var_temp__blk949_dn7)))), (((((var_nscr_dn8 * 0.5) * var_dscr0) + (assign41640_e54591 * var_dscr0_dn8)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * var_temp__blk949_dn8) * var_temp__blk949) + (assign41640_e54597 * var_temp__blk949_dn8)))), (((((var_nscr_dn9 * 0.5) * var_dscr0) + (assign41640_e54591 * var_dscr0_dn9)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * var_temp__blk949_dn9) * var_temp__blk949) + (assign41640_e54597 * var_temp__blk949_dn9)))),)
    } else {
        (var_qiscr, var_qiscr_dn4, var_qiscr_dn6, var_qiscr_dn7, var_qiscr_dn8, var_qiscr_dn9,)
    }
};
        var_qiscr = assign41640_e54603;
        var_qiscr_dn4 = assign41640_e54603_d_n4;
        var_qiscr_dn6 = assign41640_e54603_d_n6;
        var_qiscr_dn7 = assign41640_e54603_d_n7;
        var_qiscr_dn8 = assign41640_e54603_d_n8;
        var_qiscr_dn9 = assign41640_e54603_d_n9;

        let (assign41650_e54628, assign41650_e54628_d_n4, assign41650_e54628_d_n6, assign41650_e54628_d_n7, assign41650_e54628_d_n8, assign41650_e54628_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41650_e54608: f64 = (var_xg - var_qiscr);
        let assign41650_e54610: f64 = (assign41650_e54608 + 2.0);
        let assign41650_e54613: f64 = (var_xg - var_qiscr);
        let assign41650_e54615: f64 = (assign41650_e54613 - 2.0);
        let assign41650_e54618: f64 = (var_xg - var_qiscr);
        let assign41650_e54620: f64 = (assign41650_e54618 - 2.0);
        let assign41650_e54621: f64 = (assign41650_e54615 * assign41650_e54620);
        let assign41650_e54623: f64 = (assign41650_e54621 + 1.0);
        let assign41650_e54624: f64 = (assign41650_e54623).sqrt();
        let assign41650_e54625: f64 = (assign41650_e54610 + assign41650_e54624);
        let assign41650_e54626: f64 = (0.5 * assign41650_e54625);
        (assign41650_e54626, (0.5 * ((var_xg_dn4 - var_qiscr_dn4) + ((((var_xg_dn4 - var_qiscr_dn4) * assign41650_e54620) + (assign41650_e54615 * (var_xg_dn4 - var_qiscr_dn4))) / (2.0 * assign41650_e54624)))), (0.5 * ((var_xg_dn6 - var_qiscr_dn6) + ((((var_xg_dn6 - var_qiscr_dn6) * assign41650_e54620) + (assign41650_e54615 * (var_xg_dn6 - var_qiscr_dn6))) / (2.0 * assign41650_e54624)))), (0.5 * ((var_xg_dn7 - var_qiscr_dn7) + ((((var_xg_dn7 - var_qiscr_dn7) * assign41650_e54620) + (assign41650_e54615 * (var_xg_dn7 - var_qiscr_dn7))) / (2.0 * assign41650_e54624)))), (0.5 * ((var_xg_dn8 - var_qiscr_dn8) + ((((var_xg_dn8 - var_qiscr_dn8) * assign41650_e54620) + (assign41650_e54615 * (var_xg_dn8 - var_qiscr_dn8))) / (2.0 * assign41650_e54624)))), (0.5 * ((var_xg_dn9 - var_qiscr_dn9) + ((((var_xg_dn9 - var_qiscr_dn9) * assign41650_e54620) + (assign41650_e54615 * (var_xg_dn9 - var_qiscr_dn9))) / (2.0 * assign41650_e54624)))),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign41650_e54628;
        var_temp__blk949_dn4 = assign41650_e54628_d_n4;
        var_temp__blk949_dn6 = assign41650_e54628_d_n6;
        var_temp__blk949_dn7 = assign41650_e54628_d_n7;
        var_temp__blk949_dn8 = assign41650_e54628_d_n8;
        var_temp__blk949_dn9 = assign41650_e54628_d_n9;

        let (assign41660_e54645, assign41660_e54645_d_n4, assign41660_e54645_d_n6, assign41660_e54645_d_n7, assign41660_e54645_d_n8, assign41660_e54645_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41660_e54632: f64 = (0.5 * var_gf2);
        let assign41660_e54636: f64 = (4.0 / var_gf2);
        let assign41660_e54638: f64 = (assign41660_e54636 * var_temp__blk949);
        let assign41660_e54639: f64 = (1.0 + assign41660_e54638);
        let assign41660_e54640: f64 = (assign41660_e54639).sqrt();
        let assign41660_e54642: f64 = (assign41660_e54640 - 1.0);
        let assign41660_e54643: f64 = (assign41660_e54632 * assign41660_e54642);
        (assign41660_e54643, (((0.5 * var_gf2_dn4) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * var_gf2_dn4) / (var_gf2 * var_gf2))) * var_temp__blk949) + (assign41660_e54636 * var_temp__blk949_dn4)) / (2.0 * assign41660_e54640)))), (((0.5 * var_gf2_dn6) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * var_gf2_dn6) / (var_gf2 * var_gf2))) * var_temp__blk949) + (assign41660_e54636 * var_temp__blk949_dn6)) / (2.0 * assign41660_e54640)))), (((0.5 * var_gf2_dn7) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * var_gf2_dn7) / (var_gf2 * var_gf2))) * var_temp__blk949) + (assign41660_e54636 * var_temp__blk949_dn7)) / (2.0 * assign41660_e54640)))), (((0.5 * var_gf2_dn8) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * var_gf2_dn8) / (var_gf2 * var_gf2))) * var_temp__blk949) + (assign41660_e54636 * var_temp__blk949_dn8)) / (2.0 * assign41660_e54640)))), (((0.5 * var_gf2_dn9) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * var_gf2_dn9) / (var_gf2 * var_gf2))) * var_temp__blk949) + (assign41660_e54636 * var_temp__blk949_dn9)) / (2.0 * assign41660_e54640)))),)
    } else {
        (var_qbscr, var_qbscr_dn4, var_qbscr_dn6, var_qbscr_dn7, var_qbscr_dn8, var_qbscr_dn9,)
    }
};
        var_qbscr = assign41660_e54645;
        var_qbscr_dn4 = assign41660_e54645_d_n4;
        var_qbscr_dn6 = assign41660_e54645_d_n6;
        var_qbscr_dn7 = assign41660_e54645_d_n7;
        var_qbscr_dn8 = assign41660_e54645_d_n8;
        var_qbscr_dn9 = assign41660_e54645_d_n9;

        let (assign41670_e54653, assign41670_e54653_d_n4, assign41670_e54653_d_n6, assign41670_e54653_d_n7, assign41670_e54653_d_n8, assign41670_e54653_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41670_e54650: f64 = (var_qbscr + var_qiscr);
        let assign41670_e54651: f64 = (var_qbscr / assign41670_e54650);
        (assign41670_e54651, (((var_qbscr_dn4 * assign41670_e54650) - (var_qbscr * (var_qbscr_dn4 + var_qiscr_dn4))) / (assign41670_e54650 * assign41670_e54650)), (((var_qbscr_dn6 * assign41670_e54650) - (var_qbscr * (var_qbscr_dn6 + var_qiscr_dn6))) / (assign41670_e54650 * assign41670_e54650)), (((var_qbscr_dn7 * assign41670_e54650) - (var_qbscr * (var_qbscr_dn7 + var_qiscr_dn7))) / (assign41670_e54650 * assign41670_e54650)), (((var_qbscr_dn8 * assign41670_e54650) - (var_qbscr * (var_qbscr_dn8 + var_qiscr_dn8))) / (assign41670_e54650 * assign41670_e54650)), (((var_qbscr_dn9 * assign41670_e54650) - (var_qbscr * (var_qbscr_dn9 + var_qiscr_dn9))) / (assign41670_e54650 * assign41670_e54650)),)
    } else {
        (var_fscr, var_fscr_dn4, var_fscr_dn6, var_fscr_dn7, var_fscr_dn8, var_fscr_dn9,)
    }
};
        var_fscr = assign41670_e54653;
        var_fscr_dn4 = assign41670_e54653_d_n4;
        var_fscr_dn6 = assign41670_e54653_d_n6;
        var_fscr_dn7 = assign41670_e54653_d_n7;
        var_fscr_dn8 = assign41670_e54653_d_n8;
        var_fscr_dn9 = assign41670_e54653_d_n9;

        let (assign41680_e54661, assign41680_e54661_d_n4, assign41680_e54661_d_n6, assign41680_e54661_d_n7, assign41680_e54661_d_n8, assign41680_e54661_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41680_e54658: f64 = (var_fscr * var_delxb);
        let assign41680_e54659: f64 = (var_xno_s - assign41680_e54658);
        (assign41680_e54659, (var_xno_s_dn4 - ((var_fscr_dn4 * var_delxb) + (var_fscr * var_delxb_dn4))), (var_xno_s_dn6 - ((var_fscr_dn6 * var_delxb) + (var_fscr * var_delxb_dn6))), (var_xno_s_dn7 - ((var_fscr_dn7 * var_delxb) + (var_fscr * var_delxb_dn7))), (var_xno_s_dn8 - ((var_fscr_dn8 * var_delxb) + (var_fscr * var_delxb_dn8))), (var_xno_s_dn9 - ((var_fscr_dn9 * var_delxb) + (var_fscr * var_delxb_dn9))),)
    } else {
        (var_xn_s, var_xn_s_dn4, var_xn_s_dn6, var_xn_s_dn7, var_xn_s_dn8, var_xn_s_dn9,)
    }
};
        var_xn_s = assign41680_e54661;
        var_xn_s_dn4 = assign41680_e54661_d_n4;
        var_xn_s_dn6 = assign41680_e54661_d_n6;
        var_xn_s_dn7 = assign41680_e54661_d_n7;
        var_xn_s_dn8 = assign41680_e54661_d_n8;
        var_xn_s_dn9 = assign41680_e54661_d_n9;

        let assign41690_e54665: f64 = (var_gf * 0.7071067811865475);
        let assign41690_e54666: f64 = (1.0 + assign41690_e54665);
        var_xi = assign41690_e54666;
        var_xi_dn4 = (var_gf_dn4 * 0.7071067811865475);
        var_xi_dn6 = (var_gf_dn6 * 0.7071067811865475);
        var_xi_dn7 = (var_gf_dn7 * 0.7071067811865475);
        var_xi_dn8 = (var_gf_dn8 * 0.7071067811865475);
        var_xi_dn9 = (var_gf_dn9 * 0.7071067811865475);

        let assign41700_e54669: f64 = (1e-5 * var_xi);
        var_margin = assign41700_e54669;

        let assign41710_e54672: f64 = (1.0 / var_xi);
        var_inv_xi = assign41710_e54672;
        var_inv_xi_dn4 = (-(var_xi_dn4 / (var_xi * var_xi)));
        var_inv_xi_dn6 = (-(var_xi_dn6 / (var_xi * var_xi)));
        var_inv_xi_dn7 = (-(var_xi_dn7 / (var_xi * var_xi)));
        var_inv_xi_dn8 = (-(var_xi_dn8 / (var_xi * var_xi)));
        var_inv_xi_dn9 = (-(var_xi_dn9 / (var_xi * var_xi)));

        var_sp_s_x1 = 0.0;
        var_sp_s_x1_dn4 = 0.0;
        var_sp_s_x1_dn6 = 0.0;
        var_sp_s_x1_dn7 = 0.0;
        var_sp_s_x1_dn8 = 0.0;
        var_sp_s_x1_dn9 = 0.0;

        var_x_s = 0.0;
        var_x_s_dn4 = 0.0;
        var_x_s_dn6 = 0.0;
        var_x_s_dn7 = 0.0;
        var_x_s_dn8 = 0.0;
        var_x_s_dn9 = 0.0;

        let assign41740_e54677: f64 = if var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        var_guard1198 = assign41740_e54677;

        let (assign41750_e54683, assign41750_e54683_d_n4, assign41750_e54683_d_n6, assign41750_e54683_d_n7, assign41750_e54683_d_n8, assign41750_e54683_d_n9,) = {
    if (var_guard1198 != 0.0) {
        let assign41750_e54680: f64 = (-var_xn_s);
        let assign41750_e54681: f64 = (assign41750_e54680).exp();
        (assign41750_e54681, (assign41750_e54681 * (-var_xn_s_dn4)), (assign41750_e54681 * (-var_xn_s_dn6)), (assign41750_e54681 * (-var_xn_s_dn7)), (assign41750_e54681 * (-var_xn_s_dn8)), (assign41750_e54681 * (-var_xn_s_dn9)),)
    } else {
        (var_delta_ns, var_delta_ns_dn4, var_delta_ns_dn6, var_delta_ns_dn7, var_delta_ns_dn8, var_delta_ns_dn9,)
    }
};
        var_delta_ns = assign41750_e54683;
        var_delta_ns_dn4 = assign41750_e54683_d_n4;
        var_delta_ns_dn6 = assign41750_e54683_d_n6;
        var_delta_ns_dn7 = assign41750_e54683_d_n7;
        var_delta_ns_dn8 = assign41750_e54683_d_n8;
        var_delta_ns_dn9 = assign41750_e54683_d_n9;

        let (assign41760_e54710, assign41760_e54710_d_n4, assign41760_e54710_d_n6, assign41760_e54710_d_n7, assign41760_e54710_d_n8, assign41760_e54710_d_n9,) = {
    if (var_guard1198 == 0.0) {
        let assign41760_e54690: f64 = (var_xn_s - 460.51701859880916);
        let assign41760_e54695: f64 = (var_xn_s - 460.51701859880916);
        let assign41760_e54699: f64 = (var_xn_s - 460.51701859880916);
        let assign41760_e54701: f64 = (assign41760_e54699 * 0.3333333333333333);
        let assign41760_e54702: f64 = (1.0 + assign41760_e54701);
        let assign41760_e54703: f64 = (assign41760_e54695 * assign41760_e54702);
        let assign41760_e54704: f64 = (0.5 * assign41760_e54703);
        let assign41760_e54705: f64 = (1.0 + assign41760_e54704);
        let assign41760_e54706: f64 = (assign41760_e54690 * assign41760_e54705);
        let assign41760_e54707: f64 = (1.0 + assign41760_e54706);
        let assign41760_e54708: f64 = (1e-200 / assign41760_e54707);
        (assign41760_e54708, (-((1e-200 * ((var_xn_s_dn4 * assign41760_e54705) + (assign41760_e54690 * (0.5 * ((var_xn_s_dn4 * assign41760_e54702) + (assign41760_e54695 * (var_xn_s_dn4 * 0.3333333333333333))))))) / (assign41760_e54707 * assign41760_e54707))), (-((1e-200 * ((var_xn_s_dn6 * assign41760_e54705) + (assign41760_e54690 * (0.5 * ((var_xn_s_dn6 * assign41760_e54702) + (assign41760_e54695 * (var_xn_s_dn6 * 0.3333333333333333))))))) / (assign41760_e54707 * assign41760_e54707))), (-((1e-200 * ((var_xn_s_dn7 * assign41760_e54705) + (assign41760_e54690 * (0.5 * ((var_xn_s_dn7 * assign41760_e54702) + (assign41760_e54695 * (var_xn_s_dn7 * 0.3333333333333333))))))) / (assign41760_e54707 * assign41760_e54707))), (-((1e-200 * ((var_xn_s_dn8 * assign41760_e54705) + (assign41760_e54690 * (0.5 * ((var_xn_s_dn8 * assign41760_e54702) + (assign41760_e54695 * (var_xn_s_dn8 * 0.3333333333333333))))))) / (assign41760_e54707 * assign41760_e54707))), (-((1e-200 * ((var_xn_s_dn9 * assign41760_e54705) + (assign41760_e54690 * (0.5 * ((var_xn_s_dn9 * assign41760_e54702) + (assign41760_e54695 * (var_xn_s_dn9 * 0.3333333333333333))))))) / (assign41760_e54707 * assign41760_e54707))),)
    } else {
        (var_delta_ns, var_delta_ns_dn4, var_delta_ns_dn6, var_delta_ns_dn7, var_delta_ns_dn8, var_delta_ns_dn9,)
    }
};
        var_delta_ns = assign41760_e54710;
        var_delta_ns_dn4 = assign41760_e54710_d_n4;
        var_delta_ns_dn6 = assign41760_e54710_d_n6;
        var_delta_ns_dn7 = assign41760_e54710_d_n7;
        var_delta_ns_dn8 = assign41760_e54710_d_n8;
        var_delta_ns_dn9 = assign41760_e54710_d_n9;

        let assign41770_e54712: f64 = (var_xg).abs();
        let assign41770_e54714: f64 = if assign41770_e54712 <= var_margin { 1.0 } else { 0.0 };
        var_guard1199 = assign41770_e54714;

        let (assign41780_e54724, assign41780_e54724_d_n4, assign41780_e54724_d_n6, assign41780_e54724_d_n7, assign41780_e54724_d_n8, assign41780_e54724_d_n9,) = {
    if (var_guard1199 != 0.0) {
        let assign41780_e54718: f64 = (var_inv_xi * var_inv_xi);
        let assign41780_e54720: f64 = (assign41780_e54718 * 0.16666666666666666);
        let assign41780_e54722: f64 = (assign41780_e54720 * 0.7071067811865475);
        (assign41780_e54722, ((((var_inv_xi_dn4 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn4)) * 0.16666666666666666) * 0.7071067811865475), ((((var_inv_xi_dn6 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((var_inv_xi_dn7 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((var_inv_xi_dn8 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn8)) * 0.16666666666666666) * 0.7071067811865475), ((((var_inv_xi_dn9 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn9)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (var_sp_s_temp1, var_sp_s_temp1_dn4, var_sp_s_temp1_dn6, var_sp_s_temp1_dn7, var_sp_s_temp1_dn8, var_sp_s_temp1_dn9,)
    }
};
        var_sp_s_temp1 = assign41780_e54724;
        var_sp_s_temp1_dn4 = assign41780_e54724_d_n4;
        var_sp_s_temp1_dn6 = assign41780_e54724_d_n6;
        var_sp_s_temp1_dn7 = assign41780_e54724_d_n7;
        var_sp_s_temp1_dn8 = assign41780_e54724_d_n8;
        var_sp_s_temp1_dn9 = assign41780_e54724_d_n9;

        let (assign41790_e54742, assign41790_e54742_d_n4, assign41790_e54742_d_n6, assign41790_e54742_d_n7, assign41790_e54742_d_n8, assign41790_e54742_d_n9,) = {
    if (var_guard1199 != 0.0) {
        let assign41790_e54728: f64 = (var_xg * var_inv_xi);
        let assign41790_e54733: f64 = (1.0 - var_delta_ns);
        let assign41790_e54734: f64 = (var_xg * assign41790_e54733);
        let assign41790_e54736: f64 = (assign41790_e54734 * var_gf);
        let assign41790_e54738: f64 = (assign41790_e54736 * var_sp_s_temp1);
        let assign41790_e54739: f64 = (1.0 + assign41790_e54738);
        let assign41790_e54740: f64 = (assign41790_e54728 * assign41790_e54739);
        (assign41790_e54740, ((((var_xg_dn4 * var_inv_xi) + (var_xg * var_inv_xi_dn4)) * assign41790_e54739) + (assign41790_e54728 * ((((((var_xg_dn4 * assign41790_e54733) + (var_xg * (-var_delta_ns_dn4))) * var_gf) + (assign41790_e54734 * var_gf_dn4)) * var_sp_s_temp1) + (assign41790_e54736 * var_sp_s_temp1_dn4)))), ((((var_xg_dn6 * var_inv_xi) + (var_xg * var_inv_xi_dn6)) * assign41790_e54739) + (assign41790_e54728 * ((((((var_xg_dn6 * assign41790_e54733) + (var_xg * (-var_delta_ns_dn6))) * var_gf) + (assign41790_e54734 * var_gf_dn6)) * var_sp_s_temp1) + (assign41790_e54736 * var_sp_s_temp1_dn6)))), ((((var_xg_dn7 * var_inv_xi) + (var_xg * var_inv_xi_dn7)) * assign41790_e54739) + (assign41790_e54728 * ((((((var_xg_dn7 * assign41790_e54733) + (var_xg * (-var_delta_ns_dn7))) * var_gf) + (assign41790_e54734 * var_gf_dn7)) * var_sp_s_temp1) + (assign41790_e54736 * var_sp_s_temp1_dn7)))), ((((var_xg_dn8 * var_inv_xi) + (var_xg * var_inv_xi_dn8)) * assign41790_e54739) + (assign41790_e54728 * ((((((var_xg_dn8 * assign41790_e54733) + (var_xg * (-var_delta_ns_dn8))) * var_gf) + (assign41790_e54734 * var_gf_dn8)) * var_sp_s_temp1) + (assign41790_e54736 * var_sp_s_temp1_dn8)))), ((((var_xg_dn9 * var_inv_xi) + (var_xg * var_inv_xi_dn9)) * assign41790_e54739) + (assign41790_e54728 * ((((((var_xg_dn9 * assign41790_e54733) + (var_xg * (-var_delta_ns_dn9))) * var_gf) + (assign41790_e54734 * var_gf_dn9)) * var_sp_s_temp1) + (assign41790_e54736 * var_sp_s_temp1_dn9)))),)
    } else {
        (var_x_s, var_x_s_dn4, var_x_s_dn6, var_x_s_dn7, var_x_s_dn8, var_x_s_dn9,)
    }
};
        var_x_s = assign41790_e54742;
        var_x_s_dn4 = assign41790_e54742_d_n4;
        var_x_s_dn6 = assign41790_e54742_d_n6;
        var_x_s_dn7 = assign41790_e54742_d_n7;
        var_x_s_dn8 = assign41790_e54742_d_n8;
        var_x_s_dn9 = assign41790_e54742_d_n9;

        let assign41800_e54745: f64 = (-var_margin);
        let assign41800_e54746: f64 = if var_xg < assign41800_e54745 { 1.0 } else { 0.0 };
        var_guard1200 = assign41800_e54746;

        let (assign41810_e54754, assign41810_e54754_d_n4, assign41810_e54754_d_n6, assign41810_e54754_d_n7, assign41810_e54754_d_n8, assign41810_e54754_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign41810_e54752: f64 = (-var_xg);
        (assign41810_e54752, (-var_xg_dn4), (-var_xg_dn6), (-var_xg_dn7), (-var_xg_dn8), (-var_xg_dn9),)
    } else {
        (var_sp_s_yg, var_sp_s_yg_dn4, var_sp_s_yg_dn6, var_sp_s_yg_dn7, var_sp_s_yg_dn8, var_sp_s_yg_dn9,)
    }
};
        var_sp_s_yg = assign41810_e54754;
        var_sp_s_yg_dn4 = assign41810_e54754_d_n4;
        var_sp_s_yg_dn6 = assign41810_e54754_d_n6;
        var_sp_s_yg_dn7 = assign41810_e54754_d_n7;
        var_sp_s_yg_dn8 = assign41810_e54754_d_n8;
        var_sp_s_yg_dn9 = assign41810_e54754_d_n9;

        let (assign41820_e54765, assign41820_e54765_d_n4, assign41820_e54765_d_n6, assign41820_e54765_d_n7, assign41820_e54765_d_n8, assign41820_e54765_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign41820_e54762: f64 = (var_sp_s_yg * var_inv_xi);
        let assign41820_e54763: f64 = (1.25 * assign41820_e54762);
        (assign41820_e54763, (1.25 * ((var_sp_s_yg_dn4 * var_inv_xi) + (var_sp_s_yg * var_inv_xi_dn4))), (1.25 * ((var_sp_s_yg_dn6 * var_inv_xi) + (var_sp_s_yg * var_inv_xi_dn6))), (1.25 * ((var_sp_s_yg_dn7 * var_inv_xi) + (var_sp_s_yg * var_inv_xi_dn7))), (1.25 * ((var_sp_s_yg_dn8 * var_inv_xi) + (var_sp_s_yg * var_inv_xi_dn8))), (1.25 * ((var_sp_s_yg_dn9 * var_inv_xi) + (var_sp_s_yg * var_inv_xi_dn9))),)
    } else {
        (var_sp_s_ysub, var_sp_s_ysub_dn4, var_sp_s_ysub_dn6, var_sp_s_ysub_dn7, var_sp_s_ysub_dn8, var_sp_s_ysub_dn9,)
    }
};
        var_sp_s_ysub = assign41820_e54765;
        var_sp_s_ysub_dn4 = assign41820_e54765_d_n4;
        var_sp_s_ysub_dn6 = assign41820_e54765_d_n6;
        var_sp_s_ysub_dn7 = assign41820_e54765_d_n7;
        var_sp_s_ysub_dn8 = assign41820_e54765_d_n8;
        var_sp_s_ysub_dn9 = assign41820_e54765_d_n9;

        let (assign41830_e54787, assign41830_e54787_d_n4, assign41830_e54787_d_n6, assign41830_e54787_d_n7, assign41830_e54787_d_n8, assign41830_e54787_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign41830_e54773: f64 = (var_sp_s_ysub + 10.0);
        let assign41830_e54776: f64 = (var_sp_s_ysub - 6.0);
        let assign41830_e54779: f64 = (var_sp_s_ysub - 6.0);
        let assign41830_e54780: f64 = (assign41830_e54776 * assign41830_e54779);
        let assign41830_e54782: f64 = (assign41830_e54780 + 64.0);
        let assign41830_e54783: f64 = (assign41830_e54782).sqrt();
        let assign41830_e54784: f64 = (assign41830_e54773 - assign41830_e54783);
        let assign41830_e54785: f64 = (0.5 * assign41830_e54784);
        (assign41830_e54785, (0.5 * (var_sp_s_ysub_dn4 - (((var_sp_s_ysub_dn4 * assign41830_e54779) + (assign41830_e54776 * var_sp_s_ysub_dn4)) / (2.0 * assign41830_e54783)))), (0.5 * (var_sp_s_ysub_dn6 - (((var_sp_s_ysub_dn6 * assign41830_e54779) + (assign41830_e54776 * var_sp_s_ysub_dn6)) / (2.0 * assign41830_e54783)))), (0.5 * (var_sp_s_ysub_dn7 - (((var_sp_s_ysub_dn7 * assign41830_e54779) + (assign41830_e54776 * var_sp_s_ysub_dn7)) / (2.0 * assign41830_e54783)))), (0.5 * (var_sp_s_ysub_dn8 - (((var_sp_s_ysub_dn8 * assign41830_e54779) + (assign41830_e54776 * var_sp_s_ysub_dn8)) / (2.0 * assign41830_e54783)))), (0.5 * (var_sp_s_ysub_dn9 - (((var_sp_s_ysub_dn9 * assign41830_e54779) + (assign41830_e54776 * var_sp_s_ysub_dn9)) / (2.0 * assign41830_e54783)))),)
    } else {
        (var_sp_s_eta, var_sp_s_eta_dn4, var_sp_s_eta_dn6, var_sp_s_eta_dn7, var_sp_s_eta_dn8, var_sp_s_eta_dn9,)
    }
};
        var_sp_s_eta = assign41830_e54787;
        var_sp_s_eta_dn4 = assign41830_e54787_d_n4;
        var_sp_s_eta_dn6 = assign41830_e54787_d_n6;
        var_sp_s_eta_dn7 = assign41830_e54787_d_n7;
        var_sp_s_eta_dn8 = assign41830_e54787_d_n8;
        var_sp_s_eta_dn9 = assign41830_e54787_d_n9;

        let (assign41840_e54796, assign41840_e54796_d_n4, assign41840_e54796_d_n6, assign41840_e54796_d_n7, assign41840_e54796_d_n8, assign41840_e54796_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign41840_e54794: f64 = (var_sp_s_yg - var_sp_s_eta);
        (assign41840_e54794, (var_sp_s_yg_dn4 - var_sp_s_eta_dn4), (var_sp_s_yg_dn6 - var_sp_s_eta_dn6), (var_sp_s_yg_dn7 - var_sp_s_eta_dn7), (var_sp_s_yg_dn8 - var_sp_s_eta_dn8), (var_sp_s_yg_dn9 - var_sp_s_eta_dn9),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn4, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8, var_sp_s_temp_dn9,)
    }
};
        var_sp_s_temp = assign41840_e54796;
        var_sp_s_temp_dn4 = assign41840_e54796_d_n4;
        var_sp_s_temp_dn6 = assign41840_e54796_d_n6;
        var_sp_s_temp_dn7 = assign41840_e54796_d_n7;
        var_sp_s_temp_dn8 = assign41840_e54796_d_n8;
        var_sp_s_temp_dn9 = assign41840_e54796_d_n9;

        let (assign41850_e54811, assign41850_e54811_d_n4, assign41850_e54811_d_n6, assign41850_e54811_d_n7, assign41850_e54811_d_n8, assign41850_e54811_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign41850_e54803: f64 = (var_sp_s_temp * var_sp_s_temp);
        let assign41850_e54807: f64 = (var_sp_s_eta + 1.0);
        let assign41850_e54808: f64 = (var_gf2 * assign41850_e54807);
        let assign41850_e54809: f64 = (assign41850_e54803 + assign41850_e54808);
        (assign41850_e54809, (((var_sp_s_temp_dn4 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn4)) + ((var_gf2_dn4 * assign41850_e54807) + (var_gf2 * var_sp_s_eta_dn4))), (((var_sp_s_temp_dn6 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn6)) + ((var_gf2_dn6 * assign41850_e54807) + (var_gf2 * var_sp_s_eta_dn6))), (((var_sp_s_temp_dn7 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn7)) + ((var_gf2_dn7 * assign41850_e54807) + (var_gf2 * var_sp_s_eta_dn7))), (((var_sp_s_temp_dn8 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn8)) + ((var_gf2_dn8 * assign41850_e54807) + (var_gf2 * var_sp_s_eta_dn8))), (((var_sp_s_temp_dn9 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn9)) + ((var_gf2_dn9 * assign41850_e54807) + (var_gf2 * var_sp_s_eta_dn9))),)
    } else {
        (var_sp_s_a, var_sp_s_a_dn4, var_sp_s_a_dn6, var_sp_s_a_dn7, var_sp_s_a_dn8, var_sp_s_a_dn9,)
    }
};
        var_sp_s_a = assign41850_e54811;
        var_sp_s_a_dn4 = assign41850_e54811_d_n4;
        var_sp_s_a_dn6 = assign41850_e54811_d_n6;
        var_sp_s_a_dn7 = assign41850_e54811_d_n7;
        var_sp_s_a_dn8 = assign41850_e54811_d_n8;
        var_sp_s_a_dn9 = assign41850_e54811_d_n9;

        let (assign41860_e54822, assign41860_e54822_d_n4, assign41860_e54822_d_n6, assign41860_e54822_d_n7, assign41860_e54822_d_n8, assign41860_e54822_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign41860_e54818: f64 = (2.0 * var_sp_s_temp);
        let assign41860_e54820: f64 = (assign41860_e54818 - var_gf2);
        (assign41860_e54820, ((2.0 * var_sp_s_temp_dn4) - var_gf2_dn4), ((2.0 * var_sp_s_temp_dn6) - var_gf2_dn6), ((2.0 * var_sp_s_temp_dn7) - var_gf2_dn7), ((2.0 * var_sp_s_temp_dn8) - var_gf2_dn8), ((2.0 * var_sp_s_temp_dn9) - var_gf2_dn9),)
    } else {
        (var_sp_s_c, var_sp_s_c_dn4, var_sp_s_c_dn6, var_sp_s_c_dn7, var_sp_s_c_dn8, var_sp_s_c_dn9,)
    }
};
        var_sp_s_c = assign41860_e54822;
        var_sp_s_c_dn4 = assign41860_e54822_d_n4;
        var_sp_s_c_dn6 = assign41860_e54822_d_n6;
        var_sp_s_c_dn7 = assign41860_e54822_d_n7;
        var_sp_s_c_dn8 = assign41860_e54822_d_n8;
        var_sp_s_c_dn9 = assign41860_e54822_d_n9;

        let (assign41870_e54835, assign41870_e54835_d_n4, assign41870_e54835_d_n6, assign41870_e54835_d_n7, assign41870_e54835_d_n8, assign41870_e54835_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign41870_e54828: f64 = (-var_sp_s_eta);
        let assign41870_e54831: f64 = (var_sp_s_a * var_inv_gf2);
        let assign41870_e54832: f64 = (assign41870_e54831).ln();
        let assign41870_e54833: f64 = (assign41870_e54828 + assign41870_e54832);
        (assign41870_e54833, ((-var_sp_s_eta_dn4) + (((var_sp_s_a_dn4 * var_inv_gf2) + (var_sp_s_a * var_inv_gf2_dn4)) / assign41870_e54831)), ((-var_sp_s_eta_dn6) + (((var_sp_s_a_dn6 * var_inv_gf2) + (var_sp_s_a * var_inv_gf2_dn6)) / assign41870_e54831)), ((-var_sp_s_eta_dn7) + (((var_sp_s_a_dn7 * var_inv_gf2) + (var_sp_s_a * var_inv_gf2_dn7)) / assign41870_e54831)), ((-var_sp_s_eta_dn8) + (((var_sp_s_a_dn8 * var_inv_gf2) + (var_sp_s_a * var_inv_gf2_dn8)) / assign41870_e54831)), ((-var_sp_s_eta_dn9) + (((var_sp_s_a_dn9 * var_inv_gf2) + (var_sp_s_a * var_inv_gf2_dn9)) / assign41870_e54831)),)
    } else {
        (var_sp_s_tau, var_sp_s_tau_dn4, var_sp_s_tau_dn6, var_sp_s_tau_dn7, var_sp_s_tau_dn8, var_sp_s_tau_dn9,)
    }
};
        var_sp_s_tau = assign41870_e54835;
        var_sp_s_tau_dn4 = assign41870_e54835_d_n4;
        var_sp_s_tau_dn6 = assign41870_e54835_d_n6;
        var_sp_s_tau_dn7 = assign41870_e54835_d_n7;
        var_sp_s_tau_dn8 = assign41870_e54835_d_n8;
        var_sp_s_tau_dn9 = assign41870_e54835_d_n9;

        let (assign41880_e54844, assign41880_e54844_d_n4, assign41880_e54844_d_n6, assign41880_e54844_d_n7, assign41880_e54844_d_n8, assign41880_e54844_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign41880_e54842: f64 = (var_sp_s_a + var_sp_s_c);
        (assign41880_e54842, (var_sp_s_a_dn4 + var_sp_s_c_dn4), (var_sp_s_a_dn6 + var_sp_s_c_dn6), (var_sp_s_a_dn7 + var_sp_s_c_dn7), (var_sp_s_a_dn8 + var_sp_s_c_dn8), (var_sp_s_a_dn9 + var_sp_s_c_dn9),)
    } else {
        (var_nu, var_nu_dn4, var_nu_dn6, var_nu_dn7, var_nu_dn8, var_nu_dn9,)
    }
};
        var_nu = assign41880_e54844;
        var_nu_dn4 = assign41880_e54844_d_n4;
        var_nu_dn6 = assign41880_e54844_d_n6;
        var_nu_dn7 = assign41880_e54844_d_n7;
        var_nu_dn8 = assign41880_e54844_d_n8;
        var_nu_dn9 = assign41880_e54844_d_n9;

        let (assign41890_e54863, assign41890_e54863_d_n4, assign41890_e54863_d_n6, assign41890_e54863_d_n7, assign41890_e54863_d_n8, assign41890_e54863_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign41890_e54851: f64 = (var_nu * var_nu);
        let assign41890_e54856: f64 = (var_sp_s_c * var_sp_s_c);
        let assign41890_e54857: f64 = (0.5 * assign41890_e54856);
        let assign41890_e54859: f64 = (assign41890_e54857 - var_sp_s_a);
        let assign41890_e54860: f64 = (var_sp_s_tau * assign41890_e54859);
        let assign41890_e54861: f64 = (assign41890_e54851 + assign41890_e54860);
        (assign41890_e54861, (((var_nu_dn4 * var_nu) + (var_nu * var_nu_dn4)) + ((var_sp_s_tau_dn4 * assign41890_e54859) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn4 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn4))) - var_sp_s_a_dn4)))), (((var_nu_dn6 * var_nu) + (var_nu * var_nu_dn6)) + ((var_sp_s_tau_dn6 * assign41890_e54859) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn6 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn6))) - var_sp_s_a_dn6)))), (((var_nu_dn7 * var_nu) + (var_nu * var_nu_dn7)) + ((var_sp_s_tau_dn7 * assign41890_e54859) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn7 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn7))) - var_sp_s_a_dn7)))), (((var_nu_dn8 * var_nu) + (var_nu * var_nu_dn8)) + ((var_sp_s_tau_dn8 * assign41890_e54859) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn8 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn8))) - var_sp_s_a_dn8)))), (((var_nu_dn9 * var_nu) + (var_nu * var_nu_dn9)) + ((var_sp_s_tau_dn9 * assign41890_e54859) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn9 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn9))) - var_sp_s_a_dn9)))),)
    } else {
        (var_mutau, var_mutau_dn4, var_mutau_dn6, var_mutau_dn7, var_mutau_dn8, var_mutau_dn9,)
    }
};
        var_mutau = assign41890_e54863;
        var_mutau_dn4 = assign41890_e54863_d_n4;
        var_mutau_dn6 = assign41890_e54863_d_n6;
        var_mutau_dn7 = assign41890_e54863_d_n7;
        var_mutau_dn8 = assign41890_e54863_d_n8;
        var_mutau_dn9 = assign41890_e54863_d_n9;

        *var_delta_ns_slot = var_delta_ns;
        *var_delta_ns_dn4_slot = var_delta_ns_dn4;
        *var_delta_ns_dn6_slot = var_delta_ns_dn6;
        *var_delta_ns_dn7_slot = var_delta_ns_dn7;
        *var_delta_ns_dn8_slot = var_delta_ns_dn8;
        *var_delta_ns_dn9_slot = var_delta_ns_dn9;
        *var_dscr0_slot = var_dscr0;
        *var_dscr0_dn4_slot = var_dscr0_dn4;
        *var_dscr0_dn6_slot = var_dscr0_dn6;
        *var_dscr0_dn7_slot = var_dscr0_dn7;
        *var_dscr0_dn8_slot = var_dscr0_dn8;
        *var_dscr0_dn9_slot = var_dscr0_dn9;
        *var_fscr_slot = var_fscr;
        *var_fscr_dn4_slot = var_fscr_dn4;
        *var_fscr_dn6_slot = var_fscr_dn6;
        *var_fscr_dn7_slot = var_fscr_dn7;
        *var_fscr_dn8_slot = var_fscr_dn8;
        *var_fscr_dn9_slot = var_fscr_dn9;
        *var_guard1196_slot = var_guard1196;
        *var_guard1197_slot = var_guard1197;
        *var_guard1198_slot = var_guard1198;
        *var_guard1199_slot = var_guard1199;
        *var_guard1200_slot = var_guard1200;
        *var_inv_xi_slot = var_inv_xi;
        *var_inv_xi_dn4_slot = var_inv_xi_dn4;
        *var_inv_xi_dn6_slot = var_inv_xi_dn6;
        *var_inv_xi_dn7_slot = var_inv_xi_dn7;
        *var_inv_xi_dn8_slot = var_inv_xi_dn8;
        *var_inv_xi_dn9_slot = var_inv_xi_dn9;
        *var_margin_slot = var_margin;
        *var_mutau_slot = var_mutau;
        *var_mutau_dn4_slot = var_mutau_dn4;
        *var_mutau_dn6_slot = var_mutau_dn6;
        *var_mutau_dn7_slot = var_mutau_dn7;
        *var_mutau_dn8_slot = var_mutau_dn8;
        *var_mutau_dn9_slot = var_mutau_dn9;
        *var_nu_slot = var_nu;
        *var_nu_dn4_slot = var_nu_dn4;
        *var_nu_dn6_slot = var_nu_dn6;
        *var_nu_dn7_slot = var_nu_dn7;
        *var_nu_dn8_slot = var_nu_dn8;
        *var_nu_dn9_slot = var_nu_dn9;
        *var_qbscr_slot = var_qbscr;
        *var_qbscr_dn4_slot = var_qbscr_dn4;
        *var_qbscr_dn6_slot = var_qbscr_dn6;
        *var_qbscr_dn7_slot = var_qbscr_dn7;
        *var_qbscr_dn8_slot = var_qbscr_dn8;
        *var_qbscr_dn9_slot = var_qbscr_dn9;
        *var_qiscr_slot = var_qiscr;
        *var_qiscr0_slot = var_qiscr0;
        *var_qiscr0_dn4_slot = var_qiscr0_dn4;
        *var_qiscr0_dn6_slot = var_qiscr0_dn6;
        *var_qiscr0_dn7_slot = var_qiscr0_dn7;
        *var_qiscr0_dn8_slot = var_qiscr0_dn8;
        *var_qiscr0_dn9_slot = var_qiscr0_dn9;
        *var_qiscr_dn4_slot = var_qiscr_dn4;
        *var_qiscr_dn6_slot = var_qiscr_dn6;
        *var_qiscr_dn7_slot = var_qiscr_dn7;
        *var_qiscr_dn8_slot = var_qiscr_dn8;
        *var_qiscr_dn9_slot = var_qiscr_dn9;
        *var_sp_s_a_slot = var_sp_s_a;
        *var_sp_s_a_dn4_slot = var_sp_s_a_dn4;
        *var_sp_s_a_dn6_slot = var_sp_s_a_dn6;
        *var_sp_s_a_dn7_slot = var_sp_s_a_dn7;
        *var_sp_s_a_dn8_slot = var_sp_s_a_dn8;
        *var_sp_s_a_dn9_slot = var_sp_s_a_dn9;
        *var_sp_s_c_slot = var_sp_s_c;
        *var_sp_s_c_dn4_slot = var_sp_s_c_dn4;
        *var_sp_s_c_dn6_slot = var_sp_s_c_dn6;
        *var_sp_s_c_dn7_slot = var_sp_s_c_dn7;
        *var_sp_s_c_dn8_slot = var_sp_s_c_dn8;
        *var_sp_s_c_dn9_slot = var_sp_s_c_dn9;
        *var_sp_s_eta_slot = var_sp_s_eta;
        *var_sp_s_eta_dn4_slot = var_sp_s_eta_dn4;
        *var_sp_s_eta_dn6_slot = var_sp_s_eta_dn6;
        *var_sp_s_eta_dn7_slot = var_sp_s_eta_dn7;
        *var_sp_s_eta_dn8_slot = var_sp_s_eta_dn8;
        *var_sp_s_eta_dn9_slot = var_sp_s_eta_dn9;
        *var_sp_s_tau_slot = var_sp_s_tau;
        *var_sp_s_tau_dn4_slot = var_sp_s_tau_dn4;
        *var_sp_s_tau_dn6_slot = var_sp_s_tau_dn6;
        *var_sp_s_tau_dn7_slot = var_sp_s_tau_dn7;
        *var_sp_s_tau_dn8_slot = var_sp_s_tau_dn8;
        *var_sp_s_tau_dn9_slot = var_sp_s_tau_dn9;
        *var_sp_s_temp_slot = var_sp_s_temp;
        *var_sp_s_temp1_slot = var_sp_s_temp1;
        *var_sp_s_temp1_dn4_slot = var_sp_s_temp1_dn4;
        *var_sp_s_temp1_dn6_slot = var_sp_s_temp1_dn6;
        *var_sp_s_temp1_dn7_slot = var_sp_s_temp1_dn7;
        *var_sp_s_temp1_dn8_slot = var_sp_s_temp1_dn8;
        *var_sp_s_temp1_dn9_slot = var_sp_s_temp1_dn9;
        *var_sp_s_temp_dn4_slot = var_sp_s_temp_dn4;
        *var_sp_s_temp_dn6_slot = var_sp_s_temp_dn6;
        *var_sp_s_temp_dn7_slot = var_sp_s_temp_dn7;
        *var_sp_s_temp_dn8_slot = var_sp_s_temp_dn8;
        *var_sp_s_temp_dn9_slot = var_sp_s_temp_dn9;
        *var_sp_s_x1_slot = var_sp_s_x1;
        *var_sp_s_x1_dn4_slot = var_sp_s_x1_dn4;
        *var_sp_s_x1_dn6_slot = var_sp_s_x1_dn6;
        *var_sp_s_x1_dn7_slot = var_sp_s_x1_dn7;
        *var_sp_s_x1_dn8_slot = var_sp_s_x1_dn8;
        *var_sp_s_x1_dn9_slot = var_sp_s_x1_dn9;
        *var_sp_s_yg_slot = var_sp_s_yg;
        *var_sp_s_yg_dn4_slot = var_sp_s_yg_dn4;
        *var_sp_s_yg_dn6_slot = var_sp_s_yg_dn6;
        *var_sp_s_yg_dn7_slot = var_sp_s_yg_dn7;
        *var_sp_s_yg_dn8_slot = var_sp_s_yg_dn8;
        *var_sp_s_yg_dn9_slot = var_sp_s_yg_dn9;
        *var_sp_s_ysub_slot = var_sp_s_ysub;
        *var_sp_s_ysub_dn4_slot = var_sp_s_ysub_dn4;
        *var_sp_s_ysub_dn6_slot = var_sp_s_ysub_dn6;
        *var_sp_s_ysub_dn7_slot = var_sp_s_ysub_dn7;
        *var_sp_s_ysub_dn8_slot = var_sp_s_ysub_dn8;
        *var_sp_s_ysub_dn9_slot = var_sp_s_ysub_dn9;
        *var_temp__blk949_slot = var_temp__blk949;
        *var_temp__blk949_dn4_slot = var_temp__blk949_dn4;
        *var_temp__blk949_dn6_slot = var_temp__blk949_dn6;
        *var_temp__blk949_dn7_slot = var_temp__blk949_dn7;
        *var_temp__blk949_dn8_slot = var_temp__blk949_dn8;
        *var_temp__blk949_dn9_slot = var_temp__blk949_dn9;
        *var_x_s_slot = var_x_s;
        *var_x_s_dn4_slot = var_x_s_dn4;
        *var_x_s_dn6_slot = var_x_s_dn6;
        *var_x_s_dn7_slot = var_x_s_dn7;
        *var_x_s_dn8_slot = var_x_s_dn8;
        *var_x_s_dn9_slot = var_x_s_dn9;
        *var_xi_slot = var_xi;
        *var_xi_dn4_slot = var_xi_dn4;
        *var_xi_dn6_slot = var_xi_dn6;
        *var_xi_dn7_slot = var_xi_dn7;
        *var_xi_dn8_slot = var_xi_dn8;
        *var_xi_dn9_slot = var_xi_dn9;
        *var_xn_s_slot = var_xn_s;
        *var_xn_s_dn4_slot = var_xn_s_dn4;
        *var_xn_s_dn6_slot = var_xn_s_dn6;
        *var_xn_s_dn7_slot = var_xn_s_dn7;
        *var_xn_s_dn8_slot = var_xn_s_dn8;
        *var_xn_s_dn9_slot = var_xn_s_dn9;
    }

    pub(super) fn stamp_transient_block_90(
        var_delta_ns: f64,
        var_delta_ns_dn4: f64,
        var_delta_ns_dn6: f64,
        var_delta_ns_dn7: f64,
        var_delta_ns_dn8: f64,
        var_delta_ns_dn9: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn4: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf2_dn9: f64,
        var_gf_dn4: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_gf_dn9: f64,
        var_guard1199: f64,
        var_guard1200: f64,
        var_inv_xi: f64,
        var_inv_xi_dn4: f64,
        var_inv_xi_dn6: f64,
        var_inv_xi_dn7: f64,
        var_inv_xi_dn8: f64,
        var_inv_xi_dn9: f64,
        var_mutau: f64,
        var_mutau_dn4: f64,
        var_mutau_dn6: f64,
        var_mutau_dn7: f64,
        var_mutau_dn8: f64,
        var_mutau_dn9: f64,
        var_nu: f64,
        var_nu_dn4: f64,
        var_nu_dn6: f64,
        var_nu_dn7: f64,
        var_nu_dn8: f64,
        var_nu_dn9: f64,
        var_sp_s_a: f64,
        var_sp_s_a_dn4: f64,
        var_sp_s_a_dn6: f64,
        var_sp_s_a_dn7: f64,
        var_sp_s_a_dn8: f64,
        var_sp_s_a_dn9: f64,
        var_sp_s_c: f64,
        var_sp_s_c_dn4: f64,
        var_sp_s_c_dn6: f64,
        var_sp_s_c_dn7: f64,
        var_sp_s_c_dn8: f64,
        var_sp_s_c_dn9: f64,
        var_sp_s_tau: f64,
        var_sp_s_tau_dn4: f64,
        var_sp_s_tau_dn6: f64,
        var_sp_s_tau_dn7: f64,
        var_sp_s_tau_dn8: f64,
        var_sp_s_tau_dn9: f64,
        var_sp_s_yg: f64,
        var_sp_s_yg_dn4: f64,
        var_sp_s_yg_dn6: f64,
        var_sp_s_yg_dn7: f64,
        var_sp_s_yg_dn8: f64,
        var_sp_s_yg_dn9: f64,
        var_xg: f64,
        var_xg_dn4: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xg_dn9: f64,
        var_xi: f64,
        var_xi_dn4: f64,
        var_xi_dn6: f64,
        var_xi_dn7: f64,
        var_xi_dn8: f64,
        var_xi_dn9: f64,
        var_xn_s: f64,
        var_xn_s_dn4: f64,
        var_xn_s_dn6: f64,
        var_xn_s_dn7: f64,
        var_xn_s_dn8: f64,
        var_xn_s_dn9: f64,
        var_guard1201_slot: &mut f64,
        var_guard1202_slot: &mut f64,
        var_sp_s_a_fac_slot: &mut f64,
        var_sp_s_a_fac_dn4_slot: &mut f64,
        var_sp_s_a_fac_dn6_slot: &mut f64,
        var_sp_s_a_fac_dn7_slot: &mut f64,
        var_sp_s_a_fac_dn8_slot: &mut f64,
        var_sp_s_a_fac_dn9_slot: &mut f64,
        var_sp_s_bx_slot: &mut f64,
        var_sp_s_bx_dn4_slot: &mut f64,
        var_sp_s_bx_dn6_slot: &mut f64,
        var_sp_s_bx_dn7_slot: &mut f64,
        var_sp_s_bx_dn8_slot: &mut f64,
        var_sp_s_bx_dn9_slot: &mut f64,
        var_sp_s_delta0_slot: &mut f64,
        var_sp_s_delta0_dn4_slot: &mut f64,
        var_sp_s_delta0_dn6_slot: &mut f64,
        var_sp_s_delta0_dn7_slot: &mut f64,
        var_sp_s_delta0_dn8_slot: &mut f64,
        var_sp_s_delta0_dn9_slot: &mut f64,
        var_sp_s_delta1_slot: &mut f64,
        var_sp_s_delta1_dn4_slot: &mut f64,
        var_sp_s_delta1_dn6_slot: &mut f64,
        var_sp_s_delta1_dn7_slot: &mut f64,
        var_sp_s_delta1_dn8_slot: &mut f64,
        var_sp_s_delta1_dn9_slot: &mut f64,
        var_sp_s_eta_slot: &mut f64,
        var_sp_s_eta_dn4_slot: &mut f64,
        var_sp_s_eta_dn6_slot: &mut f64,
        var_sp_s_eta_dn7_slot: &mut f64,
        var_sp_s_eta_dn8_slot: &mut f64,
        var_sp_s_eta_dn9_slot: &mut f64,
        var_sp_s_pc_slot: &mut f64,
        var_sp_s_pc_dn4_slot: &mut f64,
        var_sp_s_pc_dn6_slot: &mut f64,
        var_sp_s_pc_dn7_slot: &mut f64,
        var_sp_s_pc_dn8_slot: &mut f64,
        var_sp_s_pc_dn9_slot: &mut f64,
        var_sp_s_qc_slot: &mut f64,
        var_sp_s_qc_dn4_slot: &mut f64,
        var_sp_s_qc_dn6_slot: &mut f64,
        var_sp_s_qc_dn7_slot: &mut f64,
        var_sp_s_qc_dn8_slot: &mut f64,
        var_sp_s_qc_dn9_slot: &mut f64,
        var_sp_s_temp_slot: &mut f64,
        var_sp_s_temp1_slot: &mut f64,
        var_sp_s_temp1_dn4_slot: &mut f64,
        var_sp_s_temp1_dn6_slot: &mut f64,
        var_sp_s_temp1_dn7_slot: &mut f64,
        var_sp_s_temp1_dn8_slot: &mut f64,
        var_sp_s_temp1_dn9_slot: &mut f64,
        var_sp_s_temp_dn4_slot: &mut f64,
        var_sp_s_temp_dn6_slot: &mut f64,
        var_sp_s_temp_dn7_slot: &mut f64,
        var_sp_s_temp_dn8_slot: &mut f64,
        var_sp_s_temp_dn9_slot: &mut f64,
        var_sp_s_w_slot: &mut f64,
        var_sp_s_w_dn4_slot: &mut f64,
        var_sp_s_w_dn6_slot: &mut f64,
        var_sp_s_w_dn7_slot: &mut f64,
        var_sp_s_w_dn8_slot: &mut f64,
        var_sp_s_w_dn9_slot: &mut f64,
        var_sp_s_x1_slot: &mut f64,
        var_sp_s_x1_dn4_slot: &mut f64,
        var_sp_s_x1_dn6_slot: &mut f64,
        var_sp_s_x1_dn7_slot: &mut f64,
        var_sp_s_x1_dn8_slot: &mut f64,
        var_sp_s_x1_dn9_slot: &mut f64,
        var_sp_s_xbar_slot: &mut f64,
        var_sp_s_xbar_dn4_slot: &mut f64,
        var_sp_s_xbar_dn6_slot: &mut f64,
        var_sp_s_xbar_dn7_slot: &mut f64,
        var_sp_s_xbar_dn8_slot: &mut f64,
        var_sp_s_xbar_dn9_slot: &mut f64,
        var_sp_s_xi0_slot: &mut f64,
        var_sp_s_xi0_dn4_slot: &mut f64,
        var_sp_s_xi0_dn6_slot: &mut f64,
        var_sp_s_xi0_dn7_slot: &mut f64,
        var_sp_s_xi0_dn8_slot: &mut f64,
        var_sp_s_xi0_dn9_slot: &mut f64,
        var_sp_s_xi1_slot: &mut f64,
        var_sp_s_xi1_dn4_slot: &mut f64,
        var_sp_s_xi1_dn6_slot: &mut f64,
        var_sp_s_xi1_dn7_slot: &mut f64,
        var_sp_s_xi1_dn8_slot: &mut f64,
        var_sp_s_xi1_dn9_slot: &mut f64,
        var_sp_s_xi2_slot: &mut f64,
        var_sp_s_xi2_dn4_slot: &mut f64,
        var_sp_s_xi2_dn6_slot: &mut f64,
        var_sp_s_xi2_dn7_slot: &mut f64,
        var_sp_s_xi2_dn8_slot: &mut f64,
        var_sp_s_xi2_dn9_slot: &mut f64,
        var_sp_s_y0_slot: &mut f64,
        var_sp_s_y0_dn4_slot: &mut f64,
        var_sp_s_y0_dn6_slot: &mut f64,
        var_sp_s_y0_dn7_slot: &mut f64,
        var_sp_s_y0_dn8_slot: &mut f64,
        var_sp_s_y0_dn9_slot: &mut f64,
        var_sp_xg1_slot: &mut f64,
        var_sp_xg1_dn4_slot: &mut f64,
        var_sp_xg1_dn6_slot: &mut f64,
        var_sp_xg1_dn7_slot: &mut f64,
        var_sp_xg1_dn8_slot: &mut f64,
        var_sp_xg1_dn9_slot: &mut f64,
        var_x_s_slot: &mut f64,
        var_x_s_dn4_slot: &mut f64,
        var_x_s_dn6_slot: &mut f64,
        var_x_s_dn7_slot: &mut f64,
        var_x_s_dn8_slot: &mut f64,
        var_x_s_dn9_slot: &mut f64,
    ) {
        let mut var_guard1201: f64 = *var_guard1201_slot;
        let mut var_guard1202: f64 = *var_guard1202_slot;
        let mut var_sp_s_a_fac: f64 = *var_sp_s_a_fac_slot;
        let mut var_sp_s_a_fac_dn4: f64 = *var_sp_s_a_fac_dn4_slot;
        let mut var_sp_s_a_fac_dn6: f64 = *var_sp_s_a_fac_dn6_slot;
        let mut var_sp_s_a_fac_dn7: f64 = *var_sp_s_a_fac_dn7_slot;
        let mut var_sp_s_a_fac_dn8: f64 = *var_sp_s_a_fac_dn8_slot;
        let mut var_sp_s_a_fac_dn9: f64 = *var_sp_s_a_fac_dn9_slot;
        let mut var_sp_s_bx: f64 = *var_sp_s_bx_slot;
        let mut var_sp_s_bx_dn4: f64 = *var_sp_s_bx_dn4_slot;
        let mut var_sp_s_bx_dn6: f64 = *var_sp_s_bx_dn6_slot;
        let mut var_sp_s_bx_dn7: f64 = *var_sp_s_bx_dn7_slot;
        let mut var_sp_s_bx_dn8: f64 = *var_sp_s_bx_dn8_slot;
        let mut var_sp_s_bx_dn9: f64 = *var_sp_s_bx_dn9_slot;
        let mut var_sp_s_delta0: f64 = *var_sp_s_delta0_slot;
        let mut var_sp_s_delta0_dn4: f64 = *var_sp_s_delta0_dn4_slot;
        let mut var_sp_s_delta0_dn6: f64 = *var_sp_s_delta0_dn6_slot;
        let mut var_sp_s_delta0_dn7: f64 = *var_sp_s_delta0_dn7_slot;
        let mut var_sp_s_delta0_dn8: f64 = *var_sp_s_delta0_dn8_slot;
        let mut var_sp_s_delta0_dn9: f64 = *var_sp_s_delta0_dn9_slot;
        let mut var_sp_s_delta1: f64 = *var_sp_s_delta1_slot;
        let mut var_sp_s_delta1_dn4: f64 = *var_sp_s_delta1_dn4_slot;
        let mut var_sp_s_delta1_dn6: f64 = *var_sp_s_delta1_dn6_slot;
        let mut var_sp_s_delta1_dn7: f64 = *var_sp_s_delta1_dn7_slot;
        let mut var_sp_s_delta1_dn8: f64 = *var_sp_s_delta1_dn8_slot;
        let mut var_sp_s_delta1_dn9: f64 = *var_sp_s_delta1_dn9_slot;
        let mut var_sp_s_eta: f64 = *var_sp_s_eta_slot;
        let mut var_sp_s_eta_dn4: f64 = *var_sp_s_eta_dn4_slot;
        let mut var_sp_s_eta_dn6: f64 = *var_sp_s_eta_dn6_slot;
        let mut var_sp_s_eta_dn7: f64 = *var_sp_s_eta_dn7_slot;
        let mut var_sp_s_eta_dn8: f64 = *var_sp_s_eta_dn8_slot;
        let mut var_sp_s_eta_dn9: f64 = *var_sp_s_eta_dn9_slot;
        let mut var_sp_s_pc: f64 = *var_sp_s_pc_slot;
        let mut var_sp_s_pc_dn4: f64 = *var_sp_s_pc_dn4_slot;
        let mut var_sp_s_pc_dn6: f64 = *var_sp_s_pc_dn6_slot;
        let mut var_sp_s_pc_dn7: f64 = *var_sp_s_pc_dn7_slot;
        let mut var_sp_s_pc_dn8: f64 = *var_sp_s_pc_dn8_slot;
        let mut var_sp_s_pc_dn9: f64 = *var_sp_s_pc_dn9_slot;
        let mut var_sp_s_qc: f64 = *var_sp_s_qc_slot;
        let mut var_sp_s_qc_dn4: f64 = *var_sp_s_qc_dn4_slot;
        let mut var_sp_s_qc_dn6: f64 = *var_sp_s_qc_dn6_slot;
        let mut var_sp_s_qc_dn7: f64 = *var_sp_s_qc_dn7_slot;
        let mut var_sp_s_qc_dn8: f64 = *var_sp_s_qc_dn8_slot;
        let mut var_sp_s_qc_dn9: f64 = *var_sp_s_qc_dn9_slot;
        let mut var_sp_s_temp: f64 = *var_sp_s_temp_slot;
        let mut var_sp_s_temp1: f64 = *var_sp_s_temp1_slot;
        let mut var_sp_s_temp1_dn4: f64 = *var_sp_s_temp1_dn4_slot;
        let mut var_sp_s_temp1_dn6: f64 = *var_sp_s_temp1_dn6_slot;
        let mut var_sp_s_temp1_dn7: f64 = *var_sp_s_temp1_dn7_slot;
        let mut var_sp_s_temp1_dn8: f64 = *var_sp_s_temp1_dn8_slot;
        let mut var_sp_s_temp1_dn9: f64 = *var_sp_s_temp1_dn9_slot;
        let mut var_sp_s_temp_dn4: f64 = *var_sp_s_temp_dn4_slot;
        let mut var_sp_s_temp_dn6: f64 = *var_sp_s_temp_dn6_slot;
        let mut var_sp_s_temp_dn7: f64 = *var_sp_s_temp_dn7_slot;
        let mut var_sp_s_temp_dn8: f64 = *var_sp_s_temp_dn8_slot;
        let mut var_sp_s_temp_dn9: f64 = *var_sp_s_temp_dn9_slot;
        let mut var_sp_s_w: f64 = *var_sp_s_w_slot;
        let mut var_sp_s_w_dn4: f64 = *var_sp_s_w_dn4_slot;
        let mut var_sp_s_w_dn6: f64 = *var_sp_s_w_dn6_slot;
        let mut var_sp_s_w_dn7: f64 = *var_sp_s_w_dn7_slot;
        let mut var_sp_s_w_dn8: f64 = *var_sp_s_w_dn8_slot;
        let mut var_sp_s_w_dn9: f64 = *var_sp_s_w_dn9_slot;
        let mut var_sp_s_x1: f64 = *var_sp_s_x1_slot;
        let mut var_sp_s_x1_dn4: f64 = *var_sp_s_x1_dn4_slot;
        let mut var_sp_s_x1_dn6: f64 = *var_sp_s_x1_dn6_slot;
        let mut var_sp_s_x1_dn7: f64 = *var_sp_s_x1_dn7_slot;
        let mut var_sp_s_x1_dn8: f64 = *var_sp_s_x1_dn8_slot;
        let mut var_sp_s_x1_dn9: f64 = *var_sp_s_x1_dn9_slot;
        let mut var_sp_s_xbar: f64 = *var_sp_s_xbar_slot;
        let mut var_sp_s_xbar_dn4: f64 = *var_sp_s_xbar_dn4_slot;
        let mut var_sp_s_xbar_dn6: f64 = *var_sp_s_xbar_dn6_slot;
        let mut var_sp_s_xbar_dn7: f64 = *var_sp_s_xbar_dn7_slot;
        let mut var_sp_s_xbar_dn8: f64 = *var_sp_s_xbar_dn8_slot;
        let mut var_sp_s_xbar_dn9: f64 = *var_sp_s_xbar_dn9_slot;
        let mut var_sp_s_xi0: f64 = *var_sp_s_xi0_slot;
        let mut var_sp_s_xi0_dn4: f64 = *var_sp_s_xi0_dn4_slot;
        let mut var_sp_s_xi0_dn6: f64 = *var_sp_s_xi0_dn6_slot;
        let mut var_sp_s_xi0_dn7: f64 = *var_sp_s_xi0_dn7_slot;
        let mut var_sp_s_xi0_dn8: f64 = *var_sp_s_xi0_dn8_slot;
        let mut var_sp_s_xi0_dn9: f64 = *var_sp_s_xi0_dn9_slot;
        let mut var_sp_s_xi1: f64 = *var_sp_s_xi1_slot;
        let mut var_sp_s_xi1_dn4: f64 = *var_sp_s_xi1_dn4_slot;
        let mut var_sp_s_xi1_dn6: f64 = *var_sp_s_xi1_dn6_slot;
        let mut var_sp_s_xi1_dn7: f64 = *var_sp_s_xi1_dn7_slot;
        let mut var_sp_s_xi1_dn8: f64 = *var_sp_s_xi1_dn8_slot;
        let mut var_sp_s_xi1_dn9: f64 = *var_sp_s_xi1_dn9_slot;
        let mut var_sp_s_xi2: f64 = *var_sp_s_xi2_slot;
        let mut var_sp_s_xi2_dn4: f64 = *var_sp_s_xi2_dn4_slot;
        let mut var_sp_s_xi2_dn6: f64 = *var_sp_s_xi2_dn6_slot;
        let mut var_sp_s_xi2_dn7: f64 = *var_sp_s_xi2_dn7_slot;
        let mut var_sp_s_xi2_dn8: f64 = *var_sp_s_xi2_dn8_slot;
        let mut var_sp_s_xi2_dn9: f64 = *var_sp_s_xi2_dn9_slot;
        let mut var_sp_s_y0: f64 = *var_sp_s_y0_slot;
        let mut var_sp_s_y0_dn4: f64 = *var_sp_s_y0_dn4_slot;
        let mut var_sp_s_y0_dn6: f64 = *var_sp_s_y0_dn6_slot;
        let mut var_sp_s_y0_dn7: f64 = *var_sp_s_y0_dn7_slot;
        let mut var_sp_s_y0_dn8: f64 = *var_sp_s_y0_dn8_slot;
        let mut var_sp_s_y0_dn9: f64 = *var_sp_s_y0_dn9_slot;
        let mut var_sp_xg1: f64 = *var_sp_xg1_slot;
        let mut var_sp_xg1_dn4: f64 = *var_sp_xg1_dn4_slot;
        let mut var_sp_xg1_dn6: f64 = *var_sp_xg1_dn6_slot;
        let mut var_sp_xg1_dn7: f64 = *var_sp_xg1_dn7_slot;
        let mut var_sp_xg1_dn8: f64 = *var_sp_xg1_dn8_slot;
        let mut var_sp_xg1_dn9: f64 = *var_sp_xg1_dn9_slot;
        let mut var_x_s: f64 = *var_x_s_slot;
        let mut var_x_s_dn4: f64 = *var_x_s_dn4_slot;
        let mut var_x_s_dn6: f64 = *var_x_s_dn6_slot;
        let mut var_x_s_dn7: f64 = *var_x_s_dn7_slot;
        let mut var_x_s_dn8: f64 = *var_x_s_dn8_slot;
        let mut var_x_s_dn9: f64 = *var_x_s_dn9_slot;

        let (assign41900_e54896, assign41900_e54896_d_n4, assign41900_e54896_d_n6, assign41900_e54896_d_n7, assign41900_e54896_d_n8, assign41900_e54896_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign41900_e54871: f64 = (var_sp_s_a * var_nu);
        let assign41900_e54873: f64 = (assign41900_e54871 * var_sp_s_tau);
        let assign41900_e54877: f64 = (var_nu / var_mutau);
        let assign41900_e54879: f64 = (assign41900_e54877 * var_sp_s_tau);
        let assign41900_e54881: f64 = (assign41900_e54879 * var_sp_s_tau);
        let assign41900_e54883: f64 = (assign41900_e54881 * var_sp_s_c);
        let assign41900_e54886: f64 = (var_sp_s_c * var_sp_s_c);
        let assign41900_e54888: f64 = (assign41900_e54886 * 0.3333333333333333);
        let assign41900_e54890: f64 = (assign41900_e54888 - var_sp_s_a);
        let assign41900_e54891: f64 = (assign41900_e54883 * assign41900_e54890);
        let assign41900_e54892: f64 = (var_mutau + assign41900_e54891);
        let assign41900_e54893: f64 = (assign41900_e54873 / assign41900_e54892);
        let assign41900_e54894: f64 = (var_sp_s_eta + assign41900_e54893);
        (assign41900_e54894, (var_sp_s_eta_dn4 + (((((((var_sp_s_a_dn4 * var_nu) + (var_sp_s_a * var_nu_dn4)) * var_sp_s_tau) + (assign41900_e54871 * var_sp_s_tau_dn4)) * assign41900_e54892) - (assign41900_e54873 * (var_mutau_dn4 + (((((((((((var_nu_dn4 * var_mutau) - (var_nu * var_mutau_dn4)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign41900_e54877 * var_sp_s_tau_dn4)) * var_sp_s_tau) + (assign41900_e54879 * var_sp_s_tau_dn4)) * var_sp_s_c) + (assign41900_e54881 * var_sp_s_c_dn4)) * assign41900_e54890) + (assign41900_e54883 * ((((var_sp_s_c_dn4 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn4)) * 0.3333333333333333) - var_sp_s_a_dn4)))))) / (assign41900_e54892 * assign41900_e54892))), (var_sp_s_eta_dn6 + (((((((var_sp_s_a_dn6 * var_nu) + (var_sp_s_a * var_nu_dn6)) * var_sp_s_tau) + (assign41900_e54871 * var_sp_s_tau_dn6)) * assign41900_e54892) - (assign41900_e54873 * (var_mutau_dn6 + (((((((((((var_nu_dn6 * var_mutau) - (var_nu * var_mutau_dn6)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign41900_e54877 * var_sp_s_tau_dn6)) * var_sp_s_tau) + (assign41900_e54879 * var_sp_s_tau_dn6)) * var_sp_s_c) + (assign41900_e54881 * var_sp_s_c_dn6)) * assign41900_e54890) + (assign41900_e54883 * ((((var_sp_s_c_dn6 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn6)) * 0.3333333333333333) - var_sp_s_a_dn6)))))) / (assign41900_e54892 * assign41900_e54892))), (var_sp_s_eta_dn7 + (((((((var_sp_s_a_dn7 * var_nu) + (var_sp_s_a * var_nu_dn7)) * var_sp_s_tau) + (assign41900_e54871 * var_sp_s_tau_dn7)) * assign41900_e54892) - (assign41900_e54873 * (var_mutau_dn7 + (((((((((((var_nu_dn7 * var_mutau) - (var_nu * var_mutau_dn7)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign41900_e54877 * var_sp_s_tau_dn7)) * var_sp_s_tau) + (assign41900_e54879 * var_sp_s_tau_dn7)) * var_sp_s_c) + (assign41900_e54881 * var_sp_s_c_dn7)) * assign41900_e54890) + (assign41900_e54883 * ((((var_sp_s_c_dn7 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn7)) * 0.3333333333333333) - var_sp_s_a_dn7)))))) / (assign41900_e54892 * assign41900_e54892))), (var_sp_s_eta_dn8 + (((((((var_sp_s_a_dn8 * var_nu) + (var_sp_s_a * var_nu_dn8)) * var_sp_s_tau) + (assign41900_e54871 * var_sp_s_tau_dn8)) * assign41900_e54892) - (assign41900_e54873 * (var_mutau_dn8 + (((((((((((var_nu_dn8 * var_mutau) - (var_nu * var_mutau_dn8)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign41900_e54877 * var_sp_s_tau_dn8)) * var_sp_s_tau) + (assign41900_e54879 * var_sp_s_tau_dn8)) * var_sp_s_c) + (assign41900_e54881 * var_sp_s_c_dn8)) * assign41900_e54890) + (assign41900_e54883 * ((((var_sp_s_c_dn8 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn8)) * 0.3333333333333333) - var_sp_s_a_dn8)))))) / (assign41900_e54892 * assign41900_e54892))), (var_sp_s_eta_dn9 + (((((((var_sp_s_a_dn9 * var_nu) + (var_sp_s_a * var_nu_dn9)) * var_sp_s_tau) + (assign41900_e54871 * var_sp_s_tau_dn9)) * assign41900_e54892) - (assign41900_e54873 * (var_mutau_dn9 + (((((((((((var_nu_dn9 * var_mutau) - (var_nu * var_mutau_dn9)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign41900_e54877 * var_sp_s_tau_dn9)) * var_sp_s_tau) + (assign41900_e54879 * var_sp_s_tau_dn9)) * var_sp_s_c) + (assign41900_e54881 * var_sp_s_c_dn9)) * assign41900_e54890) + (assign41900_e54883 * ((((var_sp_s_c_dn9 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn9)) * 0.3333333333333333) - var_sp_s_a_dn9)))))) / (assign41900_e54892 * assign41900_e54892))),)
    } else {
        (var_sp_s_y0, var_sp_s_y0_dn4, var_sp_s_y0_dn6, var_sp_s_y0_dn7, var_sp_s_y0_dn8, var_sp_s_y0_dn9,)
    }
};
        var_sp_s_y0 = assign41900_e54896;
        var_sp_s_y0_dn4 = assign41900_e54896_d_n4;
        var_sp_s_y0_dn6 = assign41900_e54896_d_n6;
        var_sp_s_y0_dn7 = assign41900_e54896_d_n7;
        var_sp_s_y0_dn8 = assign41900_e54896_d_n8;
        var_sp_s_y0_dn9 = assign41900_e54896_d_n9;

        let assign41910_e54899: f64 = if var_sp_s_y0 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1201 = assign41910_e54899;

        let (assign41920_e54909, assign41920_e54909_d_n4, assign41920_e54909_d_n6, assign41920_e54909_d_n7, assign41920_e54909_d_n8, assign41920_e54909_d_n9,) = {
    if (((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign41920_e54907: f64 = (var_sp_s_y0).exp();
        (assign41920_e54907, (assign41920_e54907 * var_sp_s_y0_dn4), (assign41920_e54907 * var_sp_s_y0_dn6), (assign41920_e54907 * var_sp_s_y0_dn7), (assign41920_e54907 * var_sp_s_y0_dn8), (assign41920_e54907 * var_sp_s_y0_dn9),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn4, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8, var_sp_s_delta0_dn9,)
    }
};
        var_sp_s_delta0 = assign41920_e54909;
        var_sp_s_delta0_dn4 = assign41920_e54909_d_n4;
        var_sp_s_delta0_dn6 = assign41920_e54909_d_n6;
        var_sp_s_delta0_dn7 = assign41920_e54909_d_n7;
        var_sp_s_delta0_dn8 = assign41920_e54909_d_n8;
        var_sp_s_delta0_dn9 = assign41920_e54909_d_n9;

        let (assign41930_e54941, assign41930_e54941_d_n4, assign41930_e54941_d_n6, assign41930_e54941_d_n7, assign41930_e54941_d_n8, assign41930_e54941_d_n9,) = {
    if (((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) && (var_guard1201 == 0.0)) {
        let assign41930_e54921: f64 = (var_sp_s_y0 - 230.25850929940458);
        let assign41930_e54926: f64 = (var_sp_s_y0 - 230.25850929940458);
        let assign41930_e54930: f64 = (var_sp_s_y0 - 230.25850929940458);
        let assign41930_e54932: f64 = (assign41930_e54930 * 0.3333333333333333);
        let assign41930_e54933: f64 = (1.0 + assign41930_e54932);
        let assign41930_e54934: f64 = (assign41930_e54926 * assign41930_e54933);
        let assign41930_e54935: f64 = (0.5 * assign41930_e54934);
        let assign41930_e54936: f64 = (1.0 + assign41930_e54935);
        let assign41930_e54937: f64 = (assign41930_e54921 * assign41930_e54936);
        let assign41930_e54938: f64 = (1.0 + assign41930_e54937);
        let assign41930_e54939: f64 = (1e100 * assign41930_e54938);
        (assign41930_e54939, (1e100 * ((var_sp_s_y0_dn4 * assign41930_e54936) + (assign41930_e54921 * (0.5 * ((var_sp_s_y0_dn4 * assign41930_e54933) + (assign41930_e54926 * (var_sp_s_y0_dn4 * 0.3333333333333333))))))), (1e100 * ((var_sp_s_y0_dn6 * assign41930_e54936) + (assign41930_e54921 * (0.5 * ((var_sp_s_y0_dn6 * assign41930_e54933) + (assign41930_e54926 * (var_sp_s_y0_dn6 * 0.3333333333333333))))))), (1e100 * ((var_sp_s_y0_dn7 * assign41930_e54936) + (assign41930_e54921 * (0.5 * ((var_sp_s_y0_dn7 * assign41930_e54933) + (assign41930_e54926 * (var_sp_s_y0_dn7 * 0.3333333333333333))))))), (1e100 * ((var_sp_s_y0_dn8 * assign41930_e54936) + (assign41930_e54921 * (0.5 * ((var_sp_s_y0_dn8 * assign41930_e54933) + (assign41930_e54926 * (var_sp_s_y0_dn8 * 0.3333333333333333))))))), (1e100 * ((var_sp_s_y0_dn9 * assign41930_e54936) + (assign41930_e54921 * (0.5 * ((var_sp_s_y0_dn9 * assign41930_e54933) + (assign41930_e54926 * (var_sp_s_y0_dn9 * 0.3333333333333333))))))),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn4, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8, var_sp_s_delta0_dn9,)
    }
};
        var_sp_s_delta0 = assign41930_e54941;
        var_sp_s_delta0_dn4 = assign41930_e54941_d_n4;
        var_sp_s_delta0_dn6 = assign41930_e54941_d_n6;
        var_sp_s_delta0_dn7 = assign41930_e54941_d_n7;
        var_sp_s_delta0_dn8 = assign41930_e54941_d_n8;
        var_sp_s_delta0_dn9 = assign41930_e54941_d_n9;

        let (assign41940_e54950, assign41940_e54950_d_n4, assign41940_e54950_d_n6, assign41940_e54950_d_n7, assign41940_e54950_d_n8, assign41940_e54950_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign41940_e54948: f64 = (1.0 / var_sp_s_delta0);
        (assign41940_e54948, (-(var_sp_s_delta0_dn4 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn6 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn7 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn8 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn9 / (var_sp_s_delta0 * var_sp_s_delta0))),)
    } else {
        (var_sp_s_delta1, var_sp_s_delta1_dn4, var_sp_s_delta1_dn6, var_sp_s_delta1_dn7, var_sp_s_delta1_dn8, var_sp_s_delta1_dn9,)
    }
};
        var_sp_s_delta1 = assign41940_e54950;
        var_sp_s_delta1_dn4 = assign41940_e54950_d_n4;
        var_sp_s_delta1_dn6 = assign41940_e54950_d_n6;
        var_sp_s_delta1_dn7 = assign41940_e54950_d_n7;
        var_sp_s_delta1_dn8 = assign41940_e54950_d_n8;
        var_sp_s_delta1_dn9 = assign41940_e54950_d_n9;

        let (assign41950_e54963, assign41950_e54963_d_n4, assign41950_e54963_d_n6, assign41950_e54963_d_n7, assign41950_e54963_d_n8, assign41950_e54963_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign41950_e54959: f64 = (var_sp_s_y0 * var_sp_s_y0);
        let assign41950_e54960: f64 = (2.0 + assign41950_e54959);
        let assign41950_e54961: f64 = (1.0 / assign41950_e54960);
        (assign41950_e54961, (-(((var_sp_s_y0_dn4 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn4)) / (assign41950_e54960 * assign41950_e54960))), (-(((var_sp_s_y0_dn6 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn6)) / (assign41950_e54960 * assign41950_e54960))), (-(((var_sp_s_y0_dn7 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn7)) / (assign41950_e54960 * assign41950_e54960))), (-(((var_sp_s_y0_dn8 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn8)) / (assign41950_e54960 * assign41950_e54960))), (-(((var_sp_s_y0_dn9 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn9)) / (assign41950_e54960 * assign41950_e54960))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn4, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8, var_sp_s_temp_dn9,)
    }
};
        var_sp_s_temp = assign41950_e54963;
        var_sp_s_temp_dn4 = assign41950_e54963_d_n4;
        var_sp_s_temp_dn6 = assign41950_e54963_d_n6;
        var_sp_s_temp_dn7 = assign41950_e54963_d_n7;
        var_sp_s_temp_dn8 = assign41950_e54963_d_n8;
        var_sp_s_temp_dn9 = assign41950_e54963_d_n9;

        let (assign41960_e54974, assign41960_e54974_d_n4, assign41960_e54974_d_n6, assign41960_e54974_d_n7, assign41960_e54974_d_n8, assign41960_e54974_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign41960_e54970: f64 = (var_sp_s_y0 * var_sp_s_y0);
        let assign41960_e54972: f64 = (assign41960_e54970 * var_sp_s_temp);
        (assign41960_e54972, ((((var_sp_s_y0_dn4 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn4)) * var_sp_s_temp) + (assign41960_e54970 * var_sp_s_temp_dn4)), ((((var_sp_s_y0_dn6 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn6)) * var_sp_s_temp) + (assign41960_e54970 * var_sp_s_temp_dn6)), ((((var_sp_s_y0_dn7 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn7)) * var_sp_s_temp) + (assign41960_e54970 * var_sp_s_temp_dn7)), ((((var_sp_s_y0_dn8 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn8)) * var_sp_s_temp) + (assign41960_e54970 * var_sp_s_temp_dn8)), ((((var_sp_s_y0_dn9 * var_sp_s_y0) + (var_sp_s_y0 * var_sp_s_y0_dn9)) * var_sp_s_temp) + (assign41960_e54970 * var_sp_s_temp_dn9)),)
    } else {
        (var_sp_s_xi0, var_sp_s_xi0_dn4, var_sp_s_xi0_dn6, var_sp_s_xi0_dn7, var_sp_s_xi0_dn8, var_sp_s_xi0_dn9,)
    }
};
        var_sp_s_xi0 = assign41960_e54974;
        var_sp_s_xi0_dn4 = assign41960_e54974_d_n4;
        var_sp_s_xi0_dn6 = assign41960_e54974_d_n6;
        var_sp_s_xi0_dn7 = assign41960_e54974_d_n7;
        var_sp_s_xi0_dn8 = assign41960_e54974_d_n8;
        var_sp_s_xi0_dn9 = assign41960_e54974_d_n9;

        let (assign41970_e54987, assign41970_e54987_d_n4, assign41970_e54987_d_n6, assign41970_e54987_d_n7, assign41970_e54987_d_n8, assign41970_e54987_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign41970_e54982: f64 = (var_sp_s_y0 * var_sp_s_temp);
        let assign41970_e54984: f64 = (assign41970_e54982 * var_sp_s_temp);
        let assign41970_e54985: f64 = (4.0 * assign41970_e54984);
        (assign41970_e54985, (4.0 * ((((var_sp_s_y0_dn4 * var_sp_s_temp) + (var_sp_s_y0 * var_sp_s_temp_dn4)) * var_sp_s_temp) + (assign41970_e54982 * var_sp_s_temp_dn4))), (4.0 * ((((var_sp_s_y0_dn6 * var_sp_s_temp) + (var_sp_s_y0 * var_sp_s_temp_dn6)) * var_sp_s_temp) + (assign41970_e54982 * var_sp_s_temp_dn6))), (4.0 * ((((var_sp_s_y0_dn7 * var_sp_s_temp) + (var_sp_s_y0 * var_sp_s_temp_dn7)) * var_sp_s_temp) + (assign41970_e54982 * var_sp_s_temp_dn7))), (4.0 * ((((var_sp_s_y0_dn8 * var_sp_s_temp) + (var_sp_s_y0 * var_sp_s_temp_dn8)) * var_sp_s_temp) + (assign41970_e54982 * var_sp_s_temp_dn8))), (4.0 * ((((var_sp_s_y0_dn9 * var_sp_s_temp) + (var_sp_s_y0 * var_sp_s_temp_dn9)) * var_sp_s_temp) + (assign41970_e54982 * var_sp_s_temp_dn9))),)
    } else {
        (var_sp_s_xi1, var_sp_s_xi1_dn4, var_sp_s_xi1_dn6, var_sp_s_xi1_dn7, var_sp_s_xi1_dn8, var_sp_s_xi1_dn9,)
    }
};
        var_sp_s_xi1 = assign41970_e54987;
        var_sp_s_xi1_dn4 = assign41970_e54987_d_n4;
        var_sp_s_xi1_dn6 = assign41970_e54987_d_n6;
        var_sp_s_xi1_dn7 = assign41970_e54987_d_n7;
        var_sp_s_xi1_dn8 = assign41970_e54987_d_n8;
        var_sp_s_xi1_dn9 = assign41970_e54987_d_n9;

        let (assign41980_e55004, assign41980_e55004_d_n4, assign41980_e55004_d_n6, assign41980_e55004_d_n7, assign41980_e55004_d_n8, assign41980_e55004_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign41980_e54994: f64 = (8.0 * var_sp_s_temp);
        let assign41980_e54997: f64 = (12.0 * var_sp_s_xi0);
        let assign41980_e54998: f64 = (assign41980_e54994 - assign41980_e54997);
        let assign41980_e55000: f64 = (assign41980_e54998 * var_sp_s_temp);
        let assign41980_e55002: f64 = (assign41980_e55000 * var_sp_s_temp);
        (assign41980_e55002, ((((((8.0 * var_sp_s_temp_dn4) - (12.0 * var_sp_s_xi0_dn4)) * var_sp_s_temp) + (assign41980_e54998 * var_sp_s_temp_dn4)) * var_sp_s_temp) + (assign41980_e55000 * var_sp_s_temp_dn4)), ((((((8.0 * var_sp_s_temp_dn6) - (12.0 * var_sp_s_xi0_dn6)) * var_sp_s_temp) + (assign41980_e54998 * var_sp_s_temp_dn6)) * var_sp_s_temp) + (assign41980_e55000 * var_sp_s_temp_dn6)), ((((((8.0 * var_sp_s_temp_dn7) - (12.0 * var_sp_s_xi0_dn7)) * var_sp_s_temp) + (assign41980_e54998 * var_sp_s_temp_dn7)) * var_sp_s_temp) + (assign41980_e55000 * var_sp_s_temp_dn7)), ((((((8.0 * var_sp_s_temp_dn8) - (12.0 * var_sp_s_xi0_dn8)) * var_sp_s_temp) + (assign41980_e54998 * var_sp_s_temp_dn8)) * var_sp_s_temp) + (assign41980_e55000 * var_sp_s_temp_dn8)), ((((((8.0 * var_sp_s_temp_dn9) - (12.0 * var_sp_s_xi0_dn9)) * var_sp_s_temp) + (assign41980_e54998 * var_sp_s_temp_dn9)) * var_sp_s_temp) + (assign41980_e55000 * var_sp_s_temp_dn9)),)
    } else {
        (var_sp_s_xi2, var_sp_s_xi2_dn4, var_sp_s_xi2_dn6, var_sp_s_xi2_dn7, var_sp_s_xi2_dn8, var_sp_s_xi2_dn9,)
    }
};
        var_sp_s_xi2 = assign41980_e55004;
        var_sp_s_xi2_dn4 = assign41980_e55004_d_n4;
        var_sp_s_xi2_dn6 = assign41980_e55004_d_n6;
        var_sp_s_xi2_dn7 = assign41980_e55004_d_n7;
        var_sp_s_xi2_dn8 = assign41980_e55004_d_n8;
        var_sp_s_xi2_dn9 = assign41980_e55004_d_n9;

        let (assign41990_e55013, assign41990_e55013_d_n4, assign41990_e55013_d_n6, assign41990_e55013_d_n7, assign41990_e55013_d_n8, assign41990_e55013_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign41990_e55011: f64 = (var_sp_s_yg - var_sp_s_y0);
        (assign41990_e55011, (var_sp_s_yg_dn4 - var_sp_s_y0_dn4), (var_sp_s_yg_dn6 - var_sp_s_y0_dn6), (var_sp_s_yg_dn7 - var_sp_s_y0_dn7), (var_sp_s_yg_dn8 - var_sp_s_y0_dn8), (var_sp_s_yg_dn9 - var_sp_s_y0_dn9),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn4, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8, var_sp_s_temp_dn9,)
    }
};
        var_sp_s_temp = assign41990_e55013;
        var_sp_s_temp_dn4 = assign41990_e55013_d_n4;
        var_sp_s_temp_dn6 = assign41990_e55013_d_n6;
        var_sp_s_temp_dn7 = assign41990_e55013_d_n7;
        var_sp_s_temp_dn8 = assign41990_e55013_d_n8;
        var_sp_s_temp_dn9 = assign41990_e55013_d_n9;

        let (assign42000_e55022, assign42000_e55022_d_n4, assign42000_e55022_d_n6, assign42000_e55022_d_n7, assign42000_e55022_d_n8, assign42000_e55022_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign42000_e55020: f64 = (var_delta_ns * var_sp_s_delta1);
        (assign42000_e55020, ((var_delta_ns_dn4 * var_sp_s_delta1) + (var_delta_ns * var_sp_s_delta1_dn4)), ((var_delta_ns_dn6 * var_sp_s_delta1) + (var_delta_ns * var_sp_s_delta1_dn6)), ((var_delta_ns_dn7 * var_sp_s_delta1) + (var_delta_ns * var_sp_s_delta1_dn7)), ((var_delta_ns_dn8 * var_sp_s_delta1) + (var_delta_ns * var_sp_s_delta1_dn8)), ((var_delta_ns_dn9 * var_sp_s_delta1) + (var_delta_ns * var_sp_s_delta1_dn9)),)
    } else {
        (var_sp_s_temp1, var_sp_s_temp1_dn4, var_sp_s_temp1_dn6, var_sp_s_temp1_dn7, var_sp_s_temp1_dn8, var_sp_s_temp1_dn9,)
    }
};
        var_sp_s_temp1 = assign42000_e55022;
        var_sp_s_temp1_dn4 = assign42000_e55022_d_n4;
        var_sp_s_temp1_dn6 = assign42000_e55022_d_n6;
        var_sp_s_temp1_dn7 = assign42000_e55022_d_n7;
        var_sp_s_temp1_dn8 = assign42000_e55022_d_n8;
        var_sp_s_temp1_dn9 = assign42000_e55022_d_n9;

        let (assign42010_e55045, assign42010_e55045_d_n4, assign42010_e55045_d_n6, assign42010_e55045_d_n7, assign42010_e55045_d_n8, assign42010_e55045_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign42010_e55029: f64 = (2.0 * var_sp_s_temp);
        let assign42010_e55033: f64 = (var_sp_s_delta0 - 1.0);
        let assign42010_e55035: f64 = (assign42010_e55033 - var_sp_s_temp1);
        let assign42010_e55039: f64 = (1.0 - var_sp_s_xi1);
        let assign42010_e55040: f64 = (var_delta_ns * assign42010_e55039);
        let assign42010_e55041: f64 = (assign42010_e55035 + assign42010_e55040);
        let assign42010_e55042: f64 = (var_gf2 * assign42010_e55041);
        let assign42010_e55043: f64 = (assign42010_e55029 + assign42010_e55042);
        (assign42010_e55043, ((2.0 * var_sp_s_temp_dn4) + ((var_gf2_dn4 * assign42010_e55041) + (var_gf2 * ((var_sp_s_delta0_dn4 - var_sp_s_temp1_dn4) + ((var_delta_ns_dn4 * assign42010_e55039) + (var_delta_ns * (-var_sp_s_xi1_dn4))))))), ((2.0 * var_sp_s_temp_dn6) + ((var_gf2_dn6 * assign42010_e55041) + (var_gf2 * ((var_sp_s_delta0_dn6 - var_sp_s_temp1_dn6) + ((var_delta_ns_dn6 * assign42010_e55039) + (var_delta_ns * (-var_sp_s_xi1_dn6))))))), ((2.0 * var_sp_s_temp_dn7) + ((var_gf2_dn7 * assign42010_e55041) + (var_gf2 * ((var_sp_s_delta0_dn7 - var_sp_s_temp1_dn7) + ((var_delta_ns_dn7 * assign42010_e55039) + (var_delta_ns * (-var_sp_s_xi1_dn7))))))), ((2.0 * var_sp_s_temp_dn8) + ((var_gf2_dn8 * assign42010_e55041) + (var_gf2 * ((var_sp_s_delta0_dn8 - var_sp_s_temp1_dn8) + ((var_delta_ns_dn8 * assign42010_e55039) + (var_delta_ns * (-var_sp_s_xi1_dn8))))))), ((2.0 * var_sp_s_temp_dn9) + ((var_gf2_dn9 * assign42010_e55041) + (var_gf2 * ((var_sp_s_delta0_dn9 - var_sp_s_temp1_dn9) + ((var_delta_ns_dn9 * assign42010_e55039) + (var_delta_ns * (-var_sp_s_xi1_dn9))))))),)
    } else {
        (var_sp_s_pc, var_sp_s_pc_dn4, var_sp_s_pc_dn6, var_sp_s_pc_dn7, var_sp_s_pc_dn8, var_sp_s_pc_dn9,)
    }
};
        var_sp_s_pc = assign42010_e55045;
        var_sp_s_pc_dn4 = assign42010_e55045_d_n4;
        var_sp_s_pc_dn6 = assign42010_e55045_d_n6;
        var_sp_s_pc_dn7 = assign42010_e55045_d_n7;
        var_sp_s_pc_dn8 = assign42010_e55045_d_n8;
        var_sp_s_pc_dn9 = assign42010_e55045_d_n9;

        let (assign42020_e55072, assign42020_e55072_d_n4, assign42020_e55072_d_n6, assign42020_e55072_d_n7, assign42020_e55072_d_n8, assign42020_e55072_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign42020_e55052: f64 = (var_sp_s_temp * var_sp_s_temp);
        let assign42020_e55056: f64 = (var_sp_s_delta0 - var_sp_s_y0);
        let assign42020_e55058: f64 = (assign42020_e55056 - 1.0);
        let assign42020_e55060: f64 = (assign42020_e55058 + var_sp_s_temp1);
        let assign42020_e55064: f64 = (var_sp_s_y0 - 1.0);
        let assign42020_e55066: f64 = (assign42020_e55064 - var_sp_s_xi0);
        let assign42020_e55067: f64 = (var_delta_ns * assign42020_e55066);
        let assign42020_e55068: f64 = (assign42020_e55060 + assign42020_e55067);
        let assign42020_e55069: f64 = (var_gf2 * assign42020_e55068);
        let assign42020_e55070: f64 = (assign42020_e55052 - assign42020_e55069);
        (assign42020_e55070, (((var_sp_s_temp_dn4 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn4)) - ((var_gf2_dn4 * assign42020_e55068) + (var_gf2 * (((var_sp_s_delta0_dn4 - var_sp_s_y0_dn4) + var_sp_s_temp1_dn4) + ((var_delta_ns_dn4 * assign42020_e55066) + (var_delta_ns * (var_sp_s_y0_dn4 - var_sp_s_xi0_dn4))))))), (((var_sp_s_temp_dn6 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn6)) - ((var_gf2_dn6 * assign42020_e55068) + (var_gf2 * (((var_sp_s_delta0_dn6 - var_sp_s_y0_dn6) + var_sp_s_temp1_dn6) + ((var_delta_ns_dn6 * assign42020_e55066) + (var_delta_ns * (var_sp_s_y0_dn6 - var_sp_s_xi0_dn6))))))), (((var_sp_s_temp_dn7 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn7)) - ((var_gf2_dn7 * assign42020_e55068) + (var_gf2 * (((var_sp_s_delta0_dn7 - var_sp_s_y0_dn7) + var_sp_s_temp1_dn7) + ((var_delta_ns_dn7 * assign42020_e55066) + (var_delta_ns * (var_sp_s_y0_dn7 - var_sp_s_xi0_dn7))))))), (((var_sp_s_temp_dn8 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn8)) - ((var_gf2_dn8 * assign42020_e55068) + (var_gf2 * (((var_sp_s_delta0_dn8 - var_sp_s_y0_dn8) + var_sp_s_temp1_dn8) + ((var_delta_ns_dn8 * assign42020_e55066) + (var_delta_ns * (var_sp_s_y0_dn8 - var_sp_s_xi0_dn8))))))), (((var_sp_s_temp_dn9 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn9)) - ((var_gf2_dn9 * assign42020_e55068) + (var_gf2 * (((var_sp_s_delta0_dn9 - var_sp_s_y0_dn9) + var_sp_s_temp1_dn9) + ((var_delta_ns_dn9 * assign42020_e55066) + (var_delta_ns * (var_sp_s_y0_dn9 - var_sp_s_xi0_dn9))))))),)
    } else {
        (var_sp_s_qc, var_sp_s_qc_dn4, var_sp_s_qc_dn6, var_sp_s_qc_dn7, var_sp_s_qc_dn8, var_sp_s_qc_dn9,)
    }
};
        var_sp_s_qc = assign42020_e55072;
        var_sp_s_qc_dn4 = assign42020_e55072_d_n4;
        var_sp_s_qc_dn6 = assign42020_e55072_d_n6;
        var_sp_s_qc_dn7 = assign42020_e55072_d_n7;
        var_sp_s_qc_dn8 = assign42020_e55072_d_n8;
        var_sp_s_qc_dn9 = assign42020_e55072_d_n9;

        let (assign42030_e55089, assign42030_e55089_d_n4, assign42030_e55089_d_n6, assign42030_e55089_d_n7, assign42030_e55089_d_n8, assign42030_e55089_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign42030_e55081: f64 = (var_sp_s_delta0 + var_sp_s_temp1);
        let assign42030_e55084: f64 = (var_delta_ns * var_sp_s_xi2);
        let assign42030_e55085: f64 = (assign42030_e55081 - assign42030_e55084);
        let assign42030_e55086: f64 = (var_gf2 * assign42030_e55085);
        let assign42030_e55087: f64 = (2.0 - assign42030_e55086);
        (assign42030_e55087, (-((var_gf2_dn4 * assign42030_e55085) + (var_gf2 * ((var_sp_s_delta0_dn4 + var_sp_s_temp1_dn4) - ((var_delta_ns_dn4 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn4)))))), (-((var_gf2_dn6 * assign42030_e55085) + (var_gf2 * ((var_sp_s_delta0_dn6 + var_sp_s_temp1_dn6) - ((var_delta_ns_dn6 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn6)))))), (-((var_gf2_dn7 * assign42030_e55085) + (var_gf2 * ((var_sp_s_delta0_dn7 + var_sp_s_temp1_dn7) - ((var_delta_ns_dn7 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn7)))))), (-((var_gf2_dn8 * assign42030_e55085) + (var_gf2 * ((var_sp_s_delta0_dn8 + var_sp_s_temp1_dn8) - ((var_delta_ns_dn8 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn8)))))), (-((var_gf2_dn9 * assign42030_e55085) + (var_gf2 * ((var_sp_s_delta0_dn9 + var_sp_s_temp1_dn9) - ((var_delta_ns_dn9 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn9)))))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn4, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8, var_sp_s_temp_dn9,)
    }
};
        var_sp_s_temp = assign42030_e55089;
        var_sp_s_temp_dn4 = assign42030_e55089_d_n4;
        var_sp_s_temp_dn6 = assign42030_e55089_d_n6;
        var_sp_s_temp_dn7 = assign42030_e55089_d_n7;
        var_sp_s_temp_dn8 = assign42030_e55089_d_n8;
        var_sp_s_temp_dn9 = assign42030_e55089_d_n9;

        let (assign42040_e55104, assign42040_e55104_d_n4, assign42040_e55104_d_n6, assign42040_e55104_d_n7, assign42040_e55104_d_n8, assign42040_e55104_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign42040_e55096: f64 = (var_sp_s_pc * var_sp_s_pc);
        let assign42040_e55100: f64 = (var_sp_s_qc * var_sp_s_temp);
        let assign42040_e55101: f64 = (2.0 * assign42040_e55100);
        let assign42040_e55102: f64 = (assign42040_e55096 - assign42040_e55101);
        (assign42040_e55102, (((var_sp_s_pc_dn4 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn4)) - (2.0 * ((var_sp_s_qc_dn4 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn4)))), (((var_sp_s_pc_dn6 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn6)) - (2.0 * ((var_sp_s_qc_dn6 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn6)))), (((var_sp_s_pc_dn7 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn7)) - (2.0 * ((var_sp_s_qc_dn7 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn7)))), (((var_sp_s_pc_dn8 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn8)) - (2.0 * ((var_sp_s_qc_dn8 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn8)))), (((var_sp_s_pc_dn9 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn9)) - (2.0 * ((var_sp_s_qc_dn9 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn9)))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn4, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8, var_sp_s_temp_dn9,)
    }
};
        var_sp_s_temp = assign42040_e55104;
        var_sp_s_temp_dn4 = assign42040_e55104_d_n4;
        var_sp_s_temp_dn6 = assign42040_e55104_d_n6;
        var_sp_s_temp_dn7 = assign42040_e55104_d_n7;
        var_sp_s_temp_dn8 = assign42040_e55104_d_n8;
        var_sp_s_temp_dn9 = assign42040_e55104_d_n9;

        let (assign42050_e55121, assign42050_e55121_d_n4, assign42050_e55121_d_n6, assign42050_e55121_d_n7, assign42050_e55121_d_n8, assign42050_e55121_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 != 0.0)) {
        let assign42050_e55110: f64 = (-var_sp_s_y0);
        let assign42050_e55115: f64 = (var_sp_s_temp).sqrt();
        let assign42050_e55116: f64 = (var_sp_s_pc + assign42050_e55115);
        let assign42050_e55117: f64 = (var_sp_s_qc / assign42050_e55116);
        let assign42050_e55118: f64 = (2.0 * assign42050_e55117);
        let assign42050_e55119: f64 = (assign42050_e55110 - assign42050_e55118);
        (assign42050_e55119, ((-var_sp_s_y0_dn4) - (2.0 * (((var_sp_s_qc_dn4 * assign42050_e55116) - (var_sp_s_qc * (var_sp_s_pc_dn4 + (var_sp_s_temp_dn4 / (2.0 * assign42050_e55115))))) / (assign42050_e55116 * assign42050_e55116)))), ((-var_sp_s_y0_dn6) - (2.0 * (((var_sp_s_qc_dn6 * assign42050_e55116) - (var_sp_s_qc * (var_sp_s_pc_dn6 + (var_sp_s_temp_dn6 / (2.0 * assign42050_e55115))))) / (assign42050_e55116 * assign42050_e55116)))), ((-var_sp_s_y0_dn7) - (2.0 * (((var_sp_s_qc_dn7 * assign42050_e55116) - (var_sp_s_qc * (var_sp_s_pc_dn7 + (var_sp_s_temp_dn7 / (2.0 * assign42050_e55115))))) / (assign42050_e55116 * assign42050_e55116)))), ((-var_sp_s_y0_dn8) - (2.0 * (((var_sp_s_qc_dn8 * assign42050_e55116) - (var_sp_s_qc * (var_sp_s_pc_dn8 + (var_sp_s_temp_dn8 / (2.0 * assign42050_e55115))))) / (assign42050_e55116 * assign42050_e55116)))), ((-var_sp_s_y0_dn9) - (2.0 * (((var_sp_s_qc_dn9 * assign42050_e55116) - (var_sp_s_qc * (var_sp_s_pc_dn9 + (var_sp_s_temp_dn9 / (2.0 * assign42050_e55115))))) / (assign42050_e55116 * assign42050_e55116)))),)
    } else {
        (var_x_s, var_x_s_dn4, var_x_s_dn6, var_x_s_dn7, var_x_s_dn8, var_x_s_dn9,)
    }
};
        var_x_s = assign42050_e55121;
        var_x_s_dn4 = assign42050_e55121_d_n4;
        var_x_s_dn6 = assign42050_e55121_d_n6;
        var_x_s_dn7 = assign42050_e55121_d_n7;
        var_x_s_dn8 = assign42050_e55121_d_n8;
        var_x_s_dn9 = assign42050_e55121_d_n9;

        let (assign42060_e55135, assign42060_e55135_d_n4, assign42060_e55135_d_n6, assign42060_e55135_d_n7, assign42060_e55135_d_n8, assign42060_e55135_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42060_e55131: f64 = (var_gf * 0.7324648775608221);
        let assign42060_e55132: f64 = (1.25 + assign42060_e55131);
        let assign42060_e55133: f64 = (1.0 / assign42060_e55132);
        (assign42060_e55133, (-((var_gf_dn4 * 0.7324648775608221) / (assign42060_e55132 * assign42060_e55132))), (-((var_gf_dn6 * 0.7324648775608221) / (assign42060_e55132 * assign42060_e55132))), (-((var_gf_dn7 * 0.7324648775608221) / (assign42060_e55132 * assign42060_e55132))), (-((var_gf_dn8 * 0.7324648775608221) / (assign42060_e55132 * assign42060_e55132))), (-((var_gf_dn9 * 0.7324648775608221) / (assign42060_e55132 * assign42060_e55132))),)
    } else {
        (var_sp_xg1, var_sp_xg1_dn4, var_sp_xg1_dn6, var_sp_xg1_dn7, var_sp_xg1_dn8, var_sp_xg1_dn9,)
    }
};
        var_sp_xg1 = assign42060_e55135;
        var_sp_xg1_dn4 = assign42060_e55135_d_n4;
        var_sp_xg1_dn6 = assign42060_e55135_d_n6;
        var_sp_xg1_dn7 = assign42060_e55135_d_n7;
        var_sp_xg1_dn8 = assign42060_e55135_d_n8;
        var_sp_xg1_dn9 = assign42060_e55135_d_n9;

        let (assign42070_e55151, assign42070_e55151_d_n4, assign42070_e55151_d_n6, assign42070_e55151_d_n7, assign42070_e55151_d_n8, assign42070_e55151_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42070_e55143: f64 = (var_xi * 1.25);
        let assign42070_e55145: f64 = (assign42070_e55143 * var_sp_xg1);
        let assign42070_e55147: f64 = (assign42070_e55145 - 1.0);
        let assign42070_e55149: f64 = (assign42070_e55147 * var_sp_xg1);
        (assign42070_e55149, (((((var_xi_dn4 * 1.25) * var_sp_xg1) + (assign42070_e55143 * var_sp_xg1_dn4)) * var_sp_xg1) + (assign42070_e55147 * var_sp_xg1_dn4)), (((((var_xi_dn6 * 1.25) * var_sp_xg1) + (assign42070_e55143 * var_sp_xg1_dn6)) * var_sp_xg1) + (assign42070_e55147 * var_sp_xg1_dn6)), (((((var_xi_dn7 * 1.25) * var_sp_xg1) + (assign42070_e55143 * var_sp_xg1_dn7)) * var_sp_xg1) + (assign42070_e55147 * var_sp_xg1_dn7)), (((((var_xi_dn8 * 1.25) * var_sp_xg1) + (assign42070_e55143 * var_sp_xg1_dn8)) * var_sp_xg1) + (assign42070_e55147 * var_sp_xg1_dn8)), (((((var_xi_dn9 * 1.25) * var_sp_xg1) + (assign42070_e55143 * var_sp_xg1_dn9)) * var_sp_xg1) + (assign42070_e55147 * var_sp_xg1_dn9)),)
    } else {
        (var_sp_s_a_fac, var_sp_s_a_fac_dn4, var_sp_s_a_fac_dn6, var_sp_s_a_fac_dn7, var_sp_s_a_fac_dn8, var_sp_s_a_fac_dn9,)
    }
};
        var_sp_s_a_fac = assign42070_e55151;
        var_sp_s_a_fac_dn4 = assign42070_e55151_d_n4;
        var_sp_s_a_fac_dn6 = assign42070_e55151_d_n6;
        var_sp_s_a_fac_dn7 = assign42070_e55151_d_n7;
        var_sp_s_a_fac_dn8 = assign42070_e55151_d_n8;
        var_sp_s_a_fac_dn9 = assign42070_e55151_d_n9;

        let (assign42080_e55167, assign42080_e55167_d_n4, assign42080_e55167_d_n6, assign42080_e55167_d_n7, assign42080_e55167_d_n8, assign42080_e55167_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42080_e55159: f64 = (var_xg * var_inv_xi);
        let assign42080_e55163: f64 = (var_sp_s_a_fac * var_xg);
        let assign42080_e55164: f64 = (1.0 + assign42080_e55163);
        let assign42080_e55165: f64 = (assign42080_e55159 * assign42080_e55164);
        (assign42080_e55165, ((((var_xg_dn4 * var_inv_xi) + (var_xg * var_inv_xi_dn4)) * assign42080_e55164) + (assign42080_e55159 * ((var_sp_s_a_fac_dn4 * var_xg) + (var_sp_s_a_fac * var_xg_dn4)))), ((((var_xg_dn6 * var_inv_xi) + (var_xg * var_inv_xi_dn6)) * assign42080_e55164) + (assign42080_e55159 * ((var_sp_s_a_fac_dn6 * var_xg) + (var_sp_s_a_fac * var_xg_dn6)))), ((((var_xg_dn7 * var_inv_xi) + (var_xg * var_inv_xi_dn7)) * assign42080_e55164) + (assign42080_e55159 * ((var_sp_s_a_fac_dn7 * var_xg) + (var_sp_s_a_fac * var_xg_dn7)))), ((((var_xg_dn8 * var_inv_xi) + (var_xg * var_inv_xi_dn8)) * assign42080_e55164) + (assign42080_e55159 * ((var_sp_s_a_fac_dn8 * var_xg) + (var_sp_s_a_fac * var_xg_dn8)))), ((((var_xg_dn9 * var_inv_xi) + (var_xg * var_inv_xi_dn9)) * assign42080_e55164) + (assign42080_e55159 * ((var_sp_s_a_fac_dn9 * var_xg) + (var_sp_s_a_fac * var_xg_dn9)))),)
    } else {
        (var_sp_s_xbar, var_sp_s_xbar_dn4, var_sp_s_xbar_dn6, var_sp_s_xbar_dn7, var_sp_s_xbar_dn8, var_sp_s_xbar_dn9,)
    }
};
        var_sp_s_xbar = assign42080_e55167;
        var_sp_s_xbar_dn4 = assign42080_e55167_d_n4;
        var_sp_s_xbar_dn6 = assign42080_e55167_d_n6;
        var_sp_s_xbar_dn7 = assign42080_e55167_d_n7;
        var_sp_s_xbar_dn8 = assign42080_e55167_d_n8;
        var_sp_s_xbar_dn9 = assign42080_e55167_d_n9;

        let assign42090_e55169: f64 = (-var_sp_s_xbar);
        let assign42090_e55171: f64 = (-230.25850929940458);
        let assign42090_e55172: f64 = if assign42090_e55169 > assign42090_e55171 { 1.0 } else { 0.0 };
        var_guard1202 = assign42090_e55172;

        let (assign42100_e55184, assign42100_e55184_d_n4, assign42100_e55184_d_n6, assign42100_e55184_d_n7, assign42100_e55184_d_n8, assign42100_e55184_d_n9,) = {
    if (((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) && (var_guard1202 != 0.0)) {
        let assign42100_e55181: f64 = (-var_sp_s_xbar);
        let assign42100_e55182: f64 = (assign42100_e55181).exp();
        (assign42100_e55182, (assign42100_e55182 * (-var_sp_s_xbar_dn4)), (assign42100_e55182 * (-var_sp_s_xbar_dn6)), (assign42100_e55182 * (-var_sp_s_xbar_dn7)), (assign42100_e55182 * (-var_sp_s_xbar_dn8)), (assign42100_e55182 * (-var_sp_s_xbar_dn9)),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn4, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8, var_sp_s_temp_dn9,)
    }
};
        var_sp_s_temp = assign42100_e55184;
        var_sp_s_temp_dn4 = assign42100_e55184_d_n4;
        var_sp_s_temp_dn6 = assign42100_e55184_d_n6;
        var_sp_s_temp_dn7 = assign42100_e55184_d_n7;
        var_sp_s_temp_dn8 = assign42100_e55184_d_n8;
        var_sp_s_temp_dn9 = assign42100_e55184_d_n9;

        let (assign42110_e55223, assign42110_e55223_d_n4, assign42110_e55223_d_n6, assign42110_e55223_d_n7, assign42110_e55223_d_n8, assign42110_e55223_d_n9,) = {
    if (((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) && (var_guard1202 == 0.0)) {
        let assign42110_e55196: f64 = (-230.25850929940458);
        let assign42110_e55198: f64 = (-var_sp_s_xbar);
        let assign42110_e55199: f64 = (assign42110_e55196 - assign42110_e55198);
        let assign42110_e55203: f64 = (-230.25850929940458);
        let assign42110_e55205: f64 = (-var_sp_s_xbar);
        let assign42110_e55206: f64 = (assign42110_e55203 - assign42110_e55205);
        let assign42110_e55209: f64 = (-230.25850929940458);
        let assign42110_e55211: f64 = (-var_sp_s_xbar);
        let assign42110_e55212: f64 = (assign42110_e55209 - assign42110_e55211);
        let assign42110_e55214: f64 = (assign42110_e55212 * 0.3333333333333333);
        let assign42110_e55215: f64 = (1.0 + assign42110_e55214);
        let assign42110_e55216: f64 = (assign42110_e55206 * assign42110_e55215);
        let assign42110_e55217: f64 = (0.5 * assign42110_e55216);
        let assign42110_e55218: f64 = (1.0 + assign42110_e55217);
        let assign42110_e55219: f64 = (assign42110_e55199 * assign42110_e55218);
        let assign42110_e55220: f64 = (1.0 + assign42110_e55219);
        let assign42110_e55221: f64 = (1e-100 / assign42110_e55220);
        (assign42110_e55221, (-((1e-100 * (((-(-var_sp_s_xbar_dn4)) * assign42110_e55218) + (assign42110_e55199 * (0.5 * (((-(-var_sp_s_xbar_dn4)) * assign42110_e55215) + (assign42110_e55206 * ((-(-var_sp_s_xbar_dn4)) * 0.3333333333333333))))))) / (assign42110_e55220 * assign42110_e55220))), (-((1e-100 * (((-(-var_sp_s_xbar_dn6)) * assign42110_e55218) + (assign42110_e55199 * (0.5 * (((-(-var_sp_s_xbar_dn6)) * assign42110_e55215) + (assign42110_e55206 * ((-(-var_sp_s_xbar_dn6)) * 0.3333333333333333))))))) / (assign42110_e55220 * assign42110_e55220))), (-((1e-100 * (((-(-var_sp_s_xbar_dn7)) * assign42110_e55218) + (assign42110_e55199 * (0.5 * (((-(-var_sp_s_xbar_dn7)) * assign42110_e55215) + (assign42110_e55206 * ((-(-var_sp_s_xbar_dn7)) * 0.3333333333333333))))))) / (assign42110_e55220 * assign42110_e55220))), (-((1e-100 * (((-(-var_sp_s_xbar_dn8)) * assign42110_e55218) + (assign42110_e55199 * (0.5 * (((-(-var_sp_s_xbar_dn8)) * assign42110_e55215) + (assign42110_e55206 * ((-(-var_sp_s_xbar_dn8)) * 0.3333333333333333))))))) / (assign42110_e55220 * assign42110_e55220))), (-((1e-100 * (((-(-var_sp_s_xbar_dn9)) * assign42110_e55218) + (assign42110_e55199 * (0.5 * (((-(-var_sp_s_xbar_dn9)) * assign42110_e55215) + (assign42110_e55206 * ((-(-var_sp_s_xbar_dn9)) * 0.3333333333333333))))))) / (assign42110_e55220 * assign42110_e55220))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn4, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8, var_sp_s_temp_dn9,)
    }
};
        var_sp_s_temp = assign42110_e55223;
        var_sp_s_temp_dn4 = assign42110_e55223_d_n4;
        var_sp_s_temp_dn6 = assign42110_e55223_d_n6;
        var_sp_s_temp_dn7 = assign42110_e55223_d_n7;
        var_sp_s_temp_dn8 = assign42110_e55223_d_n8;
        var_sp_s_temp_dn9 = assign42110_e55223_d_n9;

        let (assign42120_e55233, assign42120_e55233_d_n4, assign42120_e55233_d_n6, assign42120_e55233_d_n7, assign42120_e55233_d_n8, assign42120_e55233_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42120_e55231: f64 = (1.0 - var_sp_s_temp);
        (assign42120_e55231, (-var_sp_s_temp_dn4), (-var_sp_s_temp_dn6), (-var_sp_s_temp_dn7), (-var_sp_s_temp_dn8), (-var_sp_s_temp_dn9),)
    } else {
        (var_sp_s_w, var_sp_s_w_dn4, var_sp_s_w_dn6, var_sp_s_w_dn7, var_sp_s_w_dn8, var_sp_s_w_dn9,)
    }
};
        var_sp_s_w = assign42120_e55233;
        var_sp_s_w_dn4 = assign42120_e55233_d_n4;
        var_sp_s_w_dn6 = assign42120_e55233_d_n6;
        var_sp_s_w_dn7 = assign42120_e55233_d_n7;
        var_sp_s_w_dn8 = assign42120_e55233_d_n8;
        var_sp_s_w_dn9 = assign42120_e55233_d_n9;

        let (assign42130_e55256, assign42130_e55256_d_n4, assign42130_e55256_d_n6, assign42130_e55256_d_n7, assign42130_e55256_d_n8, assign42130_e55256_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42130_e55242: f64 = (var_gf2 * 0.5);
        let assign42130_e55243: f64 = (var_xg + assign42130_e55242);
        let assign42130_e55248: f64 = (var_gf2 * 0.25);
        let assign42130_e55249: f64 = (var_xg + assign42130_e55248);
        let assign42130_e55251: f64 = (assign42130_e55249 - var_sp_s_w);
        let assign42130_e55252: f64 = (assign42130_e55251).sqrt();
        let assign42130_e55253: f64 = (var_gf * assign42130_e55252);
        let assign42130_e55254: f64 = (assign42130_e55243 - assign42130_e55253);
        (assign42130_e55254, ((var_xg_dn4 + (var_gf2_dn4 * 0.5)) - ((var_gf_dn4 * assign42130_e55252) + (var_gf * (((var_xg_dn4 + (var_gf2_dn4 * 0.25)) - var_sp_s_w_dn4) / (2.0 * assign42130_e55252))))), ((var_xg_dn6 + (var_gf2_dn6 * 0.5)) - ((var_gf_dn6 * assign42130_e55252) + (var_gf * (((var_xg_dn6 + (var_gf2_dn6 * 0.25)) - var_sp_s_w_dn6) / (2.0 * assign42130_e55252))))), ((var_xg_dn7 + (var_gf2_dn7 * 0.5)) - ((var_gf_dn7 * assign42130_e55252) + (var_gf * (((var_xg_dn7 + (var_gf2_dn7 * 0.25)) - var_sp_s_w_dn7) / (2.0 * assign42130_e55252))))), ((var_xg_dn8 + (var_gf2_dn8 * 0.5)) - ((var_gf_dn8 * assign42130_e55252) + (var_gf * (((var_xg_dn8 + (var_gf2_dn8 * 0.25)) - var_sp_s_w_dn8) / (2.0 * assign42130_e55252))))), ((var_xg_dn9 + (var_gf2_dn9 * 0.5)) - ((var_gf_dn9 * assign42130_e55252) + (var_gf * (((var_xg_dn9 + (var_gf2_dn9 * 0.25)) - var_sp_s_w_dn9) / (2.0 * assign42130_e55252))))),)
    } else {
        (var_sp_s_x1, var_sp_s_x1_dn4, var_sp_s_x1_dn6, var_sp_s_x1_dn7, var_sp_s_x1_dn8, var_sp_s_x1_dn9,)
    }
};
        var_sp_s_x1 = assign42130_e55256;
        var_sp_s_x1_dn4 = assign42130_e55256_d_n4;
        var_sp_s_x1_dn6 = assign42130_e55256_d_n6;
        var_sp_s_x1_dn7 = assign42130_e55256_d_n7;
        var_sp_s_x1_dn8 = assign42130_e55256_d_n8;
        var_sp_s_x1_dn9 = assign42130_e55256_d_n9;

        let (assign42140_e55266, assign42140_e55266_d_n4, assign42140_e55266_d_n6, assign42140_e55266_d_n7, assign42140_e55266_d_n8, assign42140_e55266_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42140_e55264: f64 = (var_xn_s + 3.0);
        (assign42140_e55264, var_xn_s_dn4, var_xn_s_dn6, var_xn_s_dn7, var_xn_s_dn8, var_xn_s_dn9,)
    } else {
        (var_sp_s_bx, var_sp_s_bx_dn4, var_sp_s_bx_dn6, var_sp_s_bx_dn7, var_sp_s_bx_dn8, var_sp_s_bx_dn9,)
    }
};
        var_sp_s_bx = assign42140_e55266;
        var_sp_s_bx_dn4 = assign42140_e55266_d_n4;
        var_sp_s_bx_dn6 = assign42140_e55266_d_n6;
        var_sp_s_bx_dn7 = assign42140_e55266_d_n7;
        var_sp_s_bx_dn8 = assign42140_e55266_d_n8;
        var_sp_s_bx_dn9 = assign42140_e55266_d_n9;

        let (assign42150_e55300, assign42150_e55300_d_n4, assign42150_e55300_d_n6, assign42150_e55300_d_n7, assign42150_e55300_d_n8, assign42150_e55300_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42150_e55275: f64 = (var_sp_s_x1 + var_sp_s_bx);
        let assign42150_e55278: f64 = (var_sp_s_x1 - var_sp_s_bx);
        let assign42150_e55281: f64 = (var_sp_s_x1 - var_sp_s_bx);
        let assign42150_e55282: f64 = (assign42150_e55278 * assign42150_e55281);
        let assign42150_e55284: f64 = (assign42150_e55282 + 5.0);
        let assign42150_e55285: f64 = (assign42150_e55284).sqrt();
        let assign42150_e55286: f64 = (assign42150_e55275 - assign42150_e55285);
        let assign42150_e55287: f64 = (0.5 * assign42150_e55286);
        let assign42150_e55292: f64 = (var_sp_s_bx * var_sp_s_bx);
        let assign42150_e55294: f64 = (assign42150_e55292 + 5.0);
        let assign42150_e55295: f64 = (assign42150_e55294).sqrt();
        let assign42150_e55296: f64 = (var_sp_s_bx - assign42150_e55295);
        let assign42150_e55297: f64 = (0.5 * assign42150_e55296);
        let assign42150_e55298: f64 = (assign42150_e55287 - assign42150_e55297);
        (assign42150_e55298, ((0.5 * ((var_sp_s_x1_dn4 + var_sp_s_bx_dn4) - ((((var_sp_s_x1_dn4 - var_sp_s_bx_dn4) * assign42150_e55281) + (assign42150_e55278 * (var_sp_s_x1_dn4 - var_sp_s_bx_dn4))) / (2.0 * assign42150_e55285)))) - (0.5 * (var_sp_s_bx_dn4 - (((var_sp_s_bx_dn4 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn4)) / (2.0 * assign42150_e55295))))), ((0.5 * ((var_sp_s_x1_dn6 + var_sp_s_bx_dn6) - ((((var_sp_s_x1_dn6 - var_sp_s_bx_dn6) * assign42150_e55281) + (assign42150_e55278 * (var_sp_s_x1_dn6 - var_sp_s_bx_dn6))) / (2.0 * assign42150_e55285)))) - (0.5 * (var_sp_s_bx_dn6 - (((var_sp_s_bx_dn6 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn6)) / (2.0 * assign42150_e55295))))), ((0.5 * ((var_sp_s_x1_dn7 + var_sp_s_bx_dn7) - ((((var_sp_s_x1_dn7 - var_sp_s_bx_dn7) * assign42150_e55281) + (assign42150_e55278 * (var_sp_s_x1_dn7 - var_sp_s_bx_dn7))) / (2.0 * assign42150_e55285)))) - (0.5 * (var_sp_s_bx_dn7 - (((var_sp_s_bx_dn7 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn7)) / (2.0 * assign42150_e55295))))), ((0.5 * ((var_sp_s_x1_dn8 + var_sp_s_bx_dn8) - ((((var_sp_s_x1_dn8 - var_sp_s_bx_dn8) * assign42150_e55281) + (assign42150_e55278 * (var_sp_s_x1_dn8 - var_sp_s_bx_dn8))) / (2.0 * assign42150_e55285)))) - (0.5 * (var_sp_s_bx_dn8 - (((var_sp_s_bx_dn8 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn8)) / (2.0 * assign42150_e55295))))), ((0.5 * ((var_sp_s_x1_dn9 + var_sp_s_bx_dn9) - ((((var_sp_s_x1_dn9 - var_sp_s_bx_dn9) * assign42150_e55281) + (assign42150_e55278 * (var_sp_s_x1_dn9 - var_sp_s_bx_dn9))) / (2.0 * assign42150_e55285)))) - (0.5 * (var_sp_s_bx_dn9 - (((var_sp_s_bx_dn9 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn9)) / (2.0 * assign42150_e55295))))),)
    } else {
        (var_sp_s_eta, var_sp_s_eta_dn4, var_sp_s_eta_dn6, var_sp_s_eta_dn7, var_sp_s_eta_dn8, var_sp_s_eta_dn9,)
    }
};
        var_sp_s_eta = assign42150_e55300;
        var_sp_s_eta_dn4 = assign42150_e55300_d_n4;
        var_sp_s_eta_dn6 = assign42150_e55300_d_n6;
        var_sp_s_eta_dn7 = assign42150_e55300_d_n7;
        var_sp_s_eta_dn8 = assign42150_e55300_d_n8;
        var_sp_s_eta_dn9 = assign42150_e55300_d_n9;

        let (assign42160_e55310, assign42160_e55310_d_n4, assign42160_e55310_d_n6, assign42160_e55310_d_n7, assign42160_e55310_d_n8, assign42160_e55310_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42160_e55308: f64 = (var_xg - var_sp_s_eta);
        (assign42160_e55308, (var_xg_dn4 - var_sp_s_eta_dn4), (var_xg_dn6 - var_sp_s_eta_dn6), (var_xg_dn7 - var_sp_s_eta_dn7), (var_xg_dn8 - var_sp_s_eta_dn8), (var_xg_dn9 - var_sp_s_eta_dn9),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn4, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8, var_sp_s_temp_dn9,)
    }
};
        var_sp_s_temp = assign42160_e55310;
        var_sp_s_temp_dn4 = assign42160_e55310_d_n4;
        var_sp_s_temp_dn6 = assign42160_e55310_d_n6;
        var_sp_s_temp_dn7 = assign42160_e55310_d_n7;
        var_sp_s_temp_dn8 = assign42160_e55310_d_n8;
        var_sp_s_temp_dn9 = assign42160_e55310_d_n9;

        let (assign42170_e55320, assign42170_e55320_d_n4, assign42170_e55320_d_n6, assign42170_e55320_d_n7, assign42170_e55320_d_n8, assign42170_e55320_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42170_e55317: f64 = (-var_sp_s_eta);
        let assign42170_e55318: f64 = (assign42170_e55317).exp();
        (assign42170_e55318, (assign42170_e55318 * (-var_sp_s_eta_dn4)), (assign42170_e55318 * (-var_sp_s_eta_dn6)), (assign42170_e55318 * (-var_sp_s_eta_dn7)), (assign42170_e55318 * (-var_sp_s_eta_dn8)), (assign42170_e55318 * (-var_sp_s_eta_dn9)),)
    } else {
        (var_sp_s_temp1, var_sp_s_temp1_dn4, var_sp_s_temp1_dn6, var_sp_s_temp1_dn7, var_sp_s_temp1_dn8, var_sp_s_temp1_dn9,)
    }
};
        var_sp_s_temp1 = assign42170_e55320;
        var_sp_s_temp1_dn4 = assign42170_e55320_d_n4;
        var_sp_s_temp1_dn6 = assign42170_e55320_d_n6;
        var_sp_s_temp1_dn7 = assign42170_e55320_d_n7;
        var_sp_s_temp1_dn8 = assign42170_e55320_d_n8;
        var_sp_s_temp1_dn9 = assign42170_e55320_d_n9;

        *var_guard1201_slot = var_guard1201;
        *var_guard1202_slot = var_guard1202;
        *var_sp_s_a_fac_slot = var_sp_s_a_fac;
        *var_sp_s_a_fac_dn4_slot = var_sp_s_a_fac_dn4;
        *var_sp_s_a_fac_dn6_slot = var_sp_s_a_fac_dn6;
        *var_sp_s_a_fac_dn7_slot = var_sp_s_a_fac_dn7;
        *var_sp_s_a_fac_dn8_slot = var_sp_s_a_fac_dn8;
        *var_sp_s_a_fac_dn9_slot = var_sp_s_a_fac_dn9;
        *var_sp_s_bx_slot = var_sp_s_bx;
        *var_sp_s_bx_dn4_slot = var_sp_s_bx_dn4;
        *var_sp_s_bx_dn6_slot = var_sp_s_bx_dn6;
        *var_sp_s_bx_dn7_slot = var_sp_s_bx_dn7;
        *var_sp_s_bx_dn8_slot = var_sp_s_bx_dn8;
        *var_sp_s_bx_dn9_slot = var_sp_s_bx_dn9;
        *var_sp_s_delta0_slot = var_sp_s_delta0;
        *var_sp_s_delta0_dn4_slot = var_sp_s_delta0_dn4;
        *var_sp_s_delta0_dn6_slot = var_sp_s_delta0_dn6;
        *var_sp_s_delta0_dn7_slot = var_sp_s_delta0_dn7;
        *var_sp_s_delta0_dn8_slot = var_sp_s_delta0_dn8;
        *var_sp_s_delta0_dn9_slot = var_sp_s_delta0_dn9;
        *var_sp_s_delta1_slot = var_sp_s_delta1;
        *var_sp_s_delta1_dn4_slot = var_sp_s_delta1_dn4;
        *var_sp_s_delta1_dn6_slot = var_sp_s_delta1_dn6;
        *var_sp_s_delta1_dn7_slot = var_sp_s_delta1_dn7;
        *var_sp_s_delta1_dn8_slot = var_sp_s_delta1_dn8;
        *var_sp_s_delta1_dn9_slot = var_sp_s_delta1_dn9;
        *var_sp_s_eta_slot = var_sp_s_eta;
        *var_sp_s_eta_dn4_slot = var_sp_s_eta_dn4;
        *var_sp_s_eta_dn6_slot = var_sp_s_eta_dn6;
        *var_sp_s_eta_dn7_slot = var_sp_s_eta_dn7;
        *var_sp_s_eta_dn8_slot = var_sp_s_eta_dn8;
        *var_sp_s_eta_dn9_slot = var_sp_s_eta_dn9;
        *var_sp_s_pc_slot = var_sp_s_pc;
        *var_sp_s_pc_dn4_slot = var_sp_s_pc_dn4;
        *var_sp_s_pc_dn6_slot = var_sp_s_pc_dn6;
        *var_sp_s_pc_dn7_slot = var_sp_s_pc_dn7;
        *var_sp_s_pc_dn8_slot = var_sp_s_pc_dn8;
        *var_sp_s_pc_dn9_slot = var_sp_s_pc_dn9;
        *var_sp_s_qc_slot = var_sp_s_qc;
        *var_sp_s_qc_dn4_slot = var_sp_s_qc_dn4;
        *var_sp_s_qc_dn6_slot = var_sp_s_qc_dn6;
        *var_sp_s_qc_dn7_slot = var_sp_s_qc_dn7;
        *var_sp_s_qc_dn8_slot = var_sp_s_qc_dn8;
        *var_sp_s_qc_dn9_slot = var_sp_s_qc_dn9;
        *var_sp_s_temp_slot = var_sp_s_temp;
        *var_sp_s_temp1_slot = var_sp_s_temp1;
        *var_sp_s_temp1_dn4_slot = var_sp_s_temp1_dn4;
        *var_sp_s_temp1_dn6_slot = var_sp_s_temp1_dn6;
        *var_sp_s_temp1_dn7_slot = var_sp_s_temp1_dn7;
        *var_sp_s_temp1_dn8_slot = var_sp_s_temp1_dn8;
        *var_sp_s_temp1_dn9_slot = var_sp_s_temp1_dn9;
        *var_sp_s_temp_dn4_slot = var_sp_s_temp_dn4;
        *var_sp_s_temp_dn6_slot = var_sp_s_temp_dn6;
        *var_sp_s_temp_dn7_slot = var_sp_s_temp_dn7;
        *var_sp_s_temp_dn8_slot = var_sp_s_temp_dn8;
        *var_sp_s_temp_dn9_slot = var_sp_s_temp_dn9;
        *var_sp_s_w_slot = var_sp_s_w;
        *var_sp_s_w_dn4_slot = var_sp_s_w_dn4;
        *var_sp_s_w_dn6_slot = var_sp_s_w_dn6;
        *var_sp_s_w_dn7_slot = var_sp_s_w_dn7;
        *var_sp_s_w_dn8_slot = var_sp_s_w_dn8;
        *var_sp_s_w_dn9_slot = var_sp_s_w_dn9;
        *var_sp_s_x1_slot = var_sp_s_x1;
        *var_sp_s_x1_dn4_slot = var_sp_s_x1_dn4;
        *var_sp_s_x1_dn6_slot = var_sp_s_x1_dn6;
        *var_sp_s_x1_dn7_slot = var_sp_s_x1_dn7;
        *var_sp_s_x1_dn8_slot = var_sp_s_x1_dn8;
        *var_sp_s_x1_dn9_slot = var_sp_s_x1_dn9;
        *var_sp_s_xbar_slot = var_sp_s_xbar;
        *var_sp_s_xbar_dn4_slot = var_sp_s_xbar_dn4;
        *var_sp_s_xbar_dn6_slot = var_sp_s_xbar_dn6;
        *var_sp_s_xbar_dn7_slot = var_sp_s_xbar_dn7;
        *var_sp_s_xbar_dn8_slot = var_sp_s_xbar_dn8;
        *var_sp_s_xbar_dn9_slot = var_sp_s_xbar_dn9;
        *var_sp_s_xi0_slot = var_sp_s_xi0;
        *var_sp_s_xi0_dn4_slot = var_sp_s_xi0_dn4;
        *var_sp_s_xi0_dn6_slot = var_sp_s_xi0_dn6;
        *var_sp_s_xi0_dn7_slot = var_sp_s_xi0_dn7;
        *var_sp_s_xi0_dn8_slot = var_sp_s_xi0_dn8;
        *var_sp_s_xi0_dn9_slot = var_sp_s_xi0_dn9;
        *var_sp_s_xi1_slot = var_sp_s_xi1;
        *var_sp_s_xi1_dn4_slot = var_sp_s_xi1_dn4;
        *var_sp_s_xi1_dn6_slot = var_sp_s_xi1_dn6;
        *var_sp_s_xi1_dn7_slot = var_sp_s_xi1_dn7;
        *var_sp_s_xi1_dn8_slot = var_sp_s_xi1_dn8;
        *var_sp_s_xi1_dn9_slot = var_sp_s_xi1_dn9;
        *var_sp_s_xi2_slot = var_sp_s_xi2;
        *var_sp_s_xi2_dn4_slot = var_sp_s_xi2_dn4;
        *var_sp_s_xi2_dn6_slot = var_sp_s_xi2_dn6;
        *var_sp_s_xi2_dn7_slot = var_sp_s_xi2_dn7;
        *var_sp_s_xi2_dn8_slot = var_sp_s_xi2_dn8;
        *var_sp_s_xi2_dn9_slot = var_sp_s_xi2_dn9;
        *var_sp_s_y0_slot = var_sp_s_y0;
        *var_sp_s_y0_dn4_slot = var_sp_s_y0_dn4;
        *var_sp_s_y0_dn6_slot = var_sp_s_y0_dn6;
        *var_sp_s_y0_dn7_slot = var_sp_s_y0_dn7;
        *var_sp_s_y0_dn8_slot = var_sp_s_y0_dn8;
        *var_sp_s_y0_dn9_slot = var_sp_s_y0_dn9;
        *var_sp_xg1_slot = var_sp_xg1;
        *var_sp_xg1_dn4_slot = var_sp_xg1_dn4;
        *var_sp_xg1_dn6_slot = var_sp_xg1_dn6;
        *var_sp_xg1_dn7_slot = var_sp_xg1_dn7;
        *var_sp_xg1_dn8_slot = var_sp_xg1_dn8;
        *var_sp_xg1_dn9_slot = var_sp_xg1_dn9;
        *var_x_s_slot = var_x_s;
        *var_x_s_dn4_slot = var_x_s_dn4;
        *var_x_s_dn6_slot = var_x_s_dn6;
        *var_x_s_dn7_slot = var_x_s_dn7;
        *var_x_s_dn8_slot = var_x_s_dn8;
        *var_x_s_dn9_slot = var_x_s_dn9;
    }

    pub(super) fn stamp_transient_block_91(
        var_delta_ns: f64,
        var_delta_ns_dn4: f64,
        var_delta_ns_dn6: f64,
        var_delta_ns_dn7: f64,
        var_delta_ns_dn8: f64,
        var_delta_ns_dn9: f64,
        var_gf2: f64,
        var_gf2_dn4: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf2_dn9: f64,
        var_guard1199: f64,
        var_guard1200: f64,
        var_sp_s_eta: f64,
        var_sp_s_eta_dn4: f64,
        var_sp_s_eta_dn6: f64,
        var_sp_s_eta_dn7: f64,
        var_sp_s_eta_dn8: f64,
        var_sp_s_eta_dn9: f64,
        var_sp_s_temp1: f64,
        var_sp_s_temp1_dn4: f64,
        var_sp_s_temp1_dn6: f64,
        var_sp_s_temp1_dn7: f64,
        var_sp_s_temp1_dn8: f64,
        var_sp_s_temp1_dn9: f64,
        var_xg: f64,
        var_xg_dn4: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xg_dn9: f64,
        var_xn_s: f64,
        var_xn_s_dn4: f64,
        var_xn_s_dn6: f64,
        var_xn_s_dn7: f64,
        var_xn_s_dn8: f64,
        var_xn_s_dn9: f64,
        var_guard1203_slot: &mut f64,
        var_guard1204_slot: &mut f64,
        var_mutau_slot: &mut f64,
        var_mutau_dn4_slot: &mut f64,
        var_mutau_dn6_slot: &mut f64,
        var_mutau_dn7_slot: &mut f64,
        var_mutau_dn8_slot: &mut f64,
        var_mutau_dn9_slot: &mut f64,
        var_nu_slot: &mut f64,
        var_nu_dn4_slot: &mut f64,
        var_nu_dn6_slot: &mut f64,
        var_nu_dn7_slot: &mut f64,
        var_nu_dn8_slot: &mut f64,
        var_nu_dn9_slot: &mut f64,
        var_sp_s_a_slot: &mut f64,
        var_sp_s_a_dn4_slot: &mut f64,
        var_sp_s_a_dn6_slot: &mut f64,
        var_sp_s_a_dn7_slot: &mut f64,
        var_sp_s_a_dn8_slot: &mut f64,
        var_sp_s_a_dn9_slot: &mut f64,
        var_sp_s_b_slot: &mut f64,
        var_sp_s_b_dn4_slot: &mut f64,
        var_sp_s_b_dn6_slot: &mut f64,
        var_sp_s_b_dn7_slot: &mut f64,
        var_sp_s_b_dn8_slot: &mut f64,
        var_sp_s_b_dn9_slot: &mut f64,
        var_sp_s_c_slot: &mut f64,
        var_sp_s_c_dn4_slot: &mut f64,
        var_sp_s_c_dn6_slot: &mut f64,
        var_sp_s_c_dn7_slot: &mut f64,
        var_sp_s_c_dn8_slot: &mut f64,
        var_sp_s_c_dn9_slot: &mut f64,
        var_sp_s_delta0_slot: &mut f64,
        var_sp_s_delta0_dn4_slot: &mut f64,
        var_sp_s_delta0_dn6_slot: &mut f64,
        var_sp_s_delta0_dn7_slot: &mut f64,
        var_sp_s_delta0_dn8_slot: &mut f64,
        var_sp_s_delta0_dn9_slot: &mut f64,
        var_sp_s_delta1_slot: &mut f64,
        var_sp_s_delta1_dn4_slot: &mut f64,
        var_sp_s_delta1_dn6_slot: &mut f64,
        var_sp_s_delta1_dn7_slot: &mut f64,
        var_sp_s_delta1_dn8_slot: &mut f64,
        var_sp_s_delta1_dn9_slot: &mut f64,
        var_sp_s_pc_slot: &mut f64,
        var_sp_s_pc_dn4_slot: &mut f64,
        var_sp_s_pc_dn6_slot: &mut f64,
        var_sp_s_pc_dn7_slot: &mut f64,
        var_sp_s_pc_dn8_slot: &mut f64,
        var_sp_s_pc_dn9_slot: &mut f64,
        var_sp_s_qc_slot: &mut f64,
        var_sp_s_qc_dn4_slot: &mut f64,
        var_sp_s_qc_dn6_slot: &mut f64,
        var_sp_s_qc_dn7_slot: &mut f64,
        var_sp_s_qc_dn8_slot: &mut f64,
        var_sp_s_qc_dn9_slot: &mut f64,
        var_sp_s_tau_slot: &mut f64,
        var_sp_s_tau_dn4_slot: &mut f64,
        var_sp_s_tau_dn6_slot: &mut f64,
        var_sp_s_tau_dn7_slot: &mut f64,
        var_sp_s_tau_dn8_slot: &mut f64,
        var_sp_s_tau_dn9_slot: &mut f64,
        var_sp_s_temp_slot: &mut f64,
        var_sp_s_temp2_slot: &mut f64,
        var_sp_s_temp2_dn4_slot: &mut f64,
        var_sp_s_temp2_dn6_slot: &mut f64,
        var_sp_s_temp2_dn7_slot: &mut f64,
        var_sp_s_temp2_dn8_slot: &mut f64,
        var_sp_s_temp2_dn9_slot: &mut f64,
        var_sp_s_temp_dn4_slot: &mut f64,
        var_sp_s_temp_dn6_slot: &mut f64,
        var_sp_s_temp_dn7_slot: &mut f64,
        var_sp_s_temp_dn8_slot: &mut f64,
        var_sp_s_temp_dn9_slot: &mut f64,
        var_sp_s_x0_slot: &mut f64,
        var_sp_s_x0_dn4_slot: &mut f64,
        var_sp_s_x0_dn6_slot: &mut f64,
        var_sp_s_x0_dn7_slot: &mut f64,
        var_sp_s_x0_dn8_slot: &mut f64,
        var_sp_s_x0_dn9_slot: &mut f64,
        var_sp_s_xi0_slot: &mut f64,
        var_sp_s_xi0_dn4_slot: &mut f64,
        var_sp_s_xi0_dn6_slot: &mut f64,
        var_sp_s_xi0_dn7_slot: &mut f64,
        var_sp_s_xi0_dn8_slot: &mut f64,
        var_sp_s_xi0_dn9_slot: &mut f64,
        var_sp_s_xi1_slot: &mut f64,
        var_sp_s_xi1_dn4_slot: &mut f64,
        var_sp_s_xi1_dn6_slot: &mut f64,
        var_sp_s_xi1_dn7_slot: &mut f64,
        var_sp_s_xi1_dn8_slot: &mut f64,
        var_sp_s_xi1_dn9_slot: &mut f64,
        var_sp_s_xi2_slot: &mut f64,
        var_sp_s_xi2_dn4_slot: &mut f64,
        var_sp_s_xi2_dn6_slot: &mut f64,
        var_sp_s_xi2_dn7_slot: &mut f64,
        var_sp_s_xi2_dn8_slot: &mut f64,
        var_sp_s_xi2_dn9_slot: &mut f64,
    ) {
        let mut var_guard1203: f64 = *var_guard1203_slot;
        let mut var_guard1204: f64 = *var_guard1204_slot;
        let mut var_mutau: f64 = *var_mutau_slot;
        let mut var_mutau_dn4: f64 = *var_mutau_dn4_slot;
        let mut var_mutau_dn6: f64 = *var_mutau_dn6_slot;
        let mut var_mutau_dn7: f64 = *var_mutau_dn7_slot;
        let mut var_mutau_dn8: f64 = *var_mutau_dn8_slot;
        let mut var_mutau_dn9: f64 = *var_mutau_dn9_slot;
        let mut var_nu: f64 = *var_nu_slot;
        let mut var_nu_dn4: f64 = *var_nu_dn4_slot;
        let mut var_nu_dn6: f64 = *var_nu_dn6_slot;
        let mut var_nu_dn7: f64 = *var_nu_dn7_slot;
        let mut var_nu_dn8: f64 = *var_nu_dn8_slot;
        let mut var_nu_dn9: f64 = *var_nu_dn9_slot;
        let mut var_sp_s_a: f64 = *var_sp_s_a_slot;
        let mut var_sp_s_a_dn4: f64 = *var_sp_s_a_dn4_slot;
        let mut var_sp_s_a_dn6: f64 = *var_sp_s_a_dn6_slot;
        let mut var_sp_s_a_dn7: f64 = *var_sp_s_a_dn7_slot;
        let mut var_sp_s_a_dn8: f64 = *var_sp_s_a_dn8_slot;
        let mut var_sp_s_a_dn9: f64 = *var_sp_s_a_dn9_slot;
        let mut var_sp_s_b: f64 = *var_sp_s_b_slot;
        let mut var_sp_s_b_dn4: f64 = *var_sp_s_b_dn4_slot;
        let mut var_sp_s_b_dn6: f64 = *var_sp_s_b_dn6_slot;
        let mut var_sp_s_b_dn7: f64 = *var_sp_s_b_dn7_slot;
        let mut var_sp_s_b_dn8: f64 = *var_sp_s_b_dn8_slot;
        let mut var_sp_s_b_dn9: f64 = *var_sp_s_b_dn9_slot;
        let mut var_sp_s_c: f64 = *var_sp_s_c_slot;
        let mut var_sp_s_c_dn4: f64 = *var_sp_s_c_dn4_slot;
        let mut var_sp_s_c_dn6: f64 = *var_sp_s_c_dn6_slot;
        let mut var_sp_s_c_dn7: f64 = *var_sp_s_c_dn7_slot;
        let mut var_sp_s_c_dn8: f64 = *var_sp_s_c_dn8_slot;
        let mut var_sp_s_c_dn9: f64 = *var_sp_s_c_dn9_slot;
        let mut var_sp_s_delta0: f64 = *var_sp_s_delta0_slot;
        let mut var_sp_s_delta0_dn4: f64 = *var_sp_s_delta0_dn4_slot;
        let mut var_sp_s_delta0_dn6: f64 = *var_sp_s_delta0_dn6_slot;
        let mut var_sp_s_delta0_dn7: f64 = *var_sp_s_delta0_dn7_slot;
        let mut var_sp_s_delta0_dn8: f64 = *var_sp_s_delta0_dn8_slot;
        let mut var_sp_s_delta0_dn9: f64 = *var_sp_s_delta0_dn9_slot;
        let mut var_sp_s_delta1: f64 = *var_sp_s_delta1_slot;
        let mut var_sp_s_delta1_dn4: f64 = *var_sp_s_delta1_dn4_slot;
        let mut var_sp_s_delta1_dn6: f64 = *var_sp_s_delta1_dn6_slot;
        let mut var_sp_s_delta1_dn7: f64 = *var_sp_s_delta1_dn7_slot;
        let mut var_sp_s_delta1_dn8: f64 = *var_sp_s_delta1_dn8_slot;
        let mut var_sp_s_delta1_dn9: f64 = *var_sp_s_delta1_dn9_slot;
        let mut var_sp_s_pc: f64 = *var_sp_s_pc_slot;
        let mut var_sp_s_pc_dn4: f64 = *var_sp_s_pc_dn4_slot;
        let mut var_sp_s_pc_dn6: f64 = *var_sp_s_pc_dn6_slot;
        let mut var_sp_s_pc_dn7: f64 = *var_sp_s_pc_dn7_slot;
        let mut var_sp_s_pc_dn8: f64 = *var_sp_s_pc_dn8_slot;
        let mut var_sp_s_pc_dn9: f64 = *var_sp_s_pc_dn9_slot;
        let mut var_sp_s_qc: f64 = *var_sp_s_qc_slot;
        let mut var_sp_s_qc_dn4: f64 = *var_sp_s_qc_dn4_slot;
        let mut var_sp_s_qc_dn6: f64 = *var_sp_s_qc_dn6_slot;
        let mut var_sp_s_qc_dn7: f64 = *var_sp_s_qc_dn7_slot;
        let mut var_sp_s_qc_dn8: f64 = *var_sp_s_qc_dn8_slot;
        let mut var_sp_s_qc_dn9: f64 = *var_sp_s_qc_dn9_slot;
        let mut var_sp_s_tau: f64 = *var_sp_s_tau_slot;
        let mut var_sp_s_tau_dn4: f64 = *var_sp_s_tau_dn4_slot;
        let mut var_sp_s_tau_dn6: f64 = *var_sp_s_tau_dn6_slot;
        let mut var_sp_s_tau_dn7: f64 = *var_sp_s_tau_dn7_slot;
        let mut var_sp_s_tau_dn8: f64 = *var_sp_s_tau_dn8_slot;
        let mut var_sp_s_tau_dn9: f64 = *var_sp_s_tau_dn9_slot;
        let mut var_sp_s_temp: f64 = *var_sp_s_temp_slot;
        let mut var_sp_s_temp2: f64 = *var_sp_s_temp2_slot;
        let mut var_sp_s_temp2_dn4: f64 = *var_sp_s_temp2_dn4_slot;
        let mut var_sp_s_temp2_dn6: f64 = *var_sp_s_temp2_dn6_slot;
        let mut var_sp_s_temp2_dn7: f64 = *var_sp_s_temp2_dn7_slot;
        let mut var_sp_s_temp2_dn8: f64 = *var_sp_s_temp2_dn8_slot;
        let mut var_sp_s_temp2_dn9: f64 = *var_sp_s_temp2_dn9_slot;
        let mut var_sp_s_temp_dn4: f64 = *var_sp_s_temp_dn4_slot;
        let mut var_sp_s_temp_dn6: f64 = *var_sp_s_temp_dn6_slot;
        let mut var_sp_s_temp_dn7: f64 = *var_sp_s_temp_dn7_slot;
        let mut var_sp_s_temp_dn8: f64 = *var_sp_s_temp_dn8_slot;
        let mut var_sp_s_temp_dn9: f64 = *var_sp_s_temp_dn9_slot;
        let mut var_sp_s_x0: f64 = *var_sp_s_x0_slot;
        let mut var_sp_s_x0_dn4: f64 = *var_sp_s_x0_dn4_slot;
        let mut var_sp_s_x0_dn6: f64 = *var_sp_s_x0_dn6_slot;
        let mut var_sp_s_x0_dn7: f64 = *var_sp_s_x0_dn7_slot;
        let mut var_sp_s_x0_dn8: f64 = *var_sp_s_x0_dn8_slot;
        let mut var_sp_s_x0_dn9: f64 = *var_sp_s_x0_dn9_slot;
        let mut var_sp_s_xi0: f64 = *var_sp_s_xi0_slot;
        let mut var_sp_s_xi0_dn4: f64 = *var_sp_s_xi0_dn4_slot;
        let mut var_sp_s_xi0_dn6: f64 = *var_sp_s_xi0_dn6_slot;
        let mut var_sp_s_xi0_dn7: f64 = *var_sp_s_xi0_dn7_slot;
        let mut var_sp_s_xi0_dn8: f64 = *var_sp_s_xi0_dn8_slot;
        let mut var_sp_s_xi0_dn9: f64 = *var_sp_s_xi0_dn9_slot;
        let mut var_sp_s_xi1: f64 = *var_sp_s_xi1_slot;
        let mut var_sp_s_xi1_dn4: f64 = *var_sp_s_xi1_dn4_slot;
        let mut var_sp_s_xi1_dn6: f64 = *var_sp_s_xi1_dn6_slot;
        let mut var_sp_s_xi1_dn7: f64 = *var_sp_s_xi1_dn7_slot;
        let mut var_sp_s_xi1_dn8: f64 = *var_sp_s_xi1_dn8_slot;
        let mut var_sp_s_xi1_dn9: f64 = *var_sp_s_xi1_dn9_slot;
        let mut var_sp_s_xi2: f64 = *var_sp_s_xi2_slot;
        let mut var_sp_s_xi2_dn4: f64 = *var_sp_s_xi2_dn4_slot;
        let mut var_sp_s_xi2_dn6: f64 = *var_sp_s_xi2_dn6_slot;
        let mut var_sp_s_xi2_dn7: f64 = *var_sp_s_xi2_dn7_slot;
        let mut var_sp_s_xi2_dn8: f64 = *var_sp_s_xi2_dn8_slot;
        let mut var_sp_s_xi2_dn9: f64 = *var_sp_s_xi2_dn9_slot;

        let (assign42180_e55334, assign42180_e55334_d_n4, assign42180_e55334_d_n6, assign42180_e55334_d_n7, assign42180_e55334_d_n8, assign42180_e55334_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42180_e55330: f64 = (var_sp_s_eta * var_sp_s_eta);
        let assign42180_e55331: f64 = (2.0 + assign42180_e55330);
        let assign42180_e55332: f64 = (1.0 / assign42180_e55331);
        (assign42180_e55332, (-(((var_sp_s_eta_dn4 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn4)) / (assign42180_e55331 * assign42180_e55331))), (-(((var_sp_s_eta_dn6 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn6)) / (assign42180_e55331 * assign42180_e55331))), (-(((var_sp_s_eta_dn7 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn7)) / (assign42180_e55331 * assign42180_e55331))), (-(((var_sp_s_eta_dn8 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn8)) / (assign42180_e55331 * assign42180_e55331))), (-(((var_sp_s_eta_dn9 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn9)) / (assign42180_e55331 * assign42180_e55331))),)
    } else {
        (var_sp_s_temp2, var_sp_s_temp2_dn4, var_sp_s_temp2_dn6, var_sp_s_temp2_dn7, var_sp_s_temp2_dn8, var_sp_s_temp2_dn9,)
    }
};
        var_sp_s_temp2 = assign42180_e55334;
        var_sp_s_temp2_dn4 = assign42180_e55334_d_n4;
        var_sp_s_temp2_dn6 = assign42180_e55334_d_n6;
        var_sp_s_temp2_dn7 = assign42180_e55334_d_n7;
        var_sp_s_temp2_dn8 = assign42180_e55334_d_n8;
        var_sp_s_temp2_dn9 = assign42180_e55334_d_n9;

        let (assign42190_e55346, assign42190_e55346_d_n4, assign42190_e55346_d_n6, assign42190_e55346_d_n7, assign42190_e55346_d_n8, assign42190_e55346_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42190_e55342: f64 = (var_sp_s_eta * var_sp_s_eta);
        let assign42190_e55344: f64 = (assign42190_e55342 * var_sp_s_temp2);
        (assign42190_e55344, ((((var_sp_s_eta_dn4 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn4)) * var_sp_s_temp2) + (assign42190_e55342 * var_sp_s_temp2_dn4)), ((((var_sp_s_eta_dn6 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn6)) * var_sp_s_temp2) + (assign42190_e55342 * var_sp_s_temp2_dn6)), ((((var_sp_s_eta_dn7 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn7)) * var_sp_s_temp2) + (assign42190_e55342 * var_sp_s_temp2_dn7)), ((((var_sp_s_eta_dn8 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn8)) * var_sp_s_temp2) + (assign42190_e55342 * var_sp_s_temp2_dn8)), ((((var_sp_s_eta_dn9 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn9)) * var_sp_s_temp2) + (assign42190_e55342 * var_sp_s_temp2_dn9)),)
    } else {
        (var_sp_s_xi0, var_sp_s_xi0_dn4, var_sp_s_xi0_dn6, var_sp_s_xi0_dn7, var_sp_s_xi0_dn8, var_sp_s_xi0_dn9,)
    }
};
        var_sp_s_xi0 = assign42190_e55346;
        var_sp_s_xi0_dn4 = assign42190_e55346_d_n4;
        var_sp_s_xi0_dn6 = assign42190_e55346_d_n6;
        var_sp_s_xi0_dn7 = assign42190_e55346_d_n7;
        var_sp_s_xi0_dn8 = assign42190_e55346_d_n8;
        var_sp_s_xi0_dn9 = assign42190_e55346_d_n9;

        let (assign42200_e55360, assign42200_e55360_d_n4, assign42200_e55360_d_n6, assign42200_e55360_d_n7, assign42200_e55360_d_n8, assign42200_e55360_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42200_e55355: f64 = (var_sp_s_eta * var_sp_s_temp2);
        let assign42200_e55357: f64 = (assign42200_e55355 * var_sp_s_temp2);
        let assign42200_e55358: f64 = (4.0 * assign42200_e55357);
        (assign42200_e55358, (4.0 * ((((var_sp_s_eta_dn4 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn4)) * var_sp_s_temp2) + (assign42200_e55355 * var_sp_s_temp2_dn4))), (4.0 * ((((var_sp_s_eta_dn6 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn6)) * var_sp_s_temp2) + (assign42200_e55355 * var_sp_s_temp2_dn6))), (4.0 * ((((var_sp_s_eta_dn7 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn7)) * var_sp_s_temp2) + (assign42200_e55355 * var_sp_s_temp2_dn7))), (4.0 * ((((var_sp_s_eta_dn8 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn8)) * var_sp_s_temp2) + (assign42200_e55355 * var_sp_s_temp2_dn8))), (4.0 * ((((var_sp_s_eta_dn9 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn9)) * var_sp_s_temp2) + (assign42200_e55355 * var_sp_s_temp2_dn9))),)
    } else {
        (var_sp_s_xi1, var_sp_s_xi1_dn4, var_sp_s_xi1_dn6, var_sp_s_xi1_dn7, var_sp_s_xi1_dn8, var_sp_s_xi1_dn9,)
    }
};
        var_sp_s_xi1 = assign42200_e55360;
        var_sp_s_xi1_dn4 = assign42200_e55360_d_n4;
        var_sp_s_xi1_dn6 = assign42200_e55360_d_n6;
        var_sp_s_xi1_dn7 = assign42200_e55360_d_n7;
        var_sp_s_xi1_dn8 = assign42200_e55360_d_n8;
        var_sp_s_xi1_dn9 = assign42200_e55360_d_n9;

        let (assign42210_e55378, assign42210_e55378_d_n4, assign42210_e55378_d_n6, assign42210_e55378_d_n7, assign42210_e55378_d_n8, assign42210_e55378_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42210_e55368: f64 = (8.0 * var_sp_s_temp2);
        let assign42210_e55371: f64 = (12.0 * var_sp_s_xi0);
        let assign42210_e55372: f64 = (assign42210_e55368 - assign42210_e55371);
        let assign42210_e55374: f64 = (assign42210_e55372 * var_sp_s_temp2);
        let assign42210_e55376: f64 = (assign42210_e55374 * var_sp_s_temp2);
        (assign42210_e55376, ((((((8.0 * var_sp_s_temp2_dn4) - (12.0 * var_sp_s_xi0_dn4)) * var_sp_s_temp2) + (assign42210_e55372 * var_sp_s_temp2_dn4)) * var_sp_s_temp2) + (assign42210_e55374 * var_sp_s_temp2_dn4)), ((((((8.0 * var_sp_s_temp2_dn6) - (12.0 * var_sp_s_xi0_dn6)) * var_sp_s_temp2) + (assign42210_e55372 * var_sp_s_temp2_dn6)) * var_sp_s_temp2) + (assign42210_e55374 * var_sp_s_temp2_dn6)), ((((((8.0 * var_sp_s_temp2_dn7) - (12.0 * var_sp_s_xi0_dn7)) * var_sp_s_temp2) + (assign42210_e55372 * var_sp_s_temp2_dn7)) * var_sp_s_temp2) + (assign42210_e55374 * var_sp_s_temp2_dn7)), ((((((8.0 * var_sp_s_temp2_dn8) - (12.0 * var_sp_s_xi0_dn8)) * var_sp_s_temp2) + (assign42210_e55372 * var_sp_s_temp2_dn8)) * var_sp_s_temp2) + (assign42210_e55374 * var_sp_s_temp2_dn8)), ((((((8.0 * var_sp_s_temp2_dn9) - (12.0 * var_sp_s_xi0_dn9)) * var_sp_s_temp2) + (assign42210_e55372 * var_sp_s_temp2_dn9)) * var_sp_s_temp2) + (assign42210_e55374 * var_sp_s_temp2_dn9)),)
    } else {
        (var_sp_s_xi2, var_sp_s_xi2_dn4, var_sp_s_xi2_dn6, var_sp_s_xi2_dn7, var_sp_s_xi2_dn8, var_sp_s_xi2_dn9,)
    }
};
        var_sp_s_xi2 = assign42210_e55378;
        var_sp_s_xi2_dn4 = assign42210_e55378_d_n4;
        var_sp_s_xi2_dn6 = assign42210_e55378_d_n6;
        var_sp_s_xi2_dn7 = assign42210_e55378_d_n7;
        var_sp_s_xi2_dn8 = assign42210_e55378_d_n8;
        var_sp_s_xi2_dn9 = assign42210_e55378_d_n9;

        let (assign42220_e55427, assign42220_e55427_d_n4, assign42220_e55427_d_n6, assign42220_e55427_d_n7, assign42220_e55427_d_n8, assign42220_e55427_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42220_e55387: f64 = (var_sp_s_temp * var_sp_s_temp);
        let assign42220_e55391: f64 = (var_sp_s_temp1 + var_sp_s_eta);
        let assign42220_e55393: f64 = (assign42220_e55391 - 1.0);
        let assign42220_e55397: f64 = (var_sp_s_eta + 1.0);
        let assign42220_e55399: f64 = (assign42220_e55397 + var_sp_s_xi0);
        let assign42220_e55400: f64 = (var_delta_ns * assign42220_e55399);
        let assign42220_e55401: f64 = (assign42220_e55393 - assign42220_e55400);
        let assign42220_e55402: f64 = (var_gf2 * assign42220_e55401);
        let assign42220_e55403: f64 = (assign42220_e55387 - assign42220_e55402);
        let (assign42220_e55425, assign42220_e55425_d_n4, assign42220_e55425_d_n6, assign42220_e55425_d_n7, assign42220_e55425_d_n8, assign42220_e55425_d_n9,) = {
            if (1e-40 > assign42220_e55403) {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42220_e55408: f64 = (var_sp_s_temp * var_sp_s_temp);
                let assign42220_e55412: f64 = (var_sp_s_temp1 + var_sp_s_eta);
                let assign42220_e55414: f64 = (assign42220_e55412 - 1.0);
                let assign42220_e55418: f64 = (var_sp_s_eta + 1.0);
                let assign42220_e55420: f64 = (assign42220_e55418 + var_sp_s_xi0);
                let assign42220_e55421: f64 = (var_delta_ns * assign42220_e55420);
                let assign42220_e55422: f64 = (assign42220_e55414 - assign42220_e55421);
                let assign42220_e55423: f64 = (var_gf2 * assign42220_e55422);
                let assign42220_e55424: f64 = (assign42220_e55408 - assign42220_e55423);
                (assign42220_e55424, (((var_sp_s_temp_dn4 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn4)) - ((var_gf2_dn4 * assign42220_e55422) + (var_gf2 * ((var_sp_s_temp1_dn4 + var_sp_s_eta_dn4) - ((var_delta_ns_dn4 * assign42220_e55420) + (var_delta_ns * (var_sp_s_eta_dn4 + var_sp_s_xi0_dn4))))))), (((var_sp_s_temp_dn6 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn6)) - ((var_gf2_dn6 * assign42220_e55422) + (var_gf2 * ((var_sp_s_temp1_dn6 + var_sp_s_eta_dn6) - ((var_delta_ns_dn6 * assign42220_e55420) + (var_delta_ns * (var_sp_s_eta_dn6 + var_sp_s_xi0_dn6))))))), (((var_sp_s_temp_dn7 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn7)) - ((var_gf2_dn7 * assign42220_e55422) + (var_gf2 * ((var_sp_s_temp1_dn7 + var_sp_s_eta_dn7) - ((var_delta_ns_dn7 * assign42220_e55420) + (var_delta_ns * (var_sp_s_eta_dn7 + var_sp_s_xi0_dn7))))))), (((var_sp_s_temp_dn8 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn8)) - ((var_gf2_dn8 * assign42220_e55422) + (var_gf2 * ((var_sp_s_temp1_dn8 + var_sp_s_eta_dn8) - ((var_delta_ns_dn8 * assign42220_e55420) + (var_delta_ns * (var_sp_s_eta_dn8 + var_sp_s_xi0_dn8))))))), (((var_sp_s_temp_dn9 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn9)) - ((var_gf2_dn9 * assign42220_e55422) + (var_gf2 * ((var_sp_s_temp1_dn9 + var_sp_s_eta_dn9) - ((var_delta_ns_dn9 * assign42220_e55420) + (var_delta_ns * (var_sp_s_eta_dn9 + var_sp_s_xi0_dn9))))))),)
            }
        };
        (assign42220_e55425, assign42220_e55425_d_n4, assign42220_e55425_d_n6, assign42220_e55425_d_n7, assign42220_e55425_d_n8, assign42220_e55425_d_n9,)
    } else {
        (var_sp_s_a, var_sp_s_a_dn4, var_sp_s_a_dn6, var_sp_s_a_dn7, var_sp_s_a_dn8, var_sp_s_a_dn9,)
    }
};
        var_sp_s_a = assign42220_e55427;
        var_sp_s_a_dn4 = assign42220_e55427_d_n4;
        var_sp_s_a_dn6 = assign42220_e55427_d_n6;
        var_sp_s_a_dn7 = assign42220_e55427_d_n7;
        var_sp_s_a_dn8 = assign42220_e55427_d_n8;
        var_sp_s_a_dn9 = assign42220_e55427_d_n9;

        let (assign42230_e55445, assign42230_e55445_d_n4, assign42230_e55445_d_n6, assign42230_e55445_d_n7, assign42230_e55445_d_n8, assign42230_e55445_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42230_e55439: f64 = (var_delta_ns * var_sp_s_xi2);
        let assign42230_e55440: f64 = (var_sp_s_temp1 - assign42230_e55439);
        let assign42230_e55441: f64 = (var_gf2 * assign42230_e55440);
        let assign42230_e55442: f64 = (0.5 * assign42230_e55441);
        let assign42230_e55443: f64 = (1.0 - assign42230_e55442);
        (assign42230_e55443, (-(0.5 * ((var_gf2_dn4 * assign42230_e55440) + (var_gf2 * (var_sp_s_temp1_dn4 - ((var_delta_ns_dn4 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn4))))))), (-(0.5 * ((var_gf2_dn6 * assign42230_e55440) + (var_gf2 * (var_sp_s_temp1_dn6 - ((var_delta_ns_dn6 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn6))))))), (-(0.5 * ((var_gf2_dn7 * assign42230_e55440) + (var_gf2 * (var_sp_s_temp1_dn7 - ((var_delta_ns_dn7 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn7))))))), (-(0.5 * ((var_gf2_dn8 * assign42230_e55440) + (var_gf2 * (var_sp_s_temp1_dn8 - ((var_delta_ns_dn8 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn8))))))), (-(0.5 * ((var_gf2_dn9 * assign42230_e55440) + (var_gf2 * (var_sp_s_temp1_dn9 - ((var_delta_ns_dn9 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn9))))))),)
    } else {
        (var_sp_s_b, var_sp_s_b_dn4, var_sp_s_b_dn6, var_sp_s_b_dn7, var_sp_s_b_dn8, var_sp_s_b_dn9,)
    }
};
        var_sp_s_b = assign42230_e55445;
        var_sp_s_b_dn4 = assign42230_e55445_d_n4;
        var_sp_s_b_dn6 = assign42230_e55445_d_n6;
        var_sp_s_b_dn7 = assign42230_e55445_d_n7;
        var_sp_s_b_dn8 = assign42230_e55445_d_n8;
        var_sp_s_b_dn9 = assign42230_e55445_d_n9;

        let (assign42240_e55467, assign42240_e55467_d_n4, assign42240_e55467_d_n6, assign42240_e55467_d_n7, assign42240_e55467_d_n8, assign42240_e55467_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42240_e55453: f64 = (2.0 * var_sp_s_temp);
        let assign42240_e55457: f64 = (1.0 - var_sp_s_temp1);
        let assign42240_e55461: f64 = (1.0 + var_sp_s_xi1);
        let assign42240_e55462: f64 = (var_delta_ns * assign42240_e55461);
        let assign42240_e55463: f64 = (assign42240_e55457 - assign42240_e55462);
        let assign42240_e55464: f64 = (var_gf2 * assign42240_e55463);
        let assign42240_e55465: f64 = (assign42240_e55453 + assign42240_e55464);
        (assign42240_e55465, ((2.0 * var_sp_s_temp_dn4) + ((var_gf2_dn4 * assign42240_e55463) + (var_gf2 * ((-var_sp_s_temp1_dn4) - ((var_delta_ns_dn4 * assign42240_e55461) + (var_delta_ns * var_sp_s_xi1_dn4)))))), ((2.0 * var_sp_s_temp_dn6) + ((var_gf2_dn6 * assign42240_e55463) + (var_gf2 * ((-var_sp_s_temp1_dn6) - ((var_delta_ns_dn6 * assign42240_e55461) + (var_delta_ns * var_sp_s_xi1_dn6)))))), ((2.0 * var_sp_s_temp_dn7) + ((var_gf2_dn7 * assign42240_e55463) + (var_gf2 * ((-var_sp_s_temp1_dn7) - ((var_delta_ns_dn7 * assign42240_e55461) + (var_delta_ns * var_sp_s_xi1_dn7)))))), ((2.0 * var_sp_s_temp_dn8) + ((var_gf2_dn8 * assign42240_e55463) + (var_gf2 * ((-var_sp_s_temp1_dn8) - ((var_delta_ns_dn8 * assign42240_e55461) + (var_delta_ns * var_sp_s_xi1_dn8)))))), ((2.0 * var_sp_s_temp_dn9) + ((var_gf2_dn9 * assign42240_e55463) + (var_gf2 * ((-var_sp_s_temp1_dn9) - ((var_delta_ns_dn9 * assign42240_e55461) + (var_delta_ns * var_sp_s_xi1_dn9)))))),)
    } else {
        (var_sp_s_c, var_sp_s_c_dn4, var_sp_s_c_dn6, var_sp_s_c_dn7, var_sp_s_c_dn8, var_sp_s_c_dn9,)
    }
};
        var_sp_s_c = assign42240_e55467;
        var_sp_s_c_dn4 = assign42240_e55467_d_n4;
        var_sp_s_c_dn6 = assign42240_e55467_d_n6;
        var_sp_s_c_dn7 = assign42240_e55467_d_n7;
        var_sp_s_c_dn8 = assign42240_e55467_d_n8;
        var_sp_s_c_dn9 = assign42240_e55467_d_n9;

        let (assign42250_e55482, assign42250_e55482_d_n4, assign42250_e55482_d_n6, assign42250_e55482_d_n7, assign42250_e55482_d_n8, assign42250_e55482_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42250_e55475: f64 = (var_xn_s - var_sp_s_eta);
        let assign42250_e55478: f64 = (var_sp_s_a / var_gf2);
        let assign42250_e55479: f64 = (assign42250_e55478).ln();
        let assign42250_e55480: f64 = (assign42250_e55475 + assign42250_e55479);
        (assign42250_e55480, ((var_xn_s_dn4 - var_sp_s_eta_dn4) + ((((var_sp_s_a_dn4 * var_gf2) - (var_sp_s_a * var_gf2_dn4)) / (var_gf2 * var_gf2)) / assign42250_e55478)), ((var_xn_s_dn6 - var_sp_s_eta_dn6) + ((((var_sp_s_a_dn6 * var_gf2) - (var_sp_s_a * var_gf2_dn6)) / (var_gf2 * var_gf2)) / assign42250_e55478)), ((var_xn_s_dn7 - var_sp_s_eta_dn7) + ((((var_sp_s_a_dn7 * var_gf2) - (var_sp_s_a * var_gf2_dn7)) / (var_gf2 * var_gf2)) / assign42250_e55478)), ((var_xn_s_dn8 - var_sp_s_eta_dn8) + ((((var_sp_s_a_dn8 * var_gf2) - (var_sp_s_a * var_gf2_dn8)) / (var_gf2 * var_gf2)) / assign42250_e55478)), ((var_xn_s_dn9 - var_sp_s_eta_dn9) + ((((var_sp_s_a_dn9 * var_gf2) - (var_sp_s_a * var_gf2_dn9)) / (var_gf2 * var_gf2)) / assign42250_e55478)),)
    } else {
        (var_sp_s_tau, var_sp_s_tau_dn4, var_sp_s_tau_dn6, var_sp_s_tau_dn7, var_sp_s_tau_dn8, var_sp_s_tau_dn9,)
    }
};
        var_sp_s_tau = assign42250_e55482;
        var_sp_s_tau_dn4 = assign42250_e55482_d_n4;
        var_sp_s_tau_dn6 = assign42250_e55482_d_n6;
        var_sp_s_tau_dn7 = assign42250_e55482_d_n7;
        var_sp_s_tau_dn8 = assign42250_e55482_d_n8;
        var_sp_s_tau_dn9 = assign42250_e55482_d_n9;

        let (assign42260_e55492, assign42260_e55492_d_n4, assign42260_e55492_d_n6, assign42260_e55492_d_n7, assign42260_e55492_d_n8, assign42260_e55492_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42260_e55490: f64 = (var_sp_s_a + var_sp_s_c);
        (assign42260_e55490, (var_sp_s_a_dn4 + var_sp_s_c_dn4), (var_sp_s_a_dn6 + var_sp_s_c_dn6), (var_sp_s_a_dn7 + var_sp_s_c_dn7), (var_sp_s_a_dn8 + var_sp_s_c_dn8), (var_sp_s_a_dn9 + var_sp_s_c_dn9),)
    } else {
        (var_nu, var_nu_dn4, var_nu_dn6, var_nu_dn7, var_nu_dn8, var_nu_dn9,)
    }
};
        var_nu = assign42260_e55492;
        var_nu_dn4 = assign42260_e55492_d_n4;
        var_nu_dn6 = assign42260_e55492_d_n6;
        var_nu_dn7 = assign42260_e55492_d_n7;
        var_nu_dn8 = assign42260_e55492_d_n8;
        var_nu_dn9 = assign42260_e55492_d_n9;

        let (assign42270_e55514, assign42270_e55514_d_n4, assign42270_e55514_d_n6, assign42270_e55514_d_n7, assign42270_e55514_d_n8, assign42270_e55514_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42270_e55500: f64 = (var_nu * var_nu);
        let assign42270_e55505: f64 = (var_sp_s_c * var_sp_s_c);
        let assign42270_e55506: f64 = (0.5 * assign42270_e55505);
        let assign42270_e55509: f64 = (var_sp_s_a * var_sp_s_b);
        let assign42270_e55510: f64 = (assign42270_e55506 - assign42270_e55509);
        let assign42270_e55511: f64 = (var_sp_s_tau * assign42270_e55510);
        let assign42270_e55512: f64 = (assign42270_e55500 + assign42270_e55511);
        (assign42270_e55512, (((var_nu_dn4 * var_nu) + (var_nu * var_nu_dn4)) + ((var_sp_s_tau_dn4 * assign42270_e55510) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn4 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn4))) - ((var_sp_s_a_dn4 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn4)))))), (((var_nu_dn6 * var_nu) + (var_nu * var_nu_dn6)) + ((var_sp_s_tau_dn6 * assign42270_e55510) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn6 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn6))) - ((var_sp_s_a_dn6 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn6)))))), (((var_nu_dn7 * var_nu) + (var_nu * var_nu_dn7)) + ((var_sp_s_tau_dn7 * assign42270_e55510) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn7 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn7))) - ((var_sp_s_a_dn7 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn7)))))), (((var_nu_dn8 * var_nu) + (var_nu * var_nu_dn8)) + ((var_sp_s_tau_dn8 * assign42270_e55510) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn8 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn8))) - ((var_sp_s_a_dn8 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn8)))))), (((var_nu_dn9 * var_nu) + (var_nu * var_nu_dn9)) + ((var_sp_s_tau_dn9 * assign42270_e55510) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn9 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn9))) - ((var_sp_s_a_dn9 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn9)))))),)
    } else {
        (var_mutau, var_mutau_dn4, var_mutau_dn6, var_mutau_dn7, var_mutau_dn8, var_mutau_dn9,)
    }
};
        var_mutau = assign42270_e55514;
        var_mutau_dn4 = assign42270_e55514_d_n4;
        var_mutau_dn6 = assign42270_e55514_d_n6;
        var_mutau_dn7 = assign42270_e55514_d_n7;
        var_mutau_dn8 = assign42270_e55514_d_n8;
        var_mutau_dn9 = assign42270_e55514_d_n9;

        let (assign42280_e55550, assign42280_e55550_d_n4, assign42280_e55550_d_n6, assign42280_e55550_d_n7, assign42280_e55550_d_n8, assign42280_e55550_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42280_e55523: f64 = (var_sp_s_a * var_nu);
        let assign42280_e55525: f64 = (assign42280_e55523 * var_sp_s_tau);
        let assign42280_e55529: f64 = (var_nu / var_mutau);
        let assign42280_e55531: f64 = (assign42280_e55529 * var_sp_s_tau);
        let assign42280_e55533: f64 = (assign42280_e55531 * var_sp_s_tau);
        let assign42280_e55535: f64 = (assign42280_e55533 * var_sp_s_c);
        let assign42280_e55538: f64 = (var_sp_s_c * var_sp_s_c);
        let assign42280_e55540: f64 = (assign42280_e55538 * 0.3333333333333333);
        let assign42280_e55543: f64 = (var_sp_s_a * var_sp_s_b);
        let assign42280_e55544: f64 = (assign42280_e55540 - assign42280_e55543);
        let assign42280_e55545: f64 = (assign42280_e55535 * assign42280_e55544);
        let assign42280_e55546: f64 = (var_mutau + assign42280_e55545);
        let assign42280_e55547: f64 = (assign42280_e55525 / assign42280_e55546);
        let assign42280_e55548: f64 = (var_sp_s_eta + assign42280_e55547);
        (assign42280_e55548, (var_sp_s_eta_dn4 + (((((((var_sp_s_a_dn4 * var_nu) + (var_sp_s_a * var_nu_dn4)) * var_sp_s_tau) + (assign42280_e55523 * var_sp_s_tau_dn4)) * assign42280_e55546) - (assign42280_e55525 * (var_mutau_dn4 + (((((((((((var_nu_dn4 * var_mutau) - (var_nu * var_mutau_dn4)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign42280_e55529 * var_sp_s_tau_dn4)) * var_sp_s_tau) + (assign42280_e55531 * var_sp_s_tau_dn4)) * var_sp_s_c) + (assign42280_e55533 * var_sp_s_c_dn4)) * assign42280_e55544) + (assign42280_e55535 * ((((var_sp_s_c_dn4 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn4)) * 0.3333333333333333) - ((var_sp_s_a_dn4 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn4)))))))) / (assign42280_e55546 * assign42280_e55546))), (var_sp_s_eta_dn6 + (((((((var_sp_s_a_dn6 * var_nu) + (var_sp_s_a * var_nu_dn6)) * var_sp_s_tau) + (assign42280_e55523 * var_sp_s_tau_dn6)) * assign42280_e55546) - (assign42280_e55525 * (var_mutau_dn6 + (((((((((((var_nu_dn6 * var_mutau) - (var_nu * var_mutau_dn6)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign42280_e55529 * var_sp_s_tau_dn6)) * var_sp_s_tau) + (assign42280_e55531 * var_sp_s_tau_dn6)) * var_sp_s_c) + (assign42280_e55533 * var_sp_s_c_dn6)) * assign42280_e55544) + (assign42280_e55535 * ((((var_sp_s_c_dn6 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn6)) * 0.3333333333333333) - ((var_sp_s_a_dn6 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn6)))))))) / (assign42280_e55546 * assign42280_e55546))), (var_sp_s_eta_dn7 + (((((((var_sp_s_a_dn7 * var_nu) + (var_sp_s_a * var_nu_dn7)) * var_sp_s_tau) + (assign42280_e55523 * var_sp_s_tau_dn7)) * assign42280_e55546) - (assign42280_e55525 * (var_mutau_dn7 + (((((((((((var_nu_dn7 * var_mutau) - (var_nu * var_mutau_dn7)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign42280_e55529 * var_sp_s_tau_dn7)) * var_sp_s_tau) + (assign42280_e55531 * var_sp_s_tau_dn7)) * var_sp_s_c) + (assign42280_e55533 * var_sp_s_c_dn7)) * assign42280_e55544) + (assign42280_e55535 * ((((var_sp_s_c_dn7 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn7)) * 0.3333333333333333) - ((var_sp_s_a_dn7 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn7)))))))) / (assign42280_e55546 * assign42280_e55546))), (var_sp_s_eta_dn8 + (((((((var_sp_s_a_dn8 * var_nu) + (var_sp_s_a * var_nu_dn8)) * var_sp_s_tau) + (assign42280_e55523 * var_sp_s_tau_dn8)) * assign42280_e55546) - (assign42280_e55525 * (var_mutau_dn8 + (((((((((((var_nu_dn8 * var_mutau) - (var_nu * var_mutau_dn8)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign42280_e55529 * var_sp_s_tau_dn8)) * var_sp_s_tau) + (assign42280_e55531 * var_sp_s_tau_dn8)) * var_sp_s_c) + (assign42280_e55533 * var_sp_s_c_dn8)) * assign42280_e55544) + (assign42280_e55535 * ((((var_sp_s_c_dn8 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn8)) * 0.3333333333333333) - ((var_sp_s_a_dn8 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn8)))))))) / (assign42280_e55546 * assign42280_e55546))), (var_sp_s_eta_dn9 + (((((((var_sp_s_a_dn9 * var_nu) + (var_sp_s_a * var_nu_dn9)) * var_sp_s_tau) + (assign42280_e55523 * var_sp_s_tau_dn9)) * assign42280_e55546) - (assign42280_e55525 * (var_mutau_dn9 + (((((((((((var_nu_dn9 * var_mutau) - (var_nu * var_mutau_dn9)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign42280_e55529 * var_sp_s_tau_dn9)) * var_sp_s_tau) + (assign42280_e55531 * var_sp_s_tau_dn9)) * var_sp_s_c) + (assign42280_e55533 * var_sp_s_c_dn9)) * assign42280_e55544) + (assign42280_e55535 * ((((var_sp_s_c_dn9 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn9)) * 0.3333333333333333) - ((var_sp_s_a_dn9 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn9)))))))) / (assign42280_e55546 * assign42280_e55546))),)
    } else {
        (var_sp_s_x0, var_sp_s_x0_dn4, var_sp_s_x0_dn6, var_sp_s_x0_dn7, var_sp_s_x0_dn8, var_sp_s_x0_dn9,)
    }
};
        var_sp_s_x0 = assign42280_e55550;
        var_sp_s_x0_dn4 = assign42280_e55550_d_n4;
        var_sp_s_x0_dn6 = assign42280_e55550_d_n6;
        var_sp_s_x0_dn7 = assign42280_e55550_d_n7;
        var_sp_s_x0_dn8 = assign42280_e55550_d_n8;
        var_sp_s_x0_dn9 = assign42280_e55550_d_n9;

        let assign42290_e55553: f64 = if var_sp_s_x0 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1203 = assign42290_e55553;

        let (assign42300_e55564, assign42300_e55564_d_n4, assign42300_e55564_d_n6, assign42300_e55564_d_n7, assign42300_e55564_d_n8, assign42300_e55564_d_n9,) = {
    if (((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) && (var_guard1203 != 0.0)) {
        let assign42300_e55562: f64 = (var_sp_s_x0).exp();
        (assign42300_e55562, (assign42300_e55562 * var_sp_s_x0_dn4), (assign42300_e55562 * var_sp_s_x0_dn6), (assign42300_e55562 * var_sp_s_x0_dn7), (assign42300_e55562 * var_sp_s_x0_dn8), (assign42300_e55562 * var_sp_s_x0_dn9),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn4, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8, var_sp_s_delta0_dn9,)
    }
};
        var_sp_s_delta0 = assign42300_e55564;
        var_sp_s_delta0_dn4 = assign42300_e55564_d_n4;
        var_sp_s_delta0_dn6 = assign42300_e55564_d_n6;
        var_sp_s_delta0_dn7 = assign42300_e55564_d_n7;
        var_sp_s_delta0_dn8 = assign42300_e55564_d_n8;
        var_sp_s_delta0_dn9 = assign42300_e55564_d_n9;

        let (assign42310_e55576, assign42310_e55576_d_n4, assign42310_e55576_d_n6, assign42310_e55576_d_n7, assign42310_e55576_d_n8, assign42310_e55576_d_n9,) = {
    if (((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) && (var_guard1203 != 0.0)) {
        let assign42310_e55574: f64 = (1.0 / var_sp_s_delta0);
        (assign42310_e55574, (-(var_sp_s_delta0_dn4 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn6 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn7 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn8 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn9 / (var_sp_s_delta0 * var_sp_s_delta0))),)
    } else {
        (var_sp_s_delta1, var_sp_s_delta1_dn4, var_sp_s_delta1_dn6, var_sp_s_delta1_dn7, var_sp_s_delta1_dn8, var_sp_s_delta1_dn9,)
    }
};
        var_sp_s_delta1 = assign42310_e55576;
        var_sp_s_delta1_dn4 = assign42310_e55576_d_n4;
        var_sp_s_delta1_dn6 = assign42310_e55576_d_n6;
        var_sp_s_delta1_dn7 = assign42310_e55576_d_n7;
        var_sp_s_delta1_dn8 = assign42310_e55576_d_n8;
        var_sp_s_delta1_dn9 = assign42310_e55576_d_n9;

        let (assign42320_e55588, assign42320_e55588_d_n4, assign42320_e55588_d_n6, assign42320_e55588_d_n7, assign42320_e55588_d_n8, assign42320_e55588_d_n9,) = {
    if (((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) && (var_guard1203 != 0.0)) {
        let assign42320_e55586: f64 = (var_delta_ns * var_sp_s_delta0);
        (assign42320_e55586, ((var_delta_ns_dn4 * var_sp_s_delta0) + (var_delta_ns * var_sp_s_delta0_dn4)), ((var_delta_ns_dn6 * var_sp_s_delta0) + (var_delta_ns * var_sp_s_delta0_dn6)), ((var_delta_ns_dn7 * var_sp_s_delta0) + (var_delta_ns * var_sp_s_delta0_dn7)), ((var_delta_ns_dn8 * var_sp_s_delta0) + (var_delta_ns * var_sp_s_delta0_dn8)), ((var_delta_ns_dn9 * var_sp_s_delta0) + (var_delta_ns * var_sp_s_delta0_dn9)),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn4, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8, var_sp_s_delta0_dn9,)
    }
};
        var_sp_s_delta0 = assign42320_e55588;
        var_sp_s_delta0_dn4 = assign42320_e55588_d_n4;
        var_sp_s_delta0_dn6 = assign42320_e55588_d_n6;
        var_sp_s_delta0_dn7 = assign42320_e55588_d_n7;
        var_sp_s_delta0_dn8 = assign42320_e55588_d_n8;
        var_sp_s_delta0_dn9 = assign42320_e55588_d_n9;

        let assign42330_e55592: f64 = (var_xn_s - 230.25850929940458);
        let assign42330_e55593: f64 = if var_sp_s_x0 > assign42330_e55592 { 1.0 } else { 0.0 };
        var_guard1204 = assign42330_e55593;

        let (assign42340_e55609, assign42340_e55609_d_n4, assign42340_e55609_d_n6, assign42340_e55609_d_n7, assign42340_e55609_d_n8, assign42340_e55609_d_n9,) = {
    if ((((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) && (var_guard1203 == 0.0)) && (var_guard1204 != 0.0)) {
        let assign42340_e55606: f64 = (var_sp_s_x0 - var_xn_s);
        let assign42340_e55607: f64 = (assign42340_e55606).exp();
        (assign42340_e55607, (assign42340_e55607 * (var_sp_s_x0_dn4 - var_xn_s_dn4)), (assign42340_e55607 * (var_sp_s_x0_dn6 - var_xn_s_dn6)), (assign42340_e55607 * (var_sp_s_x0_dn7 - var_xn_s_dn7)), (assign42340_e55607 * (var_sp_s_x0_dn8 - var_xn_s_dn8)), (assign42340_e55607 * (var_sp_s_x0_dn9 - var_xn_s_dn9)),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn4, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8, var_sp_s_delta0_dn9,)
    }
};
        var_sp_s_delta0 = assign42340_e55609;
        var_sp_s_delta0_dn4 = assign42340_e55609_d_n4;
        var_sp_s_delta0_dn6 = assign42340_e55609_d_n6;
        var_sp_s_delta0_dn7 = assign42340_e55609_d_n7;
        var_sp_s_delta0_dn8 = assign42340_e55609_d_n8;
        var_sp_s_delta0_dn9 = assign42340_e55609_d_n9;

        let (assign42350_e55624, assign42350_e55624_d_n4, assign42350_e55624_d_n6, assign42350_e55624_d_n7, assign42350_e55624_d_n8, assign42350_e55624_d_n9,) = {
    if ((((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) && (var_guard1203 == 0.0)) && (var_guard1204 != 0.0)) {
        let assign42350_e55622: f64 = (var_delta_ns / var_sp_s_delta0);
        (assign42350_e55622, (((var_delta_ns_dn4 * var_sp_s_delta0) - (var_delta_ns * var_sp_s_delta0_dn4)) / (var_sp_s_delta0 * var_sp_s_delta0)), (((var_delta_ns_dn6 * var_sp_s_delta0) - (var_delta_ns * var_sp_s_delta0_dn6)) / (var_sp_s_delta0 * var_sp_s_delta0)), (((var_delta_ns_dn7 * var_sp_s_delta0) - (var_delta_ns * var_sp_s_delta0_dn7)) / (var_sp_s_delta0 * var_sp_s_delta0)), (((var_delta_ns_dn8 * var_sp_s_delta0) - (var_delta_ns * var_sp_s_delta0_dn8)) / (var_sp_s_delta0 * var_sp_s_delta0)), (((var_delta_ns_dn9 * var_sp_s_delta0) - (var_delta_ns * var_sp_s_delta0_dn9)) / (var_sp_s_delta0 * var_sp_s_delta0)),)
    } else {
        (var_sp_s_delta1, var_sp_s_delta1_dn4, var_sp_s_delta1_dn6, var_sp_s_delta1_dn7, var_sp_s_delta1_dn8, var_sp_s_delta1_dn9,)
    }
};
        var_sp_s_delta1 = assign42350_e55624;
        var_sp_s_delta1_dn4 = assign42350_e55624_d_n4;
        var_sp_s_delta1_dn6 = assign42350_e55624_d_n6;
        var_sp_s_delta1_dn7 = assign42350_e55624_d_n7;
        var_sp_s_delta1_dn8 = assign42350_e55624_d_n8;
        var_sp_s_delta1_dn9 = assign42350_e55624_d_n9;

        let (assign42360_e55666, assign42360_e55666_d_n4, assign42360_e55666_d_n6, assign42360_e55666_d_n7, assign42360_e55666_d_n8, assign42360_e55666_d_n9,) = {
    if ((((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) && (var_guard1203 == 0.0)) && (var_guard1204 == 0.0)) {
        let assign42360_e55640: f64 = (var_xn_s - var_sp_s_x0);
        let assign42360_e55642: f64 = (assign42360_e55640 - 230.25850929940458);
        let assign42360_e55647: f64 = (var_xn_s - var_sp_s_x0);
        let assign42360_e55649: f64 = (assign42360_e55647 - 230.25850929940458);
        let assign42360_e55653: f64 = (var_xn_s - var_sp_s_x0);
        let assign42360_e55655: f64 = (assign42360_e55653 - 230.25850929940458);
        let assign42360_e55657: f64 = (assign42360_e55655 * 0.3333333333333333);
        let assign42360_e55658: f64 = (1.0 + assign42360_e55657);
        let assign42360_e55659: f64 = (assign42360_e55649 * assign42360_e55658);
        let assign42360_e55660: f64 = (0.5 * assign42360_e55659);
        let assign42360_e55661: f64 = (1.0 + assign42360_e55660);
        let assign42360_e55662: f64 = (assign42360_e55642 * assign42360_e55661);
        let assign42360_e55663: f64 = (1.0 + assign42360_e55662);
        let assign42360_e55664: f64 = (1e-100 / assign42360_e55663);
        (assign42360_e55664, (-((1e-100 * (((var_xn_s_dn4 - var_sp_s_x0_dn4) * assign42360_e55661) + (assign42360_e55642 * (0.5 * (((var_xn_s_dn4 - var_sp_s_x0_dn4) * assign42360_e55658) + (assign42360_e55649 * ((var_xn_s_dn4 - var_sp_s_x0_dn4) * 0.3333333333333333))))))) / (assign42360_e55663 * assign42360_e55663))), (-((1e-100 * (((var_xn_s_dn6 - var_sp_s_x0_dn6) * assign42360_e55661) + (assign42360_e55642 * (0.5 * (((var_xn_s_dn6 - var_sp_s_x0_dn6) * assign42360_e55658) + (assign42360_e55649 * ((var_xn_s_dn6 - var_sp_s_x0_dn6) * 0.3333333333333333))))))) / (assign42360_e55663 * assign42360_e55663))), (-((1e-100 * (((var_xn_s_dn7 - var_sp_s_x0_dn7) * assign42360_e55661) + (assign42360_e55642 * (0.5 * (((var_xn_s_dn7 - var_sp_s_x0_dn7) * assign42360_e55658) + (assign42360_e55649 * ((var_xn_s_dn7 - var_sp_s_x0_dn7) * 0.3333333333333333))))))) / (assign42360_e55663 * assign42360_e55663))), (-((1e-100 * (((var_xn_s_dn8 - var_sp_s_x0_dn8) * assign42360_e55661) + (assign42360_e55642 * (0.5 * (((var_xn_s_dn8 - var_sp_s_x0_dn8) * assign42360_e55658) + (assign42360_e55649 * ((var_xn_s_dn8 - var_sp_s_x0_dn8) * 0.3333333333333333))))))) / (assign42360_e55663 * assign42360_e55663))), (-((1e-100 * (((var_xn_s_dn9 - var_sp_s_x0_dn9) * assign42360_e55661) + (assign42360_e55642 * (0.5 * (((var_xn_s_dn9 - var_sp_s_x0_dn9) * assign42360_e55658) + (assign42360_e55649 * ((var_xn_s_dn9 - var_sp_s_x0_dn9) * 0.3333333333333333))))))) / (assign42360_e55663 * assign42360_e55663))),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn4, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8, var_sp_s_delta0_dn9,)
    }
};
        var_sp_s_delta0 = assign42360_e55666;
        var_sp_s_delta0_dn4 = assign42360_e55666_d_n4;
        var_sp_s_delta0_dn6 = assign42360_e55666_d_n6;
        var_sp_s_delta0_dn7 = assign42360_e55666_d_n7;
        var_sp_s_delta0_dn8 = assign42360_e55666_d_n8;
        var_sp_s_delta0_dn9 = assign42360_e55666_d_n9;

        let (assign42370_e55702, assign42370_e55702_d_n4, assign42370_e55702_d_n6, assign42370_e55702_d_n7, assign42370_e55702_d_n8, assign42370_e55702_d_n9,) = {
    if ((((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) && (var_guard1203 == 0.0)) && (var_guard1204 == 0.0)) {
        let assign42370_e55682: f64 = (var_sp_s_x0 - 230.25850929940458);
        let assign42370_e55687: f64 = (var_sp_s_x0 - 230.25850929940458);
        let assign42370_e55691: f64 = (var_sp_s_x0 - 230.25850929940458);
        let assign42370_e55693: f64 = (assign42370_e55691 * 0.3333333333333333);
        let assign42370_e55694: f64 = (1.0 + assign42370_e55693);
        let assign42370_e55695: f64 = (assign42370_e55687 * assign42370_e55694);
        let assign42370_e55696: f64 = (0.5 * assign42370_e55695);
        let assign42370_e55697: f64 = (1.0 + assign42370_e55696);
        let assign42370_e55698: f64 = (assign42370_e55682 * assign42370_e55697);
        let assign42370_e55699: f64 = (1.0 + assign42370_e55698);
        let assign42370_e55700: f64 = (1e-100 / assign42370_e55699);
        (assign42370_e55700, (-((1e-100 * ((var_sp_s_x0_dn4 * assign42370_e55697) + (assign42370_e55682 * (0.5 * ((var_sp_s_x0_dn4 * assign42370_e55694) + (assign42370_e55687 * (var_sp_s_x0_dn4 * 0.3333333333333333))))))) / (assign42370_e55699 * assign42370_e55699))), (-((1e-100 * ((var_sp_s_x0_dn6 * assign42370_e55697) + (assign42370_e55682 * (0.5 * ((var_sp_s_x0_dn6 * assign42370_e55694) + (assign42370_e55687 * (var_sp_s_x0_dn6 * 0.3333333333333333))))))) / (assign42370_e55699 * assign42370_e55699))), (-((1e-100 * ((var_sp_s_x0_dn7 * assign42370_e55697) + (assign42370_e55682 * (0.5 * ((var_sp_s_x0_dn7 * assign42370_e55694) + (assign42370_e55687 * (var_sp_s_x0_dn7 * 0.3333333333333333))))))) / (assign42370_e55699 * assign42370_e55699))), (-((1e-100 * ((var_sp_s_x0_dn8 * assign42370_e55697) + (assign42370_e55682 * (0.5 * ((var_sp_s_x0_dn8 * assign42370_e55694) + (assign42370_e55687 * (var_sp_s_x0_dn8 * 0.3333333333333333))))))) / (assign42370_e55699 * assign42370_e55699))), (-((1e-100 * ((var_sp_s_x0_dn9 * assign42370_e55697) + (assign42370_e55682 * (0.5 * ((var_sp_s_x0_dn9 * assign42370_e55694) + (assign42370_e55687 * (var_sp_s_x0_dn9 * 0.3333333333333333))))))) / (assign42370_e55699 * assign42370_e55699))),)
    } else {
        (var_sp_s_delta1, var_sp_s_delta1_dn4, var_sp_s_delta1_dn6, var_sp_s_delta1_dn7, var_sp_s_delta1_dn8, var_sp_s_delta1_dn9,)
    }
};
        var_sp_s_delta1 = assign42370_e55702;
        var_sp_s_delta1_dn4 = assign42370_e55702_d_n4;
        var_sp_s_delta1_dn6 = assign42370_e55702_d_n6;
        var_sp_s_delta1_dn7 = assign42370_e55702_d_n7;
        var_sp_s_delta1_dn8 = assign42370_e55702_d_n8;
        var_sp_s_delta1_dn9 = assign42370_e55702_d_n9;

        let (assign42380_e55716, assign42380_e55716_d_n4, assign42380_e55716_d_n6, assign42380_e55716_d_n7, assign42380_e55716_d_n8, assign42380_e55716_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42380_e55712: f64 = (var_sp_s_x0 * var_sp_s_x0);
        let assign42380_e55713: f64 = (2.0 + assign42380_e55712);
        let assign42380_e55714: f64 = (1.0 / assign42380_e55713);
        (assign42380_e55714, (-(((var_sp_s_x0_dn4 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn4)) / (assign42380_e55713 * assign42380_e55713))), (-(((var_sp_s_x0_dn6 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn6)) / (assign42380_e55713 * assign42380_e55713))), (-(((var_sp_s_x0_dn7 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn7)) / (assign42380_e55713 * assign42380_e55713))), (-(((var_sp_s_x0_dn8 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn8)) / (assign42380_e55713 * assign42380_e55713))), (-(((var_sp_s_x0_dn9 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn9)) / (assign42380_e55713 * assign42380_e55713))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn4, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8, var_sp_s_temp_dn9,)
    }
};
        var_sp_s_temp = assign42380_e55716;
        var_sp_s_temp_dn4 = assign42380_e55716_d_n4;
        var_sp_s_temp_dn6 = assign42380_e55716_d_n6;
        var_sp_s_temp_dn7 = assign42380_e55716_d_n7;
        var_sp_s_temp_dn8 = assign42380_e55716_d_n8;
        var_sp_s_temp_dn9 = assign42380_e55716_d_n9;

        let (assign42390_e55728, assign42390_e55728_d_n4, assign42390_e55728_d_n6, assign42390_e55728_d_n7, assign42390_e55728_d_n8, assign42390_e55728_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42390_e55724: f64 = (var_sp_s_x0 * var_sp_s_x0);
        let assign42390_e55726: f64 = (assign42390_e55724 * var_sp_s_temp);
        (assign42390_e55726, ((((var_sp_s_x0_dn4 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn4)) * var_sp_s_temp) + (assign42390_e55724 * var_sp_s_temp_dn4)), ((((var_sp_s_x0_dn6 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn6)) * var_sp_s_temp) + (assign42390_e55724 * var_sp_s_temp_dn6)), ((((var_sp_s_x0_dn7 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn7)) * var_sp_s_temp) + (assign42390_e55724 * var_sp_s_temp_dn7)), ((((var_sp_s_x0_dn8 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn8)) * var_sp_s_temp) + (assign42390_e55724 * var_sp_s_temp_dn8)), ((((var_sp_s_x0_dn9 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn9)) * var_sp_s_temp) + (assign42390_e55724 * var_sp_s_temp_dn9)),)
    } else {
        (var_sp_s_xi0, var_sp_s_xi0_dn4, var_sp_s_xi0_dn6, var_sp_s_xi0_dn7, var_sp_s_xi0_dn8, var_sp_s_xi0_dn9,)
    }
};
        var_sp_s_xi0 = assign42390_e55728;
        var_sp_s_xi0_dn4 = assign42390_e55728_d_n4;
        var_sp_s_xi0_dn6 = assign42390_e55728_d_n6;
        var_sp_s_xi0_dn7 = assign42390_e55728_d_n7;
        var_sp_s_xi0_dn8 = assign42390_e55728_d_n8;
        var_sp_s_xi0_dn9 = assign42390_e55728_d_n9;

        let (assign42400_e55742, assign42400_e55742_d_n4, assign42400_e55742_d_n6, assign42400_e55742_d_n7, assign42400_e55742_d_n8, assign42400_e55742_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42400_e55737: f64 = (var_sp_s_x0 * var_sp_s_temp);
        let assign42400_e55739: f64 = (assign42400_e55737 * var_sp_s_temp);
        let assign42400_e55740: f64 = (4.0 * assign42400_e55739);
        (assign42400_e55740, (4.0 * ((((var_sp_s_x0_dn4 * var_sp_s_temp) + (var_sp_s_x0 * var_sp_s_temp_dn4)) * var_sp_s_temp) + (assign42400_e55737 * var_sp_s_temp_dn4))), (4.0 * ((((var_sp_s_x0_dn6 * var_sp_s_temp) + (var_sp_s_x0 * var_sp_s_temp_dn6)) * var_sp_s_temp) + (assign42400_e55737 * var_sp_s_temp_dn6))), (4.0 * ((((var_sp_s_x0_dn7 * var_sp_s_temp) + (var_sp_s_x0 * var_sp_s_temp_dn7)) * var_sp_s_temp) + (assign42400_e55737 * var_sp_s_temp_dn7))), (4.0 * ((((var_sp_s_x0_dn8 * var_sp_s_temp) + (var_sp_s_x0 * var_sp_s_temp_dn8)) * var_sp_s_temp) + (assign42400_e55737 * var_sp_s_temp_dn8))), (4.0 * ((((var_sp_s_x0_dn9 * var_sp_s_temp) + (var_sp_s_x0 * var_sp_s_temp_dn9)) * var_sp_s_temp) + (assign42400_e55737 * var_sp_s_temp_dn9))),)
    } else {
        (var_sp_s_xi1, var_sp_s_xi1_dn4, var_sp_s_xi1_dn6, var_sp_s_xi1_dn7, var_sp_s_xi1_dn8, var_sp_s_xi1_dn9,)
    }
};
        var_sp_s_xi1 = assign42400_e55742;
        var_sp_s_xi1_dn4 = assign42400_e55742_d_n4;
        var_sp_s_xi1_dn6 = assign42400_e55742_d_n6;
        var_sp_s_xi1_dn7 = assign42400_e55742_d_n7;
        var_sp_s_xi1_dn8 = assign42400_e55742_d_n8;
        var_sp_s_xi1_dn9 = assign42400_e55742_d_n9;

        let (assign42410_e55760, assign42410_e55760_d_n4, assign42410_e55760_d_n6, assign42410_e55760_d_n7, assign42410_e55760_d_n8, assign42410_e55760_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42410_e55750: f64 = (8.0 * var_sp_s_temp);
        let assign42410_e55753: f64 = (12.0 * var_sp_s_xi0);
        let assign42410_e55754: f64 = (assign42410_e55750 - assign42410_e55753);
        let assign42410_e55756: f64 = (assign42410_e55754 * var_sp_s_temp);
        let assign42410_e55758: f64 = (assign42410_e55756 * var_sp_s_temp);
        (assign42410_e55758, ((((((8.0 * var_sp_s_temp_dn4) - (12.0 * var_sp_s_xi0_dn4)) * var_sp_s_temp) + (assign42410_e55754 * var_sp_s_temp_dn4)) * var_sp_s_temp) + (assign42410_e55756 * var_sp_s_temp_dn4)), ((((((8.0 * var_sp_s_temp_dn6) - (12.0 * var_sp_s_xi0_dn6)) * var_sp_s_temp) + (assign42410_e55754 * var_sp_s_temp_dn6)) * var_sp_s_temp) + (assign42410_e55756 * var_sp_s_temp_dn6)), ((((((8.0 * var_sp_s_temp_dn7) - (12.0 * var_sp_s_xi0_dn7)) * var_sp_s_temp) + (assign42410_e55754 * var_sp_s_temp_dn7)) * var_sp_s_temp) + (assign42410_e55756 * var_sp_s_temp_dn7)), ((((((8.0 * var_sp_s_temp_dn8) - (12.0 * var_sp_s_xi0_dn8)) * var_sp_s_temp) + (assign42410_e55754 * var_sp_s_temp_dn8)) * var_sp_s_temp) + (assign42410_e55756 * var_sp_s_temp_dn8)), ((((((8.0 * var_sp_s_temp_dn9) - (12.0 * var_sp_s_xi0_dn9)) * var_sp_s_temp) + (assign42410_e55754 * var_sp_s_temp_dn9)) * var_sp_s_temp) + (assign42410_e55756 * var_sp_s_temp_dn9)),)
    } else {
        (var_sp_s_xi2, var_sp_s_xi2_dn4, var_sp_s_xi2_dn6, var_sp_s_xi2_dn7, var_sp_s_xi2_dn8, var_sp_s_xi2_dn9,)
    }
};
        var_sp_s_xi2 = assign42410_e55760;
        var_sp_s_xi2_dn4 = assign42410_e55760_d_n4;
        var_sp_s_xi2_dn6 = assign42410_e55760_d_n6;
        var_sp_s_xi2_dn7 = assign42410_e55760_d_n7;
        var_sp_s_xi2_dn8 = assign42410_e55760_d_n8;
        var_sp_s_xi2_dn9 = assign42410_e55760_d_n9;

        let (assign42420_e55770, assign42420_e55770_d_n4, assign42420_e55770_d_n6, assign42420_e55770_d_n7, assign42420_e55770_d_n8, assign42420_e55770_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42420_e55768: f64 = (var_xg - var_sp_s_x0);
        (assign42420_e55768, (var_xg_dn4 - var_sp_s_x0_dn4), (var_xg_dn6 - var_sp_s_x0_dn6), (var_xg_dn7 - var_sp_s_x0_dn7), (var_xg_dn8 - var_sp_s_x0_dn8), (var_xg_dn9 - var_sp_s_x0_dn9),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn4, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8, var_sp_s_temp_dn9,)
    }
};
        var_sp_s_temp = assign42420_e55770;
        var_sp_s_temp_dn4 = assign42420_e55770_d_n4;
        var_sp_s_temp_dn6 = assign42420_e55770_d_n6;
        var_sp_s_temp_dn7 = assign42420_e55770_d_n7;
        var_sp_s_temp_dn8 = assign42420_e55770_d_n8;
        var_sp_s_temp_dn9 = assign42420_e55770_d_n9;

        let (assign42430_e55794, assign42430_e55794_d_n4, assign42430_e55794_d_n6, assign42430_e55794_d_n7, assign42430_e55794_d_n8, assign42430_e55794_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42430_e55778: f64 = (2.0 * var_sp_s_temp);
        let assign42430_e55782: f64 = (1.0 - var_sp_s_delta1);
        let assign42430_e55784: f64 = (assign42430_e55782 + var_sp_s_delta0);
        let assign42430_e55788: f64 = (1.0 + var_sp_s_xi1);
        let assign42430_e55789: f64 = (var_delta_ns * assign42430_e55788);
        let assign42430_e55790: f64 = (assign42430_e55784 - assign42430_e55789);
        let assign42430_e55791: f64 = (var_gf2 * assign42430_e55790);
        let assign42430_e55792: f64 = (assign42430_e55778 + assign42430_e55791);
        (assign42430_e55792, ((2.0 * var_sp_s_temp_dn4) + ((var_gf2_dn4 * assign42430_e55790) + (var_gf2 * (((-var_sp_s_delta1_dn4) + var_sp_s_delta0_dn4) - ((var_delta_ns_dn4 * assign42430_e55788) + (var_delta_ns * var_sp_s_xi1_dn4)))))), ((2.0 * var_sp_s_temp_dn6) + ((var_gf2_dn6 * assign42430_e55790) + (var_gf2 * (((-var_sp_s_delta1_dn6) + var_sp_s_delta0_dn6) - ((var_delta_ns_dn6 * assign42430_e55788) + (var_delta_ns * var_sp_s_xi1_dn6)))))), ((2.0 * var_sp_s_temp_dn7) + ((var_gf2_dn7 * assign42430_e55790) + (var_gf2 * (((-var_sp_s_delta1_dn7) + var_sp_s_delta0_dn7) - ((var_delta_ns_dn7 * assign42430_e55788) + (var_delta_ns * var_sp_s_xi1_dn7)))))), ((2.0 * var_sp_s_temp_dn8) + ((var_gf2_dn8 * assign42430_e55790) + (var_gf2 * (((-var_sp_s_delta1_dn8) + var_sp_s_delta0_dn8) - ((var_delta_ns_dn8 * assign42430_e55788) + (var_delta_ns * var_sp_s_xi1_dn8)))))), ((2.0 * var_sp_s_temp_dn9) + ((var_gf2_dn9 * assign42430_e55790) + (var_gf2 * (((-var_sp_s_delta1_dn9) + var_sp_s_delta0_dn9) - ((var_delta_ns_dn9 * assign42430_e55788) + (var_delta_ns * var_sp_s_xi1_dn9)))))),)
    } else {
        (var_sp_s_pc, var_sp_s_pc_dn4, var_sp_s_pc_dn6, var_sp_s_pc_dn7, var_sp_s_pc_dn8, var_sp_s_pc_dn9,)
    }
};
        var_sp_s_pc = assign42430_e55794;
        var_sp_s_pc_dn4 = assign42430_e55794_d_n4;
        var_sp_s_pc_dn6 = assign42430_e55794_d_n6;
        var_sp_s_pc_dn7 = assign42430_e55794_d_n7;
        var_sp_s_pc_dn8 = assign42430_e55794_d_n8;
        var_sp_s_pc_dn9 = assign42430_e55794_d_n9;

        let (assign42440_e55822, assign42440_e55822_d_n4, assign42440_e55822_d_n6, assign42440_e55822_d_n7, assign42440_e55822_d_n8, assign42440_e55822_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42440_e55802: f64 = (var_sp_s_temp * var_sp_s_temp);
        let assign42440_e55806: f64 = (var_sp_s_delta1 + var_sp_s_x0);
        let assign42440_e55808: f64 = (assign42440_e55806 - 1.0);
        let assign42440_e55810: f64 = (assign42440_e55808 + var_sp_s_delta0);
        let assign42440_e55814: f64 = (var_sp_s_x0 + 1.0);
        let assign42440_e55816: f64 = (assign42440_e55814 + var_sp_s_xi0);
        let assign42440_e55817: f64 = (var_delta_ns * assign42440_e55816);
        let assign42440_e55818: f64 = (assign42440_e55810 - assign42440_e55817);
        let assign42440_e55819: f64 = (var_gf2 * assign42440_e55818);
        let assign42440_e55820: f64 = (assign42440_e55802 - assign42440_e55819);
        (assign42440_e55820, (((var_sp_s_temp_dn4 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn4)) - ((var_gf2_dn4 * assign42440_e55818) + (var_gf2 * (((var_sp_s_delta1_dn4 + var_sp_s_x0_dn4) + var_sp_s_delta0_dn4) - ((var_delta_ns_dn4 * assign42440_e55816) + (var_delta_ns * (var_sp_s_x0_dn4 + var_sp_s_xi0_dn4))))))), (((var_sp_s_temp_dn6 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn6)) - ((var_gf2_dn6 * assign42440_e55818) + (var_gf2 * (((var_sp_s_delta1_dn6 + var_sp_s_x0_dn6) + var_sp_s_delta0_dn6) - ((var_delta_ns_dn6 * assign42440_e55816) + (var_delta_ns * (var_sp_s_x0_dn6 + var_sp_s_xi0_dn6))))))), (((var_sp_s_temp_dn7 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn7)) - ((var_gf2_dn7 * assign42440_e55818) + (var_gf2 * (((var_sp_s_delta1_dn7 + var_sp_s_x0_dn7) + var_sp_s_delta0_dn7) - ((var_delta_ns_dn7 * assign42440_e55816) + (var_delta_ns * (var_sp_s_x0_dn7 + var_sp_s_xi0_dn7))))))), (((var_sp_s_temp_dn8 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn8)) - ((var_gf2_dn8 * assign42440_e55818) + (var_gf2 * (((var_sp_s_delta1_dn8 + var_sp_s_x0_dn8) + var_sp_s_delta0_dn8) - ((var_delta_ns_dn8 * assign42440_e55816) + (var_delta_ns * (var_sp_s_x0_dn8 + var_sp_s_xi0_dn8))))))), (((var_sp_s_temp_dn9 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn9)) - ((var_gf2_dn9 * assign42440_e55818) + (var_gf2 * (((var_sp_s_delta1_dn9 + var_sp_s_x0_dn9) + var_sp_s_delta0_dn9) - ((var_delta_ns_dn9 * assign42440_e55816) + (var_delta_ns * (var_sp_s_x0_dn9 + var_sp_s_xi0_dn9))))))),)
    } else {
        (var_sp_s_qc, var_sp_s_qc_dn4, var_sp_s_qc_dn6, var_sp_s_qc_dn7, var_sp_s_qc_dn8, var_sp_s_qc_dn9,)
    }
};
        var_sp_s_qc = assign42440_e55822;
        var_sp_s_qc_dn4 = assign42440_e55822_d_n4;
        var_sp_s_qc_dn6 = assign42440_e55822_d_n6;
        var_sp_s_qc_dn7 = assign42440_e55822_d_n7;
        var_sp_s_qc_dn8 = assign42440_e55822_d_n8;
        var_sp_s_qc_dn9 = assign42440_e55822_d_n9;

        *var_guard1203_slot = var_guard1203;
        *var_guard1204_slot = var_guard1204;
        *var_mutau_slot = var_mutau;
        *var_mutau_dn4_slot = var_mutau_dn4;
        *var_mutau_dn6_slot = var_mutau_dn6;
        *var_mutau_dn7_slot = var_mutau_dn7;
        *var_mutau_dn8_slot = var_mutau_dn8;
        *var_mutau_dn9_slot = var_mutau_dn9;
        *var_nu_slot = var_nu;
        *var_nu_dn4_slot = var_nu_dn4;
        *var_nu_dn6_slot = var_nu_dn6;
        *var_nu_dn7_slot = var_nu_dn7;
        *var_nu_dn8_slot = var_nu_dn8;
        *var_nu_dn9_slot = var_nu_dn9;
        *var_sp_s_a_slot = var_sp_s_a;
        *var_sp_s_a_dn4_slot = var_sp_s_a_dn4;
        *var_sp_s_a_dn6_slot = var_sp_s_a_dn6;
        *var_sp_s_a_dn7_slot = var_sp_s_a_dn7;
        *var_sp_s_a_dn8_slot = var_sp_s_a_dn8;
        *var_sp_s_a_dn9_slot = var_sp_s_a_dn9;
        *var_sp_s_b_slot = var_sp_s_b;
        *var_sp_s_b_dn4_slot = var_sp_s_b_dn4;
        *var_sp_s_b_dn6_slot = var_sp_s_b_dn6;
        *var_sp_s_b_dn7_slot = var_sp_s_b_dn7;
        *var_sp_s_b_dn8_slot = var_sp_s_b_dn8;
        *var_sp_s_b_dn9_slot = var_sp_s_b_dn9;
        *var_sp_s_c_slot = var_sp_s_c;
        *var_sp_s_c_dn4_slot = var_sp_s_c_dn4;
        *var_sp_s_c_dn6_slot = var_sp_s_c_dn6;
        *var_sp_s_c_dn7_slot = var_sp_s_c_dn7;
        *var_sp_s_c_dn8_slot = var_sp_s_c_dn8;
        *var_sp_s_c_dn9_slot = var_sp_s_c_dn9;
        *var_sp_s_delta0_slot = var_sp_s_delta0;
        *var_sp_s_delta0_dn4_slot = var_sp_s_delta0_dn4;
        *var_sp_s_delta0_dn6_slot = var_sp_s_delta0_dn6;
        *var_sp_s_delta0_dn7_slot = var_sp_s_delta0_dn7;
        *var_sp_s_delta0_dn8_slot = var_sp_s_delta0_dn8;
        *var_sp_s_delta0_dn9_slot = var_sp_s_delta0_dn9;
        *var_sp_s_delta1_slot = var_sp_s_delta1;
        *var_sp_s_delta1_dn4_slot = var_sp_s_delta1_dn4;
        *var_sp_s_delta1_dn6_slot = var_sp_s_delta1_dn6;
        *var_sp_s_delta1_dn7_slot = var_sp_s_delta1_dn7;
        *var_sp_s_delta1_dn8_slot = var_sp_s_delta1_dn8;
        *var_sp_s_delta1_dn9_slot = var_sp_s_delta1_dn9;
        *var_sp_s_pc_slot = var_sp_s_pc;
        *var_sp_s_pc_dn4_slot = var_sp_s_pc_dn4;
        *var_sp_s_pc_dn6_slot = var_sp_s_pc_dn6;
        *var_sp_s_pc_dn7_slot = var_sp_s_pc_dn7;
        *var_sp_s_pc_dn8_slot = var_sp_s_pc_dn8;
        *var_sp_s_pc_dn9_slot = var_sp_s_pc_dn9;
        *var_sp_s_qc_slot = var_sp_s_qc;
        *var_sp_s_qc_dn4_slot = var_sp_s_qc_dn4;
        *var_sp_s_qc_dn6_slot = var_sp_s_qc_dn6;
        *var_sp_s_qc_dn7_slot = var_sp_s_qc_dn7;
        *var_sp_s_qc_dn8_slot = var_sp_s_qc_dn8;
        *var_sp_s_qc_dn9_slot = var_sp_s_qc_dn9;
        *var_sp_s_tau_slot = var_sp_s_tau;
        *var_sp_s_tau_dn4_slot = var_sp_s_tau_dn4;
        *var_sp_s_tau_dn6_slot = var_sp_s_tau_dn6;
        *var_sp_s_tau_dn7_slot = var_sp_s_tau_dn7;
        *var_sp_s_tau_dn8_slot = var_sp_s_tau_dn8;
        *var_sp_s_tau_dn9_slot = var_sp_s_tau_dn9;
        *var_sp_s_temp_slot = var_sp_s_temp;
        *var_sp_s_temp2_slot = var_sp_s_temp2;
        *var_sp_s_temp2_dn4_slot = var_sp_s_temp2_dn4;
        *var_sp_s_temp2_dn6_slot = var_sp_s_temp2_dn6;
        *var_sp_s_temp2_dn7_slot = var_sp_s_temp2_dn7;
        *var_sp_s_temp2_dn8_slot = var_sp_s_temp2_dn8;
        *var_sp_s_temp2_dn9_slot = var_sp_s_temp2_dn9;
        *var_sp_s_temp_dn4_slot = var_sp_s_temp_dn4;
        *var_sp_s_temp_dn6_slot = var_sp_s_temp_dn6;
        *var_sp_s_temp_dn7_slot = var_sp_s_temp_dn7;
        *var_sp_s_temp_dn8_slot = var_sp_s_temp_dn8;
        *var_sp_s_temp_dn9_slot = var_sp_s_temp_dn9;
        *var_sp_s_x0_slot = var_sp_s_x0;
        *var_sp_s_x0_dn4_slot = var_sp_s_x0_dn4;
        *var_sp_s_x0_dn6_slot = var_sp_s_x0_dn6;
        *var_sp_s_x0_dn7_slot = var_sp_s_x0_dn7;
        *var_sp_s_x0_dn8_slot = var_sp_s_x0_dn8;
        *var_sp_s_x0_dn9_slot = var_sp_s_x0_dn9;
        *var_sp_s_xi0_slot = var_sp_s_xi0;
        *var_sp_s_xi0_dn4_slot = var_sp_s_xi0_dn4;
        *var_sp_s_xi0_dn6_slot = var_sp_s_xi0_dn6;
        *var_sp_s_xi0_dn7_slot = var_sp_s_xi0_dn7;
        *var_sp_s_xi0_dn8_slot = var_sp_s_xi0_dn8;
        *var_sp_s_xi0_dn9_slot = var_sp_s_xi0_dn9;
        *var_sp_s_xi1_slot = var_sp_s_xi1;
        *var_sp_s_xi1_dn4_slot = var_sp_s_xi1_dn4;
        *var_sp_s_xi1_dn6_slot = var_sp_s_xi1_dn6;
        *var_sp_s_xi1_dn7_slot = var_sp_s_xi1_dn7;
        *var_sp_s_xi1_dn8_slot = var_sp_s_xi1_dn8;
        *var_sp_s_xi1_dn9_slot = var_sp_s_xi1_dn9;
        *var_sp_s_xi2_slot = var_sp_s_xi2;
        *var_sp_s_xi2_dn4_slot = var_sp_s_xi2_dn4;
        *var_sp_s_xi2_dn6_slot = var_sp_s_xi2_dn6;
        *var_sp_s_xi2_dn7_slot = var_sp_s_xi2_dn7;
        *var_sp_s_xi2_dn8_slot = var_sp_s_xi2_dn8;
        *var_sp_s_xi2_dn9_slot = var_sp_s_xi2_dn9;
    }

    pub(super) fn stamp_transient_block_92(
        var_delta_ns: f64,
        var_delta_ns_dn4: f64,
        var_delta_ns_dn6: f64,
        var_delta_ns_dn7: f64,
        var_delta_ns_dn8: f64,
        var_delta_ns_dn9: f64,
        var_gf2: f64,
        var_gf2_dn4: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf2_dn9: f64,
        var_guard1199: f64,
        var_guard1200: f64,
        var_phit1: f64,
        var_phit1_dn4: f64,
        var_phit1_dn6: f64,
        var_phit1_dn7: f64,
        var_phit1_dn8: f64,
        var_phit1_dn9: f64,
        var_sp_s_delta0: f64,
        var_sp_s_delta0_dn4: f64,
        var_sp_s_delta0_dn6: f64,
        var_sp_s_delta0_dn7: f64,
        var_sp_s_delta0_dn8: f64,
        var_sp_s_delta0_dn9: f64,
        var_sp_s_delta1: f64,
        var_sp_s_delta1_dn4: f64,
        var_sp_s_delta1_dn6: f64,
        var_sp_s_delta1_dn7: f64,
        var_sp_s_delta1_dn8: f64,
        var_sp_s_delta1_dn9: f64,
        var_sp_s_pc: f64,
        var_sp_s_pc_dn4: f64,
        var_sp_s_pc_dn6: f64,
        var_sp_s_pc_dn7: f64,
        var_sp_s_pc_dn8: f64,
        var_sp_s_pc_dn9: f64,
        var_sp_s_qc: f64,
        var_sp_s_qc_dn4: f64,
        var_sp_s_qc_dn6: f64,
        var_sp_s_qc_dn7: f64,
        var_sp_s_qc_dn8: f64,
        var_sp_s_qc_dn9: f64,
        var_sp_s_x0: f64,
        var_sp_s_x0_dn4: f64,
        var_sp_s_x0_dn6: f64,
        var_sp_s_x0_dn7: f64,
        var_sp_s_x0_dn8: f64,
        var_sp_s_x0_dn9: f64,
        var_sp_s_xi2: f64,
        var_sp_s_xi2_dn4: f64,
        var_sp_s_xi2_dn6: f64,
        var_sp_s_xi2_dn7: f64,
        var_sp_s_xi2_dn8: f64,
        var_sp_s_xi2_dn9: f64,
        var_xg: f64,
        var_xg_dn4: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xg_dn9: f64,
        var_xn_s: f64,
        var_xn_s_dn4: f64,
        var_xn_s_dn6: f64,
        var_xn_s_dn7: f64,
        var_xn_s_dn8: f64,
        var_xn_s_dn9: f64,
        var_alphas_slot: &mut f64,
        var_alphas_dn4_slot: &mut f64,
        var_alphas_dn6_slot: &mut f64,
        var_alphas_dn7_slot: &mut f64,
        var_alphas_dn8_slot: &mut f64,
        var_alphas_dn9_slot: &mut f64,
        var_delta_1s_slot: &mut f64,
        var_delta_1s_dn4_slot: &mut f64,
        var_delta_1s_dn6_slot: &mut f64,
        var_delta_1s_dn7_slot: &mut f64,
        var_delta_1s_dn8_slot: &mut f64,
        var_delta_1s_dn9_slot: &mut f64,
        var_ds_slot: &mut f64,
        var_ds_dn4_slot: &mut f64,
        var_ds_dn6_slot: &mut f64,
        var_ds_dn7_slot: &mut f64,
        var_ds_dn8_slot: &mut f64,
        var_ds_dn9_slot: &mut f64,
        var_es_slot: &mut f64,
        var_es_dn4_slot: &mut f64,
        var_es_dn6_slot: &mut f64,
        var_es_dn7_slot: &mut f64,
        var_es_dn8_slot: &mut f64,
        var_es_dn9_slot: &mut f64,
        var_factheta_slot: &mut f64,
        var_factheta_dn4_slot: &mut f64,
        var_factheta_dn6_slot: &mut f64,
        var_factheta_dn7_slot: &mut f64,
        var_factheta_dn8_slot: &mut f64,
        var_factheta_dn9_slot: &mut f64,
        var_gmobs_slot: &mut f64,
        var_gmobs_dn4_slot: &mut f64,
        var_gmobs_dn6_slot: &mut f64,
        var_gmobs_dn7_slot: &mut f64,
        var_gmobs_dn8_slot: &mut f64,
        var_gmobs_dn9_slot: &mut f64,
        var_guard1205_slot: &mut f64,
        var_guard1206_slot: &mut f64,
        var_guard1207_slot: &mut f64,
        var_guard1208_slot: &mut f64,
        var_ps_slot: &mut f64,
        var_ps_dn4_slot: &mut f64,
        var_ps_dn6_slot: &mut f64,
        var_ps_dn7_slot: &mut f64,
        var_ps_dn8_slot: &mut f64,
        var_ps_dn9_slot: &mut f64,
        var_qbs_slot: &mut f64,
        var_qbs_dn4_slot: &mut f64,
        var_qbs_dn6_slot: &mut f64,
        var_qbs_dn7_slot: &mut f64,
        var_qbs_dn8_slot: &mut f64,
        var_qbs_dn9_slot: &mut f64,
        var_qis_slot: &mut f64,
        var_qis_dn4_slot: &mut f64,
        var_qis_dn6_slot: &mut f64,
        var_qis_dn7_slot: &mut f64,
        var_qis_dn8_slot: &mut f64,
        var_qis_dn9_slot: &mut f64,
        var_rhob_slot: &mut f64,
        var_rhob_dn4_slot: &mut f64,
        var_rhob_dn6_slot: &mut f64,
        var_rhob_dn7_slot: &mut f64,
        var_rhob_dn8_slot: &mut f64,
        var_rhob_dn9_slot: &mut f64,
        var_rhog_slot: &mut f64,
        var_rhog_dn4_slot: &mut f64,
        var_rhog_dn6_slot: &mut f64,
        var_rhog_dn7_slot: &mut f64,
        var_rhog_dn8_slot: &mut f64,
        var_rhog_dn9_slot: &mut f64,
        var_rxcor_slot: &mut f64,
        var_rxcor_dn4_slot: &mut f64,
        var_rxcor_dn6_slot: &mut f64,
        var_rxcor_dn7_slot: &mut f64,
        var_rxcor_dn8_slot: &mut f64,
        var_rxcor_dn9_slot: &mut f64,
        var_sp_s_temp_slot: &mut f64,
        var_sp_s_temp_dn4_slot: &mut f64,
        var_sp_s_temp_dn6_slot: &mut f64,
        var_sp_s_temp_dn7_slot: &mut f64,
        var_sp_s_temp_dn8_slot: &mut f64,
        var_sp_s_temp_dn9_slot: &mut f64,
        var_sqs_slot: &mut f64,
        var_sqs_dn4_slot: &mut f64,
        var_sqs_dn6_slot: &mut f64,
        var_sqs_dn7_slot: &mut f64,
        var_sqs_dn8_slot: &mut f64,
        var_sqs_dn9_slot: &mut f64,
        var_temp__blk949_slot: &mut f64,
        var_temp__blk949_dn4_slot: &mut f64,
        var_temp__blk949_dn6_slot: &mut f64,
        var_temp__blk949_dn7_slot: &mut f64,
        var_temp__blk949_dn8_slot: &mut f64,
        var_temp__blk949_dn9_slot: &mut f64,
        var_x_s_slot: &mut f64,
        var_x_s_dn4_slot: &mut f64,
        var_x_s_dn6_slot: &mut f64,
        var_x_s_dn7_slot: &mut f64,
        var_x_s_dn8_slot: &mut f64,
        var_x_s_dn9_slot: &mut f64,
        var_xgs_slot: &mut f64,
        var_xgs_dn4_slot: &mut f64,
        var_xgs_dn6_slot: &mut f64,
        var_xgs_dn7_slot: &mut f64,
        var_xgs_dn8_slot: &mut f64,
        var_xgs_dn9_slot: &mut f64,
        var_xi0s_slot: &mut f64,
        var_xi0s_dn4_slot: &mut f64,
        var_xi0s_dn6_slot: &mut f64,
        var_xi0s_dn7_slot: &mut f64,
        var_xi0s_dn8_slot: &mut f64,
        var_xi0s_dn9_slot: &mut f64,
        var_xi1s_slot: &mut f64,
        var_xi1s_dn4_slot: &mut f64,
        var_xi1s_dn6_slot: &mut f64,
        var_xi1s_dn7_slot: &mut f64,
        var_xi1s_dn8_slot: &mut f64,
        var_xi1s_dn9_slot: &mut f64,
        var_xi2s_slot: &mut f64,
        var_xi2s_dn4_slot: &mut f64,
        var_xi2s_dn6_slot: &mut f64,
        var_xi2s_dn7_slot: &mut f64,
        var_xi2s_dn8_slot: &mut f64,
        var_xi2s_dn9_slot: &mut f64,
        var_xitsb_slot: &mut f64,
        var_xitsb_dn4_slot: &mut f64,
        var_xitsb_dn6_slot: &mut f64,
        var_xitsb_dn7_slot: &mut f64,
        var_xitsb_dn8_slot: &mut f64,
        var_xitsb_dn9_slot: &mut f64,
    ) {
        let mut var_alphas: f64 = *var_alphas_slot;
        let mut var_alphas_dn4: f64 = *var_alphas_dn4_slot;
        let mut var_alphas_dn6: f64 = *var_alphas_dn6_slot;
        let mut var_alphas_dn7: f64 = *var_alphas_dn7_slot;
        let mut var_alphas_dn8: f64 = *var_alphas_dn8_slot;
        let mut var_alphas_dn9: f64 = *var_alphas_dn9_slot;
        let mut var_delta_1s: f64 = *var_delta_1s_slot;
        let mut var_delta_1s_dn4: f64 = *var_delta_1s_dn4_slot;
        let mut var_delta_1s_dn6: f64 = *var_delta_1s_dn6_slot;
        let mut var_delta_1s_dn7: f64 = *var_delta_1s_dn7_slot;
        let mut var_delta_1s_dn8: f64 = *var_delta_1s_dn8_slot;
        let mut var_delta_1s_dn9: f64 = *var_delta_1s_dn9_slot;
        let mut var_ds: f64 = *var_ds_slot;
        let mut var_ds_dn4: f64 = *var_ds_dn4_slot;
        let mut var_ds_dn6: f64 = *var_ds_dn6_slot;
        let mut var_ds_dn7: f64 = *var_ds_dn7_slot;
        let mut var_ds_dn8: f64 = *var_ds_dn8_slot;
        let mut var_ds_dn9: f64 = *var_ds_dn9_slot;
        let mut var_es: f64 = *var_es_slot;
        let mut var_es_dn4: f64 = *var_es_dn4_slot;
        let mut var_es_dn6: f64 = *var_es_dn6_slot;
        let mut var_es_dn7: f64 = *var_es_dn7_slot;
        let mut var_es_dn8: f64 = *var_es_dn8_slot;
        let mut var_es_dn9: f64 = *var_es_dn9_slot;
        let mut var_factheta: f64 = *var_factheta_slot;
        let mut var_factheta_dn4: f64 = *var_factheta_dn4_slot;
        let mut var_factheta_dn6: f64 = *var_factheta_dn6_slot;
        let mut var_factheta_dn7: f64 = *var_factheta_dn7_slot;
        let mut var_factheta_dn8: f64 = *var_factheta_dn8_slot;
        let mut var_factheta_dn9: f64 = *var_factheta_dn9_slot;
        let mut var_gmobs: f64 = *var_gmobs_slot;
        let mut var_gmobs_dn4: f64 = *var_gmobs_dn4_slot;
        let mut var_gmobs_dn6: f64 = *var_gmobs_dn6_slot;
        let mut var_gmobs_dn7: f64 = *var_gmobs_dn7_slot;
        let mut var_gmobs_dn8: f64 = *var_gmobs_dn8_slot;
        let mut var_gmobs_dn9: f64 = *var_gmobs_dn9_slot;
        let mut var_guard1205: f64 = *var_guard1205_slot;
        let mut var_guard1206: f64 = *var_guard1206_slot;
        let mut var_guard1207: f64 = *var_guard1207_slot;
        let mut var_guard1208: f64 = *var_guard1208_slot;
        let mut var_ps: f64 = *var_ps_slot;
        let mut var_ps_dn4: f64 = *var_ps_dn4_slot;
        let mut var_ps_dn6: f64 = *var_ps_dn6_slot;
        let mut var_ps_dn7: f64 = *var_ps_dn7_slot;
        let mut var_ps_dn8: f64 = *var_ps_dn8_slot;
        let mut var_ps_dn9: f64 = *var_ps_dn9_slot;
        let mut var_qbs: f64 = *var_qbs_slot;
        let mut var_qbs_dn4: f64 = *var_qbs_dn4_slot;
        let mut var_qbs_dn6: f64 = *var_qbs_dn6_slot;
        let mut var_qbs_dn7: f64 = *var_qbs_dn7_slot;
        let mut var_qbs_dn8: f64 = *var_qbs_dn8_slot;
        let mut var_qbs_dn9: f64 = *var_qbs_dn9_slot;
        let mut var_qis: f64 = *var_qis_slot;
        let mut var_qis_dn4: f64 = *var_qis_dn4_slot;
        let mut var_qis_dn6: f64 = *var_qis_dn6_slot;
        let mut var_qis_dn7: f64 = *var_qis_dn7_slot;
        let mut var_qis_dn8: f64 = *var_qis_dn8_slot;
        let mut var_qis_dn9: f64 = *var_qis_dn9_slot;
        let mut var_rhob: f64 = *var_rhob_slot;
        let mut var_rhob_dn4: f64 = *var_rhob_dn4_slot;
        let mut var_rhob_dn6: f64 = *var_rhob_dn6_slot;
        let mut var_rhob_dn7: f64 = *var_rhob_dn7_slot;
        let mut var_rhob_dn8: f64 = *var_rhob_dn8_slot;
        let mut var_rhob_dn9: f64 = *var_rhob_dn9_slot;
        let mut var_rhog: f64 = *var_rhog_slot;
        let mut var_rhog_dn4: f64 = *var_rhog_dn4_slot;
        let mut var_rhog_dn6: f64 = *var_rhog_dn6_slot;
        let mut var_rhog_dn7: f64 = *var_rhog_dn7_slot;
        let mut var_rhog_dn8: f64 = *var_rhog_dn8_slot;
        let mut var_rhog_dn9: f64 = *var_rhog_dn9_slot;
        let mut var_rxcor: f64 = *var_rxcor_slot;
        let mut var_rxcor_dn4: f64 = *var_rxcor_dn4_slot;
        let mut var_rxcor_dn6: f64 = *var_rxcor_dn6_slot;
        let mut var_rxcor_dn7: f64 = *var_rxcor_dn7_slot;
        let mut var_rxcor_dn8: f64 = *var_rxcor_dn8_slot;
        let mut var_rxcor_dn9: f64 = *var_rxcor_dn9_slot;
        let mut var_sp_s_temp: f64 = *var_sp_s_temp_slot;
        let mut var_sp_s_temp_dn4: f64 = *var_sp_s_temp_dn4_slot;
        let mut var_sp_s_temp_dn6: f64 = *var_sp_s_temp_dn6_slot;
        let mut var_sp_s_temp_dn7: f64 = *var_sp_s_temp_dn7_slot;
        let mut var_sp_s_temp_dn8: f64 = *var_sp_s_temp_dn8_slot;
        let mut var_sp_s_temp_dn9: f64 = *var_sp_s_temp_dn9_slot;
        let mut var_sqs: f64 = *var_sqs_slot;
        let mut var_sqs_dn4: f64 = *var_sqs_dn4_slot;
        let mut var_sqs_dn6: f64 = *var_sqs_dn6_slot;
        let mut var_sqs_dn7: f64 = *var_sqs_dn7_slot;
        let mut var_sqs_dn8: f64 = *var_sqs_dn8_slot;
        let mut var_sqs_dn9: f64 = *var_sqs_dn9_slot;
        let mut var_temp__blk949: f64 = *var_temp__blk949_slot;
        let mut var_temp__blk949_dn4: f64 = *var_temp__blk949_dn4_slot;
        let mut var_temp__blk949_dn6: f64 = *var_temp__blk949_dn6_slot;
        let mut var_temp__blk949_dn7: f64 = *var_temp__blk949_dn7_slot;
        let mut var_temp__blk949_dn8: f64 = *var_temp__blk949_dn8_slot;
        let mut var_temp__blk949_dn9: f64 = *var_temp__blk949_dn9_slot;
        let mut var_x_s: f64 = *var_x_s_slot;
        let mut var_x_s_dn4: f64 = *var_x_s_dn4_slot;
        let mut var_x_s_dn6: f64 = *var_x_s_dn6_slot;
        let mut var_x_s_dn7: f64 = *var_x_s_dn7_slot;
        let mut var_x_s_dn8: f64 = *var_x_s_dn8_slot;
        let mut var_x_s_dn9: f64 = *var_x_s_dn9_slot;
        let mut var_xgs: f64 = *var_xgs_slot;
        let mut var_xgs_dn4: f64 = *var_xgs_dn4_slot;
        let mut var_xgs_dn6: f64 = *var_xgs_dn6_slot;
        let mut var_xgs_dn7: f64 = *var_xgs_dn7_slot;
        let mut var_xgs_dn8: f64 = *var_xgs_dn8_slot;
        let mut var_xgs_dn9: f64 = *var_xgs_dn9_slot;
        let mut var_xi0s: f64 = *var_xi0s_slot;
        let mut var_xi0s_dn4: f64 = *var_xi0s_dn4_slot;
        let mut var_xi0s_dn6: f64 = *var_xi0s_dn6_slot;
        let mut var_xi0s_dn7: f64 = *var_xi0s_dn7_slot;
        let mut var_xi0s_dn8: f64 = *var_xi0s_dn8_slot;
        let mut var_xi0s_dn9: f64 = *var_xi0s_dn9_slot;
        let mut var_xi1s: f64 = *var_xi1s_slot;
        let mut var_xi1s_dn4: f64 = *var_xi1s_dn4_slot;
        let mut var_xi1s_dn6: f64 = *var_xi1s_dn6_slot;
        let mut var_xi1s_dn7: f64 = *var_xi1s_dn7_slot;
        let mut var_xi1s_dn8: f64 = *var_xi1s_dn8_slot;
        let mut var_xi1s_dn9: f64 = *var_xi1s_dn9_slot;
        let mut var_xi2s: f64 = *var_xi2s_slot;
        let mut var_xi2s_dn4: f64 = *var_xi2s_dn4_slot;
        let mut var_xi2s_dn6: f64 = *var_xi2s_dn6_slot;
        let mut var_xi2s_dn7: f64 = *var_xi2s_dn7_slot;
        let mut var_xi2s_dn8: f64 = *var_xi2s_dn8_slot;
        let mut var_xi2s_dn9: f64 = *var_xi2s_dn9_slot;
        let mut var_xitsb: f64 = *var_xitsb_slot;
        let mut var_xitsb_dn4: f64 = *var_xitsb_dn4_slot;
        let mut var_xitsb_dn6: f64 = *var_xitsb_dn6_slot;
        let mut var_xitsb_dn7: f64 = *var_xitsb_dn7_slot;
        let mut var_xitsb_dn8: f64 = *var_xitsb_dn8_slot;
        let mut var_xitsb_dn9: f64 = *var_xitsb_dn9_slot;

        let (assign42450_e55840, assign42450_e55840_d_n4, assign42450_e55840_d_n6, assign42450_e55840_d_n7, assign42450_e55840_d_n8, assign42450_e55840_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42450_e55832: f64 = (var_sp_s_delta1 + var_sp_s_delta0);
        let assign42450_e55835: f64 = (var_delta_ns * var_sp_s_xi2);
        let assign42450_e55836: f64 = (assign42450_e55832 - assign42450_e55835);
        let assign42450_e55837: f64 = (var_gf2 * assign42450_e55836);
        let assign42450_e55838: f64 = (2.0 - assign42450_e55837);
        (assign42450_e55838, (-((var_gf2_dn4 * assign42450_e55836) + (var_gf2 * ((var_sp_s_delta1_dn4 + var_sp_s_delta0_dn4) - ((var_delta_ns_dn4 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn4)))))), (-((var_gf2_dn6 * assign42450_e55836) + (var_gf2 * ((var_sp_s_delta1_dn6 + var_sp_s_delta0_dn6) - ((var_delta_ns_dn6 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn6)))))), (-((var_gf2_dn7 * assign42450_e55836) + (var_gf2 * ((var_sp_s_delta1_dn7 + var_sp_s_delta0_dn7) - ((var_delta_ns_dn7 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn7)))))), (-((var_gf2_dn8 * assign42450_e55836) + (var_gf2 * ((var_sp_s_delta1_dn8 + var_sp_s_delta0_dn8) - ((var_delta_ns_dn8 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn8)))))), (-((var_gf2_dn9 * assign42450_e55836) + (var_gf2 * ((var_sp_s_delta1_dn9 + var_sp_s_delta0_dn9) - ((var_delta_ns_dn9 * var_sp_s_xi2) + (var_delta_ns * var_sp_s_xi2_dn9)))))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn4, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8, var_sp_s_temp_dn9,)
    }
};
        var_sp_s_temp = assign42450_e55840;
        var_sp_s_temp_dn4 = assign42450_e55840_d_n4;
        var_sp_s_temp_dn6 = assign42450_e55840_d_n6;
        var_sp_s_temp_dn7 = assign42450_e55840_d_n7;
        var_sp_s_temp_dn8 = assign42450_e55840_d_n8;
        var_sp_s_temp_dn9 = assign42450_e55840_d_n9;

        let (assign42460_e55856, assign42460_e55856_d_n4, assign42460_e55856_d_n6, assign42460_e55856_d_n7, assign42460_e55856_d_n8, assign42460_e55856_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42460_e55848: f64 = (var_sp_s_pc * var_sp_s_pc);
        let assign42460_e55852: f64 = (var_sp_s_qc * var_sp_s_temp);
        let assign42460_e55853: f64 = (2.0 * assign42460_e55852);
        let assign42460_e55854: f64 = (assign42460_e55848 - assign42460_e55853);
        (assign42460_e55854, (((var_sp_s_pc_dn4 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn4)) - (2.0 * ((var_sp_s_qc_dn4 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn4)))), (((var_sp_s_pc_dn6 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn6)) - (2.0 * ((var_sp_s_qc_dn6 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn6)))), (((var_sp_s_pc_dn7 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn7)) - (2.0 * ((var_sp_s_qc_dn7 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn7)))), (((var_sp_s_pc_dn8 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn8)) - (2.0 * ((var_sp_s_qc_dn8 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn8)))), (((var_sp_s_pc_dn9 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn9)) - (2.0 * ((var_sp_s_qc_dn9 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn9)))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn4, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8, var_sp_s_temp_dn9,)
    }
};
        var_sp_s_temp = assign42460_e55856;
        var_sp_s_temp_dn4 = assign42460_e55856_d_n4;
        var_sp_s_temp_dn6 = assign42460_e55856_d_n6;
        var_sp_s_temp_dn7 = assign42460_e55856_d_n7;
        var_sp_s_temp_dn8 = assign42460_e55856_d_n8;
        var_sp_s_temp_dn9 = assign42460_e55856_d_n9;

        let (assign42470_e55873, assign42470_e55873_d_n4, assign42470_e55873_d_n6, assign42470_e55873_d_n7, assign42470_e55873_d_n8, assign42470_e55873_d_n9,) = {
    if ((var_guard1199 == 0.0) && (var_guard1200 == 0.0)) {
        let assign42470_e55867: f64 = (var_sp_s_temp).sqrt();
        let assign42470_e55868: f64 = (var_sp_s_pc + assign42470_e55867);
        let assign42470_e55869: f64 = (var_sp_s_qc / assign42470_e55868);
        let assign42470_e55870: f64 = (2.0 * assign42470_e55869);
        let assign42470_e55871: f64 = (var_sp_s_x0 + assign42470_e55870);
        (assign42470_e55871, (var_sp_s_x0_dn4 + (2.0 * (((var_sp_s_qc_dn4 * assign42470_e55868) - (var_sp_s_qc * (var_sp_s_pc_dn4 + (var_sp_s_temp_dn4 / (2.0 * assign42470_e55867))))) / (assign42470_e55868 * assign42470_e55868)))), (var_sp_s_x0_dn6 + (2.0 * (((var_sp_s_qc_dn6 * assign42470_e55868) - (var_sp_s_qc * (var_sp_s_pc_dn6 + (var_sp_s_temp_dn6 / (2.0 * assign42470_e55867))))) / (assign42470_e55868 * assign42470_e55868)))), (var_sp_s_x0_dn7 + (2.0 * (((var_sp_s_qc_dn7 * assign42470_e55868) - (var_sp_s_qc * (var_sp_s_pc_dn7 + (var_sp_s_temp_dn7 / (2.0 * assign42470_e55867))))) / (assign42470_e55868 * assign42470_e55868)))), (var_sp_s_x0_dn8 + (2.0 * (((var_sp_s_qc_dn8 * assign42470_e55868) - (var_sp_s_qc * (var_sp_s_pc_dn8 + (var_sp_s_temp_dn8 / (2.0 * assign42470_e55867))))) / (assign42470_e55868 * assign42470_e55868)))), (var_sp_s_x0_dn9 + (2.0 * (((var_sp_s_qc_dn9 * assign42470_e55868) - (var_sp_s_qc * (var_sp_s_pc_dn9 + (var_sp_s_temp_dn9 / (2.0 * assign42470_e55867))))) / (assign42470_e55868 * assign42470_e55868)))),)
    } else {
        (var_x_s, var_x_s_dn4, var_x_s_dn6, var_x_s_dn7, var_x_s_dn8, var_x_s_dn9,)
    }
};
        var_x_s = assign42470_e55873;
        var_x_s_dn4 = assign42470_e55873_d_n4;
        var_x_s_dn6 = assign42470_e55873_d_n6;
        var_x_s_dn7 = assign42470_e55873_d_n7;
        var_x_s_dn8 = assign42470_e55873_d_n8;
        var_x_s_dn9 = assign42470_e55873_d_n9;

        var_xi1s = 0.0;
        var_xi1s_dn4 = 0.0;
        var_xi1s_dn6 = 0.0;
        var_xi1s_dn7 = 0.0;
        var_xi1s_dn8 = 0.0;
        var_xi1s_dn9 = 0.0;

        var_xi2s = 0.0;
        var_xi2s_dn4 = 0.0;
        var_xi2s_dn6 = 0.0;
        var_xi2s_dn7 = 0.0;
        var_xi2s_dn8 = 0.0;
        var_xi2s_dn9 = 0.0;

        var_delta_1s = 0.0;
        var_delta_1s_dn4 = 0.0;
        var_delta_1s_dn6 = 0.0;
        var_delta_1s_dn7 = 0.0;
        var_delta_1s_dn8 = 0.0;
        var_delta_1s_dn9 = 0.0;

        var_es = 0.0;
        var_es_dn4 = 0.0;
        var_es_dn6 = 0.0;
        var_es_dn7 = 0.0;
        var_es_dn8 = 0.0;
        var_es_dn9 = 0.0;

        var_ds = 0.0;
        var_ds_dn4 = 0.0;
        var_ds_dn6 = 0.0;
        var_ds_dn7 = 0.0;
        var_ds_dn8 = 0.0;
        var_ds_dn9 = 0.0;

        var_ps = 0.0;
        var_ps_dn4 = 0.0;
        var_ps_dn6 = 0.0;
        var_ps_dn7 = 0.0;
        var_ps_dn8 = 0.0;
        var_ps_dn9 = 0.0;

        var_sqs = 0.0;
        var_sqs_dn4 = 0.0;
        var_sqs_dn6 = 0.0;
        var_sqs_dn7 = 0.0;
        var_sqs_dn8 = 0.0;
        var_sqs_dn9 = 0.0;

        var_alphas = 1.0;
        var_alphas_dn4 = 0.0;
        var_alphas_dn6 = 0.0;
        var_alphas_dn7 = 0.0;
        var_alphas_dn8 = 0.0;
        var_alphas_dn9 = 0.0;

        var_rxcor = 1.0;
        var_rxcor_dn4 = 0.0;
        var_rxcor_dn6 = 0.0;
        var_rxcor_dn7 = 0.0;
        var_rxcor_dn8 = 0.0;
        var_rxcor_dn9 = 0.0;

        let assign42570_e55885: f64 = (var_xg - var_x_s);
        var_xgs = assign42570_e55885;
        var_xgs_dn4 = (var_xg_dn4 - var_x_s_dn4);
        var_xgs_dn6 = (var_xg_dn6 - var_x_s_dn6);
        var_xgs_dn7 = (var_xg_dn7 - var_x_s_dn7);
        var_xgs_dn8 = (var_xg_dn8 - var_x_s_dn8);
        var_xgs_dn9 = (var_xg_dn9 - var_x_s_dn9);

        var_qis = 0.0;
        var_qis_dn4 = 0.0;
        var_qis_dn6 = 0.0;
        var_qis_dn7 = 0.0;
        var_qis_dn8 = 0.0;
        var_qis_dn9 = 0.0;

        let assign42590_e55889: f64 = (var_phit1 * var_xgs);
        var_qbs = assign42590_e55889;
        var_qbs_dn4 = ((var_phit1_dn4 * var_xgs) + (var_phit1 * var_xgs_dn4));
        var_qbs_dn6 = ((var_phit1_dn6 * var_xgs) + (var_phit1 * var_xgs_dn6));
        var_qbs_dn7 = ((var_phit1_dn7 * var_xgs) + (var_phit1 * var_xgs_dn7));
        var_qbs_dn8 = ((var_phit1_dn8 * var_xgs) + (var_phit1 * var_xgs_dn8));
        var_qbs_dn9 = ((var_phit1_dn9 * var_xgs) + (var_phit1 * var_xgs_dn9));

        var_rhob = 1.0;
        var_rhob_dn4 = 0.0;
        var_rhob_dn6 = 0.0;
        var_rhob_dn7 = 0.0;
        var_rhob_dn8 = 0.0;
        var_rhob_dn9 = 0.0;

        var_rhog = 1.0;
        var_rhog_dn4 = 0.0;
        var_rhog_dn6 = 0.0;
        var_rhog_dn7 = 0.0;
        var_rhog_dn8 = 0.0;
        var_rhog_dn9 = 0.0;

        var_gmobs = 1.0;
        var_gmobs_dn4 = 0.0;
        var_gmobs_dn6 = 0.0;
        var_gmobs_dn7 = 0.0;
        var_gmobs_dn8 = 0.0;
        var_gmobs_dn9 = 0.0;

        var_xitsb = 1.0;
        var_xitsb_dn4 = 0.0;
        var_xitsb_dn6 = 0.0;
        var_xitsb_dn7 = 0.0;
        var_xitsb_dn8 = 0.0;
        var_xitsb_dn9 = 0.0;

        var_factheta = 1.0;
        var_factheta_dn4 = 0.0;
        var_factheta_dn6 = 0.0;
        var_factheta_dn7 = 0.0;
        var_factheta_dn8 = 0.0;
        var_factheta_dn9 = 0.0;

        let assign42650_e55897: f64 = if var_xg > 0.0 { 1.0 } else { 0.0 };
        var_guard1205 = assign42650_e55897;

        let (assign42660_e55907, assign42660_e55907_d_n4, assign42660_e55907_d_n6, assign42660_e55907_d_n7, assign42660_e55907_d_n8, assign42660_e55907_d_n9,) = {
    if (var_guard1205 != 0.0) {
        let assign42660_e55903: f64 = (var_x_s * var_x_s);
        let assign42660_e55904: f64 = (2.0 + assign42660_e55903);
        let assign42660_e55905: f64 = (1.0 / assign42660_e55904);
        (assign42660_e55905, (-(((var_x_s_dn4 * var_x_s) + (var_x_s * var_x_s_dn4)) / (assign42660_e55904 * assign42660_e55904))), (-(((var_x_s_dn6 * var_x_s) + (var_x_s * var_x_s_dn6)) / (assign42660_e55904 * assign42660_e55904))), (-(((var_x_s_dn7 * var_x_s) + (var_x_s * var_x_s_dn7)) / (assign42660_e55904 * assign42660_e55904))), (-(((var_x_s_dn8 * var_x_s) + (var_x_s * var_x_s_dn8)) / (assign42660_e55904 * assign42660_e55904))), (-(((var_x_s_dn9 * var_x_s) + (var_x_s * var_x_s_dn9)) / (assign42660_e55904 * assign42660_e55904))),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign42660_e55907;
        var_temp__blk949_dn4 = assign42660_e55907_d_n4;
        var_temp__blk949_dn6 = assign42660_e55907_d_n6;
        var_temp__blk949_dn7 = assign42660_e55907_d_n7;
        var_temp__blk949_dn8 = assign42660_e55907_d_n8;
        var_temp__blk949_dn9 = assign42660_e55907_d_n9;

        let (assign42670_e55915, assign42670_e55915_d_n4, assign42670_e55915_d_n6, assign42670_e55915_d_n7, assign42670_e55915_d_n8, assign42670_e55915_d_n9,) = {
    if (var_guard1205 != 0.0) {
        let assign42670_e55911: f64 = (var_x_s * var_x_s);
        let assign42670_e55913: f64 = (assign42670_e55911 * var_temp__blk949);
        (assign42670_e55913, ((((var_x_s_dn4 * var_x_s) + (var_x_s * var_x_s_dn4)) * var_temp__blk949) + (assign42670_e55911 * var_temp__blk949_dn4)), ((((var_x_s_dn6 * var_x_s) + (var_x_s * var_x_s_dn6)) * var_temp__blk949) + (assign42670_e55911 * var_temp__blk949_dn6)), ((((var_x_s_dn7 * var_x_s) + (var_x_s * var_x_s_dn7)) * var_temp__blk949) + (assign42670_e55911 * var_temp__blk949_dn7)), ((((var_x_s_dn8 * var_x_s) + (var_x_s * var_x_s_dn8)) * var_temp__blk949) + (assign42670_e55911 * var_temp__blk949_dn8)), ((((var_x_s_dn9 * var_x_s) + (var_x_s * var_x_s_dn9)) * var_temp__blk949) + (assign42670_e55911 * var_temp__blk949_dn9)),)
    } else {
        (var_xi0s, var_xi0s_dn4, var_xi0s_dn6, var_xi0s_dn7, var_xi0s_dn8, var_xi0s_dn9,)
    }
};
        var_xi0s = assign42670_e55915;
        var_xi0s_dn4 = assign42670_e55915_d_n4;
        var_xi0s_dn6 = assign42670_e55915_d_n6;
        var_xi0s_dn7 = assign42670_e55915_d_n7;
        var_xi0s_dn8 = assign42670_e55915_d_n8;
        var_xi0s_dn9 = assign42670_e55915_d_n9;

        let (assign42680_e55925, assign42680_e55925_d_n4, assign42680_e55925_d_n6, assign42680_e55925_d_n7, assign42680_e55925_d_n8, assign42680_e55925_d_n9,) = {
    if (var_guard1205 != 0.0) {
        let assign42680_e55920: f64 = (var_x_s * var_temp__blk949);
        let assign42680_e55922: f64 = (assign42680_e55920 * var_temp__blk949);
        let assign42680_e55923: f64 = (4.0 * assign42680_e55922);
        (assign42680_e55923, (4.0 * ((((var_x_s_dn4 * var_temp__blk949) + (var_x_s * var_temp__blk949_dn4)) * var_temp__blk949) + (assign42680_e55920 * var_temp__blk949_dn4))), (4.0 * ((((var_x_s_dn6 * var_temp__blk949) + (var_x_s * var_temp__blk949_dn6)) * var_temp__blk949) + (assign42680_e55920 * var_temp__blk949_dn6))), (4.0 * ((((var_x_s_dn7 * var_temp__blk949) + (var_x_s * var_temp__blk949_dn7)) * var_temp__blk949) + (assign42680_e55920 * var_temp__blk949_dn7))), (4.0 * ((((var_x_s_dn8 * var_temp__blk949) + (var_x_s * var_temp__blk949_dn8)) * var_temp__blk949) + (assign42680_e55920 * var_temp__blk949_dn8))), (4.0 * ((((var_x_s_dn9 * var_temp__blk949) + (var_x_s * var_temp__blk949_dn9)) * var_temp__blk949) + (assign42680_e55920 * var_temp__blk949_dn9))),)
    } else {
        (var_xi1s, var_xi1s_dn4, var_xi1s_dn6, var_xi1s_dn7, var_xi1s_dn8, var_xi1s_dn9,)
    }
};
        var_xi1s = assign42680_e55925;
        var_xi1s_dn4 = assign42680_e55925_d_n4;
        var_xi1s_dn6 = assign42680_e55925_d_n6;
        var_xi1s_dn7 = assign42680_e55925_d_n7;
        var_xi1s_dn8 = assign42680_e55925_d_n8;
        var_xi1s_dn9 = assign42680_e55925_d_n9;

        let (assign42690_e55939, assign42690_e55939_d_n4, assign42690_e55939_d_n6, assign42690_e55939_d_n7, assign42690_e55939_d_n8, assign42690_e55939_d_n9,) = {
    if (var_guard1205 != 0.0) {
        let assign42690_e55929: f64 = (8.0 * var_temp__blk949);
        let assign42690_e55932: f64 = (12.0 * var_xi0s);
        let assign42690_e55933: f64 = (assign42690_e55929 - assign42690_e55932);
        let assign42690_e55935: f64 = (assign42690_e55933 * var_temp__blk949);
        let assign42690_e55937: f64 = (assign42690_e55935 * var_temp__blk949);
        (assign42690_e55937, ((((((8.0 * var_temp__blk949_dn4) - (12.0 * var_xi0s_dn4)) * var_temp__blk949) + (assign42690_e55933 * var_temp__blk949_dn4)) * var_temp__blk949) + (assign42690_e55935 * var_temp__blk949_dn4)), ((((((8.0 * var_temp__blk949_dn6) - (12.0 * var_xi0s_dn6)) * var_temp__blk949) + (assign42690_e55933 * var_temp__blk949_dn6)) * var_temp__blk949) + (assign42690_e55935 * var_temp__blk949_dn6)), ((((((8.0 * var_temp__blk949_dn7) - (12.0 * var_xi0s_dn7)) * var_temp__blk949) + (assign42690_e55933 * var_temp__blk949_dn7)) * var_temp__blk949) + (assign42690_e55935 * var_temp__blk949_dn7)), ((((((8.0 * var_temp__blk949_dn8) - (12.0 * var_xi0s_dn8)) * var_temp__blk949) + (assign42690_e55933 * var_temp__blk949_dn8)) * var_temp__blk949) + (assign42690_e55935 * var_temp__blk949_dn8)), ((((((8.0 * var_temp__blk949_dn9) - (12.0 * var_xi0s_dn9)) * var_temp__blk949) + (assign42690_e55933 * var_temp__blk949_dn9)) * var_temp__blk949) + (assign42690_e55935 * var_temp__blk949_dn9)),)
    } else {
        (var_xi2s, var_xi2s_dn4, var_xi2s_dn6, var_xi2s_dn7, var_xi2s_dn8, var_xi2s_dn9,)
    }
};
        var_xi2s = assign42690_e55939;
        var_xi2s_dn4 = assign42690_e55939_d_n4;
        var_xi2s_dn6 = assign42690_e55939_d_n6;
        var_xi2s_dn7 = assign42690_e55939_d_n7;
        var_xi2s_dn8 = assign42690_e55939_d_n8;
        var_xi2s_dn9 = assign42690_e55939_d_n9;

        let (assign42700_e55943, assign42700_e55943_d_n4, assign42700_e55943_d_n6, assign42700_e55943_d_n7, assign42700_e55943_d_n8, assign42700_e55943_d_n9,) = {
    if (var_guard1205 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_delta_1s, var_delta_1s_dn4, var_delta_1s_dn6, var_delta_1s_dn7, var_delta_1s_dn8, var_delta_1s_dn9,)
    }
};
        var_delta_1s = assign42700_e55943;
        var_delta_1s_dn4 = assign42700_e55943_d_n4;
        var_delta_1s_dn6 = assign42700_e55943_d_n6;
        var_delta_1s_dn7 = assign42700_e55943_d_n7;
        var_delta_1s_dn8 = assign42700_e55943_d_n8;
        var_delta_1s_dn9 = assign42700_e55943_d_n9;

        let assign42710_e55946: f64 = if var_x_s < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1206 = assign42710_e55946;

        let (assign42720_e55953, assign42720_e55953_d_n4, assign42720_e55953_d_n6, assign42720_e55953_d_n7, assign42720_e55953_d_n8, assign42720_e55953_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1206 != 0.0)) {
        let assign42720_e55951: f64 = (var_x_s).exp();
        (assign42720_e55951, (assign42720_e55951 * var_x_s_dn4), (assign42720_e55951 * var_x_s_dn6), (assign42720_e55951 * var_x_s_dn7), (assign42720_e55951 * var_x_s_dn8), (assign42720_e55951 * var_x_s_dn9),)
    } else {
        (var_delta_1s, var_delta_1s_dn4, var_delta_1s_dn6, var_delta_1s_dn7, var_delta_1s_dn8, var_delta_1s_dn9,)
    }
};
        var_delta_1s = assign42720_e55953;
        var_delta_1s_dn4 = assign42720_e55953_d_n4;
        var_delta_1s_dn6 = assign42720_e55953_d_n6;
        var_delta_1s_dn7 = assign42720_e55953_d_n7;
        var_delta_1s_dn8 = assign42720_e55953_d_n8;
        var_delta_1s_dn9 = assign42720_e55953_d_n9;

        let (assign42730_e55961, assign42730_e55961_d_n4, assign42730_e55961_d_n6, assign42730_e55961_d_n7, assign42730_e55961_d_n8, assign42730_e55961_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1206 != 0.0)) {
        let assign42730_e55959: f64 = (1.0 / var_delta_1s);
        (assign42730_e55959, (-(var_delta_1s_dn4 / (var_delta_1s * var_delta_1s))), (-(var_delta_1s_dn6 / (var_delta_1s * var_delta_1s))), (-(var_delta_1s_dn7 / (var_delta_1s * var_delta_1s))), (-(var_delta_1s_dn8 / (var_delta_1s * var_delta_1s))), (-(var_delta_1s_dn9 / (var_delta_1s * var_delta_1s))),)
    } else {
        (var_es, var_es_dn4, var_es_dn6, var_es_dn7, var_es_dn8, var_es_dn9,)
    }
};
        var_es = assign42730_e55961;
        var_es_dn4 = assign42730_e55961_d_n4;
        var_es_dn6 = assign42730_e55961_d_n6;
        var_es_dn7 = assign42730_e55961_d_n7;
        var_es_dn8 = assign42730_e55961_d_n8;
        var_es_dn9 = assign42730_e55961_d_n9;

        let (assign42740_e55969, assign42740_e55969_d_n4, assign42740_e55969_d_n6, assign42740_e55969_d_n7, assign42740_e55969_d_n8, assign42740_e55969_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1206 != 0.0)) {
        let assign42740_e55967: f64 = (var_delta_ns * var_delta_1s);
        (assign42740_e55967, ((var_delta_ns_dn4 * var_delta_1s) + (var_delta_ns * var_delta_1s_dn4)), ((var_delta_ns_dn6 * var_delta_1s) + (var_delta_ns * var_delta_1s_dn6)), ((var_delta_ns_dn7 * var_delta_1s) + (var_delta_ns * var_delta_1s_dn7)), ((var_delta_ns_dn8 * var_delta_1s) + (var_delta_ns * var_delta_1s_dn8)), ((var_delta_ns_dn9 * var_delta_1s) + (var_delta_ns * var_delta_1s_dn9)),)
    } else {
        (var_delta_1s, var_delta_1s_dn4, var_delta_1s_dn6, var_delta_1s_dn7, var_delta_1s_dn8, var_delta_1s_dn9,)
    }
};
        var_delta_1s = assign42740_e55969;
        var_delta_1s_dn4 = assign42740_e55969_d_n4;
        var_delta_1s_dn6 = assign42740_e55969_d_n6;
        var_delta_1s_dn7 = assign42740_e55969_d_n7;
        var_delta_1s_dn8 = assign42740_e55969_d_n8;
        var_delta_1s_dn9 = assign42740_e55969_d_n9;

        let assign42750_e55973: f64 = (var_xn_s - 230.25850929940458);
        let assign42750_e55974: f64 = if var_x_s > assign42750_e55973 { 1.0 } else { 0.0 };
        var_guard1207 = assign42750_e55974;

        let (assign42760_e55986, assign42760_e55986_d_n4, assign42760_e55986_d_n6, assign42760_e55986_d_n7, assign42760_e55986_d_n8, assign42760_e55986_d_n9,) = {
    if (((var_guard1205 != 0.0) && (var_guard1206 == 0.0)) && (var_guard1207 != 0.0)) {
        let assign42760_e55983: f64 = (var_x_s - var_xn_s);
        let assign42760_e55984: f64 = (assign42760_e55983).exp();
        (assign42760_e55984, (assign42760_e55984 * (var_x_s_dn4 - var_xn_s_dn4)), (assign42760_e55984 * (var_x_s_dn6 - var_xn_s_dn6)), (assign42760_e55984 * (var_x_s_dn7 - var_xn_s_dn7)), (assign42760_e55984 * (var_x_s_dn8 - var_xn_s_dn8)), (assign42760_e55984 * (var_x_s_dn9 - var_xn_s_dn9)),)
    } else {
        (var_delta_1s, var_delta_1s_dn4, var_delta_1s_dn6, var_delta_1s_dn7, var_delta_1s_dn8, var_delta_1s_dn9,)
    }
};
        var_delta_1s = assign42760_e55986;
        var_delta_1s_dn4 = assign42760_e55986_d_n4;
        var_delta_1s_dn6 = assign42760_e55986_d_n6;
        var_delta_1s_dn7 = assign42760_e55986_d_n7;
        var_delta_1s_dn8 = assign42760_e55986_d_n8;
        var_delta_1s_dn9 = assign42760_e55986_d_n9;

        let (assign42770_e55997, assign42770_e55997_d_n4, assign42770_e55997_d_n6, assign42770_e55997_d_n7, assign42770_e55997_d_n8, assign42770_e55997_d_n9,) = {
    if (((var_guard1205 != 0.0) && (var_guard1206 == 0.0)) && (var_guard1207 != 0.0)) {
        let assign42770_e55995: f64 = (var_delta_ns / var_delta_1s);
        (assign42770_e55995, (((var_delta_ns_dn4 * var_delta_1s) - (var_delta_ns * var_delta_1s_dn4)) / (var_delta_1s * var_delta_1s)), (((var_delta_ns_dn6 * var_delta_1s) - (var_delta_ns * var_delta_1s_dn6)) / (var_delta_1s * var_delta_1s)), (((var_delta_ns_dn7 * var_delta_1s) - (var_delta_ns * var_delta_1s_dn7)) / (var_delta_1s * var_delta_1s)), (((var_delta_ns_dn8 * var_delta_1s) - (var_delta_ns * var_delta_1s_dn8)) / (var_delta_1s * var_delta_1s)), (((var_delta_ns_dn9 * var_delta_1s) - (var_delta_ns * var_delta_1s_dn9)) / (var_delta_1s * var_delta_1s)),)
    } else {
        (var_es, var_es_dn4, var_es_dn6, var_es_dn7, var_es_dn8, var_es_dn9,)
    }
};
        var_es = assign42770_e55997;
        var_es_dn4 = assign42770_e55997_d_n4;
        var_es_dn6 = assign42770_e55997_d_n6;
        var_es_dn7 = assign42770_e55997_d_n7;
        var_es_dn8 = assign42770_e55997_d_n8;
        var_es_dn9 = assign42770_e55997_d_n9;

        let (assign42780_e56035, assign42780_e56035_d_n4, assign42780_e56035_d_n6, assign42780_e56035_d_n7, assign42780_e56035_d_n8, assign42780_e56035_d_n9,) = {
    if (((var_guard1205 != 0.0) && (var_guard1206 == 0.0)) && (var_guard1207 == 0.0)) {
        let assign42780_e56009: f64 = (var_xn_s - var_x_s);
        let assign42780_e56011: f64 = (assign42780_e56009 - 230.25850929940458);
        let assign42780_e56016: f64 = (var_xn_s - var_x_s);
        let assign42780_e56018: f64 = (assign42780_e56016 - 230.25850929940458);
        let assign42780_e56022: f64 = (var_xn_s - var_x_s);
        let assign42780_e56024: f64 = (assign42780_e56022 - 230.25850929940458);
        let assign42780_e56026: f64 = (assign42780_e56024 * 0.3333333333333333);
        let assign42780_e56027: f64 = (1.0 + assign42780_e56026);
        let assign42780_e56028: f64 = (assign42780_e56018 * assign42780_e56027);
        let assign42780_e56029: f64 = (0.5 * assign42780_e56028);
        let assign42780_e56030: f64 = (1.0 + assign42780_e56029);
        let assign42780_e56031: f64 = (assign42780_e56011 * assign42780_e56030);
        let assign42780_e56032: f64 = (1.0 + assign42780_e56031);
        let assign42780_e56033: f64 = (1e-100 / assign42780_e56032);
        (assign42780_e56033, (-((1e-100 * (((var_xn_s_dn4 - var_x_s_dn4) * assign42780_e56030) + (assign42780_e56011 * (0.5 * (((var_xn_s_dn4 - var_x_s_dn4) * assign42780_e56027) + (assign42780_e56018 * ((var_xn_s_dn4 - var_x_s_dn4) * 0.3333333333333333))))))) / (assign42780_e56032 * assign42780_e56032))), (-((1e-100 * (((var_xn_s_dn6 - var_x_s_dn6) * assign42780_e56030) + (assign42780_e56011 * (0.5 * (((var_xn_s_dn6 - var_x_s_dn6) * assign42780_e56027) + (assign42780_e56018 * ((var_xn_s_dn6 - var_x_s_dn6) * 0.3333333333333333))))))) / (assign42780_e56032 * assign42780_e56032))), (-((1e-100 * (((var_xn_s_dn7 - var_x_s_dn7) * assign42780_e56030) + (assign42780_e56011 * (0.5 * (((var_xn_s_dn7 - var_x_s_dn7) * assign42780_e56027) + (assign42780_e56018 * ((var_xn_s_dn7 - var_x_s_dn7) * 0.3333333333333333))))))) / (assign42780_e56032 * assign42780_e56032))), (-((1e-100 * (((var_xn_s_dn8 - var_x_s_dn8) * assign42780_e56030) + (assign42780_e56011 * (0.5 * (((var_xn_s_dn8 - var_x_s_dn8) * assign42780_e56027) + (assign42780_e56018 * ((var_xn_s_dn8 - var_x_s_dn8) * 0.3333333333333333))))))) / (assign42780_e56032 * assign42780_e56032))), (-((1e-100 * (((var_xn_s_dn9 - var_x_s_dn9) * assign42780_e56030) + (assign42780_e56011 * (0.5 * (((var_xn_s_dn9 - var_x_s_dn9) * assign42780_e56027) + (assign42780_e56018 * ((var_xn_s_dn9 - var_x_s_dn9) * 0.3333333333333333))))))) / (assign42780_e56032 * assign42780_e56032))),)
    } else {
        (var_delta_1s, var_delta_1s_dn4, var_delta_1s_dn6, var_delta_1s_dn7, var_delta_1s_dn8, var_delta_1s_dn9,)
    }
};
        var_delta_1s = assign42780_e56035;
        var_delta_1s_dn4 = assign42780_e56035_d_n4;
        var_delta_1s_dn6 = assign42780_e56035_d_n6;
        var_delta_1s_dn7 = assign42780_e56035_d_n7;
        var_delta_1s_dn8 = assign42780_e56035_d_n8;
        var_delta_1s_dn9 = assign42780_e56035_d_n9;

        let (assign42790_e56067, assign42790_e56067_d_n4, assign42790_e56067_d_n6, assign42790_e56067_d_n7, assign42790_e56067_d_n8, assign42790_e56067_d_n9,) = {
    if (((var_guard1205 != 0.0) && (var_guard1206 == 0.0)) && (var_guard1207 == 0.0)) {
        let assign42790_e56047: f64 = (var_x_s - 230.25850929940458);
        let assign42790_e56052: f64 = (var_x_s - 230.25850929940458);
        let assign42790_e56056: f64 = (var_x_s - 230.25850929940458);
        let assign42790_e56058: f64 = (assign42790_e56056 * 0.3333333333333333);
        let assign42790_e56059: f64 = (1.0 + assign42790_e56058);
        let assign42790_e56060: f64 = (assign42790_e56052 * assign42790_e56059);
        let assign42790_e56061: f64 = (0.5 * assign42790_e56060);
        let assign42790_e56062: f64 = (1.0 + assign42790_e56061);
        let assign42790_e56063: f64 = (assign42790_e56047 * assign42790_e56062);
        let assign42790_e56064: f64 = (1.0 + assign42790_e56063);
        let assign42790_e56065: f64 = (1e-100 / assign42790_e56064);
        (assign42790_e56065, (-((1e-100 * ((var_x_s_dn4 * assign42790_e56062) + (assign42790_e56047 * (0.5 * ((var_x_s_dn4 * assign42790_e56059) + (assign42790_e56052 * (var_x_s_dn4 * 0.3333333333333333))))))) / (assign42790_e56064 * assign42790_e56064))), (-((1e-100 * ((var_x_s_dn6 * assign42790_e56062) + (assign42790_e56047 * (0.5 * ((var_x_s_dn6 * assign42790_e56059) + (assign42790_e56052 * (var_x_s_dn6 * 0.3333333333333333))))))) / (assign42790_e56064 * assign42790_e56064))), (-((1e-100 * ((var_x_s_dn7 * assign42790_e56062) + (assign42790_e56047 * (0.5 * ((var_x_s_dn7 * assign42790_e56059) + (assign42790_e56052 * (var_x_s_dn7 * 0.3333333333333333))))))) / (assign42790_e56064 * assign42790_e56064))), (-((1e-100 * ((var_x_s_dn8 * assign42790_e56062) + (assign42790_e56047 * (0.5 * ((var_x_s_dn8 * assign42790_e56059) + (assign42790_e56052 * (var_x_s_dn8 * 0.3333333333333333))))))) / (assign42790_e56064 * assign42790_e56064))), (-((1e-100 * ((var_x_s_dn9 * assign42790_e56062) + (assign42790_e56047 * (0.5 * ((var_x_s_dn9 * assign42790_e56059) + (assign42790_e56052 * (var_x_s_dn9 * 0.3333333333333333))))))) / (assign42790_e56064 * assign42790_e56064))),)
    } else {
        (var_es, var_es_dn4, var_es_dn6, var_es_dn7, var_es_dn8, var_es_dn9,)
    }
};
        var_es = assign42790_e56067;
        var_es_dn4 = assign42790_e56067_d_n4;
        var_es_dn6 = assign42790_e56067_d_n6;
        var_es_dn7 = assign42790_e56067_d_n7;
        var_es_dn8 = assign42790_e56067_d_n8;
        var_es_dn9 = assign42790_e56067_d_n9;

        let (assign42800_e56079, assign42800_e56079_d_n4, assign42800_e56079_d_n6, assign42800_e56079_d_n7, assign42800_e56079_d_n8, assign42800_e56079_d_n9,) = {
    if (var_guard1205 != 0.0) {
        let assign42800_e56073: f64 = (var_x_s + 1.0);
        let assign42800_e56075: f64 = (assign42800_e56073 + var_xi0s);
        let assign42800_e56076: f64 = (var_delta_ns * assign42800_e56075);
        let assign42800_e56077: f64 = (var_delta_1s - assign42800_e56076);
        (assign42800_e56077, (var_delta_1s_dn4 - ((var_delta_ns_dn4 * assign42800_e56075) + (var_delta_ns * (var_x_s_dn4 + var_xi0s_dn4)))), (var_delta_1s_dn6 - ((var_delta_ns_dn6 * assign42800_e56075) + (var_delta_ns * (var_x_s_dn6 + var_xi0s_dn6)))), (var_delta_1s_dn7 - ((var_delta_ns_dn7 * assign42800_e56075) + (var_delta_ns * (var_x_s_dn7 + var_xi0s_dn7)))), (var_delta_1s_dn8 - ((var_delta_ns_dn8 * assign42800_e56075) + (var_delta_ns * (var_x_s_dn8 + var_xi0s_dn8)))), (var_delta_1s_dn9 - ((var_delta_ns_dn9 * assign42800_e56075) + (var_delta_ns * (var_x_s_dn9 + var_xi0s_dn9)))),)
    } else {
        (var_ds, var_ds_dn4, var_ds_dn6, var_ds_dn7, var_ds_dn8, var_ds_dn9,)
    }
};
        var_ds = assign42800_e56079;
        var_ds_dn4 = assign42800_e56079_d_n4;
        var_ds_dn6 = assign42800_e56079_d_n6;
        var_ds_dn7 = assign42800_e56079_d_n7;
        var_ds_dn8 = assign42800_e56079_d_n8;
        var_ds_dn9 = assign42800_e56079_d_n9;

        let assign42810_e56082: f64 = if var_x_s < 1e-5 { 1.0 } else { 0.0 };
        var_guard1208 = assign42810_e56082;

        let (assign42820_e56104, assign42820_e56104_d_n4, assign42820_e56104_d_n6, assign42820_e56104_d_n7, assign42820_e56104_d_n8, assign42820_e56104_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1208 != 0.0)) {
        let assign42820_e56089: f64 = (var_x_s * var_x_s);
        let assign42820_e56096: f64 = (0.25 * var_x_s);
        let assign42820_e56097: f64 = (1.0 - assign42820_e56096);
        let assign42820_e56098: f64 = (var_x_s * assign42820_e56097);
        let assign42820_e56099: f64 = (0.3333333333333333 * assign42820_e56098);
        let assign42820_e56100: f64 = (1.0 - assign42820_e56099);
        let assign42820_e56101: f64 = (assign42820_e56089 * assign42820_e56100);
        let assign42820_e56102: f64 = (0.5 * assign42820_e56101);
        (assign42820_e56102, (0.5 * ((((var_x_s_dn4 * var_x_s) + (var_x_s * var_x_s_dn4)) * assign42820_e56100) + (assign42820_e56089 * (-(0.3333333333333333 * ((var_x_s_dn4 * assign42820_e56097) + (var_x_s * (-(0.25 * var_x_s_dn4))))))))), (0.5 * ((((var_x_s_dn6 * var_x_s) + (var_x_s * var_x_s_dn6)) * assign42820_e56100) + (assign42820_e56089 * (-(0.3333333333333333 * ((var_x_s_dn6 * assign42820_e56097) + (var_x_s * (-(0.25 * var_x_s_dn6))))))))), (0.5 * ((((var_x_s_dn7 * var_x_s) + (var_x_s * var_x_s_dn7)) * assign42820_e56100) + (assign42820_e56089 * (-(0.3333333333333333 * ((var_x_s_dn7 * assign42820_e56097) + (var_x_s * (-(0.25 * var_x_s_dn7))))))))), (0.5 * ((((var_x_s_dn8 * var_x_s) + (var_x_s * var_x_s_dn8)) * assign42820_e56100) + (assign42820_e56089 * (-(0.3333333333333333 * ((var_x_s_dn8 * assign42820_e56097) + (var_x_s * (-(0.25 * var_x_s_dn8))))))))), (0.5 * ((((var_x_s_dn9 * var_x_s) + (var_x_s * var_x_s_dn9)) * assign42820_e56100) + (assign42820_e56089 * (-(0.3333333333333333 * ((var_x_s_dn9 * assign42820_e56097) + (var_x_s * (-(0.25 * var_x_s_dn9))))))))),)
    } else {
        (var_ps, var_ps_dn4, var_ps_dn6, var_ps_dn7, var_ps_dn8, var_ps_dn9,)
    }
};
        var_ps = assign42820_e56104;
        var_ps_dn4 = assign42820_e56104_d_n4;
        var_ps_dn6 = assign42820_e56104_d_n6;
        var_ps_dn7 = assign42820_e56104_d_n7;
        var_ps_dn8 = assign42820_e56104_d_n8;
        var_ps_dn9 = assign42820_e56104_d_n9;

        let (assign42830_e56124, assign42830_e56124_d_n4, assign42830_e56124_d_n6, assign42830_e56124_d_n7, assign42830_e56124_d_n8, assign42830_e56124_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1208 != 0.0)) {
        let assign42830_e56111: f64 = (var_delta_ns * var_x_s);
        let assign42830_e56113: f64 = (assign42830_e56111 * var_x_s);
        let assign42830_e56115: f64 = (assign42830_e56113 * var_x_s);
        let assign42830_e56119: f64 = (1.75 * var_x_s);
        let assign42830_e56120: f64 = (1.0 + assign42830_e56119);
        let assign42830_e56121: f64 = (assign42830_e56115 * assign42830_e56120);
        let assign42830_e56122: f64 = (0.16666666666666666 * assign42830_e56121);
        (assign42830_e56122, (0.16666666666666666 * ((((((((var_delta_ns_dn4 * var_x_s) + (var_delta_ns * var_x_s_dn4)) * var_x_s) + (assign42830_e56111 * var_x_s_dn4)) * var_x_s) + (assign42830_e56113 * var_x_s_dn4)) * assign42830_e56120) + (assign42830_e56115 * (1.75 * var_x_s_dn4)))), (0.16666666666666666 * ((((((((var_delta_ns_dn6 * var_x_s) + (var_delta_ns * var_x_s_dn6)) * var_x_s) + (assign42830_e56111 * var_x_s_dn6)) * var_x_s) + (assign42830_e56113 * var_x_s_dn6)) * assign42830_e56120) + (assign42830_e56115 * (1.75 * var_x_s_dn6)))), (0.16666666666666666 * ((((((((var_delta_ns_dn7 * var_x_s) + (var_delta_ns * var_x_s_dn7)) * var_x_s) + (assign42830_e56111 * var_x_s_dn7)) * var_x_s) + (assign42830_e56113 * var_x_s_dn7)) * assign42830_e56120) + (assign42830_e56115 * (1.75 * var_x_s_dn7)))), (0.16666666666666666 * ((((((((var_delta_ns_dn8 * var_x_s) + (var_delta_ns * var_x_s_dn8)) * var_x_s) + (assign42830_e56111 * var_x_s_dn8)) * var_x_s) + (assign42830_e56113 * var_x_s_dn8)) * assign42830_e56120) + (assign42830_e56115 * (1.75 * var_x_s_dn8)))), (0.16666666666666666 * ((((((((var_delta_ns_dn9 * var_x_s) + (var_delta_ns * var_x_s_dn9)) * var_x_s) + (assign42830_e56111 * var_x_s_dn9)) * var_x_s) + (assign42830_e56113 * var_x_s_dn9)) * assign42830_e56120) + (assign42830_e56115 * (1.75 * var_x_s_dn9)))),)
    } else {
        (var_ds, var_ds_dn4, var_ds_dn6, var_ds_dn7, var_ds_dn8, var_ds_dn9,)
    }
};
        var_ds = assign42830_e56124;
        var_ds_dn4 = assign42830_e56124_d_n4;
        var_ds_dn6 = assign42830_e56124_d_n6;
        var_ds_dn7 = assign42830_e56124_d_n7;
        var_ds_dn8 = assign42830_e56124_d_n8;
        var_ds_dn9 = assign42830_e56124_d_n9;

        let (assign42840_e56141, assign42840_e56141_d_n4, assign42840_e56141_d_n6, assign42840_e56141_d_n7, assign42840_e56141_d_n8, assign42840_e56141_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1208 != 0.0)) {
        let assign42840_e56134: f64 = (0.25 * var_x_s);
        let assign42840_e56135: f64 = (1.0 - assign42840_e56134);
        let assign42840_e56136: f64 = (var_x_s * assign42840_e56135);
        let assign42840_e56137: f64 = (0.3333333333333333 * assign42840_e56136);
        let assign42840_e56138: f64 = (1.0 - assign42840_e56137);
        let assign42840_e56139: f64 = (assign42840_e56138).sqrt();
        (assign42840_e56139, ((-(0.3333333333333333 * ((var_x_s_dn4 * assign42840_e56135) + (var_x_s * (-(0.25 * var_x_s_dn4)))))) / (2.0 * assign42840_e56139)), ((-(0.3333333333333333 * ((var_x_s_dn6 * assign42840_e56135) + (var_x_s * (-(0.25 * var_x_s_dn6)))))) / (2.0 * assign42840_e56139)), ((-(0.3333333333333333 * ((var_x_s_dn7 * assign42840_e56135) + (var_x_s * (-(0.25 * var_x_s_dn7)))))) / (2.0 * assign42840_e56139)), ((-(0.3333333333333333 * ((var_x_s_dn8 * assign42840_e56135) + (var_x_s * (-(0.25 * var_x_s_dn8)))))) / (2.0 * assign42840_e56139)), ((-(0.3333333333333333 * ((var_x_s_dn9 * assign42840_e56135) + (var_x_s * (-(0.25 * var_x_s_dn9)))))) / (2.0 * assign42840_e56139)),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign42840_e56141;
        var_temp__blk949_dn4 = assign42840_e56141_d_n4;
        var_temp__blk949_dn6 = assign42840_e56141_d_n6;
        var_temp__blk949_dn7 = assign42840_e56141_d_n7;
        var_temp__blk949_dn8 = assign42840_e56141_d_n8;
        var_temp__blk949_dn9 = assign42840_e56141_d_n9;

        let (assign42850_e56151, assign42850_e56151_d_n4, assign42850_e56151_d_n6, assign42850_e56151_d_n7, assign42850_e56151_d_n8, assign42850_e56151_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1208 != 0.0)) {
        let assign42850_e56148: f64 = (var_x_s * var_temp__blk949);
        let assign42850_e56149: f64 = (0.7071067811865475 * assign42850_e56148);
        (assign42850_e56149, (0.7071067811865475 * ((var_x_s_dn4 * var_temp__blk949) + (var_x_s * var_temp__blk949_dn4))), (0.7071067811865475 * ((var_x_s_dn6 * var_temp__blk949) + (var_x_s * var_temp__blk949_dn6))), (0.7071067811865475 * ((var_x_s_dn7 * var_temp__blk949) + (var_x_s * var_temp__blk949_dn7))), (0.7071067811865475 * ((var_x_s_dn8 * var_temp__blk949) + (var_x_s * var_temp__blk949_dn8))), (0.7071067811865475 * ((var_x_s_dn9 * var_temp__blk949) + (var_x_s * var_temp__blk949_dn9))),)
    } else {
        (var_sqs, var_sqs_dn4, var_sqs_dn6, var_sqs_dn7, var_sqs_dn8, var_sqs_dn9,)
    }
};
        var_sqs = assign42850_e56151;
        var_sqs_dn4 = assign42850_e56151_d_n4;
        var_sqs_dn6 = assign42850_e56151_d_n6;
        var_sqs_dn7 = assign42850_e56151_d_n7;
        var_sqs_dn8 = assign42850_e56151_d_n8;
        var_sqs_dn9 = assign42850_e56151_d_n9;

        *var_alphas_slot = var_alphas;
        *var_alphas_dn4_slot = var_alphas_dn4;
        *var_alphas_dn6_slot = var_alphas_dn6;
        *var_alphas_dn7_slot = var_alphas_dn7;
        *var_alphas_dn8_slot = var_alphas_dn8;
        *var_alphas_dn9_slot = var_alphas_dn9;
        *var_delta_1s_slot = var_delta_1s;
        *var_delta_1s_dn4_slot = var_delta_1s_dn4;
        *var_delta_1s_dn6_slot = var_delta_1s_dn6;
        *var_delta_1s_dn7_slot = var_delta_1s_dn7;
        *var_delta_1s_dn8_slot = var_delta_1s_dn8;
        *var_delta_1s_dn9_slot = var_delta_1s_dn9;
        *var_ds_slot = var_ds;
        *var_ds_dn4_slot = var_ds_dn4;
        *var_ds_dn6_slot = var_ds_dn6;
        *var_ds_dn7_slot = var_ds_dn7;
        *var_ds_dn8_slot = var_ds_dn8;
        *var_ds_dn9_slot = var_ds_dn9;
        *var_es_slot = var_es;
        *var_es_dn4_slot = var_es_dn4;
        *var_es_dn6_slot = var_es_dn6;
        *var_es_dn7_slot = var_es_dn7;
        *var_es_dn8_slot = var_es_dn8;
        *var_es_dn9_slot = var_es_dn9;
        *var_factheta_slot = var_factheta;
        *var_factheta_dn4_slot = var_factheta_dn4;
        *var_factheta_dn6_slot = var_factheta_dn6;
        *var_factheta_dn7_slot = var_factheta_dn7;
        *var_factheta_dn8_slot = var_factheta_dn8;
        *var_factheta_dn9_slot = var_factheta_dn9;
        *var_gmobs_slot = var_gmobs;
        *var_gmobs_dn4_slot = var_gmobs_dn4;
        *var_gmobs_dn6_slot = var_gmobs_dn6;
        *var_gmobs_dn7_slot = var_gmobs_dn7;
        *var_gmobs_dn8_slot = var_gmobs_dn8;
        *var_gmobs_dn9_slot = var_gmobs_dn9;
        *var_guard1205_slot = var_guard1205;
        *var_guard1206_slot = var_guard1206;
        *var_guard1207_slot = var_guard1207;
        *var_guard1208_slot = var_guard1208;
        *var_ps_slot = var_ps;
        *var_ps_dn4_slot = var_ps_dn4;
        *var_ps_dn6_slot = var_ps_dn6;
        *var_ps_dn7_slot = var_ps_dn7;
        *var_ps_dn8_slot = var_ps_dn8;
        *var_ps_dn9_slot = var_ps_dn9;
        *var_qbs_slot = var_qbs;
        *var_qbs_dn4_slot = var_qbs_dn4;
        *var_qbs_dn6_slot = var_qbs_dn6;
        *var_qbs_dn7_slot = var_qbs_dn7;
        *var_qbs_dn8_slot = var_qbs_dn8;
        *var_qbs_dn9_slot = var_qbs_dn9;
        *var_qis_slot = var_qis;
        *var_qis_dn4_slot = var_qis_dn4;
        *var_qis_dn6_slot = var_qis_dn6;
        *var_qis_dn7_slot = var_qis_dn7;
        *var_qis_dn8_slot = var_qis_dn8;
        *var_qis_dn9_slot = var_qis_dn9;
        *var_rhob_slot = var_rhob;
        *var_rhob_dn4_slot = var_rhob_dn4;
        *var_rhob_dn6_slot = var_rhob_dn6;
        *var_rhob_dn7_slot = var_rhob_dn7;
        *var_rhob_dn8_slot = var_rhob_dn8;
        *var_rhob_dn9_slot = var_rhob_dn9;
        *var_rhog_slot = var_rhog;
        *var_rhog_dn4_slot = var_rhog_dn4;
        *var_rhog_dn6_slot = var_rhog_dn6;
        *var_rhog_dn7_slot = var_rhog_dn7;
        *var_rhog_dn8_slot = var_rhog_dn8;
        *var_rhog_dn9_slot = var_rhog_dn9;
        *var_rxcor_slot = var_rxcor;
        *var_rxcor_dn4_slot = var_rxcor_dn4;
        *var_rxcor_dn6_slot = var_rxcor_dn6;
        *var_rxcor_dn7_slot = var_rxcor_dn7;
        *var_rxcor_dn8_slot = var_rxcor_dn8;
        *var_rxcor_dn9_slot = var_rxcor_dn9;
        *var_sp_s_temp_slot = var_sp_s_temp;
        *var_sp_s_temp_dn4_slot = var_sp_s_temp_dn4;
        *var_sp_s_temp_dn6_slot = var_sp_s_temp_dn6;
        *var_sp_s_temp_dn7_slot = var_sp_s_temp_dn7;
        *var_sp_s_temp_dn8_slot = var_sp_s_temp_dn8;
        *var_sp_s_temp_dn9_slot = var_sp_s_temp_dn9;
        *var_sqs_slot = var_sqs;
        *var_sqs_dn4_slot = var_sqs_dn4;
        *var_sqs_dn6_slot = var_sqs_dn6;
        *var_sqs_dn7_slot = var_sqs_dn7;
        *var_sqs_dn8_slot = var_sqs_dn8;
        *var_sqs_dn9_slot = var_sqs_dn9;
        *var_temp__blk949_slot = var_temp__blk949;
        *var_temp__blk949_dn4_slot = var_temp__blk949_dn4;
        *var_temp__blk949_dn6_slot = var_temp__blk949_dn6;
        *var_temp__blk949_dn7_slot = var_temp__blk949_dn7;
        *var_temp__blk949_dn8_slot = var_temp__blk949_dn8;
        *var_temp__blk949_dn9_slot = var_temp__blk949_dn9;
        *var_x_s_slot = var_x_s;
        *var_x_s_dn4_slot = var_x_s_dn4;
        *var_x_s_dn6_slot = var_x_s_dn6;
        *var_x_s_dn7_slot = var_x_s_dn7;
        *var_x_s_dn8_slot = var_x_s_dn8;
        *var_x_s_dn9_slot = var_x_s_dn9;
        *var_xgs_slot = var_xgs;
        *var_xgs_dn4_slot = var_xgs_dn4;
        *var_xgs_dn6_slot = var_xgs_dn6;
        *var_xgs_dn7_slot = var_xgs_dn7;
        *var_xgs_dn8_slot = var_xgs_dn8;
        *var_xgs_dn9_slot = var_xgs_dn9;
        *var_xi0s_slot = var_xi0s;
        *var_xi0s_dn4_slot = var_xi0s_dn4;
        *var_xi0s_dn6_slot = var_xi0s_dn6;
        *var_xi0s_dn7_slot = var_xi0s_dn7;
        *var_xi0s_dn8_slot = var_xi0s_dn8;
        *var_xi0s_dn9_slot = var_xi0s_dn9;
        *var_xi1s_slot = var_xi1s;
        *var_xi1s_dn4_slot = var_xi1s_dn4;
        *var_xi1s_dn6_slot = var_xi1s_dn6;
        *var_xi1s_dn7_slot = var_xi1s_dn7;
        *var_xi1s_dn8_slot = var_xi1s_dn8;
        *var_xi1s_dn9_slot = var_xi1s_dn9;
        *var_xi2s_slot = var_xi2s;
        *var_xi2s_dn4_slot = var_xi2s_dn4;
        *var_xi2s_dn6_slot = var_xi2s_dn6;
        *var_xi2s_dn7_slot = var_xi2s_dn7;
        *var_xi2s_dn8_slot = var_xi2s_dn8;
        *var_xi2s_dn9_slot = var_xi2s_dn9;
        *var_xitsb_slot = var_xitsb;
        *var_xitsb_dn4_slot = var_xitsb_dn4;
        *var_xitsb_dn6_slot = var_xitsb_dn6;
        *var_xitsb_dn7_slot = var_xitsb_dn7;
        *var_xitsb_dn8_slot = var_xitsb_dn8;
        *var_xitsb_dn9_slot = var_xitsb_dn9;
    }

    pub(super) fn stamp_transient_block_93(
        var_cs_t: f64,
        var_cs_t_dn4: f64,
        var_ds: f64,
        var_ds_dn4: f64,
        var_ds_dn6: f64,
        var_ds_dn7: f64,
        var_ds_dn8: f64,
        var_ds_dn9: f64,
        var_e_eff0: f64,
        var_es: f64,
        var_es_dn4: f64,
        var_es_dn6: f64,
        var_es_dn7: f64,
        var_es_dn8: f64,
        var_es_dn9: f64,
        var_eta_mu: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn4: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf2_dn9: f64,
        var_gf_dn4: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_gf_dn9: f64,
        var_guard1205: f64,
        var_guard1208: f64,
        var_inv_gf2: f64,
        var_inv_gf2_dn4: f64,
        var_inv_gf2_dn6: f64,
        var_inv_gf2_dn7: f64,
        var_inv_gf2_dn8: f64,
        var_inv_gf2_dn9: f64,
        var_inv_phit1: f64,
        var_inv_phit1_dn4: f64,
        var_inv_phit1_dn6: f64,
        var_inv_phit1_dn7: f64,
        var_inv_phit1_dn8: f64,
        var_inv_phit1_dn9: f64,
        var_inv_xi: f64,
        var_inv_xi_dn4: f64,
        var_inv_xi_dn6: f64,
        var_inv_xi_dn7: f64,
        var_inv_xi_dn8: f64,
        var_inv_xi_dn9: f64,
        var_margin: f64,
        var_mue_t: f64,
        var_mue_t_dn4: f64,
        var_phit1: f64,
        var_phit1_dn4: f64,
        var_phit1_dn6: f64,
        var_phit1_dn7: f64,
        var_phit1_dn8: f64,
        var_phit1_dn9: f64,
        var_rsb_i: f64,
        var_rsg_i: f64,
        var_sp_s_x1: f64,
        var_sp_s_x1_dn4: f64,
        var_sp_s_x1_dn6: f64,
        var_sp_s_x1_dn7: f64,
        var_sp_s_x1_dn8: f64,
        var_sp_s_x1_dn9: f64,
        var_temp__blk949: f64,
        var_temp__blk949_dn4: f64,
        var_temp__blk949_dn6: f64,
        var_temp__blk949_dn7: f64,
        var_temp__blk949_dn8: f64,
        var_temp__blk949_dn9: f64,
        var_thecs_t: f64,
        var_thecs_t_dn4: f64,
        var_themu_t: f64,
        var_themu_t_dn4: f64,
        var_ther_i: f64,
        var_ther_i_dn4: f64,
        var_thesatb_i: f64,
        var_thesatg_i: f64,
        var_thesatt_i: f64,
        var_vgb1: f64,
        var_vgb1_dn4: f64,
        var_vgb1_dn6: f64,
        var_vgb1_dn7: f64,
        var_vgb1_dn8: f64,
        var_vgb1_dn9: f64,
        var_vsbx: f64,
        var_vsbx_dn4: f64,
        var_vsbx_dn6: f64,
        var_vsbx_dn7: f64,
        var_vsbx_dn8: f64,
        var_vsbx_dn9: f64,
        var_x_s: f64,
        var_x_s_dn4: f64,
        var_x_s_dn6: f64,
        var_x_s_dn7: f64,
        var_x_s_dn8: f64,
        var_x_s_dn9: f64,
        var_xcor_t: f64,
        var_xcor_t_dn4: f64,
        var_xg: f64,
        var_xg_dn4: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xg_dn9: f64,
        var_xi: f64,
        var_xi_dn4: f64,
        var_xi_dn6: f64,
        var_xi_dn7: f64,
        var_xi_dn8: f64,
        var_xi_dn9: f64,
        var_xn_s: f64,
        var_xn_s_dn4: f64,
        var_xn_s_dn6: f64,
        var_xn_s_dn7: f64,
        var_xn_s_dn8: f64,
        var_xn_s_dn9: f64,
        var_xno_s: f64,
        var_xno_s_dn4: f64,
        var_xno_s_dn6: f64,
        var_xno_s_dn7: f64,
        var_xno_s_dn8: f64,
        var_xno_s_dn9: f64,
        var_alphas_slot: &mut f64,
        var_alphas_dn4_slot: &mut f64,
        var_alphas_dn6_slot: &mut f64,
        var_alphas_dn7_slot: &mut f64,
        var_alphas_dn8_slot: &mut f64,
        var_alphas_dn9_slot: &mut f64,
        var_eeffs_slot: &mut f64,
        var_eeffs_dn4_slot: &mut f64,
        var_eeffs_dn6_slot: &mut f64,
        var_eeffs_dn7_slot: &mut f64,
        var_eeffs_dn8_slot: &mut f64,
        var_eeffs_dn9_slot: &mut f64,
        var_factheta_slot: &mut f64,
        var_factheta_dn4_slot: &mut f64,
        var_factheta_dn6_slot: &mut f64,
        var_factheta_dn7_slot: &mut f64,
        var_factheta_dn8_slot: &mut f64,
        var_factheta_dn9_slot: &mut f64,
        var_gf2_dc_slot: &mut f64,
        var_gf2_dc_dn4_slot: &mut f64,
        var_gf2_dc_dn6_slot: &mut f64,
        var_gf2_dc_dn7_slot: &mut f64,
        var_gf2_dc_dn8_slot: &mut f64,
        var_gf2_dc_dn9_slot: &mut f64,
        var_gf_dc_slot: &mut f64,
        var_gf_dc_dn4_slot: &mut f64,
        var_gf_dc_dn6_slot: &mut f64,
        var_gf_dc_dn7_slot: &mut f64,
        var_gf_dc_dn8_slot: &mut f64,
        var_gf_dc_dn9_slot: &mut f64,
        var_gmobs_slot: &mut f64,
        var_gmobs_dn4_slot: &mut f64,
        var_gmobs_dn6_slot: &mut f64,
        var_gmobs_dn7_slot: &mut f64,
        var_gmobs_dn8_slot: &mut f64,
        var_gmobs_dn9_slot: &mut f64,
        var_gr_slot: &mut f64,
        var_gr_dn4_slot: &mut f64,
        var_gr_dn6_slot: &mut f64,
        var_gr_dn7_slot: &mut f64,
        var_gr_dn8_slot: &mut f64,
        var_gr_dn9_slot: &mut f64,
        var_guard1209_slot: &mut f64,
        var_guard1210_slot: &mut f64,
        var_guard1211_slot: &mut f64,
        var_guard1212_slot: &mut f64,
        var_guard1213_slot: &mut f64,
        var_inv_gf2_dc_slot: &mut f64,
        var_inv_gf2_dc_dn4_slot: &mut f64,
        var_inv_gf2_dc_dn6_slot: &mut f64,
        var_inv_gf2_dc_dn7_slot: &mut f64,
        var_inv_gf2_dc_dn8_slot: &mut f64,
        var_inv_gf2_dc_dn9_slot: &mut f64,
        var_inv_phit1_dc_slot: &mut f64,
        var_inv_phit1_dc_dn4_slot: &mut f64,
        var_inv_phit1_dc_dn6_slot: &mut f64,
        var_inv_phit1_dc_dn7_slot: &mut f64,
        var_inv_phit1_dc_dn8_slot: &mut f64,
        var_inv_phit1_dc_dn9_slot: &mut f64,
        var_inv_xi_dc_slot: &mut f64,
        var_inv_xi_dc_dn4_slot: &mut f64,
        var_inv_xi_dc_dn6_slot: &mut f64,
        var_inv_xi_dc_dn7_slot: &mut f64,
        var_inv_xi_dc_dn8_slot: &mut f64,
        var_inv_xi_dc_dn9_slot: &mut f64,
        var_margin_dc_slot: &mut f64,
        var_mutmp_slot: &mut f64,
        var_mutmp_dn4_slot: &mut f64,
        var_mutmp_dn6_slot: &mut f64,
        var_mutmp_dn7_slot: &mut f64,
        var_mutmp_dn8_slot: &mut f64,
        var_mutmp_dn9_slot: &mut f64,
        var_phit1_dc_slot: &mut f64,
        var_phit1_dc_dn4_slot: &mut f64,
        var_phit1_dc_dn6_slot: &mut f64,
        var_phit1_dc_dn7_slot: &mut f64,
        var_phit1_dc_dn8_slot: &mut f64,
        var_phit1_dc_dn9_slot: &mut f64,
        var_ps_slot: &mut f64,
        var_ps_dn4_slot: &mut f64,
        var_ps_dn6_slot: &mut f64,
        var_ps_dn7_slot: &mut f64,
        var_ps_dn8_slot: &mut f64,
        var_ps_dn9_slot: &mut f64,
        var_qbs_slot: &mut f64,
        var_qbs_dn4_slot: &mut f64,
        var_qbs_dn6_slot: &mut f64,
        var_qbs_dn7_slot: &mut f64,
        var_qbs_dn8_slot: &mut f64,
        var_qbs_dn9_slot: &mut f64,
        var_qis_slot: &mut f64,
        var_qis_dn4_slot: &mut f64,
        var_qis_dn6_slot: &mut f64,
        var_qis_dn7_slot: &mut f64,
        var_qis_dn8_slot: &mut f64,
        var_qis_dn9_slot: &mut f64,
        var_rhob_slot: &mut f64,
        var_rhob_dn4_slot: &mut f64,
        var_rhob_dn6_slot: &mut f64,
        var_rhob_dn7_slot: &mut f64,
        var_rhob_dn8_slot: &mut f64,
        var_rhob_dn9_slot: &mut f64,
        var_rhog_slot: &mut f64,
        var_rhog_dn4_slot: &mut f64,
        var_rhog_dn6_slot: &mut f64,
        var_rhog_dn7_slot: &mut f64,
        var_rhog_dn8_slot: &mut f64,
        var_rhog_dn9_slot: &mut f64,
        var_rxcor_slot: &mut f64,
        var_rxcor_dn4_slot: &mut f64,
        var_rxcor_dn6_slot: &mut f64,
        var_rxcor_dn7_slot: &mut f64,
        var_rxcor_dn8_slot: &mut f64,
        var_rxcor_dn9_slot: &mut f64,
        var_sp_s_x1_dc_slot: &mut f64,
        var_sp_s_x1_dc_dn4_slot: &mut f64,
        var_sp_s_x1_dc_dn6_slot: &mut f64,
        var_sp_s_x1_dc_dn7_slot: &mut f64,
        var_sp_s_x1_dc_dn8_slot: &mut f64,
        var_sp_s_x1_dc_dn9_slot: &mut f64,
        var_sqs_slot: &mut f64,
        var_sqs_dn4_slot: &mut f64,
        var_sqs_dn6_slot: &mut f64,
        var_sqs_dn7_slot: &mut f64,
        var_sqs_dn8_slot: &mut f64,
        var_sqs_dn9_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_vgb1_dc_slot: &mut f64,
        var_vgb1_dc_dn4_slot: &mut f64,
        var_vgb1_dc_dn6_slot: &mut f64,
        var_vgb1_dc_dn7_slot: &mut f64,
        var_vgb1_dc_dn8_slot: &mut f64,
        var_vgb1_dc_dn9_slot: &mut f64,
        var_vsbx_dc_slot: &mut f64,
        var_vsbx_dc_dn4_slot: &mut f64,
        var_vsbx_dc_dn6_slot: &mut f64,
        var_vsbx_dc_dn7_slot: &mut f64,
        var_vsbx_dc_dn8_slot: &mut f64,
        var_vsbx_dc_dn9_slot: &mut f64,
        var_wsat_slot: &mut f64,
        var_wsat_dn4_slot: &mut f64,
        var_wsat_dn6_slot: &mut f64,
        var_wsat_dn7_slot: &mut f64,
        var_wsat_dn8_slot: &mut f64,
        var_wsat_dn9_slot: &mut f64,
        var_xg_dc_slot: &mut f64,
        var_xg_dc_dn4_slot: &mut f64,
        var_xg_dc_dn6_slot: &mut f64,
        var_xg_dc_dn7_slot: &mut f64,
        var_xg_dc_dn8_slot: &mut f64,
        var_xg_dc_dn9_slot: &mut f64,
        var_xgs_slot: &mut f64,
        var_xgs_dn4_slot: &mut f64,
        var_xgs_dn6_slot: &mut f64,
        var_xgs_dn7_slot: &mut f64,
        var_xgs_dn8_slot: &mut f64,
        var_xgs_dn9_slot: &mut f64,
        var_xi_dc_slot: &mut f64,
        var_xi_dc_dn4_slot: &mut f64,
        var_xi_dc_dn6_slot: &mut f64,
        var_xi_dc_dn7_slot: &mut f64,
        var_xi_dc_dn8_slot: &mut f64,
        var_xi_dc_dn9_slot: &mut f64,
        var_xitsb_slot: &mut f64,
        var_xitsb_dn4_slot: &mut f64,
        var_xitsb_dn6_slot: &mut f64,
        var_xitsb_dn7_slot: &mut f64,
        var_xitsb_dn8_slot: &mut f64,
        var_xitsb_dn9_slot: &mut f64,
        var_xn_s_dc_slot: &mut f64,
        var_xn_s_dc_dn4_slot: &mut f64,
        var_xn_s_dc_dn6_slot: &mut f64,
        var_xn_s_dc_dn7_slot: &mut f64,
        var_xn_s_dc_dn8_slot: &mut f64,
        var_xn_s_dc_dn9_slot: &mut f64,
        var_xno_s_dc_slot: &mut f64,
        var_xno_s_dc_dn4_slot: &mut f64,
        var_xno_s_dc_dn6_slot: &mut f64,
        var_xno_s_dc_dn7_slot: &mut f64,
        var_xno_s_dc_dn8_slot: &mut f64,
        var_xno_s_dc_dn9_slot: &mut f64,
    ) {
        let mut var_alphas: f64 = *var_alphas_slot;
        let mut var_alphas_dn4: f64 = *var_alphas_dn4_slot;
        let mut var_alphas_dn6: f64 = *var_alphas_dn6_slot;
        let mut var_alphas_dn7: f64 = *var_alphas_dn7_slot;
        let mut var_alphas_dn8: f64 = *var_alphas_dn8_slot;
        let mut var_alphas_dn9: f64 = *var_alphas_dn9_slot;
        let mut var_eeffs: f64 = *var_eeffs_slot;
        let mut var_eeffs_dn4: f64 = *var_eeffs_dn4_slot;
        let mut var_eeffs_dn6: f64 = *var_eeffs_dn6_slot;
        let mut var_eeffs_dn7: f64 = *var_eeffs_dn7_slot;
        let mut var_eeffs_dn8: f64 = *var_eeffs_dn8_slot;
        let mut var_eeffs_dn9: f64 = *var_eeffs_dn9_slot;
        let mut var_factheta: f64 = *var_factheta_slot;
        let mut var_factheta_dn4: f64 = *var_factheta_dn4_slot;
        let mut var_factheta_dn6: f64 = *var_factheta_dn6_slot;
        let mut var_factheta_dn7: f64 = *var_factheta_dn7_slot;
        let mut var_factheta_dn8: f64 = *var_factheta_dn8_slot;
        let mut var_factheta_dn9: f64 = *var_factheta_dn9_slot;
        let mut var_gf2_dc: f64 = *var_gf2_dc_slot;
        let mut var_gf2_dc_dn4: f64 = *var_gf2_dc_dn4_slot;
        let mut var_gf2_dc_dn6: f64 = *var_gf2_dc_dn6_slot;
        let mut var_gf2_dc_dn7: f64 = *var_gf2_dc_dn7_slot;
        let mut var_gf2_dc_dn8: f64 = *var_gf2_dc_dn8_slot;
        let mut var_gf2_dc_dn9: f64 = *var_gf2_dc_dn9_slot;
        let mut var_gf_dc: f64 = *var_gf_dc_slot;
        let mut var_gf_dc_dn4: f64 = *var_gf_dc_dn4_slot;
        let mut var_gf_dc_dn6: f64 = *var_gf_dc_dn6_slot;
        let mut var_gf_dc_dn7: f64 = *var_gf_dc_dn7_slot;
        let mut var_gf_dc_dn8: f64 = *var_gf_dc_dn8_slot;
        let mut var_gf_dc_dn9: f64 = *var_gf_dc_dn9_slot;
        let mut var_gmobs: f64 = *var_gmobs_slot;
        let mut var_gmobs_dn4: f64 = *var_gmobs_dn4_slot;
        let mut var_gmobs_dn6: f64 = *var_gmobs_dn6_slot;
        let mut var_gmobs_dn7: f64 = *var_gmobs_dn7_slot;
        let mut var_gmobs_dn8: f64 = *var_gmobs_dn8_slot;
        let mut var_gmobs_dn9: f64 = *var_gmobs_dn9_slot;
        let mut var_gr: f64 = *var_gr_slot;
        let mut var_gr_dn4: f64 = *var_gr_dn4_slot;
        let mut var_gr_dn6: f64 = *var_gr_dn6_slot;
        let mut var_gr_dn7: f64 = *var_gr_dn7_slot;
        let mut var_gr_dn8: f64 = *var_gr_dn8_slot;
        let mut var_gr_dn9: f64 = *var_gr_dn9_slot;
        let mut var_guard1209: f64 = *var_guard1209_slot;
        let mut var_guard1210: f64 = *var_guard1210_slot;
        let mut var_guard1211: f64 = *var_guard1211_slot;
        let mut var_guard1212: f64 = *var_guard1212_slot;
        let mut var_guard1213: f64 = *var_guard1213_slot;
        let mut var_inv_gf2_dc: f64 = *var_inv_gf2_dc_slot;
        let mut var_inv_gf2_dc_dn4: f64 = *var_inv_gf2_dc_dn4_slot;
        let mut var_inv_gf2_dc_dn6: f64 = *var_inv_gf2_dc_dn6_slot;
        let mut var_inv_gf2_dc_dn7: f64 = *var_inv_gf2_dc_dn7_slot;
        let mut var_inv_gf2_dc_dn8: f64 = *var_inv_gf2_dc_dn8_slot;
        let mut var_inv_gf2_dc_dn9: f64 = *var_inv_gf2_dc_dn9_slot;
        let mut var_inv_phit1_dc: f64 = *var_inv_phit1_dc_slot;
        let mut var_inv_phit1_dc_dn4: f64 = *var_inv_phit1_dc_dn4_slot;
        let mut var_inv_phit1_dc_dn6: f64 = *var_inv_phit1_dc_dn6_slot;
        let mut var_inv_phit1_dc_dn7: f64 = *var_inv_phit1_dc_dn7_slot;
        let mut var_inv_phit1_dc_dn8: f64 = *var_inv_phit1_dc_dn8_slot;
        let mut var_inv_phit1_dc_dn9: f64 = *var_inv_phit1_dc_dn9_slot;
        let mut var_inv_xi_dc: f64 = *var_inv_xi_dc_slot;
        let mut var_inv_xi_dc_dn4: f64 = *var_inv_xi_dc_dn4_slot;
        let mut var_inv_xi_dc_dn6: f64 = *var_inv_xi_dc_dn6_slot;
        let mut var_inv_xi_dc_dn7: f64 = *var_inv_xi_dc_dn7_slot;
        let mut var_inv_xi_dc_dn8: f64 = *var_inv_xi_dc_dn8_slot;
        let mut var_inv_xi_dc_dn9: f64 = *var_inv_xi_dc_dn9_slot;
        let mut var_margin_dc: f64 = *var_margin_dc_slot;
        let mut var_mutmp: f64 = *var_mutmp_slot;
        let mut var_mutmp_dn4: f64 = *var_mutmp_dn4_slot;
        let mut var_mutmp_dn6: f64 = *var_mutmp_dn6_slot;
        let mut var_mutmp_dn7: f64 = *var_mutmp_dn7_slot;
        let mut var_mutmp_dn8: f64 = *var_mutmp_dn8_slot;
        let mut var_mutmp_dn9: f64 = *var_mutmp_dn9_slot;
        let mut var_phit1_dc: f64 = *var_phit1_dc_slot;
        let mut var_phit1_dc_dn4: f64 = *var_phit1_dc_dn4_slot;
        let mut var_phit1_dc_dn6: f64 = *var_phit1_dc_dn6_slot;
        let mut var_phit1_dc_dn7: f64 = *var_phit1_dc_dn7_slot;
        let mut var_phit1_dc_dn8: f64 = *var_phit1_dc_dn8_slot;
        let mut var_phit1_dc_dn9: f64 = *var_phit1_dc_dn9_slot;
        let mut var_ps: f64 = *var_ps_slot;
        let mut var_ps_dn4: f64 = *var_ps_dn4_slot;
        let mut var_ps_dn6: f64 = *var_ps_dn6_slot;
        let mut var_ps_dn7: f64 = *var_ps_dn7_slot;
        let mut var_ps_dn8: f64 = *var_ps_dn8_slot;
        let mut var_ps_dn9: f64 = *var_ps_dn9_slot;
        let mut var_qbs: f64 = *var_qbs_slot;
        let mut var_qbs_dn4: f64 = *var_qbs_dn4_slot;
        let mut var_qbs_dn6: f64 = *var_qbs_dn6_slot;
        let mut var_qbs_dn7: f64 = *var_qbs_dn7_slot;
        let mut var_qbs_dn8: f64 = *var_qbs_dn8_slot;
        let mut var_qbs_dn9: f64 = *var_qbs_dn9_slot;
        let mut var_qis: f64 = *var_qis_slot;
        let mut var_qis_dn4: f64 = *var_qis_dn4_slot;
        let mut var_qis_dn6: f64 = *var_qis_dn6_slot;
        let mut var_qis_dn7: f64 = *var_qis_dn7_slot;
        let mut var_qis_dn8: f64 = *var_qis_dn8_slot;
        let mut var_qis_dn9: f64 = *var_qis_dn9_slot;
        let mut var_rhob: f64 = *var_rhob_slot;
        let mut var_rhob_dn4: f64 = *var_rhob_dn4_slot;
        let mut var_rhob_dn6: f64 = *var_rhob_dn6_slot;
        let mut var_rhob_dn7: f64 = *var_rhob_dn7_slot;
        let mut var_rhob_dn8: f64 = *var_rhob_dn8_slot;
        let mut var_rhob_dn9: f64 = *var_rhob_dn9_slot;
        let mut var_rhog: f64 = *var_rhog_slot;
        let mut var_rhog_dn4: f64 = *var_rhog_dn4_slot;
        let mut var_rhog_dn6: f64 = *var_rhog_dn6_slot;
        let mut var_rhog_dn7: f64 = *var_rhog_dn7_slot;
        let mut var_rhog_dn8: f64 = *var_rhog_dn8_slot;
        let mut var_rhog_dn9: f64 = *var_rhog_dn9_slot;
        let mut var_rxcor: f64 = *var_rxcor_slot;
        let mut var_rxcor_dn4: f64 = *var_rxcor_dn4_slot;
        let mut var_rxcor_dn6: f64 = *var_rxcor_dn6_slot;
        let mut var_rxcor_dn7: f64 = *var_rxcor_dn7_slot;
        let mut var_rxcor_dn8: f64 = *var_rxcor_dn8_slot;
        let mut var_rxcor_dn9: f64 = *var_rxcor_dn9_slot;
        let mut var_sp_s_x1_dc: f64 = *var_sp_s_x1_dc_slot;
        let mut var_sp_s_x1_dc_dn4: f64 = *var_sp_s_x1_dc_dn4_slot;
        let mut var_sp_s_x1_dc_dn6: f64 = *var_sp_s_x1_dc_dn6_slot;
        let mut var_sp_s_x1_dc_dn7: f64 = *var_sp_s_x1_dc_dn7_slot;
        let mut var_sp_s_x1_dc_dn8: f64 = *var_sp_s_x1_dc_dn8_slot;
        let mut var_sp_s_x1_dc_dn9: f64 = *var_sp_s_x1_dc_dn9_slot;
        let mut var_sqs: f64 = *var_sqs_slot;
        let mut var_sqs_dn4: f64 = *var_sqs_dn4_slot;
        let mut var_sqs_dn6: f64 = *var_sqs_dn6_slot;
        let mut var_sqs_dn7: f64 = *var_sqs_dn7_slot;
        let mut var_sqs_dn8: f64 = *var_sqs_dn8_slot;
        let mut var_sqs_dn9: f64 = *var_sqs_dn9_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_vgb1_dc: f64 = *var_vgb1_dc_slot;
        let mut var_vgb1_dc_dn4: f64 = *var_vgb1_dc_dn4_slot;
        let mut var_vgb1_dc_dn6: f64 = *var_vgb1_dc_dn6_slot;
        let mut var_vgb1_dc_dn7: f64 = *var_vgb1_dc_dn7_slot;
        let mut var_vgb1_dc_dn8: f64 = *var_vgb1_dc_dn8_slot;
        let mut var_vgb1_dc_dn9: f64 = *var_vgb1_dc_dn9_slot;
        let mut var_vsbx_dc: f64 = *var_vsbx_dc_slot;
        let mut var_vsbx_dc_dn4: f64 = *var_vsbx_dc_dn4_slot;
        let mut var_vsbx_dc_dn6: f64 = *var_vsbx_dc_dn6_slot;
        let mut var_vsbx_dc_dn7: f64 = *var_vsbx_dc_dn7_slot;
        let mut var_vsbx_dc_dn8: f64 = *var_vsbx_dc_dn8_slot;
        let mut var_vsbx_dc_dn9: f64 = *var_vsbx_dc_dn9_slot;
        let mut var_wsat: f64 = *var_wsat_slot;
        let mut var_wsat_dn4: f64 = *var_wsat_dn4_slot;
        let mut var_wsat_dn6: f64 = *var_wsat_dn6_slot;
        let mut var_wsat_dn7: f64 = *var_wsat_dn7_slot;
        let mut var_wsat_dn8: f64 = *var_wsat_dn8_slot;
        let mut var_wsat_dn9: f64 = *var_wsat_dn9_slot;
        let mut var_xg_dc: f64 = *var_xg_dc_slot;
        let mut var_xg_dc_dn4: f64 = *var_xg_dc_dn4_slot;
        let mut var_xg_dc_dn6: f64 = *var_xg_dc_dn6_slot;
        let mut var_xg_dc_dn7: f64 = *var_xg_dc_dn7_slot;
        let mut var_xg_dc_dn8: f64 = *var_xg_dc_dn8_slot;
        let mut var_xg_dc_dn9: f64 = *var_xg_dc_dn9_slot;
        let mut var_xgs: f64 = *var_xgs_slot;
        let mut var_xgs_dn4: f64 = *var_xgs_dn4_slot;
        let mut var_xgs_dn6: f64 = *var_xgs_dn6_slot;
        let mut var_xgs_dn7: f64 = *var_xgs_dn7_slot;
        let mut var_xgs_dn8: f64 = *var_xgs_dn8_slot;
        let mut var_xgs_dn9: f64 = *var_xgs_dn9_slot;
        let mut var_xi_dc: f64 = *var_xi_dc_slot;
        let mut var_xi_dc_dn4: f64 = *var_xi_dc_dn4_slot;
        let mut var_xi_dc_dn6: f64 = *var_xi_dc_dn6_slot;
        let mut var_xi_dc_dn7: f64 = *var_xi_dc_dn7_slot;
        let mut var_xi_dc_dn8: f64 = *var_xi_dc_dn8_slot;
        let mut var_xi_dc_dn9: f64 = *var_xi_dc_dn9_slot;
        let mut var_xitsb: f64 = *var_xitsb_slot;
        let mut var_xitsb_dn4: f64 = *var_xitsb_dn4_slot;
        let mut var_xitsb_dn6: f64 = *var_xitsb_dn6_slot;
        let mut var_xitsb_dn7: f64 = *var_xitsb_dn7_slot;
        let mut var_xitsb_dn8: f64 = *var_xitsb_dn8_slot;
        let mut var_xitsb_dn9: f64 = *var_xitsb_dn9_slot;
        let mut var_xn_s_dc: f64 = *var_xn_s_dc_slot;
        let mut var_xn_s_dc_dn4: f64 = *var_xn_s_dc_dn4_slot;
        let mut var_xn_s_dc_dn6: f64 = *var_xn_s_dc_dn6_slot;
        let mut var_xn_s_dc_dn7: f64 = *var_xn_s_dc_dn7_slot;
        let mut var_xn_s_dc_dn8: f64 = *var_xn_s_dc_dn8_slot;
        let mut var_xn_s_dc_dn9: f64 = *var_xn_s_dc_dn9_slot;
        let mut var_xno_s_dc: f64 = *var_xno_s_dc_slot;
        let mut var_xno_s_dc_dn4: f64 = *var_xno_s_dc_dn4_slot;
        let mut var_xno_s_dc_dn6: f64 = *var_xno_s_dc_dn6_slot;
        let mut var_xno_s_dc_dn7: f64 = *var_xno_s_dc_dn7_slot;
        let mut var_xno_s_dc_dn8: f64 = *var_xno_s_dc_dn8_slot;
        let mut var_xno_s_dc_dn9: f64 = *var_xno_s_dc_dn9_slot;

        let (assign42860_e56175, assign42860_e56175_d_n4, assign42860_e56175_d_n6, assign42860_e56175_d_n7, assign42860_e56175_d_n8, assign42860_e56175_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1208 != 0.0)) {
        let assign42860_e56161: f64 = (0.5 * var_x_s);
        let assign42860_e56162: f64 = (1.0 - assign42860_e56161);
        let assign42860_e56166: f64 = (var_x_s * var_x_s);
        let assign42860_e56167: f64 = (0.16666666666666666 * assign42860_e56166);
        let assign42860_e56168: f64 = (assign42860_e56162 + assign42860_e56167);
        let assign42860_e56169: f64 = (var_gf * assign42860_e56168);
        let assign42860_e56171: f64 = (assign42860_e56169 / var_temp__blk949);
        let assign42860_e56172: f64 = (0.7071067811865475 * assign42860_e56171);
        let assign42860_e56173: f64 = (1.0 + assign42860_e56172);
        (assign42860_e56173, (0.7071067811865475 * (((((var_gf_dn4 * assign42860_e56168) + (var_gf * ((-(0.5 * var_x_s_dn4)) + (0.16666666666666666 * ((var_x_s_dn4 * var_x_s) + (var_x_s * var_x_s_dn4)))))) * var_temp__blk949) - (assign42860_e56169 * var_temp__blk949_dn4)) / (var_temp__blk949 * var_temp__blk949))), (0.7071067811865475 * (((((var_gf_dn6 * assign42860_e56168) + (var_gf * ((-(0.5 * var_x_s_dn6)) + (0.16666666666666666 * ((var_x_s_dn6 * var_x_s) + (var_x_s * var_x_s_dn6)))))) * var_temp__blk949) - (assign42860_e56169 * var_temp__blk949_dn6)) / (var_temp__blk949 * var_temp__blk949))), (0.7071067811865475 * (((((var_gf_dn7 * assign42860_e56168) + (var_gf * ((-(0.5 * var_x_s_dn7)) + (0.16666666666666666 * ((var_x_s_dn7 * var_x_s) + (var_x_s * var_x_s_dn7)))))) * var_temp__blk949) - (assign42860_e56169 * var_temp__blk949_dn7)) / (var_temp__blk949 * var_temp__blk949))), (0.7071067811865475 * (((((var_gf_dn8 * assign42860_e56168) + (var_gf * ((-(0.5 * var_x_s_dn8)) + (0.16666666666666666 * ((var_x_s_dn8 * var_x_s) + (var_x_s * var_x_s_dn8)))))) * var_temp__blk949) - (assign42860_e56169 * var_temp__blk949_dn8)) / (var_temp__blk949 * var_temp__blk949))), (0.7071067811865475 * (((((var_gf_dn9 * assign42860_e56168) + (var_gf * ((-(0.5 * var_x_s_dn9)) + (0.16666666666666666 * ((var_x_s_dn9 * var_x_s) + (var_x_s * var_x_s_dn9)))))) * var_temp__blk949) - (assign42860_e56169 * var_temp__blk949_dn9)) / (var_temp__blk949 * var_temp__blk949))),)
    } else {
        (var_alphas, var_alphas_dn4, var_alphas_dn6, var_alphas_dn7, var_alphas_dn8, var_alphas_dn9,)
    }
};
        var_alphas = assign42860_e56175;
        var_alphas_dn4 = assign42860_e56175_d_n4;
        var_alphas_dn6 = assign42860_e56175_d_n6;
        var_alphas_dn7 = assign42860_e56175_d_n7;
        var_alphas_dn8 = assign42860_e56175_d_n8;
        var_alphas_dn9 = assign42860_e56175_d_n9;

        let (assign42870_e56186, assign42870_e56186_d_n4, assign42870_e56186_d_n6, assign42870_e56186_d_n7, assign42870_e56186_d_n8, assign42870_e56186_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1208 == 0.0)) {
        let assign42870_e56182: f64 = (var_x_s - 1.0);
        let assign42870_e56184: f64 = (assign42870_e56182 + var_es);
        (assign42870_e56184, (var_x_s_dn4 + var_es_dn4), (var_x_s_dn6 + var_es_dn6), (var_x_s_dn7 + var_es_dn7), (var_x_s_dn8 + var_es_dn8), (var_x_s_dn9 + var_es_dn9),)
    } else {
        (var_ps, var_ps_dn4, var_ps_dn6, var_ps_dn7, var_ps_dn8, var_ps_dn9,)
    }
};
        var_ps = assign42870_e56186;
        var_ps_dn4 = assign42870_e56186_d_n4;
        var_ps_dn6 = assign42870_e56186_d_n6;
        var_ps_dn7 = assign42870_e56186_d_n7;
        var_ps_dn8 = assign42870_e56186_d_n8;
        var_ps_dn9 = assign42870_e56186_d_n9;

        let (assign42880_e56194, assign42880_e56194_d_n4, assign42880_e56194_d_n6, assign42880_e56194_d_n7, assign42880_e56194_d_n8, assign42880_e56194_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1208 == 0.0)) {
        let assign42880_e56192: f64 = (var_ps).sqrt();
        (assign42880_e56192, (var_ps_dn4 / (2.0 * assign42880_e56192)), (var_ps_dn6 / (2.0 * assign42880_e56192)), (var_ps_dn7 / (2.0 * assign42880_e56192)), (var_ps_dn8 / (2.0 * assign42880_e56192)), (var_ps_dn9 / (2.0 * assign42880_e56192)),)
    } else {
        (var_sqs, var_sqs_dn4, var_sqs_dn6, var_sqs_dn7, var_sqs_dn8, var_sqs_dn9,)
    }
};
        var_sqs = assign42880_e56194;
        var_sqs_dn4 = assign42880_e56194_d_n4;
        var_sqs_dn6 = assign42880_e56194_d_n6;
        var_sqs_dn7 = assign42880_e56194_d_n7;
        var_sqs_dn8 = assign42880_e56194_d_n8;
        var_sqs_dn9 = assign42880_e56194_d_n9;

        let (assign42890_e56211, assign42890_e56211_d_n4, assign42890_e56211_d_n6, assign42890_e56211_d_n7, assign42890_e56211_d_n8, assign42890_e56211_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1208 == 0.0)) {
        let assign42890_e56204: f64 = (1.0 - var_es);
        let assign42890_e56205: f64 = (var_gf * assign42890_e56204);
        let assign42890_e56207: f64 = (assign42890_e56205 / var_sqs);
        let assign42890_e56208: f64 = (0.5 * assign42890_e56207);
        let assign42890_e56209: f64 = (1.0 + assign42890_e56208);
        (assign42890_e56209, (0.5 * (((((var_gf_dn4 * assign42890_e56204) + (var_gf * (-var_es_dn4))) * var_sqs) - (assign42890_e56205 * var_sqs_dn4)) / (var_sqs * var_sqs))), (0.5 * (((((var_gf_dn6 * assign42890_e56204) + (var_gf * (-var_es_dn6))) * var_sqs) - (assign42890_e56205 * var_sqs_dn6)) / (var_sqs * var_sqs))), (0.5 * (((((var_gf_dn7 * assign42890_e56204) + (var_gf * (-var_es_dn7))) * var_sqs) - (assign42890_e56205 * var_sqs_dn7)) / (var_sqs * var_sqs))), (0.5 * (((((var_gf_dn8 * assign42890_e56204) + (var_gf * (-var_es_dn8))) * var_sqs) - (assign42890_e56205 * var_sqs_dn8)) / (var_sqs * var_sqs))), (0.5 * (((((var_gf_dn9 * assign42890_e56204) + (var_gf * (-var_es_dn9))) * var_sqs) - (assign42890_e56205 * var_sqs_dn9)) / (var_sqs * var_sqs))),)
    } else {
        (var_alphas, var_alphas_dn4, var_alphas_dn6, var_alphas_dn7, var_alphas_dn8, var_alphas_dn9,)
    }
};
        var_alphas = assign42890_e56211;
        var_alphas_dn4 = assign42890_e56211_d_n4;
        var_alphas_dn6 = assign42890_e56211_d_n6;
        var_alphas_dn7 = assign42890_e56211_d_n7;
        var_alphas_dn8 = assign42890_e56211_d_n8;
        var_alphas_dn9 = assign42890_e56211_d_n9;

        let (assign42900_e56227, assign42900_e56227_d_n4, assign42900_e56227_d_n6, assign42900_e56227_d_n7, assign42900_e56227_d_n8, assign42900_e56227_d_n9,) = {
    if (var_guard1205 != 0.0) {
        let assign42900_e56216: f64 = (0.2 * var_xcor_t);
        let assign42900_e56218: f64 = (assign42900_e56216 * var_vsbx);
        let assign42900_e56219: f64 = (1.0 + assign42900_e56218);
        let assign42900_e56223: f64 = (var_xcor_t * var_vsbx);
        let assign42900_e56224: f64 = (1.0 + assign42900_e56223);
        let assign42900_e56225: f64 = (assign42900_e56219 / assign42900_e56224);
        (assign42900_e56225, ((((((0.2 * var_xcor_t_dn4) * var_vsbx) + (assign42900_e56216 * var_vsbx_dn4)) * assign42900_e56224) - (assign42900_e56219 * ((var_xcor_t_dn4 * var_vsbx) + (var_xcor_t * var_vsbx_dn4)))) / (assign42900_e56224 * assign42900_e56224)), ((((assign42900_e56216 * var_vsbx_dn6) * assign42900_e56224) - (assign42900_e56219 * (var_xcor_t * var_vsbx_dn6))) / (assign42900_e56224 * assign42900_e56224)), ((((assign42900_e56216 * var_vsbx_dn7) * assign42900_e56224) - (assign42900_e56219 * (var_xcor_t * var_vsbx_dn7))) / (assign42900_e56224 * assign42900_e56224)), ((((assign42900_e56216 * var_vsbx_dn8) * assign42900_e56224) - (assign42900_e56219 * (var_xcor_t * var_vsbx_dn8))) / (assign42900_e56224 * assign42900_e56224)), ((((assign42900_e56216 * var_vsbx_dn9) * assign42900_e56224) - (assign42900_e56219 * (var_xcor_t * var_vsbx_dn9))) / (assign42900_e56224 * assign42900_e56224)),)
    } else {
        (var_rxcor, var_rxcor_dn4, var_rxcor_dn6, var_rxcor_dn7, var_rxcor_dn8, var_rxcor_dn9,)
    }
};
        var_rxcor = assign42900_e56227;
        var_rxcor_dn4 = assign42900_e56227_d_n4;
        var_rxcor_dn6 = assign42900_e56227_d_n6;
        var_rxcor_dn7 = assign42900_e56227_d_n7;
        var_rxcor_dn8 = assign42900_e56227_d_n8;
        var_rxcor_dn9 = assign42900_e56227_d_n9;

        let assign42910_e56230: f64 = if var_ds > 1e-100 { 1.0 } else { 0.0 };
        var_guard1209 = assign42910_e56230;

        let (assign42920_e56241, assign42920_e56241_d_n4, assign42920_e56241_d_n6, assign42920_e56241_d_n7, assign42920_e56241_d_n8, assign42920_e56241_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) {
        let assign42920_e56237: f64 = (var_ps + var_ds);
        let assign42920_e56238: f64 = (assign42920_e56237).sqrt();
        let assign42920_e56239: f64 = (var_gf * assign42920_e56238);
        (assign42920_e56239, ((var_gf_dn4 * assign42920_e56238) + (var_gf * ((var_ps_dn4 + var_ds_dn4) / (2.0 * assign42920_e56238)))), ((var_gf_dn6 * assign42920_e56238) + (var_gf * ((var_ps_dn6 + var_ds_dn6) / (2.0 * assign42920_e56238)))), ((var_gf_dn7 * assign42920_e56238) + (var_gf * ((var_ps_dn7 + var_ds_dn7) / (2.0 * assign42920_e56238)))), ((var_gf_dn8 * assign42920_e56238) + (var_gf * ((var_ps_dn8 + var_ds_dn8) / (2.0 * assign42920_e56238)))), ((var_gf_dn9 * assign42920_e56238) + (var_gf * ((var_ps_dn9 + var_ds_dn9) / (2.0 * assign42920_e56238)))),)
    } else {
        (var_xgs, var_xgs_dn4, var_xgs_dn6, var_xgs_dn7, var_xgs_dn8, var_xgs_dn9,)
    }
};
        var_xgs = assign42920_e56241;
        var_xgs_dn4 = assign42920_e56241_d_n4;
        var_xgs_dn6 = assign42920_e56241_d_n6;
        var_xgs_dn7 = assign42920_e56241_d_n7;
        var_xgs_dn8 = assign42920_e56241_d_n8;
        var_xgs_dn9 = assign42920_e56241_d_n9;

        let (assign42930_e56257, assign42930_e56257_d_n4, assign42930_e56257_d_n6, assign42930_e56257_d_n7, assign42930_e56257_d_n8, assign42930_e56257_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) {
        let assign42930_e56247: f64 = (var_gf2 * var_ds);
        let assign42930_e56249: f64 = (assign42930_e56247 * var_phit1);
        let assign42930_e56253: f64 = (var_gf * var_sqs);
        let assign42930_e56254: f64 = (var_xgs + assign42930_e56253);
        let assign42930_e56255: f64 = (assign42930_e56249 / assign42930_e56254);
        (assign42930_e56255, (((((((var_gf2_dn4 * var_ds) + (var_gf2 * var_ds_dn4)) * var_phit1) + (assign42930_e56247 * var_phit1_dn4)) * assign42930_e56254) - (assign42930_e56249 * (var_xgs_dn4 + ((var_gf_dn4 * var_sqs) + (var_gf * var_sqs_dn4))))) / (assign42930_e56254 * assign42930_e56254)), (((((((var_gf2_dn6 * var_ds) + (var_gf2 * var_ds_dn6)) * var_phit1) + (assign42930_e56247 * var_phit1_dn6)) * assign42930_e56254) - (assign42930_e56249 * (var_xgs_dn6 + ((var_gf_dn6 * var_sqs) + (var_gf * var_sqs_dn6))))) / (assign42930_e56254 * assign42930_e56254)), (((((((var_gf2_dn7 * var_ds) + (var_gf2 * var_ds_dn7)) * var_phit1) + (assign42930_e56247 * var_phit1_dn7)) * assign42930_e56254) - (assign42930_e56249 * (var_xgs_dn7 + ((var_gf_dn7 * var_sqs) + (var_gf * var_sqs_dn7))))) / (assign42930_e56254 * assign42930_e56254)), (((((((var_gf2_dn8 * var_ds) + (var_gf2 * var_ds_dn8)) * var_phit1) + (assign42930_e56247 * var_phit1_dn8)) * assign42930_e56254) - (assign42930_e56249 * (var_xgs_dn8 + ((var_gf_dn8 * var_sqs) + (var_gf * var_sqs_dn8))))) / (assign42930_e56254 * assign42930_e56254)), (((((((var_gf2_dn9 * var_ds) + (var_gf2 * var_ds_dn9)) * var_phit1) + (assign42930_e56247 * var_phit1_dn9)) * assign42930_e56254) - (assign42930_e56249 * (var_xgs_dn9 + ((var_gf_dn9 * var_sqs) + (var_gf * var_sqs_dn9))))) / (assign42930_e56254 * assign42930_e56254)),)
    } else {
        (var_qis, var_qis_dn4, var_qis_dn6, var_qis_dn7, var_qis_dn8, var_qis_dn9,)
    }
};
        var_qis = assign42930_e56257;
        var_qis_dn4 = assign42930_e56257_d_n4;
        var_qis_dn6 = assign42930_e56257_d_n6;
        var_qis_dn7 = assign42930_e56257_d_n7;
        var_qis_dn8 = assign42930_e56257_d_n8;
        var_qis_dn9 = assign42930_e56257_d_n9;

        let (assign42940_e56267, assign42940_e56267_d_n4, assign42940_e56267_d_n6, assign42940_e56267_d_n7, assign42940_e56267_d_n8, assign42940_e56267_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) {
        let assign42940_e56263: f64 = (var_sqs * var_gf);
        let assign42940_e56265: f64 = (assign42940_e56263 * var_phit1);
        (assign42940_e56265, ((((var_sqs_dn4 * var_gf) + (var_sqs * var_gf_dn4)) * var_phit1) + (assign42940_e56263 * var_phit1_dn4)), ((((var_sqs_dn6 * var_gf) + (var_sqs * var_gf_dn6)) * var_phit1) + (assign42940_e56263 * var_phit1_dn6)), ((((var_sqs_dn7 * var_gf) + (var_sqs * var_gf_dn7)) * var_phit1) + (assign42940_e56263 * var_phit1_dn7)), ((((var_sqs_dn8 * var_gf) + (var_sqs * var_gf_dn8)) * var_phit1) + (assign42940_e56263 * var_phit1_dn8)), ((((var_sqs_dn9 * var_gf) + (var_sqs * var_gf_dn9)) * var_phit1) + (assign42940_e56263 * var_phit1_dn9)),)
    } else {
        (var_qbs, var_qbs_dn4, var_qbs_dn6, var_qbs_dn7, var_qbs_dn8, var_qbs_dn9,)
    }
};
        var_qbs = assign42940_e56267;
        var_qbs_dn4 = assign42940_e56267_d_n4;
        var_qbs_dn6 = assign42940_e56267_d_n6;
        var_qbs_dn7 = assign42940_e56267_d_n7;
        var_qbs_dn8 = assign42940_e56267_d_n8;
        var_qbs_dn9 = assign42940_e56267_d_n9;

        let assign42950_e56270: f64 = if var_rsb_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1210 = assign42950_e56270;

        let (assign42960_e56284, assign42960_e56284_d_n4, assign42960_e56284_d_n6, assign42960_e56284_d_n7, assign42960_e56284_d_n8, assign42960_e56284_d_n9,) = {
    if (((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) && (var_guard1210 != 0.0)) {
        let assign42960_e56280: f64 = (var_rsb_i * var_vsbx);
        let assign42960_e56281: f64 = (1.0 - assign42960_e56280);
        let assign42960_e56282: f64 = (1.0 / assign42960_e56281);
        (assign42960_e56282, (-((-(var_rsb_i * var_vsbx_dn4)) / (assign42960_e56281 * assign42960_e56281))), (-((-(var_rsb_i * var_vsbx_dn6)) / (assign42960_e56281 * assign42960_e56281))), (-((-(var_rsb_i * var_vsbx_dn7)) / (assign42960_e56281 * assign42960_e56281))), (-((-(var_rsb_i * var_vsbx_dn8)) / (assign42960_e56281 * assign42960_e56281))), (-((-(var_rsb_i * var_vsbx_dn9)) / (assign42960_e56281 * assign42960_e56281))),)
    } else {
        (var_rhob, var_rhob_dn4, var_rhob_dn6, var_rhob_dn7, var_rhob_dn8, var_rhob_dn9,)
    }
};
        var_rhob = assign42960_e56284;
        var_rhob_dn4 = assign42960_e56284_d_n4;
        var_rhob_dn6 = assign42960_e56284_d_n6;
        var_rhob_dn7 = assign42960_e56284_d_n7;
        var_rhob_dn8 = assign42960_e56284_d_n8;
        var_rhob_dn9 = assign42960_e56284_d_n9;

        let (assign42970_e56297, assign42970_e56297_d_n4, assign42970_e56297_d_n6, assign42970_e56297_d_n7, assign42970_e56297_d_n8, assign42970_e56297_d_n9,) = {
    if (((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) && (var_guard1210 == 0.0)) {
        let assign42970_e56294: f64 = (var_rsb_i * var_vsbx);
        let assign42970_e56295: f64 = (1.0 + assign42970_e56294);
        (assign42970_e56295, (var_rsb_i * var_vsbx_dn4), (var_rsb_i * var_vsbx_dn6), (var_rsb_i * var_vsbx_dn7), (var_rsb_i * var_vsbx_dn8), (var_rsb_i * var_vsbx_dn9),)
    } else {
        (var_rhob, var_rhob_dn4, var_rhob_dn6, var_rhob_dn7, var_rhob_dn8, var_rhob_dn9,)
    }
};
        var_rhob = assign42970_e56297;
        var_rhob_dn4 = assign42970_e56297_d_n4;
        var_rhob_dn6 = assign42970_e56297_d_n6;
        var_rhob_dn7 = assign42970_e56297_d_n7;
        var_rhob_dn8 = assign42970_e56297_d_n8;
        var_rhob_dn9 = assign42970_e56297_d_n9;

        let assign42980_e56300: f64 = if var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1211 = assign42980_e56300;

        let (assign42990_e56312, assign42990_e56312_d_n4, assign42990_e56312_d_n6, assign42990_e56312_d_n7, assign42990_e56312_d_n8, assign42990_e56312_d_n9,) = {
    if (((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) && (var_guard1211 != 0.0)) {
        let assign42990_e56309: f64 = (var_rsg_i * var_qis);
        let assign42990_e56310: f64 = (1.0 - assign42990_e56309);
        (assign42990_e56310, (-(var_rsg_i * var_qis_dn4)), (-(var_rsg_i * var_qis_dn6)), (-(var_rsg_i * var_qis_dn7)), (-(var_rsg_i * var_qis_dn8)), (-(var_rsg_i * var_qis_dn9)),)
    } else {
        (var_rhog, var_rhog_dn4, var_rhog_dn6, var_rhog_dn7, var_rhog_dn8, var_rhog_dn9,)
    }
};
        var_rhog = assign42990_e56312;
        var_rhog_dn4 = assign42990_e56312_d_n4;
        var_rhog_dn6 = assign42990_e56312_d_n6;
        var_rhog_dn7 = assign42990_e56312_d_n7;
        var_rhog_dn8 = assign42990_e56312_d_n8;
        var_rhog_dn9 = assign42990_e56312_d_n9;

        let (assign43000_e56327, assign43000_e56327_d_n4, assign43000_e56327_d_n6, assign43000_e56327_d_n7, assign43000_e56327_d_n8, assign43000_e56327_d_n9,) = {
    if (((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) && (var_guard1211 == 0.0)) {
        let assign43000_e56323: f64 = (var_rsg_i * var_qis);
        let assign43000_e56324: f64 = (1.0 + assign43000_e56323);
        let assign43000_e56325: f64 = (1.0 / assign43000_e56324);
        (assign43000_e56325, (-((var_rsg_i * var_qis_dn4) / (assign43000_e56324 * assign43000_e56324))), (-((var_rsg_i * var_qis_dn6) / (assign43000_e56324 * assign43000_e56324))), (-((var_rsg_i * var_qis_dn7) / (assign43000_e56324 * assign43000_e56324))), (-((var_rsg_i * var_qis_dn8) / (assign43000_e56324 * assign43000_e56324))), (-((var_rsg_i * var_qis_dn9) / (assign43000_e56324 * assign43000_e56324))),)
    } else {
        (var_rhog, var_rhog_dn4, var_rhog_dn6, var_rhog_dn7, var_rhog_dn8, var_rhog_dn9,)
    }
};
        var_rhog = assign43000_e56327;
        var_rhog_dn4 = assign43000_e56327_d_n4;
        var_rhog_dn6 = assign43000_e56327_d_n6;
        var_rhog_dn7 = assign43000_e56327_d_n7;
        var_rhog_dn8 = assign43000_e56327_d_n8;
        var_rhog_dn9 = assign43000_e56327_d_n9;

        let (assign43010_e56339, assign43010_e56339_d_n4, assign43010_e56339_d_n6, assign43010_e56339_d_n7, assign43010_e56339_d_n8, assign43010_e56339_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) {
        let assign43010_e56333: f64 = (var_ther_i * var_rhob);
        let assign43010_e56335: f64 = (assign43010_e56333 * var_rhog);
        let assign43010_e56337: f64 = (assign43010_e56335 * var_qis);
        (assign43010_e56337, ((((((var_ther_i_dn4 * var_rhob) + (var_ther_i * var_rhob_dn4)) * var_rhog) + (assign43010_e56333 * var_rhog_dn4)) * var_qis) + (assign43010_e56335 * var_qis_dn4)), (((((var_ther_i * var_rhob_dn6) * var_rhog) + (assign43010_e56333 * var_rhog_dn6)) * var_qis) + (assign43010_e56335 * var_qis_dn6)), (((((var_ther_i * var_rhob_dn7) * var_rhog) + (assign43010_e56333 * var_rhog_dn7)) * var_qis) + (assign43010_e56335 * var_qis_dn7)), (((((var_ther_i * var_rhob_dn8) * var_rhog) + (assign43010_e56333 * var_rhog_dn8)) * var_qis) + (assign43010_e56335 * var_qis_dn8)), (((((var_ther_i * var_rhob_dn9) * var_rhog) + (assign43010_e56333 * var_rhog_dn9)) * var_qis) + (assign43010_e56335 * var_qis_dn9)),)
    } else {
        (var_gr, var_gr_dn4, var_gr_dn6, var_gr_dn7, var_gr_dn8, var_gr_dn9,)
    }
};
        var_gr = assign43010_e56339;
        var_gr_dn4 = assign43010_e56339_d_n4;
        var_gr_dn6 = assign43010_e56339_d_n6;
        var_gr_dn7 = assign43010_e56339_d_n7;
        var_gr_dn8 = assign43010_e56339_d_n8;
        var_gr_dn9 = assign43010_e56339_d_n9;

        let (assign43020_e56351, assign43020_e56351_d_n4, assign43020_e56351_d_n6, assign43020_e56351_d_n7, assign43020_e56351_d_n8, assign43020_e56351_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) {
        let assign43020_e56347: f64 = (var_eta_mu * var_qis);
        let assign43020_e56348: f64 = (var_qbs + assign43020_e56347);
        let assign43020_e56349: f64 = (var_e_eff0 * assign43020_e56348);
        (assign43020_e56349, (var_e_eff0 * (var_qbs_dn4 + (var_eta_mu * var_qis_dn4))), (var_e_eff0 * (var_qbs_dn6 + (var_eta_mu * var_qis_dn6))), (var_e_eff0 * (var_qbs_dn7 + (var_eta_mu * var_qis_dn7))), (var_e_eff0 * (var_qbs_dn8 + (var_eta_mu * var_qis_dn8))), (var_e_eff0 * (var_qbs_dn9 + (var_eta_mu * var_qis_dn9))),)
    } else {
        (var_eeffs, var_eeffs_dn4, var_eeffs_dn6, var_eeffs_dn7, var_eeffs_dn8, var_eeffs_dn9,)
    }
};
        var_eeffs = assign43020_e56351;
        var_eeffs_dn4 = assign43020_e56351_d_n4;
        var_eeffs_dn6 = assign43020_e56351_d_n6;
        var_eeffs_dn7 = assign43020_e56351_d_n7;
        var_eeffs_dn8 = assign43020_e56351_d_n8;
        var_eeffs_dn9 = assign43020_e56351_d_n9;

        let (assign43030_e56364, assign43030_e56364_d_n4, assign43030_e56364_d_n6, assign43030_e56364_d_n7, assign43030_e56364_d_n8, assign43030_e56364_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) {
        let assign43030_e56358: f64 = (var_ps + var_ds);
        let assign43030_e56360: f64 = (assign43030_e56358 + 1e-14);
        let assign43030_e56361: f64 = (var_ps / assign43030_e56360);
        let assign43030_e56362: f64 = (assign43030_e56361).ln();
        (assign43030_e56362, ((((var_ps_dn4 * assign43030_e56360) - (var_ps * (var_ps_dn4 + var_ds_dn4))) / (assign43030_e56360 * assign43030_e56360)) / assign43030_e56361), ((((var_ps_dn6 * assign43030_e56360) - (var_ps * (var_ps_dn6 + var_ds_dn6))) / (assign43030_e56360 * assign43030_e56360)) / assign43030_e56361), ((((var_ps_dn7 * assign43030_e56360) - (var_ps * (var_ps_dn7 + var_ds_dn7))) / (assign43030_e56360 * assign43030_e56360)) / assign43030_e56361), ((((var_ps_dn8 * assign43030_e56360) - (var_ps * (var_ps_dn8 + var_ds_dn8))) / (assign43030_e56360 * assign43030_e56360)) / assign43030_e56361), ((((var_ps_dn9 * assign43030_e56360) - (var_ps * (var_ps_dn9 + var_ds_dn9))) / (assign43030_e56360 * assign43030_e56360)) / assign43030_e56361),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign43030_e56364;
        var_temp1_dn4 = assign43030_e56364_d_n4;
        var_temp1_dn6 = assign43030_e56364_d_n6;
        var_temp1_dn7 = assign43030_e56364_d_n7;
        var_temp1_dn8 = assign43030_e56364_d_n8;
        var_temp1_dn9 = assign43030_e56364_d_n9;

        let (assign43040_e56383, assign43040_e56383_d_n4, assign43040_e56383_d_n6, assign43040_e56383_d_n7, assign43040_e56383_d_n8, assign43040_e56383_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) {
        let assign43040_e56370: f64 = (var_eeffs * var_mue_t);
        let assign43040_e56372: f64 = (assign43040_e56370).powf(var_themu_t);
        let assign43040_e56376: f64 = (0.5 * var_thecs_t);
        let assign43040_e56378: f64 = (assign43040_e56376 * var_temp1);
        let assign43040_e56379: f64 = (assign43040_e56378).exp();
        let assign43040_e56380: f64 = (var_cs_t * assign43040_e56379);
        let assign43040_e56381: f64 = (assign43040_e56372 + assign43040_e56380);
        (assign43040_e56381, (if var_themu_t_dn4 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43040_e56370).powf(var_themu_t - 1.0) * ((var_eeffs_dn4 * var_mue_t) + (var_eeffs * var_mue_t_dn4)))) } } else { (assign43040_e56372 * ((var_themu_t_dn4 * (assign43040_e56370).ln()) + (var_themu_t * (((var_eeffs_dn4 * var_mue_t) + (var_eeffs * var_mue_t_dn4)) / assign43040_e56370)))) } + ((var_cs_t_dn4 * assign43040_e56379) + (var_cs_t * (assign43040_e56379 * (((0.5 * var_thecs_t_dn4) * var_temp1) + (assign43040_e56376 * var_temp1_dn4)))))), (if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43040_e56370).powf(var_themu_t - 1.0) * (var_eeffs_dn6 * var_mue_t))) } } else { (assign43040_e56372 * (var_themu_t * ((var_eeffs_dn6 * var_mue_t) / assign43040_e56370))) } + (var_cs_t * (assign43040_e56379 * (assign43040_e56376 * var_temp1_dn6)))), (if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43040_e56370).powf(var_themu_t - 1.0) * (var_eeffs_dn7 * var_mue_t))) } } else { (assign43040_e56372 * (var_themu_t * ((var_eeffs_dn7 * var_mue_t) / assign43040_e56370))) } + (var_cs_t * (assign43040_e56379 * (assign43040_e56376 * var_temp1_dn7)))), (if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43040_e56370).powf(var_themu_t - 1.0) * (var_eeffs_dn8 * var_mue_t))) } } else { (assign43040_e56372 * (var_themu_t * ((var_eeffs_dn8 * var_mue_t) / assign43040_e56370))) } + (var_cs_t * (assign43040_e56379 * (assign43040_e56376 * var_temp1_dn8)))), (if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43040_e56370).powf(var_themu_t - 1.0) * (var_eeffs_dn9 * var_mue_t))) } } else { (assign43040_e56372 * (var_themu_t * ((var_eeffs_dn9 * var_mue_t) / assign43040_e56370))) } + (var_cs_t * (assign43040_e56379 * (assign43040_e56376 * var_temp1_dn9)))),)
    } else {
        (var_mutmp, var_mutmp_dn4, var_mutmp_dn6, var_mutmp_dn7, var_mutmp_dn8, var_mutmp_dn9,)
    }
};
        var_mutmp = assign43040_e56383;
        var_mutmp_dn4 = assign43040_e56383_d_n4;
        var_mutmp_dn6 = assign43040_e56383_d_n6;
        var_mutmp_dn7 = assign43040_e56383_d_n7;
        var_mutmp_dn8 = assign43040_e56383_d_n8;
        var_mutmp_dn9 = assign43040_e56383_d_n9;

        let (assign43050_e56395, assign43050_e56395_d_n4, assign43050_e56395_d_n6, assign43050_e56395_d_n7, assign43050_e56395_d_n8, assign43050_e56395_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) {
        let assign43050_e56389: f64 = (1.0 + var_mutmp);
        let assign43050_e56391: f64 = (assign43050_e56389 + var_gr);
        let assign43050_e56393: f64 = (assign43050_e56391 * var_rxcor);
        (assign43050_e56393, (((var_mutmp_dn4 + var_gr_dn4) * var_rxcor) + (assign43050_e56391 * var_rxcor_dn4)), (((var_mutmp_dn6 + var_gr_dn6) * var_rxcor) + (assign43050_e56391 * var_rxcor_dn6)), (((var_mutmp_dn7 + var_gr_dn7) * var_rxcor) + (assign43050_e56391 * var_rxcor_dn7)), (((var_mutmp_dn8 + var_gr_dn8) * var_rxcor) + (assign43050_e56391 * var_rxcor_dn8)), (((var_mutmp_dn9 + var_gr_dn9) * var_rxcor) + (assign43050_e56391 * var_rxcor_dn9)),)
    } else {
        (var_gmobs, var_gmobs_dn4, var_gmobs_dn6, var_gmobs_dn7, var_gmobs_dn8, var_gmobs_dn9,)
    }
};
        var_gmobs = assign43050_e56395;
        var_gmobs_dn4 = assign43050_e56395_d_n4;
        var_gmobs_dn6 = assign43050_e56395_d_n6;
        var_gmobs_dn7 = assign43050_e56395_d_n7;
        var_gmobs_dn8 = assign43050_e56395_d_n8;
        var_gmobs_dn9 = assign43050_e56395_d_n9;

        let assign43060_e56398: f64 = if var_thesatb_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1212 = assign43060_e56398;

        let (assign43070_e56412, assign43070_e56412_d_n4, assign43070_e56412_d_n6, assign43070_e56412_d_n7, assign43070_e56412_d_n8, assign43070_e56412_d_n9,) = {
    if (((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) && (var_guard1212 != 0.0)) {
        let assign43070_e56408: f64 = (var_thesatb_i * var_vsbx);
        let assign43070_e56409: f64 = (1.0 - assign43070_e56408);
        let assign43070_e56410: f64 = (1.0 / assign43070_e56409);
        (assign43070_e56410, (-((-(var_thesatb_i * var_vsbx_dn4)) / (assign43070_e56409 * assign43070_e56409))), (-((-(var_thesatb_i * var_vsbx_dn6)) / (assign43070_e56409 * assign43070_e56409))), (-((-(var_thesatb_i * var_vsbx_dn7)) / (assign43070_e56409 * assign43070_e56409))), (-((-(var_thesatb_i * var_vsbx_dn8)) / (assign43070_e56409 * assign43070_e56409))), (-((-(var_thesatb_i * var_vsbx_dn9)) / (assign43070_e56409 * assign43070_e56409))),)
    } else {
        (var_xitsb, var_xitsb_dn4, var_xitsb_dn6, var_xitsb_dn7, var_xitsb_dn8, var_xitsb_dn9,)
    }
};
        var_xitsb = assign43070_e56412;
        var_xitsb_dn4 = assign43070_e56412_d_n4;
        var_xitsb_dn6 = assign43070_e56412_d_n6;
        var_xitsb_dn7 = assign43070_e56412_d_n7;
        var_xitsb_dn8 = assign43070_e56412_d_n8;
        var_xitsb_dn9 = assign43070_e56412_d_n9;

        let (assign43080_e56425, assign43080_e56425_d_n4, assign43080_e56425_d_n6, assign43080_e56425_d_n7, assign43080_e56425_d_n8, assign43080_e56425_d_n9,) = {
    if (((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) && (var_guard1212 == 0.0)) {
        let assign43080_e56422: f64 = (var_thesatb_i * var_vsbx);
        let assign43080_e56423: f64 = (1.0 + assign43080_e56422);
        (assign43080_e56423, (var_thesatb_i * var_vsbx_dn4), (var_thesatb_i * var_vsbx_dn6), (var_thesatb_i * var_vsbx_dn7), (var_thesatb_i * var_vsbx_dn8), (var_thesatb_i * var_vsbx_dn9),)
    } else {
        (var_xitsb, var_xitsb_dn4, var_xitsb_dn6, var_xitsb_dn7, var_xitsb_dn8, var_xitsb_dn9,)
    }
};
        var_xitsb = assign43080_e56425;
        var_xitsb_dn4 = assign43080_e56425_d_n4;
        var_xitsb_dn6 = assign43080_e56425_d_n6;
        var_xitsb_dn7 = assign43080_e56425_d_n7;
        var_xitsb_dn8 = assign43080_e56425_d_n8;
        var_xitsb_dn9 = assign43080_e56425_d_n9;

        let (assign43090_e56433, assign43090_e56433_d_n4, assign43090_e56433_d_n6, assign43090_e56433_d_n7, assign43090_e56433_d_n8, assign43090_e56433_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) {
        let assign43090_e56431: f64 = (var_qis * var_xitsb);
        (assign43090_e56431, ((var_qis_dn4 * var_xitsb) + (var_qis * var_xitsb_dn4)), ((var_qis_dn6 * var_xitsb) + (var_qis * var_xitsb_dn6)), ((var_qis_dn7 * var_xitsb) + (var_qis * var_xitsb_dn7)), ((var_qis_dn8 * var_xitsb) + (var_qis * var_xitsb_dn8)), ((var_qis_dn9 * var_xitsb) + (var_qis * var_xitsb_dn9)),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign43090_e56433;
        var_temp2_dn4 = assign43090_e56433_d_n4;
        var_temp2_dn6 = assign43090_e56433_d_n6;
        var_temp2_dn7 = assign43090_e56433_d_n7;
        var_temp2_dn8 = assign43090_e56433_d_n8;
        var_temp2_dn9 = assign43090_e56433_d_n9;

        let (assign43100_e56443, assign43100_e56443_d_n4, assign43100_e56443_d_n6, assign43100_e56443_d_n7, assign43100_e56443_d_n8, assign43100_e56443_d_n9,) = {
    if ((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) {
        let assign43100_e56440: f64 = (var_thesatt_i + var_temp2);
        let assign43100_e56441: f64 = (var_temp2 / assign43100_e56440);
        (assign43100_e56441, (((var_temp2_dn4 * assign43100_e56440) - (var_temp2 * var_temp2_dn4)) / (assign43100_e56440 * assign43100_e56440)), (((var_temp2_dn6 * assign43100_e56440) - (var_temp2 * var_temp2_dn6)) / (assign43100_e56440 * assign43100_e56440)), (((var_temp2_dn7 * assign43100_e56440) - (var_temp2 * var_temp2_dn7)) / (assign43100_e56440 * assign43100_e56440)), (((var_temp2_dn8 * assign43100_e56440) - (var_temp2 * var_temp2_dn8)) / (assign43100_e56440 * assign43100_e56440)), (((var_temp2_dn9 * assign43100_e56440) - (var_temp2 * var_temp2_dn9)) / (assign43100_e56440 * assign43100_e56440)),)
    } else {
        (var_wsat, var_wsat_dn4, var_wsat_dn6, var_wsat_dn7, var_wsat_dn8, var_wsat_dn9,)
    }
};
        var_wsat = assign43100_e56443;
        var_wsat_dn4 = assign43100_e56443_d_n4;
        var_wsat_dn6 = assign43100_e56443_d_n6;
        var_wsat_dn7 = assign43100_e56443_d_n7;
        var_wsat_dn8 = assign43100_e56443_d_n8;
        var_wsat_dn9 = assign43100_e56443_d_n9;

        let assign43110_e56446: f64 = if var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1213 = assign43110_e56446;

        let (assign43120_e56460, assign43120_e56460_d_n4, assign43120_e56460_d_n6, assign43120_e56460_d_n7, assign43120_e56460_d_n8, assign43120_e56460_d_n9,) = {
    if (((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) && (var_guard1213 != 0.0)) {
        let assign43120_e56456: f64 = (var_thesatg_i * var_wsat);
        let assign43120_e56457: f64 = (1.0 - assign43120_e56456);
        let assign43120_e56458: f64 = (1.0 / assign43120_e56457);
        (assign43120_e56458, (-((-(var_thesatg_i * var_wsat_dn4)) / (assign43120_e56457 * assign43120_e56457))), (-((-(var_thesatg_i * var_wsat_dn6)) / (assign43120_e56457 * assign43120_e56457))), (-((-(var_thesatg_i * var_wsat_dn7)) / (assign43120_e56457 * assign43120_e56457))), (-((-(var_thesatg_i * var_wsat_dn8)) / (assign43120_e56457 * assign43120_e56457))), (-((-(var_thesatg_i * var_wsat_dn9)) / (assign43120_e56457 * assign43120_e56457))),)
    } else {
        (var_factheta, var_factheta_dn4, var_factheta_dn6, var_factheta_dn7, var_factheta_dn8, var_factheta_dn9,)
    }
};
        var_factheta = assign43120_e56460;
        var_factheta_dn4 = assign43120_e56460_d_n4;
        var_factheta_dn6 = assign43120_e56460_d_n6;
        var_factheta_dn7 = assign43120_e56460_d_n7;
        var_factheta_dn8 = assign43120_e56460_d_n8;
        var_factheta_dn9 = assign43120_e56460_d_n9;

        let (assign43130_e56473, assign43130_e56473_d_n4, assign43130_e56473_d_n6, assign43130_e56473_d_n7, assign43130_e56473_d_n8, assign43130_e56473_d_n9,) = {
    if (((var_guard1205 != 0.0) && (var_guard1209 != 0.0)) && (var_guard1213 == 0.0)) {
        let assign43130_e56470: f64 = (var_thesatg_i * var_wsat);
        let assign43130_e56471: f64 = (1.0 + assign43130_e56470);
        (assign43130_e56471, (var_thesatg_i * var_wsat_dn4), (var_thesatg_i * var_wsat_dn6), (var_thesatg_i * var_wsat_dn7), (var_thesatg_i * var_wsat_dn8), (var_thesatg_i * var_wsat_dn9),)
    } else {
        (var_factheta, var_factheta_dn4, var_factheta_dn6, var_factheta_dn7, var_factheta_dn8, var_factheta_dn9,)
    }
};
        var_factheta = assign43130_e56473;
        var_factheta_dn4 = assign43130_e56473_d_n4;
        var_factheta_dn6 = assign43130_e56473_d_n6;
        var_factheta_dn7 = assign43130_e56473_d_n7;
        var_factheta_dn8 = assign43130_e56473_d_n8;
        var_factheta_dn9 = assign43130_e56473_d_n9;

        var_vgb1_dc = var_vgb1;
        var_vgb1_dc_dn4 = var_vgb1_dn4;
        var_vgb1_dc_dn6 = var_vgb1_dn6;
        var_vgb1_dc_dn7 = var_vgb1_dn7;
        var_vgb1_dc_dn8 = var_vgb1_dn8;
        var_vgb1_dc_dn9 = var_vgb1_dn9;

        var_vsbx_dc = var_vsbx;
        var_vsbx_dc_dn4 = var_vsbx_dn4;
        var_vsbx_dc_dn6 = var_vsbx_dn6;
        var_vsbx_dc_dn7 = var_vsbx_dn7;
        var_vsbx_dc_dn8 = var_vsbx_dn8;
        var_vsbx_dc_dn9 = var_vsbx_dn9;

        var_phit1_dc = var_phit1;
        var_phit1_dc_dn4 = var_phit1_dn4;
        var_phit1_dc_dn6 = var_phit1_dn6;
        var_phit1_dc_dn7 = var_phit1_dn7;
        var_phit1_dc_dn8 = var_phit1_dn8;
        var_phit1_dc_dn9 = var_phit1_dn9;

        var_inv_phit1_dc = var_inv_phit1;
        var_inv_phit1_dc_dn4 = var_inv_phit1_dn4;
        var_inv_phit1_dc_dn6 = var_inv_phit1_dn6;
        var_inv_phit1_dc_dn7 = var_inv_phit1_dn7;
        var_inv_phit1_dc_dn8 = var_inv_phit1_dn8;
        var_inv_phit1_dc_dn9 = var_inv_phit1_dn9;

        var_gf_dc = var_gf;
        var_gf_dc_dn4 = var_gf_dn4;
        var_gf_dc_dn6 = var_gf_dn6;
        var_gf_dc_dn7 = var_gf_dn7;
        var_gf_dc_dn8 = var_gf_dn8;
        var_gf_dc_dn9 = var_gf_dn9;

        var_gf2_dc = var_gf2;
        var_gf2_dc_dn4 = var_gf2_dn4;
        var_gf2_dc_dn6 = var_gf2_dn6;
        var_gf2_dc_dn7 = var_gf2_dn7;
        var_gf2_dc_dn8 = var_gf2_dn8;
        var_gf2_dc_dn9 = var_gf2_dn9;

        var_inv_gf2_dc = var_inv_gf2;
        var_inv_gf2_dc_dn4 = var_inv_gf2_dn4;
        var_inv_gf2_dc_dn6 = var_inv_gf2_dn6;
        var_inv_gf2_dc_dn7 = var_inv_gf2_dn7;
        var_inv_gf2_dc_dn8 = var_inv_gf2_dn8;
        var_inv_gf2_dc_dn9 = var_inv_gf2_dn9;

        var_xg_dc = var_xg;
        var_xg_dc_dn4 = var_xg_dn4;
        var_xg_dc_dn6 = var_xg_dn6;
        var_xg_dc_dn7 = var_xg_dn7;
        var_xg_dc_dn8 = var_xg_dn8;
        var_xg_dc_dn9 = var_xg_dn9;

        var_xno_s_dc = var_xno_s;
        var_xno_s_dc_dn4 = var_xno_s_dn4;
        var_xno_s_dc_dn6 = var_xno_s_dn6;
        var_xno_s_dc_dn7 = var_xno_s_dn7;
        var_xno_s_dc_dn8 = var_xno_s_dn8;
        var_xno_s_dc_dn9 = var_xno_s_dn9;

        var_xn_s_dc = var_xn_s;
        var_xn_s_dc_dn4 = var_xn_s_dn4;
        var_xn_s_dc_dn6 = var_xn_s_dn6;
        var_xn_s_dc_dn7 = var_xn_s_dn7;
        var_xn_s_dc_dn8 = var_xn_s_dn8;
        var_xn_s_dc_dn9 = var_xn_s_dn9;

        var_xi_dc = var_xi;
        var_xi_dc_dn4 = var_xi_dn4;
        var_xi_dc_dn6 = var_xi_dn6;
        var_xi_dc_dn7 = var_xi_dn7;
        var_xi_dc_dn8 = var_xi_dn8;
        var_xi_dc_dn9 = var_xi_dn9;

        var_margin_dc = var_margin;

        var_inv_xi_dc = var_inv_xi;
        var_inv_xi_dc_dn4 = var_inv_xi_dn4;
        var_inv_xi_dc_dn6 = var_inv_xi_dn6;
        var_inv_xi_dc_dn7 = var_inv_xi_dn7;
        var_inv_xi_dc_dn8 = var_inv_xi_dn8;
        var_inv_xi_dc_dn9 = var_inv_xi_dn9;

        var_sp_s_x1_dc = var_sp_s_x1;
        var_sp_s_x1_dc_dn4 = var_sp_s_x1_dn4;
        var_sp_s_x1_dc_dn6 = var_sp_s_x1_dn6;
        var_sp_s_x1_dc_dn7 = var_sp_s_x1_dn7;
        var_sp_s_x1_dc_dn8 = var_sp_s_x1_dn8;
        var_sp_s_x1_dc_dn9 = var_sp_s_x1_dn9;

        *var_alphas_slot = var_alphas;
        *var_alphas_dn4_slot = var_alphas_dn4;
        *var_alphas_dn6_slot = var_alphas_dn6;
        *var_alphas_dn7_slot = var_alphas_dn7;
        *var_alphas_dn8_slot = var_alphas_dn8;
        *var_alphas_dn9_slot = var_alphas_dn9;
        *var_eeffs_slot = var_eeffs;
        *var_eeffs_dn4_slot = var_eeffs_dn4;
        *var_eeffs_dn6_slot = var_eeffs_dn6;
        *var_eeffs_dn7_slot = var_eeffs_dn7;
        *var_eeffs_dn8_slot = var_eeffs_dn8;
        *var_eeffs_dn9_slot = var_eeffs_dn9;
        *var_factheta_slot = var_factheta;
        *var_factheta_dn4_slot = var_factheta_dn4;
        *var_factheta_dn6_slot = var_factheta_dn6;
        *var_factheta_dn7_slot = var_factheta_dn7;
        *var_factheta_dn8_slot = var_factheta_dn8;
        *var_factheta_dn9_slot = var_factheta_dn9;
        *var_gf2_dc_slot = var_gf2_dc;
        *var_gf2_dc_dn4_slot = var_gf2_dc_dn4;
        *var_gf2_dc_dn6_slot = var_gf2_dc_dn6;
        *var_gf2_dc_dn7_slot = var_gf2_dc_dn7;
        *var_gf2_dc_dn8_slot = var_gf2_dc_dn8;
        *var_gf2_dc_dn9_slot = var_gf2_dc_dn9;
        *var_gf_dc_slot = var_gf_dc;
        *var_gf_dc_dn4_slot = var_gf_dc_dn4;
        *var_gf_dc_dn6_slot = var_gf_dc_dn6;
        *var_gf_dc_dn7_slot = var_gf_dc_dn7;
        *var_gf_dc_dn8_slot = var_gf_dc_dn8;
        *var_gf_dc_dn9_slot = var_gf_dc_dn9;
        *var_gmobs_slot = var_gmobs;
        *var_gmobs_dn4_slot = var_gmobs_dn4;
        *var_gmobs_dn6_slot = var_gmobs_dn6;
        *var_gmobs_dn7_slot = var_gmobs_dn7;
        *var_gmobs_dn8_slot = var_gmobs_dn8;
        *var_gmobs_dn9_slot = var_gmobs_dn9;
        *var_gr_slot = var_gr;
        *var_gr_dn4_slot = var_gr_dn4;
        *var_gr_dn6_slot = var_gr_dn6;
        *var_gr_dn7_slot = var_gr_dn7;
        *var_gr_dn8_slot = var_gr_dn8;
        *var_gr_dn9_slot = var_gr_dn9;
        *var_guard1209_slot = var_guard1209;
        *var_guard1210_slot = var_guard1210;
        *var_guard1211_slot = var_guard1211;
        *var_guard1212_slot = var_guard1212;
        *var_guard1213_slot = var_guard1213;
        *var_inv_gf2_dc_slot = var_inv_gf2_dc;
        *var_inv_gf2_dc_dn4_slot = var_inv_gf2_dc_dn4;
        *var_inv_gf2_dc_dn6_slot = var_inv_gf2_dc_dn6;
        *var_inv_gf2_dc_dn7_slot = var_inv_gf2_dc_dn7;
        *var_inv_gf2_dc_dn8_slot = var_inv_gf2_dc_dn8;
        *var_inv_gf2_dc_dn9_slot = var_inv_gf2_dc_dn9;
        *var_inv_phit1_dc_slot = var_inv_phit1_dc;
        *var_inv_phit1_dc_dn4_slot = var_inv_phit1_dc_dn4;
        *var_inv_phit1_dc_dn6_slot = var_inv_phit1_dc_dn6;
        *var_inv_phit1_dc_dn7_slot = var_inv_phit1_dc_dn7;
        *var_inv_phit1_dc_dn8_slot = var_inv_phit1_dc_dn8;
        *var_inv_phit1_dc_dn9_slot = var_inv_phit1_dc_dn9;
        *var_inv_xi_dc_slot = var_inv_xi_dc;
        *var_inv_xi_dc_dn4_slot = var_inv_xi_dc_dn4;
        *var_inv_xi_dc_dn6_slot = var_inv_xi_dc_dn6;
        *var_inv_xi_dc_dn7_slot = var_inv_xi_dc_dn7;
        *var_inv_xi_dc_dn8_slot = var_inv_xi_dc_dn8;
        *var_inv_xi_dc_dn9_slot = var_inv_xi_dc_dn9;
        *var_margin_dc_slot = var_margin_dc;
        *var_mutmp_slot = var_mutmp;
        *var_mutmp_dn4_slot = var_mutmp_dn4;
        *var_mutmp_dn6_slot = var_mutmp_dn6;
        *var_mutmp_dn7_slot = var_mutmp_dn7;
        *var_mutmp_dn8_slot = var_mutmp_dn8;
        *var_mutmp_dn9_slot = var_mutmp_dn9;
        *var_phit1_dc_slot = var_phit1_dc;
        *var_phit1_dc_dn4_slot = var_phit1_dc_dn4;
        *var_phit1_dc_dn6_slot = var_phit1_dc_dn6;
        *var_phit1_dc_dn7_slot = var_phit1_dc_dn7;
        *var_phit1_dc_dn8_slot = var_phit1_dc_dn8;
        *var_phit1_dc_dn9_slot = var_phit1_dc_dn9;
        *var_ps_slot = var_ps;
        *var_ps_dn4_slot = var_ps_dn4;
        *var_ps_dn6_slot = var_ps_dn6;
        *var_ps_dn7_slot = var_ps_dn7;
        *var_ps_dn8_slot = var_ps_dn8;
        *var_ps_dn9_slot = var_ps_dn9;
        *var_qbs_slot = var_qbs;
        *var_qbs_dn4_slot = var_qbs_dn4;
        *var_qbs_dn6_slot = var_qbs_dn6;
        *var_qbs_dn7_slot = var_qbs_dn7;
        *var_qbs_dn8_slot = var_qbs_dn8;
        *var_qbs_dn9_slot = var_qbs_dn9;
        *var_qis_slot = var_qis;
        *var_qis_dn4_slot = var_qis_dn4;
        *var_qis_dn6_slot = var_qis_dn6;
        *var_qis_dn7_slot = var_qis_dn7;
        *var_qis_dn8_slot = var_qis_dn8;
        *var_qis_dn9_slot = var_qis_dn9;
        *var_rhob_slot = var_rhob;
        *var_rhob_dn4_slot = var_rhob_dn4;
        *var_rhob_dn6_slot = var_rhob_dn6;
        *var_rhob_dn7_slot = var_rhob_dn7;
        *var_rhob_dn8_slot = var_rhob_dn8;
        *var_rhob_dn9_slot = var_rhob_dn9;
        *var_rhog_slot = var_rhog;
        *var_rhog_dn4_slot = var_rhog_dn4;
        *var_rhog_dn6_slot = var_rhog_dn6;
        *var_rhog_dn7_slot = var_rhog_dn7;
        *var_rhog_dn8_slot = var_rhog_dn8;
        *var_rhog_dn9_slot = var_rhog_dn9;
        *var_rxcor_slot = var_rxcor;
        *var_rxcor_dn4_slot = var_rxcor_dn4;
        *var_rxcor_dn6_slot = var_rxcor_dn6;
        *var_rxcor_dn7_slot = var_rxcor_dn7;
        *var_rxcor_dn8_slot = var_rxcor_dn8;
        *var_rxcor_dn9_slot = var_rxcor_dn9;
        *var_sp_s_x1_dc_slot = var_sp_s_x1_dc;
        *var_sp_s_x1_dc_dn4_slot = var_sp_s_x1_dc_dn4;
        *var_sp_s_x1_dc_dn6_slot = var_sp_s_x1_dc_dn6;
        *var_sp_s_x1_dc_dn7_slot = var_sp_s_x1_dc_dn7;
        *var_sp_s_x1_dc_dn8_slot = var_sp_s_x1_dc_dn8;
        *var_sp_s_x1_dc_dn9_slot = var_sp_s_x1_dc_dn9;
        *var_sqs_slot = var_sqs;
        *var_sqs_dn4_slot = var_sqs_dn4;
        *var_sqs_dn6_slot = var_sqs_dn6;
        *var_sqs_dn7_slot = var_sqs_dn7;
        *var_sqs_dn8_slot = var_sqs_dn8;
        *var_sqs_dn9_slot = var_sqs_dn9;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_vgb1_dc_slot = var_vgb1_dc;
        *var_vgb1_dc_dn4_slot = var_vgb1_dc_dn4;
        *var_vgb1_dc_dn6_slot = var_vgb1_dc_dn6;
        *var_vgb1_dc_dn7_slot = var_vgb1_dc_dn7;
        *var_vgb1_dc_dn8_slot = var_vgb1_dc_dn8;
        *var_vgb1_dc_dn9_slot = var_vgb1_dc_dn9;
        *var_vsbx_dc_slot = var_vsbx_dc;
        *var_vsbx_dc_dn4_slot = var_vsbx_dc_dn4;
        *var_vsbx_dc_dn6_slot = var_vsbx_dc_dn6;
        *var_vsbx_dc_dn7_slot = var_vsbx_dc_dn7;
        *var_vsbx_dc_dn8_slot = var_vsbx_dc_dn8;
        *var_vsbx_dc_dn9_slot = var_vsbx_dc_dn9;
        *var_wsat_slot = var_wsat;
        *var_wsat_dn4_slot = var_wsat_dn4;
        *var_wsat_dn6_slot = var_wsat_dn6;
        *var_wsat_dn7_slot = var_wsat_dn7;
        *var_wsat_dn8_slot = var_wsat_dn8;
        *var_wsat_dn9_slot = var_wsat_dn9;
        *var_xg_dc_slot = var_xg_dc;
        *var_xg_dc_dn4_slot = var_xg_dc_dn4;
        *var_xg_dc_dn6_slot = var_xg_dc_dn6;
        *var_xg_dc_dn7_slot = var_xg_dc_dn7;
        *var_xg_dc_dn8_slot = var_xg_dc_dn8;
        *var_xg_dc_dn9_slot = var_xg_dc_dn9;
        *var_xgs_slot = var_xgs;
        *var_xgs_dn4_slot = var_xgs_dn4;
        *var_xgs_dn6_slot = var_xgs_dn6;
        *var_xgs_dn7_slot = var_xgs_dn7;
        *var_xgs_dn8_slot = var_xgs_dn8;
        *var_xgs_dn9_slot = var_xgs_dn9;
        *var_xi_dc_slot = var_xi_dc;
        *var_xi_dc_dn4_slot = var_xi_dc_dn4;
        *var_xi_dc_dn6_slot = var_xi_dc_dn6;
        *var_xi_dc_dn7_slot = var_xi_dc_dn7;
        *var_xi_dc_dn8_slot = var_xi_dc_dn8;
        *var_xi_dc_dn9_slot = var_xi_dc_dn9;
        *var_xitsb_slot = var_xitsb;
        *var_xitsb_dn4_slot = var_xitsb_dn4;
        *var_xitsb_dn6_slot = var_xitsb_dn6;
        *var_xitsb_dn7_slot = var_xitsb_dn7;
        *var_xitsb_dn8_slot = var_xitsb_dn8;
        *var_xitsb_dn9_slot = var_xitsb_dn9;
        *var_xn_s_dc_slot = var_xn_s_dc;
        *var_xn_s_dc_dn4_slot = var_xn_s_dc_dn4;
        *var_xn_s_dc_dn6_slot = var_xn_s_dc_dn6;
        *var_xn_s_dc_dn7_slot = var_xn_s_dc_dn7;
        *var_xn_s_dc_dn8_slot = var_xn_s_dc_dn8;
        *var_xn_s_dc_dn9_slot = var_xn_s_dc_dn9;
        *var_xno_s_dc_slot = var_xno_s_dc;
        *var_xno_s_dc_dn4_slot = var_xno_s_dc_dn4;
        *var_xno_s_dc_dn6_slot = var_xno_s_dc_dn6;
        *var_xno_s_dc_dn7_slot = var_xno_s_dc_dn7;
        *var_xno_s_dc_dn8_slot = var_xno_s_dc_dn8;
        *var_xno_s_dc_dn9_slot = var_xno_s_dc_dn9;
    }

    pub(super) fn stamp_transient_block_94(
        var_alphas: f64,
        var_alphas_dn4: f64,
        var_alphas_dn6: f64,
        var_alphas_dn7: f64,
        var_alphas_dn8: f64,
        var_alphas_dn9: f64,
        var_cs_t: f64,
        var_delta_1s: f64,
        var_delta_1s_dn4: f64,
        var_delta_1s_dn6: f64,
        var_delta_1s_dn7: f64,
        var_delta_1s_dn8: f64,
        var_delta_1s_dn9: f64,
        var_delta_ns: f64,
        var_delta_ns_dn4: f64,
        var_delta_ns_dn6: f64,
        var_delta_ns_dn7: f64,
        var_delta_ns_dn8: f64,
        var_delta_ns_dn9: f64,
        var_ds: f64,
        var_ds_dn4: f64,
        var_ds_dn6: f64,
        var_ds_dn7: f64,
        var_ds_dn8: f64,
        var_ds_dn9: f64,
        var_es: f64,
        var_es_dn4: f64,
        var_es_dn6: f64,
        var_es_dn7: f64,
        var_es_dn8: f64,
        var_es_dn9: f64,
        var_factheta: f64,
        var_factheta_dn4: f64,
        var_factheta_dn6: f64,
        var_factheta_dn7: f64,
        var_factheta_dn8: f64,
        var_factheta_dn9: f64,
        var_gf2: f64,
        var_gf2_dn4: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf2_dn9: f64,
        var_gmobs: f64,
        var_gmobs_dn4: f64,
        var_gmobs_dn6: f64,
        var_gmobs_dn7: f64,
        var_gmobs_dn8: f64,
        var_gmobs_dn9: f64,
        var_inv_phit1: f64,
        var_inv_phit1_dn4: f64,
        var_inv_phit1_dn6: f64,
        var_inv_phit1_dn7: f64,
        var_inv_phit1_dn8: f64,
        var_inv_phit1_dn9: f64,
        var_phit1: f64,
        var_phit1_dn4: f64,
        var_phit1_dn6: f64,
        var_phit1_dn7: f64,
        var_phit1_dn8: f64,
        var_phit1_dn9: f64,
        var_ps: f64,
        var_ps_dn4: f64,
        var_ps_dn6: f64,
        var_ps_dn7: f64,
        var_ps_dn8: f64,
        var_ps_dn9: f64,
        var_qbs: f64,
        var_qbs_dn4: f64,
        var_qbs_dn6: f64,
        var_qbs_dn7: f64,
        var_qbs_dn8: f64,
        var_qbs_dn9: f64,
        var_qis: f64,
        var_qis_dn4: f64,
        var_qis_dn6: f64,
        var_qis_dn7: f64,
        var_qis_dn8: f64,
        var_qis_dn9: f64,
        var_rhob: f64,
        var_rhob_dn4: f64,
        var_rhob_dn6: f64,
        var_rhob_dn7: f64,
        var_rhob_dn8: f64,
        var_rhob_dn9: f64,
        var_rhog: f64,
        var_rhog_dn4: f64,
        var_rhog_dn6: f64,
        var_rhog_dn7: f64,
        var_rhog_dn8: f64,
        var_rhog_dn9: f64,
        var_rxcor: f64,
        var_rxcor_dn4: f64,
        var_rxcor_dn6: f64,
        var_rxcor_dn7: f64,
        var_rxcor_dn8: f64,
        var_rxcor_dn9: f64,
        var_sqs: f64,
        var_sqs_dn4: f64,
        var_sqs_dn6: f64,
        var_sqs_dn7: f64,
        var_sqs_dn8: f64,
        var_sqs_dn9: f64,
        var_thecs_t: f64,
        var_thesatloc: f64,
        var_thesatloc_dn4: f64,
        var_v_ds: f64,
        var_v_ds_dn7: f64,
        var_v_ds_dn8: f64,
        var_x_s: f64,
        var_x_s_dn4: f64,
        var_x_s_dn6: f64,
        var_x_s_dn7: f64,
        var_x_s_dn8: f64,
        var_x_s_dn9: f64,
        var_xg: f64,
        var_xg_dn4: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xg_dn9: f64,
        var_xgs: f64,
        var_xgs_dn4: f64,
        var_xgs_dn6: f64,
        var_xgs_dn7: f64,
        var_xgs_dn8: f64,
        var_xgs_dn9: f64,
        var_xi1s: f64,
        var_xi1s_dn4: f64,
        var_xi1s_dn6: f64,
        var_xi1s_dn7: f64,
        var_xi1s_dn8: f64,
        var_xi1s_dn9: f64,
        var_xi2s: f64,
        var_xi2s_dn4: f64,
        var_xi2s_dn6: f64,
        var_xi2s_dn7: f64,
        var_xi2s_dn8: f64,
        var_xi2s_dn9: f64,
        var_xitsb: f64,
        var_xitsb_dn4: f64,
        var_xitsb_dn6: f64,
        var_xitsb_dn7: f64,
        var_xitsb_dn8: f64,
        var_xitsb_dn9: f64,
        var_alpha_slot: &mut f64,
        var_alpha_dn4_slot: &mut f64,
        var_alpha_dn6_slot: &mut f64,
        var_alpha_dn7_slot: &mut f64,
        var_alpha_dn8_slot: &mut f64,
        var_alpha_dn9_slot: &mut f64,
        var_alphas_dc_slot: &mut f64,
        var_alphas_dc_dn4_slot: &mut f64,
        var_alphas_dc_dn6_slot: &mut f64,
        var_alphas_dc_dn7_slot: &mut f64,
        var_alphas_dc_dn8_slot: &mut f64,
        var_alphas_dc_dn9_slot: &mut f64,
        var_asat_slot: &mut f64,
        var_asat_dn4_slot: &mut f64,
        var_asat_dn6_slot: &mut f64,
        var_asat_dn7_slot: &mut f64,
        var_asat_dn8_slot: &mut f64,
        var_asat_dn9_slot: &mut f64,
        var_dd_slot: &mut f64,
        var_dd_dn4_slot: &mut f64,
        var_dd_dn6_slot: &mut f64,
        var_dd_dn7_slot: &mut f64,
        var_dd_dn8_slot: &mut f64,
        var_dd_dn9_slot: &mut f64,
        var_delta_1s_dc_slot: &mut f64,
        var_delta_1s_dc_dn4_slot: &mut f64,
        var_delta_1s_dc_dn6_slot: &mut f64,
        var_delta_1s_dc_dn7_slot: &mut f64,
        var_delta_1s_dc_dn8_slot: &mut f64,
        var_delta_1s_dc_dn9_slot: &mut f64,
        var_delta_ns_dc_slot: &mut f64,
        var_delta_ns_dc_dn4_slot: &mut f64,
        var_delta_ns_dc_dn6_slot: &mut f64,
        var_delta_ns_dc_dn7_slot: &mut f64,
        var_delta_ns_dc_dn8_slot: &mut f64,
        var_delta_ns_dc_dn9_slot: &mut f64,
        var_dm_slot: &mut f64,
        var_dm_dn4_slot: &mut f64,
        var_dm_dn6_slot: &mut f64,
        var_dm_dn7_slot: &mut f64,
        var_dm_dn8_slot: &mut f64,
        var_dm_dn9_slot: &mut f64,
        var_dps_slot: &mut f64,
        var_dps_dn4_slot: &mut f64,
        var_dps_dn6_slot: &mut f64,
        var_dps_dn7_slot: &mut f64,
        var_dps_dn8_slot: &mut f64,
        var_dps_dn9_slot: &mut f64,
        var_ds_dc_slot: &mut f64,
        var_ds_dc_dn4_slot: &mut f64,
        var_ds_dc_dn6_slot: &mut f64,
        var_ds_dc_dn7_slot: &mut f64,
        var_ds_dc_dn8_slot: &mut f64,
        var_ds_dc_dn9_slot: &mut f64,
        var_ed_slot: &mut f64,
        var_ed_dn4_slot: &mut f64,
        var_ed_dn6_slot: &mut f64,
        var_ed_dn7_slot: &mut f64,
        var_ed_dn8_slot: &mut f64,
        var_ed_dn9_slot: &mut f64,
        var_em_slot: &mut f64,
        var_em_dn4_slot: &mut f64,
        var_em_dn6_slot: &mut f64,
        var_em_dn7_slot: &mut f64,
        var_em_dn8_slot: &mut f64,
        var_em_dn9_slot: &mut f64,
        var_es_dc_slot: &mut f64,
        var_es_dc_dn4_slot: &mut f64,
        var_es_dc_dn6_slot: &mut f64,
        var_es_dc_dn7_slot: &mut f64,
        var_es_dc_dn8_slot: &mut f64,
        var_es_dc_dn9_slot: &mut f64,
        var_eta_p_slot: &mut f64,
        var_eta_p_dn4_slot: &mut f64,
        var_eta_p_dn6_slot: &mut f64,
        var_eta_p_dn7_slot: &mut f64,
        var_eta_p_dn8_slot: &mut f64,
        var_eta_p_dn9_slot: &mut f64,
        var_factheta_dc_slot: &mut f64,
        var_factheta_dc_dn4_slot: &mut f64,
        var_factheta_dc_dn6_slot: &mut f64,
        var_factheta_dc_dn7_slot: &mut f64,
        var_factheta_dc_dn8_slot: &mut f64,
        var_factheta_dc_dn9_slot: &mut f64,
        var_gmob_slot: &mut f64,
        var_gmob_dn4_slot: &mut f64,
        var_gmob_dn6_slot: &mut f64,
        var_gmob_dn7_slot: &mut f64,
        var_gmob_dn8_slot: &mut f64,
        var_gmob_dn9_slot: &mut f64,
        var_gmobs_dc_slot: &mut f64,
        var_gmobs_dc_dn4_slot: &mut f64,
        var_gmobs_dc_dn6_slot: &mut f64,
        var_gmobs_dc_dn7_slot: &mut f64,
        var_gmobs_dc_dn8_slot: &mut f64,
        var_gmobs_dc_dn9_slot: &mut f64,
        var_guard1214_slot: &mut f64,
        var_guard1215_slot: &mut f64,
        var_guard1216_slot: &mut f64,
        var_guard1217_slot: &mut f64,
        var_guard1218_slot: &mut f64,
        var_midphi0_slot: &mut f64,
        var_midphi0_dn4_slot: &mut f64,
        var_midphi0_dn6_slot: &mut f64,
        var_midphi0_dn7_slot: &mut f64,
        var_midphi0_dn8_slot: &mut f64,
        var_midphi0_dn9_slot: &mut f64,
        var_pd_slot: &mut f64,
        var_pd_dn4_slot: &mut f64,
        var_pd_dn6_slot: &mut f64,
        var_pd_dn7_slot: &mut f64,
        var_pd_dn8_slot: &mut f64,
        var_pd_dn9_slot: &mut f64,
        var_pm_slot: &mut f64,
        var_pm_dn4_slot: &mut f64,
        var_pm_dn6_slot: &mut f64,
        var_pm_dn7_slot: &mut f64,
        var_pm_dn8_slot: &mut f64,
        var_pm_dn9_slot: &mut f64,
        var_ps_dc_slot: &mut f64,
        var_ps_dc_dn4_slot: &mut f64,
        var_ps_dc_dn6_slot: &mut f64,
        var_ps_dc_dn7_slot: &mut f64,
        var_ps_dc_dn8_slot: &mut f64,
        var_ps_dc_dn9_slot: &mut f64,
        var_qbd_slot: &mut f64,
        var_qbd_dn4_slot: &mut f64,
        var_qbd_dn6_slot: &mut f64,
        var_qbd_dn7_slot: &mut f64,
        var_qbd_dn8_slot: &mut f64,
        var_qbd_dn9_slot: &mut f64,
        var_qbm_slot: &mut f64,
        var_qbm_dn4_slot: &mut f64,
        var_qbm_dn6_slot: &mut f64,
        var_qbm_dn7_slot: &mut f64,
        var_qbm_dn8_slot: &mut f64,
        var_qbm_dn9_slot: &mut f64,
        var_qbs_dc_slot: &mut f64,
        var_qbs_dc_dn4_slot: &mut f64,
        var_qbs_dc_dn6_slot: &mut f64,
        var_qbs_dc_dn7_slot: &mut f64,
        var_qbs_dc_dn8_slot: &mut f64,
        var_qbs_dc_dn9_slot: &mut f64,
        var_qeff1_slot: &mut f64,
        var_qeff1_dn4_slot: &mut f64,
        var_qeff1_dn6_slot: &mut f64,
        var_qeff1_dn7_slot: &mut f64,
        var_qeff1_dn8_slot: &mut f64,
        var_qeff1_dn9_slot: &mut f64,
        var_qim_slot: &mut f64,
        var_qim1_slot: &mut f64,
        var_qim1_dn4_slot: &mut f64,
        var_qim1_dn6_slot: &mut f64,
        var_qim1_dn7_slot: &mut f64,
        var_qim1_dn8_slot: &mut f64,
        var_qim1_dn9_slot: &mut f64,
        var_qim_dn4_slot: &mut f64,
        var_qim_dn6_slot: &mut f64,
        var_qim_dn7_slot: &mut f64,
        var_qim_dn8_slot: &mut f64,
        var_qim_dn9_slot: &mut f64,
        var_qis_dc_slot: &mut f64,
        var_qis_dc_dn4_slot: &mut f64,
        var_qis_dc_dn6_slot: &mut f64,
        var_qis_dc_dn7_slot: &mut f64,
        var_qis_dc_dn8_slot: &mut f64,
        var_qis_dc_dn9_slot: &mut f64,
        var_rhob_dc_slot: &mut f64,
        var_rhob_dc_dn4_slot: &mut f64,
        var_rhob_dc_dn6_slot: &mut f64,
        var_rhob_dc_dn7_slot: &mut f64,
        var_rhob_dc_dn8_slot: &mut f64,
        var_rhob_dc_dn9_slot: &mut f64,
        var_rhog_dc_slot: &mut f64,
        var_rhog_dc_dn4_slot: &mut f64,
        var_rhog_dc_dn6_slot: &mut f64,
        var_rhog_dc_dn7_slot: &mut f64,
        var_rhog_dc_dn8_slot: &mut f64,
        var_rhog_dc_dn9_slot: &mut f64,
        var_rxcor_dc_slot: &mut f64,
        var_rxcor_dc_dn4_slot: &mut f64,
        var_rxcor_dc_dn6_slot: &mut f64,
        var_rxcor_dc_dn7_slot: &mut f64,
        var_rxcor_dc_dn8_slot: &mut f64,
        var_rxcor_dc_dn9_slot: &mut f64,
        var_s1_slot: &mut f64,
        var_s1_dn4_slot: &mut f64,
        var_s1_dn6_slot: &mut f64,
        var_s1_dn7_slot: &mut f64,
        var_s1_dn8_slot: &mut f64,
        var_s1_dn9_slot: &mut f64,
        var_sqm_slot: &mut f64,
        var_sqm_dn4_slot: &mut f64,
        var_sqm_dn6_slot: &mut f64,
        var_sqm_dn7_slot: &mut f64,
        var_sqm_dn8_slot: &mut f64,
        var_sqm_dn9_slot: &mut f64,
        var_sqs_dc_slot: &mut f64,
        var_sqs_dc_dn4_slot: &mut f64,
        var_sqs_dc_dn6_slot: &mut f64,
        var_sqs_dc_dn7_slot: &mut f64,
        var_sqs_dc_dn8_slot: &mut f64,
        var_sqs_dc_dn9_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp__blk949_slot: &mut f64,
        var_temp__blk949_dn4_slot: &mut f64,
        var_temp__blk949_dn6_slot: &mut f64,
        var_temp__blk949_dn7_slot: &mut f64,
        var_temp__blk949_dn8_slot: &mut f64,
        var_temp__blk949_dn9_slot: &mut f64,
        var_thesat1_slot: &mut f64,
        var_thesat1_dn4_slot: &mut f64,
        var_thesat1_dn6_slot: &mut f64,
        var_thesat1_dn7_slot: &mut f64,
        var_thesat1_dn8_slot: &mut f64,
        var_thesat1_dn9_slot: &mut f64,
        var_thesateff_slot: &mut f64,
        var_thesateff_dn4_slot: &mut f64,
        var_thesateff_dn6_slot: &mut f64,
        var_thesateff_dn7_slot: &mut f64,
        var_thesateff_dn8_slot: &mut f64,
        var_thesateff_dn9_slot: &mut f64,
        var_udse_slot: &mut f64,
        var_udse_dn4_slot: &mut f64,
        var_udse_dn6_slot: &mut f64,
        var_udse_dn7_slot: &mut f64,
        var_udse_dn8_slot: &mut f64,
        var_udse_dn9_slot: &mut f64,
        var_v_dsat_slot: &mut f64,
        var_v_dsat_dn4_slot: &mut f64,
        var_v_dsat_dn6_slot: &mut f64,
        var_v_dsat_dn7_slot: &mut f64,
        var_v_dsat_dn8_slot: &mut f64,
        var_v_dsat_dn9_slot: &mut f64,
        var_vdsat_lim_slot: &mut f64,
        var_vdsat_lim_dn4_slot: &mut f64,
        var_vdsat_lim_dn6_slot: &mut f64,
        var_vdsat_lim_dn7_slot: &mut f64,
        var_vdsat_lim_dn8_slot: &mut f64,
        var_vdsat_lim_dn9_slot: &mut f64,
        var_vdse_slot: &mut f64,
        var_vdse_dn4_slot: &mut f64,
        var_vdse_dn6_slot: &mut f64,
        var_vdse_dn7_slot: &mut f64,
        var_vdse_dn8_slot: &mut f64,
        var_vdse_dn9_slot: &mut f64,
        var_voxm_slot: &mut f64,
        var_voxm_dn4_slot: &mut f64,
        var_voxm_dn6_slot: &mut f64,
        var_voxm_dn7_slot: &mut f64,
        var_voxm_dn8_slot: &mut f64,
        var_voxm_dn9_slot: &mut f64,
        var_x_d_slot: &mut f64,
        var_x_d_dn4_slot: &mut f64,
        var_x_d_dn6_slot: &mut f64,
        var_x_d_dn7_slot: &mut f64,
        var_x_d_dn8_slot: &mut f64,
        var_x_d_dn9_slot: &mut f64,
        var_x_ds_slot: &mut f64,
        var_x_ds_dn4_slot: &mut f64,
        var_x_ds_dn6_slot: &mut f64,
        var_x_ds_dn7_slot: &mut f64,
        var_x_ds_dn8_slot: &mut f64,
        var_x_ds_dn9_slot: &mut f64,
        var_x_inf0_slot: &mut f64,
        var_x_inf0_dn4_slot: &mut f64,
        var_x_inf0_dn6_slot: &mut f64,
        var_x_inf0_dn7_slot: &mut f64,
        var_x_inf0_dn8_slot: &mut f64,
        var_x_inf0_dn9_slot: &mut f64,
        var_x_m_slot: &mut f64,
        var_x_m_dn4_slot: &mut f64,
        var_x_m_dn6_slot: &mut f64,
        var_x_m_dn7_slot: &mut f64,
        var_x_m_dn8_slot: &mut f64,
        var_x_m_dn9_slot: &mut f64,
        var_x_s_dc_slot: &mut f64,
        var_x_s_dc_dn4_slot: &mut f64,
        var_x_s_dc_dn6_slot: &mut f64,
        var_x_s_dc_dn7_slot: &mut f64,
        var_x_s_dc_dn8_slot: &mut f64,
        var_x_s_dc_dn9_slot: &mut f64,
        var_xgm_slot: &mut f64,
        var_xgm_dn4_slot: &mut f64,
        var_xgm_dn6_slot: &mut f64,
        var_xgm_dn7_slot: &mut f64,
        var_xgm_dn8_slot: &mut f64,
        var_xgm_dn9_slot: &mut f64,
        var_xgs_dc_slot: &mut f64,
        var_xgs_dc_dn4_slot: &mut f64,
        var_xgs_dc_dn6_slot: &mut f64,
        var_xgs_dc_dn7_slot: &mut f64,
        var_xgs_dc_dn8_slot: &mut f64,
        var_xgs_dc_dn9_slot: &mut f64,
        var_xi1s_dc_slot: &mut f64,
        var_xi1s_dc_dn4_slot: &mut f64,
        var_xi1s_dc_dn6_slot: &mut f64,
        var_xi1s_dc_dn7_slot: &mut f64,
        var_xi1s_dc_dn8_slot: &mut f64,
        var_xi1s_dc_dn9_slot: &mut f64,
        var_xi2s_dc_slot: &mut f64,
        var_xi2s_dc_dn4_slot: &mut f64,
        var_xi2s_dc_dn6_slot: &mut f64,
        var_xi2s_dc_dn7_slot: &mut f64,
        var_xi2s_dc_dn8_slot: &mut f64,
        var_xi2s_dc_dn9_slot: &mut f64,
        var_xitsb_dc_slot: &mut f64,
        var_xitsb_dc_dn4_slot: &mut f64,
        var_xitsb_dc_dn6_slot: &mut f64,
        var_xitsb_dc_dn7_slot: &mut f64,
        var_xitsb_dc_dn8_slot: &mut f64,
        var_xitsb_dc_dn9_slot: &mut f64,
    ) {
        let mut var_alpha: f64 = *var_alpha_slot;
        let mut var_alpha_dn4: f64 = *var_alpha_dn4_slot;
        let mut var_alpha_dn6: f64 = *var_alpha_dn6_slot;
        let mut var_alpha_dn7: f64 = *var_alpha_dn7_slot;
        let mut var_alpha_dn8: f64 = *var_alpha_dn8_slot;
        let mut var_alpha_dn9: f64 = *var_alpha_dn9_slot;
        let mut var_alphas_dc: f64 = *var_alphas_dc_slot;
        let mut var_alphas_dc_dn4: f64 = *var_alphas_dc_dn4_slot;
        let mut var_alphas_dc_dn6: f64 = *var_alphas_dc_dn6_slot;
        let mut var_alphas_dc_dn7: f64 = *var_alphas_dc_dn7_slot;
        let mut var_alphas_dc_dn8: f64 = *var_alphas_dc_dn8_slot;
        let mut var_alphas_dc_dn9: f64 = *var_alphas_dc_dn9_slot;
        let mut var_asat: f64 = *var_asat_slot;
        let mut var_asat_dn4: f64 = *var_asat_dn4_slot;
        let mut var_asat_dn6: f64 = *var_asat_dn6_slot;
        let mut var_asat_dn7: f64 = *var_asat_dn7_slot;
        let mut var_asat_dn8: f64 = *var_asat_dn8_slot;
        let mut var_asat_dn9: f64 = *var_asat_dn9_slot;
        let mut var_dd: f64 = *var_dd_slot;
        let mut var_dd_dn4: f64 = *var_dd_dn4_slot;
        let mut var_dd_dn6: f64 = *var_dd_dn6_slot;
        let mut var_dd_dn7: f64 = *var_dd_dn7_slot;
        let mut var_dd_dn8: f64 = *var_dd_dn8_slot;
        let mut var_dd_dn9: f64 = *var_dd_dn9_slot;
        let mut var_delta_1s_dc: f64 = *var_delta_1s_dc_slot;
        let mut var_delta_1s_dc_dn4: f64 = *var_delta_1s_dc_dn4_slot;
        let mut var_delta_1s_dc_dn6: f64 = *var_delta_1s_dc_dn6_slot;
        let mut var_delta_1s_dc_dn7: f64 = *var_delta_1s_dc_dn7_slot;
        let mut var_delta_1s_dc_dn8: f64 = *var_delta_1s_dc_dn8_slot;
        let mut var_delta_1s_dc_dn9: f64 = *var_delta_1s_dc_dn9_slot;
        let mut var_delta_ns_dc: f64 = *var_delta_ns_dc_slot;
        let mut var_delta_ns_dc_dn4: f64 = *var_delta_ns_dc_dn4_slot;
        let mut var_delta_ns_dc_dn6: f64 = *var_delta_ns_dc_dn6_slot;
        let mut var_delta_ns_dc_dn7: f64 = *var_delta_ns_dc_dn7_slot;
        let mut var_delta_ns_dc_dn8: f64 = *var_delta_ns_dc_dn8_slot;
        let mut var_delta_ns_dc_dn9: f64 = *var_delta_ns_dc_dn9_slot;
        let mut var_dm: f64 = *var_dm_slot;
        let mut var_dm_dn4: f64 = *var_dm_dn4_slot;
        let mut var_dm_dn6: f64 = *var_dm_dn6_slot;
        let mut var_dm_dn7: f64 = *var_dm_dn7_slot;
        let mut var_dm_dn8: f64 = *var_dm_dn8_slot;
        let mut var_dm_dn9: f64 = *var_dm_dn9_slot;
        let mut var_dps: f64 = *var_dps_slot;
        let mut var_dps_dn4: f64 = *var_dps_dn4_slot;
        let mut var_dps_dn6: f64 = *var_dps_dn6_slot;
        let mut var_dps_dn7: f64 = *var_dps_dn7_slot;
        let mut var_dps_dn8: f64 = *var_dps_dn8_slot;
        let mut var_dps_dn9: f64 = *var_dps_dn9_slot;
        let mut var_ds_dc: f64 = *var_ds_dc_slot;
        let mut var_ds_dc_dn4: f64 = *var_ds_dc_dn4_slot;
        let mut var_ds_dc_dn6: f64 = *var_ds_dc_dn6_slot;
        let mut var_ds_dc_dn7: f64 = *var_ds_dc_dn7_slot;
        let mut var_ds_dc_dn8: f64 = *var_ds_dc_dn8_slot;
        let mut var_ds_dc_dn9: f64 = *var_ds_dc_dn9_slot;
        let mut var_ed: f64 = *var_ed_slot;
        let mut var_ed_dn4: f64 = *var_ed_dn4_slot;
        let mut var_ed_dn6: f64 = *var_ed_dn6_slot;
        let mut var_ed_dn7: f64 = *var_ed_dn7_slot;
        let mut var_ed_dn8: f64 = *var_ed_dn8_slot;
        let mut var_ed_dn9: f64 = *var_ed_dn9_slot;
        let mut var_em: f64 = *var_em_slot;
        let mut var_em_dn4: f64 = *var_em_dn4_slot;
        let mut var_em_dn6: f64 = *var_em_dn6_slot;
        let mut var_em_dn7: f64 = *var_em_dn7_slot;
        let mut var_em_dn8: f64 = *var_em_dn8_slot;
        let mut var_em_dn9: f64 = *var_em_dn9_slot;
        let mut var_es_dc: f64 = *var_es_dc_slot;
        let mut var_es_dc_dn4: f64 = *var_es_dc_dn4_slot;
        let mut var_es_dc_dn6: f64 = *var_es_dc_dn6_slot;
        let mut var_es_dc_dn7: f64 = *var_es_dc_dn7_slot;
        let mut var_es_dc_dn8: f64 = *var_es_dc_dn8_slot;
        let mut var_es_dc_dn9: f64 = *var_es_dc_dn9_slot;
        let mut var_eta_p: f64 = *var_eta_p_slot;
        let mut var_eta_p_dn4: f64 = *var_eta_p_dn4_slot;
        let mut var_eta_p_dn6: f64 = *var_eta_p_dn6_slot;
        let mut var_eta_p_dn7: f64 = *var_eta_p_dn7_slot;
        let mut var_eta_p_dn8: f64 = *var_eta_p_dn8_slot;
        let mut var_eta_p_dn9: f64 = *var_eta_p_dn9_slot;
        let mut var_factheta_dc: f64 = *var_factheta_dc_slot;
        let mut var_factheta_dc_dn4: f64 = *var_factheta_dc_dn4_slot;
        let mut var_factheta_dc_dn6: f64 = *var_factheta_dc_dn6_slot;
        let mut var_factheta_dc_dn7: f64 = *var_factheta_dc_dn7_slot;
        let mut var_factheta_dc_dn8: f64 = *var_factheta_dc_dn8_slot;
        let mut var_factheta_dc_dn9: f64 = *var_factheta_dc_dn9_slot;
        let mut var_gmob: f64 = *var_gmob_slot;
        let mut var_gmob_dn4: f64 = *var_gmob_dn4_slot;
        let mut var_gmob_dn6: f64 = *var_gmob_dn6_slot;
        let mut var_gmob_dn7: f64 = *var_gmob_dn7_slot;
        let mut var_gmob_dn8: f64 = *var_gmob_dn8_slot;
        let mut var_gmob_dn9: f64 = *var_gmob_dn9_slot;
        let mut var_gmobs_dc: f64 = *var_gmobs_dc_slot;
        let mut var_gmobs_dc_dn4: f64 = *var_gmobs_dc_dn4_slot;
        let mut var_gmobs_dc_dn6: f64 = *var_gmobs_dc_dn6_slot;
        let mut var_gmobs_dc_dn7: f64 = *var_gmobs_dc_dn7_slot;
        let mut var_gmobs_dc_dn8: f64 = *var_gmobs_dc_dn8_slot;
        let mut var_gmobs_dc_dn9: f64 = *var_gmobs_dc_dn9_slot;
        let mut var_guard1214: f64 = *var_guard1214_slot;
        let mut var_guard1215: f64 = *var_guard1215_slot;
        let mut var_guard1216: f64 = *var_guard1216_slot;
        let mut var_guard1217: f64 = *var_guard1217_slot;
        let mut var_guard1218: f64 = *var_guard1218_slot;
        let mut var_midphi0: f64 = *var_midphi0_slot;
        let mut var_midphi0_dn4: f64 = *var_midphi0_dn4_slot;
        let mut var_midphi0_dn6: f64 = *var_midphi0_dn6_slot;
        let mut var_midphi0_dn7: f64 = *var_midphi0_dn7_slot;
        let mut var_midphi0_dn8: f64 = *var_midphi0_dn8_slot;
        let mut var_midphi0_dn9: f64 = *var_midphi0_dn9_slot;
        let mut var_pd: f64 = *var_pd_slot;
        let mut var_pd_dn4: f64 = *var_pd_dn4_slot;
        let mut var_pd_dn6: f64 = *var_pd_dn6_slot;
        let mut var_pd_dn7: f64 = *var_pd_dn7_slot;
        let mut var_pd_dn8: f64 = *var_pd_dn8_slot;
        let mut var_pd_dn9: f64 = *var_pd_dn9_slot;
        let mut var_pm: f64 = *var_pm_slot;
        let mut var_pm_dn4: f64 = *var_pm_dn4_slot;
        let mut var_pm_dn6: f64 = *var_pm_dn6_slot;
        let mut var_pm_dn7: f64 = *var_pm_dn7_slot;
        let mut var_pm_dn8: f64 = *var_pm_dn8_slot;
        let mut var_pm_dn9: f64 = *var_pm_dn9_slot;
        let mut var_ps_dc: f64 = *var_ps_dc_slot;
        let mut var_ps_dc_dn4: f64 = *var_ps_dc_dn4_slot;
        let mut var_ps_dc_dn6: f64 = *var_ps_dc_dn6_slot;
        let mut var_ps_dc_dn7: f64 = *var_ps_dc_dn7_slot;
        let mut var_ps_dc_dn8: f64 = *var_ps_dc_dn8_slot;
        let mut var_ps_dc_dn9: f64 = *var_ps_dc_dn9_slot;
        let mut var_qbd: f64 = *var_qbd_slot;
        let mut var_qbd_dn4: f64 = *var_qbd_dn4_slot;
        let mut var_qbd_dn6: f64 = *var_qbd_dn6_slot;
        let mut var_qbd_dn7: f64 = *var_qbd_dn7_slot;
        let mut var_qbd_dn8: f64 = *var_qbd_dn8_slot;
        let mut var_qbd_dn9: f64 = *var_qbd_dn9_slot;
        let mut var_qbm: f64 = *var_qbm_slot;
        let mut var_qbm_dn4: f64 = *var_qbm_dn4_slot;
        let mut var_qbm_dn6: f64 = *var_qbm_dn6_slot;
        let mut var_qbm_dn7: f64 = *var_qbm_dn7_slot;
        let mut var_qbm_dn8: f64 = *var_qbm_dn8_slot;
        let mut var_qbm_dn9: f64 = *var_qbm_dn9_slot;
        let mut var_qbs_dc: f64 = *var_qbs_dc_slot;
        let mut var_qbs_dc_dn4: f64 = *var_qbs_dc_dn4_slot;
        let mut var_qbs_dc_dn6: f64 = *var_qbs_dc_dn6_slot;
        let mut var_qbs_dc_dn7: f64 = *var_qbs_dc_dn7_slot;
        let mut var_qbs_dc_dn8: f64 = *var_qbs_dc_dn8_slot;
        let mut var_qbs_dc_dn9: f64 = *var_qbs_dc_dn9_slot;
        let mut var_qeff1: f64 = *var_qeff1_slot;
        let mut var_qeff1_dn4: f64 = *var_qeff1_dn4_slot;
        let mut var_qeff1_dn6: f64 = *var_qeff1_dn6_slot;
        let mut var_qeff1_dn7: f64 = *var_qeff1_dn7_slot;
        let mut var_qeff1_dn8: f64 = *var_qeff1_dn8_slot;
        let mut var_qeff1_dn9: f64 = *var_qeff1_dn9_slot;
        let mut var_qim: f64 = *var_qim_slot;
        let mut var_qim1: f64 = *var_qim1_slot;
        let mut var_qim1_dn4: f64 = *var_qim1_dn4_slot;
        let mut var_qim1_dn6: f64 = *var_qim1_dn6_slot;
        let mut var_qim1_dn7: f64 = *var_qim1_dn7_slot;
        let mut var_qim1_dn8: f64 = *var_qim1_dn8_slot;
        let mut var_qim1_dn9: f64 = *var_qim1_dn9_slot;
        let mut var_qim_dn4: f64 = *var_qim_dn4_slot;
        let mut var_qim_dn6: f64 = *var_qim_dn6_slot;
        let mut var_qim_dn7: f64 = *var_qim_dn7_slot;
        let mut var_qim_dn8: f64 = *var_qim_dn8_slot;
        let mut var_qim_dn9: f64 = *var_qim_dn9_slot;
        let mut var_qis_dc: f64 = *var_qis_dc_slot;
        let mut var_qis_dc_dn4: f64 = *var_qis_dc_dn4_slot;
        let mut var_qis_dc_dn6: f64 = *var_qis_dc_dn6_slot;
        let mut var_qis_dc_dn7: f64 = *var_qis_dc_dn7_slot;
        let mut var_qis_dc_dn8: f64 = *var_qis_dc_dn8_slot;
        let mut var_qis_dc_dn9: f64 = *var_qis_dc_dn9_slot;
        let mut var_rhob_dc: f64 = *var_rhob_dc_slot;
        let mut var_rhob_dc_dn4: f64 = *var_rhob_dc_dn4_slot;
        let mut var_rhob_dc_dn6: f64 = *var_rhob_dc_dn6_slot;
        let mut var_rhob_dc_dn7: f64 = *var_rhob_dc_dn7_slot;
        let mut var_rhob_dc_dn8: f64 = *var_rhob_dc_dn8_slot;
        let mut var_rhob_dc_dn9: f64 = *var_rhob_dc_dn9_slot;
        let mut var_rhog_dc: f64 = *var_rhog_dc_slot;
        let mut var_rhog_dc_dn4: f64 = *var_rhog_dc_dn4_slot;
        let mut var_rhog_dc_dn6: f64 = *var_rhog_dc_dn6_slot;
        let mut var_rhog_dc_dn7: f64 = *var_rhog_dc_dn7_slot;
        let mut var_rhog_dc_dn8: f64 = *var_rhog_dc_dn8_slot;
        let mut var_rhog_dc_dn9: f64 = *var_rhog_dc_dn9_slot;
        let mut var_rxcor_dc: f64 = *var_rxcor_dc_slot;
        let mut var_rxcor_dc_dn4: f64 = *var_rxcor_dc_dn4_slot;
        let mut var_rxcor_dc_dn6: f64 = *var_rxcor_dc_dn6_slot;
        let mut var_rxcor_dc_dn7: f64 = *var_rxcor_dc_dn7_slot;
        let mut var_rxcor_dc_dn8: f64 = *var_rxcor_dc_dn8_slot;
        let mut var_rxcor_dc_dn9: f64 = *var_rxcor_dc_dn9_slot;
        let mut var_s1: f64 = *var_s1_slot;
        let mut var_s1_dn4: f64 = *var_s1_dn4_slot;
        let mut var_s1_dn6: f64 = *var_s1_dn6_slot;
        let mut var_s1_dn7: f64 = *var_s1_dn7_slot;
        let mut var_s1_dn8: f64 = *var_s1_dn8_slot;
        let mut var_s1_dn9: f64 = *var_s1_dn9_slot;
        let mut var_sqm: f64 = *var_sqm_slot;
        let mut var_sqm_dn4: f64 = *var_sqm_dn4_slot;
        let mut var_sqm_dn6: f64 = *var_sqm_dn6_slot;
        let mut var_sqm_dn7: f64 = *var_sqm_dn7_slot;
        let mut var_sqm_dn8: f64 = *var_sqm_dn8_slot;
        let mut var_sqm_dn9: f64 = *var_sqm_dn9_slot;
        let mut var_sqs_dc: f64 = *var_sqs_dc_slot;
        let mut var_sqs_dc_dn4: f64 = *var_sqs_dc_dn4_slot;
        let mut var_sqs_dc_dn6: f64 = *var_sqs_dc_dn6_slot;
        let mut var_sqs_dc_dn7: f64 = *var_sqs_dc_dn7_slot;
        let mut var_sqs_dc_dn8: f64 = *var_sqs_dc_dn8_slot;
        let mut var_sqs_dc_dn9: f64 = *var_sqs_dc_dn9_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp__blk949: f64 = *var_temp__blk949_slot;
        let mut var_temp__blk949_dn4: f64 = *var_temp__blk949_dn4_slot;
        let mut var_temp__blk949_dn6: f64 = *var_temp__blk949_dn6_slot;
        let mut var_temp__blk949_dn7: f64 = *var_temp__blk949_dn7_slot;
        let mut var_temp__blk949_dn8: f64 = *var_temp__blk949_dn8_slot;
        let mut var_temp__blk949_dn9: f64 = *var_temp__blk949_dn9_slot;
        let mut var_thesat1: f64 = *var_thesat1_slot;
        let mut var_thesat1_dn4: f64 = *var_thesat1_dn4_slot;
        let mut var_thesat1_dn6: f64 = *var_thesat1_dn6_slot;
        let mut var_thesat1_dn7: f64 = *var_thesat1_dn7_slot;
        let mut var_thesat1_dn8: f64 = *var_thesat1_dn8_slot;
        let mut var_thesat1_dn9: f64 = *var_thesat1_dn9_slot;
        let mut var_thesateff: f64 = *var_thesateff_slot;
        let mut var_thesateff_dn4: f64 = *var_thesateff_dn4_slot;
        let mut var_thesateff_dn6: f64 = *var_thesateff_dn6_slot;
        let mut var_thesateff_dn7: f64 = *var_thesateff_dn7_slot;
        let mut var_thesateff_dn8: f64 = *var_thesateff_dn8_slot;
        let mut var_thesateff_dn9: f64 = *var_thesateff_dn9_slot;
        let mut var_udse: f64 = *var_udse_slot;
        let mut var_udse_dn4: f64 = *var_udse_dn4_slot;
        let mut var_udse_dn6: f64 = *var_udse_dn6_slot;
        let mut var_udse_dn7: f64 = *var_udse_dn7_slot;
        let mut var_udse_dn8: f64 = *var_udse_dn8_slot;
        let mut var_udse_dn9: f64 = *var_udse_dn9_slot;
        let mut var_v_dsat: f64 = *var_v_dsat_slot;
        let mut var_v_dsat_dn4: f64 = *var_v_dsat_dn4_slot;
        let mut var_v_dsat_dn6: f64 = *var_v_dsat_dn6_slot;
        let mut var_v_dsat_dn7: f64 = *var_v_dsat_dn7_slot;
        let mut var_v_dsat_dn8: f64 = *var_v_dsat_dn8_slot;
        let mut var_v_dsat_dn9: f64 = *var_v_dsat_dn9_slot;
        let mut var_vdsat_lim: f64 = *var_vdsat_lim_slot;
        let mut var_vdsat_lim_dn4: f64 = *var_vdsat_lim_dn4_slot;
        let mut var_vdsat_lim_dn6: f64 = *var_vdsat_lim_dn6_slot;
        let mut var_vdsat_lim_dn7: f64 = *var_vdsat_lim_dn7_slot;
        let mut var_vdsat_lim_dn8: f64 = *var_vdsat_lim_dn8_slot;
        let mut var_vdsat_lim_dn9: f64 = *var_vdsat_lim_dn9_slot;
        let mut var_vdse: f64 = *var_vdse_slot;
        let mut var_vdse_dn4: f64 = *var_vdse_dn4_slot;
        let mut var_vdse_dn6: f64 = *var_vdse_dn6_slot;
        let mut var_vdse_dn7: f64 = *var_vdse_dn7_slot;
        let mut var_vdse_dn8: f64 = *var_vdse_dn8_slot;
        let mut var_vdse_dn9: f64 = *var_vdse_dn9_slot;
        let mut var_voxm: f64 = *var_voxm_slot;
        let mut var_voxm_dn4: f64 = *var_voxm_dn4_slot;
        let mut var_voxm_dn6: f64 = *var_voxm_dn6_slot;
        let mut var_voxm_dn7: f64 = *var_voxm_dn7_slot;
        let mut var_voxm_dn8: f64 = *var_voxm_dn8_slot;
        let mut var_voxm_dn9: f64 = *var_voxm_dn9_slot;
        let mut var_x_d: f64 = *var_x_d_slot;
        let mut var_x_d_dn4: f64 = *var_x_d_dn4_slot;
        let mut var_x_d_dn6: f64 = *var_x_d_dn6_slot;
        let mut var_x_d_dn7: f64 = *var_x_d_dn7_slot;
        let mut var_x_d_dn8: f64 = *var_x_d_dn8_slot;
        let mut var_x_d_dn9: f64 = *var_x_d_dn9_slot;
        let mut var_x_ds: f64 = *var_x_ds_slot;
        let mut var_x_ds_dn4: f64 = *var_x_ds_dn4_slot;
        let mut var_x_ds_dn6: f64 = *var_x_ds_dn6_slot;
        let mut var_x_ds_dn7: f64 = *var_x_ds_dn7_slot;
        let mut var_x_ds_dn8: f64 = *var_x_ds_dn8_slot;
        let mut var_x_ds_dn9: f64 = *var_x_ds_dn9_slot;
        let mut var_x_inf0: f64 = *var_x_inf0_slot;
        let mut var_x_inf0_dn4: f64 = *var_x_inf0_dn4_slot;
        let mut var_x_inf0_dn6: f64 = *var_x_inf0_dn6_slot;
        let mut var_x_inf0_dn7: f64 = *var_x_inf0_dn7_slot;
        let mut var_x_inf0_dn8: f64 = *var_x_inf0_dn8_slot;
        let mut var_x_inf0_dn9: f64 = *var_x_inf0_dn9_slot;
        let mut var_x_m: f64 = *var_x_m_slot;
        let mut var_x_m_dn4: f64 = *var_x_m_dn4_slot;
        let mut var_x_m_dn6: f64 = *var_x_m_dn6_slot;
        let mut var_x_m_dn7: f64 = *var_x_m_dn7_slot;
        let mut var_x_m_dn8: f64 = *var_x_m_dn8_slot;
        let mut var_x_m_dn9: f64 = *var_x_m_dn9_slot;
        let mut var_x_s_dc: f64 = *var_x_s_dc_slot;
        let mut var_x_s_dc_dn4: f64 = *var_x_s_dc_dn4_slot;
        let mut var_x_s_dc_dn6: f64 = *var_x_s_dc_dn6_slot;
        let mut var_x_s_dc_dn7: f64 = *var_x_s_dc_dn7_slot;
        let mut var_x_s_dc_dn8: f64 = *var_x_s_dc_dn8_slot;
        let mut var_x_s_dc_dn9: f64 = *var_x_s_dc_dn9_slot;
        let mut var_xgm: f64 = *var_xgm_slot;
        let mut var_xgm_dn4: f64 = *var_xgm_dn4_slot;
        let mut var_xgm_dn6: f64 = *var_xgm_dn6_slot;
        let mut var_xgm_dn7: f64 = *var_xgm_dn7_slot;
        let mut var_xgm_dn8: f64 = *var_xgm_dn8_slot;
        let mut var_xgm_dn9: f64 = *var_xgm_dn9_slot;
        let mut var_xgs_dc: f64 = *var_xgs_dc_slot;
        let mut var_xgs_dc_dn4: f64 = *var_xgs_dc_dn4_slot;
        let mut var_xgs_dc_dn6: f64 = *var_xgs_dc_dn6_slot;
        let mut var_xgs_dc_dn7: f64 = *var_xgs_dc_dn7_slot;
        let mut var_xgs_dc_dn8: f64 = *var_xgs_dc_dn8_slot;
        let mut var_xgs_dc_dn9: f64 = *var_xgs_dc_dn9_slot;
        let mut var_xi1s_dc: f64 = *var_xi1s_dc_slot;
        let mut var_xi1s_dc_dn4: f64 = *var_xi1s_dc_dn4_slot;
        let mut var_xi1s_dc_dn6: f64 = *var_xi1s_dc_dn6_slot;
        let mut var_xi1s_dc_dn7: f64 = *var_xi1s_dc_dn7_slot;
        let mut var_xi1s_dc_dn8: f64 = *var_xi1s_dc_dn8_slot;
        let mut var_xi1s_dc_dn9: f64 = *var_xi1s_dc_dn9_slot;
        let mut var_xi2s_dc: f64 = *var_xi2s_dc_slot;
        let mut var_xi2s_dc_dn4: f64 = *var_xi2s_dc_dn4_slot;
        let mut var_xi2s_dc_dn6: f64 = *var_xi2s_dc_dn6_slot;
        let mut var_xi2s_dc_dn7: f64 = *var_xi2s_dc_dn7_slot;
        let mut var_xi2s_dc_dn8: f64 = *var_xi2s_dc_dn8_slot;
        let mut var_xi2s_dc_dn9: f64 = *var_xi2s_dc_dn9_slot;
        let mut var_xitsb_dc: f64 = *var_xitsb_dc_slot;
        let mut var_xitsb_dc_dn4: f64 = *var_xitsb_dc_dn4_slot;
        let mut var_xitsb_dc_dn6: f64 = *var_xitsb_dc_dn6_slot;
        let mut var_xitsb_dc_dn7: f64 = *var_xitsb_dc_dn7_slot;
        let mut var_xitsb_dc_dn8: f64 = *var_xitsb_dc_dn8_slot;
        let mut var_xitsb_dc_dn9: f64 = *var_xitsb_dc_dn9_slot;

        var_delta_ns_dc = var_delta_ns;
        var_delta_ns_dc_dn4 = var_delta_ns_dn4;
        var_delta_ns_dc_dn6 = var_delta_ns_dn6;
        var_delta_ns_dc_dn7 = var_delta_ns_dn7;
        var_delta_ns_dc_dn8 = var_delta_ns_dn8;
        var_delta_ns_dc_dn9 = var_delta_ns_dn9;

        var_x_s_dc = var_x_s;
        var_x_s_dc_dn4 = var_x_s_dn4;
        var_x_s_dc_dn6 = var_x_s_dn6;
        var_x_s_dc_dn7 = var_x_s_dn7;
        var_x_s_dc_dn8 = var_x_s_dn8;
        var_x_s_dc_dn9 = var_x_s_dn9;

        var_xi1s_dc = var_xi1s;
        var_xi1s_dc_dn4 = var_xi1s_dn4;
        var_xi1s_dc_dn6 = var_xi1s_dn6;
        var_xi1s_dc_dn7 = var_xi1s_dn7;
        var_xi1s_dc_dn8 = var_xi1s_dn8;
        var_xi1s_dc_dn9 = var_xi1s_dn9;

        var_xi2s_dc = var_xi2s;
        var_xi2s_dc_dn4 = var_xi2s_dn4;
        var_xi2s_dc_dn6 = var_xi2s_dn6;
        var_xi2s_dc_dn7 = var_xi2s_dn7;
        var_xi2s_dc_dn8 = var_xi2s_dn8;
        var_xi2s_dc_dn9 = var_xi2s_dn9;

        var_delta_1s_dc = var_delta_1s;
        var_delta_1s_dc_dn4 = var_delta_1s_dn4;
        var_delta_1s_dc_dn6 = var_delta_1s_dn6;
        var_delta_1s_dc_dn7 = var_delta_1s_dn7;
        var_delta_1s_dc_dn8 = var_delta_1s_dn8;
        var_delta_1s_dc_dn9 = var_delta_1s_dn9;

        var_es_dc = var_es;
        var_es_dc_dn4 = var_es_dn4;
        var_es_dc_dn6 = var_es_dn6;
        var_es_dc_dn7 = var_es_dn7;
        var_es_dc_dn8 = var_es_dn8;
        var_es_dc_dn9 = var_es_dn9;

        var_ps_dc = var_ps;
        var_ps_dc_dn4 = var_ps_dn4;
        var_ps_dc_dn6 = var_ps_dn6;
        var_ps_dc_dn7 = var_ps_dn7;
        var_ps_dc_dn8 = var_ps_dn8;
        var_ps_dc_dn9 = var_ps_dn9;

        var_ds_dc = var_ds;
        var_ds_dc_dn4 = var_ds_dn4;
        var_ds_dc_dn6 = var_ds_dn6;
        var_ds_dc_dn7 = var_ds_dn7;
        var_ds_dc_dn8 = var_ds_dn8;
        var_ds_dc_dn9 = var_ds_dn9;

        var_sqs_dc = var_sqs;
        var_sqs_dc_dn4 = var_sqs_dn4;
        var_sqs_dc_dn6 = var_sqs_dn6;
        var_sqs_dc_dn7 = var_sqs_dn7;
        var_sqs_dc_dn8 = var_sqs_dn8;
        var_sqs_dc_dn9 = var_sqs_dn9;

        var_alphas_dc = var_alphas;
        var_alphas_dc_dn4 = var_alphas_dn4;
        var_alphas_dc_dn6 = var_alphas_dn6;
        var_alphas_dc_dn7 = var_alphas_dn7;
        var_alphas_dc_dn8 = var_alphas_dn8;
        var_alphas_dc_dn9 = var_alphas_dn9;

        var_rxcor_dc = var_rxcor;
        var_rxcor_dc_dn4 = var_rxcor_dn4;
        var_rxcor_dc_dn6 = var_rxcor_dn6;
        var_rxcor_dc_dn7 = var_rxcor_dn7;
        var_rxcor_dc_dn8 = var_rxcor_dn8;
        var_rxcor_dc_dn9 = var_rxcor_dn9;

        var_xgs_dc = var_xgs;
        var_xgs_dc_dn4 = var_xgs_dn4;
        var_xgs_dc_dn6 = var_xgs_dn6;
        var_xgs_dc_dn7 = var_xgs_dn7;
        var_xgs_dc_dn8 = var_xgs_dn8;
        var_xgs_dc_dn9 = var_xgs_dn9;

        var_qis_dc = var_qis;
        var_qis_dc_dn4 = var_qis_dn4;
        var_qis_dc_dn6 = var_qis_dn6;
        var_qis_dc_dn7 = var_qis_dn7;
        var_qis_dc_dn8 = var_qis_dn8;
        var_qis_dc_dn9 = var_qis_dn9;

        var_qbs_dc = var_qbs;
        var_qbs_dc_dn4 = var_qbs_dn4;
        var_qbs_dc_dn6 = var_qbs_dn6;
        var_qbs_dc_dn7 = var_qbs_dn7;
        var_qbs_dc_dn8 = var_qbs_dn8;
        var_qbs_dc_dn9 = var_qbs_dn9;

        var_rhob_dc = var_rhob;
        var_rhob_dc_dn4 = var_rhob_dn4;
        var_rhob_dc_dn6 = var_rhob_dn6;
        var_rhob_dc_dn7 = var_rhob_dn7;
        var_rhob_dc_dn8 = var_rhob_dn8;
        var_rhob_dc_dn9 = var_rhob_dn9;

        var_rhog_dc = var_rhog;
        var_rhog_dc_dn4 = var_rhog_dn4;
        var_rhog_dc_dn6 = var_rhog_dn6;
        var_rhog_dc_dn7 = var_rhog_dn7;
        var_rhog_dc_dn8 = var_rhog_dn8;
        var_rhog_dc_dn9 = var_rhog_dn9;

        var_gmobs_dc = var_gmobs;
        var_gmobs_dc_dn4 = var_gmobs_dn4;
        var_gmobs_dc_dn6 = var_gmobs_dn6;
        var_gmobs_dc_dn7 = var_gmobs_dn7;
        var_gmobs_dc_dn8 = var_gmobs_dn8;
        var_gmobs_dc_dn9 = var_gmobs_dn9;

        var_xitsb_dc = var_xitsb;
        var_xitsb_dc_dn4 = var_xitsb_dn4;
        var_xitsb_dc_dn6 = var_xitsb_dn6;
        var_xitsb_dc_dn7 = var_xitsb_dn7;
        var_xitsb_dc_dn8 = var_xitsb_dn8;
        var_xitsb_dc_dn9 = var_xitsb_dn9;

        var_factheta_dc = var_factheta;
        var_factheta_dc_dn4 = var_factheta_dn4;
        var_factheta_dc_dn6 = var_factheta_dn6;
        var_factheta_dc_dn7 = var_factheta_dn7;
        var_factheta_dc_dn8 = var_factheta_dn8;
        var_factheta_dc_dn9 = var_factheta_dn9;

        var_thesat1 = 0.0;
        var_thesat1_dn4 = 0.0;
        var_thesat1_dn6 = 0.0;
        var_thesat1_dn7 = 0.0;
        var_thesat1_dn8 = 0.0;
        var_thesat1_dn9 = 0.0;

        let assign43480_e56510: f64 = (var_phit1 * 4.60517018598809);
        var_vdsat_lim = assign43480_e56510;
        var_vdsat_lim_dn4 = (var_phit1_dn4 * 4.60517018598809);
        var_vdsat_lim_dn6 = (var_phit1_dn6 * 4.60517018598809);
        var_vdsat_lim_dn7 = (var_phit1_dn7 * 4.60517018598809);
        var_vdsat_lim_dn8 = (var_phit1_dn8 * 4.60517018598809);
        var_vdsat_lim_dn9 = (var_phit1_dn9 * 4.60517018598809);

        var_v_dsat = var_vdsat_lim;
        var_v_dsat_dn4 = var_vdsat_lim_dn4;
        var_v_dsat_dn6 = var_vdsat_lim_dn6;
        var_v_dsat_dn7 = var_vdsat_lim_dn7;
        var_v_dsat_dn8 = var_vdsat_lim_dn8;
        var_v_dsat_dn9 = var_vdsat_lim_dn9;

        var_vdse = var_v_ds;
        var_vdse_dn4 = 0.0;
        var_vdse_dn6 = 0.0;
        var_vdse_dn7 = var_v_ds_dn7;
        var_vdse_dn8 = var_v_ds_dn8;
        var_vdse_dn9 = 0.0;

        let assign43510_e56515: f64 = (var_v_ds * var_inv_phit1);
        var_udse = assign43510_e56515;
        var_udse_dn4 = (var_v_ds * var_inv_phit1_dn4);
        var_udse_dn6 = (var_v_ds * var_inv_phit1_dn6);
        var_udse_dn7 = ((var_v_ds_dn7 * var_inv_phit1) + (var_v_ds * var_inv_phit1_dn7));
        var_udse_dn8 = ((var_v_ds_dn8 * var_inv_phit1) + (var_v_ds * var_inv_phit1_dn8));
        var_udse_dn9 = (var_v_ds * var_inv_phit1_dn9);

        var_x_d = var_x_s;
        var_x_d_dn4 = var_x_s_dn4;
        var_x_d_dn6 = var_x_s_dn6;
        var_x_d_dn7 = var_x_s_dn7;
        var_x_d_dn8 = var_x_s_dn8;
        var_x_d_dn9 = var_x_s_dn9;

        var_x_ds = 0.0;
        var_x_ds_dn4 = 0.0;
        var_x_ds_dn6 = 0.0;
        var_x_ds_dn7 = 0.0;
        var_x_ds_dn8 = 0.0;
        var_x_ds_dn9 = 0.0;

        var_dps = 0.0;
        var_dps_dn4 = 0.0;
        var_dps_dn6 = 0.0;
        var_dps_dn7 = 0.0;
        var_dps_dn8 = 0.0;
        var_dps_dn9 = 0.0;

        var_ed = var_es;
        var_ed_dn4 = var_es_dn4;
        var_ed_dn6 = var_es_dn6;
        var_ed_dn7 = var_es_dn7;
        var_ed_dn8 = var_es_dn8;
        var_ed_dn9 = var_es_dn9;

        var_pd = var_ps;
        var_pd_dn4 = var_ps_dn4;
        var_pd_dn6 = var_ps_dn6;
        var_pd_dn7 = var_ps_dn7;
        var_pd_dn8 = var_ps_dn8;
        var_pd_dn9 = var_ps_dn9;

        var_dd = var_ds;
        var_dd_dn4 = var_ds_dn4;
        var_dd_dn6 = var_ds_dn6;
        var_dd_dn7 = var_ds_dn7;
        var_dd_dn8 = var_ds_dn8;
        var_dd_dn9 = var_ds_dn9;

        var_qbd = var_qbs;
        var_qbd_dn4 = var_qbs_dn4;
        var_qbd_dn6 = var_qbs_dn6;
        var_qbd_dn7 = var_qbs_dn7;
        var_qbd_dn8 = var_qbs_dn8;
        var_qbd_dn9 = var_qbs_dn9;

        var_x_m = var_x_s;
        var_x_m_dn4 = var_x_s_dn4;
        var_x_m_dn6 = var_x_s_dn6;
        var_x_m_dn7 = var_x_s_dn7;
        var_x_m_dn8 = var_x_s_dn8;
        var_x_m_dn9 = var_x_s_dn9;

        var_em = var_es;
        var_em_dn4 = var_es_dn4;
        var_em_dn6 = var_es_dn6;
        var_em_dn7 = var_es_dn7;
        var_em_dn8 = var_es_dn8;
        var_em_dn9 = var_es_dn9;

        var_dm = var_ds;
        var_dm_dn4 = var_ds_dn4;
        var_dm_dn6 = var_ds_dn6;
        var_dm_dn7 = var_ds_dn7;
        var_dm_dn8 = var_ds_dn8;
        var_dm_dn9 = var_ds_dn9;

        var_pm = var_ps;
        var_pm_dn4 = var_ps_dn4;
        var_pm_dn6 = var_ps_dn6;
        var_pm_dn7 = var_ps_dn7;
        var_pm_dn8 = var_ps_dn8;
        var_pm_dn9 = var_ps_dn9;

        let assign43630_e56529: f64 = (var_xg - var_x_s);
        var_xgm = assign43630_e56529;
        var_xgm_dn4 = (var_xg_dn4 - var_x_s_dn4);
        var_xgm_dn6 = (var_xg_dn6 - var_x_s_dn6);
        var_xgm_dn7 = (var_xg_dn7 - var_x_s_dn7);
        var_xgm_dn8 = (var_xg_dn8 - var_x_s_dn8);
        var_xgm_dn9 = (var_xg_dn9 - var_x_s_dn9);

        var_eta_p = 1.0;
        var_eta_p_dn4 = 0.0;
        var_eta_p_dn6 = 0.0;
        var_eta_p_dn7 = 0.0;
        var_eta_p_dn8 = 0.0;
        var_eta_p_dn9 = 0.0;

        var_alpha = 1.0;
        var_alpha_dn4 = 0.0;
        var_alpha_dn6 = 0.0;
        var_alpha_dn7 = 0.0;
        var_alpha_dn8 = 0.0;
        var_alpha_dn9 = 0.0;

        var_sqm = 0.0;
        var_sqm_dn4 = 0.0;
        var_sqm_dn6 = 0.0;
        var_sqm_dn7 = 0.0;
        var_sqm_dn8 = 0.0;
        var_sqm_dn9 = 0.0;

        var_qim = var_qis;
        var_qim_dn4 = var_qis_dn4;
        var_qim_dn6 = var_qis_dn6;
        var_qim_dn7 = var_qis_dn7;
        var_qim_dn8 = var_qis_dn8;
        var_qim_dn9 = var_qis_dn9;

        let assign43680_e56536: f64 = (var_xgm * var_phit1);
        var_qeff1 = assign43680_e56536;
        var_qeff1_dn4 = ((var_xgm_dn4 * var_phit1) + (var_xgm * var_phit1_dn4));
        var_qeff1_dn6 = ((var_xgm_dn6 * var_phit1) + (var_xgm * var_phit1_dn6));
        var_qeff1_dn7 = ((var_xgm_dn7 * var_phit1) + (var_xgm * var_phit1_dn7));
        var_qeff1_dn8 = ((var_xgm_dn8 * var_phit1) + (var_xgm * var_phit1_dn8));
        var_qeff1_dn9 = ((var_xgm_dn9 * var_phit1) + (var_xgm * var_phit1_dn9));

        var_qim1 = 0.0;
        var_qim1_dn4 = 0.0;
        var_qim1_dn6 = 0.0;
        var_qim1_dn7 = 0.0;
        var_qim1_dn8 = 0.0;
        var_qim1_dn9 = 0.0;

        var_qbm = var_qbs;
        var_qbm_dn4 = var_qbs_dn4;
        var_qbm_dn6 = var_qbs_dn6;
        var_qbm_dn7 = var_qbs_dn7;
        var_qbm_dn8 = var_qbs_dn8;
        var_qbm_dn9 = var_qbs_dn9;

        var_s1 = 0.0;
        var_s1_dn4 = 0.0;
        var_s1_dn6 = 0.0;
        var_s1_dn7 = 0.0;
        var_s1_dn8 = 0.0;
        var_s1_dn9 = 0.0;

        var_gmob = 1.0;
        var_gmob_dn4 = 0.0;
        var_gmob_dn6 = 0.0;
        var_gmob_dn7 = 0.0;
        var_gmob_dn8 = 0.0;
        var_gmob_dn9 = 0.0;

        var_thesateff = var_thesatloc;
        var_thesateff_dn4 = var_thesatloc_dn4;
        var_thesateff_dn6 = 0.0;
        var_thesateff_dn7 = 0.0;
        var_thesateff_dn8 = 0.0;
        var_thesateff_dn9 = 0.0;

        var_voxm = var_qeff1;
        var_voxm_dn4 = var_qeff1_dn4;
        var_voxm_dn6 = var_qeff1_dn6;
        var_voxm_dn7 = var_qeff1_dn7;
        var_voxm_dn8 = var_qeff1_dn8;
        var_voxm_dn9 = var_qeff1_dn9;

        let assign43750_e56545: f64 = if var_xg > 0.0 { 1.0 } else { 0.0 };
        var_guard1214 = assign43750_e56545;

        let assign43760_e56548: f64 = if var_ds > 1e-100 { 1.0 } else { 0.0 };
        var_guard1215 = assign43760_e56548;

        let (assign43770_e56556, assign43770_e56556_d_n4, assign43770_e56556_d_n6, assign43770_e56556_d_n7, assign43770_e56556_d_n8, assign43770_e56556_d_n9,) = {
    if ((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) {
        let assign43770_e56554: f64 = (var_thesatloc * var_factheta);
        (assign43770_e56554, ((var_thesatloc_dn4 * var_factheta) + (var_thesatloc * var_factheta_dn4)), (var_thesatloc * var_factheta_dn6), (var_thesatloc * var_factheta_dn7), (var_thesatloc * var_factheta_dn8), (var_thesatloc * var_factheta_dn9),)
    } else {
        (var_thesateff, var_thesateff_dn4, var_thesateff_dn6, var_thesateff_dn7, var_thesateff_dn8, var_thesateff_dn9,)
    }
};
        var_thesateff = assign43770_e56556;
        var_thesateff_dn4 = assign43770_e56556_d_n4;
        var_thesateff_dn6 = assign43770_e56556_d_n6;
        var_thesateff_dn7 = assign43770_e56556_d_n7;
        var_thesateff_dn8 = assign43770_e56556_d_n8;
        var_thesateff_dn9 = assign43770_e56556_d_n9;

        let (assign43780_e56564, assign43780_e56564_d_n4, assign43780_e56564_d_n6, assign43780_e56564_d_n7, assign43780_e56564_d_n8, assign43780_e56564_d_n9,) = {
    if ((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) {
        let assign43780_e56562: f64 = (var_thesateff / var_gmobs);
        (assign43780_e56562, (((var_thesateff_dn4 * var_gmobs) - (var_thesateff * var_gmobs_dn4)) / (var_gmobs * var_gmobs)), (((var_thesateff_dn6 * var_gmobs) - (var_thesateff * var_gmobs_dn6)) / (var_gmobs * var_gmobs)), (((var_thesateff_dn7 * var_gmobs) - (var_thesateff * var_gmobs_dn7)) / (var_gmobs * var_gmobs)), (((var_thesateff_dn8 * var_gmobs) - (var_thesateff * var_gmobs_dn8)) / (var_gmobs * var_gmobs)), (((var_thesateff_dn9 * var_gmobs) - (var_thesateff * var_gmobs_dn9)) / (var_gmobs * var_gmobs)),)
    } else {
        (var_thesat1, var_thesat1_dn4, var_thesat1_dn6, var_thesat1_dn7, var_thesat1_dn8, var_thesat1_dn9,)
    }
};
        var_thesat1 = assign43780_e56564;
        var_thesat1_dn4 = assign43780_e56564_d_n4;
        var_thesat1_dn6 = assign43780_e56564_d_n6;
        var_thesat1_dn7 = assign43780_e56564_d_n7;
        var_thesat1_dn8 = assign43780_e56564_d_n8;
        var_thesat1_dn9 = assign43780_e56564_d_n9;

        let (assign43790_e56574, assign43790_e56574_d_n4, assign43790_e56574_d_n6, assign43790_e56574_d_n7, assign43790_e56574_d_n8, assign43790_e56574_d_n9,) = {
    if ((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) {
        let assign43790_e56571: f64 = (0.5 * var_gf2);
        let assign43790_e56572: f64 = (var_xgs + assign43790_e56571);
        (assign43790_e56572, (var_xgs_dn4 + (0.5 * var_gf2_dn4)), (var_xgs_dn6 + (0.5 * var_gf2_dn6)), (var_xgs_dn7 + (0.5 * var_gf2_dn7)), (var_xgs_dn8 + (0.5 * var_gf2_dn8)), (var_xgs_dn9 + (0.5 * var_gf2_dn9)),)
    } else {
        (var_asat, var_asat_dn4, var_asat_dn6, var_asat_dn7, var_asat_dn8, var_asat_dn9,)
    }
};
        var_asat = assign43790_e56574;
        var_asat_dn4 = assign43790_e56574_d_n4;
        var_asat_dn6 = assign43790_e56574_d_n6;
        var_asat_dn7 = assign43790_e56574_d_n7;
        var_asat_dn8 = assign43790_e56574_d_n8;
        var_asat_dn9 = assign43790_e56574_d_n9;

        let (assign43800_e56586, assign43800_e56586_d_n4, assign43800_e56586_d_n6, assign43800_e56586_d_n7, assign43800_e56586_d_n8, assign43800_e56586_d_n9,) = {
    if ((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) {
        let assign43800_e56580: f64 = (var_gf2 * var_delta_1s);
        let __rspice_inv_cse_0: f64 = 1.0 / var_asat;
        let assign43800_e56582: f64 = (assign43800_e56580 * __rspice_inv_cse_0);
        let assign43800_e56584: f64 = (assign43800_e56582 * __rspice_inv_cse_0);
        (assign43800_e56584, ((((((((var_gf2_dn4 * var_delta_1s) + (var_gf2 * var_delta_1s_dn4)) * var_asat) - (assign43800_e56580 * var_asat_dn4)) / (var_asat * var_asat)) * var_asat) - (assign43800_e56582 * var_asat_dn4)) / (var_asat * var_asat)), ((((((((var_gf2_dn6 * var_delta_1s) + (var_gf2 * var_delta_1s_dn6)) * var_asat) - (assign43800_e56580 * var_asat_dn6)) / (var_asat * var_asat)) * var_asat) - (assign43800_e56582 * var_asat_dn6)) / (var_asat * var_asat)), ((((((((var_gf2_dn7 * var_delta_1s) + (var_gf2 * var_delta_1s_dn7)) * var_asat) - (assign43800_e56580 * var_asat_dn7)) / (var_asat * var_asat)) * var_asat) - (assign43800_e56582 * var_asat_dn7)) / (var_asat * var_asat)), ((((((((var_gf2_dn8 * var_delta_1s) + (var_gf2 * var_delta_1s_dn8)) * var_asat) - (assign43800_e56580 * var_asat_dn8)) / (var_asat * var_asat)) * var_asat) - (assign43800_e56582 * var_asat_dn8)) / (var_asat * var_asat)), ((((((((var_gf2_dn9 * var_delta_1s) + (var_gf2 * var_delta_1s_dn9)) * var_asat) - (assign43800_e56580 * var_asat_dn9)) / (var_asat * var_asat)) * var_asat) - (assign43800_e56582 * var_asat_dn9)) / (var_asat * var_asat)),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign43800_e56586;
        var_temp__blk949_dn4 = assign43800_e56586_d_n4;
        var_temp__blk949_dn6 = assign43800_e56586_d_n6;
        var_temp__blk949_dn7 = assign43800_e56586_d_n7;
        var_temp__blk949_dn8 = assign43800_e56586_d_n8;
        var_temp__blk949_dn9 = assign43800_e56586_d_n9;

        let assign43810_e56589: f64 = if var_temp__blk949 > 0.0001 { 1.0 } else { 0.0 };
        var_guard1216 = assign43810_e56589;

        let (assign43820_e56599, assign43820_e56599_d_n4, assign43820_e56599_d_n6, assign43820_e56599_d_n7, assign43820_e56599_d_n8, assign43820_e56599_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1216 != 0.0)) {
        let assign43820_e56597: f64 = (1.0 - var_temp__blk949);
        (assign43820_e56597, (-var_temp__blk949_dn4), (-var_temp__blk949_dn6), (-var_temp__blk949_dn7), (-var_temp__blk949_dn8), (-var_temp__blk949_dn9),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign43820_e56599;
        var_temp1_dn4 = assign43820_e56599_d_n4;
        var_temp1_dn6 = assign43820_e56599_d_n6;
        var_temp1_dn7 = assign43820_e56599_d_n7;
        var_temp1_dn8 = assign43820_e56599_d_n8;
        var_temp1_dn9 = assign43820_e56599_d_n9;

        let assign43830_e56602: f64 = if var_temp1 < 1e-10 { 1.0 } else { 0.0 };
        var_guard1217 = assign43830_e56602;

        let (assign43840_e56612, assign43840_e56612_d_n4, assign43840_e56612_d_n6, assign43840_e56612_d_n7, assign43840_e56612_d_n8, assign43840_e56612_d_n9,) = {
    if ((((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1216 != 0.0)) && (var_guard1217 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign43840_e56612;
        var_temp2_dn4 = assign43840_e56612_d_n4;
        var_temp2_dn6 = assign43840_e56612_d_n6;
        var_temp2_dn7 = assign43840_e56612_d_n7;
        var_temp2_dn8 = assign43840_e56612_d_n8;
        var_temp2_dn9 = assign43840_e56612_d_n9;

        let (assign43850_e56626, assign43850_e56626_d_n4, assign43850_e56626_d_n6, assign43850_e56626_d_n7, assign43850_e56626_d_n8, assign43850_e56626_d_n9,) = {
    if ((((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1216 != 0.0)) && (var_guard1217 == 0.0)) {
        let assign43850_e56623: f64 = (var_temp1).sqrt();
        let assign43850_e56624: f64 = (1.0 - assign43850_e56623);
        (assign43850_e56624, (-(var_temp1_dn4 / (2.0 * assign43850_e56623))), (-(var_temp1_dn6 / (2.0 * assign43850_e56623))), (-(var_temp1_dn7 / (2.0 * assign43850_e56623))), (-(var_temp1_dn8 / (2.0 * assign43850_e56623))), (-(var_temp1_dn9 / (2.0 * assign43850_e56623))),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign43850_e56626;
        var_temp2_dn4 = assign43850_e56626_d_n4;
        var_temp2_dn6 = assign43850_e56626_d_n6;
        var_temp2_dn7 = assign43850_e56626_d_n7;
        var_temp2_dn8 = assign43850_e56626_d_n8;
        var_temp2_dn9 = assign43850_e56626_d_n9;

        let (assign43860_e56637, assign43860_e56637_d_n4, assign43860_e56637_d_n6, assign43860_e56637_d_n7, assign43860_e56637_d_n8, assign43860_e56637_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1216 == 0.0)) {
        let assign43860_e56635: f64 = (0.5 * var_temp__blk949);
        (assign43860_e56635, (0.5 * var_temp__blk949_dn4), (0.5 * var_temp__blk949_dn6), (0.5 * var_temp__blk949_dn7), (0.5 * var_temp__blk949_dn8), (0.5 * var_temp__blk949_dn9),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign43860_e56637;
        var_temp2_dn4 = assign43860_e56637_d_n4;
        var_temp2_dn6 = assign43860_e56637_d_n6;
        var_temp2_dn7 = assign43860_e56637_d_n7;
        var_temp2_dn8 = assign43860_e56637_d_n8;
        var_temp2_dn9 = assign43860_e56637_d_n9;

        let (assign43870_e56645, assign43870_e56645_d_n4, assign43870_e56645_d_n6, assign43870_e56645_d_n7, assign43870_e56645_d_n8, assign43870_e56645_d_n9,) = {
    if ((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) {
        let assign43870_e56643: f64 = (var_temp2 * var_asat);
        (assign43870_e56643, ((var_temp2_dn4 * var_asat) + (var_temp2 * var_asat_dn4)), ((var_temp2_dn6 * var_asat) + (var_temp2 * var_asat_dn6)), ((var_temp2_dn7 * var_asat) + (var_temp2 * var_asat_dn7)), ((var_temp2_dn8 * var_asat) + (var_temp2 * var_asat_dn8)), ((var_temp2_dn9 * var_asat) + (var_temp2 * var_asat_dn9)),)
    } else {
        (var_x_inf0, var_x_inf0_dn4, var_x_inf0_dn6, var_x_inf0_dn7, var_x_inf0_dn8, var_x_inf0_dn9,)
    }
};
        var_x_inf0 = assign43870_e56645;
        var_x_inf0_dn4 = assign43870_e56645_d_n4;
        var_x_inf0_dn6 = assign43870_e56645_d_n6;
        var_x_inf0_dn7 = assign43870_e56645_d_n7;
        var_x_inf0_dn8 = assign43870_e56645_d_n8;
        var_x_inf0_dn9 = assign43870_e56645_d_n9;

        let assign43880_e56652: f64 = if ((var_cs_t > 0.0) && (var_thecs_t > 0.0)) { 1.0 } else { 0.0 };
        var_guard1218 = assign43880_e56652;

        let (assign43890_e56664, assign43890_e56664_d_n4, assign43890_e56664_d_n6, assign43890_e56664_d_n7, assign43890_e56664_d_n8, assign43890_e56664_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 != 0.0)) {
        let assign43890_e56660: f64 = (0.475 * var_phit1);
        let assign43890_e56662: f64 = (assign43890_e56660 * var_x_inf0);
        (assign43890_e56662, (((0.475 * var_phit1_dn4) * var_x_inf0) + (assign43890_e56660 * var_x_inf0_dn4)), (((0.475 * var_phit1_dn6) * var_x_inf0) + (assign43890_e56660 * var_x_inf0_dn6)), (((0.475 * var_phit1_dn7) * var_x_inf0) + (assign43890_e56660 * var_x_inf0_dn7)), (((0.475 * var_phit1_dn8) * var_x_inf0) + (assign43890_e56660 * var_x_inf0_dn8)), (((0.475 * var_phit1_dn9) * var_x_inf0) + (assign43890_e56660 * var_x_inf0_dn9)),)
    } else {
        (var_midphi0, var_midphi0_dn4, var_midphi0_dn6, var_midphi0_dn7, var_midphi0_dn8, var_midphi0_dn9,)
    }
};
        var_midphi0 = assign43890_e56664;
        var_midphi0_dn4 = assign43890_e56664_d_n4;
        var_midphi0_dn6 = assign43890_e56664_d_n6;
        var_midphi0_dn7 = assign43890_e56664_d_n7;
        var_midphi0_dn8 = assign43890_e56664_d_n8;
        var_midphi0_dn9 = assign43890_e56664_d_n9;

        *var_alpha_slot = var_alpha;
        *var_alpha_dn4_slot = var_alpha_dn4;
        *var_alpha_dn6_slot = var_alpha_dn6;
        *var_alpha_dn7_slot = var_alpha_dn7;
        *var_alpha_dn8_slot = var_alpha_dn8;
        *var_alpha_dn9_slot = var_alpha_dn9;
        *var_alphas_dc_slot = var_alphas_dc;
        *var_alphas_dc_dn4_slot = var_alphas_dc_dn4;
        *var_alphas_dc_dn6_slot = var_alphas_dc_dn6;
        *var_alphas_dc_dn7_slot = var_alphas_dc_dn7;
        *var_alphas_dc_dn8_slot = var_alphas_dc_dn8;
        *var_alphas_dc_dn9_slot = var_alphas_dc_dn9;
        *var_asat_slot = var_asat;
        *var_asat_dn4_slot = var_asat_dn4;
        *var_asat_dn6_slot = var_asat_dn6;
        *var_asat_dn7_slot = var_asat_dn7;
        *var_asat_dn8_slot = var_asat_dn8;
        *var_asat_dn9_slot = var_asat_dn9;
        *var_dd_slot = var_dd;
        *var_dd_dn4_slot = var_dd_dn4;
        *var_dd_dn6_slot = var_dd_dn6;
        *var_dd_dn7_slot = var_dd_dn7;
        *var_dd_dn8_slot = var_dd_dn8;
        *var_dd_dn9_slot = var_dd_dn9;
        *var_delta_1s_dc_slot = var_delta_1s_dc;
        *var_delta_1s_dc_dn4_slot = var_delta_1s_dc_dn4;
        *var_delta_1s_dc_dn6_slot = var_delta_1s_dc_dn6;
        *var_delta_1s_dc_dn7_slot = var_delta_1s_dc_dn7;
        *var_delta_1s_dc_dn8_slot = var_delta_1s_dc_dn8;
        *var_delta_1s_dc_dn9_slot = var_delta_1s_dc_dn9;
        *var_delta_ns_dc_slot = var_delta_ns_dc;
        *var_delta_ns_dc_dn4_slot = var_delta_ns_dc_dn4;
        *var_delta_ns_dc_dn6_slot = var_delta_ns_dc_dn6;
        *var_delta_ns_dc_dn7_slot = var_delta_ns_dc_dn7;
        *var_delta_ns_dc_dn8_slot = var_delta_ns_dc_dn8;
        *var_delta_ns_dc_dn9_slot = var_delta_ns_dc_dn9;
        *var_dm_slot = var_dm;
        *var_dm_dn4_slot = var_dm_dn4;
        *var_dm_dn6_slot = var_dm_dn6;
        *var_dm_dn7_slot = var_dm_dn7;
        *var_dm_dn8_slot = var_dm_dn8;
        *var_dm_dn9_slot = var_dm_dn9;
        *var_dps_slot = var_dps;
        *var_dps_dn4_slot = var_dps_dn4;
        *var_dps_dn6_slot = var_dps_dn6;
        *var_dps_dn7_slot = var_dps_dn7;
        *var_dps_dn8_slot = var_dps_dn8;
        *var_dps_dn9_slot = var_dps_dn9;
        *var_ds_dc_slot = var_ds_dc;
        *var_ds_dc_dn4_slot = var_ds_dc_dn4;
        *var_ds_dc_dn6_slot = var_ds_dc_dn6;
        *var_ds_dc_dn7_slot = var_ds_dc_dn7;
        *var_ds_dc_dn8_slot = var_ds_dc_dn8;
        *var_ds_dc_dn9_slot = var_ds_dc_dn9;
        *var_ed_slot = var_ed;
        *var_ed_dn4_slot = var_ed_dn4;
        *var_ed_dn6_slot = var_ed_dn6;
        *var_ed_dn7_slot = var_ed_dn7;
        *var_ed_dn8_slot = var_ed_dn8;
        *var_ed_dn9_slot = var_ed_dn9;
        *var_em_slot = var_em;
        *var_em_dn4_slot = var_em_dn4;
        *var_em_dn6_slot = var_em_dn6;
        *var_em_dn7_slot = var_em_dn7;
        *var_em_dn8_slot = var_em_dn8;
        *var_em_dn9_slot = var_em_dn9;
        *var_es_dc_slot = var_es_dc;
        *var_es_dc_dn4_slot = var_es_dc_dn4;
        *var_es_dc_dn6_slot = var_es_dc_dn6;
        *var_es_dc_dn7_slot = var_es_dc_dn7;
        *var_es_dc_dn8_slot = var_es_dc_dn8;
        *var_es_dc_dn9_slot = var_es_dc_dn9;
        *var_eta_p_slot = var_eta_p;
        *var_eta_p_dn4_slot = var_eta_p_dn4;
        *var_eta_p_dn6_slot = var_eta_p_dn6;
        *var_eta_p_dn7_slot = var_eta_p_dn7;
        *var_eta_p_dn8_slot = var_eta_p_dn8;
        *var_eta_p_dn9_slot = var_eta_p_dn9;
        *var_factheta_dc_slot = var_factheta_dc;
        *var_factheta_dc_dn4_slot = var_factheta_dc_dn4;
        *var_factheta_dc_dn6_slot = var_factheta_dc_dn6;
        *var_factheta_dc_dn7_slot = var_factheta_dc_dn7;
        *var_factheta_dc_dn8_slot = var_factheta_dc_dn8;
        *var_factheta_dc_dn9_slot = var_factheta_dc_dn9;
        *var_gmob_slot = var_gmob;
        *var_gmob_dn4_slot = var_gmob_dn4;
        *var_gmob_dn6_slot = var_gmob_dn6;
        *var_gmob_dn7_slot = var_gmob_dn7;
        *var_gmob_dn8_slot = var_gmob_dn8;
        *var_gmob_dn9_slot = var_gmob_dn9;
        *var_gmobs_dc_slot = var_gmobs_dc;
        *var_gmobs_dc_dn4_slot = var_gmobs_dc_dn4;
        *var_gmobs_dc_dn6_slot = var_gmobs_dc_dn6;
        *var_gmobs_dc_dn7_slot = var_gmobs_dc_dn7;
        *var_gmobs_dc_dn8_slot = var_gmobs_dc_dn8;
        *var_gmobs_dc_dn9_slot = var_gmobs_dc_dn9;
        *var_guard1214_slot = var_guard1214;
        *var_guard1215_slot = var_guard1215;
        *var_guard1216_slot = var_guard1216;
        *var_guard1217_slot = var_guard1217;
        *var_guard1218_slot = var_guard1218;
        *var_midphi0_slot = var_midphi0;
        *var_midphi0_dn4_slot = var_midphi0_dn4;
        *var_midphi0_dn6_slot = var_midphi0_dn6;
        *var_midphi0_dn7_slot = var_midphi0_dn7;
        *var_midphi0_dn8_slot = var_midphi0_dn8;
        *var_midphi0_dn9_slot = var_midphi0_dn9;
        *var_pd_slot = var_pd;
        *var_pd_dn4_slot = var_pd_dn4;
        *var_pd_dn6_slot = var_pd_dn6;
        *var_pd_dn7_slot = var_pd_dn7;
        *var_pd_dn8_slot = var_pd_dn8;
        *var_pd_dn9_slot = var_pd_dn9;
        *var_pm_slot = var_pm;
        *var_pm_dn4_slot = var_pm_dn4;
        *var_pm_dn6_slot = var_pm_dn6;
        *var_pm_dn7_slot = var_pm_dn7;
        *var_pm_dn8_slot = var_pm_dn8;
        *var_pm_dn9_slot = var_pm_dn9;
        *var_ps_dc_slot = var_ps_dc;
        *var_ps_dc_dn4_slot = var_ps_dc_dn4;
        *var_ps_dc_dn6_slot = var_ps_dc_dn6;
        *var_ps_dc_dn7_slot = var_ps_dc_dn7;
        *var_ps_dc_dn8_slot = var_ps_dc_dn8;
        *var_ps_dc_dn9_slot = var_ps_dc_dn9;
        *var_qbd_slot = var_qbd;
        *var_qbd_dn4_slot = var_qbd_dn4;
        *var_qbd_dn6_slot = var_qbd_dn6;
        *var_qbd_dn7_slot = var_qbd_dn7;
        *var_qbd_dn8_slot = var_qbd_dn8;
        *var_qbd_dn9_slot = var_qbd_dn9;
        *var_qbm_slot = var_qbm;
        *var_qbm_dn4_slot = var_qbm_dn4;
        *var_qbm_dn6_slot = var_qbm_dn6;
        *var_qbm_dn7_slot = var_qbm_dn7;
        *var_qbm_dn8_slot = var_qbm_dn8;
        *var_qbm_dn9_slot = var_qbm_dn9;
        *var_qbs_dc_slot = var_qbs_dc;
        *var_qbs_dc_dn4_slot = var_qbs_dc_dn4;
        *var_qbs_dc_dn6_slot = var_qbs_dc_dn6;
        *var_qbs_dc_dn7_slot = var_qbs_dc_dn7;
        *var_qbs_dc_dn8_slot = var_qbs_dc_dn8;
        *var_qbs_dc_dn9_slot = var_qbs_dc_dn9;
        *var_qeff1_slot = var_qeff1;
        *var_qeff1_dn4_slot = var_qeff1_dn4;
        *var_qeff1_dn6_slot = var_qeff1_dn6;
        *var_qeff1_dn7_slot = var_qeff1_dn7;
        *var_qeff1_dn8_slot = var_qeff1_dn8;
        *var_qeff1_dn9_slot = var_qeff1_dn9;
        *var_qim_slot = var_qim;
        *var_qim1_slot = var_qim1;
        *var_qim1_dn4_slot = var_qim1_dn4;
        *var_qim1_dn6_slot = var_qim1_dn6;
        *var_qim1_dn7_slot = var_qim1_dn7;
        *var_qim1_dn8_slot = var_qim1_dn8;
        *var_qim1_dn9_slot = var_qim1_dn9;
        *var_qim_dn4_slot = var_qim_dn4;
        *var_qim_dn6_slot = var_qim_dn6;
        *var_qim_dn7_slot = var_qim_dn7;
        *var_qim_dn8_slot = var_qim_dn8;
        *var_qim_dn9_slot = var_qim_dn9;
        *var_qis_dc_slot = var_qis_dc;
        *var_qis_dc_dn4_slot = var_qis_dc_dn4;
        *var_qis_dc_dn6_slot = var_qis_dc_dn6;
        *var_qis_dc_dn7_slot = var_qis_dc_dn7;
        *var_qis_dc_dn8_slot = var_qis_dc_dn8;
        *var_qis_dc_dn9_slot = var_qis_dc_dn9;
        *var_rhob_dc_slot = var_rhob_dc;
        *var_rhob_dc_dn4_slot = var_rhob_dc_dn4;
        *var_rhob_dc_dn6_slot = var_rhob_dc_dn6;
        *var_rhob_dc_dn7_slot = var_rhob_dc_dn7;
        *var_rhob_dc_dn8_slot = var_rhob_dc_dn8;
        *var_rhob_dc_dn9_slot = var_rhob_dc_dn9;
        *var_rhog_dc_slot = var_rhog_dc;
        *var_rhog_dc_dn4_slot = var_rhog_dc_dn4;
        *var_rhog_dc_dn6_slot = var_rhog_dc_dn6;
        *var_rhog_dc_dn7_slot = var_rhog_dc_dn7;
        *var_rhog_dc_dn8_slot = var_rhog_dc_dn8;
        *var_rhog_dc_dn9_slot = var_rhog_dc_dn9;
        *var_rxcor_dc_slot = var_rxcor_dc;
        *var_rxcor_dc_dn4_slot = var_rxcor_dc_dn4;
        *var_rxcor_dc_dn6_slot = var_rxcor_dc_dn6;
        *var_rxcor_dc_dn7_slot = var_rxcor_dc_dn7;
        *var_rxcor_dc_dn8_slot = var_rxcor_dc_dn8;
        *var_rxcor_dc_dn9_slot = var_rxcor_dc_dn9;
        *var_s1_slot = var_s1;
        *var_s1_dn4_slot = var_s1_dn4;
        *var_s1_dn6_slot = var_s1_dn6;
        *var_s1_dn7_slot = var_s1_dn7;
        *var_s1_dn8_slot = var_s1_dn8;
        *var_s1_dn9_slot = var_s1_dn9;
        *var_sqm_slot = var_sqm;
        *var_sqm_dn4_slot = var_sqm_dn4;
        *var_sqm_dn6_slot = var_sqm_dn6;
        *var_sqm_dn7_slot = var_sqm_dn7;
        *var_sqm_dn8_slot = var_sqm_dn8;
        *var_sqm_dn9_slot = var_sqm_dn9;
        *var_sqs_dc_slot = var_sqs_dc;
        *var_sqs_dc_dn4_slot = var_sqs_dc_dn4;
        *var_sqs_dc_dn6_slot = var_sqs_dc_dn6;
        *var_sqs_dc_dn7_slot = var_sqs_dc_dn7;
        *var_sqs_dc_dn8_slot = var_sqs_dc_dn8;
        *var_sqs_dc_dn9_slot = var_sqs_dc_dn9;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp__blk949_slot = var_temp__blk949;
        *var_temp__blk949_dn4_slot = var_temp__blk949_dn4;
        *var_temp__blk949_dn6_slot = var_temp__blk949_dn6;
        *var_temp__blk949_dn7_slot = var_temp__blk949_dn7;
        *var_temp__blk949_dn8_slot = var_temp__blk949_dn8;
        *var_temp__blk949_dn9_slot = var_temp__blk949_dn9;
        *var_thesat1_slot = var_thesat1;
        *var_thesat1_dn4_slot = var_thesat1_dn4;
        *var_thesat1_dn6_slot = var_thesat1_dn6;
        *var_thesat1_dn7_slot = var_thesat1_dn7;
        *var_thesat1_dn8_slot = var_thesat1_dn8;
        *var_thesat1_dn9_slot = var_thesat1_dn9;
        *var_thesateff_slot = var_thesateff;
        *var_thesateff_dn4_slot = var_thesateff_dn4;
        *var_thesateff_dn6_slot = var_thesateff_dn6;
        *var_thesateff_dn7_slot = var_thesateff_dn7;
        *var_thesateff_dn8_slot = var_thesateff_dn8;
        *var_thesateff_dn9_slot = var_thesateff_dn9;
        *var_udse_slot = var_udse;
        *var_udse_dn4_slot = var_udse_dn4;
        *var_udse_dn6_slot = var_udse_dn6;
        *var_udse_dn7_slot = var_udse_dn7;
        *var_udse_dn8_slot = var_udse_dn8;
        *var_udse_dn9_slot = var_udse_dn9;
        *var_v_dsat_slot = var_v_dsat;
        *var_v_dsat_dn4_slot = var_v_dsat_dn4;
        *var_v_dsat_dn6_slot = var_v_dsat_dn6;
        *var_v_dsat_dn7_slot = var_v_dsat_dn7;
        *var_v_dsat_dn8_slot = var_v_dsat_dn8;
        *var_v_dsat_dn9_slot = var_v_dsat_dn9;
        *var_vdsat_lim_slot = var_vdsat_lim;
        *var_vdsat_lim_dn4_slot = var_vdsat_lim_dn4;
        *var_vdsat_lim_dn6_slot = var_vdsat_lim_dn6;
        *var_vdsat_lim_dn7_slot = var_vdsat_lim_dn7;
        *var_vdsat_lim_dn8_slot = var_vdsat_lim_dn8;
        *var_vdsat_lim_dn9_slot = var_vdsat_lim_dn9;
        *var_vdse_slot = var_vdse;
        *var_vdse_dn4_slot = var_vdse_dn4;
        *var_vdse_dn6_slot = var_vdse_dn6;
        *var_vdse_dn7_slot = var_vdse_dn7;
        *var_vdse_dn8_slot = var_vdse_dn8;
        *var_vdse_dn9_slot = var_vdse_dn9;
        *var_voxm_slot = var_voxm;
        *var_voxm_dn4_slot = var_voxm_dn4;
        *var_voxm_dn6_slot = var_voxm_dn6;
        *var_voxm_dn7_slot = var_voxm_dn7;
        *var_voxm_dn8_slot = var_voxm_dn8;
        *var_voxm_dn9_slot = var_voxm_dn9;
        *var_x_d_slot = var_x_d;
        *var_x_d_dn4_slot = var_x_d_dn4;
        *var_x_d_dn6_slot = var_x_d_dn6;
        *var_x_d_dn7_slot = var_x_d_dn7;
        *var_x_d_dn8_slot = var_x_d_dn8;
        *var_x_d_dn9_slot = var_x_d_dn9;
        *var_x_ds_slot = var_x_ds;
        *var_x_ds_dn4_slot = var_x_ds_dn4;
        *var_x_ds_dn6_slot = var_x_ds_dn6;
        *var_x_ds_dn7_slot = var_x_ds_dn7;
        *var_x_ds_dn8_slot = var_x_ds_dn8;
        *var_x_ds_dn9_slot = var_x_ds_dn9;
        *var_x_inf0_slot = var_x_inf0;
        *var_x_inf0_dn4_slot = var_x_inf0_dn4;
        *var_x_inf0_dn6_slot = var_x_inf0_dn6;
        *var_x_inf0_dn7_slot = var_x_inf0_dn7;
        *var_x_inf0_dn8_slot = var_x_inf0_dn8;
        *var_x_inf0_dn9_slot = var_x_inf0_dn9;
        *var_x_m_slot = var_x_m;
        *var_x_m_dn4_slot = var_x_m_dn4;
        *var_x_m_dn6_slot = var_x_m_dn6;
        *var_x_m_dn7_slot = var_x_m_dn7;
        *var_x_m_dn8_slot = var_x_m_dn8;
        *var_x_m_dn9_slot = var_x_m_dn9;
        *var_x_s_dc_slot = var_x_s_dc;
        *var_x_s_dc_dn4_slot = var_x_s_dc_dn4;
        *var_x_s_dc_dn6_slot = var_x_s_dc_dn6;
        *var_x_s_dc_dn7_slot = var_x_s_dc_dn7;
        *var_x_s_dc_dn8_slot = var_x_s_dc_dn8;
        *var_x_s_dc_dn9_slot = var_x_s_dc_dn9;
        *var_xgm_slot = var_xgm;
        *var_xgm_dn4_slot = var_xgm_dn4;
        *var_xgm_dn6_slot = var_xgm_dn6;
        *var_xgm_dn7_slot = var_xgm_dn7;
        *var_xgm_dn8_slot = var_xgm_dn8;
        *var_xgm_dn9_slot = var_xgm_dn9;
        *var_xgs_dc_slot = var_xgs_dc;
        *var_xgs_dc_dn4_slot = var_xgs_dc_dn4;
        *var_xgs_dc_dn6_slot = var_xgs_dc_dn6;
        *var_xgs_dc_dn7_slot = var_xgs_dc_dn7;
        *var_xgs_dc_dn8_slot = var_xgs_dc_dn8;
        *var_xgs_dc_dn9_slot = var_xgs_dc_dn9;
        *var_xi1s_dc_slot = var_xi1s_dc;
        *var_xi1s_dc_dn4_slot = var_xi1s_dc_dn4;
        *var_xi1s_dc_dn6_slot = var_xi1s_dc_dn6;
        *var_xi1s_dc_dn7_slot = var_xi1s_dc_dn7;
        *var_xi1s_dc_dn8_slot = var_xi1s_dc_dn8;
        *var_xi1s_dc_dn9_slot = var_xi1s_dc_dn9;
        *var_xi2s_dc_slot = var_xi2s_dc;
        *var_xi2s_dc_dn4_slot = var_xi2s_dc_dn4;
        *var_xi2s_dc_dn6_slot = var_xi2s_dc_dn6;
        *var_xi2s_dc_dn7_slot = var_xi2s_dc_dn7;
        *var_xi2s_dc_dn8_slot = var_xi2s_dc_dn8;
        *var_xi2s_dc_dn9_slot = var_xi2s_dc_dn9;
        *var_xitsb_dc_slot = var_xitsb_dc;
        *var_xitsb_dc_dn4_slot = var_xitsb_dc_dn4;
        *var_xitsb_dc_dn6_slot = var_xitsb_dc_dn6;
        *var_xitsb_dc_dn7_slot = var_xitsb_dc_dn7;
        *var_xitsb_dc_dn8_slot = var_xitsb_dc_dn8;
        *var_xitsb_dc_dn9_slot = var_xitsb_dc_dn9;
    }

    pub(super) fn stamp_transient_block_95(
        var_alphas: f64,
        var_alphas_dn4: f64,
        var_alphas_dn6: f64,
        var_alphas_dn7: f64,
        var_alphas_dn8: f64,
        var_alphas_dn9: f64,
        var_arloc: f64,
        var_asat: f64,
        var_asat_dn4: f64,
        var_asat_dn6: f64,
        var_asat_dn7: f64,
        var_asat_dn8: f64,
        var_asat_dn9: f64,
        var_chnl_type: f64,
        var_cs_t: f64,
        var_cs_t_dn4: f64,
        var_ds: f64,
        var_ds_dn4: f64,
        var_ds_dn6: f64,
        var_ds_dn7: f64,
        var_ds_dn8: f64,
        var_ds_dn9: f64,
        var_e_eff0: f64,
        var_eta_mu: f64,
        var_gf2: f64,
        var_gf2_dn4: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf2_dn9: f64,
        var_guard1214: f64,
        var_guard1215: f64,
        var_guard1218: f64,
        var_inv_gf2: f64,
        var_inv_gf2_dn4: f64,
        var_inv_gf2_dn6: f64,
        var_inv_gf2_dn7: f64,
        var_inv_gf2_dn8: f64,
        var_inv_gf2_dn9: f64,
        var_midphi0: f64,
        var_midphi0_dn4: f64,
        var_midphi0_dn6: f64,
        var_midphi0_dn7: f64,
        var_midphi0_dn8: f64,
        var_midphi0_dn9: f64,
        var_mue_t: f64,
        var_mue_t_dn4: f64,
        var_phit1: f64,
        var_phit1_dn4: f64,
        var_phit1_dn6: f64,
        var_phit1_dn7: f64,
        var_phit1_dn8: f64,
        var_phit1_dn9: f64,
        var_qis: f64,
        var_qis_dn4: f64,
        var_qis_dn6: f64,
        var_qis_dn7: f64,
        var_qis_dn8: f64,
        var_qis_dn9: f64,
        var_rhob: f64,
        var_rhob_dn4: f64,
        var_rhob_dn6: f64,
        var_rhob_dn7: f64,
        var_rhob_dn8: f64,
        var_rhob_dn9: f64,
        var_rhog: f64,
        var_rhog_dn4: f64,
        var_rhog_dn6: f64,
        var_rhog_dn7: f64,
        var_rhog_dn8: f64,
        var_rhog_dn9: f64,
        var_thecs_t: f64,
        var_thecs_t_dn4: f64,
        var_themu_t: f64,
        var_themu_t_dn4: f64,
        var_ther_i: f64,
        var_ther_i_dn4: f64,
        var_thesat1: f64,
        var_thesat1_dn4: f64,
        var_thesat1_dn6: f64,
        var_thesat1_dn7: f64,
        var_thesat1_dn8: f64,
        var_thesat1_dn9: f64,
        var_vdsat_lim: f64,
        var_vdsat_lim_dn4: f64,
        var_vdsat_lim_dn6: f64,
        var_vdsat_lim_dn7: f64,
        var_vdsat_lim_dn8: f64,
        var_vdsat_lim_dn9: f64,
        var_x_inf0: f64,
        var_x_inf0_dn4: f64,
        var_x_inf0_dn6: f64,
        var_x_inf0_dn7: f64,
        var_x_inf0_dn8: f64,
        var_x_inf0_dn9: f64,
        var_xgs: f64,
        var_xgs_dn4: f64,
        var_xgs_dn6: f64,
        var_xgs_dn7: f64,
        var_xgs_dn8: f64,
        var_xgs_dn9: f64,
        var_alphasat_slot: &mut f64,
        var_alphasat_dn4_slot: &mut f64,
        var_alphasat_dn6_slot: &mut f64,
        var_alphasat_dn7_slot: &mut f64,
        var_alphasat_dn8_slot: &mut f64,
        var_alphasat_dn9_slot: &mut f64,
        var_delta_gmob_slot: &mut f64,
        var_delta_gmob_dn4_slot: &mut f64,
        var_delta_gmob_dn6_slot: &mut f64,
        var_delta_gmob_dn7_slot: &mut f64,
        var_delta_gmob_dn8_slot: &mut f64,
        var_delta_gmob_dn9_slot: &mut f64,
        var_gmobcssat_slot: &mut f64,
        var_gmobcssat_dn4_slot: &mut f64,
        var_gmobcssat_dn6_slot: &mut f64,
        var_gmobcssat_dn7_slot: &mut f64,
        var_gmobcssat_dn8_slot: &mut f64,
        var_gmobcssat_dn9_slot: &mut f64,
        var_gmobmusat_slot: &mut f64,
        var_gmobmusat_dn4_slot: &mut f64,
        var_gmobmusat_dn6_slot: &mut f64,
        var_gmobmusat_dn7_slot: &mut f64,
        var_gmobmusat_dn8_slot: &mut f64,
        var_gmobmusat_dn9_slot: &mut f64,
        var_grsat_slot: &mut f64,
        var_grsat_dn4_slot: &mut f64,
        var_grsat_dn6_slot: &mut f64,
        var_grsat_dn7_slot: &mut f64,
        var_grsat_dn8_slot: &mut f64,
        var_grsat_dn9_slot: &mut f64,
        var_guard1219_slot: &mut f64,
        var_guard1220_slot: &mut f64,
        var_qbsat_slot: &mut f64,
        var_qbsat_dn4_slot: &mut f64,
        var_qbsat_dn6_slot: &mut f64,
        var_qbsat_dn7_slot: &mut f64,
        var_qbsat_dn8_slot: &mut f64,
        var_qbsat_dn9_slot: &mut f64,
        var_qisat_slot: &mut f64,
        var_qisat_dn4_slot: &mut f64,
        var_qisat_dn6_slot: &mut f64,
        var_qisat_dn7_slot: &mut f64,
        var_qisat_dn8_slot: &mut f64,
        var_qisat_dn9_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp__blk949_slot: &mut f64,
        var_temp__blk949_dn4_slot: &mut f64,
        var_temp__blk949_dn6_slot: &mut f64,
        var_temp__blk949_dn7_slot: &mut f64,
        var_temp__blk949_dn8_slot: &mut f64,
        var_temp__blk949_dn9_slot: &mut f64,
        var_v_dsat_slot: &mut f64,
        var_v_dsat_dn4_slot: &mut f64,
        var_v_dsat_dn6_slot: &mut f64,
        var_v_dsat_dn7_slot: &mut f64,
        var_v_dsat_dn8_slot: &mut f64,
        var_v_dsat_dn9_slot: &mut f64,
        var_x_0_slot: &mut f64,
        var_x_0_dn4_slot: &mut f64,
        var_x_0_dn6_slot: &mut f64,
        var_x_0_dn7_slot: &mut f64,
        var_x_0_dn8_slot: &mut f64,
        var_x_0_dn9_slot: &mut f64,
        var_x_inf_slot: &mut f64,
        var_x_inf_dn4_slot: &mut f64,
        var_x_inf_dn6_slot: &mut f64,
        var_x_inf_dn7_slot: &mut f64,
        var_x_inf_dn8_slot: &mut f64,
        var_x_inf_dn9_slot: &mut f64,
        var_x_sat_slot: &mut f64,
        var_x_sat_dn4_slot: &mut f64,
        var_x_sat_dn6_slot: &mut f64,
        var_x_sat_dn7_slot: &mut f64,
        var_x_sat_dn8_slot: &mut f64,
        var_x_sat_dn9_slot: &mut f64,
        var_ysat_slot: &mut f64,
        var_ysat_dn4_slot: &mut f64,
        var_ysat_dn6_slot: &mut f64,
        var_ysat_dn7_slot: &mut f64,
        var_ysat_dn8_slot: &mut f64,
        var_ysat_dn9_slot: &mut f64,
        var_za_slot: &mut f64,
        var_za_dn4_slot: &mut f64,
        var_za_dn6_slot: &mut f64,
        var_za_dn7_slot: &mut f64,
        var_za_dn8_slot: &mut f64,
        var_za_dn9_slot: &mut f64,
    ) {
        let mut var_alphasat: f64 = *var_alphasat_slot;
        let mut var_alphasat_dn4: f64 = *var_alphasat_dn4_slot;
        let mut var_alphasat_dn6: f64 = *var_alphasat_dn6_slot;
        let mut var_alphasat_dn7: f64 = *var_alphasat_dn7_slot;
        let mut var_alphasat_dn8: f64 = *var_alphasat_dn8_slot;
        let mut var_alphasat_dn9: f64 = *var_alphasat_dn9_slot;
        let mut var_delta_gmob: f64 = *var_delta_gmob_slot;
        let mut var_delta_gmob_dn4: f64 = *var_delta_gmob_dn4_slot;
        let mut var_delta_gmob_dn6: f64 = *var_delta_gmob_dn6_slot;
        let mut var_delta_gmob_dn7: f64 = *var_delta_gmob_dn7_slot;
        let mut var_delta_gmob_dn8: f64 = *var_delta_gmob_dn8_slot;
        let mut var_delta_gmob_dn9: f64 = *var_delta_gmob_dn9_slot;
        let mut var_gmobcssat: f64 = *var_gmobcssat_slot;
        let mut var_gmobcssat_dn4: f64 = *var_gmobcssat_dn4_slot;
        let mut var_gmobcssat_dn6: f64 = *var_gmobcssat_dn6_slot;
        let mut var_gmobcssat_dn7: f64 = *var_gmobcssat_dn7_slot;
        let mut var_gmobcssat_dn8: f64 = *var_gmobcssat_dn8_slot;
        let mut var_gmobcssat_dn9: f64 = *var_gmobcssat_dn9_slot;
        let mut var_gmobmusat: f64 = *var_gmobmusat_slot;
        let mut var_gmobmusat_dn4: f64 = *var_gmobmusat_dn4_slot;
        let mut var_gmobmusat_dn6: f64 = *var_gmobmusat_dn6_slot;
        let mut var_gmobmusat_dn7: f64 = *var_gmobmusat_dn7_slot;
        let mut var_gmobmusat_dn8: f64 = *var_gmobmusat_dn8_slot;
        let mut var_gmobmusat_dn9: f64 = *var_gmobmusat_dn9_slot;
        let mut var_grsat: f64 = *var_grsat_slot;
        let mut var_grsat_dn4: f64 = *var_grsat_dn4_slot;
        let mut var_grsat_dn6: f64 = *var_grsat_dn6_slot;
        let mut var_grsat_dn7: f64 = *var_grsat_dn7_slot;
        let mut var_grsat_dn8: f64 = *var_grsat_dn8_slot;
        let mut var_grsat_dn9: f64 = *var_grsat_dn9_slot;
        let mut var_guard1219: f64 = *var_guard1219_slot;
        let mut var_guard1220: f64 = *var_guard1220_slot;
        let mut var_qbsat: f64 = *var_qbsat_slot;
        let mut var_qbsat_dn4: f64 = *var_qbsat_dn4_slot;
        let mut var_qbsat_dn6: f64 = *var_qbsat_dn6_slot;
        let mut var_qbsat_dn7: f64 = *var_qbsat_dn7_slot;
        let mut var_qbsat_dn8: f64 = *var_qbsat_dn8_slot;
        let mut var_qbsat_dn9: f64 = *var_qbsat_dn9_slot;
        let mut var_qisat: f64 = *var_qisat_slot;
        let mut var_qisat_dn4: f64 = *var_qisat_dn4_slot;
        let mut var_qisat_dn6: f64 = *var_qisat_dn6_slot;
        let mut var_qisat_dn7: f64 = *var_qisat_dn7_slot;
        let mut var_qisat_dn8: f64 = *var_qisat_dn8_slot;
        let mut var_qisat_dn9: f64 = *var_qisat_dn9_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp__blk949: f64 = *var_temp__blk949_slot;
        let mut var_temp__blk949_dn4: f64 = *var_temp__blk949_dn4_slot;
        let mut var_temp__blk949_dn6: f64 = *var_temp__blk949_dn6_slot;
        let mut var_temp__blk949_dn7: f64 = *var_temp__blk949_dn7_slot;
        let mut var_temp__blk949_dn8: f64 = *var_temp__blk949_dn8_slot;
        let mut var_temp__blk949_dn9: f64 = *var_temp__blk949_dn9_slot;
        let mut var_v_dsat: f64 = *var_v_dsat_slot;
        let mut var_v_dsat_dn4: f64 = *var_v_dsat_dn4_slot;
        let mut var_v_dsat_dn6: f64 = *var_v_dsat_dn6_slot;
        let mut var_v_dsat_dn7: f64 = *var_v_dsat_dn7_slot;
        let mut var_v_dsat_dn8: f64 = *var_v_dsat_dn8_slot;
        let mut var_v_dsat_dn9: f64 = *var_v_dsat_dn9_slot;
        let mut var_x_0: f64 = *var_x_0_slot;
        let mut var_x_0_dn4: f64 = *var_x_0_dn4_slot;
        let mut var_x_0_dn6: f64 = *var_x_0_dn6_slot;
        let mut var_x_0_dn7: f64 = *var_x_0_dn7_slot;
        let mut var_x_0_dn8: f64 = *var_x_0_dn8_slot;
        let mut var_x_0_dn9: f64 = *var_x_0_dn9_slot;
        let mut var_x_inf: f64 = *var_x_inf_slot;
        let mut var_x_inf_dn4: f64 = *var_x_inf_dn4_slot;
        let mut var_x_inf_dn6: f64 = *var_x_inf_dn6_slot;
        let mut var_x_inf_dn7: f64 = *var_x_inf_dn7_slot;
        let mut var_x_inf_dn8: f64 = *var_x_inf_dn8_slot;
        let mut var_x_inf_dn9: f64 = *var_x_inf_dn9_slot;
        let mut var_x_sat: f64 = *var_x_sat_slot;
        let mut var_x_sat_dn4: f64 = *var_x_sat_dn4_slot;
        let mut var_x_sat_dn6: f64 = *var_x_sat_dn6_slot;
        let mut var_x_sat_dn7: f64 = *var_x_sat_dn7_slot;
        let mut var_x_sat_dn8: f64 = *var_x_sat_dn8_slot;
        let mut var_x_sat_dn9: f64 = *var_x_sat_dn9_slot;
        let mut var_ysat: f64 = *var_ysat_slot;
        let mut var_ysat_dn4: f64 = *var_ysat_dn4_slot;
        let mut var_ysat_dn6: f64 = *var_ysat_dn6_slot;
        let mut var_ysat_dn7: f64 = *var_ysat_dn7_slot;
        let mut var_ysat_dn8: f64 = *var_ysat_dn8_slot;
        let mut var_ysat_dn9: f64 = *var_ysat_dn9_slot;
        let mut var_za: f64 = *var_za_slot;
        let mut var_za_dn4: f64 = *var_za_dn4_slot;
        let mut var_za_dn6: f64 = *var_za_dn6_slot;
        let mut var_za_dn7: f64 = *var_za_dn7_slot;
        let mut var_za_dn8: f64 = *var_za_dn8_slot;
        let mut var_za_dn9: f64 = *var_za_dn9_slot;

        let (assign43900_e56676, assign43900_e56676_d_n4, assign43900_e56676_d_n6, assign43900_e56676_d_n7, assign43900_e56676_d_n8, assign43900_e56676_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 != 0.0)) {
        let assign43900_e56673: f64 = (var_alphas * var_midphi0);
        let assign43900_e56674: f64 = (var_qis - assign43900_e56673);
        (assign43900_e56674, (var_qis_dn4 - ((var_alphas_dn4 * var_midphi0) + (var_alphas * var_midphi0_dn4))), (var_qis_dn6 - ((var_alphas_dn6 * var_midphi0) + (var_alphas * var_midphi0_dn6))), (var_qis_dn7 - ((var_alphas_dn7 * var_midphi0) + (var_alphas * var_midphi0_dn7))), (var_qis_dn8 - ((var_alphas_dn8 * var_midphi0) + (var_alphas * var_midphi0_dn8))), (var_qis_dn9 - ((var_alphas_dn9 * var_midphi0) + (var_alphas * var_midphi0_dn9))),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign43900_e56676;
        var_temp__blk949_dn4 = assign43900_e56676_d_n4;
        var_temp__blk949_dn6 = assign43900_e56676_d_n6;
        var_temp__blk949_dn7 = assign43900_e56676_d_n7;
        var_temp__blk949_dn8 = assign43900_e56676_d_n8;
        var_temp__blk949_dn9 = assign43900_e56676_d_n9;

        let (assign43910_e56693, assign43910_e56693_d_n4, assign43910_e56693_d_n6, assign43910_e56693_d_n7, assign43910_e56693_d_n8, assign43910_e56693_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 != 0.0)) {
        let assign43910_e56686: f64 = (var_temp__blk949 * var_temp__blk949);
        let assign43910_e56688: f64 = (assign43910_e56686 + 1e-12);
        let assign43910_e56689: f64 = (assign43910_e56688).sqrt();
        let assign43910_e56690: f64 = (var_temp__blk949 + assign43910_e56689);
        let assign43910_e56691: f64 = (0.5 * assign43910_e56690);
        (assign43910_e56691, (0.5 * (var_temp__blk949_dn4 + (((var_temp__blk949_dn4 * var_temp__blk949) + (var_temp__blk949 * var_temp__blk949_dn4)) / (2.0 * assign43910_e56689)))), (0.5 * (var_temp__blk949_dn6 + (((var_temp__blk949_dn6 * var_temp__blk949) + (var_temp__blk949 * var_temp__blk949_dn6)) / (2.0 * assign43910_e56689)))), (0.5 * (var_temp__blk949_dn7 + (((var_temp__blk949_dn7 * var_temp__blk949) + (var_temp__blk949 * var_temp__blk949_dn7)) / (2.0 * assign43910_e56689)))), (0.5 * (var_temp__blk949_dn8 + (((var_temp__blk949_dn8 * var_temp__blk949) + (var_temp__blk949 * var_temp__blk949_dn8)) / (2.0 * assign43910_e56689)))), (0.5 * (var_temp__blk949_dn9 + (((var_temp__blk949_dn9 * var_temp__blk949) + (var_temp__blk949 * var_temp__blk949_dn9)) / (2.0 * assign43910_e56689)))),)
    } else {
        (var_qisat, var_qisat_dn4, var_qisat_dn6, var_qisat_dn7, var_qisat_dn8, var_qisat_dn9,)
    }
};
        var_qisat = assign43910_e56693;
        var_qisat_dn4 = assign43910_e56693_d_n4;
        var_qisat_dn6 = assign43910_e56693_d_n6;
        var_qisat_dn7 = assign43910_e56693_d_n7;
        var_qisat_dn8 = assign43910_e56693_d_n8;
        var_qisat_dn9 = assign43910_e56693_d_n9;

        let (assign43920_e56711, assign43920_e56711_d_n4, assign43920_e56711_d_n6, assign43920_e56711_d_n7, assign43920_e56711_d_n8, assign43920_e56711_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 != 0.0)) {
        let assign43920_e56701: f64 = (var_phit1 * var_xgs);
        let assign43920_e56703: f64 = (assign43920_e56701 - var_qis);
        let assign43920_e56706: f64 = (var_alphas - 1.0);
        let assign43920_e56708: f64 = (assign43920_e56706 * var_midphi0);
        let assign43920_e56709: f64 = (assign43920_e56703 + assign43920_e56708);
        (assign43920_e56709, ((((var_phit1_dn4 * var_xgs) + (var_phit1 * var_xgs_dn4)) - var_qis_dn4) + ((var_alphas_dn4 * var_midphi0) + (assign43920_e56706 * var_midphi0_dn4))), ((((var_phit1_dn6 * var_xgs) + (var_phit1 * var_xgs_dn6)) - var_qis_dn6) + ((var_alphas_dn6 * var_midphi0) + (assign43920_e56706 * var_midphi0_dn6))), ((((var_phit1_dn7 * var_xgs) + (var_phit1 * var_xgs_dn7)) - var_qis_dn7) + ((var_alphas_dn7 * var_midphi0) + (assign43920_e56706 * var_midphi0_dn7))), ((((var_phit1_dn8 * var_xgs) + (var_phit1 * var_xgs_dn8)) - var_qis_dn8) + ((var_alphas_dn8 * var_midphi0) + (assign43920_e56706 * var_midphi0_dn8))), ((((var_phit1_dn9 * var_xgs) + (var_phit1 * var_xgs_dn9)) - var_qis_dn9) + ((var_alphas_dn9 * var_midphi0) + (assign43920_e56706 * var_midphi0_dn9))),)
    } else {
        (var_qbsat, var_qbsat_dn4, var_qbsat_dn6, var_qbsat_dn7, var_qbsat_dn8, var_qbsat_dn9,)
    }
};
        var_qbsat = assign43920_e56711;
        var_qbsat_dn4 = assign43920_e56711_d_n4;
        var_qbsat_dn6 = assign43920_e56711_d_n6;
        var_qbsat_dn7 = assign43920_e56711_d_n7;
        var_qbsat_dn8 = assign43920_e56711_d_n8;
        var_qbsat_dn9 = assign43920_e56711_d_n9;

        let (assign43930_e56727, assign43930_e56727_d_n4, assign43930_e56727_d_n6, assign43930_e56727_d_n7, assign43930_e56727_d_n8, assign43930_e56727_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 != 0.0)) {
        let assign43930_e56720: f64 = (0.5 * var_gf2);
        let assign43930_e56722: f64 = (assign43930_e56720 * var_phit1);
        let assign43930_e56724: f64 = (assign43930_e56722 / var_qbsat);
        let assign43930_e56725: f64 = (1.0 + assign43930_e56724);
        (assign43930_e56725, ((((((0.5 * var_gf2_dn4) * var_phit1) + (assign43930_e56720 * var_phit1_dn4)) * var_qbsat) - (assign43930_e56722 * var_qbsat_dn4)) / (var_qbsat * var_qbsat)), ((((((0.5 * var_gf2_dn6) * var_phit1) + (assign43930_e56720 * var_phit1_dn6)) * var_qbsat) - (assign43930_e56722 * var_qbsat_dn6)) / (var_qbsat * var_qbsat)), ((((((0.5 * var_gf2_dn7) * var_phit1) + (assign43930_e56720 * var_phit1_dn7)) * var_qbsat) - (assign43930_e56722 * var_qbsat_dn7)) / (var_qbsat * var_qbsat)), ((((((0.5 * var_gf2_dn8) * var_phit1) + (assign43930_e56720 * var_phit1_dn8)) * var_qbsat) - (assign43930_e56722 * var_qbsat_dn8)) / (var_qbsat * var_qbsat)), ((((((0.5 * var_gf2_dn9) * var_phit1) + (assign43930_e56720 * var_phit1_dn9)) * var_qbsat) - (assign43930_e56722 * var_qbsat_dn9)) / (var_qbsat * var_qbsat)),)
    } else {
        (var_alphasat, var_alphasat_dn4, var_alphasat_dn6, var_alphasat_dn7, var_alphasat_dn8, var_alphasat_dn9,)
    }
};
        var_alphasat = assign43930_e56727;
        var_alphasat_dn4 = assign43930_e56727_d_n4;
        var_alphasat_dn6 = assign43930_e56727_d_n6;
        var_alphasat_dn7 = assign43930_e56727_d_n7;
        var_alphasat_dn8 = assign43930_e56727_d_n8;
        var_alphasat_dn9 = assign43930_e56727_d_n9;

        let (assign43940_e56739, assign43940_e56739_d_n4, assign43940_e56739_d_n6, assign43940_e56739_d_n7, assign43940_e56739_d_n8, assign43940_e56739_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 != 0.0)) {
        let assign43940_e56736: f64 = (var_eta_mu * var_qisat);
        let assign43940_e56737: f64 = (var_qbsat + assign43940_e56736);
        (assign43940_e56737, (var_qbsat_dn4 + (var_eta_mu * var_qisat_dn4)), (var_qbsat_dn6 + (var_eta_mu * var_qisat_dn6)), (var_qbsat_dn7 + (var_eta_mu * var_qisat_dn7)), (var_qbsat_dn8 + (var_eta_mu * var_qisat_dn8)), (var_qbsat_dn9 + (var_eta_mu * var_qisat_dn9)),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign43940_e56739;
        var_temp__blk949_dn4 = assign43940_e56739_d_n4;
        var_temp__blk949_dn6 = assign43940_e56739_d_n6;
        var_temp__blk949_dn7 = assign43940_e56739_d_n7;
        var_temp__blk949_dn8 = assign43940_e56739_d_n8;
        var_temp__blk949_dn9 = assign43940_e56739_d_n9;

        let (assign43950_e56753, assign43950_e56753_d_n4, assign43950_e56753_d_n6, assign43950_e56753_d_n7, assign43950_e56753_d_n8, assign43950_e56753_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 != 0.0)) {
        let assign43950_e56747: f64 = (var_e_eff0 * var_temp__blk949);
        let assign43950_e56749: f64 = (assign43950_e56747 * var_mue_t);
        let assign43950_e56751: f64 = (assign43950_e56749).powf(var_themu_t);
        (assign43950_e56751, if var_themu_t_dn4 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43950_e56749).powf(var_themu_t - 1.0) * (((var_e_eff0 * var_temp__blk949_dn4) * var_mue_t) + (assign43950_e56747 * var_mue_t_dn4)))) } } else { (assign43950_e56751 * ((var_themu_t_dn4 * (assign43950_e56749).ln()) + (var_themu_t * ((((var_e_eff0 * var_temp__blk949_dn4) * var_mue_t) + (assign43950_e56747 * var_mue_t_dn4)) / assign43950_e56749)))) }, if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43950_e56749).powf(var_themu_t - 1.0) * ((var_e_eff0 * var_temp__blk949_dn6) * var_mue_t))) } } else { (assign43950_e56751 * (var_themu_t * (((var_e_eff0 * var_temp__blk949_dn6) * var_mue_t) / assign43950_e56749))) }, if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43950_e56749).powf(var_themu_t - 1.0) * ((var_e_eff0 * var_temp__blk949_dn7) * var_mue_t))) } } else { (assign43950_e56751 * (var_themu_t * (((var_e_eff0 * var_temp__blk949_dn7) * var_mue_t) / assign43950_e56749))) }, if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43950_e56749).powf(var_themu_t - 1.0) * ((var_e_eff0 * var_temp__blk949_dn8) * var_mue_t))) } } else { (assign43950_e56751 * (var_themu_t * (((var_e_eff0 * var_temp__blk949_dn8) * var_mue_t) / assign43950_e56749))) }, if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43950_e56749).powf(var_themu_t - 1.0) * ((var_e_eff0 * var_temp__blk949_dn9) * var_mue_t))) } } else { (assign43950_e56751 * (var_themu_t * (((var_e_eff0 * var_temp__blk949_dn9) * var_mue_t) / assign43950_e56749))) },)
    } else {
        (var_gmobmusat, var_gmobmusat_dn4, var_gmobmusat_dn6, var_gmobmusat_dn7, var_gmobmusat_dn8, var_gmobmusat_dn9,)
    }
};
        var_gmobmusat = assign43950_e56753;
        var_gmobmusat_dn4 = assign43950_e56753_d_n4;
        var_gmobmusat_dn6 = assign43950_e56753_d_n6;
        var_gmobmusat_dn7 = assign43950_e56753_d_n7;
        var_gmobmusat_dn8 = assign43950_e56753_d_n8;
        var_gmobmusat_dn9 = assign43950_e56753_d_n9;

        let (assign43960_e56773, assign43960_e56773_d_n4, assign43960_e56773_d_n6, assign43960_e56773_d_n7, assign43960_e56773_d_n8, assign43960_e56773_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 != 0.0)) {
        let assign43960_e56763: f64 = (1.0 - var_eta_mu);
        let assign43960_e56764: f64 = (var_alphasat * assign43960_e56763);
        let assign43960_e56766: f64 = (assign43960_e56764 - 1.0);
        let assign43960_e56767: f64 = (var_themu_t * assign43960_e56766);
        let assign43960_e56769: f64 = (assign43960_e56767 / var_temp__blk949);
        let assign43960_e56771: f64 = (assign43960_e56769 * var_gmobmusat);
        (assign43960_e56771, (((((((var_themu_t_dn4 * assign43960_e56766) + (var_themu_t * (var_alphasat_dn4 * assign43960_e56763))) * var_temp__blk949) - (assign43960_e56767 * var_temp__blk949_dn4)) / (var_temp__blk949 * var_temp__blk949)) * var_gmobmusat) + (assign43960_e56769 * var_gmobmusat_dn4)), ((((((var_themu_t * (var_alphasat_dn6 * assign43960_e56763)) * var_temp__blk949) - (assign43960_e56767 * var_temp__blk949_dn6)) / (var_temp__blk949 * var_temp__blk949)) * var_gmobmusat) + (assign43960_e56769 * var_gmobmusat_dn6)), ((((((var_themu_t * (var_alphasat_dn7 * assign43960_e56763)) * var_temp__blk949) - (assign43960_e56767 * var_temp__blk949_dn7)) / (var_temp__blk949 * var_temp__blk949)) * var_gmobmusat) + (assign43960_e56769 * var_gmobmusat_dn7)), ((((((var_themu_t * (var_alphasat_dn8 * assign43960_e56763)) * var_temp__blk949) - (assign43960_e56767 * var_temp__blk949_dn8)) / (var_temp__blk949 * var_temp__blk949)) * var_gmobmusat) + (assign43960_e56769 * var_gmobmusat_dn8)), ((((((var_themu_t * (var_alphasat_dn9 * assign43960_e56763)) * var_temp__blk949) - (assign43960_e56767 * var_temp__blk949_dn9)) / (var_temp__blk949 * var_temp__blk949)) * var_gmobmusat) + (assign43960_e56769 * var_gmobmusat_dn9)),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign43960_e56773;
        var_temp1_dn4 = assign43960_e56773_d_n4;
        var_temp1_dn6 = assign43960_e56773_d_n6;
        var_temp1_dn7 = assign43960_e56773_d_n7;
        var_temp1_dn8 = assign43960_e56773_d_n8;
        var_temp1_dn9 = assign43960_e56773_d_n9;

        let (assign43970_e56783, assign43970_e56783_d_n4, assign43970_e56783_d_n6, assign43970_e56783_d_n7, assign43970_e56783_d_n8, assign43970_e56783_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 != 0.0)) {
        let assign43970_e56781: f64 = (var_qisat / var_qbsat);
        (assign43970_e56781, (((var_qisat_dn4 * var_qbsat) - (var_qisat * var_qbsat_dn4)) / (var_qbsat * var_qbsat)), (((var_qisat_dn6 * var_qbsat) - (var_qisat * var_qbsat_dn6)) / (var_qbsat * var_qbsat)), (((var_qisat_dn7 * var_qbsat) - (var_qisat * var_qbsat_dn7)) / (var_qbsat * var_qbsat)), (((var_qisat_dn8 * var_qbsat) - (var_qisat * var_qbsat_dn8)) / (var_qbsat * var_qbsat)), (((var_qisat_dn9 * var_qbsat) - (var_qisat * var_qbsat_dn9)) / (var_qbsat * var_qbsat)),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign43970_e56783;
        var_temp__blk949_dn4 = assign43970_e56783_d_n4;
        var_temp__blk949_dn6 = assign43970_e56783_d_n6;
        var_temp__blk949_dn7 = assign43970_e56783_d_n7;
        var_temp__blk949_dn8 = assign43970_e56783_d_n8;
        var_temp__blk949_dn9 = assign43970_e56783_d_n9;

        let (assign43980_e56798, assign43980_e56798_d_n4, assign43980_e56798_d_n6, assign43980_e56798_d_n7, assign43980_e56798_d_n8, assign43980_e56798_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 != 0.0)) {
        let assign43980_e56792: f64 = (1.0 + var_temp__blk949);
        let assign43980_e56794: f64 = (-var_thecs_t);
        let assign43980_e56795: f64 = (assign43980_e56792).powf(assign43980_e56794);
        let assign43980_e56796: f64 = (var_cs_t * assign43980_e56795);
        (assign43980_e56796, ((var_cs_t_dn4 * assign43980_e56795) + (var_cs_t * if (-var_thecs_t_dn4) == 0.0 && ((assign43980_e56794) as f64).is_finite() && ((assign43980_e56794) as f64).fract() == 0.0 { if assign43980_e56794 == 0.0 { 0.0 } else { (assign43980_e56794 * ((assign43980_e56792).powf(assign43980_e56794 - 1.0) * var_temp__blk949_dn4)) } } else { (assign43980_e56795 * (((-var_thecs_t_dn4) * (assign43980_e56792).ln()) + (assign43980_e56794 * (var_temp__blk949_dn4 / assign43980_e56792)))) })), (var_cs_t * if 0.0 == 0.0 && ((assign43980_e56794) as f64).is_finite() && ((assign43980_e56794) as f64).fract() == 0.0 { if assign43980_e56794 == 0.0 { 0.0 } else { (assign43980_e56794 * ((assign43980_e56792).powf(assign43980_e56794 - 1.0) * var_temp__blk949_dn6)) } } else { (assign43980_e56795 * (assign43980_e56794 * (var_temp__blk949_dn6 / assign43980_e56792))) }), (var_cs_t * if 0.0 == 0.0 && ((assign43980_e56794) as f64).is_finite() && ((assign43980_e56794) as f64).fract() == 0.0 { if assign43980_e56794 == 0.0 { 0.0 } else { (assign43980_e56794 * ((assign43980_e56792).powf(assign43980_e56794 - 1.0) * var_temp__blk949_dn7)) } } else { (assign43980_e56795 * (assign43980_e56794 * (var_temp__blk949_dn7 / assign43980_e56792))) }), (var_cs_t * if 0.0 == 0.0 && ((assign43980_e56794) as f64).is_finite() && ((assign43980_e56794) as f64).fract() == 0.0 { if assign43980_e56794 == 0.0 { 0.0 } else { (assign43980_e56794 * ((assign43980_e56792).powf(assign43980_e56794 - 1.0) * var_temp__blk949_dn8)) } } else { (assign43980_e56795 * (assign43980_e56794 * (var_temp__blk949_dn8 / assign43980_e56792))) }), (var_cs_t * if 0.0 == 0.0 && ((assign43980_e56794) as f64).is_finite() && ((assign43980_e56794) as f64).fract() == 0.0 { if assign43980_e56794 == 0.0 { 0.0 } else { (assign43980_e56794 * ((assign43980_e56792).powf(assign43980_e56794 - 1.0) * var_temp__blk949_dn9)) } } else { (assign43980_e56795 * (assign43980_e56794 * (var_temp__blk949_dn9 / assign43980_e56792))) }),)
    } else {
        (var_gmobcssat, var_gmobcssat_dn4, var_gmobcssat_dn6, var_gmobcssat_dn7, var_gmobcssat_dn8, var_gmobcssat_dn9,)
    }
};
        var_gmobcssat = assign43980_e56798;
        var_gmobcssat_dn4 = assign43980_e56798_d_n4;
        var_gmobcssat_dn6 = assign43980_e56798_d_n6;
        var_gmobcssat_dn7 = assign43980_e56798_d_n7;
        var_gmobcssat_dn8 = assign43980_e56798_d_n8;
        var_gmobcssat_dn9 = assign43980_e56798_d_n9;

        let (assign43990_e56820, assign43990_e56820_d_n4, assign43990_e56820_d_n6, assign43990_e56820_d_n7, assign43990_e56820_d_n8, assign43990_e56820_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 != 0.0)) {
        let assign43990_e56807: f64 = (var_alphasat - 1.0);
        let assign43990_e56811: f64 = (var_temp__blk949 + 1.0);
        let assign43990_e56812: f64 = (1.0 / assign43990_e56811);
        let assign43990_e56813: f64 = (assign43990_e56807 + assign43990_e56812);
        let assign43990_e56814: f64 = (var_thecs_t * assign43990_e56813);
        let assign43990_e56816: f64 = (assign43990_e56814 / var_qbsat);
        let assign43990_e56818: f64 = (assign43990_e56816 * var_gmobcssat);
        (assign43990_e56818, (((((((var_thecs_t_dn4 * assign43990_e56813) + (var_thecs_t * (var_alphasat_dn4 + (-(var_temp__blk949_dn4 / (assign43990_e56811 * assign43990_e56811)))))) * var_qbsat) - (assign43990_e56814 * var_qbsat_dn4)) / (var_qbsat * var_qbsat)) * var_gmobcssat) + (assign43990_e56816 * var_gmobcssat_dn4)), ((((((var_thecs_t * (var_alphasat_dn6 + (-(var_temp__blk949_dn6 / (assign43990_e56811 * assign43990_e56811))))) * var_qbsat) - (assign43990_e56814 * var_qbsat_dn6)) / (var_qbsat * var_qbsat)) * var_gmobcssat) + (assign43990_e56816 * var_gmobcssat_dn6)), ((((((var_thecs_t * (var_alphasat_dn7 + (-(var_temp__blk949_dn7 / (assign43990_e56811 * assign43990_e56811))))) * var_qbsat) - (assign43990_e56814 * var_qbsat_dn7)) / (var_qbsat * var_qbsat)) * var_gmobcssat) + (assign43990_e56816 * var_gmobcssat_dn7)), ((((((var_thecs_t * (var_alphasat_dn8 + (-(var_temp__blk949_dn8 / (assign43990_e56811 * assign43990_e56811))))) * var_qbsat) - (assign43990_e56814 * var_qbsat_dn8)) / (var_qbsat * var_qbsat)) * var_gmobcssat) + (assign43990_e56816 * var_gmobcssat_dn8)), ((((((var_thecs_t * (var_alphasat_dn9 + (-(var_temp__blk949_dn9 / (assign43990_e56811 * assign43990_e56811))))) * var_qbsat) - (assign43990_e56814 * var_qbsat_dn9)) / (var_qbsat * var_qbsat)) * var_gmobcssat) + (assign43990_e56816 * var_gmobcssat_dn9)),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign43990_e56820;
        var_temp2_dn4 = assign43990_e56820_d_n4;
        var_temp2_dn6 = assign43990_e56820_d_n6;
        var_temp2_dn7 = assign43990_e56820_d_n7;
        var_temp2_dn8 = assign43990_e56820_d_n8;
        var_temp2_dn9 = assign43990_e56820_d_n9;

        let (assign44000_e56834, assign44000_e56834_d_n4, assign44000_e56834_d_n6, assign44000_e56834_d_n7, assign44000_e56834_d_n8, assign44000_e56834_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 != 0.0)) {
        let assign44000_e56828: f64 = (var_ther_i * var_rhob);
        let assign44000_e56830: f64 = (assign44000_e56828 * var_rhog);
        let assign44000_e56832: f64 = (assign44000_e56830 * var_qisat);
        (assign44000_e56832, ((((((var_ther_i_dn4 * var_rhob) + (var_ther_i * var_rhob_dn4)) * var_rhog) + (assign44000_e56828 * var_rhog_dn4)) * var_qisat) + (assign44000_e56830 * var_qisat_dn4)), (((((var_ther_i * var_rhob_dn6) * var_rhog) + (assign44000_e56828 * var_rhog_dn6)) * var_qisat) + (assign44000_e56830 * var_qisat_dn6)), (((((var_ther_i * var_rhob_dn7) * var_rhog) + (assign44000_e56828 * var_rhog_dn7)) * var_qisat) + (assign44000_e56830 * var_qisat_dn7)), (((((var_ther_i * var_rhob_dn8) * var_rhog) + (assign44000_e56828 * var_rhog_dn8)) * var_qisat) + (assign44000_e56830 * var_qisat_dn8)), (((((var_ther_i * var_rhob_dn9) * var_rhog) + (assign44000_e56828 * var_rhog_dn9)) * var_qisat) + (assign44000_e56830 * var_qisat_dn9)),)
    } else {
        (var_grsat, var_grsat_dn4, var_grsat_dn6, var_grsat_dn7, var_grsat_dn8, var_grsat_dn9,)
    }
};
        var_grsat = assign44000_e56834;
        var_grsat_dn4 = assign44000_e56834_d_n4;
        var_grsat_dn6 = assign44000_e56834_d_n6;
        var_grsat_dn7 = assign44000_e56834_d_n7;
        var_grsat_dn8 = assign44000_e56834_d_n8;
        var_grsat_dn9 = assign44000_e56834_d_n9;

        let (assign44010_e56854, assign44010_e56854_d_n4, assign44010_e56854_d_n6, assign44010_e56854_d_n7, assign44010_e56854_d_n8, assign44010_e56854_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 != 0.0)) {
        let assign44010_e56844: f64 = (var_ther_i * var_rhob);
        let assign44010_e56846: f64 = (assign44010_e56844 * var_rhog);
        let assign44010_e56848: f64 = (assign44010_e56846 * var_alphasat);
        let assign44010_e56849: f64 = (var_temp1 - assign44010_e56848);
        let assign44010_e56851: f64 = (assign44010_e56849 / var_temp2);
        let assign44010_e56852: f64 = (1.0 + assign44010_e56851);
        (assign44010_e56852, ((((var_temp1_dn4 - ((((((var_ther_i_dn4 * var_rhob) + (var_ther_i * var_rhob_dn4)) * var_rhog) + (assign44010_e56844 * var_rhog_dn4)) * var_alphasat) + (assign44010_e56846 * var_alphasat_dn4))) * var_temp2) - (assign44010_e56849 * var_temp2_dn4)) / (var_temp2 * var_temp2)), ((((var_temp1_dn6 - (((((var_ther_i * var_rhob_dn6) * var_rhog) + (assign44010_e56844 * var_rhog_dn6)) * var_alphasat) + (assign44010_e56846 * var_alphasat_dn6))) * var_temp2) - (assign44010_e56849 * var_temp2_dn6)) / (var_temp2 * var_temp2)), ((((var_temp1_dn7 - (((((var_ther_i * var_rhob_dn7) * var_rhog) + (assign44010_e56844 * var_rhog_dn7)) * var_alphasat) + (assign44010_e56846 * var_alphasat_dn7))) * var_temp2) - (assign44010_e56849 * var_temp2_dn7)) / (var_temp2 * var_temp2)), ((((var_temp1_dn8 - (((((var_ther_i * var_rhob_dn8) * var_rhog) + (assign44010_e56844 * var_rhog_dn8)) * var_alphasat) + (assign44010_e56846 * var_alphasat_dn8))) * var_temp2) - (assign44010_e56849 * var_temp2_dn8)) / (var_temp2 * var_temp2)), ((((var_temp1_dn9 - (((((var_ther_i * var_rhob_dn9) * var_rhog) + (assign44010_e56844 * var_rhog_dn9)) * var_alphasat) + (assign44010_e56846 * var_alphasat_dn9))) * var_temp2) - (assign44010_e56849 * var_temp2_dn9)) / (var_temp2 * var_temp2)),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign44010_e56854;
        var_temp__blk949_dn4 = assign44010_e56854_d_n4;
        var_temp__blk949_dn6 = assign44010_e56854_d_n6;
        var_temp__blk949_dn7 = assign44010_e56854_d_n7;
        var_temp__blk949_dn8 = assign44010_e56854_d_n8;
        var_temp__blk949_dn9 = assign44010_e56854_d_n9;

        let assign44020_e56857: f64 = if var_temp__blk949 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1219 = assign44020_e56857;

        let (assign44030_e56875, assign44030_e56875_d_n4, assign44030_e56875_d_n6, assign44030_e56875_d_n7, assign44030_e56875_d_n8, assign44030_e56875_d_n9,) = {
    if ((((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 != 0.0)) && (var_guard1219 != 0.0)) {
        let assign44030_e56869: f64 = (2.0 * var_temp__blk949);
        let assign44030_e56870: f64 = (assign44030_e56869).exp();
        let assign44030_e56871: f64 = (1.0 + assign44030_e56870);
        let assign44030_e56872: f64 = (assign44030_e56871).ln();
        let assign44030_e56873: f64 = (0.5 * assign44030_e56872);
        (assign44030_e56873, (0.5 * ((assign44030_e56870 * (2.0 * var_temp__blk949_dn4)) / assign44030_e56871)), (0.5 * ((assign44030_e56870 * (2.0 * var_temp__blk949_dn6)) / assign44030_e56871)), (0.5 * ((assign44030_e56870 * (2.0 * var_temp__blk949_dn7)) / assign44030_e56871)), (0.5 * ((assign44030_e56870 * (2.0 * var_temp__blk949_dn8)) / assign44030_e56871)), (0.5 * ((assign44030_e56870 * (2.0 * var_temp__blk949_dn9)) / assign44030_e56871)),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign44030_e56875;
        var_temp1_dn4 = assign44030_e56875_d_n4;
        var_temp1_dn6 = assign44030_e56875_d_n6;
        var_temp1_dn7 = assign44030_e56875_d_n7;
        var_temp1_dn8 = assign44030_e56875_d_n8;
        var_temp1_dn9 = assign44030_e56875_d_n9;

        let (assign44040_e56886, assign44040_e56886_d_n4, assign44040_e56886_d_n6, assign44040_e56886_d_n7, assign44040_e56886_d_n8, assign44040_e56886_d_n9,) = {
    if ((((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 != 0.0)) && (var_guard1219 == 0.0)) {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign44040_e56886;
        var_temp1_dn4 = assign44040_e56886_d_n4;
        var_temp1_dn6 = assign44040_e56886_d_n6;
        var_temp1_dn7 = assign44040_e56886_d_n7;
        var_temp1_dn8 = assign44040_e56886_d_n8;
        var_temp1_dn9 = assign44040_e56886_d_n9;

        let (assign44050_e56907, assign44050_e56907_d_n4, assign44050_e56907_d_n6, assign44050_e56907_d_n7, assign44050_e56907_d_n8, assign44050_e56907_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 != 0.0)) {
        let assign44050_e56893: f64 = (-var_midphi0);
        let assign44050_e56895: f64 = (assign44050_e56893 * var_temp2);
        let assign44050_e56897: f64 = (assign44050_e56895 * var_temp1);
        let assign44050_e56900: f64 = (1.0 + var_gmobmusat);
        let assign44050_e56902: f64 = (assign44050_e56900 + var_gmobcssat);
        let assign44050_e56904: f64 = (assign44050_e56902 + var_grsat);
        let assign44050_e56905: f64 = (assign44050_e56897 / assign44050_e56904);
        (assign44050_e56905, ((((((((-var_midphi0_dn4) * var_temp2) + (assign44050_e56893 * var_temp2_dn4)) * var_temp1) + (assign44050_e56895 * var_temp1_dn4)) * assign44050_e56904) - (assign44050_e56897 * ((var_gmobmusat_dn4 + var_gmobcssat_dn4) + var_grsat_dn4))) / (assign44050_e56904 * assign44050_e56904)), ((((((((-var_midphi0_dn6) * var_temp2) + (assign44050_e56893 * var_temp2_dn6)) * var_temp1) + (assign44050_e56895 * var_temp1_dn6)) * assign44050_e56904) - (assign44050_e56897 * ((var_gmobmusat_dn6 + var_gmobcssat_dn6) + var_grsat_dn6))) / (assign44050_e56904 * assign44050_e56904)), ((((((((-var_midphi0_dn7) * var_temp2) + (assign44050_e56893 * var_temp2_dn7)) * var_temp1) + (assign44050_e56895 * var_temp1_dn7)) * assign44050_e56904) - (assign44050_e56897 * ((var_gmobmusat_dn7 + var_gmobcssat_dn7) + var_grsat_dn7))) / (assign44050_e56904 * assign44050_e56904)), ((((((((-var_midphi0_dn8) * var_temp2) + (assign44050_e56893 * var_temp2_dn8)) * var_temp1) + (assign44050_e56895 * var_temp1_dn8)) * assign44050_e56904) - (assign44050_e56897 * ((var_gmobmusat_dn8 + var_gmobcssat_dn8) + var_grsat_dn8))) / (assign44050_e56904 * assign44050_e56904)), ((((((((-var_midphi0_dn9) * var_temp2) + (assign44050_e56893 * var_temp2_dn9)) * var_temp1) + (assign44050_e56895 * var_temp1_dn9)) * assign44050_e56904) - (assign44050_e56897 * ((var_gmobmusat_dn9 + var_gmobcssat_dn9) + var_grsat_dn9))) / (assign44050_e56904 * assign44050_e56904)),)
    } else {
        (var_delta_gmob, var_delta_gmob_dn4, var_delta_gmob_dn6, var_delta_gmob_dn7, var_delta_gmob_dn8, var_delta_gmob_dn9,)
    }
};
        var_delta_gmob = assign44050_e56907;
        var_delta_gmob_dn4 = assign44050_e56907_d_n4;
        var_delta_gmob_dn6 = assign44050_e56907_d_n6;
        var_delta_gmob_dn7 = assign44050_e56907_d_n7;
        var_delta_gmob_dn8 = assign44050_e56907_d_n8;
        var_delta_gmob_dn9 = assign44050_e56907_d_n9;

        let (assign44060_e56928, assign44060_e56928_d_n4, assign44060_e56928_d_n6, assign44060_e56928_d_n7, assign44060_e56928_d_n8, assign44060_e56928_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 != 0.0)) {
        let assign44060_e56920: f64 = (var_delta_gmob * var_delta_gmob);
        let assign44060_e56921: f64 = (1.0 + assign44060_e56920);
        let assign44060_e56922: f64 = (assign44060_e56921).sqrt();
        let assign44060_e56923: f64 = (1.0 + assign44060_e56922);
        let assign44060_e56924: f64 = (var_delta_gmob / assign44060_e56923);
        let assign44060_e56925: f64 = (1.0 + assign44060_e56924);
        let assign44060_e56926: f64 = (var_x_inf0 * assign44060_e56925);
        (assign44060_e56926, ((var_x_inf0_dn4 * assign44060_e56925) + (var_x_inf0 * (((var_delta_gmob_dn4 * assign44060_e56923) - (var_delta_gmob * (((var_delta_gmob_dn4 * var_delta_gmob) + (var_delta_gmob * var_delta_gmob_dn4)) / (2.0 * assign44060_e56922)))) / (assign44060_e56923 * assign44060_e56923)))), ((var_x_inf0_dn6 * assign44060_e56925) + (var_x_inf0 * (((var_delta_gmob_dn6 * assign44060_e56923) - (var_delta_gmob * (((var_delta_gmob_dn6 * var_delta_gmob) + (var_delta_gmob * var_delta_gmob_dn6)) / (2.0 * assign44060_e56922)))) / (assign44060_e56923 * assign44060_e56923)))), ((var_x_inf0_dn7 * assign44060_e56925) + (var_x_inf0 * (((var_delta_gmob_dn7 * assign44060_e56923) - (var_delta_gmob * (((var_delta_gmob_dn7 * var_delta_gmob) + (var_delta_gmob * var_delta_gmob_dn7)) / (2.0 * assign44060_e56922)))) / (assign44060_e56923 * assign44060_e56923)))), ((var_x_inf0_dn8 * assign44060_e56925) + (var_x_inf0 * (((var_delta_gmob_dn8 * assign44060_e56923) - (var_delta_gmob * (((var_delta_gmob_dn8 * var_delta_gmob) + (var_delta_gmob * var_delta_gmob_dn8)) / (2.0 * assign44060_e56922)))) / (assign44060_e56923 * assign44060_e56923)))), ((var_x_inf0_dn9 * assign44060_e56925) + (var_x_inf0 * (((var_delta_gmob_dn9 * assign44060_e56923) - (var_delta_gmob * (((var_delta_gmob_dn9 * var_delta_gmob) + (var_delta_gmob * var_delta_gmob_dn9)) / (2.0 * assign44060_e56922)))) / (assign44060_e56923 * assign44060_e56923)))),)
    } else {
        (var_x_inf, var_x_inf_dn4, var_x_inf_dn6, var_x_inf_dn7, var_x_inf_dn8, var_x_inf_dn9,)
    }
};
        var_x_inf = assign44060_e56928;
        var_x_inf_dn4 = assign44060_e56928_d_n4;
        var_x_inf_dn6 = assign44060_e56928_d_n6;
        var_x_inf_dn7 = assign44060_e56928_d_n7;
        var_x_inf_dn8 = assign44060_e56928_d_n8;
        var_x_inf_dn9 = assign44060_e56928_d_n9;

        let (assign44070_e56937, assign44070_e56937_d_n4, assign44070_e56937_d_n6, assign44070_e56937_d_n7, assign44070_e56937_d_n8, assign44070_e56937_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1218 == 0.0)) {
        (var_x_inf0, var_x_inf0_dn4, var_x_inf0_dn6, var_x_inf0_dn7, var_x_inf0_dn8, var_x_inf0_dn9,)
    } else {
        (var_x_inf, var_x_inf_dn4, var_x_inf_dn6, var_x_inf_dn7, var_x_inf_dn8, var_x_inf_dn9,)
    }
};
        var_x_inf = assign44070_e56937;
        var_x_inf_dn4 = assign44070_e56937_d_n4;
        var_x_inf_dn6 = assign44070_e56937_d_n6;
        var_x_inf_dn7 = assign44070_e56937_d_n7;
        var_x_inf_dn8 = assign44070_e56937_d_n8;
        var_x_inf_dn9 = assign44070_e56937_d_n9;

        let (assign44080_e56949, assign44080_e56949_d_n4, assign44080_e56949_d_n6, assign44080_e56949_d_n7, assign44080_e56949_d_n8, assign44080_e56949_d_n9,) = {
    if ((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) {
        let assign44080_e56943: f64 = (var_phit1 * var_thesat1);
        let assign44080_e56945: f64 = (assign44080_e56943 * var_x_inf);
        let assign44080_e56947: f64 = (assign44080_e56945 * 0.7071067811865475);
        (assign44080_e56947, (((((var_phit1_dn4 * var_thesat1) + (var_phit1 * var_thesat1_dn4)) * var_x_inf) + (assign44080_e56943 * var_x_inf_dn4)) * 0.7071067811865475), (((((var_phit1_dn6 * var_thesat1) + (var_phit1 * var_thesat1_dn6)) * var_x_inf) + (assign44080_e56943 * var_x_inf_dn6)) * 0.7071067811865475), (((((var_phit1_dn7 * var_thesat1) + (var_phit1 * var_thesat1_dn7)) * var_x_inf) + (assign44080_e56943 * var_x_inf_dn7)) * 0.7071067811865475), (((((var_phit1_dn8 * var_thesat1) + (var_phit1 * var_thesat1_dn8)) * var_x_inf) + (assign44080_e56943 * var_x_inf_dn8)) * 0.7071067811865475), (((((var_phit1_dn9 * var_thesat1) + (var_phit1 * var_thesat1_dn9)) * var_x_inf) + (assign44080_e56943 * var_x_inf_dn9)) * 0.7071067811865475),)
    } else {
        (var_ysat, var_ysat_dn4, var_ysat_dn6, var_ysat_dn7, var_ysat_dn8, var_ysat_dn9,)
    }
};
        var_ysat = assign44080_e56949;
        var_ysat_dn4 = assign44080_e56949_d_n4;
        var_ysat_dn6 = assign44080_e56949_d_n6;
        var_ysat_dn7 = assign44080_e56949_d_n7;
        var_ysat_dn8 = assign44080_e56949_d_n8;
        var_ysat_dn9 = assign44080_e56949_d_n9;

        let assign44090_e56952: f64 = (-1.0);
        let assign44090_e56953: f64 = if var_chnl_type == assign44090_e56952 { 1.0 } else { 0.0 };
        var_guard1220 = assign44090_e56953;

        let (assign44100_e56966, assign44100_e56966_d_n4, assign44100_e56966_d_n6, assign44100_e56966_d_n7, assign44100_e56966_d_n8, assign44100_e56966_d_n9,) = {
    if (((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) && (var_guard1220 != 0.0)) {
        let assign44100_e56962: f64 = (1.0 + var_ysat);
        let assign44100_e56963: f64 = (assign44100_e56962).sqrt();
        let assign44100_e56964: f64 = (var_ysat / assign44100_e56963);
        (assign44100_e56964, (((var_ysat_dn4 * assign44100_e56963) - (var_ysat * (var_ysat_dn4 / (2.0 * assign44100_e56963)))) / (assign44100_e56963 * assign44100_e56963)), (((var_ysat_dn6 * assign44100_e56963) - (var_ysat * (var_ysat_dn6 / (2.0 * assign44100_e56963)))) / (assign44100_e56963 * assign44100_e56963)), (((var_ysat_dn7 * assign44100_e56963) - (var_ysat * (var_ysat_dn7 / (2.0 * assign44100_e56963)))) / (assign44100_e56963 * assign44100_e56963)), (((var_ysat_dn8 * assign44100_e56963) - (var_ysat * (var_ysat_dn8 / (2.0 * assign44100_e56963)))) / (assign44100_e56963 * assign44100_e56963)), (((var_ysat_dn9 * assign44100_e56963) - (var_ysat * (var_ysat_dn9 / (2.0 * assign44100_e56963)))) / (assign44100_e56963 * assign44100_e56963)),)
    } else {
        (var_ysat, var_ysat_dn4, var_ysat_dn6, var_ysat_dn7, var_ysat_dn8, var_ysat_dn9,)
    }
};
        var_ysat = assign44100_e56966;
        var_ysat_dn4 = assign44100_e56966_d_n4;
        var_ysat_dn6 = assign44100_e56966_d_n6;
        var_ysat_dn7 = assign44100_e56966_d_n7;
        var_ysat_dn8 = assign44100_e56966_d_n8;
        var_ysat_dn9 = assign44100_e56966_d_n9;

        let (assign44110_e56981, assign44110_e56981_d_n4, assign44110_e56981_d_n6, assign44110_e56981_d_n7, assign44110_e56981_d_n8, assign44110_e56981_d_n9,) = {
    if ((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) {
        let assign44110_e56975: f64 = (4.0 * var_ysat);
        let assign44110_e56976: f64 = (1.0 + assign44110_e56975);
        let assign44110_e56977: f64 = (assign44110_e56976).sqrt();
        let assign44110_e56978: f64 = (1.0 + assign44110_e56977);
        let assign44110_e56979: f64 = (2.0 / assign44110_e56978);
        (assign44110_e56979, (-((2.0 * ((4.0 * var_ysat_dn4) / (2.0 * assign44110_e56977))) / (assign44110_e56978 * assign44110_e56978))), (-((2.0 * ((4.0 * var_ysat_dn6) / (2.0 * assign44110_e56977))) / (assign44110_e56978 * assign44110_e56978))), (-((2.0 * ((4.0 * var_ysat_dn7) / (2.0 * assign44110_e56977))) / (assign44110_e56978 * assign44110_e56978))), (-((2.0 * ((4.0 * var_ysat_dn8) / (2.0 * assign44110_e56977))) / (assign44110_e56978 * assign44110_e56978))), (-((2.0 * ((4.0 * var_ysat_dn9) / (2.0 * assign44110_e56977))) / (assign44110_e56978 * assign44110_e56978))),)
    } else {
        (var_za, var_za_dn4, var_za_dn6, var_za_dn7, var_za_dn8, var_za_dn9,)
    }
};
        var_za = assign44110_e56981;
        var_za_dn4 = assign44110_e56981_d_n4;
        var_za_dn6 = assign44110_e56981_d_n6;
        var_za_dn7 = assign44110_e56981_d_n7;
        var_za_dn8 = assign44110_e56981_d_n8;
        var_za_dn9 = assign44110_e56981_d_n9;

        let (assign44120_e56989, assign44120_e56989_d_n4, assign44120_e56989_d_n6, assign44120_e56989_d_n7, assign44120_e56989_d_n8, assign44120_e56989_d_n9,) = {
    if ((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) {
        let assign44120_e56987: f64 = (var_za * var_ysat);
        (assign44120_e56987, ((var_za_dn4 * var_ysat) + (var_za * var_ysat_dn4)), ((var_za_dn6 * var_ysat) + (var_za * var_ysat_dn6)), ((var_za_dn7 * var_ysat) + (var_za * var_ysat_dn7)), ((var_za_dn8 * var_ysat) + (var_za * var_ysat_dn8)), ((var_za_dn9 * var_ysat) + (var_za * var_ysat_dn9)),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign44120_e56989;
        var_temp__blk949_dn4 = assign44120_e56989_d_n4;
        var_temp__blk949_dn6 = assign44120_e56989_d_n6;
        var_temp__blk949_dn7 = assign44120_e56989_d_n7;
        var_temp__blk949_dn8 = assign44120_e56989_d_n8;
        var_temp__blk949_dn9 = assign44120_e56989_d_n9;

        let (assign44130_e57019, assign44130_e57019_d_n4, assign44130_e57019_d_n6, assign44130_e57019_d_n7, assign44130_e57019_d_n8, assign44130_e57019_d_n9,) = {
    if ((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) {
        let assign44130_e56995: f64 = (var_x_inf * var_za);
        let assign44130_e56999: f64 = (0.86 * var_temp__blk949);
        let assign44130_e57003: f64 = (var_temp__blk949 * var_za);
        let assign44130_e57004: f64 = (1.0 - assign44130_e57003);
        let assign44130_e57005: f64 = (assign44130_e56999 * assign44130_e57004);
        let assign44130_e57009: f64 = (4.0 * var_temp__blk949);
        let assign44130_e57011: f64 = (assign44130_e57009 * var_temp__blk949);
        let assign44130_e57013: f64 = (assign44130_e57011 * var_za);
        let assign44130_e57014: f64 = (1.0 + assign44130_e57013);
        let assign44130_e57015: f64 = (assign44130_e57005 / assign44130_e57014);
        let assign44130_e57016: f64 = (1.0 + assign44130_e57015);
        let assign44130_e57017: f64 = (assign44130_e56995 * assign44130_e57016);
        (assign44130_e57017, ((((var_x_inf_dn4 * var_za) + (var_x_inf * var_za_dn4)) * assign44130_e57016) + (assign44130_e56995 * ((((((0.86 * var_temp__blk949_dn4) * assign44130_e57004) + (assign44130_e56999 * (-((var_temp__blk949_dn4 * var_za) + (var_temp__blk949 * var_za_dn4))))) * assign44130_e57014) - (assign44130_e57005 * (((((4.0 * var_temp__blk949_dn4) * var_temp__blk949) + (assign44130_e57009 * var_temp__blk949_dn4)) * var_za) + (assign44130_e57011 * var_za_dn4)))) / (assign44130_e57014 * assign44130_e57014)))), ((((var_x_inf_dn6 * var_za) + (var_x_inf * var_za_dn6)) * assign44130_e57016) + (assign44130_e56995 * ((((((0.86 * var_temp__blk949_dn6) * assign44130_e57004) + (assign44130_e56999 * (-((var_temp__blk949_dn6 * var_za) + (var_temp__blk949 * var_za_dn6))))) * assign44130_e57014) - (assign44130_e57005 * (((((4.0 * var_temp__blk949_dn6) * var_temp__blk949) + (assign44130_e57009 * var_temp__blk949_dn6)) * var_za) + (assign44130_e57011 * var_za_dn6)))) / (assign44130_e57014 * assign44130_e57014)))), ((((var_x_inf_dn7 * var_za) + (var_x_inf * var_za_dn7)) * assign44130_e57016) + (assign44130_e56995 * ((((((0.86 * var_temp__blk949_dn7) * assign44130_e57004) + (assign44130_e56999 * (-((var_temp__blk949_dn7 * var_za) + (var_temp__blk949 * var_za_dn7))))) * assign44130_e57014) - (assign44130_e57005 * (((((4.0 * var_temp__blk949_dn7) * var_temp__blk949) + (assign44130_e57009 * var_temp__blk949_dn7)) * var_za) + (assign44130_e57011 * var_za_dn7)))) / (assign44130_e57014 * assign44130_e57014)))), ((((var_x_inf_dn8 * var_za) + (var_x_inf * var_za_dn8)) * assign44130_e57016) + (assign44130_e56995 * ((((((0.86 * var_temp__blk949_dn8) * assign44130_e57004) + (assign44130_e56999 * (-((var_temp__blk949_dn8 * var_za) + (var_temp__blk949 * var_za_dn8))))) * assign44130_e57014) - (assign44130_e57005 * (((((4.0 * var_temp__blk949_dn8) * var_temp__blk949) + (assign44130_e57009 * var_temp__blk949_dn8)) * var_za) + (assign44130_e57011 * var_za_dn8)))) / (assign44130_e57014 * assign44130_e57014)))), ((((var_x_inf_dn9 * var_za) + (var_x_inf * var_za_dn9)) * assign44130_e57016) + (assign44130_e56995 * ((((((0.86 * var_temp__blk949_dn9) * assign44130_e57004) + (assign44130_e56999 * (-((var_temp__blk949_dn9 * var_za) + (var_temp__blk949 * var_za_dn9))))) * assign44130_e57014) - (assign44130_e57005 * (((((4.0 * var_temp__blk949_dn9) * var_temp__blk949) + (assign44130_e57009 * var_temp__blk949_dn9)) * var_za) + (assign44130_e57011 * var_za_dn9)))) / (assign44130_e57014 * assign44130_e57014)))),)
    } else {
        (var_x_0, var_x_0_dn4, var_x_0_dn6, var_x_0_dn7, var_x_0_dn8, var_x_0_dn9,)
    }
};
        var_x_0 = assign44130_e57019;
        var_x_0_dn4 = assign44130_e57019_d_n4;
        var_x_0_dn6 = assign44130_e57019_d_n6;
        var_x_0_dn7 = assign44130_e57019_d_n7;
        var_x_0_dn8 = assign44130_e57019_d_n8;
        var_x_0_dn9 = assign44130_e57019_d_n9;

        let (assign44140_e57027, assign44140_e57027_d_n4, assign44140_e57027_d_n6, assign44140_e57027_d_n7, assign44140_e57027_d_n8, assign44140_e57027_d_n9,) = {
    if ((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) {
        let assign44140_e57025: f64 = (0.99 * var_x_0);
        (assign44140_e57025, (0.99 * var_x_0_dn4), (0.99 * var_x_0_dn6), (0.99 * var_x_0_dn7), (0.99 * var_x_0_dn8), (0.99 * var_x_0_dn9),)
    } else {
        (var_x_sat, var_x_sat_dn4, var_x_sat_dn6, var_x_sat_dn7, var_x_sat_dn8, var_x_sat_dn9,)
    }
};
        var_x_sat = assign44140_e57027;
        var_x_sat_dn4 = assign44140_e57027_d_n4;
        var_x_sat_dn6 = assign44140_e57027_d_n6;
        var_x_sat_dn7 = assign44140_e57027_d_n7;
        var_x_sat_dn8 = assign44140_e57027_d_n8;
        var_x_sat_dn9 = assign44140_e57027_d_n9;

        let (assign44150_e57043, assign44150_e57043_d_n4, assign44150_e57043_d_n6, assign44150_e57043_d_n7, assign44150_e57043_d_n8, assign44150_e57043_d_n9,) = {
    if ((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) {
        let assign44150_e57035: f64 = (2.0 * var_asat);
        let assign44150_e57036: f64 = (var_x_sat - assign44150_e57035);
        let assign44150_e57037: f64 = (var_x_sat * assign44150_e57036);
        let assign44150_e57039: f64 = (assign44150_e57037 * var_inv_gf2);
        let assign44150_e57041: f64 = (assign44150_e57039 / var_ds);
        (assign44150_e57041, (((((((var_x_sat_dn4 * assign44150_e57036) + (var_x_sat * (var_x_sat_dn4 - (2.0 * var_asat_dn4)))) * var_inv_gf2) + (assign44150_e57037 * var_inv_gf2_dn4)) * var_ds) - (assign44150_e57039 * var_ds_dn4)) / (var_ds * var_ds)), (((((((var_x_sat_dn6 * assign44150_e57036) + (var_x_sat * (var_x_sat_dn6 - (2.0 * var_asat_dn6)))) * var_inv_gf2) + (assign44150_e57037 * var_inv_gf2_dn6)) * var_ds) - (assign44150_e57039 * var_ds_dn6)) / (var_ds * var_ds)), (((((((var_x_sat_dn7 * assign44150_e57036) + (var_x_sat * (var_x_sat_dn7 - (2.0 * var_asat_dn7)))) * var_inv_gf2) + (assign44150_e57037 * var_inv_gf2_dn7)) * var_ds) - (assign44150_e57039 * var_ds_dn7)) / (var_ds * var_ds)), (((((((var_x_sat_dn8 * assign44150_e57036) + (var_x_sat * (var_x_sat_dn8 - (2.0 * var_asat_dn8)))) * var_inv_gf2) + (assign44150_e57037 * var_inv_gf2_dn8)) * var_ds) - (assign44150_e57039 * var_ds_dn8)) / (var_ds * var_ds)), (((((((var_x_sat_dn9 * assign44150_e57036) + (var_x_sat * (var_x_sat_dn9 - (2.0 * var_asat_dn9)))) * var_inv_gf2) + (assign44150_e57037 * var_inv_gf2_dn9)) * var_ds) - (assign44150_e57039 * var_ds_dn9)) / (var_ds * var_ds)),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign44150_e57043;
        var_temp__blk949_dn4 = assign44150_e57043_d_n4;
        var_temp__blk949_dn6 = assign44150_e57043_d_n6;
        var_temp__blk949_dn7 = assign44150_e57043_d_n7;
        var_temp__blk949_dn8 = assign44150_e57043_d_n8;
        var_temp__blk949_dn9 = assign44150_e57043_d_n9;

        let (assign44160_e57063, assign44160_e57063_d_n4, assign44160_e57063_d_n6, assign44160_e57063_d_n7, assign44160_e57063_d_n8, assign44160_e57063_d_n9,) = {
    if ((var_guard1214 != 0.0) && (var_guard1215 != 0.0)) {
        let assign44160_e57052: f64 = (-0.99);
        let (assign44160_e57057, assign44160_e57057_d_n4, assign44160_e57057_d_n6, assign44160_e57057_d_n7, assign44160_e57057_d_n8, assign44160_e57057_d_n9,) = {
            if (var_temp__blk949 > assign44160_e57052) {
                (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
            } else {
                let assign44160_e57056: f64 = (-0.99);
                (assign44160_e57056, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign44160_e57058: f64 = (1.0 + assign44160_e57057);
        let assign44160_e57059: f64 = (assign44160_e57058).ln();
        let assign44160_e57060: f64 = (var_x_sat - assign44160_e57059);
        let assign44160_e57061: f64 = (var_phit1 * assign44160_e57060);
        (assign44160_e57061, ((var_phit1_dn4 * assign44160_e57060) + (var_phit1 * (var_x_sat_dn4 - (assign44160_e57057_d_n4 / assign44160_e57058)))), ((var_phit1_dn6 * assign44160_e57060) + (var_phit1 * (var_x_sat_dn6 - (assign44160_e57057_d_n6 / assign44160_e57058)))), ((var_phit1_dn7 * assign44160_e57060) + (var_phit1 * (var_x_sat_dn7 - (assign44160_e57057_d_n7 / assign44160_e57058)))), ((var_phit1_dn8 * assign44160_e57060) + (var_phit1 * (var_x_sat_dn8 - (assign44160_e57057_d_n8 / assign44160_e57058)))), ((var_phit1_dn9 * assign44160_e57060) + (var_phit1 * (var_x_sat_dn9 - (assign44160_e57057_d_n9 / assign44160_e57058)))),)
    } else {
        (var_v_dsat, var_v_dsat_dn4, var_v_dsat_dn6, var_v_dsat_dn7, var_v_dsat_dn8, var_v_dsat_dn9,)
    }
};
        var_v_dsat = assign44160_e57063;
        var_v_dsat_dn4 = assign44160_e57063_d_n4;
        var_v_dsat_dn6 = assign44160_e57063_d_n6;
        var_v_dsat_dn7 = assign44160_e57063_d_n7;
        var_v_dsat_dn8 = assign44160_e57063_d_n8;
        var_v_dsat_dn9 = assign44160_e57063_d_n9;

        let (assign44170_e57070, assign44170_e57070_d_n4, assign44170_e57070_d_n6, assign44170_e57070_d_n7, assign44170_e57070_d_n8, assign44170_e57070_d_n9,) = {
    if ((var_guard1214 != 0.0) && (var_guard1215 == 0.0)) {
        (var_vdsat_lim, var_vdsat_lim_dn4, var_vdsat_lim_dn6, var_vdsat_lim_dn7, var_vdsat_lim_dn8, var_vdsat_lim_dn9,)
    } else {
        (var_v_dsat, var_v_dsat_dn4, var_v_dsat_dn6, var_v_dsat_dn7, var_v_dsat_dn8, var_v_dsat_dn9,)
    }
};
        var_v_dsat = assign44170_e57070;
        var_v_dsat_dn4 = assign44170_e57070_d_n4;
        var_v_dsat_dn6 = assign44170_e57070_d_n6;
        var_v_dsat_dn7 = assign44170_e57070_d_n7;
        var_v_dsat_dn8 = assign44170_e57070_d_n8;
        var_v_dsat_dn9 = assign44170_e57070_d_n9;

        let (assign44180_e57076, assign44180_e57076_d_n4, assign44180_e57076_d_n6, assign44180_e57076_d_n7, assign44180_e57076_d_n8, assign44180_e57076_d_n9,) = {
    if (var_guard1214 != 0.0) {
        let assign44180_e57074: f64 = (1.0 + var_arloc);
        (assign44180_e57074, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign44180_e57076;
        var_temp__blk949_dn4 = assign44180_e57076_d_n4;
        var_temp__blk949_dn6 = assign44180_e57076_d_n6;
        var_temp__blk949_dn7 = assign44180_e57076_d_n7;
        var_temp__blk949_dn8 = assign44180_e57076_d_n8;
        var_temp__blk949_dn9 = assign44180_e57076_d_n9;

        *var_alphasat_slot = var_alphasat;
        *var_alphasat_dn4_slot = var_alphasat_dn4;
        *var_alphasat_dn6_slot = var_alphasat_dn6;
        *var_alphasat_dn7_slot = var_alphasat_dn7;
        *var_alphasat_dn8_slot = var_alphasat_dn8;
        *var_alphasat_dn9_slot = var_alphasat_dn9;
        *var_delta_gmob_slot = var_delta_gmob;
        *var_delta_gmob_dn4_slot = var_delta_gmob_dn4;
        *var_delta_gmob_dn6_slot = var_delta_gmob_dn6;
        *var_delta_gmob_dn7_slot = var_delta_gmob_dn7;
        *var_delta_gmob_dn8_slot = var_delta_gmob_dn8;
        *var_delta_gmob_dn9_slot = var_delta_gmob_dn9;
        *var_gmobcssat_slot = var_gmobcssat;
        *var_gmobcssat_dn4_slot = var_gmobcssat_dn4;
        *var_gmobcssat_dn6_slot = var_gmobcssat_dn6;
        *var_gmobcssat_dn7_slot = var_gmobcssat_dn7;
        *var_gmobcssat_dn8_slot = var_gmobcssat_dn8;
        *var_gmobcssat_dn9_slot = var_gmobcssat_dn9;
        *var_gmobmusat_slot = var_gmobmusat;
        *var_gmobmusat_dn4_slot = var_gmobmusat_dn4;
        *var_gmobmusat_dn6_slot = var_gmobmusat_dn6;
        *var_gmobmusat_dn7_slot = var_gmobmusat_dn7;
        *var_gmobmusat_dn8_slot = var_gmobmusat_dn8;
        *var_gmobmusat_dn9_slot = var_gmobmusat_dn9;
        *var_grsat_slot = var_grsat;
        *var_grsat_dn4_slot = var_grsat_dn4;
        *var_grsat_dn6_slot = var_grsat_dn6;
        *var_grsat_dn7_slot = var_grsat_dn7;
        *var_grsat_dn8_slot = var_grsat_dn8;
        *var_grsat_dn9_slot = var_grsat_dn9;
        *var_guard1219_slot = var_guard1219;
        *var_guard1220_slot = var_guard1220;
        *var_qbsat_slot = var_qbsat;
        *var_qbsat_dn4_slot = var_qbsat_dn4;
        *var_qbsat_dn6_slot = var_qbsat_dn6;
        *var_qbsat_dn7_slot = var_qbsat_dn7;
        *var_qbsat_dn8_slot = var_qbsat_dn8;
        *var_qbsat_dn9_slot = var_qbsat_dn9;
        *var_qisat_slot = var_qisat;
        *var_qisat_dn4_slot = var_qisat_dn4;
        *var_qisat_dn6_slot = var_qisat_dn6;
        *var_qisat_dn7_slot = var_qisat_dn7;
        *var_qisat_dn8_slot = var_qisat_dn8;
        *var_qisat_dn9_slot = var_qisat_dn9;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp__blk949_slot = var_temp__blk949;
        *var_temp__blk949_dn4_slot = var_temp__blk949_dn4;
        *var_temp__blk949_dn6_slot = var_temp__blk949_dn6;
        *var_temp__blk949_dn7_slot = var_temp__blk949_dn7;
        *var_temp__blk949_dn8_slot = var_temp__blk949_dn8;
        *var_temp__blk949_dn9_slot = var_temp__blk949_dn9;
        *var_v_dsat_slot = var_v_dsat;
        *var_v_dsat_dn4_slot = var_v_dsat_dn4;
        *var_v_dsat_dn6_slot = var_v_dsat_dn6;
        *var_v_dsat_dn7_slot = var_v_dsat_dn7;
        *var_v_dsat_dn8_slot = var_v_dsat_dn8;
        *var_v_dsat_dn9_slot = var_v_dsat_dn9;
        *var_x_0_slot = var_x_0;
        *var_x_0_dn4_slot = var_x_0_dn4;
        *var_x_0_dn6_slot = var_x_0_dn6;
        *var_x_0_dn7_slot = var_x_0_dn7;
        *var_x_0_dn8_slot = var_x_0_dn8;
        *var_x_0_dn9_slot = var_x_0_dn9;
        *var_x_inf_slot = var_x_inf;
        *var_x_inf_dn4_slot = var_x_inf_dn4;
        *var_x_inf_dn6_slot = var_x_inf_dn6;
        *var_x_inf_dn7_slot = var_x_inf_dn7;
        *var_x_inf_dn8_slot = var_x_inf_dn8;
        *var_x_inf_dn9_slot = var_x_inf_dn9;
        *var_x_sat_slot = var_x_sat;
        *var_x_sat_dn4_slot = var_x_sat_dn4;
        *var_x_sat_dn6_slot = var_x_sat_dn6;
        *var_x_sat_dn7_slot = var_x_sat_dn7;
        *var_x_sat_dn8_slot = var_x_sat_dn8;
        *var_x_sat_dn9_slot = var_x_sat_dn9;
        *var_ysat_slot = var_ysat;
        *var_ysat_dn4_slot = var_ysat_dn4;
        *var_ysat_dn6_slot = var_ysat_dn6;
        *var_ysat_dn7_slot = var_ysat_dn7;
        *var_ysat_dn8_slot = var_ysat_dn8;
        *var_ysat_dn9_slot = var_ysat_dn9;
        *var_za_slot = var_za;
        *var_za_dn4_slot = var_za_dn4;
        *var_za_dn6_slot = var_za_dn6;
        *var_za_dn7_slot = var_za_dn7;
        *var_za_dn8_slot = var_za_dn8;
        *var_za_dn9_slot = var_za_dn9;
    }
}
