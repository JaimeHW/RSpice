#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        var_guard251: f64,
        var_guard252: f64,
        var_ktnom: f64,
        var_lg: f64,
        var_nsubcdfm_given: f64,
        var_uc_eg0: f64,
        var_uc_mueph1: f64,
        var_uc_pgd1: f64,
        var_uc_rdict1: f64,
        var_uc_rdslp1: f64,
        var_uc_rs: f64,
        var_uc_scp22: f64,
        var_wg: f64,
        var_cecox_slot: &mut f64,
        var_clmmod_slot: &mut f64,
        var_cnstpgd_slot: &mut f64,
        var_ef_mueph1_slot: &mut f64,
        var_ef_mueph1_dn0_slot: &mut f64,
        var_ef_mueph1_dn10_slot: &mut f64,
        var_ef_mueph1_dn11_slot: &mut f64,
        var_ef_mueph1_dn14_slot: &mut f64,
        var_ef_mueph1_dn2_slot: &mut f64,
        var_ef_mueph1_dn4_slot: &mut f64,
        var_ef_mueph1_dn5_slot: &mut f64,
        var_ef_mueph1_dn6_slot: &mut f64,
        var_ef_mueph1_dn7_slot: &mut f64,
        var_ef_mueph1_dn8_slot: &mut f64,
        var_ef_mueph1_dn9_slot: &mut f64,
        var_ef_nsubc_slot: &mut f64,
        var_ef_nsubc_dn0_slot: &mut f64,
        var_ef_nsubc_dn10_slot: &mut f64,
        var_ef_nsubc_dn11_slot: &mut f64,
        var_ef_nsubc_dn14_slot: &mut f64,
        var_ef_nsubc_dn2_slot: &mut f64,
        var_ef_nsubc_dn4_slot: &mut f64,
        var_ef_nsubc_dn5_slot: &mut f64,
        var_ef_nsubc_dn6_slot: &mut f64,
        var_ef_nsubc_dn7_slot: &mut f64,
        var_ef_nsubc_dn8_slot: &mut f64,
        var_ef_nsubc_dn9_slot: &mut f64,
        var_ef_nsubp_slot: &mut f64,
        var_ef_nsubp_dn0_slot: &mut f64,
        var_ef_nsubp_dn10_slot: &mut f64,
        var_ef_nsubp_dn11_slot: &mut f64,
        var_ef_nsubp_dn14_slot: &mut f64,
        var_ef_nsubp_dn2_slot: &mut f64,
        var_ef_nsubp_dn4_slot: &mut f64,
        var_ef_nsubp_dn5_slot: &mut f64,
        var_ef_nsubp_dn6_slot: &mut f64,
        var_ef_nsubp_dn7_slot: &mut f64,
        var_ef_nsubp_dn8_slot: &mut f64,
        var_ef_nsubp_dn9_slot: &mut f64,
        var_egtnom_slot: &mut f64,
        var_flg_nqs_slot: &mut f64,
        var_flg_pgd_slot: &mut f64,
        var_flg_qmetemp_slot: &mut f64,
        var_flg_qy_slot: &mut f64,
        var_flg_rs_slot: &mut f64,
        var_guard254_slot: &mut f64,
        var_guard255_slot: &mut f64,
        var_guard256_slot: &mut f64,
        var_guard257_slot: &mut f64,
        var_i_slot: &mut f64,
        var_lod_half_slot: &mut f64,
        var_lod_half_dn0_slot: &mut f64,
        var_lod_half_dn10_slot: &mut f64,
        var_lod_half_dn11_slot: &mut f64,
        var_lod_half_dn14_slot: &mut f64,
        var_lod_half_dn2_slot: &mut f64,
        var_lod_half_dn4_slot: &mut f64,
        var_lod_half_dn5_slot: &mut f64,
        var_lod_half_dn6_slot: &mut f64,
        var_lod_half_dn7_slot: &mut f64,
        var_lod_half_dn8_slot: &mut f64,
        var_lod_half_dn9_slot: &mut f64,
        var_lod_half_ref_slot: &mut f64,
        var_lod_half_ref_dn0_slot: &mut f64,
        var_lod_half_ref_dn10_slot: &mut f64,
        var_lod_half_ref_dn11_slot: &mut f64,
        var_lod_half_ref_dn14_slot: &mut f64,
        var_lod_half_ref_dn2_slot: &mut f64,
        var_lod_half_ref_dn4_slot: &mut f64,
        var_lod_half_ref_dn5_slot: &mut f64,
        var_lod_half_ref_dn6_slot: &mut f64,
        var_lod_half_ref_dn7_slot: &mut f64,
        var_lod_half_ref_dn8_slot: &mut f64,
        var_lod_half_ref_dn9_slot: &mut f64,
        var_mks_nsubcdfm_slot: &mut f64,
        var_mks_subld2_slot: &mut f64,
        var_msc_slot: &mut f64,
        var_npexte_slot: &mut f64,
        var_npexte_dn0_slot: &mut f64,
        var_npexte_dn10_slot: &mut f64,
        var_npexte_dn11_slot: &mut f64,
        var_npexte_dn14_slot: &mut f64,
        var_npexte_dn2_slot: &mut f64,
        var_npexte_dn4_slot: &mut f64,
        var_npexte_dn5_slot: &mut f64,
        var_npexte_dn6_slot: &mut f64,
        var_npexte_dn7_slot: &mut f64,
        var_npexte_dn8_slot: &mut f64,
        var_npexte_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_uc_depvmax_slot: &mut f64,
        var_uc_depvmax_dn0_slot: &mut f64,
        var_uc_depvmax_dn10_slot: &mut f64,
        var_uc_depvmax_dn11_slot: &mut f64,
        var_uc_depvmax_dn14_slot: &mut f64,
        var_uc_depvmax_dn2_slot: &mut f64,
        var_uc_depvmax_dn4_slot: &mut f64,
        var_uc_depvmax_dn5_slot: &mut f64,
        var_uc_depvmax_dn6_slot: &mut f64,
        var_uc_depvmax_dn7_slot: &mut f64,
        var_uc_depvmax_dn8_slot: &mut f64,
        var_uc_depvmax_dn9_slot: &mut f64,
        var_uc_fn2_slot: &mut f64,
        var_uc_gdld_slot: &mut f64,
        var_uc_gidl1_slot: &mut f64,
        var_uc_gidl2_slot: &mut f64,
        var_uc_gleak2_slot: &mut f64,
        var_uc_glkb2_slot: &mut f64,
        var_uc_glksd1_slot: &mut f64,
        var_uc_glksd2_slot: &mut f64,
        var_uc_muesti1_slot: &mut f64,
        var_uc_nfalp_slot: &mut f64,
        var_uc_nover_slot: &mut f64,
        var_uc_novers_slot: &mut f64,
        var_uc_npext_slot: &mut f64,
        var_uc_nsti_slot: &mut f64,
        var_uc_nsubc_slot: &mut f64,
        var_uc_nsubp_slot: &mut f64,
        var_uc_nsubpsti1_slot: &mut f64,
        var_uc_rd22_slot: &mut f64,
        var_uc_rd23_slot: &mut f64,
        var_uc_rd24_slot: &mut f64,
        var_uc_rdvd_slot: &mut f64,
        var_uc_rth0_slot: &mut f64,
        var_uc_vfbover_slot: &mut f64,
        var_uc_vmax_slot: &mut f64,
        var_uc_wfc_slot: &mut f64,
        var_wlg_slot: &mut f64,
    ) {
        let mut var_cecox: f64 = *var_cecox_slot;
        let mut var_clmmod: f64 = *var_clmmod_slot;
        let mut var_cnstpgd: f64 = *var_cnstpgd_slot;
        let mut var_ef_mueph1: f64 = *var_ef_mueph1_slot;
        let mut var_ef_mueph1_dn0: f64 = *var_ef_mueph1_dn0_slot;
        let mut var_ef_mueph1_dn10: f64 = *var_ef_mueph1_dn10_slot;
        let mut var_ef_mueph1_dn11: f64 = *var_ef_mueph1_dn11_slot;
        let mut var_ef_mueph1_dn14: f64 = *var_ef_mueph1_dn14_slot;
        let mut var_ef_mueph1_dn2: f64 = *var_ef_mueph1_dn2_slot;
        let mut var_ef_mueph1_dn4: f64 = *var_ef_mueph1_dn4_slot;
        let mut var_ef_mueph1_dn5: f64 = *var_ef_mueph1_dn5_slot;
        let mut var_ef_mueph1_dn6: f64 = *var_ef_mueph1_dn6_slot;
        let mut var_ef_mueph1_dn7: f64 = *var_ef_mueph1_dn7_slot;
        let mut var_ef_mueph1_dn8: f64 = *var_ef_mueph1_dn8_slot;
        let mut var_ef_mueph1_dn9: f64 = *var_ef_mueph1_dn9_slot;
        let mut var_ef_nsubc: f64 = *var_ef_nsubc_slot;
        let mut var_ef_nsubc_dn0: f64 = *var_ef_nsubc_dn0_slot;
        let mut var_ef_nsubc_dn10: f64 = *var_ef_nsubc_dn10_slot;
        let mut var_ef_nsubc_dn11: f64 = *var_ef_nsubc_dn11_slot;
        let mut var_ef_nsubc_dn14: f64 = *var_ef_nsubc_dn14_slot;
        let mut var_ef_nsubc_dn2: f64 = *var_ef_nsubc_dn2_slot;
        let mut var_ef_nsubc_dn4: f64 = *var_ef_nsubc_dn4_slot;
        let mut var_ef_nsubc_dn5: f64 = *var_ef_nsubc_dn5_slot;
        let mut var_ef_nsubc_dn6: f64 = *var_ef_nsubc_dn6_slot;
        let mut var_ef_nsubc_dn7: f64 = *var_ef_nsubc_dn7_slot;
        let mut var_ef_nsubc_dn8: f64 = *var_ef_nsubc_dn8_slot;
        let mut var_ef_nsubc_dn9: f64 = *var_ef_nsubc_dn9_slot;
        let mut var_ef_nsubp: f64 = *var_ef_nsubp_slot;
        let mut var_ef_nsubp_dn0: f64 = *var_ef_nsubp_dn0_slot;
        let mut var_ef_nsubp_dn10: f64 = *var_ef_nsubp_dn10_slot;
        let mut var_ef_nsubp_dn11: f64 = *var_ef_nsubp_dn11_slot;
        let mut var_ef_nsubp_dn14: f64 = *var_ef_nsubp_dn14_slot;
        let mut var_ef_nsubp_dn2: f64 = *var_ef_nsubp_dn2_slot;
        let mut var_ef_nsubp_dn4: f64 = *var_ef_nsubp_dn4_slot;
        let mut var_ef_nsubp_dn5: f64 = *var_ef_nsubp_dn5_slot;
        let mut var_ef_nsubp_dn6: f64 = *var_ef_nsubp_dn6_slot;
        let mut var_ef_nsubp_dn7: f64 = *var_ef_nsubp_dn7_slot;
        let mut var_ef_nsubp_dn8: f64 = *var_ef_nsubp_dn8_slot;
        let mut var_ef_nsubp_dn9: f64 = *var_ef_nsubp_dn9_slot;
        let mut var_egtnom: f64 = *var_egtnom_slot;
        let mut var_flg_nqs: f64 = *var_flg_nqs_slot;
        let mut var_flg_pgd: f64 = *var_flg_pgd_slot;
        let mut var_flg_qmetemp: f64 = *var_flg_qmetemp_slot;
        let mut var_flg_qy: f64 = *var_flg_qy_slot;
        let mut var_flg_rs: f64 = *var_flg_rs_slot;
        let mut var_guard254: f64 = *var_guard254_slot;
        let mut var_guard255: f64 = *var_guard255_slot;
        let mut var_guard256: f64 = *var_guard256_slot;
        let mut var_guard257: f64 = *var_guard257_slot;
        let mut var_i: f64 = *var_i_slot;
        let mut var_lod_half: f64 = *var_lod_half_slot;
        let mut var_lod_half_dn0: f64 = *var_lod_half_dn0_slot;
        let mut var_lod_half_dn10: f64 = *var_lod_half_dn10_slot;
        let mut var_lod_half_dn11: f64 = *var_lod_half_dn11_slot;
        let mut var_lod_half_dn14: f64 = *var_lod_half_dn14_slot;
        let mut var_lod_half_dn2: f64 = *var_lod_half_dn2_slot;
        let mut var_lod_half_dn4: f64 = *var_lod_half_dn4_slot;
        let mut var_lod_half_dn5: f64 = *var_lod_half_dn5_slot;
        let mut var_lod_half_dn6: f64 = *var_lod_half_dn6_slot;
        let mut var_lod_half_dn7: f64 = *var_lod_half_dn7_slot;
        let mut var_lod_half_dn8: f64 = *var_lod_half_dn8_slot;
        let mut var_lod_half_dn9: f64 = *var_lod_half_dn9_slot;
        let mut var_lod_half_ref: f64 = *var_lod_half_ref_slot;
        let mut var_lod_half_ref_dn0: f64 = *var_lod_half_ref_dn0_slot;
        let mut var_lod_half_ref_dn10: f64 = *var_lod_half_ref_dn10_slot;
        let mut var_lod_half_ref_dn11: f64 = *var_lod_half_ref_dn11_slot;
        let mut var_lod_half_ref_dn14: f64 = *var_lod_half_ref_dn14_slot;
        let mut var_lod_half_ref_dn2: f64 = *var_lod_half_ref_dn2_slot;
        let mut var_lod_half_ref_dn4: f64 = *var_lod_half_ref_dn4_slot;
        let mut var_lod_half_ref_dn5: f64 = *var_lod_half_ref_dn5_slot;
        let mut var_lod_half_ref_dn6: f64 = *var_lod_half_ref_dn6_slot;
        let mut var_lod_half_ref_dn7: f64 = *var_lod_half_ref_dn7_slot;
        let mut var_lod_half_ref_dn8: f64 = *var_lod_half_ref_dn8_slot;
        let mut var_lod_half_ref_dn9: f64 = *var_lod_half_ref_dn9_slot;
        let mut var_mks_nsubcdfm: f64 = *var_mks_nsubcdfm_slot;
        let mut var_mks_subld2: f64 = *var_mks_subld2_slot;
        let mut var_msc: f64 = *var_msc_slot;
        let mut var_npexte: f64 = *var_npexte_slot;
        let mut var_npexte_dn0: f64 = *var_npexte_dn0_slot;
        let mut var_npexte_dn10: f64 = *var_npexte_dn10_slot;
        let mut var_npexte_dn11: f64 = *var_npexte_dn11_slot;
        let mut var_npexte_dn14: f64 = *var_npexte_dn14_slot;
        let mut var_npexte_dn2: f64 = *var_npexte_dn2_slot;
        let mut var_npexte_dn4: f64 = *var_npexte_dn4_slot;
        let mut var_npexte_dn5: f64 = *var_npexte_dn5_slot;
        let mut var_npexte_dn6: f64 = *var_npexte_dn6_slot;
        let mut var_npexte_dn7: f64 = *var_npexte_dn7_slot;
        let mut var_npexte_dn8: f64 = *var_npexte_dn8_slot;
        let mut var_npexte_dn9: f64 = *var_npexte_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_uc_depvmax: f64 = *var_uc_depvmax_slot;
        let mut var_uc_depvmax_dn0: f64 = *var_uc_depvmax_dn0_slot;
        let mut var_uc_depvmax_dn10: f64 = *var_uc_depvmax_dn10_slot;
        let mut var_uc_depvmax_dn11: f64 = *var_uc_depvmax_dn11_slot;
        let mut var_uc_depvmax_dn14: f64 = *var_uc_depvmax_dn14_slot;
        let mut var_uc_depvmax_dn2: f64 = *var_uc_depvmax_dn2_slot;
        let mut var_uc_depvmax_dn4: f64 = *var_uc_depvmax_dn4_slot;
        let mut var_uc_depvmax_dn5: f64 = *var_uc_depvmax_dn5_slot;
        let mut var_uc_depvmax_dn6: f64 = *var_uc_depvmax_dn6_slot;
        let mut var_uc_depvmax_dn7: f64 = *var_uc_depvmax_dn7_slot;
        let mut var_uc_depvmax_dn8: f64 = *var_uc_depvmax_dn8_slot;
        let mut var_uc_depvmax_dn9: f64 = *var_uc_depvmax_dn9_slot;
        let mut var_uc_fn2: f64 = *var_uc_fn2_slot;
        let mut var_uc_gdld: f64 = *var_uc_gdld_slot;
        let mut var_uc_gidl1: f64 = *var_uc_gidl1_slot;
        let mut var_uc_gidl2: f64 = *var_uc_gidl2_slot;
        let mut var_uc_gleak2: f64 = *var_uc_gleak2_slot;
        let mut var_uc_glkb2: f64 = *var_uc_glkb2_slot;
        let mut var_uc_glksd1: f64 = *var_uc_glksd1_slot;
        let mut var_uc_glksd2: f64 = *var_uc_glksd2_slot;
        let mut var_uc_muesti1: f64 = *var_uc_muesti1_slot;
        let mut var_uc_nfalp: f64 = *var_uc_nfalp_slot;
        let mut var_uc_nover: f64 = *var_uc_nover_slot;
        let mut var_uc_novers: f64 = *var_uc_novers_slot;
        let mut var_uc_npext: f64 = *var_uc_npext_slot;
        let mut var_uc_nsti: f64 = *var_uc_nsti_slot;
        let mut var_uc_nsubc: f64 = *var_uc_nsubc_slot;
        let mut var_uc_nsubp: f64 = *var_uc_nsubp_slot;
        let mut var_uc_nsubpsti1: f64 = *var_uc_nsubpsti1_slot;
        let mut var_uc_rd22: f64 = *var_uc_rd22_slot;
        let mut var_uc_rd23: f64 = *var_uc_rd23_slot;
        let mut var_uc_rd24: f64 = *var_uc_rd24_slot;
        let mut var_uc_rdvd: f64 = *var_uc_rdvd_slot;
        let mut var_uc_rth0: f64 = *var_uc_rth0_slot;
        let mut var_uc_vfbover: f64 = *var_uc_vfbover_slot;
        let mut var_uc_vmax: f64 = *var_uc_vmax_slot;
        let mut var_uc_wfc: f64 = *var_uc_wfc_slot;
        let mut var_wlg: f64 = *var_wlg_slot;

        let (assign10300_e5494, assign10300_e5494_d_n0, assign10300_e5494_d_n2, assign10300_e5494_d_n4, assign10300_e5494_d_n5, assign10300_e5494_d_n6, assign10300_e5494_d_n7, assign10300_e5494_d_n8, assign10300_e5494_d_n9, assign10300_e5494_d_n10, assign10300_e5494_d_n11, assign10300_e5494_d_n14,) = {
    if ((var_guard251 == 0.0) && (var_guard252 == 0.0)) {
        let assign10300_e5472: f64 = (p.p131 * p.p3);
        let assign10300_e5474: f64 = (assign10300_e5472 * p.p7);
        let assign10300_e5478: f64 = (p.p69 * var_uc_rdslp1);
        let assign10300_e5480: f64 = (assign10300_e5478 * 1000000.0);
        let assign10300_e5482: f64 = (assign10300_e5480 + var_uc_rdict1);
        let assign10300_e5483: f64 = (var_uc_rs * assign10300_e5482);
        let assign10300_e5486: f64 = (p.p70 * p.p100);
        let assign10300_e5488: f64 = (assign10300_e5486 * 1000000.0);
        let assign10300_e5490: f64 = (assign10300_e5488 + p.p101);
        let assign10300_e5491: f64 = (assign10300_e5483 * assign10300_e5490);
        let assign10300_e5492: f64 = (assign10300_e5474 + assign10300_e5491);
        (assign10300_e5492, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign10300_e5494;
        var_t1_dn0 = assign10300_e5494_d_n0;
        var_t1_dn2 = assign10300_e5494_d_n2;
        var_t1_dn4 = assign10300_e5494_d_n4;
        var_t1_dn5 = assign10300_e5494_d_n5;
        var_t1_dn6 = assign10300_e5494_d_n6;
        var_t1_dn7 = assign10300_e5494_d_n7;
        var_t1_dn8 = assign10300_e5494_d_n8;
        var_t1_dn9 = assign10300_e5494_d_n9;
        var_t1_dn10 = assign10300_e5494_d_n10;
        var_t1_dn11 = assign10300_e5494_d_n11;
        var_t1_dn14 = assign10300_e5494_d_n14;

        let (assign10310_e5507,) = {
    if ((var_guard251 == 0.0) && (var_guard252 == 0.0)) {
        let (assign10310_e5505,) = {
            if (var_t1 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10310_e5505,)
    } else {
        (var_flg_rs,)
    }
};
        var_flg_rs = assign10310_e5507;

        let assign10320_e5510: f64 = (p.p12 / 1e-6);
        var_mks_nsubcdfm = assign10320_e5510;

        let assign10330_e5513: f64 = (p.p73 * 100.0);
        var_mks_subld2 = assign10330_e5513;

        let assign10340_e5516: f64 = (var_uc_nsubc / 1e-6);
        var_uc_nsubc = assign10340_e5516;

        let assign10350_e5519: f64 = (var_uc_nsubp / 1e-6);
        var_uc_nsubp = assign10350_e5519;

        let assign10360_e5522: f64 = (var_uc_nsti / 1e-6);
        var_uc_nsti = assign10360_e5522;

        let assign10370_e5525: f64 = (var_uc_nover / 1e-6);
        var_uc_nover = assign10370_e5525;

        let assign10380_e5528: f64 = (var_uc_novers / 1e-6);
        var_uc_novers = assign10380_e5528;

        let assign10390_e5531: f64 = (var_uc_nsubpsti1 / 100.0);
        var_uc_nsubpsti1 = assign10390_e5531;

        let assign10400_e5534: f64 = (var_uc_muesti1 / 100.0);
        var_uc_muesti1 = assign10400_e5534;

        let assign10410_e5537: f64 = (var_uc_vmax / 100.0);
        var_uc_vmax = assign10410_e5537;

        let assign10420_e5540: f64 = (var_uc_wfc * 10000.0);
        var_uc_wfc = assign10420_e5540;

        let assign10430_e5543: f64 = (var_uc_glksd1 / 100.0);
        var_uc_glksd1 = assign10430_e5543;

        let assign10440_e5546: f64 = (var_uc_glksd2 * 100.0);
        var_uc_glksd2 = assign10440_e5546;

        let assign10450_e5549: f64 = (var_uc_gleak2 * 100.0);
        var_uc_gleak2 = assign10450_e5549;

        let assign10460_e5552: f64 = (var_uc_glkb2 * 100.0);
        var_uc_glkb2 = assign10460_e5552;

        let assign10470_e5555: f64 = (var_uc_fn2 * 100.0);
        var_uc_fn2 = assign10470_e5555;

        let assign10480_e5558: f64 = (var_uc_gidl1 / 10.0);
        var_uc_gidl1 = assign10480_e5558;

        let assign10490_e5561: f64 = (var_uc_gidl2 * 100.0);
        var_uc_gidl2 = assign10490_e5561;

        let assign10500_e5564: f64 = (var_uc_nfalp / 100.0);
        var_uc_nfalp = assign10500_e5564;

        let assign10520_e5570: f64 = (var_uc_npext / 1e-6);
        var_uc_npext = assign10520_e5570;

        let assign10530_e5573: f64 = (var_uc_rd22 / 100.0);
        var_uc_rd22 = assign10530_e5573;

        let assign10540_e5576: f64 = (var_uc_rd23 / 100.0);
        var_uc_rd23 = assign10540_e5576;

        let assign10550_e5579: f64 = (var_uc_rd24 / 100.0);
        var_uc_rd24 = assign10550_e5579;

        let assign10560_e5582: f64 = (var_uc_rdvd / 100.0);
        var_uc_rdvd = assign10560_e5582;

        let assign10570_e5585: f64 = (var_uc_rth0 / 100.0);
        var_uc_rth0 = assign10570_e5585;

        let assign10580_e5587: f64 = (-var_uc_vfbover);
        var_uc_vfbover = assign10580_e5587;

        let assign10590_e5590: f64 = (var_uc_depvmax / 100.0);
        var_uc_depvmax = assign10590_e5590;
        var_uc_depvmax_dn0 = (var_uc_depvmax_dn0 / 100.0);
        var_uc_depvmax_dn2 = (var_uc_depvmax_dn2 / 100.0);
        var_uc_depvmax_dn4 = (var_uc_depvmax_dn4 / 100.0);
        var_uc_depvmax_dn5 = (var_uc_depvmax_dn5 / 100.0);
        var_uc_depvmax_dn6 = (var_uc_depvmax_dn6 / 100.0);
        var_uc_depvmax_dn7 = (var_uc_depvmax_dn7 / 100.0);
        var_uc_depvmax_dn8 = (var_uc_depvmax_dn8 / 100.0);
        var_uc_depvmax_dn9 = (var_uc_depvmax_dn9 / 100.0);
        var_uc_depvmax_dn10 = (var_uc_depvmax_dn10 / 100.0);
        var_uc_depvmax_dn11 = (var_uc_depvmax_dn11 / 100.0);
        var_uc_depvmax_dn14 = (var_uc_depvmax_dn14 / 100.0);

        var_flg_nqs = p.p28;

        let (assign10610_e5601,) = {
    if ((p.p133 != 0.0) || (p.p134 != 0.0)) {
        (1.0,)
    } else {
        (0.0,)
    }
};
        var_flg_qy = assign10610_e5601;

        let assign10630_e5615: f64 = if (((p.p235 == 0.0) && (p.p237 == 0.0)) || (p.p236 == 0.0)) { 1.0 } else { 0.0 };
        var_guard254 = assign10630_e5615;

        let (assign10640_e5619,) = {
    if (var_guard254 != 0.0) {
        (0.0,)
    } else {
        (var_flg_qmetemp,)
    }
};
        var_flg_qmetemp = assign10640_e5619;

        let (assign10650_e5624,) = {
    if (var_guard254 == 0.0) {
        (1.0,)
    } else {
        (var_flg_qmetemp,)
    }
};
        var_flg_qmetemp = assign10650_e5624;

        let assign10660_e5627: f64 = (var_wg * var_lg);
        var_wlg = assign10660_e5627;

        let assign10670_e5630: f64 = (p.p289 * 1000000.0);
        var_uc_gdld = assign10670_e5630;

        let assign10680_e5636: f64 = (var_ktnom * 1e-7);
        let assign10680_e5637: f64 = (9.025e-5 + assign10680_e5636);
        let assign10680_e5638: f64 = (var_ktnom * assign10680_e5637);
        let assign10680_e5639: f64 = (var_uc_eg0 - assign10680_e5638);
        var_egtnom = assign10680_e5639;

        let assign10690_e5642: f64 = (8.8541878e-12 * p.p267);
        var_cecox = assign10690_e5642;

        var_msc = var_uc_scp22;

        let assign10710_e5646: f64 = if var_uc_pgd1 == 0.0 { 1.0 } else { 0.0 };
        var_guard255 = assign10710_e5646;

        let (assign10720_e5650,) = {
    if (var_guard255 != 0.0) {
        (0.0,)
    } else {
        (var_flg_pgd,)
    }
};
        var_flg_pgd = assign10720_e5650;

        let (assign10730_e5654,) = {
    if (var_guard255 != 0.0) {
        (0.0,)
    } else {
        (var_cnstpgd,)
    }
};
        var_cnstpgd = assign10730_e5654;

        let (assign10740_e5659,) = {
    if (var_guard255 == 0.0) {
        (1.0,)
    } else {
        (var_flg_pgd,)
    }
};
        var_flg_pgd = assign10740_e5659;

        let (assign10750_e5672,) = {
    if (var_guard255 == 0.0) {
        let assign10750_e5665: f64 = (1.0 / var_lg);
        let assign10750_e5666: f64 = (1.0 + assign10750_e5665);
        let assign10750_e5668: f64 = (assign10750_e5666).powf(p.p153);
        let assign10750_e5670: f64 = (assign10750_e5668 * var_uc_pgd1);
        (assign10750_e5670,)
    } else {
        (var_cnstpgd,)
    }
};
        var_cnstpgd = assign10750_e5672;

        let assign10760_e5676: f64 = (var_lg).powf(p.p229);
        let assign10760_e5678: f64 = (assign10760_e5676 * p.p230);
        let assign10760_e5679: f64 = (1.0 + assign10760_e5678);
        var_clmmod = assign10760_e5679;

        let assign10770_e5684: f64 = (0.5 * p.p0);
        let assign10770_e5685: f64 = (p.p118 + assign10770_e5684);
        let assign10770_e5686: f64 = (1.0 / assign10770_e5685);
        let assign10770_e5691: f64 = (0.5 * p.p0);
        let assign10770_e5692: f64 = (p.p119 + assign10770_e5691);
        let assign10770_e5693: f64 = (1.0 / assign10770_e5692);
        let assign10770_e5694: f64 = (assign10770_e5686 + assign10770_e5693);
        var_t1 = assign10770_e5694;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn14 = 0.0;

        let assign10780_e5697: f64 = (2.0 / var_t1);
        var_lod_half_ref = assign10780_e5697;
        var_lod_half_ref_dn0 = (-((2.0 * var_t1_dn0) / (var_t1 * var_t1)));
        var_lod_half_ref_dn2 = (-((2.0 * var_t1_dn2) / (var_t1 * var_t1)));
        var_lod_half_ref_dn4 = (-((2.0 * var_t1_dn4) / (var_t1 * var_t1)));
        var_lod_half_ref_dn5 = (-((2.0 * var_t1_dn5) / (var_t1 * var_t1)));
        var_lod_half_ref_dn6 = (-((2.0 * var_t1_dn6) / (var_t1 * var_t1)));
        var_lod_half_ref_dn7 = (-((2.0 * var_t1_dn7) / (var_t1 * var_t1)));
        var_lod_half_ref_dn8 = (-((2.0 * var_t1_dn8) / (var_t1 * var_t1)));
        var_lod_half_ref_dn9 = (-((2.0 * var_t1_dn9) / (var_t1 * var_t1)));
        var_lod_half_ref_dn10 = (-((2.0 * var_t1_dn10) / (var_t1 * var_t1)));
        var_lod_half_ref_dn11 = (-((2.0 * var_t1_dn11) / (var_t1 * var_t1)));
        var_lod_half_ref_dn14 = (-((2.0 * var_t1_dn14) / (var_t1 * var_t1)));

        let assign10790_e5716: f64 = if (((p.p8 > 0.0) && (p.p9 > 0.0)) && ((p.p7 == 1.0) || ((p.p7 > 1.0) && (p.p10 > 0.0)))) { 1.0 } else { 0.0 };
        var_guard256 = assign10790_e5716;

        let (assign10800_e5720, assign10800_e5720_d_n0, assign10800_e5720_d_n2, assign10800_e5720_d_n4, assign10800_e5720_d_n5, assign10800_e5720_d_n6, assign10800_e5720_d_n7, assign10800_e5720_d_n8, assign10800_e5720_d_n9, assign10800_e5720_d_n10, assign10800_e5720_d_n11, assign10800_e5720_d_n14,) = {
    if (var_guard256 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign10800_e5720;
        var_t1_dn0 = assign10800_e5720_d_n0;
        var_t1_dn2 = assign10800_e5720_d_n2;
        var_t1_dn4 = assign10800_e5720_d_n4;
        var_t1_dn5 = assign10800_e5720_d_n5;
        var_t1_dn6 = assign10800_e5720_d_n6;
        var_t1_dn7 = assign10800_e5720_d_n7;
        var_t1_dn8 = assign10800_e5720_d_n8;
        var_t1_dn9 = assign10800_e5720_d_n9;
        var_t1_dn10 = assign10800_e5720_d_n10;
        var_t1_dn11 = assign10800_e5720_d_n11;
        var_t1_dn14 = assign10800_e5720_d_n14;

        let (assign10810_e5724,) = {
    if (var_guard256 != 0.0) {
        (0.0,)
    } else {
        (var_i,)
    }
};
        var_i = assign10810_e5724;

        let mut assign10820_loop_guard: usize = 0;
        while {
            let assign10820_cond_e5729: f64 = if ((var_guard256 != 0.0) && (var_i < p.p7)) { 1.0 } else { 0.0 };
            assign10820_cond_e5729 != 0.0
        } {
            assign10820_loop_guard += 1;
            assert!(assign10820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign10820_body0_e5761, assign10820_body0_e5761_d_n0, assign10820_body0_e5761_d_n2, assign10820_body0_e5761_d_n4, assign10820_body0_e5761_d_n5, assign10820_body0_e5761_d_n6, assign10820_body0_e5761_d_n7, assign10820_body0_e5761_d_n8, assign10820_body0_e5761_d_n9, assign10820_body0_e5761_d_n10, assign10820_body0_e5761_d_n11, assign10820_body0_e5761_d_n14,) = {
    if (var_guard256 != 0.0) {
        let assign10820_body0_e5736: f64 = (0.5 * p.p0);
        let assign10820_body0_e5737: f64 = (p.p8 + assign10820_body0_e5736);
        let assign10820_body0_e5741: f64 = (p.p10 + p.p0);
        let assign10820_body0_e5742: f64 = (var_i * assign10820_body0_e5741);
        let assign10820_body0_e5743: f64 = (assign10820_body0_e5737 + assign10820_body0_e5742);
        let assign10820_body0_e5744: f64 = (1.0 / assign10820_body0_e5743);
        let assign10820_body0_e5745: f64 = (var_t1 + assign10820_body0_e5744);
        let assign10820_body0_e5750: f64 = (0.5 * p.p0);
        let assign10820_body0_e5751: f64 = (p.p9 + assign10820_body0_e5750);
        let assign10820_body0_e5755: f64 = (p.p10 + p.p0);
        let assign10820_body0_e5756: f64 = (var_i * assign10820_body0_e5755);
        let assign10820_body0_e5757: f64 = (assign10820_body0_e5751 + assign10820_body0_e5756);
        let assign10820_body0_e5758: f64 = (1.0 / assign10820_body0_e5757);
        let assign10820_body0_e5759: f64 = (assign10820_body0_e5745 + assign10820_body0_e5758);
        (assign10820_body0_e5759, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
            var_t1 = assign10820_body0_e5761;
            var_t1_dn0 = assign10820_body0_e5761_d_n0;
            var_t1_dn2 = assign10820_body0_e5761_d_n2;
            var_t1_dn4 = assign10820_body0_e5761_d_n4;
            var_t1_dn5 = assign10820_body0_e5761_d_n5;
            var_t1_dn6 = assign10820_body0_e5761_d_n6;
            var_t1_dn7 = assign10820_body0_e5761_d_n7;
            var_t1_dn8 = assign10820_body0_e5761_d_n8;
            var_t1_dn9 = assign10820_body0_e5761_d_n9;
            var_t1_dn10 = assign10820_body0_e5761_d_n10;
            var_t1_dn11 = assign10820_body0_e5761_d_n11;
            var_t1_dn14 = assign10820_body0_e5761_d_n14;
            let (assign10820_body1_e5767,) = {
    if (var_guard256 != 0.0) {
        let assign10820_body1_e5765: f64 = (var_i + 1.0);
        (assign10820_body1_e5765,)
    } else {
        (var_i,)
    }
};
            var_i = assign10820_body1_e5767;
        }

        let (assign10830_e5775, assign10830_e5775_d_n0, assign10830_e5775_d_n2, assign10830_e5775_d_n4, assign10830_e5775_d_n5, assign10830_e5775_d_n6, assign10830_e5775_d_n7, assign10830_e5775_d_n8, assign10830_e5775_d_n9, assign10830_e5775_d_n10, assign10830_e5775_d_n11, assign10830_e5775_d_n14,) = {
    if (var_guard256 != 0.0) {
        let assign10830_e5771: f64 = (2.0 * p.p7);
        let assign10830_e5773: f64 = (assign10830_e5771 / var_t1);
        (assign10830_e5773, (-((assign10830_e5771 * var_t1_dn0) / (var_t1 * var_t1))), (-((assign10830_e5771 * var_t1_dn2) / (var_t1 * var_t1))), (-((assign10830_e5771 * var_t1_dn4) / (var_t1 * var_t1))), (-((assign10830_e5771 * var_t1_dn5) / (var_t1 * var_t1))), (-((assign10830_e5771 * var_t1_dn6) / (var_t1 * var_t1))), (-((assign10830_e5771 * var_t1_dn7) / (var_t1 * var_t1))), (-((assign10830_e5771 * var_t1_dn8) / (var_t1 * var_t1))), (-((assign10830_e5771 * var_t1_dn9) / (var_t1 * var_t1))), (-((assign10830_e5771 * var_t1_dn10) / (var_t1 * var_t1))), (-((assign10830_e5771 * var_t1_dn11) / (var_t1 * var_t1))), (-((assign10830_e5771 * var_t1_dn14) / (var_t1 * var_t1))),)
    } else {
        (var_lod_half, var_lod_half_dn0, var_lod_half_dn2, var_lod_half_dn4, var_lod_half_dn5, var_lod_half_dn6, var_lod_half_dn7, var_lod_half_dn8, var_lod_half_dn9, var_lod_half_dn10, var_lod_half_dn11, var_lod_half_dn14,)
    }
};
        var_lod_half = assign10830_e5775;
        var_lod_half_dn0 = assign10830_e5775_d_n0;
        var_lod_half_dn2 = assign10830_e5775_d_n2;
        var_lod_half_dn4 = assign10830_e5775_d_n4;
        var_lod_half_dn5 = assign10830_e5775_d_n5;
        var_lod_half_dn6 = assign10830_e5775_d_n6;
        var_lod_half_dn7 = assign10830_e5775_d_n7;
        var_lod_half_dn8 = assign10830_e5775_d_n8;
        var_lod_half_dn9 = assign10830_e5775_d_n9;
        var_lod_half_dn10 = assign10830_e5775_d_n10;
        var_lod_half_dn11 = assign10830_e5775_d_n11;
        var_lod_half_dn14 = assign10830_e5775_d_n14;

        let (assign10840_e5780, assign10840_e5780_d_n0, assign10840_e5780_d_n2, assign10840_e5780_d_n4, assign10840_e5780_d_n5, assign10840_e5780_d_n6, assign10840_e5780_d_n7, assign10840_e5780_d_n8, assign10840_e5780_d_n9, assign10840_e5780_d_n10, assign10840_e5780_d_n11, assign10840_e5780_d_n14,) = {
    if (var_guard256 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_lod_half, var_lod_half_dn0, var_lod_half_dn2, var_lod_half_dn4, var_lod_half_dn5, var_lod_half_dn6, var_lod_half_dn7, var_lod_half_dn8, var_lod_half_dn9, var_lod_half_dn10, var_lod_half_dn11, var_lod_half_dn14,)
    }
};
        var_lod_half = assign10840_e5780;
        var_lod_half_dn0 = assign10840_e5780_d_n0;
        var_lod_half_dn2 = assign10840_e5780_d_n2;
        var_lod_half_dn4 = assign10840_e5780_d_n4;
        var_lod_half_dn5 = assign10840_e5780_d_n5;
        var_lod_half_dn6 = assign10840_e5780_d_n6;
        var_lod_half_dn7 = assign10840_e5780_d_n7;
        var_lod_half_dn8 = assign10840_e5780_d_n8;
        var_lod_half_dn9 = assign10840_e5780_d_n9;
        var_lod_half_dn10 = assign10840_e5780_d_n10;
        var_lod_half_dn11 = assign10840_e5780_d_n11;
        var_lod_half_dn14 = assign10840_e5780_d_n14;

        var_npexte = var_uc_npext;
        var_npexte_dn0 = 0.0;
        var_npexte_dn2 = 0.0;
        var_npexte_dn4 = 0.0;
        var_npexte_dn5 = 0.0;
        var_npexte_dn6 = 0.0;
        var_npexte_dn7 = 0.0;
        var_npexte_dn8 = 0.0;
        var_npexte_dn9 = 0.0;
        var_npexte_dn10 = 0.0;
        var_npexte_dn11 = 0.0;
        var_npexte_dn14 = 0.0;

        var_ef_mueph1 = var_uc_mueph1;
        var_ef_mueph1_dn0 = 0.0;
        var_ef_mueph1_dn2 = 0.0;
        var_ef_mueph1_dn4 = 0.0;
        var_ef_mueph1_dn5 = 0.0;
        var_ef_mueph1_dn6 = 0.0;
        var_ef_mueph1_dn7 = 0.0;
        var_ef_mueph1_dn8 = 0.0;
        var_ef_mueph1_dn9 = 0.0;
        var_ef_mueph1_dn10 = 0.0;
        var_ef_mueph1_dn11 = 0.0;
        var_ef_mueph1_dn14 = 0.0;

        var_ef_nsubp = var_uc_nsubp;
        var_ef_nsubp_dn0 = 0.0;
        var_ef_nsubp_dn2 = 0.0;
        var_ef_nsubp_dn4 = 0.0;
        var_ef_nsubp_dn5 = 0.0;
        var_ef_nsubp_dn6 = 0.0;
        var_ef_nsubp_dn7 = 0.0;
        var_ef_nsubp_dn8 = 0.0;
        var_ef_nsubp_dn9 = 0.0;
        var_ef_nsubp_dn10 = 0.0;
        var_ef_nsubp_dn11 = 0.0;
        var_ef_nsubp_dn14 = 0.0;

        var_ef_nsubc = var_uc_nsubc;
        var_ef_nsubc_dn0 = 0.0;
        var_ef_nsubc_dn2 = 0.0;
        var_ef_nsubc_dn4 = 0.0;
        var_ef_nsubc_dn5 = 0.0;
        var_ef_nsubc_dn6 = 0.0;
        var_ef_nsubc_dn7 = 0.0;
        var_ef_nsubc_dn8 = 0.0;
        var_ef_nsubc_dn9 = 0.0;
        var_ef_nsubc_dn10 = 0.0;
        var_ef_nsubc_dn11 = 0.0;
        var_ef_nsubc_dn14 = 0.0;

        let assign10890_e5789: f64 = if ((p.p32 == 1.0) && (var_nsubcdfm_given != 0.0)) { 1.0 } else { 0.0 };
        var_guard257 = assign10890_e5789;

        let (assign10910_e5810, assign10910_e5810_d_n0, assign10910_e5810_d_n2, assign10910_e5810_d_n4, assign10910_e5810_d_n5, assign10910_e5810_d_n6, assign10910_e5810_d_n7, assign10910_e5810_d_n8, assign10910_e5810_d_n9, assign10910_e5810_d_n10, assign10910_e5810_d_n11, assign10910_e5810_d_n14,) = {
    if (var_guard257 != 0.0) {
        let assign10910_e5801: f64 = (var_mks_nsubcdfm).ln();
        let assign10910_e5803: f64 = (var_ef_nsubc).ln();
        let assign10910_e5804: f64 = (assign10910_e5801 - assign10910_e5803);
        let assign10910_e5805: f64 = (p.p282 * assign10910_e5804);
        let assign10910_e5807: f64 = (assign10910_e5805 + 1.0);
        let assign10910_e5808: f64 = (var_ef_mueph1 * assign10910_e5807);
        (assign10910_e5808, ((var_ef_mueph1_dn0 * assign10910_e5807) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn0 / var_ef_nsubc))))), ((var_ef_mueph1_dn2 * assign10910_e5807) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn2 / var_ef_nsubc))))), ((var_ef_mueph1_dn4 * assign10910_e5807) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn4 / var_ef_nsubc))))), ((var_ef_mueph1_dn5 * assign10910_e5807) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn5 / var_ef_nsubc))))), ((var_ef_mueph1_dn6 * assign10910_e5807) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn6 / var_ef_nsubc))))), ((var_ef_mueph1_dn7 * assign10910_e5807) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn7 / var_ef_nsubc))))), ((var_ef_mueph1_dn8 * assign10910_e5807) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn8 / var_ef_nsubc))))), ((var_ef_mueph1_dn9 * assign10910_e5807) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn9 / var_ef_nsubc))))), ((var_ef_mueph1_dn10 * assign10910_e5807) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn10 / var_ef_nsubc))))), ((var_ef_mueph1_dn11 * assign10910_e5807) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn11 / var_ef_nsubc))))), ((var_ef_mueph1_dn14 * assign10910_e5807) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn14 / var_ef_nsubc))))),)
    } else {
        (var_ef_mueph1, var_ef_mueph1_dn0, var_ef_mueph1_dn2, var_ef_mueph1_dn4, var_ef_mueph1_dn5, var_ef_mueph1_dn6, var_ef_mueph1_dn7, var_ef_mueph1_dn8, var_ef_mueph1_dn9, var_ef_mueph1_dn10, var_ef_mueph1_dn11, var_ef_mueph1_dn14,)
    }
};
        var_ef_mueph1 = assign10910_e5810;
        var_ef_mueph1_dn0 = assign10910_e5810_d_n0;
        var_ef_mueph1_dn2 = assign10910_e5810_d_n2;
        var_ef_mueph1_dn4 = assign10910_e5810_d_n4;
        var_ef_mueph1_dn5 = assign10910_e5810_d_n5;
        var_ef_mueph1_dn6 = assign10910_e5810_d_n6;
        var_ef_mueph1_dn7 = assign10910_e5810_d_n7;
        var_ef_mueph1_dn8 = assign10910_e5810_d_n8;
        var_ef_mueph1_dn9 = assign10910_e5810_d_n9;
        var_ef_mueph1_dn10 = assign10910_e5810_d_n10;
        var_ef_mueph1_dn11 = assign10910_e5810_d_n11;
        var_ef_mueph1_dn14 = assign10910_e5810_d_n14;

        let (assign10920_e5818, assign10920_e5818_d_n0, assign10920_e5818_d_n2, assign10920_e5818_d_n4, assign10920_e5818_d_n5, assign10920_e5818_d_n6, assign10920_e5818_d_n7, assign10920_e5818_d_n8, assign10920_e5818_d_n9, assign10920_e5818_d_n10, assign10920_e5818_d_n11, assign10920_e5818_d_n14,) = {
    if (var_guard257 != 0.0) {
        let assign10920_e5814: f64 = (var_ef_nsubp + var_mks_nsubcdfm);
        let assign10920_e5816: f64 = (assign10920_e5814 - var_ef_nsubc);
        (assign10920_e5816, (var_ef_nsubp_dn0 - var_ef_nsubc_dn0), (var_ef_nsubp_dn2 - var_ef_nsubc_dn2), (var_ef_nsubp_dn4 - var_ef_nsubc_dn4), (var_ef_nsubp_dn5 - var_ef_nsubc_dn5), (var_ef_nsubp_dn6 - var_ef_nsubc_dn6), (var_ef_nsubp_dn7 - var_ef_nsubc_dn7), (var_ef_nsubp_dn8 - var_ef_nsubc_dn8), (var_ef_nsubp_dn9 - var_ef_nsubc_dn9), (var_ef_nsubp_dn10 - var_ef_nsubc_dn10), (var_ef_nsubp_dn11 - var_ef_nsubc_dn11), (var_ef_nsubp_dn14 - var_ef_nsubc_dn14),)
    } else {
        (var_ef_nsubp, var_ef_nsubp_dn0, var_ef_nsubp_dn2, var_ef_nsubp_dn4, var_ef_nsubp_dn5, var_ef_nsubp_dn6, var_ef_nsubp_dn7, var_ef_nsubp_dn8, var_ef_nsubp_dn9, var_ef_nsubp_dn10, var_ef_nsubp_dn11, var_ef_nsubp_dn14,)
    }
};
        var_ef_nsubp = assign10920_e5818;
        var_ef_nsubp_dn0 = assign10920_e5818_d_n0;
        var_ef_nsubp_dn2 = assign10920_e5818_d_n2;
        var_ef_nsubp_dn4 = assign10920_e5818_d_n4;
        var_ef_nsubp_dn5 = assign10920_e5818_d_n5;
        var_ef_nsubp_dn6 = assign10920_e5818_d_n6;
        var_ef_nsubp_dn7 = assign10920_e5818_d_n7;
        var_ef_nsubp_dn8 = assign10920_e5818_d_n8;
        var_ef_nsubp_dn9 = assign10920_e5818_d_n9;
        var_ef_nsubp_dn10 = assign10920_e5818_d_n10;
        var_ef_nsubp_dn11 = assign10920_e5818_d_n11;
        var_ef_nsubp_dn14 = assign10920_e5818_d_n14;

        *var_cecox_slot = var_cecox;
        *var_clmmod_slot = var_clmmod;
        *var_cnstpgd_slot = var_cnstpgd;
        *var_ef_mueph1_slot = var_ef_mueph1;
        *var_ef_mueph1_dn0_slot = var_ef_mueph1_dn0;
        *var_ef_mueph1_dn10_slot = var_ef_mueph1_dn10;
        *var_ef_mueph1_dn11_slot = var_ef_mueph1_dn11;
        *var_ef_mueph1_dn14_slot = var_ef_mueph1_dn14;
        *var_ef_mueph1_dn2_slot = var_ef_mueph1_dn2;
        *var_ef_mueph1_dn4_slot = var_ef_mueph1_dn4;
        *var_ef_mueph1_dn5_slot = var_ef_mueph1_dn5;
        *var_ef_mueph1_dn6_slot = var_ef_mueph1_dn6;
        *var_ef_mueph1_dn7_slot = var_ef_mueph1_dn7;
        *var_ef_mueph1_dn8_slot = var_ef_mueph1_dn8;
        *var_ef_mueph1_dn9_slot = var_ef_mueph1_dn9;
        *var_ef_nsubc_slot = var_ef_nsubc;
        *var_ef_nsubc_dn0_slot = var_ef_nsubc_dn0;
        *var_ef_nsubc_dn10_slot = var_ef_nsubc_dn10;
        *var_ef_nsubc_dn11_slot = var_ef_nsubc_dn11;
        *var_ef_nsubc_dn14_slot = var_ef_nsubc_dn14;
        *var_ef_nsubc_dn2_slot = var_ef_nsubc_dn2;
        *var_ef_nsubc_dn4_slot = var_ef_nsubc_dn4;
        *var_ef_nsubc_dn5_slot = var_ef_nsubc_dn5;
        *var_ef_nsubc_dn6_slot = var_ef_nsubc_dn6;
        *var_ef_nsubc_dn7_slot = var_ef_nsubc_dn7;
        *var_ef_nsubc_dn8_slot = var_ef_nsubc_dn8;
        *var_ef_nsubc_dn9_slot = var_ef_nsubc_dn9;
        *var_ef_nsubp_slot = var_ef_nsubp;
        *var_ef_nsubp_dn0_slot = var_ef_nsubp_dn0;
        *var_ef_nsubp_dn10_slot = var_ef_nsubp_dn10;
        *var_ef_nsubp_dn11_slot = var_ef_nsubp_dn11;
        *var_ef_nsubp_dn14_slot = var_ef_nsubp_dn14;
        *var_ef_nsubp_dn2_slot = var_ef_nsubp_dn2;
        *var_ef_nsubp_dn4_slot = var_ef_nsubp_dn4;
        *var_ef_nsubp_dn5_slot = var_ef_nsubp_dn5;
        *var_ef_nsubp_dn6_slot = var_ef_nsubp_dn6;
        *var_ef_nsubp_dn7_slot = var_ef_nsubp_dn7;
        *var_ef_nsubp_dn8_slot = var_ef_nsubp_dn8;
        *var_ef_nsubp_dn9_slot = var_ef_nsubp_dn9;
        *var_egtnom_slot = var_egtnom;
        *var_flg_nqs_slot = var_flg_nqs;
        *var_flg_pgd_slot = var_flg_pgd;
        *var_flg_qmetemp_slot = var_flg_qmetemp;
        *var_flg_qy_slot = var_flg_qy;
        *var_flg_rs_slot = var_flg_rs;
        *var_guard254_slot = var_guard254;
        *var_guard255_slot = var_guard255;
        *var_guard256_slot = var_guard256;
        *var_guard257_slot = var_guard257;
        *var_i_slot = var_i;
        *var_lod_half_slot = var_lod_half;
        *var_lod_half_dn0_slot = var_lod_half_dn0;
        *var_lod_half_dn10_slot = var_lod_half_dn10;
        *var_lod_half_dn11_slot = var_lod_half_dn11;
        *var_lod_half_dn14_slot = var_lod_half_dn14;
        *var_lod_half_dn2_slot = var_lod_half_dn2;
        *var_lod_half_dn4_slot = var_lod_half_dn4;
        *var_lod_half_dn5_slot = var_lod_half_dn5;
        *var_lod_half_dn6_slot = var_lod_half_dn6;
        *var_lod_half_dn7_slot = var_lod_half_dn7;
        *var_lod_half_dn8_slot = var_lod_half_dn8;
        *var_lod_half_dn9_slot = var_lod_half_dn9;
        *var_lod_half_ref_slot = var_lod_half_ref;
        *var_lod_half_ref_dn0_slot = var_lod_half_ref_dn0;
        *var_lod_half_ref_dn10_slot = var_lod_half_ref_dn10;
        *var_lod_half_ref_dn11_slot = var_lod_half_ref_dn11;
        *var_lod_half_ref_dn14_slot = var_lod_half_ref_dn14;
        *var_lod_half_ref_dn2_slot = var_lod_half_ref_dn2;
        *var_lod_half_ref_dn4_slot = var_lod_half_ref_dn4;
        *var_lod_half_ref_dn5_slot = var_lod_half_ref_dn5;
        *var_lod_half_ref_dn6_slot = var_lod_half_ref_dn6;
        *var_lod_half_ref_dn7_slot = var_lod_half_ref_dn7;
        *var_lod_half_ref_dn8_slot = var_lod_half_ref_dn8;
        *var_lod_half_ref_dn9_slot = var_lod_half_ref_dn9;
        *var_mks_nsubcdfm_slot = var_mks_nsubcdfm;
        *var_mks_subld2_slot = var_mks_subld2;
        *var_msc_slot = var_msc;
        *var_npexte_slot = var_npexte;
        *var_npexte_dn0_slot = var_npexte_dn0;
        *var_npexte_dn10_slot = var_npexte_dn10;
        *var_npexte_dn11_slot = var_npexte_dn11;
        *var_npexte_dn14_slot = var_npexte_dn14;
        *var_npexte_dn2_slot = var_npexte_dn2;
        *var_npexte_dn4_slot = var_npexte_dn4;
        *var_npexte_dn5_slot = var_npexte_dn5;
        *var_npexte_dn6_slot = var_npexte_dn6;
        *var_npexte_dn7_slot = var_npexte_dn7;
        *var_npexte_dn8_slot = var_npexte_dn8;
        *var_npexte_dn9_slot = var_npexte_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_uc_depvmax_slot = var_uc_depvmax;
        *var_uc_depvmax_dn0_slot = var_uc_depvmax_dn0;
        *var_uc_depvmax_dn10_slot = var_uc_depvmax_dn10;
        *var_uc_depvmax_dn11_slot = var_uc_depvmax_dn11;
        *var_uc_depvmax_dn14_slot = var_uc_depvmax_dn14;
        *var_uc_depvmax_dn2_slot = var_uc_depvmax_dn2;
        *var_uc_depvmax_dn4_slot = var_uc_depvmax_dn4;
        *var_uc_depvmax_dn5_slot = var_uc_depvmax_dn5;
        *var_uc_depvmax_dn6_slot = var_uc_depvmax_dn6;
        *var_uc_depvmax_dn7_slot = var_uc_depvmax_dn7;
        *var_uc_depvmax_dn8_slot = var_uc_depvmax_dn8;
        *var_uc_depvmax_dn9_slot = var_uc_depvmax_dn9;
        *var_uc_fn2_slot = var_uc_fn2;
        *var_uc_gdld_slot = var_uc_gdld;
        *var_uc_gidl1_slot = var_uc_gidl1;
        *var_uc_gidl2_slot = var_uc_gidl2;
        *var_uc_gleak2_slot = var_uc_gleak2;
        *var_uc_glkb2_slot = var_uc_glkb2;
        *var_uc_glksd1_slot = var_uc_glksd1;
        *var_uc_glksd2_slot = var_uc_glksd2;
        *var_uc_muesti1_slot = var_uc_muesti1;
        *var_uc_nfalp_slot = var_uc_nfalp;
        *var_uc_nover_slot = var_uc_nover;
        *var_uc_novers_slot = var_uc_novers;
        *var_uc_npext_slot = var_uc_npext;
        *var_uc_nsti_slot = var_uc_nsti;
        *var_uc_nsubc_slot = var_uc_nsubc;
        *var_uc_nsubp_slot = var_uc_nsubp;
        *var_uc_nsubpsti1_slot = var_uc_nsubpsti1;
        *var_uc_rd22_slot = var_uc_rd22;
        *var_uc_rd23_slot = var_uc_rd23;
        *var_uc_rd24_slot = var_uc_rd24;
        *var_uc_rdvd_slot = var_uc_rdvd;
        *var_uc_rth0_slot = var_uc_rth0;
        *var_uc_vfbover_slot = var_uc_vfbover;
        *var_uc_vmax_slot = var_uc_vmax;
        *var_uc_wfc_slot = var_uc_wfc;
        *var_wlg_slot = var_wlg;
    }

    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        var_ef_mueph1: f64,
        var_ef_mueph1_dn0: f64,
        var_ef_mueph1_dn10: f64,
        var_ef_mueph1_dn11: f64,
        var_ef_mueph1_dn14: f64,
        var_ef_mueph1_dn2: f64,
        var_ef_mueph1_dn4: f64,
        var_ef_mueph1_dn5: f64,
        var_ef_mueph1_dn6: f64,
        var_ef_mueph1_dn7: f64,
        var_ef_mueph1_dn8: f64,
        var_ef_mueph1_dn9: f64,
        var_ef_nsubp: f64,
        var_ef_nsubp_dn0: f64,
        var_ef_nsubp_dn10: f64,
        var_ef_nsubp_dn11: f64,
        var_ef_nsubp_dn14: f64,
        var_ef_nsubp_dn2: f64,
        var_ef_nsubp_dn4: f64,
        var_ef_nsubp_dn5: f64,
        var_ef_nsubp_dn6: f64,
        var_ef_nsubp_dn7: f64,
        var_ef_nsubp_dn8: f64,
        var_ef_nsubp_dn9: f64,
        var_guard257: f64,
        var_lg: f64,
        var_lgate: f64,
        var_lod_half: f64,
        var_lod_half_dn0: f64,
        var_lod_half_dn10: f64,
        var_lod_half_dn11: f64,
        var_lod_half_dn14: f64,
        var_lod_half_dn2: f64,
        var_lod_half_dn4: f64,
        var_lod_half_dn5: f64,
        var_lod_half_dn6: f64,
        var_lod_half_dn7: f64,
        var_lod_half_dn8: f64,
        var_lod_half_dn9: f64,
        var_lod_half_ref: f64,
        var_lod_half_ref_dn0: f64,
        var_lod_half_ref_dn10: f64,
        var_lod_half_ref_dn11: f64,
        var_lod_half_ref_dn14: f64,
        var_lod_half_ref_dn2: f64,
        var_lod_half_ref_dn4: f64,
        var_lod_half_ref_dn5: f64,
        var_lod_half_ref_dn6: f64,
        var_lod_half_ref_dn7: f64,
        var_lod_half_ref_dn8: f64,
        var_lod_half_ref_dn9: f64,
        var_mks_ll: f64,
        var_mks_nsubcdfm: f64,
        var_mks_wl: f64,
        var_uc_muesti1: f64,
        var_uc_muesti2: f64,
        var_uc_muesti3: f64,
        var_uc_ndep: f64,
        var_uc_ninv: f64,
        var_uc_wl2: f64,
        var_uc_xldld: f64,
        var_wg: f64,
        var_wgate: f64,
        var_wlg: f64,
        var_dl_slot: &mut f64,
        var_dlld_slot: &mut f64,
        var_dvthsm_slot: &mut f64,
        var_dw_slot: &mut f64,
        var_dwcv_slot: &mut f64,
        var_dwld_slot: &mut f64,
        var_ef_nsubc_slot: &mut f64,
        var_ef_nsubc_dn0_slot: &mut f64,
        var_ef_nsubc_dn10_slot: &mut f64,
        var_ef_nsubc_dn11_slot: &mut f64,
        var_ef_nsubc_dn14_slot: &mut f64,
        var_ef_nsubc_dn2_slot: &mut f64,
        var_ef_nsubc_dn4_slot: &mut f64,
        var_ef_nsubc_dn5_slot: &mut f64,
        var_ef_nsubc_dn6_slot: &mut f64,
        var_ef_nsubc_dn7_slot: &mut f64,
        var_ef_nsubc_dn8_slot: &mut f64,
        var_ef_nsubc_dn9_slot: &mut f64,
        var_guard259_slot: &mut f64,
        var_guard267_slot: &mut f64,
        var_guard269_slot: &mut f64,
        var_leff_slot: &mut f64,
        var_lgatesm_slot: &mut f64,
        var_mueph_slot: &mut f64,
        var_mueph_dn0_slot: &mut f64,
        var_mueph_dn10_slot: &mut f64,
        var_mueph_dn11_slot: &mut f64,
        var_mueph_dn14_slot: &mut f64,
        var_mueph_dn2_slot: &mut f64,
        var_mueph_dn4_slot: &mut f64,
        var_mueph_dn5_slot: &mut f64,
        var_mueph_dn6_slot: &mut f64,
        var_mueph_dn7_slot: &mut f64,
        var_mueph_dn8_slot: &mut f64,
        var_mueph_dn9_slot: &mut f64,
        var_muesr_slot: &mut f64,
        var_ndep_o_esi_slot: &mut f64,
        var_ndep_o_esi_dn0_slot: &mut f64,
        var_ndep_o_esi_dn10_slot: &mut f64,
        var_ndep_o_esi_dn11_slot: &mut f64,
        var_ndep_o_esi_dn14_slot: &mut f64,
        var_ndep_o_esi_dn2_slot: &mut f64,
        var_ndep_o_esi_dn4_slot: &mut f64,
        var_ndep_o_esi_dn5_slot: &mut f64,
        var_ndep_o_esi_dn6_slot: &mut f64,
        var_ndep_o_esi_dn7_slot: &mut f64,
        var_ndep_o_esi_dn8_slot: &mut f64,
        var_ndep_o_esi_dn9_slot: &mut f64,
        var_ninv_o_esi_slot: &mut f64,
        var_ninvd0_slot: &mut f64,
        var_ninvd0cres_slot: &mut f64,
        var_ninvd0cres_dn0_slot: &mut f64,
        var_ninvd0cres_dn10_slot: &mut f64,
        var_ninvd0cres_dn11_slot: &mut f64,
        var_ninvd0cres_dn14_slot: &mut f64,
        var_ninvd0cres_dn2_slot: &mut f64,
        var_ninvd0cres_dn4_slot: &mut f64,
        var_ninvd0cres_dn5_slot: &mut f64,
        var_ninvd0cres_dn6_slot: &mut f64,
        var_ninvd0cres_dn7_slot: &mut f64,
        var_ninvd0cres_dn8_slot: &mut f64,
        var_ninvd0cres_dn9_slot: &mut f64,
        var_ninvd0hres_slot: &mut f64,
        var_ninvd0hres_dn0_slot: &mut f64,
        var_ninvd0hres_dn10_slot: &mut f64,
        var_ninvd0hres_dn11_slot: &mut f64,
        var_ninvd0hres_dn14_slot: &mut f64,
        var_ninvd0hres_dn2_slot: &mut f64,
        var_ninvd0hres_dn4_slot: &mut f64,
        var_ninvd0hres_dn5_slot: &mut f64,
        var_ninvd0hres_dn6_slot: &mut f64,
        var_ninvd0hres_dn7_slot: &mut f64,
        var_ninvd0hres_dn8_slot: &mut f64,
        var_ninvd0hres_dn9_slot: &mut f64,
        var_npexte_slot: &mut f64,
        var_npexte_dn0_slot: &mut f64,
        var_npexte_dn10_slot: &mut f64,
        var_npexte_dn11_slot: &mut f64,
        var_npexte_dn14_slot: &mut f64,
        var_npexte_dn2_slot: &mut f64,
        var_npexte_dn4_slot: &mut f64,
        var_npexte_dn5_slot: &mut f64,
        var_npexte_dn6_slot: &mut f64,
        var_npexte_dn7_slot: &mut f64,
        var_npexte_dn8_slot: &mut f64,
        var_npexte_dn9_slot: &mut f64,
        var_nsubpp_slot: &mut f64,
        var_nsubpp_dn0_slot: &mut f64,
        var_nsubpp_dn10_slot: &mut f64,
        var_nsubpp_dn11_slot: &mut f64,
        var_nsubpp_dn14_slot: &mut f64,
        var_nsubpp_dn2_slot: &mut f64,
        var_nsubpp_dn4_slot: &mut f64,
        var_nsubpp_dn5_slot: &mut f64,
        var_nsubpp_dn6_slot: &mut f64,
        var_nsubpp_dn7_slot: &mut f64,
        var_nsubpp_dn8_slot: &mut f64,
        var_nsubpp_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_uc_wsti_slot: &mut f64,
        var_uc_wsti_dn0_slot: &mut f64,
        var_uc_wsti_dn10_slot: &mut f64,
        var_uc_wsti_dn11_slot: &mut f64,
        var_uc_wsti_dn14_slot: &mut f64,
        var_uc_wsti_dn2_slot: &mut f64,
        var_uc_wsti_dn4_slot: &mut f64,
        var_uc_wsti_dn5_slot: &mut f64,
        var_uc_wsti_dn6_slot: &mut f64,
        var_uc_wsti_dn7_slot: &mut f64,
        var_uc_wsti_dn8_slot: &mut f64,
        var_uc_wsti_dn9_slot: &mut f64,
        var_weff_slot: &mut f64,
        var_weff_cv_slot: &mut f64,
        var_weff_ld_slot: &mut f64,
        var_weff_nf_slot: &mut f64,
        var_weffcv_nf_slot: &mut f64,
    ) {
        let mut var_dl: f64 = *var_dl_slot;
        let mut var_dlld: f64 = *var_dlld_slot;
        let mut var_dvthsm: f64 = *var_dvthsm_slot;
        let mut var_dw: f64 = *var_dw_slot;
        let mut var_dwcv: f64 = *var_dwcv_slot;
        let mut var_dwld: f64 = *var_dwld_slot;
        let mut var_ef_nsubc: f64 = *var_ef_nsubc_slot;
        let mut var_ef_nsubc_dn0: f64 = *var_ef_nsubc_dn0_slot;
        let mut var_ef_nsubc_dn10: f64 = *var_ef_nsubc_dn10_slot;
        let mut var_ef_nsubc_dn11: f64 = *var_ef_nsubc_dn11_slot;
        let mut var_ef_nsubc_dn14: f64 = *var_ef_nsubc_dn14_slot;
        let mut var_ef_nsubc_dn2: f64 = *var_ef_nsubc_dn2_slot;
        let mut var_ef_nsubc_dn4: f64 = *var_ef_nsubc_dn4_slot;
        let mut var_ef_nsubc_dn5: f64 = *var_ef_nsubc_dn5_slot;
        let mut var_ef_nsubc_dn6: f64 = *var_ef_nsubc_dn6_slot;
        let mut var_ef_nsubc_dn7: f64 = *var_ef_nsubc_dn7_slot;
        let mut var_ef_nsubc_dn8: f64 = *var_ef_nsubc_dn8_slot;
        let mut var_ef_nsubc_dn9: f64 = *var_ef_nsubc_dn9_slot;
        let mut var_guard259: f64 = *var_guard259_slot;
        let mut var_guard267: f64 = *var_guard267_slot;
        let mut var_guard269: f64 = *var_guard269_slot;
        let mut var_leff: f64 = *var_leff_slot;
        let mut var_lgatesm: f64 = *var_lgatesm_slot;
        let mut var_mueph: f64 = *var_mueph_slot;
        let mut var_mueph_dn0: f64 = *var_mueph_dn0_slot;
        let mut var_mueph_dn10: f64 = *var_mueph_dn10_slot;
        let mut var_mueph_dn11: f64 = *var_mueph_dn11_slot;
        let mut var_mueph_dn14: f64 = *var_mueph_dn14_slot;
        let mut var_mueph_dn2: f64 = *var_mueph_dn2_slot;
        let mut var_mueph_dn4: f64 = *var_mueph_dn4_slot;
        let mut var_mueph_dn5: f64 = *var_mueph_dn5_slot;
        let mut var_mueph_dn6: f64 = *var_mueph_dn6_slot;
        let mut var_mueph_dn7: f64 = *var_mueph_dn7_slot;
        let mut var_mueph_dn8: f64 = *var_mueph_dn8_slot;
        let mut var_mueph_dn9: f64 = *var_mueph_dn9_slot;
        let mut var_muesr: f64 = *var_muesr_slot;
        let mut var_ndep_o_esi: f64 = *var_ndep_o_esi_slot;
        let mut var_ndep_o_esi_dn0: f64 = *var_ndep_o_esi_dn0_slot;
        let mut var_ndep_o_esi_dn10: f64 = *var_ndep_o_esi_dn10_slot;
        let mut var_ndep_o_esi_dn11: f64 = *var_ndep_o_esi_dn11_slot;
        let mut var_ndep_o_esi_dn14: f64 = *var_ndep_o_esi_dn14_slot;
        let mut var_ndep_o_esi_dn2: f64 = *var_ndep_o_esi_dn2_slot;
        let mut var_ndep_o_esi_dn4: f64 = *var_ndep_o_esi_dn4_slot;
        let mut var_ndep_o_esi_dn5: f64 = *var_ndep_o_esi_dn5_slot;
        let mut var_ndep_o_esi_dn6: f64 = *var_ndep_o_esi_dn6_slot;
        let mut var_ndep_o_esi_dn7: f64 = *var_ndep_o_esi_dn7_slot;
        let mut var_ndep_o_esi_dn8: f64 = *var_ndep_o_esi_dn8_slot;
        let mut var_ndep_o_esi_dn9: f64 = *var_ndep_o_esi_dn9_slot;
        let mut var_ninv_o_esi: f64 = *var_ninv_o_esi_slot;
        let mut var_ninvd0: f64 = *var_ninvd0_slot;
        let mut var_ninvd0cres: f64 = *var_ninvd0cres_slot;
        let mut var_ninvd0cres_dn0: f64 = *var_ninvd0cres_dn0_slot;
        let mut var_ninvd0cres_dn10: f64 = *var_ninvd0cres_dn10_slot;
        let mut var_ninvd0cres_dn11: f64 = *var_ninvd0cres_dn11_slot;
        let mut var_ninvd0cres_dn14: f64 = *var_ninvd0cres_dn14_slot;
        let mut var_ninvd0cres_dn2: f64 = *var_ninvd0cres_dn2_slot;
        let mut var_ninvd0cres_dn4: f64 = *var_ninvd0cres_dn4_slot;
        let mut var_ninvd0cres_dn5: f64 = *var_ninvd0cres_dn5_slot;
        let mut var_ninvd0cres_dn6: f64 = *var_ninvd0cres_dn6_slot;
        let mut var_ninvd0cres_dn7: f64 = *var_ninvd0cres_dn7_slot;
        let mut var_ninvd0cres_dn8: f64 = *var_ninvd0cres_dn8_slot;
        let mut var_ninvd0cres_dn9: f64 = *var_ninvd0cres_dn9_slot;
        let mut var_ninvd0hres: f64 = *var_ninvd0hres_slot;
        let mut var_ninvd0hres_dn0: f64 = *var_ninvd0hres_dn0_slot;
        let mut var_ninvd0hres_dn10: f64 = *var_ninvd0hres_dn10_slot;
        let mut var_ninvd0hres_dn11: f64 = *var_ninvd0hres_dn11_slot;
        let mut var_ninvd0hres_dn14: f64 = *var_ninvd0hres_dn14_slot;
        let mut var_ninvd0hres_dn2: f64 = *var_ninvd0hres_dn2_slot;
        let mut var_ninvd0hres_dn4: f64 = *var_ninvd0hres_dn4_slot;
        let mut var_ninvd0hres_dn5: f64 = *var_ninvd0hres_dn5_slot;
        let mut var_ninvd0hres_dn6: f64 = *var_ninvd0hres_dn6_slot;
        let mut var_ninvd0hres_dn7: f64 = *var_ninvd0hres_dn7_slot;
        let mut var_ninvd0hres_dn8: f64 = *var_ninvd0hres_dn8_slot;
        let mut var_ninvd0hres_dn9: f64 = *var_ninvd0hres_dn9_slot;
        let mut var_npexte: f64 = *var_npexte_slot;
        let mut var_npexte_dn0: f64 = *var_npexte_dn0_slot;
        let mut var_npexte_dn10: f64 = *var_npexte_dn10_slot;
        let mut var_npexte_dn11: f64 = *var_npexte_dn11_slot;
        let mut var_npexte_dn14: f64 = *var_npexte_dn14_slot;
        let mut var_npexte_dn2: f64 = *var_npexte_dn2_slot;
        let mut var_npexte_dn4: f64 = *var_npexte_dn4_slot;
        let mut var_npexte_dn5: f64 = *var_npexte_dn5_slot;
        let mut var_npexte_dn6: f64 = *var_npexte_dn6_slot;
        let mut var_npexte_dn7: f64 = *var_npexte_dn7_slot;
        let mut var_npexte_dn8: f64 = *var_npexte_dn8_slot;
        let mut var_npexte_dn9: f64 = *var_npexte_dn9_slot;
        let mut var_nsubpp: f64 = *var_nsubpp_slot;
        let mut var_nsubpp_dn0: f64 = *var_nsubpp_dn0_slot;
        let mut var_nsubpp_dn10: f64 = *var_nsubpp_dn10_slot;
        let mut var_nsubpp_dn11: f64 = *var_nsubpp_dn11_slot;
        let mut var_nsubpp_dn14: f64 = *var_nsubpp_dn14_slot;
        let mut var_nsubpp_dn2: f64 = *var_nsubpp_dn2_slot;
        let mut var_nsubpp_dn4: f64 = *var_nsubpp_dn4_slot;
        let mut var_nsubpp_dn5: f64 = *var_nsubpp_dn5_slot;
        let mut var_nsubpp_dn6: f64 = *var_nsubpp_dn6_slot;
        let mut var_nsubpp_dn7: f64 = *var_nsubpp_dn7_slot;
        let mut var_nsubpp_dn8: f64 = *var_nsubpp_dn8_slot;
        let mut var_nsubpp_dn9: f64 = *var_nsubpp_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_uc_wsti: f64 = *var_uc_wsti_slot;
        let mut var_uc_wsti_dn0: f64 = *var_uc_wsti_dn0_slot;
        let mut var_uc_wsti_dn10: f64 = *var_uc_wsti_dn10_slot;
        let mut var_uc_wsti_dn11: f64 = *var_uc_wsti_dn11_slot;
        let mut var_uc_wsti_dn14: f64 = *var_uc_wsti_dn14_slot;
        let mut var_uc_wsti_dn2: f64 = *var_uc_wsti_dn2_slot;
        let mut var_uc_wsti_dn4: f64 = *var_uc_wsti_dn4_slot;
        let mut var_uc_wsti_dn5: f64 = *var_uc_wsti_dn5_slot;
        let mut var_uc_wsti_dn6: f64 = *var_uc_wsti_dn6_slot;
        let mut var_uc_wsti_dn7: f64 = *var_uc_wsti_dn7_slot;
        let mut var_uc_wsti_dn8: f64 = *var_uc_wsti_dn8_slot;
        let mut var_uc_wsti_dn9: f64 = *var_uc_wsti_dn9_slot;
        let mut var_weff: f64 = *var_weff_slot;
        let mut var_weff_cv: f64 = *var_weff_cv_slot;
        let mut var_weff_ld: f64 = *var_weff_ld_slot;
        let mut var_weff_nf: f64 = *var_weff_nf_slot;
        let mut var_weffcv_nf: f64 = *var_weffcv_nf_slot;

        let (assign10930_e5826, assign10930_e5826_d_n0, assign10930_e5826_d_n2, assign10930_e5826_d_n4, assign10930_e5826_d_n5, assign10930_e5826_d_n6, assign10930_e5826_d_n7, assign10930_e5826_d_n8, assign10930_e5826_d_n9, assign10930_e5826_d_n10, assign10930_e5826_d_n11, assign10930_e5826_d_n14,) = {
    if (var_guard257 != 0.0) {
        let assign10930_e5822: f64 = (var_npexte + var_mks_nsubcdfm);
        let assign10930_e5824: f64 = (assign10930_e5822 - var_ef_nsubc);
        (assign10930_e5824, (var_npexte_dn0 - var_ef_nsubc_dn0), (var_npexte_dn2 - var_ef_nsubc_dn2), (var_npexte_dn4 - var_ef_nsubc_dn4), (var_npexte_dn5 - var_ef_nsubc_dn5), (var_npexte_dn6 - var_ef_nsubc_dn6), (var_npexte_dn7 - var_ef_nsubc_dn7), (var_npexte_dn8 - var_ef_nsubc_dn8), (var_npexte_dn9 - var_ef_nsubc_dn9), (var_npexte_dn10 - var_ef_nsubc_dn10), (var_npexte_dn11 - var_ef_nsubc_dn11), (var_npexte_dn14 - var_ef_nsubc_dn14),)
    } else {
        (var_npexte, var_npexte_dn0, var_npexte_dn2, var_npexte_dn4, var_npexte_dn5, var_npexte_dn6, var_npexte_dn7, var_npexte_dn8, var_npexte_dn9, var_npexte_dn10, var_npexte_dn11, var_npexte_dn14,)
    }
};
        var_npexte = assign10930_e5826;
        var_npexte_dn0 = assign10930_e5826_d_n0;
        var_npexte_dn2 = assign10930_e5826_d_n2;
        var_npexte_dn4 = assign10930_e5826_d_n4;
        var_npexte_dn5 = assign10930_e5826_d_n5;
        var_npexte_dn6 = assign10930_e5826_d_n6;
        var_npexte_dn7 = assign10930_e5826_d_n7;
        var_npexte_dn8 = assign10930_e5826_d_n8;
        var_npexte_dn9 = assign10930_e5826_d_n9;
        var_npexte_dn10 = assign10930_e5826_d_n10;
        var_npexte_dn11 = assign10930_e5826_d_n11;
        var_npexte_dn14 = assign10930_e5826_d_n14;

        let (assign10940_e5830, assign10940_e5830_d_n0, assign10940_e5830_d_n2, assign10940_e5830_d_n4, assign10940_e5830_d_n5, assign10940_e5830_d_n6, assign10940_e5830_d_n7, assign10940_e5830_d_n8, assign10940_e5830_d_n9, assign10940_e5830_d_n10, assign10940_e5830_d_n11, assign10940_e5830_d_n14,) = {
    if (var_guard257 != 0.0) {
        (var_mks_nsubcdfm, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ef_nsubc, var_ef_nsubc_dn0, var_ef_nsubc_dn2, var_ef_nsubc_dn4, var_ef_nsubc_dn5, var_ef_nsubc_dn6, var_ef_nsubc_dn7, var_ef_nsubc_dn8, var_ef_nsubc_dn9, var_ef_nsubc_dn10, var_ef_nsubc_dn11, var_ef_nsubc_dn14,)
    }
};
        var_ef_nsubc = assign10940_e5830;
        var_ef_nsubc_dn0 = assign10940_e5830_d_n0;
        var_ef_nsubc_dn2 = assign10940_e5830_d_n2;
        var_ef_nsubc_dn4 = assign10940_e5830_d_n4;
        var_ef_nsubc_dn5 = assign10940_e5830_d_n5;
        var_ef_nsubc_dn6 = assign10940_e5830_d_n6;
        var_ef_nsubc_dn7 = assign10940_e5830_d_n7;
        var_ef_nsubc_dn8 = assign10940_e5830_d_n8;
        var_ef_nsubc_dn9 = assign10940_e5830_d_n9;
        var_ef_nsubc_dn10 = assign10940_e5830_d_n10;
        var_ef_nsubc_dn11 = assign10940_e5830_d_n11;
        var_ef_nsubc_dn14 = assign10940_e5830_d_n14;

        let assign10950_e5836: f64 = (var_wg).powf(p.p163);
        let assign10950_e5837: f64 = (p.p162 / assign10950_e5836);
        let assign10950_e5838: f64 = (1.0 + assign10950_e5837);
        let assign10950_e5839: f64 = (var_ef_mueph1 * assign10950_e5838);
        let assign10950_e5844: f64 = (var_lg).powf(p.p165);
        let assign10950_e5845: f64 = (p.p164 / assign10950_e5844);
        let assign10950_e5846: f64 = (1.0 + assign10950_e5845);
        let assign10950_e5847: f64 = (assign10950_e5839 * assign10950_e5846);
        let assign10950_e5852: f64 = (var_wlg).powf(p.p168);
        let assign10950_e5853: f64 = (p.p167 / assign10950_e5852);
        let assign10950_e5854: f64 = (1.0 + assign10950_e5853);
        let assign10950_e5855: f64 = (assign10950_e5847 * assign10950_e5854);
        var_mueph = assign10950_e5855;
        var_mueph_dn0 = (((var_ef_mueph1_dn0 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        var_mueph_dn2 = (((var_ef_mueph1_dn2 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        var_mueph_dn4 = (((var_ef_mueph1_dn4 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        var_mueph_dn5 = (((var_ef_mueph1_dn5 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        var_mueph_dn6 = (((var_ef_mueph1_dn6 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        var_mueph_dn7 = (((var_ef_mueph1_dn7 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        var_mueph_dn8 = (((var_ef_mueph1_dn8 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        var_mueph_dn9 = (((var_ef_mueph1_dn9 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        var_mueph_dn10 = (((var_ef_mueph1_dn10 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        var_mueph_dn11 = (((var_ef_mueph1_dn11 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        var_mueph_dn14 = (((var_ef_mueph1_dn14 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);

        let assign10960_e5858: f64 = if var_lod_half > 0.0 { 1.0 } else { 0.0 };
        var_guard259 = assign10960_e5858;

        let (assign10970_e5866, assign10970_e5866_d_n0, assign10970_e5866_d_n2, assign10970_e5866_d_n4, assign10970_e5866_d_n5, assign10970_e5866_d_n6, assign10970_e5866_d_n7, assign10970_e5866_d_n8, assign10970_e5866_d_n9, assign10970_e5866_d_n10, assign10970_e5866_d_n11, assign10970_e5866_d_n14,) = {
    if (var_guard259 != 0.0) {
        let assign10970_e5863: f64 = (1.0 + var_uc_muesti2);
        let assign10970_e5864: f64 = (1.0 / assign10970_e5863);
        (assign10970_e5864, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign10970_e5866;
        var_t1_dn0 = assign10970_e5866_d_n0;
        var_t1_dn2 = assign10970_e5866_d_n2;
        var_t1_dn4 = assign10970_e5866_d_n4;
        var_t1_dn5 = assign10970_e5866_d_n5;
        var_t1_dn6 = assign10970_e5866_d_n6;
        var_t1_dn7 = assign10970_e5866_d_n7;
        var_t1_dn8 = assign10970_e5866_d_n8;
        var_t1_dn9 = assign10970_e5866_d_n9;
        var_t1_dn10 = assign10970_e5866_d_n10;
        var_t1_dn11 = assign10970_e5866_d_n11;
        var_t1_dn14 = assign10970_e5866_d_n14;

        let (assign10980_e5874, assign10980_e5874_d_n0, assign10980_e5874_d_n2, assign10980_e5874_d_n4, assign10980_e5874_d_n5, assign10980_e5874_d_n6, assign10980_e5874_d_n7, assign10980_e5874_d_n8, assign10980_e5874_d_n9, assign10980_e5874_d_n10, assign10980_e5874_d_n11, assign10980_e5874_d_n14,) = {
    if (var_guard259 != 0.0) {
        let assign10980_e5870: f64 = (var_uc_muesti1 / var_lod_half);
        let assign10980_e5872: f64 = (assign10980_e5870).powf(var_uc_muesti3);
        (assign10980_e5872, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10980_e5870).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn0) / (var_lod_half * var_lod_half))))) } } else { (assign10980_e5872 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn0) / (var_lod_half * var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10980_e5870).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn2) / (var_lod_half * var_lod_half))))) } } else { (assign10980_e5872 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn2) / (var_lod_half * var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10980_e5870).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn4) / (var_lod_half * var_lod_half))))) } } else { (assign10980_e5872 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn4) / (var_lod_half * var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10980_e5870).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn5) / (var_lod_half * var_lod_half))))) } } else { (assign10980_e5872 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn5) / (var_lod_half * var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10980_e5870).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn6) / (var_lod_half * var_lod_half))))) } } else { (assign10980_e5872 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn6) / (var_lod_half * var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10980_e5870).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn7) / (var_lod_half * var_lod_half))))) } } else { (assign10980_e5872 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn7) / (var_lod_half * var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10980_e5870).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn8) / (var_lod_half * var_lod_half))))) } } else { (assign10980_e5872 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn8) / (var_lod_half * var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10980_e5870).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn9) / (var_lod_half * var_lod_half))))) } } else { (assign10980_e5872 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn9) / (var_lod_half * var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10980_e5870).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn10) / (var_lod_half * var_lod_half))))) } } else { (assign10980_e5872 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn10) / (var_lod_half * var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10980_e5870).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn11) / (var_lod_half * var_lod_half))))) } } else { (assign10980_e5872 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn11) / (var_lod_half * var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10980_e5870).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn14) / (var_lod_half * var_lod_half))))) } } else { (assign10980_e5872 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn14) / (var_lod_half * var_lod_half))) / assign10980_e5870))) },)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign10980_e5874;
        var_t2_dn0 = assign10980_e5874_d_n0;
        var_t2_dn2 = assign10980_e5874_d_n2;
        var_t2_dn4 = assign10980_e5874_d_n4;
        var_t2_dn5 = assign10980_e5874_d_n5;
        var_t2_dn6 = assign10980_e5874_d_n6;
        var_t2_dn7 = assign10980_e5874_d_n7;
        var_t2_dn8 = assign10980_e5874_d_n8;
        var_t2_dn9 = assign10980_e5874_d_n9;
        var_t2_dn10 = assign10980_e5874_d_n10;
        var_t2_dn11 = assign10980_e5874_d_n11;
        var_t2_dn14 = assign10980_e5874_d_n14;

        let (assign10990_e5882, assign10990_e5882_d_n0, assign10990_e5882_d_n2, assign10990_e5882_d_n4, assign10990_e5882_d_n5, assign10990_e5882_d_n6, assign10990_e5882_d_n7, assign10990_e5882_d_n8, assign10990_e5882_d_n9, assign10990_e5882_d_n10, assign10990_e5882_d_n11, assign10990_e5882_d_n14,) = {
    if (var_guard259 != 0.0) {
        let assign10990_e5878: f64 = (var_uc_muesti1 / var_lod_half_ref);
        let assign10990_e5880: f64 = (assign10990_e5878).powf(var_uc_muesti3);
        (assign10990_e5880, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10990_e5878).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn0) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10990_e5880 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn0) / (var_lod_half_ref * var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10990_e5878).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn2) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10990_e5880 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn2) / (var_lod_half_ref * var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10990_e5878).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn4) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10990_e5880 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn4) / (var_lod_half_ref * var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10990_e5878).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn5) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10990_e5880 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn5) / (var_lod_half_ref * var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10990_e5878).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn6) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10990_e5880 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn6) / (var_lod_half_ref * var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10990_e5878).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn7) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10990_e5880 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn7) / (var_lod_half_ref * var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10990_e5878).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn8) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10990_e5880 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn8) / (var_lod_half_ref * var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10990_e5878).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn9) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10990_e5880 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn9) / (var_lod_half_ref * var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10990_e5878).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn10) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10990_e5880 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn10) / (var_lod_half_ref * var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10990_e5878).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn11) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10990_e5880 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn11) / (var_lod_half_ref * var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10990_e5878).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn14) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10990_e5880 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn14) / (var_lod_half_ref * var_lod_half_ref))) / assign10990_e5878))) },)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign10990_e5882;
        var_t3_dn0 = assign10990_e5882_d_n0;
        var_t3_dn2 = assign10990_e5882_d_n2;
        var_t3_dn4 = assign10990_e5882_d_n4;
        var_t3_dn5 = assign10990_e5882_d_n5;
        var_t3_dn6 = assign10990_e5882_d_n6;
        var_t3_dn7 = assign10990_e5882_d_n7;
        var_t3_dn8 = assign10990_e5882_d_n8;
        var_t3_dn9 = assign10990_e5882_d_n9;
        var_t3_dn10 = assign10990_e5882_d_n10;
        var_t3_dn11 = assign10990_e5882_d_n11;
        var_t3_dn14 = assign10990_e5882_d_n14;

        let (assign11000_e5898, assign11000_e5898_d_n0, assign11000_e5898_d_n2, assign11000_e5898_d_n4, assign11000_e5898_d_n5, assign11000_e5898_d_n6, assign11000_e5898_d_n7, assign11000_e5898_d_n8, assign11000_e5898_d_n9, assign11000_e5898_d_n10, assign11000_e5898_d_n11, assign11000_e5898_d_n14,) = {
    if (var_guard259 != 0.0) {
        let assign11000_e5888: f64 = (var_t1 * var_t2);
        let assign11000_e5889: f64 = (1.0 + assign11000_e5888);
        let assign11000_e5890: f64 = (var_mueph * assign11000_e5889);
        let assign11000_e5894: f64 = (var_t1 * var_t3);
        let assign11000_e5895: f64 = (1.0 + assign11000_e5894);
        let assign11000_e5896: f64 = (assign11000_e5890 / assign11000_e5895);
        (assign11000_e5896, (((((var_mueph_dn0 * assign11000_e5889) + (var_mueph * ((var_t1_dn0 * var_t2) + (var_t1 * var_t2_dn0)))) * assign11000_e5895) - (assign11000_e5890 * ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)))) / (assign11000_e5895 * assign11000_e5895)), (((((var_mueph_dn2 * assign11000_e5889) + (var_mueph * ((var_t1_dn2 * var_t2) + (var_t1 * var_t2_dn2)))) * assign11000_e5895) - (assign11000_e5890 * ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)))) / (assign11000_e5895 * assign11000_e5895)), (((((var_mueph_dn4 * assign11000_e5889) + (var_mueph * ((var_t1_dn4 * var_t2) + (var_t1 * var_t2_dn4)))) * assign11000_e5895) - (assign11000_e5890 * ((var_t1_dn4 * var_t3) + (var_t1 * var_t3_dn4)))) / (assign11000_e5895 * assign11000_e5895)), (((((var_mueph_dn5 * assign11000_e5889) + (var_mueph * ((var_t1_dn5 * var_t2) + (var_t1 * var_t2_dn5)))) * assign11000_e5895) - (assign11000_e5890 * ((var_t1_dn5 * var_t3) + (var_t1 * var_t3_dn5)))) / (assign11000_e5895 * assign11000_e5895)), (((((var_mueph_dn6 * assign11000_e5889) + (var_mueph * ((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6)))) * assign11000_e5895) - (assign11000_e5890 * ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)))) / (assign11000_e5895 * assign11000_e5895)), (((((var_mueph_dn7 * assign11000_e5889) + (var_mueph * ((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7)))) * assign11000_e5895) - (assign11000_e5890 * ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)))) / (assign11000_e5895 * assign11000_e5895)), (((((var_mueph_dn8 * assign11000_e5889) + (var_mueph * ((var_t1_dn8 * var_t2) + (var_t1 * var_t2_dn8)))) * assign11000_e5895) - (assign11000_e5890 * ((var_t1_dn8 * var_t3) + (var_t1 * var_t3_dn8)))) / (assign11000_e5895 * assign11000_e5895)), (((((var_mueph_dn9 * assign11000_e5889) + (var_mueph * ((var_t1_dn9 * var_t2) + (var_t1 * var_t2_dn9)))) * assign11000_e5895) - (assign11000_e5890 * ((var_t1_dn9 * var_t3) + (var_t1 * var_t3_dn9)))) / (assign11000_e5895 * assign11000_e5895)), (((((var_mueph_dn10 * assign11000_e5889) + (var_mueph * ((var_t1_dn10 * var_t2) + (var_t1 * var_t2_dn10)))) * assign11000_e5895) - (assign11000_e5890 * ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)))) / (assign11000_e5895 * assign11000_e5895)), (((((var_mueph_dn11 * assign11000_e5889) + (var_mueph * ((var_t1_dn11 * var_t2) + (var_t1 * var_t2_dn11)))) * assign11000_e5895) - (assign11000_e5890 * ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)))) / (assign11000_e5895 * assign11000_e5895)), (((((var_mueph_dn14 * assign11000_e5889) + (var_mueph * ((var_t1_dn14 * var_t2) + (var_t1 * var_t2_dn14)))) * assign11000_e5895) - (assign11000_e5890 * ((var_t1_dn14 * var_t3) + (var_t1 * var_t3_dn14)))) / (assign11000_e5895 * assign11000_e5895)),)
    } else {
        (var_mueph, var_mueph_dn0, var_mueph_dn2, var_mueph_dn4, var_mueph_dn5, var_mueph_dn6, var_mueph_dn7, var_mueph_dn8, var_mueph_dn9, var_mueph_dn10, var_mueph_dn11, var_mueph_dn14,)
    }
};
        var_mueph = assign11000_e5898;
        var_mueph_dn0 = assign11000_e5898_d_n0;
        var_mueph_dn2 = assign11000_e5898_d_n2;
        var_mueph_dn4 = assign11000_e5898_d_n4;
        var_mueph_dn5 = assign11000_e5898_d_n5;
        var_mueph_dn6 = assign11000_e5898_d_n6;
        var_mueph_dn7 = assign11000_e5898_d_n7;
        var_mueph_dn8 = assign11000_e5898_d_n8;
        var_mueph_dn9 = assign11000_e5898_d_n9;
        var_mueph_dn10 = assign11000_e5898_d_n10;
        var_mueph_dn11 = assign11000_e5898_d_n11;
        var_mueph_dn14 = assign11000_e5898_d_n14;

        let assign11010_e5904: f64 = (var_lg).powf(p.p176);
        let assign11010_e5905: f64 = (p.p173 / assign11010_e5904);
        let assign11010_e5906: f64 = (1.0 + assign11010_e5905);
        let assign11010_e5907: f64 = (p.p171 * assign11010_e5906);
        let assign11010_e5912: f64 = (var_wg).powf(p.p175);
        let assign11010_e5913: f64 = (p.p174 / assign11010_e5912);
        let assign11010_e5914: f64 = (1.0 + assign11010_e5913);
        let assign11010_e5915: f64 = (assign11010_e5907 * assign11010_e5914);
        var_muesr = assign11010_e5915;

        let (assign11040_e5939, assign11040_e5939_d_n0, assign11040_e5939_d_n2, assign11040_e5939_d_n4, assign11040_e5939_d_n5, assign11040_e5939_d_n6, assign11040_e5939_d_n7, assign11040_e5939_d_n8, assign11040_e5939_d_n9, assign11040_e5939_d_n10, assign11040_e5939_d_n11, assign11040_e5939_d_n14,) = {
    if (var_mueph < 1e-25) {
        (1e-25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_mueph, var_mueph_dn0, var_mueph_dn2, var_mueph_dn4, var_mueph_dn5, var_mueph_dn6, var_mueph_dn7, var_mueph_dn8, var_mueph_dn9, var_mueph_dn10, var_mueph_dn11, var_mueph_dn14,)
    }
};
        var_mueph = assign11040_e5939;
        var_mueph_dn0 = assign11040_e5939_d_n0;
        var_mueph_dn2 = assign11040_e5939_d_n2;
        var_mueph_dn4 = assign11040_e5939_d_n4;
        var_mueph_dn5 = assign11040_e5939_d_n5;
        var_mueph_dn6 = assign11040_e5939_d_n6;
        var_mueph_dn7 = assign11040_e5939_d_n7;
        var_mueph_dn8 = assign11040_e5939_d_n8;
        var_mueph_dn9 = assign11040_e5939_d_n9;
        var_mueph_dn10 = assign11040_e5939_d_n10;
        var_mueph_dn11 = assign11040_e5939_d_n11;
        var_mueph_dn14 = assign11040_e5939_d_n14;

        let (assign11050_e5945,) = {
    if (var_muesr < 1e-25) {
        (1e-25,)
    } else {
        (var_muesr,)
    }
};
        var_muesr = assign11050_e5945;

        let assign11060_e5948: f64 = (var_lg).powf(p.p156);
        var_t1 = assign11060_e5948;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn14 = 0.0;

        let assign11070_e5951: f64 = (var_uc_ndep * var_t1);
        let assign11070_e5954: f64 = (var_t1 + p.p155);
        let assign11070_e5955: f64 = (assign11070_e5951 / assign11070_e5954);
        let assign11070_e5957: f64 = (assign11070_e5955 / 1.034943e-10);
        var_ndep_o_esi = assign11070_e5957;
        var_ndep_o_esi_dn0 = (((((var_uc_ndep * var_t1_dn0) * assign11070_e5954) - (assign11070_e5951 * var_t1_dn0)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        var_ndep_o_esi_dn2 = (((((var_uc_ndep * var_t1_dn2) * assign11070_e5954) - (assign11070_e5951 * var_t1_dn2)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        var_ndep_o_esi_dn4 = (((((var_uc_ndep * var_t1_dn4) * assign11070_e5954) - (assign11070_e5951 * var_t1_dn4)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        var_ndep_o_esi_dn5 = (((((var_uc_ndep * var_t1_dn5) * assign11070_e5954) - (assign11070_e5951 * var_t1_dn5)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        var_ndep_o_esi_dn6 = (((((var_uc_ndep * var_t1_dn6) * assign11070_e5954) - (assign11070_e5951 * var_t1_dn6)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        var_ndep_o_esi_dn7 = (((((var_uc_ndep * var_t1_dn7) * assign11070_e5954) - (assign11070_e5951 * var_t1_dn7)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        var_ndep_o_esi_dn8 = (((((var_uc_ndep * var_t1_dn8) * assign11070_e5954) - (assign11070_e5951 * var_t1_dn8)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        var_ndep_o_esi_dn9 = (((((var_uc_ndep * var_t1_dn9) * assign11070_e5954) - (assign11070_e5951 * var_t1_dn9)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        var_ndep_o_esi_dn10 = (((((var_uc_ndep * var_t1_dn10) * assign11070_e5954) - (assign11070_e5951 * var_t1_dn10)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        var_ndep_o_esi_dn11 = (((((var_uc_ndep * var_t1_dn11) * assign11070_e5954) - (assign11070_e5951 * var_t1_dn11)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        var_ndep_o_esi_dn14 = (((((var_uc_ndep * var_t1_dn14) * assign11070_e5954) - (assign11070_e5951 * var_t1_dn14)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);

        let assign11080_e5960: f64 = (var_uc_ninv / 1.034943e-10);
        var_ninv_o_esi = assign11080_e5960;

        let assign11090_e5966: f64 = (var_lg).powf(p.p321);
        let assign11090_e5967: f64 = (p.p320 / assign11090_e5966);
        let assign11090_e5968: f64 = (1.0 + assign11090_e5967);
        let assign11090_e5969: f64 = (p.p319 * assign11090_e5968);
        let assign11090_e5974: f64 = (var_wg).powf(p.p323);
        let assign11090_e5975: f64 = (p.p322 / assign11090_e5974);
        let assign11090_e5976: f64 = (1.0 + assign11090_e5975);
        let assign11090_e5977: f64 = (assign11090_e5969 * assign11090_e5976);
        var_ninvd0 = assign11090_e5977;

        let assign11100_e5982: f64 = (var_lg).powf(p.p387);
        let assign11100_e5983: f64 = (p.p386 / assign11100_e5982);
        let assign11100_e5984: f64 = (1.0 + assign11100_e5983);
        let assign11100_e5989: f64 = (var_wg).powf(p.p389);
        let assign11100_e5990: f64 = (p.p388 / assign11100_e5989);
        let assign11100_e5991: f64 = (1.0 + assign11100_e5990);
        let assign11100_e5992: f64 = (assign11100_e5984 * assign11100_e5991);
        var_t1 = assign11100_e5992;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn14 = 0.0;

        let assign11110_e5995: f64 = (p.p384 * var_t1);
        var_ninvd0cres = assign11110_e5995;
        var_ninvd0cres_dn0 = (p.p384 * var_t1_dn0);
        var_ninvd0cres_dn2 = (p.p384 * var_t1_dn2);
        var_ninvd0cres_dn4 = (p.p384 * var_t1_dn4);
        var_ninvd0cres_dn5 = (p.p384 * var_t1_dn5);
        var_ninvd0cres_dn6 = (p.p384 * var_t1_dn6);
        var_ninvd0cres_dn7 = (p.p384 * var_t1_dn7);
        var_ninvd0cres_dn8 = (p.p384 * var_t1_dn8);
        var_ninvd0cres_dn9 = (p.p384 * var_t1_dn9);
        var_ninvd0cres_dn10 = (p.p384 * var_t1_dn10);
        var_ninvd0cres_dn11 = (p.p384 * var_t1_dn11);
        var_ninvd0cres_dn14 = (p.p384 * var_t1_dn14);

        let assign11120_e5998: f64 = (p.p385 * var_t1);
        var_ninvd0hres = assign11120_e5998;
        var_ninvd0hres_dn0 = (p.p385 * var_t1_dn0);
        var_ninvd0hres_dn2 = (p.p385 * var_t1_dn2);
        var_ninvd0hres_dn4 = (p.p385 * var_t1_dn4);
        var_ninvd0hres_dn5 = (p.p385 * var_t1_dn5);
        var_ninvd0hres_dn6 = (p.p385 * var_t1_dn6);
        var_ninvd0hres_dn7 = (p.p385 * var_t1_dn7);
        var_ninvd0hres_dn8 = (p.p385 * var_t1_dn8);
        var_ninvd0hres_dn9 = (p.p385 * var_t1_dn9);
        var_ninvd0hres_dn10 = (p.p385 * var_t1_dn10);
        var_ninvd0hres_dn11 = (p.p385 * var_t1_dn11);
        var_ninvd0hres_dn14 = (p.p385 * var_t1_dn14);

        let assign11130_e6003: f64 = (var_lgate + p.p121);
        let assign11130_e6005: f64 = (assign11130_e6003).powf(p.p122);
        let assign11130_e6006: f64 = (var_mks_ll / assign11130_e6005);
        let assign11130_e6007: f64 = (p.p97 + assign11130_e6006);
        var_dl = assign11130_e6007;

        let assign11140_e6012: f64 = (var_lgate + p.p121);
        let assign11140_e6014: f64 = (assign11140_e6012).powf(p.p122);
        let assign11140_e6015: f64 = (var_mks_ll / assign11140_e6014);
        let assign11140_e6016: f64 = (var_uc_xldld + assign11140_e6015);
        var_dlld = assign11140_e6016;

        let assign11150_e6021: f64 = (var_wgate + p.p128);
        let assign11150_e6023: f64 = (assign11150_e6021).powf(p.p129);
        let assign11150_e6024: f64 = (var_mks_wl / assign11150_e6023);
        let assign11150_e6025: f64 = (p.p114 + assign11150_e6024);
        var_dw = assign11150_e6025;

        let assign11160_e6030: f64 = (var_wgate + p.p128);
        let assign11160_e6032: f64 = (assign11160_e6030).powf(p.p129);
        let assign11160_e6033: f64 = (var_mks_wl / assign11160_e6032);
        let assign11160_e6034: f64 = (p.p295 + assign11160_e6033);
        var_dwld = assign11160_e6034;

        let assign11170_e6039: f64 = (var_wgate + p.p128);
        let assign11170_e6041: f64 = (assign11170_e6039).powf(p.p129);
        let assign11170_e6042: f64 = (var_mks_wl / assign11170_e6041);
        let assign11170_e6043: f64 = (p.p115 + assign11170_e6042);
        var_dwcv = assign11170_e6043;

        let assign11180_e6047: f64 = (var_dl + var_dlld);
        let assign11180_e6048: f64 = (var_lgate - assign11180_e6047);
        var_leff = assign11180_e6048;

        let assign11210_e6060: f64 = (var_wlg).powf(p.p125);
        let assign11210_e6061: f64 = (p.p124 / assign11210_e6060);
        let assign11210_e6062: f64 = (var_lgate + assign11210_e6061);
        var_lgatesm = assign11210_e6062;

        let assign11220_e6066: f64 = (var_wlg).powf(p.p127);
        let assign11220_e6067: f64 = (var_uc_wl2 / assign11220_e6066);
        var_dvthsm = assign11220_e6067;

        let assign11230_e6072: f64 = (var_lgatesm * 1000000.0);
        let assign11230_e6074: f64 = (assign11230_e6072).powf(p.p207);
        let assign11230_e6075: f64 = (p.p206 / assign11230_e6074);
        let assign11230_e6076: f64 = (1.0 + assign11230_e6075);
        var_t1 = assign11230_e6076;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn14 = 0.0;

        let assign11240_e6081: f64 = (var_wg).powf(p.p209);
        let assign11240_e6082: f64 = (p.p208 / assign11240_e6081);
        let assign11240_e6083: f64 = (1.0 + assign11240_e6082);
        var_t2 = assign11240_e6083;
        var_t2_dn0 = 0.0;
        var_t2_dn2 = 0.0;
        var_t2_dn4 = 0.0;
        var_t2_dn5 = 0.0;
        var_t2_dn6 = 0.0;
        var_t2_dn7 = 0.0;
        var_t2_dn8 = 0.0;
        var_t2_dn9 = 0.0;
        var_t2_dn10 = 0.0;
        var_t2_dn11 = 0.0;
        var_t2_dn14 = 0.0;

        let assign11250_e6086: f64 = (var_uc_wsti * var_t1);
        let assign11250_e6088: f64 = (assign11250_e6086 * var_t2);
        var_uc_wsti = assign11250_e6088;
        var_uc_wsti_dn0 = ((((var_uc_wsti_dn0 * var_t1) + (var_uc_wsti * var_t1_dn0)) * var_t2) + (assign11250_e6086 * var_t2_dn0));
        var_uc_wsti_dn2 = ((((var_uc_wsti_dn2 * var_t1) + (var_uc_wsti * var_t1_dn2)) * var_t2) + (assign11250_e6086 * var_t2_dn2));
        var_uc_wsti_dn4 = ((((var_uc_wsti_dn4 * var_t1) + (var_uc_wsti * var_t1_dn4)) * var_t2) + (assign11250_e6086 * var_t2_dn4));
        var_uc_wsti_dn5 = ((((var_uc_wsti_dn5 * var_t1) + (var_uc_wsti * var_t1_dn5)) * var_t2) + (assign11250_e6086 * var_t2_dn5));
        var_uc_wsti_dn6 = ((((var_uc_wsti_dn6 * var_t1) + (var_uc_wsti * var_t1_dn6)) * var_t2) + (assign11250_e6086 * var_t2_dn6));
        var_uc_wsti_dn7 = ((((var_uc_wsti_dn7 * var_t1) + (var_uc_wsti * var_t1_dn7)) * var_t2) + (assign11250_e6086 * var_t2_dn7));
        var_uc_wsti_dn8 = ((((var_uc_wsti_dn8 * var_t1) + (var_uc_wsti * var_t1_dn8)) * var_t2) + (assign11250_e6086 * var_t2_dn8));
        var_uc_wsti_dn9 = ((((var_uc_wsti_dn9 * var_t1) + (var_uc_wsti * var_t1_dn9)) * var_t2) + (assign11250_e6086 * var_t2_dn9));
        var_uc_wsti_dn10 = ((((var_uc_wsti_dn10 * var_t1) + (var_uc_wsti * var_t1_dn10)) * var_t2) + (assign11250_e6086 * var_t2_dn10));
        var_uc_wsti_dn11 = ((((var_uc_wsti_dn11 * var_t1) + (var_uc_wsti * var_t1_dn11)) * var_t2) + (assign11250_e6086 * var_t2_dn11));
        var_uc_wsti_dn14 = ((((var_uc_wsti_dn14 * var_t1) + (var_uc_wsti * var_t1_dn14)) * var_t2) + (assign11250_e6086 * var_t2_dn14));

        let assign11260_e6092: f64 = (2.0 * var_dw);
        let assign11260_e6093: f64 = (var_wgate - assign11260_e6092);
        var_weff = assign11260_e6093;

        let assign11270_e6097: f64 = (2.0 * var_dwld);
        let assign11270_e6098: f64 = (var_wgate - assign11270_e6097);
        var_weff_ld = assign11270_e6098;

        let assign11280_e6102: f64 = (2.0 * var_dwcv);
        let assign11280_e6103: f64 = (var_wgate - assign11280_e6102);
        var_weff_cv = assign11280_e6103;

        let assign11350_e6127: f64 = (var_weff * p.p7);
        var_weff_nf = assign11350_e6127;

        let assign11360_e6130: f64 = (var_weff_cv * p.p7);
        var_weffcv_nf = assign11360_e6130;

        let assign11370_e6136: f64 = (var_wg).powf(p.p143);
        let assign11370_e6137: f64 = (p.p142 / assign11370_e6136);
        let assign11370_e6138: f64 = (1.0 + assign11370_e6137);
        let assign11370_e6139: f64 = (var_ef_nsubp * assign11370_e6138);
        var_nsubpp = assign11370_e6139;
        var_nsubpp_dn0 = (var_ef_nsubp_dn0 * assign11370_e6138);
        var_nsubpp_dn2 = (var_ef_nsubp_dn2 * assign11370_e6138);
        var_nsubpp_dn4 = (var_ef_nsubp_dn4 * assign11370_e6138);
        var_nsubpp_dn5 = (var_ef_nsubp_dn5 * assign11370_e6138);
        var_nsubpp_dn6 = (var_ef_nsubp_dn6 * assign11370_e6138);
        var_nsubpp_dn7 = (var_ef_nsubp_dn7 * assign11370_e6138);
        var_nsubpp_dn8 = (var_ef_nsubp_dn8 * assign11370_e6138);
        var_nsubpp_dn9 = (var_ef_nsubp_dn9 * assign11370_e6138);
        var_nsubpp_dn10 = (var_ef_nsubp_dn10 * assign11370_e6138);
        var_nsubpp_dn11 = (var_ef_nsubp_dn11 * assign11370_e6138);
        var_nsubpp_dn14 = (var_ef_nsubp_dn14 * assign11370_e6138);

        let assign11380_e6145: f64 = (var_wg).powf(p.p234);
        let assign11380_e6146: f64 = (p.p233 / assign11380_e6145);
        let assign11380_e6147: f64 = (1.0 + assign11380_e6146);
        let assign11380_e6148: f64 = (var_ef_nsubc * assign11380_e6147);
        var_ef_nsubc = assign11380_e6148;
        var_ef_nsubc_dn0 = (var_ef_nsubc_dn0 * assign11380_e6147);
        var_ef_nsubc_dn2 = (var_ef_nsubc_dn2 * assign11380_e6147);
        var_ef_nsubc_dn4 = (var_ef_nsubc_dn4 * assign11380_e6147);
        var_ef_nsubc_dn5 = (var_ef_nsubc_dn5 * assign11380_e6147);
        var_ef_nsubc_dn6 = (var_ef_nsubc_dn6 * assign11380_e6147);
        var_ef_nsubc_dn7 = (var_ef_nsubc_dn7 * assign11380_e6147);
        var_ef_nsubc_dn8 = (var_ef_nsubc_dn8 * assign11380_e6147);
        var_ef_nsubc_dn9 = (var_ef_nsubc_dn9 * assign11380_e6147);
        var_ef_nsubc_dn10 = (var_ef_nsubc_dn10 * assign11380_e6147);
        var_ef_nsubc_dn11 = (var_ef_nsubc_dn11 * assign11380_e6147);
        var_ef_nsubc_dn14 = (var_ef_nsubc_dn14 * assign11380_e6147);

        let assign11390_e6151: f64 = (var_ef_nsubc * 1e-6);
        var_t1 = assign11390_e6151;
        var_t1_dn0 = (var_ef_nsubc_dn0 * 1e-6);
        var_t1_dn2 = (var_ef_nsubc_dn2 * 1e-6);
        var_t1_dn4 = (var_ef_nsubc_dn4 * 1e-6);
        var_t1_dn5 = (var_ef_nsubc_dn5 * 1e-6);
        var_t1_dn6 = (var_ef_nsubc_dn6 * 1e-6);
        var_t1_dn7 = (var_ef_nsubc_dn7 * 1e-6);
        var_t1_dn8 = (var_ef_nsubc_dn8 * 1e-6);
        var_t1_dn9 = (var_ef_nsubc_dn9 * 1e-6);
        var_t1_dn10 = (var_ef_nsubc_dn10 * 1e-6);
        var_t1_dn11 = (var_ef_nsubc_dn11 * 1e-6);
        var_t1_dn14 = (var_ef_nsubc_dn14 * 1e-6);

        let assign11400_e6154: f64 = (var_nsubpp * 1e-6);
        var_t2 = assign11400_e6154;
        var_t2_dn0 = (var_nsubpp_dn0 * 1e-6);
        var_t2_dn2 = (var_nsubpp_dn2 * 1e-6);
        var_t2_dn4 = (var_nsubpp_dn4 * 1e-6);
        var_t2_dn5 = (var_nsubpp_dn5 * 1e-6);
        var_t2_dn6 = (var_nsubpp_dn6 * 1e-6);
        var_t2_dn7 = (var_nsubpp_dn7 * 1e-6);
        var_t2_dn8 = (var_nsubpp_dn8 * 1e-6);
        var_t2_dn9 = (var_nsubpp_dn9 * 1e-6);
        var_t2_dn10 = (var_nsubpp_dn10 * 1e-6);
        var_t2_dn11 = (var_nsubpp_dn11 * 1e-6);
        var_t2_dn14 = (var_nsubpp_dn14 * 1e-6);

        let assign11420_e6162: f64 = if var_t1 < 1000000000000000.0 { 1.0 } else { 0.0 };
        var_guard267 = assign11420_e6162;

        let (assign11430_e6166, assign11430_e6166_d_n0, assign11430_e6166_d_n2, assign11430_e6166_d_n4, assign11430_e6166_d_n5, assign11430_e6166_d_n6, assign11430_e6166_d_n7, assign11430_e6166_d_n8, assign11430_e6166_d_n9, assign11430_e6166_d_n10, assign11430_e6166_d_n11, assign11430_e6166_d_n14,) = {
    if (var_guard267 != 0.0) {
        (1000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign11430_e6166;
        var_t1_dn0 = assign11430_e6166_d_n0;
        var_t1_dn2 = assign11430_e6166_d_n2;
        var_t1_dn4 = assign11430_e6166_d_n4;
        var_t1_dn5 = assign11430_e6166_d_n5;
        var_t1_dn6 = assign11430_e6166_d_n6;
        var_t1_dn7 = assign11430_e6166_d_n7;
        var_t1_dn8 = assign11430_e6166_d_n8;
        var_t1_dn9 = assign11430_e6166_d_n9;
        var_t1_dn10 = assign11430_e6166_d_n10;
        var_t1_dn11 = assign11430_e6166_d_n11;
        var_t1_dn14 = assign11430_e6166_d_n14;

        let assign11440_e6169: f64 = (var_t1 / 1e-6);
        var_ef_nsubc = assign11440_e6169;
        var_ef_nsubc_dn0 = (var_t1_dn0 / 1e-6);
        var_ef_nsubc_dn2 = (var_t1_dn2 / 1e-6);
        var_ef_nsubc_dn4 = (var_t1_dn4 / 1e-6);
        var_ef_nsubc_dn5 = (var_t1_dn5 / 1e-6);
        var_ef_nsubc_dn6 = (var_t1_dn6 / 1e-6);
        var_ef_nsubc_dn7 = (var_t1_dn7 / 1e-6);
        var_ef_nsubc_dn8 = (var_t1_dn8 / 1e-6);
        var_ef_nsubc_dn9 = (var_t1_dn9 / 1e-6);
        var_ef_nsubc_dn10 = (var_t1_dn10 / 1e-6);
        var_ef_nsubc_dn11 = (var_t1_dn11 / 1e-6);
        var_ef_nsubc_dn14 = (var_t1_dn14 / 1e-6);

        let assign11460_e6177: f64 = if var_t2 < 1000000000000000.0 { 1.0 } else { 0.0 };
        var_guard269 = assign11460_e6177;

        *var_dl_slot = var_dl;
        *var_dlld_slot = var_dlld;
        *var_dvthsm_slot = var_dvthsm;
        *var_dw_slot = var_dw;
        *var_dwcv_slot = var_dwcv;
        *var_dwld_slot = var_dwld;
        *var_ef_nsubc_slot = var_ef_nsubc;
        *var_ef_nsubc_dn0_slot = var_ef_nsubc_dn0;
        *var_ef_nsubc_dn10_slot = var_ef_nsubc_dn10;
        *var_ef_nsubc_dn11_slot = var_ef_nsubc_dn11;
        *var_ef_nsubc_dn14_slot = var_ef_nsubc_dn14;
        *var_ef_nsubc_dn2_slot = var_ef_nsubc_dn2;
        *var_ef_nsubc_dn4_slot = var_ef_nsubc_dn4;
        *var_ef_nsubc_dn5_slot = var_ef_nsubc_dn5;
        *var_ef_nsubc_dn6_slot = var_ef_nsubc_dn6;
        *var_ef_nsubc_dn7_slot = var_ef_nsubc_dn7;
        *var_ef_nsubc_dn8_slot = var_ef_nsubc_dn8;
        *var_ef_nsubc_dn9_slot = var_ef_nsubc_dn9;
        *var_guard259_slot = var_guard259;
        *var_guard267_slot = var_guard267;
        *var_guard269_slot = var_guard269;
        *var_leff_slot = var_leff;
        *var_lgatesm_slot = var_lgatesm;
        *var_mueph_slot = var_mueph;
        *var_mueph_dn0_slot = var_mueph_dn0;
        *var_mueph_dn10_slot = var_mueph_dn10;
        *var_mueph_dn11_slot = var_mueph_dn11;
        *var_mueph_dn14_slot = var_mueph_dn14;
        *var_mueph_dn2_slot = var_mueph_dn2;
        *var_mueph_dn4_slot = var_mueph_dn4;
        *var_mueph_dn5_slot = var_mueph_dn5;
        *var_mueph_dn6_slot = var_mueph_dn6;
        *var_mueph_dn7_slot = var_mueph_dn7;
        *var_mueph_dn8_slot = var_mueph_dn8;
        *var_mueph_dn9_slot = var_mueph_dn9;
        *var_muesr_slot = var_muesr;
        *var_ndep_o_esi_slot = var_ndep_o_esi;
        *var_ndep_o_esi_dn0_slot = var_ndep_o_esi_dn0;
        *var_ndep_o_esi_dn10_slot = var_ndep_o_esi_dn10;
        *var_ndep_o_esi_dn11_slot = var_ndep_o_esi_dn11;
        *var_ndep_o_esi_dn14_slot = var_ndep_o_esi_dn14;
        *var_ndep_o_esi_dn2_slot = var_ndep_o_esi_dn2;
        *var_ndep_o_esi_dn4_slot = var_ndep_o_esi_dn4;
        *var_ndep_o_esi_dn5_slot = var_ndep_o_esi_dn5;
        *var_ndep_o_esi_dn6_slot = var_ndep_o_esi_dn6;
        *var_ndep_o_esi_dn7_slot = var_ndep_o_esi_dn7;
        *var_ndep_o_esi_dn8_slot = var_ndep_o_esi_dn8;
        *var_ndep_o_esi_dn9_slot = var_ndep_o_esi_dn9;
        *var_ninv_o_esi_slot = var_ninv_o_esi;
        *var_ninvd0_slot = var_ninvd0;
        *var_ninvd0cres_slot = var_ninvd0cres;
        *var_ninvd0cres_dn0_slot = var_ninvd0cres_dn0;
        *var_ninvd0cres_dn10_slot = var_ninvd0cres_dn10;
        *var_ninvd0cres_dn11_slot = var_ninvd0cres_dn11;
        *var_ninvd0cres_dn14_slot = var_ninvd0cres_dn14;
        *var_ninvd0cres_dn2_slot = var_ninvd0cres_dn2;
        *var_ninvd0cres_dn4_slot = var_ninvd0cres_dn4;
        *var_ninvd0cres_dn5_slot = var_ninvd0cres_dn5;
        *var_ninvd0cres_dn6_slot = var_ninvd0cres_dn6;
        *var_ninvd0cres_dn7_slot = var_ninvd0cres_dn7;
        *var_ninvd0cres_dn8_slot = var_ninvd0cres_dn8;
        *var_ninvd0cres_dn9_slot = var_ninvd0cres_dn9;
        *var_ninvd0hres_slot = var_ninvd0hres;
        *var_ninvd0hres_dn0_slot = var_ninvd0hres_dn0;
        *var_ninvd0hres_dn10_slot = var_ninvd0hres_dn10;
        *var_ninvd0hres_dn11_slot = var_ninvd0hres_dn11;
        *var_ninvd0hres_dn14_slot = var_ninvd0hres_dn14;
        *var_ninvd0hres_dn2_slot = var_ninvd0hres_dn2;
        *var_ninvd0hres_dn4_slot = var_ninvd0hres_dn4;
        *var_ninvd0hres_dn5_slot = var_ninvd0hres_dn5;
        *var_ninvd0hres_dn6_slot = var_ninvd0hres_dn6;
        *var_ninvd0hres_dn7_slot = var_ninvd0hres_dn7;
        *var_ninvd0hres_dn8_slot = var_ninvd0hres_dn8;
        *var_ninvd0hres_dn9_slot = var_ninvd0hres_dn9;
        *var_npexte_slot = var_npexte;
        *var_npexte_dn0_slot = var_npexte_dn0;
        *var_npexte_dn10_slot = var_npexte_dn10;
        *var_npexte_dn11_slot = var_npexte_dn11;
        *var_npexte_dn14_slot = var_npexte_dn14;
        *var_npexte_dn2_slot = var_npexte_dn2;
        *var_npexte_dn4_slot = var_npexte_dn4;
        *var_npexte_dn5_slot = var_npexte_dn5;
        *var_npexte_dn6_slot = var_npexte_dn6;
        *var_npexte_dn7_slot = var_npexte_dn7;
        *var_npexte_dn8_slot = var_npexte_dn8;
        *var_npexte_dn9_slot = var_npexte_dn9;
        *var_nsubpp_slot = var_nsubpp;
        *var_nsubpp_dn0_slot = var_nsubpp_dn0;
        *var_nsubpp_dn10_slot = var_nsubpp_dn10;
        *var_nsubpp_dn11_slot = var_nsubpp_dn11;
        *var_nsubpp_dn14_slot = var_nsubpp_dn14;
        *var_nsubpp_dn2_slot = var_nsubpp_dn2;
        *var_nsubpp_dn4_slot = var_nsubpp_dn4;
        *var_nsubpp_dn5_slot = var_nsubpp_dn5;
        *var_nsubpp_dn6_slot = var_nsubpp_dn6;
        *var_nsubpp_dn7_slot = var_nsubpp_dn7;
        *var_nsubpp_dn8_slot = var_nsubpp_dn8;
        *var_nsubpp_dn9_slot = var_nsubpp_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_uc_wsti_slot = var_uc_wsti;
        *var_uc_wsti_dn0_slot = var_uc_wsti_dn0;
        *var_uc_wsti_dn10_slot = var_uc_wsti_dn10;
        *var_uc_wsti_dn11_slot = var_uc_wsti_dn11;
        *var_uc_wsti_dn14_slot = var_uc_wsti_dn14;
        *var_uc_wsti_dn2_slot = var_uc_wsti_dn2;
        *var_uc_wsti_dn4_slot = var_uc_wsti_dn4;
        *var_uc_wsti_dn5_slot = var_uc_wsti_dn5;
        *var_uc_wsti_dn6_slot = var_uc_wsti_dn6;
        *var_uc_wsti_dn7_slot = var_uc_wsti_dn7;
        *var_uc_wsti_dn8_slot = var_uc_wsti_dn8;
        *var_uc_wsti_dn9_slot = var_uc_wsti_dn9;
        *var_weff_slot = var_weff;
        *var_weff_cv_slot = var_weff_cv;
        *var_weff_ld_slot = var_weff_ld;
        *var_weff_nf_slot = var_weff_nf;
        *var_weffcv_nf_slot = var_weffcv_nf;
    }

    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        var_ef_nsubc: f64,
        var_ef_nsubc_dn0: f64,
        var_ef_nsubc_dn10: f64,
        var_ef_nsubc_dn11: f64,
        var_ef_nsubc_dn14: f64,
        var_ef_nsubc_dn2: f64,
        var_ef_nsubc_dn4: f64,
        var_ef_nsubc_dn5: f64,
        var_ef_nsubc_dn6: f64,
        var_ef_nsubc_dn7: f64,
        var_ef_nsubc_dn8: f64,
        var_ef_nsubc_dn9: f64,
        var_guard269: f64,
        var_lg: f64,
        var_lgate: f64,
        var_lod_half: f64,
        var_lod_half_dn0: f64,
        var_lod_half_dn10: f64,
        var_lod_half_dn11: f64,
        var_lod_half_dn14: f64,
        var_lod_half_dn2: f64,
        var_lod_half_dn4: f64,
        var_lod_half_dn5: f64,
        var_lod_half_dn6: f64,
        var_lod_half_dn7: f64,
        var_lod_half_dn8: f64,
        var_lod_half_dn9: f64,
        var_lod_half_ref: f64,
        var_lod_half_ref_dn0: f64,
        var_lod_half_ref_dn10: f64,
        var_lod_half_ref_dn11: f64,
        var_lod_half_ref_dn14: f64,
        var_lod_half_ref_dn2: f64,
        var_lod_half_ref_dn4: f64,
        var_lod_half_ref_dn5: f64,
        var_lod_half_ref_dn6: f64,
        var_lod_half_ref_dn7: f64,
        var_lod_half_ref_dn8: f64,
        var_lod_half_ref_dn9: f64,
        var_npexte: f64,
        var_npexte_dn0: f64,
        var_npexte_dn10: f64,
        var_npexte_dn11: f64,
        var_npexte_dn14: f64,
        var_npexte_dn2: f64,
        var_npexte_dn4: f64,
        var_npexte_dn5: f64,
        var_npexte_dn6: f64,
        var_npexte_dn7: f64,
        var_npexte_dn8: f64,
        var_npexte_dn9: f64,
        var_uc_nsti: f64,
        var_uc_nsubpsti1: f64,
        var_uc_nsubpsti2: f64,
        var_uc_nsubpsti3: f64,
        var_uc_vover: f64,
        var_wlg: f64,
        var_costi00_slot: &mut f64,
        var_guard270_slot: &mut f64,
        var_guard271_slot: &mut f64,
        var_guard272_slot: &mut f64,
        var_nsti_p2_slot: &mut f64,
        var_nsub_slot: &mut f64,
        var_nsub_dn0_slot: &mut f64,
        var_nsub_dn10_slot: &mut f64,
        var_nsub_dn11_slot: &mut f64,
        var_nsub_dn14_slot: &mut f64,
        var_nsub_dn2_slot: &mut f64,
        var_nsub_dn4_slot: &mut f64,
        var_nsub_dn5_slot: &mut f64,
        var_nsub_dn6_slot: &mut f64,
        var_nsub_dn7_slot: &mut f64,
        var_nsub_dn8_slot: &mut f64,
        var_nsub_dn9_slot: &mut f64,
        var_nsubb_slot: &mut f64,
        var_nsubb_dn0_slot: &mut f64,
        var_nsubb_dn10_slot: &mut f64,
        var_nsubb_dn11_slot: &mut f64,
        var_nsubb_dn14_slot: &mut f64,
        var_nsubb_dn2_slot: &mut f64,
        var_nsubb_dn4_slot: &mut f64,
        var_nsubb_dn5_slot: &mut f64,
        var_nsubb_dn6_slot: &mut f64,
        var_nsubb_dn7_slot: &mut f64,
        var_nsubb_dn8_slot: &mut f64,
        var_nsubb_dn9_slot: &mut f64,
        var_nsubpp_slot: &mut f64,
        var_nsubpp_dn0_slot: &mut f64,
        var_nsubpp_dn10_slot: &mut f64,
        var_nsubpp_dn11_slot: &mut f64,
        var_nsubpp_dn14_slot: &mut f64,
        var_nsubpp_dn2_slot: &mut f64,
        var_nsubpp_dn4_slot: &mut f64,
        var_nsubpp_dn5_slot: &mut f64,
        var_nsubpp_dn6_slot: &mut f64,
        var_nsubpp_dn7_slot: &mut f64,
        var_nsubpp_dn8_slot: &mut f64,
        var_nsubpp_dn9_slot: &mut f64,
        var_nsubps_slot: &mut f64,
        var_nsubps_dn0_slot: &mut f64,
        var_nsubps_dn10_slot: &mut f64,
        var_nsubps_dn11_slot: &mut f64,
        var_nsubps_dn14_slot: &mut f64,
        var_nsubps_dn2_slot: &mut f64,
        var_nsubps_dn4_slot: &mut f64,
        var_nsubps_dn5_slot: &mut f64,
        var_nsubps_dn6_slot: &mut f64,
        var_nsubps_dn7_slot: &mut f64,
        var_nsubps_dn8_slot: &mut f64,
        var_nsubps_dn9_slot: &mut f64,
        var_ptovr0_slot: &mut f64,
        var_ptovr0_dn0_slot: &mut f64,
        var_ptovr0_dn10_slot: &mut f64,
        var_ptovr0_dn11_slot: &mut f64,
        var_ptovr0_dn14_slot: &mut f64,
        var_ptovr0_dn2_slot: &mut f64,
        var_ptovr0_dn4_slot: &mut f64,
        var_ptovr0_dn5_slot: &mut f64,
        var_ptovr0_dn6_slot: &mut f64,
        var_ptovr0_dn7_slot: &mut f64,
        var_ptovr0_dn8_slot: &mut f64,
        var_ptovr0_dn9_slot: &mut f64,
        var_q_nsub_slot: &mut f64,
        var_q_nsub_dn0_slot: &mut f64,
        var_q_nsub_dn10_slot: &mut f64,
        var_q_nsub_dn11_slot: &mut f64,
        var_q_nsub_dn14_slot: &mut f64,
        var_q_nsub_dn2_slot: &mut f64,
        var_q_nsub_dn4_slot: &mut f64,
        var_q_nsub_dn5_slot: &mut f64,
        var_q_nsub_dn6_slot: &mut f64,
        var_q_nsub_dn7_slot: &mut f64,
        var_q_nsub_dn8_slot: &mut f64,
        var_q_nsub_dn9_slot: &mut f64,
        var_qnsub_esi_slot: &mut f64,
        var_qnsub_esi2_slot: &mut f64,
        var_qnsub_esi2_dn0_slot: &mut f64,
        var_qnsub_esi2_dn10_slot: &mut f64,
        var_qnsub_esi2_dn11_slot: &mut f64,
        var_qnsub_esi2_dn14_slot: &mut f64,
        var_qnsub_esi2_dn2_slot: &mut f64,
        var_qnsub_esi2_dn4_slot: &mut f64,
        var_qnsub_esi2_dn5_slot: &mut f64,
        var_qnsub_esi2_dn6_slot: &mut f64,
        var_qnsub_esi2_dn7_slot: &mut f64,
        var_qnsub_esi2_dn8_slot: &mut f64,
        var_qnsub_esi2_dn9_slot: &mut f64,
        var_qnsub_esi_dn0_slot: &mut f64,
        var_qnsub_esi_dn10_slot: &mut f64,
        var_qnsub_esi_dn11_slot: &mut f64,
        var_qnsub_esi_dn14_slot: &mut f64,
        var_qnsub_esi_dn2_slot: &mut f64,
        var_qnsub_esi_dn4_slot: &mut f64,
        var_qnsub_esi_dn5_slot: &mut f64,
        var_qnsub_esi_dn6_slot: &mut f64,
        var_qnsub_esi_dn7_slot: &mut f64,
        var_qnsub_esi_dn8_slot: &mut f64,
        var_qnsub_esi_dn9_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn14_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_dn9_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn14_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_dn9_slot: &mut f64,
        var_vmax0_slot: &mut f64,
    ) {
        let mut var_costi00: f64 = *var_costi00_slot;
        let mut var_guard270: f64 = *var_guard270_slot;
        let mut var_guard271: f64 = *var_guard271_slot;
        let mut var_guard272: f64 = *var_guard272_slot;
        let mut var_nsti_p2: f64 = *var_nsti_p2_slot;
        let mut var_nsub: f64 = *var_nsub_slot;
        let mut var_nsub_dn0: f64 = *var_nsub_dn0_slot;
        let mut var_nsub_dn10: f64 = *var_nsub_dn10_slot;
        let mut var_nsub_dn11: f64 = *var_nsub_dn11_slot;
        let mut var_nsub_dn14: f64 = *var_nsub_dn14_slot;
        let mut var_nsub_dn2: f64 = *var_nsub_dn2_slot;
        let mut var_nsub_dn4: f64 = *var_nsub_dn4_slot;
        let mut var_nsub_dn5: f64 = *var_nsub_dn5_slot;
        let mut var_nsub_dn6: f64 = *var_nsub_dn6_slot;
        let mut var_nsub_dn7: f64 = *var_nsub_dn7_slot;
        let mut var_nsub_dn8: f64 = *var_nsub_dn8_slot;
        let mut var_nsub_dn9: f64 = *var_nsub_dn9_slot;
        let mut var_nsubb: f64 = *var_nsubb_slot;
        let mut var_nsubb_dn0: f64 = *var_nsubb_dn0_slot;
        let mut var_nsubb_dn10: f64 = *var_nsubb_dn10_slot;
        let mut var_nsubb_dn11: f64 = *var_nsubb_dn11_slot;
        let mut var_nsubb_dn14: f64 = *var_nsubb_dn14_slot;
        let mut var_nsubb_dn2: f64 = *var_nsubb_dn2_slot;
        let mut var_nsubb_dn4: f64 = *var_nsubb_dn4_slot;
        let mut var_nsubb_dn5: f64 = *var_nsubb_dn5_slot;
        let mut var_nsubb_dn6: f64 = *var_nsubb_dn6_slot;
        let mut var_nsubb_dn7: f64 = *var_nsubb_dn7_slot;
        let mut var_nsubb_dn8: f64 = *var_nsubb_dn8_slot;
        let mut var_nsubb_dn9: f64 = *var_nsubb_dn9_slot;
        let mut var_nsubpp: f64 = *var_nsubpp_slot;
        let mut var_nsubpp_dn0: f64 = *var_nsubpp_dn0_slot;
        let mut var_nsubpp_dn10: f64 = *var_nsubpp_dn10_slot;
        let mut var_nsubpp_dn11: f64 = *var_nsubpp_dn11_slot;
        let mut var_nsubpp_dn14: f64 = *var_nsubpp_dn14_slot;
        let mut var_nsubpp_dn2: f64 = *var_nsubpp_dn2_slot;
        let mut var_nsubpp_dn4: f64 = *var_nsubpp_dn4_slot;
        let mut var_nsubpp_dn5: f64 = *var_nsubpp_dn5_slot;
        let mut var_nsubpp_dn6: f64 = *var_nsubpp_dn6_slot;
        let mut var_nsubpp_dn7: f64 = *var_nsubpp_dn7_slot;
        let mut var_nsubpp_dn8: f64 = *var_nsubpp_dn8_slot;
        let mut var_nsubpp_dn9: f64 = *var_nsubpp_dn9_slot;
        let mut var_nsubps: f64 = *var_nsubps_slot;
        let mut var_nsubps_dn0: f64 = *var_nsubps_dn0_slot;
        let mut var_nsubps_dn10: f64 = *var_nsubps_dn10_slot;
        let mut var_nsubps_dn11: f64 = *var_nsubps_dn11_slot;
        let mut var_nsubps_dn14: f64 = *var_nsubps_dn14_slot;
        let mut var_nsubps_dn2: f64 = *var_nsubps_dn2_slot;
        let mut var_nsubps_dn4: f64 = *var_nsubps_dn4_slot;
        let mut var_nsubps_dn5: f64 = *var_nsubps_dn5_slot;
        let mut var_nsubps_dn6: f64 = *var_nsubps_dn6_slot;
        let mut var_nsubps_dn7: f64 = *var_nsubps_dn7_slot;
        let mut var_nsubps_dn8: f64 = *var_nsubps_dn8_slot;
        let mut var_nsubps_dn9: f64 = *var_nsubps_dn9_slot;
        let mut var_ptovr0: f64 = *var_ptovr0_slot;
        let mut var_ptovr0_dn0: f64 = *var_ptovr0_dn0_slot;
        let mut var_ptovr0_dn10: f64 = *var_ptovr0_dn10_slot;
        let mut var_ptovr0_dn11: f64 = *var_ptovr0_dn11_slot;
        let mut var_ptovr0_dn14: f64 = *var_ptovr0_dn14_slot;
        let mut var_ptovr0_dn2: f64 = *var_ptovr0_dn2_slot;
        let mut var_ptovr0_dn4: f64 = *var_ptovr0_dn4_slot;
        let mut var_ptovr0_dn5: f64 = *var_ptovr0_dn5_slot;
        let mut var_ptovr0_dn6: f64 = *var_ptovr0_dn6_slot;
        let mut var_ptovr0_dn7: f64 = *var_ptovr0_dn7_slot;
        let mut var_ptovr0_dn8: f64 = *var_ptovr0_dn8_slot;
        let mut var_ptovr0_dn9: f64 = *var_ptovr0_dn9_slot;
        let mut var_q_nsub: f64 = *var_q_nsub_slot;
        let mut var_q_nsub_dn0: f64 = *var_q_nsub_dn0_slot;
        let mut var_q_nsub_dn10: f64 = *var_q_nsub_dn10_slot;
        let mut var_q_nsub_dn11: f64 = *var_q_nsub_dn11_slot;
        let mut var_q_nsub_dn14: f64 = *var_q_nsub_dn14_slot;
        let mut var_q_nsub_dn2: f64 = *var_q_nsub_dn2_slot;
        let mut var_q_nsub_dn4: f64 = *var_q_nsub_dn4_slot;
        let mut var_q_nsub_dn5: f64 = *var_q_nsub_dn5_slot;
        let mut var_q_nsub_dn6: f64 = *var_q_nsub_dn6_slot;
        let mut var_q_nsub_dn7: f64 = *var_q_nsub_dn7_slot;
        let mut var_q_nsub_dn8: f64 = *var_q_nsub_dn8_slot;
        let mut var_q_nsub_dn9: f64 = *var_q_nsub_dn9_slot;
        let mut var_qnsub_esi: f64 = *var_qnsub_esi_slot;
        let mut var_qnsub_esi2: f64 = *var_qnsub_esi2_slot;
        let mut var_qnsub_esi2_dn0: f64 = *var_qnsub_esi2_dn0_slot;
        let mut var_qnsub_esi2_dn10: f64 = *var_qnsub_esi2_dn10_slot;
        let mut var_qnsub_esi2_dn11: f64 = *var_qnsub_esi2_dn11_slot;
        let mut var_qnsub_esi2_dn14: f64 = *var_qnsub_esi2_dn14_slot;
        let mut var_qnsub_esi2_dn2: f64 = *var_qnsub_esi2_dn2_slot;
        let mut var_qnsub_esi2_dn4: f64 = *var_qnsub_esi2_dn4_slot;
        let mut var_qnsub_esi2_dn5: f64 = *var_qnsub_esi2_dn5_slot;
        let mut var_qnsub_esi2_dn6: f64 = *var_qnsub_esi2_dn6_slot;
        let mut var_qnsub_esi2_dn7: f64 = *var_qnsub_esi2_dn7_slot;
        let mut var_qnsub_esi2_dn8: f64 = *var_qnsub_esi2_dn8_slot;
        let mut var_qnsub_esi2_dn9: f64 = *var_qnsub_esi2_dn9_slot;
        let mut var_qnsub_esi_dn0: f64 = *var_qnsub_esi_dn0_slot;
        let mut var_qnsub_esi_dn10: f64 = *var_qnsub_esi_dn10_slot;
        let mut var_qnsub_esi_dn11: f64 = *var_qnsub_esi_dn11_slot;
        let mut var_qnsub_esi_dn14: f64 = *var_qnsub_esi_dn14_slot;
        let mut var_qnsub_esi_dn2: f64 = *var_qnsub_esi_dn2_slot;
        let mut var_qnsub_esi_dn4: f64 = *var_qnsub_esi_dn4_slot;
        let mut var_qnsub_esi_dn5: f64 = *var_qnsub_esi_dn5_slot;
        let mut var_qnsub_esi_dn6: f64 = *var_qnsub_esi_dn6_slot;
        let mut var_qnsub_esi_dn7: f64 = *var_qnsub_esi_dn7_slot;
        let mut var_qnsub_esi_dn8: f64 = *var_qnsub_esi_dn8_slot;
        let mut var_qnsub_esi_dn9: f64 = *var_qnsub_esi_dn9_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn14: f64 = *var_tmf1_dn14_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_dn9: f64 = *var_tmf1_dn9_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn14: f64 = *var_tmf2_dn14_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_dn9: f64 = *var_tmf2_dn9_slot;
        let mut var_vmax0: f64 = *var_vmax0_slot;

        let (assign11470_e6181, assign11470_e6181_d_n0, assign11470_e6181_d_n2, assign11470_e6181_d_n4, assign11470_e6181_d_n5, assign11470_e6181_d_n6, assign11470_e6181_d_n7, assign11470_e6181_d_n8, assign11470_e6181_d_n9, assign11470_e6181_d_n10, assign11470_e6181_d_n11, assign11470_e6181_d_n14,) = {
    if (var_guard269 != 0.0) {
        (1000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign11470_e6181;
        var_t2_dn0 = assign11470_e6181_d_n0;
        var_t2_dn2 = assign11470_e6181_d_n2;
        var_t2_dn4 = assign11470_e6181_d_n4;
        var_t2_dn5 = assign11470_e6181_d_n5;
        var_t2_dn6 = assign11470_e6181_d_n6;
        var_t2_dn7 = assign11470_e6181_d_n7;
        var_t2_dn8 = assign11470_e6181_d_n8;
        var_t2_dn9 = assign11470_e6181_d_n9;
        var_t2_dn10 = assign11470_e6181_d_n10;
        var_t2_dn11 = assign11470_e6181_d_n11;
        var_t2_dn14 = assign11470_e6181_d_n14;

        let assign11480_e6184: f64 = (var_t2 / 1e-6);
        var_nsubpp = assign11480_e6184;
        var_nsubpp_dn0 = (var_t2_dn0 / 1e-6);
        var_nsubpp_dn2 = (var_t2_dn2 / 1e-6);
        var_nsubpp_dn4 = (var_t2_dn4 / 1e-6);
        var_nsubpp_dn5 = (var_t2_dn5 / 1e-6);
        var_nsubpp_dn6 = (var_t2_dn6 / 1e-6);
        var_nsubpp_dn7 = (var_t2_dn7 / 1e-6);
        var_nsubpp_dn8 = (var_t2_dn8 / 1e-6);
        var_nsubpp_dn9 = (var_t2_dn9 / 1e-6);
        var_nsubpp_dn10 = (var_t2_dn10 / 1e-6);
        var_nsubpp_dn11 = (var_t2_dn11 / 1e-6);
        var_nsubpp_dn14 = (var_t2_dn14 / 1e-6);

        let assign11490_e6187: f64 = if var_lod_half > 0.0 { 1.0 } else { 0.0 };
        var_guard270 = assign11490_e6187;

        let (assign11500_e6195, assign11500_e6195_d_n0, assign11500_e6195_d_n2, assign11500_e6195_d_n4, assign11500_e6195_d_n5, assign11500_e6195_d_n6, assign11500_e6195_d_n7, assign11500_e6195_d_n8, assign11500_e6195_d_n9, assign11500_e6195_d_n10, assign11500_e6195_d_n11, assign11500_e6195_d_n14,) = {
    if (var_guard270 != 0.0) {
        let assign11500_e6192: f64 = (1.0 + var_uc_nsubpsti2);
        let assign11500_e6193: f64 = (1.0 / assign11500_e6192);
        (assign11500_e6193, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign11500_e6195;
        var_t1_dn0 = assign11500_e6195_d_n0;
        var_t1_dn2 = assign11500_e6195_d_n2;
        var_t1_dn4 = assign11500_e6195_d_n4;
        var_t1_dn5 = assign11500_e6195_d_n5;
        var_t1_dn6 = assign11500_e6195_d_n6;
        var_t1_dn7 = assign11500_e6195_d_n7;
        var_t1_dn8 = assign11500_e6195_d_n8;
        var_t1_dn9 = assign11500_e6195_d_n9;
        var_t1_dn10 = assign11500_e6195_d_n10;
        var_t1_dn11 = assign11500_e6195_d_n11;
        var_t1_dn14 = assign11500_e6195_d_n14;

        let (assign11510_e6203, assign11510_e6203_d_n0, assign11510_e6203_d_n2, assign11510_e6203_d_n4, assign11510_e6203_d_n5, assign11510_e6203_d_n6, assign11510_e6203_d_n7, assign11510_e6203_d_n8, assign11510_e6203_d_n9, assign11510_e6203_d_n10, assign11510_e6203_d_n11, assign11510_e6203_d_n14,) = {
    if (var_guard270 != 0.0) {
        let assign11510_e6199: f64 = (var_uc_nsubpsti1 / var_lod_half);
        let assign11510_e6201: f64 = (assign11510_e6199).powf(var_uc_nsubpsti3);
        (assign11510_e6201, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11510_e6199).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn0) / (var_lod_half * var_lod_half))))) } } else { (assign11510_e6201 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn0) / (var_lod_half * var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11510_e6199).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn2) / (var_lod_half * var_lod_half))))) } } else { (assign11510_e6201 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn2) / (var_lod_half * var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11510_e6199).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn4) / (var_lod_half * var_lod_half))))) } } else { (assign11510_e6201 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn4) / (var_lod_half * var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11510_e6199).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn5) / (var_lod_half * var_lod_half))))) } } else { (assign11510_e6201 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn5) / (var_lod_half * var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11510_e6199).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn6) / (var_lod_half * var_lod_half))))) } } else { (assign11510_e6201 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn6) / (var_lod_half * var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11510_e6199).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn7) / (var_lod_half * var_lod_half))))) } } else { (assign11510_e6201 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn7) / (var_lod_half * var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11510_e6199).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn8) / (var_lod_half * var_lod_half))))) } } else { (assign11510_e6201 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn8) / (var_lod_half * var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11510_e6199).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn9) / (var_lod_half * var_lod_half))))) } } else { (assign11510_e6201 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn9) / (var_lod_half * var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11510_e6199).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn10) / (var_lod_half * var_lod_half))))) } } else { (assign11510_e6201 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn10) / (var_lod_half * var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11510_e6199).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn11) / (var_lod_half * var_lod_half))))) } } else { (assign11510_e6201 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn11) / (var_lod_half * var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11510_e6199).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn14) / (var_lod_half * var_lod_half))))) } } else { (assign11510_e6201 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn14) / (var_lod_half * var_lod_half))) / assign11510_e6199))) },)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign11510_e6203;
        var_t2_dn0 = assign11510_e6203_d_n0;
        var_t2_dn2 = assign11510_e6203_d_n2;
        var_t2_dn4 = assign11510_e6203_d_n4;
        var_t2_dn5 = assign11510_e6203_d_n5;
        var_t2_dn6 = assign11510_e6203_d_n6;
        var_t2_dn7 = assign11510_e6203_d_n7;
        var_t2_dn8 = assign11510_e6203_d_n8;
        var_t2_dn9 = assign11510_e6203_d_n9;
        var_t2_dn10 = assign11510_e6203_d_n10;
        var_t2_dn11 = assign11510_e6203_d_n11;
        var_t2_dn14 = assign11510_e6203_d_n14;

        let (assign11520_e6211, assign11520_e6211_d_n0, assign11520_e6211_d_n2, assign11520_e6211_d_n4, assign11520_e6211_d_n5, assign11520_e6211_d_n6, assign11520_e6211_d_n7, assign11520_e6211_d_n8, assign11520_e6211_d_n9, assign11520_e6211_d_n10, assign11520_e6211_d_n11, assign11520_e6211_d_n14,) = {
    if (var_guard270 != 0.0) {
        let assign11520_e6207: f64 = (var_uc_nsubpsti1 / var_lod_half_ref);
        let assign11520_e6209: f64 = (assign11520_e6207).powf(var_uc_nsubpsti3);
        (assign11520_e6209, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11520_e6207).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn0) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11520_e6209 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn0) / (var_lod_half_ref * var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11520_e6207).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn2) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11520_e6209 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn2) / (var_lod_half_ref * var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11520_e6207).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn4) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11520_e6209 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn4) / (var_lod_half_ref * var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11520_e6207).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn5) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11520_e6209 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn5) / (var_lod_half_ref * var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11520_e6207).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn6) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11520_e6209 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn6) / (var_lod_half_ref * var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11520_e6207).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn7) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11520_e6209 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn7) / (var_lod_half_ref * var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11520_e6207).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn8) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11520_e6209 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn8) / (var_lod_half_ref * var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11520_e6207).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn9) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11520_e6209 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn9) / (var_lod_half_ref * var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11520_e6207).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn10) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11520_e6209 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn10) / (var_lod_half_ref * var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11520_e6207).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn11) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11520_e6209 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn11) / (var_lod_half_ref * var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11520_e6207).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn14) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11520_e6209 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn14) / (var_lod_half_ref * var_lod_half_ref))) / assign11520_e6207))) },)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign11520_e6211;
        var_t3_dn0 = assign11520_e6211_d_n0;
        var_t3_dn2 = assign11520_e6211_d_n2;
        var_t3_dn4 = assign11520_e6211_d_n4;
        var_t3_dn5 = assign11520_e6211_d_n5;
        var_t3_dn6 = assign11520_e6211_d_n6;
        var_t3_dn7 = assign11520_e6211_d_n7;
        var_t3_dn8 = assign11520_e6211_d_n8;
        var_t3_dn9 = assign11520_e6211_d_n9;
        var_t3_dn10 = assign11520_e6211_d_n10;
        var_t3_dn11 = assign11520_e6211_d_n11;
        var_t3_dn14 = assign11520_e6211_d_n14;

        let (assign11530_e6227, assign11530_e6227_d_n0, assign11530_e6227_d_n2, assign11530_e6227_d_n4, assign11530_e6227_d_n5, assign11530_e6227_d_n6, assign11530_e6227_d_n7, assign11530_e6227_d_n8, assign11530_e6227_d_n9, assign11530_e6227_d_n10, assign11530_e6227_d_n11, assign11530_e6227_d_n14,) = {
    if (var_guard270 != 0.0) {
        let assign11530_e6217: f64 = (var_t1 * var_t2);
        let assign11530_e6218: f64 = (1.0 + assign11530_e6217);
        let assign11530_e6219: f64 = (var_nsubpp * assign11530_e6218);
        let assign11530_e6223: f64 = (var_t1 * var_t3);
        let assign11530_e6224: f64 = (1.0 + assign11530_e6223);
        let assign11530_e6225: f64 = (assign11530_e6219 / assign11530_e6224);
        (assign11530_e6225, (((((var_nsubpp_dn0 * assign11530_e6218) + (var_nsubpp * ((var_t1_dn0 * var_t2) + (var_t1 * var_t2_dn0)))) * assign11530_e6224) - (assign11530_e6219 * ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)))) / (assign11530_e6224 * assign11530_e6224)), (((((var_nsubpp_dn2 * assign11530_e6218) + (var_nsubpp * ((var_t1_dn2 * var_t2) + (var_t1 * var_t2_dn2)))) * assign11530_e6224) - (assign11530_e6219 * ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)))) / (assign11530_e6224 * assign11530_e6224)), (((((var_nsubpp_dn4 * assign11530_e6218) + (var_nsubpp * ((var_t1_dn4 * var_t2) + (var_t1 * var_t2_dn4)))) * assign11530_e6224) - (assign11530_e6219 * ((var_t1_dn4 * var_t3) + (var_t1 * var_t3_dn4)))) / (assign11530_e6224 * assign11530_e6224)), (((((var_nsubpp_dn5 * assign11530_e6218) + (var_nsubpp * ((var_t1_dn5 * var_t2) + (var_t1 * var_t2_dn5)))) * assign11530_e6224) - (assign11530_e6219 * ((var_t1_dn5 * var_t3) + (var_t1 * var_t3_dn5)))) / (assign11530_e6224 * assign11530_e6224)), (((((var_nsubpp_dn6 * assign11530_e6218) + (var_nsubpp * ((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6)))) * assign11530_e6224) - (assign11530_e6219 * ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)))) / (assign11530_e6224 * assign11530_e6224)), (((((var_nsubpp_dn7 * assign11530_e6218) + (var_nsubpp * ((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7)))) * assign11530_e6224) - (assign11530_e6219 * ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)))) / (assign11530_e6224 * assign11530_e6224)), (((((var_nsubpp_dn8 * assign11530_e6218) + (var_nsubpp * ((var_t1_dn8 * var_t2) + (var_t1 * var_t2_dn8)))) * assign11530_e6224) - (assign11530_e6219 * ((var_t1_dn8 * var_t3) + (var_t1 * var_t3_dn8)))) / (assign11530_e6224 * assign11530_e6224)), (((((var_nsubpp_dn9 * assign11530_e6218) + (var_nsubpp * ((var_t1_dn9 * var_t2) + (var_t1 * var_t2_dn9)))) * assign11530_e6224) - (assign11530_e6219 * ((var_t1_dn9 * var_t3) + (var_t1 * var_t3_dn9)))) / (assign11530_e6224 * assign11530_e6224)), (((((var_nsubpp_dn10 * assign11530_e6218) + (var_nsubpp * ((var_t1_dn10 * var_t2) + (var_t1 * var_t2_dn10)))) * assign11530_e6224) - (assign11530_e6219 * ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)))) / (assign11530_e6224 * assign11530_e6224)), (((((var_nsubpp_dn11 * assign11530_e6218) + (var_nsubpp * ((var_t1_dn11 * var_t2) + (var_t1 * var_t2_dn11)))) * assign11530_e6224) - (assign11530_e6219 * ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)))) / (assign11530_e6224 * assign11530_e6224)), (((((var_nsubpp_dn14 * assign11530_e6218) + (var_nsubpp * ((var_t1_dn14 * var_t2) + (var_t1 * var_t2_dn14)))) * assign11530_e6224) - (assign11530_e6219 * ((var_t1_dn14 * var_t3) + (var_t1 * var_t3_dn14)))) / (assign11530_e6224 * assign11530_e6224)),)
    } else {
        (var_nsubps, var_nsubps_dn0, var_nsubps_dn2, var_nsubps_dn4, var_nsubps_dn5, var_nsubps_dn6, var_nsubps_dn7, var_nsubps_dn8, var_nsubps_dn9, var_nsubps_dn10, var_nsubps_dn11, var_nsubps_dn14,)
    }
};
        var_nsubps = assign11530_e6227;
        var_nsubps_dn0 = assign11530_e6227_d_n0;
        var_nsubps_dn2 = assign11530_e6227_d_n2;
        var_nsubps_dn4 = assign11530_e6227_d_n4;
        var_nsubps_dn5 = assign11530_e6227_d_n5;
        var_nsubps_dn6 = assign11530_e6227_d_n6;
        var_nsubps_dn7 = assign11530_e6227_d_n7;
        var_nsubps_dn8 = assign11530_e6227_d_n8;
        var_nsubps_dn9 = assign11530_e6227_d_n9;
        var_nsubps_dn10 = assign11530_e6227_d_n10;
        var_nsubps_dn11 = assign11530_e6227_d_n11;
        var_nsubps_dn14 = assign11530_e6227_d_n14;

        let (assign11540_e6232, assign11540_e6232_d_n0, assign11540_e6232_d_n2, assign11540_e6232_d_n4, assign11540_e6232_d_n5, assign11540_e6232_d_n6, assign11540_e6232_d_n7, assign11540_e6232_d_n8, assign11540_e6232_d_n9, assign11540_e6232_d_n10, assign11540_e6232_d_n11, assign11540_e6232_d_n14,) = {
    if (var_guard270 == 0.0) {
        (var_nsubpp, var_nsubpp_dn0, var_nsubpp_dn2, var_nsubpp_dn4, var_nsubpp_dn5, var_nsubpp_dn6, var_nsubpp_dn7, var_nsubpp_dn8, var_nsubpp_dn9, var_nsubpp_dn10, var_nsubpp_dn11, var_nsubpp_dn14,)
    } else {
        (var_nsubps, var_nsubps_dn0, var_nsubps_dn2, var_nsubps_dn4, var_nsubps_dn5, var_nsubps_dn6, var_nsubps_dn7, var_nsubps_dn8, var_nsubps_dn9, var_nsubps_dn10, var_nsubps_dn11, var_nsubps_dn14,)
    }
};
        var_nsubps = assign11540_e6232;
        var_nsubps_dn0 = assign11540_e6232_d_n0;
        var_nsubps_dn2 = assign11540_e6232_d_n2;
        var_nsubps_dn4 = assign11540_e6232_d_n4;
        var_nsubps_dn5 = assign11540_e6232_d_n5;
        var_nsubps_dn6 = assign11540_e6232_d_n6;
        var_nsubps_dn7 = assign11540_e6232_d_n7;
        var_nsubps_dn8 = assign11540_e6232_d_n8;
        var_nsubps_dn9 = assign11540_e6232_d_n9;
        var_nsubps_dn10 = assign11540_e6232_d_n10;
        var_nsubps_dn11 = assign11540_e6232_d_n11;
        var_nsubps_dn14 = assign11540_e6232_d_n14;

        let assign11550_e6239: f64 = if ((var_lgate > p.p140) || (p.p140 <= 0.0)) { 1.0 } else { 0.0 };
        var_guard271 = assign11550_e6239;

        let (assign11560_e6253, assign11560_e6253_d_n0, assign11560_e6253_d_n2, assign11560_e6253_d_n4, assign11560_e6253_d_n5, assign11560_e6253_d_n6, assign11560_e6253_d_n7, assign11560_e6253_d_n8, assign11560_e6253_d_n9, assign11560_e6253_d_n10, assign11560_e6253_d_n11, assign11560_e6253_d_n14,) = {
    if (var_guard271 != 0.0) {
        let assign11560_e6244: f64 = (var_lgate - p.p140);
        let assign11560_e6245: f64 = (var_ef_nsubc * assign11560_e6244);
        let assign11560_e6248: f64 = (var_nsubps * p.p140);
        let assign11560_e6249: f64 = (assign11560_e6245 + assign11560_e6248);
        let assign11560_e6251: f64 = (assign11560_e6249 / var_lgate);
        (assign11560_e6251, (((var_ef_nsubc_dn0 * assign11560_e6244) + (var_nsubps_dn0 * p.p140)) / var_lgate), (((var_ef_nsubc_dn2 * assign11560_e6244) + (var_nsubps_dn2 * p.p140)) / var_lgate), (((var_ef_nsubc_dn4 * assign11560_e6244) + (var_nsubps_dn4 * p.p140)) / var_lgate), (((var_ef_nsubc_dn5 * assign11560_e6244) + (var_nsubps_dn5 * p.p140)) / var_lgate), (((var_ef_nsubc_dn6 * assign11560_e6244) + (var_nsubps_dn6 * p.p140)) / var_lgate), (((var_ef_nsubc_dn7 * assign11560_e6244) + (var_nsubps_dn7 * p.p140)) / var_lgate), (((var_ef_nsubc_dn8 * assign11560_e6244) + (var_nsubps_dn8 * p.p140)) / var_lgate), (((var_ef_nsubc_dn9 * assign11560_e6244) + (var_nsubps_dn9 * p.p140)) / var_lgate), (((var_ef_nsubc_dn10 * assign11560_e6244) + (var_nsubps_dn10 * p.p140)) / var_lgate), (((var_ef_nsubc_dn11 * assign11560_e6244) + (var_nsubps_dn11 * p.p140)) / var_lgate), (((var_ef_nsubc_dn14 * assign11560_e6244) + (var_nsubps_dn14 * p.p140)) / var_lgate),)
    } else {
        (var_nsub, var_nsub_dn0, var_nsub_dn2, var_nsub_dn4, var_nsub_dn5, var_nsub_dn6, var_nsub_dn7, var_nsub_dn8, var_nsub_dn9, var_nsub_dn10, var_nsub_dn11, var_nsub_dn14,)
    }
};
        var_nsub = assign11560_e6253;
        var_nsub_dn0 = assign11560_e6253_d_n0;
        var_nsub_dn2 = assign11560_e6253_d_n2;
        var_nsub_dn4 = assign11560_e6253_d_n4;
        var_nsub_dn5 = assign11560_e6253_d_n5;
        var_nsub_dn6 = assign11560_e6253_d_n6;
        var_nsub_dn7 = assign11560_e6253_d_n7;
        var_nsub_dn8 = assign11560_e6253_d_n8;
        var_nsub_dn9 = assign11560_e6253_d_n9;
        var_nsub_dn10 = assign11560_e6253_d_n10;
        var_nsub_dn11 = assign11560_e6253_d_n11;
        var_nsub_dn14 = assign11560_e6253_d_n14;

        let (assign11570_e6268, assign11570_e6268_d_n0, assign11570_e6268_d_n2, assign11570_e6268_d_n4, assign11570_e6268_d_n5, assign11570_e6268_d_n6, assign11570_e6268_d_n7, assign11570_e6268_d_n8, assign11570_e6268_d_n9, assign11570_e6268_d_n10, assign11570_e6268_d_n11, assign11570_e6268_d_n14,) = {
    if (var_guard271 == 0.0) {
        let assign11570_e6259: f64 = (var_nsubps - var_ef_nsubc);
        let assign11570_e6262: f64 = (p.p140 - var_lgate);
        let assign11570_e6263: f64 = (assign11570_e6259 * assign11570_e6262);
        let assign11570_e6265: f64 = (assign11570_e6263 / p.p140);
        let assign11570_e6266: f64 = (var_nsubps + assign11570_e6265);
        (assign11570_e6266, (var_nsubps_dn0 + (((var_nsubps_dn0 - var_ef_nsubc_dn0) * assign11570_e6262) / p.p140)), (var_nsubps_dn2 + (((var_nsubps_dn2 - var_ef_nsubc_dn2) * assign11570_e6262) / p.p140)), (var_nsubps_dn4 + (((var_nsubps_dn4 - var_ef_nsubc_dn4) * assign11570_e6262) / p.p140)), (var_nsubps_dn5 + (((var_nsubps_dn5 - var_ef_nsubc_dn5) * assign11570_e6262) / p.p140)), (var_nsubps_dn6 + (((var_nsubps_dn6 - var_ef_nsubc_dn6) * assign11570_e6262) / p.p140)), (var_nsubps_dn7 + (((var_nsubps_dn7 - var_ef_nsubc_dn7) * assign11570_e6262) / p.p140)), (var_nsubps_dn8 + (((var_nsubps_dn8 - var_ef_nsubc_dn8) * assign11570_e6262) / p.p140)), (var_nsubps_dn9 + (((var_nsubps_dn9 - var_ef_nsubc_dn9) * assign11570_e6262) / p.p140)), (var_nsubps_dn10 + (((var_nsubps_dn10 - var_ef_nsubc_dn10) * assign11570_e6262) / p.p140)), (var_nsubps_dn11 + (((var_nsubps_dn11 - var_ef_nsubc_dn11) * assign11570_e6262) / p.p140)), (var_nsubps_dn14 + (((var_nsubps_dn14 - var_ef_nsubc_dn14) * assign11570_e6262) / p.p140)),)
    } else {
        (var_nsub, var_nsub_dn0, var_nsub_dn2, var_nsub_dn4, var_nsub_dn5, var_nsub_dn6, var_nsub_dn7, var_nsub_dn8, var_nsub_dn9, var_nsub_dn10, var_nsub_dn11, var_nsub_dn14,)
    }
};
        var_nsub = assign11570_e6268;
        var_nsub_dn0 = assign11570_e6268_d_n0;
        var_nsub_dn2 = assign11570_e6268_d_n2;
        var_nsub_dn4 = assign11570_e6268_d_n4;
        var_nsub_dn5 = assign11570_e6268_d_n5;
        var_nsub_dn6 = assign11570_e6268_d_n6;
        var_nsub_dn7 = assign11570_e6268_d_n7;
        var_nsub_dn8 = assign11570_e6268_d_n8;
        var_nsub_dn9 = assign11570_e6268_d_n9;
        var_nsub_dn10 = assign11570_e6268_d_n10;
        var_nsub_dn11 = assign11570_e6268_d_n11;
        var_nsub_dn14 = assign11570_e6268_d_n14;

        let assign11580_e6271: f64 = (0.5 * var_lgate);
        let assign11580_e6273: f64 = (assign11580_e6271 - p.p140);
        var_t3 = assign11580_e6273;
        var_t3_dn0 = 0.0;
        var_t3_dn2 = 0.0;
        var_t3_dn4 = 0.0;
        var_t3_dn5 = 0.0;
        var_t3_dn6 = 0.0;
        var_t3_dn7 = 0.0;
        var_t3_dn8 = 0.0;
        var_t3_dn9 = 0.0;
        var_t3_dn10 = 0.0;
        var_t3_dn11 = 0.0;
        var_t3_dn14 = 0.0;

        let assign11590_e6276: f64 = (var_t3 - 1e-9);
        let assign11590_e6278: f64 = (assign11590_e6276 - 1e-10);
        var_tmf1 = assign11590_e6278;
        var_tmf1_dn0 = var_t3_dn0;
        var_tmf1_dn2 = var_t3_dn2;
        var_tmf1_dn4 = var_t3_dn4;
        var_tmf1_dn5 = var_t3_dn5;
        var_tmf1_dn6 = var_t3_dn6;
        var_tmf1_dn7 = var_t3_dn7;
        var_tmf1_dn8 = var_t3_dn8;
        var_tmf1_dn9 = var_t3_dn9;
        var_tmf1_dn10 = var_t3_dn10;
        var_tmf1_dn11 = var_t3_dn11;
        var_tmf1_dn14 = var_t3_dn14;

        let assign11600_e6281: f64 = (4.0 * 1e-9);
        let assign11600_e6283: f64 = (assign11600_e6281 * 1e-10);
        var_tmf2 = assign11600_e6283;
        var_tmf2_dn0 = 0.0;
        var_tmf2_dn2 = 0.0;
        var_tmf2_dn4 = 0.0;
        var_tmf2_dn5 = 0.0;
        var_tmf2_dn6 = 0.0;
        var_tmf2_dn7 = 0.0;
        var_tmf2_dn8 = 0.0;
        var_tmf2_dn9 = 0.0;
        var_tmf2_dn10 = 0.0;
        var_tmf2_dn11 = 0.0;
        var_tmf2_dn14 = 0.0;

        let (assign11610_e6290, assign11610_e6290_d_n0, assign11610_e6290_d_n2, assign11610_e6290_d_n4, assign11610_e6290_d_n5, assign11610_e6290_d_n6, assign11610_e6290_d_n7, assign11610_e6290_d_n8, assign11610_e6290_d_n9, assign11610_e6290_d_n10, assign11610_e6290_d_n11, assign11610_e6290_d_n14,) = {
    if (var_tmf2 > 0.0) {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    } else {
        let assign11610_e6289: f64 = (-var_tmf2);
        (assign11610_e6289, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
    }
};
        var_tmf2 = assign11610_e6290;
        var_tmf2_dn0 = assign11610_e6290_d_n0;
        var_tmf2_dn2 = assign11610_e6290_d_n2;
        var_tmf2_dn4 = assign11610_e6290_d_n4;
        var_tmf2_dn5 = assign11610_e6290_d_n5;
        var_tmf2_dn6 = assign11610_e6290_d_n6;
        var_tmf2_dn7 = assign11610_e6290_d_n7;
        var_tmf2_dn8 = assign11610_e6290_d_n8;
        var_tmf2_dn9 = assign11610_e6290_d_n9;
        var_tmf2_dn10 = assign11610_e6290_d_n10;
        var_tmf2_dn11 = assign11610_e6290_d_n11;
        var_tmf2_dn14 = assign11610_e6290_d_n14;

        let assign11620_e6293: f64 = (var_tmf1 * var_tmf1);
        let assign11620_e6295: f64 = (assign11620_e6293 + var_tmf2);
        let assign11620_e6296: f64 = (assign11620_e6295).sqrt();
        var_tmf2 = assign11620_e6296;
        var_tmf2_dn0 = ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign11620_e6296));
        var_tmf2_dn2 = ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign11620_e6296));
        var_tmf2_dn4 = ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign11620_e6296));
        var_tmf2_dn5 = ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign11620_e6296));
        var_tmf2_dn6 = ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign11620_e6296));
        var_tmf2_dn7 = ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign11620_e6296));
        var_tmf2_dn8 = ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign11620_e6296));
        var_tmf2_dn9 = ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign11620_e6296));
        var_tmf2_dn10 = ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign11620_e6296));
        var_tmf2_dn11 = ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign11620_e6296));
        var_tmf2_dn14 = ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign11620_e6296));

        let assign11630_e6301: f64 = (var_tmf1 / var_tmf2);
        let assign11630_e6302: f64 = (1.0 + assign11630_e6301);
        let assign11630_e6303: f64 = (0.5 * assign11630_e6302);
        var_t0 = assign11630_e6303;
        var_t0_dn0 = (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2)));
        var_t0_dn2 = (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2)));
        var_t0_dn4 = (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2)));
        var_t0_dn5 = (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2)));
        var_t0_dn6 = (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2)));
        var_t0_dn7 = (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2)));
        var_t0_dn8 = (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2)));
        var_t0_dn9 = (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2)));
        var_t0_dn10 = (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2)));
        var_t0_dn11 = (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2)));
        var_t0_dn14 = (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2)));

        let assign11640_e6308: f64 = (var_tmf1 + var_tmf2);
        let assign11640_e6309: f64 = (0.5 * assign11640_e6308);
        let assign11640_e6310: f64 = (1e-9 + assign11640_e6309);
        var_t3 = assign11640_e6310;
        var_t3_dn0 = (0.5 * (var_tmf1_dn0 + var_tmf2_dn0));
        var_t3_dn2 = (0.5 * (var_tmf1_dn2 + var_tmf2_dn2));
        var_t3_dn4 = (0.5 * (var_tmf1_dn4 + var_tmf2_dn4));
        var_t3_dn5 = (0.5 * (var_tmf1_dn5 + var_tmf2_dn5));
        var_t3_dn6 = (0.5 * (var_tmf1_dn6 + var_tmf2_dn6));
        var_t3_dn7 = (0.5 * (var_tmf1_dn7 + var_tmf2_dn7));
        var_t3_dn8 = (0.5 * (var_tmf1_dn8 + var_tmf2_dn8));
        var_t3_dn9 = (0.5 * (var_tmf1_dn9 + var_tmf2_dn9));
        var_t3_dn10 = (0.5 * (var_tmf1_dn10 + var_tmf2_dn10));
        var_t3_dn11 = (0.5 * (var_tmf1_dn11 + var_tmf2_dn11));
        var_t3_dn14 = (0.5 * (var_tmf1_dn14 + var_tmf2_dn14));

        let assign11650_e6314: f64 = (1.0 / var_t3);
        let assign11650_e6317: f64 = (1.0 / p.p220);
        let assign11650_e6318: f64 = (assign11650_e6314 + assign11650_e6317);
        let assign11650_e6319: f64 = (1.0 / assign11650_e6318);
        var_t1 = assign11650_e6319;
        var_t1_dn0 = (-((-(var_t3_dn0 / (var_t3 * var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        var_t1_dn2 = (-((-(var_t3_dn2 / (var_t3 * var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        var_t1_dn4 = (-((-(var_t3_dn4 / (var_t3 * var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        var_t1_dn5 = (-((-(var_t3_dn5 / (var_t3 * var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        var_t1_dn6 = (-((-(var_t3_dn6 / (var_t3 * var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        var_t1_dn7 = (-((-(var_t3_dn7 / (var_t3 * var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        var_t1_dn8 = (-((-(var_t3_dn8 / (var_t3 * var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        var_t1_dn9 = (-((-(var_t3_dn9 / (var_t3 * var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        var_t1_dn10 = (-((-(var_t3_dn10 / (var_t3 * var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        var_t1_dn11 = (-((-(var_t3_dn11 / (var_t3 * var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        var_t1_dn14 = (-((-(var_t3_dn14 / (var_t3 * var_t3))) / (assign11650_e6318 * assign11650_e6318)));

        let (assign11660_e6325, assign11660_e6325_d_n0, assign11660_e6325_d_n2, assign11660_e6325_d_n4, assign11660_e6325_d_n5, assign11660_e6325_d_n6, assign11660_e6325_d_n7, assign11660_e6325_d_n8, assign11660_e6325_d_n9, assign11660_e6325_d_n10, assign11660_e6325_d_n11, assign11660_e6325_d_n14,) = {
    if (0.0 >= var_t1) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t2 = assign11660_e6325;
        var_t2_dn0 = assign11660_e6325_d_n0;
        var_t2_dn2 = assign11660_e6325_d_n2;
        var_t2_dn4 = assign11660_e6325_d_n4;
        var_t2_dn5 = assign11660_e6325_d_n5;
        var_t2_dn6 = assign11660_e6325_d_n6;
        var_t2_dn7 = assign11660_e6325_d_n7;
        var_t2_dn8 = assign11660_e6325_d_n8;
        var_t2_dn9 = assign11660_e6325_d_n9;
        var_t2_dn10 = assign11660_e6325_d_n10;
        var_t2_dn11 = assign11660_e6325_d_n11;
        var_t2_dn14 = assign11660_e6325_d_n14;

        let assign11670_e6330: f64 = (var_npexte - var_ef_nsubc);
        let assign11670_e6331: f64 = (var_t2 * assign11670_e6330);
        let assign11670_e6333: f64 = (assign11670_e6331 / var_lgate);
        let assign11670_e6334: f64 = (var_nsub + assign11670_e6333);
        var_nsub = assign11670_e6334;
        var_nsub_dn0 = (var_nsub_dn0 + (((var_t2_dn0 * assign11670_e6330) + (var_t2 * (var_npexte_dn0 - var_ef_nsubc_dn0))) / var_lgate));
        var_nsub_dn2 = (var_nsub_dn2 + (((var_t2_dn2 * assign11670_e6330) + (var_t2 * (var_npexte_dn2 - var_ef_nsubc_dn2))) / var_lgate));
        var_nsub_dn4 = (var_nsub_dn4 + (((var_t2_dn4 * assign11670_e6330) + (var_t2 * (var_npexte_dn4 - var_ef_nsubc_dn4))) / var_lgate));
        var_nsub_dn5 = (var_nsub_dn5 + (((var_t2_dn5 * assign11670_e6330) + (var_t2 * (var_npexte_dn5 - var_ef_nsubc_dn5))) / var_lgate));
        var_nsub_dn6 = (var_nsub_dn6 + (((var_t2_dn6 * assign11670_e6330) + (var_t2 * (var_npexte_dn6 - var_ef_nsubc_dn6))) / var_lgate));
        var_nsub_dn7 = (var_nsub_dn7 + (((var_t2_dn7 * assign11670_e6330) + (var_t2 * (var_npexte_dn7 - var_ef_nsubc_dn7))) / var_lgate));
        var_nsub_dn8 = (var_nsub_dn8 + (((var_t2_dn8 * assign11670_e6330) + (var_t2 * (var_npexte_dn8 - var_ef_nsubc_dn8))) / var_lgate));
        var_nsub_dn9 = (var_nsub_dn9 + (((var_t2_dn9 * assign11670_e6330) + (var_t2 * (var_npexte_dn9 - var_ef_nsubc_dn9))) / var_lgate));
        var_nsub_dn10 = (var_nsub_dn10 + (((var_t2_dn10 * assign11670_e6330) + (var_t2 * (var_npexte_dn10 - var_ef_nsubc_dn10))) / var_lgate));
        var_nsub_dn11 = (var_nsub_dn11 + (((var_t2_dn11 * assign11670_e6330) + (var_t2 * (var_npexte_dn11 - var_ef_nsubc_dn11))) / var_lgate));
        var_nsub_dn14 = (var_nsub_dn14 + (((var_t2_dn14 * assign11670_e6330) + (var_t2 * (var_npexte_dn14 - var_ef_nsubc_dn14))) / var_lgate));

        let assign11680_e6337: f64 = (1.6021918e-19 * var_nsub);
        var_q_nsub = assign11680_e6337;
        var_q_nsub_dn0 = (1.6021918e-19 * var_nsub_dn0);
        var_q_nsub_dn2 = (1.6021918e-19 * var_nsub_dn2);
        var_q_nsub_dn4 = (1.6021918e-19 * var_nsub_dn4);
        var_q_nsub_dn5 = (1.6021918e-19 * var_nsub_dn5);
        var_q_nsub_dn6 = (1.6021918e-19 * var_nsub_dn6);
        var_q_nsub_dn7 = (1.6021918e-19 * var_nsub_dn7);
        var_q_nsub_dn8 = (1.6021918e-19 * var_nsub_dn8);
        var_q_nsub_dn9 = (1.6021918e-19 * var_nsub_dn9);
        var_q_nsub_dn10 = (1.6021918e-19 * var_nsub_dn10);
        var_q_nsub_dn11 = (1.6021918e-19 * var_nsub_dn11);
        var_q_nsub_dn14 = (1.6021918e-19 * var_nsub_dn14);

        let assign11690_e6340: f64 = (var_q_nsub * 1.034943e-10);
        var_qnsub_esi = assign11690_e6340;
        var_qnsub_esi_dn0 = (var_q_nsub_dn0 * 1.034943e-10);
        var_qnsub_esi_dn2 = (var_q_nsub_dn2 * 1.034943e-10);
        var_qnsub_esi_dn4 = (var_q_nsub_dn4 * 1.034943e-10);
        var_qnsub_esi_dn5 = (var_q_nsub_dn5 * 1.034943e-10);
        var_qnsub_esi_dn6 = (var_q_nsub_dn6 * 1.034943e-10);
        var_qnsub_esi_dn7 = (var_q_nsub_dn7 * 1.034943e-10);
        var_qnsub_esi_dn8 = (var_q_nsub_dn8 * 1.034943e-10);
        var_qnsub_esi_dn9 = (var_q_nsub_dn9 * 1.034943e-10);
        var_qnsub_esi_dn10 = (var_q_nsub_dn10 * 1.034943e-10);
        var_qnsub_esi_dn11 = (var_q_nsub_dn11 * 1.034943e-10);
        var_qnsub_esi_dn14 = (var_q_nsub_dn14 * 1.034943e-10);

        let assign11700_e6343: f64 = (2.0 * var_qnsub_esi);
        var_qnsub_esi2 = assign11700_e6343;
        var_qnsub_esi2_dn0 = (2.0 * var_qnsub_esi_dn0);
        var_qnsub_esi2_dn2 = (2.0 * var_qnsub_esi_dn2);
        var_qnsub_esi2_dn4 = (2.0 * var_qnsub_esi_dn4);
        var_qnsub_esi2_dn5 = (2.0 * var_qnsub_esi_dn5);
        var_qnsub_esi2_dn6 = (2.0 * var_qnsub_esi_dn6);
        var_qnsub_esi2_dn7 = (2.0 * var_qnsub_esi_dn7);
        var_qnsub_esi2_dn8 = (2.0 * var_qnsub_esi_dn8);
        var_qnsub_esi2_dn9 = (2.0 * var_qnsub_esi_dn9);
        var_qnsub_esi2_dn10 = (2.0 * var_qnsub_esi_dn10);
        var_qnsub_esi2_dn11 = (2.0 * var_qnsub_esi_dn11);
        var_qnsub_esi2_dn14 = (2.0 * var_qnsub_esi_dn14);

        let assign11710_e6347: f64 = (2.0 * p.p140);
        let assign11710_e6352: f64 = if ((var_lgate <= assign11710_e6347) && (p.p140 > 0.0)) { 1.0 } else { 0.0 };
        var_guard272 = assign11710_e6352;

        let (assign11720_e6368, assign11720_e6368_d_n0, assign11720_e6368_d_n2, assign11720_e6368_d_n4, assign11720_e6368_d_n5, assign11720_e6368_d_n6, assign11720_e6368_d_n7, assign11720_e6368_d_n8, assign11720_e6368_d_n9, assign11720_e6368_d_n10, assign11720_e6368_d_n11, assign11720_e6368_d_n14,) = {
    if (var_guard272 != 0.0) {
        let assign11720_e6356: f64 = (2.0 * var_nsubps);
        let assign11720_e6359: f64 = (var_nsubps - var_ef_nsubc);
        let assign11720_e6361: f64 = (assign11720_e6359 * var_lgate);
        let assign11720_e6363: f64 = (assign11720_e6361 / p.p140);
        let assign11720_e6364: f64 = (assign11720_e6356 - assign11720_e6363);
        let assign11720_e6366: f64 = (assign11720_e6364 - var_ef_nsubc);
        (assign11720_e6366, (((2.0 * var_nsubps_dn0) - (((var_nsubps_dn0 - var_ef_nsubc_dn0) * var_lgate) / p.p140)) - var_ef_nsubc_dn0), (((2.0 * var_nsubps_dn2) - (((var_nsubps_dn2 - var_ef_nsubc_dn2) * var_lgate) / p.p140)) - var_ef_nsubc_dn2), (((2.0 * var_nsubps_dn4) - (((var_nsubps_dn4 - var_ef_nsubc_dn4) * var_lgate) / p.p140)) - var_ef_nsubc_dn4), (((2.0 * var_nsubps_dn5) - (((var_nsubps_dn5 - var_ef_nsubc_dn5) * var_lgate) / p.p140)) - var_ef_nsubc_dn5), (((2.0 * var_nsubps_dn6) - (((var_nsubps_dn6 - var_ef_nsubc_dn6) * var_lgate) / p.p140)) - var_ef_nsubc_dn6), (((2.0 * var_nsubps_dn7) - (((var_nsubps_dn7 - var_ef_nsubc_dn7) * var_lgate) / p.p140)) - var_ef_nsubc_dn7), (((2.0 * var_nsubps_dn8) - (((var_nsubps_dn8 - var_ef_nsubc_dn8) * var_lgate) / p.p140)) - var_ef_nsubc_dn8), (((2.0 * var_nsubps_dn9) - (((var_nsubps_dn9 - var_ef_nsubc_dn9) * var_lgate) / p.p140)) - var_ef_nsubc_dn9), (((2.0 * var_nsubps_dn10) - (((var_nsubps_dn10 - var_ef_nsubc_dn10) * var_lgate) / p.p140)) - var_ef_nsubc_dn10), (((2.0 * var_nsubps_dn11) - (((var_nsubps_dn11 - var_ef_nsubc_dn11) * var_lgate) / p.p140)) - var_ef_nsubc_dn11), (((2.0 * var_nsubps_dn14) - (((var_nsubps_dn14 - var_ef_nsubc_dn14) * var_lgate) / p.p140)) - var_ef_nsubc_dn14),)
    } else {
        (var_nsubb, var_nsubb_dn0, var_nsubb_dn2, var_nsubb_dn4, var_nsubb_dn5, var_nsubb_dn6, var_nsubb_dn7, var_nsubb_dn8, var_nsubb_dn9, var_nsubb_dn10, var_nsubb_dn11, var_nsubb_dn14,)
    }
};
        var_nsubb = assign11720_e6368;
        var_nsubb_dn0 = assign11720_e6368_d_n0;
        var_nsubb_dn2 = assign11720_e6368_d_n2;
        var_nsubb_dn4 = assign11720_e6368_d_n4;
        var_nsubb_dn5 = assign11720_e6368_d_n5;
        var_nsubb_dn6 = assign11720_e6368_d_n6;
        var_nsubb_dn7 = assign11720_e6368_d_n7;
        var_nsubb_dn8 = assign11720_e6368_d_n8;
        var_nsubb_dn9 = assign11720_e6368_d_n9;
        var_nsubb_dn10 = assign11720_e6368_d_n10;
        var_nsubb_dn11 = assign11720_e6368_d_n11;
        var_nsubb_dn14 = assign11720_e6368_d_n14;

        let (assign11730_e6375, assign11730_e6375_d_n0, assign11730_e6375_d_n2, assign11730_e6375_d_n4, assign11730_e6375_d_n5, assign11730_e6375_d_n6, assign11730_e6375_d_n7, assign11730_e6375_d_n8, assign11730_e6375_d_n9, assign11730_e6375_d_n10, assign11730_e6375_d_n11, assign11730_e6375_d_n14,) = {
    if (var_guard272 != 0.0) {
        let assign11730_e6372: f64 = (var_nsubb / var_ef_nsubc);
        let assign11730_e6373: f64 = (assign11730_e6372).ln();
        (assign11730_e6373, ((((var_nsubb_dn0 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn0)) / (var_ef_nsubc * var_ef_nsubc)) / assign11730_e6372), ((((var_nsubb_dn2 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn2)) / (var_ef_nsubc * var_ef_nsubc)) / assign11730_e6372), ((((var_nsubb_dn4 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn4)) / (var_ef_nsubc * var_ef_nsubc)) / assign11730_e6372), ((((var_nsubb_dn5 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn5)) / (var_ef_nsubc * var_ef_nsubc)) / assign11730_e6372), ((((var_nsubb_dn6 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn6)) / (var_ef_nsubc * var_ef_nsubc)) / assign11730_e6372), ((((var_nsubb_dn7 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn7)) / (var_ef_nsubc * var_ef_nsubc)) / assign11730_e6372), ((((var_nsubb_dn8 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn8)) / (var_ef_nsubc * var_ef_nsubc)) / assign11730_e6372), ((((var_nsubb_dn9 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn9)) / (var_ef_nsubc * var_ef_nsubc)) / assign11730_e6372), ((((var_nsubb_dn10 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn10)) / (var_ef_nsubc * var_ef_nsubc)) / assign11730_e6372), ((((var_nsubb_dn11 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn11)) / (var_ef_nsubc * var_ef_nsubc)) / assign11730_e6372), ((((var_nsubb_dn14 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn14)) / (var_ef_nsubc * var_ef_nsubc)) / assign11730_e6372),)
    } else {
        (var_ptovr0, var_ptovr0_dn0, var_ptovr0_dn2, var_ptovr0_dn4, var_ptovr0_dn5, var_ptovr0_dn6, var_ptovr0_dn7, var_ptovr0_dn8, var_ptovr0_dn9, var_ptovr0_dn10, var_ptovr0_dn11, var_ptovr0_dn14,)
    }
};
        var_ptovr0 = assign11730_e6375;
        var_ptovr0_dn0 = assign11730_e6375_d_n0;
        var_ptovr0_dn2 = assign11730_e6375_d_n2;
        var_ptovr0_dn4 = assign11730_e6375_d_n4;
        var_ptovr0_dn5 = assign11730_e6375_d_n5;
        var_ptovr0_dn6 = assign11730_e6375_d_n6;
        var_ptovr0_dn7 = assign11730_e6375_d_n7;
        var_ptovr0_dn8 = assign11730_e6375_d_n8;
        var_ptovr0_dn9 = assign11730_e6375_d_n9;
        var_ptovr0_dn10 = assign11730_e6375_d_n10;
        var_ptovr0_dn11 = assign11730_e6375_d_n11;
        var_ptovr0_dn14 = assign11730_e6375_d_n14;

        let (assign11740_e6380, assign11740_e6380_d_n0, assign11740_e6380_d_n2, assign11740_e6380_d_n4, assign11740_e6380_d_n5, assign11740_e6380_d_n6, assign11740_e6380_d_n7, assign11740_e6380_d_n8, assign11740_e6380_d_n9, assign11740_e6380_d_n10, assign11740_e6380_d_n11, assign11740_e6380_d_n14,) = {
    if (var_guard272 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ptovr0, var_ptovr0_dn0, var_ptovr0_dn2, var_ptovr0_dn4, var_ptovr0_dn5, var_ptovr0_dn6, var_ptovr0_dn7, var_ptovr0_dn8, var_ptovr0_dn9, var_ptovr0_dn10, var_ptovr0_dn11, var_ptovr0_dn14,)
    }
};
        var_ptovr0 = assign11740_e6380;
        var_ptovr0_dn0 = assign11740_e6380_d_n0;
        var_ptovr0_dn2 = assign11740_e6380_d_n2;
        var_ptovr0_dn4 = assign11740_e6380_d_n4;
        var_ptovr0_dn5 = assign11740_e6380_d_n5;
        var_ptovr0_dn6 = assign11740_e6380_d_n6;
        var_ptovr0_dn7 = assign11740_e6380_d_n7;
        var_ptovr0_dn8 = assign11740_e6380_d_n8;
        var_ptovr0_dn9 = assign11740_e6380_d_n9;
        var_ptovr0_dn10 = assign11740_e6380_d_n10;
        var_ptovr0_dn11 = assign11740_e6380_d_n11;
        var_ptovr0_dn14 = assign11740_e6380_d_n14;

        let assign11750_e6383: f64 = (2.0 * 1.6021918e-19);
        let assign11750_e6385: f64 = (assign11750_e6383 * var_uc_nsti);
        let assign11750_e6387: f64 = (assign11750_e6385 * 1.034943e-10);
        let assign11750_e6388: f64 = (assign11750_e6387).sqrt();
        var_costi00 = assign11750_e6388;

        let assign11760_e6392: f64 = (var_uc_nsti * var_uc_nsti);
        let assign11760_e6393: f64 = (1.0 / assign11760_e6392);
        var_nsti_p2 = assign11760_e6393;

        let assign11770_e6398: f64 = (var_lg).powf(p.p231);
        let assign11770_e6399: f64 = (var_uc_vover / assign11770_e6398);
        let assign11770_e6400: f64 = (1.0 + assign11770_e6399);
        let assign11770_e6405: f64 = (var_wlg).powf(p.p239);
        let assign11770_e6406: f64 = (p.p238 / assign11770_e6405);
        let assign11770_e6407: f64 = (1.0 + assign11770_e6406);
        let assign11770_e6408: f64 = (assign11770_e6400 * assign11770_e6407);
        var_vmax0 = assign11770_e6408;

        *var_costi00_slot = var_costi00;
        *var_guard270_slot = var_guard270;
        *var_guard271_slot = var_guard271;
        *var_guard272_slot = var_guard272;
        *var_nsti_p2_slot = var_nsti_p2;
        *var_nsub_slot = var_nsub;
        *var_nsub_dn0_slot = var_nsub_dn0;
        *var_nsub_dn10_slot = var_nsub_dn10;
        *var_nsub_dn11_slot = var_nsub_dn11;
        *var_nsub_dn14_slot = var_nsub_dn14;
        *var_nsub_dn2_slot = var_nsub_dn2;
        *var_nsub_dn4_slot = var_nsub_dn4;
        *var_nsub_dn5_slot = var_nsub_dn5;
        *var_nsub_dn6_slot = var_nsub_dn6;
        *var_nsub_dn7_slot = var_nsub_dn7;
        *var_nsub_dn8_slot = var_nsub_dn8;
        *var_nsub_dn9_slot = var_nsub_dn9;
        *var_nsubb_slot = var_nsubb;
        *var_nsubb_dn0_slot = var_nsubb_dn0;
        *var_nsubb_dn10_slot = var_nsubb_dn10;
        *var_nsubb_dn11_slot = var_nsubb_dn11;
        *var_nsubb_dn14_slot = var_nsubb_dn14;
        *var_nsubb_dn2_slot = var_nsubb_dn2;
        *var_nsubb_dn4_slot = var_nsubb_dn4;
        *var_nsubb_dn5_slot = var_nsubb_dn5;
        *var_nsubb_dn6_slot = var_nsubb_dn6;
        *var_nsubb_dn7_slot = var_nsubb_dn7;
        *var_nsubb_dn8_slot = var_nsubb_dn8;
        *var_nsubb_dn9_slot = var_nsubb_dn9;
        *var_nsubpp_slot = var_nsubpp;
        *var_nsubpp_dn0_slot = var_nsubpp_dn0;
        *var_nsubpp_dn10_slot = var_nsubpp_dn10;
        *var_nsubpp_dn11_slot = var_nsubpp_dn11;
        *var_nsubpp_dn14_slot = var_nsubpp_dn14;
        *var_nsubpp_dn2_slot = var_nsubpp_dn2;
        *var_nsubpp_dn4_slot = var_nsubpp_dn4;
        *var_nsubpp_dn5_slot = var_nsubpp_dn5;
        *var_nsubpp_dn6_slot = var_nsubpp_dn6;
        *var_nsubpp_dn7_slot = var_nsubpp_dn7;
        *var_nsubpp_dn8_slot = var_nsubpp_dn8;
        *var_nsubpp_dn9_slot = var_nsubpp_dn9;
        *var_nsubps_slot = var_nsubps;
        *var_nsubps_dn0_slot = var_nsubps_dn0;
        *var_nsubps_dn10_slot = var_nsubps_dn10;
        *var_nsubps_dn11_slot = var_nsubps_dn11;
        *var_nsubps_dn14_slot = var_nsubps_dn14;
        *var_nsubps_dn2_slot = var_nsubps_dn2;
        *var_nsubps_dn4_slot = var_nsubps_dn4;
        *var_nsubps_dn5_slot = var_nsubps_dn5;
        *var_nsubps_dn6_slot = var_nsubps_dn6;
        *var_nsubps_dn7_slot = var_nsubps_dn7;
        *var_nsubps_dn8_slot = var_nsubps_dn8;
        *var_nsubps_dn9_slot = var_nsubps_dn9;
        *var_ptovr0_slot = var_ptovr0;
        *var_ptovr0_dn0_slot = var_ptovr0_dn0;
        *var_ptovr0_dn10_slot = var_ptovr0_dn10;
        *var_ptovr0_dn11_slot = var_ptovr0_dn11;
        *var_ptovr0_dn14_slot = var_ptovr0_dn14;
        *var_ptovr0_dn2_slot = var_ptovr0_dn2;
        *var_ptovr0_dn4_slot = var_ptovr0_dn4;
        *var_ptovr0_dn5_slot = var_ptovr0_dn5;
        *var_ptovr0_dn6_slot = var_ptovr0_dn6;
        *var_ptovr0_dn7_slot = var_ptovr0_dn7;
        *var_ptovr0_dn8_slot = var_ptovr0_dn8;
        *var_ptovr0_dn9_slot = var_ptovr0_dn9;
        *var_q_nsub_slot = var_q_nsub;
        *var_q_nsub_dn0_slot = var_q_nsub_dn0;
        *var_q_nsub_dn10_slot = var_q_nsub_dn10;
        *var_q_nsub_dn11_slot = var_q_nsub_dn11;
        *var_q_nsub_dn14_slot = var_q_nsub_dn14;
        *var_q_nsub_dn2_slot = var_q_nsub_dn2;
        *var_q_nsub_dn4_slot = var_q_nsub_dn4;
        *var_q_nsub_dn5_slot = var_q_nsub_dn5;
        *var_q_nsub_dn6_slot = var_q_nsub_dn6;
        *var_q_nsub_dn7_slot = var_q_nsub_dn7;
        *var_q_nsub_dn8_slot = var_q_nsub_dn8;
        *var_q_nsub_dn9_slot = var_q_nsub_dn9;
        *var_qnsub_esi_slot = var_qnsub_esi;
        *var_qnsub_esi2_slot = var_qnsub_esi2;
        *var_qnsub_esi2_dn0_slot = var_qnsub_esi2_dn0;
        *var_qnsub_esi2_dn10_slot = var_qnsub_esi2_dn10;
        *var_qnsub_esi2_dn11_slot = var_qnsub_esi2_dn11;
        *var_qnsub_esi2_dn14_slot = var_qnsub_esi2_dn14;
        *var_qnsub_esi2_dn2_slot = var_qnsub_esi2_dn2;
        *var_qnsub_esi2_dn4_slot = var_qnsub_esi2_dn4;
        *var_qnsub_esi2_dn5_slot = var_qnsub_esi2_dn5;
        *var_qnsub_esi2_dn6_slot = var_qnsub_esi2_dn6;
        *var_qnsub_esi2_dn7_slot = var_qnsub_esi2_dn7;
        *var_qnsub_esi2_dn8_slot = var_qnsub_esi2_dn8;
        *var_qnsub_esi2_dn9_slot = var_qnsub_esi2_dn9;
        *var_qnsub_esi_dn0_slot = var_qnsub_esi_dn0;
        *var_qnsub_esi_dn10_slot = var_qnsub_esi_dn10;
        *var_qnsub_esi_dn11_slot = var_qnsub_esi_dn11;
        *var_qnsub_esi_dn14_slot = var_qnsub_esi_dn14;
        *var_qnsub_esi_dn2_slot = var_qnsub_esi_dn2;
        *var_qnsub_esi_dn4_slot = var_qnsub_esi_dn4;
        *var_qnsub_esi_dn5_slot = var_qnsub_esi_dn5;
        *var_qnsub_esi_dn6_slot = var_qnsub_esi_dn6;
        *var_qnsub_esi_dn7_slot = var_qnsub_esi_dn7;
        *var_qnsub_esi_dn8_slot = var_qnsub_esi_dn8;
        *var_qnsub_esi_dn9_slot = var_qnsub_esi_dn9;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn14_slot = var_tmf1_dn14;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_dn9_slot = var_tmf1_dn9;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn14_slot = var_tmf2_dn14;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_dn9_slot = var_tmf2_dn9;
        *var_vmax0_slot = var_vmax0;
    }

    pub(super) fn stamp_transient_block_19(
        p: &Parameters,
        var_ef_nsubc: f64,
        var_ef_nsubc_dn0: f64,
        var_ef_nsubc_dn10: f64,
        var_ef_nsubc_dn11: f64,
        var_ef_nsubc_dn14: f64,
        var_ef_nsubc_dn2: f64,
        var_ef_nsubc_dn4: f64,
        var_ef_nsubc_dn5: f64,
        var_ef_nsubc_dn6: f64,
        var_ef_nsubc_dn7: f64,
        var_ef_nsubc_dn8: f64,
        var_ef_nsubc_dn9: f64,
        var_lg: f64,
        var_lgate: f64,
        var_nsub: f64,
        var_nsub_dn0: f64,
        var_nsub_dn10: f64,
        var_nsub_dn11: f64,
        var_nsub_dn14: f64,
        var_nsub_dn2: f64,
        var_nsub_dn4: f64,
        var_nsub_dn5: f64,
        var_nsub_dn6: f64,
        var_nsub_dn7: f64,
        var_nsub_dn8: f64,
        var_nsub_dn9: f64,
        var_uc_cordrift: f64,
        var_uc_rd: f64,
        var_uc_rd23: f64,
        var_uc_rdvd: f64,
        var_uc_rs: f64,
        var_weff: f64,
        var_wlg: f64,
        var_guard273_slot: &mut f64,
        var_guard275_slot: &mut f64,
        var_guard276_slot: &mut f64,
        var_guard277_slot: &mut f64,
        var_guard278_slot: &mut f64,
        var_guard279_slot: &mut f64,
        var_guard280_slot: &mut f64,
        var_guard281_slot: &mut f64,
        var_guard282_slot: &mut f64,
        var_pb20_slot: &mut f64,
        var_pb20_dn0_slot: &mut f64,
        var_pb20_dn10_slot: &mut f64,
        var_pb20_dn11_slot: &mut f64,
        var_pb20_dn14_slot: &mut f64,
        var_pb20_dn2_slot: &mut f64,
        var_pb20_dn4_slot: &mut f64,
        var_pb20_dn5_slot: &mut f64,
        var_pb20_dn6_slot: &mut f64,
        var_pb20_dn7_slot: &mut f64,
        var_pb20_dn8_slot: &mut f64,
        var_pb20_dn9_slot: &mut f64,
        var_pb2c_slot: &mut f64,
        var_pb2c_dn0_slot: &mut f64,
        var_pb2c_dn10_slot: &mut f64,
        var_pb2c_dn11_slot: &mut f64,
        var_pb2c_dn14_slot: &mut f64,
        var_pb2c_dn2_slot: &mut f64,
        var_pb2c_dn4_slot: &mut f64,
        var_pb2c_dn5_slot: &mut f64,
        var_pb2c_dn6_slot: &mut f64,
        var_pb2c_dn7_slot: &mut f64,
        var_pb2c_dn8_slot: &mut f64,
        var_pb2c_dn9_slot: &mut f64,
        var_rd0_slot: &mut f64,
        var_rd23e_slot: &mut f64,
        var_rd23e_dn0_slot: &mut f64,
        var_rd23e_dn10_slot: &mut f64,
        var_rd23e_dn11_slot: &mut f64,
        var_rd23e_dn14_slot: &mut f64,
        var_rd23e_dn2_slot: &mut f64,
        var_rd23e_dn4_slot: &mut f64,
        var_rd23e_dn5_slot: &mut f64,
        var_rd23e_dn6_slot: &mut f64,
        var_rd23e_dn7_slot: &mut f64,
        var_rd23e_dn8_slot: &mut f64,
        var_rd23e_dn9_slot: &mut f64,
        var_rdtemp0_slot: &mut f64,
        var_rdvdtemp0_slot: &mut f64,
        var_rdvdtemp0_dn0_slot: &mut f64,
        var_rdvdtemp0_dn10_slot: &mut f64,
        var_rdvdtemp0_dn11_slot: &mut f64,
        var_rdvdtemp0_dn14_slot: &mut f64,
        var_rdvdtemp0_dn2_slot: &mut f64,
        var_rdvdtemp0_dn4_slot: &mut f64,
        var_rdvdtemp0_dn5_slot: &mut f64,
        var_rdvdtemp0_dn6_slot: &mut f64,
        var_rdvdtemp0_dn7_slot: &mut f64,
        var_rdvdtemp0_dn8_slot: &mut f64,
        var_rdvdtemp0_dn9_slot: &mut f64,
        var_rs0_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn14_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn0_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn11_slot: &mut f64,
        var_t7_dn14_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn7_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_dn9_slot: &mut f64,
        var_xmax_slot: &mut f64,
        var_xmax_s_slot: &mut f64,
    ) {
        let mut var_guard273: f64 = *var_guard273_slot;
        let mut var_guard275: f64 = *var_guard275_slot;
        let mut var_guard276: f64 = *var_guard276_slot;
        let mut var_guard277: f64 = *var_guard277_slot;
        let mut var_guard278: f64 = *var_guard278_slot;
        let mut var_guard279: f64 = *var_guard279_slot;
        let mut var_guard280: f64 = *var_guard280_slot;
        let mut var_guard281: f64 = *var_guard281_slot;
        let mut var_guard282: f64 = *var_guard282_slot;
        let mut var_pb20: f64 = *var_pb20_slot;
        let mut var_pb20_dn0: f64 = *var_pb20_dn0_slot;
        let mut var_pb20_dn10: f64 = *var_pb20_dn10_slot;
        let mut var_pb20_dn11: f64 = *var_pb20_dn11_slot;
        let mut var_pb20_dn14: f64 = *var_pb20_dn14_slot;
        let mut var_pb20_dn2: f64 = *var_pb20_dn2_slot;
        let mut var_pb20_dn4: f64 = *var_pb20_dn4_slot;
        let mut var_pb20_dn5: f64 = *var_pb20_dn5_slot;
        let mut var_pb20_dn6: f64 = *var_pb20_dn6_slot;
        let mut var_pb20_dn7: f64 = *var_pb20_dn7_slot;
        let mut var_pb20_dn8: f64 = *var_pb20_dn8_slot;
        let mut var_pb20_dn9: f64 = *var_pb20_dn9_slot;
        let mut var_pb2c: f64 = *var_pb2c_slot;
        let mut var_pb2c_dn0: f64 = *var_pb2c_dn0_slot;
        let mut var_pb2c_dn10: f64 = *var_pb2c_dn10_slot;
        let mut var_pb2c_dn11: f64 = *var_pb2c_dn11_slot;
        let mut var_pb2c_dn14: f64 = *var_pb2c_dn14_slot;
        let mut var_pb2c_dn2: f64 = *var_pb2c_dn2_slot;
        let mut var_pb2c_dn4: f64 = *var_pb2c_dn4_slot;
        let mut var_pb2c_dn5: f64 = *var_pb2c_dn5_slot;
        let mut var_pb2c_dn6: f64 = *var_pb2c_dn6_slot;
        let mut var_pb2c_dn7: f64 = *var_pb2c_dn7_slot;
        let mut var_pb2c_dn8: f64 = *var_pb2c_dn8_slot;
        let mut var_pb2c_dn9: f64 = *var_pb2c_dn9_slot;
        let mut var_rd0: f64 = *var_rd0_slot;
        let mut var_rd23e: f64 = *var_rd23e_slot;
        let mut var_rd23e_dn0: f64 = *var_rd23e_dn0_slot;
        let mut var_rd23e_dn10: f64 = *var_rd23e_dn10_slot;
        let mut var_rd23e_dn11: f64 = *var_rd23e_dn11_slot;
        let mut var_rd23e_dn14: f64 = *var_rd23e_dn14_slot;
        let mut var_rd23e_dn2: f64 = *var_rd23e_dn2_slot;
        let mut var_rd23e_dn4: f64 = *var_rd23e_dn4_slot;
        let mut var_rd23e_dn5: f64 = *var_rd23e_dn5_slot;
        let mut var_rd23e_dn6: f64 = *var_rd23e_dn6_slot;
        let mut var_rd23e_dn7: f64 = *var_rd23e_dn7_slot;
        let mut var_rd23e_dn8: f64 = *var_rd23e_dn8_slot;
        let mut var_rd23e_dn9: f64 = *var_rd23e_dn9_slot;
        let mut var_rdtemp0: f64 = *var_rdtemp0_slot;
        let mut var_rdvdtemp0: f64 = *var_rdvdtemp0_slot;
        let mut var_rdvdtemp0_dn0: f64 = *var_rdvdtemp0_dn0_slot;
        let mut var_rdvdtemp0_dn10: f64 = *var_rdvdtemp0_dn10_slot;
        let mut var_rdvdtemp0_dn11: f64 = *var_rdvdtemp0_dn11_slot;
        let mut var_rdvdtemp0_dn14: f64 = *var_rdvdtemp0_dn14_slot;
        let mut var_rdvdtemp0_dn2: f64 = *var_rdvdtemp0_dn2_slot;
        let mut var_rdvdtemp0_dn4: f64 = *var_rdvdtemp0_dn4_slot;
        let mut var_rdvdtemp0_dn5: f64 = *var_rdvdtemp0_dn5_slot;
        let mut var_rdvdtemp0_dn6: f64 = *var_rdvdtemp0_dn6_slot;
        let mut var_rdvdtemp0_dn7: f64 = *var_rdvdtemp0_dn7_slot;
        let mut var_rdvdtemp0_dn8: f64 = *var_rdvdtemp0_dn8_slot;
        let mut var_rdvdtemp0_dn9: f64 = *var_rdvdtemp0_dn9_slot;
        let mut var_rs0: f64 = *var_rs0_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn14: f64 = *var_t6_dn14_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn0: f64 = *var_t7_dn0_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn14: f64 = *var_t7_dn14_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn7: f64 = *var_t7_dn7_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_dn9: f64 = *var_t7_dn9_slot;
        let mut var_xmax: f64 = *var_xmax_slot;
        let mut var_xmax_s: f64 = *var_xmax_s_slot;

        let assign11780_e6411: f64 = (2.0 / 38.68283);
        let assign11780_e6414: f64 = (var_nsub / 1.04e16);
        let assign11780_e6415: f64 = (assign11780_e6414).ln();
        let assign11780_e6416: f64 = (assign11780_e6411 * assign11780_e6415);
        var_pb20 = assign11780_e6416;
        var_pb20_dn0 = (assign11780_e6411 * ((var_nsub_dn0 / 1.04e16) / assign11780_e6414));
        var_pb20_dn2 = (assign11780_e6411 * ((var_nsub_dn2 / 1.04e16) / assign11780_e6414));
        var_pb20_dn4 = (assign11780_e6411 * ((var_nsub_dn4 / 1.04e16) / assign11780_e6414));
        var_pb20_dn5 = (assign11780_e6411 * ((var_nsub_dn5 / 1.04e16) / assign11780_e6414));
        var_pb20_dn6 = (assign11780_e6411 * ((var_nsub_dn6 / 1.04e16) / assign11780_e6414));
        var_pb20_dn7 = (assign11780_e6411 * ((var_nsub_dn7 / 1.04e16) / assign11780_e6414));
        var_pb20_dn8 = (assign11780_e6411 * ((var_nsub_dn8 / 1.04e16) / assign11780_e6414));
        var_pb20_dn9 = (assign11780_e6411 * ((var_nsub_dn9 / 1.04e16) / assign11780_e6414));
        var_pb20_dn10 = (assign11780_e6411 * ((var_nsub_dn10 / 1.04e16) / assign11780_e6414));
        var_pb20_dn11 = (assign11780_e6411 * ((var_nsub_dn11 / 1.04e16) / assign11780_e6414));
        var_pb20_dn14 = (assign11780_e6411 * ((var_nsub_dn14 / 1.04e16) / assign11780_e6414));

        let assign11790_e6419: f64 = (2.0 / 38.68283);
        let assign11790_e6422: f64 = (var_ef_nsubc / 1.04e16);
        let assign11790_e6423: f64 = (assign11790_e6422).ln();
        let assign11790_e6424: f64 = (assign11790_e6419 * assign11790_e6423);
        var_pb2c = assign11790_e6424;
        var_pb2c_dn0 = (assign11790_e6419 * ((var_ef_nsubc_dn0 / 1.04e16) / assign11790_e6422));
        var_pb2c_dn2 = (assign11790_e6419 * ((var_ef_nsubc_dn2 / 1.04e16) / assign11790_e6422));
        var_pb2c_dn4 = (assign11790_e6419 * ((var_ef_nsubc_dn4 / 1.04e16) / assign11790_e6422));
        var_pb2c_dn5 = (assign11790_e6419 * ((var_ef_nsubc_dn5 / 1.04e16) / assign11790_e6422));
        var_pb2c_dn6 = (assign11790_e6419 * ((var_ef_nsubc_dn6 / 1.04e16) / assign11790_e6422));
        var_pb2c_dn7 = (assign11790_e6419 * ((var_ef_nsubc_dn7 / 1.04e16) / assign11790_e6422));
        var_pb2c_dn8 = (assign11790_e6419 * ((var_ef_nsubc_dn8 / 1.04e16) / assign11790_e6422));
        var_pb2c_dn9 = (assign11790_e6419 * ((var_ef_nsubc_dn9 / 1.04e16) / assign11790_e6422));
        var_pb2c_dn10 = (assign11790_e6419 * ((var_ef_nsubc_dn10 / 1.04e16) / assign11790_e6422));
        var_pb2c_dn11 = (assign11790_e6419 * ((var_ef_nsubc_dn11 / 1.04e16) / assign11790_e6422));
        var_pb2c_dn14 = (assign11790_e6419 * ((var_ef_nsubc_dn14 / 1.04e16) / assign11790_e6422));

        let assign11800_e6427: f64 = if p.p51 == 1.0 { 1.0 } else { 0.0 };
        var_guard273 = assign11800_e6427;

        let (assign11810_e6437, assign11810_e6437_d_n0, assign11810_e6437_d_n2, assign11810_e6437_d_n4, assign11810_e6437_d_n5, assign11810_e6437_d_n6, assign11810_e6437_d_n7, assign11810_e6437_d_n8, assign11810_e6437_d_n9, assign11810_e6437_d_n10, assign11810_e6437_d_n11, assign11810_e6437_d_n14,) = {
    if (var_guard273 != 0.0) {
        let assign11810_e6433: f64 = (3.0 * p.p4);
        let assign11810_e6434: f64 = (var_weff / assign11810_e6433);
        let assign11810_e6435: f64 = (p.p5 + assign11810_e6434);
        (assign11810_e6435, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign11810_e6437;
        var_t1_dn0 = assign11810_e6437_d_n0;
        var_t1_dn2 = assign11810_e6437_d_n2;
        var_t1_dn4 = assign11810_e6437_d_n4;
        var_t1_dn5 = assign11810_e6437_d_n5;
        var_t1_dn6 = assign11810_e6437_d_n6;
        var_t1_dn7 = assign11810_e6437_d_n7;
        var_t1_dn8 = assign11810_e6437_d_n8;
        var_t1_dn9 = assign11810_e6437_d_n9;
        var_t1_dn10 = assign11810_e6437_d_n10;
        var_t1_dn11 = assign11810_e6437_d_n11;
        var_t1_dn14 = assign11810_e6437_d_n14;

        let (assign11820_e6443, assign11820_e6443_d_n0, assign11820_e6443_d_n2, assign11820_e6443_d_n4, assign11820_e6443_d_n5, assign11820_e6443_d_n6, assign11820_e6443_d_n7, assign11820_e6443_d_n8, assign11820_e6443_d_n9, assign11820_e6443_d_n10, assign11820_e6443_d_n11, assign11820_e6443_d_n14,) = {
    if (var_guard273 != 0.0) {
        let assign11820_e6441: f64 = (var_lgate - p.p6);
        (assign11820_e6441, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign11820_e6443;
        var_t2_dn0 = assign11820_e6443_d_n0;
        var_t2_dn2 = assign11820_e6443_d_n2;
        var_t2_dn4 = assign11820_e6443_d_n4;
        var_t2_dn5 = assign11820_e6443_d_n5;
        var_t2_dn6 = assign11820_e6443_d_n6;
        var_t2_dn7 = assign11820_e6443_d_n7;
        var_t2_dn8 = assign11820_e6443_d_n8;
        var_t2_dn9 = assign11820_e6443_d_n9;
        var_t2_dn10 = assign11820_e6443_d_n10;
        var_t2_dn11 = assign11820_e6443_d_n11;
        var_t2_dn14 = assign11820_e6443_d_n14;

        let assign11880_e6485: f64 = if p.p130 > 0.0 { 1.0 } else { 0.0 };
        var_guard275 = assign11880_e6485;

        let (assign11890_e6491,) = {
    if (var_guard275 != 0.0) {
        let assign11890_e6489: f64 = (p.p130 * p.p2);
        (assign11890_e6489,)
    } else {
        (var_rd0,)
    }
};
        var_rd0 = assign11890_e6491;

        let (assign11900_e6497,) = {
    if (var_guard275 != 0.0) {
        let assign11900_e6495: f64 = (p.p130 * p.p3);
        (assign11900_e6495,)
    } else {
        (var_rs0,)
    }
};
        var_rs0 = assign11900_e6497;

        let (assign11910_e6502,) = {
    if (var_guard275 == 0.0) {
        (0.0,)
    } else {
        (var_rd0,)
    }
};
        var_rd0 = assign11910_e6502;

        let (assign11920_e6507,) = {
    if (var_guard275 == 0.0) {
        (0.0,)
    } else {
        (var_rs0,)
    }
};
        var_rs0 = assign11920_e6507;

        let assign11930_e6510: f64 = if p.p131 > 0.0 { 1.0 } else { 0.0 };
        var_guard276 = assign11930_e6510;

        let (assign11940_e6516,) = {
    if (var_guard276 != 0.0) {
        let assign11940_e6514: f64 = (p.p131 * p.p3);
        (assign11940_e6514,)
    } else {
        (var_rs0,)
    }
};
        var_rs0 = assign11940_e6516;

        let (assign11950_e6521,) = {
    if (var_guard276 == 0.0) {
        (0.0,)
    } else {
        (var_rs0,)
    }
};
        var_rs0 = assign11950_e6521;

        let assign11960_e6524: f64 = if var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        var_guard277 = assign11960_e6524;

        let assign11970_e6531: f64 = if ((var_uc_rd > 0.0) || (var_uc_rs > 0.0)) { 1.0 } else { 0.0 };
        var_guard278 = assign11970_e6531;

        let (assign11980_e6543,) = {
    if ((var_guard277 != 0.0) && (var_guard278 != 0.0)) {
        let assign11980_e6539: f64 = (var_wlg).powf(p.p310);
        let assign11980_e6540: f64 = (p.p309 / assign11980_e6539);
        let assign11980_e6541: f64 = (1.0 + assign11980_e6540);
        (assign11980_e6541,)
    } else {
        (var_rdtemp0,)
    }
};
        var_rdtemp0 = assign11980_e6543;

        let assign11990_e6546: f64 = if var_uc_rdvd != 0.0 { 1.0 } else { 0.0 };
        var_guard279 = assign11990_e6546;

        let (assign12000_e6560, assign12000_e6560_d_n0, assign12000_e6560_d_n2, assign12000_e6560_d_n4, assign12000_e6560_d_n5, assign12000_e6560_d_n6, assign12000_e6560_d_n7, assign12000_e6560_d_n8, assign12000_e6560_d_n9, assign12000_e6560_d_n10, assign12000_e6560_d_n11, assign12000_e6560_d_n14,) = {
    if (((var_guard277 != 0.0) && (var_guard278 != 0.0)) && (var_guard279 != 0.0)) {
        let assign12000_e6556: f64 = (var_wlg).powf(p.p304);
        let assign12000_e6557: f64 = (p.p303 / assign12000_e6556);
        let assign12000_e6558: f64 = (1.0 + assign12000_e6557);
        (assign12000_e6558, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn11, var_t7_dn14,)
    }
};
        var_t7 = assign12000_e6560;
        var_t7_dn0 = assign12000_e6560_d_n0;
        var_t7_dn2 = assign12000_e6560_d_n2;
        var_t7_dn4 = assign12000_e6560_d_n4;
        var_t7_dn5 = assign12000_e6560_d_n5;
        var_t7_dn6 = assign12000_e6560_d_n6;
        var_t7_dn7 = assign12000_e6560_d_n7;
        var_t7_dn8 = assign12000_e6560_d_n8;
        var_t7_dn9 = assign12000_e6560_d_n9;
        var_t7_dn10 = assign12000_e6560_d_n10;
        var_t7_dn11 = assign12000_e6560_d_n11;
        var_t7_dn14 = assign12000_e6560_d_n14;

        let (assign12010_e6573, assign12010_e6573_d_n0, assign12010_e6573_d_n2, assign12010_e6573_d_n4, assign12010_e6573_d_n5, assign12010_e6573_d_n6, assign12010_e6573_d_n7, assign12010_e6573_d_n8, assign12010_e6573_d_n9, assign12010_e6573_d_n10, assign12010_e6573_d_n11, assign12010_e6573_d_n14,) = {
    if (((var_guard277 != 0.0) && (var_guard278 != 0.0)) && (var_guard279 != 0.0)) {
        let assign12010_e6567: f64 = (-p.p301);
        let assign12010_e6570: f64 = (var_lg).powf(p.p302);
        let assign12010_e6571: f64 = (assign12010_e6567 * assign12010_e6570);
        (assign12010_e6571, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn14,)
    }
};
        var_t6 = assign12010_e6573;
        var_t6_dn0 = assign12010_e6573_d_n0;
        var_t6_dn2 = assign12010_e6573_d_n2;
        var_t6_dn4 = assign12010_e6573_d_n4;
        var_t6_dn5 = assign12010_e6573_d_n5;
        var_t6_dn6 = assign12010_e6573_d_n6;
        var_t6_dn7 = assign12010_e6573_d_n7;
        var_t6_dn8 = assign12010_e6573_d_n8;
        var_t6_dn9 = assign12010_e6573_d_n9;
        var_t6_dn10 = assign12010_e6573_d_n10;
        var_t6_dn11 = assign12010_e6573_d_n11;
        var_t6_dn14 = assign12010_e6573_d_n14;

        let assign12020_e6576: f64 = if var_t6 > 60.0 { 1.0 } else { 0.0 };
        var_guard280 = assign12020_e6576;

        let (assign12030_e6586, assign12030_e6586_d_n0, assign12030_e6586_d_n2, assign12030_e6586_d_n4, assign12030_e6586_d_n5, assign12030_e6586_d_n6, assign12030_e6586_d_n7, assign12030_e6586_d_n8, assign12030_e6586_d_n9, assign12030_e6586_d_n10, assign12030_e6586_d_n11, assign12030_e6586_d_n14,) = {
    if ((((var_guard277 != 0.0) && (var_guard278 != 0.0)) && (var_guard279 != 0.0)) && (var_guard280 != 0.0)) {
        (60.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn14,)
    }
};
        var_t6 = assign12030_e6586;
        var_t6_dn0 = assign12030_e6586_d_n0;
        var_t6_dn2 = assign12030_e6586_d_n2;
        var_t6_dn4 = assign12030_e6586_d_n4;
        var_t6_dn5 = assign12030_e6586_d_n5;
        var_t6_dn6 = assign12030_e6586_d_n6;
        var_t6_dn7 = assign12030_e6586_d_n7;
        var_t6_dn8 = assign12030_e6586_d_n8;
        var_t6_dn9 = assign12030_e6586_d_n9;
        var_t6_dn10 = assign12030_e6586_d_n10;
        var_t6_dn11 = assign12030_e6586_d_n11;
        var_t6_dn14 = assign12030_e6586_d_n14;

        let (assign12040_e6595, assign12040_e6595_d_n0, assign12040_e6595_d_n2, assign12040_e6595_d_n4, assign12040_e6595_d_n5, assign12040_e6595_d_n6, assign12040_e6595_d_n7, assign12040_e6595_d_n8, assign12040_e6595_d_n9, assign12040_e6595_d_n10, assign12040_e6595_d_n11, assign12040_e6595_d_n14,) = {
    if (((var_guard277 != 0.0) && (var_guard278 != 0.0)) && (var_guard279 != 0.0)) {
        let assign12040_e6593: f64 = (var_t6).exp();
        (assign12040_e6593, (assign12040_e6593 * var_t6_dn0), (assign12040_e6593 * var_t6_dn2), (assign12040_e6593 * var_t6_dn4), (assign12040_e6593 * var_t6_dn5), (assign12040_e6593 * var_t6_dn6), (assign12040_e6593 * var_t6_dn7), (assign12040_e6593 * var_t6_dn8), (assign12040_e6593 * var_t6_dn9), (assign12040_e6593 * var_t6_dn10), (assign12040_e6593 * var_t6_dn11), (assign12040_e6593 * var_t6_dn14),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn14,)
    }
};
        var_t6 = assign12040_e6595;
        var_t6_dn0 = assign12040_e6595_d_n0;
        var_t6_dn2 = assign12040_e6595_d_n2;
        var_t6_dn4 = assign12040_e6595_d_n4;
        var_t6_dn5 = assign12040_e6595_d_n5;
        var_t6_dn6 = assign12040_e6595_d_n6;
        var_t6_dn7 = assign12040_e6595_d_n7;
        var_t6_dn8 = assign12040_e6595_d_n8;
        var_t6_dn9 = assign12040_e6595_d_n9;
        var_t6_dn10 = assign12040_e6595_d_n10;
        var_t6_dn11 = assign12040_e6595_d_n11;
        var_t6_dn14 = assign12040_e6595_d_n14;

        let (assign12050_e6605, assign12050_e6605_d_n0, assign12050_e6605_d_n2, assign12050_e6605_d_n4, assign12050_e6605_d_n5, assign12050_e6605_d_n6, assign12050_e6605_d_n7, assign12050_e6605_d_n8, assign12050_e6605_d_n9, assign12050_e6605_d_n10, assign12050_e6605_d_n11, assign12050_e6605_d_n14,) = {
    if (((var_guard277 != 0.0) && (var_guard278 != 0.0)) && (var_guard279 != 0.0)) {
        let assign12050_e6603: f64 = (var_t6 * var_t7);
        (assign12050_e6603, ((var_t6_dn0 * var_t7) + (var_t6 * var_t7_dn0)), ((var_t6_dn2 * var_t7) + (var_t6 * var_t7_dn2)), ((var_t6_dn4 * var_t7) + (var_t6 * var_t7_dn4)), ((var_t6_dn5 * var_t7) + (var_t6 * var_t7_dn5)), ((var_t6_dn6 * var_t7) + (var_t6 * var_t7_dn6)), ((var_t6_dn7 * var_t7) + (var_t6 * var_t7_dn7)), ((var_t6_dn8 * var_t7) + (var_t6 * var_t7_dn8)), ((var_t6_dn9 * var_t7) + (var_t6 * var_t7_dn9)), ((var_t6_dn10 * var_t7) + (var_t6 * var_t7_dn10)), ((var_t6_dn11 * var_t7) + (var_t6 * var_t7_dn11)), ((var_t6_dn14 * var_t7) + (var_t6 * var_t7_dn14)),)
    } else {
        (var_rdvdtemp0, var_rdvdtemp0_dn0, var_rdvdtemp0_dn2, var_rdvdtemp0_dn4, var_rdvdtemp0_dn5, var_rdvdtemp0_dn6, var_rdvdtemp0_dn7, var_rdvdtemp0_dn8, var_rdvdtemp0_dn9, var_rdvdtemp0_dn10, var_rdvdtemp0_dn11, var_rdvdtemp0_dn14,)
    }
};
        var_rdvdtemp0 = assign12050_e6605;
        var_rdvdtemp0_dn0 = assign12050_e6605_d_n0;
        var_rdvdtemp0_dn2 = assign12050_e6605_d_n2;
        var_rdvdtemp0_dn4 = assign12050_e6605_d_n4;
        var_rdvdtemp0_dn5 = assign12050_e6605_d_n5;
        var_rdvdtemp0_dn6 = assign12050_e6605_d_n6;
        var_rdvdtemp0_dn7 = assign12050_e6605_d_n7;
        var_rdvdtemp0_dn8 = assign12050_e6605_d_n8;
        var_rdvdtemp0_dn9 = assign12050_e6605_d_n9;
        var_rdvdtemp0_dn10 = assign12050_e6605_d_n10;
        var_rdvdtemp0_dn11 = assign12050_e6605_d_n11;
        var_rdvdtemp0_dn14 = assign12050_e6605_d_n14;

        let (assign12060_e6614, assign12060_e6614_d_n0, assign12060_e6614_d_n2, assign12060_e6614_d_n4, assign12060_e6614_d_n5, assign12060_e6614_d_n6, assign12060_e6614_d_n7, assign12060_e6614_d_n8, assign12060_e6614_d_n9, assign12060_e6614_d_n10, assign12060_e6614_d_n11, assign12060_e6614_d_n14,) = {
    if (((var_guard277 != 0.0) && (var_guard278 != 0.0)) && (var_guard279 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdvdtemp0, var_rdvdtemp0_dn0, var_rdvdtemp0_dn2, var_rdvdtemp0_dn4, var_rdvdtemp0_dn5, var_rdvdtemp0_dn6, var_rdvdtemp0_dn7, var_rdvdtemp0_dn8, var_rdvdtemp0_dn9, var_rdvdtemp0_dn10, var_rdvdtemp0_dn11, var_rdvdtemp0_dn14,)
    }
};
        var_rdvdtemp0 = assign12060_e6614;
        var_rdvdtemp0_dn0 = assign12060_e6614_d_n0;
        var_rdvdtemp0_dn2 = assign12060_e6614_d_n2;
        var_rdvdtemp0_dn4 = assign12060_e6614_d_n4;
        var_rdvdtemp0_dn5 = assign12060_e6614_d_n5;
        var_rdvdtemp0_dn6 = assign12060_e6614_d_n6;
        var_rdvdtemp0_dn7 = assign12060_e6614_d_n7;
        var_rdvdtemp0_dn8 = assign12060_e6614_d_n8;
        var_rdvdtemp0_dn9 = assign12060_e6614_d_n9;
        var_rdvdtemp0_dn10 = assign12060_e6614_d_n10;
        var_rdvdtemp0_dn11 = assign12060_e6614_d_n11;
        var_rdvdtemp0_dn14 = assign12060_e6614_d_n14;

        let (assign12070_e6621,) = {
    if ((var_guard277 != 0.0) && (var_guard278 == 0.0)) {
        (0.0,)
    } else {
        (var_rdtemp0,)
    }
};
        var_rdtemp0 = assign12070_e6621;

        let (assign12080_e6628, assign12080_e6628_d_n0, assign12080_e6628_d_n2, assign12080_e6628_d_n4, assign12080_e6628_d_n5, assign12080_e6628_d_n6, assign12080_e6628_d_n7, assign12080_e6628_d_n8, assign12080_e6628_d_n9, assign12080_e6628_d_n10, assign12080_e6628_d_n11, assign12080_e6628_d_n14,) = {
    if ((var_guard277 != 0.0) && (var_guard278 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdvdtemp0, var_rdvdtemp0_dn0, var_rdvdtemp0_dn2, var_rdvdtemp0_dn4, var_rdvdtemp0_dn5, var_rdvdtemp0_dn6, var_rdvdtemp0_dn7, var_rdvdtemp0_dn8, var_rdvdtemp0_dn9, var_rdvdtemp0_dn10, var_rdvdtemp0_dn11, var_rdvdtemp0_dn14,)
    }
};
        var_rdvdtemp0 = assign12080_e6628;
        var_rdvdtemp0_dn0 = assign12080_e6628_d_n0;
        var_rdvdtemp0_dn2 = assign12080_e6628_d_n2;
        var_rdvdtemp0_dn4 = assign12080_e6628_d_n4;
        var_rdvdtemp0_dn5 = assign12080_e6628_d_n5;
        var_rdvdtemp0_dn6 = assign12080_e6628_d_n6;
        var_rdvdtemp0_dn7 = assign12080_e6628_d_n7;
        var_rdvdtemp0_dn8 = assign12080_e6628_d_n8;
        var_rdvdtemp0_dn9 = assign12080_e6628_d_n9;
        var_rdvdtemp0_dn10 = assign12080_e6628_d_n10;
        var_rdvdtemp0_dn11 = assign12080_e6628_d_n11;
        var_rdvdtemp0_dn14 = assign12080_e6628_d_n14;

        let assign12090_e6631: f64 = if var_uc_rd23 != 0.0 { 1.0 } else { 0.0 };
        var_guard281 = assign12090_e6631;

        let (assign12100_e6643, assign12100_e6643_d_n0, assign12100_e6643_d_n2, assign12100_e6643_d_n4, assign12100_e6643_d_n5, assign12100_e6643_d_n6, assign12100_e6643_d_n7, assign12100_e6643_d_n8, assign12100_e6643_d_n9, assign12100_e6643_d_n10, assign12100_e6643_d_n11, assign12100_e6643_d_n14,) = {
    if ((var_guard277 != 0.0) && (var_guard281 != 0.0)) {
        let assign12100_e6639: f64 = (var_wlg).powf(p.p308);
        let assign12100_e6640: f64 = (p.p307 / assign12100_e6639);
        let assign12100_e6641: f64 = (1.0 + assign12100_e6640);
        (assign12100_e6641, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign12100_e6643;
        var_t2_dn0 = assign12100_e6643_d_n0;
        var_t2_dn2 = assign12100_e6643_d_n2;
        var_t2_dn4 = assign12100_e6643_d_n4;
        var_t2_dn5 = assign12100_e6643_d_n5;
        var_t2_dn6 = assign12100_e6643_d_n6;
        var_t2_dn7 = assign12100_e6643_d_n7;
        var_t2_dn8 = assign12100_e6643_d_n8;
        var_t2_dn9 = assign12100_e6643_d_n9;
        var_t2_dn10 = assign12100_e6643_d_n10;
        var_t2_dn11 = assign12100_e6643_d_n11;
        var_t2_dn14 = assign12100_e6643_d_n14;

        let (assign12110_e6654, assign12110_e6654_d_n0, assign12110_e6654_d_n2, assign12110_e6654_d_n4, assign12110_e6654_d_n5, assign12110_e6654_d_n6, assign12110_e6654_d_n7, assign12110_e6654_d_n8, assign12110_e6654_d_n9, assign12110_e6654_d_n10, assign12110_e6654_d_n11, assign12110_e6654_d_n14,) = {
    if ((var_guard277 != 0.0) && (var_guard281 != 0.0)) {
        let assign12110_e6648: f64 = (-p.p305);
        let assign12110_e6651: f64 = (var_lg).powf(p.p306);
        let assign12110_e6652: f64 = (assign12110_e6648 * assign12110_e6651);
        (assign12110_e6652, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign12110_e6654;
        var_t1_dn0 = assign12110_e6654_d_n0;
        var_t1_dn2 = assign12110_e6654_d_n2;
        var_t1_dn4 = assign12110_e6654_d_n4;
        var_t1_dn5 = assign12110_e6654_d_n5;
        var_t1_dn6 = assign12110_e6654_d_n6;
        var_t1_dn7 = assign12110_e6654_d_n7;
        var_t1_dn8 = assign12110_e6654_d_n8;
        var_t1_dn9 = assign12110_e6654_d_n9;
        var_t1_dn10 = assign12110_e6654_d_n10;
        var_t1_dn11 = assign12110_e6654_d_n11;
        var_t1_dn14 = assign12110_e6654_d_n14;

        let assign12120_e6657: f64 = if var_t1 > 60.0 { 1.0 } else { 0.0 };
        var_guard282 = assign12120_e6657;

        let (assign12130_e6665, assign12130_e6665_d_n0, assign12130_e6665_d_n2, assign12130_e6665_d_n4, assign12130_e6665_d_n5, assign12130_e6665_d_n6, assign12130_e6665_d_n7, assign12130_e6665_d_n8, assign12130_e6665_d_n9, assign12130_e6665_d_n10, assign12130_e6665_d_n11, assign12130_e6665_d_n14,) = {
    if (((var_guard277 != 0.0) && (var_guard281 != 0.0)) && (var_guard282 != 0.0)) {
        (60.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign12130_e6665;
        var_t1_dn0 = assign12130_e6665_d_n0;
        var_t1_dn2 = assign12130_e6665_d_n2;
        var_t1_dn4 = assign12130_e6665_d_n4;
        var_t1_dn5 = assign12130_e6665_d_n5;
        var_t1_dn6 = assign12130_e6665_d_n6;
        var_t1_dn7 = assign12130_e6665_d_n7;
        var_t1_dn8 = assign12130_e6665_d_n8;
        var_t1_dn9 = assign12130_e6665_d_n9;
        var_t1_dn10 = assign12130_e6665_d_n10;
        var_t1_dn11 = assign12130_e6665_d_n11;
        var_t1_dn14 = assign12130_e6665_d_n14;

        let (assign12140_e6672, assign12140_e6672_d_n0, assign12140_e6672_d_n2, assign12140_e6672_d_n4, assign12140_e6672_d_n5, assign12140_e6672_d_n6, assign12140_e6672_d_n7, assign12140_e6672_d_n8, assign12140_e6672_d_n9, assign12140_e6672_d_n10, assign12140_e6672_d_n11, assign12140_e6672_d_n14,) = {
    if ((var_guard277 != 0.0) && (var_guard281 != 0.0)) {
        let assign12140_e6670: f64 = (var_t1).exp();
        (assign12140_e6670, (assign12140_e6670 * var_t1_dn0), (assign12140_e6670 * var_t1_dn2), (assign12140_e6670 * var_t1_dn4), (assign12140_e6670 * var_t1_dn5), (assign12140_e6670 * var_t1_dn6), (assign12140_e6670 * var_t1_dn7), (assign12140_e6670 * var_t1_dn8), (assign12140_e6670 * var_t1_dn9), (assign12140_e6670 * var_t1_dn10), (assign12140_e6670 * var_t1_dn11), (assign12140_e6670 * var_t1_dn14),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign12140_e6672;
        var_t1_dn0 = assign12140_e6672_d_n0;
        var_t1_dn2 = assign12140_e6672_d_n2;
        var_t1_dn4 = assign12140_e6672_d_n4;
        var_t1_dn5 = assign12140_e6672_d_n5;
        var_t1_dn6 = assign12140_e6672_d_n6;
        var_t1_dn7 = assign12140_e6672_d_n7;
        var_t1_dn8 = assign12140_e6672_d_n8;
        var_t1_dn9 = assign12140_e6672_d_n9;
        var_t1_dn10 = assign12140_e6672_d_n10;
        var_t1_dn11 = assign12140_e6672_d_n11;
        var_t1_dn14 = assign12140_e6672_d_n14;

        let (assign12150_e6682, assign12150_e6682_d_n0, assign12150_e6682_d_n2, assign12150_e6682_d_n4, assign12150_e6682_d_n5, assign12150_e6682_d_n6, assign12150_e6682_d_n7, assign12150_e6682_d_n8, assign12150_e6682_d_n9, assign12150_e6682_d_n10, assign12150_e6682_d_n11, assign12150_e6682_d_n14,) = {
    if ((var_guard277 != 0.0) && (var_guard281 != 0.0)) {
        let assign12150_e6678: f64 = (var_uc_rd23 * var_t2);
        let assign12150_e6680: f64 = (assign12150_e6678 * var_t1);
        (assign12150_e6680, (((var_uc_rd23 * var_t2_dn0) * var_t1) + (assign12150_e6678 * var_t1_dn0)), (((var_uc_rd23 * var_t2_dn2) * var_t1) + (assign12150_e6678 * var_t1_dn2)), (((var_uc_rd23 * var_t2_dn4) * var_t1) + (assign12150_e6678 * var_t1_dn4)), (((var_uc_rd23 * var_t2_dn5) * var_t1) + (assign12150_e6678 * var_t1_dn5)), (((var_uc_rd23 * var_t2_dn6) * var_t1) + (assign12150_e6678 * var_t1_dn6)), (((var_uc_rd23 * var_t2_dn7) * var_t1) + (assign12150_e6678 * var_t1_dn7)), (((var_uc_rd23 * var_t2_dn8) * var_t1) + (assign12150_e6678 * var_t1_dn8)), (((var_uc_rd23 * var_t2_dn9) * var_t1) + (assign12150_e6678 * var_t1_dn9)), (((var_uc_rd23 * var_t2_dn10) * var_t1) + (assign12150_e6678 * var_t1_dn10)), (((var_uc_rd23 * var_t2_dn11) * var_t1) + (assign12150_e6678 * var_t1_dn11)), (((var_uc_rd23 * var_t2_dn14) * var_t1) + (assign12150_e6678 * var_t1_dn14)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign12150_e6682;
        var_t3_dn0 = assign12150_e6682_d_n0;
        var_t3_dn2 = assign12150_e6682_d_n2;
        var_t3_dn4 = assign12150_e6682_d_n4;
        var_t3_dn5 = assign12150_e6682_d_n5;
        var_t3_dn6 = assign12150_e6682_d_n6;
        var_t3_dn7 = assign12150_e6682_d_n7;
        var_t3_dn8 = assign12150_e6682_d_n8;
        var_t3_dn9 = assign12150_e6682_d_n9;
        var_t3_dn10 = assign12150_e6682_d_n10;
        var_t3_dn11 = assign12150_e6682_d_n11;
        var_t3_dn14 = assign12150_e6682_d_n14;

        let (assign12160_e6705, assign12160_e6705_d_n0, assign12160_e6705_d_n2, assign12160_e6705_d_n4, assign12160_e6705_d_n5, assign12160_e6705_d_n6, assign12160_e6705_d_n7, assign12160_e6705_d_n8, assign12160_e6705_d_n9, assign12160_e6705_d_n10, assign12160_e6705_d_n11, assign12160_e6705_d_n14,) = {
    if ((var_guard277 != 0.0) && (var_guard281 != 0.0)) {
        let assign12160_e6690: f64 = (var_t3 * var_t3);
        let assign12160_e6693: f64 = (4.0 * 1e-6);
        let assign12160_e6695: f64 = (assign12160_e6693 / 100.0);
        let assign12160_e6697: f64 = (assign12160_e6695 * 1e-6);
        let assign12160_e6699: f64 = (assign12160_e6697 / 100.0);
        let assign12160_e6700: f64 = (assign12160_e6690 + assign12160_e6699);
        let assign12160_e6701: f64 = (assign12160_e6700).sqrt();
        let assign12160_e6702: f64 = (var_t3 + assign12160_e6701);
        let assign12160_e6703: f64 = (0.5 * assign12160_e6702);
        (assign12160_e6703, (0.5 * (var_t3_dn0 + (((var_t3_dn0 * var_t3) + (var_t3 * var_t3_dn0)) / (2.0 * assign12160_e6701)))), (0.5 * (var_t3_dn2 + (((var_t3_dn2 * var_t3) + (var_t3 * var_t3_dn2)) / (2.0 * assign12160_e6701)))), (0.5 * (var_t3_dn4 + (((var_t3_dn4 * var_t3) + (var_t3 * var_t3_dn4)) / (2.0 * assign12160_e6701)))), (0.5 * (var_t3_dn5 + (((var_t3_dn5 * var_t3) + (var_t3 * var_t3_dn5)) / (2.0 * assign12160_e6701)))), (0.5 * (var_t3_dn6 + (((var_t3_dn6 * var_t3) + (var_t3 * var_t3_dn6)) / (2.0 * assign12160_e6701)))), (0.5 * (var_t3_dn7 + (((var_t3_dn7 * var_t3) + (var_t3 * var_t3_dn7)) / (2.0 * assign12160_e6701)))), (0.5 * (var_t3_dn8 + (((var_t3_dn8 * var_t3) + (var_t3 * var_t3_dn8)) / (2.0 * assign12160_e6701)))), (0.5 * (var_t3_dn9 + (((var_t3_dn9 * var_t3) + (var_t3 * var_t3_dn9)) / (2.0 * assign12160_e6701)))), (0.5 * (var_t3_dn10 + (((var_t3_dn10 * var_t3) + (var_t3 * var_t3_dn10)) / (2.0 * assign12160_e6701)))), (0.5 * (var_t3_dn11 + (((var_t3_dn11 * var_t3) + (var_t3 * var_t3_dn11)) / (2.0 * assign12160_e6701)))), (0.5 * (var_t3_dn14 + (((var_t3_dn14 * var_t3) + (var_t3 * var_t3_dn14)) / (2.0 * assign12160_e6701)))),)
    } else {
        (var_rd23e, var_rd23e_dn0, var_rd23e_dn2, var_rd23e_dn4, var_rd23e_dn5, var_rd23e_dn6, var_rd23e_dn7, var_rd23e_dn8, var_rd23e_dn9, var_rd23e_dn10, var_rd23e_dn11, var_rd23e_dn14,)
    }
};
        var_rd23e = assign12160_e6705;
        var_rd23e_dn0 = assign12160_e6705_d_n0;
        var_rd23e_dn2 = assign12160_e6705_d_n2;
        var_rd23e_dn4 = assign12160_e6705_d_n4;
        var_rd23e_dn5 = assign12160_e6705_d_n5;
        var_rd23e_dn6 = assign12160_e6705_d_n6;
        var_rd23e_dn7 = assign12160_e6705_d_n7;
        var_rd23e_dn8 = assign12160_e6705_d_n8;
        var_rd23e_dn9 = assign12160_e6705_d_n9;
        var_rd23e_dn10 = assign12160_e6705_d_n10;
        var_rd23e_dn11 = assign12160_e6705_d_n11;
        var_rd23e_dn14 = assign12160_e6705_d_n14;

        let (assign12170_e6712, assign12170_e6712_d_n0, assign12170_e6712_d_n2, assign12170_e6712_d_n4, assign12170_e6712_d_n5, assign12170_e6712_d_n6, assign12170_e6712_d_n7, assign12170_e6712_d_n8, assign12170_e6712_d_n9, assign12170_e6712_d_n10, assign12170_e6712_d_n11, assign12170_e6712_d_n14,) = {
    if ((var_guard277 != 0.0) && (var_guard281 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rd23e, var_rd23e_dn0, var_rd23e_dn2, var_rd23e_dn4, var_rd23e_dn5, var_rd23e_dn6, var_rd23e_dn7, var_rd23e_dn8, var_rd23e_dn9, var_rd23e_dn10, var_rd23e_dn11, var_rd23e_dn14,)
    }
};
        var_rd23e = assign12170_e6712;
        var_rd23e_dn0 = assign12170_e6712_d_n0;
        var_rd23e_dn2 = assign12170_e6712_d_n2;
        var_rd23e_dn4 = assign12170_e6712_d_n4;
        var_rd23e_dn5 = assign12170_e6712_d_n5;
        var_rd23e_dn6 = assign12170_e6712_d_n6;
        var_rd23e_dn7 = assign12170_e6712_d_n7;
        var_rd23e_dn8 = assign12170_e6712_d_n8;
        var_rd23e_dn9 = assign12170_e6712_d_n9;
        var_rd23e_dn10 = assign12170_e6712_d_n10;
        var_rd23e_dn11 = assign12170_e6712_d_n11;
        var_rd23e_dn14 = assign12170_e6712_d_n14;

        let (assign12180_e6716,) = {
    if (var_guard277 != 0.0) {
        (0.0,)
    } else {
        (var_xmax,)
    }
};
        var_xmax = assign12180_e6716;

        let (assign12190_e6720,) = {
    if (var_guard277 != 0.0) {
        (0.0,)
    } else {
        (var_xmax_s,)
    }
};
        var_xmax_s = assign12190_e6720;

        *var_guard273_slot = var_guard273;
        *var_guard275_slot = var_guard275;
        *var_guard276_slot = var_guard276;
        *var_guard277_slot = var_guard277;
        *var_guard278_slot = var_guard278;
        *var_guard279_slot = var_guard279;
        *var_guard280_slot = var_guard280;
        *var_guard281_slot = var_guard281;
        *var_guard282_slot = var_guard282;
        *var_pb20_slot = var_pb20;
        *var_pb20_dn0_slot = var_pb20_dn0;
        *var_pb20_dn10_slot = var_pb20_dn10;
        *var_pb20_dn11_slot = var_pb20_dn11;
        *var_pb20_dn14_slot = var_pb20_dn14;
        *var_pb20_dn2_slot = var_pb20_dn2;
        *var_pb20_dn4_slot = var_pb20_dn4;
        *var_pb20_dn5_slot = var_pb20_dn5;
        *var_pb20_dn6_slot = var_pb20_dn6;
        *var_pb20_dn7_slot = var_pb20_dn7;
        *var_pb20_dn8_slot = var_pb20_dn8;
        *var_pb20_dn9_slot = var_pb20_dn9;
        *var_pb2c_slot = var_pb2c;
        *var_pb2c_dn0_slot = var_pb2c_dn0;
        *var_pb2c_dn10_slot = var_pb2c_dn10;
        *var_pb2c_dn11_slot = var_pb2c_dn11;
        *var_pb2c_dn14_slot = var_pb2c_dn14;
        *var_pb2c_dn2_slot = var_pb2c_dn2;
        *var_pb2c_dn4_slot = var_pb2c_dn4;
        *var_pb2c_dn5_slot = var_pb2c_dn5;
        *var_pb2c_dn6_slot = var_pb2c_dn6;
        *var_pb2c_dn7_slot = var_pb2c_dn7;
        *var_pb2c_dn8_slot = var_pb2c_dn8;
        *var_pb2c_dn9_slot = var_pb2c_dn9;
        *var_rd0_slot = var_rd0;
        *var_rd23e_slot = var_rd23e;
        *var_rd23e_dn0_slot = var_rd23e_dn0;
        *var_rd23e_dn10_slot = var_rd23e_dn10;
        *var_rd23e_dn11_slot = var_rd23e_dn11;
        *var_rd23e_dn14_slot = var_rd23e_dn14;
        *var_rd23e_dn2_slot = var_rd23e_dn2;
        *var_rd23e_dn4_slot = var_rd23e_dn4;
        *var_rd23e_dn5_slot = var_rd23e_dn5;
        *var_rd23e_dn6_slot = var_rd23e_dn6;
        *var_rd23e_dn7_slot = var_rd23e_dn7;
        *var_rd23e_dn8_slot = var_rd23e_dn8;
        *var_rd23e_dn9_slot = var_rd23e_dn9;
        *var_rdtemp0_slot = var_rdtemp0;
        *var_rdvdtemp0_slot = var_rdvdtemp0;
        *var_rdvdtemp0_dn0_slot = var_rdvdtemp0_dn0;
        *var_rdvdtemp0_dn10_slot = var_rdvdtemp0_dn10;
        *var_rdvdtemp0_dn11_slot = var_rdvdtemp0_dn11;
        *var_rdvdtemp0_dn14_slot = var_rdvdtemp0_dn14;
        *var_rdvdtemp0_dn2_slot = var_rdvdtemp0_dn2;
        *var_rdvdtemp0_dn4_slot = var_rdvdtemp0_dn4;
        *var_rdvdtemp0_dn5_slot = var_rdvdtemp0_dn5;
        *var_rdvdtemp0_dn6_slot = var_rdvdtemp0_dn6;
        *var_rdvdtemp0_dn7_slot = var_rdvdtemp0_dn7;
        *var_rdvdtemp0_dn8_slot = var_rdvdtemp0_dn8;
        *var_rdvdtemp0_dn9_slot = var_rdvdtemp0_dn9;
        *var_rs0_slot = var_rs0;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn14_slot = var_t6_dn14;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
        *var_t7_slot = var_t7;
        *var_t7_dn0_slot = var_t7_dn0;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn14_slot = var_t7_dn14;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn7_slot = var_t7_dn7;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_dn9_slot = var_t7_dn9;
        *var_xmax_slot = var_xmax;
        *var_xmax_s_slot = var_xmax_s;
    }

    pub(super) fn stamp_transient_block_20(
        p: &Parameters,
        var_ef_nsubc: f64,
        var_ef_nsubc_dn0: f64,
        var_ef_nsubc_dn10: f64,
        var_ef_nsubc_dn11: f64,
        var_ef_nsubc_dn14: f64,
        var_ef_nsubc_dn2: f64,
        var_ef_nsubc_dn4: f64,
        var_ef_nsubc_dn5: f64,
        var_ef_nsubc_dn6: f64,
        var_ef_nsubc_dn7: f64,
        var_ef_nsubc_dn8: f64,
        var_ef_nsubc_dn9: f64,
        var_guard277: f64,
        var_lg: f64,
        var_lgate: f64,
        var_mks_slg: f64,
        var_mks_slgl: f64,
        var_mks_sub1l: f64,
        var_mks_sub2l: f64,
        var_mks_svbsl: f64,
        var_mks_svgsl: f64,
        var_mks_svgsw: f64,
        var_uc_nover: f64,
        var_uc_sub1: f64,
        var_uc_sub2: f64,
        var_uc_svbs: f64,
        var_uc_svgs: f64,
        var_uc_xldld: f64,
        var_weff: f64,
        var_wg: f64,
        var_ddlte_slot: &mut f64,
        var_ddlte_dn0_slot: &mut f64,
        var_ddlte_dn10_slot: &mut f64,
        var_ddlte_dn11_slot: &mut f64,
        var_ddlte_dn14_slot: &mut f64,
        var_ddlte_dn2_slot: &mut f64,
        var_ddlte_dn4_slot: &mut f64,
        var_ddlte_dn5_slot: &mut f64,
        var_ddlte_dn6_slot: &mut f64,
        var_ddlte_dn7_slot: &mut f64,
        var_ddlte_dn8_slot: &mut f64,
        var_ddlte_dn9_slot: &mut f64,
        var_guard283_slot: &mut f64,
        var_guard288_slot: &mut f64,
        var_guard289_slot: &mut f64,
        var_guard290_slot: &mut f64,
        var_guard292_slot: &mut f64,
        var_kdep_slot: &mut f64,
        var_kjunc_slot: &mut f64,
        var_kjunc_dn0_slot: &mut f64,
        var_kjunc_dn10_slot: &mut f64,
        var_kjunc_dn11_slot: &mut f64,
        var_kjunc_dn14_slot: &mut f64,
        var_kjunc_dn2_slot: &mut f64,
        var_kjunc_dn4_slot: &mut f64,
        var_kjunc_dn5_slot: &mut f64,
        var_kjunc_dn6_slot: &mut f64,
        var_kjunc_dn7_slot: &mut f64,
        var_kjunc_dn8_slot: &mut f64,
        var_kjunc_dn9_slot: &mut f64,
        var_rd23e_slot: &mut f64,
        var_rd23e_dn0_slot: &mut f64,
        var_rd23e_dn10_slot: &mut f64,
        var_rd23e_dn11_slot: &mut f64,
        var_rd23e_dn14_slot: &mut f64,
        var_rd23e_dn2_slot: &mut f64,
        var_rd23e_dn4_slot: &mut f64,
        var_rd23e_dn5_slot: &mut f64,
        var_rd23e_dn6_slot: &mut f64,
        var_rd23e_dn7_slot: &mut f64,
        var_rd23e_dn8_slot: &mut f64,
        var_rd23e_dn9_slot: &mut f64,
        var_rdrmuele_slot: &mut f64,
        var_rdrmuevbs_slot: &mut f64,
        var_rdrmuevbs_dn0_slot: &mut f64,
        var_rdrmuevbs_dn10_slot: &mut f64,
        var_rdrmuevbs_dn11_slot: &mut f64,
        var_rdrmuevbs_dn14_slot: &mut f64,
        var_rdrmuevbs_dn2_slot: &mut f64,
        var_rdrmuevbs_dn4_slot: &mut f64,
        var_rdrmuevbs_dn5_slot: &mut f64,
        var_rdrmuevbs_dn6_slot: &mut f64,
        var_rdrmuevbs_dn7_slot: &mut f64,
        var_rdrmuevbs_dn8_slot: &mut f64,
        var_rdrmuevbs_dn9_slot: &mut f64,
        var_rdrvmaxle_slot: &mut f64,
        var_rdrvmaxwe_slot: &mut f64,
        var_rdtemp0_slot: &mut f64,
        var_rdvdtemp0_slot: &mut f64,
        var_rdvdtemp0_dn0_slot: &mut f64,
        var_rdvdtemp0_dn10_slot: &mut f64,
        var_rdvdtemp0_dn11_slot: &mut f64,
        var_rdvdtemp0_dn14_slot: &mut f64,
        var_rdvdtemp0_dn2_slot: &mut f64,
        var_rdvdtemp0_dn4_slot: &mut f64,
        var_rdvdtemp0_dn5_slot: &mut f64,
        var_rdvdtemp0_dn6_slot: &mut f64,
        var_rdvdtemp0_dn7_slot: &mut f64,
        var_rdvdtemp0_dn8_slot: &mut f64,
        var_rdvdtemp0_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_vg2const_slot: &mut f64,
        var_vg2const_dn0_slot: &mut f64,
        var_vg2const_dn10_slot: &mut f64,
        var_vg2const_dn11_slot: &mut f64,
        var_vg2const_dn14_slot: &mut f64,
        var_vg2const_dn2_slot: &mut f64,
        var_vg2const_dn4_slot: &mut f64,
        var_vg2const_dn5_slot: &mut f64,
        var_vg2const_dn6_slot: &mut f64,
        var_vg2const_dn7_slot: &mut f64,
        var_vg2const_dn8_slot: &mut f64,
        var_vg2const_dn9_slot: &mut f64,
        var_xgate_slot: &mut f64,
        var_xmax_slot: &mut f64,
        var_xmax_s_slot: &mut f64,
        var_xsub1_slot: &mut f64,
        var_xsub2_slot: &mut f64,
        var_xvbs_slot: &mut f64,
    ) {
        let mut var_ddlte: f64 = *var_ddlte_slot;
        let mut var_ddlte_dn0: f64 = *var_ddlte_dn0_slot;
        let mut var_ddlte_dn10: f64 = *var_ddlte_dn10_slot;
        let mut var_ddlte_dn11: f64 = *var_ddlte_dn11_slot;
        let mut var_ddlte_dn14: f64 = *var_ddlte_dn14_slot;
        let mut var_ddlte_dn2: f64 = *var_ddlte_dn2_slot;
        let mut var_ddlte_dn4: f64 = *var_ddlte_dn4_slot;
        let mut var_ddlte_dn5: f64 = *var_ddlte_dn5_slot;
        let mut var_ddlte_dn6: f64 = *var_ddlte_dn6_slot;
        let mut var_ddlte_dn7: f64 = *var_ddlte_dn7_slot;
        let mut var_ddlte_dn8: f64 = *var_ddlte_dn8_slot;
        let mut var_ddlte_dn9: f64 = *var_ddlte_dn9_slot;
        let mut var_guard283: f64 = *var_guard283_slot;
        let mut var_guard288: f64 = *var_guard288_slot;
        let mut var_guard289: f64 = *var_guard289_slot;
        let mut var_guard290: f64 = *var_guard290_slot;
        let mut var_guard292: f64 = *var_guard292_slot;
        let mut var_kdep: f64 = *var_kdep_slot;
        let mut var_kjunc: f64 = *var_kjunc_slot;
        let mut var_kjunc_dn0: f64 = *var_kjunc_dn0_slot;
        let mut var_kjunc_dn10: f64 = *var_kjunc_dn10_slot;
        let mut var_kjunc_dn11: f64 = *var_kjunc_dn11_slot;
        let mut var_kjunc_dn14: f64 = *var_kjunc_dn14_slot;
        let mut var_kjunc_dn2: f64 = *var_kjunc_dn2_slot;
        let mut var_kjunc_dn4: f64 = *var_kjunc_dn4_slot;
        let mut var_kjunc_dn5: f64 = *var_kjunc_dn5_slot;
        let mut var_kjunc_dn6: f64 = *var_kjunc_dn6_slot;
        let mut var_kjunc_dn7: f64 = *var_kjunc_dn7_slot;
        let mut var_kjunc_dn8: f64 = *var_kjunc_dn8_slot;
        let mut var_kjunc_dn9: f64 = *var_kjunc_dn9_slot;
        let mut var_rd23e: f64 = *var_rd23e_slot;
        let mut var_rd23e_dn0: f64 = *var_rd23e_dn0_slot;
        let mut var_rd23e_dn10: f64 = *var_rd23e_dn10_slot;
        let mut var_rd23e_dn11: f64 = *var_rd23e_dn11_slot;
        let mut var_rd23e_dn14: f64 = *var_rd23e_dn14_slot;
        let mut var_rd23e_dn2: f64 = *var_rd23e_dn2_slot;
        let mut var_rd23e_dn4: f64 = *var_rd23e_dn4_slot;
        let mut var_rd23e_dn5: f64 = *var_rd23e_dn5_slot;
        let mut var_rd23e_dn6: f64 = *var_rd23e_dn6_slot;
        let mut var_rd23e_dn7: f64 = *var_rd23e_dn7_slot;
        let mut var_rd23e_dn8: f64 = *var_rd23e_dn8_slot;
        let mut var_rd23e_dn9: f64 = *var_rd23e_dn9_slot;
        let mut var_rdrmuele: f64 = *var_rdrmuele_slot;
        let mut var_rdrmuevbs: f64 = *var_rdrmuevbs_slot;
        let mut var_rdrmuevbs_dn0: f64 = *var_rdrmuevbs_dn0_slot;
        let mut var_rdrmuevbs_dn10: f64 = *var_rdrmuevbs_dn10_slot;
        let mut var_rdrmuevbs_dn11: f64 = *var_rdrmuevbs_dn11_slot;
        let mut var_rdrmuevbs_dn14: f64 = *var_rdrmuevbs_dn14_slot;
        let mut var_rdrmuevbs_dn2: f64 = *var_rdrmuevbs_dn2_slot;
        let mut var_rdrmuevbs_dn4: f64 = *var_rdrmuevbs_dn4_slot;
        let mut var_rdrmuevbs_dn5: f64 = *var_rdrmuevbs_dn5_slot;
        let mut var_rdrmuevbs_dn6: f64 = *var_rdrmuevbs_dn6_slot;
        let mut var_rdrmuevbs_dn7: f64 = *var_rdrmuevbs_dn7_slot;
        let mut var_rdrmuevbs_dn8: f64 = *var_rdrmuevbs_dn8_slot;
        let mut var_rdrmuevbs_dn9: f64 = *var_rdrmuevbs_dn9_slot;
        let mut var_rdrvmaxle: f64 = *var_rdrvmaxle_slot;
        let mut var_rdrvmaxwe: f64 = *var_rdrvmaxwe_slot;
        let mut var_rdtemp0: f64 = *var_rdtemp0_slot;
        let mut var_rdvdtemp0: f64 = *var_rdvdtemp0_slot;
        let mut var_rdvdtemp0_dn0: f64 = *var_rdvdtemp0_dn0_slot;
        let mut var_rdvdtemp0_dn10: f64 = *var_rdvdtemp0_dn10_slot;
        let mut var_rdvdtemp0_dn11: f64 = *var_rdvdtemp0_dn11_slot;
        let mut var_rdvdtemp0_dn14: f64 = *var_rdvdtemp0_dn14_slot;
        let mut var_rdvdtemp0_dn2: f64 = *var_rdvdtemp0_dn2_slot;
        let mut var_rdvdtemp0_dn4: f64 = *var_rdvdtemp0_dn4_slot;
        let mut var_rdvdtemp0_dn5: f64 = *var_rdvdtemp0_dn5_slot;
        let mut var_rdvdtemp0_dn6: f64 = *var_rdvdtemp0_dn6_slot;
        let mut var_rdvdtemp0_dn7: f64 = *var_rdvdtemp0_dn7_slot;
        let mut var_rdvdtemp0_dn8: f64 = *var_rdvdtemp0_dn8_slot;
        let mut var_rdvdtemp0_dn9: f64 = *var_rdvdtemp0_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_vg2const: f64 = *var_vg2const_slot;
        let mut var_vg2const_dn0: f64 = *var_vg2const_dn0_slot;
        let mut var_vg2const_dn10: f64 = *var_vg2const_dn10_slot;
        let mut var_vg2const_dn11: f64 = *var_vg2const_dn11_slot;
        let mut var_vg2const_dn14: f64 = *var_vg2const_dn14_slot;
        let mut var_vg2const_dn2: f64 = *var_vg2const_dn2_slot;
        let mut var_vg2const_dn4: f64 = *var_vg2const_dn4_slot;
        let mut var_vg2const_dn5: f64 = *var_vg2const_dn5_slot;
        let mut var_vg2const_dn6: f64 = *var_vg2const_dn6_slot;
        let mut var_vg2const_dn7: f64 = *var_vg2const_dn7_slot;
        let mut var_vg2const_dn8: f64 = *var_vg2const_dn8_slot;
        let mut var_vg2const_dn9: f64 = *var_vg2const_dn9_slot;
        let mut var_xgate: f64 = *var_xgate_slot;
        let mut var_xmax: f64 = *var_xmax_slot;
        let mut var_xmax_s: f64 = *var_xmax_s_slot;
        let mut var_xsub1: f64 = *var_xsub1_slot;
        let mut var_xsub2: f64 = *var_xsub2_slot;
        let mut var_xvbs: f64 = *var_xvbs_slot;

        let (assign12200_e6724,) = {
    if (var_guard277 != 0.0) {
        (0.0,)
    } else {
        (var_rdrvmaxwe,)
    }
};
        var_rdrvmaxwe = assign12200_e6724;

        let (assign12210_e6728,) = {
    if (var_guard277 != 0.0) {
        (0.0,)
    } else {
        (var_rdrvmaxle,)
    }
};
        var_rdrvmaxle = assign12210_e6728;

        let (assign12220_e6732,) = {
    if (var_guard277 != 0.0) {
        (0.0,)
    } else {
        (var_rdrmuele,)
    }
};
        var_rdrmuele = assign12220_e6732;

        let (assign12230_e6736, assign12230_e6736_d_n0, assign12230_e6736_d_n2, assign12230_e6736_d_n4, assign12230_e6736_d_n5, assign12230_e6736_d_n6, assign12230_e6736_d_n7, assign12230_e6736_d_n8, assign12230_e6736_d_n9, assign12230_e6736_d_n10, assign12230_e6736_d_n11, assign12230_e6736_d_n14,) = {
    if (var_guard277 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdrmuevbs, var_rdrmuevbs_dn0, var_rdrmuevbs_dn2, var_rdrmuevbs_dn4, var_rdrmuevbs_dn5, var_rdrmuevbs_dn6, var_rdrmuevbs_dn7, var_rdrmuevbs_dn8, var_rdrmuevbs_dn9, var_rdrmuevbs_dn10, var_rdrmuevbs_dn11, var_rdrmuevbs_dn14,)
    }
};
        var_rdrmuevbs = assign12230_e6736;
        var_rdrmuevbs_dn0 = assign12230_e6736_d_n0;
        var_rdrmuevbs_dn2 = assign12230_e6736_d_n2;
        var_rdrmuevbs_dn4 = assign12230_e6736_d_n4;
        var_rdrmuevbs_dn5 = assign12230_e6736_d_n5;
        var_rdrmuevbs_dn6 = assign12230_e6736_d_n6;
        var_rdrmuevbs_dn7 = assign12230_e6736_d_n7;
        var_rdrmuevbs_dn8 = assign12230_e6736_d_n8;
        var_rdrmuevbs_dn9 = assign12230_e6736_d_n9;
        var_rdrmuevbs_dn10 = assign12230_e6736_d_n10;
        var_rdrmuevbs_dn11 = assign12230_e6736_d_n11;
        var_rdrmuevbs_dn14 = assign12230_e6736_d_n14;

        let (assign12240_e6748,) = {
    if (var_guard277 == 0.0) {
        let assign12240_e6741: f64 = (p.p419 * p.p419);
        let assign12240_e6744: f64 = (var_uc_xldld * var_uc_xldld);
        let assign12240_e6745: f64 = (assign12240_e6741 + assign12240_e6744);
        let assign12240_e6746: f64 = (assign12240_e6745).sqrt();
        (assign12240_e6746,)
    } else {
        (var_xmax,)
    }
};
        var_xmax = assign12240_e6748;

        let (assign12250_e6760,) = {
    if (var_guard277 == 0.0) {
        let assign12250_e6753: f64 = (p.p419 * p.p419);
        let assign12250_e6756: f64 = (p.p97 * p.p97);
        let assign12250_e6757: f64 = (assign12250_e6753 + assign12250_e6756);
        let assign12250_e6758: f64 = (assign12250_e6757).sqrt();
        (assign12250_e6758,)
    } else {
        (var_xmax_s,)
    }
};
        var_xmax_s = assign12250_e6760;

        let (assign12260_e6771,) = {
    if (var_guard277 == 0.0) {
        let assign12260_e6767: f64 = (var_wg).powf(p.p425);
        let assign12260_e6768: f64 = (p.p424 / assign12260_e6767);
        let assign12260_e6769: f64 = (1.0 + assign12260_e6768);
        (assign12260_e6769,)
    } else {
        (var_rdrvmaxwe,)
    }
};
        var_rdrvmaxwe = assign12260_e6771;

        let (assign12270_e6782,) = {
    if (var_guard277 == 0.0) {
        let assign12270_e6778: f64 = (var_lg).powf(p.p427);
        let assign12270_e6779: f64 = (p.p426 / assign12270_e6778);
        let assign12270_e6780: f64 = (1.0 + assign12270_e6779);
        (assign12270_e6780,)
    } else {
        (var_rdrvmaxle,)
    }
};
        var_rdrvmaxle = assign12270_e6782;

        let (assign12280_e6793,) = {
    if (var_guard277 == 0.0) {
        let assign12280_e6789: f64 = (var_lg).powf(p.p429);
        let assign12280_e6790: f64 = (p.p428 / assign12280_e6789);
        let assign12280_e6791: f64 = (1.0 + assign12280_e6790);
        (assign12280_e6791,)
    } else {
        (var_rdrmuele,)
    }
};
        var_rdrmuele = assign12280_e6793;

        let (assign12290_e6798, assign12290_e6798_d_n0, assign12290_e6798_d_n2, assign12290_e6798_d_n4, assign12290_e6798_d_n5, assign12290_e6798_d_n6, assign12290_e6798_d_n7, assign12290_e6798_d_n8, assign12290_e6798_d_n9, assign12290_e6798_d_n10, assign12290_e6798_d_n11, assign12290_e6798_d_n14,) = {
    if (var_guard277 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdrmuevbs, var_rdrmuevbs_dn0, var_rdrmuevbs_dn2, var_rdrmuevbs_dn4, var_rdrmuevbs_dn5, var_rdrmuevbs_dn6, var_rdrmuevbs_dn7, var_rdrmuevbs_dn8, var_rdrmuevbs_dn9, var_rdrmuevbs_dn10, var_rdrmuevbs_dn11, var_rdrmuevbs_dn14,)
    }
};
        var_rdrmuevbs = assign12290_e6798;
        var_rdrmuevbs_dn0 = assign12290_e6798_d_n0;
        var_rdrmuevbs_dn2 = assign12290_e6798_d_n2;
        var_rdrmuevbs_dn4 = assign12290_e6798_d_n4;
        var_rdrmuevbs_dn5 = assign12290_e6798_d_n5;
        var_rdrmuevbs_dn6 = assign12290_e6798_d_n6;
        var_rdrmuevbs_dn7 = assign12290_e6798_d_n7;
        var_rdrmuevbs_dn8 = assign12290_e6798_d_n8;
        var_rdrmuevbs_dn9 = assign12290_e6798_d_n9;
        var_rdrmuevbs_dn10 = assign12290_e6798_d_n10;
        var_rdrmuevbs_dn11 = assign12290_e6798_d_n11;
        var_rdrmuevbs_dn14 = assign12290_e6798_d_n14;

        let (assign12300_e6803,) = {
    if (var_guard277 == 0.0) {
        (0.0,)
    } else {
        (var_rdtemp0,)
    }
};
        var_rdtemp0 = assign12300_e6803;

        let (assign12310_e6808, assign12310_e6808_d_n0, assign12310_e6808_d_n2, assign12310_e6808_d_n4, assign12310_e6808_d_n5, assign12310_e6808_d_n6, assign12310_e6808_d_n7, assign12310_e6808_d_n8, assign12310_e6808_d_n9, assign12310_e6808_d_n10, assign12310_e6808_d_n11, assign12310_e6808_d_n14,) = {
    if (var_guard277 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdvdtemp0, var_rdvdtemp0_dn0, var_rdvdtemp0_dn2, var_rdvdtemp0_dn4, var_rdvdtemp0_dn5, var_rdvdtemp0_dn6, var_rdvdtemp0_dn7, var_rdvdtemp0_dn8, var_rdvdtemp0_dn9, var_rdvdtemp0_dn10, var_rdvdtemp0_dn11, var_rdvdtemp0_dn14,)
    }
};
        var_rdvdtemp0 = assign12310_e6808;
        var_rdvdtemp0_dn0 = assign12310_e6808_d_n0;
        var_rdvdtemp0_dn2 = assign12310_e6808_d_n2;
        var_rdvdtemp0_dn4 = assign12310_e6808_d_n4;
        var_rdvdtemp0_dn5 = assign12310_e6808_d_n5;
        var_rdvdtemp0_dn6 = assign12310_e6808_d_n6;
        var_rdvdtemp0_dn7 = assign12310_e6808_d_n7;
        var_rdvdtemp0_dn8 = assign12310_e6808_d_n8;
        var_rdvdtemp0_dn9 = assign12310_e6808_d_n9;
        var_rdvdtemp0_dn10 = assign12310_e6808_d_n10;
        var_rdvdtemp0_dn11 = assign12310_e6808_d_n11;
        var_rdvdtemp0_dn14 = assign12310_e6808_d_n14;

        let (assign12320_e6813, assign12320_e6813_d_n0, assign12320_e6813_d_n2, assign12320_e6813_d_n4, assign12320_e6813_d_n5, assign12320_e6813_d_n6, assign12320_e6813_d_n7, assign12320_e6813_d_n8, assign12320_e6813_d_n9, assign12320_e6813_d_n10, assign12320_e6813_d_n11, assign12320_e6813_d_n14,) = {
    if (var_guard277 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rd23e, var_rd23e_dn0, var_rd23e_dn2, var_rd23e_dn4, var_rd23e_dn5, var_rd23e_dn6, var_rd23e_dn7, var_rd23e_dn8, var_rd23e_dn9, var_rd23e_dn10, var_rd23e_dn11, var_rd23e_dn14,)
    }
};
        var_rd23e = assign12320_e6813;
        var_rd23e_dn0 = assign12320_e6813_d_n0;
        var_rd23e_dn2 = assign12320_e6813_d_n2;
        var_rd23e_dn4 = assign12320_e6813_d_n4;
        var_rd23e_dn5 = assign12320_e6813_d_n5;
        var_rd23e_dn6 = assign12320_e6813_d_n6;
        var_rd23e_dn7 = assign12320_e6813_d_n7;
        var_rd23e_dn8 = assign12320_e6813_d_n8;
        var_rd23e_dn9 = assign12320_e6813_d_n9;
        var_rd23e_dn10 = assign12320_e6813_d_n10;
        var_rd23e_dn11 = assign12320_e6813_d_n11;
        var_rd23e_dn14 = assign12320_e6813_d_n14;

        let assign12330_e6816: f64 = if var_uc_nover > 0.0 { 1.0 } else { 0.0 };
        var_guard283 = assign12330_e6816;

        let (assign12340_e6826,) = {
    if (var_guard283 != 0.0) {
        let assign12340_e6820: f64 = (2.0 * 1.034943e-10);
        let assign12340_e6823: f64 = (1.6021918e-19 * var_uc_nover);
        let assign12340_e6824: f64 = (assign12340_e6820 / assign12340_e6823);
        (assign12340_e6824,)
    } else {
        (var_kdep,)
    }
};
        var_kdep = assign12340_e6826;

        let (assign12350_e6842, assign12350_e6842_d_n0, assign12350_e6842_d_n2, assign12350_e6842_d_n4, assign12350_e6842_d_n5, assign12350_e6842_d_n6, assign12350_e6842_d_n7, assign12350_e6842_d_n8, assign12350_e6842_d_n9, assign12350_e6842_d_n10, assign12350_e6842_d_n11, assign12350_e6842_d_n14,) = {
    if (var_guard283 != 0.0) {
        let assign12350_e6830: f64 = (2.0 * 1.034943e-10);
        let assign12350_e6832: f64 = (assign12350_e6830 / 1.6021918e-19);
        let assign12350_e6834: f64 = (assign12350_e6832 * var_ef_nsubc);
        let assign12350_e6837: f64 = (var_uc_nover + var_ef_nsubc);
        let assign12350_e6838: f64 = (assign12350_e6834 / assign12350_e6837);
        let assign12350_e6840: f64 = (assign12350_e6838 / var_uc_nover);
        (assign12350_e6840, (((((assign12350_e6832 * var_ef_nsubc_dn0) * assign12350_e6837) - (assign12350_e6834 * var_ef_nsubc_dn0)) / (assign12350_e6837 * assign12350_e6837)) / var_uc_nover), (((((assign12350_e6832 * var_ef_nsubc_dn2) * assign12350_e6837) - (assign12350_e6834 * var_ef_nsubc_dn2)) / (assign12350_e6837 * assign12350_e6837)) / var_uc_nover), (((((assign12350_e6832 * var_ef_nsubc_dn4) * assign12350_e6837) - (assign12350_e6834 * var_ef_nsubc_dn4)) / (assign12350_e6837 * assign12350_e6837)) / var_uc_nover), (((((assign12350_e6832 * var_ef_nsubc_dn5) * assign12350_e6837) - (assign12350_e6834 * var_ef_nsubc_dn5)) / (assign12350_e6837 * assign12350_e6837)) / var_uc_nover), (((((assign12350_e6832 * var_ef_nsubc_dn6) * assign12350_e6837) - (assign12350_e6834 * var_ef_nsubc_dn6)) / (assign12350_e6837 * assign12350_e6837)) / var_uc_nover), (((((assign12350_e6832 * var_ef_nsubc_dn7) * assign12350_e6837) - (assign12350_e6834 * var_ef_nsubc_dn7)) / (assign12350_e6837 * assign12350_e6837)) / var_uc_nover), (((((assign12350_e6832 * var_ef_nsubc_dn8) * assign12350_e6837) - (assign12350_e6834 * var_ef_nsubc_dn8)) / (assign12350_e6837 * assign12350_e6837)) / var_uc_nover), (((((assign12350_e6832 * var_ef_nsubc_dn9) * assign12350_e6837) - (assign12350_e6834 * var_ef_nsubc_dn9)) / (assign12350_e6837 * assign12350_e6837)) / var_uc_nover), (((((assign12350_e6832 * var_ef_nsubc_dn10) * assign12350_e6837) - (assign12350_e6834 * var_ef_nsubc_dn10)) / (assign12350_e6837 * assign12350_e6837)) / var_uc_nover), (((((assign12350_e6832 * var_ef_nsubc_dn11) * assign12350_e6837) - (assign12350_e6834 * var_ef_nsubc_dn11)) / (assign12350_e6837 * assign12350_e6837)) / var_uc_nover), (((((assign12350_e6832 * var_ef_nsubc_dn14) * assign12350_e6837) - (assign12350_e6834 * var_ef_nsubc_dn14)) / (assign12350_e6837 * assign12350_e6837)) / var_uc_nover),)
    } else {
        (var_kjunc, var_kjunc_dn0, var_kjunc_dn2, var_kjunc_dn4, var_kjunc_dn5, var_kjunc_dn6, var_kjunc_dn7, var_kjunc_dn8, var_kjunc_dn9, var_kjunc_dn10, var_kjunc_dn11, var_kjunc_dn14,)
    }
};
        var_kjunc = assign12350_e6842;
        var_kjunc_dn0 = assign12350_e6842_d_n0;
        var_kjunc_dn2 = assign12350_e6842_d_n2;
        var_kjunc_dn4 = assign12350_e6842_d_n4;
        var_kjunc_dn5 = assign12350_e6842_d_n5;
        var_kjunc_dn6 = assign12350_e6842_d_n6;
        var_kjunc_dn7 = assign12350_e6842_d_n7;
        var_kjunc_dn8 = assign12350_e6842_d_n8;
        var_kjunc_dn9 = assign12350_e6842_d_n9;
        var_kjunc_dn10 = assign12350_e6842_d_n10;
        var_kjunc_dn11 = assign12350_e6842_d_n11;
        var_kjunc_dn14 = assign12350_e6842_d_n14;

        let (assign12360_e6847,) = {
    if (var_guard283 == 0.0) {
        (0.0,)
    } else {
        (var_kdep,)
    }
};
        var_kdep = assign12360_e6847;

        let (assign12370_e6852, assign12370_e6852_d_n0, assign12370_e6852_d_n2, assign12370_e6852_d_n4, assign12370_e6852_d_n5, assign12370_e6852_d_n6, assign12370_e6852_d_n7, assign12370_e6852_d_n8, assign12370_e6852_d_n9, assign12370_e6852_d_n10, assign12370_e6852_d_n11, assign12370_e6852_d_n14,) = {
    if (var_guard283 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_kjunc, var_kjunc_dn0, var_kjunc_dn2, var_kjunc_dn4, var_kjunc_dn5, var_kjunc_dn6, var_kjunc_dn7, var_kjunc_dn8, var_kjunc_dn9, var_kjunc_dn10, var_kjunc_dn11, var_kjunc_dn14,)
    }
};
        var_kjunc = assign12370_e6852;
        var_kjunc_dn0 = assign12370_e6852_d_n0;
        var_kjunc_dn2 = assign12370_e6852_d_n2;
        var_kjunc_dn4 = assign12370_e6852_d_n4;
        var_kjunc_dn5 = assign12370_e6852_d_n5;
        var_kjunc_dn6 = assign12370_e6852_d_n6;
        var_kjunc_dn7 = assign12370_e6852_d_n7;
        var_kjunc_dn8 = assign12370_e6852_d_n8;
        var_kjunc_dn9 = assign12370_e6852_d_n9;
        var_kjunc_dn10 = assign12370_e6852_d_n10;
        var_kjunc_dn11 = assign12370_e6852_d_n11;
        var_kjunc_dn14 = assign12370_e6852_d_n14;

        let assign12510_e6947: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard288 = assign12510_e6947;

        let (assign12520_e6955, assign12520_e6955_d_n0, assign12520_e6955_d_n2, assign12520_e6955_d_n4, assign12520_e6955_d_n5, assign12520_e6955_d_n6, assign12520_e6955_d_n7, assign12520_e6955_d_n8, assign12520_e6955_d_n9, assign12520_e6955_d_n10, assign12520_e6955_d_n11, assign12520_e6955_d_n14,) = {
    if (var_guard288 != 0.0) {
        let assign12520_e6951: f64 = (p.p108 * var_lg);
        let assign12520_e6953: f64 = (assign12520_e6951 + p.p109);
        (assign12520_e6953, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign12520_e6955;
        var_t1_dn0 = assign12520_e6955_d_n0;
        var_t1_dn2 = assign12520_e6955_d_n2;
        var_t1_dn4 = assign12520_e6955_d_n4;
        var_t1_dn5 = assign12520_e6955_d_n5;
        var_t1_dn6 = assign12520_e6955_d_n6;
        var_t1_dn7 = assign12520_e6955_d_n7;
        var_t1_dn8 = assign12520_e6955_d_n8;
        var_t1_dn9 = assign12520_e6955_d_n9;
        var_t1_dn10 = assign12520_e6955_d_n10;
        var_t1_dn11 = assign12520_e6955_d_n11;
        var_t1_dn14 = assign12520_e6955_d_n14;

        let assign12530_e6958: f64 = if var_t1 < 0.0 { 1.0 } else { 0.0 };
        var_guard289 = assign12530_e6958;

        let (assign12540_e6964, assign12540_e6964_d_n0, assign12540_e6964_d_n2, assign12540_e6964_d_n4, assign12540_e6964_d_n5, assign12540_e6964_d_n6, assign12540_e6964_d_n7, assign12540_e6964_d_n8, assign12540_e6964_d_n9, assign12540_e6964_d_n10, assign12540_e6964_d_n11, assign12540_e6964_d_n14,) = {
    if ((var_guard288 != 0.0) && (var_guard289 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign12540_e6964;
        var_t1_dn0 = assign12540_e6964_d_n0;
        var_t1_dn2 = assign12540_e6964_d_n2;
        var_t1_dn4 = assign12540_e6964_d_n4;
        var_t1_dn5 = assign12540_e6964_d_n5;
        var_t1_dn6 = assign12540_e6964_d_n6;
        var_t1_dn7 = assign12540_e6964_d_n7;
        var_t1_dn8 = assign12540_e6964_d_n8;
        var_t1_dn9 = assign12540_e6964_d_n9;
        var_t1_dn10 = assign12540_e6964_d_n10;
        var_t1_dn11 = assign12540_e6964_d_n11;
        var_t1_dn14 = assign12540_e6964_d_n14;

        let (assign12550_e6976, assign12550_e6976_d_n0, assign12550_e6976_d_n2, assign12550_e6976_d_n4, assign12550_e6976_d_n5, assign12550_e6976_d_n6, assign12550_e6976_d_n7, assign12550_e6976_d_n8, assign12550_e6976_d_n9, assign12550_e6976_d_n10, assign12550_e6976_d_n11, assign12550_e6976_d_n14,) = {
    if (var_guard288 != 0.0) {
        let assign12550_e6968: f64 = (var_t1 * p.p107);
        let assign12550_e6971: f64 = (var_t1 + p.p107);
        let assign12550_e6972: f64 = (assign12550_e6968 / assign12550_e6971);
        let assign12550_e6974: f64 = (assign12550_e6972 + 1.0);
        (assign12550_e6974, ((((var_t1_dn0 * p.p107) * assign12550_e6971) - (assign12550_e6968 * var_t1_dn0)) / (assign12550_e6971 * assign12550_e6971)), ((((var_t1_dn2 * p.p107) * assign12550_e6971) - (assign12550_e6968 * var_t1_dn2)) / (assign12550_e6971 * assign12550_e6971)), ((((var_t1_dn4 * p.p107) * assign12550_e6971) - (assign12550_e6968 * var_t1_dn4)) / (assign12550_e6971 * assign12550_e6971)), ((((var_t1_dn5 * p.p107) * assign12550_e6971) - (assign12550_e6968 * var_t1_dn5)) / (assign12550_e6971 * assign12550_e6971)), ((((var_t1_dn6 * p.p107) * assign12550_e6971) - (assign12550_e6968 * var_t1_dn6)) / (assign12550_e6971 * assign12550_e6971)), ((((var_t1_dn7 * p.p107) * assign12550_e6971) - (assign12550_e6968 * var_t1_dn7)) / (assign12550_e6971 * assign12550_e6971)), ((((var_t1_dn8 * p.p107) * assign12550_e6971) - (assign12550_e6968 * var_t1_dn8)) / (assign12550_e6971 * assign12550_e6971)), ((((var_t1_dn9 * p.p107) * assign12550_e6971) - (assign12550_e6968 * var_t1_dn9)) / (assign12550_e6971 * assign12550_e6971)), ((((var_t1_dn10 * p.p107) * assign12550_e6971) - (assign12550_e6968 * var_t1_dn10)) / (assign12550_e6971 * assign12550_e6971)), ((((var_t1_dn11 * p.p107) * assign12550_e6971) - (assign12550_e6968 * var_t1_dn11)) / (assign12550_e6971 * assign12550_e6971)), ((((var_t1_dn14 * p.p107) * assign12550_e6971) - (assign12550_e6968 * var_t1_dn14)) / (assign12550_e6971 * assign12550_e6971)),)
    } else {
        (var_ddlte, var_ddlte_dn0, var_ddlte_dn2, var_ddlte_dn4, var_ddlte_dn5, var_ddlte_dn6, var_ddlte_dn7, var_ddlte_dn8, var_ddlte_dn9, var_ddlte_dn10, var_ddlte_dn11, var_ddlte_dn14,)
    }
};
        var_ddlte = assign12550_e6976;
        var_ddlte_dn0 = assign12550_e6976_d_n0;
        var_ddlte_dn2 = assign12550_e6976_d_n2;
        var_ddlte_dn4 = assign12550_e6976_d_n4;
        var_ddlte_dn5 = assign12550_e6976_d_n5;
        var_ddlte_dn6 = assign12550_e6976_d_n6;
        var_ddlte_dn7 = assign12550_e6976_d_n7;
        var_ddlte_dn8 = assign12550_e6976_d_n8;
        var_ddlte_dn9 = assign12550_e6976_d_n9;
        var_ddlte_dn10 = assign12550_e6976_d_n10;
        var_ddlte_dn11 = assign12550_e6976_d_n11;
        var_ddlte_dn14 = assign12550_e6976_d_n14;

        let (assign12560_e6983, assign12560_e6983_d_n0, assign12560_e6983_d_n2, assign12560_e6983_d_n4, assign12560_e6983_d_n5, assign12560_e6983_d_n6, assign12560_e6983_d_n7, assign12560_e6983_d_n8, assign12560_e6983_d_n9, assign12560_e6983_d_n10, assign12560_e6983_d_n11, assign12560_e6983_d_n14,) = {
    if (var_guard288 == 0.0) {
        let assign12560_e6981: f64 = (p.p108 * var_lg);
        (assign12560_e6981, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign12560_e6983;
        var_t1_dn0 = assign12560_e6983_d_n0;
        var_t1_dn2 = assign12560_e6983_d_n2;
        var_t1_dn4 = assign12560_e6983_d_n4;
        var_t1_dn5 = assign12560_e6983_d_n5;
        var_t1_dn6 = assign12560_e6983_d_n6;
        var_t1_dn7 = assign12560_e6983_d_n7;
        var_t1_dn8 = assign12560_e6983_d_n8;
        var_t1_dn9 = assign12560_e6983_d_n9;
        var_t1_dn10 = assign12560_e6983_d_n10;
        var_t1_dn11 = assign12560_e6983_d_n11;
        var_t1_dn14 = assign12560_e6983_d_n14;

        let assign12570_e6986: f64 = if var_t1 < 0.0 { 1.0 } else { 0.0 };
        var_guard290 = assign12570_e6986;

        let (assign12580_e6993, assign12580_e6993_d_n0, assign12580_e6993_d_n2, assign12580_e6993_d_n4, assign12580_e6993_d_n5, assign12580_e6993_d_n6, assign12580_e6993_d_n7, assign12580_e6993_d_n8, assign12580_e6993_d_n9, assign12580_e6993_d_n10, assign12580_e6993_d_n11, assign12580_e6993_d_n14,) = {
    if ((var_guard288 == 0.0) && (var_guard290 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign12580_e6993;
        var_t1_dn0 = assign12580_e6993_d_n0;
        var_t1_dn2 = assign12580_e6993_d_n2;
        var_t1_dn4 = assign12580_e6993_d_n4;
        var_t1_dn5 = assign12580_e6993_d_n5;
        var_t1_dn6 = assign12580_e6993_d_n6;
        var_t1_dn7 = assign12580_e6993_d_n7;
        var_t1_dn8 = assign12580_e6993_d_n8;
        var_t1_dn9 = assign12580_e6993_d_n9;
        var_t1_dn10 = assign12580_e6993_d_n10;
        var_t1_dn11 = assign12580_e6993_d_n11;
        var_t1_dn14 = assign12580_e6993_d_n14;

        let (assign12590_e7008, assign12590_e7008_d_n0, assign12590_e7008_d_n2, assign12590_e7008_d_n4, assign12590_e7008_d_n5, assign12590_e7008_d_n6, assign12590_e7008_d_n7, assign12590_e7008_d_n8, assign12590_e7008_d_n9, assign12590_e7008_d_n10, assign12590_e7008_d_n11, assign12590_e7008_d_n14,) = {
    if (var_guard288 == 0.0) {
        let assign12590_e6998: f64 = (var_t1 * p.p107);
        let assign12590_e7001: f64 = (var_t1 + p.p107);
        let assign12590_e7002: f64 = (assign12590_e6998 / assign12590_e7001);
        let assign12590_e7004: f64 = (assign12590_e7002 + p.p109);
        let assign12590_e7006: f64 = (assign12590_e7004 + 1e-25);
        (assign12590_e7006, ((((var_t1_dn0 * p.p107) * assign12590_e7001) - (assign12590_e6998 * var_t1_dn0)) / (assign12590_e7001 * assign12590_e7001)), ((((var_t1_dn2 * p.p107) * assign12590_e7001) - (assign12590_e6998 * var_t1_dn2)) / (assign12590_e7001 * assign12590_e7001)), ((((var_t1_dn4 * p.p107) * assign12590_e7001) - (assign12590_e6998 * var_t1_dn4)) / (assign12590_e7001 * assign12590_e7001)), ((((var_t1_dn5 * p.p107) * assign12590_e7001) - (assign12590_e6998 * var_t1_dn5)) / (assign12590_e7001 * assign12590_e7001)), ((((var_t1_dn6 * p.p107) * assign12590_e7001) - (assign12590_e6998 * var_t1_dn6)) / (assign12590_e7001 * assign12590_e7001)), ((((var_t1_dn7 * p.p107) * assign12590_e7001) - (assign12590_e6998 * var_t1_dn7)) / (assign12590_e7001 * assign12590_e7001)), ((((var_t1_dn8 * p.p107) * assign12590_e7001) - (assign12590_e6998 * var_t1_dn8)) / (assign12590_e7001 * assign12590_e7001)), ((((var_t1_dn9 * p.p107) * assign12590_e7001) - (assign12590_e6998 * var_t1_dn9)) / (assign12590_e7001 * assign12590_e7001)), ((((var_t1_dn10 * p.p107) * assign12590_e7001) - (assign12590_e6998 * var_t1_dn10)) / (assign12590_e7001 * assign12590_e7001)), ((((var_t1_dn11 * p.p107) * assign12590_e7001) - (assign12590_e6998 * var_t1_dn11)) / (assign12590_e7001 * assign12590_e7001)), ((((var_t1_dn14 * p.p107) * assign12590_e7001) - (assign12590_e6998 * var_t1_dn14)) / (assign12590_e7001 * assign12590_e7001)),)
    } else {
        (var_ddlte, var_ddlte_dn0, var_ddlte_dn2, var_ddlte_dn4, var_ddlte_dn5, var_ddlte_dn6, var_ddlte_dn7, var_ddlte_dn8, var_ddlte_dn9, var_ddlte_dn10, var_ddlte_dn11, var_ddlte_dn14,)
    }
};
        var_ddlte = assign12590_e7008;
        var_ddlte_dn0 = assign12590_e7008_d_n0;
        var_ddlte_dn2 = assign12590_e7008_d_n2;
        var_ddlte_dn4 = assign12590_e7008_d_n4;
        var_ddlte_dn5 = assign12590_e7008_d_n5;
        var_ddlte_dn6 = assign12590_e7008_d_n6;
        var_ddlte_dn7 = assign12590_e7008_d_n7;
        var_ddlte_dn8 = assign12590_e7008_d_n8;
        var_ddlte_dn9 = assign12590_e7008_d_n9;
        var_ddlte_dn10 = assign12590_e7008_d_n10;
        var_ddlte_dn11 = assign12590_e7008_d_n11;
        var_ddlte_dn14 = assign12590_e7008_d_n14;

        let assign12610_e7016: f64 = if var_ddlte < 0.1 { 1.0 } else { 0.0 };
        var_guard292 = assign12610_e7016;

        let (assign12620_e7020, assign12620_e7020_d_n0, assign12620_e7020_d_n2, assign12620_e7020_d_n4, assign12620_e7020_d_n5, assign12620_e7020_d_n6, assign12620_e7020_d_n7, assign12620_e7020_d_n8, assign12620_e7020_d_n9, assign12620_e7020_d_n10, assign12620_e7020_d_n11, assign12620_e7020_d_n14,) = {
    if (var_guard292 != 0.0) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ddlte, var_ddlte_dn0, var_ddlte_dn2, var_ddlte_dn4, var_ddlte_dn5, var_ddlte_dn6, var_ddlte_dn7, var_ddlte_dn8, var_ddlte_dn9, var_ddlte_dn10, var_ddlte_dn11, var_ddlte_dn14,)
    }
};
        var_ddlte = assign12620_e7020;
        var_ddlte_dn0 = assign12620_e7020_d_n0;
        var_ddlte_dn2 = assign12620_e7020_d_n2;
        var_ddlte_dn4 = assign12620_e7020_d_n4;
        var_ddlte_dn5 = assign12620_e7020_d_n5;
        var_ddlte_dn6 = assign12620_e7020_d_n6;
        var_ddlte_dn7 = assign12620_e7020_d_n7;
        var_ddlte_dn8 = assign12620_e7020_d_n8;
        var_ddlte_dn9 = assign12620_e7020_d_n9;
        var_ddlte_dn10 = assign12620_e7020_d_n10;
        var_ddlte_dn11 = assign12620_e7020_d_n11;
        var_ddlte_dn14 = assign12620_e7020_d_n14;

        let (assign12630_e7026, assign12630_e7026_d_n0, assign12630_e7026_d_n2, assign12630_e7026_d_n4, assign12630_e7026_d_n5, assign12630_e7026_d_n6, assign12630_e7026_d_n7, assign12630_e7026_d_n8, assign12630_e7026_d_n9, assign12630_e7026_d_n10, assign12630_e7026_d_n11, assign12630_e7026_d_n14,) = {
    if (p.p23 != 0.0) {
        let assign12630_e7024: f64 = (var_weff).powf(p.p201);
        (assign12630_e7024, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign12630_e7026;
        var_t2_dn0 = assign12630_e7026_d_n0;
        var_t2_dn2 = assign12630_e7026_d_n2;
        var_t2_dn4 = assign12630_e7026_d_n4;
        var_t2_dn5 = assign12630_e7026_d_n5;
        var_t2_dn6 = assign12630_e7026_d_n6;
        var_t2_dn7 = assign12630_e7026_d_n7;
        var_t2_dn8 = assign12630_e7026_d_n8;
        var_t2_dn9 = assign12630_e7026_d_n9;
        var_t2_dn10 = assign12630_e7026_d_n10;
        var_t2_dn11 = assign12630_e7026_d_n11;
        var_t2_dn14 = assign12630_e7026_d_n14;

        let (assign12640_e7044, assign12640_e7044_d_n0, assign12640_e7044_d_n2, assign12640_e7044_d_n4, assign12640_e7044_d_n5, assign12640_e7044_d_n6, assign12640_e7044_d_n7, assign12640_e7044_d_n8, assign12640_e7044_d_n9, assign12640_e7044_d_n10, assign12640_e7044_d_n11, assign12640_e7044_d_n14,) = {
    if (p.p23 != 0.0) {
        let assign12640_e7033: f64 = (var_lgate).powf(p.p199);
        let assign12640_e7034: f64 = (var_mks_svgsl / assign12640_e7033);
        let assign12640_e7035: f64 = (1.0 + assign12640_e7034);
        let assign12640_e7036: f64 = (var_uc_svgs * assign12640_e7035);
        let assign12640_e7040: f64 = (var_t2 + var_mks_svgsw);
        let assign12640_e7041: f64 = (var_t2 / assign12640_e7040);
        let assign12640_e7042: f64 = (assign12640_e7036 * assign12640_e7041);
        (assign12640_e7042, (assign12640_e7036 * (((var_t2_dn0 * assign12640_e7040) - (var_t2 * var_t2_dn0)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((var_t2_dn2 * assign12640_e7040) - (var_t2 * var_t2_dn2)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((var_t2_dn4 * assign12640_e7040) - (var_t2 * var_t2_dn4)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((var_t2_dn5 * assign12640_e7040) - (var_t2 * var_t2_dn5)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((var_t2_dn6 * assign12640_e7040) - (var_t2 * var_t2_dn6)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((var_t2_dn7 * assign12640_e7040) - (var_t2 * var_t2_dn7)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((var_t2_dn8 * assign12640_e7040) - (var_t2 * var_t2_dn8)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((var_t2_dn9 * assign12640_e7040) - (var_t2 * var_t2_dn9)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((var_t2_dn10 * assign12640_e7040) - (var_t2 * var_t2_dn10)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((var_t2_dn11 * assign12640_e7040) - (var_t2 * var_t2_dn11)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((var_t2_dn14 * assign12640_e7040) - (var_t2 * var_t2_dn14)) / (assign12640_e7040 * assign12640_e7040))),)
    } else {
        (var_vg2const, var_vg2const_dn0, var_vg2const_dn2, var_vg2const_dn4, var_vg2const_dn5, var_vg2const_dn6, var_vg2const_dn7, var_vg2const_dn8, var_vg2const_dn9, var_vg2const_dn10, var_vg2const_dn11, var_vg2const_dn14,)
    }
};
        var_vg2const = assign12640_e7044;
        var_vg2const_dn0 = assign12640_e7044_d_n0;
        var_vg2const_dn2 = assign12640_e7044_d_n2;
        var_vg2const_dn4 = assign12640_e7044_d_n4;
        var_vg2const_dn5 = assign12640_e7044_d_n5;
        var_vg2const_dn6 = assign12640_e7044_d_n6;
        var_vg2const_dn7 = assign12640_e7044_d_n7;
        var_vg2const_dn8 = assign12640_e7044_d_n8;
        var_vg2const_dn9 = assign12640_e7044_d_n9;
        var_vg2const_dn10 = assign12640_e7044_d_n10;
        var_vg2const_dn11 = assign12640_e7044_d_n11;
        var_vg2const_dn14 = assign12640_e7044_d_n14;

        let (assign12650_e7056,) = {
    if (p.p23 != 0.0) {
        let assign12650_e7051: f64 = (var_lgate).powf(p.p184);
        let assign12650_e7052: f64 = (var_mks_svbsl / assign12650_e7051);
        let assign12650_e7053: f64 = (1.0 + assign12650_e7052);
        let assign12650_e7054: f64 = (var_uc_svbs * assign12650_e7053);
        (assign12650_e7054,)
    } else {
        (var_xvbs,)
    }
};
        var_xvbs = assign12650_e7056;

        let (assign12660_e7068,) = {
    if (p.p23 != 0.0) {
        let assign12660_e7063: f64 = (var_lgate).powf(p.p203);
        let assign12660_e7064: f64 = (var_mks_slgl / assign12660_e7063);
        let assign12660_e7065: f64 = (1.0 + assign12660_e7064);
        let assign12660_e7066: f64 = (var_mks_slg * assign12660_e7065);
        (assign12660_e7066,)
    } else {
        (var_xgate,)
    }
};
        var_xgate = assign12660_e7068;

        let (assign12670_e7080,) = {
    if (p.p23 != 0.0) {
        let assign12670_e7075: f64 = (var_lgate).powf(p.p191);
        let assign12670_e7076: f64 = (var_mks_sub1l / assign12670_e7075);
        let assign12670_e7077: f64 = (1.0 + assign12670_e7076);
        let assign12670_e7078: f64 = (var_uc_sub1 * assign12670_e7077);
        (assign12670_e7078,)
    } else {
        (var_xsub1,)
    }
};
        var_xsub1 = assign12670_e7080;

        let (assign12680_e7090,) = {
    if (p.p23 != 0.0) {
        let assign12680_e7086: f64 = (var_mks_sub2l / var_lgate);
        let assign12680_e7087: f64 = (1.0 + assign12680_e7086);
        let assign12680_e7088: f64 = (var_uc_sub2 * assign12680_e7087);
        (assign12680_e7088,)
    } else {
        (var_xsub2,)
    }
};
        var_xsub2 = assign12680_e7090;

        *var_ddlte_slot = var_ddlte;
        *var_ddlte_dn0_slot = var_ddlte_dn0;
        *var_ddlte_dn10_slot = var_ddlte_dn10;
        *var_ddlte_dn11_slot = var_ddlte_dn11;
        *var_ddlte_dn14_slot = var_ddlte_dn14;
        *var_ddlte_dn2_slot = var_ddlte_dn2;
        *var_ddlte_dn4_slot = var_ddlte_dn4;
        *var_ddlte_dn5_slot = var_ddlte_dn5;
        *var_ddlte_dn6_slot = var_ddlte_dn6;
        *var_ddlte_dn7_slot = var_ddlte_dn7;
        *var_ddlte_dn8_slot = var_ddlte_dn8;
        *var_ddlte_dn9_slot = var_ddlte_dn9;
        *var_guard283_slot = var_guard283;
        *var_guard288_slot = var_guard288;
        *var_guard289_slot = var_guard289;
        *var_guard290_slot = var_guard290;
        *var_guard292_slot = var_guard292;
        *var_kdep_slot = var_kdep;
        *var_kjunc_slot = var_kjunc;
        *var_kjunc_dn0_slot = var_kjunc_dn0;
        *var_kjunc_dn10_slot = var_kjunc_dn10;
        *var_kjunc_dn11_slot = var_kjunc_dn11;
        *var_kjunc_dn14_slot = var_kjunc_dn14;
        *var_kjunc_dn2_slot = var_kjunc_dn2;
        *var_kjunc_dn4_slot = var_kjunc_dn4;
        *var_kjunc_dn5_slot = var_kjunc_dn5;
        *var_kjunc_dn6_slot = var_kjunc_dn6;
        *var_kjunc_dn7_slot = var_kjunc_dn7;
        *var_kjunc_dn8_slot = var_kjunc_dn8;
        *var_kjunc_dn9_slot = var_kjunc_dn9;
        *var_rd23e_slot = var_rd23e;
        *var_rd23e_dn0_slot = var_rd23e_dn0;
        *var_rd23e_dn10_slot = var_rd23e_dn10;
        *var_rd23e_dn11_slot = var_rd23e_dn11;
        *var_rd23e_dn14_slot = var_rd23e_dn14;
        *var_rd23e_dn2_slot = var_rd23e_dn2;
        *var_rd23e_dn4_slot = var_rd23e_dn4;
        *var_rd23e_dn5_slot = var_rd23e_dn5;
        *var_rd23e_dn6_slot = var_rd23e_dn6;
        *var_rd23e_dn7_slot = var_rd23e_dn7;
        *var_rd23e_dn8_slot = var_rd23e_dn8;
        *var_rd23e_dn9_slot = var_rd23e_dn9;
        *var_rdrmuele_slot = var_rdrmuele;
        *var_rdrmuevbs_slot = var_rdrmuevbs;
        *var_rdrmuevbs_dn0_slot = var_rdrmuevbs_dn0;
        *var_rdrmuevbs_dn10_slot = var_rdrmuevbs_dn10;
        *var_rdrmuevbs_dn11_slot = var_rdrmuevbs_dn11;
        *var_rdrmuevbs_dn14_slot = var_rdrmuevbs_dn14;
        *var_rdrmuevbs_dn2_slot = var_rdrmuevbs_dn2;
        *var_rdrmuevbs_dn4_slot = var_rdrmuevbs_dn4;
        *var_rdrmuevbs_dn5_slot = var_rdrmuevbs_dn5;
        *var_rdrmuevbs_dn6_slot = var_rdrmuevbs_dn6;
        *var_rdrmuevbs_dn7_slot = var_rdrmuevbs_dn7;
        *var_rdrmuevbs_dn8_slot = var_rdrmuevbs_dn8;
        *var_rdrmuevbs_dn9_slot = var_rdrmuevbs_dn9;
        *var_rdrvmaxle_slot = var_rdrvmaxle;
        *var_rdrvmaxwe_slot = var_rdrvmaxwe;
        *var_rdtemp0_slot = var_rdtemp0;
        *var_rdvdtemp0_slot = var_rdvdtemp0;
        *var_rdvdtemp0_dn0_slot = var_rdvdtemp0_dn0;
        *var_rdvdtemp0_dn10_slot = var_rdvdtemp0_dn10;
        *var_rdvdtemp0_dn11_slot = var_rdvdtemp0_dn11;
        *var_rdvdtemp0_dn14_slot = var_rdvdtemp0_dn14;
        *var_rdvdtemp0_dn2_slot = var_rdvdtemp0_dn2;
        *var_rdvdtemp0_dn4_slot = var_rdvdtemp0_dn4;
        *var_rdvdtemp0_dn5_slot = var_rdvdtemp0_dn5;
        *var_rdvdtemp0_dn6_slot = var_rdvdtemp0_dn6;
        *var_rdvdtemp0_dn7_slot = var_rdvdtemp0_dn7;
        *var_rdvdtemp0_dn8_slot = var_rdvdtemp0_dn8;
        *var_rdvdtemp0_dn9_slot = var_rdvdtemp0_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_vg2const_slot = var_vg2const;
        *var_vg2const_dn0_slot = var_vg2const_dn0;
        *var_vg2const_dn10_slot = var_vg2const_dn10;
        *var_vg2const_dn11_slot = var_vg2const_dn11;
        *var_vg2const_dn14_slot = var_vg2const_dn14;
        *var_vg2const_dn2_slot = var_vg2const_dn2;
        *var_vg2const_dn4_slot = var_vg2const_dn4;
        *var_vg2const_dn5_slot = var_vg2const_dn5;
        *var_vg2const_dn6_slot = var_vg2const_dn6;
        *var_vg2const_dn7_slot = var_vg2const_dn7;
        *var_vg2const_dn8_slot = var_vg2const_dn8;
        *var_vg2const_dn9_slot = var_vg2const_dn9;
        *var_xgate_slot = var_xgate;
        *var_xmax_slot = var_xmax;
        *var_xmax_s_slot = var_xmax_s;
        *var_xsub1_slot = var_xsub1;
        *var_xsub2_slot = var_xsub2;
        *var_xvbs_slot = var_xvbs;
    }

    pub(super) fn stamp_transient_block_21(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        var_deltemp: f64,
        var_deltemp_dn0: f64,
        var_deltemp_dn10: f64,
        var_deltemp_dn11: f64,
        var_deltemp_dn14: f64,
        var_deltemp_dn2: f64,
        var_deltemp_dn4: f64,
        var_deltemp_dn5: f64,
        var_deltemp_dn6: f64,
        var_deltemp_dn7: f64,
        var_deltemp_dn8: f64,
        var_deltemp_dn9: f64,
        var_ktnom: f64,
        var_lg: f64,
        var_lgate: f64,
        var_mfactor: f64,
        var_mks_sub1l: f64,
        var_mks_sub2l: f64,
        var_uc_gdld: f64,
        var_uc_rth0: f64,
        var_uc_sub1snp: f64,
        var_uc_sub2snp: f64,
        var_weff_nf: f64,
        var_weffcv_nf: f64,
        var_wg: f64,
        var_cfrng_slot: &mut f64,
        var_cnst0over_slot: &mut f64,
        var_cnst0over_dn0_slot: &mut f64,
        var_cnst0over_dn10_slot: &mut f64,
        var_cnst0over_dn11_slot: &mut f64,
        var_cnst0over_dn14_slot: &mut f64,
        var_cnst0over_dn2_slot: &mut f64,
        var_cnst0over_dn4_slot: &mut f64,
        var_cnst0over_dn5_slot: &mut f64,
        var_cnst0over_dn6_slot: &mut f64,
        var_cnst0over_dn7_slot: &mut f64,
        var_cnst0over_dn8_slot: &mut f64,
        var_cnst0over_dn9_slot: &mut f64,
        var_cnst0overs_slot: &mut f64,
        var_cnst0overs_dn0_slot: &mut f64,
        var_cnst0overs_dn10_slot: &mut f64,
        var_cnst0overs_dn11_slot: &mut f64,
        var_cnst0overs_dn14_slot: &mut f64,
        var_cnst0overs_dn2_slot: &mut f64,
        var_cnst0overs_dn4_slot: &mut f64,
        var_cnst0overs_dn5_slot: &mut f64,
        var_cnst0overs_dn6_slot: &mut f64,
        var_cnst0overs_dn7_slot: &mut f64,
        var_cnst0overs_dn8_slot: &mut f64,
        var_cnst0overs_dn9_slot: &mut f64,
        var_cqyb0_slot: &mut f64,
        var_gdl0_slot: &mut f64,
        var_guard293_slot: &mut f64,
        var_pt40_slot: &mut f64,
        var_ptl0_slot: &mut f64,
        var_rth_slot: &mut f64,
        var_rth_dn0_slot: &mut f64,
        var_rth_dn10_slot: &mut f64,
        var_rth_dn11_slot: &mut f64,
        var_rth_dn14_slot: &mut f64,
        var_rth_dn2_slot: &mut f64,
        var_rth_dn4_slot: &mut f64,
        var_rth_dn5_slot: &mut f64,
        var_rth_dn6_slot: &mut f64,
        var_rth_dn7_slot: &mut f64,
        var_rth_dn8_slot: &mut f64,
        var_rth_dn9_slot: &mut f64,
        var_rthtemp0_slot: &mut f64,
        var_tdiff_slot: &mut f64,
        var_tdiff0_slot: &mut f64,
        var_tdiff0_2_slot: &mut f64,
        var_tdiff0_2_dn0_slot: &mut f64,
        var_tdiff0_2_dn10_slot: &mut f64,
        var_tdiff0_2_dn11_slot: &mut f64,
        var_tdiff0_2_dn14_slot: &mut f64,
        var_tdiff0_2_dn2_slot: &mut f64,
        var_tdiff0_2_dn4_slot: &mut f64,
        var_tdiff0_2_dn5_slot: &mut f64,
        var_tdiff0_2_dn6_slot: &mut f64,
        var_tdiff0_2_dn7_slot: &mut f64,
        var_tdiff0_2_dn8_slot: &mut f64,
        var_tdiff0_2_dn9_slot: &mut f64,
        var_tdiff0_dn0_slot: &mut f64,
        var_tdiff0_dn10_slot: &mut f64,
        var_tdiff0_dn11_slot: &mut f64,
        var_tdiff0_dn14_slot: &mut f64,
        var_tdiff0_dn2_slot: &mut f64,
        var_tdiff0_dn4_slot: &mut f64,
        var_tdiff0_dn5_slot: &mut f64,
        var_tdiff0_dn6_slot: &mut f64,
        var_tdiff0_dn7_slot: &mut f64,
        var_tdiff0_dn8_slot: &mut f64,
        var_tdiff0_dn9_slot: &mut f64,
        var_tdiff_dn0_slot: &mut f64,
        var_tdiff_dn10_slot: &mut f64,
        var_tdiff_dn11_slot: &mut f64,
        var_tdiff_dn14_slot: &mut f64,
        var_tdiff_dn2_slot: &mut f64,
        var_tdiff_dn4_slot: &mut f64,
        var_tdiff_dn5_slot: &mut f64,
        var_tdiff_dn6_slot: &mut f64,
        var_tdiff_dn7_slot: &mut f64,
        var_tdiff_dn8_slot: &mut f64,
        var_tdiff_dn9_slot: &mut f64,
        var_ttemp_slot: &mut f64,
        var_ttemp0_slot: &mut f64,
        var_ttemp0_dn0_slot: &mut f64,
        var_ttemp0_dn10_slot: &mut f64,
        var_ttemp0_dn11_slot: &mut f64,
        var_ttemp0_dn14_slot: &mut f64,
        var_ttemp0_dn2_slot: &mut f64,
        var_ttemp0_dn4_slot: &mut f64,
        var_ttemp0_dn5_slot: &mut f64,
        var_ttemp0_dn6_slot: &mut f64,
        var_ttemp0_dn7_slot: &mut f64,
        var_ttemp0_dn8_slot: &mut f64,
        var_ttemp0_dn9_slot: &mut f64,
        var_ttemp_dn0_slot: &mut f64,
        var_ttemp_dn10_slot: &mut f64,
        var_ttemp_dn11_slot: &mut f64,
        var_ttemp_dn14_slot: &mut f64,
        var_ttemp_dn2_slot: &mut f64,
        var_ttemp_dn4_slot: &mut f64,
        var_ttemp_dn5_slot: &mut f64,
        var_ttemp_dn6_slot: &mut f64,
        var_ttemp_dn7_slot: &mut f64,
        var_ttemp_dn8_slot: &mut f64,
        var_ttemp_dn9_slot: &mut f64,
        var_uc_ibpc1_slot: &mut f64,
        var_uc_subld1_slot: &mut f64,
        var_vg2const_slot: &mut f64,
        var_vg2const_1_slot: &mut f64,
        var_vg2const_1_dn0_slot: &mut f64,
        var_vg2const_1_dn10_slot: &mut f64,
        var_vg2const_1_dn11_slot: &mut f64,
        var_vg2const_1_dn14_slot: &mut f64,
        var_vg2const_1_dn2_slot: &mut f64,
        var_vg2const_1_dn4_slot: &mut f64,
        var_vg2const_1_dn5_slot: &mut f64,
        var_vg2const_1_dn6_slot: &mut f64,
        var_vg2const_1_dn7_slot: &mut f64,
        var_vg2const_1_dn8_slot: &mut f64,
        var_vg2const_1_dn9_slot: &mut f64,
        var_vg2const_dn0_slot: &mut f64,
        var_vg2const_dn10_slot: &mut f64,
        var_vg2const_dn11_slot: &mut f64,
        var_vg2const_dn14_slot: &mut f64,
        var_vg2const_dn2_slot: &mut f64,
        var_vg2const_dn4_slot: &mut f64,
        var_vg2const_dn5_slot: &mut f64,
        var_vg2const_dn6_slot: &mut f64,
        var_vg2const_dn7_slot: &mut f64,
        var_vg2const_dn8_slot: &mut f64,
        var_vg2const_dn9_slot: &mut f64,
        var_xgate_slot: &mut f64,
        var_xgate_1_slot: &mut f64,
        var_xsub1_slot: &mut f64,
        var_xsub1_1_slot: &mut f64,
        var_xsub2_slot: &mut f64,
        var_xsub2_1_slot: &mut f64,
        var_xvbs_slot: &mut f64,
        var_xvbs_1_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let mut var_cfrng: f64 = *var_cfrng_slot;
        let mut var_cnst0over: f64 = *var_cnst0over_slot;
        let mut var_cnst0over_dn0: f64 = *var_cnst0over_dn0_slot;
        let mut var_cnst0over_dn10: f64 = *var_cnst0over_dn10_slot;
        let mut var_cnst0over_dn11: f64 = *var_cnst0over_dn11_slot;
        let mut var_cnst0over_dn14: f64 = *var_cnst0over_dn14_slot;
        let mut var_cnst0over_dn2: f64 = *var_cnst0over_dn2_slot;
        let mut var_cnst0over_dn4: f64 = *var_cnst0over_dn4_slot;
        let mut var_cnst0over_dn5: f64 = *var_cnst0over_dn5_slot;
        let mut var_cnst0over_dn6: f64 = *var_cnst0over_dn6_slot;
        let mut var_cnst0over_dn7: f64 = *var_cnst0over_dn7_slot;
        let mut var_cnst0over_dn8: f64 = *var_cnst0over_dn8_slot;
        let mut var_cnst0over_dn9: f64 = *var_cnst0over_dn9_slot;
        let mut var_cnst0overs: f64 = *var_cnst0overs_slot;
        let mut var_cnst0overs_dn0: f64 = *var_cnst0overs_dn0_slot;
        let mut var_cnst0overs_dn10: f64 = *var_cnst0overs_dn10_slot;
        let mut var_cnst0overs_dn11: f64 = *var_cnst0overs_dn11_slot;
        let mut var_cnst0overs_dn14: f64 = *var_cnst0overs_dn14_slot;
        let mut var_cnst0overs_dn2: f64 = *var_cnst0overs_dn2_slot;
        let mut var_cnst0overs_dn4: f64 = *var_cnst0overs_dn4_slot;
        let mut var_cnst0overs_dn5: f64 = *var_cnst0overs_dn5_slot;
        let mut var_cnst0overs_dn6: f64 = *var_cnst0overs_dn6_slot;
        let mut var_cnst0overs_dn7: f64 = *var_cnst0overs_dn7_slot;
        let mut var_cnst0overs_dn8: f64 = *var_cnst0overs_dn8_slot;
        let mut var_cnst0overs_dn9: f64 = *var_cnst0overs_dn9_slot;
        let mut var_cqyb0: f64 = *var_cqyb0_slot;
        let mut var_gdl0: f64 = *var_gdl0_slot;
        let mut var_guard293: f64 = *var_guard293_slot;
        let mut var_pt40: f64 = *var_pt40_slot;
        let mut var_ptl0: f64 = *var_ptl0_slot;
        let mut var_rth: f64 = *var_rth_slot;
        let mut var_rth_dn0: f64 = *var_rth_dn0_slot;
        let mut var_rth_dn10: f64 = *var_rth_dn10_slot;
        let mut var_rth_dn11: f64 = *var_rth_dn11_slot;
        let mut var_rth_dn14: f64 = *var_rth_dn14_slot;
        let mut var_rth_dn2: f64 = *var_rth_dn2_slot;
        let mut var_rth_dn4: f64 = *var_rth_dn4_slot;
        let mut var_rth_dn5: f64 = *var_rth_dn5_slot;
        let mut var_rth_dn6: f64 = *var_rth_dn6_slot;
        let mut var_rth_dn7: f64 = *var_rth_dn7_slot;
        let mut var_rth_dn8: f64 = *var_rth_dn8_slot;
        let mut var_rth_dn9: f64 = *var_rth_dn9_slot;
        let mut var_rthtemp0: f64 = *var_rthtemp0_slot;
        let mut var_tdiff: f64 = *var_tdiff_slot;
        let mut var_tdiff0: f64 = *var_tdiff0_slot;
        let mut var_tdiff0_2: f64 = *var_tdiff0_2_slot;
        let mut var_tdiff0_2_dn0: f64 = *var_tdiff0_2_dn0_slot;
        let mut var_tdiff0_2_dn10: f64 = *var_tdiff0_2_dn10_slot;
        let mut var_tdiff0_2_dn11: f64 = *var_tdiff0_2_dn11_slot;
        let mut var_tdiff0_2_dn14: f64 = *var_tdiff0_2_dn14_slot;
        let mut var_tdiff0_2_dn2: f64 = *var_tdiff0_2_dn2_slot;
        let mut var_tdiff0_2_dn4: f64 = *var_tdiff0_2_dn4_slot;
        let mut var_tdiff0_2_dn5: f64 = *var_tdiff0_2_dn5_slot;
        let mut var_tdiff0_2_dn6: f64 = *var_tdiff0_2_dn6_slot;
        let mut var_tdiff0_2_dn7: f64 = *var_tdiff0_2_dn7_slot;
        let mut var_tdiff0_2_dn8: f64 = *var_tdiff0_2_dn8_slot;
        let mut var_tdiff0_2_dn9: f64 = *var_tdiff0_2_dn9_slot;
        let mut var_tdiff0_dn0: f64 = *var_tdiff0_dn0_slot;
        let mut var_tdiff0_dn10: f64 = *var_tdiff0_dn10_slot;
        let mut var_tdiff0_dn11: f64 = *var_tdiff0_dn11_slot;
        let mut var_tdiff0_dn14: f64 = *var_tdiff0_dn14_slot;
        let mut var_tdiff0_dn2: f64 = *var_tdiff0_dn2_slot;
        let mut var_tdiff0_dn4: f64 = *var_tdiff0_dn4_slot;
        let mut var_tdiff0_dn5: f64 = *var_tdiff0_dn5_slot;
        let mut var_tdiff0_dn6: f64 = *var_tdiff0_dn6_slot;
        let mut var_tdiff0_dn7: f64 = *var_tdiff0_dn7_slot;
        let mut var_tdiff0_dn8: f64 = *var_tdiff0_dn8_slot;
        let mut var_tdiff0_dn9: f64 = *var_tdiff0_dn9_slot;
        let mut var_tdiff_dn0: f64 = *var_tdiff_dn0_slot;
        let mut var_tdiff_dn10: f64 = *var_tdiff_dn10_slot;
        let mut var_tdiff_dn11: f64 = *var_tdiff_dn11_slot;
        let mut var_tdiff_dn14: f64 = *var_tdiff_dn14_slot;
        let mut var_tdiff_dn2: f64 = *var_tdiff_dn2_slot;
        let mut var_tdiff_dn4: f64 = *var_tdiff_dn4_slot;
        let mut var_tdiff_dn5: f64 = *var_tdiff_dn5_slot;
        let mut var_tdiff_dn6: f64 = *var_tdiff_dn6_slot;
        let mut var_tdiff_dn7: f64 = *var_tdiff_dn7_slot;
        let mut var_tdiff_dn8: f64 = *var_tdiff_dn8_slot;
        let mut var_tdiff_dn9: f64 = *var_tdiff_dn9_slot;
        let mut var_ttemp: f64 = *var_ttemp_slot;
        let mut var_ttemp0: f64 = *var_ttemp0_slot;
        let mut var_ttemp0_dn0: f64 = *var_ttemp0_dn0_slot;
        let mut var_ttemp0_dn10: f64 = *var_ttemp0_dn10_slot;
        let mut var_ttemp0_dn11: f64 = *var_ttemp0_dn11_slot;
        let mut var_ttemp0_dn14: f64 = *var_ttemp0_dn14_slot;
        let mut var_ttemp0_dn2: f64 = *var_ttemp0_dn2_slot;
        let mut var_ttemp0_dn4: f64 = *var_ttemp0_dn4_slot;
        let mut var_ttemp0_dn5: f64 = *var_ttemp0_dn5_slot;
        let mut var_ttemp0_dn6: f64 = *var_ttemp0_dn6_slot;
        let mut var_ttemp0_dn7: f64 = *var_ttemp0_dn7_slot;
        let mut var_ttemp0_dn8: f64 = *var_ttemp0_dn8_slot;
        let mut var_ttemp0_dn9: f64 = *var_ttemp0_dn9_slot;
        let mut var_ttemp_dn0: f64 = *var_ttemp_dn0_slot;
        let mut var_ttemp_dn10: f64 = *var_ttemp_dn10_slot;
        let mut var_ttemp_dn11: f64 = *var_ttemp_dn11_slot;
        let mut var_ttemp_dn14: f64 = *var_ttemp_dn14_slot;
        let mut var_ttemp_dn2: f64 = *var_ttemp_dn2_slot;
        let mut var_ttemp_dn4: f64 = *var_ttemp_dn4_slot;
        let mut var_ttemp_dn5: f64 = *var_ttemp_dn5_slot;
        let mut var_ttemp_dn6: f64 = *var_ttemp_dn6_slot;
        let mut var_ttemp_dn7: f64 = *var_ttemp_dn7_slot;
        let mut var_ttemp_dn8: f64 = *var_ttemp_dn8_slot;
        let mut var_ttemp_dn9: f64 = *var_ttemp_dn9_slot;
        let mut var_uc_ibpc1: f64 = *var_uc_ibpc1_slot;
        let mut var_uc_subld1: f64 = *var_uc_subld1_slot;
        let mut var_vg2const: f64 = *var_vg2const_slot;
        let mut var_vg2const_1: f64 = *var_vg2const_1_slot;
        let mut var_vg2const_1_dn0: f64 = *var_vg2const_1_dn0_slot;
        let mut var_vg2const_1_dn10: f64 = *var_vg2const_1_dn10_slot;
        let mut var_vg2const_1_dn11: f64 = *var_vg2const_1_dn11_slot;
        let mut var_vg2const_1_dn14: f64 = *var_vg2const_1_dn14_slot;
        let mut var_vg2const_1_dn2: f64 = *var_vg2const_1_dn2_slot;
        let mut var_vg2const_1_dn4: f64 = *var_vg2const_1_dn4_slot;
        let mut var_vg2const_1_dn5: f64 = *var_vg2const_1_dn5_slot;
        let mut var_vg2const_1_dn6: f64 = *var_vg2const_1_dn6_slot;
        let mut var_vg2const_1_dn7: f64 = *var_vg2const_1_dn7_slot;
        let mut var_vg2const_1_dn8: f64 = *var_vg2const_1_dn8_slot;
        let mut var_vg2const_1_dn9: f64 = *var_vg2const_1_dn9_slot;
        let mut var_vg2const_dn0: f64 = *var_vg2const_dn0_slot;
        let mut var_vg2const_dn10: f64 = *var_vg2const_dn10_slot;
        let mut var_vg2const_dn11: f64 = *var_vg2const_dn11_slot;
        let mut var_vg2const_dn14: f64 = *var_vg2const_dn14_slot;
        let mut var_vg2const_dn2: f64 = *var_vg2const_dn2_slot;
        let mut var_vg2const_dn4: f64 = *var_vg2const_dn4_slot;
        let mut var_vg2const_dn5: f64 = *var_vg2const_dn5_slot;
        let mut var_vg2const_dn6: f64 = *var_vg2const_dn6_slot;
        let mut var_vg2const_dn7: f64 = *var_vg2const_dn7_slot;
        let mut var_vg2const_dn8: f64 = *var_vg2const_dn8_slot;
        let mut var_vg2const_dn9: f64 = *var_vg2const_dn9_slot;
        let mut var_xgate: f64 = *var_xgate_slot;
        let mut var_xgate_1: f64 = *var_xgate_1_slot;
        let mut var_xsub1: f64 = *var_xsub1_slot;
        let mut var_xsub1_1: f64 = *var_xsub1_1_slot;
        let mut var_xsub2: f64 = *var_xsub2_slot;
        let mut var_xsub2_1: f64 = *var_xsub2_1_slot;
        let mut var_xvbs: f64 = *var_xvbs_slot;
        let mut var_xvbs_1: f64 = *var_xvbs_1_slot;

        let (assign12690_e7094,) = {
    if (p.p23 != 0.0) {
        (var_xsub1,)
    } else {
        (var_xsub1_1,)
    }
};
        var_xsub1_1 = assign12690_e7094;

        let (assign12700_e7098,) = {
    if (p.p23 != 0.0) {
        (var_xsub2,)
    } else {
        (var_xsub2_1,)
    }
};
        var_xsub2_1 = assign12700_e7098;

        let (assign12710_e7102, assign12710_e7102_d_n0, assign12710_e7102_d_n2, assign12710_e7102_d_n4, assign12710_e7102_d_n5, assign12710_e7102_d_n6, assign12710_e7102_d_n7, assign12710_e7102_d_n8, assign12710_e7102_d_n9, assign12710_e7102_d_n10, assign12710_e7102_d_n11, assign12710_e7102_d_n14,) = {
    if (p.p23 != 0.0) {
        (var_vg2const, var_vg2const_dn0, var_vg2const_dn2, var_vg2const_dn4, var_vg2const_dn5, var_vg2const_dn6, var_vg2const_dn7, var_vg2const_dn8, var_vg2const_dn9, var_vg2const_dn10, var_vg2const_dn11, var_vg2const_dn14,)
    } else {
        (var_vg2const_1, var_vg2const_1_dn0, var_vg2const_1_dn2, var_vg2const_1_dn4, var_vg2const_1_dn5, var_vg2const_1_dn6, var_vg2const_1_dn7, var_vg2const_1_dn8, var_vg2const_1_dn9, var_vg2const_1_dn10, var_vg2const_1_dn11, var_vg2const_1_dn14,)
    }
};
        var_vg2const_1 = assign12710_e7102;
        var_vg2const_1_dn0 = assign12710_e7102_d_n0;
        var_vg2const_1_dn2 = assign12710_e7102_d_n2;
        var_vg2const_1_dn4 = assign12710_e7102_d_n4;
        var_vg2const_1_dn5 = assign12710_e7102_d_n5;
        var_vg2const_1_dn6 = assign12710_e7102_d_n6;
        var_vg2const_1_dn7 = assign12710_e7102_d_n7;
        var_vg2const_1_dn8 = assign12710_e7102_d_n8;
        var_vg2const_1_dn9 = assign12710_e7102_d_n9;
        var_vg2const_1_dn10 = assign12710_e7102_d_n10;
        var_vg2const_1_dn11 = assign12710_e7102_d_n11;
        var_vg2const_1_dn14 = assign12710_e7102_d_n14;

        let (assign12720_e7106,) = {
    if (p.p23 != 0.0) {
        (var_xvbs,)
    } else {
        (var_xvbs_1,)
    }
};
        var_xvbs_1 = assign12720_e7106;

        let (assign12730_e7110,) = {
    if (p.p23 != 0.0) {
        (var_xgate,)
    } else {
        (var_xgate_1,)
    }
};
        var_xgate_1 = assign12730_e7110;

        let (assign12740_e7124,) = {
    if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
        let assign12740_e7119: f64 = (var_lgate).powf(p.p191);
        let assign12740_e7120: f64 = (var_mks_sub1l / assign12740_e7119);
        let assign12740_e7121: f64 = (1.0 + assign12740_e7120);
        let assign12740_e7122: f64 = (var_uc_sub1snp * assign12740_e7121);
        (assign12740_e7122,)
    } else {
        (var_xsub1_1,)
    }
};
        var_xsub1_1 = assign12740_e7124;

        let (assign12750_e7136,) = {
    if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
        let assign12750_e7132: f64 = (var_mks_sub2l / var_lgate);
        let assign12750_e7133: f64 = (1.0 + assign12750_e7132);
        let assign12750_e7134: f64 = (var_uc_sub2snp * assign12750_e7133);
        (assign12750_e7134,)
    } else {
        (var_xsub2_1,)
    }
};
        var_xsub2_1 = assign12750_e7136;

        let (assign12760_e7148,) = {
    if (p.p23 != 0.0) {
        let assign12760_e7143: f64 = (var_lg).powf(p.p103);
        let assign12760_e7144: f64 = (p.p102 / assign12760_e7143);
        let assign12760_e7145: f64 = (1.0 + assign12760_e7144);
        let assign12760_e7146: f64 = (p.p72 * assign12760_e7145);
        (assign12760_e7146,)
    } else {
        (var_uc_subld1,)
    }
};
        var_uc_subld1 = assign12760_e7148;

        let (assign12770_e7153, assign12770_e7153_d_n0, assign12770_e7153_d_n2, assign12770_e7153_d_n4, assign12770_e7153_d_n5, assign12770_e7153_d_n6, assign12770_e7153_d_n7, assign12770_e7153_d_n8, assign12770_e7153_d_n9, assign12770_e7153_d_n10, assign12770_e7153_d_n11, assign12770_e7153_d_n14,) = {
    if (p.p23 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vg2const, var_vg2const_dn0, var_vg2const_dn2, var_vg2const_dn4, var_vg2const_dn5, var_vg2const_dn6, var_vg2const_dn7, var_vg2const_dn8, var_vg2const_dn9, var_vg2const_dn10, var_vg2const_dn11, var_vg2const_dn14,)
    }
};
        var_vg2const = assign12770_e7153;
        var_vg2const_dn0 = assign12770_e7153_d_n0;
        var_vg2const_dn2 = assign12770_e7153_d_n2;
        var_vg2const_dn4 = assign12770_e7153_d_n4;
        var_vg2const_dn5 = assign12770_e7153_d_n5;
        var_vg2const_dn6 = assign12770_e7153_d_n6;
        var_vg2const_dn7 = assign12770_e7153_d_n7;
        var_vg2const_dn8 = assign12770_e7153_d_n8;
        var_vg2const_dn9 = assign12770_e7153_d_n9;
        var_vg2const_dn10 = assign12770_e7153_d_n10;
        var_vg2const_dn11 = assign12770_e7153_d_n11;
        var_vg2const_dn14 = assign12770_e7153_d_n14;

        let (assign12780_e7158,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xvbs,)
    }
};
        var_xvbs = assign12780_e7158;

        let (assign12790_e7163,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xgate,)
    }
};
        var_xgate = assign12790_e7163;

        let (assign12800_e7168,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xsub1,)
    }
};
        var_xsub1 = assign12800_e7168;

        let (assign12810_e7173,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xsub2,)
    }
};
        var_xsub2 = assign12810_e7173;

        let (assign12820_e7178,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_uc_subld1,)
    }
};
        var_uc_subld1 = assign12820_e7178;

        let (assign12830_e7183, assign12830_e7183_d_n0, assign12830_e7183_d_n2, assign12830_e7183_d_n4, assign12830_e7183_d_n5, assign12830_e7183_d_n6, assign12830_e7183_d_n7, assign12830_e7183_d_n8, assign12830_e7183_d_n9, assign12830_e7183_d_n10, assign12830_e7183_d_n11, assign12830_e7183_d_n14,) = {
    if (p.p23 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vg2const_1, var_vg2const_1_dn0, var_vg2const_1_dn2, var_vg2const_1_dn4, var_vg2const_1_dn5, var_vg2const_1_dn6, var_vg2const_1_dn7, var_vg2const_1_dn8, var_vg2const_1_dn9, var_vg2const_1_dn10, var_vg2const_1_dn11, var_vg2const_1_dn14,)
    }
};
        var_vg2const_1 = assign12830_e7183;
        var_vg2const_1_dn0 = assign12830_e7183_d_n0;
        var_vg2const_1_dn2 = assign12830_e7183_d_n2;
        var_vg2const_1_dn4 = assign12830_e7183_d_n4;
        var_vg2const_1_dn5 = assign12830_e7183_d_n5;
        var_vg2const_1_dn6 = assign12830_e7183_d_n6;
        var_vg2const_1_dn7 = assign12830_e7183_d_n7;
        var_vg2const_1_dn8 = assign12830_e7183_d_n8;
        var_vg2const_1_dn9 = assign12830_e7183_d_n9;
        var_vg2const_1_dn10 = assign12830_e7183_d_n10;
        var_vg2const_1_dn11 = assign12830_e7183_d_n11;
        var_vg2const_1_dn14 = assign12830_e7183_d_n14;

        let (assign12840_e7188,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xvbs_1,)
    }
};
        var_xvbs_1 = assign12840_e7188;

        let (assign12850_e7193,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xgate_1,)
    }
};
        var_xgate_1 = assign12850_e7193;

        let (assign12860_e7198,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xsub1_1,)
    }
};
        var_xsub1_1 = assign12860_e7198;

        let (assign12870_e7203,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xsub2_1,)
    }
};
        var_xsub2_1 = assign12870_e7203;

        let (assign12880_e7217,) = {
    if (var_uc_ibpc1 != 0.0) {
        let assign12880_e7212: f64 = (var_lg).powf(p.p280);
        let assign12880_e7213: f64 = (p.p279 / assign12880_e7212);
        let assign12880_e7214: f64 = (1.0 + assign12880_e7213);
        let assign12880_e7215: f64 = (var_uc_ibpc1 * assign12880_e7214);
        (assign12880_e7215,)
    } else {
        (0.0,)
    }
};
        var_uc_ibpc1 = assign12880_e7217;

        let assign12890_e7221: f64 = (3.141592653589793 / 2.0);
        let assign12890_e7222: f64 = (3.453133e-11 / assign12890_e7221);
        let assign12890_e7224: f64 = (assign12890_e7222 * var_weffcv_nf);
        let assign12890_e7228: f64 = (p.p225 / p.p95);
        let assign12890_e7229: f64 = (1.0 + assign12890_e7228);
        let assign12890_e7230: f64 = (assign12890_e7229).ln();
        let assign12890_e7231: f64 = (assign12890_e7224 * assign12890_e7230);
        var_cfrng = assign12890_e7231;

        let (assign12900_e7245,) = {
    if (p.p134 != 0.0) {
        let assign12900_e7237: f64 = (1000000.0 * var_weffcv_nf);
        let assign12900_e7239: f64 = (assign12900_e7237 * p.p134);
        let assign12900_e7242: f64 = (var_lg).powf(p.p135);
        let assign12900_e7243: f64 = (assign12900_e7239 / assign12900_e7242);
        (assign12900_e7243,)
    } else {
        (0.0,)
    }
};
        var_cqyb0 = assign12900_e7245;

        let assign12910_e7249: f64 = (-p.p286);
        let assign12910_e7250: f64 = (var_lg).powf(assign12910_e7249);
        let assign12910_e7251: f64 = (p.p283 * assign12910_e7250);
        var_ptl0 = assign12910_e7251;

        let assign12920_e7255: f64 = (-p.p291);
        let assign12920_e7256: f64 = (var_lg).powf(assign12920_e7255);
        let assign12920_e7257: f64 = (p.p290 * assign12920_e7256);
        var_pt40 = assign12920_e7257;

        let assign12930_e7261: f64 = (var_lg + var_uc_gdld);
        let assign12930_e7263: f64 = (-p.p288);
        let assign12930_e7264: f64 = (assign12930_e7261).powf(assign12930_e7263);
        let assign12930_e7265: f64 = (p.p287 * assign12930_e7264);
        var_gdl0 = assign12930_e7265;

        let assign12940_e7269: f64 = (var_mfactor * var_weff_nf);
        let assign12940_e7270: f64 = (var_uc_rth0 / assign12940_e7269);
        let assign12940_e7275: f64 = (var_lg).powf(p.p318);
        let assign12940_e7276: f64 = (p.p317 / assign12940_e7275);
        let assign12940_e7277: f64 = (1.0 + assign12940_e7276);
        let assign12940_e7278: f64 = (assign12940_e7270 * assign12940_e7277);
        let assign12940_e7283: f64 = (var_wg).powf(p.p316);
        let assign12940_e7284: f64 = (p.p315 / assign12940_e7283);
        let assign12940_e7285: f64 = (1.0 + assign12940_e7284);
        let assign12940_e7286: f64 = (assign12940_e7278 * assign12940_e7285);
        var_rth = assign12940_e7286;
        var_rth_dn0 = 0.0;
        var_rth_dn2 = 0.0;
        var_rth_dn4 = 0.0;
        var_rth_dn5 = 0.0;
        var_rth_dn6 = 0.0;
        var_rth_dn7 = 0.0;
        var_rth_dn8 = 0.0;
        var_rth_dn9 = 0.0;
        var_rth_dn10 = 0.0;
        var_rth_dn11 = 0.0;
        var_rth_dn14 = 0.0;

        let assign12960_e7296: f64 = (p.p7).powf(p.p327);
        let assign12960_e7297: f64 = (1.0 / assign12960_e7296);
        let assign12960_e7298: f64 = (var_rth * assign12960_e7297);
        var_rth = assign12960_e7298;
        var_rth_dn0 = (var_rth_dn0 * assign12960_e7297);
        var_rth_dn2 = (var_rth_dn2 * assign12960_e7297);
        var_rth_dn4 = (var_rth_dn4 * assign12960_e7297);
        var_rth_dn5 = (var_rth_dn5 * assign12960_e7297);
        var_rth_dn6 = (var_rth_dn6 * assign12960_e7297);
        var_rth_dn7 = (var_rth_dn7 * assign12960_e7297);
        var_rth_dn8 = (var_rth_dn8 * assign12960_e7297);
        var_rth_dn9 = (var_rth_dn9 * assign12960_e7297);
        var_rth_dn10 = (var_rth_dn10 * assign12960_e7297);
        var_rth_dn11 = (var_rth_dn11 * assign12960_e7297);
        var_rth_dn14 = (var_rth_dn14 * assign12960_e7297);

        let assign12970_e7302: f64 = (p.p7).powf(p.p327);
        let assign12970_e7303: f64 = (1.0 / assign12970_e7302);
        let assign12970_e7306: f64 = (var_mfactor * var_weff_nf);
        let assign12970_e7307: f64 = (assign12970_e7303 / assign12970_e7306);
        let assign12970_e7312: f64 = (var_lg).powf(p.p318);
        let assign12970_e7313: f64 = (p.p317 / assign12970_e7312);
        let assign12970_e7314: f64 = (1.0 + assign12970_e7313);
        let assign12970_e7315: f64 = (assign12970_e7307 * assign12970_e7314);
        let assign12970_e7320: f64 = (var_wg).powf(p.p316);
        let assign12970_e7321: f64 = (p.p315 / assign12970_e7320);
        let assign12970_e7322: f64 = (1.0 + assign12970_e7321);
        let assign12970_e7323: f64 = (assign12970_e7315 * assign12970_e7322);
        var_rthtemp0 = assign12970_e7323;

        let assign12980_e7330: f64 = if ((p.p53 == 0.0) || (var_uc_rth0 == 0.0)) { 1.0 } else { 0.0 };
        var_guard293 = assign12980_e7330;

        let (assign12990_e7334, assign12990_e7334_d_n0, assign12990_e7334_d_n2, assign12990_e7334_d_n4, assign12990_e7334_d_n5, assign12990_e7334_d_n6, assign12990_e7334_d_n7, assign12990_e7334_d_n8, assign12990_e7334_d_n9, assign12990_e7334_d_n10, assign12990_e7334_d_n11, assign12990_e7334_d_n14,) = {
    if (var_guard293 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cnst0over, var_cnst0over_dn0, var_cnst0over_dn2, var_cnst0over_dn4, var_cnst0over_dn5, var_cnst0over_dn6, var_cnst0over_dn7, var_cnst0over_dn8, var_cnst0over_dn9, var_cnst0over_dn10, var_cnst0over_dn11, var_cnst0over_dn14,)
    }
};
        var_cnst0over = assign12990_e7334;
        var_cnst0over_dn0 = assign12990_e7334_d_n0;
        var_cnst0over_dn2 = assign12990_e7334_d_n2;
        var_cnst0over_dn4 = assign12990_e7334_d_n4;
        var_cnst0over_dn5 = assign12990_e7334_d_n5;
        var_cnst0over_dn6 = assign12990_e7334_d_n6;
        var_cnst0over_dn7 = assign12990_e7334_d_n7;
        var_cnst0over_dn8 = assign12990_e7334_d_n8;
        var_cnst0over_dn9 = assign12990_e7334_d_n9;
        var_cnst0over_dn10 = assign12990_e7334_d_n10;
        var_cnst0over_dn11 = assign12990_e7334_d_n11;
        var_cnst0over_dn14 = assign12990_e7334_d_n14;

        let (assign13000_e7338, assign13000_e7338_d_n0, assign13000_e7338_d_n2, assign13000_e7338_d_n4, assign13000_e7338_d_n5, assign13000_e7338_d_n6, assign13000_e7338_d_n7, assign13000_e7338_d_n8, assign13000_e7338_d_n9, assign13000_e7338_d_n10, assign13000_e7338_d_n11, assign13000_e7338_d_n14,) = {
    if (var_guard293 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cnst0overs, var_cnst0overs_dn0, var_cnst0overs_dn2, var_cnst0overs_dn4, var_cnst0overs_dn5, var_cnst0overs_dn6, var_cnst0overs_dn7, var_cnst0overs_dn8, var_cnst0overs_dn9, var_cnst0overs_dn10, var_cnst0overs_dn11, var_cnst0overs_dn14,)
    }
};
        var_cnst0overs = assign13000_e7338;
        var_cnst0overs_dn0 = assign13000_e7338_d_n0;
        var_cnst0overs_dn2 = assign13000_e7338_d_n2;
        var_cnst0overs_dn4 = assign13000_e7338_d_n4;
        var_cnst0overs_dn5 = assign13000_e7338_d_n5;
        var_cnst0overs_dn6 = assign13000_e7338_d_n6;
        var_cnst0overs_dn7 = assign13000_e7338_d_n7;
        var_cnst0overs_dn8 = assign13000_e7338_d_n8;
        var_cnst0overs_dn9 = assign13000_e7338_d_n9;
        var_cnst0overs_dn10 = assign13000_e7338_d_n10;
        var_cnst0overs_dn11 = assign13000_e7338_d_n11;
        var_cnst0overs_dn14 = assign13000_e7338_d_n14;

        let (assign13010_e7344, assign13010_e7344_d_n0, assign13010_e7344_d_n2, assign13010_e7344_d_n4, assign13010_e7344_d_n5, assign13010_e7344_d_n6, assign13010_e7344_d_n7, assign13010_e7344_d_n8, assign13010_e7344_d_n9, assign13010_e7344_d_n10, assign13010_e7344_d_n11, assign13010_e7344_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13010_e7340: f64 = ctx_temp;
        let assign13010_e7342: f64 = (assign13010_e7340 + p.p11);
        (assign13010_e7342, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ttemp, var_ttemp_dn0, var_ttemp_dn2, var_ttemp_dn4, var_ttemp_dn5, var_ttemp_dn6, var_ttemp_dn7, var_ttemp_dn8, var_ttemp_dn9, var_ttemp_dn10, var_ttemp_dn11, var_ttemp_dn14,)
    }
};
        var_ttemp = assign13010_e7344;
        var_ttemp_dn0 = assign13010_e7344_d_n0;
        var_ttemp_dn2 = assign13010_e7344_d_n2;
        var_ttemp_dn4 = assign13010_e7344_d_n4;
        var_ttemp_dn5 = assign13010_e7344_d_n5;
        var_ttemp_dn6 = assign13010_e7344_d_n6;
        var_ttemp_dn7 = assign13010_e7344_d_n7;
        var_ttemp_dn8 = assign13010_e7344_d_n8;
        var_ttemp_dn9 = assign13010_e7344_d_n9;
        var_ttemp_dn10 = assign13010_e7344_d_n10;
        var_ttemp_dn11 = assign13010_e7344_d_n11;
        var_ttemp_dn14 = assign13010_e7344_d_n14;

        let (assign13020_e7348, assign13020_e7348_d_n0, assign13020_e7348_d_n2, assign13020_e7348_d_n4, assign13020_e7348_d_n5, assign13020_e7348_d_n6, assign13020_e7348_d_n7, assign13020_e7348_d_n8, assign13020_e7348_d_n9, assign13020_e7348_d_n10, assign13020_e7348_d_n11, assign13020_e7348_d_n14,) = {
    if (var_guard293 != 0.0) {
        (var_ttemp, var_ttemp_dn0, var_ttemp_dn2, var_ttemp_dn4, var_ttemp_dn5, var_ttemp_dn6, var_ttemp_dn7, var_ttemp_dn8, var_ttemp_dn9, var_ttemp_dn10, var_ttemp_dn11, var_ttemp_dn14,)
    } else {
        (var_ttemp0, var_ttemp0_dn0, var_ttemp0_dn2, var_ttemp0_dn4, var_ttemp0_dn5, var_ttemp0_dn6, var_ttemp0_dn7, var_ttemp0_dn8, var_ttemp0_dn9, var_ttemp0_dn10, var_ttemp0_dn11, var_ttemp0_dn14,)
    }
};
        var_ttemp0 = assign13020_e7348;
        var_ttemp0_dn0 = assign13020_e7348_d_n0;
        var_ttemp0_dn2 = assign13020_e7348_d_n2;
        var_ttemp0_dn4 = assign13020_e7348_d_n4;
        var_ttemp0_dn5 = assign13020_e7348_d_n5;
        var_ttemp0_dn6 = assign13020_e7348_d_n6;
        var_ttemp0_dn7 = assign13020_e7348_d_n7;
        var_ttemp0_dn8 = assign13020_e7348_d_n8;
        var_ttemp0_dn9 = assign13020_e7348_d_n9;
        var_ttemp0_dn10 = assign13020_e7348_d_n10;
        var_ttemp0_dn11 = assign13020_e7348_d_n11;
        var_ttemp0_dn14 = assign13020_e7348_d_n14;

        let (assign13030_e7354, assign13030_e7354_d_n0, assign13030_e7354_d_n2, assign13030_e7354_d_n4, assign13030_e7354_d_n5, assign13030_e7354_d_n6, assign13030_e7354_d_n7, assign13030_e7354_d_n8, assign13030_e7354_d_n9, assign13030_e7354_d_n10, assign13030_e7354_d_n11, assign13030_e7354_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13030_e7352: f64 = (var_ttemp + var_deltemp);
        (assign13030_e7352, (var_ttemp_dn0 + var_deltemp_dn0), (var_ttemp_dn2 + var_deltemp_dn2), (var_ttemp_dn4 + var_deltemp_dn4), (var_ttemp_dn5 + var_deltemp_dn5), (var_ttemp_dn6 + var_deltemp_dn6), (var_ttemp_dn7 + var_deltemp_dn7), (var_ttemp_dn8 + var_deltemp_dn8), (var_ttemp_dn9 + var_deltemp_dn9), (var_ttemp_dn10 + var_deltemp_dn10), (var_ttemp_dn11 + var_deltemp_dn11), (var_ttemp_dn14 + var_deltemp_dn14),)
    } else {
        (var_ttemp, var_ttemp_dn0, var_ttemp_dn2, var_ttemp_dn4, var_ttemp_dn5, var_ttemp_dn6, var_ttemp_dn7, var_ttemp_dn8, var_ttemp_dn9, var_ttemp_dn10, var_ttemp_dn11, var_ttemp_dn14,)
    }
};
        var_ttemp = assign13030_e7354;
        var_ttemp_dn0 = assign13030_e7354_d_n0;
        var_ttemp_dn2 = assign13030_e7354_d_n2;
        var_ttemp_dn4 = assign13030_e7354_d_n4;
        var_ttemp_dn5 = assign13030_e7354_d_n5;
        var_ttemp_dn6 = assign13030_e7354_d_n6;
        var_ttemp_dn7 = assign13030_e7354_d_n7;
        var_ttemp_dn8 = assign13030_e7354_d_n8;
        var_ttemp_dn9 = assign13030_e7354_d_n9;
        var_ttemp_dn10 = assign13030_e7354_d_n10;
        var_ttemp_dn11 = assign13030_e7354_d_n11;
        var_ttemp_dn14 = assign13030_e7354_d_n14;

        let (assign13040_e7360, assign13040_e7360_d_n0, assign13040_e7360_d_n2, assign13040_e7360_d_n4, assign13040_e7360_d_n5, assign13040_e7360_d_n6, assign13040_e7360_d_n7, assign13040_e7360_d_n8, assign13040_e7360_d_n9, assign13040_e7360_d_n10, assign13040_e7360_d_n11, assign13040_e7360_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13040_e7358: f64 = (var_ttemp0 - var_ktnom);
        (assign13040_e7358, var_ttemp0_dn0, var_ttemp0_dn2, var_ttemp0_dn4, var_ttemp0_dn5, var_ttemp0_dn6, var_ttemp0_dn7, var_ttemp0_dn8, var_ttemp0_dn9, var_ttemp0_dn10, var_ttemp0_dn11, var_ttemp0_dn14,)
    } else {
        (var_tdiff0, var_tdiff0_dn0, var_tdiff0_dn2, var_tdiff0_dn4, var_tdiff0_dn5, var_tdiff0_dn6, var_tdiff0_dn7, var_tdiff0_dn8, var_tdiff0_dn9, var_tdiff0_dn10, var_tdiff0_dn11, var_tdiff0_dn14,)
    }
};
        var_tdiff0 = assign13040_e7360;
        var_tdiff0_dn0 = assign13040_e7360_d_n0;
        var_tdiff0_dn2 = assign13040_e7360_d_n2;
        var_tdiff0_dn4 = assign13040_e7360_d_n4;
        var_tdiff0_dn5 = assign13040_e7360_d_n5;
        var_tdiff0_dn6 = assign13040_e7360_d_n6;
        var_tdiff0_dn7 = assign13040_e7360_d_n7;
        var_tdiff0_dn8 = assign13040_e7360_d_n8;
        var_tdiff0_dn9 = assign13040_e7360_d_n9;
        var_tdiff0_dn10 = assign13040_e7360_d_n10;
        var_tdiff0_dn11 = assign13040_e7360_d_n11;
        var_tdiff0_dn14 = assign13040_e7360_d_n14;

        let (assign13050_e7370, assign13050_e7370_d_n0, assign13050_e7370_d_n2, assign13050_e7370_d_n4, assign13050_e7370_d_n5, assign13050_e7370_d_n6, assign13050_e7370_d_n7, assign13050_e7370_d_n8, assign13050_e7370_d_n9, assign13050_e7370_d_n10, assign13050_e7370_d_n11, assign13050_e7370_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13050_e7364: f64 = (var_ttemp0 * var_ttemp0);
        let assign13050_e7367: f64 = (var_ktnom * var_ktnom);
        let assign13050_e7368: f64 = (assign13050_e7364 - assign13050_e7367);
        (assign13050_e7368, ((var_ttemp0_dn0 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn0)), ((var_ttemp0_dn2 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn2)), ((var_ttemp0_dn4 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn4)), ((var_ttemp0_dn5 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn5)), ((var_ttemp0_dn6 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn6)), ((var_ttemp0_dn7 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn7)), ((var_ttemp0_dn8 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn8)), ((var_ttemp0_dn9 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn9)), ((var_ttemp0_dn10 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn10)), ((var_ttemp0_dn11 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn11)), ((var_ttemp0_dn14 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn14)),)
    } else {
        (var_tdiff0_2, var_tdiff0_2_dn0, var_tdiff0_2_dn2, var_tdiff0_2_dn4, var_tdiff0_2_dn5, var_tdiff0_2_dn6, var_tdiff0_2_dn7, var_tdiff0_2_dn8, var_tdiff0_2_dn9, var_tdiff0_2_dn10, var_tdiff0_2_dn11, var_tdiff0_2_dn14,)
    }
};
        var_tdiff0_2 = assign13050_e7370;
        var_tdiff0_2_dn0 = assign13050_e7370_d_n0;
        var_tdiff0_2_dn2 = assign13050_e7370_d_n2;
        var_tdiff0_2_dn4 = assign13050_e7370_d_n4;
        var_tdiff0_2_dn5 = assign13050_e7370_d_n5;
        var_tdiff0_2_dn6 = assign13050_e7370_d_n6;
        var_tdiff0_2_dn7 = assign13050_e7370_d_n7;
        var_tdiff0_2_dn8 = assign13050_e7370_d_n8;
        var_tdiff0_2_dn9 = assign13050_e7370_d_n9;
        var_tdiff0_2_dn10 = assign13050_e7370_d_n10;
        var_tdiff0_2_dn11 = assign13050_e7370_d_n11;
        var_tdiff0_2_dn14 = assign13050_e7370_d_n14;

        let (assign13060_e7376, assign13060_e7376_d_n0, assign13060_e7376_d_n2, assign13060_e7376_d_n4, assign13060_e7376_d_n5, assign13060_e7376_d_n6, assign13060_e7376_d_n7, assign13060_e7376_d_n8, assign13060_e7376_d_n9, assign13060_e7376_d_n10, assign13060_e7376_d_n11, assign13060_e7376_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13060_e7374: f64 = (var_ttemp - var_ktnom);
        (assign13060_e7374, var_ttemp_dn0, var_ttemp_dn2, var_ttemp_dn4, var_ttemp_dn5, var_ttemp_dn6, var_ttemp_dn7, var_ttemp_dn8, var_ttemp_dn9, var_ttemp_dn10, var_ttemp_dn11, var_ttemp_dn14,)
    } else {
        (var_tdiff, var_tdiff_dn0, var_tdiff_dn2, var_tdiff_dn4, var_tdiff_dn5, var_tdiff_dn6, var_tdiff_dn7, var_tdiff_dn8, var_tdiff_dn9, var_tdiff_dn10, var_tdiff_dn11, var_tdiff_dn14,)
    }
};
        var_tdiff = assign13060_e7376;
        var_tdiff_dn0 = assign13060_e7376_d_n0;
        var_tdiff_dn2 = assign13060_e7376_d_n2;
        var_tdiff_dn4 = assign13060_e7376_d_n4;
        var_tdiff_dn5 = assign13060_e7376_d_n5;
        var_tdiff_dn6 = assign13060_e7376_d_n6;
        var_tdiff_dn7 = assign13060_e7376_d_n7;
        var_tdiff_dn8 = assign13060_e7376_d_n8;
        var_tdiff_dn9 = assign13060_e7376_d_n9;
        var_tdiff_dn10 = assign13060_e7376_d_n10;
        var_tdiff_dn11 = assign13060_e7376_d_n11;
        var_tdiff_dn14 = assign13060_e7376_d_n14;

        *var_cfrng_slot = var_cfrng;
        *var_cnst0over_slot = var_cnst0over;
        *var_cnst0over_dn0_slot = var_cnst0over_dn0;
        *var_cnst0over_dn10_slot = var_cnst0over_dn10;
        *var_cnst0over_dn11_slot = var_cnst0over_dn11;
        *var_cnst0over_dn14_slot = var_cnst0over_dn14;
        *var_cnst0over_dn2_slot = var_cnst0over_dn2;
        *var_cnst0over_dn4_slot = var_cnst0over_dn4;
        *var_cnst0over_dn5_slot = var_cnst0over_dn5;
        *var_cnst0over_dn6_slot = var_cnst0over_dn6;
        *var_cnst0over_dn7_slot = var_cnst0over_dn7;
        *var_cnst0over_dn8_slot = var_cnst0over_dn8;
        *var_cnst0over_dn9_slot = var_cnst0over_dn9;
        *var_cnst0overs_slot = var_cnst0overs;
        *var_cnst0overs_dn0_slot = var_cnst0overs_dn0;
        *var_cnst0overs_dn10_slot = var_cnst0overs_dn10;
        *var_cnst0overs_dn11_slot = var_cnst0overs_dn11;
        *var_cnst0overs_dn14_slot = var_cnst0overs_dn14;
        *var_cnst0overs_dn2_slot = var_cnst0overs_dn2;
        *var_cnst0overs_dn4_slot = var_cnst0overs_dn4;
        *var_cnst0overs_dn5_slot = var_cnst0overs_dn5;
        *var_cnst0overs_dn6_slot = var_cnst0overs_dn6;
        *var_cnst0overs_dn7_slot = var_cnst0overs_dn7;
        *var_cnst0overs_dn8_slot = var_cnst0overs_dn8;
        *var_cnst0overs_dn9_slot = var_cnst0overs_dn9;
        *var_cqyb0_slot = var_cqyb0;
        *var_gdl0_slot = var_gdl0;
        *var_guard293_slot = var_guard293;
        *var_pt40_slot = var_pt40;
        *var_ptl0_slot = var_ptl0;
        *var_rth_slot = var_rth;
        *var_rth_dn0_slot = var_rth_dn0;
        *var_rth_dn10_slot = var_rth_dn10;
        *var_rth_dn11_slot = var_rth_dn11;
        *var_rth_dn14_slot = var_rth_dn14;
        *var_rth_dn2_slot = var_rth_dn2;
        *var_rth_dn4_slot = var_rth_dn4;
        *var_rth_dn5_slot = var_rth_dn5;
        *var_rth_dn6_slot = var_rth_dn6;
        *var_rth_dn7_slot = var_rth_dn7;
        *var_rth_dn8_slot = var_rth_dn8;
        *var_rth_dn9_slot = var_rth_dn9;
        *var_rthtemp0_slot = var_rthtemp0;
        *var_tdiff_slot = var_tdiff;
        *var_tdiff0_slot = var_tdiff0;
        *var_tdiff0_2_slot = var_tdiff0_2;
        *var_tdiff0_2_dn0_slot = var_tdiff0_2_dn0;
        *var_tdiff0_2_dn10_slot = var_tdiff0_2_dn10;
        *var_tdiff0_2_dn11_slot = var_tdiff0_2_dn11;
        *var_tdiff0_2_dn14_slot = var_tdiff0_2_dn14;
        *var_tdiff0_2_dn2_slot = var_tdiff0_2_dn2;
        *var_tdiff0_2_dn4_slot = var_tdiff0_2_dn4;
        *var_tdiff0_2_dn5_slot = var_tdiff0_2_dn5;
        *var_tdiff0_2_dn6_slot = var_tdiff0_2_dn6;
        *var_tdiff0_2_dn7_slot = var_tdiff0_2_dn7;
        *var_tdiff0_2_dn8_slot = var_tdiff0_2_dn8;
        *var_tdiff0_2_dn9_slot = var_tdiff0_2_dn9;
        *var_tdiff0_dn0_slot = var_tdiff0_dn0;
        *var_tdiff0_dn10_slot = var_tdiff0_dn10;
        *var_tdiff0_dn11_slot = var_tdiff0_dn11;
        *var_tdiff0_dn14_slot = var_tdiff0_dn14;
        *var_tdiff0_dn2_slot = var_tdiff0_dn2;
        *var_tdiff0_dn4_slot = var_tdiff0_dn4;
        *var_tdiff0_dn5_slot = var_tdiff0_dn5;
        *var_tdiff0_dn6_slot = var_tdiff0_dn6;
        *var_tdiff0_dn7_slot = var_tdiff0_dn7;
        *var_tdiff0_dn8_slot = var_tdiff0_dn8;
        *var_tdiff0_dn9_slot = var_tdiff0_dn9;
        *var_tdiff_dn0_slot = var_tdiff_dn0;
        *var_tdiff_dn10_slot = var_tdiff_dn10;
        *var_tdiff_dn11_slot = var_tdiff_dn11;
        *var_tdiff_dn14_slot = var_tdiff_dn14;
        *var_tdiff_dn2_slot = var_tdiff_dn2;
        *var_tdiff_dn4_slot = var_tdiff_dn4;
        *var_tdiff_dn5_slot = var_tdiff_dn5;
        *var_tdiff_dn6_slot = var_tdiff_dn6;
        *var_tdiff_dn7_slot = var_tdiff_dn7;
        *var_tdiff_dn8_slot = var_tdiff_dn8;
        *var_tdiff_dn9_slot = var_tdiff_dn9;
        *var_ttemp_slot = var_ttemp;
        *var_ttemp0_slot = var_ttemp0;
        *var_ttemp0_dn0_slot = var_ttemp0_dn0;
        *var_ttemp0_dn10_slot = var_ttemp0_dn10;
        *var_ttemp0_dn11_slot = var_ttemp0_dn11;
        *var_ttemp0_dn14_slot = var_ttemp0_dn14;
        *var_ttemp0_dn2_slot = var_ttemp0_dn2;
        *var_ttemp0_dn4_slot = var_ttemp0_dn4;
        *var_ttemp0_dn5_slot = var_ttemp0_dn5;
        *var_ttemp0_dn6_slot = var_ttemp0_dn6;
        *var_ttemp0_dn7_slot = var_ttemp0_dn7;
        *var_ttemp0_dn8_slot = var_ttemp0_dn8;
        *var_ttemp0_dn9_slot = var_ttemp0_dn9;
        *var_ttemp_dn0_slot = var_ttemp_dn0;
        *var_ttemp_dn10_slot = var_ttemp_dn10;
        *var_ttemp_dn11_slot = var_ttemp_dn11;
        *var_ttemp_dn14_slot = var_ttemp_dn14;
        *var_ttemp_dn2_slot = var_ttemp_dn2;
        *var_ttemp_dn4_slot = var_ttemp_dn4;
        *var_ttemp_dn5_slot = var_ttemp_dn5;
        *var_ttemp_dn6_slot = var_ttemp_dn6;
        *var_ttemp_dn7_slot = var_ttemp_dn7;
        *var_ttemp_dn8_slot = var_ttemp_dn8;
        *var_ttemp_dn9_slot = var_ttemp_dn9;
        *var_uc_ibpc1_slot = var_uc_ibpc1;
        *var_uc_subld1_slot = var_uc_subld1;
        *var_vg2const_slot = var_vg2const;
        *var_vg2const_1_slot = var_vg2const_1;
        *var_vg2const_1_dn0_slot = var_vg2const_1_dn0;
        *var_vg2const_1_dn10_slot = var_vg2const_1_dn10;
        *var_vg2const_1_dn11_slot = var_vg2const_1_dn11;
        *var_vg2const_1_dn14_slot = var_vg2const_1_dn14;
        *var_vg2const_1_dn2_slot = var_vg2const_1_dn2;
        *var_vg2const_1_dn4_slot = var_vg2const_1_dn4;
        *var_vg2const_1_dn5_slot = var_vg2const_1_dn5;
        *var_vg2const_1_dn6_slot = var_vg2const_1_dn6;
        *var_vg2const_1_dn7_slot = var_vg2const_1_dn7;
        *var_vg2const_1_dn8_slot = var_vg2const_1_dn8;
        *var_vg2const_1_dn9_slot = var_vg2const_1_dn9;
        *var_vg2const_dn0_slot = var_vg2const_dn0;
        *var_vg2const_dn10_slot = var_vg2const_dn10;
        *var_vg2const_dn11_slot = var_vg2const_dn11;
        *var_vg2const_dn14_slot = var_vg2const_dn14;
        *var_vg2const_dn2_slot = var_vg2const_dn2;
        *var_vg2const_dn4_slot = var_vg2const_dn4;
        *var_vg2const_dn5_slot = var_vg2const_dn5;
        *var_vg2const_dn6_slot = var_vg2const_dn6;
        *var_vg2const_dn7_slot = var_vg2const_dn7;
        *var_vg2const_dn8_slot = var_vg2const_dn8;
        *var_vg2const_dn9_slot = var_vg2const_dn9;
        *var_xgate_slot = var_xgate;
        *var_xgate_1_slot = var_xgate_1;
        *var_xsub1_slot = var_xsub1;
        *var_xsub1_1_slot = var_xsub1_1;
        *var_xsub2_slot = var_xsub2;
        *var_xsub2_1_slot = var_xsub2_1;
        *var_xvbs_slot = var_xvbs;
        *var_xvbs_1_slot = var_xvbs_1;
    }

    pub(super) fn stamp_transient_block_22(
        p: &Parameters,
        var_ef_nsubc: f64,
        var_ef_nsubc_dn0: f64,
        var_ef_nsubc_dn10: f64,
        var_ef_nsubc_dn11: f64,
        var_ef_nsubc_dn14: f64,
        var_ef_nsubc_dn2: f64,
        var_ef_nsubc_dn4: f64,
        var_ef_nsubc_dn5: f64,
        var_ef_nsubc_dn6: f64,
        var_ef_nsubc_dn7: f64,
        var_ef_nsubc_dn8: f64,
        var_ef_nsubc_dn9: f64,
        var_egtnom: f64,
        var_guard293: f64,
        var_ktnom: f64,
        var_mueph: f64,
        var_mueph_dn0: f64,
        var_mueph_dn10: f64,
        var_mueph_dn11: f64,
        var_mueph_dn14: f64,
        var_mueph_dn2: f64,
        var_mueph_dn4: f64,
        var_mueph_dn5: f64,
        var_mueph_dn6: f64,
        var_mueph_dn7: f64,
        var_mueph_dn8: f64,
        var_mueph_dn9: f64,
        var_tdiff: f64,
        var_tdiff_dn0: f64,
        var_tdiff_dn10: f64,
        var_tdiff_dn11: f64,
        var_tdiff_dn14: f64,
        var_tdiff_dn2: f64,
        var_tdiff_dn4: f64,
        var_tdiff_dn5: f64,
        var_tdiff_dn6: f64,
        var_tdiff_dn7: f64,
        var_tdiff_dn8: f64,
        var_tdiff_dn9: f64,
        var_ttemp: f64,
        var_ttemp_dn0: f64,
        var_ttemp_dn10: f64,
        var_ttemp_dn11: f64,
        var_ttemp_dn14: f64,
        var_ttemp_dn2: f64,
        var_ttemp_dn4: f64,
        var_ttemp_dn5: f64,
        var_ttemp_dn6: f64,
        var_ttemp_dn7: f64,
        var_ttemp_dn8: f64,
        var_ttemp_dn9: f64,
        var_uc_bgtmp1: f64,
        var_uc_bgtmp2: f64,
        var_uc_codep: f64,
        var_uc_muetmp: f64,
        var_uc_ndepm: f64,
        var_uc_ndepm_dn0: f64,
        var_uc_ndepm_dn10: f64,
        var_uc_ndepm_dn11: f64,
        var_uc_ndepm_dn14: f64,
        var_uc_ndepm_dn2: f64,
        var_uc_ndepm_dn4: f64,
        var_uc_ndepm_dn5: f64,
        var_uc_ndepm_dn6: f64,
        var_uc_ndepm_dn7: f64,
        var_uc_ndepm_dn8: f64,
        var_uc_ndepm_dn9: f64,
        var_beta_slot: &mut f64,
        var_beta2_slot: &mut f64,
        var_beta2_dn0_slot: &mut f64,
        var_beta2_dn10_slot: &mut f64,
        var_beta2_dn11_slot: &mut f64,
        var_beta2_dn14_slot: &mut f64,
        var_beta2_dn2_slot: &mut f64,
        var_beta2_dn4_slot: &mut f64,
        var_beta2_dn5_slot: &mut f64,
        var_beta2_dn6_slot: &mut f64,
        var_beta2_dn7_slot: &mut f64,
        var_beta2_dn8_slot: &mut f64,
        var_beta2_dn9_slot: &mut f64,
        var_beta_dn0_slot: &mut f64,
        var_beta_dn10_slot: &mut f64,
        var_beta_dn11_slot: &mut f64,
        var_beta_dn14_slot: &mut f64,
        var_beta_dn2_slot: &mut f64,
        var_beta_dn4_slot: &mut f64,
        var_beta_dn5_slot: &mut f64,
        var_beta_dn6_slot: &mut f64,
        var_beta_dn7_slot: &mut f64,
        var_beta_dn8_slot: &mut f64,
        var_beta_dn9_slot: &mut f64,
        var_beta_inv_slot: &mut f64,
        var_beta_inv_dn0_slot: &mut f64,
        var_beta_inv_dn10_slot: &mut f64,
        var_beta_inv_dn11_slot: &mut f64,
        var_beta_inv_dn14_slot: &mut f64,
        var_beta_inv_dn2_slot: &mut f64,
        var_beta_inv_dn4_slot: &mut f64,
        var_beta_inv_dn5_slot: &mut f64,
        var_beta_inv_dn6_slot: &mut f64,
        var_beta_inv_dn7_slot: &mut f64,
        var_beta_inv_dn8_slot: &mut f64,
        var_beta_inv_dn9_slot: &mut f64,
        var_betatnom_slot: &mut f64,
        var_cnst0_slot: &mut f64,
        var_cnst0_dn0_slot: &mut f64,
        var_cnst0_dn10_slot: &mut f64,
        var_cnst0_dn11_slot: &mut f64,
        var_cnst0_dn14_slot: &mut f64,
        var_cnst0_dn2_slot: &mut f64,
        var_cnst0_dn4_slot: &mut f64,
        var_cnst0_dn5_slot: &mut f64,
        var_cnst0_dn6_slot: &mut f64,
        var_cnst0_dn7_slot: &mut f64,
        var_cnst0_dn8_slot: &mut f64,
        var_cnst0_dn9_slot: &mut f64,
        var_cnst1_slot: &mut f64,
        var_cnst1_dn0_slot: &mut f64,
        var_cnst1_dn10_slot: &mut f64,
        var_cnst1_dn11_slot: &mut f64,
        var_cnst1_dn14_slot: &mut f64,
        var_cnst1_dn2_slot: &mut f64,
        var_cnst1_dn4_slot: &mut f64,
        var_cnst1_dn5_slot: &mut f64,
        var_cnst1_dn6_slot: &mut f64,
        var_cnst1_dn7_slot: &mut f64,
        var_cnst1_dn8_slot: &mut f64,
        var_cnst1_dn9_slot: &mut f64,
        var_eg_slot: &mut f64,
        var_eg_dn0_slot: &mut f64,
        var_eg_dn10_slot: &mut f64,
        var_eg_dn11_slot: &mut f64,
        var_eg_dn14_slot: &mut f64,
        var_eg_dn2_slot: &mut f64,
        var_eg_dn4_slot: &mut f64,
        var_eg_dn5_slot: &mut f64,
        var_eg_dn6_slot: &mut f64,
        var_eg_dn7_slot: &mut f64,
        var_eg_dn8_slot: &mut f64,
        var_eg_dn9_slot: &mut f64,
        var_egp12_slot: &mut f64,
        var_egp12_dn0_slot: &mut f64,
        var_egp12_dn10_slot: &mut f64,
        var_egp12_dn11_slot: &mut f64,
        var_egp12_dn14_slot: &mut f64,
        var_egp12_dn2_slot: &mut f64,
        var_egp12_dn4_slot: &mut f64,
        var_egp12_dn5_slot: &mut f64,
        var_egp12_dn6_slot: &mut f64,
        var_egp12_dn7_slot: &mut f64,
        var_egp12_dn8_slot: &mut f64,
        var_egp12_dn9_slot: &mut f64,
        var_egp32_slot: &mut f64,
        var_egp32_dn0_slot: &mut f64,
        var_egp32_dn10_slot: &mut f64,
        var_egp32_dn11_slot: &mut f64,
        var_egp32_dn14_slot: &mut f64,
        var_egp32_dn2_slot: &mut f64,
        var_egp32_dn4_slot: &mut f64,
        var_egp32_dn5_slot: &mut f64,
        var_egp32_dn6_slot: &mut f64,
        var_egp32_dn7_slot: &mut f64,
        var_egp32_dn8_slot: &mut f64,
        var_egp32_dn9_slot: &mut f64,
        var_guard294_slot: &mut f64,
        var_log_tratio_slot: &mut f64,
        var_log_tratio_dn0_slot: &mut f64,
        var_log_tratio_dn10_slot: &mut f64,
        var_log_tratio_dn11_slot: &mut f64,
        var_log_tratio_dn14_slot: &mut f64,
        var_log_tratio_dn2_slot: &mut f64,
        var_log_tratio_dn4_slot: &mut f64,
        var_log_tratio_dn5_slot: &mut f64,
        var_log_tratio_dn6_slot: &mut f64,
        var_log_tratio_dn7_slot: &mut f64,
        var_log_tratio_dn8_slot: &mut f64,
        var_log_tratio_dn9_slot: &mut f64,
        var_mphn0_slot: &mut f64,
        var_mphn0_dn0_slot: &mut f64,
        var_mphn0_dn10_slot: &mut f64,
        var_mphn0_dn11_slot: &mut f64,
        var_mphn0_dn14_slot: &mut f64,
        var_mphn0_dn2_slot: &mut f64,
        var_mphn0_dn4_slot: &mut f64,
        var_mphn0_dn5_slot: &mut f64,
        var_mphn0_dn6_slot: &mut f64,
        var_mphn0_dn7_slot: &mut f64,
        var_mphn0_dn8_slot: &mut f64,
        var_mphn0_dn9_slot: &mut f64,
        var_nin_slot: &mut f64,
        var_nin_dn0_slot: &mut f64,
        var_nin_dn10_slot: &mut f64,
        var_nin_dn11_slot: &mut f64,
        var_nin_dn14_slot: &mut f64,
        var_nin_dn2_slot: &mut f64,
        var_nin_dn4_slot: &mut f64,
        var_nin_dn5_slot: &mut f64,
        var_nin_dn6_slot: &mut f64,
        var_nin_dn7_slot: &mut f64,
        var_nin_dn8_slot: &mut f64,
        var_nin_dn9_slot: &mut f64,
        var_pb2n_slot: &mut f64,
        var_pb2n_dn0_slot: &mut f64,
        var_pb2n_dn10_slot: &mut f64,
        var_pb2n_dn11_slot: &mut f64,
        var_pb2n_dn14_slot: &mut f64,
        var_pb2n_dn2_slot: &mut f64,
        var_pb2n_dn4_slot: &mut f64,
        var_pb2n_dn5_slot: &mut f64,
        var_pb2n_dn6_slot: &mut f64,
        var_pb2n_dn7_slot: &mut f64,
        var_pb2n_dn8_slot: &mut f64,
        var_pb2n_dn9_slot: &mut f64,
        var_sqrt_eg_slot: &mut f64,
        var_sqrt_eg_dn0_slot: &mut f64,
        var_sqrt_eg_dn10_slot: &mut f64,
        var_sqrt_eg_dn11_slot: &mut f64,
        var_sqrt_eg_dn14_slot: &mut f64,
        var_sqrt_eg_dn2_slot: &mut f64,
        var_sqrt_eg_dn4_slot: &mut f64,
        var_sqrt_eg_dn5_slot: &mut f64,
        var_sqrt_eg_dn6_slot: &mut f64,
        var_sqrt_eg_dn7_slot: &mut f64,
        var_sqrt_eg_dn8_slot: &mut f64,
        var_sqrt_eg_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_tdiff_2_slot: &mut f64,
        var_tdiff_2_dn0_slot: &mut f64,
        var_tdiff_2_dn10_slot: &mut f64,
        var_tdiff_2_dn11_slot: &mut f64,
        var_tdiff_2_dn14_slot: &mut f64,
        var_tdiff_2_dn2_slot: &mut f64,
        var_tdiff_2_dn4_slot: &mut f64,
        var_tdiff_2_dn5_slot: &mut f64,
        var_tdiff_2_dn6_slot: &mut f64,
        var_tdiff_2_dn7_slot: &mut f64,
        var_tdiff_2_dn8_slot: &mut f64,
        var_tdiff_2_dn9_slot: &mut f64,
        var_tratio_slot: &mut f64,
        var_tratio_dn0_slot: &mut f64,
        var_tratio_dn10_slot: &mut f64,
        var_tratio_dn11_slot: &mut f64,
        var_tratio_dn14_slot: &mut f64,
        var_tratio_dn2_slot: &mut f64,
        var_tratio_dn4_slot: &mut f64,
        var_tratio_dn5_slot: &mut f64,
        var_tratio_dn6_slot: &mut f64,
        var_tratio_dn7_slot: &mut f64,
        var_tratio_dn8_slot: &mut f64,
        var_tratio_dn9_slot: &mut f64,
        var_vbipn_slot: &mut f64,
        var_vbipn_dn0_slot: &mut f64,
        var_vbipn_dn10_slot: &mut f64,
        var_vbipn_dn11_slot: &mut f64,
        var_vbipn_dn14_slot: &mut f64,
        var_vbipn_dn2_slot: &mut f64,
        var_vbipn_dn4_slot: &mut f64,
        var_vbipn_dn5_slot: &mut f64,
        var_vbipn_dn6_slot: &mut f64,
        var_vbipn_dn7_slot: &mut f64,
        var_vbipn_dn8_slot: &mut f64,
        var_vbipn_dn9_slot: &mut f64,
    ) {
        let mut var_beta: f64 = *var_beta_slot;
        let mut var_beta2: f64 = *var_beta2_slot;
        let mut var_beta2_dn0: f64 = *var_beta2_dn0_slot;
        let mut var_beta2_dn10: f64 = *var_beta2_dn10_slot;
        let mut var_beta2_dn11: f64 = *var_beta2_dn11_slot;
        let mut var_beta2_dn14: f64 = *var_beta2_dn14_slot;
        let mut var_beta2_dn2: f64 = *var_beta2_dn2_slot;
        let mut var_beta2_dn4: f64 = *var_beta2_dn4_slot;
        let mut var_beta2_dn5: f64 = *var_beta2_dn5_slot;
        let mut var_beta2_dn6: f64 = *var_beta2_dn6_slot;
        let mut var_beta2_dn7: f64 = *var_beta2_dn7_slot;
        let mut var_beta2_dn8: f64 = *var_beta2_dn8_slot;
        let mut var_beta2_dn9: f64 = *var_beta2_dn9_slot;
        let mut var_beta_dn0: f64 = *var_beta_dn0_slot;
        let mut var_beta_dn10: f64 = *var_beta_dn10_slot;
        let mut var_beta_dn11: f64 = *var_beta_dn11_slot;
        let mut var_beta_dn14: f64 = *var_beta_dn14_slot;
        let mut var_beta_dn2: f64 = *var_beta_dn2_slot;
        let mut var_beta_dn4: f64 = *var_beta_dn4_slot;
        let mut var_beta_dn5: f64 = *var_beta_dn5_slot;
        let mut var_beta_dn6: f64 = *var_beta_dn6_slot;
        let mut var_beta_dn7: f64 = *var_beta_dn7_slot;
        let mut var_beta_dn8: f64 = *var_beta_dn8_slot;
        let mut var_beta_dn9: f64 = *var_beta_dn9_slot;
        let mut var_beta_inv: f64 = *var_beta_inv_slot;
        let mut var_beta_inv_dn0: f64 = *var_beta_inv_dn0_slot;
        let mut var_beta_inv_dn10: f64 = *var_beta_inv_dn10_slot;
        let mut var_beta_inv_dn11: f64 = *var_beta_inv_dn11_slot;
        let mut var_beta_inv_dn14: f64 = *var_beta_inv_dn14_slot;
        let mut var_beta_inv_dn2: f64 = *var_beta_inv_dn2_slot;
        let mut var_beta_inv_dn4: f64 = *var_beta_inv_dn4_slot;
        let mut var_beta_inv_dn5: f64 = *var_beta_inv_dn5_slot;
        let mut var_beta_inv_dn6: f64 = *var_beta_inv_dn6_slot;
        let mut var_beta_inv_dn7: f64 = *var_beta_inv_dn7_slot;
        let mut var_beta_inv_dn8: f64 = *var_beta_inv_dn8_slot;
        let mut var_beta_inv_dn9: f64 = *var_beta_inv_dn9_slot;
        let mut var_betatnom: f64 = *var_betatnom_slot;
        let mut var_cnst0: f64 = *var_cnst0_slot;
        let mut var_cnst0_dn0: f64 = *var_cnst0_dn0_slot;
        let mut var_cnst0_dn10: f64 = *var_cnst0_dn10_slot;
        let mut var_cnst0_dn11: f64 = *var_cnst0_dn11_slot;
        let mut var_cnst0_dn14: f64 = *var_cnst0_dn14_slot;
        let mut var_cnst0_dn2: f64 = *var_cnst0_dn2_slot;
        let mut var_cnst0_dn4: f64 = *var_cnst0_dn4_slot;
        let mut var_cnst0_dn5: f64 = *var_cnst0_dn5_slot;
        let mut var_cnst0_dn6: f64 = *var_cnst0_dn6_slot;
        let mut var_cnst0_dn7: f64 = *var_cnst0_dn7_slot;
        let mut var_cnst0_dn8: f64 = *var_cnst0_dn8_slot;
        let mut var_cnst0_dn9: f64 = *var_cnst0_dn9_slot;
        let mut var_cnst1: f64 = *var_cnst1_slot;
        let mut var_cnst1_dn0: f64 = *var_cnst1_dn0_slot;
        let mut var_cnst1_dn10: f64 = *var_cnst1_dn10_slot;
        let mut var_cnst1_dn11: f64 = *var_cnst1_dn11_slot;
        let mut var_cnst1_dn14: f64 = *var_cnst1_dn14_slot;
        let mut var_cnst1_dn2: f64 = *var_cnst1_dn2_slot;
        let mut var_cnst1_dn4: f64 = *var_cnst1_dn4_slot;
        let mut var_cnst1_dn5: f64 = *var_cnst1_dn5_slot;
        let mut var_cnst1_dn6: f64 = *var_cnst1_dn6_slot;
        let mut var_cnst1_dn7: f64 = *var_cnst1_dn7_slot;
        let mut var_cnst1_dn8: f64 = *var_cnst1_dn8_slot;
        let mut var_cnst1_dn9: f64 = *var_cnst1_dn9_slot;
        let mut var_eg: f64 = *var_eg_slot;
        let mut var_eg_dn0: f64 = *var_eg_dn0_slot;
        let mut var_eg_dn10: f64 = *var_eg_dn10_slot;
        let mut var_eg_dn11: f64 = *var_eg_dn11_slot;
        let mut var_eg_dn14: f64 = *var_eg_dn14_slot;
        let mut var_eg_dn2: f64 = *var_eg_dn2_slot;
        let mut var_eg_dn4: f64 = *var_eg_dn4_slot;
        let mut var_eg_dn5: f64 = *var_eg_dn5_slot;
        let mut var_eg_dn6: f64 = *var_eg_dn6_slot;
        let mut var_eg_dn7: f64 = *var_eg_dn7_slot;
        let mut var_eg_dn8: f64 = *var_eg_dn8_slot;
        let mut var_eg_dn9: f64 = *var_eg_dn9_slot;
        let mut var_egp12: f64 = *var_egp12_slot;
        let mut var_egp12_dn0: f64 = *var_egp12_dn0_slot;
        let mut var_egp12_dn10: f64 = *var_egp12_dn10_slot;
        let mut var_egp12_dn11: f64 = *var_egp12_dn11_slot;
        let mut var_egp12_dn14: f64 = *var_egp12_dn14_slot;
        let mut var_egp12_dn2: f64 = *var_egp12_dn2_slot;
        let mut var_egp12_dn4: f64 = *var_egp12_dn4_slot;
        let mut var_egp12_dn5: f64 = *var_egp12_dn5_slot;
        let mut var_egp12_dn6: f64 = *var_egp12_dn6_slot;
        let mut var_egp12_dn7: f64 = *var_egp12_dn7_slot;
        let mut var_egp12_dn8: f64 = *var_egp12_dn8_slot;
        let mut var_egp12_dn9: f64 = *var_egp12_dn9_slot;
        let mut var_egp32: f64 = *var_egp32_slot;
        let mut var_egp32_dn0: f64 = *var_egp32_dn0_slot;
        let mut var_egp32_dn10: f64 = *var_egp32_dn10_slot;
        let mut var_egp32_dn11: f64 = *var_egp32_dn11_slot;
        let mut var_egp32_dn14: f64 = *var_egp32_dn14_slot;
        let mut var_egp32_dn2: f64 = *var_egp32_dn2_slot;
        let mut var_egp32_dn4: f64 = *var_egp32_dn4_slot;
        let mut var_egp32_dn5: f64 = *var_egp32_dn5_slot;
        let mut var_egp32_dn6: f64 = *var_egp32_dn6_slot;
        let mut var_egp32_dn7: f64 = *var_egp32_dn7_slot;
        let mut var_egp32_dn8: f64 = *var_egp32_dn8_slot;
        let mut var_egp32_dn9: f64 = *var_egp32_dn9_slot;
        let mut var_guard294: f64 = *var_guard294_slot;
        let mut var_log_tratio: f64 = *var_log_tratio_slot;
        let mut var_log_tratio_dn0: f64 = *var_log_tratio_dn0_slot;
        let mut var_log_tratio_dn10: f64 = *var_log_tratio_dn10_slot;
        let mut var_log_tratio_dn11: f64 = *var_log_tratio_dn11_slot;
        let mut var_log_tratio_dn14: f64 = *var_log_tratio_dn14_slot;
        let mut var_log_tratio_dn2: f64 = *var_log_tratio_dn2_slot;
        let mut var_log_tratio_dn4: f64 = *var_log_tratio_dn4_slot;
        let mut var_log_tratio_dn5: f64 = *var_log_tratio_dn5_slot;
        let mut var_log_tratio_dn6: f64 = *var_log_tratio_dn6_slot;
        let mut var_log_tratio_dn7: f64 = *var_log_tratio_dn7_slot;
        let mut var_log_tratio_dn8: f64 = *var_log_tratio_dn8_slot;
        let mut var_log_tratio_dn9: f64 = *var_log_tratio_dn9_slot;
        let mut var_mphn0: f64 = *var_mphn0_slot;
        let mut var_mphn0_dn0: f64 = *var_mphn0_dn0_slot;
        let mut var_mphn0_dn10: f64 = *var_mphn0_dn10_slot;
        let mut var_mphn0_dn11: f64 = *var_mphn0_dn11_slot;
        let mut var_mphn0_dn14: f64 = *var_mphn0_dn14_slot;
        let mut var_mphn0_dn2: f64 = *var_mphn0_dn2_slot;
        let mut var_mphn0_dn4: f64 = *var_mphn0_dn4_slot;
        let mut var_mphn0_dn5: f64 = *var_mphn0_dn5_slot;
        let mut var_mphn0_dn6: f64 = *var_mphn0_dn6_slot;
        let mut var_mphn0_dn7: f64 = *var_mphn0_dn7_slot;
        let mut var_mphn0_dn8: f64 = *var_mphn0_dn8_slot;
        let mut var_mphn0_dn9: f64 = *var_mphn0_dn9_slot;
        let mut var_nin: f64 = *var_nin_slot;
        let mut var_nin_dn0: f64 = *var_nin_dn0_slot;
        let mut var_nin_dn10: f64 = *var_nin_dn10_slot;
        let mut var_nin_dn11: f64 = *var_nin_dn11_slot;
        let mut var_nin_dn14: f64 = *var_nin_dn14_slot;
        let mut var_nin_dn2: f64 = *var_nin_dn2_slot;
        let mut var_nin_dn4: f64 = *var_nin_dn4_slot;
        let mut var_nin_dn5: f64 = *var_nin_dn5_slot;
        let mut var_nin_dn6: f64 = *var_nin_dn6_slot;
        let mut var_nin_dn7: f64 = *var_nin_dn7_slot;
        let mut var_nin_dn8: f64 = *var_nin_dn8_slot;
        let mut var_nin_dn9: f64 = *var_nin_dn9_slot;
        let mut var_pb2n: f64 = *var_pb2n_slot;
        let mut var_pb2n_dn0: f64 = *var_pb2n_dn0_slot;
        let mut var_pb2n_dn10: f64 = *var_pb2n_dn10_slot;
        let mut var_pb2n_dn11: f64 = *var_pb2n_dn11_slot;
        let mut var_pb2n_dn14: f64 = *var_pb2n_dn14_slot;
        let mut var_pb2n_dn2: f64 = *var_pb2n_dn2_slot;
        let mut var_pb2n_dn4: f64 = *var_pb2n_dn4_slot;
        let mut var_pb2n_dn5: f64 = *var_pb2n_dn5_slot;
        let mut var_pb2n_dn6: f64 = *var_pb2n_dn6_slot;
        let mut var_pb2n_dn7: f64 = *var_pb2n_dn7_slot;
        let mut var_pb2n_dn8: f64 = *var_pb2n_dn8_slot;
        let mut var_pb2n_dn9: f64 = *var_pb2n_dn9_slot;
        let mut var_sqrt_eg: f64 = *var_sqrt_eg_slot;
        let mut var_sqrt_eg_dn0: f64 = *var_sqrt_eg_dn0_slot;
        let mut var_sqrt_eg_dn10: f64 = *var_sqrt_eg_dn10_slot;
        let mut var_sqrt_eg_dn11: f64 = *var_sqrt_eg_dn11_slot;
        let mut var_sqrt_eg_dn14: f64 = *var_sqrt_eg_dn14_slot;
        let mut var_sqrt_eg_dn2: f64 = *var_sqrt_eg_dn2_slot;
        let mut var_sqrt_eg_dn4: f64 = *var_sqrt_eg_dn4_slot;
        let mut var_sqrt_eg_dn5: f64 = *var_sqrt_eg_dn5_slot;
        let mut var_sqrt_eg_dn6: f64 = *var_sqrt_eg_dn6_slot;
        let mut var_sqrt_eg_dn7: f64 = *var_sqrt_eg_dn7_slot;
        let mut var_sqrt_eg_dn8: f64 = *var_sqrt_eg_dn8_slot;
        let mut var_sqrt_eg_dn9: f64 = *var_sqrt_eg_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_tdiff_2: f64 = *var_tdiff_2_slot;
        let mut var_tdiff_2_dn0: f64 = *var_tdiff_2_dn0_slot;
        let mut var_tdiff_2_dn10: f64 = *var_tdiff_2_dn10_slot;
        let mut var_tdiff_2_dn11: f64 = *var_tdiff_2_dn11_slot;
        let mut var_tdiff_2_dn14: f64 = *var_tdiff_2_dn14_slot;
        let mut var_tdiff_2_dn2: f64 = *var_tdiff_2_dn2_slot;
        let mut var_tdiff_2_dn4: f64 = *var_tdiff_2_dn4_slot;
        let mut var_tdiff_2_dn5: f64 = *var_tdiff_2_dn5_slot;
        let mut var_tdiff_2_dn6: f64 = *var_tdiff_2_dn6_slot;
        let mut var_tdiff_2_dn7: f64 = *var_tdiff_2_dn7_slot;
        let mut var_tdiff_2_dn8: f64 = *var_tdiff_2_dn8_slot;
        let mut var_tdiff_2_dn9: f64 = *var_tdiff_2_dn9_slot;
        let mut var_tratio: f64 = *var_tratio_slot;
        let mut var_tratio_dn0: f64 = *var_tratio_dn0_slot;
        let mut var_tratio_dn10: f64 = *var_tratio_dn10_slot;
        let mut var_tratio_dn11: f64 = *var_tratio_dn11_slot;
        let mut var_tratio_dn14: f64 = *var_tratio_dn14_slot;
        let mut var_tratio_dn2: f64 = *var_tratio_dn2_slot;
        let mut var_tratio_dn4: f64 = *var_tratio_dn4_slot;
        let mut var_tratio_dn5: f64 = *var_tratio_dn5_slot;
        let mut var_tratio_dn6: f64 = *var_tratio_dn6_slot;
        let mut var_tratio_dn7: f64 = *var_tratio_dn7_slot;
        let mut var_tratio_dn8: f64 = *var_tratio_dn8_slot;
        let mut var_tratio_dn9: f64 = *var_tratio_dn9_slot;
        let mut var_vbipn: f64 = *var_vbipn_slot;
        let mut var_vbipn_dn0: f64 = *var_vbipn_dn0_slot;
        let mut var_vbipn_dn10: f64 = *var_vbipn_dn10_slot;
        let mut var_vbipn_dn11: f64 = *var_vbipn_dn11_slot;
        let mut var_vbipn_dn14: f64 = *var_vbipn_dn14_slot;
        let mut var_vbipn_dn2: f64 = *var_vbipn_dn2_slot;
        let mut var_vbipn_dn4: f64 = *var_vbipn_dn4_slot;
        let mut var_vbipn_dn5: f64 = *var_vbipn_dn5_slot;
        let mut var_vbipn_dn6: f64 = *var_vbipn_dn6_slot;
        let mut var_vbipn_dn7: f64 = *var_vbipn_dn7_slot;
        let mut var_vbipn_dn8: f64 = *var_vbipn_dn8_slot;
        let mut var_vbipn_dn9: f64 = *var_vbipn_dn9_slot;

        let (assign13070_e7386, assign13070_e7386_d_n0, assign13070_e7386_d_n2, assign13070_e7386_d_n4, assign13070_e7386_d_n5, assign13070_e7386_d_n6, assign13070_e7386_d_n7, assign13070_e7386_d_n8, assign13070_e7386_d_n9, assign13070_e7386_d_n10, assign13070_e7386_d_n11, assign13070_e7386_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13070_e7380: f64 = (var_ttemp * var_ttemp);
        let assign13070_e7383: f64 = (var_ktnom * var_ktnom);
        let assign13070_e7384: f64 = (assign13070_e7380 - assign13070_e7383);
        (assign13070_e7384, ((var_ttemp_dn0 * var_ttemp) + (var_ttemp * var_ttemp_dn0)), ((var_ttemp_dn2 * var_ttemp) + (var_ttemp * var_ttemp_dn2)), ((var_ttemp_dn4 * var_ttemp) + (var_ttemp * var_ttemp_dn4)), ((var_ttemp_dn5 * var_ttemp) + (var_ttemp * var_ttemp_dn5)), ((var_ttemp_dn6 * var_ttemp) + (var_ttemp * var_ttemp_dn6)), ((var_ttemp_dn7 * var_ttemp) + (var_ttemp * var_ttemp_dn7)), ((var_ttemp_dn8 * var_ttemp) + (var_ttemp * var_ttemp_dn8)), ((var_ttemp_dn9 * var_ttemp) + (var_ttemp * var_ttemp_dn9)), ((var_ttemp_dn10 * var_ttemp) + (var_ttemp * var_ttemp_dn10)), ((var_ttemp_dn11 * var_ttemp) + (var_ttemp * var_ttemp_dn11)), ((var_ttemp_dn14 * var_ttemp) + (var_ttemp * var_ttemp_dn14)),)
    } else {
        (var_tdiff_2, var_tdiff_2_dn0, var_tdiff_2_dn2, var_tdiff_2_dn4, var_tdiff_2_dn5, var_tdiff_2_dn6, var_tdiff_2_dn7, var_tdiff_2_dn8, var_tdiff_2_dn9, var_tdiff_2_dn10, var_tdiff_2_dn11, var_tdiff_2_dn14,)
    }
};
        var_tdiff_2 = assign13070_e7386;
        var_tdiff_2_dn0 = assign13070_e7386_d_n0;
        var_tdiff_2_dn2 = assign13070_e7386_d_n2;
        var_tdiff_2_dn4 = assign13070_e7386_d_n4;
        var_tdiff_2_dn5 = assign13070_e7386_d_n5;
        var_tdiff_2_dn6 = assign13070_e7386_d_n6;
        var_tdiff_2_dn7 = assign13070_e7386_d_n7;
        var_tdiff_2_dn8 = assign13070_e7386_d_n8;
        var_tdiff_2_dn9 = assign13070_e7386_d_n9;
        var_tdiff_2_dn10 = assign13070_e7386_d_n10;
        var_tdiff_2_dn11 = assign13070_e7386_d_n11;
        var_tdiff_2_dn14 = assign13070_e7386_d_n14;

        let (assign13080_e7392, assign13080_e7392_d_n0, assign13080_e7392_d_n2, assign13080_e7392_d_n4, assign13080_e7392_d_n5, assign13080_e7392_d_n6, assign13080_e7392_d_n7, assign13080_e7392_d_n8, assign13080_e7392_d_n9, assign13080_e7392_d_n10, assign13080_e7392_d_n11, assign13080_e7392_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13080_e7390: f64 = (var_ttemp / var_ktnom);
        (assign13080_e7390, (var_ttemp_dn0 / var_ktnom), (var_ttemp_dn2 / var_ktnom), (var_ttemp_dn4 / var_ktnom), (var_ttemp_dn5 / var_ktnom), (var_ttemp_dn6 / var_ktnom), (var_ttemp_dn7 / var_ktnom), (var_ttemp_dn8 / var_ktnom), (var_ttemp_dn9 / var_ktnom), (var_ttemp_dn10 / var_ktnom), (var_ttemp_dn11 / var_ktnom), (var_ttemp_dn14 / var_ktnom),)
    } else {
        (var_tratio, var_tratio_dn0, var_tratio_dn2, var_tratio_dn4, var_tratio_dn5, var_tratio_dn6, var_tratio_dn7, var_tratio_dn8, var_tratio_dn9, var_tratio_dn10, var_tratio_dn11, var_tratio_dn14,)
    }
};
        var_tratio = assign13080_e7392;
        var_tratio_dn0 = assign13080_e7392_d_n0;
        var_tratio_dn2 = assign13080_e7392_d_n2;
        var_tratio_dn4 = assign13080_e7392_d_n4;
        var_tratio_dn5 = assign13080_e7392_d_n5;
        var_tratio_dn6 = assign13080_e7392_d_n6;
        var_tratio_dn7 = assign13080_e7392_d_n7;
        var_tratio_dn8 = assign13080_e7392_d_n8;
        var_tratio_dn9 = assign13080_e7392_d_n9;
        var_tratio_dn10 = assign13080_e7392_d_n10;
        var_tratio_dn11 = assign13080_e7392_d_n11;
        var_tratio_dn14 = assign13080_e7392_d_n14;

        let (assign13090_e7397, assign13090_e7397_d_n0, assign13090_e7397_d_n2, assign13090_e7397_d_n4, assign13090_e7397_d_n5, assign13090_e7397_d_n6, assign13090_e7397_d_n7, assign13090_e7397_d_n8, assign13090_e7397_d_n9, assign13090_e7397_d_n10, assign13090_e7397_d_n11, assign13090_e7397_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13090_e7395: f64 = (var_tratio).ln();
        (assign13090_e7395, (var_tratio_dn0 / var_tratio), (var_tratio_dn2 / var_tratio), (var_tratio_dn4 / var_tratio), (var_tratio_dn5 / var_tratio), (var_tratio_dn6 / var_tratio), (var_tratio_dn7 / var_tratio), (var_tratio_dn8 / var_tratio), (var_tratio_dn9 / var_tratio), (var_tratio_dn10 / var_tratio), (var_tratio_dn11 / var_tratio), (var_tratio_dn14 / var_tratio),)
    } else {
        (var_log_tratio, var_log_tratio_dn0, var_log_tratio_dn2, var_log_tratio_dn4, var_log_tratio_dn5, var_log_tratio_dn6, var_log_tratio_dn7, var_log_tratio_dn8, var_log_tratio_dn9, var_log_tratio_dn10, var_log_tratio_dn11, var_log_tratio_dn14,)
    }
};
        var_log_tratio = assign13090_e7397;
        var_log_tratio_dn0 = assign13090_e7397_d_n0;
        var_log_tratio_dn2 = assign13090_e7397_d_n2;
        var_log_tratio_dn4 = assign13090_e7397_d_n4;
        var_log_tratio_dn5 = assign13090_e7397_d_n5;
        var_log_tratio_dn6 = assign13090_e7397_d_n6;
        var_log_tratio_dn7 = assign13090_e7397_d_n7;
        var_log_tratio_dn8 = assign13090_e7397_d_n8;
        var_log_tratio_dn9 = assign13090_e7397_d_n9;
        var_log_tratio_dn10 = assign13090_e7397_d_n10;
        var_log_tratio_dn11 = assign13090_e7397_d_n11;
        var_log_tratio_dn14 = assign13090_e7397_d_n14;

        let (assign13100_e7409, assign13100_e7409_d_n0, assign13100_e7409_d_n2, assign13100_e7409_d_n4, assign13100_e7409_d_n5, assign13100_e7409_d_n6, assign13100_e7409_d_n7, assign13100_e7409_d_n8, assign13100_e7409_d_n9, assign13100_e7409_d_n10, assign13100_e7409_d_n11, assign13100_e7409_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13100_e7402: f64 = (var_uc_bgtmp1 * var_tdiff);
        let assign13100_e7403: f64 = (var_egtnom - assign13100_e7402);
        let assign13100_e7406: f64 = (var_uc_bgtmp2 * var_tdiff_2);
        let assign13100_e7407: f64 = (assign13100_e7403 - assign13100_e7406);
        (assign13100_e7407, ((-(var_uc_bgtmp1 * var_tdiff_dn0)) - (var_uc_bgtmp2 * var_tdiff_2_dn0)), ((-(var_uc_bgtmp1 * var_tdiff_dn2)) - (var_uc_bgtmp2 * var_tdiff_2_dn2)), ((-(var_uc_bgtmp1 * var_tdiff_dn4)) - (var_uc_bgtmp2 * var_tdiff_2_dn4)), ((-(var_uc_bgtmp1 * var_tdiff_dn5)) - (var_uc_bgtmp2 * var_tdiff_2_dn5)), ((-(var_uc_bgtmp1 * var_tdiff_dn6)) - (var_uc_bgtmp2 * var_tdiff_2_dn6)), ((-(var_uc_bgtmp1 * var_tdiff_dn7)) - (var_uc_bgtmp2 * var_tdiff_2_dn7)), ((-(var_uc_bgtmp1 * var_tdiff_dn8)) - (var_uc_bgtmp2 * var_tdiff_2_dn8)), ((-(var_uc_bgtmp1 * var_tdiff_dn9)) - (var_uc_bgtmp2 * var_tdiff_2_dn9)), ((-(var_uc_bgtmp1 * var_tdiff_dn10)) - (var_uc_bgtmp2 * var_tdiff_2_dn10)), ((-(var_uc_bgtmp1 * var_tdiff_dn11)) - (var_uc_bgtmp2 * var_tdiff_2_dn11)), ((-(var_uc_bgtmp1 * var_tdiff_dn14)) - (var_uc_bgtmp2 * var_tdiff_2_dn14)),)
    } else {
        (var_eg, var_eg_dn0, var_eg_dn2, var_eg_dn4, var_eg_dn5, var_eg_dn6, var_eg_dn7, var_eg_dn8, var_eg_dn9, var_eg_dn10, var_eg_dn11, var_eg_dn14,)
    }
};
        var_eg = assign13100_e7409;
        var_eg_dn0 = assign13100_e7409_d_n0;
        var_eg_dn2 = assign13100_e7409_d_n2;
        var_eg_dn4 = assign13100_e7409_d_n4;
        var_eg_dn5 = assign13100_e7409_d_n5;
        var_eg_dn6 = assign13100_e7409_d_n6;
        var_eg_dn7 = assign13100_e7409_d_n7;
        var_eg_dn8 = assign13100_e7409_d_n8;
        var_eg_dn9 = assign13100_e7409_d_n9;
        var_eg_dn10 = assign13100_e7409_d_n10;
        var_eg_dn11 = assign13100_e7409_d_n11;
        var_eg_dn14 = assign13100_e7409_d_n14;

        let (assign13110_e7414, assign13110_e7414_d_n0, assign13110_e7414_d_n2, assign13110_e7414_d_n4, assign13110_e7414_d_n5, assign13110_e7414_d_n6, assign13110_e7414_d_n7, assign13110_e7414_d_n8, assign13110_e7414_d_n9, assign13110_e7414_d_n10, assign13110_e7414_d_n11, assign13110_e7414_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13110_e7412: f64 = (var_eg).sqrt();
        (assign13110_e7412, (var_eg_dn0 / (2.0 * assign13110_e7412)), (var_eg_dn2 / (2.0 * assign13110_e7412)), (var_eg_dn4 / (2.0 * assign13110_e7412)), (var_eg_dn5 / (2.0 * assign13110_e7412)), (var_eg_dn6 / (2.0 * assign13110_e7412)), (var_eg_dn7 / (2.0 * assign13110_e7412)), (var_eg_dn8 / (2.0 * assign13110_e7412)), (var_eg_dn9 / (2.0 * assign13110_e7412)), (var_eg_dn10 / (2.0 * assign13110_e7412)), (var_eg_dn11 / (2.0 * assign13110_e7412)), (var_eg_dn14 / (2.0 * assign13110_e7412)),)
    } else {
        (var_sqrt_eg, var_sqrt_eg_dn0, var_sqrt_eg_dn2, var_sqrt_eg_dn4, var_sqrt_eg_dn5, var_sqrt_eg_dn6, var_sqrt_eg_dn7, var_sqrt_eg_dn8, var_sqrt_eg_dn9, var_sqrt_eg_dn10, var_sqrt_eg_dn11, var_sqrt_eg_dn14,)
    }
};
        var_sqrt_eg = assign13110_e7414;
        var_sqrt_eg_dn0 = assign13110_e7414_d_n0;
        var_sqrt_eg_dn2 = assign13110_e7414_d_n2;
        var_sqrt_eg_dn4 = assign13110_e7414_d_n4;
        var_sqrt_eg_dn5 = assign13110_e7414_d_n5;
        var_sqrt_eg_dn6 = assign13110_e7414_d_n6;
        var_sqrt_eg_dn7 = assign13110_e7414_d_n7;
        var_sqrt_eg_dn8 = assign13110_e7414_d_n8;
        var_sqrt_eg_dn9 = assign13110_e7414_d_n9;
        var_sqrt_eg_dn10 = assign13110_e7414_d_n10;
        var_sqrt_eg_dn11 = assign13110_e7414_d_n11;
        var_sqrt_eg_dn14 = assign13110_e7414_d_n14;

        let (assign13120_e7420, assign13120_e7420_d_n0, assign13120_e7420_d_n2, assign13120_e7420_d_n4, assign13120_e7420_d_n5, assign13120_e7420_d_n6, assign13120_e7420_d_n7, assign13120_e7420_d_n8, assign13120_e7420_d_n9, assign13120_e7420_d_n10, assign13120_e7420_d_n11, assign13120_e7420_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13120_e7418: f64 = (1.0 / var_ttemp);
        (assign13120_e7418, (-(var_ttemp_dn0 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn2 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn4 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn5 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn6 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn7 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn8 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn9 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn10 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn11 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn14 / (var_ttemp * var_ttemp))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign13120_e7420;
        var_t1_dn0 = assign13120_e7420_d_n0;
        var_t1_dn2 = assign13120_e7420_d_n2;
        var_t1_dn4 = assign13120_e7420_d_n4;
        var_t1_dn5 = assign13120_e7420_d_n5;
        var_t1_dn6 = assign13120_e7420_d_n6;
        var_t1_dn7 = assign13120_e7420_d_n7;
        var_t1_dn8 = assign13120_e7420_d_n8;
        var_t1_dn9 = assign13120_e7420_d_n9;
        var_t1_dn10 = assign13120_e7420_d_n10;
        var_t1_dn11 = assign13120_e7420_d_n11;
        var_t1_dn14 = assign13120_e7420_d_n14;

        let (assign13130_e7426, assign13130_e7426_d_n0, assign13130_e7426_d_n2, assign13130_e7426_d_n4, assign13130_e7426_d_n5, assign13130_e7426_d_n6, assign13130_e7426_d_n7, assign13130_e7426_d_n8, assign13130_e7426_d_n9, assign13130_e7426_d_n10, assign13130_e7426_d_n11, assign13130_e7426_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13130_e7424: f64 = (1.0 / var_ktnom);
        (assign13130_e7424, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign13130_e7426;
        var_t2_dn0 = assign13130_e7426_d_n0;
        var_t2_dn2 = assign13130_e7426_d_n2;
        var_t2_dn4 = assign13130_e7426_d_n4;
        var_t2_dn5 = assign13130_e7426_d_n5;
        var_t2_dn6 = assign13130_e7426_d_n6;
        var_t2_dn7 = assign13130_e7426_d_n7;
        var_t2_dn8 = assign13130_e7426_d_n8;
        var_t2_dn9 = assign13130_e7426_d_n9;
        var_t2_dn10 = assign13130_e7426_d_n10;
        var_t2_dn11 = assign13130_e7426_d_n11;
        var_t2_dn14 = assign13130_e7426_d_n14;

        let (assign13140_e7448, assign13140_e7448_d_n0, assign13140_e7448_d_n2, assign13140_e7448_d_n4, assign13140_e7448_d_n5, assign13140_e7448_d_n6, assign13140_e7448_d_n7, assign13140_e7448_d_n8, assign13140_e7448_d_n9, assign13140_e7448_d_n10, assign13140_e7448_d_n11, assign13140_e7448_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13140_e7430: f64 = (var_egtnom + p.p259);
        let assign13140_e7434: f64 = (var_t1 - var_t2);
        let assign13140_e7435: f64 = (p.p260 * assign13140_e7434);
        let assign13140_e7436: f64 = (assign13140_e7430 + assign13140_e7435);
        let assign13140_e7440: f64 = (var_t1 * var_t1);
        let assign13140_e7443: f64 = (var_t2 * var_t2);
        let assign13140_e7444: f64 = (assign13140_e7440 - assign13140_e7443);
        let assign13140_e7445: f64 = (p.p261 * assign13140_e7444);
        let assign13140_e7446: f64 = (assign13140_e7436 + assign13140_e7445);
        (assign13140_e7446, ((p.p260 * (var_t1_dn0 - var_t2_dn0)) + (p.p261 * (((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) - ((var_t2_dn0 * var_t2) + (var_t2 * var_t2_dn0))))), ((p.p260 * (var_t1_dn2 - var_t2_dn2)) + (p.p261 * (((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) - ((var_t2_dn2 * var_t2) + (var_t2 * var_t2_dn2))))), ((p.p260 * (var_t1_dn4 - var_t2_dn4)) + (p.p261 * (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) - ((var_t2_dn4 * var_t2) + (var_t2 * var_t2_dn4))))), ((p.p260 * (var_t1_dn5 - var_t2_dn5)) + (p.p261 * (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) - ((var_t2_dn5 * var_t2) + (var_t2 * var_t2_dn5))))), ((p.p260 * (var_t1_dn6 - var_t2_dn6)) + (p.p261 * (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) - ((var_t2_dn6 * var_t2) + (var_t2 * var_t2_dn6))))), ((p.p260 * (var_t1_dn7 - var_t2_dn7)) + (p.p261 * (((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) - ((var_t2_dn7 * var_t2) + (var_t2 * var_t2_dn7))))), ((p.p260 * (var_t1_dn8 - var_t2_dn8)) + (p.p261 * (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) - ((var_t2_dn8 * var_t2) + (var_t2 * var_t2_dn8))))), ((p.p260 * (var_t1_dn9 - var_t2_dn9)) + (p.p261 * (((var_t1_dn9 * var_t1) + (var_t1 * var_t1_dn9)) - ((var_t2_dn9 * var_t2) + (var_t2 * var_t2_dn9))))), ((p.p260 * (var_t1_dn10 - var_t2_dn10)) + (p.p261 * (((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) - ((var_t2_dn10 * var_t2) + (var_t2 * var_t2_dn10))))), ((p.p260 * (var_t1_dn11 - var_t2_dn11)) + (p.p261 * (((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)) - ((var_t2_dn11 * var_t2) + (var_t2 * var_t2_dn11))))), ((p.p260 * (var_t1_dn14 - var_t2_dn14)) + (p.p261 * (((var_t1_dn14 * var_t1) + (var_t1 * var_t1_dn14)) - ((var_t2_dn14 * var_t2) + (var_t2 * var_t2_dn14))))),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign13140_e7448;
        var_t3_dn0 = assign13140_e7448_d_n0;
        var_t3_dn2 = assign13140_e7448_d_n2;
        var_t3_dn4 = assign13140_e7448_d_n4;
        var_t3_dn5 = assign13140_e7448_d_n5;
        var_t3_dn6 = assign13140_e7448_d_n6;
        var_t3_dn7 = assign13140_e7448_d_n7;
        var_t3_dn8 = assign13140_e7448_d_n8;
        var_t3_dn9 = assign13140_e7448_d_n9;
        var_t3_dn10 = assign13140_e7448_d_n10;
        var_t3_dn11 = assign13140_e7448_d_n11;
        var_t3_dn14 = assign13140_e7448_d_n14;

        let (assign13150_e7453, assign13150_e7453_d_n0, assign13150_e7453_d_n2, assign13150_e7453_d_n4, assign13150_e7453_d_n5, assign13150_e7453_d_n6, assign13150_e7453_d_n7, assign13150_e7453_d_n8, assign13150_e7453_d_n9, assign13150_e7453_d_n10, assign13150_e7453_d_n11, assign13150_e7453_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13150_e7451: f64 = (var_t3).sqrt();
        (assign13150_e7451, (var_t3_dn0 / (2.0 * assign13150_e7451)), (var_t3_dn2 / (2.0 * assign13150_e7451)), (var_t3_dn4 / (2.0 * assign13150_e7451)), (var_t3_dn5 / (2.0 * assign13150_e7451)), (var_t3_dn6 / (2.0 * assign13150_e7451)), (var_t3_dn7 / (2.0 * assign13150_e7451)), (var_t3_dn8 / (2.0 * assign13150_e7451)), (var_t3_dn9 / (2.0 * assign13150_e7451)), (var_t3_dn10 / (2.0 * assign13150_e7451)), (var_t3_dn11 / (2.0 * assign13150_e7451)), (var_t3_dn14 / (2.0 * assign13150_e7451)),)
    } else {
        (var_egp12, var_egp12_dn0, var_egp12_dn2, var_egp12_dn4, var_egp12_dn5, var_egp12_dn6, var_egp12_dn7, var_egp12_dn8, var_egp12_dn9, var_egp12_dn10, var_egp12_dn11, var_egp12_dn14,)
    }
};
        var_egp12 = assign13150_e7453;
        var_egp12_dn0 = assign13150_e7453_d_n0;
        var_egp12_dn2 = assign13150_e7453_d_n2;
        var_egp12_dn4 = assign13150_e7453_d_n4;
        var_egp12_dn5 = assign13150_e7453_d_n5;
        var_egp12_dn6 = assign13150_e7453_d_n6;
        var_egp12_dn7 = assign13150_e7453_d_n7;
        var_egp12_dn8 = assign13150_e7453_d_n8;
        var_egp12_dn9 = assign13150_e7453_d_n9;
        var_egp12_dn10 = assign13150_e7453_d_n10;
        var_egp12_dn11 = assign13150_e7453_d_n11;
        var_egp12_dn14 = assign13150_e7453_d_n14;

        let (assign13160_e7459, assign13160_e7459_d_n0, assign13160_e7459_d_n2, assign13160_e7459_d_n4, assign13160_e7459_d_n5, assign13160_e7459_d_n6, assign13160_e7459_d_n7, assign13160_e7459_d_n8, assign13160_e7459_d_n9, assign13160_e7459_d_n10, assign13160_e7459_d_n11, assign13160_e7459_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13160_e7457: f64 = (var_t3 * var_egp12);
        (assign13160_e7457, ((var_t3_dn0 * var_egp12) + (var_t3 * var_egp12_dn0)), ((var_t3_dn2 * var_egp12) + (var_t3 * var_egp12_dn2)), ((var_t3_dn4 * var_egp12) + (var_t3 * var_egp12_dn4)), ((var_t3_dn5 * var_egp12) + (var_t3 * var_egp12_dn5)), ((var_t3_dn6 * var_egp12) + (var_t3 * var_egp12_dn6)), ((var_t3_dn7 * var_egp12) + (var_t3 * var_egp12_dn7)), ((var_t3_dn8 * var_egp12) + (var_t3 * var_egp12_dn8)), ((var_t3_dn9 * var_egp12) + (var_t3 * var_egp12_dn9)), ((var_t3_dn10 * var_egp12) + (var_t3 * var_egp12_dn10)), ((var_t3_dn11 * var_egp12) + (var_t3 * var_egp12_dn11)), ((var_t3_dn14 * var_egp12) + (var_t3 * var_egp12_dn14)),)
    } else {
        (var_egp32, var_egp32_dn0, var_egp32_dn2, var_egp32_dn4, var_egp32_dn5, var_egp32_dn6, var_egp32_dn7, var_egp32_dn8, var_egp32_dn9, var_egp32_dn10, var_egp32_dn11, var_egp32_dn14,)
    }
};
        var_egp32 = assign13160_e7459;
        var_egp32_dn0 = assign13160_e7459_d_n0;
        var_egp32_dn2 = assign13160_e7459_d_n2;
        var_egp32_dn4 = assign13160_e7459_d_n4;
        var_egp32_dn5 = assign13160_e7459_d_n5;
        var_egp32_dn6 = assign13160_e7459_d_n6;
        var_egp32_dn7 = assign13160_e7459_d_n7;
        var_egp32_dn8 = assign13160_e7459_d_n8;
        var_egp32_dn9 = assign13160_e7459_d_n9;
        var_egp32_dn10 = assign13160_e7459_d_n10;
        var_egp32_dn11 = assign13160_e7459_d_n11;
        var_egp32_dn14 = assign13160_e7459_d_n14;

        let (assign13170_e7467, assign13170_e7467_d_n0, assign13170_e7467_d_n2, assign13170_e7467_d_n4, assign13170_e7467_d_n5, assign13170_e7467_d_n6, assign13170_e7467_d_n7, assign13170_e7467_d_n8, assign13170_e7467_d_n9, assign13170_e7467_d_n10, assign13170_e7467_d_n11, assign13170_e7467_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13170_e7464: f64 = (1.3806226e-23 * var_ttemp);
        let assign13170_e7465: f64 = (1.6021918e-19 / assign13170_e7464);
        (assign13170_e7465, (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn0)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn2)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn4)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn5)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn6)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn7)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn8)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn9)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn10)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn11)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn14)) / (assign13170_e7464 * assign13170_e7464))),)
    } else {
        (var_beta, var_beta_dn0, var_beta_dn2, var_beta_dn4, var_beta_dn5, var_beta_dn6, var_beta_dn7, var_beta_dn8, var_beta_dn9, var_beta_dn10, var_beta_dn11, var_beta_dn14,)
    }
};
        var_beta = assign13170_e7467;
        var_beta_dn0 = assign13170_e7467_d_n0;
        var_beta_dn2 = assign13170_e7467_d_n2;
        var_beta_dn4 = assign13170_e7467_d_n4;
        var_beta_dn5 = assign13170_e7467_d_n5;
        var_beta_dn6 = assign13170_e7467_d_n6;
        var_beta_dn7 = assign13170_e7467_d_n7;
        var_beta_dn8 = assign13170_e7467_d_n8;
        var_beta_dn9 = assign13170_e7467_d_n9;
        var_beta_dn10 = assign13170_e7467_d_n10;
        var_beta_dn11 = assign13170_e7467_d_n11;
        var_beta_dn14 = assign13170_e7467_d_n14;

        let (assign13180_e7473, assign13180_e7473_d_n0, assign13180_e7473_d_n2, assign13180_e7473_d_n4, assign13180_e7473_d_n5, assign13180_e7473_d_n6, assign13180_e7473_d_n7, assign13180_e7473_d_n8, assign13180_e7473_d_n9, assign13180_e7473_d_n10, assign13180_e7473_d_n11, assign13180_e7473_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13180_e7471: f64 = (1.0 / var_beta);
        (assign13180_e7471, (-(var_beta_dn0 / (var_beta * var_beta))), (-(var_beta_dn2 / (var_beta * var_beta))), (-(var_beta_dn4 / (var_beta * var_beta))), (-(var_beta_dn5 / (var_beta * var_beta))), (-(var_beta_dn6 / (var_beta * var_beta))), (-(var_beta_dn7 / (var_beta * var_beta))), (-(var_beta_dn8 / (var_beta * var_beta))), (-(var_beta_dn9 / (var_beta * var_beta))), (-(var_beta_dn10 / (var_beta * var_beta))), (-(var_beta_dn11 / (var_beta * var_beta))), (-(var_beta_dn14 / (var_beta * var_beta))),)
    } else {
        (var_beta_inv, var_beta_inv_dn0, var_beta_inv_dn2, var_beta_inv_dn4, var_beta_inv_dn5, var_beta_inv_dn6, var_beta_inv_dn7, var_beta_inv_dn8, var_beta_inv_dn9, var_beta_inv_dn10, var_beta_inv_dn11, var_beta_inv_dn14,)
    }
};
        var_beta_inv = assign13180_e7473;
        var_beta_inv_dn0 = assign13180_e7473_d_n0;
        var_beta_inv_dn2 = assign13180_e7473_d_n2;
        var_beta_inv_dn4 = assign13180_e7473_d_n4;
        var_beta_inv_dn5 = assign13180_e7473_d_n5;
        var_beta_inv_dn6 = assign13180_e7473_d_n6;
        var_beta_inv_dn7 = assign13180_e7473_d_n7;
        var_beta_inv_dn8 = assign13180_e7473_d_n8;
        var_beta_inv_dn9 = assign13180_e7473_d_n9;
        var_beta_inv_dn10 = assign13180_e7473_d_n10;
        var_beta_inv_dn11 = assign13180_e7473_d_n11;
        var_beta_inv_dn14 = assign13180_e7473_d_n14;

        let (assign13190_e7479, assign13190_e7479_d_n0, assign13190_e7479_d_n2, assign13190_e7479_d_n4, assign13190_e7479_d_n5, assign13190_e7479_d_n6, assign13190_e7479_d_n7, assign13190_e7479_d_n8, assign13190_e7479_d_n9, assign13190_e7479_d_n10, assign13190_e7479_d_n11, assign13190_e7479_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13190_e7477: f64 = (var_beta * var_beta);
        (assign13190_e7477, ((var_beta_dn0 * var_beta) + (var_beta * var_beta_dn0)), ((var_beta_dn2 * var_beta) + (var_beta * var_beta_dn2)), ((var_beta_dn4 * var_beta) + (var_beta * var_beta_dn4)), ((var_beta_dn5 * var_beta) + (var_beta * var_beta_dn5)), ((var_beta_dn6 * var_beta) + (var_beta * var_beta_dn6)), ((var_beta_dn7 * var_beta) + (var_beta * var_beta_dn7)), ((var_beta_dn8 * var_beta) + (var_beta * var_beta_dn8)), ((var_beta_dn9 * var_beta) + (var_beta * var_beta_dn9)), ((var_beta_dn10 * var_beta) + (var_beta * var_beta_dn10)), ((var_beta_dn11 * var_beta) + (var_beta * var_beta_dn11)), ((var_beta_dn14 * var_beta) + (var_beta * var_beta_dn14)),)
    } else {
        (var_beta2, var_beta2_dn0, var_beta2_dn2, var_beta2_dn4, var_beta2_dn5, var_beta2_dn6, var_beta2_dn7, var_beta2_dn8, var_beta2_dn9, var_beta2_dn10, var_beta2_dn11, var_beta2_dn14,)
    }
};
        var_beta2 = assign13190_e7479;
        var_beta2_dn0 = assign13190_e7479_d_n0;
        var_beta2_dn2 = assign13190_e7479_d_n2;
        var_beta2_dn4 = assign13190_e7479_d_n4;
        var_beta2_dn5 = assign13190_e7479_d_n5;
        var_beta2_dn6 = assign13190_e7479_d_n6;
        var_beta2_dn7 = assign13190_e7479_d_n7;
        var_beta2_dn8 = assign13190_e7479_d_n8;
        var_beta2_dn9 = assign13190_e7479_d_n9;
        var_beta2_dn10 = assign13190_e7479_d_n10;
        var_beta2_dn11 = assign13190_e7479_d_n11;
        var_beta2_dn14 = assign13190_e7479_d_n14;

        let (assign13200_e7487,) = {
    if (var_guard293 != 0.0) {
        let assign13200_e7484: f64 = (1.3806226e-23 * var_ktnom);
        let assign13200_e7485: f64 = (1.6021918e-19 / assign13200_e7484);
        (assign13200_e7485,)
    } else {
        (var_betatnom,)
    }
};
        var_betatnom = assign13200_e7487;

        let (assign13210_e7510, assign13210_e7510_d_n0, assign13210_e7510_d_n2, assign13210_e7510_d_n4, assign13210_e7510_d_n5, assign13210_e7510_d_n6, assign13210_e7510_d_n7, assign13210_e7510_d_n8, assign13210_e7510_d_n9, assign13210_e7510_d_n10, assign13210_e7510_d_n11, assign13210_e7510_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13210_e7492: f64 = (var_log_tratio * 1.5);
        let assign13210_e7493: f64 = (assign13210_e7492).exp();
        let assign13210_e7494: f64 = (1.04e16 * assign13210_e7493);
        let assign13210_e7496: f64 = (-var_eg);
        let assign13210_e7498: f64 = (assign13210_e7496 / 2.0);
        let assign13210_e7500: f64 = (assign13210_e7498 * var_beta);
        let assign13210_e7503: f64 = (var_egtnom / 2.0);
        let assign13210_e7505: f64 = (assign13210_e7503 * var_betatnom);
        let assign13210_e7506: f64 = (assign13210_e7500 + assign13210_e7505);
        let assign13210_e7507: f64 = (assign13210_e7506).exp();
        let assign13210_e7508: f64 = (assign13210_e7494 * assign13210_e7507);
        (assign13210_e7508, (((1.04e16 * (assign13210_e7493 * (var_log_tratio_dn0 * 1.5))) * assign13210_e7507) + (assign13210_e7494 * (assign13210_e7507 * ((((-var_eg_dn0) / 2.0) * var_beta) + (assign13210_e7498 * var_beta_dn0))))), (((1.04e16 * (assign13210_e7493 * (var_log_tratio_dn2 * 1.5))) * assign13210_e7507) + (assign13210_e7494 * (assign13210_e7507 * ((((-var_eg_dn2) / 2.0) * var_beta) + (assign13210_e7498 * var_beta_dn2))))), (((1.04e16 * (assign13210_e7493 * (var_log_tratio_dn4 * 1.5))) * assign13210_e7507) + (assign13210_e7494 * (assign13210_e7507 * ((((-var_eg_dn4) / 2.0) * var_beta) + (assign13210_e7498 * var_beta_dn4))))), (((1.04e16 * (assign13210_e7493 * (var_log_tratio_dn5 * 1.5))) * assign13210_e7507) + (assign13210_e7494 * (assign13210_e7507 * ((((-var_eg_dn5) / 2.0) * var_beta) + (assign13210_e7498 * var_beta_dn5))))), (((1.04e16 * (assign13210_e7493 * (var_log_tratio_dn6 * 1.5))) * assign13210_e7507) + (assign13210_e7494 * (assign13210_e7507 * ((((-var_eg_dn6) / 2.0) * var_beta) + (assign13210_e7498 * var_beta_dn6))))), (((1.04e16 * (assign13210_e7493 * (var_log_tratio_dn7 * 1.5))) * assign13210_e7507) + (assign13210_e7494 * (assign13210_e7507 * ((((-var_eg_dn7) / 2.0) * var_beta) + (assign13210_e7498 * var_beta_dn7))))), (((1.04e16 * (assign13210_e7493 * (var_log_tratio_dn8 * 1.5))) * assign13210_e7507) + (assign13210_e7494 * (assign13210_e7507 * ((((-var_eg_dn8) / 2.0) * var_beta) + (assign13210_e7498 * var_beta_dn8))))), (((1.04e16 * (assign13210_e7493 * (var_log_tratio_dn9 * 1.5))) * assign13210_e7507) + (assign13210_e7494 * (assign13210_e7507 * ((((-var_eg_dn9) / 2.0) * var_beta) + (assign13210_e7498 * var_beta_dn9))))), (((1.04e16 * (assign13210_e7493 * (var_log_tratio_dn10 * 1.5))) * assign13210_e7507) + (assign13210_e7494 * (assign13210_e7507 * ((((-var_eg_dn10) / 2.0) * var_beta) + (assign13210_e7498 * var_beta_dn10))))), (((1.04e16 * (assign13210_e7493 * (var_log_tratio_dn11 * 1.5))) * assign13210_e7507) + (assign13210_e7494 * (assign13210_e7507 * ((((-var_eg_dn11) / 2.0) * var_beta) + (assign13210_e7498 * var_beta_dn11))))), (((1.04e16 * (assign13210_e7493 * (var_log_tratio_dn14 * 1.5))) * assign13210_e7507) + (assign13210_e7494 * (assign13210_e7507 * ((((-var_eg_dn14) / 2.0) * var_beta) + (assign13210_e7498 * var_beta_dn14))))),)
    } else {
        (var_nin, var_nin_dn0, var_nin_dn2, var_nin_dn4, var_nin_dn5, var_nin_dn6, var_nin_dn7, var_nin_dn8, var_nin_dn9, var_nin_dn10, var_nin_dn11, var_nin_dn14,)
    }
};
        var_nin = assign13210_e7510;
        var_nin_dn0 = assign13210_e7510_d_n0;
        var_nin_dn2 = assign13210_e7510_d_n2;
        var_nin_dn4 = assign13210_e7510_d_n4;
        var_nin_dn5 = assign13210_e7510_d_n5;
        var_nin_dn6 = assign13210_e7510_d_n6;
        var_nin_dn7 = assign13210_e7510_d_n7;
        var_nin_dn8 = assign13210_e7510_d_n8;
        var_nin_dn9 = assign13210_e7510_d_n9;
        var_nin_dn10 = assign13210_e7510_d_n10;
        var_nin_dn11 = assign13210_e7510_d_n11;
        var_nin_dn14 = assign13210_e7510_d_n14;

        let (assign13220_e7517, assign13220_e7517_d_n0, assign13220_e7517_d_n2, assign13220_e7517_d_n4, assign13220_e7517_d_n5, assign13220_e7517_d_n6, assign13220_e7517_d_n7, assign13220_e7517_d_n8, assign13220_e7517_d_n9, assign13220_e7517_d_n10, assign13220_e7517_d_n11, assign13220_e7517_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13220_e7514: f64 = (var_log_tratio * var_uc_muetmp);
        let assign13220_e7515: f64 = (assign13220_e7514).exp();
        (assign13220_e7515, (assign13220_e7515 * (var_log_tratio_dn0 * var_uc_muetmp)), (assign13220_e7515 * (var_log_tratio_dn2 * var_uc_muetmp)), (assign13220_e7515 * (var_log_tratio_dn4 * var_uc_muetmp)), (assign13220_e7515 * (var_log_tratio_dn5 * var_uc_muetmp)), (assign13220_e7515 * (var_log_tratio_dn6 * var_uc_muetmp)), (assign13220_e7515 * (var_log_tratio_dn7 * var_uc_muetmp)), (assign13220_e7515 * (var_log_tratio_dn8 * var_uc_muetmp)), (assign13220_e7515 * (var_log_tratio_dn9 * var_uc_muetmp)), (assign13220_e7515 * (var_log_tratio_dn10 * var_uc_muetmp)), (assign13220_e7515 * (var_log_tratio_dn11 * var_uc_muetmp)), (assign13220_e7515 * (var_log_tratio_dn14 * var_uc_muetmp)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign13220_e7517;
        var_t1_dn0 = assign13220_e7517_d_n0;
        var_t1_dn2 = assign13220_e7517_d_n2;
        var_t1_dn4 = assign13220_e7517_d_n4;
        var_t1_dn5 = assign13220_e7517_d_n5;
        var_t1_dn6 = assign13220_e7517_d_n6;
        var_t1_dn7 = assign13220_e7517_d_n7;
        var_t1_dn8 = assign13220_e7517_d_n8;
        var_t1_dn9 = assign13220_e7517_d_n9;
        var_t1_dn10 = assign13220_e7517_d_n10;
        var_t1_dn11 = assign13220_e7517_d_n11;
        var_t1_dn14 = assign13220_e7517_d_n14;

        let (assign13230_e7523, assign13230_e7523_d_n0, assign13230_e7523_d_n2, assign13230_e7523_d_n4, assign13230_e7523_d_n5, assign13230_e7523_d_n6, assign13230_e7523_d_n7, assign13230_e7523_d_n8, assign13230_e7523_d_n9, assign13230_e7523_d_n10, assign13230_e7523_d_n11, assign13230_e7523_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13230_e7521: f64 = (var_t1 / var_mueph);
        (assign13230_e7521, (((var_t1_dn0 * var_mueph) - (var_t1 * var_mueph_dn0)) / (var_mueph * var_mueph)), (((var_t1_dn2 * var_mueph) - (var_t1 * var_mueph_dn2)) / (var_mueph * var_mueph)), (((var_t1_dn4 * var_mueph) - (var_t1 * var_mueph_dn4)) / (var_mueph * var_mueph)), (((var_t1_dn5 * var_mueph) - (var_t1 * var_mueph_dn5)) / (var_mueph * var_mueph)), (((var_t1_dn6 * var_mueph) - (var_t1 * var_mueph_dn6)) / (var_mueph * var_mueph)), (((var_t1_dn7 * var_mueph) - (var_t1 * var_mueph_dn7)) / (var_mueph * var_mueph)), (((var_t1_dn8 * var_mueph) - (var_t1 * var_mueph_dn8)) / (var_mueph * var_mueph)), (((var_t1_dn9 * var_mueph) - (var_t1 * var_mueph_dn9)) / (var_mueph * var_mueph)), (((var_t1_dn10 * var_mueph) - (var_t1 * var_mueph_dn10)) / (var_mueph * var_mueph)), (((var_t1_dn11 * var_mueph) - (var_t1 * var_mueph_dn11)) / (var_mueph * var_mueph)), (((var_t1_dn14 * var_mueph) - (var_t1 * var_mueph_dn14)) / (var_mueph * var_mueph)),)
    } else {
        (var_mphn0, var_mphn0_dn0, var_mphn0_dn2, var_mphn0_dn4, var_mphn0_dn5, var_mphn0_dn6, var_mphn0_dn7, var_mphn0_dn8, var_mphn0_dn9, var_mphn0_dn10, var_mphn0_dn11, var_mphn0_dn14,)
    }
};
        var_mphn0 = assign13230_e7523;
        var_mphn0_dn0 = assign13230_e7523_d_n0;
        var_mphn0_dn2 = assign13230_e7523_d_n2;
        var_mphn0_dn4 = assign13230_e7523_d_n4;
        var_mphn0_dn5 = assign13230_e7523_d_n5;
        var_mphn0_dn6 = assign13230_e7523_d_n6;
        var_mphn0_dn7 = assign13230_e7523_d_n7;
        var_mphn0_dn8 = assign13230_e7523_d_n8;
        var_mphn0_dn9 = assign13230_e7523_d_n9;
        var_mphn0_dn10 = assign13230_e7523_d_n10;
        var_mphn0_dn11 = assign13230_e7523_d_n11;
        var_mphn0_dn14 = assign13230_e7523_d_n14;

        let assign13240_e7530: f64 = if ((var_uc_codep != 0.0) && (var_uc_codep < 3.0)) { 1.0 } else { 0.0 };
        var_guard294 = assign13240_e7530;

        let (assign13250_e7545, assign13250_e7545_d_n0, assign13250_e7545_d_n2, assign13250_e7545_d_n4, assign13250_e7545_d_n5, assign13250_e7545_d_n6, assign13250_e7545_d_n7, assign13250_e7545_d_n8, assign13250_e7545_d_n9, assign13250_e7545_d_n10, assign13250_e7545_d_n11, assign13250_e7545_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard294 != 0.0)) {
        let assign13250_e7536: f64 = (2.0 * 1.034943e-10);
        let assign13250_e7538: f64 = (assign13250_e7536 * 1.6021918e-19);
        let assign13250_e7540: f64 = (assign13250_e7538 * var_uc_ndepm);
        let assign13250_e7542: f64 = (assign13250_e7540 * var_beta_inv);
        let assign13250_e7543: f64 = (assign13250_e7542).sqrt();
        (assign13250_e7543, ((((assign13250_e7538 * var_uc_ndepm_dn0) * var_beta_inv) + (assign13250_e7540 * var_beta_inv_dn0)) / (2.0 * assign13250_e7543)), ((((assign13250_e7538 * var_uc_ndepm_dn2) * var_beta_inv) + (assign13250_e7540 * var_beta_inv_dn2)) / (2.0 * assign13250_e7543)), ((((assign13250_e7538 * var_uc_ndepm_dn4) * var_beta_inv) + (assign13250_e7540 * var_beta_inv_dn4)) / (2.0 * assign13250_e7543)), ((((assign13250_e7538 * var_uc_ndepm_dn5) * var_beta_inv) + (assign13250_e7540 * var_beta_inv_dn5)) / (2.0 * assign13250_e7543)), ((((assign13250_e7538 * var_uc_ndepm_dn6) * var_beta_inv) + (assign13250_e7540 * var_beta_inv_dn6)) / (2.0 * assign13250_e7543)), ((((assign13250_e7538 * var_uc_ndepm_dn7) * var_beta_inv) + (assign13250_e7540 * var_beta_inv_dn7)) / (2.0 * assign13250_e7543)), ((((assign13250_e7538 * var_uc_ndepm_dn8) * var_beta_inv) + (assign13250_e7540 * var_beta_inv_dn8)) / (2.0 * assign13250_e7543)), ((((assign13250_e7538 * var_uc_ndepm_dn9) * var_beta_inv) + (assign13250_e7540 * var_beta_inv_dn9)) / (2.0 * assign13250_e7543)), ((((assign13250_e7538 * var_uc_ndepm_dn10) * var_beta_inv) + (assign13250_e7540 * var_beta_inv_dn10)) / (2.0 * assign13250_e7543)), ((((assign13250_e7538 * var_uc_ndepm_dn11) * var_beta_inv) + (assign13250_e7540 * var_beta_inv_dn11)) / (2.0 * assign13250_e7543)), ((((assign13250_e7538 * var_uc_ndepm_dn14) * var_beta_inv) + (assign13250_e7540 * var_beta_inv_dn14)) / (2.0 * assign13250_e7543)),)
    } else {
        (var_cnst0, var_cnst0_dn0, var_cnst0_dn2, var_cnst0_dn4, var_cnst0_dn5, var_cnst0_dn6, var_cnst0_dn7, var_cnst0_dn8, var_cnst0_dn9, var_cnst0_dn10, var_cnst0_dn11, var_cnst0_dn14,)
    }
};
        var_cnst0 = assign13250_e7545;
        var_cnst0_dn0 = assign13250_e7545_d_n0;
        var_cnst0_dn2 = assign13250_e7545_d_n2;
        var_cnst0_dn4 = assign13250_e7545_d_n4;
        var_cnst0_dn5 = assign13250_e7545_d_n5;
        var_cnst0_dn6 = assign13250_e7545_d_n6;
        var_cnst0_dn7 = assign13250_e7545_d_n7;
        var_cnst0_dn8 = assign13250_e7545_d_n8;
        var_cnst0_dn9 = assign13250_e7545_d_n9;
        var_cnst0_dn10 = assign13250_e7545_d_n10;
        var_cnst0_dn11 = assign13250_e7545_d_n11;
        var_cnst0_dn14 = assign13250_e7545_d_n14;

        let (assign13260_e7557, assign13260_e7557_d_n0, assign13260_e7557_d_n2, assign13260_e7557_d_n4, assign13260_e7557_d_n5, assign13260_e7557_d_n6, assign13260_e7557_d_n7, assign13260_e7557_d_n8, assign13260_e7557_d_n9, assign13260_e7557_d_n10, assign13260_e7557_d_n11, assign13260_e7557_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard294 != 0.0)) {
        let assign13260_e7551: f64 = (var_nin * var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / var_uc_ndepm;
        let assign13260_e7553: f64 = (assign13260_e7551 * __rspice_inv_cse_0);
        let assign13260_e7555: f64 = (assign13260_e7553 * __rspice_inv_cse_0);
        (assign13260_e7555, ((((((((var_nin_dn0 * var_nin) + (var_nin * var_nin_dn0)) * var_uc_ndepm) - (assign13260_e7551 * var_uc_ndepm_dn0)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13260_e7553 * var_uc_ndepm_dn0)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn2 * var_nin) + (var_nin * var_nin_dn2)) * var_uc_ndepm) - (assign13260_e7551 * var_uc_ndepm_dn2)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13260_e7553 * var_uc_ndepm_dn2)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn4 * var_nin) + (var_nin * var_nin_dn4)) * var_uc_ndepm) - (assign13260_e7551 * var_uc_ndepm_dn4)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13260_e7553 * var_uc_ndepm_dn4)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn5 * var_nin) + (var_nin * var_nin_dn5)) * var_uc_ndepm) - (assign13260_e7551 * var_uc_ndepm_dn5)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13260_e7553 * var_uc_ndepm_dn5)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn6 * var_nin) + (var_nin * var_nin_dn6)) * var_uc_ndepm) - (assign13260_e7551 * var_uc_ndepm_dn6)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13260_e7553 * var_uc_ndepm_dn6)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn7 * var_nin) + (var_nin * var_nin_dn7)) * var_uc_ndepm) - (assign13260_e7551 * var_uc_ndepm_dn7)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13260_e7553 * var_uc_ndepm_dn7)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn8 * var_nin) + (var_nin * var_nin_dn8)) * var_uc_ndepm) - (assign13260_e7551 * var_uc_ndepm_dn8)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13260_e7553 * var_uc_ndepm_dn8)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn9 * var_nin) + (var_nin * var_nin_dn9)) * var_uc_ndepm) - (assign13260_e7551 * var_uc_ndepm_dn9)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13260_e7553 * var_uc_ndepm_dn9)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn10 * var_nin) + (var_nin * var_nin_dn10)) * var_uc_ndepm) - (assign13260_e7551 * var_uc_ndepm_dn10)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13260_e7553 * var_uc_ndepm_dn10)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn11 * var_nin) + (var_nin * var_nin_dn11)) * var_uc_ndepm) - (assign13260_e7551 * var_uc_ndepm_dn11)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13260_e7553 * var_uc_ndepm_dn11)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn14 * var_nin) + (var_nin * var_nin_dn14)) * var_uc_ndepm) - (assign13260_e7551 * var_uc_ndepm_dn14)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13260_e7553 * var_uc_ndepm_dn14)) / (var_uc_ndepm * var_uc_ndepm)),)
    } else {
        (var_cnst1, var_cnst1_dn0, var_cnst1_dn2, var_cnst1_dn4, var_cnst1_dn5, var_cnst1_dn6, var_cnst1_dn7, var_cnst1_dn8, var_cnst1_dn9, var_cnst1_dn10, var_cnst1_dn11, var_cnst1_dn14,)
    }
};
        var_cnst1 = assign13260_e7557;
        var_cnst1_dn0 = assign13260_e7557_d_n0;
        var_cnst1_dn2 = assign13260_e7557_d_n2;
        var_cnst1_dn4 = assign13260_e7557_d_n4;
        var_cnst1_dn5 = assign13260_e7557_d_n5;
        var_cnst1_dn6 = assign13260_e7557_d_n6;
        var_cnst1_dn7 = assign13260_e7557_d_n7;
        var_cnst1_dn8 = assign13260_e7557_d_n8;
        var_cnst1_dn9 = assign13260_e7557_d_n9;
        var_cnst1_dn10 = assign13260_e7557_d_n10;
        var_cnst1_dn11 = assign13260_e7557_d_n11;
        var_cnst1_dn14 = assign13260_e7557_d_n14;

        let (assign13270_e7570, assign13270_e7570_d_n0, assign13270_e7570_d_n2, assign13270_e7570_d_n4, assign13270_e7570_d_n5, assign13270_e7570_d_n6, assign13270_e7570_d_n7, assign13270_e7570_d_n8, assign13270_e7570_d_n9, assign13270_e7570_d_n10, assign13270_e7570_d_n11, assign13270_e7570_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard294 != 0.0)) {
        let assign13270_e7563: f64 = (2.0 * var_beta_inv);
        let assign13270_e7566: f64 = (var_uc_ndepm / var_nin);
        let assign13270_e7567: f64 = (assign13270_e7566).ln();
        let assign13270_e7568: f64 = (assign13270_e7563 * assign13270_e7567);
        (assign13270_e7568, (((2.0 * var_beta_inv_dn0) * assign13270_e7567) + (assign13270_e7563 * ((((var_uc_ndepm_dn0 * var_nin) - (var_uc_ndepm * var_nin_dn0)) / (var_nin * var_nin)) / assign13270_e7566))), (((2.0 * var_beta_inv_dn2) * assign13270_e7567) + (assign13270_e7563 * ((((var_uc_ndepm_dn2 * var_nin) - (var_uc_ndepm * var_nin_dn2)) / (var_nin * var_nin)) / assign13270_e7566))), (((2.0 * var_beta_inv_dn4) * assign13270_e7567) + (assign13270_e7563 * ((((var_uc_ndepm_dn4 * var_nin) - (var_uc_ndepm * var_nin_dn4)) / (var_nin * var_nin)) / assign13270_e7566))), (((2.0 * var_beta_inv_dn5) * assign13270_e7567) + (assign13270_e7563 * ((((var_uc_ndepm_dn5 * var_nin) - (var_uc_ndepm * var_nin_dn5)) / (var_nin * var_nin)) / assign13270_e7566))), (((2.0 * var_beta_inv_dn6) * assign13270_e7567) + (assign13270_e7563 * ((((var_uc_ndepm_dn6 * var_nin) - (var_uc_ndepm * var_nin_dn6)) / (var_nin * var_nin)) / assign13270_e7566))), (((2.0 * var_beta_inv_dn7) * assign13270_e7567) + (assign13270_e7563 * ((((var_uc_ndepm_dn7 * var_nin) - (var_uc_ndepm * var_nin_dn7)) / (var_nin * var_nin)) / assign13270_e7566))), (((2.0 * var_beta_inv_dn8) * assign13270_e7567) + (assign13270_e7563 * ((((var_uc_ndepm_dn8 * var_nin) - (var_uc_ndepm * var_nin_dn8)) / (var_nin * var_nin)) / assign13270_e7566))), (((2.0 * var_beta_inv_dn9) * assign13270_e7567) + (assign13270_e7563 * ((((var_uc_ndepm_dn9 * var_nin) - (var_uc_ndepm * var_nin_dn9)) / (var_nin * var_nin)) / assign13270_e7566))), (((2.0 * var_beta_inv_dn10) * assign13270_e7567) + (assign13270_e7563 * ((((var_uc_ndepm_dn10 * var_nin) - (var_uc_ndepm * var_nin_dn10)) / (var_nin * var_nin)) / assign13270_e7566))), (((2.0 * var_beta_inv_dn11) * assign13270_e7567) + (assign13270_e7563 * ((((var_uc_ndepm_dn11 * var_nin) - (var_uc_ndepm * var_nin_dn11)) / (var_nin * var_nin)) / assign13270_e7566))), (((2.0 * var_beta_inv_dn14) * assign13270_e7567) + (assign13270_e7563 * ((((var_uc_ndepm_dn14 * var_nin) - (var_uc_ndepm * var_nin_dn14)) / (var_nin * var_nin)) / assign13270_e7566))),)
    } else {
        (var_pb2n, var_pb2n_dn0, var_pb2n_dn2, var_pb2n_dn4, var_pb2n_dn5, var_pb2n_dn6, var_pb2n_dn7, var_pb2n_dn8, var_pb2n_dn9, var_pb2n_dn10, var_pb2n_dn11, var_pb2n_dn14,)
    }
};
        var_pb2n = assign13270_e7570;
        var_pb2n_dn0 = assign13270_e7570_d_n0;
        var_pb2n_dn2 = assign13270_e7570_d_n2;
        var_pb2n_dn4 = assign13270_e7570_d_n4;
        var_pb2n_dn5 = assign13270_e7570_d_n5;
        var_pb2n_dn6 = assign13270_e7570_d_n6;
        var_pb2n_dn7 = assign13270_e7570_d_n7;
        var_pb2n_dn8 = assign13270_e7570_d_n8;
        var_pb2n_dn9 = assign13270_e7570_d_n9;
        var_pb2n_dn10 = assign13270_e7570_d_n10;
        var_pb2n_dn11 = assign13270_e7570_d_n11;
        var_pb2n_dn14 = assign13270_e7570_d_n14;

        let (assign13280_e7585, assign13280_e7585_d_n0, assign13280_e7585_d_n2, assign13280_e7585_d_n4, assign13280_e7585_d_n5, assign13280_e7585_d_n6, assign13280_e7585_d_n7, assign13280_e7585_d_n8, assign13280_e7585_d_n9, assign13280_e7585_d_n10, assign13280_e7585_d_n11, assign13280_e7585_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard294 != 0.0)) {
        let assign13280_e7577: f64 = (var_uc_ndepm * var_ef_nsubc);
        let __rspice_inv_cse_1: f64 = 1.0 / var_nin;
        let assign13280_e7579: f64 = (assign13280_e7577 * __rspice_inv_cse_1);
        let assign13280_e7581: f64 = (assign13280_e7579 * __rspice_inv_cse_1);
        let assign13280_e7582: f64 = (assign13280_e7581).ln();
        let assign13280_e7583: f64 = (var_beta_inv * assign13280_e7582);
        (assign13280_e7583, ((var_beta_inv_dn0 * assign13280_e7582) + (var_beta_inv * (((((((((var_uc_ndepm_dn0 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn0)) * var_nin) - (assign13280_e7577 * var_nin_dn0)) / (var_nin * var_nin)) * var_nin) - (assign13280_e7579 * var_nin_dn0)) / (var_nin * var_nin)) / assign13280_e7581))), ((var_beta_inv_dn2 * assign13280_e7582) + (var_beta_inv * (((((((((var_uc_ndepm_dn2 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn2)) * var_nin) - (assign13280_e7577 * var_nin_dn2)) / (var_nin * var_nin)) * var_nin) - (assign13280_e7579 * var_nin_dn2)) / (var_nin * var_nin)) / assign13280_e7581))), ((var_beta_inv_dn4 * assign13280_e7582) + (var_beta_inv * (((((((((var_uc_ndepm_dn4 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn4)) * var_nin) - (assign13280_e7577 * var_nin_dn4)) / (var_nin * var_nin)) * var_nin) - (assign13280_e7579 * var_nin_dn4)) / (var_nin * var_nin)) / assign13280_e7581))), ((var_beta_inv_dn5 * assign13280_e7582) + (var_beta_inv * (((((((((var_uc_ndepm_dn5 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn5)) * var_nin) - (assign13280_e7577 * var_nin_dn5)) / (var_nin * var_nin)) * var_nin) - (assign13280_e7579 * var_nin_dn5)) / (var_nin * var_nin)) / assign13280_e7581))), ((var_beta_inv_dn6 * assign13280_e7582) + (var_beta_inv * (((((((((var_uc_ndepm_dn6 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn6)) * var_nin) - (assign13280_e7577 * var_nin_dn6)) / (var_nin * var_nin)) * var_nin) - (assign13280_e7579 * var_nin_dn6)) / (var_nin * var_nin)) / assign13280_e7581))), ((var_beta_inv_dn7 * assign13280_e7582) + (var_beta_inv * (((((((((var_uc_ndepm_dn7 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn7)) * var_nin) - (assign13280_e7577 * var_nin_dn7)) / (var_nin * var_nin)) * var_nin) - (assign13280_e7579 * var_nin_dn7)) / (var_nin * var_nin)) / assign13280_e7581))), ((var_beta_inv_dn8 * assign13280_e7582) + (var_beta_inv * (((((((((var_uc_ndepm_dn8 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn8)) * var_nin) - (assign13280_e7577 * var_nin_dn8)) / (var_nin * var_nin)) * var_nin) - (assign13280_e7579 * var_nin_dn8)) / (var_nin * var_nin)) / assign13280_e7581))), ((var_beta_inv_dn9 * assign13280_e7582) + (var_beta_inv * (((((((((var_uc_ndepm_dn9 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn9)) * var_nin) - (assign13280_e7577 * var_nin_dn9)) / (var_nin * var_nin)) * var_nin) - (assign13280_e7579 * var_nin_dn9)) / (var_nin * var_nin)) / assign13280_e7581))), ((var_beta_inv_dn10 * assign13280_e7582) + (var_beta_inv * (((((((((var_uc_ndepm_dn10 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn10)) * var_nin) - (assign13280_e7577 * var_nin_dn10)) / (var_nin * var_nin)) * var_nin) - (assign13280_e7579 * var_nin_dn10)) / (var_nin * var_nin)) / assign13280_e7581))), ((var_beta_inv_dn11 * assign13280_e7582) + (var_beta_inv * (((((((((var_uc_ndepm_dn11 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn11)) * var_nin) - (assign13280_e7577 * var_nin_dn11)) / (var_nin * var_nin)) * var_nin) - (assign13280_e7579 * var_nin_dn11)) / (var_nin * var_nin)) / assign13280_e7581))), ((var_beta_inv_dn14 * assign13280_e7582) + (var_beta_inv * (((((((((var_uc_ndepm_dn14 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn14)) * var_nin) - (assign13280_e7577 * var_nin_dn14)) / (var_nin * var_nin)) * var_nin) - (assign13280_e7579 * var_nin_dn14)) / (var_nin * var_nin)) / assign13280_e7581))),)
    } else {
        (var_vbipn, var_vbipn_dn0, var_vbipn_dn2, var_vbipn_dn4, var_vbipn_dn5, var_vbipn_dn6, var_vbipn_dn7, var_vbipn_dn8, var_vbipn_dn9, var_vbipn_dn10, var_vbipn_dn11, var_vbipn_dn14,)
    }
};
        var_vbipn = assign13280_e7585;
        var_vbipn_dn0 = assign13280_e7585_d_n0;
        var_vbipn_dn2 = assign13280_e7585_d_n2;
        var_vbipn_dn4 = assign13280_e7585_d_n4;
        var_vbipn_dn5 = assign13280_e7585_d_n5;
        var_vbipn_dn6 = assign13280_e7585_d_n6;
        var_vbipn_dn7 = assign13280_e7585_d_n7;
        var_vbipn_dn8 = assign13280_e7585_d_n8;
        var_vbipn_dn9 = assign13280_e7585_d_n9;
        var_vbipn_dn10 = assign13280_e7585_d_n10;
        var_vbipn_dn11 = assign13280_e7585_d_n11;
        var_vbipn_dn14 = assign13280_e7585_d_n14;

        let (assign13290_e7594, assign13290_e7594_d_n0, assign13290_e7594_d_n2, assign13290_e7594_d_n4, assign13290_e7594_d_n5, assign13290_e7594_d_n6, assign13290_e7594_d_n7, assign13290_e7594_d_n8, assign13290_e7594_d_n9, assign13290_e7594_d_n10, assign13290_e7594_d_n11, assign13290_e7594_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard294 != 0.0)) {
        let assign13290_e7591: f64 = (var_log_tratio * p.p380);
        let assign13290_e7592: f64 = (assign13290_e7591).exp();
        (assign13290_e7592, (assign13290_e7592 * (var_log_tratio_dn0 * p.p380)), (assign13290_e7592 * (var_log_tratio_dn2 * p.p380)), (assign13290_e7592 * (var_log_tratio_dn4 * p.p380)), (assign13290_e7592 * (var_log_tratio_dn5 * p.p380)), (assign13290_e7592 * (var_log_tratio_dn6 * p.p380)), (assign13290_e7592 * (var_log_tratio_dn7 * p.p380)), (assign13290_e7592 * (var_log_tratio_dn8 * p.p380)), (assign13290_e7592 * (var_log_tratio_dn9 * p.p380)), (assign13290_e7592 * (var_log_tratio_dn10 * p.p380)), (assign13290_e7592 * (var_log_tratio_dn11 * p.p380)), (assign13290_e7592 * (var_log_tratio_dn14 * p.p380)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign13290_e7594;
        var_t1_dn0 = assign13290_e7594_d_n0;
        var_t1_dn2 = assign13290_e7594_d_n2;
        var_t1_dn4 = assign13290_e7594_d_n4;
        var_t1_dn5 = assign13290_e7594_d_n5;
        var_t1_dn6 = assign13290_e7594_d_n6;
        var_t1_dn7 = assign13290_e7594_d_n7;
        var_t1_dn8 = assign13290_e7594_d_n8;
        var_t1_dn9 = assign13290_e7594_d_n9;
        var_t1_dn10 = assign13290_e7594_d_n10;
        var_t1_dn11 = assign13290_e7594_d_n11;
        var_t1_dn14 = assign13290_e7594_d_n14;

        *var_beta_slot = var_beta;
        *var_beta2_slot = var_beta2;
        *var_beta2_dn0_slot = var_beta2_dn0;
        *var_beta2_dn10_slot = var_beta2_dn10;
        *var_beta2_dn11_slot = var_beta2_dn11;
        *var_beta2_dn14_slot = var_beta2_dn14;
        *var_beta2_dn2_slot = var_beta2_dn2;
        *var_beta2_dn4_slot = var_beta2_dn4;
        *var_beta2_dn5_slot = var_beta2_dn5;
        *var_beta2_dn6_slot = var_beta2_dn6;
        *var_beta2_dn7_slot = var_beta2_dn7;
        *var_beta2_dn8_slot = var_beta2_dn8;
        *var_beta2_dn9_slot = var_beta2_dn9;
        *var_beta_dn0_slot = var_beta_dn0;
        *var_beta_dn10_slot = var_beta_dn10;
        *var_beta_dn11_slot = var_beta_dn11;
        *var_beta_dn14_slot = var_beta_dn14;
        *var_beta_dn2_slot = var_beta_dn2;
        *var_beta_dn4_slot = var_beta_dn4;
        *var_beta_dn5_slot = var_beta_dn5;
        *var_beta_dn6_slot = var_beta_dn6;
        *var_beta_dn7_slot = var_beta_dn7;
        *var_beta_dn8_slot = var_beta_dn8;
        *var_beta_dn9_slot = var_beta_dn9;
        *var_beta_inv_slot = var_beta_inv;
        *var_beta_inv_dn0_slot = var_beta_inv_dn0;
        *var_beta_inv_dn10_slot = var_beta_inv_dn10;
        *var_beta_inv_dn11_slot = var_beta_inv_dn11;
        *var_beta_inv_dn14_slot = var_beta_inv_dn14;
        *var_beta_inv_dn2_slot = var_beta_inv_dn2;
        *var_beta_inv_dn4_slot = var_beta_inv_dn4;
        *var_beta_inv_dn5_slot = var_beta_inv_dn5;
        *var_beta_inv_dn6_slot = var_beta_inv_dn6;
        *var_beta_inv_dn7_slot = var_beta_inv_dn7;
        *var_beta_inv_dn8_slot = var_beta_inv_dn8;
        *var_beta_inv_dn9_slot = var_beta_inv_dn9;
        *var_betatnom_slot = var_betatnom;
        *var_cnst0_slot = var_cnst0;
        *var_cnst0_dn0_slot = var_cnst0_dn0;
        *var_cnst0_dn10_slot = var_cnst0_dn10;
        *var_cnst0_dn11_slot = var_cnst0_dn11;
        *var_cnst0_dn14_slot = var_cnst0_dn14;
        *var_cnst0_dn2_slot = var_cnst0_dn2;
        *var_cnst0_dn4_slot = var_cnst0_dn4;
        *var_cnst0_dn5_slot = var_cnst0_dn5;
        *var_cnst0_dn6_slot = var_cnst0_dn6;
        *var_cnst0_dn7_slot = var_cnst0_dn7;
        *var_cnst0_dn8_slot = var_cnst0_dn8;
        *var_cnst0_dn9_slot = var_cnst0_dn9;
        *var_cnst1_slot = var_cnst1;
        *var_cnst1_dn0_slot = var_cnst1_dn0;
        *var_cnst1_dn10_slot = var_cnst1_dn10;
        *var_cnst1_dn11_slot = var_cnst1_dn11;
        *var_cnst1_dn14_slot = var_cnst1_dn14;
        *var_cnst1_dn2_slot = var_cnst1_dn2;
        *var_cnst1_dn4_slot = var_cnst1_dn4;
        *var_cnst1_dn5_slot = var_cnst1_dn5;
        *var_cnst1_dn6_slot = var_cnst1_dn6;
        *var_cnst1_dn7_slot = var_cnst1_dn7;
        *var_cnst1_dn8_slot = var_cnst1_dn8;
        *var_cnst1_dn9_slot = var_cnst1_dn9;
        *var_eg_slot = var_eg;
        *var_eg_dn0_slot = var_eg_dn0;
        *var_eg_dn10_slot = var_eg_dn10;
        *var_eg_dn11_slot = var_eg_dn11;
        *var_eg_dn14_slot = var_eg_dn14;
        *var_eg_dn2_slot = var_eg_dn2;
        *var_eg_dn4_slot = var_eg_dn4;
        *var_eg_dn5_slot = var_eg_dn5;
        *var_eg_dn6_slot = var_eg_dn6;
        *var_eg_dn7_slot = var_eg_dn7;
        *var_eg_dn8_slot = var_eg_dn8;
        *var_eg_dn9_slot = var_eg_dn9;
        *var_egp12_slot = var_egp12;
        *var_egp12_dn0_slot = var_egp12_dn0;
        *var_egp12_dn10_slot = var_egp12_dn10;
        *var_egp12_dn11_slot = var_egp12_dn11;
        *var_egp12_dn14_slot = var_egp12_dn14;
        *var_egp12_dn2_slot = var_egp12_dn2;
        *var_egp12_dn4_slot = var_egp12_dn4;
        *var_egp12_dn5_slot = var_egp12_dn5;
        *var_egp12_dn6_slot = var_egp12_dn6;
        *var_egp12_dn7_slot = var_egp12_dn7;
        *var_egp12_dn8_slot = var_egp12_dn8;
        *var_egp12_dn9_slot = var_egp12_dn9;
        *var_egp32_slot = var_egp32;
        *var_egp32_dn0_slot = var_egp32_dn0;
        *var_egp32_dn10_slot = var_egp32_dn10;
        *var_egp32_dn11_slot = var_egp32_dn11;
        *var_egp32_dn14_slot = var_egp32_dn14;
        *var_egp32_dn2_slot = var_egp32_dn2;
        *var_egp32_dn4_slot = var_egp32_dn4;
        *var_egp32_dn5_slot = var_egp32_dn5;
        *var_egp32_dn6_slot = var_egp32_dn6;
        *var_egp32_dn7_slot = var_egp32_dn7;
        *var_egp32_dn8_slot = var_egp32_dn8;
        *var_egp32_dn9_slot = var_egp32_dn9;
        *var_guard294_slot = var_guard294;
        *var_log_tratio_slot = var_log_tratio;
        *var_log_tratio_dn0_slot = var_log_tratio_dn0;
        *var_log_tratio_dn10_slot = var_log_tratio_dn10;
        *var_log_tratio_dn11_slot = var_log_tratio_dn11;
        *var_log_tratio_dn14_slot = var_log_tratio_dn14;
        *var_log_tratio_dn2_slot = var_log_tratio_dn2;
        *var_log_tratio_dn4_slot = var_log_tratio_dn4;
        *var_log_tratio_dn5_slot = var_log_tratio_dn5;
        *var_log_tratio_dn6_slot = var_log_tratio_dn6;
        *var_log_tratio_dn7_slot = var_log_tratio_dn7;
        *var_log_tratio_dn8_slot = var_log_tratio_dn8;
        *var_log_tratio_dn9_slot = var_log_tratio_dn9;
        *var_mphn0_slot = var_mphn0;
        *var_mphn0_dn0_slot = var_mphn0_dn0;
        *var_mphn0_dn10_slot = var_mphn0_dn10;
        *var_mphn0_dn11_slot = var_mphn0_dn11;
        *var_mphn0_dn14_slot = var_mphn0_dn14;
        *var_mphn0_dn2_slot = var_mphn0_dn2;
        *var_mphn0_dn4_slot = var_mphn0_dn4;
        *var_mphn0_dn5_slot = var_mphn0_dn5;
        *var_mphn0_dn6_slot = var_mphn0_dn6;
        *var_mphn0_dn7_slot = var_mphn0_dn7;
        *var_mphn0_dn8_slot = var_mphn0_dn8;
        *var_mphn0_dn9_slot = var_mphn0_dn9;
        *var_nin_slot = var_nin;
        *var_nin_dn0_slot = var_nin_dn0;
        *var_nin_dn10_slot = var_nin_dn10;
        *var_nin_dn11_slot = var_nin_dn11;
        *var_nin_dn14_slot = var_nin_dn14;
        *var_nin_dn2_slot = var_nin_dn2;
        *var_nin_dn4_slot = var_nin_dn4;
        *var_nin_dn5_slot = var_nin_dn5;
        *var_nin_dn6_slot = var_nin_dn6;
        *var_nin_dn7_slot = var_nin_dn7;
        *var_nin_dn8_slot = var_nin_dn8;
        *var_nin_dn9_slot = var_nin_dn9;
        *var_pb2n_slot = var_pb2n;
        *var_pb2n_dn0_slot = var_pb2n_dn0;
        *var_pb2n_dn10_slot = var_pb2n_dn10;
        *var_pb2n_dn11_slot = var_pb2n_dn11;
        *var_pb2n_dn14_slot = var_pb2n_dn14;
        *var_pb2n_dn2_slot = var_pb2n_dn2;
        *var_pb2n_dn4_slot = var_pb2n_dn4;
        *var_pb2n_dn5_slot = var_pb2n_dn5;
        *var_pb2n_dn6_slot = var_pb2n_dn6;
        *var_pb2n_dn7_slot = var_pb2n_dn7;
        *var_pb2n_dn8_slot = var_pb2n_dn8;
        *var_pb2n_dn9_slot = var_pb2n_dn9;
        *var_sqrt_eg_slot = var_sqrt_eg;
        *var_sqrt_eg_dn0_slot = var_sqrt_eg_dn0;
        *var_sqrt_eg_dn10_slot = var_sqrt_eg_dn10;
        *var_sqrt_eg_dn11_slot = var_sqrt_eg_dn11;
        *var_sqrt_eg_dn14_slot = var_sqrt_eg_dn14;
        *var_sqrt_eg_dn2_slot = var_sqrt_eg_dn2;
        *var_sqrt_eg_dn4_slot = var_sqrt_eg_dn4;
        *var_sqrt_eg_dn5_slot = var_sqrt_eg_dn5;
        *var_sqrt_eg_dn6_slot = var_sqrt_eg_dn6;
        *var_sqrt_eg_dn7_slot = var_sqrt_eg_dn7;
        *var_sqrt_eg_dn8_slot = var_sqrt_eg_dn8;
        *var_sqrt_eg_dn9_slot = var_sqrt_eg_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_tdiff_2_slot = var_tdiff_2;
        *var_tdiff_2_dn0_slot = var_tdiff_2_dn0;
        *var_tdiff_2_dn10_slot = var_tdiff_2_dn10;
        *var_tdiff_2_dn11_slot = var_tdiff_2_dn11;
        *var_tdiff_2_dn14_slot = var_tdiff_2_dn14;
        *var_tdiff_2_dn2_slot = var_tdiff_2_dn2;
        *var_tdiff_2_dn4_slot = var_tdiff_2_dn4;
        *var_tdiff_2_dn5_slot = var_tdiff_2_dn5;
        *var_tdiff_2_dn6_slot = var_tdiff_2_dn6;
        *var_tdiff_2_dn7_slot = var_tdiff_2_dn7;
        *var_tdiff_2_dn8_slot = var_tdiff_2_dn8;
        *var_tdiff_2_dn9_slot = var_tdiff_2_dn9;
        *var_tratio_slot = var_tratio;
        *var_tratio_dn0_slot = var_tratio_dn0;
        *var_tratio_dn10_slot = var_tratio_dn10;
        *var_tratio_dn11_slot = var_tratio_dn11;
        *var_tratio_dn14_slot = var_tratio_dn14;
        *var_tratio_dn2_slot = var_tratio_dn2;
        *var_tratio_dn4_slot = var_tratio_dn4;
        *var_tratio_dn5_slot = var_tratio_dn5;
        *var_tratio_dn6_slot = var_tratio_dn6;
        *var_tratio_dn7_slot = var_tratio_dn7;
        *var_tratio_dn8_slot = var_tratio_dn8;
        *var_tratio_dn9_slot = var_tratio_dn9;
        *var_vbipn_slot = var_vbipn;
        *var_vbipn_dn0_slot = var_vbipn_dn0;
        *var_vbipn_dn10_slot = var_vbipn_dn10;
        *var_vbipn_dn11_slot = var_vbipn_dn11;
        *var_vbipn_dn14_slot = var_vbipn_dn14;
        *var_vbipn_dn2_slot = var_vbipn_dn2;
        *var_vbipn_dn4_slot = var_vbipn_dn4;
        *var_vbipn_dn5_slot = var_vbipn_dn5;
        *var_vbipn_dn6_slot = var_vbipn_dn6;
        *var_vbipn_dn7_slot = var_vbipn_dn7;
        *var_vbipn_dn8_slot = var_vbipn_dn8;
        *var_vbipn_dn9_slot = var_vbipn_dn9;
    }

    pub(super) fn stamp_transient_block_23(
        p: &Parameters,
        var_beta_inv: f64,
        var_beta_inv_dn0: f64,
        var_beta_inv_dn10: f64,
        var_beta_inv_dn11: f64,
        var_beta_inv_dn14: f64,
        var_beta_inv_dn2: f64,
        var_beta_inv_dn4: f64,
        var_beta_inv_dn5: f64,
        var_beta_inv_dn6: f64,
        var_beta_inv_dn7: f64,
        var_beta_inv_dn8: f64,
        var_beta_inv_dn9: f64,
        var_ef_nsubc: f64,
        var_ef_nsubc_dn0: f64,
        var_ef_nsubc_dn10: f64,
        var_ef_nsubc_dn11: f64,
        var_ef_nsubc_dn14: f64,
        var_ef_nsubc_dn2: f64,
        var_ef_nsubc_dn4: f64,
        var_ef_nsubc_dn5: f64,
        var_ef_nsubc_dn6: f64,
        var_ef_nsubc_dn7: f64,
        var_ef_nsubc_dn8: f64,
        var_ef_nsubc_dn9: f64,
        var_guard293: f64,
        var_guard294: f64,
        var_ktnom: f64,
        var_log_tratio: f64,
        var_log_tratio_dn0: f64,
        var_log_tratio_dn10: f64,
        var_log_tratio_dn11: f64,
        var_log_tratio_dn14: f64,
        var_log_tratio_dn2: f64,
        var_log_tratio_dn4: f64,
        var_log_tratio_dn5: f64,
        var_log_tratio_dn6: f64,
        var_log_tratio_dn7: f64,
        var_log_tratio_dn8: f64,
        var_log_tratio_dn9: f64,
        var_nin: f64,
        var_nin_dn0: f64,
        var_nin_dn10: f64,
        var_nin_dn11: f64,
        var_nin_dn14: f64,
        var_nin_dn2: f64,
        var_nin_dn4: f64,
        var_nin_dn5: f64,
        var_nin_dn6: f64,
        var_nin_dn7: f64,
        var_nin_dn8: f64,
        var_nin_dn9: f64,
        var_nsub: f64,
        var_nsub_dn0: f64,
        var_nsub_dn10: f64,
        var_nsub_dn11: f64,
        var_nsub_dn14: f64,
        var_nsub_dn2: f64,
        var_nsub_dn4: f64,
        var_nsub_dn5: f64,
        var_nsub_dn6: f64,
        var_nsub_dn7: f64,
        var_nsub_dn8: f64,
        var_nsub_dn9: f64,
        var_ptovr0: f64,
        var_ptovr0_dn0: f64,
        var_ptovr0_dn10: f64,
        var_ptovr0_dn11: f64,
        var_ptovr0_dn14: f64,
        var_ptovr0_dn2: f64,
        var_ptovr0_dn4: f64,
        var_ptovr0_dn5: f64,
        var_ptovr0_dn6: f64,
        var_ptovr0_dn7: f64,
        var_ptovr0_dn8: f64,
        var_ptovr0_dn9: f64,
        var_tratio: f64,
        var_tratio_dn0: f64,
        var_tratio_dn10: f64,
        var_tratio_dn11: f64,
        var_tratio_dn14: f64,
        var_tratio_dn2: f64,
        var_tratio_dn4: f64,
        var_tratio_dn5: f64,
        var_tratio_dn6: f64,
        var_tratio_dn7: f64,
        var_tratio_dn8: f64,
        var_tratio_dn9: f64,
        var_ttemp: f64,
        var_ttemp_dn0: f64,
        var_ttemp_dn10: f64,
        var_ttemp_dn11: f64,
        var_ttemp_dn14: f64,
        var_ttemp_dn2: f64,
        var_ttemp_dn4: f64,
        var_ttemp_dn5: f64,
        var_ttemp_dn6: f64,
        var_ttemp_dn7: f64,
        var_ttemp_dn8: f64,
        var_ttemp_dn9: f64,
        var_uc_codep: f64,
        var_uc_depmueph1: f64,
        var_uc_ndepm: f64,
        var_uc_ndepm_dn0: f64,
        var_uc_ndepm_dn10: f64,
        var_uc_ndepm_dn11: f64,
        var_uc_ndepm_dn14: f64,
        var_uc_ndepm_dn2: f64,
        var_uc_ndepm_dn4: f64,
        var_uc_ndepm_dn5: f64,
        var_uc_ndepm_dn6: f64,
        var_uc_ndepm_dn7: f64,
        var_uc_ndepm_dn8: f64,
        var_uc_ndepm_dn9: f64,
        var_uc_njunc: f64,
        var_cnst0_slot: &mut f64,
        var_cnst0_dn0_slot: &mut f64,
        var_cnst0_dn10_slot: &mut f64,
        var_cnst0_dn11_slot: &mut f64,
        var_cnst0_dn14_slot: &mut f64,
        var_cnst0_dn2_slot: &mut f64,
        var_cnst0_dn4_slot: &mut f64,
        var_cnst0_dn5_slot: &mut f64,
        var_cnst0_dn6_slot: &mut f64,
        var_cnst0_dn7_slot: &mut f64,
        var_cnst0_dn8_slot: &mut f64,
        var_cnst0_dn9_slot: &mut f64,
        var_cnst1_slot: &mut f64,
        var_cnst1_dn0_slot: &mut f64,
        var_cnst1_dn10_slot: &mut f64,
        var_cnst1_dn11_slot: &mut f64,
        var_cnst1_dn14_slot: &mut f64,
        var_cnst1_dn2_slot: &mut f64,
        var_cnst1_dn4_slot: &mut f64,
        var_cnst1_dn5_slot: &mut f64,
        var_cnst1_dn6_slot: &mut f64,
        var_cnst1_dn7_slot: &mut f64,
        var_cnst1_dn8_slot: &mut f64,
        var_cnst1_dn9_slot: &mut f64,
        var_depmphn0_slot: &mut f64,
        var_depmphn0_dn0_slot: &mut f64,
        var_depmphn0_dn10_slot: &mut f64,
        var_depmphn0_dn11_slot: &mut f64,
        var_depmphn0_dn14_slot: &mut f64,
        var_depmphn0_dn2_slot: &mut f64,
        var_depmphn0_dn4_slot: &mut f64,
        var_depmphn0_dn5_slot: &mut f64,
        var_depmphn0_dn6_slot: &mut f64,
        var_depmphn0_dn7_slot: &mut f64,
        var_depmphn0_dn8_slot: &mut f64,
        var_depmphn0_dn9_slot: &mut f64,
        var_guard296_slot: &mut f64,
        var_guard297_slot: &mut f64,
        var_guard299_slot: &mut f64,
        var_pb2n_slot: &mut f64,
        var_pb2n_dn0_slot: &mut f64,
        var_pb2n_dn10_slot: &mut f64,
        var_pb2n_dn11_slot: &mut f64,
        var_pb2n_dn14_slot: &mut f64,
        var_pb2n_dn2_slot: &mut f64,
        var_pb2n_dn4_slot: &mut f64,
        var_pb2n_dn5_slot: &mut f64,
        var_pb2n_dn6_slot: &mut f64,
        var_pb2n_dn7_slot: &mut f64,
        var_pb2n_dn8_slot: &mut f64,
        var_pb2n_dn9_slot: &mut f64,
        var_ptovr_slot: &mut f64,
        var_ptovr_dn0_slot: &mut f64,
        var_ptovr_dn10_slot: &mut f64,
        var_ptovr_dn11_slot: &mut f64,
        var_ptovr_dn14_slot: &mut f64,
        var_ptovr_dn2_slot: &mut f64,
        var_ptovr_dn4_slot: &mut f64,
        var_ptovr_dn5_slot: &mut f64,
        var_ptovr_dn6_slot: &mut f64,
        var_ptovr_dn7_slot: &mut f64,
        var_ptovr_dn8_slot: &mut f64,
        var_ptovr_dn9_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_uc_depmue0_slot: &mut f64,
        var_uc_depmue0_dn0_slot: &mut f64,
        var_uc_depmue0_dn10_slot: &mut f64,
        var_uc_depmue0_dn11_slot: &mut f64,
        var_uc_depmue0_dn14_slot: &mut f64,
        var_uc_depmue0_dn2_slot: &mut f64,
        var_uc_depmue0_dn4_slot: &mut f64,
        var_uc_depmue0_dn5_slot: &mut f64,
        var_uc_depmue0_dn6_slot: &mut f64,
        var_uc_depmue0_dn7_slot: &mut f64,
        var_uc_depmue0_dn8_slot: &mut f64,
        var_uc_depmue0_dn9_slot: &mut f64,
        var_uc_depmue2_slot: &mut f64,
        var_uc_depmue2_dn0_slot: &mut f64,
        var_uc_depmue2_dn10_slot: &mut f64,
        var_uc_depmue2_dn11_slot: &mut f64,
        var_uc_depmue2_dn14_slot: &mut f64,
        var_uc_depmue2_dn2_slot: &mut f64,
        var_uc_depmue2_dn4_slot: &mut f64,
        var_uc_depmue2_dn5_slot: &mut f64,
        var_uc_depmue2_dn6_slot: &mut f64,
        var_uc_depmue2_dn7_slot: &mut f64,
        var_uc_depmue2_dn8_slot: &mut f64,
        var_uc_depmue2_dn9_slot: &mut f64,
        var_uc_depvmax_slot: &mut f64,
        var_uc_depvmax_dn0_slot: &mut f64,
        var_uc_depvmax_dn10_slot: &mut f64,
        var_uc_depvmax_dn11_slot: &mut f64,
        var_uc_depvmax_dn14_slot: &mut f64,
        var_uc_depvmax_dn2_slot: &mut f64,
        var_uc_depvmax_dn4_slot: &mut f64,
        var_uc_depvmax_dn5_slot: &mut f64,
        var_uc_depvmax_dn6_slot: &mut f64,
        var_uc_depvmax_dn7_slot: &mut f64,
        var_uc_depvmax_dn8_slot: &mut f64,
        var_uc_depvmax_dn9_slot: &mut f64,
        var_uc_depwlp_slot: &mut f64,
        var_uc_depwlp_dn0_slot: &mut f64,
        var_uc_depwlp_dn10_slot: &mut f64,
        var_uc_depwlp_dn11_slot: &mut f64,
        var_uc_depwlp_dn14_slot: &mut f64,
        var_uc_depwlp_dn2_slot: &mut f64,
        var_uc_depwlp_dn4_slot: &mut f64,
        var_uc_depwlp_dn5_slot: &mut f64,
        var_uc_depwlp_dn6_slot: &mut f64,
        var_uc_depwlp_dn7_slot: &mut f64,
        var_uc_depwlp_dn8_slot: &mut f64,
        var_uc_depwlp_dn9_slot: &mut f64,
        var_vbipn_slot: &mut f64,
        var_vbipn_dn0_slot: &mut f64,
        var_vbipn_dn10_slot: &mut f64,
        var_vbipn_dn11_slot: &mut f64,
        var_vbipn_dn14_slot: &mut f64,
        var_vbipn_dn2_slot: &mut f64,
        var_vbipn_dn4_slot: &mut f64,
        var_vbipn_dn5_slot: &mut f64,
        var_vbipn_dn6_slot: &mut f64,
        var_vbipn_dn7_slot: &mut f64,
        var_vbipn_dn8_slot: &mut f64,
        var_vbipn_dn9_slot: &mut f64,
    ) {
        let mut var_cnst0: f64 = *var_cnst0_slot;
        let mut var_cnst0_dn0: f64 = *var_cnst0_dn0_slot;
        let mut var_cnst0_dn10: f64 = *var_cnst0_dn10_slot;
        let mut var_cnst0_dn11: f64 = *var_cnst0_dn11_slot;
        let mut var_cnst0_dn14: f64 = *var_cnst0_dn14_slot;
        let mut var_cnst0_dn2: f64 = *var_cnst0_dn2_slot;
        let mut var_cnst0_dn4: f64 = *var_cnst0_dn4_slot;
        let mut var_cnst0_dn5: f64 = *var_cnst0_dn5_slot;
        let mut var_cnst0_dn6: f64 = *var_cnst0_dn6_slot;
        let mut var_cnst0_dn7: f64 = *var_cnst0_dn7_slot;
        let mut var_cnst0_dn8: f64 = *var_cnst0_dn8_slot;
        let mut var_cnst0_dn9: f64 = *var_cnst0_dn9_slot;
        let mut var_cnst1: f64 = *var_cnst1_slot;
        let mut var_cnst1_dn0: f64 = *var_cnst1_dn0_slot;
        let mut var_cnst1_dn10: f64 = *var_cnst1_dn10_slot;
        let mut var_cnst1_dn11: f64 = *var_cnst1_dn11_slot;
        let mut var_cnst1_dn14: f64 = *var_cnst1_dn14_slot;
        let mut var_cnst1_dn2: f64 = *var_cnst1_dn2_slot;
        let mut var_cnst1_dn4: f64 = *var_cnst1_dn4_slot;
        let mut var_cnst1_dn5: f64 = *var_cnst1_dn5_slot;
        let mut var_cnst1_dn6: f64 = *var_cnst1_dn6_slot;
        let mut var_cnst1_dn7: f64 = *var_cnst1_dn7_slot;
        let mut var_cnst1_dn8: f64 = *var_cnst1_dn8_slot;
        let mut var_cnst1_dn9: f64 = *var_cnst1_dn9_slot;
        let mut var_depmphn0: f64 = *var_depmphn0_slot;
        let mut var_depmphn0_dn0: f64 = *var_depmphn0_dn0_slot;
        let mut var_depmphn0_dn10: f64 = *var_depmphn0_dn10_slot;
        let mut var_depmphn0_dn11: f64 = *var_depmphn0_dn11_slot;
        let mut var_depmphn0_dn14: f64 = *var_depmphn0_dn14_slot;
        let mut var_depmphn0_dn2: f64 = *var_depmphn0_dn2_slot;
        let mut var_depmphn0_dn4: f64 = *var_depmphn0_dn4_slot;
        let mut var_depmphn0_dn5: f64 = *var_depmphn0_dn5_slot;
        let mut var_depmphn0_dn6: f64 = *var_depmphn0_dn6_slot;
        let mut var_depmphn0_dn7: f64 = *var_depmphn0_dn7_slot;
        let mut var_depmphn0_dn8: f64 = *var_depmphn0_dn8_slot;
        let mut var_depmphn0_dn9: f64 = *var_depmphn0_dn9_slot;
        let mut var_guard296: f64 = *var_guard296_slot;
        let mut var_guard297: f64 = *var_guard297_slot;
        let mut var_guard299: f64 = *var_guard299_slot;
        let mut var_pb2n: f64 = *var_pb2n_slot;
        let mut var_pb2n_dn0: f64 = *var_pb2n_dn0_slot;
        let mut var_pb2n_dn10: f64 = *var_pb2n_dn10_slot;
        let mut var_pb2n_dn11: f64 = *var_pb2n_dn11_slot;
        let mut var_pb2n_dn14: f64 = *var_pb2n_dn14_slot;
        let mut var_pb2n_dn2: f64 = *var_pb2n_dn2_slot;
        let mut var_pb2n_dn4: f64 = *var_pb2n_dn4_slot;
        let mut var_pb2n_dn5: f64 = *var_pb2n_dn5_slot;
        let mut var_pb2n_dn6: f64 = *var_pb2n_dn6_slot;
        let mut var_pb2n_dn7: f64 = *var_pb2n_dn7_slot;
        let mut var_pb2n_dn8: f64 = *var_pb2n_dn8_slot;
        let mut var_pb2n_dn9: f64 = *var_pb2n_dn9_slot;
        let mut var_ptovr: f64 = *var_ptovr_slot;
        let mut var_ptovr_dn0: f64 = *var_ptovr_dn0_slot;
        let mut var_ptovr_dn10: f64 = *var_ptovr_dn10_slot;
        let mut var_ptovr_dn11: f64 = *var_ptovr_dn11_slot;
        let mut var_ptovr_dn14: f64 = *var_ptovr_dn14_slot;
        let mut var_ptovr_dn2: f64 = *var_ptovr_dn2_slot;
        let mut var_ptovr_dn4: f64 = *var_ptovr_dn4_slot;
        let mut var_ptovr_dn5: f64 = *var_ptovr_dn5_slot;
        let mut var_ptovr_dn6: f64 = *var_ptovr_dn6_slot;
        let mut var_ptovr_dn7: f64 = *var_ptovr_dn7_slot;
        let mut var_ptovr_dn8: f64 = *var_ptovr_dn8_slot;
        let mut var_ptovr_dn9: f64 = *var_ptovr_dn9_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_uc_depmue0: f64 = *var_uc_depmue0_slot;
        let mut var_uc_depmue0_dn0: f64 = *var_uc_depmue0_dn0_slot;
        let mut var_uc_depmue0_dn10: f64 = *var_uc_depmue0_dn10_slot;
        let mut var_uc_depmue0_dn11: f64 = *var_uc_depmue0_dn11_slot;
        let mut var_uc_depmue0_dn14: f64 = *var_uc_depmue0_dn14_slot;
        let mut var_uc_depmue0_dn2: f64 = *var_uc_depmue0_dn2_slot;
        let mut var_uc_depmue0_dn4: f64 = *var_uc_depmue0_dn4_slot;
        let mut var_uc_depmue0_dn5: f64 = *var_uc_depmue0_dn5_slot;
        let mut var_uc_depmue0_dn6: f64 = *var_uc_depmue0_dn6_slot;
        let mut var_uc_depmue0_dn7: f64 = *var_uc_depmue0_dn7_slot;
        let mut var_uc_depmue0_dn8: f64 = *var_uc_depmue0_dn8_slot;
        let mut var_uc_depmue0_dn9: f64 = *var_uc_depmue0_dn9_slot;
        let mut var_uc_depmue2: f64 = *var_uc_depmue2_slot;
        let mut var_uc_depmue2_dn0: f64 = *var_uc_depmue2_dn0_slot;
        let mut var_uc_depmue2_dn10: f64 = *var_uc_depmue2_dn10_slot;
        let mut var_uc_depmue2_dn11: f64 = *var_uc_depmue2_dn11_slot;
        let mut var_uc_depmue2_dn14: f64 = *var_uc_depmue2_dn14_slot;
        let mut var_uc_depmue2_dn2: f64 = *var_uc_depmue2_dn2_slot;
        let mut var_uc_depmue2_dn4: f64 = *var_uc_depmue2_dn4_slot;
        let mut var_uc_depmue2_dn5: f64 = *var_uc_depmue2_dn5_slot;
        let mut var_uc_depmue2_dn6: f64 = *var_uc_depmue2_dn6_slot;
        let mut var_uc_depmue2_dn7: f64 = *var_uc_depmue2_dn7_slot;
        let mut var_uc_depmue2_dn8: f64 = *var_uc_depmue2_dn8_slot;
        let mut var_uc_depmue2_dn9: f64 = *var_uc_depmue2_dn9_slot;
        let mut var_uc_depvmax: f64 = *var_uc_depvmax_slot;
        let mut var_uc_depvmax_dn0: f64 = *var_uc_depvmax_dn0_slot;
        let mut var_uc_depvmax_dn10: f64 = *var_uc_depvmax_dn10_slot;
        let mut var_uc_depvmax_dn11: f64 = *var_uc_depvmax_dn11_slot;
        let mut var_uc_depvmax_dn14: f64 = *var_uc_depvmax_dn14_slot;
        let mut var_uc_depvmax_dn2: f64 = *var_uc_depvmax_dn2_slot;
        let mut var_uc_depvmax_dn4: f64 = *var_uc_depvmax_dn4_slot;
        let mut var_uc_depvmax_dn5: f64 = *var_uc_depvmax_dn5_slot;
        let mut var_uc_depvmax_dn6: f64 = *var_uc_depvmax_dn6_slot;
        let mut var_uc_depvmax_dn7: f64 = *var_uc_depvmax_dn7_slot;
        let mut var_uc_depvmax_dn8: f64 = *var_uc_depvmax_dn8_slot;
        let mut var_uc_depvmax_dn9: f64 = *var_uc_depvmax_dn9_slot;
        let mut var_uc_depwlp: f64 = *var_uc_depwlp_slot;
        let mut var_uc_depwlp_dn0: f64 = *var_uc_depwlp_dn0_slot;
        let mut var_uc_depwlp_dn10: f64 = *var_uc_depwlp_dn10_slot;
        let mut var_uc_depwlp_dn11: f64 = *var_uc_depwlp_dn11_slot;
        let mut var_uc_depwlp_dn14: f64 = *var_uc_depwlp_dn14_slot;
        let mut var_uc_depwlp_dn2: f64 = *var_uc_depwlp_dn2_slot;
        let mut var_uc_depwlp_dn4: f64 = *var_uc_depwlp_dn4_slot;
        let mut var_uc_depwlp_dn5: f64 = *var_uc_depwlp_dn5_slot;
        let mut var_uc_depwlp_dn6: f64 = *var_uc_depwlp_dn6_slot;
        let mut var_uc_depwlp_dn7: f64 = *var_uc_depwlp_dn7_slot;
        let mut var_uc_depwlp_dn8: f64 = *var_uc_depwlp_dn8_slot;
        let mut var_uc_depwlp_dn9: f64 = *var_uc_depwlp_dn9_slot;
        let mut var_vbipn: f64 = *var_vbipn_slot;
        let mut var_vbipn_dn0: f64 = *var_vbipn_dn0_slot;
        let mut var_vbipn_dn10: f64 = *var_vbipn_dn10_slot;
        let mut var_vbipn_dn11: f64 = *var_vbipn_dn11_slot;
        let mut var_vbipn_dn14: f64 = *var_vbipn_dn14_slot;
        let mut var_vbipn_dn2: f64 = *var_vbipn_dn2_slot;
        let mut var_vbipn_dn4: f64 = *var_vbipn_dn4_slot;
        let mut var_vbipn_dn5: f64 = *var_vbipn_dn5_slot;
        let mut var_vbipn_dn6: f64 = *var_vbipn_dn6_slot;
        let mut var_vbipn_dn7: f64 = *var_vbipn_dn7_slot;
        let mut var_vbipn_dn8: f64 = *var_vbipn_dn8_slot;
        let mut var_vbipn_dn9: f64 = *var_vbipn_dn9_slot;

        let (assign13300_e7602, assign13300_e7602_d_n0, assign13300_e7602_d_n2, assign13300_e7602_d_n4, assign13300_e7602_d_n5, assign13300_e7602_d_n6, assign13300_e7602_d_n7, assign13300_e7602_d_n8, assign13300_e7602_d_n9, assign13300_e7602_d_n10, assign13300_e7602_d_n11, assign13300_e7602_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard294 != 0.0)) {
        let assign13300_e7600: f64 = (var_t1 / var_uc_depmueph1);
        (assign13300_e7600, (var_t1_dn0 / var_uc_depmueph1), (var_t1_dn2 / var_uc_depmueph1), (var_t1_dn4 / var_uc_depmueph1), (var_t1_dn5 / var_uc_depmueph1), (var_t1_dn6 / var_uc_depmueph1), (var_t1_dn7 / var_uc_depmueph1), (var_t1_dn8 / var_uc_depmueph1), (var_t1_dn9 / var_uc_depmueph1), (var_t1_dn10 / var_uc_depmueph1), (var_t1_dn11 / var_uc_depmueph1), (var_t1_dn14 / var_uc_depmueph1),)
    } else {
        (var_depmphn0, var_depmphn0_dn0, var_depmphn0_dn2, var_depmphn0_dn4, var_depmphn0_dn5, var_depmphn0_dn6, var_depmphn0_dn7, var_depmphn0_dn8, var_depmphn0_dn9, var_depmphn0_dn10, var_depmphn0_dn11, var_depmphn0_dn14,)
    }
};
        var_depmphn0 = assign13300_e7602;
        var_depmphn0_dn0 = assign13300_e7602_d_n0;
        var_depmphn0_dn2 = assign13300_e7602_d_n2;
        var_depmphn0_dn4 = assign13300_e7602_d_n4;
        var_depmphn0_dn5 = assign13300_e7602_d_n5;
        var_depmphn0_dn6 = assign13300_e7602_d_n6;
        var_depmphn0_dn7 = assign13300_e7602_d_n7;
        var_depmphn0_dn8 = assign13300_e7602_d_n8;
        var_depmphn0_dn9 = assign13300_e7602_d_n9;
        var_depmphn0_dn10 = assign13300_e7602_d_n10;
        var_depmphn0_dn11 = assign13300_e7602_d_n11;
        var_depmphn0_dn14 = assign13300_e7602_d_n14;

        let (assign13310_e7624, assign13310_e7624_d_n0, assign13310_e7624_d_n2, assign13310_e7624_d_n4, assign13310_e7624_d_n5, assign13310_e7624_d_n6, assign13310_e7624_d_n7, assign13310_e7624_d_n8, assign13310_e7624_d_n9, assign13310_e7624_d_n10, assign13310_e7624_d_n11, assign13310_e7624_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard294 != 0.0)) {
        let assign13310_e7609: f64 = (0.4 * var_tratio);
        let assign13310_e7610: f64 = (1.8 + assign13310_e7609);
        let assign13310_e7613: f64 = (0.1 * var_tratio);
        let assign13310_e7615: f64 = (assign13310_e7613 * var_tratio);
        let assign13310_e7616: f64 = (assign13310_e7610 + assign13310_e7615);
        let assign13310_e7620: f64 = (1.0 - var_tratio);
        let assign13310_e7621: f64 = (p.p379 * assign13310_e7620);
        let assign13310_e7622: f64 = (assign13310_e7616 - assign13310_e7621);
        (assign13310_e7622, (((0.4 * var_tratio_dn0) + (((0.1 * var_tratio_dn0) * var_tratio) + (assign13310_e7613 * var_tratio_dn0))) - (p.p379 * (-var_tratio_dn0))), (((0.4 * var_tratio_dn2) + (((0.1 * var_tratio_dn2) * var_tratio) + (assign13310_e7613 * var_tratio_dn2))) - (p.p379 * (-var_tratio_dn2))), (((0.4 * var_tratio_dn4) + (((0.1 * var_tratio_dn4) * var_tratio) + (assign13310_e7613 * var_tratio_dn4))) - (p.p379 * (-var_tratio_dn4))), (((0.4 * var_tratio_dn5) + (((0.1 * var_tratio_dn5) * var_tratio) + (assign13310_e7613 * var_tratio_dn5))) - (p.p379 * (-var_tratio_dn5))), (((0.4 * var_tratio_dn6) + (((0.1 * var_tratio_dn6) * var_tratio) + (assign13310_e7613 * var_tratio_dn6))) - (p.p379 * (-var_tratio_dn6))), (((0.4 * var_tratio_dn7) + (((0.1 * var_tratio_dn7) * var_tratio) + (assign13310_e7613 * var_tratio_dn7))) - (p.p379 * (-var_tratio_dn7))), (((0.4 * var_tratio_dn8) + (((0.1 * var_tratio_dn8) * var_tratio) + (assign13310_e7613 * var_tratio_dn8))) - (p.p379 * (-var_tratio_dn8))), (((0.4 * var_tratio_dn9) + (((0.1 * var_tratio_dn9) * var_tratio) + (assign13310_e7613 * var_tratio_dn9))) - (p.p379 * (-var_tratio_dn9))), (((0.4 * var_tratio_dn10) + (((0.1 * var_tratio_dn10) * var_tratio) + (assign13310_e7613 * var_tratio_dn10))) - (p.p379 * (-var_tratio_dn10))), (((0.4 * var_tratio_dn11) + (((0.1 * var_tratio_dn11) * var_tratio) + (assign13310_e7613 * var_tratio_dn11))) - (p.p379 * (-var_tratio_dn11))), (((0.4 * var_tratio_dn14) + (((0.1 * var_tratio_dn14) * var_tratio) + (assign13310_e7613 * var_tratio_dn14))) - (p.p379 * (-var_tratio_dn14))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign13310_e7624;
        var_t0_dn0 = assign13310_e7624_d_n0;
        var_t0_dn2 = assign13310_e7624_d_n2;
        var_t0_dn4 = assign13310_e7624_d_n4;
        var_t0_dn5 = assign13310_e7624_d_n5;
        var_t0_dn6 = assign13310_e7624_d_n6;
        var_t0_dn7 = assign13310_e7624_d_n7;
        var_t0_dn8 = assign13310_e7624_d_n8;
        var_t0_dn9 = assign13310_e7624_d_n9;
        var_t0_dn10 = assign13310_e7624_d_n10;
        var_t0_dn11 = assign13310_e7624_d_n11;
        var_t0_dn14 = assign13310_e7624_d_n14;

        let (assign13320_e7632, assign13320_e7632_d_n0, assign13320_e7632_d_n2, assign13320_e7632_d_n4, assign13320_e7632_d_n5, assign13320_e7632_d_n6, assign13320_e7632_d_n7, assign13320_e7632_d_n8, assign13320_e7632_d_n9, assign13320_e7632_d_n10, assign13320_e7632_d_n11, assign13320_e7632_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard294 != 0.0)) {
        let assign13320_e7630: f64 = (var_uc_depvmax / var_t0);
        (assign13320_e7630, (((var_uc_depvmax_dn0 * var_t0) - (var_uc_depvmax * var_t0_dn0)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn2 * var_t0) - (var_uc_depvmax * var_t0_dn2)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn4 * var_t0) - (var_uc_depvmax * var_t0_dn4)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn5 * var_t0) - (var_uc_depvmax * var_t0_dn5)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn6 * var_t0) - (var_uc_depvmax * var_t0_dn6)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn7 * var_t0) - (var_uc_depvmax * var_t0_dn7)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn8 * var_t0) - (var_uc_depvmax * var_t0_dn8)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn9 * var_t0) - (var_uc_depvmax * var_t0_dn9)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn10 * var_t0) - (var_uc_depvmax * var_t0_dn10)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn11 * var_t0) - (var_uc_depvmax * var_t0_dn11)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn14 * var_t0) - (var_uc_depvmax * var_t0_dn14)) / (var_t0 * var_t0)),)
    } else {
        (var_uc_depvmax, var_uc_depvmax_dn0, var_uc_depvmax_dn2, var_uc_depvmax_dn4, var_uc_depvmax_dn5, var_uc_depvmax_dn6, var_uc_depvmax_dn7, var_uc_depvmax_dn8, var_uc_depvmax_dn9, var_uc_depvmax_dn10, var_uc_depvmax_dn11, var_uc_depvmax_dn14,)
    }
};
        var_uc_depvmax = assign13320_e7632;
        var_uc_depvmax_dn0 = assign13320_e7632_d_n0;
        var_uc_depvmax_dn2 = assign13320_e7632_d_n2;
        var_uc_depvmax_dn4 = assign13320_e7632_d_n4;
        var_uc_depvmax_dn5 = assign13320_e7632_d_n5;
        var_uc_depvmax_dn6 = assign13320_e7632_d_n6;
        var_uc_depvmax_dn7 = assign13320_e7632_d_n7;
        var_uc_depvmax_dn8 = assign13320_e7632_d_n8;
        var_uc_depvmax_dn9 = assign13320_e7632_d_n9;
        var_uc_depvmax_dn10 = assign13320_e7632_d_n10;
        var_uc_depvmax_dn11 = assign13320_e7632_d_n11;
        var_uc_depvmax_dn14 = assign13320_e7632_d_n14;

        let assign13340_e7640: f64 = if var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        var_guard296 = assign13340_e7640;

        let (assign13350_e7648, assign13350_e7648_d_n0, assign13350_e7648_d_n2, assign13350_e7648_d_n4, assign13350_e7648_d_n5, assign13350_e7648_d_n6, assign13350_e7648_d_n7, assign13350_e7648_d_n8, assign13350_e7648_d_n9, assign13350_e7648_d_n10, assign13350_e7648_d_n11, assign13350_e7648_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard294 != 0.0)) && (var_guard296 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depvmax, var_uc_depvmax_dn0, var_uc_depvmax_dn2, var_uc_depvmax_dn4, var_uc_depvmax_dn5, var_uc_depvmax_dn6, var_uc_depvmax_dn7, var_uc_depvmax_dn8, var_uc_depvmax_dn9, var_uc_depvmax_dn10, var_uc_depvmax_dn11, var_uc_depvmax_dn14,)
    }
};
        var_uc_depvmax = assign13350_e7648;
        var_uc_depvmax_dn0 = assign13350_e7648_d_n0;
        var_uc_depvmax_dn2 = assign13350_e7648_d_n2;
        var_uc_depvmax_dn4 = assign13350_e7648_d_n4;
        var_uc_depvmax_dn5 = assign13350_e7648_d_n5;
        var_uc_depvmax_dn6 = assign13350_e7648_d_n6;
        var_uc_depvmax_dn7 = assign13350_e7648_d_n7;
        var_uc_depvmax_dn8 = assign13350_e7648_d_n8;
        var_uc_depvmax_dn9 = assign13350_e7648_d_n9;
        var_uc_depvmax_dn10 = assign13350_e7648_d_n10;
        var_uc_depvmax_dn11 = assign13350_e7648_d_n11;
        var_uc_depvmax_dn14 = assign13350_e7648_d_n14;

        let (assign13360_e7658, assign13360_e7658_d_n0, assign13360_e7658_d_n2, assign13360_e7658_d_n4, assign13360_e7658_d_n5, assign13360_e7658_d_n6, assign13360_e7658_d_n7, assign13360_e7658_d_n8, assign13360_e7658_d_n9, assign13360_e7658_d_n10, assign13360_e7658_d_n11, assign13360_e7658_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard294 != 0.0)) {
        let assign13360_e7655: f64 = (var_tratio).powf(p.p381);
        let assign13360_e7656: f64 = (var_uc_depmue0 / assign13360_e7655);
        (assign13360_e7656, (((var_uc_depmue0_dn0 * assign13360_e7655) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn0)) } } else { (assign13360_e7655 * (p.p381 * (var_tratio_dn0 / var_tratio))) })) / (assign13360_e7655 * assign13360_e7655)), (((var_uc_depmue0_dn2 * assign13360_e7655) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn2)) } } else { (assign13360_e7655 * (p.p381 * (var_tratio_dn2 / var_tratio))) })) / (assign13360_e7655 * assign13360_e7655)), (((var_uc_depmue0_dn4 * assign13360_e7655) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn4)) } } else { (assign13360_e7655 * (p.p381 * (var_tratio_dn4 / var_tratio))) })) / (assign13360_e7655 * assign13360_e7655)), (((var_uc_depmue0_dn5 * assign13360_e7655) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn5)) } } else { (assign13360_e7655 * (p.p381 * (var_tratio_dn5 / var_tratio))) })) / (assign13360_e7655 * assign13360_e7655)), (((var_uc_depmue0_dn6 * assign13360_e7655) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn6)) } } else { (assign13360_e7655 * (p.p381 * (var_tratio_dn6 / var_tratio))) })) / (assign13360_e7655 * assign13360_e7655)), (((var_uc_depmue0_dn7 * assign13360_e7655) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn7)) } } else { (assign13360_e7655 * (p.p381 * (var_tratio_dn7 / var_tratio))) })) / (assign13360_e7655 * assign13360_e7655)), (((var_uc_depmue0_dn8 * assign13360_e7655) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn8)) } } else { (assign13360_e7655 * (p.p381 * (var_tratio_dn8 / var_tratio))) })) / (assign13360_e7655 * assign13360_e7655)), (((var_uc_depmue0_dn9 * assign13360_e7655) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn9)) } } else { (assign13360_e7655 * (p.p381 * (var_tratio_dn9 / var_tratio))) })) / (assign13360_e7655 * assign13360_e7655)), (((var_uc_depmue0_dn10 * assign13360_e7655) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn10)) } } else { (assign13360_e7655 * (p.p381 * (var_tratio_dn10 / var_tratio))) })) / (assign13360_e7655 * assign13360_e7655)), (((var_uc_depmue0_dn11 * assign13360_e7655) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn11)) } } else { (assign13360_e7655 * (p.p381 * (var_tratio_dn11 / var_tratio))) })) / (assign13360_e7655 * assign13360_e7655)), (((var_uc_depmue0_dn14 * assign13360_e7655) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn14)) } } else { (assign13360_e7655 * (p.p381 * (var_tratio_dn14 / var_tratio))) })) / (assign13360_e7655 * assign13360_e7655)),)
    } else {
        (var_uc_depmue0, var_uc_depmue0_dn0, var_uc_depmue0_dn2, var_uc_depmue0_dn4, var_uc_depmue0_dn5, var_uc_depmue0_dn6, var_uc_depmue0_dn7, var_uc_depmue0_dn8, var_uc_depmue0_dn9, var_uc_depmue0_dn10, var_uc_depmue0_dn11, var_uc_depmue0_dn14,)
    }
};
        var_uc_depmue0 = assign13360_e7658;
        var_uc_depmue0_dn0 = assign13360_e7658_d_n0;
        var_uc_depmue0_dn2 = assign13360_e7658_d_n2;
        var_uc_depmue0_dn4 = assign13360_e7658_d_n4;
        var_uc_depmue0_dn5 = assign13360_e7658_d_n5;
        var_uc_depmue0_dn6 = assign13360_e7658_d_n6;
        var_uc_depmue0_dn7 = assign13360_e7658_d_n7;
        var_uc_depmue0_dn8 = assign13360_e7658_d_n8;
        var_uc_depmue0_dn9 = assign13360_e7658_d_n9;
        var_uc_depmue0_dn10 = assign13360_e7658_d_n10;
        var_uc_depmue0_dn11 = assign13360_e7658_d_n11;
        var_uc_depmue0_dn14 = assign13360_e7658_d_n14;

        let (assign13370_e7668, assign13370_e7668_d_n0, assign13370_e7668_d_n2, assign13370_e7668_d_n4, assign13370_e7668_d_n5, assign13370_e7668_d_n6, assign13370_e7668_d_n7, assign13370_e7668_d_n8, assign13370_e7668_d_n9, assign13370_e7668_d_n10, assign13370_e7668_d_n11, assign13370_e7668_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard294 != 0.0)) {
        let assign13370_e7665: f64 = (var_tratio).powf(p.p382);
        let assign13370_e7666: f64 = (var_uc_depmue2 / assign13370_e7665);
        (assign13370_e7666, (((var_uc_depmue2_dn0 * assign13370_e7665) - (var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((var_tratio).powf(p.p382 - 1.0) * var_tratio_dn0)) } } else { (assign13370_e7665 * (p.p382 * (var_tratio_dn0 / var_tratio))) })) / (assign13370_e7665 * assign13370_e7665)), (((var_uc_depmue2_dn2 * assign13370_e7665) - (var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((var_tratio).powf(p.p382 - 1.0) * var_tratio_dn2)) } } else { (assign13370_e7665 * (p.p382 * (var_tratio_dn2 / var_tratio))) })) / (assign13370_e7665 * assign13370_e7665)), (((var_uc_depmue2_dn4 * assign13370_e7665) - (var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((var_tratio).powf(p.p382 - 1.0) * var_tratio_dn4)) } } else { (assign13370_e7665 * (p.p382 * (var_tratio_dn4 / var_tratio))) })) / (assign13370_e7665 * assign13370_e7665)), (((var_uc_depmue2_dn5 * assign13370_e7665) - (var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((var_tratio).powf(p.p382 - 1.0) * var_tratio_dn5)) } } else { (assign13370_e7665 * (p.p382 * (var_tratio_dn5 / var_tratio))) })) / (assign13370_e7665 * assign13370_e7665)), (((var_uc_depmue2_dn6 * assign13370_e7665) - (var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((var_tratio).powf(p.p382 - 1.0) * var_tratio_dn6)) } } else { (assign13370_e7665 * (p.p382 * (var_tratio_dn6 / var_tratio))) })) / (assign13370_e7665 * assign13370_e7665)), (((var_uc_depmue2_dn7 * assign13370_e7665) - (var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((var_tratio).powf(p.p382 - 1.0) * var_tratio_dn7)) } } else { (assign13370_e7665 * (p.p382 * (var_tratio_dn7 / var_tratio))) })) / (assign13370_e7665 * assign13370_e7665)), (((var_uc_depmue2_dn8 * assign13370_e7665) - (var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((var_tratio).powf(p.p382 - 1.0) * var_tratio_dn8)) } } else { (assign13370_e7665 * (p.p382 * (var_tratio_dn8 / var_tratio))) })) / (assign13370_e7665 * assign13370_e7665)), (((var_uc_depmue2_dn9 * assign13370_e7665) - (var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((var_tratio).powf(p.p382 - 1.0) * var_tratio_dn9)) } } else { (assign13370_e7665 * (p.p382 * (var_tratio_dn9 / var_tratio))) })) / (assign13370_e7665 * assign13370_e7665)), (((var_uc_depmue2_dn10 * assign13370_e7665) - (var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((var_tratio).powf(p.p382 - 1.0) * var_tratio_dn10)) } } else { (assign13370_e7665 * (p.p382 * (var_tratio_dn10 / var_tratio))) })) / (assign13370_e7665 * assign13370_e7665)), (((var_uc_depmue2_dn11 * assign13370_e7665) - (var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((var_tratio).powf(p.p382 - 1.0) * var_tratio_dn11)) } } else { (assign13370_e7665 * (p.p382 * (var_tratio_dn11 / var_tratio))) })) / (assign13370_e7665 * assign13370_e7665)), (((var_uc_depmue2_dn14 * assign13370_e7665) - (var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((var_tratio).powf(p.p382 - 1.0) * var_tratio_dn14)) } } else { (assign13370_e7665 * (p.p382 * (var_tratio_dn14 / var_tratio))) })) / (assign13370_e7665 * assign13370_e7665)),)
    } else {
        (var_uc_depmue2, var_uc_depmue2_dn0, var_uc_depmue2_dn2, var_uc_depmue2_dn4, var_uc_depmue2_dn5, var_uc_depmue2_dn6, var_uc_depmue2_dn7, var_uc_depmue2_dn8, var_uc_depmue2_dn9, var_uc_depmue2_dn10, var_uc_depmue2_dn11, var_uc_depmue2_dn14,)
    }
};
        var_uc_depmue2 = assign13370_e7668;
        var_uc_depmue2_dn0 = assign13370_e7668_d_n0;
        var_uc_depmue2_dn2 = assign13370_e7668_d_n2;
        var_uc_depmue2_dn4 = assign13370_e7668_d_n4;
        var_uc_depmue2_dn5 = assign13370_e7668_d_n5;
        var_uc_depmue2_dn6 = assign13370_e7668_d_n6;
        var_uc_depmue2_dn7 = assign13370_e7668_d_n7;
        var_uc_depmue2_dn8 = assign13370_e7668_d_n8;
        var_uc_depmue2_dn9 = assign13370_e7668_d_n9;
        var_uc_depmue2_dn10 = assign13370_e7668_d_n10;
        var_uc_depmue2_dn11 = assign13370_e7668_d_n11;
        var_uc_depmue2_dn14 = assign13370_e7668_d_n14;

        let assign13380_e7671: f64 = if var_uc_codep == 3.0 { 1.0 } else { 0.0 };
        var_guard297 = assign13380_e7671;

        let (assign13390_e7689, assign13390_e7689_d_n0, assign13390_e7689_d_n2, assign13390_e7689_d_n4, assign13390_e7689_d_n5, assign13390_e7689_d_n6, assign13390_e7689_d_n7, assign13390_e7689_d_n8, assign13390_e7689_d_n9, assign13390_e7689_d_n10, assign13390_e7689_d_n11, assign13390_e7689_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard294 == 0.0)) && (var_guard297 != 0.0)) {
        let assign13390_e7680: f64 = (2.0 * 1.034943e-10);
        let assign13390_e7682: f64 = (assign13390_e7680 * 1.6021918e-19);
        let assign13390_e7684: f64 = (assign13390_e7682 * var_uc_ndepm);
        let assign13390_e7686: f64 = (assign13390_e7684 * var_beta_inv);
        let assign13390_e7687: f64 = (assign13390_e7686).sqrt();
        (assign13390_e7687, ((((assign13390_e7682 * var_uc_ndepm_dn0) * var_beta_inv) + (assign13390_e7684 * var_beta_inv_dn0)) / (2.0 * assign13390_e7687)), ((((assign13390_e7682 * var_uc_ndepm_dn2) * var_beta_inv) + (assign13390_e7684 * var_beta_inv_dn2)) / (2.0 * assign13390_e7687)), ((((assign13390_e7682 * var_uc_ndepm_dn4) * var_beta_inv) + (assign13390_e7684 * var_beta_inv_dn4)) / (2.0 * assign13390_e7687)), ((((assign13390_e7682 * var_uc_ndepm_dn5) * var_beta_inv) + (assign13390_e7684 * var_beta_inv_dn5)) / (2.0 * assign13390_e7687)), ((((assign13390_e7682 * var_uc_ndepm_dn6) * var_beta_inv) + (assign13390_e7684 * var_beta_inv_dn6)) / (2.0 * assign13390_e7687)), ((((assign13390_e7682 * var_uc_ndepm_dn7) * var_beta_inv) + (assign13390_e7684 * var_beta_inv_dn7)) / (2.0 * assign13390_e7687)), ((((assign13390_e7682 * var_uc_ndepm_dn8) * var_beta_inv) + (assign13390_e7684 * var_beta_inv_dn8)) / (2.0 * assign13390_e7687)), ((((assign13390_e7682 * var_uc_ndepm_dn9) * var_beta_inv) + (assign13390_e7684 * var_beta_inv_dn9)) / (2.0 * assign13390_e7687)), ((((assign13390_e7682 * var_uc_ndepm_dn10) * var_beta_inv) + (assign13390_e7684 * var_beta_inv_dn10)) / (2.0 * assign13390_e7687)), ((((assign13390_e7682 * var_uc_ndepm_dn11) * var_beta_inv) + (assign13390_e7684 * var_beta_inv_dn11)) / (2.0 * assign13390_e7687)), ((((assign13390_e7682 * var_uc_ndepm_dn14) * var_beta_inv) + (assign13390_e7684 * var_beta_inv_dn14)) / (2.0 * assign13390_e7687)),)
    } else {
        (var_cnst0, var_cnst0_dn0, var_cnst0_dn2, var_cnst0_dn4, var_cnst0_dn5, var_cnst0_dn6, var_cnst0_dn7, var_cnst0_dn8, var_cnst0_dn9, var_cnst0_dn10, var_cnst0_dn11, var_cnst0_dn14,)
    }
};
        var_cnst0 = assign13390_e7689;
        var_cnst0_dn0 = assign13390_e7689_d_n0;
        var_cnst0_dn2 = assign13390_e7689_d_n2;
        var_cnst0_dn4 = assign13390_e7689_d_n4;
        var_cnst0_dn5 = assign13390_e7689_d_n5;
        var_cnst0_dn6 = assign13390_e7689_d_n6;
        var_cnst0_dn7 = assign13390_e7689_d_n7;
        var_cnst0_dn8 = assign13390_e7689_d_n8;
        var_cnst0_dn9 = assign13390_e7689_d_n9;
        var_cnst0_dn10 = assign13390_e7689_d_n10;
        var_cnst0_dn11 = assign13390_e7689_d_n11;
        var_cnst0_dn14 = assign13390_e7689_d_n14;

        let (assign13400_e7704, assign13400_e7704_d_n0, assign13400_e7704_d_n2, assign13400_e7704_d_n4, assign13400_e7704_d_n5, assign13400_e7704_d_n6, assign13400_e7704_d_n7, assign13400_e7704_d_n8, assign13400_e7704_d_n9, assign13400_e7704_d_n10, assign13400_e7704_d_n11, assign13400_e7704_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard294 == 0.0)) && (var_guard297 != 0.0)) {
        let assign13400_e7698: f64 = (var_nin * var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / var_uc_ndepm;
        let assign13400_e7700: f64 = (assign13400_e7698 * __rspice_inv_cse_0);
        let assign13400_e7702: f64 = (assign13400_e7700 * __rspice_inv_cse_0);
        (assign13400_e7702, ((((((((var_nin_dn0 * var_nin) + (var_nin * var_nin_dn0)) * var_uc_ndepm) - (assign13400_e7698 * var_uc_ndepm_dn0)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13400_e7700 * var_uc_ndepm_dn0)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn2 * var_nin) + (var_nin * var_nin_dn2)) * var_uc_ndepm) - (assign13400_e7698 * var_uc_ndepm_dn2)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13400_e7700 * var_uc_ndepm_dn2)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn4 * var_nin) + (var_nin * var_nin_dn4)) * var_uc_ndepm) - (assign13400_e7698 * var_uc_ndepm_dn4)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13400_e7700 * var_uc_ndepm_dn4)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn5 * var_nin) + (var_nin * var_nin_dn5)) * var_uc_ndepm) - (assign13400_e7698 * var_uc_ndepm_dn5)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13400_e7700 * var_uc_ndepm_dn5)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn6 * var_nin) + (var_nin * var_nin_dn6)) * var_uc_ndepm) - (assign13400_e7698 * var_uc_ndepm_dn6)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13400_e7700 * var_uc_ndepm_dn6)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn7 * var_nin) + (var_nin * var_nin_dn7)) * var_uc_ndepm) - (assign13400_e7698 * var_uc_ndepm_dn7)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13400_e7700 * var_uc_ndepm_dn7)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn8 * var_nin) + (var_nin * var_nin_dn8)) * var_uc_ndepm) - (assign13400_e7698 * var_uc_ndepm_dn8)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13400_e7700 * var_uc_ndepm_dn8)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn9 * var_nin) + (var_nin * var_nin_dn9)) * var_uc_ndepm) - (assign13400_e7698 * var_uc_ndepm_dn9)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13400_e7700 * var_uc_ndepm_dn9)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn10 * var_nin) + (var_nin * var_nin_dn10)) * var_uc_ndepm) - (assign13400_e7698 * var_uc_ndepm_dn10)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13400_e7700 * var_uc_ndepm_dn10)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn11 * var_nin) + (var_nin * var_nin_dn11)) * var_uc_ndepm) - (assign13400_e7698 * var_uc_ndepm_dn11)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13400_e7700 * var_uc_ndepm_dn11)) / (var_uc_ndepm * var_uc_ndepm)), ((((((((var_nin_dn14 * var_nin) + (var_nin * var_nin_dn14)) * var_uc_ndepm) - (assign13400_e7698 * var_uc_ndepm_dn14)) / (var_uc_ndepm * var_uc_ndepm)) * var_uc_ndepm) - (assign13400_e7700 * var_uc_ndepm_dn14)) / (var_uc_ndepm * var_uc_ndepm)),)
    } else {
        (var_cnst1, var_cnst1_dn0, var_cnst1_dn2, var_cnst1_dn4, var_cnst1_dn5, var_cnst1_dn6, var_cnst1_dn7, var_cnst1_dn8, var_cnst1_dn9, var_cnst1_dn10, var_cnst1_dn11, var_cnst1_dn14,)
    }
};
        var_cnst1 = assign13400_e7704;
        var_cnst1_dn0 = assign13400_e7704_d_n0;
        var_cnst1_dn2 = assign13400_e7704_d_n2;
        var_cnst1_dn4 = assign13400_e7704_d_n4;
        var_cnst1_dn5 = assign13400_e7704_d_n5;
        var_cnst1_dn6 = assign13400_e7704_d_n6;
        var_cnst1_dn7 = assign13400_e7704_d_n7;
        var_cnst1_dn8 = assign13400_e7704_d_n8;
        var_cnst1_dn9 = assign13400_e7704_d_n9;
        var_cnst1_dn10 = assign13400_e7704_d_n10;
        var_cnst1_dn11 = assign13400_e7704_d_n11;
        var_cnst1_dn14 = assign13400_e7704_d_n14;

        let (assign13410_e7720, assign13410_e7720_d_n0, assign13410_e7720_d_n2, assign13410_e7720_d_n4, assign13410_e7720_d_n5, assign13410_e7720_d_n6, assign13410_e7720_d_n7, assign13410_e7720_d_n8, assign13410_e7720_d_n9, assign13410_e7720_d_n10, assign13410_e7720_d_n11, assign13410_e7720_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard294 == 0.0)) && (var_guard297 != 0.0)) {
        let assign13410_e7713: f64 = (2.0 * var_beta_inv);
        let assign13410_e7716: f64 = (var_uc_ndepm / var_nin);
        let assign13410_e7717: f64 = (assign13410_e7716).ln();
        let assign13410_e7718: f64 = (assign13410_e7713 * assign13410_e7717);
        (assign13410_e7718, (((2.0 * var_beta_inv_dn0) * assign13410_e7717) + (assign13410_e7713 * ((((var_uc_ndepm_dn0 * var_nin) - (var_uc_ndepm * var_nin_dn0)) / (var_nin * var_nin)) / assign13410_e7716))), (((2.0 * var_beta_inv_dn2) * assign13410_e7717) + (assign13410_e7713 * ((((var_uc_ndepm_dn2 * var_nin) - (var_uc_ndepm * var_nin_dn2)) / (var_nin * var_nin)) / assign13410_e7716))), (((2.0 * var_beta_inv_dn4) * assign13410_e7717) + (assign13410_e7713 * ((((var_uc_ndepm_dn4 * var_nin) - (var_uc_ndepm * var_nin_dn4)) / (var_nin * var_nin)) / assign13410_e7716))), (((2.0 * var_beta_inv_dn5) * assign13410_e7717) + (assign13410_e7713 * ((((var_uc_ndepm_dn5 * var_nin) - (var_uc_ndepm * var_nin_dn5)) / (var_nin * var_nin)) / assign13410_e7716))), (((2.0 * var_beta_inv_dn6) * assign13410_e7717) + (assign13410_e7713 * ((((var_uc_ndepm_dn6 * var_nin) - (var_uc_ndepm * var_nin_dn6)) / (var_nin * var_nin)) / assign13410_e7716))), (((2.0 * var_beta_inv_dn7) * assign13410_e7717) + (assign13410_e7713 * ((((var_uc_ndepm_dn7 * var_nin) - (var_uc_ndepm * var_nin_dn7)) / (var_nin * var_nin)) / assign13410_e7716))), (((2.0 * var_beta_inv_dn8) * assign13410_e7717) + (assign13410_e7713 * ((((var_uc_ndepm_dn8 * var_nin) - (var_uc_ndepm * var_nin_dn8)) / (var_nin * var_nin)) / assign13410_e7716))), (((2.0 * var_beta_inv_dn9) * assign13410_e7717) + (assign13410_e7713 * ((((var_uc_ndepm_dn9 * var_nin) - (var_uc_ndepm * var_nin_dn9)) / (var_nin * var_nin)) / assign13410_e7716))), (((2.0 * var_beta_inv_dn10) * assign13410_e7717) + (assign13410_e7713 * ((((var_uc_ndepm_dn10 * var_nin) - (var_uc_ndepm * var_nin_dn10)) / (var_nin * var_nin)) / assign13410_e7716))), (((2.0 * var_beta_inv_dn11) * assign13410_e7717) + (assign13410_e7713 * ((((var_uc_ndepm_dn11 * var_nin) - (var_uc_ndepm * var_nin_dn11)) / (var_nin * var_nin)) / assign13410_e7716))), (((2.0 * var_beta_inv_dn14) * assign13410_e7717) + (assign13410_e7713 * ((((var_uc_ndepm_dn14 * var_nin) - (var_uc_ndepm * var_nin_dn14)) / (var_nin * var_nin)) / assign13410_e7716))),)
    } else {
        (var_pb2n, var_pb2n_dn0, var_pb2n_dn2, var_pb2n_dn4, var_pb2n_dn5, var_pb2n_dn6, var_pb2n_dn7, var_pb2n_dn8, var_pb2n_dn9, var_pb2n_dn10, var_pb2n_dn11, var_pb2n_dn14,)
    }
};
        var_pb2n = assign13410_e7720;
        var_pb2n_dn0 = assign13410_e7720_d_n0;
        var_pb2n_dn2 = assign13410_e7720_d_n2;
        var_pb2n_dn4 = assign13410_e7720_d_n4;
        var_pb2n_dn5 = assign13410_e7720_d_n5;
        var_pb2n_dn6 = assign13410_e7720_d_n6;
        var_pb2n_dn7 = assign13410_e7720_d_n7;
        var_pb2n_dn8 = assign13410_e7720_d_n8;
        var_pb2n_dn9 = assign13410_e7720_d_n9;
        var_pb2n_dn10 = assign13410_e7720_d_n10;
        var_pb2n_dn11 = assign13410_e7720_d_n11;
        var_pb2n_dn14 = assign13410_e7720_d_n14;

        let (assign13420_e7738, assign13420_e7738_d_n0, assign13420_e7738_d_n2, assign13420_e7738_d_n4, assign13420_e7738_d_n5, assign13420_e7738_d_n6, assign13420_e7738_d_n7, assign13420_e7738_d_n8, assign13420_e7738_d_n9, assign13420_e7738_d_n10, assign13420_e7738_d_n11, assign13420_e7738_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard294 == 0.0)) && (var_guard297 != 0.0)) {
        let assign13420_e7730: f64 = (var_uc_ndepm * var_ef_nsubc);
        let __rspice_inv_cse_1: f64 = 1.0 / var_nin;
        let assign13420_e7732: f64 = (assign13420_e7730 * __rspice_inv_cse_1);
        let assign13420_e7734: f64 = (assign13420_e7732 * __rspice_inv_cse_1);
        let assign13420_e7735: f64 = (assign13420_e7734).ln();
        let assign13420_e7736: f64 = (var_beta_inv * assign13420_e7735);
        (assign13420_e7736, ((var_beta_inv_dn0 * assign13420_e7735) + (var_beta_inv * (((((((((var_uc_ndepm_dn0 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn0)) * var_nin) - (assign13420_e7730 * var_nin_dn0)) / (var_nin * var_nin)) * var_nin) - (assign13420_e7732 * var_nin_dn0)) / (var_nin * var_nin)) / assign13420_e7734))), ((var_beta_inv_dn2 * assign13420_e7735) + (var_beta_inv * (((((((((var_uc_ndepm_dn2 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn2)) * var_nin) - (assign13420_e7730 * var_nin_dn2)) / (var_nin * var_nin)) * var_nin) - (assign13420_e7732 * var_nin_dn2)) / (var_nin * var_nin)) / assign13420_e7734))), ((var_beta_inv_dn4 * assign13420_e7735) + (var_beta_inv * (((((((((var_uc_ndepm_dn4 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn4)) * var_nin) - (assign13420_e7730 * var_nin_dn4)) / (var_nin * var_nin)) * var_nin) - (assign13420_e7732 * var_nin_dn4)) / (var_nin * var_nin)) / assign13420_e7734))), ((var_beta_inv_dn5 * assign13420_e7735) + (var_beta_inv * (((((((((var_uc_ndepm_dn5 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn5)) * var_nin) - (assign13420_e7730 * var_nin_dn5)) / (var_nin * var_nin)) * var_nin) - (assign13420_e7732 * var_nin_dn5)) / (var_nin * var_nin)) / assign13420_e7734))), ((var_beta_inv_dn6 * assign13420_e7735) + (var_beta_inv * (((((((((var_uc_ndepm_dn6 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn6)) * var_nin) - (assign13420_e7730 * var_nin_dn6)) / (var_nin * var_nin)) * var_nin) - (assign13420_e7732 * var_nin_dn6)) / (var_nin * var_nin)) / assign13420_e7734))), ((var_beta_inv_dn7 * assign13420_e7735) + (var_beta_inv * (((((((((var_uc_ndepm_dn7 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn7)) * var_nin) - (assign13420_e7730 * var_nin_dn7)) / (var_nin * var_nin)) * var_nin) - (assign13420_e7732 * var_nin_dn7)) / (var_nin * var_nin)) / assign13420_e7734))), ((var_beta_inv_dn8 * assign13420_e7735) + (var_beta_inv * (((((((((var_uc_ndepm_dn8 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn8)) * var_nin) - (assign13420_e7730 * var_nin_dn8)) / (var_nin * var_nin)) * var_nin) - (assign13420_e7732 * var_nin_dn8)) / (var_nin * var_nin)) / assign13420_e7734))), ((var_beta_inv_dn9 * assign13420_e7735) + (var_beta_inv * (((((((((var_uc_ndepm_dn9 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn9)) * var_nin) - (assign13420_e7730 * var_nin_dn9)) / (var_nin * var_nin)) * var_nin) - (assign13420_e7732 * var_nin_dn9)) / (var_nin * var_nin)) / assign13420_e7734))), ((var_beta_inv_dn10 * assign13420_e7735) + (var_beta_inv * (((((((((var_uc_ndepm_dn10 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn10)) * var_nin) - (assign13420_e7730 * var_nin_dn10)) / (var_nin * var_nin)) * var_nin) - (assign13420_e7732 * var_nin_dn10)) / (var_nin * var_nin)) / assign13420_e7734))), ((var_beta_inv_dn11 * assign13420_e7735) + (var_beta_inv * (((((((((var_uc_ndepm_dn11 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn11)) * var_nin) - (assign13420_e7730 * var_nin_dn11)) / (var_nin * var_nin)) * var_nin) - (assign13420_e7732 * var_nin_dn11)) / (var_nin * var_nin)) / assign13420_e7734))), ((var_beta_inv_dn14 * assign13420_e7735) + (var_beta_inv * (((((((((var_uc_ndepm_dn14 * var_ef_nsubc) + (var_uc_ndepm * var_ef_nsubc_dn14)) * var_nin) - (assign13420_e7730 * var_nin_dn14)) / (var_nin * var_nin)) * var_nin) - (assign13420_e7732 * var_nin_dn14)) / (var_nin * var_nin)) / assign13420_e7734))),)
    } else {
        (var_vbipn, var_vbipn_dn0, var_vbipn_dn2, var_vbipn_dn4, var_vbipn_dn5, var_vbipn_dn6, var_vbipn_dn7, var_vbipn_dn8, var_vbipn_dn9, var_vbipn_dn10, var_vbipn_dn11, var_vbipn_dn14,)
    }
};
        var_vbipn = assign13420_e7738;
        var_vbipn_dn0 = assign13420_e7738_d_n0;
        var_vbipn_dn2 = assign13420_e7738_d_n2;
        var_vbipn_dn4 = assign13420_e7738_d_n4;
        var_vbipn_dn5 = assign13420_e7738_d_n5;
        var_vbipn_dn6 = assign13420_e7738_d_n6;
        var_vbipn_dn7 = assign13420_e7738_d_n7;
        var_vbipn_dn8 = assign13420_e7738_d_n8;
        var_vbipn_dn9 = assign13420_e7738_d_n9;
        var_vbipn_dn10 = assign13420_e7738_d_n10;
        var_vbipn_dn11 = assign13420_e7738_d_n11;
        var_vbipn_dn14 = assign13420_e7738_d_n14;

        let (assign13430_e7750, assign13430_e7750_d_n0, assign13430_e7750_d_n2, assign13430_e7750_d_n4, assign13430_e7750_d_n5, assign13430_e7750_d_n6, assign13430_e7750_d_n7, assign13430_e7750_d_n8, assign13430_e7750_d_n9, assign13430_e7750_d_n10, assign13430_e7750_d_n11, assign13430_e7750_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard294 == 0.0)) && (var_guard297 != 0.0)) {
        let assign13430_e7747: f64 = (var_log_tratio * p.p380);
        let assign13430_e7748: f64 = (assign13430_e7747).exp();
        (assign13430_e7748, (assign13430_e7748 * (var_log_tratio_dn0 * p.p380)), (assign13430_e7748 * (var_log_tratio_dn2 * p.p380)), (assign13430_e7748 * (var_log_tratio_dn4 * p.p380)), (assign13430_e7748 * (var_log_tratio_dn5 * p.p380)), (assign13430_e7748 * (var_log_tratio_dn6 * p.p380)), (assign13430_e7748 * (var_log_tratio_dn7 * p.p380)), (assign13430_e7748 * (var_log_tratio_dn8 * p.p380)), (assign13430_e7748 * (var_log_tratio_dn9 * p.p380)), (assign13430_e7748 * (var_log_tratio_dn10 * p.p380)), (assign13430_e7748 * (var_log_tratio_dn11 * p.p380)), (assign13430_e7748 * (var_log_tratio_dn14 * p.p380)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign13430_e7750;
        var_t1_dn0 = assign13430_e7750_d_n0;
        var_t1_dn2 = assign13430_e7750_d_n2;
        var_t1_dn4 = assign13430_e7750_d_n4;
        var_t1_dn5 = assign13430_e7750_d_n5;
        var_t1_dn6 = assign13430_e7750_d_n6;
        var_t1_dn7 = assign13430_e7750_d_n7;
        var_t1_dn8 = assign13430_e7750_d_n8;
        var_t1_dn9 = assign13430_e7750_d_n9;
        var_t1_dn10 = assign13430_e7750_d_n10;
        var_t1_dn11 = assign13430_e7750_d_n11;
        var_t1_dn14 = assign13430_e7750_d_n14;

        let (assign13440_e7761, assign13440_e7761_d_n0, assign13440_e7761_d_n2, assign13440_e7761_d_n4, assign13440_e7761_d_n5, assign13440_e7761_d_n6, assign13440_e7761_d_n7, assign13440_e7761_d_n8, assign13440_e7761_d_n9, assign13440_e7761_d_n10, assign13440_e7761_d_n11, assign13440_e7761_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard294 == 0.0)) && (var_guard297 != 0.0)) {
        let assign13440_e7759: f64 = (var_t1 / var_uc_depmueph1);
        (assign13440_e7759, (var_t1_dn0 / var_uc_depmueph1), (var_t1_dn2 / var_uc_depmueph1), (var_t1_dn4 / var_uc_depmueph1), (var_t1_dn5 / var_uc_depmueph1), (var_t1_dn6 / var_uc_depmueph1), (var_t1_dn7 / var_uc_depmueph1), (var_t1_dn8 / var_uc_depmueph1), (var_t1_dn9 / var_uc_depmueph1), (var_t1_dn10 / var_uc_depmueph1), (var_t1_dn11 / var_uc_depmueph1), (var_t1_dn14 / var_uc_depmueph1),)
    } else {
        (var_depmphn0, var_depmphn0_dn0, var_depmphn0_dn2, var_depmphn0_dn4, var_depmphn0_dn5, var_depmphn0_dn6, var_depmphn0_dn7, var_depmphn0_dn8, var_depmphn0_dn9, var_depmphn0_dn10, var_depmphn0_dn11, var_depmphn0_dn14,)
    }
};
        var_depmphn0 = assign13440_e7761;
        var_depmphn0_dn0 = assign13440_e7761_d_n0;
        var_depmphn0_dn2 = assign13440_e7761_d_n2;
        var_depmphn0_dn4 = assign13440_e7761_d_n4;
        var_depmphn0_dn5 = assign13440_e7761_d_n5;
        var_depmphn0_dn6 = assign13440_e7761_d_n6;
        var_depmphn0_dn7 = assign13440_e7761_d_n7;
        var_depmphn0_dn8 = assign13440_e7761_d_n8;
        var_depmphn0_dn9 = assign13440_e7761_d_n9;
        var_depmphn0_dn10 = assign13440_e7761_d_n10;
        var_depmphn0_dn11 = assign13440_e7761_d_n11;
        var_depmphn0_dn14 = assign13440_e7761_d_n14;

        let (assign13450_e7786, assign13450_e7786_d_n0, assign13450_e7786_d_n2, assign13450_e7786_d_n4, assign13450_e7786_d_n5, assign13450_e7786_d_n6, assign13450_e7786_d_n7, assign13450_e7786_d_n8, assign13450_e7786_d_n9, assign13450_e7786_d_n10, assign13450_e7786_d_n11, assign13450_e7786_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard294 == 0.0)) && (var_guard297 != 0.0)) {
        let assign13450_e7771: f64 = (0.4 * var_tratio);
        let assign13450_e7772: f64 = (1.8 + assign13450_e7771);
        let assign13450_e7775: f64 = (0.1 * var_tratio);
        let assign13450_e7777: f64 = (assign13450_e7775 * var_tratio);
        let assign13450_e7778: f64 = (assign13450_e7772 + assign13450_e7777);
        let assign13450_e7782: f64 = (1.0 - var_tratio);
        let assign13450_e7783: f64 = (p.p379 * assign13450_e7782);
        let assign13450_e7784: f64 = (assign13450_e7778 - assign13450_e7783);
        (assign13450_e7784, (((0.4 * var_tratio_dn0) + (((0.1 * var_tratio_dn0) * var_tratio) + (assign13450_e7775 * var_tratio_dn0))) - (p.p379 * (-var_tratio_dn0))), (((0.4 * var_tratio_dn2) + (((0.1 * var_tratio_dn2) * var_tratio) + (assign13450_e7775 * var_tratio_dn2))) - (p.p379 * (-var_tratio_dn2))), (((0.4 * var_tratio_dn4) + (((0.1 * var_tratio_dn4) * var_tratio) + (assign13450_e7775 * var_tratio_dn4))) - (p.p379 * (-var_tratio_dn4))), (((0.4 * var_tratio_dn5) + (((0.1 * var_tratio_dn5) * var_tratio) + (assign13450_e7775 * var_tratio_dn5))) - (p.p379 * (-var_tratio_dn5))), (((0.4 * var_tratio_dn6) + (((0.1 * var_tratio_dn6) * var_tratio) + (assign13450_e7775 * var_tratio_dn6))) - (p.p379 * (-var_tratio_dn6))), (((0.4 * var_tratio_dn7) + (((0.1 * var_tratio_dn7) * var_tratio) + (assign13450_e7775 * var_tratio_dn7))) - (p.p379 * (-var_tratio_dn7))), (((0.4 * var_tratio_dn8) + (((0.1 * var_tratio_dn8) * var_tratio) + (assign13450_e7775 * var_tratio_dn8))) - (p.p379 * (-var_tratio_dn8))), (((0.4 * var_tratio_dn9) + (((0.1 * var_tratio_dn9) * var_tratio) + (assign13450_e7775 * var_tratio_dn9))) - (p.p379 * (-var_tratio_dn9))), (((0.4 * var_tratio_dn10) + (((0.1 * var_tratio_dn10) * var_tratio) + (assign13450_e7775 * var_tratio_dn10))) - (p.p379 * (-var_tratio_dn10))), (((0.4 * var_tratio_dn11) + (((0.1 * var_tratio_dn11) * var_tratio) + (assign13450_e7775 * var_tratio_dn11))) - (p.p379 * (-var_tratio_dn11))), (((0.4 * var_tratio_dn14) + (((0.1 * var_tratio_dn14) * var_tratio) + (assign13450_e7775 * var_tratio_dn14))) - (p.p379 * (-var_tratio_dn14))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign13450_e7786;
        var_t0_dn0 = assign13450_e7786_d_n0;
        var_t0_dn2 = assign13450_e7786_d_n2;
        var_t0_dn4 = assign13450_e7786_d_n4;
        var_t0_dn5 = assign13450_e7786_d_n5;
        var_t0_dn6 = assign13450_e7786_d_n6;
        var_t0_dn7 = assign13450_e7786_d_n7;
        var_t0_dn8 = assign13450_e7786_d_n8;
        var_t0_dn9 = assign13450_e7786_d_n9;
        var_t0_dn10 = assign13450_e7786_d_n10;
        var_t0_dn11 = assign13450_e7786_d_n11;
        var_t0_dn14 = assign13450_e7786_d_n14;

        let (assign13460_e7797, assign13460_e7797_d_n0, assign13460_e7797_d_n2, assign13460_e7797_d_n4, assign13460_e7797_d_n5, assign13460_e7797_d_n6, assign13460_e7797_d_n7, assign13460_e7797_d_n8, assign13460_e7797_d_n9, assign13460_e7797_d_n10, assign13460_e7797_d_n11, assign13460_e7797_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard294 == 0.0)) && (var_guard297 != 0.0)) {
        let assign13460_e7795: f64 = (var_uc_depvmax / var_t0);
        (assign13460_e7795, (((var_uc_depvmax_dn0 * var_t0) - (var_uc_depvmax * var_t0_dn0)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn2 * var_t0) - (var_uc_depvmax * var_t0_dn2)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn4 * var_t0) - (var_uc_depvmax * var_t0_dn4)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn5 * var_t0) - (var_uc_depvmax * var_t0_dn5)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn6 * var_t0) - (var_uc_depvmax * var_t0_dn6)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn7 * var_t0) - (var_uc_depvmax * var_t0_dn7)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn8 * var_t0) - (var_uc_depvmax * var_t0_dn8)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn9 * var_t0) - (var_uc_depvmax * var_t0_dn9)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn10 * var_t0) - (var_uc_depvmax * var_t0_dn10)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn11 * var_t0) - (var_uc_depvmax * var_t0_dn11)) / (var_t0 * var_t0)), (((var_uc_depvmax_dn14 * var_t0) - (var_uc_depvmax * var_t0_dn14)) / (var_t0 * var_t0)),)
    } else {
        (var_uc_depvmax, var_uc_depvmax_dn0, var_uc_depvmax_dn2, var_uc_depvmax_dn4, var_uc_depvmax_dn5, var_uc_depvmax_dn6, var_uc_depvmax_dn7, var_uc_depvmax_dn8, var_uc_depvmax_dn9, var_uc_depvmax_dn10, var_uc_depvmax_dn11, var_uc_depvmax_dn14,)
    }
};
        var_uc_depvmax = assign13460_e7797;
        var_uc_depvmax_dn0 = assign13460_e7797_d_n0;
        var_uc_depvmax_dn2 = assign13460_e7797_d_n2;
        var_uc_depvmax_dn4 = assign13460_e7797_d_n4;
        var_uc_depvmax_dn5 = assign13460_e7797_d_n5;
        var_uc_depvmax_dn6 = assign13460_e7797_d_n6;
        var_uc_depvmax_dn7 = assign13460_e7797_d_n7;
        var_uc_depvmax_dn8 = assign13460_e7797_d_n8;
        var_uc_depvmax_dn9 = assign13460_e7797_d_n9;
        var_uc_depvmax_dn10 = assign13460_e7797_d_n10;
        var_uc_depvmax_dn11 = assign13460_e7797_d_n11;
        var_uc_depvmax_dn14 = assign13460_e7797_d_n14;

        let assign13480_e7805: f64 = if var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        var_guard299 = assign13480_e7805;

        let (assign13490_e7816, assign13490_e7816_d_n0, assign13490_e7816_d_n2, assign13490_e7816_d_n4, assign13490_e7816_d_n5, assign13490_e7816_d_n6, assign13490_e7816_d_n7, assign13490_e7816_d_n8, assign13490_e7816_d_n9, assign13490_e7816_d_n10, assign13490_e7816_d_n11, assign13490_e7816_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard294 == 0.0)) && (var_guard297 != 0.0)) && (var_guard299 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depvmax, var_uc_depvmax_dn0, var_uc_depvmax_dn2, var_uc_depvmax_dn4, var_uc_depvmax_dn5, var_uc_depvmax_dn6, var_uc_depvmax_dn7, var_uc_depvmax_dn8, var_uc_depvmax_dn9, var_uc_depvmax_dn10, var_uc_depvmax_dn11, var_uc_depvmax_dn14,)
    }
};
        var_uc_depvmax = assign13490_e7816;
        var_uc_depvmax_dn0 = assign13490_e7816_d_n0;
        var_uc_depvmax_dn2 = assign13490_e7816_d_n2;
        var_uc_depvmax_dn4 = assign13490_e7816_d_n4;
        var_uc_depvmax_dn5 = assign13490_e7816_d_n5;
        var_uc_depvmax_dn6 = assign13490_e7816_d_n6;
        var_uc_depvmax_dn7 = assign13490_e7816_d_n7;
        var_uc_depvmax_dn8 = assign13490_e7816_d_n8;
        var_uc_depvmax_dn9 = assign13490_e7816_d_n9;
        var_uc_depvmax_dn10 = assign13490_e7816_d_n10;
        var_uc_depvmax_dn11 = assign13490_e7816_d_n11;
        var_uc_depvmax_dn14 = assign13490_e7816_d_n14;

        let (assign13500_e7829, assign13500_e7829_d_n0, assign13500_e7829_d_n2, assign13500_e7829_d_n4, assign13500_e7829_d_n5, assign13500_e7829_d_n6, assign13500_e7829_d_n7, assign13500_e7829_d_n8, assign13500_e7829_d_n9, assign13500_e7829_d_n10, assign13500_e7829_d_n11, assign13500_e7829_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard294 == 0.0)) && (var_guard297 != 0.0)) {
        let assign13500_e7826: f64 = (var_tratio).powf(p.p381);
        let assign13500_e7827: f64 = (var_uc_depmue0 / assign13500_e7826);
        (assign13500_e7827, (((var_uc_depmue0_dn0 * assign13500_e7826) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn0)) } } else { (assign13500_e7826 * (p.p381 * (var_tratio_dn0 / var_tratio))) })) / (assign13500_e7826 * assign13500_e7826)), (((var_uc_depmue0_dn2 * assign13500_e7826) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn2)) } } else { (assign13500_e7826 * (p.p381 * (var_tratio_dn2 / var_tratio))) })) / (assign13500_e7826 * assign13500_e7826)), (((var_uc_depmue0_dn4 * assign13500_e7826) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn4)) } } else { (assign13500_e7826 * (p.p381 * (var_tratio_dn4 / var_tratio))) })) / (assign13500_e7826 * assign13500_e7826)), (((var_uc_depmue0_dn5 * assign13500_e7826) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn5)) } } else { (assign13500_e7826 * (p.p381 * (var_tratio_dn5 / var_tratio))) })) / (assign13500_e7826 * assign13500_e7826)), (((var_uc_depmue0_dn6 * assign13500_e7826) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn6)) } } else { (assign13500_e7826 * (p.p381 * (var_tratio_dn6 / var_tratio))) })) / (assign13500_e7826 * assign13500_e7826)), (((var_uc_depmue0_dn7 * assign13500_e7826) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn7)) } } else { (assign13500_e7826 * (p.p381 * (var_tratio_dn7 / var_tratio))) })) / (assign13500_e7826 * assign13500_e7826)), (((var_uc_depmue0_dn8 * assign13500_e7826) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn8)) } } else { (assign13500_e7826 * (p.p381 * (var_tratio_dn8 / var_tratio))) })) / (assign13500_e7826 * assign13500_e7826)), (((var_uc_depmue0_dn9 * assign13500_e7826) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn9)) } } else { (assign13500_e7826 * (p.p381 * (var_tratio_dn9 / var_tratio))) })) / (assign13500_e7826 * assign13500_e7826)), (((var_uc_depmue0_dn10 * assign13500_e7826) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn10)) } } else { (assign13500_e7826 * (p.p381 * (var_tratio_dn10 / var_tratio))) })) / (assign13500_e7826 * assign13500_e7826)), (((var_uc_depmue0_dn11 * assign13500_e7826) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn11)) } } else { (assign13500_e7826 * (p.p381 * (var_tratio_dn11 / var_tratio))) })) / (assign13500_e7826 * assign13500_e7826)), (((var_uc_depmue0_dn14 * assign13500_e7826) - (var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((var_tratio).powf(p.p381 - 1.0) * var_tratio_dn14)) } } else { (assign13500_e7826 * (p.p381 * (var_tratio_dn14 / var_tratio))) })) / (assign13500_e7826 * assign13500_e7826)),)
    } else {
        (var_uc_depmue0, var_uc_depmue0_dn0, var_uc_depmue0_dn2, var_uc_depmue0_dn4, var_uc_depmue0_dn5, var_uc_depmue0_dn6, var_uc_depmue0_dn7, var_uc_depmue0_dn8, var_uc_depmue0_dn9, var_uc_depmue0_dn10, var_uc_depmue0_dn11, var_uc_depmue0_dn14,)
    }
};
        var_uc_depmue0 = assign13500_e7829;
        var_uc_depmue0_dn0 = assign13500_e7829_d_n0;
        var_uc_depmue0_dn2 = assign13500_e7829_d_n2;
        var_uc_depmue0_dn4 = assign13500_e7829_d_n4;
        var_uc_depmue0_dn5 = assign13500_e7829_d_n5;
        var_uc_depmue0_dn6 = assign13500_e7829_d_n6;
        var_uc_depmue0_dn7 = assign13500_e7829_d_n7;
        var_uc_depmue0_dn8 = assign13500_e7829_d_n8;
        var_uc_depmue0_dn9 = assign13500_e7829_d_n9;
        var_uc_depmue0_dn10 = assign13500_e7829_d_n10;
        var_uc_depmue0_dn11 = assign13500_e7829_d_n11;
        var_uc_depmue0_dn14 = assign13500_e7829_d_n14;

        let (assign13510_e7844, assign13510_e7844_d_n0, assign13510_e7844_d_n2, assign13510_e7844_d_n4, assign13510_e7844_d_n5, assign13510_e7844_d_n6, assign13510_e7844_d_n7, assign13510_e7844_d_n8, assign13510_e7844_d_n9, assign13510_e7844_d_n10, assign13510_e7844_d_n11, assign13510_e7844_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard294 == 0.0)) && (var_guard297 != 0.0)) {
        let assign13510_e7840: f64 = (var_tratio - 1.0);
        let assign13510_e7841: f64 = (p.p365 * assign13510_e7840);
        let assign13510_e7842: f64 = (p.p364 + assign13510_e7841);
        (assign13510_e7842, (p.p365 * var_tratio_dn0), (p.p365 * var_tratio_dn2), (p.p365 * var_tratio_dn4), (p.p365 * var_tratio_dn5), (p.p365 * var_tratio_dn6), (p.p365 * var_tratio_dn7), (p.p365 * var_tratio_dn8), (p.p365 * var_tratio_dn9), (p.p365 * var_tratio_dn10), (p.p365 * var_tratio_dn11), (p.p365 * var_tratio_dn14),)
    } else {
        (var_uc_depwlp, var_uc_depwlp_dn0, var_uc_depwlp_dn2, var_uc_depwlp_dn4, var_uc_depwlp_dn5, var_uc_depwlp_dn6, var_uc_depwlp_dn7, var_uc_depwlp_dn8, var_uc_depwlp_dn9, var_uc_depwlp_dn10, var_uc_depwlp_dn11, var_uc_depwlp_dn14,)
    }
};
        var_uc_depwlp = assign13510_e7844;
        var_uc_depwlp_dn0 = assign13510_e7844_d_n0;
        var_uc_depwlp_dn2 = assign13510_e7844_d_n2;
        var_uc_depwlp_dn4 = assign13510_e7844_d_n4;
        var_uc_depwlp_dn5 = assign13510_e7844_d_n5;
        var_uc_depwlp_dn6 = assign13510_e7844_d_n6;
        var_uc_depwlp_dn7 = assign13510_e7844_d_n7;
        var_uc_depwlp_dn8 = assign13510_e7844_d_n8;
        var_uc_depwlp_dn9 = assign13510_e7844_d_n9;
        var_uc_depwlp_dn10 = assign13510_e7844_d_n10;
        var_uc_depwlp_dn11 = assign13510_e7844_d_n11;
        var_uc_depwlp_dn14 = assign13510_e7844_d_n14;

        let (assign13520_e7854, assign13520_e7854_d_n0, assign13520_e7854_d_n2, assign13520_e7854_d_n4, assign13520_e7854_d_n5, assign13520_e7854_d_n6, assign13520_e7854_d_n7, assign13520_e7854_d_n8, assign13520_e7854_d_n9, assign13520_e7854_d_n10, assign13520_e7854_d_n11, assign13520_e7854_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard294 == 0.0)) && (var_guard297 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pb2n, var_pb2n_dn0, var_pb2n_dn2, var_pb2n_dn4, var_pb2n_dn5, var_pb2n_dn6, var_pb2n_dn7, var_pb2n_dn8, var_pb2n_dn9, var_pb2n_dn10, var_pb2n_dn11, var_pb2n_dn14,)
    }
};
        var_pb2n = assign13520_e7854;
        var_pb2n_dn0 = assign13520_e7854_d_n0;
        var_pb2n_dn2 = assign13520_e7854_d_n2;
        var_pb2n_dn4 = assign13520_e7854_d_n4;
        var_pb2n_dn5 = assign13520_e7854_d_n5;
        var_pb2n_dn6 = assign13520_e7854_d_n6;
        var_pb2n_dn7 = assign13520_e7854_d_n7;
        var_pb2n_dn8 = assign13520_e7854_d_n8;
        var_pb2n_dn9 = assign13520_e7854_d_n9;
        var_pb2n_dn10 = assign13520_e7854_d_n10;
        var_pb2n_dn11 = assign13520_e7854_d_n11;
        var_pb2n_dn14 = assign13520_e7854_d_n14;

        let (assign13530_e7873, assign13530_e7873_d_n0, assign13530_e7873_d_n2, assign13530_e7873_d_n4, assign13530_e7873_d_n5, assign13530_e7873_d_n6, assign13530_e7873_d_n7, assign13530_e7873_d_n8, assign13530_e7873_d_n9, assign13530_e7873_d_n10, assign13530_e7873_d_n11, assign13530_e7873_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard294 == 0.0)) && (var_guard297 == 0.0)) {
        let assign13530_e7865: f64 = (var_uc_njunc / var_nin);
        let assign13530_e7867: f64 = (assign13530_e7865 * var_nsub);
        let assign13530_e7869: f64 = (assign13530_e7867 / var_nin);
        let assign13530_e7870: f64 = (assign13530_e7869).ln();
        let assign13530_e7871: f64 = (var_beta_inv * assign13530_e7870);
        (assign13530_e7871, ((var_beta_inv_dn0 * assign13530_e7870) + (var_beta_inv * (((((((-((var_uc_njunc * var_nin_dn0) / (var_nin * var_nin))) * var_nsub) + (assign13530_e7865 * var_nsub_dn0)) * var_nin) - (assign13530_e7867 * var_nin_dn0)) / (var_nin * var_nin)) / assign13530_e7869))), ((var_beta_inv_dn2 * assign13530_e7870) + (var_beta_inv * (((((((-((var_uc_njunc * var_nin_dn2) / (var_nin * var_nin))) * var_nsub) + (assign13530_e7865 * var_nsub_dn2)) * var_nin) - (assign13530_e7867 * var_nin_dn2)) / (var_nin * var_nin)) / assign13530_e7869))), ((var_beta_inv_dn4 * assign13530_e7870) + (var_beta_inv * (((((((-((var_uc_njunc * var_nin_dn4) / (var_nin * var_nin))) * var_nsub) + (assign13530_e7865 * var_nsub_dn4)) * var_nin) - (assign13530_e7867 * var_nin_dn4)) / (var_nin * var_nin)) / assign13530_e7869))), ((var_beta_inv_dn5 * assign13530_e7870) + (var_beta_inv * (((((((-((var_uc_njunc * var_nin_dn5) / (var_nin * var_nin))) * var_nsub) + (assign13530_e7865 * var_nsub_dn5)) * var_nin) - (assign13530_e7867 * var_nin_dn5)) / (var_nin * var_nin)) / assign13530_e7869))), ((var_beta_inv_dn6 * assign13530_e7870) + (var_beta_inv * (((((((-((var_uc_njunc * var_nin_dn6) / (var_nin * var_nin))) * var_nsub) + (assign13530_e7865 * var_nsub_dn6)) * var_nin) - (assign13530_e7867 * var_nin_dn6)) / (var_nin * var_nin)) / assign13530_e7869))), ((var_beta_inv_dn7 * assign13530_e7870) + (var_beta_inv * (((((((-((var_uc_njunc * var_nin_dn7) / (var_nin * var_nin))) * var_nsub) + (assign13530_e7865 * var_nsub_dn7)) * var_nin) - (assign13530_e7867 * var_nin_dn7)) / (var_nin * var_nin)) / assign13530_e7869))), ((var_beta_inv_dn8 * assign13530_e7870) + (var_beta_inv * (((((((-((var_uc_njunc * var_nin_dn8) / (var_nin * var_nin))) * var_nsub) + (assign13530_e7865 * var_nsub_dn8)) * var_nin) - (assign13530_e7867 * var_nin_dn8)) / (var_nin * var_nin)) / assign13530_e7869))), ((var_beta_inv_dn9 * assign13530_e7870) + (var_beta_inv * (((((((-((var_uc_njunc * var_nin_dn9) / (var_nin * var_nin))) * var_nsub) + (assign13530_e7865 * var_nsub_dn9)) * var_nin) - (assign13530_e7867 * var_nin_dn9)) / (var_nin * var_nin)) / assign13530_e7869))), ((var_beta_inv_dn10 * assign13530_e7870) + (var_beta_inv * (((((((-((var_uc_njunc * var_nin_dn10) / (var_nin * var_nin))) * var_nsub) + (assign13530_e7865 * var_nsub_dn10)) * var_nin) - (assign13530_e7867 * var_nin_dn10)) / (var_nin * var_nin)) / assign13530_e7869))), ((var_beta_inv_dn11 * assign13530_e7870) + (var_beta_inv * (((((((-((var_uc_njunc * var_nin_dn11) / (var_nin * var_nin))) * var_nsub) + (assign13530_e7865 * var_nsub_dn11)) * var_nin) - (assign13530_e7867 * var_nin_dn11)) / (var_nin * var_nin)) / assign13530_e7869))), ((var_beta_inv_dn14 * assign13530_e7870) + (var_beta_inv * (((((((-((var_uc_njunc * var_nin_dn14) / (var_nin * var_nin))) * var_nsub) + (assign13530_e7865 * var_nsub_dn14)) * var_nin) - (assign13530_e7867 * var_nin_dn14)) / (var_nin * var_nin)) / assign13530_e7869))),)
    } else {
        (var_vbipn, var_vbipn_dn0, var_vbipn_dn2, var_vbipn_dn4, var_vbipn_dn5, var_vbipn_dn6, var_vbipn_dn7, var_vbipn_dn8, var_vbipn_dn9, var_vbipn_dn10, var_vbipn_dn11, var_vbipn_dn14,)
    }
};
        var_vbipn = assign13530_e7873;
        var_vbipn_dn0 = assign13530_e7873_d_n0;
        var_vbipn_dn2 = assign13530_e7873_d_n2;
        var_vbipn_dn4 = assign13530_e7873_d_n4;
        var_vbipn_dn5 = assign13530_e7873_d_n5;
        var_vbipn_dn6 = assign13530_e7873_d_n6;
        var_vbipn_dn7 = assign13530_e7873_d_n7;
        var_vbipn_dn8 = assign13530_e7873_d_n8;
        var_vbipn_dn9 = assign13530_e7873_d_n9;
        var_vbipn_dn10 = assign13530_e7873_d_n10;
        var_vbipn_dn11 = assign13530_e7873_d_n11;
        var_vbipn_dn14 = assign13530_e7873_d_n14;

        let (assign13540_e7883, assign13540_e7883_d_n0, assign13540_e7883_d_n2, assign13540_e7883_d_n4, assign13540_e7883_d_n5, assign13540_e7883_d_n6, assign13540_e7883_d_n7, assign13540_e7883_d_n8, assign13540_e7883_d_n9, assign13540_e7883_d_n10, assign13540_e7883_d_n11, assign13540_e7883_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard294 == 0.0)) && (var_guard297 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_depmphn0, var_depmphn0_dn0, var_depmphn0_dn2, var_depmphn0_dn4, var_depmphn0_dn5, var_depmphn0_dn6, var_depmphn0_dn7, var_depmphn0_dn8, var_depmphn0_dn9, var_depmphn0_dn10, var_depmphn0_dn11, var_depmphn0_dn14,)
    }
};
        var_depmphn0 = assign13540_e7883;
        var_depmphn0_dn0 = assign13540_e7883_d_n0;
        var_depmphn0_dn2 = assign13540_e7883_d_n2;
        var_depmphn0_dn4 = assign13540_e7883_d_n4;
        var_depmphn0_dn5 = assign13540_e7883_d_n5;
        var_depmphn0_dn6 = assign13540_e7883_d_n6;
        var_depmphn0_dn7 = assign13540_e7883_d_n7;
        var_depmphn0_dn8 = assign13540_e7883_d_n8;
        var_depmphn0_dn9 = assign13540_e7883_d_n9;
        var_depmphn0_dn10 = assign13540_e7883_d_n10;
        var_depmphn0_dn11 = assign13540_e7883_d_n11;
        var_depmphn0_dn14 = assign13540_e7883_d_n14;

        let (assign13550_e7889, assign13550_e7889_d_n0, assign13550_e7889_d_n2, assign13550_e7889_d_n4, assign13550_e7889_d_n5, assign13550_e7889_d_n6, assign13550_e7889_d_n7, assign13550_e7889_d_n8, assign13550_e7889_d_n9, assign13550_e7889_d_n10, assign13550_e7889_d_n11, assign13550_e7889_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13550_e7887: f64 = (var_ptovr0 * var_beta_inv);
        (assign13550_e7887, ((var_ptovr0_dn0 * var_beta_inv) + (var_ptovr0 * var_beta_inv_dn0)), ((var_ptovr0_dn2 * var_beta_inv) + (var_ptovr0 * var_beta_inv_dn2)), ((var_ptovr0_dn4 * var_beta_inv) + (var_ptovr0 * var_beta_inv_dn4)), ((var_ptovr0_dn5 * var_beta_inv) + (var_ptovr0 * var_beta_inv_dn5)), ((var_ptovr0_dn6 * var_beta_inv) + (var_ptovr0 * var_beta_inv_dn6)), ((var_ptovr0_dn7 * var_beta_inv) + (var_ptovr0 * var_beta_inv_dn7)), ((var_ptovr0_dn8 * var_beta_inv) + (var_ptovr0 * var_beta_inv_dn8)), ((var_ptovr0_dn9 * var_beta_inv) + (var_ptovr0 * var_beta_inv_dn9)), ((var_ptovr0_dn10 * var_beta_inv) + (var_ptovr0 * var_beta_inv_dn10)), ((var_ptovr0_dn11 * var_beta_inv) + (var_ptovr0 * var_beta_inv_dn11)), ((var_ptovr0_dn14 * var_beta_inv) + (var_ptovr0 * var_beta_inv_dn14)),)
    } else {
        (var_ptovr, var_ptovr_dn0, var_ptovr_dn2, var_ptovr_dn4, var_ptovr_dn5, var_ptovr_dn6, var_ptovr_dn7, var_ptovr_dn8, var_ptovr_dn9, var_ptovr_dn10, var_ptovr_dn11, var_ptovr_dn14,)
    }
};
        var_ptovr = assign13550_e7889;
        var_ptovr_dn0 = assign13550_e7889_d_n0;
        var_ptovr_dn2 = assign13550_e7889_d_n2;
        var_ptovr_dn4 = assign13550_e7889_d_n4;
        var_ptovr_dn5 = assign13550_e7889_d_n5;
        var_ptovr_dn6 = assign13550_e7889_d_n6;
        var_ptovr_dn7 = assign13550_e7889_d_n7;
        var_ptovr_dn8 = assign13550_e7889_d_n8;
        var_ptovr_dn9 = assign13550_e7889_d_n9;
        var_ptovr_dn10 = assign13550_e7889_d_n10;
        var_ptovr_dn11 = assign13550_e7889_d_n11;
        var_ptovr_dn14 = assign13550_e7889_d_n14;

        let (assign13560_e7895, assign13560_e7895_d_n0, assign13560_e7895_d_n2, assign13560_e7895_d_n4, assign13560_e7895_d_n5, assign13560_e7895_d_n6, assign13560_e7895_d_n7, assign13560_e7895_d_n8, assign13560_e7895_d_n9, assign13560_e7895_d_n10, assign13560_e7895_d_n11, assign13560_e7895_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13560_e7893: f64 = (var_ttemp / var_ktnom);
        (assign13560_e7893, (var_ttemp_dn0 / var_ktnom), (var_ttemp_dn2 / var_ktnom), (var_ttemp_dn4 / var_ktnom), (var_ttemp_dn5 / var_ktnom), (var_ttemp_dn6 / var_ktnom), (var_ttemp_dn7 / var_ktnom), (var_ttemp_dn8 / var_ktnom), (var_ttemp_dn9 / var_ktnom), (var_ttemp_dn10 / var_ktnom), (var_ttemp_dn11 / var_ktnom), (var_ttemp_dn14 / var_ktnom),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign13560_e7895;
        var_t1_dn0 = assign13560_e7895_d_n0;
        var_t1_dn2 = assign13560_e7895_d_n2;
        var_t1_dn4 = assign13560_e7895_d_n4;
        var_t1_dn5 = assign13560_e7895_d_n5;
        var_t1_dn6 = assign13560_e7895_d_n6;
        var_t1_dn7 = assign13560_e7895_d_n7;
        var_t1_dn8 = assign13560_e7895_d_n8;
        var_t1_dn9 = assign13560_e7895_d_n9;
        var_t1_dn10 = assign13560_e7895_d_n10;
        var_t1_dn11 = assign13560_e7895_d_n11;
        var_t1_dn14 = assign13560_e7895_d_n14;

        *var_cnst0_slot = var_cnst0;
        *var_cnst0_dn0_slot = var_cnst0_dn0;
        *var_cnst0_dn10_slot = var_cnst0_dn10;
        *var_cnst0_dn11_slot = var_cnst0_dn11;
        *var_cnst0_dn14_slot = var_cnst0_dn14;
        *var_cnst0_dn2_slot = var_cnst0_dn2;
        *var_cnst0_dn4_slot = var_cnst0_dn4;
        *var_cnst0_dn5_slot = var_cnst0_dn5;
        *var_cnst0_dn6_slot = var_cnst0_dn6;
        *var_cnst0_dn7_slot = var_cnst0_dn7;
        *var_cnst0_dn8_slot = var_cnst0_dn8;
        *var_cnst0_dn9_slot = var_cnst0_dn9;
        *var_cnst1_slot = var_cnst1;
        *var_cnst1_dn0_slot = var_cnst1_dn0;
        *var_cnst1_dn10_slot = var_cnst1_dn10;
        *var_cnst1_dn11_slot = var_cnst1_dn11;
        *var_cnst1_dn14_slot = var_cnst1_dn14;
        *var_cnst1_dn2_slot = var_cnst1_dn2;
        *var_cnst1_dn4_slot = var_cnst1_dn4;
        *var_cnst1_dn5_slot = var_cnst1_dn5;
        *var_cnst1_dn6_slot = var_cnst1_dn6;
        *var_cnst1_dn7_slot = var_cnst1_dn7;
        *var_cnst1_dn8_slot = var_cnst1_dn8;
        *var_cnst1_dn9_slot = var_cnst1_dn9;
        *var_depmphn0_slot = var_depmphn0;
        *var_depmphn0_dn0_slot = var_depmphn0_dn0;
        *var_depmphn0_dn10_slot = var_depmphn0_dn10;
        *var_depmphn0_dn11_slot = var_depmphn0_dn11;
        *var_depmphn0_dn14_slot = var_depmphn0_dn14;
        *var_depmphn0_dn2_slot = var_depmphn0_dn2;
        *var_depmphn0_dn4_slot = var_depmphn0_dn4;
        *var_depmphn0_dn5_slot = var_depmphn0_dn5;
        *var_depmphn0_dn6_slot = var_depmphn0_dn6;
        *var_depmphn0_dn7_slot = var_depmphn0_dn7;
        *var_depmphn0_dn8_slot = var_depmphn0_dn8;
        *var_depmphn0_dn9_slot = var_depmphn0_dn9;
        *var_guard296_slot = var_guard296;
        *var_guard297_slot = var_guard297;
        *var_guard299_slot = var_guard299;
        *var_pb2n_slot = var_pb2n;
        *var_pb2n_dn0_slot = var_pb2n_dn0;
        *var_pb2n_dn10_slot = var_pb2n_dn10;
        *var_pb2n_dn11_slot = var_pb2n_dn11;
        *var_pb2n_dn14_slot = var_pb2n_dn14;
        *var_pb2n_dn2_slot = var_pb2n_dn2;
        *var_pb2n_dn4_slot = var_pb2n_dn4;
        *var_pb2n_dn5_slot = var_pb2n_dn5;
        *var_pb2n_dn6_slot = var_pb2n_dn6;
        *var_pb2n_dn7_slot = var_pb2n_dn7;
        *var_pb2n_dn8_slot = var_pb2n_dn8;
        *var_pb2n_dn9_slot = var_pb2n_dn9;
        *var_ptovr_slot = var_ptovr;
        *var_ptovr_dn0_slot = var_ptovr_dn0;
        *var_ptovr_dn10_slot = var_ptovr_dn10;
        *var_ptovr_dn11_slot = var_ptovr_dn11;
        *var_ptovr_dn14_slot = var_ptovr_dn14;
        *var_ptovr_dn2_slot = var_ptovr_dn2;
        *var_ptovr_dn4_slot = var_ptovr_dn4;
        *var_ptovr_dn5_slot = var_ptovr_dn5;
        *var_ptovr_dn6_slot = var_ptovr_dn6;
        *var_ptovr_dn7_slot = var_ptovr_dn7;
        *var_ptovr_dn8_slot = var_ptovr_dn8;
        *var_ptovr_dn9_slot = var_ptovr_dn9;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_uc_depmue0_slot = var_uc_depmue0;
        *var_uc_depmue0_dn0_slot = var_uc_depmue0_dn0;
        *var_uc_depmue0_dn10_slot = var_uc_depmue0_dn10;
        *var_uc_depmue0_dn11_slot = var_uc_depmue0_dn11;
        *var_uc_depmue0_dn14_slot = var_uc_depmue0_dn14;
        *var_uc_depmue0_dn2_slot = var_uc_depmue0_dn2;
        *var_uc_depmue0_dn4_slot = var_uc_depmue0_dn4;
        *var_uc_depmue0_dn5_slot = var_uc_depmue0_dn5;
        *var_uc_depmue0_dn6_slot = var_uc_depmue0_dn6;
        *var_uc_depmue0_dn7_slot = var_uc_depmue0_dn7;
        *var_uc_depmue0_dn8_slot = var_uc_depmue0_dn8;
        *var_uc_depmue0_dn9_slot = var_uc_depmue0_dn9;
        *var_uc_depmue2_slot = var_uc_depmue2;
        *var_uc_depmue2_dn0_slot = var_uc_depmue2_dn0;
        *var_uc_depmue2_dn10_slot = var_uc_depmue2_dn10;
        *var_uc_depmue2_dn11_slot = var_uc_depmue2_dn11;
        *var_uc_depmue2_dn14_slot = var_uc_depmue2_dn14;
        *var_uc_depmue2_dn2_slot = var_uc_depmue2_dn2;
        *var_uc_depmue2_dn4_slot = var_uc_depmue2_dn4;
        *var_uc_depmue2_dn5_slot = var_uc_depmue2_dn5;
        *var_uc_depmue2_dn6_slot = var_uc_depmue2_dn6;
        *var_uc_depmue2_dn7_slot = var_uc_depmue2_dn7;
        *var_uc_depmue2_dn8_slot = var_uc_depmue2_dn8;
        *var_uc_depmue2_dn9_slot = var_uc_depmue2_dn9;
        *var_uc_depvmax_slot = var_uc_depvmax;
        *var_uc_depvmax_dn0_slot = var_uc_depvmax_dn0;
        *var_uc_depvmax_dn10_slot = var_uc_depvmax_dn10;
        *var_uc_depvmax_dn11_slot = var_uc_depvmax_dn11;
        *var_uc_depvmax_dn14_slot = var_uc_depvmax_dn14;
        *var_uc_depvmax_dn2_slot = var_uc_depvmax_dn2;
        *var_uc_depvmax_dn4_slot = var_uc_depvmax_dn4;
        *var_uc_depvmax_dn5_slot = var_uc_depvmax_dn5;
        *var_uc_depvmax_dn6_slot = var_uc_depvmax_dn6;
        *var_uc_depvmax_dn7_slot = var_uc_depvmax_dn7;
        *var_uc_depvmax_dn8_slot = var_uc_depvmax_dn8;
        *var_uc_depvmax_dn9_slot = var_uc_depvmax_dn9;
        *var_uc_depwlp_slot = var_uc_depwlp;
        *var_uc_depwlp_dn0_slot = var_uc_depwlp_dn0;
        *var_uc_depwlp_dn10_slot = var_uc_depwlp_dn10;
        *var_uc_depwlp_dn11_slot = var_uc_depwlp_dn11;
        *var_uc_depwlp_dn14_slot = var_uc_depwlp_dn14;
        *var_uc_depwlp_dn2_slot = var_uc_depwlp_dn2;
        *var_uc_depwlp_dn4_slot = var_uc_depwlp_dn4;
        *var_uc_depwlp_dn5_slot = var_uc_depwlp_dn5;
        *var_uc_depwlp_dn6_slot = var_uc_depwlp_dn6;
        *var_uc_depwlp_dn7_slot = var_uc_depwlp_dn7;
        *var_uc_depwlp_dn8_slot = var_uc_depwlp_dn8;
        *var_uc_depwlp_dn9_slot = var_uc_depwlp_dn9;
        *var_vbipn_slot = var_vbipn;
        *var_vbipn_dn0_slot = var_vbipn_dn0;
        *var_vbipn_dn10_slot = var_vbipn_dn10;
        *var_vbipn_dn11_slot = var_vbipn_dn11;
        *var_vbipn_dn14_slot = var_vbipn_dn14;
        *var_vbipn_dn2_slot = var_vbipn_dn2;
        *var_vbipn_dn4_slot = var_vbipn_dn4;
        *var_vbipn_dn5_slot = var_vbipn_dn5;
        *var_vbipn_dn6_slot = var_vbipn_dn6;
        *var_vbipn_dn7_slot = var_vbipn_dn7;
        *var_vbipn_dn8_slot = var_vbipn_dn8;
        *var_vbipn_dn9_slot = var_vbipn_dn9;
    }

    pub(super) fn stamp_transient_block_24(
        p: &Parameters,
        var_guard293: f64,
        var_ninvd0: f64,
        var_ninvd0cres: f64,
        var_ninvd0cres_dn0: f64,
        var_ninvd0cres_dn10: f64,
        var_ninvd0cres_dn11: f64,
        var_ninvd0cres_dn14: f64,
        var_ninvd0cres_dn2: f64,
        var_ninvd0cres_dn4: f64,
        var_ninvd0cres_dn5: f64,
        var_ninvd0cres_dn6: f64,
        var_ninvd0cres_dn7: f64,
        var_ninvd0cres_dn8: f64,
        var_ninvd0cres_dn9: f64,
        var_ninvd0hres: f64,
        var_ninvd0hres_dn0: f64,
        var_ninvd0hres_dn10: f64,
        var_ninvd0hres_dn11: f64,
        var_ninvd0hres_dn14: f64,
        var_ninvd0hres_dn2: f64,
        var_ninvd0hres_dn4: f64,
        var_ninvd0hres_dn5: f64,
        var_ninvd0hres_dn6: f64,
        var_ninvd0hres_dn7: f64,
        var_ninvd0hres_dn8: f64,
        var_ninvd0hres_dn9: f64,
        var_rthtemp0: f64,
        var_tdiff: f64,
        var_tdiff0: f64,
        var_tdiff0_2: f64,
        var_tdiff0_2_dn0: f64,
        var_tdiff0_2_dn10: f64,
        var_tdiff0_2_dn11: f64,
        var_tdiff0_2_dn14: f64,
        var_tdiff0_2_dn2: f64,
        var_tdiff0_2_dn4: f64,
        var_tdiff0_2_dn5: f64,
        var_tdiff0_2_dn6: f64,
        var_tdiff0_2_dn7: f64,
        var_tdiff0_2_dn8: f64,
        var_tdiff0_2_dn9: f64,
        var_tdiff0_dn0: f64,
        var_tdiff0_dn10: f64,
        var_tdiff0_dn11: f64,
        var_tdiff0_dn14: f64,
        var_tdiff0_dn2: f64,
        var_tdiff0_dn4: f64,
        var_tdiff0_dn5: f64,
        var_tdiff0_dn6: f64,
        var_tdiff0_dn7: f64,
        var_tdiff0_dn8: f64,
        var_tdiff0_dn9: f64,
        var_tdiff_2: f64,
        var_tdiff_2_dn0: f64,
        var_tdiff_2_dn10: f64,
        var_tdiff_2_dn11: f64,
        var_tdiff_2_dn14: f64,
        var_tdiff_2_dn2: f64,
        var_tdiff_2_dn4: f64,
        var_tdiff_2_dn5: f64,
        var_tdiff_2_dn6: f64,
        var_tdiff_2_dn7: f64,
        var_tdiff_2_dn8: f64,
        var_tdiff_2_dn9: f64,
        var_tdiff_dn0: f64,
        var_tdiff_dn10: f64,
        var_tdiff_dn11: f64,
        var_tdiff_dn14: f64,
        var_tdiff_dn2: f64,
        var_tdiff_dn4: f64,
        var_tdiff_dn5: f64,
        var_tdiff_dn6: f64,
        var_tdiff_dn7: f64,
        var_tdiff_dn8: f64,
        var_tdiff_dn9: f64,
        var_uc_powrat: f64,
        var_uc_rth0: f64,
        var_uc_vmax: f64,
        var_uc_vtmp: f64,
        var_vmax0: f64,
        var_guard300_slot: &mut f64,
        var_guard302_slot: &mut f64,
        var_guard304_slot: &mut f64,
        var_guard306_slot: &mut f64,
        var_guard308_slot: &mut f64,
        var_guard310_slot: &mut f64,
        var_ninvde_slot: &mut f64,
        var_ninvde_dn0_slot: &mut f64,
        var_ninvde_dn10_slot: &mut f64,
        var_ninvde_dn11_slot: &mut f64,
        var_ninvde_dn14_slot: &mut f64,
        var_ninvde_dn2_slot: &mut f64,
        var_ninvde_dn4_slot: &mut f64,
        var_ninvde_dn5_slot: &mut f64,
        var_ninvde_dn6_slot: &mut f64,
        var_ninvde_dn7_slot: &mut f64,
        var_ninvde_dn8_slot: &mut f64,
        var_ninvde_dn9_slot: &mut f64,
        var_ninvdecres_slot: &mut f64,
        var_ninvdecres_dn0_slot: &mut f64,
        var_ninvdecres_dn10_slot: &mut f64,
        var_ninvdecres_dn11_slot: &mut f64,
        var_ninvdecres_dn14_slot: &mut f64,
        var_ninvdecres_dn2_slot: &mut f64,
        var_ninvdecres_dn4_slot: &mut f64,
        var_ninvdecres_dn5_slot: &mut f64,
        var_ninvdecres_dn6_slot: &mut f64,
        var_ninvdecres_dn7_slot: &mut f64,
        var_ninvdecres_dn8_slot: &mut f64,
        var_ninvdecres_dn9_slot: &mut f64,
        var_ninvdehres_slot: &mut f64,
        var_ninvdehres_dn0_slot: &mut f64,
        var_ninvdehres_dn10_slot: &mut f64,
        var_ninvdehres_dn11_slot: &mut f64,
        var_ninvdehres_dn14_slot: &mut f64,
        var_ninvdehres_dn2_slot: &mut f64,
        var_ninvdehres_dn4_slot: &mut f64,
        var_ninvdehres_dn5_slot: &mut f64,
        var_ninvdehres_dn6_slot: &mut f64,
        var_ninvdehres_dn7_slot: &mut f64,
        var_ninvdehres_dn8_slot: &mut f64,
        var_ninvdehres_dn9_slot: &mut f64,
        var_rth_slot: &mut f64,
        var_rth_dn0_slot: &mut f64,
        var_rth_dn10_slot: &mut f64,
        var_rth_dn11_slot: &mut f64,
        var_rth_dn14_slot: &mut f64,
        var_rth_dn2_slot: &mut f64,
        var_rth_dn4_slot: &mut f64,
        var_rth_dn5_slot: &mut f64,
        var_rth_dn6_slot: &mut f64,
        var_rth_dn7_slot: &mut f64,
        var_rth_dn8_slot: &mut f64,
        var_rth_dn9_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn14_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_dn9_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn14_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_dn9_slot: &mut f64,
        var_vmaxeff_slot: &mut f64,
        var_vmaxeff_dn0_slot: &mut f64,
        var_vmaxeff_dn10_slot: &mut f64,
        var_vmaxeff_dn11_slot: &mut f64,
        var_vmaxeff_dn14_slot: &mut f64,
        var_vmaxeff_dn2_slot: &mut f64,
        var_vmaxeff_dn4_slot: &mut f64,
        var_vmaxeff_dn5_slot: &mut f64,
        var_vmaxeff_dn6_slot: &mut f64,
        var_vmaxeff_dn7_slot: &mut f64,
        var_vmaxeff_dn8_slot: &mut f64,
        var_vmaxeff_dn9_slot: &mut f64,
    ) {
        let mut var_guard300: f64 = *var_guard300_slot;
        let mut var_guard302: f64 = *var_guard302_slot;
        let mut var_guard304: f64 = *var_guard304_slot;
        let mut var_guard306: f64 = *var_guard306_slot;
        let mut var_guard308: f64 = *var_guard308_slot;
        let mut var_guard310: f64 = *var_guard310_slot;
        let mut var_ninvde: f64 = *var_ninvde_slot;
        let mut var_ninvde_dn0: f64 = *var_ninvde_dn0_slot;
        let mut var_ninvde_dn10: f64 = *var_ninvde_dn10_slot;
        let mut var_ninvde_dn11: f64 = *var_ninvde_dn11_slot;
        let mut var_ninvde_dn14: f64 = *var_ninvde_dn14_slot;
        let mut var_ninvde_dn2: f64 = *var_ninvde_dn2_slot;
        let mut var_ninvde_dn4: f64 = *var_ninvde_dn4_slot;
        let mut var_ninvde_dn5: f64 = *var_ninvde_dn5_slot;
        let mut var_ninvde_dn6: f64 = *var_ninvde_dn6_slot;
        let mut var_ninvde_dn7: f64 = *var_ninvde_dn7_slot;
        let mut var_ninvde_dn8: f64 = *var_ninvde_dn8_slot;
        let mut var_ninvde_dn9: f64 = *var_ninvde_dn9_slot;
        let mut var_ninvdecres: f64 = *var_ninvdecres_slot;
        let mut var_ninvdecres_dn0: f64 = *var_ninvdecres_dn0_slot;
        let mut var_ninvdecres_dn10: f64 = *var_ninvdecres_dn10_slot;
        let mut var_ninvdecres_dn11: f64 = *var_ninvdecres_dn11_slot;
        let mut var_ninvdecres_dn14: f64 = *var_ninvdecres_dn14_slot;
        let mut var_ninvdecres_dn2: f64 = *var_ninvdecres_dn2_slot;
        let mut var_ninvdecres_dn4: f64 = *var_ninvdecres_dn4_slot;
        let mut var_ninvdecres_dn5: f64 = *var_ninvdecres_dn5_slot;
        let mut var_ninvdecres_dn6: f64 = *var_ninvdecres_dn6_slot;
        let mut var_ninvdecres_dn7: f64 = *var_ninvdecres_dn7_slot;
        let mut var_ninvdecres_dn8: f64 = *var_ninvdecres_dn8_slot;
        let mut var_ninvdecres_dn9: f64 = *var_ninvdecres_dn9_slot;
        let mut var_ninvdehres: f64 = *var_ninvdehres_slot;
        let mut var_ninvdehres_dn0: f64 = *var_ninvdehres_dn0_slot;
        let mut var_ninvdehres_dn10: f64 = *var_ninvdehres_dn10_slot;
        let mut var_ninvdehres_dn11: f64 = *var_ninvdehres_dn11_slot;
        let mut var_ninvdehres_dn14: f64 = *var_ninvdehres_dn14_slot;
        let mut var_ninvdehres_dn2: f64 = *var_ninvdehres_dn2_slot;
        let mut var_ninvdehres_dn4: f64 = *var_ninvdehres_dn4_slot;
        let mut var_ninvdehres_dn5: f64 = *var_ninvdehres_dn5_slot;
        let mut var_ninvdehres_dn6: f64 = *var_ninvdehres_dn6_slot;
        let mut var_ninvdehres_dn7: f64 = *var_ninvdehres_dn7_slot;
        let mut var_ninvdehres_dn8: f64 = *var_ninvdehres_dn8_slot;
        let mut var_ninvdehres_dn9: f64 = *var_ninvdehres_dn9_slot;
        let mut var_rth: f64 = *var_rth_slot;
        let mut var_rth_dn0: f64 = *var_rth_dn0_slot;
        let mut var_rth_dn10: f64 = *var_rth_dn10_slot;
        let mut var_rth_dn11: f64 = *var_rth_dn11_slot;
        let mut var_rth_dn14: f64 = *var_rth_dn14_slot;
        let mut var_rth_dn2: f64 = *var_rth_dn2_slot;
        let mut var_rth_dn4: f64 = *var_rth_dn4_slot;
        let mut var_rth_dn5: f64 = *var_rth_dn5_slot;
        let mut var_rth_dn6: f64 = *var_rth_dn6_slot;
        let mut var_rth_dn7: f64 = *var_rth_dn7_slot;
        let mut var_rth_dn8: f64 = *var_rth_dn8_slot;
        let mut var_rth_dn9: f64 = *var_rth_dn9_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn14: f64 = *var_tmf1_dn14_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_dn9: f64 = *var_tmf1_dn9_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn14: f64 = *var_tmf2_dn14_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_dn9: f64 = *var_tmf2_dn9_slot;
        let mut var_vmaxeff: f64 = *var_vmaxeff_slot;
        let mut var_vmaxeff_dn0: f64 = *var_vmaxeff_dn0_slot;
        let mut var_vmaxeff_dn10: f64 = *var_vmaxeff_dn10_slot;
        let mut var_vmaxeff_dn11: f64 = *var_vmaxeff_dn11_slot;
        let mut var_vmaxeff_dn14: f64 = *var_vmaxeff_dn14_slot;
        let mut var_vmaxeff_dn2: f64 = *var_vmaxeff_dn2_slot;
        let mut var_vmaxeff_dn4: f64 = *var_vmaxeff_dn4_slot;
        let mut var_vmaxeff_dn5: f64 = *var_vmaxeff_dn5_slot;
        let mut var_vmaxeff_dn6: f64 = *var_vmaxeff_dn6_slot;
        let mut var_vmaxeff_dn7: f64 = *var_vmaxeff_dn7_slot;
        let mut var_vmaxeff_dn8: f64 = *var_vmaxeff_dn8_slot;
        let mut var_vmaxeff_dn9: f64 = *var_vmaxeff_dn9_slot;

        let (assign13570_e7915, assign13570_e7915_d_n0, assign13570_e7915_d_n2, assign13570_e7915_d_n4, assign13570_e7915_d_n5, assign13570_e7915_d_n6, assign13570_e7915_d_n7, assign13570_e7915_d_n8, assign13570_e7915_d_n9, assign13570_e7915_d_n10, assign13570_e7915_d_n11, assign13570_e7915_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13570_e7900: f64 = (0.4 * var_t1);
        let assign13570_e7901: f64 = (1.8 + assign13570_e7900);
        let assign13570_e7904: f64 = (0.1 * var_t1);
        let assign13570_e7906: f64 = (assign13570_e7904 * var_t1);
        let assign13570_e7907: f64 = (assign13570_e7901 + assign13570_e7906);
        let assign13570_e7911: f64 = (1.0 - var_t1);
        let assign13570_e7912: f64 = (var_uc_vtmp * assign13570_e7911);
        let assign13570_e7913: f64 = (assign13570_e7907 - assign13570_e7912);
        (assign13570_e7913, (((0.4 * var_t1_dn0) + (((0.1 * var_t1_dn0) * var_t1) + (assign13570_e7904 * var_t1_dn0))) - (var_uc_vtmp * (-var_t1_dn0))), (((0.4 * var_t1_dn2) + (((0.1 * var_t1_dn2) * var_t1) + (assign13570_e7904 * var_t1_dn2))) - (var_uc_vtmp * (-var_t1_dn2))), (((0.4 * var_t1_dn4) + (((0.1 * var_t1_dn4) * var_t1) + (assign13570_e7904 * var_t1_dn4))) - (var_uc_vtmp * (-var_t1_dn4))), (((0.4 * var_t1_dn5) + (((0.1 * var_t1_dn5) * var_t1) + (assign13570_e7904 * var_t1_dn5))) - (var_uc_vtmp * (-var_t1_dn5))), (((0.4 * var_t1_dn6) + (((0.1 * var_t1_dn6) * var_t1) + (assign13570_e7904 * var_t1_dn6))) - (var_uc_vtmp * (-var_t1_dn6))), (((0.4 * var_t1_dn7) + (((0.1 * var_t1_dn7) * var_t1) + (assign13570_e7904 * var_t1_dn7))) - (var_uc_vtmp * (-var_t1_dn7))), (((0.4 * var_t1_dn8) + (((0.1 * var_t1_dn8) * var_t1) + (assign13570_e7904 * var_t1_dn8))) - (var_uc_vtmp * (-var_t1_dn8))), (((0.4 * var_t1_dn9) + (((0.1 * var_t1_dn9) * var_t1) + (assign13570_e7904 * var_t1_dn9))) - (var_uc_vtmp * (-var_t1_dn9))), (((0.4 * var_t1_dn10) + (((0.1 * var_t1_dn10) * var_t1) + (assign13570_e7904 * var_t1_dn10))) - (var_uc_vtmp * (-var_t1_dn10))), (((0.4 * var_t1_dn11) + (((0.1 * var_t1_dn11) * var_t1) + (assign13570_e7904 * var_t1_dn11))) - (var_uc_vtmp * (-var_t1_dn11))), (((0.4 * var_t1_dn14) + (((0.1 * var_t1_dn14) * var_t1) + (assign13570_e7904 * var_t1_dn14))) - (var_uc_vtmp * (-var_t1_dn14))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign13570_e7915;
        var_t0_dn0 = assign13570_e7915_d_n0;
        var_t0_dn2 = assign13570_e7915_d_n2;
        var_t0_dn4 = assign13570_e7915_d_n4;
        var_t0_dn5 = assign13570_e7915_d_n5;
        var_t0_dn6 = assign13570_e7915_d_n6;
        var_t0_dn7 = assign13570_e7915_d_n7;
        var_t0_dn8 = assign13570_e7915_d_n8;
        var_t0_dn9 = assign13570_e7915_d_n9;
        var_t0_dn10 = assign13570_e7915_d_n10;
        var_t0_dn11 = assign13570_e7915_d_n11;
        var_t0_dn14 = assign13570_e7915_d_n14;

        let assign13580_e7918: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        var_guard300 = assign13580_e7918;

        let (assign13590_e7938, assign13590_e7938_d_n0, assign13590_e7938_d_n2, assign13590_e7938_d_n4, assign13590_e7938_d_n5, assign13590_e7938_d_n6, assign13590_e7938_d_n7, assign13590_e7938_d_n8, assign13590_e7938_d_n9, assign13590_e7938_d_n10, assign13590_e7938_d_n11, assign13590_e7938_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard300 != 0.0)) {
        let assign13590_e7924: f64 = (var_vmax0 * var_uc_vmax);
        let assign13590_e7926: f64 = (assign13590_e7924 / var_t0);
        let assign13590_e7930: f64 = (p.p90 * var_tdiff0);
        let assign13590_e7931: f64 = (1.0 + assign13590_e7930);
        let assign13590_e7934: f64 = (p.p91 * var_tdiff0_2);
        let assign13590_e7935: f64 = (assign13590_e7931 + assign13590_e7934);
        let assign13590_e7936: f64 = (assign13590_e7926 * assign13590_e7935);
        (assign13590_e7936, (((-((assign13590_e7924 * var_t0_dn0) / (var_t0 * var_t0))) * assign13590_e7935) + (assign13590_e7926 * ((p.p90 * var_tdiff0_dn0) + (p.p91 * var_tdiff0_2_dn0)))), (((-((assign13590_e7924 * var_t0_dn2) / (var_t0 * var_t0))) * assign13590_e7935) + (assign13590_e7926 * ((p.p90 * var_tdiff0_dn2) + (p.p91 * var_tdiff0_2_dn2)))), (((-((assign13590_e7924 * var_t0_dn4) / (var_t0 * var_t0))) * assign13590_e7935) + (assign13590_e7926 * ((p.p90 * var_tdiff0_dn4) + (p.p91 * var_tdiff0_2_dn4)))), (((-((assign13590_e7924 * var_t0_dn5) / (var_t0 * var_t0))) * assign13590_e7935) + (assign13590_e7926 * ((p.p90 * var_tdiff0_dn5) + (p.p91 * var_tdiff0_2_dn5)))), (((-((assign13590_e7924 * var_t0_dn6) / (var_t0 * var_t0))) * assign13590_e7935) + (assign13590_e7926 * ((p.p90 * var_tdiff0_dn6) + (p.p91 * var_tdiff0_2_dn6)))), (((-((assign13590_e7924 * var_t0_dn7) / (var_t0 * var_t0))) * assign13590_e7935) + (assign13590_e7926 * ((p.p90 * var_tdiff0_dn7) + (p.p91 * var_tdiff0_2_dn7)))), (((-((assign13590_e7924 * var_t0_dn8) / (var_t0 * var_t0))) * assign13590_e7935) + (assign13590_e7926 * ((p.p90 * var_tdiff0_dn8) + (p.p91 * var_tdiff0_2_dn8)))), (((-((assign13590_e7924 * var_t0_dn9) / (var_t0 * var_t0))) * assign13590_e7935) + (assign13590_e7926 * ((p.p90 * var_tdiff0_dn9) + (p.p91 * var_tdiff0_2_dn9)))), (((-((assign13590_e7924 * var_t0_dn10) / (var_t0 * var_t0))) * assign13590_e7935) + (assign13590_e7926 * ((p.p90 * var_tdiff0_dn10) + (p.p91 * var_tdiff0_2_dn10)))), (((-((assign13590_e7924 * var_t0_dn11) / (var_t0 * var_t0))) * assign13590_e7935) + (assign13590_e7926 * ((p.p90 * var_tdiff0_dn11) + (p.p91 * var_tdiff0_2_dn11)))), (((-((assign13590_e7924 * var_t0_dn14) / (var_t0 * var_t0))) * assign13590_e7935) + (assign13590_e7926 * ((p.p90 * var_tdiff0_dn14) + (p.p91 * var_tdiff0_2_dn14)))),)
    } else {
        (var_vmaxeff, var_vmaxeff_dn0, var_vmaxeff_dn2, var_vmaxeff_dn4, var_vmaxeff_dn5, var_vmaxeff_dn6, var_vmaxeff_dn7, var_vmaxeff_dn8, var_vmaxeff_dn9, var_vmaxeff_dn10, var_vmaxeff_dn11, var_vmaxeff_dn14,)
    }
};
        var_vmaxeff = assign13590_e7938;
        var_vmaxeff_dn0 = assign13590_e7938_d_n0;
        var_vmaxeff_dn2 = assign13590_e7938_d_n2;
        var_vmaxeff_dn4 = assign13590_e7938_d_n4;
        var_vmaxeff_dn5 = assign13590_e7938_d_n5;
        var_vmaxeff_dn6 = assign13590_e7938_d_n6;
        var_vmaxeff_dn7 = assign13590_e7938_d_n7;
        var_vmaxeff_dn8 = assign13590_e7938_d_n8;
        var_vmaxeff_dn9 = assign13590_e7938_d_n9;
        var_vmaxeff_dn10 = assign13590_e7938_d_n10;
        var_vmaxeff_dn11 = assign13590_e7938_d_n11;
        var_vmaxeff_dn14 = assign13590_e7938_d_n14;

        let (assign13600_e7959, assign13600_e7959_d_n0, assign13600_e7959_d_n2, assign13600_e7959_d_n4, assign13600_e7959_d_n5, assign13600_e7959_d_n6, assign13600_e7959_d_n7, assign13600_e7959_d_n8, assign13600_e7959_d_n9, assign13600_e7959_d_n10, assign13600_e7959_d_n11, assign13600_e7959_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard300 == 0.0)) {
        let assign13600_e7945: f64 = (var_vmax0 * var_uc_vmax);
        let assign13600_e7947: f64 = (assign13600_e7945 / var_t0);
        let assign13600_e7951: f64 = (p.p90 * var_tdiff);
        let assign13600_e7952: f64 = (1.0 + assign13600_e7951);
        let assign13600_e7955: f64 = (p.p91 * var_tdiff_2);
        let assign13600_e7956: f64 = (assign13600_e7952 + assign13600_e7955);
        let assign13600_e7957: f64 = (assign13600_e7947 * assign13600_e7956);
        (assign13600_e7957, (((-((assign13600_e7945 * var_t0_dn0) / (var_t0 * var_t0))) * assign13600_e7956) + (assign13600_e7947 * ((p.p90 * var_tdiff_dn0) + (p.p91 * var_tdiff_2_dn0)))), (((-((assign13600_e7945 * var_t0_dn2) / (var_t0 * var_t0))) * assign13600_e7956) + (assign13600_e7947 * ((p.p90 * var_tdiff_dn2) + (p.p91 * var_tdiff_2_dn2)))), (((-((assign13600_e7945 * var_t0_dn4) / (var_t0 * var_t0))) * assign13600_e7956) + (assign13600_e7947 * ((p.p90 * var_tdiff_dn4) + (p.p91 * var_tdiff_2_dn4)))), (((-((assign13600_e7945 * var_t0_dn5) / (var_t0 * var_t0))) * assign13600_e7956) + (assign13600_e7947 * ((p.p90 * var_tdiff_dn5) + (p.p91 * var_tdiff_2_dn5)))), (((-((assign13600_e7945 * var_t0_dn6) / (var_t0 * var_t0))) * assign13600_e7956) + (assign13600_e7947 * ((p.p90 * var_tdiff_dn6) + (p.p91 * var_tdiff_2_dn6)))), (((-((assign13600_e7945 * var_t0_dn7) / (var_t0 * var_t0))) * assign13600_e7956) + (assign13600_e7947 * ((p.p90 * var_tdiff_dn7) + (p.p91 * var_tdiff_2_dn7)))), (((-((assign13600_e7945 * var_t0_dn8) / (var_t0 * var_t0))) * assign13600_e7956) + (assign13600_e7947 * ((p.p90 * var_tdiff_dn8) + (p.p91 * var_tdiff_2_dn8)))), (((-((assign13600_e7945 * var_t0_dn9) / (var_t0 * var_t0))) * assign13600_e7956) + (assign13600_e7947 * ((p.p90 * var_tdiff_dn9) + (p.p91 * var_tdiff_2_dn9)))), (((-((assign13600_e7945 * var_t0_dn10) / (var_t0 * var_t0))) * assign13600_e7956) + (assign13600_e7947 * ((p.p90 * var_tdiff_dn10) + (p.p91 * var_tdiff_2_dn10)))), (((-((assign13600_e7945 * var_t0_dn11) / (var_t0 * var_t0))) * assign13600_e7956) + (assign13600_e7947 * ((p.p90 * var_tdiff_dn11) + (p.p91 * var_tdiff_2_dn11)))), (((-((assign13600_e7945 * var_t0_dn14) / (var_t0 * var_t0))) * assign13600_e7956) + (assign13600_e7947 * ((p.p90 * var_tdiff_dn14) + (p.p91 * var_tdiff_2_dn14)))),)
    } else {
        (var_vmaxeff, var_vmaxeff_dn0, var_vmaxeff_dn2, var_vmaxeff_dn4, var_vmaxeff_dn5, var_vmaxeff_dn6, var_vmaxeff_dn7, var_vmaxeff_dn8, var_vmaxeff_dn9, var_vmaxeff_dn10, var_vmaxeff_dn11, var_vmaxeff_dn14,)
    }
};
        var_vmaxeff = assign13600_e7959;
        var_vmaxeff_dn0 = assign13600_e7959_d_n0;
        var_vmaxeff_dn2 = assign13600_e7959_d_n2;
        var_vmaxeff_dn4 = assign13600_e7959_d_n4;
        var_vmaxeff_dn5 = assign13600_e7959_d_n5;
        var_vmaxeff_dn6 = assign13600_e7959_d_n6;
        var_vmaxeff_dn7 = assign13600_e7959_d_n7;
        var_vmaxeff_dn8 = assign13600_e7959_d_n8;
        var_vmaxeff_dn9 = assign13600_e7959_d_n9;
        var_vmaxeff_dn10 = assign13600_e7959_d_n10;
        var_vmaxeff_dn11 = assign13600_e7959_d_n11;
        var_vmaxeff_dn14 = assign13600_e7959_d_n14;

        let assign13620_e7967: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        var_guard302 = assign13620_e7967;

        let (assign13630_e7983, assign13630_e7983_d_n0, assign13630_e7983_d_n2, assign13630_e7983_d_n4, assign13630_e7983_d_n5, assign13630_e7983_d_n6, assign13630_e7983_d_n7, assign13630_e7983_d_n8, assign13630_e7983_d_n9, assign13630_e7983_d_n10, assign13630_e7983_d_n11, assign13630_e7983_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard302 != 0.0)) {
        let assign13630_e7975: f64 = (p.p324 * var_tdiff0);
        let assign13630_e7976: f64 = (1.0 + assign13630_e7975);
        let assign13630_e7979: f64 = (p.p325 * var_tdiff0_2);
        let assign13630_e7980: f64 = (assign13630_e7976 + assign13630_e7979);
        let assign13630_e7981: f64 = (var_ninvd0 * assign13630_e7980);
        (assign13630_e7981, (var_ninvd0 * ((p.p324 * var_tdiff0_dn0) + (p.p325 * var_tdiff0_2_dn0))), (var_ninvd0 * ((p.p324 * var_tdiff0_dn2) + (p.p325 * var_tdiff0_2_dn2))), (var_ninvd0 * ((p.p324 * var_tdiff0_dn4) + (p.p325 * var_tdiff0_2_dn4))), (var_ninvd0 * ((p.p324 * var_tdiff0_dn5) + (p.p325 * var_tdiff0_2_dn5))), (var_ninvd0 * ((p.p324 * var_tdiff0_dn6) + (p.p325 * var_tdiff0_2_dn6))), (var_ninvd0 * ((p.p324 * var_tdiff0_dn7) + (p.p325 * var_tdiff0_2_dn7))), (var_ninvd0 * ((p.p324 * var_tdiff0_dn8) + (p.p325 * var_tdiff0_2_dn8))), (var_ninvd0 * ((p.p324 * var_tdiff0_dn9) + (p.p325 * var_tdiff0_2_dn9))), (var_ninvd0 * ((p.p324 * var_tdiff0_dn10) + (p.p325 * var_tdiff0_2_dn10))), (var_ninvd0 * ((p.p324 * var_tdiff0_dn11) + (p.p325 * var_tdiff0_2_dn11))), (var_ninvd0 * ((p.p324 * var_tdiff0_dn14) + (p.p325 * var_tdiff0_2_dn14))),)
    } else {
        (var_ninvde, var_ninvde_dn0, var_ninvde_dn2, var_ninvde_dn4, var_ninvde_dn5, var_ninvde_dn6, var_ninvde_dn7, var_ninvde_dn8, var_ninvde_dn9, var_ninvde_dn10, var_ninvde_dn11, var_ninvde_dn14,)
    }
};
        var_ninvde = assign13630_e7983;
        var_ninvde_dn0 = assign13630_e7983_d_n0;
        var_ninvde_dn2 = assign13630_e7983_d_n2;
        var_ninvde_dn4 = assign13630_e7983_d_n4;
        var_ninvde_dn5 = assign13630_e7983_d_n5;
        var_ninvde_dn6 = assign13630_e7983_d_n6;
        var_ninvde_dn7 = assign13630_e7983_d_n7;
        var_ninvde_dn8 = assign13630_e7983_d_n8;
        var_ninvde_dn9 = assign13630_e7983_d_n9;
        var_ninvde_dn10 = assign13630_e7983_d_n10;
        var_ninvde_dn11 = assign13630_e7983_d_n11;
        var_ninvde_dn14 = assign13630_e7983_d_n14;

        let (assign13640_e7997, assign13640_e7997_d_n0, assign13640_e7997_d_n2, assign13640_e7997_d_n4, assign13640_e7997_d_n5, assign13640_e7997_d_n6, assign13640_e7997_d_n7, assign13640_e7997_d_n8, assign13640_e7997_d_n9, assign13640_e7997_d_n10, assign13640_e7997_d_n11, assign13640_e7997_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard302 != 0.0)) {
        let assign13640_e7990: f64 = (p.p390 * var_tdiff0);
        let assign13640_e7991: f64 = (1.0 + assign13640_e7990);
        let assign13640_e7994: f64 = (p.p391 * var_tdiff0_2);
        let assign13640_e7995: f64 = (assign13640_e7991 + assign13640_e7994);
        (assign13640_e7995, ((p.p390 * var_tdiff0_dn0) + (p.p391 * var_tdiff0_2_dn0)), ((p.p390 * var_tdiff0_dn2) + (p.p391 * var_tdiff0_2_dn2)), ((p.p390 * var_tdiff0_dn4) + (p.p391 * var_tdiff0_2_dn4)), ((p.p390 * var_tdiff0_dn5) + (p.p391 * var_tdiff0_2_dn5)), ((p.p390 * var_tdiff0_dn6) + (p.p391 * var_tdiff0_2_dn6)), ((p.p390 * var_tdiff0_dn7) + (p.p391 * var_tdiff0_2_dn7)), ((p.p390 * var_tdiff0_dn8) + (p.p391 * var_tdiff0_2_dn8)), ((p.p390 * var_tdiff0_dn9) + (p.p391 * var_tdiff0_2_dn9)), ((p.p390 * var_tdiff0_dn10) + (p.p391 * var_tdiff0_2_dn10)), ((p.p390 * var_tdiff0_dn11) + (p.p391 * var_tdiff0_2_dn11)), ((p.p390 * var_tdiff0_dn14) + (p.p391 * var_tdiff0_2_dn14)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign13640_e7997;
        var_t1_dn0 = assign13640_e7997_d_n0;
        var_t1_dn2 = assign13640_e7997_d_n2;
        var_t1_dn4 = assign13640_e7997_d_n4;
        var_t1_dn5 = assign13640_e7997_d_n5;
        var_t1_dn6 = assign13640_e7997_d_n6;
        var_t1_dn7 = assign13640_e7997_d_n7;
        var_t1_dn8 = assign13640_e7997_d_n8;
        var_t1_dn9 = assign13640_e7997_d_n9;
        var_t1_dn10 = assign13640_e7997_d_n10;
        var_t1_dn11 = assign13640_e7997_d_n11;
        var_t1_dn14 = assign13640_e7997_d_n14;

        let (assign13650_e8005, assign13650_e8005_d_n0, assign13650_e8005_d_n2, assign13650_e8005_d_n4, assign13650_e8005_d_n5, assign13650_e8005_d_n6, assign13650_e8005_d_n7, assign13650_e8005_d_n8, assign13650_e8005_d_n9, assign13650_e8005_d_n10, assign13650_e8005_d_n11, assign13650_e8005_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard302 != 0.0)) {
        let assign13650_e8003: f64 = (var_ninvd0cres * var_t1);
        (assign13650_e8003, ((var_ninvd0cres_dn0 * var_t1) + (var_ninvd0cres * var_t1_dn0)), ((var_ninvd0cres_dn2 * var_t1) + (var_ninvd0cres * var_t1_dn2)), ((var_ninvd0cres_dn4 * var_t1) + (var_ninvd0cres * var_t1_dn4)), ((var_ninvd0cres_dn5 * var_t1) + (var_ninvd0cres * var_t1_dn5)), ((var_ninvd0cres_dn6 * var_t1) + (var_ninvd0cres * var_t1_dn6)), ((var_ninvd0cres_dn7 * var_t1) + (var_ninvd0cres * var_t1_dn7)), ((var_ninvd0cres_dn8 * var_t1) + (var_ninvd0cres * var_t1_dn8)), ((var_ninvd0cres_dn9 * var_t1) + (var_ninvd0cres * var_t1_dn9)), ((var_ninvd0cres_dn10 * var_t1) + (var_ninvd0cres * var_t1_dn10)), ((var_ninvd0cres_dn11 * var_t1) + (var_ninvd0cres * var_t1_dn11)), ((var_ninvd0cres_dn14 * var_t1) + (var_ninvd0cres * var_t1_dn14)),)
    } else {
        (var_ninvdecres, var_ninvdecres_dn0, var_ninvdecres_dn2, var_ninvdecres_dn4, var_ninvdecres_dn5, var_ninvdecres_dn6, var_ninvdecres_dn7, var_ninvdecres_dn8, var_ninvdecres_dn9, var_ninvdecres_dn10, var_ninvdecres_dn11, var_ninvdecres_dn14,)
    }
};
        var_ninvdecres = assign13650_e8005;
        var_ninvdecres_dn0 = assign13650_e8005_d_n0;
        var_ninvdecres_dn2 = assign13650_e8005_d_n2;
        var_ninvdecres_dn4 = assign13650_e8005_d_n4;
        var_ninvdecres_dn5 = assign13650_e8005_d_n5;
        var_ninvdecres_dn6 = assign13650_e8005_d_n6;
        var_ninvdecres_dn7 = assign13650_e8005_d_n7;
        var_ninvdecres_dn8 = assign13650_e8005_d_n8;
        var_ninvdecres_dn9 = assign13650_e8005_d_n9;
        var_ninvdecres_dn10 = assign13650_e8005_d_n10;
        var_ninvdecres_dn11 = assign13650_e8005_d_n11;
        var_ninvdecres_dn14 = assign13650_e8005_d_n14;

        let (assign13660_e8013, assign13660_e8013_d_n0, assign13660_e8013_d_n2, assign13660_e8013_d_n4, assign13660_e8013_d_n5, assign13660_e8013_d_n6, assign13660_e8013_d_n7, assign13660_e8013_d_n8, assign13660_e8013_d_n9, assign13660_e8013_d_n10, assign13660_e8013_d_n11, assign13660_e8013_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard302 != 0.0)) {
        let assign13660_e8011: f64 = (var_ninvd0hres * var_t1);
        (assign13660_e8011, ((var_ninvd0hres_dn0 * var_t1) + (var_ninvd0hres * var_t1_dn0)), ((var_ninvd0hres_dn2 * var_t1) + (var_ninvd0hres * var_t1_dn2)), ((var_ninvd0hres_dn4 * var_t1) + (var_ninvd0hres * var_t1_dn4)), ((var_ninvd0hres_dn5 * var_t1) + (var_ninvd0hres * var_t1_dn5)), ((var_ninvd0hres_dn6 * var_t1) + (var_ninvd0hres * var_t1_dn6)), ((var_ninvd0hres_dn7 * var_t1) + (var_ninvd0hres * var_t1_dn7)), ((var_ninvd0hres_dn8 * var_t1) + (var_ninvd0hres * var_t1_dn8)), ((var_ninvd0hres_dn9 * var_t1) + (var_ninvd0hres * var_t1_dn9)), ((var_ninvd0hres_dn10 * var_t1) + (var_ninvd0hres * var_t1_dn10)), ((var_ninvd0hres_dn11 * var_t1) + (var_ninvd0hres * var_t1_dn11)), ((var_ninvd0hres_dn14 * var_t1) + (var_ninvd0hres * var_t1_dn14)),)
    } else {
        (var_ninvdehres, var_ninvdehres_dn0, var_ninvdehres_dn2, var_ninvdehres_dn4, var_ninvdehres_dn5, var_ninvdehres_dn6, var_ninvdehres_dn7, var_ninvdehres_dn8, var_ninvdehres_dn9, var_ninvdehres_dn10, var_ninvdehres_dn11, var_ninvdehres_dn14,)
    }
};
        var_ninvdehres = assign13660_e8013;
        var_ninvdehres_dn0 = assign13660_e8013_d_n0;
        var_ninvdehres_dn2 = assign13660_e8013_d_n2;
        var_ninvdehres_dn4 = assign13660_e8013_d_n4;
        var_ninvdehres_dn5 = assign13660_e8013_d_n5;
        var_ninvdehres_dn6 = assign13660_e8013_d_n6;
        var_ninvdehres_dn7 = assign13660_e8013_d_n7;
        var_ninvdehres_dn8 = assign13660_e8013_d_n8;
        var_ninvdehres_dn9 = assign13660_e8013_d_n9;
        var_ninvdehres_dn10 = assign13660_e8013_d_n10;
        var_ninvdehres_dn11 = assign13660_e8013_d_n11;
        var_ninvdehres_dn14 = assign13660_e8013_d_n14;

        let (assign13670_e8030, assign13670_e8030_d_n0, assign13670_e8030_d_n2, assign13670_e8030_d_n4, assign13670_e8030_d_n5, assign13670_e8030_d_n6, assign13670_e8030_d_n7, assign13670_e8030_d_n8, assign13670_e8030_d_n9, assign13670_e8030_d_n10, assign13670_e8030_d_n11, assign13670_e8030_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard302 == 0.0)) {
        let assign13670_e8022: f64 = (p.p324 * var_tdiff);
        let assign13670_e8023: f64 = (1.0 + assign13670_e8022);
        let assign13670_e8026: f64 = (p.p325 * var_tdiff_2);
        let assign13670_e8027: f64 = (assign13670_e8023 + assign13670_e8026);
        let assign13670_e8028: f64 = (var_ninvd0 * assign13670_e8027);
        (assign13670_e8028, (var_ninvd0 * ((p.p324 * var_tdiff_dn0) + (p.p325 * var_tdiff_2_dn0))), (var_ninvd0 * ((p.p324 * var_tdiff_dn2) + (p.p325 * var_tdiff_2_dn2))), (var_ninvd0 * ((p.p324 * var_tdiff_dn4) + (p.p325 * var_tdiff_2_dn4))), (var_ninvd0 * ((p.p324 * var_tdiff_dn5) + (p.p325 * var_tdiff_2_dn5))), (var_ninvd0 * ((p.p324 * var_tdiff_dn6) + (p.p325 * var_tdiff_2_dn6))), (var_ninvd0 * ((p.p324 * var_tdiff_dn7) + (p.p325 * var_tdiff_2_dn7))), (var_ninvd0 * ((p.p324 * var_tdiff_dn8) + (p.p325 * var_tdiff_2_dn8))), (var_ninvd0 * ((p.p324 * var_tdiff_dn9) + (p.p325 * var_tdiff_2_dn9))), (var_ninvd0 * ((p.p324 * var_tdiff_dn10) + (p.p325 * var_tdiff_2_dn10))), (var_ninvd0 * ((p.p324 * var_tdiff_dn11) + (p.p325 * var_tdiff_2_dn11))), (var_ninvd0 * ((p.p324 * var_tdiff_dn14) + (p.p325 * var_tdiff_2_dn14))),)
    } else {
        (var_ninvde, var_ninvde_dn0, var_ninvde_dn2, var_ninvde_dn4, var_ninvde_dn5, var_ninvde_dn6, var_ninvde_dn7, var_ninvde_dn8, var_ninvde_dn9, var_ninvde_dn10, var_ninvde_dn11, var_ninvde_dn14,)
    }
};
        var_ninvde = assign13670_e8030;
        var_ninvde_dn0 = assign13670_e8030_d_n0;
        var_ninvde_dn2 = assign13670_e8030_d_n2;
        var_ninvde_dn4 = assign13670_e8030_d_n4;
        var_ninvde_dn5 = assign13670_e8030_d_n5;
        var_ninvde_dn6 = assign13670_e8030_d_n6;
        var_ninvde_dn7 = assign13670_e8030_d_n7;
        var_ninvde_dn8 = assign13670_e8030_d_n8;
        var_ninvde_dn9 = assign13670_e8030_d_n9;
        var_ninvde_dn10 = assign13670_e8030_d_n10;
        var_ninvde_dn11 = assign13670_e8030_d_n11;
        var_ninvde_dn14 = assign13670_e8030_d_n14;

        let (assign13680_e8045, assign13680_e8045_d_n0, assign13680_e8045_d_n2, assign13680_e8045_d_n4, assign13680_e8045_d_n5, assign13680_e8045_d_n6, assign13680_e8045_d_n7, assign13680_e8045_d_n8, assign13680_e8045_d_n9, assign13680_e8045_d_n10, assign13680_e8045_d_n11, assign13680_e8045_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard302 == 0.0)) {
        let assign13680_e8038: f64 = (p.p390 * var_tdiff);
        let assign13680_e8039: f64 = (1.0 + assign13680_e8038);
        let assign13680_e8042: f64 = (p.p391 * var_tdiff_2);
        let assign13680_e8043: f64 = (assign13680_e8039 + assign13680_e8042);
        (assign13680_e8043, ((p.p390 * var_tdiff_dn0) + (p.p391 * var_tdiff_2_dn0)), ((p.p390 * var_tdiff_dn2) + (p.p391 * var_tdiff_2_dn2)), ((p.p390 * var_tdiff_dn4) + (p.p391 * var_tdiff_2_dn4)), ((p.p390 * var_tdiff_dn5) + (p.p391 * var_tdiff_2_dn5)), ((p.p390 * var_tdiff_dn6) + (p.p391 * var_tdiff_2_dn6)), ((p.p390 * var_tdiff_dn7) + (p.p391 * var_tdiff_2_dn7)), ((p.p390 * var_tdiff_dn8) + (p.p391 * var_tdiff_2_dn8)), ((p.p390 * var_tdiff_dn9) + (p.p391 * var_tdiff_2_dn9)), ((p.p390 * var_tdiff_dn10) + (p.p391 * var_tdiff_2_dn10)), ((p.p390 * var_tdiff_dn11) + (p.p391 * var_tdiff_2_dn11)), ((p.p390 * var_tdiff_dn14) + (p.p391 * var_tdiff_2_dn14)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign13680_e8045;
        var_t1_dn0 = assign13680_e8045_d_n0;
        var_t1_dn2 = assign13680_e8045_d_n2;
        var_t1_dn4 = assign13680_e8045_d_n4;
        var_t1_dn5 = assign13680_e8045_d_n5;
        var_t1_dn6 = assign13680_e8045_d_n6;
        var_t1_dn7 = assign13680_e8045_d_n7;
        var_t1_dn8 = assign13680_e8045_d_n8;
        var_t1_dn9 = assign13680_e8045_d_n9;
        var_t1_dn10 = assign13680_e8045_d_n10;
        var_t1_dn11 = assign13680_e8045_d_n11;
        var_t1_dn14 = assign13680_e8045_d_n14;

        let (assign13690_e8054, assign13690_e8054_d_n0, assign13690_e8054_d_n2, assign13690_e8054_d_n4, assign13690_e8054_d_n5, assign13690_e8054_d_n6, assign13690_e8054_d_n7, assign13690_e8054_d_n8, assign13690_e8054_d_n9, assign13690_e8054_d_n10, assign13690_e8054_d_n11, assign13690_e8054_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard302 == 0.0)) {
        let assign13690_e8052: f64 = (var_ninvd0cres * var_t1);
        (assign13690_e8052, ((var_ninvd0cres_dn0 * var_t1) + (var_ninvd0cres * var_t1_dn0)), ((var_ninvd0cres_dn2 * var_t1) + (var_ninvd0cres * var_t1_dn2)), ((var_ninvd0cres_dn4 * var_t1) + (var_ninvd0cres * var_t1_dn4)), ((var_ninvd0cres_dn5 * var_t1) + (var_ninvd0cres * var_t1_dn5)), ((var_ninvd0cres_dn6 * var_t1) + (var_ninvd0cres * var_t1_dn6)), ((var_ninvd0cres_dn7 * var_t1) + (var_ninvd0cres * var_t1_dn7)), ((var_ninvd0cres_dn8 * var_t1) + (var_ninvd0cres * var_t1_dn8)), ((var_ninvd0cres_dn9 * var_t1) + (var_ninvd0cres * var_t1_dn9)), ((var_ninvd0cres_dn10 * var_t1) + (var_ninvd0cres * var_t1_dn10)), ((var_ninvd0cres_dn11 * var_t1) + (var_ninvd0cres * var_t1_dn11)), ((var_ninvd0cres_dn14 * var_t1) + (var_ninvd0cres * var_t1_dn14)),)
    } else {
        (var_ninvdecres, var_ninvdecres_dn0, var_ninvdecres_dn2, var_ninvdecres_dn4, var_ninvdecres_dn5, var_ninvdecres_dn6, var_ninvdecres_dn7, var_ninvdecres_dn8, var_ninvdecres_dn9, var_ninvdecres_dn10, var_ninvdecres_dn11, var_ninvdecres_dn14,)
    }
};
        var_ninvdecres = assign13690_e8054;
        var_ninvdecres_dn0 = assign13690_e8054_d_n0;
        var_ninvdecres_dn2 = assign13690_e8054_d_n2;
        var_ninvdecres_dn4 = assign13690_e8054_d_n4;
        var_ninvdecres_dn5 = assign13690_e8054_d_n5;
        var_ninvdecres_dn6 = assign13690_e8054_d_n6;
        var_ninvdecres_dn7 = assign13690_e8054_d_n7;
        var_ninvdecres_dn8 = assign13690_e8054_d_n8;
        var_ninvdecres_dn9 = assign13690_e8054_d_n9;
        var_ninvdecres_dn10 = assign13690_e8054_d_n10;
        var_ninvdecres_dn11 = assign13690_e8054_d_n11;
        var_ninvdecres_dn14 = assign13690_e8054_d_n14;

        let (assign13700_e8063, assign13700_e8063_d_n0, assign13700_e8063_d_n2, assign13700_e8063_d_n4, assign13700_e8063_d_n5, assign13700_e8063_d_n6, assign13700_e8063_d_n7, assign13700_e8063_d_n8, assign13700_e8063_d_n9, assign13700_e8063_d_n10, assign13700_e8063_d_n11, assign13700_e8063_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard302 == 0.0)) {
        let assign13700_e8061: f64 = (var_ninvd0hres * var_t1);
        (assign13700_e8061, ((var_ninvd0hres_dn0 * var_t1) + (var_ninvd0hres * var_t1_dn0)), ((var_ninvd0hres_dn2 * var_t1) + (var_ninvd0hres * var_t1_dn2)), ((var_ninvd0hres_dn4 * var_t1) + (var_ninvd0hres * var_t1_dn4)), ((var_ninvd0hres_dn5 * var_t1) + (var_ninvd0hres * var_t1_dn5)), ((var_ninvd0hres_dn6 * var_t1) + (var_ninvd0hres * var_t1_dn6)), ((var_ninvd0hres_dn7 * var_t1) + (var_ninvd0hres * var_t1_dn7)), ((var_ninvd0hres_dn8 * var_t1) + (var_ninvd0hres * var_t1_dn8)), ((var_ninvd0hres_dn9 * var_t1) + (var_ninvd0hres * var_t1_dn9)), ((var_ninvd0hres_dn10 * var_t1) + (var_ninvd0hres * var_t1_dn10)), ((var_ninvd0hres_dn11 * var_t1) + (var_ninvd0hres * var_t1_dn11)), ((var_ninvd0hres_dn14 * var_t1) + (var_ninvd0hres * var_t1_dn14)),)
    } else {
        (var_ninvdehres, var_ninvdehres_dn0, var_ninvdehres_dn2, var_ninvdehres_dn4, var_ninvdehres_dn5, var_ninvdehres_dn6, var_ninvdehres_dn7, var_ninvdehres_dn8, var_ninvdehres_dn9, var_ninvdehres_dn10, var_ninvdehres_dn11, var_ninvdehres_dn14,)
    }
};
        var_ninvdehres = assign13700_e8063;
        var_ninvdehres_dn0 = assign13700_e8063_d_n0;
        var_ninvdehres_dn2 = assign13700_e8063_d_n2;
        var_ninvdehres_dn4 = assign13700_e8063_d_n4;
        var_ninvdehres_dn5 = assign13700_e8063_d_n5;
        var_ninvdehres_dn6 = assign13700_e8063_d_n6;
        var_ninvdehres_dn7 = assign13700_e8063_d_n7;
        var_ninvdehres_dn8 = assign13700_e8063_d_n8;
        var_ninvdehres_dn9 = assign13700_e8063_d_n9;
        var_ninvdehres_dn10 = assign13700_e8063_d_n10;
        var_ninvdehres_dn11 = assign13700_e8063_d_n11;
        var_ninvdehres_dn14 = assign13700_e8063_d_n14;

        let assign13720_e8071: f64 = if var_ninvde < 0.0 { 1.0 } else { 0.0 };
        var_guard304 = assign13720_e8071;

        let (assign13730_e8077, assign13730_e8077_d_n0, assign13730_e8077_d_n2, assign13730_e8077_d_n4, assign13730_e8077_d_n5, assign13730_e8077_d_n6, assign13730_e8077_d_n7, assign13730_e8077_d_n8, assign13730_e8077_d_n9, assign13730_e8077_d_n10, assign13730_e8077_d_n11, assign13730_e8077_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard304 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ninvde, var_ninvde_dn0, var_ninvde_dn2, var_ninvde_dn4, var_ninvde_dn5, var_ninvde_dn6, var_ninvde_dn7, var_ninvde_dn8, var_ninvde_dn9, var_ninvde_dn10, var_ninvde_dn11, var_ninvde_dn14,)
    }
};
        var_ninvde = assign13730_e8077;
        var_ninvde_dn0 = assign13730_e8077_d_n0;
        var_ninvde_dn2 = assign13730_e8077_d_n2;
        var_ninvde_dn4 = assign13730_e8077_d_n4;
        var_ninvde_dn5 = assign13730_e8077_d_n5;
        var_ninvde_dn6 = assign13730_e8077_d_n6;
        var_ninvde_dn7 = assign13730_e8077_d_n7;
        var_ninvde_dn8 = assign13730_e8077_d_n8;
        var_ninvde_dn9 = assign13730_e8077_d_n9;
        var_ninvde_dn10 = assign13730_e8077_d_n10;
        var_ninvde_dn11 = assign13730_e8077_d_n11;
        var_ninvde_dn14 = assign13730_e8077_d_n14;

        let assign13750_e8085: f64 = if var_ninvdecres < 0.0 { 1.0 } else { 0.0 };
        var_guard306 = assign13750_e8085;

        let (assign13760_e8091, assign13760_e8091_d_n0, assign13760_e8091_d_n2, assign13760_e8091_d_n4, assign13760_e8091_d_n5, assign13760_e8091_d_n6, assign13760_e8091_d_n7, assign13760_e8091_d_n8, assign13760_e8091_d_n9, assign13760_e8091_d_n10, assign13760_e8091_d_n11, assign13760_e8091_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard306 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ninvdecres, var_ninvdecres_dn0, var_ninvdecres_dn2, var_ninvdecres_dn4, var_ninvdecres_dn5, var_ninvdecres_dn6, var_ninvdecres_dn7, var_ninvdecres_dn8, var_ninvdecres_dn9, var_ninvdecres_dn10, var_ninvdecres_dn11, var_ninvdecres_dn14,)
    }
};
        var_ninvdecres = assign13760_e8091;
        var_ninvdecres_dn0 = assign13760_e8091_d_n0;
        var_ninvdecres_dn2 = assign13760_e8091_d_n2;
        var_ninvdecres_dn4 = assign13760_e8091_d_n4;
        var_ninvdecres_dn5 = assign13760_e8091_d_n5;
        var_ninvdecres_dn6 = assign13760_e8091_d_n6;
        var_ninvdecres_dn7 = assign13760_e8091_d_n7;
        var_ninvdecres_dn8 = assign13760_e8091_d_n8;
        var_ninvdecres_dn9 = assign13760_e8091_d_n9;
        var_ninvdecres_dn10 = assign13760_e8091_d_n10;
        var_ninvdecres_dn11 = assign13760_e8091_d_n11;
        var_ninvdecres_dn14 = assign13760_e8091_d_n14;

        let assign13780_e8099: f64 = if var_ninvdehres < 0.0 { 1.0 } else { 0.0 };
        var_guard308 = assign13780_e8099;

        let (assign13790_e8105, assign13790_e8105_d_n0, assign13790_e8105_d_n2, assign13790_e8105_d_n4, assign13790_e8105_d_n5, assign13790_e8105_d_n6, assign13790_e8105_d_n7, assign13790_e8105_d_n8, assign13790_e8105_d_n9, assign13790_e8105_d_n10, assign13790_e8105_d_n11, assign13790_e8105_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard308 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ninvdehres, var_ninvdehres_dn0, var_ninvdehres_dn2, var_ninvdehres_dn4, var_ninvdehres_dn5, var_ninvdehres_dn6, var_ninvdehres_dn7, var_ninvdehres_dn8, var_ninvdehres_dn9, var_ninvdehres_dn10, var_ninvdehres_dn11, var_ninvdehres_dn14,)
    }
};
        var_ninvdehres = assign13790_e8105;
        var_ninvdehres_dn0 = assign13790_e8105_d_n0;
        var_ninvdehres_dn2 = assign13790_e8105_d_n2;
        var_ninvdehres_dn4 = assign13790_e8105_d_n4;
        var_ninvdehres_dn5 = assign13790_e8105_d_n5;
        var_ninvdehres_dn6 = assign13790_e8105_d_n6;
        var_ninvdehres_dn7 = assign13790_e8105_d_n7;
        var_ninvdehres_dn8 = assign13790_e8105_d_n8;
        var_ninvdehres_dn9 = assign13790_e8105_d_n9;
        var_ninvdehres_dn10 = assign13790_e8105_d_n10;
        var_ninvdehres_dn11 = assign13790_e8105_d_n11;
        var_ninvdehres_dn14 = assign13790_e8105_d_n14;

        let (assign13800_e8121, assign13800_e8121_d_n0, assign13800_e8121_d_n2, assign13800_e8121_d_n4, assign13800_e8121_d_n5, assign13800_e8121_d_n6, assign13800_e8121_d_n7, assign13800_e8121_d_n8, assign13800_e8121_d_n9, assign13800_e8121_d_n10, assign13800_e8121_d_n11, assign13800_e8121_d_n14,) = {
    if ((var_guard293 != 0.0) && (p.p53 != 0.0)) {
        let assign13800_e8112: f64 = (p.p328 * var_tdiff0);
        let assign13800_e8113: f64 = (var_uc_rth0 + assign13800_e8112);
        let assign13800_e8116: f64 = (p.p329 * var_tdiff0_2);
        let assign13800_e8117: f64 = (assign13800_e8113 + assign13800_e8116);
        let assign13800_e8119: f64 = (assign13800_e8117 * var_rthtemp0);
        (assign13800_e8119, (((p.p328 * var_tdiff0_dn0) + (p.p329 * var_tdiff0_2_dn0)) * var_rthtemp0), (((p.p328 * var_tdiff0_dn2) + (p.p329 * var_tdiff0_2_dn2)) * var_rthtemp0), (((p.p328 * var_tdiff0_dn4) + (p.p329 * var_tdiff0_2_dn4)) * var_rthtemp0), (((p.p328 * var_tdiff0_dn5) + (p.p329 * var_tdiff0_2_dn5)) * var_rthtemp0), (((p.p328 * var_tdiff0_dn6) + (p.p329 * var_tdiff0_2_dn6)) * var_rthtemp0), (((p.p328 * var_tdiff0_dn7) + (p.p329 * var_tdiff0_2_dn7)) * var_rthtemp0), (((p.p328 * var_tdiff0_dn8) + (p.p329 * var_tdiff0_2_dn8)) * var_rthtemp0), (((p.p328 * var_tdiff0_dn9) + (p.p329 * var_tdiff0_2_dn9)) * var_rthtemp0), (((p.p328 * var_tdiff0_dn10) + (p.p329 * var_tdiff0_2_dn10)) * var_rthtemp0), (((p.p328 * var_tdiff0_dn11) + (p.p329 * var_tdiff0_2_dn11)) * var_rthtemp0), (((p.p328 * var_tdiff0_dn14) + (p.p329 * var_tdiff0_2_dn14)) * var_rthtemp0),)
    } else {
        (var_rth, var_rth_dn0, var_rth_dn2, var_rth_dn4, var_rth_dn5, var_rth_dn6, var_rth_dn7, var_rth_dn8, var_rth_dn9, var_rth_dn10, var_rth_dn11, var_rth_dn14,)
    }
};
        var_rth = assign13800_e8121;
        var_rth_dn0 = assign13800_e8121_d_n0;
        var_rth_dn2 = assign13800_e8121_d_n2;
        var_rth_dn4 = assign13800_e8121_d_n4;
        var_rth_dn5 = assign13800_e8121_d_n5;
        var_rth_dn6 = assign13800_e8121_d_n6;
        var_rth_dn7 = assign13800_e8121_d_n7;
        var_rth_dn8 = assign13800_e8121_d_n8;
        var_rth_dn9 = assign13800_e8121_d_n9;
        var_rth_dn10 = assign13800_e8121_d_n10;
        var_rth_dn11 = assign13800_e8121_d_n11;
        var_rth_dn14 = assign13800_e8121_d_n14;

        let assign13820_e8129: f64 = if var_rth < 0.0001 { 1.0 } else { 0.0 };
        var_guard310 = assign13820_e8129;

        let (assign13830_e8137, assign13830_e8137_d_n0, assign13830_e8137_d_n2, assign13830_e8137_d_n4, assign13830_e8137_d_n5, assign13830_e8137_d_n6, assign13830_e8137_d_n7, assign13830_e8137_d_n8, assign13830_e8137_d_n9, assign13830_e8137_d_n10, assign13830_e8137_d_n11, assign13830_e8137_d_n14,) = {
    if (((var_guard293 != 0.0) && (p.p53 != 0.0)) && (var_guard310 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rth, var_rth_dn0, var_rth_dn2, var_rth_dn4, var_rth_dn5, var_rth_dn6, var_rth_dn7, var_rth_dn8, var_rth_dn9, var_rth_dn10, var_rth_dn11, var_rth_dn14,)
    }
};
        var_rth = assign13830_e8137;
        var_rth_dn0 = assign13830_e8137_d_n0;
        var_rth_dn2 = assign13830_e8137_d_n2;
        var_rth_dn4 = assign13830_e8137_d_n4;
        var_rth_dn5 = assign13830_e8137_d_n5;
        var_rth_dn6 = assign13830_e8137_d_n6;
        var_rth_dn7 = assign13830_e8137_d_n7;
        var_rth_dn8 = assign13830_e8137_d_n8;
        var_rth_dn9 = assign13830_e8137_d_n9;
        var_rth_dn10 = assign13830_e8137_d_n10;
        var_rth_dn11 = assign13830_e8137_d_n11;
        var_rth_dn14 = assign13830_e8137_d_n14;

        let (assign13840_e8149, assign13840_e8149_d_n0, assign13840_e8149_d_n2, assign13840_e8149_d_n4, assign13840_e8149_d_n5, assign13840_e8149_d_n6, assign13840_e8149_d_n7, assign13840_e8149_d_n8, assign13840_e8149_d_n9, assign13840_e8149_d_n10, assign13840_e8149_d_n11, assign13840_e8149_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13840_e8142: f64 = (p.p330 * var_tdiff0);
        let assign13840_e8143: f64 = (var_uc_powrat + assign13840_e8142);
        let assign13840_e8146: f64 = (p.p331 * var_tdiff0_2);
        let assign13840_e8147: f64 = (assign13840_e8143 + assign13840_e8146);
        (assign13840_e8147, ((p.p330 * var_tdiff0_dn0) + (p.p331 * var_tdiff0_2_dn0)), ((p.p330 * var_tdiff0_dn2) + (p.p331 * var_tdiff0_2_dn2)), ((p.p330 * var_tdiff0_dn4) + (p.p331 * var_tdiff0_2_dn4)), ((p.p330 * var_tdiff0_dn5) + (p.p331 * var_tdiff0_2_dn5)), ((p.p330 * var_tdiff0_dn6) + (p.p331 * var_tdiff0_2_dn6)), ((p.p330 * var_tdiff0_dn7) + (p.p331 * var_tdiff0_2_dn7)), ((p.p330 * var_tdiff0_dn8) + (p.p331 * var_tdiff0_2_dn8)), ((p.p330 * var_tdiff0_dn9) + (p.p331 * var_tdiff0_2_dn9)), ((p.p330 * var_tdiff0_dn10) + (p.p331 * var_tdiff0_2_dn10)), ((p.p330 * var_tdiff0_dn11) + (p.p331 * var_tdiff0_2_dn11)), ((p.p330 * var_tdiff0_dn14) + (p.p331 * var_tdiff0_2_dn14)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign13840_e8149;
        var_t2_dn0 = assign13840_e8149_d_n0;
        var_t2_dn2 = assign13840_e8149_d_n2;
        var_t2_dn4 = assign13840_e8149_d_n4;
        var_t2_dn5 = assign13840_e8149_d_n5;
        var_t2_dn6 = assign13840_e8149_d_n6;
        var_t2_dn7 = assign13840_e8149_d_n7;
        var_t2_dn8 = assign13840_e8149_d_n8;
        var_t2_dn9 = assign13840_e8149_d_n9;
        var_t2_dn10 = assign13840_e8149_d_n10;
        var_t2_dn11 = assign13840_e8149_d_n11;
        var_t2_dn14 = assign13840_e8149_d_n14;

        let (assign13850_e8157, assign13850_e8157_d_n0, assign13850_e8157_d_n2, assign13850_e8157_d_n4, assign13850_e8157_d_n5, assign13850_e8157_d_n6, assign13850_e8157_d_n7, assign13850_e8157_d_n8, assign13850_e8157_d_n9, assign13850_e8157_d_n10, assign13850_e8157_d_n11, assign13850_e8157_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13850_e8153: f64 = var_t2;
        let assign13850_e8155: f64 = (assign13850_e8153 - 0.05);
        (assign13850_e8155, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign13850_e8157;
        var_tmf1_dn0 = assign13850_e8157_d_n0;
        var_tmf1_dn2 = assign13850_e8157_d_n2;
        var_tmf1_dn4 = assign13850_e8157_d_n4;
        var_tmf1_dn5 = assign13850_e8157_d_n5;
        var_tmf1_dn6 = assign13850_e8157_d_n6;
        var_tmf1_dn7 = assign13850_e8157_d_n7;
        var_tmf1_dn8 = assign13850_e8157_d_n8;
        var_tmf1_dn9 = assign13850_e8157_d_n9;
        var_tmf1_dn10 = assign13850_e8157_d_n10;
        var_tmf1_dn11 = assign13850_e8157_d_n11;
        var_tmf1_dn14 = assign13850_e8157_d_n14;

        let (assign13860_e8165, assign13860_e8165_d_n0, assign13860_e8165_d_n2, assign13860_e8165_d_n4, assign13860_e8165_d_n5, assign13860_e8165_d_n6, assign13860_e8165_d_n7, assign13860_e8165_d_n8, assign13860_e8165_d_n9, assign13860_e8165_d_n10, assign13860_e8165_d_n11, assign13860_e8165_d_n14,) = {
    if (var_guard293 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign13860_e8165;
        var_tmf2_dn0 = assign13860_e8165_d_n0;
        var_tmf2_dn2 = assign13860_e8165_d_n2;
        var_tmf2_dn4 = assign13860_e8165_d_n4;
        var_tmf2_dn5 = assign13860_e8165_d_n5;
        var_tmf2_dn6 = assign13860_e8165_d_n6;
        var_tmf2_dn7 = assign13860_e8165_d_n7;
        var_tmf2_dn8 = assign13860_e8165_d_n8;
        var_tmf2_dn9 = assign13860_e8165_d_n9;
        var_tmf2_dn10 = assign13860_e8165_d_n10;
        var_tmf2_dn11 = assign13860_e8165_d_n11;
        var_tmf2_dn14 = assign13860_e8165_d_n14;

        let (assign13870_e8175, assign13870_e8175_d_n0, assign13870_e8175_d_n2, assign13870_e8175_d_n4, assign13870_e8175_d_n5, assign13870_e8175_d_n6, assign13870_e8175_d_n7, assign13870_e8175_d_n8, assign13870_e8175_d_n9, assign13870_e8175_d_n10, assign13870_e8175_d_n11, assign13870_e8175_d_n14,) = {
    if (var_guard293 != 0.0) {
        let (assign13870_e8173, assign13870_e8173_d_n0, assign13870_e8173_d_n2, assign13870_e8173_d_n4, assign13870_e8173_d_n5, assign13870_e8173_d_n6, assign13870_e8173_d_n7, assign13870_e8173_d_n8, assign13870_e8173_d_n9, assign13870_e8173_d_n10, assign13870_e8173_d_n11, assign13870_e8173_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign13870_e8172: f64 = (-var_tmf2);
                (assign13870_e8172, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign13870_e8173, assign13870_e8173_d_n0, assign13870_e8173_d_n2, assign13870_e8173_d_n4, assign13870_e8173_d_n5, assign13870_e8173_d_n6, assign13870_e8173_d_n7, assign13870_e8173_d_n8, assign13870_e8173_d_n9, assign13870_e8173_d_n10, assign13870_e8173_d_n11, assign13870_e8173_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign13870_e8175;
        var_tmf2_dn0 = assign13870_e8175_d_n0;
        var_tmf2_dn2 = assign13870_e8175_d_n2;
        var_tmf2_dn4 = assign13870_e8175_d_n4;
        var_tmf2_dn5 = assign13870_e8175_d_n5;
        var_tmf2_dn6 = assign13870_e8175_d_n6;
        var_tmf2_dn7 = assign13870_e8175_d_n7;
        var_tmf2_dn8 = assign13870_e8175_d_n8;
        var_tmf2_dn9 = assign13870_e8175_d_n9;
        var_tmf2_dn10 = assign13870_e8175_d_n10;
        var_tmf2_dn11 = assign13870_e8175_d_n11;
        var_tmf2_dn14 = assign13870_e8175_d_n14;

        let (assign13880_e8184, assign13880_e8184_d_n0, assign13880_e8184_d_n2, assign13880_e8184_d_n4, assign13880_e8184_d_n5, assign13880_e8184_d_n6, assign13880_e8184_d_n7, assign13880_e8184_d_n8, assign13880_e8184_d_n9, assign13880_e8184_d_n10, assign13880_e8184_d_n11, assign13880_e8184_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13880_e8179: f64 = (var_tmf1 * var_tmf1);
        let assign13880_e8181: f64 = (assign13880_e8179 + var_tmf2);
        let assign13880_e8182: f64 = (assign13880_e8181).sqrt();
        (assign13880_e8182, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign13880_e8182)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign13880_e8182)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign13880_e8182)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign13880_e8182)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign13880_e8182)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign13880_e8182)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign13880_e8182)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign13880_e8182)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign13880_e8182)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign13880_e8182)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign13880_e8182)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign13880_e8184;
        var_tmf2_dn0 = assign13880_e8184_d_n0;
        var_tmf2_dn2 = assign13880_e8184_d_n2;
        var_tmf2_dn4 = assign13880_e8184_d_n4;
        var_tmf2_dn5 = assign13880_e8184_d_n5;
        var_tmf2_dn6 = assign13880_e8184_d_n6;
        var_tmf2_dn7 = assign13880_e8184_d_n7;
        var_tmf2_dn8 = assign13880_e8184_d_n8;
        var_tmf2_dn9 = assign13880_e8184_d_n9;
        var_tmf2_dn10 = assign13880_e8184_d_n10;
        var_tmf2_dn11 = assign13880_e8184_d_n11;
        var_tmf2_dn14 = assign13880_e8184_d_n14;

        *var_guard300_slot = var_guard300;
        *var_guard302_slot = var_guard302;
        *var_guard304_slot = var_guard304;
        *var_guard306_slot = var_guard306;
        *var_guard308_slot = var_guard308;
        *var_guard310_slot = var_guard310;
        *var_ninvde_slot = var_ninvde;
        *var_ninvde_dn0_slot = var_ninvde_dn0;
        *var_ninvde_dn10_slot = var_ninvde_dn10;
        *var_ninvde_dn11_slot = var_ninvde_dn11;
        *var_ninvde_dn14_slot = var_ninvde_dn14;
        *var_ninvde_dn2_slot = var_ninvde_dn2;
        *var_ninvde_dn4_slot = var_ninvde_dn4;
        *var_ninvde_dn5_slot = var_ninvde_dn5;
        *var_ninvde_dn6_slot = var_ninvde_dn6;
        *var_ninvde_dn7_slot = var_ninvde_dn7;
        *var_ninvde_dn8_slot = var_ninvde_dn8;
        *var_ninvde_dn9_slot = var_ninvde_dn9;
        *var_ninvdecres_slot = var_ninvdecres;
        *var_ninvdecres_dn0_slot = var_ninvdecres_dn0;
        *var_ninvdecres_dn10_slot = var_ninvdecres_dn10;
        *var_ninvdecres_dn11_slot = var_ninvdecres_dn11;
        *var_ninvdecres_dn14_slot = var_ninvdecres_dn14;
        *var_ninvdecres_dn2_slot = var_ninvdecres_dn2;
        *var_ninvdecres_dn4_slot = var_ninvdecres_dn4;
        *var_ninvdecres_dn5_slot = var_ninvdecres_dn5;
        *var_ninvdecres_dn6_slot = var_ninvdecres_dn6;
        *var_ninvdecres_dn7_slot = var_ninvdecres_dn7;
        *var_ninvdecres_dn8_slot = var_ninvdecres_dn8;
        *var_ninvdecres_dn9_slot = var_ninvdecres_dn9;
        *var_ninvdehres_slot = var_ninvdehres;
        *var_ninvdehres_dn0_slot = var_ninvdehres_dn0;
        *var_ninvdehres_dn10_slot = var_ninvdehres_dn10;
        *var_ninvdehres_dn11_slot = var_ninvdehres_dn11;
        *var_ninvdehres_dn14_slot = var_ninvdehres_dn14;
        *var_ninvdehres_dn2_slot = var_ninvdehres_dn2;
        *var_ninvdehres_dn4_slot = var_ninvdehres_dn4;
        *var_ninvdehres_dn5_slot = var_ninvdehres_dn5;
        *var_ninvdehres_dn6_slot = var_ninvdehres_dn6;
        *var_ninvdehres_dn7_slot = var_ninvdehres_dn7;
        *var_ninvdehres_dn8_slot = var_ninvdehres_dn8;
        *var_ninvdehres_dn9_slot = var_ninvdehres_dn9;
        *var_rth_slot = var_rth;
        *var_rth_dn0_slot = var_rth_dn0;
        *var_rth_dn10_slot = var_rth_dn10;
        *var_rth_dn11_slot = var_rth_dn11;
        *var_rth_dn14_slot = var_rth_dn14;
        *var_rth_dn2_slot = var_rth_dn2;
        *var_rth_dn4_slot = var_rth_dn4;
        *var_rth_dn5_slot = var_rth_dn5;
        *var_rth_dn6_slot = var_rth_dn6;
        *var_rth_dn7_slot = var_rth_dn7;
        *var_rth_dn8_slot = var_rth_dn8;
        *var_rth_dn9_slot = var_rth_dn9;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn14_slot = var_tmf1_dn14;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_dn9_slot = var_tmf1_dn9;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn14_slot = var_tmf2_dn14;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_dn9_slot = var_tmf2_dn9;
        *var_vmaxeff_slot = var_vmaxeff;
        *var_vmaxeff_dn0_slot = var_vmaxeff_dn0;
        *var_vmaxeff_dn10_slot = var_vmaxeff_dn10;
        *var_vmaxeff_dn11_slot = var_vmaxeff_dn11;
        *var_vmaxeff_dn14_slot = var_vmaxeff_dn14;
        *var_vmaxeff_dn2_slot = var_vmaxeff_dn2;
        *var_vmaxeff_dn4_slot = var_vmaxeff_dn4;
        *var_vmaxeff_dn5_slot = var_vmaxeff_dn5;
        *var_vmaxeff_dn6_slot = var_vmaxeff_dn6;
        *var_vmaxeff_dn7_slot = var_vmaxeff_dn7;
        *var_vmaxeff_dn8_slot = var_vmaxeff_dn8;
        *var_vmaxeff_dn9_slot = var_vmaxeff_dn9;
    }

    pub(super) fn stamp_transient_block_25(
        p: &Parameters,
        var_beta_inv: f64,
        var_beta_inv_dn0: f64,
        var_beta_inv_dn10: f64,
        var_beta_inv_dn11: f64,
        var_beta_inv_dn14: f64,
        var_beta_inv_dn2: f64,
        var_beta_inv_dn4: f64,
        var_beta_inv_dn5: f64,
        var_beta_inv_dn6: f64,
        var_beta_inv_dn7: f64,
        var_beta_inv_dn8: f64,
        var_beta_inv_dn9: f64,
        var_ef_nsubp: f64,
        var_ef_nsubp_dn0: f64,
        var_ef_nsubp_dn10: f64,
        var_ef_nsubp_dn11: f64,
        var_ef_nsubp_dn14: f64,
        var_ef_nsubp_dn2: f64,
        var_ef_nsubp_dn4: f64,
        var_ef_nsubp_dn5: f64,
        var_ef_nsubp_dn6: f64,
        var_ef_nsubp_dn7: f64,
        var_ef_nsubp_dn8: f64,
        var_ef_nsubp_dn9: f64,
        var_guard293: f64,
        var_nin: f64,
        var_nin_dn0: f64,
        var_nin_dn10: f64,
        var_nin_dn11: f64,
        var_nin_dn14: f64,
        var_nin_dn2: f64,
        var_nin_dn4: f64,
        var_nin_dn5: f64,
        var_nin_dn6: f64,
        var_nin_dn7: f64,
        var_nin_dn8: f64,
        var_nin_dn9: f64,
        var_nsub: f64,
        var_nsub_dn0: f64,
        var_nsub_dn10: f64,
        var_nsub_dn11: f64,
        var_nsub_dn14: f64,
        var_nsub_dn2: f64,
        var_nsub_dn4: f64,
        var_nsub_dn5: f64,
        var_nsub_dn6: f64,
        var_nsub_dn7: f64,
        var_nsub_dn8: f64,
        var_nsub_dn9: f64,
        var_rdtemp0: f64,
        var_uc_codep: f64,
        var_uc_cordrift: f64,
        var_uc_ndepm: f64,
        var_uc_ndepm_dn0: f64,
        var_uc_ndepm_dn10: f64,
        var_uc_ndepm_dn11: f64,
        var_uc_ndepm_dn14: f64,
        var_uc_ndepm_dn2: f64,
        var_uc_ndepm_dn4: f64,
        var_uc_ndepm_dn5: f64,
        var_uc_ndepm_dn6: f64,
        var_uc_ndepm_dn7: f64,
        var_uc_ndepm_dn8: f64,
        var_uc_ndepm_dn9: f64,
        var_uc_nover: f64,
        var_uc_novers: f64,
        var_uc_rd: f64,
        var_uc_rdict1: f64,
        var_uc_rdslp1: f64,
        var_cnst0_slot: &mut f64,
        var_cnst0_dn0_slot: &mut f64,
        var_cnst0_dn10_slot: &mut f64,
        var_cnst0_dn11_slot: &mut f64,
        var_cnst0_dn14_slot: &mut f64,
        var_cnst0_dn2_slot: &mut f64,
        var_cnst0_dn4_slot: &mut f64,
        var_cnst0_dn5_slot: &mut f64,
        var_cnst0_dn6_slot: &mut f64,
        var_cnst0_dn7_slot: &mut f64,
        var_cnst0_dn8_slot: &mut f64,
        var_cnst0_dn9_slot: &mut f64,
        var_cnst0over_slot: &mut f64,
        var_cnst0over_dn0_slot: &mut f64,
        var_cnst0over_dn10_slot: &mut f64,
        var_cnst0over_dn11_slot: &mut f64,
        var_cnst0over_dn14_slot: &mut f64,
        var_cnst0over_dn2_slot: &mut f64,
        var_cnst0over_dn4_slot: &mut f64,
        var_cnst0over_dn5_slot: &mut f64,
        var_cnst0over_dn6_slot: &mut f64,
        var_cnst0over_dn7_slot: &mut f64,
        var_cnst0over_dn8_slot: &mut f64,
        var_cnst0over_dn9_slot: &mut f64,
        var_cnst0overs_slot: &mut f64,
        var_cnst0overs_dn0_slot: &mut f64,
        var_cnst0overs_dn10_slot: &mut f64,
        var_cnst0overs_dn11_slot: &mut f64,
        var_cnst0overs_dn14_slot: &mut f64,
        var_cnst0overs_dn2_slot: &mut f64,
        var_cnst0overs_dn4_slot: &mut f64,
        var_cnst0overs_dn5_slot: &mut f64,
        var_cnst0overs_dn6_slot: &mut f64,
        var_cnst0overs_dn7_slot: &mut f64,
        var_cnst0overs_dn8_slot: &mut f64,
        var_cnst0overs_dn9_slot: &mut f64,
        var_cnst1_slot: &mut f64,
        var_cnst1_dn0_slot: &mut f64,
        var_cnst1_dn10_slot: &mut f64,
        var_cnst1_dn11_slot: &mut f64,
        var_cnst1_dn14_slot: &mut f64,
        var_cnst1_dn2_slot: &mut f64,
        var_cnst1_dn4_slot: &mut f64,
        var_cnst1_dn5_slot: &mut f64,
        var_cnst1_dn6_slot: &mut f64,
        var_cnst1_dn7_slot: &mut f64,
        var_cnst1_dn8_slot: &mut f64,
        var_cnst1_dn9_slot: &mut f64,
        var_guard311_slot: &mut f64,
        var_guard312_slot: &mut f64,
        var_guard313_slot: &mut f64,
        var_guard314_slot: &mut f64,
        var_guard315_slot: &mut f64,
        var_guard316_slot: &mut f64,
        var_guard317_slot: &mut f64,
        var_guard318_slot: &mut f64,
        var_guard319_slot: &mut f64,
        var_pb2_slot: &mut f64,
        var_pb2_dn0_slot: &mut f64,
        var_pb2_dn10_slot: &mut f64,
        var_pb2_dn11_slot: &mut f64,
        var_pb2_dn14_slot: &mut f64,
        var_pb2_dn2_slot: &mut f64,
        var_pb2_dn4_slot: &mut f64,
        var_pb2_dn5_slot: &mut f64,
        var_pb2_dn6_slot: &mut f64,
        var_pb2_dn7_slot: &mut f64,
        var_pb2_dn8_slot: &mut f64,
        var_pb2_dn9_slot: &mut f64,
        var_powratio_slot: &mut f64,
        var_powratio_dn0_slot: &mut f64,
        var_powratio_dn10_slot: &mut f64,
        var_powratio_dn11_slot: &mut f64,
        var_powratio_dn14_slot: &mut f64,
        var_powratio_dn2_slot: &mut f64,
        var_powratio_dn4_slot: &mut f64,
        var_powratio_dn5_slot: &mut f64,
        var_powratio_dn6_slot: &mut f64,
        var_powratio_dn7_slot: &mut f64,
        var_powratio_dn8_slot: &mut f64,
        var_powratio_dn9_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn14_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_dn9_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn14_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_dn9_slot: &mut f64,
        var_wdpl_slot: &mut f64,
        var_wdpl_dn0_slot: &mut f64,
        var_wdpl_dn10_slot: &mut f64,
        var_wdpl_dn11_slot: &mut f64,
        var_wdpl_dn14_slot: &mut f64,
        var_wdpl_dn2_slot: &mut f64,
        var_wdpl_dn4_slot: &mut f64,
        var_wdpl_dn5_slot: &mut f64,
        var_wdpl_dn6_slot: &mut f64,
        var_wdpl_dn7_slot: &mut f64,
        var_wdpl_dn8_slot: &mut f64,
        var_wdpl_dn9_slot: &mut f64,
        var_wdplp_slot: &mut f64,
        var_wdplp_dn0_slot: &mut f64,
        var_wdplp_dn10_slot: &mut f64,
        var_wdplp_dn11_slot: &mut f64,
        var_wdplp_dn14_slot: &mut f64,
        var_wdplp_dn2_slot: &mut f64,
        var_wdplp_dn4_slot: &mut f64,
        var_wdplp_dn5_slot: &mut f64,
        var_wdplp_dn6_slot: &mut f64,
        var_wdplp_dn7_slot: &mut f64,
        var_wdplp_dn8_slot: &mut f64,
        var_wdplp_dn9_slot: &mut f64,
    ) {
        let mut var_cnst0: f64 = *var_cnst0_slot;
        let mut var_cnst0_dn0: f64 = *var_cnst0_dn0_slot;
        let mut var_cnst0_dn10: f64 = *var_cnst0_dn10_slot;
        let mut var_cnst0_dn11: f64 = *var_cnst0_dn11_slot;
        let mut var_cnst0_dn14: f64 = *var_cnst0_dn14_slot;
        let mut var_cnst0_dn2: f64 = *var_cnst0_dn2_slot;
        let mut var_cnst0_dn4: f64 = *var_cnst0_dn4_slot;
        let mut var_cnst0_dn5: f64 = *var_cnst0_dn5_slot;
        let mut var_cnst0_dn6: f64 = *var_cnst0_dn6_slot;
        let mut var_cnst0_dn7: f64 = *var_cnst0_dn7_slot;
        let mut var_cnst0_dn8: f64 = *var_cnst0_dn8_slot;
        let mut var_cnst0_dn9: f64 = *var_cnst0_dn9_slot;
        let mut var_cnst0over: f64 = *var_cnst0over_slot;
        let mut var_cnst0over_dn0: f64 = *var_cnst0over_dn0_slot;
        let mut var_cnst0over_dn10: f64 = *var_cnst0over_dn10_slot;
        let mut var_cnst0over_dn11: f64 = *var_cnst0over_dn11_slot;
        let mut var_cnst0over_dn14: f64 = *var_cnst0over_dn14_slot;
        let mut var_cnst0over_dn2: f64 = *var_cnst0over_dn2_slot;
        let mut var_cnst0over_dn4: f64 = *var_cnst0over_dn4_slot;
        let mut var_cnst0over_dn5: f64 = *var_cnst0over_dn5_slot;
        let mut var_cnst0over_dn6: f64 = *var_cnst0over_dn6_slot;
        let mut var_cnst0over_dn7: f64 = *var_cnst0over_dn7_slot;
        let mut var_cnst0over_dn8: f64 = *var_cnst0over_dn8_slot;
        let mut var_cnst0over_dn9: f64 = *var_cnst0over_dn9_slot;
        let mut var_cnst0overs: f64 = *var_cnst0overs_slot;
        let mut var_cnst0overs_dn0: f64 = *var_cnst0overs_dn0_slot;
        let mut var_cnst0overs_dn10: f64 = *var_cnst0overs_dn10_slot;
        let mut var_cnst0overs_dn11: f64 = *var_cnst0overs_dn11_slot;
        let mut var_cnst0overs_dn14: f64 = *var_cnst0overs_dn14_slot;
        let mut var_cnst0overs_dn2: f64 = *var_cnst0overs_dn2_slot;
        let mut var_cnst0overs_dn4: f64 = *var_cnst0overs_dn4_slot;
        let mut var_cnst0overs_dn5: f64 = *var_cnst0overs_dn5_slot;
        let mut var_cnst0overs_dn6: f64 = *var_cnst0overs_dn6_slot;
        let mut var_cnst0overs_dn7: f64 = *var_cnst0overs_dn7_slot;
        let mut var_cnst0overs_dn8: f64 = *var_cnst0overs_dn8_slot;
        let mut var_cnst0overs_dn9: f64 = *var_cnst0overs_dn9_slot;
        let mut var_cnst1: f64 = *var_cnst1_slot;
        let mut var_cnst1_dn0: f64 = *var_cnst1_dn0_slot;
        let mut var_cnst1_dn10: f64 = *var_cnst1_dn10_slot;
        let mut var_cnst1_dn11: f64 = *var_cnst1_dn11_slot;
        let mut var_cnst1_dn14: f64 = *var_cnst1_dn14_slot;
        let mut var_cnst1_dn2: f64 = *var_cnst1_dn2_slot;
        let mut var_cnst1_dn4: f64 = *var_cnst1_dn4_slot;
        let mut var_cnst1_dn5: f64 = *var_cnst1_dn5_slot;
        let mut var_cnst1_dn6: f64 = *var_cnst1_dn6_slot;
        let mut var_cnst1_dn7: f64 = *var_cnst1_dn7_slot;
        let mut var_cnst1_dn8: f64 = *var_cnst1_dn8_slot;
        let mut var_cnst1_dn9: f64 = *var_cnst1_dn9_slot;
        let mut var_guard311: f64 = *var_guard311_slot;
        let mut var_guard312: f64 = *var_guard312_slot;
        let mut var_guard313: f64 = *var_guard313_slot;
        let mut var_guard314: f64 = *var_guard314_slot;
        let mut var_guard315: f64 = *var_guard315_slot;
        let mut var_guard316: f64 = *var_guard316_slot;
        let mut var_guard317: f64 = *var_guard317_slot;
        let mut var_guard318: f64 = *var_guard318_slot;
        let mut var_guard319: f64 = *var_guard319_slot;
        let mut var_pb2: f64 = *var_pb2_slot;
        let mut var_pb2_dn0: f64 = *var_pb2_dn0_slot;
        let mut var_pb2_dn10: f64 = *var_pb2_dn10_slot;
        let mut var_pb2_dn11: f64 = *var_pb2_dn11_slot;
        let mut var_pb2_dn14: f64 = *var_pb2_dn14_slot;
        let mut var_pb2_dn2: f64 = *var_pb2_dn2_slot;
        let mut var_pb2_dn4: f64 = *var_pb2_dn4_slot;
        let mut var_pb2_dn5: f64 = *var_pb2_dn5_slot;
        let mut var_pb2_dn6: f64 = *var_pb2_dn6_slot;
        let mut var_pb2_dn7: f64 = *var_pb2_dn7_slot;
        let mut var_pb2_dn8: f64 = *var_pb2_dn8_slot;
        let mut var_pb2_dn9: f64 = *var_pb2_dn9_slot;
        let mut var_powratio: f64 = *var_powratio_slot;
        let mut var_powratio_dn0: f64 = *var_powratio_dn0_slot;
        let mut var_powratio_dn10: f64 = *var_powratio_dn10_slot;
        let mut var_powratio_dn11: f64 = *var_powratio_dn11_slot;
        let mut var_powratio_dn14: f64 = *var_powratio_dn14_slot;
        let mut var_powratio_dn2: f64 = *var_powratio_dn2_slot;
        let mut var_powratio_dn4: f64 = *var_powratio_dn4_slot;
        let mut var_powratio_dn5: f64 = *var_powratio_dn5_slot;
        let mut var_powratio_dn6: f64 = *var_powratio_dn6_slot;
        let mut var_powratio_dn7: f64 = *var_powratio_dn7_slot;
        let mut var_powratio_dn8: f64 = *var_powratio_dn8_slot;
        let mut var_powratio_dn9: f64 = *var_powratio_dn9_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn14: f64 = *var_tmf1_dn14_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_dn9: f64 = *var_tmf1_dn9_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn14: f64 = *var_tmf2_dn14_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_dn9: f64 = *var_tmf2_dn9_slot;
        let mut var_wdpl: f64 = *var_wdpl_slot;
        let mut var_wdpl_dn0: f64 = *var_wdpl_dn0_slot;
        let mut var_wdpl_dn10: f64 = *var_wdpl_dn10_slot;
        let mut var_wdpl_dn11: f64 = *var_wdpl_dn11_slot;
        let mut var_wdpl_dn14: f64 = *var_wdpl_dn14_slot;
        let mut var_wdpl_dn2: f64 = *var_wdpl_dn2_slot;
        let mut var_wdpl_dn4: f64 = *var_wdpl_dn4_slot;
        let mut var_wdpl_dn5: f64 = *var_wdpl_dn5_slot;
        let mut var_wdpl_dn6: f64 = *var_wdpl_dn6_slot;
        let mut var_wdpl_dn7: f64 = *var_wdpl_dn7_slot;
        let mut var_wdpl_dn8: f64 = *var_wdpl_dn8_slot;
        let mut var_wdpl_dn9: f64 = *var_wdpl_dn9_slot;
        let mut var_wdplp: f64 = *var_wdplp_slot;
        let mut var_wdplp_dn0: f64 = *var_wdplp_dn0_slot;
        let mut var_wdplp_dn10: f64 = *var_wdplp_dn10_slot;
        let mut var_wdplp_dn11: f64 = *var_wdplp_dn11_slot;
        let mut var_wdplp_dn14: f64 = *var_wdplp_dn14_slot;
        let mut var_wdplp_dn2: f64 = *var_wdplp_dn2_slot;
        let mut var_wdplp_dn4: f64 = *var_wdplp_dn4_slot;
        let mut var_wdplp_dn5: f64 = *var_wdplp_dn5_slot;
        let mut var_wdplp_dn6: f64 = *var_wdplp_dn6_slot;
        let mut var_wdplp_dn7: f64 = *var_wdplp_dn7_slot;
        let mut var_wdplp_dn8: f64 = *var_wdplp_dn8_slot;
        let mut var_wdplp_dn9: f64 = *var_wdplp_dn9_slot;

        let (assign13890_e8194, assign13890_e8194_d_n0, assign13890_e8194_d_n2, assign13890_e8194_d_n4, assign13890_e8194_d_n5, assign13890_e8194_d_n6, assign13890_e8194_d_n7, assign13890_e8194_d_n8, assign13890_e8194_d_n9, assign13890_e8194_d_n10, assign13890_e8194_d_n11, assign13890_e8194_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13890_e8190: f64 = (var_tmf1 / var_tmf2);
        let assign13890_e8191: f64 = (1.0 + assign13890_e8190);
        let assign13890_e8192: f64 = (0.5 * assign13890_e8191);
        (assign13890_e8192, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign13890_e8194;
        var_t0_dn0 = assign13890_e8194_d_n0;
        var_t0_dn2 = assign13890_e8194_d_n2;
        var_t0_dn4 = assign13890_e8194_d_n4;
        var_t0_dn5 = assign13890_e8194_d_n5;
        var_t0_dn6 = assign13890_e8194_d_n6;
        var_t0_dn7 = assign13890_e8194_d_n7;
        var_t0_dn8 = assign13890_e8194_d_n8;
        var_t0_dn9 = assign13890_e8194_d_n9;
        var_t0_dn10 = assign13890_e8194_d_n10;
        var_t0_dn11 = assign13890_e8194_d_n11;
        var_t0_dn14 = assign13890_e8194_d_n14;

        let (assign13900_e8204, assign13900_e8204_d_n0, assign13900_e8204_d_n2, assign13900_e8204_d_n4, assign13900_e8204_d_n5, assign13900_e8204_d_n6, assign13900_e8204_d_n7, assign13900_e8204_d_n8, assign13900_e8204_d_n9, assign13900_e8204_d_n10, assign13900_e8204_d_n11, assign13900_e8204_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13900_e8200: f64 = (var_tmf1 + var_tmf2);
        let assign13900_e8201: f64 = (0.5 * assign13900_e8200);
        let assign13900_e8202: f64 = assign13900_e8201;
        (assign13900_e8202, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_dn6 + var_tmf2_dn6)), (0.5 * (var_tmf1_dn7 + var_tmf2_dn7)), (0.5 * (var_tmf1_dn8 + var_tmf2_dn8)), (0.5 * (var_tmf1_dn9 + var_tmf2_dn9)), (0.5 * (var_tmf1_dn10 + var_tmf2_dn10)), (0.5 * (var_tmf1_dn11 + var_tmf2_dn11)), (0.5 * (var_tmf1_dn14 + var_tmf2_dn14)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign13900_e8204;
        var_t2_dn0 = assign13900_e8204_d_n0;
        var_t2_dn2 = assign13900_e8204_d_n2;
        var_t2_dn4 = assign13900_e8204_d_n4;
        var_t2_dn5 = assign13900_e8204_d_n5;
        var_t2_dn6 = assign13900_e8204_d_n6;
        var_t2_dn7 = assign13900_e8204_d_n7;
        var_t2_dn8 = assign13900_e8204_d_n8;
        var_t2_dn9 = assign13900_e8204_d_n9;
        var_t2_dn10 = assign13900_e8204_d_n10;
        var_t2_dn11 = assign13900_e8204_d_n11;
        var_t2_dn14 = assign13900_e8204_d_n14;

        let (assign13910_e8212, assign13910_e8212_d_n0, assign13910_e8212_d_n2, assign13910_e8212_d_n4, assign13910_e8212_d_n5, assign13910_e8212_d_n6, assign13910_e8212_d_n7, assign13910_e8212_d_n8, assign13910_e8212_d_n9, assign13910_e8212_d_n10, assign13910_e8212_d_n11, assign13910_e8212_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13910_e8208: f64 = (1.0 - var_t2);
        let assign13910_e8210: f64 = (assign13910_e8208 - 0.05);
        (assign13910_e8210, (-var_t2_dn0), (-var_t2_dn2), (-var_t2_dn4), (-var_t2_dn5), (-var_t2_dn6), (-var_t2_dn7), (-var_t2_dn8), (-var_t2_dn9), (-var_t2_dn10), (-var_t2_dn11), (-var_t2_dn14),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign13910_e8212;
        var_tmf1_dn0 = assign13910_e8212_d_n0;
        var_tmf1_dn2 = assign13910_e8212_d_n2;
        var_tmf1_dn4 = assign13910_e8212_d_n4;
        var_tmf1_dn5 = assign13910_e8212_d_n5;
        var_tmf1_dn6 = assign13910_e8212_d_n6;
        var_tmf1_dn7 = assign13910_e8212_d_n7;
        var_tmf1_dn8 = assign13910_e8212_d_n8;
        var_tmf1_dn9 = assign13910_e8212_d_n9;
        var_tmf1_dn10 = assign13910_e8212_d_n10;
        var_tmf1_dn11 = assign13910_e8212_d_n11;
        var_tmf1_dn14 = assign13910_e8212_d_n14;

        let (assign13920_e8220, assign13920_e8220_d_n0, assign13920_e8220_d_n2, assign13920_e8220_d_n4, assign13920_e8220_d_n5, assign13920_e8220_d_n6, assign13920_e8220_d_n7, assign13920_e8220_d_n8, assign13920_e8220_d_n9, assign13920_e8220_d_n10, assign13920_e8220_d_n11, assign13920_e8220_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13920_e8216: f64 = 4.0;
        let assign13920_e8218: f64 = (assign13920_e8216 * 0.05);
        (assign13920_e8218, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign13920_e8220;
        var_tmf2_dn0 = assign13920_e8220_d_n0;
        var_tmf2_dn2 = assign13920_e8220_d_n2;
        var_tmf2_dn4 = assign13920_e8220_d_n4;
        var_tmf2_dn5 = assign13920_e8220_d_n5;
        var_tmf2_dn6 = assign13920_e8220_d_n6;
        var_tmf2_dn7 = assign13920_e8220_d_n7;
        var_tmf2_dn8 = assign13920_e8220_d_n8;
        var_tmf2_dn9 = assign13920_e8220_d_n9;
        var_tmf2_dn10 = assign13920_e8220_d_n10;
        var_tmf2_dn11 = assign13920_e8220_d_n11;
        var_tmf2_dn14 = assign13920_e8220_d_n14;

        let (assign13930_e8230, assign13930_e8230_d_n0, assign13930_e8230_d_n2, assign13930_e8230_d_n4, assign13930_e8230_d_n5, assign13930_e8230_d_n6, assign13930_e8230_d_n7, assign13930_e8230_d_n8, assign13930_e8230_d_n9, assign13930_e8230_d_n10, assign13930_e8230_d_n11, assign13930_e8230_d_n14,) = {
    if (var_guard293 != 0.0) {
        let (assign13930_e8228, assign13930_e8228_d_n0, assign13930_e8228_d_n2, assign13930_e8228_d_n4, assign13930_e8228_d_n5, assign13930_e8228_d_n6, assign13930_e8228_d_n7, assign13930_e8228_d_n8, assign13930_e8228_d_n9, assign13930_e8228_d_n10, assign13930_e8228_d_n11, assign13930_e8228_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign13930_e8227: f64 = (-var_tmf2);
                (assign13930_e8227, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign13930_e8228, assign13930_e8228_d_n0, assign13930_e8228_d_n2, assign13930_e8228_d_n4, assign13930_e8228_d_n5, assign13930_e8228_d_n6, assign13930_e8228_d_n7, assign13930_e8228_d_n8, assign13930_e8228_d_n9, assign13930_e8228_d_n10, assign13930_e8228_d_n11, assign13930_e8228_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign13930_e8230;
        var_tmf2_dn0 = assign13930_e8230_d_n0;
        var_tmf2_dn2 = assign13930_e8230_d_n2;
        var_tmf2_dn4 = assign13930_e8230_d_n4;
        var_tmf2_dn5 = assign13930_e8230_d_n5;
        var_tmf2_dn6 = assign13930_e8230_d_n6;
        var_tmf2_dn7 = assign13930_e8230_d_n7;
        var_tmf2_dn8 = assign13930_e8230_d_n8;
        var_tmf2_dn9 = assign13930_e8230_d_n9;
        var_tmf2_dn10 = assign13930_e8230_d_n10;
        var_tmf2_dn11 = assign13930_e8230_d_n11;
        var_tmf2_dn14 = assign13930_e8230_d_n14;

        let (assign13940_e8239, assign13940_e8239_d_n0, assign13940_e8239_d_n2, assign13940_e8239_d_n4, assign13940_e8239_d_n5, assign13940_e8239_d_n6, assign13940_e8239_d_n7, assign13940_e8239_d_n8, assign13940_e8239_d_n9, assign13940_e8239_d_n10, assign13940_e8239_d_n11, assign13940_e8239_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13940_e8234: f64 = (var_tmf1 * var_tmf1);
        let assign13940_e8236: f64 = (assign13940_e8234 + var_tmf2);
        let assign13940_e8237: f64 = (assign13940_e8236).sqrt();
        (assign13940_e8237, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign13940_e8237)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign13940_e8237)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign13940_e8237)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign13940_e8237)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign13940_e8237)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign13940_e8237)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign13940_e8237)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign13940_e8237)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign13940_e8237)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign13940_e8237)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign13940_e8237)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign13940_e8239;
        var_tmf2_dn0 = assign13940_e8239_d_n0;
        var_tmf2_dn2 = assign13940_e8239_d_n2;
        var_tmf2_dn4 = assign13940_e8239_d_n4;
        var_tmf2_dn5 = assign13940_e8239_d_n5;
        var_tmf2_dn6 = assign13940_e8239_d_n6;
        var_tmf2_dn7 = assign13940_e8239_d_n7;
        var_tmf2_dn8 = assign13940_e8239_d_n8;
        var_tmf2_dn9 = assign13940_e8239_d_n9;
        var_tmf2_dn10 = assign13940_e8239_d_n10;
        var_tmf2_dn11 = assign13940_e8239_d_n11;
        var_tmf2_dn14 = assign13940_e8239_d_n14;

        let (assign13950_e8249, assign13950_e8249_d_n0, assign13950_e8249_d_n2, assign13950_e8249_d_n4, assign13950_e8249_d_n5, assign13950_e8249_d_n6, assign13950_e8249_d_n7, assign13950_e8249_d_n8, assign13950_e8249_d_n9, assign13950_e8249_d_n10, assign13950_e8249_d_n11, assign13950_e8249_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13950_e8245: f64 = (var_tmf1 / var_tmf2);
        let assign13950_e8246: f64 = (1.0 + assign13950_e8245);
        let assign13950_e8247: f64 = (0.5 * assign13950_e8246);
        (assign13950_e8247, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign13950_e8249;
        var_t0_dn0 = assign13950_e8249_d_n0;
        var_t0_dn2 = assign13950_e8249_d_n2;
        var_t0_dn4 = assign13950_e8249_d_n4;
        var_t0_dn5 = assign13950_e8249_d_n5;
        var_t0_dn6 = assign13950_e8249_d_n6;
        var_t0_dn7 = assign13950_e8249_d_n7;
        var_t0_dn8 = assign13950_e8249_d_n8;
        var_t0_dn9 = assign13950_e8249_d_n9;
        var_t0_dn10 = assign13950_e8249_d_n10;
        var_t0_dn11 = assign13950_e8249_d_n11;
        var_t0_dn14 = assign13950_e8249_d_n14;

        let (assign13960_e8259, assign13960_e8259_d_n0, assign13960_e8259_d_n2, assign13960_e8259_d_n4, assign13960_e8259_d_n5, assign13960_e8259_d_n6, assign13960_e8259_d_n7, assign13960_e8259_d_n8, assign13960_e8259_d_n9, assign13960_e8259_d_n10, assign13960_e8259_d_n11, assign13960_e8259_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13960_e8255: f64 = (var_tmf1 + var_tmf2);
        let assign13960_e8256: f64 = (0.5 * assign13960_e8255);
        let assign13960_e8257: f64 = (1.0 - assign13960_e8256);
        (assign13960_e8257, (-(0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (-(0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (-(0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (-(0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (-(0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (-(0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (-(0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), (-(0.5 * (var_tmf1_dn9 + var_tmf2_dn9))), (-(0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (-(0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (-(0.5 * (var_tmf1_dn14 + var_tmf2_dn14))),)
    } else {
        (var_powratio, var_powratio_dn0, var_powratio_dn2, var_powratio_dn4, var_powratio_dn5, var_powratio_dn6, var_powratio_dn7, var_powratio_dn8, var_powratio_dn9, var_powratio_dn10, var_powratio_dn11, var_powratio_dn14,)
    }
};
        var_powratio = assign13960_e8259;
        var_powratio_dn0 = assign13960_e8259_d_n0;
        var_powratio_dn2 = assign13960_e8259_d_n2;
        var_powratio_dn4 = assign13960_e8259_d_n4;
        var_powratio_dn5 = assign13960_e8259_d_n5;
        var_powratio_dn6 = assign13960_e8259_d_n6;
        var_powratio_dn7 = assign13960_e8259_d_n7;
        var_powratio_dn8 = assign13960_e8259_d_n8;
        var_powratio_dn9 = assign13960_e8259_d_n9;
        var_powratio_dn10 = assign13960_e8259_d_n10;
        var_powratio_dn11 = assign13960_e8259_d_n11;
        var_powratio_dn14 = assign13960_e8259_d_n14;

        let (assign13970_e8270, assign13970_e8270_d_n0, assign13970_e8270_d_n2, assign13970_e8270_d_n4, assign13970_e8270_d_n5, assign13970_e8270_d_n6, assign13970_e8270_d_n7, assign13970_e8270_d_n8, assign13970_e8270_d_n9, assign13970_e8270_d_n10, assign13970_e8270_d_n11, assign13970_e8270_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13970_e8263: f64 = (2.0 * var_beta_inv);
        let assign13970_e8266: f64 = (var_nsub / var_nin);
        let assign13970_e8267: f64 = (assign13970_e8266).ln();
        let assign13970_e8268: f64 = (assign13970_e8263 * assign13970_e8267);
        (assign13970_e8268, (((2.0 * var_beta_inv_dn0) * assign13970_e8267) + (assign13970_e8263 * ((((var_nsub_dn0 * var_nin) - (var_nsub * var_nin_dn0)) / (var_nin * var_nin)) / assign13970_e8266))), (((2.0 * var_beta_inv_dn2) * assign13970_e8267) + (assign13970_e8263 * ((((var_nsub_dn2 * var_nin) - (var_nsub * var_nin_dn2)) / (var_nin * var_nin)) / assign13970_e8266))), (((2.0 * var_beta_inv_dn4) * assign13970_e8267) + (assign13970_e8263 * ((((var_nsub_dn4 * var_nin) - (var_nsub * var_nin_dn4)) / (var_nin * var_nin)) / assign13970_e8266))), (((2.0 * var_beta_inv_dn5) * assign13970_e8267) + (assign13970_e8263 * ((((var_nsub_dn5 * var_nin) - (var_nsub * var_nin_dn5)) / (var_nin * var_nin)) / assign13970_e8266))), (((2.0 * var_beta_inv_dn6) * assign13970_e8267) + (assign13970_e8263 * ((((var_nsub_dn6 * var_nin) - (var_nsub * var_nin_dn6)) / (var_nin * var_nin)) / assign13970_e8266))), (((2.0 * var_beta_inv_dn7) * assign13970_e8267) + (assign13970_e8263 * ((((var_nsub_dn7 * var_nin) - (var_nsub * var_nin_dn7)) / (var_nin * var_nin)) / assign13970_e8266))), (((2.0 * var_beta_inv_dn8) * assign13970_e8267) + (assign13970_e8263 * ((((var_nsub_dn8 * var_nin) - (var_nsub * var_nin_dn8)) / (var_nin * var_nin)) / assign13970_e8266))), (((2.0 * var_beta_inv_dn9) * assign13970_e8267) + (assign13970_e8263 * ((((var_nsub_dn9 * var_nin) - (var_nsub * var_nin_dn9)) / (var_nin * var_nin)) / assign13970_e8266))), (((2.0 * var_beta_inv_dn10) * assign13970_e8267) + (assign13970_e8263 * ((((var_nsub_dn10 * var_nin) - (var_nsub * var_nin_dn10)) / (var_nin * var_nin)) / assign13970_e8266))), (((2.0 * var_beta_inv_dn11) * assign13970_e8267) + (assign13970_e8263 * ((((var_nsub_dn11 * var_nin) - (var_nsub * var_nin_dn11)) / (var_nin * var_nin)) / assign13970_e8266))), (((2.0 * var_beta_inv_dn14) * assign13970_e8267) + (assign13970_e8263 * ((((var_nsub_dn14 * var_nin) - (var_nsub * var_nin_dn14)) / (var_nin * var_nin)) / assign13970_e8266))),)
    } else {
        (var_pb2, var_pb2_dn0, var_pb2_dn2, var_pb2_dn4, var_pb2_dn5, var_pb2_dn6, var_pb2_dn7, var_pb2_dn8, var_pb2_dn9, var_pb2_dn10, var_pb2_dn11, var_pb2_dn14,)
    }
};
        var_pb2 = assign13970_e8270;
        var_pb2_dn0 = assign13970_e8270_d_n0;
        var_pb2_dn2 = assign13970_e8270_d_n2;
        var_pb2_dn4 = assign13970_e8270_d_n4;
        var_pb2_dn5 = assign13970_e8270_d_n5;
        var_pb2_dn6 = assign13970_e8270_d_n6;
        var_pb2_dn7 = assign13970_e8270_d_n7;
        var_pb2_dn8 = assign13970_e8270_d_n8;
        var_pb2_dn9 = assign13970_e8270_d_n9;
        var_pb2_dn10 = assign13970_e8270_d_n10;
        var_pb2_dn11 = assign13970_e8270_d_n11;
        var_pb2_dn14 = assign13970_e8270_d_n14;

        let (assign13980_e8278, assign13980_e8278_d_n0, assign13980_e8278_d_n2, assign13980_e8278_d_n4, assign13980_e8278_d_n5, assign13980_e8278_d_n6, assign13980_e8278_d_n7, assign13980_e8278_d_n8, assign13980_e8278_d_n9, assign13980_e8278_d_n10, assign13980_e8278_d_n11, assign13980_e8278_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13980_e8274: f64 = (2.0 * 1.034943e-10);
        let assign13980_e8276: f64 = (assign13980_e8274 / 1.6021918e-19);
        (assign13980_e8276, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign13980_e8278;
        var_t1_dn0 = assign13980_e8278_d_n0;
        var_t1_dn2 = assign13980_e8278_d_n2;
        var_t1_dn4 = assign13980_e8278_d_n4;
        var_t1_dn5 = assign13980_e8278_d_n5;
        var_t1_dn6 = assign13980_e8278_d_n6;
        var_t1_dn7 = assign13980_e8278_d_n7;
        var_t1_dn8 = assign13980_e8278_d_n8;
        var_t1_dn9 = assign13980_e8278_d_n9;
        var_t1_dn10 = assign13980_e8278_d_n10;
        var_t1_dn11 = assign13980_e8278_d_n11;
        var_t1_dn14 = assign13980_e8278_d_n14;

        let (assign13990_e8285, assign13990_e8285_d_n0, assign13990_e8285_d_n2, assign13990_e8285_d_n4, assign13990_e8285_d_n5, assign13990_e8285_d_n6, assign13990_e8285_d_n7, assign13990_e8285_d_n8, assign13990_e8285_d_n9, assign13990_e8285_d_n10, assign13990_e8285_d_n11, assign13990_e8285_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign13990_e8282: f64 = (var_t1 / var_nsub);
        let assign13990_e8283: f64 = (assign13990_e8282).sqrt();
        (assign13990_e8283, ((((var_t1_dn0 * var_nsub) - (var_t1 * var_nsub_dn0)) / (var_nsub * var_nsub)) / (2.0 * assign13990_e8283)), ((((var_t1_dn2 * var_nsub) - (var_t1 * var_nsub_dn2)) / (var_nsub * var_nsub)) / (2.0 * assign13990_e8283)), ((((var_t1_dn4 * var_nsub) - (var_t1 * var_nsub_dn4)) / (var_nsub * var_nsub)) / (2.0 * assign13990_e8283)), ((((var_t1_dn5 * var_nsub) - (var_t1 * var_nsub_dn5)) / (var_nsub * var_nsub)) / (2.0 * assign13990_e8283)), ((((var_t1_dn6 * var_nsub) - (var_t1 * var_nsub_dn6)) / (var_nsub * var_nsub)) / (2.0 * assign13990_e8283)), ((((var_t1_dn7 * var_nsub) - (var_t1 * var_nsub_dn7)) / (var_nsub * var_nsub)) / (2.0 * assign13990_e8283)), ((((var_t1_dn8 * var_nsub) - (var_t1 * var_nsub_dn8)) / (var_nsub * var_nsub)) / (2.0 * assign13990_e8283)), ((((var_t1_dn9 * var_nsub) - (var_t1 * var_nsub_dn9)) / (var_nsub * var_nsub)) / (2.0 * assign13990_e8283)), ((((var_t1_dn10 * var_nsub) - (var_t1 * var_nsub_dn10)) / (var_nsub * var_nsub)) / (2.0 * assign13990_e8283)), ((((var_t1_dn11 * var_nsub) - (var_t1 * var_nsub_dn11)) / (var_nsub * var_nsub)) / (2.0 * assign13990_e8283)), ((((var_t1_dn14 * var_nsub) - (var_t1 * var_nsub_dn14)) / (var_nsub * var_nsub)) / (2.0 * assign13990_e8283)),)
    } else {
        (var_wdpl, var_wdpl_dn0, var_wdpl_dn2, var_wdpl_dn4, var_wdpl_dn5, var_wdpl_dn6, var_wdpl_dn7, var_wdpl_dn8, var_wdpl_dn9, var_wdpl_dn10, var_wdpl_dn11, var_wdpl_dn14,)
    }
};
        var_wdpl = assign13990_e8285;
        var_wdpl_dn0 = assign13990_e8285_d_n0;
        var_wdpl_dn2 = assign13990_e8285_d_n2;
        var_wdpl_dn4 = assign13990_e8285_d_n4;
        var_wdpl_dn5 = assign13990_e8285_d_n5;
        var_wdpl_dn6 = assign13990_e8285_d_n6;
        var_wdpl_dn7 = assign13990_e8285_d_n7;
        var_wdpl_dn8 = assign13990_e8285_d_n8;
        var_wdpl_dn9 = assign13990_e8285_d_n9;
        var_wdpl_dn10 = assign13990_e8285_d_n10;
        var_wdpl_dn11 = assign13990_e8285_d_n11;
        var_wdpl_dn14 = assign13990_e8285_d_n14;

        let (assign14000_e8292, assign14000_e8292_d_n0, assign14000_e8292_d_n2, assign14000_e8292_d_n4, assign14000_e8292_d_n5, assign14000_e8292_d_n6, assign14000_e8292_d_n7, assign14000_e8292_d_n8, assign14000_e8292_d_n9, assign14000_e8292_d_n10, assign14000_e8292_d_n11, assign14000_e8292_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign14000_e8289: f64 = (var_t1 / var_ef_nsubp);
        let assign14000_e8290: f64 = (assign14000_e8289).sqrt();
        (assign14000_e8290, ((((var_t1_dn0 * var_ef_nsubp) - (var_t1 * var_ef_nsubp_dn0)) / (var_ef_nsubp * var_ef_nsubp)) / (2.0 * assign14000_e8290)), ((((var_t1_dn2 * var_ef_nsubp) - (var_t1 * var_ef_nsubp_dn2)) / (var_ef_nsubp * var_ef_nsubp)) / (2.0 * assign14000_e8290)), ((((var_t1_dn4 * var_ef_nsubp) - (var_t1 * var_ef_nsubp_dn4)) / (var_ef_nsubp * var_ef_nsubp)) / (2.0 * assign14000_e8290)), ((((var_t1_dn5 * var_ef_nsubp) - (var_t1 * var_ef_nsubp_dn5)) / (var_ef_nsubp * var_ef_nsubp)) / (2.0 * assign14000_e8290)), ((((var_t1_dn6 * var_ef_nsubp) - (var_t1 * var_ef_nsubp_dn6)) / (var_ef_nsubp * var_ef_nsubp)) / (2.0 * assign14000_e8290)), ((((var_t1_dn7 * var_ef_nsubp) - (var_t1 * var_ef_nsubp_dn7)) / (var_ef_nsubp * var_ef_nsubp)) / (2.0 * assign14000_e8290)), ((((var_t1_dn8 * var_ef_nsubp) - (var_t1 * var_ef_nsubp_dn8)) / (var_ef_nsubp * var_ef_nsubp)) / (2.0 * assign14000_e8290)), ((((var_t1_dn9 * var_ef_nsubp) - (var_t1 * var_ef_nsubp_dn9)) / (var_ef_nsubp * var_ef_nsubp)) / (2.0 * assign14000_e8290)), ((((var_t1_dn10 * var_ef_nsubp) - (var_t1 * var_ef_nsubp_dn10)) / (var_ef_nsubp * var_ef_nsubp)) / (2.0 * assign14000_e8290)), ((((var_t1_dn11 * var_ef_nsubp) - (var_t1 * var_ef_nsubp_dn11)) / (var_ef_nsubp * var_ef_nsubp)) / (2.0 * assign14000_e8290)), ((((var_t1_dn14 * var_ef_nsubp) - (var_t1 * var_ef_nsubp_dn14)) / (var_ef_nsubp * var_ef_nsubp)) / (2.0 * assign14000_e8290)),)
    } else {
        (var_wdplp, var_wdplp_dn0, var_wdplp_dn2, var_wdplp_dn4, var_wdplp_dn5, var_wdplp_dn6, var_wdplp_dn7, var_wdplp_dn8, var_wdplp_dn9, var_wdplp_dn10, var_wdplp_dn11, var_wdplp_dn14,)
    }
};
        var_wdplp = assign14000_e8292;
        var_wdplp_dn0 = assign14000_e8292_d_n0;
        var_wdplp_dn2 = assign14000_e8292_d_n2;
        var_wdplp_dn4 = assign14000_e8292_d_n4;
        var_wdplp_dn5 = assign14000_e8292_d_n5;
        var_wdplp_dn6 = assign14000_e8292_d_n6;
        var_wdplp_dn7 = assign14000_e8292_d_n7;
        var_wdplp_dn8 = assign14000_e8292_d_n8;
        var_wdplp_dn9 = assign14000_e8292_d_n9;
        var_wdplp_dn10 = assign14000_e8292_d_n10;
        var_wdplp_dn11 = assign14000_e8292_d_n11;
        var_wdplp_dn14 = assign14000_e8292_d_n14;

        let assign14010_e8295: f64 = if var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        var_guard311 = assign14010_e8295;

        let (assign14020_e8310, assign14020_e8310_d_n0, assign14020_e8310_d_n2, assign14020_e8310_d_n4, assign14020_e8310_d_n5, assign14020_e8310_d_n6, assign14020_e8310_d_n7, assign14020_e8310_d_n8, assign14020_e8310_d_n9, assign14020_e8310_d_n10, assign14020_e8310_d_n11, assign14020_e8310_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard311 != 0.0)) {
        let assign14020_e8301: f64 = (2.0 * 1.034943e-10);
        let assign14020_e8303: f64 = (assign14020_e8301 * 1.6021918e-19);
        let assign14020_e8305: f64 = (assign14020_e8303 * var_nsub);
        let assign14020_e8307: f64 = (assign14020_e8305 * var_beta_inv);
        let assign14020_e8308: f64 = (assign14020_e8307).sqrt();
        (assign14020_e8308, ((((assign14020_e8303 * var_nsub_dn0) * var_beta_inv) + (assign14020_e8305 * var_beta_inv_dn0)) / (2.0 * assign14020_e8308)), ((((assign14020_e8303 * var_nsub_dn2) * var_beta_inv) + (assign14020_e8305 * var_beta_inv_dn2)) / (2.0 * assign14020_e8308)), ((((assign14020_e8303 * var_nsub_dn4) * var_beta_inv) + (assign14020_e8305 * var_beta_inv_dn4)) / (2.0 * assign14020_e8308)), ((((assign14020_e8303 * var_nsub_dn5) * var_beta_inv) + (assign14020_e8305 * var_beta_inv_dn5)) / (2.0 * assign14020_e8308)), ((((assign14020_e8303 * var_nsub_dn6) * var_beta_inv) + (assign14020_e8305 * var_beta_inv_dn6)) / (2.0 * assign14020_e8308)), ((((assign14020_e8303 * var_nsub_dn7) * var_beta_inv) + (assign14020_e8305 * var_beta_inv_dn7)) / (2.0 * assign14020_e8308)), ((((assign14020_e8303 * var_nsub_dn8) * var_beta_inv) + (assign14020_e8305 * var_beta_inv_dn8)) / (2.0 * assign14020_e8308)), ((((assign14020_e8303 * var_nsub_dn9) * var_beta_inv) + (assign14020_e8305 * var_beta_inv_dn9)) / (2.0 * assign14020_e8308)), ((((assign14020_e8303 * var_nsub_dn10) * var_beta_inv) + (assign14020_e8305 * var_beta_inv_dn10)) / (2.0 * assign14020_e8308)), ((((assign14020_e8303 * var_nsub_dn11) * var_beta_inv) + (assign14020_e8305 * var_beta_inv_dn11)) / (2.0 * assign14020_e8308)), ((((assign14020_e8303 * var_nsub_dn14) * var_beta_inv) + (assign14020_e8305 * var_beta_inv_dn14)) / (2.0 * assign14020_e8308)),)
    } else {
        (var_cnst0, var_cnst0_dn0, var_cnst0_dn2, var_cnst0_dn4, var_cnst0_dn5, var_cnst0_dn6, var_cnst0_dn7, var_cnst0_dn8, var_cnst0_dn9, var_cnst0_dn10, var_cnst0_dn11, var_cnst0_dn14,)
    }
};
        var_cnst0 = assign14020_e8310;
        var_cnst0_dn0 = assign14020_e8310_d_n0;
        var_cnst0_dn2 = assign14020_e8310_d_n2;
        var_cnst0_dn4 = assign14020_e8310_d_n4;
        var_cnst0_dn5 = assign14020_e8310_d_n5;
        var_cnst0_dn6 = assign14020_e8310_d_n6;
        var_cnst0_dn7 = assign14020_e8310_d_n7;
        var_cnst0_dn8 = assign14020_e8310_d_n8;
        var_cnst0_dn9 = assign14020_e8310_d_n9;
        var_cnst0_dn10 = assign14020_e8310_d_n10;
        var_cnst0_dn11 = assign14020_e8310_d_n11;
        var_cnst0_dn14 = assign14020_e8310_d_n14;

        let (assign14030_e8318, assign14030_e8318_d_n0, assign14030_e8318_d_n2, assign14030_e8318_d_n4, assign14030_e8318_d_n5, assign14030_e8318_d_n6, assign14030_e8318_d_n7, assign14030_e8318_d_n8, assign14030_e8318_d_n9, assign14030_e8318_d_n10, assign14030_e8318_d_n11, assign14030_e8318_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard311 != 0.0)) {
        let assign14030_e8316: f64 = (var_nin / var_nsub);
        (assign14030_e8316, (((var_nin_dn0 * var_nsub) - (var_nin * var_nsub_dn0)) / (var_nsub * var_nsub)), (((var_nin_dn2 * var_nsub) - (var_nin * var_nsub_dn2)) / (var_nsub * var_nsub)), (((var_nin_dn4 * var_nsub) - (var_nin * var_nsub_dn4)) / (var_nsub * var_nsub)), (((var_nin_dn5 * var_nsub) - (var_nin * var_nsub_dn5)) / (var_nsub * var_nsub)), (((var_nin_dn6 * var_nsub) - (var_nin * var_nsub_dn6)) / (var_nsub * var_nsub)), (((var_nin_dn7 * var_nsub) - (var_nin * var_nsub_dn7)) / (var_nsub * var_nsub)), (((var_nin_dn8 * var_nsub) - (var_nin * var_nsub_dn8)) / (var_nsub * var_nsub)), (((var_nin_dn9 * var_nsub) - (var_nin * var_nsub_dn9)) / (var_nsub * var_nsub)), (((var_nin_dn10 * var_nsub) - (var_nin * var_nsub_dn10)) / (var_nsub * var_nsub)), (((var_nin_dn11 * var_nsub) - (var_nin * var_nsub_dn11)) / (var_nsub * var_nsub)), (((var_nin_dn14 * var_nsub) - (var_nin * var_nsub_dn14)) / (var_nsub * var_nsub)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign14030_e8318;
        var_t1_dn0 = assign14030_e8318_d_n0;
        var_t1_dn2 = assign14030_e8318_d_n2;
        var_t1_dn4 = assign14030_e8318_d_n4;
        var_t1_dn5 = assign14030_e8318_d_n5;
        var_t1_dn6 = assign14030_e8318_d_n6;
        var_t1_dn7 = assign14030_e8318_d_n7;
        var_t1_dn8 = assign14030_e8318_d_n8;
        var_t1_dn9 = assign14030_e8318_d_n9;
        var_t1_dn10 = assign14030_e8318_d_n10;
        var_t1_dn11 = assign14030_e8318_d_n11;
        var_t1_dn14 = assign14030_e8318_d_n14;

        let (assign14040_e8326, assign14040_e8326_d_n0, assign14040_e8326_d_n2, assign14040_e8326_d_n4, assign14040_e8326_d_n5, assign14040_e8326_d_n6, assign14040_e8326_d_n7, assign14040_e8326_d_n8, assign14040_e8326_d_n9, assign14040_e8326_d_n10, assign14040_e8326_d_n11, assign14040_e8326_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard311 != 0.0)) {
        let assign14040_e8324: f64 = (var_t1 * var_t1);
        (assign14040_e8324, ((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)), ((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)), ((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)), ((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)), ((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)), ((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)), ((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)), ((var_t1_dn9 * var_t1) + (var_t1 * var_t1_dn9)), ((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)), ((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)), ((var_t1_dn14 * var_t1) + (var_t1 * var_t1_dn14)),)
    } else {
        (var_cnst1, var_cnst1_dn0, var_cnst1_dn2, var_cnst1_dn4, var_cnst1_dn5, var_cnst1_dn6, var_cnst1_dn7, var_cnst1_dn8, var_cnst1_dn9, var_cnst1_dn10, var_cnst1_dn11, var_cnst1_dn14,)
    }
};
        var_cnst1 = assign14040_e8326;
        var_cnst1_dn0 = assign14040_e8326_d_n0;
        var_cnst1_dn2 = assign14040_e8326_d_n2;
        var_cnst1_dn4 = assign14040_e8326_d_n4;
        var_cnst1_dn5 = assign14040_e8326_d_n5;
        var_cnst1_dn6 = assign14040_e8326_d_n6;
        var_cnst1_dn7 = assign14040_e8326_d_n7;
        var_cnst1_dn8 = assign14040_e8326_d_n8;
        var_cnst1_dn9 = assign14040_e8326_d_n9;
        var_cnst1_dn10 = assign14040_e8326_d_n10;
        var_cnst1_dn11 = assign14040_e8326_d_n11;
        var_cnst1_dn14 = assign14040_e8326_d_n14;

        let assign14050_e8329: f64 = if var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        var_guard312 = assign14050_e8329;

        let assign14060_e8332: f64 = if var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        var_guard313 = assign14060_e8332;

        let (assign14070_e8345, assign14070_e8345_d_n0, assign14070_e8345_d_n2, assign14070_e8345_d_n4, assign14070_e8345_d_n5, assign14070_e8345_d_n6, assign14070_e8345_d_n7, assign14070_e8345_d_n8, assign14070_e8345_d_n9, assign14070_e8345_d_n10, assign14070_e8345_d_n11, assign14070_e8345_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard312 != 0.0)) && (var_guard313 != 0.0)) {
        let assign14070_e8341: f64 = (var_uc_nover / var_nsub);
        let assign14070_e8342: f64 = (assign14070_e8341).sqrt();
        let assign14070_e8343: f64 = (var_cnst0 * assign14070_e8342);
        (assign14070_e8343, ((var_cnst0_dn0 * assign14070_e8342) + (var_cnst0 * ((-((var_uc_nover * var_nsub_dn0) / (var_nsub * var_nsub))) / (2.0 * assign14070_e8342)))), ((var_cnst0_dn2 * assign14070_e8342) + (var_cnst0 * ((-((var_uc_nover * var_nsub_dn2) / (var_nsub * var_nsub))) / (2.0 * assign14070_e8342)))), ((var_cnst0_dn4 * assign14070_e8342) + (var_cnst0 * ((-((var_uc_nover * var_nsub_dn4) / (var_nsub * var_nsub))) / (2.0 * assign14070_e8342)))), ((var_cnst0_dn5 * assign14070_e8342) + (var_cnst0 * ((-((var_uc_nover * var_nsub_dn5) / (var_nsub * var_nsub))) / (2.0 * assign14070_e8342)))), ((var_cnst0_dn6 * assign14070_e8342) + (var_cnst0 * ((-((var_uc_nover * var_nsub_dn6) / (var_nsub * var_nsub))) / (2.0 * assign14070_e8342)))), ((var_cnst0_dn7 * assign14070_e8342) + (var_cnst0 * ((-((var_uc_nover * var_nsub_dn7) / (var_nsub * var_nsub))) / (2.0 * assign14070_e8342)))), ((var_cnst0_dn8 * assign14070_e8342) + (var_cnst0 * ((-((var_uc_nover * var_nsub_dn8) / (var_nsub * var_nsub))) / (2.0 * assign14070_e8342)))), ((var_cnst0_dn9 * assign14070_e8342) + (var_cnst0 * ((-((var_uc_nover * var_nsub_dn9) / (var_nsub * var_nsub))) / (2.0 * assign14070_e8342)))), ((var_cnst0_dn10 * assign14070_e8342) + (var_cnst0 * ((-((var_uc_nover * var_nsub_dn10) / (var_nsub * var_nsub))) / (2.0 * assign14070_e8342)))), ((var_cnst0_dn11 * assign14070_e8342) + (var_cnst0 * ((-((var_uc_nover * var_nsub_dn11) / (var_nsub * var_nsub))) / (2.0 * assign14070_e8342)))), ((var_cnst0_dn14 * assign14070_e8342) + (var_cnst0 * ((-((var_uc_nover * var_nsub_dn14) / (var_nsub * var_nsub))) / (2.0 * assign14070_e8342)))),)
    } else {
        (var_cnst0over, var_cnst0over_dn0, var_cnst0over_dn2, var_cnst0over_dn4, var_cnst0over_dn5, var_cnst0over_dn6, var_cnst0over_dn7, var_cnst0over_dn8, var_cnst0over_dn9, var_cnst0over_dn10, var_cnst0over_dn11, var_cnst0over_dn14,)
    }
};
        var_cnst0over = assign14070_e8345;
        var_cnst0over_dn0 = assign14070_e8345_d_n0;
        var_cnst0over_dn2 = assign14070_e8345_d_n2;
        var_cnst0over_dn4 = assign14070_e8345_d_n4;
        var_cnst0over_dn5 = assign14070_e8345_d_n5;
        var_cnst0over_dn6 = assign14070_e8345_d_n6;
        var_cnst0over_dn7 = assign14070_e8345_d_n7;
        var_cnst0over_dn8 = assign14070_e8345_d_n8;
        var_cnst0over_dn9 = assign14070_e8345_d_n9;
        var_cnst0over_dn10 = assign14070_e8345_d_n10;
        var_cnst0over_dn11 = assign14070_e8345_d_n11;
        var_cnst0over_dn14 = assign14070_e8345_d_n14;

        let assign14080_e8348: f64 = if var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        var_guard314 = assign14080_e8348;

        let (assign14090_e8361, assign14090_e8361_d_n0, assign14090_e8361_d_n2, assign14090_e8361_d_n4, assign14090_e8361_d_n5, assign14090_e8361_d_n6, assign14090_e8361_d_n7, assign14090_e8361_d_n8, assign14090_e8361_d_n9, assign14090_e8361_d_n10, assign14090_e8361_d_n11, assign14090_e8361_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard312 != 0.0)) && (var_guard314 != 0.0)) {
        let assign14090_e8357: f64 = (var_uc_novers / var_nsub);
        let assign14090_e8358: f64 = (assign14090_e8357).sqrt();
        let assign14090_e8359: f64 = (var_cnst0 * assign14090_e8358);
        (assign14090_e8359, ((var_cnst0_dn0 * assign14090_e8358) + (var_cnst0 * ((-((var_uc_novers * var_nsub_dn0) / (var_nsub * var_nsub))) / (2.0 * assign14090_e8358)))), ((var_cnst0_dn2 * assign14090_e8358) + (var_cnst0 * ((-((var_uc_novers * var_nsub_dn2) / (var_nsub * var_nsub))) / (2.0 * assign14090_e8358)))), ((var_cnst0_dn4 * assign14090_e8358) + (var_cnst0 * ((-((var_uc_novers * var_nsub_dn4) / (var_nsub * var_nsub))) / (2.0 * assign14090_e8358)))), ((var_cnst0_dn5 * assign14090_e8358) + (var_cnst0 * ((-((var_uc_novers * var_nsub_dn5) / (var_nsub * var_nsub))) / (2.0 * assign14090_e8358)))), ((var_cnst0_dn6 * assign14090_e8358) + (var_cnst0 * ((-((var_uc_novers * var_nsub_dn6) / (var_nsub * var_nsub))) / (2.0 * assign14090_e8358)))), ((var_cnst0_dn7 * assign14090_e8358) + (var_cnst0 * ((-((var_uc_novers * var_nsub_dn7) / (var_nsub * var_nsub))) / (2.0 * assign14090_e8358)))), ((var_cnst0_dn8 * assign14090_e8358) + (var_cnst0 * ((-((var_uc_novers * var_nsub_dn8) / (var_nsub * var_nsub))) / (2.0 * assign14090_e8358)))), ((var_cnst0_dn9 * assign14090_e8358) + (var_cnst0 * ((-((var_uc_novers * var_nsub_dn9) / (var_nsub * var_nsub))) / (2.0 * assign14090_e8358)))), ((var_cnst0_dn10 * assign14090_e8358) + (var_cnst0 * ((-((var_uc_novers * var_nsub_dn10) / (var_nsub * var_nsub))) / (2.0 * assign14090_e8358)))), ((var_cnst0_dn11 * assign14090_e8358) + (var_cnst0 * ((-((var_uc_novers * var_nsub_dn11) / (var_nsub * var_nsub))) / (2.0 * assign14090_e8358)))), ((var_cnst0_dn14 * assign14090_e8358) + (var_cnst0 * ((-((var_uc_novers * var_nsub_dn14) / (var_nsub * var_nsub))) / (2.0 * assign14090_e8358)))),)
    } else {
        (var_cnst0overs, var_cnst0overs_dn0, var_cnst0overs_dn2, var_cnst0overs_dn4, var_cnst0overs_dn5, var_cnst0overs_dn6, var_cnst0overs_dn7, var_cnst0overs_dn8, var_cnst0overs_dn9, var_cnst0overs_dn10, var_cnst0overs_dn11, var_cnst0overs_dn14,)
    }
};
        var_cnst0overs = assign14090_e8361;
        var_cnst0overs_dn0 = assign14090_e8361_d_n0;
        var_cnst0overs_dn2 = assign14090_e8361_d_n2;
        var_cnst0overs_dn4 = assign14090_e8361_d_n4;
        var_cnst0overs_dn5 = assign14090_e8361_d_n5;
        var_cnst0overs_dn6 = assign14090_e8361_d_n6;
        var_cnst0overs_dn7 = assign14090_e8361_d_n7;
        var_cnst0overs_dn8 = assign14090_e8361_d_n8;
        var_cnst0overs_dn9 = assign14090_e8361_d_n9;
        var_cnst0overs_dn10 = assign14090_e8361_d_n10;
        var_cnst0overs_dn11 = assign14090_e8361_d_n11;
        var_cnst0overs_dn14 = assign14090_e8361_d_n14;

        let assign14100_e8364: f64 = if var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        var_guard315 = assign14100_e8364;

        let (assign14110_e8378, assign14110_e8378_d_n0, assign14110_e8378_d_n2, assign14110_e8378_d_n4, assign14110_e8378_d_n5, assign14110_e8378_d_n6, assign14110_e8378_d_n7, assign14110_e8378_d_n8, assign14110_e8378_d_n9, assign14110_e8378_d_n10, assign14110_e8378_d_n11, assign14110_e8378_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard312 == 0.0)) && (var_guard315 != 0.0)) {
        let assign14110_e8374: f64 = (var_uc_nover / var_uc_ndepm);
        let assign14110_e8375: f64 = (assign14110_e8374).sqrt();
        let assign14110_e8376: f64 = (var_cnst0 * assign14110_e8375);
        (assign14110_e8376, ((var_cnst0_dn0 * assign14110_e8375) + (var_cnst0 * ((-((var_uc_nover * var_uc_ndepm_dn0) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14110_e8375)))), ((var_cnst0_dn2 * assign14110_e8375) + (var_cnst0 * ((-((var_uc_nover * var_uc_ndepm_dn2) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14110_e8375)))), ((var_cnst0_dn4 * assign14110_e8375) + (var_cnst0 * ((-((var_uc_nover * var_uc_ndepm_dn4) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14110_e8375)))), ((var_cnst0_dn5 * assign14110_e8375) + (var_cnst0 * ((-((var_uc_nover * var_uc_ndepm_dn5) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14110_e8375)))), ((var_cnst0_dn6 * assign14110_e8375) + (var_cnst0 * ((-((var_uc_nover * var_uc_ndepm_dn6) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14110_e8375)))), ((var_cnst0_dn7 * assign14110_e8375) + (var_cnst0 * ((-((var_uc_nover * var_uc_ndepm_dn7) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14110_e8375)))), ((var_cnst0_dn8 * assign14110_e8375) + (var_cnst0 * ((-((var_uc_nover * var_uc_ndepm_dn8) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14110_e8375)))), ((var_cnst0_dn9 * assign14110_e8375) + (var_cnst0 * ((-((var_uc_nover * var_uc_ndepm_dn9) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14110_e8375)))), ((var_cnst0_dn10 * assign14110_e8375) + (var_cnst0 * ((-((var_uc_nover * var_uc_ndepm_dn10) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14110_e8375)))), ((var_cnst0_dn11 * assign14110_e8375) + (var_cnst0 * ((-((var_uc_nover * var_uc_ndepm_dn11) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14110_e8375)))), ((var_cnst0_dn14 * assign14110_e8375) + (var_cnst0 * ((-((var_uc_nover * var_uc_ndepm_dn14) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14110_e8375)))),)
    } else {
        (var_cnst0over, var_cnst0over_dn0, var_cnst0over_dn2, var_cnst0over_dn4, var_cnst0over_dn5, var_cnst0over_dn6, var_cnst0over_dn7, var_cnst0over_dn8, var_cnst0over_dn9, var_cnst0over_dn10, var_cnst0over_dn11, var_cnst0over_dn14,)
    }
};
        var_cnst0over = assign14110_e8378;
        var_cnst0over_dn0 = assign14110_e8378_d_n0;
        var_cnst0over_dn2 = assign14110_e8378_d_n2;
        var_cnst0over_dn4 = assign14110_e8378_d_n4;
        var_cnst0over_dn5 = assign14110_e8378_d_n5;
        var_cnst0over_dn6 = assign14110_e8378_d_n6;
        var_cnst0over_dn7 = assign14110_e8378_d_n7;
        var_cnst0over_dn8 = assign14110_e8378_d_n8;
        var_cnst0over_dn9 = assign14110_e8378_d_n9;
        var_cnst0over_dn10 = assign14110_e8378_d_n10;
        var_cnst0over_dn11 = assign14110_e8378_d_n11;
        var_cnst0over_dn14 = assign14110_e8378_d_n14;

        let assign14120_e8381: f64 = if var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        var_guard316 = assign14120_e8381;

        let (assign14130_e8395, assign14130_e8395_d_n0, assign14130_e8395_d_n2, assign14130_e8395_d_n4, assign14130_e8395_d_n5, assign14130_e8395_d_n6, assign14130_e8395_d_n7, assign14130_e8395_d_n8, assign14130_e8395_d_n9, assign14130_e8395_d_n10, assign14130_e8395_d_n11, assign14130_e8395_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard312 == 0.0)) && (var_guard316 != 0.0)) {
        let assign14130_e8391: f64 = (var_uc_novers / var_uc_ndepm);
        let assign14130_e8392: f64 = (assign14130_e8391).sqrt();
        let assign14130_e8393: f64 = (var_cnst0 * assign14130_e8392);
        (assign14130_e8393, ((var_cnst0_dn0 * assign14130_e8392) + (var_cnst0 * ((-((var_uc_novers * var_uc_ndepm_dn0) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14130_e8392)))), ((var_cnst0_dn2 * assign14130_e8392) + (var_cnst0 * ((-((var_uc_novers * var_uc_ndepm_dn2) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14130_e8392)))), ((var_cnst0_dn4 * assign14130_e8392) + (var_cnst0 * ((-((var_uc_novers * var_uc_ndepm_dn4) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14130_e8392)))), ((var_cnst0_dn5 * assign14130_e8392) + (var_cnst0 * ((-((var_uc_novers * var_uc_ndepm_dn5) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14130_e8392)))), ((var_cnst0_dn6 * assign14130_e8392) + (var_cnst0 * ((-((var_uc_novers * var_uc_ndepm_dn6) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14130_e8392)))), ((var_cnst0_dn7 * assign14130_e8392) + (var_cnst0 * ((-((var_uc_novers * var_uc_ndepm_dn7) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14130_e8392)))), ((var_cnst0_dn8 * assign14130_e8392) + (var_cnst0 * ((-((var_uc_novers * var_uc_ndepm_dn8) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14130_e8392)))), ((var_cnst0_dn9 * assign14130_e8392) + (var_cnst0 * ((-((var_uc_novers * var_uc_ndepm_dn9) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14130_e8392)))), ((var_cnst0_dn10 * assign14130_e8392) + (var_cnst0 * ((-((var_uc_novers * var_uc_ndepm_dn10) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14130_e8392)))), ((var_cnst0_dn11 * assign14130_e8392) + (var_cnst0 * ((-((var_uc_novers * var_uc_ndepm_dn11) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14130_e8392)))), ((var_cnst0_dn14 * assign14130_e8392) + (var_cnst0 * ((-((var_uc_novers * var_uc_ndepm_dn14) / (var_uc_ndepm * var_uc_ndepm))) / (2.0 * assign14130_e8392)))),)
    } else {
        (var_cnst0overs, var_cnst0overs_dn0, var_cnst0overs_dn2, var_cnst0overs_dn4, var_cnst0overs_dn5, var_cnst0overs_dn6, var_cnst0overs_dn7, var_cnst0overs_dn8, var_cnst0overs_dn9, var_cnst0overs_dn10, var_cnst0overs_dn11, var_cnst0overs_dn14,)
    }
};
        var_cnst0overs = assign14130_e8395;
        var_cnst0overs_dn0 = assign14130_e8395_d_n0;
        var_cnst0overs_dn2 = assign14130_e8395_d_n2;
        var_cnst0overs_dn4 = assign14130_e8395_d_n4;
        var_cnst0overs_dn5 = assign14130_e8395_d_n5;
        var_cnst0overs_dn6 = assign14130_e8395_d_n6;
        var_cnst0overs_dn7 = assign14130_e8395_d_n7;
        var_cnst0overs_dn8 = assign14130_e8395_d_n8;
        var_cnst0overs_dn9 = assign14130_e8395_d_n9;
        var_cnst0overs_dn10 = assign14130_e8395_d_n10;
        var_cnst0overs_dn11 = assign14130_e8395_d_n11;
        var_cnst0overs_dn14 = assign14130_e8395_d_n14;

        let assign14140_e8398: f64 = if var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        var_guard317 = assign14140_e8398;

        let assign14150_e8401: f64 = if var_uc_rd > 0.0 { 1.0 } else { 0.0 };
        var_guard318 = assign14150_e8401;

        let (assign14160_e8425, assign14160_e8425_d_n0, assign14160_e8425_d_n2, assign14160_e8425_d_n4, assign14160_e8425_d_n5, assign14160_e8425_d_n6, assign14160_e8425_d_n7, assign14160_e8425_d_n8, assign14160_e8425_d_n9, assign14160_e8425_d_n10, assign14160_e8425_d_n11, assign14160_e8425_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard318 != 0.0)) {
        let assign14160_e8410: f64 = (p.p67 * var_uc_rdslp1);
        let assign14160_e8412: f64 = (assign14160_e8410 * 1000000.0);
        let assign14160_e8414: f64 = (assign14160_e8412 + var_uc_rdict1);
        let assign14160_e8415: f64 = (var_rdtemp0 * assign14160_e8414);
        let assign14160_e8418: f64 = (p.p68 * p.p100);
        let assign14160_e8420: f64 = (assign14160_e8418 * 1000000.0);
        let assign14160_e8422: f64 = (assign14160_e8420 + p.p101);
        let assign14160_e8423: f64 = (assign14160_e8415 * assign14160_e8422);
        (assign14160_e8423, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign14160_e8425;
        var_t2_dn0 = assign14160_e8425_d_n0;
        var_t2_dn2 = assign14160_e8425_d_n2;
        var_t2_dn4 = assign14160_e8425_d_n4;
        var_t2_dn5 = assign14160_e8425_d_n5;
        var_t2_dn6 = assign14160_e8425_d_n6;
        var_t2_dn7 = assign14160_e8425_d_n7;
        var_t2_dn8 = assign14160_e8425_d_n8;
        var_t2_dn9 = assign14160_e8425_d_n9;
        var_t2_dn10 = assign14160_e8425_d_n10;
        var_t2_dn11 = assign14160_e8425_d_n11;
        var_t2_dn14 = assign14160_e8425_d_n14;

        let assign14170_e8428: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        var_guard319 = assign14170_e8428;

        *var_cnst0_slot = var_cnst0;
        *var_cnst0_dn0_slot = var_cnst0_dn0;
        *var_cnst0_dn10_slot = var_cnst0_dn10;
        *var_cnst0_dn11_slot = var_cnst0_dn11;
        *var_cnst0_dn14_slot = var_cnst0_dn14;
        *var_cnst0_dn2_slot = var_cnst0_dn2;
        *var_cnst0_dn4_slot = var_cnst0_dn4;
        *var_cnst0_dn5_slot = var_cnst0_dn5;
        *var_cnst0_dn6_slot = var_cnst0_dn6;
        *var_cnst0_dn7_slot = var_cnst0_dn7;
        *var_cnst0_dn8_slot = var_cnst0_dn8;
        *var_cnst0_dn9_slot = var_cnst0_dn9;
        *var_cnst0over_slot = var_cnst0over;
        *var_cnst0over_dn0_slot = var_cnst0over_dn0;
        *var_cnst0over_dn10_slot = var_cnst0over_dn10;
        *var_cnst0over_dn11_slot = var_cnst0over_dn11;
        *var_cnst0over_dn14_slot = var_cnst0over_dn14;
        *var_cnst0over_dn2_slot = var_cnst0over_dn2;
        *var_cnst0over_dn4_slot = var_cnst0over_dn4;
        *var_cnst0over_dn5_slot = var_cnst0over_dn5;
        *var_cnst0over_dn6_slot = var_cnst0over_dn6;
        *var_cnst0over_dn7_slot = var_cnst0over_dn7;
        *var_cnst0over_dn8_slot = var_cnst0over_dn8;
        *var_cnst0over_dn9_slot = var_cnst0over_dn9;
        *var_cnst0overs_slot = var_cnst0overs;
        *var_cnst0overs_dn0_slot = var_cnst0overs_dn0;
        *var_cnst0overs_dn10_slot = var_cnst0overs_dn10;
        *var_cnst0overs_dn11_slot = var_cnst0overs_dn11;
        *var_cnst0overs_dn14_slot = var_cnst0overs_dn14;
        *var_cnst0overs_dn2_slot = var_cnst0overs_dn2;
        *var_cnst0overs_dn4_slot = var_cnst0overs_dn4;
        *var_cnst0overs_dn5_slot = var_cnst0overs_dn5;
        *var_cnst0overs_dn6_slot = var_cnst0overs_dn6;
        *var_cnst0overs_dn7_slot = var_cnst0overs_dn7;
        *var_cnst0overs_dn8_slot = var_cnst0overs_dn8;
        *var_cnst0overs_dn9_slot = var_cnst0overs_dn9;
        *var_cnst1_slot = var_cnst1;
        *var_cnst1_dn0_slot = var_cnst1_dn0;
        *var_cnst1_dn10_slot = var_cnst1_dn10;
        *var_cnst1_dn11_slot = var_cnst1_dn11;
        *var_cnst1_dn14_slot = var_cnst1_dn14;
        *var_cnst1_dn2_slot = var_cnst1_dn2;
        *var_cnst1_dn4_slot = var_cnst1_dn4;
        *var_cnst1_dn5_slot = var_cnst1_dn5;
        *var_cnst1_dn6_slot = var_cnst1_dn6;
        *var_cnst1_dn7_slot = var_cnst1_dn7;
        *var_cnst1_dn8_slot = var_cnst1_dn8;
        *var_cnst1_dn9_slot = var_cnst1_dn9;
        *var_guard311_slot = var_guard311;
        *var_guard312_slot = var_guard312;
        *var_guard313_slot = var_guard313;
        *var_guard314_slot = var_guard314;
        *var_guard315_slot = var_guard315;
        *var_guard316_slot = var_guard316;
        *var_guard317_slot = var_guard317;
        *var_guard318_slot = var_guard318;
        *var_guard319_slot = var_guard319;
        *var_pb2_slot = var_pb2;
        *var_pb2_dn0_slot = var_pb2_dn0;
        *var_pb2_dn10_slot = var_pb2_dn10;
        *var_pb2_dn11_slot = var_pb2_dn11;
        *var_pb2_dn14_slot = var_pb2_dn14;
        *var_pb2_dn2_slot = var_pb2_dn2;
        *var_pb2_dn4_slot = var_pb2_dn4;
        *var_pb2_dn5_slot = var_pb2_dn5;
        *var_pb2_dn6_slot = var_pb2_dn6;
        *var_pb2_dn7_slot = var_pb2_dn7;
        *var_pb2_dn8_slot = var_pb2_dn8;
        *var_pb2_dn9_slot = var_pb2_dn9;
        *var_powratio_slot = var_powratio;
        *var_powratio_dn0_slot = var_powratio_dn0;
        *var_powratio_dn10_slot = var_powratio_dn10;
        *var_powratio_dn11_slot = var_powratio_dn11;
        *var_powratio_dn14_slot = var_powratio_dn14;
        *var_powratio_dn2_slot = var_powratio_dn2;
        *var_powratio_dn4_slot = var_powratio_dn4;
        *var_powratio_dn5_slot = var_powratio_dn5;
        *var_powratio_dn6_slot = var_powratio_dn6;
        *var_powratio_dn7_slot = var_powratio_dn7;
        *var_powratio_dn8_slot = var_powratio_dn8;
        *var_powratio_dn9_slot = var_powratio_dn9;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn14_slot = var_tmf1_dn14;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_dn9_slot = var_tmf1_dn9;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn14_slot = var_tmf2_dn14;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_dn9_slot = var_tmf2_dn9;
        *var_wdpl_slot = var_wdpl;
        *var_wdpl_dn0_slot = var_wdpl_dn0;
        *var_wdpl_dn10_slot = var_wdpl_dn10;
        *var_wdpl_dn11_slot = var_wdpl_dn11;
        *var_wdpl_dn14_slot = var_wdpl_dn14;
        *var_wdpl_dn2_slot = var_wdpl_dn2;
        *var_wdpl_dn4_slot = var_wdpl_dn4;
        *var_wdpl_dn5_slot = var_wdpl_dn5;
        *var_wdpl_dn6_slot = var_wdpl_dn6;
        *var_wdpl_dn7_slot = var_wdpl_dn7;
        *var_wdpl_dn8_slot = var_wdpl_dn8;
        *var_wdpl_dn9_slot = var_wdpl_dn9;
        *var_wdplp_slot = var_wdplp;
        *var_wdplp_dn0_slot = var_wdplp_dn0;
        *var_wdplp_dn10_slot = var_wdplp_dn10;
        *var_wdplp_dn11_slot = var_wdplp_dn11;
        *var_wdplp_dn14_slot = var_wdplp_dn14;
        *var_wdplp_dn2_slot = var_wdplp_dn2;
        *var_wdplp_dn4_slot = var_wdplp_dn4;
        *var_wdplp_dn5_slot = var_wdplp_dn5;
        *var_wdplp_dn6_slot = var_wdplp_dn6;
        *var_wdplp_dn7_slot = var_wdplp_dn7;
        *var_wdplp_dn8_slot = var_wdplp_dn8;
        *var_wdplp_dn9_slot = var_wdplp_dn9;
    }

    pub(super) fn stamp_transient_block_26(
        p: &Parameters,
        var_guard293: f64,
        var_guard317: f64,
        var_guard318: f64,
        var_guard319: f64,
        var_mks_rdtemp1: f64,
        var_mks_rdtemp2: f64,
        var_rdtemp0: f64,
        var_tdiff: f64,
        var_tdiff0: f64,
        var_tdiff0_2: f64,
        var_tdiff0_2_dn0: f64,
        var_tdiff0_2_dn10: f64,
        var_tdiff0_2_dn11: f64,
        var_tdiff0_2_dn14: f64,
        var_tdiff0_2_dn2: f64,
        var_tdiff0_2_dn4: f64,
        var_tdiff0_2_dn5: f64,
        var_tdiff0_2_dn6: f64,
        var_tdiff0_2_dn7: f64,
        var_tdiff0_2_dn8: f64,
        var_tdiff0_2_dn9: f64,
        var_tdiff0_dn0: f64,
        var_tdiff0_dn10: f64,
        var_tdiff0_dn11: f64,
        var_tdiff0_dn14: f64,
        var_tdiff0_dn2: f64,
        var_tdiff0_dn4: f64,
        var_tdiff0_dn5: f64,
        var_tdiff0_dn6: f64,
        var_tdiff0_dn7: f64,
        var_tdiff0_dn8: f64,
        var_tdiff0_dn9: f64,
        var_tdiff_2: f64,
        var_tdiff_2_dn0: f64,
        var_tdiff_2_dn10: f64,
        var_tdiff_2_dn11: f64,
        var_tdiff_2_dn14: f64,
        var_tdiff_2_dn2: f64,
        var_tdiff_2_dn4: f64,
        var_tdiff_2_dn5: f64,
        var_tdiff_2_dn6: f64,
        var_tdiff_2_dn7: f64,
        var_tdiff_2_dn8: f64,
        var_tdiff_2_dn9: f64,
        var_tdiff_dn0: f64,
        var_tdiff_dn10: f64,
        var_tdiff_dn11: f64,
        var_tdiff_dn14: f64,
        var_tdiff_dn2: f64,
        var_tdiff_dn4: f64,
        var_tdiff_dn5: f64,
        var_tdiff_dn6: f64,
        var_tdiff_dn7: f64,
        var_tdiff_dn8: f64,
        var_tdiff_dn9: f64,
        var_uc_rd: f64,
        var_uc_rdict1: f64,
        var_uc_rdslp1: f64,
        var_uc_rs: f64,
        var_guard320_slot: &mut f64,
        var_guard321_slot: &mut f64,
        var_rde_slot: &mut f64,
        var_rde_dn0_slot: &mut f64,
        var_rde_dn10_slot: &mut f64,
        var_rde_dn11_slot: &mut f64,
        var_rde_dn14_slot: &mut f64,
        var_rde_dn2_slot: &mut f64,
        var_rde_dn4_slot: &mut f64,
        var_rde_dn5_slot: &mut f64,
        var_rde_dn6_slot: &mut f64,
        var_rde_dn7_slot: &mut f64,
        var_rde_dn8_slot: &mut f64,
        var_rde_dn9_slot: &mut f64,
        var_rse_slot: &mut f64,
        var_rse_dn0_slot: &mut f64,
        var_rse_dn10_slot: &mut f64,
        var_rse_dn11_slot: &mut f64,
        var_rse_dn14_slot: &mut f64,
        var_rse_dn2_slot: &mut f64,
        var_rse_dn4_slot: &mut f64,
        var_rse_dn5_slot: &mut f64,
        var_rse_dn6_slot: &mut f64,
        var_rse_dn7_slot: &mut f64,
        var_rse_dn8_slot: &mut f64,
        var_rse_dn9_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn14_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_dn9_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn14_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_dn9_slot: &mut f64,
    ) {
        let mut var_guard320: f64 = *var_guard320_slot;
        let mut var_guard321: f64 = *var_guard321_slot;
        let mut var_rde: f64 = *var_rde_slot;
        let mut var_rde_dn0: f64 = *var_rde_dn0_slot;
        let mut var_rde_dn10: f64 = *var_rde_dn10_slot;
        let mut var_rde_dn11: f64 = *var_rde_dn11_slot;
        let mut var_rde_dn14: f64 = *var_rde_dn14_slot;
        let mut var_rde_dn2: f64 = *var_rde_dn2_slot;
        let mut var_rde_dn4: f64 = *var_rde_dn4_slot;
        let mut var_rde_dn5: f64 = *var_rde_dn5_slot;
        let mut var_rde_dn6: f64 = *var_rde_dn6_slot;
        let mut var_rde_dn7: f64 = *var_rde_dn7_slot;
        let mut var_rde_dn8: f64 = *var_rde_dn8_slot;
        let mut var_rde_dn9: f64 = *var_rde_dn9_slot;
        let mut var_rse: f64 = *var_rse_slot;
        let mut var_rse_dn0: f64 = *var_rse_dn0_slot;
        let mut var_rse_dn10: f64 = *var_rse_dn10_slot;
        let mut var_rse_dn11: f64 = *var_rse_dn11_slot;
        let mut var_rse_dn14: f64 = *var_rse_dn14_slot;
        let mut var_rse_dn2: f64 = *var_rse_dn2_slot;
        let mut var_rse_dn4: f64 = *var_rse_dn4_slot;
        let mut var_rse_dn5: f64 = *var_rse_dn5_slot;
        let mut var_rse_dn6: f64 = *var_rse_dn6_slot;
        let mut var_rse_dn7: f64 = *var_rse_dn7_slot;
        let mut var_rse_dn8: f64 = *var_rse_dn8_slot;
        let mut var_rse_dn9: f64 = *var_rse_dn9_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn14: f64 = *var_tmf1_dn14_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_dn9: f64 = *var_tmf1_dn9_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn14: f64 = *var_tmf2_dn14_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_dn9: f64 = *var_tmf2_dn9_slot;

        let (assign14180_e8448, assign14180_e8448_d_n0, assign14180_e8448_d_n2, assign14180_e8448_d_n4, assign14180_e8448_d_n5, assign14180_e8448_d_n6, assign14180_e8448_d_n7, assign14180_e8448_d_n8, assign14180_e8448_d_n9, assign14180_e8448_d_n10, assign14180_e8448_d_n11, assign14180_e8448_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard318 != 0.0)) && (var_guard319 != 0.0)) {
        let assign14180_e8439: f64 = (var_mks_rdtemp1 * var_tdiff0);
        let assign14180_e8440: f64 = (var_uc_rd + assign14180_e8439);
        let assign14180_e8443: f64 = (var_mks_rdtemp2 * var_tdiff0_2);
        let assign14180_e8444: f64 = (assign14180_e8440 + assign14180_e8443);
        let assign14180_e8446: f64 = (assign14180_e8444 * var_t2);
        (assign14180_e8446, ((((var_mks_rdtemp1 * var_tdiff0_dn0) + (var_mks_rdtemp2 * var_tdiff0_2_dn0)) * var_t2) + (assign14180_e8444 * var_t2_dn0)), ((((var_mks_rdtemp1 * var_tdiff0_dn2) + (var_mks_rdtemp2 * var_tdiff0_2_dn2)) * var_t2) + (assign14180_e8444 * var_t2_dn2)), ((((var_mks_rdtemp1 * var_tdiff0_dn4) + (var_mks_rdtemp2 * var_tdiff0_2_dn4)) * var_t2) + (assign14180_e8444 * var_t2_dn4)), ((((var_mks_rdtemp1 * var_tdiff0_dn5) + (var_mks_rdtemp2 * var_tdiff0_2_dn5)) * var_t2) + (assign14180_e8444 * var_t2_dn5)), ((((var_mks_rdtemp1 * var_tdiff0_dn6) + (var_mks_rdtemp2 * var_tdiff0_2_dn6)) * var_t2) + (assign14180_e8444 * var_t2_dn6)), ((((var_mks_rdtemp1 * var_tdiff0_dn7) + (var_mks_rdtemp2 * var_tdiff0_2_dn7)) * var_t2) + (assign14180_e8444 * var_t2_dn7)), ((((var_mks_rdtemp1 * var_tdiff0_dn8) + (var_mks_rdtemp2 * var_tdiff0_2_dn8)) * var_t2) + (assign14180_e8444 * var_t2_dn8)), ((((var_mks_rdtemp1 * var_tdiff0_dn9) + (var_mks_rdtemp2 * var_tdiff0_2_dn9)) * var_t2) + (assign14180_e8444 * var_t2_dn9)), ((((var_mks_rdtemp1 * var_tdiff0_dn10) + (var_mks_rdtemp2 * var_tdiff0_2_dn10)) * var_t2) + (assign14180_e8444 * var_t2_dn10)), ((((var_mks_rdtemp1 * var_tdiff0_dn11) + (var_mks_rdtemp2 * var_tdiff0_2_dn11)) * var_t2) + (assign14180_e8444 * var_t2_dn11)), ((((var_mks_rdtemp1 * var_tdiff0_dn14) + (var_mks_rdtemp2 * var_tdiff0_2_dn14)) * var_t2) + (assign14180_e8444 * var_t2_dn14)),)
    } else {
        (var_rde, var_rde_dn0, var_rde_dn2, var_rde_dn4, var_rde_dn5, var_rde_dn6, var_rde_dn7, var_rde_dn8, var_rde_dn9, var_rde_dn10, var_rde_dn11, var_rde_dn14,)
    }
};
        var_rde = assign14180_e8448;
        var_rde_dn0 = assign14180_e8448_d_n0;
        var_rde_dn2 = assign14180_e8448_d_n2;
        var_rde_dn4 = assign14180_e8448_d_n4;
        var_rde_dn5 = assign14180_e8448_d_n5;
        var_rde_dn6 = assign14180_e8448_d_n6;
        var_rde_dn7 = assign14180_e8448_d_n7;
        var_rde_dn8 = assign14180_e8448_d_n8;
        var_rde_dn9 = assign14180_e8448_d_n9;
        var_rde_dn10 = assign14180_e8448_d_n10;
        var_rde_dn11 = assign14180_e8448_d_n11;
        var_rde_dn14 = assign14180_e8448_d_n14;

        let (assign14190_e8466, assign14190_e8466_d_n0, assign14190_e8466_d_n2, assign14190_e8466_d_n4, assign14190_e8466_d_n5, assign14190_e8466_d_n6, assign14190_e8466_d_n7, assign14190_e8466_d_n8, assign14190_e8466_d_n9, assign14190_e8466_d_n10, assign14190_e8466_d_n11, assign14190_e8466_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard318 != 0.0)) && (var_guard319 != 0.0)) {
        let assign14190_e8459: f64 = (0.005 * var_uc_rd);
        let assign14190_e8460: f64 = (var_rde - assign14190_e8459);
        let assign14190_e8463: f64 = (0.01 * var_uc_rd);
        let assign14190_e8464: f64 = (assign14190_e8460 - assign14190_e8463);
        (assign14190_e8464, var_rde_dn0, var_rde_dn2, var_rde_dn4, var_rde_dn5, var_rde_dn6, var_rde_dn7, var_rde_dn8, var_rde_dn9, var_rde_dn10, var_rde_dn11, var_rde_dn14,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign14190_e8466;
        var_tmf1_dn0 = assign14190_e8466_d_n0;
        var_tmf1_dn2 = assign14190_e8466_d_n2;
        var_tmf1_dn4 = assign14190_e8466_d_n4;
        var_tmf1_dn5 = assign14190_e8466_d_n5;
        var_tmf1_dn6 = assign14190_e8466_d_n6;
        var_tmf1_dn7 = assign14190_e8466_d_n7;
        var_tmf1_dn8 = assign14190_e8466_d_n8;
        var_tmf1_dn9 = assign14190_e8466_d_n9;
        var_tmf1_dn10 = assign14190_e8466_d_n10;
        var_tmf1_dn11 = assign14190_e8466_d_n11;
        var_tmf1_dn14 = assign14190_e8466_d_n14;

        let (assign14200_e8484, assign14200_e8484_d_n0, assign14200_e8484_d_n2, assign14200_e8484_d_n4, assign14200_e8484_d_n5, assign14200_e8484_d_n6, assign14200_e8484_d_n7, assign14200_e8484_d_n8, assign14200_e8484_d_n9, assign14200_e8484_d_n10, assign14200_e8484_d_n11, assign14200_e8484_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard318 != 0.0)) && (var_guard319 != 0.0)) {
        let assign14200_e8477: f64 = (0.005 * var_uc_rd);
        let assign14200_e8478: f64 = (4.0 * assign14200_e8477);
        let assign14200_e8481: f64 = (0.01 * var_uc_rd);
        let assign14200_e8482: f64 = (assign14200_e8478 * assign14200_e8481);
        (assign14200_e8482, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14200_e8484;
        var_tmf2_dn0 = assign14200_e8484_d_n0;
        var_tmf2_dn2 = assign14200_e8484_d_n2;
        var_tmf2_dn4 = assign14200_e8484_d_n4;
        var_tmf2_dn5 = assign14200_e8484_d_n5;
        var_tmf2_dn6 = assign14200_e8484_d_n6;
        var_tmf2_dn7 = assign14200_e8484_d_n7;
        var_tmf2_dn8 = assign14200_e8484_d_n8;
        var_tmf2_dn9 = assign14200_e8484_d_n9;
        var_tmf2_dn10 = assign14200_e8484_d_n10;
        var_tmf2_dn11 = assign14200_e8484_d_n11;
        var_tmf2_dn14 = assign14200_e8484_d_n14;

        let (assign14210_e8500, assign14210_e8500_d_n0, assign14210_e8500_d_n2, assign14210_e8500_d_n4, assign14210_e8500_d_n5, assign14210_e8500_d_n6, assign14210_e8500_d_n7, assign14210_e8500_d_n8, assign14210_e8500_d_n9, assign14210_e8500_d_n10, assign14210_e8500_d_n11, assign14210_e8500_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard318 != 0.0)) && (var_guard319 != 0.0)) {
        let (assign14210_e8498, assign14210_e8498_d_n0, assign14210_e8498_d_n2, assign14210_e8498_d_n4, assign14210_e8498_d_n5, assign14210_e8498_d_n6, assign14210_e8498_d_n7, assign14210_e8498_d_n8, assign14210_e8498_d_n9, assign14210_e8498_d_n10, assign14210_e8498_d_n11, assign14210_e8498_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign14210_e8497: f64 = (-var_tmf2);
                (assign14210_e8497, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign14210_e8498, assign14210_e8498_d_n0, assign14210_e8498_d_n2, assign14210_e8498_d_n4, assign14210_e8498_d_n5, assign14210_e8498_d_n6, assign14210_e8498_d_n7, assign14210_e8498_d_n8, assign14210_e8498_d_n9, assign14210_e8498_d_n10, assign14210_e8498_d_n11, assign14210_e8498_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14210_e8500;
        var_tmf2_dn0 = assign14210_e8500_d_n0;
        var_tmf2_dn2 = assign14210_e8500_d_n2;
        var_tmf2_dn4 = assign14210_e8500_d_n4;
        var_tmf2_dn5 = assign14210_e8500_d_n5;
        var_tmf2_dn6 = assign14210_e8500_d_n6;
        var_tmf2_dn7 = assign14210_e8500_d_n7;
        var_tmf2_dn8 = assign14210_e8500_d_n8;
        var_tmf2_dn9 = assign14210_e8500_d_n9;
        var_tmf2_dn10 = assign14210_e8500_d_n10;
        var_tmf2_dn11 = assign14210_e8500_d_n11;
        var_tmf2_dn14 = assign14210_e8500_d_n14;

        let (assign14220_e8515, assign14220_e8515_d_n0, assign14220_e8515_d_n2, assign14220_e8515_d_n4, assign14220_e8515_d_n5, assign14220_e8515_d_n6, assign14220_e8515_d_n7, assign14220_e8515_d_n8, assign14220_e8515_d_n9, assign14220_e8515_d_n10, assign14220_e8515_d_n11, assign14220_e8515_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard318 != 0.0)) && (var_guard319 != 0.0)) {
        let assign14220_e8510: f64 = (var_tmf1 * var_tmf1);
        let assign14220_e8512: f64 = (assign14220_e8510 + var_tmf2);
        let assign14220_e8513: f64 = (assign14220_e8512).sqrt();
        (assign14220_e8513, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14220_e8513)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14220_e8513)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14220_e8513)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14220_e8513)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign14220_e8513)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign14220_e8513)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign14220_e8513)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign14220_e8513)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign14220_e8513)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign14220_e8513)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign14220_e8513)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14220_e8515;
        var_tmf2_dn0 = assign14220_e8515_d_n0;
        var_tmf2_dn2 = assign14220_e8515_d_n2;
        var_tmf2_dn4 = assign14220_e8515_d_n4;
        var_tmf2_dn5 = assign14220_e8515_d_n5;
        var_tmf2_dn6 = assign14220_e8515_d_n6;
        var_tmf2_dn7 = assign14220_e8515_d_n7;
        var_tmf2_dn8 = assign14220_e8515_d_n8;
        var_tmf2_dn9 = assign14220_e8515_d_n9;
        var_tmf2_dn10 = assign14220_e8515_d_n10;
        var_tmf2_dn11 = assign14220_e8515_d_n11;
        var_tmf2_dn14 = assign14220_e8515_d_n14;

        let (assign14230_e8531, assign14230_e8531_d_n0, assign14230_e8531_d_n2, assign14230_e8531_d_n4, assign14230_e8531_d_n5, assign14230_e8531_d_n6, assign14230_e8531_d_n7, assign14230_e8531_d_n8, assign14230_e8531_d_n9, assign14230_e8531_d_n10, assign14230_e8531_d_n11, assign14230_e8531_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard318 != 0.0)) && (var_guard319 != 0.0)) {
        let assign14230_e8527: f64 = (var_tmf1 / var_tmf2);
        let assign14230_e8528: f64 = (1.0 + assign14230_e8527);
        let assign14230_e8529: f64 = (0.5 * assign14230_e8528);
        (assign14230_e8529, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign14230_e8531;
        var_t0_dn0 = assign14230_e8531_d_n0;
        var_t0_dn2 = assign14230_e8531_d_n2;
        var_t0_dn4 = assign14230_e8531_d_n4;
        var_t0_dn5 = assign14230_e8531_d_n5;
        var_t0_dn6 = assign14230_e8531_d_n6;
        var_t0_dn7 = assign14230_e8531_d_n7;
        var_t0_dn8 = assign14230_e8531_d_n8;
        var_t0_dn9 = assign14230_e8531_d_n9;
        var_t0_dn10 = assign14230_e8531_d_n10;
        var_t0_dn11 = assign14230_e8531_d_n11;
        var_t0_dn14 = assign14230_e8531_d_n14;

        let (assign14240_e8549, assign14240_e8549_d_n0, assign14240_e8549_d_n2, assign14240_e8549_d_n4, assign14240_e8549_d_n5, assign14240_e8549_d_n6, assign14240_e8549_d_n7, assign14240_e8549_d_n8, assign14240_e8549_d_n9, assign14240_e8549_d_n10, assign14240_e8549_d_n11, assign14240_e8549_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard318 != 0.0)) && (var_guard319 != 0.0)) {
        let assign14240_e8541: f64 = (0.005 * var_uc_rd);
        let assign14240_e8545: f64 = (var_tmf1 + var_tmf2);
        let assign14240_e8546: f64 = (0.5 * assign14240_e8545);
        let assign14240_e8547: f64 = (assign14240_e8541 + assign14240_e8546);
        (assign14240_e8547, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_dn6 + var_tmf2_dn6)), (0.5 * (var_tmf1_dn7 + var_tmf2_dn7)), (0.5 * (var_tmf1_dn8 + var_tmf2_dn8)), (0.5 * (var_tmf1_dn9 + var_tmf2_dn9)), (0.5 * (var_tmf1_dn10 + var_tmf2_dn10)), (0.5 * (var_tmf1_dn11 + var_tmf2_dn11)), (0.5 * (var_tmf1_dn14 + var_tmf2_dn14)),)
    } else {
        (var_rde, var_rde_dn0, var_rde_dn2, var_rde_dn4, var_rde_dn5, var_rde_dn6, var_rde_dn7, var_rde_dn8, var_rde_dn9, var_rde_dn10, var_rde_dn11, var_rde_dn14,)
    }
};
        var_rde = assign14240_e8549;
        var_rde_dn0 = assign14240_e8549_d_n0;
        var_rde_dn2 = assign14240_e8549_d_n2;
        var_rde_dn4 = assign14240_e8549_d_n4;
        var_rde_dn5 = assign14240_e8549_d_n5;
        var_rde_dn6 = assign14240_e8549_d_n6;
        var_rde_dn7 = assign14240_e8549_d_n7;
        var_rde_dn8 = assign14240_e8549_d_n8;
        var_rde_dn9 = assign14240_e8549_d_n9;
        var_rde_dn10 = assign14240_e8549_d_n10;
        var_rde_dn11 = assign14240_e8549_d_n11;
        var_rde_dn14 = assign14240_e8549_d_n14;

        let (assign14250_e8570, assign14250_e8570_d_n0, assign14250_e8570_d_n2, assign14250_e8570_d_n4, assign14250_e8570_d_n5, assign14250_e8570_d_n6, assign14250_e8570_d_n7, assign14250_e8570_d_n8, assign14250_e8570_d_n9, assign14250_e8570_d_n10, assign14250_e8570_d_n11, assign14250_e8570_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard318 != 0.0)) && (var_guard319 == 0.0)) {
        let assign14250_e8561: f64 = (var_mks_rdtemp1 * var_tdiff);
        let assign14250_e8562: f64 = (var_uc_rd + assign14250_e8561);
        let assign14250_e8565: f64 = (var_mks_rdtemp2 * var_tdiff_2);
        let assign14250_e8566: f64 = (assign14250_e8562 + assign14250_e8565);
        let assign14250_e8568: f64 = (assign14250_e8566 * var_t2);
        (assign14250_e8568, ((((var_mks_rdtemp1 * var_tdiff_dn0) + (var_mks_rdtemp2 * var_tdiff_2_dn0)) * var_t2) + (assign14250_e8566 * var_t2_dn0)), ((((var_mks_rdtemp1 * var_tdiff_dn2) + (var_mks_rdtemp2 * var_tdiff_2_dn2)) * var_t2) + (assign14250_e8566 * var_t2_dn2)), ((((var_mks_rdtemp1 * var_tdiff_dn4) + (var_mks_rdtemp2 * var_tdiff_2_dn4)) * var_t2) + (assign14250_e8566 * var_t2_dn4)), ((((var_mks_rdtemp1 * var_tdiff_dn5) + (var_mks_rdtemp2 * var_tdiff_2_dn5)) * var_t2) + (assign14250_e8566 * var_t2_dn5)), ((((var_mks_rdtemp1 * var_tdiff_dn6) + (var_mks_rdtemp2 * var_tdiff_2_dn6)) * var_t2) + (assign14250_e8566 * var_t2_dn6)), ((((var_mks_rdtemp1 * var_tdiff_dn7) + (var_mks_rdtemp2 * var_tdiff_2_dn7)) * var_t2) + (assign14250_e8566 * var_t2_dn7)), ((((var_mks_rdtemp1 * var_tdiff_dn8) + (var_mks_rdtemp2 * var_tdiff_2_dn8)) * var_t2) + (assign14250_e8566 * var_t2_dn8)), ((((var_mks_rdtemp1 * var_tdiff_dn9) + (var_mks_rdtemp2 * var_tdiff_2_dn9)) * var_t2) + (assign14250_e8566 * var_t2_dn9)), ((((var_mks_rdtemp1 * var_tdiff_dn10) + (var_mks_rdtemp2 * var_tdiff_2_dn10)) * var_t2) + (assign14250_e8566 * var_t2_dn10)), ((((var_mks_rdtemp1 * var_tdiff_dn11) + (var_mks_rdtemp2 * var_tdiff_2_dn11)) * var_t2) + (assign14250_e8566 * var_t2_dn11)), ((((var_mks_rdtemp1 * var_tdiff_dn14) + (var_mks_rdtemp2 * var_tdiff_2_dn14)) * var_t2) + (assign14250_e8566 * var_t2_dn14)),)
    } else {
        (var_rde, var_rde_dn0, var_rde_dn2, var_rde_dn4, var_rde_dn5, var_rde_dn6, var_rde_dn7, var_rde_dn8, var_rde_dn9, var_rde_dn10, var_rde_dn11, var_rde_dn14,)
    }
};
        var_rde = assign14250_e8570;
        var_rde_dn0 = assign14250_e8570_d_n0;
        var_rde_dn2 = assign14250_e8570_d_n2;
        var_rde_dn4 = assign14250_e8570_d_n4;
        var_rde_dn5 = assign14250_e8570_d_n5;
        var_rde_dn6 = assign14250_e8570_d_n6;
        var_rde_dn7 = assign14250_e8570_d_n7;
        var_rde_dn8 = assign14250_e8570_d_n8;
        var_rde_dn9 = assign14250_e8570_d_n9;
        var_rde_dn10 = assign14250_e8570_d_n10;
        var_rde_dn11 = assign14250_e8570_d_n11;
        var_rde_dn14 = assign14250_e8570_d_n14;

        let (assign14260_e8589, assign14260_e8589_d_n0, assign14260_e8589_d_n2, assign14260_e8589_d_n4, assign14260_e8589_d_n5, assign14260_e8589_d_n6, assign14260_e8589_d_n7, assign14260_e8589_d_n8, assign14260_e8589_d_n9, assign14260_e8589_d_n10, assign14260_e8589_d_n11, assign14260_e8589_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard318 != 0.0)) && (var_guard319 == 0.0)) {
        let assign14260_e8582: f64 = (0.005 * var_uc_rd);
        let assign14260_e8583: f64 = (var_rde - assign14260_e8582);
        let assign14260_e8586: f64 = (0.01 * var_uc_rd);
        let assign14260_e8587: f64 = (assign14260_e8583 - assign14260_e8586);
        (assign14260_e8587, var_rde_dn0, var_rde_dn2, var_rde_dn4, var_rde_dn5, var_rde_dn6, var_rde_dn7, var_rde_dn8, var_rde_dn9, var_rde_dn10, var_rde_dn11, var_rde_dn14,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign14260_e8589;
        var_tmf1_dn0 = assign14260_e8589_d_n0;
        var_tmf1_dn2 = assign14260_e8589_d_n2;
        var_tmf1_dn4 = assign14260_e8589_d_n4;
        var_tmf1_dn5 = assign14260_e8589_d_n5;
        var_tmf1_dn6 = assign14260_e8589_d_n6;
        var_tmf1_dn7 = assign14260_e8589_d_n7;
        var_tmf1_dn8 = assign14260_e8589_d_n8;
        var_tmf1_dn9 = assign14260_e8589_d_n9;
        var_tmf1_dn10 = assign14260_e8589_d_n10;
        var_tmf1_dn11 = assign14260_e8589_d_n11;
        var_tmf1_dn14 = assign14260_e8589_d_n14;

        let (assign14270_e8608, assign14270_e8608_d_n0, assign14270_e8608_d_n2, assign14270_e8608_d_n4, assign14270_e8608_d_n5, assign14270_e8608_d_n6, assign14270_e8608_d_n7, assign14270_e8608_d_n8, assign14270_e8608_d_n9, assign14270_e8608_d_n10, assign14270_e8608_d_n11, assign14270_e8608_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard318 != 0.0)) && (var_guard319 == 0.0)) {
        let assign14270_e8601: f64 = (0.005 * var_uc_rd);
        let assign14270_e8602: f64 = (4.0 * assign14270_e8601);
        let assign14270_e8605: f64 = (0.01 * var_uc_rd);
        let assign14270_e8606: f64 = (assign14270_e8602 * assign14270_e8605);
        (assign14270_e8606, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14270_e8608;
        var_tmf2_dn0 = assign14270_e8608_d_n0;
        var_tmf2_dn2 = assign14270_e8608_d_n2;
        var_tmf2_dn4 = assign14270_e8608_d_n4;
        var_tmf2_dn5 = assign14270_e8608_d_n5;
        var_tmf2_dn6 = assign14270_e8608_d_n6;
        var_tmf2_dn7 = assign14270_e8608_d_n7;
        var_tmf2_dn8 = assign14270_e8608_d_n8;
        var_tmf2_dn9 = assign14270_e8608_d_n9;
        var_tmf2_dn10 = assign14270_e8608_d_n10;
        var_tmf2_dn11 = assign14270_e8608_d_n11;
        var_tmf2_dn14 = assign14270_e8608_d_n14;

        let (assign14280_e8625, assign14280_e8625_d_n0, assign14280_e8625_d_n2, assign14280_e8625_d_n4, assign14280_e8625_d_n5, assign14280_e8625_d_n6, assign14280_e8625_d_n7, assign14280_e8625_d_n8, assign14280_e8625_d_n9, assign14280_e8625_d_n10, assign14280_e8625_d_n11, assign14280_e8625_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard318 != 0.0)) && (var_guard319 == 0.0)) {
        let (assign14280_e8623, assign14280_e8623_d_n0, assign14280_e8623_d_n2, assign14280_e8623_d_n4, assign14280_e8623_d_n5, assign14280_e8623_d_n6, assign14280_e8623_d_n7, assign14280_e8623_d_n8, assign14280_e8623_d_n9, assign14280_e8623_d_n10, assign14280_e8623_d_n11, assign14280_e8623_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign14280_e8622: f64 = (-var_tmf2);
                (assign14280_e8622, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign14280_e8623, assign14280_e8623_d_n0, assign14280_e8623_d_n2, assign14280_e8623_d_n4, assign14280_e8623_d_n5, assign14280_e8623_d_n6, assign14280_e8623_d_n7, assign14280_e8623_d_n8, assign14280_e8623_d_n9, assign14280_e8623_d_n10, assign14280_e8623_d_n11, assign14280_e8623_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14280_e8625;
        var_tmf2_dn0 = assign14280_e8625_d_n0;
        var_tmf2_dn2 = assign14280_e8625_d_n2;
        var_tmf2_dn4 = assign14280_e8625_d_n4;
        var_tmf2_dn5 = assign14280_e8625_d_n5;
        var_tmf2_dn6 = assign14280_e8625_d_n6;
        var_tmf2_dn7 = assign14280_e8625_d_n7;
        var_tmf2_dn8 = assign14280_e8625_d_n8;
        var_tmf2_dn9 = assign14280_e8625_d_n9;
        var_tmf2_dn10 = assign14280_e8625_d_n10;
        var_tmf2_dn11 = assign14280_e8625_d_n11;
        var_tmf2_dn14 = assign14280_e8625_d_n14;

        let (assign14290_e8641, assign14290_e8641_d_n0, assign14290_e8641_d_n2, assign14290_e8641_d_n4, assign14290_e8641_d_n5, assign14290_e8641_d_n6, assign14290_e8641_d_n7, assign14290_e8641_d_n8, assign14290_e8641_d_n9, assign14290_e8641_d_n10, assign14290_e8641_d_n11, assign14290_e8641_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard318 != 0.0)) && (var_guard319 == 0.0)) {
        let assign14290_e8636: f64 = (var_tmf1 * var_tmf1);
        let assign14290_e8638: f64 = (assign14290_e8636 + var_tmf2);
        let assign14290_e8639: f64 = (assign14290_e8638).sqrt();
        (assign14290_e8639, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14290_e8639)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14290_e8639)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14290_e8639)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14290_e8639)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign14290_e8639)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign14290_e8639)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign14290_e8639)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign14290_e8639)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign14290_e8639)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign14290_e8639)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign14290_e8639)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14290_e8641;
        var_tmf2_dn0 = assign14290_e8641_d_n0;
        var_tmf2_dn2 = assign14290_e8641_d_n2;
        var_tmf2_dn4 = assign14290_e8641_d_n4;
        var_tmf2_dn5 = assign14290_e8641_d_n5;
        var_tmf2_dn6 = assign14290_e8641_d_n6;
        var_tmf2_dn7 = assign14290_e8641_d_n7;
        var_tmf2_dn8 = assign14290_e8641_d_n8;
        var_tmf2_dn9 = assign14290_e8641_d_n9;
        var_tmf2_dn10 = assign14290_e8641_d_n10;
        var_tmf2_dn11 = assign14290_e8641_d_n11;
        var_tmf2_dn14 = assign14290_e8641_d_n14;

        let (assign14300_e8658, assign14300_e8658_d_n0, assign14300_e8658_d_n2, assign14300_e8658_d_n4, assign14300_e8658_d_n5, assign14300_e8658_d_n6, assign14300_e8658_d_n7, assign14300_e8658_d_n8, assign14300_e8658_d_n9, assign14300_e8658_d_n10, assign14300_e8658_d_n11, assign14300_e8658_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard318 != 0.0)) && (var_guard319 == 0.0)) {
        let assign14300_e8654: f64 = (var_tmf1 / var_tmf2);
        let assign14300_e8655: f64 = (1.0 + assign14300_e8654);
        let assign14300_e8656: f64 = (0.5 * assign14300_e8655);
        (assign14300_e8656, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign14300_e8658;
        var_t0_dn0 = assign14300_e8658_d_n0;
        var_t0_dn2 = assign14300_e8658_d_n2;
        var_t0_dn4 = assign14300_e8658_d_n4;
        var_t0_dn5 = assign14300_e8658_d_n5;
        var_t0_dn6 = assign14300_e8658_d_n6;
        var_t0_dn7 = assign14300_e8658_d_n7;
        var_t0_dn8 = assign14300_e8658_d_n8;
        var_t0_dn9 = assign14300_e8658_d_n9;
        var_t0_dn10 = assign14300_e8658_d_n10;
        var_t0_dn11 = assign14300_e8658_d_n11;
        var_t0_dn14 = assign14300_e8658_d_n14;

        let (assign14310_e8677, assign14310_e8677_d_n0, assign14310_e8677_d_n2, assign14310_e8677_d_n4, assign14310_e8677_d_n5, assign14310_e8677_d_n6, assign14310_e8677_d_n7, assign14310_e8677_d_n8, assign14310_e8677_d_n9, assign14310_e8677_d_n10, assign14310_e8677_d_n11, assign14310_e8677_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard318 != 0.0)) && (var_guard319 == 0.0)) {
        let assign14310_e8669: f64 = (0.005 * var_uc_rd);
        let assign14310_e8673: f64 = (var_tmf1 + var_tmf2);
        let assign14310_e8674: f64 = (0.5 * assign14310_e8673);
        let assign14310_e8675: f64 = (assign14310_e8669 + assign14310_e8674);
        (assign14310_e8675, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_dn6 + var_tmf2_dn6)), (0.5 * (var_tmf1_dn7 + var_tmf2_dn7)), (0.5 * (var_tmf1_dn8 + var_tmf2_dn8)), (0.5 * (var_tmf1_dn9 + var_tmf2_dn9)), (0.5 * (var_tmf1_dn10 + var_tmf2_dn10)), (0.5 * (var_tmf1_dn11 + var_tmf2_dn11)), (0.5 * (var_tmf1_dn14 + var_tmf2_dn14)),)
    } else {
        (var_rde, var_rde_dn0, var_rde_dn2, var_rde_dn4, var_rde_dn5, var_rde_dn6, var_rde_dn7, var_rde_dn8, var_rde_dn9, var_rde_dn10, var_rde_dn11, var_rde_dn14,)
    }
};
        var_rde = assign14310_e8677;
        var_rde_dn0 = assign14310_e8677_d_n0;
        var_rde_dn2 = assign14310_e8677_d_n2;
        var_rde_dn4 = assign14310_e8677_d_n4;
        var_rde_dn5 = assign14310_e8677_d_n5;
        var_rde_dn6 = assign14310_e8677_d_n6;
        var_rde_dn7 = assign14310_e8677_d_n7;
        var_rde_dn8 = assign14310_e8677_d_n8;
        var_rde_dn9 = assign14310_e8677_d_n9;
        var_rde_dn10 = assign14310_e8677_d_n10;
        var_rde_dn11 = assign14310_e8677_d_n11;
        var_rde_dn14 = assign14310_e8677_d_n14;

        let (assign14320_e8686, assign14320_e8686_d_n0, assign14320_e8686_d_n2, assign14320_e8686_d_n4, assign14320_e8686_d_n5, assign14320_e8686_d_n6, assign14320_e8686_d_n7, assign14320_e8686_d_n8, assign14320_e8686_d_n9, assign14320_e8686_d_n10, assign14320_e8686_d_n11, assign14320_e8686_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard318 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rde, var_rde_dn0, var_rde_dn2, var_rde_dn4, var_rde_dn5, var_rde_dn6, var_rde_dn7, var_rde_dn8, var_rde_dn9, var_rde_dn10, var_rde_dn11, var_rde_dn14,)
    }
};
        var_rde = assign14320_e8686;
        var_rde_dn0 = assign14320_e8686_d_n0;
        var_rde_dn2 = assign14320_e8686_d_n2;
        var_rde_dn4 = assign14320_e8686_d_n4;
        var_rde_dn5 = assign14320_e8686_d_n5;
        var_rde_dn6 = assign14320_e8686_d_n6;
        var_rde_dn7 = assign14320_e8686_d_n7;
        var_rde_dn8 = assign14320_e8686_d_n8;
        var_rde_dn9 = assign14320_e8686_d_n9;
        var_rde_dn10 = assign14320_e8686_d_n10;
        var_rde_dn11 = assign14320_e8686_d_n11;
        var_rde_dn14 = assign14320_e8686_d_n14;

        let assign14330_e8689: f64 = if var_uc_rs > 0.0 { 1.0 } else { 0.0 };
        var_guard320 = assign14330_e8689;

        let (assign14340_e8713, assign14340_e8713_d_n0, assign14340_e8713_d_n2, assign14340_e8713_d_n4, assign14340_e8713_d_n5, assign14340_e8713_d_n6, assign14340_e8713_d_n7, assign14340_e8713_d_n8, assign14340_e8713_d_n9, assign14340_e8713_d_n10, assign14340_e8713_d_n11, assign14340_e8713_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard320 != 0.0)) {
        let assign14340_e8698: f64 = (p.p69 * var_uc_rdslp1);
        let assign14340_e8700: f64 = (assign14340_e8698 * 1000000.0);
        let assign14340_e8702: f64 = (assign14340_e8700 + var_uc_rdict1);
        let assign14340_e8703: f64 = (var_rdtemp0 * assign14340_e8702);
        let assign14340_e8706: f64 = (p.p70 * p.p100);
        let assign14340_e8708: f64 = (assign14340_e8706 * 1000000.0);
        let assign14340_e8710: f64 = (assign14340_e8708 + p.p101);
        let assign14340_e8711: f64 = (assign14340_e8703 * assign14340_e8710);
        (assign14340_e8711, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign14340_e8713;
        var_t2_dn0 = assign14340_e8713_d_n0;
        var_t2_dn2 = assign14340_e8713_d_n2;
        var_t2_dn4 = assign14340_e8713_d_n4;
        var_t2_dn5 = assign14340_e8713_d_n5;
        var_t2_dn6 = assign14340_e8713_d_n6;
        var_t2_dn7 = assign14340_e8713_d_n7;
        var_t2_dn8 = assign14340_e8713_d_n8;
        var_t2_dn9 = assign14340_e8713_d_n9;
        var_t2_dn10 = assign14340_e8713_d_n10;
        var_t2_dn11 = assign14340_e8713_d_n11;
        var_t2_dn14 = assign14340_e8713_d_n14;

        let assign14350_e8716: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        var_guard321 = assign14350_e8716;

        let (assign14360_e8736, assign14360_e8736_d_n0, assign14360_e8736_d_n2, assign14360_e8736_d_n4, assign14360_e8736_d_n5, assign14360_e8736_d_n6, assign14360_e8736_d_n7, assign14360_e8736_d_n8, assign14360_e8736_d_n9, assign14360_e8736_d_n10, assign14360_e8736_d_n11, assign14360_e8736_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard320 != 0.0)) && (var_guard321 != 0.0)) {
        let assign14360_e8727: f64 = (var_mks_rdtemp1 * var_tdiff0);
        let assign14360_e8728: f64 = (var_uc_rs + assign14360_e8727);
        let assign14360_e8731: f64 = (var_mks_rdtemp2 * var_tdiff0_2);
        let assign14360_e8732: f64 = (assign14360_e8728 + assign14360_e8731);
        let assign14360_e8734: f64 = (assign14360_e8732 * var_t2);
        (assign14360_e8734, ((((var_mks_rdtemp1 * var_tdiff0_dn0) + (var_mks_rdtemp2 * var_tdiff0_2_dn0)) * var_t2) + (assign14360_e8732 * var_t2_dn0)), ((((var_mks_rdtemp1 * var_tdiff0_dn2) + (var_mks_rdtemp2 * var_tdiff0_2_dn2)) * var_t2) + (assign14360_e8732 * var_t2_dn2)), ((((var_mks_rdtemp1 * var_tdiff0_dn4) + (var_mks_rdtemp2 * var_tdiff0_2_dn4)) * var_t2) + (assign14360_e8732 * var_t2_dn4)), ((((var_mks_rdtemp1 * var_tdiff0_dn5) + (var_mks_rdtemp2 * var_tdiff0_2_dn5)) * var_t2) + (assign14360_e8732 * var_t2_dn5)), ((((var_mks_rdtemp1 * var_tdiff0_dn6) + (var_mks_rdtemp2 * var_tdiff0_2_dn6)) * var_t2) + (assign14360_e8732 * var_t2_dn6)), ((((var_mks_rdtemp1 * var_tdiff0_dn7) + (var_mks_rdtemp2 * var_tdiff0_2_dn7)) * var_t2) + (assign14360_e8732 * var_t2_dn7)), ((((var_mks_rdtemp1 * var_tdiff0_dn8) + (var_mks_rdtemp2 * var_tdiff0_2_dn8)) * var_t2) + (assign14360_e8732 * var_t2_dn8)), ((((var_mks_rdtemp1 * var_tdiff0_dn9) + (var_mks_rdtemp2 * var_tdiff0_2_dn9)) * var_t2) + (assign14360_e8732 * var_t2_dn9)), ((((var_mks_rdtemp1 * var_tdiff0_dn10) + (var_mks_rdtemp2 * var_tdiff0_2_dn10)) * var_t2) + (assign14360_e8732 * var_t2_dn10)), ((((var_mks_rdtemp1 * var_tdiff0_dn11) + (var_mks_rdtemp2 * var_tdiff0_2_dn11)) * var_t2) + (assign14360_e8732 * var_t2_dn11)), ((((var_mks_rdtemp1 * var_tdiff0_dn14) + (var_mks_rdtemp2 * var_tdiff0_2_dn14)) * var_t2) + (assign14360_e8732 * var_t2_dn14)),)
    } else {
        (var_rse, var_rse_dn0, var_rse_dn2, var_rse_dn4, var_rse_dn5, var_rse_dn6, var_rse_dn7, var_rse_dn8, var_rse_dn9, var_rse_dn10, var_rse_dn11, var_rse_dn14,)
    }
};
        var_rse = assign14360_e8736;
        var_rse_dn0 = assign14360_e8736_d_n0;
        var_rse_dn2 = assign14360_e8736_d_n2;
        var_rse_dn4 = assign14360_e8736_d_n4;
        var_rse_dn5 = assign14360_e8736_d_n5;
        var_rse_dn6 = assign14360_e8736_d_n6;
        var_rse_dn7 = assign14360_e8736_d_n7;
        var_rse_dn8 = assign14360_e8736_d_n8;
        var_rse_dn9 = assign14360_e8736_d_n9;
        var_rse_dn10 = assign14360_e8736_d_n10;
        var_rse_dn11 = assign14360_e8736_d_n11;
        var_rse_dn14 = assign14360_e8736_d_n14;

        let (assign14370_e8754, assign14370_e8754_d_n0, assign14370_e8754_d_n2, assign14370_e8754_d_n4, assign14370_e8754_d_n5, assign14370_e8754_d_n6, assign14370_e8754_d_n7, assign14370_e8754_d_n8, assign14370_e8754_d_n9, assign14370_e8754_d_n10, assign14370_e8754_d_n11, assign14370_e8754_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard320 != 0.0)) && (var_guard321 != 0.0)) {
        let assign14370_e8747: f64 = (0.005 * var_uc_rs);
        let assign14370_e8748: f64 = (var_rse - assign14370_e8747);
        let assign14370_e8751: f64 = (0.01 * var_uc_rs);
        let assign14370_e8752: f64 = (assign14370_e8748 - assign14370_e8751);
        (assign14370_e8752, var_rse_dn0, var_rse_dn2, var_rse_dn4, var_rse_dn5, var_rse_dn6, var_rse_dn7, var_rse_dn8, var_rse_dn9, var_rse_dn10, var_rse_dn11, var_rse_dn14,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign14370_e8754;
        var_tmf1_dn0 = assign14370_e8754_d_n0;
        var_tmf1_dn2 = assign14370_e8754_d_n2;
        var_tmf1_dn4 = assign14370_e8754_d_n4;
        var_tmf1_dn5 = assign14370_e8754_d_n5;
        var_tmf1_dn6 = assign14370_e8754_d_n6;
        var_tmf1_dn7 = assign14370_e8754_d_n7;
        var_tmf1_dn8 = assign14370_e8754_d_n8;
        var_tmf1_dn9 = assign14370_e8754_d_n9;
        var_tmf1_dn10 = assign14370_e8754_d_n10;
        var_tmf1_dn11 = assign14370_e8754_d_n11;
        var_tmf1_dn14 = assign14370_e8754_d_n14;

        let (assign14380_e8772, assign14380_e8772_d_n0, assign14380_e8772_d_n2, assign14380_e8772_d_n4, assign14380_e8772_d_n5, assign14380_e8772_d_n6, assign14380_e8772_d_n7, assign14380_e8772_d_n8, assign14380_e8772_d_n9, assign14380_e8772_d_n10, assign14380_e8772_d_n11, assign14380_e8772_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard320 != 0.0)) && (var_guard321 != 0.0)) {
        let assign14380_e8765: f64 = (0.005 * var_uc_rs);
        let assign14380_e8766: f64 = (4.0 * assign14380_e8765);
        let assign14380_e8769: f64 = (0.01 * var_uc_rs);
        let assign14380_e8770: f64 = (assign14380_e8766 * assign14380_e8769);
        (assign14380_e8770, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14380_e8772;
        var_tmf2_dn0 = assign14380_e8772_d_n0;
        var_tmf2_dn2 = assign14380_e8772_d_n2;
        var_tmf2_dn4 = assign14380_e8772_d_n4;
        var_tmf2_dn5 = assign14380_e8772_d_n5;
        var_tmf2_dn6 = assign14380_e8772_d_n6;
        var_tmf2_dn7 = assign14380_e8772_d_n7;
        var_tmf2_dn8 = assign14380_e8772_d_n8;
        var_tmf2_dn9 = assign14380_e8772_d_n9;
        var_tmf2_dn10 = assign14380_e8772_d_n10;
        var_tmf2_dn11 = assign14380_e8772_d_n11;
        var_tmf2_dn14 = assign14380_e8772_d_n14;

        let (assign14390_e8788, assign14390_e8788_d_n0, assign14390_e8788_d_n2, assign14390_e8788_d_n4, assign14390_e8788_d_n5, assign14390_e8788_d_n6, assign14390_e8788_d_n7, assign14390_e8788_d_n8, assign14390_e8788_d_n9, assign14390_e8788_d_n10, assign14390_e8788_d_n11, assign14390_e8788_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard320 != 0.0)) && (var_guard321 != 0.0)) {
        let (assign14390_e8786, assign14390_e8786_d_n0, assign14390_e8786_d_n2, assign14390_e8786_d_n4, assign14390_e8786_d_n5, assign14390_e8786_d_n6, assign14390_e8786_d_n7, assign14390_e8786_d_n8, assign14390_e8786_d_n9, assign14390_e8786_d_n10, assign14390_e8786_d_n11, assign14390_e8786_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign14390_e8785: f64 = (-var_tmf2);
                (assign14390_e8785, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign14390_e8786, assign14390_e8786_d_n0, assign14390_e8786_d_n2, assign14390_e8786_d_n4, assign14390_e8786_d_n5, assign14390_e8786_d_n6, assign14390_e8786_d_n7, assign14390_e8786_d_n8, assign14390_e8786_d_n9, assign14390_e8786_d_n10, assign14390_e8786_d_n11, assign14390_e8786_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14390_e8788;
        var_tmf2_dn0 = assign14390_e8788_d_n0;
        var_tmf2_dn2 = assign14390_e8788_d_n2;
        var_tmf2_dn4 = assign14390_e8788_d_n4;
        var_tmf2_dn5 = assign14390_e8788_d_n5;
        var_tmf2_dn6 = assign14390_e8788_d_n6;
        var_tmf2_dn7 = assign14390_e8788_d_n7;
        var_tmf2_dn8 = assign14390_e8788_d_n8;
        var_tmf2_dn9 = assign14390_e8788_d_n9;
        var_tmf2_dn10 = assign14390_e8788_d_n10;
        var_tmf2_dn11 = assign14390_e8788_d_n11;
        var_tmf2_dn14 = assign14390_e8788_d_n14;

        *var_guard320_slot = var_guard320;
        *var_guard321_slot = var_guard321;
        *var_rde_slot = var_rde;
        *var_rde_dn0_slot = var_rde_dn0;
        *var_rde_dn10_slot = var_rde_dn10;
        *var_rde_dn11_slot = var_rde_dn11;
        *var_rde_dn14_slot = var_rde_dn14;
        *var_rde_dn2_slot = var_rde_dn2;
        *var_rde_dn4_slot = var_rde_dn4;
        *var_rde_dn5_slot = var_rde_dn5;
        *var_rde_dn6_slot = var_rde_dn6;
        *var_rde_dn7_slot = var_rde_dn7;
        *var_rde_dn8_slot = var_rde_dn8;
        *var_rde_dn9_slot = var_rde_dn9;
        *var_rse_slot = var_rse;
        *var_rse_dn0_slot = var_rse_dn0;
        *var_rse_dn10_slot = var_rse_dn10;
        *var_rse_dn11_slot = var_rse_dn11;
        *var_rse_dn14_slot = var_rse_dn14;
        *var_rse_dn2_slot = var_rse_dn2;
        *var_rse_dn4_slot = var_rse_dn4;
        *var_rse_dn5_slot = var_rse_dn5;
        *var_rse_dn6_slot = var_rse_dn6;
        *var_rse_dn7_slot = var_rse_dn7;
        *var_rse_dn8_slot = var_rse_dn8;
        *var_rse_dn9_slot = var_rse_dn9;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn14_slot = var_tmf1_dn14;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_dn9_slot = var_tmf1_dn9;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn14_slot = var_tmf2_dn14;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_dn9_slot = var_tmf2_dn9;
    }

    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
        var_guard293: f64,
        var_guard317: f64,
        var_guard320: f64,
        var_guard321: f64,
        var_mks_rdtemp1: f64,
        var_mks_rdtemp2: f64,
        var_rdvdtemp0: f64,
        var_rdvdtemp0_dn0: f64,
        var_rdvdtemp0_dn10: f64,
        var_rdvdtemp0_dn11: f64,
        var_rdvdtemp0_dn14: f64,
        var_rdvdtemp0_dn2: f64,
        var_rdvdtemp0_dn4: f64,
        var_rdvdtemp0_dn5: f64,
        var_rdvdtemp0_dn6: f64,
        var_rdvdtemp0_dn7: f64,
        var_rdvdtemp0_dn8: f64,
        var_rdvdtemp0_dn9: f64,
        var_tdiff: f64,
        var_tdiff_2: f64,
        var_tdiff_2_dn0: f64,
        var_tdiff_2_dn10: f64,
        var_tdiff_2_dn11: f64,
        var_tdiff_2_dn14: f64,
        var_tdiff_2_dn2: f64,
        var_tdiff_2_dn4: f64,
        var_tdiff_2_dn5: f64,
        var_tdiff_2_dn6: f64,
        var_tdiff_2_dn7: f64,
        var_tdiff_2_dn8: f64,
        var_tdiff_2_dn9: f64,
        var_tdiff_dn0: f64,
        var_tdiff_dn10: f64,
        var_tdiff_dn11: f64,
        var_tdiff_dn14: f64,
        var_tdiff_dn2: f64,
        var_tdiff_dn4: f64,
        var_tdiff_dn5: f64,
        var_tdiff_dn6: f64,
        var_tdiff_dn7: f64,
        var_tdiff_dn8: f64,
        var_tdiff_dn9: f64,
        var_uc_rdict1: f64,
        var_uc_rdov13: f64,
        var_uc_rdslp1: f64,
        var_uc_rdvd: f64,
        var_uc_rs: f64,
        var_guard322_slot: &mut f64,
        var_guard323_slot: &mut f64,
        var_rse_slot: &mut f64,
        var_rse_dn0_slot: &mut f64,
        var_rse_dn10_slot: &mut f64,
        var_rse_dn11_slot: &mut f64,
        var_rse_dn14_slot: &mut f64,
        var_rse_dn2_slot: &mut f64,
        var_rse_dn4_slot: &mut f64,
        var_rse_dn5_slot: &mut f64,
        var_rse_dn6_slot: &mut f64,
        var_rse_dn7_slot: &mut f64,
        var_rse_dn8_slot: &mut f64,
        var_rse_dn9_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn14_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn0_slot: &mut f64,
        var_t8_dn10_slot: &mut f64,
        var_t8_dn11_slot: &mut f64,
        var_t8_dn14_slot: &mut f64,
        var_t8_dn2_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t8_dn9_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn14_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_dn9_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn14_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_dn9_slot: &mut f64,
    ) {
        let mut var_guard322: f64 = *var_guard322_slot;
        let mut var_guard323: f64 = *var_guard323_slot;
        let mut var_rse: f64 = *var_rse_slot;
        let mut var_rse_dn0: f64 = *var_rse_dn0_slot;
        let mut var_rse_dn10: f64 = *var_rse_dn10_slot;
        let mut var_rse_dn11: f64 = *var_rse_dn11_slot;
        let mut var_rse_dn14: f64 = *var_rse_dn14_slot;
        let mut var_rse_dn2: f64 = *var_rse_dn2_slot;
        let mut var_rse_dn4: f64 = *var_rse_dn4_slot;
        let mut var_rse_dn5: f64 = *var_rse_dn5_slot;
        let mut var_rse_dn6: f64 = *var_rse_dn6_slot;
        let mut var_rse_dn7: f64 = *var_rse_dn7_slot;
        let mut var_rse_dn8: f64 = *var_rse_dn8_slot;
        let mut var_rse_dn9: f64 = *var_rse_dn9_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn14: f64 = *var_t4_dn14_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn0: f64 = *var_t8_dn0_slot;
        let mut var_t8_dn10: f64 = *var_t8_dn10_slot;
        let mut var_t8_dn11: f64 = *var_t8_dn11_slot;
        let mut var_t8_dn14: f64 = *var_t8_dn14_slot;
        let mut var_t8_dn2: f64 = *var_t8_dn2_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t8_dn9: f64 = *var_t8_dn9_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn14: f64 = *var_tmf1_dn14_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_dn9: f64 = *var_tmf1_dn9_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn14: f64 = *var_tmf2_dn14_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_dn9: f64 = *var_tmf2_dn9_slot;

        let (assign14400_e8803, assign14400_e8803_d_n0, assign14400_e8803_d_n2, assign14400_e8803_d_n4, assign14400_e8803_d_n5, assign14400_e8803_d_n6, assign14400_e8803_d_n7, assign14400_e8803_d_n8, assign14400_e8803_d_n9, assign14400_e8803_d_n10, assign14400_e8803_d_n11, assign14400_e8803_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard320 != 0.0)) && (var_guard321 != 0.0)) {
        let assign14400_e8798: f64 = (var_tmf1 * var_tmf1);
        let assign14400_e8800: f64 = (assign14400_e8798 + var_tmf2);
        let assign14400_e8801: f64 = (assign14400_e8800).sqrt();
        (assign14400_e8801, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14400_e8801)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14400_e8801)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14400_e8801)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14400_e8801)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign14400_e8801)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign14400_e8801)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign14400_e8801)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign14400_e8801)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign14400_e8801)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign14400_e8801)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign14400_e8801)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14400_e8803;
        var_tmf2_dn0 = assign14400_e8803_d_n0;
        var_tmf2_dn2 = assign14400_e8803_d_n2;
        var_tmf2_dn4 = assign14400_e8803_d_n4;
        var_tmf2_dn5 = assign14400_e8803_d_n5;
        var_tmf2_dn6 = assign14400_e8803_d_n6;
        var_tmf2_dn7 = assign14400_e8803_d_n7;
        var_tmf2_dn8 = assign14400_e8803_d_n8;
        var_tmf2_dn9 = assign14400_e8803_d_n9;
        var_tmf2_dn10 = assign14400_e8803_d_n10;
        var_tmf2_dn11 = assign14400_e8803_d_n11;
        var_tmf2_dn14 = assign14400_e8803_d_n14;

        let (assign14410_e8819, assign14410_e8819_d_n0, assign14410_e8819_d_n2, assign14410_e8819_d_n4, assign14410_e8819_d_n5, assign14410_e8819_d_n6, assign14410_e8819_d_n7, assign14410_e8819_d_n8, assign14410_e8819_d_n9, assign14410_e8819_d_n10, assign14410_e8819_d_n11, assign14410_e8819_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard320 != 0.0)) && (var_guard321 != 0.0)) {
        let assign14410_e8815: f64 = (var_tmf1 / var_tmf2);
        let assign14410_e8816: f64 = (1.0 + assign14410_e8815);
        let assign14410_e8817: f64 = (0.5 * assign14410_e8816);
        (assign14410_e8817, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign14410_e8819;
        var_t0_dn0 = assign14410_e8819_d_n0;
        var_t0_dn2 = assign14410_e8819_d_n2;
        var_t0_dn4 = assign14410_e8819_d_n4;
        var_t0_dn5 = assign14410_e8819_d_n5;
        var_t0_dn6 = assign14410_e8819_d_n6;
        var_t0_dn7 = assign14410_e8819_d_n7;
        var_t0_dn8 = assign14410_e8819_d_n8;
        var_t0_dn9 = assign14410_e8819_d_n9;
        var_t0_dn10 = assign14410_e8819_d_n10;
        var_t0_dn11 = assign14410_e8819_d_n11;
        var_t0_dn14 = assign14410_e8819_d_n14;

        let (assign14420_e8837, assign14420_e8837_d_n0, assign14420_e8837_d_n2, assign14420_e8837_d_n4, assign14420_e8837_d_n5, assign14420_e8837_d_n6, assign14420_e8837_d_n7, assign14420_e8837_d_n8, assign14420_e8837_d_n9, assign14420_e8837_d_n10, assign14420_e8837_d_n11, assign14420_e8837_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard320 != 0.0)) && (var_guard321 != 0.0)) {
        let assign14420_e8829: f64 = (0.005 * var_uc_rs);
        let assign14420_e8833: f64 = (var_tmf1 + var_tmf2);
        let assign14420_e8834: f64 = (0.5 * assign14420_e8833);
        let assign14420_e8835: f64 = (assign14420_e8829 + assign14420_e8834);
        (assign14420_e8835, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_dn6 + var_tmf2_dn6)), (0.5 * (var_tmf1_dn7 + var_tmf2_dn7)), (0.5 * (var_tmf1_dn8 + var_tmf2_dn8)), (0.5 * (var_tmf1_dn9 + var_tmf2_dn9)), (0.5 * (var_tmf1_dn10 + var_tmf2_dn10)), (0.5 * (var_tmf1_dn11 + var_tmf2_dn11)), (0.5 * (var_tmf1_dn14 + var_tmf2_dn14)),)
    } else {
        (var_rse, var_rse_dn0, var_rse_dn2, var_rse_dn4, var_rse_dn5, var_rse_dn6, var_rse_dn7, var_rse_dn8, var_rse_dn9, var_rse_dn10, var_rse_dn11, var_rse_dn14,)
    }
};
        var_rse = assign14420_e8837;
        var_rse_dn0 = assign14420_e8837_d_n0;
        var_rse_dn2 = assign14420_e8837_d_n2;
        var_rse_dn4 = assign14420_e8837_d_n4;
        var_rse_dn5 = assign14420_e8837_d_n5;
        var_rse_dn6 = assign14420_e8837_d_n6;
        var_rse_dn7 = assign14420_e8837_d_n7;
        var_rse_dn8 = assign14420_e8837_d_n8;
        var_rse_dn9 = assign14420_e8837_d_n9;
        var_rse_dn10 = assign14420_e8837_d_n10;
        var_rse_dn11 = assign14420_e8837_d_n11;
        var_rse_dn14 = assign14420_e8837_d_n14;

        let (assign14430_e8858, assign14430_e8858_d_n0, assign14430_e8858_d_n2, assign14430_e8858_d_n4, assign14430_e8858_d_n5, assign14430_e8858_d_n6, assign14430_e8858_d_n7, assign14430_e8858_d_n8, assign14430_e8858_d_n9, assign14430_e8858_d_n10, assign14430_e8858_d_n11, assign14430_e8858_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard320 != 0.0)) && (var_guard321 == 0.0)) {
        let assign14430_e8849: f64 = (var_mks_rdtemp1 * var_tdiff);
        let assign14430_e8850: f64 = (var_uc_rs + assign14430_e8849);
        let assign14430_e8853: f64 = (var_mks_rdtemp2 * var_tdiff_2);
        let assign14430_e8854: f64 = (assign14430_e8850 + assign14430_e8853);
        let assign14430_e8856: f64 = (assign14430_e8854 * var_t2);
        (assign14430_e8856, ((((var_mks_rdtemp1 * var_tdiff_dn0) + (var_mks_rdtemp2 * var_tdiff_2_dn0)) * var_t2) + (assign14430_e8854 * var_t2_dn0)), ((((var_mks_rdtemp1 * var_tdiff_dn2) + (var_mks_rdtemp2 * var_tdiff_2_dn2)) * var_t2) + (assign14430_e8854 * var_t2_dn2)), ((((var_mks_rdtemp1 * var_tdiff_dn4) + (var_mks_rdtemp2 * var_tdiff_2_dn4)) * var_t2) + (assign14430_e8854 * var_t2_dn4)), ((((var_mks_rdtemp1 * var_tdiff_dn5) + (var_mks_rdtemp2 * var_tdiff_2_dn5)) * var_t2) + (assign14430_e8854 * var_t2_dn5)), ((((var_mks_rdtemp1 * var_tdiff_dn6) + (var_mks_rdtemp2 * var_tdiff_2_dn6)) * var_t2) + (assign14430_e8854 * var_t2_dn6)), ((((var_mks_rdtemp1 * var_tdiff_dn7) + (var_mks_rdtemp2 * var_tdiff_2_dn7)) * var_t2) + (assign14430_e8854 * var_t2_dn7)), ((((var_mks_rdtemp1 * var_tdiff_dn8) + (var_mks_rdtemp2 * var_tdiff_2_dn8)) * var_t2) + (assign14430_e8854 * var_t2_dn8)), ((((var_mks_rdtemp1 * var_tdiff_dn9) + (var_mks_rdtemp2 * var_tdiff_2_dn9)) * var_t2) + (assign14430_e8854 * var_t2_dn9)), ((((var_mks_rdtemp1 * var_tdiff_dn10) + (var_mks_rdtemp2 * var_tdiff_2_dn10)) * var_t2) + (assign14430_e8854 * var_t2_dn10)), ((((var_mks_rdtemp1 * var_tdiff_dn11) + (var_mks_rdtemp2 * var_tdiff_2_dn11)) * var_t2) + (assign14430_e8854 * var_t2_dn11)), ((((var_mks_rdtemp1 * var_tdiff_dn14) + (var_mks_rdtemp2 * var_tdiff_2_dn14)) * var_t2) + (assign14430_e8854 * var_t2_dn14)),)
    } else {
        (var_rse, var_rse_dn0, var_rse_dn2, var_rse_dn4, var_rse_dn5, var_rse_dn6, var_rse_dn7, var_rse_dn8, var_rse_dn9, var_rse_dn10, var_rse_dn11, var_rse_dn14,)
    }
};
        var_rse = assign14430_e8858;
        var_rse_dn0 = assign14430_e8858_d_n0;
        var_rse_dn2 = assign14430_e8858_d_n2;
        var_rse_dn4 = assign14430_e8858_d_n4;
        var_rse_dn5 = assign14430_e8858_d_n5;
        var_rse_dn6 = assign14430_e8858_d_n6;
        var_rse_dn7 = assign14430_e8858_d_n7;
        var_rse_dn8 = assign14430_e8858_d_n8;
        var_rse_dn9 = assign14430_e8858_d_n9;
        var_rse_dn10 = assign14430_e8858_d_n10;
        var_rse_dn11 = assign14430_e8858_d_n11;
        var_rse_dn14 = assign14430_e8858_d_n14;

        let (assign14440_e8877, assign14440_e8877_d_n0, assign14440_e8877_d_n2, assign14440_e8877_d_n4, assign14440_e8877_d_n5, assign14440_e8877_d_n6, assign14440_e8877_d_n7, assign14440_e8877_d_n8, assign14440_e8877_d_n9, assign14440_e8877_d_n10, assign14440_e8877_d_n11, assign14440_e8877_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard320 != 0.0)) && (var_guard321 == 0.0)) {
        let assign14440_e8870: f64 = (0.005 * var_uc_rs);
        let assign14440_e8871: f64 = (var_rse - assign14440_e8870);
        let assign14440_e8874: f64 = (0.01 * var_uc_rs);
        let assign14440_e8875: f64 = (assign14440_e8871 - assign14440_e8874);
        (assign14440_e8875, var_rse_dn0, var_rse_dn2, var_rse_dn4, var_rse_dn5, var_rse_dn6, var_rse_dn7, var_rse_dn8, var_rse_dn9, var_rse_dn10, var_rse_dn11, var_rse_dn14,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign14440_e8877;
        var_tmf1_dn0 = assign14440_e8877_d_n0;
        var_tmf1_dn2 = assign14440_e8877_d_n2;
        var_tmf1_dn4 = assign14440_e8877_d_n4;
        var_tmf1_dn5 = assign14440_e8877_d_n5;
        var_tmf1_dn6 = assign14440_e8877_d_n6;
        var_tmf1_dn7 = assign14440_e8877_d_n7;
        var_tmf1_dn8 = assign14440_e8877_d_n8;
        var_tmf1_dn9 = assign14440_e8877_d_n9;
        var_tmf1_dn10 = assign14440_e8877_d_n10;
        var_tmf1_dn11 = assign14440_e8877_d_n11;
        var_tmf1_dn14 = assign14440_e8877_d_n14;

        let (assign14450_e8896, assign14450_e8896_d_n0, assign14450_e8896_d_n2, assign14450_e8896_d_n4, assign14450_e8896_d_n5, assign14450_e8896_d_n6, assign14450_e8896_d_n7, assign14450_e8896_d_n8, assign14450_e8896_d_n9, assign14450_e8896_d_n10, assign14450_e8896_d_n11, assign14450_e8896_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard320 != 0.0)) && (var_guard321 == 0.0)) {
        let assign14450_e8889: f64 = (0.005 * var_uc_rs);
        let assign14450_e8890: f64 = (4.0 * assign14450_e8889);
        let assign14450_e8893: f64 = (0.01 * var_uc_rs);
        let assign14450_e8894: f64 = (assign14450_e8890 * assign14450_e8893);
        (assign14450_e8894, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14450_e8896;
        var_tmf2_dn0 = assign14450_e8896_d_n0;
        var_tmf2_dn2 = assign14450_e8896_d_n2;
        var_tmf2_dn4 = assign14450_e8896_d_n4;
        var_tmf2_dn5 = assign14450_e8896_d_n5;
        var_tmf2_dn6 = assign14450_e8896_d_n6;
        var_tmf2_dn7 = assign14450_e8896_d_n7;
        var_tmf2_dn8 = assign14450_e8896_d_n8;
        var_tmf2_dn9 = assign14450_e8896_d_n9;
        var_tmf2_dn10 = assign14450_e8896_d_n10;
        var_tmf2_dn11 = assign14450_e8896_d_n11;
        var_tmf2_dn14 = assign14450_e8896_d_n14;

        let (assign14460_e8913, assign14460_e8913_d_n0, assign14460_e8913_d_n2, assign14460_e8913_d_n4, assign14460_e8913_d_n5, assign14460_e8913_d_n6, assign14460_e8913_d_n7, assign14460_e8913_d_n8, assign14460_e8913_d_n9, assign14460_e8913_d_n10, assign14460_e8913_d_n11, assign14460_e8913_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard320 != 0.0)) && (var_guard321 == 0.0)) {
        let (assign14460_e8911, assign14460_e8911_d_n0, assign14460_e8911_d_n2, assign14460_e8911_d_n4, assign14460_e8911_d_n5, assign14460_e8911_d_n6, assign14460_e8911_d_n7, assign14460_e8911_d_n8, assign14460_e8911_d_n9, assign14460_e8911_d_n10, assign14460_e8911_d_n11, assign14460_e8911_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign14460_e8910: f64 = (-var_tmf2);
                (assign14460_e8910, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign14460_e8911, assign14460_e8911_d_n0, assign14460_e8911_d_n2, assign14460_e8911_d_n4, assign14460_e8911_d_n5, assign14460_e8911_d_n6, assign14460_e8911_d_n7, assign14460_e8911_d_n8, assign14460_e8911_d_n9, assign14460_e8911_d_n10, assign14460_e8911_d_n11, assign14460_e8911_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14460_e8913;
        var_tmf2_dn0 = assign14460_e8913_d_n0;
        var_tmf2_dn2 = assign14460_e8913_d_n2;
        var_tmf2_dn4 = assign14460_e8913_d_n4;
        var_tmf2_dn5 = assign14460_e8913_d_n5;
        var_tmf2_dn6 = assign14460_e8913_d_n6;
        var_tmf2_dn7 = assign14460_e8913_d_n7;
        var_tmf2_dn8 = assign14460_e8913_d_n8;
        var_tmf2_dn9 = assign14460_e8913_d_n9;
        var_tmf2_dn10 = assign14460_e8913_d_n10;
        var_tmf2_dn11 = assign14460_e8913_d_n11;
        var_tmf2_dn14 = assign14460_e8913_d_n14;

        let (assign14470_e8929, assign14470_e8929_d_n0, assign14470_e8929_d_n2, assign14470_e8929_d_n4, assign14470_e8929_d_n5, assign14470_e8929_d_n6, assign14470_e8929_d_n7, assign14470_e8929_d_n8, assign14470_e8929_d_n9, assign14470_e8929_d_n10, assign14470_e8929_d_n11, assign14470_e8929_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard320 != 0.0)) && (var_guard321 == 0.0)) {
        let assign14470_e8924: f64 = (var_tmf1 * var_tmf1);
        let assign14470_e8926: f64 = (assign14470_e8924 + var_tmf2);
        let assign14470_e8927: f64 = (assign14470_e8926).sqrt();
        (assign14470_e8927, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14470_e8927)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14470_e8927)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14470_e8927)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14470_e8927)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign14470_e8927)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign14470_e8927)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign14470_e8927)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign14470_e8927)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign14470_e8927)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign14470_e8927)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign14470_e8927)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14470_e8929;
        var_tmf2_dn0 = assign14470_e8929_d_n0;
        var_tmf2_dn2 = assign14470_e8929_d_n2;
        var_tmf2_dn4 = assign14470_e8929_d_n4;
        var_tmf2_dn5 = assign14470_e8929_d_n5;
        var_tmf2_dn6 = assign14470_e8929_d_n6;
        var_tmf2_dn7 = assign14470_e8929_d_n7;
        var_tmf2_dn8 = assign14470_e8929_d_n8;
        var_tmf2_dn9 = assign14470_e8929_d_n9;
        var_tmf2_dn10 = assign14470_e8929_d_n10;
        var_tmf2_dn11 = assign14470_e8929_d_n11;
        var_tmf2_dn14 = assign14470_e8929_d_n14;

        let (assign14480_e8946, assign14480_e8946_d_n0, assign14480_e8946_d_n2, assign14480_e8946_d_n4, assign14480_e8946_d_n5, assign14480_e8946_d_n6, assign14480_e8946_d_n7, assign14480_e8946_d_n8, assign14480_e8946_d_n9, assign14480_e8946_d_n10, assign14480_e8946_d_n11, assign14480_e8946_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard320 != 0.0)) && (var_guard321 == 0.0)) {
        let assign14480_e8942: f64 = (var_tmf1 / var_tmf2);
        let assign14480_e8943: f64 = (1.0 + assign14480_e8942);
        let assign14480_e8944: f64 = (0.5 * assign14480_e8943);
        (assign14480_e8944, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign14480_e8946;
        var_t0_dn0 = assign14480_e8946_d_n0;
        var_t0_dn2 = assign14480_e8946_d_n2;
        var_t0_dn4 = assign14480_e8946_d_n4;
        var_t0_dn5 = assign14480_e8946_d_n5;
        var_t0_dn6 = assign14480_e8946_d_n6;
        var_t0_dn7 = assign14480_e8946_d_n7;
        var_t0_dn8 = assign14480_e8946_d_n8;
        var_t0_dn9 = assign14480_e8946_d_n9;
        var_t0_dn10 = assign14480_e8946_d_n10;
        var_t0_dn11 = assign14480_e8946_d_n11;
        var_t0_dn14 = assign14480_e8946_d_n14;

        let (assign14490_e8965, assign14490_e8965_d_n0, assign14490_e8965_d_n2, assign14490_e8965_d_n4, assign14490_e8965_d_n5, assign14490_e8965_d_n6, assign14490_e8965_d_n7, assign14490_e8965_d_n8, assign14490_e8965_d_n9, assign14490_e8965_d_n10, assign14490_e8965_d_n11, assign14490_e8965_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard320 != 0.0)) && (var_guard321 == 0.0)) {
        let assign14490_e8957: f64 = (0.005 * var_uc_rs);
        let assign14490_e8961: f64 = (var_tmf1 + var_tmf2);
        let assign14490_e8962: f64 = (0.5 * assign14490_e8961);
        let assign14490_e8963: f64 = (assign14490_e8957 + assign14490_e8962);
        (assign14490_e8963, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_dn6 + var_tmf2_dn6)), (0.5 * (var_tmf1_dn7 + var_tmf2_dn7)), (0.5 * (var_tmf1_dn8 + var_tmf2_dn8)), (0.5 * (var_tmf1_dn9 + var_tmf2_dn9)), (0.5 * (var_tmf1_dn10 + var_tmf2_dn10)), (0.5 * (var_tmf1_dn11 + var_tmf2_dn11)), (0.5 * (var_tmf1_dn14 + var_tmf2_dn14)),)
    } else {
        (var_rse, var_rse_dn0, var_rse_dn2, var_rse_dn4, var_rse_dn5, var_rse_dn6, var_rse_dn7, var_rse_dn8, var_rse_dn9, var_rse_dn10, var_rse_dn11, var_rse_dn14,)
    }
};
        var_rse = assign14490_e8965;
        var_rse_dn0 = assign14490_e8965_d_n0;
        var_rse_dn2 = assign14490_e8965_d_n2;
        var_rse_dn4 = assign14490_e8965_d_n4;
        var_rse_dn5 = assign14490_e8965_d_n5;
        var_rse_dn6 = assign14490_e8965_d_n6;
        var_rse_dn7 = assign14490_e8965_d_n7;
        var_rse_dn8 = assign14490_e8965_d_n8;
        var_rse_dn9 = assign14490_e8965_d_n9;
        var_rse_dn10 = assign14490_e8965_d_n10;
        var_rse_dn11 = assign14490_e8965_d_n11;
        var_rse_dn14 = assign14490_e8965_d_n14;

        let (assign14500_e8974, assign14500_e8974_d_n0, assign14500_e8974_d_n2, assign14500_e8974_d_n4, assign14500_e8974_d_n5, assign14500_e8974_d_n6, assign14500_e8974_d_n7, assign14500_e8974_d_n8, assign14500_e8974_d_n9, assign14500_e8974_d_n10, assign14500_e8974_d_n11, assign14500_e8974_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard320 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rse, var_rse_dn0, var_rse_dn2, var_rse_dn4, var_rse_dn5, var_rse_dn6, var_rse_dn7, var_rse_dn8, var_rse_dn9, var_rse_dn10, var_rse_dn11, var_rse_dn14,)
    }
};
        var_rse = assign14500_e8974;
        var_rse_dn0 = assign14500_e8974_d_n0;
        var_rse_dn2 = assign14500_e8974_d_n2;
        var_rse_dn4 = assign14500_e8974_d_n4;
        var_rse_dn5 = assign14500_e8974_d_n5;
        var_rse_dn6 = assign14500_e8974_d_n6;
        var_rse_dn7 = assign14500_e8974_d_n7;
        var_rse_dn8 = assign14500_e8974_d_n8;
        var_rse_dn9 = assign14500_e8974_d_n9;
        var_rse_dn10 = assign14500_e8974_d_n10;
        var_rse_dn11 = assign14500_e8974_d_n11;
        var_rse_dn14 = assign14500_e8974_d_n14;

        let assign14510_e8977: f64 = if var_uc_rdvd > 0.0 { 1.0 } else { 0.0 };
        var_guard322 = assign14510_e8977;

        let (assign14520_e9001, assign14520_e9001_d_n0, assign14520_e9001_d_n2, assign14520_e9001_d_n4, assign14520_e9001_d_n5, assign14520_e9001_d_n6, assign14520_e9001_d_n7, assign14520_e9001_d_n8, assign14520_e9001_d_n9, assign14520_e9001_d_n10, assign14520_e9001_d_n11, assign14520_e9001_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14520_e8986: f64 = (p.p67 * var_uc_rdslp1);
        let assign14520_e8988: f64 = (assign14520_e8986 * 1000000.0);
        let assign14520_e8990: f64 = (assign14520_e8988 + var_uc_rdict1);
        let assign14520_e8991: f64 = (var_rdvdtemp0 * assign14520_e8990);
        let assign14520_e8994: f64 = (p.p68 * p.p100);
        let assign14520_e8996: f64 = (assign14520_e8994 * 1000000.0);
        let assign14520_e8998: f64 = (assign14520_e8996 + p.p101);
        let assign14520_e8999: f64 = (assign14520_e8991 * assign14520_e8998);
        (assign14520_e8999, ((var_rdvdtemp0_dn0 * assign14520_e8990) * assign14520_e8998), ((var_rdvdtemp0_dn2 * assign14520_e8990) * assign14520_e8998), ((var_rdvdtemp0_dn4 * assign14520_e8990) * assign14520_e8998), ((var_rdvdtemp0_dn5 * assign14520_e8990) * assign14520_e8998), ((var_rdvdtemp0_dn6 * assign14520_e8990) * assign14520_e8998), ((var_rdvdtemp0_dn7 * assign14520_e8990) * assign14520_e8998), ((var_rdvdtemp0_dn8 * assign14520_e8990) * assign14520_e8998), ((var_rdvdtemp0_dn9 * assign14520_e8990) * assign14520_e8998), ((var_rdvdtemp0_dn10 * assign14520_e8990) * assign14520_e8998), ((var_rdvdtemp0_dn11 * assign14520_e8990) * assign14520_e8998), ((var_rdvdtemp0_dn14 * assign14520_e8990) * assign14520_e8998),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11, var_t4_dn14,)
    }
};
        var_t4 = assign14520_e9001;
        var_t4_dn0 = assign14520_e9001_d_n0;
        var_t4_dn2 = assign14520_e9001_d_n2;
        var_t4_dn4 = assign14520_e9001_d_n4;
        var_t4_dn5 = assign14520_e9001_d_n5;
        var_t4_dn6 = assign14520_e9001_d_n6;
        var_t4_dn7 = assign14520_e9001_d_n7;
        var_t4_dn8 = assign14520_e9001_d_n8;
        var_t4_dn9 = assign14520_e9001_d_n9;
        var_t4_dn10 = assign14520_e9001_d_n10;
        var_t4_dn11 = assign14520_e9001_d_n11;
        var_t4_dn14 = assign14520_e9001_d_n14;

        let (assign14530_e9015, assign14530_e9015_d_n0, assign14530_e9015_d_n2, assign14530_e9015_d_n4, assign14530_e9015_d_n5, assign14530_e9015_d_n6, assign14530_e9015_d_n7, assign14530_e9015_d_n8, assign14530_e9015_d_n9, assign14530_e9015_d_n10, assign14530_e9015_d_n11, assign14530_e9015_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14530_e9009: f64 = (1.0 - var_uc_rdov13);
        let assign14530_e9011: f64 = (assign14530_e9009 * p.p63);
        let assign14530_e9013: f64 = (assign14530_e9011 * 1000000.0);
        (assign14530_e9013, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign14530_e9015;
        var_t1_dn0 = assign14530_e9015_d_n0;
        var_t1_dn2 = assign14530_e9015_d_n2;
        var_t1_dn4 = assign14530_e9015_d_n4;
        var_t1_dn5 = assign14530_e9015_d_n5;
        var_t1_dn6 = assign14530_e9015_d_n6;
        var_t1_dn7 = assign14530_e9015_d_n7;
        var_t1_dn8 = assign14530_e9015_d_n8;
        var_t1_dn9 = assign14530_e9015_d_n9;
        var_t1_dn10 = assign14530_e9015_d_n10;
        var_t1_dn11 = assign14530_e9015_d_n11;
        var_t1_dn14 = assign14530_e9015_d_n14;

        let (assign14540_e9036, assign14540_e9036_d_n0, assign14540_e9036_d_n2, assign14540_e9036_d_n4, assign14540_e9036_d_n5, assign14540_e9036_d_n6, assign14540_e9036_d_n7, assign14540_e9036_d_n8, assign14540_e9036_d_n9, assign14540_e9036_d_n10, assign14540_e9036_d_n11, assign14540_e9036_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14540_e9023: f64 = (p.p99 * p.p99);
        let assign14540_e9027: f64 = (0.0001 * 0.01);
        let assign14540_e9028: f64 = (4.0 * assign14540_e9027);
        let assign14540_e9031: f64 = (0.0001 * 0.01);
        let assign14540_e9032: f64 = (assign14540_e9028 * assign14540_e9031);
        let assign14540_e9033: f64 = (assign14540_e9023 + assign14540_e9032);
        let assign14540_e9034: f64 = (assign14540_e9033).sqrt();
        (assign14540_e9034, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14540_e9036;
        var_tmf2_dn0 = assign14540_e9036_d_n0;
        var_tmf2_dn2 = assign14540_e9036_d_n2;
        var_tmf2_dn4 = assign14540_e9036_d_n4;
        var_tmf2_dn5 = assign14540_e9036_d_n5;
        var_tmf2_dn6 = assign14540_e9036_d_n6;
        var_tmf2_dn7 = assign14540_e9036_d_n7;
        var_tmf2_dn8 = assign14540_e9036_d_n8;
        var_tmf2_dn9 = assign14540_e9036_d_n9;
        var_tmf2_dn10 = assign14540_e9036_d_n10;
        var_tmf2_dn11 = assign14540_e9036_d_n11;
        var_tmf2_dn14 = assign14540_e9036_d_n14;

        let (assign14550_e9050, assign14550_e9050_d_n0, assign14550_e9050_d_n2, assign14550_e9050_d_n4, assign14550_e9050_d_n5, assign14550_e9050_d_n6, assign14550_e9050_d_n7, assign14550_e9050_d_n8, assign14550_e9050_d_n9, assign14550_e9050_d_n10, assign14550_e9050_d_n11, assign14550_e9050_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14550_e9046: f64 = (p.p99 / var_tmf2);
        let assign14550_e9047: f64 = (1.0 + assign14550_e9046);
        let assign14550_e9048: f64 = (0.5 * assign14550_e9047);
        (assign14550_e9048, (0.5 * (-((p.p99 * var_tmf2_dn0) / (var_tmf2 * var_tmf2)))), (0.5 * (-((p.p99 * var_tmf2_dn2) / (var_tmf2 * var_tmf2)))), (0.5 * (-((p.p99 * var_tmf2_dn4) / (var_tmf2 * var_tmf2)))), (0.5 * (-((p.p99 * var_tmf2_dn5) / (var_tmf2 * var_tmf2)))), (0.5 * (-((p.p99 * var_tmf2_dn6) / (var_tmf2 * var_tmf2)))), (0.5 * (-((p.p99 * var_tmf2_dn7) / (var_tmf2 * var_tmf2)))), (0.5 * (-((p.p99 * var_tmf2_dn8) / (var_tmf2 * var_tmf2)))), (0.5 * (-((p.p99 * var_tmf2_dn9) / (var_tmf2 * var_tmf2)))), (0.5 * (-((p.p99 * var_tmf2_dn10) / (var_tmf2 * var_tmf2)))), (0.5 * (-((p.p99 * var_tmf2_dn11) / (var_tmf2 * var_tmf2)))), (0.5 * (-((p.p99 * var_tmf2_dn14) / (var_tmf2 * var_tmf2)))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign14550_e9050;
        var_t0_dn0 = assign14550_e9050_d_n0;
        var_t0_dn2 = assign14550_e9050_d_n2;
        var_t0_dn4 = assign14550_e9050_d_n4;
        var_t0_dn5 = assign14550_e9050_d_n5;
        var_t0_dn6 = assign14550_e9050_d_n6;
        var_t0_dn7 = assign14550_e9050_d_n7;
        var_t0_dn8 = assign14550_e9050_d_n8;
        var_t0_dn9 = assign14550_e9050_d_n9;
        var_t0_dn10 = assign14550_e9050_d_n10;
        var_t0_dn11 = assign14550_e9050_d_n11;
        var_t0_dn14 = assign14550_e9050_d_n14;

        let (assign14560_e9062, assign14560_e9062_d_n0, assign14560_e9062_d_n2, assign14560_e9062_d_n4, assign14560_e9062_d_n5, assign14560_e9062_d_n6, assign14560_e9062_d_n7, assign14560_e9062_d_n8, assign14560_e9062_d_n9, assign14560_e9062_d_n10, assign14560_e9062_d_n11, assign14560_e9062_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14560_e9059: f64 = (p.p99 + var_tmf2);
        let assign14560_e9060: f64 = (0.5 * assign14560_e9059);
        (assign14560_e9060, (0.5 * var_tmf2_dn0), (0.5 * var_tmf2_dn2), (0.5 * var_tmf2_dn4), (0.5 * var_tmf2_dn5), (0.5 * var_tmf2_dn6), (0.5 * var_tmf2_dn7), (0.5 * var_tmf2_dn8), (0.5 * var_tmf2_dn9), (0.5 * var_tmf2_dn10), (0.5 * var_tmf2_dn11), (0.5 * var_tmf2_dn14),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign14560_e9062;
        var_t2_dn0 = assign14560_e9062_d_n0;
        var_t2_dn2 = assign14560_e9062_d_n2;
        var_t2_dn4 = assign14560_e9062_d_n4;
        var_t2_dn5 = assign14560_e9062_d_n5;
        var_t2_dn6 = assign14560_e9062_d_n6;
        var_t2_dn7 = assign14560_e9062_d_n7;
        var_t2_dn8 = assign14560_e9062_d_n8;
        var_t2_dn9 = assign14560_e9062_d_n9;
        var_t2_dn10 = assign14560_e9062_d_n10;
        var_t2_dn11 = assign14560_e9062_d_n11;
        var_t2_dn14 = assign14560_e9062_d_n14;

        let assign14570_e9065: f64 = if var_t2 < 0.0 { 1.0 } else { 0.0 };
        var_guard323 = assign14570_e9065;

        let (assign14580_e9075, assign14580_e9075_d_n0, assign14580_e9075_d_n2, assign14580_e9075_d_n4, assign14580_e9075_d_n5, assign14580_e9075_d_n6, assign14580_e9075_d_n7, assign14580_e9075_d_n8, assign14580_e9075_d_n9, assign14580_e9075_d_n10, assign14580_e9075_d_n11, assign14580_e9075_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard323 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign14580_e9075;
        var_t2_dn0 = assign14580_e9075_d_n0;
        var_t2_dn2 = assign14580_e9075_d_n2;
        var_t2_dn4 = assign14580_e9075_d_n4;
        var_t2_dn5 = assign14580_e9075_d_n5;
        var_t2_dn6 = assign14580_e9075_d_n6;
        var_t2_dn7 = assign14580_e9075_d_n7;
        var_t2_dn8 = assign14580_e9075_d_n8;
        var_t2_dn9 = assign14580_e9075_d_n9;
        var_t2_dn10 = assign14580_e9075_d_n10;
        var_t2_dn11 = assign14580_e9075_d_n11;
        var_t2_dn14 = assign14580_e9075_d_n14;

        let (assign14590_e9085, assign14590_e9085_d_n0, assign14590_e9085_d_n2, assign14590_e9085_d_n4, assign14590_e9085_d_n5, assign14590_e9085_d_n6, assign14590_e9085_d_n7, assign14590_e9085_d_n8, assign14590_e9085_d_n9, assign14590_e9085_d_n10, assign14590_e9085_d_n11, assign14590_e9085_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard323 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign14590_e9085;
        var_t0_dn0 = assign14590_e9085_d_n0;
        var_t0_dn2 = assign14590_e9085_d_n2;
        var_t0_dn4 = assign14590_e9085_d_n4;
        var_t0_dn5 = assign14590_e9085_d_n5;
        var_t0_dn6 = assign14590_e9085_d_n6;
        var_t0_dn7 = assign14590_e9085_d_n7;
        var_t0_dn8 = assign14590_e9085_d_n8;
        var_t0_dn9 = assign14590_e9085_d_n9;
        var_t0_dn10 = assign14590_e9085_d_n10;
        var_t0_dn11 = assign14590_e9085_d_n11;
        var_t0_dn14 = assign14590_e9085_d_n14;

        let (assign14600_e9096, assign14600_e9096_d_n0, assign14600_e9096_d_n2, assign14600_e9096_d_n4, assign14600_e9096_d_n5, assign14600_e9096_d_n6, assign14600_e9096_d_n7, assign14600_e9096_d_n8, assign14600_e9096_d_n9, assign14600_e9096_d_n10, assign14600_e9096_d_n11, assign14600_e9096_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14600_e9092: f64 = (-p.p98);
        let assign14600_e9094: f64 = (assign14600_e9092 / var_t2);
        (assign14600_e9094, (-((assign14600_e9092 * var_t2_dn0) / (var_t2 * var_t2))), (-((assign14600_e9092 * var_t2_dn2) / (var_t2 * var_t2))), (-((assign14600_e9092 * var_t2_dn4) / (var_t2 * var_t2))), (-((assign14600_e9092 * var_t2_dn5) / (var_t2 * var_t2))), (-((assign14600_e9092 * var_t2_dn6) / (var_t2 * var_t2))), (-((assign14600_e9092 * var_t2_dn7) / (var_t2 * var_t2))), (-((assign14600_e9092 * var_t2_dn8) / (var_t2 * var_t2))), (-((assign14600_e9092 * var_t2_dn9) / (var_t2 * var_t2))), (-((assign14600_e9092 * var_t2_dn10) / (var_t2 * var_t2))), (-((assign14600_e9092 * var_t2_dn11) / (var_t2 * var_t2))), (-((assign14600_e9092 * var_t2_dn14) / (var_t2 * var_t2))),)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn11, var_t8_dn14,)
    }
};
        var_t8 = assign14600_e9096;
        var_t8_dn0 = assign14600_e9096_d_n0;
        var_t8_dn2 = assign14600_e9096_d_n2;
        var_t8_dn4 = assign14600_e9096_d_n4;
        var_t8_dn5 = assign14600_e9096_d_n5;
        var_t8_dn6 = assign14600_e9096_d_n6;
        var_t8_dn7 = assign14600_e9096_d_n7;
        var_t8_dn8 = assign14600_e9096_d_n8;
        var_t8_dn9 = assign14600_e9096_d_n9;
        var_t8_dn10 = assign14600_e9096_d_n10;
        var_t8_dn11 = assign14600_e9096_d_n11;
        var_t8_dn14 = assign14600_e9096_d_n14;

        let (assign14610_e9112, assign14610_e9112_d_n0, assign14610_e9112_d_n2, assign14610_e9112_d_n4, assign14610_e9112_d_n5, assign14610_e9112_d_n6, assign14610_e9112_d_n7, assign14610_e9112_d_n8, assign14610_e9112_d_n9, assign14610_e9112_d_n10, assign14610_e9112_d_n11, assign14610_e9112_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14610_e9104: f64 = (var_t8 * p.p63);
        let assign14610_e9106: f64 = (assign14610_e9104 * 1000000.0);
        let assign14610_e9108: f64 = (assign14610_e9106 + 1.0);
        let assign14610_e9110: f64 = (assign14610_e9108 + p.p98);
        (assign14610_e9110, ((var_t8_dn0 * p.p63) * 1000000.0), ((var_t8_dn2 * p.p63) * 1000000.0), ((var_t8_dn4 * p.p63) * 1000000.0), ((var_t8_dn5 * p.p63) * 1000000.0), ((var_t8_dn6 * p.p63) * 1000000.0), ((var_t8_dn7 * p.p63) * 1000000.0), ((var_t8_dn8 * p.p63) * 1000000.0), ((var_t8_dn9 * p.p63) * 1000000.0), ((var_t8_dn10 * p.p63) * 1000000.0), ((var_t8_dn11 * p.p63) * 1000000.0), ((var_t8_dn14 * p.p63) * 1000000.0),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign14610_e9112;
        var_t3_dn0 = assign14610_e9112_d_n0;
        var_t3_dn2 = assign14610_e9112_d_n2;
        var_t3_dn4 = assign14610_e9112_d_n4;
        var_t3_dn5 = assign14610_e9112_d_n5;
        var_t3_dn6 = assign14610_e9112_d_n6;
        var_t3_dn7 = assign14610_e9112_d_n7;
        var_t3_dn8 = assign14610_e9112_d_n8;
        var_t3_dn9 = assign14610_e9112_d_n9;
        var_t3_dn10 = assign14610_e9112_d_n10;
        var_t3_dn11 = assign14610_e9112_d_n11;
        var_t3_dn14 = assign14610_e9112_d_n14;

        let (assign14620_e9126, assign14620_e9126_d_n0, assign14620_e9126_d_n2, assign14620_e9126_d_n4, assign14620_e9126_d_n5, assign14620_e9126_d_n6, assign14620_e9126_d_n7, assign14620_e9126_d_n8, assign14620_e9126_d_n9, assign14620_e9126_d_n10, assign14620_e9126_d_n11, assign14620_e9126_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14620_e9120: f64 = (var_t3 * var_t4);
        let assign14620_e9122: f64 = (assign14620_e9120 - var_t4);
        let assign14620_e9124: f64 = (assign14620_e9122 - 0.01);
        (assign14620_e9124, (((var_t3_dn0 * var_t4) + (var_t3 * var_t4_dn0)) - var_t4_dn0), (((var_t3_dn2 * var_t4) + (var_t3 * var_t4_dn2)) - var_t4_dn2), (((var_t3_dn4 * var_t4) + (var_t3 * var_t4_dn4)) - var_t4_dn4), (((var_t3_dn5 * var_t4) + (var_t3 * var_t4_dn5)) - var_t4_dn5), (((var_t3_dn6 * var_t4) + (var_t3 * var_t4_dn6)) - var_t4_dn6), (((var_t3_dn7 * var_t4) + (var_t3 * var_t4_dn7)) - var_t4_dn7), (((var_t3_dn8 * var_t4) + (var_t3 * var_t4_dn8)) - var_t4_dn8), (((var_t3_dn9 * var_t4) + (var_t3 * var_t4_dn9)) - var_t4_dn9), (((var_t3_dn10 * var_t4) + (var_t3 * var_t4_dn10)) - var_t4_dn10), (((var_t3_dn11 * var_t4) + (var_t3 * var_t4_dn11)) - var_t4_dn11), (((var_t3_dn14 * var_t4) + (var_t3 * var_t4_dn14)) - var_t4_dn14),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign14620_e9126;
        var_tmf1_dn0 = assign14620_e9126_d_n0;
        var_tmf1_dn2 = assign14620_e9126_d_n2;
        var_tmf1_dn4 = assign14620_e9126_d_n4;
        var_tmf1_dn5 = assign14620_e9126_d_n5;
        var_tmf1_dn6 = assign14620_e9126_d_n6;
        var_tmf1_dn7 = assign14620_e9126_d_n7;
        var_tmf1_dn8 = assign14620_e9126_d_n8;
        var_tmf1_dn9 = assign14620_e9126_d_n9;
        var_tmf1_dn10 = assign14620_e9126_d_n10;
        var_tmf1_dn11 = assign14620_e9126_d_n11;
        var_tmf1_dn14 = assign14620_e9126_d_n14;

        *var_guard322_slot = var_guard322;
        *var_guard323_slot = var_guard323;
        *var_rse_slot = var_rse;
        *var_rse_dn0_slot = var_rse_dn0;
        *var_rse_dn10_slot = var_rse_dn10;
        *var_rse_dn11_slot = var_rse_dn11;
        *var_rse_dn14_slot = var_rse_dn14;
        *var_rse_dn2_slot = var_rse_dn2;
        *var_rse_dn4_slot = var_rse_dn4;
        *var_rse_dn5_slot = var_rse_dn5;
        *var_rse_dn6_slot = var_rse_dn6;
        *var_rse_dn7_slot = var_rse_dn7;
        *var_rse_dn8_slot = var_rse_dn8;
        *var_rse_dn9_slot = var_rse_dn9;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn14_slot = var_t4_dn14;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t8_slot = var_t8;
        *var_t8_dn0_slot = var_t8_dn0;
        *var_t8_dn10_slot = var_t8_dn10;
        *var_t8_dn11_slot = var_t8_dn11;
        *var_t8_dn14_slot = var_t8_dn14;
        *var_t8_dn2_slot = var_t8_dn2;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t8_dn9_slot = var_t8_dn9;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn14_slot = var_tmf1_dn14;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_dn9_slot = var_tmf1_dn9;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn14_slot = var_tmf2_dn14;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_dn9_slot = var_tmf2_dn9;
    }

    pub(super) fn stamp_transient_block_28(
        p: &Parameters,
        var_guard293: f64,
        var_guard317: f64,
        var_guard322: f64,
        var_mks_rdvdtemp1: f64,
        var_mks_rdvdtemp2: f64,
        var_t1: f64,
        var_t1_dn0: f64,
        var_t1_dn10: f64,
        var_t1_dn11: f64,
        var_t1_dn14: f64,
        var_t1_dn2: f64,
        var_t1_dn4: f64,
        var_t1_dn5: f64,
        var_t1_dn6: f64,
        var_t1_dn7: f64,
        var_t1_dn8: f64,
        var_t1_dn9: f64,
        var_t4: f64,
        var_t4_dn0: f64,
        var_t4_dn10: f64,
        var_t4_dn11: f64,
        var_t4_dn14: f64,
        var_t4_dn2: f64,
        var_t4_dn4: f64,
        var_t4_dn5: f64,
        var_t4_dn6: f64,
        var_t4_dn7: f64,
        var_t4_dn8: f64,
        var_t4_dn9: f64,
        var_tdiff0: f64,
        var_tdiff0_2: f64,
        var_tdiff0_2_dn0: f64,
        var_tdiff0_2_dn10: f64,
        var_tdiff0_2_dn11: f64,
        var_tdiff0_2_dn14: f64,
        var_tdiff0_2_dn2: f64,
        var_tdiff0_2_dn4: f64,
        var_tdiff0_2_dn5: f64,
        var_tdiff0_2_dn6: f64,
        var_tdiff0_2_dn7: f64,
        var_tdiff0_2_dn8: f64,
        var_tdiff0_2_dn9: f64,
        var_tdiff0_dn0: f64,
        var_tdiff0_dn10: f64,
        var_tdiff0_dn11: f64,
        var_tdiff0_dn14: f64,
        var_tdiff0_dn2: f64,
        var_tdiff0_dn4: f64,
        var_tdiff0_dn5: f64,
        var_tdiff0_dn6: f64,
        var_tdiff0_dn7: f64,
        var_tdiff0_dn8: f64,
        var_tdiff0_dn9: f64,
        var_uc_rdvd: f64,
        var_guard324_slot: &mut f64,
        var_rdvde_slot: &mut f64,
        var_rdvde_dn0_slot: &mut f64,
        var_rdvde_dn10_slot: &mut f64,
        var_rdvde_dn11_slot: &mut f64,
        var_rdvde_dn14_slot: &mut f64,
        var_rdvde_dn2_slot: &mut f64,
        var_rdvde_dn4_slot: &mut f64,
        var_rdvde_dn5_slot: &mut f64,
        var_rdvde_dn6_slot: &mut f64,
        var_rdvde_dn7_slot: &mut f64,
        var_rdvde_dn8_slot: &mut f64,
        var_rdvde_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn14_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_dn9_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn14_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn0_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn11_slot: &mut f64,
        var_t7_dn14_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn7_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_dn9_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn14_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_dn9_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn14_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_dn9_slot: &mut f64,
    ) {
        let mut var_guard324: f64 = *var_guard324_slot;
        let mut var_rdvde: f64 = *var_rdvde_slot;
        let mut var_rdvde_dn0: f64 = *var_rdvde_dn0_slot;
        let mut var_rdvde_dn10: f64 = *var_rdvde_dn10_slot;
        let mut var_rdvde_dn11: f64 = *var_rdvde_dn11_slot;
        let mut var_rdvde_dn14: f64 = *var_rdvde_dn14_slot;
        let mut var_rdvde_dn2: f64 = *var_rdvde_dn2_slot;
        let mut var_rdvde_dn4: f64 = *var_rdvde_dn4_slot;
        let mut var_rdvde_dn5: f64 = *var_rdvde_dn5_slot;
        let mut var_rdvde_dn6: f64 = *var_rdvde_dn6_slot;
        let mut var_rdvde_dn7: f64 = *var_rdvde_dn7_slot;
        let mut var_rdvde_dn8: f64 = *var_rdvde_dn8_slot;
        let mut var_rdvde_dn9: f64 = *var_rdvde_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn14: f64 = *var_t5_dn14_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_dn9: f64 = *var_t5_dn9_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn14: f64 = *var_t6_dn14_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn0: f64 = *var_t7_dn0_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn14: f64 = *var_t7_dn14_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn7: f64 = *var_t7_dn7_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_dn9: f64 = *var_t7_dn9_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn14: f64 = *var_tmf1_dn14_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_dn9: f64 = *var_tmf1_dn9_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn14: f64 = *var_tmf2_dn14_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_dn9: f64 = *var_tmf2_dn9_slot;

        let (assign14630_e9138, assign14630_e9138_d_n0, assign14630_e9138_d_n2, assign14630_e9138_d_n4, assign14630_e9138_d_n5, assign14630_e9138_d_n6, assign14630_e9138_d_n7, assign14630_e9138_d_n8, assign14630_e9138_d_n9, assign14630_e9138_d_n10, assign14630_e9138_d_n11, assign14630_e9138_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14630_e9134: f64 = (4.0 * var_t4);
        let assign14630_e9136: f64 = (assign14630_e9134 * 0.01);
        (assign14630_e9136, ((4.0 * var_t4_dn0) * 0.01), ((4.0 * var_t4_dn2) * 0.01), ((4.0 * var_t4_dn4) * 0.01), ((4.0 * var_t4_dn5) * 0.01), ((4.0 * var_t4_dn6) * 0.01), ((4.0 * var_t4_dn7) * 0.01), ((4.0 * var_t4_dn8) * 0.01), ((4.0 * var_t4_dn9) * 0.01), ((4.0 * var_t4_dn10) * 0.01), ((4.0 * var_t4_dn11) * 0.01), ((4.0 * var_t4_dn14) * 0.01),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14630_e9138;
        var_tmf2_dn0 = assign14630_e9138_d_n0;
        var_tmf2_dn2 = assign14630_e9138_d_n2;
        var_tmf2_dn4 = assign14630_e9138_d_n4;
        var_tmf2_dn5 = assign14630_e9138_d_n5;
        var_tmf2_dn6 = assign14630_e9138_d_n6;
        var_tmf2_dn7 = assign14630_e9138_d_n7;
        var_tmf2_dn8 = assign14630_e9138_d_n8;
        var_tmf2_dn9 = assign14630_e9138_d_n9;
        var_tmf2_dn10 = assign14630_e9138_d_n10;
        var_tmf2_dn11 = assign14630_e9138_d_n11;
        var_tmf2_dn14 = assign14630_e9138_d_n14;

        let (assign14640_e9152, assign14640_e9152_d_n0, assign14640_e9152_d_n2, assign14640_e9152_d_n4, assign14640_e9152_d_n5, assign14640_e9152_d_n6, assign14640_e9152_d_n7, assign14640_e9152_d_n8, assign14640_e9152_d_n9, assign14640_e9152_d_n10, assign14640_e9152_d_n11, assign14640_e9152_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let (assign14640_e9150, assign14640_e9150_d_n0, assign14640_e9150_d_n2, assign14640_e9150_d_n4, assign14640_e9150_d_n5, assign14640_e9150_d_n6, assign14640_e9150_d_n7, assign14640_e9150_d_n8, assign14640_e9150_d_n9, assign14640_e9150_d_n10, assign14640_e9150_d_n11, assign14640_e9150_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign14640_e9149: f64 = (-var_tmf2);
                (assign14640_e9149, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign14640_e9150, assign14640_e9150_d_n0, assign14640_e9150_d_n2, assign14640_e9150_d_n4, assign14640_e9150_d_n5, assign14640_e9150_d_n6, assign14640_e9150_d_n7, assign14640_e9150_d_n8, assign14640_e9150_d_n9, assign14640_e9150_d_n10, assign14640_e9150_d_n11, assign14640_e9150_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14640_e9152;
        var_tmf2_dn0 = assign14640_e9152_d_n0;
        var_tmf2_dn2 = assign14640_e9152_d_n2;
        var_tmf2_dn4 = assign14640_e9152_d_n4;
        var_tmf2_dn5 = assign14640_e9152_d_n5;
        var_tmf2_dn6 = assign14640_e9152_d_n6;
        var_tmf2_dn7 = assign14640_e9152_d_n7;
        var_tmf2_dn8 = assign14640_e9152_d_n8;
        var_tmf2_dn9 = assign14640_e9152_d_n9;
        var_tmf2_dn10 = assign14640_e9152_d_n10;
        var_tmf2_dn11 = assign14640_e9152_d_n11;
        var_tmf2_dn14 = assign14640_e9152_d_n14;

        let (assign14650_e9165, assign14650_e9165_d_n0, assign14650_e9165_d_n2, assign14650_e9165_d_n4, assign14650_e9165_d_n5, assign14650_e9165_d_n6, assign14650_e9165_d_n7, assign14650_e9165_d_n8, assign14650_e9165_d_n9, assign14650_e9165_d_n10, assign14650_e9165_d_n11, assign14650_e9165_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14650_e9160: f64 = (var_tmf1 * var_tmf1);
        let assign14650_e9162: f64 = (assign14650_e9160 + var_tmf2);
        let assign14650_e9163: f64 = (assign14650_e9162).sqrt();
        (assign14650_e9163, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14650_e9163)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14650_e9163)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14650_e9163)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14650_e9163)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign14650_e9163)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign14650_e9163)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign14650_e9163)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign14650_e9163)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign14650_e9163)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign14650_e9163)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign14650_e9163)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14650_e9165;
        var_tmf2_dn0 = assign14650_e9165_d_n0;
        var_tmf2_dn2 = assign14650_e9165_d_n2;
        var_tmf2_dn4 = assign14650_e9165_d_n4;
        var_tmf2_dn5 = assign14650_e9165_d_n5;
        var_tmf2_dn6 = assign14650_e9165_d_n6;
        var_tmf2_dn7 = assign14650_e9165_d_n7;
        var_tmf2_dn8 = assign14650_e9165_d_n8;
        var_tmf2_dn9 = assign14650_e9165_d_n9;
        var_tmf2_dn10 = assign14650_e9165_d_n10;
        var_tmf2_dn11 = assign14650_e9165_d_n11;
        var_tmf2_dn14 = assign14650_e9165_d_n14;

        let (assign14660_e9179, assign14660_e9179_d_n0, assign14660_e9179_d_n2, assign14660_e9179_d_n4, assign14660_e9179_d_n5, assign14660_e9179_d_n6, assign14660_e9179_d_n7, assign14660_e9179_d_n8, assign14660_e9179_d_n9, assign14660_e9179_d_n10, assign14660_e9179_d_n11, assign14660_e9179_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14660_e9175: f64 = (var_tmf1 / var_tmf2);
        let assign14660_e9176: f64 = (1.0 + assign14660_e9175);
        let assign14660_e9177: f64 = (0.5 * assign14660_e9176);
        (assign14660_e9177, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn14,)
    }
};
        var_t6 = assign14660_e9179;
        var_t6_dn0 = assign14660_e9179_d_n0;
        var_t6_dn2 = assign14660_e9179_d_n2;
        var_t6_dn4 = assign14660_e9179_d_n4;
        var_t6_dn5 = assign14660_e9179_d_n5;
        var_t6_dn6 = assign14660_e9179_d_n6;
        var_t6_dn7 = assign14660_e9179_d_n7;
        var_t6_dn8 = assign14660_e9179_d_n8;
        var_t6_dn9 = assign14660_e9179_d_n9;
        var_t6_dn10 = assign14660_e9179_d_n10;
        var_t6_dn11 = assign14660_e9179_d_n11;
        var_t6_dn14 = assign14660_e9179_d_n14;

        let (assign14670_e9193, assign14670_e9193_d_n0, assign14670_e9193_d_n2, assign14670_e9193_d_n4, assign14670_e9193_d_n5, assign14670_e9193_d_n6, assign14670_e9193_d_n7, assign14670_e9193_d_n8, assign14670_e9193_d_n9, assign14670_e9193_d_n10, assign14670_e9193_d_n11, assign14670_e9193_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14670_e9189: f64 = (var_tmf1 + var_tmf2);
        let assign14670_e9190: f64 = (0.5 * assign14670_e9189);
        let assign14670_e9191: f64 = (var_t4 + assign14670_e9190);
        (assign14670_e9191, (var_t4_dn0 + (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_t4_dn2 + (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_t4_dn4 + (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_t4_dn5 + (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_t4_dn6 + (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_t4_dn7 + (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (var_t4_dn8 + (0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), (var_t4_dn9 + (0.5 * (var_tmf1_dn9 + var_tmf2_dn9))), (var_t4_dn10 + (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_t4_dn11 + (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (var_t4_dn14 + (0.5 * (var_tmf1_dn14 + var_tmf2_dn14))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11, var_t5_dn14,)
    }
};
        var_t5 = assign14670_e9193;
        var_t5_dn0 = assign14670_e9193_d_n0;
        var_t5_dn2 = assign14670_e9193_d_n2;
        var_t5_dn4 = assign14670_e9193_d_n4;
        var_t5_dn5 = assign14670_e9193_d_n5;
        var_t5_dn6 = assign14670_e9193_d_n6;
        var_t5_dn7 = assign14670_e9193_d_n7;
        var_t5_dn8 = assign14670_e9193_d_n8;
        var_t5_dn9 = assign14670_e9193_d_n9;
        var_t5_dn10 = assign14670_e9193_d_n10;
        var_t5_dn11 = assign14670_e9193_d_n11;
        var_t5_dn14 = assign14670_e9193_d_n14;

        let (assign14680_e9209, assign14680_e9209_d_n0, assign14680_e9209_d_n2, assign14680_e9209_d_n4, assign14680_e9209_d_n5, assign14680_e9209_d_n6, assign14680_e9209_d_n7, assign14680_e9209_d_n8, assign14680_e9209_d_n9, assign14680_e9209_d_n10, assign14680_e9209_d_n11, assign14680_e9209_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14680_e9202: f64 = (p.p98 + 1.0);
        let assign14680_e9203: f64 = (var_t4 * assign14680_e9202);
        let assign14680_e9205: f64 = (assign14680_e9203 - var_t5);
        let assign14680_e9207: f64 = (assign14680_e9205 - 5e-5);
        (assign14680_e9207, ((var_t4_dn0 * assign14680_e9202) - var_t5_dn0), ((var_t4_dn2 * assign14680_e9202) - var_t5_dn2), ((var_t4_dn4 * assign14680_e9202) - var_t5_dn4), ((var_t4_dn5 * assign14680_e9202) - var_t5_dn5), ((var_t4_dn6 * assign14680_e9202) - var_t5_dn6), ((var_t4_dn7 * assign14680_e9202) - var_t5_dn7), ((var_t4_dn8 * assign14680_e9202) - var_t5_dn8), ((var_t4_dn9 * assign14680_e9202) - var_t5_dn9), ((var_t4_dn10 * assign14680_e9202) - var_t5_dn10), ((var_t4_dn11 * assign14680_e9202) - var_t5_dn11), ((var_t4_dn14 * assign14680_e9202) - var_t5_dn14),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign14680_e9209;
        var_tmf1_dn0 = assign14680_e9209_d_n0;
        var_tmf1_dn2 = assign14680_e9209_d_n2;
        var_tmf1_dn4 = assign14680_e9209_d_n4;
        var_tmf1_dn5 = assign14680_e9209_d_n5;
        var_tmf1_dn6 = assign14680_e9209_d_n6;
        var_tmf1_dn7 = assign14680_e9209_d_n7;
        var_tmf1_dn8 = assign14680_e9209_d_n8;
        var_tmf1_dn9 = assign14680_e9209_d_n9;
        var_tmf1_dn10 = assign14680_e9209_d_n10;
        var_tmf1_dn11 = assign14680_e9209_d_n11;
        var_tmf1_dn14 = assign14680_e9209_d_n14;

        let (assign14690_e9225, assign14690_e9225_d_n0, assign14690_e9225_d_n2, assign14690_e9225_d_n4, assign14690_e9225_d_n5, assign14690_e9225_d_n6, assign14690_e9225_d_n7, assign14690_e9225_d_n8, assign14690_e9225_d_n9, assign14690_e9225_d_n10, assign14690_e9225_d_n11, assign14690_e9225_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14690_e9219: f64 = (p.p98 + 1.0);
        let assign14690_e9220: f64 = (var_t4 * assign14690_e9219);
        let assign14690_e9221: f64 = (4.0 * assign14690_e9220);
        let assign14690_e9223: f64 = (assign14690_e9221 * 5e-5);
        (assign14690_e9223, ((4.0 * (var_t4_dn0 * assign14690_e9219)) * 5e-5), ((4.0 * (var_t4_dn2 * assign14690_e9219)) * 5e-5), ((4.0 * (var_t4_dn4 * assign14690_e9219)) * 5e-5), ((4.0 * (var_t4_dn5 * assign14690_e9219)) * 5e-5), ((4.0 * (var_t4_dn6 * assign14690_e9219)) * 5e-5), ((4.0 * (var_t4_dn7 * assign14690_e9219)) * 5e-5), ((4.0 * (var_t4_dn8 * assign14690_e9219)) * 5e-5), ((4.0 * (var_t4_dn9 * assign14690_e9219)) * 5e-5), ((4.0 * (var_t4_dn10 * assign14690_e9219)) * 5e-5), ((4.0 * (var_t4_dn11 * assign14690_e9219)) * 5e-5), ((4.0 * (var_t4_dn14 * assign14690_e9219)) * 5e-5),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14690_e9225;
        var_tmf2_dn0 = assign14690_e9225_d_n0;
        var_tmf2_dn2 = assign14690_e9225_d_n2;
        var_tmf2_dn4 = assign14690_e9225_d_n4;
        var_tmf2_dn5 = assign14690_e9225_d_n5;
        var_tmf2_dn6 = assign14690_e9225_d_n6;
        var_tmf2_dn7 = assign14690_e9225_d_n7;
        var_tmf2_dn8 = assign14690_e9225_d_n8;
        var_tmf2_dn9 = assign14690_e9225_d_n9;
        var_tmf2_dn10 = assign14690_e9225_d_n10;
        var_tmf2_dn11 = assign14690_e9225_d_n11;
        var_tmf2_dn14 = assign14690_e9225_d_n14;

        let (assign14700_e9239, assign14700_e9239_d_n0, assign14700_e9239_d_n2, assign14700_e9239_d_n4, assign14700_e9239_d_n5, assign14700_e9239_d_n6, assign14700_e9239_d_n7, assign14700_e9239_d_n8, assign14700_e9239_d_n9, assign14700_e9239_d_n10, assign14700_e9239_d_n11, assign14700_e9239_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let (assign14700_e9237, assign14700_e9237_d_n0, assign14700_e9237_d_n2, assign14700_e9237_d_n4, assign14700_e9237_d_n5, assign14700_e9237_d_n6, assign14700_e9237_d_n7, assign14700_e9237_d_n8, assign14700_e9237_d_n9, assign14700_e9237_d_n10, assign14700_e9237_d_n11, assign14700_e9237_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign14700_e9236: f64 = (-var_tmf2);
                (assign14700_e9236, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign14700_e9237, assign14700_e9237_d_n0, assign14700_e9237_d_n2, assign14700_e9237_d_n4, assign14700_e9237_d_n5, assign14700_e9237_d_n6, assign14700_e9237_d_n7, assign14700_e9237_d_n8, assign14700_e9237_d_n9, assign14700_e9237_d_n10, assign14700_e9237_d_n11, assign14700_e9237_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14700_e9239;
        var_tmf2_dn0 = assign14700_e9239_d_n0;
        var_tmf2_dn2 = assign14700_e9239_d_n2;
        var_tmf2_dn4 = assign14700_e9239_d_n4;
        var_tmf2_dn5 = assign14700_e9239_d_n5;
        var_tmf2_dn6 = assign14700_e9239_d_n6;
        var_tmf2_dn7 = assign14700_e9239_d_n7;
        var_tmf2_dn8 = assign14700_e9239_d_n8;
        var_tmf2_dn9 = assign14700_e9239_d_n9;
        var_tmf2_dn10 = assign14700_e9239_d_n10;
        var_tmf2_dn11 = assign14700_e9239_d_n11;
        var_tmf2_dn14 = assign14700_e9239_d_n14;

        let (assign14710_e9252, assign14710_e9252_d_n0, assign14710_e9252_d_n2, assign14710_e9252_d_n4, assign14710_e9252_d_n5, assign14710_e9252_d_n6, assign14710_e9252_d_n7, assign14710_e9252_d_n8, assign14710_e9252_d_n9, assign14710_e9252_d_n10, assign14710_e9252_d_n11, assign14710_e9252_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14710_e9247: f64 = (var_tmf1 * var_tmf1);
        let assign14710_e9249: f64 = (assign14710_e9247 + var_tmf2);
        let assign14710_e9250: f64 = (assign14710_e9249).sqrt();
        (assign14710_e9250, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14710_e9250)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14710_e9250)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14710_e9250)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14710_e9250)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign14710_e9250)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign14710_e9250)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign14710_e9250)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign14710_e9250)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign14710_e9250)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign14710_e9250)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign14710_e9250)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14710_e9252;
        var_tmf2_dn0 = assign14710_e9252_d_n0;
        var_tmf2_dn2 = assign14710_e9252_d_n2;
        var_tmf2_dn4 = assign14710_e9252_d_n4;
        var_tmf2_dn5 = assign14710_e9252_d_n5;
        var_tmf2_dn6 = assign14710_e9252_d_n6;
        var_tmf2_dn7 = assign14710_e9252_d_n7;
        var_tmf2_dn8 = assign14710_e9252_d_n8;
        var_tmf2_dn9 = assign14710_e9252_d_n9;
        var_tmf2_dn10 = assign14710_e9252_d_n10;
        var_tmf2_dn11 = assign14710_e9252_d_n11;
        var_tmf2_dn14 = assign14710_e9252_d_n14;

        let (assign14720_e9266, assign14720_e9266_d_n0, assign14720_e9266_d_n2, assign14720_e9266_d_n4, assign14720_e9266_d_n5, assign14720_e9266_d_n6, assign14720_e9266_d_n7, assign14720_e9266_d_n8, assign14720_e9266_d_n9, assign14720_e9266_d_n10, assign14720_e9266_d_n11, assign14720_e9266_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14720_e9262: f64 = (var_tmf1 / var_tmf2);
        let assign14720_e9263: f64 = (1.0 + assign14720_e9262);
        let assign14720_e9264: f64 = (0.5 * assign14720_e9263);
        (assign14720_e9264, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn14,)
    }
};
        var_t6 = assign14720_e9266;
        var_t6_dn0 = assign14720_e9266_d_n0;
        var_t6_dn2 = assign14720_e9266_d_n2;
        var_t6_dn4 = assign14720_e9266_d_n4;
        var_t6_dn5 = assign14720_e9266_d_n5;
        var_t6_dn6 = assign14720_e9266_d_n6;
        var_t6_dn7 = assign14720_e9266_d_n7;
        var_t6_dn8 = assign14720_e9266_d_n8;
        var_t6_dn9 = assign14720_e9266_d_n9;
        var_t6_dn10 = assign14720_e9266_d_n10;
        var_t6_dn11 = assign14720_e9266_d_n11;
        var_t6_dn14 = assign14720_e9266_d_n14;

        let (assign14730_e9284, assign14730_e9284_d_n0, assign14730_e9284_d_n2, assign14730_e9284_d_n4, assign14730_e9284_d_n5, assign14730_e9284_d_n6, assign14730_e9284_d_n7, assign14730_e9284_d_n8, assign14730_e9284_d_n9, assign14730_e9284_d_n10, assign14730_e9284_d_n11, assign14730_e9284_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14730_e9275: f64 = (p.p98 + 1.0);
        let assign14730_e9276: f64 = (var_t4 * assign14730_e9275);
        let assign14730_e9280: f64 = (var_tmf1 + var_tmf2);
        let assign14730_e9281: f64 = (0.5 * assign14730_e9280);
        let assign14730_e9282: f64 = (assign14730_e9276 - assign14730_e9281);
        (assign14730_e9282, ((var_t4_dn0 * assign14730_e9275) - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), ((var_t4_dn2 * assign14730_e9275) - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), ((var_t4_dn4 * assign14730_e9275) - (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), ((var_t4_dn5 * assign14730_e9275) - (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), ((var_t4_dn6 * assign14730_e9275) - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), ((var_t4_dn7 * assign14730_e9275) - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), ((var_t4_dn8 * assign14730_e9275) - (0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), ((var_t4_dn9 * assign14730_e9275) - (0.5 * (var_tmf1_dn9 + var_tmf2_dn9))), ((var_t4_dn10 * assign14730_e9275) - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), ((var_t4_dn11 * assign14730_e9275) - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), ((var_t4_dn14 * assign14730_e9275) - (0.5 * (var_tmf1_dn14 + var_tmf2_dn14))),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn11, var_t7_dn14,)
    }
};
        var_t7 = assign14730_e9284;
        var_t7_dn0 = assign14730_e9284_d_n0;
        var_t7_dn2 = assign14730_e9284_d_n2;
        var_t7_dn4 = assign14730_e9284_d_n4;
        var_t7_dn5 = assign14730_e9284_d_n5;
        var_t7_dn6 = assign14730_e9284_d_n6;
        var_t7_dn7 = assign14730_e9284_d_n7;
        var_t7_dn8 = assign14730_e9284_d_n8;
        var_t7_dn9 = assign14730_e9284_d_n9;
        var_t7_dn10 = assign14730_e9284_d_n10;
        var_t7_dn11 = assign14730_e9284_d_n11;
        var_t7_dn14 = assign14730_e9284_d_n14;

        let (assign14740_e9300, assign14740_e9300_d_n0, assign14740_e9300_d_n2, assign14740_e9300_d_n4, assign14740_e9300_d_n5, assign14740_e9300_d_n6, assign14740_e9300_d_n7, assign14740_e9300_d_n8, assign14740_e9300_d_n9, assign14740_e9300_d_n10, assign14740_e9300_d_n11, assign14740_e9300_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14740_e9293: f64 = (var_t1 * var_t4);
        let assign14740_e9294: f64 = (var_t7 + assign14740_e9293);
        let assign14740_e9296: f64 = assign14740_e9294;
        let assign14740_e9298: f64 = (assign14740_e9296 - 5e-5);
        (assign14740_e9298, (var_t7_dn0 + ((var_t1_dn0 * var_t4) + (var_t1 * var_t4_dn0))), (var_t7_dn2 + ((var_t1_dn2 * var_t4) + (var_t1 * var_t4_dn2))), (var_t7_dn4 + ((var_t1_dn4 * var_t4) + (var_t1 * var_t4_dn4))), (var_t7_dn5 + ((var_t1_dn5 * var_t4) + (var_t1 * var_t4_dn5))), (var_t7_dn6 + ((var_t1_dn6 * var_t4) + (var_t1 * var_t4_dn6))), (var_t7_dn7 + ((var_t1_dn7 * var_t4) + (var_t1 * var_t4_dn7))), (var_t7_dn8 + ((var_t1_dn8 * var_t4) + (var_t1 * var_t4_dn8))), (var_t7_dn9 + ((var_t1_dn9 * var_t4) + (var_t1 * var_t4_dn9))), (var_t7_dn10 + ((var_t1_dn10 * var_t4) + (var_t1 * var_t4_dn10))), (var_t7_dn11 + ((var_t1_dn11 * var_t4) + (var_t1 * var_t4_dn11))), (var_t7_dn14 + ((var_t1_dn14 * var_t4) + (var_t1 * var_t4_dn14))),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign14740_e9300;
        var_tmf1_dn0 = assign14740_e9300_d_n0;
        var_tmf1_dn2 = assign14740_e9300_d_n2;
        var_tmf1_dn4 = assign14740_e9300_d_n4;
        var_tmf1_dn5 = assign14740_e9300_d_n5;
        var_tmf1_dn6 = assign14740_e9300_d_n6;
        var_tmf1_dn7 = assign14740_e9300_d_n7;
        var_tmf1_dn8 = assign14740_e9300_d_n8;
        var_tmf1_dn9 = assign14740_e9300_d_n9;
        var_tmf1_dn10 = assign14740_e9300_d_n10;
        var_tmf1_dn11 = assign14740_e9300_d_n11;
        var_tmf1_dn14 = assign14740_e9300_d_n14;

        let (assign14750_e9312, assign14750_e9312_d_n0, assign14750_e9312_d_n2, assign14750_e9312_d_n4, assign14750_e9312_d_n5, assign14750_e9312_d_n6, assign14750_e9312_d_n7, assign14750_e9312_d_n8, assign14750_e9312_d_n9, assign14750_e9312_d_n10, assign14750_e9312_d_n11, assign14750_e9312_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14750_e9312;
        var_tmf2_dn0 = assign14750_e9312_d_n0;
        var_tmf2_dn2 = assign14750_e9312_d_n2;
        var_tmf2_dn4 = assign14750_e9312_d_n4;
        var_tmf2_dn5 = assign14750_e9312_d_n5;
        var_tmf2_dn6 = assign14750_e9312_d_n6;
        var_tmf2_dn7 = assign14750_e9312_d_n7;
        var_tmf2_dn8 = assign14750_e9312_d_n8;
        var_tmf2_dn9 = assign14750_e9312_d_n9;
        var_tmf2_dn10 = assign14750_e9312_d_n10;
        var_tmf2_dn11 = assign14750_e9312_d_n11;
        var_tmf2_dn14 = assign14750_e9312_d_n14;

        let (assign14760_e9326, assign14760_e9326_d_n0, assign14760_e9326_d_n2, assign14760_e9326_d_n4, assign14760_e9326_d_n5, assign14760_e9326_d_n6, assign14760_e9326_d_n7, assign14760_e9326_d_n8, assign14760_e9326_d_n9, assign14760_e9326_d_n10, assign14760_e9326_d_n11, assign14760_e9326_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let (assign14760_e9324, assign14760_e9324_d_n0, assign14760_e9324_d_n2, assign14760_e9324_d_n4, assign14760_e9324_d_n5, assign14760_e9324_d_n6, assign14760_e9324_d_n7, assign14760_e9324_d_n8, assign14760_e9324_d_n9, assign14760_e9324_d_n10, assign14760_e9324_d_n11, assign14760_e9324_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign14760_e9323: f64 = (-var_tmf2);
                (assign14760_e9323, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign14760_e9324, assign14760_e9324_d_n0, assign14760_e9324_d_n2, assign14760_e9324_d_n4, assign14760_e9324_d_n5, assign14760_e9324_d_n6, assign14760_e9324_d_n7, assign14760_e9324_d_n8, assign14760_e9324_d_n9, assign14760_e9324_d_n10, assign14760_e9324_d_n11, assign14760_e9324_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14760_e9326;
        var_tmf2_dn0 = assign14760_e9326_d_n0;
        var_tmf2_dn2 = assign14760_e9326_d_n2;
        var_tmf2_dn4 = assign14760_e9326_d_n4;
        var_tmf2_dn5 = assign14760_e9326_d_n5;
        var_tmf2_dn6 = assign14760_e9326_d_n6;
        var_tmf2_dn7 = assign14760_e9326_d_n7;
        var_tmf2_dn8 = assign14760_e9326_d_n8;
        var_tmf2_dn9 = assign14760_e9326_d_n9;
        var_tmf2_dn10 = assign14760_e9326_d_n10;
        var_tmf2_dn11 = assign14760_e9326_d_n11;
        var_tmf2_dn14 = assign14760_e9326_d_n14;

        let (assign14770_e9339, assign14770_e9339_d_n0, assign14770_e9339_d_n2, assign14770_e9339_d_n4, assign14770_e9339_d_n5, assign14770_e9339_d_n6, assign14770_e9339_d_n7, assign14770_e9339_d_n8, assign14770_e9339_d_n9, assign14770_e9339_d_n10, assign14770_e9339_d_n11, assign14770_e9339_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14770_e9334: f64 = (var_tmf1 * var_tmf1);
        let assign14770_e9336: f64 = (assign14770_e9334 + var_tmf2);
        let assign14770_e9337: f64 = (assign14770_e9336).sqrt();
        (assign14770_e9337, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14770_e9337)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14770_e9337)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14770_e9337)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14770_e9337)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign14770_e9337)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign14770_e9337)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign14770_e9337)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign14770_e9337)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign14770_e9337)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign14770_e9337)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign14770_e9337)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14770_e9339;
        var_tmf2_dn0 = assign14770_e9339_d_n0;
        var_tmf2_dn2 = assign14770_e9339_d_n2;
        var_tmf2_dn4 = assign14770_e9339_d_n4;
        var_tmf2_dn5 = assign14770_e9339_d_n5;
        var_tmf2_dn6 = assign14770_e9339_d_n6;
        var_tmf2_dn7 = assign14770_e9339_d_n7;
        var_tmf2_dn8 = assign14770_e9339_d_n8;
        var_tmf2_dn9 = assign14770_e9339_d_n9;
        var_tmf2_dn10 = assign14770_e9339_d_n10;
        var_tmf2_dn11 = assign14770_e9339_d_n11;
        var_tmf2_dn14 = assign14770_e9339_d_n14;

        let (assign14780_e9353, assign14780_e9353_d_n0, assign14780_e9353_d_n2, assign14780_e9353_d_n4, assign14780_e9353_d_n5, assign14780_e9353_d_n6, assign14780_e9353_d_n7, assign14780_e9353_d_n8, assign14780_e9353_d_n9, assign14780_e9353_d_n10, assign14780_e9353_d_n11, assign14780_e9353_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14780_e9349: f64 = (var_tmf1 / var_tmf2);
        let assign14780_e9350: f64 = (1.0 + assign14780_e9349);
        let assign14780_e9351: f64 = (0.5 * assign14780_e9350);
        (assign14780_e9351, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn14,)
    }
};
        var_t6 = assign14780_e9353;
        var_t6_dn0 = assign14780_e9353_d_n0;
        var_t6_dn2 = assign14780_e9353_d_n2;
        var_t6_dn4 = assign14780_e9353_d_n4;
        var_t6_dn5 = assign14780_e9353_d_n5;
        var_t6_dn6 = assign14780_e9353_d_n6;
        var_t6_dn7 = assign14780_e9353_d_n7;
        var_t6_dn8 = assign14780_e9353_d_n8;
        var_t6_dn9 = assign14780_e9353_d_n9;
        var_t6_dn10 = assign14780_e9353_d_n10;
        var_t6_dn11 = assign14780_e9353_d_n11;
        var_t6_dn14 = assign14780_e9353_d_n14;

        let (assign14790_e9367, assign14790_e9367_d_n0, assign14790_e9367_d_n2, assign14790_e9367_d_n4, assign14790_e9367_d_n5, assign14790_e9367_d_n6, assign14790_e9367_d_n7, assign14790_e9367_d_n8, assign14790_e9367_d_n9, assign14790_e9367_d_n10, assign14790_e9367_d_n11, assign14790_e9367_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14790_e9363: f64 = (var_tmf1 + var_tmf2);
        let assign14790_e9364: f64 = (0.5 * assign14790_e9363);
        let assign14790_e9365: f64 = assign14790_e9364;
        (assign14790_e9365, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_dn6 + var_tmf2_dn6)), (0.5 * (var_tmf1_dn7 + var_tmf2_dn7)), (0.5 * (var_tmf1_dn8 + var_tmf2_dn8)), (0.5 * (var_tmf1_dn9 + var_tmf2_dn9)), (0.5 * (var_tmf1_dn10 + var_tmf2_dn10)), (0.5 * (var_tmf1_dn11 + var_tmf2_dn11)), (0.5 * (var_tmf1_dn14 + var_tmf2_dn14)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign14790_e9367;
        var_t2_dn0 = assign14790_e9367_d_n0;
        var_t2_dn2 = assign14790_e9367_d_n2;
        var_t2_dn4 = assign14790_e9367_d_n4;
        var_t2_dn5 = assign14790_e9367_d_n5;
        var_t2_dn6 = assign14790_e9367_d_n6;
        var_t2_dn7 = assign14790_e9367_d_n7;
        var_t2_dn8 = assign14790_e9367_d_n8;
        var_t2_dn9 = assign14790_e9367_d_n9;
        var_t2_dn10 = assign14790_e9367_d_n10;
        var_t2_dn11 = assign14790_e9367_d_n11;
        var_t2_dn14 = assign14790_e9367_d_n14;

        let assign14800_e9374: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        var_guard324 = assign14800_e9374;

        let (assign14810_e9394, assign14810_e9394_d_n0, assign14810_e9394_d_n2, assign14810_e9394_d_n4, assign14810_e9394_d_n5, assign14810_e9394_d_n6, assign14810_e9394_d_n7, assign14810_e9394_d_n8, assign14810_e9394_d_n9, assign14810_e9394_d_n10, assign14810_e9394_d_n11, assign14810_e9394_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard324 != 0.0)) {
        let assign14810_e9385: f64 = (var_mks_rdvdtemp1 * var_tdiff0);
        let assign14810_e9386: f64 = (var_uc_rdvd + assign14810_e9385);
        let assign14810_e9389: f64 = (var_mks_rdvdtemp2 * var_tdiff0_2);
        let assign14810_e9390: f64 = (assign14810_e9386 + assign14810_e9389);
        let assign14810_e9392: f64 = (assign14810_e9390 * var_t2);
        (assign14810_e9392, ((((var_mks_rdvdtemp1 * var_tdiff0_dn0) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn0)) * var_t2) + (assign14810_e9390 * var_t2_dn0)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn2) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn2)) * var_t2) + (assign14810_e9390 * var_t2_dn2)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn4) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn4)) * var_t2) + (assign14810_e9390 * var_t2_dn4)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn5) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn5)) * var_t2) + (assign14810_e9390 * var_t2_dn5)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn6) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn6)) * var_t2) + (assign14810_e9390 * var_t2_dn6)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn7) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn7)) * var_t2) + (assign14810_e9390 * var_t2_dn7)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn8) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn8)) * var_t2) + (assign14810_e9390 * var_t2_dn8)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn9) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn9)) * var_t2) + (assign14810_e9390 * var_t2_dn9)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn10) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn10)) * var_t2) + (assign14810_e9390 * var_t2_dn10)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn11) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn11)) * var_t2) + (assign14810_e9390 * var_t2_dn11)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn14) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn14)) * var_t2) + (assign14810_e9390 * var_t2_dn14)),)
    } else {
        (var_rdvde, var_rdvde_dn0, var_rdvde_dn2, var_rdvde_dn4, var_rdvde_dn5, var_rdvde_dn6, var_rdvde_dn7, var_rdvde_dn8, var_rdvde_dn9, var_rdvde_dn10, var_rdvde_dn11, var_rdvde_dn14,)
    }
};
        var_rdvde = assign14810_e9394;
        var_rdvde_dn0 = assign14810_e9394_d_n0;
        var_rdvde_dn2 = assign14810_e9394_d_n2;
        var_rdvde_dn4 = assign14810_e9394_d_n4;
        var_rdvde_dn5 = assign14810_e9394_d_n5;
        var_rdvde_dn6 = assign14810_e9394_d_n6;
        var_rdvde_dn7 = assign14810_e9394_d_n7;
        var_rdvde_dn8 = assign14810_e9394_d_n8;
        var_rdvde_dn9 = assign14810_e9394_d_n9;
        var_rdvde_dn10 = assign14810_e9394_d_n10;
        var_rdvde_dn11 = assign14810_e9394_d_n11;
        var_rdvde_dn14 = assign14810_e9394_d_n14;

        let (assign14820_e9412, assign14820_e9412_d_n0, assign14820_e9412_d_n2, assign14820_e9412_d_n4, assign14820_e9412_d_n5, assign14820_e9412_d_n6, assign14820_e9412_d_n7, assign14820_e9412_d_n8, assign14820_e9412_d_n9, assign14820_e9412_d_n10, assign14820_e9412_d_n11, assign14820_e9412_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard324 != 0.0)) {
        let assign14820_e9405: f64 = (0.005 * var_uc_rdvd);
        let assign14820_e9406: f64 = (var_rdvde - assign14820_e9405);
        let assign14820_e9409: f64 = (0.01 * var_uc_rdvd);
        let assign14820_e9410: f64 = (assign14820_e9406 - assign14820_e9409);
        (assign14820_e9410, var_rdvde_dn0, var_rdvde_dn2, var_rdvde_dn4, var_rdvde_dn5, var_rdvde_dn6, var_rdvde_dn7, var_rdvde_dn8, var_rdvde_dn9, var_rdvde_dn10, var_rdvde_dn11, var_rdvde_dn14,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign14820_e9412;
        var_tmf1_dn0 = assign14820_e9412_d_n0;
        var_tmf1_dn2 = assign14820_e9412_d_n2;
        var_tmf1_dn4 = assign14820_e9412_d_n4;
        var_tmf1_dn5 = assign14820_e9412_d_n5;
        var_tmf1_dn6 = assign14820_e9412_d_n6;
        var_tmf1_dn7 = assign14820_e9412_d_n7;
        var_tmf1_dn8 = assign14820_e9412_d_n8;
        var_tmf1_dn9 = assign14820_e9412_d_n9;
        var_tmf1_dn10 = assign14820_e9412_d_n10;
        var_tmf1_dn11 = assign14820_e9412_d_n11;
        var_tmf1_dn14 = assign14820_e9412_d_n14;

        let (assign14830_e9430, assign14830_e9430_d_n0, assign14830_e9430_d_n2, assign14830_e9430_d_n4, assign14830_e9430_d_n5, assign14830_e9430_d_n6, assign14830_e9430_d_n7, assign14830_e9430_d_n8, assign14830_e9430_d_n9, assign14830_e9430_d_n10, assign14830_e9430_d_n11, assign14830_e9430_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard324 != 0.0)) {
        let assign14830_e9423: f64 = (0.005 * var_uc_rdvd);
        let assign14830_e9424: f64 = (4.0 * assign14830_e9423);
        let assign14830_e9427: f64 = (0.01 * var_uc_rdvd);
        let assign14830_e9428: f64 = (assign14830_e9424 * assign14830_e9427);
        (assign14830_e9428, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14830_e9430;
        var_tmf2_dn0 = assign14830_e9430_d_n0;
        var_tmf2_dn2 = assign14830_e9430_d_n2;
        var_tmf2_dn4 = assign14830_e9430_d_n4;
        var_tmf2_dn5 = assign14830_e9430_d_n5;
        var_tmf2_dn6 = assign14830_e9430_d_n6;
        var_tmf2_dn7 = assign14830_e9430_d_n7;
        var_tmf2_dn8 = assign14830_e9430_d_n8;
        var_tmf2_dn9 = assign14830_e9430_d_n9;
        var_tmf2_dn10 = assign14830_e9430_d_n10;
        var_tmf2_dn11 = assign14830_e9430_d_n11;
        var_tmf2_dn14 = assign14830_e9430_d_n14;

        let (assign14840_e9446, assign14840_e9446_d_n0, assign14840_e9446_d_n2, assign14840_e9446_d_n4, assign14840_e9446_d_n5, assign14840_e9446_d_n6, assign14840_e9446_d_n7, assign14840_e9446_d_n8, assign14840_e9446_d_n9, assign14840_e9446_d_n10, assign14840_e9446_d_n11, assign14840_e9446_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard324 != 0.0)) {
        let (assign14840_e9444, assign14840_e9444_d_n0, assign14840_e9444_d_n2, assign14840_e9444_d_n4, assign14840_e9444_d_n5, assign14840_e9444_d_n6, assign14840_e9444_d_n7, assign14840_e9444_d_n8, assign14840_e9444_d_n9, assign14840_e9444_d_n10, assign14840_e9444_d_n11, assign14840_e9444_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign14840_e9443: f64 = (-var_tmf2);
                (assign14840_e9443, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign14840_e9444, assign14840_e9444_d_n0, assign14840_e9444_d_n2, assign14840_e9444_d_n4, assign14840_e9444_d_n5, assign14840_e9444_d_n6, assign14840_e9444_d_n7, assign14840_e9444_d_n8, assign14840_e9444_d_n9, assign14840_e9444_d_n10, assign14840_e9444_d_n11, assign14840_e9444_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14840_e9446;
        var_tmf2_dn0 = assign14840_e9446_d_n0;
        var_tmf2_dn2 = assign14840_e9446_d_n2;
        var_tmf2_dn4 = assign14840_e9446_d_n4;
        var_tmf2_dn5 = assign14840_e9446_d_n5;
        var_tmf2_dn6 = assign14840_e9446_d_n6;
        var_tmf2_dn7 = assign14840_e9446_d_n7;
        var_tmf2_dn8 = assign14840_e9446_d_n8;
        var_tmf2_dn9 = assign14840_e9446_d_n9;
        var_tmf2_dn10 = assign14840_e9446_d_n10;
        var_tmf2_dn11 = assign14840_e9446_d_n11;
        var_tmf2_dn14 = assign14840_e9446_d_n14;

        *var_guard324_slot = var_guard324;
        *var_rdvde_slot = var_rdvde;
        *var_rdvde_dn0_slot = var_rdvde_dn0;
        *var_rdvde_dn10_slot = var_rdvde_dn10;
        *var_rdvde_dn11_slot = var_rdvde_dn11;
        *var_rdvde_dn14_slot = var_rdvde_dn14;
        *var_rdvde_dn2_slot = var_rdvde_dn2;
        *var_rdvde_dn4_slot = var_rdvde_dn4;
        *var_rdvde_dn5_slot = var_rdvde_dn5;
        *var_rdvde_dn6_slot = var_rdvde_dn6;
        *var_rdvde_dn7_slot = var_rdvde_dn7;
        *var_rdvde_dn8_slot = var_rdvde_dn8;
        *var_rdvde_dn9_slot = var_rdvde_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn14_slot = var_t5_dn14;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_dn9_slot = var_t5_dn9;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn14_slot = var_t6_dn14;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
        *var_t7_slot = var_t7;
        *var_t7_dn0_slot = var_t7_dn0;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn14_slot = var_t7_dn14;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn7_slot = var_t7_dn7;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_dn9_slot = var_t7_dn9;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn14_slot = var_tmf1_dn14;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_dn9_slot = var_tmf1_dn9;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn14_slot = var_tmf2_dn14;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_dn9_slot = var_tmf2_dn9;
    }

    pub(super) fn stamp_transient_block_29(
        p: &Parameters,
        var_guard293: f64,
        var_guard317: f64,
        var_guard322: f64,
        var_guard324: f64,
        var_mks_rdvdtemp1: f64,
        var_mks_rdvdtemp2: f64,
        var_rdvdtemp0: f64,
        var_rdvdtemp0_dn0: f64,
        var_rdvdtemp0_dn10: f64,
        var_rdvdtemp0_dn11: f64,
        var_rdvdtemp0_dn14: f64,
        var_rdvdtemp0_dn2: f64,
        var_rdvdtemp0_dn4: f64,
        var_rdvdtemp0_dn5: f64,
        var_rdvdtemp0_dn6: f64,
        var_rdvdtemp0_dn7: f64,
        var_rdvdtemp0_dn8: f64,
        var_rdvdtemp0_dn9: f64,
        var_t2: f64,
        var_t2_dn0: f64,
        var_t2_dn10: f64,
        var_t2_dn11: f64,
        var_t2_dn14: f64,
        var_t2_dn2: f64,
        var_t2_dn4: f64,
        var_t2_dn5: f64,
        var_t2_dn6: f64,
        var_t2_dn7: f64,
        var_t2_dn8: f64,
        var_t2_dn9: f64,
        var_t8: f64,
        var_t8_dn0: f64,
        var_t8_dn10: f64,
        var_t8_dn11: f64,
        var_t8_dn14: f64,
        var_t8_dn2: f64,
        var_t8_dn4: f64,
        var_t8_dn5: f64,
        var_t8_dn6: f64,
        var_t8_dn7: f64,
        var_t8_dn8: f64,
        var_t8_dn9: f64,
        var_tdiff: f64,
        var_tdiff_2: f64,
        var_tdiff_2_dn0: f64,
        var_tdiff_2_dn10: f64,
        var_tdiff_2_dn11: f64,
        var_tdiff_2_dn14: f64,
        var_tdiff_2_dn2: f64,
        var_tdiff_2_dn4: f64,
        var_tdiff_2_dn5: f64,
        var_tdiff_2_dn6: f64,
        var_tdiff_2_dn7: f64,
        var_tdiff_2_dn8: f64,
        var_tdiff_2_dn9: f64,
        var_tdiff_dn0: f64,
        var_tdiff_dn10: f64,
        var_tdiff_dn11: f64,
        var_tdiff_dn14: f64,
        var_tdiff_dn2: f64,
        var_tdiff_dn4: f64,
        var_tdiff_dn5: f64,
        var_tdiff_dn6: f64,
        var_tdiff_dn7: f64,
        var_tdiff_dn8: f64,
        var_tdiff_dn9: f64,
        var_uc_rdict1: f64,
        var_uc_rdov13: f64,
        var_uc_rdslp1: f64,
        var_uc_rdvd: f64,
        var_rdvde_slot: &mut f64,
        var_rdvde_dn0_slot: &mut f64,
        var_rdvde_dn10_slot: &mut f64,
        var_rdvde_dn11_slot: &mut f64,
        var_rdvde_dn14_slot: &mut f64,
        var_rdvde_dn2_slot: &mut f64,
        var_rdvde_dn4_slot: &mut f64,
        var_rdvde_dn5_slot: &mut f64,
        var_rdvde_dn6_slot: &mut f64,
        var_rdvde_dn7_slot: &mut f64,
        var_rdvde_dn8_slot: &mut f64,
        var_rdvde_dn9_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn14_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn14_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_dn9_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn14_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn14_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_dn9_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn14_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_dn9_slot: &mut f64,
    ) {
        let mut var_rdvde: f64 = *var_rdvde_slot;
        let mut var_rdvde_dn0: f64 = *var_rdvde_dn0_slot;
        let mut var_rdvde_dn10: f64 = *var_rdvde_dn10_slot;
        let mut var_rdvde_dn11: f64 = *var_rdvde_dn11_slot;
        let mut var_rdvde_dn14: f64 = *var_rdvde_dn14_slot;
        let mut var_rdvde_dn2: f64 = *var_rdvde_dn2_slot;
        let mut var_rdvde_dn4: f64 = *var_rdvde_dn4_slot;
        let mut var_rdvde_dn5: f64 = *var_rdvde_dn5_slot;
        let mut var_rdvde_dn6: f64 = *var_rdvde_dn6_slot;
        let mut var_rdvde_dn7: f64 = *var_rdvde_dn7_slot;
        let mut var_rdvde_dn8: f64 = *var_rdvde_dn8_slot;
        let mut var_rdvde_dn9: f64 = *var_rdvde_dn9_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn14: f64 = *var_t4_dn14_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn14: f64 = *var_t5_dn14_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_dn9: f64 = *var_t5_dn9_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn14: f64 = *var_t6_dn14_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn14: f64 = *var_tmf1_dn14_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_dn9: f64 = *var_tmf1_dn9_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn14: f64 = *var_tmf2_dn14_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_dn9: f64 = *var_tmf2_dn9_slot;

        let (assign14850_e9461, assign14850_e9461_d_n0, assign14850_e9461_d_n2, assign14850_e9461_d_n4, assign14850_e9461_d_n5, assign14850_e9461_d_n6, assign14850_e9461_d_n7, assign14850_e9461_d_n8, assign14850_e9461_d_n9, assign14850_e9461_d_n10, assign14850_e9461_d_n11, assign14850_e9461_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard324 != 0.0)) {
        let assign14850_e9456: f64 = (var_tmf1 * var_tmf1);
        let assign14850_e9458: f64 = (assign14850_e9456 + var_tmf2);
        let assign14850_e9459: f64 = (assign14850_e9458).sqrt();
        (assign14850_e9459, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14850_e9459)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14850_e9459)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14850_e9459)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14850_e9459)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign14850_e9459)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign14850_e9459)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign14850_e9459)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign14850_e9459)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign14850_e9459)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign14850_e9459)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign14850_e9459)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14850_e9461;
        var_tmf2_dn0 = assign14850_e9461_d_n0;
        var_tmf2_dn2 = assign14850_e9461_d_n2;
        var_tmf2_dn4 = assign14850_e9461_d_n4;
        var_tmf2_dn5 = assign14850_e9461_d_n5;
        var_tmf2_dn6 = assign14850_e9461_d_n6;
        var_tmf2_dn7 = assign14850_e9461_d_n7;
        var_tmf2_dn8 = assign14850_e9461_d_n8;
        var_tmf2_dn9 = assign14850_e9461_d_n9;
        var_tmf2_dn10 = assign14850_e9461_d_n10;
        var_tmf2_dn11 = assign14850_e9461_d_n11;
        var_tmf2_dn14 = assign14850_e9461_d_n14;

        let (assign14860_e9477, assign14860_e9477_d_n0, assign14860_e9477_d_n2, assign14860_e9477_d_n4, assign14860_e9477_d_n5, assign14860_e9477_d_n6, assign14860_e9477_d_n7, assign14860_e9477_d_n8, assign14860_e9477_d_n9, assign14860_e9477_d_n10, assign14860_e9477_d_n11, assign14860_e9477_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard324 != 0.0)) {
        let assign14860_e9473: f64 = (var_tmf1 / var_tmf2);
        let assign14860_e9474: f64 = (1.0 + assign14860_e9473);
        let assign14860_e9475: f64 = (0.5 * assign14860_e9474);
        (assign14860_e9475, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign14860_e9477;
        var_t0_dn0 = assign14860_e9477_d_n0;
        var_t0_dn2 = assign14860_e9477_d_n2;
        var_t0_dn4 = assign14860_e9477_d_n4;
        var_t0_dn5 = assign14860_e9477_d_n5;
        var_t0_dn6 = assign14860_e9477_d_n6;
        var_t0_dn7 = assign14860_e9477_d_n7;
        var_t0_dn8 = assign14860_e9477_d_n8;
        var_t0_dn9 = assign14860_e9477_d_n9;
        var_t0_dn10 = assign14860_e9477_d_n10;
        var_t0_dn11 = assign14860_e9477_d_n11;
        var_t0_dn14 = assign14860_e9477_d_n14;

        let (assign14870_e9495, assign14870_e9495_d_n0, assign14870_e9495_d_n2, assign14870_e9495_d_n4, assign14870_e9495_d_n5, assign14870_e9495_d_n6, assign14870_e9495_d_n7, assign14870_e9495_d_n8, assign14870_e9495_d_n9, assign14870_e9495_d_n10, assign14870_e9495_d_n11, assign14870_e9495_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard324 != 0.0)) {
        let assign14870_e9487: f64 = (0.005 * var_uc_rdvd);
        let assign14870_e9491: f64 = (var_tmf1 + var_tmf2);
        let assign14870_e9492: f64 = (0.5 * assign14870_e9491);
        let assign14870_e9493: f64 = (assign14870_e9487 + assign14870_e9492);
        (assign14870_e9493, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_dn6 + var_tmf2_dn6)), (0.5 * (var_tmf1_dn7 + var_tmf2_dn7)), (0.5 * (var_tmf1_dn8 + var_tmf2_dn8)), (0.5 * (var_tmf1_dn9 + var_tmf2_dn9)), (0.5 * (var_tmf1_dn10 + var_tmf2_dn10)), (0.5 * (var_tmf1_dn11 + var_tmf2_dn11)), (0.5 * (var_tmf1_dn14 + var_tmf2_dn14)),)
    } else {
        (var_rdvde, var_rdvde_dn0, var_rdvde_dn2, var_rdvde_dn4, var_rdvde_dn5, var_rdvde_dn6, var_rdvde_dn7, var_rdvde_dn8, var_rdvde_dn9, var_rdvde_dn10, var_rdvde_dn11, var_rdvde_dn14,)
    }
};
        var_rdvde = assign14870_e9495;
        var_rdvde_dn0 = assign14870_e9495_d_n0;
        var_rdvde_dn2 = assign14870_e9495_d_n2;
        var_rdvde_dn4 = assign14870_e9495_d_n4;
        var_rdvde_dn5 = assign14870_e9495_d_n5;
        var_rdvde_dn6 = assign14870_e9495_d_n6;
        var_rdvde_dn7 = assign14870_e9495_d_n7;
        var_rdvde_dn8 = assign14870_e9495_d_n8;
        var_rdvde_dn9 = assign14870_e9495_d_n9;
        var_rdvde_dn10 = assign14870_e9495_d_n10;
        var_rdvde_dn11 = assign14870_e9495_d_n11;
        var_rdvde_dn14 = assign14870_e9495_d_n14;

        let (assign14880_e9516, assign14880_e9516_d_n0, assign14880_e9516_d_n2, assign14880_e9516_d_n4, assign14880_e9516_d_n5, assign14880_e9516_d_n6, assign14880_e9516_d_n7, assign14880_e9516_d_n8, assign14880_e9516_d_n9, assign14880_e9516_d_n10, assign14880_e9516_d_n11, assign14880_e9516_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard324 == 0.0)) {
        let assign14880_e9507: f64 = (var_mks_rdvdtemp1 * var_tdiff);
        let assign14880_e9508: f64 = (var_uc_rdvd + assign14880_e9507);
        let assign14880_e9511: f64 = (var_mks_rdvdtemp2 * var_tdiff_2);
        let assign14880_e9512: f64 = (assign14880_e9508 + assign14880_e9511);
        let assign14880_e9514: f64 = (assign14880_e9512 * var_t2);
        (assign14880_e9514, ((((var_mks_rdvdtemp1 * var_tdiff_dn0) + (var_mks_rdvdtemp2 * var_tdiff_2_dn0)) * var_t2) + (assign14880_e9512 * var_t2_dn0)), ((((var_mks_rdvdtemp1 * var_tdiff_dn2) + (var_mks_rdvdtemp2 * var_tdiff_2_dn2)) * var_t2) + (assign14880_e9512 * var_t2_dn2)), ((((var_mks_rdvdtemp1 * var_tdiff_dn4) + (var_mks_rdvdtemp2 * var_tdiff_2_dn4)) * var_t2) + (assign14880_e9512 * var_t2_dn4)), ((((var_mks_rdvdtemp1 * var_tdiff_dn5) + (var_mks_rdvdtemp2 * var_tdiff_2_dn5)) * var_t2) + (assign14880_e9512 * var_t2_dn5)), ((((var_mks_rdvdtemp1 * var_tdiff_dn6) + (var_mks_rdvdtemp2 * var_tdiff_2_dn6)) * var_t2) + (assign14880_e9512 * var_t2_dn6)), ((((var_mks_rdvdtemp1 * var_tdiff_dn7) + (var_mks_rdvdtemp2 * var_tdiff_2_dn7)) * var_t2) + (assign14880_e9512 * var_t2_dn7)), ((((var_mks_rdvdtemp1 * var_tdiff_dn8) + (var_mks_rdvdtemp2 * var_tdiff_2_dn8)) * var_t2) + (assign14880_e9512 * var_t2_dn8)), ((((var_mks_rdvdtemp1 * var_tdiff_dn9) + (var_mks_rdvdtemp2 * var_tdiff_2_dn9)) * var_t2) + (assign14880_e9512 * var_t2_dn9)), ((((var_mks_rdvdtemp1 * var_tdiff_dn10) + (var_mks_rdvdtemp2 * var_tdiff_2_dn10)) * var_t2) + (assign14880_e9512 * var_t2_dn10)), ((((var_mks_rdvdtemp1 * var_tdiff_dn11) + (var_mks_rdvdtemp2 * var_tdiff_2_dn11)) * var_t2) + (assign14880_e9512 * var_t2_dn11)), ((((var_mks_rdvdtemp1 * var_tdiff_dn14) + (var_mks_rdvdtemp2 * var_tdiff_2_dn14)) * var_t2) + (assign14880_e9512 * var_t2_dn14)),)
    } else {
        (var_rdvde, var_rdvde_dn0, var_rdvde_dn2, var_rdvde_dn4, var_rdvde_dn5, var_rdvde_dn6, var_rdvde_dn7, var_rdvde_dn8, var_rdvde_dn9, var_rdvde_dn10, var_rdvde_dn11, var_rdvde_dn14,)
    }
};
        var_rdvde = assign14880_e9516;
        var_rdvde_dn0 = assign14880_e9516_d_n0;
        var_rdvde_dn2 = assign14880_e9516_d_n2;
        var_rdvde_dn4 = assign14880_e9516_d_n4;
        var_rdvde_dn5 = assign14880_e9516_d_n5;
        var_rdvde_dn6 = assign14880_e9516_d_n6;
        var_rdvde_dn7 = assign14880_e9516_d_n7;
        var_rdvde_dn8 = assign14880_e9516_d_n8;
        var_rdvde_dn9 = assign14880_e9516_d_n9;
        var_rdvde_dn10 = assign14880_e9516_d_n10;
        var_rdvde_dn11 = assign14880_e9516_d_n11;
        var_rdvde_dn14 = assign14880_e9516_d_n14;

        let (assign14890_e9535, assign14890_e9535_d_n0, assign14890_e9535_d_n2, assign14890_e9535_d_n4, assign14890_e9535_d_n5, assign14890_e9535_d_n6, assign14890_e9535_d_n7, assign14890_e9535_d_n8, assign14890_e9535_d_n9, assign14890_e9535_d_n10, assign14890_e9535_d_n11, assign14890_e9535_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard324 == 0.0)) {
        let assign14890_e9528: f64 = (0.005 * var_uc_rdvd);
        let assign14890_e9529: f64 = (var_rdvde - assign14890_e9528);
        let assign14890_e9532: f64 = (0.01 * var_uc_rdvd);
        let assign14890_e9533: f64 = (assign14890_e9529 - assign14890_e9532);
        (assign14890_e9533, var_rdvde_dn0, var_rdvde_dn2, var_rdvde_dn4, var_rdvde_dn5, var_rdvde_dn6, var_rdvde_dn7, var_rdvde_dn8, var_rdvde_dn9, var_rdvde_dn10, var_rdvde_dn11, var_rdvde_dn14,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign14890_e9535;
        var_tmf1_dn0 = assign14890_e9535_d_n0;
        var_tmf1_dn2 = assign14890_e9535_d_n2;
        var_tmf1_dn4 = assign14890_e9535_d_n4;
        var_tmf1_dn5 = assign14890_e9535_d_n5;
        var_tmf1_dn6 = assign14890_e9535_d_n6;
        var_tmf1_dn7 = assign14890_e9535_d_n7;
        var_tmf1_dn8 = assign14890_e9535_d_n8;
        var_tmf1_dn9 = assign14890_e9535_d_n9;
        var_tmf1_dn10 = assign14890_e9535_d_n10;
        var_tmf1_dn11 = assign14890_e9535_d_n11;
        var_tmf1_dn14 = assign14890_e9535_d_n14;

        let (assign14900_e9554, assign14900_e9554_d_n0, assign14900_e9554_d_n2, assign14900_e9554_d_n4, assign14900_e9554_d_n5, assign14900_e9554_d_n6, assign14900_e9554_d_n7, assign14900_e9554_d_n8, assign14900_e9554_d_n9, assign14900_e9554_d_n10, assign14900_e9554_d_n11, assign14900_e9554_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard324 == 0.0)) {
        let assign14900_e9547: f64 = (0.005 * var_uc_rdvd);
        let assign14900_e9548: f64 = (4.0 * assign14900_e9547);
        let assign14900_e9551: f64 = (0.01 * var_uc_rdvd);
        let assign14900_e9552: f64 = (assign14900_e9548 * assign14900_e9551);
        (assign14900_e9552, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14900_e9554;
        var_tmf2_dn0 = assign14900_e9554_d_n0;
        var_tmf2_dn2 = assign14900_e9554_d_n2;
        var_tmf2_dn4 = assign14900_e9554_d_n4;
        var_tmf2_dn5 = assign14900_e9554_d_n5;
        var_tmf2_dn6 = assign14900_e9554_d_n6;
        var_tmf2_dn7 = assign14900_e9554_d_n7;
        var_tmf2_dn8 = assign14900_e9554_d_n8;
        var_tmf2_dn9 = assign14900_e9554_d_n9;
        var_tmf2_dn10 = assign14900_e9554_d_n10;
        var_tmf2_dn11 = assign14900_e9554_d_n11;
        var_tmf2_dn14 = assign14900_e9554_d_n14;

        let (assign14910_e9571, assign14910_e9571_d_n0, assign14910_e9571_d_n2, assign14910_e9571_d_n4, assign14910_e9571_d_n5, assign14910_e9571_d_n6, assign14910_e9571_d_n7, assign14910_e9571_d_n8, assign14910_e9571_d_n9, assign14910_e9571_d_n10, assign14910_e9571_d_n11, assign14910_e9571_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard324 == 0.0)) {
        let (assign14910_e9569, assign14910_e9569_d_n0, assign14910_e9569_d_n2, assign14910_e9569_d_n4, assign14910_e9569_d_n5, assign14910_e9569_d_n6, assign14910_e9569_d_n7, assign14910_e9569_d_n8, assign14910_e9569_d_n9, assign14910_e9569_d_n10, assign14910_e9569_d_n11, assign14910_e9569_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign14910_e9568: f64 = (-var_tmf2);
                (assign14910_e9568, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign14910_e9569, assign14910_e9569_d_n0, assign14910_e9569_d_n2, assign14910_e9569_d_n4, assign14910_e9569_d_n5, assign14910_e9569_d_n6, assign14910_e9569_d_n7, assign14910_e9569_d_n8, assign14910_e9569_d_n9, assign14910_e9569_d_n10, assign14910_e9569_d_n11, assign14910_e9569_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14910_e9571;
        var_tmf2_dn0 = assign14910_e9571_d_n0;
        var_tmf2_dn2 = assign14910_e9571_d_n2;
        var_tmf2_dn4 = assign14910_e9571_d_n4;
        var_tmf2_dn5 = assign14910_e9571_d_n5;
        var_tmf2_dn6 = assign14910_e9571_d_n6;
        var_tmf2_dn7 = assign14910_e9571_d_n7;
        var_tmf2_dn8 = assign14910_e9571_d_n8;
        var_tmf2_dn9 = assign14910_e9571_d_n9;
        var_tmf2_dn10 = assign14910_e9571_d_n10;
        var_tmf2_dn11 = assign14910_e9571_d_n11;
        var_tmf2_dn14 = assign14910_e9571_d_n14;

        let (assign14920_e9587, assign14920_e9587_d_n0, assign14920_e9587_d_n2, assign14920_e9587_d_n4, assign14920_e9587_d_n5, assign14920_e9587_d_n6, assign14920_e9587_d_n7, assign14920_e9587_d_n8, assign14920_e9587_d_n9, assign14920_e9587_d_n10, assign14920_e9587_d_n11, assign14920_e9587_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard324 == 0.0)) {
        let assign14920_e9582: f64 = (var_tmf1 * var_tmf1);
        let assign14920_e9584: f64 = (assign14920_e9582 + var_tmf2);
        let assign14920_e9585: f64 = (assign14920_e9584).sqrt();
        (assign14920_e9585, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign14920_e9585)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign14920_e9585)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign14920_e9585)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign14920_e9585)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign14920_e9585)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign14920_e9585)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign14920_e9585)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign14920_e9585)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign14920_e9585)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign14920_e9585)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign14920_e9585)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14920_e9587;
        var_tmf2_dn0 = assign14920_e9587_d_n0;
        var_tmf2_dn2 = assign14920_e9587_d_n2;
        var_tmf2_dn4 = assign14920_e9587_d_n4;
        var_tmf2_dn5 = assign14920_e9587_d_n5;
        var_tmf2_dn6 = assign14920_e9587_d_n6;
        var_tmf2_dn7 = assign14920_e9587_d_n7;
        var_tmf2_dn8 = assign14920_e9587_d_n8;
        var_tmf2_dn9 = assign14920_e9587_d_n9;
        var_tmf2_dn10 = assign14920_e9587_d_n10;
        var_tmf2_dn11 = assign14920_e9587_d_n11;
        var_tmf2_dn14 = assign14920_e9587_d_n14;

        let (assign14930_e9604, assign14930_e9604_d_n0, assign14930_e9604_d_n2, assign14930_e9604_d_n4, assign14930_e9604_d_n5, assign14930_e9604_d_n6, assign14930_e9604_d_n7, assign14930_e9604_d_n8, assign14930_e9604_d_n9, assign14930_e9604_d_n10, assign14930_e9604_d_n11, assign14930_e9604_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard324 == 0.0)) {
        let assign14930_e9600: f64 = (var_tmf1 / var_tmf2);
        let assign14930_e9601: f64 = (1.0 + assign14930_e9600);
        let assign14930_e9602: f64 = (0.5 * assign14930_e9601);
        (assign14930_e9602, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign14930_e9604;
        var_t0_dn0 = assign14930_e9604_d_n0;
        var_t0_dn2 = assign14930_e9604_d_n2;
        var_t0_dn4 = assign14930_e9604_d_n4;
        var_t0_dn5 = assign14930_e9604_d_n5;
        var_t0_dn6 = assign14930_e9604_d_n6;
        var_t0_dn7 = assign14930_e9604_d_n7;
        var_t0_dn8 = assign14930_e9604_d_n8;
        var_t0_dn9 = assign14930_e9604_d_n9;
        var_t0_dn10 = assign14930_e9604_d_n10;
        var_t0_dn11 = assign14930_e9604_d_n11;
        var_t0_dn14 = assign14930_e9604_d_n14;

        let (assign14940_e9623, assign14940_e9623_d_n0, assign14940_e9623_d_n2, assign14940_e9623_d_n4, assign14940_e9623_d_n5, assign14940_e9623_d_n6, assign14940_e9623_d_n7, assign14940_e9623_d_n8, assign14940_e9623_d_n9, assign14940_e9623_d_n10, assign14940_e9623_d_n11, assign14940_e9623_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard324 == 0.0)) {
        let assign14940_e9615: f64 = (0.005 * var_uc_rdvd);
        let assign14940_e9619: f64 = (var_tmf1 + var_tmf2);
        let assign14940_e9620: f64 = (0.5 * assign14940_e9619);
        let assign14940_e9621: f64 = (assign14940_e9615 + assign14940_e9620);
        (assign14940_e9621, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_dn6 + var_tmf2_dn6)), (0.5 * (var_tmf1_dn7 + var_tmf2_dn7)), (0.5 * (var_tmf1_dn8 + var_tmf2_dn8)), (0.5 * (var_tmf1_dn9 + var_tmf2_dn9)), (0.5 * (var_tmf1_dn10 + var_tmf2_dn10)), (0.5 * (var_tmf1_dn11 + var_tmf2_dn11)), (0.5 * (var_tmf1_dn14 + var_tmf2_dn14)),)
    } else {
        (var_rdvde, var_rdvde_dn0, var_rdvde_dn2, var_rdvde_dn4, var_rdvde_dn5, var_rdvde_dn6, var_rdvde_dn7, var_rdvde_dn8, var_rdvde_dn9, var_rdvde_dn10, var_rdvde_dn11, var_rdvde_dn14,)
    }
};
        var_rdvde = assign14940_e9623;
        var_rdvde_dn0 = assign14940_e9623_d_n0;
        var_rdvde_dn2 = assign14940_e9623_d_n2;
        var_rdvde_dn4 = assign14940_e9623_d_n4;
        var_rdvde_dn5 = assign14940_e9623_d_n5;
        var_rdvde_dn6 = assign14940_e9623_d_n6;
        var_rdvde_dn7 = assign14940_e9623_d_n7;
        var_rdvde_dn8 = assign14940_e9623_d_n8;
        var_rdvde_dn9 = assign14940_e9623_d_n9;
        var_rdvde_dn10 = assign14940_e9623_d_n10;
        var_rdvde_dn11 = assign14940_e9623_d_n11;
        var_rdvde_dn14 = assign14940_e9623_d_n14;

        let (assign14950_e9647, assign14950_e9647_d_n0, assign14950_e9647_d_n2, assign14950_e9647_d_n4, assign14950_e9647_d_n5, assign14950_e9647_d_n6, assign14950_e9647_d_n7, assign14950_e9647_d_n8, assign14950_e9647_d_n9, assign14950_e9647_d_n10, assign14950_e9647_d_n11, assign14950_e9647_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14950_e9632: f64 = (p.p69 * var_uc_rdslp1);
        let assign14950_e9634: f64 = (assign14950_e9632 * 1000000.0);
        let assign14950_e9636: f64 = (assign14950_e9634 + var_uc_rdict1);
        let assign14950_e9637: f64 = (var_rdvdtemp0 * assign14950_e9636);
        let assign14950_e9640: f64 = (p.p70 * p.p100);
        let assign14950_e9642: f64 = (assign14950_e9640 * 1000000.0);
        let assign14950_e9644: f64 = (assign14950_e9642 + p.p101);
        let assign14950_e9645: f64 = (assign14950_e9637 * assign14950_e9644);
        (assign14950_e9645, ((var_rdvdtemp0_dn0 * assign14950_e9636) * assign14950_e9644), ((var_rdvdtemp0_dn2 * assign14950_e9636) * assign14950_e9644), ((var_rdvdtemp0_dn4 * assign14950_e9636) * assign14950_e9644), ((var_rdvdtemp0_dn5 * assign14950_e9636) * assign14950_e9644), ((var_rdvdtemp0_dn6 * assign14950_e9636) * assign14950_e9644), ((var_rdvdtemp0_dn7 * assign14950_e9636) * assign14950_e9644), ((var_rdvdtemp0_dn8 * assign14950_e9636) * assign14950_e9644), ((var_rdvdtemp0_dn9 * assign14950_e9636) * assign14950_e9644), ((var_rdvdtemp0_dn10 * assign14950_e9636) * assign14950_e9644), ((var_rdvdtemp0_dn11 * assign14950_e9636) * assign14950_e9644), ((var_rdvdtemp0_dn14 * assign14950_e9636) * assign14950_e9644),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11, var_t4_dn14,)
    }
};
        var_t4 = assign14950_e9647;
        var_t4_dn0 = assign14950_e9647_d_n0;
        var_t4_dn2 = assign14950_e9647_d_n2;
        var_t4_dn4 = assign14950_e9647_d_n4;
        var_t4_dn5 = assign14950_e9647_d_n5;
        var_t4_dn6 = assign14950_e9647_d_n6;
        var_t4_dn7 = assign14950_e9647_d_n7;
        var_t4_dn8 = assign14950_e9647_d_n8;
        var_t4_dn9 = assign14950_e9647_d_n9;
        var_t4_dn10 = assign14950_e9647_d_n10;
        var_t4_dn11 = assign14950_e9647_d_n11;
        var_t4_dn14 = assign14950_e9647_d_n14;

        let (assign14960_e9661, assign14960_e9661_d_n0, assign14960_e9661_d_n2, assign14960_e9661_d_n4, assign14960_e9661_d_n5, assign14960_e9661_d_n6, assign14960_e9661_d_n7, assign14960_e9661_d_n8, assign14960_e9661_d_n9, assign14960_e9661_d_n10, assign14960_e9661_d_n11, assign14960_e9661_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14960_e9655: f64 = (1.0 - var_uc_rdov13);
        let assign14960_e9657: f64 = (assign14960_e9655 * p.p66);
        let assign14960_e9659: f64 = (assign14960_e9657 * 1000000.0);
        (assign14960_e9659, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign14960_e9661;
        var_t1_dn0 = assign14960_e9661_d_n0;
        var_t1_dn2 = assign14960_e9661_d_n2;
        var_t1_dn4 = assign14960_e9661_d_n4;
        var_t1_dn5 = assign14960_e9661_d_n5;
        var_t1_dn6 = assign14960_e9661_d_n6;
        var_t1_dn7 = assign14960_e9661_d_n7;
        var_t1_dn8 = assign14960_e9661_d_n8;
        var_t1_dn9 = assign14960_e9661_d_n9;
        var_t1_dn10 = assign14960_e9661_d_n10;
        var_t1_dn11 = assign14960_e9661_d_n11;
        var_t1_dn14 = assign14960_e9661_d_n14;

        let (assign14970_e9677, assign14970_e9677_d_n0, assign14970_e9677_d_n2, assign14970_e9677_d_n4, assign14970_e9677_d_n5, assign14970_e9677_d_n6, assign14970_e9677_d_n7, assign14970_e9677_d_n8, assign14970_e9677_d_n9, assign14970_e9677_d_n10, assign14970_e9677_d_n11, assign14970_e9677_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14970_e9669: f64 = (var_t8 * p.p66);
        let assign14970_e9671: f64 = (assign14970_e9669 * 1000000.0);
        let assign14970_e9673: f64 = (assign14970_e9671 + 1.0);
        let assign14970_e9675: f64 = (assign14970_e9673 + p.p98);
        (assign14970_e9675, ((var_t8_dn0 * p.p66) * 1000000.0), ((var_t8_dn2 * p.p66) * 1000000.0), ((var_t8_dn4 * p.p66) * 1000000.0), ((var_t8_dn5 * p.p66) * 1000000.0), ((var_t8_dn6 * p.p66) * 1000000.0), ((var_t8_dn7 * p.p66) * 1000000.0), ((var_t8_dn8 * p.p66) * 1000000.0), ((var_t8_dn9 * p.p66) * 1000000.0), ((var_t8_dn10 * p.p66) * 1000000.0), ((var_t8_dn11 * p.p66) * 1000000.0), ((var_t8_dn14 * p.p66) * 1000000.0),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign14970_e9677;
        var_t3_dn0 = assign14970_e9677_d_n0;
        var_t3_dn2 = assign14970_e9677_d_n2;
        var_t3_dn4 = assign14970_e9677_d_n4;
        var_t3_dn5 = assign14970_e9677_d_n5;
        var_t3_dn6 = assign14970_e9677_d_n6;
        var_t3_dn7 = assign14970_e9677_d_n7;
        var_t3_dn8 = assign14970_e9677_d_n8;
        var_t3_dn9 = assign14970_e9677_d_n9;
        var_t3_dn10 = assign14970_e9677_d_n10;
        var_t3_dn11 = assign14970_e9677_d_n11;
        var_t3_dn14 = assign14970_e9677_d_n14;

        let (assign14980_e9691, assign14980_e9691_d_n0, assign14980_e9691_d_n2, assign14980_e9691_d_n4, assign14980_e9691_d_n5, assign14980_e9691_d_n6, assign14980_e9691_d_n7, assign14980_e9691_d_n8, assign14980_e9691_d_n9, assign14980_e9691_d_n10, assign14980_e9691_d_n11, assign14980_e9691_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14980_e9685: f64 = (var_t3 * var_t4);
        let assign14980_e9687: f64 = (assign14980_e9685 - var_t4);
        let assign14980_e9689: f64 = (assign14980_e9687 - 0.01);
        (assign14980_e9689, (((var_t3_dn0 * var_t4) + (var_t3 * var_t4_dn0)) - var_t4_dn0), (((var_t3_dn2 * var_t4) + (var_t3 * var_t4_dn2)) - var_t4_dn2), (((var_t3_dn4 * var_t4) + (var_t3 * var_t4_dn4)) - var_t4_dn4), (((var_t3_dn5 * var_t4) + (var_t3 * var_t4_dn5)) - var_t4_dn5), (((var_t3_dn6 * var_t4) + (var_t3 * var_t4_dn6)) - var_t4_dn6), (((var_t3_dn7 * var_t4) + (var_t3 * var_t4_dn7)) - var_t4_dn7), (((var_t3_dn8 * var_t4) + (var_t3 * var_t4_dn8)) - var_t4_dn8), (((var_t3_dn9 * var_t4) + (var_t3 * var_t4_dn9)) - var_t4_dn9), (((var_t3_dn10 * var_t4) + (var_t3 * var_t4_dn10)) - var_t4_dn10), (((var_t3_dn11 * var_t4) + (var_t3 * var_t4_dn11)) - var_t4_dn11), (((var_t3_dn14 * var_t4) + (var_t3 * var_t4_dn14)) - var_t4_dn14),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign14980_e9691;
        var_tmf1_dn0 = assign14980_e9691_d_n0;
        var_tmf1_dn2 = assign14980_e9691_d_n2;
        var_tmf1_dn4 = assign14980_e9691_d_n4;
        var_tmf1_dn5 = assign14980_e9691_d_n5;
        var_tmf1_dn6 = assign14980_e9691_d_n6;
        var_tmf1_dn7 = assign14980_e9691_d_n7;
        var_tmf1_dn8 = assign14980_e9691_d_n8;
        var_tmf1_dn9 = assign14980_e9691_d_n9;
        var_tmf1_dn10 = assign14980_e9691_d_n10;
        var_tmf1_dn11 = assign14980_e9691_d_n11;
        var_tmf1_dn14 = assign14980_e9691_d_n14;

        let (assign14990_e9703, assign14990_e9703_d_n0, assign14990_e9703_d_n2, assign14990_e9703_d_n4, assign14990_e9703_d_n5, assign14990_e9703_d_n6, assign14990_e9703_d_n7, assign14990_e9703_d_n8, assign14990_e9703_d_n9, assign14990_e9703_d_n10, assign14990_e9703_d_n11, assign14990_e9703_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign14990_e9699: f64 = (4.0 * var_t4);
        let assign14990_e9701: f64 = (assign14990_e9699 * 0.01);
        (assign14990_e9701, ((4.0 * var_t4_dn0) * 0.01), ((4.0 * var_t4_dn2) * 0.01), ((4.0 * var_t4_dn4) * 0.01), ((4.0 * var_t4_dn5) * 0.01), ((4.0 * var_t4_dn6) * 0.01), ((4.0 * var_t4_dn7) * 0.01), ((4.0 * var_t4_dn8) * 0.01), ((4.0 * var_t4_dn9) * 0.01), ((4.0 * var_t4_dn10) * 0.01), ((4.0 * var_t4_dn11) * 0.01), ((4.0 * var_t4_dn14) * 0.01),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign14990_e9703;
        var_tmf2_dn0 = assign14990_e9703_d_n0;
        var_tmf2_dn2 = assign14990_e9703_d_n2;
        var_tmf2_dn4 = assign14990_e9703_d_n4;
        var_tmf2_dn5 = assign14990_e9703_d_n5;
        var_tmf2_dn6 = assign14990_e9703_d_n6;
        var_tmf2_dn7 = assign14990_e9703_d_n7;
        var_tmf2_dn8 = assign14990_e9703_d_n8;
        var_tmf2_dn9 = assign14990_e9703_d_n9;
        var_tmf2_dn10 = assign14990_e9703_d_n10;
        var_tmf2_dn11 = assign14990_e9703_d_n11;
        var_tmf2_dn14 = assign14990_e9703_d_n14;

        let (assign15000_e9717, assign15000_e9717_d_n0, assign15000_e9717_d_n2, assign15000_e9717_d_n4, assign15000_e9717_d_n5, assign15000_e9717_d_n6, assign15000_e9717_d_n7, assign15000_e9717_d_n8, assign15000_e9717_d_n9, assign15000_e9717_d_n10, assign15000_e9717_d_n11, assign15000_e9717_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let (assign15000_e9715, assign15000_e9715_d_n0, assign15000_e9715_d_n2, assign15000_e9715_d_n4, assign15000_e9715_d_n5, assign15000_e9715_d_n6, assign15000_e9715_d_n7, assign15000_e9715_d_n8, assign15000_e9715_d_n9, assign15000_e9715_d_n10, assign15000_e9715_d_n11, assign15000_e9715_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign15000_e9714: f64 = (-var_tmf2);
                (assign15000_e9714, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign15000_e9715, assign15000_e9715_d_n0, assign15000_e9715_d_n2, assign15000_e9715_d_n4, assign15000_e9715_d_n5, assign15000_e9715_d_n6, assign15000_e9715_d_n7, assign15000_e9715_d_n8, assign15000_e9715_d_n9, assign15000_e9715_d_n10, assign15000_e9715_d_n11, assign15000_e9715_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign15000_e9717;
        var_tmf2_dn0 = assign15000_e9717_d_n0;
        var_tmf2_dn2 = assign15000_e9717_d_n2;
        var_tmf2_dn4 = assign15000_e9717_d_n4;
        var_tmf2_dn5 = assign15000_e9717_d_n5;
        var_tmf2_dn6 = assign15000_e9717_d_n6;
        var_tmf2_dn7 = assign15000_e9717_d_n7;
        var_tmf2_dn8 = assign15000_e9717_d_n8;
        var_tmf2_dn9 = assign15000_e9717_d_n9;
        var_tmf2_dn10 = assign15000_e9717_d_n10;
        var_tmf2_dn11 = assign15000_e9717_d_n11;
        var_tmf2_dn14 = assign15000_e9717_d_n14;

        let (assign15010_e9730, assign15010_e9730_d_n0, assign15010_e9730_d_n2, assign15010_e9730_d_n4, assign15010_e9730_d_n5, assign15010_e9730_d_n6, assign15010_e9730_d_n7, assign15010_e9730_d_n8, assign15010_e9730_d_n9, assign15010_e9730_d_n10, assign15010_e9730_d_n11, assign15010_e9730_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign15010_e9725: f64 = (var_tmf1 * var_tmf1);
        let assign15010_e9727: f64 = (assign15010_e9725 + var_tmf2);
        let assign15010_e9728: f64 = (assign15010_e9727).sqrt();
        (assign15010_e9728, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15010_e9728)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15010_e9728)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign15010_e9728)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign15010_e9728)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign15010_e9728)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign15010_e9728)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign15010_e9728)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign15010_e9728)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign15010_e9728)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign15010_e9728)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign15010_e9728)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign15010_e9730;
        var_tmf2_dn0 = assign15010_e9730_d_n0;
        var_tmf2_dn2 = assign15010_e9730_d_n2;
        var_tmf2_dn4 = assign15010_e9730_d_n4;
        var_tmf2_dn5 = assign15010_e9730_d_n5;
        var_tmf2_dn6 = assign15010_e9730_d_n6;
        var_tmf2_dn7 = assign15010_e9730_d_n7;
        var_tmf2_dn8 = assign15010_e9730_d_n8;
        var_tmf2_dn9 = assign15010_e9730_d_n9;
        var_tmf2_dn10 = assign15010_e9730_d_n10;
        var_tmf2_dn11 = assign15010_e9730_d_n11;
        var_tmf2_dn14 = assign15010_e9730_d_n14;

        let (assign15020_e9744, assign15020_e9744_d_n0, assign15020_e9744_d_n2, assign15020_e9744_d_n4, assign15020_e9744_d_n5, assign15020_e9744_d_n6, assign15020_e9744_d_n7, assign15020_e9744_d_n8, assign15020_e9744_d_n9, assign15020_e9744_d_n10, assign15020_e9744_d_n11, assign15020_e9744_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign15020_e9740: f64 = (var_tmf1 / var_tmf2);
        let assign15020_e9741: f64 = (1.0 + assign15020_e9740);
        let assign15020_e9742: f64 = (0.5 * assign15020_e9741);
        (assign15020_e9742, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn14,)
    }
};
        var_t6 = assign15020_e9744;
        var_t6_dn0 = assign15020_e9744_d_n0;
        var_t6_dn2 = assign15020_e9744_d_n2;
        var_t6_dn4 = assign15020_e9744_d_n4;
        var_t6_dn5 = assign15020_e9744_d_n5;
        var_t6_dn6 = assign15020_e9744_d_n6;
        var_t6_dn7 = assign15020_e9744_d_n7;
        var_t6_dn8 = assign15020_e9744_d_n8;
        var_t6_dn9 = assign15020_e9744_d_n9;
        var_t6_dn10 = assign15020_e9744_d_n10;
        var_t6_dn11 = assign15020_e9744_d_n11;
        var_t6_dn14 = assign15020_e9744_d_n14;

        let (assign15030_e9758, assign15030_e9758_d_n0, assign15030_e9758_d_n2, assign15030_e9758_d_n4, assign15030_e9758_d_n5, assign15030_e9758_d_n6, assign15030_e9758_d_n7, assign15030_e9758_d_n8, assign15030_e9758_d_n9, assign15030_e9758_d_n10, assign15030_e9758_d_n11, assign15030_e9758_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign15030_e9754: f64 = (var_tmf1 + var_tmf2);
        let assign15030_e9755: f64 = (0.5 * assign15030_e9754);
        let assign15030_e9756: f64 = (var_t4 + assign15030_e9755);
        (assign15030_e9756, (var_t4_dn0 + (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_t4_dn2 + (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_t4_dn4 + (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_t4_dn5 + (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_t4_dn6 + (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_t4_dn7 + (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (var_t4_dn8 + (0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), (var_t4_dn9 + (0.5 * (var_tmf1_dn9 + var_tmf2_dn9))), (var_t4_dn10 + (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_t4_dn11 + (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), (var_t4_dn14 + (0.5 * (var_tmf1_dn14 + var_tmf2_dn14))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11, var_t5_dn14,)
    }
};
        var_t5 = assign15030_e9758;
        var_t5_dn0 = assign15030_e9758_d_n0;
        var_t5_dn2 = assign15030_e9758_d_n2;
        var_t5_dn4 = assign15030_e9758_d_n4;
        var_t5_dn5 = assign15030_e9758_d_n5;
        var_t5_dn6 = assign15030_e9758_d_n6;
        var_t5_dn7 = assign15030_e9758_d_n7;
        var_t5_dn8 = assign15030_e9758_d_n8;
        var_t5_dn9 = assign15030_e9758_d_n9;
        var_t5_dn10 = assign15030_e9758_d_n10;
        var_t5_dn11 = assign15030_e9758_d_n11;
        var_t5_dn14 = assign15030_e9758_d_n14;

        let (assign15040_e9774, assign15040_e9774_d_n0, assign15040_e9774_d_n2, assign15040_e9774_d_n4, assign15040_e9774_d_n5, assign15040_e9774_d_n6, assign15040_e9774_d_n7, assign15040_e9774_d_n8, assign15040_e9774_d_n9, assign15040_e9774_d_n10, assign15040_e9774_d_n11, assign15040_e9774_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign15040_e9767: f64 = (p.p98 + 1.0);
        let assign15040_e9768: f64 = (var_t4 * assign15040_e9767);
        let assign15040_e9770: f64 = (assign15040_e9768 - var_t5);
        let assign15040_e9772: f64 = (assign15040_e9770 - 5e-5);
        (assign15040_e9772, ((var_t4_dn0 * assign15040_e9767) - var_t5_dn0), ((var_t4_dn2 * assign15040_e9767) - var_t5_dn2), ((var_t4_dn4 * assign15040_e9767) - var_t5_dn4), ((var_t4_dn5 * assign15040_e9767) - var_t5_dn5), ((var_t4_dn6 * assign15040_e9767) - var_t5_dn6), ((var_t4_dn7 * assign15040_e9767) - var_t5_dn7), ((var_t4_dn8 * assign15040_e9767) - var_t5_dn8), ((var_t4_dn9 * assign15040_e9767) - var_t5_dn9), ((var_t4_dn10 * assign15040_e9767) - var_t5_dn10), ((var_t4_dn11 * assign15040_e9767) - var_t5_dn11), ((var_t4_dn14 * assign15040_e9767) - var_t5_dn14),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign15040_e9774;
        var_tmf1_dn0 = assign15040_e9774_d_n0;
        var_tmf1_dn2 = assign15040_e9774_d_n2;
        var_tmf1_dn4 = assign15040_e9774_d_n4;
        var_tmf1_dn5 = assign15040_e9774_d_n5;
        var_tmf1_dn6 = assign15040_e9774_d_n6;
        var_tmf1_dn7 = assign15040_e9774_d_n7;
        var_tmf1_dn8 = assign15040_e9774_d_n8;
        var_tmf1_dn9 = assign15040_e9774_d_n9;
        var_tmf1_dn10 = assign15040_e9774_d_n10;
        var_tmf1_dn11 = assign15040_e9774_d_n11;
        var_tmf1_dn14 = assign15040_e9774_d_n14;

        let (assign15050_e9790, assign15050_e9790_d_n0, assign15050_e9790_d_n2, assign15050_e9790_d_n4, assign15050_e9790_d_n5, assign15050_e9790_d_n6, assign15050_e9790_d_n7, assign15050_e9790_d_n8, assign15050_e9790_d_n9, assign15050_e9790_d_n10, assign15050_e9790_d_n11, assign15050_e9790_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign15050_e9784: f64 = (p.p98 + 1.0);
        let assign15050_e9785: f64 = (var_t4 * assign15050_e9784);
        let assign15050_e9786: f64 = (4.0 * assign15050_e9785);
        let assign15050_e9788: f64 = (assign15050_e9786 * 5e-5);
        (assign15050_e9788, ((4.0 * (var_t4_dn0 * assign15050_e9784)) * 5e-5), ((4.0 * (var_t4_dn2 * assign15050_e9784)) * 5e-5), ((4.0 * (var_t4_dn4 * assign15050_e9784)) * 5e-5), ((4.0 * (var_t4_dn5 * assign15050_e9784)) * 5e-5), ((4.0 * (var_t4_dn6 * assign15050_e9784)) * 5e-5), ((4.0 * (var_t4_dn7 * assign15050_e9784)) * 5e-5), ((4.0 * (var_t4_dn8 * assign15050_e9784)) * 5e-5), ((4.0 * (var_t4_dn9 * assign15050_e9784)) * 5e-5), ((4.0 * (var_t4_dn10 * assign15050_e9784)) * 5e-5), ((4.0 * (var_t4_dn11 * assign15050_e9784)) * 5e-5), ((4.0 * (var_t4_dn14 * assign15050_e9784)) * 5e-5),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign15050_e9790;
        var_tmf2_dn0 = assign15050_e9790_d_n0;
        var_tmf2_dn2 = assign15050_e9790_d_n2;
        var_tmf2_dn4 = assign15050_e9790_d_n4;
        var_tmf2_dn5 = assign15050_e9790_d_n5;
        var_tmf2_dn6 = assign15050_e9790_d_n6;
        var_tmf2_dn7 = assign15050_e9790_d_n7;
        var_tmf2_dn8 = assign15050_e9790_d_n8;
        var_tmf2_dn9 = assign15050_e9790_d_n9;
        var_tmf2_dn10 = assign15050_e9790_d_n10;
        var_tmf2_dn11 = assign15050_e9790_d_n11;
        var_tmf2_dn14 = assign15050_e9790_d_n14;

        *var_rdvde_slot = var_rdvde;
        *var_rdvde_dn0_slot = var_rdvde_dn0;
        *var_rdvde_dn10_slot = var_rdvde_dn10;
        *var_rdvde_dn11_slot = var_rdvde_dn11;
        *var_rdvde_dn14_slot = var_rdvde_dn14;
        *var_rdvde_dn2_slot = var_rdvde_dn2;
        *var_rdvde_dn4_slot = var_rdvde_dn4;
        *var_rdvde_dn5_slot = var_rdvde_dn5;
        *var_rdvde_dn6_slot = var_rdvde_dn6;
        *var_rdvde_dn7_slot = var_rdvde_dn7;
        *var_rdvde_dn8_slot = var_rdvde_dn8;
        *var_rdvde_dn9_slot = var_rdvde_dn9;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn14_slot = var_t4_dn14;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn14_slot = var_t5_dn14;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_dn9_slot = var_t5_dn9;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn14_slot = var_t6_dn14;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn14_slot = var_tmf1_dn14;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_dn9_slot = var_tmf1_dn9;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn14_slot = var_tmf2_dn14;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_dn9_slot = var_tmf2_dn9;
    }

    pub(super) fn stamp_transient_block_30(
        p: &Parameters,
        var_guard293: f64,
        var_guard317: f64,
        var_guard322: f64,
        var_mks_rdvdtemp1: f64,
        var_mks_rdvdtemp2: f64,
        var_t1: f64,
        var_t1_dn0: f64,
        var_t1_dn10: f64,
        var_t1_dn11: f64,
        var_t1_dn14: f64,
        var_t1_dn2: f64,
        var_t1_dn4: f64,
        var_t1_dn5: f64,
        var_t1_dn6: f64,
        var_t1_dn7: f64,
        var_t1_dn8: f64,
        var_t1_dn9: f64,
        var_t4: f64,
        var_t4_dn0: f64,
        var_t4_dn10: f64,
        var_t4_dn11: f64,
        var_t4_dn14: f64,
        var_t4_dn2: f64,
        var_t4_dn4: f64,
        var_t4_dn5: f64,
        var_t4_dn6: f64,
        var_t4_dn7: f64,
        var_t4_dn8: f64,
        var_t4_dn9: f64,
        var_tdiff: f64,
        var_tdiff0: f64,
        var_tdiff0_2: f64,
        var_tdiff0_2_dn0: f64,
        var_tdiff0_2_dn10: f64,
        var_tdiff0_2_dn11: f64,
        var_tdiff0_2_dn14: f64,
        var_tdiff0_2_dn2: f64,
        var_tdiff0_2_dn4: f64,
        var_tdiff0_2_dn5: f64,
        var_tdiff0_2_dn6: f64,
        var_tdiff0_2_dn7: f64,
        var_tdiff0_2_dn8: f64,
        var_tdiff0_2_dn9: f64,
        var_tdiff0_dn0: f64,
        var_tdiff0_dn10: f64,
        var_tdiff0_dn11: f64,
        var_tdiff0_dn14: f64,
        var_tdiff0_dn2: f64,
        var_tdiff0_dn4: f64,
        var_tdiff0_dn5: f64,
        var_tdiff0_dn6: f64,
        var_tdiff0_dn7: f64,
        var_tdiff0_dn8: f64,
        var_tdiff0_dn9: f64,
        var_tdiff_2: f64,
        var_tdiff_2_dn0: f64,
        var_tdiff_2_dn10: f64,
        var_tdiff_2_dn11: f64,
        var_tdiff_2_dn14: f64,
        var_tdiff_2_dn2: f64,
        var_tdiff_2_dn4: f64,
        var_tdiff_2_dn5: f64,
        var_tdiff_2_dn6: f64,
        var_tdiff_2_dn7: f64,
        var_tdiff_2_dn8: f64,
        var_tdiff_2_dn9: f64,
        var_tdiff_dn0: f64,
        var_tdiff_dn10: f64,
        var_tdiff_dn11: f64,
        var_tdiff_dn14: f64,
        var_tdiff_dn2: f64,
        var_tdiff_dn4: f64,
        var_tdiff_dn5: f64,
        var_tdiff_dn6: f64,
        var_tdiff_dn7: f64,
        var_tdiff_dn8: f64,
        var_tdiff_dn9: f64,
        var_uc_rdvd: f64,
        var_guard325_slot: &mut f64,
        var_rsvde_slot: &mut f64,
        var_rsvde_dn0_slot: &mut f64,
        var_rsvde_dn10_slot: &mut f64,
        var_rsvde_dn11_slot: &mut f64,
        var_rsvde_dn14_slot: &mut f64,
        var_rsvde_dn2_slot: &mut f64,
        var_rsvde_dn4_slot: &mut f64,
        var_rsvde_dn5_slot: &mut f64,
        var_rsvde_dn6_slot: &mut f64,
        var_rsvde_dn7_slot: &mut f64,
        var_rsvde_dn8_slot: &mut f64,
        var_rsvde_dn9_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn14_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn0_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn11_slot: &mut f64,
        var_t7_dn14_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn7_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_dn9_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn14_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_dn9_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn14_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_dn9_slot: &mut f64,
    ) {
        let mut var_guard325: f64 = *var_guard325_slot;
        let mut var_rsvde: f64 = *var_rsvde_slot;
        let mut var_rsvde_dn0: f64 = *var_rsvde_dn0_slot;
        let mut var_rsvde_dn10: f64 = *var_rsvde_dn10_slot;
        let mut var_rsvde_dn11: f64 = *var_rsvde_dn11_slot;
        let mut var_rsvde_dn14: f64 = *var_rsvde_dn14_slot;
        let mut var_rsvde_dn2: f64 = *var_rsvde_dn2_slot;
        let mut var_rsvde_dn4: f64 = *var_rsvde_dn4_slot;
        let mut var_rsvde_dn5: f64 = *var_rsvde_dn5_slot;
        let mut var_rsvde_dn6: f64 = *var_rsvde_dn6_slot;
        let mut var_rsvde_dn7: f64 = *var_rsvde_dn7_slot;
        let mut var_rsvde_dn8: f64 = *var_rsvde_dn8_slot;
        let mut var_rsvde_dn9: f64 = *var_rsvde_dn9_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn14: f64 = *var_t6_dn14_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn0: f64 = *var_t7_dn0_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn14: f64 = *var_t7_dn14_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn7: f64 = *var_t7_dn7_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_dn9: f64 = *var_t7_dn9_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn14: f64 = *var_tmf1_dn14_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_dn9: f64 = *var_tmf1_dn9_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn14: f64 = *var_tmf2_dn14_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_dn9: f64 = *var_tmf2_dn9_slot;

        let (assign15060_e9804, assign15060_e9804_d_n0, assign15060_e9804_d_n2, assign15060_e9804_d_n4, assign15060_e9804_d_n5, assign15060_e9804_d_n6, assign15060_e9804_d_n7, assign15060_e9804_d_n8, assign15060_e9804_d_n9, assign15060_e9804_d_n10, assign15060_e9804_d_n11, assign15060_e9804_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let (assign15060_e9802, assign15060_e9802_d_n0, assign15060_e9802_d_n2, assign15060_e9802_d_n4, assign15060_e9802_d_n5, assign15060_e9802_d_n6, assign15060_e9802_d_n7, assign15060_e9802_d_n8, assign15060_e9802_d_n9, assign15060_e9802_d_n10, assign15060_e9802_d_n11, assign15060_e9802_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign15060_e9801: f64 = (-var_tmf2);
                (assign15060_e9801, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign15060_e9802, assign15060_e9802_d_n0, assign15060_e9802_d_n2, assign15060_e9802_d_n4, assign15060_e9802_d_n5, assign15060_e9802_d_n6, assign15060_e9802_d_n7, assign15060_e9802_d_n8, assign15060_e9802_d_n9, assign15060_e9802_d_n10, assign15060_e9802_d_n11, assign15060_e9802_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign15060_e9804;
        var_tmf2_dn0 = assign15060_e9804_d_n0;
        var_tmf2_dn2 = assign15060_e9804_d_n2;
        var_tmf2_dn4 = assign15060_e9804_d_n4;
        var_tmf2_dn5 = assign15060_e9804_d_n5;
        var_tmf2_dn6 = assign15060_e9804_d_n6;
        var_tmf2_dn7 = assign15060_e9804_d_n7;
        var_tmf2_dn8 = assign15060_e9804_d_n8;
        var_tmf2_dn9 = assign15060_e9804_d_n9;
        var_tmf2_dn10 = assign15060_e9804_d_n10;
        var_tmf2_dn11 = assign15060_e9804_d_n11;
        var_tmf2_dn14 = assign15060_e9804_d_n14;

        let (assign15070_e9817, assign15070_e9817_d_n0, assign15070_e9817_d_n2, assign15070_e9817_d_n4, assign15070_e9817_d_n5, assign15070_e9817_d_n6, assign15070_e9817_d_n7, assign15070_e9817_d_n8, assign15070_e9817_d_n9, assign15070_e9817_d_n10, assign15070_e9817_d_n11, assign15070_e9817_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign15070_e9812: f64 = (var_tmf1 * var_tmf1);
        let assign15070_e9814: f64 = (assign15070_e9812 + var_tmf2);
        let assign15070_e9815: f64 = (assign15070_e9814).sqrt();
        (assign15070_e9815, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15070_e9815)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15070_e9815)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign15070_e9815)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign15070_e9815)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign15070_e9815)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign15070_e9815)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign15070_e9815)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign15070_e9815)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign15070_e9815)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign15070_e9815)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign15070_e9815)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign15070_e9817;
        var_tmf2_dn0 = assign15070_e9817_d_n0;
        var_tmf2_dn2 = assign15070_e9817_d_n2;
        var_tmf2_dn4 = assign15070_e9817_d_n4;
        var_tmf2_dn5 = assign15070_e9817_d_n5;
        var_tmf2_dn6 = assign15070_e9817_d_n6;
        var_tmf2_dn7 = assign15070_e9817_d_n7;
        var_tmf2_dn8 = assign15070_e9817_d_n8;
        var_tmf2_dn9 = assign15070_e9817_d_n9;
        var_tmf2_dn10 = assign15070_e9817_d_n10;
        var_tmf2_dn11 = assign15070_e9817_d_n11;
        var_tmf2_dn14 = assign15070_e9817_d_n14;

        let (assign15080_e9831, assign15080_e9831_d_n0, assign15080_e9831_d_n2, assign15080_e9831_d_n4, assign15080_e9831_d_n5, assign15080_e9831_d_n6, assign15080_e9831_d_n7, assign15080_e9831_d_n8, assign15080_e9831_d_n9, assign15080_e9831_d_n10, assign15080_e9831_d_n11, assign15080_e9831_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign15080_e9827: f64 = (var_tmf1 / var_tmf2);
        let assign15080_e9828: f64 = (1.0 + assign15080_e9827);
        let assign15080_e9829: f64 = (0.5 * assign15080_e9828);
        (assign15080_e9829, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn14,)
    }
};
        var_t6 = assign15080_e9831;
        var_t6_dn0 = assign15080_e9831_d_n0;
        var_t6_dn2 = assign15080_e9831_d_n2;
        var_t6_dn4 = assign15080_e9831_d_n4;
        var_t6_dn5 = assign15080_e9831_d_n5;
        var_t6_dn6 = assign15080_e9831_d_n6;
        var_t6_dn7 = assign15080_e9831_d_n7;
        var_t6_dn8 = assign15080_e9831_d_n8;
        var_t6_dn9 = assign15080_e9831_d_n9;
        var_t6_dn10 = assign15080_e9831_d_n10;
        var_t6_dn11 = assign15080_e9831_d_n11;
        var_t6_dn14 = assign15080_e9831_d_n14;

        let (assign15090_e9849, assign15090_e9849_d_n0, assign15090_e9849_d_n2, assign15090_e9849_d_n4, assign15090_e9849_d_n5, assign15090_e9849_d_n6, assign15090_e9849_d_n7, assign15090_e9849_d_n8, assign15090_e9849_d_n9, assign15090_e9849_d_n10, assign15090_e9849_d_n11, assign15090_e9849_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign15090_e9840: f64 = (p.p98 + 1.0);
        let assign15090_e9841: f64 = (var_t4 * assign15090_e9840);
        let assign15090_e9845: f64 = (var_tmf1 + var_tmf2);
        let assign15090_e9846: f64 = (0.5 * assign15090_e9845);
        let assign15090_e9847: f64 = (assign15090_e9841 - assign15090_e9846);
        (assign15090_e9847, ((var_t4_dn0 * assign15090_e9840) - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), ((var_t4_dn2 * assign15090_e9840) - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), ((var_t4_dn4 * assign15090_e9840) - (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), ((var_t4_dn5 * assign15090_e9840) - (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), ((var_t4_dn6 * assign15090_e9840) - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), ((var_t4_dn7 * assign15090_e9840) - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), ((var_t4_dn8 * assign15090_e9840) - (0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), ((var_t4_dn9 * assign15090_e9840) - (0.5 * (var_tmf1_dn9 + var_tmf2_dn9))), ((var_t4_dn10 * assign15090_e9840) - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), ((var_t4_dn11 * assign15090_e9840) - (0.5 * (var_tmf1_dn11 + var_tmf2_dn11))), ((var_t4_dn14 * assign15090_e9840) - (0.5 * (var_tmf1_dn14 + var_tmf2_dn14))),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn11, var_t7_dn14,)
    }
};
        var_t7 = assign15090_e9849;
        var_t7_dn0 = assign15090_e9849_d_n0;
        var_t7_dn2 = assign15090_e9849_d_n2;
        var_t7_dn4 = assign15090_e9849_d_n4;
        var_t7_dn5 = assign15090_e9849_d_n5;
        var_t7_dn6 = assign15090_e9849_d_n6;
        var_t7_dn7 = assign15090_e9849_d_n7;
        var_t7_dn8 = assign15090_e9849_d_n8;
        var_t7_dn9 = assign15090_e9849_d_n9;
        var_t7_dn10 = assign15090_e9849_d_n10;
        var_t7_dn11 = assign15090_e9849_d_n11;
        var_t7_dn14 = assign15090_e9849_d_n14;

        let (assign15100_e9865, assign15100_e9865_d_n0, assign15100_e9865_d_n2, assign15100_e9865_d_n4, assign15100_e9865_d_n5, assign15100_e9865_d_n6, assign15100_e9865_d_n7, assign15100_e9865_d_n8, assign15100_e9865_d_n9, assign15100_e9865_d_n10, assign15100_e9865_d_n11, assign15100_e9865_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign15100_e9858: f64 = (var_t1 * var_t4);
        let assign15100_e9859: f64 = (var_t7 + assign15100_e9858);
        let assign15100_e9861: f64 = assign15100_e9859;
        let assign15100_e9863: f64 = (assign15100_e9861 - 5e-5);
        (assign15100_e9863, (var_t7_dn0 + ((var_t1_dn0 * var_t4) + (var_t1 * var_t4_dn0))), (var_t7_dn2 + ((var_t1_dn2 * var_t4) + (var_t1 * var_t4_dn2))), (var_t7_dn4 + ((var_t1_dn4 * var_t4) + (var_t1 * var_t4_dn4))), (var_t7_dn5 + ((var_t1_dn5 * var_t4) + (var_t1 * var_t4_dn5))), (var_t7_dn6 + ((var_t1_dn6 * var_t4) + (var_t1 * var_t4_dn6))), (var_t7_dn7 + ((var_t1_dn7 * var_t4) + (var_t1 * var_t4_dn7))), (var_t7_dn8 + ((var_t1_dn8 * var_t4) + (var_t1 * var_t4_dn8))), (var_t7_dn9 + ((var_t1_dn9 * var_t4) + (var_t1 * var_t4_dn9))), (var_t7_dn10 + ((var_t1_dn10 * var_t4) + (var_t1 * var_t4_dn10))), (var_t7_dn11 + ((var_t1_dn11 * var_t4) + (var_t1 * var_t4_dn11))), (var_t7_dn14 + ((var_t1_dn14 * var_t4) + (var_t1 * var_t4_dn14))),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign15100_e9865;
        var_tmf1_dn0 = assign15100_e9865_d_n0;
        var_tmf1_dn2 = assign15100_e9865_d_n2;
        var_tmf1_dn4 = assign15100_e9865_d_n4;
        var_tmf1_dn5 = assign15100_e9865_d_n5;
        var_tmf1_dn6 = assign15100_e9865_d_n6;
        var_tmf1_dn7 = assign15100_e9865_d_n7;
        var_tmf1_dn8 = assign15100_e9865_d_n8;
        var_tmf1_dn9 = assign15100_e9865_d_n9;
        var_tmf1_dn10 = assign15100_e9865_d_n10;
        var_tmf1_dn11 = assign15100_e9865_d_n11;
        var_tmf1_dn14 = assign15100_e9865_d_n14;

        let (assign15110_e9877, assign15110_e9877_d_n0, assign15110_e9877_d_n2, assign15110_e9877_d_n4, assign15110_e9877_d_n5, assign15110_e9877_d_n6, assign15110_e9877_d_n7, assign15110_e9877_d_n8, assign15110_e9877_d_n9, assign15110_e9877_d_n10, assign15110_e9877_d_n11, assign15110_e9877_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign15110_e9877;
        var_tmf2_dn0 = assign15110_e9877_d_n0;
        var_tmf2_dn2 = assign15110_e9877_d_n2;
        var_tmf2_dn4 = assign15110_e9877_d_n4;
        var_tmf2_dn5 = assign15110_e9877_d_n5;
        var_tmf2_dn6 = assign15110_e9877_d_n6;
        var_tmf2_dn7 = assign15110_e9877_d_n7;
        var_tmf2_dn8 = assign15110_e9877_d_n8;
        var_tmf2_dn9 = assign15110_e9877_d_n9;
        var_tmf2_dn10 = assign15110_e9877_d_n10;
        var_tmf2_dn11 = assign15110_e9877_d_n11;
        var_tmf2_dn14 = assign15110_e9877_d_n14;

        let (assign15120_e9891, assign15120_e9891_d_n0, assign15120_e9891_d_n2, assign15120_e9891_d_n4, assign15120_e9891_d_n5, assign15120_e9891_d_n6, assign15120_e9891_d_n7, assign15120_e9891_d_n8, assign15120_e9891_d_n9, assign15120_e9891_d_n10, assign15120_e9891_d_n11, assign15120_e9891_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let (assign15120_e9889, assign15120_e9889_d_n0, assign15120_e9889_d_n2, assign15120_e9889_d_n4, assign15120_e9889_d_n5, assign15120_e9889_d_n6, assign15120_e9889_d_n7, assign15120_e9889_d_n8, assign15120_e9889_d_n9, assign15120_e9889_d_n10, assign15120_e9889_d_n11, assign15120_e9889_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign15120_e9888: f64 = (-var_tmf2);
                (assign15120_e9888, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign15120_e9889, assign15120_e9889_d_n0, assign15120_e9889_d_n2, assign15120_e9889_d_n4, assign15120_e9889_d_n5, assign15120_e9889_d_n6, assign15120_e9889_d_n7, assign15120_e9889_d_n8, assign15120_e9889_d_n9, assign15120_e9889_d_n10, assign15120_e9889_d_n11, assign15120_e9889_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign15120_e9891;
        var_tmf2_dn0 = assign15120_e9891_d_n0;
        var_tmf2_dn2 = assign15120_e9891_d_n2;
        var_tmf2_dn4 = assign15120_e9891_d_n4;
        var_tmf2_dn5 = assign15120_e9891_d_n5;
        var_tmf2_dn6 = assign15120_e9891_d_n6;
        var_tmf2_dn7 = assign15120_e9891_d_n7;
        var_tmf2_dn8 = assign15120_e9891_d_n8;
        var_tmf2_dn9 = assign15120_e9891_d_n9;
        var_tmf2_dn10 = assign15120_e9891_d_n10;
        var_tmf2_dn11 = assign15120_e9891_d_n11;
        var_tmf2_dn14 = assign15120_e9891_d_n14;

        let (assign15130_e9904, assign15130_e9904_d_n0, assign15130_e9904_d_n2, assign15130_e9904_d_n4, assign15130_e9904_d_n5, assign15130_e9904_d_n6, assign15130_e9904_d_n7, assign15130_e9904_d_n8, assign15130_e9904_d_n9, assign15130_e9904_d_n10, assign15130_e9904_d_n11, assign15130_e9904_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign15130_e9899: f64 = (var_tmf1 * var_tmf1);
        let assign15130_e9901: f64 = (assign15130_e9899 + var_tmf2);
        let assign15130_e9902: f64 = (assign15130_e9901).sqrt();
        (assign15130_e9902, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15130_e9902)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15130_e9902)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign15130_e9902)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign15130_e9902)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign15130_e9902)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign15130_e9902)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign15130_e9902)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign15130_e9902)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign15130_e9902)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign15130_e9902)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign15130_e9902)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign15130_e9904;
        var_tmf2_dn0 = assign15130_e9904_d_n0;
        var_tmf2_dn2 = assign15130_e9904_d_n2;
        var_tmf2_dn4 = assign15130_e9904_d_n4;
        var_tmf2_dn5 = assign15130_e9904_d_n5;
        var_tmf2_dn6 = assign15130_e9904_d_n6;
        var_tmf2_dn7 = assign15130_e9904_d_n7;
        var_tmf2_dn8 = assign15130_e9904_d_n8;
        var_tmf2_dn9 = assign15130_e9904_d_n9;
        var_tmf2_dn10 = assign15130_e9904_d_n10;
        var_tmf2_dn11 = assign15130_e9904_d_n11;
        var_tmf2_dn14 = assign15130_e9904_d_n14;

        let (assign15140_e9918, assign15140_e9918_d_n0, assign15140_e9918_d_n2, assign15140_e9918_d_n4, assign15140_e9918_d_n5, assign15140_e9918_d_n6, assign15140_e9918_d_n7, assign15140_e9918_d_n8, assign15140_e9918_d_n9, assign15140_e9918_d_n10, assign15140_e9918_d_n11, assign15140_e9918_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign15140_e9914: f64 = (var_tmf1 / var_tmf2);
        let assign15140_e9915: f64 = (1.0 + assign15140_e9914);
        let assign15140_e9916: f64 = (0.5 * assign15140_e9915);
        (assign15140_e9916, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn14,)
    }
};
        var_t6 = assign15140_e9918;
        var_t6_dn0 = assign15140_e9918_d_n0;
        var_t6_dn2 = assign15140_e9918_d_n2;
        var_t6_dn4 = assign15140_e9918_d_n4;
        var_t6_dn5 = assign15140_e9918_d_n5;
        var_t6_dn6 = assign15140_e9918_d_n6;
        var_t6_dn7 = assign15140_e9918_d_n7;
        var_t6_dn8 = assign15140_e9918_d_n8;
        var_t6_dn9 = assign15140_e9918_d_n9;
        var_t6_dn10 = assign15140_e9918_d_n10;
        var_t6_dn11 = assign15140_e9918_d_n11;
        var_t6_dn14 = assign15140_e9918_d_n14;

        let (assign15150_e9932, assign15150_e9932_d_n0, assign15150_e9932_d_n2, assign15150_e9932_d_n4, assign15150_e9932_d_n5, assign15150_e9932_d_n6, assign15150_e9932_d_n7, assign15150_e9932_d_n8, assign15150_e9932_d_n9, assign15150_e9932_d_n10, assign15150_e9932_d_n11, assign15150_e9932_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) {
        let assign15150_e9928: f64 = (var_tmf1 + var_tmf2);
        let assign15150_e9929: f64 = (0.5 * assign15150_e9928);
        let assign15150_e9930: f64 = assign15150_e9929;
        (assign15150_e9930, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_dn6 + var_tmf2_dn6)), (0.5 * (var_tmf1_dn7 + var_tmf2_dn7)), (0.5 * (var_tmf1_dn8 + var_tmf2_dn8)), (0.5 * (var_tmf1_dn9 + var_tmf2_dn9)), (0.5 * (var_tmf1_dn10 + var_tmf2_dn10)), (0.5 * (var_tmf1_dn11 + var_tmf2_dn11)), (0.5 * (var_tmf1_dn14 + var_tmf2_dn14)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign15150_e9932;
        var_t2_dn0 = assign15150_e9932_d_n0;
        var_t2_dn2 = assign15150_e9932_d_n2;
        var_t2_dn4 = assign15150_e9932_d_n4;
        var_t2_dn5 = assign15150_e9932_d_n5;
        var_t2_dn6 = assign15150_e9932_d_n6;
        var_t2_dn7 = assign15150_e9932_d_n7;
        var_t2_dn8 = assign15150_e9932_d_n8;
        var_t2_dn9 = assign15150_e9932_d_n9;
        var_t2_dn10 = assign15150_e9932_d_n10;
        var_t2_dn11 = assign15150_e9932_d_n11;
        var_t2_dn14 = assign15150_e9932_d_n14;

        let assign15160_e9939: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        var_guard325 = assign15160_e9939;

        let (assign15170_e9959, assign15170_e9959_d_n0, assign15170_e9959_d_n2, assign15170_e9959_d_n4, assign15170_e9959_d_n5, assign15170_e9959_d_n6, assign15170_e9959_d_n7, assign15170_e9959_d_n8, assign15170_e9959_d_n9, assign15170_e9959_d_n10, assign15170_e9959_d_n11, assign15170_e9959_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard325 != 0.0)) {
        let assign15170_e9950: f64 = (var_mks_rdvdtemp1 * var_tdiff0);
        let assign15170_e9951: f64 = (var_uc_rdvd + assign15170_e9950);
        let assign15170_e9954: f64 = (var_mks_rdvdtemp2 * var_tdiff0_2);
        let assign15170_e9955: f64 = (assign15170_e9951 + assign15170_e9954);
        let assign15170_e9957: f64 = (assign15170_e9955 * var_t2);
        (assign15170_e9957, ((((var_mks_rdvdtemp1 * var_tdiff0_dn0) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn0)) * var_t2) + (assign15170_e9955 * var_t2_dn0)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn2) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn2)) * var_t2) + (assign15170_e9955 * var_t2_dn2)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn4) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn4)) * var_t2) + (assign15170_e9955 * var_t2_dn4)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn5) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn5)) * var_t2) + (assign15170_e9955 * var_t2_dn5)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn6) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn6)) * var_t2) + (assign15170_e9955 * var_t2_dn6)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn7) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn7)) * var_t2) + (assign15170_e9955 * var_t2_dn7)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn8) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn8)) * var_t2) + (assign15170_e9955 * var_t2_dn8)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn9) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn9)) * var_t2) + (assign15170_e9955 * var_t2_dn9)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn10) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn10)) * var_t2) + (assign15170_e9955 * var_t2_dn10)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn11) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn11)) * var_t2) + (assign15170_e9955 * var_t2_dn11)), ((((var_mks_rdvdtemp1 * var_tdiff0_dn14) + (var_mks_rdvdtemp2 * var_tdiff0_2_dn14)) * var_t2) + (assign15170_e9955 * var_t2_dn14)),)
    } else {
        (var_rsvde, var_rsvde_dn0, var_rsvde_dn2, var_rsvde_dn4, var_rsvde_dn5, var_rsvde_dn6, var_rsvde_dn7, var_rsvde_dn8, var_rsvde_dn9, var_rsvde_dn10, var_rsvde_dn11, var_rsvde_dn14,)
    }
};
        var_rsvde = assign15170_e9959;
        var_rsvde_dn0 = assign15170_e9959_d_n0;
        var_rsvde_dn2 = assign15170_e9959_d_n2;
        var_rsvde_dn4 = assign15170_e9959_d_n4;
        var_rsvde_dn5 = assign15170_e9959_d_n5;
        var_rsvde_dn6 = assign15170_e9959_d_n6;
        var_rsvde_dn7 = assign15170_e9959_d_n7;
        var_rsvde_dn8 = assign15170_e9959_d_n8;
        var_rsvde_dn9 = assign15170_e9959_d_n9;
        var_rsvde_dn10 = assign15170_e9959_d_n10;
        var_rsvde_dn11 = assign15170_e9959_d_n11;
        var_rsvde_dn14 = assign15170_e9959_d_n14;

        let (assign15180_e9977, assign15180_e9977_d_n0, assign15180_e9977_d_n2, assign15180_e9977_d_n4, assign15180_e9977_d_n5, assign15180_e9977_d_n6, assign15180_e9977_d_n7, assign15180_e9977_d_n8, assign15180_e9977_d_n9, assign15180_e9977_d_n10, assign15180_e9977_d_n11, assign15180_e9977_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard325 != 0.0)) {
        let assign15180_e9970: f64 = (0.005 * var_uc_rdvd);
        let assign15180_e9971: f64 = (var_rsvde - assign15180_e9970);
        let assign15180_e9974: f64 = (0.01 * var_uc_rdvd);
        let assign15180_e9975: f64 = (assign15180_e9971 - assign15180_e9974);
        (assign15180_e9975, var_rsvde_dn0, var_rsvde_dn2, var_rsvde_dn4, var_rsvde_dn5, var_rsvde_dn6, var_rsvde_dn7, var_rsvde_dn8, var_rsvde_dn9, var_rsvde_dn10, var_rsvde_dn11, var_rsvde_dn14,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign15180_e9977;
        var_tmf1_dn0 = assign15180_e9977_d_n0;
        var_tmf1_dn2 = assign15180_e9977_d_n2;
        var_tmf1_dn4 = assign15180_e9977_d_n4;
        var_tmf1_dn5 = assign15180_e9977_d_n5;
        var_tmf1_dn6 = assign15180_e9977_d_n6;
        var_tmf1_dn7 = assign15180_e9977_d_n7;
        var_tmf1_dn8 = assign15180_e9977_d_n8;
        var_tmf1_dn9 = assign15180_e9977_d_n9;
        var_tmf1_dn10 = assign15180_e9977_d_n10;
        var_tmf1_dn11 = assign15180_e9977_d_n11;
        var_tmf1_dn14 = assign15180_e9977_d_n14;

        let (assign15190_e9995, assign15190_e9995_d_n0, assign15190_e9995_d_n2, assign15190_e9995_d_n4, assign15190_e9995_d_n5, assign15190_e9995_d_n6, assign15190_e9995_d_n7, assign15190_e9995_d_n8, assign15190_e9995_d_n9, assign15190_e9995_d_n10, assign15190_e9995_d_n11, assign15190_e9995_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard325 != 0.0)) {
        let assign15190_e9988: f64 = (0.005 * var_uc_rdvd);
        let assign15190_e9989: f64 = (4.0 * assign15190_e9988);
        let assign15190_e9992: f64 = (0.01 * var_uc_rdvd);
        let assign15190_e9993: f64 = (assign15190_e9989 * assign15190_e9992);
        (assign15190_e9993, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign15190_e9995;
        var_tmf2_dn0 = assign15190_e9995_d_n0;
        var_tmf2_dn2 = assign15190_e9995_d_n2;
        var_tmf2_dn4 = assign15190_e9995_d_n4;
        var_tmf2_dn5 = assign15190_e9995_d_n5;
        var_tmf2_dn6 = assign15190_e9995_d_n6;
        var_tmf2_dn7 = assign15190_e9995_d_n7;
        var_tmf2_dn8 = assign15190_e9995_d_n8;
        var_tmf2_dn9 = assign15190_e9995_d_n9;
        var_tmf2_dn10 = assign15190_e9995_d_n10;
        var_tmf2_dn11 = assign15190_e9995_d_n11;
        var_tmf2_dn14 = assign15190_e9995_d_n14;

        let (assign15200_e10011, assign15200_e10011_d_n0, assign15200_e10011_d_n2, assign15200_e10011_d_n4, assign15200_e10011_d_n5, assign15200_e10011_d_n6, assign15200_e10011_d_n7, assign15200_e10011_d_n8, assign15200_e10011_d_n9, assign15200_e10011_d_n10, assign15200_e10011_d_n11, assign15200_e10011_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard325 != 0.0)) {
        let (assign15200_e10009, assign15200_e10009_d_n0, assign15200_e10009_d_n2, assign15200_e10009_d_n4, assign15200_e10009_d_n5, assign15200_e10009_d_n6, assign15200_e10009_d_n7, assign15200_e10009_d_n8, assign15200_e10009_d_n9, assign15200_e10009_d_n10, assign15200_e10009_d_n11, assign15200_e10009_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign15200_e10008: f64 = (-var_tmf2);
                (assign15200_e10008, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign15200_e10009, assign15200_e10009_d_n0, assign15200_e10009_d_n2, assign15200_e10009_d_n4, assign15200_e10009_d_n5, assign15200_e10009_d_n6, assign15200_e10009_d_n7, assign15200_e10009_d_n8, assign15200_e10009_d_n9, assign15200_e10009_d_n10, assign15200_e10009_d_n11, assign15200_e10009_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign15200_e10011;
        var_tmf2_dn0 = assign15200_e10011_d_n0;
        var_tmf2_dn2 = assign15200_e10011_d_n2;
        var_tmf2_dn4 = assign15200_e10011_d_n4;
        var_tmf2_dn5 = assign15200_e10011_d_n5;
        var_tmf2_dn6 = assign15200_e10011_d_n6;
        var_tmf2_dn7 = assign15200_e10011_d_n7;
        var_tmf2_dn8 = assign15200_e10011_d_n8;
        var_tmf2_dn9 = assign15200_e10011_d_n9;
        var_tmf2_dn10 = assign15200_e10011_d_n10;
        var_tmf2_dn11 = assign15200_e10011_d_n11;
        var_tmf2_dn14 = assign15200_e10011_d_n14;

        let (assign15210_e10026, assign15210_e10026_d_n0, assign15210_e10026_d_n2, assign15210_e10026_d_n4, assign15210_e10026_d_n5, assign15210_e10026_d_n6, assign15210_e10026_d_n7, assign15210_e10026_d_n8, assign15210_e10026_d_n9, assign15210_e10026_d_n10, assign15210_e10026_d_n11, assign15210_e10026_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard325 != 0.0)) {
        let assign15210_e10021: f64 = (var_tmf1 * var_tmf1);
        let assign15210_e10023: f64 = (assign15210_e10021 + var_tmf2);
        let assign15210_e10024: f64 = (assign15210_e10023).sqrt();
        (assign15210_e10024, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15210_e10024)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15210_e10024)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign15210_e10024)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign15210_e10024)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign15210_e10024)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign15210_e10024)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign15210_e10024)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign15210_e10024)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign15210_e10024)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign15210_e10024)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign15210_e10024)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign15210_e10026;
        var_tmf2_dn0 = assign15210_e10026_d_n0;
        var_tmf2_dn2 = assign15210_e10026_d_n2;
        var_tmf2_dn4 = assign15210_e10026_d_n4;
        var_tmf2_dn5 = assign15210_e10026_d_n5;
        var_tmf2_dn6 = assign15210_e10026_d_n6;
        var_tmf2_dn7 = assign15210_e10026_d_n7;
        var_tmf2_dn8 = assign15210_e10026_d_n8;
        var_tmf2_dn9 = assign15210_e10026_d_n9;
        var_tmf2_dn10 = assign15210_e10026_d_n10;
        var_tmf2_dn11 = assign15210_e10026_d_n11;
        var_tmf2_dn14 = assign15210_e10026_d_n14;

        let (assign15220_e10042, assign15220_e10042_d_n0, assign15220_e10042_d_n2, assign15220_e10042_d_n4, assign15220_e10042_d_n5, assign15220_e10042_d_n6, assign15220_e10042_d_n7, assign15220_e10042_d_n8, assign15220_e10042_d_n9, assign15220_e10042_d_n10, assign15220_e10042_d_n11, assign15220_e10042_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard325 != 0.0)) {
        let assign15220_e10038: f64 = (var_tmf1 / var_tmf2);
        let assign15220_e10039: f64 = (1.0 + assign15220_e10038);
        let assign15220_e10040: f64 = (0.5 * assign15220_e10039);
        (assign15220_e10040, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign15220_e10042;
        var_t0_dn0 = assign15220_e10042_d_n0;
        var_t0_dn2 = assign15220_e10042_d_n2;
        var_t0_dn4 = assign15220_e10042_d_n4;
        var_t0_dn5 = assign15220_e10042_d_n5;
        var_t0_dn6 = assign15220_e10042_d_n6;
        var_t0_dn7 = assign15220_e10042_d_n7;
        var_t0_dn8 = assign15220_e10042_d_n8;
        var_t0_dn9 = assign15220_e10042_d_n9;
        var_t0_dn10 = assign15220_e10042_d_n10;
        var_t0_dn11 = assign15220_e10042_d_n11;
        var_t0_dn14 = assign15220_e10042_d_n14;

        let (assign15230_e10060, assign15230_e10060_d_n0, assign15230_e10060_d_n2, assign15230_e10060_d_n4, assign15230_e10060_d_n5, assign15230_e10060_d_n6, assign15230_e10060_d_n7, assign15230_e10060_d_n8, assign15230_e10060_d_n9, assign15230_e10060_d_n10, assign15230_e10060_d_n11, assign15230_e10060_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard325 != 0.0)) {
        let assign15230_e10052: f64 = (0.005 * var_uc_rdvd);
        let assign15230_e10056: f64 = (var_tmf1 + var_tmf2);
        let assign15230_e10057: f64 = (0.5 * assign15230_e10056);
        let assign15230_e10058: f64 = (assign15230_e10052 + assign15230_e10057);
        (assign15230_e10058, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_dn6 + var_tmf2_dn6)), (0.5 * (var_tmf1_dn7 + var_tmf2_dn7)), (0.5 * (var_tmf1_dn8 + var_tmf2_dn8)), (0.5 * (var_tmf1_dn9 + var_tmf2_dn9)), (0.5 * (var_tmf1_dn10 + var_tmf2_dn10)), (0.5 * (var_tmf1_dn11 + var_tmf2_dn11)), (0.5 * (var_tmf1_dn14 + var_tmf2_dn14)),)
    } else {
        (var_rsvde, var_rsvde_dn0, var_rsvde_dn2, var_rsvde_dn4, var_rsvde_dn5, var_rsvde_dn6, var_rsvde_dn7, var_rsvde_dn8, var_rsvde_dn9, var_rsvde_dn10, var_rsvde_dn11, var_rsvde_dn14,)
    }
};
        var_rsvde = assign15230_e10060;
        var_rsvde_dn0 = assign15230_e10060_d_n0;
        var_rsvde_dn2 = assign15230_e10060_d_n2;
        var_rsvde_dn4 = assign15230_e10060_d_n4;
        var_rsvde_dn5 = assign15230_e10060_d_n5;
        var_rsvde_dn6 = assign15230_e10060_d_n6;
        var_rsvde_dn7 = assign15230_e10060_d_n7;
        var_rsvde_dn8 = assign15230_e10060_d_n8;
        var_rsvde_dn9 = assign15230_e10060_d_n9;
        var_rsvde_dn10 = assign15230_e10060_d_n10;
        var_rsvde_dn11 = assign15230_e10060_d_n11;
        var_rsvde_dn14 = assign15230_e10060_d_n14;

        let (assign15240_e10081, assign15240_e10081_d_n0, assign15240_e10081_d_n2, assign15240_e10081_d_n4, assign15240_e10081_d_n5, assign15240_e10081_d_n6, assign15240_e10081_d_n7, assign15240_e10081_d_n8, assign15240_e10081_d_n9, assign15240_e10081_d_n10, assign15240_e10081_d_n11, assign15240_e10081_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard325 == 0.0)) {
        let assign15240_e10072: f64 = (var_mks_rdvdtemp1 * var_tdiff);
        let assign15240_e10073: f64 = (var_uc_rdvd + assign15240_e10072);
        let assign15240_e10076: f64 = (var_mks_rdvdtemp2 * var_tdiff_2);
        let assign15240_e10077: f64 = (assign15240_e10073 + assign15240_e10076);
        let assign15240_e10079: f64 = (assign15240_e10077 * var_t2);
        (assign15240_e10079, ((((var_mks_rdvdtemp1 * var_tdiff_dn0) + (var_mks_rdvdtemp2 * var_tdiff_2_dn0)) * var_t2) + (assign15240_e10077 * var_t2_dn0)), ((((var_mks_rdvdtemp1 * var_tdiff_dn2) + (var_mks_rdvdtemp2 * var_tdiff_2_dn2)) * var_t2) + (assign15240_e10077 * var_t2_dn2)), ((((var_mks_rdvdtemp1 * var_tdiff_dn4) + (var_mks_rdvdtemp2 * var_tdiff_2_dn4)) * var_t2) + (assign15240_e10077 * var_t2_dn4)), ((((var_mks_rdvdtemp1 * var_tdiff_dn5) + (var_mks_rdvdtemp2 * var_tdiff_2_dn5)) * var_t2) + (assign15240_e10077 * var_t2_dn5)), ((((var_mks_rdvdtemp1 * var_tdiff_dn6) + (var_mks_rdvdtemp2 * var_tdiff_2_dn6)) * var_t2) + (assign15240_e10077 * var_t2_dn6)), ((((var_mks_rdvdtemp1 * var_tdiff_dn7) + (var_mks_rdvdtemp2 * var_tdiff_2_dn7)) * var_t2) + (assign15240_e10077 * var_t2_dn7)), ((((var_mks_rdvdtemp1 * var_tdiff_dn8) + (var_mks_rdvdtemp2 * var_tdiff_2_dn8)) * var_t2) + (assign15240_e10077 * var_t2_dn8)), ((((var_mks_rdvdtemp1 * var_tdiff_dn9) + (var_mks_rdvdtemp2 * var_tdiff_2_dn9)) * var_t2) + (assign15240_e10077 * var_t2_dn9)), ((((var_mks_rdvdtemp1 * var_tdiff_dn10) + (var_mks_rdvdtemp2 * var_tdiff_2_dn10)) * var_t2) + (assign15240_e10077 * var_t2_dn10)), ((((var_mks_rdvdtemp1 * var_tdiff_dn11) + (var_mks_rdvdtemp2 * var_tdiff_2_dn11)) * var_t2) + (assign15240_e10077 * var_t2_dn11)), ((((var_mks_rdvdtemp1 * var_tdiff_dn14) + (var_mks_rdvdtemp2 * var_tdiff_2_dn14)) * var_t2) + (assign15240_e10077 * var_t2_dn14)),)
    } else {
        (var_rsvde, var_rsvde_dn0, var_rsvde_dn2, var_rsvde_dn4, var_rsvde_dn5, var_rsvde_dn6, var_rsvde_dn7, var_rsvde_dn8, var_rsvde_dn9, var_rsvde_dn10, var_rsvde_dn11, var_rsvde_dn14,)
    }
};
        var_rsvde = assign15240_e10081;
        var_rsvde_dn0 = assign15240_e10081_d_n0;
        var_rsvde_dn2 = assign15240_e10081_d_n2;
        var_rsvde_dn4 = assign15240_e10081_d_n4;
        var_rsvde_dn5 = assign15240_e10081_d_n5;
        var_rsvde_dn6 = assign15240_e10081_d_n6;
        var_rsvde_dn7 = assign15240_e10081_d_n7;
        var_rsvde_dn8 = assign15240_e10081_d_n8;
        var_rsvde_dn9 = assign15240_e10081_d_n9;
        var_rsvde_dn10 = assign15240_e10081_d_n10;
        var_rsvde_dn11 = assign15240_e10081_d_n11;
        var_rsvde_dn14 = assign15240_e10081_d_n14;

        let (assign15250_e10100, assign15250_e10100_d_n0, assign15250_e10100_d_n2, assign15250_e10100_d_n4, assign15250_e10100_d_n5, assign15250_e10100_d_n6, assign15250_e10100_d_n7, assign15250_e10100_d_n8, assign15250_e10100_d_n9, assign15250_e10100_d_n10, assign15250_e10100_d_n11, assign15250_e10100_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard325 == 0.0)) {
        let assign15250_e10093: f64 = (0.005 * var_uc_rdvd);
        let assign15250_e10094: f64 = (var_rsvde - assign15250_e10093);
        let assign15250_e10097: f64 = (0.01 * var_uc_rdvd);
        let assign15250_e10098: f64 = (assign15250_e10094 - assign15250_e10097);
        (assign15250_e10098, var_rsvde_dn0, var_rsvde_dn2, var_rsvde_dn4, var_rsvde_dn5, var_rsvde_dn6, var_rsvde_dn7, var_rsvde_dn8, var_rsvde_dn9, var_rsvde_dn10, var_rsvde_dn11, var_rsvde_dn14,)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn11, var_tmf1_dn14,)
    }
};
        var_tmf1 = assign15250_e10100;
        var_tmf1_dn0 = assign15250_e10100_d_n0;
        var_tmf1_dn2 = assign15250_e10100_d_n2;
        var_tmf1_dn4 = assign15250_e10100_d_n4;
        var_tmf1_dn5 = assign15250_e10100_d_n5;
        var_tmf1_dn6 = assign15250_e10100_d_n6;
        var_tmf1_dn7 = assign15250_e10100_d_n7;
        var_tmf1_dn8 = assign15250_e10100_d_n8;
        var_tmf1_dn9 = assign15250_e10100_d_n9;
        var_tmf1_dn10 = assign15250_e10100_d_n10;
        var_tmf1_dn11 = assign15250_e10100_d_n11;
        var_tmf1_dn14 = assign15250_e10100_d_n14;

        let (assign15260_e10119, assign15260_e10119_d_n0, assign15260_e10119_d_n2, assign15260_e10119_d_n4, assign15260_e10119_d_n5, assign15260_e10119_d_n6, assign15260_e10119_d_n7, assign15260_e10119_d_n8, assign15260_e10119_d_n9, assign15260_e10119_d_n10, assign15260_e10119_d_n11, assign15260_e10119_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard325 == 0.0)) {
        let assign15260_e10112: f64 = (0.005 * var_uc_rdvd);
        let assign15260_e10113: f64 = (4.0 * assign15260_e10112);
        let assign15260_e10116: f64 = (0.01 * var_uc_rdvd);
        let assign15260_e10117: f64 = (assign15260_e10113 * assign15260_e10116);
        (assign15260_e10117, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign15260_e10119;
        var_tmf2_dn0 = assign15260_e10119_d_n0;
        var_tmf2_dn2 = assign15260_e10119_d_n2;
        var_tmf2_dn4 = assign15260_e10119_d_n4;
        var_tmf2_dn5 = assign15260_e10119_d_n5;
        var_tmf2_dn6 = assign15260_e10119_d_n6;
        var_tmf2_dn7 = assign15260_e10119_d_n7;
        var_tmf2_dn8 = assign15260_e10119_d_n8;
        var_tmf2_dn9 = assign15260_e10119_d_n9;
        var_tmf2_dn10 = assign15260_e10119_d_n10;
        var_tmf2_dn11 = assign15260_e10119_d_n11;
        var_tmf2_dn14 = assign15260_e10119_d_n14;

        *var_guard325_slot = var_guard325;
        *var_rsvde_slot = var_rsvde;
        *var_rsvde_dn0_slot = var_rsvde_dn0;
        *var_rsvde_dn10_slot = var_rsvde_dn10;
        *var_rsvde_dn11_slot = var_rsvde_dn11;
        *var_rsvde_dn14_slot = var_rsvde_dn14;
        *var_rsvde_dn2_slot = var_rsvde_dn2;
        *var_rsvde_dn4_slot = var_rsvde_dn4;
        *var_rsvde_dn5_slot = var_rsvde_dn5;
        *var_rsvde_dn6_slot = var_rsvde_dn6;
        *var_rsvde_dn7_slot = var_rsvde_dn7;
        *var_rsvde_dn8_slot = var_rsvde_dn8;
        *var_rsvde_dn9_slot = var_rsvde_dn9;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn14_slot = var_t6_dn14;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
        *var_t7_slot = var_t7;
        *var_t7_dn0_slot = var_t7_dn0;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn14_slot = var_t7_dn14;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn7_slot = var_t7_dn7;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_dn9_slot = var_t7_dn9;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn14_slot = var_tmf1_dn14;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_dn9_slot = var_tmf1_dn9;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn14_slot = var_tmf2_dn14;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_dn9_slot = var_tmf2_dn9;
    }

    pub(super) fn stamp_transient_block_31(
        p: &Parameters,
        var_beta_inv: f64,
        var_beta_inv_dn0: f64,
        var_beta_inv_dn10: f64,
        var_beta_inv_dn11: f64,
        var_beta_inv_dn14: f64,
        var_beta_inv_dn2: f64,
        var_beta_inv_dn4: f64,
        var_beta_inv_dn5: f64,
        var_beta_inv_dn6: f64,
        var_beta_inv_dn7: f64,
        var_beta_inv_dn8: f64,
        var_beta_inv_dn9: f64,
        var_costi00: f64,
        var_guard293: f64,
        var_guard317: f64,
        var_guard322: f64,
        var_guard325: f64,
        var_ktnom: f64,
        var_mks_rdrmue: f64,
        var_mks_rdrmues: f64,
        var_mks_rdrvmax: f64,
        var_mks_rdrvmaxs: f64,
        var_nin: f64,
        var_nin_dn0: f64,
        var_nin_dn10: f64,
        var_nin_dn11: f64,
        var_nin_dn14: f64,
        var_nin_dn2: f64,
        var_nin_dn4: f64,
        var_nin_dn5: f64,
        var_nin_dn6: f64,
        var_nin_dn7: f64,
        var_nin_dn8: f64,
        var_nin_dn9: f64,
        var_nsti_p2: f64,
        var_tdiff: f64,
        var_tdiff_dn0: f64,
        var_tdiff_dn10: f64,
        var_tdiff_dn11: f64,
        var_tdiff_dn14: f64,
        var_tdiff_dn2: f64,
        var_tdiff_dn4: f64,
        var_tdiff_dn5: f64,
        var_tdiff_dn6: f64,
        var_tdiff_dn7: f64,
        var_tdiff_dn8: f64,
        var_tdiff_dn9: f64,
        var_tmf1: f64,
        var_tmf1_dn0: f64,
        var_tmf1_dn10: f64,
        var_tmf1_dn11: f64,
        var_tmf1_dn14: f64,
        var_tmf1_dn2: f64,
        var_tmf1_dn4: f64,
        var_tmf1_dn5: f64,
        var_tmf1_dn6: f64,
        var_tmf1_dn7: f64,
        var_tmf1_dn8: f64,
        var_tmf1_dn9: f64,
        var_tratio: f64,
        var_tratio_dn0: f64,
        var_tratio_dn10: f64,
        var_tratio_dn11: f64,
        var_tratio_dn14: f64,
        var_tratio_dn2: f64,
        var_tratio_dn4: f64,
        var_tratio_dn5: f64,
        var_tratio_dn6: f64,
        var_tratio_dn7: f64,
        var_tratio_dn8: f64,
        var_tratio_dn9: f64,
        var_ttemp: f64,
        var_ttemp_dn0: f64,
        var_ttemp_dn10: f64,
        var_ttemp_dn11: f64,
        var_ttemp_dn14: f64,
        var_ttemp_dn2: f64,
        var_ttemp_dn4: f64,
        var_ttemp_dn5: f64,
        var_ttemp_dn6: f64,
        var_ttemp_dn7: f64,
        var_ttemp_dn8: f64,
        var_ttemp_dn9: f64,
        var_uc_cordrift: f64,
        var_uc_rdvd: f64,
        var_costi0_slot: &mut f64,
        var_costi0_dn0_slot: &mut f64,
        var_costi0_dn10_slot: &mut f64,
        var_costi0_dn11_slot: &mut f64,
        var_costi0_dn14_slot: &mut f64,
        var_costi0_dn2_slot: &mut f64,
        var_costi0_dn4_slot: &mut f64,
        var_costi0_dn5_slot: &mut f64,
        var_costi0_dn6_slot: &mut f64,
        var_costi0_dn7_slot: &mut f64,
        var_costi0_dn8_slot: &mut f64,
        var_costi0_dn9_slot: &mut f64,
        var_costi0_p2_slot: &mut f64,
        var_costi0_p2_dn0_slot: &mut f64,
        var_costi0_p2_dn10_slot: &mut f64,
        var_costi0_p2_dn11_slot: &mut f64,
        var_costi0_p2_dn14_slot: &mut f64,
        var_costi0_p2_dn2_slot: &mut f64,
        var_costi0_p2_dn4_slot: &mut f64,
        var_costi0_p2_dn5_slot: &mut f64,
        var_costi0_p2_dn6_slot: &mut f64,
        var_costi0_p2_dn7_slot: &mut f64,
        var_costi0_p2_dn8_slot: &mut f64,
        var_costi0_p2_dn9_slot: &mut f64,
        var_costi1_slot: &mut f64,
        var_costi1_dn0_slot: &mut f64,
        var_costi1_dn10_slot: &mut f64,
        var_costi1_dn11_slot: &mut f64,
        var_costi1_dn14_slot: &mut f64,
        var_costi1_dn2_slot: &mut f64,
        var_costi1_dn4_slot: &mut f64,
        var_costi1_dn5_slot: &mut f64,
        var_costi1_dn6_slot: &mut f64,
        var_costi1_dn7_slot: &mut f64,
        var_costi1_dn8_slot: &mut f64,
        var_costi1_dn9_slot: &mut f64,
        var_guard328_slot: &mut f64,
        var_guard329_slot: &mut f64,
        var_guard330_slot: &mut f64,
        var_guard332_slot: &mut f64,
        var_hbdceff_slot: &mut f64,
        var_hbdceff_dn0_slot: &mut f64,
        var_hbdceff_dn10_slot: &mut f64,
        var_hbdceff_dn11_slot: &mut f64,
        var_hbdceff_dn14_slot: &mut f64,
        var_hbdceff_dn2_slot: &mut f64,
        var_hbdceff_dn4_slot: &mut f64,
        var_hbdceff_dn5_slot: &mut f64,
        var_hbdceff_dn6_slot: &mut f64,
        var_hbdceff_dn7_slot: &mut f64,
        var_hbdceff_dn8_slot: &mut f64,
        var_hbdceff_dn9_slot: &mut f64,
        var_rdvde_slot: &mut f64,
        var_rdvde_dn0_slot: &mut f64,
        var_rdvde_dn10_slot: &mut f64,
        var_rdvde_dn11_slot: &mut f64,
        var_rdvde_dn14_slot: &mut f64,
        var_rdvde_dn2_slot: &mut f64,
        var_rdvde_dn4_slot: &mut f64,
        var_rdvde_dn5_slot: &mut f64,
        var_rdvde_dn6_slot: &mut f64,
        var_rdvde_dn7_slot: &mut f64,
        var_rdvde_dn8_slot: &mut f64,
        var_rdvde_dn9_slot: &mut f64,
        var_rrdrmue_slot: &mut f64,
        var_rrdrmue_dn0_slot: &mut f64,
        var_rrdrmue_dn10_slot: &mut f64,
        var_rrdrmue_dn11_slot: &mut f64,
        var_rrdrmue_dn14_slot: &mut f64,
        var_rrdrmue_dn2_slot: &mut f64,
        var_rrdrmue_dn4_slot: &mut f64,
        var_rrdrmue_dn5_slot: &mut f64,
        var_rrdrmue_dn6_slot: &mut f64,
        var_rrdrmue_dn7_slot: &mut f64,
        var_rrdrmue_dn8_slot: &mut f64,
        var_rrdrmue_dn9_slot: &mut f64,
        var_rrdrmues_slot: &mut f64,
        var_rrdrmues_dn0_slot: &mut f64,
        var_rrdrmues_dn10_slot: &mut f64,
        var_rrdrmues_dn11_slot: &mut f64,
        var_rrdrmues_dn14_slot: &mut f64,
        var_rrdrmues_dn2_slot: &mut f64,
        var_rrdrmues_dn4_slot: &mut f64,
        var_rrdrmues_dn5_slot: &mut f64,
        var_rrdrmues_dn6_slot: &mut f64,
        var_rrdrmues_dn7_slot: &mut f64,
        var_rrdrmues_dn8_slot: &mut f64,
        var_rrdrmues_dn9_slot: &mut f64,
        var_rrdrvmax_slot: &mut f64,
        var_rrdrvmax_dn0_slot: &mut f64,
        var_rrdrvmax_dn10_slot: &mut f64,
        var_rrdrvmax_dn11_slot: &mut f64,
        var_rrdrvmax_dn14_slot: &mut f64,
        var_rrdrvmax_dn2_slot: &mut f64,
        var_rrdrvmax_dn4_slot: &mut f64,
        var_rrdrvmax_dn5_slot: &mut f64,
        var_rrdrvmax_dn6_slot: &mut f64,
        var_rrdrvmax_dn7_slot: &mut f64,
        var_rrdrvmax_dn8_slot: &mut f64,
        var_rrdrvmax_dn9_slot: &mut f64,
        var_rrdrvmaxs_slot: &mut f64,
        var_rrdrvmaxs_dn0_slot: &mut f64,
        var_rrdrvmaxs_dn10_slot: &mut f64,
        var_rrdrvmaxs_dn11_slot: &mut f64,
        var_rrdrvmaxs_dn14_slot: &mut f64,
        var_rrdrvmaxs_dn2_slot: &mut f64,
        var_rrdrvmaxs_dn4_slot: &mut f64,
        var_rrdrvmaxs_dn5_slot: &mut f64,
        var_rrdrvmaxs_dn6_slot: &mut f64,
        var_rrdrvmaxs_dn7_slot: &mut f64,
        var_rrdrvmaxs_dn8_slot: &mut f64,
        var_rrdrvmaxs_dn9_slot: &mut f64,
        var_rsvde_slot: &mut f64,
        var_rsvde_dn0_slot: &mut f64,
        var_rsvde_dn10_slot: &mut f64,
        var_rsvde_dn11_slot: &mut f64,
        var_rsvde_dn14_slot: &mut f64,
        var_rsvde_dn2_slot: &mut f64,
        var_rsvde_dn4_slot: &mut f64,
        var_rsvde_dn5_slot: &mut f64,
        var_rsvde_dn6_slot: &mut f64,
        var_rsvde_dn7_slot: &mut f64,
        var_rsvde_dn8_slot: &mut f64,
        var_rsvde_dn9_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn14_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_dn9_slot: &mut f64,
        var_uc_rdrbb_slot: &mut f64,
        var_uc_rdrbb_dn0_slot: &mut f64,
        var_uc_rdrbb_dn10_slot: &mut f64,
        var_uc_rdrbb_dn11_slot: &mut f64,
        var_uc_rdrbb_dn14_slot: &mut f64,
        var_uc_rdrbb_dn2_slot: &mut f64,
        var_uc_rdrbb_dn4_slot: &mut f64,
        var_uc_rdrbb_dn5_slot: &mut f64,
        var_uc_rdrbb_dn6_slot: &mut f64,
        var_uc_rdrbb_dn7_slot: &mut f64,
        var_uc_rdrbb_dn8_slot: &mut f64,
        var_uc_rdrbb_dn9_slot: &mut f64,
        var_uc_rdrbb_s_slot: &mut f64,
        var_uc_rdrbb_s_dn0_slot: &mut f64,
        var_uc_rdrbb_s_dn10_slot: &mut f64,
        var_uc_rdrbb_s_dn11_slot: &mut f64,
        var_uc_rdrbb_s_dn14_slot: &mut f64,
        var_uc_rdrbb_s_dn2_slot: &mut f64,
        var_uc_rdrbb_s_dn4_slot: &mut f64,
        var_uc_rdrbb_s_dn5_slot: &mut f64,
        var_uc_rdrbb_s_dn6_slot: &mut f64,
        var_uc_rdrbb_s_dn7_slot: &mut f64,
        var_uc_rdrbb_s_dn8_slot: &mut f64,
        var_uc_rdrbb_s_dn9_slot: &mut f64,
        var_uc_subtmp_slot: &mut f64,
    ) {
        let mut var_costi0: f64 = *var_costi0_slot;
        let mut var_costi0_dn0: f64 = *var_costi0_dn0_slot;
        let mut var_costi0_dn10: f64 = *var_costi0_dn10_slot;
        let mut var_costi0_dn11: f64 = *var_costi0_dn11_slot;
        let mut var_costi0_dn14: f64 = *var_costi0_dn14_slot;
        let mut var_costi0_dn2: f64 = *var_costi0_dn2_slot;
        let mut var_costi0_dn4: f64 = *var_costi0_dn4_slot;
        let mut var_costi0_dn5: f64 = *var_costi0_dn5_slot;
        let mut var_costi0_dn6: f64 = *var_costi0_dn6_slot;
        let mut var_costi0_dn7: f64 = *var_costi0_dn7_slot;
        let mut var_costi0_dn8: f64 = *var_costi0_dn8_slot;
        let mut var_costi0_dn9: f64 = *var_costi0_dn9_slot;
        let mut var_costi0_p2: f64 = *var_costi0_p2_slot;
        let mut var_costi0_p2_dn0: f64 = *var_costi0_p2_dn0_slot;
        let mut var_costi0_p2_dn10: f64 = *var_costi0_p2_dn10_slot;
        let mut var_costi0_p2_dn11: f64 = *var_costi0_p2_dn11_slot;
        let mut var_costi0_p2_dn14: f64 = *var_costi0_p2_dn14_slot;
        let mut var_costi0_p2_dn2: f64 = *var_costi0_p2_dn2_slot;
        let mut var_costi0_p2_dn4: f64 = *var_costi0_p2_dn4_slot;
        let mut var_costi0_p2_dn5: f64 = *var_costi0_p2_dn5_slot;
        let mut var_costi0_p2_dn6: f64 = *var_costi0_p2_dn6_slot;
        let mut var_costi0_p2_dn7: f64 = *var_costi0_p2_dn7_slot;
        let mut var_costi0_p2_dn8: f64 = *var_costi0_p2_dn8_slot;
        let mut var_costi0_p2_dn9: f64 = *var_costi0_p2_dn9_slot;
        let mut var_costi1: f64 = *var_costi1_slot;
        let mut var_costi1_dn0: f64 = *var_costi1_dn0_slot;
        let mut var_costi1_dn10: f64 = *var_costi1_dn10_slot;
        let mut var_costi1_dn11: f64 = *var_costi1_dn11_slot;
        let mut var_costi1_dn14: f64 = *var_costi1_dn14_slot;
        let mut var_costi1_dn2: f64 = *var_costi1_dn2_slot;
        let mut var_costi1_dn4: f64 = *var_costi1_dn4_slot;
        let mut var_costi1_dn5: f64 = *var_costi1_dn5_slot;
        let mut var_costi1_dn6: f64 = *var_costi1_dn6_slot;
        let mut var_costi1_dn7: f64 = *var_costi1_dn7_slot;
        let mut var_costi1_dn8: f64 = *var_costi1_dn8_slot;
        let mut var_costi1_dn9: f64 = *var_costi1_dn9_slot;
        let mut var_guard328: f64 = *var_guard328_slot;
        let mut var_guard329: f64 = *var_guard329_slot;
        let mut var_guard330: f64 = *var_guard330_slot;
        let mut var_guard332: f64 = *var_guard332_slot;
        let mut var_hbdceff: f64 = *var_hbdceff_slot;
        let mut var_hbdceff_dn0: f64 = *var_hbdceff_dn0_slot;
        let mut var_hbdceff_dn10: f64 = *var_hbdceff_dn10_slot;
        let mut var_hbdceff_dn11: f64 = *var_hbdceff_dn11_slot;
        let mut var_hbdceff_dn14: f64 = *var_hbdceff_dn14_slot;
        let mut var_hbdceff_dn2: f64 = *var_hbdceff_dn2_slot;
        let mut var_hbdceff_dn4: f64 = *var_hbdceff_dn4_slot;
        let mut var_hbdceff_dn5: f64 = *var_hbdceff_dn5_slot;
        let mut var_hbdceff_dn6: f64 = *var_hbdceff_dn6_slot;
        let mut var_hbdceff_dn7: f64 = *var_hbdceff_dn7_slot;
        let mut var_hbdceff_dn8: f64 = *var_hbdceff_dn8_slot;
        let mut var_hbdceff_dn9: f64 = *var_hbdceff_dn9_slot;
        let mut var_rdvde: f64 = *var_rdvde_slot;
        let mut var_rdvde_dn0: f64 = *var_rdvde_dn0_slot;
        let mut var_rdvde_dn10: f64 = *var_rdvde_dn10_slot;
        let mut var_rdvde_dn11: f64 = *var_rdvde_dn11_slot;
        let mut var_rdvde_dn14: f64 = *var_rdvde_dn14_slot;
        let mut var_rdvde_dn2: f64 = *var_rdvde_dn2_slot;
        let mut var_rdvde_dn4: f64 = *var_rdvde_dn4_slot;
        let mut var_rdvde_dn5: f64 = *var_rdvde_dn5_slot;
        let mut var_rdvde_dn6: f64 = *var_rdvde_dn6_slot;
        let mut var_rdvde_dn7: f64 = *var_rdvde_dn7_slot;
        let mut var_rdvde_dn8: f64 = *var_rdvde_dn8_slot;
        let mut var_rdvde_dn9: f64 = *var_rdvde_dn9_slot;
        let mut var_rrdrmue: f64 = *var_rrdrmue_slot;
        let mut var_rrdrmue_dn0: f64 = *var_rrdrmue_dn0_slot;
        let mut var_rrdrmue_dn10: f64 = *var_rrdrmue_dn10_slot;
        let mut var_rrdrmue_dn11: f64 = *var_rrdrmue_dn11_slot;
        let mut var_rrdrmue_dn14: f64 = *var_rrdrmue_dn14_slot;
        let mut var_rrdrmue_dn2: f64 = *var_rrdrmue_dn2_slot;
        let mut var_rrdrmue_dn4: f64 = *var_rrdrmue_dn4_slot;
        let mut var_rrdrmue_dn5: f64 = *var_rrdrmue_dn5_slot;
        let mut var_rrdrmue_dn6: f64 = *var_rrdrmue_dn6_slot;
        let mut var_rrdrmue_dn7: f64 = *var_rrdrmue_dn7_slot;
        let mut var_rrdrmue_dn8: f64 = *var_rrdrmue_dn8_slot;
        let mut var_rrdrmue_dn9: f64 = *var_rrdrmue_dn9_slot;
        let mut var_rrdrmues: f64 = *var_rrdrmues_slot;
        let mut var_rrdrmues_dn0: f64 = *var_rrdrmues_dn0_slot;
        let mut var_rrdrmues_dn10: f64 = *var_rrdrmues_dn10_slot;
        let mut var_rrdrmues_dn11: f64 = *var_rrdrmues_dn11_slot;
        let mut var_rrdrmues_dn14: f64 = *var_rrdrmues_dn14_slot;
        let mut var_rrdrmues_dn2: f64 = *var_rrdrmues_dn2_slot;
        let mut var_rrdrmues_dn4: f64 = *var_rrdrmues_dn4_slot;
        let mut var_rrdrmues_dn5: f64 = *var_rrdrmues_dn5_slot;
        let mut var_rrdrmues_dn6: f64 = *var_rrdrmues_dn6_slot;
        let mut var_rrdrmues_dn7: f64 = *var_rrdrmues_dn7_slot;
        let mut var_rrdrmues_dn8: f64 = *var_rrdrmues_dn8_slot;
        let mut var_rrdrmues_dn9: f64 = *var_rrdrmues_dn9_slot;
        let mut var_rrdrvmax: f64 = *var_rrdrvmax_slot;
        let mut var_rrdrvmax_dn0: f64 = *var_rrdrvmax_dn0_slot;
        let mut var_rrdrvmax_dn10: f64 = *var_rrdrvmax_dn10_slot;
        let mut var_rrdrvmax_dn11: f64 = *var_rrdrvmax_dn11_slot;
        let mut var_rrdrvmax_dn14: f64 = *var_rrdrvmax_dn14_slot;
        let mut var_rrdrvmax_dn2: f64 = *var_rrdrvmax_dn2_slot;
        let mut var_rrdrvmax_dn4: f64 = *var_rrdrvmax_dn4_slot;
        let mut var_rrdrvmax_dn5: f64 = *var_rrdrvmax_dn5_slot;
        let mut var_rrdrvmax_dn6: f64 = *var_rrdrvmax_dn6_slot;
        let mut var_rrdrvmax_dn7: f64 = *var_rrdrvmax_dn7_slot;
        let mut var_rrdrvmax_dn8: f64 = *var_rrdrvmax_dn8_slot;
        let mut var_rrdrvmax_dn9: f64 = *var_rrdrvmax_dn9_slot;
        let mut var_rrdrvmaxs: f64 = *var_rrdrvmaxs_slot;
        let mut var_rrdrvmaxs_dn0: f64 = *var_rrdrvmaxs_dn0_slot;
        let mut var_rrdrvmaxs_dn10: f64 = *var_rrdrvmaxs_dn10_slot;
        let mut var_rrdrvmaxs_dn11: f64 = *var_rrdrvmaxs_dn11_slot;
        let mut var_rrdrvmaxs_dn14: f64 = *var_rrdrvmaxs_dn14_slot;
        let mut var_rrdrvmaxs_dn2: f64 = *var_rrdrvmaxs_dn2_slot;
        let mut var_rrdrvmaxs_dn4: f64 = *var_rrdrvmaxs_dn4_slot;
        let mut var_rrdrvmaxs_dn5: f64 = *var_rrdrvmaxs_dn5_slot;
        let mut var_rrdrvmaxs_dn6: f64 = *var_rrdrvmaxs_dn6_slot;
        let mut var_rrdrvmaxs_dn7: f64 = *var_rrdrvmaxs_dn7_slot;
        let mut var_rrdrvmaxs_dn8: f64 = *var_rrdrvmaxs_dn8_slot;
        let mut var_rrdrvmaxs_dn9: f64 = *var_rrdrvmaxs_dn9_slot;
        let mut var_rsvde: f64 = *var_rsvde_slot;
        let mut var_rsvde_dn0: f64 = *var_rsvde_dn0_slot;
        let mut var_rsvde_dn10: f64 = *var_rsvde_dn10_slot;
        let mut var_rsvde_dn11: f64 = *var_rsvde_dn11_slot;
        let mut var_rsvde_dn14: f64 = *var_rsvde_dn14_slot;
        let mut var_rsvde_dn2: f64 = *var_rsvde_dn2_slot;
        let mut var_rsvde_dn4: f64 = *var_rsvde_dn4_slot;
        let mut var_rsvde_dn5: f64 = *var_rsvde_dn5_slot;
        let mut var_rsvde_dn6: f64 = *var_rsvde_dn6_slot;
        let mut var_rsvde_dn7: f64 = *var_rsvde_dn7_slot;
        let mut var_rsvde_dn8: f64 = *var_rsvde_dn8_slot;
        let mut var_rsvde_dn9: f64 = *var_rsvde_dn9_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn14: f64 = *var_tmf2_dn14_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_dn9: f64 = *var_tmf2_dn9_slot;
        let mut var_uc_rdrbb: f64 = *var_uc_rdrbb_slot;
        let mut var_uc_rdrbb_dn0: f64 = *var_uc_rdrbb_dn0_slot;
        let mut var_uc_rdrbb_dn10: f64 = *var_uc_rdrbb_dn10_slot;
        let mut var_uc_rdrbb_dn11: f64 = *var_uc_rdrbb_dn11_slot;
        let mut var_uc_rdrbb_dn14: f64 = *var_uc_rdrbb_dn14_slot;
        let mut var_uc_rdrbb_dn2: f64 = *var_uc_rdrbb_dn2_slot;
        let mut var_uc_rdrbb_dn4: f64 = *var_uc_rdrbb_dn4_slot;
        let mut var_uc_rdrbb_dn5: f64 = *var_uc_rdrbb_dn5_slot;
        let mut var_uc_rdrbb_dn6: f64 = *var_uc_rdrbb_dn6_slot;
        let mut var_uc_rdrbb_dn7: f64 = *var_uc_rdrbb_dn7_slot;
        let mut var_uc_rdrbb_dn8: f64 = *var_uc_rdrbb_dn8_slot;
        let mut var_uc_rdrbb_dn9: f64 = *var_uc_rdrbb_dn9_slot;
        let mut var_uc_rdrbb_s: f64 = *var_uc_rdrbb_s_slot;
        let mut var_uc_rdrbb_s_dn0: f64 = *var_uc_rdrbb_s_dn0_slot;
        let mut var_uc_rdrbb_s_dn10: f64 = *var_uc_rdrbb_s_dn10_slot;
        let mut var_uc_rdrbb_s_dn11: f64 = *var_uc_rdrbb_s_dn11_slot;
        let mut var_uc_rdrbb_s_dn14: f64 = *var_uc_rdrbb_s_dn14_slot;
        let mut var_uc_rdrbb_s_dn2: f64 = *var_uc_rdrbb_s_dn2_slot;
        let mut var_uc_rdrbb_s_dn4: f64 = *var_uc_rdrbb_s_dn4_slot;
        let mut var_uc_rdrbb_s_dn5: f64 = *var_uc_rdrbb_s_dn5_slot;
        let mut var_uc_rdrbb_s_dn6: f64 = *var_uc_rdrbb_s_dn6_slot;
        let mut var_uc_rdrbb_s_dn7: f64 = *var_uc_rdrbb_s_dn7_slot;
        let mut var_uc_rdrbb_s_dn8: f64 = *var_uc_rdrbb_s_dn8_slot;
        let mut var_uc_rdrbb_s_dn9: f64 = *var_uc_rdrbb_s_dn9_slot;
        let mut var_uc_subtmp: f64 = *var_uc_subtmp_slot;

        let (assign15270_e10136, assign15270_e10136_d_n0, assign15270_e10136_d_n2, assign15270_e10136_d_n4, assign15270_e10136_d_n5, assign15270_e10136_d_n6, assign15270_e10136_d_n7, assign15270_e10136_d_n8, assign15270_e10136_d_n9, assign15270_e10136_d_n10, assign15270_e10136_d_n11, assign15270_e10136_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard325 == 0.0)) {
        let (assign15270_e10134, assign15270_e10134_d_n0, assign15270_e10134_d_n2, assign15270_e10134_d_n4, assign15270_e10134_d_n5, assign15270_e10134_d_n6, assign15270_e10134_d_n7, assign15270_e10134_d_n8, assign15270_e10134_d_n9, assign15270_e10134_d_n10, assign15270_e10134_d_n11, assign15270_e10134_d_n14,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
            } else {
                let assign15270_e10133: f64 = (-var_tmf2);
                (assign15270_e10133, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
            }
        };
        (assign15270_e10134, assign15270_e10134_d_n0, assign15270_e10134_d_n2, assign15270_e10134_d_n4, assign15270_e10134_d_n5, assign15270_e10134_d_n6, assign15270_e10134_d_n7, assign15270_e10134_d_n8, assign15270_e10134_d_n9, assign15270_e10134_d_n10, assign15270_e10134_d_n11, assign15270_e10134_d_n14,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign15270_e10136;
        var_tmf2_dn0 = assign15270_e10136_d_n0;
        var_tmf2_dn2 = assign15270_e10136_d_n2;
        var_tmf2_dn4 = assign15270_e10136_d_n4;
        var_tmf2_dn5 = assign15270_e10136_d_n5;
        var_tmf2_dn6 = assign15270_e10136_d_n6;
        var_tmf2_dn7 = assign15270_e10136_d_n7;
        var_tmf2_dn8 = assign15270_e10136_d_n8;
        var_tmf2_dn9 = assign15270_e10136_d_n9;
        var_tmf2_dn10 = assign15270_e10136_d_n10;
        var_tmf2_dn11 = assign15270_e10136_d_n11;
        var_tmf2_dn14 = assign15270_e10136_d_n14;

        let (assign15280_e10152, assign15280_e10152_d_n0, assign15280_e10152_d_n2, assign15280_e10152_d_n4, assign15280_e10152_d_n5, assign15280_e10152_d_n6, assign15280_e10152_d_n7, assign15280_e10152_d_n8, assign15280_e10152_d_n9, assign15280_e10152_d_n10, assign15280_e10152_d_n11, assign15280_e10152_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard325 == 0.0)) {
        let assign15280_e10147: f64 = (var_tmf1 * var_tmf1);
        let assign15280_e10149: f64 = (assign15280_e10147 + var_tmf2);
        let assign15280_e10150: f64 = (assign15280_e10149).sqrt();
        (assign15280_e10150, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign15280_e10150)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign15280_e10150)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign15280_e10150)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign15280_e10150)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign15280_e10150)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign15280_e10150)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign15280_e10150)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign15280_e10150)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign15280_e10150)), ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign15280_e10150)), ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign15280_e10150)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    }
};
        var_tmf2 = assign15280_e10152;
        var_tmf2_dn0 = assign15280_e10152_d_n0;
        var_tmf2_dn2 = assign15280_e10152_d_n2;
        var_tmf2_dn4 = assign15280_e10152_d_n4;
        var_tmf2_dn5 = assign15280_e10152_d_n5;
        var_tmf2_dn6 = assign15280_e10152_d_n6;
        var_tmf2_dn7 = assign15280_e10152_d_n7;
        var_tmf2_dn8 = assign15280_e10152_d_n8;
        var_tmf2_dn9 = assign15280_e10152_d_n9;
        var_tmf2_dn10 = assign15280_e10152_d_n10;
        var_tmf2_dn11 = assign15280_e10152_d_n11;
        var_tmf2_dn14 = assign15280_e10152_d_n14;

        let (assign15290_e10169, assign15290_e10169_d_n0, assign15290_e10169_d_n2, assign15290_e10169_d_n4, assign15290_e10169_d_n5, assign15290_e10169_d_n6, assign15290_e10169_d_n7, assign15290_e10169_d_n8, assign15290_e10169_d_n9, assign15290_e10169_d_n10, assign15290_e10169_d_n11, assign15290_e10169_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard325 == 0.0)) {
        let assign15290_e10165: f64 = (var_tmf1 / var_tmf2);
        let assign15290_e10166: f64 = (1.0 + assign15290_e10165);
        let assign15290_e10167: f64 = (0.5 * assign15290_e10166);
        (assign15290_e10167, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign15290_e10169;
        var_t0_dn0 = assign15290_e10169_d_n0;
        var_t0_dn2 = assign15290_e10169_d_n2;
        var_t0_dn4 = assign15290_e10169_d_n4;
        var_t0_dn5 = assign15290_e10169_d_n5;
        var_t0_dn6 = assign15290_e10169_d_n6;
        var_t0_dn7 = assign15290_e10169_d_n7;
        var_t0_dn8 = assign15290_e10169_d_n8;
        var_t0_dn9 = assign15290_e10169_d_n9;
        var_t0_dn10 = assign15290_e10169_d_n10;
        var_t0_dn11 = assign15290_e10169_d_n11;
        var_t0_dn14 = assign15290_e10169_d_n14;

        let (assign15300_e10188, assign15300_e10188_d_n0, assign15300_e10188_d_n2, assign15300_e10188_d_n4, assign15300_e10188_d_n5, assign15300_e10188_d_n6, assign15300_e10188_d_n7, assign15300_e10188_d_n8, assign15300_e10188_d_n9, assign15300_e10188_d_n10, assign15300_e10188_d_n11, assign15300_e10188_d_n14,) = {
    if ((((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 != 0.0)) && (var_guard325 == 0.0)) {
        let assign15300_e10180: f64 = (0.005 * var_uc_rdvd);
        let assign15300_e10184: f64 = (var_tmf1 + var_tmf2);
        let assign15300_e10185: f64 = (0.5 * assign15300_e10184);
        let assign15300_e10186: f64 = (assign15300_e10180 + assign15300_e10185);
        (assign15300_e10186, (0.5 * (var_tmf1_dn0 + var_tmf2_dn0)), (0.5 * (var_tmf1_dn2 + var_tmf2_dn2)), (0.5 * (var_tmf1_dn4 + var_tmf2_dn4)), (0.5 * (var_tmf1_dn5 + var_tmf2_dn5)), (0.5 * (var_tmf1_dn6 + var_tmf2_dn6)), (0.5 * (var_tmf1_dn7 + var_tmf2_dn7)), (0.5 * (var_tmf1_dn8 + var_tmf2_dn8)), (0.5 * (var_tmf1_dn9 + var_tmf2_dn9)), (0.5 * (var_tmf1_dn10 + var_tmf2_dn10)), (0.5 * (var_tmf1_dn11 + var_tmf2_dn11)), (0.5 * (var_tmf1_dn14 + var_tmf2_dn14)),)
    } else {
        (var_rsvde, var_rsvde_dn0, var_rsvde_dn2, var_rsvde_dn4, var_rsvde_dn5, var_rsvde_dn6, var_rsvde_dn7, var_rsvde_dn8, var_rsvde_dn9, var_rsvde_dn10, var_rsvde_dn11, var_rsvde_dn14,)
    }
};
        var_rsvde = assign15300_e10188;
        var_rsvde_dn0 = assign15300_e10188_d_n0;
        var_rsvde_dn2 = assign15300_e10188_d_n2;
        var_rsvde_dn4 = assign15300_e10188_d_n4;
        var_rsvde_dn5 = assign15300_e10188_d_n5;
        var_rsvde_dn6 = assign15300_e10188_d_n6;
        var_rsvde_dn7 = assign15300_e10188_d_n7;
        var_rsvde_dn8 = assign15300_e10188_d_n8;
        var_rsvde_dn9 = assign15300_e10188_d_n9;
        var_rsvde_dn10 = assign15300_e10188_d_n10;
        var_rsvde_dn11 = assign15300_e10188_d_n11;
        var_rsvde_dn14 = assign15300_e10188_d_n14;

        let (assign15310_e10197, assign15310_e10197_d_n0, assign15310_e10197_d_n2, assign15310_e10197_d_n4, assign15310_e10197_d_n5, assign15310_e10197_d_n6, assign15310_e10197_d_n7, assign15310_e10197_d_n8, assign15310_e10197_d_n9, assign15310_e10197_d_n10, assign15310_e10197_d_n11, assign15310_e10197_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdvde, var_rdvde_dn0, var_rdvde_dn2, var_rdvde_dn4, var_rdvde_dn5, var_rdvde_dn6, var_rdvde_dn7, var_rdvde_dn8, var_rdvde_dn9, var_rdvde_dn10, var_rdvde_dn11, var_rdvde_dn14,)
    }
};
        var_rdvde = assign15310_e10197;
        var_rdvde_dn0 = assign15310_e10197_d_n0;
        var_rdvde_dn2 = assign15310_e10197_d_n2;
        var_rdvde_dn4 = assign15310_e10197_d_n4;
        var_rdvde_dn5 = assign15310_e10197_d_n5;
        var_rdvde_dn6 = assign15310_e10197_d_n6;
        var_rdvde_dn7 = assign15310_e10197_d_n7;
        var_rdvde_dn8 = assign15310_e10197_d_n8;
        var_rdvde_dn9 = assign15310_e10197_d_n9;
        var_rdvde_dn10 = assign15310_e10197_d_n10;
        var_rdvde_dn11 = assign15310_e10197_d_n11;
        var_rdvde_dn14 = assign15310_e10197_d_n14;

        let (assign15320_e10206, assign15320_e10206_d_n0, assign15320_e10206_d_n2, assign15320_e10206_d_n4, assign15320_e10206_d_n5, assign15320_e10206_d_n6, assign15320_e10206_d_n7, assign15320_e10206_d_n8, assign15320_e10206_d_n9, assign15320_e10206_d_n10, assign15320_e10206_d_n11, assign15320_e10206_d_n14,) = {
    if (((var_guard293 != 0.0) && (var_guard317 != 0.0)) && (var_guard322 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rsvde, var_rsvde_dn0, var_rsvde_dn2, var_rsvde_dn4, var_rsvde_dn5, var_rsvde_dn6, var_rsvde_dn7, var_rsvde_dn8, var_rsvde_dn9, var_rsvde_dn10, var_rsvde_dn11, var_rsvde_dn14,)
    }
};
        var_rsvde = assign15320_e10206;
        var_rsvde_dn0 = assign15320_e10206_d_n0;
        var_rsvde_dn2 = assign15320_e10206_d_n2;
        var_rsvde_dn4 = assign15320_e10206_d_n4;
        var_rsvde_dn5 = assign15320_e10206_d_n5;
        var_rsvde_dn6 = assign15320_e10206_d_n6;
        var_rsvde_dn7 = assign15320_e10206_d_n7;
        var_rsvde_dn8 = assign15320_e10206_d_n8;
        var_rsvde_dn9 = assign15320_e10206_d_n9;
        var_rsvde_dn10 = assign15320_e10206_d_n10;
        var_rsvde_dn11 = assign15320_e10206_d_n11;
        var_rsvde_dn14 = assign15320_e10206_d_n14;

        let (assign15330_e10213, assign15330_e10213_d_n0, assign15330_e10213_d_n2, assign15330_e10213_d_n4, assign15330_e10213_d_n5, assign15330_e10213_d_n6, assign15330_e10213_d_n7, assign15330_e10213_d_n8, assign15330_e10213_d_n9, assign15330_e10213_d_n10, assign15330_e10213_d_n11, assign15330_e10213_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign15330_e10210: f64 = (var_beta_inv).sqrt();
        let assign15330_e10211: f64 = (var_costi00 * assign15330_e10210);
        (assign15330_e10211, (var_costi00 * (var_beta_inv_dn0 / (2.0 * assign15330_e10210))), (var_costi00 * (var_beta_inv_dn2 / (2.0 * assign15330_e10210))), (var_costi00 * (var_beta_inv_dn4 / (2.0 * assign15330_e10210))), (var_costi00 * (var_beta_inv_dn5 / (2.0 * assign15330_e10210))), (var_costi00 * (var_beta_inv_dn6 / (2.0 * assign15330_e10210))), (var_costi00 * (var_beta_inv_dn7 / (2.0 * assign15330_e10210))), (var_costi00 * (var_beta_inv_dn8 / (2.0 * assign15330_e10210))), (var_costi00 * (var_beta_inv_dn9 / (2.0 * assign15330_e10210))), (var_costi00 * (var_beta_inv_dn10 / (2.0 * assign15330_e10210))), (var_costi00 * (var_beta_inv_dn11 / (2.0 * assign15330_e10210))), (var_costi00 * (var_beta_inv_dn14 / (2.0 * assign15330_e10210))),)
    } else {
        (var_costi0, var_costi0_dn0, var_costi0_dn2, var_costi0_dn4, var_costi0_dn5, var_costi0_dn6, var_costi0_dn7, var_costi0_dn8, var_costi0_dn9, var_costi0_dn10, var_costi0_dn11, var_costi0_dn14,)
    }
};
        var_costi0 = assign15330_e10213;
        var_costi0_dn0 = assign15330_e10213_d_n0;
        var_costi0_dn2 = assign15330_e10213_d_n2;
        var_costi0_dn4 = assign15330_e10213_d_n4;
        var_costi0_dn5 = assign15330_e10213_d_n5;
        var_costi0_dn6 = assign15330_e10213_d_n6;
        var_costi0_dn7 = assign15330_e10213_d_n7;
        var_costi0_dn8 = assign15330_e10213_d_n8;
        var_costi0_dn9 = assign15330_e10213_d_n9;
        var_costi0_dn10 = assign15330_e10213_d_n10;
        var_costi0_dn11 = assign15330_e10213_d_n11;
        var_costi0_dn14 = assign15330_e10213_d_n14;

        let (assign15340_e10219, assign15340_e10219_d_n0, assign15340_e10219_d_n2, assign15340_e10219_d_n4, assign15340_e10219_d_n5, assign15340_e10219_d_n6, assign15340_e10219_d_n7, assign15340_e10219_d_n8, assign15340_e10219_d_n9, assign15340_e10219_d_n10, assign15340_e10219_d_n11, assign15340_e10219_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign15340_e10217: f64 = (var_costi0 * var_costi0);
        (assign15340_e10217, ((var_costi0_dn0 * var_costi0) + (var_costi0 * var_costi0_dn0)), ((var_costi0_dn2 * var_costi0) + (var_costi0 * var_costi0_dn2)), ((var_costi0_dn4 * var_costi0) + (var_costi0 * var_costi0_dn4)), ((var_costi0_dn5 * var_costi0) + (var_costi0 * var_costi0_dn5)), ((var_costi0_dn6 * var_costi0) + (var_costi0 * var_costi0_dn6)), ((var_costi0_dn7 * var_costi0) + (var_costi0 * var_costi0_dn7)), ((var_costi0_dn8 * var_costi0) + (var_costi0 * var_costi0_dn8)), ((var_costi0_dn9 * var_costi0) + (var_costi0 * var_costi0_dn9)), ((var_costi0_dn10 * var_costi0) + (var_costi0 * var_costi0_dn10)), ((var_costi0_dn11 * var_costi0) + (var_costi0 * var_costi0_dn11)), ((var_costi0_dn14 * var_costi0) + (var_costi0 * var_costi0_dn14)),)
    } else {
        (var_costi0_p2, var_costi0_p2_dn0, var_costi0_p2_dn2, var_costi0_p2_dn4, var_costi0_p2_dn5, var_costi0_p2_dn6, var_costi0_p2_dn7, var_costi0_p2_dn8, var_costi0_p2_dn9, var_costi0_p2_dn10, var_costi0_p2_dn11, var_costi0_p2_dn14,)
    }
};
        var_costi0_p2 = assign15340_e10219;
        var_costi0_p2_dn0 = assign15340_e10219_d_n0;
        var_costi0_p2_dn2 = assign15340_e10219_d_n2;
        var_costi0_p2_dn4 = assign15340_e10219_d_n4;
        var_costi0_p2_dn5 = assign15340_e10219_d_n5;
        var_costi0_p2_dn6 = assign15340_e10219_d_n6;
        var_costi0_p2_dn7 = assign15340_e10219_d_n7;
        var_costi0_p2_dn8 = assign15340_e10219_d_n8;
        var_costi0_p2_dn9 = assign15340_e10219_d_n9;
        var_costi0_p2_dn10 = assign15340_e10219_d_n10;
        var_costi0_p2_dn11 = assign15340_e10219_d_n11;
        var_costi0_p2_dn14 = assign15340_e10219_d_n14;

        let (assign15350_e10227, assign15350_e10227_d_n0, assign15350_e10227_d_n2, assign15350_e10227_d_n4, assign15350_e10227_d_n5, assign15350_e10227_d_n6, assign15350_e10227_d_n7, assign15350_e10227_d_n8, assign15350_e10227_d_n9, assign15350_e10227_d_n10, assign15350_e10227_d_n11, assign15350_e10227_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign15350_e10223: f64 = (var_nin * var_nin);
        let assign15350_e10225: f64 = (assign15350_e10223 * var_nsti_p2);
        (assign15350_e10225, (((var_nin_dn0 * var_nin) + (var_nin * var_nin_dn0)) * var_nsti_p2), (((var_nin_dn2 * var_nin) + (var_nin * var_nin_dn2)) * var_nsti_p2), (((var_nin_dn4 * var_nin) + (var_nin * var_nin_dn4)) * var_nsti_p2), (((var_nin_dn5 * var_nin) + (var_nin * var_nin_dn5)) * var_nsti_p2), (((var_nin_dn6 * var_nin) + (var_nin * var_nin_dn6)) * var_nsti_p2), (((var_nin_dn7 * var_nin) + (var_nin * var_nin_dn7)) * var_nsti_p2), (((var_nin_dn8 * var_nin) + (var_nin * var_nin_dn8)) * var_nsti_p2), (((var_nin_dn9 * var_nin) + (var_nin * var_nin_dn9)) * var_nsti_p2), (((var_nin_dn10 * var_nin) + (var_nin * var_nin_dn10)) * var_nsti_p2), (((var_nin_dn11 * var_nin) + (var_nin * var_nin_dn11)) * var_nsti_p2), (((var_nin_dn14 * var_nin) + (var_nin * var_nin_dn14)) * var_nsti_p2),)
    } else {
        (var_costi1, var_costi1_dn0, var_costi1_dn2, var_costi1_dn4, var_costi1_dn5, var_costi1_dn6, var_costi1_dn7, var_costi1_dn8, var_costi1_dn9, var_costi1_dn10, var_costi1_dn11, var_costi1_dn14,)
    }
};
        var_costi1 = assign15350_e10227;
        var_costi1_dn0 = assign15350_e10227_d_n0;
        var_costi1_dn2 = assign15350_e10227_d_n2;
        var_costi1_dn4 = assign15350_e10227_d_n4;
        var_costi1_dn5 = assign15350_e10227_d_n5;
        var_costi1_dn6 = assign15350_e10227_d_n6;
        var_costi1_dn7 = assign15350_e10227_d_n7;
        var_costi1_dn8 = assign15350_e10227_d_n8;
        var_costi1_dn9 = assign15350_e10227_d_n9;
        var_costi1_dn10 = assign15350_e10227_d_n10;
        var_costi1_dn11 = assign15350_e10227_d_n11;
        var_costi1_dn14 = assign15350_e10227_d_n14;

        let (assign15360_e10235, assign15360_e10235_d_n0, assign15360_e10235_d_n2, assign15360_e10235_d_n4, assign15360_e10235_d_n5, assign15360_e10235_d_n6, assign15360_e10235_d_n7, assign15360_e10235_d_n8, assign15360_e10235_d_n9, assign15360_e10235_d_n10, assign15360_e10235_d_n11, assign15360_e10235_d_n14,) = {
    if (var_guard293 != 0.0) {
        let assign15360_e10232: f64 = (p.p448 * var_tdiff);
        let assign15360_e10233: f64 = (p.p447 + assign15360_e10232);
        (assign15360_e10233, (p.p448 * var_tdiff_dn0), (p.p448 * var_tdiff_dn2), (p.p448 * var_tdiff_dn4), (p.p448 * var_tdiff_dn5), (p.p448 * var_tdiff_dn6), (p.p448 * var_tdiff_dn7), (p.p448 * var_tdiff_dn8), (p.p448 * var_tdiff_dn9), (p.p448 * var_tdiff_dn10), (p.p448 * var_tdiff_dn11), (p.p448 * var_tdiff_dn14),)
    } else {
        (var_hbdceff, var_hbdceff_dn0, var_hbdceff_dn2, var_hbdceff_dn4, var_hbdceff_dn5, var_hbdceff_dn6, var_hbdceff_dn7, var_hbdceff_dn8, var_hbdceff_dn9, var_hbdceff_dn10, var_hbdceff_dn11, var_hbdceff_dn14,)
    }
};
        var_hbdceff = assign15360_e10235;
        var_hbdceff_dn0 = assign15360_e10235_d_n0;
        var_hbdceff_dn2 = assign15360_e10235_d_n2;
        var_hbdceff_dn4 = assign15360_e10235_d_n4;
        var_hbdceff_dn5 = assign15360_e10235_d_n5;
        var_hbdceff_dn6 = assign15360_e10235_d_n6;
        var_hbdceff_dn7 = assign15360_e10235_d_n7;
        var_hbdceff_dn8 = assign15360_e10235_d_n8;
        var_hbdceff_dn9 = assign15360_e10235_d_n9;
        var_hbdceff_dn10 = assign15360_e10235_d_n10;
        var_hbdceff_dn11 = assign15360_e10235_d_n11;
        var_hbdceff_dn14 = assign15360_e10235_d_n14;

        let (assign15370_e10239,) = {
    if (var_guard293 != 0.0) {
        (p.p193,)
    } else {
        (var_uc_subtmp,)
    }
};
        var_uc_subtmp = assign15370_e10239;

        let assign15400_e10252: f64 = if var_uc_subtmp < 0.0 { 1.0 } else { 0.0 };
        var_guard328 = assign15400_e10252;

        let (assign15410_e10258,) = {
    if ((var_guard293 != 0.0) && (var_guard328 != 0.0)) {
        (0.0,)
    } else {
        (var_uc_subtmp,)
    }
};
        var_uc_subtmp = assign15410_e10258;

        let assign15420_e10261: f64 = if var_uc_subtmp > 0.005 { 1.0 } else { 0.0 };
        var_guard329 = assign15420_e10261;

        let (assign15430_e10267,) = {
    if ((var_guard293 != 0.0) && (var_guard329 != 0.0)) {
        (0.005,)
    } else {
        (var_uc_subtmp,)
    }
};
        var_uc_subtmp = assign15430_e10267;

        let assign15440_e10270: f64 = if var_uc_cordrift > 0.0 { 1.0 } else { 0.0 };
        var_guard330 = assign15440_e10270;

        let (assign15450_e10283, assign15450_e10283_d_n0, assign15450_e10283_d_n2, assign15450_e10283_d_n4, assign15450_e10283_d_n5, assign15450_e10283_d_n6, assign15450_e10283_d_n7, assign15450_e10283_d_n8, assign15450_e10283_d_n9, assign15450_e10283_d_n10, assign15450_e10283_d_n11, assign15450_e10283_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard330 != 0.0)) {
        let (assign15450_e10281, assign15450_e10281_d_n0, assign15450_e10281_d_n2, assign15450_e10281_d_n4, assign15450_e10281_d_n5, assign15450_e10281_d_n6, assign15450_e10281_d_n7, assign15450_e10281_d_n8, assign15450_e10281_d_n9, assign15450_e10281_d_n10, assign15450_e10281_d_n11, assign15450_e10281_d_n14,) = {
            if (var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign15450_e10280: f64 = (var_tratio).powf(p.p416);
                (assign15450_e10280, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((var_tratio).powf(p.p416 - 1.0) * var_tratio_dn0)) } } else { (assign15450_e10280 * (p.p416 * (var_tratio_dn0 / var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((var_tratio).powf(p.p416 - 1.0) * var_tratio_dn2)) } } else { (assign15450_e10280 * (p.p416 * (var_tratio_dn2 / var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((var_tratio).powf(p.p416 - 1.0) * var_tratio_dn4)) } } else { (assign15450_e10280 * (p.p416 * (var_tratio_dn4 / var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((var_tratio).powf(p.p416 - 1.0) * var_tratio_dn5)) } } else { (assign15450_e10280 * (p.p416 * (var_tratio_dn5 / var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((var_tratio).powf(p.p416 - 1.0) * var_tratio_dn6)) } } else { (assign15450_e10280 * (p.p416 * (var_tratio_dn6 / var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((var_tratio).powf(p.p416 - 1.0) * var_tratio_dn7)) } } else { (assign15450_e10280 * (p.p416 * (var_tratio_dn7 / var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((var_tratio).powf(p.p416 - 1.0) * var_tratio_dn8)) } } else { (assign15450_e10280 * (p.p416 * (var_tratio_dn8 / var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((var_tratio).powf(p.p416 - 1.0) * var_tratio_dn9)) } } else { (assign15450_e10280 * (p.p416 * (var_tratio_dn9 / var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((var_tratio).powf(p.p416 - 1.0) * var_tratio_dn10)) } } else { (assign15450_e10280 * (p.p416 * (var_tratio_dn10 / var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((var_tratio).powf(p.p416 - 1.0) * var_tratio_dn11)) } } else { (assign15450_e10280 * (p.p416 * (var_tratio_dn11 / var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((var_tratio).powf(p.p416 - 1.0) * var_tratio_dn14)) } } else { (assign15450_e10280 * (p.p416 * (var_tratio_dn14 / var_tratio))) },)
            }
        };
        (assign15450_e10281, assign15450_e10281_d_n0, assign15450_e10281_d_n2, assign15450_e10281_d_n4, assign15450_e10281_d_n5, assign15450_e10281_d_n6, assign15450_e10281_d_n7, assign15450_e10281_d_n8, assign15450_e10281_d_n9, assign15450_e10281_d_n10, assign15450_e10281_d_n11, assign15450_e10281_d_n14,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign15450_e10283;
        var_t1_dn0 = assign15450_e10283_d_n0;
        var_t1_dn2 = assign15450_e10283_d_n2;
        var_t1_dn4 = assign15450_e10283_d_n4;
        var_t1_dn5 = assign15450_e10283_d_n5;
        var_t1_dn6 = assign15450_e10283_d_n6;
        var_t1_dn7 = assign15450_e10283_d_n7;
        var_t1_dn8 = assign15450_e10283_d_n8;
        var_t1_dn9 = assign15450_e10283_d_n9;
        var_t1_dn10 = assign15450_e10283_d_n10;
        var_t1_dn11 = assign15450_e10283_d_n11;
        var_t1_dn14 = assign15450_e10283_d_n14;

        let (assign15460_e10291, assign15460_e10291_d_n0, assign15460_e10291_d_n2, assign15460_e10291_d_n4, assign15460_e10291_d_n5, assign15460_e10291_d_n6, assign15460_e10291_d_n7, assign15460_e10291_d_n8, assign15460_e10291_d_n9, assign15460_e10291_d_n10, assign15460_e10291_d_n11, assign15460_e10291_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard330 != 0.0)) {
        let assign15460_e10289: f64 = (var_mks_rdrmues / var_t1);
        (assign15460_e10289, (-((var_mks_rdrmues * var_t1_dn0) / (var_t1 * var_t1))), (-((var_mks_rdrmues * var_t1_dn2) / (var_t1 * var_t1))), (-((var_mks_rdrmues * var_t1_dn4) / (var_t1 * var_t1))), (-((var_mks_rdrmues * var_t1_dn5) / (var_t1 * var_t1))), (-((var_mks_rdrmues * var_t1_dn6) / (var_t1 * var_t1))), (-((var_mks_rdrmues * var_t1_dn7) / (var_t1 * var_t1))), (-((var_mks_rdrmues * var_t1_dn8) / (var_t1 * var_t1))), (-((var_mks_rdrmues * var_t1_dn9) / (var_t1 * var_t1))), (-((var_mks_rdrmues * var_t1_dn10) / (var_t1 * var_t1))), (-((var_mks_rdrmues * var_t1_dn11) / (var_t1 * var_t1))), (-((var_mks_rdrmues * var_t1_dn14) / (var_t1 * var_t1))),)
    } else {
        (var_rrdrmues, var_rrdrmues_dn0, var_rrdrmues_dn2, var_rrdrmues_dn4, var_rrdrmues_dn5, var_rrdrmues_dn6, var_rrdrmues_dn7, var_rrdrmues_dn8, var_rrdrmues_dn9, var_rrdrmues_dn10, var_rrdrmues_dn11, var_rrdrmues_dn14,)
    }
};
        var_rrdrmues = assign15460_e10291;
        var_rrdrmues_dn0 = assign15460_e10291_d_n0;
        var_rrdrmues_dn2 = assign15460_e10291_d_n2;
        var_rrdrmues_dn4 = assign15460_e10291_d_n4;
        var_rrdrmues_dn5 = assign15460_e10291_d_n5;
        var_rrdrmues_dn6 = assign15460_e10291_d_n6;
        var_rrdrmues_dn7 = assign15460_e10291_d_n7;
        var_rrdrmues_dn8 = assign15460_e10291_d_n8;
        var_rrdrmues_dn9 = assign15460_e10291_d_n9;
        var_rrdrmues_dn10 = assign15460_e10291_d_n10;
        var_rrdrmues_dn11 = assign15460_e10291_d_n11;
        var_rrdrmues_dn14 = assign15460_e10291_d_n14;

        let (assign15470_e10313, assign15470_e10313_d_n0, assign15470_e10313_d_n2, assign15470_e10313_d_n4, assign15470_e10313_d_n5, assign15470_e10313_d_n6, assign15470_e10313_d_n7, assign15470_e10313_d_n8, assign15470_e10313_d_n9, assign15470_e10313_d_n10, assign15470_e10313_d_n11, assign15470_e10313_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard330 != 0.0)) {
        let assign15470_e10298: f64 = (0.4 * var_tratio);
        let assign15470_e10299: f64 = (1.8 + assign15470_e10298);
        let assign15470_e10302: f64 = (0.1 * var_tratio);
        let assign15470_e10304: f64 = (assign15470_e10302 * var_tratio);
        let assign15470_e10305: f64 = (assign15470_e10299 + assign15470_e10304);
        let assign15470_e10309: f64 = (1.0 - var_tratio);
        let assign15470_e10310: f64 = (p.p418 * assign15470_e10309);
        let assign15470_e10311: f64 = (assign15470_e10305 - assign15470_e10310);
        (assign15470_e10311, (((0.4 * var_tratio_dn0) + (((0.1 * var_tratio_dn0) * var_tratio) + (assign15470_e10302 * var_tratio_dn0))) - (p.p418 * (-var_tratio_dn0))), (((0.4 * var_tratio_dn2) + (((0.1 * var_tratio_dn2) * var_tratio) + (assign15470_e10302 * var_tratio_dn2))) - (p.p418 * (-var_tratio_dn2))), (((0.4 * var_tratio_dn4) + (((0.1 * var_tratio_dn4) * var_tratio) + (assign15470_e10302 * var_tratio_dn4))) - (p.p418 * (-var_tratio_dn4))), (((0.4 * var_tratio_dn5) + (((0.1 * var_tratio_dn5) * var_tratio) + (assign15470_e10302 * var_tratio_dn5))) - (p.p418 * (-var_tratio_dn5))), (((0.4 * var_tratio_dn6) + (((0.1 * var_tratio_dn6) * var_tratio) + (assign15470_e10302 * var_tratio_dn6))) - (p.p418 * (-var_tratio_dn6))), (((0.4 * var_tratio_dn7) + (((0.1 * var_tratio_dn7) * var_tratio) + (assign15470_e10302 * var_tratio_dn7))) - (p.p418 * (-var_tratio_dn7))), (((0.4 * var_tratio_dn8) + (((0.1 * var_tratio_dn8) * var_tratio) + (assign15470_e10302 * var_tratio_dn8))) - (p.p418 * (-var_tratio_dn8))), (((0.4 * var_tratio_dn9) + (((0.1 * var_tratio_dn9) * var_tratio) + (assign15470_e10302 * var_tratio_dn9))) - (p.p418 * (-var_tratio_dn9))), (((0.4 * var_tratio_dn10) + (((0.1 * var_tratio_dn10) * var_tratio) + (assign15470_e10302 * var_tratio_dn10))) - (p.p418 * (-var_tratio_dn10))), (((0.4 * var_tratio_dn11) + (((0.1 * var_tratio_dn11) * var_tratio) + (assign15470_e10302 * var_tratio_dn11))) - (p.p418 * (-var_tratio_dn11))), (((0.4 * var_tratio_dn14) + (((0.1 * var_tratio_dn14) * var_tratio) + (assign15470_e10302 * var_tratio_dn14))) - (p.p418 * (-var_tratio_dn14))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign15470_e10313;
        var_t0_dn0 = assign15470_e10313_d_n0;
        var_t0_dn2 = assign15470_e10313_d_n2;
        var_t0_dn4 = assign15470_e10313_d_n4;
        var_t0_dn5 = assign15470_e10313_d_n5;
        var_t0_dn6 = assign15470_e10313_d_n6;
        var_t0_dn7 = assign15470_e10313_d_n7;
        var_t0_dn8 = assign15470_e10313_d_n8;
        var_t0_dn9 = assign15470_e10313_d_n9;
        var_t0_dn10 = assign15470_e10313_d_n10;
        var_t0_dn11 = assign15470_e10313_d_n11;
        var_t0_dn14 = assign15470_e10313_d_n14;

        let (assign15480_e10321, assign15480_e10321_d_n0, assign15480_e10321_d_n2, assign15480_e10321_d_n4, assign15480_e10321_d_n5, assign15480_e10321_d_n6, assign15480_e10321_d_n7, assign15480_e10321_d_n8, assign15480_e10321_d_n9, assign15480_e10321_d_n10, assign15480_e10321_d_n11, assign15480_e10321_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard330 != 0.0)) {
        let assign15480_e10319: f64 = (var_mks_rdrvmaxs / var_t0);
        (assign15480_e10319, (-((var_mks_rdrvmaxs * var_t0_dn0) / (var_t0 * var_t0))), (-((var_mks_rdrvmaxs * var_t0_dn2) / (var_t0 * var_t0))), (-((var_mks_rdrvmaxs * var_t0_dn4) / (var_t0 * var_t0))), (-((var_mks_rdrvmaxs * var_t0_dn5) / (var_t0 * var_t0))), (-((var_mks_rdrvmaxs * var_t0_dn6) / (var_t0 * var_t0))), (-((var_mks_rdrvmaxs * var_t0_dn7) / (var_t0 * var_t0))), (-((var_mks_rdrvmaxs * var_t0_dn8) / (var_t0 * var_t0))), (-((var_mks_rdrvmaxs * var_t0_dn9) / (var_t0 * var_t0))), (-((var_mks_rdrvmaxs * var_t0_dn10) / (var_t0 * var_t0))), (-((var_mks_rdrvmaxs * var_t0_dn11) / (var_t0 * var_t0))), (-((var_mks_rdrvmaxs * var_t0_dn14) / (var_t0 * var_t0))),)
    } else {
        (var_rrdrvmaxs, var_rrdrvmaxs_dn0, var_rrdrvmaxs_dn2, var_rrdrvmaxs_dn4, var_rrdrvmaxs_dn5, var_rrdrvmaxs_dn6, var_rrdrvmaxs_dn7, var_rrdrvmaxs_dn8, var_rrdrvmaxs_dn9, var_rrdrvmaxs_dn10, var_rrdrvmaxs_dn11, var_rrdrvmaxs_dn14,)
    }
};
        var_rrdrvmaxs = assign15480_e10321;
        var_rrdrvmaxs_dn0 = assign15480_e10321_d_n0;
        var_rrdrvmaxs_dn2 = assign15480_e10321_d_n2;
        var_rrdrvmaxs_dn4 = assign15480_e10321_d_n4;
        var_rrdrvmaxs_dn5 = assign15480_e10321_d_n5;
        var_rrdrvmaxs_dn6 = assign15480_e10321_d_n6;
        var_rrdrvmaxs_dn7 = assign15480_e10321_d_n7;
        var_rrdrvmaxs_dn8 = assign15480_e10321_d_n8;
        var_rrdrvmaxs_dn9 = assign15480_e10321_d_n9;
        var_rrdrvmaxs_dn10 = assign15480_e10321_d_n10;
        var_rrdrvmaxs_dn11 = assign15480_e10321_d_n11;
        var_rrdrvmaxs_dn14 = assign15480_e10321_d_n14;

        let (assign15490_e10333, assign15490_e10333_d_n0, assign15490_e10333_d_n2, assign15490_e10333_d_n4, assign15490_e10333_d_n5, assign15490_e10333_d_n6, assign15490_e10333_d_n7, assign15490_e10333_d_n8, assign15490_e10333_d_n9, assign15490_e10333_d_n10, assign15490_e10333_d_n11, assign15490_e10333_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard330 != 0.0)) {
        let assign15490_e10329: f64 = (var_ttemp - var_ktnom);
        let assign15490_e10330: f64 = (p.p439 * assign15490_e10329);
        let assign15490_e10331: f64 = (var_uc_rdrbb_s + assign15490_e10330);
        (assign15490_e10331, (var_uc_rdrbb_s_dn0 + (p.p439 * var_ttemp_dn0)), (var_uc_rdrbb_s_dn2 + (p.p439 * var_ttemp_dn2)), (var_uc_rdrbb_s_dn4 + (p.p439 * var_ttemp_dn4)), (var_uc_rdrbb_s_dn5 + (p.p439 * var_ttemp_dn5)), (var_uc_rdrbb_s_dn6 + (p.p439 * var_ttemp_dn6)), (var_uc_rdrbb_s_dn7 + (p.p439 * var_ttemp_dn7)), (var_uc_rdrbb_s_dn8 + (p.p439 * var_ttemp_dn8)), (var_uc_rdrbb_s_dn9 + (p.p439 * var_ttemp_dn9)), (var_uc_rdrbb_s_dn10 + (p.p439 * var_ttemp_dn10)), (var_uc_rdrbb_s_dn11 + (p.p439 * var_ttemp_dn11)), (var_uc_rdrbb_s_dn14 + (p.p439 * var_ttemp_dn14)),)
    } else {
        (var_uc_rdrbb_s, var_uc_rdrbb_s_dn0, var_uc_rdrbb_s_dn2, var_uc_rdrbb_s_dn4, var_uc_rdrbb_s_dn5, var_uc_rdrbb_s_dn6, var_uc_rdrbb_s_dn7, var_uc_rdrbb_s_dn8, var_uc_rdrbb_s_dn9, var_uc_rdrbb_s_dn10, var_uc_rdrbb_s_dn11, var_uc_rdrbb_s_dn14,)
    }
};
        var_uc_rdrbb_s = assign15490_e10333;
        var_uc_rdrbb_s_dn0 = assign15490_e10333_d_n0;
        var_uc_rdrbb_s_dn2 = assign15490_e10333_d_n2;
        var_uc_rdrbb_s_dn4 = assign15490_e10333_d_n4;
        var_uc_rdrbb_s_dn5 = assign15490_e10333_d_n5;
        var_uc_rdrbb_s_dn6 = assign15490_e10333_d_n6;
        var_uc_rdrbb_s_dn7 = assign15490_e10333_d_n7;
        var_uc_rdrbb_s_dn8 = assign15490_e10333_d_n8;
        var_uc_rdrbb_s_dn9 = assign15490_e10333_d_n9;
        var_uc_rdrbb_s_dn10 = assign15490_e10333_d_n10;
        var_uc_rdrbb_s_dn11 = assign15490_e10333_d_n11;
        var_uc_rdrbb_s_dn14 = assign15490_e10333_d_n14;

        let (assign15500_e10346, assign15500_e10346_d_n0, assign15500_e10346_d_n2, assign15500_e10346_d_n4, assign15500_e10346_d_n5, assign15500_e10346_d_n6, assign15500_e10346_d_n7, assign15500_e10346_d_n8, assign15500_e10346_d_n9, assign15500_e10346_d_n10, assign15500_e10346_d_n11, assign15500_e10346_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard330 != 0.0)) {
        let (assign15500_e10344, assign15500_e10344_d_n0, assign15500_e10344_d_n2, assign15500_e10344_d_n4, assign15500_e10344_d_n5, assign15500_e10344_d_n6, assign15500_e10344_d_n7, assign15500_e10344_d_n8, assign15500_e10344_d_n9, assign15500_e10344_d_n10, assign15500_e10344_d_n11, assign15500_e10344_d_n14,) = {
            if (var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign15500_e10343: f64 = (var_tratio).powf(p.p415);
                (assign15500_e10343, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((var_tratio).powf(p.p415 - 1.0) * var_tratio_dn0)) } } else { (assign15500_e10343 * (p.p415 * (var_tratio_dn0 / var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((var_tratio).powf(p.p415 - 1.0) * var_tratio_dn2)) } } else { (assign15500_e10343 * (p.p415 * (var_tratio_dn2 / var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((var_tratio).powf(p.p415 - 1.0) * var_tratio_dn4)) } } else { (assign15500_e10343 * (p.p415 * (var_tratio_dn4 / var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((var_tratio).powf(p.p415 - 1.0) * var_tratio_dn5)) } } else { (assign15500_e10343 * (p.p415 * (var_tratio_dn5 / var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((var_tratio).powf(p.p415 - 1.0) * var_tratio_dn6)) } } else { (assign15500_e10343 * (p.p415 * (var_tratio_dn6 / var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((var_tratio).powf(p.p415 - 1.0) * var_tratio_dn7)) } } else { (assign15500_e10343 * (p.p415 * (var_tratio_dn7 / var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((var_tratio).powf(p.p415 - 1.0) * var_tratio_dn8)) } } else { (assign15500_e10343 * (p.p415 * (var_tratio_dn8 / var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((var_tratio).powf(p.p415 - 1.0) * var_tratio_dn9)) } } else { (assign15500_e10343 * (p.p415 * (var_tratio_dn9 / var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((var_tratio).powf(p.p415 - 1.0) * var_tratio_dn10)) } } else { (assign15500_e10343 * (p.p415 * (var_tratio_dn10 / var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((var_tratio).powf(p.p415 - 1.0) * var_tratio_dn11)) } } else { (assign15500_e10343 * (p.p415 * (var_tratio_dn11 / var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((var_tratio).powf(p.p415 - 1.0) * var_tratio_dn14)) } } else { (assign15500_e10343 * (p.p415 * (var_tratio_dn14 / var_tratio))) },)
            }
        };
        (assign15500_e10344, assign15500_e10344_d_n0, assign15500_e10344_d_n2, assign15500_e10344_d_n4, assign15500_e10344_d_n5, assign15500_e10344_d_n6, assign15500_e10344_d_n7, assign15500_e10344_d_n8, assign15500_e10344_d_n9, assign15500_e10344_d_n10, assign15500_e10344_d_n11, assign15500_e10344_d_n14,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign15500_e10346;
        var_t1_dn0 = assign15500_e10346_d_n0;
        var_t1_dn2 = assign15500_e10346_d_n2;
        var_t1_dn4 = assign15500_e10346_d_n4;
        var_t1_dn5 = assign15500_e10346_d_n5;
        var_t1_dn6 = assign15500_e10346_d_n6;
        var_t1_dn7 = assign15500_e10346_d_n7;
        var_t1_dn8 = assign15500_e10346_d_n8;
        var_t1_dn9 = assign15500_e10346_d_n9;
        var_t1_dn10 = assign15500_e10346_d_n10;
        var_t1_dn11 = assign15500_e10346_d_n11;
        var_t1_dn14 = assign15500_e10346_d_n14;

        let (assign15510_e10354, assign15510_e10354_d_n0, assign15510_e10354_d_n2, assign15510_e10354_d_n4, assign15510_e10354_d_n5, assign15510_e10354_d_n6, assign15510_e10354_d_n7, assign15510_e10354_d_n8, assign15510_e10354_d_n9, assign15510_e10354_d_n10, assign15510_e10354_d_n11, assign15510_e10354_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard330 != 0.0)) {
        let assign15510_e10352: f64 = (var_mks_rdrmue / var_t1);
        (assign15510_e10352, (-((var_mks_rdrmue * var_t1_dn0) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn2) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn4) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn5) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn6) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn7) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn8) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn9) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn10) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn11) / (var_t1 * var_t1))), (-((var_mks_rdrmue * var_t1_dn14) / (var_t1 * var_t1))),)
    } else {
        (var_rrdrmue, var_rrdrmue_dn0, var_rrdrmue_dn2, var_rrdrmue_dn4, var_rrdrmue_dn5, var_rrdrmue_dn6, var_rrdrmue_dn7, var_rrdrmue_dn8, var_rrdrmue_dn9, var_rrdrmue_dn10, var_rrdrmue_dn11, var_rrdrmue_dn14,)
    }
};
        var_rrdrmue = assign15510_e10354;
        var_rrdrmue_dn0 = assign15510_e10354_d_n0;
        var_rrdrmue_dn2 = assign15510_e10354_d_n2;
        var_rrdrmue_dn4 = assign15510_e10354_d_n4;
        var_rrdrmue_dn5 = assign15510_e10354_d_n5;
        var_rrdrmue_dn6 = assign15510_e10354_d_n6;
        var_rrdrmue_dn7 = assign15510_e10354_d_n7;
        var_rrdrmue_dn8 = assign15510_e10354_d_n8;
        var_rrdrmue_dn9 = assign15510_e10354_d_n9;
        var_rrdrmue_dn10 = assign15510_e10354_d_n10;
        var_rrdrmue_dn11 = assign15510_e10354_d_n11;
        var_rrdrmue_dn14 = assign15510_e10354_d_n14;

        let (assign15520_e10376, assign15520_e10376_d_n0, assign15520_e10376_d_n2, assign15520_e10376_d_n4, assign15520_e10376_d_n5, assign15520_e10376_d_n6, assign15520_e10376_d_n7, assign15520_e10376_d_n8, assign15520_e10376_d_n9, assign15520_e10376_d_n10, assign15520_e10376_d_n11, assign15520_e10376_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard330 != 0.0)) {
        let assign15520_e10361: f64 = (0.4 * var_tratio);
        let assign15520_e10362: f64 = (1.8 + assign15520_e10361);
        let assign15520_e10365: f64 = (0.1 * var_tratio);
        let assign15520_e10367: f64 = (assign15520_e10365 * var_tratio);
        let assign15520_e10368: f64 = (assign15520_e10362 + assign15520_e10367);
        let assign15520_e10372: f64 = (1.0 - var_tratio);
        let assign15520_e10373: f64 = (p.p417 * assign15520_e10372);
        let assign15520_e10374: f64 = (assign15520_e10368 - assign15520_e10373);
        (assign15520_e10374, (((0.4 * var_tratio_dn0) + (((0.1 * var_tratio_dn0) * var_tratio) + (assign15520_e10365 * var_tratio_dn0))) - (p.p417 * (-var_tratio_dn0))), (((0.4 * var_tratio_dn2) + (((0.1 * var_tratio_dn2) * var_tratio) + (assign15520_e10365 * var_tratio_dn2))) - (p.p417 * (-var_tratio_dn2))), (((0.4 * var_tratio_dn4) + (((0.1 * var_tratio_dn4) * var_tratio) + (assign15520_e10365 * var_tratio_dn4))) - (p.p417 * (-var_tratio_dn4))), (((0.4 * var_tratio_dn5) + (((0.1 * var_tratio_dn5) * var_tratio) + (assign15520_e10365 * var_tratio_dn5))) - (p.p417 * (-var_tratio_dn5))), (((0.4 * var_tratio_dn6) + (((0.1 * var_tratio_dn6) * var_tratio) + (assign15520_e10365 * var_tratio_dn6))) - (p.p417 * (-var_tratio_dn6))), (((0.4 * var_tratio_dn7) + (((0.1 * var_tratio_dn7) * var_tratio) + (assign15520_e10365 * var_tratio_dn7))) - (p.p417 * (-var_tratio_dn7))), (((0.4 * var_tratio_dn8) + (((0.1 * var_tratio_dn8) * var_tratio) + (assign15520_e10365 * var_tratio_dn8))) - (p.p417 * (-var_tratio_dn8))), (((0.4 * var_tratio_dn9) + (((0.1 * var_tratio_dn9) * var_tratio) + (assign15520_e10365 * var_tratio_dn9))) - (p.p417 * (-var_tratio_dn9))), (((0.4 * var_tratio_dn10) + (((0.1 * var_tratio_dn10) * var_tratio) + (assign15520_e10365 * var_tratio_dn10))) - (p.p417 * (-var_tratio_dn10))), (((0.4 * var_tratio_dn11) + (((0.1 * var_tratio_dn11) * var_tratio) + (assign15520_e10365 * var_tratio_dn11))) - (p.p417 * (-var_tratio_dn11))), (((0.4 * var_tratio_dn14) + (((0.1 * var_tratio_dn14) * var_tratio) + (assign15520_e10365 * var_tratio_dn14))) - (p.p417 * (-var_tratio_dn14))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn14,)
    }
};
        var_t0 = assign15520_e10376;
        var_t0_dn0 = assign15520_e10376_d_n0;
        var_t0_dn2 = assign15520_e10376_d_n2;
        var_t0_dn4 = assign15520_e10376_d_n4;
        var_t0_dn5 = assign15520_e10376_d_n5;
        var_t0_dn6 = assign15520_e10376_d_n6;
        var_t0_dn7 = assign15520_e10376_d_n7;
        var_t0_dn8 = assign15520_e10376_d_n8;
        var_t0_dn9 = assign15520_e10376_d_n9;
        var_t0_dn10 = assign15520_e10376_d_n10;
        var_t0_dn11 = assign15520_e10376_d_n11;
        var_t0_dn14 = assign15520_e10376_d_n14;

        let (assign15530_e10384, assign15530_e10384_d_n0, assign15530_e10384_d_n2, assign15530_e10384_d_n4, assign15530_e10384_d_n5, assign15530_e10384_d_n6, assign15530_e10384_d_n7, assign15530_e10384_d_n8, assign15530_e10384_d_n9, assign15530_e10384_d_n10, assign15530_e10384_d_n11, assign15530_e10384_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard330 != 0.0)) {
        let assign15530_e10382: f64 = (var_mks_rdrvmax / var_t0);
        (assign15530_e10382, (-((var_mks_rdrvmax * var_t0_dn0) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn2) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn4) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn5) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn6) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn7) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn8) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn9) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn10) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn11) / (var_t0 * var_t0))), (-((var_mks_rdrvmax * var_t0_dn14) / (var_t0 * var_t0))),)
    } else {
        (var_rrdrvmax, var_rrdrvmax_dn0, var_rrdrvmax_dn2, var_rrdrvmax_dn4, var_rrdrvmax_dn5, var_rrdrvmax_dn6, var_rrdrvmax_dn7, var_rrdrvmax_dn8, var_rrdrvmax_dn9, var_rrdrvmax_dn10, var_rrdrvmax_dn11, var_rrdrvmax_dn14,)
    }
};
        var_rrdrvmax = assign15530_e10384;
        var_rrdrvmax_dn0 = assign15530_e10384_d_n0;
        var_rrdrvmax_dn2 = assign15530_e10384_d_n2;
        var_rrdrvmax_dn4 = assign15530_e10384_d_n4;
        var_rrdrvmax_dn5 = assign15530_e10384_d_n5;
        var_rrdrvmax_dn6 = assign15530_e10384_d_n6;
        var_rrdrvmax_dn7 = assign15530_e10384_d_n7;
        var_rrdrvmax_dn8 = assign15530_e10384_d_n8;
        var_rrdrvmax_dn9 = assign15530_e10384_d_n9;
        var_rrdrvmax_dn10 = assign15530_e10384_d_n10;
        var_rrdrvmax_dn11 = assign15530_e10384_d_n11;
        var_rrdrvmax_dn14 = assign15530_e10384_d_n14;

        let (assign15540_e10396, assign15540_e10396_d_n0, assign15540_e10396_d_n2, assign15540_e10396_d_n4, assign15540_e10396_d_n5, assign15540_e10396_d_n6, assign15540_e10396_d_n7, assign15540_e10396_d_n8, assign15540_e10396_d_n9, assign15540_e10396_d_n10, assign15540_e10396_d_n11, assign15540_e10396_d_n14,) = {
    if ((var_guard293 != 0.0) && (var_guard330 != 0.0)) {
        let assign15540_e10392: f64 = (var_ttemp - var_ktnom);
        let assign15540_e10393: f64 = (p.p438 * assign15540_e10392);
        let assign15540_e10394: f64 = (var_uc_rdrbb + assign15540_e10393);
        (assign15540_e10394, (var_uc_rdrbb_dn0 + (p.p438 * var_ttemp_dn0)), (var_uc_rdrbb_dn2 + (p.p438 * var_ttemp_dn2)), (var_uc_rdrbb_dn4 + (p.p438 * var_ttemp_dn4)), (var_uc_rdrbb_dn5 + (p.p438 * var_ttemp_dn5)), (var_uc_rdrbb_dn6 + (p.p438 * var_ttemp_dn6)), (var_uc_rdrbb_dn7 + (p.p438 * var_ttemp_dn7)), (var_uc_rdrbb_dn8 + (p.p438 * var_ttemp_dn8)), (var_uc_rdrbb_dn9 + (p.p438 * var_ttemp_dn9)), (var_uc_rdrbb_dn10 + (p.p438 * var_ttemp_dn10)), (var_uc_rdrbb_dn11 + (p.p438 * var_ttemp_dn11)), (var_uc_rdrbb_dn14 + (p.p438 * var_ttemp_dn14)),)
    } else {
        (var_uc_rdrbb, var_uc_rdrbb_dn0, var_uc_rdrbb_dn2, var_uc_rdrbb_dn4, var_uc_rdrbb_dn5, var_uc_rdrbb_dn6, var_uc_rdrbb_dn7, var_uc_rdrbb_dn8, var_uc_rdrbb_dn9, var_uc_rdrbb_dn10, var_uc_rdrbb_dn11, var_uc_rdrbb_dn14,)
    }
};
        var_uc_rdrbb = assign15540_e10396;
        var_uc_rdrbb_dn0 = assign15540_e10396_d_n0;
        var_uc_rdrbb_dn2 = assign15540_e10396_d_n2;
        var_uc_rdrbb_dn4 = assign15540_e10396_d_n4;
        var_uc_rdrbb_dn5 = assign15540_e10396_d_n5;
        var_uc_rdrbb_dn6 = assign15540_e10396_d_n6;
        var_uc_rdrbb_dn7 = assign15540_e10396_d_n7;
        var_uc_rdrbb_dn8 = assign15540_e10396_d_n8;
        var_uc_rdrbb_dn9 = assign15540_e10396_d_n9;
        var_uc_rdrbb_dn10 = assign15540_e10396_d_n10;
        var_uc_rdrbb_dn11 = assign15540_e10396_d_n11;
        var_uc_rdrbb_dn14 = assign15540_e10396_d_n14;

        let assign15560_e10404: f64 = if var_uc_rdrbb < 0.1 { 1.0 } else { 0.0 };
        var_guard332 = assign15560_e10404;

        *var_costi0_slot = var_costi0;
        *var_costi0_dn0_slot = var_costi0_dn0;
        *var_costi0_dn10_slot = var_costi0_dn10;
        *var_costi0_dn11_slot = var_costi0_dn11;
        *var_costi0_dn14_slot = var_costi0_dn14;
        *var_costi0_dn2_slot = var_costi0_dn2;
        *var_costi0_dn4_slot = var_costi0_dn4;
        *var_costi0_dn5_slot = var_costi0_dn5;
        *var_costi0_dn6_slot = var_costi0_dn6;
        *var_costi0_dn7_slot = var_costi0_dn7;
        *var_costi0_dn8_slot = var_costi0_dn8;
        *var_costi0_dn9_slot = var_costi0_dn9;
        *var_costi0_p2_slot = var_costi0_p2;
        *var_costi0_p2_dn0_slot = var_costi0_p2_dn0;
        *var_costi0_p2_dn10_slot = var_costi0_p2_dn10;
        *var_costi0_p2_dn11_slot = var_costi0_p2_dn11;
        *var_costi0_p2_dn14_slot = var_costi0_p2_dn14;
        *var_costi0_p2_dn2_slot = var_costi0_p2_dn2;
        *var_costi0_p2_dn4_slot = var_costi0_p2_dn4;
        *var_costi0_p2_dn5_slot = var_costi0_p2_dn5;
        *var_costi0_p2_dn6_slot = var_costi0_p2_dn6;
        *var_costi0_p2_dn7_slot = var_costi0_p2_dn7;
        *var_costi0_p2_dn8_slot = var_costi0_p2_dn8;
        *var_costi0_p2_dn9_slot = var_costi0_p2_dn9;
        *var_costi1_slot = var_costi1;
        *var_costi1_dn0_slot = var_costi1_dn0;
        *var_costi1_dn10_slot = var_costi1_dn10;
        *var_costi1_dn11_slot = var_costi1_dn11;
        *var_costi1_dn14_slot = var_costi1_dn14;
        *var_costi1_dn2_slot = var_costi1_dn2;
        *var_costi1_dn4_slot = var_costi1_dn4;
        *var_costi1_dn5_slot = var_costi1_dn5;
        *var_costi1_dn6_slot = var_costi1_dn6;
        *var_costi1_dn7_slot = var_costi1_dn7;
        *var_costi1_dn8_slot = var_costi1_dn8;
        *var_costi1_dn9_slot = var_costi1_dn9;
        *var_guard328_slot = var_guard328;
        *var_guard329_slot = var_guard329;
        *var_guard330_slot = var_guard330;
        *var_guard332_slot = var_guard332;
        *var_hbdceff_slot = var_hbdceff;
        *var_hbdceff_dn0_slot = var_hbdceff_dn0;
        *var_hbdceff_dn10_slot = var_hbdceff_dn10;
        *var_hbdceff_dn11_slot = var_hbdceff_dn11;
        *var_hbdceff_dn14_slot = var_hbdceff_dn14;
        *var_hbdceff_dn2_slot = var_hbdceff_dn2;
        *var_hbdceff_dn4_slot = var_hbdceff_dn4;
        *var_hbdceff_dn5_slot = var_hbdceff_dn5;
        *var_hbdceff_dn6_slot = var_hbdceff_dn6;
        *var_hbdceff_dn7_slot = var_hbdceff_dn7;
        *var_hbdceff_dn8_slot = var_hbdceff_dn8;
        *var_hbdceff_dn9_slot = var_hbdceff_dn9;
        *var_rdvde_slot = var_rdvde;
        *var_rdvde_dn0_slot = var_rdvde_dn0;
        *var_rdvde_dn10_slot = var_rdvde_dn10;
        *var_rdvde_dn11_slot = var_rdvde_dn11;
        *var_rdvde_dn14_slot = var_rdvde_dn14;
        *var_rdvde_dn2_slot = var_rdvde_dn2;
        *var_rdvde_dn4_slot = var_rdvde_dn4;
        *var_rdvde_dn5_slot = var_rdvde_dn5;
        *var_rdvde_dn6_slot = var_rdvde_dn6;
        *var_rdvde_dn7_slot = var_rdvde_dn7;
        *var_rdvde_dn8_slot = var_rdvde_dn8;
        *var_rdvde_dn9_slot = var_rdvde_dn9;
        *var_rrdrmue_slot = var_rrdrmue;
        *var_rrdrmue_dn0_slot = var_rrdrmue_dn0;
        *var_rrdrmue_dn10_slot = var_rrdrmue_dn10;
        *var_rrdrmue_dn11_slot = var_rrdrmue_dn11;
        *var_rrdrmue_dn14_slot = var_rrdrmue_dn14;
        *var_rrdrmue_dn2_slot = var_rrdrmue_dn2;
        *var_rrdrmue_dn4_slot = var_rrdrmue_dn4;
        *var_rrdrmue_dn5_slot = var_rrdrmue_dn5;
        *var_rrdrmue_dn6_slot = var_rrdrmue_dn6;
        *var_rrdrmue_dn7_slot = var_rrdrmue_dn7;
        *var_rrdrmue_dn8_slot = var_rrdrmue_dn8;
        *var_rrdrmue_dn9_slot = var_rrdrmue_dn9;
        *var_rrdrmues_slot = var_rrdrmues;
        *var_rrdrmues_dn0_slot = var_rrdrmues_dn0;
        *var_rrdrmues_dn10_slot = var_rrdrmues_dn10;
        *var_rrdrmues_dn11_slot = var_rrdrmues_dn11;
        *var_rrdrmues_dn14_slot = var_rrdrmues_dn14;
        *var_rrdrmues_dn2_slot = var_rrdrmues_dn2;
        *var_rrdrmues_dn4_slot = var_rrdrmues_dn4;
        *var_rrdrmues_dn5_slot = var_rrdrmues_dn5;
        *var_rrdrmues_dn6_slot = var_rrdrmues_dn6;
        *var_rrdrmues_dn7_slot = var_rrdrmues_dn7;
        *var_rrdrmues_dn8_slot = var_rrdrmues_dn8;
        *var_rrdrmues_dn9_slot = var_rrdrmues_dn9;
        *var_rrdrvmax_slot = var_rrdrvmax;
        *var_rrdrvmax_dn0_slot = var_rrdrvmax_dn0;
        *var_rrdrvmax_dn10_slot = var_rrdrvmax_dn10;
        *var_rrdrvmax_dn11_slot = var_rrdrvmax_dn11;
        *var_rrdrvmax_dn14_slot = var_rrdrvmax_dn14;
        *var_rrdrvmax_dn2_slot = var_rrdrvmax_dn2;
        *var_rrdrvmax_dn4_slot = var_rrdrvmax_dn4;
        *var_rrdrvmax_dn5_slot = var_rrdrvmax_dn5;
        *var_rrdrvmax_dn6_slot = var_rrdrvmax_dn6;
        *var_rrdrvmax_dn7_slot = var_rrdrvmax_dn7;
        *var_rrdrvmax_dn8_slot = var_rrdrvmax_dn8;
        *var_rrdrvmax_dn9_slot = var_rrdrvmax_dn9;
        *var_rrdrvmaxs_slot = var_rrdrvmaxs;
        *var_rrdrvmaxs_dn0_slot = var_rrdrvmaxs_dn0;
        *var_rrdrvmaxs_dn10_slot = var_rrdrvmaxs_dn10;
        *var_rrdrvmaxs_dn11_slot = var_rrdrvmaxs_dn11;
        *var_rrdrvmaxs_dn14_slot = var_rrdrvmaxs_dn14;
        *var_rrdrvmaxs_dn2_slot = var_rrdrvmaxs_dn2;
        *var_rrdrvmaxs_dn4_slot = var_rrdrvmaxs_dn4;
        *var_rrdrvmaxs_dn5_slot = var_rrdrvmaxs_dn5;
        *var_rrdrvmaxs_dn6_slot = var_rrdrvmaxs_dn6;
        *var_rrdrvmaxs_dn7_slot = var_rrdrvmaxs_dn7;
        *var_rrdrvmaxs_dn8_slot = var_rrdrvmaxs_dn8;
        *var_rrdrvmaxs_dn9_slot = var_rrdrvmaxs_dn9;
        *var_rsvde_slot = var_rsvde;
        *var_rsvde_dn0_slot = var_rsvde_dn0;
        *var_rsvde_dn10_slot = var_rsvde_dn10;
        *var_rsvde_dn11_slot = var_rsvde_dn11;
        *var_rsvde_dn14_slot = var_rsvde_dn14;
        *var_rsvde_dn2_slot = var_rsvde_dn2;
        *var_rsvde_dn4_slot = var_rsvde_dn4;
        *var_rsvde_dn5_slot = var_rsvde_dn5;
        *var_rsvde_dn6_slot = var_rsvde_dn6;
        *var_rsvde_dn7_slot = var_rsvde_dn7;
        *var_rsvde_dn8_slot = var_rsvde_dn8;
        *var_rsvde_dn9_slot = var_rsvde_dn9;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn14_slot = var_tmf2_dn14;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_dn9_slot = var_tmf2_dn9;
        *var_uc_rdrbb_slot = var_uc_rdrbb;
        *var_uc_rdrbb_dn0_slot = var_uc_rdrbb_dn0;
        *var_uc_rdrbb_dn10_slot = var_uc_rdrbb_dn10;
        *var_uc_rdrbb_dn11_slot = var_uc_rdrbb_dn11;
        *var_uc_rdrbb_dn14_slot = var_uc_rdrbb_dn14;
        *var_uc_rdrbb_dn2_slot = var_uc_rdrbb_dn2;
        *var_uc_rdrbb_dn4_slot = var_uc_rdrbb_dn4;
        *var_uc_rdrbb_dn5_slot = var_uc_rdrbb_dn5;
        *var_uc_rdrbb_dn6_slot = var_uc_rdrbb_dn6;
        *var_uc_rdrbb_dn7_slot = var_uc_rdrbb_dn7;
        *var_uc_rdrbb_dn8_slot = var_uc_rdrbb_dn8;
        *var_uc_rdrbb_dn9_slot = var_uc_rdrbb_dn9;
        *var_uc_rdrbb_s_slot = var_uc_rdrbb_s;
        *var_uc_rdrbb_s_dn0_slot = var_uc_rdrbb_s_dn0;
        *var_uc_rdrbb_s_dn10_slot = var_uc_rdrbb_s_dn10;
        *var_uc_rdrbb_s_dn11_slot = var_uc_rdrbb_s_dn11;
        *var_uc_rdrbb_s_dn14_slot = var_uc_rdrbb_s_dn14;
        *var_uc_rdrbb_s_dn2_slot = var_uc_rdrbb_s_dn2;
        *var_uc_rdrbb_s_dn4_slot = var_uc_rdrbb_s_dn4;
        *var_uc_rdrbb_s_dn5_slot = var_uc_rdrbb_s_dn5;
        *var_uc_rdrbb_s_dn6_slot = var_uc_rdrbb_s_dn6;
        *var_uc_rdrbb_s_dn7_slot = var_uc_rdrbb_s_dn7;
        *var_uc_rdrbb_s_dn8_slot = var_uc_rdrbb_s_dn8;
        *var_uc_rdrbb_s_dn9_slot = var_uc_rdrbb_s_dn9;
        *var_uc_subtmp_slot = var_uc_subtmp;
    }
}
