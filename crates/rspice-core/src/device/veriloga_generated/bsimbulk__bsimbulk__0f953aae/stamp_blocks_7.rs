#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        var_bin_l: f64,
        var_bin_w: f64,
        var_bin_wl: f64,
        var_delta_i_slot: &mut f64,
        var_delta_i_dn0_slot: &mut f64,
        var_delta_i_dn10_slot: &mut f64,
        var_delta_i_dn11_slot: &mut f64,
        var_delta_i_dn12_slot: &mut f64,
        var_delta_i_dn13_slot: &mut f64,
        var_delta_i_dn14_slot: &mut f64,
        var_delta_i_dn2_slot: &mut f64,
        var_delta_i_dn3_slot: &mut f64,
        var_delta_i_dn4_slot: &mut f64,
        var_delta_i_dn5_slot: &mut f64,
        var_delta_i_dn6_slot: &mut f64,
        var_delta_i_dn7_slot: &mut f64,
        var_delta_i_dn8_slot: &mut f64,
        var_delta_i_dn9_slot: &mut f64,
        var_delta_i_rv_slot: &mut f64,
        var_eta0_i_slot: &mut f64,
        var_eta0_i_dn0_slot: &mut f64,
        var_eta0_i_dn10_slot: &mut f64,
        var_eta0_i_dn11_slot: &mut f64,
        var_eta0_i_dn12_slot: &mut f64,
        var_eta0_i_dn13_slot: &mut f64,
        var_eta0_i_dn14_slot: &mut f64,
        var_eta0_i_dn2_slot: &mut f64,
        var_eta0_i_dn3_slot: &mut f64,
        var_eta0_i_dn4_slot: &mut f64,
        var_eta0_i_dn5_slot: &mut f64,
        var_eta0_i_dn6_slot: &mut f64,
        var_eta0_i_dn7_slot: &mut f64,
        var_eta0_i_dn8_slot: &mut f64,
        var_eta0_i_dn9_slot: &mut f64,
        var_eta0_i_rv_slot: &mut f64,
        var_etab_i_slot: &mut f64,
        var_etab_i_rv_slot: &mut f64,
        var_eu_i_slot: &mut f64,
        var_eu_i_dn0_slot: &mut f64,
        var_eu_i_dn10_slot: &mut f64,
        var_eu_i_dn11_slot: &mut f64,
        var_eu_i_dn12_slot: &mut f64,
        var_eu_i_dn13_slot: &mut f64,
        var_eu_i_dn14_slot: &mut f64,
        var_eu_i_dn2_slot: &mut f64,
        var_eu_i_dn3_slot: &mut f64,
        var_eu_i_dn4_slot: &mut f64,
        var_eu_i_dn5_slot: &mut f64,
        var_eu_i_dn6_slot: &mut f64,
        var_eu_i_dn7_slot: &mut f64,
        var_eu_i_dn8_slot: &mut f64,
        var_eu_i_dn9_slot: &mut f64,
        var_eu_i_rv_slot: &mut f64,
        var_fprout_i_slot: &mut f64,
        var_fprout_i_rv_slot: &mut f64,
        var_k1_i_slot: &mut f64,
        var_k1_i_dn0_slot: &mut f64,
        var_k1_i_dn10_slot: &mut f64,
        var_k1_i_dn11_slot: &mut f64,
        var_k1_i_dn12_slot: &mut f64,
        var_k1_i_dn13_slot: &mut f64,
        var_k1_i_dn14_slot: &mut f64,
        var_k1_i_dn2_slot: &mut f64,
        var_k1_i_dn3_slot: &mut f64,
        var_k1_i_dn4_slot: &mut f64,
        var_k1_i_dn5_slot: &mut f64,
        var_k1_i_dn6_slot: &mut f64,
        var_k1_i_dn7_slot: &mut f64,
        var_k1_i_dn8_slot: &mut f64,
        var_k1_i_dn9_slot: &mut f64,
        var_k1_i_rv_slot: &mut f64,
        var_k2_i_slot: &mut f64,
        var_k2_i_dn0_slot: &mut f64,
        var_k2_i_dn10_slot: &mut f64,
        var_k2_i_dn11_slot: &mut f64,
        var_k2_i_dn12_slot: &mut f64,
        var_k2_i_dn13_slot: &mut f64,
        var_k2_i_dn14_slot: &mut f64,
        var_k2_i_dn2_slot: &mut f64,
        var_k2_i_dn3_slot: &mut f64,
        var_k2_i_dn4_slot: &mut f64,
        var_k2_i_dn5_slot: &mut f64,
        var_k2_i_dn6_slot: &mut f64,
        var_k2_i_dn7_slot: &mut f64,
        var_k2_i_dn8_slot: &mut f64,
        var_k2_i_dn9_slot: &mut f64,
        var_k2_i_rv_slot: &mut f64,
        var_pclm_i_slot: &mut f64,
        var_pclm_i_dn0_slot: &mut f64,
        var_pclm_i_dn10_slot: &mut f64,
        var_pclm_i_dn11_slot: &mut f64,
        var_pclm_i_dn12_slot: &mut f64,
        var_pclm_i_dn13_slot: &mut f64,
        var_pclm_i_dn14_slot: &mut f64,
        var_pclm_i_dn2_slot: &mut f64,
        var_pclm_i_dn3_slot: &mut f64,
        var_pclm_i_dn4_slot: &mut f64,
        var_pclm_i_dn5_slot: &mut f64,
        var_pclm_i_dn6_slot: &mut f64,
        var_pclm_i_dn7_slot: &mut f64,
        var_pclm_i_dn8_slot: &mut f64,
        var_pclm_i_dn9_slot: &mut f64,
        var_pclm_i_rv_slot: &mut f64,
        var_pclmcv_i_slot: &mut f64,
        var_pclmcv_i_rv_slot: &mut f64,
        var_pdiblc_i_slot: &mut f64,
        var_pdiblc_i_dn0_slot: &mut f64,
        var_pdiblc_i_dn10_slot: &mut f64,
        var_pdiblc_i_dn11_slot: &mut f64,
        var_pdiblc_i_dn12_slot: &mut f64,
        var_pdiblc_i_dn13_slot: &mut f64,
        var_pdiblc_i_dn14_slot: &mut f64,
        var_pdiblc_i_dn2_slot: &mut f64,
        var_pdiblc_i_dn3_slot: &mut f64,
        var_pdiblc_i_dn4_slot: &mut f64,
        var_pdiblc_i_dn5_slot: &mut f64,
        var_pdiblc_i_dn6_slot: &mut f64,
        var_pdiblc_i_dn7_slot: &mut f64,
        var_pdiblc_i_dn8_slot: &mut f64,
        var_pdiblc_i_dn9_slot: &mut f64,
        var_pdiblc_i_rv_slot: &mut f64,
        var_pdiblcb_i_slot: &mut f64,
        var_pdiblcb_i_rv_slot: &mut f64,
        var_pdits_i_slot: &mut f64,
        var_pdits_i_rv_slot: &mut f64,
        var_pditsd_i_slot: &mut f64,
        var_pditsd_i_rv_slot: &mut f64,
        var_phin_i_slot: &mut f64,
        var_phin_i_rv_slot: &mut f64,
        var_prwb_i_slot: &mut f64,
        var_prwb_i_rv_slot: &mut f64,
        var_prwg_i_slot: &mut f64,
        var_prwg_i_rv_slot: &mut f64,
        var_psat_i_slot: &mut f64,
        var_psat_i_rv_slot: &mut f64,
        var_pscbe1_i_slot: &mut f64,
        var_pscbe1_i_rv_slot: &mut f64,
        var_pscbe2_i_slot: &mut f64,
        var_pscbe2_i_rv_slot: &mut f64,
        var_ptwg_i_slot: &mut f64,
        var_ptwg_i_dn0_slot: &mut f64,
        var_ptwg_i_dn10_slot: &mut f64,
        var_ptwg_i_dn11_slot: &mut f64,
        var_ptwg_i_dn12_slot: &mut f64,
        var_ptwg_i_dn13_slot: &mut f64,
        var_ptwg_i_dn14_slot: &mut f64,
        var_ptwg_i_dn2_slot: &mut f64,
        var_ptwg_i_dn3_slot: &mut f64,
        var_ptwg_i_dn4_slot: &mut f64,
        var_ptwg_i_dn5_slot: &mut f64,
        var_ptwg_i_dn6_slot: &mut f64,
        var_ptwg_i_dn7_slot: &mut f64,
        var_ptwg_i_dn8_slot: &mut f64,
        var_ptwg_i_dn9_slot: &mut f64,
        var_ptwg_i_rv_slot: &mut f64,
        var_pvag_i_slot: &mut f64,
        var_pvag_i_rv_slot: &mut f64,
        var_rdsw_i_slot: &mut f64,
        var_rdsw_i_rv_slot: &mut f64,
        var_rdswmin_i_slot: &mut f64,
        var_rdswmin_i_rv_slot: &mut f64,
        var_rdw_i_slot: &mut f64,
        var_rdw_i_rv_slot: &mut f64,
        var_rdwmin_i_slot: &mut f64,
        var_rdwmin_i_rv_slot: &mut f64,
        var_rsw_i_slot: &mut f64,
        var_rsw_i_rv_slot: &mut f64,
        var_rswmin_i_slot: &mut f64,
        var_rswmin_i_rv_slot: &mut f64,
        var_u0_i_slot: &mut f64,
        var_u0_i_rv_slot: &mut f64,
        var_ua_i_slot: &mut f64,
        var_ua_i_dn0_slot: &mut f64,
        var_ua_i_dn10_slot: &mut f64,
        var_ua_i_dn11_slot: &mut f64,
        var_ua_i_dn12_slot: &mut f64,
        var_ua_i_dn13_slot: &mut f64,
        var_ua_i_dn14_slot: &mut f64,
        var_ua_i_dn2_slot: &mut f64,
        var_ua_i_dn3_slot: &mut f64,
        var_ua_i_dn4_slot: &mut f64,
        var_ua_i_dn5_slot: &mut f64,
        var_ua_i_dn6_slot: &mut f64,
        var_ua_i_dn7_slot: &mut f64,
        var_ua_i_dn8_slot: &mut f64,
        var_ua_i_dn9_slot: &mut f64,
        var_ua_i_rv_slot: &mut f64,
        var_uc_i_slot: &mut f64,
        var_uc_i_dn0_slot: &mut f64,
        var_uc_i_dn10_slot: &mut f64,
        var_uc_i_dn11_slot: &mut f64,
        var_uc_i_dn12_slot: &mut f64,
        var_uc_i_dn13_slot: &mut f64,
        var_uc_i_dn14_slot: &mut f64,
        var_uc_i_dn2_slot: &mut f64,
        var_uc_i_dn3_slot: &mut f64,
        var_uc_i_dn4_slot: &mut f64,
        var_uc_i_dn5_slot: &mut f64,
        var_uc_i_dn6_slot: &mut f64,
        var_uc_i_dn7_slot: &mut f64,
        var_uc_i_dn8_slot: &mut f64,
        var_uc_i_dn9_slot: &mut f64,
        var_uc_i_rv_slot: &mut f64,
        var_ucs_i_slot: &mut f64,
        var_ucs_i_rv_slot: &mut f64,
        var_ud_i_slot: &mut f64,
        var_ud_i_dn0_slot: &mut f64,
        var_ud_i_dn10_slot: &mut f64,
        var_ud_i_dn11_slot: &mut f64,
        var_ud_i_dn12_slot: &mut f64,
        var_ud_i_dn13_slot: &mut f64,
        var_ud_i_dn14_slot: &mut f64,
        var_ud_i_dn2_slot: &mut f64,
        var_ud_i_dn3_slot: &mut f64,
        var_ud_i_dn4_slot: &mut f64,
        var_ud_i_dn5_slot: &mut f64,
        var_ud_i_dn6_slot: &mut f64,
        var_ud_i_dn7_slot: &mut f64,
        var_ud_i_dn8_slot: &mut f64,
        var_ud_i_dn9_slot: &mut f64,
        var_ud_i_rv_slot: &mut f64,
        var_vsat_i_slot: &mut f64,
        var_vsat_i_dn0_slot: &mut f64,
        var_vsat_i_dn10_slot: &mut f64,
        var_vsat_i_dn11_slot: &mut f64,
        var_vsat_i_dn12_slot: &mut f64,
        var_vsat_i_dn13_slot: &mut f64,
        var_vsat_i_dn14_slot: &mut f64,
        var_vsat_i_dn2_slot: &mut f64,
        var_vsat_i_dn3_slot: &mut f64,
        var_vsat_i_dn4_slot: &mut f64,
        var_vsat_i_dn5_slot: &mut f64,
        var_vsat_i_dn6_slot: &mut f64,
        var_vsat_i_dn7_slot: &mut f64,
        var_vsat_i_dn8_slot: &mut f64,
        var_vsat_i_dn9_slot: &mut f64,
        var_vsat_i_rv_slot: &mut f64,
        var_vsatcv_i_slot: &mut f64,
        var_vsatcv_i_dn0_slot: &mut f64,
        var_vsatcv_i_dn10_slot: &mut f64,
        var_vsatcv_i_dn11_slot: &mut f64,
        var_vsatcv_i_dn12_slot: &mut f64,
        var_vsatcv_i_dn13_slot: &mut f64,
        var_vsatcv_i_dn14_slot: &mut f64,
        var_vsatcv_i_dn2_slot: &mut f64,
        var_vsatcv_i_dn3_slot: &mut f64,
        var_vsatcv_i_dn4_slot: &mut f64,
        var_vsatcv_i_dn5_slot: &mut f64,
        var_vsatcv_i_dn6_slot: &mut f64,
        var_vsatcv_i_dn7_slot: &mut f64,
        var_vsatcv_i_dn8_slot: &mut f64,
        var_vsatcv_i_dn9_slot: &mut f64,
        var_vsatcv_i_rv_slot: &mut f64,
        var_wr_i_slot: &mut f64,
        var_wr_i_rv_slot: &mut f64,
        var_xj_i_slot: &mut f64,
        var_xj_i_rv_slot: &mut f64,
    ) {
        let mut var_delta_i: f64 = *var_delta_i_slot;
        let mut var_delta_i_dn0: f64 = *var_delta_i_dn0_slot;
        let mut var_delta_i_dn10: f64 = *var_delta_i_dn10_slot;
        let mut var_delta_i_dn11: f64 = *var_delta_i_dn11_slot;
        let mut var_delta_i_dn12: f64 = *var_delta_i_dn12_slot;
        let mut var_delta_i_dn13: f64 = *var_delta_i_dn13_slot;
        let mut var_delta_i_dn14: f64 = *var_delta_i_dn14_slot;
        let mut var_delta_i_dn2: f64 = *var_delta_i_dn2_slot;
        let mut var_delta_i_dn3: f64 = *var_delta_i_dn3_slot;
        let mut var_delta_i_dn4: f64 = *var_delta_i_dn4_slot;
        let mut var_delta_i_dn5: f64 = *var_delta_i_dn5_slot;
        let mut var_delta_i_dn6: f64 = *var_delta_i_dn6_slot;
        let mut var_delta_i_dn7: f64 = *var_delta_i_dn7_slot;
        let mut var_delta_i_dn8: f64 = *var_delta_i_dn8_slot;
        let mut var_delta_i_dn9: f64 = *var_delta_i_dn9_slot;
        let mut var_delta_i_rv: f64 = *var_delta_i_rv_slot;
        let mut var_eta0_i: f64 = *var_eta0_i_slot;
        let mut var_eta0_i_dn0: f64 = *var_eta0_i_dn0_slot;
        let mut var_eta0_i_dn10: f64 = *var_eta0_i_dn10_slot;
        let mut var_eta0_i_dn11: f64 = *var_eta0_i_dn11_slot;
        let mut var_eta0_i_dn12: f64 = *var_eta0_i_dn12_slot;
        let mut var_eta0_i_dn13: f64 = *var_eta0_i_dn13_slot;
        let mut var_eta0_i_dn14: f64 = *var_eta0_i_dn14_slot;
        let mut var_eta0_i_dn2: f64 = *var_eta0_i_dn2_slot;
        let mut var_eta0_i_dn3: f64 = *var_eta0_i_dn3_slot;
        let mut var_eta0_i_dn4: f64 = *var_eta0_i_dn4_slot;
        let mut var_eta0_i_dn5: f64 = *var_eta0_i_dn5_slot;
        let mut var_eta0_i_dn6: f64 = *var_eta0_i_dn6_slot;
        let mut var_eta0_i_dn7: f64 = *var_eta0_i_dn7_slot;
        let mut var_eta0_i_dn8: f64 = *var_eta0_i_dn8_slot;
        let mut var_eta0_i_dn9: f64 = *var_eta0_i_dn9_slot;
        let mut var_eta0_i_rv: f64 = *var_eta0_i_rv_slot;
        let mut var_etab_i: f64 = *var_etab_i_slot;
        let mut var_etab_i_rv: f64 = *var_etab_i_rv_slot;
        let mut var_eu_i: f64 = *var_eu_i_slot;
        let mut var_eu_i_dn0: f64 = *var_eu_i_dn0_slot;
        let mut var_eu_i_dn10: f64 = *var_eu_i_dn10_slot;
        let mut var_eu_i_dn11: f64 = *var_eu_i_dn11_slot;
        let mut var_eu_i_dn12: f64 = *var_eu_i_dn12_slot;
        let mut var_eu_i_dn13: f64 = *var_eu_i_dn13_slot;
        let mut var_eu_i_dn14: f64 = *var_eu_i_dn14_slot;
        let mut var_eu_i_dn2: f64 = *var_eu_i_dn2_slot;
        let mut var_eu_i_dn3: f64 = *var_eu_i_dn3_slot;
        let mut var_eu_i_dn4: f64 = *var_eu_i_dn4_slot;
        let mut var_eu_i_dn5: f64 = *var_eu_i_dn5_slot;
        let mut var_eu_i_dn6: f64 = *var_eu_i_dn6_slot;
        let mut var_eu_i_dn7: f64 = *var_eu_i_dn7_slot;
        let mut var_eu_i_dn8: f64 = *var_eu_i_dn8_slot;
        let mut var_eu_i_dn9: f64 = *var_eu_i_dn9_slot;
        let mut var_eu_i_rv: f64 = *var_eu_i_rv_slot;
        let mut var_fprout_i: f64 = *var_fprout_i_slot;
        let mut var_fprout_i_rv: f64 = *var_fprout_i_rv_slot;
        let mut var_k1_i: f64 = *var_k1_i_slot;
        let mut var_k1_i_dn0: f64 = *var_k1_i_dn0_slot;
        let mut var_k1_i_dn10: f64 = *var_k1_i_dn10_slot;
        let mut var_k1_i_dn11: f64 = *var_k1_i_dn11_slot;
        let mut var_k1_i_dn12: f64 = *var_k1_i_dn12_slot;
        let mut var_k1_i_dn13: f64 = *var_k1_i_dn13_slot;
        let mut var_k1_i_dn14: f64 = *var_k1_i_dn14_slot;
        let mut var_k1_i_dn2: f64 = *var_k1_i_dn2_slot;
        let mut var_k1_i_dn3: f64 = *var_k1_i_dn3_slot;
        let mut var_k1_i_dn4: f64 = *var_k1_i_dn4_slot;
        let mut var_k1_i_dn5: f64 = *var_k1_i_dn5_slot;
        let mut var_k1_i_dn6: f64 = *var_k1_i_dn6_slot;
        let mut var_k1_i_dn7: f64 = *var_k1_i_dn7_slot;
        let mut var_k1_i_dn8: f64 = *var_k1_i_dn8_slot;
        let mut var_k1_i_dn9: f64 = *var_k1_i_dn9_slot;
        let mut var_k1_i_rv: f64 = *var_k1_i_rv_slot;
        let mut var_k2_i: f64 = *var_k2_i_slot;
        let mut var_k2_i_dn0: f64 = *var_k2_i_dn0_slot;
        let mut var_k2_i_dn10: f64 = *var_k2_i_dn10_slot;
        let mut var_k2_i_dn11: f64 = *var_k2_i_dn11_slot;
        let mut var_k2_i_dn12: f64 = *var_k2_i_dn12_slot;
        let mut var_k2_i_dn13: f64 = *var_k2_i_dn13_slot;
        let mut var_k2_i_dn14: f64 = *var_k2_i_dn14_slot;
        let mut var_k2_i_dn2: f64 = *var_k2_i_dn2_slot;
        let mut var_k2_i_dn3: f64 = *var_k2_i_dn3_slot;
        let mut var_k2_i_dn4: f64 = *var_k2_i_dn4_slot;
        let mut var_k2_i_dn5: f64 = *var_k2_i_dn5_slot;
        let mut var_k2_i_dn6: f64 = *var_k2_i_dn6_slot;
        let mut var_k2_i_dn7: f64 = *var_k2_i_dn7_slot;
        let mut var_k2_i_dn8: f64 = *var_k2_i_dn8_slot;
        let mut var_k2_i_dn9: f64 = *var_k2_i_dn9_slot;
        let mut var_k2_i_rv: f64 = *var_k2_i_rv_slot;
        let mut var_pclm_i: f64 = *var_pclm_i_slot;
        let mut var_pclm_i_dn0: f64 = *var_pclm_i_dn0_slot;
        let mut var_pclm_i_dn10: f64 = *var_pclm_i_dn10_slot;
        let mut var_pclm_i_dn11: f64 = *var_pclm_i_dn11_slot;
        let mut var_pclm_i_dn12: f64 = *var_pclm_i_dn12_slot;
        let mut var_pclm_i_dn13: f64 = *var_pclm_i_dn13_slot;
        let mut var_pclm_i_dn14: f64 = *var_pclm_i_dn14_slot;
        let mut var_pclm_i_dn2: f64 = *var_pclm_i_dn2_slot;
        let mut var_pclm_i_dn3: f64 = *var_pclm_i_dn3_slot;
        let mut var_pclm_i_dn4: f64 = *var_pclm_i_dn4_slot;
        let mut var_pclm_i_dn5: f64 = *var_pclm_i_dn5_slot;
        let mut var_pclm_i_dn6: f64 = *var_pclm_i_dn6_slot;
        let mut var_pclm_i_dn7: f64 = *var_pclm_i_dn7_slot;
        let mut var_pclm_i_dn8: f64 = *var_pclm_i_dn8_slot;
        let mut var_pclm_i_dn9: f64 = *var_pclm_i_dn9_slot;
        let mut var_pclm_i_rv: f64 = *var_pclm_i_rv_slot;
        let mut var_pclmcv_i: f64 = *var_pclmcv_i_slot;
        let mut var_pclmcv_i_rv: f64 = *var_pclmcv_i_rv_slot;
        let mut var_pdiblc_i: f64 = *var_pdiblc_i_slot;
        let mut var_pdiblc_i_dn0: f64 = *var_pdiblc_i_dn0_slot;
        let mut var_pdiblc_i_dn10: f64 = *var_pdiblc_i_dn10_slot;
        let mut var_pdiblc_i_dn11: f64 = *var_pdiblc_i_dn11_slot;
        let mut var_pdiblc_i_dn12: f64 = *var_pdiblc_i_dn12_slot;
        let mut var_pdiblc_i_dn13: f64 = *var_pdiblc_i_dn13_slot;
        let mut var_pdiblc_i_dn14: f64 = *var_pdiblc_i_dn14_slot;
        let mut var_pdiblc_i_dn2: f64 = *var_pdiblc_i_dn2_slot;
        let mut var_pdiblc_i_dn3: f64 = *var_pdiblc_i_dn3_slot;
        let mut var_pdiblc_i_dn4: f64 = *var_pdiblc_i_dn4_slot;
        let mut var_pdiblc_i_dn5: f64 = *var_pdiblc_i_dn5_slot;
        let mut var_pdiblc_i_dn6: f64 = *var_pdiblc_i_dn6_slot;
        let mut var_pdiblc_i_dn7: f64 = *var_pdiblc_i_dn7_slot;
        let mut var_pdiblc_i_dn8: f64 = *var_pdiblc_i_dn8_slot;
        let mut var_pdiblc_i_dn9: f64 = *var_pdiblc_i_dn9_slot;
        let mut var_pdiblc_i_rv: f64 = *var_pdiblc_i_rv_slot;
        let mut var_pdiblcb_i: f64 = *var_pdiblcb_i_slot;
        let mut var_pdiblcb_i_rv: f64 = *var_pdiblcb_i_rv_slot;
        let mut var_pdits_i: f64 = *var_pdits_i_slot;
        let mut var_pdits_i_rv: f64 = *var_pdits_i_rv_slot;
        let mut var_pditsd_i: f64 = *var_pditsd_i_slot;
        let mut var_pditsd_i_rv: f64 = *var_pditsd_i_rv_slot;
        let mut var_phin_i: f64 = *var_phin_i_slot;
        let mut var_phin_i_rv: f64 = *var_phin_i_rv_slot;
        let mut var_prwb_i: f64 = *var_prwb_i_slot;
        let mut var_prwb_i_rv: f64 = *var_prwb_i_rv_slot;
        let mut var_prwg_i: f64 = *var_prwg_i_slot;
        let mut var_prwg_i_rv: f64 = *var_prwg_i_rv_slot;
        let mut var_psat_i: f64 = *var_psat_i_slot;
        let mut var_psat_i_rv: f64 = *var_psat_i_rv_slot;
        let mut var_pscbe1_i: f64 = *var_pscbe1_i_slot;
        let mut var_pscbe1_i_rv: f64 = *var_pscbe1_i_rv_slot;
        let mut var_pscbe2_i: f64 = *var_pscbe2_i_slot;
        let mut var_pscbe2_i_rv: f64 = *var_pscbe2_i_rv_slot;
        let mut var_ptwg_i: f64 = *var_ptwg_i_slot;
        let mut var_ptwg_i_dn0: f64 = *var_ptwg_i_dn0_slot;
        let mut var_ptwg_i_dn10: f64 = *var_ptwg_i_dn10_slot;
        let mut var_ptwg_i_dn11: f64 = *var_ptwg_i_dn11_slot;
        let mut var_ptwg_i_dn12: f64 = *var_ptwg_i_dn12_slot;
        let mut var_ptwg_i_dn13: f64 = *var_ptwg_i_dn13_slot;
        let mut var_ptwg_i_dn14: f64 = *var_ptwg_i_dn14_slot;
        let mut var_ptwg_i_dn2: f64 = *var_ptwg_i_dn2_slot;
        let mut var_ptwg_i_dn3: f64 = *var_ptwg_i_dn3_slot;
        let mut var_ptwg_i_dn4: f64 = *var_ptwg_i_dn4_slot;
        let mut var_ptwg_i_dn5: f64 = *var_ptwg_i_dn5_slot;
        let mut var_ptwg_i_dn6: f64 = *var_ptwg_i_dn6_slot;
        let mut var_ptwg_i_dn7: f64 = *var_ptwg_i_dn7_slot;
        let mut var_ptwg_i_dn8: f64 = *var_ptwg_i_dn8_slot;
        let mut var_ptwg_i_dn9: f64 = *var_ptwg_i_dn9_slot;
        let mut var_ptwg_i_rv: f64 = *var_ptwg_i_rv_slot;
        let mut var_pvag_i: f64 = *var_pvag_i_slot;
        let mut var_pvag_i_rv: f64 = *var_pvag_i_rv_slot;
        let mut var_rdsw_i: f64 = *var_rdsw_i_slot;
        let mut var_rdsw_i_rv: f64 = *var_rdsw_i_rv_slot;
        let mut var_rdswmin_i: f64 = *var_rdswmin_i_slot;
        let mut var_rdswmin_i_rv: f64 = *var_rdswmin_i_rv_slot;
        let mut var_rdw_i: f64 = *var_rdw_i_slot;
        let mut var_rdw_i_rv: f64 = *var_rdw_i_rv_slot;
        let mut var_rdwmin_i: f64 = *var_rdwmin_i_slot;
        let mut var_rdwmin_i_rv: f64 = *var_rdwmin_i_rv_slot;
        let mut var_rsw_i: f64 = *var_rsw_i_slot;
        let mut var_rsw_i_rv: f64 = *var_rsw_i_rv_slot;
        let mut var_rswmin_i: f64 = *var_rswmin_i_slot;
        let mut var_rswmin_i_rv: f64 = *var_rswmin_i_rv_slot;
        let mut var_u0_i: f64 = *var_u0_i_slot;
        let mut var_u0_i_rv: f64 = *var_u0_i_rv_slot;
        let mut var_ua_i: f64 = *var_ua_i_slot;
        let mut var_ua_i_dn0: f64 = *var_ua_i_dn0_slot;
        let mut var_ua_i_dn10: f64 = *var_ua_i_dn10_slot;
        let mut var_ua_i_dn11: f64 = *var_ua_i_dn11_slot;
        let mut var_ua_i_dn12: f64 = *var_ua_i_dn12_slot;
        let mut var_ua_i_dn13: f64 = *var_ua_i_dn13_slot;
        let mut var_ua_i_dn14: f64 = *var_ua_i_dn14_slot;
        let mut var_ua_i_dn2: f64 = *var_ua_i_dn2_slot;
        let mut var_ua_i_dn3: f64 = *var_ua_i_dn3_slot;
        let mut var_ua_i_dn4: f64 = *var_ua_i_dn4_slot;
        let mut var_ua_i_dn5: f64 = *var_ua_i_dn5_slot;
        let mut var_ua_i_dn6: f64 = *var_ua_i_dn6_slot;
        let mut var_ua_i_dn7: f64 = *var_ua_i_dn7_slot;
        let mut var_ua_i_dn8: f64 = *var_ua_i_dn8_slot;
        let mut var_ua_i_dn9: f64 = *var_ua_i_dn9_slot;
        let mut var_ua_i_rv: f64 = *var_ua_i_rv_slot;
        let mut var_uc_i: f64 = *var_uc_i_slot;
        let mut var_uc_i_dn0: f64 = *var_uc_i_dn0_slot;
        let mut var_uc_i_dn10: f64 = *var_uc_i_dn10_slot;
        let mut var_uc_i_dn11: f64 = *var_uc_i_dn11_slot;
        let mut var_uc_i_dn12: f64 = *var_uc_i_dn12_slot;
        let mut var_uc_i_dn13: f64 = *var_uc_i_dn13_slot;
        let mut var_uc_i_dn14: f64 = *var_uc_i_dn14_slot;
        let mut var_uc_i_dn2: f64 = *var_uc_i_dn2_slot;
        let mut var_uc_i_dn3: f64 = *var_uc_i_dn3_slot;
        let mut var_uc_i_dn4: f64 = *var_uc_i_dn4_slot;
        let mut var_uc_i_dn5: f64 = *var_uc_i_dn5_slot;
        let mut var_uc_i_dn6: f64 = *var_uc_i_dn6_slot;
        let mut var_uc_i_dn7: f64 = *var_uc_i_dn7_slot;
        let mut var_uc_i_dn8: f64 = *var_uc_i_dn8_slot;
        let mut var_uc_i_dn9: f64 = *var_uc_i_dn9_slot;
        let mut var_uc_i_rv: f64 = *var_uc_i_rv_slot;
        let mut var_ucs_i: f64 = *var_ucs_i_slot;
        let mut var_ucs_i_rv: f64 = *var_ucs_i_rv_slot;
        let mut var_ud_i: f64 = *var_ud_i_slot;
        let mut var_ud_i_dn0: f64 = *var_ud_i_dn0_slot;
        let mut var_ud_i_dn10: f64 = *var_ud_i_dn10_slot;
        let mut var_ud_i_dn11: f64 = *var_ud_i_dn11_slot;
        let mut var_ud_i_dn12: f64 = *var_ud_i_dn12_slot;
        let mut var_ud_i_dn13: f64 = *var_ud_i_dn13_slot;
        let mut var_ud_i_dn14: f64 = *var_ud_i_dn14_slot;
        let mut var_ud_i_dn2: f64 = *var_ud_i_dn2_slot;
        let mut var_ud_i_dn3: f64 = *var_ud_i_dn3_slot;
        let mut var_ud_i_dn4: f64 = *var_ud_i_dn4_slot;
        let mut var_ud_i_dn5: f64 = *var_ud_i_dn5_slot;
        let mut var_ud_i_dn6: f64 = *var_ud_i_dn6_slot;
        let mut var_ud_i_dn7: f64 = *var_ud_i_dn7_slot;
        let mut var_ud_i_dn8: f64 = *var_ud_i_dn8_slot;
        let mut var_ud_i_dn9: f64 = *var_ud_i_dn9_slot;
        let mut var_ud_i_rv: f64 = *var_ud_i_rv_slot;
        let mut var_vsat_i: f64 = *var_vsat_i_slot;
        let mut var_vsat_i_dn0: f64 = *var_vsat_i_dn0_slot;
        let mut var_vsat_i_dn10: f64 = *var_vsat_i_dn10_slot;
        let mut var_vsat_i_dn11: f64 = *var_vsat_i_dn11_slot;
        let mut var_vsat_i_dn12: f64 = *var_vsat_i_dn12_slot;
        let mut var_vsat_i_dn13: f64 = *var_vsat_i_dn13_slot;
        let mut var_vsat_i_dn14: f64 = *var_vsat_i_dn14_slot;
        let mut var_vsat_i_dn2: f64 = *var_vsat_i_dn2_slot;
        let mut var_vsat_i_dn3: f64 = *var_vsat_i_dn3_slot;
        let mut var_vsat_i_dn4: f64 = *var_vsat_i_dn4_slot;
        let mut var_vsat_i_dn5: f64 = *var_vsat_i_dn5_slot;
        let mut var_vsat_i_dn6: f64 = *var_vsat_i_dn6_slot;
        let mut var_vsat_i_dn7: f64 = *var_vsat_i_dn7_slot;
        let mut var_vsat_i_dn8: f64 = *var_vsat_i_dn8_slot;
        let mut var_vsat_i_dn9: f64 = *var_vsat_i_dn9_slot;
        let mut var_vsat_i_rv: f64 = *var_vsat_i_rv_slot;
        let mut var_vsatcv_i: f64 = *var_vsatcv_i_slot;
        let mut var_vsatcv_i_dn0: f64 = *var_vsatcv_i_dn0_slot;
        let mut var_vsatcv_i_dn10: f64 = *var_vsatcv_i_dn10_slot;
        let mut var_vsatcv_i_dn11: f64 = *var_vsatcv_i_dn11_slot;
        let mut var_vsatcv_i_dn12: f64 = *var_vsatcv_i_dn12_slot;
        let mut var_vsatcv_i_dn13: f64 = *var_vsatcv_i_dn13_slot;
        let mut var_vsatcv_i_dn14: f64 = *var_vsatcv_i_dn14_slot;
        let mut var_vsatcv_i_dn2: f64 = *var_vsatcv_i_dn2_slot;
        let mut var_vsatcv_i_dn3: f64 = *var_vsatcv_i_dn3_slot;
        let mut var_vsatcv_i_dn4: f64 = *var_vsatcv_i_dn4_slot;
        let mut var_vsatcv_i_dn5: f64 = *var_vsatcv_i_dn5_slot;
        let mut var_vsatcv_i_dn6: f64 = *var_vsatcv_i_dn6_slot;
        let mut var_vsatcv_i_dn7: f64 = *var_vsatcv_i_dn7_slot;
        let mut var_vsatcv_i_dn8: f64 = *var_vsatcv_i_dn8_slot;
        let mut var_vsatcv_i_dn9: f64 = *var_vsatcv_i_dn9_slot;
        let mut var_vsatcv_i_rv: f64 = *var_vsatcv_i_rv_slot;
        let mut var_wr_i: f64 = *var_wr_i_slot;
        let mut var_wr_i_rv: f64 = *var_wr_i_rv_slot;
        let mut var_xj_i: f64 = *var_xj_i_slot;
        let mut var_xj_i_rv: f64 = *var_xj_i_rv_slot;

        let assign1850_e2687: f64 = (var_bin_l * p.p202);
        let assign1850_e2688: f64 = (p.p195 + assign1850_e2687);
        let assign1850_e2691: f64 = (var_bin_w * p.p203);
        let assign1850_e2692: f64 = (assign1850_e2688 + assign1850_e2691);
        let assign1850_e2695: f64 = (var_bin_wl * p.p204);
        let assign1850_e2696: f64 = (assign1850_e2692 + assign1850_e2695);
        var_k2_i = assign1850_e2696;
        var_k2_i_dn0 = 0.0;
        var_k2_i_dn2 = 0.0;
        var_k2_i_dn3 = 0.0;
        var_k2_i_dn4 = 0.0;
        var_k2_i_dn5 = 0.0;
        var_k2_i_dn6 = 0.0;
        var_k2_i_dn7 = 0.0;
        var_k2_i_dn8 = 0.0;
        var_k2_i_dn9 = 0.0;
        var_k2_i_dn10 = 0.0;
        var_k2_i_dn11 = 0.0;
        var_k2_i_dn12 = 0.0;
        var_k2_i_dn13 = 0.0;
        var_k2_i_dn14 = 0.0;
        var_k2_i_rv = 0.0;

        let assign1860_e2700: f64 = (var_bin_l * p.p192);
        let assign1860_e2701: f64 = (p.p185 + assign1860_e2700);
        let assign1860_e2704: f64 = (var_bin_w * p.p193);
        let assign1860_e2705: f64 = (assign1860_e2701 + assign1860_e2704);
        let assign1860_e2708: f64 = (var_bin_wl * p.p194);
        let assign1860_e2709: f64 = (assign1860_e2705 + assign1860_e2708);
        var_k1_i = assign1860_e2709;
        var_k1_i_dn0 = 0.0;
        var_k1_i_dn2 = 0.0;
        var_k1_i_dn3 = 0.0;
        var_k1_i_dn4 = 0.0;
        var_k1_i_dn5 = 0.0;
        var_k1_i_dn6 = 0.0;
        var_k1_i_dn7 = 0.0;
        var_k1_i_dn8 = 0.0;
        var_k1_i_dn9 = 0.0;
        var_k1_i_dn10 = 0.0;
        var_k1_i_dn11 = 0.0;
        var_k1_i_dn12 = 0.0;
        var_k1_i_dn13 = 0.0;
        var_k1_i_dn14 = 0.0;
        var_k1_i_rv = 0.0;

        let assign1870_e2713: f64 = (var_bin_l * p.p113);
        let assign1870_e2714: f64 = (p.p112 + assign1870_e2713);
        let assign1870_e2717: f64 = (var_bin_w * p.p114);
        let assign1870_e2718: f64 = (assign1870_e2714 + assign1870_e2717);
        let assign1870_e2721: f64 = (var_bin_wl * p.p115);
        let assign1870_e2722: f64 = (assign1870_e2718 + assign1870_e2721);
        var_xj_i = assign1870_e2722;
        var_xj_i_rv = 0.0;

        let assign1880_e2726: f64 = (var_bin_l * p.p168);
        let assign1880_e2727: f64 = (p.p167 + assign1880_e2726);
        let assign1880_e2730: f64 = (var_bin_w * p.p169);
        let assign1880_e2731: f64 = (assign1880_e2727 + assign1880_e2730);
        let assign1880_e2734: f64 = (var_bin_wl * p.p170);
        let assign1880_e2735: f64 = (assign1880_e2731 + assign1880_e2734);
        var_phin_i = assign1880_e2735;
        var_phin_i_rv = 0.0;

        let assign1890_e2739: f64 = (var_bin_l * p.p172);
        let assign1890_e2740: f64 = (p.p171 + assign1890_e2739);
        let assign1890_e2743: f64 = (var_bin_w * p.p173);
        let assign1890_e2744: f64 = (assign1890_e2740 + assign1890_e2743);
        let assign1890_e2747: f64 = (var_bin_wl * p.p174);
        let assign1890_e2748: f64 = (assign1890_e2744 + assign1890_e2747);
        var_eta0_i = assign1890_e2748;
        var_eta0_i_dn0 = 0.0;
        var_eta0_i_dn2 = 0.0;
        var_eta0_i_dn3 = 0.0;
        var_eta0_i_dn4 = 0.0;
        var_eta0_i_dn5 = 0.0;
        var_eta0_i_dn6 = 0.0;
        var_eta0_i_dn7 = 0.0;
        var_eta0_i_dn8 = 0.0;
        var_eta0_i_dn9 = 0.0;
        var_eta0_i_dn10 = 0.0;
        var_eta0_i_dn11 = 0.0;
        var_eta0_i_dn12 = 0.0;
        var_eta0_i_dn13 = 0.0;
        var_eta0_i_dn14 = 0.0;
        var_eta0_i_rv = 0.0;

        let assign1900_e2752: f64 = (var_bin_l * p.p182);
        let assign1900_e2753: f64 = (p.p180 + assign1900_e2752);
        let assign1900_e2756: f64 = (var_bin_w * p.p183);
        let assign1900_e2757: f64 = (assign1900_e2753 + assign1900_e2756);
        let assign1900_e2760: f64 = (var_bin_wl * p.p184);
        let assign1900_e2761: f64 = (assign1900_e2757 + assign1900_e2760);
        var_etab_i = assign1900_e2761;
        var_etab_i_rv = 0.0;

        let assign1910_e2765: f64 = (var_bin_l * p.p254);
        let assign1910_e2766: f64 = (p.p253 + assign1910_e2765);
        let assign1910_e2769: f64 = (var_bin_w * p.p255);
        let assign1910_e2770: f64 = (assign1910_e2766 + assign1910_e2769);
        let assign1910_e2773: f64 = (var_bin_wl * p.p256);
        let assign1910_e2774: f64 = (assign1910_e2770 + assign1910_e2773);
        var_delta_i = assign1910_e2774;
        var_delta_i_dn0 = 0.0;
        var_delta_i_dn2 = 0.0;
        var_delta_i_dn3 = 0.0;
        var_delta_i_dn4 = 0.0;
        var_delta_i_dn5 = 0.0;
        var_delta_i_dn6 = 0.0;
        var_delta_i_dn7 = 0.0;
        var_delta_i_dn8 = 0.0;
        var_delta_i_dn9 = 0.0;
        var_delta_i_dn10 = 0.0;
        var_delta_i_dn11 = 0.0;
        var_delta_i_dn12 = 0.0;
        var_delta_i_dn13 = 0.0;
        var_delta_i_dn14 = 0.0;
        var_delta_i_rv = 0.0;

        let assign1920_e2778: f64 = (var_bin_l * p.p276);
        let assign1920_e2779: f64 = (p.p273 + assign1920_e2778);
        let assign1920_e2782: f64 = (var_bin_w * p.p277);
        let assign1920_e2783: f64 = (assign1920_e2779 + assign1920_e2782);
        let assign1920_e2786: f64 = (var_bin_wl * p.p278);
        let assign1920_e2787: f64 = (assign1920_e2783 + assign1920_e2786);
        var_u0_i = assign1920_e2787;
        var_u0_i_rv = 0.0;

        let assign1930_e2791: f64 = (var_bin_l * p.p291);
        let assign1930_e2792: f64 = (p.p284 + assign1930_e2791);
        let assign1930_e2795: f64 = (var_bin_w * p.p292);
        let assign1930_e2796: f64 = (assign1930_e2792 + assign1930_e2795);
        let assign1930_e2799: f64 = (var_bin_wl * p.p293);
        let assign1930_e2800: f64 = (assign1930_e2796 + assign1930_e2799);
        var_ua_i = assign1930_e2800;
        var_ua_i_dn0 = 0.0;
        var_ua_i_dn2 = 0.0;
        var_ua_i_dn3 = 0.0;
        var_ua_i_dn4 = 0.0;
        var_ua_i_dn5 = 0.0;
        var_ua_i_dn6 = 0.0;
        var_ua_i_dn7 = 0.0;
        var_ua_i_dn8 = 0.0;
        var_ua_i_dn9 = 0.0;
        var_ua_i_dn10 = 0.0;
        var_ua_i_dn11 = 0.0;
        var_ua_i_dn12 = 0.0;
        var_ua_i_dn13 = 0.0;
        var_ua_i_dn14 = 0.0;
        var_ua_i_rv = 0.0;

        let assign1940_e2804: f64 = (var_bin_l * p.p311);
        let assign1940_e2805: f64 = (p.p308 + assign1940_e2804);
        let assign1940_e2808: f64 = (var_bin_w * p.p312);
        let assign1940_e2809: f64 = (assign1940_e2805 + assign1940_e2808);
        let assign1940_e2812: f64 = (var_bin_wl * p.p313);
        let assign1940_e2813: f64 = (assign1940_e2809 + assign1940_e2812);
        var_ud_i = assign1940_e2813;
        var_ud_i_dn0 = 0.0;
        var_ud_i_dn2 = 0.0;
        var_ud_i_dn3 = 0.0;
        var_ud_i_dn4 = 0.0;
        var_ud_i_dn5 = 0.0;
        var_ud_i_dn6 = 0.0;
        var_ud_i_dn7 = 0.0;
        var_ud_i_dn8 = 0.0;
        var_ud_i_dn9 = 0.0;
        var_ud_i_dn10 = 0.0;
        var_ud_i_dn11 = 0.0;
        var_ud_i_dn12 = 0.0;
        var_ud_i_dn13 = 0.0;
        var_ud_i_dn14 = 0.0;
        var_ud_i_rv = 0.0;

        let assign1950_e2817: f64 = (var_bin_l * p.p299);
        let assign1950_e2818: f64 = (p.p298 + assign1950_e2817);
        let assign1950_e2821: f64 = (var_bin_w * p.p300);
        let assign1950_e2822: f64 = (assign1950_e2818 + assign1950_e2821);
        let assign1950_e2825: f64 = (var_bin_wl * p.p301);
        let assign1950_e2826: f64 = (assign1950_e2822 + assign1950_e2825);
        var_eu_i = assign1950_e2826;
        var_eu_i_dn0 = 0.0;
        var_eu_i_dn2 = 0.0;
        var_eu_i_dn3 = 0.0;
        var_eu_i_dn4 = 0.0;
        var_eu_i_dn5 = 0.0;
        var_eu_i_dn6 = 0.0;
        var_eu_i_dn7 = 0.0;
        var_eu_i_dn8 = 0.0;
        var_eu_i_dn9 = 0.0;
        var_eu_i_dn10 = 0.0;
        var_eu_i_dn11 = 0.0;
        var_eu_i_dn12 = 0.0;
        var_eu_i_dn13 = 0.0;
        var_eu_i_dn14 = 0.0;
        var_eu_i_rv = 0.0;

        let assign1960_e2830: f64 = (var_bin_l * p.p319);
        let assign1960_e2831: f64 = (p.p318 + assign1960_e2830);
        let assign1960_e2834: f64 = (var_bin_w * p.p320);
        let assign1960_e2835: f64 = (assign1960_e2831 + assign1960_e2834);
        let assign1960_e2838: f64 = (var_bin_wl * p.p321);
        let assign1960_e2839: f64 = (assign1960_e2835 + assign1960_e2838);
        var_ucs_i = assign1960_e2839;
        var_ucs_i_rv = 0.0;

        let assign1970_e2843: f64 = (var_bin_l * p.p333);
        let assign1970_e2844: f64 = (p.p326 + assign1970_e2843);
        let assign1970_e2847: f64 = (var_bin_w * p.p334);
        let assign1970_e2848: f64 = (assign1970_e2844 + assign1970_e2847);
        let assign1970_e2851: f64 = (var_bin_wl * p.p335);
        let assign1970_e2852: f64 = (assign1970_e2848 + assign1970_e2851);
        var_uc_i = assign1970_e2852;
        var_uc_i_dn0 = 0.0;
        var_uc_i_dn2 = 0.0;
        var_uc_i_dn3 = 0.0;
        var_uc_i_dn4 = 0.0;
        var_uc_i_dn5 = 0.0;
        var_uc_i_dn6 = 0.0;
        var_uc_i_dn7 = 0.0;
        var_uc_i_dn8 = 0.0;
        var_uc_i_dn9 = 0.0;
        var_uc_i_dn10 = 0.0;
        var_uc_i_dn11 = 0.0;
        var_uc_i_dn12 = 0.0;
        var_uc_i_dn13 = 0.0;
        var_uc_i_dn14 = 0.0;
        var_uc_i_rv = 0.0;

        let assign1980_e2856: f64 = (var_bin_l * p.p343);
        let assign1980_e2857: f64 = (p.p340 + assign1980_e2856);
        let assign1980_e2860: f64 = (var_bin_w * p.p344);
        let assign1980_e2861: f64 = (assign1980_e2857 + assign1980_e2860);
        let assign1980_e2864: f64 = (var_bin_wl * p.p345);
        let assign1980_e2865: f64 = (assign1980_e2861 + assign1980_e2864);
        var_pclm_i = assign1980_e2865;
        var_pclm_i_dn0 = 0.0;
        var_pclm_i_dn2 = 0.0;
        var_pclm_i_dn3 = 0.0;
        var_pclm_i_dn4 = 0.0;
        var_pclm_i_dn5 = 0.0;
        var_pclm_i_dn6 = 0.0;
        var_pclm_i_dn7 = 0.0;
        var_pclm_i_dn8 = 0.0;
        var_pclm_i_dn9 = 0.0;
        var_pclm_i_dn10 = 0.0;
        var_pclm_i_dn11 = 0.0;
        var_pclm_i_dn12 = 0.0;
        var_pclm_i_dn13 = 0.0;
        var_pclm_i_dn14 = 0.0;
        var_pclm_i_rv = 0.0;

        let assign1990_e2869: f64 = (var_bin_l * p.p354);
        let assign1990_e2870: f64 = (p.p351 + assign1990_e2869);
        let assign1990_e2873: f64 = (var_bin_w * p.p355);
        let assign1990_e2874: f64 = (assign1990_e2870 + assign1990_e2873);
        let assign1990_e2877: f64 = (var_bin_wl * p.p356);
        let assign1990_e2878: f64 = (assign1990_e2874 + assign1990_e2877);
        var_pclmcv_i = assign1990_e2878;
        var_pclmcv_i_rv = 0.0;

        let assign2000_e2882: f64 = (var_bin_l * p.p394);
        let assign2000_e2883: f64 = (p.p393 + assign2000_e2882);
        let assign2000_e2886: f64 = (var_bin_w * p.p395);
        let assign2000_e2887: f64 = (assign2000_e2883 + assign2000_e2886);
        let assign2000_e2890: f64 = (var_bin_wl * p.p396);
        let assign2000_e2891: f64 = (assign2000_e2887 + assign2000_e2890);
        var_rsw_i = assign2000_e2891;
        var_rsw_i_rv = 0.0;

        let assign2010_e2895: f64 = (var_bin_l * p.p404);
        let assign2010_e2896: f64 = (p.p403 + assign2010_e2895);
        let assign2010_e2899: f64 = (var_bin_w * p.p405);
        let assign2010_e2900: f64 = (assign2010_e2896 + assign2010_e2899);
        let assign2010_e2903: f64 = (var_bin_wl * p.p406);
        let assign2010_e2904: f64 = (assign2010_e2900 + assign2010_e2903);
        var_rdw_i = assign2010_e2904;
        var_rdw_i_rv = 0.0;

        let assign2020_e2908: f64 = (var_bin_l * p.p376);
        let assign2020_e2909: f64 = (p.p375 + assign2020_e2908);
        let assign2020_e2912: f64 = (var_bin_w * p.p377);
        let assign2020_e2913: f64 = (assign2020_e2909 + assign2020_e2912);
        let assign2020_e2916: f64 = (var_bin_wl * p.p378);
        let assign2020_e2917: f64 = (assign2020_e2913 + assign2020_e2916);
        var_prwg_i = assign2020_e2917;
        var_prwg_i_rv = 0.0;

        let assign2030_e2921: f64 = (var_bin_l * p.p380);
        let assign2030_e2922: f64 = (p.p379 + assign2030_e2921);
        let assign2030_e2925: f64 = (var_bin_w * p.p381);
        let assign2030_e2926: f64 = (assign2030_e2922 + assign2030_e2925);
        let assign2030_e2929: f64 = (var_bin_wl * p.p382);
        let assign2030_e2930: f64 = (assign2030_e2926 + assign2030_e2929);
        var_prwb_i = assign2030_e2930;
        var_prwb_i_rv = 0.0;

        let assign2040_e2934: f64 = (var_bin_l * p.p386);
        let assign2040_e2935: f64 = (p.p385 + assign2040_e2934);
        let assign2040_e2938: f64 = (var_bin_w * p.p387);
        let assign2040_e2939: f64 = (assign2040_e2935 + assign2040_e2938);
        let assign2040_e2942: f64 = (var_bin_wl * p.p388);
        let assign2040_e2943: f64 = (assign2040_e2939 + assign2040_e2942);
        var_wr_i = assign2040_e2943;
        var_wr_i_rv = 0.0;

        let assign2050_e2947: f64 = (var_bin_l * p.p390);
        let assign2050_e2948: f64 = (p.p389 + assign2050_e2947);
        let assign2050_e2951: f64 = (var_bin_w * p.p391);
        let assign2050_e2952: f64 = (assign2050_e2948 + assign2050_e2951);
        let assign2050_e2955: f64 = (var_bin_wl * p.p392);
        let assign2050_e2956: f64 = (assign2050_e2952 + assign2050_e2955);
        var_rswmin_i = assign2050_e2956;
        var_rswmin_i_rv = 0.0;

        let assign2060_e2960: f64 = (var_bin_l * p.p400);
        let assign2060_e2961: f64 = (p.p399 + assign2060_e2960);
        let assign2060_e2964: f64 = (var_bin_w * p.p401);
        let assign2060_e2965: f64 = (assign2060_e2961 + assign2060_e2964);
        let assign2060_e2968: f64 = (var_bin_wl * p.p402);
        let assign2060_e2969: f64 = (assign2060_e2965 + assign2060_e2968);
        var_rdwmin_i = assign2060_e2969;
        var_rdwmin_i_rv = 0.0;

        let assign2070_e2973: f64 = (var_bin_l * p.p416);
        let assign2070_e2974: f64 = (p.p413 + assign2070_e2973);
        let assign2070_e2977: f64 = (var_bin_w * p.p417);
        let assign2070_e2978: f64 = (assign2070_e2974 + assign2070_e2977);
        let assign2070_e2981: f64 = (var_bin_wl * p.p418);
        let assign2070_e2982: f64 = (assign2070_e2978 + assign2070_e2981);
        var_rdsw_i = assign2070_e2982;
        var_rdsw_i_rv = 0.0;

        let assign2080_e2986: f64 = (var_bin_l * p.p410);
        let assign2080_e2987: f64 = (p.p409 + assign2080_e2986);
        let assign2080_e2990: f64 = (var_bin_w * p.p411);
        let assign2080_e2991: f64 = (assign2080_e2987 + assign2080_e2990);
        let assign2080_e2994: f64 = (var_bin_wl * p.p412);
        let assign2080_e2995: f64 = (assign2080_e2991 + assign2080_e2994);
        var_rdswmin_i = assign2080_e2995;
        var_rdswmin_i_rv = 0.0;

        let assign2090_e2999: f64 = (var_bin_l * p.p435);
        let assign2090_e3000: f64 = (p.p434 + assign2090_e2999);
        let assign2090_e3003: f64 = (var_bin_w * p.p436);
        let assign2090_e3004: f64 = (assign2090_e3000 + assign2090_e3003);
        let assign2090_e3007: f64 = (var_bin_wl * p.p437);
        let assign2090_e3008: f64 = (assign2090_e3004 + assign2090_e3007);
        var_ptwg_i = assign2090_e3008;
        var_ptwg_i_dn0 = 0.0;
        var_ptwg_i_dn2 = 0.0;
        var_ptwg_i_dn3 = 0.0;
        var_ptwg_i_dn4 = 0.0;
        var_ptwg_i_dn5 = 0.0;
        var_ptwg_i_dn6 = 0.0;
        var_ptwg_i_dn7 = 0.0;
        var_ptwg_i_dn8 = 0.0;
        var_ptwg_i_dn9 = 0.0;
        var_ptwg_i_dn10 = 0.0;
        var_ptwg_i_dn11 = 0.0;
        var_ptwg_i_dn12 = 0.0;
        var_ptwg_i_dn13 = 0.0;
        var_ptwg_i_dn14 = 0.0;
        var_ptwg_i_rv = 0.0;

        let assign2100_e3012: f64 = (var_bin_l * p.p463);
        let assign2100_e3013: f64 = (p.p460 + assign2100_e3012);
        let assign2100_e3016: f64 = (var_bin_w * p.p464);
        let assign2100_e3017: f64 = (assign2100_e3013 + assign2100_e3016);
        let assign2100_e3020: f64 = (var_bin_wl * p.p465);
        let assign2100_e3021: f64 = (assign2100_e3017 + assign2100_e3020);
        var_pdiblc_i = assign2100_e3021;
        var_pdiblc_i_dn0 = 0.0;
        var_pdiblc_i_dn2 = 0.0;
        var_pdiblc_i_dn3 = 0.0;
        var_pdiblc_i_dn4 = 0.0;
        var_pdiblc_i_dn5 = 0.0;
        var_pdiblc_i_dn6 = 0.0;
        var_pdiblc_i_dn7 = 0.0;
        var_pdiblc_i_dn8 = 0.0;
        var_pdiblc_i_dn9 = 0.0;
        var_pdiblc_i_dn10 = 0.0;
        var_pdiblc_i_dn11 = 0.0;
        var_pdiblc_i_dn12 = 0.0;
        var_pdiblc_i_dn13 = 0.0;
        var_pdiblc_i_dn14 = 0.0;
        var_pdiblc_i_rv = 0.0;

        let assign2110_e3025: f64 = (var_bin_l * p.p471);
        let assign2110_e3026: f64 = (p.p470 + assign2110_e3025);
        let assign2110_e3029: f64 = (var_bin_w * p.p472);
        let assign2110_e3030: f64 = (assign2110_e3026 + assign2110_e3029);
        let assign2110_e3033: f64 = (var_bin_wl * p.p473);
        let assign2110_e3034: f64 = (assign2110_e3030 + assign2110_e3033);
        var_pdiblcb_i = assign2110_e3034;
        var_pdiblcb_i_rv = 0.0;

        let assign2120_e3038: f64 = (var_bin_l * p.p358);
        let assign2120_e3039: f64 = (p.p357 + assign2120_e3038);
        let assign2120_e3042: f64 = (var_bin_w * p.p359);
        let assign2120_e3043: f64 = (assign2120_e3039 + assign2120_e3042);
        let assign2120_e3046: f64 = (var_bin_wl * p.p360);
        let assign2120_e3047: f64 = (assign2120_e3043 + assign2120_e3046);
        var_pscbe1_i = assign2120_e3047;
        var_pscbe1_i_rv = 0.0;

        let assign2130_e3051: f64 = (var_bin_l * p.p362);
        let assign2130_e3052: f64 = (p.p361 + assign2130_e3051);
        let assign2130_e3055: f64 = (var_bin_w * p.p363);
        let assign2130_e3056: f64 = (assign2130_e3052 + assign2130_e3055);
        let assign2130_e3059: f64 = (var_bin_wl * p.p364);
        let assign2130_e3060: f64 = (assign2130_e3056 + assign2130_e3059);
        var_pscbe2_i = assign2130_e3060;
        var_pscbe2_i_rv = 0.0;

        let assign2140_e3064: f64 = (var_bin_l * p.p366);
        let assign2140_e3065: f64 = (p.p365 + assign2140_e3064);
        let assign2140_e3068: f64 = (var_bin_w * p.p367);
        let assign2140_e3069: f64 = (assign2140_e3065 + assign2140_e3068);
        let assign2140_e3072: f64 = (var_bin_wl * p.p368);
        let assign2140_e3073: f64 = (assign2140_e3069 + assign2140_e3072);
        var_pdits_i = assign2140_e3073;
        var_pdits_i_rv = 0.0;

        let assign2150_e3077: f64 = (var_bin_l * p.p371);
        let assign2150_e3078: f64 = (p.p370 + assign2150_e3077);
        let assign2150_e3081: f64 = (var_bin_w * p.p372);
        let assign2150_e3082: f64 = (assign2150_e3078 + assign2150_e3081);
        let assign2150_e3085: f64 = (var_bin_wl * p.p373);
        let assign2150_e3086: f64 = (assign2150_e3082 + assign2150_e3085);
        var_pditsd_i = assign2150_e3086;
        var_pditsd_i_rv = 0.0;

        let assign2160_e3090: f64 = (var_bin_l * p.p481);
        let assign2160_e3091: f64 = (p.p478 + assign2160_e3090);
        let assign2160_e3094: f64 = (var_bin_w * p.p482);
        let assign2160_e3095: f64 = (assign2160_e3091 + assign2160_e3094);
        let assign2160_e3098: f64 = (var_bin_wl * p.p483);
        let assign2160_e3099: f64 = (assign2160_e3095 + assign2160_e3098);
        var_fprout_i = assign2160_e3099;
        var_fprout_i_rv = 0.0;

        let assign2170_e3103: f64 = (var_bin_l * p.p475);
        let assign2170_e3104: f64 = (p.p474 + assign2170_e3103);
        let assign2170_e3107: f64 = (var_bin_w * p.p476);
        let assign2170_e3108: f64 = (assign2170_e3104 + assign2170_e3107);
        let assign2170_e3111: f64 = (var_bin_wl * p.p477);
        let assign2170_e3112: f64 = (assign2170_e3108 + assign2170_e3111);
        var_pvag_i = assign2170_e3112;
        var_pvag_i_rv = 0.0;

        let assign2180_e3116: f64 = (var_bin_l * p.p240);
        let assign2180_e3117: f64 = (p.p239 + assign2180_e3116);
        let assign2180_e3120: f64 = (var_bin_w * p.p241);
        let assign2180_e3121: f64 = (assign2180_e3117 + assign2180_e3120);
        let assign2180_e3124: f64 = (var_bin_wl * p.p242);
        let assign2180_e3125: f64 = (assign2180_e3121 + assign2180_e3124);
        var_vsat_i = assign2180_e3125;
        var_vsat_i_dn0 = 0.0;
        var_vsat_i_dn2 = 0.0;
        var_vsat_i_dn3 = 0.0;
        var_vsat_i_dn4 = 0.0;
        var_vsat_i_dn5 = 0.0;
        var_vsat_i_dn6 = 0.0;
        var_vsat_i_dn7 = 0.0;
        var_vsat_i_dn8 = 0.0;
        var_vsat_i_dn9 = 0.0;
        var_vsat_i_dn10 = 0.0;
        var_vsat_i_dn11 = 0.0;
        var_vsat_i_dn12 = 0.0;
        var_vsat_i_dn13 = 0.0;
        var_vsat_i_dn14 = 0.0;
        var_vsat_i_rv = 0.0;

        let assign2190_e3129: f64 = (var_bin_l * p.p420);
        let assign2190_e3130: f64 = (p.p419 + assign2190_e3129);
        let assign2190_e3133: f64 = (var_bin_w * p.p421);
        let assign2190_e3134: f64 = (assign2190_e3130 + assign2190_e3133);
        let assign2190_e3137: f64 = (var_bin_wl * p.p422);
        let assign2190_e3138: f64 = (assign2190_e3134 + assign2190_e3137);
        var_psat_i = assign2190_e3138;
        var_psat_i_rv = 0.0;

        let assign2200_e3142: f64 = (var_bin_l * p.p260);
        let assign2200_e3143: f64 = (p.p259 + assign2200_e3142);
        let assign2200_e3146: f64 = (var_bin_w * p.p261);
        let assign2200_e3147: f64 = (assign2200_e3143 + assign2200_e3146);
        let assign2200_e3150: f64 = (var_bin_wl * p.p262);
        let assign2200_e3151: f64 = (assign2200_e3147 + assign2200_e3150);
        var_vsatcv_i = assign2200_e3151;
        var_vsatcv_i_dn0 = 0.0;
        var_vsatcv_i_dn2 = 0.0;
        var_vsatcv_i_dn3 = 0.0;
        var_vsatcv_i_dn4 = 0.0;
        var_vsatcv_i_dn5 = 0.0;
        var_vsatcv_i_dn6 = 0.0;
        var_vsatcv_i_dn7 = 0.0;
        var_vsatcv_i_dn8 = 0.0;
        var_vsatcv_i_dn9 = 0.0;
        var_vsatcv_i_dn10 = 0.0;
        var_vsatcv_i_dn11 = 0.0;
        var_vsatcv_i_dn12 = 0.0;
        var_vsatcv_i_dn13 = 0.0;
        var_vsatcv_i_dn14 = 0.0;
        var_vsatcv_i_rv = 0.0;

        *var_delta_i_slot = var_delta_i;
        *var_delta_i_dn0_slot = var_delta_i_dn0;
        *var_delta_i_dn10_slot = var_delta_i_dn10;
        *var_delta_i_dn11_slot = var_delta_i_dn11;
        *var_delta_i_dn12_slot = var_delta_i_dn12;
        *var_delta_i_dn13_slot = var_delta_i_dn13;
        *var_delta_i_dn14_slot = var_delta_i_dn14;
        *var_delta_i_dn2_slot = var_delta_i_dn2;
        *var_delta_i_dn3_slot = var_delta_i_dn3;
        *var_delta_i_dn4_slot = var_delta_i_dn4;
        *var_delta_i_dn5_slot = var_delta_i_dn5;
        *var_delta_i_dn6_slot = var_delta_i_dn6;
        *var_delta_i_dn7_slot = var_delta_i_dn7;
        *var_delta_i_dn8_slot = var_delta_i_dn8;
        *var_delta_i_dn9_slot = var_delta_i_dn9;
        *var_delta_i_rv_slot = var_delta_i_rv;
        *var_eta0_i_slot = var_eta0_i;
        *var_eta0_i_dn0_slot = var_eta0_i_dn0;
        *var_eta0_i_dn10_slot = var_eta0_i_dn10;
        *var_eta0_i_dn11_slot = var_eta0_i_dn11;
        *var_eta0_i_dn12_slot = var_eta0_i_dn12;
        *var_eta0_i_dn13_slot = var_eta0_i_dn13;
        *var_eta0_i_dn14_slot = var_eta0_i_dn14;
        *var_eta0_i_dn2_slot = var_eta0_i_dn2;
        *var_eta0_i_dn3_slot = var_eta0_i_dn3;
        *var_eta0_i_dn4_slot = var_eta0_i_dn4;
        *var_eta0_i_dn5_slot = var_eta0_i_dn5;
        *var_eta0_i_dn6_slot = var_eta0_i_dn6;
        *var_eta0_i_dn7_slot = var_eta0_i_dn7;
        *var_eta0_i_dn8_slot = var_eta0_i_dn8;
        *var_eta0_i_dn9_slot = var_eta0_i_dn9;
        *var_eta0_i_rv_slot = var_eta0_i_rv;
        *var_etab_i_slot = var_etab_i;
        *var_etab_i_rv_slot = var_etab_i_rv;
        *var_eu_i_slot = var_eu_i;
        *var_eu_i_dn0_slot = var_eu_i_dn0;
        *var_eu_i_dn10_slot = var_eu_i_dn10;
        *var_eu_i_dn11_slot = var_eu_i_dn11;
        *var_eu_i_dn12_slot = var_eu_i_dn12;
        *var_eu_i_dn13_slot = var_eu_i_dn13;
        *var_eu_i_dn14_slot = var_eu_i_dn14;
        *var_eu_i_dn2_slot = var_eu_i_dn2;
        *var_eu_i_dn3_slot = var_eu_i_dn3;
        *var_eu_i_dn4_slot = var_eu_i_dn4;
        *var_eu_i_dn5_slot = var_eu_i_dn5;
        *var_eu_i_dn6_slot = var_eu_i_dn6;
        *var_eu_i_dn7_slot = var_eu_i_dn7;
        *var_eu_i_dn8_slot = var_eu_i_dn8;
        *var_eu_i_dn9_slot = var_eu_i_dn9;
        *var_eu_i_rv_slot = var_eu_i_rv;
        *var_fprout_i_slot = var_fprout_i;
        *var_fprout_i_rv_slot = var_fprout_i_rv;
        *var_k1_i_slot = var_k1_i;
        *var_k1_i_dn0_slot = var_k1_i_dn0;
        *var_k1_i_dn10_slot = var_k1_i_dn10;
        *var_k1_i_dn11_slot = var_k1_i_dn11;
        *var_k1_i_dn12_slot = var_k1_i_dn12;
        *var_k1_i_dn13_slot = var_k1_i_dn13;
        *var_k1_i_dn14_slot = var_k1_i_dn14;
        *var_k1_i_dn2_slot = var_k1_i_dn2;
        *var_k1_i_dn3_slot = var_k1_i_dn3;
        *var_k1_i_dn4_slot = var_k1_i_dn4;
        *var_k1_i_dn5_slot = var_k1_i_dn5;
        *var_k1_i_dn6_slot = var_k1_i_dn6;
        *var_k1_i_dn7_slot = var_k1_i_dn7;
        *var_k1_i_dn8_slot = var_k1_i_dn8;
        *var_k1_i_dn9_slot = var_k1_i_dn9;
        *var_k1_i_rv_slot = var_k1_i_rv;
        *var_k2_i_slot = var_k2_i;
        *var_k2_i_dn0_slot = var_k2_i_dn0;
        *var_k2_i_dn10_slot = var_k2_i_dn10;
        *var_k2_i_dn11_slot = var_k2_i_dn11;
        *var_k2_i_dn12_slot = var_k2_i_dn12;
        *var_k2_i_dn13_slot = var_k2_i_dn13;
        *var_k2_i_dn14_slot = var_k2_i_dn14;
        *var_k2_i_dn2_slot = var_k2_i_dn2;
        *var_k2_i_dn3_slot = var_k2_i_dn3;
        *var_k2_i_dn4_slot = var_k2_i_dn4;
        *var_k2_i_dn5_slot = var_k2_i_dn5;
        *var_k2_i_dn6_slot = var_k2_i_dn6;
        *var_k2_i_dn7_slot = var_k2_i_dn7;
        *var_k2_i_dn8_slot = var_k2_i_dn8;
        *var_k2_i_dn9_slot = var_k2_i_dn9;
        *var_k2_i_rv_slot = var_k2_i_rv;
        *var_pclm_i_slot = var_pclm_i;
        *var_pclm_i_dn0_slot = var_pclm_i_dn0;
        *var_pclm_i_dn10_slot = var_pclm_i_dn10;
        *var_pclm_i_dn11_slot = var_pclm_i_dn11;
        *var_pclm_i_dn12_slot = var_pclm_i_dn12;
        *var_pclm_i_dn13_slot = var_pclm_i_dn13;
        *var_pclm_i_dn14_slot = var_pclm_i_dn14;
        *var_pclm_i_dn2_slot = var_pclm_i_dn2;
        *var_pclm_i_dn3_slot = var_pclm_i_dn3;
        *var_pclm_i_dn4_slot = var_pclm_i_dn4;
        *var_pclm_i_dn5_slot = var_pclm_i_dn5;
        *var_pclm_i_dn6_slot = var_pclm_i_dn6;
        *var_pclm_i_dn7_slot = var_pclm_i_dn7;
        *var_pclm_i_dn8_slot = var_pclm_i_dn8;
        *var_pclm_i_dn9_slot = var_pclm_i_dn9;
        *var_pclm_i_rv_slot = var_pclm_i_rv;
        *var_pclmcv_i_slot = var_pclmcv_i;
        *var_pclmcv_i_rv_slot = var_pclmcv_i_rv;
        *var_pdiblc_i_slot = var_pdiblc_i;
        *var_pdiblc_i_dn0_slot = var_pdiblc_i_dn0;
        *var_pdiblc_i_dn10_slot = var_pdiblc_i_dn10;
        *var_pdiblc_i_dn11_slot = var_pdiblc_i_dn11;
        *var_pdiblc_i_dn12_slot = var_pdiblc_i_dn12;
        *var_pdiblc_i_dn13_slot = var_pdiblc_i_dn13;
        *var_pdiblc_i_dn14_slot = var_pdiblc_i_dn14;
        *var_pdiblc_i_dn2_slot = var_pdiblc_i_dn2;
        *var_pdiblc_i_dn3_slot = var_pdiblc_i_dn3;
        *var_pdiblc_i_dn4_slot = var_pdiblc_i_dn4;
        *var_pdiblc_i_dn5_slot = var_pdiblc_i_dn5;
        *var_pdiblc_i_dn6_slot = var_pdiblc_i_dn6;
        *var_pdiblc_i_dn7_slot = var_pdiblc_i_dn7;
        *var_pdiblc_i_dn8_slot = var_pdiblc_i_dn8;
        *var_pdiblc_i_dn9_slot = var_pdiblc_i_dn9;
        *var_pdiblc_i_rv_slot = var_pdiblc_i_rv;
        *var_pdiblcb_i_slot = var_pdiblcb_i;
        *var_pdiblcb_i_rv_slot = var_pdiblcb_i_rv;
        *var_pdits_i_slot = var_pdits_i;
        *var_pdits_i_rv_slot = var_pdits_i_rv;
        *var_pditsd_i_slot = var_pditsd_i;
        *var_pditsd_i_rv_slot = var_pditsd_i_rv;
        *var_phin_i_slot = var_phin_i;
        *var_phin_i_rv_slot = var_phin_i_rv;
        *var_prwb_i_slot = var_prwb_i;
        *var_prwb_i_rv_slot = var_prwb_i_rv;
        *var_prwg_i_slot = var_prwg_i;
        *var_prwg_i_rv_slot = var_prwg_i_rv;
        *var_psat_i_slot = var_psat_i;
        *var_psat_i_rv_slot = var_psat_i_rv;
        *var_pscbe1_i_slot = var_pscbe1_i;
        *var_pscbe1_i_rv_slot = var_pscbe1_i_rv;
        *var_pscbe2_i_slot = var_pscbe2_i;
        *var_pscbe2_i_rv_slot = var_pscbe2_i_rv;
        *var_ptwg_i_slot = var_ptwg_i;
        *var_ptwg_i_dn0_slot = var_ptwg_i_dn0;
        *var_ptwg_i_dn10_slot = var_ptwg_i_dn10;
        *var_ptwg_i_dn11_slot = var_ptwg_i_dn11;
        *var_ptwg_i_dn12_slot = var_ptwg_i_dn12;
        *var_ptwg_i_dn13_slot = var_ptwg_i_dn13;
        *var_ptwg_i_dn14_slot = var_ptwg_i_dn14;
        *var_ptwg_i_dn2_slot = var_ptwg_i_dn2;
        *var_ptwg_i_dn3_slot = var_ptwg_i_dn3;
        *var_ptwg_i_dn4_slot = var_ptwg_i_dn4;
        *var_ptwg_i_dn5_slot = var_ptwg_i_dn5;
        *var_ptwg_i_dn6_slot = var_ptwg_i_dn6;
        *var_ptwg_i_dn7_slot = var_ptwg_i_dn7;
        *var_ptwg_i_dn8_slot = var_ptwg_i_dn8;
        *var_ptwg_i_dn9_slot = var_ptwg_i_dn9;
        *var_ptwg_i_rv_slot = var_ptwg_i_rv;
        *var_pvag_i_slot = var_pvag_i;
        *var_pvag_i_rv_slot = var_pvag_i_rv;
        *var_rdsw_i_slot = var_rdsw_i;
        *var_rdsw_i_rv_slot = var_rdsw_i_rv;
        *var_rdswmin_i_slot = var_rdswmin_i;
        *var_rdswmin_i_rv_slot = var_rdswmin_i_rv;
        *var_rdw_i_slot = var_rdw_i;
        *var_rdw_i_rv_slot = var_rdw_i_rv;
        *var_rdwmin_i_slot = var_rdwmin_i;
        *var_rdwmin_i_rv_slot = var_rdwmin_i_rv;
        *var_rsw_i_slot = var_rsw_i;
        *var_rsw_i_rv_slot = var_rsw_i_rv;
        *var_rswmin_i_slot = var_rswmin_i;
        *var_rswmin_i_rv_slot = var_rswmin_i_rv;
        *var_u0_i_slot = var_u0_i;
        *var_u0_i_rv_slot = var_u0_i_rv;
        *var_ua_i_slot = var_ua_i;
        *var_ua_i_dn0_slot = var_ua_i_dn0;
        *var_ua_i_dn10_slot = var_ua_i_dn10;
        *var_ua_i_dn11_slot = var_ua_i_dn11;
        *var_ua_i_dn12_slot = var_ua_i_dn12;
        *var_ua_i_dn13_slot = var_ua_i_dn13;
        *var_ua_i_dn14_slot = var_ua_i_dn14;
        *var_ua_i_dn2_slot = var_ua_i_dn2;
        *var_ua_i_dn3_slot = var_ua_i_dn3;
        *var_ua_i_dn4_slot = var_ua_i_dn4;
        *var_ua_i_dn5_slot = var_ua_i_dn5;
        *var_ua_i_dn6_slot = var_ua_i_dn6;
        *var_ua_i_dn7_slot = var_ua_i_dn7;
        *var_ua_i_dn8_slot = var_ua_i_dn8;
        *var_ua_i_dn9_slot = var_ua_i_dn9;
        *var_ua_i_rv_slot = var_ua_i_rv;
        *var_uc_i_slot = var_uc_i;
        *var_uc_i_dn0_slot = var_uc_i_dn0;
        *var_uc_i_dn10_slot = var_uc_i_dn10;
        *var_uc_i_dn11_slot = var_uc_i_dn11;
        *var_uc_i_dn12_slot = var_uc_i_dn12;
        *var_uc_i_dn13_slot = var_uc_i_dn13;
        *var_uc_i_dn14_slot = var_uc_i_dn14;
        *var_uc_i_dn2_slot = var_uc_i_dn2;
        *var_uc_i_dn3_slot = var_uc_i_dn3;
        *var_uc_i_dn4_slot = var_uc_i_dn4;
        *var_uc_i_dn5_slot = var_uc_i_dn5;
        *var_uc_i_dn6_slot = var_uc_i_dn6;
        *var_uc_i_dn7_slot = var_uc_i_dn7;
        *var_uc_i_dn8_slot = var_uc_i_dn8;
        *var_uc_i_dn9_slot = var_uc_i_dn9;
        *var_uc_i_rv_slot = var_uc_i_rv;
        *var_ucs_i_slot = var_ucs_i;
        *var_ucs_i_rv_slot = var_ucs_i_rv;
        *var_ud_i_slot = var_ud_i;
        *var_ud_i_dn0_slot = var_ud_i_dn0;
        *var_ud_i_dn10_slot = var_ud_i_dn10;
        *var_ud_i_dn11_slot = var_ud_i_dn11;
        *var_ud_i_dn12_slot = var_ud_i_dn12;
        *var_ud_i_dn13_slot = var_ud_i_dn13;
        *var_ud_i_dn14_slot = var_ud_i_dn14;
        *var_ud_i_dn2_slot = var_ud_i_dn2;
        *var_ud_i_dn3_slot = var_ud_i_dn3;
        *var_ud_i_dn4_slot = var_ud_i_dn4;
        *var_ud_i_dn5_slot = var_ud_i_dn5;
        *var_ud_i_dn6_slot = var_ud_i_dn6;
        *var_ud_i_dn7_slot = var_ud_i_dn7;
        *var_ud_i_dn8_slot = var_ud_i_dn8;
        *var_ud_i_dn9_slot = var_ud_i_dn9;
        *var_ud_i_rv_slot = var_ud_i_rv;
        *var_vsat_i_slot = var_vsat_i;
        *var_vsat_i_dn0_slot = var_vsat_i_dn0;
        *var_vsat_i_dn10_slot = var_vsat_i_dn10;
        *var_vsat_i_dn11_slot = var_vsat_i_dn11;
        *var_vsat_i_dn12_slot = var_vsat_i_dn12;
        *var_vsat_i_dn13_slot = var_vsat_i_dn13;
        *var_vsat_i_dn14_slot = var_vsat_i_dn14;
        *var_vsat_i_dn2_slot = var_vsat_i_dn2;
        *var_vsat_i_dn3_slot = var_vsat_i_dn3;
        *var_vsat_i_dn4_slot = var_vsat_i_dn4;
        *var_vsat_i_dn5_slot = var_vsat_i_dn5;
        *var_vsat_i_dn6_slot = var_vsat_i_dn6;
        *var_vsat_i_dn7_slot = var_vsat_i_dn7;
        *var_vsat_i_dn8_slot = var_vsat_i_dn8;
        *var_vsat_i_dn9_slot = var_vsat_i_dn9;
        *var_vsat_i_rv_slot = var_vsat_i_rv;
        *var_vsatcv_i_slot = var_vsatcv_i;
        *var_vsatcv_i_dn0_slot = var_vsatcv_i_dn0;
        *var_vsatcv_i_dn10_slot = var_vsatcv_i_dn10;
        *var_vsatcv_i_dn11_slot = var_vsatcv_i_dn11;
        *var_vsatcv_i_dn12_slot = var_vsatcv_i_dn12;
        *var_vsatcv_i_dn13_slot = var_vsatcv_i_dn13;
        *var_vsatcv_i_dn14_slot = var_vsatcv_i_dn14;
        *var_vsatcv_i_dn2_slot = var_vsatcv_i_dn2;
        *var_vsatcv_i_dn3_slot = var_vsatcv_i_dn3;
        *var_vsatcv_i_dn4_slot = var_vsatcv_i_dn4;
        *var_vsatcv_i_dn5_slot = var_vsatcv_i_dn5;
        *var_vsatcv_i_dn6_slot = var_vsatcv_i_dn6;
        *var_vsatcv_i_dn7_slot = var_vsatcv_i_dn7;
        *var_vsatcv_i_dn8_slot = var_vsatcv_i_dn8;
        *var_vsatcv_i_dn9_slot = var_vsatcv_i_dn9;
        *var_vsatcv_i_rv_slot = var_vsatcv_i_rv;
        *var_wr_i_slot = var_wr_i;
        *var_wr_i_rv_slot = var_wr_i_rv;
        *var_xj_i_slot = var_xj_i;
        *var_xj_i_rv_slot = var_xj_i_rv;
    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        var_bin_l: f64,
        var_bin_w: f64,
        var_bin_wl: f64,
        var_a11_i_slot: &mut f64,
        var_a11_i_rv_slot: &mut f64,
        var_a1_i_slot: &mut f64,
        var_a1_i_rv_slot: &mut f64,
        var_agidl_i_slot: &mut f64,
        var_agidl_i_rv_slot: &mut f64,
        var_agisl_i_slot: &mut f64,
        var_agisl_i_rv_slot: &mut f64,
        var_aigbacc_i_slot: &mut f64,
        var_aigbacc_i_rv_slot: &mut f64,
        var_aigbinv_i_slot: &mut f64,
        var_aigbinv_i_rv_slot: &mut f64,
        var_aigc_i_slot: &mut f64,
        var_aigc_i_rv_slot: &mut f64,
        var_aigd_i_slot: &mut f64,
        var_aigd_i_rv_slot: &mut f64,
        var_aigs_i_slot: &mut f64,
        var_aigs_i_rv_slot: &mut f64,
        var_alpha0_i_slot: &mut f64,
        var_alpha0_i_dn0_slot: &mut f64,
        var_alpha0_i_dn10_slot: &mut f64,
        var_alpha0_i_dn11_slot: &mut f64,
        var_alpha0_i_dn12_slot: &mut f64,
        var_alpha0_i_dn13_slot: &mut f64,
        var_alpha0_i_dn14_slot: &mut f64,
        var_alpha0_i_dn2_slot: &mut f64,
        var_alpha0_i_dn3_slot: &mut f64,
        var_alpha0_i_dn4_slot: &mut f64,
        var_alpha0_i_dn5_slot: &mut f64,
        var_alpha0_i_dn6_slot: &mut f64,
        var_alpha0_i_dn7_slot: &mut f64,
        var_alpha0_i_dn8_slot: &mut f64,
        var_alpha0_i_dn9_slot: &mut f64,
        var_alpha0_i_rv_slot: &mut f64,
        var_at_i_slot: &mut f64,
        var_at_i_rv_slot: &mut f64,
        var_beta0_i_slot: &mut f64,
        var_beta0_i_dn0_slot: &mut f64,
        var_beta0_i_dn10_slot: &mut f64,
        var_beta0_i_dn11_slot: &mut f64,
        var_beta0_i_dn12_slot: &mut f64,
        var_beta0_i_dn13_slot: &mut f64,
        var_beta0_i_dn14_slot: &mut f64,
        var_beta0_i_dn2_slot: &mut f64,
        var_beta0_i_dn3_slot: &mut f64,
        var_beta0_i_dn4_slot: &mut f64,
        var_beta0_i_dn5_slot: &mut f64,
        var_beta0_i_dn6_slot: &mut f64,
        var_beta0_i_dn7_slot: &mut f64,
        var_beta0_i_dn8_slot: &mut f64,
        var_beta0_i_dn9_slot: &mut f64,
        var_beta0_i_rv_slot: &mut f64,
        var_bgidl_i_slot: &mut f64,
        var_bgidl_i_rv_slot: &mut f64,
        var_bgisl_i_slot: &mut f64,
        var_bgisl_i_rv_slot: &mut f64,
        var_bigbacc_i_slot: &mut f64,
        var_bigbacc_i_rv_slot: &mut f64,
        var_bigbinv_i_slot: &mut f64,
        var_bigbinv_i_rv_slot: &mut f64,
        var_bigc_i_slot: &mut f64,
        var_bigc_i_rv_slot: &mut f64,
        var_bigd_i_slot: &mut f64,
        var_bigd_i_rv_slot: &mut f64,
        var_bigs_i_slot: &mut f64,
        var_bigs_i_rv_slot: &mut f64,
        var_cf_i_slot: &mut f64,
        var_cf_i_rv_slot: &mut f64,
        var_cgdl_i_slot: &mut f64,
        var_cgdl_i_rv_slot: &mut f64,
        var_cgidl_i_slot: &mut f64,
        var_cgidl_i_rv_slot: &mut f64,
        var_cgisl_i_slot: &mut f64,
        var_cgisl_i_rv_slot: &mut f64,
        var_cgsl_i_slot: &mut f64,
        var_cgsl_i_rv_slot: &mut f64,
        var_cigbacc_i_slot: &mut f64,
        var_cigbacc_i_rv_slot: &mut f64,
        var_cigbinv_i_slot: &mut f64,
        var_cigbinv_i_rv_slot: &mut f64,
        var_cigc_i_slot: &mut f64,
        var_cigc_i_rv_slot: &mut f64,
        var_cigd_i_slot: &mut f64,
        var_cigd_i_rv_slot: &mut f64,
        var_cigs_i_slot: &mut f64,
        var_cigs_i_rv_slot: &mut f64,
        var_ckappad_i_slot: &mut f64,
        var_ckappad_i_rv_slot: &mut f64,
        var_ckappas_i_slot: &mut f64,
        var_ckappas_i_rv_slot: &mut f64,
        var_egidl_i_slot: &mut f64,
        var_egidl_i_rv_slot: &mut f64,
        var_egisl_i_slot: &mut f64,
        var_egisl_i_rv_slot: &mut f64,
        var_eigbinv_i_slot: &mut f64,
        var_eigbinv_i_rv_slot: &mut f64,
        var_eu1_i_slot: &mut f64,
        var_eu1_i_rv_slot: &mut f64,
        var_iit_i_slot: &mut f64,
        var_iit_i_rv_slot: &mut f64,
        var_k2we_i_slot: &mut f64,
        var_k2we_i_rv_slot: &mut f64,
        var_kt1_i_slot: &mut f64,
        var_kt1_i_rv_slot: &mut f64,
        var_kt2_i_slot: &mut f64,
        var_kt2_i_rv_slot: &mut f64,
        var_ku0we_i_slot: &mut f64,
        var_ku0we_i_rv_slot: &mut f64,
        var_kvth0we_i_slot: &mut f64,
        var_kvth0we_i_rv_slot: &mut f64,
        var_nigbacc_i_slot: &mut f64,
        var_nigbacc_i_rv_slot: &mut f64,
        var_nigbinv_i_slot: &mut f64,
        var_nigbinv_i_rv_slot: &mut f64,
        var_poxedge_i_slot: &mut f64,
        var_poxedge_i_rv_slot: &mut f64,
        var_prt_i_slot: &mut f64,
        var_prt_i_rv_slot: &mut f64,
        var_psatb_i_slot: &mut f64,
        var_psatb_i_rv_slot: &mut f64,
        var_ptwgt_i_slot: &mut f64,
        var_ptwgt_i_rv_slot: &mut f64,
        var_tgidl_i_slot: &mut f64,
        var_tgidl_i_rv_slot: &mut f64,
        var_ua1_i_slot: &mut f64,
        var_ua1_i_rv_slot: &mut f64,
        var_uc1_i_slot: &mut f64,
        var_uc1_i_rv_slot: &mut f64,
        var_ucste_i_slot: &mut f64,
        var_ucste_i_rv_slot: &mut f64,
        var_ud1_i_slot: &mut f64,
        var_ud1_i_rv_slot: &mut f64,
        var_ute_i_slot: &mut f64,
        var_ute_i_rv_slot: &mut f64,
    ) {
        let mut var_a11_i: f64 = *var_a11_i_slot;
        let mut var_a11_i_rv: f64 = *var_a11_i_rv_slot;
        let mut var_a1_i: f64 = *var_a1_i_slot;
        let mut var_a1_i_rv: f64 = *var_a1_i_rv_slot;
        let mut var_agidl_i: f64 = *var_agidl_i_slot;
        let mut var_agidl_i_rv: f64 = *var_agidl_i_rv_slot;
        let mut var_agisl_i: f64 = *var_agisl_i_slot;
        let mut var_agisl_i_rv: f64 = *var_agisl_i_rv_slot;
        let mut var_aigbacc_i: f64 = *var_aigbacc_i_slot;
        let mut var_aigbacc_i_rv: f64 = *var_aigbacc_i_rv_slot;
        let mut var_aigbinv_i: f64 = *var_aigbinv_i_slot;
        let mut var_aigbinv_i_rv: f64 = *var_aigbinv_i_rv_slot;
        let mut var_aigc_i: f64 = *var_aigc_i_slot;
        let mut var_aigc_i_rv: f64 = *var_aigc_i_rv_slot;
        let mut var_aigd_i: f64 = *var_aigd_i_slot;
        let mut var_aigd_i_rv: f64 = *var_aigd_i_rv_slot;
        let mut var_aigs_i: f64 = *var_aigs_i_slot;
        let mut var_aigs_i_rv: f64 = *var_aigs_i_rv_slot;
        let mut var_alpha0_i: f64 = *var_alpha0_i_slot;
        let mut var_alpha0_i_dn0: f64 = *var_alpha0_i_dn0_slot;
        let mut var_alpha0_i_dn10: f64 = *var_alpha0_i_dn10_slot;
        let mut var_alpha0_i_dn11: f64 = *var_alpha0_i_dn11_slot;
        let mut var_alpha0_i_dn12: f64 = *var_alpha0_i_dn12_slot;
        let mut var_alpha0_i_dn13: f64 = *var_alpha0_i_dn13_slot;
        let mut var_alpha0_i_dn14: f64 = *var_alpha0_i_dn14_slot;
        let mut var_alpha0_i_dn2: f64 = *var_alpha0_i_dn2_slot;
        let mut var_alpha0_i_dn3: f64 = *var_alpha0_i_dn3_slot;
        let mut var_alpha0_i_dn4: f64 = *var_alpha0_i_dn4_slot;
        let mut var_alpha0_i_dn5: f64 = *var_alpha0_i_dn5_slot;
        let mut var_alpha0_i_dn6: f64 = *var_alpha0_i_dn6_slot;
        let mut var_alpha0_i_dn7: f64 = *var_alpha0_i_dn7_slot;
        let mut var_alpha0_i_dn8: f64 = *var_alpha0_i_dn8_slot;
        let mut var_alpha0_i_dn9: f64 = *var_alpha0_i_dn9_slot;
        let mut var_alpha0_i_rv: f64 = *var_alpha0_i_rv_slot;
        let mut var_at_i: f64 = *var_at_i_slot;
        let mut var_at_i_rv: f64 = *var_at_i_rv_slot;
        let mut var_beta0_i: f64 = *var_beta0_i_slot;
        let mut var_beta0_i_dn0: f64 = *var_beta0_i_dn0_slot;
        let mut var_beta0_i_dn10: f64 = *var_beta0_i_dn10_slot;
        let mut var_beta0_i_dn11: f64 = *var_beta0_i_dn11_slot;
        let mut var_beta0_i_dn12: f64 = *var_beta0_i_dn12_slot;
        let mut var_beta0_i_dn13: f64 = *var_beta0_i_dn13_slot;
        let mut var_beta0_i_dn14: f64 = *var_beta0_i_dn14_slot;
        let mut var_beta0_i_dn2: f64 = *var_beta0_i_dn2_slot;
        let mut var_beta0_i_dn3: f64 = *var_beta0_i_dn3_slot;
        let mut var_beta0_i_dn4: f64 = *var_beta0_i_dn4_slot;
        let mut var_beta0_i_dn5: f64 = *var_beta0_i_dn5_slot;
        let mut var_beta0_i_dn6: f64 = *var_beta0_i_dn6_slot;
        let mut var_beta0_i_dn7: f64 = *var_beta0_i_dn7_slot;
        let mut var_beta0_i_dn8: f64 = *var_beta0_i_dn8_slot;
        let mut var_beta0_i_dn9: f64 = *var_beta0_i_dn9_slot;
        let mut var_beta0_i_rv: f64 = *var_beta0_i_rv_slot;
        let mut var_bgidl_i: f64 = *var_bgidl_i_slot;
        let mut var_bgidl_i_rv: f64 = *var_bgidl_i_rv_slot;
        let mut var_bgisl_i: f64 = *var_bgisl_i_slot;
        let mut var_bgisl_i_rv: f64 = *var_bgisl_i_rv_slot;
        let mut var_bigbacc_i: f64 = *var_bigbacc_i_slot;
        let mut var_bigbacc_i_rv: f64 = *var_bigbacc_i_rv_slot;
        let mut var_bigbinv_i: f64 = *var_bigbinv_i_slot;
        let mut var_bigbinv_i_rv: f64 = *var_bigbinv_i_rv_slot;
        let mut var_bigc_i: f64 = *var_bigc_i_slot;
        let mut var_bigc_i_rv: f64 = *var_bigc_i_rv_slot;
        let mut var_bigd_i: f64 = *var_bigd_i_slot;
        let mut var_bigd_i_rv: f64 = *var_bigd_i_rv_slot;
        let mut var_bigs_i: f64 = *var_bigs_i_slot;
        let mut var_bigs_i_rv: f64 = *var_bigs_i_rv_slot;
        let mut var_cf_i: f64 = *var_cf_i_slot;
        let mut var_cf_i_rv: f64 = *var_cf_i_rv_slot;
        let mut var_cgdl_i: f64 = *var_cgdl_i_slot;
        let mut var_cgdl_i_rv: f64 = *var_cgdl_i_rv_slot;
        let mut var_cgidl_i: f64 = *var_cgidl_i_slot;
        let mut var_cgidl_i_rv: f64 = *var_cgidl_i_rv_slot;
        let mut var_cgisl_i: f64 = *var_cgisl_i_slot;
        let mut var_cgisl_i_rv: f64 = *var_cgisl_i_rv_slot;
        let mut var_cgsl_i: f64 = *var_cgsl_i_slot;
        let mut var_cgsl_i_rv: f64 = *var_cgsl_i_rv_slot;
        let mut var_cigbacc_i: f64 = *var_cigbacc_i_slot;
        let mut var_cigbacc_i_rv: f64 = *var_cigbacc_i_rv_slot;
        let mut var_cigbinv_i: f64 = *var_cigbinv_i_slot;
        let mut var_cigbinv_i_rv: f64 = *var_cigbinv_i_rv_slot;
        let mut var_cigc_i: f64 = *var_cigc_i_slot;
        let mut var_cigc_i_rv: f64 = *var_cigc_i_rv_slot;
        let mut var_cigd_i: f64 = *var_cigd_i_slot;
        let mut var_cigd_i_rv: f64 = *var_cigd_i_rv_slot;
        let mut var_cigs_i: f64 = *var_cigs_i_slot;
        let mut var_cigs_i_rv: f64 = *var_cigs_i_rv_slot;
        let mut var_ckappad_i: f64 = *var_ckappad_i_slot;
        let mut var_ckappad_i_rv: f64 = *var_ckappad_i_rv_slot;
        let mut var_ckappas_i: f64 = *var_ckappas_i_slot;
        let mut var_ckappas_i_rv: f64 = *var_ckappas_i_rv_slot;
        let mut var_egidl_i: f64 = *var_egidl_i_slot;
        let mut var_egidl_i_rv: f64 = *var_egidl_i_rv_slot;
        let mut var_egisl_i: f64 = *var_egisl_i_slot;
        let mut var_egisl_i_rv: f64 = *var_egisl_i_rv_slot;
        let mut var_eigbinv_i: f64 = *var_eigbinv_i_slot;
        let mut var_eigbinv_i_rv: f64 = *var_eigbinv_i_rv_slot;
        let mut var_eu1_i: f64 = *var_eu1_i_slot;
        let mut var_eu1_i_rv: f64 = *var_eu1_i_rv_slot;
        let mut var_iit_i: f64 = *var_iit_i_slot;
        let mut var_iit_i_rv: f64 = *var_iit_i_rv_slot;
        let mut var_k2we_i: f64 = *var_k2we_i_slot;
        let mut var_k2we_i_rv: f64 = *var_k2we_i_rv_slot;
        let mut var_kt1_i: f64 = *var_kt1_i_slot;
        let mut var_kt1_i_rv: f64 = *var_kt1_i_rv_slot;
        let mut var_kt2_i: f64 = *var_kt2_i_slot;
        let mut var_kt2_i_rv: f64 = *var_kt2_i_rv_slot;
        let mut var_ku0we_i: f64 = *var_ku0we_i_slot;
        let mut var_ku0we_i_rv: f64 = *var_ku0we_i_rv_slot;
        let mut var_kvth0we_i: f64 = *var_kvth0we_i_slot;
        let mut var_kvth0we_i_rv: f64 = *var_kvth0we_i_rv_slot;
        let mut var_nigbacc_i: f64 = *var_nigbacc_i_slot;
        let mut var_nigbacc_i_rv: f64 = *var_nigbacc_i_rv_slot;
        let mut var_nigbinv_i: f64 = *var_nigbinv_i_slot;
        let mut var_nigbinv_i_rv: f64 = *var_nigbinv_i_rv_slot;
        let mut var_poxedge_i: f64 = *var_poxedge_i_slot;
        let mut var_poxedge_i_rv: f64 = *var_poxedge_i_rv_slot;
        let mut var_prt_i: f64 = *var_prt_i_slot;
        let mut var_prt_i_rv: f64 = *var_prt_i_rv_slot;
        let mut var_psatb_i: f64 = *var_psatb_i_slot;
        let mut var_psatb_i_rv: f64 = *var_psatb_i_rv_slot;
        let mut var_ptwgt_i: f64 = *var_ptwgt_i_slot;
        let mut var_ptwgt_i_rv: f64 = *var_ptwgt_i_rv_slot;
        let mut var_tgidl_i: f64 = *var_tgidl_i_slot;
        let mut var_tgidl_i_rv: f64 = *var_tgidl_i_rv_slot;
        let mut var_ua1_i: f64 = *var_ua1_i_slot;
        let mut var_ua1_i_rv: f64 = *var_ua1_i_rv_slot;
        let mut var_uc1_i: f64 = *var_uc1_i_slot;
        let mut var_uc1_i_rv: f64 = *var_uc1_i_rv_slot;
        let mut var_ucste_i: f64 = *var_ucste_i_slot;
        let mut var_ucste_i_rv: f64 = *var_ucste_i_rv_slot;
        let mut var_ud1_i: f64 = *var_ud1_i_slot;
        let mut var_ud1_i_rv: f64 = *var_ud1_i_rv_slot;
        let mut var_ute_i: f64 = *var_ute_i_slot;
        let mut var_ute_i_rv: f64 = *var_ute_i_rv_slot;

        let assign2210_e3155: f64 = (var_bin_l * p.p667);
        let assign2210_e3156: f64 = (p.p666 + assign2210_e3155);
        let assign2210_e3159: f64 = (var_bin_w * p.p668);
        let assign2210_e3160: f64 = (assign2210_e3156 + assign2210_e3159);
        let assign2210_e3163: f64 = (var_bin_wl * p.p669);
        let assign2210_e3164: f64 = (assign2210_e3160 + assign2210_e3163);
        var_cf_i = assign2210_e3164;
        var_cf_i_rv = 0.0;

        let assign2220_e3168: f64 = (var_bin_l * p.p675);
        let assign2220_e3169: f64 = (p.p674 + assign2220_e3168);
        let assign2220_e3172: f64 = (var_bin_w * p.p676);
        let assign2220_e3173: f64 = (assign2220_e3169 + assign2220_e3172);
        let assign2220_e3176: f64 = (var_bin_wl * p.p677);
        let assign2220_e3177: f64 = (assign2220_e3173 + assign2220_e3176);
        var_cgsl_i = assign2220_e3177;
        var_cgsl_i_rv = 0.0;

        let assign2230_e3181: f64 = (var_bin_l * p.p679);
        let assign2230_e3182: f64 = (p.p678 + assign2230_e3181);
        let assign2230_e3185: f64 = (var_bin_w * p.p680);
        let assign2230_e3186: f64 = (assign2230_e3182 + assign2230_e3185);
        let assign2230_e3189: f64 = (var_bin_wl * p.p681);
        let assign2230_e3190: f64 = (assign2230_e3186 + assign2230_e3189);
        var_cgdl_i = assign2230_e3190;
        var_cgdl_i_rv = 0.0;

        let assign2240_e3194: f64 = (var_bin_l * p.p683);
        let assign2240_e3195: f64 = (p.p682 + assign2240_e3194);
        let assign2240_e3198: f64 = (var_bin_w * p.p684);
        let assign2240_e3199: f64 = (assign2240_e3195 + assign2240_e3198);
        let assign2240_e3202: f64 = (var_bin_wl * p.p685);
        let assign2240_e3203: f64 = (assign2240_e3199 + assign2240_e3202);
        var_ckappas_i = assign2240_e3203;
        var_ckappas_i_rv = 0.0;

        let assign2250_e3207: f64 = (var_bin_l * p.p687);
        let assign2250_e3208: f64 = (p.p686 + assign2250_e3207);
        let assign2250_e3211: f64 = (var_bin_w * p.p688);
        let assign2250_e3212: f64 = (assign2250_e3208 + assign2250_e3211);
        let assign2250_e3215: f64 = (var_bin_wl * p.p689);
        let assign2250_e3216: f64 = (assign2250_e3212 + assign2250_e3215);
        var_ckappad_i = assign2250_e3216;
        var_ckappad_i_rv = 0.0;

        let assign2260_e3220: f64 = (var_bin_l * p.p489);
        let assign2260_e3221: f64 = (p.p484 + assign2260_e3220);
        let assign2260_e3224: f64 = (var_bin_w * p.p490);
        let assign2260_e3225: f64 = (assign2260_e3221 + assign2260_e3224);
        let assign2260_e3228: f64 = (var_bin_wl * p.p491);
        let assign2260_e3229: f64 = (assign2260_e3225 + assign2260_e3228);
        var_alpha0_i = assign2260_e3229;
        var_alpha0_i_dn0 = 0.0;
        var_alpha0_i_dn2 = 0.0;
        var_alpha0_i_dn3 = 0.0;
        var_alpha0_i_dn4 = 0.0;
        var_alpha0_i_dn5 = 0.0;
        var_alpha0_i_dn6 = 0.0;
        var_alpha0_i_dn7 = 0.0;
        var_alpha0_i_dn8 = 0.0;
        var_alpha0_i_dn9 = 0.0;
        var_alpha0_i_dn10 = 0.0;
        var_alpha0_i_dn11 = 0.0;
        var_alpha0_i_dn12 = 0.0;
        var_alpha0_i_dn13 = 0.0;
        var_alpha0_i_dn14 = 0.0;
        var_alpha0_i_rv = 0.0;

        let assign2270_e3233: f64 = (var_bin_l * p.p497);
        let assign2270_e3234: f64 = (p.p494 + assign2270_e3233);
        let assign2270_e3237: f64 = (var_bin_w * p.p498);
        let assign2270_e3238: f64 = (assign2270_e3234 + assign2270_e3237);
        let assign2270_e3241: f64 = (var_bin_wl * p.p499);
        let assign2270_e3242: f64 = (assign2270_e3238 + assign2270_e3241);
        var_beta0_i = assign2270_e3242;
        var_beta0_i_dn0 = 0.0;
        var_beta0_i_dn2 = 0.0;
        var_beta0_i_dn3 = 0.0;
        var_beta0_i_dn4 = 0.0;
        var_beta0_i_dn5 = 0.0;
        var_beta0_i_dn6 = 0.0;
        var_beta0_i_dn7 = 0.0;
        var_beta0_i_dn8 = 0.0;
        var_beta0_i_dn9 = 0.0;
        var_beta0_i_dn10 = 0.0;
        var_beta0_i_dn11 = 0.0;
        var_beta0_i_dn12 = 0.0;
        var_beta0_i_dn13 = 0.0;
        var_beta0_i_dn14 = 0.0;
        var_beta0_i_rv = 0.0;

        let assign2280_e3246: f64 = (var_bin_l * p.p936);
        let assign2280_e3247: f64 = (p.p935 + assign2280_e3246);
        let assign2280_e3250: f64 = (var_bin_w * p.p937);
        let assign2280_e3251: f64 = (assign2280_e3247 + assign2280_e3250);
        let assign2280_e3254: f64 = (var_bin_wl * p.p938);
        let assign2280_e3255: f64 = (assign2280_e3251 + assign2280_e3254);
        var_kvth0we_i = assign2280_e3255;
        var_kvth0we_i_rv = 0.0;

        let assign2290_e3259: f64 = (var_bin_l * p.p940);
        let assign2290_e3260: f64 = (p.p939 + assign2290_e3259);
        let assign2290_e3263: f64 = (var_bin_w * p.p941);
        let assign2290_e3264: f64 = (assign2290_e3260 + assign2290_e3263);
        let assign2290_e3267: f64 = (var_bin_wl * p.p942);
        let assign2290_e3268: f64 = (assign2290_e3264 + assign2290_e3267);
        var_k2we_i = assign2290_e3268;
        var_k2we_i_rv = 0.0;

        let assign2300_e3272: f64 = (var_bin_l * p.p944);
        let assign2300_e3273: f64 = (p.p943 + assign2300_e3272);
        let assign2300_e3276: f64 = (var_bin_w * p.p945);
        let assign2300_e3277: f64 = (assign2300_e3273 + assign2300_e3276);
        let assign2300_e3280: f64 = (var_bin_wl * p.p946);
        let assign2300_e3281: f64 = (assign2300_e3277 + assign2300_e3280);
        var_ku0we_i = assign2300_e3281;
        var_ku0we_i_rv = 0.0;

        let assign2310_e3285: f64 = (var_bin_l * p.p633);
        let assign2310_e3286: f64 = (p.p630 + assign2310_e3285);
        let assign2310_e3289: f64 = (var_bin_w * p.p634);
        let assign2310_e3290: f64 = (assign2310_e3286 + assign2310_e3289);
        let assign2310_e3293: f64 = (var_bin_wl * p.p635);
        let assign2310_e3294: f64 = (assign2310_e3290 + assign2310_e3293);
        var_agidl_i = assign2310_e3294;
        var_agidl_i_rv = 0.0;

        let assign2320_e3298: f64 = (var_bin_l * p.p637);
        let assign2320_e3299: f64 = (p.p636 + assign2320_e3298);
        let assign2320_e3302: f64 = (var_bin_w * p.p638);
        let assign2320_e3303: f64 = (assign2320_e3299 + assign2320_e3302);
        let assign2320_e3306: f64 = (var_bin_wl * p.p639);
        let assign2320_e3307: f64 = (assign2320_e3303 + assign2320_e3306);
        var_bgidl_i = assign2320_e3307;
        var_bgidl_i_rv = 0.0;

        let assign2330_e3311: f64 = (var_bin_l * p.p641);
        let assign2330_e3312: f64 = (p.p640 + assign2330_e3311);
        let assign2330_e3315: f64 = (var_bin_w * p.p642);
        let assign2330_e3316: f64 = (assign2330_e3312 + assign2330_e3315);
        let assign2330_e3319: f64 = (var_bin_wl * p.p643);
        let assign2330_e3320: f64 = (assign2330_e3316 + assign2330_e3319);
        var_cgidl_i = assign2330_e3320;
        var_cgidl_i_rv = 0.0;

        let assign2340_e3324: f64 = (var_bin_l * p.p645);
        let assign2340_e3325: f64 = (p.p644 + assign2340_e3324);
        let assign2340_e3328: f64 = (var_bin_w * p.p646);
        let assign2340_e3329: f64 = (assign2340_e3325 + assign2340_e3328);
        let assign2340_e3332: f64 = (var_bin_wl * p.p647);
        let assign2340_e3333: f64 = (assign2340_e3329 + assign2340_e3332);
        var_egidl_i = assign2340_e3333;
        var_egidl_i_rv = 0.0;

        let assign2350_e3337: f64 = (var_bin_l * p.p651);
        let assign2350_e3338: f64 = (p.p648 + assign2350_e3337);
        let assign2350_e3341: f64 = (var_bin_w * p.p652);
        let assign2350_e3342: f64 = (assign2350_e3338 + assign2350_e3341);
        let assign2350_e3345: f64 = (var_bin_wl * p.p653);
        let assign2350_e3346: f64 = (assign2350_e3342 + assign2350_e3345);
        var_agisl_i = assign2350_e3346;
        var_agisl_i_rv = 0.0;

        let assign2360_e3350: f64 = (var_bin_l * p.p655);
        let assign2360_e3351: f64 = (p.p654 + assign2360_e3350);
        let assign2360_e3354: f64 = (var_bin_w * p.p656);
        let assign2360_e3355: f64 = (assign2360_e3351 + assign2360_e3354);
        let assign2360_e3358: f64 = (var_bin_wl * p.p657);
        let assign2360_e3359: f64 = (assign2360_e3355 + assign2360_e3358);
        var_bgisl_i = assign2360_e3359;
        var_bgisl_i_rv = 0.0;

        let assign2370_e3363: f64 = (var_bin_l * p.p659);
        let assign2370_e3364: f64 = (p.p658 + assign2370_e3363);
        let assign2370_e3367: f64 = (var_bin_w * p.p660);
        let assign2370_e3368: f64 = (assign2370_e3364 + assign2370_e3367);
        let assign2370_e3371: f64 = (var_bin_wl * p.p661);
        let assign2370_e3372: f64 = (assign2370_e3368 + assign2370_e3371);
        var_cgisl_i = assign2370_e3372;
        var_cgisl_i_rv = 0.0;

        let assign2380_e3376: f64 = (var_bin_l * p.p663);
        let assign2380_e3377: f64 = (p.p662 + assign2380_e3376);
        let assign2380_e3380: f64 = (var_bin_w * p.p664);
        let assign2380_e3381: f64 = (assign2380_e3377 + assign2380_e3380);
        let assign2380_e3384: f64 = (var_bin_wl * p.p665);
        let assign2380_e3385: f64 = (assign2380_e3381 + assign2380_e3384);
        var_egisl_i = assign2380_e3385;
        var_egisl_i_rv = 0.0;

        let assign2390_e3389: f64 = (var_bin_l * p.p825);
        let assign2390_e3390: f64 = (p.p824 + assign2390_e3389);
        let assign2390_e3393: f64 = (var_bin_w * p.p826);
        let assign2390_e3394: f64 = (assign2390_e3390 + assign2390_e3393);
        let assign2390_e3397: f64 = (var_bin_wl * p.p827);
        let assign2390_e3398: f64 = (assign2390_e3394 + assign2390_e3397);
        var_ute_i = assign2390_e3398;
        var_ute_i_rv = 0.0;

        let assign2400_e3402: f64 = (var_bin_l * p.p830);
        let assign2400_e3403: f64 = (p.p829 + assign2400_e3402);
        let assign2400_e3406: f64 = (var_bin_w * p.p831);
        let assign2400_e3407: f64 = (assign2400_e3403 + assign2400_e3406);
        let assign2400_e3410: f64 = (var_bin_wl * p.p832);
        let assign2400_e3411: f64 = (assign2400_e3407 + assign2400_e3410);
        var_ua1_i = assign2400_e3411;
        var_ua1_i_rv = 0.0;

        let assign2410_e3415: f64 = (var_bin_l * p.p835);
        let assign2410_e3416: f64 = (p.p834 + assign2410_e3415);
        let assign2410_e3419: f64 = (var_bin_w * p.p836);
        let assign2410_e3420: f64 = (assign2410_e3416 + assign2410_e3419);
        let assign2410_e3423: f64 = (var_bin_wl * p.p837);
        let assign2410_e3424: f64 = (assign2410_e3420 + assign2410_e3423);
        var_uc1_i = assign2410_e3424;
        var_uc1_i_rv = 0.0;

        let assign2420_e3428: f64 = (var_bin_l * p.p839);
        let assign2420_e3429: f64 = (p.p838 + assign2420_e3428);
        let assign2420_e3432: f64 = (var_bin_w * p.p840);
        let assign2420_e3433: f64 = (assign2420_e3429 + assign2420_e3432);
        let assign2420_e3436: f64 = (var_bin_wl * p.p841);
        let assign2420_e3437: f64 = (assign2420_e3433 + assign2420_e3436);
        var_ud1_i = assign2420_e3437;
        var_ud1_i_rv = 0.0;

        let assign2430_e3441: f64 = (var_bin_l * p.p844);
        let assign2430_e3442: f64 = (p.p843 + assign2430_e3441);
        let assign2430_e3445: f64 = (var_bin_w * p.p845);
        let assign2430_e3446: f64 = (assign2430_e3442 + assign2430_e3445);
        let assign2430_e3449: f64 = (var_bin_wl * p.p846);
        let assign2430_e3450: f64 = (assign2430_e3446 + assign2430_e3449);
        var_eu1_i = assign2430_e3450;
        var_eu1_i_rv = 0.0;

        let assign2440_e3454: f64 = (var_bin_l * p.p848);
        let assign2440_e3455: f64 = (p.p847 + assign2440_e3454);
        let assign2440_e3458: f64 = (var_bin_w * p.p849);
        let assign2440_e3459: f64 = (assign2440_e3455 + assign2440_e3458);
        let assign2440_e3462: f64 = (var_bin_wl * p.p850);
        let assign2440_e3463: f64 = (assign2440_e3459 + assign2440_e3462);
        var_ucste_i = assign2440_e3463;
        var_ucste_i_rv = 0.0;

        let assign2450_e3467: f64 = (var_bin_l * p.p853);
        let assign2450_e3468: f64 = (p.p852 + assign2450_e3467);
        let assign2450_e3471: f64 = (var_bin_w * p.p854);
        let assign2450_e3472: f64 = (assign2450_e3468 + assign2450_e3471);
        let assign2450_e3475: f64 = (var_bin_wl * p.p855);
        let assign2450_e3476: f64 = (assign2450_e3472 + assign2450_e3475);
        var_prt_i = assign2450_e3476;
        var_prt_i_rv = 0.0;

        let assign2460_e3480: f64 = (var_bin_l * p.p857);
        let assign2460_e3481: f64 = (p.p856 + assign2460_e3480);
        let assign2460_e3484: f64 = (var_bin_w * p.p858);
        let assign2460_e3485: f64 = (assign2460_e3481 + assign2460_e3484);
        let assign2460_e3488: f64 = (var_bin_wl * p.p859);
        let assign2460_e3489: f64 = (assign2460_e3485 + assign2460_e3488);
        var_at_i = assign2460_e3489;
        var_at_i_rv = 0.0;

        let assign2470_e3493: f64 = (var_bin_l * p.p863);
        let assign2470_e3494: f64 = (p.p862 + assign2470_e3493);
        let assign2470_e3497: f64 = (var_bin_w * p.p864);
        let assign2470_e3498: f64 = (assign2470_e3494 + assign2470_e3497);
        let assign2470_e3501: f64 = (var_bin_wl * p.p865);
        let assign2470_e3502: f64 = (assign2470_e3498 + assign2470_e3501);
        var_ptwgt_i = assign2470_e3502;
        var_ptwgt_i_rv = 0.0;

        let assign2480_e3506: f64 = (var_bin_l * p.p878);
        let assign2480_e3507: f64 = (p.p877 + assign2480_e3506);
        let assign2480_e3510: f64 = (var_bin_w * p.p879);
        let assign2480_e3511: f64 = (assign2480_e3507 + assign2480_e3510);
        let assign2480_e3514: f64 = (var_bin_wl * p.p880);
        let assign2480_e3515: f64 = (assign2480_e3511 + assign2480_e3514);
        var_iit_i = assign2480_e3515;
        var_iit_i_rv = 0.0;

        let assign2490_e3519: f64 = (var_bin_l * p.p886);
        let assign2490_e3520: f64 = (p.p885 + assign2490_e3519);
        let assign2490_e3523: f64 = (var_bin_w * p.p887);
        let assign2490_e3524: f64 = (assign2490_e3520 + assign2490_e3523);
        let assign2490_e3527: f64 = (var_bin_wl * p.p888);
        let assign2490_e3528: f64 = (assign2490_e3524 + assign2490_e3527);
        var_tgidl_i = assign2490_e3528;
        var_tgidl_i_rv = 0.0;

        let assign2510_e3545: f64 = (var_bin_l * p.p564);
        let assign2510_e3546: f64 = (p.p537 + assign2510_e3545);
        let assign2510_e3549: f64 = (var_bin_w * p.p565);
        let assign2510_e3550: f64 = (assign2510_e3546 + assign2510_e3549);
        let assign2510_e3553: f64 = (var_bin_wl * p.p566);
        let assign2510_e3554: f64 = (assign2510_e3550 + assign2510_e3553);
        var_aigbinv_i = assign2510_e3554;
        var_aigbinv_i_rv = 0.0;

        let assign2520_e3558: f64 = (var_bin_l * p.p567);
        let assign2520_e3559: f64 = (p.p538 + assign2520_e3558);
        let assign2520_e3562: f64 = (var_bin_w * p.p568);
        let assign2520_e3563: f64 = (assign2520_e3559 + assign2520_e3562);
        let assign2520_e3566: f64 = (var_bin_wl * p.p569);
        let assign2520_e3567: f64 = (assign2520_e3563 + assign2520_e3566);
        var_bigbinv_i = assign2520_e3567;
        var_bigbinv_i_rv = 0.0;

        let assign2530_e3571: f64 = (var_bin_l * p.p570);
        let assign2530_e3572: f64 = (p.p539 + assign2530_e3571);
        let assign2530_e3575: f64 = (var_bin_w * p.p571);
        let assign2530_e3576: f64 = (assign2530_e3572 + assign2530_e3575);
        let assign2530_e3579: f64 = (var_bin_wl * p.p572);
        let assign2530_e3580: f64 = (assign2530_e3576 + assign2530_e3579);
        var_cigbinv_i = assign2530_e3580;
        var_cigbinv_i_rv = 0.0;

        let assign2540_e3584: f64 = (var_bin_l * p.p573);
        let assign2540_e3585: f64 = (p.p540 + assign2540_e3584);
        let assign2540_e3588: f64 = (var_bin_w * p.p574);
        let assign2540_e3589: f64 = (assign2540_e3585 + assign2540_e3588);
        let assign2540_e3592: f64 = (var_bin_wl * p.p575);
        let assign2540_e3593: f64 = (assign2540_e3589 + assign2540_e3592);
        var_eigbinv_i = assign2540_e3593;
        var_eigbinv_i_rv = 0.0;

        let assign2550_e3597: f64 = (var_bin_l * p.p576);
        let assign2550_e3598: f64 = (p.p541 + assign2550_e3597);
        let assign2550_e3601: f64 = (var_bin_w * p.p577);
        let assign2550_e3602: f64 = (assign2550_e3598 + assign2550_e3601);
        let assign2550_e3605: f64 = (var_bin_wl * p.p578);
        let assign2550_e3606: f64 = (assign2550_e3602 + assign2550_e3605);
        var_nigbinv_i = assign2550_e3606;
        var_nigbinv_i_rv = 0.0;

        let assign2560_e3610: f64 = (var_bin_l * p.p579);
        let assign2560_e3611: f64 = (p.p533 + assign2560_e3610);
        let assign2560_e3614: f64 = (var_bin_w * p.p580);
        let assign2560_e3615: f64 = (assign2560_e3611 + assign2560_e3614);
        let assign2560_e3618: f64 = (var_bin_wl * p.p581);
        let assign2560_e3619: f64 = (assign2560_e3615 + assign2560_e3618);
        var_aigbacc_i = assign2560_e3619;
        var_aigbacc_i_rv = 0.0;

        let assign2570_e3623: f64 = (var_bin_l * p.p582);
        let assign2570_e3624: f64 = (p.p534 + assign2570_e3623);
        let assign2570_e3627: f64 = (var_bin_w * p.p583);
        let assign2570_e3628: f64 = (assign2570_e3624 + assign2570_e3627);
        let assign2570_e3631: f64 = (var_bin_wl * p.p584);
        let assign2570_e3632: f64 = (assign2570_e3628 + assign2570_e3631);
        var_bigbacc_i = assign2570_e3632;
        var_bigbacc_i_rv = 0.0;

        let assign2580_e3636: f64 = (var_bin_l * p.p585);
        let assign2580_e3637: f64 = (p.p535 + assign2580_e3636);
        let assign2580_e3640: f64 = (var_bin_w * p.p586);
        let assign2580_e3641: f64 = (assign2580_e3637 + assign2580_e3640);
        let assign2580_e3644: f64 = (var_bin_wl * p.p587);
        let assign2580_e3645: f64 = (assign2580_e3641 + assign2580_e3644);
        var_cigbacc_i = assign2580_e3645;
        var_cigbacc_i_rv = 0.0;

        let assign2590_e3649: f64 = (var_bin_l * p.p588);
        let assign2590_e3650: f64 = (p.p536 + assign2590_e3649);
        let assign2590_e3653: f64 = (var_bin_w * p.p589);
        let assign2590_e3654: f64 = (assign2590_e3650 + assign2590_e3653);
        let assign2590_e3657: f64 = (var_bin_wl * p.p590);
        let assign2590_e3658: f64 = (assign2590_e3654 + assign2590_e3657);
        var_nigbacc_i = assign2590_e3658;
        var_nigbacc_i_rv = 0.0;

        let assign2600_e3662: f64 = (var_bin_l * p.p591);
        let assign2600_e3663: f64 = (p.p542 + assign2600_e3662);
        let assign2600_e3666: f64 = (var_bin_w * p.p592);
        let assign2600_e3667: f64 = (assign2600_e3663 + assign2600_e3666);
        let assign2600_e3670: f64 = (var_bin_wl * p.p593);
        let assign2600_e3671: f64 = (assign2600_e3667 + assign2600_e3670);
        var_aigc_i = assign2600_e3671;
        var_aigc_i_rv = 0.0;

        let assign2610_e3675: f64 = (var_bin_l * p.p594);
        let assign2610_e3676: f64 = (p.p543 + assign2610_e3675);
        let assign2610_e3679: f64 = (var_bin_w * p.p595);
        let assign2610_e3680: f64 = (assign2610_e3676 + assign2610_e3679);
        let assign2610_e3683: f64 = (var_bin_wl * p.p596);
        let assign2610_e3684: f64 = (assign2610_e3680 + assign2610_e3683);
        var_bigc_i = assign2610_e3684;
        var_bigc_i_rv = 0.0;

        let assign2620_e3688: f64 = (var_bin_l * p.p597);
        let assign2620_e3689: f64 = (p.p544 + assign2620_e3688);
        let assign2620_e3692: f64 = (var_bin_w * p.p598);
        let assign2620_e3693: f64 = (assign2620_e3689 + assign2620_e3692);
        let assign2620_e3696: f64 = (var_bin_wl * p.p599);
        let assign2620_e3697: f64 = (assign2620_e3693 + assign2620_e3696);
        var_cigc_i = assign2620_e3697;
        var_cigc_i_rv = 0.0;

        let assign2630_e3701: f64 = (var_bin_l * p.p600);
        let assign2630_e3702: f64 = (p.p545 + assign2630_e3701);
        let assign2630_e3705: f64 = (var_bin_w * p.p601);
        let assign2630_e3706: f64 = (assign2630_e3702 + assign2630_e3705);
        let assign2630_e3709: f64 = (var_bin_wl * p.p602);
        let assign2630_e3710: f64 = (assign2630_e3706 + assign2630_e3709);
        var_aigs_i = assign2630_e3710;
        var_aigs_i_rv = 0.0;

        let assign2640_e3714: f64 = (var_bin_l * p.p603);
        let assign2640_e3715: f64 = (p.p546 + assign2640_e3714);
        let assign2640_e3718: f64 = (var_bin_w * p.p604);
        let assign2640_e3719: f64 = (assign2640_e3715 + assign2640_e3718);
        let assign2640_e3722: f64 = (var_bin_wl * p.p605);
        let assign2640_e3723: f64 = (assign2640_e3719 + assign2640_e3722);
        var_bigs_i = assign2640_e3723;
        var_bigs_i_rv = 0.0;

        let assign2650_e3727: f64 = (var_bin_l * p.p606);
        let assign2650_e3728: f64 = (p.p547 + assign2650_e3727);
        let assign2650_e3731: f64 = (var_bin_w * p.p607);
        let assign2650_e3732: f64 = (assign2650_e3728 + assign2650_e3731);
        let assign2650_e3735: f64 = (var_bin_wl * p.p608);
        let assign2650_e3736: f64 = (assign2650_e3732 + assign2650_e3735);
        var_cigs_i = assign2650_e3736;
        var_cigs_i_rv = 0.0;

        let assign2660_e3740: f64 = (var_bin_l * p.p609);
        let assign2660_e3741: f64 = (p.p548 + assign2660_e3740);
        let assign2660_e3744: f64 = (var_bin_w * p.p610);
        let assign2660_e3745: f64 = (assign2660_e3741 + assign2660_e3744);
        let assign2660_e3748: f64 = (var_bin_wl * p.p611);
        let assign2660_e3749: f64 = (assign2660_e3745 + assign2660_e3748);
        var_aigd_i = assign2660_e3749;
        var_aigd_i_rv = 0.0;

        let assign2670_e3753: f64 = (var_bin_l * p.p612);
        let assign2670_e3754: f64 = (p.p549 + assign2670_e3753);
        let assign2670_e3757: f64 = (var_bin_w * p.p613);
        let assign2670_e3758: f64 = (assign2670_e3754 + assign2670_e3757);
        let assign2670_e3761: f64 = (var_bin_wl * p.p614);
        let assign2670_e3762: f64 = (assign2670_e3758 + assign2670_e3761);
        var_bigd_i = assign2670_e3762;
        var_bigd_i_rv = 0.0;

        let assign2680_e3766: f64 = (var_bin_l * p.p615);
        let assign2680_e3767: f64 = (p.p550 + assign2680_e3766);
        let assign2680_e3770: f64 = (var_bin_w * p.p616);
        let assign2680_e3771: f64 = (assign2680_e3767 + assign2680_e3770);
        let assign2680_e3774: f64 = (var_bin_wl * p.p617);
        let assign2680_e3775: f64 = (assign2680_e3771 + assign2680_e3774);
        var_cigd_i = assign2680_e3775;
        var_cigd_i_rv = 0.0;

        let assign2690_e3779: f64 = (var_bin_l * p.p618);
        let assign2690_e3780: f64 = (p.p553 + assign2690_e3779);
        let assign2690_e3783: f64 = (var_bin_w * p.p619);
        let assign2690_e3784: f64 = (assign2690_e3780 + assign2690_e3783);
        let assign2690_e3787: f64 = (var_bin_wl * p.p620);
        let assign2690_e3788: f64 = (assign2690_e3784 + assign2690_e3787);
        var_poxedge_i = assign2690_e3788;
        var_poxedge_i_rv = 0.0;

        let assign2730_e3831: f64 = (var_bin_l * p.p870);
        let assign2730_e3832: f64 = (p.p867 + assign2730_e3831);
        let assign2730_e3835: f64 = (var_bin_w * p.p871);
        let assign2730_e3836: f64 = (assign2730_e3832 + assign2730_e3835);
        let assign2730_e3839: f64 = (var_bin_wl * p.p872);
        let assign2730_e3840: f64 = (assign2730_e3836 + assign2730_e3839);
        var_kt1_i = assign2730_e3840;
        var_kt1_i_rv = 0.0;

        let assign2740_e3844: f64 = (var_bin_l * p.p874);
        let assign2740_e3845: f64 = (p.p873 + assign2740_e3844);
        let assign2740_e3848: f64 = (var_bin_w * p.p875);
        let assign2740_e3849: f64 = (assign2740_e3845 + assign2740_e3848);
        let assign2740_e3852: f64 = (var_bin_wl * p.p876);
        let assign2740_e3853: f64 = (assign2740_e3849 + assign2740_e3852);
        var_kt2_i = assign2740_e3853;
        var_kt2_i_rv = 0.0;

        let assign2750_e3857: f64 = (var_bin_l * p.p430);
        let assign2750_e3858: f64 = (p.p425 + assign2750_e3857);
        let assign2750_e3861: f64 = (var_bin_w * p.p431);
        let assign2750_e3862: f64 = (assign2750_e3858 + assign2750_e3861);
        let assign2750_e3865: f64 = (var_bin_wl * p.p432);
        let assign2750_e3866: f64 = (assign2750_e3862 + assign2750_e3865);
        var_psatb_i = assign2750_e3866;
        var_psatb_i_rv = 0.0;

        let assign2760_e3870: f64 = (var_bin_l * p.p445);
        let assign2760_e3871: f64 = (p.p444 + assign2760_e3870);
        let assign2760_e3874: f64 = (var_bin_w * p.p446);
        let assign2760_e3875: f64 = (assign2760_e3871 + assign2760_e3874);
        let assign2760_e3878: f64 = (var_bin_wl * p.p447);
        let assign2760_e3879: f64 = (assign2760_e3875 + assign2760_e3878);
        var_a1_i = assign2760_e3879;
        var_a1_i_rv = 0.0;

        let assign2770_e3883: f64 = (var_bin_l * p.p449);
        let assign2770_e3884: f64 = (p.p448 + assign2770_e3883);
        let assign2770_e3887: f64 = (var_bin_w * p.p450);
        let assign2770_e3888: f64 = (assign2770_e3884 + assign2770_e3887);
        let assign2770_e3891: f64 = (var_bin_wl * p.p451);
        let assign2770_e3892: f64 = (assign2770_e3888 + assign2770_e3891);
        var_a11_i = assign2770_e3892;
        var_a11_i_rv = 0.0;

        *var_a11_i_slot = var_a11_i;
        *var_a11_i_rv_slot = var_a11_i_rv;
        *var_a1_i_slot = var_a1_i;
        *var_a1_i_rv_slot = var_a1_i_rv;
        *var_agidl_i_slot = var_agidl_i;
        *var_agidl_i_rv_slot = var_agidl_i_rv;
        *var_agisl_i_slot = var_agisl_i;
        *var_agisl_i_rv_slot = var_agisl_i_rv;
        *var_aigbacc_i_slot = var_aigbacc_i;
        *var_aigbacc_i_rv_slot = var_aigbacc_i_rv;
        *var_aigbinv_i_slot = var_aigbinv_i;
        *var_aigbinv_i_rv_slot = var_aigbinv_i_rv;
        *var_aigc_i_slot = var_aigc_i;
        *var_aigc_i_rv_slot = var_aigc_i_rv;
        *var_aigd_i_slot = var_aigd_i;
        *var_aigd_i_rv_slot = var_aigd_i_rv;
        *var_aigs_i_slot = var_aigs_i;
        *var_aigs_i_rv_slot = var_aigs_i_rv;
        *var_alpha0_i_slot = var_alpha0_i;
        *var_alpha0_i_dn0_slot = var_alpha0_i_dn0;
        *var_alpha0_i_dn10_slot = var_alpha0_i_dn10;
        *var_alpha0_i_dn11_slot = var_alpha0_i_dn11;
        *var_alpha0_i_dn12_slot = var_alpha0_i_dn12;
        *var_alpha0_i_dn13_slot = var_alpha0_i_dn13;
        *var_alpha0_i_dn14_slot = var_alpha0_i_dn14;
        *var_alpha0_i_dn2_slot = var_alpha0_i_dn2;
        *var_alpha0_i_dn3_slot = var_alpha0_i_dn3;
        *var_alpha0_i_dn4_slot = var_alpha0_i_dn4;
        *var_alpha0_i_dn5_slot = var_alpha0_i_dn5;
        *var_alpha0_i_dn6_slot = var_alpha0_i_dn6;
        *var_alpha0_i_dn7_slot = var_alpha0_i_dn7;
        *var_alpha0_i_dn8_slot = var_alpha0_i_dn8;
        *var_alpha0_i_dn9_slot = var_alpha0_i_dn9;
        *var_alpha0_i_rv_slot = var_alpha0_i_rv;
        *var_at_i_slot = var_at_i;
        *var_at_i_rv_slot = var_at_i_rv;
        *var_beta0_i_slot = var_beta0_i;
        *var_beta0_i_dn0_slot = var_beta0_i_dn0;
        *var_beta0_i_dn10_slot = var_beta0_i_dn10;
        *var_beta0_i_dn11_slot = var_beta0_i_dn11;
        *var_beta0_i_dn12_slot = var_beta0_i_dn12;
        *var_beta0_i_dn13_slot = var_beta0_i_dn13;
        *var_beta0_i_dn14_slot = var_beta0_i_dn14;
        *var_beta0_i_dn2_slot = var_beta0_i_dn2;
        *var_beta0_i_dn3_slot = var_beta0_i_dn3;
        *var_beta0_i_dn4_slot = var_beta0_i_dn4;
        *var_beta0_i_dn5_slot = var_beta0_i_dn5;
        *var_beta0_i_dn6_slot = var_beta0_i_dn6;
        *var_beta0_i_dn7_slot = var_beta0_i_dn7;
        *var_beta0_i_dn8_slot = var_beta0_i_dn8;
        *var_beta0_i_dn9_slot = var_beta0_i_dn9;
        *var_beta0_i_rv_slot = var_beta0_i_rv;
        *var_bgidl_i_slot = var_bgidl_i;
        *var_bgidl_i_rv_slot = var_bgidl_i_rv;
        *var_bgisl_i_slot = var_bgisl_i;
        *var_bgisl_i_rv_slot = var_bgisl_i_rv;
        *var_bigbacc_i_slot = var_bigbacc_i;
        *var_bigbacc_i_rv_slot = var_bigbacc_i_rv;
        *var_bigbinv_i_slot = var_bigbinv_i;
        *var_bigbinv_i_rv_slot = var_bigbinv_i_rv;
        *var_bigc_i_slot = var_bigc_i;
        *var_bigc_i_rv_slot = var_bigc_i_rv;
        *var_bigd_i_slot = var_bigd_i;
        *var_bigd_i_rv_slot = var_bigd_i_rv;
        *var_bigs_i_slot = var_bigs_i;
        *var_bigs_i_rv_slot = var_bigs_i_rv;
        *var_cf_i_slot = var_cf_i;
        *var_cf_i_rv_slot = var_cf_i_rv;
        *var_cgdl_i_slot = var_cgdl_i;
        *var_cgdl_i_rv_slot = var_cgdl_i_rv;
        *var_cgidl_i_slot = var_cgidl_i;
        *var_cgidl_i_rv_slot = var_cgidl_i_rv;
        *var_cgisl_i_slot = var_cgisl_i;
        *var_cgisl_i_rv_slot = var_cgisl_i_rv;
        *var_cgsl_i_slot = var_cgsl_i;
        *var_cgsl_i_rv_slot = var_cgsl_i_rv;
        *var_cigbacc_i_slot = var_cigbacc_i;
        *var_cigbacc_i_rv_slot = var_cigbacc_i_rv;
        *var_cigbinv_i_slot = var_cigbinv_i;
        *var_cigbinv_i_rv_slot = var_cigbinv_i_rv;
        *var_cigc_i_slot = var_cigc_i;
        *var_cigc_i_rv_slot = var_cigc_i_rv;
        *var_cigd_i_slot = var_cigd_i;
        *var_cigd_i_rv_slot = var_cigd_i_rv;
        *var_cigs_i_slot = var_cigs_i;
        *var_cigs_i_rv_slot = var_cigs_i_rv;
        *var_ckappad_i_slot = var_ckappad_i;
        *var_ckappad_i_rv_slot = var_ckappad_i_rv;
        *var_ckappas_i_slot = var_ckappas_i;
        *var_ckappas_i_rv_slot = var_ckappas_i_rv;
        *var_egidl_i_slot = var_egidl_i;
        *var_egidl_i_rv_slot = var_egidl_i_rv;
        *var_egisl_i_slot = var_egisl_i;
        *var_egisl_i_rv_slot = var_egisl_i_rv;
        *var_eigbinv_i_slot = var_eigbinv_i;
        *var_eigbinv_i_rv_slot = var_eigbinv_i_rv;
        *var_eu1_i_slot = var_eu1_i;
        *var_eu1_i_rv_slot = var_eu1_i_rv;
        *var_iit_i_slot = var_iit_i;
        *var_iit_i_rv_slot = var_iit_i_rv;
        *var_k2we_i_slot = var_k2we_i;
        *var_k2we_i_rv_slot = var_k2we_i_rv;
        *var_kt1_i_slot = var_kt1_i;
        *var_kt1_i_rv_slot = var_kt1_i_rv;
        *var_kt2_i_slot = var_kt2_i;
        *var_kt2_i_rv_slot = var_kt2_i_rv;
        *var_ku0we_i_slot = var_ku0we_i;
        *var_ku0we_i_rv_slot = var_ku0we_i_rv;
        *var_kvth0we_i_slot = var_kvth0we_i;
        *var_kvth0we_i_rv_slot = var_kvth0we_i_rv;
        *var_nigbacc_i_slot = var_nigbacc_i;
        *var_nigbacc_i_rv_slot = var_nigbacc_i_rv;
        *var_nigbinv_i_slot = var_nigbinv_i;
        *var_nigbinv_i_rv_slot = var_nigbinv_i_rv;
        *var_poxedge_i_slot = var_poxedge_i;
        *var_poxedge_i_rv_slot = var_poxedge_i_rv;
        *var_prt_i_slot = var_prt_i;
        *var_prt_i_rv_slot = var_prt_i_rv;
        *var_psatb_i_slot = var_psatb_i;
        *var_psatb_i_rv_slot = var_psatb_i_rv;
        *var_ptwgt_i_slot = var_ptwgt_i;
        *var_ptwgt_i_rv_slot = var_ptwgt_i_rv;
        *var_tgidl_i_slot = var_tgidl_i;
        *var_tgidl_i_rv_slot = var_tgidl_i_rv;
        *var_ua1_i_slot = var_ua1_i;
        *var_ua1_i_rv_slot = var_ua1_i_rv;
        *var_uc1_i_slot = var_uc1_i;
        *var_uc1_i_rv_slot = var_uc1_i_rv;
        *var_ucste_i_slot = var_ucste_i;
        *var_ucste_i_rv_slot = var_ucste_i_rv;
        *var_ud1_i_slot = var_ud1_i;
        *var_ud1_i_rv_slot = var_ud1_i_rv;
        *var_ute_i_slot = var_ute_i;
        *var_ute_i_rv_slot = var_ute_i_rv;
    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        var_bin_l: f64,
        var_bin_w: f64,
        var_bin_wl: f64,
        var_a21_i_slot: &mut f64,
        var_a21_i_rv_slot: &mut f64,
        var_a2_i_slot: &mut f64,
        var_a2_i_rv_slot: &mut f64,
        var_c01_i_slot: &mut f64,
        var_c01_i_rv_slot: &mut f64,
        var_c0_i_slot: &mut f64,
        var_c0_i_rv_slot: &mut f64,
        var_c0si1_i_slot: &mut f64,
        var_c0si1_i_rv_slot: &mut f64,
        var_c0si_i_slot: &mut f64,
        var_c0si_i_rv_slot: &mut f64,
        var_c0sisat1_i_slot: &mut f64,
        var_c0sisat1_i_rv_slot: &mut f64,
        var_c0sisat_i_slot: &mut f64,
        var_c0sisat_i_rv_slot: &mut f64,
        var_cdscbedge_i_slot: &mut f64,
        var_cdscbedge_i_rv_slot: &mut f64,
        var_cdscdedge_i_slot: &mut f64,
        var_cdscdedge_i_rv_slot: &mut f64,
        var_cdscdr_i_slot: &mut f64,
        var_cdscdr_i_dn0_slot: &mut f64,
        var_cdscdr_i_dn10_slot: &mut f64,
        var_cdscdr_i_dn11_slot: &mut f64,
        var_cdscdr_i_dn12_slot: &mut f64,
        var_cdscdr_i_dn13_slot: &mut f64,
        var_cdscdr_i_dn14_slot: &mut f64,
        var_cdscdr_i_dn2_slot: &mut f64,
        var_cdscdr_i_dn3_slot: &mut f64,
        var_cdscdr_i_dn4_slot: &mut f64,
        var_cdscdr_i_dn5_slot: &mut f64,
        var_cdscdr_i_dn6_slot: &mut f64,
        var_cdscdr_i_dn7_slot: &mut f64,
        var_cdscdr_i_dn8_slot: &mut f64,
        var_cdscdr_i_dn9_slot: &mut f64,
        var_cdscdr_i_rv_slot: &mut f64,
        var_citedge_i_slot: &mut f64,
        var_citedge_i_rv_slot: &mut f64,
        var_eta0edge_i_slot: &mut f64,
        var_eta0edge_i_dn0_slot: &mut f64,
        var_eta0edge_i_dn10_slot: &mut f64,
        var_eta0edge_i_dn11_slot: &mut f64,
        var_eta0edge_i_dn12_slot: &mut f64,
        var_eta0edge_i_dn13_slot: &mut f64,
        var_eta0edge_i_dn14_slot: &mut f64,
        var_eta0edge_i_dn2_slot: &mut f64,
        var_eta0edge_i_dn3_slot: &mut f64,
        var_eta0edge_i_dn4_slot: &mut f64,
        var_eta0edge_i_dn5_slot: &mut f64,
        var_eta0edge_i_dn6_slot: &mut f64,
        var_eta0edge_i_dn7_slot: &mut f64,
        var_eta0edge_i_dn8_slot: &mut f64,
        var_eta0edge_i_dn9_slot: &mut f64,
        var_eta0edge_i_rv_slot: &mut f64,
        var_eta0r_i_slot: &mut f64,
        var_eta0r_i_dn0_slot: &mut f64,
        var_eta0r_i_dn10_slot: &mut f64,
        var_eta0r_i_dn11_slot: &mut f64,
        var_eta0r_i_dn12_slot: &mut f64,
        var_eta0r_i_dn13_slot: &mut f64,
        var_eta0r_i_dn14_slot: &mut f64,
        var_eta0r_i_dn2_slot: &mut f64,
        var_eta0r_i_dn3_slot: &mut f64,
        var_eta0r_i_dn4_slot: &mut f64,
        var_eta0r_i_dn5_slot: &mut f64,
        var_eta0r_i_dn6_slot: &mut f64,
        var_eta0r_i_dn7_slot: &mut f64,
        var_eta0r_i_dn8_slot: &mut f64,
        var_eta0r_i_dn9_slot: &mut f64,
        var_eta0r_i_rv_slot: &mut f64,
        var_etabedge_i_slot: &mut f64,
        var_etabedge_i_rv_slot: &mut f64,
        var_guard21_slot: &mut f64,
        var_guard21_rv_slot: &mut f64,
        var_k01_i_slot: &mut f64,
        var_k01_i_rv_slot: &mut f64,
        var_k0_i_slot: &mut f64,
        var_k0_i_rv_slot: &mut f64,
        var_k2edge_i_slot: &mut f64,
        var_k2edge_i_dn0_slot: &mut f64,
        var_k2edge_i_dn10_slot: &mut f64,
        var_k2edge_i_dn11_slot: &mut f64,
        var_k2edge_i_dn12_slot: &mut f64,
        var_k2edge_i_dn13_slot: &mut f64,
        var_k2edge_i_dn14_slot: &mut f64,
        var_k2edge_i_dn2_slot: &mut f64,
        var_k2edge_i_dn3_slot: &mut f64,
        var_k2edge_i_dn4_slot: &mut f64,
        var_k2edge_i_dn5_slot: &mut f64,
        var_k2edge_i_dn6_slot: &mut f64,
        var_k2edge_i_dn7_slot: &mut f64,
        var_k2edge_i_dn8_slot: &mut f64,
        var_k2edge_i_dn9_slot: &mut f64,
        var_k2edge_i_rv_slot: &mut f64,
        var_k2edgewe_i_slot: &mut f64,
        var_k2edgewe_i_rv_slot: &mut f64,
        var_kt1edge_i_slot: &mut f64,
        var_kt1edge_i_rv_slot: &mut f64,
        var_kt1expedge_i_slot: &mut f64,
        var_kt1expedge_i_rv_slot: &mut f64,
        var_kt1ledge_i_slot: &mut f64,
        var_kt1ledge_i_rv_slot: &mut f64,
        var_kt2edge_i_slot: &mut f64,
        var_kt2edge_i_rv_slot: &mut f64,
        var_kvth0edge_i_slot: &mut f64,
        var_kvth0edge_i_rv_slot: &mut f64,
        var_kvth0edgewe_i_slot: &mut f64,
        var_kvth0edgewe_i_rv_slot: &mut f64,
        var_m01_i_slot: &mut f64,
        var_m01_i_rv_slot: &mut f64,
        var_m0_i_slot: &mut f64,
        var_m0_i_rv_slot: &mut f64,
        var_mpower_i_slot: &mut f64,
        var_mpower_i_rv_slot: &mut f64,
        var_ndepedge_i_slot: &mut f64,
        var_ndepedge_i_rv_slot: &mut f64,
        var_nfactoredge_i_slot: &mut f64,
        var_nfactoredge_i_rv_slot: &mut f64,
        var_noia3_i_slot: &mut f64,
        var_noia3_i_rv_slot: &mut f64,
        var_qsref_i_slot: &mut f64,
        var_qsref_i_rv_slot: &mut f64,
        var_steta0edge_i_slot: &mut f64,
        var_steta0edge_i_rv_slot: &mut f64,
        var_stk2edge_i_slot: &mut f64,
        var_stk2edge_i_rv_slot: &mut f64,
        var_teta0edge_i_slot: &mut f64,
        var_teta0edge_i_rv_slot: &mut f64,
        var_tnfactoredge_i_slot: &mut f64,
        var_tnfactoredge_i_rv_slot: &mut f64,
        var_u0r_i_slot: &mut f64,
        var_u0r_i_rv_slot: &mut f64,
        var_uar_i_slot: &mut f64,
        var_uar_i_dn0_slot: &mut f64,
        var_uar_i_dn10_slot: &mut f64,
        var_uar_i_dn11_slot: &mut f64,
        var_uar_i_dn12_slot: &mut f64,
        var_uar_i_dn13_slot: &mut f64,
        var_uar_i_dn14_slot: &mut f64,
        var_uar_i_dn2_slot: &mut f64,
        var_uar_i_dn3_slot: &mut f64,
        var_uar_i_dn4_slot: &mut f64,
        var_uar_i_dn5_slot: &mut f64,
        var_uar_i_dn6_slot: &mut f64,
        var_uar_i_dn7_slot: &mut f64,
        var_uar_i_dn8_slot: &mut f64,
        var_uar_i_dn9_slot: &mut f64,
        var_uar_i_rv_slot: &mut f64,
        var_ucsr_i_slot: &mut f64,
        var_ucsr_i_rv_slot: &mut f64,
        var_udr_i_slot: &mut f64,
        var_udr_i_dn0_slot: &mut f64,
        var_udr_i_dn10_slot: &mut f64,
        var_udr_i_dn11_slot: &mut f64,
        var_udr_i_dn12_slot: &mut f64,
        var_udr_i_dn13_slot: &mut f64,
        var_udr_i_dn14_slot: &mut f64,
        var_udr_i_dn2_slot: &mut f64,
        var_udr_i_dn3_slot: &mut f64,
        var_udr_i_dn4_slot: &mut f64,
        var_udr_i_dn5_slot: &mut f64,
        var_udr_i_dn6_slot: &mut f64,
        var_udr_i_dn7_slot: &mut f64,
        var_udr_i_dn8_slot: &mut f64,
        var_udr_i_dn9_slot: &mut f64,
        var_udr_i_rv_slot: &mut f64,
    ) {
        let mut var_a21_i: f64 = *var_a21_i_slot;
        let mut var_a21_i_rv: f64 = *var_a21_i_rv_slot;
        let mut var_a2_i: f64 = *var_a2_i_slot;
        let mut var_a2_i_rv: f64 = *var_a2_i_rv_slot;
        let mut var_c01_i: f64 = *var_c01_i_slot;
        let mut var_c01_i_rv: f64 = *var_c01_i_rv_slot;
        let mut var_c0_i: f64 = *var_c0_i_slot;
        let mut var_c0_i_rv: f64 = *var_c0_i_rv_slot;
        let mut var_c0si1_i: f64 = *var_c0si1_i_slot;
        let mut var_c0si1_i_rv: f64 = *var_c0si1_i_rv_slot;
        let mut var_c0si_i: f64 = *var_c0si_i_slot;
        let mut var_c0si_i_rv: f64 = *var_c0si_i_rv_slot;
        let mut var_c0sisat1_i: f64 = *var_c0sisat1_i_slot;
        let mut var_c0sisat1_i_rv: f64 = *var_c0sisat1_i_rv_slot;
        let mut var_c0sisat_i: f64 = *var_c0sisat_i_slot;
        let mut var_c0sisat_i_rv: f64 = *var_c0sisat_i_rv_slot;
        let mut var_cdscbedge_i: f64 = *var_cdscbedge_i_slot;
        let mut var_cdscbedge_i_rv: f64 = *var_cdscbedge_i_rv_slot;
        let mut var_cdscdedge_i: f64 = *var_cdscdedge_i_slot;
        let mut var_cdscdedge_i_rv: f64 = *var_cdscdedge_i_rv_slot;
        let mut var_cdscdr_i: f64 = *var_cdscdr_i_slot;
        let mut var_cdscdr_i_dn0: f64 = *var_cdscdr_i_dn0_slot;
        let mut var_cdscdr_i_dn10: f64 = *var_cdscdr_i_dn10_slot;
        let mut var_cdscdr_i_dn11: f64 = *var_cdscdr_i_dn11_slot;
        let mut var_cdscdr_i_dn12: f64 = *var_cdscdr_i_dn12_slot;
        let mut var_cdscdr_i_dn13: f64 = *var_cdscdr_i_dn13_slot;
        let mut var_cdscdr_i_dn14: f64 = *var_cdscdr_i_dn14_slot;
        let mut var_cdscdr_i_dn2: f64 = *var_cdscdr_i_dn2_slot;
        let mut var_cdscdr_i_dn3: f64 = *var_cdscdr_i_dn3_slot;
        let mut var_cdscdr_i_dn4: f64 = *var_cdscdr_i_dn4_slot;
        let mut var_cdscdr_i_dn5: f64 = *var_cdscdr_i_dn5_slot;
        let mut var_cdscdr_i_dn6: f64 = *var_cdscdr_i_dn6_slot;
        let mut var_cdscdr_i_dn7: f64 = *var_cdscdr_i_dn7_slot;
        let mut var_cdscdr_i_dn8: f64 = *var_cdscdr_i_dn8_slot;
        let mut var_cdscdr_i_dn9: f64 = *var_cdscdr_i_dn9_slot;
        let mut var_cdscdr_i_rv: f64 = *var_cdscdr_i_rv_slot;
        let mut var_citedge_i: f64 = *var_citedge_i_slot;
        let mut var_citedge_i_rv: f64 = *var_citedge_i_rv_slot;
        let mut var_eta0edge_i: f64 = *var_eta0edge_i_slot;
        let mut var_eta0edge_i_dn0: f64 = *var_eta0edge_i_dn0_slot;
        let mut var_eta0edge_i_dn10: f64 = *var_eta0edge_i_dn10_slot;
        let mut var_eta0edge_i_dn11: f64 = *var_eta0edge_i_dn11_slot;
        let mut var_eta0edge_i_dn12: f64 = *var_eta0edge_i_dn12_slot;
        let mut var_eta0edge_i_dn13: f64 = *var_eta0edge_i_dn13_slot;
        let mut var_eta0edge_i_dn14: f64 = *var_eta0edge_i_dn14_slot;
        let mut var_eta0edge_i_dn2: f64 = *var_eta0edge_i_dn2_slot;
        let mut var_eta0edge_i_dn3: f64 = *var_eta0edge_i_dn3_slot;
        let mut var_eta0edge_i_dn4: f64 = *var_eta0edge_i_dn4_slot;
        let mut var_eta0edge_i_dn5: f64 = *var_eta0edge_i_dn5_slot;
        let mut var_eta0edge_i_dn6: f64 = *var_eta0edge_i_dn6_slot;
        let mut var_eta0edge_i_dn7: f64 = *var_eta0edge_i_dn7_slot;
        let mut var_eta0edge_i_dn8: f64 = *var_eta0edge_i_dn8_slot;
        let mut var_eta0edge_i_dn9: f64 = *var_eta0edge_i_dn9_slot;
        let mut var_eta0edge_i_rv: f64 = *var_eta0edge_i_rv_slot;
        let mut var_eta0r_i: f64 = *var_eta0r_i_slot;
        let mut var_eta0r_i_dn0: f64 = *var_eta0r_i_dn0_slot;
        let mut var_eta0r_i_dn10: f64 = *var_eta0r_i_dn10_slot;
        let mut var_eta0r_i_dn11: f64 = *var_eta0r_i_dn11_slot;
        let mut var_eta0r_i_dn12: f64 = *var_eta0r_i_dn12_slot;
        let mut var_eta0r_i_dn13: f64 = *var_eta0r_i_dn13_slot;
        let mut var_eta0r_i_dn14: f64 = *var_eta0r_i_dn14_slot;
        let mut var_eta0r_i_dn2: f64 = *var_eta0r_i_dn2_slot;
        let mut var_eta0r_i_dn3: f64 = *var_eta0r_i_dn3_slot;
        let mut var_eta0r_i_dn4: f64 = *var_eta0r_i_dn4_slot;
        let mut var_eta0r_i_dn5: f64 = *var_eta0r_i_dn5_slot;
        let mut var_eta0r_i_dn6: f64 = *var_eta0r_i_dn6_slot;
        let mut var_eta0r_i_dn7: f64 = *var_eta0r_i_dn7_slot;
        let mut var_eta0r_i_dn8: f64 = *var_eta0r_i_dn8_slot;
        let mut var_eta0r_i_dn9: f64 = *var_eta0r_i_dn9_slot;
        let mut var_eta0r_i_rv: f64 = *var_eta0r_i_rv_slot;
        let mut var_etabedge_i: f64 = *var_etabedge_i_slot;
        let mut var_etabedge_i_rv: f64 = *var_etabedge_i_rv_slot;
        let mut var_guard21: f64 = *var_guard21_slot;
        let mut var_guard21_rv: f64 = *var_guard21_rv_slot;
        let mut var_k01_i: f64 = *var_k01_i_slot;
        let mut var_k01_i_rv: f64 = *var_k01_i_rv_slot;
        let mut var_k0_i: f64 = *var_k0_i_slot;
        let mut var_k0_i_rv: f64 = *var_k0_i_rv_slot;
        let mut var_k2edge_i: f64 = *var_k2edge_i_slot;
        let mut var_k2edge_i_dn0: f64 = *var_k2edge_i_dn0_slot;
        let mut var_k2edge_i_dn10: f64 = *var_k2edge_i_dn10_slot;
        let mut var_k2edge_i_dn11: f64 = *var_k2edge_i_dn11_slot;
        let mut var_k2edge_i_dn12: f64 = *var_k2edge_i_dn12_slot;
        let mut var_k2edge_i_dn13: f64 = *var_k2edge_i_dn13_slot;
        let mut var_k2edge_i_dn14: f64 = *var_k2edge_i_dn14_slot;
        let mut var_k2edge_i_dn2: f64 = *var_k2edge_i_dn2_slot;
        let mut var_k2edge_i_dn3: f64 = *var_k2edge_i_dn3_slot;
        let mut var_k2edge_i_dn4: f64 = *var_k2edge_i_dn4_slot;
        let mut var_k2edge_i_dn5: f64 = *var_k2edge_i_dn5_slot;
        let mut var_k2edge_i_dn6: f64 = *var_k2edge_i_dn6_slot;
        let mut var_k2edge_i_dn7: f64 = *var_k2edge_i_dn7_slot;
        let mut var_k2edge_i_dn8: f64 = *var_k2edge_i_dn8_slot;
        let mut var_k2edge_i_dn9: f64 = *var_k2edge_i_dn9_slot;
        let mut var_k2edge_i_rv: f64 = *var_k2edge_i_rv_slot;
        let mut var_k2edgewe_i: f64 = *var_k2edgewe_i_slot;
        let mut var_k2edgewe_i_rv: f64 = *var_k2edgewe_i_rv_slot;
        let mut var_kt1edge_i: f64 = *var_kt1edge_i_slot;
        let mut var_kt1edge_i_rv: f64 = *var_kt1edge_i_rv_slot;
        let mut var_kt1expedge_i: f64 = *var_kt1expedge_i_slot;
        let mut var_kt1expedge_i_rv: f64 = *var_kt1expedge_i_rv_slot;
        let mut var_kt1ledge_i: f64 = *var_kt1ledge_i_slot;
        let mut var_kt1ledge_i_rv: f64 = *var_kt1ledge_i_rv_slot;
        let mut var_kt2edge_i: f64 = *var_kt2edge_i_slot;
        let mut var_kt2edge_i_rv: f64 = *var_kt2edge_i_rv_slot;
        let mut var_kvth0edge_i: f64 = *var_kvth0edge_i_slot;
        let mut var_kvth0edge_i_rv: f64 = *var_kvth0edge_i_rv_slot;
        let mut var_kvth0edgewe_i: f64 = *var_kvth0edgewe_i_slot;
        let mut var_kvth0edgewe_i_rv: f64 = *var_kvth0edgewe_i_rv_slot;
        let mut var_m01_i: f64 = *var_m01_i_slot;
        let mut var_m01_i_rv: f64 = *var_m01_i_rv_slot;
        let mut var_m0_i: f64 = *var_m0_i_slot;
        let mut var_m0_i_rv: f64 = *var_m0_i_rv_slot;
        let mut var_mpower_i: f64 = *var_mpower_i_slot;
        let mut var_mpower_i_rv: f64 = *var_mpower_i_rv_slot;
        let mut var_ndepedge_i: f64 = *var_ndepedge_i_slot;
        let mut var_ndepedge_i_rv: f64 = *var_ndepedge_i_rv_slot;
        let mut var_nfactoredge_i: f64 = *var_nfactoredge_i_slot;
        let mut var_nfactoredge_i_rv: f64 = *var_nfactoredge_i_rv_slot;
        let mut var_noia3_i: f64 = *var_noia3_i_slot;
        let mut var_noia3_i_rv: f64 = *var_noia3_i_rv_slot;
        let mut var_qsref_i: f64 = *var_qsref_i_slot;
        let mut var_qsref_i_rv: f64 = *var_qsref_i_rv_slot;
        let mut var_steta0edge_i: f64 = *var_steta0edge_i_slot;
        let mut var_steta0edge_i_rv: f64 = *var_steta0edge_i_rv_slot;
        let mut var_stk2edge_i: f64 = *var_stk2edge_i_slot;
        let mut var_stk2edge_i_rv: f64 = *var_stk2edge_i_rv_slot;
        let mut var_teta0edge_i: f64 = *var_teta0edge_i_slot;
        let mut var_teta0edge_i_rv: f64 = *var_teta0edge_i_rv_slot;
        let mut var_tnfactoredge_i: f64 = *var_tnfactoredge_i_slot;
        let mut var_tnfactoredge_i_rv: f64 = *var_tnfactoredge_i_rv_slot;
        let mut var_u0r_i: f64 = *var_u0r_i_slot;
        let mut var_u0r_i_rv: f64 = *var_u0r_i_rv_slot;
        let mut var_uar_i: f64 = *var_uar_i_slot;
        let mut var_uar_i_dn0: f64 = *var_uar_i_dn0_slot;
        let mut var_uar_i_dn10: f64 = *var_uar_i_dn10_slot;
        let mut var_uar_i_dn11: f64 = *var_uar_i_dn11_slot;
        let mut var_uar_i_dn12: f64 = *var_uar_i_dn12_slot;
        let mut var_uar_i_dn13: f64 = *var_uar_i_dn13_slot;
        let mut var_uar_i_dn14: f64 = *var_uar_i_dn14_slot;
        let mut var_uar_i_dn2: f64 = *var_uar_i_dn2_slot;
        let mut var_uar_i_dn3: f64 = *var_uar_i_dn3_slot;
        let mut var_uar_i_dn4: f64 = *var_uar_i_dn4_slot;
        let mut var_uar_i_dn5: f64 = *var_uar_i_dn5_slot;
        let mut var_uar_i_dn6: f64 = *var_uar_i_dn6_slot;
        let mut var_uar_i_dn7: f64 = *var_uar_i_dn7_slot;
        let mut var_uar_i_dn8: f64 = *var_uar_i_dn8_slot;
        let mut var_uar_i_dn9: f64 = *var_uar_i_dn9_slot;
        let mut var_uar_i_rv: f64 = *var_uar_i_rv_slot;
        let mut var_ucsr_i: f64 = *var_ucsr_i_slot;
        let mut var_ucsr_i_rv: f64 = *var_ucsr_i_rv_slot;
        let mut var_udr_i: f64 = *var_udr_i_slot;
        let mut var_udr_i_dn0: f64 = *var_udr_i_dn0_slot;
        let mut var_udr_i_dn10: f64 = *var_udr_i_dn10_slot;
        let mut var_udr_i_dn11: f64 = *var_udr_i_dn11_slot;
        let mut var_udr_i_dn12: f64 = *var_udr_i_dn12_slot;
        let mut var_udr_i_dn13: f64 = *var_udr_i_dn13_slot;
        let mut var_udr_i_dn14: f64 = *var_udr_i_dn14_slot;
        let mut var_udr_i_dn2: f64 = *var_udr_i_dn2_slot;
        let mut var_udr_i_dn3: f64 = *var_udr_i_dn3_slot;
        let mut var_udr_i_dn4: f64 = *var_udr_i_dn4_slot;
        let mut var_udr_i_dn5: f64 = *var_udr_i_dn5_slot;
        let mut var_udr_i_dn6: f64 = *var_udr_i_dn6_slot;
        let mut var_udr_i_dn7: f64 = *var_udr_i_dn7_slot;
        let mut var_udr_i_dn8: f64 = *var_udr_i_dn8_slot;
        let mut var_udr_i_dn9: f64 = *var_udr_i_dn9_slot;
        let mut var_udr_i_rv: f64 = *var_udr_i_rv_slot;

        let assign2780_e3896: f64 = (var_bin_l * p.p453);
        let assign2780_e3897: f64 = (p.p452 + assign2780_e3896);
        let assign2780_e3900: f64 = (var_bin_w * p.p454);
        let assign2780_e3901: f64 = (assign2780_e3897 + assign2780_e3900);
        let assign2780_e3904: f64 = (var_bin_wl * p.p455);
        let assign2780_e3905: f64 = (assign2780_e3901 + assign2780_e3904);
        var_a2_i = assign2780_e3905;
        var_a2_i_rv = 0.0;

        let assign2790_e3909: f64 = (var_bin_l * p.p457);
        let assign2790_e3910: f64 = (p.p456 + assign2790_e3909);
        let assign2790_e3913: f64 = (var_bin_w * p.p458);
        let assign2790_e3914: f64 = (assign2790_e3910 + assign2790_e3913);
        let assign2790_e3917: f64 = (var_bin_wl * p.p459);
        let assign2790_e3918: f64 = (assign2790_e3914 + assign2790_e3917);
        var_a21_i = assign2790_e3918;
        var_a21_i_rv = 0.0;

        let assign2800_e3922: f64 = (var_bin_l * p.p1047);
        let assign2800_e3923: f64 = (p.p1046 + assign2800_e3922);
        let assign2800_e3926: f64 = (var_bin_w * p.p1048);
        let assign2800_e3927: f64 = (assign2800_e3923 + assign2800_e3926);
        let assign2800_e3930: f64 = (var_bin_wl * p.p1049);
        let assign2800_e3931: f64 = (assign2800_e3927 + assign2800_e3930);
        var_k0_i = assign2800_e3931;
        var_k0_i_rv = 0.0;

        let assign2810_e3935: f64 = (var_bin_l * p.p1055);
        let assign2810_e3936: f64 = (p.p1054 + assign2810_e3935);
        let assign2810_e3939: f64 = (var_bin_w * p.p1056);
        let assign2810_e3940: f64 = (assign2810_e3936 + assign2810_e3939);
        let assign2810_e3943: f64 = (var_bin_wl * p.p1057);
        let assign2810_e3944: f64 = (assign2810_e3940 + assign2810_e3943);
        var_m0_i = assign2810_e3944;
        var_m0_i_rv = 0.0;

        let assign2820_e3948: f64 = (var_bin_l * p.p1051);
        let assign2820_e3949: f64 = (p.p1050 + assign2820_e3948);
        let assign2820_e3952: f64 = (var_bin_w * p.p1052);
        let assign2820_e3953: f64 = (assign2820_e3949 + assign2820_e3952);
        let assign2820_e3956: f64 = (var_bin_wl * p.p1053);
        let assign2820_e3957: f64 = (assign2820_e3953 + assign2820_e3956);
        var_k01_i = assign2820_e3957;
        var_k01_i_rv = 0.0;

        let assign2830_e3961: f64 = (var_bin_l * p.p1059);
        let assign2830_e3962: f64 = (p.p1058 + assign2830_e3961);
        let assign2830_e3965: f64 = (var_bin_w * p.p1060);
        let assign2830_e3966: f64 = (assign2830_e3962 + assign2830_e3965);
        let assign2830_e3969: f64 = (var_bin_wl * p.p1061);
        let assign2830_e3970: f64 = (assign2830_e3966 + assign2830_e3969);
        var_m01_i = assign2830_e3970;
        var_m01_i_rv = 0.0;

        let assign2840_e3974: f64 = (var_bin_l * p.p967);
        let assign2840_e3975: f64 = (p.p966 + assign2840_e3974);
        let assign2840_e3978: f64 = (var_bin_w * p.p968);
        let assign2840_e3979: f64 = (assign2840_e3975 + assign2840_e3978);
        let assign2840_e3982: f64 = (var_bin_wl * p.p969);
        let assign2840_e3983: f64 = (assign2840_e3979 + assign2840_e3982);
        var_nfactoredge_i = assign2840_e3983;
        var_nfactoredge_i_rv = 0.0;

        let assign2850_e3987: f64 = (var_bin_l * p.p963);
        let assign2850_e3988: f64 = (p.p962 + assign2850_e3987);
        let assign2850_e3991: f64 = (var_bin_w * p.p964);
        let assign2850_e3992: f64 = (assign2850_e3988 + assign2850_e3991);
        let assign2850_e3995: f64 = (var_bin_wl * p.p965);
        let assign2850_e3996: f64 = (assign2850_e3992 + assign2850_e3995);
        var_ndepedge_i = assign2850_e3996;
        var_ndepedge_i_rv = 0.0;

        let assign2860_e4000: f64 = (var_bin_l * p.p971);
        let assign2860_e4001: f64 = (p.p970 + assign2860_e4000);
        let assign2860_e4004: f64 = (var_bin_w * p.p972);
        let assign2860_e4005: f64 = (assign2860_e4001 + assign2860_e4004);
        let assign2860_e4008: f64 = (var_bin_wl * p.p973);
        let assign2860_e4009: f64 = (assign2860_e4005 + assign2860_e4008);
        var_citedge_i = assign2860_e4009;
        var_citedge_i_rv = 0.0;

        let assign2870_e4013: f64 = (var_bin_l * p.p975);
        let assign2870_e4014: f64 = (p.p974 + assign2870_e4013);
        let assign2870_e4017: f64 = (var_bin_w * p.p976);
        let assign2870_e4018: f64 = (assign2870_e4014 + assign2870_e4017);
        let assign2870_e4021: f64 = (var_bin_wl * p.p977);
        let assign2870_e4022: f64 = (assign2870_e4018 + assign2870_e4021);
        var_cdscdedge_i = assign2870_e4022;
        var_cdscdedge_i_rv = 0.0;

        let assign2880_e4026: f64 = (var_bin_l * p.p979);
        let assign2880_e4027: f64 = (p.p978 + assign2880_e4026);
        let assign2880_e4030: f64 = (var_bin_w * p.p980);
        let assign2880_e4031: f64 = (assign2880_e4027 + assign2880_e4030);
        let assign2880_e4034: f64 = (var_bin_wl * p.p981);
        let assign2880_e4035: f64 = (assign2880_e4031 + assign2880_e4034);
        var_cdscbedge_i = assign2880_e4035;
        var_cdscbedge_i_rv = 0.0;

        let assign2890_e4039: f64 = (var_bin_l * p.p983);
        let assign2890_e4040: f64 = (p.p982 + assign2890_e4039);
        let assign2890_e4043: f64 = (var_bin_w * p.p984);
        let assign2890_e4044: f64 = (assign2890_e4040 + assign2890_e4043);
        let assign2890_e4047: f64 = (var_bin_wl * p.p985);
        let assign2890_e4048: f64 = (assign2890_e4044 + assign2890_e4047);
        var_eta0edge_i = assign2890_e4048;
        var_eta0edge_i_dn0 = 0.0;
        var_eta0edge_i_dn2 = 0.0;
        var_eta0edge_i_dn3 = 0.0;
        var_eta0edge_i_dn4 = 0.0;
        var_eta0edge_i_dn5 = 0.0;
        var_eta0edge_i_dn6 = 0.0;
        var_eta0edge_i_dn7 = 0.0;
        var_eta0edge_i_dn8 = 0.0;
        var_eta0edge_i_dn9 = 0.0;
        var_eta0edge_i_dn10 = 0.0;
        var_eta0edge_i_dn11 = 0.0;
        var_eta0edge_i_dn12 = 0.0;
        var_eta0edge_i_dn13 = 0.0;
        var_eta0edge_i_dn14 = 0.0;
        var_eta0edge_i_rv = 0.0;

        let assign2900_e4052: f64 = (var_bin_l * p.p987);
        let assign2900_e4053: f64 = (p.p986 + assign2900_e4052);
        let assign2900_e4056: f64 = (var_bin_w * p.p988);
        let assign2900_e4057: f64 = (assign2900_e4053 + assign2900_e4056);
        let assign2900_e4060: f64 = (var_bin_wl * p.p989);
        let assign2900_e4061: f64 = (assign2900_e4057 + assign2900_e4060);
        var_etabedge_i = assign2900_e4061;
        var_etabedge_i_rv = 0.0;

        let assign2910_e4065: f64 = (var_bin_l * p.p991);
        let assign2910_e4066: f64 = (p.p990 + assign2910_e4065);
        let assign2910_e4069: f64 = (var_bin_w * p.p992);
        let assign2910_e4070: f64 = (assign2910_e4066 + assign2910_e4069);
        let assign2910_e4073: f64 = (var_bin_wl * p.p993);
        let assign2910_e4074: f64 = (assign2910_e4070 + assign2910_e4073);
        var_kt1edge_i = assign2910_e4074;
        var_kt1edge_i_rv = 0.0;

        let assign2920_e4078: f64 = (var_bin_l * p.p995);
        let assign2920_e4079: f64 = (p.p994 + assign2920_e4078);
        let assign2920_e4082: f64 = (var_bin_w * p.p996);
        let assign2920_e4083: f64 = (assign2920_e4079 + assign2920_e4082);
        let assign2920_e4086: f64 = (var_bin_wl * p.p997);
        let assign2920_e4087: f64 = (assign2920_e4083 + assign2920_e4086);
        var_kt1ledge_i = assign2920_e4087;
        var_kt1ledge_i_rv = 0.0;

        let assign2930_e4091: f64 = (var_bin_l * p.p999);
        let assign2930_e4092: f64 = (p.p998 + assign2930_e4091);
        let assign2930_e4095: f64 = (var_bin_w * p.p1000);
        let assign2930_e4096: f64 = (assign2930_e4092 + assign2930_e4095);
        let assign2930_e4099: f64 = (var_bin_wl * p.p1001);
        let assign2930_e4100: f64 = (assign2930_e4096 + assign2930_e4099);
        var_kt2edge_i = assign2930_e4100;
        var_kt2edge_i_rv = 0.0;

        let assign2940_e4104: f64 = (var_bin_l * p.p1003);
        let assign2940_e4105: f64 = (p.p1002 + assign2940_e4104);
        let assign2940_e4108: f64 = (var_bin_w * p.p1004);
        let assign2940_e4109: f64 = (assign2940_e4105 + assign2940_e4108);
        let assign2940_e4112: f64 = (var_bin_wl * p.p1005);
        let assign2940_e4113: f64 = (assign2940_e4109 + assign2940_e4112);
        var_kt1expedge_i = assign2940_e4113;
        var_kt1expedge_i_rv = 0.0;

        let assign2950_e4117: f64 = (var_bin_l * p.p1007);
        let assign2950_e4118: f64 = (p.p1006 + assign2950_e4117);
        let assign2950_e4121: f64 = (var_bin_w * p.p1008);
        let assign2950_e4122: f64 = (assign2950_e4118 + assign2950_e4121);
        let assign2950_e4125: f64 = (var_bin_wl * p.p1009);
        let assign2950_e4126: f64 = (assign2950_e4122 + assign2950_e4125);
        var_tnfactoredge_i = assign2950_e4126;
        var_tnfactoredge_i_rv = 0.0;

        let assign2960_e4130: f64 = (var_bin_l * p.p1011);
        let assign2960_e4131: f64 = (p.p1010 + assign2960_e4130);
        let assign2960_e4134: f64 = (var_bin_w * p.p1012);
        let assign2960_e4135: f64 = (assign2960_e4131 + assign2960_e4134);
        let assign2960_e4138: f64 = (var_bin_wl * p.p1013);
        let assign2960_e4139: f64 = (assign2960_e4135 + assign2960_e4138);
        var_teta0edge_i = assign2960_e4139;
        var_teta0edge_i_rv = 0.0;

        let assign2970_e4143: f64 = (var_bin_l * p.p1018);
        let assign2970_e4144: f64 = (p.p1017 + assign2970_e4143);
        let assign2970_e4147: f64 = (var_bin_w * p.p1019);
        let assign2970_e4148: f64 = (assign2970_e4144 + assign2970_e4147);
        let assign2970_e4151: f64 = (var_bin_wl * p.p1020);
        let assign2970_e4152: f64 = (assign2970_e4148 + assign2970_e4151);
        var_k2edge_i = assign2970_e4152;
        var_k2edge_i_dn0 = 0.0;
        var_k2edge_i_dn2 = 0.0;
        var_k2edge_i_dn3 = 0.0;
        var_k2edge_i_dn4 = 0.0;
        var_k2edge_i_dn5 = 0.0;
        var_k2edge_i_dn6 = 0.0;
        var_k2edge_i_dn7 = 0.0;
        var_k2edge_i_dn8 = 0.0;
        var_k2edge_i_dn9 = 0.0;
        var_k2edge_i_dn10 = 0.0;
        var_k2edge_i_dn11 = 0.0;
        var_k2edge_i_dn12 = 0.0;
        var_k2edge_i_dn13 = 0.0;
        var_k2edge_i_dn14 = 0.0;
        var_k2edge_i_rv = 0.0;

        let assign2980_e4156: f64 = (var_bin_l * p.p1022);
        let assign2980_e4157: f64 = (p.p1021 + assign2980_e4156);
        let assign2980_e4160: f64 = (var_bin_w * p.p1023);
        let assign2980_e4161: f64 = (assign2980_e4157 + assign2980_e4160);
        let assign2980_e4164: f64 = (var_bin_wl * p.p1024);
        let assign2980_e4165: f64 = (assign2980_e4161 + assign2980_e4164);
        var_kvth0edge_i = assign2980_e4165;
        var_kvth0edge_i_rv = 0.0;

        let assign2990_e4169: f64 = (var_bin_l * p.p1030);
        let assign2990_e4170: f64 = (p.p1029 + assign2990_e4169);
        let assign2990_e4173: f64 = (var_bin_w * p.p1031);
        let assign2990_e4174: f64 = (assign2990_e4170 + assign2990_e4173);
        let assign2990_e4177: f64 = (var_bin_wl * p.p1032);
        let assign2990_e4178: f64 = (assign2990_e4174 + assign2990_e4177);
        var_k2edgewe_i = assign2990_e4178;
        var_k2edgewe_i_rv = 0.0;

        let assign3000_e4182: f64 = (var_bin_l * p.p1026);
        let assign3000_e4183: f64 = (p.p1025 + assign3000_e4182);
        let assign3000_e4186: f64 = (var_bin_w * p.p1027);
        let assign3000_e4187: f64 = (assign3000_e4183 + assign3000_e4186);
        let assign3000_e4190: f64 = (var_bin_wl * p.p1028);
        let assign3000_e4191: f64 = (assign3000_e4187 + assign3000_e4190);
        var_kvth0edgewe_i = assign3000_e4191;
        var_kvth0edgewe_i_rv = 0.0;

        let assign3010_e4195: f64 = (var_bin_l * p.p1034);
        let assign3010_e4196: f64 = (p.p1033 + assign3010_e4195);
        let assign3010_e4199: f64 = (var_bin_w * p.p1035);
        let assign3010_e4200: f64 = (assign3010_e4196 + assign3010_e4199);
        let assign3010_e4203: f64 = (var_bin_wl * p.p1036);
        let assign3010_e4204: f64 = (assign3010_e4200 + assign3010_e4203);
        var_stk2edge_i = assign3010_e4204;
        var_stk2edge_i_rv = 0.0;

        let assign3020_e4208: f64 = (var_bin_l * p.p1038);
        let assign3020_e4209: f64 = (p.p1037 + assign3020_e4208);
        let assign3020_e4212: f64 = (var_bin_w * p.p1039);
        let assign3020_e4213: f64 = (assign3020_e4209 + assign3020_e4212);
        let assign3020_e4216: f64 = (var_bin_wl * p.p1040);
        let assign3020_e4217: f64 = (assign3020_e4213 + assign3020_e4216);
        var_steta0edge_i = assign3020_e4217;
        var_steta0edge_i_rv = 0.0;

        let assign3030_e4221: f64 = (var_bin_l * p.p1070);
        let assign3030_e4222: f64 = (p.p1069 + assign3030_e4221);
        let assign3030_e4225: f64 = (var_bin_w * p.p1071);
        let assign3030_e4226: f64 = (assign3030_e4222 + assign3030_e4225);
        let assign3030_e4229: f64 = (var_bin_wl * p.p1072);
        let assign3030_e4230: f64 = (assign3030_e4226 + assign3030_e4229);
        var_c0_i = assign3030_e4230;
        var_c0_i_rv = 0.0;

        let assign3040_e4234: f64 = (var_bin_l * p.p1074);
        let assign3040_e4235: f64 = (p.p1073 + assign3040_e4234);
        let assign3040_e4238: f64 = (var_bin_w * p.p1075);
        let assign3040_e4239: f64 = (assign3040_e4235 + assign3040_e4238);
        let assign3040_e4242: f64 = (var_bin_wl * p.p1076);
        let assign3040_e4243: f64 = (assign3040_e4239 + assign3040_e4242);
        var_c01_i = assign3040_e4243;
        var_c01_i_rv = 0.0;

        let assign3050_e4247: f64 = (var_bin_l * p.p1078);
        let assign3050_e4248: f64 = (p.p1077 + assign3050_e4247);
        let assign3050_e4251: f64 = (var_bin_w * p.p1079);
        let assign3050_e4252: f64 = (assign3050_e4248 + assign3050_e4251);
        let assign3050_e4255: f64 = (var_bin_wl * p.p1080);
        let assign3050_e4256: f64 = (assign3050_e4252 + assign3050_e4255);
        var_c0si_i = assign3050_e4256;
        var_c0si_i_rv = 0.0;

        let assign3060_e4260: f64 = (var_bin_l * p.p1082);
        let assign3060_e4261: f64 = (p.p1081 + assign3060_e4260);
        let assign3060_e4264: f64 = (var_bin_w * p.p1083);
        let assign3060_e4265: f64 = (assign3060_e4261 + assign3060_e4264);
        let assign3060_e4268: f64 = (var_bin_wl * p.p1084);
        let assign3060_e4269: f64 = (assign3060_e4265 + assign3060_e4268);
        var_c0si1_i = assign3060_e4269;
        var_c0si1_i_rv = 0.0;

        let assign3070_e4273: f64 = (var_bin_l * p.p1086);
        let assign3070_e4274: f64 = (p.p1085 + assign3070_e4273);
        let assign3070_e4277: f64 = (var_bin_w * p.p1087);
        let assign3070_e4278: f64 = (assign3070_e4274 + assign3070_e4277);
        let assign3070_e4281: f64 = (var_bin_wl * p.p1088);
        let assign3070_e4282: f64 = (assign3070_e4278 + assign3070_e4281);
        var_c0sisat_i = assign3070_e4282;
        var_c0sisat_i_rv = 0.0;

        let assign3080_e4286: f64 = (var_bin_l * p.p1090);
        let assign3080_e4287: f64 = (p.p1089 + assign3080_e4286);
        let assign3080_e4290: f64 = (var_bin_w * p.p1091);
        let assign3080_e4291: f64 = (assign3080_e4287 + assign3080_e4290);
        let assign3080_e4294: f64 = (var_bin_wl * p.p1092);
        let assign3080_e4295: f64 = (assign3080_e4291 + assign3080_e4294);
        var_c0sisat1_i = assign3080_e4295;
        var_c0sisat1_i_rv = 0.0;

        let assign3090_e4299: f64 = (var_bin_l * p.p787);
        let assign3090_e4300: f64 = (p.p786 + assign3090_e4299);
        let assign3090_e4303: f64 = (var_bin_w * p.p788);
        let assign3090_e4304: f64 = (assign3090_e4300 + assign3090_e4303);
        let assign3090_e4307: f64 = (var_bin_wl * p.p789);
        let assign3090_e4308: f64 = (assign3090_e4304 + assign3090_e4307);
        var_noia3_i = assign3090_e4308;
        var_noia3_i_rv = 0.0;

        let assign3100_e4312: f64 = (var_bin_l * p.p795);
        let assign3100_e4313: f64 = (p.p794 + assign3100_e4312);
        let assign3100_e4316: f64 = (var_bin_w * p.p796);
        let assign3100_e4317: f64 = (assign3100_e4313 + assign3100_e4316);
        let assign3100_e4320: f64 = (var_bin_wl * p.p797);
        let assign3100_e4321: f64 = (assign3100_e4317 + assign3100_e4320);
        var_qsref_i = assign3100_e4321;
        var_qsref_i_rv = 0.0;

        let assign3110_e4325: f64 = (var_bin_l * p.p791);
        let assign3110_e4326: f64 = (p.p790 + assign3110_e4325);
        let assign3110_e4329: f64 = (var_bin_w * p.p792);
        let assign3110_e4330: f64 = (assign3110_e4326 + assign3110_e4329);
        let assign3110_e4333: f64 = (var_bin_wl * p.p793);
        let assign3110_e4334: f64 = (assign3110_e4330 + assign3110_e4333);
        var_mpower_i = assign3110_e4334;
        var_mpower_i_rv = 0.0;

        let assign3120_e4337: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        var_guard21 = assign3120_e4337;
        var_guard21_rv = 0.0;

        let (assign3130_e4353, assign3130_e4353_d_n0, assign3130_e4353_d_n2, assign3130_e4353_d_n3, assign3130_e4353_d_n4, assign3130_e4353_d_n5, assign3130_e4353_d_n6, assign3130_e4353_d_n7, assign3130_e4353_d_n8, assign3130_e4353_d_n9, assign3130_e4353_d_n10, assign3130_e4353_d_n11, assign3130_e4353_d_n12, assign3130_e4353_d_n13, assign3130_e4353_d_n14,) = {
    if (var_guard21 != 0.0) {
        let assign3130_e4342: f64 = (var_bin_l * p.p230);
        let assign3130_e4343: f64 = (p.p229 + assign3130_e4342);
        let assign3130_e4346: f64 = (var_bin_w * p.p231);
        let assign3130_e4347: f64 = (assign3130_e4343 + assign3130_e4346);
        let assign3130_e4350: f64 = (var_bin_wl * p.p232);
        let assign3130_e4351: f64 = (assign3130_e4347 + assign3130_e4350);
        (assign3130_e4351, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cdscdr_i, var_cdscdr_i_dn0, var_cdscdr_i_dn2, var_cdscdr_i_dn3, var_cdscdr_i_dn4, var_cdscdr_i_dn5, var_cdscdr_i_dn6, var_cdscdr_i_dn7, var_cdscdr_i_dn8, var_cdscdr_i_dn9, var_cdscdr_i_dn10, var_cdscdr_i_dn11, var_cdscdr_i_dn12, var_cdscdr_i_dn13, var_cdscdr_i_dn14,)
    }
};
        var_cdscdr_i = assign3130_e4353;
        var_cdscdr_i_dn0 = assign3130_e4353_d_n0;
        var_cdscdr_i_dn2 = assign3130_e4353_d_n2;
        var_cdscdr_i_dn3 = assign3130_e4353_d_n3;
        var_cdscdr_i_dn4 = assign3130_e4353_d_n4;
        var_cdscdr_i_dn5 = assign3130_e4353_d_n5;
        var_cdscdr_i_dn6 = assign3130_e4353_d_n6;
        var_cdscdr_i_dn7 = assign3130_e4353_d_n7;
        var_cdscdr_i_dn8 = assign3130_e4353_d_n8;
        var_cdscdr_i_dn9 = assign3130_e4353_d_n9;
        var_cdscdr_i_dn10 = assign3130_e4353_d_n10;
        var_cdscdr_i_dn11 = assign3130_e4353_d_n11;
        var_cdscdr_i_dn12 = assign3130_e4353_d_n12;
        var_cdscdr_i_dn13 = assign3130_e4353_d_n13;
        var_cdscdr_i_dn14 = assign3130_e4353_d_n14;
        var_cdscdr_i_rv = 0.0;

        let (assign3140_e4369, assign3140_e4369_d_n0, assign3140_e4369_d_n2, assign3140_e4369_d_n3, assign3140_e4369_d_n4, assign3140_e4369_d_n5, assign3140_e4369_d_n6, assign3140_e4369_d_n7, assign3140_e4369_d_n8, assign3140_e4369_d_n9, assign3140_e4369_d_n10, assign3140_e4369_d_n11, assign3140_e4369_d_n12, assign3140_e4369_d_n13, assign3140_e4369_d_n14,) = {
    if (var_guard21 != 0.0) {
        let assign3140_e4358: f64 = (var_bin_l * p.p176);
        let assign3140_e4359: f64 = (p.p175 + assign3140_e4358);
        let assign3140_e4362: f64 = (var_bin_w * p.p177);
        let assign3140_e4363: f64 = (assign3140_e4359 + assign3140_e4362);
        let assign3140_e4366: f64 = (var_bin_wl * p.p178);
        let assign3140_e4367: f64 = (assign3140_e4363 + assign3140_e4366);
        (assign3140_e4367, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_eta0r_i, var_eta0r_i_dn0, var_eta0r_i_dn2, var_eta0r_i_dn3, var_eta0r_i_dn4, var_eta0r_i_dn5, var_eta0r_i_dn6, var_eta0r_i_dn7, var_eta0r_i_dn8, var_eta0r_i_dn9, var_eta0r_i_dn10, var_eta0r_i_dn11, var_eta0r_i_dn12, var_eta0r_i_dn13, var_eta0r_i_dn14,)
    }
};
        var_eta0r_i = assign3140_e4369;
        var_eta0r_i_dn0 = assign3140_e4369_d_n0;
        var_eta0r_i_dn2 = assign3140_e4369_d_n2;
        var_eta0r_i_dn3 = assign3140_e4369_d_n3;
        var_eta0r_i_dn4 = assign3140_e4369_d_n4;
        var_eta0r_i_dn5 = assign3140_e4369_d_n5;
        var_eta0r_i_dn6 = assign3140_e4369_d_n6;
        var_eta0r_i_dn7 = assign3140_e4369_d_n7;
        var_eta0r_i_dn8 = assign3140_e4369_d_n8;
        var_eta0r_i_dn9 = assign3140_e4369_d_n9;
        var_eta0r_i_dn10 = assign3140_e4369_d_n10;
        var_eta0r_i_dn11 = assign3140_e4369_d_n11;
        var_eta0r_i_dn12 = assign3140_e4369_d_n12;
        var_eta0r_i_dn13 = assign3140_e4369_d_n13;
        var_eta0r_i_dn14 = assign3140_e4369_d_n14;
        var_eta0r_i_rv = 0.0;

        let (assign3150_e4385,) = {
    if (var_guard21 != 0.0) {
        let assign3150_e4374: f64 = (var_bin_l * p.p280);
        let assign3150_e4375: f64 = (p.p279 + assign3150_e4374);
        let assign3150_e4378: f64 = (var_bin_w * p.p281);
        let assign3150_e4379: f64 = (assign3150_e4375 + assign3150_e4378);
        let assign3150_e4382: f64 = (var_bin_wl * p.p282);
        let assign3150_e4383: f64 = (assign3150_e4379 + assign3150_e4382);
        (assign3150_e4383,)
    } else {
        (var_u0r_i,)
    }
};
        var_u0r_i = assign3150_e4385;
        var_u0r_i_rv = 0.0;

        let (assign3160_e4401, assign3160_e4401_d_n0, assign3160_e4401_d_n2, assign3160_e4401_d_n3, assign3160_e4401_d_n4, assign3160_e4401_d_n5, assign3160_e4401_d_n6, assign3160_e4401_d_n7, assign3160_e4401_d_n8, assign3160_e4401_d_n9, assign3160_e4401_d_n10, assign3160_e4401_d_n11, assign3160_e4401_d_n12, assign3160_e4401_d_n13, assign3160_e4401_d_n14,) = {
    if (var_guard21 != 0.0) {
        let assign3160_e4390: f64 = (var_bin_l * p.p295);
        let assign3160_e4391: f64 = (p.p294 + assign3160_e4390);
        let assign3160_e4394: f64 = (var_bin_w * p.p296);
        let assign3160_e4395: f64 = (assign3160_e4391 + assign3160_e4394);
        let assign3160_e4398: f64 = (var_bin_wl * p.p297);
        let assign3160_e4399: f64 = (assign3160_e4395 + assign3160_e4398);
        (assign3160_e4399, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uar_i, var_uar_i_dn0, var_uar_i_dn2, var_uar_i_dn3, var_uar_i_dn4, var_uar_i_dn5, var_uar_i_dn6, var_uar_i_dn7, var_uar_i_dn8, var_uar_i_dn9, var_uar_i_dn10, var_uar_i_dn11, var_uar_i_dn12, var_uar_i_dn13, var_uar_i_dn14,)
    }
};
        var_uar_i = assign3160_e4401;
        var_uar_i_dn0 = assign3160_e4401_d_n0;
        var_uar_i_dn2 = assign3160_e4401_d_n2;
        var_uar_i_dn3 = assign3160_e4401_d_n3;
        var_uar_i_dn4 = assign3160_e4401_d_n4;
        var_uar_i_dn5 = assign3160_e4401_d_n5;
        var_uar_i_dn6 = assign3160_e4401_d_n6;
        var_uar_i_dn7 = assign3160_e4401_d_n7;
        var_uar_i_dn8 = assign3160_e4401_d_n8;
        var_uar_i_dn9 = assign3160_e4401_d_n9;
        var_uar_i_dn10 = assign3160_e4401_d_n10;
        var_uar_i_dn11 = assign3160_e4401_d_n11;
        var_uar_i_dn12 = assign3160_e4401_d_n12;
        var_uar_i_dn13 = assign3160_e4401_d_n13;
        var_uar_i_dn14 = assign3160_e4401_d_n14;
        var_uar_i_rv = 0.0;

        let (assign3170_e4417, assign3170_e4417_d_n0, assign3170_e4417_d_n2, assign3170_e4417_d_n3, assign3170_e4417_d_n4, assign3170_e4417_d_n5, assign3170_e4417_d_n6, assign3170_e4417_d_n7, assign3170_e4417_d_n8, assign3170_e4417_d_n9, assign3170_e4417_d_n10, assign3170_e4417_d_n11, assign3170_e4417_d_n12, assign3170_e4417_d_n13, assign3170_e4417_d_n14,) = {
    if (var_guard21 != 0.0) {
        let assign3170_e4406: f64 = (var_bin_l * p.p315);
        let assign3170_e4407: f64 = (p.p314 + assign3170_e4406);
        let assign3170_e4410: f64 = (var_bin_w * p.p316);
        let assign3170_e4411: f64 = (assign3170_e4407 + assign3170_e4410);
        let assign3170_e4414: f64 = (var_bin_wl * p.p317);
        let assign3170_e4415: f64 = (assign3170_e4411 + assign3170_e4414);
        (assign3170_e4415, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_udr_i, var_udr_i_dn0, var_udr_i_dn2, var_udr_i_dn3, var_udr_i_dn4, var_udr_i_dn5, var_udr_i_dn6, var_udr_i_dn7, var_udr_i_dn8, var_udr_i_dn9, var_udr_i_dn10, var_udr_i_dn11, var_udr_i_dn12, var_udr_i_dn13, var_udr_i_dn14,)
    }
};
        var_udr_i = assign3170_e4417;
        var_udr_i_dn0 = assign3170_e4417_d_n0;
        var_udr_i_dn2 = assign3170_e4417_d_n2;
        var_udr_i_dn3 = assign3170_e4417_d_n3;
        var_udr_i_dn4 = assign3170_e4417_d_n4;
        var_udr_i_dn5 = assign3170_e4417_d_n5;
        var_udr_i_dn6 = assign3170_e4417_d_n6;
        var_udr_i_dn7 = assign3170_e4417_d_n7;
        var_udr_i_dn8 = assign3170_e4417_d_n8;
        var_udr_i_dn9 = assign3170_e4417_d_n9;
        var_udr_i_dn10 = assign3170_e4417_d_n10;
        var_udr_i_dn11 = assign3170_e4417_d_n11;
        var_udr_i_dn12 = assign3170_e4417_d_n12;
        var_udr_i_dn13 = assign3170_e4417_d_n13;
        var_udr_i_dn14 = assign3170_e4417_d_n14;
        var_udr_i_rv = 0.0;

        let (assign3180_e4433,) = {
    if (var_guard21 != 0.0) {
        let assign3180_e4422: f64 = (var_bin_l * p.p323);
        let assign3180_e4423: f64 = (p.p322 + assign3180_e4422);
        let assign3180_e4426: f64 = (var_bin_w * p.p324);
        let assign3180_e4427: f64 = (assign3180_e4423 + assign3180_e4426);
        let assign3180_e4430: f64 = (var_bin_wl * p.p325);
        let assign3180_e4431: f64 = (assign3180_e4427 + assign3180_e4430);
        (assign3180_e4431,)
    } else {
        (var_ucsr_i,)
    }
};
        var_ucsr_i = assign3180_e4433;
        var_ucsr_i_rv = 0.0;

        *var_a21_i_slot = var_a21_i;
        *var_a21_i_rv_slot = var_a21_i_rv;
        *var_a2_i_slot = var_a2_i;
        *var_a2_i_rv_slot = var_a2_i_rv;
        *var_c01_i_slot = var_c01_i;
        *var_c01_i_rv_slot = var_c01_i_rv;
        *var_c0_i_slot = var_c0_i;
        *var_c0_i_rv_slot = var_c0_i_rv;
        *var_c0si1_i_slot = var_c0si1_i;
        *var_c0si1_i_rv_slot = var_c0si1_i_rv;
        *var_c0si_i_slot = var_c0si_i;
        *var_c0si_i_rv_slot = var_c0si_i_rv;
        *var_c0sisat1_i_slot = var_c0sisat1_i;
        *var_c0sisat1_i_rv_slot = var_c0sisat1_i_rv;
        *var_c0sisat_i_slot = var_c0sisat_i;
        *var_c0sisat_i_rv_slot = var_c0sisat_i_rv;
        *var_cdscbedge_i_slot = var_cdscbedge_i;
        *var_cdscbedge_i_rv_slot = var_cdscbedge_i_rv;
        *var_cdscdedge_i_slot = var_cdscdedge_i;
        *var_cdscdedge_i_rv_slot = var_cdscdedge_i_rv;
        *var_cdscdr_i_slot = var_cdscdr_i;
        *var_cdscdr_i_dn0_slot = var_cdscdr_i_dn0;
        *var_cdscdr_i_dn10_slot = var_cdscdr_i_dn10;
        *var_cdscdr_i_dn11_slot = var_cdscdr_i_dn11;
        *var_cdscdr_i_dn12_slot = var_cdscdr_i_dn12;
        *var_cdscdr_i_dn13_slot = var_cdscdr_i_dn13;
        *var_cdscdr_i_dn14_slot = var_cdscdr_i_dn14;
        *var_cdscdr_i_dn2_slot = var_cdscdr_i_dn2;
        *var_cdscdr_i_dn3_slot = var_cdscdr_i_dn3;
        *var_cdscdr_i_dn4_slot = var_cdscdr_i_dn4;
        *var_cdscdr_i_dn5_slot = var_cdscdr_i_dn5;
        *var_cdscdr_i_dn6_slot = var_cdscdr_i_dn6;
        *var_cdscdr_i_dn7_slot = var_cdscdr_i_dn7;
        *var_cdscdr_i_dn8_slot = var_cdscdr_i_dn8;
        *var_cdscdr_i_dn9_slot = var_cdscdr_i_dn9;
        *var_cdscdr_i_rv_slot = var_cdscdr_i_rv;
        *var_citedge_i_slot = var_citedge_i;
        *var_citedge_i_rv_slot = var_citedge_i_rv;
        *var_eta0edge_i_slot = var_eta0edge_i;
        *var_eta0edge_i_dn0_slot = var_eta0edge_i_dn0;
        *var_eta0edge_i_dn10_slot = var_eta0edge_i_dn10;
        *var_eta0edge_i_dn11_slot = var_eta0edge_i_dn11;
        *var_eta0edge_i_dn12_slot = var_eta0edge_i_dn12;
        *var_eta0edge_i_dn13_slot = var_eta0edge_i_dn13;
        *var_eta0edge_i_dn14_slot = var_eta0edge_i_dn14;
        *var_eta0edge_i_dn2_slot = var_eta0edge_i_dn2;
        *var_eta0edge_i_dn3_slot = var_eta0edge_i_dn3;
        *var_eta0edge_i_dn4_slot = var_eta0edge_i_dn4;
        *var_eta0edge_i_dn5_slot = var_eta0edge_i_dn5;
        *var_eta0edge_i_dn6_slot = var_eta0edge_i_dn6;
        *var_eta0edge_i_dn7_slot = var_eta0edge_i_dn7;
        *var_eta0edge_i_dn8_slot = var_eta0edge_i_dn8;
        *var_eta0edge_i_dn9_slot = var_eta0edge_i_dn9;
        *var_eta0edge_i_rv_slot = var_eta0edge_i_rv;
        *var_eta0r_i_slot = var_eta0r_i;
        *var_eta0r_i_dn0_slot = var_eta0r_i_dn0;
        *var_eta0r_i_dn10_slot = var_eta0r_i_dn10;
        *var_eta0r_i_dn11_slot = var_eta0r_i_dn11;
        *var_eta0r_i_dn12_slot = var_eta0r_i_dn12;
        *var_eta0r_i_dn13_slot = var_eta0r_i_dn13;
        *var_eta0r_i_dn14_slot = var_eta0r_i_dn14;
        *var_eta0r_i_dn2_slot = var_eta0r_i_dn2;
        *var_eta0r_i_dn3_slot = var_eta0r_i_dn3;
        *var_eta0r_i_dn4_slot = var_eta0r_i_dn4;
        *var_eta0r_i_dn5_slot = var_eta0r_i_dn5;
        *var_eta0r_i_dn6_slot = var_eta0r_i_dn6;
        *var_eta0r_i_dn7_slot = var_eta0r_i_dn7;
        *var_eta0r_i_dn8_slot = var_eta0r_i_dn8;
        *var_eta0r_i_dn9_slot = var_eta0r_i_dn9;
        *var_eta0r_i_rv_slot = var_eta0r_i_rv;
        *var_etabedge_i_slot = var_etabedge_i;
        *var_etabedge_i_rv_slot = var_etabedge_i_rv;
        *var_guard21_slot = var_guard21;
        *var_guard21_rv_slot = var_guard21_rv;
        *var_k01_i_slot = var_k01_i;
        *var_k01_i_rv_slot = var_k01_i_rv;
        *var_k0_i_slot = var_k0_i;
        *var_k0_i_rv_slot = var_k0_i_rv;
        *var_k2edge_i_slot = var_k2edge_i;
        *var_k2edge_i_dn0_slot = var_k2edge_i_dn0;
        *var_k2edge_i_dn10_slot = var_k2edge_i_dn10;
        *var_k2edge_i_dn11_slot = var_k2edge_i_dn11;
        *var_k2edge_i_dn12_slot = var_k2edge_i_dn12;
        *var_k2edge_i_dn13_slot = var_k2edge_i_dn13;
        *var_k2edge_i_dn14_slot = var_k2edge_i_dn14;
        *var_k2edge_i_dn2_slot = var_k2edge_i_dn2;
        *var_k2edge_i_dn3_slot = var_k2edge_i_dn3;
        *var_k2edge_i_dn4_slot = var_k2edge_i_dn4;
        *var_k2edge_i_dn5_slot = var_k2edge_i_dn5;
        *var_k2edge_i_dn6_slot = var_k2edge_i_dn6;
        *var_k2edge_i_dn7_slot = var_k2edge_i_dn7;
        *var_k2edge_i_dn8_slot = var_k2edge_i_dn8;
        *var_k2edge_i_dn9_slot = var_k2edge_i_dn9;
        *var_k2edge_i_rv_slot = var_k2edge_i_rv;
        *var_k2edgewe_i_slot = var_k2edgewe_i;
        *var_k2edgewe_i_rv_slot = var_k2edgewe_i_rv;
        *var_kt1edge_i_slot = var_kt1edge_i;
        *var_kt1edge_i_rv_slot = var_kt1edge_i_rv;
        *var_kt1expedge_i_slot = var_kt1expedge_i;
        *var_kt1expedge_i_rv_slot = var_kt1expedge_i_rv;
        *var_kt1ledge_i_slot = var_kt1ledge_i;
        *var_kt1ledge_i_rv_slot = var_kt1ledge_i_rv;
        *var_kt2edge_i_slot = var_kt2edge_i;
        *var_kt2edge_i_rv_slot = var_kt2edge_i_rv;
        *var_kvth0edge_i_slot = var_kvth0edge_i;
        *var_kvth0edge_i_rv_slot = var_kvth0edge_i_rv;
        *var_kvth0edgewe_i_slot = var_kvth0edgewe_i;
        *var_kvth0edgewe_i_rv_slot = var_kvth0edgewe_i_rv;
        *var_m01_i_slot = var_m01_i;
        *var_m01_i_rv_slot = var_m01_i_rv;
        *var_m0_i_slot = var_m0_i;
        *var_m0_i_rv_slot = var_m0_i_rv;
        *var_mpower_i_slot = var_mpower_i;
        *var_mpower_i_rv_slot = var_mpower_i_rv;
        *var_ndepedge_i_slot = var_ndepedge_i;
        *var_ndepedge_i_rv_slot = var_ndepedge_i_rv;
        *var_nfactoredge_i_slot = var_nfactoredge_i;
        *var_nfactoredge_i_rv_slot = var_nfactoredge_i_rv;
        *var_noia3_i_slot = var_noia3_i;
        *var_noia3_i_rv_slot = var_noia3_i_rv;
        *var_qsref_i_slot = var_qsref_i;
        *var_qsref_i_rv_slot = var_qsref_i_rv;
        *var_steta0edge_i_slot = var_steta0edge_i;
        *var_steta0edge_i_rv_slot = var_steta0edge_i_rv;
        *var_stk2edge_i_slot = var_stk2edge_i;
        *var_stk2edge_i_rv_slot = var_stk2edge_i_rv;
        *var_teta0edge_i_slot = var_teta0edge_i;
        *var_teta0edge_i_rv_slot = var_teta0edge_i_rv;
        *var_tnfactoredge_i_slot = var_tnfactoredge_i;
        *var_tnfactoredge_i_rv_slot = var_tnfactoredge_i_rv;
        *var_u0r_i_slot = var_u0r_i;
        *var_u0r_i_rv_slot = var_u0r_i_rv;
        *var_uar_i_slot = var_uar_i;
        *var_uar_i_dn0_slot = var_uar_i_dn0;
        *var_uar_i_dn10_slot = var_uar_i_dn10;
        *var_uar_i_dn11_slot = var_uar_i_dn11;
        *var_uar_i_dn12_slot = var_uar_i_dn12;
        *var_uar_i_dn13_slot = var_uar_i_dn13;
        *var_uar_i_dn14_slot = var_uar_i_dn14;
        *var_uar_i_dn2_slot = var_uar_i_dn2;
        *var_uar_i_dn3_slot = var_uar_i_dn3;
        *var_uar_i_dn4_slot = var_uar_i_dn4;
        *var_uar_i_dn5_slot = var_uar_i_dn5;
        *var_uar_i_dn6_slot = var_uar_i_dn6;
        *var_uar_i_dn7_slot = var_uar_i_dn7;
        *var_uar_i_dn8_slot = var_uar_i_dn8;
        *var_uar_i_dn9_slot = var_uar_i_dn9;
        *var_uar_i_rv_slot = var_uar_i_rv;
        *var_ucsr_i_slot = var_ucsr_i;
        *var_ucsr_i_rv_slot = var_ucsr_i_rv;
        *var_udr_i_slot = var_udr_i;
        *var_udr_i_dn0_slot = var_udr_i_dn0;
        *var_udr_i_dn10_slot = var_udr_i_dn10;
        *var_udr_i_dn11_slot = var_udr_i_dn11;
        *var_udr_i_dn12_slot = var_udr_i_dn12;
        *var_udr_i_dn13_slot = var_udr_i_dn13;
        *var_udr_i_dn14_slot = var_udr_i_dn14;
        *var_udr_i_dn2_slot = var_udr_i_dn2;
        *var_udr_i_dn3_slot = var_udr_i_dn3;
        *var_udr_i_dn4_slot = var_udr_i_dn4;
        *var_udr_i_dn5_slot = var_udr_i_dn5;
        *var_udr_i_dn6_slot = var_udr_i_dn6;
        *var_udr_i_dn7_slot = var_udr_i_dn7;
        *var_udr_i_dn8_slot = var_udr_i_dn8;
        *var_udr_i_dn9_slot = var_udr_i_dn9;
        *var_udr_i_rv_slot = var_udr_i_rv;
    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        var_bin_l: f64,
        var_bin_w: f64,
        var_bin_wl: f64,
        var_guard21: f64,
        var_inv_l: f64,
        var_inv_llong: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_inv_wwide: f64,
        var_alpha0r_i_slot: &mut f64,
        var_alpha0r_i_dn0_slot: &mut f64,
        var_alpha0r_i_dn10_slot: &mut f64,
        var_alpha0r_i_dn11_slot: &mut f64,
        var_alpha0r_i_dn12_slot: &mut f64,
        var_alpha0r_i_dn13_slot: &mut f64,
        var_alpha0r_i_dn14_slot: &mut f64,
        var_alpha0r_i_dn2_slot: &mut f64,
        var_alpha0r_i_dn3_slot: &mut f64,
        var_alpha0r_i_dn4_slot: &mut f64,
        var_alpha0r_i_dn5_slot: &mut f64,
        var_alpha0r_i_dn6_slot: &mut f64,
        var_alpha0r_i_dn7_slot: &mut f64,
        var_alpha0r_i_dn8_slot: &mut f64,
        var_alpha0r_i_dn9_slot: &mut f64,
        var_alpha0r_i_rv_slot: &mut f64,
        var_beta0r_i_slot: &mut f64,
        var_beta0r_i_rv_slot: &mut f64,
        var_cdscb_i_slot: &mut f64,
        var_cdscb_i_rv_slot: &mut f64,
        var_cdscd_i_slot: &mut f64,
        var_cdscd_i_dn0_slot: &mut f64,
        var_cdscd_i_dn10_slot: &mut f64,
        var_cdscd_i_dn11_slot: &mut f64,
        var_cdscd_i_dn12_slot: &mut f64,
        var_cdscd_i_dn13_slot: &mut f64,
        var_cdscd_i_dn14_slot: &mut f64,
        var_cdscd_i_dn2_slot: &mut f64,
        var_cdscd_i_dn3_slot: &mut f64,
        var_cdscd_i_dn4_slot: &mut f64,
        var_cdscd_i_dn5_slot: &mut f64,
        var_cdscd_i_dn6_slot: &mut f64,
        var_cdscd_i_dn7_slot: &mut f64,
        var_cdscd_i_dn8_slot: &mut f64,
        var_cdscd_i_dn9_slot: &mut f64,
        var_cdscd_i_rv_slot: &mut f64,
        var_cdscdr_i_slot: &mut f64,
        var_cdscdr_i_dn0_slot: &mut f64,
        var_cdscdr_i_dn10_slot: &mut f64,
        var_cdscdr_i_dn11_slot: &mut f64,
        var_cdscdr_i_dn12_slot: &mut f64,
        var_cdscdr_i_dn13_slot: &mut f64,
        var_cdscdr_i_dn14_slot: &mut f64,
        var_cdscdr_i_dn2_slot: &mut f64,
        var_cdscdr_i_dn3_slot: &mut f64,
        var_cdscdr_i_dn4_slot: &mut f64,
        var_cdscdr_i_dn5_slot: &mut f64,
        var_cdscdr_i_dn6_slot: &mut f64,
        var_cdscdr_i_dn7_slot: &mut f64,
        var_cdscdr_i_dn8_slot: &mut f64,
        var_cdscdr_i_dn9_slot: &mut f64,
        var_cdscdr_i_rv_slot: &mut f64,
        var_guard22_slot: &mut f64,
        var_guard22_rv_slot: &mut f64,
        var_guard23_slot: &mut f64,
        var_guard23_rv_slot: &mut f64,
        var_guard24_slot: &mut f64,
        var_guard24_rv_slot: &mut f64,
        var_guard25_slot: &mut f64,
        var_guard25_rv_slot: &mut f64,
        var_guard26_slot: &mut f64,
        var_guard26_rv_slot: &mut f64,
        var_ndep_i_slot: &mut f64,
        var_ndep_i_dn0_slot: &mut f64,
        var_ndep_i_dn10_slot: &mut f64,
        var_ndep_i_dn11_slot: &mut f64,
        var_ndep_i_dn12_slot: &mut f64,
        var_ndep_i_dn13_slot: &mut f64,
        var_ndep_i_dn14_slot: &mut f64,
        var_ndep_i_dn2_slot: &mut f64,
        var_ndep_i_dn3_slot: &mut f64,
        var_ndep_i_dn4_slot: &mut f64,
        var_ndep_i_dn5_slot: &mut f64,
        var_ndep_i_dn6_slot: &mut f64,
        var_ndep_i_dn7_slot: &mut f64,
        var_ndep_i_dn8_slot: &mut f64,
        var_ndep_i_dn9_slot: &mut f64,
        var_ndep_i_rv_slot: &mut f64,
        var_nfactor_i_slot: &mut f64,
        var_nfactor_i_dn0_slot: &mut f64,
        var_nfactor_i_dn10_slot: &mut f64,
        var_nfactor_i_dn11_slot: &mut f64,
        var_nfactor_i_dn12_slot: &mut f64,
        var_nfactor_i_dn13_slot: &mut f64,
        var_nfactor_i_dn14_slot: &mut f64,
        var_nfactor_i_dn2_slot: &mut f64,
        var_nfactor_i_dn3_slot: &mut f64,
        var_nfactor_i_dn4_slot: &mut f64,
        var_nfactor_i_dn5_slot: &mut f64,
        var_nfactor_i_dn6_slot: &mut f64,
        var_nfactor_i_dn7_slot: &mut f64,
        var_nfactor_i_dn8_slot: &mut f64,
        var_nfactor_i_dn9_slot: &mut f64,
        var_nfactor_i_rv_slot: &mut f64,
        var_pclmr_i_slot: &mut f64,
        var_pclmr_i_dn0_slot: &mut f64,
        var_pclmr_i_dn10_slot: &mut f64,
        var_pclmr_i_dn11_slot: &mut f64,
        var_pclmr_i_dn12_slot: &mut f64,
        var_pclmr_i_dn13_slot: &mut f64,
        var_pclmr_i_dn14_slot: &mut f64,
        var_pclmr_i_dn2_slot: &mut f64,
        var_pclmr_i_dn3_slot: &mut f64,
        var_pclmr_i_dn4_slot: &mut f64,
        var_pclmr_i_dn5_slot: &mut f64,
        var_pclmr_i_dn6_slot: &mut f64,
        var_pclmr_i_dn7_slot: &mut f64,
        var_pclmr_i_dn8_slot: &mut f64,
        var_pclmr_i_dn9_slot: &mut f64,
        var_pclmr_i_rv_slot: &mut f64,
        var_pdiblcr_i_slot: &mut f64,
        var_pdiblcr_i_dn0_slot: &mut f64,
        var_pdiblcr_i_dn10_slot: &mut f64,
        var_pdiblcr_i_dn11_slot: &mut f64,
        var_pdiblcr_i_dn12_slot: &mut f64,
        var_pdiblcr_i_dn13_slot: &mut f64,
        var_pdiblcr_i_dn14_slot: &mut f64,
        var_pdiblcr_i_dn2_slot: &mut f64,
        var_pdiblcr_i_dn3_slot: &mut f64,
        var_pdiblcr_i_dn4_slot: &mut f64,
        var_pdiblcr_i_dn5_slot: &mut f64,
        var_pdiblcr_i_dn6_slot: &mut f64,
        var_pdiblcr_i_dn7_slot: &mut f64,
        var_pdiblcr_i_dn8_slot: &mut f64,
        var_pdiblcr_i_dn9_slot: &mut f64,
        var_pdiblcr_i_rv_slot: &mut f64,
        var_psatr_i_slot: &mut f64,
        var_psatr_i_rv_slot: &mut f64,
        var_ptwgr_i_slot: &mut f64,
        var_ptwgr_i_dn0_slot: &mut f64,
        var_ptwgr_i_dn10_slot: &mut f64,
        var_ptwgr_i_dn11_slot: &mut f64,
        var_ptwgr_i_dn12_slot: &mut f64,
        var_ptwgr_i_dn13_slot: &mut f64,
        var_ptwgr_i_dn14_slot: &mut f64,
        var_ptwgr_i_dn2_slot: &mut f64,
        var_ptwgr_i_dn3_slot: &mut f64,
        var_ptwgr_i_dn4_slot: &mut f64,
        var_ptwgr_i_dn5_slot: &mut f64,
        var_ptwgr_i_dn6_slot: &mut f64,
        var_ptwgr_i_dn7_slot: &mut f64,
        var_ptwgr_i_dn8_slot: &mut f64,
        var_ptwgr_i_dn9_slot: &mut f64,
        var_ptwgr_i_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_u0_i_slot: &mut f64,
        var_u0_i_rv_slot: &mut f64,
        var_u0r_i_slot: &mut f64,
        var_u0r_i_rv_slot: &mut f64,
        var_ucr_i_slot: &mut f64,
        var_ucr_i_dn0_slot: &mut f64,
        var_ucr_i_dn10_slot: &mut f64,
        var_ucr_i_dn11_slot: &mut f64,
        var_ucr_i_dn12_slot: &mut f64,
        var_ucr_i_dn13_slot: &mut f64,
        var_ucr_i_dn14_slot: &mut f64,
        var_ucr_i_dn2_slot: &mut f64,
        var_ucr_i_dn3_slot: &mut f64,
        var_ucr_i_dn4_slot: &mut f64,
        var_ucr_i_dn5_slot: &mut f64,
        var_ucr_i_dn6_slot: &mut f64,
        var_ucr_i_dn7_slot: &mut f64,
        var_ucr_i_dn8_slot: &mut f64,
        var_ucr_i_dn9_slot: &mut f64,
        var_ucr_i_rv_slot: &mut f64,
        var_vsatr_i_slot: &mut f64,
        var_vsatr_i_dn0_slot: &mut f64,
        var_vsatr_i_dn10_slot: &mut f64,
        var_vsatr_i_dn11_slot: &mut f64,
        var_vsatr_i_dn12_slot: &mut f64,
        var_vsatr_i_dn13_slot: &mut f64,
        var_vsatr_i_dn14_slot: &mut f64,
        var_vsatr_i_dn2_slot: &mut f64,
        var_vsatr_i_dn3_slot: &mut f64,
        var_vsatr_i_dn4_slot: &mut f64,
        var_vsatr_i_dn5_slot: &mut f64,
        var_vsatr_i_dn6_slot: &mut f64,
        var_vsatr_i_dn7_slot: &mut f64,
        var_vsatr_i_dn8_slot: &mut f64,
        var_vsatr_i_dn9_slot: &mut f64,
        var_vsatr_i_rv_slot: &mut f64,
    ) {
        let mut var_alpha0r_i: f64 = *var_alpha0r_i_slot;
        let mut var_alpha0r_i_dn0: f64 = *var_alpha0r_i_dn0_slot;
        let mut var_alpha0r_i_dn10: f64 = *var_alpha0r_i_dn10_slot;
        let mut var_alpha0r_i_dn11: f64 = *var_alpha0r_i_dn11_slot;
        let mut var_alpha0r_i_dn12: f64 = *var_alpha0r_i_dn12_slot;
        let mut var_alpha0r_i_dn13: f64 = *var_alpha0r_i_dn13_slot;
        let mut var_alpha0r_i_dn14: f64 = *var_alpha0r_i_dn14_slot;
        let mut var_alpha0r_i_dn2: f64 = *var_alpha0r_i_dn2_slot;
        let mut var_alpha0r_i_dn3: f64 = *var_alpha0r_i_dn3_slot;
        let mut var_alpha0r_i_dn4: f64 = *var_alpha0r_i_dn4_slot;
        let mut var_alpha0r_i_dn5: f64 = *var_alpha0r_i_dn5_slot;
        let mut var_alpha0r_i_dn6: f64 = *var_alpha0r_i_dn6_slot;
        let mut var_alpha0r_i_dn7: f64 = *var_alpha0r_i_dn7_slot;
        let mut var_alpha0r_i_dn8: f64 = *var_alpha0r_i_dn8_slot;
        let mut var_alpha0r_i_dn9: f64 = *var_alpha0r_i_dn9_slot;
        let mut var_alpha0r_i_rv: f64 = *var_alpha0r_i_rv_slot;
        let mut var_beta0r_i: f64 = *var_beta0r_i_slot;
        let mut var_beta0r_i_rv: f64 = *var_beta0r_i_rv_slot;
        let mut var_cdscb_i: f64 = *var_cdscb_i_slot;
        let mut var_cdscb_i_rv: f64 = *var_cdscb_i_rv_slot;
        let mut var_cdscd_i: f64 = *var_cdscd_i_slot;
        let mut var_cdscd_i_dn0: f64 = *var_cdscd_i_dn0_slot;
        let mut var_cdscd_i_dn10: f64 = *var_cdscd_i_dn10_slot;
        let mut var_cdscd_i_dn11: f64 = *var_cdscd_i_dn11_slot;
        let mut var_cdscd_i_dn12: f64 = *var_cdscd_i_dn12_slot;
        let mut var_cdscd_i_dn13: f64 = *var_cdscd_i_dn13_slot;
        let mut var_cdscd_i_dn14: f64 = *var_cdscd_i_dn14_slot;
        let mut var_cdscd_i_dn2: f64 = *var_cdscd_i_dn2_slot;
        let mut var_cdscd_i_dn3: f64 = *var_cdscd_i_dn3_slot;
        let mut var_cdscd_i_dn4: f64 = *var_cdscd_i_dn4_slot;
        let mut var_cdscd_i_dn5: f64 = *var_cdscd_i_dn5_slot;
        let mut var_cdscd_i_dn6: f64 = *var_cdscd_i_dn6_slot;
        let mut var_cdscd_i_dn7: f64 = *var_cdscd_i_dn7_slot;
        let mut var_cdscd_i_dn8: f64 = *var_cdscd_i_dn8_slot;
        let mut var_cdscd_i_dn9: f64 = *var_cdscd_i_dn9_slot;
        let mut var_cdscd_i_rv: f64 = *var_cdscd_i_rv_slot;
        let mut var_cdscdr_i: f64 = *var_cdscdr_i_slot;
        let mut var_cdscdr_i_dn0: f64 = *var_cdscdr_i_dn0_slot;
        let mut var_cdscdr_i_dn10: f64 = *var_cdscdr_i_dn10_slot;
        let mut var_cdscdr_i_dn11: f64 = *var_cdscdr_i_dn11_slot;
        let mut var_cdscdr_i_dn12: f64 = *var_cdscdr_i_dn12_slot;
        let mut var_cdscdr_i_dn13: f64 = *var_cdscdr_i_dn13_slot;
        let mut var_cdscdr_i_dn14: f64 = *var_cdscdr_i_dn14_slot;
        let mut var_cdscdr_i_dn2: f64 = *var_cdscdr_i_dn2_slot;
        let mut var_cdscdr_i_dn3: f64 = *var_cdscdr_i_dn3_slot;
        let mut var_cdscdr_i_dn4: f64 = *var_cdscdr_i_dn4_slot;
        let mut var_cdscdr_i_dn5: f64 = *var_cdscdr_i_dn5_slot;
        let mut var_cdscdr_i_dn6: f64 = *var_cdscdr_i_dn6_slot;
        let mut var_cdscdr_i_dn7: f64 = *var_cdscdr_i_dn7_slot;
        let mut var_cdscdr_i_dn8: f64 = *var_cdscdr_i_dn8_slot;
        let mut var_cdscdr_i_dn9: f64 = *var_cdscdr_i_dn9_slot;
        let mut var_cdscdr_i_rv: f64 = *var_cdscdr_i_rv_slot;
        let mut var_guard22: f64 = *var_guard22_slot;
        let mut var_guard22_rv: f64 = *var_guard22_rv_slot;
        let mut var_guard23: f64 = *var_guard23_slot;
        let mut var_guard23_rv: f64 = *var_guard23_rv_slot;
        let mut var_guard24: f64 = *var_guard24_slot;
        let mut var_guard24_rv: f64 = *var_guard24_rv_slot;
        let mut var_guard25: f64 = *var_guard25_slot;
        let mut var_guard25_rv: f64 = *var_guard25_rv_slot;
        let mut var_guard26: f64 = *var_guard26_slot;
        let mut var_guard26_rv: f64 = *var_guard26_rv_slot;
        let mut var_ndep_i: f64 = *var_ndep_i_slot;
        let mut var_ndep_i_dn0: f64 = *var_ndep_i_dn0_slot;
        let mut var_ndep_i_dn10: f64 = *var_ndep_i_dn10_slot;
        let mut var_ndep_i_dn11: f64 = *var_ndep_i_dn11_slot;
        let mut var_ndep_i_dn12: f64 = *var_ndep_i_dn12_slot;
        let mut var_ndep_i_dn13: f64 = *var_ndep_i_dn13_slot;
        let mut var_ndep_i_dn14: f64 = *var_ndep_i_dn14_slot;
        let mut var_ndep_i_dn2: f64 = *var_ndep_i_dn2_slot;
        let mut var_ndep_i_dn3: f64 = *var_ndep_i_dn3_slot;
        let mut var_ndep_i_dn4: f64 = *var_ndep_i_dn4_slot;
        let mut var_ndep_i_dn5: f64 = *var_ndep_i_dn5_slot;
        let mut var_ndep_i_dn6: f64 = *var_ndep_i_dn6_slot;
        let mut var_ndep_i_dn7: f64 = *var_ndep_i_dn7_slot;
        let mut var_ndep_i_dn8: f64 = *var_ndep_i_dn8_slot;
        let mut var_ndep_i_dn9: f64 = *var_ndep_i_dn9_slot;
        let mut var_ndep_i_rv: f64 = *var_ndep_i_rv_slot;
        let mut var_nfactor_i: f64 = *var_nfactor_i_slot;
        let mut var_nfactor_i_dn0: f64 = *var_nfactor_i_dn0_slot;
        let mut var_nfactor_i_dn10: f64 = *var_nfactor_i_dn10_slot;
        let mut var_nfactor_i_dn11: f64 = *var_nfactor_i_dn11_slot;
        let mut var_nfactor_i_dn12: f64 = *var_nfactor_i_dn12_slot;
        let mut var_nfactor_i_dn13: f64 = *var_nfactor_i_dn13_slot;
        let mut var_nfactor_i_dn14: f64 = *var_nfactor_i_dn14_slot;
        let mut var_nfactor_i_dn2: f64 = *var_nfactor_i_dn2_slot;
        let mut var_nfactor_i_dn3: f64 = *var_nfactor_i_dn3_slot;
        let mut var_nfactor_i_dn4: f64 = *var_nfactor_i_dn4_slot;
        let mut var_nfactor_i_dn5: f64 = *var_nfactor_i_dn5_slot;
        let mut var_nfactor_i_dn6: f64 = *var_nfactor_i_dn6_slot;
        let mut var_nfactor_i_dn7: f64 = *var_nfactor_i_dn7_slot;
        let mut var_nfactor_i_dn8: f64 = *var_nfactor_i_dn8_slot;
        let mut var_nfactor_i_dn9: f64 = *var_nfactor_i_dn9_slot;
        let mut var_nfactor_i_rv: f64 = *var_nfactor_i_rv_slot;
        let mut var_pclmr_i: f64 = *var_pclmr_i_slot;
        let mut var_pclmr_i_dn0: f64 = *var_pclmr_i_dn0_slot;
        let mut var_pclmr_i_dn10: f64 = *var_pclmr_i_dn10_slot;
        let mut var_pclmr_i_dn11: f64 = *var_pclmr_i_dn11_slot;
        let mut var_pclmr_i_dn12: f64 = *var_pclmr_i_dn12_slot;
        let mut var_pclmr_i_dn13: f64 = *var_pclmr_i_dn13_slot;
        let mut var_pclmr_i_dn14: f64 = *var_pclmr_i_dn14_slot;
        let mut var_pclmr_i_dn2: f64 = *var_pclmr_i_dn2_slot;
        let mut var_pclmr_i_dn3: f64 = *var_pclmr_i_dn3_slot;
        let mut var_pclmr_i_dn4: f64 = *var_pclmr_i_dn4_slot;
        let mut var_pclmr_i_dn5: f64 = *var_pclmr_i_dn5_slot;
        let mut var_pclmr_i_dn6: f64 = *var_pclmr_i_dn6_slot;
        let mut var_pclmr_i_dn7: f64 = *var_pclmr_i_dn7_slot;
        let mut var_pclmr_i_dn8: f64 = *var_pclmr_i_dn8_slot;
        let mut var_pclmr_i_dn9: f64 = *var_pclmr_i_dn9_slot;
        let mut var_pclmr_i_rv: f64 = *var_pclmr_i_rv_slot;
        let mut var_pdiblcr_i: f64 = *var_pdiblcr_i_slot;
        let mut var_pdiblcr_i_dn0: f64 = *var_pdiblcr_i_dn0_slot;
        let mut var_pdiblcr_i_dn10: f64 = *var_pdiblcr_i_dn10_slot;
        let mut var_pdiblcr_i_dn11: f64 = *var_pdiblcr_i_dn11_slot;
        let mut var_pdiblcr_i_dn12: f64 = *var_pdiblcr_i_dn12_slot;
        let mut var_pdiblcr_i_dn13: f64 = *var_pdiblcr_i_dn13_slot;
        let mut var_pdiblcr_i_dn14: f64 = *var_pdiblcr_i_dn14_slot;
        let mut var_pdiblcr_i_dn2: f64 = *var_pdiblcr_i_dn2_slot;
        let mut var_pdiblcr_i_dn3: f64 = *var_pdiblcr_i_dn3_slot;
        let mut var_pdiblcr_i_dn4: f64 = *var_pdiblcr_i_dn4_slot;
        let mut var_pdiblcr_i_dn5: f64 = *var_pdiblcr_i_dn5_slot;
        let mut var_pdiblcr_i_dn6: f64 = *var_pdiblcr_i_dn6_slot;
        let mut var_pdiblcr_i_dn7: f64 = *var_pdiblcr_i_dn7_slot;
        let mut var_pdiblcr_i_dn8: f64 = *var_pdiblcr_i_dn8_slot;
        let mut var_pdiblcr_i_dn9: f64 = *var_pdiblcr_i_dn9_slot;
        let mut var_pdiblcr_i_rv: f64 = *var_pdiblcr_i_rv_slot;
        let mut var_psatr_i: f64 = *var_psatr_i_slot;
        let mut var_psatr_i_rv: f64 = *var_psatr_i_rv_slot;
        let mut var_ptwgr_i: f64 = *var_ptwgr_i_slot;
        let mut var_ptwgr_i_dn0: f64 = *var_ptwgr_i_dn0_slot;
        let mut var_ptwgr_i_dn10: f64 = *var_ptwgr_i_dn10_slot;
        let mut var_ptwgr_i_dn11: f64 = *var_ptwgr_i_dn11_slot;
        let mut var_ptwgr_i_dn12: f64 = *var_ptwgr_i_dn12_slot;
        let mut var_ptwgr_i_dn13: f64 = *var_ptwgr_i_dn13_slot;
        let mut var_ptwgr_i_dn14: f64 = *var_ptwgr_i_dn14_slot;
        let mut var_ptwgr_i_dn2: f64 = *var_ptwgr_i_dn2_slot;
        let mut var_ptwgr_i_dn3: f64 = *var_ptwgr_i_dn3_slot;
        let mut var_ptwgr_i_dn4: f64 = *var_ptwgr_i_dn4_slot;
        let mut var_ptwgr_i_dn5: f64 = *var_ptwgr_i_dn5_slot;
        let mut var_ptwgr_i_dn6: f64 = *var_ptwgr_i_dn6_slot;
        let mut var_ptwgr_i_dn7: f64 = *var_ptwgr_i_dn7_slot;
        let mut var_ptwgr_i_dn8: f64 = *var_ptwgr_i_dn8_slot;
        let mut var_ptwgr_i_dn9: f64 = *var_ptwgr_i_dn9_slot;
        let mut var_ptwgr_i_rv: f64 = *var_ptwgr_i_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_u0_i: f64 = *var_u0_i_slot;
        let mut var_u0_i_rv: f64 = *var_u0_i_rv_slot;
        let mut var_u0r_i: f64 = *var_u0r_i_slot;
        let mut var_u0r_i_rv: f64 = *var_u0r_i_rv_slot;
        let mut var_ucr_i: f64 = *var_ucr_i_slot;
        let mut var_ucr_i_dn0: f64 = *var_ucr_i_dn0_slot;
        let mut var_ucr_i_dn10: f64 = *var_ucr_i_dn10_slot;
        let mut var_ucr_i_dn11: f64 = *var_ucr_i_dn11_slot;
        let mut var_ucr_i_dn12: f64 = *var_ucr_i_dn12_slot;
        let mut var_ucr_i_dn13: f64 = *var_ucr_i_dn13_slot;
        let mut var_ucr_i_dn14: f64 = *var_ucr_i_dn14_slot;
        let mut var_ucr_i_dn2: f64 = *var_ucr_i_dn2_slot;
        let mut var_ucr_i_dn3: f64 = *var_ucr_i_dn3_slot;
        let mut var_ucr_i_dn4: f64 = *var_ucr_i_dn4_slot;
        let mut var_ucr_i_dn5: f64 = *var_ucr_i_dn5_slot;
        let mut var_ucr_i_dn6: f64 = *var_ucr_i_dn6_slot;
        let mut var_ucr_i_dn7: f64 = *var_ucr_i_dn7_slot;
        let mut var_ucr_i_dn8: f64 = *var_ucr_i_dn8_slot;
        let mut var_ucr_i_dn9: f64 = *var_ucr_i_dn9_slot;
        let mut var_ucr_i_rv: f64 = *var_ucr_i_rv_slot;
        let mut var_vsatr_i: f64 = *var_vsatr_i_slot;
        let mut var_vsatr_i_dn0: f64 = *var_vsatr_i_dn0_slot;
        let mut var_vsatr_i_dn10: f64 = *var_vsatr_i_dn10_slot;
        let mut var_vsatr_i_dn11: f64 = *var_vsatr_i_dn11_slot;
        let mut var_vsatr_i_dn12: f64 = *var_vsatr_i_dn12_slot;
        let mut var_vsatr_i_dn13: f64 = *var_vsatr_i_dn13_slot;
        let mut var_vsatr_i_dn14: f64 = *var_vsatr_i_dn14_slot;
        let mut var_vsatr_i_dn2: f64 = *var_vsatr_i_dn2_slot;
        let mut var_vsatr_i_dn3: f64 = *var_vsatr_i_dn3_slot;
        let mut var_vsatr_i_dn4: f64 = *var_vsatr_i_dn4_slot;
        let mut var_vsatr_i_dn5: f64 = *var_vsatr_i_dn5_slot;
        let mut var_vsatr_i_dn6: f64 = *var_vsatr_i_dn6_slot;
        let mut var_vsatr_i_dn7: f64 = *var_vsatr_i_dn7_slot;
        let mut var_vsatr_i_dn8: f64 = *var_vsatr_i_dn8_slot;
        let mut var_vsatr_i_dn9: f64 = *var_vsatr_i_dn9_slot;
        let mut var_vsatr_i_rv: f64 = *var_vsatr_i_rv_slot;

        let (assign3190_e4449, assign3190_e4449_d_n0, assign3190_e4449_d_n2, assign3190_e4449_d_n3, assign3190_e4449_d_n4, assign3190_e4449_d_n5, assign3190_e4449_d_n6, assign3190_e4449_d_n7, assign3190_e4449_d_n8, assign3190_e4449_d_n9, assign3190_e4449_d_n10, assign3190_e4449_d_n11, assign3190_e4449_d_n12, assign3190_e4449_d_n13, assign3190_e4449_d_n14,) = {
    if (var_guard21 != 0.0) {
        let assign3190_e4438: f64 = (var_bin_l * p.p337);
        let assign3190_e4439: f64 = (p.p336 + assign3190_e4438);
        let assign3190_e4442: f64 = (var_bin_w * p.p338);
        let assign3190_e4443: f64 = (assign3190_e4439 + assign3190_e4442);
        let assign3190_e4446: f64 = (var_bin_wl * p.p339);
        let assign3190_e4447: f64 = (assign3190_e4443 + assign3190_e4446);
        (assign3190_e4447, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ucr_i, var_ucr_i_dn0, var_ucr_i_dn2, var_ucr_i_dn3, var_ucr_i_dn4, var_ucr_i_dn5, var_ucr_i_dn6, var_ucr_i_dn7, var_ucr_i_dn8, var_ucr_i_dn9, var_ucr_i_dn10, var_ucr_i_dn11, var_ucr_i_dn12, var_ucr_i_dn13, var_ucr_i_dn14,)
    }
};
        var_ucr_i = assign3190_e4449;
        var_ucr_i_dn0 = assign3190_e4449_d_n0;
        var_ucr_i_dn2 = assign3190_e4449_d_n2;
        var_ucr_i_dn3 = assign3190_e4449_d_n3;
        var_ucr_i_dn4 = assign3190_e4449_d_n4;
        var_ucr_i_dn5 = assign3190_e4449_d_n5;
        var_ucr_i_dn6 = assign3190_e4449_d_n6;
        var_ucr_i_dn7 = assign3190_e4449_d_n7;
        var_ucr_i_dn8 = assign3190_e4449_d_n8;
        var_ucr_i_dn9 = assign3190_e4449_d_n9;
        var_ucr_i_dn10 = assign3190_e4449_d_n10;
        var_ucr_i_dn11 = assign3190_e4449_d_n11;
        var_ucr_i_dn12 = assign3190_e4449_d_n12;
        var_ucr_i_dn13 = assign3190_e4449_d_n13;
        var_ucr_i_dn14 = assign3190_e4449_d_n14;
        var_ucr_i_rv = 0.0;

        let (assign3200_e4465, assign3200_e4465_d_n0, assign3200_e4465_d_n2, assign3200_e4465_d_n3, assign3200_e4465_d_n4, assign3200_e4465_d_n5, assign3200_e4465_d_n6, assign3200_e4465_d_n7, assign3200_e4465_d_n8, assign3200_e4465_d_n9, assign3200_e4465_d_n10, assign3200_e4465_d_n11, assign3200_e4465_d_n12, assign3200_e4465_d_n13, assign3200_e4465_d_n14,) = {
    if (var_guard21 != 0.0) {
        let assign3200_e4454: f64 = (var_bin_l * p.p347);
        let assign3200_e4455: f64 = (p.p346 + assign3200_e4454);
        let assign3200_e4458: f64 = (var_bin_w * p.p348);
        let assign3200_e4459: f64 = (assign3200_e4455 + assign3200_e4458);
        let assign3200_e4462: f64 = (var_bin_wl * p.p349);
        let assign3200_e4463: f64 = (assign3200_e4459 + assign3200_e4462);
        (assign3200_e4463, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pclmr_i, var_pclmr_i_dn0, var_pclmr_i_dn2, var_pclmr_i_dn3, var_pclmr_i_dn4, var_pclmr_i_dn5, var_pclmr_i_dn6, var_pclmr_i_dn7, var_pclmr_i_dn8, var_pclmr_i_dn9, var_pclmr_i_dn10, var_pclmr_i_dn11, var_pclmr_i_dn12, var_pclmr_i_dn13, var_pclmr_i_dn14,)
    }
};
        var_pclmr_i = assign3200_e4465;
        var_pclmr_i_dn0 = assign3200_e4465_d_n0;
        var_pclmr_i_dn2 = assign3200_e4465_d_n2;
        var_pclmr_i_dn3 = assign3200_e4465_d_n3;
        var_pclmr_i_dn4 = assign3200_e4465_d_n4;
        var_pclmr_i_dn5 = assign3200_e4465_d_n5;
        var_pclmr_i_dn6 = assign3200_e4465_d_n6;
        var_pclmr_i_dn7 = assign3200_e4465_d_n7;
        var_pclmr_i_dn8 = assign3200_e4465_d_n8;
        var_pclmr_i_dn9 = assign3200_e4465_d_n9;
        var_pclmr_i_dn10 = assign3200_e4465_d_n10;
        var_pclmr_i_dn11 = assign3200_e4465_d_n11;
        var_pclmr_i_dn12 = assign3200_e4465_d_n12;
        var_pclmr_i_dn13 = assign3200_e4465_d_n13;
        var_pclmr_i_dn14 = assign3200_e4465_d_n14;
        var_pclmr_i_rv = 0.0;

        let (assign3210_e4481, assign3210_e4481_d_n0, assign3210_e4481_d_n2, assign3210_e4481_d_n3, assign3210_e4481_d_n4, assign3210_e4481_d_n5, assign3210_e4481_d_n6, assign3210_e4481_d_n7, assign3210_e4481_d_n8, assign3210_e4481_d_n9, assign3210_e4481_d_n10, assign3210_e4481_d_n11, assign3210_e4481_d_n12, assign3210_e4481_d_n13, assign3210_e4481_d_n14,) = {
    if (var_guard21 != 0.0) {
        let assign3210_e4470: f64 = (var_bin_l * p.p467);
        let assign3210_e4471: f64 = (p.p466 + assign3210_e4470);
        let assign3210_e4474: f64 = (var_bin_w * p.p468);
        let assign3210_e4475: f64 = (assign3210_e4471 + assign3210_e4474);
        let assign3210_e4478: f64 = (var_bin_wl * p.p469);
        let assign3210_e4479: f64 = (assign3210_e4475 + assign3210_e4478);
        (assign3210_e4479, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pdiblcr_i, var_pdiblcr_i_dn0, var_pdiblcr_i_dn2, var_pdiblcr_i_dn3, var_pdiblcr_i_dn4, var_pdiblcr_i_dn5, var_pdiblcr_i_dn6, var_pdiblcr_i_dn7, var_pdiblcr_i_dn8, var_pdiblcr_i_dn9, var_pdiblcr_i_dn10, var_pdiblcr_i_dn11, var_pdiblcr_i_dn12, var_pdiblcr_i_dn13, var_pdiblcr_i_dn14,)
    }
};
        var_pdiblcr_i = assign3210_e4481;
        var_pdiblcr_i_dn0 = assign3210_e4481_d_n0;
        var_pdiblcr_i_dn2 = assign3210_e4481_d_n2;
        var_pdiblcr_i_dn3 = assign3210_e4481_d_n3;
        var_pdiblcr_i_dn4 = assign3210_e4481_d_n4;
        var_pdiblcr_i_dn5 = assign3210_e4481_d_n5;
        var_pdiblcr_i_dn6 = assign3210_e4481_d_n6;
        var_pdiblcr_i_dn7 = assign3210_e4481_d_n7;
        var_pdiblcr_i_dn8 = assign3210_e4481_d_n8;
        var_pdiblcr_i_dn9 = assign3210_e4481_d_n9;
        var_pdiblcr_i_dn10 = assign3210_e4481_d_n10;
        var_pdiblcr_i_dn11 = assign3210_e4481_d_n11;
        var_pdiblcr_i_dn12 = assign3210_e4481_d_n12;
        var_pdiblcr_i_dn13 = assign3210_e4481_d_n13;
        var_pdiblcr_i_dn14 = assign3210_e4481_d_n14;
        var_pdiblcr_i_rv = 0.0;

        let (assign3220_e4497, assign3220_e4497_d_n0, assign3220_e4497_d_n2, assign3220_e4497_d_n3, assign3220_e4497_d_n4, assign3220_e4497_d_n5, assign3220_e4497_d_n6, assign3220_e4497_d_n7, assign3220_e4497_d_n8, assign3220_e4497_d_n9, assign3220_e4497_d_n10, assign3220_e4497_d_n11, assign3220_e4497_d_n12, assign3220_e4497_d_n13, assign3220_e4497_d_n14,) = {
    if (var_guard21 != 0.0) {
        let assign3220_e4486: f64 = (var_bin_l * p.p250);
        let assign3220_e4487: f64 = (p.p249 + assign3220_e4486);
        let assign3220_e4490: f64 = (var_bin_w * p.p251);
        let assign3220_e4491: f64 = (assign3220_e4487 + assign3220_e4490);
        let assign3220_e4494: f64 = (var_bin_wl * p.p252);
        let assign3220_e4495: f64 = (assign3220_e4491 + assign3220_e4494);
        (assign3220_e4495, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vsatr_i, var_vsatr_i_dn0, var_vsatr_i_dn2, var_vsatr_i_dn3, var_vsatr_i_dn4, var_vsatr_i_dn5, var_vsatr_i_dn6, var_vsatr_i_dn7, var_vsatr_i_dn8, var_vsatr_i_dn9, var_vsatr_i_dn10, var_vsatr_i_dn11, var_vsatr_i_dn12, var_vsatr_i_dn13, var_vsatr_i_dn14,)
    }
};
        var_vsatr_i = assign3220_e4497;
        var_vsatr_i_dn0 = assign3220_e4497_d_n0;
        var_vsatr_i_dn2 = assign3220_e4497_d_n2;
        var_vsatr_i_dn3 = assign3220_e4497_d_n3;
        var_vsatr_i_dn4 = assign3220_e4497_d_n4;
        var_vsatr_i_dn5 = assign3220_e4497_d_n5;
        var_vsatr_i_dn6 = assign3220_e4497_d_n6;
        var_vsatr_i_dn7 = assign3220_e4497_d_n7;
        var_vsatr_i_dn8 = assign3220_e4497_d_n8;
        var_vsatr_i_dn9 = assign3220_e4497_d_n9;
        var_vsatr_i_dn10 = assign3220_e4497_d_n10;
        var_vsatr_i_dn11 = assign3220_e4497_d_n11;
        var_vsatr_i_dn12 = assign3220_e4497_d_n12;
        var_vsatr_i_dn13 = assign3220_e4497_d_n13;
        var_vsatr_i_dn14 = assign3220_e4497_d_n14;
        var_vsatr_i_rv = 0.0;

        let (assign3230_e4513,) = {
    if (var_guard21 != 0.0) {
        let assign3230_e4502: f64 = (var_bin_l * p.p427);
        let assign3230_e4503: f64 = (p.p426 + assign3230_e4502);
        let assign3230_e4506: f64 = (var_bin_w * p.p428);
        let assign3230_e4507: f64 = (assign3230_e4503 + assign3230_e4506);
        let assign3230_e4510: f64 = (var_bin_wl * p.p429);
        let assign3230_e4511: f64 = (assign3230_e4507 + assign3230_e4510);
        (assign3230_e4511,)
    } else {
        (var_psatr_i,)
    }
};
        var_psatr_i = assign3230_e4513;
        var_psatr_i_rv = 0.0;

        let (assign3240_e4529, assign3240_e4529_d_n0, assign3240_e4529_d_n2, assign3240_e4529_d_n3, assign3240_e4529_d_n4, assign3240_e4529_d_n5, assign3240_e4529_d_n6, assign3240_e4529_d_n7, assign3240_e4529_d_n8, assign3240_e4529_d_n9, assign3240_e4529_d_n10, assign3240_e4529_d_n11, assign3240_e4529_d_n12, assign3240_e4529_d_n13, assign3240_e4529_d_n14,) = {
    if (var_guard21 != 0.0) {
        let assign3240_e4518: f64 = (var_bin_l * p.p441);
        let assign3240_e4519: f64 = (p.p440 + assign3240_e4518);
        let assign3240_e4522: f64 = (var_bin_w * p.p442);
        let assign3240_e4523: f64 = (assign3240_e4519 + assign3240_e4522);
        let assign3240_e4526: f64 = (var_bin_wl * p.p443);
        let assign3240_e4527: f64 = (assign3240_e4523 + assign3240_e4526);
        (assign3240_e4527, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ptwgr_i, var_ptwgr_i_dn0, var_ptwgr_i_dn2, var_ptwgr_i_dn3, var_ptwgr_i_dn4, var_ptwgr_i_dn5, var_ptwgr_i_dn6, var_ptwgr_i_dn7, var_ptwgr_i_dn8, var_ptwgr_i_dn9, var_ptwgr_i_dn10, var_ptwgr_i_dn11, var_ptwgr_i_dn12, var_ptwgr_i_dn13, var_ptwgr_i_dn14,)
    }
};
        var_ptwgr_i = assign3240_e4529;
        var_ptwgr_i_dn0 = assign3240_e4529_d_n0;
        var_ptwgr_i_dn2 = assign3240_e4529_d_n2;
        var_ptwgr_i_dn3 = assign3240_e4529_d_n3;
        var_ptwgr_i_dn4 = assign3240_e4529_d_n4;
        var_ptwgr_i_dn5 = assign3240_e4529_d_n5;
        var_ptwgr_i_dn6 = assign3240_e4529_d_n6;
        var_ptwgr_i_dn7 = assign3240_e4529_d_n7;
        var_ptwgr_i_dn8 = assign3240_e4529_d_n8;
        var_ptwgr_i_dn9 = assign3240_e4529_d_n9;
        var_ptwgr_i_dn10 = assign3240_e4529_d_n10;
        var_ptwgr_i_dn11 = assign3240_e4529_d_n11;
        var_ptwgr_i_dn12 = assign3240_e4529_d_n12;
        var_ptwgr_i_dn13 = assign3240_e4529_d_n13;
        var_ptwgr_i_dn14 = assign3240_e4529_d_n14;
        var_ptwgr_i_rv = 0.0;

        let (assign3250_e4545, assign3250_e4545_d_n0, assign3250_e4545_d_n2, assign3250_e4545_d_n3, assign3250_e4545_d_n4, assign3250_e4545_d_n5, assign3250_e4545_d_n6, assign3250_e4545_d_n7, assign3250_e4545_d_n8, assign3250_e4545_d_n9, assign3250_e4545_d_n10, assign3250_e4545_d_n11, assign3250_e4545_d_n12, assign3250_e4545_d_n13, assign3250_e4545_d_n14,) = {
    if (var_guard21 != 0.0) {
        let assign3250_e4534: f64 = (var_bin_l * p.p526);
        let assign3250_e4535: f64 = (p.p525 + assign3250_e4534);
        let assign3250_e4538: f64 = (var_bin_w * p.p527);
        let assign3250_e4539: f64 = (assign3250_e4535 + assign3250_e4538);
        let assign3250_e4542: f64 = (var_bin_wl * p.p528);
        let assign3250_e4543: f64 = (assign3250_e4539 + assign3250_e4542);
        (assign3250_e4543, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_alpha0r_i, var_alpha0r_i_dn0, var_alpha0r_i_dn2, var_alpha0r_i_dn3, var_alpha0r_i_dn4, var_alpha0r_i_dn5, var_alpha0r_i_dn6, var_alpha0r_i_dn7, var_alpha0r_i_dn8, var_alpha0r_i_dn9, var_alpha0r_i_dn10, var_alpha0r_i_dn11, var_alpha0r_i_dn12, var_alpha0r_i_dn13, var_alpha0r_i_dn14,)
    }
};
        var_alpha0r_i = assign3250_e4545;
        var_alpha0r_i_dn0 = assign3250_e4545_d_n0;
        var_alpha0r_i_dn2 = assign3250_e4545_d_n2;
        var_alpha0r_i_dn3 = assign3250_e4545_d_n3;
        var_alpha0r_i_dn4 = assign3250_e4545_d_n4;
        var_alpha0r_i_dn5 = assign3250_e4545_d_n5;
        var_alpha0r_i_dn6 = assign3250_e4545_d_n6;
        var_alpha0r_i_dn7 = assign3250_e4545_d_n7;
        var_alpha0r_i_dn8 = assign3250_e4545_d_n8;
        var_alpha0r_i_dn9 = assign3250_e4545_d_n9;
        var_alpha0r_i_dn10 = assign3250_e4545_d_n10;
        var_alpha0r_i_dn11 = assign3250_e4545_d_n11;
        var_alpha0r_i_dn12 = assign3250_e4545_d_n12;
        var_alpha0r_i_dn13 = assign3250_e4545_d_n13;
        var_alpha0r_i_dn14 = assign3250_e4545_d_n14;
        var_alpha0r_i_rv = 0.0;

        let (assign3260_e4561,) = {
    if (var_guard21 != 0.0) {
        let assign3260_e4550: f64 = (var_bin_l * p.p530);
        let assign3260_e4551: f64 = (p.p529 + assign3260_e4550);
        let assign3260_e4554: f64 = (var_bin_w * p.p531);
        let assign3260_e4555: f64 = (assign3260_e4551 + assign3260_e4554);
        let assign3260_e4558: f64 = (var_bin_wl * p.p532);
        let assign3260_e4559: f64 = (assign3260_e4555 + assign3260_e4558);
        (assign3260_e4559,)
    } else {
        (var_beta0r_i,)
    }
};
        var_beta0r_i = assign3260_e4561;
        var_beta0r_i_rv = 0.0;

        let assign3270_e4565: f64 = (var_inv_l).powf(p.p82);
        let assign3270_e4568: f64 = (var_inv_llong).powf(p.p82);
        let assign3270_e4569: f64 = (assign3270_e4565 - assign3270_e4568);
        let assign3270_e4571: f64 = (assign3270_e4569).max(0.0);
        let assign3270_e4572: f64 = (p.p81 * assign3270_e4571);
        let assign3270_e4576: f64 = (var_inv_l).powf(p.p84);
        let assign3270_e4579: f64 = (var_inv_llong).powf(p.p84);
        let assign3270_e4580: f64 = (assign3270_e4576 - assign3270_e4579);
        let assign3270_e4582: f64 = (assign3270_e4580).max(0.0);
        let assign3270_e4583: f64 = (p.p83 * assign3270_e4582);
        let assign3270_e4584: f64 = (assign3270_e4572 + assign3270_e4583);
        var_t0 = assign3270_e4584;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign3280_e4588: f64 = (var_inv_w).powf(p.p86);
        let assign3280_e4591: f64 = (var_inv_wwide).powf(p.p86);
        let assign3280_e4592: f64 = (assign3280_e4588 - assign3280_e4591);
        let assign3280_e4594: f64 = (assign3280_e4592).max(0.0);
        let assign3280_e4595: f64 = (p.p85 * assign3280_e4594);
        let assign3280_e4599: f64 = (var_inv_w * var_inv_l);
        let assign3280_e4601: f64 = (assign3280_e4599).powf(p.p88);
        let assign3280_e4602: f64 = (p.p87 * assign3280_e4601);
        let assign3280_e4603: f64 = (assign3280_e4595 + assign3280_e4602);
        var_t1 = assign3280_e4603;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        let assign3290_e4607: f64 = (1.0 + var_t0);
        let assign3290_e4609: f64 = (assign3290_e4607 + var_t1);
        let assign3290_e4610: f64 = (var_ndep_i * assign3290_e4609);
        var_ndep_i = assign3290_e4610;
        var_ndep_i_dn0 = ((var_ndep_i_dn0 * assign3290_e4609) + (var_ndep_i * (var_t0_dn0 + var_t1_dn0)));
        var_ndep_i_dn2 = ((var_ndep_i_dn2 * assign3290_e4609) + (var_ndep_i * (var_t0_dn2 + var_t1_dn2)));
        var_ndep_i_dn3 = ((var_ndep_i_dn3 * assign3290_e4609) + (var_ndep_i * (var_t0_dn3 + var_t1_dn3)));
        var_ndep_i_dn4 = ((var_ndep_i_dn4 * assign3290_e4609) + (var_ndep_i * (var_t0_dn4 + var_t1_dn4)));
        var_ndep_i_dn5 = ((var_ndep_i_dn5 * assign3290_e4609) + (var_ndep_i * (var_t0_dn5 + var_t1_dn5)));
        var_ndep_i_dn6 = ((var_ndep_i_dn6 * assign3290_e4609) + (var_ndep_i * (var_t0_dn6 + var_t1_dn6)));
        var_ndep_i_dn7 = ((var_ndep_i_dn7 * assign3290_e4609) + (var_ndep_i * (var_t0_dn7 + var_t1_dn7)));
        var_ndep_i_dn8 = ((var_ndep_i_dn8 * assign3290_e4609) + (var_ndep_i * (var_t0_dn8 + var_t1_dn8)));
        var_ndep_i_dn9 = ((var_ndep_i_dn9 * assign3290_e4609) + (var_ndep_i * (var_t0_dn9 + var_t1_dn9)));
        var_ndep_i_dn10 = ((var_ndep_i_dn10 * assign3290_e4609) + (var_ndep_i * (var_t0_dn10 + var_t1_dn10)));
        var_ndep_i_dn11 = ((var_ndep_i_dn11 * assign3290_e4609) + (var_ndep_i * (var_t0_dn11 + var_t1_dn11)));
        var_ndep_i_dn12 = ((var_ndep_i_dn12 * assign3290_e4609) + (var_ndep_i * (var_t0_dn12 + var_t1_dn12)));
        var_ndep_i_dn13 = ((var_ndep_i_dn13 * assign3290_e4609) + (var_ndep_i * (var_t0_dn13 + var_t1_dn13)));
        var_ndep_i_dn14 = ((var_ndep_i_dn14 * assign3290_e4609) + (var_ndep_i * (var_t0_dn14 + var_t1_dn14)));
        var_ndep_i_rv = 0.0;

        let assign3300_e4614: f64 = (var_inv_l).powf(p.p215);
        let assign3300_e4617: f64 = (var_inv_llong).powf(p.p215);
        let assign3300_e4618: f64 = (assign3300_e4614 - assign3300_e4617);
        let assign3300_e4620: f64 = (assign3300_e4618).max(0.0);
        let assign3300_e4621: f64 = (p.p214 * assign3300_e4620);
        var_t0 = assign3300_e4621;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign3310_e4625: f64 = (var_inv_w).powf(p.p217);
        let assign3310_e4628: f64 = (var_inv_wwide).powf(p.p217);
        let assign3310_e4629: f64 = (assign3310_e4625 - assign3310_e4628);
        let assign3310_e4631: f64 = (assign3310_e4629).max(0.0);
        let assign3310_e4632: f64 = (p.p216 * assign3310_e4631);
        let assign3310_e4636: f64 = (var_inv_wl).powf(p.p219);
        let assign3310_e4637: f64 = (p.p218 * assign3310_e4636);
        let assign3310_e4638: f64 = (assign3310_e4632 + assign3310_e4637);
        var_t1 = assign3310_e4638;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        let assign3320_e4642: f64 = (1.0 + var_t0);
        let assign3320_e4644: f64 = (assign3320_e4642 + var_t1);
        let assign3320_e4645: f64 = (var_nfactor_i * assign3320_e4644);
        var_nfactor_i = assign3320_e4645;
        var_nfactor_i_dn0 = ((var_nfactor_i_dn0 * assign3320_e4644) + (var_nfactor_i * (var_t0_dn0 + var_t1_dn0)));
        var_nfactor_i_dn2 = ((var_nfactor_i_dn2 * assign3320_e4644) + (var_nfactor_i * (var_t0_dn2 + var_t1_dn2)));
        var_nfactor_i_dn3 = ((var_nfactor_i_dn3 * assign3320_e4644) + (var_nfactor_i * (var_t0_dn3 + var_t1_dn3)));
        var_nfactor_i_dn4 = ((var_nfactor_i_dn4 * assign3320_e4644) + (var_nfactor_i * (var_t0_dn4 + var_t1_dn4)));
        var_nfactor_i_dn5 = ((var_nfactor_i_dn5 * assign3320_e4644) + (var_nfactor_i * (var_t0_dn5 + var_t1_dn5)));
        var_nfactor_i_dn6 = ((var_nfactor_i_dn6 * assign3320_e4644) + (var_nfactor_i * (var_t0_dn6 + var_t1_dn6)));
        var_nfactor_i_dn7 = ((var_nfactor_i_dn7 * assign3320_e4644) + (var_nfactor_i * (var_t0_dn7 + var_t1_dn7)));
        var_nfactor_i_dn8 = ((var_nfactor_i_dn8 * assign3320_e4644) + (var_nfactor_i * (var_t0_dn8 + var_t1_dn8)));
        var_nfactor_i_dn9 = ((var_nfactor_i_dn9 * assign3320_e4644) + (var_nfactor_i * (var_t0_dn9 + var_t1_dn9)));
        var_nfactor_i_dn10 = ((var_nfactor_i_dn10 * assign3320_e4644) + (var_nfactor_i * (var_t0_dn10 + var_t1_dn10)));
        var_nfactor_i_dn11 = ((var_nfactor_i_dn11 * assign3320_e4644) + (var_nfactor_i * (var_t0_dn11 + var_t1_dn11)));
        var_nfactor_i_dn12 = ((var_nfactor_i_dn12 * assign3320_e4644) + (var_nfactor_i * (var_t0_dn12 + var_t1_dn12)));
        var_nfactor_i_dn13 = ((var_nfactor_i_dn13 * assign3320_e4644) + (var_nfactor_i * (var_t0_dn13 + var_t1_dn13)));
        var_nfactor_i_dn14 = ((var_nfactor_i_dn14 * assign3320_e4644) + (var_nfactor_i * (var_t0_dn14 + var_t1_dn14)));
        var_nfactor_i_rv = 0.0;

        let assign3330_e4650: f64 = (var_inv_l).powf(p.p225);
        let assign3330_e4653: f64 = (var_inv_llong).powf(p.p225);
        let assign3330_e4654: f64 = (assign3330_e4650 - assign3330_e4653);
        let assign3330_e4656: f64 = (assign3330_e4654).max(0.0);
        let assign3330_e4657: f64 = (p.p224 * assign3330_e4656);
        let assign3330_e4658: f64 = (1.0 + assign3330_e4657);
        var_t0 = assign3330_e4658;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign3340_e4661: f64 = (var_cdscd_i * var_t0);
        var_cdscd_i = assign3340_e4661;
        var_cdscd_i_dn0 = ((var_cdscd_i_dn0 * var_t0) + (var_cdscd_i * var_t0_dn0));
        var_cdscd_i_dn2 = ((var_cdscd_i_dn2 * var_t0) + (var_cdscd_i * var_t0_dn2));
        var_cdscd_i_dn3 = ((var_cdscd_i_dn3 * var_t0) + (var_cdscd_i * var_t0_dn3));
        var_cdscd_i_dn4 = ((var_cdscd_i_dn4 * var_t0) + (var_cdscd_i * var_t0_dn4));
        var_cdscd_i_dn5 = ((var_cdscd_i_dn5 * var_t0) + (var_cdscd_i * var_t0_dn5));
        var_cdscd_i_dn6 = ((var_cdscd_i_dn6 * var_t0) + (var_cdscd_i * var_t0_dn6));
        var_cdscd_i_dn7 = ((var_cdscd_i_dn7 * var_t0) + (var_cdscd_i * var_t0_dn7));
        var_cdscd_i_dn8 = ((var_cdscd_i_dn8 * var_t0) + (var_cdscd_i * var_t0_dn8));
        var_cdscd_i_dn9 = ((var_cdscd_i_dn9 * var_t0) + (var_cdscd_i * var_t0_dn9));
        var_cdscd_i_dn10 = ((var_cdscd_i_dn10 * var_t0) + (var_cdscd_i * var_t0_dn10));
        var_cdscd_i_dn11 = ((var_cdscd_i_dn11 * var_t0) + (var_cdscd_i * var_t0_dn11));
        var_cdscd_i_dn12 = ((var_cdscd_i_dn12 * var_t0) + (var_cdscd_i * var_t0_dn12));
        var_cdscd_i_dn13 = ((var_cdscd_i_dn13 * var_t0) + (var_cdscd_i * var_t0_dn13));
        var_cdscd_i_dn14 = ((var_cdscd_i_dn14 * var_t0) + (var_cdscd_i * var_t0_dn14));
        var_cdscd_i_rv = 0.0;

        let assign3350_e4664: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        var_guard22 = assign3350_e4664;
        var_guard22_rv = 0.0;

        let (assign3360_e4670, assign3360_e4670_d_n0, assign3360_e4670_d_n2, assign3360_e4670_d_n3, assign3360_e4670_d_n4, assign3360_e4670_d_n5, assign3360_e4670_d_n6, assign3360_e4670_d_n7, assign3360_e4670_d_n8, assign3360_e4670_d_n9, assign3360_e4670_d_n10, assign3360_e4670_d_n11, assign3360_e4670_d_n12, assign3360_e4670_d_n13, assign3360_e4670_d_n14,) = {
    if (var_guard22 != 0.0) {
        let assign3360_e4668: f64 = (var_cdscdr_i * var_t0);
        (assign3360_e4668, ((var_cdscdr_i_dn0 * var_t0) + (var_cdscdr_i * var_t0_dn0)), ((var_cdscdr_i_dn2 * var_t0) + (var_cdscdr_i * var_t0_dn2)), ((var_cdscdr_i_dn3 * var_t0) + (var_cdscdr_i * var_t0_dn3)), ((var_cdscdr_i_dn4 * var_t0) + (var_cdscdr_i * var_t0_dn4)), ((var_cdscdr_i_dn5 * var_t0) + (var_cdscdr_i * var_t0_dn5)), ((var_cdscdr_i_dn6 * var_t0) + (var_cdscdr_i * var_t0_dn6)), ((var_cdscdr_i_dn7 * var_t0) + (var_cdscdr_i * var_t0_dn7)), ((var_cdscdr_i_dn8 * var_t0) + (var_cdscdr_i * var_t0_dn8)), ((var_cdscdr_i_dn9 * var_t0) + (var_cdscdr_i * var_t0_dn9)), ((var_cdscdr_i_dn10 * var_t0) + (var_cdscdr_i * var_t0_dn10)), ((var_cdscdr_i_dn11 * var_t0) + (var_cdscdr_i * var_t0_dn11)), ((var_cdscdr_i_dn12 * var_t0) + (var_cdscdr_i * var_t0_dn12)), ((var_cdscdr_i_dn13 * var_t0) + (var_cdscdr_i * var_t0_dn13)), ((var_cdscdr_i_dn14 * var_t0) + (var_cdscdr_i * var_t0_dn14)),)
    } else {
        (var_cdscdr_i, var_cdscdr_i_dn0, var_cdscdr_i_dn2, var_cdscdr_i_dn3, var_cdscdr_i_dn4, var_cdscdr_i_dn5, var_cdscdr_i_dn6, var_cdscdr_i_dn7, var_cdscdr_i_dn8, var_cdscdr_i_dn9, var_cdscdr_i_dn10, var_cdscdr_i_dn11, var_cdscdr_i_dn12, var_cdscdr_i_dn13, var_cdscdr_i_dn14,)
    }
};
        var_cdscdr_i = assign3360_e4670;
        var_cdscdr_i_dn0 = assign3360_e4670_d_n0;
        var_cdscdr_i_dn2 = assign3360_e4670_d_n2;
        var_cdscdr_i_dn3 = assign3360_e4670_d_n3;
        var_cdscdr_i_dn4 = assign3360_e4670_d_n4;
        var_cdscdr_i_dn5 = assign3360_e4670_d_n5;
        var_cdscdr_i_dn6 = assign3360_e4670_d_n6;
        var_cdscdr_i_dn7 = assign3360_e4670_d_n7;
        var_cdscdr_i_dn8 = assign3360_e4670_d_n8;
        var_cdscdr_i_dn9 = assign3360_e4670_d_n9;
        var_cdscdr_i_dn10 = assign3360_e4670_d_n10;
        var_cdscdr_i_dn11 = assign3360_e4670_d_n11;
        var_cdscdr_i_dn12 = assign3360_e4670_d_n12;
        var_cdscdr_i_dn13 = assign3360_e4670_d_n13;
        var_cdscdr_i_dn14 = assign3360_e4670_d_n14;
        var_cdscdr_i_rv = 0.0;

        let assign3370_e4676: f64 = (var_inv_l).powf(p.p235);
        let assign3370_e4679: f64 = (var_inv_llong).powf(p.p235);
        let assign3370_e4680: f64 = (assign3370_e4676 - assign3370_e4679);
        let assign3370_e4682: f64 = (assign3370_e4680).max(0.0);
        let assign3370_e4683: f64 = (p.p234 * assign3370_e4682);
        let assign3370_e4684: f64 = (1.0 + assign3370_e4683);
        let assign3370_e4685: f64 = (var_cdscb_i * assign3370_e4684);
        var_cdscb_i = assign3370_e4685;
        var_cdscb_i_rv = 0.0;

        let assign3380_e4688: f64 = (p.p34 * var_u0_i);
        var_u0_i = assign3380_e4688;
        var_u0_i_rv = 0.0;

        let assign3390_e4691: f64 = if p.p50 != 1.0 { 1.0 } else { 0.0 };
        var_guard23 = assign3390_e4691;
        var_guard23_rv = 0.0;

        let assign3400_e4694: f64 = if p.p275 > 0.0 { 1.0 } else { 0.0 };
        var_guard24 = assign3400_e4694;
        var_guard24_rv = 0.0;

        let (assign3410_e4714,) = {
    if ((var_guard23 != 0.0) && (var_guard24 != 0.0)) {
        let assign3410_e4703: f64 = (var_inv_l).powf(p.p275);
        let assign3410_e4706: f64 = (var_inv_llong).powf(p.p275);
        let assign3410_e4707: f64 = (assign3410_e4703 - assign3410_e4706);
        let assign3410_e4709: f64 = (assign3410_e4707).max(0.0);
        let assign3410_e4710: f64 = (p.p274 * assign3410_e4709);
        let assign3410_e4711: f64 = (1.0 - assign3410_e4710);
        let assign3410_e4712: f64 = (var_u0_i * assign3410_e4711);
        (assign3410_e4712,)
    } else {
        (var_u0_i,)
    }
};
        var_u0_i = assign3410_e4714;
        var_u0_i_rv = 0.0;

        let assign3420_e4717: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        var_guard25 = assign3420_e4717;
        var_guard25_rv = 0.0;

        let (assign3430_e4739,) = {
    if (((var_guard23 != 0.0) && (var_guard24 != 0.0)) && (var_guard25 != 0.0)) {
        let assign3430_e4728: f64 = (var_inv_l).powf(p.p275);
        let assign3430_e4731: f64 = (var_inv_llong).powf(p.p275);
        let assign3430_e4732: f64 = (assign3430_e4728 - assign3430_e4731);
        let assign3430_e4734: f64 = (assign3430_e4732).max(0.0);
        let assign3430_e4735: f64 = (p.p274 * assign3430_e4734);
        let assign3430_e4736: f64 = (1.0 - assign3430_e4735);
        let assign3430_e4737: f64 = (var_u0r_i * assign3430_e4736);
        (assign3430_e4737,)
    } else {
        (var_u0r_i,)
    }
};
        var_u0r_i = assign3430_e4739;
        var_u0r_i_rv = 0.0;

        let (assign3440_e4750,) = {
    if ((var_guard23 != 0.0) && (var_guard24 == 0.0)) {
        let assign3440_e4747: f64 = (1.0 - p.p274);
        let assign3440_e4748: f64 = (var_u0_i * assign3440_e4747);
        (assign3440_e4748,)
    } else {
        (var_u0_i,)
    }
};
        var_u0_i = assign3440_e4750;
        var_u0_i_rv = 0.0;

        let assign3450_e4753: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        var_guard26 = assign3450_e4753;
        var_guard26_rv = 0.0;

        let (assign3460_e4766,) = {
    if (((var_guard23 != 0.0) && (var_guard24 == 0.0)) && (var_guard26 != 0.0)) {
        let assign3460_e4763: f64 = (1.0 - p.p274);
        let assign3460_e4764: f64 = (var_u0r_i * assign3460_e4763);
        (assign3460_e4764,)
    } else {
        (var_u0r_i,)
    }
};
        var_u0r_i = assign3460_e4766;
        var_u0r_i_rv = 0.0;

        *var_alpha0r_i_slot = var_alpha0r_i;
        *var_alpha0r_i_dn0_slot = var_alpha0r_i_dn0;
        *var_alpha0r_i_dn10_slot = var_alpha0r_i_dn10;
        *var_alpha0r_i_dn11_slot = var_alpha0r_i_dn11;
        *var_alpha0r_i_dn12_slot = var_alpha0r_i_dn12;
        *var_alpha0r_i_dn13_slot = var_alpha0r_i_dn13;
        *var_alpha0r_i_dn14_slot = var_alpha0r_i_dn14;
        *var_alpha0r_i_dn2_slot = var_alpha0r_i_dn2;
        *var_alpha0r_i_dn3_slot = var_alpha0r_i_dn3;
        *var_alpha0r_i_dn4_slot = var_alpha0r_i_dn4;
        *var_alpha0r_i_dn5_slot = var_alpha0r_i_dn5;
        *var_alpha0r_i_dn6_slot = var_alpha0r_i_dn6;
        *var_alpha0r_i_dn7_slot = var_alpha0r_i_dn7;
        *var_alpha0r_i_dn8_slot = var_alpha0r_i_dn8;
        *var_alpha0r_i_dn9_slot = var_alpha0r_i_dn9;
        *var_alpha0r_i_rv_slot = var_alpha0r_i_rv;
        *var_beta0r_i_slot = var_beta0r_i;
        *var_beta0r_i_rv_slot = var_beta0r_i_rv;
        *var_cdscb_i_slot = var_cdscb_i;
        *var_cdscb_i_rv_slot = var_cdscb_i_rv;
        *var_cdscd_i_slot = var_cdscd_i;
        *var_cdscd_i_dn0_slot = var_cdscd_i_dn0;
        *var_cdscd_i_dn10_slot = var_cdscd_i_dn10;
        *var_cdscd_i_dn11_slot = var_cdscd_i_dn11;
        *var_cdscd_i_dn12_slot = var_cdscd_i_dn12;
        *var_cdscd_i_dn13_slot = var_cdscd_i_dn13;
        *var_cdscd_i_dn14_slot = var_cdscd_i_dn14;
        *var_cdscd_i_dn2_slot = var_cdscd_i_dn2;
        *var_cdscd_i_dn3_slot = var_cdscd_i_dn3;
        *var_cdscd_i_dn4_slot = var_cdscd_i_dn4;
        *var_cdscd_i_dn5_slot = var_cdscd_i_dn5;
        *var_cdscd_i_dn6_slot = var_cdscd_i_dn6;
        *var_cdscd_i_dn7_slot = var_cdscd_i_dn7;
        *var_cdscd_i_dn8_slot = var_cdscd_i_dn8;
        *var_cdscd_i_dn9_slot = var_cdscd_i_dn9;
        *var_cdscd_i_rv_slot = var_cdscd_i_rv;
        *var_cdscdr_i_slot = var_cdscdr_i;
        *var_cdscdr_i_dn0_slot = var_cdscdr_i_dn0;
        *var_cdscdr_i_dn10_slot = var_cdscdr_i_dn10;
        *var_cdscdr_i_dn11_slot = var_cdscdr_i_dn11;
        *var_cdscdr_i_dn12_slot = var_cdscdr_i_dn12;
        *var_cdscdr_i_dn13_slot = var_cdscdr_i_dn13;
        *var_cdscdr_i_dn14_slot = var_cdscdr_i_dn14;
        *var_cdscdr_i_dn2_slot = var_cdscdr_i_dn2;
        *var_cdscdr_i_dn3_slot = var_cdscdr_i_dn3;
        *var_cdscdr_i_dn4_slot = var_cdscdr_i_dn4;
        *var_cdscdr_i_dn5_slot = var_cdscdr_i_dn5;
        *var_cdscdr_i_dn6_slot = var_cdscdr_i_dn6;
        *var_cdscdr_i_dn7_slot = var_cdscdr_i_dn7;
        *var_cdscdr_i_dn8_slot = var_cdscdr_i_dn8;
        *var_cdscdr_i_dn9_slot = var_cdscdr_i_dn9;
        *var_cdscdr_i_rv_slot = var_cdscdr_i_rv;
        *var_guard22_slot = var_guard22;
        *var_guard22_rv_slot = var_guard22_rv;
        *var_guard23_slot = var_guard23;
        *var_guard23_rv_slot = var_guard23_rv;
        *var_guard24_slot = var_guard24;
        *var_guard24_rv_slot = var_guard24_rv;
        *var_guard25_slot = var_guard25;
        *var_guard25_rv_slot = var_guard25_rv;
        *var_guard26_slot = var_guard26;
        *var_guard26_rv_slot = var_guard26_rv;
        *var_ndep_i_slot = var_ndep_i;
        *var_ndep_i_dn0_slot = var_ndep_i_dn0;
        *var_ndep_i_dn10_slot = var_ndep_i_dn10;
        *var_ndep_i_dn11_slot = var_ndep_i_dn11;
        *var_ndep_i_dn12_slot = var_ndep_i_dn12;
        *var_ndep_i_dn13_slot = var_ndep_i_dn13;
        *var_ndep_i_dn14_slot = var_ndep_i_dn14;
        *var_ndep_i_dn2_slot = var_ndep_i_dn2;
        *var_ndep_i_dn3_slot = var_ndep_i_dn3;
        *var_ndep_i_dn4_slot = var_ndep_i_dn4;
        *var_ndep_i_dn5_slot = var_ndep_i_dn5;
        *var_ndep_i_dn6_slot = var_ndep_i_dn6;
        *var_ndep_i_dn7_slot = var_ndep_i_dn7;
        *var_ndep_i_dn8_slot = var_ndep_i_dn8;
        *var_ndep_i_dn9_slot = var_ndep_i_dn9;
        *var_ndep_i_rv_slot = var_ndep_i_rv;
        *var_nfactor_i_slot = var_nfactor_i;
        *var_nfactor_i_dn0_slot = var_nfactor_i_dn0;
        *var_nfactor_i_dn10_slot = var_nfactor_i_dn10;
        *var_nfactor_i_dn11_slot = var_nfactor_i_dn11;
        *var_nfactor_i_dn12_slot = var_nfactor_i_dn12;
        *var_nfactor_i_dn13_slot = var_nfactor_i_dn13;
        *var_nfactor_i_dn14_slot = var_nfactor_i_dn14;
        *var_nfactor_i_dn2_slot = var_nfactor_i_dn2;
        *var_nfactor_i_dn3_slot = var_nfactor_i_dn3;
        *var_nfactor_i_dn4_slot = var_nfactor_i_dn4;
        *var_nfactor_i_dn5_slot = var_nfactor_i_dn5;
        *var_nfactor_i_dn6_slot = var_nfactor_i_dn6;
        *var_nfactor_i_dn7_slot = var_nfactor_i_dn7;
        *var_nfactor_i_dn8_slot = var_nfactor_i_dn8;
        *var_nfactor_i_dn9_slot = var_nfactor_i_dn9;
        *var_nfactor_i_rv_slot = var_nfactor_i_rv;
        *var_pclmr_i_slot = var_pclmr_i;
        *var_pclmr_i_dn0_slot = var_pclmr_i_dn0;
        *var_pclmr_i_dn10_slot = var_pclmr_i_dn10;
        *var_pclmr_i_dn11_slot = var_pclmr_i_dn11;
        *var_pclmr_i_dn12_slot = var_pclmr_i_dn12;
        *var_pclmr_i_dn13_slot = var_pclmr_i_dn13;
        *var_pclmr_i_dn14_slot = var_pclmr_i_dn14;
        *var_pclmr_i_dn2_slot = var_pclmr_i_dn2;
        *var_pclmr_i_dn3_slot = var_pclmr_i_dn3;
        *var_pclmr_i_dn4_slot = var_pclmr_i_dn4;
        *var_pclmr_i_dn5_slot = var_pclmr_i_dn5;
        *var_pclmr_i_dn6_slot = var_pclmr_i_dn6;
        *var_pclmr_i_dn7_slot = var_pclmr_i_dn7;
        *var_pclmr_i_dn8_slot = var_pclmr_i_dn8;
        *var_pclmr_i_dn9_slot = var_pclmr_i_dn9;
        *var_pclmr_i_rv_slot = var_pclmr_i_rv;
        *var_pdiblcr_i_slot = var_pdiblcr_i;
        *var_pdiblcr_i_dn0_slot = var_pdiblcr_i_dn0;
        *var_pdiblcr_i_dn10_slot = var_pdiblcr_i_dn10;
        *var_pdiblcr_i_dn11_slot = var_pdiblcr_i_dn11;
        *var_pdiblcr_i_dn12_slot = var_pdiblcr_i_dn12;
        *var_pdiblcr_i_dn13_slot = var_pdiblcr_i_dn13;
        *var_pdiblcr_i_dn14_slot = var_pdiblcr_i_dn14;
        *var_pdiblcr_i_dn2_slot = var_pdiblcr_i_dn2;
        *var_pdiblcr_i_dn3_slot = var_pdiblcr_i_dn3;
        *var_pdiblcr_i_dn4_slot = var_pdiblcr_i_dn4;
        *var_pdiblcr_i_dn5_slot = var_pdiblcr_i_dn5;
        *var_pdiblcr_i_dn6_slot = var_pdiblcr_i_dn6;
        *var_pdiblcr_i_dn7_slot = var_pdiblcr_i_dn7;
        *var_pdiblcr_i_dn8_slot = var_pdiblcr_i_dn8;
        *var_pdiblcr_i_dn9_slot = var_pdiblcr_i_dn9;
        *var_pdiblcr_i_rv_slot = var_pdiblcr_i_rv;
        *var_psatr_i_slot = var_psatr_i;
        *var_psatr_i_rv_slot = var_psatr_i_rv;
        *var_ptwgr_i_slot = var_ptwgr_i;
        *var_ptwgr_i_dn0_slot = var_ptwgr_i_dn0;
        *var_ptwgr_i_dn10_slot = var_ptwgr_i_dn10;
        *var_ptwgr_i_dn11_slot = var_ptwgr_i_dn11;
        *var_ptwgr_i_dn12_slot = var_ptwgr_i_dn12;
        *var_ptwgr_i_dn13_slot = var_ptwgr_i_dn13;
        *var_ptwgr_i_dn14_slot = var_ptwgr_i_dn14;
        *var_ptwgr_i_dn2_slot = var_ptwgr_i_dn2;
        *var_ptwgr_i_dn3_slot = var_ptwgr_i_dn3;
        *var_ptwgr_i_dn4_slot = var_ptwgr_i_dn4;
        *var_ptwgr_i_dn5_slot = var_ptwgr_i_dn5;
        *var_ptwgr_i_dn6_slot = var_ptwgr_i_dn6;
        *var_ptwgr_i_dn7_slot = var_ptwgr_i_dn7;
        *var_ptwgr_i_dn8_slot = var_ptwgr_i_dn8;
        *var_ptwgr_i_dn9_slot = var_ptwgr_i_dn9;
        *var_ptwgr_i_rv_slot = var_ptwgr_i_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn13_slot = var_t0_dn13;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_u0_i_slot = var_u0_i;
        *var_u0_i_rv_slot = var_u0_i_rv;
        *var_u0r_i_slot = var_u0r_i;
        *var_u0r_i_rv_slot = var_u0r_i_rv;
        *var_ucr_i_slot = var_ucr_i;
        *var_ucr_i_dn0_slot = var_ucr_i_dn0;
        *var_ucr_i_dn10_slot = var_ucr_i_dn10;
        *var_ucr_i_dn11_slot = var_ucr_i_dn11;
        *var_ucr_i_dn12_slot = var_ucr_i_dn12;
        *var_ucr_i_dn13_slot = var_ucr_i_dn13;
        *var_ucr_i_dn14_slot = var_ucr_i_dn14;
        *var_ucr_i_dn2_slot = var_ucr_i_dn2;
        *var_ucr_i_dn3_slot = var_ucr_i_dn3;
        *var_ucr_i_dn4_slot = var_ucr_i_dn4;
        *var_ucr_i_dn5_slot = var_ucr_i_dn5;
        *var_ucr_i_dn6_slot = var_ucr_i_dn6;
        *var_ucr_i_dn7_slot = var_ucr_i_dn7;
        *var_ucr_i_dn8_slot = var_ucr_i_dn8;
        *var_ucr_i_dn9_slot = var_ucr_i_dn9;
        *var_ucr_i_rv_slot = var_ucr_i_rv;
        *var_vsatr_i_slot = var_vsatr_i;
        *var_vsatr_i_dn0_slot = var_vsatr_i_dn0;
        *var_vsatr_i_dn10_slot = var_vsatr_i_dn10;
        *var_vsatr_i_dn11_slot = var_vsatr_i_dn11;
        *var_vsatr_i_dn12_slot = var_vsatr_i_dn12;
        *var_vsatr_i_dn13_slot = var_vsatr_i_dn13;
        *var_vsatr_i_dn14_slot = var_vsatr_i_dn14;
        *var_vsatr_i_dn2_slot = var_vsatr_i_dn2;
        *var_vsatr_i_dn3_slot = var_vsatr_i_dn3;
        *var_vsatr_i_dn4_slot = var_vsatr_i_dn4;
        *var_vsatr_i_dn5_slot = var_vsatr_i_dn5;
        *var_vsatr_i_dn6_slot = var_vsatr_i_dn6;
        *var_vsatr_i_dn7_slot = var_vsatr_i_dn7;
        *var_vsatr_i_dn8_slot = var_vsatr_i_dn8;
        *var_vsatr_i_dn9_slot = var_vsatr_i_dn9;
        *var_vsatr_i_rv_slot = var_vsatr_i_rv;
    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        var_guard23: f64,
        var_inv_l: f64,
        var_inv_llong: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_inv_wwide: f64,
        var_leff: f64,
        var_eta0_i_slot: &mut f64,
        var_eta0_i_dn0_slot: &mut f64,
        var_eta0_i_dn10_slot: &mut f64,
        var_eta0_i_dn11_slot: &mut f64,
        var_eta0_i_dn12_slot: &mut f64,
        var_eta0_i_dn13_slot: &mut f64,
        var_eta0_i_dn14_slot: &mut f64,
        var_eta0_i_dn2_slot: &mut f64,
        var_eta0_i_dn3_slot: &mut f64,
        var_eta0_i_dn4_slot: &mut f64,
        var_eta0_i_dn5_slot: &mut f64,
        var_eta0_i_dn6_slot: &mut f64,
        var_eta0_i_dn7_slot: &mut f64,
        var_eta0_i_dn8_slot: &mut f64,
        var_eta0_i_dn9_slot: &mut f64,
        var_eta0_i_rv_slot: &mut f64,
        var_eta0r_i_slot: &mut f64,
        var_eta0r_i_dn0_slot: &mut f64,
        var_eta0r_i_dn10_slot: &mut f64,
        var_eta0r_i_dn11_slot: &mut f64,
        var_eta0r_i_dn12_slot: &mut f64,
        var_eta0r_i_dn13_slot: &mut f64,
        var_eta0r_i_dn14_slot: &mut f64,
        var_eta0r_i_dn2_slot: &mut f64,
        var_eta0r_i_dn3_slot: &mut f64,
        var_eta0r_i_dn4_slot: &mut f64,
        var_eta0r_i_dn5_slot: &mut f64,
        var_eta0r_i_dn6_slot: &mut f64,
        var_eta0r_i_dn7_slot: &mut f64,
        var_eta0r_i_dn8_slot: &mut f64,
        var_eta0r_i_dn9_slot: &mut f64,
        var_eta0r_i_rv_slot: &mut f64,
        var_etab_i_slot: &mut f64,
        var_etab_i_rv_slot: &mut f64,
        var_eu_i_slot: &mut f64,
        var_eu_i_dn0_slot: &mut f64,
        var_eu_i_dn10_slot: &mut f64,
        var_eu_i_dn11_slot: &mut f64,
        var_eu_i_dn12_slot: &mut f64,
        var_eu_i_dn13_slot: &mut f64,
        var_eu_i_dn14_slot: &mut f64,
        var_eu_i_dn2_slot: &mut f64,
        var_eu_i_dn3_slot: &mut f64,
        var_eu_i_dn4_slot: &mut f64,
        var_eu_i_dn5_slot: &mut f64,
        var_eu_i_dn6_slot: &mut f64,
        var_eu_i_dn7_slot: &mut f64,
        var_eu_i_dn8_slot: &mut f64,
        var_eu_i_dn9_slot: &mut f64,
        var_eu_i_rv_slot: &mut f64,
        var_guard27_slot: &mut f64,
        var_guard27_rv_slot: &mut f64,
        var_guard28_slot: &mut f64,
        var_guard28_rv_slot: &mut f64,
        var_guard29_slot: &mut f64,
        var_guard29_rv_slot: &mut f64,
        var_guard30_slot: &mut f64,
        var_guard30_rv_slot: &mut f64,
        var_guard31_slot: &mut f64,
        var_guard31_rv_slot: &mut f64,
        var_guard32_slot: &mut f64,
        var_guard32_rv_slot: &mut f64,
        var_pdiblc_i_slot: &mut f64,
        var_pdiblc_i_dn0_slot: &mut f64,
        var_pdiblc_i_dn10_slot: &mut f64,
        var_pdiblc_i_dn11_slot: &mut f64,
        var_pdiblc_i_dn12_slot: &mut f64,
        var_pdiblc_i_dn13_slot: &mut f64,
        var_pdiblc_i_dn14_slot: &mut f64,
        var_pdiblc_i_dn2_slot: &mut f64,
        var_pdiblc_i_dn3_slot: &mut f64,
        var_pdiblc_i_dn4_slot: &mut f64,
        var_pdiblc_i_dn5_slot: &mut f64,
        var_pdiblc_i_dn6_slot: &mut f64,
        var_pdiblc_i_dn7_slot: &mut f64,
        var_pdiblc_i_dn8_slot: &mut f64,
        var_pdiblc_i_dn9_slot: &mut f64,
        var_pdiblc_i_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_u0_i_slot: &mut f64,
        var_u0_i_rv_slot: &mut f64,
        var_u0r_i_slot: &mut f64,
        var_u0r_i_rv_slot: &mut f64,
        var_ua_i_slot: &mut f64,
        var_ua_i_dn0_slot: &mut f64,
        var_ua_i_dn10_slot: &mut f64,
        var_ua_i_dn11_slot: &mut f64,
        var_ua_i_dn12_slot: &mut f64,
        var_ua_i_dn13_slot: &mut f64,
        var_ua_i_dn14_slot: &mut f64,
        var_ua_i_dn2_slot: &mut f64,
        var_ua_i_dn3_slot: &mut f64,
        var_ua_i_dn4_slot: &mut f64,
        var_ua_i_dn5_slot: &mut f64,
        var_ua_i_dn6_slot: &mut f64,
        var_ua_i_dn7_slot: &mut f64,
        var_ua_i_dn8_slot: &mut f64,
        var_ua_i_dn9_slot: &mut f64,
        var_ua_i_rv_slot: &mut f64,
        var_uar_i_slot: &mut f64,
        var_uar_i_dn0_slot: &mut f64,
        var_uar_i_dn10_slot: &mut f64,
        var_uar_i_dn11_slot: &mut f64,
        var_uar_i_dn12_slot: &mut f64,
        var_uar_i_dn13_slot: &mut f64,
        var_uar_i_dn14_slot: &mut f64,
        var_uar_i_dn2_slot: &mut f64,
        var_uar_i_dn3_slot: &mut f64,
        var_uar_i_dn4_slot: &mut f64,
        var_uar_i_dn5_slot: &mut f64,
        var_uar_i_dn6_slot: &mut f64,
        var_uar_i_dn7_slot: &mut f64,
        var_uar_i_dn8_slot: &mut f64,
        var_uar_i_dn9_slot: &mut f64,
        var_uar_i_rv_slot: &mut f64,
        var_uc_i_slot: &mut f64,
        var_uc_i_dn0_slot: &mut f64,
        var_uc_i_dn10_slot: &mut f64,
        var_uc_i_dn11_slot: &mut f64,
        var_uc_i_dn12_slot: &mut f64,
        var_uc_i_dn13_slot: &mut f64,
        var_uc_i_dn14_slot: &mut f64,
        var_uc_i_dn2_slot: &mut f64,
        var_uc_i_dn3_slot: &mut f64,
        var_uc_i_dn4_slot: &mut f64,
        var_uc_i_dn5_slot: &mut f64,
        var_uc_i_dn6_slot: &mut f64,
        var_uc_i_dn7_slot: &mut f64,
        var_uc_i_dn8_slot: &mut f64,
        var_uc_i_dn9_slot: &mut f64,
        var_uc_i_rv_slot: &mut f64,
        var_ucr_i_slot: &mut f64,
        var_ucr_i_dn0_slot: &mut f64,
        var_ucr_i_dn10_slot: &mut f64,
        var_ucr_i_dn11_slot: &mut f64,
        var_ucr_i_dn12_slot: &mut f64,
        var_ucr_i_dn13_slot: &mut f64,
        var_ucr_i_dn14_slot: &mut f64,
        var_ucr_i_dn2_slot: &mut f64,
        var_ucr_i_dn3_slot: &mut f64,
        var_ucr_i_dn4_slot: &mut f64,
        var_ucr_i_dn5_slot: &mut f64,
        var_ucr_i_dn6_slot: &mut f64,
        var_ucr_i_dn7_slot: &mut f64,
        var_ucr_i_dn8_slot: &mut f64,
        var_ucr_i_dn9_slot: &mut f64,
        var_ucr_i_rv_slot: &mut f64,
        var_ud_i_slot: &mut f64,
        var_ud_i_dn0_slot: &mut f64,
        var_ud_i_dn10_slot: &mut f64,
        var_ud_i_dn11_slot: &mut f64,
        var_ud_i_dn12_slot: &mut f64,
        var_ud_i_dn13_slot: &mut f64,
        var_ud_i_dn14_slot: &mut f64,
        var_ud_i_dn2_slot: &mut f64,
        var_ud_i_dn3_slot: &mut f64,
        var_ud_i_dn4_slot: &mut f64,
        var_ud_i_dn5_slot: &mut f64,
        var_ud_i_dn6_slot: &mut f64,
        var_ud_i_dn7_slot: &mut f64,
        var_ud_i_dn8_slot: &mut f64,
        var_ud_i_dn9_slot: &mut f64,
        var_ud_i_rv_slot: &mut f64,
        var_udr_i_slot: &mut f64,
        var_udr_i_dn0_slot: &mut f64,
        var_udr_i_dn10_slot: &mut f64,
        var_udr_i_dn11_slot: &mut f64,
        var_udr_i_dn12_slot: &mut f64,
        var_udr_i_dn13_slot: &mut f64,
        var_udr_i_dn14_slot: &mut f64,
        var_udr_i_dn2_slot: &mut f64,
        var_udr_i_dn3_slot: &mut f64,
        var_udr_i_dn4_slot: &mut f64,
        var_udr_i_dn5_slot: &mut f64,
        var_udr_i_dn6_slot: &mut f64,
        var_udr_i_dn7_slot: &mut f64,
        var_udr_i_dn8_slot: &mut f64,
        var_udr_i_dn9_slot: &mut f64,
        var_udr_i_rv_slot: &mut f64,
    ) {
        let mut var_eta0_i: f64 = *var_eta0_i_slot;
        let mut var_eta0_i_dn0: f64 = *var_eta0_i_dn0_slot;
        let mut var_eta0_i_dn10: f64 = *var_eta0_i_dn10_slot;
        let mut var_eta0_i_dn11: f64 = *var_eta0_i_dn11_slot;
        let mut var_eta0_i_dn12: f64 = *var_eta0_i_dn12_slot;
        let mut var_eta0_i_dn13: f64 = *var_eta0_i_dn13_slot;
        let mut var_eta0_i_dn14: f64 = *var_eta0_i_dn14_slot;
        let mut var_eta0_i_dn2: f64 = *var_eta0_i_dn2_slot;
        let mut var_eta0_i_dn3: f64 = *var_eta0_i_dn3_slot;
        let mut var_eta0_i_dn4: f64 = *var_eta0_i_dn4_slot;
        let mut var_eta0_i_dn5: f64 = *var_eta0_i_dn5_slot;
        let mut var_eta0_i_dn6: f64 = *var_eta0_i_dn6_slot;
        let mut var_eta0_i_dn7: f64 = *var_eta0_i_dn7_slot;
        let mut var_eta0_i_dn8: f64 = *var_eta0_i_dn8_slot;
        let mut var_eta0_i_dn9: f64 = *var_eta0_i_dn9_slot;
        let mut var_eta0_i_rv: f64 = *var_eta0_i_rv_slot;
        let mut var_eta0r_i: f64 = *var_eta0r_i_slot;
        let mut var_eta0r_i_dn0: f64 = *var_eta0r_i_dn0_slot;
        let mut var_eta0r_i_dn10: f64 = *var_eta0r_i_dn10_slot;
        let mut var_eta0r_i_dn11: f64 = *var_eta0r_i_dn11_slot;
        let mut var_eta0r_i_dn12: f64 = *var_eta0r_i_dn12_slot;
        let mut var_eta0r_i_dn13: f64 = *var_eta0r_i_dn13_slot;
        let mut var_eta0r_i_dn14: f64 = *var_eta0r_i_dn14_slot;
        let mut var_eta0r_i_dn2: f64 = *var_eta0r_i_dn2_slot;
        let mut var_eta0r_i_dn3: f64 = *var_eta0r_i_dn3_slot;
        let mut var_eta0r_i_dn4: f64 = *var_eta0r_i_dn4_slot;
        let mut var_eta0r_i_dn5: f64 = *var_eta0r_i_dn5_slot;
        let mut var_eta0r_i_dn6: f64 = *var_eta0r_i_dn6_slot;
        let mut var_eta0r_i_dn7: f64 = *var_eta0r_i_dn7_slot;
        let mut var_eta0r_i_dn8: f64 = *var_eta0r_i_dn8_slot;
        let mut var_eta0r_i_dn9: f64 = *var_eta0r_i_dn9_slot;
        let mut var_eta0r_i_rv: f64 = *var_eta0r_i_rv_slot;
        let mut var_etab_i: f64 = *var_etab_i_slot;
        let mut var_etab_i_rv: f64 = *var_etab_i_rv_slot;
        let mut var_eu_i: f64 = *var_eu_i_slot;
        let mut var_eu_i_dn0: f64 = *var_eu_i_dn0_slot;
        let mut var_eu_i_dn10: f64 = *var_eu_i_dn10_slot;
        let mut var_eu_i_dn11: f64 = *var_eu_i_dn11_slot;
        let mut var_eu_i_dn12: f64 = *var_eu_i_dn12_slot;
        let mut var_eu_i_dn13: f64 = *var_eu_i_dn13_slot;
        let mut var_eu_i_dn14: f64 = *var_eu_i_dn14_slot;
        let mut var_eu_i_dn2: f64 = *var_eu_i_dn2_slot;
        let mut var_eu_i_dn3: f64 = *var_eu_i_dn3_slot;
        let mut var_eu_i_dn4: f64 = *var_eu_i_dn4_slot;
        let mut var_eu_i_dn5: f64 = *var_eu_i_dn5_slot;
        let mut var_eu_i_dn6: f64 = *var_eu_i_dn6_slot;
        let mut var_eu_i_dn7: f64 = *var_eu_i_dn7_slot;
        let mut var_eu_i_dn8: f64 = *var_eu_i_dn8_slot;
        let mut var_eu_i_dn9: f64 = *var_eu_i_dn9_slot;
        let mut var_eu_i_rv: f64 = *var_eu_i_rv_slot;
        let mut var_guard27: f64 = *var_guard27_slot;
        let mut var_guard27_rv: f64 = *var_guard27_rv_slot;
        let mut var_guard28: f64 = *var_guard28_slot;
        let mut var_guard28_rv: f64 = *var_guard28_rv_slot;
        let mut var_guard29: f64 = *var_guard29_slot;
        let mut var_guard29_rv: f64 = *var_guard29_rv_slot;
        let mut var_guard30: f64 = *var_guard30_slot;
        let mut var_guard30_rv: f64 = *var_guard30_rv_slot;
        let mut var_guard31: f64 = *var_guard31_slot;
        let mut var_guard31_rv: f64 = *var_guard31_rv_slot;
        let mut var_guard32: f64 = *var_guard32_slot;
        let mut var_guard32_rv: f64 = *var_guard32_rv_slot;
        let mut var_pdiblc_i: f64 = *var_pdiblc_i_slot;
        let mut var_pdiblc_i_dn0: f64 = *var_pdiblc_i_dn0_slot;
        let mut var_pdiblc_i_dn10: f64 = *var_pdiblc_i_dn10_slot;
        let mut var_pdiblc_i_dn11: f64 = *var_pdiblc_i_dn11_slot;
        let mut var_pdiblc_i_dn12: f64 = *var_pdiblc_i_dn12_slot;
        let mut var_pdiblc_i_dn13: f64 = *var_pdiblc_i_dn13_slot;
        let mut var_pdiblc_i_dn14: f64 = *var_pdiblc_i_dn14_slot;
        let mut var_pdiblc_i_dn2: f64 = *var_pdiblc_i_dn2_slot;
        let mut var_pdiblc_i_dn3: f64 = *var_pdiblc_i_dn3_slot;
        let mut var_pdiblc_i_dn4: f64 = *var_pdiblc_i_dn4_slot;
        let mut var_pdiblc_i_dn5: f64 = *var_pdiblc_i_dn5_slot;
        let mut var_pdiblc_i_dn6: f64 = *var_pdiblc_i_dn6_slot;
        let mut var_pdiblc_i_dn7: f64 = *var_pdiblc_i_dn7_slot;
        let mut var_pdiblc_i_dn8: f64 = *var_pdiblc_i_dn8_slot;
        let mut var_pdiblc_i_dn9: f64 = *var_pdiblc_i_dn9_slot;
        let mut var_pdiblc_i_rv: f64 = *var_pdiblc_i_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_u0_i: f64 = *var_u0_i_slot;
        let mut var_u0_i_rv: f64 = *var_u0_i_rv_slot;
        let mut var_u0r_i: f64 = *var_u0r_i_slot;
        let mut var_u0r_i_rv: f64 = *var_u0r_i_rv_slot;
        let mut var_ua_i: f64 = *var_ua_i_slot;
        let mut var_ua_i_dn0: f64 = *var_ua_i_dn0_slot;
        let mut var_ua_i_dn10: f64 = *var_ua_i_dn10_slot;
        let mut var_ua_i_dn11: f64 = *var_ua_i_dn11_slot;
        let mut var_ua_i_dn12: f64 = *var_ua_i_dn12_slot;
        let mut var_ua_i_dn13: f64 = *var_ua_i_dn13_slot;
        let mut var_ua_i_dn14: f64 = *var_ua_i_dn14_slot;
        let mut var_ua_i_dn2: f64 = *var_ua_i_dn2_slot;
        let mut var_ua_i_dn3: f64 = *var_ua_i_dn3_slot;
        let mut var_ua_i_dn4: f64 = *var_ua_i_dn4_slot;
        let mut var_ua_i_dn5: f64 = *var_ua_i_dn5_slot;
        let mut var_ua_i_dn6: f64 = *var_ua_i_dn6_slot;
        let mut var_ua_i_dn7: f64 = *var_ua_i_dn7_slot;
        let mut var_ua_i_dn8: f64 = *var_ua_i_dn8_slot;
        let mut var_ua_i_dn9: f64 = *var_ua_i_dn9_slot;
        let mut var_ua_i_rv: f64 = *var_ua_i_rv_slot;
        let mut var_uar_i: f64 = *var_uar_i_slot;
        let mut var_uar_i_dn0: f64 = *var_uar_i_dn0_slot;
        let mut var_uar_i_dn10: f64 = *var_uar_i_dn10_slot;
        let mut var_uar_i_dn11: f64 = *var_uar_i_dn11_slot;
        let mut var_uar_i_dn12: f64 = *var_uar_i_dn12_slot;
        let mut var_uar_i_dn13: f64 = *var_uar_i_dn13_slot;
        let mut var_uar_i_dn14: f64 = *var_uar_i_dn14_slot;
        let mut var_uar_i_dn2: f64 = *var_uar_i_dn2_slot;
        let mut var_uar_i_dn3: f64 = *var_uar_i_dn3_slot;
        let mut var_uar_i_dn4: f64 = *var_uar_i_dn4_slot;
        let mut var_uar_i_dn5: f64 = *var_uar_i_dn5_slot;
        let mut var_uar_i_dn6: f64 = *var_uar_i_dn6_slot;
        let mut var_uar_i_dn7: f64 = *var_uar_i_dn7_slot;
        let mut var_uar_i_dn8: f64 = *var_uar_i_dn8_slot;
        let mut var_uar_i_dn9: f64 = *var_uar_i_dn9_slot;
        let mut var_uar_i_rv: f64 = *var_uar_i_rv_slot;
        let mut var_uc_i: f64 = *var_uc_i_slot;
        let mut var_uc_i_dn0: f64 = *var_uc_i_dn0_slot;
        let mut var_uc_i_dn10: f64 = *var_uc_i_dn10_slot;
        let mut var_uc_i_dn11: f64 = *var_uc_i_dn11_slot;
        let mut var_uc_i_dn12: f64 = *var_uc_i_dn12_slot;
        let mut var_uc_i_dn13: f64 = *var_uc_i_dn13_slot;
        let mut var_uc_i_dn14: f64 = *var_uc_i_dn14_slot;
        let mut var_uc_i_dn2: f64 = *var_uc_i_dn2_slot;
        let mut var_uc_i_dn3: f64 = *var_uc_i_dn3_slot;
        let mut var_uc_i_dn4: f64 = *var_uc_i_dn4_slot;
        let mut var_uc_i_dn5: f64 = *var_uc_i_dn5_slot;
        let mut var_uc_i_dn6: f64 = *var_uc_i_dn6_slot;
        let mut var_uc_i_dn7: f64 = *var_uc_i_dn7_slot;
        let mut var_uc_i_dn8: f64 = *var_uc_i_dn8_slot;
        let mut var_uc_i_dn9: f64 = *var_uc_i_dn9_slot;
        let mut var_uc_i_rv: f64 = *var_uc_i_rv_slot;
        let mut var_ucr_i: f64 = *var_ucr_i_slot;
        let mut var_ucr_i_dn0: f64 = *var_ucr_i_dn0_slot;
        let mut var_ucr_i_dn10: f64 = *var_ucr_i_dn10_slot;
        let mut var_ucr_i_dn11: f64 = *var_ucr_i_dn11_slot;
        let mut var_ucr_i_dn12: f64 = *var_ucr_i_dn12_slot;
        let mut var_ucr_i_dn13: f64 = *var_ucr_i_dn13_slot;
        let mut var_ucr_i_dn14: f64 = *var_ucr_i_dn14_slot;
        let mut var_ucr_i_dn2: f64 = *var_ucr_i_dn2_slot;
        let mut var_ucr_i_dn3: f64 = *var_ucr_i_dn3_slot;
        let mut var_ucr_i_dn4: f64 = *var_ucr_i_dn4_slot;
        let mut var_ucr_i_dn5: f64 = *var_ucr_i_dn5_slot;
        let mut var_ucr_i_dn6: f64 = *var_ucr_i_dn6_slot;
        let mut var_ucr_i_dn7: f64 = *var_ucr_i_dn7_slot;
        let mut var_ucr_i_dn8: f64 = *var_ucr_i_dn8_slot;
        let mut var_ucr_i_dn9: f64 = *var_ucr_i_dn9_slot;
        let mut var_ucr_i_rv: f64 = *var_ucr_i_rv_slot;
        let mut var_ud_i: f64 = *var_ud_i_slot;
        let mut var_ud_i_dn0: f64 = *var_ud_i_dn0_slot;
        let mut var_ud_i_dn10: f64 = *var_ud_i_dn10_slot;
        let mut var_ud_i_dn11: f64 = *var_ud_i_dn11_slot;
        let mut var_ud_i_dn12: f64 = *var_ud_i_dn12_slot;
        let mut var_ud_i_dn13: f64 = *var_ud_i_dn13_slot;
        let mut var_ud_i_dn14: f64 = *var_ud_i_dn14_slot;
        let mut var_ud_i_dn2: f64 = *var_ud_i_dn2_slot;
        let mut var_ud_i_dn3: f64 = *var_ud_i_dn3_slot;
        let mut var_ud_i_dn4: f64 = *var_ud_i_dn4_slot;
        let mut var_ud_i_dn5: f64 = *var_ud_i_dn5_slot;
        let mut var_ud_i_dn6: f64 = *var_ud_i_dn6_slot;
        let mut var_ud_i_dn7: f64 = *var_ud_i_dn7_slot;
        let mut var_ud_i_dn8: f64 = *var_ud_i_dn8_slot;
        let mut var_ud_i_dn9: f64 = *var_ud_i_dn9_slot;
        let mut var_ud_i_rv: f64 = *var_ud_i_rv_slot;
        let mut var_udr_i: f64 = *var_udr_i_slot;
        let mut var_udr_i_dn0: f64 = *var_udr_i_dn0_slot;
        let mut var_udr_i_dn10: f64 = *var_udr_i_dn10_slot;
        let mut var_udr_i_dn11: f64 = *var_udr_i_dn11_slot;
        let mut var_udr_i_dn12: f64 = *var_udr_i_dn12_slot;
        let mut var_udr_i_dn13: f64 = *var_udr_i_dn13_slot;
        let mut var_udr_i_dn14: f64 = *var_udr_i_dn14_slot;
        let mut var_udr_i_dn2: f64 = *var_udr_i_dn2_slot;
        let mut var_udr_i_dn3: f64 = *var_udr_i_dn3_slot;
        let mut var_udr_i_dn4: f64 = *var_udr_i_dn4_slot;
        let mut var_udr_i_dn5: f64 = *var_udr_i_dn5_slot;
        let mut var_udr_i_dn6: f64 = *var_udr_i_dn6_slot;
        let mut var_udr_i_dn7: f64 = *var_udr_i_dn7_slot;
        let mut var_udr_i_dn8: f64 = *var_udr_i_dn8_slot;
        let mut var_udr_i_dn9: f64 = *var_udr_i_dn9_slot;
        let mut var_udr_i_rv: f64 = *var_udr_i_rv_slot;

        let (assign3470_e4789,) = {
    if (var_guard23 == 0.0) {
        let assign3470_e4773: f64 = (-var_leff);
        let assign3470_e4775: f64 = (assign3470_e4773 / p.p270);
        let assign3470_e4776: f64 = { let limited_exp_arg = assign3470_e4775; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign3470_e4777: f64 = (p.p269 * assign3470_e4776);
        let assign3470_e4778: f64 = (1.0 - assign3470_e4777);
        let assign3470_e4781: f64 = (-var_leff);
        let assign3470_e4783: f64 = (assign3470_e4781 / p.p272);
        let assign3470_e4784: f64 = { let limited_exp_arg = assign3470_e4783; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign3470_e4785: f64 = (p.p271 * assign3470_e4784);
        let assign3470_e4786: f64 = (assign3470_e4778 - assign3470_e4785);
        let assign3470_e4787: f64 = (var_u0_i * assign3470_e4786);
        (assign3470_e4787,)
    } else {
        (var_u0_i,)
    }
};
        var_u0_i = assign3470_e4789;
        var_u0_i_rv = 0.0;

        let assign3480_e4792: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        var_guard27 = assign3480_e4792;
        var_guard27_rv = 0.0;

        let (assign3490_e4817,) = {
    if ((var_guard23 == 0.0) && (var_guard27 != 0.0)) {
        let assign3490_e4801: f64 = (-var_leff);
        let assign3490_e4803: f64 = (assign3490_e4801 / p.p270);
        let assign3490_e4804: f64 = { let limited_exp_arg = assign3490_e4803; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign3490_e4805: f64 = (p.p269 * assign3490_e4804);
        let assign3490_e4806: f64 = (1.0 - assign3490_e4805);
        let assign3490_e4809: f64 = (-var_leff);
        let assign3490_e4811: f64 = (assign3490_e4809 / p.p272);
        let assign3490_e4812: f64 = { let limited_exp_arg = assign3490_e4811; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign3490_e4813: f64 = (p.p271 * assign3490_e4812);
        let assign3490_e4814: f64 = (assign3490_e4806 - assign3490_e4813);
        let assign3490_e4815: f64 = (var_u0r_i * assign3490_e4814);
        (assign3490_e4815,)
    } else {
        (var_u0r_i,)
    }
};
        var_u0r_i = assign3490_e4817;
        var_u0r_i_rv = 0.0;

        let assign3500_e4821: f64 = (var_inv_l).powf(p.p286);
        let assign3500_e4824: f64 = (var_inv_llong).powf(p.p286);
        let assign3500_e4825: f64 = (assign3500_e4821 - assign3500_e4824);
        let assign3500_e4827: f64 = (assign3500_e4825).max(0.0);
        let assign3500_e4828: f64 = (p.p285 * assign3500_e4827);
        var_t0 = assign3500_e4828;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign3510_e4832: f64 = (var_inv_w).powf(p.p288);
        let assign3510_e4835: f64 = (var_inv_wwide).powf(p.p288);
        let assign3510_e4836: f64 = (assign3510_e4832 - assign3510_e4835);
        let assign3510_e4838: f64 = (assign3510_e4836).max(0.0);
        let assign3510_e4839: f64 = (p.p287 * assign3510_e4838);
        let assign3510_e4843: f64 = (var_inv_wl).powf(p.p290);
        let assign3510_e4844: f64 = (p.p289 * assign3510_e4843);
        let assign3510_e4845: f64 = (assign3510_e4839 + assign3510_e4844);
        var_t1 = assign3510_e4845;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        let assign3520_e4849: f64 = (1.0 + var_t0);
        let assign3520_e4851: f64 = (assign3520_e4849 + var_t1);
        let assign3520_e4852: f64 = (var_ua_i * assign3520_e4851);
        var_ua_i = assign3520_e4852;
        var_ua_i_dn0 = ((var_ua_i_dn0 * assign3520_e4851) + (var_ua_i * (var_t0_dn0 + var_t1_dn0)));
        var_ua_i_dn2 = ((var_ua_i_dn2 * assign3520_e4851) + (var_ua_i * (var_t0_dn2 + var_t1_dn2)));
        var_ua_i_dn3 = ((var_ua_i_dn3 * assign3520_e4851) + (var_ua_i * (var_t0_dn3 + var_t1_dn3)));
        var_ua_i_dn4 = ((var_ua_i_dn4 * assign3520_e4851) + (var_ua_i * (var_t0_dn4 + var_t1_dn4)));
        var_ua_i_dn5 = ((var_ua_i_dn5 * assign3520_e4851) + (var_ua_i * (var_t0_dn5 + var_t1_dn5)));
        var_ua_i_dn6 = ((var_ua_i_dn6 * assign3520_e4851) + (var_ua_i * (var_t0_dn6 + var_t1_dn6)));
        var_ua_i_dn7 = ((var_ua_i_dn7 * assign3520_e4851) + (var_ua_i * (var_t0_dn7 + var_t1_dn7)));
        var_ua_i_dn8 = ((var_ua_i_dn8 * assign3520_e4851) + (var_ua_i * (var_t0_dn8 + var_t1_dn8)));
        var_ua_i_dn9 = ((var_ua_i_dn9 * assign3520_e4851) + (var_ua_i * (var_t0_dn9 + var_t1_dn9)));
        var_ua_i_dn10 = ((var_ua_i_dn10 * assign3520_e4851) + (var_ua_i * (var_t0_dn10 + var_t1_dn10)));
        var_ua_i_dn11 = ((var_ua_i_dn11 * assign3520_e4851) + (var_ua_i * (var_t0_dn11 + var_t1_dn11)));
        var_ua_i_dn12 = ((var_ua_i_dn12 * assign3520_e4851) + (var_ua_i * (var_t0_dn12 + var_t1_dn12)));
        var_ua_i_dn13 = ((var_ua_i_dn13 * assign3520_e4851) + (var_ua_i * (var_t0_dn13 + var_t1_dn13)));
        var_ua_i_dn14 = ((var_ua_i_dn14 * assign3520_e4851) + (var_ua_i * (var_t0_dn14 + var_t1_dn14)));
        var_ua_i_rv = 0.0;

        let assign3530_e4855: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        var_guard28 = assign3530_e4855;
        var_guard28_rv = 0.0;

        let (assign3540_e4865, assign3540_e4865_d_n0, assign3540_e4865_d_n2, assign3540_e4865_d_n3, assign3540_e4865_d_n4, assign3540_e4865_d_n5, assign3540_e4865_d_n6, assign3540_e4865_d_n7, assign3540_e4865_d_n8, assign3540_e4865_d_n9, assign3540_e4865_d_n10, assign3540_e4865_d_n11, assign3540_e4865_d_n12, assign3540_e4865_d_n13, assign3540_e4865_d_n14,) = {
    if (var_guard28 != 0.0) {
        let assign3540_e4860: f64 = (1.0 + var_t0);
        let assign3540_e4862: f64 = (assign3540_e4860 + var_t1);
        let assign3540_e4863: f64 = (var_uar_i * assign3540_e4862);
        (assign3540_e4863, ((var_uar_i_dn0 * assign3540_e4862) + (var_uar_i * (var_t0_dn0 + var_t1_dn0))), ((var_uar_i_dn2 * assign3540_e4862) + (var_uar_i * (var_t0_dn2 + var_t1_dn2))), ((var_uar_i_dn3 * assign3540_e4862) + (var_uar_i * (var_t0_dn3 + var_t1_dn3))), ((var_uar_i_dn4 * assign3540_e4862) + (var_uar_i * (var_t0_dn4 + var_t1_dn4))), ((var_uar_i_dn5 * assign3540_e4862) + (var_uar_i * (var_t0_dn5 + var_t1_dn5))), ((var_uar_i_dn6 * assign3540_e4862) + (var_uar_i * (var_t0_dn6 + var_t1_dn6))), ((var_uar_i_dn7 * assign3540_e4862) + (var_uar_i * (var_t0_dn7 + var_t1_dn7))), ((var_uar_i_dn8 * assign3540_e4862) + (var_uar_i * (var_t0_dn8 + var_t1_dn8))), ((var_uar_i_dn9 * assign3540_e4862) + (var_uar_i * (var_t0_dn9 + var_t1_dn9))), ((var_uar_i_dn10 * assign3540_e4862) + (var_uar_i * (var_t0_dn10 + var_t1_dn10))), ((var_uar_i_dn11 * assign3540_e4862) + (var_uar_i * (var_t0_dn11 + var_t1_dn11))), ((var_uar_i_dn12 * assign3540_e4862) + (var_uar_i * (var_t0_dn12 + var_t1_dn12))), ((var_uar_i_dn13 * assign3540_e4862) + (var_uar_i * (var_t0_dn13 + var_t1_dn13))), ((var_uar_i_dn14 * assign3540_e4862) + (var_uar_i * (var_t0_dn14 + var_t1_dn14))),)
    } else {
        (var_uar_i, var_uar_i_dn0, var_uar_i_dn2, var_uar_i_dn3, var_uar_i_dn4, var_uar_i_dn5, var_uar_i_dn6, var_uar_i_dn7, var_uar_i_dn8, var_uar_i_dn9, var_uar_i_dn10, var_uar_i_dn11, var_uar_i_dn12, var_uar_i_dn13, var_uar_i_dn14,)
    }
};
        var_uar_i = assign3540_e4865;
        var_uar_i_dn0 = assign3540_e4865_d_n0;
        var_uar_i_dn2 = assign3540_e4865_d_n2;
        var_uar_i_dn3 = assign3540_e4865_d_n3;
        var_uar_i_dn4 = assign3540_e4865_d_n4;
        var_uar_i_dn5 = assign3540_e4865_d_n5;
        var_uar_i_dn6 = assign3540_e4865_d_n6;
        var_uar_i_dn7 = assign3540_e4865_d_n7;
        var_uar_i_dn8 = assign3540_e4865_d_n8;
        var_uar_i_dn9 = assign3540_e4865_d_n9;
        var_uar_i_dn10 = assign3540_e4865_d_n10;
        var_uar_i_dn11 = assign3540_e4865_d_n11;
        var_uar_i_dn12 = assign3540_e4865_d_n12;
        var_uar_i_dn13 = assign3540_e4865_d_n13;
        var_uar_i_dn14 = assign3540_e4865_d_n14;
        var_uar_i_rv = 0.0;

        let assign3550_e4869: f64 = (var_inv_l).powf(p.p303);
        let assign3550_e4872: f64 = (var_inv_llong).powf(p.p303);
        let assign3550_e4873: f64 = (assign3550_e4869 - assign3550_e4872);
        let assign3550_e4875: f64 = (assign3550_e4873).max(0.0);
        let assign3550_e4876: f64 = (p.p302 * assign3550_e4875);
        var_t0 = assign3550_e4876;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign3560_e4880: f64 = (var_inv_w).powf(p.p305);
        let assign3560_e4883: f64 = (var_inv_wwide).powf(p.p305);
        let assign3560_e4884: f64 = (assign3560_e4880 - assign3560_e4883);
        let assign3560_e4886: f64 = (assign3560_e4884).max(0.0);
        let assign3560_e4887: f64 = (p.p304 * assign3560_e4886);
        let assign3560_e4891: f64 = (var_inv_wl).powf(p.p307);
        let assign3560_e4892: f64 = (p.p306 * assign3560_e4891);
        let assign3560_e4893: f64 = (assign3560_e4887 + assign3560_e4892);
        var_t1 = assign3560_e4893;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        let assign3570_e4897: f64 = (1.0 + var_t0);
        let assign3570_e4899: f64 = (assign3570_e4897 + var_t1);
        let assign3570_e4900: f64 = (var_eu_i * assign3570_e4899);
        var_eu_i = assign3570_e4900;
        var_eu_i_dn0 = ((var_eu_i_dn0 * assign3570_e4899) + (var_eu_i * (var_t0_dn0 + var_t1_dn0)));
        var_eu_i_dn2 = ((var_eu_i_dn2 * assign3570_e4899) + (var_eu_i * (var_t0_dn2 + var_t1_dn2)));
        var_eu_i_dn3 = ((var_eu_i_dn3 * assign3570_e4899) + (var_eu_i * (var_t0_dn3 + var_t1_dn3)));
        var_eu_i_dn4 = ((var_eu_i_dn4 * assign3570_e4899) + (var_eu_i * (var_t0_dn4 + var_t1_dn4)));
        var_eu_i_dn5 = ((var_eu_i_dn5 * assign3570_e4899) + (var_eu_i * (var_t0_dn5 + var_t1_dn5)));
        var_eu_i_dn6 = ((var_eu_i_dn6 * assign3570_e4899) + (var_eu_i * (var_t0_dn6 + var_t1_dn6)));
        var_eu_i_dn7 = ((var_eu_i_dn7 * assign3570_e4899) + (var_eu_i * (var_t0_dn7 + var_t1_dn7)));
        var_eu_i_dn8 = ((var_eu_i_dn8 * assign3570_e4899) + (var_eu_i * (var_t0_dn8 + var_t1_dn8)));
        var_eu_i_dn9 = ((var_eu_i_dn9 * assign3570_e4899) + (var_eu_i * (var_t0_dn9 + var_t1_dn9)));
        var_eu_i_dn10 = ((var_eu_i_dn10 * assign3570_e4899) + (var_eu_i * (var_t0_dn10 + var_t1_dn10)));
        var_eu_i_dn11 = ((var_eu_i_dn11 * assign3570_e4899) + (var_eu_i * (var_t0_dn11 + var_t1_dn11)));
        var_eu_i_dn12 = ((var_eu_i_dn12 * assign3570_e4899) + (var_eu_i * (var_t0_dn12 + var_t1_dn12)));
        var_eu_i_dn13 = ((var_eu_i_dn13 * assign3570_e4899) + (var_eu_i * (var_t0_dn13 + var_t1_dn13)));
        var_eu_i_dn14 = ((var_eu_i_dn14 * assign3570_e4899) + (var_eu_i * (var_t0_dn14 + var_t1_dn14)));
        var_eu_i_rv = 0.0;

        let assign3580_e4905: f64 = (var_inv_l).powf(p.p310);
        let assign3580_e4908: f64 = (var_inv_llong).powf(p.p310);
        let assign3580_e4909: f64 = (assign3580_e4905 - assign3580_e4908);
        let assign3580_e4911: f64 = (assign3580_e4909).max(0.0);
        let assign3580_e4912: f64 = (p.p309 * assign3580_e4911);
        let assign3580_e4913: f64 = (1.0 + assign3580_e4912);
        var_t0 = assign3580_e4913;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign3590_e4916: f64 = (var_ud_i * var_t0);
        var_ud_i = assign3590_e4916;
        var_ud_i_dn0 = ((var_ud_i_dn0 * var_t0) + (var_ud_i * var_t0_dn0));
        var_ud_i_dn2 = ((var_ud_i_dn2 * var_t0) + (var_ud_i * var_t0_dn2));
        var_ud_i_dn3 = ((var_ud_i_dn3 * var_t0) + (var_ud_i * var_t0_dn3));
        var_ud_i_dn4 = ((var_ud_i_dn4 * var_t0) + (var_ud_i * var_t0_dn4));
        var_ud_i_dn5 = ((var_ud_i_dn5 * var_t0) + (var_ud_i * var_t0_dn5));
        var_ud_i_dn6 = ((var_ud_i_dn6 * var_t0) + (var_ud_i * var_t0_dn6));
        var_ud_i_dn7 = ((var_ud_i_dn7 * var_t0) + (var_ud_i * var_t0_dn7));
        var_ud_i_dn8 = ((var_ud_i_dn8 * var_t0) + (var_ud_i * var_t0_dn8));
        var_ud_i_dn9 = ((var_ud_i_dn9 * var_t0) + (var_ud_i * var_t0_dn9));
        var_ud_i_dn10 = ((var_ud_i_dn10 * var_t0) + (var_ud_i * var_t0_dn10));
        var_ud_i_dn11 = ((var_ud_i_dn11 * var_t0) + (var_ud_i * var_t0_dn11));
        var_ud_i_dn12 = ((var_ud_i_dn12 * var_t0) + (var_ud_i * var_t0_dn12));
        var_ud_i_dn13 = ((var_ud_i_dn13 * var_t0) + (var_ud_i * var_t0_dn13));
        var_ud_i_dn14 = ((var_ud_i_dn14 * var_t0) + (var_ud_i * var_t0_dn14));
        var_ud_i_rv = 0.0;

        let assign3600_e4919: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        var_guard29 = assign3600_e4919;
        var_guard29_rv = 0.0;

        let (assign3610_e4925, assign3610_e4925_d_n0, assign3610_e4925_d_n2, assign3610_e4925_d_n3, assign3610_e4925_d_n4, assign3610_e4925_d_n5, assign3610_e4925_d_n6, assign3610_e4925_d_n7, assign3610_e4925_d_n8, assign3610_e4925_d_n9, assign3610_e4925_d_n10, assign3610_e4925_d_n11, assign3610_e4925_d_n12, assign3610_e4925_d_n13, assign3610_e4925_d_n14,) = {
    if (var_guard29 != 0.0) {
        let assign3610_e4923: f64 = (var_udr_i * var_t0);
        (assign3610_e4923, ((var_udr_i_dn0 * var_t0) + (var_udr_i * var_t0_dn0)), ((var_udr_i_dn2 * var_t0) + (var_udr_i * var_t0_dn2)), ((var_udr_i_dn3 * var_t0) + (var_udr_i * var_t0_dn3)), ((var_udr_i_dn4 * var_t0) + (var_udr_i * var_t0_dn4)), ((var_udr_i_dn5 * var_t0) + (var_udr_i * var_t0_dn5)), ((var_udr_i_dn6 * var_t0) + (var_udr_i * var_t0_dn6)), ((var_udr_i_dn7 * var_t0) + (var_udr_i * var_t0_dn7)), ((var_udr_i_dn8 * var_t0) + (var_udr_i * var_t0_dn8)), ((var_udr_i_dn9 * var_t0) + (var_udr_i * var_t0_dn9)), ((var_udr_i_dn10 * var_t0) + (var_udr_i * var_t0_dn10)), ((var_udr_i_dn11 * var_t0) + (var_udr_i * var_t0_dn11)), ((var_udr_i_dn12 * var_t0) + (var_udr_i * var_t0_dn12)), ((var_udr_i_dn13 * var_t0) + (var_udr_i * var_t0_dn13)), ((var_udr_i_dn14 * var_t0) + (var_udr_i * var_t0_dn14)),)
    } else {
        (var_udr_i, var_udr_i_dn0, var_udr_i_dn2, var_udr_i_dn3, var_udr_i_dn4, var_udr_i_dn5, var_udr_i_dn6, var_udr_i_dn7, var_udr_i_dn8, var_udr_i_dn9, var_udr_i_dn10, var_udr_i_dn11, var_udr_i_dn12, var_udr_i_dn13, var_udr_i_dn14,)
    }
};
        var_udr_i = assign3610_e4925;
        var_udr_i_dn0 = assign3610_e4925_d_n0;
        var_udr_i_dn2 = assign3610_e4925_d_n2;
        var_udr_i_dn3 = assign3610_e4925_d_n3;
        var_udr_i_dn4 = assign3610_e4925_d_n4;
        var_udr_i_dn5 = assign3610_e4925_d_n5;
        var_udr_i_dn6 = assign3610_e4925_d_n6;
        var_udr_i_dn7 = assign3610_e4925_d_n7;
        var_udr_i_dn8 = assign3610_e4925_d_n8;
        var_udr_i_dn9 = assign3610_e4925_d_n9;
        var_udr_i_dn10 = assign3610_e4925_d_n10;
        var_udr_i_dn11 = assign3610_e4925_d_n11;
        var_udr_i_dn12 = assign3610_e4925_d_n12;
        var_udr_i_dn13 = assign3610_e4925_d_n13;
        var_udr_i_dn14 = assign3610_e4925_d_n14;
        var_udr_i_rv = 0.0;

        let assign3620_e4929: f64 = (var_inv_l).powf(p.p328);
        let assign3620_e4932: f64 = (var_inv_llong).powf(p.p328);
        let assign3620_e4933: f64 = (assign3620_e4929 - assign3620_e4932);
        let assign3620_e4935: f64 = (assign3620_e4933).max(0.0);
        let assign3620_e4936: f64 = (p.p327 * assign3620_e4935);
        var_t0 = assign3620_e4936;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign3630_e4940: f64 = (var_inv_w).powf(p.p330);
        let assign3630_e4943: f64 = (var_inv_wwide).powf(p.p330);
        let assign3630_e4944: f64 = (assign3630_e4940 - assign3630_e4943);
        let assign3630_e4946: f64 = (assign3630_e4944).max(0.0);
        let assign3630_e4947: f64 = (p.p329 * assign3630_e4946);
        let assign3630_e4951: f64 = (var_inv_wl).powf(p.p332);
        let assign3630_e4952: f64 = (p.p331 * assign3630_e4951);
        let assign3630_e4953: f64 = (assign3630_e4947 + assign3630_e4952);
        var_t1 = assign3630_e4953;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        let assign3640_e4957: f64 = (1.0 + var_t0);
        let assign3640_e4959: f64 = (assign3640_e4957 + var_t1);
        let assign3640_e4960: f64 = (var_uc_i * assign3640_e4959);
        var_uc_i = assign3640_e4960;
        var_uc_i_dn0 = ((var_uc_i_dn0 * assign3640_e4959) + (var_uc_i * (var_t0_dn0 + var_t1_dn0)));
        var_uc_i_dn2 = ((var_uc_i_dn2 * assign3640_e4959) + (var_uc_i * (var_t0_dn2 + var_t1_dn2)));
        var_uc_i_dn3 = ((var_uc_i_dn3 * assign3640_e4959) + (var_uc_i * (var_t0_dn3 + var_t1_dn3)));
        var_uc_i_dn4 = ((var_uc_i_dn4 * assign3640_e4959) + (var_uc_i * (var_t0_dn4 + var_t1_dn4)));
        var_uc_i_dn5 = ((var_uc_i_dn5 * assign3640_e4959) + (var_uc_i * (var_t0_dn5 + var_t1_dn5)));
        var_uc_i_dn6 = ((var_uc_i_dn6 * assign3640_e4959) + (var_uc_i * (var_t0_dn6 + var_t1_dn6)));
        var_uc_i_dn7 = ((var_uc_i_dn7 * assign3640_e4959) + (var_uc_i * (var_t0_dn7 + var_t1_dn7)));
        var_uc_i_dn8 = ((var_uc_i_dn8 * assign3640_e4959) + (var_uc_i * (var_t0_dn8 + var_t1_dn8)));
        var_uc_i_dn9 = ((var_uc_i_dn9 * assign3640_e4959) + (var_uc_i * (var_t0_dn9 + var_t1_dn9)));
        var_uc_i_dn10 = ((var_uc_i_dn10 * assign3640_e4959) + (var_uc_i * (var_t0_dn10 + var_t1_dn10)));
        var_uc_i_dn11 = ((var_uc_i_dn11 * assign3640_e4959) + (var_uc_i * (var_t0_dn11 + var_t1_dn11)));
        var_uc_i_dn12 = ((var_uc_i_dn12 * assign3640_e4959) + (var_uc_i * (var_t0_dn12 + var_t1_dn12)));
        var_uc_i_dn13 = ((var_uc_i_dn13 * assign3640_e4959) + (var_uc_i * (var_t0_dn13 + var_t1_dn13)));
        var_uc_i_dn14 = ((var_uc_i_dn14 * assign3640_e4959) + (var_uc_i * (var_t0_dn14 + var_t1_dn14)));
        var_uc_i_rv = 0.0;

        let assign3650_e4963: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        var_guard30 = assign3650_e4963;
        var_guard30_rv = 0.0;

        let (assign3660_e4973, assign3660_e4973_d_n0, assign3660_e4973_d_n2, assign3660_e4973_d_n3, assign3660_e4973_d_n4, assign3660_e4973_d_n5, assign3660_e4973_d_n6, assign3660_e4973_d_n7, assign3660_e4973_d_n8, assign3660_e4973_d_n9, assign3660_e4973_d_n10, assign3660_e4973_d_n11, assign3660_e4973_d_n12, assign3660_e4973_d_n13, assign3660_e4973_d_n14,) = {
    if (var_guard30 != 0.0) {
        let assign3660_e4968: f64 = (1.0 + var_t0);
        let assign3660_e4970: f64 = (assign3660_e4968 + var_t1);
        let assign3660_e4971: f64 = (var_ucr_i * assign3660_e4970);
        (assign3660_e4971, ((var_ucr_i_dn0 * assign3660_e4970) + (var_ucr_i * (var_t0_dn0 + var_t1_dn0))), ((var_ucr_i_dn2 * assign3660_e4970) + (var_ucr_i * (var_t0_dn2 + var_t1_dn2))), ((var_ucr_i_dn3 * assign3660_e4970) + (var_ucr_i * (var_t0_dn3 + var_t1_dn3))), ((var_ucr_i_dn4 * assign3660_e4970) + (var_ucr_i * (var_t0_dn4 + var_t1_dn4))), ((var_ucr_i_dn5 * assign3660_e4970) + (var_ucr_i * (var_t0_dn5 + var_t1_dn5))), ((var_ucr_i_dn6 * assign3660_e4970) + (var_ucr_i * (var_t0_dn6 + var_t1_dn6))), ((var_ucr_i_dn7 * assign3660_e4970) + (var_ucr_i * (var_t0_dn7 + var_t1_dn7))), ((var_ucr_i_dn8 * assign3660_e4970) + (var_ucr_i * (var_t0_dn8 + var_t1_dn8))), ((var_ucr_i_dn9 * assign3660_e4970) + (var_ucr_i * (var_t0_dn9 + var_t1_dn9))), ((var_ucr_i_dn10 * assign3660_e4970) + (var_ucr_i * (var_t0_dn10 + var_t1_dn10))), ((var_ucr_i_dn11 * assign3660_e4970) + (var_ucr_i * (var_t0_dn11 + var_t1_dn11))), ((var_ucr_i_dn12 * assign3660_e4970) + (var_ucr_i * (var_t0_dn12 + var_t1_dn12))), ((var_ucr_i_dn13 * assign3660_e4970) + (var_ucr_i * (var_t0_dn13 + var_t1_dn13))), ((var_ucr_i_dn14 * assign3660_e4970) + (var_ucr_i * (var_t0_dn14 + var_t1_dn14))),)
    } else {
        (var_ucr_i, var_ucr_i_dn0, var_ucr_i_dn2, var_ucr_i_dn3, var_ucr_i_dn4, var_ucr_i_dn5, var_ucr_i_dn6, var_ucr_i_dn7, var_ucr_i_dn8, var_ucr_i_dn9, var_ucr_i_dn10, var_ucr_i_dn11, var_ucr_i_dn12, var_ucr_i_dn13, var_ucr_i_dn14,)
    }
};
        var_ucr_i = assign3660_e4973;
        var_ucr_i_dn0 = assign3660_e4973_d_n0;
        var_ucr_i_dn2 = assign3660_e4973_d_n2;
        var_ucr_i_dn3 = assign3660_e4973_d_n3;
        var_ucr_i_dn4 = assign3660_e4973_d_n4;
        var_ucr_i_dn5 = assign3660_e4973_d_n5;
        var_ucr_i_dn6 = assign3660_e4973_d_n6;
        var_ucr_i_dn7 = assign3660_e4973_d_n7;
        var_ucr_i_dn8 = assign3660_e4973_d_n8;
        var_ucr_i_dn9 = assign3660_e4973_d_n9;
        var_ucr_i_dn10 = assign3660_e4973_d_n10;
        var_ucr_i_dn11 = assign3660_e4973_d_n11;
        var_ucr_i_dn12 = assign3660_e4973_d_n12;
        var_ucr_i_dn13 = assign3660_e4973_d_n13;
        var_ucr_i_dn14 = assign3660_e4973_d_n14;
        var_ucr_i_rv = 0.0;

        let assign3670_e4976: f64 = (var_inv_l).powf(p.p179);
        let assign3670_e4979: f64 = (var_inv_llong).powf(p.p179);
        let assign3670_e4980: f64 = (assign3670_e4976 - assign3670_e4979);
        let assign3670_e4982: f64 = (assign3670_e4980).max(0.0);
        var_t0 = assign3670_e4982;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign3680_e4985: f64 = (var_eta0_i * var_t0);
        var_eta0_i = assign3680_e4985;
        var_eta0_i_dn0 = ((var_eta0_i_dn0 * var_t0) + (var_eta0_i * var_t0_dn0));
        var_eta0_i_dn2 = ((var_eta0_i_dn2 * var_t0) + (var_eta0_i * var_t0_dn2));
        var_eta0_i_dn3 = ((var_eta0_i_dn3 * var_t0) + (var_eta0_i * var_t0_dn3));
        var_eta0_i_dn4 = ((var_eta0_i_dn4 * var_t0) + (var_eta0_i * var_t0_dn4));
        var_eta0_i_dn5 = ((var_eta0_i_dn5 * var_t0) + (var_eta0_i * var_t0_dn5));
        var_eta0_i_dn6 = ((var_eta0_i_dn6 * var_t0) + (var_eta0_i * var_t0_dn6));
        var_eta0_i_dn7 = ((var_eta0_i_dn7 * var_t0) + (var_eta0_i * var_t0_dn7));
        var_eta0_i_dn8 = ((var_eta0_i_dn8 * var_t0) + (var_eta0_i * var_t0_dn8));
        var_eta0_i_dn9 = ((var_eta0_i_dn9 * var_t0) + (var_eta0_i * var_t0_dn9));
        var_eta0_i_dn10 = ((var_eta0_i_dn10 * var_t0) + (var_eta0_i * var_t0_dn10));
        var_eta0_i_dn11 = ((var_eta0_i_dn11 * var_t0) + (var_eta0_i * var_t0_dn11));
        var_eta0_i_dn12 = ((var_eta0_i_dn12 * var_t0) + (var_eta0_i * var_t0_dn12));
        var_eta0_i_dn13 = ((var_eta0_i_dn13 * var_t0) + (var_eta0_i * var_t0_dn13));
        var_eta0_i_dn14 = ((var_eta0_i_dn14 * var_t0) + (var_eta0_i * var_t0_dn14));
        var_eta0_i_rv = 0.0;

        let assign3690_e4988: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        var_guard31 = assign3690_e4988;
        var_guard31_rv = 0.0;

        let (assign3700_e4994, assign3700_e4994_d_n0, assign3700_e4994_d_n2, assign3700_e4994_d_n3, assign3700_e4994_d_n4, assign3700_e4994_d_n5, assign3700_e4994_d_n6, assign3700_e4994_d_n7, assign3700_e4994_d_n8, assign3700_e4994_d_n9, assign3700_e4994_d_n10, assign3700_e4994_d_n11, assign3700_e4994_d_n12, assign3700_e4994_d_n13, assign3700_e4994_d_n14,) = {
    if (var_guard31 != 0.0) {
        let assign3700_e4992: f64 = (var_eta0r_i * var_t0);
        (assign3700_e4992, ((var_eta0r_i_dn0 * var_t0) + (var_eta0r_i * var_t0_dn0)), ((var_eta0r_i_dn2 * var_t0) + (var_eta0r_i * var_t0_dn2)), ((var_eta0r_i_dn3 * var_t0) + (var_eta0r_i * var_t0_dn3)), ((var_eta0r_i_dn4 * var_t0) + (var_eta0r_i * var_t0_dn4)), ((var_eta0r_i_dn5 * var_t0) + (var_eta0r_i * var_t0_dn5)), ((var_eta0r_i_dn6 * var_t0) + (var_eta0r_i * var_t0_dn6)), ((var_eta0r_i_dn7 * var_t0) + (var_eta0r_i * var_t0_dn7)), ((var_eta0r_i_dn8 * var_t0) + (var_eta0r_i * var_t0_dn8)), ((var_eta0r_i_dn9 * var_t0) + (var_eta0r_i * var_t0_dn9)), ((var_eta0r_i_dn10 * var_t0) + (var_eta0r_i * var_t0_dn10)), ((var_eta0r_i_dn11 * var_t0) + (var_eta0r_i * var_t0_dn11)), ((var_eta0r_i_dn12 * var_t0) + (var_eta0r_i * var_t0_dn12)), ((var_eta0r_i_dn13 * var_t0) + (var_eta0r_i * var_t0_dn13)), ((var_eta0r_i_dn14 * var_t0) + (var_eta0r_i * var_t0_dn14)),)
    } else {
        (var_eta0r_i, var_eta0r_i_dn0, var_eta0r_i_dn2, var_eta0r_i_dn3, var_eta0r_i_dn4, var_eta0r_i_dn5, var_eta0r_i_dn6, var_eta0r_i_dn7, var_eta0r_i_dn8, var_eta0r_i_dn9, var_eta0r_i_dn10, var_eta0r_i_dn11, var_eta0r_i_dn12, var_eta0r_i_dn13, var_eta0r_i_dn14,)
    }
};
        var_eta0r_i = assign3700_e4994;
        var_eta0r_i_dn0 = assign3700_e4994_d_n0;
        var_eta0r_i_dn2 = assign3700_e4994_d_n2;
        var_eta0r_i_dn3 = assign3700_e4994_d_n3;
        var_eta0r_i_dn4 = assign3700_e4994_d_n4;
        var_eta0r_i_dn5 = assign3700_e4994_d_n5;
        var_eta0r_i_dn6 = assign3700_e4994_d_n6;
        var_eta0r_i_dn7 = assign3700_e4994_d_n7;
        var_eta0r_i_dn8 = assign3700_e4994_d_n8;
        var_eta0r_i_dn9 = assign3700_e4994_d_n9;
        var_eta0r_i_dn10 = assign3700_e4994_d_n10;
        var_eta0r_i_dn11 = assign3700_e4994_d_n11;
        var_eta0r_i_dn12 = assign3700_e4994_d_n12;
        var_eta0r_i_dn13 = assign3700_e4994_d_n13;
        var_eta0r_i_dn14 = assign3700_e4994_d_n14;
        var_eta0r_i_rv = 0.0;

        let assign3710_e4998: f64 = (var_inv_l).powf(p.p181);
        let assign3710_e5001: f64 = (var_inv_llong).powf(p.p181);
        let assign3710_e5002: f64 = (assign3710_e4998 - assign3710_e5001);
        let assign3710_e5004: f64 = (assign3710_e5002).max(0.0);
        let assign3710_e5005: f64 = (var_etab_i * assign3710_e5004);
        var_etab_i = assign3710_e5005;
        var_etab_i_rv = 0.0;

        let assign3720_e5010: f64 = (var_inv_l).powf(p.p462);
        let assign3720_e5013: f64 = (var_inv_llong).powf(p.p462);
        let assign3720_e5014: f64 = (assign3720_e5010 - assign3720_e5013);
        let assign3720_e5016: f64 = (assign3720_e5014).max(0.0);
        let assign3720_e5017: f64 = (p.p461 * assign3720_e5016);
        let assign3720_e5018: f64 = (1.0 + assign3720_e5017);
        var_t0 = assign3720_e5018;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign3730_e5021: f64 = (var_pdiblc_i * var_t0);
        var_pdiblc_i = assign3730_e5021;
        var_pdiblc_i_dn0 = ((var_pdiblc_i_dn0 * var_t0) + (var_pdiblc_i * var_t0_dn0));
        var_pdiblc_i_dn2 = ((var_pdiblc_i_dn2 * var_t0) + (var_pdiblc_i * var_t0_dn2));
        var_pdiblc_i_dn3 = ((var_pdiblc_i_dn3 * var_t0) + (var_pdiblc_i * var_t0_dn3));
        var_pdiblc_i_dn4 = ((var_pdiblc_i_dn4 * var_t0) + (var_pdiblc_i * var_t0_dn4));
        var_pdiblc_i_dn5 = ((var_pdiblc_i_dn5 * var_t0) + (var_pdiblc_i * var_t0_dn5));
        var_pdiblc_i_dn6 = ((var_pdiblc_i_dn6 * var_t0) + (var_pdiblc_i * var_t0_dn6));
        var_pdiblc_i_dn7 = ((var_pdiblc_i_dn7 * var_t0) + (var_pdiblc_i * var_t0_dn7));
        var_pdiblc_i_dn8 = ((var_pdiblc_i_dn8 * var_t0) + (var_pdiblc_i * var_t0_dn8));
        var_pdiblc_i_dn9 = ((var_pdiblc_i_dn9 * var_t0) + (var_pdiblc_i * var_t0_dn9));
        var_pdiblc_i_dn10 = ((var_pdiblc_i_dn10 * var_t0) + (var_pdiblc_i * var_t0_dn10));
        var_pdiblc_i_dn11 = ((var_pdiblc_i_dn11 * var_t0) + (var_pdiblc_i * var_t0_dn11));
        var_pdiblc_i_dn12 = ((var_pdiblc_i_dn12 * var_t0) + (var_pdiblc_i * var_t0_dn12));
        var_pdiblc_i_dn13 = ((var_pdiblc_i_dn13 * var_t0) + (var_pdiblc_i * var_t0_dn13));
        var_pdiblc_i_dn14 = ((var_pdiblc_i_dn14 * var_t0) + (var_pdiblc_i * var_t0_dn14));
        var_pdiblc_i_rv = 0.0;

        let assign3740_e5024: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        var_guard32 = assign3740_e5024;
        var_guard32_rv = 0.0;

        *var_eta0_i_slot = var_eta0_i;
        *var_eta0_i_dn0_slot = var_eta0_i_dn0;
        *var_eta0_i_dn10_slot = var_eta0_i_dn10;
        *var_eta0_i_dn11_slot = var_eta0_i_dn11;
        *var_eta0_i_dn12_slot = var_eta0_i_dn12;
        *var_eta0_i_dn13_slot = var_eta0_i_dn13;
        *var_eta0_i_dn14_slot = var_eta0_i_dn14;
        *var_eta0_i_dn2_slot = var_eta0_i_dn2;
        *var_eta0_i_dn3_slot = var_eta0_i_dn3;
        *var_eta0_i_dn4_slot = var_eta0_i_dn4;
        *var_eta0_i_dn5_slot = var_eta0_i_dn5;
        *var_eta0_i_dn6_slot = var_eta0_i_dn6;
        *var_eta0_i_dn7_slot = var_eta0_i_dn7;
        *var_eta0_i_dn8_slot = var_eta0_i_dn8;
        *var_eta0_i_dn9_slot = var_eta0_i_dn9;
        *var_eta0_i_rv_slot = var_eta0_i_rv;
        *var_eta0r_i_slot = var_eta0r_i;
        *var_eta0r_i_dn0_slot = var_eta0r_i_dn0;
        *var_eta0r_i_dn10_slot = var_eta0r_i_dn10;
        *var_eta0r_i_dn11_slot = var_eta0r_i_dn11;
        *var_eta0r_i_dn12_slot = var_eta0r_i_dn12;
        *var_eta0r_i_dn13_slot = var_eta0r_i_dn13;
        *var_eta0r_i_dn14_slot = var_eta0r_i_dn14;
        *var_eta0r_i_dn2_slot = var_eta0r_i_dn2;
        *var_eta0r_i_dn3_slot = var_eta0r_i_dn3;
        *var_eta0r_i_dn4_slot = var_eta0r_i_dn4;
        *var_eta0r_i_dn5_slot = var_eta0r_i_dn5;
        *var_eta0r_i_dn6_slot = var_eta0r_i_dn6;
        *var_eta0r_i_dn7_slot = var_eta0r_i_dn7;
        *var_eta0r_i_dn8_slot = var_eta0r_i_dn8;
        *var_eta0r_i_dn9_slot = var_eta0r_i_dn9;
        *var_eta0r_i_rv_slot = var_eta0r_i_rv;
        *var_etab_i_slot = var_etab_i;
        *var_etab_i_rv_slot = var_etab_i_rv;
        *var_eu_i_slot = var_eu_i;
        *var_eu_i_dn0_slot = var_eu_i_dn0;
        *var_eu_i_dn10_slot = var_eu_i_dn10;
        *var_eu_i_dn11_slot = var_eu_i_dn11;
        *var_eu_i_dn12_slot = var_eu_i_dn12;
        *var_eu_i_dn13_slot = var_eu_i_dn13;
        *var_eu_i_dn14_slot = var_eu_i_dn14;
        *var_eu_i_dn2_slot = var_eu_i_dn2;
        *var_eu_i_dn3_slot = var_eu_i_dn3;
        *var_eu_i_dn4_slot = var_eu_i_dn4;
        *var_eu_i_dn5_slot = var_eu_i_dn5;
        *var_eu_i_dn6_slot = var_eu_i_dn6;
        *var_eu_i_dn7_slot = var_eu_i_dn7;
        *var_eu_i_dn8_slot = var_eu_i_dn8;
        *var_eu_i_dn9_slot = var_eu_i_dn9;
        *var_eu_i_rv_slot = var_eu_i_rv;
        *var_guard27_slot = var_guard27;
        *var_guard27_rv_slot = var_guard27_rv;
        *var_guard28_slot = var_guard28;
        *var_guard28_rv_slot = var_guard28_rv;
        *var_guard29_slot = var_guard29;
        *var_guard29_rv_slot = var_guard29_rv;
        *var_guard30_slot = var_guard30;
        *var_guard30_rv_slot = var_guard30_rv;
        *var_guard31_slot = var_guard31;
        *var_guard31_rv_slot = var_guard31_rv;
        *var_guard32_slot = var_guard32;
        *var_guard32_rv_slot = var_guard32_rv;
        *var_pdiblc_i_slot = var_pdiblc_i;
        *var_pdiblc_i_dn0_slot = var_pdiblc_i_dn0;
        *var_pdiblc_i_dn10_slot = var_pdiblc_i_dn10;
        *var_pdiblc_i_dn11_slot = var_pdiblc_i_dn11;
        *var_pdiblc_i_dn12_slot = var_pdiblc_i_dn12;
        *var_pdiblc_i_dn13_slot = var_pdiblc_i_dn13;
        *var_pdiblc_i_dn14_slot = var_pdiblc_i_dn14;
        *var_pdiblc_i_dn2_slot = var_pdiblc_i_dn2;
        *var_pdiblc_i_dn3_slot = var_pdiblc_i_dn3;
        *var_pdiblc_i_dn4_slot = var_pdiblc_i_dn4;
        *var_pdiblc_i_dn5_slot = var_pdiblc_i_dn5;
        *var_pdiblc_i_dn6_slot = var_pdiblc_i_dn6;
        *var_pdiblc_i_dn7_slot = var_pdiblc_i_dn7;
        *var_pdiblc_i_dn8_slot = var_pdiblc_i_dn8;
        *var_pdiblc_i_dn9_slot = var_pdiblc_i_dn9;
        *var_pdiblc_i_rv_slot = var_pdiblc_i_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn13_slot = var_t0_dn13;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_u0_i_slot = var_u0_i;
        *var_u0_i_rv_slot = var_u0_i_rv;
        *var_u0r_i_slot = var_u0r_i;
        *var_u0r_i_rv_slot = var_u0r_i_rv;
        *var_ua_i_slot = var_ua_i;
        *var_ua_i_dn0_slot = var_ua_i_dn0;
        *var_ua_i_dn10_slot = var_ua_i_dn10;
        *var_ua_i_dn11_slot = var_ua_i_dn11;
        *var_ua_i_dn12_slot = var_ua_i_dn12;
        *var_ua_i_dn13_slot = var_ua_i_dn13;
        *var_ua_i_dn14_slot = var_ua_i_dn14;
        *var_ua_i_dn2_slot = var_ua_i_dn2;
        *var_ua_i_dn3_slot = var_ua_i_dn3;
        *var_ua_i_dn4_slot = var_ua_i_dn4;
        *var_ua_i_dn5_slot = var_ua_i_dn5;
        *var_ua_i_dn6_slot = var_ua_i_dn6;
        *var_ua_i_dn7_slot = var_ua_i_dn7;
        *var_ua_i_dn8_slot = var_ua_i_dn8;
        *var_ua_i_dn9_slot = var_ua_i_dn9;
        *var_ua_i_rv_slot = var_ua_i_rv;
        *var_uar_i_slot = var_uar_i;
        *var_uar_i_dn0_slot = var_uar_i_dn0;
        *var_uar_i_dn10_slot = var_uar_i_dn10;
        *var_uar_i_dn11_slot = var_uar_i_dn11;
        *var_uar_i_dn12_slot = var_uar_i_dn12;
        *var_uar_i_dn13_slot = var_uar_i_dn13;
        *var_uar_i_dn14_slot = var_uar_i_dn14;
        *var_uar_i_dn2_slot = var_uar_i_dn2;
        *var_uar_i_dn3_slot = var_uar_i_dn3;
        *var_uar_i_dn4_slot = var_uar_i_dn4;
        *var_uar_i_dn5_slot = var_uar_i_dn5;
        *var_uar_i_dn6_slot = var_uar_i_dn6;
        *var_uar_i_dn7_slot = var_uar_i_dn7;
        *var_uar_i_dn8_slot = var_uar_i_dn8;
        *var_uar_i_dn9_slot = var_uar_i_dn9;
        *var_uar_i_rv_slot = var_uar_i_rv;
        *var_uc_i_slot = var_uc_i;
        *var_uc_i_dn0_slot = var_uc_i_dn0;
        *var_uc_i_dn10_slot = var_uc_i_dn10;
        *var_uc_i_dn11_slot = var_uc_i_dn11;
        *var_uc_i_dn12_slot = var_uc_i_dn12;
        *var_uc_i_dn13_slot = var_uc_i_dn13;
        *var_uc_i_dn14_slot = var_uc_i_dn14;
        *var_uc_i_dn2_slot = var_uc_i_dn2;
        *var_uc_i_dn3_slot = var_uc_i_dn3;
        *var_uc_i_dn4_slot = var_uc_i_dn4;
        *var_uc_i_dn5_slot = var_uc_i_dn5;
        *var_uc_i_dn6_slot = var_uc_i_dn6;
        *var_uc_i_dn7_slot = var_uc_i_dn7;
        *var_uc_i_dn8_slot = var_uc_i_dn8;
        *var_uc_i_dn9_slot = var_uc_i_dn9;
        *var_uc_i_rv_slot = var_uc_i_rv;
        *var_ucr_i_slot = var_ucr_i;
        *var_ucr_i_dn0_slot = var_ucr_i_dn0;
        *var_ucr_i_dn10_slot = var_ucr_i_dn10;
        *var_ucr_i_dn11_slot = var_ucr_i_dn11;
        *var_ucr_i_dn12_slot = var_ucr_i_dn12;
        *var_ucr_i_dn13_slot = var_ucr_i_dn13;
        *var_ucr_i_dn14_slot = var_ucr_i_dn14;
        *var_ucr_i_dn2_slot = var_ucr_i_dn2;
        *var_ucr_i_dn3_slot = var_ucr_i_dn3;
        *var_ucr_i_dn4_slot = var_ucr_i_dn4;
        *var_ucr_i_dn5_slot = var_ucr_i_dn5;
        *var_ucr_i_dn6_slot = var_ucr_i_dn6;
        *var_ucr_i_dn7_slot = var_ucr_i_dn7;
        *var_ucr_i_dn8_slot = var_ucr_i_dn8;
        *var_ucr_i_dn9_slot = var_ucr_i_dn9;
        *var_ucr_i_rv_slot = var_ucr_i_rv;
        *var_ud_i_slot = var_ud_i;
        *var_ud_i_dn0_slot = var_ud_i_dn0;
        *var_ud_i_dn10_slot = var_ud_i_dn10;
        *var_ud_i_dn11_slot = var_ud_i_dn11;
        *var_ud_i_dn12_slot = var_ud_i_dn12;
        *var_ud_i_dn13_slot = var_ud_i_dn13;
        *var_ud_i_dn14_slot = var_ud_i_dn14;
        *var_ud_i_dn2_slot = var_ud_i_dn2;
        *var_ud_i_dn3_slot = var_ud_i_dn3;
        *var_ud_i_dn4_slot = var_ud_i_dn4;
        *var_ud_i_dn5_slot = var_ud_i_dn5;
        *var_ud_i_dn6_slot = var_ud_i_dn6;
        *var_ud_i_dn7_slot = var_ud_i_dn7;
        *var_ud_i_dn8_slot = var_ud_i_dn8;
        *var_ud_i_dn9_slot = var_ud_i_dn9;
        *var_ud_i_rv_slot = var_ud_i_rv;
        *var_udr_i_slot = var_udr_i;
        *var_udr_i_dn0_slot = var_udr_i_dn0;
        *var_udr_i_dn10_slot = var_udr_i_dn10;
        *var_udr_i_dn11_slot = var_udr_i_dn11;
        *var_udr_i_dn12_slot = var_udr_i_dn12;
        *var_udr_i_dn13_slot = var_udr_i_dn13;
        *var_udr_i_dn14_slot = var_udr_i_dn14;
        *var_udr_i_dn2_slot = var_udr_i_dn2;
        *var_udr_i_dn3_slot = var_udr_i_dn3;
        *var_udr_i_dn4_slot = var_udr_i_dn4;
        *var_udr_i_dn5_slot = var_udr_i_dn5;
        *var_udr_i_dn6_slot = var_udr_i_dn6;
        *var_udr_i_dn7_slot = var_udr_i_dn7;
        *var_udr_i_dn8_slot = var_udr_i_dn8;
        *var_udr_i_dn9_slot = var_udr_i_dn9;
        *var_udr_i_rv_slot = var_udr_i_rv;
    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        var_guard32: f64,
        var_inv_l: f64,
        var_inv_llong: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_inv_wwide: f64,
        var_alpha0_i_slot: &mut f64,
        var_alpha0_i_dn0_slot: &mut f64,
        var_alpha0_i_dn10_slot: &mut f64,
        var_alpha0_i_dn11_slot: &mut f64,
        var_alpha0_i_dn12_slot: &mut f64,
        var_alpha0_i_dn13_slot: &mut f64,
        var_alpha0_i_dn14_slot: &mut f64,
        var_alpha0_i_dn2_slot: &mut f64,
        var_alpha0_i_dn3_slot: &mut f64,
        var_alpha0_i_dn4_slot: &mut f64,
        var_alpha0_i_dn5_slot: &mut f64,
        var_alpha0_i_dn6_slot: &mut f64,
        var_alpha0_i_dn7_slot: &mut f64,
        var_alpha0_i_dn8_slot: &mut f64,
        var_alpha0_i_dn9_slot: &mut f64,
        var_alpha0_i_rv_slot: &mut f64,
        var_alpha0r_i_slot: &mut f64,
        var_alpha0r_i_dn0_slot: &mut f64,
        var_alpha0r_i_dn10_slot: &mut f64,
        var_alpha0r_i_dn11_slot: &mut f64,
        var_alpha0r_i_dn12_slot: &mut f64,
        var_alpha0r_i_dn13_slot: &mut f64,
        var_alpha0r_i_dn14_slot: &mut f64,
        var_alpha0r_i_dn2_slot: &mut f64,
        var_alpha0r_i_dn3_slot: &mut f64,
        var_alpha0r_i_dn4_slot: &mut f64,
        var_alpha0r_i_dn5_slot: &mut f64,
        var_alpha0r_i_dn6_slot: &mut f64,
        var_alpha0r_i_dn7_slot: &mut f64,
        var_alpha0r_i_dn8_slot: &mut f64,
        var_alpha0r_i_dn9_slot: &mut f64,
        var_alpha0r_i_rv_slot: &mut f64,
        var_delta_i_slot: &mut f64,
        var_delta_i_dn0_slot: &mut f64,
        var_delta_i_dn10_slot: &mut f64,
        var_delta_i_dn11_slot: &mut f64,
        var_delta_i_dn12_slot: &mut f64,
        var_delta_i_dn13_slot: &mut f64,
        var_delta_i_dn14_slot: &mut f64,
        var_delta_i_dn2_slot: &mut f64,
        var_delta_i_dn3_slot: &mut f64,
        var_delta_i_dn4_slot: &mut f64,
        var_delta_i_dn5_slot: &mut f64,
        var_delta_i_dn6_slot: &mut f64,
        var_delta_i_dn7_slot: &mut f64,
        var_delta_i_dn8_slot: &mut f64,
        var_delta_i_dn9_slot: &mut f64,
        var_delta_i_rv_slot: &mut f64,
        var_fprout_i_slot: &mut f64,
        var_fprout_i_rv_slot: &mut f64,
        var_guard33_slot: &mut f64,
        var_guard33_rv_slot: &mut f64,
        var_guard34_slot: &mut f64,
        var_guard34_rv_slot: &mut f64,
        var_guard35_slot: &mut f64,
        var_guard35_rv_slot: &mut f64,
        var_guard36_slot: &mut f64,
        var_guard36_rv_slot: &mut f64,
        var_guard37_slot: &mut f64,
        var_guard37_rv_slot: &mut f64,
        var_pclm_i_slot: &mut f64,
        var_pclm_i_dn0_slot: &mut f64,
        var_pclm_i_dn10_slot: &mut f64,
        var_pclm_i_dn11_slot: &mut f64,
        var_pclm_i_dn12_slot: &mut f64,
        var_pclm_i_dn13_slot: &mut f64,
        var_pclm_i_dn14_slot: &mut f64,
        var_pclm_i_dn2_slot: &mut f64,
        var_pclm_i_dn3_slot: &mut f64,
        var_pclm_i_dn4_slot: &mut f64,
        var_pclm_i_dn5_slot: &mut f64,
        var_pclm_i_dn6_slot: &mut f64,
        var_pclm_i_dn7_slot: &mut f64,
        var_pclm_i_dn8_slot: &mut f64,
        var_pclm_i_dn9_slot: &mut f64,
        var_pclm_i_rv_slot: &mut f64,
        var_pclmr_i_slot: &mut f64,
        var_pclmr_i_dn0_slot: &mut f64,
        var_pclmr_i_dn10_slot: &mut f64,
        var_pclmr_i_dn11_slot: &mut f64,
        var_pclmr_i_dn12_slot: &mut f64,
        var_pclmr_i_dn13_slot: &mut f64,
        var_pclmr_i_dn14_slot: &mut f64,
        var_pclmr_i_dn2_slot: &mut f64,
        var_pclmr_i_dn3_slot: &mut f64,
        var_pclmr_i_dn4_slot: &mut f64,
        var_pclmr_i_dn5_slot: &mut f64,
        var_pclmr_i_dn6_slot: &mut f64,
        var_pclmr_i_dn7_slot: &mut f64,
        var_pclmr_i_dn8_slot: &mut f64,
        var_pclmr_i_dn9_slot: &mut f64,
        var_pclmr_i_rv_slot: &mut f64,
        var_pdiblcr_i_slot: &mut f64,
        var_pdiblcr_i_dn0_slot: &mut f64,
        var_pdiblcr_i_dn10_slot: &mut f64,
        var_pdiblcr_i_dn11_slot: &mut f64,
        var_pdiblcr_i_dn12_slot: &mut f64,
        var_pdiblcr_i_dn13_slot: &mut f64,
        var_pdiblcr_i_dn14_slot: &mut f64,
        var_pdiblcr_i_dn2_slot: &mut f64,
        var_pdiblcr_i_dn3_slot: &mut f64,
        var_pdiblcr_i_dn4_slot: &mut f64,
        var_pdiblcr_i_dn5_slot: &mut f64,
        var_pdiblcr_i_dn6_slot: &mut f64,
        var_pdiblcr_i_dn7_slot: &mut f64,
        var_pdiblcr_i_dn8_slot: &mut f64,
        var_pdiblcr_i_dn9_slot: &mut f64,
        var_pdiblcr_i_rv_slot: &mut f64,
        var_psat_i_slot: &mut f64,
        var_psat_i_rv_slot: &mut f64,
        var_psatr_i_slot: &mut f64,
        var_psatr_i_rv_slot: &mut f64,
        var_ptwg_i_slot: &mut f64,
        var_ptwg_i_dn0_slot: &mut f64,
        var_ptwg_i_dn10_slot: &mut f64,
        var_ptwg_i_dn11_slot: &mut f64,
        var_ptwg_i_dn12_slot: &mut f64,
        var_ptwg_i_dn13_slot: &mut f64,
        var_ptwg_i_dn14_slot: &mut f64,
        var_ptwg_i_dn2_slot: &mut f64,
        var_ptwg_i_dn3_slot: &mut f64,
        var_ptwg_i_dn4_slot: &mut f64,
        var_ptwg_i_dn5_slot: &mut f64,
        var_ptwg_i_dn6_slot: &mut f64,
        var_ptwg_i_dn7_slot: &mut f64,
        var_ptwg_i_dn8_slot: &mut f64,
        var_ptwg_i_dn9_slot: &mut f64,
        var_ptwg_i_rv_slot: &mut f64,
        var_ptwgr_i_slot: &mut f64,
        var_ptwgr_i_dn0_slot: &mut f64,
        var_ptwgr_i_dn10_slot: &mut f64,
        var_ptwgr_i_dn11_slot: &mut f64,
        var_ptwgr_i_dn12_slot: &mut f64,
        var_ptwgr_i_dn13_slot: &mut f64,
        var_ptwgr_i_dn14_slot: &mut f64,
        var_ptwgr_i_dn2_slot: &mut f64,
        var_ptwgr_i_dn3_slot: &mut f64,
        var_ptwgr_i_dn4_slot: &mut f64,
        var_ptwgr_i_dn5_slot: &mut f64,
        var_ptwgr_i_dn6_slot: &mut f64,
        var_ptwgr_i_dn7_slot: &mut f64,
        var_ptwgr_i_dn8_slot: &mut f64,
        var_ptwgr_i_dn9_slot: &mut f64,
        var_ptwgr_i_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_vsat_i_slot: &mut f64,
        var_vsat_i_dn0_slot: &mut f64,
        var_vsat_i_dn10_slot: &mut f64,
        var_vsat_i_dn11_slot: &mut f64,
        var_vsat_i_dn12_slot: &mut f64,
        var_vsat_i_dn13_slot: &mut f64,
        var_vsat_i_dn14_slot: &mut f64,
        var_vsat_i_dn2_slot: &mut f64,
        var_vsat_i_dn3_slot: &mut f64,
        var_vsat_i_dn4_slot: &mut f64,
        var_vsat_i_dn5_slot: &mut f64,
        var_vsat_i_dn6_slot: &mut f64,
        var_vsat_i_dn7_slot: &mut f64,
        var_vsat_i_dn8_slot: &mut f64,
        var_vsat_i_dn9_slot: &mut f64,
        var_vsat_i_rv_slot: &mut f64,
        var_vsatr_i_slot: &mut f64,
        var_vsatr_i_dn0_slot: &mut f64,
        var_vsatr_i_dn10_slot: &mut f64,
        var_vsatr_i_dn11_slot: &mut f64,
        var_vsatr_i_dn12_slot: &mut f64,
        var_vsatr_i_dn13_slot: &mut f64,
        var_vsatr_i_dn14_slot: &mut f64,
        var_vsatr_i_dn2_slot: &mut f64,
        var_vsatr_i_dn3_slot: &mut f64,
        var_vsatr_i_dn4_slot: &mut f64,
        var_vsatr_i_dn5_slot: &mut f64,
        var_vsatr_i_dn6_slot: &mut f64,
        var_vsatr_i_dn7_slot: &mut f64,
        var_vsatr_i_dn8_slot: &mut f64,
        var_vsatr_i_dn9_slot: &mut f64,
        var_vsatr_i_rv_slot: &mut f64,
    ) {
        let mut var_alpha0_i: f64 = *var_alpha0_i_slot;
        let mut var_alpha0_i_dn0: f64 = *var_alpha0_i_dn0_slot;
        let mut var_alpha0_i_dn10: f64 = *var_alpha0_i_dn10_slot;
        let mut var_alpha0_i_dn11: f64 = *var_alpha0_i_dn11_slot;
        let mut var_alpha0_i_dn12: f64 = *var_alpha0_i_dn12_slot;
        let mut var_alpha0_i_dn13: f64 = *var_alpha0_i_dn13_slot;
        let mut var_alpha0_i_dn14: f64 = *var_alpha0_i_dn14_slot;
        let mut var_alpha0_i_dn2: f64 = *var_alpha0_i_dn2_slot;
        let mut var_alpha0_i_dn3: f64 = *var_alpha0_i_dn3_slot;
        let mut var_alpha0_i_dn4: f64 = *var_alpha0_i_dn4_slot;
        let mut var_alpha0_i_dn5: f64 = *var_alpha0_i_dn5_slot;
        let mut var_alpha0_i_dn6: f64 = *var_alpha0_i_dn6_slot;
        let mut var_alpha0_i_dn7: f64 = *var_alpha0_i_dn7_slot;
        let mut var_alpha0_i_dn8: f64 = *var_alpha0_i_dn8_slot;
        let mut var_alpha0_i_dn9: f64 = *var_alpha0_i_dn9_slot;
        let mut var_alpha0_i_rv: f64 = *var_alpha0_i_rv_slot;
        let mut var_alpha0r_i: f64 = *var_alpha0r_i_slot;
        let mut var_alpha0r_i_dn0: f64 = *var_alpha0r_i_dn0_slot;
        let mut var_alpha0r_i_dn10: f64 = *var_alpha0r_i_dn10_slot;
        let mut var_alpha0r_i_dn11: f64 = *var_alpha0r_i_dn11_slot;
        let mut var_alpha0r_i_dn12: f64 = *var_alpha0r_i_dn12_slot;
        let mut var_alpha0r_i_dn13: f64 = *var_alpha0r_i_dn13_slot;
        let mut var_alpha0r_i_dn14: f64 = *var_alpha0r_i_dn14_slot;
        let mut var_alpha0r_i_dn2: f64 = *var_alpha0r_i_dn2_slot;
        let mut var_alpha0r_i_dn3: f64 = *var_alpha0r_i_dn3_slot;
        let mut var_alpha0r_i_dn4: f64 = *var_alpha0r_i_dn4_slot;
        let mut var_alpha0r_i_dn5: f64 = *var_alpha0r_i_dn5_slot;
        let mut var_alpha0r_i_dn6: f64 = *var_alpha0r_i_dn6_slot;
        let mut var_alpha0r_i_dn7: f64 = *var_alpha0r_i_dn7_slot;
        let mut var_alpha0r_i_dn8: f64 = *var_alpha0r_i_dn8_slot;
        let mut var_alpha0r_i_dn9: f64 = *var_alpha0r_i_dn9_slot;
        let mut var_alpha0r_i_rv: f64 = *var_alpha0r_i_rv_slot;
        let mut var_delta_i: f64 = *var_delta_i_slot;
        let mut var_delta_i_dn0: f64 = *var_delta_i_dn0_slot;
        let mut var_delta_i_dn10: f64 = *var_delta_i_dn10_slot;
        let mut var_delta_i_dn11: f64 = *var_delta_i_dn11_slot;
        let mut var_delta_i_dn12: f64 = *var_delta_i_dn12_slot;
        let mut var_delta_i_dn13: f64 = *var_delta_i_dn13_slot;
        let mut var_delta_i_dn14: f64 = *var_delta_i_dn14_slot;
        let mut var_delta_i_dn2: f64 = *var_delta_i_dn2_slot;
        let mut var_delta_i_dn3: f64 = *var_delta_i_dn3_slot;
        let mut var_delta_i_dn4: f64 = *var_delta_i_dn4_slot;
        let mut var_delta_i_dn5: f64 = *var_delta_i_dn5_slot;
        let mut var_delta_i_dn6: f64 = *var_delta_i_dn6_slot;
        let mut var_delta_i_dn7: f64 = *var_delta_i_dn7_slot;
        let mut var_delta_i_dn8: f64 = *var_delta_i_dn8_slot;
        let mut var_delta_i_dn9: f64 = *var_delta_i_dn9_slot;
        let mut var_delta_i_rv: f64 = *var_delta_i_rv_slot;
        let mut var_fprout_i: f64 = *var_fprout_i_slot;
        let mut var_fprout_i_rv: f64 = *var_fprout_i_rv_slot;
        let mut var_guard33: f64 = *var_guard33_slot;
        let mut var_guard33_rv: f64 = *var_guard33_rv_slot;
        let mut var_guard34: f64 = *var_guard34_slot;
        let mut var_guard34_rv: f64 = *var_guard34_rv_slot;
        let mut var_guard35: f64 = *var_guard35_slot;
        let mut var_guard35_rv: f64 = *var_guard35_rv_slot;
        let mut var_guard36: f64 = *var_guard36_slot;
        let mut var_guard36_rv: f64 = *var_guard36_rv_slot;
        let mut var_guard37: f64 = *var_guard37_slot;
        let mut var_guard37_rv: f64 = *var_guard37_rv_slot;
        let mut var_pclm_i: f64 = *var_pclm_i_slot;
        let mut var_pclm_i_dn0: f64 = *var_pclm_i_dn0_slot;
        let mut var_pclm_i_dn10: f64 = *var_pclm_i_dn10_slot;
        let mut var_pclm_i_dn11: f64 = *var_pclm_i_dn11_slot;
        let mut var_pclm_i_dn12: f64 = *var_pclm_i_dn12_slot;
        let mut var_pclm_i_dn13: f64 = *var_pclm_i_dn13_slot;
        let mut var_pclm_i_dn14: f64 = *var_pclm_i_dn14_slot;
        let mut var_pclm_i_dn2: f64 = *var_pclm_i_dn2_slot;
        let mut var_pclm_i_dn3: f64 = *var_pclm_i_dn3_slot;
        let mut var_pclm_i_dn4: f64 = *var_pclm_i_dn4_slot;
        let mut var_pclm_i_dn5: f64 = *var_pclm_i_dn5_slot;
        let mut var_pclm_i_dn6: f64 = *var_pclm_i_dn6_slot;
        let mut var_pclm_i_dn7: f64 = *var_pclm_i_dn7_slot;
        let mut var_pclm_i_dn8: f64 = *var_pclm_i_dn8_slot;
        let mut var_pclm_i_dn9: f64 = *var_pclm_i_dn9_slot;
        let mut var_pclm_i_rv: f64 = *var_pclm_i_rv_slot;
        let mut var_pclmr_i: f64 = *var_pclmr_i_slot;
        let mut var_pclmr_i_dn0: f64 = *var_pclmr_i_dn0_slot;
        let mut var_pclmr_i_dn10: f64 = *var_pclmr_i_dn10_slot;
        let mut var_pclmr_i_dn11: f64 = *var_pclmr_i_dn11_slot;
        let mut var_pclmr_i_dn12: f64 = *var_pclmr_i_dn12_slot;
        let mut var_pclmr_i_dn13: f64 = *var_pclmr_i_dn13_slot;
        let mut var_pclmr_i_dn14: f64 = *var_pclmr_i_dn14_slot;
        let mut var_pclmr_i_dn2: f64 = *var_pclmr_i_dn2_slot;
        let mut var_pclmr_i_dn3: f64 = *var_pclmr_i_dn3_slot;
        let mut var_pclmr_i_dn4: f64 = *var_pclmr_i_dn4_slot;
        let mut var_pclmr_i_dn5: f64 = *var_pclmr_i_dn5_slot;
        let mut var_pclmr_i_dn6: f64 = *var_pclmr_i_dn6_slot;
        let mut var_pclmr_i_dn7: f64 = *var_pclmr_i_dn7_slot;
        let mut var_pclmr_i_dn8: f64 = *var_pclmr_i_dn8_slot;
        let mut var_pclmr_i_dn9: f64 = *var_pclmr_i_dn9_slot;
        let mut var_pclmr_i_rv: f64 = *var_pclmr_i_rv_slot;
        let mut var_pdiblcr_i: f64 = *var_pdiblcr_i_slot;
        let mut var_pdiblcr_i_dn0: f64 = *var_pdiblcr_i_dn0_slot;
        let mut var_pdiblcr_i_dn10: f64 = *var_pdiblcr_i_dn10_slot;
        let mut var_pdiblcr_i_dn11: f64 = *var_pdiblcr_i_dn11_slot;
        let mut var_pdiblcr_i_dn12: f64 = *var_pdiblcr_i_dn12_slot;
        let mut var_pdiblcr_i_dn13: f64 = *var_pdiblcr_i_dn13_slot;
        let mut var_pdiblcr_i_dn14: f64 = *var_pdiblcr_i_dn14_slot;
        let mut var_pdiblcr_i_dn2: f64 = *var_pdiblcr_i_dn2_slot;
        let mut var_pdiblcr_i_dn3: f64 = *var_pdiblcr_i_dn3_slot;
        let mut var_pdiblcr_i_dn4: f64 = *var_pdiblcr_i_dn4_slot;
        let mut var_pdiblcr_i_dn5: f64 = *var_pdiblcr_i_dn5_slot;
        let mut var_pdiblcr_i_dn6: f64 = *var_pdiblcr_i_dn6_slot;
        let mut var_pdiblcr_i_dn7: f64 = *var_pdiblcr_i_dn7_slot;
        let mut var_pdiblcr_i_dn8: f64 = *var_pdiblcr_i_dn8_slot;
        let mut var_pdiblcr_i_dn9: f64 = *var_pdiblcr_i_dn9_slot;
        let mut var_pdiblcr_i_rv: f64 = *var_pdiblcr_i_rv_slot;
        let mut var_psat_i: f64 = *var_psat_i_slot;
        let mut var_psat_i_rv: f64 = *var_psat_i_rv_slot;
        let mut var_psatr_i: f64 = *var_psatr_i_slot;
        let mut var_psatr_i_rv: f64 = *var_psatr_i_rv_slot;
        let mut var_ptwg_i: f64 = *var_ptwg_i_slot;
        let mut var_ptwg_i_dn0: f64 = *var_ptwg_i_dn0_slot;
        let mut var_ptwg_i_dn10: f64 = *var_ptwg_i_dn10_slot;
        let mut var_ptwg_i_dn11: f64 = *var_ptwg_i_dn11_slot;
        let mut var_ptwg_i_dn12: f64 = *var_ptwg_i_dn12_slot;
        let mut var_ptwg_i_dn13: f64 = *var_ptwg_i_dn13_slot;
        let mut var_ptwg_i_dn14: f64 = *var_ptwg_i_dn14_slot;
        let mut var_ptwg_i_dn2: f64 = *var_ptwg_i_dn2_slot;
        let mut var_ptwg_i_dn3: f64 = *var_ptwg_i_dn3_slot;
        let mut var_ptwg_i_dn4: f64 = *var_ptwg_i_dn4_slot;
        let mut var_ptwg_i_dn5: f64 = *var_ptwg_i_dn5_slot;
        let mut var_ptwg_i_dn6: f64 = *var_ptwg_i_dn6_slot;
        let mut var_ptwg_i_dn7: f64 = *var_ptwg_i_dn7_slot;
        let mut var_ptwg_i_dn8: f64 = *var_ptwg_i_dn8_slot;
        let mut var_ptwg_i_dn9: f64 = *var_ptwg_i_dn9_slot;
        let mut var_ptwg_i_rv: f64 = *var_ptwg_i_rv_slot;
        let mut var_ptwgr_i: f64 = *var_ptwgr_i_slot;
        let mut var_ptwgr_i_dn0: f64 = *var_ptwgr_i_dn0_slot;
        let mut var_ptwgr_i_dn10: f64 = *var_ptwgr_i_dn10_slot;
        let mut var_ptwgr_i_dn11: f64 = *var_ptwgr_i_dn11_slot;
        let mut var_ptwgr_i_dn12: f64 = *var_ptwgr_i_dn12_slot;
        let mut var_ptwgr_i_dn13: f64 = *var_ptwgr_i_dn13_slot;
        let mut var_ptwgr_i_dn14: f64 = *var_ptwgr_i_dn14_slot;
        let mut var_ptwgr_i_dn2: f64 = *var_ptwgr_i_dn2_slot;
        let mut var_ptwgr_i_dn3: f64 = *var_ptwgr_i_dn3_slot;
        let mut var_ptwgr_i_dn4: f64 = *var_ptwgr_i_dn4_slot;
        let mut var_ptwgr_i_dn5: f64 = *var_ptwgr_i_dn5_slot;
        let mut var_ptwgr_i_dn6: f64 = *var_ptwgr_i_dn6_slot;
        let mut var_ptwgr_i_dn7: f64 = *var_ptwgr_i_dn7_slot;
        let mut var_ptwgr_i_dn8: f64 = *var_ptwgr_i_dn8_slot;
        let mut var_ptwgr_i_dn9: f64 = *var_ptwgr_i_dn9_slot;
        let mut var_ptwgr_i_rv: f64 = *var_ptwgr_i_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_vsat_i: f64 = *var_vsat_i_slot;
        let mut var_vsat_i_dn0: f64 = *var_vsat_i_dn0_slot;
        let mut var_vsat_i_dn10: f64 = *var_vsat_i_dn10_slot;
        let mut var_vsat_i_dn11: f64 = *var_vsat_i_dn11_slot;
        let mut var_vsat_i_dn12: f64 = *var_vsat_i_dn12_slot;
        let mut var_vsat_i_dn13: f64 = *var_vsat_i_dn13_slot;
        let mut var_vsat_i_dn14: f64 = *var_vsat_i_dn14_slot;
        let mut var_vsat_i_dn2: f64 = *var_vsat_i_dn2_slot;
        let mut var_vsat_i_dn3: f64 = *var_vsat_i_dn3_slot;
        let mut var_vsat_i_dn4: f64 = *var_vsat_i_dn4_slot;
        let mut var_vsat_i_dn5: f64 = *var_vsat_i_dn5_slot;
        let mut var_vsat_i_dn6: f64 = *var_vsat_i_dn6_slot;
        let mut var_vsat_i_dn7: f64 = *var_vsat_i_dn7_slot;
        let mut var_vsat_i_dn8: f64 = *var_vsat_i_dn8_slot;
        let mut var_vsat_i_dn9: f64 = *var_vsat_i_dn9_slot;
        let mut var_vsat_i_rv: f64 = *var_vsat_i_rv_slot;
        let mut var_vsatr_i: f64 = *var_vsatr_i_slot;
        let mut var_vsatr_i_dn0: f64 = *var_vsatr_i_dn0_slot;
        let mut var_vsatr_i_dn10: f64 = *var_vsatr_i_dn10_slot;
        let mut var_vsatr_i_dn11: f64 = *var_vsatr_i_dn11_slot;
        let mut var_vsatr_i_dn12: f64 = *var_vsatr_i_dn12_slot;
        let mut var_vsatr_i_dn13: f64 = *var_vsatr_i_dn13_slot;
        let mut var_vsatr_i_dn14: f64 = *var_vsatr_i_dn14_slot;
        let mut var_vsatr_i_dn2: f64 = *var_vsatr_i_dn2_slot;
        let mut var_vsatr_i_dn3: f64 = *var_vsatr_i_dn3_slot;
        let mut var_vsatr_i_dn4: f64 = *var_vsatr_i_dn4_slot;
        let mut var_vsatr_i_dn5: f64 = *var_vsatr_i_dn5_slot;
        let mut var_vsatr_i_dn6: f64 = *var_vsatr_i_dn6_slot;
        let mut var_vsatr_i_dn7: f64 = *var_vsatr_i_dn7_slot;
        let mut var_vsatr_i_dn8: f64 = *var_vsatr_i_dn8_slot;
        let mut var_vsatr_i_dn9: f64 = *var_vsatr_i_dn9_slot;
        let mut var_vsatr_i_rv: f64 = *var_vsatr_i_rv_slot;

        let (assign3750_e5030, assign3750_e5030_d_n0, assign3750_e5030_d_n2, assign3750_e5030_d_n3, assign3750_e5030_d_n4, assign3750_e5030_d_n5, assign3750_e5030_d_n6, assign3750_e5030_d_n7, assign3750_e5030_d_n8, assign3750_e5030_d_n9, assign3750_e5030_d_n10, assign3750_e5030_d_n11, assign3750_e5030_d_n12, assign3750_e5030_d_n13, assign3750_e5030_d_n14,) = {
    if (var_guard32 != 0.0) {
        let assign3750_e5028: f64 = (var_pdiblcr_i * var_t0);
        (assign3750_e5028, ((var_pdiblcr_i_dn0 * var_t0) + (var_pdiblcr_i * var_t0_dn0)), ((var_pdiblcr_i_dn2 * var_t0) + (var_pdiblcr_i * var_t0_dn2)), ((var_pdiblcr_i_dn3 * var_t0) + (var_pdiblcr_i * var_t0_dn3)), ((var_pdiblcr_i_dn4 * var_t0) + (var_pdiblcr_i * var_t0_dn4)), ((var_pdiblcr_i_dn5 * var_t0) + (var_pdiblcr_i * var_t0_dn5)), ((var_pdiblcr_i_dn6 * var_t0) + (var_pdiblcr_i * var_t0_dn6)), ((var_pdiblcr_i_dn7 * var_t0) + (var_pdiblcr_i * var_t0_dn7)), ((var_pdiblcr_i_dn8 * var_t0) + (var_pdiblcr_i * var_t0_dn8)), ((var_pdiblcr_i_dn9 * var_t0) + (var_pdiblcr_i * var_t0_dn9)), ((var_pdiblcr_i_dn10 * var_t0) + (var_pdiblcr_i * var_t0_dn10)), ((var_pdiblcr_i_dn11 * var_t0) + (var_pdiblcr_i * var_t0_dn11)), ((var_pdiblcr_i_dn12 * var_t0) + (var_pdiblcr_i * var_t0_dn12)), ((var_pdiblcr_i_dn13 * var_t0) + (var_pdiblcr_i * var_t0_dn13)), ((var_pdiblcr_i_dn14 * var_t0) + (var_pdiblcr_i * var_t0_dn14)),)
    } else {
        (var_pdiblcr_i, var_pdiblcr_i_dn0, var_pdiblcr_i_dn2, var_pdiblcr_i_dn3, var_pdiblcr_i_dn4, var_pdiblcr_i_dn5, var_pdiblcr_i_dn6, var_pdiblcr_i_dn7, var_pdiblcr_i_dn8, var_pdiblcr_i_dn9, var_pdiblcr_i_dn10, var_pdiblcr_i_dn11, var_pdiblcr_i_dn12, var_pdiblcr_i_dn13, var_pdiblcr_i_dn14,)
    }
};
        var_pdiblcr_i = assign3750_e5030;
        var_pdiblcr_i_dn0 = assign3750_e5030_d_n0;
        var_pdiblcr_i_dn2 = assign3750_e5030_d_n2;
        var_pdiblcr_i_dn3 = assign3750_e5030_d_n3;
        var_pdiblcr_i_dn4 = assign3750_e5030_d_n4;
        var_pdiblcr_i_dn5 = assign3750_e5030_d_n5;
        var_pdiblcr_i_dn6 = assign3750_e5030_d_n6;
        var_pdiblcr_i_dn7 = assign3750_e5030_d_n7;
        var_pdiblcr_i_dn8 = assign3750_e5030_d_n8;
        var_pdiblcr_i_dn9 = assign3750_e5030_d_n9;
        var_pdiblcr_i_dn10 = assign3750_e5030_d_n10;
        var_pdiblcr_i_dn11 = assign3750_e5030_d_n11;
        var_pdiblcr_i_dn12 = assign3750_e5030_d_n12;
        var_pdiblcr_i_dn13 = assign3750_e5030_d_n13;
        var_pdiblcr_i_dn14 = assign3750_e5030_d_n14;
        var_pdiblcr_i_rv = 0.0;

        let assign3760_e5036: f64 = (var_inv_l).powf(p.p258);
        let assign3760_e5039: f64 = (var_inv_llong).powf(p.p258);
        let assign3760_e5040: f64 = (assign3760_e5036 - assign3760_e5039);
        let assign3760_e5042: f64 = (assign3760_e5040).max(0.0);
        let assign3760_e5043: f64 = (p.p257 * assign3760_e5042);
        let assign3760_e5044: f64 = (1.0 + assign3760_e5043);
        let assign3760_e5045: f64 = (var_delta_i * assign3760_e5044);
        var_t0 = assign3760_e5045;
        var_t0_dn0 = (var_delta_i_dn0 * assign3760_e5044);
        var_t0_dn2 = (var_delta_i_dn2 * assign3760_e5044);
        var_t0_dn3 = (var_delta_i_dn3 * assign3760_e5044);
        var_t0_dn4 = (var_delta_i_dn4 * assign3760_e5044);
        var_t0_dn5 = (var_delta_i_dn5 * assign3760_e5044);
        var_t0_dn6 = (var_delta_i_dn6 * assign3760_e5044);
        var_t0_dn7 = (var_delta_i_dn7 * assign3760_e5044);
        var_t0_dn8 = (var_delta_i_dn8 * assign3760_e5044);
        var_t0_dn9 = (var_delta_i_dn9 * assign3760_e5044);
        var_t0_dn10 = (var_delta_i_dn10 * assign3760_e5044);
        var_t0_dn11 = (var_delta_i_dn11 * assign3760_e5044);
        var_t0_dn12 = (var_delta_i_dn12 * assign3760_e5044);
        var_t0_dn13 = (var_delta_i_dn13 * assign3760_e5044);
        var_t0_dn14 = (var_delta_i_dn14 * assign3760_e5044);
        var_t0_rv = 0.0;

        let assign3770_e5048: f64 = (var_t0).min(0.5);
        var_delta_i = assign3770_e5048;
        var_delta_i_dn0 = if var_t0 <= 0.5 { var_t0_dn0 } else { 0.0 };
        var_delta_i_dn2 = if var_t0 <= 0.5 { var_t0_dn2 } else { 0.0 };
        var_delta_i_dn3 = if var_t0 <= 0.5 { var_t0_dn3 } else { 0.0 };
        var_delta_i_dn4 = if var_t0 <= 0.5 { var_t0_dn4 } else { 0.0 };
        var_delta_i_dn5 = if var_t0 <= 0.5 { var_t0_dn5 } else { 0.0 };
        var_delta_i_dn6 = if var_t0 <= 0.5 { var_t0_dn6 } else { 0.0 };
        var_delta_i_dn7 = if var_t0 <= 0.5 { var_t0_dn7 } else { 0.0 };
        var_delta_i_dn8 = if var_t0 <= 0.5 { var_t0_dn8 } else { 0.0 };
        var_delta_i_dn9 = if var_t0 <= 0.5 { var_t0_dn9 } else { 0.0 };
        var_delta_i_dn10 = if var_t0 <= 0.5 { var_t0_dn10 } else { 0.0 };
        var_delta_i_dn11 = if var_t0 <= 0.5 { var_t0_dn11 } else { 0.0 };
        var_delta_i_dn12 = if var_t0 <= 0.5 { var_t0_dn12 } else { 0.0 };
        var_delta_i_dn13 = if var_t0 <= 0.5 { var_t0_dn13 } else { 0.0 };
        var_delta_i_dn14 = if var_t0 <= 0.5 { var_t0_dn14 } else { 0.0 };
        var_delta_i_rv = 0.0;

        let assign3780_e5054: f64 = (var_inv_l).powf(p.p480);
        let assign3780_e5057: f64 = (var_inv_llong).powf(p.p480);
        let assign3780_e5058: f64 = (assign3780_e5054 - assign3780_e5057);
        let assign3780_e5060: f64 = (assign3780_e5058).max(0.0);
        let assign3780_e5061: f64 = (p.p479 * assign3780_e5060);
        let assign3780_e5062: f64 = (1.0 + assign3780_e5061);
        let assign3780_e5063: f64 = (var_fprout_i * assign3780_e5062);
        var_fprout_i = assign3780_e5063;
        var_fprout_i_rv = 0.0;

        let assign3790_e5068: f64 = (var_inv_l).powf(p.p342);
        let assign3790_e5071: f64 = (var_inv_llong).powf(p.p342);
        let assign3790_e5072: f64 = (assign3790_e5068 - assign3790_e5071);
        let assign3790_e5074: f64 = (assign3790_e5072).max(0.0);
        let assign3790_e5075: f64 = (p.p341 * assign3790_e5074);
        let assign3790_e5076: f64 = (1.0 + assign3790_e5075);
        var_t0 = assign3790_e5076;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign3800_e5079: f64 = (var_pclm_i * var_t0);
        var_pclm_i = assign3800_e5079;
        var_pclm_i_dn0 = ((var_pclm_i_dn0 * var_t0) + (var_pclm_i * var_t0_dn0));
        var_pclm_i_dn2 = ((var_pclm_i_dn2 * var_t0) + (var_pclm_i * var_t0_dn2));
        var_pclm_i_dn3 = ((var_pclm_i_dn3 * var_t0) + (var_pclm_i * var_t0_dn3));
        var_pclm_i_dn4 = ((var_pclm_i_dn4 * var_t0) + (var_pclm_i * var_t0_dn4));
        var_pclm_i_dn5 = ((var_pclm_i_dn5 * var_t0) + (var_pclm_i * var_t0_dn5));
        var_pclm_i_dn6 = ((var_pclm_i_dn6 * var_t0) + (var_pclm_i * var_t0_dn6));
        var_pclm_i_dn7 = ((var_pclm_i_dn7 * var_t0) + (var_pclm_i * var_t0_dn7));
        var_pclm_i_dn8 = ((var_pclm_i_dn8 * var_t0) + (var_pclm_i * var_t0_dn8));
        var_pclm_i_dn9 = ((var_pclm_i_dn9 * var_t0) + (var_pclm_i * var_t0_dn9));
        var_pclm_i_dn10 = ((var_pclm_i_dn10 * var_t0) + (var_pclm_i * var_t0_dn10));
        var_pclm_i_dn11 = ((var_pclm_i_dn11 * var_t0) + (var_pclm_i * var_t0_dn11));
        var_pclm_i_dn12 = ((var_pclm_i_dn12 * var_t0) + (var_pclm_i * var_t0_dn12));
        var_pclm_i_dn13 = ((var_pclm_i_dn13 * var_t0) + (var_pclm_i * var_t0_dn13));
        var_pclm_i_dn14 = ((var_pclm_i_dn14 * var_t0) + (var_pclm_i * var_t0_dn14));
        var_pclm_i_rv = 0.0;

        let assign3810_e5082: f64 = (var_pclm_i).max(0.0);
        var_pclm_i = assign3810_e5082;
        var_pclm_i_dn0 = if var_pclm_i >= 0.0 { var_pclm_i_dn0 } else { 0.0 };
        var_pclm_i_dn2 = if var_pclm_i >= 0.0 { var_pclm_i_dn2 } else { 0.0 };
        var_pclm_i_dn3 = if var_pclm_i >= 0.0 { var_pclm_i_dn3 } else { 0.0 };
        var_pclm_i_dn4 = if var_pclm_i >= 0.0 { var_pclm_i_dn4 } else { 0.0 };
        var_pclm_i_dn5 = if var_pclm_i >= 0.0 { var_pclm_i_dn5 } else { 0.0 };
        var_pclm_i_dn6 = if var_pclm_i >= 0.0 { var_pclm_i_dn6 } else { 0.0 };
        var_pclm_i_dn7 = if var_pclm_i >= 0.0 { var_pclm_i_dn7 } else { 0.0 };
        var_pclm_i_dn8 = if var_pclm_i >= 0.0 { var_pclm_i_dn8 } else { 0.0 };
        var_pclm_i_dn9 = if var_pclm_i >= 0.0 { var_pclm_i_dn9 } else { 0.0 };
        var_pclm_i_dn10 = if var_pclm_i >= 0.0 { var_pclm_i_dn10 } else { 0.0 };
        var_pclm_i_dn11 = if var_pclm_i >= 0.0 { var_pclm_i_dn11 } else { 0.0 };
        var_pclm_i_dn12 = if var_pclm_i >= 0.0 { var_pclm_i_dn12 } else { 0.0 };
        var_pclm_i_dn13 = if var_pclm_i >= 0.0 { var_pclm_i_dn13 } else { 0.0 };
        var_pclm_i_dn14 = if var_pclm_i >= 0.0 { var_pclm_i_dn14 } else { 0.0 };
        var_pclm_i_rv = 0.0;

        let assign3820_e5085: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        var_guard33 = assign3820_e5085;
        var_guard33_rv = 0.0;

        let (assign3830_e5091, assign3830_e5091_d_n0, assign3830_e5091_d_n2, assign3830_e5091_d_n3, assign3830_e5091_d_n4, assign3830_e5091_d_n5, assign3830_e5091_d_n6, assign3830_e5091_d_n7, assign3830_e5091_d_n8, assign3830_e5091_d_n9, assign3830_e5091_d_n10, assign3830_e5091_d_n11, assign3830_e5091_d_n12, assign3830_e5091_d_n13, assign3830_e5091_d_n14,) = {
    if (var_guard33 != 0.0) {
        let assign3830_e5089: f64 = (var_pclmr_i * var_t0);
        (assign3830_e5089, ((var_pclmr_i_dn0 * var_t0) + (var_pclmr_i * var_t0_dn0)), ((var_pclmr_i_dn2 * var_t0) + (var_pclmr_i * var_t0_dn2)), ((var_pclmr_i_dn3 * var_t0) + (var_pclmr_i * var_t0_dn3)), ((var_pclmr_i_dn4 * var_t0) + (var_pclmr_i * var_t0_dn4)), ((var_pclmr_i_dn5 * var_t0) + (var_pclmr_i * var_t0_dn5)), ((var_pclmr_i_dn6 * var_t0) + (var_pclmr_i * var_t0_dn6)), ((var_pclmr_i_dn7 * var_t0) + (var_pclmr_i * var_t0_dn7)), ((var_pclmr_i_dn8 * var_t0) + (var_pclmr_i * var_t0_dn8)), ((var_pclmr_i_dn9 * var_t0) + (var_pclmr_i * var_t0_dn9)), ((var_pclmr_i_dn10 * var_t0) + (var_pclmr_i * var_t0_dn10)), ((var_pclmr_i_dn11 * var_t0) + (var_pclmr_i * var_t0_dn11)), ((var_pclmr_i_dn12 * var_t0) + (var_pclmr_i * var_t0_dn12)), ((var_pclmr_i_dn13 * var_t0) + (var_pclmr_i * var_t0_dn13)), ((var_pclmr_i_dn14 * var_t0) + (var_pclmr_i * var_t0_dn14)),)
    } else {
        (var_pclmr_i, var_pclmr_i_dn0, var_pclmr_i_dn2, var_pclmr_i_dn3, var_pclmr_i_dn4, var_pclmr_i_dn5, var_pclmr_i_dn6, var_pclmr_i_dn7, var_pclmr_i_dn8, var_pclmr_i_dn9, var_pclmr_i_dn10, var_pclmr_i_dn11, var_pclmr_i_dn12, var_pclmr_i_dn13, var_pclmr_i_dn14,)
    }
};
        var_pclmr_i = assign3830_e5091;
        var_pclmr_i_dn0 = assign3830_e5091_d_n0;
        var_pclmr_i_dn2 = assign3830_e5091_d_n2;
        var_pclmr_i_dn3 = assign3830_e5091_d_n3;
        var_pclmr_i_dn4 = assign3830_e5091_d_n4;
        var_pclmr_i_dn5 = assign3830_e5091_d_n5;
        var_pclmr_i_dn6 = assign3830_e5091_d_n6;
        var_pclmr_i_dn7 = assign3830_e5091_d_n7;
        var_pclmr_i_dn8 = assign3830_e5091_d_n8;
        var_pclmr_i_dn9 = assign3830_e5091_d_n9;
        var_pclmr_i_dn10 = assign3830_e5091_d_n10;
        var_pclmr_i_dn11 = assign3830_e5091_d_n11;
        var_pclmr_i_dn12 = assign3830_e5091_d_n12;
        var_pclmr_i_dn13 = assign3830_e5091_d_n13;
        var_pclmr_i_dn14 = assign3830_e5091_d_n14;
        var_pclmr_i_rv = 0.0;

        let (assign3840_e5097, assign3840_e5097_d_n0, assign3840_e5097_d_n2, assign3840_e5097_d_n3, assign3840_e5097_d_n4, assign3840_e5097_d_n5, assign3840_e5097_d_n6, assign3840_e5097_d_n7, assign3840_e5097_d_n8, assign3840_e5097_d_n9, assign3840_e5097_d_n10, assign3840_e5097_d_n11, assign3840_e5097_d_n12, assign3840_e5097_d_n13, assign3840_e5097_d_n14,) = {
    if (var_guard33 != 0.0) {
        let assign3840_e5095: f64 = (var_pclmr_i).max(0.0);
        (assign3840_e5095, if var_pclmr_i >= 0.0 { var_pclmr_i_dn0 } else { 0.0 }, if var_pclmr_i >= 0.0 { var_pclmr_i_dn2 } else { 0.0 }, if var_pclmr_i >= 0.0 { var_pclmr_i_dn3 } else { 0.0 }, if var_pclmr_i >= 0.0 { var_pclmr_i_dn4 } else { 0.0 }, if var_pclmr_i >= 0.0 { var_pclmr_i_dn5 } else { 0.0 }, if var_pclmr_i >= 0.0 { var_pclmr_i_dn6 } else { 0.0 }, if var_pclmr_i >= 0.0 { var_pclmr_i_dn7 } else { 0.0 }, if var_pclmr_i >= 0.0 { var_pclmr_i_dn8 } else { 0.0 }, if var_pclmr_i >= 0.0 { var_pclmr_i_dn9 } else { 0.0 }, if var_pclmr_i >= 0.0 { var_pclmr_i_dn10 } else { 0.0 }, if var_pclmr_i >= 0.0 { var_pclmr_i_dn11 } else { 0.0 }, if var_pclmr_i >= 0.0 { var_pclmr_i_dn12 } else { 0.0 }, if var_pclmr_i >= 0.0 { var_pclmr_i_dn13 } else { 0.0 }, if var_pclmr_i >= 0.0 { var_pclmr_i_dn14 } else { 0.0 },)
    } else {
        (var_pclmr_i, var_pclmr_i_dn0, var_pclmr_i_dn2, var_pclmr_i_dn3, var_pclmr_i_dn4, var_pclmr_i_dn5, var_pclmr_i_dn6, var_pclmr_i_dn7, var_pclmr_i_dn8, var_pclmr_i_dn9, var_pclmr_i_dn10, var_pclmr_i_dn11, var_pclmr_i_dn12, var_pclmr_i_dn13, var_pclmr_i_dn14,)
    }
};
        var_pclmr_i = assign3840_e5097;
        var_pclmr_i_dn0 = assign3840_e5097_d_n0;
        var_pclmr_i_dn2 = assign3840_e5097_d_n2;
        var_pclmr_i_dn3 = assign3840_e5097_d_n3;
        var_pclmr_i_dn4 = assign3840_e5097_d_n4;
        var_pclmr_i_dn5 = assign3840_e5097_d_n5;
        var_pclmr_i_dn6 = assign3840_e5097_d_n6;
        var_pclmr_i_dn7 = assign3840_e5097_d_n7;
        var_pclmr_i_dn8 = assign3840_e5097_d_n8;
        var_pclmr_i_dn9 = assign3840_e5097_d_n9;
        var_pclmr_i_dn10 = assign3840_e5097_d_n10;
        var_pclmr_i_dn11 = assign3840_e5097_d_n11;
        var_pclmr_i_dn12 = assign3840_e5097_d_n12;
        var_pclmr_i_dn13 = assign3840_e5097_d_n13;
        var_pclmr_i_dn14 = assign3840_e5097_d_n14;
        var_pclmr_i_rv = 0.0;

        let assign3850_e5101: f64 = (var_inv_l).powf(p.p244);
        let assign3850_e5104: f64 = (var_inv_llong).powf(p.p244);
        let assign3850_e5105: f64 = (assign3850_e5101 - assign3850_e5104);
        let assign3850_e5107: f64 = (assign3850_e5105).max(0.0);
        let assign3850_e5108: f64 = (p.p243 * assign3850_e5107);
        var_t0 = assign3850_e5108;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign3860_e5112: f64 = (var_inv_w).powf(p.p246);
        let assign3860_e5115: f64 = (var_inv_wwide).powf(p.p246);
        let assign3860_e5116: f64 = (assign3860_e5112 - assign3860_e5115);
        let assign3860_e5118: f64 = (assign3860_e5116).max(0.0);
        let assign3860_e5119: f64 = (p.p245 * assign3860_e5118);
        let assign3860_e5123: f64 = (var_inv_wl).powf(p.p248);
        let assign3860_e5124: f64 = (p.p247 * assign3860_e5123);
        let assign3860_e5125: f64 = (assign3860_e5119 + assign3860_e5124);
        var_t1 = assign3860_e5125;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        let assign3870_e5129: f64 = (1.0 + var_t0);
        let assign3870_e5131: f64 = (assign3870_e5129 + var_t1);
        let assign3870_e5132: f64 = (var_vsat_i * assign3870_e5131);
        var_vsat_i = assign3870_e5132;
        var_vsat_i_dn0 = ((var_vsat_i_dn0 * assign3870_e5131) + (var_vsat_i * (var_t0_dn0 + var_t1_dn0)));
        var_vsat_i_dn2 = ((var_vsat_i_dn2 * assign3870_e5131) + (var_vsat_i * (var_t0_dn2 + var_t1_dn2)));
        var_vsat_i_dn3 = ((var_vsat_i_dn3 * assign3870_e5131) + (var_vsat_i * (var_t0_dn3 + var_t1_dn3)));
        var_vsat_i_dn4 = ((var_vsat_i_dn4 * assign3870_e5131) + (var_vsat_i * (var_t0_dn4 + var_t1_dn4)));
        var_vsat_i_dn5 = ((var_vsat_i_dn5 * assign3870_e5131) + (var_vsat_i * (var_t0_dn5 + var_t1_dn5)));
        var_vsat_i_dn6 = ((var_vsat_i_dn6 * assign3870_e5131) + (var_vsat_i * (var_t0_dn6 + var_t1_dn6)));
        var_vsat_i_dn7 = ((var_vsat_i_dn7 * assign3870_e5131) + (var_vsat_i * (var_t0_dn7 + var_t1_dn7)));
        var_vsat_i_dn8 = ((var_vsat_i_dn8 * assign3870_e5131) + (var_vsat_i * (var_t0_dn8 + var_t1_dn8)));
        var_vsat_i_dn9 = ((var_vsat_i_dn9 * assign3870_e5131) + (var_vsat_i * (var_t0_dn9 + var_t1_dn9)));
        var_vsat_i_dn10 = ((var_vsat_i_dn10 * assign3870_e5131) + (var_vsat_i * (var_t0_dn10 + var_t1_dn10)));
        var_vsat_i_dn11 = ((var_vsat_i_dn11 * assign3870_e5131) + (var_vsat_i * (var_t0_dn11 + var_t1_dn11)));
        var_vsat_i_dn12 = ((var_vsat_i_dn12 * assign3870_e5131) + (var_vsat_i * (var_t0_dn12 + var_t1_dn12)));
        var_vsat_i_dn13 = ((var_vsat_i_dn13 * assign3870_e5131) + (var_vsat_i * (var_t0_dn13 + var_t1_dn13)));
        var_vsat_i_dn14 = ((var_vsat_i_dn14 * assign3870_e5131) + (var_vsat_i * (var_t0_dn14 + var_t1_dn14)));
        var_vsat_i_rv = 0.0;

        let assign3880_e5135: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        var_guard34 = assign3880_e5135;
        var_guard34_rv = 0.0;

        let (assign3890_e5145, assign3890_e5145_d_n0, assign3890_e5145_d_n2, assign3890_e5145_d_n3, assign3890_e5145_d_n4, assign3890_e5145_d_n5, assign3890_e5145_d_n6, assign3890_e5145_d_n7, assign3890_e5145_d_n8, assign3890_e5145_d_n9, assign3890_e5145_d_n10, assign3890_e5145_d_n11, assign3890_e5145_d_n12, assign3890_e5145_d_n13, assign3890_e5145_d_n14,) = {
    if (var_guard34 != 0.0) {
        let assign3890_e5140: f64 = (1.0 + var_t0);
        let assign3890_e5142: f64 = (assign3890_e5140 + var_t1);
        let assign3890_e5143: f64 = (var_vsatr_i * assign3890_e5142);
        (assign3890_e5143, ((var_vsatr_i_dn0 * assign3890_e5142) + (var_vsatr_i * (var_t0_dn0 + var_t1_dn0))), ((var_vsatr_i_dn2 * assign3890_e5142) + (var_vsatr_i * (var_t0_dn2 + var_t1_dn2))), ((var_vsatr_i_dn3 * assign3890_e5142) + (var_vsatr_i * (var_t0_dn3 + var_t1_dn3))), ((var_vsatr_i_dn4 * assign3890_e5142) + (var_vsatr_i * (var_t0_dn4 + var_t1_dn4))), ((var_vsatr_i_dn5 * assign3890_e5142) + (var_vsatr_i * (var_t0_dn5 + var_t1_dn5))), ((var_vsatr_i_dn6 * assign3890_e5142) + (var_vsatr_i * (var_t0_dn6 + var_t1_dn6))), ((var_vsatr_i_dn7 * assign3890_e5142) + (var_vsatr_i * (var_t0_dn7 + var_t1_dn7))), ((var_vsatr_i_dn8 * assign3890_e5142) + (var_vsatr_i * (var_t0_dn8 + var_t1_dn8))), ((var_vsatr_i_dn9 * assign3890_e5142) + (var_vsatr_i * (var_t0_dn9 + var_t1_dn9))), ((var_vsatr_i_dn10 * assign3890_e5142) + (var_vsatr_i * (var_t0_dn10 + var_t1_dn10))), ((var_vsatr_i_dn11 * assign3890_e5142) + (var_vsatr_i * (var_t0_dn11 + var_t1_dn11))), ((var_vsatr_i_dn12 * assign3890_e5142) + (var_vsatr_i * (var_t0_dn12 + var_t1_dn12))), ((var_vsatr_i_dn13 * assign3890_e5142) + (var_vsatr_i * (var_t0_dn13 + var_t1_dn13))), ((var_vsatr_i_dn14 * assign3890_e5142) + (var_vsatr_i * (var_t0_dn14 + var_t1_dn14))),)
    } else {
        (var_vsatr_i, var_vsatr_i_dn0, var_vsatr_i_dn2, var_vsatr_i_dn3, var_vsatr_i_dn4, var_vsatr_i_dn5, var_vsatr_i_dn6, var_vsatr_i_dn7, var_vsatr_i_dn8, var_vsatr_i_dn9, var_vsatr_i_dn10, var_vsatr_i_dn11, var_vsatr_i_dn12, var_vsatr_i_dn13, var_vsatr_i_dn14,)
    }
};
        var_vsatr_i = assign3890_e5145;
        var_vsatr_i_dn0 = assign3890_e5145_d_n0;
        var_vsatr_i_dn2 = assign3890_e5145_d_n2;
        var_vsatr_i_dn3 = assign3890_e5145_d_n3;
        var_vsatr_i_dn4 = assign3890_e5145_d_n4;
        var_vsatr_i_dn5 = assign3890_e5145_d_n5;
        var_vsatr_i_dn6 = assign3890_e5145_d_n6;
        var_vsatr_i_dn7 = assign3890_e5145_d_n7;
        var_vsatr_i_dn8 = assign3890_e5145_d_n8;
        var_vsatr_i_dn9 = assign3890_e5145_d_n9;
        var_vsatr_i_dn10 = assign3890_e5145_d_n10;
        var_vsatr_i_dn11 = assign3890_e5145_d_n11;
        var_vsatr_i_dn12 = assign3890_e5145_d_n12;
        var_vsatr_i_dn13 = assign3890_e5145_d_n13;
        var_vsatr_i_dn14 = assign3890_e5145_d_n14;
        var_vsatr_i_rv = 0.0;

        let assign3900_e5151: f64 = (var_inv_l).powf(p.p424);
        let assign3900_e5154: f64 = (var_inv_llong).powf(p.p424);
        let assign3900_e5155: f64 = (assign3900_e5151 - assign3900_e5154);
        let assign3900_e5157: f64 = (assign3900_e5155).max(0.0);
        let assign3900_e5158: f64 = (p.p423 * assign3900_e5157);
        let assign3900_e5159: f64 = (1.0 + assign3900_e5158);
        let assign3900_e5160: f64 = (var_psat_i * assign3900_e5159);
        let assign3900_e5162: f64 = (assign3900_e5160).max(0.25);
        var_psat_i = assign3900_e5162;
        var_psat_i_rv = 0.0;

        let assign3910_e5165: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        var_guard35 = assign3910_e5165;
        var_guard35_rv = 0.0;

        let (assign3920_e5185,) = {
    if (var_guard35 != 0.0) {
        let assign3920_e5172: f64 = (var_inv_l).powf(p.p424);
        let assign3920_e5175: f64 = (var_inv_llong).powf(p.p424);
        let assign3920_e5176: f64 = (assign3920_e5172 - assign3920_e5175);
        let assign3920_e5178: f64 = (assign3920_e5176).max(0.0);
        let assign3920_e5179: f64 = (p.p423 * assign3920_e5178);
        let assign3920_e5180: f64 = (1.0 + assign3920_e5179);
        let assign3920_e5181: f64 = (var_psatr_i * assign3920_e5180);
        let assign3920_e5183: f64 = (assign3920_e5181).max(0.25);
        (assign3920_e5183,)
    } else {
        (var_psatr_i,)
    }
};
        var_psatr_i = assign3920_e5185;
        var_psatr_i_rv = 0.0;

        let assign3930_e5190: f64 = (var_inv_l).powf(p.p439);
        let assign3930_e5193: f64 = (var_inv_llong).powf(p.p439);
        let assign3930_e5194: f64 = (assign3930_e5190 - assign3930_e5193);
        let assign3930_e5196: f64 = (assign3930_e5194).max(0.0);
        let assign3930_e5197: f64 = (p.p438 * assign3930_e5196);
        let assign3930_e5198: f64 = (1.0 + assign3930_e5197);
        var_t0 = assign3930_e5198;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign3940_e5201: f64 = (var_ptwg_i * var_t0);
        var_ptwg_i = assign3940_e5201;
        var_ptwg_i_dn0 = ((var_ptwg_i_dn0 * var_t0) + (var_ptwg_i * var_t0_dn0));
        var_ptwg_i_dn2 = ((var_ptwg_i_dn2 * var_t0) + (var_ptwg_i * var_t0_dn2));
        var_ptwg_i_dn3 = ((var_ptwg_i_dn3 * var_t0) + (var_ptwg_i * var_t0_dn3));
        var_ptwg_i_dn4 = ((var_ptwg_i_dn4 * var_t0) + (var_ptwg_i * var_t0_dn4));
        var_ptwg_i_dn5 = ((var_ptwg_i_dn5 * var_t0) + (var_ptwg_i * var_t0_dn5));
        var_ptwg_i_dn6 = ((var_ptwg_i_dn6 * var_t0) + (var_ptwg_i * var_t0_dn6));
        var_ptwg_i_dn7 = ((var_ptwg_i_dn7 * var_t0) + (var_ptwg_i * var_t0_dn7));
        var_ptwg_i_dn8 = ((var_ptwg_i_dn8 * var_t0) + (var_ptwg_i * var_t0_dn8));
        var_ptwg_i_dn9 = ((var_ptwg_i_dn9 * var_t0) + (var_ptwg_i * var_t0_dn9));
        var_ptwg_i_dn10 = ((var_ptwg_i_dn10 * var_t0) + (var_ptwg_i * var_t0_dn10));
        var_ptwg_i_dn11 = ((var_ptwg_i_dn11 * var_t0) + (var_ptwg_i * var_t0_dn11));
        var_ptwg_i_dn12 = ((var_ptwg_i_dn12 * var_t0) + (var_ptwg_i * var_t0_dn12));
        var_ptwg_i_dn13 = ((var_ptwg_i_dn13 * var_t0) + (var_ptwg_i * var_t0_dn13));
        var_ptwg_i_dn14 = ((var_ptwg_i_dn14 * var_t0) + (var_ptwg_i * var_t0_dn14));
        var_ptwg_i_rv = 0.0;

        let assign3950_e5204: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        var_guard36 = assign3950_e5204;
        var_guard36_rv = 0.0;

        let (assign3960_e5210, assign3960_e5210_d_n0, assign3960_e5210_d_n2, assign3960_e5210_d_n3, assign3960_e5210_d_n4, assign3960_e5210_d_n5, assign3960_e5210_d_n6, assign3960_e5210_d_n7, assign3960_e5210_d_n8, assign3960_e5210_d_n9, assign3960_e5210_d_n10, assign3960_e5210_d_n11, assign3960_e5210_d_n12, assign3960_e5210_d_n13, assign3960_e5210_d_n14,) = {
    if (var_guard36 != 0.0) {
        let assign3960_e5208: f64 = (var_ptwgr_i * var_t0);
        (assign3960_e5208, ((var_ptwgr_i_dn0 * var_t0) + (var_ptwgr_i * var_t0_dn0)), ((var_ptwgr_i_dn2 * var_t0) + (var_ptwgr_i * var_t0_dn2)), ((var_ptwgr_i_dn3 * var_t0) + (var_ptwgr_i * var_t0_dn3)), ((var_ptwgr_i_dn4 * var_t0) + (var_ptwgr_i * var_t0_dn4)), ((var_ptwgr_i_dn5 * var_t0) + (var_ptwgr_i * var_t0_dn5)), ((var_ptwgr_i_dn6 * var_t0) + (var_ptwgr_i * var_t0_dn6)), ((var_ptwgr_i_dn7 * var_t0) + (var_ptwgr_i * var_t0_dn7)), ((var_ptwgr_i_dn8 * var_t0) + (var_ptwgr_i * var_t0_dn8)), ((var_ptwgr_i_dn9 * var_t0) + (var_ptwgr_i * var_t0_dn9)), ((var_ptwgr_i_dn10 * var_t0) + (var_ptwgr_i * var_t0_dn10)), ((var_ptwgr_i_dn11 * var_t0) + (var_ptwgr_i * var_t0_dn11)), ((var_ptwgr_i_dn12 * var_t0) + (var_ptwgr_i * var_t0_dn12)), ((var_ptwgr_i_dn13 * var_t0) + (var_ptwgr_i * var_t0_dn13)), ((var_ptwgr_i_dn14 * var_t0) + (var_ptwgr_i * var_t0_dn14)),)
    } else {
        (var_ptwgr_i, var_ptwgr_i_dn0, var_ptwgr_i_dn2, var_ptwgr_i_dn3, var_ptwgr_i_dn4, var_ptwgr_i_dn5, var_ptwgr_i_dn6, var_ptwgr_i_dn7, var_ptwgr_i_dn8, var_ptwgr_i_dn9, var_ptwgr_i_dn10, var_ptwgr_i_dn11, var_ptwgr_i_dn12, var_ptwgr_i_dn13, var_ptwgr_i_dn14,)
    }
};
        var_ptwgr_i = assign3960_e5210;
        var_ptwgr_i_dn0 = assign3960_e5210_d_n0;
        var_ptwgr_i_dn2 = assign3960_e5210_d_n2;
        var_ptwgr_i_dn3 = assign3960_e5210_d_n3;
        var_ptwgr_i_dn4 = assign3960_e5210_d_n4;
        var_ptwgr_i_dn5 = assign3960_e5210_d_n5;
        var_ptwgr_i_dn6 = assign3960_e5210_d_n6;
        var_ptwgr_i_dn7 = assign3960_e5210_d_n7;
        var_ptwgr_i_dn8 = assign3960_e5210_d_n8;
        var_ptwgr_i_dn9 = assign3960_e5210_d_n9;
        var_ptwgr_i_dn10 = assign3960_e5210_d_n10;
        var_ptwgr_i_dn11 = assign3960_e5210_d_n11;
        var_ptwgr_i_dn12 = assign3960_e5210_d_n12;
        var_ptwgr_i_dn13 = assign3960_e5210_d_n13;
        var_ptwgr_i_dn14 = assign3960_e5210_d_n14;
        var_ptwgr_i_rv = 0.0;

        let assign3970_e5214: f64 = (var_inv_l).powf(p.p486);
        let assign3970_e5217: f64 = (var_inv_llong).powf(p.p486);
        let assign3970_e5218: f64 = (assign3970_e5214 - assign3970_e5217);
        let assign3970_e5220: f64 = (assign3970_e5218).max(0.0);
        let assign3970_e5221: f64 = (p.p485 * assign3970_e5220);
        var_t0 = assign3970_e5221;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign3980_e5225: f64 = (var_inv_w).powf(p.p488);
        let assign3980_e5228: f64 = (var_inv_wwide).powf(p.p488);
        let assign3980_e5229: f64 = (assign3980_e5225 - assign3980_e5228);
        let assign3980_e5231: f64 = (assign3980_e5229).max(0.0);
        let assign3980_e5232: f64 = (p.p487 * assign3980_e5231);
        var_t1 = assign3980_e5232;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        let assign3990_e5236: f64 = (1.0 + var_t0);
        let assign3990_e5238: f64 = (assign3990_e5236 + var_t1);
        let assign3990_e5239: f64 = (var_alpha0_i * assign3990_e5238);
        var_alpha0_i = assign3990_e5239;
        var_alpha0_i_dn0 = ((var_alpha0_i_dn0 * assign3990_e5238) + (var_alpha0_i * (var_t0_dn0 + var_t1_dn0)));
        var_alpha0_i_dn2 = ((var_alpha0_i_dn2 * assign3990_e5238) + (var_alpha0_i * (var_t0_dn2 + var_t1_dn2)));
        var_alpha0_i_dn3 = ((var_alpha0_i_dn3 * assign3990_e5238) + (var_alpha0_i * (var_t0_dn3 + var_t1_dn3)));
        var_alpha0_i_dn4 = ((var_alpha0_i_dn4 * assign3990_e5238) + (var_alpha0_i * (var_t0_dn4 + var_t1_dn4)));
        var_alpha0_i_dn5 = ((var_alpha0_i_dn5 * assign3990_e5238) + (var_alpha0_i * (var_t0_dn5 + var_t1_dn5)));
        var_alpha0_i_dn6 = ((var_alpha0_i_dn6 * assign3990_e5238) + (var_alpha0_i * (var_t0_dn6 + var_t1_dn6)));
        var_alpha0_i_dn7 = ((var_alpha0_i_dn7 * assign3990_e5238) + (var_alpha0_i * (var_t0_dn7 + var_t1_dn7)));
        var_alpha0_i_dn8 = ((var_alpha0_i_dn8 * assign3990_e5238) + (var_alpha0_i * (var_t0_dn8 + var_t1_dn8)));
        var_alpha0_i_dn9 = ((var_alpha0_i_dn9 * assign3990_e5238) + (var_alpha0_i * (var_t0_dn9 + var_t1_dn9)));
        var_alpha0_i_dn10 = ((var_alpha0_i_dn10 * assign3990_e5238) + (var_alpha0_i * (var_t0_dn10 + var_t1_dn10)));
        var_alpha0_i_dn11 = ((var_alpha0_i_dn11 * assign3990_e5238) + (var_alpha0_i * (var_t0_dn11 + var_t1_dn11)));
        var_alpha0_i_dn12 = ((var_alpha0_i_dn12 * assign3990_e5238) + (var_alpha0_i * (var_t0_dn12 + var_t1_dn12)));
        var_alpha0_i_dn13 = ((var_alpha0_i_dn13 * assign3990_e5238) + (var_alpha0_i * (var_t0_dn13 + var_t1_dn13)));
        var_alpha0_i_dn14 = ((var_alpha0_i_dn14 * assign3990_e5238) + (var_alpha0_i * (var_t0_dn14 + var_t1_dn14)));
        var_alpha0_i_rv = 0.0;

        let assign4000_e5242: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        var_guard37 = assign4000_e5242;
        var_guard37_rv = 0.0;

        let (assign4010_e5252, assign4010_e5252_d_n0, assign4010_e5252_d_n2, assign4010_e5252_d_n3, assign4010_e5252_d_n4, assign4010_e5252_d_n5, assign4010_e5252_d_n6, assign4010_e5252_d_n7, assign4010_e5252_d_n8, assign4010_e5252_d_n9, assign4010_e5252_d_n10, assign4010_e5252_d_n11, assign4010_e5252_d_n12, assign4010_e5252_d_n13, assign4010_e5252_d_n14,) = {
    if (var_guard37 != 0.0) {
        let assign4010_e5247: f64 = (1.0 + var_t0);
        let assign4010_e5249: f64 = (assign4010_e5247 + var_t1);
        let assign4010_e5250: f64 = (var_alpha0r_i * assign4010_e5249);
        (assign4010_e5250, ((var_alpha0r_i_dn0 * assign4010_e5249) + (var_alpha0r_i * (var_t0_dn0 + var_t1_dn0))), ((var_alpha0r_i_dn2 * assign4010_e5249) + (var_alpha0r_i * (var_t0_dn2 + var_t1_dn2))), ((var_alpha0r_i_dn3 * assign4010_e5249) + (var_alpha0r_i * (var_t0_dn3 + var_t1_dn3))), ((var_alpha0r_i_dn4 * assign4010_e5249) + (var_alpha0r_i * (var_t0_dn4 + var_t1_dn4))), ((var_alpha0r_i_dn5 * assign4010_e5249) + (var_alpha0r_i * (var_t0_dn5 + var_t1_dn5))), ((var_alpha0r_i_dn6 * assign4010_e5249) + (var_alpha0r_i * (var_t0_dn6 + var_t1_dn6))), ((var_alpha0r_i_dn7 * assign4010_e5249) + (var_alpha0r_i * (var_t0_dn7 + var_t1_dn7))), ((var_alpha0r_i_dn8 * assign4010_e5249) + (var_alpha0r_i * (var_t0_dn8 + var_t1_dn8))), ((var_alpha0r_i_dn9 * assign4010_e5249) + (var_alpha0r_i * (var_t0_dn9 + var_t1_dn9))), ((var_alpha0r_i_dn10 * assign4010_e5249) + (var_alpha0r_i * (var_t0_dn10 + var_t1_dn10))), ((var_alpha0r_i_dn11 * assign4010_e5249) + (var_alpha0r_i * (var_t0_dn11 + var_t1_dn11))), ((var_alpha0r_i_dn12 * assign4010_e5249) + (var_alpha0r_i * (var_t0_dn12 + var_t1_dn12))), ((var_alpha0r_i_dn13 * assign4010_e5249) + (var_alpha0r_i * (var_t0_dn13 + var_t1_dn13))), ((var_alpha0r_i_dn14 * assign4010_e5249) + (var_alpha0r_i * (var_t0_dn14 + var_t1_dn14))),)
    } else {
        (var_alpha0r_i, var_alpha0r_i_dn0, var_alpha0r_i_dn2, var_alpha0r_i_dn3, var_alpha0r_i_dn4, var_alpha0r_i_dn5, var_alpha0r_i_dn6, var_alpha0r_i_dn7, var_alpha0r_i_dn8, var_alpha0r_i_dn9, var_alpha0r_i_dn10, var_alpha0r_i_dn11, var_alpha0r_i_dn12, var_alpha0r_i_dn13, var_alpha0r_i_dn14,)
    }
};
        var_alpha0r_i = assign4010_e5252;
        var_alpha0r_i_dn0 = assign4010_e5252_d_n0;
        var_alpha0r_i_dn2 = assign4010_e5252_d_n2;
        var_alpha0r_i_dn3 = assign4010_e5252_d_n3;
        var_alpha0r_i_dn4 = assign4010_e5252_d_n4;
        var_alpha0r_i_dn5 = assign4010_e5252_d_n5;
        var_alpha0r_i_dn6 = assign4010_e5252_d_n6;
        var_alpha0r_i_dn7 = assign4010_e5252_d_n7;
        var_alpha0r_i_dn8 = assign4010_e5252_d_n8;
        var_alpha0r_i_dn9 = assign4010_e5252_d_n9;
        var_alpha0r_i_dn10 = assign4010_e5252_d_n10;
        var_alpha0r_i_dn11 = assign4010_e5252_d_n11;
        var_alpha0r_i_dn12 = assign4010_e5252_d_n12;
        var_alpha0r_i_dn13 = assign4010_e5252_d_n13;
        var_alpha0r_i_dn14 = assign4010_e5252_d_n14;
        var_alpha0r_i_rv = 0.0;

        let assign4020_e5256: f64 = (var_inv_w).powf(p.p496);
        let assign4020_e5259: f64 = (var_inv_wwide).powf(p.p496);
        let assign4020_e5260: f64 = (assign4020_e5256 - assign4020_e5259);
        let assign4020_e5262: f64 = (assign4020_e5260).max(0.0);
        let assign4020_e5263: f64 = (p.p495 * assign4020_e5262);
        var_t1 = assign4020_e5263;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        *var_alpha0_i_slot = var_alpha0_i;
        *var_alpha0_i_dn0_slot = var_alpha0_i_dn0;
        *var_alpha0_i_dn10_slot = var_alpha0_i_dn10;
        *var_alpha0_i_dn11_slot = var_alpha0_i_dn11;
        *var_alpha0_i_dn12_slot = var_alpha0_i_dn12;
        *var_alpha0_i_dn13_slot = var_alpha0_i_dn13;
        *var_alpha0_i_dn14_slot = var_alpha0_i_dn14;
        *var_alpha0_i_dn2_slot = var_alpha0_i_dn2;
        *var_alpha0_i_dn3_slot = var_alpha0_i_dn3;
        *var_alpha0_i_dn4_slot = var_alpha0_i_dn4;
        *var_alpha0_i_dn5_slot = var_alpha0_i_dn5;
        *var_alpha0_i_dn6_slot = var_alpha0_i_dn6;
        *var_alpha0_i_dn7_slot = var_alpha0_i_dn7;
        *var_alpha0_i_dn8_slot = var_alpha0_i_dn8;
        *var_alpha0_i_dn9_slot = var_alpha0_i_dn9;
        *var_alpha0_i_rv_slot = var_alpha0_i_rv;
        *var_alpha0r_i_slot = var_alpha0r_i;
        *var_alpha0r_i_dn0_slot = var_alpha0r_i_dn0;
        *var_alpha0r_i_dn10_slot = var_alpha0r_i_dn10;
        *var_alpha0r_i_dn11_slot = var_alpha0r_i_dn11;
        *var_alpha0r_i_dn12_slot = var_alpha0r_i_dn12;
        *var_alpha0r_i_dn13_slot = var_alpha0r_i_dn13;
        *var_alpha0r_i_dn14_slot = var_alpha0r_i_dn14;
        *var_alpha0r_i_dn2_slot = var_alpha0r_i_dn2;
        *var_alpha0r_i_dn3_slot = var_alpha0r_i_dn3;
        *var_alpha0r_i_dn4_slot = var_alpha0r_i_dn4;
        *var_alpha0r_i_dn5_slot = var_alpha0r_i_dn5;
        *var_alpha0r_i_dn6_slot = var_alpha0r_i_dn6;
        *var_alpha0r_i_dn7_slot = var_alpha0r_i_dn7;
        *var_alpha0r_i_dn8_slot = var_alpha0r_i_dn8;
        *var_alpha0r_i_dn9_slot = var_alpha0r_i_dn9;
        *var_alpha0r_i_rv_slot = var_alpha0r_i_rv;
        *var_delta_i_slot = var_delta_i;
        *var_delta_i_dn0_slot = var_delta_i_dn0;
        *var_delta_i_dn10_slot = var_delta_i_dn10;
        *var_delta_i_dn11_slot = var_delta_i_dn11;
        *var_delta_i_dn12_slot = var_delta_i_dn12;
        *var_delta_i_dn13_slot = var_delta_i_dn13;
        *var_delta_i_dn14_slot = var_delta_i_dn14;
        *var_delta_i_dn2_slot = var_delta_i_dn2;
        *var_delta_i_dn3_slot = var_delta_i_dn3;
        *var_delta_i_dn4_slot = var_delta_i_dn4;
        *var_delta_i_dn5_slot = var_delta_i_dn5;
        *var_delta_i_dn6_slot = var_delta_i_dn6;
        *var_delta_i_dn7_slot = var_delta_i_dn7;
        *var_delta_i_dn8_slot = var_delta_i_dn8;
        *var_delta_i_dn9_slot = var_delta_i_dn9;
        *var_delta_i_rv_slot = var_delta_i_rv;
        *var_fprout_i_slot = var_fprout_i;
        *var_fprout_i_rv_slot = var_fprout_i_rv;
        *var_guard33_slot = var_guard33;
        *var_guard33_rv_slot = var_guard33_rv;
        *var_guard34_slot = var_guard34;
        *var_guard34_rv_slot = var_guard34_rv;
        *var_guard35_slot = var_guard35;
        *var_guard35_rv_slot = var_guard35_rv;
        *var_guard36_slot = var_guard36;
        *var_guard36_rv_slot = var_guard36_rv;
        *var_guard37_slot = var_guard37;
        *var_guard37_rv_slot = var_guard37_rv;
        *var_pclm_i_slot = var_pclm_i;
        *var_pclm_i_dn0_slot = var_pclm_i_dn0;
        *var_pclm_i_dn10_slot = var_pclm_i_dn10;
        *var_pclm_i_dn11_slot = var_pclm_i_dn11;
        *var_pclm_i_dn12_slot = var_pclm_i_dn12;
        *var_pclm_i_dn13_slot = var_pclm_i_dn13;
        *var_pclm_i_dn14_slot = var_pclm_i_dn14;
        *var_pclm_i_dn2_slot = var_pclm_i_dn2;
        *var_pclm_i_dn3_slot = var_pclm_i_dn3;
        *var_pclm_i_dn4_slot = var_pclm_i_dn4;
        *var_pclm_i_dn5_slot = var_pclm_i_dn5;
        *var_pclm_i_dn6_slot = var_pclm_i_dn6;
        *var_pclm_i_dn7_slot = var_pclm_i_dn7;
        *var_pclm_i_dn8_slot = var_pclm_i_dn8;
        *var_pclm_i_dn9_slot = var_pclm_i_dn9;
        *var_pclm_i_rv_slot = var_pclm_i_rv;
        *var_pclmr_i_slot = var_pclmr_i;
        *var_pclmr_i_dn0_slot = var_pclmr_i_dn0;
        *var_pclmr_i_dn10_slot = var_pclmr_i_dn10;
        *var_pclmr_i_dn11_slot = var_pclmr_i_dn11;
        *var_pclmr_i_dn12_slot = var_pclmr_i_dn12;
        *var_pclmr_i_dn13_slot = var_pclmr_i_dn13;
        *var_pclmr_i_dn14_slot = var_pclmr_i_dn14;
        *var_pclmr_i_dn2_slot = var_pclmr_i_dn2;
        *var_pclmr_i_dn3_slot = var_pclmr_i_dn3;
        *var_pclmr_i_dn4_slot = var_pclmr_i_dn4;
        *var_pclmr_i_dn5_slot = var_pclmr_i_dn5;
        *var_pclmr_i_dn6_slot = var_pclmr_i_dn6;
        *var_pclmr_i_dn7_slot = var_pclmr_i_dn7;
        *var_pclmr_i_dn8_slot = var_pclmr_i_dn8;
        *var_pclmr_i_dn9_slot = var_pclmr_i_dn9;
        *var_pclmr_i_rv_slot = var_pclmr_i_rv;
        *var_pdiblcr_i_slot = var_pdiblcr_i;
        *var_pdiblcr_i_dn0_slot = var_pdiblcr_i_dn0;
        *var_pdiblcr_i_dn10_slot = var_pdiblcr_i_dn10;
        *var_pdiblcr_i_dn11_slot = var_pdiblcr_i_dn11;
        *var_pdiblcr_i_dn12_slot = var_pdiblcr_i_dn12;
        *var_pdiblcr_i_dn13_slot = var_pdiblcr_i_dn13;
        *var_pdiblcr_i_dn14_slot = var_pdiblcr_i_dn14;
        *var_pdiblcr_i_dn2_slot = var_pdiblcr_i_dn2;
        *var_pdiblcr_i_dn3_slot = var_pdiblcr_i_dn3;
        *var_pdiblcr_i_dn4_slot = var_pdiblcr_i_dn4;
        *var_pdiblcr_i_dn5_slot = var_pdiblcr_i_dn5;
        *var_pdiblcr_i_dn6_slot = var_pdiblcr_i_dn6;
        *var_pdiblcr_i_dn7_slot = var_pdiblcr_i_dn7;
        *var_pdiblcr_i_dn8_slot = var_pdiblcr_i_dn8;
        *var_pdiblcr_i_dn9_slot = var_pdiblcr_i_dn9;
        *var_pdiblcr_i_rv_slot = var_pdiblcr_i_rv;
        *var_psat_i_slot = var_psat_i;
        *var_psat_i_rv_slot = var_psat_i_rv;
        *var_psatr_i_slot = var_psatr_i;
        *var_psatr_i_rv_slot = var_psatr_i_rv;
        *var_ptwg_i_slot = var_ptwg_i;
        *var_ptwg_i_dn0_slot = var_ptwg_i_dn0;
        *var_ptwg_i_dn10_slot = var_ptwg_i_dn10;
        *var_ptwg_i_dn11_slot = var_ptwg_i_dn11;
        *var_ptwg_i_dn12_slot = var_ptwg_i_dn12;
        *var_ptwg_i_dn13_slot = var_ptwg_i_dn13;
        *var_ptwg_i_dn14_slot = var_ptwg_i_dn14;
        *var_ptwg_i_dn2_slot = var_ptwg_i_dn2;
        *var_ptwg_i_dn3_slot = var_ptwg_i_dn3;
        *var_ptwg_i_dn4_slot = var_ptwg_i_dn4;
        *var_ptwg_i_dn5_slot = var_ptwg_i_dn5;
        *var_ptwg_i_dn6_slot = var_ptwg_i_dn6;
        *var_ptwg_i_dn7_slot = var_ptwg_i_dn7;
        *var_ptwg_i_dn8_slot = var_ptwg_i_dn8;
        *var_ptwg_i_dn9_slot = var_ptwg_i_dn9;
        *var_ptwg_i_rv_slot = var_ptwg_i_rv;
        *var_ptwgr_i_slot = var_ptwgr_i;
        *var_ptwgr_i_dn0_slot = var_ptwgr_i_dn0;
        *var_ptwgr_i_dn10_slot = var_ptwgr_i_dn10;
        *var_ptwgr_i_dn11_slot = var_ptwgr_i_dn11;
        *var_ptwgr_i_dn12_slot = var_ptwgr_i_dn12;
        *var_ptwgr_i_dn13_slot = var_ptwgr_i_dn13;
        *var_ptwgr_i_dn14_slot = var_ptwgr_i_dn14;
        *var_ptwgr_i_dn2_slot = var_ptwgr_i_dn2;
        *var_ptwgr_i_dn3_slot = var_ptwgr_i_dn3;
        *var_ptwgr_i_dn4_slot = var_ptwgr_i_dn4;
        *var_ptwgr_i_dn5_slot = var_ptwgr_i_dn5;
        *var_ptwgr_i_dn6_slot = var_ptwgr_i_dn6;
        *var_ptwgr_i_dn7_slot = var_ptwgr_i_dn7;
        *var_ptwgr_i_dn8_slot = var_ptwgr_i_dn8;
        *var_ptwgr_i_dn9_slot = var_ptwgr_i_dn9;
        *var_ptwgr_i_rv_slot = var_ptwgr_i_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn13_slot = var_t0_dn13;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_vsat_i_slot = var_vsat_i;
        *var_vsat_i_dn0_slot = var_vsat_i_dn0;
        *var_vsat_i_dn10_slot = var_vsat_i_dn10;
        *var_vsat_i_dn11_slot = var_vsat_i_dn11;
        *var_vsat_i_dn12_slot = var_vsat_i_dn12;
        *var_vsat_i_dn13_slot = var_vsat_i_dn13;
        *var_vsat_i_dn14_slot = var_vsat_i_dn14;
        *var_vsat_i_dn2_slot = var_vsat_i_dn2;
        *var_vsat_i_dn3_slot = var_vsat_i_dn3;
        *var_vsat_i_dn4_slot = var_vsat_i_dn4;
        *var_vsat_i_dn5_slot = var_vsat_i_dn5;
        *var_vsat_i_dn6_slot = var_vsat_i_dn6;
        *var_vsat_i_dn7_slot = var_vsat_i_dn7;
        *var_vsat_i_dn8_slot = var_vsat_i_dn8;
        *var_vsat_i_dn9_slot = var_vsat_i_dn9;
        *var_vsat_i_rv_slot = var_vsat_i_rv;
        *var_vsatr_i_slot = var_vsatr_i;
        *var_vsatr_i_dn0_slot = var_vsatr_i_dn0;
        *var_vsatr_i_dn10_slot = var_vsatr_i_dn10;
        *var_vsatr_i_dn11_slot = var_vsatr_i_dn11;
        *var_vsatr_i_dn12_slot = var_vsatr_i_dn12;
        *var_vsatr_i_dn13_slot = var_vsatr_i_dn13;
        *var_vsatr_i_dn14_slot = var_vsatr_i_dn14;
        *var_vsatr_i_dn2_slot = var_vsatr_i_dn2;
        *var_vsatr_i_dn3_slot = var_vsatr_i_dn3;
        *var_vsatr_i_dn4_slot = var_vsatr_i_dn4;
        *var_vsatr_i_dn5_slot = var_vsatr_i_dn5;
        *var_vsatr_i_dn6_slot = var_vsatr_i_dn6;
        *var_vsatr_i_dn7_slot = var_vsatr_i_dn7;
        *var_vsatr_i_dn8_slot = var_vsatr_i_dn8;
        *var_vsatr_i_dn9_slot = var_vsatr_i_dn9;
        *var_vsatr_i_rv_slot = var_vsatr_i_rv;
    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        var_inv_l: f64,
        var_inv_lact: f64,
        var_inv_llong: f64,
        var_inv_w: f64,
        var_inv_wact: f64,
        var_inv_wl: f64,
        var_inv_wwide: f64,
        var_agidl_i_slot: &mut f64,
        var_agidl_i_rv_slot: &mut f64,
        var_agisl_i_slot: &mut f64,
        var_agisl_i_rv_slot: &mut f64,
        var_aigc_i_slot: &mut f64,
        var_aigc_i_rv_slot: &mut f64,
        var_aigd_i_slot: &mut f64,
        var_aigd_i_rv_slot: &mut f64,
        var_aigs_i_slot: &mut f64,
        var_aigs_i_rv_slot: &mut f64,
        var_beta0_i_slot: &mut f64,
        var_beta0_i_dn0_slot: &mut f64,
        var_beta0_i_dn10_slot: &mut f64,
        var_beta0_i_dn11_slot: &mut f64,
        var_beta0_i_dn12_slot: &mut f64,
        var_beta0_i_dn13_slot: &mut f64,
        var_beta0_i_dn14_slot: &mut f64,
        var_beta0_i_dn2_slot: &mut f64,
        var_beta0_i_dn3_slot: &mut f64,
        var_beta0_i_dn4_slot: &mut f64,
        var_beta0_i_dn5_slot: &mut f64,
        var_beta0_i_dn6_slot: &mut f64,
        var_beta0_i_dn7_slot: &mut f64,
        var_beta0_i_dn8_slot: &mut f64,
        var_beta0_i_dn9_slot: &mut f64,
        var_beta0_i_rv_slot: &mut f64,
        var_beta1_i_slot: &mut f64,
        var_beta1_i_dn0_slot: &mut f64,
        var_beta1_i_dn10_slot: &mut f64,
        var_beta1_i_dn11_slot: &mut f64,
        var_beta1_i_dn12_slot: &mut f64,
        var_beta1_i_dn13_slot: &mut f64,
        var_beta1_i_dn14_slot: &mut f64,
        var_beta1_i_dn2_slot: &mut f64,
        var_beta1_i_dn3_slot: &mut f64,
        var_beta1_i_dn4_slot: &mut f64,
        var_beta1_i_dn5_slot: &mut f64,
        var_beta1_i_dn6_slot: &mut f64,
        var_beta1_i_dn7_slot: &mut f64,
        var_beta1_i_dn8_slot: &mut f64,
        var_beta1_i_dn9_slot: &mut f64,
        var_beta1_i_rv_slot: &mut f64,
        var_beta2_i_slot: &mut f64,
        var_beta2_i_dn0_slot: &mut f64,
        var_beta2_i_dn10_slot: &mut f64,
        var_beta2_i_dn11_slot: &mut f64,
        var_beta2_i_dn12_slot: &mut f64,
        var_beta2_i_dn13_slot: &mut f64,
        var_beta2_i_dn14_slot: &mut f64,
        var_beta2_i_dn2_slot: &mut f64,
        var_beta2_i_dn3_slot: &mut f64,
        var_beta2_i_dn4_slot: &mut f64,
        var_beta2_i_dn5_slot: &mut f64,
        var_beta2_i_dn6_slot: &mut f64,
        var_beta2_i_dn7_slot: &mut f64,
        var_beta2_i_dn8_slot: &mut f64,
        var_beta2_i_dn9_slot: &mut f64,
        var_beta2_i_rv_slot: &mut f64,
        var_ndepcv_i_slot: &mut f64,
        var_ndepcv_i_dn0_slot: &mut f64,
        var_ndepcv_i_dn10_slot: &mut f64,
        var_ndepcv_i_dn11_slot: &mut f64,
        var_ndepcv_i_dn12_slot: &mut f64,
        var_ndepcv_i_dn13_slot: &mut f64,
        var_ndepcv_i_dn14_slot: &mut f64,
        var_ndepcv_i_dn2_slot: &mut f64,
        var_ndepcv_i_dn3_slot: &mut f64,
        var_ndepcv_i_dn4_slot: &mut f64,
        var_ndepcv_i_dn5_slot: &mut f64,
        var_ndepcv_i_dn6_slot: &mut f64,
        var_ndepcv_i_dn7_slot: &mut f64,
        var_ndepcv_i_dn8_slot: &mut f64,
        var_ndepcv_i_dn9_slot: &mut f64,
        var_ndepcv_i_rv_slot: &mut f64,
        var_pclmcv_i_slot: &mut f64,
        var_pclmcv_i_rv_slot: &mut f64,
        var_pigcd_i_slot: &mut f64,
        var_pigcd_i_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_vfb_i_slot: &mut f64,
        var_vfb_i_dn0_slot: &mut f64,
        var_vfb_i_dn10_slot: &mut f64,
        var_vfb_i_dn11_slot: &mut f64,
        var_vfb_i_dn12_slot: &mut f64,
        var_vfb_i_dn13_slot: &mut f64,
        var_vfb_i_dn14_slot: &mut f64,
        var_vfb_i_dn2_slot: &mut f64,
        var_vfb_i_dn3_slot: &mut f64,
        var_vfb_i_dn4_slot: &mut f64,
        var_vfb_i_dn5_slot: &mut f64,
        var_vfb_i_dn6_slot: &mut f64,
        var_vfb_i_dn7_slot: &mut f64,
        var_vfb_i_dn8_slot: &mut f64,
        var_vfb_i_dn9_slot: &mut f64,
        var_vfb_i_rv_slot: &mut f64,
        var_vfbcv_i_slot: &mut f64,
        var_vfbcv_i_dn0_slot: &mut f64,
        var_vfbcv_i_dn10_slot: &mut f64,
        var_vfbcv_i_dn11_slot: &mut f64,
        var_vfbcv_i_dn12_slot: &mut f64,
        var_vfbcv_i_dn13_slot: &mut f64,
        var_vfbcv_i_dn14_slot: &mut f64,
        var_vfbcv_i_dn2_slot: &mut f64,
        var_vfbcv_i_dn3_slot: &mut f64,
        var_vfbcv_i_dn4_slot: &mut f64,
        var_vfbcv_i_dn5_slot: &mut f64,
        var_vfbcv_i_dn6_slot: &mut f64,
        var_vfbcv_i_dn7_slot: &mut f64,
        var_vfbcv_i_dn8_slot: &mut f64,
        var_vfbcv_i_dn9_slot: &mut f64,
        var_vfbcv_i_rv_slot: &mut f64,
        var_vsatcv_i_slot: &mut f64,
        var_vsatcv_i_dn0_slot: &mut f64,
        var_vsatcv_i_dn10_slot: &mut f64,
        var_vsatcv_i_dn11_slot: &mut f64,
        var_vsatcv_i_dn12_slot: &mut f64,
        var_vsatcv_i_dn13_slot: &mut f64,
        var_vsatcv_i_dn14_slot: &mut f64,
        var_vsatcv_i_dn2_slot: &mut f64,
        var_vsatcv_i_dn3_slot: &mut f64,
        var_vsatcv_i_dn4_slot: &mut f64,
        var_vsatcv_i_dn5_slot: &mut f64,
        var_vsatcv_i_dn6_slot: &mut f64,
        var_vsatcv_i_dn7_slot: &mut f64,
        var_vsatcv_i_dn8_slot: &mut f64,
        var_vsatcv_i_dn9_slot: &mut f64,
        var_vsatcv_i_rv_slot: &mut f64,
    ) {
        let mut var_agidl_i: f64 = *var_agidl_i_slot;
        let mut var_agidl_i_rv: f64 = *var_agidl_i_rv_slot;
        let mut var_agisl_i: f64 = *var_agisl_i_slot;
        let mut var_agisl_i_rv: f64 = *var_agisl_i_rv_slot;
        let mut var_aigc_i: f64 = *var_aigc_i_slot;
        let mut var_aigc_i_rv: f64 = *var_aigc_i_rv_slot;
        let mut var_aigd_i: f64 = *var_aigd_i_slot;
        let mut var_aigd_i_rv: f64 = *var_aigd_i_rv_slot;
        let mut var_aigs_i: f64 = *var_aigs_i_slot;
        let mut var_aigs_i_rv: f64 = *var_aigs_i_rv_slot;
        let mut var_beta0_i: f64 = *var_beta0_i_slot;
        let mut var_beta0_i_dn0: f64 = *var_beta0_i_dn0_slot;
        let mut var_beta0_i_dn10: f64 = *var_beta0_i_dn10_slot;
        let mut var_beta0_i_dn11: f64 = *var_beta0_i_dn11_slot;
        let mut var_beta0_i_dn12: f64 = *var_beta0_i_dn12_slot;
        let mut var_beta0_i_dn13: f64 = *var_beta0_i_dn13_slot;
        let mut var_beta0_i_dn14: f64 = *var_beta0_i_dn14_slot;
        let mut var_beta0_i_dn2: f64 = *var_beta0_i_dn2_slot;
        let mut var_beta0_i_dn3: f64 = *var_beta0_i_dn3_slot;
        let mut var_beta0_i_dn4: f64 = *var_beta0_i_dn4_slot;
        let mut var_beta0_i_dn5: f64 = *var_beta0_i_dn5_slot;
        let mut var_beta0_i_dn6: f64 = *var_beta0_i_dn6_slot;
        let mut var_beta0_i_dn7: f64 = *var_beta0_i_dn7_slot;
        let mut var_beta0_i_dn8: f64 = *var_beta0_i_dn8_slot;
        let mut var_beta0_i_dn9: f64 = *var_beta0_i_dn9_slot;
        let mut var_beta0_i_rv: f64 = *var_beta0_i_rv_slot;
        let mut var_beta1_i: f64 = *var_beta1_i_slot;
        let mut var_beta1_i_dn0: f64 = *var_beta1_i_dn0_slot;
        let mut var_beta1_i_dn10: f64 = *var_beta1_i_dn10_slot;
        let mut var_beta1_i_dn11: f64 = *var_beta1_i_dn11_slot;
        let mut var_beta1_i_dn12: f64 = *var_beta1_i_dn12_slot;
        let mut var_beta1_i_dn13: f64 = *var_beta1_i_dn13_slot;
        let mut var_beta1_i_dn14: f64 = *var_beta1_i_dn14_slot;
        let mut var_beta1_i_dn2: f64 = *var_beta1_i_dn2_slot;
        let mut var_beta1_i_dn3: f64 = *var_beta1_i_dn3_slot;
        let mut var_beta1_i_dn4: f64 = *var_beta1_i_dn4_slot;
        let mut var_beta1_i_dn5: f64 = *var_beta1_i_dn5_slot;
        let mut var_beta1_i_dn6: f64 = *var_beta1_i_dn6_slot;
        let mut var_beta1_i_dn7: f64 = *var_beta1_i_dn7_slot;
        let mut var_beta1_i_dn8: f64 = *var_beta1_i_dn8_slot;
        let mut var_beta1_i_dn9: f64 = *var_beta1_i_dn9_slot;
        let mut var_beta1_i_rv: f64 = *var_beta1_i_rv_slot;
        let mut var_beta2_i: f64 = *var_beta2_i_slot;
        let mut var_beta2_i_dn0: f64 = *var_beta2_i_dn0_slot;
        let mut var_beta2_i_dn10: f64 = *var_beta2_i_dn10_slot;
        let mut var_beta2_i_dn11: f64 = *var_beta2_i_dn11_slot;
        let mut var_beta2_i_dn12: f64 = *var_beta2_i_dn12_slot;
        let mut var_beta2_i_dn13: f64 = *var_beta2_i_dn13_slot;
        let mut var_beta2_i_dn14: f64 = *var_beta2_i_dn14_slot;
        let mut var_beta2_i_dn2: f64 = *var_beta2_i_dn2_slot;
        let mut var_beta2_i_dn3: f64 = *var_beta2_i_dn3_slot;
        let mut var_beta2_i_dn4: f64 = *var_beta2_i_dn4_slot;
        let mut var_beta2_i_dn5: f64 = *var_beta2_i_dn5_slot;
        let mut var_beta2_i_dn6: f64 = *var_beta2_i_dn6_slot;
        let mut var_beta2_i_dn7: f64 = *var_beta2_i_dn7_slot;
        let mut var_beta2_i_dn8: f64 = *var_beta2_i_dn8_slot;
        let mut var_beta2_i_dn9: f64 = *var_beta2_i_dn9_slot;
        let mut var_beta2_i_rv: f64 = *var_beta2_i_rv_slot;
        let mut var_ndepcv_i: f64 = *var_ndepcv_i_slot;
        let mut var_ndepcv_i_dn0: f64 = *var_ndepcv_i_dn0_slot;
        let mut var_ndepcv_i_dn10: f64 = *var_ndepcv_i_dn10_slot;
        let mut var_ndepcv_i_dn11: f64 = *var_ndepcv_i_dn11_slot;
        let mut var_ndepcv_i_dn12: f64 = *var_ndepcv_i_dn12_slot;
        let mut var_ndepcv_i_dn13: f64 = *var_ndepcv_i_dn13_slot;
        let mut var_ndepcv_i_dn14: f64 = *var_ndepcv_i_dn14_slot;
        let mut var_ndepcv_i_dn2: f64 = *var_ndepcv_i_dn2_slot;
        let mut var_ndepcv_i_dn3: f64 = *var_ndepcv_i_dn3_slot;
        let mut var_ndepcv_i_dn4: f64 = *var_ndepcv_i_dn4_slot;
        let mut var_ndepcv_i_dn5: f64 = *var_ndepcv_i_dn5_slot;
        let mut var_ndepcv_i_dn6: f64 = *var_ndepcv_i_dn6_slot;
        let mut var_ndepcv_i_dn7: f64 = *var_ndepcv_i_dn7_slot;
        let mut var_ndepcv_i_dn8: f64 = *var_ndepcv_i_dn8_slot;
        let mut var_ndepcv_i_dn9: f64 = *var_ndepcv_i_dn9_slot;
        let mut var_ndepcv_i_rv: f64 = *var_ndepcv_i_rv_slot;
        let mut var_pclmcv_i: f64 = *var_pclmcv_i_slot;
        let mut var_pclmcv_i_rv: f64 = *var_pclmcv_i_rv_slot;
        let mut var_pigcd_i: f64 = *var_pigcd_i_slot;
        let mut var_pigcd_i_rv: f64 = *var_pigcd_i_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_vfb_i: f64 = *var_vfb_i_slot;
        let mut var_vfb_i_dn0: f64 = *var_vfb_i_dn0_slot;
        let mut var_vfb_i_dn10: f64 = *var_vfb_i_dn10_slot;
        let mut var_vfb_i_dn11: f64 = *var_vfb_i_dn11_slot;
        let mut var_vfb_i_dn12: f64 = *var_vfb_i_dn12_slot;
        let mut var_vfb_i_dn13: f64 = *var_vfb_i_dn13_slot;
        let mut var_vfb_i_dn14: f64 = *var_vfb_i_dn14_slot;
        let mut var_vfb_i_dn2: f64 = *var_vfb_i_dn2_slot;
        let mut var_vfb_i_dn3: f64 = *var_vfb_i_dn3_slot;
        let mut var_vfb_i_dn4: f64 = *var_vfb_i_dn4_slot;
        let mut var_vfb_i_dn5: f64 = *var_vfb_i_dn5_slot;
        let mut var_vfb_i_dn6: f64 = *var_vfb_i_dn6_slot;
        let mut var_vfb_i_dn7: f64 = *var_vfb_i_dn7_slot;
        let mut var_vfb_i_dn8: f64 = *var_vfb_i_dn8_slot;
        let mut var_vfb_i_dn9: f64 = *var_vfb_i_dn9_slot;
        let mut var_vfb_i_rv: f64 = *var_vfb_i_rv_slot;
        let mut var_vfbcv_i: f64 = *var_vfbcv_i_slot;
        let mut var_vfbcv_i_dn0: f64 = *var_vfbcv_i_dn0_slot;
        let mut var_vfbcv_i_dn10: f64 = *var_vfbcv_i_dn10_slot;
        let mut var_vfbcv_i_dn11: f64 = *var_vfbcv_i_dn11_slot;
        let mut var_vfbcv_i_dn12: f64 = *var_vfbcv_i_dn12_slot;
        let mut var_vfbcv_i_dn13: f64 = *var_vfbcv_i_dn13_slot;
        let mut var_vfbcv_i_dn14: f64 = *var_vfbcv_i_dn14_slot;
        let mut var_vfbcv_i_dn2: f64 = *var_vfbcv_i_dn2_slot;
        let mut var_vfbcv_i_dn3: f64 = *var_vfbcv_i_dn3_slot;
        let mut var_vfbcv_i_dn4: f64 = *var_vfbcv_i_dn4_slot;
        let mut var_vfbcv_i_dn5: f64 = *var_vfbcv_i_dn5_slot;
        let mut var_vfbcv_i_dn6: f64 = *var_vfbcv_i_dn6_slot;
        let mut var_vfbcv_i_dn7: f64 = *var_vfbcv_i_dn7_slot;
        let mut var_vfbcv_i_dn8: f64 = *var_vfbcv_i_dn8_slot;
        let mut var_vfbcv_i_dn9: f64 = *var_vfbcv_i_dn9_slot;
        let mut var_vfbcv_i_rv: f64 = *var_vfbcv_i_rv_slot;
        let mut var_vsatcv_i: f64 = *var_vsatcv_i_slot;
        let mut var_vsatcv_i_dn0: f64 = *var_vsatcv_i_dn0_slot;
        let mut var_vsatcv_i_dn10: f64 = *var_vsatcv_i_dn10_slot;
        let mut var_vsatcv_i_dn11: f64 = *var_vsatcv_i_dn11_slot;
        let mut var_vsatcv_i_dn12: f64 = *var_vsatcv_i_dn12_slot;
        let mut var_vsatcv_i_dn13: f64 = *var_vsatcv_i_dn13_slot;
        let mut var_vsatcv_i_dn14: f64 = *var_vsatcv_i_dn14_slot;
        let mut var_vsatcv_i_dn2: f64 = *var_vsatcv_i_dn2_slot;
        let mut var_vsatcv_i_dn3: f64 = *var_vsatcv_i_dn3_slot;
        let mut var_vsatcv_i_dn4: f64 = *var_vsatcv_i_dn4_slot;
        let mut var_vsatcv_i_dn5: f64 = *var_vsatcv_i_dn5_slot;
        let mut var_vsatcv_i_dn6: f64 = *var_vsatcv_i_dn6_slot;
        let mut var_vsatcv_i_dn7: f64 = *var_vsatcv_i_dn7_slot;
        let mut var_vsatcv_i_dn8: f64 = *var_vsatcv_i_dn8_slot;
        let mut var_vsatcv_i_dn9: f64 = *var_vsatcv_i_dn9_slot;
        let mut var_vsatcv_i_rv: f64 = *var_vsatcv_i_rv_slot;

        let assign4030_e5267: f64 = (1.0 + var_t1);
        let assign4030_e5268: f64 = (var_beta0_i * assign4030_e5267);
        var_beta0_i = assign4030_e5268;
        var_beta0_i_dn0 = ((var_beta0_i_dn0 * assign4030_e5267) + (var_beta0_i * var_t1_dn0));
        var_beta0_i_dn2 = ((var_beta0_i_dn2 * assign4030_e5267) + (var_beta0_i * var_t1_dn2));
        var_beta0_i_dn3 = ((var_beta0_i_dn3 * assign4030_e5267) + (var_beta0_i * var_t1_dn3));
        var_beta0_i_dn4 = ((var_beta0_i_dn4 * assign4030_e5267) + (var_beta0_i * var_t1_dn4));
        var_beta0_i_dn5 = ((var_beta0_i_dn5 * assign4030_e5267) + (var_beta0_i * var_t1_dn5));
        var_beta0_i_dn6 = ((var_beta0_i_dn6 * assign4030_e5267) + (var_beta0_i * var_t1_dn6));
        var_beta0_i_dn7 = ((var_beta0_i_dn7 * assign4030_e5267) + (var_beta0_i * var_t1_dn7));
        var_beta0_i_dn8 = ((var_beta0_i_dn8 * assign4030_e5267) + (var_beta0_i * var_t1_dn8));
        var_beta0_i_dn9 = ((var_beta0_i_dn9 * assign4030_e5267) + (var_beta0_i * var_t1_dn9));
        var_beta0_i_dn10 = ((var_beta0_i_dn10 * assign4030_e5267) + (var_beta0_i * var_t1_dn10));
        var_beta0_i_dn11 = ((var_beta0_i_dn11 * assign4030_e5267) + (var_beta0_i * var_t1_dn11));
        var_beta0_i_dn12 = ((var_beta0_i_dn12 * assign4030_e5267) + (var_beta0_i * var_t1_dn12));
        var_beta0_i_dn13 = ((var_beta0_i_dn13 * assign4030_e5267) + (var_beta0_i * var_t1_dn13));
        var_beta0_i_dn14 = ((var_beta0_i_dn14 * assign4030_e5267) + (var_beta0_i * var_t1_dn14));
        var_beta0_i_rv = 0.0;

        let assign4040_e5272: f64 = (var_inv_w).powf(p.p520);
        let assign4040_e5275: f64 = (var_inv_wwide).powf(p.p520);
        let assign4040_e5276: f64 = (assign4040_e5272 - assign4040_e5275);
        let assign4040_e5278: f64 = (assign4040_e5276).max(0.0);
        let assign4040_e5279: f64 = (p.p519 * assign4040_e5278);
        var_t1 = assign4040_e5279;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        var_beta1_i = p.p518;
        var_beta1_i_dn0 = 0.0;
        var_beta1_i_dn2 = 0.0;
        var_beta1_i_dn3 = 0.0;
        var_beta1_i_dn4 = 0.0;
        var_beta1_i_dn5 = 0.0;
        var_beta1_i_dn6 = 0.0;
        var_beta1_i_dn7 = 0.0;
        var_beta1_i_dn8 = 0.0;
        var_beta1_i_dn9 = 0.0;
        var_beta1_i_dn10 = 0.0;
        var_beta1_i_dn11 = 0.0;
        var_beta1_i_dn12 = 0.0;
        var_beta1_i_dn13 = 0.0;
        var_beta1_i_dn14 = 0.0;
        var_beta1_i_rv = 0.0;

        let assign4060_e5284: f64 = (1.0 + var_t1);
        let assign4060_e5285: f64 = (var_beta1_i * assign4060_e5284);
        var_beta1_i = assign4060_e5285;
        var_beta1_i_dn0 = ((var_beta1_i_dn0 * assign4060_e5284) + (var_beta1_i * var_t1_dn0));
        var_beta1_i_dn2 = ((var_beta1_i_dn2 * assign4060_e5284) + (var_beta1_i * var_t1_dn2));
        var_beta1_i_dn3 = ((var_beta1_i_dn3 * assign4060_e5284) + (var_beta1_i * var_t1_dn3));
        var_beta1_i_dn4 = ((var_beta1_i_dn4 * assign4060_e5284) + (var_beta1_i * var_t1_dn4));
        var_beta1_i_dn5 = ((var_beta1_i_dn5 * assign4060_e5284) + (var_beta1_i * var_t1_dn5));
        var_beta1_i_dn6 = ((var_beta1_i_dn6 * assign4060_e5284) + (var_beta1_i * var_t1_dn6));
        var_beta1_i_dn7 = ((var_beta1_i_dn7 * assign4060_e5284) + (var_beta1_i * var_t1_dn7));
        var_beta1_i_dn8 = ((var_beta1_i_dn8 * assign4060_e5284) + (var_beta1_i * var_t1_dn8));
        var_beta1_i_dn9 = ((var_beta1_i_dn9 * assign4060_e5284) + (var_beta1_i * var_t1_dn9));
        var_beta1_i_dn10 = ((var_beta1_i_dn10 * assign4060_e5284) + (var_beta1_i * var_t1_dn10));
        var_beta1_i_dn11 = ((var_beta1_i_dn11 * assign4060_e5284) + (var_beta1_i * var_t1_dn11));
        var_beta1_i_dn12 = ((var_beta1_i_dn12 * assign4060_e5284) + (var_beta1_i * var_t1_dn12));
        var_beta1_i_dn13 = ((var_beta1_i_dn13 * assign4060_e5284) + (var_beta1_i * var_t1_dn13));
        var_beta1_i_dn14 = ((var_beta1_i_dn14 * assign4060_e5284) + (var_beta1_i * var_t1_dn14));
        var_beta1_i_rv = 0.0;

        let assign4070_e5289: f64 = (var_inv_w).powf(p.p523);
        let assign4070_e5292: f64 = (var_inv_wwide).powf(p.p523);
        let assign4070_e5293: f64 = (assign4070_e5289 - assign4070_e5292);
        let assign4070_e5295: f64 = (assign4070_e5293).max(0.0);
        let assign4070_e5296: f64 = (p.p522 * assign4070_e5295);
        var_t1 = assign4070_e5296;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        var_beta2_i = p.p521;
        var_beta2_i_dn0 = 0.0;
        var_beta2_i_dn2 = 0.0;
        var_beta2_i_dn3 = 0.0;
        var_beta2_i_dn4 = 0.0;
        var_beta2_i_dn5 = 0.0;
        var_beta2_i_dn6 = 0.0;
        var_beta2_i_dn7 = 0.0;
        var_beta2_i_dn8 = 0.0;
        var_beta2_i_dn9 = 0.0;
        var_beta2_i_dn10 = 0.0;
        var_beta2_i_dn11 = 0.0;
        var_beta2_i_dn12 = 0.0;
        var_beta2_i_dn13 = 0.0;
        var_beta2_i_dn14 = 0.0;
        var_beta2_i_rv = 0.0;

        let assign4090_e5301: f64 = (1.0 + var_t1);
        let assign4090_e5302: f64 = (var_beta2_i * assign4090_e5301);
        var_beta2_i = assign4090_e5302;
        var_beta2_i_dn0 = ((var_beta2_i_dn0 * assign4090_e5301) + (var_beta2_i * var_t1_dn0));
        var_beta2_i_dn2 = ((var_beta2_i_dn2 * assign4090_e5301) + (var_beta2_i * var_t1_dn2));
        var_beta2_i_dn3 = ((var_beta2_i_dn3 * assign4090_e5301) + (var_beta2_i * var_t1_dn3));
        var_beta2_i_dn4 = ((var_beta2_i_dn4 * assign4090_e5301) + (var_beta2_i * var_t1_dn4));
        var_beta2_i_dn5 = ((var_beta2_i_dn5 * assign4090_e5301) + (var_beta2_i * var_t1_dn5));
        var_beta2_i_dn6 = ((var_beta2_i_dn6 * assign4090_e5301) + (var_beta2_i * var_t1_dn6));
        var_beta2_i_dn7 = ((var_beta2_i_dn7 * assign4090_e5301) + (var_beta2_i * var_t1_dn7));
        var_beta2_i_dn8 = ((var_beta2_i_dn8 * assign4090_e5301) + (var_beta2_i * var_t1_dn8));
        var_beta2_i_dn9 = ((var_beta2_i_dn9 * assign4090_e5301) + (var_beta2_i * var_t1_dn9));
        var_beta2_i_dn10 = ((var_beta2_i_dn10 * assign4090_e5301) + (var_beta2_i * var_t1_dn10));
        var_beta2_i_dn11 = ((var_beta2_i_dn11 * assign4090_e5301) + (var_beta2_i * var_t1_dn11));
        var_beta2_i_dn12 = ((var_beta2_i_dn12 * assign4090_e5301) + (var_beta2_i * var_t1_dn12));
        var_beta2_i_dn13 = ((var_beta2_i_dn13 * assign4090_e5301) + (var_beta2_i * var_t1_dn13));
        var_beta2_i_dn14 = ((var_beta2_i_dn14 * assign4090_e5301) + (var_beta2_i * var_t1_dn14));
        var_beta2_i_rv = 0.0;

        let assign4100_e5307: f64 = (p.p631 * var_inv_l);
        let assign4100_e5308: f64 = (1.0 + assign4100_e5307);
        let assign4100_e5311: f64 = (p.p632 * var_inv_w);
        let assign4100_e5312: f64 = (assign4100_e5308 + assign4100_e5311);
        let assign4100_e5313: f64 = (var_agidl_i * assign4100_e5312);
        var_agidl_i = assign4100_e5313;
        var_agidl_i_rv = 0.0;

        let assign4110_e5318: f64 = (p.p649 * var_inv_l);
        let assign4110_e5319: f64 = (1.0 + assign4110_e5318);
        let assign4110_e5322: f64 = (p.p650 * var_inv_w);
        let assign4110_e5323: f64 = (assign4110_e5319 + assign4110_e5322);
        let assign4110_e5324: f64 = (var_agisl_i * assign4110_e5323);
        var_agisl_i = assign4110_e5324;
        var_agisl_i_rv = 0.0;

        let assign4120_e5329: f64 = (p.p557 * var_inv_l);
        let assign4120_e5330: f64 = (1.0 + assign4120_e5329);
        let assign4120_e5333: f64 = (p.p558 * var_inv_w);
        let assign4120_e5334: f64 = (assign4120_e5330 + assign4120_e5333);
        let assign4120_e5335: f64 = (var_aigc_i * assign4120_e5334);
        var_aigc_i = assign4120_e5335;
        var_aigc_i_rv = 0.0;

        let assign4130_e5340: f64 = (p.p559 * var_inv_l);
        let assign4130_e5341: f64 = (1.0 + assign4130_e5340);
        let assign4130_e5344: f64 = (p.p560 * var_inv_w);
        let assign4130_e5345: f64 = (assign4130_e5341 + assign4130_e5344);
        let assign4130_e5346: f64 = (var_aigs_i * assign4130_e5345);
        var_aigs_i = assign4130_e5346;
        var_aigs_i_rv = 0.0;

        let assign4140_e5351: f64 = (p.p561 * var_inv_l);
        let assign4140_e5352: f64 = (1.0 + assign4140_e5351);
        let assign4140_e5355: f64 = (p.p562 * var_inv_w);
        let assign4140_e5356: f64 = (assign4140_e5352 + assign4140_e5355);
        let assign4140_e5357: f64 = (var_aigd_i * assign4140_e5356);
        var_aigd_i = assign4140_e5357;
        var_aigd_i_rv = 0.0;

        let assign4150_e5362: f64 = (p.p563 * var_inv_l);
        let assign4150_e5363: f64 = (1.0 + assign4150_e5362);
        let assign4150_e5364: f64 = (p.p556 * assign4150_e5363);
        var_pigcd_i = assign4150_e5364;
        var_pigcd_i_rv = 0.0;

        let assign4160_e5368: f64 = (var_inv_lact).powf(p.p94);
        let assign4160_e5371: f64 = (var_inv_llong).powf(p.p94);
        let assign4160_e5372: f64 = (assign4160_e5368 - assign4160_e5371);
        let assign4160_e5374: f64 = (assign4160_e5372).max(0.0);
        let assign4160_e5375: f64 = (p.p93 * assign4160_e5374);
        let assign4160_e5379: f64 = (var_inv_lact).powf(p.p96);
        let assign4160_e5382: f64 = (var_inv_llong).powf(p.p96);
        let assign4160_e5383: f64 = (assign4160_e5379 - assign4160_e5382);
        let assign4160_e5385: f64 = (assign4160_e5383).max(0.0);
        let assign4160_e5386: f64 = (p.p95 * assign4160_e5385);
        let assign4160_e5387: f64 = (assign4160_e5375 + assign4160_e5386);
        var_t0 = assign4160_e5387;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign4170_e5391: f64 = (var_inv_wact).powf(p.p98);
        let assign4170_e5394: f64 = (var_inv_wwide).powf(p.p98);
        let assign4170_e5395: f64 = (assign4170_e5391 - assign4170_e5394);
        let assign4170_e5397: f64 = (assign4170_e5395).max(0.0);
        let assign4170_e5398: f64 = (p.p97 * assign4170_e5397);
        let assign4170_e5402: f64 = (var_inv_wact * var_inv_lact);
        let assign4170_e5404: f64 = (assign4170_e5402).powf(p.p100);
        let assign4170_e5405: f64 = (p.p99 * assign4170_e5404);
        let assign4170_e5406: f64 = (assign4170_e5398 + assign4170_e5405);
        var_t1 = assign4170_e5406;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        let assign4180_e5410: f64 = (1.0 + var_t0);
        let assign4180_e5412: f64 = (assign4180_e5410 + var_t1);
        let assign4180_e5413: f64 = (var_ndepcv_i * assign4180_e5412);
        var_ndepcv_i = assign4180_e5413;
        var_ndepcv_i_dn0 = ((var_ndepcv_i_dn0 * assign4180_e5412) + (var_ndepcv_i * (var_t0_dn0 + var_t1_dn0)));
        var_ndepcv_i_dn2 = ((var_ndepcv_i_dn2 * assign4180_e5412) + (var_ndepcv_i * (var_t0_dn2 + var_t1_dn2)));
        var_ndepcv_i_dn3 = ((var_ndepcv_i_dn3 * assign4180_e5412) + (var_ndepcv_i * (var_t0_dn3 + var_t1_dn3)));
        var_ndepcv_i_dn4 = ((var_ndepcv_i_dn4 * assign4180_e5412) + (var_ndepcv_i * (var_t0_dn4 + var_t1_dn4)));
        var_ndepcv_i_dn5 = ((var_ndepcv_i_dn5 * assign4180_e5412) + (var_ndepcv_i * (var_t0_dn5 + var_t1_dn5)));
        var_ndepcv_i_dn6 = ((var_ndepcv_i_dn6 * assign4180_e5412) + (var_ndepcv_i * (var_t0_dn6 + var_t1_dn6)));
        var_ndepcv_i_dn7 = ((var_ndepcv_i_dn7 * assign4180_e5412) + (var_ndepcv_i * (var_t0_dn7 + var_t1_dn7)));
        var_ndepcv_i_dn8 = ((var_ndepcv_i_dn8 * assign4180_e5412) + (var_ndepcv_i * (var_t0_dn8 + var_t1_dn8)));
        var_ndepcv_i_dn9 = ((var_ndepcv_i_dn9 * assign4180_e5412) + (var_ndepcv_i * (var_t0_dn9 + var_t1_dn9)));
        var_ndepcv_i_dn10 = ((var_ndepcv_i_dn10 * assign4180_e5412) + (var_ndepcv_i * (var_t0_dn10 + var_t1_dn10)));
        var_ndepcv_i_dn11 = ((var_ndepcv_i_dn11 * assign4180_e5412) + (var_ndepcv_i * (var_t0_dn11 + var_t1_dn11)));
        var_ndepcv_i_dn12 = ((var_ndepcv_i_dn12 * assign4180_e5412) + (var_ndepcv_i * (var_t0_dn12 + var_t1_dn12)));
        var_ndepcv_i_dn13 = ((var_ndepcv_i_dn13 * assign4180_e5412) + (var_ndepcv_i * (var_t0_dn13 + var_t1_dn13)));
        var_ndepcv_i_dn14 = ((var_ndepcv_i_dn14 * assign4180_e5412) + (var_ndepcv_i * (var_t0_dn14 + var_t1_dn14)));
        var_ndepcv_i_rv = 0.0;

        let assign4190_e5417: f64 = (var_inv_lact).powf(p.p121);
        let assign4190_e5420: f64 = (var_inv_llong).powf(p.p121);
        let assign4190_e5421: f64 = (assign4190_e5417 - assign4190_e5420);
        let assign4190_e5423: f64 = (assign4190_e5421).max(0.0);
        let assign4190_e5424: f64 = (p.p120 * assign4190_e5423);
        var_t0 = assign4190_e5424;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign4200_e5428: f64 = (var_inv_wact).powf(p.p123);
        let assign4200_e5431: f64 = (var_inv_wwide).powf(p.p123);
        let assign4200_e5432: f64 = (assign4200_e5428 - assign4200_e5431);
        let assign4200_e5434: f64 = (assign4200_e5432).max(0.0);
        let assign4200_e5435: f64 = (p.p122 * assign4200_e5434);
        let assign4200_e5439: f64 = (var_inv_wl).powf(p.p125);
        let assign4200_e5440: f64 = (p.p124 * assign4200_e5439);
        let assign4200_e5441: f64 = (assign4200_e5435 + assign4200_e5440);
        var_t1 = assign4200_e5441;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        let assign4210_e5445: f64 = (1.0 + var_t0);
        let assign4210_e5447: f64 = (assign4210_e5445 + var_t1);
        let assign4210_e5448: f64 = (var_vfb_i * assign4210_e5447);
        var_vfb_i = assign4210_e5448;
        var_vfb_i_dn0 = ((var_vfb_i_dn0 * assign4210_e5447) + (var_vfb_i * (var_t0_dn0 + var_t1_dn0)));
        var_vfb_i_dn2 = ((var_vfb_i_dn2 * assign4210_e5447) + (var_vfb_i * (var_t0_dn2 + var_t1_dn2)));
        var_vfb_i_dn3 = ((var_vfb_i_dn3 * assign4210_e5447) + (var_vfb_i * (var_t0_dn3 + var_t1_dn3)));
        var_vfb_i_dn4 = ((var_vfb_i_dn4 * assign4210_e5447) + (var_vfb_i * (var_t0_dn4 + var_t1_dn4)));
        var_vfb_i_dn5 = ((var_vfb_i_dn5 * assign4210_e5447) + (var_vfb_i * (var_t0_dn5 + var_t1_dn5)));
        var_vfb_i_dn6 = ((var_vfb_i_dn6 * assign4210_e5447) + (var_vfb_i * (var_t0_dn6 + var_t1_dn6)));
        var_vfb_i_dn7 = ((var_vfb_i_dn7 * assign4210_e5447) + (var_vfb_i * (var_t0_dn7 + var_t1_dn7)));
        var_vfb_i_dn8 = ((var_vfb_i_dn8 * assign4210_e5447) + (var_vfb_i * (var_t0_dn8 + var_t1_dn8)));
        var_vfb_i_dn9 = ((var_vfb_i_dn9 * assign4210_e5447) + (var_vfb_i * (var_t0_dn9 + var_t1_dn9)));
        var_vfb_i_dn10 = ((var_vfb_i_dn10 * assign4210_e5447) + (var_vfb_i * (var_t0_dn10 + var_t1_dn10)));
        var_vfb_i_dn11 = ((var_vfb_i_dn11 * assign4210_e5447) + (var_vfb_i * (var_t0_dn11 + var_t1_dn11)));
        var_vfb_i_dn12 = ((var_vfb_i_dn12 * assign4210_e5447) + (var_vfb_i * (var_t0_dn12 + var_t1_dn12)));
        var_vfb_i_dn13 = ((var_vfb_i_dn13 * assign4210_e5447) + (var_vfb_i * (var_t0_dn13 + var_t1_dn13)));
        var_vfb_i_dn14 = ((var_vfb_i_dn14 * assign4210_e5447) + (var_vfb_i * (var_t0_dn14 + var_t1_dn14)));
        var_vfb_i_rv = 0.0;

        let assign4220_e5452: f64 = (var_inv_lact).powf(p.p131);
        let assign4220_e5455: f64 = (var_inv_llong).powf(p.p131);
        let assign4220_e5456: f64 = (assign4220_e5452 - assign4220_e5455);
        let assign4220_e5458: f64 = (assign4220_e5456).max(0.0);
        let assign4220_e5459: f64 = (p.p130 * assign4220_e5458);
        var_t0 = assign4220_e5459;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign4230_e5463: f64 = (var_inv_wact).powf(p.p133);
        let assign4230_e5466: f64 = (var_inv_wwide).powf(p.p133);
        let assign4230_e5467: f64 = (assign4230_e5463 - assign4230_e5466);
        let assign4230_e5469: f64 = (assign4230_e5467).max(0.0);
        let assign4230_e5470: f64 = (p.p132 * assign4230_e5469);
        let assign4230_e5474: f64 = (var_inv_wl).powf(p.p135);
        let assign4230_e5475: f64 = (p.p134 * assign4230_e5474);
        let assign4230_e5476: f64 = (assign4230_e5470 + assign4230_e5475);
        var_t1 = assign4230_e5476;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        let assign4240_e5480: f64 = (1.0 + var_t0);
        let assign4240_e5482: f64 = (assign4240_e5480 + var_t1);
        let assign4240_e5483: f64 = (var_vfbcv_i * assign4240_e5482);
        var_vfbcv_i = assign4240_e5483;
        var_vfbcv_i_dn0 = ((var_vfbcv_i_dn0 * assign4240_e5482) + (var_vfbcv_i * (var_t0_dn0 + var_t1_dn0)));
        var_vfbcv_i_dn2 = ((var_vfbcv_i_dn2 * assign4240_e5482) + (var_vfbcv_i * (var_t0_dn2 + var_t1_dn2)));
        var_vfbcv_i_dn3 = ((var_vfbcv_i_dn3 * assign4240_e5482) + (var_vfbcv_i * (var_t0_dn3 + var_t1_dn3)));
        var_vfbcv_i_dn4 = ((var_vfbcv_i_dn4 * assign4240_e5482) + (var_vfbcv_i * (var_t0_dn4 + var_t1_dn4)));
        var_vfbcv_i_dn5 = ((var_vfbcv_i_dn5 * assign4240_e5482) + (var_vfbcv_i * (var_t0_dn5 + var_t1_dn5)));
        var_vfbcv_i_dn6 = ((var_vfbcv_i_dn6 * assign4240_e5482) + (var_vfbcv_i * (var_t0_dn6 + var_t1_dn6)));
        var_vfbcv_i_dn7 = ((var_vfbcv_i_dn7 * assign4240_e5482) + (var_vfbcv_i * (var_t0_dn7 + var_t1_dn7)));
        var_vfbcv_i_dn8 = ((var_vfbcv_i_dn8 * assign4240_e5482) + (var_vfbcv_i * (var_t0_dn8 + var_t1_dn8)));
        var_vfbcv_i_dn9 = ((var_vfbcv_i_dn9 * assign4240_e5482) + (var_vfbcv_i * (var_t0_dn9 + var_t1_dn9)));
        var_vfbcv_i_dn10 = ((var_vfbcv_i_dn10 * assign4240_e5482) + (var_vfbcv_i * (var_t0_dn10 + var_t1_dn10)));
        var_vfbcv_i_dn11 = ((var_vfbcv_i_dn11 * assign4240_e5482) + (var_vfbcv_i * (var_t0_dn11 + var_t1_dn11)));
        var_vfbcv_i_dn12 = ((var_vfbcv_i_dn12 * assign4240_e5482) + (var_vfbcv_i * (var_t0_dn12 + var_t1_dn12)));
        var_vfbcv_i_dn13 = ((var_vfbcv_i_dn13 * assign4240_e5482) + (var_vfbcv_i * (var_t0_dn13 + var_t1_dn13)));
        var_vfbcv_i_dn14 = ((var_vfbcv_i_dn14 * assign4240_e5482) + (var_vfbcv_i * (var_t0_dn14 + var_t1_dn14)));
        var_vfbcv_i_rv = 0.0;

        let assign4250_e5487: f64 = (var_inv_lact).powf(p.p264);
        let assign4250_e5490: f64 = (var_inv_llong).powf(p.p264);
        let assign4250_e5491: f64 = (assign4250_e5487 - assign4250_e5490);
        let assign4250_e5493: f64 = (assign4250_e5491).max(0.0);
        let assign4250_e5494: f64 = (p.p263 * assign4250_e5493);
        var_t0 = assign4250_e5494;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign4260_e5498: f64 = (var_inv_w).powf(p.p266);
        let assign4260_e5501: f64 = (var_inv_wwide).powf(p.p266);
        let assign4260_e5502: f64 = (assign4260_e5498 - assign4260_e5501);
        let assign4260_e5504: f64 = (assign4260_e5502).max(0.0);
        let assign4260_e5505: f64 = (p.p265 * assign4260_e5504);
        let assign4260_e5509: f64 = (var_inv_wl).powf(p.p268);
        let assign4260_e5510: f64 = (p.p267 * assign4260_e5509);
        let assign4260_e5511: f64 = (assign4260_e5505 + assign4260_e5510);
        var_t1 = assign4260_e5511;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        let assign4270_e5515: f64 = (1.0 + var_t0);
        let assign4270_e5517: f64 = (assign4270_e5515 + var_t1);
        let assign4270_e5518: f64 = (var_vsatcv_i * assign4270_e5517);
        var_vsatcv_i = assign4270_e5518;
        var_vsatcv_i_dn0 = ((var_vsatcv_i_dn0 * assign4270_e5517) + (var_vsatcv_i * (var_t0_dn0 + var_t1_dn0)));
        var_vsatcv_i_dn2 = ((var_vsatcv_i_dn2 * assign4270_e5517) + (var_vsatcv_i * (var_t0_dn2 + var_t1_dn2)));
        var_vsatcv_i_dn3 = ((var_vsatcv_i_dn3 * assign4270_e5517) + (var_vsatcv_i * (var_t0_dn3 + var_t1_dn3)));
        var_vsatcv_i_dn4 = ((var_vsatcv_i_dn4 * assign4270_e5517) + (var_vsatcv_i * (var_t0_dn4 + var_t1_dn4)));
        var_vsatcv_i_dn5 = ((var_vsatcv_i_dn5 * assign4270_e5517) + (var_vsatcv_i * (var_t0_dn5 + var_t1_dn5)));
        var_vsatcv_i_dn6 = ((var_vsatcv_i_dn6 * assign4270_e5517) + (var_vsatcv_i * (var_t0_dn6 + var_t1_dn6)));
        var_vsatcv_i_dn7 = ((var_vsatcv_i_dn7 * assign4270_e5517) + (var_vsatcv_i * (var_t0_dn7 + var_t1_dn7)));
        var_vsatcv_i_dn8 = ((var_vsatcv_i_dn8 * assign4270_e5517) + (var_vsatcv_i * (var_t0_dn8 + var_t1_dn8)));
        var_vsatcv_i_dn9 = ((var_vsatcv_i_dn9 * assign4270_e5517) + (var_vsatcv_i * (var_t0_dn9 + var_t1_dn9)));
        var_vsatcv_i_dn10 = ((var_vsatcv_i_dn10 * assign4270_e5517) + (var_vsatcv_i * (var_t0_dn10 + var_t1_dn10)));
        var_vsatcv_i_dn11 = ((var_vsatcv_i_dn11 * assign4270_e5517) + (var_vsatcv_i * (var_t0_dn11 + var_t1_dn11)));
        var_vsatcv_i_dn12 = ((var_vsatcv_i_dn12 * assign4270_e5517) + (var_vsatcv_i * (var_t0_dn12 + var_t1_dn12)));
        var_vsatcv_i_dn13 = ((var_vsatcv_i_dn13 * assign4270_e5517) + (var_vsatcv_i * (var_t0_dn13 + var_t1_dn13)));
        var_vsatcv_i_dn14 = ((var_vsatcv_i_dn14 * assign4270_e5517) + (var_vsatcv_i * (var_t0_dn14 + var_t1_dn14)));
        var_vsatcv_i_rv = 0.0;

        let assign4280_e5524: f64 = (var_inv_lact).powf(p.p353);
        let assign4280_e5527: f64 = (var_inv_llong).powf(p.p353);
        let assign4280_e5528: f64 = (assign4280_e5524 - assign4280_e5527);
        let assign4280_e5530: f64 = (assign4280_e5528).max(0.0);
        let assign4280_e5531: f64 = (p.p352 * assign4280_e5530);
        let assign4280_e5532: f64 = (1.0 + assign4280_e5531);
        let assign4280_e5533: f64 = (var_pclmcv_i * assign4280_e5532);
        var_pclmcv_i = assign4280_e5533;
        var_pclmcv_i_rv = 0.0;

        let assign4290_e5536: f64 = (var_pclmcv_i).max(0.0);
        var_pclmcv_i = assign4290_e5536;
        var_pclmcv_i_rv = 0.0;

        let assign4300_e5540: f64 = (var_inv_l).powf(p.p187);
        let assign4300_e5543: f64 = (var_inv_llong).powf(p.p187);
        let assign4300_e5544: f64 = (assign4300_e5540 - assign4300_e5543);
        let assign4300_e5546: f64 = (assign4300_e5544).max(0.0);
        let assign4300_e5547: f64 = (p.p186 * assign4300_e5546);
        var_t0 = assign4300_e5547;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        *var_agidl_i_slot = var_agidl_i;
        *var_agidl_i_rv_slot = var_agidl_i_rv;
        *var_agisl_i_slot = var_agisl_i;
        *var_agisl_i_rv_slot = var_agisl_i_rv;
        *var_aigc_i_slot = var_aigc_i;
        *var_aigc_i_rv_slot = var_aigc_i_rv;
        *var_aigd_i_slot = var_aigd_i;
        *var_aigd_i_rv_slot = var_aigd_i_rv;
        *var_aigs_i_slot = var_aigs_i;
        *var_aigs_i_rv_slot = var_aigs_i_rv;
        *var_beta0_i_slot = var_beta0_i;
        *var_beta0_i_dn0_slot = var_beta0_i_dn0;
        *var_beta0_i_dn10_slot = var_beta0_i_dn10;
        *var_beta0_i_dn11_slot = var_beta0_i_dn11;
        *var_beta0_i_dn12_slot = var_beta0_i_dn12;
        *var_beta0_i_dn13_slot = var_beta0_i_dn13;
        *var_beta0_i_dn14_slot = var_beta0_i_dn14;
        *var_beta0_i_dn2_slot = var_beta0_i_dn2;
        *var_beta0_i_dn3_slot = var_beta0_i_dn3;
        *var_beta0_i_dn4_slot = var_beta0_i_dn4;
        *var_beta0_i_dn5_slot = var_beta0_i_dn5;
        *var_beta0_i_dn6_slot = var_beta0_i_dn6;
        *var_beta0_i_dn7_slot = var_beta0_i_dn7;
        *var_beta0_i_dn8_slot = var_beta0_i_dn8;
        *var_beta0_i_dn9_slot = var_beta0_i_dn9;
        *var_beta0_i_rv_slot = var_beta0_i_rv;
        *var_beta1_i_slot = var_beta1_i;
        *var_beta1_i_dn0_slot = var_beta1_i_dn0;
        *var_beta1_i_dn10_slot = var_beta1_i_dn10;
        *var_beta1_i_dn11_slot = var_beta1_i_dn11;
        *var_beta1_i_dn12_slot = var_beta1_i_dn12;
        *var_beta1_i_dn13_slot = var_beta1_i_dn13;
        *var_beta1_i_dn14_slot = var_beta1_i_dn14;
        *var_beta1_i_dn2_slot = var_beta1_i_dn2;
        *var_beta1_i_dn3_slot = var_beta1_i_dn3;
        *var_beta1_i_dn4_slot = var_beta1_i_dn4;
        *var_beta1_i_dn5_slot = var_beta1_i_dn5;
        *var_beta1_i_dn6_slot = var_beta1_i_dn6;
        *var_beta1_i_dn7_slot = var_beta1_i_dn7;
        *var_beta1_i_dn8_slot = var_beta1_i_dn8;
        *var_beta1_i_dn9_slot = var_beta1_i_dn9;
        *var_beta1_i_rv_slot = var_beta1_i_rv;
        *var_beta2_i_slot = var_beta2_i;
        *var_beta2_i_dn0_slot = var_beta2_i_dn0;
        *var_beta2_i_dn10_slot = var_beta2_i_dn10;
        *var_beta2_i_dn11_slot = var_beta2_i_dn11;
        *var_beta2_i_dn12_slot = var_beta2_i_dn12;
        *var_beta2_i_dn13_slot = var_beta2_i_dn13;
        *var_beta2_i_dn14_slot = var_beta2_i_dn14;
        *var_beta2_i_dn2_slot = var_beta2_i_dn2;
        *var_beta2_i_dn3_slot = var_beta2_i_dn3;
        *var_beta2_i_dn4_slot = var_beta2_i_dn4;
        *var_beta2_i_dn5_slot = var_beta2_i_dn5;
        *var_beta2_i_dn6_slot = var_beta2_i_dn6;
        *var_beta2_i_dn7_slot = var_beta2_i_dn7;
        *var_beta2_i_dn8_slot = var_beta2_i_dn8;
        *var_beta2_i_dn9_slot = var_beta2_i_dn9;
        *var_beta2_i_rv_slot = var_beta2_i_rv;
        *var_ndepcv_i_slot = var_ndepcv_i;
        *var_ndepcv_i_dn0_slot = var_ndepcv_i_dn0;
        *var_ndepcv_i_dn10_slot = var_ndepcv_i_dn10;
        *var_ndepcv_i_dn11_slot = var_ndepcv_i_dn11;
        *var_ndepcv_i_dn12_slot = var_ndepcv_i_dn12;
        *var_ndepcv_i_dn13_slot = var_ndepcv_i_dn13;
        *var_ndepcv_i_dn14_slot = var_ndepcv_i_dn14;
        *var_ndepcv_i_dn2_slot = var_ndepcv_i_dn2;
        *var_ndepcv_i_dn3_slot = var_ndepcv_i_dn3;
        *var_ndepcv_i_dn4_slot = var_ndepcv_i_dn4;
        *var_ndepcv_i_dn5_slot = var_ndepcv_i_dn5;
        *var_ndepcv_i_dn6_slot = var_ndepcv_i_dn6;
        *var_ndepcv_i_dn7_slot = var_ndepcv_i_dn7;
        *var_ndepcv_i_dn8_slot = var_ndepcv_i_dn8;
        *var_ndepcv_i_dn9_slot = var_ndepcv_i_dn9;
        *var_ndepcv_i_rv_slot = var_ndepcv_i_rv;
        *var_pclmcv_i_slot = var_pclmcv_i;
        *var_pclmcv_i_rv_slot = var_pclmcv_i_rv;
        *var_pigcd_i_slot = var_pigcd_i;
        *var_pigcd_i_rv_slot = var_pigcd_i_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn13_slot = var_t0_dn13;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_vfb_i_slot = var_vfb_i;
        *var_vfb_i_dn0_slot = var_vfb_i_dn0;
        *var_vfb_i_dn10_slot = var_vfb_i_dn10;
        *var_vfb_i_dn11_slot = var_vfb_i_dn11;
        *var_vfb_i_dn12_slot = var_vfb_i_dn12;
        *var_vfb_i_dn13_slot = var_vfb_i_dn13;
        *var_vfb_i_dn14_slot = var_vfb_i_dn14;
        *var_vfb_i_dn2_slot = var_vfb_i_dn2;
        *var_vfb_i_dn3_slot = var_vfb_i_dn3;
        *var_vfb_i_dn4_slot = var_vfb_i_dn4;
        *var_vfb_i_dn5_slot = var_vfb_i_dn5;
        *var_vfb_i_dn6_slot = var_vfb_i_dn6;
        *var_vfb_i_dn7_slot = var_vfb_i_dn7;
        *var_vfb_i_dn8_slot = var_vfb_i_dn8;
        *var_vfb_i_dn9_slot = var_vfb_i_dn9;
        *var_vfb_i_rv_slot = var_vfb_i_rv;
        *var_vfbcv_i_slot = var_vfbcv_i;
        *var_vfbcv_i_dn0_slot = var_vfbcv_i_dn0;
        *var_vfbcv_i_dn10_slot = var_vfbcv_i_dn10;
        *var_vfbcv_i_dn11_slot = var_vfbcv_i_dn11;
        *var_vfbcv_i_dn12_slot = var_vfbcv_i_dn12;
        *var_vfbcv_i_dn13_slot = var_vfbcv_i_dn13;
        *var_vfbcv_i_dn14_slot = var_vfbcv_i_dn14;
        *var_vfbcv_i_dn2_slot = var_vfbcv_i_dn2;
        *var_vfbcv_i_dn3_slot = var_vfbcv_i_dn3;
        *var_vfbcv_i_dn4_slot = var_vfbcv_i_dn4;
        *var_vfbcv_i_dn5_slot = var_vfbcv_i_dn5;
        *var_vfbcv_i_dn6_slot = var_vfbcv_i_dn6;
        *var_vfbcv_i_dn7_slot = var_vfbcv_i_dn7;
        *var_vfbcv_i_dn8_slot = var_vfbcv_i_dn8;
        *var_vfbcv_i_dn9_slot = var_vfbcv_i_dn9;
        *var_vfbcv_i_rv_slot = var_vfbcv_i_rv;
        *var_vsatcv_i_slot = var_vsatcv_i;
        *var_vsatcv_i_dn0_slot = var_vsatcv_i_dn0;
        *var_vsatcv_i_dn10_slot = var_vsatcv_i_dn10;
        *var_vsatcv_i_dn11_slot = var_vsatcv_i_dn11;
        *var_vsatcv_i_dn12_slot = var_vsatcv_i_dn12;
        *var_vsatcv_i_dn13_slot = var_vsatcv_i_dn13;
        *var_vsatcv_i_dn14_slot = var_vsatcv_i_dn14;
        *var_vsatcv_i_dn2_slot = var_vsatcv_i_dn2;
        *var_vsatcv_i_dn3_slot = var_vsatcv_i_dn3;
        *var_vsatcv_i_dn4_slot = var_vsatcv_i_dn4;
        *var_vsatcv_i_dn5_slot = var_vsatcv_i_dn5;
        *var_vsatcv_i_dn6_slot = var_vsatcv_i_dn6;
        *var_vsatcv_i_dn7_slot = var_vsatcv_i_dn7;
        *var_vsatcv_i_dn8_slot = var_vsatcv_i_dn8;
        *var_vsatcv_i_dn9_slot = var_vsatcv_i_dn9;
        *var_vsatcv_i_rv_slot = var_vsatcv_i_rv;
    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        var_inv_l: f64,
        var_inv_llong: f64,
        var_inv_w: f64,
        var_inv_wl: f64,
        var_inv_wwide: f64,
        var_leff: f64,
        var_at_i_slot: &mut f64,
        var_at_i_rv_slot: &mut f64,
        var_beta1_i_slot: &mut f64,
        var_beta1_i_dn0_slot: &mut f64,
        var_beta1_i_dn10_slot: &mut f64,
        var_beta1_i_dn11_slot: &mut f64,
        var_beta1_i_dn12_slot: &mut f64,
        var_beta1_i_dn13_slot: &mut f64,
        var_beta1_i_dn14_slot: &mut f64,
        var_beta1_i_dn2_slot: &mut f64,
        var_beta1_i_dn3_slot: &mut f64,
        var_beta1_i_dn4_slot: &mut f64,
        var_beta1_i_dn5_slot: &mut f64,
        var_beta1_i_dn6_slot: &mut f64,
        var_beta1_i_dn7_slot: &mut f64,
        var_beta1_i_dn8_slot: &mut f64,
        var_beta1_i_dn9_slot: &mut f64,
        var_beta1_i_rv_slot: &mut f64,
        var_eu_i_slot: &mut f64,
        var_eu_i_dn0_slot: &mut f64,
        var_eu_i_dn10_slot: &mut f64,
        var_eu_i_dn11_slot: &mut f64,
        var_eu_i_dn12_slot: &mut f64,
        var_eu_i_dn13_slot: &mut f64,
        var_eu_i_dn14_slot: &mut f64,
        var_eu_i_dn2_slot: &mut f64,
        var_eu_i_dn3_slot: &mut f64,
        var_eu_i_dn4_slot: &mut f64,
        var_eu_i_dn5_slot: &mut f64,
        var_eu_i_dn6_slot: &mut f64,
        var_eu_i_dn7_slot: &mut f64,
        var_eu_i_dn8_slot: &mut f64,
        var_eu_i_dn9_slot: &mut f64,
        var_eu_i_rv_slot: &mut f64,
        var_guard40_slot: &mut f64,
        var_guard40_rv_slot: &mut f64,
        var_guard41_slot: &mut f64,
        var_guard41_rv_slot: &mut f64,
        var_guard42_slot: &mut f64,
        var_guard42_rv_slot: &mut f64,
        var_guard43_slot: &mut f64,
        var_guard43_rv_slot: &mut f64,
        var_guard44_slot: &mut f64,
        var_guard44_rv_slot: &mut f64,
        var_guard45_slot: &mut f64,
        var_guard45_rv_slot: &mut f64,
        var_guard67_slot: &mut f64,
        var_guard67_rv_slot: &mut f64,
        var_guard68_slot: &mut f64,
        var_guard68_rv_slot: &mut f64,
        var_guard69_slot: &mut f64,
        var_guard69_rv_slot: &mut f64,
        var_guard70_slot: &mut f64,
        var_guard70_rv_slot: &mut f64,
        var_guard71_slot: &mut f64,
        var_guard71_rv_slot: &mut f64,
        var_guard72_slot: &mut f64,
        var_guard72_rv_slot: &mut f64,
        var_guard73_slot: &mut f64,
        var_guard73_rv_slot: &mut f64,
        var_guard74_slot: &mut f64,
        var_guard74_rv_slot: &mut f64,
        var_guard75_slot: &mut f64,
        var_guard75_rv_slot: &mut f64,
        var_guard76_slot: &mut f64,
        var_guard76_rv_slot: &mut f64,
        var_k1_i_slot: &mut f64,
        var_k1_i_dn0_slot: &mut f64,
        var_k1_i_dn10_slot: &mut f64,
        var_k1_i_dn11_slot: &mut f64,
        var_k1_i_dn12_slot: &mut f64,
        var_k1_i_dn13_slot: &mut f64,
        var_k1_i_dn14_slot: &mut f64,
        var_k1_i_dn2_slot: &mut f64,
        var_k1_i_dn3_slot: &mut f64,
        var_k1_i_dn4_slot: &mut f64,
        var_k1_i_dn5_slot: &mut f64,
        var_k1_i_dn6_slot: &mut f64,
        var_k1_i_dn7_slot: &mut f64,
        var_k1_i_dn8_slot: &mut f64,
        var_k1_i_dn9_slot: &mut f64,
        var_k1_i_rv_slot: &mut f64,
        var_k2_i_slot: &mut f64,
        var_k2_i_dn0_slot: &mut f64,
        var_k2_i_dn10_slot: &mut f64,
        var_k2_i_dn11_slot: &mut f64,
        var_k2_i_dn12_slot: &mut f64,
        var_k2_i_dn13_slot: &mut f64,
        var_k2_i_dn14_slot: &mut f64,
        var_k2_i_dn2_slot: &mut f64,
        var_k2_i_dn3_slot: &mut f64,
        var_k2_i_dn4_slot: &mut f64,
        var_k2_i_dn5_slot: &mut f64,
        var_k2_i_dn6_slot: &mut f64,
        var_k2_i_dn7_slot: &mut f64,
        var_k2_i_dn8_slot: &mut f64,
        var_k2_i_dn9_slot: &mut f64,
        var_k2_i_rv_slot: &mut f64,
        var_lh1_slot: &mut f64,
        var_lh1_rv_slot: &mut f64,
        var_m0_i_slot: &mut f64,
        var_m0_i_rv_slot: &mut f64,
        var_prwb_i_slot: &mut f64,
        var_prwb_i_rv_slot: &mut f64,
        var_ptwgt_i_slot: &mut f64,
        var_ptwgt_i_rv_slot: &mut f64,
        var_rdsw_i_slot: &mut f64,
        var_rdsw_i_rv_slot: &mut f64,
        var_rdw_i_slot: &mut f64,
        var_rdw_i_rv_slot: &mut f64,
        var_rsw_i_slot: &mut f64,
        var_rsw_i_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_u0_i_slot: &mut f64,
        var_u0_i_rv_slot: &mut f64,
        var_ua1_i_slot: &mut f64,
        var_ua1_i_rv_slot: &mut f64,
        var_ua_i_slot: &mut f64,
        var_ua_i_dn0_slot: &mut f64,
        var_ua_i_dn10_slot: &mut f64,
        var_ua_i_dn11_slot: &mut f64,
        var_ua_i_dn12_slot: &mut f64,
        var_ua_i_dn13_slot: &mut f64,
        var_ua_i_dn14_slot: &mut f64,
        var_ua_i_dn2_slot: &mut f64,
        var_ua_i_dn3_slot: &mut f64,
        var_ua_i_dn4_slot: &mut f64,
        var_ua_i_dn5_slot: &mut f64,
        var_ua_i_dn6_slot: &mut f64,
        var_ua_i_dn7_slot: &mut f64,
        var_ua_i_dn8_slot: &mut f64,
        var_ua_i_dn9_slot: &mut f64,
        var_ua_i_rv_slot: &mut f64,
        var_ucs_i_slot: &mut f64,
        var_ucs_i_rv_slot: &mut f64,
        var_ucsr_i_slot: &mut f64,
        var_ucsr_i_rv_slot: &mut f64,
        var_ud1_i_slot: &mut f64,
        var_ud1_i_rv_slot: &mut f64,
        var_ud_i_slot: &mut f64,
        var_ud_i_dn0_slot: &mut f64,
        var_ud_i_dn10_slot: &mut f64,
        var_ud_i_dn11_slot: &mut f64,
        var_ud_i_dn12_slot: &mut f64,
        var_ud_i_dn13_slot: &mut f64,
        var_ud_i_dn14_slot: &mut f64,
        var_ud_i_dn2_slot: &mut f64,
        var_ud_i_dn3_slot: &mut f64,
        var_ud_i_dn4_slot: &mut f64,
        var_ud_i_dn5_slot: &mut f64,
        var_ud_i_dn6_slot: &mut f64,
        var_ud_i_dn7_slot: &mut f64,
        var_ud_i_dn8_slot: &mut f64,
        var_ud_i_dn9_slot: &mut f64,
        var_ud_i_rv_slot: &mut f64,
        var_ute_i_slot: &mut f64,
        var_ute_i_rv_slot: &mut f64,
    ) {
        let mut var_at_i: f64 = *var_at_i_slot;
        let mut var_at_i_rv: f64 = *var_at_i_rv_slot;
        let mut var_beta1_i: f64 = *var_beta1_i_slot;
        let mut var_beta1_i_dn0: f64 = *var_beta1_i_dn0_slot;
        let mut var_beta1_i_dn10: f64 = *var_beta1_i_dn10_slot;
        let mut var_beta1_i_dn11: f64 = *var_beta1_i_dn11_slot;
        let mut var_beta1_i_dn12: f64 = *var_beta1_i_dn12_slot;
        let mut var_beta1_i_dn13: f64 = *var_beta1_i_dn13_slot;
        let mut var_beta1_i_dn14: f64 = *var_beta1_i_dn14_slot;
        let mut var_beta1_i_dn2: f64 = *var_beta1_i_dn2_slot;
        let mut var_beta1_i_dn3: f64 = *var_beta1_i_dn3_slot;
        let mut var_beta1_i_dn4: f64 = *var_beta1_i_dn4_slot;
        let mut var_beta1_i_dn5: f64 = *var_beta1_i_dn5_slot;
        let mut var_beta1_i_dn6: f64 = *var_beta1_i_dn6_slot;
        let mut var_beta1_i_dn7: f64 = *var_beta1_i_dn7_slot;
        let mut var_beta1_i_dn8: f64 = *var_beta1_i_dn8_slot;
        let mut var_beta1_i_dn9: f64 = *var_beta1_i_dn9_slot;
        let mut var_beta1_i_rv: f64 = *var_beta1_i_rv_slot;
        let mut var_eu_i: f64 = *var_eu_i_slot;
        let mut var_eu_i_dn0: f64 = *var_eu_i_dn0_slot;
        let mut var_eu_i_dn10: f64 = *var_eu_i_dn10_slot;
        let mut var_eu_i_dn11: f64 = *var_eu_i_dn11_slot;
        let mut var_eu_i_dn12: f64 = *var_eu_i_dn12_slot;
        let mut var_eu_i_dn13: f64 = *var_eu_i_dn13_slot;
        let mut var_eu_i_dn14: f64 = *var_eu_i_dn14_slot;
        let mut var_eu_i_dn2: f64 = *var_eu_i_dn2_slot;
        let mut var_eu_i_dn3: f64 = *var_eu_i_dn3_slot;
        let mut var_eu_i_dn4: f64 = *var_eu_i_dn4_slot;
        let mut var_eu_i_dn5: f64 = *var_eu_i_dn5_slot;
        let mut var_eu_i_dn6: f64 = *var_eu_i_dn6_slot;
        let mut var_eu_i_dn7: f64 = *var_eu_i_dn7_slot;
        let mut var_eu_i_dn8: f64 = *var_eu_i_dn8_slot;
        let mut var_eu_i_dn9: f64 = *var_eu_i_dn9_slot;
        let mut var_eu_i_rv: f64 = *var_eu_i_rv_slot;
        let mut var_guard40: f64 = *var_guard40_slot;
        let mut var_guard40_rv: f64 = *var_guard40_rv_slot;
        let mut var_guard41: f64 = *var_guard41_slot;
        let mut var_guard41_rv: f64 = *var_guard41_rv_slot;
        let mut var_guard42: f64 = *var_guard42_slot;
        let mut var_guard42_rv: f64 = *var_guard42_rv_slot;
        let mut var_guard43: f64 = *var_guard43_slot;
        let mut var_guard43_rv: f64 = *var_guard43_rv_slot;
        let mut var_guard44: f64 = *var_guard44_slot;
        let mut var_guard44_rv: f64 = *var_guard44_rv_slot;
        let mut var_guard45: f64 = *var_guard45_slot;
        let mut var_guard45_rv: f64 = *var_guard45_rv_slot;
        let mut var_guard67: f64 = *var_guard67_slot;
        let mut var_guard67_rv: f64 = *var_guard67_rv_slot;
        let mut var_guard68: f64 = *var_guard68_slot;
        let mut var_guard68_rv: f64 = *var_guard68_rv_slot;
        let mut var_guard69: f64 = *var_guard69_slot;
        let mut var_guard69_rv: f64 = *var_guard69_rv_slot;
        let mut var_guard70: f64 = *var_guard70_slot;
        let mut var_guard70_rv: f64 = *var_guard70_rv_slot;
        let mut var_guard71: f64 = *var_guard71_slot;
        let mut var_guard71_rv: f64 = *var_guard71_rv_slot;
        let mut var_guard72: f64 = *var_guard72_slot;
        let mut var_guard72_rv: f64 = *var_guard72_rv_slot;
        let mut var_guard73: f64 = *var_guard73_slot;
        let mut var_guard73_rv: f64 = *var_guard73_rv_slot;
        let mut var_guard74: f64 = *var_guard74_slot;
        let mut var_guard74_rv: f64 = *var_guard74_rv_slot;
        let mut var_guard75: f64 = *var_guard75_slot;
        let mut var_guard75_rv: f64 = *var_guard75_rv_slot;
        let mut var_guard76: f64 = *var_guard76_slot;
        let mut var_guard76_rv: f64 = *var_guard76_rv_slot;
        let mut var_k1_i: f64 = *var_k1_i_slot;
        let mut var_k1_i_dn0: f64 = *var_k1_i_dn0_slot;
        let mut var_k1_i_dn10: f64 = *var_k1_i_dn10_slot;
        let mut var_k1_i_dn11: f64 = *var_k1_i_dn11_slot;
        let mut var_k1_i_dn12: f64 = *var_k1_i_dn12_slot;
        let mut var_k1_i_dn13: f64 = *var_k1_i_dn13_slot;
        let mut var_k1_i_dn14: f64 = *var_k1_i_dn14_slot;
        let mut var_k1_i_dn2: f64 = *var_k1_i_dn2_slot;
        let mut var_k1_i_dn3: f64 = *var_k1_i_dn3_slot;
        let mut var_k1_i_dn4: f64 = *var_k1_i_dn4_slot;
        let mut var_k1_i_dn5: f64 = *var_k1_i_dn5_slot;
        let mut var_k1_i_dn6: f64 = *var_k1_i_dn6_slot;
        let mut var_k1_i_dn7: f64 = *var_k1_i_dn7_slot;
        let mut var_k1_i_dn8: f64 = *var_k1_i_dn8_slot;
        let mut var_k1_i_dn9: f64 = *var_k1_i_dn9_slot;
        let mut var_k1_i_rv: f64 = *var_k1_i_rv_slot;
        let mut var_k2_i: f64 = *var_k2_i_slot;
        let mut var_k2_i_dn0: f64 = *var_k2_i_dn0_slot;
        let mut var_k2_i_dn10: f64 = *var_k2_i_dn10_slot;
        let mut var_k2_i_dn11: f64 = *var_k2_i_dn11_slot;
        let mut var_k2_i_dn12: f64 = *var_k2_i_dn12_slot;
        let mut var_k2_i_dn13: f64 = *var_k2_i_dn13_slot;
        let mut var_k2_i_dn14: f64 = *var_k2_i_dn14_slot;
        let mut var_k2_i_dn2: f64 = *var_k2_i_dn2_slot;
        let mut var_k2_i_dn3: f64 = *var_k2_i_dn3_slot;
        let mut var_k2_i_dn4: f64 = *var_k2_i_dn4_slot;
        let mut var_k2_i_dn5: f64 = *var_k2_i_dn5_slot;
        let mut var_k2_i_dn6: f64 = *var_k2_i_dn6_slot;
        let mut var_k2_i_dn7: f64 = *var_k2_i_dn7_slot;
        let mut var_k2_i_dn8: f64 = *var_k2_i_dn8_slot;
        let mut var_k2_i_dn9: f64 = *var_k2_i_dn9_slot;
        let mut var_k2_i_rv: f64 = *var_k2_i_rv_slot;
        let mut var_lh1: f64 = *var_lh1_slot;
        let mut var_lh1_rv: f64 = *var_lh1_rv_slot;
        let mut var_m0_i: f64 = *var_m0_i_slot;
        let mut var_m0_i_rv: f64 = *var_m0_i_rv_slot;
        let mut var_prwb_i: f64 = *var_prwb_i_slot;
        let mut var_prwb_i_rv: f64 = *var_prwb_i_rv_slot;
        let mut var_ptwgt_i: f64 = *var_ptwgt_i_slot;
        let mut var_ptwgt_i_rv: f64 = *var_ptwgt_i_rv_slot;
        let mut var_rdsw_i: f64 = *var_rdsw_i_slot;
        let mut var_rdsw_i_rv: f64 = *var_rdsw_i_rv_slot;
        let mut var_rdw_i: f64 = *var_rdw_i_slot;
        let mut var_rdw_i_rv: f64 = *var_rdw_i_rv_slot;
        let mut var_rsw_i: f64 = *var_rsw_i_slot;
        let mut var_rsw_i_rv: f64 = *var_rsw_i_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_u0_i: f64 = *var_u0_i_slot;
        let mut var_u0_i_rv: f64 = *var_u0_i_rv_slot;
        let mut var_ua1_i: f64 = *var_ua1_i_slot;
        let mut var_ua1_i_rv: f64 = *var_ua1_i_rv_slot;
        let mut var_ua_i: f64 = *var_ua_i_slot;
        let mut var_ua_i_dn0: f64 = *var_ua_i_dn0_slot;
        let mut var_ua_i_dn10: f64 = *var_ua_i_dn10_slot;
        let mut var_ua_i_dn11: f64 = *var_ua_i_dn11_slot;
        let mut var_ua_i_dn12: f64 = *var_ua_i_dn12_slot;
        let mut var_ua_i_dn13: f64 = *var_ua_i_dn13_slot;
        let mut var_ua_i_dn14: f64 = *var_ua_i_dn14_slot;
        let mut var_ua_i_dn2: f64 = *var_ua_i_dn2_slot;
        let mut var_ua_i_dn3: f64 = *var_ua_i_dn3_slot;
        let mut var_ua_i_dn4: f64 = *var_ua_i_dn4_slot;
        let mut var_ua_i_dn5: f64 = *var_ua_i_dn5_slot;
        let mut var_ua_i_dn6: f64 = *var_ua_i_dn6_slot;
        let mut var_ua_i_dn7: f64 = *var_ua_i_dn7_slot;
        let mut var_ua_i_dn8: f64 = *var_ua_i_dn8_slot;
        let mut var_ua_i_dn9: f64 = *var_ua_i_dn9_slot;
        let mut var_ua_i_rv: f64 = *var_ua_i_rv_slot;
        let mut var_ucs_i: f64 = *var_ucs_i_slot;
        let mut var_ucs_i_rv: f64 = *var_ucs_i_rv_slot;
        let mut var_ucsr_i: f64 = *var_ucsr_i_slot;
        let mut var_ucsr_i_rv: f64 = *var_ucsr_i_rv_slot;
        let mut var_ud1_i: f64 = *var_ud1_i_slot;
        let mut var_ud1_i_rv: f64 = *var_ud1_i_rv_slot;
        let mut var_ud_i: f64 = *var_ud_i_slot;
        let mut var_ud_i_dn0: f64 = *var_ud_i_dn0_slot;
        let mut var_ud_i_dn10: f64 = *var_ud_i_dn10_slot;
        let mut var_ud_i_dn11: f64 = *var_ud_i_dn11_slot;
        let mut var_ud_i_dn12: f64 = *var_ud_i_dn12_slot;
        let mut var_ud_i_dn13: f64 = *var_ud_i_dn13_slot;
        let mut var_ud_i_dn14: f64 = *var_ud_i_dn14_slot;
        let mut var_ud_i_dn2: f64 = *var_ud_i_dn2_slot;
        let mut var_ud_i_dn3: f64 = *var_ud_i_dn3_slot;
        let mut var_ud_i_dn4: f64 = *var_ud_i_dn4_slot;
        let mut var_ud_i_dn5: f64 = *var_ud_i_dn5_slot;
        let mut var_ud_i_dn6: f64 = *var_ud_i_dn6_slot;
        let mut var_ud_i_dn7: f64 = *var_ud_i_dn7_slot;
        let mut var_ud_i_dn8: f64 = *var_ud_i_dn8_slot;
        let mut var_ud_i_dn9: f64 = *var_ud_i_dn9_slot;
        let mut var_ud_i_rv: f64 = *var_ud_i_rv_slot;
        let mut var_ute_i: f64 = *var_ute_i_slot;
        let mut var_ute_i_rv: f64 = *var_ute_i_rv_slot;

        let assign4310_e5551: f64 = (var_inv_w).powf(p.p189);
        let assign4310_e5554: f64 = (var_inv_wwide).powf(p.p189);
        let assign4310_e5555: f64 = (assign4310_e5551 - assign4310_e5554);
        let assign4310_e5557: f64 = (assign4310_e5555).max(0.0);
        let assign4310_e5558: f64 = (p.p188 * assign4310_e5557);
        let assign4310_e5562: f64 = (var_inv_wl).powf(p.p191);
        let assign4310_e5563: f64 = (p.p190 * assign4310_e5562);
        let assign4310_e5564: f64 = (assign4310_e5558 + assign4310_e5563);
        var_t1 = assign4310_e5564;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        let assign4320_e5568: f64 = (1.0 + var_t0);
        let assign4320_e5570: f64 = (assign4320_e5568 + var_t1);
        let assign4320_e5571: f64 = (var_k1_i * assign4320_e5570);
        var_k1_i = assign4320_e5571;
        var_k1_i_dn0 = ((var_k1_i_dn0 * assign4320_e5570) + (var_k1_i * (var_t0_dn0 + var_t1_dn0)));
        var_k1_i_dn2 = ((var_k1_i_dn2 * assign4320_e5570) + (var_k1_i * (var_t0_dn2 + var_t1_dn2)));
        var_k1_i_dn3 = ((var_k1_i_dn3 * assign4320_e5570) + (var_k1_i * (var_t0_dn3 + var_t1_dn3)));
        var_k1_i_dn4 = ((var_k1_i_dn4 * assign4320_e5570) + (var_k1_i * (var_t0_dn4 + var_t1_dn4)));
        var_k1_i_dn5 = ((var_k1_i_dn5 * assign4320_e5570) + (var_k1_i * (var_t0_dn5 + var_t1_dn5)));
        var_k1_i_dn6 = ((var_k1_i_dn6 * assign4320_e5570) + (var_k1_i * (var_t0_dn6 + var_t1_dn6)));
        var_k1_i_dn7 = ((var_k1_i_dn7 * assign4320_e5570) + (var_k1_i * (var_t0_dn7 + var_t1_dn7)));
        var_k1_i_dn8 = ((var_k1_i_dn8 * assign4320_e5570) + (var_k1_i * (var_t0_dn8 + var_t1_dn8)));
        var_k1_i_dn9 = ((var_k1_i_dn9 * assign4320_e5570) + (var_k1_i * (var_t0_dn9 + var_t1_dn9)));
        var_k1_i_dn10 = ((var_k1_i_dn10 * assign4320_e5570) + (var_k1_i * (var_t0_dn10 + var_t1_dn10)));
        var_k1_i_dn11 = ((var_k1_i_dn11 * assign4320_e5570) + (var_k1_i * (var_t0_dn11 + var_t1_dn11)));
        var_k1_i_dn12 = ((var_k1_i_dn12 * assign4320_e5570) + (var_k1_i * (var_t0_dn12 + var_t1_dn12)));
        var_k1_i_dn13 = ((var_k1_i_dn13 * assign4320_e5570) + (var_k1_i * (var_t0_dn13 + var_t1_dn13)));
        var_k1_i_dn14 = ((var_k1_i_dn14 * assign4320_e5570) + (var_k1_i * (var_t0_dn14 + var_t1_dn14)));
        var_k1_i_rv = 0.0;

        let assign4330_e5575: f64 = (var_inv_l).powf(p.p197);
        let assign4330_e5578: f64 = (var_inv_llong).powf(p.p197);
        let assign4330_e5579: f64 = (assign4330_e5575 - assign4330_e5578);
        let assign4330_e5581: f64 = (assign4330_e5579).max(0.0);
        let assign4330_e5582: f64 = (p.p196 * assign4330_e5581);
        var_t0 = assign4330_e5582;
        var_t0_dn0 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn10 = 0.0;
        var_t0_dn11 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn13 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_rv = 0.0;

        let assign4340_e5586: f64 = (var_inv_w).powf(p.p199);
        let assign4340_e5589: f64 = (var_inv_wwide).powf(p.p199);
        let assign4340_e5590: f64 = (assign4340_e5586 - assign4340_e5589);
        let assign4340_e5592: f64 = (assign4340_e5590).max(0.0);
        let assign4340_e5593: f64 = (p.p198 * assign4340_e5592);
        let assign4340_e5597: f64 = (var_inv_wl).powf(p.p201);
        let assign4340_e5598: f64 = (p.p200 * assign4340_e5597);
        let assign4340_e5599: f64 = (assign4340_e5593 + assign4340_e5598);
        var_t1 = assign4340_e5599;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn3 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn12 = 0.0;
        var_t1_dn13 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        let assign4350_e5603: f64 = (1.0 + var_t0);
        let assign4350_e5605: f64 = (assign4350_e5603 + var_t1);
        let assign4350_e5606: f64 = (var_k2_i * assign4350_e5605);
        var_k2_i = assign4350_e5606;
        var_k2_i_dn0 = ((var_k2_i_dn0 * assign4350_e5605) + (var_k2_i * (var_t0_dn0 + var_t1_dn0)));
        var_k2_i_dn2 = ((var_k2_i_dn2 * assign4350_e5605) + (var_k2_i * (var_t0_dn2 + var_t1_dn2)));
        var_k2_i_dn3 = ((var_k2_i_dn3 * assign4350_e5605) + (var_k2_i * (var_t0_dn3 + var_t1_dn3)));
        var_k2_i_dn4 = ((var_k2_i_dn4 * assign4350_e5605) + (var_k2_i * (var_t0_dn4 + var_t1_dn4)));
        var_k2_i_dn5 = ((var_k2_i_dn5 * assign4350_e5605) + (var_k2_i * (var_t0_dn5 + var_t1_dn5)));
        var_k2_i_dn6 = ((var_k2_i_dn6 * assign4350_e5605) + (var_k2_i * (var_t0_dn6 + var_t1_dn6)));
        var_k2_i_dn7 = ((var_k2_i_dn7 * assign4350_e5605) + (var_k2_i * (var_t0_dn7 + var_t1_dn7)));
        var_k2_i_dn8 = ((var_k2_i_dn8 * assign4350_e5605) + (var_k2_i * (var_t0_dn8 + var_t1_dn8)));
        var_k2_i_dn9 = ((var_k2_i_dn9 * assign4350_e5605) + (var_k2_i * (var_t0_dn9 + var_t1_dn9)));
        var_k2_i_dn10 = ((var_k2_i_dn10 * assign4350_e5605) + (var_k2_i * (var_t0_dn10 + var_t1_dn10)));
        var_k2_i_dn11 = ((var_k2_i_dn11 * assign4350_e5605) + (var_k2_i * (var_t0_dn11 + var_t1_dn11)));
        var_k2_i_dn12 = ((var_k2_i_dn12 * assign4350_e5605) + (var_k2_i * (var_t0_dn12 + var_t1_dn12)));
        var_k2_i_dn13 = ((var_k2_i_dn13 * assign4350_e5605) + (var_k2_i * (var_t0_dn13 + var_t1_dn13)));
        var_k2_i_dn14 = ((var_k2_i_dn14 * assign4350_e5605) + (var_k2_i * (var_t0_dn14 + var_t1_dn14)));
        var_k2_i_rv = 0.0;

        let assign4360_e5612: f64 = (var_inv_l).powf(p.p384);
        let assign4360_e5615: f64 = (var_inv_llong).powf(p.p384);
        let assign4360_e5616: f64 = (assign4360_e5612 - assign4360_e5615);
        let assign4360_e5618: f64 = (assign4360_e5616).max(0.0);
        let assign4360_e5619: f64 = (p.p383 * assign4360_e5618);
        let assign4360_e5620: f64 = (1.0 + assign4360_e5619);
        let assign4360_e5621: f64 = (var_prwb_i * assign4360_e5620);
        var_prwb_i = assign4360_e5621;
        var_prwb_i_rv = 0.0;

        let assign4370_e5626: f64 = (var_inv_l * p.p828);
        let assign4370_e5627: f64 = (1.0 + assign4370_e5626);
        let assign4370_e5628: f64 = (var_ute_i * assign4370_e5627);
        var_ute_i = assign4370_e5628;
        var_ute_i_rv = 0.0;

        let assign4380_e5633: f64 = (var_inv_l * p.p833);
        let assign4380_e5634: f64 = (1.0 + assign4380_e5633);
        let assign4380_e5635: f64 = (var_ua1_i * assign4380_e5634);
        var_ua1_i = assign4380_e5635;
        var_ua1_i_rv = 0.0;

        let assign4390_e5640: f64 = (var_inv_l * p.p842);
        let assign4390_e5641: f64 = (1.0 + assign4390_e5640);
        let assign4390_e5642: f64 = (var_ud1_i * assign4390_e5641);
        var_ud1_i = assign4390_e5642;
        var_ud1_i_rv = 0.0;

        let assign4400_e5647: f64 = (var_inv_l * p.p860);
        let assign4400_e5648: f64 = (1.0 + assign4400_e5647);
        let assign4400_e5649: f64 = (var_at_i * assign4400_e5648);
        var_at_i = assign4400_e5649;
        var_at_i_rv = 0.0;

        let assign4410_e5654: f64 = (var_inv_l * p.p866);
        let assign4410_e5655: f64 = (1.0 + assign4410_e5654);
        let assign4410_e5656: f64 = (var_ptwgt_i * assign4410_e5655);
        var_ptwgt_i = assign4410_e5656;
        var_ptwgt_i_rv = 0.0;

        let assign4440_e5670: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        var_guard40 = assign4440_e5670;
        var_guard40_rv = 0.0;

        let (assign4450_e5688,) = {
    if (var_guard40 != 0.0) {
        let assign4450_e5677: f64 = (var_inv_l).powf(p.p398);
        let assign4450_e5680: f64 = (var_inv_llong).powf(p.p398);
        let assign4450_e5681: f64 = (assign4450_e5677 - assign4450_e5680);
        let assign4450_e5683: f64 = (assign4450_e5681).max(0.0);
        let assign4450_e5684: f64 = (p.p397 * assign4450_e5683);
        let assign4450_e5685: f64 = (1.0 + assign4450_e5684);
        let assign4450_e5686: f64 = (var_rsw_i * assign4450_e5685);
        (assign4450_e5686,)
    } else {
        (var_rsw_i,)
    }
};
        var_rsw_i = assign4450_e5688;
        var_rsw_i_rv = 0.0;

        let (assign4460_e5706,) = {
    if (var_guard40 != 0.0) {
        let assign4460_e5695: f64 = (var_inv_l).powf(p.p408);
        let assign4460_e5698: f64 = (var_inv_llong).powf(p.p408);
        let assign4460_e5699: f64 = (assign4460_e5695 - assign4460_e5698);
        let assign4460_e5701: f64 = (assign4460_e5699).max(0.0);
        let assign4460_e5702: f64 = (p.p407 * assign4460_e5701);
        let assign4460_e5703: f64 = (1.0 + assign4460_e5702);
        let assign4460_e5704: f64 = (var_rdw_i * assign4460_e5703);
        (assign4460_e5704,)
    } else {
        (var_rdw_i,)
    }
};
        var_rdw_i = assign4460_e5706;
        var_rdw_i_rv = 0.0;

        let (assign4470_e5725,) = {
    if (var_guard40 == 0.0) {
        let assign4470_e5714: f64 = (var_inv_l).powf(p.p415);
        let assign4470_e5717: f64 = (var_inv_llong).powf(p.p415);
        let assign4470_e5718: f64 = (assign4470_e5714 - assign4470_e5717);
        let assign4470_e5720: f64 = (assign4470_e5718).max(0.0);
        let assign4470_e5721: f64 = (p.p414 * assign4470_e5720);
        let assign4470_e5722: f64 = (1.0 + assign4470_e5721);
        let assign4470_e5723: f64 = (var_rdsw_i * assign4470_e5722);
        (assign4470_e5723,)
    } else {
        (var_rdsw_i,)
    }
};
        var_rdsw_i = assign4470_e5725;
        var_rdsw_i_rv = 0.0;

        let assign4480_e5728: f64 = if var_ucs_i < 1.0 { 1.0 } else { 0.0 };
        var_guard41 = assign4480_e5728;
        var_guard41_rv = 0.0;

        let (assign4490_e5732,) = {
    if (var_guard41 != 0.0) {
        (1.0,)
    } else {
        (var_ucs_i,)
    }
};
        var_ucs_i = assign4490_e5732;
        var_ucs_i_rv = 0.0;

        let assign4500_e5735: f64 = if var_ucs_i > 2.0 { 1.0 } else { 0.0 };
        var_guard42 = assign4500_e5735;
        var_guard42_rv = 0.0;

        let (assign4510_e5742,) = {
    if ((var_guard41 == 0.0) && (var_guard42 != 0.0)) {
        (2.0,)
    } else {
        (var_ucs_i,)
    }
};
        var_ucs_i = assign4510_e5742;
        var_ucs_i_rv = 0.0;

        let assign4520_e5745: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        var_guard43 = assign4520_e5745;
        var_guard43_rv = 0.0;

        let assign4530_e5748: f64 = if var_ucsr_i < 1.0 { 1.0 } else { 0.0 };
        var_guard44 = assign4530_e5748;
        var_guard44_rv = 0.0;

        let (assign4540_e5754,) = {
    if ((var_guard43 != 0.0) && (var_guard44 != 0.0)) {
        (1.0,)
    } else {
        (var_ucsr_i,)
    }
};
        var_ucsr_i = assign4540_e5754;
        var_ucsr_i_rv = 0.0;

        let assign4550_e5757: f64 = if var_ucsr_i > 2.0 { 1.0 } else { 0.0 };
        var_guard45 = assign4550_e5757;
        var_guard45_rv = 0.0;

        let (assign4560_e5766,) = {
    if (((var_guard43 != 0.0) && (var_guard44 == 0.0)) && (var_guard45 != 0.0)) {
        (2.0,)
    } else {
        (var_ucsr_i,)
    }
};
        var_ucsr_i = assign4560_e5766;
        var_ucsr_i_rv = 0.0;

        let assign4800_e5840: f64 = if var_m0_i < 0.0 { 1.0 } else { 0.0 };
        var_guard67 = assign4800_e5840;
        var_guard67_rv = 0.0;

        let (assign4810_e5844,) = {
    if (var_guard67 != 0.0) {
        (0.0,)
    } else {
        (var_m0_i,)
    }
};
        var_m0_i = assign4810_e5844;
        var_m0_i_rv = 0.0;

        let assign4820_e5847: f64 = if var_u0_i <= 0.0 { 1.0 } else { 0.0 };
        var_guard68 = assign4820_e5847;
        var_guard68_rv = 0.0;

        let (assign4830_e5851,) = {
    if (var_guard68 != 0.0) {
        (0.067,)
    } else {
        (var_u0_i,)
    }
};
        var_u0_i = assign4830_e5851;
        var_u0_i_rv = 0.0;

        let assign4840_e5854: f64 = if var_ua_i < 0.0 { 1.0 } else { 0.0 };
        var_guard69 = assign4840_e5854;
        var_guard69_rv = 0.0;

        let (assign4850_e5858, assign4850_e5858_d_n0, assign4850_e5858_d_n2, assign4850_e5858_d_n3, assign4850_e5858_d_n4, assign4850_e5858_d_n5, assign4850_e5858_d_n6, assign4850_e5858_d_n7, assign4850_e5858_d_n8, assign4850_e5858_d_n9, assign4850_e5858_d_n10, assign4850_e5858_d_n11, assign4850_e5858_d_n12, assign4850_e5858_d_n13, assign4850_e5858_d_n14,) = {
    if (var_guard69 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ua_i, var_ua_i_dn0, var_ua_i_dn2, var_ua_i_dn3, var_ua_i_dn4, var_ua_i_dn5, var_ua_i_dn6, var_ua_i_dn7, var_ua_i_dn8, var_ua_i_dn9, var_ua_i_dn10, var_ua_i_dn11, var_ua_i_dn12, var_ua_i_dn13, var_ua_i_dn14,)
    }
};
        var_ua_i = assign4850_e5858;
        var_ua_i_dn0 = assign4850_e5858_d_n0;
        var_ua_i_dn2 = assign4850_e5858_d_n2;
        var_ua_i_dn3 = assign4850_e5858_d_n3;
        var_ua_i_dn4 = assign4850_e5858_d_n4;
        var_ua_i_dn5 = assign4850_e5858_d_n5;
        var_ua_i_dn6 = assign4850_e5858_d_n6;
        var_ua_i_dn7 = assign4850_e5858_d_n7;
        var_ua_i_dn8 = assign4850_e5858_d_n8;
        var_ua_i_dn9 = assign4850_e5858_d_n9;
        var_ua_i_dn10 = assign4850_e5858_d_n10;
        var_ua_i_dn11 = assign4850_e5858_d_n11;
        var_ua_i_dn12 = assign4850_e5858_d_n12;
        var_ua_i_dn13 = assign4850_e5858_d_n13;
        var_ua_i_dn14 = assign4850_e5858_d_n14;
        var_ua_i_rv = 0.0;

        let assign4860_e5861: f64 = if var_eu_i < 0.0 { 1.0 } else { 0.0 };
        var_guard70 = assign4860_e5861;
        var_guard70_rv = 0.0;

        let (assign4870_e5865, assign4870_e5865_d_n0, assign4870_e5865_d_n2, assign4870_e5865_d_n3, assign4870_e5865_d_n4, assign4870_e5865_d_n5, assign4870_e5865_d_n6, assign4870_e5865_d_n7, assign4870_e5865_d_n8, assign4870_e5865_d_n9, assign4870_e5865_d_n10, assign4870_e5865_d_n11, assign4870_e5865_d_n12, assign4870_e5865_d_n13, assign4870_e5865_d_n14,) = {
    if (var_guard70 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_eu_i, var_eu_i_dn0, var_eu_i_dn2, var_eu_i_dn3, var_eu_i_dn4, var_eu_i_dn5, var_eu_i_dn6, var_eu_i_dn7, var_eu_i_dn8, var_eu_i_dn9, var_eu_i_dn10, var_eu_i_dn11, var_eu_i_dn12, var_eu_i_dn13, var_eu_i_dn14,)
    }
};
        var_eu_i = assign4870_e5865;
        var_eu_i_dn0 = assign4870_e5865_d_n0;
        var_eu_i_dn2 = assign4870_e5865_d_n2;
        var_eu_i_dn3 = assign4870_e5865_d_n3;
        var_eu_i_dn4 = assign4870_e5865_d_n4;
        var_eu_i_dn5 = assign4870_e5865_d_n5;
        var_eu_i_dn6 = assign4870_e5865_d_n6;
        var_eu_i_dn7 = assign4870_e5865_d_n7;
        var_eu_i_dn8 = assign4870_e5865_d_n8;
        var_eu_i_dn9 = assign4870_e5865_d_n9;
        var_eu_i_dn10 = assign4870_e5865_d_n10;
        var_eu_i_dn11 = assign4870_e5865_d_n11;
        var_eu_i_dn12 = assign4870_e5865_d_n12;
        var_eu_i_dn13 = assign4870_e5865_d_n13;
        var_eu_i_dn14 = assign4870_e5865_d_n14;
        var_eu_i_rv = 0.0;

        let assign4880_e5868: f64 = if var_ud_i < 0.0 { 1.0 } else { 0.0 };
        var_guard71 = assign4880_e5868;
        var_guard71_rv = 0.0;

        let (assign4890_e5872, assign4890_e5872_d_n0, assign4890_e5872_d_n2, assign4890_e5872_d_n3, assign4890_e5872_d_n4, assign4890_e5872_d_n5, assign4890_e5872_d_n6, assign4890_e5872_d_n7, assign4890_e5872_d_n8, assign4890_e5872_d_n9, assign4890_e5872_d_n10, assign4890_e5872_d_n11, assign4890_e5872_d_n12, assign4890_e5872_d_n13, assign4890_e5872_d_n14,) = {
    if (var_guard71 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ud_i, var_ud_i_dn0, var_ud_i_dn2, var_ud_i_dn3, var_ud_i_dn4, var_ud_i_dn5, var_ud_i_dn6, var_ud_i_dn7, var_ud_i_dn8, var_ud_i_dn9, var_ud_i_dn10, var_ud_i_dn11, var_ud_i_dn12, var_ud_i_dn13, var_ud_i_dn14,)
    }
};
        var_ud_i = assign4890_e5872;
        var_ud_i_dn0 = assign4890_e5872_d_n0;
        var_ud_i_dn2 = assign4890_e5872_d_n2;
        var_ud_i_dn3 = assign4890_e5872_d_n3;
        var_ud_i_dn4 = assign4890_e5872_d_n4;
        var_ud_i_dn5 = assign4890_e5872_d_n5;
        var_ud_i_dn6 = assign4890_e5872_d_n6;
        var_ud_i_dn7 = assign4890_e5872_d_n7;
        var_ud_i_dn8 = assign4890_e5872_d_n8;
        var_ud_i_dn9 = assign4890_e5872_d_n9;
        var_ud_i_dn10 = assign4890_e5872_d_n10;
        var_ud_i_dn11 = assign4890_e5872_d_n11;
        var_ud_i_dn12 = assign4890_e5872_d_n12;
        var_ud_i_dn13 = assign4890_e5872_d_n13;
        var_ud_i_dn14 = assign4890_e5872_d_n14;
        var_ud_i_rv = 0.0;

        let assign4900_e5875: f64 = if var_ucs_i < 0.0 { 1.0 } else { 0.0 };
        var_guard72 = assign4900_e5875;
        var_guard72_rv = 0.0;

        let (assign4910_e5879,) = {
    if (var_guard72 != 0.0) {
        (0.0,)
    } else {
        (var_ucs_i,)
    }
};
        var_ucs_i = assign4910_e5879;
        var_ucs_i_rv = 0.0;

        let assign4920_e5882: f64 = if var_beta1_i < 0.0 { 1.0 } else { 0.0 };
        var_guard73 = assign4920_e5882;
        var_guard73_rv = 0.0;

        let (assign4930_e5886, assign4930_e5886_d_n0, assign4930_e5886_d_n2, assign4930_e5886_d_n3, assign4930_e5886_d_n4, assign4930_e5886_d_n5, assign4930_e5886_d_n6, assign4930_e5886_d_n7, assign4930_e5886_d_n8, assign4930_e5886_d_n9, assign4930_e5886_d_n10, assign4930_e5886_d_n11, assign4930_e5886_d_n12, assign4930_e5886_d_n13, assign4930_e5886_d_n14,) = {
    if (var_guard73 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_beta1_i, var_beta1_i_dn0, var_beta1_i_dn2, var_beta1_i_dn3, var_beta1_i_dn4, var_beta1_i_dn5, var_beta1_i_dn6, var_beta1_i_dn7, var_beta1_i_dn8, var_beta1_i_dn9, var_beta1_i_dn10, var_beta1_i_dn11, var_beta1_i_dn12, var_beta1_i_dn13, var_beta1_i_dn14,)
    }
};
        var_beta1_i = assign4930_e5886;
        var_beta1_i_dn0 = assign4930_e5886_d_n0;
        var_beta1_i_dn2 = assign4930_e5886_d_n2;
        var_beta1_i_dn3 = assign4930_e5886_d_n3;
        var_beta1_i_dn4 = assign4930_e5886_d_n4;
        var_beta1_i_dn5 = assign4930_e5886_d_n5;
        var_beta1_i_dn6 = assign4930_e5886_d_n6;
        var_beta1_i_dn7 = assign4930_e5886_d_n7;
        var_beta1_i_dn8 = assign4930_e5886_d_n8;
        var_beta1_i_dn9 = assign4930_e5886_d_n9;
        var_beta1_i_dn10 = assign4930_e5886_d_n10;
        var_beta1_i_dn11 = assign4930_e5886_d_n11;
        var_beta1_i_dn12 = assign4930_e5886_d_n12;
        var_beta1_i_dn13 = assign4930_e5886_d_n13;
        var_beta1_i_dn14 = assign4930_e5886_d_n14;
        var_beta1_i_rv = 0.0;

        let assign4940_e5889: f64 = if p.p1065 == 1.0 { 1.0 } else { 0.0 };
        var_guard74 = assign4940_e5889;
        var_guard74_rv = 0.0;

        let (assign4950_e5893,) = {
    if (var_guard74 != 0.0) {
        (p.p1066,)
    } else {
        (var_lh1,)
    }
};
        var_lh1 = assign4950_e5893;
        var_lh1_rv = 0.0;

        let assign4960_e5896: f64 = if var_leff > var_lh1 { 1.0 } else { 0.0 };
        var_guard75 = assign4960_e5896;
        var_guard75_rv = 0.0;

        let (assign4970_e5904, assign4970_e5904_d_n0, assign4970_e5904_d_n2, assign4970_e5904_d_n3, assign4970_e5904_d_n4, assign4970_e5904_d_n5, assign4970_e5904_d_n6, assign4970_e5904_d_n7, assign4970_e5904_d_n8, assign4970_e5904_d_n9, assign4970_e5904_d_n10, assign4970_e5904_d_n11, assign4970_e5904_d_n12, assign4970_e5904_d_n13, assign4970_e5904_d_n14,) = {
    if ((var_guard74 != 0.0) && (var_guard75 != 0.0)) {
        let assign4970_e5902: f64 = (var_leff - var_lh1);
        (assign4970_e5902, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn12, var_t0_dn13, var_t0_dn14,)
    }
};
        var_t0 = assign4970_e5904;
        var_t0_dn0 = assign4970_e5904_d_n0;
        var_t0_dn2 = assign4970_e5904_d_n2;
        var_t0_dn3 = assign4970_e5904_d_n3;
        var_t0_dn4 = assign4970_e5904_d_n4;
        var_t0_dn5 = assign4970_e5904_d_n5;
        var_t0_dn6 = assign4970_e5904_d_n6;
        var_t0_dn7 = assign4970_e5904_d_n7;
        var_t0_dn8 = assign4970_e5904_d_n8;
        var_t0_dn9 = assign4970_e5904_d_n9;
        var_t0_dn10 = assign4970_e5904_d_n10;
        var_t0_dn11 = assign4970_e5904_d_n11;
        var_t0_dn12 = assign4970_e5904_d_n12;
        var_t0_dn13 = assign4970_e5904_d_n13;
        var_t0_dn14 = assign4970_e5904_d_n14;
        var_t0_rv = 0.0;

        let (assign4980_e5911,) = {
    if ((var_guard74 != 0.0) && (var_guard75 == 0.0)) {
        (var_leff,)
    } else {
        (var_lh1,)
    }
};
        var_lh1 = assign4980_e5911;
        var_lh1_rv = 0.0;

        let (assign4990_e5918, assign4990_e5918_d_n0, assign4990_e5918_d_n2, assign4990_e5918_d_n3, assign4990_e5918_d_n4, assign4990_e5918_d_n5, assign4990_e5918_d_n6, assign4990_e5918_d_n7, assign4990_e5918_d_n8, assign4990_e5918_d_n9, assign4990_e5918_d_n10, assign4990_e5918_d_n11, assign4990_e5918_d_n12, assign4990_e5918_d_n13, assign4990_e5918_d_n14,) = {
    if ((var_guard74 != 0.0) && (var_guard75 == 0.0)) {
        (var_lh1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11, var_t0_dn12, var_t0_dn13, var_t0_dn14,)
    }
};
        var_t0 = assign4990_e5918;
        var_t0_dn0 = assign4990_e5918_d_n0;
        var_t0_dn2 = assign4990_e5918_d_n2;
        var_t0_dn3 = assign4990_e5918_d_n3;
        var_t0_dn4 = assign4990_e5918_d_n4;
        var_t0_dn5 = assign4990_e5918_d_n5;
        var_t0_dn6 = assign4990_e5918_d_n6;
        var_t0_dn7 = assign4990_e5918_d_n7;
        var_t0_dn8 = assign4990_e5918_d_n8;
        var_t0_dn9 = assign4990_e5918_d_n9;
        var_t0_dn10 = assign4990_e5918_d_n10;
        var_t0_dn11 = assign4990_e5918_d_n11;
        var_t0_dn12 = assign4990_e5918_d_n12;
        var_t0_dn13 = assign4990_e5918_d_n13;
        var_t0_dn14 = assign4990_e5918_d_n14;
        var_t0_rv = 0.0;

        let assign5000_e5922: f64 = (var_t0 / 2.0);
        let assign5000_e5923: f64 = if p.p801 >= assign5000_e5922 { 1.0 } else { 0.0 };
        var_guard76 = assign5000_e5923;
        var_guard76_rv = 0.0;

        *var_at_i_slot = var_at_i;
        *var_at_i_rv_slot = var_at_i_rv;
        *var_beta1_i_slot = var_beta1_i;
        *var_beta1_i_dn0_slot = var_beta1_i_dn0;
        *var_beta1_i_dn10_slot = var_beta1_i_dn10;
        *var_beta1_i_dn11_slot = var_beta1_i_dn11;
        *var_beta1_i_dn12_slot = var_beta1_i_dn12;
        *var_beta1_i_dn13_slot = var_beta1_i_dn13;
        *var_beta1_i_dn14_slot = var_beta1_i_dn14;
        *var_beta1_i_dn2_slot = var_beta1_i_dn2;
        *var_beta1_i_dn3_slot = var_beta1_i_dn3;
        *var_beta1_i_dn4_slot = var_beta1_i_dn4;
        *var_beta1_i_dn5_slot = var_beta1_i_dn5;
        *var_beta1_i_dn6_slot = var_beta1_i_dn6;
        *var_beta1_i_dn7_slot = var_beta1_i_dn7;
        *var_beta1_i_dn8_slot = var_beta1_i_dn8;
        *var_beta1_i_dn9_slot = var_beta1_i_dn9;
        *var_beta1_i_rv_slot = var_beta1_i_rv;
        *var_eu_i_slot = var_eu_i;
        *var_eu_i_dn0_slot = var_eu_i_dn0;
        *var_eu_i_dn10_slot = var_eu_i_dn10;
        *var_eu_i_dn11_slot = var_eu_i_dn11;
        *var_eu_i_dn12_slot = var_eu_i_dn12;
        *var_eu_i_dn13_slot = var_eu_i_dn13;
        *var_eu_i_dn14_slot = var_eu_i_dn14;
        *var_eu_i_dn2_slot = var_eu_i_dn2;
        *var_eu_i_dn3_slot = var_eu_i_dn3;
        *var_eu_i_dn4_slot = var_eu_i_dn4;
        *var_eu_i_dn5_slot = var_eu_i_dn5;
        *var_eu_i_dn6_slot = var_eu_i_dn6;
        *var_eu_i_dn7_slot = var_eu_i_dn7;
        *var_eu_i_dn8_slot = var_eu_i_dn8;
        *var_eu_i_dn9_slot = var_eu_i_dn9;
        *var_eu_i_rv_slot = var_eu_i_rv;
        *var_guard40_slot = var_guard40;
        *var_guard40_rv_slot = var_guard40_rv;
        *var_guard41_slot = var_guard41;
        *var_guard41_rv_slot = var_guard41_rv;
        *var_guard42_slot = var_guard42;
        *var_guard42_rv_slot = var_guard42_rv;
        *var_guard43_slot = var_guard43;
        *var_guard43_rv_slot = var_guard43_rv;
        *var_guard44_slot = var_guard44;
        *var_guard44_rv_slot = var_guard44_rv;
        *var_guard45_slot = var_guard45;
        *var_guard45_rv_slot = var_guard45_rv;
        *var_guard67_slot = var_guard67;
        *var_guard67_rv_slot = var_guard67_rv;
        *var_guard68_slot = var_guard68;
        *var_guard68_rv_slot = var_guard68_rv;
        *var_guard69_slot = var_guard69;
        *var_guard69_rv_slot = var_guard69_rv;
        *var_guard70_slot = var_guard70;
        *var_guard70_rv_slot = var_guard70_rv;
        *var_guard71_slot = var_guard71;
        *var_guard71_rv_slot = var_guard71_rv;
        *var_guard72_slot = var_guard72;
        *var_guard72_rv_slot = var_guard72_rv;
        *var_guard73_slot = var_guard73;
        *var_guard73_rv_slot = var_guard73_rv;
        *var_guard74_slot = var_guard74;
        *var_guard74_rv_slot = var_guard74_rv;
        *var_guard75_slot = var_guard75;
        *var_guard75_rv_slot = var_guard75_rv;
        *var_guard76_slot = var_guard76;
        *var_guard76_rv_slot = var_guard76_rv;
        *var_k1_i_slot = var_k1_i;
        *var_k1_i_dn0_slot = var_k1_i_dn0;
        *var_k1_i_dn10_slot = var_k1_i_dn10;
        *var_k1_i_dn11_slot = var_k1_i_dn11;
        *var_k1_i_dn12_slot = var_k1_i_dn12;
        *var_k1_i_dn13_slot = var_k1_i_dn13;
        *var_k1_i_dn14_slot = var_k1_i_dn14;
        *var_k1_i_dn2_slot = var_k1_i_dn2;
        *var_k1_i_dn3_slot = var_k1_i_dn3;
        *var_k1_i_dn4_slot = var_k1_i_dn4;
        *var_k1_i_dn5_slot = var_k1_i_dn5;
        *var_k1_i_dn6_slot = var_k1_i_dn6;
        *var_k1_i_dn7_slot = var_k1_i_dn7;
        *var_k1_i_dn8_slot = var_k1_i_dn8;
        *var_k1_i_dn9_slot = var_k1_i_dn9;
        *var_k1_i_rv_slot = var_k1_i_rv;
        *var_k2_i_slot = var_k2_i;
        *var_k2_i_dn0_slot = var_k2_i_dn0;
        *var_k2_i_dn10_slot = var_k2_i_dn10;
        *var_k2_i_dn11_slot = var_k2_i_dn11;
        *var_k2_i_dn12_slot = var_k2_i_dn12;
        *var_k2_i_dn13_slot = var_k2_i_dn13;
        *var_k2_i_dn14_slot = var_k2_i_dn14;
        *var_k2_i_dn2_slot = var_k2_i_dn2;
        *var_k2_i_dn3_slot = var_k2_i_dn3;
        *var_k2_i_dn4_slot = var_k2_i_dn4;
        *var_k2_i_dn5_slot = var_k2_i_dn5;
        *var_k2_i_dn6_slot = var_k2_i_dn6;
        *var_k2_i_dn7_slot = var_k2_i_dn7;
        *var_k2_i_dn8_slot = var_k2_i_dn8;
        *var_k2_i_dn9_slot = var_k2_i_dn9;
        *var_k2_i_rv_slot = var_k2_i_rv;
        *var_lh1_slot = var_lh1;
        *var_lh1_rv_slot = var_lh1_rv;
        *var_m0_i_slot = var_m0_i;
        *var_m0_i_rv_slot = var_m0_i_rv;
        *var_prwb_i_slot = var_prwb_i;
        *var_prwb_i_rv_slot = var_prwb_i_rv;
        *var_ptwgt_i_slot = var_ptwgt_i;
        *var_ptwgt_i_rv_slot = var_ptwgt_i_rv;
        *var_rdsw_i_slot = var_rdsw_i;
        *var_rdsw_i_rv_slot = var_rdsw_i_rv;
        *var_rdw_i_slot = var_rdw_i;
        *var_rdw_i_rv_slot = var_rdw_i_rv;
        *var_rsw_i_slot = var_rsw_i;
        *var_rsw_i_rv_slot = var_rsw_i_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn13_slot = var_t0_dn13;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_u0_i_slot = var_u0_i;
        *var_u0_i_rv_slot = var_u0_i_rv;
        *var_ua1_i_slot = var_ua1_i;
        *var_ua1_i_rv_slot = var_ua1_i_rv;
        *var_ua_i_slot = var_ua_i;
        *var_ua_i_dn0_slot = var_ua_i_dn0;
        *var_ua_i_dn10_slot = var_ua_i_dn10;
        *var_ua_i_dn11_slot = var_ua_i_dn11;
        *var_ua_i_dn12_slot = var_ua_i_dn12;
        *var_ua_i_dn13_slot = var_ua_i_dn13;
        *var_ua_i_dn14_slot = var_ua_i_dn14;
        *var_ua_i_dn2_slot = var_ua_i_dn2;
        *var_ua_i_dn3_slot = var_ua_i_dn3;
        *var_ua_i_dn4_slot = var_ua_i_dn4;
        *var_ua_i_dn5_slot = var_ua_i_dn5;
        *var_ua_i_dn6_slot = var_ua_i_dn6;
        *var_ua_i_dn7_slot = var_ua_i_dn7;
        *var_ua_i_dn8_slot = var_ua_i_dn8;
        *var_ua_i_dn9_slot = var_ua_i_dn9;
        *var_ua_i_rv_slot = var_ua_i_rv;
        *var_ucs_i_slot = var_ucs_i;
        *var_ucs_i_rv_slot = var_ucs_i_rv;
        *var_ucsr_i_slot = var_ucsr_i;
        *var_ucsr_i_rv_slot = var_ucsr_i_rv;
        *var_ud1_i_slot = var_ud1_i;
        *var_ud1_i_rv_slot = var_ud1_i_rv;
        *var_ud_i_slot = var_ud_i;
        *var_ud_i_dn0_slot = var_ud_i_dn0;
        *var_ud_i_dn10_slot = var_ud_i_dn10;
        *var_ud_i_dn11_slot = var_ud_i_dn11;
        *var_ud_i_dn12_slot = var_ud_i_dn12;
        *var_ud_i_dn13_slot = var_ud_i_dn13;
        *var_ud_i_dn14_slot = var_ud_i_dn14;
        *var_ud_i_dn2_slot = var_ud_i_dn2;
        *var_ud_i_dn3_slot = var_ud_i_dn3;
        *var_ud_i_dn4_slot = var_ud_i_dn4;
        *var_ud_i_dn5_slot = var_ud_i_dn5;
        *var_ud_i_dn6_slot = var_ud_i_dn6;
        *var_ud_i_dn7_slot = var_ud_i_dn7;
        *var_ud_i_dn8_slot = var_ud_i_dn8;
        *var_ud_i_dn9_slot = var_ud_i_dn9;
        *var_ud_i_rv_slot = var_ud_i_rv;
        *var_ute_i_slot = var_ute_i;
        *var_ute_i_rv_slot = var_ute_i_rv;
    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard74: f64,
        var_guard76: f64,
        var_weff: f64,
        var_dmcgeff_slot: &mut f64,
        var_dmcgeff_rv_slot: &mut f64,
        var_dmcieff_slot: &mut f64,
        var_dmcieff_rv_slot: &mut f64,
        var_dmdgeff_slot: &mut f64,
        var_dmdgeff_rv_slot: &mut f64,
        var_guard100_slot: &mut f64,
        var_guard100_rv_slot: &mut f64,
        var_guard102_slot: &mut f64,
        var_guard102_rv_slot: &mut f64,
        var_guard103_slot: &mut f64,
        var_guard103_rv_slot: &mut f64,
        var_guard104_slot: &mut f64,
        var_guard104_rv_slot: &mut f64,
        var_guard105_slot: &mut f64,
        var_guard105_rv_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_guard107_rv_slot: &mut f64,
        var_guard108_slot: &mut f64,
        var_guard108_rv_slot: &mut f64,
        var_guard109_slot: &mut f64,
        var_guard109_rv_slot: &mut f64,
        var_guard110_slot: &mut f64,
        var_guard110_rv_slot: &mut f64,
        var_guard111_slot: &mut f64,
        var_guard111_rv_slot: &mut f64,
        var_guard77_slot: &mut f64,
        var_guard77_rv_slot: &mut f64,
        var_guard78_slot: &mut f64,
        var_guard78_rv_slot: &mut f64,
        var_guard79_slot: &mut f64,
        var_guard79_rv_slot: &mut f64,
        var_guard80_slot: &mut f64,
        var_guard80_rv_slot: &mut f64,
        var_guard81_slot: &mut f64,
        var_guard81_rv_slot: &mut f64,
        var_guard82_slot: &mut f64,
        var_guard82_rv_slot: &mut f64,
        var_guard83_slot: &mut f64,
        var_guard83_rv_slot: &mut f64,
        var_guard84_slot: &mut f64,
        var_guard84_rv_slot: &mut f64,
        var_guard85_slot: &mut f64,
        var_guard85_rv_slot: &mut f64,
        var_guard86_slot: &mut f64,
        var_guard86_rv_slot: &mut f64,
        var_guard87_slot: &mut f64,
        var_guard87_rv_slot: &mut f64,
        var_guard88_slot: &mut f64,
        var_guard88_rv_slot: &mut f64,
        var_guard89_slot: &mut f64,
        var_guard89_rv_slot: &mut f64,
        var_guard90_slot: &mut f64,
        var_guard90_rv_slot: &mut f64,
        var_guard91_slot: &mut f64,
        var_guard91_rv_slot: &mut f64,
        var_guard92_slot: &mut f64,
        var_guard92_rv_slot: &mut f64,
        var_guard93_slot: &mut f64,
        var_guard93_rv_slot: &mut f64,
        var_guard94_slot: &mut f64,
        var_guard94_rv_slot: &mut f64,
        var_guard95_slot: &mut f64,
        var_guard95_rv_slot: &mut f64,
        var_guard96_slot: &mut f64,
        var_guard96_rv_slot: &mut f64,
        var_guard97_slot: &mut f64,
        var_guard97_rv_slot: &mut f64,
        var_guard98_slot: &mut f64,
        var_guard98_rv_slot: &mut f64,
        var_guard99_slot: &mut f64,
        var_guard99_rv_slot: &mut f64,
        var_lintnoi_i_slot: &mut f64,
        var_lintnoi_i_rv_slot: &mut f64,
        var_nuendd_slot: &mut f64,
        var_nuendd_rv_slot: &mut f64,
        var_nuends_slot: &mut f64,
        var_nuends_rv_slot: &mut f64,
        var_nuintd_slot: &mut f64,
        var_nuintd_rv_slot: &mut f64,
        var_nuints_slot: &mut f64,
        var_nuints_rv_slot: &mut f64,
        var_rend_slot: &mut f64,
        var_rend_rv_slot: &mut f64,
        var_rint_slot: &mut f64,
        var_rint_rv_slot: &mut f64,
        var_rsourcegeo_slot: &mut f64,
        var_rsourcegeo_rv_slot: &mut f64,
    ) {
        let mut var_dmcgeff: f64 = *var_dmcgeff_slot;
        let mut var_dmcgeff_rv: f64 = *var_dmcgeff_rv_slot;
        let mut var_dmcieff: f64 = *var_dmcieff_slot;
        let mut var_dmcieff_rv: f64 = *var_dmcieff_rv_slot;
        let mut var_dmdgeff: f64 = *var_dmdgeff_slot;
        let mut var_dmdgeff_rv: f64 = *var_dmdgeff_rv_slot;
        let mut var_guard100: f64 = *var_guard100_slot;
        let mut var_guard100_rv: f64 = *var_guard100_rv_slot;
        let mut var_guard102: f64 = *var_guard102_slot;
        let mut var_guard102_rv: f64 = *var_guard102_rv_slot;
        let mut var_guard103: f64 = *var_guard103_slot;
        let mut var_guard103_rv: f64 = *var_guard103_rv_slot;
        let mut var_guard104: f64 = *var_guard104_slot;
        let mut var_guard104_rv: f64 = *var_guard104_rv_slot;
        let mut var_guard105: f64 = *var_guard105_slot;
        let mut var_guard105_rv: f64 = *var_guard105_rv_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_guard107_rv: f64 = *var_guard107_rv_slot;
        let mut var_guard108: f64 = *var_guard108_slot;
        let mut var_guard108_rv: f64 = *var_guard108_rv_slot;
        let mut var_guard109: f64 = *var_guard109_slot;
        let mut var_guard109_rv: f64 = *var_guard109_rv_slot;
        let mut var_guard110: f64 = *var_guard110_slot;
        let mut var_guard110_rv: f64 = *var_guard110_rv_slot;
        let mut var_guard111: f64 = *var_guard111_slot;
        let mut var_guard111_rv: f64 = *var_guard111_rv_slot;
        let mut var_guard77: f64 = *var_guard77_slot;
        let mut var_guard77_rv: f64 = *var_guard77_rv_slot;
        let mut var_guard78: f64 = *var_guard78_slot;
        let mut var_guard78_rv: f64 = *var_guard78_rv_slot;
        let mut var_guard79: f64 = *var_guard79_slot;
        let mut var_guard79_rv: f64 = *var_guard79_rv_slot;
        let mut var_guard80: f64 = *var_guard80_slot;
        let mut var_guard80_rv: f64 = *var_guard80_rv_slot;
        let mut var_guard81: f64 = *var_guard81_slot;
        let mut var_guard81_rv: f64 = *var_guard81_rv_slot;
        let mut var_guard82: f64 = *var_guard82_slot;
        let mut var_guard82_rv: f64 = *var_guard82_rv_slot;
        let mut var_guard83: f64 = *var_guard83_slot;
        let mut var_guard83_rv: f64 = *var_guard83_rv_slot;
        let mut var_guard84: f64 = *var_guard84_slot;
        let mut var_guard84_rv: f64 = *var_guard84_rv_slot;
        let mut var_guard85: f64 = *var_guard85_slot;
        let mut var_guard85_rv: f64 = *var_guard85_rv_slot;
        let mut var_guard86: f64 = *var_guard86_slot;
        let mut var_guard86_rv: f64 = *var_guard86_rv_slot;
        let mut var_guard87: f64 = *var_guard87_slot;
        let mut var_guard87_rv: f64 = *var_guard87_rv_slot;
        let mut var_guard88: f64 = *var_guard88_slot;
        let mut var_guard88_rv: f64 = *var_guard88_rv_slot;
        let mut var_guard89: f64 = *var_guard89_slot;
        let mut var_guard89_rv: f64 = *var_guard89_rv_slot;
        let mut var_guard90: f64 = *var_guard90_slot;
        let mut var_guard90_rv: f64 = *var_guard90_rv_slot;
        let mut var_guard91: f64 = *var_guard91_slot;
        let mut var_guard91_rv: f64 = *var_guard91_rv_slot;
        let mut var_guard92: f64 = *var_guard92_slot;
        let mut var_guard92_rv: f64 = *var_guard92_rv_slot;
        let mut var_guard93: f64 = *var_guard93_slot;
        let mut var_guard93_rv: f64 = *var_guard93_rv_slot;
        let mut var_guard94: f64 = *var_guard94_slot;
        let mut var_guard94_rv: f64 = *var_guard94_rv_slot;
        let mut var_guard95: f64 = *var_guard95_slot;
        let mut var_guard95_rv: f64 = *var_guard95_rv_slot;
        let mut var_guard96: f64 = *var_guard96_slot;
        let mut var_guard96_rv: f64 = *var_guard96_rv_slot;
        let mut var_guard97: f64 = *var_guard97_slot;
        let mut var_guard97_rv: f64 = *var_guard97_rv_slot;
        let mut var_guard98: f64 = *var_guard98_slot;
        let mut var_guard98_rv: f64 = *var_guard98_rv_slot;
        let mut var_guard99: f64 = *var_guard99_slot;
        let mut var_guard99_rv: f64 = *var_guard99_rv_slot;
        let mut var_lintnoi_i: f64 = *var_lintnoi_i_slot;
        let mut var_lintnoi_i_rv: f64 = *var_lintnoi_i_rv_slot;
        let mut var_nuendd: f64 = *var_nuendd_slot;
        let mut var_nuendd_rv: f64 = *var_nuendd_rv_slot;
        let mut var_nuends: f64 = *var_nuends_slot;
        let mut var_nuends_rv: f64 = *var_nuends_rv_slot;
        let mut var_nuintd: f64 = *var_nuintd_slot;
        let mut var_nuintd_rv: f64 = *var_nuintd_rv_slot;
        let mut var_nuints: f64 = *var_nuints_slot;
        let mut var_nuints_rv: f64 = *var_nuints_rv_slot;
        let mut var_rend: f64 = *var_rend_slot;
        let mut var_rend_rv: f64 = *var_rend_rv_slot;
        let mut var_rint: f64 = *var_rint_slot;
        let mut var_rint_rv: f64 = *var_rint_rv_slot;
        let mut var_rsourcegeo: f64 = *var_rsourcegeo_slot;
        let mut var_rsourcegeo_rv: f64 = *var_rsourcegeo_rv_slot;

        let (assign5010_e5929,) = {
    if ((var_guard74 != 0.0) && (var_guard76 != 0.0)) {
        (0.0,)
    } else {
        (var_lintnoi_i,)
    }
};
        var_lintnoi_i = assign5010_e5929;
        var_lintnoi_i_rv = 0.0;

        let (assign5020_e5936,) = {
    if ((var_guard74 != 0.0) && (var_guard76 == 0.0)) {
        (p.p801,)
    } else {
        (var_lintnoi_i,)
    }
};
        var_lintnoi_i = assign5020_e5936;
        var_lintnoi_i_rv = 0.0;

        var_nuendd = 0.0;
        var_nuendd_rv = 0.0;

        var_nuends = 0.0;
        var_nuends_rv = 0.0;

        var_nuintd = 0.0;
        var_nuintd_rv = 0.0;

        var_nuints = 0.0;
        var_nuints_rv = 0.0;

        var_rend = 0.0;
        var_rend_rv = 0.0;

        var_rint = 0.0;
        var_rint_rv = 0.0;

        let assign5090_e5945: f64 = (p.p695 - p.p698);
        var_dmcgeff = assign5090_e5945;
        var_dmcgeff_rv = 0.0;

        var_dmcieff = p.p696;
        var_dmcieff_rv = 0.0;

        let assign5110_e5949: f64 = (p.p697 - p.p698);
        var_dmdgeff = assign5110_e5949;
        var_dmdgeff_rv = 0.0;

        let assign5120_e5951: f64 = if param_given[3] { 1.0 } else { 0.0 };
        var_guard77 = assign5120_e5951;
        var_guard77_rv = 0.0;

        let (assign5130_e5957,) = {
    if (var_guard77 != 0.0) {
        let assign5130_e5955: f64 = (p.p374 * p.p3);
        (assign5130_e5955,)
    } else {
        (var_rsourcegeo,)
    }
};
        var_rsourcegeo = assign5130_e5957;
        var_rsourcegeo_rv = 0.0;

        let assign5140_e5964: f64 = if ((p.p10 > 0.0) && (p.p374 > 0.0)) { 1.0 } else { 0.0 };
        var_guard78 = assign5140_e5964;
        var_guard78_rv = 0.0;

        let assign5150_e5967: f64 = if p.p9 < 9.0 { 1.0 } else { 0.0 };
        var_guard79 = assign5150_e5967;
        var_guard79_rv = 0.0;

        let assign5160_e5970: f64 = (p.p2 % 2.0);
        let assign5160_e5972: f64 = if assign5160_e5970 != 0.0 { 1.0 } else { 0.0 };
        var_guard80 = assign5160_e5972;
        var_guard80_rv = 0.0;

        let (assign5170_e5983,) = {
    if ((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) {
        (1.0,)
    } else {
        (var_nuendd,)
    }
};
        var_nuendd = assign5170_e5983;
        var_nuendd_rv = 0.0;

        let (assign5180_e5994,) = {
    if ((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) {
        (1.0,)
    } else {
        (var_nuends,)
    }
};
        var_nuends = assign5180_e5994;
        var_nuends_rv = 0.0;

        let (assign5190_e6013,) = {
    if ((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) {
        let assign5190_e6006: f64 = (p.p2 - 1.0);
        let assign5190_e6008: f64 = (assign5190_e6006 / 2.0);
        let assign5190_e6010: f64 = (assign5190_e6008).max(0.0);
        let assign5190_e6011: f64 = (2.0 * assign5190_e6010);
        (assign5190_e6011,)
    } else {
        (var_nuintd,)
    }
};
        var_nuintd = assign5190_e6013;
        var_nuintd_rv = 0.0;

        let (assign5200_e6024,) = {
    if ((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard79 != 0.0)) && (var_guard80 != 0.0)) {
        (var_nuintd,)
    } else {
        (var_nuints,)
    }
};
        var_nuints = assign5200_e6024;
        var_nuints_rv = 0.0;

        let assign5210_e6027: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        var_guard81 = assign5210_e6027;
        var_guard81_rv = 0.0;

        let (assign5220_e6041,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) && (var_guard81 != 0.0)) {
        (2.0,)
    } else {
        (var_nuendd,)
    }
};
        var_nuendd = assign5220_e6041;
        var_nuendd_rv = 0.0;

        let (assign5230_e6063,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) && (var_guard81 != 0.0)) {
        let assign5230_e6056: f64 = (p.p2 / 2.0);
        let assign5230_e6058: f64 = (assign5230_e6056 - 1.0);
        let assign5230_e6060: f64 = (assign5230_e6058).max(0.0);
        let assign5230_e6061: f64 = (2.0 * assign5230_e6060);
        (assign5230_e6061,)
    } else {
        (var_nuintd,)
    }
};
        var_nuintd = assign5230_e6063;
        var_nuintd_rv = 0.0;

        let (assign5240_e6077,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) && (var_guard81 != 0.0)) {
        (0.0,)
    } else {
        (var_nuends,)
    }
};
        var_nuends = assign5240_e6077;
        var_nuends_rv = 0.0;

        let (assign5250_e6091,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) && (var_guard81 != 0.0)) {
        (p.p2,)
    } else {
        (var_nuints,)
    }
};
        var_nuints = assign5250_e6091;
        var_nuints_rv = 0.0;

        let (assign5260_e6106,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) && (var_guard81 == 0.0)) {
        (0.0,)
    } else {
        (var_nuendd,)
    }
};
        var_nuendd = assign5260_e6106;
        var_nuendd_rv = 0.0;

        let (assign5270_e6121,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) && (var_guard81 == 0.0)) {
        (p.p2,)
    } else {
        (var_nuintd,)
    }
};
        var_nuintd = assign5270_e6121;
        var_nuintd_rv = 0.0;

        let (assign5280_e6136,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) && (var_guard81 == 0.0)) {
        (2.0,)
    } else {
        (var_nuends,)
    }
};
        var_nuends = assign5280_e6136;
        var_nuends_rv = 0.0;

        let (assign5290_e6159,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard79 != 0.0)) && (var_guard80 == 0.0)) && (var_guard81 == 0.0)) {
        let assign5290_e6152: f64 = (p.p2 / 2.0);
        let assign5290_e6154: f64 = (assign5290_e6152 - 1.0);
        let assign5290_e6156: f64 = (assign5290_e6154).max(0.0);
        let assign5290_e6157: f64 = (2.0 * assign5290_e6156);
        (assign5290_e6157,)
    } else {
        (var_nuints,)
    }
};
        var_nuints = assign5290_e6159;
        var_nuints_rv = 0.0;

        let assign5300_e6162: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard82 = assign5300_e6162;
        var_guard82_rv = 0.0;

        let assign5310_e6165: f64 = if var_nuints == 0.0 { 1.0 } else { 0.0 };
        var_guard83 = assign5310_e6165;
        var_guard83_rv = 0.0;

        let (assign5320_e6178,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard79 != 0.0)) && (var_guard82 != 0.0)) && (var_guard83 != 0.0)) {
        (0.0,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign5320_e6178;
        var_rint_rv = 0.0;

        let (assign5330_e6198,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard79 != 0.0)) && (var_guard82 != 0.0)) && (var_guard83 == 0.0)) {
        let assign5330_e6192: f64 = (p.p374 * var_dmcgeff);
        let assign5330_e6195: f64 = (var_weff * var_nuints);
        let assign5330_e6196: f64 = (assign5330_e6192 / assign5330_e6195);
        (assign5330_e6196,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign5330_e6198;
        var_rint_rv = 0.0;

        let assign5340_e6201: f64 = if var_nuintd == 0.0 { 1.0 } else { 0.0 };
        var_guard84 = assign5340_e6201;
        var_guard84_rv = 0.0;

        let (assign5350_e6215,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard79 != 0.0)) && (var_guard82 == 0.0)) && (var_guard84 != 0.0)) {
        (0.0,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign5350_e6215;
        var_rint_rv = 0.0;

        let (assign5360_e6236,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard79 != 0.0)) && (var_guard82 == 0.0)) && (var_guard84 == 0.0)) {
        let assign5360_e6230: f64 = (p.p374 * var_dmcgeff);
        let assign5360_e6233: f64 = (var_weff * var_nuintd);
        let assign5360_e6234: f64 = (assign5360_e6230 / assign5360_e6233);
        (assign5360_e6234,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign5360_e6236;
        var_rint_rv = 0.0;

        let assign5370_e6239: f64 = if p.p9 == 0.0 { 1.0 } else { 0.0 };
        var_guard85 = assign5370_e6239;
        var_guard85_rv = 0.0;

        let assign5380_e6242: f64 = if p.p9 == 1.0 { 1.0 } else { 0.0 };
        var_guard86 = assign5380_e6242;
        var_guard86_rv = 0.0;

        let assign5390_e6245: f64 = if p.p9 == 2.0 { 1.0 } else { 0.0 };
        var_guard87 = assign5390_e6245;
        var_guard87_rv = 0.0;

        let assign5400_e6248: f64 = if p.p9 == 3.0 { 1.0 } else { 0.0 };
        var_guard88 = assign5400_e6248;
        var_guard88_rv = 0.0;

        let assign5410_e6251: f64 = if p.p9 == 4.0 { 1.0 } else { 0.0 };
        var_guard89 = assign5410_e6251;
        var_guard89_rv = 0.0;

        let assign5420_e6254: f64 = if p.p9 == 5.0 { 1.0 } else { 0.0 };
        var_guard90 = assign5420_e6254;
        var_guard90_rv = 0.0;

        let assign5430_e6257: f64 = if p.p9 == 6.0 { 1.0 } else { 0.0 };
        var_guard91 = assign5430_e6257;
        var_guard91_rv = 0.0;

        let assign5440_e6260: f64 = if p.p9 == 7.0 { 1.0 } else { 0.0 };
        var_guard92 = assign5440_e6260;
        var_guard92_rv = 0.0;

        let assign5450_e6263: f64 = if p.p9 == 8.0 { 1.0 } else { 0.0 };
        var_guard93 = assign5450_e6263;
        var_guard93_rv = 0.0;

        let assign5460_e6266: f64 = if p.p9 == 9.0 { 1.0 } else { 0.0 };
        var_guard94 = assign5460_e6266;
        var_guard94_rv = 0.0;

        let assign5470_e6269: f64 = if p.p9 == 10.0 { 1.0 } else { 0.0 };
        var_guard95 = assign5470_e6269;
        var_guard95_rv = 0.0;

        let assign5480_e6272: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard96 = assign5480_e6272;
        var_guard96_rv = 0.0;

        let assign5490_e6275: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard97 = assign5490_e6275;
        var_guard97_rv = 0.0;

        let assign5500_e6286: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard98 = assign5500_e6286;
        var_guard98_rv = 0.0;

        let assign5510_e6297: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard99 = assign5510_e6297;
        var_guard99_rv = 0.0;

        let assign5520_e6300: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard100 = assign5520_e6300;
        var_guard100_rv = 0.0;

        let (assign5530_e6317,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 != 0.0)) && (var_guard97 != 0.0)) && (var_guard98 != 0.0)) && (var_guard100 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5530_e6317;
        var_rend_rv = 0.0;

        let (assign5540_e6341,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 != 0.0)) && (var_guard97 != 0.0)) && (var_guard98 != 0.0)) && (var_guard100 == 0.0)) {
        let assign5540_e6335: f64 = (p.p374 * var_dmcgeff);
        let assign5540_e6338: f64 = (var_weff * var_nuends);
        let assign5540_e6339: f64 = (assign5540_e6335 / assign5540_e6338);
        (assign5540_e6339,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5540_e6341;
        var_rend_rv = 0.0;

        let assign5560_e6352: f64 = (var_dmcgeff + var_dmcieff);
        let assign5560_e6355: f64 = if ((var_nuends == 0.0) || (assign5560_e6352 == 0.0)) { 1.0 } else { 0.0 };
        var_guard102 = assign5560_e6355;
        var_guard102_rv = 0.0;

        let (assign5570_e6375,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 != 0.0)) && (var_guard97 != 0.0)) && ((var_guard99 != 0.0) && (var_guard98 == 0.0))) && (var_guard102 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5570_e6375;
        var_rend_rv = 0.0;

        let (assign5580_e6406,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 != 0.0)) && (var_guard97 != 0.0)) && ((var_guard99 != 0.0) && (var_guard98 == 0.0))) && (var_guard102 == 0.0)) {
        let assign5580_e6396: f64 = (p.p374 * var_weff);
        let assign5580_e6399: f64 = (3.0 * var_nuends);
        let assign5580_e6402: f64 = (var_dmcgeff + var_dmcieff);
        let assign5580_e6403: f64 = (assign5580_e6399 * assign5580_e6402);
        let assign5580_e6404: f64 = (assign5580_e6396 / assign5580_e6403);
        (assign5580_e6404,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5580_e6406;
        var_rend_rv = 0.0;

        let (assign5590_e6424,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 != 0.0)) && (var_guard97 != 0.0)) && (!((var_guard98 != 0.0) || (var_guard99 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5590_e6424;
        var_rend_rv = 0.0;

        let assign5600_e6435: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard103 = assign5600_e6435;
        var_guard103_rv = 0.0;

        let assign5610_e6446: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard104 = assign5610_e6446;
        var_guard104_rv = 0.0;

        let assign5620_e6449: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard105 = assign5620_e6449;
        var_guard105_rv = 0.0;

        let (assign5630_e6467,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 != 0.0)) && (var_guard97 == 0.0)) && (var_guard103 != 0.0)) && (var_guard105 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5630_e6467;
        var_rend_rv = 0.0;

        let (assign5640_e6492,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 != 0.0)) && (var_guard97 == 0.0)) && (var_guard103 != 0.0)) && (var_guard105 == 0.0)) {
        let assign5640_e6486: f64 = (p.p374 * var_dmcgeff);
        let assign5640_e6489: f64 = (var_weff * var_nuends);
        let assign5640_e6490: f64 = (assign5640_e6486 / assign5640_e6489);
        (assign5640_e6490,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5640_e6492;
        var_rend_rv = 0.0;

        let assign5660_e6503: f64 = (var_dmcgeff + var_dmcieff);
        let assign5660_e6506: f64 = if ((var_nuends == 0.0) || (assign5660_e6503 == 0.0)) { 1.0 } else { 0.0 };
        var_guard107 = assign5660_e6506;
        var_guard107_rv = 0.0;

        let (assign5670_e6527,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 != 0.0)) && (var_guard97 == 0.0)) && ((var_guard104 != 0.0) && (var_guard103 == 0.0))) && (var_guard107 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5670_e6527;
        var_rend_rv = 0.0;

        let (assign5680_e6559,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 != 0.0)) && (var_guard97 == 0.0)) && ((var_guard104 != 0.0) && (var_guard103 == 0.0))) && (var_guard107 == 0.0)) {
        let assign5680_e6549: f64 = (p.p374 * var_weff);
        let assign5680_e6552: f64 = (3.0 * var_nuends);
        let assign5680_e6555: f64 = (var_dmcgeff + var_dmcieff);
        let assign5680_e6556: f64 = (assign5680_e6552 * assign5680_e6555);
        let assign5680_e6557: f64 = (assign5680_e6549 / assign5680_e6556);
        (assign5680_e6557,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5680_e6559;
        var_rend_rv = 0.0;

        let (assign5690_e6578,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 != 0.0)) && (var_guard97 == 0.0)) && (!((var_guard103 != 0.0) || (var_guard104 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5690_e6578;
        var_rend_rv = 0.0;

        let assign5700_e6581: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard108 = assign5700_e6581;
        var_guard108_rv = 0.0;

        let assign5710_e6592: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard109 = assign5710_e6592;
        var_guard109_rv = 0.0;

        let assign5720_e6603: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard110 = assign5720_e6603;
        var_guard110_rv = 0.0;

        let assign5730_e6606: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard111 = assign5730_e6606;
        var_guard111_rv = 0.0;

        let (assign5740_e6624,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 == 0.0)) && (var_guard108 != 0.0)) && (var_guard109 != 0.0)) && (var_guard111 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5740_e6624;
        var_rend_rv = 0.0;

        let (assign5750_e6649,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 == 0.0)) && (var_guard108 != 0.0)) && (var_guard109 != 0.0)) && (var_guard111 == 0.0)) {
        let assign5750_e6643: f64 = (p.p374 * var_dmcgeff);
        let assign5750_e6646: f64 = (var_weff * var_nuendd);
        let assign5750_e6647: f64 = (assign5750_e6643 / assign5750_e6646);
        (assign5750_e6647,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5750_e6649;
        var_rend_rv = 0.0;

        *var_dmcgeff_slot = var_dmcgeff;
        *var_dmcgeff_rv_slot = var_dmcgeff_rv;
        *var_dmcieff_slot = var_dmcieff;
        *var_dmcieff_rv_slot = var_dmcieff_rv;
        *var_dmdgeff_slot = var_dmdgeff;
        *var_dmdgeff_rv_slot = var_dmdgeff_rv;
        *var_guard100_slot = var_guard100;
        *var_guard100_rv_slot = var_guard100_rv;
        *var_guard102_slot = var_guard102;
        *var_guard102_rv_slot = var_guard102_rv;
        *var_guard103_slot = var_guard103;
        *var_guard103_rv_slot = var_guard103_rv;
        *var_guard104_slot = var_guard104;
        *var_guard104_rv_slot = var_guard104_rv;
        *var_guard105_slot = var_guard105;
        *var_guard105_rv_slot = var_guard105_rv;
        *var_guard107_slot = var_guard107;
        *var_guard107_rv_slot = var_guard107_rv;
        *var_guard108_slot = var_guard108;
        *var_guard108_rv_slot = var_guard108_rv;
        *var_guard109_slot = var_guard109;
        *var_guard109_rv_slot = var_guard109_rv;
        *var_guard110_slot = var_guard110;
        *var_guard110_rv_slot = var_guard110_rv;
        *var_guard111_slot = var_guard111;
        *var_guard111_rv_slot = var_guard111_rv;
        *var_guard77_slot = var_guard77;
        *var_guard77_rv_slot = var_guard77_rv;
        *var_guard78_slot = var_guard78;
        *var_guard78_rv_slot = var_guard78_rv;
        *var_guard79_slot = var_guard79;
        *var_guard79_rv_slot = var_guard79_rv;
        *var_guard80_slot = var_guard80;
        *var_guard80_rv_slot = var_guard80_rv;
        *var_guard81_slot = var_guard81;
        *var_guard81_rv_slot = var_guard81_rv;
        *var_guard82_slot = var_guard82;
        *var_guard82_rv_slot = var_guard82_rv;
        *var_guard83_slot = var_guard83;
        *var_guard83_rv_slot = var_guard83_rv;
        *var_guard84_slot = var_guard84;
        *var_guard84_rv_slot = var_guard84_rv;
        *var_guard85_slot = var_guard85;
        *var_guard85_rv_slot = var_guard85_rv;
        *var_guard86_slot = var_guard86;
        *var_guard86_rv_slot = var_guard86_rv;
        *var_guard87_slot = var_guard87;
        *var_guard87_rv_slot = var_guard87_rv;
        *var_guard88_slot = var_guard88;
        *var_guard88_rv_slot = var_guard88_rv;
        *var_guard89_slot = var_guard89;
        *var_guard89_rv_slot = var_guard89_rv;
        *var_guard90_slot = var_guard90;
        *var_guard90_rv_slot = var_guard90_rv;
        *var_guard91_slot = var_guard91;
        *var_guard91_rv_slot = var_guard91_rv;
        *var_guard92_slot = var_guard92;
        *var_guard92_rv_slot = var_guard92_rv;
        *var_guard93_slot = var_guard93;
        *var_guard93_rv_slot = var_guard93_rv;
        *var_guard94_slot = var_guard94;
        *var_guard94_rv_slot = var_guard94_rv;
        *var_guard95_slot = var_guard95;
        *var_guard95_rv_slot = var_guard95_rv;
        *var_guard96_slot = var_guard96;
        *var_guard96_rv_slot = var_guard96_rv;
        *var_guard97_slot = var_guard97;
        *var_guard97_rv_slot = var_guard97_rv;
        *var_guard98_slot = var_guard98;
        *var_guard98_rv_slot = var_guard98_rv;
        *var_guard99_slot = var_guard99;
        *var_guard99_rv_slot = var_guard99_rv;
        *var_lintnoi_i_slot = var_lintnoi_i;
        *var_lintnoi_i_rv_slot = var_lintnoi_i_rv;
        *var_nuendd_slot = var_nuendd;
        *var_nuendd_rv_slot = var_nuendd_rv;
        *var_nuends_slot = var_nuends;
        *var_nuends_rv_slot = var_nuends_rv;
        *var_nuintd_slot = var_nuintd;
        *var_nuintd_rv_slot = var_nuintd_rv;
        *var_nuints_slot = var_nuints;
        *var_nuints_rv_slot = var_nuints_rv;
        *var_rend_slot = var_rend;
        *var_rend_rv_slot = var_rend_rv;
        *var_rint_slot = var_rint;
        *var_rint_rv_slot = var_rint_rv;
        *var_rsourcegeo_slot = var_rsourcegeo;
        *var_rsourcegeo_rv_slot = var_rsourcegeo_rv;
    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        var_dmcgeff: f64,
        var_dmcieff: f64,
        var_guard108: f64,
        var_guard109: f64,
        var_guard110: f64,
        var_guard77: f64,
        var_guard78: f64,
        var_guard85: f64,
        var_guard86: f64,
        var_guard87: f64,
        var_guard96: f64,
        var_nuendd: f64,
        var_nuends: f64,
        var_weff: f64,
        var_guard113_slot: &mut f64,
        var_guard113_rv_slot: &mut f64,
        var_guard114_slot: &mut f64,
        var_guard114_rv_slot: &mut f64,
        var_guard115_slot: &mut f64,
        var_guard115_rv_slot: &mut f64,
        var_guard116_slot: &mut f64,
        var_guard116_rv_slot: &mut f64,
        var_guard118_slot: &mut f64,
        var_guard118_rv_slot: &mut f64,
        var_guard119_slot: &mut f64,
        var_guard119_rv_slot: &mut f64,
        var_guard120_slot: &mut f64,
        var_guard120_rv_slot: &mut f64,
        var_guard121_slot: &mut f64,
        var_guard121_rv_slot: &mut f64,
        var_guard122_slot: &mut f64,
        var_guard122_rv_slot: &mut f64,
        var_guard123_slot: &mut f64,
        var_guard123_rv_slot: &mut f64,
        var_guard125_slot: &mut f64,
        var_guard125_rv_slot: &mut f64,
        var_guard126_slot: &mut f64,
        var_guard126_rv_slot: &mut f64,
        var_guard127_slot: &mut f64,
        var_guard127_rv_slot: &mut f64,
        var_guard128_slot: &mut f64,
        var_guard128_rv_slot: &mut f64,
        var_guard130_slot: &mut f64,
        var_guard130_rv_slot: &mut f64,
        var_guard131_slot: &mut f64,
        var_guard131_rv_slot: &mut f64,
        var_guard132_slot: &mut f64,
        var_guard132_rv_slot: &mut f64,
        var_guard133_slot: &mut f64,
        var_guard133_rv_slot: &mut f64,
        var_guard134_slot: &mut f64,
        var_guard134_rv_slot: &mut f64,
        var_guard136_slot: &mut f64,
        var_guard136_rv_slot: &mut f64,
        var_guard137_slot: &mut f64,
        var_guard137_rv_slot: &mut f64,
        var_guard138_slot: &mut f64,
        var_guard138_rv_slot: &mut f64,
        var_guard139_slot: &mut f64,
        var_guard139_rv_slot: &mut f64,
        var_guard141_slot: &mut f64,
        var_guard141_rv_slot: &mut f64,
        var_guard142_slot: &mut f64,
        var_guard142_rv_slot: &mut f64,
        var_guard143_slot: &mut f64,
        var_guard143_rv_slot: &mut f64,
        var_guard144_slot: &mut f64,
        var_guard144_rv_slot: &mut f64,
        var_guard145_slot: &mut f64,
        var_guard145_rv_slot: &mut f64,
        var_guard146_slot: &mut f64,
        var_guard146_rv_slot: &mut f64,
        var_guard148_slot: &mut f64,
        var_guard148_rv_slot: &mut f64,
        var_guard149_slot: &mut f64,
        var_guard149_rv_slot: &mut f64,
        var_guard150_slot: &mut f64,
        var_guard150_rv_slot: &mut f64,
        var_rend_slot: &mut f64,
        var_rend_rv_slot: &mut f64,
    ) {
        let mut var_guard113: f64 = *var_guard113_slot;
        let mut var_guard113_rv: f64 = *var_guard113_rv_slot;
        let mut var_guard114: f64 = *var_guard114_slot;
        let mut var_guard114_rv: f64 = *var_guard114_rv_slot;
        let mut var_guard115: f64 = *var_guard115_slot;
        let mut var_guard115_rv: f64 = *var_guard115_rv_slot;
        let mut var_guard116: f64 = *var_guard116_slot;
        let mut var_guard116_rv: f64 = *var_guard116_rv_slot;
        let mut var_guard118: f64 = *var_guard118_slot;
        let mut var_guard118_rv: f64 = *var_guard118_rv_slot;
        let mut var_guard119: f64 = *var_guard119_slot;
        let mut var_guard119_rv: f64 = *var_guard119_rv_slot;
        let mut var_guard120: f64 = *var_guard120_slot;
        let mut var_guard120_rv: f64 = *var_guard120_rv_slot;
        let mut var_guard121: f64 = *var_guard121_slot;
        let mut var_guard121_rv: f64 = *var_guard121_rv_slot;
        let mut var_guard122: f64 = *var_guard122_slot;
        let mut var_guard122_rv: f64 = *var_guard122_rv_slot;
        let mut var_guard123: f64 = *var_guard123_slot;
        let mut var_guard123_rv: f64 = *var_guard123_rv_slot;
        let mut var_guard125: f64 = *var_guard125_slot;
        let mut var_guard125_rv: f64 = *var_guard125_rv_slot;
        let mut var_guard126: f64 = *var_guard126_slot;
        let mut var_guard126_rv: f64 = *var_guard126_rv_slot;
        let mut var_guard127: f64 = *var_guard127_slot;
        let mut var_guard127_rv: f64 = *var_guard127_rv_slot;
        let mut var_guard128: f64 = *var_guard128_slot;
        let mut var_guard128_rv: f64 = *var_guard128_rv_slot;
        let mut var_guard130: f64 = *var_guard130_slot;
        let mut var_guard130_rv: f64 = *var_guard130_rv_slot;
        let mut var_guard131: f64 = *var_guard131_slot;
        let mut var_guard131_rv: f64 = *var_guard131_rv_slot;
        let mut var_guard132: f64 = *var_guard132_slot;
        let mut var_guard132_rv: f64 = *var_guard132_rv_slot;
        let mut var_guard133: f64 = *var_guard133_slot;
        let mut var_guard133_rv: f64 = *var_guard133_rv_slot;
        let mut var_guard134: f64 = *var_guard134_slot;
        let mut var_guard134_rv: f64 = *var_guard134_rv_slot;
        let mut var_guard136: f64 = *var_guard136_slot;
        let mut var_guard136_rv: f64 = *var_guard136_rv_slot;
        let mut var_guard137: f64 = *var_guard137_slot;
        let mut var_guard137_rv: f64 = *var_guard137_rv_slot;
        let mut var_guard138: f64 = *var_guard138_slot;
        let mut var_guard138_rv: f64 = *var_guard138_rv_slot;
        let mut var_guard139: f64 = *var_guard139_slot;
        let mut var_guard139_rv: f64 = *var_guard139_rv_slot;
        let mut var_guard141: f64 = *var_guard141_slot;
        let mut var_guard141_rv: f64 = *var_guard141_rv_slot;
        let mut var_guard142: f64 = *var_guard142_slot;
        let mut var_guard142_rv: f64 = *var_guard142_rv_slot;
        let mut var_guard143: f64 = *var_guard143_slot;
        let mut var_guard143_rv: f64 = *var_guard143_rv_slot;
        let mut var_guard144: f64 = *var_guard144_slot;
        let mut var_guard144_rv: f64 = *var_guard144_rv_slot;
        let mut var_guard145: f64 = *var_guard145_slot;
        let mut var_guard145_rv: f64 = *var_guard145_rv_slot;
        let mut var_guard146: f64 = *var_guard146_slot;
        let mut var_guard146_rv: f64 = *var_guard146_rv_slot;
        let mut var_guard148: f64 = *var_guard148_slot;
        let mut var_guard148_rv: f64 = *var_guard148_rv_slot;
        let mut var_guard149: f64 = *var_guard149_slot;
        let mut var_guard149_rv: f64 = *var_guard149_rv_slot;
        let mut var_guard150: f64 = *var_guard150_slot;
        let mut var_guard150_rv: f64 = *var_guard150_rv_slot;
        let mut var_rend: f64 = *var_rend_slot;
        let mut var_rend_rv: f64 = *var_rend_rv_slot;

        let assign5770_e6660: f64 = (var_dmcgeff + var_dmcieff);
        let assign5770_e6663: f64 = if ((var_nuendd == 0.0) || (assign5770_e6660 == 0.0)) { 1.0 } else { 0.0 };
        var_guard113 = assign5770_e6663;
        var_guard113_rv = 0.0;

        let (assign5780_e6684,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 == 0.0)) && (var_guard108 != 0.0)) && ((var_guard110 != 0.0) && (var_guard109 == 0.0))) && (var_guard113 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5780_e6684;
        var_rend_rv = 0.0;

        let (assign5790_e6716,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 == 0.0)) && (var_guard108 != 0.0)) && ((var_guard110 != 0.0) && (var_guard109 == 0.0))) && (var_guard113 == 0.0)) {
        let assign5790_e6706: f64 = (p.p374 * var_weff);
        let assign5790_e6709: f64 = (3.0 * var_nuendd);
        let assign5790_e6712: f64 = (var_dmcgeff + var_dmcieff);
        let assign5790_e6713: f64 = (assign5790_e6709 * assign5790_e6712);
        let assign5790_e6714: f64 = (assign5790_e6706 / assign5790_e6713);
        (assign5790_e6714,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5790_e6716;
        var_rend_rv = 0.0;

        let (assign5800_e6735,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 == 0.0)) && (var_guard108 != 0.0)) && (!((var_guard109 != 0.0) || (var_guard110 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5800_e6735;
        var_rend_rv = 0.0;

        let assign5810_e6746: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard114 = assign5810_e6746;
        var_guard114_rv = 0.0;

        let assign5820_e6757: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard115 = assign5820_e6757;
        var_guard115_rv = 0.0;

        let assign5830_e6760: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard116 = assign5830_e6760;
        var_guard116_rv = 0.0;

        let (assign5840_e6779,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 == 0.0)) && (var_guard108 == 0.0)) && (var_guard114 != 0.0)) && (var_guard116 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5840_e6779;
        var_rend_rv = 0.0;

        let (assign5850_e6805,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 == 0.0)) && (var_guard108 == 0.0)) && (var_guard114 != 0.0)) && (var_guard116 == 0.0)) {
        let assign5850_e6799: f64 = (p.p374 * var_dmcgeff);
        let assign5850_e6802: f64 = (var_weff * var_nuendd);
        let assign5850_e6803: f64 = (assign5850_e6799 / assign5850_e6802);
        (assign5850_e6803,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5850_e6805;
        var_rend_rv = 0.0;

        let assign5870_e6816: f64 = (var_dmcgeff + var_dmcieff);
        let assign5870_e6819: f64 = if ((var_nuendd == 0.0) || (assign5870_e6816 == 0.0)) { 1.0 } else { 0.0 };
        var_guard118 = assign5870_e6819;
        var_guard118_rv = 0.0;

        let (assign5880_e6841,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 == 0.0)) && (var_guard108 == 0.0)) && ((var_guard115 != 0.0) && (var_guard114 == 0.0))) && (var_guard118 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5880_e6841;
        var_rend_rv = 0.0;

        let (assign5890_e6874,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 == 0.0)) && (var_guard108 == 0.0)) && ((var_guard115 != 0.0) && (var_guard114 == 0.0))) && (var_guard118 == 0.0)) {
        let assign5890_e6864: f64 = (p.p374 * var_weff);
        let assign5890_e6867: f64 = (3.0 * var_nuendd);
        let assign5890_e6870: f64 = (var_dmcgeff + var_dmcieff);
        let assign5890_e6871: f64 = (assign5890_e6867 * assign5890_e6870);
        let assign5890_e6872: f64 = (assign5890_e6864 / assign5890_e6871);
        (assign5890_e6872,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5890_e6874;
        var_rend_rv = 0.0;

        let (assign5900_e6894,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard85 != 0.0)) && (var_guard96 == 0.0)) && (var_guard108 == 0.0)) && (!((var_guard114 != 0.0) || (var_guard115 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5900_e6894;
        var_rend_rv = 0.0;

        let assign5910_e6897: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard119 = assign5910_e6897;
        var_guard119_rv = 0.0;

        let assign5920_e6900: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard120 = assign5920_e6900;
        var_guard120_rv = 0.0;

        let assign5930_e6911: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard121 = assign5930_e6911;
        var_guard121_rv = 0.0;

        let assign5940_e6922: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard122 = assign5940_e6922;
        var_guard122_rv = 0.0;

        let assign5950_e6925: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard123 = assign5950_e6925;
        var_guard123_rv = 0.0;

        let (assign5960_e6945,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 != 0.0)) && (var_guard120 != 0.0)) && (var_guard121 != 0.0)) && (var_guard123 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5960_e6945;
        var_rend_rv = 0.0;

        let (assign5970_e6972,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 != 0.0)) && (var_guard120 != 0.0)) && (var_guard121 != 0.0)) && (var_guard123 == 0.0)) {
        let assign5970_e6966: f64 = (p.p374 * var_dmcgeff);
        let assign5970_e6969: f64 = (var_weff * var_nuends);
        let assign5970_e6970: f64 = (assign5970_e6966 / assign5970_e6969);
        (assign5970_e6970,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign5970_e6972;
        var_rend_rv = 0.0;

        let assign5990_e6983: f64 = (var_dmcgeff + var_dmcieff);
        let assign5990_e6986: f64 = if ((var_nuends == 0.0) || (assign5990_e6983 == 0.0)) { 1.0 } else { 0.0 };
        var_guard125 = assign5990_e6986;
        var_guard125_rv = 0.0;

        let (assign6000_e7009,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 != 0.0)) && (var_guard120 != 0.0)) && ((var_guard122 != 0.0) && (var_guard121 == 0.0))) && (var_guard125 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6000_e7009;
        var_rend_rv = 0.0;

        let (assign6010_e7043,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 != 0.0)) && (var_guard120 != 0.0)) && ((var_guard122 != 0.0) && (var_guard121 == 0.0))) && (var_guard125 == 0.0)) {
        let assign6010_e7033: f64 = (p.p374 * var_weff);
        let assign6010_e7036: f64 = (3.0 * var_nuends);
        let assign6010_e7039: f64 = (var_dmcgeff + var_dmcieff);
        let assign6010_e7040: f64 = (assign6010_e7036 * assign6010_e7039);
        let assign6010_e7041: f64 = (assign6010_e7033 / assign6010_e7040);
        (assign6010_e7041,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6010_e7043;
        var_rend_rv = 0.0;

        let (assign6020_e7064,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 != 0.0)) && (var_guard120 != 0.0)) && (!((var_guard121 != 0.0) || (var_guard122 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6020_e7064;
        var_rend_rv = 0.0;

        let assign6030_e7075: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard126 = assign6030_e7075;
        var_guard126_rv = 0.0;

        let assign6040_e7086: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard127 = assign6040_e7086;
        var_guard127_rv = 0.0;

        let assign6050_e7089: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard128 = assign6050_e7089;
        var_guard128_rv = 0.0;

        let (assign6060_e7110,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 != 0.0)) && (var_guard120 == 0.0)) && (var_guard126 != 0.0)) && (var_guard128 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6060_e7110;
        var_rend_rv = 0.0;

        let (assign6070_e7138,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 != 0.0)) && (var_guard120 == 0.0)) && (var_guard126 != 0.0)) && (var_guard128 == 0.0)) {
        let assign6070_e7132: f64 = (p.p374 * var_dmcgeff);
        let assign6070_e7135: f64 = (var_weff * var_nuends);
        let assign6070_e7136: f64 = (assign6070_e7132 / assign6070_e7135);
        (assign6070_e7136,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6070_e7138;
        var_rend_rv = 0.0;

        let assign6090_e7149: f64 = (var_dmcgeff + var_dmcieff);
        let assign6090_e7152: f64 = if ((var_nuends == 0.0) || (assign6090_e7149 == 0.0)) { 1.0 } else { 0.0 };
        var_guard130 = assign6090_e7152;
        var_guard130_rv = 0.0;

        let (assign6100_e7176,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 != 0.0)) && (var_guard120 == 0.0)) && ((var_guard127 != 0.0) && (var_guard126 == 0.0))) && (var_guard130 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6100_e7176;
        var_rend_rv = 0.0;

        let (assign6110_e7211,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 != 0.0)) && (var_guard120 == 0.0)) && ((var_guard127 != 0.0) && (var_guard126 == 0.0))) && (var_guard130 == 0.0)) {
        let assign6110_e7201: f64 = (p.p374 * var_weff);
        let assign6110_e7204: f64 = (3.0 * var_nuends);
        let assign6110_e7207: f64 = (var_dmcgeff + var_dmcieff);
        let assign6110_e7208: f64 = (assign6110_e7204 * assign6110_e7207);
        let assign6110_e7209: f64 = (assign6110_e7201 / assign6110_e7208);
        (assign6110_e7209,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6110_e7211;
        var_rend_rv = 0.0;

        let (assign6120_e7233,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 != 0.0)) && (var_guard120 == 0.0)) && (!((var_guard126 != 0.0) || (var_guard127 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6120_e7233;
        var_rend_rv = 0.0;

        let assign6130_e7236: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard131 = assign6130_e7236;
        var_guard131_rv = 0.0;

        let assign6140_e7247: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard132 = assign6140_e7247;
        var_guard132_rv = 0.0;

        let assign6150_e7258: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard133 = assign6150_e7258;
        var_guard133_rv = 0.0;

        let assign6160_e7261: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard134 = assign6160_e7261;
        var_guard134_rv = 0.0;

        let (assign6170_e7282,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 == 0.0)) && (var_guard131 != 0.0)) && (var_guard132 != 0.0)) && (var_guard134 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6170_e7282;
        var_rend_rv = 0.0;

        let (assign6180_e7310,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 == 0.0)) && (var_guard131 != 0.0)) && (var_guard132 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6180_e7304: f64 = (p.p374 * var_dmcgeff);
        let assign6180_e7307: f64 = (var_weff * var_nuendd);
        let assign6180_e7308: f64 = (assign6180_e7304 / assign6180_e7307);
        (assign6180_e7308,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6180_e7310;
        var_rend_rv = 0.0;

        let assign6200_e7320: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard136 = assign6200_e7320;
        var_guard136_rv = 0.0;

        let (assign6210_e7344,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 == 0.0)) && (var_guard131 != 0.0)) && ((var_guard133 != 0.0) && (var_guard132 == 0.0))) && (var_guard136 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6210_e7344;
        var_rend_rv = 0.0;

        let (assign6220_e7377,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 == 0.0)) && (var_guard131 != 0.0)) && ((var_guard133 != 0.0) && (var_guard132 == 0.0))) && (var_guard136 == 0.0)) {
        let assign6220_e7369: f64 = (p.p374 * var_weff);
        let assign6220_e7372: f64 = (6.0 * var_nuendd);
        let assign6220_e7374: f64 = (assign6220_e7372 * var_dmcgeff);
        let assign6220_e7375: f64 = (assign6220_e7369 / assign6220_e7374);
        (assign6220_e7375,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6220_e7377;
        var_rend_rv = 0.0;

        let (assign6230_e7399,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 == 0.0)) && (var_guard131 != 0.0)) && (!((var_guard132 != 0.0) || (var_guard133 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6230_e7399;
        var_rend_rv = 0.0;

        let assign6240_e7410: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard137 = assign6240_e7410;
        var_guard137_rv = 0.0;

        let assign6250_e7421: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard138 = assign6250_e7421;
        var_guard138_rv = 0.0;

        let assign6260_e7424: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard139 = assign6260_e7424;
        var_guard139_rv = 0.0;

        let (assign6270_e7446,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 == 0.0)) && (var_guard131 == 0.0)) && (var_guard137 != 0.0)) && (var_guard139 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6270_e7446;
        var_rend_rv = 0.0;

        let (assign6280_e7475,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 == 0.0)) && (var_guard131 == 0.0)) && (var_guard137 != 0.0)) && (var_guard139 == 0.0)) {
        let assign6280_e7469: f64 = (p.p374 * var_dmcgeff);
        let assign6280_e7472: f64 = (var_weff * var_nuendd);
        let assign6280_e7473: f64 = (assign6280_e7469 / assign6280_e7472);
        (assign6280_e7473,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6280_e7475;
        var_rend_rv = 0.0;

        let assign6300_e7485: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard141 = assign6300_e7485;
        var_guard141_rv = 0.0;

        let (assign6310_e7510,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 == 0.0)) && (var_guard131 == 0.0)) && ((var_guard138 != 0.0) && (var_guard137 == 0.0))) && (var_guard141 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6310_e7510;
        var_rend_rv = 0.0;

        let (assign6320_e7544,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 == 0.0)) && (var_guard131 == 0.0)) && ((var_guard138 != 0.0) && (var_guard137 == 0.0))) && (var_guard141 == 0.0)) {
        let assign6320_e7536: f64 = (p.p374 * var_weff);
        let assign6320_e7539: f64 = (6.0 * var_nuendd);
        let assign6320_e7541: f64 = (assign6320_e7539 * var_dmcgeff);
        let assign6320_e7542: f64 = (assign6320_e7536 / assign6320_e7541);
        (assign6320_e7542,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6320_e7544;
        var_rend_rv = 0.0;

        let (assign6330_e7567,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard86 != 0.0) && (var_guard85 == 0.0))) && (var_guard119 == 0.0)) && (var_guard131 == 0.0)) && (!((var_guard137 != 0.0) || (var_guard138 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6330_e7567;
        var_rend_rv = 0.0;

        let assign6340_e7570: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard142 = assign6340_e7570;
        var_guard142_rv = 0.0;

        let assign6350_e7573: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard143 = assign6350_e7573;
        var_guard143_rv = 0.0;

        let assign6360_e7584: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard144 = assign6360_e7584;
        var_guard144_rv = 0.0;

        let assign6370_e7595: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard145 = assign6370_e7595;
        var_guard145_rv = 0.0;

        let assign6380_e7598: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard146 = assign6380_e7598;
        var_guard146_rv = 0.0;

        let (assign6390_e7620,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 != 0.0)) && (var_guard143 != 0.0)) && (var_guard144 != 0.0)) && (var_guard146 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6390_e7620;
        var_rend_rv = 0.0;

        let (assign6400_e7649,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 != 0.0)) && (var_guard143 != 0.0)) && (var_guard144 != 0.0)) && (var_guard146 == 0.0)) {
        let assign6400_e7643: f64 = (p.p374 * var_dmcgeff);
        let assign6400_e7646: f64 = (var_weff * var_nuends);
        let assign6400_e7647: f64 = (assign6400_e7643 / assign6400_e7646);
        (assign6400_e7647,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6400_e7649;
        var_rend_rv = 0.0;

        let assign6420_e7659: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard148 = assign6420_e7659;
        var_guard148_rv = 0.0;

        let (assign6430_e7684,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 != 0.0)) && (var_guard143 != 0.0)) && ((var_guard145 != 0.0) && (var_guard144 == 0.0))) && (var_guard148 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6430_e7684;
        var_rend_rv = 0.0;

        let (assign6440_e7718,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 != 0.0)) && (var_guard143 != 0.0)) && ((var_guard145 != 0.0) && (var_guard144 == 0.0))) && (var_guard148 == 0.0)) {
        let assign6440_e7710: f64 = (p.p374 * var_weff);
        let assign6440_e7713: f64 = (6.0 * var_nuends);
        let assign6440_e7715: f64 = (assign6440_e7713 * var_dmcgeff);
        let assign6440_e7716: f64 = (assign6440_e7710 / assign6440_e7715);
        (assign6440_e7716,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6440_e7718;
        var_rend_rv = 0.0;

        let (assign6450_e7741,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 != 0.0)) && (var_guard143 != 0.0)) && (!((var_guard144 != 0.0) || (var_guard145 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6450_e7741;
        var_rend_rv = 0.0;

        let assign6460_e7752: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard149 = assign6460_e7752;
        var_guard149_rv = 0.0;

        let assign6470_e7763: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard150 = assign6470_e7763;
        var_guard150_rv = 0.0;

        *var_guard113_slot = var_guard113;
        *var_guard113_rv_slot = var_guard113_rv;
        *var_guard114_slot = var_guard114;
        *var_guard114_rv_slot = var_guard114_rv;
        *var_guard115_slot = var_guard115;
        *var_guard115_rv_slot = var_guard115_rv;
        *var_guard116_slot = var_guard116;
        *var_guard116_rv_slot = var_guard116_rv;
        *var_guard118_slot = var_guard118;
        *var_guard118_rv_slot = var_guard118_rv;
        *var_guard119_slot = var_guard119;
        *var_guard119_rv_slot = var_guard119_rv;
        *var_guard120_slot = var_guard120;
        *var_guard120_rv_slot = var_guard120_rv;
        *var_guard121_slot = var_guard121;
        *var_guard121_rv_slot = var_guard121_rv;
        *var_guard122_slot = var_guard122;
        *var_guard122_rv_slot = var_guard122_rv;
        *var_guard123_slot = var_guard123;
        *var_guard123_rv_slot = var_guard123_rv;
        *var_guard125_slot = var_guard125;
        *var_guard125_rv_slot = var_guard125_rv;
        *var_guard126_slot = var_guard126;
        *var_guard126_rv_slot = var_guard126_rv;
        *var_guard127_slot = var_guard127;
        *var_guard127_rv_slot = var_guard127_rv;
        *var_guard128_slot = var_guard128;
        *var_guard128_rv_slot = var_guard128_rv;
        *var_guard130_slot = var_guard130;
        *var_guard130_rv_slot = var_guard130_rv;
        *var_guard131_slot = var_guard131;
        *var_guard131_rv_slot = var_guard131_rv;
        *var_guard132_slot = var_guard132;
        *var_guard132_rv_slot = var_guard132_rv;
        *var_guard133_slot = var_guard133;
        *var_guard133_rv_slot = var_guard133_rv;
        *var_guard134_slot = var_guard134;
        *var_guard134_rv_slot = var_guard134_rv;
        *var_guard136_slot = var_guard136;
        *var_guard136_rv_slot = var_guard136_rv;
        *var_guard137_slot = var_guard137;
        *var_guard137_rv_slot = var_guard137_rv;
        *var_guard138_slot = var_guard138;
        *var_guard138_rv_slot = var_guard138_rv;
        *var_guard139_slot = var_guard139;
        *var_guard139_rv_slot = var_guard139_rv;
        *var_guard141_slot = var_guard141;
        *var_guard141_rv_slot = var_guard141_rv;
        *var_guard142_slot = var_guard142;
        *var_guard142_rv_slot = var_guard142_rv;
        *var_guard143_slot = var_guard143;
        *var_guard143_rv_slot = var_guard143_rv;
        *var_guard144_slot = var_guard144;
        *var_guard144_rv_slot = var_guard144_rv;
        *var_guard145_slot = var_guard145;
        *var_guard145_rv_slot = var_guard145_rv;
        *var_guard146_slot = var_guard146;
        *var_guard146_rv_slot = var_guard146_rv;
        *var_guard148_slot = var_guard148;
        *var_guard148_rv_slot = var_guard148_rv;
        *var_guard149_slot = var_guard149;
        *var_guard149_rv_slot = var_guard149_rv;
        *var_guard150_slot = var_guard150;
        *var_guard150_rv_slot = var_guard150_rv;
        *var_rend_slot = var_rend;
        *var_rend_rv_slot = var_rend_rv;
    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        var_dmcgeff: f64,
        var_dmcieff: f64,
        var_guard142: f64,
        var_guard143: f64,
        var_guard149: f64,
        var_guard150: f64,
        var_guard77: f64,
        var_guard78: f64,
        var_guard85: f64,
        var_guard86: f64,
        var_guard87: f64,
        var_guard88: f64,
        var_nuendd: f64,
        var_nuends: f64,
        var_weff: f64,
        var_guard151_slot: &mut f64,
        var_guard151_rv_slot: &mut f64,
        var_guard153_slot: &mut f64,
        var_guard153_rv_slot: &mut f64,
        var_guard154_slot: &mut f64,
        var_guard154_rv_slot: &mut f64,
        var_guard155_slot: &mut f64,
        var_guard155_rv_slot: &mut f64,
        var_guard156_slot: &mut f64,
        var_guard156_rv_slot: &mut f64,
        var_guard157_slot: &mut f64,
        var_guard157_rv_slot: &mut f64,
        var_guard159_slot: &mut f64,
        var_guard159_rv_slot: &mut f64,
        var_guard160_slot: &mut f64,
        var_guard160_rv_slot: &mut f64,
        var_guard161_slot: &mut f64,
        var_guard161_rv_slot: &mut f64,
        var_guard162_slot: &mut f64,
        var_guard162_rv_slot: &mut f64,
        var_guard164_slot: &mut f64,
        var_guard164_rv_slot: &mut f64,
        var_guard165_slot: &mut f64,
        var_guard165_rv_slot: &mut f64,
        var_guard166_slot: &mut f64,
        var_guard166_rv_slot: &mut f64,
        var_guard167_slot: &mut f64,
        var_guard167_rv_slot: &mut f64,
        var_guard168_slot: &mut f64,
        var_guard168_rv_slot: &mut f64,
        var_guard169_slot: &mut f64,
        var_guard169_rv_slot: &mut f64,
        var_guard171_slot: &mut f64,
        var_guard171_rv_slot: &mut f64,
        var_guard172_slot: &mut f64,
        var_guard172_rv_slot: &mut f64,
        var_guard173_slot: &mut f64,
        var_guard173_rv_slot: &mut f64,
        var_guard174_slot: &mut f64,
        var_guard174_rv_slot: &mut f64,
        var_guard176_slot: &mut f64,
        var_guard176_rv_slot: &mut f64,
        var_guard177_slot: &mut f64,
        var_guard177_rv_slot: &mut f64,
        var_guard178_slot: &mut f64,
        var_guard178_rv_slot: &mut f64,
        var_guard179_slot: &mut f64,
        var_guard179_rv_slot: &mut f64,
        var_guard180_slot: &mut f64,
        var_guard180_rv_slot: &mut f64,
        var_guard182_slot: &mut f64,
        var_guard182_rv_slot: &mut f64,
        var_guard183_slot: &mut f64,
        var_guard183_rv_slot: &mut f64,
        var_guard184_slot: &mut f64,
        var_guard184_rv_slot: &mut f64,
        var_guard185_slot: &mut f64,
        var_guard185_rv_slot: &mut f64,
        var_guard187_slot: &mut f64,
        var_guard187_rv_slot: &mut f64,
        var_rend_slot: &mut f64,
        var_rend_rv_slot: &mut f64,
    ) {
        let mut var_guard151: f64 = *var_guard151_slot;
        let mut var_guard151_rv: f64 = *var_guard151_rv_slot;
        let mut var_guard153: f64 = *var_guard153_slot;
        let mut var_guard153_rv: f64 = *var_guard153_rv_slot;
        let mut var_guard154: f64 = *var_guard154_slot;
        let mut var_guard154_rv: f64 = *var_guard154_rv_slot;
        let mut var_guard155: f64 = *var_guard155_slot;
        let mut var_guard155_rv: f64 = *var_guard155_rv_slot;
        let mut var_guard156: f64 = *var_guard156_slot;
        let mut var_guard156_rv: f64 = *var_guard156_rv_slot;
        let mut var_guard157: f64 = *var_guard157_slot;
        let mut var_guard157_rv: f64 = *var_guard157_rv_slot;
        let mut var_guard159: f64 = *var_guard159_slot;
        let mut var_guard159_rv: f64 = *var_guard159_rv_slot;
        let mut var_guard160: f64 = *var_guard160_slot;
        let mut var_guard160_rv: f64 = *var_guard160_rv_slot;
        let mut var_guard161: f64 = *var_guard161_slot;
        let mut var_guard161_rv: f64 = *var_guard161_rv_slot;
        let mut var_guard162: f64 = *var_guard162_slot;
        let mut var_guard162_rv: f64 = *var_guard162_rv_slot;
        let mut var_guard164: f64 = *var_guard164_slot;
        let mut var_guard164_rv: f64 = *var_guard164_rv_slot;
        let mut var_guard165: f64 = *var_guard165_slot;
        let mut var_guard165_rv: f64 = *var_guard165_rv_slot;
        let mut var_guard166: f64 = *var_guard166_slot;
        let mut var_guard166_rv: f64 = *var_guard166_rv_slot;
        let mut var_guard167: f64 = *var_guard167_slot;
        let mut var_guard167_rv: f64 = *var_guard167_rv_slot;
        let mut var_guard168: f64 = *var_guard168_slot;
        let mut var_guard168_rv: f64 = *var_guard168_rv_slot;
        let mut var_guard169: f64 = *var_guard169_slot;
        let mut var_guard169_rv: f64 = *var_guard169_rv_slot;
        let mut var_guard171: f64 = *var_guard171_slot;
        let mut var_guard171_rv: f64 = *var_guard171_rv_slot;
        let mut var_guard172: f64 = *var_guard172_slot;
        let mut var_guard172_rv: f64 = *var_guard172_rv_slot;
        let mut var_guard173: f64 = *var_guard173_slot;
        let mut var_guard173_rv: f64 = *var_guard173_rv_slot;
        let mut var_guard174: f64 = *var_guard174_slot;
        let mut var_guard174_rv: f64 = *var_guard174_rv_slot;
        let mut var_guard176: f64 = *var_guard176_slot;
        let mut var_guard176_rv: f64 = *var_guard176_rv_slot;
        let mut var_guard177: f64 = *var_guard177_slot;
        let mut var_guard177_rv: f64 = *var_guard177_rv_slot;
        let mut var_guard178: f64 = *var_guard178_slot;
        let mut var_guard178_rv: f64 = *var_guard178_rv_slot;
        let mut var_guard179: f64 = *var_guard179_slot;
        let mut var_guard179_rv: f64 = *var_guard179_rv_slot;
        let mut var_guard180: f64 = *var_guard180_slot;
        let mut var_guard180_rv: f64 = *var_guard180_rv_slot;
        let mut var_guard182: f64 = *var_guard182_slot;
        let mut var_guard182_rv: f64 = *var_guard182_rv_slot;
        let mut var_guard183: f64 = *var_guard183_slot;
        let mut var_guard183_rv: f64 = *var_guard183_rv_slot;
        let mut var_guard184: f64 = *var_guard184_slot;
        let mut var_guard184_rv: f64 = *var_guard184_rv_slot;
        let mut var_guard185: f64 = *var_guard185_slot;
        let mut var_guard185_rv: f64 = *var_guard185_rv_slot;
        let mut var_guard187: f64 = *var_guard187_slot;
        let mut var_guard187_rv: f64 = *var_guard187_rv_slot;
        let mut var_rend: f64 = *var_rend_slot;
        let mut var_rend_rv: f64 = *var_rend_rv_slot;

        let assign6480_e7766: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard151 = assign6480_e7766;
        var_guard151_rv = 0.0;

        let (assign6490_e7789,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 != 0.0)) && (var_guard143 == 0.0)) && (var_guard149 != 0.0)) && (var_guard151 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6490_e7789;
        var_rend_rv = 0.0;

        let (assign6500_e7819,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 != 0.0)) && (var_guard143 == 0.0)) && (var_guard149 != 0.0)) && (var_guard151 == 0.0)) {
        let assign6500_e7813: f64 = (p.p374 * var_dmcgeff);
        let assign6500_e7816: f64 = (var_weff * var_nuends);
        let assign6500_e7817: f64 = (assign6500_e7813 / assign6500_e7816);
        (assign6500_e7817,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6500_e7819;
        var_rend_rv = 0.0;

        let assign6520_e7829: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard153 = assign6520_e7829;
        var_guard153_rv = 0.0;

        let (assign6530_e7855,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 != 0.0)) && (var_guard143 == 0.0)) && ((var_guard150 != 0.0) && (var_guard149 == 0.0))) && (var_guard153 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6530_e7855;
        var_rend_rv = 0.0;

        let (assign6540_e7890,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 != 0.0)) && (var_guard143 == 0.0)) && ((var_guard150 != 0.0) && (var_guard149 == 0.0))) && (var_guard153 == 0.0)) {
        let assign6540_e7882: f64 = (p.p374 * var_weff);
        let assign6540_e7885: f64 = (6.0 * var_nuends);
        let assign6540_e7887: f64 = (assign6540_e7885 * var_dmcgeff);
        let assign6540_e7888: f64 = (assign6540_e7882 / assign6540_e7887);
        (assign6540_e7888,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6540_e7890;
        var_rend_rv = 0.0;

        let (assign6550_e7914,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 != 0.0)) && (var_guard143 == 0.0)) && (!((var_guard149 != 0.0) || (var_guard150 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6550_e7914;
        var_rend_rv = 0.0;

        let assign6560_e7917: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard154 = assign6560_e7917;
        var_guard154_rv = 0.0;

        let assign6570_e7928: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard155 = assign6570_e7928;
        var_guard155_rv = 0.0;

        let assign6580_e7939: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard156 = assign6580_e7939;
        var_guard156_rv = 0.0;

        let assign6590_e7942: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard157 = assign6590_e7942;
        var_guard157_rv = 0.0;

        let (assign6600_e7965,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 == 0.0)) && (var_guard154 != 0.0)) && (var_guard155 != 0.0)) && (var_guard157 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6600_e7965;
        var_rend_rv = 0.0;

        let (assign6610_e7995,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 == 0.0)) && (var_guard154 != 0.0)) && (var_guard155 != 0.0)) && (var_guard157 == 0.0)) {
        let assign6610_e7989: f64 = (p.p374 * var_dmcgeff);
        let assign6610_e7992: f64 = (var_weff * var_nuendd);
        let assign6610_e7993: f64 = (assign6610_e7989 / assign6610_e7992);
        (assign6610_e7993,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6610_e7995;
        var_rend_rv = 0.0;

        let assign6630_e8006: f64 = (var_dmcgeff + var_dmcieff);
        let assign6630_e8009: f64 = if ((var_nuendd == 0.0) || (assign6630_e8006 == 0.0)) { 1.0 } else { 0.0 };
        var_guard159 = assign6630_e8009;
        var_guard159_rv = 0.0;

        let (assign6640_e8035,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 == 0.0)) && (var_guard154 != 0.0)) && ((var_guard156 != 0.0) && (var_guard155 == 0.0))) && (var_guard159 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6640_e8035;
        var_rend_rv = 0.0;

        let (assign6650_e8072,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 == 0.0)) && (var_guard154 != 0.0)) && ((var_guard156 != 0.0) && (var_guard155 == 0.0))) && (var_guard159 == 0.0)) {
        let assign6650_e8062: f64 = (p.p374 * var_weff);
        let assign6650_e8065: f64 = (3.0 * var_nuendd);
        let assign6650_e8068: f64 = (var_dmcgeff + var_dmcieff);
        let assign6650_e8069: f64 = (assign6650_e8065 * assign6650_e8068);
        let assign6650_e8070: f64 = (assign6650_e8062 / assign6650_e8069);
        (assign6650_e8070,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6650_e8072;
        var_rend_rv = 0.0;

        let (assign6660_e8096,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 == 0.0)) && (var_guard154 != 0.0)) && (!((var_guard155 != 0.0) || (var_guard156 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6660_e8096;
        var_rend_rv = 0.0;

        let assign6670_e8107: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard160 = assign6670_e8107;
        var_guard160_rv = 0.0;

        let assign6680_e8118: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard161 = assign6680_e8118;
        var_guard161_rv = 0.0;

        let assign6690_e8121: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard162 = assign6690_e8121;
        var_guard162_rv = 0.0;

        let (assign6700_e8145,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 == 0.0)) && (var_guard154 == 0.0)) && (var_guard160 != 0.0)) && (var_guard162 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6700_e8145;
        var_rend_rv = 0.0;

        let (assign6710_e8176,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 == 0.0)) && (var_guard154 == 0.0)) && (var_guard160 != 0.0)) && (var_guard162 == 0.0)) {
        let assign6710_e8170: f64 = (p.p374 * var_dmcgeff);
        let assign6710_e8173: f64 = (var_weff * var_nuendd);
        let assign6710_e8174: f64 = (assign6710_e8170 / assign6710_e8173);
        (assign6710_e8174,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6710_e8176;
        var_rend_rv = 0.0;

        let assign6730_e8187: f64 = (var_dmcgeff + var_dmcieff);
        let assign6730_e8190: f64 = if ((var_nuendd == 0.0) || (assign6730_e8187 == 0.0)) { 1.0 } else { 0.0 };
        var_guard164 = assign6730_e8190;
        var_guard164_rv = 0.0;

        let (assign6740_e8217,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 == 0.0)) && (var_guard154 == 0.0)) && ((var_guard161 != 0.0) && (var_guard160 == 0.0))) && (var_guard164 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6740_e8217;
        var_rend_rv = 0.0;

        let (assign6750_e8255,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 == 0.0)) && (var_guard154 == 0.0)) && ((var_guard161 != 0.0) && (var_guard160 == 0.0))) && (var_guard164 == 0.0)) {
        let assign6750_e8245: f64 = (p.p374 * var_weff);
        let assign6750_e8248: f64 = (3.0 * var_nuendd);
        let assign6750_e8251: f64 = (var_dmcgeff + var_dmcieff);
        let assign6750_e8252: f64 = (assign6750_e8248 * assign6750_e8251);
        let assign6750_e8253: f64 = (assign6750_e8245 / assign6750_e8252);
        (assign6750_e8253,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6750_e8255;
        var_rend_rv = 0.0;

        let (assign6760_e8280,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard87 != 0.0) && (!((var_guard85 != 0.0) || (var_guard86 != 0.0))))) && (var_guard142 == 0.0)) && (var_guard154 == 0.0)) && (!((var_guard160 != 0.0) || (var_guard161 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6760_e8280;
        var_rend_rv = 0.0;

        let assign6770_e8283: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard165 = assign6770_e8283;
        var_guard165_rv = 0.0;

        let assign6780_e8286: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard166 = assign6780_e8286;
        var_guard166_rv = 0.0;

        let assign6790_e8297: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard167 = assign6790_e8297;
        var_guard167_rv = 0.0;

        let assign6800_e8308: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard168 = assign6800_e8308;
        var_guard168_rv = 0.0;

        let assign6810_e8311: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard169 = assign6810_e8311;
        var_guard169_rv = 0.0;

        let (assign6820_e8335,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 != 0.0)) && (var_guard166 != 0.0)) && (var_guard167 != 0.0)) && (var_guard169 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6820_e8335;
        var_rend_rv = 0.0;

        let (assign6830_e8366,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 != 0.0)) && (var_guard166 != 0.0)) && (var_guard167 != 0.0)) && (var_guard169 == 0.0)) {
        let assign6830_e8360: f64 = (p.p374 * var_dmcgeff);
        let assign6830_e8363: f64 = (var_weff * var_nuends);
        let assign6830_e8364: f64 = (assign6830_e8360 / assign6830_e8363);
        (assign6830_e8364,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6830_e8366;
        var_rend_rv = 0.0;

        let assign6850_e8376: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard171 = assign6850_e8376;
        var_guard171_rv = 0.0;

        let (assign6860_e8403,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 != 0.0)) && (var_guard166 != 0.0)) && ((var_guard168 != 0.0) && (var_guard167 == 0.0))) && (var_guard171 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6860_e8403;
        var_rend_rv = 0.0;

        let (assign6870_e8439,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 != 0.0)) && (var_guard166 != 0.0)) && ((var_guard168 != 0.0) && (var_guard167 == 0.0))) && (var_guard171 == 0.0)) {
        let assign6870_e8431: f64 = (p.p374 * var_weff);
        let assign6870_e8434: f64 = (6.0 * var_nuends);
        let assign6870_e8436: f64 = (assign6870_e8434 * var_dmcgeff);
        let assign6870_e8437: f64 = (assign6870_e8431 / assign6870_e8436);
        (assign6870_e8437,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6870_e8439;
        var_rend_rv = 0.0;

        let (assign6880_e8464,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 != 0.0)) && (var_guard166 != 0.0)) && (!((var_guard167 != 0.0) || (var_guard168 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6880_e8464;
        var_rend_rv = 0.0;

        let assign6890_e8475: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard172 = assign6890_e8475;
        var_guard172_rv = 0.0;

        let assign6900_e8486: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard173 = assign6900_e8486;
        var_guard173_rv = 0.0;

        let assign6910_e8489: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard174 = assign6910_e8489;
        var_guard174_rv = 0.0;

        let (assign6920_e8514,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 != 0.0)) && (var_guard166 == 0.0)) && (var_guard172 != 0.0)) && (var_guard174 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6920_e8514;
        var_rend_rv = 0.0;

        let (assign6930_e8546,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 != 0.0)) && (var_guard166 == 0.0)) && (var_guard172 != 0.0)) && (var_guard174 == 0.0)) {
        let assign6930_e8540: f64 = (p.p374 * var_dmcgeff);
        let assign6930_e8543: f64 = (var_weff * var_nuends);
        let assign6930_e8544: f64 = (assign6930_e8540 / assign6930_e8543);
        (assign6930_e8544,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6930_e8546;
        var_rend_rv = 0.0;

        let assign6950_e8556: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard176 = assign6950_e8556;
        var_guard176_rv = 0.0;

        let (assign6960_e8584,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 != 0.0)) && (var_guard166 == 0.0)) && ((var_guard173 != 0.0) && (var_guard172 == 0.0))) && (var_guard176 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6960_e8584;
        var_rend_rv = 0.0;

        let (assign6970_e8621,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 != 0.0)) && (var_guard166 == 0.0)) && ((var_guard173 != 0.0) && (var_guard172 == 0.0))) && (var_guard176 == 0.0)) {
        let assign6970_e8613: f64 = (p.p374 * var_weff);
        let assign6970_e8616: f64 = (6.0 * var_nuends);
        let assign6970_e8618: f64 = (assign6970_e8616 * var_dmcgeff);
        let assign6970_e8619: f64 = (assign6970_e8613 / assign6970_e8618);
        (assign6970_e8619,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6970_e8621;
        var_rend_rv = 0.0;

        let (assign6980_e8647,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 != 0.0)) && (var_guard166 == 0.0)) && (!((var_guard172 != 0.0) || (var_guard173 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign6980_e8647;
        var_rend_rv = 0.0;

        let assign6990_e8650: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard177 = assign6990_e8650;
        var_guard177_rv = 0.0;

        let assign7000_e8661: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard178 = assign7000_e8661;
        var_guard178_rv = 0.0;

        let assign7010_e8672: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard179 = assign7010_e8672;
        var_guard179_rv = 0.0;

        let assign7020_e8675: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard180 = assign7020_e8675;
        var_guard180_rv = 0.0;

        let (assign7030_e8700,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 == 0.0)) && (var_guard177 != 0.0)) && (var_guard178 != 0.0)) && (var_guard180 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7030_e8700;
        var_rend_rv = 0.0;

        let (assign7040_e8732,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 == 0.0)) && (var_guard177 != 0.0)) && (var_guard178 != 0.0)) && (var_guard180 == 0.0)) {
        let assign7040_e8726: f64 = (p.p374 * var_dmcgeff);
        let assign7040_e8729: f64 = (var_weff * var_nuendd);
        let assign7040_e8730: f64 = (assign7040_e8726 / assign7040_e8729);
        (assign7040_e8730,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7040_e8732;
        var_rend_rv = 0.0;

        let assign7060_e8742: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard182 = assign7060_e8742;
        var_guard182_rv = 0.0;

        let (assign7070_e8770,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 == 0.0)) && (var_guard177 != 0.0)) && ((var_guard179 != 0.0) && (var_guard178 == 0.0))) && (var_guard182 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7070_e8770;
        var_rend_rv = 0.0;

        let (assign7080_e8807,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 == 0.0)) && (var_guard177 != 0.0)) && ((var_guard179 != 0.0) && (var_guard178 == 0.0))) && (var_guard182 == 0.0)) {
        let assign7080_e8799: f64 = (p.p374 * var_weff);
        let assign7080_e8802: f64 = (6.0 * var_nuendd);
        let assign7080_e8804: f64 = (assign7080_e8802 * var_dmcgeff);
        let assign7080_e8805: f64 = (assign7080_e8799 / assign7080_e8804);
        (assign7080_e8805,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7080_e8807;
        var_rend_rv = 0.0;

        let (assign7090_e8833,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 == 0.0)) && (var_guard177 != 0.0)) && (!((var_guard178 != 0.0) || (var_guard179 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7090_e8833;
        var_rend_rv = 0.0;

        let assign7100_e8844: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard183 = assign7100_e8844;
        var_guard183_rv = 0.0;

        let assign7110_e8855: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard184 = assign7110_e8855;
        var_guard184_rv = 0.0;

        let assign7120_e8858: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard185 = assign7120_e8858;
        var_guard185_rv = 0.0;

        let (assign7130_e8884,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 == 0.0)) && (var_guard177 == 0.0)) && (var_guard183 != 0.0)) && (var_guard185 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7130_e8884;
        var_rend_rv = 0.0;

        let (assign7140_e8917,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 == 0.0)) && (var_guard177 == 0.0)) && (var_guard183 != 0.0)) && (var_guard185 == 0.0)) {
        let assign7140_e8911: f64 = (p.p374 * var_dmcgeff);
        let assign7140_e8914: f64 = (var_weff * var_nuendd);
        let assign7140_e8915: f64 = (assign7140_e8911 / assign7140_e8914);
        (assign7140_e8915,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7140_e8917;
        var_rend_rv = 0.0;

        let assign7160_e8927: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard187 = assign7160_e8927;
        var_guard187_rv = 0.0;

        let (assign7170_e8956,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 == 0.0)) && (var_guard177 == 0.0)) && ((var_guard184 != 0.0) && (var_guard183 == 0.0))) && (var_guard187 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7170_e8956;
        var_rend_rv = 0.0;

        *var_guard151_slot = var_guard151;
        *var_guard151_rv_slot = var_guard151_rv;
        *var_guard153_slot = var_guard153;
        *var_guard153_rv_slot = var_guard153_rv;
        *var_guard154_slot = var_guard154;
        *var_guard154_rv_slot = var_guard154_rv;
        *var_guard155_slot = var_guard155;
        *var_guard155_rv_slot = var_guard155_rv;
        *var_guard156_slot = var_guard156;
        *var_guard156_rv_slot = var_guard156_rv;
        *var_guard157_slot = var_guard157;
        *var_guard157_rv_slot = var_guard157_rv;
        *var_guard159_slot = var_guard159;
        *var_guard159_rv_slot = var_guard159_rv;
        *var_guard160_slot = var_guard160;
        *var_guard160_rv_slot = var_guard160_rv;
        *var_guard161_slot = var_guard161;
        *var_guard161_rv_slot = var_guard161_rv;
        *var_guard162_slot = var_guard162;
        *var_guard162_rv_slot = var_guard162_rv;
        *var_guard164_slot = var_guard164;
        *var_guard164_rv_slot = var_guard164_rv;
        *var_guard165_slot = var_guard165;
        *var_guard165_rv_slot = var_guard165_rv;
        *var_guard166_slot = var_guard166;
        *var_guard166_rv_slot = var_guard166_rv;
        *var_guard167_slot = var_guard167;
        *var_guard167_rv_slot = var_guard167_rv;
        *var_guard168_slot = var_guard168;
        *var_guard168_rv_slot = var_guard168_rv;
        *var_guard169_slot = var_guard169;
        *var_guard169_rv_slot = var_guard169_rv;
        *var_guard171_slot = var_guard171;
        *var_guard171_rv_slot = var_guard171_rv;
        *var_guard172_slot = var_guard172;
        *var_guard172_rv_slot = var_guard172_rv;
        *var_guard173_slot = var_guard173;
        *var_guard173_rv_slot = var_guard173_rv;
        *var_guard174_slot = var_guard174;
        *var_guard174_rv_slot = var_guard174_rv;
        *var_guard176_slot = var_guard176;
        *var_guard176_rv_slot = var_guard176_rv;
        *var_guard177_slot = var_guard177;
        *var_guard177_rv_slot = var_guard177_rv;
        *var_guard178_slot = var_guard178;
        *var_guard178_rv_slot = var_guard178_rv;
        *var_guard179_slot = var_guard179;
        *var_guard179_rv_slot = var_guard179_rv;
        *var_guard180_slot = var_guard180;
        *var_guard180_rv_slot = var_guard180_rv;
        *var_guard182_slot = var_guard182;
        *var_guard182_rv_slot = var_guard182_rv;
        *var_guard183_slot = var_guard183;
        *var_guard183_rv_slot = var_guard183_rv;
        *var_guard184_slot = var_guard184;
        *var_guard184_rv_slot = var_guard184_rv;
        *var_guard185_slot = var_guard185;
        *var_guard185_rv_slot = var_guard185_rv;
        *var_guard187_slot = var_guard187;
        *var_guard187_rv_slot = var_guard187_rv;
        *var_rend_slot = var_rend;
        *var_rend_rv_slot = var_rend_rv;
    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        var_dmcgeff: f64,
        var_dmcieff: f64,
        var_dmdgeff: f64,
        var_guard165: f64,
        var_guard177: f64,
        var_guard183: f64,
        var_guard184: f64,
        var_guard187: f64,
        var_guard77: f64,
        var_guard78: f64,
        var_guard85: f64,
        var_guard86: f64,
        var_guard87: f64,
        var_guard88: f64,
        var_guard89: f64,
        var_guard90: f64,
        var_guard91: f64,
        var_nuendd: f64,
        var_nuends: f64,
        var_weff: f64,
        var_guard188_slot: &mut f64,
        var_guard188_rv_slot: &mut f64,
        var_guard189_slot: &mut f64,
        var_guard189_rv_slot: &mut f64,
        var_guard190_slot: &mut f64,
        var_guard190_rv_slot: &mut f64,
        var_guard191_slot: &mut f64,
        var_guard191_rv_slot: &mut f64,
        var_guard192_slot: &mut f64,
        var_guard192_rv_slot: &mut f64,
        var_guard194_slot: &mut f64,
        var_guard194_rv_slot: &mut f64,
        var_guard195_slot: &mut f64,
        var_guard195_rv_slot: &mut f64,
        var_guard196_slot: &mut f64,
        var_guard196_rv_slot: &mut f64,
        var_guard197_slot: &mut f64,
        var_guard197_rv_slot: &mut f64,
        var_guard199_slot: &mut f64,
        var_guard199_rv_slot: &mut f64,
        var_guard200_slot: &mut f64,
        var_guard200_rv_slot: &mut f64,
        var_guard201_slot: &mut f64,
        var_guard201_rv_slot: &mut f64,
        var_guard202_slot: &mut f64,
        var_guard202_rv_slot: &mut f64,
        var_guard203_slot: &mut f64,
        var_guard203_rv_slot: &mut f64,
        var_guard204_slot: &mut f64,
        var_guard204_rv_slot: &mut f64,
        var_guard206_slot: &mut f64,
        var_guard206_rv_slot: &mut f64,
        var_guard207_slot: &mut f64,
        var_guard207_rv_slot: &mut f64,
        var_guard208_slot: &mut f64,
        var_guard208_rv_slot: &mut f64,
        var_guard209_slot: &mut f64,
        var_guard209_rv_slot: &mut f64,
        var_guard211_slot: &mut f64,
        var_guard211_rv_slot: &mut f64,
        var_guard212_slot: &mut f64,
        var_guard212_rv_slot: &mut f64,
        var_guard213_slot: &mut f64,
        var_guard213_rv_slot: &mut f64,
        var_guard214_slot: &mut f64,
        var_guard214_rv_slot: &mut f64,
        var_guard215_slot: &mut f64,
        var_guard215_rv_slot: &mut f64,
        var_guard216_slot: &mut f64,
        var_guard216_rv_slot: &mut f64,
        var_guard217_slot: &mut f64,
        var_guard217_rv_slot: &mut f64,
        var_guard219_slot: &mut f64,
        var_guard219_rv_slot: &mut f64,
        var_guard220_slot: &mut f64,
        var_guard220_rv_slot: &mut f64,
        var_guard221_slot: &mut f64,
        var_guard221_rv_slot: &mut f64,
        var_guard222_slot: &mut f64,
        var_guard222_rv_slot: &mut f64,
        var_guard224_slot: &mut f64,
        var_guard224_rv_slot: &mut f64,
        var_rend_slot: &mut f64,
        var_rend_rv_slot: &mut f64,
    ) {
        let mut var_guard188: f64 = *var_guard188_slot;
        let mut var_guard188_rv: f64 = *var_guard188_rv_slot;
        let mut var_guard189: f64 = *var_guard189_slot;
        let mut var_guard189_rv: f64 = *var_guard189_rv_slot;
        let mut var_guard190: f64 = *var_guard190_slot;
        let mut var_guard190_rv: f64 = *var_guard190_rv_slot;
        let mut var_guard191: f64 = *var_guard191_slot;
        let mut var_guard191_rv: f64 = *var_guard191_rv_slot;
        let mut var_guard192: f64 = *var_guard192_slot;
        let mut var_guard192_rv: f64 = *var_guard192_rv_slot;
        let mut var_guard194: f64 = *var_guard194_slot;
        let mut var_guard194_rv: f64 = *var_guard194_rv_slot;
        let mut var_guard195: f64 = *var_guard195_slot;
        let mut var_guard195_rv: f64 = *var_guard195_rv_slot;
        let mut var_guard196: f64 = *var_guard196_slot;
        let mut var_guard196_rv: f64 = *var_guard196_rv_slot;
        let mut var_guard197: f64 = *var_guard197_slot;
        let mut var_guard197_rv: f64 = *var_guard197_rv_slot;
        let mut var_guard199: f64 = *var_guard199_slot;
        let mut var_guard199_rv: f64 = *var_guard199_rv_slot;
        let mut var_guard200: f64 = *var_guard200_slot;
        let mut var_guard200_rv: f64 = *var_guard200_rv_slot;
        let mut var_guard201: f64 = *var_guard201_slot;
        let mut var_guard201_rv: f64 = *var_guard201_rv_slot;
        let mut var_guard202: f64 = *var_guard202_slot;
        let mut var_guard202_rv: f64 = *var_guard202_rv_slot;
        let mut var_guard203: f64 = *var_guard203_slot;
        let mut var_guard203_rv: f64 = *var_guard203_rv_slot;
        let mut var_guard204: f64 = *var_guard204_slot;
        let mut var_guard204_rv: f64 = *var_guard204_rv_slot;
        let mut var_guard206: f64 = *var_guard206_slot;
        let mut var_guard206_rv: f64 = *var_guard206_rv_slot;
        let mut var_guard207: f64 = *var_guard207_slot;
        let mut var_guard207_rv: f64 = *var_guard207_rv_slot;
        let mut var_guard208: f64 = *var_guard208_slot;
        let mut var_guard208_rv: f64 = *var_guard208_rv_slot;
        let mut var_guard209: f64 = *var_guard209_slot;
        let mut var_guard209_rv: f64 = *var_guard209_rv_slot;
        let mut var_guard211: f64 = *var_guard211_slot;
        let mut var_guard211_rv: f64 = *var_guard211_rv_slot;
        let mut var_guard212: f64 = *var_guard212_slot;
        let mut var_guard212_rv: f64 = *var_guard212_rv_slot;
        let mut var_guard213: f64 = *var_guard213_slot;
        let mut var_guard213_rv: f64 = *var_guard213_rv_slot;
        let mut var_guard214: f64 = *var_guard214_slot;
        let mut var_guard214_rv: f64 = *var_guard214_rv_slot;
        let mut var_guard215: f64 = *var_guard215_slot;
        let mut var_guard215_rv: f64 = *var_guard215_rv_slot;
        let mut var_guard216: f64 = *var_guard216_slot;
        let mut var_guard216_rv: f64 = *var_guard216_rv_slot;
        let mut var_guard217: f64 = *var_guard217_slot;
        let mut var_guard217_rv: f64 = *var_guard217_rv_slot;
        let mut var_guard219: f64 = *var_guard219_slot;
        let mut var_guard219_rv: f64 = *var_guard219_rv_slot;
        let mut var_guard220: f64 = *var_guard220_slot;
        let mut var_guard220_rv: f64 = *var_guard220_rv_slot;
        let mut var_guard221: f64 = *var_guard221_slot;
        let mut var_guard221_rv: f64 = *var_guard221_rv_slot;
        let mut var_guard222: f64 = *var_guard222_slot;
        let mut var_guard222_rv: f64 = *var_guard222_rv_slot;
        let mut var_guard224: f64 = *var_guard224_slot;
        let mut var_guard224_rv: f64 = *var_guard224_rv_slot;
        let mut var_rend: f64 = *var_rend_slot;
        let mut var_rend_rv: f64 = *var_rend_rv_slot;

        let (assign7180_e8994,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 == 0.0)) && (var_guard177 == 0.0)) && ((var_guard184 != 0.0) && (var_guard183 == 0.0))) && (var_guard187 == 0.0)) {
        let assign7180_e8986: f64 = (p.p374 * var_weff);
        let assign7180_e8989: f64 = (6.0 * var_nuendd);
        let assign7180_e8991: f64 = (assign7180_e8989 * var_dmcgeff);
        let assign7180_e8992: f64 = (assign7180_e8986 / assign7180_e8991);
        (assign7180_e8992,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7180_e8994;
        var_rend_rv = 0.0;

        let (assign7190_e9021,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard88 != 0.0) && (!(((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0))))) && (var_guard165 == 0.0)) && (var_guard177 == 0.0)) && (!((var_guard183 != 0.0) || (var_guard184 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7190_e9021;
        var_rend_rv = 0.0;

        let assign7200_e9024: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard188 = assign7200_e9024;
        var_guard188_rv = 0.0;

        let assign7210_e9027: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard189 = assign7210_e9027;
        var_guard189_rv = 0.0;

        let assign7220_e9038: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard190 = assign7220_e9038;
        var_guard190_rv = 0.0;

        let assign7230_e9049: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard191 = assign7230_e9049;
        var_guard191_rv = 0.0;

        let assign7240_e9052: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard192 = assign7240_e9052;
        var_guard192_rv = 0.0;

        let (assign7250_e9078,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard89 != 0.0) && (!((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard188 != 0.0)) && (var_guard189 != 0.0)) && (var_guard190 != 0.0)) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7250_e9078;
        var_rend_rv = 0.0;

        let (assign7260_e9111,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard89 != 0.0) && (!((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard188 != 0.0)) && (var_guard189 != 0.0)) && (var_guard190 != 0.0)) && (var_guard192 == 0.0)) {
        let assign7260_e9105: f64 = (p.p374 * var_dmcgeff);
        let assign7260_e9108: f64 = (var_weff * var_nuends);
        let assign7260_e9109: f64 = (assign7260_e9105 / assign7260_e9108);
        (assign7260_e9109,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7260_e9111;
        var_rend_rv = 0.0;

        let assign7280_e9122: f64 = (var_dmcgeff + var_dmcieff);
        let assign7280_e9125: f64 = if ((var_nuends == 0.0) || (assign7280_e9122 == 0.0)) { 1.0 } else { 0.0 };
        var_guard194 = assign7280_e9125;
        var_guard194_rv = 0.0;

        let (assign7290_e9154,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard89 != 0.0) && (!((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard188 != 0.0)) && (var_guard189 != 0.0)) && ((var_guard191 != 0.0) && (var_guard190 == 0.0))) && (var_guard194 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7290_e9154;
        var_rend_rv = 0.0;

        let (assign7300_e9194,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard89 != 0.0) && (!((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard188 != 0.0)) && (var_guard189 != 0.0)) && ((var_guard191 != 0.0) && (var_guard190 == 0.0))) && (var_guard194 == 0.0)) {
        let assign7300_e9184: f64 = (p.p374 * var_weff);
        let assign7300_e9187: f64 = (3.0 * var_nuends);
        let assign7300_e9190: f64 = (var_dmcgeff + var_dmcieff);
        let assign7300_e9191: f64 = (assign7300_e9187 * assign7300_e9190);
        let assign7300_e9192: f64 = (assign7300_e9184 / assign7300_e9191);
        (assign7300_e9192,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7300_e9194;
        var_rend_rv = 0.0;

        let (assign7310_e9221,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard89 != 0.0) && (!((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard188 != 0.0)) && (var_guard189 != 0.0)) && (!((var_guard190 != 0.0) || (var_guard191 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7310_e9221;
        var_rend_rv = 0.0;

        let assign7320_e9232: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard195 = assign7320_e9232;
        var_guard195_rv = 0.0;

        let assign7330_e9243: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard196 = assign7330_e9243;
        var_guard196_rv = 0.0;

        let assign7340_e9246: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard197 = assign7340_e9246;
        var_guard197_rv = 0.0;

        let (assign7350_e9273,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard89 != 0.0) && (!((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard188 != 0.0)) && (var_guard189 == 0.0)) && (var_guard195 != 0.0)) && (var_guard197 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7350_e9273;
        var_rend_rv = 0.0;

        let (assign7360_e9307,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard89 != 0.0) && (!((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard188 != 0.0)) && (var_guard189 == 0.0)) && (var_guard195 != 0.0)) && (var_guard197 == 0.0)) {
        let assign7360_e9301: f64 = (p.p374 * var_dmcgeff);
        let assign7360_e9304: f64 = (var_weff * var_nuends);
        let assign7360_e9305: f64 = (assign7360_e9301 / assign7360_e9304);
        (assign7360_e9305,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7360_e9307;
        var_rend_rv = 0.0;

        let assign7380_e9318: f64 = (var_dmcgeff + var_dmcieff);
        let assign7380_e9321: f64 = if ((var_nuends == 0.0) || (assign7380_e9318 == 0.0)) { 1.0 } else { 0.0 };
        var_guard199 = assign7380_e9321;
        var_guard199_rv = 0.0;

        let (assign7390_e9351,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard89 != 0.0) && (!((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard188 != 0.0)) && (var_guard189 == 0.0)) && ((var_guard196 != 0.0) && (var_guard195 == 0.0))) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7390_e9351;
        var_rend_rv = 0.0;

        let (assign7400_e9392,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard89 != 0.0) && (!((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard188 != 0.0)) && (var_guard189 == 0.0)) && ((var_guard196 != 0.0) && (var_guard195 == 0.0))) && (var_guard199 == 0.0)) {
        let assign7400_e9382: f64 = (p.p374 * var_weff);
        let assign7400_e9385: f64 = (3.0 * var_nuends);
        let assign7400_e9388: f64 = (var_dmcgeff + var_dmcieff);
        let assign7400_e9389: f64 = (assign7400_e9385 * assign7400_e9388);
        let assign7400_e9390: f64 = (assign7400_e9382 / assign7400_e9389);
        (assign7400_e9390,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7400_e9392;
        var_rend_rv = 0.0;

        let (assign7410_e9420,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard89 != 0.0) && (!((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard188 != 0.0)) && (var_guard189 == 0.0)) && (!((var_guard195 != 0.0) || (var_guard196 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7410_e9420;
        var_rend_rv = 0.0;

        let (assign7420_e9445,) = {
    if ((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard89 != 0.0) && (!((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0))))) && (var_guard188 == 0.0)) {
        let assign7420_e9441: f64 = (p.p374 * var_dmdgeff);
        let assign7420_e9443: f64 = (assign7420_e9441 / var_weff);
        (assign7420_e9443,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7420_e9445;
        var_rend_rv = 0.0;

        let assign7430_e9448: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard200 = assign7430_e9448;
        var_guard200_rv = 0.0;

        let assign7440_e9451: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard201 = assign7440_e9451;
        var_guard201_rv = 0.0;

        let assign7450_e9462: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard202 = assign7450_e9462;
        var_guard202_rv = 0.0;

        let assign7460_e9473: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard203 = assign7460_e9473;
        var_guard203_rv = 0.0;

        let assign7470_e9476: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard204 = assign7470_e9476;
        var_guard204_rv = 0.0;

        let (assign7480_e9504,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard90 != 0.0) && (!(((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard200 != 0.0)) && (var_guard201 != 0.0)) && (var_guard202 != 0.0)) && (var_guard204 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7480_e9504;
        var_rend_rv = 0.0;

        let (assign7490_e9539,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard90 != 0.0) && (!(((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard200 != 0.0)) && (var_guard201 != 0.0)) && (var_guard202 != 0.0)) && (var_guard204 == 0.0)) {
        let assign7490_e9533: f64 = (p.p374 * var_dmcgeff);
        let assign7490_e9536: f64 = (var_weff * var_nuends);
        let assign7490_e9537: f64 = (assign7490_e9533 / assign7490_e9536);
        (assign7490_e9537,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7490_e9539;
        var_rend_rv = 0.0;

        let assign7510_e9549: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard206 = assign7510_e9549;
        var_guard206_rv = 0.0;

        let (assign7520_e9580,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard90 != 0.0) && (!(((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard200 != 0.0)) && (var_guard201 != 0.0)) && ((var_guard203 != 0.0) && (var_guard202 == 0.0))) && (var_guard206 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7520_e9580;
        var_rend_rv = 0.0;

        let (assign7530_e9620,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard90 != 0.0) && (!(((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard200 != 0.0)) && (var_guard201 != 0.0)) && ((var_guard203 != 0.0) && (var_guard202 == 0.0))) && (var_guard206 == 0.0)) {
        let assign7530_e9612: f64 = (p.p374 * var_weff);
        let assign7530_e9615: f64 = (6.0 * var_nuends);
        let assign7530_e9617: f64 = (assign7530_e9615 * var_dmcgeff);
        let assign7530_e9618: f64 = (assign7530_e9612 / assign7530_e9617);
        (assign7530_e9618,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7530_e9620;
        var_rend_rv = 0.0;

        let (assign7540_e9649,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard90 != 0.0) && (!(((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard200 != 0.0)) && (var_guard201 != 0.0)) && (!((var_guard202 != 0.0) || (var_guard203 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7540_e9649;
        var_rend_rv = 0.0;

        let assign7550_e9660: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard207 = assign7550_e9660;
        var_guard207_rv = 0.0;

        let assign7560_e9671: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard208 = assign7560_e9671;
        var_guard208_rv = 0.0;

        let assign7570_e9674: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard209 = assign7570_e9674;
        var_guard209_rv = 0.0;

        let (assign7580_e9703,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard90 != 0.0) && (!(((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard200 != 0.0)) && (var_guard201 == 0.0)) && (var_guard207 != 0.0)) && (var_guard209 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7580_e9703;
        var_rend_rv = 0.0;

        let (assign7590_e9739,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard90 != 0.0) && (!(((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard200 != 0.0)) && (var_guard201 == 0.0)) && (var_guard207 != 0.0)) && (var_guard209 == 0.0)) {
        let assign7590_e9733: f64 = (p.p374 * var_dmcgeff);
        let assign7590_e9736: f64 = (var_weff * var_nuends);
        let assign7590_e9737: f64 = (assign7590_e9733 / assign7590_e9736);
        (assign7590_e9737,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7590_e9739;
        var_rend_rv = 0.0;

        let assign7610_e9749: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard211 = assign7610_e9749;
        var_guard211_rv = 0.0;

        let (assign7620_e9781,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard90 != 0.0) && (!(((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard200 != 0.0)) && (var_guard201 == 0.0)) && ((var_guard208 != 0.0) && (var_guard207 == 0.0))) && (var_guard211 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7620_e9781;
        var_rend_rv = 0.0;

        let (assign7630_e9822,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard90 != 0.0) && (!(((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard200 != 0.0)) && (var_guard201 == 0.0)) && ((var_guard208 != 0.0) && (var_guard207 == 0.0))) && (var_guard211 == 0.0)) {
        let assign7630_e9814: f64 = (p.p374 * var_weff);
        let assign7630_e9817: f64 = (6.0 * var_nuends);
        let assign7630_e9819: f64 = (assign7630_e9817 * var_dmcgeff);
        let assign7630_e9820: f64 = (assign7630_e9814 / assign7630_e9819);
        (assign7630_e9820,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7630_e9822;
        var_rend_rv = 0.0;

        let (assign7640_e9852,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard90 != 0.0) && (!(((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard200 != 0.0)) && (var_guard201 == 0.0)) && (!((var_guard207 != 0.0) || (var_guard208 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7640_e9852;
        var_rend_rv = 0.0;

        let assign7650_e9855: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard212 = assign7650_e9855;
        var_guard212_rv = 0.0;

        let (assign7660_e9880,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard90 != 0.0) && (!(((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard200 == 0.0)) && (var_guard212 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7660_e9880;
        var_rend_rv = 0.0;

        let (assign7670_e9912,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard90 != 0.0) && (!(((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0))))) && (var_guard200 == 0.0)) && (var_guard212 == 0.0)) {
        let assign7670_e9906: f64 = (p.p374 * var_dmdgeff);
        let assign7670_e9909: f64 = (var_weff * var_nuendd);
        let assign7670_e9910: f64 = (assign7670_e9906 / assign7670_e9909);
        (assign7670_e9910,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7670_e9912;
        var_rend_rv = 0.0;

        let assign7680_e9915: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard213 = assign7680_e9915;
        var_guard213_rv = 0.0;

        let (assign7690_e9943,) = {
    if ((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard91 != 0.0) && (!((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard213 != 0.0)) {
        let assign7690_e9939: f64 = (p.p374 * var_dmdgeff);
        let assign7690_e9941: f64 = (assign7690_e9939 / var_weff);
        (assign7690_e9941,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7690_e9943;
        var_rend_rv = 0.0;

        let assign7700_e9946: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard214 = assign7700_e9946;
        var_guard214_rv = 0.0;

        let assign7710_e9957: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard215 = assign7710_e9957;
        var_guard215_rv = 0.0;

        let assign7720_e9968: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard216 = assign7720_e9968;
        var_guard216_rv = 0.0;

        let assign7730_e9971: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard217 = assign7730_e9971;
        var_guard217_rv = 0.0;

        let (assign7740_e10002,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard91 != 0.0) && (!((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard213 == 0.0)) && (var_guard214 != 0.0)) && (var_guard215 != 0.0)) && (var_guard217 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7740_e10002;
        var_rend_rv = 0.0;

        let (assign7750_e10040,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard91 != 0.0) && (!((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard213 == 0.0)) && (var_guard214 != 0.0)) && (var_guard215 != 0.0)) && (var_guard217 == 0.0)) {
        let assign7750_e10034: f64 = (p.p374 * var_dmcgeff);
        let assign7750_e10037: f64 = (var_weff * var_nuendd);
        let assign7750_e10038: f64 = (assign7750_e10034 / assign7750_e10037);
        (assign7750_e10038,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7750_e10040;
        var_rend_rv = 0.0;

        let assign7770_e10051: f64 = (var_dmcgeff + var_dmcieff);
        let assign7770_e10054: f64 = if ((var_nuendd == 0.0) || (assign7770_e10051 == 0.0)) { 1.0 } else { 0.0 };
        var_guard219 = assign7770_e10054;
        var_guard219_rv = 0.0;

        let (assign7780_e10088,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard91 != 0.0) && (!((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard213 == 0.0)) && (var_guard214 != 0.0)) && ((var_guard216 != 0.0) && (var_guard215 == 0.0))) && (var_guard219 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7780_e10088;
        var_rend_rv = 0.0;

        let (assign7790_e10133,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard91 != 0.0) && (!((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard213 == 0.0)) && (var_guard214 != 0.0)) && ((var_guard216 != 0.0) && (var_guard215 == 0.0))) && (var_guard219 == 0.0)) {
        let assign7790_e10123: f64 = (p.p374 * var_weff);
        let assign7790_e10126: f64 = (3.0 * var_nuendd);
        let assign7790_e10129: f64 = (var_dmcgeff + var_dmcieff);
        let assign7790_e10130: f64 = (assign7790_e10126 * assign7790_e10129);
        let assign7790_e10131: f64 = (assign7790_e10123 / assign7790_e10130);
        (assign7790_e10131,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7790_e10133;
        var_rend_rv = 0.0;

        let (assign7800_e10165,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard91 != 0.0) && (!((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard213 == 0.0)) && (var_guard214 != 0.0)) && (!((var_guard215 != 0.0) || (var_guard216 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7800_e10165;
        var_rend_rv = 0.0;

        let assign7810_e10176: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard220 = assign7810_e10176;
        var_guard220_rv = 0.0;

        let assign7820_e10187: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard221 = assign7820_e10187;
        var_guard221_rv = 0.0;

        let assign7830_e10190: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard222 = assign7830_e10190;
        var_guard222_rv = 0.0;

        let (assign7840_e10222,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard91 != 0.0) && (!((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard213 == 0.0)) && (var_guard214 == 0.0)) && (var_guard220 != 0.0)) && (var_guard222 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7840_e10222;
        var_rend_rv = 0.0;

        let (assign7850_e10261,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard91 != 0.0) && (!((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard213 == 0.0)) && (var_guard214 == 0.0)) && (var_guard220 != 0.0)) && (var_guard222 == 0.0)) {
        let assign7850_e10255: f64 = (p.p374 * var_dmcgeff);
        let assign7850_e10258: f64 = (var_weff * var_nuendd);
        let assign7850_e10259: f64 = (assign7850_e10255 / assign7850_e10258);
        (assign7850_e10259,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7850_e10261;
        var_rend_rv = 0.0;

        let assign7870_e10272: f64 = (var_dmcgeff + var_dmcieff);
        let assign7870_e10275: f64 = if ((var_nuendd == 0.0) || (assign7870_e10272 == 0.0)) { 1.0 } else { 0.0 };
        var_guard224 = assign7870_e10275;
        var_guard224_rv = 0.0;

        *var_guard188_slot = var_guard188;
        *var_guard188_rv_slot = var_guard188_rv;
        *var_guard189_slot = var_guard189;
        *var_guard189_rv_slot = var_guard189_rv;
        *var_guard190_slot = var_guard190;
        *var_guard190_rv_slot = var_guard190_rv;
        *var_guard191_slot = var_guard191;
        *var_guard191_rv_slot = var_guard191_rv;
        *var_guard192_slot = var_guard192;
        *var_guard192_rv_slot = var_guard192_rv;
        *var_guard194_slot = var_guard194;
        *var_guard194_rv_slot = var_guard194_rv;
        *var_guard195_slot = var_guard195;
        *var_guard195_rv_slot = var_guard195_rv;
        *var_guard196_slot = var_guard196;
        *var_guard196_rv_slot = var_guard196_rv;
        *var_guard197_slot = var_guard197;
        *var_guard197_rv_slot = var_guard197_rv;
        *var_guard199_slot = var_guard199;
        *var_guard199_rv_slot = var_guard199_rv;
        *var_guard200_slot = var_guard200;
        *var_guard200_rv_slot = var_guard200_rv;
        *var_guard201_slot = var_guard201;
        *var_guard201_rv_slot = var_guard201_rv;
        *var_guard202_slot = var_guard202;
        *var_guard202_rv_slot = var_guard202_rv;
        *var_guard203_slot = var_guard203;
        *var_guard203_rv_slot = var_guard203_rv;
        *var_guard204_slot = var_guard204;
        *var_guard204_rv_slot = var_guard204_rv;
        *var_guard206_slot = var_guard206;
        *var_guard206_rv_slot = var_guard206_rv;
        *var_guard207_slot = var_guard207;
        *var_guard207_rv_slot = var_guard207_rv;
        *var_guard208_slot = var_guard208;
        *var_guard208_rv_slot = var_guard208_rv;
        *var_guard209_slot = var_guard209;
        *var_guard209_rv_slot = var_guard209_rv;
        *var_guard211_slot = var_guard211;
        *var_guard211_rv_slot = var_guard211_rv;
        *var_guard212_slot = var_guard212;
        *var_guard212_rv_slot = var_guard212_rv;
        *var_guard213_slot = var_guard213;
        *var_guard213_rv_slot = var_guard213_rv;
        *var_guard214_slot = var_guard214;
        *var_guard214_rv_slot = var_guard214_rv;
        *var_guard215_slot = var_guard215;
        *var_guard215_rv_slot = var_guard215_rv;
        *var_guard216_slot = var_guard216;
        *var_guard216_rv_slot = var_guard216_rv;
        *var_guard217_slot = var_guard217;
        *var_guard217_rv_slot = var_guard217_rv;
        *var_guard219_slot = var_guard219;
        *var_guard219_rv_slot = var_guard219_rv;
        *var_guard220_slot = var_guard220;
        *var_guard220_rv_slot = var_guard220_rv;
        *var_guard221_slot = var_guard221;
        *var_guard221_rv_slot = var_guard221_rv;
        *var_guard222_slot = var_guard222;
        *var_guard222_rv_slot = var_guard222_rv;
        *var_guard224_slot = var_guard224;
        *var_guard224_rv_slot = var_guard224_rv;
        *var_rend_slot = var_rend;
        *var_rend_rv_slot = var_rend_rv;
    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_dmcgeff: f64,
        var_dmcieff: f64,
        var_dmdgeff: f64,
        var_guard213: f64,
        var_guard214: f64,
        var_guard220: f64,
        var_guard221: f64,
        var_guard224: f64,
        var_guard77: f64,
        var_guard78: f64,
        var_guard85: f64,
        var_guard86: f64,
        var_guard87: f64,
        var_guard88: f64,
        var_guard89: f64,
        var_guard90: f64,
        var_guard91: f64,
        var_guard92: f64,
        var_guard93: f64,
        var_guard94: f64,
        var_guard95: f64,
        var_weff: f64,
        var_guard225_slot: &mut f64,
        var_guard225_rv_slot: &mut f64,
        var_guard226_slot: &mut f64,
        var_guard226_rv_slot: &mut f64,
        var_guard227_slot: &mut f64,
        var_guard227_rv_slot: &mut f64,
        var_guard228_slot: &mut f64,
        var_guard228_rv_slot: &mut f64,
        var_guard229_slot: &mut f64,
        var_guard229_rv_slot: &mut f64,
        var_guard230_slot: &mut f64,
        var_guard230_rv_slot: &mut f64,
        var_guard232_slot: &mut f64,
        var_guard232_rv_slot: &mut f64,
        var_guard233_slot: &mut f64,
        var_guard233_rv_slot: &mut f64,
        var_guard234_slot: &mut f64,
        var_guard234_rv_slot: &mut f64,
        var_guard235_slot: &mut f64,
        var_guard235_rv_slot: &mut f64,
        var_guard237_slot: &mut f64,
        var_guard237_rv_slot: &mut f64,
        var_guard238_slot: &mut f64,
        var_guard238_rv_slot: &mut f64,
        var_guard239_slot: &mut f64,
        var_guard239_rv_slot: &mut f64,
        var_guard240_slot: &mut f64,
        var_guard240_rv_slot: &mut f64,
        var_guard241_slot: &mut f64,
        var_guard241_rv_slot: &mut f64,
        var_guard242_slot: &mut f64,
        var_guard242_rv_slot: &mut f64,
        var_guard243_slot: &mut f64,
        var_guard243_rv_slot: &mut f64,
        var_guard245_slot: &mut f64,
        var_guard245_rv_slot: &mut f64,
        var_guard246_slot: &mut f64,
        var_guard246_rv_slot: &mut f64,
        var_guard247_slot: &mut f64,
        var_guard247_rv_slot: &mut f64,
        var_guard248_slot: &mut f64,
        var_guard248_rv_slot: &mut f64,
        var_guard249_slot: &mut f64,
        var_guard249_rv_slot: &mut f64,
        var_nuendd_slot: &mut f64,
        var_nuendd_rv_slot: &mut f64,
        var_nuends_slot: &mut f64,
        var_nuends_rv_slot: &mut f64,
        var_nuintd_slot: &mut f64,
        var_nuintd_rv_slot: &mut f64,
        var_nuints_slot: &mut f64,
        var_nuints_rv_slot: &mut f64,
        var_rdraingeo_slot: &mut f64,
        var_rdraingeo_rv_slot: &mut f64,
        var_rend_slot: &mut f64,
        var_rend_rv_slot: &mut f64,
        var_rint_slot: &mut f64,
        var_rint_rv_slot: &mut f64,
        var_rsourcegeo_slot: &mut f64,
        var_rsourcegeo_rv_slot: &mut f64,
    ) {
        let mut var_guard225: f64 = *var_guard225_slot;
        let mut var_guard225_rv: f64 = *var_guard225_rv_slot;
        let mut var_guard226: f64 = *var_guard226_slot;
        let mut var_guard226_rv: f64 = *var_guard226_rv_slot;
        let mut var_guard227: f64 = *var_guard227_slot;
        let mut var_guard227_rv: f64 = *var_guard227_rv_slot;
        let mut var_guard228: f64 = *var_guard228_slot;
        let mut var_guard228_rv: f64 = *var_guard228_rv_slot;
        let mut var_guard229: f64 = *var_guard229_slot;
        let mut var_guard229_rv: f64 = *var_guard229_rv_slot;
        let mut var_guard230: f64 = *var_guard230_slot;
        let mut var_guard230_rv: f64 = *var_guard230_rv_slot;
        let mut var_guard232: f64 = *var_guard232_slot;
        let mut var_guard232_rv: f64 = *var_guard232_rv_slot;
        let mut var_guard233: f64 = *var_guard233_slot;
        let mut var_guard233_rv: f64 = *var_guard233_rv_slot;
        let mut var_guard234: f64 = *var_guard234_slot;
        let mut var_guard234_rv: f64 = *var_guard234_rv_slot;
        let mut var_guard235: f64 = *var_guard235_slot;
        let mut var_guard235_rv: f64 = *var_guard235_rv_slot;
        let mut var_guard237: f64 = *var_guard237_slot;
        let mut var_guard237_rv: f64 = *var_guard237_rv_slot;
        let mut var_guard238: f64 = *var_guard238_slot;
        let mut var_guard238_rv: f64 = *var_guard238_rv_slot;
        let mut var_guard239: f64 = *var_guard239_slot;
        let mut var_guard239_rv: f64 = *var_guard239_rv_slot;
        let mut var_guard240: f64 = *var_guard240_slot;
        let mut var_guard240_rv: f64 = *var_guard240_rv_slot;
        let mut var_guard241: f64 = *var_guard241_slot;
        let mut var_guard241_rv: f64 = *var_guard241_rv_slot;
        let mut var_guard242: f64 = *var_guard242_slot;
        let mut var_guard242_rv: f64 = *var_guard242_rv_slot;
        let mut var_guard243: f64 = *var_guard243_slot;
        let mut var_guard243_rv: f64 = *var_guard243_rv_slot;
        let mut var_guard245: f64 = *var_guard245_slot;
        let mut var_guard245_rv: f64 = *var_guard245_rv_slot;
        let mut var_guard246: f64 = *var_guard246_slot;
        let mut var_guard246_rv: f64 = *var_guard246_rv_slot;
        let mut var_guard247: f64 = *var_guard247_slot;
        let mut var_guard247_rv: f64 = *var_guard247_rv_slot;
        let mut var_guard248: f64 = *var_guard248_slot;
        let mut var_guard248_rv: f64 = *var_guard248_rv_slot;
        let mut var_guard249: f64 = *var_guard249_slot;
        let mut var_guard249_rv: f64 = *var_guard249_rv_slot;
        let mut var_nuendd: f64 = *var_nuendd_slot;
        let mut var_nuendd_rv: f64 = *var_nuendd_rv_slot;
        let mut var_nuends: f64 = *var_nuends_slot;
        let mut var_nuends_rv: f64 = *var_nuends_rv_slot;
        let mut var_nuintd: f64 = *var_nuintd_slot;
        let mut var_nuintd_rv: f64 = *var_nuintd_rv_slot;
        let mut var_nuints: f64 = *var_nuints_slot;
        let mut var_nuints_rv: f64 = *var_nuints_rv_slot;
        let mut var_rdraingeo: f64 = *var_rdraingeo_slot;
        let mut var_rdraingeo_rv: f64 = *var_rdraingeo_rv_slot;
        let mut var_rend: f64 = *var_rend_slot;
        let mut var_rend_rv: f64 = *var_rend_rv_slot;
        let mut var_rint: f64 = *var_rint_slot;
        let mut var_rint_rv: f64 = *var_rint_rv_slot;
        let mut var_rsourcegeo: f64 = *var_rsourcegeo_slot;
        let mut var_rsourcegeo_rv: f64 = *var_rsourcegeo_rv_slot;

        let (assign7880_e10310,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard91 != 0.0) && (!((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard213 == 0.0)) && (var_guard214 == 0.0)) && ((var_guard221 != 0.0) && (var_guard220 == 0.0))) && (var_guard224 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7880_e10310;
        var_rend_rv = 0.0;

        let (assign7890_e10356,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard91 != 0.0) && (!((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard213 == 0.0)) && (var_guard214 == 0.0)) && ((var_guard221 != 0.0) && (var_guard220 == 0.0))) && (var_guard224 == 0.0)) {
        let assign7890_e10346: f64 = (p.p374 * var_weff);
        let assign7890_e10349: f64 = (3.0 * var_nuendd);
        let assign7890_e10352: f64 = (var_dmcgeff + var_dmcieff);
        let assign7890_e10353: f64 = (assign7890_e10349 * assign7890_e10352);
        let assign7890_e10354: f64 = (assign7890_e10346 / assign7890_e10353);
        (assign7890_e10354,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7890_e10356;
        var_rend_rv = 0.0;

        let (assign7900_e10389,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard91 != 0.0) && (!((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0))))) && (var_guard213 == 0.0)) && (var_guard214 == 0.0)) && (!((var_guard220 != 0.0) || (var_guard221 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7900_e10389;
        var_rend_rv = 0.0;

        let assign7910_e10392: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard225 = assign7910_e10392;
        var_guard225_rv = 0.0;

        let assign7920_e10395: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard226 = assign7920_e10395;
        var_guard226_rv = 0.0;

        let (assign7930_e10423,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard92 != 0.0) && (!(((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard225 != 0.0)) && (var_guard226 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7930_e10423;
        var_rend_rv = 0.0;

        let (assign7940_e10458,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard92 != 0.0) && (!(((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard225 != 0.0)) && (var_guard226 == 0.0)) {
        let assign7940_e10452: f64 = (p.p374 * var_dmdgeff);
        let assign7940_e10455: f64 = (var_weff * var_nuends);
        let assign7940_e10456: f64 = (assign7940_e10452 / assign7940_e10455);
        (assign7940_e10456,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7940_e10458;
        var_rend_rv = 0.0;

        let assign7950_e10461: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard227 = assign7950_e10461;
        var_guard227_rv = 0.0;

        let assign7960_e10472: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard228 = assign7960_e10472;
        var_guard228_rv = 0.0;

        let assign7970_e10483: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard229 = assign7970_e10483;
        var_guard229_rv = 0.0;

        let assign7980_e10486: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard230 = assign7980_e10486;
        var_guard230_rv = 0.0;

        let (assign7990_e10519,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard92 != 0.0) && (!(((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard225 == 0.0)) && (var_guard227 != 0.0)) && (var_guard228 != 0.0)) && (var_guard230 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign7990_e10519;
        var_rend_rv = 0.0;

        let (assign8000_e10559,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard92 != 0.0) && (!(((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard225 == 0.0)) && (var_guard227 != 0.0)) && (var_guard228 != 0.0)) && (var_guard230 == 0.0)) {
        let assign8000_e10553: f64 = (p.p374 * var_dmcgeff);
        let assign8000_e10556: f64 = (var_weff * var_nuendd);
        let assign8000_e10557: f64 = (assign8000_e10553 / assign8000_e10556);
        (assign8000_e10557,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8000_e10559;
        var_rend_rv = 0.0;

        let assign8020_e10569: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard232 = assign8020_e10569;
        var_guard232_rv = 0.0;

        let (assign8030_e10605,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard92 != 0.0) && (!(((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard225 == 0.0)) && (var_guard227 != 0.0)) && ((var_guard229 != 0.0) && (var_guard228 == 0.0))) && (var_guard232 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8030_e10605;
        var_rend_rv = 0.0;

        let (assign8040_e10650,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard92 != 0.0) && (!(((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard225 == 0.0)) && (var_guard227 != 0.0)) && ((var_guard229 != 0.0) && (var_guard228 == 0.0))) && (var_guard232 == 0.0)) {
        let assign8040_e10642: f64 = (p.p374 * var_weff);
        let assign8040_e10645: f64 = (6.0 * var_nuendd);
        let assign8040_e10647: f64 = (assign8040_e10645 * var_dmcgeff);
        let assign8040_e10648: f64 = (assign8040_e10642 / assign8040_e10647);
        (assign8040_e10648,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8040_e10650;
        var_rend_rv = 0.0;

        let (assign8050_e10684,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard92 != 0.0) && (!(((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard225 == 0.0)) && (var_guard227 != 0.0)) && (!((var_guard228 != 0.0) || (var_guard229 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8050_e10684;
        var_rend_rv = 0.0;

        let assign8060_e10695: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard233 = assign8060_e10695;
        var_guard233_rv = 0.0;

        let assign8070_e10706: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard234 = assign8070_e10706;
        var_guard234_rv = 0.0;

        let assign8080_e10709: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard235 = assign8080_e10709;
        var_guard235_rv = 0.0;

        let (assign8090_e10743,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard92 != 0.0) && (!(((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard225 == 0.0)) && (var_guard227 == 0.0)) && (var_guard233 != 0.0)) && (var_guard235 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8090_e10743;
        var_rend_rv = 0.0;

        let (assign8100_e10784,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard92 != 0.0) && (!(((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard225 == 0.0)) && (var_guard227 == 0.0)) && (var_guard233 != 0.0)) && (var_guard235 == 0.0)) {
        let assign8100_e10778: f64 = (p.p374 * var_dmcgeff);
        let assign8100_e10781: f64 = (var_weff * var_nuendd);
        let assign8100_e10782: f64 = (assign8100_e10778 / assign8100_e10781);
        (assign8100_e10782,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8100_e10784;
        var_rend_rv = 0.0;

        let assign8120_e10794: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard237 = assign8120_e10794;
        var_guard237_rv = 0.0;

        let (assign8130_e10831,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard92 != 0.0) && (!(((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard225 == 0.0)) && (var_guard227 == 0.0)) && ((var_guard234 != 0.0) && (var_guard233 == 0.0))) && (var_guard237 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8130_e10831;
        var_rend_rv = 0.0;

        let (assign8140_e10877,) = {
    if (((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard92 != 0.0) && (!(((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard225 == 0.0)) && (var_guard227 == 0.0)) && ((var_guard234 != 0.0) && (var_guard233 == 0.0))) && (var_guard237 == 0.0)) {
        let assign8140_e10869: f64 = (p.p374 * var_weff);
        let assign8140_e10872: f64 = (6.0 * var_nuendd);
        let assign8140_e10874: f64 = (assign8140_e10872 * var_dmcgeff);
        let assign8140_e10875: f64 = (assign8140_e10869 / assign8140_e10874);
        (assign8140_e10875,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8140_e10877;
        var_rend_rv = 0.0;

        let (assign8150_e10912,) = {
    if ((((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard92 != 0.0) && (!(((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0))))) && (var_guard225 == 0.0)) && (var_guard227 == 0.0)) && (!((var_guard233 != 0.0) || (var_guard234 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8150_e10912;
        var_rend_rv = 0.0;

        let (assign8160_e10942,) = {
    if (((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard93 != 0.0) && (!((((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0))))) {
        let assign8160_e10938: f64 = (p.p374 * var_dmdgeff);
        let assign8160_e10940: f64 = (assign8160_e10938 / var_weff);
        (assign8160_e10940,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8160_e10942;
        var_rend_rv = 0.0;

        let assign8170_e10945: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard238 = assign8170_e10945;
        var_guard238_rv = 0.0;

        let (assign8180_e10981,) = {
    if ((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard94 != 0.0) && (!(((((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0))))) && (var_guard238 != 0.0)) {
        let assign8180_e10975: f64 = (0.5 * p.p374);
        let assign8180_e10977: f64 = (assign8180_e10975 * var_dmcgeff);
        let assign8180_e10979: f64 = (assign8180_e10977 / var_weff);
        (assign8180_e10979,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8180_e10981;
        var_rend_rv = 0.0;

        let assign8190_e10984: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        var_guard239 = assign8190_e10984;
        var_guard239_rv = 0.0;

        let (assign8200_e11016,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard94 != 0.0) && (!(((((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0))))) && (var_guard238 != 0.0)) && (var_guard239 != 0.0)) {
        (0.0,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign8200_e11016;
        var_rint_rv = 0.0;

        let (assign8210_e11057,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard94 != 0.0) && (!(((((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0))))) && (var_guard238 != 0.0)) && (var_guard239 == 0.0)) {
        let assign8210_e11049: f64 = (p.p374 * var_dmcgeff);
        let assign8210_e11053: f64 = (p.p2 - 2.0);
        let assign8210_e11054: f64 = (var_weff * assign8210_e11053);
        let assign8210_e11055: f64 = (assign8210_e11049 / assign8210_e11054);
        (assign8210_e11055,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign8210_e11057;
        var_rint_rv = 0.0;

        let (assign8220_e11088,) = {
    if ((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard94 != 0.0) && (!(((((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0))))) && (var_guard238 == 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8220_e11088;
        var_rend_rv = 0.0;

        let (assign8230_e11125,) = {
    if ((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard94 != 0.0) && (!(((((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0))))) && (var_guard238 == 0.0)) {
        let assign8230_e11119: f64 = (p.p374 * var_dmcgeff);
        let assign8230_e11122: f64 = (var_weff * p.p2);
        let assign8230_e11123: f64 = (assign8230_e11119 / assign8230_e11122);
        (assign8230_e11123,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign8230_e11125;
        var_rint_rv = 0.0;

        let assign8240_e11128: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard240 = assign8240_e11128;
        var_guard240_rv = 0.0;

        let (assign8250_e11160,) = {
    if ((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard95 != 0.0) && (!((((((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0)) || (var_guard94 != 0.0))))) && (var_guard240 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8250_e11160;
        var_rend_rv = 0.0;

        let (assign8260_e11198,) = {
    if ((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard95 != 0.0) && (!((((((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0)) || (var_guard94 != 0.0))))) && (var_guard240 != 0.0)) {
        let assign8260_e11192: f64 = (p.p374 * var_dmcgeff);
        let assign8260_e11195: f64 = (var_weff * p.p2);
        let assign8260_e11196: f64 = (assign8260_e11192 / assign8260_e11195);
        (assign8260_e11196,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign8260_e11198;
        var_rint_rv = 0.0;

        let (assign8270_e11237,) = {
    if ((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard95 != 0.0) && (!((((((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0)) || (var_guard94 != 0.0))))) && (var_guard240 == 0.0)) {
        let assign8270_e11231: f64 = (0.5 * p.p374);
        let assign8270_e11233: f64 = (assign8270_e11231 * var_dmcgeff);
        let assign8270_e11235: f64 = (assign8270_e11233 / var_weff);
        (assign8270_e11235,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8270_e11237;
        var_rend_rv = 0.0;

        let assign8280_e11240: f64 = if p.p2 == 2.0 { 1.0 } else { 0.0 };
        var_guard241 = assign8280_e11240;
        var_guard241_rv = 0.0;

        let (assign8290_e11275,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard95 != 0.0) && (!((((((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0)) || (var_guard94 != 0.0))))) && (var_guard240 == 0.0)) && (var_guard241 != 0.0)) {
        (0.0,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign8290_e11275;
        var_rint_rv = 0.0;

        let (assign8300_e11319,) = {
    if (((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && ((var_guard95 != 0.0) && (!((((((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0)) || (var_guard94 != 0.0))))) && (var_guard240 == 0.0)) && (var_guard241 == 0.0)) {
        let assign8300_e11311: f64 = (p.p374 * var_dmcgeff);
        let assign8300_e11315: f64 = (p.p2 - 2.0);
        let assign8300_e11316: f64 = (var_weff * assign8300_e11315);
        let assign8300_e11317: f64 = (assign8300_e11311 / assign8300_e11316);
        (assign8300_e11317,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign8300_e11319;
        var_rint_rv = 0.0;

        let (assign8310_e11349,) = {
    if (((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (!(((((((((((var_guard85 != 0.0) || (var_guard86 != 0.0)) || (var_guard87 != 0.0)) || (var_guard88 != 0.0)) || (var_guard89 != 0.0)) || (var_guard90 != 0.0)) || (var_guard91 != 0.0)) || (var_guard92 != 0.0)) || (var_guard93 != 0.0)) || (var_guard94 != 0.0)) || (var_guard95 != 0.0)))) {
        (0.0,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign8310_e11349;
        var_rint_rv = 0.0;

        let assign8320_e11352: f64 = if var_rint <= 0.0 { 1.0 } else { 0.0 };
        var_guard242 = assign8320_e11352;
        var_guard242_rv = 0.0;

        let (assign8330_e11361,) = {
    if (((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard242 != 0.0)) {
        (var_rend,)
    } else {
        (var_rsourcegeo,)
    }
};
        var_rsourcegeo = assign8330_e11361;
        var_rsourcegeo_rv = 0.0;

        let assign8340_e11364: f64 = if var_rend <= 0.0 { 1.0 } else { 0.0 };
        var_guard243 = assign8340_e11364;
        var_guard243_rv = 0.0;

        let (assign8350_e11376,) = {
    if ((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard242 == 0.0)) && (var_guard243 != 0.0)) {
        (var_rint,)
    } else {
        (var_rsourcegeo,)
    }
};
        var_rsourcegeo = assign8350_e11376;
        var_rsourcegeo_rv = 0.0;

        let (assign8360_e11395,) = {
    if ((((var_guard77 == 0.0) && (var_guard78 != 0.0)) && (var_guard242 == 0.0)) && (var_guard243 == 0.0)) {
        let assign8360_e11389: f64 = (var_rint * var_rend);
        let assign8360_e11392: f64 = (var_rint + var_rend);
        let assign8360_e11393: f64 = (assign8360_e11389 / assign8360_e11392);
        (assign8360_e11393,)
    } else {
        (var_rsourcegeo,)
    }
};
        var_rsourcegeo = assign8360_e11395;
        var_rsourcegeo_rv = 0.0;

        let (assign8380_e11406,) = {
    if ((var_guard77 == 0.0) && (var_guard78 == 0.0)) {
        (0.0,)
    } else {
        (var_rsourcegeo,)
    }
};
        var_rsourcegeo = assign8380_e11406;
        var_rsourcegeo_rv = 0.0;

        let assign8390_e11408: f64 = if param_given[4] { 1.0 } else { 0.0 };
        var_guard245 = assign8390_e11408;
        var_guard245_rv = 0.0;

        let (assign8400_e11414,) = {
    if (var_guard245 != 0.0) {
        let assign8400_e11412: f64 = (p.p374 * p.p4);
        (assign8400_e11412,)
    } else {
        (var_rdraingeo,)
    }
};
        var_rdraingeo = assign8400_e11414;
        var_rdraingeo_rv = 0.0;

        let assign8410_e11421: f64 = if ((p.p10 > 0.0) && (p.p374 > 0.0)) { 1.0 } else { 0.0 };
        var_guard246 = assign8410_e11421;
        var_guard246_rv = 0.0;

        let assign8420_e11424: f64 = if p.p9 < 9.0 { 1.0 } else { 0.0 };
        var_guard247 = assign8420_e11424;
        var_guard247_rv = 0.0;

        let assign8430_e11427: f64 = (p.p2 % 2.0);
        let assign8430_e11429: f64 = if assign8430_e11427 != 0.0 { 1.0 } else { 0.0 };
        var_guard248 = assign8430_e11429;
        var_guard248_rv = 0.0;

        let (assign8440_e11440,) = {
    if ((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) {
        (1.0,)
    } else {
        (var_nuendd,)
    }
};
        var_nuendd = assign8440_e11440;
        var_nuendd_rv = 0.0;

        let (assign8450_e11451,) = {
    if ((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) {
        (1.0,)
    } else {
        (var_nuends,)
    }
};
        var_nuends = assign8450_e11451;
        var_nuends_rv = 0.0;

        let (assign8460_e11470,) = {
    if ((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) {
        let assign8460_e11463: f64 = (p.p2 - 1.0);
        let assign8460_e11465: f64 = (assign8460_e11463 / 2.0);
        let assign8460_e11467: f64 = (assign8460_e11465).max(0.0);
        let assign8460_e11468: f64 = (2.0 * assign8460_e11467);
        (assign8460_e11468,)
    } else {
        (var_nuintd,)
    }
};
        var_nuintd = assign8460_e11470;
        var_nuintd_rv = 0.0;

        let (assign8470_e11481,) = {
    if ((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard247 != 0.0)) && (var_guard248 != 0.0)) {
        (var_nuintd,)
    } else {
        (var_nuints,)
    }
};
        var_nuints = assign8470_e11481;
        var_nuints_rv = 0.0;

        let assign8480_e11484: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        var_guard249 = assign8480_e11484;
        var_guard249_rv = 0.0;

        let (assign8490_e11498,) = {
    if (((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard247 != 0.0)) && (var_guard248 == 0.0)) && (var_guard249 != 0.0)) {
        (2.0,)
    } else {
        (var_nuendd,)
    }
};
        var_nuendd = assign8490_e11498;
        var_nuendd_rv = 0.0;

        *var_guard225_slot = var_guard225;
        *var_guard225_rv_slot = var_guard225_rv;
        *var_guard226_slot = var_guard226;
        *var_guard226_rv_slot = var_guard226_rv;
        *var_guard227_slot = var_guard227;
        *var_guard227_rv_slot = var_guard227_rv;
        *var_guard228_slot = var_guard228;
        *var_guard228_rv_slot = var_guard228_rv;
        *var_guard229_slot = var_guard229;
        *var_guard229_rv_slot = var_guard229_rv;
        *var_guard230_slot = var_guard230;
        *var_guard230_rv_slot = var_guard230_rv;
        *var_guard232_slot = var_guard232;
        *var_guard232_rv_slot = var_guard232_rv;
        *var_guard233_slot = var_guard233;
        *var_guard233_rv_slot = var_guard233_rv;
        *var_guard234_slot = var_guard234;
        *var_guard234_rv_slot = var_guard234_rv;
        *var_guard235_slot = var_guard235;
        *var_guard235_rv_slot = var_guard235_rv;
        *var_guard237_slot = var_guard237;
        *var_guard237_rv_slot = var_guard237_rv;
        *var_guard238_slot = var_guard238;
        *var_guard238_rv_slot = var_guard238_rv;
        *var_guard239_slot = var_guard239;
        *var_guard239_rv_slot = var_guard239_rv;
        *var_guard240_slot = var_guard240;
        *var_guard240_rv_slot = var_guard240_rv;
        *var_guard241_slot = var_guard241;
        *var_guard241_rv_slot = var_guard241_rv;
        *var_guard242_slot = var_guard242;
        *var_guard242_rv_slot = var_guard242_rv;
        *var_guard243_slot = var_guard243;
        *var_guard243_rv_slot = var_guard243_rv;
        *var_guard245_slot = var_guard245;
        *var_guard245_rv_slot = var_guard245_rv;
        *var_guard246_slot = var_guard246;
        *var_guard246_rv_slot = var_guard246_rv;
        *var_guard247_slot = var_guard247;
        *var_guard247_rv_slot = var_guard247_rv;
        *var_guard248_slot = var_guard248;
        *var_guard248_rv_slot = var_guard248_rv;
        *var_guard249_slot = var_guard249;
        *var_guard249_rv_slot = var_guard249_rv;
        *var_nuendd_slot = var_nuendd;
        *var_nuendd_rv_slot = var_nuendd_rv;
        *var_nuends_slot = var_nuends;
        *var_nuends_rv_slot = var_nuends_rv;
        *var_nuintd_slot = var_nuintd;
        *var_nuintd_rv_slot = var_nuintd_rv;
        *var_nuints_slot = var_nuints;
        *var_nuints_rv_slot = var_nuints_rv;
        *var_rdraingeo_slot = var_rdraingeo;
        *var_rdraingeo_rv_slot = var_rdraingeo_rv;
        *var_rend_slot = var_rend;
        *var_rend_rv_slot = var_rend_rv;
        *var_rint_slot = var_rint;
        *var_rint_rv_slot = var_rint_rv;
        *var_rsourcegeo_slot = var_rsourcegeo;
        *var_rsourcegeo_rv_slot = var_rsourcegeo_rv;
    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        var_dmcgeff: f64,
        var_dmcieff: f64,
        var_guard245: f64,
        var_guard246: f64,
        var_guard247: f64,
        var_guard248: f64,
        var_guard249: f64,
        var_weff: f64,
        var_guard250_slot: &mut f64,
        var_guard250_rv_slot: &mut f64,
        var_guard251_slot: &mut f64,
        var_guard251_rv_slot: &mut f64,
        var_guard252_slot: &mut f64,
        var_guard252_rv_slot: &mut f64,
        var_guard253_slot: &mut f64,
        var_guard253_rv_slot: &mut f64,
        var_guard254_slot: &mut f64,
        var_guard254_rv_slot: &mut f64,
        var_guard255_slot: &mut f64,
        var_guard255_rv_slot: &mut f64,
        var_guard256_slot: &mut f64,
        var_guard256_rv_slot: &mut f64,
        var_guard257_slot: &mut f64,
        var_guard257_rv_slot: &mut f64,
        var_guard258_slot: &mut f64,
        var_guard258_rv_slot: &mut f64,
        var_guard259_slot: &mut f64,
        var_guard259_rv_slot: &mut f64,
        var_guard260_slot: &mut f64,
        var_guard260_rv_slot: &mut f64,
        var_guard261_slot: &mut f64,
        var_guard261_rv_slot: &mut f64,
        var_guard262_slot: &mut f64,
        var_guard262_rv_slot: &mut f64,
        var_guard263_slot: &mut f64,
        var_guard263_rv_slot: &mut f64,
        var_guard264_slot: &mut f64,
        var_guard264_rv_slot: &mut f64,
        var_guard265_slot: &mut f64,
        var_guard265_rv_slot: &mut f64,
        var_guard266_slot: &mut f64,
        var_guard266_rv_slot: &mut f64,
        var_guard267_slot: &mut f64,
        var_guard267_rv_slot: &mut f64,
        var_guard268_slot: &mut f64,
        var_guard268_rv_slot: &mut f64,
        var_guard270_slot: &mut f64,
        var_guard270_rv_slot: &mut f64,
        var_guard271_slot: &mut f64,
        var_guard271_rv_slot: &mut f64,
        var_guard272_slot: &mut f64,
        var_guard272_rv_slot: &mut f64,
        var_guard273_slot: &mut f64,
        var_guard273_rv_slot: &mut f64,
        var_guard275_slot: &mut f64,
        var_guard275_rv_slot: &mut f64,
        var_guard276_slot: &mut f64,
        var_guard276_rv_slot: &mut f64,
        var_guard277_slot: &mut f64,
        var_guard277_rv_slot: &mut f64,
        var_guard278_slot: &mut f64,
        var_guard278_rv_slot: &mut f64,
        var_guard279_slot: &mut f64,
        var_guard279_rv_slot: &mut f64,
        var_guard281_slot: &mut f64,
        var_guard281_rv_slot: &mut f64,
        var_guard282_slot: &mut f64,
        var_guard282_rv_slot: &mut f64,
        var_guard283_slot: &mut f64,
        var_guard283_rv_slot: &mut f64,
        var_guard284_slot: &mut f64,
        var_guard284_rv_slot: &mut f64,
        var_guard286_slot: &mut f64,
        var_guard286_rv_slot: &mut f64,
        var_guard287_slot: &mut f64,
        var_guard287_rv_slot: &mut f64,
        var_guard288_slot: &mut f64,
        var_guard288_rv_slot: &mut f64,
        var_guard289_slot: &mut f64,
        var_guard289_rv_slot: &mut f64,
        var_guard290_slot: &mut f64,
        var_guard290_rv_slot: &mut f64,
        var_guard291_slot: &mut f64,
        var_guard291_rv_slot: &mut f64,
        var_nuendd_slot: &mut f64,
        var_nuendd_rv_slot: &mut f64,
        var_nuends_slot: &mut f64,
        var_nuends_rv_slot: &mut f64,
        var_nuintd_slot: &mut f64,
        var_nuintd_rv_slot: &mut f64,
        var_nuints_slot: &mut f64,
        var_nuints_rv_slot: &mut f64,
        var_rend_slot: &mut f64,
        var_rend_rv_slot: &mut f64,
        var_rint_slot: &mut f64,
        var_rint_rv_slot: &mut f64,
    ) {
        let mut var_guard250: f64 = *var_guard250_slot;
        let mut var_guard250_rv: f64 = *var_guard250_rv_slot;
        let mut var_guard251: f64 = *var_guard251_slot;
        let mut var_guard251_rv: f64 = *var_guard251_rv_slot;
        let mut var_guard252: f64 = *var_guard252_slot;
        let mut var_guard252_rv: f64 = *var_guard252_rv_slot;
        let mut var_guard253: f64 = *var_guard253_slot;
        let mut var_guard253_rv: f64 = *var_guard253_rv_slot;
        let mut var_guard254: f64 = *var_guard254_slot;
        let mut var_guard254_rv: f64 = *var_guard254_rv_slot;
        let mut var_guard255: f64 = *var_guard255_slot;
        let mut var_guard255_rv: f64 = *var_guard255_rv_slot;
        let mut var_guard256: f64 = *var_guard256_slot;
        let mut var_guard256_rv: f64 = *var_guard256_rv_slot;
        let mut var_guard257: f64 = *var_guard257_slot;
        let mut var_guard257_rv: f64 = *var_guard257_rv_slot;
        let mut var_guard258: f64 = *var_guard258_slot;
        let mut var_guard258_rv: f64 = *var_guard258_rv_slot;
        let mut var_guard259: f64 = *var_guard259_slot;
        let mut var_guard259_rv: f64 = *var_guard259_rv_slot;
        let mut var_guard260: f64 = *var_guard260_slot;
        let mut var_guard260_rv: f64 = *var_guard260_rv_slot;
        let mut var_guard261: f64 = *var_guard261_slot;
        let mut var_guard261_rv: f64 = *var_guard261_rv_slot;
        let mut var_guard262: f64 = *var_guard262_slot;
        let mut var_guard262_rv: f64 = *var_guard262_rv_slot;
        let mut var_guard263: f64 = *var_guard263_slot;
        let mut var_guard263_rv: f64 = *var_guard263_rv_slot;
        let mut var_guard264: f64 = *var_guard264_slot;
        let mut var_guard264_rv: f64 = *var_guard264_rv_slot;
        let mut var_guard265: f64 = *var_guard265_slot;
        let mut var_guard265_rv: f64 = *var_guard265_rv_slot;
        let mut var_guard266: f64 = *var_guard266_slot;
        let mut var_guard266_rv: f64 = *var_guard266_rv_slot;
        let mut var_guard267: f64 = *var_guard267_slot;
        let mut var_guard267_rv: f64 = *var_guard267_rv_slot;
        let mut var_guard268: f64 = *var_guard268_slot;
        let mut var_guard268_rv: f64 = *var_guard268_rv_slot;
        let mut var_guard270: f64 = *var_guard270_slot;
        let mut var_guard270_rv: f64 = *var_guard270_rv_slot;
        let mut var_guard271: f64 = *var_guard271_slot;
        let mut var_guard271_rv: f64 = *var_guard271_rv_slot;
        let mut var_guard272: f64 = *var_guard272_slot;
        let mut var_guard272_rv: f64 = *var_guard272_rv_slot;
        let mut var_guard273: f64 = *var_guard273_slot;
        let mut var_guard273_rv: f64 = *var_guard273_rv_slot;
        let mut var_guard275: f64 = *var_guard275_slot;
        let mut var_guard275_rv: f64 = *var_guard275_rv_slot;
        let mut var_guard276: f64 = *var_guard276_slot;
        let mut var_guard276_rv: f64 = *var_guard276_rv_slot;
        let mut var_guard277: f64 = *var_guard277_slot;
        let mut var_guard277_rv: f64 = *var_guard277_rv_slot;
        let mut var_guard278: f64 = *var_guard278_slot;
        let mut var_guard278_rv: f64 = *var_guard278_rv_slot;
        let mut var_guard279: f64 = *var_guard279_slot;
        let mut var_guard279_rv: f64 = *var_guard279_rv_slot;
        let mut var_guard281: f64 = *var_guard281_slot;
        let mut var_guard281_rv: f64 = *var_guard281_rv_slot;
        let mut var_guard282: f64 = *var_guard282_slot;
        let mut var_guard282_rv: f64 = *var_guard282_rv_slot;
        let mut var_guard283: f64 = *var_guard283_slot;
        let mut var_guard283_rv: f64 = *var_guard283_rv_slot;
        let mut var_guard284: f64 = *var_guard284_slot;
        let mut var_guard284_rv: f64 = *var_guard284_rv_slot;
        let mut var_guard286: f64 = *var_guard286_slot;
        let mut var_guard286_rv: f64 = *var_guard286_rv_slot;
        let mut var_guard287: f64 = *var_guard287_slot;
        let mut var_guard287_rv: f64 = *var_guard287_rv_slot;
        let mut var_guard288: f64 = *var_guard288_slot;
        let mut var_guard288_rv: f64 = *var_guard288_rv_slot;
        let mut var_guard289: f64 = *var_guard289_slot;
        let mut var_guard289_rv: f64 = *var_guard289_rv_slot;
        let mut var_guard290: f64 = *var_guard290_slot;
        let mut var_guard290_rv: f64 = *var_guard290_rv_slot;
        let mut var_guard291: f64 = *var_guard291_slot;
        let mut var_guard291_rv: f64 = *var_guard291_rv_slot;
        let mut var_nuendd: f64 = *var_nuendd_slot;
        let mut var_nuendd_rv: f64 = *var_nuendd_rv_slot;
        let mut var_nuends: f64 = *var_nuends_slot;
        let mut var_nuends_rv: f64 = *var_nuends_rv_slot;
        let mut var_nuintd: f64 = *var_nuintd_slot;
        let mut var_nuintd_rv: f64 = *var_nuintd_rv_slot;
        let mut var_nuints: f64 = *var_nuints_slot;
        let mut var_nuints_rv: f64 = *var_nuints_rv_slot;
        let mut var_rend: f64 = *var_rend_slot;
        let mut var_rend_rv: f64 = *var_rend_rv_slot;
        let mut var_rint: f64 = *var_rint_slot;
        let mut var_rint_rv: f64 = *var_rint_rv_slot;

        let (assign8500_e11520,) = {
    if (((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard247 != 0.0)) && (var_guard248 == 0.0)) && (var_guard249 != 0.0)) {
        let assign8500_e11513: f64 = (p.p2 / 2.0);
        let assign8500_e11515: f64 = (assign8500_e11513 - 1.0);
        let assign8500_e11517: f64 = (assign8500_e11515).max(0.0);
        let assign8500_e11518: f64 = (2.0 * assign8500_e11517);
        (assign8500_e11518,)
    } else {
        (var_nuintd,)
    }
};
        var_nuintd = assign8500_e11520;
        var_nuintd_rv = 0.0;

        let (assign8510_e11534,) = {
    if (((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard247 != 0.0)) && (var_guard248 == 0.0)) && (var_guard249 != 0.0)) {
        (0.0,)
    } else {
        (var_nuends,)
    }
};
        var_nuends = assign8510_e11534;
        var_nuends_rv = 0.0;

        let (assign8520_e11548,) = {
    if (((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard247 != 0.0)) && (var_guard248 == 0.0)) && (var_guard249 != 0.0)) {
        (p.p2,)
    } else {
        (var_nuints,)
    }
};
        var_nuints = assign8520_e11548;
        var_nuints_rv = 0.0;

        let (assign8530_e11563,) = {
    if (((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard247 != 0.0)) && (var_guard248 == 0.0)) && (var_guard249 == 0.0)) {
        (0.0,)
    } else {
        (var_nuendd,)
    }
};
        var_nuendd = assign8530_e11563;
        var_nuendd_rv = 0.0;

        let (assign8540_e11578,) = {
    if (((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard247 != 0.0)) && (var_guard248 == 0.0)) && (var_guard249 == 0.0)) {
        (p.p2,)
    } else {
        (var_nuintd,)
    }
};
        var_nuintd = assign8540_e11578;
        var_nuintd_rv = 0.0;

        let (assign8550_e11593,) = {
    if (((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard247 != 0.0)) && (var_guard248 == 0.0)) && (var_guard249 == 0.0)) {
        (2.0,)
    } else {
        (var_nuends,)
    }
};
        var_nuends = assign8550_e11593;
        var_nuends_rv = 0.0;

        let (assign8560_e11616,) = {
    if (((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard247 != 0.0)) && (var_guard248 == 0.0)) && (var_guard249 == 0.0)) {
        let assign8560_e11609: f64 = (p.p2 / 2.0);
        let assign8560_e11611: f64 = (assign8560_e11609 - 1.0);
        let assign8560_e11613: f64 = (assign8560_e11611).max(0.0);
        let assign8560_e11614: f64 = (2.0 * assign8560_e11613);
        (assign8560_e11614,)
    } else {
        (var_nuints,)
    }
};
        var_nuints = assign8560_e11616;
        var_nuints_rv = 0.0;

        let assign8570_e11619: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard250 = assign8570_e11619;
        var_guard250_rv = 0.0;

        let assign8580_e11622: f64 = if var_nuints == 0.0 { 1.0 } else { 0.0 };
        var_guard251 = assign8580_e11622;
        var_guard251_rv = 0.0;

        let (assign8590_e11635,) = {
    if (((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard247 != 0.0)) && (var_guard250 != 0.0)) && (var_guard251 != 0.0)) {
        (0.0,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign8590_e11635;
        var_rint_rv = 0.0;

        let (assign8600_e11655,) = {
    if (((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard247 != 0.0)) && (var_guard250 != 0.0)) && (var_guard251 == 0.0)) {
        let assign8600_e11649: f64 = (p.p374 * var_dmcgeff);
        let assign8600_e11652: f64 = (var_weff * var_nuints);
        let assign8600_e11653: f64 = (assign8600_e11649 / assign8600_e11652);
        (assign8600_e11653,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign8600_e11655;
        var_rint_rv = 0.0;

        let assign8610_e11658: f64 = if var_nuintd == 0.0 { 1.0 } else { 0.0 };
        var_guard252 = assign8610_e11658;
        var_guard252_rv = 0.0;

        let (assign8620_e11672,) = {
    if (((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard247 != 0.0)) && (var_guard250 == 0.0)) && (var_guard252 != 0.0)) {
        (0.0,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign8620_e11672;
        var_rint_rv = 0.0;

        let (assign8630_e11693,) = {
    if (((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard247 != 0.0)) && (var_guard250 == 0.0)) && (var_guard252 == 0.0)) {
        let assign8630_e11687: f64 = (p.p374 * var_dmcgeff);
        let assign8630_e11690: f64 = (var_weff * var_nuintd);
        let assign8630_e11691: f64 = (assign8630_e11687 / assign8630_e11690);
        (assign8630_e11691,)
    } else {
        (var_rint,)
    }
};
        var_rint = assign8630_e11693;
        var_rint_rv = 0.0;

        let assign8640_e11696: f64 = if p.p9 == 0.0 { 1.0 } else { 0.0 };
        var_guard253 = assign8640_e11696;
        var_guard253_rv = 0.0;

        let assign8650_e11699: f64 = if p.p9 == 1.0 { 1.0 } else { 0.0 };
        var_guard254 = assign8650_e11699;
        var_guard254_rv = 0.0;

        let assign8660_e11702: f64 = if p.p9 == 2.0 { 1.0 } else { 0.0 };
        var_guard255 = assign8660_e11702;
        var_guard255_rv = 0.0;

        let assign8670_e11705: f64 = if p.p9 == 3.0 { 1.0 } else { 0.0 };
        var_guard256 = assign8670_e11705;
        var_guard256_rv = 0.0;

        let assign8680_e11708: f64 = if p.p9 == 4.0 { 1.0 } else { 0.0 };
        var_guard257 = assign8680_e11708;
        var_guard257_rv = 0.0;

        let assign8690_e11711: f64 = if p.p9 == 5.0 { 1.0 } else { 0.0 };
        var_guard258 = assign8690_e11711;
        var_guard258_rv = 0.0;

        let assign8700_e11714: f64 = if p.p9 == 6.0 { 1.0 } else { 0.0 };
        var_guard259 = assign8700_e11714;
        var_guard259_rv = 0.0;

        let assign8710_e11717: f64 = if p.p9 == 7.0 { 1.0 } else { 0.0 };
        var_guard260 = assign8710_e11717;
        var_guard260_rv = 0.0;

        let assign8720_e11720: f64 = if p.p9 == 8.0 { 1.0 } else { 0.0 };
        var_guard261 = assign8720_e11720;
        var_guard261_rv = 0.0;

        let assign8730_e11723: f64 = if p.p9 == 9.0 { 1.0 } else { 0.0 };
        var_guard262 = assign8730_e11723;
        var_guard262_rv = 0.0;

        let assign8740_e11726: f64 = if p.p9 == 10.0 { 1.0 } else { 0.0 };
        var_guard263 = assign8740_e11726;
        var_guard263_rv = 0.0;

        let assign8750_e11729: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard264 = assign8750_e11729;
        var_guard264_rv = 0.0;

        let assign8760_e11732: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard265 = assign8760_e11732;
        var_guard265_rv = 0.0;

        let assign8770_e11743: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard266 = assign8770_e11743;
        var_guard266_rv = 0.0;

        let assign8780_e11754: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard267 = assign8780_e11754;
        var_guard267_rv = 0.0;

        let assign8790_e11757: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard268 = assign8790_e11757;
        var_guard268_rv = 0.0;

        let (assign8800_e11774,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 != 0.0)) && (var_guard265 != 0.0)) && (var_guard266 != 0.0)) && (var_guard268 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8800_e11774;
        var_rend_rv = 0.0;

        let (assign8810_e11798,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 != 0.0)) && (var_guard265 != 0.0)) && (var_guard266 != 0.0)) && (var_guard268 == 0.0)) {
        let assign8810_e11792: f64 = (p.p374 * var_dmcgeff);
        let assign8810_e11795: f64 = (var_weff * var_nuends);
        let assign8810_e11796: f64 = (assign8810_e11792 / assign8810_e11795);
        (assign8810_e11796,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8810_e11798;
        var_rend_rv = 0.0;

        let assign8830_e11809: f64 = (var_dmcgeff + var_dmcieff);
        let assign8830_e11812: f64 = if ((var_nuends == 0.0) || (assign8830_e11809 == 0.0)) { 1.0 } else { 0.0 };
        var_guard270 = assign8830_e11812;
        var_guard270_rv = 0.0;

        let (assign8840_e11832,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 != 0.0)) && (var_guard265 != 0.0)) && ((var_guard267 != 0.0) && (var_guard266 == 0.0))) && (var_guard270 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8840_e11832;
        var_rend_rv = 0.0;

        let (assign8850_e11863,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 != 0.0)) && (var_guard265 != 0.0)) && ((var_guard267 != 0.0) && (var_guard266 == 0.0))) && (var_guard270 == 0.0)) {
        let assign8850_e11853: f64 = (p.p374 * var_weff);
        let assign8850_e11856: f64 = (3.0 * var_nuends);
        let assign8850_e11859: f64 = (var_dmcgeff + var_dmcieff);
        let assign8850_e11860: f64 = (assign8850_e11856 * assign8850_e11859);
        let assign8850_e11861: f64 = (assign8850_e11853 / assign8850_e11860);
        (assign8850_e11861,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8850_e11863;
        var_rend_rv = 0.0;

        let (assign8860_e11881,) = {
    if ((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 != 0.0)) && (var_guard265 != 0.0)) && (!((var_guard266 != 0.0) || (var_guard267 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8860_e11881;
        var_rend_rv = 0.0;

        let assign8870_e11892: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard271 = assign8870_e11892;
        var_guard271_rv = 0.0;

        let assign8880_e11903: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard272 = assign8880_e11903;
        var_guard272_rv = 0.0;

        let assign8890_e11906: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard273 = assign8890_e11906;
        var_guard273_rv = 0.0;

        let (assign8900_e11924,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 != 0.0)) && (var_guard265 == 0.0)) && (var_guard271 != 0.0)) && (var_guard273 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8900_e11924;
        var_rend_rv = 0.0;

        let (assign8910_e11949,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 != 0.0)) && (var_guard265 == 0.0)) && (var_guard271 != 0.0)) && (var_guard273 == 0.0)) {
        let assign8910_e11943: f64 = (p.p374 * var_dmcgeff);
        let assign8910_e11946: f64 = (var_weff * var_nuends);
        let assign8910_e11947: f64 = (assign8910_e11943 / assign8910_e11946);
        (assign8910_e11947,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8910_e11949;
        var_rend_rv = 0.0;

        let assign8930_e11960: f64 = (var_dmcgeff + var_dmcieff);
        let assign8930_e11963: f64 = if ((var_nuends == 0.0) || (assign8930_e11960 == 0.0)) { 1.0 } else { 0.0 };
        var_guard275 = assign8930_e11963;
        var_guard275_rv = 0.0;

        let (assign8940_e11984,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 != 0.0)) && (var_guard265 == 0.0)) && ((var_guard272 != 0.0) && (var_guard271 == 0.0))) && (var_guard275 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8940_e11984;
        var_rend_rv = 0.0;

        let (assign8950_e12016,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 != 0.0)) && (var_guard265 == 0.0)) && ((var_guard272 != 0.0) && (var_guard271 == 0.0))) && (var_guard275 == 0.0)) {
        let assign8950_e12006: f64 = (p.p374 * var_weff);
        let assign8950_e12009: f64 = (3.0 * var_nuends);
        let assign8950_e12012: f64 = (var_dmcgeff + var_dmcieff);
        let assign8950_e12013: f64 = (assign8950_e12009 * assign8950_e12012);
        let assign8950_e12014: f64 = (assign8950_e12006 / assign8950_e12013);
        (assign8950_e12014,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8950_e12016;
        var_rend_rv = 0.0;

        let (assign8960_e12035,) = {
    if ((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 != 0.0)) && (var_guard265 == 0.0)) && (!((var_guard271 != 0.0) || (var_guard272 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign8960_e12035;
        var_rend_rv = 0.0;

        let assign8970_e12038: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard276 = assign8970_e12038;
        var_guard276_rv = 0.0;

        let assign8980_e12049: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard277 = assign8980_e12049;
        var_guard277_rv = 0.0;

        let assign8990_e12060: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard278 = assign8990_e12060;
        var_guard278_rv = 0.0;

        let assign9000_e12063: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard279 = assign9000_e12063;
        var_guard279_rv = 0.0;

        let (assign9010_e12081,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 == 0.0)) && (var_guard276 != 0.0)) && (var_guard277 != 0.0)) && (var_guard279 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9010_e12081;
        var_rend_rv = 0.0;

        let (assign9020_e12106,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 == 0.0)) && (var_guard276 != 0.0)) && (var_guard277 != 0.0)) && (var_guard279 == 0.0)) {
        let assign9020_e12100: f64 = (p.p374 * var_dmcgeff);
        let assign9020_e12103: f64 = (var_weff * var_nuendd);
        let assign9020_e12104: f64 = (assign9020_e12100 / assign9020_e12103);
        (assign9020_e12104,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9020_e12106;
        var_rend_rv = 0.0;

        let assign9040_e12117: f64 = (var_dmcgeff + var_dmcieff);
        let assign9040_e12120: f64 = if ((var_nuendd == 0.0) || (assign9040_e12117 == 0.0)) { 1.0 } else { 0.0 };
        var_guard281 = assign9040_e12120;
        var_guard281_rv = 0.0;

        let (assign9050_e12141,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 == 0.0)) && (var_guard276 != 0.0)) && ((var_guard278 != 0.0) && (var_guard277 == 0.0))) && (var_guard281 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9050_e12141;
        var_rend_rv = 0.0;

        let (assign9060_e12173,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 == 0.0)) && (var_guard276 != 0.0)) && ((var_guard278 != 0.0) && (var_guard277 == 0.0))) && (var_guard281 == 0.0)) {
        let assign9060_e12163: f64 = (p.p374 * var_weff);
        let assign9060_e12166: f64 = (3.0 * var_nuendd);
        let assign9060_e12169: f64 = (var_dmcgeff + var_dmcieff);
        let assign9060_e12170: f64 = (assign9060_e12166 * assign9060_e12169);
        let assign9060_e12171: f64 = (assign9060_e12163 / assign9060_e12170);
        (assign9060_e12171,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9060_e12173;
        var_rend_rv = 0.0;

        let (assign9070_e12192,) = {
    if ((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 == 0.0)) && (var_guard276 != 0.0)) && (!((var_guard277 != 0.0) || (var_guard278 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9070_e12192;
        var_rend_rv = 0.0;

        let assign9080_e12203: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard282 = assign9080_e12203;
        var_guard282_rv = 0.0;

        let assign9090_e12214: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard283 = assign9090_e12214;
        var_guard283_rv = 0.0;

        let assign9100_e12217: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard284 = assign9100_e12217;
        var_guard284_rv = 0.0;

        let (assign9110_e12236,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 == 0.0)) && (var_guard276 == 0.0)) && (var_guard282 != 0.0)) && (var_guard284 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9110_e12236;
        var_rend_rv = 0.0;

        let (assign9120_e12262,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 == 0.0)) && (var_guard276 == 0.0)) && (var_guard282 != 0.0)) && (var_guard284 == 0.0)) {
        let assign9120_e12256: f64 = (p.p374 * var_dmcgeff);
        let assign9120_e12259: f64 = (var_weff * var_nuendd);
        let assign9120_e12260: f64 = (assign9120_e12256 / assign9120_e12259);
        (assign9120_e12260,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9120_e12262;
        var_rend_rv = 0.0;

        let assign9140_e12273: f64 = (var_dmcgeff + var_dmcieff);
        let assign9140_e12276: f64 = if ((var_nuendd == 0.0) || (assign9140_e12273 == 0.0)) { 1.0 } else { 0.0 };
        var_guard286 = assign9140_e12276;
        var_guard286_rv = 0.0;

        let (assign9150_e12298,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 == 0.0)) && (var_guard276 == 0.0)) && ((var_guard283 != 0.0) && (var_guard282 == 0.0))) && (var_guard286 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9150_e12298;
        var_rend_rv = 0.0;

        let (assign9160_e12331,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 == 0.0)) && (var_guard276 == 0.0)) && ((var_guard283 != 0.0) && (var_guard282 == 0.0))) && (var_guard286 == 0.0)) {
        let assign9160_e12321: f64 = (p.p374 * var_weff);
        let assign9160_e12324: f64 = (3.0 * var_nuendd);
        let assign9160_e12327: f64 = (var_dmcgeff + var_dmcieff);
        let assign9160_e12328: f64 = (assign9160_e12324 * assign9160_e12327);
        let assign9160_e12329: f64 = (assign9160_e12321 / assign9160_e12328);
        (assign9160_e12329,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9160_e12331;
        var_rend_rv = 0.0;

        let (assign9170_e12351,) = {
    if ((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && (var_guard253 != 0.0)) && (var_guard264 == 0.0)) && (var_guard276 == 0.0)) && (!((var_guard282 != 0.0) || (var_guard283 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9170_e12351;
        var_rend_rv = 0.0;

        let assign9180_e12354: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard287 = assign9180_e12354;
        var_guard287_rv = 0.0;

        let assign9190_e12357: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard288 = assign9190_e12357;
        var_guard288_rv = 0.0;

        let assign9200_e12368: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard289 = assign9200_e12368;
        var_guard289_rv = 0.0;

        let assign9210_e12379: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard290 = assign9210_e12379;
        var_guard290_rv = 0.0;

        let assign9220_e12382: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard291 = assign9220_e12382;
        var_guard291_rv = 0.0;

        *var_guard250_slot = var_guard250;
        *var_guard250_rv_slot = var_guard250_rv;
        *var_guard251_slot = var_guard251;
        *var_guard251_rv_slot = var_guard251_rv;
        *var_guard252_slot = var_guard252;
        *var_guard252_rv_slot = var_guard252_rv;
        *var_guard253_slot = var_guard253;
        *var_guard253_rv_slot = var_guard253_rv;
        *var_guard254_slot = var_guard254;
        *var_guard254_rv_slot = var_guard254_rv;
        *var_guard255_slot = var_guard255;
        *var_guard255_rv_slot = var_guard255_rv;
        *var_guard256_slot = var_guard256;
        *var_guard256_rv_slot = var_guard256_rv;
        *var_guard257_slot = var_guard257;
        *var_guard257_rv_slot = var_guard257_rv;
        *var_guard258_slot = var_guard258;
        *var_guard258_rv_slot = var_guard258_rv;
        *var_guard259_slot = var_guard259;
        *var_guard259_rv_slot = var_guard259_rv;
        *var_guard260_slot = var_guard260;
        *var_guard260_rv_slot = var_guard260_rv;
        *var_guard261_slot = var_guard261;
        *var_guard261_rv_slot = var_guard261_rv;
        *var_guard262_slot = var_guard262;
        *var_guard262_rv_slot = var_guard262_rv;
        *var_guard263_slot = var_guard263;
        *var_guard263_rv_slot = var_guard263_rv;
        *var_guard264_slot = var_guard264;
        *var_guard264_rv_slot = var_guard264_rv;
        *var_guard265_slot = var_guard265;
        *var_guard265_rv_slot = var_guard265_rv;
        *var_guard266_slot = var_guard266;
        *var_guard266_rv_slot = var_guard266_rv;
        *var_guard267_slot = var_guard267;
        *var_guard267_rv_slot = var_guard267_rv;
        *var_guard268_slot = var_guard268;
        *var_guard268_rv_slot = var_guard268_rv;
        *var_guard270_slot = var_guard270;
        *var_guard270_rv_slot = var_guard270_rv;
        *var_guard271_slot = var_guard271;
        *var_guard271_rv_slot = var_guard271_rv;
        *var_guard272_slot = var_guard272;
        *var_guard272_rv_slot = var_guard272_rv;
        *var_guard273_slot = var_guard273;
        *var_guard273_rv_slot = var_guard273_rv;
        *var_guard275_slot = var_guard275;
        *var_guard275_rv_slot = var_guard275_rv;
        *var_guard276_slot = var_guard276;
        *var_guard276_rv_slot = var_guard276_rv;
        *var_guard277_slot = var_guard277;
        *var_guard277_rv_slot = var_guard277_rv;
        *var_guard278_slot = var_guard278;
        *var_guard278_rv_slot = var_guard278_rv;
        *var_guard279_slot = var_guard279;
        *var_guard279_rv_slot = var_guard279_rv;
        *var_guard281_slot = var_guard281;
        *var_guard281_rv_slot = var_guard281_rv;
        *var_guard282_slot = var_guard282;
        *var_guard282_rv_slot = var_guard282_rv;
        *var_guard283_slot = var_guard283;
        *var_guard283_rv_slot = var_guard283_rv;
        *var_guard284_slot = var_guard284;
        *var_guard284_rv_slot = var_guard284_rv;
        *var_guard286_slot = var_guard286;
        *var_guard286_rv_slot = var_guard286_rv;
        *var_guard287_slot = var_guard287;
        *var_guard287_rv_slot = var_guard287_rv;
        *var_guard288_slot = var_guard288;
        *var_guard288_rv_slot = var_guard288_rv;
        *var_guard289_slot = var_guard289;
        *var_guard289_rv_slot = var_guard289_rv;
        *var_guard290_slot = var_guard290;
        *var_guard290_rv_slot = var_guard290_rv;
        *var_guard291_slot = var_guard291;
        *var_guard291_rv_slot = var_guard291_rv;
        *var_nuendd_slot = var_nuendd;
        *var_nuendd_rv_slot = var_nuendd_rv;
        *var_nuends_slot = var_nuends;
        *var_nuends_rv_slot = var_nuends_rv;
        *var_nuintd_slot = var_nuintd;
        *var_nuintd_rv_slot = var_nuintd_rv;
        *var_nuints_slot = var_nuints;
        *var_nuints_rv_slot = var_nuints_rv;
        *var_rend_slot = var_rend;
        *var_rend_rv_slot = var_rend_rv;
        *var_rint_slot = var_rint;
        *var_rint_rv_slot = var_rint_rv;
    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        var_dmcgeff: f64,
        var_dmcieff: f64,
        var_guard245: f64,
        var_guard246: f64,
        var_guard253: f64,
        var_guard254: f64,
        var_guard255: f64,
        var_guard287: f64,
        var_guard288: f64,
        var_guard289: f64,
        var_guard290: f64,
        var_guard291: f64,
        var_nuendd: f64,
        var_nuends: f64,
        var_weff: f64,
        var_guard293_slot: &mut f64,
        var_guard293_rv_slot: &mut f64,
        var_guard294_slot: &mut f64,
        var_guard294_rv_slot: &mut f64,
        var_guard295_slot: &mut f64,
        var_guard295_rv_slot: &mut f64,
        var_guard296_slot: &mut f64,
        var_guard296_rv_slot: &mut f64,
        var_guard298_slot: &mut f64,
        var_guard298_rv_slot: &mut f64,
        var_guard299_slot: &mut f64,
        var_guard299_rv_slot: &mut f64,
        var_guard300_slot: &mut f64,
        var_guard300_rv_slot: &mut f64,
        var_guard301_slot: &mut f64,
        var_guard301_rv_slot: &mut f64,
        var_guard302_slot: &mut f64,
        var_guard302_rv_slot: &mut f64,
        var_guard304_slot: &mut f64,
        var_guard304_rv_slot: &mut f64,
        var_guard305_slot: &mut f64,
        var_guard305_rv_slot: &mut f64,
        var_guard306_slot: &mut f64,
        var_guard306_rv_slot: &mut f64,
        var_guard307_slot: &mut f64,
        var_guard307_rv_slot: &mut f64,
        var_guard309_slot: &mut f64,
        var_guard309_rv_slot: &mut f64,
        var_guard310_slot: &mut f64,
        var_guard310_rv_slot: &mut f64,
        var_guard311_slot: &mut f64,
        var_guard311_rv_slot: &mut f64,
        var_guard312_slot: &mut f64,
        var_guard312_rv_slot: &mut f64,
        var_guard313_slot: &mut f64,
        var_guard313_rv_slot: &mut f64,
        var_guard314_slot: &mut f64,
        var_guard314_rv_slot: &mut f64,
        var_guard316_slot: &mut f64,
        var_guard316_rv_slot: &mut f64,
        var_guard317_slot: &mut f64,
        var_guard317_rv_slot: &mut f64,
        var_guard318_slot: &mut f64,
        var_guard318_rv_slot: &mut f64,
        var_guard319_slot: &mut f64,
        var_guard319_rv_slot: &mut f64,
        var_guard321_slot: &mut f64,
        var_guard321_rv_slot: &mut f64,
        var_guard322_slot: &mut f64,
        var_guard322_rv_slot: &mut f64,
        var_guard323_slot: &mut f64,
        var_guard323_rv_slot: &mut f64,
        var_guard324_slot: &mut f64,
        var_guard324_rv_slot: &mut f64,
        var_guard325_slot: &mut f64,
        var_guard325_rv_slot: &mut f64,
        var_guard327_slot: &mut f64,
        var_guard327_rv_slot: &mut f64,
        var_rend_slot: &mut f64,
        var_rend_rv_slot: &mut f64,
    ) {
        let mut var_guard293: f64 = *var_guard293_slot;
        let mut var_guard293_rv: f64 = *var_guard293_rv_slot;
        let mut var_guard294: f64 = *var_guard294_slot;
        let mut var_guard294_rv: f64 = *var_guard294_rv_slot;
        let mut var_guard295: f64 = *var_guard295_slot;
        let mut var_guard295_rv: f64 = *var_guard295_rv_slot;
        let mut var_guard296: f64 = *var_guard296_slot;
        let mut var_guard296_rv: f64 = *var_guard296_rv_slot;
        let mut var_guard298: f64 = *var_guard298_slot;
        let mut var_guard298_rv: f64 = *var_guard298_rv_slot;
        let mut var_guard299: f64 = *var_guard299_slot;
        let mut var_guard299_rv: f64 = *var_guard299_rv_slot;
        let mut var_guard300: f64 = *var_guard300_slot;
        let mut var_guard300_rv: f64 = *var_guard300_rv_slot;
        let mut var_guard301: f64 = *var_guard301_slot;
        let mut var_guard301_rv: f64 = *var_guard301_rv_slot;
        let mut var_guard302: f64 = *var_guard302_slot;
        let mut var_guard302_rv: f64 = *var_guard302_rv_slot;
        let mut var_guard304: f64 = *var_guard304_slot;
        let mut var_guard304_rv: f64 = *var_guard304_rv_slot;
        let mut var_guard305: f64 = *var_guard305_slot;
        let mut var_guard305_rv: f64 = *var_guard305_rv_slot;
        let mut var_guard306: f64 = *var_guard306_slot;
        let mut var_guard306_rv: f64 = *var_guard306_rv_slot;
        let mut var_guard307: f64 = *var_guard307_slot;
        let mut var_guard307_rv: f64 = *var_guard307_rv_slot;
        let mut var_guard309: f64 = *var_guard309_slot;
        let mut var_guard309_rv: f64 = *var_guard309_rv_slot;
        let mut var_guard310: f64 = *var_guard310_slot;
        let mut var_guard310_rv: f64 = *var_guard310_rv_slot;
        let mut var_guard311: f64 = *var_guard311_slot;
        let mut var_guard311_rv: f64 = *var_guard311_rv_slot;
        let mut var_guard312: f64 = *var_guard312_slot;
        let mut var_guard312_rv: f64 = *var_guard312_rv_slot;
        let mut var_guard313: f64 = *var_guard313_slot;
        let mut var_guard313_rv: f64 = *var_guard313_rv_slot;
        let mut var_guard314: f64 = *var_guard314_slot;
        let mut var_guard314_rv: f64 = *var_guard314_rv_slot;
        let mut var_guard316: f64 = *var_guard316_slot;
        let mut var_guard316_rv: f64 = *var_guard316_rv_slot;
        let mut var_guard317: f64 = *var_guard317_slot;
        let mut var_guard317_rv: f64 = *var_guard317_rv_slot;
        let mut var_guard318: f64 = *var_guard318_slot;
        let mut var_guard318_rv: f64 = *var_guard318_rv_slot;
        let mut var_guard319: f64 = *var_guard319_slot;
        let mut var_guard319_rv: f64 = *var_guard319_rv_slot;
        let mut var_guard321: f64 = *var_guard321_slot;
        let mut var_guard321_rv: f64 = *var_guard321_rv_slot;
        let mut var_guard322: f64 = *var_guard322_slot;
        let mut var_guard322_rv: f64 = *var_guard322_rv_slot;
        let mut var_guard323: f64 = *var_guard323_slot;
        let mut var_guard323_rv: f64 = *var_guard323_rv_slot;
        let mut var_guard324: f64 = *var_guard324_slot;
        let mut var_guard324_rv: f64 = *var_guard324_rv_slot;
        let mut var_guard325: f64 = *var_guard325_slot;
        let mut var_guard325_rv: f64 = *var_guard325_rv_slot;
        let mut var_guard327: f64 = *var_guard327_slot;
        let mut var_guard327_rv: f64 = *var_guard327_rv_slot;
        let mut var_rend: f64 = *var_rend_slot;
        let mut var_rend_rv: f64 = *var_rend_rv_slot;

        let (assign9230_e12402,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 != 0.0)) && (var_guard288 != 0.0)) && (var_guard289 != 0.0)) && (var_guard291 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9230_e12402;
        var_rend_rv = 0.0;

        let (assign9240_e12429,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 != 0.0)) && (var_guard288 != 0.0)) && (var_guard289 != 0.0)) && (var_guard291 == 0.0)) {
        let assign9240_e12423: f64 = (p.p374 * var_dmcgeff);
        let assign9240_e12426: f64 = (var_weff * var_nuends);
        let assign9240_e12427: f64 = (assign9240_e12423 / assign9240_e12426);
        (assign9240_e12427,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9240_e12429;
        var_rend_rv = 0.0;

        let assign9260_e12440: f64 = (var_dmcgeff + var_dmcieff);
        let assign9260_e12443: f64 = if ((var_nuends == 0.0) || (assign9260_e12440 == 0.0)) { 1.0 } else { 0.0 };
        var_guard293 = assign9260_e12443;
        var_guard293_rv = 0.0;

        let (assign9270_e12466,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 != 0.0)) && (var_guard288 != 0.0)) && ((var_guard290 != 0.0) && (var_guard289 == 0.0))) && (var_guard293 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9270_e12466;
        var_rend_rv = 0.0;

        let (assign9280_e12500,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 != 0.0)) && (var_guard288 != 0.0)) && ((var_guard290 != 0.0) && (var_guard289 == 0.0))) && (var_guard293 == 0.0)) {
        let assign9280_e12490: f64 = (p.p374 * var_weff);
        let assign9280_e12493: f64 = (3.0 * var_nuends);
        let assign9280_e12496: f64 = (var_dmcgeff + var_dmcieff);
        let assign9280_e12497: f64 = (assign9280_e12493 * assign9280_e12496);
        let assign9280_e12498: f64 = (assign9280_e12490 / assign9280_e12497);
        (assign9280_e12498,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9280_e12500;
        var_rend_rv = 0.0;

        let (assign9290_e12521,) = {
    if ((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 != 0.0)) && (var_guard288 != 0.0)) && (!((var_guard289 != 0.0) || (var_guard290 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9290_e12521;
        var_rend_rv = 0.0;

        let assign9300_e12532: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard294 = assign9300_e12532;
        var_guard294_rv = 0.0;

        let assign9310_e12543: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard295 = assign9310_e12543;
        var_guard295_rv = 0.0;

        let assign9320_e12546: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard296 = assign9320_e12546;
        var_guard296_rv = 0.0;

        let (assign9330_e12567,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 != 0.0)) && (var_guard288 == 0.0)) && (var_guard294 != 0.0)) && (var_guard296 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9330_e12567;
        var_rend_rv = 0.0;

        let (assign9340_e12595,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 != 0.0)) && (var_guard288 == 0.0)) && (var_guard294 != 0.0)) && (var_guard296 == 0.0)) {
        let assign9340_e12589: f64 = (p.p374 * var_dmcgeff);
        let assign9340_e12592: f64 = (var_weff * var_nuends);
        let assign9340_e12593: f64 = (assign9340_e12589 / assign9340_e12592);
        (assign9340_e12593,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9340_e12595;
        var_rend_rv = 0.0;

        let assign9360_e12606: f64 = (var_dmcgeff + var_dmcieff);
        let assign9360_e12609: f64 = if ((var_nuends == 0.0) || (assign9360_e12606 == 0.0)) { 1.0 } else { 0.0 };
        var_guard298 = assign9360_e12609;
        var_guard298_rv = 0.0;

        let (assign9370_e12633,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 != 0.0)) && (var_guard288 == 0.0)) && ((var_guard295 != 0.0) && (var_guard294 == 0.0))) && (var_guard298 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9370_e12633;
        var_rend_rv = 0.0;

        let (assign9380_e12668,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 != 0.0)) && (var_guard288 == 0.0)) && ((var_guard295 != 0.0) && (var_guard294 == 0.0))) && (var_guard298 == 0.0)) {
        let assign9380_e12658: f64 = (p.p374 * var_weff);
        let assign9380_e12661: f64 = (3.0 * var_nuends);
        let assign9380_e12664: f64 = (var_dmcgeff + var_dmcieff);
        let assign9380_e12665: f64 = (assign9380_e12661 * assign9380_e12664);
        let assign9380_e12666: f64 = (assign9380_e12658 / assign9380_e12665);
        (assign9380_e12666,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9380_e12668;
        var_rend_rv = 0.0;

        let (assign9390_e12690,) = {
    if ((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 != 0.0)) && (var_guard288 == 0.0)) && (!((var_guard294 != 0.0) || (var_guard295 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9390_e12690;
        var_rend_rv = 0.0;

        let assign9400_e12693: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard299 = assign9400_e12693;
        var_guard299_rv = 0.0;

        let assign9410_e12704: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard300 = assign9410_e12704;
        var_guard300_rv = 0.0;

        let assign9420_e12715: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard301 = assign9420_e12715;
        var_guard301_rv = 0.0;

        let assign9430_e12718: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard302 = assign9430_e12718;
        var_guard302_rv = 0.0;

        let (assign9440_e12739,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 == 0.0)) && (var_guard299 != 0.0)) && (var_guard300 != 0.0)) && (var_guard302 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9440_e12739;
        var_rend_rv = 0.0;

        let (assign9450_e12767,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 == 0.0)) && (var_guard299 != 0.0)) && (var_guard300 != 0.0)) && (var_guard302 == 0.0)) {
        let assign9450_e12761: f64 = (p.p374 * var_dmcgeff);
        let assign9450_e12764: f64 = (var_weff * var_nuendd);
        let assign9450_e12765: f64 = (assign9450_e12761 / assign9450_e12764);
        (assign9450_e12765,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9450_e12767;
        var_rend_rv = 0.0;

        let assign9470_e12777: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard304 = assign9470_e12777;
        var_guard304_rv = 0.0;

        let (assign9480_e12801,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 == 0.0)) && (var_guard299 != 0.0)) && ((var_guard301 != 0.0) && (var_guard300 == 0.0))) && (var_guard304 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9480_e12801;
        var_rend_rv = 0.0;

        let (assign9490_e12834,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 == 0.0)) && (var_guard299 != 0.0)) && ((var_guard301 != 0.0) && (var_guard300 == 0.0))) && (var_guard304 == 0.0)) {
        let assign9490_e12826: f64 = (p.p374 * var_weff);
        let assign9490_e12829: f64 = (6.0 * var_nuendd);
        let assign9490_e12831: f64 = (assign9490_e12829 * var_dmcgeff);
        let assign9490_e12832: f64 = (assign9490_e12826 / assign9490_e12831);
        (assign9490_e12832,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9490_e12834;
        var_rend_rv = 0.0;

        let (assign9500_e12856,) = {
    if ((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 == 0.0)) && (var_guard299 != 0.0)) && (!((var_guard300 != 0.0) || (var_guard301 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9500_e12856;
        var_rend_rv = 0.0;

        let assign9510_e12867: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard305 = assign9510_e12867;
        var_guard305_rv = 0.0;

        let assign9520_e12878: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard306 = assign9520_e12878;
        var_guard306_rv = 0.0;

        let assign9530_e12881: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard307 = assign9530_e12881;
        var_guard307_rv = 0.0;

        let (assign9540_e12903,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 == 0.0)) && (var_guard299 == 0.0)) && (var_guard305 != 0.0)) && (var_guard307 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9540_e12903;
        var_rend_rv = 0.0;

        let (assign9550_e12932,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 == 0.0)) && (var_guard299 == 0.0)) && (var_guard305 != 0.0)) && (var_guard307 == 0.0)) {
        let assign9550_e12926: f64 = (p.p374 * var_dmcgeff);
        let assign9550_e12929: f64 = (var_weff * var_nuendd);
        let assign9550_e12930: f64 = (assign9550_e12926 / assign9550_e12929);
        (assign9550_e12930,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9550_e12932;
        var_rend_rv = 0.0;

        let assign9570_e12942: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard309 = assign9570_e12942;
        var_guard309_rv = 0.0;

        let (assign9580_e12967,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 == 0.0)) && (var_guard299 == 0.0)) && ((var_guard306 != 0.0) && (var_guard305 == 0.0))) && (var_guard309 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9580_e12967;
        var_rend_rv = 0.0;

        let (assign9590_e13001,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 == 0.0)) && (var_guard299 == 0.0)) && ((var_guard306 != 0.0) && (var_guard305 == 0.0))) && (var_guard309 == 0.0)) {
        let assign9590_e12993: f64 = (p.p374 * var_weff);
        let assign9590_e12996: f64 = (6.0 * var_nuendd);
        let assign9590_e12998: f64 = (assign9590_e12996 * var_dmcgeff);
        let assign9590_e12999: f64 = (assign9590_e12993 / assign9590_e12998);
        (assign9590_e12999,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9590_e13001;
        var_rend_rv = 0.0;

        let (assign9600_e13024,) = {
    if ((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard254 != 0.0) && (var_guard253 == 0.0))) && (var_guard287 == 0.0)) && (var_guard299 == 0.0)) && (!((var_guard305 != 0.0) || (var_guard306 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9600_e13024;
        var_rend_rv = 0.0;

        let assign9610_e13027: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard310 = assign9610_e13027;
        var_guard310_rv = 0.0;

        let assign9620_e13030: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard311 = assign9620_e13030;
        var_guard311_rv = 0.0;

        let assign9630_e13041: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard312 = assign9630_e13041;
        var_guard312_rv = 0.0;

        let assign9640_e13052: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard313 = assign9640_e13052;
        var_guard313_rv = 0.0;

        let assign9650_e13055: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard314 = assign9650_e13055;
        var_guard314_rv = 0.0;

        let (assign9660_e13077,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 != 0.0)) && (var_guard311 != 0.0)) && (var_guard312 != 0.0)) && (var_guard314 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9660_e13077;
        var_rend_rv = 0.0;

        let (assign9670_e13106,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 != 0.0)) && (var_guard311 != 0.0)) && (var_guard312 != 0.0)) && (var_guard314 == 0.0)) {
        let assign9670_e13100: f64 = (p.p374 * var_dmcgeff);
        let assign9670_e13103: f64 = (var_weff * var_nuends);
        let assign9670_e13104: f64 = (assign9670_e13100 / assign9670_e13103);
        (assign9670_e13104,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9670_e13106;
        var_rend_rv = 0.0;

        let assign9690_e13116: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard316 = assign9690_e13116;
        var_guard316_rv = 0.0;

        let (assign9700_e13141,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 != 0.0)) && (var_guard311 != 0.0)) && ((var_guard313 != 0.0) && (var_guard312 == 0.0))) && (var_guard316 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9700_e13141;
        var_rend_rv = 0.0;

        let (assign9710_e13175,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 != 0.0)) && (var_guard311 != 0.0)) && ((var_guard313 != 0.0) && (var_guard312 == 0.0))) && (var_guard316 == 0.0)) {
        let assign9710_e13167: f64 = (p.p374 * var_weff);
        let assign9710_e13170: f64 = (6.0 * var_nuends);
        let assign9710_e13172: f64 = (assign9710_e13170 * var_dmcgeff);
        let assign9710_e13173: f64 = (assign9710_e13167 / assign9710_e13172);
        (assign9710_e13173,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9710_e13175;
        var_rend_rv = 0.0;

        let (assign9720_e13198,) = {
    if ((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 != 0.0)) && (var_guard311 != 0.0)) && (!((var_guard312 != 0.0) || (var_guard313 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9720_e13198;
        var_rend_rv = 0.0;

        let assign9730_e13209: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard317 = assign9730_e13209;
        var_guard317_rv = 0.0;

        let assign9740_e13220: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard318 = assign9740_e13220;
        var_guard318_rv = 0.0;

        let assign9750_e13223: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard319 = assign9750_e13223;
        var_guard319_rv = 0.0;

        let (assign9760_e13246,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 != 0.0)) && (var_guard311 == 0.0)) && (var_guard317 != 0.0)) && (var_guard319 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9760_e13246;
        var_rend_rv = 0.0;

        let (assign9770_e13276,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 != 0.0)) && (var_guard311 == 0.0)) && (var_guard317 != 0.0)) && (var_guard319 == 0.0)) {
        let assign9770_e13270: f64 = (p.p374 * var_dmcgeff);
        let assign9770_e13273: f64 = (var_weff * var_nuends);
        let assign9770_e13274: f64 = (assign9770_e13270 / assign9770_e13273);
        (assign9770_e13274,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9770_e13276;
        var_rend_rv = 0.0;

        let assign9790_e13286: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard321 = assign9790_e13286;
        var_guard321_rv = 0.0;

        let (assign9800_e13312,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 != 0.0)) && (var_guard311 == 0.0)) && ((var_guard318 != 0.0) && (var_guard317 == 0.0))) && (var_guard321 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9800_e13312;
        var_rend_rv = 0.0;

        let (assign9810_e13347,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 != 0.0)) && (var_guard311 == 0.0)) && ((var_guard318 != 0.0) && (var_guard317 == 0.0))) && (var_guard321 == 0.0)) {
        let assign9810_e13339: f64 = (p.p374 * var_weff);
        let assign9810_e13342: f64 = (6.0 * var_nuends);
        let assign9810_e13344: f64 = (assign9810_e13342 * var_dmcgeff);
        let assign9810_e13345: f64 = (assign9810_e13339 / assign9810_e13344);
        (assign9810_e13345,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9810_e13347;
        var_rend_rv = 0.0;

        let (assign9820_e13371,) = {
    if ((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 != 0.0)) && (var_guard311 == 0.0)) && (!((var_guard317 != 0.0) || (var_guard318 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9820_e13371;
        var_rend_rv = 0.0;

        let assign9830_e13374: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard322 = assign9830_e13374;
        var_guard322_rv = 0.0;

        let assign9840_e13385: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard323 = assign9840_e13385;
        var_guard323_rv = 0.0;

        let assign9850_e13396: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard324 = assign9850_e13396;
        var_guard324_rv = 0.0;

        let assign9860_e13399: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard325 = assign9860_e13399;
        var_guard325_rv = 0.0;

        let (assign9870_e13422,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 == 0.0)) && (var_guard322 != 0.0)) && (var_guard323 != 0.0)) && (var_guard325 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9870_e13422;
        var_rend_rv = 0.0;

        let (assign9880_e13452,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 == 0.0)) && (var_guard322 != 0.0)) && (var_guard323 != 0.0)) && (var_guard325 == 0.0)) {
        let assign9880_e13446: f64 = (p.p374 * var_dmcgeff);
        let assign9880_e13449: f64 = (var_weff * var_nuendd);
        let assign9880_e13450: f64 = (assign9880_e13446 / assign9880_e13449);
        (assign9880_e13450,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9880_e13452;
        var_rend_rv = 0.0;

        let assign9900_e13463: f64 = (var_dmcgeff + var_dmcieff);
        let assign9900_e13466: f64 = if ((var_nuendd == 0.0) || (assign9900_e13463 == 0.0)) { 1.0 } else { 0.0 };
        var_guard327 = assign9900_e13466;
        var_guard327_rv = 0.0;

        let (assign9910_e13492,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 == 0.0)) && (var_guard322 != 0.0)) && ((var_guard324 != 0.0) && (var_guard323 == 0.0))) && (var_guard327 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9910_e13492;
        var_rend_rv = 0.0;

        let (assign9920_e13529,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 == 0.0)) && (var_guard322 != 0.0)) && ((var_guard324 != 0.0) && (var_guard323 == 0.0))) && (var_guard327 == 0.0)) {
        let assign9920_e13519: f64 = (p.p374 * var_weff);
        let assign9920_e13522: f64 = (3.0 * var_nuendd);
        let assign9920_e13525: f64 = (var_dmcgeff + var_dmcieff);
        let assign9920_e13526: f64 = (assign9920_e13522 * assign9920_e13525);
        let assign9920_e13527: f64 = (assign9920_e13519 / assign9920_e13526);
        (assign9920_e13527,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9920_e13529;
        var_rend_rv = 0.0;

        *var_guard293_slot = var_guard293;
        *var_guard293_rv_slot = var_guard293_rv;
        *var_guard294_slot = var_guard294;
        *var_guard294_rv_slot = var_guard294_rv;
        *var_guard295_slot = var_guard295;
        *var_guard295_rv_slot = var_guard295_rv;
        *var_guard296_slot = var_guard296;
        *var_guard296_rv_slot = var_guard296_rv;
        *var_guard298_slot = var_guard298;
        *var_guard298_rv_slot = var_guard298_rv;
        *var_guard299_slot = var_guard299;
        *var_guard299_rv_slot = var_guard299_rv;
        *var_guard300_slot = var_guard300;
        *var_guard300_rv_slot = var_guard300_rv;
        *var_guard301_slot = var_guard301;
        *var_guard301_rv_slot = var_guard301_rv;
        *var_guard302_slot = var_guard302;
        *var_guard302_rv_slot = var_guard302_rv;
        *var_guard304_slot = var_guard304;
        *var_guard304_rv_slot = var_guard304_rv;
        *var_guard305_slot = var_guard305;
        *var_guard305_rv_slot = var_guard305_rv;
        *var_guard306_slot = var_guard306;
        *var_guard306_rv_slot = var_guard306_rv;
        *var_guard307_slot = var_guard307;
        *var_guard307_rv_slot = var_guard307_rv;
        *var_guard309_slot = var_guard309;
        *var_guard309_rv_slot = var_guard309_rv;
        *var_guard310_slot = var_guard310;
        *var_guard310_rv_slot = var_guard310_rv;
        *var_guard311_slot = var_guard311;
        *var_guard311_rv_slot = var_guard311_rv;
        *var_guard312_slot = var_guard312;
        *var_guard312_rv_slot = var_guard312_rv;
        *var_guard313_slot = var_guard313;
        *var_guard313_rv_slot = var_guard313_rv;
        *var_guard314_slot = var_guard314;
        *var_guard314_rv_slot = var_guard314_rv;
        *var_guard316_slot = var_guard316;
        *var_guard316_rv_slot = var_guard316_rv;
        *var_guard317_slot = var_guard317;
        *var_guard317_rv_slot = var_guard317_rv;
        *var_guard318_slot = var_guard318;
        *var_guard318_rv_slot = var_guard318_rv;
        *var_guard319_slot = var_guard319;
        *var_guard319_rv_slot = var_guard319_rv;
        *var_guard321_slot = var_guard321;
        *var_guard321_rv_slot = var_guard321_rv;
        *var_guard322_slot = var_guard322;
        *var_guard322_rv_slot = var_guard322_rv;
        *var_guard323_slot = var_guard323;
        *var_guard323_rv_slot = var_guard323_rv;
        *var_guard324_slot = var_guard324;
        *var_guard324_rv_slot = var_guard324_rv;
        *var_guard325_slot = var_guard325;
        *var_guard325_rv_slot = var_guard325_rv;
        *var_guard327_slot = var_guard327;
        *var_guard327_rv_slot = var_guard327_rv;
        *var_rend_slot = var_rend;
        *var_rend_rv_slot = var_rend_rv;
    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        var_dmcgeff: f64,
        var_dmcieff: f64,
        var_guard245: f64,
        var_guard246: f64,
        var_guard253: f64,
        var_guard254: f64,
        var_guard255: f64,
        var_guard256: f64,
        var_guard257: f64,
        var_guard310: f64,
        var_guard322: f64,
        var_guard323: f64,
        var_guard324: f64,
        var_nuendd: f64,
        var_nuends: f64,
        var_weff: f64,
        var_guard328_slot: &mut f64,
        var_guard328_rv_slot: &mut f64,
        var_guard329_slot: &mut f64,
        var_guard329_rv_slot: &mut f64,
        var_guard330_slot: &mut f64,
        var_guard330_rv_slot: &mut f64,
        var_guard332_slot: &mut f64,
        var_guard332_rv_slot: &mut f64,
        var_guard333_slot: &mut f64,
        var_guard333_rv_slot: &mut f64,
        var_guard334_slot: &mut f64,
        var_guard334_rv_slot: &mut f64,
        var_guard335_slot: &mut f64,
        var_guard335_rv_slot: &mut f64,
        var_guard336_slot: &mut f64,
        var_guard336_rv_slot: &mut f64,
        var_guard337_slot: &mut f64,
        var_guard337_rv_slot: &mut f64,
        var_guard339_slot: &mut f64,
        var_guard339_rv_slot: &mut f64,
        var_guard340_slot: &mut f64,
        var_guard340_rv_slot: &mut f64,
        var_guard341_slot: &mut f64,
        var_guard341_rv_slot: &mut f64,
        var_guard342_slot: &mut f64,
        var_guard342_rv_slot: &mut f64,
        var_guard344_slot: &mut f64,
        var_guard344_rv_slot: &mut f64,
        var_guard345_slot: &mut f64,
        var_guard345_rv_slot: &mut f64,
        var_guard346_slot: &mut f64,
        var_guard346_rv_slot: &mut f64,
        var_guard347_slot: &mut f64,
        var_guard347_rv_slot: &mut f64,
        var_guard348_slot: &mut f64,
        var_guard348_rv_slot: &mut f64,
        var_guard350_slot: &mut f64,
        var_guard350_rv_slot: &mut f64,
        var_guard351_slot: &mut f64,
        var_guard351_rv_slot: &mut f64,
        var_guard352_slot: &mut f64,
        var_guard352_rv_slot: &mut f64,
        var_guard353_slot: &mut f64,
        var_guard353_rv_slot: &mut f64,
        var_guard355_slot: &mut f64,
        var_guard355_rv_slot: &mut f64,
        var_guard356_slot: &mut f64,
        var_guard356_rv_slot: &mut f64,
        var_guard357_slot: &mut f64,
        var_guard357_rv_slot: &mut f64,
        var_guard358_slot: &mut f64,
        var_guard358_rv_slot: &mut f64,
        var_guard359_slot: &mut f64,
        var_guard359_rv_slot: &mut f64,
        var_guard360_slot: &mut f64,
        var_guard360_rv_slot: &mut f64,
        var_guard362_slot: &mut f64,
        var_guard362_rv_slot: &mut f64,
        var_guard363_slot: &mut f64,
        var_guard363_rv_slot: &mut f64,
        var_guard364_slot: &mut f64,
        var_guard364_rv_slot: &mut f64,
        var_guard365_slot: &mut f64,
        var_guard365_rv_slot: &mut f64,
        var_guard367_slot: &mut f64,
        var_guard367_rv_slot: &mut f64,
        var_rend_slot: &mut f64,
        var_rend_rv_slot: &mut f64,
    ) {
        let mut var_guard328: f64 = *var_guard328_slot;
        let mut var_guard328_rv: f64 = *var_guard328_rv_slot;
        let mut var_guard329: f64 = *var_guard329_slot;
        let mut var_guard329_rv: f64 = *var_guard329_rv_slot;
        let mut var_guard330: f64 = *var_guard330_slot;
        let mut var_guard330_rv: f64 = *var_guard330_rv_slot;
        let mut var_guard332: f64 = *var_guard332_slot;
        let mut var_guard332_rv: f64 = *var_guard332_rv_slot;
        let mut var_guard333: f64 = *var_guard333_slot;
        let mut var_guard333_rv: f64 = *var_guard333_rv_slot;
        let mut var_guard334: f64 = *var_guard334_slot;
        let mut var_guard334_rv: f64 = *var_guard334_rv_slot;
        let mut var_guard335: f64 = *var_guard335_slot;
        let mut var_guard335_rv: f64 = *var_guard335_rv_slot;
        let mut var_guard336: f64 = *var_guard336_slot;
        let mut var_guard336_rv: f64 = *var_guard336_rv_slot;
        let mut var_guard337: f64 = *var_guard337_slot;
        let mut var_guard337_rv: f64 = *var_guard337_rv_slot;
        let mut var_guard339: f64 = *var_guard339_slot;
        let mut var_guard339_rv: f64 = *var_guard339_rv_slot;
        let mut var_guard340: f64 = *var_guard340_slot;
        let mut var_guard340_rv: f64 = *var_guard340_rv_slot;
        let mut var_guard341: f64 = *var_guard341_slot;
        let mut var_guard341_rv: f64 = *var_guard341_rv_slot;
        let mut var_guard342: f64 = *var_guard342_slot;
        let mut var_guard342_rv: f64 = *var_guard342_rv_slot;
        let mut var_guard344: f64 = *var_guard344_slot;
        let mut var_guard344_rv: f64 = *var_guard344_rv_slot;
        let mut var_guard345: f64 = *var_guard345_slot;
        let mut var_guard345_rv: f64 = *var_guard345_rv_slot;
        let mut var_guard346: f64 = *var_guard346_slot;
        let mut var_guard346_rv: f64 = *var_guard346_rv_slot;
        let mut var_guard347: f64 = *var_guard347_slot;
        let mut var_guard347_rv: f64 = *var_guard347_rv_slot;
        let mut var_guard348: f64 = *var_guard348_slot;
        let mut var_guard348_rv: f64 = *var_guard348_rv_slot;
        let mut var_guard350: f64 = *var_guard350_slot;
        let mut var_guard350_rv: f64 = *var_guard350_rv_slot;
        let mut var_guard351: f64 = *var_guard351_slot;
        let mut var_guard351_rv: f64 = *var_guard351_rv_slot;
        let mut var_guard352: f64 = *var_guard352_slot;
        let mut var_guard352_rv: f64 = *var_guard352_rv_slot;
        let mut var_guard353: f64 = *var_guard353_slot;
        let mut var_guard353_rv: f64 = *var_guard353_rv_slot;
        let mut var_guard355: f64 = *var_guard355_slot;
        let mut var_guard355_rv: f64 = *var_guard355_rv_slot;
        let mut var_guard356: f64 = *var_guard356_slot;
        let mut var_guard356_rv: f64 = *var_guard356_rv_slot;
        let mut var_guard357: f64 = *var_guard357_slot;
        let mut var_guard357_rv: f64 = *var_guard357_rv_slot;
        let mut var_guard358: f64 = *var_guard358_slot;
        let mut var_guard358_rv: f64 = *var_guard358_rv_slot;
        let mut var_guard359: f64 = *var_guard359_slot;
        let mut var_guard359_rv: f64 = *var_guard359_rv_slot;
        let mut var_guard360: f64 = *var_guard360_slot;
        let mut var_guard360_rv: f64 = *var_guard360_rv_slot;
        let mut var_guard362: f64 = *var_guard362_slot;
        let mut var_guard362_rv: f64 = *var_guard362_rv_slot;
        let mut var_guard363: f64 = *var_guard363_slot;
        let mut var_guard363_rv: f64 = *var_guard363_rv_slot;
        let mut var_guard364: f64 = *var_guard364_slot;
        let mut var_guard364_rv: f64 = *var_guard364_rv_slot;
        let mut var_guard365: f64 = *var_guard365_slot;
        let mut var_guard365_rv: f64 = *var_guard365_rv_slot;
        let mut var_guard367: f64 = *var_guard367_slot;
        let mut var_guard367_rv: f64 = *var_guard367_rv_slot;
        let mut var_rend: f64 = *var_rend_slot;
        let mut var_rend_rv: f64 = *var_rend_rv_slot;

        let (assign9930_e13553,) = {
    if ((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 == 0.0)) && (var_guard322 != 0.0)) && (!((var_guard323 != 0.0) || (var_guard324 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9930_e13553;
        var_rend_rv = 0.0;

        let assign9940_e13564: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard328 = assign9940_e13564;
        var_guard328_rv = 0.0;

        let assign9950_e13575: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard329 = assign9950_e13575;
        var_guard329_rv = 0.0;

        let assign9960_e13578: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard330 = assign9960_e13578;
        var_guard330_rv = 0.0;

        let (assign9970_e13602,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 == 0.0)) && (var_guard322 == 0.0)) && (var_guard328 != 0.0)) && (var_guard330 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9970_e13602;
        var_rend_rv = 0.0;

        let (assign9980_e13633,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 == 0.0)) && (var_guard322 == 0.0)) && (var_guard328 != 0.0)) && (var_guard330 == 0.0)) {
        let assign9980_e13627: f64 = (p.p374 * var_dmcgeff);
        let assign9980_e13630: f64 = (var_weff * var_nuendd);
        let assign9980_e13631: f64 = (assign9980_e13627 / assign9980_e13630);
        (assign9980_e13631,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign9980_e13633;
        var_rend_rv = 0.0;

        let assign10000_e13644: f64 = (var_dmcgeff + var_dmcieff);
        let assign10000_e13647: f64 = if ((var_nuendd == 0.0) || (assign10000_e13644 == 0.0)) { 1.0 } else { 0.0 };
        var_guard332 = assign10000_e13647;
        var_guard332_rv = 0.0;

        let (assign10010_e13674,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 == 0.0)) && (var_guard322 == 0.0)) && ((var_guard329 != 0.0) && (var_guard328 == 0.0))) && (var_guard332 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10010_e13674;
        var_rend_rv = 0.0;

        let (assign10020_e13712,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 == 0.0)) && (var_guard322 == 0.0)) && ((var_guard329 != 0.0) && (var_guard328 == 0.0))) && (var_guard332 == 0.0)) {
        let assign10020_e13702: f64 = (p.p374 * var_weff);
        let assign10020_e13705: f64 = (3.0 * var_nuendd);
        let assign10020_e13708: f64 = (var_dmcgeff + var_dmcieff);
        let assign10020_e13709: f64 = (assign10020_e13705 * assign10020_e13708);
        let assign10020_e13710: f64 = (assign10020_e13702 / assign10020_e13709);
        (assign10020_e13710,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10020_e13712;
        var_rend_rv = 0.0;

        let (assign10030_e13737,) = {
    if ((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard255 != 0.0) && (!((var_guard253 != 0.0) || (var_guard254 != 0.0))))) && (var_guard310 == 0.0)) && (var_guard322 == 0.0)) && (!((var_guard328 != 0.0) || (var_guard329 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10030_e13737;
        var_rend_rv = 0.0;

        let assign10040_e13740: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard333 = assign10040_e13740;
        var_guard333_rv = 0.0;

        let assign10050_e13743: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard334 = assign10050_e13743;
        var_guard334_rv = 0.0;

        let assign10060_e13754: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard335 = assign10060_e13754;
        var_guard335_rv = 0.0;

        let assign10070_e13765: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard336 = assign10070_e13765;
        var_guard336_rv = 0.0;

        let assign10080_e13768: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard337 = assign10080_e13768;
        var_guard337_rv = 0.0;

        let (assign10090_e13792,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 != 0.0)) && (var_guard334 != 0.0)) && (var_guard335 != 0.0)) && (var_guard337 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10090_e13792;
        var_rend_rv = 0.0;

        let (assign10100_e13823,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 != 0.0)) && (var_guard334 != 0.0)) && (var_guard335 != 0.0)) && (var_guard337 == 0.0)) {
        let assign10100_e13817: f64 = (p.p374 * var_dmcgeff);
        let assign10100_e13820: f64 = (var_weff * var_nuends);
        let assign10100_e13821: f64 = (assign10100_e13817 / assign10100_e13820);
        (assign10100_e13821,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10100_e13823;
        var_rend_rv = 0.0;

        let assign10120_e13833: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard339 = assign10120_e13833;
        var_guard339_rv = 0.0;

        let (assign10130_e13860,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 != 0.0)) && (var_guard334 != 0.0)) && ((var_guard336 != 0.0) && (var_guard335 == 0.0))) && (var_guard339 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10130_e13860;
        var_rend_rv = 0.0;

        let (assign10140_e13896,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 != 0.0)) && (var_guard334 != 0.0)) && ((var_guard336 != 0.0) && (var_guard335 == 0.0))) && (var_guard339 == 0.0)) {
        let assign10140_e13888: f64 = (p.p374 * var_weff);
        let assign10140_e13891: f64 = (6.0 * var_nuends);
        let assign10140_e13893: f64 = (assign10140_e13891 * var_dmcgeff);
        let assign10140_e13894: f64 = (assign10140_e13888 / assign10140_e13893);
        (assign10140_e13894,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10140_e13896;
        var_rend_rv = 0.0;

        let (assign10150_e13921,) = {
    if ((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 != 0.0)) && (var_guard334 != 0.0)) && (!((var_guard335 != 0.0) || (var_guard336 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10150_e13921;
        var_rend_rv = 0.0;

        let assign10160_e13932: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard340 = assign10160_e13932;
        var_guard340_rv = 0.0;

        let assign10170_e13943: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard341 = assign10170_e13943;
        var_guard341_rv = 0.0;

        let assign10180_e13946: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard342 = assign10180_e13946;
        var_guard342_rv = 0.0;

        let (assign10190_e13971,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 != 0.0)) && (var_guard334 == 0.0)) && (var_guard340 != 0.0)) && (var_guard342 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10190_e13971;
        var_rend_rv = 0.0;

        let (assign10200_e14003,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 != 0.0)) && (var_guard334 == 0.0)) && (var_guard340 != 0.0)) && (var_guard342 == 0.0)) {
        let assign10200_e13997: f64 = (p.p374 * var_dmcgeff);
        let assign10200_e14000: f64 = (var_weff * var_nuends);
        let assign10200_e14001: f64 = (assign10200_e13997 / assign10200_e14000);
        (assign10200_e14001,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10200_e14003;
        var_rend_rv = 0.0;

        let assign10220_e14013: f64 = if ((var_nuends == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard344 = assign10220_e14013;
        var_guard344_rv = 0.0;

        let (assign10230_e14041,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 != 0.0)) && (var_guard334 == 0.0)) && ((var_guard341 != 0.0) && (var_guard340 == 0.0))) && (var_guard344 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10230_e14041;
        var_rend_rv = 0.0;

        let (assign10240_e14078,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 != 0.0)) && (var_guard334 == 0.0)) && ((var_guard341 != 0.0) && (var_guard340 == 0.0))) && (var_guard344 == 0.0)) {
        let assign10240_e14070: f64 = (p.p374 * var_weff);
        let assign10240_e14073: f64 = (6.0 * var_nuends);
        let assign10240_e14075: f64 = (assign10240_e14073 * var_dmcgeff);
        let assign10240_e14076: f64 = (assign10240_e14070 / assign10240_e14075);
        (assign10240_e14076,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10240_e14078;
        var_rend_rv = 0.0;

        let (assign10250_e14104,) = {
    if ((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 != 0.0)) && (var_guard334 == 0.0)) && (!((var_guard340 != 0.0) || (var_guard341 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10250_e14104;
        var_rend_rv = 0.0;

        let assign10260_e14107: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard345 = assign10260_e14107;
        var_guard345_rv = 0.0;

        let assign10270_e14118: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard346 = assign10270_e14118;
        var_guard346_rv = 0.0;

        let assign10280_e14129: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard347 = assign10280_e14129;
        var_guard347_rv = 0.0;

        let assign10290_e14132: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard348 = assign10290_e14132;
        var_guard348_rv = 0.0;

        let (assign10300_e14157,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 == 0.0)) && (var_guard345 != 0.0)) && (var_guard346 != 0.0)) && (var_guard348 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10300_e14157;
        var_rend_rv = 0.0;

        let (assign10310_e14189,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 == 0.0)) && (var_guard345 != 0.0)) && (var_guard346 != 0.0)) && (var_guard348 == 0.0)) {
        let assign10310_e14183: f64 = (p.p374 * var_dmcgeff);
        let assign10310_e14186: f64 = (var_weff * var_nuendd);
        let assign10310_e14187: f64 = (assign10310_e14183 / assign10310_e14186);
        (assign10310_e14187,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10310_e14189;
        var_rend_rv = 0.0;

        let assign10330_e14199: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard350 = assign10330_e14199;
        var_guard350_rv = 0.0;

        let (assign10340_e14227,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 == 0.0)) && (var_guard345 != 0.0)) && ((var_guard347 != 0.0) && (var_guard346 == 0.0))) && (var_guard350 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10340_e14227;
        var_rend_rv = 0.0;

        let (assign10350_e14264,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 == 0.0)) && (var_guard345 != 0.0)) && ((var_guard347 != 0.0) && (var_guard346 == 0.0))) && (var_guard350 == 0.0)) {
        let assign10350_e14256: f64 = (p.p374 * var_weff);
        let assign10350_e14259: f64 = (6.0 * var_nuendd);
        let assign10350_e14261: f64 = (assign10350_e14259 * var_dmcgeff);
        let assign10350_e14262: f64 = (assign10350_e14256 / assign10350_e14261);
        (assign10350_e14262,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10350_e14264;
        var_rend_rv = 0.0;

        let (assign10360_e14290,) = {
    if ((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 == 0.0)) && (var_guard345 != 0.0)) && (!((var_guard346 != 0.0) || (var_guard347 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10360_e14290;
        var_rend_rv = 0.0;

        let assign10370_e14301: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard351 = assign10370_e14301;
        var_guard351_rv = 0.0;

        let assign10380_e14312: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard352 = assign10380_e14312;
        var_guard352_rv = 0.0;

        let assign10390_e14315: f64 = if var_nuendd == 0.0 { 1.0 } else { 0.0 };
        var_guard353 = assign10390_e14315;
        var_guard353_rv = 0.0;

        let (assign10400_e14341,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 == 0.0)) && (var_guard345 == 0.0)) && (var_guard351 != 0.0)) && (var_guard353 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10400_e14341;
        var_rend_rv = 0.0;

        let (assign10410_e14374,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 == 0.0)) && (var_guard345 == 0.0)) && (var_guard351 != 0.0)) && (var_guard353 == 0.0)) {
        let assign10410_e14368: f64 = (p.p374 * var_dmcgeff);
        let assign10410_e14371: f64 = (var_weff * var_nuendd);
        let assign10410_e14372: f64 = (assign10410_e14368 / assign10410_e14371);
        (assign10410_e14372,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10410_e14374;
        var_rend_rv = 0.0;

        let assign10430_e14384: f64 = if ((var_nuendd == 0.0) || (var_dmcgeff == 0.0)) { 1.0 } else { 0.0 };
        var_guard355 = assign10430_e14384;
        var_guard355_rv = 0.0;

        let (assign10440_e14413,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 == 0.0)) && (var_guard345 == 0.0)) && ((var_guard352 != 0.0) && (var_guard351 == 0.0))) && (var_guard355 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10440_e14413;
        var_rend_rv = 0.0;

        let (assign10450_e14451,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 == 0.0)) && (var_guard345 == 0.0)) && ((var_guard352 != 0.0) && (var_guard351 == 0.0))) && (var_guard355 == 0.0)) {
        let assign10450_e14443: f64 = (p.p374 * var_weff);
        let assign10450_e14446: f64 = (6.0 * var_nuendd);
        let assign10450_e14448: f64 = (assign10450_e14446 * var_dmcgeff);
        let assign10450_e14449: f64 = (assign10450_e14443 / assign10450_e14448);
        (assign10450_e14449,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10450_e14451;
        var_rend_rv = 0.0;

        let (assign10460_e14478,) = {
    if ((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard256 != 0.0) && (!(((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0))))) && (var_guard333 == 0.0)) && (var_guard345 == 0.0)) && (!((var_guard351 != 0.0) || (var_guard352 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10460_e14478;
        var_rend_rv = 0.0;

        let assign10470_e14481: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard356 = assign10470_e14481;
        var_guard356_rv = 0.0;

        let assign10480_e14484: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard357 = assign10480_e14484;
        var_guard357_rv = 0.0;

        let assign10490_e14495: f64 = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };
        var_guard358 = assign10490_e14495;
        var_guard358_rv = 0.0;

        let assign10500_e14506: f64 = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };
        var_guard359 = assign10500_e14506;
        var_guard359_rv = 0.0;

        let assign10510_e14509: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard360 = assign10510_e14509;
        var_guard360_rv = 0.0;

        let (assign10520_e14535,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard257 != 0.0) && (!((((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard356 != 0.0)) && (var_guard357 != 0.0)) && (var_guard358 != 0.0)) && (var_guard360 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10520_e14535;
        var_rend_rv = 0.0;

        let (assign10530_e14568,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard257 != 0.0) && (!((((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard356 != 0.0)) && (var_guard357 != 0.0)) && (var_guard358 != 0.0)) && (var_guard360 == 0.0)) {
        let assign10530_e14562: f64 = (p.p374 * var_dmcgeff);
        let assign10530_e14565: f64 = (var_weff * var_nuends);
        let assign10530_e14566: f64 = (assign10530_e14562 / assign10530_e14565);
        (assign10530_e14566,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10530_e14568;
        var_rend_rv = 0.0;

        let assign10550_e14579: f64 = (var_dmcgeff + var_dmcieff);
        let assign10550_e14582: f64 = if ((var_nuends == 0.0) || (assign10550_e14579 == 0.0)) { 1.0 } else { 0.0 };
        var_guard362 = assign10550_e14582;
        var_guard362_rv = 0.0;

        let (assign10560_e14611,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard257 != 0.0) && (!((((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard356 != 0.0)) && (var_guard357 != 0.0)) && ((var_guard359 != 0.0) && (var_guard358 == 0.0))) && (var_guard362 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10560_e14611;
        var_rend_rv = 0.0;

        let (assign10570_e14651,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard257 != 0.0) && (!((((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard356 != 0.0)) && (var_guard357 != 0.0)) && ((var_guard359 != 0.0) && (var_guard358 == 0.0))) && (var_guard362 == 0.0)) {
        let assign10570_e14641: f64 = (p.p374 * var_weff);
        let assign10570_e14644: f64 = (3.0 * var_nuends);
        let assign10570_e14647: f64 = (var_dmcgeff + var_dmcieff);
        let assign10570_e14648: f64 = (assign10570_e14644 * assign10570_e14647);
        let assign10570_e14649: f64 = (assign10570_e14641 / assign10570_e14648);
        (assign10570_e14649,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10570_e14651;
        var_rend_rv = 0.0;

        let (assign10580_e14678,) = {
    if ((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard257 != 0.0) && (!((((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard356 != 0.0)) && (var_guard357 != 0.0)) && (!((var_guard358 != 0.0) || (var_guard359 != 0.0)))) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10580_e14678;
        var_rend_rv = 0.0;

        let assign10590_e14689: f64 = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };
        var_guard363 = assign10590_e14689;
        var_guard363_rv = 0.0;

        let assign10600_e14700: f64 = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };
        var_guard364 = assign10600_e14700;
        var_guard364_rv = 0.0;

        let assign10610_e14703: f64 = if var_nuends == 0.0 { 1.0 } else { 0.0 };
        var_guard365 = assign10610_e14703;
        var_guard365_rv = 0.0;

        let (assign10620_e14730,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard257 != 0.0) && (!((((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard356 != 0.0)) && (var_guard357 == 0.0)) && (var_guard363 != 0.0)) && (var_guard365 != 0.0)) {
        (0.0,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10620_e14730;
        var_rend_rv = 0.0;

        let (assign10630_e14764,) = {
    if (((((((var_guard245 == 0.0) && (var_guard246 != 0.0)) && ((var_guard257 != 0.0) && (!((((var_guard253 != 0.0) || (var_guard254 != 0.0)) || (var_guard255 != 0.0)) || (var_guard256 != 0.0))))) && (var_guard356 != 0.0)) && (var_guard357 == 0.0)) && (var_guard363 != 0.0)) && (var_guard365 == 0.0)) {
        let assign10630_e14758: f64 = (p.p374 * var_dmcgeff);
        let assign10630_e14761: f64 = (var_weff * var_nuends);
        let assign10630_e14762: f64 = (assign10630_e14758 / assign10630_e14761);
        (assign10630_e14762,)
    } else {
        (var_rend,)
    }
};
        var_rend = assign10630_e14764;
        var_rend_rv = 0.0;

        let assign10650_e14775: f64 = (var_dmcgeff + var_dmcieff);
        let assign10650_e14778: f64 = if ((var_nuends == 0.0) || (assign10650_e14775 == 0.0)) { 1.0 } else { 0.0 };
        var_guard367 = assign10650_e14778;
        var_guard367_rv = 0.0;

        *var_guard328_slot = var_guard328;
        *var_guard328_rv_slot = var_guard328_rv;
        *var_guard329_slot = var_guard329;
        *var_guard329_rv_slot = var_guard329_rv;
        *var_guard330_slot = var_guard330;
        *var_guard330_rv_slot = var_guard330_rv;
        *var_guard332_slot = var_guard332;
        *var_guard332_rv_slot = var_guard332_rv;
        *var_guard333_slot = var_guard333;
        *var_guard333_rv_slot = var_guard333_rv;
        *var_guard334_slot = var_guard334;
        *var_guard334_rv_slot = var_guard334_rv;
        *var_guard335_slot = var_guard335;
        *var_guard335_rv_slot = var_guard335_rv;
        *var_guard336_slot = var_guard336;
        *var_guard336_rv_slot = var_guard336_rv;
        *var_guard337_slot = var_guard337;
        *var_guard337_rv_slot = var_guard337_rv;
        *var_guard339_slot = var_guard339;
        *var_guard339_rv_slot = var_guard339_rv;
        *var_guard340_slot = var_guard340;
        *var_guard340_rv_slot = var_guard340_rv;
        *var_guard341_slot = var_guard341;
        *var_guard341_rv_slot = var_guard341_rv;
        *var_guard342_slot = var_guard342;
        *var_guard342_rv_slot = var_guard342_rv;
        *var_guard344_slot = var_guard344;
        *var_guard344_rv_slot = var_guard344_rv;
        *var_guard345_slot = var_guard345;
        *var_guard345_rv_slot = var_guard345_rv;
        *var_guard346_slot = var_guard346;
        *var_guard346_rv_slot = var_guard346_rv;
        *var_guard347_slot = var_guard347;
        *var_guard347_rv_slot = var_guard347_rv;
        *var_guard348_slot = var_guard348;
        *var_guard348_rv_slot = var_guard348_rv;
        *var_guard350_slot = var_guard350;
        *var_guard350_rv_slot = var_guard350_rv;
        *var_guard351_slot = var_guard351;
        *var_guard351_rv_slot = var_guard351_rv;
        *var_guard352_slot = var_guard352;
        *var_guard352_rv_slot = var_guard352_rv;
        *var_guard353_slot = var_guard353;
        *var_guard353_rv_slot = var_guard353_rv;
        *var_guard355_slot = var_guard355;
        *var_guard355_rv_slot = var_guard355_rv;
        *var_guard356_slot = var_guard356;
        *var_guard356_rv_slot = var_guard356_rv;
        *var_guard357_slot = var_guard357;
        *var_guard357_rv_slot = var_guard357_rv;
        *var_guard358_slot = var_guard358;
        *var_guard358_rv_slot = var_guard358_rv;
        *var_guard359_slot = var_guard359;
        *var_guard359_rv_slot = var_guard359_rv;
        *var_guard360_slot = var_guard360;
        *var_guard360_rv_slot = var_guard360_rv;
        *var_guard362_slot = var_guard362;
        *var_guard362_rv_slot = var_guard362_rv;
        *var_guard363_slot = var_guard363;
        *var_guard363_rv_slot = var_guard363_rv;
        *var_guard364_slot = var_guard364;
        *var_guard364_rv_slot = var_guard364_rv;
        *var_guard365_slot = var_guard365;
        *var_guard365_rv_slot = var_guard365_rv;
        *var_guard367_slot = var_guard367;
        *var_guard367_rv_slot = var_guard367_rv;
        *var_rend_slot = var_rend;
        *var_rend_rv_slot = var_rend_rv;
    }
}
