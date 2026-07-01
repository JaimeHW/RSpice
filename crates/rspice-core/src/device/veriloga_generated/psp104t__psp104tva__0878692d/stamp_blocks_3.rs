#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_48(
        p: &Parameters,
        var_atatsti: f64,
        var_btatpartsti: f64,
        var_ftdsti: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard487: f64,
        var_idmult: f64,
        var_idsatsti: f64,
        var_one_minus_psti: f64,
        var_one_over_one_minus_psti: f64,
        var_perfc: f64,
        var_two_psistar: f64,
        var_vbirstiinv: f64,
        var_vbisti: f64,
        var_vjsrh: f64,
        var_wdepnulrsti: f64,
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
        var_guard488_slot: &mut f64,
        var_guard489_slot: &mut f64,
        var_guard490_slot: &mut f64,
        var_guard491_slot: &mut f64,
        var_guard492_slot: &mut f64,
        var_guard493_slot: &mut f64,
        var_guard494_slot: &mut f64,
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
        let mut var_guard488: f64 = *var_guard488_slot;
        let mut var_guard489: f64 = *var_guard489_slot;
        let mut var_guard490: f64 = *var_guard490_slot;
        let mut var_guard491: f64 = *var_guard491_slot;
        let mut var_guard492: f64 = *var_guard492_slot;
        let mut var_guard493: f64 = *var_guard493_slot;
        let mut var_guard494: f64 = *var_guard494_slot;
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

        let (assign25030_e29588,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) {
        let assign25030_e29586: f64 = (var_idsatsti * var_idmult);
        (assign25030_e29586,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign25030_e29588;

        let assign25040_e29595: f64 = if ((p.p858 == 0.0) && (p.p863 == 0.0)) { 1.0 } else { 0.0 };
        var_guard488 = assign25040_e29595;

        let (assign25050_e29606, assign25050_e29606_d_n6, assign25050_e29606_d_n7, assign25050_e29606_d_n8, assign25050_e29606_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard488 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign25050_e29606;
        var_isrh_dn6 = assign25050_e29606_d_n6;
        var_isrh_dn7 = assign25050_e29606_d_n7;
        var_isrh_dn8 = assign25050_e29606_d_n8;
        var_isrh_dn9 = assign25050_e29606_d_n9;

        let (assign25060_e29620,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard488 == 0.0)) {
        let assign25060_e29618: f64 = (var_vbisti - var_vjsrh);
        (assign25060_e29618,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign25060_e29620;

        let (assign25070_e29639,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard488 == 0.0)) {
        let assign25070_e29634: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign25070_e29635: f64 = (1.0 - assign25070_e29634);
        let assign25070_e29636: f64 = (assign25070_e29635).sqrt();
        let assign25070_e29637: f64 = (1.0 - assign25070_e29636);
        (assign25070_e29637,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign25070_e29639;

        let assign25080_e29642: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };
        var_guard489 = assign25080_e29642;

        let (assign25090_e29656,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard488 == 0.0)) && (var_guard489 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign25090_e29656;

        let (assign25100_e29688,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard488 == 0.0)) && (var_guard489 == 0.0)) {
        let assign25100_e29671: f64 = (var_wsrhstep * var_wsrhstep);
        let assign25100_e29673: f64 = (var_wsrhstep).ln();
        let assign25100_e29674: f64 = (assign25100_e29671 * assign25100_e29673);
        let assign25100_e29677: f64 = (1.0 - var_wsrhstep);
        let assign25100_e29678: f64 = (assign25100_e29674 / assign25100_e29677);
        let assign25100_e29680: f64 = (assign25100_e29678 + var_wsrhstep);
        let assign25100_e29684: f64 = (2.0 * p.p849);
        let assign25100_e29685: f64 = (1.0 - assign25100_e29684);
        let assign25100_e29686: f64 = (assign25100_e29680 * assign25100_e29685);
        (assign25100_e29686,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign25100_e29688;

        let (assign25110_e29702,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard488 == 0.0)) {
        let assign25110_e29700: f64 = (var_wsrhstep + var_dwsrh);
        (assign25110_e29700,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign25110_e29702;

        let assign25120_e29705: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };
        var_guard490 = assign25120_e29705;

        let (assign25130_e29722, assign25130_e29722_d_n6, assign25130_e29722_d_n7, assign25130_e29722_d_n8, assign25130_e29722_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard488 == 0.0)) && (var_guard490 != 0.0)) {
        let assign25130_e29719: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign25130_e29720: f64 = (assign25130_e29719).sqrt();
        (assign25130_e29720, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign25130_e29722;
        var_tmp_dn6 = assign25130_e29722_d_n6;
        var_tmp_dn7 = assign25130_e29722_d_n7;
        var_tmp_dn8 = assign25130_e29722_d_n8;
        var_tmp_dn9 = assign25130_e29722_d_n9;

        let (assign25140_e29741, assign25140_e29741_d_n6, assign25140_e29741_d_n7, assign25140_e29741_d_n8, assign25140_e29741_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard488 == 0.0)) && (var_guard490 == 0.0)) {
        let assign25140_e29737: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign25140_e29739: f64 = (assign25140_e29737).powf(p.p849);
        (assign25140_e29739, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign25140_e29741;
        var_tmp_dn6 = assign25140_e29741_d_n6;
        var_tmp_dn7 = assign25140_e29741_d_n7;
        var_tmp_dn8 = assign25140_e29741_d_n8;
        var_tmp_dn9 = assign25140_e29741_d_n9;

        let (assign25150_e29755, assign25150_e29755_d_n6, assign25150_e29755_d_n7, assign25150_e29755_d_n8, assign25150_e29755_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard488 == 0.0)) {
        let assign25150_e29753: f64 = (var_wdepnulrsti * var_tmp);
        (assign25150_e29753, (var_wdepnulrsti * var_tmp_dn6), (var_wdepnulrsti * var_tmp_dn7), (var_wdepnulrsti * var_tmp_dn8), (var_wdepnulrsti * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign25150_e29755;
        var_wdep_dn6 = assign25150_e29755_d_n6;
        var_wdep_dn7 = assign25150_e29755_d_n7;
        var_wdep_dn8 = assign25150_e29755_d_n8;
        var_wdep_dn9 = assign25150_e29755_d_n9;

        let (assign25160_e29773, assign25160_e29773_d_n6, assign25160_e29773_d_n7, assign25160_e29773_d_n8, assign25160_e29773_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard488 == 0.0)) {
        let assign25160_e29768: f64 = (var_zinv - 1.0);
        let assign25160_e29770: f64 = (assign25160_e29768 * var_wdep);
        let assign25160_e29771: f64 = (var_ftdsti * assign25160_e29770);
        (assign25160_e29771, (var_ftdsti * (assign25160_e29768 * var_wdep_dn6)), (var_ftdsti * (assign25160_e29768 * var_wdep_dn7)), (var_ftdsti * (assign25160_e29768 * var_wdep_dn8)), (var_ftdsti * (assign25160_e29768 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign25160_e29773;
        var_asrh_dn6 = assign25160_e29773_d_n6;
        var_asrh_dn7 = assign25160_e29773_d_n7;
        var_asrh_dn8 = assign25160_e29773_d_n8;
        var_asrh_dn9 = assign25160_e29773_d_n9;

        let (assign25170_e29789, assign25170_e29789_d_n6, assign25170_e29789_d_n7, assign25170_e29789_d_n8, assign25170_e29789_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard488 == 0.0)) {
        let assign25170_e29786: f64 = (var_asrh * var_wsrh);
        let assign25170_e29787: f64 = (p.p858 * assign25170_e29786);
        (assign25170_e29787, (p.p858 * (var_asrh_dn6 * var_wsrh)), (p.p858 * (var_asrh_dn7 * var_wsrh)), (p.p858 * (var_asrh_dn8 * var_wsrh)), (p.p858 * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign25170_e29789;
        var_isrh_dn6 = assign25170_e29789_d_n6;
        var_isrh_dn7 = assign25170_e29789_d_n7;
        var_isrh_dn8 = assign25170_e29789_d_n8;
        var_isrh_dn9 = assign25170_e29789_d_n9;

        let assign25180_e29792: f64 = if p.p863 == 0.0 { 1.0 } else { 0.0 };
        var_guard491 = assign25180_e29792;

        let (assign25190_e29803, assign25190_e29803_d_n6, assign25190_e29803_d_n7, assign25190_e29803_d_n8, assign25190_e29803_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign25190_e29803;
        var_itat_dn6 = assign25190_e29803_d_n6;
        var_itat_dn7 = assign25190_e29803_d_n7;
        var_itat_dn8 = assign25190_e29803_d_n8;
        var_itat_dn9 = assign25190_e29803_d_n9;

        let (assign25200_e29821, assign25200_e29821_d_n6, assign25200_e29821_d_n7, assign25200_e29821_d_n8, assign25200_e29821_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) {
        let assign25200_e29816: f64 = (var_wdep * var_one_minus_psti);
        let assign25200_e29818: f64 = (assign25200_e29816 / var_vbi_minus_vjsrh);
        let assign25200_e29819: f64 = (var_btatpartsti * assign25200_e29818);
        (assign25200_e29819, (var_btatpartsti * ((var_wdep_dn6 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn7 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn8 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn9 * var_one_minus_psti) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign25200_e29821;
        var_btat_dn6 = assign25200_e29821_d_n6;
        var_btat_dn7 = assign25200_e29821_d_n7;
        var_btat_dn8 = assign25200_e29821_d_n8;
        var_btat_dn9 = assign25200_e29821_d_n9;

        let (assign25210_e29837, assign25210_e29837_d_n6, assign25210_e29837_d_n7, assign25210_e29837_d_n8, assign25210_e29837_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) {
        let assign25210_e29833: f64 = (0.666666666666667 * var_atatsti);
        let assign25210_e29835: f64 = (assign25210_e29833 / var_btat);
        (assign25210_e29835, (-((assign25210_e29833 * var_btat_dn6) / (var_btat * var_btat))), (-((assign25210_e29833 * var_btat_dn7) / (var_btat * var_btat))), (-((assign25210_e29833 * var_btat_dn8) / (var_btat * var_btat))), (-((assign25210_e29833 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign25210_e29837;
        var_twoatatoverthreebtat_dn6 = assign25210_e29837_d_n6;
        var_twoatatoverthreebtat_dn7 = assign25210_e29837_d_n7;
        var_twoatatoverthreebtat_dn8 = assign25210_e29837_d_n8;
        var_twoatatoverthreebtat_dn9 = assign25210_e29837_d_n9;

        let (assign25220_e29851, assign25220_e29851_d_n6, assign25220_e29851_d_n7, assign25220_e29851_d_n8, assign25220_e29851_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) {
        let assign25220_e29849: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign25220_e29849, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign25220_e29851;
        var_umaxbeforelimiting_dn6 = assign25220_e29851_d_n6;
        var_umaxbeforelimiting_dn7 = assign25220_e29851_d_n7;
        var_umaxbeforelimiting_dn8 = assign25220_e29851_d_n8;
        var_umaxbeforelimiting_dn9 = assign25220_e29851_d_n9;

        let (assign25230_e29872, assign25230_e29872_d_n6, assign25230_e29872_d_n7, assign25230_e29872_d_n8, assign25230_e29872_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) {
        let assign25230_e29863: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign25230_e29866: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign25230_e29868: f64 = (assign25230_e29866 + 1.0);
        let assign25230_e29869: f64 = (assign25230_e29863 / assign25230_e29868);
        let assign25230_e29870: f64 = (assign25230_e29869).sqrt();
        (assign25230_e29870, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign25230_e29868) - (assign25230_e29863 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign25230_e29868 * assign25230_e29868)) / (2.0 * assign25230_e29870)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign25230_e29868) - (assign25230_e29863 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign25230_e29868 * assign25230_e29868)) / (2.0 * assign25230_e29870)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign25230_e29868) - (assign25230_e29863 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign25230_e29868 * assign25230_e29868)) / (2.0 * assign25230_e29870)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign25230_e29868) - (assign25230_e29863 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign25230_e29868 * assign25230_e29868)) / (2.0 * assign25230_e29870)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign25230_e29872;
        var_umax_dn6 = assign25230_e29872_d_n6;
        var_umax_dn7 = assign25230_e29872_d_n7;
        var_umax_dn8 = assign25230_e29872_d_n8;
        var_umax_dn9 = assign25230_e29872_d_n9;

        let (assign25240_e29885, assign25240_e29885_d_n6, assign25240_e29885_d_n7, assign25240_e29885_d_n8, assign25240_e29885_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) {
        let assign25240_e29883: f64 = (var_umax).sqrt();
        (assign25240_e29883, (var_umax_dn6 / (2.0 * assign25240_e29883)), (var_umax_dn7 / (2.0 * assign25240_e29883)), (var_umax_dn8 / (2.0 * assign25240_e29883)), (var_umax_dn9 / (2.0 * assign25240_e29883)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign25240_e29885;
        var_sqrtumax_dn6 = assign25240_e29885_d_n6;
        var_sqrtumax_dn7 = assign25240_e29885_d_n7;
        var_sqrtumax_dn8 = assign25240_e29885_d_n8;
        var_sqrtumax_dn9 = assign25240_e29885_d_n9;

        let (assign25250_e29899, assign25250_e29899_d_n6, assign25250_e29899_d_n7, assign25250_e29899_d_n8, assign25250_e29899_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) {
        let assign25250_e29897: f64 = (var_umax * var_sqrtumax);
        (assign25250_e29897, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign25250_e29899;
        var_umaxpoweronepointfive_dn6 = assign25250_e29899_d_n6;
        var_umaxpoweronepointfive_dn7 = assign25250_e29899_d_n7;
        var_umaxpoweronepointfive_dn8 = assign25250_e29899_d_n8;
        var_umaxpoweronepointfive_dn9 = assign25250_e29899_d_n9;

        let assign25260_e29901: f64 = (-p.p849);
        let assign25260_e29903: f64 = (assign25260_e29901 * var_one_over_one_minus_psti);
        let assign25260_e29905: f64 = (-1.0);
        let assign25260_e29906: f64 = if assign25260_e29903 == assign25260_e29905 { 1.0 } else { 0.0 };
        var_guard492 = assign25260_e29906;

        let (assign25270_e29926, assign25270_e29926_d_n6, assign25270_e29926_d_n7, assign25270_e29926_d_n8, assign25270_e29926_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) && (var_guard492 != 0.0)) {
        let assign25270_e29922: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign25270_e29923: f64 = (1.0 + assign25270_e29922);
        let assign25270_e29924: f64 = (1.0 / assign25270_e29923);
        (assign25270_e29924, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign25270_e29923 * assign25270_e29923))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign25270_e29923 * assign25270_e29923))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign25270_e29923 * assign25270_e29923))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign25270_e29923 * assign25270_e29923))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign25270_e29926;
        var_wgamma_dn6 = assign25270_e29926_d_n6;
        var_wgamma_dn7 = assign25270_e29926_d_n7;
        var_wgamma_dn8 = assign25270_e29926_d_n8;
        var_wgamma_dn9 = assign25270_e29926_d_n9;

        let (assign25280_e29950, assign25280_e29950_d_n6, assign25280_e29950_d_n7, assign25280_e29950_d_n8, assign25280_e29950_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) && (var_guard492 == 0.0)) {
        let assign25280_e29942: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign25280_e29943: f64 = (1.0 + assign25280_e29942);
        let assign25280_e29945: f64 = (-p.p849);
        let assign25280_e29947: f64 = (assign25280_e29945 * var_one_over_one_minus_psti);
        let assign25280_e29948: f64 = (assign25280_e29943).powf(assign25280_e29947);
        (assign25280_e29948, if 0.0 == 0.0 && ((assign25280_e29947) as f64).is_finite() && ((assign25280_e29947) as f64).fract() == 0.0 { if assign25280_e29947 == 0.0 { 0.0 } else { (assign25280_e29947 * ((assign25280_e29943).powf(assign25280_e29947 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign25280_e29948 * (assign25280_e29947 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign25280_e29943))) }, if 0.0 == 0.0 && ((assign25280_e29947) as f64).is_finite() && ((assign25280_e29947) as f64).fract() == 0.0 { if assign25280_e29947 == 0.0 { 0.0 } else { (assign25280_e29947 * ((assign25280_e29943).powf(assign25280_e29947 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign25280_e29948 * (assign25280_e29947 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign25280_e29943))) }, if 0.0 == 0.0 && ((assign25280_e29947) as f64).is_finite() && ((assign25280_e29947) as f64).fract() == 0.0 { if assign25280_e29947 == 0.0 { 0.0 } else { (assign25280_e29947 * ((assign25280_e29943).powf(assign25280_e29947 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign25280_e29948 * (assign25280_e29947 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign25280_e29943))) }, if 0.0 == 0.0 && ((assign25280_e29947) as f64).is_finite() && ((assign25280_e29947) as f64).fract() == 0.0 { if assign25280_e29947 == 0.0 { 0.0 } else { (assign25280_e29947 * ((assign25280_e29943).powf(assign25280_e29947 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign25280_e29948 * (assign25280_e29947 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign25280_e29943))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign25280_e29950;
        var_wgamma_dn6 = assign25280_e29950_d_n6;
        var_wgamma_dn7 = assign25280_e29950_d_n7;
        var_wgamma_dn8 = assign25280_e29950_d_n8;
        var_wgamma_dn9 = assign25280_e29950_d_n9;

        let (assign25290_e29968, assign25290_e29968_d_n6, assign25290_e29968_d_n7, assign25290_e29968_d_n8, assign25290_e29968_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) {
        let assign25290_e29962: f64 = (var_wsrh * var_wgamma);
        let assign25290_e29965: f64 = (var_wsrh + var_wgamma);
        let assign25290_e29966: f64 = (assign25290_e29962 / assign25290_e29965);
        (assign25290_e29966, ((((var_wsrh * var_wgamma_dn6) * assign25290_e29965) - (assign25290_e29962 * var_wgamma_dn6)) / (assign25290_e29965 * assign25290_e29965)), ((((var_wsrh * var_wgamma_dn7) * assign25290_e29965) - (assign25290_e29962 * var_wgamma_dn7)) / (assign25290_e29965 * assign25290_e29965)), ((((var_wsrh * var_wgamma_dn8) * assign25290_e29965) - (assign25290_e29962 * var_wgamma_dn8)) / (assign25290_e29965 * assign25290_e29965)), ((((var_wsrh * var_wgamma_dn9) * assign25290_e29965) - (assign25290_e29962 * var_wgamma_dn9)) / (assign25290_e29965 * assign25290_e29965)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign25290_e29968;
        var_wtat_dn6 = assign25290_e29968_d_n6;
        var_wtat_dn7 = assign25290_e29968_d_n7;
        var_wtat_dn8 = assign25290_e29968_d_n8;
        var_wtat_dn9 = assign25290_e29968_d_n9;

        let (assign25300_e29985, assign25300_e29985_d_n6, assign25300_e29985_d_n7, assign25300_e29985_d_n8, assign25300_e29985_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) {
        let assign25300_e29981: f64 = (var_btat / var_sqrtumax);
        let assign25300_e29982: f64 = (0.375 * assign25300_e29981);
        let assign25300_e29983: f64 = (assign25300_e29982).sqrt();
        (assign25300_e29983, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign25300_e29983)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign25300_e29983)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign25300_e29983)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign25300_e29983)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign25300_e29985;
        var_ktat_dn6 = assign25300_e29985_d_n6;
        var_ktat_dn7 = assign25300_e29985_d_n7;
        var_ktat_dn8 = assign25300_e29985_d_n8;
        var_ktat_dn9 = assign25300_e29985_d_n9;

        let (assign25310_e30003, assign25310_e30003_d_n6, assign25310_e30003_d_n7, assign25310_e30003_d_n8, assign25310_e30003_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) {
        let assign25310_e29998: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign25310_e29999: f64 = (2.0 * assign25310_e29998);
        let assign25310_e30001: f64 = (assign25310_e29999 - var_umax);
        (assign25310_e30001, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign25310_e30003;
        var_ltat_dn6 = assign25310_e30003_d_n6;
        var_ltat_dn7 = assign25310_e30003_d_n7;
        var_ltat_dn8 = assign25310_e30003_d_n8;
        var_ltat_dn9 = assign25310_e30003_d_n9;

        let (assign25320_e30029, assign25320_e30029_d_n6, assign25320_e30029_d_n7, assign25320_e30029_d_n8, assign25320_e30029_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) {
        let assign25320_e30015: f64 = (var_atatsti * var_twoatatoverthreebtat);
        let assign25320_e30017: f64 = (assign25320_e30015 * var_sqrtumax);
        let assign25320_e30020: f64 = (var_atatsti * var_umax);
        let assign25320_e30021: f64 = (assign25320_e30017 - assign25320_e30020);
        let assign25320_e30025: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign25320_e30026: f64 = (0.5 * assign25320_e30025);
        let assign25320_e30027: f64 = (assign25320_e30021 + assign25320_e30026);
        (assign25320_e30027, (((((var_atatsti * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign25320_e30015 * var_sqrtumax_dn6)) - (var_atatsti * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign25320_e30015 * var_sqrtumax_dn7)) - (var_atatsti * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign25320_e30015 * var_sqrtumax_dn8)) - (var_atatsti * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatsti * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign25320_e30015 * var_sqrtumax_dn9)) - (var_atatsti * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign25320_e30029;
        var_mtat_dn6 = assign25320_e30029_d_n6;
        var_mtat_dn7 = assign25320_e30029_d_n7;
        var_mtat_dn8 = assign25320_e30029_d_n8;
        var_mtat_dn9 = assign25320_e30029_d_n9;

        let (assign25330_e30045, assign25330_e30045_d_n6, assign25330_e30045_d_n7, assign25330_e30045_d_n8, assign25330_e30045_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) {
        let assign25330_e30041: f64 = (var_ltat - 1.0);
        let assign25330_e30043: f64 = (assign25330_e30041 * var_ktat);
        (assign25330_e30043, ((var_ltat_dn6 * var_ktat) + (assign25330_e30041 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign25330_e30041 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign25330_e30041 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign25330_e30041 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign25330_e30045;
        var_xerfc_dn6 = assign25330_e30045_d_n6;
        var_xerfc_dn7 = assign25330_e30045_d_n7;
        var_xerfc_dn8 = assign25330_e30045_d_n8;
        var_xerfc_dn9 = assign25330_e30045_d_n9;

        let (assign25340_e30059, assign25340_e30059_d_n6, assign25340_e30059_d_n7, assign25340_e30059_d_n8, assign25340_e30059_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) {
        let assign25340_e30057: f64 = (var_xerfc * var_xerfc);
        (assign25340_e30057, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign25340_e30059;
        var_ysq_dn6 = assign25340_e30059_d_n6;
        var_ysq_dn7 = assign25340_e30059_d_n7;
        var_ysq_dn8 = assign25340_e30059_d_n8;
        var_ysq_dn9 = assign25340_e30059_d_n9;

        let assign25350_e30062: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard493 = assign25350_e30062;

        let (assign25360_e30082, assign25360_e30082_d_n6, assign25360_e30082_d_n7, assign25360_e30082_d_n8, assign25360_e30082_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) && (var_guard493 != 0.0)) {
        let assign25360_e30078: f64 = (var_perfc * var_xerfc);
        let assign25360_e30079: f64 = (1.0 + assign25360_e30078);
        let assign25360_e30080: f64 = (1.0 / assign25360_e30079);
        (assign25360_e30080, (-((var_perfc * var_xerfc_dn6) / (assign25360_e30079 * assign25360_e30079))), (-((var_perfc * var_xerfc_dn7) / (assign25360_e30079 * assign25360_e30079))), (-((var_perfc * var_xerfc_dn8) / (assign25360_e30079 * assign25360_e30079))), (-((var_perfc * var_xerfc_dn9) / (assign25360_e30079 * assign25360_e30079))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign25360_e30082;
        var_terfc_dn6 = assign25360_e30082_d_n6;
        var_terfc_dn7 = assign25360_e30082_d_n7;
        var_terfc_dn8 = assign25360_e30082_d_n8;
        var_terfc_dn9 = assign25360_e30082_d_n9;

        let (assign25370_e30103, assign25370_e30103_d_n6, assign25370_e30103_d_n7, assign25370_e30103_d_n8, assign25370_e30103_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) && (var_guard493 == 0.0)) {
        let assign25370_e30099: f64 = (var_perfc * var_xerfc);
        let assign25370_e30100: f64 = (1.0 - assign25370_e30099);
        let assign25370_e30101: f64 = (1.0 / assign25370_e30100);
        (assign25370_e30101, (-((-(var_perfc * var_xerfc_dn6)) / (assign25370_e30100 * assign25370_e30100))), (-((-(var_perfc * var_xerfc_dn7)) / (assign25370_e30100 * assign25370_e30100))), (-((-(var_perfc * var_xerfc_dn8)) / (assign25370_e30100 * assign25370_e30100))), (-((-(var_perfc * var_xerfc_dn9)) / (assign25370_e30100 * assign25370_e30100))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign25370_e30103;
        var_terfc_dn6 = assign25370_e30103_d_n6;
        var_terfc_dn7 = assign25370_e30103_d_n7;
        var_terfc_dn8 = assign25370_e30103_d_n8;
        var_terfc_dn9 = assign25370_e30103_d_n9;

        let assign25380_e30105: f64 = (-var_ysq);
        let assign25380_e30107: f64 = (assign25380_e30105 + var_mtat);
        let assign25380_e30109: f64 = (-230.25850929940458);
        let assign25380_e30110: f64 = if assign25380_e30107 > assign25380_e30109 { 1.0 } else { 0.0 };
        var_guard494 = assign25380_e30110;

        let (assign25390_e30128, assign25390_e30128_d_n6, assign25390_e30128_d_n7, assign25390_e30128_d_n8, assign25390_e30128_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) && (var_guard494 != 0.0)) {
        let assign25390_e30123: f64 = (-var_ysq);
        let assign25390_e30125: f64 = (assign25390_e30123 + var_mtat);
        let assign25390_e30126: f64 = (assign25390_e30125).exp();
        (assign25390_e30126, (assign25390_e30126 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign25390_e30126 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign25390_e30126 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign25390_e30126 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign25390_e30128;
        var_tmp_dn6 = assign25390_e30128_d_n6;
        var_tmp_dn7 = assign25390_e30128_d_n7;
        var_tmp_dn8 = assign25390_e30128_d_n8;
        var_tmp_dn9 = assign25390_e30128_d_n9;

        let (assign25400_e30177, assign25400_e30177_d_n6, assign25400_e30177_d_n7, assign25400_e30177_d_n8, assign25400_e30177_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) && (var_guard494 == 0.0)) {
        let assign25400_e30144: f64 = (-230.25850929940458);
        let assign25400_e30146: f64 = (-var_ysq);
        let assign25400_e30148: f64 = (assign25400_e30146 + var_mtat);
        let assign25400_e30149: f64 = (assign25400_e30144 - assign25400_e30148);
        let assign25400_e30153: f64 = (-230.25850929940458);
        let assign25400_e30155: f64 = (-var_ysq);
        let assign25400_e30157: f64 = (assign25400_e30155 + var_mtat);
        let assign25400_e30158: f64 = (assign25400_e30153 - assign25400_e30157);
        let assign25400_e30161: f64 = (-230.25850929940458);
        let assign25400_e30163: f64 = (-var_ysq);
        let assign25400_e30165: f64 = (assign25400_e30163 + var_mtat);
        let assign25400_e30166: f64 = (assign25400_e30161 - assign25400_e30165);
        let assign25400_e30168: f64 = (assign25400_e30166 * 0.3333333333333333);
        let assign25400_e30169: f64 = (1.0 + assign25400_e30168);
        let assign25400_e30170: f64 = (assign25400_e30158 * assign25400_e30169);
        let assign25400_e30171: f64 = (0.5 * assign25400_e30170);
        let assign25400_e30172: f64 = (1.0 + assign25400_e30171);
        let assign25400_e30173: f64 = (assign25400_e30149 * assign25400_e30172);
        let assign25400_e30174: f64 = (1.0 + assign25400_e30173);
        let assign25400_e30175: f64 = (1e-100 / assign25400_e30174);
        (assign25400_e30175, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign25400_e30172) + (assign25400_e30149 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign25400_e30169) + (assign25400_e30158 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign25400_e30174 * assign25400_e30174))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign25400_e30172) + (assign25400_e30149 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign25400_e30169) + (assign25400_e30158 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign25400_e30174 * assign25400_e30174))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign25400_e30172) + (assign25400_e30149 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign25400_e30169) + (assign25400_e30158 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign25400_e30174 * assign25400_e30174))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign25400_e30172) + (assign25400_e30149 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign25400_e30169) + (assign25400_e30158 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign25400_e30174 * assign25400_e30174))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign25400_e30177;
        var_tmp_dn6 = assign25400_e30177_d_n6;
        var_tmp_dn7 = assign25400_e30177_d_n7;
        var_tmp_dn8 = assign25400_e30177_d_n8;
        var_tmp_dn9 = assign25400_e30177_d_n9;

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
        *var_guard488_slot = var_guard488;
        *var_guard489_slot = var_guard489;
        *var_guard490_slot = var_guard490;
        *var_guard491_slot = var_guard491;
        *var_guard492_slot = var_guard492;
        *var_guard493_slot = var_guard493;
        *var_guard494_slot = var_guard494;
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

    pub(super) fn stamp_transient_block_49(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatsti: f64,
        var_berfc: f64,
        var_cerfc: f64,
        var_fbbtsti: f64,
        var_fstopsti: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard487: f64,
        var_guard491: f64,
        var_idmult: f64,
        var_idsatgat: f64,
        var_ktat: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_ktat_dn9: f64,
        var_lgsource_i: f64,
        var_mtat: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_mtat_dn9: f64,
        var_one_over_one_minus_psti: f64,
        var_slopesti: f64,
        var_terfc: f64,
        var_terfc_dn6: f64,
        var_terfc_dn7: f64,
        var_terfc_dn8: f64,
        var_terfc_dn9: f64,
        var_two_psistar: f64,
        var_v5: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbigat: f64,
        var_vbirstiinv: f64,
        var_vbrinvsti: f64,
        var_vjsrh: f64,
        var_wdepnulrinvsti: f64,
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
        var_guard505_slot: &mut f64,
        var_guard506_slot: &mut f64,
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
        let mut var_guard505: f64 = *var_guard505_slot;
        let mut var_guard506: f64 = *var_guard506_slot;
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

        let (assign25410_e30207, assign25410_e30207_d_n6, assign25410_e30207_d_n7, assign25410_e30207_d_n8, assign25410_e30207_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) {
        let assign25410_e30189: f64 = (0.29214664 * var_terfc);
        let assign25410_e30193: f64 = (var_terfc * var_terfc);
        let assign25410_e30194: f64 = (var_berfc * assign25410_e30193);
        let assign25410_e30195: f64 = (assign25410_e30189 + assign25410_e30194);
        let assign25410_e30199: f64 = (var_terfc * var_terfc);
        let assign25410_e30201: f64 = (assign25410_e30199 * var_terfc);
        let assign25410_e30202: f64 = (var_cerfc * assign25410_e30201);
        let assign25410_e30203: f64 = (assign25410_e30195 + assign25410_e30202);
        let assign25410_e30205: f64 = (assign25410_e30203 * var_tmp);
        (assign25410_e30205, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign25410_e30199 * var_terfc_dn6)))) * var_tmp) + (assign25410_e30203 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign25410_e30199 * var_terfc_dn7)))) * var_tmp) + (assign25410_e30203 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign25410_e30199 * var_terfc_dn8)))) * var_tmp) + (assign25410_e30203 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign25410_e30199 * var_terfc_dn9)))) * var_tmp) + (assign25410_e30203 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign25410_e30207;
        var_erfcpos_dn6 = assign25410_e30207_d_n6;
        var_erfcpos_dn7 = assign25410_e30207_d_n7;
        var_erfcpos_dn8 = assign25410_e30207_d_n8;
        var_erfcpos_dn9 = assign25410_e30207_d_n9;

        let assign25420_e30210: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard495 = assign25420_e30210;

        let (assign25430_e30224, assign25430_e30224_d_n6, assign25430_e30224_d_n7, assign25430_e30224_d_n8, assign25430_e30224_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) && (var_guard495 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign25430_e30224;
        var_erfctimesexpmtat_dn6 = assign25430_e30224_d_n6;
        var_erfctimesexpmtat_dn7 = assign25430_e30224_d_n7;
        var_erfctimesexpmtat_dn8 = assign25430_e30224_d_n8;
        var_erfctimesexpmtat_dn9 = assign25430_e30224_d_n9;

        let assign25440_e30227: f64 = (-230.25850929940458);
        let assign25440_e30228: f64 = if var_mtat > assign25440_e30227 { 1.0 } else { 0.0 };
        var_guard496 = assign25440_e30228;

        let (assign25450_e30246, assign25450_e30246_d_n6, assign25450_e30246_d_n7, assign25450_e30246_d_n8, assign25450_e30246_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) && (var_guard495 == 0.0)) && (var_guard496 != 0.0)) {
        let assign25450_e30244: f64 = (var_mtat).exp();
        (assign25450_e30244, (assign25450_e30244 * var_mtat_dn6), (assign25450_e30244 * var_mtat_dn7), (assign25450_e30244 * var_mtat_dn8), (assign25450_e30244 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign25450_e30246;
        var_tmp_dn6 = assign25450_e30246_d_n6;
        var_tmp_dn7 = assign25450_e30246_d_n7;
        var_tmp_dn8 = assign25450_e30246_d_n8;
        var_tmp_dn9 = assign25450_e30246_d_n9;

        let (assign25460_e30289, assign25460_e30289_d_n6, assign25460_e30289_d_n7, assign25460_e30289_d_n8, assign25460_e30289_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) && (var_guard495 == 0.0)) && (var_guard496 == 0.0)) {
        let assign25460_e30265: f64 = (-230.25850929940458);
        let assign25460_e30267: f64 = (assign25460_e30265 - var_mtat);
        let assign25460_e30271: f64 = (-230.25850929940458);
        let assign25460_e30273: f64 = (assign25460_e30271 - var_mtat);
        let assign25460_e30276: f64 = (-230.25850929940458);
        let assign25460_e30278: f64 = (assign25460_e30276 - var_mtat);
        let assign25460_e30280: f64 = (assign25460_e30278 * 0.3333333333333333);
        let assign25460_e30281: f64 = (1.0 + assign25460_e30280);
        let assign25460_e30282: f64 = (assign25460_e30273 * assign25460_e30281);
        let assign25460_e30283: f64 = (0.5 * assign25460_e30282);
        let assign25460_e30284: f64 = (1.0 + assign25460_e30283);
        let assign25460_e30285: f64 = (assign25460_e30267 * assign25460_e30284);
        let assign25460_e30286: f64 = (1.0 + assign25460_e30285);
        let assign25460_e30287: f64 = (1e-100 / assign25460_e30286);
        (assign25460_e30287, (-((1e-100 * (((-var_mtat_dn6) * assign25460_e30284) + (assign25460_e30267 * (0.5 * (((-var_mtat_dn6) * assign25460_e30281) + (assign25460_e30273 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign25460_e30286 * assign25460_e30286))), (-((1e-100 * (((-var_mtat_dn7) * assign25460_e30284) + (assign25460_e30267 * (0.5 * (((-var_mtat_dn7) * assign25460_e30281) + (assign25460_e30273 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign25460_e30286 * assign25460_e30286))), (-((1e-100 * (((-var_mtat_dn8) * assign25460_e30284) + (assign25460_e30267 * (0.5 * (((-var_mtat_dn8) * assign25460_e30281) + (assign25460_e30273 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign25460_e30286 * assign25460_e30286))), (-((1e-100 * (((-var_mtat_dn9) * assign25460_e30284) + (assign25460_e30267 * (0.5 * (((-var_mtat_dn9) * assign25460_e30281) + (assign25460_e30273 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign25460_e30286 * assign25460_e30286))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign25460_e30289;
        var_tmp_dn6 = assign25460_e30289_d_n6;
        var_tmp_dn7 = assign25460_e30289_d_n7;
        var_tmp_dn8 = assign25460_e30289_d_n8;
        var_tmp_dn9 = assign25460_e30289_d_n9;

        let (assign25470_e30308, assign25470_e30308_d_n6, assign25470_e30308_d_n7, assign25470_e30308_d_n8, assign25470_e30308_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) && (var_guard495 == 0.0)) {
        let assign25470_e30304: f64 = (2.0 * var_tmp);
        let assign25470_e30306: f64 = (assign25470_e30304 - var_erfcpos);
        (assign25470_e30306, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign25470_e30308;
        var_erfctimesexpmtat_dn6 = assign25470_e30308_d_n6;
        var_erfctimesexpmtat_dn7 = assign25470_e30308_d_n7;
        var_erfctimesexpmtat_dn8 = assign25470_e30308_d_n8;
        var_erfctimesexpmtat_dn9 = assign25470_e30308_d_n9;

        let (assign25480_e30328, assign25480_e30328_d_n6, assign25480_e30328_d_n7, assign25480_e30328_d_n8, assign25480_e30328_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) {
        let assign25480_e30320: f64 = (1.772453850905516 * 0.5);
        let assign25480_e30323: f64 = (var_atatsti * var_erfctimesexpmtat);
        let assign25480_e30325: f64 = (assign25480_e30323 / var_ktat);
        let assign25480_e30326: f64 = (assign25480_e30320 * assign25480_e30325);
        (assign25480_e30326, (assign25480_e30320 * ((((var_atatsti * var_erfctimesexpmtat_dn6) * var_ktat) - (assign25480_e30323 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign25480_e30320 * ((((var_atatsti * var_erfctimesexpmtat_dn7) * var_ktat) - (assign25480_e30323 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign25480_e30320 * ((((var_atatsti * var_erfctimesexpmtat_dn8) * var_ktat) - (assign25480_e30323 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign25480_e30320 * ((((var_atatsti * var_erfctimesexpmtat_dn9) * var_ktat) - (assign25480_e30323 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign25480_e30328;
        var_gammamax_dn6 = assign25480_e30328_d_n6;
        var_gammamax_dn7 = assign25480_e30328_d_n7;
        var_gammamax_dn8 = assign25480_e30328_d_n8;
        var_gammamax_dn9 = assign25480_e30328_d_n9;

        let (assign25490_e30346, assign25490_e30346_d_n6, assign25490_e30346_d_n7, assign25490_e30346_d_n8, assign25490_e30346_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard491 == 0.0)) {
        let assign25490_e30341: f64 = (var_asrh * var_gammamax);
        let assign25490_e30343: f64 = (assign25490_e30341 * var_wtat);
        let assign25490_e30344: f64 = (p.p863 * assign25490_e30343);
        (assign25490_e30344, (p.p863 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign25490_e30341 * var_wtat_dn6))), (p.p863 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign25490_e30341 * var_wtat_dn7))), (p.p863 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign25490_e30341 * var_wtat_dn8))), (p.p863 * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign25490_e30341 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign25490_e30346;
        var_itat_dn6 = assign25490_e30346_d_n6;
        var_itat_dn7 = assign25490_e30346_d_n7;
        var_itat_dn8 = assign25490_e30346_d_n8;
        var_itat_dn9 = assign25490_e30346_d_n9;

        let assign25500_e30349: f64 = if p.p869 == 0.0 { 1.0 } else { 0.0 };
        var_guard497 = assign25500_e30349;

        let (assign25510_e30360, assign25510_e30360_d_n6, assign25510_e30360_d_n7, assign25510_e30360_d_n8, assign25510_e30360_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard497 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign25510_e30360;
        var_ibbt_dn6 = assign25510_e30360_d_n6;
        var_ibbt_dn7 = assign25510_e30360_d_n7;
        var_ibbt_dn8 = assign25510_e30360_d_n8;
        var_ibbt_dn9 = assign25510_e30360_d_n9;

        let assign25520_e30363: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };
        var_guard498 = assign25520_e30363;

        let (assign25530_e30382, assign25530_e30382_d_n6, assign25530_e30382_d_n7, assign25530_e30382_d_n8, assign25530_e30382_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard497 == 0.0)) && (var_guard498 != 0.0)) {
        let assign25530_e30377: f64 = (p.p846 - var_vbbt);
        let assign25530_e30379: f64 = (assign25530_e30377 * var_vbirstiinv);
        let assign25530_e30380: f64 = (assign25530_e30379).sqrt();
        (assign25530_e30380, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign25530_e30382;
        var_tmp_dn6 = assign25530_e30382_d_n6;
        var_tmp_dn7 = assign25530_e30382_d_n7;
        var_tmp_dn8 = assign25530_e30382_d_n8;
        var_tmp_dn9 = assign25530_e30382_d_n9;

        let (assign25540_e30403, assign25540_e30403_d_n6, assign25540_e30403_d_n7, assign25540_e30403_d_n8, assign25540_e30403_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard497 == 0.0)) && (var_guard498 == 0.0)) {
        let assign25540_e30397: f64 = (p.p846 - var_vbbt);
        let assign25540_e30399: f64 = (assign25540_e30397 * var_vbirstiinv);
        let assign25540_e30401: f64 = (assign25540_e30399).powf(p.p849);
        (assign25540_e30401, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign25540_e30403;
        var_tmp_dn6 = assign25540_e30403_d_n6;
        var_tmp_dn7 = assign25540_e30403_d_n7;
        var_tmp_dn8 = assign25540_e30403_d_n8;
        var_tmp_dn9 = assign25540_e30403_d_n9;

        let (assign25550_e30423, assign25550_e30423_d_n6, assign25550_e30423_d_n7, assign25550_e30423_d_n8, assign25550_e30423_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard497 == 0.0)) {
        let assign25550_e30416: f64 = (p.p846 - var_vbbt);
        let assign25550_e30418: f64 = (assign25550_e30416 * var_wdepnulrinvsti);
        let assign25550_e30420: f64 = (assign25550_e30418 / var_tmp);
        let assign25550_e30421: f64 = (var_one_over_one_minus_psti * assign25550_e30420);
        (assign25550_e30421, (var_one_over_one_minus_psti * (-((assign25550_e30418 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign25550_e30418 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign25550_e30418 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign25550_e30418 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign25550_e30423;
        var_fmaxr_dn6 = assign25550_e30423_d_n6;
        var_fmaxr_dn7 = assign25550_e30423_d_n7;
        var_fmaxr_dn8 = assign25550_e30423_d_n8;
        var_fmaxr_dn9 = assign25550_e30423_d_n9;

        let assign25560_e30425: f64 = (-var_fbbtsti);
        let assign25560_e30427: f64 = (assign25560_e30425 / var_fmaxr);
        let assign25560_e30428: f64 = (assign25560_e30427).abs();
        let assign25560_e30430: f64 = if assign25560_e30428 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard499 = assign25560_e30430;

        let (assign25570_e30448, assign25570_e30448_d_n6, assign25570_e30448_d_n7, assign25570_e30448_d_n8, assign25570_e30448_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard497 == 0.0)) && (var_guard499 != 0.0)) {
        let assign25570_e30443: f64 = (-var_fbbtsti);
        let assign25570_e30445: f64 = (assign25570_e30443 / var_fmaxr);
        let assign25570_e30446: f64 = (assign25570_e30445).exp();
        (assign25570_e30446, (assign25570_e30446 * (-((assign25570_e30443 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign25570_e30446 * (-((assign25570_e30443 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign25570_e30446 * (-((assign25570_e30443 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign25570_e30446 * (-((assign25570_e30443 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign25570_e30448;
        var_tmp_dn6 = assign25570_e30448_d_n6;
        var_tmp_dn7 = assign25570_e30448_d_n7;
        var_tmp_dn8 = assign25570_e30448_d_n8;
        var_tmp_dn9 = assign25570_e30448_d_n9;

        let assign25580_e30450: f64 = (-var_fbbtsti);
        let assign25580_e30452: f64 = (assign25580_e30450 / var_fmaxr);
        let assign25580_e30454: f64 = if assign25580_e30452 < 0.0 { 1.0 } else { 0.0 };
        var_guard500 = assign25580_e30454;

        let (assign25590_e30505, assign25590_e30505_d_n6, assign25590_e30505_d_n7, assign25590_e30505_d_n8, assign25590_e30505_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard497 == 0.0)) && (var_guard499 == 0.0)) && (var_guard500 != 0.0)) {
        let assign25590_e30472: f64 = (-230.25850929940458);
        let assign25590_e30474: f64 = (-var_fbbtsti);
        let assign25590_e30476: f64 = (assign25590_e30474 / var_fmaxr);
        let assign25590_e30477: f64 = (assign25590_e30472 - assign25590_e30476);
        let assign25590_e30481: f64 = (-230.25850929940458);
        let assign25590_e30483: f64 = (-var_fbbtsti);
        let assign25590_e30485: f64 = (assign25590_e30483 / var_fmaxr);
        let assign25590_e30486: f64 = (assign25590_e30481 - assign25590_e30485);
        let assign25590_e30489: f64 = (-230.25850929940458);
        let assign25590_e30491: f64 = (-var_fbbtsti);
        let assign25590_e30493: f64 = (assign25590_e30491 / var_fmaxr);
        let assign25590_e30494: f64 = (assign25590_e30489 - assign25590_e30493);
        let assign25590_e30496: f64 = (assign25590_e30494 * 0.3333333333333333);
        let assign25590_e30497: f64 = (1.0 + assign25590_e30496);
        let assign25590_e30498: f64 = (assign25590_e30486 * assign25590_e30497);
        let assign25590_e30499: f64 = (0.5 * assign25590_e30498);
        let assign25590_e30500: f64 = (1.0 + assign25590_e30499);
        let assign25590_e30501: f64 = (assign25590_e30477 * assign25590_e30500);
        let assign25590_e30502: f64 = (1.0 + assign25590_e30501);
        let assign25590_e30503: f64 = (1e-100 / assign25590_e30502);
        (assign25590_e30503, (-((1e-100 * (((-(-((assign25590_e30474 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign25590_e30500) + (assign25590_e30477 * (0.5 * (((-(-((assign25590_e30483 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign25590_e30497) + (assign25590_e30486 * ((-(-((assign25590_e30491 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign25590_e30502 * assign25590_e30502))), (-((1e-100 * (((-(-((assign25590_e30474 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign25590_e30500) + (assign25590_e30477 * (0.5 * (((-(-((assign25590_e30483 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign25590_e30497) + (assign25590_e30486 * ((-(-((assign25590_e30491 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign25590_e30502 * assign25590_e30502))), (-((1e-100 * (((-(-((assign25590_e30474 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign25590_e30500) + (assign25590_e30477 * (0.5 * (((-(-((assign25590_e30483 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign25590_e30497) + (assign25590_e30486 * ((-(-((assign25590_e30491 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign25590_e30502 * assign25590_e30502))), (-((1e-100 * (((-(-((assign25590_e30474 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign25590_e30500) + (assign25590_e30477 * (0.5 * (((-(-((assign25590_e30483 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign25590_e30497) + (assign25590_e30486 * ((-(-((assign25590_e30491 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign25590_e30502 * assign25590_e30502))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign25590_e30505;
        var_tmp_dn6 = assign25590_e30505_d_n6;
        var_tmp_dn7 = assign25590_e30505_d_n7;
        var_tmp_dn8 = assign25590_e30505_d_n8;
        var_tmp_dn9 = assign25590_e30505_d_n9;

        let (assign25600_e30554, assign25600_e30554_d_n6, assign25600_e30554_d_n7, assign25600_e30554_d_n8, assign25600_e30554_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard497 == 0.0)) && (var_guard499 == 0.0)) && (var_guard500 == 0.0)) {
        let assign25600_e30524: f64 = (-var_fbbtsti);
        let assign25600_e30526: f64 = (assign25600_e30524 / var_fmaxr);
        let assign25600_e30528: f64 = (assign25600_e30526 - 230.25850929940458);
        let assign25600_e30532: f64 = (-var_fbbtsti);
        let assign25600_e30534: f64 = (assign25600_e30532 / var_fmaxr);
        let assign25600_e30536: f64 = (assign25600_e30534 - 230.25850929940458);
        let assign25600_e30539: f64 = (-var_fbbtsti);
        let assign25600_e30541: f64 = (assign25600_e30539 / var_fmaxr);
        let assign25600_e30543: f64 = (assign25600_e30541 - 230.25850929940458);
        let assign25600_e30545: f64 = (assign25600_e30543 * 0.3333333333333333);
        let assign25600_e30546: f64 = (1.0 + assign25600_e30545);
        let assign25600_e30547: f64 = (assign25600_e30536 * assign25600_e30546);
        let assign25600_e30548: f64 = (0.5 * assign25600_e30547);
        let assign25600_e30549: f64 = (1.0 + assign25600_e30548);
        let assign25600_e30550: f64 = (assign25600_e30528 * assign25600_e30549);
        let assign25600_e30551: f64 = (1.0 + assign25600_e30550);
        let assign25600_e30552: f64 = (1e100 * assign25600_e30551);
        (assign25600_e30552, (1e100 * (((-((assign25600_e30524 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign25600_e30549) + (assign25600_e30528 * (0.5 * (((-((assign25600_e30532 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign25600_e30546) + (assign25600_e30536 * ((-((assign25600_e30539 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign25600_e30524 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign25600_e30549) + (assign25600_e30528 * (0.5 * (((-((assign25600_e30532 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign25600_e30546) + (assign25600_e30536 * ((-((assign25600_e30539 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign25600_e30524 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign25600_e30549) + (assign25600_e30528 * (0.5 * (((-((assign25600_e30532 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign25600_e30546) + (assign25600_e30536 * ((-((assign25600_e30539 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign25600_e30524 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign25600_e30549) + (assign25600_e30528 * (0.5 * (((-((assign25600_e30532 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign25600_e30546) + (assign25600_e30536 * ((-((assign25600_e30539 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign25600_e30554;
        var_tmp_dn6 = assign25600_e30554_d_n6;
        var_tmp_dn7 = assign25600_e30554_d_n7;
        var_tmp_dn8 = assign25600_e30554_d_n8;
        var_tmp_dn9 = assign25600_e30554_d_n9;

        let (assign25610_e30574, assign25610_e30574_d_n6, assign25610_e30574_d_n7, assign25610_e30574_d_n8, assign25610_e30574_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard497 == 0.0)) {
        let assign25610_e30567: f64 = (var_v5 * var_fmaxr);
        let assign25610_e30569: f64 = (assign25610_e30567 * var_fmaxr);
        let assign25610_e30571: f64 = (assign25610_e30569 * var_tmp);
        let assign25610_e30572: f64 = (p.p869 * assign25610_e30571);
        (assign25610_e30572, (p.p869 * (((((var_v5 * var_fmaxr_dn6) * var_fmaxr) + (assign25610_e30567 * var_fmaxr_dn6)) * var_tmp) + (assign25610_e30569 * var_tmp_dn6))), (p.p869 * (((((var_v5 * var_fmaxr_dn7) * var_fmaxr) + (assign25610_e30567 * var_fmaxr_dn7)) * var_tmp) + (assign25610_e30569 * var_tmp_dn7))), (p.p869 * (((((var_v5 * var_fmaxr_dn8) * var_fmaxr) + (assign25610_e30567 * var_fmaxr_dn8)) * var_tmp) + (assign25610_e30569 * var_tmp_dn8))), (p.p869 * (((((var_v5 * var_fmaxr_dn9) * var_fmaxr) + (assign25610_e30567 * var_fmaxr_dn9)) * var_tmp) + (assign25610_e30569 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign25610_e30574;
        var_ibbt_dn6 = assign25610_e30574_d_n6;
        var_ibbt_dn7 = assign25610_e30574_d_n7;
        var_ibbt_dn8 = assign25610_e30574_d_n8;
        var_ibbt_dn9 = assign25610_e30574_d_n9;

        let assign25620_e30577: f64 = if p.p878 > 1000.0 { 1.0 } else { 0.0 };
        var_guard501 = assign25620_e30577;

        let (assign25630_e30588, assign25630_e30588_d_n6, assign25630_e30588_d_n7, assign25630_e30588_d_n8, assign25630_e30588_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard501 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign25630_e30588;
        var_fbreakdown_dn6 = assign25630_e30588_d_n6;
        var_fbreakdown_dn7 = assign25630_e30588_d_n7;
        var_fbreakdown_dn8 = assign25630_e30588_d_n8;
        var_fbreakdown_dn9 = assign25630_e30588_d_n9;

        let assign25640_e30591: f64 = (-var_alphaav);
        let assign25640_e30593: f64 = (assign25640_e30591 * p.p878);
        let assign25640_e30594: f64 = if var_vav > assign25640_e30593 { 1.0 } else { 0.0 };
        var_guard502 = assign25640_e30594;

        let assign25650_e30597: f64 = if p.p881 == 4.0 { 1.0 } else { 0.0 };
        var_guard503 = assign25650_e30597;

        let (assign25660_e30627, assign25660_e30627_d_n6, assign25660_e30627_d_n7, assign25660_e30627_d_n8, assign25660_e30627_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard501 == 0.0)) && (var_guard502 != 0.0)) && (var_guard503 != 0.0)) {
        let assign25660_e30613: f64 = (var_vav * var_vbrinvsti);
        let assign25660_e30616: f64 = (var_vav * var_vbrinvsti);
        let assign25660_e30617: f64 = (assign25660_e30613 * assign25660_e30616);
        let assign25660_e30620: f64 = (var_vav * var_vbrinvsti);
        let assign25660_e30621: f64 = (assign25660_e30617 * assign25660_e30620);
        let assign25660_e30624: f64 = (var_vav * var_vbrinvsti);
        let assign25660_e30625: f64 = (assign25660_e30621 * assign25660_e30624);
        (assign25660_e30625, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign25660_e30627;
        var_tmp_dn6 = assign25660_e30627_d_n6;
        var_tmp_dn7 = assign25660_e30627_d_n7;
        var_tmp_dn8 = assign25660_e30627_d_n8;
        var_tmp_dn9 = assign25660_e30627_d_n9;

        let (assign25670_e30649, assign25670_e30649_d_n6, assign25670_e30649_d_n7, assign25670_e30649_d_n8, assign25670_e30649_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard501 == 0.0)) && (var_guard502 != 0.0)) && (var_guard503 == 0.0)) {
        let assign25670_e30644: f64 = (var_vav * var_vbrinvsti);
        let assign25670_e30645: f64 = (assign25670_e30644).abs();
        let assign25670_e30647: f64 = (assign25670_e30645).powf(p.p881);
        (assign25670_e30647, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign25670_e30649;
        var_tmp_dn6 = assign25670_e30649_d_n6;
        var_tmp_dn7 = assign25670_e30649_d_n7;
        var_tmp_dn8 = assign25670_e30649_d_n8;
        var_tmp_dn9 = assign25670_e30649_d_n9;

        let (assign25680_e30667, assign25680_e30667_d_n6, assign25680_e30667_d_n7, assign25680_e30667_d_n8, assign25680_e30667_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard501 == 0.0)) && (var_guard502 != 0.0)) {
        let assign25680_e30664: f64 = (1.0 - var_tmp);
        let assign25680_e30665: f64 = (1.0 / assign25680_e30664);
        (assign25680_e30665, (-((-var_tmp_dn6) / (assign25680_e30664 * assign25680_e30664))), (-((-var_tmp_dn7) / (assign25680_e30664 * assign25680_e30664))), (-((-var_tmp_dn8) / (assign25680_e30664 * assign25680_e30664))), (-((-var_tmp_dn9) / (assign25680_e30664 * assign25680_e30664))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign25680_e30667;
        var_fbreakdown_dn6 = assign25680_e30667_d_n6;
        var_fbreakdown_dn7 = assign25680_e30667_d_n7;
        var_fbreakdown_dn8 = assign25680_e30667_d_n8;
        var_fbreakdown_dn9 = assign25680_e30667_d_n9;

        let (assign25690_e30690, assign25690_e30690_d_n6, assign25690_e30690_d_n7, assign25690_e30690_d_n8, assign25690_e30690_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) && (var_guard501 == 0.0)) && (var_guard502 == 0.0)) {
        let assign25690_e30684: f64 = (var_alphaav * p.p878);
        let assign25690_e30685: f64 = (var_vav + assign25690_e30684);
        let assign25690_e30687: f64 = (assign25690_e30685 * var_slopesti);
        let assign25690_e30688: f64 = (var_fstopsti + assign25690_e30687);
        (assign25690_e30688, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign25690_e30690;
        var_fbreakdown_dn6 = assign25690_e30690_d_n6;
        var_fbreakdown_dn7 = assign25690_e30690_d_n7;
        var_fbreakdown_dn8 = assign25690_e30690_d_n8;
        var_fbreakdown_dn9 = assign25690_e30690_d_n9;

        let (assign25700_e30709, assign25700_e30709_d_n6, assign25700_e30709_d_n7, assign25700_e30709_d_n8, assign25700_e30709_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard487 == 0.0)) {
        let assign25700_e30700: f64 = (var_id__blk212 + var_isrh);
        let assign25700_e30702: f64 = (assign25700_e30700 + var_itat);
        let assign25700_e30704: f64 = (assign25700_e30702 + var_ibbt);
        let assign25700_e30705: f64 = (p.p29 * assign25700_e30704);
        let assign25700_e30707: f64 = (assign25700_e30705 * var_fbreakdown);
        (assign25700_e30707, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign25700_e30705 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign25700_e30705 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign25700_e30705 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign25700_e30705 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign25700_e30709;
        var_ijunsti_dn6 = assign25700_e30709_d_n6;
        var_ijunsti_dn7 = assign25700_e30709_d_n7;
        var_ijunsti_dn8 = assign25700_e30709_d_n8;
        var_ijunsti_dn9 = assign25700_e30709_d_n9;

        let assign25710_e30712: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard504 = assign25710_e30712;

        let (assign25720_e30720, assign25720_e30720_d_n6, assign25720_e30720_d_n7, assign25720_e30720_d_n8, assign25720_e30720_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign25720_e30720;
        var_ijungat_dn6 = assign25720_e30720_d_n6;
        var_ijungat_dn7 = assign25720_e30720_d_n7;
        var_ijungat_dn8 = assign25720_e30720_d_n8;
        var_ijungat_dn9 = assign25720_e30720_d_n9;

        let (assign25730_e30731,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) {
        let assign25730_e30729: f64 = (var_idsatgat * var_idmult);
        (assign25730_e30729,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign25730_e30731;

        let assign25740_e30738: f64 = if ((p.p859 == 0.0) && (p.p864 == 0.0)) { 1.0 } else { 0.0 };
        var_guard505 = assign25740_e30738;

        let (assign25750_e30749, assign25750_e30749_d_n6, assign25750_e30749_d_n7, assign25750_e30749_d_n8, assign25750_e30749_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard505 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign25750_e30749;
        var_isrh_dn6 = assign25750_e30749_d_n6;
        var_isrh_dn7 = assign25750_e30749_d_n7;
        var_isrh_dn8 = assign25750_e30749_d_n8;
        var_isrh_dn9 = assign25750_e30749_d_n9;

        let (assign25760_e30763,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard505 == 0.0)) {
        let assign25760_e30761: f64 = (var_vbigat - var_vjsrh);
        (assign25760_e30761,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign25760_e30763;

        let (assign25770_e30782,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard505 == 0.0)) {
        let assign25770_e30777: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign25770_e30778: f64 = (1.0 - assign25770_e30777);
        let assign25770_e30779: f64 = (assign25770_e30778).sqrt();
        let assign25770_e30780: f64 = (1.0 - assign25770_e30779);
        (assign25770_e30780,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign25770_e30782;

        let assign25780_e30785: f64 = if p.p850 == 0.5 { 1.0 } else { 0.0 };
        var_guard506 = assign25780_e30785;

        let (assign25790_e30799,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard505 == 0.0)) && (var_guard506 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign25790_e30799;

        let (assign25800_e30831,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard505 == 0.0)) && (var_guard506 == 0.0)) {
        let assign25800_e30814: f64 = (var_wsrhstep * var_wsrhstep);
        let assign25800_e30816: f64 = (var_wsrhstep).ln();
        let assign25800_e30817: f64 = (assign25800_e30814 * assign25800_e30816);
        let assign25800_e30820: f64 = (1.0 - var_wsrhstep);
        let assign25800_e30821: f64 = (assign25800_e30817 / assign25800_e30820);
        let assign25800_e30823: f64 = (assign25800_e30821 + var_wsrhstep);
        let assign25800_e30827: f64 = (2.0 * p.p850);
        let assign25800_e30828: f64 = (1.0 - assign25800_e30827);
        let assign25800_e30829: f64 = (assign25800_e30823 * assign25800_e30828);
        (assign25800_e30829,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign25800_e30831;

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
        *var_guard505_slot = var_guard505;
        *var_guard506_slot = var_guard506;
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

    pub(super) fn stamp_transient_block_50(
        p: &Parameters,
        var_atatgat: f64,
        var_berfc: f64,
        var_btatpartgat: f64,
        var_cerfc: f64,
        var_dwsrh: f64,
        var_ftdgat: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard504: f64,
        var_guard505: f64,
        var_one_minus_pgat: f64,
        var_one_over_one_minus_pgat: f64,
        var_perfc: f64,
        var_vbi_minus_vjsrh: f64,
        var_vbirgatinv: f64,
        var_wdepnulrgat: f64,
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
        var_guard507_slot: &mut f64,
        var_guard508_slot: &mut f64,
        var_guard509_slot: &mut f64,
        var_guard510_slot: &mut f64,
        var_guard511_slot: &mut f64,
        var_guard512_slot: &mut f64,
        var_guard513_slot: &mut f64,
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
        let mut var_guard507: f64 = *var_guard507_slot;
        let mut var_guard508: f64 = *var_guard508_slot;
        let mut var_guard509: f64 = *var_guard509_slot;
        let mut var_guard510: f64 = *var_guard510_slot;
        let mut var_guard511: f64 = *var_guard511_slot;
        let mut var_guard512: f64 = *var_guard512_slot;
        let mut var_guard513: f64 = *var_guard513_slot;
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

        let (assign25810_e30845,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard505 == 0.0)) {
        let assign25810_e30843: f64 = (var_wsrhstep + var_dwsrh);
        (assign25810_e30843,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign25810_e30845;

        let assign25820_e30848: f64 = if p.p850 == 0.5 { 1.0 } else { 0.0 };
        var_guard507 = assign25820_e30848;

        let (assign25830_e30865, assign25830_e30865_d_n6, assign25830_e30865_d_n7, assign25830_e30865_d_n8, assign25830_e30865_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard505 == 0.0)) && (var_guard507 != 0.0)) {
        let assign25830_e30862: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign25830_e30863: f64 = (assign25830_e30862).sqrt();
        (assign25830_e30863, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign25830_e30865;
        var_tmp_dn6 = assign25830_e30865_d_n6;
        var_tmp_dn7 = assign25830_e30865_d_n7;
        var_tmp_dn8 = assign25830_e30865_d_n8;
        var_tmp_dn9 = assign25830_e30865_d_n9;

        let (assign25840_e30884, assign25840_e30884_d_n6, assign25840_e30884_d_n7, assign25840_e30884_d_n8, assign25840_e30884_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard505 == 0.0)) && (var_guard507 == 0.0)) {
        let assign25840_e30880: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign25840_e30882: f64 = (assign25840_e30880).powf(p.p850);
        (assign25840_e30882, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign25840_e30884;
        var_tmp_dn6 = assign25840_e30884_d_n6;
        var_tmp_dn7 = assign25840_e30884_d_n7;
        var_tmp_dn8 = assign25840_e30884_d_n8;
        var_tmp_dn9 = assign25840_e30884_d_n9;

        let (assign25850_e30898, assign25850_e30898_d_n6, assign25850_e30898_d_n7, assign25850_e30898_d_n8, assign25850_e30898_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard505 == 0.0)) {
        let assign25850_e30896: f64 = (var_wdepnulrgat * var_tmp);
        (assign25850_e30896, (var_wdepnulrgat * var_tmp_dn6), (var_wdepnulrgat * var_tmp_dn7), (var_wdepnulrgat * var_tmp_dn8), (var_wdepnulrgat * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign25850_e30898;
        var_wdep_dn6 = assign25850_e30898_d_n6;
        var_wdep_dn7 = assign25850_e30898_d_n7;
        var_wdep_dn8 = assign25850_e30898_d_n8;
        var_wdep_dn9 = assign25850_e30898_d_n9;

        let (assign25860_e30916, assign25860_e30916_d_n6, assign25860_e30916_d_n7, assign25860_e30916_d_n8, assign25860_e30916_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard505 == 0.0)) {
        let assign25860_e30911: f64 = (var_zinv - 1.0);
        let assign25860_e30913: f64 = (assign25860_e30911 * var_wdep);
        let assign25860_e30914: f64 = (var_ftdgat * assign25860_e30913);
        (assign25860_e30914, (var_ftdgat * (assign25860_e30911 * var_wdep_dn6)), (var_ftdgat * (assign25860_e30911 * var_wdep_dn7)), (var_ftdgat * (assign25860_e30911 * var_wdep_dn8)), (var_ftdgat * (assign25860_e30911 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign25860_e30916;
        var_asrh_dn6 = assign25860_e30916_d_n6;
        var_asrh_dn7 = assign25860_e30916_d_n7;
        var_asrh_dn8 = assign25860_e30916_d_n8;
        var_asrh_dn9 = assign25860_e30916_d_n9;

        let (assign25870_e30932, assign25870_e30932_d_n6, assign25870_e30932_d_n7, assign25870_e30932_d_n8, assign25870_e30932_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard505 == 0.0)) {
        let assign25870_e30929: f64 = (var_asrh * var_wsrh);
        let assign25870_e30930: f64 = (p.p859 * assign25870_e30929);
        (assign25870_e30930, (p.p859 * (var_asrh_dn6 * var_wsrh)), (p.p859 * (var_asrh_dn7 * var_wsrh)), (p.p859 * (var_asrh_dn8 * var_wsrh)), (p.p859 * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign25870_e30932;
        var_isrh_dn6 = assign25870_e30932_d_n6;
        var_isrh_dn7 = assign25870_e30932_d_n7;
        var_isrh_dn8 = assign25870_e30932_d_n8;
        var_isrh_dn9 = assign25870_e30932_d_n9;

        let assign25880_e30935: f64 = if p.p864 == 0.0 { 1.0 } else { 0.0 };
        var_guard508 = assign25880_e30935;

        let (assign25890_e30946, assign25890_e30946_d_n6, assign25890_e30946_d_n7, assign25890_e30946_d_n8, assign25890_e30946_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign25890_e30946;
        var_itat_dn6 = assign25890_e30946_d_n6;
        var_itat_dn7 = assign25890_e30946_d_n7;
        var_itat_dn8 = assign25890_e30946_d_n8;
        var_itat_dn9 = assign25890_e30946_d_n9;

        let (assign25900_e30964, assign25900_e30964_d_n6, assign25900_e30964_d_n7, assign25900_e30964_d_n8, assign25900_e30964_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) {
        let assign25900_e30959: f64 = (var_wdep * var_one_minus_pgat);
        let assign25900_e30961: f64 = (assign25900_e30959 / var_vbi_minus_vjsrh);
        let assign25900_e30962: f64 = (var_btatpartgat * assign25900_e30961);
        (assign25900_e30962, (var_btatpartgat * ((var_wdep_dn6 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn7 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn8 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn9 * var_one_minus_pgat) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign25900_e30964;
        var_btat_dn6 = assign25900_e30964_d_n6;
        var_btat_dn7 = assign25900_e30964_d_n7;
        var_btat_dn8 = assign25900_e30964_d_n8;
        var_btat_dn9 = assign25900_e30964_d_n9;

        let (assign25910_e30980, assign25910_e30980_d_n6, assign25910_e30980_d_n7, assign25910_e30980_d_n8, assign25910_e30980_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) {
        let assign25910_e30976: f64 = (0.666666666666667 * var_atatgat);
        let assign25910_e30978: f64 = (assign25910_e30976 / var_btat);
        (assign25910_e30978, (-((assign25910_e30976 * var_btat_dn6) / (var_btat * var_btat))), (-((assign25910_e30976 * var_btat_dn7) / (var_btat * var_btat))), (-((assign25910_e30976 * var_btat_dn8) / (var_btat * var_btat))), (-((assign25910_e30976 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign25910_e30980;
        var_twoatatoverthreebtat_dn6 = assign25910_e30980_d_n6;
        var_twoatatoverthreebtat_dn7 = assign25910_e30980_d_n7;
        var_twoatatoverthreebtat_dn8 = assign25910_e30980_d_n8;
        var_twoatatoverthreebtat_dn9 = assign25910_e30980_d_n9;

        let (assign25920_e30994, assign25920_e30994_d_n6, assign25920_e30994_d_n7, assign25920_e30994_d_n8, assign25920_e30994_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) {
        let assign25920_e30992: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign25920_e30992, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign25920_e30994;
        var_umaxbeforelimiting_dn6 = assign25920_e30994_d_n6;
        var_umaxbeforelimiting_dn7 = assign25920_e30994_d_n7;
        var_umaxbeforelimiting_dn8 = assign25920_e30994_d_n8;
        var_umaxbeforelimiting_dn9 = assign25920_e30994_d_n9;

        let (assign25930_e31015, assign25930_e31015_d_n6, assign25930_e31015_d_n7, assign25930_e31015_d_n8, assign25930_e31015_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) {
        let assign25930_e31006: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign25930_e31009: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign25930_e31011: f64 = (assign25930_e31009 + 1.0);
        let assign25930_e31012: f64 = (assign25930_e31006 / assign25930_e31011);
        let assign25930_e31013: f64 = (assign25930_e31012).sqrt();
        (assign25930_e31013, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign25930_e31011) - (assign25930_e31006 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign25930_e31011 * assign25930_e31011)) / (2.0 * assign25930_e31013)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign25930_e31011) - (assign25930_e31006 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign25930_e31011 * assign25930_e31011)) / (2.0 * assign25930_e31013)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign25930_e31011) - (assign25930_e31006 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign25930_e31011 * assign25930_e31011)) / (2.0 * assign25930_e31013)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign25930_e31011) - (assign25930_e31006 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign25930_e31011 * assign25930_e31011)) / (2.0 * assign25930_e31013)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign25930_e31015;
        var_umax_dn6 = assign25930_e31015_d_n6;
        var_umax_dn7 = assign25930_e31015_d_n7;
        var_umax_dn8 = assign25930_e31015_d_n8;
        var_umax_dn9 = assign25930_e31015_d_n9;

        let (assign25940_e31028, assign25940_e31028_d_n6, assign25940_e31028_d_n7, assign25940_e31028_d_n8, assign25940_e31028_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) {
        let assign25940_e31026: f64 = (var_umax).sqrt();
        (assign25940_e31026, (var_umax_dn6 / (2.0 * assign25940_e31026)), (var_umax_dn7 / (2.0 * assign25940_e31026)), (var_umax_dn8 / (2.0 * assign25940_e31026)), (var_umax_dn9 / (2.0 * assign25940_e31026)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign25940_e31028;
        var_sqrtumax_dn6 = assign25940_e31028_d_n6;
        var_sqrtumax_dn7 = assign25940_e31028_d_n7;
        var_sqrtumax_dn8 = assign25940_e31028_d_n8;
        var_sqrtumax_dn9 = assign25940_e31028_d_n9;

        let (assign25950_e31042, assign25950_e31042_d_n6, assign25950_e31042_d_n7, assign25950_e31042_d_n8, assign25950_e31042_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) {
        let assign25950_e31040: f64 = (var_umax * var_sqrtumax);
        (assign25950_e31040, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign25950_e31042;
        var_umaxpoweronepointfive_dn6 = assign25950_e31042_d_n6;
        var_umaxpoweronepointfive_dn7 = assign25950_e31042_d_n7;
        var_umaxpoweronepointfive_dn8 = assign25950_e31042_d_n8;
        var_umaxpoweronepointfive_dn9 = assign25950_e31042_d_n9;

        let assign25960_e31044: f64 = (-p.p850);
        let assign25960_e31046: f64 = (assign25960_e31044 * var_one_over_one_minus_pgat);
        let assign25960_e31048: f64 = (-1.0);
        let assign25960_e31049: f64 = if assign25960_e31046 == assign25960_e31048 { 1.0 } else { 0.0 };
        var_guard509 = assign25960_e31049;

        let (assign25970_e31069, assign25970_e31069_d_n6, assign25970_e31069_d_n7, assign25970_e31069_d_n8, assign25970_e31069_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) && (var_guard509 != 0.0)) {
        let assign25970_e31065: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign25970_e31066: f64 = (1.0 + assign25970_e31065);
        let assign25970_e31067: f64 = (1.0 / assign25970_e31066);
        (assign25970_e31067, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign25970_e31066 * assign25970_e31066))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign25970_e31066 * assign25970_e31066))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign25970_e31066 * assign25970_e31066))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign25970_e31066 * assign25970_e31066))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign25970_e31069;
        var_wgamma_dn6 = assign25970_e31069_d_n6;
        var_wgamma_dn7 = assign25970_e31069_d_n7;
        var_wgamma_dn8 = assign25970_e31069_d_n8;
        var_wgamma_dn9 = assign25970_e31069_d_n9;

        let (assign25980_e31093, assign25980_e31093_d_n6, assign25980_e31093_d_n7, assign25980_e31093_d_n8, assign25980_e31093_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) && (var_guard509 == 0.0)) {
        let assign25980_e31085: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign25980_e31086: f64 = (1.0 + assign25980_e31085);
        let assign25980_e31088: f64 = (-p.p850);
        let assign25980_e31090: f64 = (assign25980_e31088 * var_one_over_one_minus_pgat);
        let assign25980_e31091: f64 = (assign25980_e31086).powf(assign25980_e31090);
        (assign25980_e31091, if 0.0 == 0.0 && ((assign25980_e31090) as f64).is_finite() && ((assign25980_e31090) as f64).fract() == 0.0 { if assign25980_e31090 == 0.0 { 0.0 } else { (assign25980_e31090 * ((assign25980_e31086).powf(assign25980_e31090 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign25980_e31091 * (assign25980_e31090 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign25980_e31086))) }, if 0.0 == 0.0 && ((assign25980_e31090) as f64).is_finite() && ((assign25980_e31090) as f64).fract() == 0.0 { if assign25980_e31090 == 0.0 { 0.0 } else { (assign25980_e31090 * ((assign25980_e31086).powf(assign25980_e31090 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign25980_e31091 * (assign25980_e31090 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign25980_e31086))) }, if 0.0 == 0.0 && ((assign25980_e31090) as f64).is_finite() && ((assign25980_e31090) as f64).fract() == 0.0 { if assign25980_e31090 == 0.0 { 0.0 } else { (assign25980_e31090 * ((assign25980_e31086).powf(assign25980_e31090 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign25980_e31091 * (assign25980_e31090 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign25980_e31086))) }, if 0.0 == 0.0 && ((assign25980_e31090) as f64).is_finite() && ((assign25980_e31090) as f64).fract() == 0.0 { if assign25980_e31090 == 0.0 { 0.0 } else { (assign25980_e31090 * ((assign25980_e31086).powf(assign25980_e31090 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign25980_e31091 * (assign25980_e31090 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign25980_e31086))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign25980_e31093;
        var_wgamma_dn6 = assign25980_e31093_d_n6;
        var_wgamma_dn7 = assign25980_e31093_d_n7;
        var_wgamma_dn8 = assign25980_e31093_d_n8;
        var_wgamma_dn9 = assign25980_e31093_d_n9;

        let (assign25990_e31111, assign25990_e31111_d_n6, assign25990_e31111_d_n7, assign25990_e31111_d_n8, assign25990_e31111_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) {
        let assign25990_e31105: f64 = (var_wsrh * var_wgamma);
        let assign25990_e31108: f64 = (var_wsrh + var_wgamma);
        let assign25990_e31109: f64 = (assign25990_e31105 / assign25990_e31108);
        (assign25990_e31109, ((((var_wsrh * var_wgamma_dn6) * assign25990_e31108) - (assign25990_e31105 * var_wgamma_dn6)) / (assign25990_e31108 * assign25990_e31108)), ((((var_wsrh * var_wgamma_dn7) * assign25990_e31108) - (assign25990_e31105 * var_wgamma_dn7)) / (assign25990_e31108 * assign25990_e31108)), ((((var_wsrh * var_wgamma_dn8) * assign25990_e31108) - (assign25990_e31105 * var_wgamma_dn8)) / (assign25990_e31108 * assign25990_e31108)), ((((var_wsrh * var_wgamma_dn9) * assign25990_e31108) - (assign25990_e31105 * var_wgamma_dn9)) / (assign25990_e31108 * assign25990_e31108)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign25990_e31111;
        var_wtat_dn6 = assign25990_e31111_d_n6;
        var_wtat_dn7 = assign25990_e31111_d_n7;
        var_wtat_dn8 = assign25990_e31111_d_n8;
        var_wtat_dn9 = assign25990_e31111_d_n9;

        let (assign26000_e31128, assign26000_e31128_d_n6, assign26000_e31128_d_n7, assign26000_e31128_d_n8, assign26000_e31128_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) {
        let assign26000_e31124: f64 = (var_btat / var_sqrtumax);
        let assign26000_e31125: f64 = (0.375 * assign26000_e31124);
        let assign26000_e31126: f64 = (assign26000_e31125).sqrt();
        (assign26000_e31126, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign26000_e31126)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign26000_e31126)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign26000_e31126)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign26000_e31126)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign26000_e31128;
        var_ktat_dn6 = assign26000_e31128_d_n6;
        var_ktat_dn7 = assign26000_e31128_d_n7;
        var_ktat_dn8 = assign26000_e31128_d_n8;
        var_ktat_dn9 = assign26000_e31128_d_n9;

        let (assign26010_e31146, assign26010_e31146_d_n6, assign26010_e31146_d_n7, assign26010_e31146_d_n8, assign26010_e31146_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) {
        let assign26010_e31141: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign26010_e31142: f64 = (2.0 * assign26010_e31141);
        let assign26010_e31144: f64 = (assign26010_e31142 - var_umax);
        (assign26010_e31144, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign26010_e31146;
        var_ltat_dn6 = assign26010_e31146_d_n6;
        var_ltat_dn7 = assign26010_e31146_d_n7;
        var_ltat_dn8 = assign26010_e31146_d_n8;
        var_ltat_dn9 = assign26010_e31146_d_n9;

        let (assign26020_e31172, assign26020_e31172_d_n6, assign26020_e31172_d_n7, assign26020_e31172_d_n8, assign26020_e31172_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) {
        let assign26020_e31158: f64 = (var_atatgat * var_twoatatoverthreebtat);
        let assign26020_e31160: f64 = (assign26020_e31158 * var_sqrtumax);
        let assign26020_e31163: f64 = (var_atatgat * var_umax);
        let assign26020_e31164: f64 = (assign26020_e31160 - assign26020_e31163);
        let assign26020_e31168: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign26020_e31169: f64 = (0.5 * assign26020_e31168);
        let assign26020_e31170: f64 = (assign26020_e31164 + assign26020_e31169);
        (assign26020_e31170, (((((var_atatgat * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign26020_e31158 * var_sqrtumax_dn6)) - (var_atatgat * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign26020_e31158 * var_sqrtumax_dn7)) - (var_atatgat * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign26020_e31158 * var_sqrtumax_dn8)) - (var_atatgat * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatgat * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign26020_e31158 * var_sqrtumax_dn9)) - (var_atatgat * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign26020_e31172;
        var_mtat_dn6 = assign26020_e31172_d_n6;
        var_mtat_dn7 = assign26020_e31172_d_n7;
        var_mtat_dn8 = assign26020_e31172_d_n8;
        var_mtat_dn9 = assign26020_e31172_d_n9;

        let (assign26030_e31188, assign26030_e31188_d_n6, assign26030_e31188_d_n7, assign26030_e31188_d_n8, assign26030_e31188_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) {
        let assign26030_e31184: f64 = (var_ltat - 1.0);
        let assign26030_e31186: f64 = (assign26030_e31184 * var_ktat);
        (assign26030_e31186, ((var_ltat_dn6 * var_ktat) + (assign26030_e31184 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign26030_e31184 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign26030_e31184 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign26030_e31184 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign26030_e31188;
        var_xerfc_dn6 = assign26030_e31188_d_n6;
        var_xerfc_dn7 = assign26030_e31188_d_n7;
        var_xerfc_dn8 = assign26030_e31188_d_n8;
        var_xerfc_dn9 = assign26030_e31188_d_n9;

        let (assign26040_e31202, assign26040_e31202_d_n6, assign26040_e31202_d_n7, assign26040_e31202_d_n8, assign26040_e31202_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) {
        let assign26040_e31200: f64 = (var_xerfc * var_xerfc);
        (assign26040_e31200, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign26040_e31202;
        var_ysq_dn6 = assign26040_e31202_d_n6;
        var_ysq_dn7 = assign26040_e31202_d_n7;
        var_ysq_dn8 = assign26040_e31202_d_n8;
        var_ysq_dn9 = assign26040_e31202_d_n9;

        let assign26050_e31205: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard510 = assign26050_e31205;

        let (assign26060_e31225, assign26060_e31225_d_n6, assign26060_e31225_d_n7, assign26060_e31225_d_n8, assign26060_e31225_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) && (var_guard510 != 0.0)) {
        let assign26060_e31221: f64 = (var_perfc * var_xerfc);
        let assign26060_e31222: f64 = (1.0 + assign26060_e31221);
        let assign26060_e31223: f64 = (1.0 / assign26060_e31222);
        (assign26060_e31223, (-((var_perfc * var_xerfc_dn6) / (assign26060_e31222 * assign26060_e31222))), (-((var_perfc * var_xerfc_dn7) / (assign26060_e31222 * assign26060_e31222))), (-((var_perfc * var_xerfc_dn8) / (assign26060_e31222 * assign26060_e31222))), (-((var_perfc * var_xerfc_dn9) / (assign26060_e31222 * assign26060_e31222))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign26060_e31225;
        var_terfc_dn6 = assign26060_e31225_d_n6;
        var_terfc_dn7 = assign26060_e31225_d_n7;
        var_terfc_dn8 = assign26060_e31225_d_n8;
        var_terfc_dn9 = assign26060_e31225_d_n9;

        let (assign26070_e31246, assign26070_e31246_d_n6, assign26070_e31246_d_n7, assign26070_e31246_d_n8, assign26070_e31246_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) && (var_guard510 == 0.0)) {
        let assign26070_e31242: f64 = (var_perfc * var_xerfc);
        let assign26070_e31243: f64 = (1.0 - assign26070_e31242);
        let assign26070_e31244: f64 = (1.0 / assign26070_e31243);
        (assign26070_e31244, (-((-(var_perfc * var_xerfc_dn6)) / (assign26070_e31243 * assign26070_e31243))), (-((-(var_perfc * var_xerfc_dn7)) / (assign26070_e31243 * assign26070_e31243))), (-((-(var_perfc * var_xerfc_dn8)) / (assign26070_e31243 * assign26070_e31243))), (-((-(var_perfc * var_xerfc_dn9)) / (assign26070_e31243 * assign26070_e31243))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign26070_e31246;
        var_terfc_dn6 = assign26070_e31246_d_n6;
        var_terfc_dn7 = assign26070_e31246_d_n7;
        var_terfc_dn8 = assign26070_e31246_d_n8;
        var_terfc_dn9 = assign26070_e31246_d_n9;

        let assign26080_e31248: f64 = (-var_ysq);
        let assign26080_e31250: f64 = (assign26080_e31248 + var_mtat);
        let assign26080_e31252: f64 = (-230.25850929940458);
        let assign26080_e31253: f64 = if assign26080_e31250 > assign26080_e31252 { 1.0 } else { 0.0 };
        var_guard511 = assign26080_e31253;

        let (assign26090_e31271, assign26090_e31271_d_n6, assign26090_e31271_d_n7, assign26090_e31271_d_n8, assign26090_e31271_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) && (var_guard511 != 0.0)) {
        let assign26090_e31266: f64 = (-var_ysq);
        let assign26090_e31268: f64 = (assign26090_e31266 + var_mtat);
        let assign26090_e31269: f64 = (assign26090_e31268).exp();
        (assign26090_e31269, (assign26090_e31269 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign26090_e31269 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign26090_e31269 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign26090_e31269 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign26090_e31271;
        var_tmp_dn6 = assign26090_e31271_d_n6;
        var_tmp_dn7 = assign26090_e31271_d_n7;
        var_tmp_dn8 = assign26090_e31271_d_n8;
        var_tmp_dn9 = assign26090_e31271_d_n9;

        let (assign26100_e31320, assign26100_e31320_d_n6, assign26100_e31320_d_n7, assign26100_e31320_d_n8, assign26100_e31320_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) && (var_guard511 == 0.0)) {
        let assign26100_e31287: f64 = (-230.25850929940458);
        let assign26100_e31289: f64 = (-var_ysq);
        let assign26100_e31291: f64 = (assign26100_e31289 + var_mtat);
        let assign26100_e31292: f64 = (assign26100_e31287 - assign26100_e31291);
        let assign26100_e31296: f64 = (-230.25850929940458);
        let assign26100_e31298: f64 = (-var_ysq);
        let assign26100_e31300: f64 = (assign26100_e31298 + var_mtat);
        let assign26100_e31301: f64 = (assign26100_e31296 - assign26100_e31300);
        let assign26100_e31304: f64 = (-230.25850929940458);
        let assign26100_e31306: f64 = (-var_ysq);
        let assign26100_e31308: f64 = (assign26100_e31306 + var_mtat);
        let assign26100_e31309: f64 = (assign26100_e31304 - assign26100_e31308);
        let assign26100_e31311: f64 = (assign26100_e31309 * 0.3333333333333333);
        let assign26100_e31312: f64 = (1.0 + assign26100_e31311);
        let assign26100_e31313: f64 = (assign26100_e31301 * assign26100_e31312);
        let assign26100_e31314: f64 = (0.5 * assign26100_e31313);
        let assign26100_e31315: f64 = (1.0 + assign26100_e31314);
        let assign26100_e31316: f64 = (assign26100_e31292 * assign26100_e31315);
        let assign26100_e31317: f64 = (1.0 + assign26100_e31316);
        let assign26100_e31318: f64 = (1e-100 / assign26100_e31317);
        (assign26100_e31318, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign26100_e31315) + (assign26100_e31292 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign26100_e31312) + (assign26100_e31301 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign26100_e31317 * assign26100_e31317))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign26100_e31315) + (assign26100_e31292 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign26100_e31312) + (assign26100_e31301 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign26100_e31317 * assign26100_e31317))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign26100_e31315) + (assign26100_e31292 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign26100_e31312) + (assign26100_e31301 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign26100_e31317 * assign26100_e31317))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign26100_e31315) + (assign26100_e31292 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign26100_e31312) + (assign26100_e31301 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign26100_e31317 * assign26100_e31317))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign26100_e31320;
        var_tmp_dn6 = assign26100_e31320_d_n6;
        var_tmp_dn7 = assign26100_e31320_d_n7;
        var_tmp_dn8 = assign26100_e31320_d_n8;
        var_tmp_dn9 = assign26100_e31320_d_n9;

        let (assign26110_e31350, assign26110_e31350_d_n6, assign26110_e31350_d_n7, assign26110_e31350_d_n8, assign26110_e31350_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) {
        let assign26110_e31332: f64 = (0.29214664 * var_terfc);
        let assign26110_e31336: f64 = (var_terfc * var_terfc);
        let assign26110_e31337: f64 = (var_berfc * assign26110_e31336);
        let assign26110_e31338: f64 = (assign26110_e31332 + assign26110_e31337);
        let assign26110_e31342: f64 = (var_terfc * var_terfc);
        let assign26110_e31344: f64 = (assign26110_e31342 * var_terfc);
        let assign26110_e31345: f64 = (var_cerfc * assign26110_e31344);
        let assign26110_e31346: f64 = (assign26110_e31338 + assign26110_e31345);
        let assign26110_e31348: f64 = (assign26110_e31346 * var_tmp);
        (assign26110_e31348, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign26110_e31342 * var_terfc_dn6)))) * var_tmp) + (assign26110_e31346 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign26110_e31342 * var_terfc_dn7)))) * var_tmp) + (assign26110_e31346 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign26110_e31342 * var_terfc_dn8)))) * var_tmp) + (assign26110_e31346 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign26110_e31342 * var_terfc_dn9)))) * var_tmp) + (assign26110_e31346 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign26110_e31350;
        var_erfcpos_dn6 = assign26110_e31350_d_n6;
        var_erfcpos_dn7 = assign26110_e31350_d_n7;
        var_erfcpos_dn8 = assign26110_e31350_d_n8;
        var_erfcpos_dn9 = assign26110_e31350_d_n9;

        let assign26120_e31353: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard512 = assign26120_e31353;

        let (assign26130_e31367, assign26130_e31367_d_n6, assign26130_e31367_d_n7, assign26130_e31367_d_n8, assign26130_e31367_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) && (var_guard512 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign26130_e31367;
        var_erfctimesexpmtat_dn6 = assign26130_e31367_d_n6;
        var_erfctimesexpmtat_dn7 = assign26130_e31367_d_n7;
        var_erfctimesexpmtat_dn8 = assign26130_e31367_d_n8;
        var_erfctimesexpmtat_dn9 = assign26130_e31367_d_n9;

        let assign26140_e31370: f64 = (-230.25850929940458);
        let assign26140_e31371: f64 = if var_mtat > assign26140_e31370 { 1.0 } else { 0.0 };
        var_guard513 = assign26140_e31371;

        let (assign26150_e31389, assign26150_e31389_d_n6, assign26150_e31389_d_n7, assign26150_e31389_d_n8, assign26150_e31389_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) && (var_guard512 == 0.0)) && (var_guard513 != 0.0)) {
        let assign26150_e31387: f64 = (var_mtat).exp();
        (assign26150_e31387, (assign26150_e31387 * var_mtat_dn6), (assign26150_e31387 * var_mtat_dn7), (assign26150_e31387 * var_mtat_dn8), (assign26150_e31387 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign26150_e31389;
        var_tmp_dn6 = assign26150_e31389_d_n6;
        var_tmp_dn7 = assign26150_e31389_d_n7;
        var_tmp_dn8 = assign26150_e31389_d_n8;
        var_tmp_dn9 = assign26150_e31389_d_n9;

        let (assign26160_e31432, assign26160_e31432_d_n6, assign26160_e31432_d_n7, assign26160_e31432_d_n8, assign26160_e31432_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) && (var_guard512 == 0.0)) && (var_guard513 == 0.0)) {
        let assign26160_e31408: f64 = (-230.25850929940458);
        let assign26160_e31410: f64 = (assign26160_e31408 - var_mtat);
        let assign26160_e31414: f64 = (-230.25850929940458);
        let assign26160_e31416: f64 = (assign26160_e31414 - var_mtat);
        let assign26160_e31419: f64 = (-230.25850929940458);
        let assign26160_e31421: f64 = (assign26160_e31419 - var_mtat);
        let assign26160_e31423: f64 = (assign26160_e31421 * 0.3333333333333333);
        let assign26160_e31424: f64 = (1.0 + assign26160_e31423);
        let assign26160_e31425: f64 = (assign26160_e31416 * assign26160_e31424);
        let assign26160_e31426: f64 = (0.5 * assign26160_e31425);
        let assign26160_e31427: f64 = (1.0 + assign26160_e31426);
        let assign26160_e31428: f64 = (assign26160_e31410 * assign26160_e31427);
        let assign26160_e31429: f64 = (1.0 + assign26160_e31428);
        let assign26160_e31430: f64 = (1e-100 / assign26160_e31429);
        (assign26160_e31430, (-((1e-100 * (((-var_mtat_dn6) * assign26160_e31427) + (assign26160_e31410 * (0.5 * (((-var_mtat_dn6) * assign26160_e31424) + (assign26160_e31416 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign26160_e31429 * assign26160_e31429))), (-((1e-100 * (((-var_mtat_dn7) * assign26160_e31427) + (assign26160_e31410 * (0.5 * (((-var_mtat_dn7) * assign26160_e31424) + (assign26160_e31416 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign26160_e31429 * assign26160_e31429))), (-((1e-100 * (((-var_mtat_dn8) * assign26160_e31427) + (assign26160_e31410 * (0.5 * (((-var_mtat_dn8) * assign26160_e31424) + (assign26160_e31416 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign26160_e31429 * assign26160_e31429))), (-((1e-100 * (((-var_mtat_dn9) * assign26160_e31427) + (assign26160_e31410 * (0.5 * (((-var_mtat_dn9) * assign26160_e31424) + (assign26160_e31416 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign26160_e31429 * assign26160_e31429))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign26160_e31432;
        var_tmp_dn6 = assign26160_e31432_d_n6;
        var_tmp_dn7 = assign26160_e31432_d_n7;
        var_tmp_dn8 = assign26160_e31432_d_n8;
        var_tmp_dn9 = assign26160_e31432_d_n9;

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
        *var_guard507_slot = var_guard507;
        *var_guard508_slot = var_guard508;
        *var_guard509_slot = var_guard509;
        *var_guard510_slot = var_guard510;
        *var_guard511_slot = var_guard511;
        *var_guard512_slot = var_guard512;
        *var_guard513_slot = var_guard513;
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

    pub(super) fn stamp_transient_block_51(
        p: &Parameters,
        var_absource_i: f64,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatgat: f64,
        var_erfcpos: f64,
        var_erfcpos_dn6: f64,
        var_erfcpos_dn7: f64,
        var_erfcpos_dn8: f64,
        var_erfcpos_dn9: f64,
        var_fbbtgat: f64,
        var_fbbtgat_dn6: f64,
        var_fbbtgat_dn7: f64,
        var_fbbtgat_dn8: f64,
        var_fbbtgat_dn9: f64,
        var_fstopgat: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard504: f64,
        var_guard508: f64,
        var_guard512: f64,
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
        var_idsatbot: f64,
        var_idsatgat: f64,
        var_idsatsti: f64,
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
        var_lgsource_i: f64,
        var_lssource_i: f64,
        var_mfor1_s: f64,
        var_one_over_one_minus_pgat: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_slopegat: f64,
        var_slopegat_dn6: f64,
        var_slopegat_dn7: f64,
        var_slopegat_dn8: f64,
        var_slopegat_dn9: f64,
        var_v1: f64,
        var_v4: f64,
        var_v5: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirgatinv: f64,
        var_vbrinvgat: f64,
        var_vbrinvgat_dn6: f64,
        var_vbrinvgat_dn7: f64,
        var_vbrinvgat_dn8: f64,
        var_vbrinvgat_dn9: f64,
        var_wdepnulrinvgat: f64,
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
        var_guard514_slot: &mut f64,
        var_guard515_slot: &mut f64,
        var_guard516_slot: &mut f64,
        var_guard517_slot: &mut f64,
        var_guard518_slot: &mut f64,
        var_guard519_slot: &mut f64,
        var_guard520_slot: &mut f64,
        var_guard521_slot: &mut f64,
        var_guard522_slot: &mut f64,
        var_guard523_slot: &mut f64,
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
        var_isatfor1_s_slot: &mut f64,
        var_isatfor2_s_slot: &mut f64,
        var_isatfor2_s_dn6_slot: &mut f64,
        var_isatfor2_s_dn7_slot: &mut f64,
        var_isatfor2_s_dn8_slot: &mut f64,
        var_isatfor2_s_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_mfor2_s_slot: &mut f64,
        var_mfor2_s_dn6_slot: &mut f64,
        var_mfor2_s_dn7_slot: &mut f64,
        var_mfor2_s_dn8_slot: &mut f64,
        var_mfor2_s_dn9_slot: &mut f64,
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
        let mut var_guard514: f64 = *var_guard514_slot;
        let mut var_guard515: f64 = *var_guard515_slot;
        let mut var_guard516: f64 = *var_guard516_slot;
        let mut var_guard517: f64 = *var_guard517_slot;
        let mut var_guard518: f64 = *var_guard518_slot;
        let mut var_guard519: f64 = *var_guard519_slot;
        let mut var_guard520: f64 = *var_guard520_slot;
        let mut var_guard521: f64 = *var_guard521_slot;
        let mut var_guard522: f64 = *var_guard522_slot;
        let mut var_guard523: f64 = *var_guard523_slot;
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
        let mut var_isatfor1_s: f64 = *var_isatfor1_s_slot;
        let mut var_isatfor2_s: f64 = *var_isatfor2_s_slot;
        let mut var_isatfor2_s_dn6: f64 = *var_isatfor2_s_dn6_slot;
        let mut var_isatfor2_s_dn7: f64 = *var_isatfor2_s_dn7_slot;
        let mut var_isatfor2_s_dn8: f64 = *var_isatfor2_s_dn8_slot;
        let mut var_isatfor2_s_dn9: f64 = *var_isatfor2_s_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_mfor2_s: f64 = *var_mfor2_s_slot;
        let mut var_mfor2_s_dn6: f64 = *var_mfor2_s_dn6_slot;
        let mut var_mfor2_s_dn7: f64 = *var_mfor2_s_dn7_slot;
        let mut var_mfor2_s_dn8: f64 = *var_mfor2_s_dn8_slot;
        let mut var_mfor2_s_dn9: f64 = *var_mfor2_s_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;

        let (assign26170_e31451, assign26170_e31451_d_n6, assign26170_e31451_d_n7, assign26170_e31451_d_n8, assign26170_e31451_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) && (var_guard512 == 0.0)) {
        let assign26170_e31447: f64 = (2.0 * var_tmp);
        let assign26170_e31449: f64 = (assign26170_e31447 - var_erfcpos);
        (assign26170_e31449, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign26170_e31451;
        var_erfctimesexpmtat_dn6 = assign26170_e31451_d_n6;
        var_erfctimesexpmtat_dn7 = assign26170_e31451_d_n7;
        var_erfctimesexpmtat_dn8 = assign26170_e31451_d_n8;
        var_erfctimesexpmtat_dn9 = assign26170_e31451_d_n9;

        let (assign26180_e31471, assign26180_e31471_d_n6, assign26180_e31471_d_n7, assign26180_e31471_d_n8, assign26180_e31471_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) {
        let assign26180_e31463: f64 = (1.772453850905516 * 0.5);
        let assign26180_e31466: f64 = (var_atatgat * var_erfctimesexpmtat);
        let assign26180_e31468: f64 = (assign26180_e31466 / var_ktat);
        let assign26180_e31469: f64 = (assign26180_e31463 * assign26180_e31468);
        (assign26180_e31469, (assign26180_e31463 * ((((var_atatgat * var_erfctimesexpmtat_dn6) * var_ktat) - (assign26180_e31466 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign26180_e31463 * ((((var_atatgat * var_erfctimesexpmtat_dn7) * var_ktat) - (assign26180_e31466 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign26180_e31463 * ((((var_atatgat * var_erfctimesexpmtat_dn8) * var_ktat) - (assign26180_e31466 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign26180_e31463 * ((((var_atatgat * var_erfctimesexpmtat_dn9) * var_ktat) - (assign26180_e31466 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign26180_e31471;
        var_gammamax_dn6 = assign26180_e31471_d_n6;
        var_gammamax_dn7 = assign26180_e31471_d_n7;
        var_gammamax_dn8 = assign26180_e31471_d_n8;
        var_gammamax_dn9 = assign26180_e31471_d_n9;

        let (assign26190_e31489, assign26190_e31489_d_n6, assign26190_e31489_d_n7, assign26190_e31489_d_n8, assign26190_e31489_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard508 == 0.0)) {
        let assign26190_e31484: f64 = (var_asrh * var_gammamax);
        let assign26190_e31486: f64 = (assign26190_e31484 * var_wtat);
        let assign26190_e31487: f64 = (p.p864 * assign26190_e31486);
        (assign26190_e31487, (p.p864 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign26190_e31484 * var_wtat_dn6))), (p.p864 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign26190_e31484 * var_wtat_dn7))), (p.p864 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign26190_e31484 * var_wtat_dn8))), (p.p864 * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign26190_e31484 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign26190_e31489;
        var_itat_dn6 = assign26190_e31489_d_n6;
        var_itat_dn7 = assign26190_e31489_d_n7;
        var_itat_dn8 = assign26190_e31489_d_n8;
        var_itat_dn9 = assign26190_e31489_d_n9;

        let assign26200_e31492: f64 = if p.p870 == 0.0 { 1.0 } else { 0.0 };
        var_guard514 = assign26200_e31492;

        let (assign26210_e31503, assign26210_e31503_d_n6, assign26210_e31503_d_n7, assign26210_e31503_d_n8, assign26210_e31503_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard514 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign26210_e31503;
        var_ibbt_dn6 = assign26210_e31503_d_n6;
        var_ibbt_dn7 = assign26210_e31503_d_n7;
        var_ibbt_dn8 = assign26210_e31503_d_n8;
        var_ibbt_dn9 = assign26210_e31503_d_n9;

        let assign26220_e31506: f64 = if p.p850 == 0.5 { 1.0 } else { 0.0 };
        var_guard515 = assign26220_e31506;

        let (assign26230_e31525, assign26230_e31525_d_n6, assign26230_e31525_d_n7, assign26230_e31525_d_n8, assign26230_e31525_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard514 == 0.0)) && (var_guard515 != 0.0)) {
        let assign26230_e31520: f64 = (p.p847 - var_vbbt);
        let assign26230_e31522: f64 = (assign26230_e31520 * var_vbirgatinv);
        let assign26230_e31523: f64 = (assign26230_e31522).sqrt();
        (assign26230_e31523, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign26230_e31525;
        var_tmp_dn6 = assign26230_e31525_d_n6;
        var_tmp_dn7 = assign26230_e31525_d_n7;
        var_tmp_dn8 = assign26230_e31525_d_n8;
        var_tmp_dn9 = assign26230_e31525_d_n9;

        let (assign26240_e31546, assign26240_e31546_d_n6, assign26240_e31546_d_n7, assign26240_e31546_d_n8, assign26240_e31546_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard514 == 0.0)) && (var_guard515 == 0.0)) {
        let assign26240_e31540: f64 = (p.p847 - var_vbbt);
        let assign26240_e31542: f64 = (assign26240_e31540 * var_vbirgatinv);
        let assign26240_e31544: f64 = (assign26240_e31542).powf(p.p850);
        (assign26240_e31544, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign26240_e31546;
        var_tmp_dn6 = assign26240_e31546_d_n6;
        var_tmp_dn7 = assign26240_e31546_d_n7;
        var_tmp_dn8 = assign26240_e31546_d_n8;
        var_tmp_dn9 = assign26240_e31546_d_n9;

        let (assign26250_e31566, assign26250_e31566_d_n6, assign26250_e31566_d_n7, assign26250_e31566_d_n8, assign26250_e31566_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard514 == 0.0)) {
        let assign26250_e31559: f64 = (p.p847 - var_vbbt);
        let assign26250_e31561: f64 = (assign26250_e31559 * var_wdepnulrinvgat);
        let assign26250_e31563: f64 = (assign26250_e31561 / var_tmp);
        let assign26250_e31564: f64 = (var_one_over_one_minus_pgat * assign26250_e31563);
        (assign26250_e31564, (var_one_over_one_minus_pgat * (-((assign26250_e31561 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign26250_e31561 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign26250_e31561 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign26250_e31561 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign26250_e31566;
        var_fmaxr_dn6 = assign26250_e31566_d_n6;
        var_fmaxr_dn7 = assign26250_e31566_d_n7;
        var_fmaxr_dn8 = assign26250_e31566_d_n8;
        var_fmaxr_dn9 = assign26250_e31566_d_n9;

        let assign26260_e31568: f64 = (-var_fbbtgat);
        let assign26260_e31570: f64 = (assign26260_e31568 / var_fmaxr);
        let assign26260_e31571: f64 = (assign26260_e31570).abs();
        let assign26260_e31573: f64 = if assign26260_e31571 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard516 = assign26260_e31573;

        let (assign26270_e31591, assign26270_e31591_d_n6, assign26270_e31591_d_n7, assign26270_e31591_d_n8, assign26270_e31591_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard514 == 0.0)) && (var_guard516 != 0.0)) {
        let assign26270_e31586: f64 = (-var_fbbtgat);
        let assign26270_e31588: f64 = (assign26270_e31586 / var_fmaxr);
        let assign26270_e31589: f64 = (assign26270_e31588).exp();
        (assign26270_e31589, (assign26270_e31589 * ((((-var_fbbtgat_dn6) * var_fmaxr) - (assign26270_e31586 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign26270_e31589 * ((((-var_fbbtgat_dn7) * var_fmaxr) - (assign26270_e31586 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign26270_e31589 * ((((-var_fbbtgat_dn8) * var_fmaxr) - (assign26270_e31586 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))), (assign26270_e31589 * ((((-var_fbbtgat_dn9) * var_fmaxr) - (assign26270_e31586 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign26270_e31591;
        var_tmp_dn6 = assign26270_e31591_d_n6;
        var_tmp_dn7 = assign26270_e31591_d_n7;
        var_tmp_dn8 = assign26270_e31591_d_n8;
        var_tmp_dn9 = assign26270_e31591_d_n9;

        let assign26280_e31593: f64 = (-var_fbbtgat);
        let assign26280_e31595: f64 = (assign26280_e31593 / var_fmaxr);
        let assign26280_e31597: f64 = if assign26280_e31595 < 0.0 { 1.0 } else { 0.0 };
        var_guard517 = assign26280_e31597;

        let (assign26290_e31648, assign26290_e31648_d_n6, assign26290_e31648_d_n7, assign26290_e31648_d_n8, assign26290_e31648_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard514 == 0.0)) && (var_guard516 == 0.0)) && (var_guard517 != 0.0)) {
        let assign26290_e31615: f64 = (-230.25850929940458);
        let assign26290_e31617: f64 = (-var_fbbtgat);
        let assign26290_e31619: f64 = (assign26290_e31617 / var_fmaxr);
        let assign26290_e31620: f64 = (assign26290_e31615 - assign26290_e31619);
        let assign26290_e31624: f64 = (-230.25850929940458);
        let assign26290_e31626: f64 = (-var_fbbtgat);
        let assign26290_e31628: f64 = (assign26290_e31626 / var_fmaxr);
        let assign26290_e31629: f64 = (assign26290_e31624 - assign26290_e31628);
        let assign26290_e31632: f64 = (-230.25850929940458);
        let assign26290_e31634: f64 = (-var_fbbtgat);
        let assign26290_e31636: f64 = (assign26290_e31634 / var_fmaxr);
        let assign26290_e31637: f64 = (assign26290_e31632 - assign26290_e31636);
        let assign26290_e31639: f64 = (assign26290_e31637 * 0.3333333333333333);
        let assign26290_e31640: f64 = (1.0 + assign26290_e31639);
        let assign26290_e31641: f64 = (assign26290_e31629 * assign26290_e31640);
        let assign26290_e31642: f64 = (0.5 * assign26290_e31641);
        let assign26290_e31643: f64 = (1.0 + assign26290_e31642);
        let assign26290_e31644: f64 = (assign26290_e31620 * assign26290_e31643);
        let assign26290_e31645: f64 = (1.0 + assign26290_e31644);
        let assign26290_e31646: f64 = (1e-100 / assign26290_e31645);
        (assign26290_e31646, (-((1e-100 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign26290_e31617 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign26290_e31643) + (assign26290_e31620 * (0.5 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign26290_e31626 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign26290_e31640) + (assign26290_e31629 * ((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign26290_e31634 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign26290_e31645 * assign26290_e31645))), (-((1e-100 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign26290_e31617 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign26290_e31643) + (assign26290_e31620 * (0.5 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign26290_e31626 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign26290_e31640) + (assign26290_e31629 * ((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign26290_e31634 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign26290_e31645 * assign26290_e31645))), (-((1e-100 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign26290_e31617 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign26290_e31643) + (assign26290_e31620 * (0.5 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign26290_e31626 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign26290_e31640) + (assign26290_e31629 * ((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign26290_e31634 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign26290_e31645 * assign26290_e31645))), (-((1e-100 * (((-((((-var_fbbtgat_dn9) * var_fmaxr) - (assign26290_e31617 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign26290_e31643) + (assign26290_e31620 * (0.5 * (((-((((-var_fbbtgat_dn9) * var_fmaxr) - (assign26290_e31626 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign26290_e31640) + (assign26290_e31629 * ((-((((-var_fbbtgat_dn9) * var_fmaxr) - (assign26290_e31634 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign26290_e31645 * assign26290_e31645))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign26290_e31648;
        var_tmp_dn6 = assign26290_e31648_d_n6;
        var_tmp_dn7 = assign26290_e31648_d_n7;
        var_tmp_dn8 = assign26290_e31648_d_n8;
        var_tmp_dn9 = assign26290_e31648_d_n9;

        let (assign26300_e31697, assign26300_e31697_d_n6, assign26300_e31697_d_n7, assign26300_e31697_d_n8, assign26300_e31697_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard514 == 0.0)) && (var_guard516 == 0.0)) && (var_guard517 == 0.0)) {
        let assign26300_e31667: f64 = (-var_fbbtgat);
        let assign26300_e31669: f64 = (assign26300_e31667 / var_fmaxr);
        let assign26300_e31671: f64 = (assign26300_e31669 - 230.25850929940458);
        let assign26300_e31675: f64 = (-var_fbbtgat);
        let assign26300_e31677: f64 = (assign26300_e31675 / var_fmaxr);
        let assign26300_e31679: f64 = (assign26300_e31677 - 230.25850929940458);
        let assign26300_e31682: f64 = (-var_fbbtgat);
        let assign26300_e31684: f64 = (assign26300_e31682 / var_fmaxr);
        let assign26300_e31686: f64 = (assign26300_e31684 - 230.25850929940458);
        let assign26300_e31688: f64 = (assign26300_e31686 * 0.3333333333333333);
        let assign26300_e31689: f64 = (1.0 + assign26300_e31688);
        let assign26300_e31690: f64 = (assign26300_e31679 * assign26300_e31689);
        let assign26300_e31691: f64 = (0.5 * assign26300_e31690);
        let assign26300_e31692: f64 = (1.0 + assign26300_e31691);
        let assign26300_e31693: f64 = (assign26300_e31671 * assign26300_e31692);
        let assign26300_e31694: f64 = (1.0 + assign26300_e31693);
        let assign26300_e31695: f64 = (1e100 * assign26300_e31694);
        (assign26300_e31695, (1e100 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign26300_e31667 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign26300_e31692) + (assign26300_e31671 * (0.5 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign26300_e31675 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign26300_e31689) + (assign26300_e31679 * (((((-var_fbbtgat_dn6) * var_fmaxr) - (assign26300_e31682 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign26300_e31667 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign26300_e31692) + (assign26300_e31671 * (0.5 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign26300_e31675 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign26300_e31689) + (assign26300_e31679 * (((((-var_fbbtgat_dn7) * var_fmaxr) - (assign26300_e31682 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign26300_e31667 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign26300_e31692) + (assign26300_e31671 * (0.5 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign26300_e31675 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign26300_e31689) + (assign26300_e31679 * (((((-var_fbbtgat_dn8) * var_fmaxr) - (assign26300_e31682 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn9) * var_fmaxr) - (assign26300_e31667 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign26300_e31692) + (assign26300_e31671 * (0.5 * ((((((-var_fbbtgat_dn9) * var_fmaxr) - (assign26300_e31675 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign26300_e31689) + (assign26300_e31679 * (((((-var_fbbtgat_dn9) * var_fmaxr) - (assign26300_e31682 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign26300_e31697;
        var_tmp_dn6 = assign26300_e31697_d_n6;
        var_tmp_dn7 = assign26300_e31697_d_n7;
        var_tmp_dn8 = assign26300_e31697_d_n8;
        var_tmp_dn9 = assign26300_e31697_d_n9;

        let (assign26310_e31717, assign26310_e31717_d_n6, assign26310_e31717_d_n7, assign26310_e31717_d_n8, assign26310_e31717_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard514 == 0.0)) {
        let assign26310_e31710: f64 = (var_v5 * var_fmaxr);
        let assign26310_e31712: f64 = (assign26310_e31710 * var_fmaxr);
        let assign26310_e31714: f64 = (assign26310_e31712 * var_tmp);
        let assign26310_e31715: f64 = (p.p870 * assign26310_e31714);
        (assign26310_e31715, (p.p870 * (((((var_v5 * var_fmaxr_dn6) * var_fmaxr) + (assign26310_e31710 * var_fmaxr_dn6)) * var_tmp) + (assign26310_e31712 * var_tmp_dn6))), (p.p870 * (((((var_v5 * var_fmaxr_dn7) * var_fmaxr) + (assign26310_e31710 * var_fmaxr_dn7)) * var_tmp) + (assign26310_e31712 * var_tmp_dn7))), (p.p870 * (((((var_v5 * var_fmaxr_dn8) * var_fmaxr) + (assign26310_e31710 * var_fmaxr_dn8)) * var_tmp) + (assign26310_e31712 * var_tmp_dn8))), (p.p870 * (((((var_v5 * var_fmaxr_dn9) * var_fmaxr) + (assign26310_e31710 * var_fmaxr_dn9)) * var_tmp) + (assign26310_e31712 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign26310_e31717;
        var_ibbt_dn6 = assign26310_e31717_d_n6;
        var_ibbt_dn7 = assign26310_e31717_d_n7;
        var_ibbt_dn8 = assign26310_e31717_d_n8;
        var_ibbt_dn9 = assign26310_e31717_d_n9;

        let assign26320_e31720: f64 = if p.p879 > 1000.0 { 1.0 } else { 0.0 };
        var_guard518 = assign26320_e31720;

        let (assign26330_e31731, assign26330_e31731_d_n6, assign26330_e31731_d_n7, assign26330_e31731_d_n8, assign26330_e31731_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard518 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign26330_e31731;
        var_fbreakdown_dn6 = assign26330_e31731_d_n6;
        var_fbreakdown_dn7 = assign26330_e31731_d_n7;
        var_fbreakdown_dn8 = assign26330_e31731_d_n8;
        var_fbreakdown_dn9 = assign26330_e31731_d_n9;

        let assign26340_e31734: f64 = (-var_alphaav);
        let assign26340_e31736: f64 = (assign26340_e31734 * p.p879);
        let assign26340_e31737: f64 = if var_vav > assign26340_e31736 { 1.0 } else { 0.0 };
        var_guard519 = assign26340_e31737;

        let assign26350_e31740: f64 = if p.p882 == 4.0 { 1.0 } else { 0.0 };
        var_guard520 = assign26350_e31740;

        let (assign26360_e31770, assign26360_e31770_d_n6, assign26360_e31770_d_n7, assign26360_e31770_d_n8, assign26360_e31770_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard518 == 0.0)) && (var_guard519 != 0.0)) && (var_guard520 != 0.0)) {
        let assign26360_e31756: f64 = (var_vav * var_vbrinvgat);
        let assign26360_e31759: f64 = (var_vav * var_vbrinvgat);
        let assign26360_e31760: f64 = (assign26360_e31756 * assign26360_e31759);
        let assign26360_e31763: f64 = (var_vav * var_vbrinvgat);
        let assign26360_e31764: f64 = (assign26360_e31760 * assign26360_e31763);
        let assign26360_e31767: f64 = (var_vav * var_vbrinvgat);
        let assign26360_e31768: f64 = (assign26360_e31764 * assign26360_e31767);
        (assign26360_e31768, (((((((var_vav * var_vbrinvgat_dn6) * assign26360_e31759) + (assign26360_e31756 * (var_vav * var_vbrinvgat_dn6))) * assign26360_e31763) + (assign26360_e31760 * (var_vav * var_vbrinvgat_dn6))) * assign26360_e31767) + (assign26360_e31764 * (var_vav * var_vbrinvgat_dn6))), (((((((var_vav * var_vbrinvgat_dn7) * assign26360_e31759) + (assign26360_e31756 * (var_vav * var_vbrinvgat_dn7))) * assign26360_e31763) + (assign26360_e31760 * (var_vav * var_vbrinvgat_dn7))) * assign26360_e31767) + (assign26360_e31764 * (var_vav * var_vbrinvgat_dn7))), (((((((var_vav * var_vbrinvgat_dn8) * assign26360_e31759) + (assign26360_e31756 * (var_vav * var_vbrinvgat_dn8))) * assign26360_e31763) + (assign26360_e31760 * (var_vav * var_vbrinvgat_dn8))) * assign26360_e31767) + (assign26360_e31764 * (var_vav * var_vbrinvgat_dn8))), (((((((var_vav * var_vbrinvgat_dn9) * assign26360_e31759) + (assign26360_e31756 * (var_vav * var_vbrinvgat_dn9))) * assign26360_e31763) + (assign26360_e31760 * (var_vav * var_vbrinvgat_dn9))) * assign26360_e31767) + (assign26360_e31764 * (var_vav * var_vbrinvgat_dn9))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign26360_e31770;
        var_tmp_dn6 = assign26360_e31770_d_n6;
        var_tmp_dn7 = assign26360_e31770_d_n7;
        var_tmp_dn8 = assign26360_e31770_d_n8;
        var_tmp_dn9 = assign26360_e31770_d_n9;

        let (assign26370_e31792, assign26370_e31792_d_n6, assign26370_e31792_d_n7, assign26370_e31792_d_n8, assign26370_e31792_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard518 == 0.0)) && (var_guard519 != 0.0)) && (var_guard520 == 0.0)) {
        let assign26370_e31787: f64 = (var_vav * var_vbrinvgat);
        let assign26370_e31788: f64 = (assign26370_e31787).abs();
        let assign26370_e31790: f64 = (assign26370_e31788).powf(p.p882);
        (assign26370_e31790, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign26370_e31788).powf(p.p882 - 1.0) * if assign26370_e31787 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) })) } } else { (assign26370_e31790 * (p.p882 * (if assign26370_e31787 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) } / assign26370_e31788))) }, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign26370_e31788).powf(p.p882 - 1.0) * if assign26370_e31787 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) })) } } else { (assign26370_e31790 * (p.p882 * (if assign26370_e31787 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) } / assign26370_e31788))) }, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign26370_e31788).powf(p.p882 - 1.0) * if assign26370_e31787 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) })) } } else { (assign26370_e31790 * (p.p882 * (if assign26370_e31787 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) } / assign26370_e31788))) }, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign26370_e31788).powf(p.p882 - 1.0) * if assign26370_e31787 >= 0.0 { (var_vav * var_vbrinvgat_dn9) } else { (-(var_vav * var_vbrinvgat_dn9)) })) } } else { (assign26370_e31790 * (p.p882 * (if assign26370_e31787 >= 0.0 { (var_vav * var_vbrinvgat_dn9) } else { (-(var_vav * var_vbrinvgat_dn9)) } / assign26370_e31788))) },)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign26370_e31792;
        var_tmp_dn6 = assign26370_e31792_d_n6;
        var_tmp_dn7 = assign26370_e31792_d_n7;
        var_tmp_dn8 = assign26370_e31792_d_n8;
        var_tmp_dn9 = assign26370_e31792_d_n9;

        let (assign26380_e31810, assign26380_e31810_d_n6, assign26380_e31810_d_n7, assign26380_e31810_d_n8, assign26380_e31810_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard518 == 0.0)) && (var_guard519 != 0.0)) {
        let assign26380_e31807: f64 = (1.0 - var_tmp);
        let assign26380_e31808: f64 = (1.0 / assign26380_e31807);
        (assign26380_e31808, (-((-var_tmp_dn6) / (assign26380_e31807 * assign26380_e31807))), (-((-var_tmp_dn7) / (assign26380_e31807 * assign26380_e31807))), (-((-var_tmp_dn8) / (assign26380_e31807 * assign26380_e31807))), (-((-var_tmp_dn9) / (assign26380_e31807 * assign26380_e31807))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign26380_e31810;
        var_fbreakdown_dn6 = assign26380_e31810_d_n6;
        var_fbreakdown_dn7 = assign26380_e31810_d_n7;
        var_fbreakdown_dn8 = assign26380_e31810_d_n8;
        var_fbreakdown_dn9 = assign26380_e31810_d_n9;

        let (assign26390_e31833, assign26390_e31833_d_n6, assign26390_e31833_d_n7, assign26390_e31833_d_n8, assign26390_e31833_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) && (var_guard518 == 0.0)) && (var_guard519 == 0.0)) {
        let assign26390_e31827: f64 = (var_alphaav * p.p879);
        let assign26390_e31828: f64 = (var_vav + assign26390_e31827);
        let assign26390_e31830: f64 = (assign26390_e31828 * var_slopegat);
        let assign26390_e31831: f64 = (var_fstopgat + assign26390_e31830);
        (assign26390_e31831, (assign26390_e31828 * var_slopegat_dn6), (assign26390_e31828 * var_slopegat_dn7), (assign26390_e31828 * var_slopegat_dn8), (assign26390_e31828 * var_slopegat_dn9),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign26390_e31833;
        var_fbreakdown_dn6 = assign26390_e31833_d_n6;
        var_fbreakdown_dn7 = assign26390_e31833_d_n7;
        var_fbreakdown_dn8 = assign26390_e31833_d_n8;
        var_fbreakdown_dn9 = assign26390_e31833_d_n9;

        let (assign26400_e31852, assign26400_e31852_d_n6, assign26400_e31852_d_n7, assign26400_e31852_d_n8, assign26400_e31852_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard504 == 0.0)) {
        let assign26400_e31843: f64 = (var_id__blk212 + var_isrh);
        let assign26400_e31845: f64 = (assign26400_e31843 + var_itat);
        let assign26400_e31847: f64 = (assign26400_e31845 + var_ibbt);
        let assign26400_e31848: f64 = (p.p29 * assign26400_e31847);
        let assign26400_e31850: f64 = (assign26400_e31848 * var_fbreakdown);
        (assign26400_e31850, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign26400_e31848 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign26400_e31848 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign26400_e31848 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign26400_e31848 * var_fbreakdown_dn9)),)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign26400_e31852;
        var_ijungat_dn6 = assign26400_e31852_d_n6;
        var_ijungat_dn7 = assign26400_e31852_d_n7;
        var_ijungat_dn8 = assign26400_e31852_d_n8;
        var_ijungat_dn9 = assign26400_e31852_d_n9;

        let (assign26410_e31868, assign26410_e31868_d_n6, assign26410_e31868_d_n7, assign26410_e31868_d_n8, assign26410_e31868_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign26410_e31858: f64 = (var_absource_i * var_ijunbot);
        let assign26410_e31861: f64 = (var_lssource_i * var_ijunsti);
        let assign26410_e31862: f64 = (assign26410_e31858 + assign26410_e31861);
        let assign26410_e31865: f64 = (var_lgsource_i * var_ijungat);
        let assign26410_e31866: f64 = (assign26410_e31862 + assign26410_e31865);
        (assign26410_e31866, (((var_absource_i * var_ijunbot_dn6) + (var_lssource_i * var_ijunsti_dn6)) + (var_lgsource_i * var_ijungat_dn6)), (((var_absource_i * var_ijunbot_dn7) + (var_lssource_i * var_ijunsti_dn7)) + (var_lgsource_i * var_ijungat_dn7)), (((var_absource_i * var_ijunbot_dn8) + (var_lssource_i * var_ijunsti_dn8)) + (var_lgsource_i * var_ijungat_dn8)), (((var_absource_i * var_ijunbot_dn9) + (var_lssource_i * var_ijunsti_dn9)) + (var_lgsource_i * var_ijungat_dn9)),)
    } else {
        (var_i5, var_i5_dn6, var_i5_dn7, var_i5_dn8, var_i5_dn9,)
    }
};
        var_i5 = assign26410_e31868;
        var_i5_dn6 = assign26410_e31868_d_n6;
        var_i5_dn7 = assign26410_e31868_d_n7;
        var_i5_dn8 = assign26410_e31868_d_n8;
        var_i5_dn9 = assign26410_e31868_d_n9;

        let (assign26420_e31884,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign26420_e31874: f64 = (var_absource_i * var_idsatbot);
        let assign26420_e31877: f64 = (var_lssource_i * var_idsatsti);
        let assign26420_e31878: f64 = (assign26420_e31874 + assign26420_e31877);
        let assign26420_e31881: f64 = (var_lgsource_i * var_idsatgat);
        let assign26420_e31882: f64 = (assign26420_e31878 + assign26420_e31881);
        (assign26420_e31882,)
    } else {
        (var_isatfor1_s,)
    }
};
        var_isatfor1_s = assign26420_e31884;

        let (assign26430_e31901, assign26430_e31901_d_n6, assign26430_e31901_d_n7, assign26430_e31901_d_n8, assign26430_e31901_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign26430_e31892: f64 = (var_v4 * var_phitdinv);
        let assign26430_e31894: f64 = (assign26430_e31892 * var_mfor1_s);
        let assign26430_e31895: f64 = (assign26430_e31894).exp();
        let assign26430_e31897: f64 = (assign26430_e31895 - 1.0);
        let assign26430_e31898: f64 = (var_isatfor1_s * assign26430_e31897);
        let assign26430_e31899: f64 = (var_i4 - assign26430_e31898);
        (assign26430_e31899, var_i4_dn6, var_i4_dn7, var_i4_dn8, var_i4_dn9,)
    } else {
        (var_i4_cor, var_i4_cor_dn6, var_i4_cor_dn7, var_i4_cor_dn8, var_i4_cor_dn9,)
    }
};
        var_i4_cor = assign26430_e31901;
        var_i4_cor_dn6 = assign26430_e31901_d_n6;
        var_i4_cor_dn7 = assign26430_e31901_d_n7;
        var_i4_cor_dn8 = assign26430_e31901_d_n8;
        var_i4_cor_dn9 = assign26430_e31901_d_n9;

        let (assign26440_e31918, assign26440_e31918_d_n6, assign26440_e31918_d_n7, assign26440_e31918_d_n8, assign26440_e31918_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign26440_e31909: f64 = (var_v5 * var_phitdinv);
        let assign26440_e31911: f64 = (assign26440_e31909 * var_mfor1_s);
        let assign26440_e31912: f64 = (assign26440_e31911).exp();
        let assign26440_e31914: f64 = (assign26440_e31912 - 1.0);
        let assign26440_e31915: f64 = (var_isatfor1_s * assign26440_e31914);
        let assign26440_e31916: f64 = (var_i5 - assign26440_e31915);
        (assign26440_e31916, var_i5_dn6, var_i5_dn7, var_i5_dn8, var_i5_dn9,)
    } else {
        (var_i5_cor, var_i5_cor_dn6, var_i5_cor_dn7, var_i5_cor_dn8, var_i5_cor_dn9,)
    }
};
        var_i5_cor = assign26440_e31918;
        var_i5_cor_dn6 = assign26440_e31918_d_n6;
        var_i5_cor_dn7 = assign26440_e31918_d_n7;
        var_i5_cor_dn8 = assign26440_e31918_d_n8;
        var_i5_cor_dn9 = assign26440_e31918_d_n9;

        let assign26450_e31930: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard521 = assign26450_e31930;

        let assign26460_e31937: f64 = if ((var_i4 > 0.0) && (var_i5 > 0.0)) { 1.0 } else { 0.0 };
        var_guard522 = assign26460_e31937;

        let assign26470_e31940: f64 = (var_i4_cor / var_i4);
        let assign26470_e31945: f64 = (var_i5_cor / var_i5);
        let assign26470_e31960: f64 = if (((((assign26470_e31940 > 0.001) || (assign26470_e31945 > 0.001)) && (var_i4_cor > 0.0)) && (var_i5_cor > 0.0)) && (var_i5_cor > var_i4_cor)) { 1.0 } else { 0.0 };
        var_guard523 = assign26470_e31960;

        let (assign26480_e31974, assign26480_e31974_d_n6, assign26480_e31974_d_n7, assign26480_e31974_d_n8, assign26480_e31974_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) && (var_guard522 != 0.0)) && (var_guard523 != 0.0)) {
        let assign26480_e31972: f64 = (var_i4_cor / var_i5_cor);
        (assign26480_e31972, (((var_i4_cor_dn6 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn6)) / (var_i5_cor * var_i5_cor)), (((var_i4_cor_dn7 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn7)) / (var_i5_cor * var_i5_cor)), (((var_i4_cor_dn8 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn8)) / (var_i5_cor * var_i5_cor)), (((var_i4_cor_dn9 * var_i5_cor) - (var_i4_cor * var_i5_cor_dn9)) / (var_i5_cor * var_i5_cor)),)
    } else {
        (var_alphaje, var_alphaje_dn6, var_alphaje_dn7, var_alphaje_dn8, var_alphaje_dn9,)
    }
};
        var_alphaje = assign26480_e31974;
        var_alphaje_dn6 = assign26480_e31974_d_n6;
        var_alphaje_dn7 = assign26480_e31974_d_n7;
        var_alphaje_dn8 = assign26480_e31974_d_n8;
        var_alphaje_dn9 = assign26480_e31974_d_n9;

        let (assign26490_e31993, assign26490_e31993_d_n6, assign26490_e31993_d_n7, assign26490_e31993_d_n8, assign26490_e31993_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) && (var_guard522 != 0.0)) && (var_guard523 != 0.0)) {
        let assign26490_e31986: f64 = (var_alphaje).ln();
        let assign26490_e31987: f64 = (var_phitd * assign26490_e31986);
        let assign26490_e31990: f64 = (var_v4 - var_v5);
        let assign26490_e31991: f64 = (assign26490_e31987 / assign26490_e31990);
        (assign26490_e31991, ((var_phitd * (var_alphaje_dn6 / var_alphaje)) / assign26490_e31990), ((var_phitd * (var_alphaje_dn7 / var_alphaje)) / assign26490_e31990), ((var_phitd * (var_alphaje_dn8 / var_alphaje)) / assign26490_e31990), ((var_phitd * (var_alphaje_dn9 / var_alphaje)) / assign26490_e31990),)
    } else {
        (var_mfor2_s, var_mfor2_s_dn6, var_mfor2_s_dn7, var_mfor2_s_dn8, var_mfor2_s_dn9,)
    }
};
        var_mfor2_s = assign26490_e31993;
        var_mfor2_s_dn6 = assign26490_e31993_d_n6;
        var_mfor2_s_dn7 = assign26490_e31993_d_n7;
        var_mfor2_s_dn8 = assign26490_e31993_d_n8;
        var_mfor2_s_dn9 = assign26490_e31993_d_n9;

        let (assign26500_e32014, assign26500_e32014_d_n6, assign26500_e32014_d_n7, assign26500_e32014_d_n8, assign26500_e32014_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) && (var_guard522 != 0.0)) && (var_guard523 != 0.0)) {
        let assign26500_e32006: f64 = (var_v4 * var_phitdinv);
        let assign26500_e32008: f64 = (assign26500_e32006 * var_mfor2_s);
        let assign26500_e32009: f64 = (assign26500_e32008).exp();
        let assign26500_e32011: f64 = (assign26500_e32009 - 1.0);
        let assign26500_e32012: f64 = (var_i4_cor / assign26500_e32011);
        (assign26500_e32012, (((var_i4_cor_dn6 * assign26500_e32011) - (var_i4_cor * (assign26500_e32009 * (assign26500_e32006 * var_mfor2_s_dn6)))) / (assign26500_e32011 * assign26500_e32011)), (((var_i4_cor_dn7 * assign26500_e32011) - (var_i4_cor * (assign26500_e32009 * (assign26500_e32006 * var_mfor2_s_dn7)))) / (assign26500_e32011 * assign26500_e32011)), (((var_i4_cor_dn8 * assign26500_e32011) - (var_i4_cor * (assign26500_e32009 * (assign26500_e32006 * var_mfor2_s_dn8)))) / (assign26500_e32011 * assign26500_e32011)), (((var_i4_cor_dn9 * assign26500_e32011) - (var_i4_cor * (assign26500_e32009 * (assign26500_e32006 * var_mfor2_s_dn9)))) / (assign26500_e32011 * assign26500_e32011)),)
    } else {
        (var_isatfor2_s, var_isatfor2_s_dn6, var_isatfor2_s_dn7, var_isatfor2_s_dn8, var_isatfor2_s_dn9,)
    }
};
        var_isatfor2_s = assign26500_e32014;
        var_isatfor2_s_dn6 = assign26500_e32014_d_n6;
        var_isatfor2_s_dn7 = assign26500_e32014_d_n7;
        var_isatfor2_s_dn8 = assign26500_e32014_d_n8;
        var_isatfor2_s_dn9 = assign26500_e32014_d_n9;

        let (assign26510_e32044, assign26510_e32044_d_n6, assign26510_e32044_d_n7, assign26510_e32044_d_n8, assign26510_e32044_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) {
        let assign26510_e32024: f64 = (var_v1 * var_phitdinv);
        let assign26510_e32026: f64 = (assign26510_e32024 * var_mfor1_s);
        let assign26510_e32027: f64 = (assign26510_e32026).exp();
        let assign26510_e32029: f64 = (assign26510_e32027 - 1.0);
        let assign26510_e32030: f64 = (var_isatfor1_s * assign26510_e32029);
        let assign26510_e32031: f64 = (var_i1 - assign26510_e32030);
        let assign26510_e32035: f64 = (var_v1 * var_phitdinv);
        let assign26510_e32037: f64 = (assign26510_e32035 * var_mfor2_s);
        let assign26510_e32038: f64 = (assign26510_e32037).exp();
        let assign26510_e32040: f64 = (assign26510_e32038 - 1.0);
        let assign26510_e32041: f64 = (var_isatfor2_s * assign26510_e32040);
        let assign26510_e32042: f64 = (assign26510_e32031 - assign26510_e32041);
        (assign26510_e32042, (var_i1_dn6 - ((var_isatfor2_s_dn6 * assign26510_e32040) + (var_isatfor2_s * (assign26510_e32038 * (assign26510_e32035 * var_mfor2_s_dn6))))), (var_i1_dn7 - ((var_isatfor2_s_dn7 * assign26510_e32040) + (var_isatfor2_s * (assign26510_e32038 * (assign26510_e32035 * var_mfor2_s_dn7))))), (var_i1_dn8 - ((var_isatfor2_s_dn8 * assign26510_e32040) + (var_isatfor2_s * (assign26510_e32038 * (assign26510_e32035 * var_mfor2_s_dn8))))), (var_i1_dn9 - ((var_isatfor2_s_dn9 * assign26510_e32040) + (var_isatfor2_s * (assign26510_e32038 * (assign26510_e32035 * var_mfor2_s_dn9))))),)
    } else {
        (var_i1_cor, var_i1_cor_dn6, var_i1_cor_dn7, var_i1_cor_dn8, var_i1_cor_dn9,)
    }
};
        var_i1_cor = assign26510_e32044;
        var_i1_cor_dn6 = assign26510_e32044_d_n6;
        var_i1_cor_dn7 = assign26510_e32044_d_n7;
        var_i1_cor_dn8 = assign26510_e32044_d_n8;
        var_i1_cor_dn9 = assign26510_e32044_d_n9;

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
        *var_guard514_slot = var_guard514;
        *var_guard515_slot = var_guard515;
        *var_guard516_slot = var_guard516;
        *var_guard517_slot = var_guard517;
        *var_guard518_slot = var_guard518;
        *var_guard519_slot = var_guard519;
        *var_guard520_slot = var_guard520;
        *var_guard521_slot = var_guard521;
        *var_guard522_slot = var_guard522;
        *var_guard523_slot = var_guard523;
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
        *var_isatfor1_s_slot = var_isatfor1_s;
        *var_isatfor2_s_slot = var_isatfor2_s;
        *var_isatfor2_s_dn6_slot = var_isatfor2_s_dn6;
        *var_isatfor2_s_dn7_slot = var_isatfor2_s_dn7;
        *var_isatfor2_s_dn8_slot = var_isatfor2_s_dn8;
        *var_isatfor2_s_dn9_slot = var_isatfor2_s_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_mfor2_s_slot = var_mfor2_s;
        *var_mfor2_s_dn6_slot = var_mfor2_s_dn6;
        *var_mfor2_s_dn7_slot = var_mfor2_s_dn7;
        *var_mfor2_s_dn8_slot = var_mfor2_s_dn8;
        *var_mfor2_s_dn9_slot = var_mfor2_s_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
    }

    pub(super) fn stamp_transient_block_52(
        p: &Parameters,
        var_absource_i: f64,
        var_cjobot: f64,
        var_cjogat: f64,
        var_cjosti: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard521: f64,
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
        var_isatfor1_s: f64,
        var_isatfor2_s: f64,
        var_isatfor2_s_dn6: f64,
        var_isatfor2_s_dn7: f64,
        var_isatfor2_s_dn8: f64,
        var_isatfor2_s_dn9: f64,
        var_lgsource_i: f64,
        var_lssource_i: f64,
        var_mfor1_s: f64,
        var_mfor2_s: f64,
        var_mfor2_s_dn6: f64,
        var_mfor2_s_dn7: f64,
        var_mfor2_s_dn8: f64,
        var_mfor2_s_dn9: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_vjunrefd_i: f64,
        var_alphaje_slot: &mut f64,
        var_alphaje_dn6_slot: &mut f64,
        var_alphaje_dn7_slot: &mut f64,
        var_alphaje_dn8_slot: &mut f64,
        var_alphaje_dn9_slot: &mut f64,
        var_expxhf1_s_slot: &mut f64,
        var_expxhf2_s_slot: &mut f64,
        var_expxhf2_s_dn6_slot: &mut f64,
        var_expxhf2_s_dn7_slot: &mut f64,
        var_expxhf2_s_dn8_slot: &mut f64,
        var_expxhf2_s_dn9_slot: &mut f64,
        var_expxhr_s_slot: &mut f64,
        var_expxhr_s_dn6_slot: &mut f64,
        var_expxhr_s_dn7_slot: &mut f64,
        var_expxhr_s_dn8_slot: &mut f64,
        var_expxhr_s_dn9_slot: &mut f64,
        var_fraci_slot: &mut f64,
        var_fracna_slot: &mut f64,
        var_fracnb_slot: &mut f64,
        var_guard524_slot: &mut f64,
        var_guard525_slot: &mut f64,
        var_guard526_slot: &mut f64,
        var_guard527_slot: &mut f64,
        var_guard528_slot: &mut f64,
        var_guard529_slot: &mut f64,
        var_guard530_slot: &mut f64,
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
        var_isatrev_s_slot: &mut f64,
        var_isatrev_s_dn6_slot: &mut f64,
        var_isatrev_s_dn7_slot: &mut f64,
        var_isatrev_s_dn8_slot: &mut f64,
        var_isatrev_s_dn9_slot: &mut f64,
        var_m0_rev_slot: &mut f64,
        var_m0_rev_dn6_slot: &mut f64,
        var_m0_rev_dn7_slot: &mut f64,
        var_m0_rev_dn8_slot: &mut f64,
        var_m0_rev_dn9_slot: &mut f64,
        var_m0flag_s_slot: &mut f64,
        var_mcor_rev_slot: &mut f64,
        var_mcor_rev_dn6_slot: &mut f64,
        var_mcor_rev_dn7_slot: &mut f64,
        var_mcor_rev_dn8_slot: &mut f64,
        var_mcor_rev_dn9_slot: &mut f64,
        var_mrev_s_slot: &mut f64,
        var_mrev_s_dn6_slot: &mut f64,
        var_mrev_s_dn7_slot: &mut f64,
        var_mrev_s_dn8_slot: &mut f64,
        var_mrev_s_dn9_slot: &mut f64,
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
        var_v1_slot: &mut f64,
        var_v2_slot: &mut f64,
        var_v3_slot: &mut f64,
        var_v4_slot: &mut f64,
        var_xhighf1_s_slot: &mut f64,
        var_xhighf2_s_slot: &mut f64,
        var_xhighf2_s_dn6_slot: &mut f64,
        var_xhighf2_s_dn7_slot: &mut f64,
        var_xhighf2_s_dn8_slot: &mut f64,
        var_xhighf2_s_dn9_slot: &mut f64,
        var_xhighr_s_slot: &mut f64,
        var_xhighr_s_dn6_slot: &mut f64,
        var_xhighr_s_dn7_slot: &mut f64,
        var_xhighr_s_dn8_slot: &mut f64,
        var_xhighr_s_dn9_slot: &mut f64,
        var_zflagbot_s_slot: &mut f64,
        var_zflaggat_s_slot: &mut f64,
        var_zflagsti_s_slot: &mut f64,
        var_zfrac_slot: &mut f64,
    ) {
        let mut var_alphaje: f64 = *var_alphaje_slot;
        let mut var_alphaje_dn6: f64 = *var_alphaje_dn6_slot;
        let mut var_alphaje_dn7: f64 = *var_alphaje_dn7_slot;
        let mut var_alphaje_dn8: f64 = *var_alphaje_dn8_slot;
        let mut var_alphaje_dn9: f64 = *var_alphaje_dn9_slot;
        let mut var_expxhf1_s: f64 = *var_expxhf1_s_slot;
        let mut var_expxhf2_s: f64 = *var_expxhf2_s_slot;
        let mut var_expxhf2_s_dn6: f64 = *var_expxhf2_s_dn6_slot;
        let mut var_expxhf2_s_dn7: f64 = *var_expxhf2_s_dn7_slot;
        let mut var_expxhf2_s_dn8: f64 = *var_expxhf2_s_dn8_slot;
        let mut var_expxhf2_s_dn9: f64 = *var_expxhf2_s_dn9_slot;
        let mut var_expxhr_s: f64 = *var_expxhr_s_slot;
        let mut var_expxhr_s_dn6: f64 = *var_expxhr_s_dn6_slot;
        let mut var_expxhr_s_dn7: f64 = *var_expxhr_s_dn7_slot;
        let mut var_expxhr_s_dn8: f64 = *var_expxhr_s_dn8_slot;
        let mut var_expxhr_s_dn9: f64 = *var_expxhr_s_dn9_slot;
        let mut var_fraci: f64 = *var_fraci_slot;
        let mut var_fracna: f64 = *var_fracna_slot;
        let mut var_fracnb: f64 = *var_fracnb_slot;
        let mut var_guard524: f64 = *var_guard524_slot;
        let mut var_guard525: f64 = *var_guard525_slot;
        let mut var_guard526: f64 = *var_guard526_slot;
        let mut var_guard527: f64 = *var_guard527_slot;
        let mut var_guard528: f64 = *var_guard528_slot;
        let mut var_guard529: f64 = *var_guard529_slot;
        let mut var_guard530: f64 = *var_guard530_slot;
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
        let mut var_isatrev_s: f64 = *var_isatrev_s_slot;
        let mut var_isatrev_s_dn6: f64 = *var_isatrev_s_dn6_slot;
        let mut var_isatrev_s_dn7: f64 = *var_isatrev_s_dn7_slot;
        let mut var_isatrev_s_dn8: f64 = *var_isatrev_s_dn8_slot;
        let mut var_isatrev_s_dn9: f64 = *var_isatrev_s_dn9_slot;
        let mut var_m0_rev: f64 = *var_m0_rev_slot;
        let mut var_m0_rev_dn6: f64 = *var_m0_rev_dn6_slot;
        let mut var_m0_rev_dn7: f64 = *var_m0_rev_dn7_slot;
        let mut var_m0_rev_dn8: f64 = *var_m0_rev_dn8_slot;
        let mut var_m0_rev_dn9: f64 = *var_m0_rev_dn9_slot;
        let mut var_m0flag_s: f64 = *var_m0flag_s_slot;
        let mut var_mcor_rev: f64 = *var_mcor_rev_slot;
        let mut var_mcor_rev_dn6: f64 = *var_mcor_rev_dn6_slot;
        let mut var_mcor_rev_dn7: f64 = *var_mcor_rev_dn7_slot;
        let mut var_mcor_rev_dn8: f64 = *var_mcor_rev_dn8_slot;
        let mut var_mcor_rev_dn9: f64 = *var_mcor_rev_dn9_slot;
        let mut var_mrev_s: f64 = *var_mrev_s_slot;
        let mut var_mrev_s_dn6: f64 = *var_mrev_s_dn6_slot;
        let mut var_mrev_s_dn7: f64 = *var_mrev_s_dn7_slot;
        let mut var_mrev_s_dn8: f64 = *var_mrev_s_dn8_slot;
        let mut var_mrev_s_dn9: f64 = *var_mrev_s_dn9_slot;
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
        let mut var_v1: f64 = *var_v1_slot;
        let mut var_v2: f64 = *var_v2_slot;
        let mut var_v3: f64 = *var_v3_slot;
        let mut var_v4: f64 = *var_v4_slot;
        let mut var_xhighf1_s: f64 = *var_xhighf1_s_slot;
        let mut var_xhighf2_s: f64 = *var_xhighf2_s_slot;
        let mut var_xhighf2_s_dn6: f64 = *var_xhighf2_s_dn6_slot;
        let mut var_xhighf2_s_dn7: f64 = *var_xhighf2_s_dn7_slot;
        let mut var_xhighf2_s_dn8: f64 = *var_xhighf2_s_dn8_slot;
        let mut var_xhighf2_s_dn9: f64 = *var_xhighf2_s_dn9_slot;
        let mut var_xhighr_s: f64 = *var_xhighr_s_slot;
        let mut var_xhighr_s_dn6: f64 = *var_xhighr_s_dn6_slot;
        let mut var_xhighr_s_dn7: f64 = *var_xhighr_s_dn7_slot;
        let mut var_xhighr_s_dn8: f64 = *var_xhighr_s_dn8_slot;
        let mut var_xhighr_s_dn9: f64 = *var_xhighr_s_dn9_slot;
        let mut var_zflagbot_s: f64 = *var_zflagbot_s_slot;
        let mut var_zflaggat_s: f64 = *var_zflaggat_s_slot;
        let mut var_zflagsti_s: f64 = *var_zflagsti_s_slot;
        let mut var_zfrac: f64 = *var_zfrac_slot;

        let (assign26520_e32074, assign26520_e32074_d_n6, assign26520_e32074_d_n7, assign26520_e32074_d_n8, assign26520_e32074_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) {
        let assign26520_e32054: f64 = (var_v2 * var_phitdinv);
        let assign26520_e32056: f64 = (assign26520_e32054 * var_mfor1_s);
        let assign26520_e32057: f64 = (assign26520_e32056).exp();
        let assign26520_e32059: f64 = (assign26520_e32057 - 1.0);
        let assign26520_e32060: f64 = (var_isatfor1_s * assign26520_e32059);
        let assign26520_e32061: f64 = (var_i2 - assign26520_e32060);
        let assign26520_e32065: f64 = (var_v2 * var_phitdinv);
        let assign26520_e32067: f64 = (assign26520_e32065 * var_mfor2_s);
        let assign26520_e32068: f64 = (assign26520_e32067).exp();
        let assign26520_e32070: f64 = (assign26520_e32068 - 1.0);
        let assign26520_e32071: f64 = (var_isatfor2_s * assign26520_e32070);
        let assign26520_e32072: f64 = (assign26520_e32061 - assign26520_e32071);
        (assign26520_e32072, (var_i2_dn6 - ((var_isatfor2_s_dn6 * assign26520_e32070) + (var_isatfor2_s * (assign26520_e32068 * (assign26520_e32065 * var_mfor2_s_dn6))))), (var_i2_dn7 - ((var_isatfor2_s_dn7 * assign26520_e32070) + (var_isatfor2_s * (assign26520_e32068 * (assign26520_e32065 * var_mfor2_s_dn7))))), (var_i2_dn8 - ((var_isatfor2_s_dn8 * assign26520_e32070) + (var_isatfor2_s * (assign26520_e32068 * (assign26520_e32065 * var_mfor2_s_dn8))))), (var_i2_dn9 - ((var_isatfor2_s_dn9 * assign26520_e32070) + (var_isatfor2_s * (assign26520_e32068 * (assign26520_e32065 * var_mfor2_s_dn9))))),)
    } else {
        (var_i2_cor, var_i2_cor_dn6, var_i2_cor_dn7, var_i2_cor_dn8, var_i2_cor_dn9,)
    }
};
        var_i2_cor = assign26520_e32074;
        var_i2_cor_dn6 = assign26520_e32074_d_n6;
        var_i2_cor_dn7 = assign26520_e32074_d_n7;
        var_i2_cor_dn8 = assign26520_e32074_d_n8;
        var_i2_cor_dn9 = assign26520_e32074_d_n9;

        let (assign26530_e32104, assign26530_e32104_d_n6, assign26530_e32104_d_n7, assign26530_e32104_d_n8, assign26530_e32104_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) {
        let assign26530_e32084: f64 = (var_v3 * var_phitdinv);
        let assign26530_e32086: f64 = (assign26530_e32084 * var_mfor1_s);
        let assign26530_e32087: f64 = (assign26530_e32086).exp();
        let assign26530_e32089: f64 = (assign26530_e32087 - 1.0);
        let assign26530_e32090: f64 = (var_isatfor1_s * assign26530_e32089);
        let assign26530_e32091: f64 = (var_i3 - assign26530_e32090);
        let assign26530_e32095: f64 = (var_v3 * var_phitdinv);
        let assign26530_e32097: f64 = (assign26530_e32095 * var_mfor2_s);
        let assign26530_e32098: f64 = (assign26530_e32097).exp();
        let assign26530_e32100: f64 = (assign26530_e32098 - 1.0);
        let assign26530_e32101: f64 = (var_isatfor2_s * assign26530_e32100);
        let assign26530_e32102: f64 = (assign26530_e32091 - assign26530_e32101);
        (assign26530_e32102, (var_i3_dn6 - ((var_isatfor2_s_dn6 * assign26530_e32100) + (var_isatfor2_s * (assign26530_e32098 * (assign26530_e32095 * var_mfor2_s_dn6))))), (var_i3_dn7 - ((var_isatfor2_s_dn7 * assign26530_e32100) + (var_isatfor2_s * (assign26530_e32098 * (assign26530_e32095 * var_mfor2_s_dn7))))), (var_i3_dn8 - ((var_isatfor2_s_dn8 * assign26530_e32100) + (var_isatfor2_s * (assign26530_e32098 * (assign26530_e32095 * var_mfor2_s_dn8))))), (var_i3_dn9 - ((var_isatfor2_s_dn9 * assign26530_e32100) + (var_isatfor2_s * (assign26530_e32098 * (assign26530_e32095 * var_mfor2_s_dn9))))),)
    } else {
        (var_i3_cor, var_i3_cor_dn6, var_i3_cor_dn7, var_i3_cor_dn8, var_i3_cor_dn9,)
    }
};
        var_i3_cor = assign26530_e32104;
        var_i3_cor_dn6 = assign26530_e32104_d_n6;
        var_i3_cor_dn7 = assign26530_e32104_d_n7;
        var_i3_cor_dn8 = assign26530_e32104_d_n8;
        var_i3_cor_dn9 = assign26530_e32104_d_n9;

        let assign26540_e32115: f64 = if (((var_i1 < 0.0) && (var_i2 < 0.0)) && (var_i3 < 0.0)) { 1.0 } else { 0.0 };
        var_guard524 = assign26540_e32115;

        let assign26550_e32118: f64 = (var_i1_cor / var_i1);
        let assign26550_e32123: f64 = (var_i2_cor / var_i2);
        let assign26550_e32129: f64 = (var_i3_cor / var_i3);
        let assign26550_e32144: f64 = if ((((((assign26550_e32118 > 0.001) || (assign26550_e32123 > 0.001)) || (assign26550_e32129 > 0.001)) && (var_i1_cor < 0.0)) && (var_i2_cor < 0.0)) && (var_i3_cor < 0.0)) { 1.0 } else { 0.0 };
        var_guard525 = assign26550_e32144;

        let (assign26560_e32158, assign26560_e32158_d_n6, assign26560_e32158_d_n7, assign26560_e32158_d_n8, assign26560_e32158_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) && (var_guard524 != 0.0)) && (var_guard525 != 0.0)) {
        let assign26560_e32156: f64 = (var_i1_cor / var_i2_cor);
        (assign26560_e32156, (((var_i1_cor_dn6 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn6)) / (var_i2_cor * var_i2_cor)), (((var_i1_cor_dn7 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn7)) / (var_i2_cor * var_i2_cor)), (((var_i1_cor_dn8 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn8)) / (var_i2_cor * var_i2_cor)), (((var_i1_cor_dn9 * var_i2_cor) - (var_i1_cor * var_i2_cor_dn9)) / (var_i2_cor * var_i2_cor)),)
    } else {
        (var_alphaje, var_alphaje_dn6, var_alphaje_dn7, var_alphaje_dn8, var_alphaje_dn9,)
    }
};
        var_alphaje = assign26560_e32158;
        var_alphaje_dn6 = assign26560_e32158_d_n6;
        var_alphaje_dn7 = assign26560_e32158_d_n7;
        var_alphaje_dn8 = assign26560_e32158_d_n8;
        var_alphaje_dn9 = assign26560_e32158_d_n9;

        let (assign26570_e32178, assign26570_e32178_d_n6, assign26570_e32178_d_n7, assign26570_e32178_d_n8, assign26570_e32178_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) && (var_guard524 != 0.0)) && (var_guard525 != 0.0)) {
        let assign26570_e32169: f64 = (-var_phitd);
        let assign26570_e32171: f64 = (var_alphaje).ln();
        let assign26570_e32172: f64 = (assign26570_e32169 * assign26570_e32171);
        let assign26570_e32175: f64 = (var_v1 - var_v2);
        let assign26570_e32176: f64 = (assign26570_e32172 / assign26570_e32175);
        (assign26570_e32176, ((assign26570_e32169 * (var_alphaje_dn6 / var_alphaje)) / assign26570_e32175), ((assign26570_e32169 * (var_alphaje_dn7 / var_alphaje)) / assign26570_e32175), ((assign26570_e32169 * (var_alphaje_dn8 / var_alphaje)) / assign26570_e32175), ((assign26570_e32169 * (var_alphaje_dn9 / var_alphaje)) / assign26570_e32175),)
    } else {
        (var_m0_rev, var_m0_rev_dn6, var_m0_rev_dn7, var_m0_rev_dn8, var_m0_rev_dn9,)
    }
};
        var_m0_rev = assign26570_e32178;
        var_m0_rev_dn6 = assign26570_e32178_d_n6;
        var_m0_rev_dn7 = assign26570_e32178_d_n7;
        var_m0_rev_dn8 = assign26570_e32178_d_n8;
        var_m0_rev_dn9 = assign26570_e32178_d_n9;

        let (assign26580_e32194,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) && (var_guard524 != 0.0)) && (var_guard525 != 0.0)) {
        let assign26580_e32191: f64 = (var_v2 - var_v1);
        let assign26580_e32192: f64 = (var_v2 / assign26580_e32191);
        (assign26580_e32192,)
    } else {
        (var_tt0,)
    }
};
        var_tt0 = assign26580_e32194;

        let (assign26590_e32216, assign26590_e32216_d_n6, assign26590_e32216_d_n7, assign26590_e32216_d_n8, assign26590_e32216_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) && (var_guard524 != 0.0)) && (var_guard525 != 0.0)) {
        let assign26590_e32207: f64 = (var_alphaje - 1.0);
        let assign26590_e32208: f64 = (var_phitd * assign26590_e32207);
        let assign26590_e32211: f64 = (var_alphaje).powf(var_tt0);
        let assign26590_e32213: f64 = (assign26590_e32211 - 1.0);
        let assign26590_e32214: f64 = (assign26590_e32208 * assign26590_e32213);
        (assign26590_e32214, (((var_phitd * var_alphaje_dn6) * assign26590_e32213) + (assign26590_e32208 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn6)) } } else { (assign26590_e32211 * (var_tt0 * (var_alphaje_dn6 / var_alphaje))) })), (((var_phitd * var_alphaje_dn7) * assign26590_e32213) + (assign26590_e32208 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn7)) } } else { (assign26590_e32211 * (var_tt0 * (var_alphaje_dn7 / var_alphaje))) })), (((var_phitd * var_alphaje_dn8) * assign26590_e32213) + (assign26590_e32208 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn8)) } } else { (assign26590_e32211 * (var_tt0 * (var_alphaje_dn8 / var_alphaje))) })), (((var_phitd * var_alphaje_dn9) * assign26590_e32213) + (assign26590_e32208 * if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn9)) } } else { (assign26590_e32211 * (var_tt0 * (var_alphaje_dn9 / var_alphaje))) })),)
    } else {
        (var_tt1, var_tt1_dn6, var_tt1_dn7, var_tt1_dn8, var_tt1_dn9,)
    }
};
        var_tt1 = assign26590_e32216;
        var_tt1_dn6 = assign26590_e32216_d_n6;
        var_tt1_dn7 = assign26590_e32216_d_n7;
        var_tt1_dn8 = assign26590_e32216_d_n8;
        var_tt1_dn9 = assign26590_e32216_d_n9;

        let (assign26600_e32232,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) && (var_guard524 != 0.0)) && (var_guard525 != 0.0)) {
        let assign26600_e32229: f64 = (var_v1 - var_v2);
        let assign26600_e32230: f64 = (var_v1 / assign26600_e32229);
        (assign26600_e32230,)
    } else {
        (var_tt0,)
    }
};
        var_tt0 = assign26600_e32232;

        let (assign26610_e32256, assign26610_e32256_d_n6, assign26610_e32256_d_n7, assign26610_e32256_d_n8, assign26610_e32256_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) && (var_guard524 != 0.0)) && (var_guard525 != 0.0)) {
        let assign26610_e32244: f64 = (var_alphaje).powf(var_tt0);
        let assign26610_e32247: f64 = (var_v2 - var_v1);
        let assign26610_e32248: f64 = (assign26610_e32244 * assign26610_e32247);
        let assign26610_e32251: f64 = (var_alphaje * var_v1);
        let assign26610_e32252: f64 = (assign26610_e32248 + assign26610_e32251);
        let assign26610_e32254: f64 = (assign26610_e32252 - var_v2);
        (assign26610_e32254, ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn6)) } } else { (assign26610_e32244 * (var_tt0 * (var_alphaje_dn6 / var_alphaje))) } * assign26610_e32247) + (var_alphaje_dn6 * var_v1)), ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn7)) } } else { (assign26610_e32244 * (var_tt0 * (var_alphaje_dn7 / var_alphaje))) } * assign26610_e32247) + (var_alphaje_dn7 * var_v1)), ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn8)) } } else { (assign26610_e32244 * (var_tt0 * (var_alphaje_dn8 / var_alphaje))) } * assign26610_e32247) + (var_alphaje_dn8 * var_v1)), ((if 0.0 == 0.0 && ((var_tt0) as f64).is_finite() && ((var_tt0) as f64).fract() == 0.0 { if var_tt0 == 0.0 { 0.0 } else { (var_tt0 * ((var_alphaje).powf(var_tt0 - 1.0) * var_alphaje_dn9)) } } else { (assign26610_e32244 * (var_tt0 * (var_alphaje_dn9 / var_alphaje))) } * assign26610_e32247) + (var_alphaje_dn9 * var_v1)),)
    } else {
        (var_tt2, var_tt2_dn6, var_tt2_dn7, var_tt2_dn8, var_tt2_dn9,)
    }
};
        var_tt2 = assign26610_e32256;
        var_tt2_dn6 = assign26610_e32256_d_n6;
        var_tt2_dn7 = assign26610_e32256_d_n7;
        var_tt2_dn8 = assign26610_e32256_d_n8;
        var_tt2_dn9 = assign26610_e32256_d_n9;

        let (assign26620_e32270, assign26620_e32270_d_n6, assign26620_e32270_d_n7, assign26620_e32270_d_n8, assign26620_e32270_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) && (var_guard524 != 0.0)) && (var_guard525 != 0.0)) {
        let assign26620_e32268: f64 = (var_tt1 / var_tt2);
        (assign26620_e32268, (((var_tt1_dn6 * var_tt2) - (var_tt1 * var_tt2_dn6)) / (var_tt2 * var_tt2)), (((var_tt1_dn7 * var_tt2) - (var_tt1 * var_tt2_dn7)) / (var_tt2 * var_tt2)), (((var_tt1_dn8 * var_tt2) - (var_tt1 * var_tt2_dn8)) / (var_tt2 * var_tt2)), (((var_tt1_dn9 * var_tt2) - (var_tt1 * var_tt2_dn9)) / (var_tt2 * var_tt2)),)
    } else {
        (var_mcor_rev, var_mcor_rev_dn6, var_mcor_rev_dn7, var_mcor_rev_dn8, var_mcor_rev_dn9,)
    }
};
        var_mcor_rev = assign26620_e32270;
        var_mcor_rev_dn6 = assign26620_e32270_d_n6;
        var_mcor_rev_dn7 = assign26620_e32270_d_n7;
        var_mcor_rev_dn8 = assign26620_e32270_d_n8;
        var_mcor_rev_dn9 = assign26620_e32270_d_n9;

        let (assign26630_e32284, assign26630_e32284_d_n6, assign26630_e32284_d_n7, assign26630_e32284_d_n8, assign26630_e32284_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) && (var_guard524 != 0.0)) && (var_guard525 != 0.0)) {
        let assign26630_e32282: f64 = (var_m0_rev + var_mcor_rev);
        (assign26630_e32282, (var_m0_rev_dn6 + var_mcor_rev_dn6), (var_m0_rev_dn7 + var_mcor_rev_dn7), (var_m0_rev_dn8 + var_mcor_rev_dn8), (var_m0_rev_dn9 + var_mcor_rev_dn9),)
    } else {
        (var_mrev_s, var_mrev_s_dn6, var_mrev_s_dn7, var_mrev_s_dn8, var_mrev_s_dn9,)
    }
};
        var_mrev_s = assign26630_e32284;
        var_mrev_s_dn6 = assign26630_e32284_d_n6;
        var_mrev_s_dn7 = assign26630_e32284_d_n7;
        var_mrev_s_dn8 = assign26630_e32284_d_n8;
        var_mrev_s_dn9 = assign26630_e32284_d_n9;

        let assign26640_e32287: f64 = (var_v3 * var_phitdinv);
        let assign26640_e32289: f64 = (assign26640_e32287 * var_mrev_s);
        let assign26640_e32290: f64 = (assign26640_e32289).abs();
        let assign26640_e32292: f64 = if assign26640_e32290 < 1e-6 { 1.0 } else { 0.0 };
        var_guard526 = assign26640_e32292;

        let (assign26650_e32306,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) && (var_guard524 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 != 0.0)) {
        (1.0,)
    } else {
        (var_m0flag_s,)
    }
};
        var_m0flag_s = assign26650_e32306;

        let (assign26660_e32330, assign26660_e32330_d_n6, assign26660_e32330_d_n7, assign26660_e32330_d_n8, assign26660_e32330_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) && (var_guard524 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 != 0.0)) {
        let assign26660_e32321: f64 = (1.0 / var_v3);
        let assign26660_e32324: f64 = (0.5 * var_phitdinv);
        let assign26660_e32326: f64 = (assign26660_e32324 * var_mrev_s);
        let assign26660_e32327: f64 = (assign26660_e32321 + assign26660_e32326);
        let assign26660_e32328: f64 = (var_i3_cor * assign26660_e32327);
        (assign26660_e32328, ((var_i3_cor_dn6 * assign26660_e32327) + (var_i3_cor * (assign26660_e32324 * var_mrev_s_dn6))), ((var_i3_cor_dn7 * assign26660_e32327) + (var_i3_cor * (assign26660_e32324 * var_mrev_s_dn7))), ((var_i3_cor_dn8 * assign26660_e32327) + (var_i3_cor * (assign26660_e32324 * var_mrev_s_dn8))), ((var_i3_cor_dn9 * assign26660_e32327) + (var_i3_cor * (assign26660_e32324 * var_mrev_s_dn9))),)
    } else {
        (var_isatrev_s, var_isatrev_s_dn6, var_isatrev_s_dn7, var_isatrev_s_dn8, var_isatrev_s_dn9,)
    }
};
        var_isatrev_s = assign26660_e32330;
        var_isatrev_s_dn6 = assign26660_e32330_d_n6;
        var_isatrev_s_dn7 = assign26660_e32330_d_n7;
        var_isatrev_s_dn8 = assign26660_e32330_d_n8;
        var_isatrev_s_dn9 = assign26660_e32330_d_n9;

        let (assign26670_e32353, assign26670_e32353_d_n6, assign26670_e32353_d_n7, assign26670_e32353_d_n8, assign26670_e32353_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) && (var_guard524 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 != 0.0)) {
        let assign26670_e32343: f64 = (-0.5);
        let assign26670_e32345: f64 = (assign26670_e32343 * var_i3_cor);
        let assign26670_e32347: f64 = (assign26670_e32345 * var_mrev_s);
        let assign26670_e32349: f64 = (assign26670_e32347 * var_phitdinv);
        let assign26670_e32351: f64 = (assign26670_e32349 / var_v3);
        (assign26670_e32351, (((((assign26670_e32343 * var_i3_cor_dn6) * var_mrev_s) + (assign26670_e32345 * var_mrev_s_dn6)) * var_phitdinv) / var_v3), (((((assign26670_e32343 * var_i3_cor_dn7) * var_mrev_s) + (assign26670_e32345 * var_mrev_s_dn7)) * var_phitdinv) / var_v3), (((((assign26670_e32343 * var_i3_cor_dn8) * var_mrev_s) + (assign26670_e32345 * var_mrev_s_dn8)) * var_phitdinv) / var_v3), (((((assign26670_e32343 * var_i3_cor_dn9) * var_mrev_s) + (assign26670_e32345 * var_mrev_s_dn9)) * var_phitdinv) / var_v3),)
    } else {
        (var_mrev_s, var_mrev_s_dn6, var_mrev_s_dn7, var_mrev_s_dn8, var_mrev_s_dn9,)
    }
};
        var_mrev_s = assign26670_e32353;
        var_mrev_s_dn6 = assign26670_e32353_d_n6;
        var_mrev_s_dn7 = assign26670_e32353_d_n7;
        var_mrev_s_dn8 = assign26670_e32353_d_n8;
        var_mrev_s_dn9 = assign26670_e32353_d_n9;

        let (assign26680_e32368,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) && (var_guard524 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 == 0.0)) {
        (0.0,)
    } else {
        (var_m0flag_s,)
    }
};
        var_m0flag_s = assign26680_e32368;

        let (assign26690_e32394, assign26690_e32394_d_n6, assign26690_e32394_d_n7, assign26690_e32394_d_n8, assign26690_e32394_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard521 != 0.0)) && (var_guard524 != 0.0)) && (var_guard525 != 0.0)) && (var_guard526 == 0.0)) {
        let assign26690_e32382: f64 = (-var_i3_cor);
        let assign26690_e32384: f64 = (-var_v3);
        let assign26690_e32386: f64 = (assign26690_e32384 * var_phitdinv);
        let assign26690_e32388: f64 = (assign26690_e32386 * var_mrev_s);
        let assign26690_e32389: f64 = (assign26690_e32388).exp();
        let assign26690_e32391: f64 = (assign26690_e32389 - 1.0);
        let assign26690_e32392: f64 = (assign26690_e32382 / assign26690_e32391);
        (assign26690_e32392, ((((-var_i3_cor_dn6) * assign26690_e32391) - (assign26690_e32382 * (assign26690_e32389 * (assign26690_e32386 * var_mrev_s_dn6)))) / (assign26690_e32391 * assign26690_e32391)), ((((-var_i3_cor_dn7) * assign26690_e32391) - (assign26690_e32382 * (assign26690_e32389 * (assign26690_e32386 * var_mrev_s_dn7)))) / (assign26690_e32391 * assign26690_e32391)), ((((-var_i3_cor_dn8) * assign26690_e32391) - (assign26690_e32382 * (assign26690_e32389 * (assign26690_e32386 * var_mrev_s_dn8)))) / (assign26690_e32391 * assign26690_e32391)), ((((-var_i3_cor_dn9) * assign26690_e32391) - (assign26690_e32382 * (assign26690_e32389 * (assign26690_e32386 * var_mrev_s_dn9)))) / (assign26690_e32391 * assign26690_e32391)),)
    } else {
        (var_isatrev_s, var_isatrev_s_dn6, var_isatrev_s_dn7, var_isatrev_s_dn8, var_isatrev_s_dn9,)
    }
};
        var_isatrev_s = assign26690_e32394;
        var_isatrev_s_dn6 = assign26690_e32394_d_n6;
        var_isatrev_s_dn7 = assign26690_e32394_d_n7;
        var_isatrev_s_dn8 = assign26690_e32394_d_n8;
        var_isatrev_s_dn9 = assign26690_e32394_d_n9;

        let (assign26700_e32412,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign26700_e32401: f64 = (var_absource_i * var_cjobot);
        let assign26700_e32404: f64 = (var_lssource_i * var_cjosti);
        let assign26700_e32405: f64 = (assign26700_e32401 + assign26700_e32404);
        let assign26700_e32408: f64 = (var_lgsource_i * var_cjogat);
        let assign26700_e32409: f64 = (assign26700_e32405 + assign26700_e32408);
        let assign26700_e32410: f64 = (p.p946 * assign26700_e32409);
        (assign26700_e32410,)
    } else {
        (var_zfrac,)
    }
};
        var_zfrac = assign26700_e32412;

        let assign26710_e32415: f64 = (var_absource_i * var_cjobot);
        let assign26710_e32417: f64 = if assign26710_e32415 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard527 = assign26710_e32417;

        let (assign26720_e32425,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard527 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagbot_s,)
    }
};
        var_zflagbot_s = assign26720_e32425;

        let assign26730_e32428: f64 = (var_lssource_i * var_cjosti);
        let assign26730_e32430: f64 = if assign26730_e32428 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard528 = assign26730_e32430;

        let (assign26740_e32438,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard528 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagsti_s,)
    }
};
        var_zflagsti_s = assign26740_e32438;

        let assign26750_e32441: f64 = (var_lgsource_i * var_cjogat);
        let assign26750_e32443: f64 = if assign26750_e32441 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard529 = assign26750_e32443;

        let (assign26760_e32451,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard529 != 0.0)) {
        (0.0,)
    } else {
        (var_zflaggat_s,)
    }
};
        var_zflaggat_s = assign26760_e32451;

        let assign26770_e32463: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard530 = assign26770_e32463;

        let (assign26780_e32478,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard530 != 0.0)) {
        let assign26780_e32471: f64 = (0.5 * p.p839);
        let assign26780_e32474: f64 = (var_isatfor1_s + 1e-21);
        let assign26780_e32475: f64 = (assign26780_e32471 / assign26780_e32474);
        let assign26780_e32476: f64 = (assign26780_e32475).ln();
        (assign26780_e32476,)
    } else {
        (var_xhighf1_s,)
    }
};
        var_xhighf1_s = assign26780_e32478;

        let (assign26790_e32493, assign26790_e32493_d_n6, assign26790_e32493_d_n7, assign26790_e32493_d_n8, assign26790_e32493_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard530 != 0.0)) {
        let assign26790_e32486: f64 = (0.5 * p.p839);
        let assign26790_e32489: f64 = (var_isatfor2_s + 1e-21);
        let assign26790_e32490: f64 = (assign26790_e32486 / assign26790_e32489);
        let assign26790_e32491: f64 = (assign26790_e32490).ln();
        (assign26790_e32491, ((-((assign26790_e32486 * var_isatfor2_s_dn6) / (assign26790_e32489 * assign26790_e32489))) / assign26790_e32490), ((-((assign26790_e32486 * var_isatfor2_s_dn7) / (assign26790_e32489 * assign26790_e32489))) / assign26790_e32490), ((-((assign26790_e32486 * var_isatfor2_s_dn8) / (assign26790_e32489 * assign26790_e32489))) / assign26790_e32490), ((-((assign26790_e32486 * var_isatfor2_s_dn9) / (assign26790_e32489 * assign26790_e32489))) / assign26790_e32490),)
    } else {
        (var_xhighf2_s, var_xhighf2_s_dn6, var_xhighf2_s_dn7, var_xhighf2_s_dn8, var_xhighf2_s_dn9,)
    }
};
        var_xhighf2_s = assign26790_e32493;
        var_xhighf2_s_dn6 = assign26790_e32493_d_n6;
        var_xhighf2_s_dn7 = assign26790_e32493_d_n7;
        var_xhighf2_s_dn8 = assign26790_e32493_d_n8;
        var_xhighf2_s_dn9 = assign26790_e32493_d_n9;

        let (assign26800_e32509, assign26800_e32509_d_n6, assign26800_e32509_d_n7, assign26800_e32509_d_n8, assign26800_e32509_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard530 != 0.0)) {
        let assign26800_e32501: f64 = (0.5 * p.p839);
        let assign26800_e32503: f64 = (var_isatrev_s).abs();
        let assign26800_e32505: f64 = (assign26800_e32503 + 1e-21);
        let assign26800_e32506: f64 = (assign26800_e32501 / assign26800_e32505);
        let assign26800_e32507: f64 = (assign26800_e32506).ln();
        (assign26800_e32507, ((-((assign26800_e32501 * if var_isatrev_s >= 0.0 { var_isatrev_s_dn6 } else { (-var_isatrev_s_dn6) }) / (assign26800_e32505 * assign26800_e32505))) / assign26800_e32506), ((-((assign26800_e32501 * if var_isatrev_s >= 0.0 { var_isatrev_s_dn7 } else { (-var_isatrev_s_dn7) }) / (assign26800_e32505 * assign26800_e32505))) / assign26800_e32506), ((-((assign26800_e32501 * if var_isatrev_s >= 0.0 { var_isatrev_s_dn8 } else { (-var_isatrev_s_dn8) }) / (assign26800_e32505 * assign26800_e32505))) / assign26800_e32506), ((-((assign26800_e32501 * if var_isatrev_s >= 0.0 { var_isatrev_s_dn9 } else { (-var_isatrev_s_dn9) }) / (assign26800_e32505 * assign26800_e32505))) / assign26800_e32506),)
    } else {
        (var_xhighr_s, var_xhighr_s_dn6, var_xhighr_s_dn7, var_xhighr_s_dn8, var_xhighr_s_dn9,)
    }
};
        var_xhighr_s = assign26800_e32509;
        var_xhighr_s_dn6 = assign26800_e32509_d_n6;
        var_xhighr_s_dn7 = assign26800_e32509_d_n7;
        var_xhighr_s_dn8 = assign26800_e32509_d_n8;
        var_xhighr_s_dn9 = assign26800_e32509_d_n9;

        let (assign26810_e32517,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign26810_e32515: f64 = (var_xhighf1_s).min(230.25850929940458);
        (assign26810_e32515,)
    } else {
        (var_xhighf1_s,)
    }
};
        var_xhighf1_s = assign26810_e32517;

        let (assign26820_e32524,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign26820_e32522: f64 = (var_xhighf1_s).exp();
        (assign26820_e32522,)
    } else {
        (var_expxhf1_s,)
    }
};
        var_expxhf1_s = assign26820_e32524;

        let (assign26830_e32532, assign26830_e32532_d_n6, assign26830_e32532_d_n7, assign26830_e32532_d_n8, assign26830_e32532_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign26830_e32530: f64 = (var_xhighf2_s).min(230.25850929940458);
        (assign26830_e32530, if var_xhighf2_s <= 230.25850929940458 { var_xhighf2_s_dn6 } else { 0.0 }, if var_xhighf2_s <= 230.25850929940458 { var_xhighf2_s_dn7 } else { 0.0 }, if var_xhighf2_s <= 230.25850929940458 { var_xhighf2_s_dn8 } else { 0.0 }, if var_xhighf2_s <= 230.25850929940458 { var_xhighf2_s_dn9 } else { 0.0 },)
    } else {
        (var_xhighf2_s, var_xhighf2_s_dn6, var_xhighf2_s_dn7, var_xhighf2_s_dn8, var_xhighf2_s_dn9,)
    }
};
        var_xhighf2_s = assign26830_e32532;
        var_xhighf2_s_dn6 = assign26830_e32532_d_n6;
        var_xhighf2_s_dn7 = assign26830_e32532_d_n7;
        var_xhighf2_s_dn8 = assign26830_e32532_d_n8;
        var_xhighf2_s_dn9 = assign26830_e32532_d_n9;

        let (assign26840_e32539, assign26840_e32539_d_n6, assign26840_e32539_d_n7, assign26840_e32539_d_n8, assign26840_e32539_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign26840_e32537: f64 = (var_xhighf2_s).exp();
        (assign26840_e32537, (assign26840_e32537 * var_xhighf2_s_dn6), (assign26840_e32537 * var_xhighf2_s_dn7), (assign26840_e32537 * var_xhighf2_s_dn8), (assign26840_e32537 * var_xhighf2_s_dn9),)
    } else {
        (var_expxhf2_s, var_expxhf2_s_dn6, var_expxhf2_s_dn7, var_expxhf2_s_dn8, var_expxhf2_s_dn9,)
    }
};
        var_expxhf2_s = assign26840_e32539;
        var_expxhf2_s_dn6 = assign26840_e32539_d_n6;
        var_expxhf2_s_dn7 = assign26840_e32539_d_n7;
        var_expxhf2_s_dn8 = assign26840_e32539_d_n8;
        var_expxhf2_s_dn9 = assign26840_e32539_d_n9;

        let (assign26850_e32547, assign26850_e32547_d_n6, assign26850_e32547_d_n7, assign26850_e32547_d_n8, assign26850_e32547_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign26850_e32545: f64 = (var_xhighr_s).min(230.25850929940458);
        (assign26850_e32545, if var_xhighr_s <= 230.25850929940458 { var_xhighr_s_dn6 } else { 0.0 }, if var_xhighr_s <= 230.25850929940458 { var_xhighr_s_dn7 } else { 0.0 }, if var_xhighr_s <= 230.25850929940458 { var_xhighr_s_dn8 } else { 0.0 }, if var_xhighr_s <= 230.25850929940458 { var_xhighr_s_dn9 } else { 0.0 },)
    } else {
        (var_xhighr_s, var_xhighr_s_dn6, var_xhighr_s_dn7, var_xhighr_s_dn8, var_xhighr_s_dn9,)
    }
};
        var_xhighr_s = assign26850_e32547;
        var_xhighr_s_dn6 = assign26850_e32547_d_n6;
        var_xhighr_s_dn7 = assign26850_e32547_d_n7;
        var_xhighr_s_dn8 = assign26850_e32547_d_n8;
        var_xhighr_s_dn9 = assign26850_e32547_d_n9;

        let (assign26860_e32554, assign26860_e32554_d_n6, assign26860_e32554_d_n7, assign26860_e32554_d_n8, assign26860_e32554_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign26860_e32552: f64 = (var_xhighr_s).exp();
        (assign26860_e32552, (assign26860_e32552 * var_xhighr_s_dn6), (assign26860_e32552 * var_xhighr_s_dn7), (assign26860_e32552 * var_xhighr_s_dn8), (assign26860_e32552 * var_xhighr_s_dn9),)
    } else {
        (var_expxhr_s, var_expxhr_s_dn6, var_expxhr_s_dn7, var_expxhr_s_dn8, var_expxhr_s_dn9,)
    }
};
        var_expxhr_s = assign26860_e32554;
        var_expxhr_s_dn6 = assign26860_e32554_d_n6;
        var_expxhr_s_dn7 = assign26860_e32554_d_n7;
        var_expxhr_s_dn8 = assign26860_e32554_d_n8;
        var_expxhr_s_dn9 = assign26860_e32554_d_n9;

        let (assign26870_e32560,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.4,)
    } else {
        (var_fracna,)
    }
};
        var_fracna = assign26870_e32560;

        let (assign26880_e32566,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.65,)
    } else {
        (var_fracnb,)
    }
};
        var_fracnb = assign26880_e32566;

        let (assign26890_e32572,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.8,)
    } else {
        (var_fraci,)
    }
};
        var_fraci = assign26890_e32572;

        let (assign26900_e32581,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign26900_e32577: f64 = (-var_fracna);
        let assign26900_e32579: f64 = (assign26900_e32577 * var_vjunrefd_i);
        (assign26900_e32579,)
    } else {
        (var_v1,)
    }
};
        var_v1 = assign26900_e32581;

        let (assign26910_e32590,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign26910_e32586: f64 = (-var_fracnb);
        let assign26910_e32588: f64 = (assign26910_e32586 * var_vjunrefd_i);
        (assign26910_e32588,)
    } else {
        (var_v2,)
    }
};
        var_v2 = assign26910_e32590;

        let (assign26920_e32599,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign26920_e32595: f64 = (-var_fraci);
        let assign26920_e32597: f64 = (assign26920_e32595 * var_vjunrefd_i);
        (assign26920_e32597,)
    } else {
        (var_v3,)
    }
};
        var_v3 = assign26920_e32599;

        let (assign26930_e32605,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.1,)
    } else {
        (var_v4,)
    }
};
        var_v4 = assign26930_e32605;

        *var_alphaje_slot = var_alphaje;
        *var_alphaje_dn6_slot = var_alphaje_dn6;
        *var_alphaje_dn7_slot = var_alphaje_dn7;
        *var_alphaje_dn8_slot = var_alphaje_dn8;
        *var_alphaje_dn9_slot = var_alphaje_dn9;
        *var_expxhf1_s_slot = var_expxhf1_s;
        *var_expxhf2_s_slot = var_expxhf2_s;
        *var_expxhf2_s_dn6_slot = var_expxhf2_s_dn6;
        *var_expxhf2_s_dn7_slot = var_expxhf2_s_dn7;
        *var_expxhf2_s_dn8_slot = var_expxhf2_s_dn8;
        *var_expxhf2_s_dn9_slot = var_expxhf2_s_dn9;
        *var_expxhr_s_slot = var_expxhr_s;
        *var_expxhr_s_dn6_slot = var_expxhr_s_dn6;
        *var_expxhr_s_dn7_slot = var_expxhr_s_dn7;
        *var_expxhr_s_dn8_slot = var_expxhr_s_dn8;
        *var_expxhr_s_dn9_slot = var_expxhr_s_dn9;
        *var_fraci_slot = var_fraci;
        *var_fracna_slot = var_fracna;
        *var_fracnb_slot = var_fracnb;
        *var_guard524_slot = var_guard524;
        *var_guard525_slot = var_guard525;
        *var_guard526_slot = var_guard526;
        *var_guard527_slot = var_guard527;
        *var_guard528_slot = var_guard528;
        *var_guard529_slot = var_guard529;
        *var_guard530_slot = var_guard530;
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
        *var_isatrev_s_slot = var_isatrev_s;
        *var_isatrev_s_dn6_slot = var_isatrev_s_dn6;
        *var_isatrev_s_dn7_slot = var_isatrev_s_dn7;
        *var_isatrev_s_dn8_slot = var_isatrev_s_dn8;
        *var_isatrev_s_dn9_slot = var_isatrev_s_dn9;
        *var_m0_rev_slot = var_m0_rev;
        *var_m0_rev_dn6_slot = var_m0_rev_dn6;
        *var_m0_rev_dn7_slot = var_m0_rev_dn7;
        *var_m0_rev_dn8_slot = var_m0_rev_dn8;
        *var_m0_rev_dn9_slot = var_m0_rev_dn9;
        *var_m0flag_s_slot = var_m0flag_s;
        *var_mcor_rev_slot = var_mcor_rev;
        *var_mcor_rev_dn6_slot = var_mcor_rev_dn6;
        *var_mcor_rev_dn7_slot = var_mcor_rev_dn7;
        *var_mcor_rev_dn8_slot = var_mcor_rev_dn8;
        *var_mcor_rev_dn9_slot = var_mcor_rev_dn9;
        *var_mrev_s_slot = var_mrev_s;
        *var_mrev_s_dn6_slot = var_mrev_s_dn6;
        *var_mrev_s_dn7_slot = var_mrev_s_dn7;
        *var_mrev_s_dn8_slot = var_mrev_s_dn8;
        *var_mrev_s_dn9_slot = var_mrev_s_dn9;
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
        *var_v1_slot = var_v1;
        *var_v2_slot = var_v2;
        *var_v3_slot = var_v3;
        *var_v4_slot = var_v4;
        *var_xhighf1_s_slot = var_xhighf1_s;
        *var_xhighf2_s_slot = var_xhighf2_s;
        *var_xhighf2_s_dn6_slot = var_xhighf2_s_dn6;
        *var_xhighf2_s_dn7_slot = var_xhighf2_s_dn7;
        *var_xhighf2_s_dn8_slot = var_xhighf2_s_dn8;
        *var_xhighf2_s_dn9_slot = var_xhighf2_s_dn9;
        *var_xhighr_s_slot = var_xhighr_s;
        *var_xhighr_s_dn6_slot = var_xhighr_s_dn6;
        *var_xhighr_s_dn7_slot = var_xhighr_s_dn7;
        *var_xhighr_s_dn8_slot = var_xhighr_s_dn8;
        *var_xhighr_s_dn9_slot = var_xhighr_s_dn9;
        *var_zflagbot_s_slot = var_zflagbot_s;
        *var_zflaggat_s_slot = var_zflaggat_s;
        *var_zflagsti_s_slot = var_zflagsti_s;
        *var_zfrac_slot = var_zfrac;
    }

    pub(super) fn stamp_transient_block_53(
        var_abdrain_i: f64,
        var_btatpartbot_d: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_exp_vmax_over_phitd_d: f64,
        var_ftdbot_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_idsatbot_d: f64,
        var_lgdrain_i: f64,
        var_lsdrain_i: f64,
        var_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_v1: f64,
        var_vbbtlim_d: f64,
        var_vbibot_d: f64,
        var_vbimin_d: f64,
        var_vbirbotinv_d: f64,
        var_vmax_d: f64,
        var_wdepnulrbot_d: f64,
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
        var_guard531_slot: &mut f64,
        var_guard532_slot: &mut f64,
        var_guard533_slot: &mut f64,
        var_guard534_slot: &mut f64,
        var_guard535_slot: &mut f64,
        var_guard536_slot: &mut f64,
        var_guard537_slot: &mut f64,
        var_guard538_slot: &mut f64,
        var_guard539_slot: &mut f64,
        var_guard540_slot: &mut f64,
        var_id__blk212_slot: &mut f64,
        var_idmult_slot: &mut f64,
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
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_v5_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wdep_dn9_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
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
        let mut var_guard531: f64 = *var_guard531_slot;
        let mut var_guard532: f64 = *var_guard532_slot;
        let mut var_guard533: f64 = *var_guard533_slot;
        let mut var_guard534: f64 = *var_guard534_slot;
        let mut var_guard535: f64 = *var_guard535_slot;
        let mut var_guard536: f64 = *var_guard536_slot;
        let mut var_guard537: f64 = *var_guard537_slot;
        let mut var_guard538: f64 = *var_guard538_slot;
        let mut var_guard539: f64 = *var_guard539_slot;
        let mut var_guard540: f64 = *var_guard540_slot;
        let mut var_id__blk212: f64 = *var_id__blk212_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
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
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_v5: f64 = *var_v5_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wdep_dn9: f64 = *var_wdep_dn9_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign26940_e32611,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.2,)
    } else {
        (var_v5,)
    }
};
        var_v5 = assign26940_e32611;

        let (assign26950_e32617,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign26950_e32617;

        let (assign26960_e32623,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign26960_e32623;

        let assign26970_e32635: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard531 = assign26970_e32635;

        let assign27050_e32721: f64 = if var_v1 < var_vmax_d { 1.0 } else { 0.0 };
        var_guard532 = assign27050_e32721;

        let assign27060_e32723: f64 = (-0.5);
        let assign27060_e32726: f64 = (var_v1 * var_phitdinv);
        let assign27060_e32727: f64 = (assign27060_e32723 * assign27060_e32726);
        let assign27060_e32728: f64 = (assign27060_e32727).abs();
        let assign27060_e32730: f64 = if assign27060_e32728 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard533 = assign27060_e32730;

        let (assign27070_e32748,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) && (var_guard533 != 0.0)) {
        let assign27070_e32741: f64 = (-0.5);
        let assign27070_e32744: f64 = (var_v1 * var_phitdinv);
        let assign27070_e32745: f64 = (assign27070_e32741 * assign27070_e32744);
        let assign27070_e32746: f64 = (assign27070_e32745).exp();
        (assign27070_e32746,)
    } else {
        (var_z,)
    }
};
        var_z = assign27070_e32748;

        let assign27080_e32750: f64 = (-0.5);
        let assign27080_e32753: f64 = (var_v1 * var_phitdinv);
        let assign27080_e32754: f64 = (assign27080_e32750 * assign27080_e32753);
        let assign27080_e32756: f64 = if assign27080_e32754 < 0.0 { 1.0 } else { 0.0 };
        var_guard534 = assign27080_e32756;

        let (assign27090_e32811,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) && (var_guard533 == 0.0)) && (var_guard534 != 0.0)) {
        let assign27090_e32772: f64 = (-230.25850929940458);
        let assign27090_e32774: f64 = (-0.5);
        let assign27090_e32777: f64 = (var_v1 * var_phitdinv);
        let assign27090_e32778: f64 = (assign27090_e32774 * assign27090_e32777);
        let assign27090_e32779: f64 = (assign27090_e32772 - assign27090_e32778);
        let assign27090_e32783: f64 = (-230.25850929940458);
        let assign27090_e32785: f64 = (-0.5);
        let assign27090_e32788: f64 = (var_v1 * var_phitdinv);
        let assign27090_e32789: f64 = (assign27090_e32785 * assign27090_e32788);
        let assign27090_e32790: f64 = (assign27090_e32783 - assign27090_e32789);
        let assign27090_e32793: f64 = (-230.25850929940458);
        let assign27090_e32795: f64 = (-0.5);
        let assign27090_e32798: f64 = (var_v1 * var_phitdinv);
        let assign27090_e32799: f64 = (assign27090_e32795 * assign27090_e32798);
        let assign27090_e32800: f64 = (assign27090_e32793 - assign27090_e32799);
        let assign27090_e32802: f64 = (assign27090_e32800 * 0.3333333333333333);
        let assign27090_e32803: f64 = (1.0 + assign27090_e32802);
        let assign27090_e32804: f64 = (assign27090_e32790 * assign27090_e32803);
        let assign27090_e32805: f64 = (0.5 * assign27090_e32804);
        let assign27090_e32806: f64 = (1.0 + assign27090_e32805);
        let assign27090_e32807: f64 = (assign27090_e32779 * assign27090_e32806);
        let assign27090_e32808: f64 = (1.0 + assign27090_e32807);
        let assign27090_e32809: f64 = (1e-100 / assign27090_e32808);
        (assign27090_e32809,)
    } else {
        (var_z,)
    }
};
        var_z = assign27090_e32811;

        let (assign27100_e32864,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) && (var_guard533 == 0.0)) && (var_guard534 == 0.0)) {
        let assign27100_e32828: f64 = (-0.5);
        let assign27100_e32831: f64 = (var_v1 * var_phitdinv);
        let assign27100_e32832: f64 = (assign27100_e32828 * assign27100_e32831);
        let assign27100_e32834: f64 = (assign27100_e32832 - 230.25850929940458);
        let assign27100_e32838: f64 = (-0.5);
        let assign27100_e32841: f64 = (var_v1 * var_phitdinv);
        let assign27100_e32842: f64 = (assign27100_e32838 * assign27100_e32841);
        let assign27100_e32844: f64 = (assign27100_e32842 - 230.25850929940458);
        let assign27100_e32847: f64 = (-0.5);
        let assign27100_e32850: f64 = (var_v1 * var_phitdinv);
        let assign27100_e32851: f64 = (assign27100_e32847 * assign27100_e32850);
        let assign27100_e32853: f64 = (assign27100_e32851 - 230.25850929940458);
        let assign27100_e32855: f64 = (assign27100_e32853 * 0.3333333333333333);
        let assign27100_e32856: f64 = (1.0 + assign27100_e32855);
        let assign27100_e32857: f64 = (assign27100_e32844 * assign27100_e32856);
        let assign27100_e32858: f64 = (0.5 * assign27100_e32857);
        let assign27100_e32859: f64 = (1.0 + assign27100_e32858);
        let assign27100_e32860: f64 = (assign27100_e32834 * assign27100_e32859);
        let assign27100_e32861: f64 = (1.0 + assign27100_e32860);
        let assign27100_e32862: f64 = (1e100 * assign27100_e32861);
        (assign27100_e32862,)
    } else {
        (var_z,)
    }
};
        var_z = assign27100_e32864;

        let (assign27110_e32876,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) {
        let assign27110_e32874: f64 = (1.0 / var_z);
        (assign27110_e32874,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign27110_e32876;

        let (assign27120_e32888,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 != 0.0)) {
        let assign27120_e32886: f64 = (var_zinv * var_zinv);
        (assign27120_e32886,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign27120_e32888;

        let (assign27130_e32907,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 == 0.0)) {
        let assign27130_e32900: f64 = (var_v1 - var_vmax_d);
        let assign27130_e32902: f64 = (assign27130_e32900 * var_phitdinv);
        let assign27130_e32903: f64 = (1.0 + assign27130_e32902);
        let assign27130_e32905: f64 = (assign27130_e32903 * var_exp_vmax_over_phitd_d);
        (assign27130_e32905,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign27130_e32907;

        let (assign27140_e32919,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 == 0.0)) {
        let assign27140_e32917: f64 = (var_idmult).sqrt();
        (assign27140_e32917,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign27140_e32919;

        let (assign27150_e32932,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard531 != 0.0)) && (var_guard532 == 0.0)) {
        let assign27150_e32930: f64 = (1.0 / var_zinv);
        (assign27150_e32930,)
    } else {
        (var_z,)
    }
};
        var_z = assign27150_e32932;

        let (assign27160_e32942,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard531 != 0.0)) {
        let assign27160_e32940: f64 = (var_idmult - 1.0);
        (assign27160_e32940,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign27160_e32942;

        let assign27170_e32945: f64 = if var_v1 > 0.0 { 1.0 } else { 0.0 };
        var_guard535 = assign27170_e32945;

        let (assign27180_e32971,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard531 != 0.0)) && (var_guard535 != 0.0)) {
        let assign27180_e32957: f64 = (2.0 + var_z);
        let assign27180_e32960: f64 = (var_z + 1.0);
        let assign27180_e32963: f64 = (var_z + 3.0);
        let assign27180_e32964: f64 = (assign27180_e32960 * assign27180_e32963);
        let assign27180_e32965: f64 = (assign27180_e32964).sqrt();
        let assign27180_e32966: f64 = (assign27180_e32957 + assign27180_e32965);
        let assign27180_e32967: f64 = (assign27180_e32966).ln();
        let assign27180_e32968: f64 = (var_phitd * assign27180_e32967);
        let assign27180_e32969: f64 = (2.0 * assign27180_e32968);
        (assign27180_e32969,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign27180_e32971;

        let (assign27190_e33005,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard531 != 0.0)) && (var_guard535 == 0.0)) {
        let assign27190_e32981: f64 = (-var_v1);
        let assign27190_e32986: f64 = (2.0 * var_zinv);
        let assign27190_e32988: f64 = (assign27190_e32986 + 1.0);
        let assign27190_e32991: f64 = (1.0 + var_zinv);
        let assign27190_e32995: f64 = (3.0 * var_zinv);
        let assign27190_e32996: f64 = (1.0 + assign27190_e32995);
        let assign27190_e32997: f64 = (assign27190_e32991 * assign27190_e32996);
        let assign27190_e32998: f64 = (assign27190_e32997).sqrt();
        let assign27190_e32999: f64 = (assign27190_e32988 + assign27190_e32998);
        let assign27190_e33000: f64 = (assign27190_e32999).ln();
        let assign27190_e33001: f64 = (var_phitd * assign27190_e33000);
        let assign27190_e33002: f64 = (2.0 * assign27190_e33001);
        let assign27190_e33003: f64 = (assign27190_e32981 + assign27190_e33002);
        (assign27190_e33003,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign27190_e33005;

        let (assign27200_e33015,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard531 != 0.0)) {
        let assign27200_e33013: f64 = (var_vbimin_d - var_two_psistar);
        (assign27200_e33013,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign27200_e33015;

        let (assign27210_e33042,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard531 != 0.0)) {
        let assign27210_e33024: f64 = (var_v1 + var_vjlim);
        let assign27210_e33027: f64 = (var_v1 - var_vjlim);
        let assign27210_e33030: f64 = (var_v1 - var_vjlim);
        let assign27210_e33031: f64 = (assign27210_e33027 * assign27210_e33030);
        let assign27210_e33034: f64 = (4.0 * var_phitd);
        let assign27210_e33036: f64 = (assign27210_e33034 * var_phitd);
        let assign27210_e33037: f64 = (assign27210_e33031 + assign27210_e33036);
        let assign27210_e33038: f64 = (assign27210_e33037).sqrt();
        let assign27210_e33039: f64 = (assign27210_e33024 - assign27210_e33038);
        let assign27210_e33040: f64 = (0.5 * assign27210_e33039);
        (assign27210_e33040,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign27210_e33042;

        let (assign27220_e33069,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard531 != 0.0)) {
        let assign27220_e33051: f64 = (var_v1 + var_vbbtlim_d);
        let assign27220_e33054: f64 = (var_v1 - var_vbbtlim_d);
        let assign27220_e33057: f64 = (var_v1 - var_vbbtlim_d);
        let assign27220_e33058: f64 = (assign27220_e33054 * assign27220_e33057);
        let assign27220_e33061: f64 = (4.0 * var_phitr);
        let assign27220_e33063: f64 = (assign27220_e33061 * var_phitr);
        let assign27220_e33064: f64 = (assign27220_e33058 + assign27220_e33063);
        let assign27220_e33065: f64 = (assign27220_e33064).sqrt();
        let assign27220_e33066: f64 = (assign27220_e33051 - assign27220_e33065);
        let assign27220_e33067: f64 = (0.5 * assign27220_e33066);
        (assign27220_e33067,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign27220_e33069;

        let (assign27230_e33096,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard531 != 0.0)) {
        let assign27230_e33078: f64 = var_v1;
        let assign27230_e33081: f64 = var_v1;
        let assign27230_e33084: f64 = var_v1;
        let assign27230_e33085: f64 = (assign27230_e33081 * assign27230_e33084);
        let assign27230_e33088: f64 = (4.0 * 1e-6);
        let assign27230_e33090: f64 = (assign27230_e33088 * 1e-6);
        let assign27230_e33091: f64 = (assign27230_e33085 + assign27230_e33090);
        let assign27230_e33092: f64 = (assign27230_e33091).sqrt();
        let assign27230_e33093: f64 = (assign27230_e33078 - assign27230_e33092);
        let assign27230_e33094: f64 = (0.5 * assign27230_e33093);
        (assign27230_e33094,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign27230_e33096;

        let assign27240_e33099: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard536 = assign27240_e33099;

        let (assign27250_e33107, assign27250_e33107_d_n6, assign27250_e33107_d_n7, assign27250_e33107_d_n8, assign27250_e33107_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign27250_e33107;
        var_ijunbot_dn6 = assign27250_e33107_d_n6;
        var_ijunbot_dn7 = assign27250_e33107_d_n7;
        var_ijunbot_dn8 = assign27250_e33107_d_n8;
        var_ijunbot_dn9 = assign27250_e33107_d_n9;

        let (assign27260_e33118,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) {
        let assign27260_e33116: f64 = (var_idsatbot_d * var_idmult);
        (assign27260_e33116,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign27260_e33118;

        let assign27270_e33125: f64 = if ((var_csrhbotd_i == 0.0) && (var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard537 = assign27270_e33125;

        let (assign27280_e33136, assign27280_e33136_d_n6, assign27280_e33136_d_n7, assign27280_e33136_d_n8, assign27280_e33136_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard537 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign27280_e33136;
        var_isrh_dn6 = assign27280_e33136_d_n6;
        var_isrh_dn7 = assign27280_e33136_d_n7;
        var_isrh_dn8 = assign27280_e33136_d_n8;
        var_isrh_dn9 = assign27280_e33136_d_n9;

        let (assign27290_e33150,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard537 == 0.0)) {
        let assign27290_e33148: f64 = (var_vbibot_d - var_vjsrh);
        (assign27290_e33148,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign27290_e33150;

        let (assign27300_e33169,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard537 == 0.0)) {
        let assign27300_e33164: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign27300_e33165: f64 = (1.0 - assign27300_e33164);
        let assign27300_e33166: f64 = (assign27300_e33165).sqrt();
        let assign27300_e33167: f64 = (1.0 - assign27300_e33166);
        (assign27300_e33167,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign27300_e33169;

        let assign27310_e33172: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard538 = assign27310_e33172;

        let (assign27320_e33186,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard537 == 0.0)) && (var_guard538 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign27320_e33186;

        let (assign27330_e33218,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard537 == 0.0)) && (var_guard538 == 0.0)) {
        let assign27330_e33201: f64 = (var_wsrhstep * var_wsrhstep);
        let assign27330_e33203: f64 = (var_wsrhstep).ln();
        let assign27330_e33204: f64 = (assign27330_e33201 * assign27330_e33203);
        let assign27330_e33207: f64 = (1.0 - var_wsrhstep);
        let assign27330_e33208: f64 = (assign27330_e33204 / assign27330_e33207);
        let assign27330_e33210: f64 = (assign27330_e33208 + var_wsrhstep);
        let assign27330_e33214: f64 = (2.0 * var_pbotd_i);
        let assign27330_e33215: f64 = (1.0 - assign27330_e33214);
        let assign27330_e33216: f64 = (assign27330_e33210 * assign27330_e33215);
        (assign27330_e33216,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign27330_e33218;

        let (assign27340_e33232,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard537 == 0.0)) {
        let assign27340_e33230: f64 = (var_wsrhstep + var_dwsrh);
        (assign27340_e33230,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign27340_e33232;

        let assign27350_e33235: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard539 = assign27350_e33235;

        let (assign27360_e33252, assign27360_e33252_d_n6, assign27360_e33252_d_n7, assign27360_e33252_d_n8, assign27360_e33252_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard537 == 0.0)) && (var_guard539 != 0.0)) {
        let assign27360_e33249: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign27360_e33250: f64 = (assign27360_e33249).sqrt();
        (assign27360_e33250, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign27360_e33252;
        var_tmp_dn6 = assign27360_e33252_d_n6;
        var_tmp_dn7 = assign27360_e33252_d_n7;
        var_tmp_dn8 = assign27360_e33252_d_n8;
        var_tmp_dn9 = assign27360_e33252_d_n9;

        let (assign27370_e33271, assign27370_e33271_d_n6, assign27370_e33271_d_n7, assign27370_e33271_d_n8, assign27370_e33271_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard537 == 0.0)) && (var_guard539 == 0.0)) {
        let assign27370_e33267: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign27370_e33269: f64 = (assign27370_e33267).powf(var_pbotd_i);
        (assign27370_e33269, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign27370_e33271;
        var_tmp_dn6 = assign27370_e33271_d_n6;
        var_tmp_dn7 = assign27370_e33271_d_n7;
        var_tmp_dn8 = assign27370_e33271_d_n8;
        var_tmp_dn9 = assign27370_e33271_d_n9;

        let (assign27380_e33285, assign27380_e33285_d_n6, assign27380_e33285_d_n7, assign27380_e33285_d_n8, assign27380_e33285_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard537 == 0.0)) {
        let assign27380_e33283: f64 = (var_wdepnulrbot_d * var_tmp);
        (assign27380_e33283, (var_wdepnulrbot_d * var_tmp_dn6), (var_wdepnulrbot_d * var_tmp_dn7), (var_wdepnulrbot_d * var_tmp_dn8), (var_wdepnulrbot_d * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign27380_e33285;
        var_wdep_dn6 = assign27380_e33285_d_n6;
        var_wdep_dn7 = assign27380_e33285_d_n7;
        var_wdep_dn8 = assign27380_e33285_d_n8;
        var_wdep_dn9 = assign27380_e33285_d_n9;

        let (assign27390_e33303, assign27390_e33303_d_n6, assign27390_e33303_d_n7, assign27390_e33303_d_n8, assign27390_e33303_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard537 == 0.0)) {
        let assign27390_e33298: f64 = (var_zinv - 1.0);
        let assign27390_e33300: f64 = (assign27390_e33298 * var_wdep);
        let assign27390_e33301: f64 = (var_ftdbot_d * assign27390_e33300);
        (assign27390_e33301, (var_ftdbot_d * (assign27390_e33298 * var_wdep_dn6)), (var_ftdbot_d * (assign27390_e33298 * var_wdep_dn7)), (var_ftdbot_d * (assign27390_e33298 * var_wdep_dn8)), (var_ftdbot_d * (assign27390_e33298 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign27390_e33303;
        var_asrh_dn6 = assign27390_e33303_d_n6;
        var_asrh_dn7 = assign27390_e33303_d_n7;
        var_asrh_dn8 = assign27390_e33303_d_n8;
        var_asrh_dn9 = assign27390_e33303_d_n9;

        let (assign27400_e33319, assign27400_e33319_d_n6, assign27400_e33319_d_n7, assign27400_e33319_d_n8, assign27400_e33319_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard537 == 0.0)) {
        let assign27400_e33316: f64 = (var_asrh * var_wsrh);
        let assign27400_e33317: f64 = (var_csrhbotd_i * assign27400_e33316);
        (assign27400_e33317, (var_csrhbotd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn8 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign27400_e33319;
        var_isrh_dn6 = assign27400_e33319_d_n6;
        var_isrh_dn7 = assign27400_e33319_d_n7;
        var_isrh_dn8 = assign27400_e33319_d_n8;
        var_isrh_dn9 = assign27400_e33319_d_n9;

        let assign27410_e33322: f64 = if var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard540 = assign27410_e33322;

        let (assign27420_e33333, assign27420_e33333_d_n6, assign27420_e33333_d_n7, assign27420_e33333_d_n8, assign27420_e33333_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign27420_e33333;
        var_itat_dn6 = assign27420_e33333_d_n6;
        var_itat_dn7 = assign27420_e33333_d_n7;
        var_itat_dn8 = assign27420_e33333_d_n8;
        var_itat_dn9 = assign27420_e33333_d_n9;

        let (assign27430_e33351, assign27430_e33351_d_n6, assign27430_e33351_d_n7, assign27430_e33351_d_n8, assign27430_e33351_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) {
        let assign27430_e33346: f64 = (var_wdep * var_one_minus_pbot_d);
        let assign27430_e33348: f64 = (assign27430_e33346 / var_vbi_minus_vjsrh);
        let assign27430_e33349: f64 = (var_btatpartbot_d * assign27430_e33348);
        (assign27430_e33349, (var_btatpartbot_d * ((var_wdep_dn6 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn7 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn8 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn9 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign27430_e33351;
        var_btat_dn6 = assign27430_e33351_d_n6;
        var_btat_dn7 = assign27430_e33351_d_n7;
        var_btat_dn8 = assign27430_e33351_d_n8;
        var_btat_dn9 = assign27430_e33351_d_n9;

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
        *var_guard531_slot = var_guard531;
        *var_guard532_slot = var_guard532;
        *var_guard533_slot = var_guard533;
        *var_guard534_slot = var_guard534;
        *var_guard535_slot = var_guard535;
        *var_guard536_slot = var_guard536;
        *var_guard537_slot = var_guard537;
        *var_guard538_slot = var_guard538;
        *var_guard539_slot = var_guard539;
        *var_guard540_slot = var_guard540;
        *var_id__blk212_slot = var_id__blk212;
        *var_idmult_slot = var_idmult;
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
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_two_psistar_slot = var_two_psistar;
        *var_v5_slot = var_v5;
        *var_vav_slot = var_vav;
        *var_vbbt_slot = var_vbbt;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_vjlim_slot = var_vjlim;
        *var_vjsrh_slot = var_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wdep_dn9_slot = var_wdep_dn9;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_54(
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatbot_d: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_btat_dn9: f64,
        var_cbbtbotd_i: f64,
        var_cerfc: f64,
        var_ctatbotd_i: f64,
        var_fbbtbot_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard536: f64,
        var_guard540: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_perfc: f64,
        var_vbbt: f64,
        var_vbirbotd_i: f64,
        var_vbirbotinv_d: f64,
        var_wdepnulrinvbot_d: f64,
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
        var_guard541_slot: &mut f64,
        var_guard542_slot: &mut f64,
        var_guard543_slot: &mut f64,
        var_guard544_slot: &mut f64,
        var_guard545_slot: &mut f64,
        var_guard546_slot: &mut f64,
        var_guard547_slot: &mut f64,
        var_guard548_slot: &mut f64,
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
        let mut var_guard541: f64 = *var_guard541_slot;
        let mut var_guard542: f64 = *var_guard542_slot;
        let mut var_guard543: f64 = *var_guard543_slot;
        let mut var_guard544: f64 = *var_guard544_slot;
        let mut var_guard545: f64 = *var_guard545_slot;
        let mut var_guard546: f64 = *var_guard546_slot;
        let mut var_guard547: f64 = *var_guard547_slot;
        let mut var_guard548: f64 = *var_guard548_slot;
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

        let (assign27440_e33367, assign27440_e33367_d_n6, assign27440_e33367_d_n7, assign27440_e33367_d_n8, assign27440_e33367_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) {
        let assign27440_e33363: f64 = (0.666666666666667 * var_atatbot_d);
        let assign27440_e33365: f64 = (assign27440_e33363 / var_btat);
        (assign27440_e33365, (-((assign27440_e33363 * var_btat_dn6) / (var_btat * var_btat))), (-((assign27440_e33363 * var_btat_dn7) / (var_btat * var_btat))), (-((assign27440_e33363 * var_btat_dn8) / (var_btat * var_btat))), (-((assign27440_e33363 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign27440_e33367;
        var_twoatatoverthreebtat_dn6 = assign27440_e33367_d_n6;
        var_twoatatoverthreebtat_dn7 = assign27440_e33367_d_n7;
        var_twoatatoverthreebtat_dn8 = assign27440_e33367_d_n8;
        var_twoatatoverthreebtat_dn9 = assign27440_e33367_d_n9;

        let (assign27450_e33381, assign27450_e33381_d_n6, assign27450_e33381_d_n7, assign27450_e33381_d_n8, assign27450_e33381_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) {
        let assign27450_e33379: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign27450_e33379, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign27450_e33381;
        var_umaxbeforelimiting_dn6 = assign27450_e33381_d_n6;
        var_umaxbeforelimiting_dn7 = assign27450_e33381_d_n7;
        var_umaxbeforelimiting_dn8 = assign27450_e33381_d_n8;
        var_umaxbeforelimiting_dn9 = assign27450_e33381_d_n9;

        let (assign27460_e33402, assign27460_e33402_d_n6, assign27460_e33402_d_n7, assign27460_e33402_d_n8, assign27460_e33402_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) {
        let assign27460_e33393: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign27460_e33396: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign27460_e33398: f64 = (assign27460_e33396 + 1.0);
        let assign27460_e33399: f64 = (assign27460_e33393 / assign27460_e33398);
        let assign27460_e33400: f64 = (assign27460_e33399).sqrt();
        (assign27460_e33400, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign27460_e33398) - (assign27460_e33393 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign27460_e33398 * assign27460_e33398)) / (2.0 * assign27460_e33400)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign27460_e33398) - (assign27460_e33393 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign27460_e33398 * assign27460_e33398)) / (2.0 * assign27460_e33400)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign27460_e33398) - (assign27460_e33393 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign27460_e33398 * assign27460_e33398)) / (2.0 * assign27460_e33400)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign27460_e33398) - (assign27460_e33393 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign27460_e33398 * assign27460_e33398)) / (2.0 * assign27460_e33400)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign27460_e33402;
        var_umax_dn6 = assign27460_e33402_d_n6;
        var_umax_dn7 = assign27460_e33402_d_n7;
        var_umax_dn8 = assign27460_e33402_d_n8;
        var_umax_dn9 = assign27460_e33402_d_n9;

        let (assign27470_e33415, assign27470_e33415_d_n6, assign27470_e33415_d_n7, assign27470_e33415_d_n8, assign27470_e33415_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) {
        let assign27470_e33413: f64 = (var_umax).sqrt();
        (assign27470_e33413, (var_umax_dn6 / (2.0 * assign27470_e33413)), (var_umax_dn7 / (2.0 * assign27470_e33413)), (var_umax_dn8 / (2.0 * assign27470_e33413)), (var_umax_dn9 / (2.0 * assign27470_e33413)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign27470_e33415;
        var_sqrtumax_dn6 = assign27470_e33415_d_n6;
        var_sqrtumax_dn7 = assign27470_e33415_d_n7;
        var_sqrtumax_dn8 = assign27470_e33415_d_n8;
        var_sqrtumax_dn9 = assign27470_e33415_d_n9;

        let (assign27480_e33429, assign27480_e33429_d_n6, assign27480_e33429_d_n7, assign27480_e33429_d_n8, assign27480_e33429_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) {
        let assign27480_e33427: f64 = (var_umax * var_sqrtumax);
        (assign27480_e33427, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign27480_e33429;
        var_umaxpoweronepointfive_dn6 = assign27480_e33429_d_n6;
        var_umaxpoweronepointfive_dn7 = assign27480_e33429_d_n7;
        var_umaxpoweronepointfive_dn8 = assign27480_e33429_d_n8;
        var_umaxpoweronepointfive_dn9 = assign27480_e33429_d_n9;

        let assign27490_e33431: f64 = (-var_pbotd_i);
        let assign27490_e33433: f64 = (assign27490_e33431 * var_one_over_one_minus_pbot_d);
        let assign27490_e33435: f64 = (-1.0);
        let assign27490_e33436: f64 = if assign27490_e33433 == assign27490_e33435 { 1.0 } else { 0.0 };
        var_guard541 = assign27490_e33436;

        let (assign27500_e33456, assign27500_e33456_d_n6, assign27500_e33456_d_n7, assign27500_e33456_d_n8, assign27500_e33456_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) && (var_guard541 != 0.0)) {
        let assign27500_e33452: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign27500_e33453: f64 = (1.0 + assign27500_e33452);
        let assign27500_e33454: f64 = (1.0 / assign27500_e33453);
        (assign27500_e33454, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign27500_e33453 * assign27500_e33453))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign27500_e33453 * assign27500_e33453))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign27500_e33453 * assign27500_e33453))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign27500_e33453 * assign27500_e33453))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign27500_e33456;
        var_wgamma_dn6 = assign27500_e33456_d_n6;
        var_wgamma_dn7 = assign27500_e33456_d_n7;
        var_wgamma_dn8 = assign27500_e33456_d_n8;
        var_wgamma_dn9 = assign27500_e33456_d_n9;

        let (assign27510_e33480, assign27510_e33480_d_n6, assign27510_e33480_d_n7, assign27510_e33480_d_n8, assign27510_e33480_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) && (var_guard541 == 0.0)) {
        let assign27510_e33472: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign27510_e33473: f64 = (1.0 + assign27510_e33472);
        let assign27510_e33475: f64 = (-var_pbotd_i);
        let assign27510_e33477: f64 = (assign27510_e33475 * var_one_over_one_minus_pbot_d);
        let assign27510_e33478: f64 = (assign27510_e33473).powf(assign27510_e33477);
        (assign27510_e33478, if 0.0 == 0.0 && ((assign27510_e33477) as f64).is_finite() && ((assign27510_e33477) as f64).fract() == 0.0 { if assign27510_e33477 == 0.0 { 0.0 } else { (assign27510_e33477 * ((assign27510_e33473).powf(assign27510_e33477 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign27510_e33478 * (assign27510_e33477 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign27510_e33473))) }, if 0.0 == 0.0 && ((assign27510_e33477) as f64).is_finite() && ((assign27510_e33477) as f64).fract() == 0.0 { if assign27510_e33477 == 0.0 { 0.0 } else { (assign27510_e33477 * ((assign27510_e33473).powf(assign27510_e33477 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign27510_e33478 * (assign27510_e33477 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign27510_e33473))) }, if 0.0 == 0.0 && ((assign27510_e33477) as f64).is_finite() && ((assign27510_e33477) as f64).fract() == 0.0 { if assign27510_e33477 == 0.0 { 0.0 } else { (assign27510_e33477 * ((assign27510_e33473).powf(assign27510_e33477 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign27510_e33478 * (assign27510_e33477 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign27510_e33473))) }, if 0.0 == 0.0 && ((assign27510_e33477) as f64).is_finite() && ((assign27510_e33477) as f64).fract() == 0.0 { if assign27510_e33477 == 0.0 { 0.0 } else { (assign27510_e33477 * ((assign27510_e33473).powf(assign27510_e33477 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign27510_e33478 * (assign27510_e33477 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign27510_e33473))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign27510_e33480;
        var_wgamma_dn6 = assign27510_e33480_d_n6;
        var_wgamma_dn7 = assign27510_e33480_d_n7;
        var_wgamma_dn8 = assign27510_e33480_d_n8;
        var_wgamma_dn9 = assign27510_e33480_d_n9;

        let (assign27520_e33498, assign27520_e33498_d_n6, assign27520_e33498_d_n7, assign27520_e33498_d_n8, assign27520_e33498_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) {
        let assign27520_e33492: f64 = (var_wsrh * var_wgamma);
        let assign27520_e33495: f64 = (var_wsrh + var_wgamma);
        let assign27520_e33496: f64 = (assign27520_e33492 / assign27520_e33495);
        (assign27520_e33496, ((((var_wsrh * var_wgamma_dn6) * assign27520_e33495) - (assign27520_e33492 * var_wgamma_dn6)) / (assign27520_e33495 * assign27520_e33495)), ((((var_wsrh * var_wgamma_dn7) * assign27520_e33495) - (assign27520_e33492 * var_wgamma_dn7)) / (assign27520_e33495 * assign27520_e33495)), ((((var_wsrh * var_wgamma_dn8) * assign27520_e33495) - (assign27520_e33492 * var_wgamma_dn8)) / (assign27520_e33495 * assign27520_e33495)), ((((var_wsrh * var_wgamma_dn9) * assign27520_e33495) - (assign27520_e33492 * var_wgamma_dn9)) / (assign27520_e33495 * assign27520_e33495)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign27520_e33498;
        var_wtat_dn6 = assign27520_e33498_d_n6;
        var_wtat_dn7 = assign27520_e33498_d_n7;
        var_wtat_dn8 = assign27520_e33498_d_n8;
        var_wtat_dn9 = assign27520_e33498_d_n9;

        let (assign27530_e33515, assign27530_e33515_d_n6, assign27530_e33515_d_n7, assign27530_e33515_d_n8, assign27530_e33515_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) {
        let assign27530_e33511: f64 = (var_btat / var_sqrtumax);
        let assign27530_e33512: f64 = (0.375 * assign27530_e33511);
        let assign27530_e33513: f64 = (assign27530_e33512).sqrt();
        (assign27530_e33513, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign27530_e33513)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign27530_e33513)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign27530_e33513)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign27530_e33513)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign27530_e33515;
        var_ktat_dn6 = assign27530_e33515_d_n6;
        var_ktat_dn7 = assign27530_e33515_d_n7;
        var_ktat_dn8 = assign27530_e33515_d_n8;
        var_ktat_dn9 = assign27530_e33515_d_n9;

        let (assign27540_e33533, assign27540_e33533_d_n6, assign27540_e33533_d_n7, assign27540_e33533_d_n8, assign27540_e33533_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) {
        let assign27540_e33528: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign27540_e33529: f64 = (2.0 * assign27540_e33528);
        let assign27540_e33531: f64 = (assign27540_e33529 - var_umax);
        (assign27540_e33531, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign27540_e33533;
        var_ltat_dn6 = assign27540_e33533_d_n6;
        var_ltat_dn7 = assign27540_e33533_d_n7;
        var_ltat_dn8 = assign27540_e33533_d_n8;
        var_ltat_dn9 = assign27540_e33533_d_n9;

        let (assign27550_e33559, assign27550_e33559_d_n6, assign27550_e33559_d_n7, assign27550_e33559_d_n8, assign27550_e33559_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) {
        let assign27550_e33545: f64 = (var_atatbot_d * var_twoatatoverthreebtat);
        let assign27550_e33547: f64 = (assign27550_e33545 * var_sqrtumax);
        let assign27550_e33550: f64 = (var_atatbot_d * var_umax);
        let assign27550_e33551: f64 = (assign27550_e33547 - assign27550_e33550);
        let assign27550_e33555: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign27550_e33556: f64 = (0.5 * assign27550_e33555);
        let assign27550_e33557: f64 = (assign27550_e33551 + assign27550_e33556);
        (assign27550_e33557, (((((var_atatbot_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign27550_e33545 * var_sqrtumax_dn6)) - (var_atatbot_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign27550_e33545 * var_sqrtumax_dn7)) - (var_atatbot_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign27550_e33545 * var_sqrtumax_dn8)) - (var_atatbot_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign27550_e33545 * var_sqrtumax_dn9)) - (var_atatbot_d * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign27550_e33559;
        var_mtat_dn6 = assign27550_e33559_d_n6;
        var_mtat_dn7 = assign27550_e33559_d_n7;
        var_mtat_dn8 = assign27550_e33559_d_n8;
        var_mtat_dn9 = assign27550_e33559_d_n9;

        let (assign27560_e33575, assign27560_e33575_d_n6, assign27560_e33575_d_n7, assign27560_e33575_d_n8, assign27560_e33575_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) {
        let assign27560_e33571: f64 = (var_ltat - 1.0);
        let assign27560_e33573: f64 = (assign27560_e33571 * var_ktat);
        (assign27560_e33573, ((var_ltat_dn6 * var_ktat) + (assign27560_e33571 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign27560_e33571 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign27560_e33571 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign27560_e33571 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign27560_e33575;
        var_xerfc_dn6 = assign27560_e33575_d_n6;
        var_xerfc_dn7 = assign27560_e33575_d_n7;
        var_xerfc_dn8 = assign27560_e33575_d_n8;
        var_xerfc_dn9 = assign27560_e33575_d_n9;

        let (assign27570_e33589, assign27570_e33589_d_n6, assign27570_e33589_d_n7, assign27570_e33589_d_n8, assign27570_e33589_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) {
        let assign27570_e33587: f64 = (var_xerfc * var_xerfc);
        (assign27570_e33587, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign27570_e33589;
        var_ysq_dn6 = assign27570_e33589_d_n6;
        var_ysq_dn7 = assign27570_e33589_d_n7;
        var_ysq_dn8 = assign27570_e33589_d_n8;
        var_ysq_dn9 = assign27570_e33589_d_n9;

        let assign27580_e33592: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard542 = assign27580_e33592;

        let (assign27590_e33612, assign27590_e33612_d_n6, assign27590_e33612_d_n7, assign27590_e33612_d_n8, assign27590_e33612_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) && (var_guard542 != 0.0)) {
        let assign27590_e33608: f64 = (var_perfc * var_xerfc);
        let assign27590_e33609: f64 = (1.0 + assign27590_e33608);
        let assign27590_e33610: f64 = (1.0 / assign27590_e33609);
        (assign27590_e33610, (-((var_perfc * var_xerfc_dn6) / (assign27590_e33609 * assign27590_e33609))), (-((var_perfc * var_xerfc_dn7) / (assign27590_e33609 * assign27590_e33609))), (-((var_perfc * var_xerfc_dn8) / (assign27590_e33609 * assign27590_e33609))), (-((var_perfc * var_xerfc_dn9) / (assign27590_e33609 * assign27590_e33609))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign27590_e33612;
        var_terfc_dn6 = assign27590_e33612_d_n6;
        var_terfc_dn7 = assign27590_e33612_d_n7;
        var_terfc_dn8 = assign27590_e33612_d_n8;
        var_terfc_dn9 = assign27590_e33612_d_n9;

        let (assign27600_e33633, assign27600_e33633_d_n6, assign27600_e33633_d_n7, assign27600_e33633_d_n8, assign27600_e33633_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) && (var_guard542 == 0.0)) {
        let assign27600_e33629: f64 = (var_perfc * var_xerfc);
        let assign27600_e33630: f64 = (1.0 - assign27600_e33629);
        let assign27600_e33631: f64 = (1.0 / assign27600_e33630);
        (assign27600_e33631, (-((-(var_perfc * var_xerfc_dn6)) / (assign27600_e33630 * assign27600_e33630))), (-((-(var_perfc * var_xerfc_dn7)) / (assign27600_e33630 * assign27600_e33630))), (-((-(var_perfc * var_xerfc_dn8)) / (assign27600_e33630 * assign27600_e33630))), (-((-(var_perfc * var_xerfc_dn9)) / (assign27600_e33630 * assign27600_e33630))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign27600_e33633;
        var_terfc_dn6 = assign27600_e33633_d_n6;
        var_terfc_dn7 = assign27600_e33633_d_n7;
        var_terfc_dn8 = assign27600_e33633_d_n8;
        var_terfc_dn9 = assign27600_e33633_d_n9;

        let assign27610_e33635: f64 = (-var_ysq);
        let assign27610_e33637: f64 = (assign27610_e33635 + var_mtat);
        let assign27610_e33639: f64 = (-230.25850929940458);
        let assign27610_e33640: f64 = if assign27610_e33637 > assign27610_e33639 { 1.0 } else { 0.0 };
        var_guard543 = assign27610_e33640;

        let (assign27620_e33658, assign27620_e33658_d_n6, assign27620_e33658_d_n7, assign27620_e33658_d_n8, assign27620_e33658_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) && (var_guard543 != 0.0)) {
        let assign27620_e33653: f64 = (-var_ysq);
        let assign27620_e33655: f64 = (assign27620_e33653 + var_mtat);
        let assign27620_e33656: f64 = (assign27620_e33655).exp();
        (assign27620_e33656, (assign27620_e33656 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign27620_e33656 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign27620_e33656 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign27620_e33656 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign27620_e33658;
        var_tmp_dn6 = assign27620_e33658_d_n6;
        var_tmp_dn7 = assign27620_e33658_d_n7;
        var_tmp_dn8 = assign27620_e33658_d_n8;
        var_tmp_dn9 = assign27620_e33658_d_n9;

        let (assign27630_e33707, assign27630_e33707_d_n6, assign27630_e33707_d_n7, assign27630_e33707_d_n8, assign27630_e33707_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) && (var_guard543 == 0.0)) {
        let assign27630_e33674: f64 = (-230.25850929940458);
        let assign27630_e33676: f64 = (-var_ysq);
        let assign27630_e33678: f64 = (assign27630_e33676 + var_mtat);
        let assign27630_e33679: f64 = (assign27630_e33674 - assign27630_e33678);
        let assign27630_e33683: f64 = (-230.25850929940458);
        let assign27630_e33685: f64 = (-var_ysq);
        let assign27630_e33687: f64 = (assign27630_e33685 + var_mtat);
        let assign27630_e33688: f64 = (assign27630_e33683 - assign27630_e33687);
        let assign27630_e33691: f64 = (-230.25850929940458);
        let assign27630_e33693: f64 = (-var_ysq);
        let assign27630_e33695: f64 = (assign27630_e33693 + var_mtat);
        let assign27630_e33696: f64 = (assign27630_e33691 - assign27630_e33695);
        let assign27630_e33698: f64 = (assign27630_e33696 * 0.3333333333333333);
        let assign27630_e33699: f64 = (1.0 + assign27630_e33698);
        let assign27630_e33700: f64 = (assign27630_e33688 * assign27630_e33699);
        let assign27630_e33701: f64 = (0.5 * assign27630_e33700);
        let assign27630_e33702: f64 = (1.0 + assign27630_e33701);
        let assign27630_e33703: f64 = (assign27630_e33679 * assign27630_e33702);
        let assign27630_e33704: f64 = (1.0 + assign27630_e33703);
        let assign27630_e33705: f64 = (1e-100 / assign27630_e33704);
        (assign27630_e33705, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign27630_e33702) + (assign27630_e33679 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign27630_e33699) + (assign27630_e33688 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign27630_e33704 * assign27630_e33704))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign27630_e33702) + (assign27630_e33679 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign27630_e33699) + (assign27630_e33688 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign27630_e33704 * assign27630_e33704))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign27630_e33702) + (assign27630_e33679 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign27630_e33699) + (assign27630_e33688 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign27630_e33704 * assign27630_e33704))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign27630_e33702) + (assign27630_e33679 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign27630_e33699) + (assign27630_e33688 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign27630_e33704 * assign27630_e33704))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign27630_e33707;
        var_tmp_dn6 = assign27630_e33707_d_n6;
        var_tmp_dn7 = assign27630_e33707_d_n7;
        var_tmp_dn8 = assign27630_e33707_d_n8;
        var_tmp_dn9 = assign27630_e33707_d_n9;

        let (assign27640_e33737, assign27640_e33737_d_n6, assign27640_e33737_d_n7, assign27640_e33737_d_n8, assign27640_e33737_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) {
        let assign27640_e33719: f64 = (0.29214664 * var_terfc);
        let assign27640_e33723: f64 = (var_terfc * var_terfc);
        let assign27640_e33724: f64 = (var_berfc * assign27640_e33723);
        let assign27640_e33725: f64 = (assign27640_e33719 + assign27640_e33724);
        let assign27640_e33729: f64 = (var_terfc * var_terfc);
        let assign27640_e33731: f64 = (assign27640_e33729 * var_terfc);
        let assign27640_e33732: f64 = (var_cerfc * assign27640_e33731);
        let assign27640_e33733: f64 = (assign27640_e33725 + assign27640_e33732);
        let assign27640_e33735: f64 = (assign27640_e33733 * var_tmp);
        (assign27640_e33735, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign27640_e33729 * var_terfc_dn6)))) * var_tmp) + (assign27640_e33733 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign27640_e33729 * var_terfc_dn7)))) * var_tmp) + (assign27640_e33733 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign27640_e33729 * var_terfc_dn8)))) * var_tmp) + (assign27640_e33733 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign27640_e33729 * var_terfc_dn9)))) * var_tmp) + (assign27640_e33733 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign27640_e33737;
        var_erfcpos_dn6 = assign27640_e33737_d_n6;
        var_erfcpos_dn7 = assign27640_e33737_d_n7;
        var_erfcpos_dn8 = assign27640_e33737_d_n8;
        var_erfcpos_dn9 = assign27640_e33737_d_n9;

        let assign27650_e33740: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard544 = assign27650_e33740;

        let (assign27660_e33754, assign27660_e33754_d_n6, assign27660_e33754_d_n7, assign27660_e33754_d_n8, assign27660_e33754_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) && (var_guard544 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign27660_e33754;
        var_erfctimesexpmtat_dn6 = assign27660_e33754_d_n6;
        var_erfctimesexpmtat_dn7 = assign27660_e33754_d_n7;
        var_erfctimesexpmtat_dn8 = assign27660_e33754_d_n8;
        var_erfctimesexpmtat_dn9 = assign27660_e33754_d_n9;

        let assign27670_e33757: f64 = (-230.25850929940458);
        let assign27670_e33758: f64 = if var_mtat > assign27670_e33757 { 1.0 } else { 0.0 };
        var_guard545 = assign27670_e33758;

        let (assign27680_e33776, assign27680_e33776_d_n6, assign27680_e33776_d_n7, assign27680_e33776_d_n8, assign27680_e33776_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) && (var_guard544 == 0.0)) && (var_guard545 != 0.0)) {
        let assign27680_e33774: f64 = (var_mtat).exp();
        (assign27680_e33774, (assign27680_e33774 * var_mtat_dn6), (assign27680_e33774 * var_mtat_dn7), (assign27680_e33774 * var_mtat_dn8), (assign27680_e33774 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign27680_e33776;
        var_tmp_dn6 = assign27680_e33776_d_n6;
        var_tmp_dn7 = assign27680_e33776_d_n7;
        var_tmp_dn8 = assign27680_e33776_d_n8;
        var_tmp_dn9 = assign27680_e33776_d_n9;

        let (assign27690_e33819, assign27690_e33819_d_n6, assign27690_e33819_d_n7, assign27690_e33819_d_n8, assign27690_e33819_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) && (var_guard544 == 0.0)) && (var_guard545 == 0.0)) {
        let assign27690_e33795: f64 = (-230.25850929940458);
        let assign27690_e33797: f64 = (assign27690_e33795 - var_mtat);
        let assign27690_e33801: f64 = (-230.25850929940458);
        let assign27690_e33803: f64 = (assign27690_e33801 - var_mtat);
        let assign27690_e33806: f64 = (-230.25850929940458);
        let assign27690_e33808: f64 = (assign27690_e33806 - var_mtat);
        let assign27690_e33810: f64 = (assign27690_e33808 * 0.3333333333333333);
        let assign27690_e33811: f64 = (1.0 + assign27690_e33810);
        let assign27690_e33812: f64 = (assign27690_e33803 * assign27690_e33811);
        let assign27690_e33813: f64 = (0.5 * assign27690_e33812);
        let assign27690_e33814: f64 = (1.0 + assign27690_e33813);
        let assign27690_e33815: f64 = (assign27690_e33797 * assign27690_e33814);
        let assign27690_e33816: f64 = (1.0 + assign27690_e33815);
        let assign27690_e33817: f64 = (1e-100 / assign27690_e33816);
        (assign27690_e33817, (-((1e-100 * (((-var_mtat_dn6) * assign27690_e33814) + (assign27690_e33797 * (0.5 * (((-var_mtat_dn6) * assign27690_e33811) + (assign27690_e33803 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign27690_e33816 * assign27690_e33816))), (-((1e-100 * (((-var_mtat_dn7) * assign27690_e33814) + (assign27690_e33797 * (0.5 * (((-var_mtat_dn7) * assign27690_e33811) + (assign27690_e33803 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign27690_e33816 * assign27690_e33816))), (-((1e-100 * (((-var_mtat_dn8) * assign27690_e33814) + (assign27690_e33797 * (0.5 * (((-var_mtat_dn8) * assign27690_e33811) + (assign27690_e33803 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign27690_e33816 * assign27690_e33816))), (-((1e-100 * (((-var_mtat_dn9) * assign27690_e33814) + (assign27690_e33797 * (0.5 * (((-var_mtat_dn9) * assign27690_e33811) + (assign27690_e33803 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign27690_e33816 * assign27690_e33816))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign27690_e33819;
        var_tmp_dn6 = assign27690_e33819_d_n6;
        var_tmp_dn7 = assign27690_e33819_d_n7;
        var_tmp_dn8 = assign27690_e33819_d_n8;
        var_tmp_dn9 = assign27690_e33819_d_n9;

        let (assign27700_e33838, assign27700_e33838_d_n6, assign27700_e33838_d_n7, assign27700_e33838_d_n8, assign27700_e33838_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) && (var_guard544 == 0.0)) {
        let assign27700_e33834: f64 = (2.0 * var_tmp);
        let assign27700_e33836: f64 = (assign27700_e33834 - var_erfcpos);
        (assign27700_e33836, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign27700_e33838;
        var_erfctimesexpmtat_dn6 = assign27700_e33838_d_n6;
        var_erfctimesexpmtat_dn7 = assign27700_e33838_d_n7;
        var_erfctimesexpmtat_dn8 = assign27700_e33838_d_n8;
        var_erfctimesexpmtat_dn9 = assign27700_e33838_d_n9;

        let (assign27710_e33858, assign27710_e33858_d_n6, assign27710_e33858_d_n7, assign27710_e33858_d_n8, assign27710_e33858_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) {
        let assign27710_e33850: f64 = (1.772453850905516 * 0.5);
        let assign27710_e33853: f64 = (var_atatbot_d * var_erfctimesexpmtat);
        let assign27710_e33855: f64 = (assign27710_e33853 / var_ktat);
        let assign27710_e33856: f64 = (assign27710_e33850 * assign27710_e33855);
        (assign27710_e33856, (assign27710_e33850 * ((((var_atatbot_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign27710_e33853 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign27710_e33850 * ((((var_atatbot_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign27710_e33853 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign27710_e33850 * ((((var_atatbot_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign27710_e33853 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign27710_e33850 * ((((var_atatbot_d * var_erfctimesexpmtat_dn9) * var_ktat) - (assign27710_e33853 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign27710_e33858;
        var_gammamax_dn6 = assign27710_e33858_d_n6;
        var_gammamax_dn7 = assign27710_e33858_d_n7;
        var_gammamax_dn8 = assign27710_e33858_d_n8;
        var_gammamax_dn9 = assign27710_e33858_d_n9;

        let (assign27720_e33876, assign27720_e33876_d_n6, assign27720_e33876_d_n7, assign27720_e33876_d_n8, assign27720_e33876_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard540 == 0.0)) {
        let assign27720_e33871: f64 = (var_asrh * var_gammamax);
        let assign27720_e33873: f64 = (assign27720_e33871 * var_wtat);
        let assign27720_e33874: f64 = (var_ctatbotd_i * assign27720_e33873);
        (assign27720_e33874, (var_ctatbotd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign27720_e33871 * var_wtat_dn6))), (var_ctatbotd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign27720_e33871 * var_wtat_dn7))), (var_ctatbotd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign27720_e33871 * var_wtat_dn8))), (var_ctatbotd_i * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign27720_e33871 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign27720_e33876;
        var_itat_dn6 = assign27720_e33876_d_n6;
        var_itat_dn7 = assign27720_e33876_d_n7;
        var_itat_dn8 = assign27720_e33876_d_n8;
        var_itat_dn9 = assign27720_e33876_d_n9;

        let assign27730_e33879: f64 = if var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard546 = assign27730_e33879;

        let (assign27740_e33890, assign27740_e33890_d_n6, assign27740_e33890_d_n7, assign27740_e33890_d_n8, assign27740_e33890_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard546 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign27740_e33890;
        var_ibbt_dn6 = assign27740_e33890_d_n6;
        var_ibbt_dn7 = assign27740_e33890_d_n7;
        var_ibbt_dn8 = assign27740_e33890_d_n8;
        var_ibbt_dn9 = assign27740_e33890_d_n9;

        let assign27750_e33893: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard547 = assign27750_e33893;

        let (assign27760_e33912, assign27760_e33912_d_n6, assign27760_e33912_d_n7, assign27760_e33912_d_n8, assign27760_e33912_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard546 == 0.0)) && (var_guard547 != 0.0)) {
        let assign27760_e33907: f64 = (var_vbirbotd_i - var_vbbt);
        let assign27760_e33909: f64 = (assign27760_e33907 * var_vbirbotinv_d);
        let assign27760_e33910: f64 = (assign27760_e33909).sqrt();
        (assign27760_e33910, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign27760_e33912;
        var_tmp_dn6 = assign27760_e33912_d_n6;
        var_tmp_dn7 = assign27760_e33912_d_n7;
        var_tmp_dn8 = assign27760_e33912_d_n8;
        var_tmp_dn9 = assign27760_e33912_d_n9;

        let (assign27770_e33933, assign27770_e33933_d_n6, assign27770_e33933_d_n7, assign27770_e33933_d_n8, assign27770_e33933_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard546 == 0.0)) && (var_guard547 == 0.0)) {
        let assign27770_e33927: f64 = (var_vbirbotd_i - var_vbbt);
        let assign27770_e33929: f64 = (assign27770_e33927 * var_vbirbotinv_d);
        let assign27770_e33931: f64 = (assign27770_e33929).powf(var_pbotd_i);
        (assign27770_e33931, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign27770_e33933;
        var_tmp_dn6 = assign27770_e33933_d_n6;
        var_tmp_dn7 = assign27770_e33933_d_n7;
        var_tmp_dn8 = assign27770_e33933_d_n8;
        var_tmp_dn9 = assign27770_e33933_d_n9;

        let (assign27780_e33953, assign27780_e33953_d_n6, assign27780_e33953_d_n7, assign27780_e33953_d_n8, assign27780_e33953_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard546 == 0.0)) {
        let assign27780_e33946: f64 = (var_vbirbotd_i - var_vbbt);
        let assign27780_e33948: f64 = (assign27780_e33946 * var_wdepnulrinvbot_d);
        let assign27780_e33950: f64 = (assign27780_e33948 / var_tmp);
        let assign27780_e33951: f64 = (var_one_over_one_minus_pbot_d * assign27780_e33950);
        (assign27780_e33951, (var_one_over_one_minus_pbot_d * (-((assign27780_e33948 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign27780_e33948 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign27780_e33948 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign27780_e33948 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign27780_e33953;
        var_fmaxr_dn6 = assign27780_e33953_d_n6;
        var_fmaxr_dn7 = assign27780_e33953_d_n7;
        var_fmaxr_dn8 = assign27780_e33953_d_n8;
        var_fmaxr_dn9 = assign27780_e33953_d_n9;

        let assign27790_e33955: f64 = (-var_fbbtbot_d);
        let assign27790_e33957: f64 = (assign27790_e33955 / var_fmaxr);
        let assign27790_e33958: f64 = (assign27790_e33957).abs();
        let assign27790_e33960: f64 = if assign27790_e33958 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard548 = assign27790_e33960;

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
        *var_guard541_slot = var_guard541;
        *var_guard542_slot = var_guard542;
        *var_guard543_slot = var_guard543;
        *var_guard544_slot = var_guard544;
        *var_guard545_slot = var_guard545;
        *var_guard546_slot = var_guard546;
        *var_guard547_slot = var_guard547;
        *var_guard548_slot = var_guard548;
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

    pub(super) fn stamp_transient_block_55(
        p: &Parameters,
        var_alphaav: f64,
        var_atatsti_d: f64,
        var_btatpartsti_d: f64,
        var_cbbtbotd_i: f64,
        var_csrhstid_i: f64,
        var_ctatstid_i: f64,
        var_fbbtbot_d: f64,
        var_fmaxr: f64,
        var_fmaxr_dn6: f64,
        var_fmaxr_dn7: f64,
        var_fmaxr_dn8: f64,
        var_fmaxr_dn9: f64,
        var_fstopbot_d: f64,
        var_ftdsti_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard536: f64,
        var_guard546: f64,
        var_guard548: f64,
        var_idmult: f64,
        var_idsatsti_d: f64,
        var_lsdrain_i: f64,
        var_one_minus_psti_d: f64,
        var_one_over_one_minus_psti_d: f64,
        var_pbrbotd_i: f64,
        var_pstid_i: f64,
        var_slopebot_d: f64,
        var_two_psistar: f64,
        var_v1: f64,
        var_vav: f64,
        var_vbirstiinv_d: f64,
        var_vbisti_d: f64,
        var_vbrbotd_i: f64,
        var_vbrinvbot_d: f64,
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
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
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
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_sqrtumax_dn9_slot: &mut f64,
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
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_sqrtumax_dn9: f64 = *var_sqrtumax_dn9_slot;
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
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;

        let (assign27800_e33978, assign27800_e33978_d_n6, assign27800_e33978_d_n7, assign27800_e33978_d_n8, assign27800_e33978_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard546 == 0.0)) && (var_guard548 != 0.0)) {
        let assign27800_e33973: f64 = (-var_fbbtbot_d);
        let assign27800_e33975: f64 = (assign27800_e33973 / var_fmaxr);
        let assign27800_e33976: f64 = (assign27800_e33975).exp();
        (assign27800_e33976, (assign27800_e33976 * (-((assign27800_e33973 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign27800_e33976 * (-((assign27800_e33973 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign27800_e33976 * (-((assign27800_e33973 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign27800_e33976 * (-((assign27800_e33973 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign27800_e33978;
        var_tmp_dn6 = assign27800_e33978_d_n6;
        var_tmp_dn7 = assign27800_e33978_d_n7;
        var_tmp_dn8 = assign27800_e33978_d_n8;
        var_tmp_dn9 = assign27800_e33978_d_n9;

        let assign27810_e33980: f64 = (-var_fbbtbot_d);
        let assign27810_e33982: f64 = (assign27810_e33980 / var_fmaxr);
        let assign27810_e33984: f64 = if assign27810_e33982 < 0.0 { 1.0 } else { 0.0 };
        var_guard549 = assign27810_e33984;

        let (assign27820_e34035, assign27820_e34035_d_n6, assign27820_e34035_d_n7, assign27820_e34035_d_n8, assign27820_e34035_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard546 == 0.0)) && (var_guard548 == 0.0)) && (var_guard549 != 0.0)) {
        let assign27820_e34002: f64 = (-230.25850929940458);
        let assign27820_e34004: f64 = (-var_fbbtbot_d);
        let assign27820_e34006: f64 = (assign27820_e34004 / var_fmaxr);
        let assign27820_e34007: f64 = (assign27820_e34002 - assign27820_e34006);
        let assign27820_e34011: f64 = (-230.25850929940458);
        let assign27820_e34013: f64 = (-var_fbbtbot_d);
        let assign27820_e34015: f64 = (assign27820_e34013 / var_fmaxr);
        let assign27820_e34016: f64 = (assign27820_e34011 - assign27820_e34015);
        let assign27820_e34019: f64 = (-230.25850929940458);
        let assign27820_e34021: f64 = (-var_fbbtbot_d);
        let assign27820_e34023: f64 = (assign27820_e34021 / var_fmaxr);
        let assign27820_e34024: f64 = (assign27820_e34019 - assign27820_e34023);
        let assign27820_e34026: f64 = (assign27820_e34024 * 0.3333333333333333);
        let assign27820_e34027: f64 = (1.0 + assign27820_e34026);
        let assign27820_e34028: f64 = (assign27820_e34016 * assign27820_e34027);
        let assign27820_e34029: f64 = (0.5 * assign27820_e34028);
        let assign27820_e34030: f64 = (1.0 + assign27820_e34029);
        let assign27820_e34031: f64 = (assign27820_e34007 * assign27820_e34030);
        let assign27820_e34032: f64 = (1.0 + assign27820_e34031);
        let assign27820_e34033: f64 = (1e-100 / assign27820_e34032);
        (assign27820_e34033, (-((1e-100 * (((-(-((assign27820_e34004 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign27820_e34030) + (assign27820_e34007 * (0.5 * (((-(-((assign27820_e34013 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign27820_e34027) + (assign27820_e34016 * ((-(-((assign27820_e34021 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign27820_e34032 * assign27820_e34032))), (-((1e-100 * (((-(-((assign27820_e34004 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign27820_e34030) + (assign27820_e34007 * (0.5 * (((-(-((assign27820_e34013 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign27820_e34027) + (assign27820_e34016 * ((-(-((assign27820_e34021 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign27820_e34032 * assign27820_e34032))), (-((1e-100 * (((-(-((assign27820_e34004 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign27820_e34030) + (assign27820_e34007 * (0.5 * (((-(-((assign27820_e34013 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign27820_e34027) + (assign27820_e34016 * ((-(-((assign27820_e34021 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign27820_e34032 * assign27820_e34032))), (-((1e-100 * (((-(-((assign27820_e34004 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign27820_e34030) + (assign27820_e34007 * (0.5 * (((-(-((assign27820_e34013 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign27820_e34027) + (assign27820_e34016 * ((-(-((assign27820_e34021 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign27820_e34032 * assign27820_e34032))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign27820_e34035;
        var_tmp_dn6 = assign27820_e34035_d_n6;
        var_tmp_dn7 = assign27820_e34035_d_n7;
        var_tmp_dn8 = assign27820_e34035_d_n8;
        var_tmp_dn9 = assign27820_e34035_d_n9;

        let (assign27830_e34084, assign27830_e34084_d_n6, assign27830_e34084_d_n7, assign27830_e34084_d_n8, assign27830_e34084_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard546 == 0.0)) && (var_guard548 == 0.0)) && (var_guard549 == 0.0)) {
        let assign27830_e34054: f64 = (-var_fbbtbot_d);
        let assign27830_e34056: f64 = (assign27830_e34054 / var_fmaxr);
        let assign27830_e34058: f64 = (assign27830_e34056 - 230.25850929940458);
        let assign27830_e34062: f64 = (-var_fbbtbot_d);
        let assign27830_e34064: f64 = (assign27830_e34062 / var_fmaxr);
        let assign27830_e34066: f64 = (assign27830_e34064 - 230.25850929940458);
        let assign27830_e34069: f64 = (-var_fbbtbot_d);
        let assign27830_e34071: f64 = (assign27830_e34069 / var_fmaxr);
        let assign27830_e34073: f64 = (assign27830_e34071 - 230.25850929940458);
        let assign27830_e34075: f64 = (assign27830_e34073 * 0.3333333333333333);
        let assign27830_e34076: f64 = (1.0 + assign27830_e34075);
        let assign27830_e34077: f64 = (assign27830_e34066 * assign27830_e34076);
        let assign27830_e34078: f64 = (0.5 * assign27830_e34077);
        let assign27830_e34079: f64 = (1.0 + assign27830_e34078);
        let assign27830_e34080: f64 = (assign27830_e34058 * assign27830_e34079);
        let assign27830_e34081: f64 = (1.0 + assign27830_e34080);
        let assign27830_e34082: f64 = (1e100 * assign27830_e34081);
        (assign27830_e34082, (1e100 * (((-((assign27830_e34054 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign27830_e34079) + (assign27830_e34058 * (0.5 * (((-((assign27830_e34062 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign27830_e34076) + (assign27830_e34066 * ((-((assign27830_e34069 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign27830_e34054 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign27830_e34079) + (assign27830_e34058 * (0.5 * (((-((assign27830_e34062 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign27830_e34076) + (assign27830_e34066 * ((-((assign27830_e34069 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign27830_e34054 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign27830_e34079) + (assign27830_e34058 * (0.5 * (((-((assign27830_e34062 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign27830_e34076) + (assign27830_e34066 * ((-((assign27830_e34069 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign27830_e34054 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign27830_e34079) + (assign27830_e34058 * (0.5 * (((-((assign27830_e34062 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign27830_e34076) + (assign27830_e34066 * ((-((assign27830_e34069 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign27830_e34084;
        var_tmp_dn6 = assign27830_e34084_d_n6;
        var_tmp_dn7 = assign27830_e34084_d_n7;
        var_tmp_dn8 = assign27830_e34084_d_n8;
        var_tmp_dn9 = assign27830_e34084_d_n9;

        let (assign27840_e34104, assign27840_e34104_d_n6, assign27840_e34104_d_n7, assign27840_e34104_d_n8, assign27840_e34104_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard546 == 0.0)) {
        let assign27840_e34097: f64 = (var_v1 * var_fmaxr);
        let assign27840_e34099: f64 = (assign27840_e34097 * var_fmaxr);
        let assign27840_e34101: f64 = (assign27840_e34099 * var_tmp);
        let assign27840_e34102: f64 = (var_cbbtbotd_i * assign27840_e34101);
        (assign27840_e34102, (var_cbbtbotd_i * (((((var_v1 * var_fmaxr_dn6) * var_fmaxr) + (assign27840_e34097 * var_fmaxr_dn6)) * var_tmp) + (assign27840_e34099 * var_tmp_dn6))), (var_cbbtbotd_i * (((((var_v1 * var_fmaxr_dn7) * var_fmaxr) + (assign27840_e34097 * var_fmaxr_dn7)) * var_tmp) + (assign27840_e34099 * var_tmp_dn7))), (var_cbbtbotd_i * (((((var_v1 * var_fmaxr_dn8) * var_fmaxr) + (assign27840_e34097 * var_fmaxr_dn8)) * var_tmp) + (assign27840_e34099 * var_tmp_dn8))), (var_cbbtbotd_i * (((((var_v1 * var_fmaxr_dn9) * var_fmaxr) + (assign27840_e34097 * var_fmaxr_dn9)) * var_tmp) + (assign27840_e34099 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign27840_e34104;
        var_ibbt_dn6 = assign27840_e34104_d_n6;
        var_ibbt_dn7 = assign27840_e34104_d_n7;
        var_ibbt_dn8 = assign27840_e34104_d_n8;
        var_ibbt_dn9 = assign27840_e34104_d_n9;

        let assign27850_e34107: f64 = if var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard550 = assign27850_e34107;

        let (assign27860_e34118, assign27860_e34118_d_n6, assign27860_e34118_d_n7, assign27860_e34118_d_n8, assign27860_e34118_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard550 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign27860_e34118;
        var_fbreakdown_dn6 = assign27860_e34118_d_n6;
        var_fbreakdown_dn7 = assign27860_e34118_d_n7;
        var_fbreakdown_dn8 = assign27860_e34118_d_n8;
        var_fbreakdown_dn9 = assign27860_e34118_d_n9;

        let assign27870_e34121: f64 = (-var_alphaav);
        let assign27870_e34123: f64 = (assign27870_e34121 * var_vbrbotd_i);
        let assign27870_e34124: f64 = if var_vav > assign27870_e34123 { 1.0 } else { 0.0 };
        var_guard551 = assign27870_e34124;

        let assign27880_e34127: f64 = if var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard552 = assign27880_e34127;

        let (assign27890_e34157, assign27890_e34157_d_n6, assign27890_e34157_d_n7, assign27890_e34157_d_n8, assign27890_e34157_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard550 == 0.0)) && (var_guard551 != 0.0)) && (var_guard552 != 0.0)) {
        let assign27890_e34143: f64 = (var_vav * var_vbrinvbot_d);
        let assign27890_e34146: f64 = (var_vav * var_vbrinvbot_d);
        let assign27890_e34147: f64 = (assign27890_e34143 * assign27890_e34146);
        let assign27890_e34150: f64 = (var_vav * var_vbrinvbot_d);
        let assign27890_e34151: f64 = (assign27890_e34147 * assign27890_e34150);
        let assign27890_e34154: f64 = (var_vav * var_vbrinvbot_d);
        let assign27890_e34155: f64 = (assign27890_e34151 * assign27890_e34154);
        (assign27890_e34155, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign27890_e34157;
        var_tmp_dn6 = assign27890_e34157_d_n6;
        var_tmp_dn7 = assign27890_e34157_d_n7;
        var_tmp_dn8 = assign27890_e34157_d_n8;
        var_tmp_dn9 = assign27890_e34157_d_n9;

        let (assign27900_e34179, assign27900_e34179_d_n6, assign27900_e34179_d_n7, assign27900_e34179_d_n8, assign27900_e34179_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard550 == 0.0)) && (var_guard551 != 0.0)) && (var_guard552 == 0.0)) {
        let assign27900_e34174: f64 = (var_vav * var_vbrinvbot_d);
        let assign27900_e34175: f64 = (assign27900_e34174).abs();
        let assign27900_e34177: f64 = (assign27900_e34175).powf(var_pbrbotd_i);
        (assign27900_e34177, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign27900_e34179;
        var_tmp_dn6 = assign27900_e34179_d_n6;
        var_tmp_dn7 = assign27900_e34179_d_n7;
        var_tmp_dn8 = assign27900_e34179_d_n8;
        var_tmp_dn9 = assign27900_e34179_d_n9;

        let (assign27910_e34197, assign27910_e34197_d_n6, assign27910_e34197_d_n7, assign27910_e34197_d_n8, assign27910_e34197_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard550 == 0.0)) && (var_guard551 != 0.0)) {
        let assign27910_e34194: f64 = (1.0 - var_tmp);
        let assign27910_e34195: f64 = (1.0 / assign27910_e34194);
        (assign27910_e34195, (-((-var_tmp_dn6) / (assign27910_e34194 * assign27910_e34194))), (-((-var_tmp_dn7) / (assign27910_e34194 * assign27910_e34194))), (-((-var_tmp_dn8) / (assign27910_e34194 * assign27910_e34194))), (-((-var_tmp_dn9) / (assign27910_e34194 * assign27910_e34194))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign27910_e34197;
        var_fbreakdown_dn6 = assign27910_e34197_d_n6;
        var_fbreakdown_dn7 = assign27910_e34197_d_n7;
        var_fbreakdown_dn8 = assign27910_e34197_d_n8;
        var_fbreakdown_dn9 = assign27910_e34197_d_n9;

        let (assign27920_e34220, assign27920_e34220_d_n6, assign27920_e34220_d_n7, assign27920_e34220_d_n8, assign27920_e34220_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) && (var_guard550 == 0.0)) && (var_guard551 == 0.0)) {
        let assign27920_e34214: f64 = (var_alphaav * var_vbrbotd_i);
        let assign27920_e34215: f64 = (var_vav + assign27920_e34214);
        let assign27920_e34217: f64 = (assign27920_e34215 * var_slopebot_d);
        let assign27920_e34218: f64 = (var_fstopbot_d + assign27920_e34217);
        (assign27920_e34218, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign27920_e34220;
        var_fbreakdown_dn6 = assign27920_e34220_d_n6;
        var_fbreakdown_dn7 = assign27920_e34220_d_n7;
        var_fbreakdown_dn8 = assign27920_e34220_d_n8;
        var_fbreakdown_dn9 = assign27920_e34220_d_n9;

        let (assign27930_e34239, assign27930_e34239_d_n6, assign27930_e34239_d_n7, assign27930_e34239_d_n8, assign27930_e34239_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard536 == 0.0)) {
        let assign27930_e34230: f64 = (var_id__blk212 + var_isrh);
        let assign27930_e34232: f64 = (assign27930_e34230 + var_itat);
        let assign27930_e34234: f64 = (assign27930_e34232 + var_ibbt);
        let assign27930_e34235: f64 = (p.p29 * assign27930_e34234);
        let assign27930_e34237: f64 = (assign27930_e34235 * var_fbreakdown);
        (assign27930_e34237, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign27930_e34235 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign27930_e34235 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign27930_e34235 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign27930_e34235 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign27930_e34239;
        var_ijunbot_dn6 = assign27930_e34239_d_n6;
        var_ijunbot_dn7 = assign27930_e34239_d_n7;
        var_ijunbot_dn8 = assign27930_e34239_d_n8;
        var_ijunbot_dn9 = assign27930_e34239_d_n9;

        let assign27940_e34242: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard553 = assign27940_e34242;

        let (assign27950_e34250, assign27950_e34250_d_n6, assign27950_e34250_d_n7, assign27950_e34250_d_n8, assign27950_e34250_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign27950_e34250;
        var_ijunsti_dn6 = assign27950_e34250_d_n6;
        var_ijunsti_dn7 = assign27950_e34250_d_n7;
        var_ijunsti_dn8 = assign27950_e34250_d_n8;
        var_ijunsti_dn9 = assign27950_e34250_d_n9;

        let (assign27960_e34261,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) {
        let assign27960_e34259: f64 = (var_idsatsti_d * var_idmult);
        (assign27960_e34259,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign27960_e34261;

        let assign27970_e34268: f64 = if ((var_csrhstid_i == 0.0) && (var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard554 = assign27970_e34268;

        let (assign27980_e34279, assign27980_e34279_d_n6, assign27980_e34279_d_n7, assign27980_e34279_d_n8, assign27980_e34279_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard554 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign27980_e34279;
        var_isrh_dn6 = assign27980_e34279_d_n6;
        var_isrh_dn7 = assign27980_e34279_d_n7;
        var_isrh_dn8 = assign27980_e34279_d_n8;
        var_isrh_dn9 = assign27980_e34279_d_n9;

        let (assign27990_e34293,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard554 == 0.0)) {
        let assign27990_e34291: f64 = (var_vbisti_d - var_vjsrh);
        (assign27990_e34291,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign27990_e34293;

        let (assign28000_e34312,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard554 == 0.0)) {
        let assign28000_e34307: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign28000_e34308: f64 = (1.0 - assign28000_e34307);
        let assign28000_e34309: f64 = (assign28000_e34308).sqrt();
        let assign28000_e34310: f64 = (1.0 - assign28000_e34309);
        (assign28000_e34310,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign28000_e34312;

        let assign28010_e34315: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard555 = assign28010_e34315;

        let (assign28020_e34329,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard554 == 0.0)) && (var_guard555 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign28020_e34329;

        let (assign28030_e34361,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard554 == 0.0)) && (var_guard555 == 0.0)) {
        let assign28030_e34344: f64 = (var_wsrhstep * var_wsrhstep);
        let assign28030_e34346: f64 = (var_wsrhstep).ln();
        let assign28030_e34347: f64 = (assign28030_e34344 * assign28030_e34346);
        let assign28030_e34350: f64 = (1.0 - var_wsrhstep);
        let assign28030_e34351: f64 = (assign28030_e34347 / assign28030_e34350);
        let assign28030_e34353: f64 = (assign28030_e34351 + var_wsrhstep);
        let assign28030_e34357: f64 = (2.0 * var_pstid_i);
        let assign28030_e34358: f64 = (1.0 - assign28030_e34357);
        let assign28030_e34359: f64 = (assign28030_e34353 * assign28030_e34358);
        (assign28030_e34359,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign28030_e34361;

        let (assign28040_e34375,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard554 == 0.0)) {
        let assign28040_e34373: f64 = (var_wsrhstep + var_dwsrh);
        (assign28040_e34373,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign28040_e34375;

        let assign28050_e34378: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard556 = assign28050_e34378;

        let (assign28060_e34395, assign28060_e34395_d_n6, assign28060_e34395_d_n7, assign28060_e34395_d_n8, assign28060_e34395_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard554 == 0.0)) && (var_guard556 != 0.0)) {
        let assign28060_e34392: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign28060_e34393: f64 = (assign28060_e34392).sqrt();
        (assign28060_e34393, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign28060_e34395;
        var_tmp_dn6 = assign28060_e34395_d_n6;
        var_tmp_dn7 = assign28060_e34395_d_n7;
        var_tmp_dn8 = assign28060_e34395_d_n8;
        var_tmp_dn9 = assign28060_e34395_d_n9;

        let (assign28070_e34414, assign28070_e34414_d_n6, assign28070_e34414_d_n7, assign28070_e34414_d_n8, assign28070_e34414_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard554 == 0.0)) && (var_guard556 == 0.0)) {
        let assign28070_e34410: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign28070_e34412: f64 = (assign28070_e34410).powf(var_pstid_i);
        (assign28070_e34412, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign28070_e34414;
        var_tmp_dn6 = assign28070_e34414_d_n6;
        var_tmp_dn7 = assign28070_e34414_d_n7;
        var_tmp_dn8 = assign28070_e34414_d_n8;
        var_tmp_dn9 = assign28070_e34414_d_n9;

        let (assign28080_e34428, assign28080_e34428_d_n6, assign28080_e34428_d_n7, assign28080_e34428_d_n8, assign28080_e34428_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard554 == 0.0)) {
        let assign28080_e34426: f64 = (var_wdepnulrsti_d * var_tmp);
        (assign28080_e34426, (var_wdepnulrsti_d * var_tmp_dn6), (var_wdepnulrsti_d * var_tmp_dn7), (var_wdepnulrsti_d * var_tmp_dn8), (var_wdepnulrsti_d * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign28080_e34428;
        var_wdep_dn6 = assign28080_e34428_d_n6;
        var_wdep_dn7 = assign28080_e34428_d_n7;
        var_wdep_dn8 = assign28080_e34428_d_n8;
        var_wdep_dn9 = assign28080_e34428_d_n9;

        let (assign28090_e34446, assign28090_e34446_d_n6, assign28090_e34446_d_n7, assign28090_e34446_d_n8, assign28090_e34446_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard554 == 0.0)) {
        let assign28090_e34441: f64 = (var_zinv - 1.0);
        let assign28090_e34443: f64 = (assign28090_e34441 * var_wdep);
        let assign28090_e34444: f64 = (var_ftdsti_d * assign28090_e34443);
        (assign28090_e34444, (var_ftdsti_d * (assign28090_e34441 * var_wdep_dn6)), (var_ftdsti_d * (assign28090_e34441 * var_wdep_dn7)), (var_ftdsti_d * (assign28090_e34441 * var_wdep_dn8)), (var_ftdsti_d * (assign28090_e34441 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign28090_e34446;
        var_asrh_dn6 = assign28090_e34446_d_n6;
        var_asrh_dn7 = assign28090_e34446_d_n7;
        var_asrh_dn8 = assign28090_e34446_d_n8;
        var_asrh_dn9 = assign28090_e34446_d_n9;

        let (assign28100_e34462, assign28100_e34462_d_n6, assign28100_e34462_d_n7, assign28100_e34462_d_n8, assign28100_e34462_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard554 == 0.0)) {
        let assign28100_e34459: f64 = (var_asrh * var_wsrh);
        let assign28100_e34460: f64 = (var_csrhstid_i * assign28100_e34459);
        (assign28100_e34460, (var_csrhstid_i * (var_asrh_dn6 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn7 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn8 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign28100_e34462;
        var_isrh_dn6 = assign28100_e34462_d_n6;
        var_isrh_dn7 = assign28100_e34462_d_n7;
        var_isrh_dn8 = assign28100_e34462_d_n8;
        var_isrh_dn9 = assign28100_e34462_d_n9;

        let assign28110_e34465: f64 = if var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard557 = assign28110_e34465;

        let (assign28120_e34476, assign28120_e34476_d_n6, assign28120_e34476_d_n7, assign28120_e34476_d_n8, assign28120_e34476_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign28120_e34476;
        var_itat_dn6 = assign28120_e34476_d_n6;
        var_itat_dn7 = assign28120_e34476_d_n7;
        var_itat_dn8 = assign28120_e34476_d_n8;
        var_itat_dn9 = assign28120_e34476_d_n9;

        let (assign28130_e34494, assign28130_e34494_d_n6, assign28130_e34494_d_n7, assign28130_e34494_d_n8, assign28130_e34494_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) {
        let assign28130_e34489: f64 = (var_wdep * var_one_minus_psti_d);
        let assign28130_e34491: f64 = (assign28130_e34489 / var_vbi_minus_vjsrh);
        let assign28130_e34492: f64 = (var_btatpartsti_d * assign28130_e34491);
        (assign28130_e34492, (var_btatpartsti_d * ((var_wdep_dn6 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn7 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn8 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn9 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign28130_e34494;
        var_btat_dn6 = assign28130_e34494_d_n6;
        var_btat_dn7 = assign28130_e34494_d_n7;
        var_btat_dn8 = assign28130_e34494_d_n8;
        var_btat_dn9 = assign28130_e34494_d_n9;

        let (assign28140_e34510, assign28140_e34510_d_n6, assign28140_e34510_d_n7, assign28140_e34510_d_n8, assign28140_e34510_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) {
        let assign28140_e34506: f64 = (0.666666666666667 * var_atatsti_d);
        let assign28140_e34508: f64 = (assign28140_e34506 / var_btat);
        (assign28140_e34508, (-((assign28140_e34506 * var_btat_dn6) / (var_btat * var_btat))), (-((assign28140_e34506 * var_btat_dn7) / (var_btat * var_btat))), (-((assign28140_e34506 * var_btat_dn8) / (var_btat * var_btat))), (-((assign28140_e34506 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign28140_e34510;
        var_twoatatoverthreebtat_dn6 = assign28140_e34510_d_n6;
        var_twoatatoverthreebtat_dn7 = assign28140_e34510_d_n7;
        var_twoatatoverthreebtat_dn8 = assign28140_e34510_d_n8;
        var_twoatatoverthreebtat_dn9 = assign28140_e34510_d_n9;

        let (assign28150_e34524, assign28150_e34524_d_n6, assign28150_e34524_d_n7, assign28150_e34524_d_n8, assign28150_e34524_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) {
        let assign28150_e34522: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign28150_e34522, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign28150_e34524;
        var_umaxbeforelimiting_dn6 = assign28150_e34524_d_n6;
        var_umaxbeforelimiting_dn7 = assign28150_e34524_d_n7;
        var_umaxbeforelimiting_dn8 = assign28150_e34524_d_n8;
        var_umaxbeforelimiting_dn9 = assign28150_e34524_d_n9;

        let (assign28160_e34545, assign28160_e34545_d_n6, assign28160_e34545_d_n7, assign28160_e34545_d_n8, assign28160_e34545_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) {
        let assign28160_e34536: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign28160_e34539: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign28160_e34541: f64 = (assign28160_e34539 + 1.0);
        let assign28160_e34542: f64 = (assign28160_e34536 / assign28160_e34541);
        let assign28160_e34543: f64 = (assign28160_e34542).sqrt();
        (assign28160_e34543, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign28160_e34541) - (assign28160_e34536 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign28160_e34541 * assign28160_e34541)) / (2.0 * assign28160_e34543)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign28160_e34541) - (assign28160_e34536 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign28160_e34541 * assign28160_e34541)) / (2.0 * assign28160_e34543)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign28160_e34541) - (assign28160_e34536 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign28160_e34541 * assign28160_e34541)) / (2.0 * assign28160_e34543)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign28160_e34541) - (assign28160_e34536 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign28160_e34541 * assign28160_e34541)) / (2.0 * assign28160_e34543)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign28160_e34545;
        var_umax_dn6 = assign28160_e34545_d_n6;
        var_umax_dn7 = assign28160_e34545_d_n7;
        var_umax_dn8 = assign28160_e34545_d_n8;
        var_umax_dn9 = assign28160_e34545_d_n9;

        let (assign28170_e34558, assign28170_e34558_d_n6, assign28170_e34558_d_n7, assign28170_e34558_d_n8, assign28170_e34558_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) {
        let assign28170_e34556: f64 = (var_umax).sqrt();
        (assign28170_e34556, (var_umax_dn6 / (2.0 * assign28170_e34556)), (var_umax_dn7 / (2.0 * assign28170_e34556)), (var_umax_dn8 / (2.0 * assign28170_e34556)), (var_umax_dn9 / (2.0 * assign28170_e34556)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign28170_e34558;
        var_sqrtumax_dn6 = assign28170_e34558_d_n6;
        var_sqrtumax_dn7 = assign28170_e34558_d_n7;
        var_sqrtumax_dn8 = assign28170_e34558_d_n8;
        var_sqrtumax_dn9 = assign28170_e34558_d_n9;

        let (assign28180_e34572, assign28180_e34572_d_n6, assign28180_e34572_d_n7, assign28180_e34572_d_n8, assign28180_e34572_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) {
        let assign28180_e34570: f64 = (var_umax * var_sqrtumax);
        (assign28180_e34570, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign28180_e34572;
        var_umaxpoweronepointfive_dn6 = assign28180_e34572_d_n6;
        var_umaxpoweronepointfive_dn7 = assign28180_e34572_d_n7;
        var_umaxpoweronepointfive_dn8 = assign28180_e34572_d_n8;
        var_umaxpoweronepointfive_dn9 = assign28180_e34572_d_n9;

        let assign28190_e34574: f64 = (-var_pstid_i);
        let assign28190_e34576: f64 = (assign28190_e34574 * var_one_over_one_minus_psti_d);
        let assign28190_e34578: f64 = (-1.0);
        let assign28190_e34579: f64 = if assign28190_e34576 == assign28190_e34578 { 1.0 } else { 0.0 };
        var_guard558 = assign28190_e34579;

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
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_sqrtumax_dn9_slot = var_sqrtumax_dn9;
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
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
    }

    pub(super) fn stamp_transient_block_56(
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatsti_d: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_btat_dn9: f64,
        var_cbbtstid_i: f64,
        var_cerfc: f64,
        var_ctatstid_i: f64,
        var_fbbtsti_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard553: f64,
        var_guard557: f64,
        var_guard558: f64,
        var_one_over_one_minus_psti_d: f64,
        var_perfc: f64,
        var_pstid_i: f64,
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
        var_vbbt: f64,
        var_vbirstid_i: f64,
        var_vbirstiinv_d: f64,
        var_wdepnulrinvsti_d: f64,
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
        var_guard559_slot: &mut f64,
        var_guard560_slot: &mut f64,
        var_guard561_slot: &mut f64,
        var_guard562_slot: &mut f64,
        var_guard563_slot: &mut f64,
        var_guard564_slot: &mut f64,
        var_guard565_slot: &mut f64,
        var_guard566_slot: &mut f64,
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
        let mut var_guard559: f64 = *var_guard559_slot;
        let mut var_guard560: f64 = *var_guard560_slot;
        let mut var_guard561: f64 = *var_guard561_slot;
        let mut var_guard562: f64 = *var_guard562_slot;
        let mut var_guard563: f64 = *var_guard563_slot;
        let mut var_guard564: f64 = *var_guard564_slot;
        let mut var_guard565: f64 = *var_guard565_slot;
        let mut var_guard566: f64 = *var_guard566_slot;
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

        let (assign28200_e34599, assign28200_e34599_d_n6, assign28200_e34599_d_n7, assign28200_e34599_d_n8, assign28200_e34599_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) && (var_guard558 != 0.0)) {
        let assign28200_e34595: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign28200_e34596: f64 = (1.0 + assign28200_e34595);
        let assign28200_e34597: f64 = (1.0 / assign28200_e34596);
        (assign28200_e34597, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign28200_e34596 * assign28200_e34596))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign28200_e34596 * assign28200_e34596))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign28200_e34596 * assign28200_e34596))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign28200_e34596 * assign28200_e34596))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign28200_e34599;
        var_wgamma_dn6 = assign28200_e34599_d_n6;
        var_wgamma_dn7 = assign28200_e34599_d_n7;
        var_wgamma_dn8 = assign28200_e34599_d_n8;
        var_wgamma_dn9 = assign28200_e34599_d_n9;

        let (assign28210_e34623, assign28210_e34623_d_n6, assign28210_e34623_d_n7, assign28210_e34623_d_n8, assign28210_e34623_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) && (var_guard558 == 0.0)) {
        let assign28210_e34615: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign28210_e34616: f64 = (1.0 + assign28210_e34615);
        let assign28210_e34618: f64 = (-var_pstid_i);
        let assign28210_e34620: f64 = (assign28210_e34618 * var_one_over_one_minus_psti_d);
        let assign28210_e34621: f64 = (assign28210_e34616).powf(assign28210_e34620);
        (assign28210_e34621, if 0.0 == 0.0 && ((assign28210_e34620) as f64).is_finite() && ((assign28210_e34620) as f64).fract() == 0.0 { if assign28210_e34620 == 0.0 { 0.0 } else { (assign28210_e34620 * ((assign28210_e34616).powf(assign28210_e34620 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign28210_e34621 * (assign28210_e34620 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign28210_e34616))) }, if 0.0 == 0.0 && ((assign28210_e34620) as f64).is_finite() && ((assign28210_e34620) as f64).fract() == 0.0 { if assign28210_e34620 == 0.0 { 0.0 } else { (assign28210_e34620 * ((assign28210_e34616).powf(assign28210_e34620 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign28210_e34621 * (assign28210_e34620 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign28210_e34616))) }, if 0.0 == 0.0 && ((assign28210_e34620) as f64).is_finite() && ((assign28210_e34620) as f64).fract() == 0.0 { if assign28210_e34620 == 0.0 { 0.0 } else { (assign28210_e34620 * ((assign28210_e34616).powf(assign28210_e34620 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign28210_e34621 * (assign28210_e34620 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign28210_e34616))) }, if 0.0 == 0.0 && ((assign28210_e34620) as f64).is_finite() && ((assign28210_e34620) as f64).fract() == 0.0 { if assign28210_e34620 == 0.0 { 0.0 } else { (assign28210_e34620 * ((assign28210_e34616).powf(assign28210_e34620 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign28210_e34621 * (assign28210_e34620 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign28210_e34616))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign28210_e34623;
        var_wgamma_dn6 = assign28210_e34623_d_n6;
        var_wgamma_dn7 = assign28210_e34623_d_n7;
        var_wgamma_dn8 = assign28210_e34623_d_n8;
        var_wgamma_dn9 = assign28210_e34623_d_n9;

        let (assign28220_e34641, assign28220_e34641_d_n6, assign28220_e34641_d_n7, assign28220_e34641_d_n8, assign28220_e34641_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) {
        let assign28220_e34635: f64 = (var_wsrh * var_wgamma);
        let assign28220_e34638: f64 = (var_wsrh + var_wgamma);
        let assign28220_e34639: f64 = (assign28220_e34635 / assign28220_e34638);
        (assign28220_e34639, ((((var_wsrh * var_wgamma_dn6) * assign28220_e34638) - (assign28220_e34635 * var_wgamma_dn6)) / (assign28220_e34638 * assign28220_e34638)), ((((var_wsrh * var_wgamma_dn7) * assign28220_e34638) - (assign28220_e34635 * var_wgamma_dn7)) / (assign28220_e34638 * assign28220_e34638)), ((((var_wsrh * var_wgamma_dn8) * assign28220_e34638) - (assign28220_e34635 * var_wgamma_dn8)) / (assign28220_e34638 * assign28220_e34638)), ((((var_wsrh * var_wgamma_dn9) * assign28220_e34638) - (assign28220_e34635 * var_wgamma_dn9)) / (assign28220_e34638 * assign28220_e34638)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign28220_e34641;
        var_wtat_dn6 = assign28220_e34641_d_n6;
        var_wtat_dn7 = assign28220_e34641_d_n7;
        var_wtat_dn8 = assign28220_e34641_d_n8;
        var_wtat_dn9 = assign28220_e34641_d_n9;

        let (assign28230_e34658, assign28230_e34658_d_n6, assign28230_e34658_d_n7, assign28230_e34658_d_n8, assign28230_e34658_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) {
        let assign28230_e34654: f64 = (var_btat / var_sqrtumax);
        let assign28230_e34655: f64 = (0.375 * assign28230_e34654);
        let assign28230_e34656: f64 = (assign28230_e34655).sqrt();
        (assign28230_e34656, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign28230_e34656)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign28230_e34656)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign28230_e34656)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign28230_e34656)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign28230_e34658;
        var_ktat_dn6 = assign28230_e34658_d_n6;
        var_ktat_dn7 = assign28230_e34658_d_n7;
        var_ktat_dn8 = assign28230_e34658_d_n8;
        var_ktat_dn9 = assign28230_e34658_d_n9;

        let (assign28240_e34676, assign28240_e34676_d_n6, assign28240_e34676_d_n7, assign28240_e34676_d_n8, assign28240_e34676_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) {
        let assign28240_e34671: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign28240_e34672: f64 = (2.0 * assign28240_e34671);
        let assign28240_e34674: f64 = (assign28240_e34672 - var_umax);
        (assign28240_e34674, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign28240_e34676;
        var_ltat_dn6 = assign28240_e34676_d_n6;
        var_ltat_dn7 = assign28240_e34676_d_n7;
        var_ltat_dn8 = assign28240_e34676_d_n8;
        var_ltat_dn9 = assign28240_e34676_d_n9;

        let (assign28250_e34702, assign28250_e34702_d_n6, assign28250_e34702_d_n7, assign28250_e34702_d_n8, assign28250_e34702_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) {
        let assign28250_e34688: f64 = (var_atatsti_d * var_twoatatoverthreebtat);
        let assign28250_e34690: f64 = (assign28250_e34688 * var_sqrtumax);
        let assign28250_e34693: f64 = (var_atatsti_d * var_umax);
        let assign28250_e34694: f64 = (assign28250_e34690 - assign28250_e34693);
        let assign28250_e34698: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign28250_e34699: f64 = (0.5 * assign28250_e34698);
        let assign28250_e34700: f64 = (assign28250_e34694 + assign28250_e34699);
        (assign28250_e34700, (((((var_atatsti_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign28250_e34688 * var_sqrtumax_dn6)) - (var_atatsti_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign28250_e34688 * var_sqrtumax_dn7)) - (var_atatsti_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign28250_e34688 * var_sqrtumax_dn8)) - (var_atatsti_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign28250_e34688 * var_sqrtumax_dn9)) - (var_atatsti_d * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign28250_e34702;
        var_mtat_dn6 = assign28250_e34702_d_n6;
        var_mtat_dn7 = assign28250_e34702_d_n7;
        var_mtat_dn8 = assign28250_e34702_d_n8;
        var_mtat_dn9 = assign28250_e34702_d_n9;

        let (assign28260_e34718, assign28260_e34718_d_n6, assign28260_e34718_d_n7, assign28260_e34718_d_n8, assign28260_e34718_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) {
        let assign28260_e34714: f64 = (var_ltat - 1.0);
        let assign28260_e34716: f64 = (assign28260_e34714 * var_ktat);
        (assign28260_e34716, ((var_ltat_dn6 * var_ktat) + (assign28260_e34714 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign28260_e34714 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign28260_e34714 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign28260_e34714 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign28260_e34718;
        var_xerfc_dn6 = assign28260_e34718_d_n6;
        var_xerfc_dn7 = assign28260_e34718_d_n7;
        var_xerfc_dn8 = assign28260_e34718_d_n8;
        var_xerfc_dn9 = assign28260_e34718_d_n9;

        let (assign28270_e34732, assign28270_e34732_d_n6, assign28270_e34732_d_n7, assign28270_e34732_d_n8, assign28270_e34732_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) {
        let assign28270_e34730: f64 = (var_xerfc * var_xerfc);
        (assign28270_e34730, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign28270_e34732;
        var_ysq_dn6 = assign28270_e34732_d_n6;
        var_ysq_dn7 = assign28270_e34732_d_n7;
        var_ysq_dn8 = assign28270_e34732_d_n8;
        var_ysq_dn9 = assign28270_e34732_d_n9;

        let assign28280_e34735: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard559 = assign28280_e34735;

        let (assign28290_e34755, assign28290_e34755_d_n6, assign28290_e34755_d_n7, assign28290_e34755_d_n8, assign28290_e34755_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) && (var_guard559 != 0.0)) {
        let assign28290_e34751: f64 = (var_perfc * var_xerfc);
        let assign28290_e34752: f64 = (1.0 + assign28290_e34751);
        let assign28290_e34753: f64 = (1.0 / assign28290_e34752);
        (assign28290_e34753, (-((var_perfc * var_xerfc_dn6) / (assign28290_e34752 * assign28290_e34752))), (-((var_perfc * var_xerfc_dn7) / (assign28290_e34752 * assign28290_e34752))), (-((var_perfc * var_xerfc_dn8) / (assign28290_e34752 * assign28290_e34752))), (-((var_perfc * var_xerfc_dn9) / (assign28290_e34752 * assign28290_e34752))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign28290_e34755;
        var_terfc_dn6 = assign28290_e34755_d_n6;
        var_terfc_dn7 = assign28290_e34755_d_n7;
        var_terfc_dn8 = assign28290_e34755_d_n8;
        var_terfc_dn9 = assign28290_e34755_d_n9;

        let (assign28300_e34776, assign28300_e34776_d_n6, assign28300_e34776_d_n7, assign28300_e34776_d_n8, assign28300_e34776_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) && (var_guard559 == 0.0)) {
        let assign28300_e34772: f64 = (var_perfc * var_xerfc);
        let assign28300_e34773: f64 = (1.0 - assign28300_e34772);
        let assign28300_e34774: f64 = (1.0 / assign28300_e34773);
        (assign28300_e34774, (-((-(var_perfc * var_xerfc_dn6)) / (assign28300_e34773 * assign28300_e34773))), (-((-(var_perfc * var_xerfc_dn7)) / (assign28300_e34773 * assign28300_e34773))), (-((-(var_perfc * var_xerfc_dn8)) / (assign28300_e34773 * assign28300_e34773))), (-((-(var_perfc * var_xerfc_dn9)) / (assign28300_e34773 * assign28300_e34773))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign28300_e34776;
        var_terfc_dn6 = assign28300_e34776_d_n6;
        var_terfc_dn7 = assign28300_e34776_d_n7;
        var_terfc_dn8 = assign28300_e34776_d_n8;
        var_terfc_dn9 = assign28300_e34776_d_n9;

        let assign28310_e34778: f64 = (-var_ysq);
        let assign28310_e34780: f64 = (assign28310_e34778 + var_mtat);
        let assign28310_e34782: f64 = (-230.25850929940458);
        let assign28310_e34783: f64 = if assign28310_e34780 > assign28310_e34782 { 1.0 } else { 0.0 };
        var_guard560 = assign28310_e34783;

        let (assign28320_e34801, assign28320_e34801_d_n6, assign28320_e34801_d_n7, assign28320_e34801_d_n8, assign28320_e34801_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) && (var_guard560 != 0.0)) {
        let assign28320_e34796: f64 = (-var_ysq);
        let assign28320_e34798: f64 = (assign28320_e34796 + var_mtat);
        let assign28320_e34799: f64 = (assign28320_e34798).exp();
        (assign28320_e34799, (assign28320_e34799 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign28320_e34799 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign28320_e34799 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign28320_e34799 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign28320_e34801;
        var_tmp_dn6 = assign28320_e34801_d_n6;
        var_tmp_dn7 = assign28320_e34801_d_n7;
        var_tmp_dn8 = assign28320_e34801_d_n8;
        var_tmp_dn9 = assign28320_e34801_d_n9;

        let (assign28330_e34850, assign28330_e34850_d_n6, assign28330_e34850_d_n7, assign28330_e34850_d_n8, assign28330_e34850_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) && (var_guard560 == 0.0)) {
        let assign28330_e34817: f64 = (-230.25850929940458);
        let assign28330_e34819: f64 = (-var_ysq);
        let assign28330_e34821: f64 = (assign28330_e34819 + var_mtat);
        let assign28330_e34822: f64 = (assign28330_e34817 - assign28330_e34821);
        let assign28330_e34826: f64 = (-230.25850929940458);
        let assign28330_e34828: f64 = (-var_ysq);
        let assign28330_e34830: f64 = (assign28330_e34828 + var_mtat);
        let assign28330_e34831: f64 = (assign28330_e34826 - assign28330_e34830);
        let assign28330_e34834: f64 = (-230.25850929940458);
        let assign28330_e34836: f64 = (-var_ysq);
        let assign28330_e34838: f64 = (assign28330_e34836 + var_mtat);
        let assign28330_e34839: f64 = (assign28330_e34834 - assign28330_e34838);
        let assign28330_e34841: f64 = (assign28330_e34839 * 0.3333333333333333);
        let assign28330_e34842: f64 = (1.0 + assign28330_e34841);
        let assign28330_e34843: f64 = (assign28330_e34831 * assign28330_e34842);
        let assign28330_e34844: f64 = (0.5 * assign28330_e34843);
        let assign28330_e34845: f64 = (1.0 + assign28330_e34844);
        let assign28330_e34846: f64 = (assign28330_e34822 * assign28330_e34845);
        let assign28330_e34847: f64 = (1.0 + assign28330_e34846);
        let assign28330_e34848: f64 = (1e-100 / assign28330_e34847);
        (assign28330_e34848, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign28330_e34845) + (assign28330_e34822 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign28330_e34842) + (assign28330_e34831 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign28330_e34847 * assign28330_e34847))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign28330_e34845) + (assign28330_e34822 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign28330_e34842) + (assign28330_e34831 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign28330_e34847 * assign28330_e34847))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign28330_e34845) + (assign28330_e34822 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign28330_e34842) + (assign28330_e34831 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign28330_e34847 * assign28330_e34847))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign28330_e34845) + (assign28330_e34822 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign28330_e34842) + (assign28330_e34831 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign28330_e34847 * assign28330_e34847))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign28330_e34850;
        var_tmp_dn6 = assign28330_e34850_d_n6;
        var_tmp_dn7 = assign28330_e34850_d_n7;
        var_tmp_dn8 = assign28330_e34850_d_n8;
        var_tmp_dn9 = assign28330_e34850_d_n9;

        let (assign28340_e34880, assign28340_e34880_d_n6, assign28340_e34880_d_n7, assign28340_e34880_d_n8, assign28340_e34880_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) {
        let assign28340_e34862: f64 = (0.29214664 * var_terfc);
        let assign28340_e34866: f64 = (var_terfc * var_terfc);
        let assign28340_e34867: f64 = (var_berfc * assign28340_e34866);
        let assign28340_e34868: f64 = (assign28340_e34862 + assign28340_e34867);
        let assign28340_e34872: f64 = (var_terfc * var_terfc);
        let assign28340_e34874: f64 = (assign28340_e34872 * var_terfc);
        let assign28340_e34875: f64 = (var_cerfc * assign28340_e34874);
        let assign28340_e34876: f64 = (assign28340_e34868 + assign28340_e34875);
        let assign28340_e34878: f64 = (assign28340_e34876 * var_tmp);
        (assign28340_e34878, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign28340_e34872 * var_terfc_dn6)))) * var_tmp) + (assign28340_e34876 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign28340_e34872 * var_terfc_dn7)))) * var_tmp) + (assign28340_e34876 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign28340_e34872 * var_terfc_dn8)))) * var_tmp) + (assign28340_e34876 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign28340_e34872 * var_terfc_dn9)))) * var_tmp) + (assign28340_e34876 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign28340_e34880;
        var_erfcpos_dn6 = assign28340_e34880_d_n6;
        var_erfcpos_dn7 = assign28340_e34880_d_n7;
        var_erfcpos_dn8 = assign28340_e34880_d_n8;
        var_erfcpos_dn9 = assign28340_e34880_d_n9;

        let assign28350_e34883: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard561 = assign28350_e34883;

        let (assign28360_e34897, assign28360_e34897_d_n6, assign28360_e34897_d_n7, assign28360_e34897_d_n8, assign28360_e34897_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) && (var_guard561 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign28360_e34897;
        var_erfctimesexpmtat_dn6 = assign28360_e34897_d_n6;
        var_erfctimesexpmtat_dn7 = assign28360_e34897_d_n7;
        var_erfctimesexpmtat_dn8 = assign28360_e34897_d_n8;
        var_erfctimesexpmtat_dn9 = assign28360_e34897_d_n9;

        let assign28370_e34900: f64 = (-230.25850929940458);
        let assign28370_e34901: f64 = if var_mtat > assign28370_e34900 { 1.0 } else { 0.0 };
        var_guard562 = assign28370_e34901;

        let (assign28380_e34919, assign28380_e34919_d_n6, assign28380_e34919_d_n7, assign28380_e34919_d_n8, assign28380_e34919_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) && (var_guard561 == 0.0)) && (var_guard562 != 0.0)) {
        let assign28380_e34917: f64 = (var_mtat).exp();
        (assign28380_e34917, (assign28380_e34917 * var_mtat_dn6), (assign28380_e34917 * var_mtat_dn7), (assign28380_e34917 * var_mtat_dn8), (assign28380_e34917 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign28380_e34919;
        var_tmp_dn6 = assign28380_e34919_d_n6;
        var_tmp_dn7 = assign28380_e34919_d_n7;
        var_tmp_dn8 = assign28380_e34919_d_n8;
        var_tmp_dn9 = assign28380_e34919_d_n9;

        let (assign28390_e34962, assign28390_e34962_d_n6, assign28390_e34962_d_n7, assign28390_e34962_d_n8, assign28390_e34962_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) && (var_guard561 == 0.0)) && (var_guard562 == 0.0)) {
        let assign28390_e34938: f64 = (-230.25850929940458);
        let assign28390_e34940: f64 = (assign28390_e34938 - var_mtat);
        let assign28390_e34944: f64 = (-230.25850929940458);
        let assign28390_e34946: f64 = (assign28390_e34944 - var_mtat);
        let assign28390_e34949: f64 = (-230.25850929940458);
        let assign28390_e34951: f64 = (assign28390_e34949 - var_mtat);
        let assign28390_e34953: f64 = (assign28390_e34951 * 0.3333333333333333);
        let assign28390_e34954: f64 = (1.0 + assign28390_e34953);
        let assign28390_e34955: f64 = (assign28390_e34946 * assign28390_e34954);
        let assign28390_e34956: f64 = (0.5 * assign28390_e34955);
        let assign28390_e34957: f64 = (1.0 + assign28390_e34956);
        let assign28390_e34958: f64 = (assign28390_e34940 * assign28390_e34957);
        let assign28390_e34959: f64 = (1.0 + assign28390_e34958);
        let assign28390_e34960: f64 = (1e-100 / assign28390_e34959);
        (assign28390_e34960, (-((1e-100 * (((-var_mtat_dn6) * assign28390_e34957) + (assign28390_e34940 * (0.5 * (((-var_mtat_dn6) * assign28390_e34954) + (assign28390_e34946 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign28390_e34959 * assign28390_e34959))), (-((1e-100 * (((-var_mtat_dn7) * assign28390_e34957) + (assign28390_e34940 * (0.5 * (((-var_mtat_dn7) * assign28390_e34954) + (assign28390_e34946 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign28390_e34959 * assign28390_e34959))), (-((1e-100 * (((-var_mtat_dn8) * assign28390_e34957) + (assign28390_e34940 * (0.5 * (((-var_mtat_dn8) * assign28390_e34954) + (assign28390_e34946 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign28390_e34959 * assign28390_e34959))), (-((1e-100 * (((-var_mtat_dn9) * assign28390_e34957) + (assign28390_e34940 * (0.5 * (((-var_mtat_dn9) * assign28390_e34954) + (assign28390_e34946 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign28390_e34959 * assign28390_e34959))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign28390_e34962;
        var_tmp_dn6 = assign28390_e34962_d_n6;
        var_tmp_dn7 = assign28390_e34962_d_n7;
        var_tmp_dn8 = assign28390_e34962_d_n8;
        var_tmp_dn9 = assign28390_e34962_d_n9;

        let (assign28400_e34981, assign28400_e34981_d_n6, assign28400_e34981_d_n7, assign28400_e34981_d_n8, assign28400_e34981_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) && (var_guard561 == 0.0)) {
        let assign28400_e34977: f64 = (2.0 * var_tmp);
        let assign28400_e34979: f64 = (assign28400_e34977 - var_erfcpos);
        (assign28400_e34979, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign28400_e34981;
        var_erfctimesexpmtat_dn6 = assign28400_e34981_d_n6;
        var_erfctimesexpmtat_dn7 = assign28400_e34981_d_n7;
        var_erfctimesexpmtat_dn8 = assign28400_e34981_d_n8;
        var_erfctimesexpmtat_dn9 = assign28400_e34981_d_n9;

        let (assign28410_e35001, assign28410_e35001_d_n6, assign28410_e35001_d_n7, assign28410_e35001_d_n8, assign28410_e35001_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) {
        let assign28410_e34993: f64 = (1.772453850905516 * 0.5);
        let assign28410_e34996: f64 = (var_atatsti_d * var_erfctimesexpmtat);
        let assign28410_e34998: f64 = (assign28410_e34996 / var_ktat);
        let assign28410_e34999: f64 = (assign28410_e34993 * assign28410_e34998);
        (assign28410_e34999, (assign28410_e34993 * ((((var_atatsti_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign28410_e34996 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign28410_e34993 * ((((var_atatsti_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign28410_e34996 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign28410_e34993 * ((((var_atatsti_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign28410_e34996 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign28410_e34993 * ((((var_atatsti_d * var_erfctimesexpmtat_dn9) * var_ktat) - (assign28410_e34996 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign28410_e35001;
        var_gammamax_dn6 = assign28410_e35001_d_n6;
        var_gammamax_dn7 = assign28410_e35001_d_n7;
        var_gammamax_dn8 = assign28410_e35001_d_n8;
        var_gammamax_dn9 = assign28410_e35001_d_n9;

        let (assign28420_e35019, assign28420_e35019_d_n6, assign28420_e35019_d_n7, assign28420_e35019_d_n8, assign28420_e35019_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard557 == 0.0)) {
        let assign28420_e35014: f64 = (var_asrh * var_gammamax);
        let assign28420_e35016: f64 = (assign28420_e35014 * var_wtat);
        let assign28420_e35017: f64 = (var_ctatstid_i * assign28420_e35016);
        (assign28420_e35017, (var_ctatstid_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign28420_e35014 * var_wtat_dn6))), (var_ctatstid_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign28420_e35014 * var_wtat_dn7))), (var_ctatstid_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign28420_e35014 * var_wtat_dn8))), (var_ctatstid_i * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign28420_e35014 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign28420_e35019;
        var_itat_dn6 = assign28420_e35019_d_n6;
        var_itat_dn7 = assign28420_e35019_d_n7;
        var_itat_dn8 = assign28420_e35019_d_n8;
        var_itat_dn9 = assign28420_e35019_d_n9;

        let assign28430_e35022: f64 = if var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard563 = assign28430_e35022;

        let (assign28440_e35033, assign28440_e35033_d_n6, assign28440_e35033_d_n7, assign28440_e35033_d_n8, assign28440_e35033_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard563 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign28440_e35033;
        var_ibbt_dn6 = assign28440_e35033_d_n6;
        var_ibbt_dn7 = assign28440_e35033_d_n7;
        var_ibbt_dn8 = assign28440_e35033_d_n8;
        var_ibbt_dn9 = assign28440_e35033_d_n9;

        let assign28450_e35036: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard564 = assign28450_e35036;

        let (assign28460_e35055, assign28460_e35055_d_n6, assign28460_e35055_d_n7, assign28460_e35055_d_n8, assign28460_e35055_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard563 == 0.0)) && (var_guard564 != 0.0)) {
        let assign28460_e35050: f64 = (var_vbirstid_i - var_vbbt);
        let assign28460_e35052: f64 = (assign28460_e35050 * var_vbirstiinv_d);
        let assign28460_e35053: f64 = (assign28460_e35052).sqrt();
        (assign28460_e35053, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign28460_e35055;
        var_tmp_dn6 = assign28460_e35055_d_n6;
        var_tmp_dn7 = assign28460_e35055_d_n7;
        var_tmp_dn8 = assign28460_e35055_d_n8;
        var_tmp_dn9 = assign28460_e35055_d_n9;

        let (assign28470_e35076, assign28470_e35076_d_n6, assign28470_e35076_d_n7, assign28470_e35076_d_n8, assign28470_e35076_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard563 == 0.0)) && (var_guard564 == 0.0)) {
        let assign28470_e35070: f64 = (var_vbirstid_i - var_vbbt);
        let assign28470_e35072: f64 = (assign28470_e35070 * var_vbirstiinv_d);
        let assign28470_e35074: f64 = (assign28470_e35072).powf(var_pstid_i);
        (assign28470_e35074, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign28470_e35076;
        var_tmp_dn6 = assign28470_e35076_d_n6;
        var_tmp_dn7 = assign28470_e35076_d_n7;
        var_tmp_dn8 = assign28470_e35076_d_n8;
        var_tmp_dn9 = assign28470_e35076_d_n9;

        let (assign28480_e35096, assign28480_e35096_d_n6, assign28480_e35096_d_n7, assign28480_e35096_d_n8, assign28480_e35096_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard563 == 0.0)) {
        let assign28480_e35089: f64 = (var_vbirstid_i - var_vbbt);
        let assign28480_e35091: f64 = (assign28480_e35089 * var_wdepnulrinvsti_d);
        let assign28480_e35093: f64 = (assign28480_e35091 / var_tmp);
        let assign28480_e35094: f64 = (var_one_over_one_minus_psti_d * assign28480_e35093);
        (assign28480_e35094, (var_one_over_one_minus_psti_d * (-((assign28480_e35091 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign28480_e35091 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign28480_e35091 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign28480_e35091 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign28480_e35096;
        var_fmaxr_dn6 = assign28480_e35096_d_n6;
        var_fmaxr_dn7 = assign28480_e35096_d_n7;
        var_fmaxr_dn8 = assign28480_e35096_d_n8;
        var_fmaxr_dn9 = assign28480_e35096_d_n9;

        let assign28490_e35098: f64 = (-var_fbbtsti_d);
        let assign28490_e35100: f64 = (assign28490_e35098 / var_fmaxr);
        let assign28490_e35101: f64 = (assign28490_e35100).abs();
        let assign28490_e35103: f64 = if assign28490_e35101 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard565 = assign28490_e35103;

        let (assign28500_e35121, assign28500_e35121_d_n6, assign28500_e35121_d_n7, assign28500_e35121_d_n8, assign28500_e35121_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard563 == 0.0)) && (var_guard565 != 0.0)) {
        let assign28500_e35116: f64 = (-var_fbbtsti_d);
        let assign28500_e35118: f64 = (assign28500_e35116 / var_fmaxr);
        let assign28500_e35119: f64 = (assign28500_e35118).exp();
        (assign28500_e35119, (assign28500_e35119 * (-((assign28500_e35116 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign28500_e35119 * (-((assign28500_e35116 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign28500_e35119 * (-((assign28500_e35116 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign28500_e35119 * (-((assign28500_e35116 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign28500_e35121;
        var_tmp_dn6 = assign28500_e35121_d_n6;
        var_tmp_dn7 = assign28500_e35121_d_n7;
        var_tmp_dn8 = assign28500_e35121_d_n8;
        var_tmp_dn9 = assign28500_e35121_d_n9;

        let assign28510_e35123: f64 = (-var_fbbtsti_d);
        let assign28510_e35125: f64 = (assign28510_e35123 / var_fmaxr);
        let assign28510_e35127: f64 = if assign28510_e35125 < 0.0 { 1.0 } else { 0.0 };
        var_guard566 = assign28510_e35127;

        let (assign28520_e35178, assign28520_e35178_d_n6, assign28520_e35178_d_n7, assign28520_e35178_d_n8, assign28520_e35178_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard563 == 0.0)) && (var_guard565 == 0.0)) && (var_guard566 != 0.0)) {
        let assign28520_e35145: f64 = (-230.25850929940458);
        let assign28520_e35147: f64 = (-var_fbbtsti_d);
        let assign28520_e35149: f64 = (assign28520_e35147 / var_fmaxr);
        let assign28520_e35150: f64 = (assign28520_e35145 - assign28520_e35149);
        let assign28520_e35154: f64 = (-230.25850929940458);
        let assign28520_e35156: f64 = (-var_fbbtsti_d);
        let assign28520_e35158: f64 = (assign28520_e35156 / var_fmaxr);
        let assign28520_e35159: f64 = (assign28520_e35154 - assign28520_e35158);
        let assign28520_e35162: f64 = (-230.25850929940458);
        let assign28520_e35164: f64 = (-var_fbbtsti_d);
        let assign28520_e35166: f64 = (assign28520_e35164 / var_fmaxr);
        let assign28520_e35167: f64 = (assign28520_e35162 - assign28520_e35166);
        let assign28520_e35169: f64 = (assign28520_e35167 * 0.3333333333333333);
        let assign28520_e35170: f64 = (1.0 + assign28520_e35169);
        let assign28520_e35171: f64 = (assign28520_e35159 * assign28520_e35170);
        let assign28520_e35172: f64 = (0.5 * assign28520_e35171);
        let assign28520_e35173: f64 = (1.0 + assign28520_e35172);
        let assign28520_e35174: f64 = (assign28520_e35150 * assign28520_e35173);
        let assign28520_e35175: f64 = (1.0 + assign28520_e35174);
        let assign28520_e35176: f64 = (1e-100 / assign28520_e35175);
        (assign28520_e35176, (-((1e-100 * (((-(-((assign28520_e35147 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign28520_e35173) + (assign28520_e35150 * (0.5 * (((-(-((assign28520_e35156 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign28520_e35170) + (assign28520_e35159 * ((-(-((assign28520_e35164 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign28520_e35175 * assign28520_e35175))), (-((1e-100 * (((-(-((assign28520_e35147 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign28520_e35173) + (assign28520_e35150 * (0.5 * (((-(-((assign28520_e35156 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign28520_e35170) + (assign28520_e35159 * ((-(-((assign28520_e35164 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign28520_e35175 * assign28520_e35175))), (-((1e-100 * (((-(-((assign28520_e35147 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign28520_e35173) + (assign28520_e35150 * (0.5 * (((-(-((assign28520_e35156 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign28520_e35170) + (assign28520_e35159 * ((-(-((assign28520_e35164 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign28520_e35175 * assign28520_e35175))), (-((1e-100 * (((-(-((assign28520_e35147 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign28520_e35173) + (assign28520_e35150 * (0.5 * (((-(-((assign28520_e35156 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign28520_e35170) + (assign28520_e35159 * ((-(-((assign28520_e35164 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign28520_e35175 * assign28520_e35175))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign28520_e35178;
        var_tmp_dn6 = assign28520_e35178_d_n6;
        var_tmp_dn7 = assign28520_e35178_d_n7;
        var_tmp_dn8 = assign28520_e35178_d_n8;
        var_tmp_dn9 = assign28520_e35178_d_n9;

        let (assign28530_e35227, assign28530_e35227_d_n6, assign28530_e35227_d_n7, assign28530_e35227_d_n8, assign28530_e35227_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard563 == 0.0)) && (var_guard565 == 0.0)) && (var_guard566 == 0.0)) {
        let assign28530_e35197: f64 = (-var_fbbtsti_d);
        let assign28530_e35199: f64 = (assign28530_e35197 / var_fmaxr);
        let assign28530_e35201: f64 = (assign28530_e35199 - 230.25850929940458);
        let assign28530_e35205: f64 = (-var_fbbtsti_d);
        let assign28530_e35207: f64 = (assign28530_e35205 / var_fmaxr);
        let assign28530_e35209: f64 = (assign28530_e35207 - 230.25850929940458);
        let assign28530_e35212: f64 = (-var_fbbtsti_d);
        let assign28530_e35214: f64 = (assign28530_e35212 / var_fmaxr);
        let assign28530_e35216: f64 = (assign28530_e35214 - 230.25850929940458);
        let assign28530_e35218: f64 = (assign28530_e35216 * 0.3333333333333333);
        let assign28530_e35219: f64 = (1.0 + assign28530_e35218);
        let assign28530_e35220: f64 = (assign28530_e35209 * assign28530_e35219);
        let assign28530_e35221: f64 = (0.5 * assign28530_e35220);
        let assign28530_e35222: f64 = (1.0 + assign28530_e35221);
        let assign28530_e35223: f64 = (assign28530_e35201 * assign28530_e35222);
        let assign28530_e35224: f64 = (1.0 + assign28530_e35223);
        let assign28530_e35225: f64 = (1e100 * assign28530_e35224);
        (assign28530_e35225, (1e100 * (((-((assign28530_e35197 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign28530_e35222) + (assign28530_e35201 * (0.5 * (((-((assign28530_e35205 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign28530_e35219) + (assign28530_e35209 * ((-((assign28530_e35212 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign28530_e35197 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign28530_e35222) + (assign28530_e35201 * (0.5 * (((-((assign28530_e35205 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign28530_e35219) + (assign28530_e35209 * ((-((assign28530_e35212 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign28530_e35197 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign28530_e35222) + (assign28530_e35201 * (0.5 * (((-((assign28530_e35205 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign28530_e35219) + (assign28530_e35209 * ((-((assign28530_e35212 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign28530_e35197 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign28530_e35222) + (assign28530_e35201 * (0.5 * (((-((assign28530_e35205 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign28530_e35219) + (assign28530_e35209 * ((-((assign28530_e35212 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign28530_e35227;
        var_tmp_dn6 = assign28530_e35227_d_n6;
        var_tmp_dn7 = assign28530_e35227_d_n7;
        var_tmp_dn8 = assign28530_e35227_d_n8;
        var_tmp_dn9 = assign28530_e35227_d_n9;

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
        *var_guard559_slot = var_guard559;
        *var_guard560_slot = var_guard560;
        *var_guard561_slot = var_guard561;
        *var_guard562_slot = var_guard562;
        *var_guard563_slot = var_guard563;
        *var_guard564_slot = var_guard564;
        *var_guard565_slot = var_guard565;
        *var_guard566_slot = var_guard566;
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

    pub(super) fn stamp_transient_block_57(
        p: &Parameters,
        var_alphaav: f64,
        var_atatgat_d: f64,
        var_btatpartgat_d: f64,
        var_cbbtstid_i: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_fmaxr: f64,
        var_fmaxr_dn6: f64,
        var_fmaxr_dn7: f64,
        var_fmaxr_dn8: f64,
        var_fmaxr_dn9: f64,
        var_fstopsti_d: f64,
        var_ftdgat_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard553: f64,
        var_guard563: f64,
        var_idmult: f64,
        var_idsatgat_d: f64,
        var_lgdrain_i: f64,
        var_one_minus_pgat_d: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_pbrstid_i: f64,
        var_pgatd_i: f64,
        var_slopesti_d: f64,
        var_two_psistar: f64,
        var_v1: f64,
        var_vav: f64,
        var_vbigat_d: f64,
        var_vbirgatinv_d: f64,
        var_vbrinvsti_d: f64,
        var_vbrstid_i: f64,
        var_vjsrh: f64,
        var_wdepnulrgat_d: f64,
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
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_sqrtumax_dn9_slot: &mut f64,
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
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_sqrtumax_dn9: f64 = *var_sqrtumax_dn9_slot;
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

        let (assign28540_e35247, assign28540_e35247_d_n6, assign28540_e35247_d_n7, assign28540_e35247_d_n8, assign28540_e35247_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard563 == 0.0)) {
        let assign28540_e35240: f64 = (var_v1 * var_fmaxr);
        let assign28540_e35242: f64 = (assign28540_e35240 * var_fmaxr);
        let assign28540_e35244: f64 = (assign28540_e35242 * var_tmp);
        let assign28540_e35245: f64 = (var_cbbtstid_i * assign28540_e35244);
        (assign28540_e35245, (var_cbbtstid_i * (((((var_v1 * var_fmaxr_dn6) * var_fmaxr) + (assign28540_e35240 * var_fmaxr_dn6)) * var_tmp) + (assign28540_e35242 * var_tmp_dn6))), (var_cbbtstid_i * (((((var_v1 * var_fmaxr_dn7) * var_fmaxr) + (assign28540_e35240 * var_fmaxr_dn7)) * var_tmp) + (assign28540_e35242 * var_tmp_dn7))), (var_cbbtstid_i * (((((var_v1 * var_fmaxr_dn8) * var_fmaxr) + (assign28540_e35240 * var_fmaxr_dn8)) * var_tmp) + (assign28540_e35242 * var_tmp_dn8))), (var_cbbtstid_i * (((((var_v1 * var_fmaxr_dn9) * var_fmaxr) + (assign28540_e35240 * var_fmaxr_dn9)) * var_tmp) + (assign28540_e35242 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign28540_e35247;
        var_ibbt_dn6 = assign28540_e35247_d_n6;
        var_ibbt_dn7 = assign28540_e35247_d_n7;
        var_ibbt_dn8 = assign28540_e35247_d_n8;
        var_ibbt_dn9 = assign28540_e35247_d_n9;

        let assign28550_e35250: f64 = if var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard567 = assign28550_e35250;

        let (assign28560_e35261, assign28560_e35261_d_n6, assign28560_e35261_d_n7, assign28560_e35261_d_n8, assign28560_e35261_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard567 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign28560_e35261;
        var_fbreakdown_dn6 = assign28560_e35261_d_n6;
        var_fbreakdown_dn7 = assign28560_e35261_d_n7;
        var_fbreakdown_dn8 = assign28560_e35261_d_n8;
        var_fbreakdown_dn9 = assign28560_e35261_d_n9;

        let assign28570_e35264: f64 = (-var_alphaav);
        let assign28570_e35266: f64 = (assign28570_e35264 * var_vbrstid_i);
        let assign28570_e35267: f64 = if var_vav > assign28570_e35266 { 1.0 } else { 0.0 };
        var_guard568 = assign28570_e35267;

        let assign28580_e35270: f64 = if var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        var_guard569 = assign28580_e35270;

        let (assign28590_e35300, assign28590_e35300_d_n6, assign28590_e35300_d_n7, assign28590_e35300_d_n8, assign28590_e35300_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard567 == 0.0)) && (var_guard568 != 0.0)) && (var_guard569 != 0.0)) {
        let assign28590_e35286: f64 = (var_vav * var_vbrinvsti_d);
        let assign28590_e35289: f64 = (var_vav * var_vbrinvsti_d);
        let assign28590_e35290: f64 = (assign28590_e35286 * assign28590_e35289);
        let assign28590_e35293: f64 = (var_vav * var_vbrinvsti_d);
        let assign28590_e35294: f64 = (assign28590_e35290 * assign28590_e35293);
        let assign28590_e35297: f64 = (var_vav * var_vbrinvsti_d);
        let assign28590_e35298: f64 = (assign28590_e35294 * assign28590_e35297);
        (assign28590_e35298, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign28590_e35300;
        var_tmp_dn6 = assign28590_e35300_d_n6;
        var_tmp_dn7 = assign28590_e35300_d_n7;
        var_tmp_dn8 = assign28590_e35300_d_n8;
        var_tmp_dn9 = assign28590_e35300_d_n9;

        let (assign28600_e35322, assign28600_e35322_d_n6, assign28600_e35322_d_n7, assign28600_e35322_d_n8, assign28600_e35322_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard567 == 0.0)) && (var_guard568 != 0.0)) && (var_guard569 == 0.0)) {
        let assign28600_e35317: f64 = (var_vav * var_vbrinvsti_d);
        let assign28600_e35318: f64 = (assign28600_e35317).abs();
        let assign28600_e35320: f64 = (assign28600_e35318).powf(var_pbrstid_i);
        (assign28600_e35320, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign28600_e35322;
        var_tmp_dn6 = assign28600_e35322_d_n6;
        var_tmp_dn7 = assign28600_e35322_d_n7;
        var_tmp_dn8 = assign28600_e35322_d_n8;
        var_tmp_dn9 = assign28600_e35322_d_n9;

        let (assign28610_e35340, assign28610_e35340_d_n6, assign28610_e35340_d_n7, assign28610_e35340_d_n8, assign28610_e35340_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard567 == 0.0)) && (var_guard568 != 0.0)) {
        let assign28610_e35337: f64 = (1.0 - var_tmp);
        let assign28610_e35338: f64 = (1.0 / assign28610_e35337);
        (assign28610_e35338, (-((-var_tmp_dn6) / (assign28610_e35337 * assign28610_e35337))), (-((-var_tmp_dn7) / (assign28610_e35337 * assign28610_e35337))), (-((-var_tmp_dn8) / (assign28610_e35337 * assign28610_e35337))), (-((-var_tmp_dn9) / (assign28610_e35337 * assign28610_e35337))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign28610_e35340;
        var_fbreakdown_dn6 = assign28610_e35340_d_n6;
        var_fbreakdown_dn7 = assign28610_e35340_d_n7;
        var_fbreakdown_dn8 = assign28610_e35340_d_n8;
        var_fbreakdown_dn9 = assign28610_e35340_d_n9;

        let (assign28620_e35363, assign28620_e35363_d_n6, assign28620_e35363_d_n7, assign28620_e35363_d_n8, assign28620_e35363_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) && (var_guard567 == 0.0)) && (var_guard568 == 0.0)) {
        let assign28620_e35357: f64 = (var_alphaav * var_vbrstid_i);
        let assign28620_e35358: f64 = (var_vav + assign28620_e35357);
        let assign28620_e35360: f64 = (assign28620_e35358 * var_slopesti_d);
        let assign28620_e35361: f64 = (var_fstopsti_d + assign28620_e35360);
        (assign28620_e35361, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign28620_e35363;
        var_fbreakdown_dn6 = assign28620_e35363_d_n6;
        var_fbreakdown_dn7 = assign28620_e35363_d_n7;
        var_fbreakdown_dn8 = assign28620_e35363_d_n8;
        var_fbreakdown_dn9 = assign28620_e35363_d_n9;

        let (assign28630_e35382, assign28630_e35382_d_n6, assign28630_e35382_d_n7, assign28630_e35382_d_n8, assign28630_e35382_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard553 == 0.0)) {
        let assign28630_e35373: f64 = (var_id__blk212 + var_isrh);
        let assign28630_e35375: f64 = (assign28630_e35373 + var_itat);
        let assign28630_e35377: f64 = (assign28630_e35375 + var_ibbt);
        let assign28630_e35378: f64 = (p.p29 * assign28630_e35377);
        let assign28630_e35380: f64 = (assign28630_e35378 * var_fbreakdown);
        (assign28630_e35380, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign28630_e35378 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign28630_e35378 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign28630_e35378 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign28630_e35378 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign28630_e35382;
        var_ijunsti_dn6 = assign28630_e35382_d_n6;
        var_ijunsti_dn7 = assign28630_e35382_d_n7;
        var_ijunsti_dn8 = assign28630_e35382_d_n8;
        var_ijunsti_dn9 = assign28630_e35382_d_n9;

        let assign28640_e35385: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard570 = assign28640_e35385;

        let (assign28650_e35393, assign28650_e35393_d_n6, assign28650_e35393_d_n7, assign28650_e35393_d_n8, assign28650_e35393_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign28650_e35393;
        var_ijungat_dn6 = assign28650_e35393_d_n6;
        var_ijungat_dn7 = assign28650_e35393_d_n7;
        var_ijungat_dn8 = assign28650_e35393_d_n8;
        var_ijungat_dn9 = assign28650_e35393_d_n9;

        let (assign28660_e35404,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) {
        let assign28660_e35402: f64 = (var_idsatgat_d * var_idmult);
        (assign28660_e35402,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign28660_e35404;

        let assign28670_e35411: f64 = if ((var_csrhgatd_i == 0.0) && (var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard571 = assign28670_e35411;

        let (assign28680_e35422, assign28680_e35422_d_n6, assign28680_e35422_d_n7, assign28680_e35422_d_n8, assign28680_e35422_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard571 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign28680_e35422;
        var_isrh_dn6 = assign28680_e35422_d_n6;
        var_isrh_dn7 = assign28680_e35422_d_n7;
        var_isrh_dn8 = assign28680_e35422_d_n8;
        var_isrh_dn9 = assign28680_e35422_d_n9;

        let (assign28690_e35436,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard571 == 0.0)) {
        let assign28690_e35434: f64 = (var_vbigat_d - var_vjsrh);
        (assign28690_e35434,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign28690_e35436;

        let (assign28700_e35455,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard571 == 0.0)) {
        let assign28700_e35450: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign28700_e35451: f64 = (1.0 - assign28700_e35450);
        let assign28700_e35452: f64 = (assign28700_e35451).sqrt();
        let assign28700_e35453: f64 = (1.0 - assign28700_e35452);
        (assign28700_e35453,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign28700_e35455;

        let assign28710_e35458: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard572 = assign28710_e35458;

        let (assign28720_e35472,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard571 == 0.0)) && (var_guard572 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign28720_e35472;

        let (assign28730_e35504,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard571 == 0.0)) && (var_guard572 == 0.0)) {
        let assign28730_e35487: f64 = (var_wsrhstep * var_wsrhstep);
        let assign28730_e35489: f64 = (var_wsrhstep).ln();
        let assign28730_e35490: f64 = (assign28730_e35487 * assign28730_e35489);
        let assign28730_e35493: f64 = (1.0 - var_wsrhstep);
        let assign28730_e35494: f64 = (assign28730_e35490 / assign28730_e35493);
        let assign28730_e35496: f64 = (assign28730_e35494 + var_wsrhstep);
        let assign28730_e35500: f64 = (2.0 * var_pgatd_i);
        let assign28730_e35501: f64 = (1.0 - assign28730_e35500);
        let assign28730_e35502: f64 = (assign28730_e35496 * assign28730_e35501);
        (assign28730_e35502,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign28730_e35504;

        let (assign28740_e35518,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard571 == 0.0)) {
        let assign28740_e35516: f64 = (var_wsrhstep + var_dwsrh);
        (assign28740_e35516,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign28740_e35518;

        let assign28750_e35521: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard573 = assign28750_e35521;

        let (assign28760_e35538, assign28760_e35538_d_n6, assign28760_e35538_d_n7, assign28760_e35538_d_n8, assign28760_e35538_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard571 == 0.0)) && (var_guard573 != 0.0)) {
        let assign28760_e35535: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign28760_e35536: f64 = (assign28760_e35535).sqrt();
        (assign28760_e35536, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign28760_e35538;
        var_tmp_dn6 = assign28760_e35538_d_n6;
        var_tmp_dn7 = assign28760_e35538_d_n7;
        var_tmp_dn8 = assign28760_e35538_d_n8;
        var_tmp_dn9 = assign28760_e35538_d_n9;

        let (assign28770_e35557, assign28770_e35557_d_n6, assign28770_e35557_d_n7, assign28770_e35557_d_n8, assign28770_e35557_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard571 == 0.0)) && (var_guard573 == 0.0)) {
        let assign28770_e35553: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign28770_e35555: f64 = (assign28770_e35553).powf(var_pgatd_i);
        (assign28770_e35555, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign28770_e35557;
        var_tmp_dn6 = assign28770_e35557_d_n6;
        var_tmp_dn7 = assign28770_e35557_d_n7;
        var_tmp_dn8 = assign28770_e35557_d_n8;
        var_tmp_dn9 = assign28770_e35557_d_n9;

        let (assign28780_e35571, assign28780_e35571_d_n6, assign28780_e35571_d_n7, assign28780_e35571_d_n8, assign28780_e35571_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard571 == 0.0)) {
        let assign28780_e35569: f64 = (var_wdepnulrgat_d * var_tmp);
        (assign28780_e35569, (var_wdepnulrgat_d * var_tmp_dn6), (var_wdepnulrgat_d * var_tmp_dn7), (var_wdepnulrgat_d * var_tmp_dn8), (var_wdepnulrgat_d * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign28780_e35571;
        var_wdep_dn6 = assign28780_e35571_d_n6;
        var_wdep_dn7 = assign28780_e35571_d_n7;
        var_wdep_dn8 = assign28780_e35571_d_n8;
        var_wdep_dn9 = assign28780_e35571_d_n9;

        let (assign28790_e35589, assign28790_e35589_d_n6, assign28790_e35589_d_n7, assign28790_e35589_d_n8, assign28790_e35589_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard571 == 0.0)) {
        let assign28790_e35584: f64 = (var_zinv - 1.0);
        let assign28790_e35586: f64 = (assign28790_e35584 * var_wdep);
        let assign28790_e35587: f64 = (var_ftdgat_d * assign28790_e35586);
        (assign28790_e35587, (var_ftdgat_d * (assign28790_e35584 * var_wdep_dn6)), (var_ftdgat_d * (assign28790_e35584 * var_wdep_dn7)), (var_ftdgat_d * (assign28790_e35584 * var_wdep_dn8)), (var_ftdgat_d * (assign28790_e35584 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign28790_e35589;
        var_asrh_dn6 = assign28790_e35589_d_n6;
        var_asrh_dn7 = assign28790_e35589_d_n7;
        var_asrh_dn8 = assign28790_e35589_d_n8;
        var_asrh_dn9 = assign28790_e35589_d_n9;

        let (assign28800_e35605, assign28800_e35605_d_n6, assign28800_e35605_d_n7, assign28800_e35605_d_n8, assign28800_e35605_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard571 == 0.0)) {
        let assign28800_e35602: f64 = (var_asrh * var_wsrh);
        let assign28800_e35603: f64 = (var_csrhgatd_i * assign28800_e35602);
        (assign28800_e35603, (var_csrhgatd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn8 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign28800_e35605;
        var_isrh_dn6 = assign28800_e35605_d_n6;
        var_isrh_dn7 = assign28800_e35605_d_n7;
        var_isrh_dn8 = assign28800_e35605_d_n8;
        var_isrh_dn9 = assign28800_e35605_d_n9;

        let assign28810_e35608: f64 = if var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard574 = assign28810_e35608;

        let (assign28820_e35619, assign28820_e35619_d_n6, assign28820_e35619_d_n7, assign28820_e35619_d_n8, assign28820_e35619_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign28820_e35619;
        var_itat_dn6 = assign28820_e35619_d_n6;
        var_itat_dn7 = assign28820_e35619_d_n7;
        var_itat_dn8 = assign28820_e35619_d_n8;
        var_itat_dn9 = assign28820_e35619_d_n9;

        let (assign28830_e35637, assign28830_e35637_d_n6, assign28830_e35637_d_n7, assign28830_e35637_d_n8, assign28830_e35637_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) {
        let assign28830_e35632: f64 = (var_wdep * var_one_minus_pgat_d);
        let assign28830_e35634: f64 = (assign28830_e35632 / var_vbi_minus_vjsrh);
        let assign28830_e35635: f64 = (var_btatpartgat_d * assign28830_e35634);
        (assign28830_e35635, (var_btatpartgat_d * ((var_wdep_dn6 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn7 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn8 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn9 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign28830_e35637;
        var_btat_dn6 = assign28830_e35637_d_n6;
        var_btat_dn7 = assign28830_e35637_d_n7;
        var_btat_dn8 = assign28830_e35637_d_n8;
        var_btat_dn9 = assign28830_e35637_d_n9;

        let (assign28840_e35653, assign28840_e35653_d_n6, assign28840_e35653_d_n7, assign28840_e35653_d_n8, assign28840_e35653_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) {
        let assign28840_e35649: f64 = (0.666666666666667 * var_atatgat_d);
        let assign28840_e35651: f64 = (assign28840_e35649 / var_btat);
        (assign28840_e35651, (-((assign28840_e35649 * var_btat_dn6) / (var_btat * var_btat))), (-((assign28840_e35649 * var_btat_dn7) / (var_btat * var_btat))), (-((assign28840_e35649 * var_btat_dn8) / (var_btat * var_btat))), (-((assign28840_e35649 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign28840_e35653;
        var_twoatatoverthreebtat_dn6 = assign28840_e35653_d_n6;
        var_twoatatoverthreebtat_dn7 = assign28840_e35653_d_n7;
        var_twoatatoverthreebtat_dn8 = assign28840_e35653_d_n8;
        var_twoatatoverthreebtat_dn9 = assign28840_e35653_d_n9;

        let (assign28850_e35667, assign28850_e35667_d_n6, assign28850_e35667_d_n7, assign28850_e35667_d_n8, assign28850_e35667_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) {
        let assign28850_e35665: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign28850_e35665, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign28850_e35667;
        var_umaxbeforelimiting_dn6 = assign28850_e35667_d_n6;
        var_umaxbeforelimiting_dn7 = assign28850_e35667_d_n7;
        var_umaxbeforelimiting_dn8 = assign28850_e35667_d_n8;
        var_umaxbeforelimiting_dn9 = assign28850_e35667_d_n9;

        let (assign28860_e35688, assign28860_e35688_d_n6, assign28860_e35688_d_n7, assign28860_e35688_d_n8, assign28860_e35688_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) {
        let assign28860_e35679: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign28860_e35682: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign28860_e35684: f64 = (assign28860_e35682 + 1.0);
        let assign28860_e35685: f64 = (assign28860_e35679 / assign28860_e35684);
        let assign28860_e35686: f64 = (assign28860_e35685).sqrt();
        (assign28860_e35686, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign28860_e35684) - (assign28860_e35679 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign28860_e35684 * assign28860_e35684)) / (2.0 * assign28860_e35686)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign28860_e35684) - (assign28860_e35679 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign28860_e35684 * assign28860_e35684)) / (2.0 * assign28860_e35686)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign28860_e35684) - (assign28860_e35679 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign28860_e35684 * assign28860_e35684)) / (2.0 * assign28860_e35686)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign28860_e35684) - (assign28860_e35679 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign28860_e35684 * assign28860_e35684)) / (2.0 * assign28860_e35686)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign28860_e35688;
        var_umax_dn6 = assign28860_e35688_d_n6;
        var_umax_dn7 = assign28860_e35688_d_n7;
        var_umax_dn8 = assign28860_e35688_d_n8;
        var_umax_dn9 = assign28860_e35688_d_n9;

        let (assign28870_e35701, assign28870_e35701_d_n6, assign28870_e35701_d_n7, assign28870_e35701_d_n8, assign28870_e35701_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) {
        let assign28870_e35699: f64 = (var_umax).sqrt();
        (assign28870_e35699, (var_umax_dn6 / (2.0 * assign28870_e35699)), (var_umax_dn7 / (2.0 * assign28870_e35699)), (var_umax_dn8 / (2.0 * assign28870_e35699)), (var_umax_dn9 / (2.0 * assign28870_e35699)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign28870_e35701;
        var_sqrtumax_dn6 = assign28870_e35701_d_n6;
        var_sqrtumax_dn7 = assign28870_e35701_d_n7;
        var_sqrtumax_dn8 = assign28870_e35701_d_n8;
        var_sqrtumax_dn9 = assign28870_e35701_d_n9;

        let (assign28880_e35715, assign28880_e35715_d_n6, assign28880_e35715_d_n7, assign28880_e35715_d_n8, assign28880_e35715_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) {
        let assign28880_e35713: f64 = (var_umax * var_sqrtumax);
        (assign28880_e35713, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign28880_e35715;
        var_umaxpoweronepointfive_dn6 = assign28880_e35715_d_n6;
        var_umaxpoweronepointfive_dn7 = assign28880_e35715_d_n7;
        var_umaxpoweronepointfive_dn8 = assign28880_e35715_d_n8;
        var_umaxpoweronepointfive_dn9 = assign28880_e35715_d_n9;

        let assign28890_e35717: f64 = (-var_pgatd_i);
        let assign28890_e35719: f64 = (assign28890_e35717 * var_one_over_one_minus_pgat_d);
        let assign28890_e35721: f64 = (-1.0);
        let assign28890_e35722: f64 = if assign28890_e35719 == assign28890_e35721 { 1.0 } else { 0.0 };
        var_guard575 = assign28890_e35722;

        let (assign28900_e35742, assign28900_e35742_d_n6, assign28900_e35742_d_n7, assign28900_e35742_d_n8, assign28900_e35742_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) && (var_guard575 != 0.0)) {
        let assign28900_e35738: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign28900_e35739: f64 = (1.0 + assign28900_e35738);
        let assign28900_e35740: f64 = (1.0 / assign28900_e35739);
        (assign28900_e35740, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign28900_e35739 * assign28900_e35739))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign28900_e35739 * assign28900_e35739))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign28900_e35739 * assign28900_e35739))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign28900_e35739 * assign28900_e35739))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign28900_e35742;
        var_wgamma_dn6 = assign28900_e35742_d_n6;
        var_wgamma_dn7 = assign28900_e35742_d_n7;
        var_wgamma_dn8 = assign28900_e35742_d_n8;
        var_wgamma_dn9 = assign28900_e35742_d_n9;

        let (assign28910_e35766, assign28910_e35766_d_n6, assign28910_e35766_d_n7, assign28910_e35766_d_n8, assign28910_e35766_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) && (var_guard575 == 0.0)) {
        let assign28910_e35758: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign28910_e35759: f64 = (1.0 + assign28910_e35758);
        let assign28910_e35761: f64 = (-var_pgatd_i);
        let assign28910_e35763: f64 = (assign28910_e35761 * var_one_over_one_minus_pgat_d);
        let assign28910_e35764: f64 = (assign28910_e35759).powf(assign28910_e35763);
        (assign28910_e35764, if 0.0 == 0.0 && ((assign28910_e35763) as f64).is_finite() && ((assign28910_e35763) as f64).fract() == 0.0 { if assign28910_e35763 == 0.0 { 0.0 } else { (assign28910_e35763 * ((assign28910_e35759).powf(assign28910_e35763 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign28910_e35764 * (assign28910_e35763 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign28910_e35759))) }, if 0.0 == 0.0 && ((assign28910_e35763) as f64).is_finite() && ((assign28910_e35763) as f64).fract() == 0.0 { if assign28910_e35763 == 0.0 { 0.0 } else { (assign28910_e35763 * ((assign28910_e35759).powf(assign28910_e35763 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign28910_e35764 * (assign28910_e35763 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign28910_e35759))) }, if 0.0 == 0.0 && ((assign28910_e35763) as f64).is_finite() && ((assign28910_e35763) as f64).fract() == 0.0 { if assign28910_e35763 == 0.0 { 0.0 } else { (assign28910_e35763 * ((assign28910_e35759).powf(assign28910_e35763 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign28910_e35764 * (assign28910_e35763 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign28910_e35759))) }, if 0.0 == 0.0 && ((assign28910_e35763) as f64).is_finite() && ((assign28910_e35763) as f64).fract() == 0.0 { if assign28910_e35763 == 0.0 { 0.0 } else { (assign28910_e35763 * ((assign28910_e35759).powf(assign28910_e35763 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign28910_e35764 * (assign28910_e35763 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign28910_e35759))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign28910_e35766;
        var_wgamma_dn6 = assign28910_e35766_d_n6;
        var_wgamma_dn7 = assign28910_e35766_d_n7;
        var_wgamma_dn8 = assign28910_e35766_d_n8;
        var_wgamma_dn9 = assign28910_e35766_d_n9;

        let (assign28920_e35784, assign28920_e35784_d_n6, assign28920_e35784_d_n7, assign28920_e35784_d_n8, assign28920_e35784_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) {
        let assign28920_e35778: f64 = (var_wsrh * var_wgamma);
        let assign28920_e35781: f64 = (var_wsrh + var_wgamma);
        let assign28920_e35782: f64 = (assign28920_e35778 / assign28920_e35781);
        (assign28920_e35782, ((((var_wsrh * var_wgamma_dn6) * assign28920_e35781) - (assign28920_e35778 * var_wgamma_dn6)) / (assign28920_e35781 * assign28920_e35781)), ((((var_wsrh * var_wgamma_dn7) * assign28920_e35781) - (assign28920_e35778 * var_wgamma_dn7)) / (assign28920_e35781 * assign28920_e35781)), ((((var_wsrh * var_wgamma_dn8) * assign28920_e35781) - (assign28920_e35778 * var_wgamma_dn8)) / (assign28920_e35781 * assign28920_e35781)), ((((var_wsrh * var_wgamma_dn9) * assign28920_e35781) - (assign28920_e35778 * var_wgamma_dn9)) / (assign28920_e35781 * assign28920_e35781)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign28920_e35784;
        var_wtat_dn6 = assign28920_e35784_d_n6;
        var_wtat_dn7 = assign28920_e35784_d_n7;
        var_wtat_dn8 = assign28920_e35784_d_n8;
        var_wtat_dn9 = assign28920_e35784_d_n9;

        let (assign28930_e35801, assign28930_e35801_d_n6, assign28930_e35801_d_n7, assign28930_e35801_d_n8, assign28930_e35801_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) {
        let assign28930_e35797: f64 = (var_btat / var_sqrtumax);
        let assign28930_e35798: f64 = (0.375 * assign28930_e35797);
        let assign28930_e35799: f64 = (assign28930_e35798).sqrt();
        (assign28930_e35799, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign28930_e35799)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign28930_e35799)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign28930_e35799)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign28930_e35799)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign28930_e35801;
        var_ktat_dn6 = assign28930_e35801_d_n6;
        var_ktat_dn7 = assign28930_e35801_d_n7;
        var_ktat_dn8 = assign28930_e35801_d_n8;
        var_ktat_dn9 = assign28930_e35801_d_n9;

        let (assign28940_e35819, assign28940_e35819_d_n6, assign28940_e35819_d_n7, assign28940_e35819_d_n8, assign28940_e35819_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) {
        let assign28940_e35814: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign28940_e35815: f64 = (2.0 * assign28940_e35814);
        let assign28940_e35817: f64 = (assign28940_e35815 - var_umax);
        (assign28940_e35817, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign28940_e35819;
        var_ltat_dn6 = assign28940_e35819_d_n6;
        var_ltat_dn7 = assign28940_e35819_d_n7;
        var_ltat_dn8 = assign28940_e35819_d_n8;
        var_ltat_dn9 = assign28940_e35819_d_n9;

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
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_sqrtumax_dn9_slot = var_sqrtumax_dn9;
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
    }

    pub(super) fn stamp_transient_block_58(
        var_alphaav: f64,
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
        var_guard570: f64,
        var_guard574: f64,
        var_ktat: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_ktat_dn9: f64,
        var_ltat: f64,
        var_ltat_dn6: f64,
        var_ltat_dn7: f64,
        var_ltat_dn8: f64,
        var_ltat_dn9: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_pbrgatd_i: f64,
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
        var_v1: f64,
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
        var_guard576_slot: &mut f64,
        var_guard577_slot: &mut f64,
        var_guard578_slot: &mut f64,
        var_guard579_slot: &mut f64,
        var_guard580_slot: &mut f64,
        var_guard581_slot: &mut f64,
        var_guard582_slot: &mut f64,
        var_guard583_slot: &mut f64,
        var_guard584_slot: &mut f64,
        var_guard585_slot: &mut f64,
        var_guard586_slot: &mut f64,
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
        let mut var_guard576: f64 = *var_guard576_slot;
        let mut var_guard577: f64 = *var_guard577_slot;
        let mut var_guard578: f64 = *var_guard578_slot;
        let mut var_guard579: f64 = *var_guard579_slot;
        let mut var_guard580: f64 = *var_guard580_slot;
        let mut var_guard581: f64 = *var_guard581_slot;
        let mut var_guard582: f64 = *var_guard582_slot;
        let mut var_guard583: f64 = *var_guard583_slot;
        let mut var_guard584: f64 = *var_guard584_slot;
        let mut var_guard585: f64 = *var_guard585_slot;
        let mut var_guard586: f64 = *var_guard586_slot;
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

        let (assign28950_e35845, assign28950_e35845_d_n6, assign28950_e35845_d_n7, assign28950_e35845_d_n8, assign28950_e35845_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) {
        let assign28950_e35831: f64 = (var_atatgat_d * var_twoatatoverthreebtat);
        let assign28950_e35833: f64 = (assign28950_e35831 * var_sqrtumax);
        let assign28950_e35836: f64 = (var_atatgat_d * var_umax);
        let assign28950_e35837: f64 = (assign28950_e35833 - assign28950_e35836);
        let assign28950_e35841: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign28950_e35842: f64 = (0.5 * assign28950_e35841);
        let assign28950_e35843: f64 = (assign28950_e35837 + assign28950_e35842);
        (assign28950_e35843, (((((var_atatgat_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign28950_e35831 * var_sqrtumax_dn6)) - (var_atatgat_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign28950_e35831 * var_sqrtumax_dn7)) - (var_atatgat_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign28950_e35831 * var_sqrtumax_dn8)) - (var_atatgat_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatgat_d * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign28950_e35831 * var_sqrtumax_dn9)) - (var_atatgat_d * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign28950_e35845;
        var_mtat_dn6 = assign28950_e35845_d_n6;
        var_mtat_dn7 = assign28950_e35845_d_n7;
        var_mtat_dn8 = assign28950_e35845_d_n8;
        var_mtat_dn9 = assign28950_e35845_d_n9;

        let (assign28960_e35861, assign28960_e35861_d_n6, assign28960_e35861_d_n7, assign28960_e35861_d_n8, assign28960_e35861_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) {
        let assign28960_e35857: f64 = (var_ltat - 1.0);
        let assign28960_e35859: f64 = (assign28960_e35857 * var_ktat);
        (assign28960_e35859, ((var_ltat_dn6 * var_ktat) + (assign28960_e35857 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign28960_e35857 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign28960_e35857 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign28960_e35857 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign28960_e35861;
        var_xerfc_dn6 = assign28960_e35861_d_n6;
        var_xerfc_dn7 = assign28960_e35861_d_n7;
        var_xerfc_dn8 = assign28960_e35861_d_n8;
        var_xerfc_dn9 = assign28960_e35861_d_n9;

        let (assign28970_e35875, assign28970_e35875_d_n6, assign28970_e35875_d_n7, assign28970_e35875_d_n8, assign28970_e35875_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) {
        let assign28970_e35873: f64 = (var_xerfc * var_xerfc);
        (assign28970_e35873, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign28970_e35875;
        var_ysq_dn6 = assign28970_e35875_d_n6;
        var_ysq_dn7 = assign28970_e35875_d_n7;
        var_ysq_dn8 = assign28970_e35875_d_n8;
        var_ysq_dn9 = assign28970_e35875_d_n9;

        let assign28980_e35878: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard576 = assign28980_e35878;

        let (assign28990_e35898, assign28990_e35898_d_n6, assign28990_e35898_d_n7, assign28990_e35898_d_n8, assign28990_e35898_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) && (var_guard576 != 0.0)) {
        let assign28990_e35894: f64 = (var_perfc * var_xerfc);
        let assign28990_e35895: f64 = (1.0 + assign28990_e35894);
        let assign28990_e35896: f64 = (1.0 / assign28990_e35895);
        (assign28990_e35896, (-((var_perfc * var_xerfc_dn6) / (assign28990_e35895 * assign28990_e35895))), (-((var_perfc * var_xerfc_dn7) / (assign28990_e35895 * assign28990_e35895))), (-((var_perfc * var_xerfc_dn8) / (assign28990_e35895 * assign28990_e35895))), (-((var_perfc * var_xerfc_dn9) / (assign28990_e35895 * assign28990_e35895))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign28990_e35898;
        var_terfc_dn6 = assign28990_e35898_d_n6;
        var_terfc_dn7 = assign28990_e35898_d_n7;
        var_terfc_dn8 = assign28990_e35898_d_n8;
        var_terfc_dn9 = assign28990_e35898_d_n9;

        let (assign29000_e35919, assign29000_e35919_d_n6, assign29000_e35919_d_n7, assign29000_e35919_d_n8, assign29000_e35919_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) && (var_guard576 == 0.0)) {
        let assign29000_e35915: f64 = (var_perfc * var_xerfc);
        let assign29000_e35916: f64 = (1.0 - assign29000_e35915);
        let assign29000_e35917: f64 = (1.0 / assign29000_e35916);
        (assign29000_e35917, (-((-(var_perfc * var_xerfc_dn6)) / (assign29000_e35916 * assign29000_e35916))), (-((-(var_perfc * var_xerfc_dn7)) / (assign29000_e35916 * assign29000_e35916))), (-((-(var_perfc * var_xerfc_dn8)) / (assign29000_e35916 * assign29000_e35916))), (-((-(var_perfc * var_xerfc_dn9)) / (assign29000_e35916 * assign29000_e35916))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign29000_e35919;
        var_terfc_dn6 = assign29000_e35919_d_n6;
        var_terfc_dn7 = assign29000_e35919_d_n7;
        var_terfc_dn8 = assign29000_e35919_d_n8;
        var_terfc_dn9 = assign29000_e35919_d_n9;

        let assign29010_e35921: f64 = (-var_ysq);
        let assign29010_e35923: f64 = (assign29010_e35921 + var_mtat);
        let assign29010_e35925: f64 = (-230.25850929940458);
        let assign29010_e35926: f64 = if assign29010_e35923 > assign29010_e35925 { 1.0 } else { 0.0 };
        var_guard577 = assign29010_e35926;

        let (assign29020_e35944, assign29020_e35944_d_n6, assign29020_e35944_d_n7, assign29020_e35944_d_n8, assign29020_e35944_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) && (var_guard577 != 0.0)) {
        let assign29020_e35939: f64 = (-var_ysq);
        let assign29020_e35941: f64 = (assign29020_e35939 + var_mtat);
        let assign29020_e35942: f64 = (assign29020_e35941).exp();
        (assign29020_e35942, (assign29020_e35942 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign29020_e35942 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign29020_e35942 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign29020_e35942 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign29020_e35944;
        var_tmp_dn6 = assign29020_e35944_d_n6;
        var_tmp_dn7 = assign29020_e35944_d_n7;
        var_tmp_dn8 = assign29020_e35944_d_n8;
        var_tmp_dn9 = assign29020_e35944_d_n9;

        let (assign29030_e35993, assign29030_e35993_d_n6, assign29030_e35993_d_n7, assign29030_e35993_d_n8, assign29030_e35993_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) && (var_guard577 == 0.0)) {
        let assign29030_e35960: f64 = (-230.25850929940458);
        let assign29030_e35962: f64 = (-var_ysq);
        let assign29030_e35964: f64 = (assign29030_e35962 + var_mtat);
        let assign29030_e35965: f64 = (assign29030_e35960 - assign29030_e35964);
        let assign29030_e35969: f64 = (-230.25850929940458);
        let assign29030_e35971: f64 = (-var_ysq);
        let assign29030_e35973: f64 = (assign29030_e35971 + var_mtat);
        let assign29030_e35974: f64 = (assign29030_e35969 - assign29030_e35973);
        let assign29030_e35977: f64 = (-230.25850929940458);
        let assign29030_e35979: f64 = (-var_ysq);
        let assign29030_e35981: f64 = (assign29030_e35979 + var_mtat);
        let assign29030_e35982: f64 = (assign29030_e35977 - assign29030_e35981);
        let assign29030_e35984: f64 = (assign29030_e35982 * 0.3333333333333333);
        let assign29030_e35985: f64 = (1.0 + assign29030_e35984);
        let assign29030_e35986: f64 = (assign29030_e35974 * assign29030_e35985);
        let assign29030_e35987: f64 = (0.5 * assign29030_e35986);
        let assign29030_e35988: f64 = (1.0 + assign29030_e35987);
        let assign29030_e35989: f64 = (assign29030_e35965 * assign29030_e35988);
        let assign29030_e35990: f64 = (1.0 + assign29030_e35989);
        let assign29030_e35991: f64 = (1e-100 / assign29030_e35990);
        (assign29030_e35991, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign29030_e35988) + (assign29030_e35965 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign29030_e35985) + (assign29030_e35974 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign29030_e35990 * assign29030_e35990))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign29030_e35988) + (assign29030_e35965 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign29030_e35985) + (assign29030_e35974 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign29030_e35990 * assign29030_e35990))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign29030_e35988) + (assign29030_e35965 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign29030_e35985) + (assign29030_e35974 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign29030_e35990 * assign29030_e35990))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign29030_e35988) + (assign29030_e35965 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign29030_e35985) + (assign29030_e35974 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign29030_e35990 * assign29030_e35990))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign29030_e35993;
        var_tmp_dn6 = assign29030_e35993_d_n6;
        var_tmp_dn7 = assign29030_e35993_d_n7;
        var_tmp_dn8 = assign29030_e35993_d_n8;
        var_tmp_dn9 = assign29030_e35993_d_n9;

        let (assign29040_e36023, assign29040_e36023_d_n6, assign29040_e36023_d_n7, assign29040_e36023_d_n8, assign29040_e36023_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) {
        let assign29040_e36005: f64 = (0.29214664 * var_terfc);
        let assign29040_e36009: f64 = (var_terfc * var_terfc);
        let assign29040_e36010: f64 = (var_berfc * assign29040_e36009);
        let assign29040_e36011: f64 = (assign29040_e36005 + assign29040_e36010);
        let assign29040_e36015: f64 = (var_terfc * var_terfc);
        let assign29040_e36017: f64 = (assign29040_e36015 * var_terfc);
        let assign29040_e36018: f64 = (var_cerfc * assign29040_e36017);
        let assign29040_e36019: f64 = (assign29040_e36011 + assign29040_e36018);
        let assign29040_e36021: f64 = (assign29040_e36019 * var_tmp);
        (assign29040_e36021, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign29040_e36015 * var_terfc_dn6)))) * var_tmp) + (assign29040_e36019 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign29040_e36015 * var_terfc_dn7)))) * var_tmp) + (assign29040_e36019 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign29040_e36015 * var_terfc_dn8)))) * var_tmp) + (assign29040_e36019 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign29040_e36015 * var_terfc_dn9)))) * var_tmp) + (assign29040_e36019 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign29040_e36023;
        var_erfcpos_dn6 = assign29040_e36023_d_n6;
        var_erfcpos_dn7 = assign29040_e36023_d_n7;
        var_erfcpos_dn8 = assign29040_e36023_d_n8;
        var_erfcpos_dn9 = assign29040_e36023_d_n9;

        let assign29050_e36026: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard578 = assign29050_e36026;

        let (assign29060_e36040, assign29060_e36040_d_n6, assign29060_e36040_d_n7, assign29060_e36040_d_n8, assign29060_e36040_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) && (var_guard578 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign29060_e36040;
        var_erfctimesexpmtat_dn6 = assign29060_e36040_d_n6;
        var_erfctimesexpmtat_dn7 = assign29060_e36040_d_n7;
        var_erfctimesexpmtat_dn8 = assign29060_e36040_d_n8;
        var_erfctimesexpmtat_dn9 = assign29060_e36040_d_n9;

        let assign29070_e36043: f64 = (-230.25850929940458);
        let assign29070_e36044: f64 = if var_mtat > assign29070_e36043 { 1.0 } else { 0.0 };
        var_guard579 = assign29070_e36044;

        let (assign29080_e36062, assign29080_e36062_d_n6, assign29080_e36062_d_n7, assign29080_e36062_d_n8, assign29080_e36062_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) && (var_guard578 == 0.0)) && (var_guard579 != 0.0)) {
        let assign29080_e36060: f64 = (var_mtat).exp();
        (assign29080_e36060, (assign29080_e36060 * var_mtat_dn6), (assign29080_e36060 * var_mtat_dn7), (assign29080_e36060 * var_mtat_dn8), (assign29080_e36060 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign29080_e36062;
        var_tmp_dn6 = assign29080_e36062_d_n6;
        var_tmp_dn7 = assign29080_e36062_d_n7;
        var_tmp_dn8 = assign29080_e36062_d_n8;
        var_tmp_dn9 = assign29080_e36062_d_n9;

        let (assign29090_e36105, assign29090_e36105_d_n6, assign29090_e36105_d_n7, assign29090_e36105_d_n8, assign29090_e36105_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) && (var_guard578 == 0.0)) && (var_guard579 == 0.0)) {
        let assign29090_e36081: f64 = (-230.25850929940458);
        let assign29090_e36083: f64 = (assign29090_e36081 - var_mtat);
        let assign29090_e36087: f64 = (-230.25850929940458);
        let assign29090_e36089: f64 = (assign29090_e36087 - var_mtat);
        let assign29090_e36092: f64 = (-230.25850929940458);
        let assign29090_e36094: f64 = (assign29090_e36092 - var_mtat);
        let assign29090_e36096: f64 = (assign29090_e36094 * 0.3333333333333333);
        let assign29090_e36097: f64 = (1.0 + assign29090_e36096);
        let assign29090_e36098: f64 = (assign29090_e36089 * assign29090_e36097);
        let assign29090_e36099: f64 = (0.5 * assign29090_e36098);
        let assign29090_e36100: f64 = (1.0 + assign29090_e36099);
        let assign29090_e36101: f64 = (assign29090_e36083 * assign29090_e36100);
        let assign29090_e36102: f64 = (1.0 + assign29090_e36101);
        let assign29090_e36103: f64 = (1e-100 / assign29090_e36102);
        (assign29090_e36103, (-((1e-100 * (((-var_mtat_dn6) * assign29090_e36100) + (assign29090_e36083 * (0.5 * (((-var_mtat_dn6) * assign29090_e36097) + (assign29090_e36089 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign29090_e36102 * assign29090_e36102))), (-((1e-100 * (((-var_mtat_dn7) * assign29090_e36100) + (assign29090_e36083 * (0.5 * (((-var_mtat_dn7) * assign29090_e36097) + (assign29090_e36089 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign29090_e36102 * assign29090_e36102))), (-((1e-100 * (((-var_mtat_dn8) * assign29090_e36100) + (assign29090_e36083 * (0.5 * (((-var_mtat_dn8) * assign29090_e36097) + (assign29090_e36089 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign29090_e36102 * assign29090_e36102))), (-((1e-100 * (((-var_mtat_dn9) * assign29090_e36100) + (assign29090_e36083 * (0.5 * (((-var_mtat_dn9) * assign29090_e36097) + (assign29090_e36089 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign29090_e36102 * assign29090_e36102))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign29090_e36105;
        var_tmp_dn6 = assign29090_e36105_d_n6;
        var_tmp_dn7 = assign29090_e36105_d_n7;
        var_tmp_dn8 = assign29090_e36105_d_n8;
        var_tmp_dn9 = assign29090_e36105_d_n9;

        let (assign29100_e36124, assign29100_e36124_d_n6, assign29100_e36124_d_n7, assign29100_e36124_d_n8, assign29100_e36124_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) && (var_guard578 == 0.0)) {
        let assign29100_e36120: f64 = (2.0 * var_tmp);
        let assign29100_e36122: f64 = (assign29100_e36120 - var_erfcpos);
        (assign29100_e36122, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign29100_e36124;
        var_erfctimesexpmtat_dn6 = assign29100_e36124_d_n6;
        var_erfctimesexpmtat_dn7 = assign29100_e36124_d_n7;
        var_erfctimesexpmtat_dn8 = assign29100_e36124_d_n8;
        var_erfctimesexpmtat_dn9 = assign29100_e36124_d_n9;

        let (assign29110_e36144, assign29110_e36144_d_n6, assign29110_e36144_d_n7, assign29110_e36144_d_n8, assign29110_e36144_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) {
        let assign29110_e36136: f64 = (1.772453850905516 * 0.5);
        let assign29110_e36139: f64 = (var_atatgat_d * var_erfctimesexpmtat);
        let assign29110_e36141: f64 = (assign29110_e36139 / var_ktat);
        let assign29110_e36142: f64 = (assign29110_e36136 * assign29110_e36141);
        (assign29110_e36142, (assign29110_e36136 * ((((var_atatgat_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign29110_e36139 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign29110_e36136 * ((((var_atatgat_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign29110_e36139 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign29110_e36136 * ((((var_atatgat_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign29110_e36139 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign29110_e36136 * ((((var_atatgat_d * var_erfctimesexpmtat_dn9) * var_ktat) - (assign29110_e36139 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign29110_e36144;
        var_gammamax_dn6 = assign29110_e36144_d_n6;
        var_gammamax_dn7 = assign29110_e36144_d_n7;
        var_gammamax_dn8 = assign29110_e36144_d_n8;
        var_gammamax_dn9 = assign29110_e36144_d_n9;

        let (assign29120_e36162, assign29120_e36162_d_n6, assign29120_e36162_d_n7, assign29120_e36162_d_n8, assign29120_e36162_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard574 == 0.0)) {
        let assign29120_e36157: f64 = (var_asrh * var_gammamax);
        let assign29120_e36159: f64 = (assign29120_e36157 * var_wtat);
        let assign29120_e36160: f64 = (var_ctatgatd_i * assign29120_e36159);
        (assign29120_e36160, (var_ctatgatd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign29120_e36157 * var_wtat_dn6))), (var_ctatgatd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign29120_e36157 * var_wtat_dn7))), (var_ctatgatd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign29120_e36157 * var_wtat_dn8))), (var_ctatgatd_i * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign29120_e36157 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign29120_e36162;
        var_itat_dn6 = assign29120_e36162_d_n6;
        var_itat_dn7 = assign29120_e36162_d_n7;
        var_itat_dn8 = assign29120_e36162_d_n8;
        var_itat_dn9 = assign29120_e36162_d_n9;

        let assign29130_e36165: f64 = if var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard580 = assign29130_e36165;

        let (assign29140_e36176, assign29140_e36176_d_n6, assign29140_e36176_d_n7, assign29140_e36176_d_n8, assign29140_e36176_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard580 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign29140_e36176;
        var_ibbt_dn6 = assign29140_e36176_d_n6;
        var_ibbt_dn7 = assign29140_e36176_d_n7;
        var_ibbt_dn8 = assign29140_e36176_d_n8;
        var_ibbt_dn9 = assign29140_e36176_d_n9;

        let assign29150_e36179: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard581 = assign29150_e36179;

        let (assign29160_e36198, assign29160_e36198_d_n6, assign29160_e36198_d_n7, assign29160_e36198_d_n8, assign29160_e36198_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard580 == 0.0)) && (var_guard581 != 0.0)) {
        let assign29160_e36193: f64 = (var_vbirgatd_i - var_vbbt);
        let assign29160_e36195: f64 = (assign29160_e36193 * var_vbirgatinv_d);
        let assign29160_e36196: f64 = (assign29160_e36195).sqrt();
        (assign29160_e36196, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign29160_e36198;
        var_tmp_dn6 = assign29160_e36198_d_n6;
        var_tmp_dn7 = assign29160_e36198_d_n7;
        var_tmp_dn8 = assign29160_e36198_d_n8;
        var_tmp_dn9 = assign29160_e36198_d_n9;

        let (assign29170_e36219, assign29170_e36219_d_n6, assign29170_e36219_d_n7, assign29170_e36219_d_n8, assign29170_e36219_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard580 == 0.0)) && (var_guard581 == 0.0)) {
        let assign29170_e36213: f64 = (var_vbirgatd_i - var_vbbt);
        let assign29170_e36215: f64 = (assign29170_e36213 * var_vbirgatinv_d);
        let assign29170_e36217: f64 = (assign29170_e36215).powf(var_pgatd_i);
        (assign29170_e36217, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign29170_e36219;
        var_tmp_dn6 = assign29170_e36219_d_n6;
        var_tmp_dn7 = assign29170_e36219_d_n7;
        var_tmp_dn8 = assign29170_e36219_d_n8;
        var_tmp_dn9 = assign29170_e36219_d_n9;

        let (assign29180_e36239, assign29180_e36239_d_n6, assign29180_e36239_d_n7, assign29180_e36239_d_n8, assign29180_e36239_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard580 == 0.0)) {
        let assign29180_e36232: f64 = (var_vbirgatd_i - var_vbbt);
        let assign29180_e36234: f64 = (assign29180_e36232 * var_wdepnulrinvgat_d);
        let assign29180_e36236: f64 = (assign29180_e36234 / var_tmp);
        let assign29180_e36237: f64 = (var_one_over_one_minus_pgat_d * assign29180_e36236);
        (assign29180_e36237, (var_one_over_one_minus_pgat_d * (-((assign29180_e36234 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign29180_e36234 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign29180_e36234 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat_d * (-((assign29180_e36234 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign29180_e36239;
        var_fmaxr_dn6 = assign29180_e36239_d_n6;
        var_fmaxr_dn7 = assign29180_e36239_d_n7;
        var_fmaxr_dn8 = assign29180_e36239_d_n8;
        var_fmaxr_dn9 = assign29180_e36239_d_n9;

        let assign29190_e36241: f64 = (-var_fbbtgat_d);
        let assign29190_e36243: f64 = (assign29190_e36241 / var_fmaxr);
        let assign29190_e36244: f64 = (assign29190_e36243).abs();
        let assign29190_e36246: f64 = if assign29190_e36244 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard582 = assign29190_e36246;

        let (assign29200_e36264, assign29200_e36264_d_n6, assign29200_e36264_d_n7, assign29200_e36264_d_n8, assign29200_e36264_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard580 == 0.0)) && (var_guard582 != 0.0)) {
        let assign29200_e36259: f64 = (-var_fbbtgat_d);
        let assign29200_e36261: f64 = (assign29200_e36259 / var_fmaxr);
        let assign29200_e36262: f64 = (assign29200_e36261).exp();
        (assign29200_e36262, (assign29200_e36262 * ((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign29200_e36259 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign29200_e36262 * ((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign29200_e36259 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign29200_e36262 * ((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign29200_e36259 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))), (assign29200_e36262 * ((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign29200_e36259 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign29200_e36264;
        var_tmp_dn6 = assign29200_e36264_d_n6;
        var_tmp_dn7 = assign29200_e36264_d_n7;
        var_tmp_dn8 = assign29200_e36264_d_n8;
        var_tmp_dn9 = assign29200_e36264_d_n9;

        let assign29210_e36266: f64 = (-var_fbbtgat_d);
        let assign29210_e36268: f64 = (assign29210_e36266 / var_fmaxr);
        let assign29210_e36270: f64 = if assign29210_e36268 < 0.0 { 1.0 } else { 0.0 };
        var_guard583 = assign29210_e36270;

        let (assign29220_e36321, assign29220_e36321_d_n6, assign29220_e36321_d_n7, assign29220_e36321_d_n8, assign29220_e36321_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard580 == 0.0)) && (var_guard582 == 0.0)) && (var_guard583 != 0.0)) {
        let assign29220_e36288: f64 = (-230.25850929940458);
        let assign29220_e36290: f64 = (-var_fbbtgat_d);
        let assign29220_e36292: f64 = (assign29220_e36290 / var_fmaxr);
        let assign29220_e36293: f64 = (assign29220_e36288 - assign29220_e36292);
        let assign29220_e36297: f64 = (-230.25850929940458);
        let assign29220_e36299: f64 = (-var_fbbtgat_d);
        let assign29220_e36301: f64 = (assign29220_e36299 / var_fmaxr);
        let assign29220_e36302: f64 = (assign29220_e36297 - assign29220_e36301);
        let assign29220_e36305: f64 = (-230.25850929940458);
        let assign29220_e36307: f64 = (-var_fbbtgat_d);
        let assign29220_e36309: f64 = (assign29220_e36307 / var_fmaxr);
        let assign29220_e36310: f64 = (assign29220_e36305 - assign29220_e36309);
        let assign29220_e36312: f64 = (assign29220_e36310 * 0.3333333333333333);
        let assign29220_e36313: f64 = (1.0 + assign29220_e36312);
        let assign29220_e36314: f64 = (assign29220_e36302 * assign29220_e36313);
        let assign29220_e36315: f64 = (0.5 * assign29220_e36314);
        let assign29220_e36316: f64 = (1.0 + assign29220_e36315);
        let assign29220_e36317: f64 = (assign29220_e36293 * assign29220_e36316);
        let assign29220_e36318: f64 = (1.0 + assign29220_e36317);
        let assign29220_e36319: f64 = (1e-100 / assign29220_e36318);
        (assign29220_e36319, (-((1e-100 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign29220_e36290 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign29220_e36316) + (assign29220_e36293 * (0.5 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign29220_e36299 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign29220_e36313) + (assign29220_e36302 * ((-((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign29220_e36307 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign29220_e36318 * assign29220_e36318))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign29220_e36290 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign29220_e36316) + (assign29220_e36293 * (0.5 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign29220_e36299 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign29220_e36313) + (assign29220_e36302 * ((-((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign29220_e36307 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign29220_e36318 * assign29220_e36318))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign29220_e36290 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign29220_e36316) + (assign29220_e36293 * (0.5 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign29220_e36299 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign29220_e36313) + (assign29220_e36302 * ((-((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign29220_e36307 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign29220_e36318 * assign29220_e36318))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign29220_e36290 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign29220_e36316) + (assign29220_e36293 * (0.5 * (((-((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign29220_e36299 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign29220_e36313) + (assign29220_e36302 * ((-((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign29220_e36307 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign29220_e36318 * assign29220_e36318))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign29220_e36321;
        var_tmp_dn6 = assign29220_e36321_d_n6;
        var_tmp_dn7 = assign29220_e36321_d_n7;
        var_tmp_dn8 = assign29220_e36321_d_n8;
        var_tmp_dn9 = assign29220_e36321_d_n9;

        let (assign29230_e36370, assign29230_e36370_d_n6, assign29230_e36370_d_n7, assign29230_e36370_d_n8, assign29230_e36370_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard580 == 0.0)) && (var_guard582 == 0.0)) && (var_guard583 == 0.0)) {
        let assign29230_e36340: f64 = (-var_fbbtgat_d);
        let assign29230_e36342: f64 = (assign29230_e36340 / var_fmaxr);
        let assign29230_e36344: f64 = (assign29230_e36342 - 230.25850929940458);
        let assign29230_e36348: f64 = (-var_fbbtgat_d);
        let assign29230_e36350: f64 = (assign29230_e36348 / var_fmaxr);
        let assign29230_e36352: f64 = (assign29230_e36350 - 230.25850929940458);
        let assign29230_e36355: f64 = (-var_fbbtgat_d);
        let assign29230_e36357: f64 = (assign29230_e36355 / var_fmaxr);
        let assign29230_e36359: f64 = (assign29230_e36357 - 230.25850929940458);
        let assign29230_e36361: f64 = (assign29230_e36359 * 0.3333333333333333);
        let assign29230_e36362: f64 = (1.0 + assign29230_e36361);
        let assign29230_e36363: f64 = (assign29230_e36352 * assign29230_e36362);
        let assign29230_e36364: f64 = (0.5 * assign29230_e36363);
        let assign29230_e36365: f64 = (1.0 + assign29230_e36364);
        let assign29230_e36366: f64 = (assign29230_e36344 * assign29230_e36365);
        let assign29230_e36367: f64 = (1.0 + assign29230_e36366);
        let assign29230_e36368: f64 = (1e100 * assign29230_e36367);
        (assign29230_e36368, (1e100 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign29230_e36340 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign29230_e36365) + (assign29230_e36344 * (0.5 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign29230_e36348 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign29230_e36362) + (assign29230_e36352 * (((((-var_fbbtgat_d_dn6) * var_fmaxr) - (assign29230_e36355 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign29230_e36340 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign29230_e36365) + (assign29230_e36344 * (0.5 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign29230_e36348 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign29230_e36362) + (assign29230_e36352 * (((((-var_fbbtgat_d_dn7) * var_fmaxr) - (assign29230_e36355 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign29230_e36340 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign29230_e36365) + (assign29230_e36344 * (0.5 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign29230_e36348 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign29230_e36362) + (assign29230_e36352 * (((((-var_fbbtgat_d_dn8) * var_fmaxr) - (assign29230_e36355 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign29230_e36340 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign29230_e36365) + (assign29230_e36344 * (0.5 * ((((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign29230_e36348 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign29230_e36362) + (assign29230_e36352 * (((((-var_fbbtgat_d_dn9) * var_fmaxr) - (assign29230_e36355 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign29230_e36370;
        var_tmp_dn6 = assign29230_e36370_d_n6;
        var_tmp_dn7 = assign29230_e36370_d_n7;
        var_tmp_dn8 = assign29230_e36370_d_n8;
        var_tmp_dn9 = assign29230_e36370_d_n9;

        let (assign29240_e36390, assign29240_e36390_d_n6, assign29240_e36390_d_n7, assign29240_e36390_d_n8, assign29240_e36390_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard580 == 0.0)) {
        let assign29240_e36383: f64 = (var_v1 * var_fmaxr);
        let assign29240_e36385: f64 = (assign29240_e36383 * var_fmaxr);
        let assign29240_e36387: f64 = (assign29240_e36385 * var_tmp);
        let assign29240_e36388: f64 = (var_cbbtgatd_i * assign29240_e36387);
        (assign29240_e36388, (var_cbbtgatd_i * (((((var_v1 * var_fmaxr_dn6) * var_fmaxr) + (assign29240_e36383 * var_fmaxr_dn6)) * var_tmp) + (assign29240_e36385 * var_tmp_dn6))), (var_cbbtgatd_i * (((((var_v1 * var_fmaxr_dn7) * var_fmaxr) + (assign29240_e36383 * var_fmaxr_dn7)) * var_tmp) + (assign29240_e36385 * var_tmp_dn7))), (var_cbbtgatd_i * (((((var_v1 * var_fmaxr_dn8) * var_fmaxr) + (assign29240_e36383 * var_fmaxr_dn8)) * var_tmp) + (assign29240_e36385 * var_tmp_dn8))), (var_cbbtgatd_i * (((((var_v1 * var_fmaxr_dn9) * var_fmaxr) + (assign29240_e36383 * var_fmaxr_dn9)) * var_tmp) + (assign29240_e36385 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign29240_e36390;
        var_ibbt_dn6 = assign29240_e36390_d_n6;
        var_ibbt_dn7 = assign29240_e36390_d_n7;
        var_ibbt_dn8 = assign29240_e36390_d_n8;
        var_ibbt_dn9 = assign29240_e36390_d_n9;

        let assign29250_e36393: f64 = if var_vbrgatd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard584 = assign29250_e36393;

        let (assign29260_e36404, assign29260_e36404_d_n6, assign29260_e36404_d_n7, assign29260_e36404_d_n8, assign29260_e36404_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard584 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign29260_e36404;
        var_fbreakdown_dn6 = assign29260_e36404_d_n6;
        var_fbreakdown_dn7 = assign29260_e36404_d_n7;
        var_fbreakdown_dn8 = assign29260_e36404_d_n8;
        var_fbreakdown_dn9 = assign29260_e36404_d_n9;

        let assign29270_e36407: f64 = (-var_alphaav);
        let assign29270_e36409: f64 = (assign29270_e36407 * var_vbrgatd_i);
        let assign29270_e36410: f64 = if var_vav > assign29270_e36409 { 1.0 } else { 0.0 };
        var_guard585 = assign29270_e36410;

        let assign29280_e36413: f64 = if var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard586 = assign29280_e36413;

        let (assign29290_e36443, assign29290_e36443_d_n6, assign29290_e36443_d_n7, assign29290_e36443_d_n8, assign29290_e36443_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard584 == 0.0)) && (var_guard585 != 0.0)) && (var_guard586 != 0.0)) {
        let assign29290_e36429: f64 = (var_vav * var_vbrinvgat_d);
        let assign29290_e36432: f64 = (var_vav * var_vbrinvgat_d);
        let assign29290_e36433: f64 = (assign29290_e36429 * assign29290_e36432);
        let assign29290_e36436: f64 = (var_vav * var_vbrinvgat_d);
        let assign29290_e36437: f64 = (assign29290_e36433 * assign29290_e36436);
        let assign29290_e36440: f64 = (var_vav * var_vbrinvgat_d);
        let assign29290_e36441: f64 = (assign29290_e36437 * assign29290_e36440);
        (assign29290_e36441, (((((((var_vav * var_vbrinvgat_d_dn6) * assign29290_e36432) + (assign29290_e36429 * (var_vav * var_vbrinvgat_d_dn6))) * assign29290_e36436) + (assign29290_e36433 * (var_vav * var_vbrinvgat_d_dn6))) * assign29290_e36440) + (assign29290_e36437 * (var_vav * var_vbrinvgat_d_dn6))), (((((((var_vav * var_vbrinvgat_d_dn7) * assign29290_e36432) + (assign29290_e36429 * (var_vav * var_vbrinvgat_d_dn7))) * assign29290_e36436) + (assign29290_e36433 * (var_vav * var_vbrinvgat_d_dn7))) * assign29290_e36440) + (assign29290_e36437 * (var_vav * var_vbrinvgat_d_dn7))), (((((((var_vav * var_vbrinvgat_d_dn8) * assign29290_e36432) + (assign29290_e36429 * (var_vav * var_vbrinvgat_d_dn8))) * assign29290_e36436) + (assign29290_e36433 * (var_vav * var_vbrinvgat_d_dn8))) * assign29290_e36440) + (assign29290_e36437 * (var_vav * var_vbrinvgat_d_dn8))), (((((((var_vav * var_vbrinvgat_d_dn9) * assign29290_e36432) + (assign29290_e36429 * (var_vav * var_vbrinvgat_d_dn9))) * assign29290_e36436) + (assign29290_e36433 * (var_vav * var_vbrinvgat_d_dn9))) * assign29290_e36440) + (assign29290_e36437 * (var_vav * var_vbrinvgat_d_dn9))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign29290_e36443;
        var_tmp_dn6 = assign29290_e36443_d_n6;
        var_tmp_dn7 = assign29290_e36443_d_n7;
        var_tmp_dn8 = assign29290_e36443_d_n8;
        var_tmp_dn9 = assign29290_e36443_d_n9;

        let (assign29300_e36465, assign29300_e36465_d_n6, assign29300_e36465_d_n7, assign29300_e36465_d_n8, assign29300_e36465_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard584 == 0.0)) && (var_guard585 != 0.0)) && (var_guard586 == 0.0)) {
        let assign29300_e36460: f64 = (var_vav * var_vbrinvgat_d);
        let assign29300_e36461: f64 = (assign29300_e36460).abs();
        let assign29300_e36463: f64 = (assign29300_e36461).powf(var_pbrgatd_i);
        (assign29300_e36463, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign29300_e36461).powf(var_pbrgatd_i - 1.0) * if assign29300_e36460 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) })) } } else { (assign29300_e36463 * (var_pbrgatd_i * (if assign29300_e36460 >= 0.0 { (var_vav * var_vbrinvgat_d_dn6) } else { (-(var_vav * var_vbrinvgat_d_dn6)) } / assign29300_e36461))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign29300_e36461).powf(var_pbrgatd_i - 1.0) * if assign29300_e36460 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) })) } } else { (assign29300_e36463 * (var_pbrgatd_i * (if assign29300_e36460 >= 0.0 { (var_vav * var_vbrinvgat_d_dn7) } else { (-(var_vav * var_vbrinvgat_d_dn7)) } / assign29300_e36461))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign29300_e36461).powf(var_pbrgatd_i - 1.0) * if assign29300_e36460 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) })) } } else { (assign29300_e36463 * (var_pbrgatd_i * (if assign29300_e36460 >= 0.0 { (var_vav * var_vbrinvgat_d_dn8) } else { (-(var_vav * var_vbrinvgat_d_dn8)) } / assign29300_e36461))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign29300_e36461).powf(var_pbrgatd_i - 1.0) * if assign29300_e36460 >= 0.0 { (var_vav * var_vbrinvgat_d_dn9) } else { (-(var_vav * var_vbrinvgat_d_dn9)) })) } } else { (assign29300_e36463 * (var_pbrgatd_i * (if assign29300_e36460 >= 0.0 { (var_vav * var_vbrinvgat_d_dn9) } else { (-(var_vav * var_vbrinvgat_d_dn9)) } / assign29300_e36461))) },)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign29300_e36465;
        var_tmp_dn6 = assign29300_e36465_d_n6;
        var_tmp_dn7 = assign29300_e36465_d_n7;
        var_tmp_dn8 = assign29300_e36465_d_n8;
        var_tmp_dn9 = assign29300_e36465_d_n9;

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
        *var_guard576_slot = var_guard576;
        *var_guard577_slot = var_guard577;
        *var_guard578_slot = var_guard578;
        *var_guard579_slot = var_guard579;
        *var_guard580_slot = var_guard580;
        *var_guard581_slot = var_guard581;
        *var_guard582_slot = var_guard582;
        *var_guard583_slot = var_guard583;
        *var_guard584_slot = var_guard584;
        *var_guard585_slot = var_guard585;
        *var_guard586_slot = var_guard586;
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

    pub(super) fn stamp_transient_block_59(
        p: &Parameters,
        var_abdrain_i: f64,
        var_alphaav: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_exp_vmax_over_phitd_d: f64,
        var_fstopgat_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard570: f64,
        var_guard584: f64,
        var_guard585: f64,
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
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_slopegat_d: f64,
        var_slopegat_d_dn6: f64,
        var_slopegat_d_dn7: f64,
        var_slopegat_d_dn8: f64,
        var_slopegat_d_dn9: f64,
        var_v2: f64,
        var_vbbtlim_d: f64,
        var_vbibot_d: f64,
        var_vbimin_d: f64,
        var_vbirbotinv_d: f64,
        var_vbrgatd_i: f64,
        var_vmax_d: f64,
        var_wdepnulrbot_d: f64,
        var_dwsrh_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_guard587_slot: &mut f64,
        var_guard588_slot: &mut f64,
        var_guard589_slot: &mut f64,
        var_guard590_slot: &mut f64,
        var_guard591_slot: &mut f64,
        var_guard592_slot: &mut f64,
        var_guard593_slot: &mut f64,
        var_guard594_slot: &mut f64,
        var_guard595_slot: &mut f64,
        var_i1_slot: &mut f64,
        var_i1_dn6_slot: &mut f64,
        var_i1_dn7_slot: &mut f64,
        var_i1_dn8_slot: &mut f64,
        var_i1_dn9_slot: &mut f64,
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
        var_wdep_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wdep_dn9_slot: &mut f64,
        var_wsrh_slot: &mut f64,
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
        let mut var_guard587: f64 = *var_guard587_slot;
        let mut var_guard588: f64 = *var_guard588_slot;
        let mut var_guard589: f64 = *var_guard589_slot;
        let mut var_guard590: f64 = *var_guard590_slot;
        let mut var_guard591: f64 = *var_guard591_slot;
        let mut var_guard592: f64 = *var_guard592_slot;
        let mut var_guard593: f64 = *var_guard593_slot;
        let mut var_guard594: f64 = *var_guard594_slot;
        let mut var_guard595: f64 = *var_guard595_slot;
        let mut var_i1: f64 = *var_i1_slot;
        let mut var_i1_dn6: f64 = *var_i1_dn6_slot;
        let mut var_i1_dn7: f64 = *var_i1_dn7_slot;
        let mut var_i1_dn8: f64 = *var_i1_dn8_slot;
        let mut var_i1_dn9: f64 = *var_i1_dn9_slot;
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
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wdep_dn9: f64 = *var_wdep_dn9_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign29310_e36483, assign29310_e36483_d_n6, assign29310_e36483_d_n7, assign29310_e36483_d_n8, assign29310_e36483_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard584 == 0.0)) && (var_guard585 != 0.0)) {
        let assign29310_e36480: f64 = (1.0 - var_tmp);
        let assign29310_e36481: f64 = (1.0 / assign29310_e36480);
        (assign29310_e36481, (-((-var_tmp_dn6) / (assign29310_e36480 * assign29310_e36480))), (-((-var_tmp_dn7) / (assign29310_e36480 * assign29310_e36480))), (-((-var_tmp_dn8) / (assign29310_e36480 * assign29310_e36480))), (-((-var_tmp_dn9) / (assign29310_e36480 * assign29310_e36480))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign29310_e36483;
        var_fbreakdown_dn6 = assign29310_e36483_d_n6;
        var_fbreakdown_dn7 = assign29310_e36483_d_n7;
        var_fbreakdown_dn8 = assign29310_e36483_d_n8;
        var_fbreakdown_dn9 = assign29310_e36483_d_n9;

        let (assign29320_e36506, assign29320_e36506_d_n6, assign29320_e36506_d_n7, assign29320_e36506_d_n8, assign29320_e36506_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) && (var_guard584 == 0.0)) && (var_guard585 == 0.0)) {
        let assign29320_e36500: f64 = (var_alphaav * var_vbrgatd_i);
        let assign29320_e36501: f64 = (var_vav + assign29320_e36500);
        let assign29320_e36503: f64 = (assign29320_e36501 * var_slopegat_d);
        let assign29320_e36504: f64 = (var_fstopgat_d + assign29320_e36503);
        (assign29320_e36504, (assign29320_e36501 * var_slopegat_d_dn6), (assign29320_e36501 * var_slopegat_d_dn7), (assign29320_e36501 * var_slopegat_d_dn8), (assign29320_e36501 * var_slopegat_d_dn9),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign29320_e36506;
        var_fbreakdown_dn6 = assign29320_e36506_d_n6;
        var_fbreakdown_dn7 = assign29320_e36506_d_n7;
        var_fbreakdown_dn8 = assign29320_e36506_d_n8;
        var_fbreakdown_dn9 = assign29320_e36506_d_n9;

        let (assign29330_e36525, assign29330_e36525_d_n6, assign29330_e36525_d_n7, assign29330_e36525_d_n8, assign29330_e36525_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard570 == 0.0)) {
        let assign29330_e36516: f64 = (var_id__blk212 + var_isrh);
        let assign29330_e36518: f64 = (assign29330_e36516 + var_itat);
        let assign29330_e36520: f64 = (assign29330_e36518 + var_ibbt);
        let assign29330_e36521: f64 = (p.p29 * assign29330_e36520);
        let assign29330_e36523: f64 = (assign29330_e36521 * var_fbreakdown);
        (assign29330_e36523, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign29330_e36521 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign29330_e36521 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign29330_e36521 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign29330_e36521 * var_fbreakdown_dn9)),)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign29330_e36525;
        var_ijungat_dn6 = assign29330_e36525_d_n6;
        var_ijungat_dn7 = assign29330_e36525_d_n7;
        var_ijungat_dn8 = assign29330_e36525_d_n8;
        var_ijungat_dn9 = assign29330_e36525_d_n9;

        let (assign29340_e36541, assign29340_e36541_d_n6, assign29340_e36541_d_n7, assign29340_e36541_d_n8, assign29340_e36541_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign29340_e36531: f64 = (var_abdrain_i * var_ijunbot);
        let assign29340_e36534: f64 = (var_lsdrain_i * var_ijunsti);
        let assign29340_e36535: f64 = (assign29340_e36531 + assign29340_e36534);
        let assign29340_e36538: f64 = (var_lgdrain_i * var_ijungat);
        let assign29340_e36539: f64 = (assign29340_e36535 + assign29340_e36538);
        (assign29340_e36539, (((var_abdrain_i * var_ijunbot_dn6) + (var_lsdrain_i * var_ijunsti_dn6)) + (var_lgdrain_i * var_ijungat_dn6)), (((var_abdrain_i * var_ijunbot_dn7) + (var_lsdrain_i * var_ijunsti_dn7)) + (var_lgdrain_i * var_ijungat_dn7)), (((var_abdrain_i * var_ijunbot_dn8) + (var_lsdrain_i * var_ijunsti_dn8)) + (var_lgdrain_i * var_ijungat_dn8)), (((var_abdrain_i * var_ijunbot_dn9) + (var_lsdrain_i * var_ijunsti_dn9)) + (var_lgdrain_i * var_ijungat_dn9)),)
    } else {
        (var_i1, var_i1_dn6, var_i1_dn7, var_i1_dn8, var_i1_dn9,)
    }
};
        var_i1 = assign29340_e36541;
        var_i1_dn6 = assign29340_e36541_d_n6;
        var_i1_dn7 = assign29340_e36541_d_n7;
        var_i1_dn8 = assign29340_e36541_d_n8;
        var_i1_dn9 = assign29340_e36541_d_n9;

        let (assign29350_e36547,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign29350_e36547;

        let (assign29360_e36553,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign29360_e36553;

        let assign29370_e36565: f64 = if (!(((var_abdrain_i == 0.0) && (var_lsdrain_i == 0.0)) && (var_lgdrain_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard587 = assign29370_e36565;

        let assign29450_e36651: f64 = if var_v2 < var_vmax_d { 1.0 } else { 0.0 };
        var_guard588 = assign29450_e36651;

        let assign29460_e36653: f64 = (-0.5);
        let assign29460_e36656: f64 = (var_v2 * var_phitdinv);
        let assign29460_e36657: f64 = (assign29460_e36653 * assign29460_e36656);
        let assign29460_e36658: f64 = (assign29460_e36657).abs();
        let assign29460_e36660: f64 = if assign29460_e36658 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard589 = assign29460_e36660;

        let (assign29470_e36678,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard587 != 0.0)) && (var_guard588 != 0.0)) && (var_guard589 != 0.0)) {
        let assign29470_e36671: f64 = (-0.5);
        let assign29470_e36674: f64 = (var_v2 * var_phitdinv);
        let assign29470_e36675: f64 = (assign29470_e36671 * assign29470_e36674);
        let assign29470_e36676: f64 = (assign29470_e36675).exp();
        (assign29470_e36676,)
    } else {
        (var_z,)
    }
};
        var_z = assign29470_e36678;

        let assign29480_e36680: f64 = (-0.5);
        let assign29480_e36683: f64 = (var_v2 * var_phitdinv);
        let assign29480_e36684: f64 = (assign29480_e36680 * assign29480_e36683);
        let assign29480_e36686: f64 = if assign29480_e36684 < 0.0 { 1.0 } else { 0.0 };
        var_guard590 = assign29480_e36686;

        let (assign29490_e36741,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard587 != 0.0)) && (var_guard588 != 0.0)) && (var_guard589 == 0.0)) && (var_guard590 != 0.0)) {
        let assign29490_e36702: f64 = (-230.25850929940458);
        let assign29490_e36704: f64 = (-0.5);
        let assign29490_e36707: f64 = (var_v2 * var_phitdinv);
        let assign29490_e36708: f64 = (assign29490_e36704 * assign29490_e36707);
        let assign29490_e36709: f64 = (assign29490_e36702 - assign29490_e36708);
        let assign29490_e36713: f64 = (-230.25850929940458);
        let assign29490_e36715: f64 = (-0.5);
        let assign29490_e36718: f64 = (var_v2 * var_phitdinv);
        let assign29490_e36719: f64 = (assign29490_e36715 * assign29490_e36718);
        let assign29490_e36720: f64 = (assign29490_e36713 - assign29490_e36719);
        let assign29490_e36723: f64 = (-230.25850929940458);
        let assign29490_e36725: f64 = (-0.5);
        let assign29490_e36728: f64 = (var_v2 * var_phitdinv);
        let assign29490_e36729: f64 = (assign29490_e36725 * assign29490_e36728);
        let assign29490_e36730: f64 = (assign29490_e36723 - assign29490_e36729);
        let assign29490_e36732: f64 = (assign29490_e36730 * 0.3333333333333333);
        let assign29490_e36733: f64 = (1.0 + assign29490_e36732);
        let assign29490_e36734: f64 = (assign29490_e36720 * assign29490_e36733);
        let assign29490_e36735: f64 = (0.5 * assign29490_e36734);
        let assign29490_e36736: f64 = (1.0 + assign29490_e36735);
        let assign29490_e36737: f64 = (assign29490_e36709 * assign29490_e36736);
        let assign29490_e36738: f64 = (1.0 + assign29490_e36737);
        let assign29490_e36739: f64 = (1e-100 / assign29490_e36738);
        (assign29490_e36739,)
    } else {
        (var_z,)
    }
};
        var_z = assign29490_e36741;

        let (assign29500_e36794,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard587 != 0.0)) && (var_guard588 != 0.0)) && (var_guard589 == 0.0)) && (var_guard590 == 0.0)) {
        let assign29500_e36758: f64 = (-0.5);
        let assign29500_e36761: f64 = (var_v2 * var_phitdinv);
        let assign29500_e36762: f64 = (assign29500_e36758 * assign29500_e36761);
        let assign29500_e36764: f64 = (assign29500_e36762 - 230.25850929940458);
        let assign29500_e36768: f64 = (-0.5);
        let assign29500_e36771: f64 = (var_v2 * var_phitdinv);
        let assign29500_e36772: f64 = (assign29500_e36768 * assign29500_e36771);
        let assign29500_e36774: f64 = (assign29500_e36772 - 230.25850929940458);
        let assign29500_e36777: f64 = (-0.5);
        let assign29500_e36780: f64 = (var_v2 * var_phitdinv);
        let assign29500_e36781: f64 = (assign29500_e36777 * assign29500_e36780);
        let assign29500_e36783: f64 = (assign29500_e36781 - 230.25850929940458);
        let assign29500_e36785: f64 = (assign29500_e36783 * 0.3333333333333333);
        let assign29500_e36786: f64 = (1.0 + assign29500_e36785);
        let assign29500_e36787: f64 = (assign29500_e36774 * assign29500_e36786);
        let assign29500_e36788: f64 = (0.5 * assign29500_e36787);
        let assign29500_e36789: f64 = (1.0 + assign29500_e36788);
        let assign29500_e36790: f64 = (assign29500_e36764 * assign29500_e36789);
        let assign29500_e36791: f64 = (1.0 + assign29500_e36790);
        let assign29500_e36792: f64 = (1e100 * assign29500_e36791);
        (assign29500_e36792,)
    } else {
        (var_z,)
    }
};
        var_z = assign29500_e36794;

        let (assign29510_e36806,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard587 != 0.0)) && (var_guard588 != 0.0)) {
        let assign29510_e36804: f64 = (1.0 / var_z);
        (assign29510_e36804,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign29510_e36806;

        let (assign29520_e36818,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard587 != 0.0)) && (var_guard588 != 0.0)) {
        let assign29520_e36816: f64 = (var_zinv * var_zinv);
        (assign29520_e36816,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign29520_e36818;

        let (assign29530_e36837,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard587 != 0.0)) && (var_guard588 == 0.0)) {
        let assign29530_e36830: f64 = (var_v2 - var_vmax_d);
        let assign29530_e36832: f64 = (assign29530_e36830 * var_phitdinv);
        let assign29530_e36833: f64 = (1.0 + assign29530_e36832);
        let assign29530_e36835: f64 = (assign29530_e36833 * var_exp_vmax_over_phitd_d);
        (assign29530_e36835,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign29530_e36837;

        let (assign29540_e36849,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard587 != 0.0)) && (var_guard588 == 0.0)) {
        let assign29540_e36847: f64 = (var_idmult).sqrt();
        (assign29540_e36847,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign29540_e36849;

        let (assign29550_e36862,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard587 != 0.0)) && (var_guard588 == 0.0)) {
        let assign29550_e36860: f64 = (1.0 / var_zinv);
        (assign29550_e36860,)
    } else {
        (var_z,)
    }
};
        var_z = assign29550_e36862;

        let (assign29560_e36872,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard587 != 0.0)) {
        let assign29560_e36870: f64 = (var_idmult - 1.0);
        (assign29560_e36870,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign29560_e36872;

        let assign29570_e36875: f64 = if var_v2 > 0.0 { 1.0 } else { 0.0 };
        var_guard591 = assign29570_e36875;

        let (assign29580_e36901,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard587 != 0.0)) && (var_guard591 != 0.0)) {
        let assign29580_e36887: f64 = (2.0 + var_z);
        let assign29580_e36890: f64 = (var_z + 1.0);
        let assign29580_e36893: f64 = (var_z + 3.0);
        let assign29580_e36894: f64 = (assign29580_e36890 * assign29580_e36893);
        let assign29580_e36895: f64 = (assign29580_e36894).sqrt();
        let assign29580_e36896: f64 = (assign29580_e36887 + assign29580_e36895);
        let assign29580_e36897: f64 = (assign29580_e36896).ln();
        let assign29580_e36898: f64 = (var_phitd * assign29580_e36897);
        let assign29580_e36899: f64 = (2.0 * assign29580_e36898);
        (assign29580_e36899,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign29580_e36901;

        let (assign29590_e36935,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard587 != 0.0)) && (var_guard591 == 0.0)) {
        let assign29590_e36911: f64 = (-var_v2);
        let assign29590_e36916: f64 = (2.0 * var_zinv);
        let assign29590_e36918: f64 = (assign29590_e36916 + 1.0);
        let assign29590_e36921: f64 = (1.0 + var_zinv);
        let assign29590_e36925: f64 = (3.0 * var_zinv);
        let assign29590_e36926: f64 = (1.0 + assign29590_e36925);
        let assign29590_e36927: f64 = (assign29590_e36921 * assign29590_e36926);
        let assign29590_e36928: f64 = (assign29590_e36927).sqrt();
        let assign29590_e36929: f64 = (assign29590_e36918 + assign29590_e36928);
        let assign29590_e36930: f64 = (assign29590_e36929).ln();
        let assign29590_e36931: f64 = (var_phitd * assign29590_e36930);
        let assign29590_e36932: f64 = (2.0 * assign29590_e36931);
        let assign29590_e36933: f64 = (assign29590_e36911 + assign29590_e36932);
        (assign29590_e36933,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign29590_e36935;

        let (assign29600_e36945,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard587 != 0.0)) {
        let assign29600_e36943: f64 = (var_vbimin_d - var_two_psistar);
        (assign29600_e36943,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign29600_e36945;

        let (assign29610_e36972,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard587 != 0.0)) {
        let assign29610_e36954: f64 = (var_v2 + var_vjlim);
        let assign29610_e36957: f64 = (var_v2 - var_vjlim);
        let assign29610_e36960: f64 = (var_v2 - var_vjlim);
        let assign29610_e36961: f64 = (assign29610_e36957 * assign29610_e36960);
        let assign29610_e36964: f64 = (4.0 * var_phitd);
        let assign29610_e36966: f64 = (assign29610_e36964 * var_phitd);
        let assign29610_e36967: f64 = (assign29610_e36961 + assign29610_e36966);
        let assign29610_e36968: f64 = (assign29610_e36967).sqrt();
        let assign29610_e36969: f64 = (assign29610_e36954 - assign29610_e36968);
        let assign29610_e36970: f64 = (0.5 * assign29610_e36969);
        (assign29610_e36970,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign29610_e36972;

        let (assign29620_e36999,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard587 != 0.0)) {
        let assign29620_e36981: f64 = (var_v2 + var_vbbtlim_d);
        let assign29620_e36984: f64 = (var_v2 - var_vbbtlim_d);
        let assign29620_e36987: f64 = (var_v2 - var_vbbtlim_d);
        let assign29620_e36988: f64 = (assign29620_e36984 * assign29620_e36987);
        let assign29620_e36991: f64 = (4.0 * var_phitr);
        let assign29620_e36993: f64 = (assign29620_e36991 * var_phitr);
        let assign29620_e36994: f64 = (assign29620_e36988 + assign29620_e36993);
        let assign29620_e36995: f64 = (assign29620_e36994).sqrt();
        let assign29620_e36996: f64 = (assign29620_e36981 - assign29620_e36995);
        let assign29620_e36997: f64 = (0.5 * assign29620_e36996);
        (assign29620_e36997,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign29620_e36999;

        let (assign29630_e37026,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard587 != 0.0)) {
        let assign29630_e37008: f64 = var_v2;
        let assign29630_e37011: f64 = var_v2;
        let assign29630_e37014: f64 = var_v2;
        let assign29630_e37015: f64 = (assign29630_e37011 * assign29630_e37014);
        let assign29630_e37018: f64 = (4.0 * 1e-6);
        let assign29630_e37020: f64 = (assign29630_e37018 * 1e-6);
        let assign29630_e37021: f64 = (assign29630_e37015 + assign29630_e37020);
        let assign29630_e37022: f64 = (assign29630_e37021).sqrt();
        let assign29630_e37023: f64 = (assign29630_e37008 - assign29630_e37022);
        let assign29630_e37024: f64 = (0.5 * assign29630_e37023);
        (assign29630_e37024,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign29630_e37026;

        let assign29640_e37029: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard592 = assign29640_e37029;

        let (assign29650_e37037, assign29650_e37037_d_n6, assign29650_e37037_d_n7, assign29650_e37037_d_n8, assign29650_e37037_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign29650_e37037;
        var_ijunbot_dn6 = assign29650_e37037_d_n6;
        var_ijunbot_dn7 = assign29650_e37037_d_n7;
        var_ijunbot_dn8 = assign29650_e37037_d_n8;
        var_ijunbot_dn9 = assign29650_e37037_d_n9;

        let (assign29660_e37048,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) {
        let assign29660_e37046: f64 = (var_idsatbot_d * var_idmult);
        (assign29660_e37046,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign29660_e37048;

        let assign29670_e37055: f64 = if ((var_csrhbotd_i == 0.0) && (var_ctatbotd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard593 = assign29670_e37055;

        let (assign29680_e37066, assign29680_e37066_d_n6, assign29680_e37066_d_n7, assign29680_e37066_d_n8, assign29680_e37066_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard593 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign29680_e37066;
        var_isrh_dn6 = assign29680_e37066_d_n6;
        var_isrh_dn7 = assign29680_e37066_d_n7;
        var_isrh_dn8 = assign29680_e37066_d_n8;
        var_isrh_dn9 = assign29680_e37066_d_n9;

        let (assign29690_e37080,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard593 == 0.0)) {
        let assign29690_e37078: f64 = (var_vbibot_d - var_vjsrh);
        (assign29690_e37078,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign29690_e37080;

        let (assign29700_e37099,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard593 == 0.0)) {
        let assign29700_e37094: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign29700_e37095: f64 = (1.0 - assign29700_e37094);
        let assign29700_e37096: f64 = (assign29700_e37095).sqrt();
        let assign29700_e37097: f64 = (1.0 - assign29700_e37096);
        (assign29700_e37097,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign29700_e37099;

        let assign29710_e37102: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard594 = assign29710_e37102;

        let (assign29720_e37116,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard593 == 0.0)) && (var_guard594 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign29720_e37116;

        let (assign29730_e37148,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard593 == 0.0)) && (var_guard594 == 0.0)) {
        let assign29730_e37131: f64 = (var_wsrhstep * var_wsrhstep);
        let assign29730_e37133: f64 = (var_wsrhstep).ln();
        let assign29730_e37134: f64 = (assign29730_e37131 * assign29730_e37133);
        let assign29730_e37137: f64 = (1.0 - var_wsrhstep);
        let assign29730_e37138: f64 = (assign29730_e37134 / assign29730_e37137);
        let assign29730_e37140: f64 = (assign29730_e37138 + var_wsrhstep);
        let assign29730_e37144: f64 = (2.0 * var_pbotd_i);
        let assign29730_e37145: f64 = (1.0 - assign29730_e37144);
        let assign29730_e37146: f64 = (assign29730_e37140 * assign29730_e37145);
        (assign29730_e37146,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign29730_e37148;

        let (assign29740_e37162,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard593 == 0.0)) {
        let assign29740_e37160: f64 = (var_wsrhstep + var_dwsrh);
        (assign29740_e37160,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign29740_e37162;

        let assign29750_e37165: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard595 = assign29750_e37165;

        let (assign29760_e37182, assign29760_e37182_d_n6, assign29760_e37182_d_n7, assign29760_e37182_d_n8, assign29760_e37182_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard593 == 0.0)) && (var_guard595 != 0.0)) {
        let assign29760_e37179: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign29760_e37180: f64 = (assign29760_e37179).sqrt();
        (assign29760_e37180, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign29760_e37182;
        var_tmp_dn6 = assign29760_e37182_d_n6;
        var_tmp_dn7 = assign29760_e37182_d_n7;
        var_tmp_dn8 = assign29760_e37182_d_n8;
        var_tmp_dn9 = assign29760_e37182_d_n9;

        let (assign29770_e37201, assign29770_e37201_d_n6, assign29770_e37201_d_n7, assign29770_e37201_d_n8, assign29770_e37201_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard593 == 0.0)) && (var_guard595 == 0.0)) {
        let assign29770_e37197: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv_d);
        let assign29770_e37199: f64 = (assign29770_e37197).powf(var_pbotd_i);
        (assign29770_e37199, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign29770_e37201;
        var_tmp_dn6 = assign29770_e37201_d_n6;
        var_tmp_dn7 = assign29770_e37201_d_n7;
        var_tmp_dn8 = assign29770_e37201_d_n8;
        var_tmp_dn9 = assign29770_e37201_d_n9;

        let (assign29780_e37215, assign29780_e37215_d_n6, assign29780_e37215_d_n7, assign29780_e37215_d_n8, assign29780_e37215_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard593 == 0.0)) {
        let assign29780_e37213: f64 = (var_wdepnulrbot_d * var_tmp);
        (assign29780_e37213, (var_wdepnulrbot_d * var_tmp_dn6), (var_wdepnulrbot_d * var_tmp_dn7), (var_wdepnulrbot_d * var_tmp_dn8), (var_wdepnulrbot_d * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign29780_e37215;
        var_wdep_dn6 = assign29780_e37215_d_n6;
        var_wdep_dn7 = assign29780_e37215_d_n7;
        var_wdep_dn8 = assign29780_e37215_d_n8;
        var_wdep_dn9 = assign29780_e37215_d_n9;

        *var_dwsrh_slot = var_dwsrh;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_guard587_slot = var_guard587;
        *var_guard588_slot = var_guard588;
        *var_guard589_slot = var_guard589;
        *var_guard590_slot = var_guard590;
        *var_guard591_slot = var_guard591;
        *var_guard592_slot = var_guard592;
        *var_guard593_slot = var_guard593;
        *var_guard594_slot = var_guard594;
        *var_guard595_slot = var_guard595;
        *var_i1_slot = var_i1;
        *var_i1_dn6_slot = var_i1_dn6;
        *var_i1_dn7_slot = var_i1_dn7;
        *var_i1_dn8_slot = var_i1_dn8;
        *var_i1_dn9_slot = var_i1_dn9;
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
        *var_wdep_slot = var_wdep;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wdep_dn9_slot = var_wdep_dn9;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_60(
        var_atatbot_d: f64,
        var_berfc: f64,
        var_btatpartbot_d: f64,
        var_cbbtbotd_i: f64,
        var_cerfc: f64,
        var_csrhbotd_i: f64,
        var_ctatbotd_i: f64,
        var_ftdbot_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard592: f64,
        var_guard593: f64,
        var_one_minus_pbot_d: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_perfc: f64,
        var_vbi_minus_vjsrh: f64,
        var_wdep: f64,
        var_wdep_dn6: f64,
        var_wdep_dn7: f64,
        var_wdep_dn8: f64,
        var_wdep_dn9: f64,
        var_wsrh: f64,
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
        var_gammamax_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_gammamax_dn9_slot: &mut f64,
        var_guard596_slot: &mut f64,
        var_guard597_slot: &mut f64,
        var_guard598_slot: &mut f64,
        var_guard599_slot: &mut f64,
        var_guard600_slot: &mut f64,
        var_guard601_slot: &mut f64,
        var_guard602_slot: &mut f64,
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
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_gammamax_dn9: f64 = *var_gammamax_dn9_slot;
        let mut var_guard596: f64 = *var_guard596_slot;
        let mut var_guard597: f64 = *var_guard597_slot;
        let mut var_guard598: f64 = *var_guard598_slot;
        let mut var_guard599: f64 = *var_guard599_slot;
        let mut var_guard600: f64 = *var_guard600_slot;
        let mut var_guard601: f64 = *var_guard601_slot;
        let mut var_guard602: f64 = *var_guard602_slot;
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

        let (assign29790_e37233, assign29790_e37233_d_n6, assign29790_e37233_d_n7, assign29790_e37233_d_n8, assign29790_e37233_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard593 == 0.0)) {
        let assign29790_e37228: f64 = (var_zinv - 1.0);
        let assign29790_e37230: f64 = (assign29790_e37228 * var_wdep);
        let assign29790_e37231: f64 = (var_ftdbot_d * assign29790_e37230);
        (assign29790_e37231, (var_ftdbot_d * (assign29790_e37228 * var_wdep_dn6)), (var_ftdbot_d * (assign29790_e37228 * var_wdep_dn7)), (var_ftdbot_d * (assign29790_e37228 * var_wdep_dn8)), (var_ftdbot_d * (assign29790_e37228 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign29790_e37233;
        var_asrh_dn6 = assign29790_e37233_d_n6;
        var_asrh_dn7 = assign29790_e37233_d_n7;
        var_asrh_dn8 = assign29790_e37233_d_n8;
        var_asrh_dn9 = assign29790_e37233_d_n9;

        let (assign29800_e37249, assign29800_e37249_d_n6, assign29800_e37249_d_n7, assign29800_e37249_d_n8, assign29800_e37249_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard593 == 0.0)) {
        let assign29800_e37246: f64 = (var_asrh * var_wsrh);
        let assign29800_e37247: f64 = (var_csrhbotd_i * assign29800_e37246);
        (assign29800_e37247, (var_csrhbotd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn8 * var_wsrh)), (var_csrhbotd_i * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign29800_e37249;
        var_isrh_dn6 = assign29800_e37249_d_n6;
        var_isrh_dn7 = assign29800_e37249_d_n7;
        var_isrh_dn8 = assign29800_e37249_d_n8;
        var_isrh_dn9 = assign29800_e37249_d_n9;

        let assign29810_e37252: f64 = if var_ctatbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard596 = assign29810_e37252;

        let (assign29820_e37263, assign29820_e37263_d_n6, assign29820_e37263_d_n7, assign29820_e37263_d_n8, assign29820_e37263_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign29820_e37263;
        var_itat_dn6 = assign29820_e37263_d_n6;
        var_itat_dn7 = assign29820_e37263_d_n7;
        var_itat_dn8 = assign29820_e37263_d_n8;
        var_itat_dn9 = assign29820_e37263_d_n9;

        let (assign29830_e37281, assign29830_e37281_d_n6, assign29830_e37281_d_n7, assign29830_e37281_d_n8, assign29830_e37281_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) {
        let assign29830_e37276: f64 = (var_wdep * var_one_minus_pbot_d);
        let assign29830_e37278: f64 = (assign29830_e37276 / var_vbi_minus_vjsrh);
        let assign29830_e37279: f64 = (var_btatpartbot_d * assign29830_e37278);
        (assign29830_e37279, (var_btatpartbot_d * ((var_wdep_dn6 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn7 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn8 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)), (var_btatpartbot_d * ((var_wdep_dn9 * var_one_minus_pbot_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign29830_e37281;
        var_btat_dn6 = assign29830_e37281_d_n6;
        var_btat_dn7 = assign29830_e37281_d_n7;
        var_btat_dn8 = assign29830_e37281_d_n8;
        var_btat_dn9 = assign29830_e37281_d_n9;

        let (assign29840_e37297, assign29840_e37297_d_n6, assign29840_e37297_d_n7, assign29840_e37297_d_n8, assign29840_e37297_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) {
        let assign29840_e37293: f64 = (0.666666666666667 * var_atatbot_d);
        let assign29840_e37295: f64 = (assign29840_e37293 / var_btat);
        (assign29840_e37295, (-((assign29840_e37293 * var_btat_dn6) / (var_btat * var_btat))), (-((assign29840_e37293 * var_btat_dn7) / (var_btat * var_btat))), (-((assign29840_e37293 * var_btat_dn8) / (var_btat * var_btat))), (-((assign29840_e37293 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign29840_e37297;
        var_twoatatoverthreebtat_dn6 = assign29840_e37297_d_n6;
        var_twoatatoverthreebtat_dn7 = assign29840_e37297_d_n7;
        var_twoatatoverthreebtat_dn8 = assign29840_e37297_d_n8;
        var_twoatatoverthreebtat_dn9 = assign29840_e37297_d_n9;

        let (assign29850_e37311, assign29850_e37311_d_n6, assign29850_e37311_d_n7, assign29850_e37311_d_n8, assign29850_e37311_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) {
        let assign29850_e37309: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign29850_e37309, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign29850_e37311;
        var_umaxbeforelimiting_dn6 = assign29850_e37311_d_n6;
        var_umaxbeforelimiting_dn7 = assign29850_e37311_d_n7;
        var_umaxbeforelimiting_dn8 = assign29850_e37311_d_n8;
        var_umaxbeforelimiting_dn9 = assign29850_e37311_d_n9;

        let (assign29860_e37332, assign29860_e37332_d_n6, assign29860_e37332_d_n7, assign29860_e37332_d_n8, assign29860_e37332_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) {
        let assign29860_e37323: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign29860_e37326: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign29860_e37328: f64 = (assign29860_e37326 + 1.0);
        let assign29860_e37329: f64 = (assign29860_e37323 / assign29860_e37328);
        let assign29860_e37330: f64 = (assign29860_e37329).sqrt();
        (assign29860_e37330, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign29860_e37328) - (assign29860_e37323 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign29860_e37328 * assign29860_e37328)) / (2.0 * assign29860_e37330)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign29860_e37328) - (assign29860_e37323 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign29860_e37328 * assign29860_e37328)) / (2.0 * assign29860_e37330)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign29860_e37328) - (assign29860_e37323 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign29860_e37328 * assign29860_e37328)) / (2.0 * assign29860_e37330)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign29860_e37328) - (assign29860_e37323 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign29860_e37328 * assign29860_e37328)) / (2.0 * assign29860_e37330)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign29860_e37332;
        var_umax_dn6 = assign29860_e37332_d_n6;
        var_umax_dn7 = assign29860_e37332_d_n7;
        var_umax_dn8 = assign29860_e37332_d_n8;
        var_umax_dn9 = assign29860_e37332_d_n9;

        let (assign29870_e37345, assign29870_e37345_d_n6, assign29870_e37345_d_n7, assign29870_e37345_d_n8, assign29870_e37345_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) {
        let assign29870_e37343: f64 = (var_umax).sqrt();
        (assign29870_e37343, (var_umax_dn6 / (2.0 * assign29870_e37343)), (var_umax_dn7 / (2.0 * assign29870_e37343)), (var_umax_dn8 / (2.0 * assign29870_e37343)), (var_umax_dn9 / (2.0 * assign29870_e37343)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign29870_e37345;
        var_sqrtumax_dn6 = assign29870_e37345_d_n6;
        var_sqrtumax_dn7 = assign29870_e37345_d_n7;
        var_sqrtumax_dn8 = assign29870_e37345_d_n8;
        var_sqrtumax_dn9 = assign29870_e37345_d_n9;

        let (assign29880_e37359, assign29880_e37359_d_n6, assign29880_e37359_d_n7, assign29880_e37359_d_n8, assign29880_e37359_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) {
        let assign29880_e37357: f64 = (var_umax * var_sqrtumax);
        (assign29880_e37357, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign29880_e37359;
        var_umaxpoweronepointfive_dn6 = assign29880_e37359_d_n6;
        var_umaxpoweronepointfive_dn7 = assign29880_e37359_d_n7;
        var_umaxpoweronepointfive_dn8 = assign29880_e37359_d_n8;
        var_umaxpoweronepointfive_dn9 = assign29880_e37359_d_n9;

        let assign29890_e37361: f64 = (-var_pbotd_i);
        let assign29890_e37363: f64 = (assign29890_e37361 * var_one_over_one_minus_pbot_d);
        let assign29890_e37365: f64 = (-1.0);
        let assign29890_e37366: f64 = if assign29890_e37363 == assign29890_e37365 { 1.0 } else { 0.0 };
        var_guard597 = assign29890_e37366;

        let (assign29900_e37386, assign29900_e37386_d_n6, assign29900_e37386_d_n7, assign29900_e37386_d_n8, assign29900_e37386_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) && (var_guard597 != 0.0)) {
        let assign29900_e37382: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign29900_e37383: f64 = (1.0 + assign29900_e37382);
        let assign29900_e37384: f64 = (1.0 / assign29900_e37383);
        (assign29900_e37384, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign29900_e37383 * assign29900_e37383))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign29900_e37383 * assign29900_e37383))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign29900_e37383 * assign29900_e37383))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign29900_e37383 * assign29900_e37383))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign29900_e37386;
        var_wgamma_dn6 = assign29900_e37386_d_n6;
        var_wgamma_dn7 = assign29900_e37386_d_n7;
        var_wgamma_dn8 = assign29900_e37386_d_n8;
        var_wgamma_dn9 = assign29900_e37386_d_n9;

        let (assign29910_e37410, assign29910_e37410_d_n6, assign29910_e37410_d_n7, assign29910_e37410_d_n8, assign29910_e37410_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) && (var_guard597 == 0.0)) {
        let assign29910_e37402: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign29910_e37403: f64 = (1.0 + assign29910_e37402);
        let assign29910_e37405: f64 = (-var_pbotd_i);
        let assign29910_e37407: f64 = (assign29910_e37405 * var_one_over_one_minus_pbot_d);
        let assign29910_e37408: f64 = (assign29910_e37403).powf(assign29910_e37407);
        (assign29910_e37408, if 0.0 == 0.0 && ((assign29910_e37407) as f64).is_finite() && ((assign29910_e37407) as f64).fract() == 0.0 { if assign29910_e37407 == 0.0 { 0.0 } else { (assign29910_e37407 * ((assign29910_e37403).powf(assign29910_e37407 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign29910_e37408 * (assign29910_e37407 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign29910_e37403))) }, if 0.0 == 0.0 && ((assign29910_e37407) as f64).is_finite() && ((assign29910_e37407) as f64).fract() == 0.0 { if assign29910_e37407 == 0.0 { 0.0 } else { (assign29910_e37407 * ((assign29910_e37403).powf(assign29910_e37407 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign29910_e37408 * (assign29910_e37407 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign29910_e37403))) }, if 0.0 == 0.0 && ((assign29910_e37407) as f64).is_finite() && ((assign29910_e37407) as f64).fract() == 0.0 { if assign29910_e37407 == 0.0 { 0.0 } else { (assign29910_e37407 * ((assign29910_e37403).powf(assign29910_e37407 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign29910_e37408 * (assign29910_e37407 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign29910_e37403))) }, if 0.0 == 0.0 && ((assign29910_e37407) as f64).is_finite() && ((assign29910_e37407) as f64).fract() == 0.0 { if assign29910_e37407 == 0.0 { 0.0 } else { (assign29910_e37407 * ((assign29910_e37403).powf(assign29910_e37407 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign29910_e37408 * (assign29910_e37407 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign29910_e37403))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign29910_e37410;
        var_wgamma_dn6 = assign29910_e37410_d_n6;
        var_wgamma_dn7 = assign29910_e37410_d_n7;
        var_wgamma_dn8 = assign29910_e37410_d_n8;
        var_wgamma_dn9 = assign29910_e37410_d_n9;

        let (assign29920_e37428, assign29920_e37428_d_n6, assign29920_e37428_d_n7, assign29920_e37428_d_n8, assign29920_e37428_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) {
        let assign29920_e37422: f64 = (var_wsrh * var_wgamma);
        let assign29920_e37425: f64 = (var_wsrh + var_wgamma);
        let assign29920_e37426: f64 = (assign29920_e37422 / assign29920_e37425);
        (assign29920_e37426, ((((var_wsrh * var_wgamma_dn6) * assign29920_e37425) - (assign29920_e37422 * var_wgamma_dn6)) / (assign29920_e37425 * assign29920_e37425)), ((((var_wsrh * var_wgamma_dn7) * assign29920_e37425) - (assign29920_e37422 * var_wgamma_dn7)) / (assign29920_e37425 * assign29920_e37425)), ((((var_wsrh * var_wgamma_dn8) * assign29920_e37425) - (assign29920_e37422 * var_wgamma_dn8)) / (assign29920_e37425 * assign29920_e37425)), ((((var_wsrh * var_wgamma_dn9) * assign29920_e37425) - (assign29920_e37422 * var_wgamma_dn9)) / (assign29920_e37425 * assign29920_e37425)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign29920_e37428;
        var_wtat_dn6 = assign29920_e37428_d_n6;
        var_wtat_dn7 = assign29920_e37428_d_n7;
        var_wtat_dn8 = assign29920_e37428_d_n8;
        var_wtat_dn9 = assign29920_e37428_d_n9;

        let (assign29930_e37445, assign29930_e37445_d_n6, assign29930_e37445_d_n7, assign29930_e37445_d_n8, assign29930_e37445_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) {
        let assign29930_e37441: f64 = (var_btat / var_sqrtumax);
        let assign29930_e37442: f64 = (0.375 * assign29930_e37441);
        let assign29930_e37443: f64 = (assign29930_e37442).sqrt();
        (assign29930_e37443, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign29930_e37443)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign29930_e37443)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign29930_e37443)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign29930_e37443)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign29930_e37445;
        var_ktat_dn6 = assign29930_e37445_d_n6;
        var_ktat_dn7 = assign29930_e37445_d_n7;
        var_ktat_dn8 = assign29930_e37445_d_n8;
        var_ktat_dn9 = assign29930_e37445_d_n9;

        let (assign29940_e37463, assign29940_e37463_d_n6, assign29940_e37463_d_n7, assign29940_e37463_d_n8, assign29940_e37463_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) {
        let assign29940_e37458: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign29940_e37459: f64 = (2.0 * assign29940_e37458);
        let assign29940_e37461: f64 = (assign29940_e37459 - var_umax);
        (assign29940_e37461, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign29940_e37463;
        var_ltat_dn6 = assign29940_e37463_d_n6;
        var_ltat_dn7 = assign29940_e37463_d_n7;
        var_ltat_dn8 = assign29940_e37463_d_n8;
        var_ltat_dn9 = assign29940_e37463_d_n9;

        let (assign29950_e37489, assign29950_e37489_d_n6, assign29950_e37489_d_n7, assign29950_e37489_d_n8, assign29950_e37489_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) {
        let assign29950_e37475: f64 = (var_atatbot_d * var_twoatatoverthreebtat);
        let assign29950_e37477: f64 = (assign29950_e37475 * var_sqrtumax);
        let assign29950_e37480: f64 = (var_atatbot_d * var_umax);
        let assign29950_e37481: f64 = (assign29950_e37477 - assign29950_e37480);
        let assign29950_e37485: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign29950_e37486: f64 = (0.5 * assign29950_e37485);
        let assign29950_e37487: f64 = (assign29950_e37481 + assign29950_e37486);
        (assign29950_e37487, (((((var_atatbot_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign29950_e37475 * var_sqrtumax_dn6)) - (var_atatbot_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign29950_e37475 * var_sqrtumax_dn7)) - (var_atatbot_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign29950_e37475 * var_sqrtumax_dn8)) - (var_atatbot_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatbot_d * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign29950_e37475 * var_sqrtumax_dn9)) - (var_atatbot_d * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign29950_e37489;
        var_mtat_dn6 = assign29950_e37489_d_n6;
        var_mtat_dn7 = assign29950_e37489_d_n7;
        var_mtat_dn8 = assign29950_e37489_d_n8;
        var_mtat_dn9 = assign29950_e37489_d_n9;

        let (assign29960_e37505, assign29960_e37505_d_n6, assign29960_e37505_d_n7, assign29960_e37505_d_n8, assign29960_e37505_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) {
        let assign29960_e37501: f64 = (var_ltat - 1.0);
        let assign29960_e37503: f64 = (assign29960_e37501 * var_ktat);
        (assign29960_e37503, ((var_ltat_dn6 * var_ktat) + (assign29960_e37501 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign29960_e37501 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign29960_e37501 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign29960_e37501 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign29960_e37505;
        var_xerfc_dn6 = assign29960_e37505_d_n6;
        var_xerfc_dn7 = assign29960_e37505_d_n7;
        var_xerfc_dn8 = assign29960_e37505_d_n8;
        var_xerfc_dn9 = assign29960_e37505_d_n9;

        let (assign29970_e37519, assign29970_e37519_d_n6, assign29970_e37519_d_n7, assign29970_e37519_d_n8, assign29970_e37519_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) {
        let assign29970_e37517: f64 = (var_xerfc * var_xerfc);
        (assign29970_e37517, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign29970_e37519;
        var_ysq_dn6 = assign29970_e37519_d_n6;
        var_ysq_dn7 = assign29970_e37519_d_n7;
        var_ysq_dn8 = assign29970_e37519_d_n8;
        var_ysq_dn9 = assign29970_e37519_d_n9;

        let assign29980_e37522: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard598 = assign29980_e37522;

        let (assign29990_e37542, assign29990_e37542_d_n6, assign29990_e37542_d_n7, assign29990_e37542_d_n8, assign29990_e37542_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) && (var_guard598 != 0.0)) {
        let assign29990_e37538: f64 = (var_perfc * var_xerfc);
        let assign29990_e37539: f64 = (1.0 + assign29990_e37538);
        let assign29990_e37540: f64 = (1.0 / assign29990_e37539);
        (assign29990_e37540, (-((var_perfc * var_xerfc_dn6) / (assign29990_e37539 * assign29990_e37539))), (-((var_perfc * var_xerfc_dn7) / (assign29990_e37539 * assign29990_e37539))), (-((var_perfc * var_xerfc_dn8) / (assign29990_e37539 * assign29990_e37539))), (-((var_perfc * var_xerfc_dn9) / (assign29990_e37539 * assign29990_e37539))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign29990_e37542;
        var_terfc_dn6 = assign29990_e37542_d_n6;
        var_terfc_dn7 = assign29990_e37542_d_n7;
        var_terfc_dn8 = assign29990_e37542_d_n8;
        var_terfc_dn9 = assign29990_e37542_d_n9;

        let (assign30000_e37563, assign30000_e37563_d_n6, assign30000_e37563_d_n7, assign30000_e37563_d_n8, assign30000_e37563_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) && (var_guard598 == 0.0)) {
        let assign30000_e37559: f64 = (var_perfc * var_xerfc);
        let assign30000_e37560: f64 = (1.0 - assign30000_e37559);
        let assign30000_e37561: f64 = (1.0 / assign30000_e37560);
        (assign30000_e37561, (-((-(var_perfc * var_xerfc_dn6)) / (assign30000_e37560 * assign30000_e37560))), (-((-(var_perfc * var_xerfc_dn7)) / (assign30000_e37560 * assign30000_e37560))), (-((-(var_perfc * var_xerfc_dn8)) / (assign30000_e37560 * assign30000_e37560))), (-((-(var_perfc * var_xerfc_dn9)) / (assign30000_e37560 * assign30000_e37560))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign30000_e37563;
        var_terfc_dn6 = assign30000_e37563_d_n6;
        var_terfc_dn7 = assign30000_e37563_d_n7;
        var_terfc_dn8 = assign30000_e37563_d_n8;
        var_terfc_dn9 = assign30000_e37563_d_n9;

        let assign30010_e37565: f64 = (-var_ysq);
        let assign30010_e37567: f64 = (assign30010_e37565 + var_mtat);
        let assign30010_e37569: f64 = (-230.25850929940458);
        let assign30010_e37570: f64 = if assign30010_e37567 > assign30010_e37569 { 1.0 } else { 0.0 };
        var_guard599 = assign30010_e37570;

        let (assign30020_e37588, assign30020_e37588_d_n6, assign30020_e37588_d_n7, assign30020_e37588_d_n8, assign30020_e37588_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) && (var_guard599 != 0.0)) {
        let assign30020_e37583: f64 = (-var_ysq);
        let assign30020_e37585: f64 = (assign30020_e37583 + var_mtat);
        let assign30020_e37586: f64 = (assign30020_e37585).exp();
        (assign30020_e37586, (assign30020_e37586 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign30020_e37586 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign30020_e37586 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign30020_e37586 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30020_e37588;
        var_tmp_dn6 = assign30020_e37588_d_n6;
        var_tmp_dn7 = assign30020_e37588_d_n7;
        var_tmp_dn8 = assign30020_e37588_d_n8;
        var_tmp_dn9 = assign30020_e37588_d_n9;

        let (assign30030_e37637, assign30030_e37637_d_n6, assign30030_e37637_d_n7, assign30030_e37637_d_n8, assign30030_e37637_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) && (var_guard599 == 0.0)) {
        let assign30030_e37604: f64 = (-230.25850929940458);
        let assign30030_e37606: f64 = (-var_ysq);
        let assign30030_e37608: f64 = (assign30030_e37606 + var_mtat);
        let assign30030_e37609: f64 = (assign30030_e37604 - assign30030_e37608);
        let assign30030_e37613: f64 = (-230.25850929940458);
        let assign30030_e37615: f64 = (-var_ysq);
        let assign30030_e37617: f64 = (assign30030_e37615 + var_mtat);
        let assign30030_e37618: f64 = (assign30030_e37613 - assign30030_e37617);
        let assign30030_e37621: f64 = (-230.25850929940458);
        let assign30030_e37623: f64 = (-var_ysq);
        let assign30030_e37625: f64 = (assign30030_e37623 + var_mtat);
        let assign30030_e37626: f64 = (assign30030_e37621 - assign30030_e37625);
        let assign30030_e37628: f64 = (assign30030_e37626 * 0.3333333333333333);
        let assign30030_e37629: f64 = (1.0 + assign30030_e37628);
        let assign30030_e37630: f64 = (assign30030_e37618 * assign30030_e37629);
        let assign30030_e37631: f64 = (0.5 * assign30030_e37630);
        let assign30030_e37632: f64 = (1.0 + assign30030_e37631);
        let assign30030_e37633: f64 = (assign30030_e37609 * assign30030_e37632);
        let assign30030_e37634: f64 = (1.0 + assign30030_e37633);
        let assign30030_e37635: f64 = (1e-100 / assign30030_e37634);
        (assign30030_e37635, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign30030_e37632) + (assign30030_e37609 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign30030_e37629) + (assign30030_e37618 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign30030_e37634 * assign30030_e37634))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign30030_e37632) + (assign30030_e37609 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign30030_e37629) + (assign30030_e37618 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign30030_e37634 * assign30030_e37634))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign30030_e37632) + (assign30030_e37609 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign30030_e37629) + (assign30030_e37618 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign30030_e37634 * assign30030_e37634))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign30030_e37632) + (assign30030_e37609 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign30030_e37629) + (assign30030_e37618 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign30030_e37634 * assign30030_e37634))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30030_e37637;
        var_tmp_dn6 = assign30030_e37637_d_n6;
        var_tmp_dn7 = assign30030_e37637_d_n7;
        var_tmp_dn8 = assign30030_e37637_d_n8;
        var_tmp_dn9 = assign30030_e37637_d_n9;

        let (assign30040_e37667, assign30040_e37667_d_n6, assign30040_e37667_d_n7, assign30040_e37667_d_n8, assign30040_e37667_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) {
        let assign30040_e37649: f64 = (0.29214664 * var_terfc);
        let assign30040_e37653: f64 = (var_terfc * var_terfc);
        let assign30040_e37654: f64 = (var_berfc * assign30040_e37653);
        let assign30040_e37655: f64 = (assign30040_e37649 + assign30040_e37654);
        let assign30040_e37659: f64 = (var_terfc * var_terfc);
        let assign30040_e37661: f64 = (assign30040_e37659 * var_terfc);
        let assign30040_e37662: f64 = (var_cerfc * assign30040_e37661);
        let assign30040_e37663: f64 = (assign30040_e37655 + assign30040_e37662);
        let assign30040_e37665: f64 = (assign30040_e37663 * var_tmp);
        (assign30040_e37665, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign30040_e37659 * var_terfc_dn6)))) * var_tmp) + (assign30040_e37663 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign30040_e37659 * var_terfc_dn7)))) * var_tmp) + (assign30040_e37663 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign30040_e37659 * var_terfc_dn8)))) * var_tmp) + (assign30040_e37663 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign30040_e37659 * var_terfc_dn9)))) * var_tmp) + (assign30040_e37663 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign30040_e37667;
        var_erfcpos_dn6 = assign30040_e37667_d_n6;
        var_erfcpos_dn7 = assign30040_e37667_d_n7;
        var_erfcpos_dn8 = assign30040_e37667_d_n8;
        var_erfcpos_dn9 = assign30040_e37667_d_n9;

        let assign30050_e37670: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard600 = assign30050_e37670;

        let (assign30060_e37684, assign30060_e37684_d_n6, assign30060_e37684_d_n7, assign30060_e37684_d_n8, assign30060_e37684_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) && (var_guard600 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign30060_e37684;
        var_erfctimesexpmtat_dn6 = assign30060_e37684_d_n6;
        var_erfctimesexpmtat_dn7 = assign30060_e37684_d_n7;
        var_erfctimesexpmtat_dn8 = assign30060_e37684_d_n8;
        var_erfctimesexpmtat_dn9 = assign30060_e37684_d_n9;

        let assign30070_e37687: f64 = (-230.25850929940458);
        let assign30070_e37688: f64 = if var_mtat > assign30070_e37687 { 1.0 } else { 0.0 };
        var_guard601 = assign30070_e37688;

        let (assign30080_e37706, assign30080_e37706_d_n6, assign30080_e37706_d_n7, assign30080_e37706_d_n8, assign30080_e37706_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) && (var_guard600 == 0.0)) && (var_guard601 != 0.0)) {
        let assign30080_e37704: f64 = (var_mtat).exp();
        (assign30080_e37704, (assign30080_e37704 * var_mtat_dn6), (assign30080_e37704 * var_mtat_dn7), (assign30080_e37704 * var_mtat_dn8), (assign30080_e37704 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30080_e37706;
        var_tmp_dn6 = assign30080_e37706_d_n6;
        var_tmp_dn7 = assign30080_e37706_d_n7;
        var_tmp_dn8 = assign30080_e37706_d_n8;
        var_tmp_dn9 = assign30080_e37706_d_n9;

        let (assign30090_e37749, assign30090_e37749_d_n6, assign30090_e37749_d_n7, assign30090_e37749_d_n8, assign30090_e37749_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) && (var_guard600 == 0.0)) && (var_guard601 == 0.0)) {
        let assign30090_e37725: f64 = (-230.25850929940458);
        let assign30090_e37727: f64 = (assign30090_e37725 - var_mtat);
        let assign30090_e37731: f64 = (-230.25850929940458);
        let assign30090_e37733: f64 = (assign30090_e37731 - var_mtat);
        let assign30090_e37736: f64 = (-230.25850929940458);
        let assign30090_e37738: f64 = (assign30090_e37736 - var_mtat);
        let assign30090_e37740: f64 = (assign30090_e37738 * 0.3333333333333333);
        let assign30090_e37741: f64 = (1.0 + assign30090_e37740);
        let assign30090_e37742: f64 = (assign30090_e37733 * assign30090_e37741);
        let assign30090_e37743: f64 = (0.5 * assign30090_e37742);
        let assign30090_e37744: f64 = (1.0 + assign30090_e37743);
        let assign30090_e37745: f64 = (assign30090_e37727 * assign30090_e37744);
        let assign30090_e37746: f64 = (1.0 + assign30090_e37745);
        let assign30090_e37747: f64 = (1e-100 / assign30090_e37746);
        (assign30090_e37747, (-((1e-100 * (((-var_mtat_dn6) * assign30090_e37744) + (assign30090_e37727 * (0.5 * (((-var_mtat_dn6) * assign30090_e37741) + (assign30090_e37733 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign30090_e37746 * assign30090_e37746))), (-((1e-100 * (((-var_mtat_dn7) * assign30090_e37744) + (assign30090_e37727 * (0.5 * (((-var_mtat_dn7) * assign30090_e37741) + (assign30090_e37733 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign30090_e37746 * assign30090_e37746))), (-((1e-100 * (((-var_mtat_dn8) * assign30090_e37744) + (assign30090_e37727 * (0.5 * (((-var_mtat_dn8) * assign30090_e37741) + (assign30090_e37733 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign30090_e37746 * assign30090_e37746))), (-((1e-100 * (((-var_mtat_dn9) * assign30090_e37744) + (assign30090_e37727 * (0.5 * (((-var_mtat_dn9) * assign30090_e37741) + (assign30090_e37733 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign30090_e37746 * assign30090_e37746))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30090_e37749;
        var_tmp_dn6 = assign30090_e37749_d_n6;
        var_tmp_dn7 = assign30090_e37749_d_n7;
        var_tmp_dn8 = assign30090_e37749_d_n8;
        var_tmp_dn9 = assign30090_e37749_d_n9;

        let (assign30100_e37768, assign30100_e37768_d_n6, assign30100_e37768_d_n7, assign30100_e37768_d_n8, assign30100_e37768_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) && (var_guard600 == 0.0)) {
        let assign30100_e37764: f64 = (2.0 * var_tmp);
        let assign30100_e37766: f64 = (assign30100_e37764 - var_erfcpos);
        (assign30100_e37766, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign30100_e37768;
        var_erfctimesexpmtat_dn6 = assign30100_e37768_d_n6;
        var_erfctimesexpmtat_dn7 = assign30100_e37768_d_n7;
        var_erfctimesexpmtat_dn8 = assign30100_e37768_d_n8;
        var_erfctimesexpmtat_dn9 = assign30100_e37768_d_n9;

        let (assign30110_e37788, assign30110_e37788_d_n6, assign30110_e37788_d_n7, assign30110_e37788_d_n8, assign30110_e37788_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) {
        let assign30110_e37780: f64 = (1.772453850905516 * 0.5);
        let assign30110_e37783: f64 = (var_atatbot_d * var_erfctimesexpmtat);
        let assign30110_e37785: f64 = (assign30110_e37783 / var_ktat);
        let assign30110_e37786: f64 = (assign30110_e37780 * assign30110_e37785);
        (assign30110_e37786, (assign30110_e37780 * ((((var_atatbot_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign30110_e37783 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign30110_e37780 * ((((var_atatbot_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign30110_e37783 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign30110_e37780 * ((((var_atatbot_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign30110_e37783 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign30110_e37780 * ((((var_atatbot_d * var_erfctimesexpmtat_dn9) * var_ktat) - (assign30110_e37783 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign30110_e37788;
        var_gammamax_dn6 = assign30110_e37788_d_n6;
        var_gammamax_dn7 = assign30110_e37788_d_n7;
        var_gammamax_dn8 = assign30110_e37788_d_n8;
        var_gammamax_dn9 = assign30110_e37788_d_n9;

        let (assign30120_e37806, assign30120_e37806_d_n6, assign30120_e37806_d_n7, assign30120_e37806_d_n8, assign30120_e37806_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard596 == 0.0)) {
        let assign30120_e37801: f64 = (var_asrh * var_gammamax);
        let assign30120_e37803: f64 = (assign30120_e37801 * var_wtat);
        let assign30120_e37804: f64 = (var_ctatbotd_i * assign30120_e37803);
        (assign30120_e37804, (var_ctatbotd_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign30120_e37801 * var_wtat_dn6))), (var_ctatbotd_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign30120_e37801 * var_wtat_dn7))), (var_ctatbotd_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign30120_e37801 * var_wtat_dn8))), (var_ctatbotd_i * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign30120_e37801 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign30120_e37806;
        var_itat_dn6 = assign30120_e37806_d_n6;
        var_itat_dn7 = assign30120_e37806_d_n7;
        var_itat_dn8 = assign30120_e37806_d_n8;
        var_itat_dn9 = assign30120_e37806_d_n9;

        let assign30130_e37809: f64 = if var_cbbtbotd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard602 = assign30130_e37809;

        let (assign30140_e37820, assign30140_e37820_d_n6, assign30140_e37820_d_n7, assign30140_e37820_d_n8, assign30140_e37820_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard602 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign30140_e37820;
        var_ibbt_dn6 = assign30140_e37820_d_n6;
        var_ibbt_dn7 = assign30140_e37820_d_n7;
        var_ibbt_dn8 = assign30140_e37820_d_n8;
        var_ibbt_dn9 = assign30140_e37820_d_n9;

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
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_gammamax_dn9_slot = var_gammamax_dn9;
        *var_guard596_slot = var_guard596;
        *var_guard597_slot = var_guard597;
        *var_guard598_slot = var_guard598;
        *var_guard599_slot = var_guard599;
        *var_guard600_slot = var_guard600;
        *var_guard601_slot = var_guard601;
        *var_guard602_slot = var_guard602;
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

    pub(super) fn stamp_transient_block_61(
        p: &Parameters,
        var_alphaav: f64,
        var_atatsti_d: f64,
        var_btatpartsti_d: f64,
        var_cbbtbotd_i: f64,
        var_csrhstid_i: f64,
        var_ctatstid_i: f64,
        var_fbbtbot_d: f64,
        var_fstopbot_d: f64,
        var_ftdsti_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard592: f64,
        var_guard602: f64,
        var_idmult: f64,
        var_idsatsti_d: f64,
        var_lsdrain_i: f64,
        var_one_minus_psti_d: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_pbotd_i: f64,
        var_pbrbotd_i: f64,
        var_pstid_i: f64,
        var_slopebot_d: f64,
        var_two_psistar: f64,
        var_v2: f64,
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
        var_guard613_slot: &mut f64,
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
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_dn9_slot: &mut f64,
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
        let mut var_guard613: f64 = *var_guard613_slot;
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
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_twoatatoverthreebtat_dn9: f64 = *var_twoatatoverthreebtat_dn9_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wdep_dn9: f64 = *var_wdep_dn9_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;

        let assign30150_e37823: f64 = if var_pbotd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard603 = assign30150_e37823;

        let (assign30160_e37842, assign30160_e37842_d_n6, assign30160_e37842_d_n7, assign30160_e37842_d_n8, assign30160_e37842_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard602 == 0.0)) && (var_guard603 != 0.0)) {
        let assign30160_e37837: f64 = (var_vbirbotd_i - var_vbbt);
        let assign30160_e37839: f64 = (assign30160_e37837 * var_vbirbotinv_d);
        let assign30160_e37840: f64 = (assign30160_e37839).sqrt();
        (assign30160_e37840, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30160_e37842;
        var_tmp_dn6 = assign30160_e37842_d_n6;
        var_tmp_dn7 = assign30160_e37842_d_n7;
        var_tmp_dn8 = assign30160_e37842_d_n8;
        var_tmp_dn9 = assign30160_e37842_d_n9;

        let (assign30170_e37863, assign30170_e37863_d_n6, assign30170_e37863_d_n7, assign30170_e37863_d_n8, assign30170_e37863_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard602 == 0.0)) && (var_guard603 == 0.0)) {
        let assign30170_e37857: f64 = (var_vbirbotd_i - var_vbbt);
        let assign30170_e37859: f64 = (assign30170_e37857 * var_vbirbotinv_d);
        let assign30170_e37861: f64 = (assign30170_e37859).powf(var_pbotd_i);
        (assign30170_e37861, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30170_e37863;
        var_tmp_dn6 = assign30170_e37863_d_n6;
        var_tmp_dn7 = assign30170_e37863_d_n7;
        var_tmp_dn8 = assign30170_e37863_d_n8;
        var_tmp_dn9 = assign30170_e37863_d_n9;

        let (assign30180_e37883, assign30180_e37883_d_n6, assign30180_e37883_d_n7, assign30180_e37883_d_n8, assign30180_e37883_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard602 == 0.0)) {
        let assign30180_e37876: f64 = (var_vbirbotd_i - var_vbbt);
        let assign30180_e37878: f64 = (assign30180_e37876 * var_wdepnulrinvbot_d);
        let assign30180_e37880: f64 = (assign30180_e37878 / var_tmp);
        let assign30180_e37881: f64 = (var_one_over_one_minus_pbot_d * assign30180_e37880);
        (assign30180_e37881, (var_one_over_one_minus_pbot_d * (-((assign30180_e37878 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign30180_e37878 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign30180_e37878 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot_d * (-((assign30180_e37878 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign30180_e37883;
        var_fmaxr_dn6 = assign30180_e37883_d_n6;
        var_fmaxr_dn7 = assign30180_e37883_d_n7;
        var_fmaxr_dn8 = assign30180_e37883_d_n8;
        var_fmaxr_dn9 = assign30180_e37883_d_n9;

        let assign30190_e37885: f64 = (-var_fbbtbot_d);
        let assign30190_e37887: f64 = (assign30190_e37885 / var_fmaxr);
        let assign30190_e37888: f64 = (assign30190_e37887).abs();
        let assign30190_e37890: f64 = if assign30190_e37888 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard604 = assign30190_e37890;

        let (assign30200_e37908, assign30200_e37908_d_n6, assign30200_e37908_d_n7, assign30200_e37908_d_n8, assign30200_e37908_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard602 == 0.0)) && (var_guard604 != 0.0)) {
        let assign30200_e37903: f64 = (-var_fbbtbot_d);
        let assign30200_e37905: f64 = (assign30200_e37903 / var_fmaxr);
        let assign30200_e37906: f64 = (assign30200_e37905).exp();
        (assign30200_e37906, (assign30200_e37906 * (-((assign30200_e37903 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign30200_e37906 * (-((assign30200_e37903 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign30200_e37906 * (-((assign30200_e37903 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign30200_e37906 * (-((assign30200_e37903 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30200_e37908;
        var_tmp_dn6 = assign30200_e37908_d_n6;
        var_tmp_dn7 = assign30200_e37908_d_n7;
        var_tmp_dn8 = assign30200_e37908_d_n8;
        var_tmp_dn9 = assign30200_e37908_d_n9;

        let assign30210_e37910: f64 = (-var_fbbtbot_d);
        let assign30210_e37912: f64 = (assign30210_e37910 / var_fmaxr);
        let assign30210_e37914: f64 = if assign30210_e37912 < 0.0 { 1.0 } else { 0.0 };
        var_guard605 = assign30210_e37914;

        let (assign30220_e37965, assign30220_e37965_d_n6, assign30220_e37965_d_n7, assign30220_e37965_d_n8, assign30220_e37965_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard602 == 0.0)) && (var_guard604 == 0.0)) && (var_guard605 != 0.0)) {
        let assign30220_e37932: f64 = (-230.25850929940458);
        let assign30220_e37934: f64 = (-var_fbbtbot_d);
        let assign30220_e37936: f64 = (assign30220_e37934 / var_fmaxr);
        let assign30220_e37937: f64 = (assign30220_e37932 - assign30220_e37936);
        let assign30220_e37941: f64 = (-230.25850929940458);
        let assign30220_e37943: f64 = (-var_fbbtbot_d);
        let assign30220_e37945: f64 = (assign30220_e37943 / var_fmaxr);
        let assign30220_e37946: f64 = (assign30220_e37941 - assign30220_e37945);
        let assign30220_e37949: f64 = (-230.25850929940458);
        let assign30220_e37951: f64 = (-var_fbbtbot_d);
        let assign30220_e37953: f64 = (assign30220_e37951 / var_fmaxr);
        let assign30220_e37954: f64 = (assign30220_e37949 - assign30220_e37953);
        let assign30220_e37956: f64 = (assign30220_e37954 * 0.3333333333333333);
        let assign30220_e37957: f64 = (1.0 + assign30220_e37956);
        let assign30220_e37958: f64 = (assign30220_e37946 * assign30220_e37957);
        let assign30220_e37959: f64 = (0.5 * assign30220_e37958);
        let assign30220_e37960: f64 = (1.0 + assign30220_e37959);
        let assign30220_e37961: f64 = (assign30220_e37937 * assign30220_e37960);
        let assign30220_e37962: f64 = (1.0 + assign30220_e37961);
        let assign30220_e37963: f64 = (1e-100 / assign30220_e37962);
        (assign30220_e37963, (-((1e-100 * (((-(-((assign30220_e37934 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign30220_e37960) + (assign30220_e37937 * (0.5 * (((-(-((assign30220_e37943 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign30220_e37957) + (assign30220_e37946 * ((-(-((assign30220_e37951 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign30220_e37962 * assign30220_e37962))), (-((1e-100 * (((-(-((assign30220_e37934 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign30220_e37960) + (assign30220_e37937 * (0.5 * (((-(-((assign30220_e37943 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign30220_e37957) + (assign30220_e37946 * ((-(-((assign30220_e37951 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign30220_e37962 * assign30220_e37962))), (-((1e-100 * (((-(-((assign30220_e37934 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign30220_e37960) + (assign30220_e37937 * (0.5 * (((-(-((assign30220_e37943 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign30220_e37957) + (assign30220_e37946 * ((-(-((assign30220_e37951 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign30220_e37962 * assign30220_e37962))), (-((1e-100 * (((-(-((assign30220_e37934 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign30220_e37960) + (assign30220_e37937 * (0.5 * (((-(-((assign30220_e37943 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign30220_e37957) + (assign30220_e37946 * ((-(-((assign30220_e37951 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign30220_e37962 * assign30220_e37962))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30220_e37965;
        var_tmp_dn6 = assign30220_e37965_d_n6;
        var_tmp_dn7 = assign30220_e37965_d_n7;
        var_tmp_dn8 = assign30220_e37965_d_n8;
        var_tmp_dn9 = assign30220_e37965_d_n9;

        let (assign30230_e38014, assign30230_e38014_d_n6, assign30230_e38014_d_n7, assign30230_e38014_d_n8, assign30230_e38014_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard602 == 0.0)) && (var_guard604 == 0.0)) && (var_guard605 == 0.0)) {
        let assign30230_e37984: f64 = (-var_fbbtbot_d);
        let assign30230_e37986: f64 = (assign30230_e37984 / var_fmaxr);
        let assign30230_e37988: f64 = (assign30230_e37986 - 230.25850929940458);
        let assign30230_e37992: f64 = (-var_fbbtbot_d);
        let assign30230_e37994: f64 = (assign30230_e37992 / var_fmaxr);
        let assign30230_e37996: f64 = (assign30230_e37994 - 230.25850929940458);
        let assign30230_e37999: f64 = (-var_fbbtbot_d);
        let assign30230_e38001: f64 = (assign30230_e37999 / var_fmaxr);
        let assign30230_e38003: f64 = (assign30230_e38001 - 230.25850929940458);
        let assign30230_e38005: f64 = (assign30230_e38003 * 0.3333333333333333);
        let assign30230_e38006: f64 = (1.0 + assign30230_e38005);
        let assign30230_e38007: f64 = (assign30230_e37996 * assign30230_e38006);
        let assign30230_e38008: f64 = (0.5 * assign30230_e38007);
        let assign30230_e38009: f64 = (1.0 + assign30230_e38008);
        let assign30230_e38010: f64 = (assign30230_e37988 * assign30230_e38009);
        let assign30230_e38011: f64 = (1.0 + assign30230_e38010);
        let assign30230_e38012: f64 = (1e100 * assign30230_e38011);
        (assign30230_e38012, (1e100 * (((-((assign30230_e37984 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign30230_e38009) + (assign30230_e37988 * (0.5 * (((-((assign30230_e37992 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign30230_e38006) + (assign30230_e37996 * ((-((assign30230_e37999 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign30230_e37984 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign30230_e38009) + (assign30230_e37988 * (0.5 * (((-((assign30230_e37992 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign30230_e38006) + (assign30230_e37996 * ((-((assign30230_e37999 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign30230_e37984 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign30230_e38009) + (assign30230_e37988 * (0.5 * (((-((assign30230_e37992 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign30230_e38006) + (assign30230_e37996 * ((-((assign30230_e37999 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign30230_e37984 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign30230_e38009) + (assign30230_e37988 * (0.5 * (((-((assign30230_e37992 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign30230_e38006) + (assign30230_e37996 * ((-((assign30230_e37999 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30230_e38014;
        var_tmp_dn6 = assign30230_e38014_d_n6;
        var_tmp_dn7 = assign30230_e38014_d_n7;
        var_tmp_dn8 = assign30230_e38014_d_n8;
        var_tmp_dn9 = assign30230_e38014_d_n9;

        let (assign30240_e38034, assign30240_e38034_d_n6, assign30240_e38034_d_n7, assign30240_e38034_d_n8, assign30240_e38034_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard602 == 0.0)) {
        let assign30240_e38027: f64 = (var_v2 * var_fmaxr);
        let assign30240_e38029: f64 = (assign30240_e38027 * var_fmaxr);
        let assign30240_e38031: f64 = (assign30240_e38029 * var_tmp);
        let assign30240_e38032: f64 = (var_cbbtbotd_i * assign30240_e38031);
        (assign30240_e38032, (var_cbbtbotd_i * (((((var_v2 * var_fmaxr_dn6) * var_fmaxr) + (assign30240_e38027 * var_fmaxr_dn6)) * var_tmp) + (assign30240_e38029 * var_tmp_dn6))), (var_cbbtbotd_i * (((((var_v2 * var_fmaxr_dn7) * var_fmaxr) + (assign30240_e38027 * var_fmaxr_dn7)) * var_tmp) + (assign30240_e38029 * var_tmp_dn7))), (var_cbbtbotd_i * (((((var_v2 * var_fmaxr_dn8) * var_fmaxr) + (assign30240_e38027 * var_fmaxr_dn8)) * var_tmp) + (assign30240_e38029 * var_tmp_dn8))), (var_cbbtbotd_i * (((((var_v2 * var_fmaxr_dn9) * var_fmaxr) + (assign30240_e38027 * var_fmaxr_dn9)) * var_tmp) + (assign30240_e38029 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign30240_e38034;
        var_ibbt_dn6 = assign30240_e38034_d_n6;
        var_ibbt_dn7 = assign30240_e38034_d_n7;
        var_ibbt_dn8 = assign30240_e38034_d_n8;
        var_ibbt_dn9 = assign30240_e38034_d_n9;

        let assign30250_e38037: f64 = if var_vbrbotd_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard606 = assign30250_e38037;

        let (assign30260_e38048, assign30260_e38048_d_n6, assign30260_e38048_d_n7, assign30260_e38048_d_n8, assign30260_e38048_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard606 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign30260_e38048;
        var_fbreakdown_dn6 = assign30260_e38048_d_n6;
        var_fbreakdown_dn7 = assign30260_e38048_d_n7;
        var_fbreakdown_dn8 = assign30260_e38048_d_n8;
        var_fbreakdown_dn9 = assign30260_e38048_d_n9;

        let assign30270_e38051: f64 = (-var_alphaav);
        let assign30270_e38053: f64 = (assign30270_e38051 * var_vbrbotd_i);
        let assign30270_e38054: f64 = if var_vav > assign30270_e38053 { 1.0 } else { 0.0 };
        var_guard607 = assign30270_e38054;

        let assign30280_e38057: f64 = if var_pbrbotd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard608 = assign30280_e38057;

        let (assign30290_e38087, assign30290_e38087_d_n6, assign30290_e38087_d_n7, assign30290_e38087_d_n8, assign30290_e38087_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard606 == 0.0)) && (var_guard607 != 0.0)) && (var_guard608 != 0.0)) {
        let assign30290_e38073: f64 = (var_vav * var_vbrinvbot_d);
        let assign30290_e38076: f64 = (var_vav * var_vbrinvbot_d);
        let assign30290_e38077: f64 = (assign30290_e38073 * assign30290_e38076);
        let assign30290_e38080: f64 = (var_vav * var_vbrinvbot_d);
        let assign30290_e38081: f64 = (assign30290_e38077 * assign30290_e38080);
        let assign30290_e38084: f64 = (var_vav * var_vbrinvbot_d);
        let assign30290_e38085: f64 = (assign30290_e38081 * assign30290_e38084);
        (assign30290_e38085, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30290_e38087;
        var_tmp_dn6 = assign30290_e38087_d_n6;
        var_tmp_dn7 = assign30290_e38087_d_n7;
        var_tmp_dn8 = assign30290_e38087_d_n8;
        var_tmp_dn9 = assign30290_e38087_d_n9;

        let (assign30300_e38109, assign30300_e38109_d_n6, assign30300_e38109_d_n7, assign30300_e38109_d_n8, assign30300_e38109_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard606 == 0.0)) && (var_guard607 != 0.0)) && (var_guard608 == 0.0)) {
        let assign30300_e38104: f64 = (var_vav * var_vbrinvbot_d);
        let assign30300_e38105: f64 = (assign30300_e38104).abs();
        let assign30300_e38107: f64 = (assign30300_e38105).powf(var_pbrbotd_i);
        (assign30300_e38107, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30300_e38109;
        var_tmp_dn6 = assign30300_e38109_d_n6;
        var_tmp_dn7 = assign30300_e38109_d_n7;
        var_tmp_dn8 = assign30300_e38109_d_n8;
        var_tmp_dn9 = assign30300_e38109_d_n9;

        let (assign30310_e38127, assign30310_e38127_d_n6, assign30310_e38127_d_n7, assign30310_e38127_d_n8, assign30310_e38127_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard606 == 0.0)) && (var_guard607 != 0.0)) {
        let assign30310_e38124: f64 = (1.0 - var_tmp);
        let assign30310_e38125: f64 = (1.0 / assign30310_e38124);
        (assign30310_e38125, (-((-var_tmp_dn6) / (assign30310_e38124 * assign30310_e38124))), (-((-var_tmp_dn7) / (assign30310_e38124 * assign30310_e38124))), (-((-var_tmp_dn8) / (assign30310_e38124 * assign30310_e38124))), (-((-var_tmp_dn9) / (assign30310_e38124 * assign30310_e38124))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign30310_e38127;
        var_fbreakdown_dn6 = assign30310_e38127_d_n6;
        var_fbreakdown_dn7 = assign30310_e38127_d_n7;
        var_fbreakdown_dn8 = assign30310_e38127_d_n8;
        var_fbreakdown_dn9 = assign30310_e38127_d_n9;

        let (assign30320_e38150, assign30320_e38150_d_n6, assign30320_e38150_d_n7, assign30320_e38150_d_n8, assign30320_e38150_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) && (var_guard606 == 0.0)) && (var_guard607 == 0.0)) {
        let assign30320_e38144: f64 = (var_alphaav * var_vbrbotd_i);
        let assign30320_e38145: f64 = (var_vav + assign30320_e38144);
        let assign30320_e38147: f64 = (assign30320_e38145 * var_slopebot_d);
        let assign30320_e38148: f64 = (var_fstopbot_d + assign30320_e38147);
        (assign30320_e38148, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign30320_e38150;
        var_fbreakdown_dn6 = assign30320_e38150_d_n6;
        var_fbreakdown_dn7 = assign30320_e38150_d_n7;
        var_fbreakdown_dn8 = assign30320_e38150_d_n8;
        var_fbreakdown_dn9 = assign30320_e38150_d_n9;

        let (assign30330_e38169, assign30330_e38169_d_n6, assign30330_e38169_d_n7, assign30330_e38169_d_n8, assign30330_e38169_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard592 == 0.0)) {
        let assign30330_e38160: f64 = (var_id__blk212 + var_isrh);
        let assign30330_e38162: f64 = (assign30330_e38160 + var_itat);
        let assign30330_e38164: f64 = (assign30330_e38162 + var_ibbt);
        let assign30330_e38165: f64 = (p.p29 * assign30330_e38164);
        let assign30330_e38167: f64 = (assign30330_e38165 * var_fbreakdown);
        (assign30330_e38167, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign30330_e38165 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign30330_e38165 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign30330_e38165 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign30330_e38165 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign30330_e38169;
        var_ijunbot_dn6 = assign30330_e38169_d_n6;
        var_ijunbot_dn7 = assign30330_e38169_d_n7;
        var_ijunbot_dn8 = assign30330_e38169_d_n8;
        var_ijunbot_dn9 = assign30330_e38169_d_n9;

        let assign30340_e38172: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard609 = assign30340_e38172;

        let (assign30350_e38180, assign30350_e38180_d_n6, assign30350_e38180_d_n7, assign30350_e38180_d_n8, assign30350_e38180_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign30350_e38180;
        var_ijunsti_dn6 = assign30350_e38180_d_n6;
        var_ijunsti_dn7 = assign30350_e38180_d_n7;
        var_ijunsti_dn8 = assign30350_e38180_d_n8;
        var_ijunsti_dn9 = assign30350_e38180_d_n9;

        let (assign30360_e38191,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) {
        let assign30360_e38189: f64 = (var_idsatsti_d * var_idmult);
        (assign30360_e38189,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign30360_e38191;

        let assign30370_e38198: f64 = if ((var_csrhstid_i == 0.0) && (var_ctatstid_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard610 = assign30370_e38198;

        let (assign30380_e38209, assign30380_e38209_d_n6, assign30380_e38209_d_n7, assign30380_e38209_d_n8, assign30380_e38209_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard610 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign30380_e38209;
        var_isrh_dn6 = assign30380_e38209_d_n6;
        var_isrh_dn7 = assign30380_e38209_d_n7;
        var_isrh_dn8 = assign30380_e38209_d_n8;
        var_isrh_dn9 = assign30380_e38209_d_n9;

        let (assign30390_e38223,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard610 == 0.0)) {
        let assign30390_e38221: f64 = (var_vbisti_d - var_vjsrh);
        (assign30390_e38221,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign30390_e38223;

        let (assign30400_e38242,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard610 == 0.0)) {
        let assign30400_e38237: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign30400_e38238: f64 = (1.0 - assign30400_e38237);
        let assign30400_e38239: f64 = (assign30400_e38238).sqrt();
        let assign30400_e38240: f64 = (1.0 - assign30400_e38239);
        (assign30400_e38240,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign30400_e38242;

        let assign30410_e38245: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard611 = assign30410_e38245;

        let (assign30420_e38259,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard610 == 0.0)) && (var_guard611 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign30420_e38259;

        let (assign30430_e38291,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard610 == 0.0)) && (var_guard611 == 0.0)) {
        let assign30430_e38274: f64 = (var_wsrhstep * var_wsrhstep);
        let assign30430_e38276: f64 = (var_wsrhstep).ln();
        let assign30430_e38277: f64 = (assign30430_e38274 * assign30430_e38276);
        let assign30430_e38280: f64 = (1.0 - var_wsrhstep);
        let assign30430_e38281: f64 = (assign30430_e38277 / assign30430_e38280);
        let assign30430_e38283: f64 = (assign30430_e38281 + var_wsrhstep);
        let assign30430_e38287: f64 = (2.0 * var_pstid_i);
        let assign30430_e38288: f64 = (1.0 - assign30430_e38287);
        let assign30430_e38289: f64 = (assign30430_e38283 * assign30430_e38288);
        (assign30430_e38289,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign30430_e38291;

        let (assign30440_e38305,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard610 == 0.0)) {
        let assign30440_e38303: f64 = (var_wsrhstep + var_dwsrh);
        (assign30440_e38303,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign30440_e38305;

        let assign30450_e38308: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard612 = assign30450_e38308;

        let (assign30460_e38325, assign30460_e38325_d_n6, assign30460_e38325_d_n7, assign30460_e38325_d_n8, assign30460_e38325_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard610 == 0.0)) && (var_guard612 != 0.0)) {
        let assign30460_e38322: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign30460_e38323: f64 = (assign30460_e38322).sqrt();
        (assign30460_e38323, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30460_e38325;
        var_tmp_dn6 = assign30460_e38325_d_n6;
        var_tmp_dn7 = assign30460_e38325_d_n7;
        var_tmp_dn8 = assign30460_e38325_d_n8;
        var_tmp_dn9 = assign30460_e38325_d_n9;

        let (assign30470_e38344, assign30470_e38344_d_n6, assign30470_e38344_d_n7, assign30470_e38344_d_n8, assign30470_e38344_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard610 == 0.0)) && (var_guard612 == 0.0)) {
        let assign30470_e38340: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv_d);
        let assign30470_e38342: f64 = (assign30470_e38340).powf(var_pstid_i);
        (assign30470_e38342, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30470_e38344;
        var_tmp_dn6 = assign30470_e38344_d_n6;
        var_tmp_dn7 = assign30470_e38344_d_n7;
        var_tmp_dn8 = assign30470_e38344_d_n8;
        var_tmp_dn9 = assign30470_e38344_d_n9;

        let (assign30480_e38358, assign30480_e38358_d_n6, assign30480_e38358_d_n7, assign30480_e38358_d_n8, assign30480_e38358_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard610 == 0.0)) {
        let assign30480_e38356: f64 = (var_wdepnulrsti_d * var_tmp);
        (assign30480_e38356, (var_wdepnulrsti_d * var_tmp_dn6), (var_wdepnulrsti_d * var_tmp_dn7), (var_wdepnulrsti_d * var_tmp_dn8), (var_wdepnulrsti_d * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign30480_e38358;
        var_wdep_dn6 = assign30480_e38358_d_n6;
        var_wdep_dn7 = assign30480_e38358_d_n7;
        var_wdep_dn8 = assign30480_e38358_d_n8;
        var_wdep_dn9 = assign30480_e38358_d_n9;

        let (assign30490_e38376, assign30490_e38376_d_n6, assign30490_e38376_d_n7, assign30490_e38376_d_n8, assign30490_e38376_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard610 == 0.0)) {
        let assign30490_e38371: f64 = (var_zinv - 1.0);
        let assign30490_e38373: f64 = (assign30490_e38371 * var_wdep);
        let assign30490_e38374: f64 = (var_ftdsti_d * assign30490_e38373);
        (assign30490_e38374, (var_ftdsti_d * (assign30490_e38371 * var_wdep_dn6)), (var_ftdsti_d * (assign30490_e38371 * var_wdep_dn7)), (var_ftdsti_d * (assign30490_e38371 * var_wdep_dn8)), (var_ftdsti_d * (assign30490_e38371 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign30490_e38376;
        var_asrh_dn6 = assign30490_e38376_d_n6;
        var_asrh_dn7 = assign30490_e38376_d_n7;
        var_asrh_dn8 = assign30490_e38376_d_n8;
        var_asrh_dn9 = assign30490_e38376_d_n9;

        let (assign30500_e38392, assign30500_e38392_d_n6, assign30500_e38392_d_n7, assign30500_e38392_d_n8, assign30500_e38392_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard610 == 0.0)) {
        let assign30500_e38389: f64 = (var_asrh * var_wsrh);
        let assign30500_e38390: f64 = (var_csrhstid_i * assign30500_e38389);
        (assign30500_e38390, (var_csrhstid_i * (var_asrh_dn6 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn7 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn8 * var_wsrh)), (var_csrhstid_i * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign30500_e38392;
        var_isrh_dn6 = assign30500_e38392_d_n6;
        var_isrh_dn7 = assign30500_e38392_d_n7;
        var_isrh_dn8 = assign30500_e38392_d_n8;
        var_isrh_dn9 = assign30500_e38392_d_n9;

        let assign30510_e38395: f64 = if var_ctatstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard613 = assign30510_e38395;

        let (assign30520_e38406, assign30520_e38406_d_n6, assign30520_e38406_d_n7, assign30520_e38406_d_n8, assign30520_e38406_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign30520_e38406;
        var_itat_dn6 = assign30520_e38406_d_n6;
        var_itat_dn7 = assign30520_e38406_d_n7;
        var_itat_dn8 = assign30520_e38406_d_n8;
        var_itat_dn9 = assign30520_e38406_d_n9;

        let (assign30530_e38424, assign30530_e38424_d_n6, assign30530_e38424_d_n7, assign30530_e38424_d_n8, assign30530_e38424_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) {
        let assign30530_e38419: f64 = (var_wdep * var_one_minus_psti_d);
        let assign30530_e38421: f64 = (assign30530_e38419 / var_vbi_minus_vjsrh);
        let assign30530_e38422: f64 = (var_btatpartsti_d * assign30530_e38421);
        (assign30530_e38422, (var_btatpartsti_d * ((var_wdep_dn6 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn7 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn8 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)), (var_btatpartsti_d * ((var_wdep_dn9 * var_one_minus_psti_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign30530_e38424;
        var_btat_dn6 = assign30530_e38424_d_n6;
        var_btat_dn7 = assign30530_e38424_d_n7;
        var_btat_dn8 = assign30530_e38424_d_n8;
        var_btat_dn9 = assign30530_e38424_d_n9;

        let (assign30540_e38440, assign30540_e38440_d_n6, assign30540_e38440_d_n7, assign30540_e38440_d_n8, assign30540_e38440_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) {
        let assign30540_e38436: f64 = (0.666666666666667 * var_atatsti_d);
        let assign30540_e38438: f64 = (assign30540_e38436 / var_btat);
        (assign30540_e38438, (-((assign30540_e38436 * var_btat_dn6) / (var_btat * var_btat))), (-((assign30540_e38436 * var_btat_dn7) / (var_btat * var_btat))), (-((assign30540_e38436 * var_btat_dn8) / (var_btat * var_btat))), (-((assign30540_e38436 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign30540_e38440;
        var_twoatatoverthreebtat_dn6 = assign30540_e38440_d_n6;
        var_twoatatoverthreebtat_dn7 = assign30540_e38440_d_n7;
        var_twoatatoverthreebtat_dn8 = assign30540_e38440_d_n8;
        var_twoatatoverthreebtat_dn9 = assign30540_e38440_d_n9;

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
        *var_guard613_slot = var_guard613;
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
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_twoatatoverthreebtat_dn9_slot = var_twoatatoverthreebtat_dn9;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wdep_dn9_slot = var_wdep_dn9;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
    }

    pub(super) fn stamp_transient_block_62(
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatsti_d: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_btat_dn9: f64,
        var_cbbtstid_i: f64,
        var_cerfc: f64,
        var_ctatstid_i: f64,
        var_fbbtsti_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard609: f64,
        var_guard613: f64,
        var_one_over_one_minus_psti_d: f64,
        var_perfc: f64,
        var_pstid_i: f64,
        var_twoatatoverthreebtat: f64,
        var_twoatatoverthreebtat_dn6: f64,
        var_twoatatoverthreebtat_dn7: f64,
        var_twoatatoverthreebtat_dn8: f64,
        var_twoatatoverthreebtat_dn9: f64,
        var_vbbt: f64,
        var_vbirstid_i: f64,
        var_vbirstiinv_d: f64,
        var_wdepnulrinvsti_d: f64,
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
        var_guard614_slot: &mut f64,
        var_guard615_slot: &mut f64,
        var_guard616_slot: &mut f64,
        var_guard617_slot: &mut f64,
        var_guard618_slot: &mut f64,
        var_guard619_slot: &mut f64,
        var_guard620_slot: &mut f64,
        var_guard621_slot: &mut f64,
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
        let mut var_guard614: f64 = *var_guard614_slot;
        let mut var_guard615: f64 = *var_guard615_slot;
        let mut var_guard616: f64 = *var_guard616_slot;
        let mut var_guard617: f64 = *var_guard617_slot;
        let mut var_guard618: f64 = *var_guard618_slot;
        let mut var_guard619: f64 = *var_guard619_slot;
        let mut var_guard620: f64 = *var_guard620_slot;
        let mut var_guard621: f64 = *var_guard621_slot;
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

        let (assign30550_e38454, assign30550_e38454_d_n6, assign30550_e38454_d_n7, assign30550_e38454_d_n8, assign30550_e38454_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) {
        let assign30550_e38452: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign30550_e38452, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign30550_e38454;
        var_umaxbeforelimiting_dn6 = assign30550_e38454_d_n6;
        var_umaxbeforelimiting_dn7 = assign30550_e38454_d_n7;
        var_umaxbeforelimiting_dn8 = assign30550_e38454_d_n8;
        var_umaxbeforelimiting_dn9 = assign30550_e38454_d_n9;

        let (assign30560_e38475, assign30560_e38475_d_n6, assign30560_e38475_d_n7, assign30560_e38475_d_n8, assign30560_e38475_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) {
        let assign30560_e38466: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign30560_e38469: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign30560_e38471: f64 = (assign30560_e38469 + 1.0);
        let assign30560_e38472: f64 = (assign30560_e38466 / assign30560_e38471);
        let assign30560_e38473: f64 = (assign30560_e38472).sqrt();
        (assign30560_e38473, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign30560_e38471) - (assign30560_e38466 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign30560_e38471 * assign30560_e38471)) / (2.0 * assign30560_e38473)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign30560_e38471) - (assign30560_e38466 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign30560_e38471 * assign30560_e38471)) / (2.0 * assign30560_e38473)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign30560_e38471) - (assign30560_e38466 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign30560_e38471 * assign30560_e38471)) / (2.0 * assign30560_e38473)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign30560_e38471) - (assign30560_e38466 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign30560_e38471 * assign30560_e38471)) / (2.0 * assign30560_e38473)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign30560_e38475;
        var_umax_dn6 = assign30560_e38475_d_n6;
        var_umax_dn7 = assign30560_e38475_d_n7;
        var_umax_dn8 = assign30560_e38475_d_n8;
        var_umax_dn9 = assign30560_e38475_d_n9;

        let (assign30570_e38488, assign30570_e38488_d_n6, assign30570_e38488_d_n7, assign30570_e38488_d_n8, assign30570_e38488_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) {
        let assign30570_e38486: f64 = (var_umax).sqrt();
        (assign30570_e38486, (var_umax_dn6 / (2.0 * assign30570_e38486)), (var_umax_dn7 / (2.0 * assign30570_e38486)), (var_umax_dn8 / (2.0 * assign30570_e38486)), (var_umax_dn9 / (2.0 * assign30570_e38486)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign30570_e38488;
        var_sqrtumax_dn6 = assign30570_e38488_d_n6;
        var_sqrtumax_dn7 = assign30570_e38488_d_n7;
        var_sqrtumax_dn8 = assign30570_e38488_d_n8;
        var_sqrtumax_dn9 = assign30570_e38488_d_n9;

        let (assign30580_e38502, assign30580_e38502_d_n6, assign30580_e38502_d_n7, assign30580_e38502_d_n8, assign30580_e38502_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) {
        let assign30580_e38500: f64 = (var_umax * var_sqrtumax);
        (assign30580_e38500, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign30580_e38502;
        var_umaxpoweronepointfive_dn6 = assign30580_e38502_d_n6;
        var_umaxpoweronepointfive_dn7 = assign30580_e38502_d_n7;
        var_umaxpoweronepointfive_dn8 = assign30580_e38502_d_n8;
        var_umaxpoweronepointfive_dn9 = assign30580_e38502_d_n9;

        let assign30590_e38504: f64 = (-var_pstid_i);
        let assign30590_e38506: f64 = (assign30590_e38504 * var_one_over_one_minus_psti_d);
        let assign30590_e38508: f64 = (-1.0);
        let assign30590_e38509: f64 = if assign30590_e38506 == assign30590_e38508 { 1.0 } else { 0.0 };
        var_guard614 = assign30590_e38509;

        let (assign30600_e38529, assign30600_e38529_d_n6, assign30600_e38529_d_n7, assign30600_e38529_d_n8, assign30600_e38529_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) && (var_guard614 != 0.0)) {
        let assign30600_e38525: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign30600_e38526: f64 = (1.0 + assign30600_e38525);
        let assign30600_e38527: f64 = (1.0 / assign30600_e38526);
        (assign30600_e38527, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign30600_e38526 * assign30600_e38526))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign30600_e38526 * assign30600_e38526))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign30600_e38526 * assign30600_e38526))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign30600_e38526 * assign30600_e38526))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign30600_e38529;
        var_wgamma_dn6 = assign30600_e38529_d_n6;
        var_wgamma_dn7 = assign30600_e38529_d_n7;
        var_wgamma_dn8 = assign30600_e38529_d_n8;
        var_wgamma_dn9 = assign30600_e38529_d_n9;

        let (assign30610_e38553, assign30610_e38553_d_n6, assign30610_e38553_d_n7, assign30610_e38553_d_n8, assign30610_e38553_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) && (var_guard614 == 0.0)) {
        let assign30610_e38545: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign30610_e38546: f64 = (1.0 + assign30610_e38545);
        let assign30610_e38548: f64 = (-var_pstid_i);
        let assign30610_e38550: f64 = (assign30610_e38548 * var_one_over_one_minus_psti_d);
        let assign30610_e38551: f64 = (assign30610_e38546).powf(assign30610_e38550);
        (assign30610_e38551, if 0.0 == 0.0 && ((assign30610_e38550) as f64).is_finite() && ((assign30610_e38550) as f64).fract() == 0.0 { if assign30610_e38550 == 0.0 { 0.0 } else { (assign30610_e38550 * ((assign30610_e38546).powf(assign30610_e38550 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign30610_e38551 * (assign30610_e38550 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign30610_e38546))) }, if 0.0 == 0.0 && ((assign30610_e38550) as f64).is_finite() && ((assign30610_e38550) as f64).fract() == 0.0 { if assign30610_e38550 == 0.0 { 0.0 } else { (assign30610_e38550 * ((assign30610_e38546).powf(assign30610_e38550 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign30610_e38551 * (assign30610_e38550 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign30610_e38546))) }, if 0.0 == 0.0 && ((assign30610_e38550) as f64).is_finite() && ((assign30610_e38550) as f64).fract() == 0.0 { if assign30610_e38550 == 0.0 { 0.0 } else { (assign30610_e38550 * ((assign30610_e38546).powf(assign30610_e38550 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign30610_e38551 * (assign30610_e38550 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign30610_e38546))) }, if 0.0 == 0.0 && ((assign30610_e38550) as f64).is_finite() && ((assign30610_e38550) as f64).fract() == 0.0 { if assign30610_e38550 == 0.0 { 0.0 } else { (assign30610_e38550 * ((assign30610_e38546).powf(assign30610_e38550 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign30610_e38551 * (assign30610_e38550 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign30610_e38546))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign30610_e38553;
        var_wgamma_dn6 = assign30610_e38553_d_n6;
        var_wgamma_dn7 = assign30610_e38553_d_n7;
        var_wgamma_dn8 = assign30610_e38553_d_n8;
        var_wgamma_dn9 = assign30610_e38553_d_n9;

        let (assign30620_e38571, assign30620_e38571_d_n6, assign30620_e38571_d_n7, assign30620_e38571_d_n8, assign30620_e38571_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) {
        let assign30620_e38565: f64 = (var_wsrh * var_wgamma);
        let assign30620_e38568: f64 = (var_wsrh + var_wgamma);
        let assign30620_e38569: f64 = (assign30620_e38565 / assign30620_e38568);
        (assign30620_e38569, ((((var_wsrh * var_wgamma_dn6) * assign30620_e38568) - (assign30620_e38565 * var_wgamma_dn6)) / (assign30620_e38568 * assign30620_e38568)), ((((var_wsrh * var_wgamma_dn7) * assign30620_e38568) - (assign30620_e38565 * var_wgamma_dn7)) / (assign30620_e38568 * assign30620_e38568)), ((((var_wsrh * var_wgamma_dn8) * assign30620_e38568) - (assign30620_e38565 * var_wgamma_dn8)) / (assign30620_e38568 * assign30620_e38568)), ((((var_wsrh * var_wgamma_dn9) * assign30620_e38568) - (assign30620_e38565 * var_wgamma_dn9)) / (assign30620_e38568 * assign30620_e38568)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign30620_e38571;
        var_wtat_dn6 = assign30620_e38571_d_n6;
        var_wtat_dn7 = assign30620_e38571_d_n7;
        var_wtat_dn8 = assign30620_e38571_d_n8;
        var_wtat_dn9 = assign30620_e38571_d_n9;

        let (assign30630_e38588, assign30630_e38588_d_n6, assign30630_e38588_d_n7, assign30630_e38588_d_n8, assign30630_e38588_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) {
        let assign30630_e38584: f64 = (var_btat / var_sqrtumax);
        let assign30630_e38585: f64 = (0.375 * assign30630_e38584);
        let assign30630_e38586: f64 = (assign30630_e38585).sqrt();
        (assign30630_e38586, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign30630_e38586)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign30630_e38586)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign30630_e38586)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign30630_e38586)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign30630_e38588;
        var_ktat_dn6 = assign30630_e38588_d_n6;
        var_ktat_dn7 = assign30630_e38588_d_n7;
        var_ktat_dn8 = assign30630_e38588_d_n8;
        var_ktat_dn9 = assign30630_e38588_d_n9;

        let (assign30640_e38606, assign30640_e38606_d_n6, assign30640_e38606_d_n7, assign30640_e38606_d_n8, assign30640_e38606_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) {
        let assign30640_e38601: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign30640_e38602: f64 = (2.0 * assign30640_e38601);
        let assign30640_e38604: f64 = (assign30640_e38602 - var_umax);
        (assign30640_e38604, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign30640_e38606;
        var_ltat_dn6 = assign30640_e38606_d_n6;
        var_ltat_dn7 = assign30640_e38606_d_n7;
        var_ltat_dn8 = assign30640_e38606_d_n8;
        var_ltat_dn9 = assign30640_e38606_d_n9;

        let (assign30650_e38632, assign30650_e38632_d_n6, assign30650_e38632_d_n7, assign30650_e38632_d_n8, assign30650_e38632_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) {
        let assign30650_e38618: f64 = (var_atatsti_d * var_twoatatoverthreebtat);
        let assign30650_e38620: f64 = (assign30650_e38618 * var_sqrtumax);
        let assign30650_e38623: f64 = (var_atatsti_d * var_umax);
        let assign30650_e38624: f64 = (assign30650_e38620 - assign30650_e38623);
        let assign30650_e38628: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign30650_e38629: f64 = (0.5 * assign30650_e38628);
        let assign30650_e38630: f64 = (assign30650_e38624 + assign30650_e38629);
        (assign30650_e38630, (((((var_atatsti_d * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign30650_e38618 * var_sqrtumax_dn6)) - (var_atatsti_d * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign30650_e38618 * var_sqrtumax_dn7)) - (var_atatsti_d * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign30650_e38618 * var_sqrtumax_dn8)) - (var_atatsti_d * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatsti_d * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign30650_e38618 * var_sqrtumax_dn9)) - (var_atatsti_d * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign30650_e38632;
        var_mtat_dn6 = assign30650_e38632_d_n6;
        var_mtat_dn7 = assign30650_e38632_d_n7;
        var_mtat_dn8 = assign30650_e38632_d_n8;
        var_mtat_dn9 = assign30650_e38632_d_n9;

        let (assign30660_e38648, assign30660_e38648_d_n6, assign30660_e38648_d_n7, assign30660_e38648_d_n8, assign30660_e38648_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) {
        let assign30660_e38644: f64 = (var_ltat - 1.0);
        let assign30660_e38646: f64 = (assign30660_e38644 * var_ktat);
        (assign30660_e38646, ((var_ltat_dn6 * var_ktat) + (assign30660_e38644 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign30660_e38644 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign30660_e38644 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign30660_e38644 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign30660_e38648;
        var_xerfc_dn6 = assign30660_e38648_d_n6;
        var_xerfc_dn7 = assign30660_e38648_d_n7;
        var_xerfc_dn8 = assign30660_e38648_d_n8;
        var_xerfc_dn9 = assign30660_e38648_d_n9;

        let (assign30670_e38662, assign30670_e38662_d_n6, assign30670_e38662_d_n7, assign30670_e38662_d_n8, assign30670_e38662_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) {
        let assign30670_e38660: f64 = (var_xerfc * var_xerfc);
        (assign30670_e38660, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign30670_e38662;
        var_ysq_dn6 = assign30670_e38662_d_n6;
        var_ysq_dn7 = assign30670_e38662_d_n7;
        var_ysq_dn8 = assign30670_e38662_d_n8;
        var_ysq_dn9 = assign30670_e38662_d_n9;

        let assign30680_e38665: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard615 = assign30680_e38665;

        let (assign30690_e38685, assign30690_e38685_d_n6, assign30690_e38685_d_n7, assign30690_e38685_d_n8, assign30690_e38685_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) && (var_guard615 != 0.0)) {
        let assign30690_e38681: f64 = (var_perfc * var_xerfc);
        let assign30690_e38682: f64 = (1.0 + assign30690_e38681);
        let assign30690_e38683: f64 = (1.0 / assign30690_e38682);
        (assign30690_e38683, (-((var_perfc * var_xerfc_dn6) / (assign30690_e38682 * assign30690_e38682))), (-((var_perfc * var_xerfc_dn7) / (assign30690_e38682 * assign30690_e38682))), (-((var_perfc * var_xerfc_dn8) / (assign30690_e38682 * assign30690_e38682))), (-((var_perfc * var_xerfc_dn9) / (assign30690_e38682 * assign30690_e38682))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign30690_e38685;
        var_terfc_dn6 = assign30690_e38685_d_n6;
        var_terfc_dn7 = assign30690_e38685_d_n7;
        var_terfc_dn8 = assign30690_e38685_d_n8;
        var_terfc_dn9 = assign30690_e38685_d_n9;

        let (assign30700_e38706, assign30700_e38706_d_n6, assign30700_e38706_d_n7, assign30700_e38706_d_n8, assign30700_e38706_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) && (var_guard615 == 0.0)) {
        let assign30700_e38702: f64 = (var_perfc * var_xerfc);
        let assign30700_e38703: f64 = (1.0 - assign30700_e38702);
        let assign30700_e38704: f64 = (1.0 / assign30700_e38703);
        (assign30700_e38704, (-((-(var_perfc * var_xerfc_dn6)) / (assign30700_e38703 * assign30700_e38703))), (-((-(var_perfc * var_xerfc_dn7)) / (assign30700_e38703 * assign30700_e38703))), (-((-(var_perfc * var_xerfc_dn8)) / (assign30700_e38703 * assign30700_e38703))), (-((-(var_perfc * var_xerfc_dn9)) / (assign30700_e38703 * assign30700_e38703))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign30700_e38706;
        var_terfc_dn6 = assign30700_e38706_d_n6;
        var_terfc_dn7 = assign30700_e38706_d_n7;
        var_terfc_dn8 = assign30700_e38706_d_n8;
        var_terfc_dn9 = assign30700_e38706_d_n9;

        let assign30710_e38708: f64 = (-var_ysq);
        let assign30710_e38710: f64 = (assign30710_e38708 + var_mtat);
        let assign30710_e38712: f64 = (-230.25850929940458);
        let assign30710_e38713: f64 = if assign30710_e38710 > assign30710_e38712 { 1.0 } else { 0.0 };
        var_guard616 = assign30710_e38713;

        let (assign30720_e38731, assign30720_e38731_d_n6, assign30720_e38731_d_n7, assign30720_e38731_d_n8, assign30720_e38731_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) && (var_guard616 != 0.0)) {
        let assign30720_e38726: f64 = (-var_ysq);
        let assign30720_e38728: f64 = (assign30720_e38726 + var_mtat);
        let assign30720_e38729: f64 = (assign30720_e38728).exp();
        (assign30720_e38729, (assign30720_e38729 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign30720_e38729 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign30720_e38729 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign30720_e38729 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30720_e38731;
        var_tmp_dn6 = assign30720_e38731_d_n6;
        var_tmp_dn7 = assign30720_e38731_d_n7;
        var_tmp_dn8 = assign30720_e38731_d_n8;
        var_tmp_dn9 = assign30720_e38731_d_n9;

        let (assign30730_e38780, assign30730_e38780_d_n6, assign30730_e38780_d_n7, assign30730_e38780_d_n8, assign30730_e38780_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) && (var_guard616 == 0.0)) {
        let assign30730_e38747: f64 = (-230.25850929940458);
        let assign30730_e38749: f64 = (-var_ysq);
        let assign30730_e38751: f64 = (assign30730_e38749 + var_mtat);
        let assign30730_e38752: f64 = (assign30730_e38747 - assign30730_e38751);
        let assign30730_e38756: f64 = (-230.25850929940458);
        let assign30730_e38758: f64 = (-var_ysq);
        let assign30730_e38760: f64 = (assign30730_e38758 + var_mtat);
        let assign30730_e38761: f64 = (assign30730_e38756 - assign30730_e38760);
        let assign30730_e38764: f64 = (-230.25850929940458);
        let assign30730_e38766: f64 = (-var_ysq);
        let assign30730_e38768: f64 = (assign30730_e38766 + var_mtat);
        let assign30730_e38769: f64 = (assign30730_e38764 - assign30730_e38768);
        let assign30730_e38771: f64 = (assign30730_e38769 * 0.3333333333333333);
        let assign30730_e38772: f64 = (1.0 + assign30730_e38771);
        let assign30730_e38773: f64 = (assign30730_e38761 * assign30730_e38772);
        let assign30730_e38774: f64 = (0.5 * assign30730_e38773);
        let assign30730_e38775: f64 = (1.0 + assign30730_e38774);
        let assign30730_e38776: f64 = (assign30730_e38752 * assign30730_e38775);
        let assign30730_e38777: f64 = (1.0 + assign30730_e38776);
        let assign30730_e38778: f64 = (1e-100 / assign30730_e38777);
        (assign30730_e38778, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign30730_e38775) + (assign30730_e38752 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign30730_e38772) + (assign30730_e38761 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign30730_e38777 * assign30730_e38777))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign30730_e38775) + (assign30730_e38752 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign30730_e38772) + (assign30730_e38761 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign30730_e38777 * assign30730_e38777))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign30730_e38775) + (assign30730_e38752 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign30730_e38772) + (assign30730_e38761 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign30730_e38777 * assign30730_e38777))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign30730_e38775) + (assign30730_e38752 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign30730_e38772) + (assign30730_e38761 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign30730_e38777 * assign30730_e38777))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30730_e38780;
        var_tmp_dn6 = assign30730_e38780_d_n6;
        var_tmp_dn7 = assign30730_e38780_d_n7;
        var_tmp_dn8 = assign30730_e38780_d_n8;
        var_tmp_dn9 = assign30730_e38780_d_n9;

        let (assign30740_e38810, assign30740_e38810_d_n6, assign30740_e38810_d_n7, assign30740_e38810_d_n8, assign30740_e38810_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) {
        let assign30740_e38792: f64 = (0.29214664 * var_terfc);
        let assign30740_e38796: f64 = (var_terfc * var_terfc);
        let assign30740_e38797: f64 = (var_berfc * assign30740_e38796);
        let assign30740_e38798: f64 = (assign30740_e38792 + assign30740_e38797);
        let assign30740_e38802: f64 = (var_terfc * var_terfc);
        let assign30740_e38804: f64 = (assign30740_e38802 * var_terfc);
        let assign30740_e38805: f64 = (var_cerfc * assign30740_e38804);
        let assign30740_e38806: f64 = (assign30740_e38798 + assign30740_e38805);
        let assign30740_e38808: f64 = (assign30740_e38806 * var_tmp);
        (assign30740_e38808, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign30740_e38802 * var_terfc_dn6)))) * var_tmp) + (assign30740_e38806 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign30740_e38802 * var_terfc_dn7)))) * var_tmp) + (assign30740_e38806 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign30740_e38802 * var_terfc_dn8)))) * var_tmp) + (assign30740_e38806 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign30740_e38802 * var_terfc_dn9)))) * var_tmp) + (assign30740_e38806 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign30740_e38810;
        var_erfcpos_dn6 = assign30740_e38810_d_n6;
        var_erfcpos_dn7 = assign30740_e38810_d_n7;
        var_erfcpos_dn8 = assign30740_e38810_d_n8;
        var_erfcpos_dn9 = assign30740_e38810_d_n9;

        let assign30750_e38813: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard617 = assign30750_e38813;

        let (assign30760_e38827, assign30760_e38827_d_n6, assign30760_e38827_d_n7, assign30760_e38827_d_n8, assign30760_e38827_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) && (var_guard617 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign30760_e38827;
        var_erfctimesexpmtat_dn6 = assign30760_e38827_d_n6;
        var_erfctimesexpmtat_dn7 = assign30760_e38827_d_n7;
        var_erfctimesexpmtat_dn8 = assign30760_e38827_d_n8;
        var_erfctimesexpmtat_dn9 = assign30760_e38827_d_n9;

        let assign30770_e38830: f64 = (-230.25850929940458);
        let assign30770_e38831: f64 = if var_mtat > assign30770_e38830 { 1.0 } else { 0.0 };
        var_guard618 = assign30770_e38831;

        let (assign30780_e38849, assign30780_e38849_d_n6, assign30780_e38849_d_n7, assign30780_e38849_d_n8, assign30780_e38849_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) && (var_guard617 == 0.0)) && (var_guard618 != 0.0)) {
        let assign30780_e38847: f64 = (var_mtat).exp();
        (assign30780_e38847, (assign30780_e38847 * var_mtat_dn6), (assign30780_e38847 * var_mtat_dn7), (assign30780_e38847 * var_mtat_dn8), (assign30780_e38847 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30780_e38849;
        var_tmp_dn6 = assign30780_e38849_d_n6;
        var_tmp_dn7 = assign30780_e38849_d_n7;
        var_tmp_dn8 = assign30780_e38849_d_n8;
        var_tmp_dn9 = assign30780_e38849_d_n9;

        let (assign30790_e38892, assign30790_e38892_d_n6, assign30790_e38892_d_n7, assign30790_e38892_d_n8, assign30790_e38892_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) && (var_guard617 == 0.0)) && (var_guard618 == 0.0)) {
        let assign30790_e38868: f64 = (-230.25850929940458);
        let assign30790_e38870: f64 = (assign30790_e38868 - var_mtat);
        let assign30790_e38874: f64 = (-230.25850929940458);
        let assign30790_e38876: f64 = (assign30790_e38874 - var_mtat);
        let assign30790_e38879: f64 = (-230.25850929940458);
        let assign30790_e38881: f64 = (assign30790_e38879 - var_mtat);
        let assign30790_e38883: f64 = (assign30790_e38881 * 0.3333333333333333);
        let assign30790_e38884: f64 = (1.0 + assign30790_e38883);
        let assign30790_e38885: f64 = (assign30790_e38876 * assign30790_e38884);
        let assign30790_e38886: f64 = (0.5 * assign30790_e38885);
        let assign30790_e38887: f64 = (1.0 + assign30790_e38886);
        let assign30790_e38888: f64 = (assign30790_e38870 * assign30790_e38887);
        let assign30790_e38889: f64 = (1.0 + assign30790_e38888);
        let assign30790_e38890: f64 = (1e-100 / assign30790_e38889);
        (assign30790_e38890, (-((1e-100 * (((-var_mtat_dn6) * assign30790_e38887) + (assign30790_e38870 * (0.5 * (((-var_mtat_dn6) * assign30790_e38884) + (assign30790_e38876 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign30790_e38889 * assign30790_e38889))), (-((1e-100 * (((-var_mtat_dn7) * assign30790_e38887) + (assign30790_e38870 * (0.5 * (((-var_mtat_dn7) * assign30790_e38884) + (assign30790_e38876 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign30790_e38889 * assign30790_e38889))), (-((1e-100 * (((-var_mtat_dn8) * assign30790_e38887) + (assign30790_e38870 * (0.5 * (((-var_mtat_dn8) * assign30790_e38884) + (assign30790_e38876 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign30790_e38889 * assign30790_e38889))), (-((1e-100 * (((-var_mtat_dn9) * assign30790_e38887) + (assign30790_e38870 * (0.5 * (((-var_mtat_dn9) * assign30790_e38884) + (assign30790_e38876 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign30790_e38889 * assign30790_e38889))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30790_e38892;
        var_tmp_dn6 = assign30790_e38892_d_n6;
        var_tmp_dn7 = assign30790_e38892_d_n7;
        var_tmp_dn8 = assign30790_e38892_d_n8;
        var_tmp_dn9 = assign30790_e38892_d_n9;

        let (assign30800_e38911, assign30800_e38911_d_n6, assign30800_e38911_d_n7, assign30800_e38911_d_n8, assign30800_e38911_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) && (var_guard617 == 0.0)) {
        let assign30800_e38907: f64 = (2.0 * var_tmp);
        let assign30800_e38909: f64 = (assign30800_e38907 - var_erfcpos);
        (assign30800_e38909, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign30800_e38911;
        var_erfctimesexpmtat_dn6 = assign30800_e38911_d_n6;
        var_erfctimesexpmtat_dn7 = assign30800_e38911_d_n7;
        var_erfctimesexpmtat_dn8 = assign30800_e38911_d_n8;
        var_erfctimesexpmtat_dn9 = assign30800_e38911_d_n9;

        let (assign30810_e38931, assign30810_e38931_d_n6, assign30810_e38931_d_n7, assign30810_e38931_d_n8, assign30810_e38931_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) {
        let assign30810_e38923: f64 = (1.772453850905516 * 0.5);
        let assign30810_e38926: f64 = (var_atatsti_d * var_erfctimesexpmtat);
        let assign30810_e38928: f64 = (assign30810_e38926 / var_ktat);
        let assign30810_e38929: f64 = (assign30810_e38923 * assign30810_e38928);
        (assign30810_e38929, (assign30810_e38923 * ((((var_atatsti_d * var_erfctimesexpmtat_dn6) * var_ktat) - (assign30810_e38926 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign30810_e38923 * ((((var_atatsti_d * var_erfctimesexpmtat_dn7) * var_ktat) - (assign30810_e38926 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign30810_e38923 * ((((var_atatsti_d * var_erfctimesexpmtat_dn8) * var_ktat) - (assign30810_e38926 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign30810_e38923 * ((((var_atatsti_d * var_erfctimesexpmtat_dn9) * var_ktat) - (assign30810_e38926 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign30810_e38931;
        var_gammamax_dn6 = assign30810_e38931_d_n6;
        var_gammamax_dn7 = assign30810_e38931_d_n7;
        var_gammamax_dn8 = assign30810_e38931_d_n8;
        var_gammamax_dn9 = assign30810_e38931_d_n9;

        let (assign30820_e38949, assign30820_e38949_d_n6, assign30820_e38949_d_n7, assign30820_e38949_d_n8, assign30820_e38949_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard613 == 0.0)) {
        let assign30820_e38944: f64 = (var_asrh * var_gammamax);
        let assign30820_e38946: f64 = (assign30820_e38944 * var_wtat);
        let assign30820_e38947: f64 = (var_ctatstid_i * assign30820_e38946);
        (assign30820_e38947, (var_ctatstid_i * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign30820_e38944 * var_wtat_dn6))), (var_ctatstid_i * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign30820_e38944 * var_wtat_dn7))), (var_ctatstid_i * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign30820_e38944 * var_wtat_dn8))), (var_ctatstid_i * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign30820_e38944 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign30820_e38949;
        var_itat_dn6 = assign30820_e38949_d_n6;
        var_itat_dn7 = assign30820_e38949_d_n7;
        var_itat_dn8 = assign30820_e38949_d_n8;
        var_itat_dn9 = assign30820_e38949_d_n9;

        let assign30830_e38952: f64 = if var_cbbtstid_i == 0.0 { 1.0 } else { 0.0 };
        var_guard619 = assign30830_e38952;

        let (assign30840_e38963, assign30840_e38963_d_n6, assign30840_e38963_d_n7, assign30840_e38963_d_n8, assign30840_e38963_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard619 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign30840_e38963;
        var_ibbt_dn6 = assign30840_e38963_d_n6;
        var_ibbt_dn7 = assign30840_e38963_d_n7;
        var_ibbt_dn8 = assign30840_e38963_d_n8;
        var_ibbt_dn9 = assign30840_e38963_d_n9;

        let assign30850_e38966: f64 = if var_pstid_i == 0.5 { 1.0 } else { 0.0 };
        var_guard620 = assign30850_e38966;

        let (assign30860_e38985, assign30860_e38985_d_n6, assign30860_e38985_d_n7, assign30860_e38985_d_n8, assign30860_e38985_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard619 == 0.0)) && (var_guard620 != 0.0)) {
        let assign30860_e38980: f64 = (var_vbirstid_i - var_vbbt);
        let assign30860_e38982: f64 = (assign30860_e38980 * var_vbirstiinv_d);
        let assign30860_e38983: f64 = (assign30860_e38982).sqrt();
        (assign30860_e38983, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30860_e38985;
        var_tmp_dn6 = assign30860_e38985_d_n6;
        var_tmp_dn7 = assign30860_e38985_d_n7;
        var_tmp_dn8 = assign30860_e38985_d_n8;
        var_tmp_dn9 = assign30860_e38985_d_n9;

        let (assign30870_e39006, assign30870_e39006_d_n6, assign30870_e39006_d_n7, assign30870_e39006_d_n8, assign30870_e39006_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard619 == 0.0)) && (var_guard620 == 0.0)) {
        let assign30870_e39000: f64 = (var_vbirstid_i - var_vbbt);
        let assign30870_e39002: f64 = (assign30870_e39000 * var_vbirstiinv_d);
        let assign30870_e39004: f64 = (assign30870_e39002).powf(var_pstid_i);
        (assign30870_e39004, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30870_e39006;
        var_tmp_dn6 = assign30870_e39006_d_n6;
        var_tmp_dn7 = assign30870_e39006_d_n7;
        var_tmp_dn8 = assign30870_e39006_d_n8;
        var_tmp_dn9 = assign30870_e39006_d_n9;

        let (assign30880_e39026, assign30880_e39026_d_n6, assign30880_e39026_d_n7, assign30880_e39026_d_n8, assign30880_e39026_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard619 == 0.0)) {
        let assign30880_e39019: f64 = (var_vbirstid_i - var_vbbt);
        let assign30880_e39021: f64 = (assign30880_e39019 * var_wdepnulrinvsti_d);
        let assign30880_e39023: f64 = (assign30880_e39021 / var_tmp);
        let assign30880_e39024: f64 = (var_one_over_one_minus_psti_d * assign30880_e39023);
        (assign30880_e39024, (var_one_over_one_minus_psti_d * (-((assign30880_e39021 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign30880_e39021 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign30880_e39021 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti_d * (-((assign30880_e39021 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign30880_e39026;
        var_fmaxr_dn6 = assign30880_e39026_d_n6;
        var_fmaxr_dn7 = assign30880_e39026_d_n7;
        var_fmaxr_dn8 = assign30880_e39026_d_n8;
        var_fmaxr_dn9 = assign30880_e39026_d_n9;

        let assign30890_e39028: f64 = (-var_fbbtsti_d);
        let assign30890_e39030: f64 = (assign30890_e39028 / var_fmaxr);
        let assign30890_e39031: f64 = (assign30890_e39030).abs();
        let assign30890_e39033: f64 = if assign30890_e39031 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard621 = assign30890_e39033;

        let (assign30900_e39051, assign30900_e39051_d_n6, assign30900_e39051_d_n7, assign30900_e39051_d_n8, assign30900_e39051_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard619 == 0.0)) && (var_guard621 != 0.0)) {
        let assign30900_e39046: f64 = (-var_fbbtsti_d);
        let assign30900_e39048: f64 = (assign30900_e39046 / var_fmaxr);
        let assign30900_e39049: f64 = (assign30900_e39048).exp();
        (assign30900_e39049, (assign30900_e39049 * (-((assign30900_e39046 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign30900_e39049 * (-((assign30900_e39046 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign30900_e39049 * (-((assign30900_e39046 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign30900_e39049 * (-((assign30900_e39046 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30900_e39051;
        var_tmp_dn6 = assign30900_e39051_d_n6;
        var_tmp_dn7 = assign30900_e39051_d_n7;
        var_tmp_dn8 = assign30900_e39051_d_n8;
        var_tmp_dn9 = assign30900_e39051_d_n9;

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
        *var_guard614_slot = var_guard614;
        *var_guard615_slot = var_guard615;
        *var_guard616_slot = var_guard616;
        *var_guard617_slot = var_guard617;
        *var_guard618_slot = var_guard618;
        *var_guard619_slot = var_guard619;
        *var_guard620_slot = var_guard620;
        *var_guard621_slot = var_guard621;
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

    pub(super) fn stamp_transient_block_63(
        p: &Parameters,
        var_alphaav: f64,
        var_atatgat_d: f64,
        var_btatpartgat_d: f64,
        var_cbbtstid_i: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_fbbtsti_d: f64,
        var_fmaxr: f64,
        var_fmaxr_dn6: f64,
        var_fmaxr_dn7: f64,
        var_fmaxr_dn8: f64,
        var_fmaxr_dn9: f64,
        var_fstopsti_d: f64,
        var_ftdgat_d: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard609: f64,
        var_guard619: f64,
        var_guard621: f64,
        var_idmult: f64,
        var_idsatgat_d: f64,
        var_lgdrain_i: f64,
        var_one_minus_pgat_d: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_pbrstid_i: f64,
        var_pgatd_i: f64,
        var_slopesti_d: f64,
        var_two_psistar: f64,
        var_v2: f64,
        var_vav: f64,
        var_vbigat_d: f64,
        var_vbirgatinv_d: f64,
        var_vbrinvsti_d: f64,
        var_vbrstid_i: f64,
        var_vjsrh: f64,
        var_wdepnulrgat_d: f64,
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
        var_guard622_slot: &mut f64,
        var_guard623_slot: &mut f64,
        var_guard624_slot: &mut f64,
        var_guard625_slot: &mut f64,
        var_guard626_slot: &mut f64,
        var_guard627_slot: &mut f64,
        var_guard628_slot: &mut f64,
        var_guard629_slot: &mut f64,
        var_guard630_slot: &mut f64,
        var_guard631_slot: &mut f64,
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
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_sqrtumax_dn9_slot: &mut f64,
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
        let mut var_guard622: f64 = *var_guard622_slot;
        let mut var_guard623: f64 = *var_guard623_slot;
        let mut var_guard624: f64 = *var_guard624_slot;
        let mut var_guard625: f64 = *var_guard625_slot;
        let mut var_guard626: f64 = *var_guard626_slot;
        let mut var_guard627: f64 = *var_guard627_slot;
        let mut var_guard628: f64 = *var_guard628_slot;
        let mut var_guard629: f64 = *var_guard629_slot;
        let mut var_guard630: f64 = *var_guard630_slot;
        let mut var_guard631: f64 = *var_guard631_slot;
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
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_sqrtumax_dn9: f64 = *var_sqrtumax_dn9_slot;
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

        let assign30910_e39053: f64 = (-var_fbbtsti_d);
        let assign30910_e39055: f64 = (assign30910_e39053 / var_fmaxr);
        let assign30910_e39057: f64 = if assign30910_e39055 < 0.0 { 1.0 } else { 0.0 };
        var_guard622 = assign30910_e39057;

        let (assign30920_e39108, assign30920_e39108_d_n6, assign30920_e39108_d_n7, assign30920_e39108_d_n8, assign30920_e39108_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard619 == 0.0)) && (var_guard621 == 0.0)) && (var_guard622 != 0.0)) {
        let assign30920_e39075: f64 = (-230.25850929940458);
        let assign30920_e39077: f64 = (-var_fbbtsti_d);
        let assign30920_e39079: f64 = (assign30920_e39077 / var_fmaxr);
        let assign30920_e39080: f64 = (assign30920_e39075 - assign30920_e39079);
        let assign30920_e39084: f64 = (-230.25850929940458);
        let assign30920_e39086: f64 = (-var_fbbtsti_d);
        let assign30920_e39088: f64 = (assign30920_e39086 / var_fmaxr);
        let assign30920_e39089: f64 = (assign30920_e39084 - assign30920_e39088);
        let assign30920_e39092: f64 = (-230.25850929940458);
        let assign30920_e39094: f64 = (-var_fbbtsti_d);
        let assign30920_e39096: f64 = (assign30920_e39094 / var_fmaxr);
        let assign30920_e39097: f64 = (assign30920_e39092 - assign30920_e39096);
        let assign30920_e39099: f64 = (assign30920_e39097 * 0.3333333333333333);
        let assign30920_e39100: f64 = (1.0 + assign30920_e39099);
        let assign30920_e39101: f64 = (assign30920_e39089 * assign30920_e39100);
        let assign30920_e39102: f64 = (0.5 * assign30920_e39101);
        let assign30920_e39103: f64 = (1.0 + assign30920_e39102);
        let assign30920_e39104: f64 = (assign30920_e39080 * assign30920_e39103);
        let assign30920_e39105: f64 = (1.0 + assign30920_e39104);
        let assign30920_e39106: f64 = (1e-100 / assign30920_e39105);
        (assign30920_e39106, (-((1e-100 * (((-(-((assign30920_e39077 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign30920_e39103) + (assign30920_e39080 * (0.5 * (((-(-((assign30920_e39086 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign30920_e39100) + (assign30920_e39089 * ((-(-((assign30920_e39094 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign30920_e39105 * assign30920_e39105))), (-((1e-100 * (((-(-((assign30920_e39077 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign30920_e39103) + (assign30920_e39080 * (0.5 * (((-(-((assign30920_e39086 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign30920_e39100) + (assign30920_e39089 * ((-(-((assign30920_e39094 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign30920_e39105 * assign30920_e39105))), (-((1e-100 * (((-(-((assign30920_e39077 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign30920_e39103) + (assign30920_e39080 * (0.5 * (((-(-((assign30920_e39086 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign30920_e39100) + (assign30920_e39089 * ((-(-((assign30920_e39094 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign30920_e39105 * assign30920_e39105))), (-((1e-100 * (((-(-((assign30920_e39077 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign30920_e39103) + (assign30920_e39080 * (0.5 * (((-(-((assign30920_e39086 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign30920_e39100) + (assign30920_e39089 * ((-(-((assign30920_e39094 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign30920_e39105 * assign30920_e39105))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30920_e39108;
        var_tmp_dn6 = assign30920_e39108_d_n6;
        var_tmp_dn7 = assign30920_e39108_d_n7;
        var_tmp_dn8 = assign30920_e39108_d_n8;
        var_tmp_dn9 = assign30920_e39108_d_n9;

        let (assign30930_e39157, assign30930_e39157_d_n6, assign30930_e39157_d_n7, assign30930_e39157_d_n8, assign30930_e39157_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard619 == 0.0)) && (var_guard621 == 0.0)) && (var_guard622 == 0.0)) {
        let assign30930_e39127: f64 = (-var_fbbtsti_d);
        let assign30930_e39129: f64 = (assign30930_e39127 / var_fmaxr);
        let assign30930_e39131: f64 = (assign30930_e39129 - 230.25850929940458);
        let assign30930_e39135: f64 = (-var_fbbtsti_d);
        let assign30930_e39137: f64 = (assign30930_e39135 / var_fmaxr);
        let assign30930_e39139: f64 = (assign30930_e39137 - 230.25850929940458);
        let assign30930_e39142: f64 = (-var_fbbtsti_d);
        let assign30930_e39144: f64 = (assign30930_e39142 / var_fmaxr);
        let assign30930_e39146: f64 = (assign30930_e39144 - 230.25850929940458);
        let assign30930_e39148: f64 = (assign30930_e39146 * 0.3333333333333333);
        let assign30930_e39149: f64 = (1.0 + assign30930_e39148);
        let assign30930_e39150: f64 = (assign30930_e39139 * assign30930_e39149);
        let assign30930_e39151: f64 = (0.5 * assign30930_e39150);
        let assign30930_e39152: f64 = (1.0 + assign30930_e39151);
        let assign30930_e39153: f64 = (assign30930_e39131 * assign30930_e39152);
        let assign30930_e39154: f64 = (1.0 + assign30930_e39153);
        let assign30930_e39155: f64 = (1e100 * assign30930_e39154);
        (assign30930_e39155, (1e100 * (((-((assign30930_e39127 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign30930_e39152) + (assign30930_e39131 * (0.5 * (((-((assign30930_e39135 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign30930_e39149) + (assign30930_e39139 * ((-((assign30930_e39142 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign30930_e39127 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign30930_e39152) + (assign30930_e39131 * (0.5 * (((-((assign30930_e39135 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign30930_e39149) + (assign30930_e39139 * ((-((assign30930_e39142 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign30930_e39127 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign30930_e39152) + (assign30930_e39131 * (0.5 * (((-((assign30930_e39135 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign30930_e39149) + (assign30930_e39139 * ((-((assign30930_e39142 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign30930_e39127 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign30930_e39152) + (assign30930_e39131 * (0.5 * (((-((assign30930_e39135 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign30930_e39149) + (assign30930_e39139 * ((-((assign30930_e39142 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30930_e39157;
        var_tmp_dn6 = assign30930_e39157_d_n6;
        var_tmp_dn7 = assign30930_e39157_d_n7;
        var_tmp_dn8 = assign30930_e39157_d_n8;
        var_tmp_dn9 = assign30930_e39157_d_n9;

        let (assign30940_e39177, assign30940_e39177_d_n6, assign30940_e39177_d_n7, assign30940_e39177_d_n8, assign30940_e39177_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard619 == 0.0)) {
        let assign30940_e39170: f64 = (var_v2 * var_fmaxr);
        let assign30940_e39172: f64 = (assign30940_e39170 * var_fmaxr);
        let assign30940_e39174: f64 = (assign30940_e39172 * var_tmp);
        let assign30940_e39175: f64 = (var_cbbtstid_i * assign30940_e39174);
        (assign30940_e39175, (var_cbbtstid_i * (((((var_v2 * var_fmaxr_dn6) * var_fmaxr) + (assign30940_e39170 * var_fmaxr_dn6)) * var_tmp) + (assign30940_e39172 * var_tmp_dn6))), (var_cbbtstid_i * (((((var_v2 * var_fmaxr_dn7) * var_fmaxr) + (assign30940_e39170 * var_fmaxr_dn7)) * var_tmp) + (assign30940_e39172 * var_tmp_dn7))), (var_cbbtstid_i * (((((var_v2 * var_fmaxr_dn8) * var_fmaxr) + (assign30940_e39170 * var_fmaxr_dn8)) * var_tmp) + (assign30940_e39172 * var_tmp_dn8))), (var_cbbtstid_i * (((((var_v2 * var_fmaxr_dn9) * var_fmaxr) + (assign30940_e39170 * var_fmaxr_dn9)) * var_tmp) + (assign30940_e39172 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign30940_e39177;
        var_ibbt_dn6 = assign30940_e39177_d_n6;
        var_ibbt_dn7 = assign30940_e39177_d_n7;
        var_ibbt_dn8 = assign30940_e39177_d_n8;
        var_ibbt_dn9 = assign30940_e39177_d_n9;

        let assign30950_e39180: f64 = if var_vbrstid_i > 1000.0 { 1.0 } else { 0.0 };
        var_guard623 = assign30950_e39180;

        let (assign30960_e39191, assign30960_e39191_d_n6, assign30960_e39191_d_n7, assign30960_e39191_d_n8, assign30960_e39191_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard623 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign30960_e39191;
        var_fbreakdown_dn6 = assign30960_e39191_d_n6;
        var_fbreakdown_dn7 = assign30960_e39191_d_n7;
        var_fbreakdown_dn8 = assign30960_e39191_d_n8;
        var_fbreakdown_dn9 = assign30960_e39191_d_n9;

        let assign30970_e39194: f64 = (-var_alphaav);
        let assign30970_e39196: f64 = (assign30970_e39194 * var_vbrstid_i);
        let assign30970_e39197: f64 = if var_vav > assign30970_e39196 { 1.0 } else { 0.0 };
        var_guard624 = assign30970_e39197;

        let assign30980_e39200: f64 = if var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        var_guard625 = assign30980_e39200;

        let (assign30990_e39230, assign30990_e39230_d_n6, assign30990_e39230_d_n7, assign30990_e39230_d_n8, assign30990_e39230_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard623 == 0.0)) && (var_guard624 != 0.0)) && (var_guard625 != 0.0)) {
        let assign30990_e39216: f64 = (var_vav * var_vbrinvsti_d);
        let assign30990_e39219: f64 = (var_vav * var_vbrinvsti_d);
        let assign30990_e39220: f64 = (assign30990_e39216 * assign30990_e39219);
        let assign30990_e39223: f64 = (var_vav * var_vbrinvsti_d);
        let assign30990_e39224: f64 = (assign30990_e39220 * assign30990_e39223);
        let assign30990_e39227: f64 = (var_vav * var_vbrinvsti_d);
        let assign30990_e39228: f64 = (assign30990_e39224 * assign30990_e39227);
        (assign30990_e39228, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign30990_e39230;
        var_tmp_dn6 = assign30990_e39230_d_n6;
        var_tmp_dn7 = assign30990_e39230_d_n7;
        var_tmp_dn8 = assign30990_e39230_d_n8;
        var_tmp_dn9 = assign30990_e39230_d_n9;

        let (assign31000_e39252, assign31000_e39252_d_n6, assign31000_e39252_d_n7, assign31000_e39252_d_n8, assign31000_e39252_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard623 == 0.0)) && (var_guard624 != 0.0)) && (var_guard625 == 0.0)) {
        let assign31000_e39247: f64 = (var_vav * var_vbrinvsti_d);
        let assign31000_e39248: f64 = (assign31000_e39247).abs();
        let assign31000_e39250: f64 = (assign31000_e39248).powf(var_pbrstid_i);
        (assign31000_e39250, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign31000_e39252;
        var_tmp_dn6 = assign31000_e39252_d_n6;
        var_tmp_dn7 = assign31000_e39252_d_n7;
        var_tmp_dn8 = assign31000_e39252_d_n8;
        var_tmp_dn9 = assign31000_e39252_d_n9;

        let (assign31010_e39270, assign31010_e39270_d_n6, assign31010_e39270_d_n7, assign31010_e39270_d_n8, assign31010_e39270_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard623 == 0.0)) && (var_guard624 != 0.0)) {
        let assign31010_e39267: f64 = (1.0 - var_tmp);
        let assign31010_e39268: f64 = (1.0 / assign31010_e39267);
        (assign31010_e39268, (-((-var_tmp_dn6) / (assign31010_e39267 * assign31010_e39267))), (-((-var_tmp_dn7) / (assign31010_e39267 * assign31010_e39267))), (-((-var_tmp_dn8) / (assign31010_e39267 * assign31010_e39267))), (-((-var_tmp_dn9) / (assign31010_e39267 * assign31010_e39267))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign31010_e39270;
        var_fbreakdown_dn6 = assign31010_e39270_d_n6;
        var_fbreakdown_dn7 = assign31010_e39270_d_n7;
        var_fbreakdown_dn8 = assign31010_e39270_d_n8;
        var_fbreakdown_dn9 = assign31010_e39270_d_n9;

        let (assign31020_e39293, assign31020_e39293_d_n6, assign31020_e39293_d_n7, assign31020_e39293_d_n8, assign31020_e39293_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) && (var_guard623 == 0.0)) && (var_guard624 == 0.0)) {
        let assign31020_e39287: f64 = (var_alphaav * var_vbrstid_i);
        let assign31020_e39288: f64 = (var_vav + assign31020_e39287);
        let assign31020_e39290: f64 = (assign31020_e39288 * var_slopesti_d);
        let assign31020_e39291: f64 = (var_fstopsti_d + assign31020_e39290);
        (assign31020_e39291, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign31020_e39293;
        var_fbreakdown_dn6 = assign31020_e39293_d_n6;
        var_fbreakdown_dn7 = assign31020_e39293_d_n7;
        var_fbreakdown_dn8 = assign31020_e39293_d_n8;
        var_fbreakdown_dn9 = assign31020_e39293_d_n9;

        let (assign31030_e39312, assign31030_e39312_d_n6, assign31030_e39312_d_n7, assign31030_e39312_d_n8, assign31030_e39312_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard609 == 0.0)) {
        let assign31030_e39303: f64 = (var_id__blk212 + var_isrh);
        let assign31030_e39305: f64 = (assign31030_e39303 + var_itat);
        let assign31030_e39307: f64 = (assign31030_e39305 + var_ibbt);
        let assign31030_e39308: f64 = (p.p29 * assign31030_e39307);
        let assign31030_e39310: f64 = (assign31030_e39308 * var_fbreakdown);
        (assign31030_e39310, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign31030_e39308 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign31030_e39308 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign31030_e39308 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign31030_e39308 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign31030_e39312;
        var_ijunsti_dn6 = assign31030_e39312_d_n6;
        var_ijunsti_dn7 = assign31030_e39312_d_n7;
        var_ijunsti_dn8 = assign31030_e39312_d_n8;
        var_ijunsti_dn9 = assign31030_e39312_d_n9;

        let assign31040_e39315: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard626 = assign31040_e39315;

        let (assign31050_e39323, assign31050_e39323_d_n6, assign31050_e39323_d_n7, assign31050_e39323_d_n8, assign31050_e39323_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign31050_e39323;
        var_ijungat_dn6 = assign31050_e39323_d_n6;
        var_ijungat_dn7 = assign31050_e39323_d_n7;
        var_ijungat_dn8 = assign31050_e39323_d_n8;
        var_ijungat_dn9 = assign31050_e39323_d_n9;

        let (assign31060_e39334,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) {
        let assign31060_e39332: f64 = (var_idsatgat_d * var_idmult);
        (assign31060_e39332,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign31060_e39334;

        let assign31070_e39341: f64 = if ((var_csrhgatd_i == 0.0) && (var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard627 = assign31070_e39341;

        let (assign31080_e39352, assign31080_e39352_d_n6, assign31080_e39352_d_n7, assign31080_e39352_d_n8, assign31080_e39352_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard627 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign31080_e39352;
        var_isrh_dn6 = assign31080_e39352_d_n6;
        var_isrh_dn7 = assign31080_e39352_d_n7;
        var_isrh_dn8 = assign31080_e39352_d_n8;
        var_isrh_dn9 = assign31080_e39352_d_n9;

        let (assign31090_e39366,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard627 == 0.0)) {
        let assign31090_e39364: f64 = (var_vbigat_d - var_vjsrh);
        (assign31090_e39364,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign31090_e39366;

        let (assign31100_e39385,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard627 == 0.0)) {
        let assign31100_e39380: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign31100_e39381: f64 = (1.0 - assign31100_e39380);
        let assign31100_e39382: f64 = (assign31100_e39381).sqrt();
        let assign31100_e39383: f64 = (1.0 - assign31100_e39382);
        (assign31100_e39383,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign31100_e39385;

        let assign31110_e39388: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard628 = assign31110_e39388;

        let (assign31120_e39402,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard627 == 0.0)) && (var_guard628 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign31120_e39402;

        let (assign31130_e39434,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard627 == 0.0)) && (var_guard628 == 0.0)) {
        let assign31130_e39417: f64 = (var_wsrhstep * var_wsrhstep);
        let assign31130_e39419: f64 = (var_wsrhstep).ln();
        let assign31130_e39420: f64 = (assign31130_e39417 * assign31130_e39419);
        let assign31130_e39423: f64 = (1.0 - var_wsrhstep);
        let assign31130_e39424: f64 = (assign31130_e39420 / assign31130_e39423);
        let assign31130_e39426: f64 = (assign31130_e39424 + var_wsrhstep);
        let assign31130_e39430: f64 = (2.0 * var_pgatd_i);
        let assign31130_e39431: f64 = (1.0 - assign31130_e39430);
        let assign31130_e39432: f64 = (assign31130_e39426 * assign31130_e39431);
        (assign31130_e39432,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign31130_e39434;

        let (assign31140_e39448,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard627 == 0.0)) {
        let assign31140_e39446: f64 = (var_wsrhstep + var_dwsrh);
        (assign31140_e39446,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign31140_e39448;

        let assign31150_e39451: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard629 = assign31150_e39451;

        let (assign31160_e39468, assign31160_e39468_d_n6, assign31160_e39468_d_n7, assign31160_e39468_d_n8, assign31160_e39468_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard627 == 0.0)) && (var_guard629 != 0.0)) {
        let assign31160_e39465: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign31160_e39466: f64 = (assign31160_e39465).sqrt();
        (assign31160_e39466, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign31160_e39468;
        var_tmp_dn6 = assign31160_e39468_d_n6;
        var_tmp_dn7 = assign31160_e39468_d_n7;
        var_tmp_dn8 = assign31160_e39468_d_n8;
        var_tmp_dn9 = assign31160_e39468_d_n9;

        let (assign31170_e39487, assign31170_e39487_d_n6, assign31170_e39487_d_n7, assign31170_e39487_d_n8, assign31170_e39487_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard627 == 0.0)) && (var_guard629 == 0.0)) {
        let assign31170_e39483: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv_d);
        let assign31170_e39485: f64 = (assign31170_e39483).powf(var_pgatd_i);
        (assign31170_e39485, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign31170_e39487;
        var_tmp_dn6 = assign31170_e39487_d_n6;
        var_tmp_dn7 = assign31170_e39487_d_n7;
        var_tmp_dn8 = assign31170_e39487_d_n8;
        var_tmp_dn9 = assign31170_e39487_d_n9;

        let (assign31180_e39501, assign31180_e39501_d_n6, assign31180_e39501_d_n7, assign31180_e39501_d_n8, assign31180_e39501_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard627 == 0.0)) {
        let assign31180_e39499: f64 = (var_wdepnulrgat_d * var_tmp);
        (assign31180_e39499, (var_wdepnulrgat_d * var_tmp_dn6), (var_wdepnulrgat_d * var_tmp_dn7), (var_wdepnulrgat_d * var_tmp_dn8), (var_wdepnulrgat_d * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign31180_e39501;
        var_wdep_dn6 = assign31180_e39501_d_n6;
        var_wdep_dn7 = assign31180_e39501_d_n7;
        var_wdep_dn8 = assign31180_e39501_d_n8;
        var_wdep_dn9 = assign31180_e39501_d_n9;

        let (assign31190_e39519, assign31190_e39519_d_n6, assign31190_e39519_d_n7, assign31190_e39519_d_n8, assign31190_e39519_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard627 == 0.0)) {
        let assign31190_e39514: f64 = (var_zinv - 1.0);
        let assign31190_e39516: f64 = (assign31190_e39514 * var_wdep);
        let assign31190_e39517: f64 = (var_ftdgat_d * assign31190_e39516);
        (assign31190_e39517, (var_ftdgat_d * (assign31190_e39514 * var_wdep_dn6)), (var_ftdgat_d * (assign31190_e39514 * var_wdep_dn7)), (var_ftdgat_d * (assign31190_e39514 * var_wdep_dn8)), (var_ftdgat_d * (assign31190_e39514 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign31190_e39519;
        var_asrh_dn6 = assign31190_e39519_d_n6;
        var_asrh_dn7 = assign31190_e39519_d_n7;
        var_asrh_dn8 = assign31190_e39519_d_n8;
        var_asrh_dn9 = assign31190_e39519_d_n9;

        let (assign31200_e39535, assign31200_e39535_d_n6, assign31200_e39535_d_n7, assign31200_e39535_d_n8, assign31200_e39535_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard627 == 0.0)) {
        let assign31200_e39532: f64 = (var_asrh * var_wsrh);
        let assign31200_e39533: f64 = (var_csrhgatd_i * assign31200_e39532);
        (assign31200_e39533, (var_csrhgatd_i * (var_asrh_dn6 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn7 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn8 * var_wsrh)), (var_csrhgatd_i * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign31200_e39535;
        var_isrh_dn6 = assign31200_e39535_d_n6;
        var_isrh_dn7 = assign31200_e39535_d_n7;
        var_isrh_dn8 = assign31200_e39535_d_n8;
        var_isrh_dn9 = assign31200_e39535_d_n9;

        let assign31210_e39538: f64 = if var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard630 = assign31210_e39538;

        let (assign31220_e39549, assign31220_e39549_d_n6, assign31220_e39549_d_n7, assign31220_e39549_d_n8, assign31220_e39549_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign31220_e39549;
        var_itat_dn6 = assign31220_e39549_d_n6;
        var_itat_dn7 = assign31220_e39549_d_n7;
        var_itat_dn8 = assign31220_e39549_d_n8;
        var_itat_dn9 = assign31220_e39549_d_n9;

        let (assign31230_e39567, assign31230_e39567_d_n6, assign31230_e39567_d_n7, assign31230_e39567_d_n8, assign31230_e39567_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) {
        let assign31230_e39562: f64 = (var_wdep * var_one_minus_pgat_d);
        let assign31230_e39564: f64 = (assign31230_e39562 / var_vbi_minus_vjsrh);
        let assign31230_e39565: f64 = (var_btatpartgat_d * assign31230_e39564);
        (assign31230_e39565, (var_btatpartgat_d * ((var_wdep_dn6 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn7 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn8 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)), (var_btatpartgat_d * ((var_wdep_dn9 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign31230_e39567;
        var_btat_dn6 = assign31230_e39567_d_n6;
        var_btat_dn7 = assign31230_e39567_d_n7;
        var_btat_dn8 = assign31230_e39567_d_n8;
        var_btat_dn9 = assign31230_e39567_d_n9;

        let (assign31240_e39583, assign31240_e39583_d_n6, assign31240_e39583_d_n7, assign31240_e39583_d_n8, assign31240_e39583_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) {
        let assign31240_e39579: f64 = (0.666666666666667 * var_atatgat_d);
        let assign31240_e39581: f64 = (assign31240_e39579 / var_btat);
        (assign31240_e39581, (-((assign31240_e39579 * var_btat_dn6) / (var_btat * var_btat))), (-((assign31240_e39579 * var_btat_dn7) / (var_btat * var_btat))), (-((assign31240_e39579 * var_btat_dn8) / (var_btat * var_btat))), (-((assign31240_e39579 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign31240_e39583;
        var_twoatatoverthreebtat_dn6 = assign31240_e39583_d_n6;
        var_twoatatoverthreebtat_dn7 = assign31240_e39583_d_n7;
        var_twoatatoverthreebtat_dn8 = assign31240_e39583_d_n8;
        var_twoatatoverthreebtat_dn9 = assign31240_e39583_d_n9;

        let (assign31250_e39597, assign31250_e39597_d_n6, assign31250_e39597_d_n7, assign31250_e39597_d_n8, assign31250_e39597_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) {
        let assign31250_e39595: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign31250_e39595, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign31250_e39597;
        var_umaxbeforelimiting_dn6 = assign31250_e39597_d_n6;
        var_umaxbeforelimiting_dn7 = assign31250_e39597_d_n7;
        var_umaxbeforelimiting_dn8 = assign31250_e39597_d_n8;
        var_umaxbeforelimiting_dn9 = assign31250_e39597_d_n9;

        let (assign31260_e39618, assign31260_e39618_d_n6, assign31260_e39618_d_n7, assign31260_e39618_d_n8, assign31260_e39618_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) {
        let assign31260_e39609: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign31260_e39612: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign31260_e39614: f64 = (assign31260_e39612 + 1.0);
        let assign31260_e39615: f64 = (assign31260_e39609 / assign31260_e39614);
        let assign31260_e39616: f64 = (assign31260_e39615).sqrt();
        (assign31260_e39616, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign31260_e39614) - (assign31260_e39609 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign31260_e39614 * assign31260_e39614)) / (2.0 * assign31260_e39616)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign31260_e39614) - (assign31260_e39609 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign31260_e39614 * assign31260_e39614)) / (2.0 * assign31260_e39616)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign31260_e39614) - (assign31260_e39609 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign31260_e39614 * assign31260_e39614)) / (2.0 * assign31260_e39616)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign31260_e39614) - (assign31260_e39609 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign31260_e39614 * assign31260_e39614)) / (2.0 * assign31260_e39616)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign31260_e39618;
        var_umax_dn6 = assign31260_e39618_d_n6;
        var_umax_dn7 = assign31260_e39618_d_n7;
        var_umax_dn8 = assign31260_e39618_d_n8;
        var_umax_dn9 = assign31260_e39618_d_n9;

        let (assign31270_e39631, assign31270_e39631_d_n6, assign31270_e39631_d_n7, assign31270_e39631_d_n8, assign31270_e39631_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) {
        let assign31270_e39629: f64 = (var_umax).sqrt();
        (assign31270_e39629, (var_umax_dn6 / (2.0 * assign31270_e39629)), (var_umax_dn7 / (2.0 * assign31270_e39629)), (var_umax_dn8 / (2.0 * assign31270_e39629)), (var_umax_dn9 / (2.0 * assign31270_e39629)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign31270_e39631;
        var_sqrtumax_dn6 = assign31270_e39631_d_n6;
        var_sqrtumax_dn7 = assign31270_e39631_d_n7;
        var_sqrtumax_dn8 = assign31270_e39631_d_n8;
        var_sqrtumax_dn9 = assign31270_e39631_d_n9;

        let (assign31280_e39645, assign31280_e39645_d_n6, assign31280_e39645_d_n7, assign31280_e39645_d_n8, assign31280_e39645_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) {
        let assign31280_e39643: f64 = (var_umax * var_sqrtumax);
        (assign31280_e39643, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign31280_e39645;
        var_umaxpoweronepointfive_dn6 = assign31280_e39645_d_n6;
        var_umaxpoweronepointfive_dn7 = assign31280_e39645_d_n7;
        var_umaxpoweronepointfive_dn8 = assign31280_e39645_d_n8;
        var_umaxpoweronepointfive_dn9 = assign31280_e39645_d_n9;

        let assign31290_e39647: f64 = (-var_pgatd_i);
        let assign31290_e39649: f64 = (assign31290_e39647 * var_one_over_one_minus_pgat_d);
        let assign31290_e39651: f64 = (-1.0);
        let assign31290_e39652: f64 = if assign31290_e39649 == assign31290_e39651 { 1.0 } else { 0.0 };
        var_guard631 = assign31290_e39652;

        let (assign31300_e39672, assign31300_e39672_d_n6, assign31300_e39672_d_n7, assign31300_e39672_d_n8, assign31300_e39672_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard626 == 0.0)) && (var_guard630 == 0.0)) && (var_guard631 != 0.0)) {
        let assign31300_e39668: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign31300_e39669: f64 = (1.0 + assign31300_e39668);
        let assign31300_e39670: f64 = (1.0 / assign31300_e39669);
        (assign31300_e39670, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign31300_e39669 * assign31300_e39669))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign31300_e39669 * assign31300_e39669))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign31300_e39669 * assign31300_e39669))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign31300_e39669 * assign31300_e39669))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign31300_e39672;
        var_wgamma_dn6 = assign31300_e39672_d_n6;
        var_wgamma_dn7 = assign31300_e39672_d_n7;
        var_wgamma_dn8 = assign31300_e39672_d_n8;
        var_wgamma_dn9 = assign31300_e39672_d_n9;

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
        *var_guard622_slot = var_guard622;
        *var_guard623_slot = var_guard623;
        *var_guard624_slot = var_guard624;
        *var_guard625_slot = var_guard625;
        *var_guard626_slot = var_guard626;
        *var_guard627_slot = var_guard627;
        *var_guard628_slot = var_guard628;
        *var_guard629_slot = var_guard629;
        *var_guard630_slot = var_guard630;
        *var_guard631_slot = var_guard631;
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
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_sqrtumax_dn9_slot = var_sqrtumax_dn9;
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
    }
}
